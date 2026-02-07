d# Multi-VM AI Agent Workspace Tool

[![License: MIT/Apache 2.0](https://img.shields.io/badge/License-MIT%2FApache%202.0-blue.svg)](./LICENSE-MIT)
[![Status: Planning Complete](https://img.shields.io/badge/Status-Planning%20Complete-brightgreen.svg)](./docs/README.md)

## Vision

A unified desktop application that enables developers to manage multiple remote VMs simultaneously while running AI coding agents (Claude Code, OpenCode) in parallel. Replace 10+ terminal windows with a single, intelligent workspace.

## The Problem

Developers working with AI coding assistants across 2-10 remote VMs face critical friction:

- **Window Management Hell**: 9-15 terminal windows open simultaneously, constant context switching
- **No Visual File Navigation**: Exploring remote codebases requires memorizing directory structures
- **Resource Blindness**: No visibility into CPU/RAM usage until SSH becomes unresponsive
- **Repetitive Setup**: Same SSH commands and AI CLI launches 10+ times per day
- **Session Recovery Nightmare**: Network drops require manual reconnection and restart for each VM

## The Solution

A desktop application that:
- **Unified Workspace**: View 2-10 VM terminals in a single grid layout (1x1, 2x2, 2x3, etc.)
- **Workset Profiles**: Save and restore entire development environments with one click
- **Visual File Browser**: Browse remote file trees without leaving the workspace
- **Resource Monitoring**: Real-time CPU/RAM visibility across all VMs
- **AI Agent Integration**: Auto-launch Claude Code, OpenCode, or custom CLI commands
- **Session Persistence**: Auto-reconnect and resume after network interruptions
- **Markdown Viewer**: Preview documentation directly in the workspace

## MVP Features (10 Core Capabilities)

1. **Workset Profile Management** — Save/load SSH configs, project paths, AI CLI commands, grid layouts
2. **SSH Connection** — Key-based, password, and ~/.ssh/config integration
3. **Multi-VM Terminal Grid** — View 2-10 terminals in customizable layouts
4. **File Browser** — Visual navigation of remote file trees
5. **Markdown Viewer** — Preview README.md and documentation files
6. **Resource Monitoring** — Real-time CPU/RAM usage per VM
7. **AI CLI Auto-Launch** — Auto-execute Claude Code, OpenCode, or custom commands
8. **Session Persistence** — Auto-reconnect and resume after disconnections
9. **Workset Sharing** — Export/import profiles for team collaboration
10. **Keyboard Shortcuts** — Fast navigation between terminals and features

## Target Users

- **Solo Developers**: Managing 2-3 AWS/GCP instances for different projects
- **Startup Teams**: Running 5-7 microservices with parallel AI refactoring tasks
- **AI-First Developers**: Using Claude Code, OpenCode, or similar agents across multiple environments

## Current Status

✅ **Planning Complete**
- Product Requirements Document (PRD) finalized
- MVP specification with 10 core features defined
- Market research and competitive analysis completed
- Architecture design documented
- Glossary and terminology established

## Next Steps

### Phase 1: Technical Spikes (Weeks 1-2)
- [ ] Evaluate Tauri vs Electron for desktop framework
- [ ] Prototype SSH connection pooling with Rust
- [ ] Test multi-terminal rendering performance
- [ ] Validate grid layout responsiveness

### Phase 2: Community Building (Weeks 1-4)
- [ ] Launch GitHub discussions for early adopter feedback
- [ ] Create Discord community for developers
- [ ] Publish development blog with architecture insights
- [ ] Recruit 5-10 beta testers from AI developer community

### Phase 3: User Validation (Weeks 2-4)
- [ ] Conduct user interviews with target personas
- [ ] Validate Workset profile UX with prototypes
- [ ] Test SSH authentication flows with real VMs
- [ ] Gather feedback on grid layout preferences

## Documentation

All planning documents are in the `docs/` folder:

- **[Documentation Guide](./docs/README.md)** — Navigation guide for all planning documents
- **[Glossary](./docs/glossary.md)** — Key terminology and definitions
- **[Market Research](./docs/product/market-research.md)** — Competitive analysis and market opportunity
- **[PRD](./docs/product/prd.md)** — Complete product requirements and user personas
- **[Architecture](./docs/engineering/architecture.md)** — Technical design and system architecture
- **[MVP Spec](./docs/qa/mvp-spec.md)** — Detailed MVP feature specifications

## Contributing

We welcome contributions from developers, designers, and community members! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on:

- Setting up your development environment
- Submitting issues and feature requests
- Creating pull requests
- Code style and conventions
- Community code of conduct

## License

This project is dual-licensed under:

- **MIT License** — See [LICENSE-MIT](./LICENSE-MIT)
- **Apache License 2.0** — See [LICENSE-APACHE](./LICENSE-APACHE)

You may choose either license for your use of this software.

## Community

- **GitHub Issues**: Report bugs and request features
- **GitHub Discussions**: Ask questions and share ideas
- **Discord** (coming soon): Real-time community chat

## Acknowledgments

This project is built for the AI-first developer community. Special thanks to all early adopters and beta testers who help shape the future of multi-VM development workflows.

---

**Last Updated**: February 7, 2026  
**Status**: Planning Phase — Ready for Technical Spikes and Community Building
