# AGENTS.md — Multi-VM AI Agent Workspace Tool

> Last Updated: 2026-02-07
> Status: MVP Feature 1–4 구현 완료 (Workset CRUD + SSH + Terminal + Grid)

---

## 1. Project Overview

**Multi-VM AI Agent Workspace Tool**은 개발자가 2-10개의 원격 VM에서 AI 코딩 에이전트(Claude Code, OpenCode)를 동시에 운용할 수 있는 **Tauri 기반 크로스플랫폼 데스크톱 앱**이다.

**핵심 가치**: 10개 이상의 터미널 창을 하나의 통합 워크스페이스로 대체. Workset 프로필 하나로 SSH 접속 → 프로젝트 폴더 이동 → AI CLI 자동 실행 → Grid Layout 복원을 한 번에 수행.

**현재 상태**: MVP Feature 1–4 구현 완료. Workset CRUD + SSH 연결 + 터미널 에뮬레이터 + Grid Layout이 E2E로 동작. Feature 5–10 구현 예정.

**라이선스**: MIT / Apache 2.0 (듀얼 라이선스)

---

## 2. Tech Stack (계획)

| Layer | Technology | Role |
|-------|------------|------|
| Desktop Framework | **Tauri v2** | Rust Core + Web Frontend + IPC Bridge (ADR-001) |
| Backend | **Rust** | SSH 연결 관리, 프로세스 관리, 리소스 폴링, 파일 접근, Workset 저장 |
| Frontend | **TypeScript/JavaScript** | Grid Layout, UI 컴포넌트 |
| Terminal Rendering | **xterm.js** | WebGL 렌더러 기반 터미널 에뮬레이션 (ADR-002) |
| SSH Protocol | **Rust SSH Library** | SSH v2.0, 채널 멀티플렉싱, 자동 재접속 (ADR-003) |
| Data Persistence | **JSON Files** | Workset 프로필 (`~/.config/multivm-workspace/worksets/`) |
| Credential Storage | **OS Keystore** | macOS Keychain, Linux Secret Service, Windows Credential Manager |

---

## 3. Architecture Summary

### Trust Boundary Model (Tauri)

```
┌──────────────────────────────────────────────────────────────┐
│                    Tauri Desktop App                          │
│                                                              │
│  ┌─────────────────────────┐    ┌──────────────────────────┐ │
│  │   Rust Core (Trusted)   │    │ Web Frontend (Sandboxed) │ │
│  │                         │    │                          │ │
│  │  • SSH Connection Mgr   │◄──►│  • Grid Layout Engine    │ │
│  │  • Process Manager      │ IPC│  • Terminal UI (xterm.js)│ │
│  │  • Resource Poller      │    │  • File Browser UI       │ │
│  │  • Workset Store        │    │  • Markdown Viewer UI    │ │
│  │  • File Access Layer    │    │  • Resource Monitor UI   │ │
│  │                         │    │  • Workset Manager UI    │ │
│  └────────────┬────────────┘    └──────────────────────────┘ │
│               │                                              │
└───────────────┼──────────────────────────────────────────────┘
                │ SSH v2.0
    ┌───────────┼───────────────┐
    ▼           ▼               ▼
 Remote VM 1  Remote VM 2  ... Remote VM N (max 10)
```

### 9 Core Components

| # | Component | Layer | Responsibility |
|---|-----------|-------|----------------|
| 1 | SSH Connection Manager | Rust Core | 다중 SSH 세션 수명주기, 연결 풀링, 자동 재접속 |
| 2 | Process Manager | Rust Core | AI CLI 자동 실행, PTY 관리, 원격 프로세스 수명주기 |
| 3 | Resource Poller | Rust Core | SSH exec로 CPU/RAM/Disk 주기적 수집 (5초 간격) |
| 4 | Workset Store | Rust Core | Workset 프로필 CRUD, JSON 영속화 |
| 5 | File Access Layer | Rust Core | SFTP/SSH exec 기반 원격 파일 시스템 접근 |
| 6 | IPC Bridge | Tauri | Frontend↔Backend 비동기 메시지 패싱 (Commands + Events) |
| 7 | Grid Layout Engine | Frontend | NxM 패인 분할, 리사이즈, 콘텐츠 할당 |
| 8 | Terminal Emulator UI | Frontend | xterm.js WebGL 렌더러, 256색/truecolor |
| 9 | Workset Manager UI | Frontend | Workset 라이브러리 사이드바, CRUD 폼 |

---

## 4. Project Structure

```
multivm-workspace-tool/
├── CLAUDE.md                    # AI 에이전트 설정 (프로젝트 규칙)
├── AGENTS.md                    # 프로젝트 지식 베이스 (본 문서)
├── project-structure.md         # 프로젝트 구조 정의서
├── README.md                    # 프로젝트 소개 및 개요
├── CONTRIBUTING.md              # 기여 가이드
├── LICENSE-MIT                  # MIT 라이선스
├── LICENSE-APACHE               # Apache 2.0 라이선스
├── .gitignore                   # Git 무시 규칙
│
├── package.json                 # Node.js 의존성 (@xterm/xterm, @xterm/addon-webgl, @xterm/addon-fit)
├── tsconfig.json                # TypeScript 설정
├── vite.config.ts               # Vite 빌드 설정
├── index.html                   # Tauri WebView 진입점
│
├── src/                         # Web Frontend (TypeScript, vanilla)
│   ├── main.ts                  # 앱 진입점 — Workset CRUD UI + Workspace activation + E2E IPC wiring (~770줄)
│   ├── styles.css               # 다크 테마 전체 스타일 (~670줄)
│   ├── grid.ts                  # Grid Layout Engine — 5 presets, CSS Grid, layout toolbar (96줄)
│   ├── terminal.ts              # Terminal Emulator — xterm.js WebGL/Canvas, FitAddon (79줄)
│   ├── workspace.ts             # Grid-Terminal integration — OutputBuffer, ResizeObserver (206줄)
│   └── vite-env.d.ts            # Vite 타입 선언
│
├── src-tauri/                   # Rust Core (Tauri backend)
│   ├── Cargo.toml               # Rust 의존성 (ssh2, tokio, uuid, dirs, serde_json, chrono)
│   ├── tauri.conf.json          # Tauri 앱 설정
│   ├── capabilities/            # Tauri v2 capabilities 설정
│   └── src/
│       ├── main.rs              # Tauri 앱 진입점
│       ├── lib.rs               # 모듈 선언 + 9개 IPC Commands (179줄)
│       ├── workset/             # Workset Store
│       │   └── mod.rs           #   데이터 모델 + JSON CRUD 영속화 (420줄)
│       ├── ssh/                 # SSH Connection Manager
│       │   ├── mod.rs           #   SshConnectionManager — connect_all, disconnect_all (127줄)
│       │   └── session.rs       #   SSH session worker thread — PTY, keepalive, events (328줄)
│       └── bin/
│           └── spike_2_ssh_harness.rs  # SPIKE-2 테스트 하네스
│
└── docs/                        # 기획 문서 모음
    ├── README.md                # 문서 네비게이션 가이드
    ├── glossary.md              # 용어 정의 (23개 핵심 용어)
    ├── product/
    │   ├── market-research.md   # 경쟁 분석 (8개 1차 + 4개 인접 경쟁사)
    │   └── prd.md               # 제품 요구사항 (2 페르소나, 8 MUST, MoSCoW)
    ├── engineering/
    │   └── architecture.md      # 아키텍처 블루프린트 (C4 다이어그램, 9 컴포넌트, 3 ADR)
    └── qa/
        └── mvp-spec.md          # MVP 사양 (10 기능, 10 제외, 138 체크박스)
```

---

## 5. Key Decisions (ADR Summary)

| ADR | Decision | Status | Rationale |
|-----|----------|--------|-----------|
| ADR-001 | **Tauri** (not Electron) | Accepted | 번들 <10MB, 메모리 30-50MB, Rust 백엔드 보안/성능 |
| ADR-002 | **xterm.js** for terminal | Proposed (SPIKE-1 후 확정) | 업계 표준 (VS Code, Wave, Tabby), WebGL 렌더러, 19.8k★ |
| ADR-003 | **SSH in Rust Core** | Accepted | 보안 (키/비밀번호 Trusted Zone), 성능, 메모리 안전성 |

---

## 6. MVP Features (10개)

| # | Feature | PRD Mapping | Architecture Component | Status |
|---|---------|-------------|----------------------|--------|
| 1 | Workset Profile Management (CRUD) | MUST-1 | Workset Manager | ✅ 완료 |
| 2 | SSH Connection (Key/Password) | MUST-2 | SSH Connection Manager | ✅ 완료 |
| 3 | Terminal Emulator (xterm.js, 256-color) | MUST-3 | Terminal Emulator | ✅ 완료 |
| 4 | Grid Layout (1x1, 2x1, 2x2, 2x3, 3x2) | MUST-4 | Grid Layout Engine | ✅ 완료 |
| 5 | File Browser (Read-Only) | MUST-5 | File Browser | ⬜ 미구현 |
| 6 | Markdown Viewer | MUST-6 | Markdown Renderer | ⬜ 미구현 |
| 7 | Resource Monitoring (CPU/RAM/Disk) | MUST-7 | Resource Poller | ⬜ 미구현 |
| 8 | AI CLI Auto-Launch | MUST-8 | Process Manager | ⬜ 미구현 |
| 9 | SSH Auto-Reconnect | MUST-2 enhanced | SSH Connection Manager | ⬜ 미구현 |
| 10 | Dark/Light Theme | SHOULD-1 promoted | Frontend | ⬜ 미구현 |

---

## 7. Technical Risks

| Risk | Severity | Description | Mitigation |
|------|----------|-------------|------------|
| RISK-1 | CRITICAL | Tauri WebView + xterm.js latency | SPIKE-1: 프로토타입 성능 검증 |
| RISK-2 | HIGH | 다중 SSH 세션 안정성/재접속 | SPIKE-2: 10개 세션 30분 스트레스 테스트 |
| RISK-3 | MEDIUM | 이기종 VM 리소스 모니터링 호환성 | SPIKE-3: Ubuntu/Alpine/macOS 호환 테스트 |
| RISK-4 | HIGH | IPC 직렬화로 다중 터미널 성능 저하 | 바이너리 IPC, 출력 스로틀링, 백프레셔 |

---

## 8. Technical Spikes (MVP 전 검증)

| Spike | Priority | Objective | Success Criteria |
|-------|----------|-----------|------------------|
| SPIKE-1 | CRITICAL | Tauri + xterm.js latency 검증 | 키 입력 <50ms, 10K줄 <100ms |
| SPIKE-2 | HIGH | SSH 연결 풀링 스트레스 테스트 | 10세션 30분 유지, 재접속 ≥90% |
| SPIKE-3 | MEDIUM | 이기종 VM 리소스 수집 호환성 | Ubuntu/Alpine/macOS 정상 파싱 |

---

## 9. Competitive Landscape

**핵심 차별점**: "AI CLI Auto-Launch"를 제공하는 경쟁 제품은 **0개**.

| Competitor | SSH Manager | Grid Layout | File Browser | AI Integration | AI CLI Auto-Launch |
|------------|:-----------:|:-----------:|:------------:|:--------------:|:------------------:|
| Termius | ✅ | ❌ | ⚠️ | ⚠️ | ❌ |
| MobaXterm | ✅ | ❌ | ✅ | ❌ | ❌ |
| Warp | ⚠️ | ❌ | ❌ | ✅ | ❌ |
| Tabby | ✅ | ⚠️ | ⚠️ | ❌ | ❌ |
| Zellij | ❌ | ✅ | ❌ | ❌ | ❌ |
| Wave Terminal | ✅ | ⚠️ | ✅ | ✅ | ❌ |
| **This Product** | ✅ | ✅ | ✅ | ✅ | ✅ |

---

## 10. Document Map

### 필수 참조 문서

| Document | Path | Purpose |
|----------|------|---------|
| Project Structure | `./project-structure.md` | 프로젝트 구조, 규칙, 컨벤션 정의 |
| Glossary | `./docs/glossary.md` | 23개 핵심 용어 정의 |
| PRD | `./docs/product/prd.md` | 제품 요구사항, 페르소나, MoSCoW |
| Architecture | `./docs/engineering/architecture.md` | C4 다이어그램, 컴포넌트, ADR, 리스크 |
| MVP Spec | `./docs/qa/mvp-spec.md` | 10 기능, 10 제외, E2E 시나리오, AC |
| Market Research | `./docs/product/market-research.md` | 8 경쟁사, 4 시장 공백, Pain Points |
| Contributing | `./CONTRIBUTING.md` | 기여 가이드, 코드 스타일, PR 프로세스 |

### 문서 간 참조 흐름

```
Glossary (용어 기반)
    ↓
Market Research (시장 분석) → PRD (요구사항 정의)
                                  ↓
                           Architecture (기술 설계)
                                  ↓
                           MVP Spec (구현 범위)
```

---

## 11. Non-Functional Requirements (핵심)

| NFR | Requirement | Target |
|-----|-------------|--------|
| NFR-1 | SSH 연결 지연 | ≤2초 (로컬), ≤5초 (인터넷) |
| NFR-2 | 터미널 렌더링 | 10,000줄 출력 시 <100ms 지연 |
| NFR-3 | 패인 리사이즈 | <50ms 응답 |
| NFR-5 | OS 지원 | macOS 11+, Ubuntu 20.04+, Windows 10+ |
| NFR-8 | 자동 재접속 성공률 | ≥90%, 15초 이내 |
| NFR-10 | Workset 활성화 | 4VM 2x2 기준 ≤10초 |
| NFR-12 | SSH 키 보안 | Workset에 키 경로만 저장 (내용 저장 금지) |
| NFR-13 | 비밀번호 보안 | OS 네이티브 보안 저장소 사용 |

---

## 12. AI Agent 활용 가이드

### 이 프로젝트에서 AI 에이전트가 할 일

1. **기획 문서 유지보수**: docs/ 내 문서의 정합성 유지, 용어 일관성 검증
2. **기술 스파이크 구현**: SPIKE-1/2/3 프로토타입 코드 작성 및 벤치마크
3. **MVP 구현**: 10개 MVP 기능 구현 (Architecture 컴포넌트 기반)
4. **QA**: 138개 Done Criteria 체크박스 + 10개 AC 섹션 기반 테스트

### 코드 작성 시 반드시 지킬 것

- **Rust Core**: 시스템 리소스 접근(SSH, 파일, OS Keystore)은 반드시 Rust Core에서 처리
- **Frontend**: WebView 샌드박스 내에서만 동작, 시스템 리소스 직접 접근 금지
- **IPC**: Tauri Commands(FE→BE) / Events(BE→FE)만 사용, 직접 소켓 금지
- **보안**: SSH 키 내용을 JSON에 저장하지 말 것, 비밀번호는 OS Keystore에만 저장
- **터미널 데이터**: 대량 출력 시 바이너리 IPC 또는 배치 처리 고려
