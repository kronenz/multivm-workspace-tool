# Project Structure — Multi-VM AI Agent Workspace Tool

> Last Updated: 2026-02-07
> Version: 1.0

---

## 1. Overview

**Multi-VM AI Agent Workspace Tool**은 개발자가 2-10개의 원격 VM에서 AI 코딩 에이전트(Claude Code, OpenCode)를 동시에 운용할 수 있는 **Tauri 기반 크로스플랫폼 데스크톱 앱**이다.

10개 이상의 터미널 창을 하나의 통합 워크스페이스로 대체하며, **Workset 프로필** 하나로 SSH 접속 → 프로젝트 폴더 이동 → AI CLI 자동 실행 → Grid Layout 복원을 한 번에 수행한다.

**두 가지 핵심 소비자:**
1. **개인 개발자** — 2-3개 VM에서 다른 프로젝트를 동시에 관리
2. **스타트업 팀** — 5-7개 마이크로서비스를 병렬 AI 리팩토링으로 운용

---

## 2. Current Project Phase

**Planning Complete → 기술 스파이크 대기 중**

현재 프로젝트는 코드 구현 전 단계이며, 5개 기획 문서가 완성되어 있다. 코드 구현은 기술 스파이크(SPIKE-1/2/3) 완료 후 시작한다.

### Phase 로드맵

```
✅ Phase 0: Planning
   └── 5개 기획 문서 완성 (glossary, market-research, prd, architecture, mvp-spec)

⬜ Phase 1: Technical Spikes (Weeks 1-2)
   ├── SPIKE-1: Tauri + xterm.js latency 검증 [CRITICAL]
   ├── SPIKE-2: SSH 연결 풀링 스트레스 테스트 [HIGH]
   └── SPIKE-3: 이기종 VM 리소스 수집 호환성 [MEDIUM]

⬜ Phase 2: MVP Development (Sprints 1-5)
   ├── Sprint 1: SSH Connection Manager + Terminal Emulator
   ├── Sprint 2: Workset Manager + Grid Layout
   ├── Sprint 3: File Browser + Markdown Viewer
   ├── Sprint 4: Resource Monitoring + AI CLI Auto-Launch
   └── Sprint 5: Auto-Reconnect + Theme + Polish

⬜ Phase 3: QA & Release
   ├── 138개 Done Criteria 체크박스 검증
   ├── 10개 Acceptance Criteria 섹션 테스트
   └── 초기 릴리스
```

---

## 3. Folder Structure

### 현재 구조 (Planning Phase)

```
multivm-workspace-tool/
├── CLAUDE.md                    # AI 에이전트 설정 (프로젝트 규칙)
├── AGENTS.md                    # 프로젝트 지식 베이스
├── project-structure.md         # 프로젝트 구조 정의 (본 문서)
├── README.md                    # 프로젝트 소개 및 개요
├── CONTRIBUTING.md              # 기여 가이드
├── LICENSE-MIT                  # MIT 라이선스
├── LICENSE-APACHE               # Apache 2.0 라이선스
├── .gitignore                   # Git 무시 규칙
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

### 향후 구현 시 예상 구조

```
multivm-workspace-tool/
│
│  ┌─── 프로젝트 메타 ─────────────────────────────────────────┐
│  │                                                            │
├── CLAUDE.md                    # AI 에이전트 설정
├── AGENTS.md                    # 프로젝트 지식 베이스
├── project-structure.md         # 프로젝트 구조 정의 (본 문서)
├── README.md                    # 프로젝트 소개 및 개요
├── CONTRIBUTING.md              # 기여 가이드
├── LICENSE-MIT                  # MIT 라이선스
├── LICENSE-APACHE               # Apache 2.0 라이선스
├── .gitignore                   # Git 무시 규칙
│  │                                                            │
│  └────────────────────────────────────────────────────────────┘
│
│  ┌─── 기획 문서 ──────────────────────────────────────────────┐
│  │                                                            │
├── docs/                        # 기획 문서 모음
│   ├── README.md                # 문서 네비게이션 가이드
│   ├── glossary.md              # 용어 정의
│   ├── product/
│   │   ├── market-research.md   # 경쟁 분석
│   │   └── prd.md               # 제품 요구사항
│   ├── engineering/
│   │   └── architecture.md      # 아키텍처 블루프린트
│   └── qa/
│       └── mvp-spec.md          # MVP 사양
│  │                                                            │
│  └────────────────────────────────────────────────────────────┘
│
│  ┌─── Rust Core (Trusted Zone) ───────────────────────────────┐
│  │                                                            │
├── src-tauri/                   # Tauri Backend
│   ├── src/
│   │   ├── main.rs              # Tauri 앱 진입점
│   │   ├── lib.rs               # 모듈 선언
│   │   │
│   │   ├── ssh/                 # SSH Connection Manager
│   │   │   ├── mod.rs           #   모듈 루트
│   │   │   ├── connection.rs    #   SSH 연결 수립/종료
│   │   │   ├── pool.rs          #   연결 풀링/채널 멀티플렉싱
│   │   │   ├── auth.rs          #   인증 (키/비밀번호/config)
│   │   │   └── reconnect.rs     #   자동 재접속 로직
│   │   │
│   │   ├── process/             # Process Manager
│   │   │   ├── mod.rs
│   │   │   ├── pty.rs           #   PTY 세션 관리
│   │   │   └── launcher.rs      #   AI CLI 자동 실행
│   │   │
│   │   ├── resource/            # Resource Poller
│   │   │   ├── mod.rs
│   │   │   ├── poller.rs        #   주기적 SSH exec 실행
│   │   │   └── parser.rs        #   OS별 출력 파싱 (Linux/macOS/Alpine)
│   │   │
│   │   ├── workset/             # Workset Store
│   │   │   ├── mod.rs
│   │   │   ├── model.rs         #   Workset 데이터 모델
│   │   │   └── store.rs         #   JSON CRUD 영속화
│   │   │
│   │   ├── file_access/         # File Access Layer
│   │   │   ├── mod.rs
│   │   │   ├── sftp.rs          #   SFTP 기반 파일 접근
│   │   │   └── exec.rs          #   SSH exec 기반 파일 접근
│   │   │
│   │   └── ipc/                 # IPC Command Handlers
│   │       ├── mod.rs
│   │       ├── commands.rs      #   Tauri Commands 정의
│   │       └── events.rs        #   Tauri Events 정의
│   │
│   ├── Cargo.toml               # Rust 의존성
│   └── tauri.conf.json          # Tauri 앱 설정
│  │                                                            │
│  └────────────────────────────────────────────────────────────┘
│
│  ┌─── Web Frontend (Sandboxed WebView) ───────────────────────┐
│  │                                                            │
├── src/                         # Web Frontend
│   ├── components/
│   │   ├── grid/                # Grid Layout Engine
│   │   │   ├── GridContainer.tsx
│   │   │   ├── Pane.tsx
│   │   │   └── Divider.tsx
│   │   │
│   │   ├── terminal/            # Terminal Emulator (xterm.js)
│   │   │   ├── TerminalPane.tsx
│   │   │   └── useTerminal.ts
│   │   │
│   │   ├── file-browser/        # File Browser UI
│   │   │   ├── FileBrowser.tsx
│   │   │   └── FileTree.tsx
│   │   │
│   │   ├── markdown-viewer/     # Markdown Viewer UI
│   │   │   └── MarkdownViewer.tsx
│   │   │
│   │   ├── resource-monitor/    # Resource Monitor UI
│   │   │   └── ResourceBar.tsx
│   │   │
│   │   └── workset/             # Workset Manager UI
│   │       ├── WorksetSidebar.tsx
│   │       └── WorksetForm.tsx
│   │
│   ├── hooks/                   # Shared React Hooks
│   ├── utils/                   # 유틸리티 함수
│   ├── types/                   # TypeScript 타입 정의
│   ├── App.tsx                  # 앱 루트 컴포넌트
│   └── main.tsx                 # 앱 진입점
│  │                                                            │
│  └────────────────────────────────────────────────────────────┘
│
├── package.json                 # Node.js 의존성
├── tsconfig.json                # TypeScript 설정
└── vite.config.ts               # Vite 빌드 설정
```

---

## 4. Rules

### 4.1 Architecture Boundary Rules

Tauri의 Trust Boundary 모델에 따라, 코드 작성 시 반드시 아래 규칙을 지킨다.

| 규칙 | 설명 | 위반 시 |
|------|------|---------|
| **시스템 접근 = Rust Core** | SSH, 파일 시스템, OS Keystore 접근은 반드시 Rust Core에서 처리 | 보안 위반 |
| **Frontend = 샌드박스** | Web Frontend는 WebView 내에서만 동작, 시스템 리소스 직접 접근 금지 | Tauri 보안 모델 위반 |
| **IPC만 사용** | Frontend↔Backend 통신은 Tauri Commands / Events만 사용 | 직접 소켓 금지 |
| **SSH 키 내용 저장 금지** | Workset JSON에는 키 파일 경로만 저장 (NFR-12) | 보안 위반 |
| **비밀번호 = OS Keystore** | SSH 비밀번호는 OS 네이티브 보안 저장소에만 저장 (NFR-13) | 보안 위반 |

### 4.2 Code Ownership

구현 시 각 디렉토리의 주요 기술 영역:

| 디렉토리 | 기술 | 책임 범위 |
|----------|------|----------|
| `src-tauri/src/ssh/` | Rust + SSH Library | SSH 연결, 인증, 재접속, 채널 관리 |
| `src-tauri/src/process/` | Rust | PTY 관리, AI CLI 실행 |
| `src-tauri/src/resource/` | Rust | CPU/RAM/Disk 수집, OS별 파싱 |
| `src-tauri/src/workset/` | Rust + JSON | Workset CRUD, 영속화 |
| `src-tauri/src/file_access/` | Rust + SFTP | 원격 파일 시스템 읽기 |
| `src-tauri/src/ipc/` | Rust + Tauri | Command/Event 핸들러 |
| `src/components/grid/` | TypeScript/React | Grid Layout 렌더링, 리사이즈 |
| `src/components/terminal/` | TypeScript + xterm.js | 터미널 UI, WebGL 렌더링 |
| `src/components/file-browser/` | TypeScript/React | 파일 트리 뷰 |
| `src/components/markdown-viewer/` | TypeScript/React | MD 렌더링, 구문 강조 |
| `src/components/resource-monitor/` | TypeScript/React | CPU/RAM/Disk 표시 |
| `src/components/workset/` | TypeScript/React | Workset 사이드바, CRUD 폼 |

### 4.3 Naming Conventions

| 대상 | 규칙 | 예시 |
|------|------|------|
| **Rust 모듈** | snake_case | `ssh_connection.rs`, `resource_poller.rs` |
| **Rust 타입/구조체** | PascalCase | `SshSession`, `WorksetProfile` |
| **Rust 함수** | snake_case | `connect_ssh()`, `parse_cpu_usage()` |
| **TypeScript 컴포넌트** | PascalCase | `TerminalPane.tsx`, `WorksetForm.tsx` |
| **TypeScript 훅** | camelCase with `use` prefix | `useTerminal.ts`, `useWorkset.ts` |
| **TypeScript 유틸** | camelCase | `formatBytes.ts`, `parseConfig.ts` |
| **IPC Commands** | snake_case | `connect_ssh`, `list_directory`, `activate_workset` |
| **IPC Events** | snake_case | `terminal_output`, `resource_update`, `ssh_state_changed` |
| **Workset 파일** | kebab-case | `my-project.json`, `microservices-dev.json` |
| **기획 문서** | kebab-case | `market-research.md`, `mvp-spec.md` |

### 4.4 Documentation Rules

| 규칙 | 설명 |
|------|------|
| **용어 일관성** | 모든 문서는 `docs/glossary.md`의 정의를 따른다 |
| **영문 파일명** | 파일명은 영문 소문자 + 하이픈 (kebab-case). 한글 금지 |
| **문서 간 참조** | 상대 경로로 링크. 예: `[PRD](./docs/product/prd.md)` |
| **ADR 기록** | 주요 아키텍처 결정은 `docs/engineering/architecture.md`의 ADR 섹션에 기록 |
| **변경 이력** | 각 문서 하단에 Revision History 유지 |

### 4.5 Security Rules

| 규칙 | 근거 |
|------|------|
| SSH 키 **내용** 절대 JSON 저장 금지 | NFR-12 |
| SSH 비밀번호는 OS Keystore만 사용 | NFR-13 |
| Tauri Capabilities로 Command별 접근 제어 | ADR-001 |
| CSP로 WebView 외부 리소스 접근 제한 | Tauri 보안 모델 |
| `.env`, 인증 파일은 `.gitignore`에 포함 | 기본 보안 |

---

## 5. Component → Feature Mapping

PRD 요구사항 → Architecture 컴포넌트 → MVP 기능의 완전한 매핑:

| PRD Feature | Architecture Component | MVP Feature | AC |
|-------------|------------------------|-------------|-----|
| MUST-1: Workset Profile | Workset Manager | Feature 1 | AC-1 |
| MUST-2: SSH Connection | SSH Connection Manager | Feature 2 | AC-2 |
| MUST-3: Terminal Emulator | Terminal Emulator | Feature 3 | AC-3 |
| MUST-4: Grid Layout | Grid Layout Engine | Feature 4 | AC-4 |
| MUST-5: File Browser | File Browser | Feature 5 | AC-5 |
| MUST-6: Markdown Viewer | Markdown Renderer | Feature 6 | AC-6 |
| MUST-7: Resource Monitoring | Resource Poller | Feature 7 | AC-7 |
| MUST-8: AI CLI Auto-Launch | Process Manager | Feature 8 | AC-8 |
| MUST-2 enhanced | SSH Connection Manager | Feature 9 | AC-9 |
| SHOULD-1 promoted | Frontend (Theme) | Feature 10 | AC-10 |

**Coverage**: 8/8 MUST features → 9 Architecture components → 10 MVP features → 10 AC sections. 100% 커버리지.

---

## 6. Communication Flow

### IPC 명령/이벤트 요약

**Commands (Frontend → Rust Core)**:

| Command | Source Component | Target Component | Description |
|---------|-----------------|-----------------|-------------|
| `connect_ssh` | Workset Manager UI | SSH Connection Manager | SSH 연결 수립 |
| `disconnect_ssh` | Workset Manager UI | SSH Connection Manager | SSH 연결 종료 |
| `terminal_input` | Terminal Emulator UI | Process Manager | 키 입력 전달 |
| `terminal_resize` | Terminal Emulator UI | Process Manager | 터미널 크기 변경 |
| `list_directory` | File Browser UI | File Access Layer | 디렉토리 목록 요청 |
| `read_file` | Markdown Viewer UI | File Access Layer | 파일 내용 읽기 |
| `activate_workset` | Workset Manager UI | Workset Store | Workset 활성화 |
| `create_workset` | Workset Manager UI | Workset Store | Workset 생성 |
| `update_workset` | Workset Manager UI | Workset Store | Workset 수정 |
| `delete_workset` | Workset Manager UI | Workset Store | Workset 삭제 |
| `list_worksets` | Workset Manager UI | Workset Store | Workset 목록 조회 |
| `save_layout` | Grid Layout Engine | Workset Store | 레이아웃 저장 |
| `start_polling` | Workset Manager | Resource Poller | 리소스 수집 시작 |
| `stop_polling` | Workset Manager | Resource Poller | 리소스 수집 중지 |

**Events (Rust Core → Frontend)**:

| Event | Source Component | Target Component | Description |
|-------|-----------------|-----------------|-------------|
| `terminal_output` | Process Manager | Terminal Emulator UI | PTY 출력 스트리밍 |
| `ssh_state_changed` | SSH Connection Manager | Workset Manager UI | 연결 상태 변경 |
| `resource_update` | Resource Poller | Resource Monitor UI | CPU/RAM/Disk 데이터 |
| `process_exited` | Process Manager | Terminal Emulator UI | 프로세스 종료 알림 |
| `file_content_updated` | File Access Layer | Markdown Viewer UI | 파일 변경 감지 |

---

## 7. Data Persistence

### Workset 저장 경로

```
~/.config/multivm-workspace/
├── worksets/                    # Workset 프로필 (JSON)
│   ├── my-project.json
│   ├── microservices-dev.json
│   └── ...
└── settings.json                # 앱 설정 (테마, 단축키 등)
```

### Workset JSON 스키마 (예상)

```json
{
  "name": "Microservices Dev",
  "gridLayout": "2x2",
  "panes": [
    {
      "position": "top-left",
      "type": "terminal",
      "ssh": {
        "host": "auth-vm.example.com",
        "port": 22,
        "user": "dev",
        "authMethod": "key",
        "keyPath": "~/.ssh/id_rsa"
      },
      "projectFolder": "/home/dev/auth-service",
      "aiCliCommand": "claude-code"
    }
  ]
}
```

> **보안 참고**: `ssh.keyPath`에는 키 파일 경로만 저장. 키 내용이나 비밀번호는 절대 JSON에 포함하지 않음.

---

## 8. AI Agent 활용 가이드

### 문서 참조 순서

AI 에이전트가 이 프로젝트를 이해하기 위한 권장 순서:

1. **본 문서 (`project-structure.md`)** — 구조, 규칙, 컨벤션
2. **`AGENTS.md`** — 프로젝트 전체 요약, 아키텍처, 기술 스택
3. **`docs/glossary.md`** — 23개 핵심 용어 정의
4. **`docs/engineering/architecture.md`** — C4 다이어그램, 컴포넌트, ADR, 리스크
5. **`docs/qa/mvp-spec.md`** — 10 기능, 10 제외, E2E 시나리오, AC

### 코드 구현 시 참조 문서

| 작업 | 참조 문서 |
|------|----------|
| SSH 연결 구현 | `docs/engineering/architecture.md` § SSH Connection Manager, ADR-003 |
| 터미널 구현 | `docs/engineering/architecture.md` § Terminal Emulator, ADR-002 |
| Workset 구현 | `docs/qa/mvp-spec.md` § Feature 1, AC-1 |
| Grid Layout 구현 | `docs/qa/mvp-spec.md` § Feature 4, AC-4 |
| 리소스 모니터링 | `docs/qa/mvp-spec.md` § Feature 7, Resource Monitoring Detailed Scope |
| AI CLI 자동 실행 | `docs/qa/mvp-spec.md` § Feature 8, AC-8 |
| 보안 설계 | `docs/engineering/architecture.md` § Security Considerations |
| 용어 확인 | `docs/glossary.md` |
| 경쟁 분석 맥락 | `docs/product/market-research.md` |

---

## Revision History

| Date | Version | Changes |
|------|---------|---------|
| 2026-02-07 | 1.0 | 현재 프로젝트(Multi-VM Workspace Tool)에 맞게 전면 재작성. 기존 TeamKnowledge Vault 내용 제거 |
