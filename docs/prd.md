# Product Requirements Document (PRD) — Multi-VM AI Agent Workspace Tool

## Document Information

| Field | Value |
|-------|-------|
| **Document Type** | Product Requirements Document (PRD) |
| **Version** | 1.0 |
| **Date** | 2026-02-07 |
| **Status** | Draft |
| **Related Documents** | [Glossary](./glossary.md), [Market Research](./market-research.md) |

---

## Executive Summary

This document defines the product requirements for a **Desktop Application** that enables developers to manage multiple remote VMs simultaneously while running AI coding agents (Claude Code, OpenCode) in parallel. The product addresses the critical gap in existing SSH clients and terminal emulators that lack unified workspace management for AI-driven multi-VM workflows.

**Core Value Proposition**: A single desktop application that replaces the need to juggle multiple terminal windows, SSH sessions, and file browsers when working with AI agents across 2-10 remote VMs concurrently.

**Target Market**: Individual developers and small teams using AI coding assistants in distributed development environments.

**License**: Full Open Source (MIT or Apache 2.0)

---

## User Personas

### Persona 1: Alex Chen — Solo Developer with Multi-Environment Projects

**Background**:
- **Role**: Independent full-stack developer
- **Experience**: 5 years in web development
- **Current Setup**: MacBook Pro, 2-3 AWS EC2 instances for different projects
- **AI Tool Usage**: Claude Code for backend refactoring, OpenCode for frontend experiments

**Pain Points**:
1. **Window Management Hell**: Constantly switching between 6+ terminal windows (2 VMs × 3 terminals each) loses context and breaks flow
2. **No Visual File Navigation**: Must use `ls`, `cd`, `cat` repeatedly to explore project structure on remote VMs — wastes 20+ minutes daily
3. **Markdown Documentation Friction**: Cannot quickly preview README.md or API docs without downloading files or using `cat` with poor formatting
4. **Resource Blindness**: No idea which VM is overloaded until SSH becomes unresponsive — has crashed production builds twice
5. **Repetitive Setup**: Types the same SSH commands, `cd` paths, and AI CLI launch commands 10+ times per day

**Goals**:
- Launch all project environments (dev, staging, prod) with one click
- See file trees and markdown docs without leaving the terminal workspace
- Monitor CPU/RAM usage to prevent resource exhaustion
- Reduce context switching time by 50%

**Workflow**:
- Morning: Open "E-commerce Project" Workset → 2x1 grid with dev VM (left) and staging VM (right) → Claude Code auto-launches in both
- Afternoon: Switch to "API Refactor" Workset → 1x1 grid with single VM → OpenCode auto-launches
- Uses file browser to verify generated code structure, markdown viewer to check updated docs

---

### Persona 2: Jordan Kim — Startup Developer with Parallel AI Workflows

**Background**:
- **Role**: Lead developer at 8-person startup
- **Experience**: 8 years, specializes in microservices architecture
- **Current Setup**: Linux workstation, 5-7 GCP VMs running different microservices
- **AI Tool Usage**: Runs Claude Code on 4+ VMs simultaneously for parallel refactoring tasks

**Pain Points**:
1. **Parallel AI Orchestration Chaos**: Needs to monitor 5 AI agents working on different services simultaneously — current setup requires 5 separate VS Code Remote windows, each consuming 500MB+ RAM
2. **Cross-Service Context Loss**: When debugging interactions between services, must manually correlate logs and file changes across 4+ terminal windows
3. **No Unified Resource View**: Cannot see at a glance which VM is CPU-bound and which is idle — leads to inefficient task distribution
4. **Session Recovery Nightmare**: If SSH connection drops, must manually reconnect, navigate to project folder, and restart AI CLI for each VM (15+ minutes of downtime)
5. **Team Onboarding Friction**: New developers spend 2+ hours setting up SSH configs and learning the "correct" terminal layout for multi-service development

**Goals**:
- View 4-6 VM terminals in a single grid layout (2x2 or 2x3)
- Quickly identify resource bottlenecks across all VMs
- Save and share Workset profiles with team members
- Auto-reconnect and resume AI CLI sessions after network interruptions
- Reduce onboarding time for new developers from 2 hours to 15 minutes

**Workflow**:
- Morning standup: Opens "Microservices Dev" Workset → 2x3 grid with 6 VMs (auth, payment, inventory, shipping, notification, gateway)
- Assigns refactoring tasks to Claude Code on 4 VMs in parallel
- Uses resource monitoring to identify that "payment" VM is CPU-saturated → moves task to idle "shipping" VM
- Shares Workset config file with junior developer via Slack → they replicate the exact setup in 5 minutes

---

## Problem Definition

### Current Workflow Friction Points

**1. Terminal Window Proliferation**
- Developers working with 3+ remote VMs end up with 9-15 terminal windows open simultaneously
- macOS/Linux window managers are not optimized for terminal-heavy workflows
- Context switching between windows wastes 15-30 minutes daily

**2. Lack of Visual File Navigation**
- SSH sessions provide only CLI-based file access (`ls`, `cd`, `find`)
- Exploring unfamiliar codebases on remote VMs requires memorizing directory structures
- AI-generated code changes are hard to verify without visual file trees

**3. Markdown Documentation Invisibility**
- AI agents frequently generate or update README.md, CHANGELOG.md, API docs
- Developers must either:
  - Use `cat` (poor formatting, no syntax highlighting)
  - Download files locally (breaks workflow)
  - Open separate browser with GitHub/GitLab (requires push first)

**4. Resource Monitoring Blindness**
- No real-time visibility into remote VM resource usage (CPU, RAM, Disk)
- Developers discover resource exhaustion only when:
  - SSH becomes unresponsive
  - AI CLI crashes with OOM errors
  - Build processes fail mysteriously

**5. Repetitive Session Setup**
- Every morning: SSH into 3-5 VMs, navigate to project folders, launch AI CLI tools
- No way to save and restore workspace configurations
- Team members cannot share standardized development environments

**6. AI CLI Integration Gap**
- Existing SSH clients (Termius, MobaXterm) have no awareness of AI coding tools
- Developers must manually type `claude-code` or `opencode` commands after every SSH connection
- No automation for launching AI agents with specific project contexts

---

## Feature Requirements (MoSCoW Prioritization)

### MUST Have (MVP Core Features)

**MUST-1: Workset Profile Management**
- **Description**: Create, save, load, edit, and delete Workset profiles
- **Details**:
  - Each Workset stores: SSH connection details (host, port, user, auth method), project folder path, AI CLI command, grid layout configuration
  - Worksets saved as JSON files in `~/.config/multivm-workspace/worksets/`
  - UI: Workset library sidebar with search/filter
- **Acceptance Criteria**: User can create a Workset, save it, close the app, reopen, and activate the Workset to restore the exact workspace state

**MUST-2: SSH Connection Management**
- **Description**: Connect to remote VMs via SSH with multiple authentication methods
- **Details**:
  - Support: SSH key-based auth, password auth, `~/.ssh/config` file integration
  - Connection pooling: Maintain persistent SSH sessions for each VM
  - Auto-reconnect: Detect connection drops and attempt reconnection (max 3 retries, 5-second intervals)
- **Acceptance Criteria**: User can connect to a VM using `~/.ssh/config` alias, connection survives network interruptions with auto-reconnect

**MUST-3: Terminal Emulator**
- **Description**: Interactive terminal interface for each remote VM
- **Details**:
  - 256-color and truecolor support
  - Escape sequence handling (cursor movement, text formatting)
  - Copy/paste support (Ctrl+Shift+C/V)
  - Scrollback buffer (10,000 lines)
- **Acceptance Criteria**: User can run `vim`, `htop`, `claude-code` in the terminal with full interactivity and color rendering

**MUST-4: Grid Layout Engine**
- **Description**: Arrange multiple terminal panes, file browsers, and viewers in a grid
- **Details**:
  - Preset layouts: 1x1 (single pane), 2x1 (horizontal split), 2x2 (quad grid)
  - Custom splits: User-defined NxM grids (e.g., 2x3, 3x2)
  - Drag-to-resize pane dividers
  - Each pane can contain: Terminal, File Browser, or Markdown Viewer
- **Acceptance Criteria**: User can create a 2x2 grid, assign 4 different VM terminals to each pane, resize panes, and save the layout in a Workset

**MUST-5: File Browser (Read-Only)**
- **Description**: Tree view of remote file system
- **Details**:
  - Access method: SFTP or SSH exec (`ls -la`, `find`)
  - Features: Expand/collapse folders, file size display, last modified timestamp
  - Actions: Click file → open in Markdown Viewer (if `.md`) or show "read-only" message
  - **Explicitly excluded**: File editing, file upload/download, file deletion
- **Acceptance Criteria**: User can browse `/home/user/project/` on a remote VM, expand `src/` folder, and click `README.md` to open in Markdown Viewer

**MUST-6: Markdown Viewer**
- **Description**: Render Markdown files from remote VMs in formatted view
- **Details**:
  - Fetch file content via SFTP or SSH exec (`cat`)
  - Render with syntax highlighting for code blocks
  - Support: Headers, lists, tables, links, images (if accessible via URL)
  - Auto-refresh: Detect file changes and re-render (polling every 5 seconds)
- **Acceptance Criteria**: User opens `docs/API.md` from remote VM, sees formatted headers and code blocks, file updates when AI agent modifies it

**MUST-7: Resource Monitoring**
- **Description**: Display real-time CPU, RAM, and Disk usage for each remote VM
- **Details**:
  - Collection method: SSH exec commands (`top -bn1`, `free -m`, `df -h`)
  - Update frequency: Every 5 seconds
  - Display format: Percentage values with color coding (green <50%, yellow 50-80%, red >80%)
  - **Explicitly excluded**: Time-series graphs, historical data storage, alerting
- **Acceptance Criteria**: User sees "CPU: 45%, RAM: 62%, Disk: 78%" for each VM, values update every 5 seconds, color changes to yellow/red when thresholds exceeded

**MUST-8: AI CLI Auto-Launch**
- **Description**: Automatically execute AI CLI commands when Workset is activated
- **Details**:
  - Workset stores AI CLI command (e.g., `claude-code`, `opencode --model sonnet`)
  - On Workset activation: SSH connect → `cd` to project folder → execute AI CLI command
  - Terminal shows AI CLI output immediately
- **Acceptance Criteria**: User activates "Backend Refactor" Workset, sees terminal automatically run `cd ~/api-service && claude-code`, AI agent starts without manual input

---

### SHOULD Have (High Priority, Post-MVP)

**SHOULD-1: Theme Customization**
- **Description**: Dark mode and light mode themes
- **Details**: Toggle in settings, applies to entire UI (terminal background, file browser, markdown viewer)
- **Rationale**: Developer preference, accessibility

**SHOULD-2: Keyboard Shortcut Customization**
- **Description**: User-defined keybindings for common actions
- **Details**: Rebind shortcuts for "New Terminal", "Switch Pane", "Activate Workset", "Toggle File Browser"
- **Rationale**: Power users expect customizable shortcuts

**SHOULD-3: SSH Config Import**
- **Description**: Parse `~/.ssh/config` and auto-populate Workset connection details
- **Details**: Detect hosts, ports, identity files, user names from SSH config
- **Rationale**: Reduces manual data entry for users with existing SSH setups

**SHOULD-4: Pane Focus Indicators**
- **Description**: Visual highlight for active pane (border color change)
- **Details**: Active pane has blue border, inactive panes have gray border
- **Rationale**: Improves usability in multi-pane layouts

---

### COULD Have (Nice-to-Have, Future Consideration)

**COULD-1: Session Recording and Playback**
- **Description**: Record terminal sessions and replay them later
- **Details**: Save terminal output to `.cast` files (asciinema format), playback UI
- **Rationale**: Useful for debugging, training, documentation

**COULD-2: Multi-Tab Worksets**
- **Description**: Multiple Workset tabs within a single window
- **Details**: Tab bar at top, each tab loads a different Workset
- **Rationale**: Reduces window clutter for users managing 5+ Worksets

**COULD-3: Custom Resource Monitoring Commands**
- **Description**: User-defined SSH commands for resource collection
- **Details**: Override default `top`, `free`, `df` commands for non-standard VM environments
- **Rationale**: Supports Alpine Linux, macOS, BSD systems with different command syntax

**COULD-4: Workset Templates**
- **Description**: Pre-configured Workset templates for common scenarios
- **Details**: Templates for "Single VM Dev", "Microservices (2x3)", "Frontend + Backend (2x1)"
- **Rationale**: Speeds up onboarding for new users

---

### WON'T Have (Explicitly Excluded from MVP)

**WON'T-1: Team Collaboration Features**
- **Excluded**: Multi-user session sharing, real-time collaboration, shared Worksets with live sync
- **Rationale**: MVP targets single-user workflows; team features require complex infrastructure (auth, sync, conflict resolution)

**WON'T-2: Cloud VM Provider API Integration**
- **Excluded**: AWS EC2, GCP Compute Engine, Azure VM provisioning/management via cloud APIs
- **Rationale**: Product connects to existing VMs via SSH; VM lifecycle management is out of scope

**WON'T-3: Plugin/Extension System**
- **Excluded**: Plugin architecture, third-party extension APIs, marketplace
- **Rationale**: Adds architectural complexity; defer until core product is stable

**WON'T-4: File Editing**
- **Excluded**: Text editor, code editor, file upload/download, file deletion
- **Rationale**: Users already have preferred editors (vim, VS Code Remote); read-only file browsing is sufficient for MVP

**WON'T-5: Git Integration**
- **Excluded**: Git commit, push, pull, diff visualization, branch management
- **Rationale**: Git operations are well-served by existing tools (terminal git, VS Code, GitKraken); not a differentiator

**WON'T-6: AI Agent Orchestration**
- **Excluded**: Controlling AI agent behavior, multi-agent coordination, task distribution across agents
- **Rationale**: This product **launches** AI CLI tools; it does NOT orchestrate or control their internal behavior (see Glossary)

**WON'T-7: Advanced Window Management**
- **Excluded**: Floating windows, arbitrary nesting, tiling algorithms, window snapping
- **Rationale**: Grid layout with preset + custom splits is sufficient; full window manager features are out of scope

**WON'T-8: Time-Series Resource Monitoring**
- **Excluded**: Historical data storage, time-series graphs, alerting, anomaly detection
- **Rationale**: MVP provides snapshot values only; advanced monitoring requires database and analytics infrastructure

**WON'T-9: Session Persistence Across Reboots**
- **Excluded**: Save terminal scrollback, command history, running processes across app restarts
- **Rationale**: SSH sessions are ephemeral; persistence requires complex state management

**WON'T-10: Custom Themes Beyond Dark/Light**
- **Excluded**: User-created color schemes, theme marketplace, syntax highlighting customization
- **Rationale**: Dark/light modes cover 95% of use cases; custom themes are low-priority polish

---

## Must NOT Have (Scope Guardrails)

> **Purpose**: This section explicitly lists features, decisions, and content that MUST NOT appear in this PRD or related planning documents. Violating these guardrails indicates scope creep or misunderstanding of project goals.

**1. Implementation Timelines or Sprint Plans**
- **Reason**: No development team has been formed; timeline estimation is premature
- **Violation Example**: "Sprint 1: SSH connection (2 weeks), Sprint 2: Grid layout (3 weeks)"

**2. Specific Rust Crate or JavaScript Library Recommendations**
- **Reason**: Technology choices belong in the implementation phase, not product requirements
- **Violation Example**: "Use `ssh2` crate for SSH, `tokio` for async runtime, `xterm.js` for terminal"

**3. Wireframes or Visual Design Mockups**
- **Reason**: UI/UX design is a separate phase; PRD focuses on functional requirements
- **Violation Example**: Including Figma mockups, color palettes, typography specifications

**4. TAM/SAM/SOM Financial Modeling**
- **Reason**: This is an open-source project, not seeking venture funding
- **Violation Example**: "Total Addressable Market: $2.3B, Serviceable Addressable Market: $450M"

**5. Confusion Between "AI Agent Orchestration" and "AI CLI Auto-Launch"**
- **Reason**: This product **launches** AI CLI tools on remote VMs; it does NOT orchestrate, coordinate, or control AI agent behavior
- **Violation Example**: "The product will distribute tasks across multiple AI agents and merge their outputs"

**6. Plugin/Extension System Architecture**
- **Reason**: Deferred to post-MVP; including plugin specs now adds unnecessary complexity
- **Violation Example**: "Plugin API will expose hooks for `onSSHConnect`, `onFileOpen`, `onResourceUpdate`"

**7. Multi-User or Team Features**
- **Reason**: MVP is single-user only; team features require authentication, authorization, and sync infrastructure
- **Violation Example**: "Users can share Worksets with team members and see live cursor positions"

**8. Cloud VM Provider API Integration**
- **Reason**: Product connects to existing VMs via SSH; VM provisioning is out of scope
- **Violation Example**: "Integrate AWS SDK to launch EC2 instances from the app"

**9. File Editing Capabilities**
- **Reason**: MVP is read-only for file browsing; editing is explicitly excluded
- **Violation Example**: "File browser will support inline editing with syntax highlighting"

**10. Complex Window Manager Features**
- **Reason**: Grid layout supports preset + custom splits only; not a full tiling window manager
- **Violation Example**: "Support arbitrary window nesting, floating windows, and i3-style tiling algorithms"

**11. Git Client Functionality**
- **Reason**: Git operations are out of scope; users have existing Git tools
- **Violation Example**: "Add commit, push, pull buttons to the file browser"

**12. Session Recording/Playback in MVP**
- **Reason**: This is a "Could Have" feature, not MVP
- **Violation Example**: "MVP must include asciinema-compatible session recording"

**13. Historical Resource Monitoring Data**
- **Reason**: MVP provides snapshot values only; time-series data requires database infrastructure
- **Violation Example**: "Store CPU/RAM/Disk metrics in SQLite and display 24-hour graphs"

**14. Revenue Models or Monetization Strategies**
- **Reason**: Open-source project with community-driven development; no commercial plans
- **Violation Example**: "Freemium model: Free for 3 VMs, $9/month for unlimited VMs"

**15. Competitive Feature Parity Checklists**
- **Reason**: This product targets a unique use case (AI + multi-VM); feature-for-feature comparison with Termius/MobaXterm is irrelevant
- **Violation Example**: "Must match Termius's SFTP upload speed and MobaXterm's X11 forwarding"

---

## Non-Functional Requirements

### Performance

**NFR-1: SSH Connection Latency**
- **Requirement**: SSH connection establishment ≤ 2 seconds (local network), ≤ 5 seconds (internet)
- **Measurement**: Time from "Connect" button click to terminal prompt appearance
- **Rationale**: Users expect near-instant connections for local VMs

**NFR-2: Terminal Rendering Performance**
- **Requirement**: Terminal can render 10,000 lines of output without UI freeze
- **Measurement**: Scroll through `cat large-log.txt` (10,000 lines) with <100ms lag
- **Rationale**: AI agents generate verbose output; terminal must remain responsive

**NFR-3: Grid Layout Responsiveness**
- **Requirement**: Pane resize operations complete within 50ms
- **Measurement**: Drag pane divider, measure time until UI updates
- **Rationale**: Smooth resizing is critical for multi-pane workflows

**NFR-4: Resource Monitoring Update Frequency**
- **Requirement**: CPU/RAM/Disk values update every 5 seconds (±1 second)
- **Measurement**: Timestamp difference between consecutive updates
- **Rationale**: 5-second intervals balance freshness and SSH overhead

---

### Compatibility

**NFR-5: Operating System Support**
- **Requirement**: Desktop app runs on macOS 11+, Ubuntu 20.04+, Windows 10+
- **Priority**: macOS (primary), Linux (secondary), Windows (tertiary)
- **Rationale**: Developer demographics skew toward macOS and Linux

**NFR-6: SSH Protocol Support**
- **Requirement**: Support SSH protocol version 2.0 (OpenSSH 7.0+)
- **Rationale**: SSH 2.0 is the industry standard; SSH 1.0 is deprecated and insecure

**NFR-7: Remote VM OS Support**
- **Requirement**: Compatible with Ubuntu, Debian, CentOS, Alpine Linux, macOS (SSH server enabled)
- **Rationale**: Covers 95% of developer VM environments

---

### Reliability

**NFR-8: Auto-Reconnect Success Rate**
- **Requirement**: 90% of dropped SSH connections successfully reconnect within 15 seconds
- **Measurement**: Simulate network interruptions, measure reconnection success rate
- **Rationale**: Network instability is common; auto-reconnect prevents workflow disruption

**NFR-9: Crash Recovery**
- **Requirement**: App crash does not corrupt Workset configuration files
- **Measurement**: Force-quit app during Workset save, verify file integrity on restart
- **Rationale**: Users must trust that Worksets are safe from data loss

---

### Usability

**NFR-10: Workset Activation Time**
- **Requirement**: Activating a Workset (4 VMs, 2x2 grid) completes within 10 seconds
- **Measurement**: Time from Workset click to all 4 terminals showing prompts
- **Rationale**: Fast activation is critical for "one-click workspace" value proposition

**NFR-11: Onboarding Time for New Users**
- **Requirement**: New user can create and activate their first Workset within 5 minutes (no tutorial)
- **Measurement**: User testing with developers unfamiliar with the product
- **Rationale**: Low learning curve is essential for adoption

---

### Security

**NFR-12: SSH Key Storage**
- **Requirement**: SSH private keys are never stored in Workset files; only file paths are saved
- **Rationale**: Prevents accidental exposure of credentials in shared Workset configs

**NFR-13: Password Handling**
- **Requirement**: SSH passwords are stored in OS-native secure storage (macOS Keychain, Linux Secret Service, Windows Credential Manager)
- **Rationale**: Passwords must not be stored in plaintext

---

## Open Source Strategy

### License Selection

**Decision**: MIT License (preferred) or Apache 2.0 License
- **Rationale**:
  - **MIT**: Maximum permissiveness, simplest for contributors, widely adopted in developer tools (VS Code, Atom, Hyper)
  - **Apache 2.0**: Explicit patent grant, better for corporate contributors, used by Rust ecosystem (Tauri, Tokio)
- **Final Decision**: Deferred to community vote after initial prototype

---

### Community Building Approach

**Phase 1: Early Adopters (Months 1-3)**
- **Goal**: Recruit 10-20 power users for feedback
- **Channels**: Reddit (`r/rust`, `r/commandline`), Hacker News Show HN, Twitter/X
- **Incentive**: Early access to features, direct influence on roadmap

**Phase 2: Contributor Onboarding (Months 4-6)**
- **Goal**: Attract 5-10 code contributors
- **Strategy**:
  - Label GitHub issues as "good first issue", "help wanted"
  - Write detailed CONTRIBUTING.md with architecture overview
  - Host monthly community calls (Discord/Zoom)
- **Recognition**: Contributors page, release notes shoutouts

**Phase 3: Ecosystem Growth (Months 7-12)**
- **Goal**: 100+ GitHub stars, 5+ community-contributed features
- **Strategy**:
  - Publish blog posts on Rust/Tauri development challenges
  - Submit talks to RustConf, FOSDEM
  - Create video tutorials (YouTube)

---

### Contribution Guidelines Philosophy

**Principles**:
1. **Welcoming**: No gatekeeping, no "RTFM" responses
2. **Transparent**: All architectural decisions documented in ADRs (Architecture Decision Records)
3. **Responsive**: Maintainers respond to issues/PRs within 48 hours
4. **Quality-Focused**: CI/CD enforces tests, linting, formatting (no manual review for style)

**Governance**:
- **Benevolent Dictator**: Original author has final say on roadmap and breaking changes
- **Core Team**: 3-5 trusted contributors with merge rights (earned after 10+ merged PRs)
- **Community Voting**: Major feature additions (e.g., plugin system) require RFC (Request for Comments) process

---

## Success Metrics (Post-Launch)

> **Note**: These metrics are for post-launch evaluation, NOT part of MVP scope.

**Adoption Metrics**:
- GitHub stars: 500+ in first 6 months
- Weekly active users: 100+ (telemetry opt-in)
- Worksets created: 1,000+ (anonymized telemetry)

**Engagement Metrics**:
- Average session duration: 30+ minutes
- Worksets per user: 3+ (indicates multi-project usage)
- Grid layouts used: 60% use 2x2 or larger (validates multi-VM use case)

**Community Health**:
- GitHub issues closed: 80%+ within 30 days
- Contributor retention: 50%+ of first-time contributors make 2+ PRs
- Community sentiment: 4.5+ stars on GitHub, positive HN/Reddit comments

---

## Appendix: Terminology Reference

All terms in this document follow definitions in [Glossary](./glossary.md). Key terms:

- **Workset**: Saved profile with SSH details, project path, AI CLI command, grid layout
- **Grid Layout**: Visual arrangement of terminal panes, file browsers, viewers
- **AI Agent / AI CLI**: Command-line AI tools (Claude Code, OpenCode) that this product **launches** (not orchestrates)
- **Session**: Active connection state between app and remote VM
- **Remote VM**: Virtual machine or server accessed via SSH
- **Desktop App**: Standalone Tauri-based application (not web app, not VS Code extension)

---

## Revision History

| Date | Version | Changes | Author |
|------|---------|---------|--------|
| 2026-02-07 | 1.0 | Initial PRD with 2 personas, 8 MUST requirements, 10 Must NOT items, non-functional requirements, open-source strategy | AI Planning Agent |
