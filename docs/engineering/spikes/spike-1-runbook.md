# SPIKE-1 Runbook: Tauri + xterm.js Latency Prototype

## Prerequisites

- **Rust** toolchain (rustc 1.70+, cargo)
- **Node.js** 18+ and npm
- **Linux**: `sudo apt-get install build-essential libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`
- **macOS**: Xcode Command Line Tools
- **Windows**: Visual Studio Build Tools + WebView2 runtime

## Quick Start

```bash
# 1. Install frontend dependencies
npm install

# 2. Run in dev mode (launches both Vite dev server and Tauri window)
npm run tauri dev

# 3. Alternatively, build for production
npm run build                          # frontend only
cargo check --manifest-path src-tauri/Cargo.toml  # rust type-check
npm run tauri build                    # full desktop bundle
```

## What the Prototype Does

The app opens a single window with:

1. **Control bar** at the top with buttons for Flood and Stream tests
2. **Stats panel** showing real-time latency percentiles and renderer info
3. **xterm.js terminal** filling the rest of the window

### Test 1: Keystroke Echo Latency

Type characters in the terminal. Each keystroke:
- Captures `keydown` timestamp on the terminal container (capture phase)
- Calls Rust `echo_key` command via Tauri invoke
- Writes the returned character to xterm.js
- Waits for xterm to process the write + 2 animation frames (approximate "next paint")
- Records `(painted - keydown)` in milliseconds

The stats panel shows p50/p95/p99 in real time.

**Pass criterion**: p95 < 50ms

### Test 2: 10,000-Line Flood

Click **Run Flood** (default 10,000 lines). The Rust backend emits output
in chunks (to reduce event overhead) and publishes a `terminal-flood-done`
event when emission completes. The frontend stops the timer when it sees
the done event and at least one frame has been presented.

Both Rust-side (emit loop) and frontend-side (end-to-end) elapsed times are reported.

**Pass criterion**: Total render time for 10k lines < 100ms rendering lag
(i.e., the terminal remains responsive after flood completes)

### Test 3: Streamed Output

Click **Run Stream** to emit lines at a configurable delay (0ms allowed).
Useful for
observing steady-state rendering behavior and checking for memory leaks
during sustained output.

## Recording Results

Fill in the data table in `docs/engineering/spikes/spike-1-tauri-xterm-latency.md`.

For each OS / renderer / terminal-count combination:

1. Launch the app
2. Type ~50 characters to warm up, then clear stats
3. Type ~100 characters and record keystroke latency p50/p95/p99
4. Run the 10k-line flood and record timing
5. Note renderer type shown in stats panel (webgl vs canvas)
6. Record approximate CPU/memory from OS task manager

## Renderer Detection

The stats panel shows `webgl` or `canvas (webgl unavailable)`. If WebGL
fails to initialize (common on some Linux WebKitGTK builds), the app
automatically falls back to the Canvas renderer. Record which renderer
was active during each test run.

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| `error: linker cc not found` | Install `build-essential` (Linux) |
| `webkit2gtk not found` | Install `libwebkit2gtk-4.1-dev` (Linux) |
| Blank window | Check `npm run dev` is running; inspect WebView console |
| WebGL not available | Expected on some Linux configs; Canvas fallback is fine |
| `npm run tauri dev` hangs | Ensure port 1420 is free |

## File Map

| Path | Purpose |
|------|---------|
| `src/main.ts` | Frontend entry: xterm.js setup, IPC wiring, latency measurement |
| `src/latency.ts` | LatencyRecorder class with percentile calculation |
| `src/styles.css` | Minimal dark theme for the spike UI |
| `index.html` | App shell with controls and terminal container |
| `src-tauri/src/lib.rs` | Rust backend: echo_key, start_flood, start_stream commands |
| `src-tauri/tauri.conf.json` | Tauri v2 configuration |
