mod workset;
mod ssh;
mod settings;
mod keystore;

use serde::Serialize;
use ssh2_config::{ParseRule, SshConfig};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use uuid::Uuid;
use workset::{AuthMethod, CreateWorksetInput, UpdateWorksetInput, Workset, WorksetStore, WorksetSummary};
use ssh::{FileEntry, ReadFileResult, SshConnectionManager, SshSessionConfig};
use settings::{AppSettings, SettingsStore};

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

// ── Settings Commands ──

#[tauri::command]
fn get_settings(store: tauri::State<'_, SettingsStore>) -> AppSettings {
    store.get()
}

#[tauri::command]
fn set_theme(theme: String, store: tauri::State<'_, SettingsStore>) -> Result<AppSettings, String> {
    let parsed = SettingsStore::parse_theme(&theme).map_err(|e| e.to_string())?;
    store.set_theme_result(parsed).map_err(|e| e.to_string())
}

// ── OS Keystore Commands (SSH password storage) ──

#[tauri::command]
fn store_ssh_password(host: String, user: String, password: String) -> Result<(), String> {
    keystore::store_password(&host, &user, &password)
}

#[tauri::command]
fn retrieve_ssh_password(host: String, user: String) -> Result<Option<String>, String> {
    keystore::retrieve_password(&host, &user)
}

#[tauri::command]
fn delete_ssh_password(host: String, user: String) -> Result<(), String> {
    keystore::delete_password(&host, &user)
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
    let mut parsed_ssh_config: Option<SshConfig> = None;
    let mut ssh_config_path: Option<PathBuf> = None;
    let mut ssh_home_dir: Option<PathBuf> = None;
    for (i, conn) in workset.connections.iter().enumerate() {
        let (host, port, user, auth_method_str, key_path) = match conn.auth_method {
            AuthMethod::Key => (
                conn.host.clone(),
                conn.port,
                conn.user.clone(),
                "key".to_string(),
                conn.key_path.clone(),
            ),
            AuthMethod::Password => (
                conn.host.clone(),
                conn.port,
                conn.user.clone(),
                "password".to_string(),
                conn.key_path.clone(),
            ),
            AuthMethod::SshConfig => {
                let home = if let Some(h) = ssh_home_dir.as_ref() {
                    h.clone()
                } else {
                    let h = dirs::home_dir()
                        .ok_or_else(|| "unable to resolve home directory for ~/.ssh/config".to_string())?;
                    ssh_home_dir = Some(h.clone());
                    h
                };

                let config_path = if let Some(p) = ssh_config_path.as_ref() {
                    p.clone()
                } else {
                    let p = home.join(".ssh").join("config");
                    ssh_config_path = Some(p.clone());
                    p
                };

                if !config_path.exists() {
                    return Err(format!(
                        "SSH config auth selected but config file not found: {}",
                        config_path.display()
                    ));
                }

                if parsed_ssh_config.is_none() {
                    let file = File::open(&config_path)
                        .map_err(|e| format!("open {}: {e}", config_path.display()))?;
                    let mut reader = BufReader::new(file);
                    let cfg = SshConfig::default()
                        .parse(&mut reader, ParseRule::STRICT)
                        .map_err(|e| format!("parse {}: {e}", config_path.display()))?;
                    parsed_ssh_config = Some(cfg);
                }

                let cfg = parsed_ssh_config
                    .as_ref()
                    .ok_or_else(|| "internal error: ssh config missing after parse".to_string())?;
                let params = cfg.query(&conn.host);

                let resolved_host = params.host_name.clone().unwrap_or_else(|| conn.host.clone());
                let resolved_port = params.port.unwrap_or(conn.port);
                let resolved_user = params.user.clone().unwrap_or_else(|| conn.user.clone());

                let normalize_identity_path = |p: &PathBuf| -> PathBuf {
                    if let Some(s) = p.to_str() {
                        if let Some(rest) = s.strip_prefix("~/") {
                            return home.join(rest);
                        }
                    }
                    if p.is_absolute() {
                        return p.clone();
                    }
                    // OpenSSH treats relative IdentityFile paths as relative to ~/.ssh.
                    home.join(".ssh").join(p)
                };

                let mut resolved_key_path: Option<String> = None;
                if let Some(files) = params.identity_file.as_ref() {
                    for p in files {
                        let candidate = normalize_identity_path(p);
                        if candidate.exists() {
                            resolved_key_path = Some(candidate.to_string_lossy().to_string());
                            break;
                        }
                    }
                }

                if resolved_key_path.is_none() {
                    let defaults = [
                        home.join(".ssh").join("id_rsa"),
                        home.join(".ssh").join("id_ed25519"),
                    ];
                    for p in defaults {
                        if p.exists() {
                            resolved_key_path = Some(p.to_string_lossy().to_string());
                            break;
                        }
                    }
                }

                let resolved_key_path = resolved_key_path.ok_or_else(|| {
                    format!(
                        "SSH config auth selected for host alias '{}' but no IdentityFile found (and no default ~/.ssh/id_rsa or ~/.ssh/id_ed25519 present)",
                        conn.host
                    )
                })?;

                (
                    resolved_host,
                    resolved_port,
                    resolved_user,
                    "key".to_string(),
                    Some(resolved_key_path),
                )
            }
        };

        let password = if auth_method_str == "password" {
            passwords.get(i).cloned().flatten()
        } else {
            None
        };

        configs.push(SshSessionConfig {
            id: Uuid::new_v4().to_string(),
            host,
            port,
            user,
            auth_method: auth_method_str,
            key_path,
            password,
            project_path: conn.project_path.clone(),
            ai_cli_command: conn.ai_cli_command.clone(),
            keepalive_interval_secs: conn.keepalive_interval_secs,
            reconnect_max_retries: conn.reconnect_max_retries,
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
fn restart_ai_cli(
    session_id: String,
    ssh_manager: tauri::State<'_, SshConnectionManager>,
) -> Result<(), String> {
    ssh_manager
        .restart_ai_cli(&session_id)
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
        .manage(SettingsStore::new())
        .manage(SshConnectionManager::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_worksets,
            get_workset,
            create_workset,
            update_workset,
            delete_workset,
            get_settings,
            set_theme,
            store_ssh_password,
            retrieve_ssh_password,
            delete_ssh_password,
            activate_workset,
            deactivate_workset,
            terminal_input,
            terminal_resize,
            terminal_reconnect,
            restart_ai_cli,
            list_directory,
            read_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
