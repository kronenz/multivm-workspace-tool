pub mod session;

pub use session::{FileEntry, ReadFileResult, SessionCommand, SessionStatus, SshError, SshSessionConfig, SshSessionHandle};

use std::collections::HashMap;
use std::sync::Mutex;

pub struct SshConnectionManager {
    sessions: Mutex<HashMap<String, SshSessionHandle>>,
}

impl SshConnectionManager {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    pub fn connect(
        &self,
        config: SshSessionConfig,
        app_handle: tauri::AppHandle,
    ) -> Result<String, SshError> {
        let handle = SshSessionHandle::spawn(config, app_handle)?;
        let id = handle.id.clone();
        let mut sessions = self.sessions.lock().map_err(|_| {
            SshError::Channel("session lock poisoned".to_string())
        })?;
        sessions.insert(id.clone(), handle);
        Ok(id)
    }

    pub fn connect_all(
        &self,
        configs: Vec<SshSessionConfig>,
        app_handle: tauri::AppHandle,
    ) -> Vec<(usize, Result<String, SshError>)> {
        let results: Vec<(usize, Result<SshSessionHandle, SshError>)> = std::thread::scope(|s| {
            let handles: Vec<_> = configs
                .into_iter()
                .enumerate()
                .map(|(idx, config)| {
                    let app = app_handle.clone();
                    s.spawn(move || (idx, SshSessionHandle::spawn(config, app)))
                })
                .collect();

            handles
                .into_iter()
                .map(|h| {
                    h.join().unwrap_or_else(|_| {
                        // If thread panicked, return a descriptive error
                        // We need to figure out what idx was — use a sentinel
                        (usize::MAX, Err(SshError::Channel("worker thread panicked".to_string())))
                    })
                })
                .collect()
        });

        let mut sessions = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
        let mut output = Vec::new();

        for (idx, result) in results {
            match result {
                Ok(handle) => {
                    let id = handle.id.clone();
                    sessions.insert(id.clone(), handle);
                    output.push((idx, Ok(id)));
                }
                Err(e) => {
                    output.push((idx, Err(e)));
                }
            }
        }

        output
    }

    pub fn send_input(&self, session_id: &str, data: Vec<u8>) -> Result<(), SshError> {
        let sessions = self.sessions.lock().map_err(|_| {
            SshError::Channel("session lock poisoned".to_string())
        })?;
        let handle = sessions.get(session_id).ok_or(SshError::SessionNotFound)?;
        handle.send_input(data)
    }

    pub fn resize(&self, session_id: &str, cols: u32, rows: u32) -> Result<(), SshError> {
        let sessions = self.sessions.lock().map_err(|_| {
            SshError::Channel("session lock poisoned".to_string())
        })?;
        let handle = sessions.get(session_id).ok_or(SshError::SessionNotFound)?;
        handle.resize(cols, rows)
    }

    pub fn list_directory(&self, session_id: &str, path: String) -> Result<Vec<FileEntry>, SshError> {
        let sessions = self.sessions.lock().map_err(|_| {
            SshError::Channel("session lock poisoned".to_string())
        })?;
        let handle = sessions.get(session_id).ok_or(SshError::SessionNotFound)?;
        handle.list_directory(path)
    }

    pub fn read_file(
        &self,
        session_id: &str,
        path: String,
        max_bytes: Option<u64>,
    ) -> Result<ReadFileResult, SshError> {
        let sessions = self.sessions.lock().map_err(|_| {
            SshError::Channel("session lock poisoned".to_string())
        })?;
        let handle = sessions.get(session_id).ok_or(SshError::SessionNotFound)?;
        handle.read_file(path, max_bytes)
    }

    pub fn disconnect(&self, session_id: &str) -> Result<(), SshError> {
        let mut sessions = self.sessions.lock().map_err(|_| {
            SshError::Channel("session lock poisoned".to_string())
        })?;
        let mut handle = sessions.remove(session_id).ok_or(SshError::SessionNotFound)?;
        handle.shutdown();
        Ok(())
    }

    pub fn disconnect_all(&self) {
        let mut sessions = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
        let all: Vec<(String, SshSessionHandle)> = sessions.drain().collect();
        drop(sessions);
        for (_, mut handle) in all {
            handle.shutdown();
        }
    }

    pub fn active_sessions(&self) -> Vec<(String, String)> {
        let sessions = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
        sessions
            .iter()
            .map(|(id, handle)| (id.clone(), handle.host_display.clone()))
            .collect()
    }
}

impl Drop for SshConnectionManager {
    fn drop(&mut self) {
        self.disconnect_all();
    }
}
