# Project Structure — Multi-VM AI Agent Workspace Tool

> Last Updated: 2026-02-07
> Version: 2.0

---

## 1. Overview

**Multi-VM AI Agent Workspace Tool**은 개발자가 2-10개의 원격 VM에서 AI 코딩 에이전트(Claude Code, OpenCode)를 동시에 운용할 수 있는 **Tauri 기반 크로스플랫폼 데스크톱 앱**이다.

10개 이상의 터미널 창을 하나의 통합 워크스페이스로 대체하며, **Workset 프로필** 하나로 SSH 접속 → 프로젝트 폴더 이동 → AI CLI 자동 실행 → Grid Layout 복원을 한 번에 수행한다.

**두 가지 핵심 소비자:**
1. **개인 개발자** — 2-3개 VM에서 다른 프로젝트를 동시에 관리
2. **스타트업 팀** — 5-7개 마이크로서비스를 병렬 AI 리팩토링으로 운용

---

## 2. Current Project Phase

**MVP Feature 1–4 구현 완료 → Feature 5–10 구현 예정**

SPIKE 검증 완료 후 MVP 구현 진행 중. Feature 1(Workset CRUD) + Feature 2(SSH) + Feature 3(Terminal) + Feature 4(Grid Layout) E2E 동작 확인.

### Phase 로드맵

```
✅ Phase 0: Planning
   └── 5개 기획 문서 완성 (glossary, market-research, prd, architecture, mvp-spec)

✅ Phase 1: Technical Spikes
   ├── SPIKE-1: Tauri + xterm.js latency 검증 — PASS
   ├── SPIKE-2: SSH 연결 풀링 스트레스 테스트 — PASS
   └── SPIKE-3: 이기종 VM 리소스 수집 호환성 [MEDIUM] — 미실행

🔨 Phase 2: MVP Development
   ├── ✅ Feature 1: Workset Profile Management (CRUD)
   ├── ✅ Feature 2: SSH Connection (Key/Password)
   ├── ✅ Feature 3: Terminal Emulator (xterm.js, 256-color)
   ├── ✅ Feature 4: Grid Layout (5 presets: 1x1, 2x1, 2x2, 2x3, 3x2)
   ├── ⬜ Feature 5: File Browser (Read-Only)
   ├── ⬜ Feature 6: Markdown Viewer
   ├── ⬜ Feature 7: Resource Monitoring (CPU/RAM/Disk)
   ├── ⬜ Feature 8: AI CLI Auto-Launch
   ├── ⬜ Feature 9: SSH Auto-Reconnect
   └── ⬜ Feature 10: Dark/Light Theme

⬜ Phase 3: QA & Release
   ├── 138개 Done Criteria 체크박스 검증
   ├── 10개 Acceptance Criteria 섹션 테스트
   └── 초기 릴리스
```

---

## 3. Folder Structure

### 현재 구조 (MVP Feature 1–4 구현 완료)

```
multivm-workspace-tool/
│
│  ┌─── 프로젝트 메타 ─────────────────────────────────────────┐
│  │                                                            │
├── CLAUDE.md                    # AI 에이전트 설정 (프로젝트 규칙)
├── AGENTS.md                    # 프로젝트 지식 베이스
├── project-structure.md         # 프로젝트 구조 정의 (본 문서)
├── README.md                    # 프로젝트 소개 및 개요
├── CONTRIBUTING.md              # 기여 가이드
├── LICENSE-MIT / LICENSE-APACHE  # 듀얼 라이선스
├── .gitignore                   # Git 무시 규칙
│  │                                                            │
│  └────────────────────────────────────────────────────────────┘
│
│  ┌─── 빌드 설정 ──────────────────────────────────────────────┐
│  │                                                            │
├── package.json                 # Node.js 의존성 (@xterm/xterm, @xterm/addon-webgl, @xterm/addon-fit)
├── tsconfig.json                # TypeScript 설정
├── vite.config.ts               # Vite 빌드 설정
├── index.html                   # Tauri WebView 진입점 (workspace-view 포함)
│  │                                                            │
│  └────────────────────────────────────────────────────────────┘
│
│  ┌─── Web Frontend (Sandboxed WebView) — vanilla TypeScript ──┐
│  │                                                            │
├── src/
│   ├── main.ts                  # 앱 진입점 (~770줄) — Workset CRUD UI + Workspace 활성화 + E2E IPC
│   ├── styles.css               # 글로벌 스타일 (~670줄) — 다크 테마 + grid/pane/toolbar CSS
│   ├── grid.ts                  # Grid Layout Engine (96줄) — 5개 프리셋, CSS Grid, 레이아웃 툴바
│   ├── terminal.ts              # Terminal Emulator (79줄) — xterm.js WebGL/Canvas, FitAddon
│   ├── workspace.ts             # Grid-Terminal 통합 (206줄) — OutputBuffer(rAF), ResizeObserver
│   └── vite-env.d.ts            # Vite 타입 선언
│  │                                                            │
│  └────────────────────────────────────────────────────────────┘
│
│  ┌─── Rust Core (Trusted Zone) ───────────────────────────────┐
│  │                                                            │
├── src-tauri/
│   ├── Cargo.toml               # Rust 의존성 (ssh2, tokio, uuid, dirs, serde_json, chrono)
│   ├── tauri.conf.json          # Tauri 앱 설정
│   ├── capabilities/            # Tauri v2 Capability 정의
│   └── src/
│       ├── main.rs              # Tauri 앱 진입점
│       ├── lib.rs               # IPC Commands (179줄) — 9개 명령 + SSH state 등록
│       │
│       ├── workset/             # ✅ Workset Store (Feature 1)
│       │   └── mod.rs           #   데이터 모델 + JSON CRUD + Validation (420줄)
│       │
│       ├── ssh/                 # ✅ SSH Connection Manager (Feature 2)
│       │   ├── mod.rs           #   SshConnectionManager — connect_all, disconnect_all (127줄)
│       │   └── session.rs       #   SSH Session Worker — PTY, keepalive, events (328줄)
│       │
│       └── bin/
│           └── spike_2_ssh_harness.rs  # SPIKE-2 테스트 하네스
│  │                                                            │
│  └────────────────────────────────────────────────────────────┘
│
│  ┌─── 기획 문서 ──────────────────────────────────────────────┐
│  │                                                            │
├── docs/
│   ├── README.md                # 문서 네비게이션 가이드
│   ├── glossary.md              # 용어 정의 (23개 핵심 용어)
│   ├── product/
│   │   ├── market-research.md   # 경쟁 분석 (8개 1차 + 4개 인접 경쟁사)
│   │   └── prd.md               # 제품 요구사항 (2 페르소나, 8 MUST, MoSCoW)
│   ├── engineering/
│   │   ├── architecture.md      # 아키텍처 블루프린트 (C4, 9 컴포넌트, 3 ADR)
│   │   ├── spike-1-tauri-xterm-latency.md   # SPIKE-1 결과 리포트
│   │   └── spike-2-ssh-pooling-stress.md    # SPIKE-2 결과 리포트
│   └── qa/
│       └── mvp-spec.md          # MVP 사양 (10 기능, 10 제외, 138 체크박스)
│  │                                                            │
│  └────────────────────────────────────────────────────────────┘
```

### 미구현 모듈 (Feature 5–10에서 추가 예정)

```
src-tauri/src/
├── process/             # Process Manager (Feature 8: AI CLI Auto-Launch)
├── resource/            # Resource Poller (Feature 7: CPU/RAM/Disk)
├── file_access/         # File Access Layer (Feature 5: File Browser)
└── ssh/reconnect.rs     # SSH Auto-Reconnect (Feature 9)

src/
├── (file-browser)       # File Browser UI (Feature 5)
├── (markdown-viewer)    # Markdown Viewer UI (Feature 6)
└── (resource-monitor)   # Resource Monitor UI (Feature 7)
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

각 디렉토리/파일의 주요 기술 영역:

| 디렉토리/파일 | 기술 | 책임 범위 | 상태 |
|--------------|------|----------|------|
| `src-tauri/src/lib.rs` | Rust + Tauri | IPC Commands 정의, state 등록 | ✅ |
| `src-tauri/src/workset/` | Rust + JSON | Workset CRUD, Validation, 영속화 | ✅ |
| `src-tauri/src/ssh/` | Rust + ssh2 | SSH 연결, PTY, keepalive, events | ✅ |
| `src-tauri/src/process/` | Rust | AI CLI 자동 실행, PTY 관리 | ⬜ |
| `src-tauri/src/resource/` | Rust | CPU/RAM/Disk 수집, OS별 파싱 | ⬜ |
| `src-tauri/src/file_access/` | Rust + SFTP | 원격 파일 시스템 읽기 | ⬜ |
| `src/main.ts` | TypeScript | Workset CRUD UI, 워크스페이스 활성화, E2E IPC | ✅ |
| `src/grid.ts` | TypeScript | CSS Grid 레이아웃, 5개 프리셋, 툴바 | ✅ |
| `src/terminal.ts` | TypeScript + xterm.js | 터미널 UI, WebGL 렌더링 | ✅ |
| `src/workspace.ts` | TypeScript | Grid-Terminal 통합, OutputBuffer, ResizeObserver | ✅ |
| `src/styles.css` | CSS | 다크 테마, Grid/Pane/Toolbar 스타일 | ✅ |

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

**Commands (Frontend → Rust Core)** — 구현 완료:

| Command | Source | Target | Description | 상태 |
|---------|--------|--------|-------------|------|
| `list_worksets` | Workset UI | Workset Store | Workset 목록 조회 | ✅ |
| `get_workset` | Workset UI | Workset Store | 단건 조회 | ✅ |
| `create_workset` | Workset UI | Workset Store | Workset 생성 | ✅ |
| `update_workset` | Workset UI | Workset Store | Workset 수정 | ✅ |
| `delete_workset` | Workset UI | Workset Store | Workset 삭제 | ✅ |
| `activate_workset` | Workspace | SSH Manager | Workset 활성화 → SSH 연결 → PTY | ✅ |
| `deactivate_workset` | Workspace | SSH Manager | 모든 SSH 세션 종료 | ✅ |
| `terminal_input` | Terminal UI | SSH Session | 키 입력 전달 | ✅ |
| `terminal_resize` | Terminal UI | SSH Session | 터미널 크기 변경 | ✅ |

**Commands — 미구현:**

| Command | Description | Feature |
|---------|-------------|---------|
| `list_directory` | 디렉토리 목록 요청 | Feature 5 |
| `read_file` | 파일 내용 읽기 | Feature 5/6 |
| `start_polling` / `stop_polling` | 리소스 수집 시작/중지 | Feature 7 |

**Events (Rust Core → Frontend)** — 구현 완료:

| Event | Source | Target | Description | 상태 |
|-------|--------|--------|-------------|------|
| `terminal-output-{session_id}` | SSH Session | Terminal UI | PTY 출력 (Vec<u8> as JSON) | ✅ |
| `session-status-{session_id}` | SSH Session | Workspace UI | 연결 상태 변경 | ✅ |

**Events — 미구현:**

| Event | Description | Feature |
|-------|-------------|---------|
| `resource_update` | CPU/RAM/Disk 데이터 | Feature 7 |
| `process_exited` | 프로세스 종료 알림 | Feature 8 |

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
| 2026-02-07 | 2.0 | MVP Feature 1–4 구현 완료 반영. 폴더 구조, Phase 로드맵, IPC 명령/이벤트, Code Ownership 갱신 |
| 2026-02-07 | 1.0 | 현재 프로젝트(Multi-VM Workspace Tool)에 맞게 전면 재작성. 기존 TeamKnowledge Vault 내용 제거 |
