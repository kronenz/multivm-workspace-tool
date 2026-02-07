# MVP Specification — Multi-VM AI Agent Workspace Tool

## Document Information

| Field | Value |
|-------|-------|
| **Document Type** | MVP Specification |
| **Version** | 1.0 |
| **Date** | 2026-02-07 |
| **Status** | Draft |
| **Related Documents** | [Glossary](./glossary.md), [PRD](./prd.md), [Architecture](./architecture.md), [Market Research](./market-research.md) |

---

## Executive Summary

This document defines the **Minimum Viable Product (MVP)** scope for the Multi-VM AI Agent Workspace Tool. The MVP delivers exactly **10 core features** that enable developers to manage 2-10 remote VMs with AI coding agents in a unified desktop workspace.

**MVP Goal**: Validate the core value proposition — "Replace 10+ terminal windows with a single desktop app that auto-launches AI agents across multiple VMs" — with the minimum feature set required for real-world usage.

**Target Users**: Individual developers working with AI coding assistants (Claude Code, OpenCode) across multiple remote development environments.

**Success Criteria**: 
- 20+ early adopters use the MVP for daily work (30+ minutes/day)
- 80%+ of users create 2+ Worksets (validates multi-project use case)
- 60%+ of users use 2x2 or larger grid layouts (validates multi-VM use case)

---

## MVP Feature List (Exactly 10 Features)

### Feature 1: Workset Profile Management (CRUD)

**User Story**: As a developer managing multiple projects across different VMs, I want to save my workspace configurations (SSH details, project paths, AI CLI commands, grid layouts) as reusable profiles, so that I can restore my entire development environment with one click.

**Description**: 
Create, Read, Update, Delete (CRUD) operations for Workset profiles. Each Workset stores:
- SSH connection details (host, port, username, authentication method)
- Project folder path on remote VM
- AI CLI command to auto-execute (e.g., `claude-code`, `opencode --model sonnet`)
- Grid layout configuration (preset or custom NxM)

Worksets are persisted as JSON files in `~/.config/multivm-workspace/worksets/`.

**Done Criteria**:
- [ ] User can create a new Workset via UI form (all fields: SSH host, port, user, auth method, project path, AI CLI command, grid layout)
- [ ] User can save Workset to disk (JSON file created in config directory)
- [ ] User can view list of saved Worksets in sidebar (with search/filter by name)
- [ ] User can edit existing Workset (modify any field, save changes)
- [ ] User can delete Workset (with confirmation dialog)
- [ ] User can activate Workset (triggers SSH connection, AI CLI launch, grid layout restoration)
- [ ] App restart preserves all saved Worksets (JSON files persist across sessions)

**PRD Mapping**: MUST-1

---

### Feature 2: SSH Connection (Key-Based, Password, ~/.ssh/config)

**User Story**: As a developer with existing SSH configurations, I want to connect to remote VMs using my SSH keys, passwords, or `~/.ssh/config` aliases, so that I don't have to reconfigure authentication methods I already use.

**Description**:
Establish SSH connections to remote VMs with three authentication methods:
1. **SSH Key-Based**: User provides path to private key file (e.g., `~/.ssh/id_rsa`)
2. **Password**: User enters password (stored securely in OS keystore)
3. **~/.ssh/config Integration**: User provides host alias from `~/.ssh/config`, app auto-detects host, port, user, identity file

Connection pooling: Maintain persistent SSH sessions for each VM. Multiple components (Terminal, File Browser, Resource Poller) share the same SSH connection via channel multiplexing.

**Done Criteria**:
- [ ] User can connect to VM using SSH key file path (authentication succeeds, terminal prompt appears)
- [ ] User can connect to VM using password (password stored in OS keystore: macOS Keychain, Linux Secret Service, Windows Credential Manager)
- [ ] User can connect to VM using `~/.ssh/config` alias (app parses config file, extracts host/port/user/identity file, connects successfully)
- [ ] SSH connection survives for 30+ minutes without manual keepalive (ServerAliveInterval configured)
- [ ] Connection failure shows clear error message (e.g., "Authentication failed", "Host unreachable", "Timeout")
- [ ] User can disconnect from VM (SSH session closed cleanly)

**PRD Mapping**: MUST-2

---

### Feature 3: Terminal Emulator (xterm.js-based, 256-color/truecolor)

**User Story**: As a developer running AI coding agents and interactive CLI tools (vim, htop) on remote VMs, I want a high-performance terminal emulator with full color support and escape sequence handling, so that I can interact with remote shells as if I were using a native terminal.

**Description**:
Interactive terminal interface for each remote VM, rendered using xterm.js with WebGL renderer (Canvas fallback). Supports:
- 256-color and truecolor (24-bit RGB)
- Full VT100/VT220 escape sequence handling
- Copy/paste (Ctrl+Shift+C/V on Linux/Windows, Cmd+C/V on macOS)
- Scrollback buffer (10,000 lines)
- Resizable terminal (fits pane dimensions)

Terminal I/O flows: User input → Frontend → IPC → Rust Core → SSH PTY → Remote VM → SSH PTY → Rust Core → IPC → Frontend → xterm.js rendering.

**Done Criteria**:
- [ ] User can type commands in terminal, see output in real-time (e.g., `ls -la`, `echo "test"`)
- [ ] User can run interactive TUI apps (vim, htop, nano) with full keyboard navigation
- [ ] User can run AI CLI tools (claude-code, opencode) and see colored output
- [ ] User can copy text from terminal (Ctrl+Shift+C), paste into terminal (Ctrl+Shift+V)
- [ ] User can scroll through 10,000+ lines of output without UI freeze (e.g., `cat large-log.txt`)
- [ ] Terminal displays 256-color and truecolor correctly (test with `curl -s https://gist.githubusercontent.com/lilyball/8b1b3e3e0c1e4c9e5e5e5e5e5e5e5e5e/raw/256-colors.sh | bash`)
- [ ] Terminal resizes when pane is resized (columns/rows adjust automatically)

**PRD Mapping**: MUST-3

---

### Feature 4: Grid Layout (Presets: 1x1, 2x1, 2x2 + Custom Split)

**User Story**: As a developer working with 4-6 VMs simultaneously, I want to arrange terminal panes, file browsers, and markdown viewers in a flexible grid layout, so that I can view multiple remote environments side-by-side without switching windows.

**Description**:
Visual layout engine that arranges UI components (Terminal, File Browser, Markdown Viewer) in an NxM grid. Supports:
- **Preset Layouts**: 1x1 (single pane), 2x1 (horizontal split), 2x2 (quad grid)
- **Custom Splits**: User-defined NxM grids (e.g., 2x3, 3x2, 1x3)
- **Drag-to-Resize**: User can drag pane dividers to adjust relative sizes
- **Content Assignment**: Each pane can contain Terminal, File Browser, or Markdown Viewer

Layout state is saved in Workset profile and restored on activation.

**Done Criteria**:
- [ ] User can select preset layout (1x1, 2x1, 2x2) from UI menu
- [ ] User can create custom NxM layout (e.g., 2x3) via layout editor
- [ ] User can drag pane dividers to resize panes (smooth animation, <50ms response)
- [ ] User can assign content type to each pane (Terminal, File Browser, Markdown Viewer) via dropdown or context menu
- [ ] User can assign different VM connections to different panes (e.g., Pane 1 = VM A terminal, Pane 2 = VM B terminal)
- [ ] Layout state persists in Workset (saved to JSON, restored on activation)
- [ ] Pane focus indicator (active pane has colored border, inactive panes have gray border)

**PRD Mapping**: MUST-4

---

### Feature 5: File Browser (READ-ONLY, SFTP/SSH exec tree view)

**User Story**: As a developer verifying AI-generated code changes on remote VMs, I want a visual file tree browser, so that I can quickly navigate project structure and locate files without typing `ls` and `cd` commands repeatedly.

**Description**:
Read-only tree view of remote file system, accessed via SFTP or SSH exec commands (`ls -la`, `find`). Features:
- Expand/collapse folders
- File metadata display (size, last modified timestamp)
- Click file → open in Markdown Viewer (if `.md` extension) or show "read-only" message
- **Explicitly excluded**: File editing, file upload/download, file deletion, file creation

File Browser UI is rendered in Web Frontend, file system access is handled by Rust Core via SSH.

**Done Criteria**:
- [ ] User can browse remote file system starting from project folder (tree view with folders and files)
- [ ] User can expand folder to see contents (click folder → children appear)
- [ ] User can collapse folder (click expanded folder → children hide)
- [ ] User can see file metadata (size in KB/MB, last modified date/time)
- [ ] User can click `.md` file → file opens in Markdown Viewer pane
- [ ] User can click non-`.md` file → message "Read-only file browser. Use terminal to edit files."
- [ ] File browser updates when remote file system changes (manual refresh button or auto-refresh every 10 seconds)

**PRD Mapping**: MUST-5

---

### Feature 6: Markdown Viewer (Remote MD file rendering)

**User Story**: As a developer reviewing AI-generated documentation (README.md, API docs, CHANGELOG.md) on remote VMs, I want a formatted Markdown viewer, so that I can read documentation without downloading files or using poorly-formatted `cat` output.

**Description**:
Render Markdown files from remote VMs in a formatted, readable view. Features:
- Fetch file content via SFTP or SSH exec (`cat <file>`)
- Render with syntax highlighting for code blocks (using Prism.js or similar)
- Support: Headers, lists, tables, links, inline code, code blocks, images (if accessible via URL)
- Auto-refresh: Poll file every 5 seconds, re-render if content changes

Markdown Viewer UI is rendered in Web Frontend, file content is fetched by Rust Core via SSH.

**Done Criteria**:
- [ ] User can open `.md` file from File Browser → file renders in Markdown Viewer pane
- [ ] Markdown rendering supports: Headers (H1-H6), bold, italic, lists (ordered/unordered), tables, links, inline code, code blocks
- [ ] Code blocks have syntax highlighting (detect language from fence: ```python, ```javascript, etc.)
- [ ] User can click links in Markdown (external URLs open in browser, internal links show "not supported" message)
- [ ] Markdown Viewer auto-refreshes when file changes on remote VM (5-second polling interval)
- [ ] User can manually refresh Markdown Viewer (refresh button)

**PRD Mapping**: MUST-6

---

### Feature 7: Resource Monitoring (CPU/RAM/Disk snapshot, periodic refresh)

**User Story**: As a developer running resource-intensive AI agents on multiple VMs, I want real-time visibility into CPU, RAM, and Disk usage, so that I can identify resource bottlenecks and prevent VM crashes before they happen.

**Description**:
Display snapshot values of remote VM system resources (CPU %, RAM %, Disk %) with periodic updates. Features:
- **Collection Method**: SSH exec commands (`top -bn1`, `free -m`, `df -h`) executed every 5 seconds
- **Parsing**: Extract CPU %, RAM %, Disk % from command output (OS-specific parsing logic)
- **Display Format**: Percentage values with color coding (green <50%, yellow 50-80%, red >80%)
- **Update Frequency**: Every 5 seconds (±1 second)
- **Explicitly excluded**: Time-series graphs, historical data storage, alerting, anomaly detection

Resource Poller runs in Rust Core, sends updates to Frontend via IPC Events.

**Done Criteria**:
- [ ] User sees CPU usage % for each connected VM (updated every 5 seconds)
- [ ] User sees RAM usage % for each connected VM (updated every 5 seconds)
- [ ] User sees Disk usage % for each connected VM (updated every 5 seconds)
- [ ] Resource values have color coding (green <50%, yellow 50-80%, red >80%)
- [ ] Resource monitoring works on Ubuntu 22.04, Debian 11, CentOS 8 (Linux variants)
- [ ] Resource monitoring gracefully fails on unsupported OS (shows "N/A" instead of crashing)
- [ ] User can see resource values in dedicated pane or status bar (UI placement TBD)

**PRD Mapping**: MUST-7

**Architecture Mapping**: Resource Poller component (Architecture Blueprint)

---

### Feature 8: AI CLI Auto-Launch (Workset-defined CLI command auto-execution)

**User Story**: As a developer who runs the same AI CLI commands (claude-code, opencode) every time I connect to a VM, I want the app to automatically execute these commands when I activate a Workset, so that I don't have to type them manually 10+ times per day.

**Description**:
Automatically execute AI CLI commands when Workset is activated. Workflow:
1. User activates Workset
2. App establishes SSH connection to VM
3. App executes `cd <project_folder>` (from Workset config)
4. App executes `<ai_cli_command>` (from Workset config, e.g., `claude-code`, `opencode --model sonnet`)
5. Terminal shows AI CLI output immediately

AI CLI command is stored in Workset profile. User can edit command in Workset settings.

**Done Criteria**:
- [ ] User can specify AI CLI command in Workset creation form (text input field)
- [ ] User activates Workset → terminal automatically runs `cd <project_folder> && <ai_cli_command>`
- [ ] User sees AI CLI output in terminal (e.g., Claude Code welcome message, prompt)
- [ ] User can interact with AI CLI immediately (no manual command typing required)
- [ ] User can edit AI CLI command in Workset settings (change command, save, re-activate Workset)
- [ ] If AI CLI command fails (e.g., command not found), terminal shows error message (does not crash app)

**PRD Mapping**: MUST-8

**Architecture Mapping**: Process Manager component (Architecture Blueprint)

---

### Feature 9: SSH Auto-Reconnect (Connection drop auto-reconnection)

**User Story**: As a developer working on unstable networks (Wi-Fi, VPN), I want the app to automatically reconnect to VMs when SSH connections drop, so that I don't have to manually reconnect and lose my workflow context.

**Description**:
Detect SSH connection drops and attempt automatic reconnection. Features:
- **Detection**: Monitor SSH connection state (keepalive packets, read/write errors)
- **Reconnection Logic**: Max 3 retries, 5-second intervals between retries, exponential backoff with jitter
- **UI Feedback**: Show "Reconnecting... (1/3)" message in terminal pane
- **Session Recovery**: After reconnection, restore terminal state (re-execute `cd <project_folder>` if needed)
- **Failure Handling**: After 3 failed retries, show "Connection lost. Click to reconnect manually."

Auto-reconnect is handled by SSH Connection Manager in Rust Core.

**Done Criteria**:
- [ ] User's SSH connection drops (simulate by killing SSH server or blocking network) → app detects drop within 10 seconds
- [ ] App automatically attempts reconnection (max 3 retries, 5-second intervals)
- [ ] User sees "Reconnecting... (1/3)" message in terminal pane during reconnection
- [ ] Reconnection succeeds → terminal prompt reappears, user can continue working
- [ ] Reconnection fails after 3 retries → user sees "Connection lost. Click to reconnect manually." with reconnect button
- [ ] User clicks manual reconnect button → app attempts new connection
- [ ] Auto-reconnect success rate ≥90% in network interruption tests (NFR-8)

**PRD Mapping**: MUST-2 (SSH Connection Management), NFR-8 (Auto-Reconnect Success Rate)

**Architecture Mapping**: SSH Connection Manager component (Architecture Blueprint)

---

### Feature 10: Dark/Light Theme

**User Story**: As a developer who works in different lighting conditions (bright office, dark room), I want to switch between dark and light themes, so that I can reduce eye strain and match my OS theme preference.

**Description**:
Toggle between dark mode and light mode themes. Theme applies to:
- Application window background
- Terminal background and text colors
- File Browser background and text colors
- Markdown Viewer background and text colors
- UI controls (buttons, inputs, dropdowns)

Theme preference is saved in app settings and persists across sessions.

**Done Criteria**:
- [ ] User can toggle theme via settings menu (Dark/Light toggle switch)
- [ ] Dark theme: Dark backgrounds, light text (e.g., #1e1e1e background, #d4d4d4 text)
- [ ] Light theme: Light backgrounds, dark text (e.g., #ffffff background, #333333 text)
- [ ] Theme applies to all UI components (terminal, file browser, markdown viewer, sidebar, status bar)
- [ ] Theme preference persists across app restarts (saved in config file)
- [ ] Theme change takes effect immediately (no app restart required)

**PRD Mapping**: SHOULD-1 (promoted to MUST for MVP to improve usability)

---

## Explicit Exclusions (NOT in MVP)

> **Purpose**: This section lists features, capabilities, and scope items that are **explicitly excluded** from the MVP. These may be considered for post-MVP releases but are NOT part of the initial product.

### Exclusion 1: Team Features (Multi-User, Session Sharing)

**What's Excluded**:
- Multi-user authentication and authorization
- Real-time session sharing (multiple users viewing/controlling the same terminal)
- Shared Workset libraries with live sync
- User roles and permissions (admin, viewer, editor)
- Team collaboration features (chat, comments, annotations)

**Rationale**: MVP targets single-user workflows. Team features require complex infrastructure (auth servers, WebSocket sync, conflict resolution) that would delay MVP launch by 3-6 months.

**Post-MVP Consideration**: Could be added in v2.0 if user demand is validated.

---

### Exclusion 2: Cloud APIs (AWS/GCP/Azure VM Provisioning)

**What's Excluded**:
- Integration with cloud provider APIs (AWS EC2, GCP Compute Engine, Azure VMs)
- VM provisioning, starting, stopping, resizing via app UI
- Cloud resource cost monitoring
- Auto-scaling or VM lifecycle management

**Rationale**: This product connects to **existing** VMs via SSH. VM provisioning is a separate concern handled by cloud consoles, Terraform, or other IaC tools. Adding cloud API integration would expand scope significantly and require cloud-specific authentication flows.

**Post-MVP Consideration**: Could add "quick launch" integrations if users request it, but not core to value proposition.

---

### Exclusion 3: Plugins (Extension System, Third-Party Integrations)

**What's Excluded**:
- Plugin architecture (hooks, APIs for third-party extensions)
- Plugin marketplace or registry
- Custom UI components via plugins
- Third-party integrations (Slack, GitHub, Jira, etc.)

**Rationale**: Plugin systems add architectural complexity (API stability, security sandboxing, versioning). MVP must validate core product-market fit before investing in extensibility.

**Post-MVP Consideration**: High priority for v2.0 if community requests custom integrations.

---

### Exclusion 4: File Editing (Text Editor, Code Editor)

**What's Excluded**:
- In-app text editor or code editor
- File upload/download via UI
- File creation, deletion, renaming via UI
- Syntax highlighting for code editing
- Multi-file editing or project-wide search-and-replace

**Rationale**: Users already have preferred editors (vim, VS Code Remote, Emacs). MVP provides **read-only** file browsing to verify AI-generated changes. Editing is out of scope.

**Post-MVP Consideration**: Unlikely to add; read-only browsing is sufficient for target use case.

---

### Exclusion 5: Git Integration (Commit, Push, Pull, Diff)

**What's Excluded**:
- Git commit, push, pull, fetch via UI
- Diff visualization (side-by-side, inline)
- Branch management (create, switch, merge, rebase)
- Git history viewer (log, blame, bisect)
- Conflict resolution UI

**Rationale**: Git operations are well-served by existing tools (terminal git, VS Code Git, GitKraken, Sublime Merge). Not a differentiator for this product.

**Post-MVP Consideration**: Low priority; users can use terminal for Git commands.

---

### Exclusion 6: Session Sharing (Real-Time Collaboration)

**What's Excluded**:
- Share terminal session URL with other users
- Real-time cursor positions and typing indicators
- Session recording and playback (asciinema-style)
- Session export/import for sharing

**Rationale**: Session sharing requires WebSocket infrastructure, authentication, and conflict resolution. MVP is single-user only.

**Post-MVP Consideration**: Could add session recording (local-only) in v1.5, real-time sharing in v2.0.

---

### Exclusion 7: Agent Orchestration (Multi-Agent Coordination)

**What's Excluded**:
- Controlling AI agent behavior (task distribution, priority, stop/start)
- Multi-agent coordination (agent A waits for agent B to finish)
- Task queue management across agents
- Agent output merging or conflict resolution

**Rationale**: This product **launches** AI CLI tools on remote VMs. It does NOT orchestrate or control AI agent behavior. AI agents (Claude Code, OpenCode) manage their own workflows.

**Post-MVP Consideration**: Out of scope permanently; not aligned with product vision.

---

### Exclusion 8: Custom Themes (Beyond Dark/Light)

**What's Excluded**:
- User-created color schemes
- Theme marketplace or sharing
- Per-component theme customization (terminal colors separate from UI colors)
- Syntax highlighting theme customization

**Rationale**: Dark and light modes cover 95% of user preferences. Custom themes are polish, not core functionality.

**Post-MVP Consideration**: Low priority; could add in v1.5 if users request it.

---

### Exclusion 9: Notifications/Alerts (Resource Thresholds, Events)

**What's Excluded**:
- Desktop notifications (e.g., "CPU usage >90% on VM A")
- Alert rules (e.g., "Notify me when Disk >80%")
- Event logging (e.g., "SSH connection dropped at 10:32 AM")
- Alert history or notification center

**Rationale**: MVP provides real-time resource monitoring with color coding. Alerting requires background monitoring, notification permissions, and alert management UI.

**Post-MVP Consideration**: Could add in v1.5 if users request proactive alerts.

---

### Exclusion 10: Time-Series Graphs (Historical Resource Data)

**What's Excluded**:
- Time-series graphs (CPU/RAM/Disk over time)
- Historical data storage (database, CSV export)
- Trend analysis or anomaly detection
- Resource usage reports or dashboards

**Rationale**: MVP provides **snapshot values only** (current CPU %, RAM %, Disk %). Historical data requires database infrastructure and analytics.

**Post-MVP Consideration**: Could add in v2.0 if users request historical analysis.

---

## End-to-End User Journey (E2E Scenario)

> **Purpose**: This section describes a complete user workflow from start to finish, demonstrating how all 10 MVP features work together in a real-world scenario.

### Scenario: New Workset Creation → 2x2 Grid with 4 VMs → AI Agents Running → Code Verification

**Persona**: Jordan Kim (Startup Developer with Parallel AI Workflows)

**Context**: Jordan is refactoring 4 microservices (auth, payment, inventory, shipping) in parallel using Claude Code. Each service runs on a separate GCP VM.

---

#### Step 1: Create New Workset

**Action**: Jordan opens the app and clicks "New Workset" button in the sidebar.

**UI**: Workset creation form appears with fields:
- Workset Name: `Microservices Refactor`
- Grid Layout: `2x2` (selected from preset dropdown)
- VM 1 (Top-Left Pane):
  - SSH Host: `auth-vm.example.com`
  - Port: `22`
  - User: `jordan`
  - Auth Method: `SSH Key` (selected from dropdown)
  - Key Path: `~/.ssh/gcp_key`
  - Project Folder: `/home/jordan/auth-service`
  - AI CLI Command: `claude-code`
- VM 2 (Top-Right Pane):
  - SSH Host: `payment-vm.example.com`
  - Port: `22`
  - User: `jordan`
  - Auth Method: `~/.ssh/config alias` (selected from dropdown)
  - Config Alias: `payment-vm`
  - Project Folder: `/home/jordan/payment-service`
  - AI CLI Command: `claude-code`
- VM 3 (Bottom-Left Pane):
  - SSH Host: `inventory-vm.example.com`
  - Port: `22`
  - User: `jordan`
  - Auth Method: `SSH Key`
  - Key Path: `~/.ssh/gcp_key`
  - Project Folder: `/home/jordan/inventory-service`
  - AI CLI Command: `claude-code`
- VM 4 (Bottom-Right Pane):
  - SSH Host: `shipping-vm.example.com`
  - Port: `22`
  - User: `jordan`
  - Auth Method: `SSH Key`
  - Key Path: `~/.ssh/gcp_key`
  - Project Folder: `/home/jordan/shipping-service`
  - AI CLI Command: `claude-code`

**Action**: Jordan clicks "Save Workset" button.

**Result**: Workset saved to `~/.config/multivm-workspace/worksets/microservices-refactor.json`. Workset appears in sidebar list.

**Features Used**: Feature 1 (Workset Profile Management)

---

#### Step 2: Activate Workset → 2x2 Grid with 4 VMs Simultaneous Connection

**Action**: Jordan clicks "Microservices Refactor" Workset in sidebar.

**UI**: App window transitions to 2x2 grid layout. Each pane shows "Connecting to <VM>..." message.

**Backend**:
1. SSH Connection Manager establishes 4 SSH connections in parallel (Feature 2)
2. Each connection uses specified auth method (SSH key for VM 1/3/4, `~/.ssh/config` for VM 2)
3. Process Manager executes `cd <project_folder>` on each VM (Feature 8)
4. Process Manager executes `claude-code` on each VM (Feature 8)
5. Resource Poller starts collecting CPU/RAM/Disk for each VM (Feature 7)

**Result**: After 5-8 seconds, all 4 panes show terminal prompts with Claude Code welcome messages.

**Features Used**: Feature 1 (Workset Activation), Feature 2 (SSH Connection), Feature 4 (Grid Layout), Feature 8 (AI CLI Auto-Launch), Feature 7 (Resource Monitoring starts)

---

#### Step 3: Each VM Runs Claude Code → Parallel Refactoring

**Action**: Jordan types refactoring instructions in each terminal:
- VM 1 (auth-service): `Refactor JWT validation to use RS256 instead of HS256`
- VM 2 (payment-service): `Add retry logic to Stripe API calls`
- VM 3 (inventory-service): `Optimize database queries in product search`
- VM 4 (shipping-service): `Add rate limiting to shipping cost API`

**UI**: Each terminal shows Claude Code processing the request, generating code, running tests.

**Backend**: Terminal Emulator (Feature 3) handles:
- User input → IPC → Rust Core → SSH PTY → Remote VM
- Remote VM output → SSH PTY → Rust Core → IPC → xterm.js rendering
- 256-color output, escape sequences, scrollback buffer

**Result**: After 2-5 minutes, Claude Code completes refactoring in all 4 terminals. Jordan sees success messages and file change summaries.

**Features Used**: Feature 3 (Terminal Emulator)

---

#### Step 4: File Browser Verifies Generated Code

**Action**: Jordan wants to verify that Claude Code correctly updated the JWT validation file in auth-service.

**UI**: Jordan right-clicks on Top-Left pane (auth-service terminal) → selects "Open File Browser" from context menu.

**Result**: File Browser pane replaces terminal in Top-Left pane. Jordan sees file tree:
```
/home/jordan/auth-service/
├── src/
│   ├── auth/
│   │   ├── jwt.ts (modified 2 minutes ago)
│   │   ├── middleware.ts
│   ├── routes/
│   ├── utils/
├── tests/
├── package.json
├── README.md
```

**Action**: Jordan clicks `src/auth/jwt.ts` in file tree.

**Result**: File Browser shows "Read-only file browser. Use terminal to edit files." message (Feature 5 excludes editing).

**Action**: Jordan clicks `README.md` in file tree.

**Result**: Markdown Viewer pane opens (replaces File Browser). Jordan sees formatted README with updated documentation about RS256 JWT validation.

**Features Used**: Feature 5 (File Browser), Feature 6 (Markdown Viewer)

---

#### Step 5: Resource Monitoring Checks CPU

**Action**: Jordan notices that VM 3 (inventory-service) terminal is slow to respond. Jordan looks at resource monitoring display in status bar.

**UI**: Status bar shows:
- VM 1 (auth-vm): CPU 12% (green), RAM 45% (green), Disk 62% (yellow)
- VM 2 (payment-vm): CPU 8% (green), RAM 38% (green), Disk 58% (yellow)
- VM 3 (inventory-vm): **CPU 89% (red)**, RAM 72% (yellow), Disk 65% (yellow)
- VM 4 (shipping-vm): CPU 15% (green), RAM 41% (green), Disk 60% (yellow)

**Insight**: VM 3 is CPU-saturated (89%). Jordan realizes the database query optimization is running a heavy migration script.

**Action**: Jordan decides to wait for VM 3 to finish before assigning new tasks.

**Result**: After 3 minutes, VM 3 CPU drops to 18% (green). Jordan can continue working.

**Features Used**: Feature 7 (Resource Monitoring)

---

#### Step 6: Network Interruption → Auto-Reconnect

**Action**: Jordan's Wi-Fi briefly disconnects (5 seconds) due to router reboot.

**UI**: All 4 terminal panes show "Connection lost. Reconnecting... (1/3)" message.

**Backend**: SSH Connection Manager detects connection drops, attempts reconnection with 5-second intervals.

**Result**: After 8 seconds, all 4 SSH connections successfully reconnect. Terminals show prompts again. Jordan can continue working without manual intervention.

**Features Used**: Feature 9 (SSH Auto-Reconnect)

---

#### Step 7: Switch to Light Theme (Optional)

**Action**: Jordan moves to a brightly-lit conference room and finds dark theme hard to read. Jordan opens Settings → toggles "Theme" to "Light".

**UI**: App immediately switches to light theme:
- Window background: White
- Terminal background: Light gray
- Text: Dark gray/black
- File Browser: White background, dark text
- Markdown Viewer: White background, dark text

**Result**: Jordan can read the screen comfortably in bright lighting.

**Features Used**: Feature 10 (Dark/Light Theme)

---

#### Step 8: Save and Exit

**Action**: Jordan finishes refactoring session. Jordan closes the app.

**Result**: Workset state is saved (grid layout, pane assignments). SSH connections are closed cleanly. Next time Jordan opens the app and activates "Microservices Refactor" Workset, the exact same 2x2 layout with 4 VMs is restored.

**Features Used**: Feature 1 (Workset Persistence)

---

### E2E Journey Summary

**Total Time**: ~20 minutes (5 min setup, 10 min AI refactoring, 5 min verification)

**Features Demonstrated**: All 10 MVP features used in realistic workflow

**Value Delivered**:
- **Before**: Jordan would have 12+ terminal windows open (4 VMs × 3 terminals each), manually typing SSH commands, `cd` paths, and `claude-code` commands 10+ times
- **After**: Jordan clicks 1 Workset, waits 8 seconds, and has 4 VMs with AI agents running in a unified workspace

**Success Metrics**:
- Workset activation time: 8 seconds (target: <10 seconds, NFR-10)
- Auto-reconnect success: 4/4 connections (100%, target: 90%, NFR-8)
- Resource monitoring update frequency: 5 seconds (target: 5±1 seconds, NFR-4)

---

## Technical Spike Priorities (from Architecture Document)

> **Purpose**: This section prioritizes technical spikes (prototypes) that must be completed **before** MVP development to validate architectural assumptions and reduce risk.

### SPIKE-1: Tauri + xterm.js Latency Verification (CRITICAL — MUST DO BEFORE MVP)

**Priority**: **CRITICAL** — Blocks MVP development

**Risk Addressed**: RISK-1 (Tauri WebView + xterm.js terminal latency)

**Objective**: Validate that Tauri WebView + xterm.js combination delivers acceptable terminal performance (<50ms key input latency, 10,000-line output without UI freeze).

**Validation Criteria**:
- [ ] Key input latency <50ms on macOS, Ubuntu, Windows
- [ ] 10,000-line output (`cat large-file.txt`) renders in <100ms
- [ ] vim, htop, nano work correctly (full keyboard navigation, no visual glitches)
- [ ] WebGL renderer works on all 3 OS (fallback to Canvas if needed)

**Prototype Scope**:
- Minimal Tauri app with single xterm.js instance
- Local PTY (no SSH) → SSH added in second iteration
- Benchmark on macOS 11+, Ubuntu 20.04+, Windows 10+

**Timeline**: 1 week (before MVP sprint 1)

**Decision Point**: If latency >100ms or vim doesn't work → consider alternative terminal libraries (hterm, custom Canvas renderer) or abandon Tauri for Electron.

**Architecture Reference**: Architecture Blueprint § RISK-1, § SPIKE-1

---

### SPIKE-2: SSH Connection Pooling Stress Test (HIGH — SHOULD DO BEFORE MVP)

**Priority**: **HIGH** — Recommended before MVP, but not blocking

**Risk Addressed**: RISK-2 (Multi-SSH session stability), RISK-4 (IPC serialization bottleneck)

**Objective**: Validate that Rust SSH library can maintain 10 concurrent SSH sessions for 30+ minutes without crashes, memory leaks, or performance degradation.

**Validation Criteria**:
- [ ] 10 SSH sessions maintained for 30 minutes (0% abnormal termination)
- [ ] Auto-reconnect success rate ≥90% after simulated network interruptions
- [ ] Memory usage <200MB for 10 sessions
- [ ] CPU overhead <5% for keepalive + resource polling

**Prototype Scope**:
- Rust SSH library (e.g., `russh`, `ssh2-rs`) with 10 concurrent connections
- Each connection: PTY session + periodic exec commands (simulate resource polling)
- Network interruption simulation (iptables packet drop)

**Timeline**: 1 week (parallel with SPIKE-1 or after)

**Decision Point**: If reconnection success rate <80% → implement more aggressive keepalive or connection health checks.

**Architecture Reference**: Architecture Blueprint § RISK-2, § SPIKE-2

---

### SPIKE-3: Heterogeneous VM Compatibility Test (MEDIUM — OPTIONAL)

**Priority**: **MEDIUM** — Nice to have, can be deferred to MVP testing phase

**Risk Addressed**: RISK-3 (OS-specific resource monitoring command differences)

**Objective**: Validate that resource monitoring commands work on Ubuntu, Alpine Linux (BusyBox), macOS.

**Validation Criteria**:
- [ ] CPU/RAM/Disk values correctly extracted on Ubuntu 22.04
- [ ] CPU/RAM/Disk values correctly extracted on Alpine 3.18 (BusyBox)
- [ ] CPU/RAM/Disk values correctly extracted on macOS 14
- [ ] Graceful failure (show "N/A") when commands are missing

**Prototype Scope**:
- 3 VMs (Ubuntu, Alpine, macOS) with SSH access
- Resource collection script (Rust or shell) that runs OS-specific commands
- Parsing logic unit tests

**Timeline**: 3 days (can be done during MVP sprint 2)

**Decision Point**: If parsing fails on Alpine/macOS → implement OS detection + per-OS command strategy pattern.

**Architecture Reference**: Architecture Blueprint § RISK-3, § SPIKE-3

---

## Resource Monitoring Detailed Scope

> **Purpose**: This section provides detailed specifications for Feature 7 (Resource Monitoring) to clarify MVP scope and prevent scope creep.

### MVP Scope: Snapshot Values Only

**What's Included**:
- **CPU Usage %**: Current CPU utilization percentage (0-100%)
- **RAM Usage %**: Current memory utilization percentage (0-100%)
- **Disk Usage %**: Current disk space utilization percentage (0-100%)
- **Update Frequency**: Every 5 seconds (±1 second)
- **Display Format**: Percentage values with color coding (green <50%, yellow 50-80%, red >80%)
- **UI Placement**: Status bar or dedicated resource monitoring pane (TBD during UI design)

**Collection Method**:
1. **SSH exec commands** (executed by Resource Poller in Rust Core):
   - CPU: `top -bn1 | grep "Cpu(s)"` (Linux) or `top -l1 | grep "CPU usage"` (macOS)
   - RAM: `free -m | grep "Mem:"` (Linux) or `vm_stat` (macOS)
   - Disk: `df -h /` (Linux/macOS)
2. **Parsing**: Extract percentage values from command output (OS-specific regex patterns)
3. **Event emission**: Send `resource_update` event to Frontend via IPC (JSON payload: `{ vmId, cpu, ram, disk }`)
4. **Frontend rendering**: Update UI with new values, apply color coding

**OS Compatibility**:
- **Ubuntu/Debian**: `top`, `free`, `df` (standard GNU coreutils)
- **CentOS/RHEL**: `top`, `free`, `df` (standard GNU coreutils)
- **Alpine Linux (BusyBox)**: `top`, `free`, `df` (BusyBox variants, different output format)
- **macOS**: `top`, `vm_stat`, `df` (BSD variants, different output format)

**Graceful Failure**:
- If command fails (e.g., `top` not found on Alpine) → show "N/A" for that metric
- If parsing fails (e.g., unexpected output format) → show "N/A" for that metric
- Do NOT crash app or stop monitoring other VMs

---

### Explicitly Excluded from MVP

**What's NOT Included**:
- **Time-series graphs**: No line charts, area charts, or sparklines showing CPU/RAM/Disk over time
- **Historical data storage**: No database, CSV export, or data retention beyond current snapshot
- **Alerting**: No desktop notifications, email alerts, or threshold-based warnings
- **Anomaly detection**: No ML-based anomaly detection or trend analysis
- **Custom metrics**: No user-defined metrics (e.g., network I/O, process count, GPU usage)
- **Per-process monitoring**: No breakdown of CPU/RAM by process (e.g., "claude-code using 45% CPU")
- **Comparative views**: No side-by-side comparison of resource usage across VMs (e.g., "VM A uses 2x more RAM than VM B")

**Rationale**: MVP focuses on **real-time visibility** to prevent resource exhaustion. Advanced monitoring features (graphs, alerts, analytics) require database infrastructure and analytics logic that would delay MVP by 2-3 months.

**Post-MVP Consideration**: Time-series graphs and alerting are high-priority for v1.5 if users request them.

---

## Acceptance Criteria (QA Scenarios)

> **Purpose**: This section defines testable acceptance criteria for the MVP. All criteria must pass before MVP is considered complete.

### AC-1: Workset CRUD Operations

- [ ] User can create new Workset with all required fields (SSH host, port, user, auth method, project path, AI CLI command, grid layout)
- [ ] User can save Workset to disk (JSON file exists in `~/.config/multivm-workspace/worksets/`)
- [ ] User can view list of saved Worksets in sidebar (search/filter works)
- [ ] User can edit existing Workset (modify fields, save changes)
- [ ] User can delete Workset (confirmation dialog appears, file deleted from disk)
- [ ] User can activate Workset (SSH connection established, AI CLI launched, grid layout restored)
- [ ] App restart preserves all saved Worksets (JSON files persist)

---

### AC-2: SSH Connection Methods

- [ ] User can connect to VM using SSH key file path (authentication succeeds)
- [ ] User can connect to VM using password (password stored in OS keystore)
- [ ] User can connect to VM using `~/.ssh/config` alias (app parses config, connects successfully)
- [ ] SSH connection survives 30+ minutes without manual keepalive
- [ ] Connection failure shows clear error message (e.g., "Authentication failed", "Host unreachable")

---

### AC-3: Terminal Emulator Functionality

- [ ] User can type commands, see output in real-time (e.g., `ls -la`, `echo "test"`)
- [ ] User can run interactive TUI apps (vim, htop, nano) with full keyboard navigation
- [ ] User can run AI CLI tools (claude-code, opencode) and see colored output
- [ ] User can copy/paste text (Ctrl+Shift+C/V)
- [ ] User can scroll through 10,000+ lines without UI freeze
- [ ] Terminal displays 256-color and truecolor correctly
- [ ] Terminal resizes when pane is resized

---

### AC-4: Grid Layout Operations

- [ ] User can select preset layout (1x1, 2x1, 2x2)
- [ ] User can create custom NxM layout (e.g., 2x3)
- [ ] User can drag pane dividers to resize panes (<50ms response)
- [ ] User can assign content type to each pane (Terminal, File Browser, Markdown Viewer)
- [ ] User can assign different VM connections to different panes
- [ ] Layout state persists in Workset (saved to JSON, restored on activation)

---

### AC-5: File Browser Navigation

- [ ] User can browse remote file system (tree view with folders and files)
- [ ] User can expand/collapse folders
- [ ] User can see file metadata (size, last modified)
- [ ] User can click `.md` file → opens in Markdown Viewer
- [ ] User can click non-`.md` file → shows "read-only" message
- [ ] File browser updates when remote file system changes (manual refresh or auto-refresh)

---

### AC-6: Markdown Viewer Rendering

- [ ] User can open `.md` file from File Browser → renders in Markdown Viewer
- [ ] Markdown rendering supports: Headers, bold, italic, lists, tables, links, code blocks
- [ ] Code blocks have syntax highlighting
- [ ] Markdown Viewer auto-refreshes when file changes (5-second polling)
- [ ] User can manually refresh Markdown Viewer

---

### AC-7: Resource Monitoring Display

- [ ] User sees CPU usage % for each connected VM (updated every 5 seconds)
- [ ] User sees RAM usage % for each connected VM (updated every 5 seconds)
- [ ] User sees Disk usage % for each connected VM (updated every 5 seconds)
- [ ] Resource values have color coding (green <50%, yellow 50-80%, red >80%)
- [ ] Resource monitoring works on Ubuntu, Debian, CentOS
- [ ] Resource monitoring gracefully fails on unsupported OS (shows "N/A")

---

### AC-8: AI CLI Auto-Launch

- [ ] User can specify AI CLI command in Workset creation form
- [ ] User activates Workset → terminal automatically runs `cd <project_folder> && <ai_cli_command>`
- [ ] User sees AI CLI output in terminal
- [ ] User can interact with AI CLI immediately (no manual command typing)
- [ ] User can edit AI CLI command in Workset settings
- [ ] If AI CLI command fails, terminal shows error message (does not crash app)

---

### AC-9: SSH Auto-Reconnect

- [ ] SSH connection drop detected within 10 seconds
- [ ] App automatically attempts reconnection (max 3 retries, 5-second intervals)
- [ ] User sees "Reconnecting... (1/3)" message during reconnection
- [ ] Reconnection succeeds → terminal prompt reappears
- [ ] Reconnection fails after 3 retries → user sees "Connection lost. Click to reconnect manually."
- [ ] Auto-reconnect success rate ≥90% in network interruption tests

---

### AC-10: Dark/Light Theme

- [ ] User can toggle theme via settings menu
- [ ] Dark theme: Dark backgrounds, light text
- [ ] Light theme: Light backgrounds, dark text
- [ ] Theme applies to all UI components (terminal, file browser, markdown viewer, sidebar)
- [ ] Theme preference persists across app restarts
- [ ] Theme change takes effect immediately (no app restart required)

---

## Terminology Reference

All terms in this document follow definitions in [Glossary](./glossary.md). Key terms:

- **Workset**: Saved profile with SSH details, project path, AI CLI command, grid layout
- **Grid Layout**: Visual arrangement of terminal panes, file browsers, viewers (NxM grid)
- **AI Agent / AI CLI**: Command-line AI tools (Claude Code, OpenCode) that this product **launches** (not orchestrates)
- **Session**: Active connection state between app and remote VM
- **Remote VM**: Virtual machine or server accessed via SSH
- **Desktop App**: Standalone Tauri-based application (not web app, not VS Code extension)
- **Resource Monitoring**: Real-time display of CPU %, RAM %, Disk % (snapshot values only, no historical data)
- **Auto-Reconnect**: Automatic SSH reconnection after connection drop (max 3 retries, 5-second intervals)

---

## Revision History

| Date | Version | Changes | Author |
|------|---------|---------|--------|
| 2026-02-07 | 1.0 | Initial MVP specification with 10 features, 10 exclusions, E2E journey, technical spike priorities, resource monitoring scope, acceptance criteria | AI Implementation Agent |
