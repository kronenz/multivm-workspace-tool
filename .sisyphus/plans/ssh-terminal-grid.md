# SSH Connection + Terminal Emulator + Grid Layout (MVP Features 2/3/4)

## TL;DR

> **Quick Summary**: Implement the core interactive backbone of the Multi-VM Workspace app — SSH connections managed in Rust, xterm.js terminals rendered in a CSS Grid, all wired through Tauri IPC. This gives users a working "click Activate → see terminals connected to VMs" flow.
>
> **Deliverables**:
> - Rust SSH module (`ssh/session.rs`, `ssh/mod.rs`) with per-session worker threads
> - Frontend grid layout engine with 5 preset layouts
> - xterm.js terminal pane component with WebGL rendering
> - 4 new Tauri IPC commands for terminal I/O
> - End-to-end workset activation flow
>
> **Estimated Effort**: Large (8 tasks across 4 parallel waves)
> **Parallel Execution**: YES — 4 waves, max 3 concurrent tasks
> **Critical Path**: Task 1 → Task 4 → Task 6 → Task 8

---

## Context

### Original Request
Implement MVP Features 2 (SSH Connection), 3 (Terminal Emulator), and 4 (Grid Layout) for the Tauri v2 desktop app. These 3 features are tightly coupled — SSH provides data, terminals render it, grid arranges them — and must be planned together to maximize parallelism.

### Current Codebase State
- **Rust backend**: `lib.rs` (55 lines, 5 sync Tauri commands), `workset/mod.rs` (420 lines, full CRUD + validation)
- **Frontend**: `main.ts` (642 lines, monolithic Workset CRUD UI), `styles.css` (551 lines, dark theme with CSS custom properties), `index.html` (35 lines)
- **Data model**: `ConnectionConfig { host, port, user, auth_method (Key|Password|SshConfig), key_path, project_path, ai_cli_command }` — NO password field
- **Dependencies already present**: `ssh2 = "0.9"`, `tokio = { features = ["time"] }`, `@xterm/xterm ^5.5`, `@xterm/addon-webgl ^0.18`, `@xterm/addon-fit ^0.10`
- **SPIKE-2 validated**: ssh2 handles 10 concurrent sessions with reconnect; patterns in `spike_2_ssh_harness.rs`

### Interview Summary
**Key Decisions**:
- SSH sessions use `std::thread` (NOT tokio tasks) because ssh2 is synchronous and not Send-safe across await points
- Each worker thread communicates via `std::sync::mpsc` for commands, `AppHandle.emit()` for output
- All new Tauri commands are `async fn` (existing CRUD commands are sync — DO NOT follow that pattern)
- Frontend terminal/grid code goes in separate `.ts` modules, NOT appended to the 642-line `main.ts`
- Password is runtime-only (prompted on activation, NEVER persisted to JSON) per NFR-12/13
- Test strategy: Tests-after — verify via `cargo check` / `npm run build` / `cargo build --release` only

**Research Findings**:
- xterm.js init order: FitAddon load → `open(container)` → `fit()` → WebGL addon (try/catch → Canvas fallback)
- Terminal write batching via `requestAnimationFrame` prevents WebView freeze under heavy output
- `AppHandle` is `Send + Sync` — safe to call `.emit()` from `std::thread`
- SPIKE-2 connect pattern: `TcpStream::connect_timeout` → `Session::new` → `set_tcp_stream` → `handshake` → `userauth_*` → `set_keepalive(true, 15)`
- xterm.css MUST be imported for proper terminal rendering

### Metis Review
**Identified Gaps** (all addressed in plan):
- UI state machine for workspace activation (empty/detail/form → workspace active) — addressed in Task 7
- Session ID generation and connection-to-pane mapping — addressed in Tasks 4, 6
- `connections.len() != rows * cols` mismatch — addressed with min/empty-pane logic in Tasks 2, 8
- App close cleanup for SSH threads — addressed in Task 8
- Double-activation prevention — addressed in Task 8
- TCP connect timeout (10s) to prevent activation stall — addressed in Task 1
- Terminal write batching — addressed in Task 5
- Resize debouncing (100ms) — addressed in Task 5

---

## Work Objectives

### Core Objective
Enable users to click "Activate" on a saved workset and see a grid of live SSH terminal sessions connected to their remote VMs, with bidirectional I/O flowing through Tauri IPC.

### Concrete Deliverables
- `src-tauri/src/ssh/session.rs` — SSH session worker thread with PTY management
- `src-tauri/src/ssh/mod.rs` — Connection manager (HashMap, parallel connect, cleanup)
- `src-tauri/src/lib.rs` — Updated with 4 new async Tauri commands + SSH state registration
- `src/grid.ts` — Grid layout engine (5 presets, CSS Grid rendering)
- `src/terminal.ts` — Terminal pane component (xterm.js, WebGL, FitAddon, cleanup)
- `src/workspace.ts` — Workspace view integration (grid + terminals + IPC wiring)
- `src/styles.css` — Grid, terminal pane, status indicator, and toolbar styles
- `index.html` — Workspace view container added

### Definition of Done
- [ ] `cargo check` exits 0 with no errors
- [ ] `npm run build` (tsc + vite) exits 0 with no errors
- [ ] `cargo build --release` exits 0 with no errors
- [ ] Activate button visible in workset detail view
- [ ] Grid renders correct number of panes for each preset
- [ ] Terminal panes contain xterm.js instances
- [ ] All IPC commands registered and type-check cleanly

### Must Have
- SSH Key authentication (pubkey file)
- SSH Password authentication (runtime-only, prompted on activate)
- Bidirectional terminal I/O via Tauri IPC
- 5 grid layout presets (1x1, 2x1, 2x2, 2x3, 3x3)
- Per-pane status indicator (connecting/connected/error/disconnected)
- Per-pane VM label (user@host)
- Disconnect All functionality
- Worker thread cleanup on disconnect and app close
- 10,000 line terminal scrollback
- WebGL renderer with Canvas fallback

### Must NOT Have (Guardrails)
- **NO password persistence**: Password MUST NOT appear in any JSON file on disk. Runtime-only via IPC. Violating NFR-12/13.
- **NO `~/.ssh/config` parsing**: `AuthMethod::SshConfig` returns a "not yet supported" error for this batch. Complex host-matching/ProxyJump parsing is a separate task.
- **NO drag-to-resize pane dividers**: CSS Grid with equal fractions (`1fr`) only. Drag-resize is a follow-up feature.
- **NO auto-reconnect logic**: Emit `disconnected` status events only. Auto-reconnect is Feature 9, separate batch.
- **NO pane content-type switching**: All panes are terminals. File Browser (Feature 5) and Markdown Viewer (Feature 6) are separate batches.
- **NO OS Keystore integration**: Requires platform-specific deps (`keytar`, `secret-service`, Keychain). Separate task.
- **NO async Rust on SSH threads**: Each SSH session is a `std::thread` with blocking ssh2 calls. Do NOT use tokio tasks for ssh2 operations — ssh2 is not Send-safe across await points.
- **NO unit tests in this batch**: Tests-after strategy. This batch is implementation-only. Verify via build commands.
- **NO modifications to existing CRUD UI**: The sidebar and workset create/edit form remain unchanged. Grid replaces only `#main-content` children.
- **NO over-engineering error handling**: Plain text error messages in status bar. No custom error modal components.

---

## Verification Strategy

> **UNIVERSAL RULE: ZERO HUMAN INTERVENTION**
>
> ALL tasks are verified by the executing agent running build commands.
> No human action is required for any acceptance criterion.

### Test Decision
- **Infrastructure exists**: Cargo built-in (`cargo test`), but NO test files in this batch
- **Automated tests**: None in this batch (tests-after strategy)
- **Framework**: N/A for this batch

### Build Verification Commands (ALL tasks)
```bash
cargo check                    # Rust compilation check (fast)
npm run build                  # TypeScript + Vite build
cargo build --release          # Full release build (final verification)
```

### Agent-Executed QA Scenarios

Since this is a desktop app requiring SSH targets to fully test, QA scenarios focus on **build verification + structural checks**:

```
Scenario: Full build pipeline passes
  Tool: Bash
  Steps:
    1. cargo check 2>&1 | tail -5
    2. Assert: exit code 0, no "error[E" in output
    3. npm run build 2>&1 | tail -5
    4. Assert: exit code 0, "built in" message present
    5. cargo build --release 2>&1 | tail -5
    6. Assert: exit code 0, "Finished" message present
  Expected Result: All three build commands succeed
  Evidence: Build output captured

Scenario: Module structure verification
  Tool: Bash
  Steps:
    1. ls src-tauri/src/ssh/session.rs src-tauri/src/ssh/mod.rs
    2. Assert: both files exist
    3. ls src/grid.ts src/terminal.ts src/workspace.ts
    4. Assert: all three files exist
    5. grep -c "activate_workset" src-tauri/src/lib.rs
    6. Assert: count >= 1
  Expected Result: All expected files exist, commands registered
  Evidence: ls and grep output captured

Scenario: TypeScript type-check passes
  Tool: Bash
  Steps:
    1. npx tsc --noEmit 2>&1
    2. Assert: exit code 0, no type errors
  Expected Result: Zero TypeScript errors
  Evidence: tsc output captured
```

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — 3 parallel tasks):
├── Task 1: SSH Session Core (Rust)           [no deps]
├── Task 2: Grid Layout Engine (Frontend)     [no deps]
└── Task 3: Terminal Pane Component (Frontend) [no deps]

Wave 2 (After Wave 1 — 2 parallel tasks):
├── Task 4: SSH Connection Manager (Rust)     [depends: 1]
└── Task 5: Grid-Terminal Integration (Frontend) [depends: 2, 3]

Wave 3 (After Wave 2 — 2 parallel tasks):
├── Task 6: Tauri IPC Commands (Rust+lib.rs)  [depends: 4]
└── Task 7: Workspace View + Status UI (Frontend) [depends: 5]

Wave 4 (After Wave 3 — 1 integration task):
└── Task 8: End-to-End Integration            [depends: 6, 7]

Critical Path: Task 1 → Task 4 → Task 6 → Task 8
Parallel Speedup: ~45% faster than sequential (backend & frontend fully independent through Waves 1–3)
```

### Dependency Matrix

| Task | Depends On | Blocks | Can Parallelize With |
|------|------------|--------|---------------------|
| 1 | None | 4 | 2, 3 |
| 2 | None | 5 | 1, 3 |
| 3 | None | 5 | 1, 2 |
| 4 | 1 | 6 | 5 |
| 5 | 2, 3 | 7 | 4 |
| 6 | 4 | 8 | 7 |
| 7 | 5 | 8 | 6 |
| 8 | 6, 7 | None | None (final) |

### Agent Dispatch Summary

| Wave | Tasks | Recommended Dispatch |
|------|-------|---------------------|
| 1 | 1, 2, 3 | 3 parallel: `delegate_task` with category per task |
| 2 | 4, 5 | 2 parallel after Wave 1 completes |
| 3 | 6, 7 | 2 parallel after Wave 2 completes |
| 4 | 8 | Final integration after Wave 3 completes |

---

## TODOs

---

- [ ] 1. SSH Session Core

  **What to do**:
  - Create directory `src-tauri/src/ssh/`
  - Create `src-tauri/src/ssh/session.rs` with SSH session types and worker thread
  - Update `src-tauri/Cargo.toml`: add `uuid` features if needed (already present for v4)
  - Do NOT add `mod ssh;` to `lib.rs` yet — that is Task 6's job

  **Types to define**:
  - `SshError` enum: `TcpConnect(String)`, `Handshake(String)`, `Auth(String)`, `Channel(String)`, `Pty(String)`, `Send(String)`, `SessionNotFound`
  - `SessionCommand` enum: `Write(Vec<u8>)`, `Resize { cols: u32, rows: u32 }`, `Shutdown`
  - `SessionStatus` enum: `Connecting`, `Connected`, `Disconnected`, `Error(String)` — derive `Serialize, Clone`
  - `SshSessionConfig` struct: `id: String`, `host: String`, `port: u16`, `user: String`, `auth_method: String` ("key" | "password"), `key_path: Option<String>`, `password: Option<String>`, `project_path: String`, `ai_cli_command: Option<String>`
  - `SshSessionHandle` struct: `id: String`, `host_display: String` (user@host:port), `cmd_tx: mpsc::Sender<SessionCommand>`, `worker: Option<JoinHandle<()>>`

  **Functions to implement**:
  - `SshSessionHandle::spawn(config: SshSessionConfig, app_handle: tauri::AppHandle) -> Result<Self, SshError>` — creates mpsc channel, spawns worker thread, returns handle
  - `SshSessionHandle::send_input(&self, data: Vec<u8>) -> Result<(), SshError>` — sends Write via mpsc
  - `SshSessionHandle::resize(&self, cols: u32, rows: u32) -> Result<(), SshError>` — sends Resize via mpsc
  - `SshSessionHandle::shutdown(&mut self)` — sends Shutdown, joins worker thread with 5s timeout
  - `fn session_worker(config: SshSessionConfig, app_handle: tauri::AppHandle, cmd_rx: mpsc::Receiver<SessionCommand>)` — the thread entry point

  **Worker thread implementation** (session_worker):
  1. Emit status event: `session-status-{id}` with `Connecting`
  2. `TcpStream::connect_timeout(&addr, Duration::from_secs(10))` — 10s timeout per Metis
  3. `tcp.set_nodelay(true)` for low latency
  4. `Session::new()` → `sess.set_tcp_stream(tcp.try_clone())` → `sess.handshake()`
  5. Auth dispatch:
     - `"key"`: `sess.userauth_pubkey_file(&user, None, Path::new(&key_path), None)`
     - `"password"`: `sess.userauth_password(&user, &password)`
     - other: emit Error status, return
  6. Verify `sess.authenticated()`
  7. `sess.set_keepalive(true, 15)` — 15 second interval
  8. `sess.channel_session()` → `channel.request_pty("xterm-256color", None, Some((80, 24, 0, 0)))` → `channel.shell()`
  9. If `project_path` not empty: `channel.write_all(format!("cd {}\n", project_path).as_bytes())`
  10. If `ai_cli_command` is Some: `channel.write_all(format!("{}\n", cmd).as_bytes())`
  11. Emit status event: `session-status-{id}` with `Connected`
  12. `sess.set_blocking(false)` — enable non-blocking reads
  13. Main loop:
      a. `match cmd_rx.try_recv()`:
         - `Ok(Write(data))` → `channel.write_all(&data)` (set blocking=true briefly, then back to false)
         - `Ok(Resize { cols, rows })` → `channel.request_pty_size(cols, rows, None, None)`
         - `Ok(Shutdown)` → break
         - `Err(TryRecvError::Disconnected)` → break
         - `Err(TryRecvError::Empty)` → continue
      b. Read from channel: `let mut buf = [0u8; 4096]; match channel.read(&mut buf)`:
         - `Ok(0)` → EOF, break
         - `Ok(n)` → emit `terminal-output-{id}` event with `&buf[..n]` as `Vec<u8>`
         - `Err(ref e) if e.kind() == WouldBlock` → no data, continue
         - `Err(e)` → emit Error status, break
      c. `std::thread::sleep(Duration::from_millis(10))` — prevent busy loop
  14. On exit: `channel.close()`, `channel.wait_close()`, emit `Disconnected` status

  **Event payloads** (emitted via `app_handle.emit()`):
  - `terminal-output-{id}`: `Vec<u8>` (raw bytes, serialized as JSON number array)
  - `session-status-{id}`: `SessionStatus` (serialized as JSON)

  **Must NOT do**:
  - Do NOT use `tokio::task::spawn` or any async runtime for SSH operations
  - Do NOT store or log passwords
  - Do NOT implement auto-reconnect (just emit Disconnected and exit the loop)
  - Do NOT add `mod ssh` to `lib.rs` (Task 6 does that)
  - Do NOT parse `~/.ssh/config`

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: SSH worker thread with non-blocking I/O, mpsc coordination, and Tauri event emission is a complex systems programming task requiring careful error handling
  - **Skills**: `[]`
    - No specialized skills needed — pure Rust systems programming

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3)
  - **Blocks**: Task 4
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References**:
  - `src-tauri/src/bin/spike_2_ssh_harness.rs:126-167` — SSH connect pattern: TcpStream + Session + handshake + userauth_pubkey_file + set_keepalive. Copy this auth flow.
  - `src-tauri/src/bin/spike_2_ssh_harness.rs:181-203` — PTY request pattern: channel_session + request_pty("xterm") + exec. Adapt for interactive shell instead of exec.
  - `src-tauri/src/bin/spike_2_ssh_harness.rs:238-247` — SessionResult struct pattern for tracking session metadata.
  - `src-tauri/src/workset/mod.rs:78-114` — Error enum pattern (StoreError). Follow same Display impl + From conversions for SshError.
  - `src-tauri/src/workset/mod.rs:41-47` — AuthMethod enum definition. Reference this when matching auth methods.

  **API/Type References**:
  - `src-tauri/src/workset/mod.rs:29-39` — ConnectionConfig fields. SshSessionConfig mirrors these fields plus password.
  - `src-tauri/src/workset/mod.rs:49-54` — GridLayout struct used by workset activation.

  **External References**:
  - ssh2 crate docs: `https://docs.rs/ssh2/0.9/ssh2/struct.Session.html` — Session API (handshake, userauth_password, userauth_pubkey_file, set_keepalive, set_blocking, channel_session)
  - ssh2 Channel docs: `https://docs.rs/ssh2/0.9/ssh2/struct.Channel.html` — Channel API (request_pty, shell, read, write_all, request_pty_size, close, wait_close, eof)
  - Tauri AppHandle emit: `https://docs.rs/tauri/latest/tauri/struct.AppHandle.html#method.emit` — verify Send + Sync for cross-thread usage

  **Acceptance Criteria**:
  - [ ] File exists: `src-tauri/src/ssh/session.rs`
  - [ ] Contains: `pub struct SshSessionHandle`, `pub enum SessionCommand`, `pub enum SshError`, `pub enum SessionStatus`
  - [ ] Contains: `pub fn spawn()` that returns `Result<SshSessionHandle, SshError>`
  - [ ] Contains: `fn session_worker()` with the full read/write loop
  - [ ] Contains: `send_input()`, `resize()`, `shutdown()` methods on SshSessionHandle
  - [ ] Worker uses `std::thread::spawn`, NOT tokio tasks
  - [ ] Worker emits events via `app_handle.emit()` for terminal output and session status
  - [ ] TCP connect uses 10-second timeout
  - [ ] Keepalive set to 15 seconds
  - [ ] PTY requested as "xterm-256color" with initial size 80x24
  - [ ] Non-blocking read loop with 10ms sleep to prevent busy loop
  - [ ] `cargo check` → exit 0, no errors (may need to temporarily add `mod ssh;` for the check, then revert — OR create a minimal `ssh/mod.rs` that re-exports)

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: SSH session module compiles cleanly
    Tool: Bash
    Preconditions: Rust toolchain installed, project root
    Steps:
      1. Ensure src-tauri/src/ssh/mod.rs exists with `pub mod session;` (create minimal one if needed for cargo check)
      2. Ensure lib.rs temporarily has `mod ssh;` (add and revert after check)
      3. Run: cargo check --manifest-path src-tauri/Cargo.toml 2>&1
      4. Assert: exit code 0
      5. Assert: no "error[E" in output
      6. Revert any temporary lib.rs changes
    Expected Result: Module compiles with all types and functions
    Evidence: cargo check output captured

  Scenario: Session types are complete
    Tool: Bash
    Preconditions: session.rs exists
    Steps:
      1. grep -c "pub enum SessionCommand" src-tauri/src/ssh/session.rs
      2. Assert: count == 1
      3. grep -c "pub struct SshSessionHandle" src-tauri/src/ssh/session.rs
      4. Assert: count == 1
      5. grep -c "pub enum SshError" src-tauri/src/ssh/session.rs
      6. Assert: count == 1
      7. grep -c "fn session_worker" src-tauri/src/ssh/session.rs
      8. Assert: count == 1
    Expected Result: All core types defined
    Evidence: grep output captured
  ```

  **Commit**: YES
  - Message: `feat(ssh): add SSH session core with PTY worker thread`
  - Files: `src-tauri/src/ssh/session.rs`, `src-tauri/Cargo.toml` (if changed)
  - Pre-commit: `cargo check --manifest-path src-tauri/Cargo.toml`

---

- [ ] 2. Grid Layout Engine

  **What to do**:
  - Create `src/grid.ts` — grid layout module
  - Add grid + toolbar + pane CSS to `src/styles.css`
  - Add `#workspace-view` container to `index.html`
  - Do NOT import or use grid.ts from main.ts yet — that is Task 7's job

  **grid.ts exports**:
  - `GRID_PRESETS` constant: `Record<string, { rows: number; cols: number }>` with keys: `'1x1'`, `'2x1'`, `'2x2'`, `'2x3'`, `'3x3'`
  - `createGrid(container: HTMLElement, rows: number, cols: number): HTMLElement[]` — sets CSS Grid on container, creates N pane div elements (N = rows * cols), returns array of pane elements
  - `destroyGrid(container: HTMLElement): void` — removes all child pane elements
  - `createLayoutToolbar(container: HTMLElement, activePreset: string, onChange: (preset: string) => void): void` — renders preset buttons, highlights active
  - `setActivePane(container: HTMLElement, index: number): void` — adds `.active` class to the target pane, removes from others

  **createGrid implementation details**:
  - Set `container.style.gridTemplateRows = repeat(rows, 1fr)`
  - Set `container.style.gridTemplateColumns = repeat(cols, 1fr)`
  - Create `rows * cols` div elements with class `.grid-pane` and `data-pane-index` attribute
  - Return the array of pane divs for terminal attachment

  **CSS to add to styles.css** (use existing CSS custom properties):
  ```
  /* Workspace View */
  #workspace-view — display:none; flex-direction:column; height:100%;
  #workspace-view.active — display:flex;

  /* Layout Toolbar */
  .layout-toolbar — display:flex; align-items:center; gap:6px; padding:8px 12px; background:var(--bg-secondary); border-bottom:1px solid var(--border);
  .layout-toolbar-btn — btn-ghost style, font-size:12px, padding:4px 10px
  .layout-toolbar-btn.active — accent background

  /* Disconnect All button in toolbar */
  .btn-disconnect-all — btn-danger style, margin-left:auto

  /* Grid Container */
  .grid-container — display:grid; flex:1; gap:2px; padding:2px; background:var(--bg-primary); overflow:hidden;

  /* Grid Pane */
  .grid-pane — position:relative; overflow:hidden; background:#000; border:1px solid var(--border); border-radius:var(--radius-sm); min-height:0; min-width:0;
  .grid-pane.active — border-color:var(--accent); border-width:2px;
  .grid-pane-empty — display:flex; align-items:center; justify-content:center; color:var(--text-dim); font-size:13px;
  ```

  **index.html changes**:
  - Inside `<main id="main-content">`, add after existing divs:
    ```html
    <div id="workspace-view">
      <div id="layout-toolbar"></div>
      <div id="grid-container" class="grid-container"></div>
    </div>
    ```
  - This div is hidden by default (CSS `display:none`), shown when workspace is activated

  **Must NOT do**:
  - Do NOT import grid.ts from main.ts (Task 7 wires it up)
  - Do NOT implement drag-to-resize dividers — use equal `1fr` fractions
  - Do NOT add terminal logic — grid only creates empty containers
  - Do NOT change existing sidebar, detail, or form views

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: CSS Grid layout + HTML structure + visual toolbar is frontend design work
  - **Skills**: `['frontend-ui-ux']`
    - `frontend-ui-ux`: Grid layout design, dark theme consistency, visual hierarchy

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3)
  - **Blocks**: Task 5
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References**:
  - `src/main.ts:99-105` — Existing `GRID_PRESETS` constant. The grid.ts module should export the same map (or import from a shared location). Values: 1x1, 2x1, 2x2, 2x3, 3x3.
  - `src/main.ts:269-285` — View state functions (`showEmptyState`, `showDetailView`, `showFormView`). Follow this pattern for showing/hiding workspace view. Use `style.display` toggling.
  - `src/styles.css:11-34` — CSS custom properties. ALL new CSS must use these variables for colors, borders, radii, transitions.
  - `src/styles.css:127-167` — Sidebar card styling pattern. Follow same padding/border/radius conventions for toolbar buttons.
  - `index.html:21-31` — Current `#main-content` structure with empty-state, workset-detail, workset-form. Add workspace-view alongside these (same level).

  **Acceptance Criteria**:
  - [ ] File exists: `src/grid.ts`
  - [ ] Exports: `createGrid`, `destroyGrid`, `createLayoutToolbar`, `setActivePane`, `GRID_PRESETS`
  - [ ] `createGrid` generates correct number of `.grid-pane` elements (e.g., 4 for 2x2, 6 for 2x3)
  - [ ] CSS contains `.grid-container`, `.grid-pane`, `.grid-pane.active`, `.layout-toolbar` classes
  - [ ] `#workspace-view` exists in `index.html` with toolbar and grid container children
  - [ ] All CSS uses existing custom properties (`--bg-primary`, `--accent`, `--border`, etc.)
  - [ ] `npm run build` → exit 0, no errors

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Grid module compiles and exports correctly
    Tool: Bash
    Preconditions: Node.js installed, project dependencies installed
    Steps:
      1. Run: npm run build 2>&1
      2. Assert: exit code 0
      3. Assert: no "error TS" in output
      4. grep -c "export function createGrid" src/grid.ts
      5. Assert: count == 1
      6. grep -c "export function destroyGrid" src/grid.ts
      7. Assert: count == 1
    Expected Result: Module compiles and exports are present
    Evidence: Build output + grep results captured

  Scenario: HTML structure includes workspace view
    Tool: Bash
    Steps:
      1. grep -c 'id="workspace-view"' index.html
      2. Assert: count == 1
      3. grep -c 'id="grid-container"' index.html
      4. Assert: count == 1
      5. grep -c 'id="layout-toolbar"' index.html
      6. Assert: count == 1
    Expected Result: All workspace HTML elements exist
    Evidence: grep output captured
  ```

  **Commit**: YES
  - Message: `feat(grid): add CSS Grid layout engine with 5 preset layouts`
  - Files: `src/grid.ts`, `src/styles.css`, `index.html`
  - Pre-commit: `npm run build`

---

- [ ] 3. Terminal Pane Component

  **What to do**:
  - Create `src/terminal.ts` — xterm.js terminal component module
  - Ensure xterm.css is imported (either in terminal.ts or in main entry point)
  - Do NOT import or use terminal.ts from main.ts yet — that is Task 5's job

  **terminal.ts exports**:
  - `TerminalInstance` interface: `{ terminal: Terminal; fitAddon: FitAddon; disposables: Array<{ dispose(): void }>; }`
  - `createTerminal(container: HTMLElement): TerminalInstance` — creates + configures xterm
  - `destroyTerminal(instance: TerminalInstance): void` — disposes all resources
  - `writeToTerminal(instance: TerminalInstance, data: Uint8Array | string): void` — writes data to terminal
  - `TERMINAL_THEME` constant — dark theme colors

  **createTerminal implementation**:
  1. Create `Terminal` with options:
     - `fontFamily: 'JetBrains Mono, Menlo, Monaco, Consolas, Courier New, monospace'`
     - `fontSize: 14`
     - `cursorBlink: true`
     - `scrollback: 10000`
     - `allowProposedApi: true`
     - `theme`: dark theme matching CSS vars (`background: '#0f0f1a'`, `foreground: '#e0e0e0'`, `cursor: '#00d4ff'`, `selectionBackground: 'rgba(0, 212, 255, 0.3)'`)
  2. Create `FitAddon`, load it: `terminal.loadAddon(fitAddon)`
  3. `terminal.open(container)`
  4. `fitAddon.fit()`
  5. Try WebGL addon:
     ```typescript
     try {
       const webglAddon = new WebglAddon();
       webglAddon.onContextLoss(() => { webglAddon.dispose(); });
       terminal.loadAddon(webglAddon);
     } catch (e) {
       console.warn('[Terminal] WebGL not available, using Canvas renderer');
     }
     ```
  6. Return `{ terminal, fitAddon, disposables: [] }`

  **destroyTerminal implementation**:
  1. Iterate `disposables`, call `.dispose()` on each
  2. Clear `disposables` array
  3. Call `fitAddon.dispose()`
  4. Call `terminal.dispose()`

  **xterm.css import**:
  - Add `import '@xterm/xterm/css/xterm.css';` at the top of `terminal.ts`
  - Vite handles CSS imports from node_modules automatically

  **Must NOT do**:
  - Do NOT add IPC wiring (event listeners for terminal output) — that is Task 5/8
  - Do NOT import terminal.ts from main.ts (Task 5 does the integration)
  - Do NOT add resize observer logic — that is Task 5
  - Do NOT add copy/paste logic beyond xterm.js defaults (xterm handles Ctrl+Shift+C/V natively in terminal context)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: xterm.js initialization with WebGL, theming, and addon management is frontend component work
  - **Skills**: `['frontend-ui-ux']`
    - `frontend-ui-ux`: Terminal theme design matching existing dark palette

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2)
  - **Blocks**: Task 5
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References**:
  - `src/styles.css:11-34` — CSS custom properties for theme colors. Terminal theme must match: `--bg-primary: #0f0f1a`, `--text-primary: #e0e0e0`, `--accent: #00d4ff`.
  - `src/main.ts:1` — Import pattern for Tauri API: `import { invoke } from "@tauri-apps/api/core"`. Follow same ES module import style for xterm.

  **External References**:
  - xterm.js docs: `https://xtermjs.org/docs/api/terminal/classes/terminal/` — Terminal class constructor options
  - WebGL addon: `https://www.npmjs.com/package/@xterm/addon-webgl` — Setup, onContextLoss handler
  - FitAddon: `https://www.npmjs.com/package/@xterm/addon-fit` — fit() method, must call after container is sized
  - Package versions in `package.json:15-17`: `@xterm/xterm ^5.5.0`, `@xterm/addon-webgl ^0.18.0`, `@xterm/addon-fit ^0.10.0`

  **Acceptance Criteria**:
  - [ ] File exists: `src/terminal.ts`
  - [ ] Exports: `createTerminal`, `destroyTerminal`, `writeToTerminal`, `TerminalInstance`
  - [ ] `createTerminal` configures: scrollback 10000, cursorBlink true, dark theme, fontSize 14
  - [ ] WebGL addon loaded in try/catch with Canvas fallback
  - [ ] FitAddon loaded and `fit()` called after `open()`
  - [ ] `destroyTerminal` calls `dispose()` on all addons and terminal
  - [ ] xterm.css is imported (either directly or via a CSS import)
  - [ ] `npm run build` → exit 0, no errors

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Terminal module compiles with xterm imports
    Tool: Bash
    Preconditions: npm install completed
    Steps:
      1. Run: npm run build 2>&1
      2. Assert: exit code 0
      3. grep -c "import.*@xterm/xterm" src/terminal.ts
      4. Assert: count >= 1
      5. grep -c "export function createTerminal" src/terminal.ts
      6. Assert: count == 1
      7. grep -c "scrollback.*10000" src/terminal.ts
      8. Assert: count == 1
    Expected Result: Module compiles with correct xterm configuration
    Evidence: Build output + grep results captured
  ```

  **Commit**: YES
  - Message: `feat(terminal): add xterm.js terminal pane component with WebGL`
  - Files: `src/terminal.ts`
  - Pre-commit: `npm run build`

---

- [ ] 4. SSH Connection Manager

  **What to do**:
  - Create `src-tauri/src/ssh/mod.rs` — manager that owns all sessions
  - Re-export session types from this module
  - Do NOT register `mod ssh` in `lib.rs` yet — Task 6 does that

  **mod.rs contents**:
  - `pub mod session;` — re-export the session module
  - Re-exports: `pub use session::{SshSessionHandle, SshSessionConfig, SessionCommand, SessionStatus, SshError};`
  - `SshConnectionManager` struct:
    ```rust
    pub struct SshConnectionManager {
        sessions: std::sync::Mutex<HashMap<String, SshSessionHandle>>,
    }
    ```
  - `impl SshConnectionManager`:
    - `pub fn new() -> Self` — empty HashMap
    - `pub fn connect(&self, config: SshSessionConfig, app_handle: tauri::AppHandle) -> Result<String, SshError>` — spawns session, stores handle, returns session_id
    - `pub fn connect_all(&self, configs: Vec<SshSessionConfig>, app_handle: tauri::AppHandle) -> Vec<(usize, Result<String, SshError>)>` — connects all in parallel using `std::thread::scope` or thread pool, returns vec of (index, result) pairs
    - `pub fn send_input(&self, session_id: &str, data: Vec<u8>) -> Result<(), SshError>` — locks map, finds session, calls send_input
    - `pub fn resize(&self, session_id: &str, cols: u32, rows: u32) -> Result<(), SshError>` — locks map, finds session, calls resize
    - `pub fn disconnect(&self, session_id: &str) -> Result<(), SshError>` — locks map, removes session, calls shutdown
    - `pub fn disconnect_all(&self)` — locks map, drains all sessions, shuts each down
    - `pub fn active_sessions(&self) -> Vec<(String, String)>` — returns vec of (session_id, host_display)
  - `impl Drop for SshConnectionManager` — calls `disconnect_all()`

  **connect_all implementation**:
  ```rust
  pub fn connect_all(
      &self,
      configs: Vec<SshSessionConfig>,
      app_handle: tauri::AppHandle,
  ) -> Vec<(usize, Result<String, SshError>)> {
      let results: Vec<_> = std::thread::scope(|s| {
          let handles: Vec<_> = configs.into_iter().enumerate().map(|(idx, config)| {
              let app = app_handle.clone();
              s.spawn(move || (idx, SshSessionHandle::spawn(config, app)))
          }).collect();
          handles.into_iter().map(|h| h.join().unwrap_or_else(|_| /* handle panic */)).collect()
      });
      // Store successful sessions
      let mut sessions = self.sessions.lock().unwrap();
      let mut output = Vec::new();
      for (idx, result) in results {
          match result {
              Ok(handle) => {
                  let id = handle.id.clone();
                  sessions.insert(id.clone(), handle);
                  output.push((idx, Ok(id)));
              }
              Err(e) => output.push((idx, Err(e))),
          }
      }
      output
  }
  ```

  **Must NOT do**:
  - Do NOT add `mod ssh` to `lib.rs` (Task 6)
  - Do NOT implement auto-reconnect
  - Do NOT use tokio channels — use `std::sync::Mutex` and `std::sync::mpsc` only
  - Do NOT implement session listing/querying beyond `active_sessions()`

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Thread-safe session management with parallel connect, mutex handling, and Drop cleanup
  - **Skills**: `[]`
    - No specialized skills — Rust concurrency work

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Task 5)
  - **Blocks**: Task 6
  - **Blocked By**: Task 1 (needs session.rs types and SshSessionHandle::spawn)

  **References**:

  **Pattern References**:
  - `src-tauri/src/ssh/session.rs` (from Task 1) — SshSessionHandle API: `spawn()`, `send_input()`, `resize()`, `shutdown()`. Manager wraps these with session-ID lookup and mutex protection.
  - `src-tauri/src/workset/mod.rs:116-131` — WorksetStore struct pattern: uses `Mutex<()>` for thread safety. Follow similar pattern but hold `Mutex<HashMap<>>`.
  - `src-tauri/src/workset/mod.rs:229-231` — `lock_guard()` helper pattern for clean mutex access. Consider similar helper for SshConnectionManager.
  - `src-tauri/src/bin/spike_2_ssh_harness.rs:633-657` — Parallel thread spawning pattern with `mpsc::channel` for collecting results. Adapt for `connect_all`.

  **Acceptance Criteria**:
  - [ ] File exists: `src-tauri/src/ssh/mod.rs`
  - [ ] Contains: `pub mod session;` and re-exports
  - [ ] Contains: `pub struct SshConnectionManager` with `Mutex<HashMap<String, SshSessionHandle>>`
  - [ ] Contains: `connect`, `connect_all`, `send_input`, `resize`, `disconnect`, `disconnect_all`, `active_sessions`
  - [ ] `connect_all` spawns connections in parallel (not sequentially)
  - [ ] `Drop` impl calls `disconnect_all`
  - [ ] `cargo check` → exit 0 (with temporary `mod ssh` in lib.rs for verification, then revert)

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: SSH manager module compiles with session dependency
    Tool: Bash
    Steps:
      1. Verify src-tauri/src/ssh/session.rs exists (from Task 1)
      2. Verify src-tauri/src/ssh/mod.rs exists
      3. Temporarily add "mod ssh;" to lib.rs for compilation check
      4. Run: cargo check --manifest-path src-tauri/Cargo.toml 2>&1
      5. Assert: exit code 0
      6. Revert lib.rs change
    Expected Result: Both SSH modules compile together
    Evidence: cargo check output captured

  Scenario: Manager has all required methods
    Tool: Bash
    Steps:
      1. grep -c "pub fn connect_all" src-tauri/src/ssh/mod.rs → assert 1
      2. grep -c "pub fn disconnect_all" src-tauri/src/ssh/mod.rs → assert 1
      3. grep -c "pub fn send_input" src-tauri/src/ssh/mod.rs → assert 1
      4. grep -c "pub fn resize" src-tauri/src/ssh/mod.rs → assert 1
      5. grep -c "impl Drop" src-tauri/src/ssh/mod.rs → assert 1
    Expected Result: All manager methods present
    Evidence: grep output captured
  ```

  **Commit**: YES
  - Message: `feat(ssh): add connection manager with parallel connect and cleanup`
  - Files: `src-tauri/src/ssh/mod.rs`
  - Pre-commit: `cargo check --manifest-path src-tauri/Cargo.toml` (with temporary mod registration)

---

- [ ] 5. Grid-Terminal Integration

  **What to do**:
  - Create `src/workspace.ts` — combines grid + terminal + output batching + resize handling
  - Wire xterm.js instances into grid pane containers
  - Add ResizeObserver per pane for FitAddon.fit()
  - Add active pane tracking (click → focus)
  - Add output write batching via requestAnimationFrame
  - Do NOT add IPC wiring to Tauri (that is Task 8)
  - Do NOT import workspace.ts from main.ts (Task 7 does that)

  **workspace.ts exports**:
  ```typescript
  interface PaneState {
    index: number;
    sessionId: string | null;
    terminal: TerminalInstance | null;
    container: HTMLElement;
    statusEl: HTMLElement | null;
    hostLabel: string;
    resizeObserver: ResizeObserver | null;
    outputBuffer: OutputBuffer | null;
  }

  // Output batching class
  class OutputBuffer {
    private chunks: Uint8Array[];
    private scheduled: boolean;
    private terminal: Terminal;
    write(data: Uint8Array): void;  // Buffers and flushes on rAF
  }

  export function createWorkspace(
    gridContainer: HTMLElement,
    rows: number,
    cols: number,
    connectionCount: number,
  ): PaneState[];

  export function attachTerminal(pane: PaneState): void;
  export function detachTerminal(pane: PaneState): void;
  export function destroyWorkspace(panes: PaneState[]): void;
  export function getActivePaneIndex(): number;
  export function writeToPaneBuffer(pane: PaneState, data: Uint8Array): void;
  ```

  **createWorkspace implementation**:
  1. Call `createGrid(gridContainer, rows, cols)` — gets pane elements
  2. For each pane up to `min(paneElements.length, connectionCount)`:
     - Create PaneState with terminal: null, sessionId: null
     - Set up click handler: `pane.addEventListener('click', () => setActivePane(...))`
  3. For excess panes (index >= connectionCount):
     - Add `.grid-pane-empty` class, show "No connection" text
  4. Return PaneState array

  **attachTerminal implementation**:
  1. Call `createTerminal(pane.container)` from terminal.ts
  2. Store in `pane.terminal`
  3. Create ResizeObserver on `pane.container`:
     - On resize: debounce 100ms → `fitAddon.fit()` → emit resize dimensions
  4. Create OutputBuffer for this terminal
  5. Store disposables

  **OutputBuffer implementation** (critical for performance — Metis guardrail G5):
  ```typescript
  class OutputBuffer {
    private chunks: Uint8Array[] = [];
    private scheduled = false;
    constructor(private terminal: Terminal) {}
    write(data: Uint8Array): void {
      this.chunks.push(data);
      if (!this.scheduled) {
        this.scheduled = true;
        requestAnimationFrame(() => {
          const merged = this.mergeChunks();
          this.terminal.write(merged);
          this.chunks = [];
          this.scheduled = false;
        });
      }
    }
    private mergeChunks(): Uint8Array {
      if (this.chunks.length === 1) return this.chunks[0];
      const total = this.chunks.reduce((sum, c) => sum + c.length, 0);
      const merged = new Uint8Array(total);
      let offset = 0;
      for (const chunk of this.chunks) {
        merged.set(chunk, offset);
        offset += chunk.length;
      }
      return merged;
    }
  }
  ```

  **Resize debounce** (100ms per Metis):
  ```typescript
  let resizeTimer: ReturnType<typeof setTimeout> | null = null;
  const observer = new ResizeObserver(() => {
    if (resizeTimer) clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => {
      pane.terminal?.fitAddon.fit();
      // Dimensions available via terminal.cols, terminal.rows
    }, 100);
  });
  observer.observe(pane.container);
  ```

  **Must NOT do**:
  - Do NOT add Tauri IPC calls (invoke, listen) — that is Task 8
  - Do NOT add status bar/indicators — that is Task 7
  - Do NOT import workspace.ts from main.ts — Task 7 does that
  - Do NOT implement pane content-type switching — all panes are terminals

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Frontend component integration, ResizeObserver, DOM management, performance optimization
  - **Skills**: `['frontend-ui-ux']`
    - `frontend-ui-ux`: Component composition, resize behavior, focus management

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Task 4)
  - **Blocks**: Task 7
  - **Blocked By**: Task 2 (needs grid.ts), Task 3 (needs terminal.ts)

  **References**:

  **Pattern References**:
  - `src/grid.ts` (from Task 2) — `createGrid()`, `setActivePane()` APIs. workspace.ts calls these.
  - `src/terminal.ts` (from Task 3) — `createTerminal()`, `destroyTerminal()`, `writeToTerminal()` APIs. workspace.ts wraps these.
  - `src/main.ts:81-97` — `showToast()` pattern for user notifications. Can reuse for connection status toasts.

  **External References**:
  - ResizeObserver API: `https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver` — observe pane containers for size changes
  - requestAnimationFrame: `https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame` — for output batching

  **Acceptance Criteria**:
  - [ ] File exists: `src/workspace.ts`
  - [ ] Exports: `createWorkspace`, `attachTerminal`, `detachTerminal`, `destroyWorkspace`, `writeToPaneBuffer`
  - [ ] `createWorkspace` calls `createGrid` and handles connection count vs cell count mismatch
  - [ ] `attachTerminal` creates xterm instance + ResizeObserver
  - [ ] OutputBuffer batches writes via requestAnimationFrame (not immediate writes)
  - [ ] Resize is debounced at 100ms
  - [ ] Active pane tracked via click handlers
  - [ ] `destroyWorkspace` disposes all terminals and observers
  - [ ] `npm run build` → exit 0, no errors

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Workspace module compiles with grid+terminal dependencies
    Tool: Bash
    Steps:
      1. Verify src/grid.ts exists (from Task 2)
      2. Verify src/terminal.ts exists (from Task 3)
      3. Run: npm run build 2>&1
      4. Assert: exit code 0
      5. grep -c "export function createWorkspace" src/workspace.ts → assert 1
      6. grep -c "requestAnimationFrame" src/workspace.ts → assert >= 1
      7. grep -c "ResizeObserver" src/workspace.ts → assert >= 1
    Expected Result: Workspace module compiles and uses batching + resize
    Evidence: Build output + grep results captured
  ```

  **Commit**: YES
  - Message: `feat(workspace): integrate terminals into grid with output batching`
  - Files: `src/workspace.ts`
  - Pre-commit: `npm run build`

---

- [ ] 6. Tauri IPC Commands + State Registration

  **What to do**:
  - Major update to `src-tauri/src/lib.rs`:
    - Add `mod ssh;` declaration
    - Register `SshConnectionManager` as managed state
    - Add 4 new async Tauri commands
    - Register commands in `invoke_handler`
  - Define `SessionInfo` return type

  **New types in lib.rs** (or in ssh module and re-exported):
  ```rust
  #[derive(Serialize, Clone)]
  pub struct SessionInfo {
      pub session_id: String,
      pub connection_index: usize,
      pub host: String,
      pub status: String,
  }
  ```

  **New Tauri commands**:

  ```rust
  #[tauri::command]
  async fn activate_workset(
      workset_id: String,
      passwords: Vec<Option<String>>,
      app: tauri::AppHandle,
      store: tauri::State<'_, WorksetStore>,
      ssh_manager: tauri::State<'_, ssh::SshConnectionManager>,
  ) -> Result<Vec<SessionInfo>, String>
  ```
  Implementation:
  1. `store.get(&workset_id)` → if None, return Err
  2. `ssh_manager.disconnect_all()` first (prevent double activation — Metis edge case)
  3. Build `Vec<SshSessionConfig>` from workset.connections:
     - Map each `ConnectionConfig` → `SshSessionConfig` with UUID as session id
     - For password auth: use `passwords[i]` if available
     - For key auth: password entry ignored
     - For ssh_config: return Err("SshConfig auth is not yet supported")
  4. Call `ssh_manager.connect_all(configs, app)`
  5. Map results to `Vec<SessionInfo>` with session_id, connection_index, host, status

  ```rust
  #[tauri::command]
  async fn deactivate_workset(
      ssh_manager: tauri::State<'_, ssh::SshConnectionManager>,
  ) -> Result<(), String>
  ```
  Implementation: `ssh_manager.disconnect_all(); Ok(())`

  ```rust
  #[tauri::command]
  fn terminal_input(
      session_id: String,
      data: String,
      ssh_manager: tauri::State<'_, ssh::SshConnectionManager>,
  ) -> Result<(), String>
  ```
  Implementation: `ssh_manager.send_input(&session_id, data.into_bytes()).map_err(|e| e.to_string())`

  ```rust
  #[tauri::command]
  fn terminal_resize(
      session_id: String,
      cols: u32,
      rows: u32,
      ssh_manager: tauri::State<'_, ssh::SshConnectionManager>,
  ) -> Result<(), String>
  ```
  Implementation: `ssh_manager.resize(&session_id, cols, rows).map_err(|e| e.to_string())`

  **Update run() function**:
  ```rust
  pub fn run() {
      tauri::Builder::default()
          .manage(WorksetStore::new())
          .manage(ssh::SshConnectionManager::new())
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
          ])
          .run(tauri::generate_context!())
          .expect("error while running tauri application");
  }
  ```

  **Must NOT do**:
  - Do NOT change existing CRUD commands (they work, leave them alone)
  - Do NOT store passwords anywhere (they flow through IPC only)
  - Do NOT implement OS Keystore lookups
  - Do NOT implement reconnect logic in commands

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Tauri state management + async command patterns + cross-module integration is complex Rust work
  - **Skills**: `[]`
    - No specialized skills — Rust Tauri integration

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Task 7)
  - **Blocks**: Task 8
  - **Blocked By**: Task 4 (needs SshConnectionManager)

  **References**:

  **Pattern References**:
  - `src-tauri/src/lib.rs:6-38` — Existing Tauri command patterns. Note: existing commands are sync, but new commands MUST be async. Reference the function signatures and State parameter patterns.
  - `src-tauri/src/lib.rs:40-54` — `run()` function: `.manage()` and `.invoke_handler()` patterns. Add new state and commands here.
  - `src-tauri/src/workset/mod.rs:19-27` — `Workset` struct with `connections: Vec<ConnectionConfig>` and `grid_layout: GridLayout`. The `activate_workset` command reads this to build SSH configs.
  - `src-tauri/src/workset/mod.rs:29-47` — `ConnectionConfig` and `AuthMethod` types. Map these to `SshSessionConfig` fields.
  - `src-tauri/src/ssh/mod.rs` (from Task 4) — `SshConnectionManager` API: `connect_all()`, `disconnect_all()`, `send_input()`, `resize()`.

  **Acceptance Criteria**:
  - [ ] `mod ssh;` declared in `lib.rs`
  - [ ] `SshConnectionManager` registered as managed state via `.manage()`
  - [ ] Commands registered: `activate_workset`, `deactivate_workset`, `terminal_input`, `terminal_resize`
  - [ ] `activate_workset` is `async fn` and accepts `workset_id: String` + `passwords: Vec<Option<String>>`
  - [ ] `activate_workset` calls `disconnect_all()` before connecting (prevent double activation)
  - [ ] `activate_workset` returns `Vec<SessionInfo>` with session_id, connection_index, host, status
  - [ ] `terminal_input` sends `data.into_bytes()` to the correct session
  - [ ] `terminal_resize` sends resize to the correct session
  - [ ] SshConfig auth method returns clear error: "SshConfig auth is not yet supported"
  - [ ] `cargo check` → exit 0
  - [ ] `cargo build --release` → exit 0

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: All IPC commands compile and are registered
    Tool: Bash
    Steps:
      1. Run: cargo check --manifest-path src-tauri/Cargo.toml 2>&1
      2. Assert: exit code 0, no errors
      3. grep -c "activate_workset" src-tauri/src/lib.rs → assert >= 2 (fn + handler registration)
      4. grep -c "deactivate_workset" src-tauri/src/lib.rs → assert >= 2
      5. grep -c "terminal_input" src-tauri/src/lib.rs → assert >= 2
      6. grep -c "terminal_resize" src-tauri/src/lib.rs → assert >= 2
      7. grep -c "SshConnectionManager::new()" src-tauri/src/lib.rs → assert 1
    Expected Result: All commands registered and compiled
    Evidence: cargo check + grep output captured

  Scenario: Full release build succeeds
    Tool: Bash
    Steps:
      1. Run: cargo build --release --manifest-path src-tauri/Cargo.toml 2>&1 | tail -5
      2. Assert: exit code 0
      3. Assert: "Finished" in output
    Expected Result: Release build compiles cleanly
    Evidence: Build output captured
  ```

  **Commit**: YES
  - Message: `feat(ipc): add SSH terminal IPC commands and state registration`
  - Files: `src-tauri/src/lib.rs`
  - Pre-commit: `cargo build --release --manifest-path src-tauri/Cargo.toml`

---

- [ ] 7. Workspace View + Pane Status UI

  **What to do**:
  - Update `src/main.ts` to add workspace view state management
  - Add fourth app state: `WorkspaceActive` alongside existing `EmptyState | WorksetDetail | WorksetForm`
  - Add status bar HTML/CSS per terminal pane (connecting/connected/error dot + host label)
  - Add "Disconnect All" button in layout toolbar
  - Add "Activate" button in workset detail view
  - Import and wire grid.ts, terminal.ts, workspace.ts into main.ts

  **App state machine update in main.ts**:
  - Add `let activeWorkspace: { worksetId: string; panes: PaneState[]; sessionInfos: SessionInfo[] } | null = null;`
  - New functions:
    - `showWorkspaceView()` — hides empty/detail/form, shows workspace-view
    - `hideWorkspaceView()` — hides workspace-view, shows appropriate view
  - Update `showEmptyState`, `showDetailView`, `showFormView` to also hide workspace-view

  **Activate button in detail view**:
  - Add to `renderWorksetDetail()` in the `.detail-header-actions` div:
    ```html
    <button class="btn btn-primary" id="btn-activate-workset">Activate</button>
    ```
  - Wire click handler (actual activation logic is Task 8, but the button + handler skeleton goes here)

  **Pane status bar** (added per pane by workspace.ts or here):
  - HTML per pane:
    ```html
    <div class="pane-status-bar">
      <span class="pane-status-dot connecting"></span>
      <span class="pane-host-label">user@host:port</span>
    </div>
    ```
  - Add exported functions to workspace.ts or a new file:
    - `updatePaneStatus(pane: PaneState, status: string): void` — updates dot class
    - `setPaneHostLabel(pane: PaneState, label: string): void`

  **CSS additions to styles.css**:
  ```
  .pane-status-bar — position:absolute; top:0; left:0; right:0; height:24px; display:flex; align-items:center; padding:0 8px; background:rgba(0,0,0,0.6); z-index:10; font-family:monospace;
  .pane-status-dot — width:8px; height:8px; border-radius:50%; margin-right:6px; flex-shrink:0;
  .pane-status-dot.connecting — background:var(--accent); animation:pulse 1.5s infinite;
  .pane-status-dot.connected — background:var(--success);
  .pane-status-dot.error — background:var(--danger);
  .pane-status-dot.disconnected — background:var(--text-dim);
  .pane-host-label — font-size:11px; color:var(--text-secondary); white-space:nowrap; overflow:hidden; text-overflow:ellipsis;
  @keyframes pulse — 0%{opacity:1} 50%{opacity:0.4} 100%{opacity:1}
  ```

  **Disconnect All button** (in layout toolbar):
  - Added by `createLayoutToolbar` or here
  - `<button class="btn btn-danger btn-sm btn-disconnect-all">Disconnect All</button>`
  - Handler skeleton only (actual disconnect logic is Task 8)

  **Must NOT do**:
  - Do NOT implement actual IPC calls (invoke/listen) — that is Task 8
  - Do NOT implement the full activation flow — just UI elements and state transitions
  - Do NOT modify existing CRUD form or sidebar logic
  - Do NOT add close/cleanup handlers (Task 8)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: UI state machine, status indicators, button placement, CSS animations
  - **Skills**: `['frontend-ui-ux']`
    - `frontend-ui-ux`: Visual status indicators, UI transitions, component layout

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Task 6)
  - **Blocks**: Task 8
  - **Blocked By**: Task 5 (needs workspace.ts PaneState types)

  **References**:

  **Pattern References**:
  - `src/main.ts:269-285` — `showEmptyState()`, `showDetailView()`, `showFormView()` — add `showWorkspaceView()` following this pattern.
  - `src/main.ts:327-386` — `renderWorksetDetail()` function. Add "Activate" button to `.detail-header-actions` div alongside existing Edit/Delete buttons.
  - `src/main.ts:47-55` — `$()` helper function for DOM access. Reuse for new elements.
  - `src/styles.css:442-519` — Button style patterns (`.btn-primary`, `.btn-danger`, `.btn-ghost`). Follow for Activate and Disconnect All buttons.
  - `src/workspace.ts` (from Task 5) — `PaneState` interface, `createWorkspace()`, `destroyWorkspace()` APIs. Import and use these.
  - `src/grid.ts` (from Task 2) — `createLayoutToolbar()`. Use for toolbar with preset buttons.

  **Acceptance Criteria**:
  - [ ] `showWorkspaceView()` and `hideWorkspaceView()` functions added to main.ts
  - [ ] "Activate" button added to workset detail view (in detail-header-actions)
  - [ ] "Disconnect All" button added to layout toolbar
  - [ ] Pane status bar CSS with `.connecting`, `.connected`, `.error`, `.disconnected` dot states
  - [ ] Pulse animation on `.connecting` status dot
  - [ ] Host label displays in pane status bar
  - [ ] workspace.ts, grid.ts, terminal.ts imported into main.ts
  - [ ] `npm run build` → exit 0, no errors

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Workspace view integration compiles
    Tool: Bash
    Steps:
      1. Run: npm run build 2>&1
      2. Assert: exit code 0
      3. grep -c "showWorkspaceView" src/main.ts → assert >= 1
      4. grep -c "btn-activate-workset" src/main.ts → assert >= 1
      5. grep -c "btn-disconnect-all" src/main.ts OR src/styles.css → assert >= 1
      6. grep -c "pane-status-dot" src/styles.css → assert >= 1
      7. grep -c "@keyframes pulse" src/styles.css → assert 1
    Expected Result: All UI elements present and build passes
    Evidence: Build output + grep results captured
  ```

  **Commit**: YES
  - Message: `feat(ui): add workspace view, pane status indicators, and activation button`
  - Files: `src/main.ts`, `src/styles.css`, `src/workspace.ts` (if status functions added there)
  - Pre-commit: `npm run build`

---

- [ ] 8. End-to-End Integration + Activation Flow

  **What to do**:
  - Wire everything together: UI → IPC → SSH → Terminal
  - Implement the complete workset activation flow
  - Add Tauri event listeners for terminal output and session status
  - Wire terminal input to IPC command
  - Wire terminal resize to IPC command
  - Add password prompt for password-auth connections
  - Add Disconnect All handler
  - Add app close cleanup
  - Prevent double activation

  **Activation flow implementation** (in main.ts, triggered by Activate button):
  1. Get workset data: `const workset = await invoke<Workset>('get_workset', { id: selectedWorksetId })`
  2. Collect passwords for password-auth connections:
     ```typescript
     const passwords: (string | null)[] = workset.connections.map((conn, i) => {
       if (conn.auth_method === 'password') {
         const pw = window.prompt(`Password for ${conn.user}@${conn.host}:`);
         if (pw === null) return null; // User cancelled
         return pw;
       }
       return null;
     });
     // If any required password was cancelled, abort
     if (workset.connections.some((c, i) => c.auth_method === 'password' && passwords[i] === null)) {
       showToast('Activation cancelled', 'error');
       return;
     }
     ```
  3. Switch to workspace view: `showWorkspaceView()`
  4. Create grid + terminal panes: `createWorkspace(gridContainer, rows, cols, connections.length)`
  5. Render layout toolbar with current preset
  6. Invoke activation:
     ```typescript
     const sessions = await invoke<SessionInfo[]>('activate_workset', {
       worksetId: selectedWorksetId,
       passwords,
     });
     ```
  7. Map session IDs to panes: `sessions.forEach(s => { panes[s.connection_index].sessionId = s.session_id; })`
  8. Set host labels: `sessions.forEach(s => setPaneHostLabel(panes[s.connection_index], s.host))`
  9. Attach terminals and set up event listeners per pane (see below)

  **Per-pane event wiring**:
  ```typescript
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';

  const unlisteners: UnlistenFn[] = [];

  for (const pane of panes) {
    if (!pane.sessionId || !pane.terminal) continue;
    const sid = pane.sessionId;

    // Terminal output → write to terminal (with batching)
    const unlisten1 = await listen<number[]>(`terminal-output-${sid}`, (event) => {
      const data = new Uint8Array(event.payload);
      writeToPaneBuffer(pane, data);
    });
    unlisteners.push(unlisten1);

    // Session status → update pane status dot
    const unlisten2 = await listen<SessionStatus>(`session-status-${sid}`, (event) => {
      updatePaneStatus(pane, event.payload);
    });
    unlisteners.push(unlisten2);

    // Terminal input → send to SSH
    const inputDisposable = pane.terminal.terminal.onData((data: string) => {
      invoke('terminal_input', { sessionId: sid, data }).catch(console.error);
    });
    pane.terminal.disposables.push(inputDisposable);

    // Terminal resize → send to SSH
    const resizeDisposable = pane.terminal.terminal.onResize(({ cols, rows }) => {
      invoke('terminal_resize', { sessionId: sid, cols, rows }).catch(console.error);
    });
    pane.terminal.disposables.push(resizeDisposable);
  }
  ```

  **Disconnect All handler**:
  ```typescript
  async function handleDisconnectAll() {
    try {
      await invoke('deactivate_workset');
      // Cleanup event listeners
      for (const unlisten of unlisteners) { unlisten(); }
      unlisteners.length = 0;
      // Destroy workspace
      destroyWorkspace(activeWorkspace.panes);
      activeWorkspace = null;
      hideWorkspaceView();
      if (selectedWorksetId) selectWorkset(selectedWorksetId);
      showToast('Disconnected all sessions', 'success');
    } catch (err) {
      showToast(`Disconnect failed: ${String(err)}`, 'error');
    }
  }
  ```

  **App close cleanup** (Tauri close_requested):
  ```typescript
  import { getCurrentWindow } from '@tauri-apps/api/window';
  // In DOMContentLoaded:
  getCurrentWindow().onCloseRequested(async () => {
    if (activeWorkspace) {
      await invoke('deactivate_workset');
    }
  });
  ```

  **Layout toolbar preset switching** (while workspace is active):
  - Switching presets: disconnect all, destroy workspace, re-create with new grid dimensions
  - This is a heavyweight operation but correct for MVP

  **Double-activation prevention**:
  - `activate_workset` IPC command already calls `disconnect_all()` first (Task 6)
  - Frontend also checks `if (activeWorkspace) handleDisconnectAll()` before activating

  **Must NOT do**:
  - Do NOT implement auto-reconnect
  - Do NOT implement OS Keystore password retrieval
  - Do NOT implement SshConfig auth
  - Do NOT add file browser or markdown viewer to panes
  - Do NOT implement custom error modals (plain window.prompt for passwords, toast for errors)

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Full-stack integration wiring Rust IPC commands to frontend event listeners across multiple modules. This is the most complex task — touching lib.rs types, main.ts, workspace.ts, and coordinating async event flows.
  - **Skills**: `['frontend-ui-ux']`
    - `frontend-ui-ux`: UI state transitions during activation, error display

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4 (solo — final integration)
  - **Blocks**: None (this is the final task)
  - **Blocked By**: Task 6 (IPC commands must exist), Task 7 (workspace view UI must exist)

  **References**:

  **Pattern References**:
  - `src/main.ts:107-116` — `loadWorksets()` async pattern with invoke + error handling. Follow same try/catch + showToast pattern for activate/deactivate.
  - `src/main.ts:118-127` — `selectWorkset()` async pattern. Activation flow follows similar structure.
  - `src/main.ts:129-163` — `saveWorkset()` shows how to collect form data and invoke commands. Reference for password collection flow.
  - `src/main.ts:629-641` — `DOMContentLoaded` event handler. Add close_requested handler and activate button wiring here.
  - `src-tauri/src/lib.rs` (from Task 6) — `activate_workset` command signature: accepts `workset_id: String` + `passwords: Vec<Option<String>>`, returns `Vec<SessionInfo>`.
  - `src/workspace.ts` (from Task 5) — `createWorkspace()`, `destroyWorkspace()`, `writeToPaneBuffer()`, `PaneState` interface.
  - `src/grid.ts` (from Task 2) — `createLayoutToolbar()`, `GRID_PRESETS`.

  **External References**:
  - Tauri event API: `@tauri-apps/api/event` — `listen()` returns `UnlistenFn` (must be called on cleanup)
  - Tauri window API: `@tauri-apps/api/window` — `getCurrentWindow().onCloseRequested()` for app close cleanup
  - xterm.js onData: `https://xtermjs.org/docs/api/terminal/classes/terminal/#ondata` — fires when user types, provides string data
  - xterm.js onResize: `https://xtermjs.org/docs/api/terminal/classes/terminal/#onresize` — fires after FitAddon.fit(), provides { cols, rows }

  **Acceptance Criteria**:
  - [ ] "Activate" button click triggers the full activation flow
  - [ ] Password prompt appears for password-auth connections via `window.prompt()`
  - [ ] Grid renders with correct layout from workset.grid_layout
  - [ ] Terminal panes created for each connection (up to grid cell count)
  - [ ] `listen('terminal-output-{id}')` registered for each session
  - [ ] `terminal.onData()` fires `invoke('terminal_input')` for each session
  - [ ] `terminal.onResize()` fires `invoke('terminal_resize')` for each session
  - [ ] "Disconnect All" button cleans up all listeners, destroys workspace, returns to detail view
  - [ ] App close (`onCloseRequested`) calls `deactivate_workset`
  - [ ] All unlisteners collected and cleaned up on disconnect
  - [ ] `cargo check` → exit 0
  - [ ] `npm run build` → exit 0
  - [ ] `cargo build --release` → exit 0

  **Agent-Executed QA Scenarios**:

  ```
  Scenario: Full build pipeline passes after integration
    Tool: Bash
    Steps:
      1. Run: cargo check --manifest-path src-tauri/Cargo.toml 2>&1
      2. Assert: exit code 0
      3. Run: npm run build 2>&1
      4. Assert: exit code 0
      5. Run: cargo build --release --manifest-path src-tauri/Cargo.toml 2>&1 | tail -3
      6. Assert: exit code 0, "Finished" in output
    Expected Result: All three builds pass cleanly
    Evidence: Build outputs captured

  Scenario: Integration wiring is present
    Tool: Bash
    Steps:
      1. grep -c "terminal-output-" src/main.ts → assert >= 1
      2. grep -c "terminal_input" src/main.ts → assert >= 1
      3. grep -c "terminal_resize" src/main.ts → assert >= 1
      4. grep -c "activate_workset" src/main.ts → assert >= 1
      5. grep -c "deactivate_workset" src/main.ts → assert >= 1
      6. grep -c "onCloseRequested" src/main.ts → assert >= 1
      7. grep -c "onData" src/main.ts OR src/workspace.ts → assert >= 1
    Expected Result: All IPC wiring present in frontend
    Evidence: grep output captured

  Scenario: Module structure complete
    Tool: Bash
    Steps:
      1. ls src-tauri/src/ssh/session.rs src-tauri/src/ssh/mod.rs → assert both exist
      2. ls src/grid.ts src/terminal.ts src/workspace.ts → assert all exist
      3. grep "mod ssh" src-tauri/src/lib.rs → assert present
      4. grep "SshConnectionManager" src-tauri/src/lib.rs → assert present
    Expected Result: All files created, modules registered
    Evidence: ls + grep output captured
  ```

  **Commit**: YES
  - Message: `feat(core): wire end-to-end workset activation with SSH terminals`
  - Files: `src/main.ts`, `src/workspace.ts` (if modified)
  - Pre-commit: `cargo build --release --manifest-path src-tauri/Cargo.toml && npm run build`

---

## Commit Strategy

| After Task | Message | Key Files | Verification |
|------------|---------|-----------|--------------|
| 1 | `feat(ssh): add SSH session core with PTY worker thread` | `ssh/session.rs` | `cargo check` |
| 2 | `feat(grid): add CSS Grid layout engine with 5 preset layouts` | `grid.ts`, `styles.css`, `index.html` | `npm run build` |
| 3 | `feat(terminal): add xterm.js terminal pane component with WebGL` | `terminal.ts` | `npm run build` |
| 4 | `feat(ssh): add connection manager with parallel connect and cleanup` | `ssh/mod.rs` | `cargo check` |
| 5 | `feat(workspace): integrate terminals into grid with output batching` | `workspace.ts` | `npm run build` |
| 6 | `feat(ipc): add SSH terminal IPC commands and state registration` | `lib.rs` | `cargo build --release` |
| 7 | `feat(ui): add workspace view, pane status indicators, and activation button` | `main.ts`, `styles.css` | `npm run build` |
| 8 | `feat(core): wire end-to-end workset activation with SSH terminals` | `main.ts`, `workspace.ts` | `cargo build --release && npm run build` |

---

## Success Criteria

### Verification Commands
```bash
# After all 8 tasks complete:
cargo check --manifest-path src-tauri/Cargo.toml   # Expected: exit 0, zero errors
npm run build                                       # Expected: exit 0, tsc + vite succeed
npx tsc --noEmit                                    # Expected: exit 0, zero type errors
cargo build --release --manifest-path src-tauri/Cargo.toml  # Expected: exit 0, Finished message
```

### File Checklist
```bash
# All new files exist:
ls src-tauri/src/ssh/session.rs   # SSH session core
ls src-tauri/src/ssh/mod.rs       # SSH connection manager
ls src/grid.ts                    # Grid layout engine
ls src/terminal.ts                # Terminal pane component
ls src/workspace.ts               # Grid-terminal integration
```

### Final Checklist
- [ ] All "Must Have" features present (Key auth, Password auth, bidirectional I/O, 5 grid presets, status indicators, VM labels, Disconnect All, worker cleanup, 10K scrollback, WebGL+Canvas)
- [ ] All "Must NOT Have" guardrails respected (no password on disk, no SshConfig parsing, no drag-resize, no auto-reconnect, no content-type switching, no OS Keystore, no tokio tasks for ssh2, no tests in this batch)
- [ ] SSH module uses `std::thread` with `std::sync::mpsc` (NOT tokio)
- [ ] All new Tauri commands are `async fn`
- [ ] Frontend code split into modules (grid.ts, terminal.ts, workspace.ts)
- [ ] Terminal writes batched via requestAnimationFrame
- [ ] Resize debounced at 100ms
- [ ] App close triggers SSH session cleanup
- [ ] Double activation prevented
- [ ] All 3 build commands pass: `cargo check`, `npm run build`, `cargo build --release`
