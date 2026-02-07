use serde::Serialize;
use ssh2::{Channel, Session};
use std::io::Read;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tauri::Emitter;

#[derive(Serialize, Clone, Debug)]
pub struct FileEntry {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub mtime_epoch: Option<u64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ReadFileResult {
    pub path: String,
    pub bytes: Vec<u8>,
    pub truncated: bool,
}

#[derive(Debug)]
pub enum SshError {
    TcpConnect(String),
    Handshake(String),
    Auth(String),
    Channel(String),
    Pty(String),
    Send(String),
    SessionNotFound,
}

// Implement Display for SshError (follow pattern from src-tauri/src/workset/mod.rs:87-100)
impl std::fmt::Display for SshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SshError::TcpConnect(e) => write!(f, "TCP connect failed: {e}"),
            SshError::Handshake(e) => write!(f, "SSH handshake failed: {e}"),
            SshError::Auth(e) => write!(f, "SSH authentication failed: {e}"),
            SshError::Channel(e) => write!(f, "SSH channel error: {e}"),
            SshError::Pty(e) => write!(f, "PTY error: {e}"),
            SshError::Send(e) => write!(f, "send error: {e}"),
            SshError::SessionNotFound => write!(f, "session not found"),
        }
    }
}

impl std::error::Error for SshError {}

pub enum SessionCommand {
    Write(Vec<u8>),
    Resize { cols: u32, rows: u32 },
    ListDirectory {
        path: String,
        reply_tx: mpsc::Sender<Result<Vec<FileEntry>, String>>,
    },
    ReadFile {
        path: String,
        max_bytes: Option<u64>,
        reply_tx: mpsc::Sender<Result<ReadFileResult, String>>,
    },
    Shutdown,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Connecting,
    Connected,
    Disconnected,
    Error(String),
}

pub struct SshSessionConfig {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub auth_method: String, // "key" or "password"
    pub key_path: Option<String>,
    pub password: Option<String>,
    pub project_path: String,
    pub ai_cli_command: Option<String>,
}

pub struct SshSessionHandle {
    pub id: String,
    pub host_display: String,
    cmd_tx: mpsc::Sender<SessionCommand>,
    worker: Option<JoinHandle<()>>,
}

impl SshSessionHandle {
    pub fn spawn(
        config: SshSessionConfig,
        app_handle: tauri::AppHandle,
    ) -> Result<Self, SshError> {
        let (cmd_tx, cmd_rx) = mpsc::channel::<SessionCommand>();

        let id = config.id.clone();
        let host_display = format!("{}@{}:{}", config.user, config.host, config.port);
        let app_handle_for_thread = app_handle.clone();

        let worker = thread::spawn(move || session_worker(config, app_handle_for_thread, cmd_rx));

        Ok(Self {
            id,
            host_display,
            cmd_tx,
            worker: Some(worker),
        })
    }

    pub fn send_input(&self, data: Vec<u8>) -> Result<(), SshError> {
        self.cmd_tx
            .send(SessionCommand::Write(data))
            .map_err(|e| SshError::Send(e.to_string()))
    }

    pub fn resize(&self, cols: u32, rows: u32) -> Result<(), SshError> {
        self.cmd_tx
            .send(SessionCommand::Resize { cols, rows })
            .map_err(|e| SshError::Send(e.to_string()))
    }

    pub fn shutdown(&mut self) {
        let _ = self.cmd_tx.send(SessionCommand::Shutdown);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }

    pub fn list_directory(&self, path: String) -> Result<Vec<FileEntry>, SshError> {
        let (reply_tx, reply_rx) = mpsc::channel::<Result<Vec<FileEntry>, String>>();
        self.cmd_tx
            .send(SessionCommand::ListDirectory { path, reply_tx })
            .map_err(|e| SshError::Send(e.to_string()))?;

        reply_rx
            .recv_timeout(Duration::from_secs(15))
            .map_err(|e| SshError::Channel(format!("list_directory response timeout: {e}")))?
            .map_err(SshError::Channel)
    }

    pub fn read_file(&self, path: String, max_bytes: Option<u64>) -> Result<ReadFileResult, SshError> {
        let (reply_tx, reply_rx) = mpsc::channel::<Result<ReadFileResult, String>>();
        self.cmd_tx
            .send(SessionCommand::ReadFile {
                path,
                max_bytes,
                reply_tx,
            })
            .map_err(|e| SshError::Send(e.to_string()))?;

        reply_rx
            .recv_timeout(Duration::from_secs(30))
            .map_err(|e| SshError::Channel(format!("read_file response timeout: {e}")))?
            .map_err(SshError::Channel)
    }
}

fn stat_is_dir(perm: Option<u32>) -> bool {
    // POSIX file mode bits. When available, ssh2 exposes st_mode via FileStat.perm.
    // https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/sys_stat.h.html
    const S_IFMT: u32 = 0o170000;
    const S_IFDIR: u32 = 0o040000;
    match perm {
        Some(p) => (p & S_IFMT) == S_IFDIR,
        None => false,
    }
}

fn sftp_list_directory(sess: &Session, path: &str) -> Result<Vec<FileEntry>, String> {
    let sftp = sess.sftp().map_err(|e| format!("sftp init: {e}"))?;
    let entries = sftp
        .readdir(Path::new(path))
        .map_err(|e| format!("sftp readdir {path}: {e}"))?;

    let mut out = Vec::new();
    for (p, stat) in entries {
        let name = p
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| p.to_string_lossy().to_string());

        if name == "." || name == ".." {
            continue;
        }

        out.push(FileEntry {
            path: p.to_string_lossy().to_string(),
            name,
            is_dir: stat_is_dir(stat.perm),
            size_bytes: stat.size.unwrap_or(0),
            mtime_epoch: stat.mtime,
        });
    }

    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

fn sftp_read_file(sess: &Session, path: &str, max_bytes: Option<u64>) -> Result<ReadFileResult, String> {
    const DEFAULT_MAX_BYTES: u64 = 1024 * 1024; // 1 MiB
    let limit = max_bytes.unwrap_or(DEFAULT_MAX_BYTES);

    let sftp = sess.sftp().map_err(|e| format!("sftp init: {e}"))?;
    let mut file = sftp
        .open(Path::new(path))
        .map_err(|e| format!("sftp open {path}: {e}"))?;

    let mut out: Vec<u8> = Vec::new();
    let mut buf = [0u8; 8192];
    while (out.len() as u64) < limit {
        let remaining = (limit - out.len() as u64) as usize;
        let to_read = std::cmp::min(buf.len(), remaining);
        let n = file
            .read(&mut buf[..to_read])
            .map_err(|e| format!("sftp read {path}: {e}"))?;
        if n == 0 {
            break;
        }
        out.extend_from_slice(&buf[..n]);
    }

    // If we hit the limit exactly, probe one more byte to determine truncation.
    // Do not include the byte in output to preserve the requested maximum size.
    let truncated = if (out.len() as u64) < limit {
        false
    } else {
        let mut one = [0u8; 1];
        match file.read(&mut one) {
            Ok(0) => false,
            Ok(_) => true,
            Err(e) => return Err(format!("sftp read {path}: {e}")),
        }
    };

    Ok(ReadFileResult {
        path: path.to_string(),
        truncated,
        bytes: out,
    })
}

fn session_worker(
    config: SshSessionConfig,
    app_handle: tauri::AppHandle,
    cmd_rx: mpsc::Receiver<SessionCommand>,
) {
    let status_event = format!("session-status-{}", config.id);
    let output_event = format!("terminal-output-{}", config.id);

    let emit_status = |status: SessionStatus| {
        let _ = app_handle.emit(&status_event, status);
    };

    emit_status(SessionStatus::Connecting);

    // 2) DNS resolution
    let addr_str = format!("{}:{}", config.host, config.port);
    let sock_addr = match addr_str.to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(a) => a,
            None => {
                emit_status(SessionStatus::Error(format!("no addresses for {addr_str}")));
                return;
            }
        },
        Err(e) => {
            emit_status(SessionStatus::Error(format!("resolve {addr_str}: {e}")));
            return;
        }
    };

    // 3) TCP connect with timeout
    let tcp = match TcpStream::connect_timeout(&sock_addr, Duration::from_secs(10)) {
        Ok(t) => t,
        Err(e) => {
            emit_status(SessionStatus::Error(format!("tcp connect {addr_str}: {e}")));
            return;
        }
    };

    // 4) low latency
    if let Err(e) = tcp.set_nodelay(true) {
        emit_status(SessionStatus::Error(format!("tcp set_nodelay: {e}")));
        return;
    }

    // 5) SSH handshake
    let mut sess = match Session::new() {
        Ok(s) => s,
        Err(e) => {
            emit_status(SessionStatus::Error(format!("Session::new: {e}")));
            return;
        }
    };

    let tcp_clone = match tcp.try_clone() {
        Ok(t) => t,
        Err(e) => {
            emit_status(SessionStatus::Error(format!("clone tcp: {e}")));
            return;
        }
    };
    sess.set_tcp_stream(tcp_clone);
    if let Err(e) = sess.handshake() {
        emit_status(SessionStatus::Error(format!("handshake {addr_str}: {e}")));
        return;
    }

    // 6) Auth dispatch
    match config.auth_method.as_str() {
        "key" => {
            let key_path = match config.key_path.as_deref() {
                Some(p) if !p.is_empty() => p,
                _ => {
                    emit_status(SessionStatus::Error(
                        "key auth requires key_path".to_string(),
                    ));
                    return;
                }
            };

            if let Err(e) = sess.userauth_pubkey_file(
                &config.user,
                None,
                Path::new(key_path),
                None,
            ) {
                emit_status(SessionStatus::Error(format!("auth {addr_str}: {e}")));
                return;
            }
        }
        "password" => {
            let password = match config.password.as_deref() {
                Some(p) if !p.is_empty() => p,
                _ => {
                    emit_status(SessionStatus::Error(
                        "password auth requires password".to_string(),
                    ));
                    return;
                }
            };

            if let Err(e) = sess.userauth_password(&config.user, password) {
                emit_status(SessionStatus::Error(format!("auth {addr_str}: {e}")));
                return;
            }
        }
        other => {
            emit_status(SessionStatus::Error(format!(
                "unsupported auth method: {other}"
            )));
            return;
        }
    }

    // 7) Verify authentication
    if !sess.authenticated() {
        emit_status(SessionStatus::Error(format!(
            "not authenticated on {addr_str}"
        )));
        return;
    }

    // 8) keepalive
    sess.set_keepalive(true, 15);

    // 9) PTY + shell
    let mut channel: Channel = match sess.channel_session() {
        Ok(c) => c,
        Err(e) => {
            emit_status(SessionStatus::Error(format!("channel_session: {e}")));
            return;
        }
    };

    if let Err(e) = channel.request_pty("xterm-256color", None, Some((80, 24, 0, 0))) {
        emit_status(SessionStatus::Error(format!("request_pty: {e}")));
        return;
    }

    if let Err(e) = channel.shell() {
        emit_status(SessionStatus::Error(format!("shell: {e}")));
        return;
    }

    // 10) cd project path
    if !config.project_path.is_empty() {
        if let Err(e) = channel.write_all(format!("cd {}\n", config.project_path).as_bytes()) {
            emit_status(SessionStatus::Error(format!("write cd command: {e}")));
            return;
        }
    }

    // 11) optionally launch AI CLI
    if let Some(cmd) = config.ai_cli_command.as_deref() {
        if !cmd.is_empty() {
            if let Err(e) = channel.write_all(format!("{}\n", cmd).as_bytes()) {
                emit_status(SessionStatus::Error(format!("write ai cli command: {e}")));
                return;
            }
        }
    }

    // 12) connected
    emit_status(SessionStatus::Connected);

    // 13) non-blocking reads
    sess.set_blocking(false);

    // 14) main loop
    loop {
        match cmd_rx.try_recv() {
            Ok(SessionCommand::Write(data)) => {
                sess.set_blocking(true);
                if let Err(e) = channel.write_all(&data) {
                    emit_status(SessionStatus::Error(format!("channel write: {e}")));
                    break;
                }
                sess.set_blocking(false);
            }
            Ok(SessionCommand::Resize { cols, rows }) => {
                let _ = channel.request_pty_size(cols, rows, None, None);
            }
            Ok(SessionCommand::ListDirectory { path, reply_tx }) => {
                sess.set_blocking(true);
                let result = sftp_list_directory(&sess, &path);
                sess.set_blocking(false);
                let _ = reply_tx.send(result);
            }
            Ok(SessionCommand::ReadFile {
                path,
                max_bytes,
                reply_tx,
            }) => {
                sess.set_blocking(true);
                let result = sftp_read_file(&sess, &path, max_bytes);
                sess.set_blocking(false);
                let _ = reply_tx.send(result);
            }
            Ok(SessionCommand::Shutdown) => break,
            Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {
                // proceed to read
            }
        }

        let mut buf = [0u8; 4096];
        match channel.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let _ = app_handle.emit(&output_event, buf[..n].to_vec());
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // no data
            }
            Err(e) => {
                emit_status(SessionStatus::Error(format!("channel read: {e}")));
                break;
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    // 15) cleanup
    let _ = channel.close();
    let _ = channel.wait_close();
    emit_status(SessionStatus::Disconnected);
}
