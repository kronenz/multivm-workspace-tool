use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
};
use uuid::Uuid;

fn default_ssh_port() -> u16 {
    22
}

fn now_iso8601() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Workset {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub connections: Vec<ConnectionConfig>,
    pub grid_layout: GridLayout,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConnectionConfig {
    pub host: String,
    #[serde(default = "default_ssh_port")]
    pub port: u16,
    pub user: String,
    pub auth_method: AuthMethod,
    pub key_path: Option<String>,
    pub project_path: String,
    pub ai_cli_command: Option<String>,
    #[serde(default)]
    pub keepalive_interval_secs: Option<u32>,
    #[serde(default)]
    pub reconnect_max_retries: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    Key,
    Password,
    SshConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GridLayout {
    pub preset: Option<String>,
    pub rows: u32,
    pub cols: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorksetSummary {
    pub id: String,
    pub name: String,
    pub connection_count: u32,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateWorksetInput {
    pub name: String,
    pub connections: Vec<ConnectionConfig>,
    pub grid_layout: GridLayout,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateWorksetInput {
    pub name: Option<String>,
    pub connections: Option<Vec<ConnectionConfig>>,
    pub grid_layout: Option<GridLayout>,
}

#[derive(Debug)]
pub enum StoreError {
    ConfigDirUnavailable,
    Io(io::Error),
    Json(serde_json::Error),
    LockPoisoned,
    Validation(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::ConfigDirUnavailable => write!(
                f,
                "platform config directory unavailable (dirs::config_dir returned None)"
            ),
            StoreError::Io(e) => write!(f, "io error: {e}"),
            StoreError::Json(e) => write!(f, "json error: {e}"),
            StoreError::LockPoisoned => write!(f, "workset store lock poisoned"),
            StoreError::Validation(msg) => write!(f, "validation error: {msg}"),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<io::Error> for StoreError {
    fn from(value: io::Error) -> Self {
        StoreError::Io(value)
    }
}

impl From<serde_json::Error> for StoreError {
    fn from(value: serde_json::Error) -> Self {
        StoreError::Json(value)
    }
}

#[derive(Debug)]
pub struct WorksetStore {
    base_dir: Option<PathBuf>,
    lock: Mutex<()>,
}

impl WorksetStore {
    pub fn new() -> Self {
        let base_dir = dirs::config_dir()
            .map(|p| p.join("multivm-workspace").join("worksets"));

        Self {
            base_dir,
            lock: Mutex::new(()),
        }
    }

    #[cfg(test)]
    pub fn with_dir(dir: PathBuf) -> Self {
        Self {
            base_dir: Some(dir),
            lock: Mutex::new(()),
        }
    }

    pub fn list(&self) -> Vec<WorksetSummary> {
        match self.list_result() {
            Ok(items) => items,
            Err(_) => Vec::new(),
        }
    }

    pub fn get(&self, id: &str) -> Option<Workset> {
        match self.get_result(id) {
            Ok(workset) => Some(workset),
            Err(StoreError::Io(e)) if e.kind() == io::ErrorKind::NotFound => None,
            Err(_) => None,
        }
    }

    pub fn create(&self, input: CreateWorksetInput) -> Workset {
        let now = now_iso8601();
        let workset = Workset {
            id: Uuid::new_v4().to_string(),
            name: input.name,
            created_at: now.clone(),
            updated_at: now,
            connections: input.connections,
            grid_layout: input.grid_layout,
        };

        workset
    }

    pub fn update(&self, id: &str, input: UpdateWorksetInput) -> Option<Workset> {
        match self.update_result(id, input) {
            Ok(updated) => updated,
            Err(_) => None,
        }
    }

    pub fn delete(&self, id: &str) -> bool {
        match self.delete_result(id) {
            Ok(deleted) => deleted,
            Err(_) => false,
        }
    }

    pub fn create_result(&self, input: CreateWorksetInput) -> Result<Workset, StoreError> {
        let _guard = self.lock_guard()?;
        validate_create(&input)?;

        let workset = self.create(input);
        self.write_workset(&workset)?;
        Ok(workset)
    }

    pub fn update_result(
        &self,
        id: &str,
        input: UpdateWorksetInput,
    ) -> Result<Option<Workset>, StoreError> {
        let _guard = self.lock_guard()?;
        let mut current = match self.read_workset(id) {
            Ok(w) => w,
            Err(StoreError::Io(e)) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e),
        };

        if let Some(name) = input.name {
            if name.trim().is_empty() {
                return Err(StoreError::Validation("name must not be empty".to_string()));
            }
            current.name = name;
        }

        if let Some(connections) = input.connections {
            validate_connections(&connections)?;
            current.connections = connections;
        }

        if let Some(grid_layout) = input.grid_layout {
            validate_grid_layout(&grid_layout)?;
            current.grid_layout = grid_layout;
        }

        current.updated_at = now_iso8601();
        self.write_workset(&current)?;
        Ok(Some(current))
    }

    pub fn delete_result(&self, id: &str) -> Result<bool, StoreError> {
        let _guard = self.lock_guard()?;
        let path = self.path_for_id(id)?;
        match fs::remove_file(&path) {
            Ok(()) => Ok(true),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(StoreError::Io(e)),
        }
    }

    fn lock_guard(&self) -> Result<MutexGuard<'_, ()>, StoreError> {
        self.lock.lock().map_err(|_| StoreError::LockPoisoned)
    }

    fn ensure_dir(&self) -> Result<PathBuf, StoreError> {
        let base = self
            .base_dir
            .clone()
            .ok_or(StoreError::ConfigDirUnavailable)?;

        fs::create_dir_all(&base)?;
        Ok(base)
    }

    fn path_for_id(&self, id: &str) -> Result<PathBuf, StoreError> {
        let base = self.ensure_dir()?;
        Ok(base.join(format!("{id}.json")))
    }

    fn get_result(&self, id: &str) -> Result<Workset, StoreError> {
        let _guard = self.lock_guard()?;
        self.read_workset(id)
    }

    /// Read a workset from disk without acquiring the lock.
    /// Caller MUST hold `self.lock` before calling this.
    fn read_workset(&self, id: &str) -> Result<Workset, StoreError> {
        let path = self.path_for_id(id)?;
        let contents = fs::read_to_string(&path)?;
        Ok(serde_json::from_str::<Workset>(&contents)?)
    }

    fn list_result(&self) -> Result<Vec<WorksetSummary>, StoreError> {
        let _guard = self.lock_guard()?;
        let base = self.ensure_dir()?;
        let mut out: Vec<WorksetSummary> = Vec::new();

        for entry in fs::read_dir(&base)? {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path = entry.path();
            if !is_workset_json_path(&path) {
                continue;
            }

            let id = match path.file_stem().and_then(|s| s.to_str()) {
                Some(s) if !s.is_empty() => s.to_string(),
                _ => continue,
            };

            let contents = match fs::read_to_string(&path) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let value: serde_json::Value = match serde_json::from_str(&contents) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let name = match value.get("name").and_then(|v| v.as_str()) {
                Some(s) => s.to_string(),
                None => continue,
            };

            let updated_at = match value.get("updated_at").and_then(|v| v.as_str()) {
                Some(s) => s.to_string(),
                None => continue,
            };

            let connection_count = value
                .get("connections")
                .and_then(|v| v.as_array())
                .map(|a| a.len() as u32)
                .unwrap_or(0);

            out.push(WorksetSummary {
                id,
                name,
                connection_count,
                updated_at,
            });
        }

        out.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(out)
    }

    fn write_workset(&self, workset: &Workset) -> Result<(), StoreError> {
        let path = self.path_for_id(&workset.id)?;
        write_json_atomically(&path, workset)
    }
}

fn is_workset_json_path(path: &Path) -> bool {
    path.extension().and_then(|s| s.to_str()) == Some("json")
}

fn write_json_atomically<T: Serialize>(path: &Path, value: &T) -> Result<(), StoreError> {
    let bytes = serde_json::to_vec_pretty(value)?;

    let tmp_path = match path.file_name().and_then(|s| s.to_str()) {
        Some(name) => path.with_file_name(format!("{name}.tmp")),
        None => return Err(StoreError::Validation("invalid workset filename".to_string())),
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&tmp_path, bytes)?;

    match fs::remove_file(path) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(StoreError::Io(e)),
    }

    match fs::rename(&tmp_path, path) {
        Ok(()) => Ok(()),
        Err(e) => {
            let _ = fs::remove_file(&tmp_path);
            Err(StoreError::Io(e))
        }
    }
}

fn validate_create(input: &CreateWorksetInput) -> Result<(), StoreError> {
    if input.name.trim().is_empty() {
        return Err(StoreError::Validation("name must not be empty".to_string()));
    }
    validate_connections(&input.connections)?;
    validate_grid_layout(&input.grid_layout)?;
    Ok(())
}

fn validate_connections(connections: &[ConnectionConfig]) -> Result<(), StoreError> {
    if connections.is_empty() {
        return Err(StoreError::Validation(
            "connections must contain at least 1 item".to_string(),
        ));
    }
    if connections.len() > 10 {
        return Err(StoreError::Validation(
            "connections must contain at most 10 items".to_string(),
        ));
    }

    for (idx, c) in connections.iter().enumerate() {
        if c.host.trim().is_empty() {
            return Err(StoreError::Validation(format!(
                "connections[{idx}].host must not be empty"
            )));
        }
        if c.user.trim().is_empty() {
            return Err(StoreError::Validation(format!(
                "connections[{idx}].user must not be empty"
            )));
        }
        if c.project_path.trim().is_empty() {
            return Err(StoreError::Validation(format!(
                "connections[{idx}].project_path must not be empty"
            )));
        }

        if matches!(c.auth_method, AuthMethod::Key) {
            let key_ok = c
                .key_path
                .as_ref()
                .map(|p| !p.trim().is_empty())
                .unwrap_or(false);
            if !key_ok {
                return Err(StoreError::Validation(format!(
                    "connections[{idx}].key_path must be set when auth_method is key"
                )));
            }
        }
    }

    Ok(())
}

fn validate_grid_layout(grid_layout: &GridLayout) -> Result<(), StoreError> {
    if grid_layout.rows == 0 {
        return Err(StoreError::Validation(
            "grid_layout.rows must be >= 1".to_string(),
        ));
    }
    if grid_layout.cols == 0 {
        return Err(StoreError::Validation(
            "grid_layout.cols must be >= 1".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    use tempfile::tempdir;

    fn test_connection() -> ConnectionConfig {
        ConnectionConfig {
            host: "test.example.com".into(),
            port: 22,
            user: "testuser".into(),
            auth_method: AuthMethod::Key,
            key_path: Some("/home/user/.ssh/id_rsa".into()),
            project_path: "/home/user/project".into(),
            ai_cli_command: None,
            keepalive_interval_secs: None,
            reconnect_max_retries: None,
        }
    }

    fn test_grid() -> GridLayout {
        GridLayout {
            preset: Some("2x2".into()),
            rows: 2,
            cols: 2,
        }
    }

    fn test_input() -> CreateWorksetInput {
        CreateWorksetInput {
            name: "Test Workset".into(),
            connections: vec![test_connection()],
            grid_layout: test_grid(),
        }
    }

    fn update_name(store: &WorksetStore, id: &str, new_name: &str) -> Result<Workset, StoreError> {
        let input = UpdateWorksetInput {
            name: Some(new_name.to_string()),
            connections: None,
            grid_layout: None,
        };
        match store.update_result(id, input)? {
            Some(w) => Ok(w),
            None => Err(StoreError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                "workset not found",
            ))),
        }
    }

    #[test]
    fn test_create_workset_basic() {
        let dir = tempdir().expect("create tempdir");
        let store = WorksetStore::with_dir(dir.path().to_path_buf());

        let workset = store.create(test_input());
        assert!(!workset.id.trim().is_empty());
        assert_eq!(workset.name, "Test Workset");
        assert!(!workset.created_at.trim().is_empty());
        assert!(!workset.updated_at.trim().is_empty());
        assert_eq!(workset.created_at, workset.updated_at);

        DateTime::parse_from_rfc3339(&workset.created_at).expect("created_at is RFC3339");
        DateTime::parse_from_rfc3339(&workset.updated_at).expect("updated_at is RFC3339");
    }

    #[test]
    fn test_create_workset_empty_name_fails() {
        let dir = tempdir().expect("create tempdir");
        let store = WorksetStore::with_dir(dir.path().to_path_buf());

        let mut input = test_input();
        input.name = "".into();

        let res = store.create_result(input);
        assert!(matches!(res, Err(StoreError::Validation(_))));
    }

    #[test]
    fn test_create_workset_no_connections_fails() {
        let dir = tempdir().expect("create tempdir");
        let store = WorksetStore::with_dir(dir.path().to_path_buf());

        let mut input = test_input();
        input.connections = Vec::new();

        let res = store.create_result(input);
        assert!(matches!(res, Err(StoreError::Validation(_))));
    }

    #[test]
    fn test_create_workset_11_connections_fails() {
        let dir = tempdir().expect("create tempdir");
        let store = WorksetStore::with_dir(dir.path().to_path_buf());

        let mut input = test_input();
        input.connections = (0..11).map(|_| test_connection()).collect();

        let res = store.create_result(input);
        assert!(matches!(res, Err(StoreError::Validation(_))));
    }

    #[test]
    fn test_validate_connection_empty_host_fails() {
        let mut c = test_connection();
        c.host = "".into();

        let res = validate_connections(&[c]);
        assert!(matches!(res, Err(StoreError::Validation(_))));
    }

    #[test]
    fn test_validate_connection_empty_user_fails() {
        let mut c = test_connection();
        c.user = "".into();

        let res = validate_connections(&[c]);
        assert!(matches!(res, Err(StoreError::Validation(_))));
    }

    #[test]
    fn test_validate_connection_key_without_keypath_fails() {
        let mut c = test_connection();
        c.key_path = None;
        c.auth_method = AuthMethod::Key;

        let res = validate_connections(&[c]);
        assert!(matches!(res, Err(StoreError::Validation(_))));
    }

    #[test]
    fn test_validate_grid_layout_zero_rows_fails() {
        let grid = GridLayout {
            preset: Some("2x2".into()),
            rows: 0,
            cols: 2,
        };

        let res = validate_grid_layout(&grid);
        assert!(matches!(res, Err(StoreError::Validation(_))));
    }

    #[test]
    fn test_validate_grid_layout_zero_cols_fails() {
        let grid = GridLayout {
            preset: Some("2x2".into()),
            rows: 2,
            cols: 0,
        };

        let res = validate_grid_layout(&grid);
        assert!(matches!(res, Err(StoreError::Validation(_))));
    }

    #[test]
    fn test_workset_store_crud_with_tempdir() {
        let dir = tempdir().expect("create tempdir");
        let store = WorksetStore::with_dir(dir.path().to_path_buf());

        let created = store
            .create_result(test_input())
            .expect("create_result succeeds");

        let got = store.get(&created.id).expect("get returns Some");
        assert_eq!(got.id, created.id);
        assert_eq!(got.name, created.name);
        assert_eq!(got.connections.len(), 1);

        let list = store.list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, created.id);
        assert_eq!(list[0].name, created.name);
        assert_eq!(list[0].connection_count, 1);
        DateTime::parse_from_rfc3339(&list[0].updated_at).expect("summary updated_at is RFC3339");

        let updated = update_name(&store, &created.id, "Updated Workset")
            .expect("manual update succeeds");
        assert_eq!(updated.id, created.id);
        assert_eq!(updated.name, "Updated Workset");

        let list_after_update = store.list();
        assert_eq!(list_after_update.len(), 1);
        assert_eq!(list_after_update[0].id, created.id);
        assert_eq!(list_after_update[0].name, "Updated Workset");

        let deleted = store
            .delete_result(&created.id)
            .expect("delete_result succeeds");
        assert!(deleted);
        assert!(store.get(&created.id).is_none());
        assert!(store.list().is_empty());
    }

    #[test]
    fn test_update_workset_partial() {
        let dir = tempdir().expect("create tempdir");
        let store = WorksetStore::with_dir(dir.path().to_path_buf());

        let created = store
            .create_result(test_input())
            .expect("create_result succeeds");
        let original_connections = created.connections.clone();

        let updated = update_name(&store, &created.id, "New Name")
            .expect("manual update succeeds");

        assert_eq!(updated.name, "New Name");
        assert_eq!(updated.connections.len(), original_connections.len());
        assert_eq!(updated.connections[0].host, original_connections[0].host);
        assert_eq!(updated.connections[0].user, original_connections[0].user);
        assert_eq!(updated.connections[0].project_path, original_connections[0].project_path);
        assert_eq!(updated.connections[0].key_path, original_connections[0].key_path);
    }

    #[test]
    fn test_delete_nonexistent_returns_false() {
        let dir = tempdir().expect("create tempdir");
        let store = WorksetStore::with_dir(dir.path().to_path_buf());

        let deleted = store
            .delete_result("does-not-exist")
            .expect("delete_result succeeds");
        assert!(!deleted);
    }

    #[test]
    fn test_now_iso8601_format() {
        let now = now_iso8601();
        assert!(now.contains('T'));
        assert!(now.ends_with('Z'));
        DateTime::parse_from_rfc3339(&now).expect("now_iso8601 is RFC3339");
    }
}
