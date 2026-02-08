# .agents/ — AI Agent Organization Directory

> **Last Updated**: 2026-02-08  
> **Version**: 1.0  
> **Source**: AGENTS.md v3.0

---

## 목적 (Purpose)

이 폴더는 **Multi-VM AI Agent Workspace Tool** 프로젝트를 운영하는 AI 에이전트 조직의 **역할 정의, 책임 범위, 협업 프로토콜**을 구조화된 파일로 제공한다.

**대상 독자**:
- AI 에이전트 (Claude, GPT 등) — 작업 시 역할과 책임 확인
- 인간 개발자 — 조직 구조 이해 및 협업 규칙 참조

---

## 역할 조회 방법 (How to Look Up Roles)

### 방법 1: 인덱스 번호로 찾기

| 인덱스 | 역할 | 파일 경로 |
|--------|------|----------|
| **1.1** | Product Director | `.agents/executive/product-director.md` |
| **1.2** | Technical Director | `.agents/executive/technical-director.md` |
| **2.1** | Backend Division | `.agents/divisions/backend.md` |
| **2.2** | Frontend Division | `.agents/divisions/frontend.md` |
| **2.3** | QA Division | `.agents/divisions/qa.md` |
| **2.4** | Operations Division | `.agents/divisions/operations.md` |
| **3.1** | SSH/Connection Team | `.agents/teams/ssh-connection.md` |
| **3.2** | Workset Store Team | `.agents/teams/workset-store.md` |
| **3.3** | Process & Resource Team | `.agents/teams/process-resource.md` |
| **3.4** | Terminal Team | `.agents/teams/terminal.md` |
| **3.5** | Grid & Layout Team | `.agents/teams/grid-layout.md` |
| **3.6** | UI Components Team | `.agents/teams/ui-components.md` |
| **5.1** | Documentation Support | `.agents/support/documentation.md` |
| **5.2** | Security Support | `.agents/support/security.md` |

### 방법 2: 파일 경로로 찾기

소스 파일을 수정해야 할 때, 해당 파일의 소유 팀을 확인하려면:

1. `.agents/registry.json` 파일 열기
2. `file_ownership` 섹션에서 파일 경로 검색
3. 소유 팀의 인덱스 번호 확인
4. 해당 팀 파일 참조

**예시**:
```json
"src-tauri/src/ssh/mod.rs": "3.1"
```
→ SSH/Connection Team (3.1) 소유 → `.agents/teams/ssh-connection.md` 참조

---

## 폴더 구조 (Folder Structure)

```
.agents/
├── README.md                          # 본 파일 (조직 가이드)
├── registry.json                      # 기계 판독 가능한 역할/파일 매핑
│
├── executive/                         # 전략 계층 (1.x)
│   ├── product-director.md            # 1.1 제품 디렉터
│   ├── technical-director.md          # 1.2 기술 디렉터
│   └── escalation-matrix.md           # 1.3 의사결정 에스컬레이션
│
├── divisions/                         # 부문 계층 (2.x)
│   ├── backend.md                     # 2.1 백엔드 부문
│   ├── frontend.md                    # 2.2 프론트엔드 부문
│   ├── qa.md                          # 2.3 품질 부문
│   └── operations.md                  # 2.4 운영 부문
│
├── teams/                             # 팀 계층 (3.x)
│   ├── ssh-connection.md              # 3.1 SSH/Connection Team
│   ├── workset-store.md               # 3.2 Workset Store Team
│   ├── process-resource.md            # 3.3 Process & Resource Team
│   ├── terminal.md                    # 3.4 Terminal Team
│   ├── grid-layout.md                 # 3.5 Grid & Layout Team
│   └── ui-components.md               # 3.6 UI Components Team
│
├── guidelines/                        # 실무 가이드라인 (4.x)
│   ├── code-rules.md                  # 4.1 코드 작성 규칙
│   ├── naming-conventions.md          # 4.2 네이밍 컨벤션
│   ├── git-conventions.md             # 4.3 Git 규칙
│   └── pre-work-checklist.md          # 4.4 작업 시작 전 체크리스트
│
├── support/                           # 지원 계층 (5.x)
│   ├── documentation.md               # 5.1 문서 지원
│   └── security.md                    # 5.2 보안 지원
│
└── protocols/                         # 협업 프로토콜 (9.x)
    ├── quick-decision.md              # 9.1 빠른 결정
    ├── cross-team.md                  # 9.2 팀 간 협업
    ├── hotfix.md                      # 9.3 긴급 수정
    ├── feature-implementation.md      # 9.4 기능 구현 절차
    └── communication.md               # 9.5 커뮤니케이션 규칙
```

---

## 도메인 지식 참조 (Cross-Reference to docs/)

`.agents/` 폴더는 **조직 구조와 협업 규칙**을 정의한다.  
**도메인 지식 (제품, 기술, 품질)**은 `docs/` 폴더를 참조할 것:

| 주제 | 참조 문서 |
|------|----------|
| 제품 요구사항 | `docs/product/prd.md` |
| 시장 분석 | `docs/product/market-research.md` |
| 아키텍처 설계 | `docs/engineering/architecture.md` |
| MVP 기능 명세 | `docs/qa/mvp-spec.md` |
| 용어 정의 | `docs/glossary.md` |

---

## 핵심 원칙 (Core Principles)

1. **회사처럼 움직인다** — 빠른 의사결정, 명확한 책임, 최소 마찰의 협업
2. **소유권 명확화** — 모든 파일과 IPC는 명확한 소유 팀이 있다
3. **Trust Boundary 준수** — 시스템 접근은 Rust Core만 (ADR-003)
4. **프로토콜 우선** — 팀 간 협업은 정해진 프로토콜을 따른다

---

## 빠른 참조 — 누구에게 물어볼 것인가?

| 질문 | 담당 인덱스 | 파일 |
|------|------------|------|
| "이 기능 만들어야 해?" | **1.1** | `executive/product-director.md` |
| "아키텍처를 어떻게 잡지?" | **1.2** | `executive/technical-director.md` |
| "Rust 코드 어디에 넣지?" | **2.1** | `divisions/backend.md` |
| "UI 어떻게 만들지?" | **2.2** | `divisions/frontend.md` |
| "이거 테스트 통과해?" | **2.3** | `divisions/qa.md` |
| "빌드가 깨졌어" | **2.4** | `divisions/operations.md` |
| "SSH 모듈 수정해야 해" | **3.1** | `teams/ssh-connection.md` |
| "터미널이 깨져 보여" | **3.4** | `teams/terminal.md` |
| "보안 이슈 발견" | **5.2** | `support/security.md` |
| "협업 규칙이 뭐야?" | **9.x** | `protocols/*.md` |

---

## 사용 예시 (Usage Examples)

### 예시 1: 새 기능 구현 시작

```
1. Feature 5 (File Browser) 구현 시작
2. `docs/qa/mvp-spec.md` § Feature 5 Done Criteria 읽기
3. `.agents/teams/ssh-connection.md` (3.1 BE) + `.agents/teams/ui-components.md` (3.6 FE) 확인
4. `.agents/protocols/feature-implementation.md` (9.4) 5단계 워크플로우 따르기
5. IPC 인터페이스 변경 시 `.agents/protocols/cross-team.md` (9.2) 참조
```

### 예시 2: 빌드 실패 대응

```
1. 빌드 실패 발견
2. `.agents/protocols/hotfix.md` (9.3) 긴급 수정 프로토콜 확인
3. 해당 파일의 소유 팀 확인 (`.agents/registry.json` → `file_ownership`)
4. 소유 팀 파일 참조하여 수정
5. 30분 내 해결 불가 시 부문장(2.x)에게 에스컬레이션
```

### 예시 3: 보안 이슈 발견

```
1. SSH 키가 JSON에 평문 저장되는 코드 발견
2. `.agents/support/security.md` (5.2) 보안 체크리스트 확인
3. NFR-12 위반 확인 → 즉시 거부
4. `.agents/executive/escalation-matrix.md` (1.3) 참조 → 1.2 Technical Director 최종 승인
```

---

## 변경 이력 (Revision History)

| Date | Version | Changes |
|------|---------|---------|
| 2026-02-08 | 1.0 | Initial .agents/ folder structure created from AGENTS.md v3.0 |
