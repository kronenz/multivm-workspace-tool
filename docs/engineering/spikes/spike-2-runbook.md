# SPIKE-2 Runbook: SSH Connection Pooling Stress Test

## Prerequisites

- **Rust** toolchain (rustc 1.70+, cargo)
- **Linux**: `sudo apt-get install libssh2-1-dev pkg-config` (required by the `ssh2` crate)
- **macOS**: `brew install libssh2`
- **Windows**: vcpkg with `libssh2` or use pre-built binaries
- **SSH target(s)**: at least one host reachable via SSH key auth

Notes:
- The harness does **not** verify host keys (known_hosts). Run only in a trusted environment.

## Build

```bash
cargo build --manifest-path src-tauri/Cargo.toml --bin spike_2_ssh_harness --release
```

The binary is at `src-tauri/target/release/spike_2_ssh_harness`.

For a debug build (faster compile, slower runtime):

```bash
cargo build --manifest-path src-tauri/Cargo.toml --bin spike_2_ssh_harness
```

## Quick Start

### Option A: 10 different VMs (realistic topology)

```bash
cargo run --manifest-path src-tauri/Cargo.toml --bin spike_2_ssh_harness -- \
  --targets "vm1.example.com:22,vm2.example.com:22,vm3.example.com:22,vm4.example.com:22,vm5.example.com:22,vm6.example.com:22,vm7.example.com:22,vm8.example.com:22,vm9.example.com:22,vm10.example.com:22" \
  --user ubuntu \
  --key ~/.ssh/id_ed25519 \
  --duration-secs 1800 \
  --log-path spike2_run_A.jsonl
```

### Option B: 1 host repeated 10 times (easy setup)

```bash
cargo run --manifest-path src-tauri/Cargo.toml --bin spike_2_ssh_harness -- \
  --targets "myhost.example.com:22" \
  --user ubuntu \
  --key ~/.ssh/id_ed25519 \
  --duration-secs 1800 \
  --log-path spike2_run_B.jsonl
```

When fewer than 10 targets are given, the harness automatically repeats the list to fill 10 slots.

### With disconnect injection

```bash
cargo run --manifest-path src-tauri/Cargo.toml --bin spike_2_ssh_harness -- \
  --targets "myhost.example.com:22" \
  --user ubuntu \
  --key ~/.ssh/id_ed25519 \
  --duration-secs 1800 \
  --disconnect-after-secs 300 \
  --max-reconnect-attempts 5 \
  --log-path spike2_run_disconnect.jsonl
```

This drops all sessions at the 5-minute mark and exercises the reconnect loop with per-session jitter.

## CLI Reference

| Flag | Default | Description |
|------|---------|-------------|
| `--targets` | (required) | Comma-separated `host:port` list; auto-padded to 10 |
| `--user` | (required) | SSH username |
| `--key` | (required) | Path to SSH private key (contents never stored) |
| `--passphrase` | `""` | Key passphrase (WARNING: visible in process list/history) |
| `--duration-secs` | `1800` | Total run duration (30 min default) |
| `--poll-interval-secs` | `5` | Exec poll interval |
| `--intensity` | `medium` | PTY output intensity: `low` / `medium` / `high` |
| `--disconnect-after-secs` | (none) | Inject disconnect on all sessions after N seconds |
| `--max-reconnect-attempts` | `3` | Max reconnect tries per session per disconnect event |
| `--reconnect-timeout-secs` | `15` | TCP connect timeout for reconnect |
| `--log-path` | `spike2_results.jsonl` | JSONL output file |

## Graceful Shutdown

Type `q` or `quit` + Enter on stdin to stop the harness early. It will print the summary and flush the log.

## Workload Model

The harness runs **10 session workers concurrently** (one thread per session).

Per session:

1. **Exec Polling** (every `--poll-interval-secs`): Runs `cat /proc/stat`, `/proc/meminfo`, `df -h /` to simulate the Resource Poller.
2. **PTY I/O**: Opens a PTY channel and runs a command whose output volume depends on `--intensity`:
   - `low`: 5 echo lines
   - `medium`: 20 echo lines
   - `high`: 200 lines from `yes` (burst)

## JSONL Log Format

Each line is a JSON object:

```json
{
  "ts": "2026-02-07T12:00:00.123456+00:00",
  "session_id": 3,
  "host": "myhost.example.com:22",
  "event": "exec_poll",
  "detail": "cpu=... mem=... disk=...",
  "elapsed_ms": null,
  "attempt": null,
  "success": true
}
```

Event types: `connected`, `connect_failed`, `disconnect_injected`, `reconnected`, `reconnect_failed`, `exec_poll`, `exec_poll_error`, `pty_cycle`, `pty_error`, `summary`.

## Recording Pass/Fail

After each run, evaluate against the criteria in [spike-2-ssh-pooling-stress.md](./spike-2-ssh-pooling-stress.md):

| Criterion | How to verify | Pass threshold |
|-----------|---------------|----------------|
| 10 sessions × 30 min | Check summary: `abnormal_ends` across all sessions | 0 |
| Reconnect ≥ 90% within 15s | Check summary: `global_reconnect_rate_pct` and `global_reconnect_p95_ms` | ≥90%, p95 ≤ 15000ms |
| Memory < 200MB | Run `htop` or `cat /proc/<pid>/status \| grep VmRSS` during the test | < 200MB |
| CPU < 5% | Run `top -p <pid>` or `pidstat -p <pid> 5` during the test | < 5% |

### Extracting metrics from JSONL

```bash
# Count reconnect events
grep '"reconnected"' spike2_results.jsonl | wc -l

# Count failed reconnects
grep '"reconnect_failed"' spike2_results.jsonl | wc -l

# Extract summary line
grep '"summary"' spike2_results.jsonl | python3 -m json.tool

# Reconnect times
grep '"reconnected"' spike2_results.jsonl | python3 -c "
import sys, json
times = [json.loads(l)['elapsed_ms'] for l in sys.stdin]
if times:
    times.sort()
    print(f'p50={times[len(times)//2]}ms  p95={times[int(len(times)*0.95)]}ms  max={max(times)}ms')
"
```

### Data recording table

Copy the template from [spike-2-ssh-pooling-stress.md § 데이터 기록 테이블](./spike-2-ssh-pooling-stress.md) and fill in after each run.

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| `libssh2-sys` build failure | Install `libssh2-1-dev` and `pkg-config` (Ubuntu) |
| `Permission denied (publickey)` | Verify `--key` path points to valid private key; check `--user` |
| `Connection refused` on target | Ensure sshd is running and port is correct |
| All sessions disconnect immediately | Check server `MaxSessions` in `/etc/ssh/sshd_config` (default 10 may need increase) |
| Harness seems stuck | PTY workload with `high` intensity can block if remote `yes` is slow; try `medium` |

## File Map

| Path | Purpose |
|------|---------|
| `src-tauri/src/bin/spike_2_ssh_harness.rs` | Harness binary source |
| `src-tauri/Cargo.toml` | Dependencies: `ssh2`, `clap`, `chrono` |
| `docs/engineering/spikes/spike-2-ssh-pooling-stress.md` | Spike plan with success criteria and data tables |
| `docs/engineering/spikes/spike-2-runbook.md` | This file |
