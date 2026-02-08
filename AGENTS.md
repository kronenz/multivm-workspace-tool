# AGENTS.md — AI Agent Organization Index

> Last Updated: 2026-02-08
> Version: 4.0
> Status: MVP Feature 1–4 완료, Feature 5–10 구현 대기

---

## 0. 조직 개요 (Organization Overview)

이 문서는 **Multi-VM AI Agent Workspace Tool** 프로젝트를 운영하는 AI 에이전트 조직의 **계층 구조, 역할별 가이드라인, 협업 프로토콜**을 정의한다.

**핵심 원칙**: 회사처럼 움직인다. 빠른 의사결정, 명확한 책임, 최소 마찰의 협업.

**기계 판독 인덱스**: [.agents/registry.json](./.agents/registry.json)

### 0.1 인덱스 번호 체계

```
1.x.x — Executive (전략 계층)        → 프로젝트 방향, 아키텍처 결정
2.x.x — Division (부문 계층)         → Backend / Frontend / QA / Ops 부문장
3.x.x — Team (팀 계층)               → 컴포넌트별 팀 리드
4.x.x — Individual Contributor (실무) → 기능 구현, 버그 수정
5.x.x — Support (지원 계층)          → 문서, 보안, DevOps
9.x.x — Protocol (프로토콜)          → 조직 횡단 협업 규칙
```

### 0.2 빠른 참조 — 누구에게 물어볼 것인가?

| 질문 | 담당 인덱스 | 역할 | 상세 |
|------|------------|------|------|
| "이 기능 만들어야 해?" | **1.1** | Product Director | [.agents/executive/product-director.md](./.agents/executive/product-director.md) |
| "아키텍처를 어떻게 잡지?" | **1.2** | Technical Director | [.agents/executive/technical-director.md](./.agents/executive/technical-director.md) |
| "Rust 코드 어디에 넣지?" | **2.1** | Backend Division | [.agents/divisions/backend.md](./.agents/divisions/backend.md) |
| "UI 어떻게 만들지?" | **2.2** | Frontend Division | [.agents/divisions/frontend.md](./.agents/divisions/frontend.md) |
| "이거 테스트 통과해?" | **2.3** | QA Division | [.agents/divisions/qa.md](./.agents/divisions/qa.md) |
| "빌드가 깨졌어" | **2.4** | Operations Division | [.agents/divisions/operations.md](./.agents/divisions/operations.md) |
| "SSH 모듈 수정해야 해" | **3.1** | SSH/Connection Team | [.agents/teams/ssh-connection.md](./.agents/teams/ssh-connection.md) |
| "터미널이 깨져 보여" | **3.4** | Terminal Team | [.agents/teams/terminal.md](./.agents/teams/terminal.md) |
| "보안 이슈 발견" | **5.2** | Security Support | [.agents/support/security.md](./.agents/support/security.md) |
| "협업 규칙이 뭐야?" | **9.x** | Protocol | [.agents/protocols/](./.agents/protocols/) |

---

## 1. Roles (역할 인덱스)

### Executive Layer (전략 계층)

| 인덱스 | 역할 | 책임 | 상세 |
|--------|------|------|------|
| 1.1 | Product Director | 제품 비전, 로드맵, 우선순위 | [.agents/executive/product-director.md](./.agents/executive/product-director.md) |
| 1.2 | Technical Director | 아키텍처, ADR, Trust Boundary | [.agents/executive/technical-director.md](./.agents/executive/technical-director.md) |
| 1.3 | Escalation Matrix | 의사결정 에스컬레이션 | [.agents/executive/escalation.md](./.agents/executive/escalation.md) |

### Division Layer (부문 계층)

| 인덱스 | 부문 | 관할 | 하위 팀 | 상세 |
|--------|------|------|--------|------|
| 2.1 | Backend | `src-tauri/` | 3.1, 3.2, 3.3 | [.agents/divisions/backend.md](./.agents/divisions/backend.md) |
| 2.2 | Frontend | `src/` | 3.4, 3.5, 3.6 | [.agents/divisions/frontend.md](./.agents/divisions/frontend.md) |
| 2.3 | QA | 테스트, AC, NFR | — | [.agents/divisions/qa.md](./.agents/divisions/qa.md) |
| 2.4 | Operations | 빌드, CI/CD, 릴리스 | — | [.agents/divisions/operations.md](./.agents/divisions/operations.md) |

### Team Layer (팀 계층)

| 인덱스 | 팀 | 부문 | 담당 기능 | 상세 |
|--------|----|----|---------|------|
| 3.1 | SSH/Connection | 2.1 | F2, F5, F9 | [.agents/teams/ssh-connection.md](./.agents/teams/ssh-connection.md) |
| 3.2 | Workset Store | 2.1 | F1 | [.agents/teams/workset-store.md](./.agents/teams/workset-store.md) |
| 3.3 | Process & Resource | 2.1 | F7, F8 | [.agents/teams/process-resource.md](./.agents/teams/process-resource.md) |
| 3.4 | Terminal | 2.2 | F3 | [.agents/teams/terminal.md](./.agents/teams/terminal.md) |
| 3.5 | Grid & Layout | 2.2 | F4 | [.agents/teams/grid-layout.md](./.agents/teams/grid-layout.md) |
| 3.6 | UI Components | 2.2 | F5, F6, F7, F10 | [.agents/teams/ui-components.md](./.agents/teams/ui-components.md) |

### Individual Contributor Guidelines (실무 가이드라인)

| 항목 | 설명 | 상세 |
|------|------|------|
| 4.1 | 코드 작성 규칙 | Trust Boundary, IPC Only, Type Safety | [.agents/ic/code-rules.md](./.agents/ic/code-rules.md) |
| 4.2 | 네이밍 컨벤션 | Rust, TypeScript, IPC, JSON | [.agents/ic/naming.md](./.agents/ic/naming.md) |
| 4.3 | 커밋 & 브랜치 | Atomic commits, branch naming | [.agents/ic/git-rules.md](./.agents/ic/git-rules.md) |
| 4.4 | 작업 시작 체크리스트 | Feature, Team, Trust Boundary | [.agents/ic/checklist.md](./.agents/ic/checklist.md) |

### Support Layer (지원 계층)

| 인덱스 | 역할 | 책임 | 상세 |
|--------|------|------|------|
| 5.1 | Documentation Support | 문서 정합성, 용어 일관성 | [.agents/support/documentation.md](./.agents/support/documentation.md) |
| 5.2 | Security Support | 보안 규칙, Capabilities, CSP | [.agents/support/security.md](./.agents/support/security.md) |

---

## 2. Protocols (협업 프로토콜)

| 프로토콜 | 목적 | 상세 |
|---------|------|------|
| 9.1 Quick Decision | 30초 안에 결정 또는 에스컬레이션 | [.agents/protocols/quick-decision.md](./.agents/protocols/quick-decision.md) |
| 9.2 Cross-Team | 팀 간 협업, IPC 계약 변경 | [.agents/protocols/cross-team.md](./.agents/protocols/cross-team.md) |
| 9.3 Hotfix | 빌드 깨짐, 크리티컬 버그 대응 | [.agents/protocols/hotfix.md](./.agents/protocols/hotfix.md) |
| 9.4 Feature Implementation | Feature 5–10 구현 절차 | [.agents/protocols/feature-implementation.md](./.agents/protocols/feature-implementation.md) |
| 9.5 Communication Rules | 즉시 공유, 결정 기록, 최소 컨텍스트 | [.agents/protocols/communication.md](./.agents/protocols/communication.md) |

---

## A. Appendix: Project Quick Reference

### A.1 Tech Stack

| Layer | Technology | Role |
|-------|------------|------|
| Desktop Framework | Tauri v2 | Rust Core + Web Frontend + IPC Bridge |
| Backend | Rust (tokio, ssh2, serde_json) | SSH, 프로세스, 리소스, Workset, 파일 |
| Frontend | TypeScript (vanilla) | Grid Layout, UI 컴포넌트 |
| Terminal | xterm.js v6 | WebGL 렌더러 기반 터미널 에뮬레이션 |
| Data Persistence | JSON Files | `~/.config/multivm-workspace/worksets/` |
| Credential Storage | OS Keystore | macOS Keychain, Linux Secret Service, Windows Credential Manager |

### A.2 MVP Features Status

| # | Feature | Owner Team | PRD | Status |
|---|---------|-----------|-----|--------|
| 1 | Workset CRUD | 3.2 | MUST-1 | ✅ |
| 2 | SSH Connection | 3.1 | MUST-2 | ✅ |
| 3 | Terminal Emulator | 3.4 | MUST-3 | ✅ |
| 4 | Grid Layout | 3.5 | MUST-4 | ✅ |
| 5 | File Browser | 3.1 + 3.6 | MUST-5 | ⬜ |
| 6 | Markdown Viewer | 3.6 | MUST-6 | ⬜ |
| 7 | Resource Monitoring | 3.3 + 3.6 | MUST-7 | ⬜ |
| 8 | AI CLI Auto-Launch | 3.3 | MUST-8 | ⬜ |
| 9 | SSH Auto-Reconnect | 3.1 | MUST-2+ | ⬜ |
| 10 | Dark/Light Theme | 3.6 | SHOULD-1 | ⬜ |

### A.3 Technical Risks

| Risk | Severity | Mitigation | Status |
|------|----------|------------|--------|
| RISK-1: xterm.js latency | CRITICAL | SPIKE-1 | ✅ PARTIAL PASS |
| RISK-2: Multi-SSH stability | HIGH | SPIKE-2 | ✅ PASS |
| RISK-3: OS resource compat | MEDIUM | SPIKE-3 | ⬜ 미실행 |
| RISK-4: IPC serialization | HIGH | Binary IPC, throttling | ⬜ 미검증 |

### A.4 Key Decisions (ADR)

| ADR | Decision | Status |
|-----|----------|--------|
| ADR-001 | Tauri (not Electron) | Accepted |
| ADR-002 | xterm.js for terminal | Conditionally Accepted |
| ADR-003 | SSH in Rust Core | Accepted |

### A.5 Document Map

| Document | Path | Purpose |
|----------|------|---------|
| CLAUDE.md | `./CLAUDE.md` | AI 에이전트 설정 (규칙 요약) |
| AGENTS.md | `./AGENTS.md` | 본 문서 (조직 인덱스) |
| Project Structure | `./project-structure.md` | 프로젝트 구조, 규칙, 컨벤션 |
| .agents/ folder | `./.agents/` | 상세 역할, 팀, 프로토콜 문서 |
| Glossary | `./docs/glossary.md` | 23개 핵심 용어 |
| PRD | `./docs/product/prd.md` | 제품 요구사항, MoSCoW |
| Architecture | `./docs/engineering/architecture.md` | C4, 9 컴포넌트, 3 ADR |
| MVP Spec | `./docs/qa/mvp-spec.md` | 10 기능, 138 체크박스, E2E |
| Market Research | `./docs/product/market-research.md` | 8 경쟁사, 시장 분석 |

### A.6 NFR Quick Reference

| NFR | Requirement | Target |
|-----|-------------|--------|
| NFR-1 | SSH 연결 지연 | ≤2초 (로컬), ≤5초 (인터넷) |
| NFR-2 | 터미널 렌더링 | 10K줄 <100ms |
| NFR-3 | 패인 리사이즈 | <50ms |
| NFR-5 | OS 지원 | macOS 11+, Ubuntu 20.04+, Win 10+ |
| NFR-8 | 자동 재접속 | ≥90%, 15초 이내 |
| NFR-10 | Workset 활성화 | 4VM 2x2 ≤10초 |
| NFR-12 | SSH 키 보안 | 경로만 저장 |
| NFR-13 | 비밀번호 보안 | OS Keystore만 |

---

## Revision History

| Date | Version | Changes |
|------|---------|---------|
| 2026-02-08 | 4.0 | AGENTS.md 슬림화: 694줄 → 150줄 인덱스. 상세 내용을 .agents/ 폴더 파일로 이동. 모든 섹션에 링크 추가. |
| 2026-02-08 | 3.0 | 조직 계층 구조 전면 재설계: 5계층 인덱스 체계(1-5, 9), 역할별 가이드라인, 협업 프로토콜(9.1-9.5), 팀별 소유 파일/IPC 매핑 |
| 2026-02-07 | 2.0 | MVP Feature 1–4 구현 완료 반영 |
| 2026-02-07 | 1.0 | Initial project knowledge base |
