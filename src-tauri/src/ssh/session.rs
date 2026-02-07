use serde::Serialize;
use ssh2::{Channel, Session};
use std::io::Read;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
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

#[derive(Serialize, Clone, Debug)]
pub struct ResourceSnapshot {
    pub cpu_percent: Option<f64>,
    pub ram_percent: Option<f64>,
    pub disk_percent: Option<f64>,
    pub ts_epoch: u64,
    pub disk_path: String,
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
    ReconnectNow,
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
    Reconnecting { attempt: u32, max: u32 },
    ReconnectFailed,
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

    pub fn reconnect_now(&self) -> Result<(), SshError> {
        self.cmd_tx
            .send(SessionCommand::ReconnectNow)
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

fn exec_read_to_string(sess: &Session, cmd: &str) -> Result<String, String> {
    let mut channel = sess
        .channel_session()
        .map_err(|e| format!("channel_session: {e}"))?;

    channel.exec(cmd).map_err(|e| format!("exec {cmd}: {e}"))?;

    let mut out = String::new();
    channel
        .read_to_string(&mut out)
        .map_err(|e| format!("read stdout: {e}"))?;

    let mut err_out = String::new();
    let _ = channel.stderr().read_to_string(&mut err_out);

    let _ = channel.close();
    let _ = channel.wait_close();

    if !err_out.trim().is_empty() {
        if !out.ends_with('\n') {
            out.push('\n');
        }
        out.push_str(&err_out);
    }

    Ok(out)
}

fn parse_cpu_percent_from_top(output: &str) -> Option<f64> {
    let line = output
        .lines()
        .find(|l| l.contains("Cpu(s)") || l.contains("%Cpu(s)") || l.contains("CPU:"))?;

    if line.contains("Cpu(s)") || line.contains("%Cpu(s)") {
        let parts = line.split(':').nth(1).unwrap_or(line);
        let mut idle: Option<f64> = None;
        for seg in parts.split(',') {
            let s = seg.trim();
            let fields: Vec<&str> = s.split_whitespace().collect();
            if fields.len() >= 2 && fields[1] == "id" {
                if let Ok(v) = fields[0].parse::<f64>() {
                    idle = Some(v);
                    break;
                }
            }
        }
        idle.map(|id| (100.0 - id).max(0.0).min(100.0))
    } else {
        // Busybox-style: "CPU:  0% usr  0% sys  0% nic 99% idle  0% io  0% irq  0% sirq"
        let mut iter = line.split_whitespace().peekable();
        while let Some(tok) = iter.next() {
            if tok.ends_with('%') {
                let val_str = tok.trim_end_matches('%');
                if let Some(label) = iter.peek().copied() {
                    if label == "idle" {
                        if let Ok(idle) = val_str.parse::<f64>() {
                            return Some((100.0 - idle).max(0.0).min(100.0));
                        }
                    }
                }
            }
        }
        None
    }
}

fn parse_ram_percent_from_free(output: &str) -> Option<f64> {
    let line = output.lines().find(|l| l.trim_start().starts_with("Mem:"))?;
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 3 {
        return None;
    }

    let total = fields.get(1)?.parse::<f64>().ok()?;
    if total <= 0.0 {
        return None;
    }

    // `free -m` Mem line: total is fields[1], used is fields[2]
    let used = fields.get(2)?.parse::<f64>().ok()?;
    Some(((used / total) * 100.0).max(0.0).min(100.0))
}

fn parse_disk_percent_from_df(output: &str) -> Option<f64> {
    let line = output.lines().skip(1).find(|l| !l.trim().is_empty())?;
    let fields: Vec<&str> = line.split_whitespace().collect();
    let use_field = fields.iter().find(|f| f.ends_with('%'))?;
    let v = use_field.trim_end_matches('%').parse::<f64>().ok()?;
    Some(v.max(0.0).min(100.0))
}

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs()
}

fn collect_resource_snapshot(sess: &Session, project_path: &str) -> ResourceSnapshot {
    let cpu_output = exec_read_to_string(sess, "LANG=C top -bn1");
    let free_output = exec_read_to_string(sess, "LANG=C free -m");

    let preferred_path = if project_path.trim().is_empty() {
        "/"
    } else {
        project_path.trim()
    };

    let df_cmd = format!("LANG=C df -P {}", shell_escape(preferred_path));
    let mut disk_path = preferred_path.to_string();
    let mut df_output = exec_read_to_string(sess, &df_cmd);
    if df_output.is_err() && preferred_path != "/" {
        disk_path = "/".to_string();
        df_output = exec_read_to_string(sess, "LANG=C df -P /");
    }

    let cpu_percent = cpu_output.ok().and_then(|o| parse_cpu_percent_from_top(&o));
    let ram_percent = free_output.ok().and_then(|o| parse_ram_percent_from_free(&o));
    let disk_percent = df_output.ok().and_then(|o| parse_disk_percent_from_df(&o));

    ResourceSnapshot {
        cpu_percent,
        ram_percent,
        disk_percent,
        ts_epoch: now_epoch(),
        disk_path,
    }
}

fn shell_escape(value: &str) -> String {
    // Single-quote escaping for /bin/sh -c. Safe for most POSIX shells.
    let mut out = String::new();
    out.push('\'');
    for ch in value.chars() {
        if ch == '\'' {
            out.push_str("'\\''");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

fn session_worker(
    config: SshSessionConfig,
    app_handle: tauri::AppHandle,
    cmd_rx: mpsc::Receiver<SessionCommand>,
) {
    const MAX_RETRIES: u32 = 3;
    const RETRY_SCHEDULE_SECS: [u64; 3] = [0, 5, 10];

    let status_event = format!("session-status-{}", config.id);
    let output_event = format!("terminal-output-{}", config.id);
    let resource_event = format!("resource-update-{}", config.id);

    let emit_status = |status: SessionStatus| {
        let _ = app_handle.emit(&status_event, status);
    };

    let jitter_ms: u64 = config
        .id
        .as_bytes()
        .iter()
        .fold(0u64, |acc, b| acc.wrapping_add(*b as u64))
        % 400;

    let is_auth_error = |msg: &str| {
        msg.contains("auth")
            || msg.contains("not authenticated")
            || msg.contains("key auth requires")
            || msg.contains("password auth requires")
    };

    let mut initial_connect = true;
    let mut disconnect_started = Instant::now();
    let mut next_attempt_at = Instant::now();
    let mut next_attempt_num: u32 = 0;

    let mut pty_cols: u32 = 80;
    let mut pty_rows: u32 = 24;

    // Helper to establish a fresh SSH session + interactive shell.
    let connect_shell =
        |run_ai_cli: bool, pty_cols: u32, pty_rows: u32| -> Result<(Session, Channel), String> {
        // DNS resolution
        let addr_str = format!("{}:{}", config.host, config.port);
        let sock_addr = match addr_str.to_socket_addrs() {
            Ok(mut addrs) => addrs.next().ok_or_else(|| format!("no addresses for {addr_str}"))?,
            Err(e) => return Err(format!("resolve {addr_str}: {e}")),
        };

        // TCP connect with timeout
        let tcp = TcpStream::connect_timeout(&sock_addr, Duration::from_secs(10))
            .map_err(|e| format!("tcp connect {addr_str}: {e}"))?;
        tcp.set_nodelay(true)
            .map_err(|e| format!("tcp set_nodelay: {e}"))?;

        // SSH handshake
        let mut sess = Session::new().map_err(|e| format!("Session::new: {e}"))?;
        let tcp_clone = tcp.try_clone().map_err(|e| format!("clone tcp: {e}"))?;
        sess.set_tcp_stream(tcp_clone);
        sess.handshake().map_err(|e| format!("handshake {addr_str}: {e}"))?;

        // Auth dispatch
        match config.auth_method.as_str() {
            "key" => {
                let key_path = config
                    .key_path
                    .as_deref()
                    .filter(|p| !p.is_empty())
                    .ok_or_else(|| "key auth requires key_path".to_string())?;

                sess.userauth_pubkey_file(&config.user, None, Path::new(key_path), None)
                    .map_err(|e| format!("auth {addr_str}: {e}"))?;
            }
            "password" => {
                let password = config
                    .password
                    .as_deref()
                    .filter(|p| !p.is_empty())
                    .ok_or_else(|| "password auth requires password".to_string())?;

                sess.userauth_password(&config.user, password)
                    .map_err(|e| format!("auth {addr_str}: {e}"))?;
            }
            other => return Err(format!("unsupported auth method: {other}")),
        }

        if !sess.authenticated() {
            return Err(format!("not authenticated on {addr_str}"));
        }

        sess.set_keepalive(true, 15);

        // PTY + shell
        let mut channel: Channel = sess
            .channel_session()
            .map_err(|e| format!("channel_session: {e}"))?;
        channel
            .request_pty("xterm-256color", None, Some((pty_cols, pty_rows, 0, 0)))
            .map_err(|e| format!("request_pty: {e}"))?;
        channel.shell().map_err(|e| format!("shell: {e}"))?;

        // Restore working directory
        if !config.project_path.is_empty() {
            channel
                .write_all(format!("cd {}\n", config.project_path).as_bytes())
                .map_err(|e| format!("write cd command: {e}"))?;
        }

        // Only auto-run AI CLI on first activation.
        if run_ai_cli {
            if let Some(cmd) = config.ai_cli_command.as_deref() {
                if !cmd.is_empty() {
                    channel
                        .write_all(format!("{}\n", cmd).as_bytes())
                        .map_err(|e| format!("write ai cli command: {e}"))?;
                }
            }
        }

        Ok((sess, channel))
    };

    // Outer loop: connect → run until drop/shutdown → reconnect as needed.
    'outer: loop {
        // Decide which state we are in.
        if initial_connect {
            emit_status(SessionStatus::Connecting);
        } else if next_attempt_num > 0 {
            emit_status(SessionStatus::Reconnecting {
                attempt: next_attempt_num,
                max: MAX_RETRIES,
            });
        }

        // Wait until it's time to attempt a (re)connect.
        while !initial_connect && Instant::now() < next_attempt_at {
            match cmd_rx.try_recv() {
                Ok(SessionCommand::Shutdown) | Err(TryRecvError::Disconnected) => {
                    emit_status(SessionStatus::Disconnected);
                    break 'outer;
                }
                Ok(SessionCommand::ReconnectNow) => {
                    // Manual reconnect request: attempt immediately.
                    disconnect_started = Instant::now();
                    next_attempt_num = 1;
                    break;
                }
                Ok(SessionCommand::Resize { cols, rows }) => {
                    pty_cols = cols;
                    pty_rows = rows;
                }
                Ok(SessionCommand::ListDirectory { reply_tx, .. }) => {
                    let _ = reply_tx.send(Err("not connected".to_string()));
                }
                Ok(SessionCommand::ReadFile { reply_tx, .. }) => {
                    let _ = reply_tx.send(Err("not connected".to_string()));
                }
                Ok(_) => {
                    // Ignore terminal input while disconnected.
                }
                Err(TryRecvError::Empty) => {
                    // wait
                }
            }

            std::thread::sleep(Duration::from_millis(50));
        }

        // Attempt connection.
        let run_ai_cli = initial_connect;
        let connect_result = connect_shell(run_ai_cli, pty_cols, pty_rows);

        let (sess, mut channel) = match connect_result {
            Ok(v) => v,
            Err(msg) => {
                emit_status(SessionStatus::Error(msg.clone()));

                // Auth failures stop auto-reconnect and require manual action.
                if initial_connect || is_auth_error(&msg) {
                    emit_status(SessionStatus::ReconnectFailed);
                    // Manual wait loop
                    loop {
                        match cmd_rx.try_recv() {
                            Ok(SessionCommand::Shutdown) | Err(TryRecvError::Disconnected) => {
                                emit_status(SessionStatus::Disconnected);
                                break 'outer;
                            }
                            Ok(SessionCommand::ReconnectNow) => {
                                initial_connect = false;
                                disconnect_started = Instant::now();
                                next_attempt_num = 1;
                                break;
                            }
                            Ok(SessionCommand::Resize { cols, rows }) => {
                                pty_cols = cols;
                                pty_rows = rows;
                            }
                            Ok(SessionCommand::ListDirectory { reply_tx, .. }) => {
                                let _ = reply_tx.send(Err("not connected".to_string()));
                            }
                            Ok(SessionCommand::ReadFile { reply_tx, .. }) => {
                                let _ = reply_tx.send(Err("not connected".to_string()));
                            }
                            Ok(_) => {}
                            Err(TryRecvError::Empty) => {}
                        }
                        std::thread::sleep(Duration::from_millis(80));
                    }
                    continue 'outer;
                }

                // Transient failures: keep retrying within budget.
                if next_attempt_num >= MAX_RETRIES {
                    emit_status(SessionStatus::ReconnectFailed);
                    // Manual wait
                    loop {
                        match cmd_rx.try_recv() {
                            Ok(SessionCommand::Shutdown) | Err(TryRecvError::Disconnected) => {
                                emit_status(SessionStatus::Disconnected);
                                break 'outer;
                            }
                            Ok(SessionCommand::ReconnectNow) => {
                                disconnect_started = Instant::now();
                                next_attempt_num = 1;
                                break;
                            }
                            Ok(SessionCommand::Resize { cols, rows }) => {
                                pty_cols = cols;
                                pty_rows = rows;
                            }
                            Ok(SessionCommand::ListDirectory { reply_tx, .. }) => {
                                let _ = reply_tx.send(Err("not connected".to_string()));
                            }
                            Ok(SessionCommand::ReadFile { reply_tx, .. }) => {
                                let _ = reply_tx.send(Err("not connected".to_string()));
                            }
                            Ok(_) => {}
                            Err(TryRecvError::Empty) => {}
                        }
                        std::thread::sleep(Duration::from_millis(80));
                    }
                    continue 'outer;
                }

                // Schedule next attempt.
                let next = (next_attempt_num + 1).min(MAX_RETRIES);
                let schedule_idx = (next.saturating_sub(1)) as usize;
                let delay = RETRY_SCHEDULE_SECS.get(schedule_idx).copied().unwrap_or(10);
                next_attempt_num = next;
                next_attempt_at = disconnect_started
                    + Duration::from_secs(delay)
                    + Duration::from_millis(jitter_ms);
                continue 'outer;
            }
        };

        // Connected
        emit_status(SessionStatus::Connected);
        initial_connect = false;

        sess.set_blocking(false);
        let mut last_resource_emit = Instant::now()
            .checked_sub(Duration::from_secs(5))
            .unwrap_or_else(Instant::now);

        // Connected main loop
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
                    pty_cols = cols;
                    pty_rows = rows;
                    let _ = channel.request_pty_size(cols, rows, None, None);
                }
                Ok(SessionCommand::ReconnectNow) => {
                    // Ignore while connected.
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
                Ok(SessionCommand::Shutdown) => {
                    let _ = channel.close();
                    let _ = channel.wait_close();
                    emit_status(SessionStatus::Disconnected);
                    break 'outer;
                }
                Err(TryRecvError::Disconnected) => {
                    let _ = channel.close();
                    let _ = channel.wait_close();
                    emit_status(SessionStatus::Disconnected);
                    break 'outer;
                }
                Err(TryRecvError::Empty) => {}
            }

            let mut buf = [0u8; 4096];
            match channel.read(&mut buf) {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    let _ = app_handle.emit(&output_event, buf[..n].to_vec());
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => {
                    emit_status(SessionStatus::Error(format!("channel read: {e}")));
                    break;
                }
            }

            if last_resource_emit.elapsed() >= Duration::from_secs(5) {
                sess.set_blocking(true);
                let snapshot = collect_resource_snapshot(&sess, &config.project_path);
                sess.set_blocking(false);
                let _ = app_handle.emit(&resource_event, snapshot);
                last_resource_emit = Instant::now();
            }

            std::thread::sleep(Duration::from_millis(10));
        }

        // Cleanup on drop.
        let _ = channel.close();
        let _ = channel.wait_close();

        disconnect_started = Instant::now();
        next_attempt_num = 1;
        next_attempt_at = disconnect_started
            + Duration::from_secs(RETRY_SCHEDULE_SECS[0])
            + Duration::from_millis(jitter_ms);
        continue 'outer;
    }
}
