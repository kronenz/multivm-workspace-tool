use std::fs::OpenOptions;
use std::io::{BufRead, BufWriter, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use chrono::Utc;
use clap::Parser;
use serde::Serialize;
use ssh2::Session;

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

#[derive(Parser, Clone)]
#[command(
    name = "spike_2_ssh_harness",
    about = "SPIKE-2: Stress-test 10 concurrent SSH sessions with reconnect and JSONL logging"
)]
struct Cli {
    #[arg(
        long,
        required = true,
        value_delimiter = ',',
        help = "Comma-separated host:port targets (repeat one entry to fill 10 slots)"
    )]
    targets: Vec<String>,

    #[arg(long, help = "SSH username")]
    user: String,

    #[arg(long, help = "Path to SSH private key file (contents never stored)")]
    key: PathBuf,

    #[arg(
        long,
        default_value = "",
        help = "Key passphrase (WARNING: visible in process list/history; empty = none)"
    )]
    passphrase: String,

    #[arg(
        long,
        default_value_t = 1800,
        help = "Total run duration in seconds (default 1800 = 30 min)"
    )]
    duration_secs: u64,

    #[arg(long, default_value_t = 5, help = "Exec poll interval in seconds")]
    poll_interval_secs: u64,

    #[arg(
        long,
        default_value = "medium",
        help = "PTY output intensity: low / medium / high"
    )]
    intensity: String,

    #[arg(long, help = "Inject disconnect after this many seconds (0 = disabled)")]
    disconnect_after_secs: Option<u64>,

    #[arg(long, default_value = "spike2_results.jsonl", help = "JSONL output log path")]
    log_path: PathBuf,

    #[arg(long, default_value_t = 3, help = "Max reconnect attempts per disconnect")]
    max_reconnect_attempts: u32,

    #[arg(long, default_value_t = 15, help = "Reconnect budget in seconds (disconnect->usable)")]
    reconnect_timeout_secs: u64,
}

// ---------------------------------------------------------------------------
// JSONL event types
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct LogEvent {
    ts: String,
    session_id: usize,
    host: String,
    event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elapsed_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attempt: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success: Option<bool>,
}

type LogWriter = Arc<Mutex<BufWriter<std::fs::File>>>;

fn log_event(writer: &LogWriter, evt: &LogEvent) {
    if let Ok(mut w) = writer.lock() {
        if let Ok(line) = serde_json::to_string(evt) {
            let _ = writeln!(w, "{}", line);
        }
    }
}

fn flush_log(writer: &LogWriter) {
    if let Ok(mut w) = writer.lock() {
        let _ = w.flush();
    }
}

// ---------------------------------------------------------------------------
// SSH helpers
// ---------------------------------------------------------------------------

fn parse_host_port(target: &str) -> (String, u16) {
    if let Some(idx) = target.rfind(':') {
        let host = target[..idx].to_string();
        let port = target[idx + 1..].parse::<u16>().unwrap_or(22);
        (host, port)
    } else {
        (target.to_string(), 22)
    }
}

fn connect_session(
    host: &str,
    port: u16,
    user: &str,
    key: &PathBuf,
    passphrase: &str,
    timeout: Duration,
) -> Result<(TcpStream, Session), String> {
    let addr = format!("{}:{}", host, port);
    let sock_addr = addr
        .to_socket_addrs()
        .map_err(|e| format!("resolve {}: {}", addr, e))?
        .next()
        .ok_or_else(|| format!("no addresses for {}", addr))?;
    let tcp = TcpStream::connect_timeout(&sock_addr, timeout)
        .map_err(|e| format!("tcp connect {}: {}", addr, e))?;

    tcp.set_read_timeout(Some(timeout))
        .map_err(|e| format!("set_read_timeout: {}", e))?;
    tcp.set_write_timeout(Some(timeout))
        .map_err(|e| format!("set_write_timeout: {}", e))?;

    let mut sess = Session::new().map_err(|e| format!("Session::new: {}", e))?;
    sess.set_tcp_stream(tcp.try_clone().map_err(|e| format!("clone tcp: {}", e))?);
    sess.handshake()
        .map_err(|e| format!("handshake {}: {}", addr, e))?;

    let pass: Option<&str> = if passphrase.is_empty() {
        None
    } else {
        Some(passphrase)
    };
    sess.userauth_pubkey_file(user, None, key.as_path(), pass)
        .map_err(|e| format!("auth {}: {}", addr, e))?;

    if !sess.authenticated() {
        return Err(format!("not authenticated on {}", addr));
    }

    sess.set_keepalive(true, 15);
    Ok((tcp, sess))
}

// ---------------------------------------------------------------------------
// Workload runners
// ---------------------------------------------------------------------------

fn pty_workload_cmd(intensity: &str) -> &'static str {
    match intensity {
        "low" => "for i in $(seq 1 5); do echo pty-heartbeat-$i; done",
        "high" => "yes 'pty-flood-line' | head -n 200",
        _ => "for i in $(seq 1 20); do echo pty-line-$i; done",
    }
}

fn run_pty_cycle(sess: &Session, intensity: &str) -> Result<usize, String> {
    let mut channel = sess
        .channel_session()
        .map_err(|e| format!("channel_session: {}", e))?;
    channel
        .request_pty("xterm", None, None)
        .map_err(|e| format!("request_pty: {}", e))?;
    channel
        .exec(pty_workload_cmd(intensity))
        .map_err(|e| format!("exec pty: {}", e))?;

    let mut buf = vec![0u8; 4096];
    let mut total_bytes = 0usize;
    loop {
        match channel.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => total_bytes += n,
            Err(e) => return Err(format!("pty read: {}", e)),
        }
    }
    channel.wait_close().map_err(|e| format!("pty close: {}", e))?;
    Ok(total_bytes)
}

fn run_exec_poll(sess: &Session) -> Result<String, String> {
    let mut channel = sess
        .channel_session()
        .map_err(|e| format!("channel_session: {}", e))?;
    channel
        .exec("echo cpu=$(cat /proc/stat 2>/dev/null | head -1) mem=$(cat /proc/meminfo 2>/dev/null | head -3 | tr '\\n' ' ') disk=$(df -h / 2>/dev/null | tail -1) || echo 'resource-poll-fallback'")
        .map_err(|e| format!("exec poll: {}", e))?;

    let mut output = String::new();
    channel
        .read_to_string(&mut output)
        .map_err(|e| format!("poll read: {}", e))?;
    channel.wait_close().map_err(|e| format!("poll close: {}", e))?;
    Ok(output.trim().to_string())
}

// ---------------------------------------------------------------------------
// Session worker
// ---------------------------------------------------------------------------

struct SessionResult {
    id: usize,
    host: String,
    port: u16,
    connected_once: bool,
    total_polls: u64,
    total_pty_bytes: usize,
    abnormal_ends: u32,
    reconnect_attempts: u32,
    reconnect_successes: u32,
    reconnect_times_ms: Vec<u64>,
}

fn session_worker(
    id: usize,
    target: String,
    cli: Cli,
    start: Instant,
    duration: Duration,
    running: Arc<AtomicBool>,
    inject_disconnect: Arc<AtomicBool>,
    log: LogWriter,
) -> SessionResult {
    let (host, port) = parse_host_port(&target);
    let host_port = format!("{}:{}", host, port);

    let poll_interval = Duration::from_secs(cli.poll_interval_secs);
    let reconnect_budget = Duration::from_secs(cli.reconnect_timeout_secs);

    let mut sess: Option<Session> = None;
    let mut _tcp: Option<TcpStream> = None;
    let mut connected = false;
    let mut connected_once = false;
    let mut injection_handled = false;

    let mut total_polls = 0u64;
    let mut total_pty_bytes = 0usize;
    let mut abnormal_ends = 0u32;

    let mut reconnect_attempts_total = 0u32;
    let mut reconnect_successes = 0u32;
    let mut reconnect_times_ms: Vec<u64> = Vec::new();

    let mut disconnected_at: Option<Instant> = None;
    let mut attempts_since_disconnect = 0u32;
    let mut last_poll = Instant::now() - poll_interval;

    // Initial connect attempt
    let connect_start = Instant::now();
    match connect_session(
        &host,
        port,
        &cli.user,
        &cli.key,
        &cli.passphrase,
        reconnect_budget,
    ) {
        Ok((tcp, s)) => {
            connected = true;
            connected_once = true;
            sess = Some(s);
            _tcp = Some(tcp);
            log_event(
                &log,
                &LogEvent {
                    ts: Utc::now().to_rfc3339(),
                    session_id: id,
                    host: host_port.clone(),
                    event: "connected".into(),
                    detail: None,
                    elapsed_ms: Some(connect_start.elapsed().as_millis() as u64),
                    attempt: None,
                    success: Some(true),
                },
            );
            eprintln!("[session-{}] connected to {} ({}ms)", id, host_port, connect_start.elapsed().as_millis());
        }
        Err(e) => {
            abnormal_ends += 1;
            disconnected_at = Some(Instant::now());
            attempts_since_disconnect = 0;
            log_event(
                &log,
                &LogEvent {
                    ts: Utc::now().to_rfc3339(),
                    session_id: id,
                    host: host_port.clone(),
                    event: "connect_failed".into(),
                    detail: Some(e.clone()),
                    elapsed_ms: Some(connect_start.elapsed().as_millis() as u64),
                    attempt: None,
                    success: Some(false),
                },
            );
            eprintln!("[session-{}] FAILED to connect to {}: {}", id, host_port, e);
        }
    }

    while running.load(Ordering::Relaxed) && start.elapsed() < duration {
        let now = Instant::now();

        if inject_disconnect.load(Ordering::Relaxed) && connected && !injection_handled {
            injection_handled = true;
            connected = false;
            sess = None;
            _tcp = None;
            disconnected_at = Some(Instant::now());
            attempts_since_disconnect = 0;
            log_event(
                &log,
                &LogEvent {
                    ts: Utc::now().to_rfc3339(),
                    session_id: id,
                    host: host_port.clone(),
                    event: "disconnect_injected".into(),
                    detail: None,
                    elapsed_ms: Some(start.elapsed().as_millis() as u64),
                    attempt: None,
                    success: None,
                },
            );
        }

        if !connected {
            if attempts_since_disconnect >= cli.max_reconnect_attempts {
                thread::sleep(Duration::from_millis(200));
                continue;
            }

            let jitter_ms = (id as u64 * 500) + 1000;
            if let Some(disc_at) = disconnected_at {
                if disc_at.elapsed() < Duration::from_millis(jitter_ms) {
                    thread::sleep(Duration::from_millis(50));
                    continue;
                }
            }

            attempts_since_disconnect += 1;
            reconnect_attempts_total += 1;
            let attempt = attempts_since_disconnect;

            let rc_start = Instant::now();
            match connect_session(
                &host,
                port,
                &cli.user,
                &cli.key,
                &cli.passphrase,
                reconnect_budget,
            ) {
                Ok((tcp, s)) => {
                    connected = true;
                    connected_once = true;
                    sess = Some(s);
                    _tcp = Some(tcp);
                    reconnect_successes += 1;

                    if let Some(disc_at) = disconnected_at {
                        let wall_ms = disc_at.elapsed().as_millis() as u64;
                        reconnect_times_ms.push(wall_ms);
                        log_event(
                            &log,
                            &LogEvent {
                                ts: Utc::now().to_rfc3339(),
                                session_id: id,
                                host: host_port.clone(),
                                event: "reconnected".into(),
                                detail: Some(format!(
                                    "wall_ms={} connect_ms={}",
                                    wall_ms,
                                    rc_start.elapsed().as_millis()
                                )),
                                elapsed_ms: Some(wall_ms),
                                attempt: Some(attempt),
                                success: Some(true),
                            },
                        );
                    } else {
                        log_event(
                            &log,
                            &LogEvent {
                                ts: Utc::now().to_rfc3339(),
                                session_id: id,
                                host: host_port.clone(),
                                event: "connected".into(),
                                detail: None,
                                elapsed_ms: Some(rc_start.elapsed().as_millis() as u64),
                                attempt: Some(attempt),
                                success: Some(true),
                            },
                        );
                    }

                    disconnected_at = None;
                    attempts_since_disconnect = 0;
                }
                Err(e) => {
                    log_event(
                        &log,
                        &LogEvent {
                            ts: Utc::now().to_rfc3339(),
                            session_id: id,
                            host: host_port.clone(),
                            event: "reconnect_failed".into(),
                            detail: Some(e.clone()),
                            elapsed_ms: Some(rc_start.elapsed().as_millis() as u64),
                            attempt: Some(attempt),
                            success: Some(false),
                        },
                    );
                    eprintln!("[session-{}] reconnect failed to {}: {}", id, host_port, e);
                }
            }

            thread::sleep(Duration::from_millis(200));
            continue;
        }

        let Some(ref sref) = sess else {
            thread::sleep(Duration::from_millis(200));
            continue;
        };

        if now.duration_since(last_poll) >= poll_interval {
            match run_exec_poll(sref) {
                Ok(out) => {
                    total_polls += 1;
                    last_poll = now;
                    log_event(
                        &log,
                        &LogEvent {
                            ts: Utc::now().to_rfc3339(),
                            session_id: id,
                            host: host_port.clone(),
                            event: "exec_poll".into(),
                            detail: Some(out.chars().take(200).collect()),
                            elapsed_ms: None,
                            attempt: None,
                            success: Some(true),
                        },
                    );
                }
                Err(e) => {
                    abnormal_ends += 1;
                    connected = false;
                    sess = None;
                    _tcp = None;
                    disconnected_at = Some(Instant::now());
                    attempts_since_disconnect = 0;
                    log_event(
                        &log,
                        &LogEvent {
                            ts: Utc::now().to_rfc3339(),
                            session_id: id,
                            host: host_port.clone(),
                            event: "exec_poll_error".into(),
                            detail: Some(e),
                            elapsed_ms: None,
                            attempt: None,
                            success: Some(false),
                        },
                    );
                    continue;
                }
            }
        }

        match run_pty_cycle(sref, &cli.intensity) {
            Ok(bytes) => {
                total_pty_bytes += bytes;
                log_event(
                    &log,
                    &LogEvent {
                        ts: Utc::now().to_rfc3339(),
                        session_id: id,
                        host: host_port.clone(),
                        event: "pty_cycle".into(),
                        detail: Some(format!("{} bytes", bytes)),
                        elapsed_ms: None,
                        attempt: None,
                        success: Some(true),
                    },
                );
            }
            Err(e) => {
                abnormal_ends += 1;
                connected = false;
                sess = None;
                _tcp = None;
                disconnected_at = Some(Instant::now());
                attempts_since_disconnect = 0;
                log_event(
                    &log,
                    &LogEvent {
                        ts: Utc::now().to_rfc3339(),
                        session_id: id,
                        host: host_port.clone(),
                        event: "pty_error".into(),
                        detail: Some(e),
                        elapsed_ms: None,
                        attempt: None,
                        success: Some(false),
                    },
                );
            }
        }

        thread::sleep(Duration::from_millis(200));
    }

    SessionResult {
        id,
        host,
        port,
        connected_once,
        total_polls,
        total_pty_bytes,
        abnormal_ends,
        reconnect_attempts: reconnect_attempts_total,
        reconnect_successes,
        reconnect_times_ms,
    }
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct SummaryReport {
    total_runtime_secs: f64,
    sessions: Vec<SessionSummary>,
    global_reconnect_attempts: u32,
    global_reconnect_successes: u32,
    global_reconnect_rate_pct: f64,
    global_reconnect_p50_ms: u64,
    global_reconnect_p95_ms: u64,
}

#[derive(Serialize)]
struct SessionSummary {
    session_id: usize,
    host: String,
    connected_once: bool,
    total_polls: u64,
    total_pty_bytes: usize,
    abnormal_ends: u32,
    reconnect_attempts: u32,
    reconnect_successes: u32,
    reconnect_rate_pct: f64,
    reconnect_p50_ms: u64,
    reconnect_p95_ms: u64,
}

fn percentile(values: &[u64], pct: f64) -> u64 {
    if values.is_empty() {
        return 0;
    }
    let mut data = values.to_vec();
    data.sort_unstable();
    let idx = ((pct / 100.0) * (data.len() as f64 - 1.0)).round() as usize;
    data[idx.min(data.len() - 1)]
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let cli = Cli::parse();

    let mut targets: Vec<String> = cli.targets.clone();
    while targets.len() < 10 {
        let wrap = targets[targets.len() % cli.targets.len()].clone();
        targets.push(wrap);
    }
    targets.truncate(10);

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&cli.log_path)
        .expect("failed to open log file");
    let log: LogWriter = Arc::new(Mutex::new(BufWriter::new(log_file)));

    let running = Arc::new(AtomicBool::new(true));
    setup_shutdown_listener(&running);

    let start = Instant::now();
    let duration = Duration::from_secs(cli.duration_secs);

    let inject_disconnect = Arc::new(AtomicBool::new(false));
    if let Some(secs) = cli.disconnect_after_secs {
        if secs > 0 {
            let flag = inject_disconnect.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(secs));
                flag.store(true, Ordering::Relaxed);
            });
        }
    }

    eprintln!(
        "[spike-2] Starting {} sessions for {} seconds",
        targets.len(),
        cli.duration_secs
    );

    let (tx, rx) = mpsc::channel::<SessionResult>();
    let mut handles = Vec::with_capacity(10);
    for (id, target) in targets.into_iter().enumerate() {
        let tx = tx.clone();
        let cli_clone = cli.clone();
        let running = running.clone();
        let inject_disconnect = inject_disconnect.clone();
        let log = log.clone();
        let start_copy = start;

        let handle = thread::spawn(move || {
            let res = session_worker(
                id,
                target,
                cli_clone,
                start_copy,
                duration,
                running,
                inject_disconnect,
                log,
            );
            let _ = tx.send(res);
        });
        handles.push(handle);
    }
    drop(tx);

    let mut results: Vec<SessionResult> = Vec::with_capacity(10);
    for r in rx.iter() {
        results.push(r);
        if results.len() == 10 {
            break;
        }
    }
    for h in handles {
        let _ = h.join();
    }

    let total_elapsed = start.elapsed();
    eprintln!("\n[spike-2] ===== SUMMARY =====");
    eprintln!("[spike-2] Total runtime: {:.1}s", total_elapsed.as_secs_f64());

    let mut all_reconnect_times: Vec<u64> = Vec::new();
    let mut summary = SummaryReport {
        total_runtime_secs: total_elapsed.as_secs_f64(),
        sessions: Vec::new(),
        global_reconnect_attempts: 0,
        global_reconnect_successes: 0,
        global_reconnect_rate_pct: 0.0,
        global_reconnect_p50_ms: 0,
        global_reconnect_p95_ms: 0,
    };

    for s in &results {
        let rc_rate = if s.reconnect_attempts > 0 {
            (s.reconnect_successes as f64 / s.reconnect_attempts as f64) * 100.0
        } else {
            0.0
        };
        let p50 = percentile(&s.reconnect_times_ms, 50.0);
        let p95 = percentile(&s.reconnect_times_ms, 95.0);

        all_reconnect_times.extend_from_slice(&s.reconnect_times_ms);
        summary.global_reconnect_attempts += s.reconnect_attempts;
        summary.global_reconnect_successes += s.reconnect_successes;

        eprintln!(
            "  session-{}: host={}:{} connected_once={} polls={} pty_bytes={} abnormal={} rc_attempts={} rc_ok={} rc_rate={:.0}% rc_p50={}ms rc_p95={}ms",
            s.id,
            s.host,
            s.port,
            s.connected_once,
            s.total_polls,
            s.total_pty_bytes,
            s.abnormal_ends,
            s.reconnect_attempts,
            s.reconnect_successes,
            rc_rate,
            p50,
            p95
        );

        summary.sessions.push(SessionSummary {
            session_id: s.id,
            host: format!("{}:{}", s.host, s.port),
            connected_once: s.connected_once,
            total_polls: s.total_polls,
            total_pty_bytes: s.total_pty_bytes,
            abnormal_ends: s.abnormal_ends,
            reconnect_attempts: s.reconnect_attempts,
            reconnect_successes: s.reconnect_successes,
            reconnect_rate_pct: rc_rate,
            reconnect_p50_ms: p50,
            reconnect_p95_ms: p95,
        });
    }

    if summary.global_reconnect_attempts > 0 {
        summary.global_reconnect_rate_pct =
            (summary.global_reconnect_successes as f64 / summary.global_reconnect_attempts as f64)
                * 100.0;
    }
    summary.global_reconnect_p50_ms = percentile(&all_reconnect_times, 50.0);
    summary.global_reconnect_p95_ms = percentile(&all_reconnect_times, 95.0);

    log_event(
        &log,
        &LogEvent {
            ts: Utc::now().to_rfc3339(),
            session_id: 0,
            host: "harness".into(),
            event: "summary".into(),
            detail: serde_json::to_string(&summary).ok(),
            elapsed_ms: Some(total_elapsed.as_millis() as u64),
            attempt: None,
            success: None,
        },
    );
    flush_log(&log);

    let total_abnormal: u32 = results.iter().map(|s| s.abnormal_ends).sum();
    let connected_once_count = results.iter().filter(|s| s.connected_once).count();
    let uptime_ok = total_abnormal == 0 && connected_once_count == 10;

    let reconnect_tested = cli.disconnect_after_secs.unwrap_or(0) > 0;
    let reconnect_ok = if reconnect_tested {
        summary.global_reconnect_rate_pct >= 90.0 && summary.global_reconnect_p95_ms <= 15_000
    } else {
        true
    };

    eprintln!("\n[spike-2] PASS/FAIL checklist:");
    eprintln!(
        "  [{}] 10 sessions 30min uptime (connected_once: {}/10, abnormal ends: {})",
        if uptime_ok { "PASS" } else { "FAIL" },
        connected_once_count,
        total_abnormal
    );

    if reconnect_tested {
        eprintln!(
            "  [{}] Reconnect >= 90% and p95 <= 15s (rate: {:.1}%, ok/attempts: {}/{}, p95={}ms)",
            if reconnect_ok { "PASS" } else { "FAIL" },
            summary.global_reconnect_rate_pct,
            summary.global_reconnect_successes,
            summary.global_reconnect_attempts,
            summary.global_reconnect_p95_ms
        );
    } else {
        eprintln!("  [N/A ] Reconnect test skipped (no --disconnect-after-secs)");
    }

    eprintln!("  [INFO] Memory/CPU: check externally (top, htop, /proc/self/status)");
    eprintln!("[spike-2] Log written to: {}", cli.log_path.display());

    if !uptime_ok || !reconnect_ok {
        std::process::exit(1);
    }
}

fn setup_shutdown_listener(flag: &Arc<AtomicBool>) {
    let f = flag.clone();
    let _ = thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) if l.trim() == "q" || l.trim() == "quit" => {
                    eprintln!("[spike-2] Graceful shutdown requested via stdin");
                    f.store(false, Ordering::Relaxed);
                    break;
                }
                Err(_) => break,
                _ => {}
            }
        }
    });
}
