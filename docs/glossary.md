# Glossary — 용어 정의

> **Purpose**: This document defines core terminology for the Multi-VM AI Agent Workspace Tool project. All project documentation must use these standardized terms consistently.
>
> **목적**: Multi-VM AI Agent Workspace Tool 프로젝트의 핵심 용어를 정의합니다. 모든 프로젝트 문서는 이 표준화된 용어를 일관되게 사용해야 합니다.

---

## Core Concepts — 핵심 개념

- **Workset (워크셋)** — A saved profile containing SSH connection details, project folder path, AI CLI command, and grid layout configuration. When activated, a Workset automatically connects to the remote VM, opens the specified project folder, launches the AI agent CLI, and arranges the workspace according to the saved layout.

- **Grid Layout (그리드 레이아웃)** — The visual arrangement of terminal panes, file browsers, and viewers within the desktop application window. Supports preset configurations (1x1, 2x1, 2x2) and custom splits (e.g., 2|3, NxM) to display multiple remote VMs simultaneously.

- **AI Agent / AI CLI (AI 에이전트 / AI CLI)** — Command-line interface tools for AI-powered coding assistance, such as Claude Code or OpenCode. This product **launches** these tools automatically via SSH; it does NOT orchestrate or control their behavior.

- **Session (세션)** — An active connection state between the desktop application and a remote VM, including the SSH connection, running terminal processes, and associated UI components (file browser, markdown viewer).

- **Remote VM (원격 VM)** — A virtual machine or remote server accessed via SSH. The target environment where AI agents run and development work occurs.

---

## Features — 기능

- **SSH Connection (SSH 접속)** — Secure Shell protocol-based connection to remote VMs. Supports key-based authentication, password authentication, and `~/.ssh/config` file integration.

- **Terminal Emulator (터미널 에뮬레이터)** — A component that provides interactive command-line interface within the desktop application, emulating terminal behavior (256-color/truecolor support, escape sequences, etc.).

- **File Browser (파일 브라우저)** — A read-only tree view of the remote file system, accessed via SFTP or SSH exec commands. In MVP, this component does NOT support file editing.

- **Markdown Viewer (마크다운 뷰어)** — A component that renders Markdown files from remote VMs in a formatted, readable view within the application.

- **Resource Monitoring (자원 모니터링)** — Real-time or periodic display of remote VM system resources (CPU usage %, RAM usage %, Disk usage %). MVP scope: snapshot values only, updated every 5 seconds via SSH exec commands (`top`, `free`, `df`).

---

## Technical Terms — 기술 용어

- **Desktop App (데스크톱 앱)** — A standalone application running on the user's local machine (macOS, Linux, or Windows), built with Tauri framework. NOT a web application, browser extension, or VS Code extension.

- **Tauri** — A Rust-based framework for building lightweight desktop applications with web frontend technologies. Chosen for cross-platform support and performance.

- **xterm.js** — A JavaScript terminal emulator library used for rendering terminal interfaces in web-based frontends. Candidate technology for the Terminal Emulator component.

- **SFTP (SSH File Transfer Protocol)** — A secure file transfer protocol running over SSH, used for accessing remote file systems in the File Browser component.

- **C4 Model** — A hierarchical approach to software architecture diagrams (Context, Container, Component, Code levels). Used in the Architecture Blueprint document.

---

## Scope Boundaries — 범위 경계

### What This Product IS — 이 제품이 무엇인가

This product is a **desktop application** that:
- Manages multiple SSH connections to remote VMs simultaneously
- Provides a unified workspace with grid-based layout for viewing multiple terminals, file browsers, and markdown viewers
- Automates the launch of AI CLI tools (Claude Code, OpenCode) on remote VMs via saved Workset profiles
- Monitors remote VM resource usage (CPU, RAM, Disk) in real-time
- Targets individual developers working with 2-10 remote VMs concurrently

이 제품은 다음을 수행하는 **데스크톱 애플리케이션**입니다:
- 여러 원격 VM에 대한 SSH 접속을 동시에 관리
- 여러 터미널, 파일 브라우저, 마크다운 뷰어를 그리드 기반 레이아웃으로 통합 제공
- Workset 프로필을 통해 원격 VM에서 AI CLI 도구(Claude Code, OpenCode) 자동 실행
- 원격 VM의 자원 사용량(CPU, RAM, Disk)을 실시간 모니터링
- 2-10개의 원격 VM을 동시에 사용하는 개인 개발자를 대상으로 함

---

### What This Product is NOT — 이 제품이 아닌 것

This product is **explicitly excluded** from the following:

**NOT an AI Agent Orchestrator**
- This product **launches** AI CLI tools on remote VMs; it does NOT orchestrate, coordinate, or control the behavior of AI agents themselves.
- 이 제품은 원격 VM에서 AI CLI 도구를 **실행**하지만, AI 에이전트의 동작을 오케스트레이션하거나 제어하지 않습니다.

**NOT a File Editor**
- MVP provides READ-ONLY file browsing. File editing is explicitly excluded.
- MVP는 READ-ONLY 파일 브라우징만 제공합니다. 파일 편집 기능은 명시적으로 제외됩니다.

**NOT a Team Collaboration Tool**
- MVP targets single-user workflows. Multi-user features, session sharing, and team collaboration are excluded.
- MVP는 단일 사용자 워크플로우를 대상으로 합니다. 멀티유저 기능, 세션 공유, 팀 협업 기능은 제외됩니다.

**NOT a Cloud VM Provider Integration**
- This product connects to existing VMs via SSH. It does NOT integrate with cloud provider APIs (AWS, GCP, Azure) to provision or manage VMs.
- 이 제품은 SSH를 통해 기존 VM에 접속합니다. 클라우드 프로바이더 API(AWS, GCP, Azure)와 통합하여 VM을 프로비저닝하거나 관리하지 않습니다.

**NOT a Plugin/Extension System**
- MVP does not include plugin architecture, extension APIs, or third-party integration mechanisms. This is a post-MVP consideration.
- MVP는 플러그인 아키텍처, 확장 API, 서드파티 통합 메커니즘을 포함하지 않습니다. 이는 MVP 이후 고려 사항입니다.

**NOT a Complex Window Manager**
- Grid layout supports preset configurations (1x1, 2x1, 2x2) and custom splits. It is NOT a full-featured tiling window manager with arbitrary nesting, floating windows, or advanced layout algorithms.
- 그리드 레이아웃은 프리셋 구성(1x1, 2x1, 2x2)과 커스텀 분할을 지원합니다. 임의 중첩, 플로팅 윈도우, 고급 레이아웃 알고리즘을 가진 완전한 타일링 윈도우 매니저가 아닙니다.

**NOT a Git Client**
- Git integration (commit, push, pull, diff visualization) is excluded from MVP scope.
- Git 통합(커밋, 푸시, 풀, diff 시각화)은 MVP 범위에서 제외됩니다.

**NOT a Session Recording/Playback Tool**
- Session recording and playback features are excluded from MVP. This is a "Could Have" feature for future consideration.
- 세션 녹화 및 재생 기능은 MVP에서 제외됩니다. 이는 향후 고려할 "Could Have" 기능입니다.

**NOT a Resource Monitoring Dashboard with Historical Data**
- MVP provides snapshot resource values (CPU %, RAM %, Disk %) updated every 5 seconds. Time-series graphs, historical data storage, and alerting are excluded.
- MVP는 5초마다 업데이트되는 스냅샷 자원 값(CPU %, RAM %, Disk %)을 제공합니다. 시계열 그래프, 히스토리 데이터 저장, 알럿 기능은 제외됩니다.

**NOT a Financial/Business Plan**
- This glossary and associated planning documents do NOT include TAM/SAM/SOM analysis, revenue projections, or go-to-market strategies. The project is open-source (MIT/Apache license) with community-driven development.
- 이 용어집과 관련 기획 문서는 TAM/SAM/SOM 분석, 매출 예측, 시장 진출 전략을 포함하지 않습니다. 이 프로젝트는 오픈소스(MIT/Apache 라이선스)이며 커뮤니티 주도 개발입니다.

**NOT an Implementation Timeline**
- This glossary defines WHAT the product is, not WHEN it will be built. No sprint plans, milestones, or delivery dates are included.
- 이 용어집은 제품이 무엇인지 정의하며, 언제 구축될지는 정의하지 않습니다. 스프린트 계획, 마일스톤, 배포 일정은 포함되지 않습니다.

---

## Document Relationships — 문서 관계

This glossary serves as the **terminology foundation** for all other planning documents:

- **Market Research Report** (`market-research.md`) — Uses these terms when analyzing competitors and market gaps
- **Product Requirements Document** (`prd.md`) — Uses these terms when defining features and requirements
- **Architecture Blueprint** (`architecture.md`) — Uses these terms when naming components and systems
- **MVP Specification** (`mvp-spec.md`) — Uses these terms when scoping features and exclusions

이 용어집은 모든 기획 문서의 **용어 기반**으로 사용됩니다:

- **시장 조사 보고서** (`market-research.md`) — 경쟁사 분석 및 시장 공백 분석 시 이 용어 사용
- **제품 요구사항 정의서** (`prd.md`) — 기능 및 요구사항 정의 시 이 용어 사용
- **아키텍처 블루프린트** (`architecture.md`) — 컴포넌트 및 시스템 명명 시 이 용어 사용
- **MVP 사양서** (`mvp-spec.md`) — 기능 범위 및 제외 항목 정의 시 이 용어 사용

---

## Revision History — 개정 이력

| Date | Version | Changes |
|------|---------|---------|
| 2026-02-07 | 1.0 | Initial glossary created with 15 core terms and scope boundaries |
