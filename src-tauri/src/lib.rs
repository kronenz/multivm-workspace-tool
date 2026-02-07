mod workset;
mod ssh;

use serde::Serialize;
use uuid::Uuid;
use workset::{AuthMethod, CreateWorksetInput, UpdateWorksetInput, Workset, WorksetStore, WorksetSummary};
use ssh::{FileEntry, ReadFileResult, SshConnectionManager, SshSessionConfig};

// ── Return type for activate_workset ──

#[derive(Serialize, Clone)]
pub struct SessionInfo {
    pub session_id: String,
    pub connection_index: usize,
    pub host: String,
    pub status: String,
}

// ── Existing CRUD Commands (unchanged) ──

#[tauri::command]
fn list_worksets(store: tauri::State<'_, WorksetStore>) -> Vec<WorksetSummary> {
    store.list()
}

#[tauri::command]
fn get_workset(id: String, store: tauri::State<'_, WorksetStore>) -> Option<Workset> {
    store.get(&id)
}

#[tauri::command]
fn create_workset(
    input: CreateWorksetInput,
    store: tauri::State<'_, WorksetStore>,
) -> Result<Workset, String> {
    store.create_result(input).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_workset(
    id: String,
    input: UpdateWorksetInput,
    store: tauri::State<'_, WorksetStore>,
) -> Result<Workset, String> {
    match store.update(&id, input) {
        Some(workset) => Ok(workset),
        None => Err(format!("workset not found: {id}")),
    }
}

#[tauri::command]
fn delete_workset(id: String, store: tauri::State<'_, WorksetStore>) -> Result<bool, String> {
    Ok(store.delete(&id))
}

// ── New SSH Terminal Commands ──

#[tauri::command]
async fn activate_workset(
    workset_id: String,
    passwords: Vec<Option<String>>,
    app: tauri::AppHandle,
    store: tauri::State<'_, WorksetStore>,
    ssh_manager: tauri::State<'_, SshConnectionManager>,
) -> Result<Vec<SessionInfo>, String> {
    let workset = store.get(&workset_id).ok_or_else(|| format!("workset not found: {workset_id}"))?;

    // Prevent double activation
    ssh_manager.disconnect_all();

    let mut configs: Vec<SshSessionConfig> = Vec::new();
    for (i, conn) in workset.connections.iter().enumerate() {
        let auth_method_str = match conn.auth_method {
            AuthMethod::Key => "key".to_string(),
            AuthMethod::Password => "password".to_string(),
            AuthMethod::SshConfig => {
                return Err("SshConfig auth is not yet supported".to_string());
            }
        };

        let password = if matches!(conn.auth_method, AuthMethod::Password) {
            passwords.get(i).cloned().flatten()
        } else {
            None
        };

        configs.push(SshSessionConfig {
            id: Uuid::new_v4().to_string(),
            host: conn.host.clone(),
            port: conn.port,
            user: conn.user.clone(),
            auth_method: auth_method_str,
            key_path: conn.key_path.clone(),
            password,
            project_path: conn.project_path.clone(),
            ai_cli_command: conn.ai_cli_command.clone(),
        });
    }

    let results = ssh_manager.connect_all(configs, app);

    let session_infos: Vec<SessionInfo> = results
        .into_iter()
        .map(|(idx, result)| {
            let conn = &workset.connections[idx];
            match result {
                Ok(session_id) => SessionInfo {
                    session_id,
                    connection_index: idx,
                    host: format!("{}@{}:{}", conn.user, conn.host, conn.port),
                    status: "connecting".to_string(),
                },
                Err(e) => SessionInfo {
                    session_id: String::new(),
                    connection_index: idx,
                    host: format!("{}@{}:{}", conn.user, conn.host, conn.port),
                    status: format!("error: {e}"),
                },
            }
        })
        .collect();

    Ok(session_infos)
}

#[tauri::command]
async fn deactivate_workset(
    ssh_manager: tauri::State<'_, SshConnectionManager>,
) -> Result<(), String> {
    ssh_manager.disconnect_all();
    Ok(())
}

#[tauri::command]
fn terminal_input(
    session_id: String,
    data: String,
    ssh_manager: tauri::State<'_, SshConnectionManager>,
) -> Result<(), String> {
    ssh_manager
        .send_input(&session_id, data.into_bytes())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn terminal_resize(
    session_id: String,
    cols: u32,
    rows: u32,
    ssh_manager: tauri::State<'_, SshConnectionManager>,
) -> Result<(), String> {
    ssh_manager
        .resize(&session_id, cols, rows)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn terminal_reconnect(
    session_id: String,
    ssh_manager: tauri::State<'_, SshConnectionManager>,
) -> Result<(), String> {
    ssh_manager
        .reconnect(&session_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_directory(
    session_id: String,
    path: String,
    ssh_manager: tauri::State<'_, SshConnectionManager>,
) -> Result<Vec<FileEntry>, String> {
    ssh_manager
        .list_directory(&session_id, path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn read_file(
    session_id: String,
    path: String,
    max_bytes: Option<u64>,
    ssh_manager: tauri::State<'_, SshConnectionManager>,
) -> Result<ReadFileResult, String> {
    ssh_manager
        .read_file(&session_id, path, max_bytes)
        .map_err(|e| e.to_string())
}

// ── App Entry ──

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(WorksetStore::new())
        .manage(SshConnectionManager::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_worksets,
            get_workset,
            create_workset,
            update_workset,
            delete_workset,
            activate_workset,
            deactivate_workset,
            terminal_input,
            terminal_resize,
            terminal_reconnect,
            list_directory,
            read_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
