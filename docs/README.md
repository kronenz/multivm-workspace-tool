# Documentation Hub — 역할 기반 문서 가이드

> Multi-VM AI Agent Workspace Tool의 전체 문서 네비게이션 허브.
> 역할(Product, Engineering, Design, QA, Operations)에 따라 구성되어 있다.

---

## 폴더 구조

```
docs/
├── README.md                          ← 현재 문서 (네비게이션 허브)
├── glossary.md                        ← 공통 용어 정의 (23개 핵심 용어)
│
├── product/                           ← Product Management
│   ├── PRINCIPLES.md                  ← 제품 원칙
│   ├── prd.md                         ← 제품 요구사항 정의서
│   └── market-research.md             ← 시장 조사 및 경쟁 분석
│
├── engineering/                       ← Engineering
│   ├── PRINCIPLES.md                  ← 엔지니어링 원칙
│   ├── architecture.md                ← 아키텍처 블루프린트
│   ├── adr/                           ← Architecture Decision Records
│   │   └── README.md                  ← ADR 인덱스 + 템플릿
│   └── spikes/                        ← 기술 스파이크
│       └── README.md                  ← 스파이크 현황 + 템플릿
│
├── design/                            ← Design & UI/UX
│   ├── PRINCIPLES.md                  ← 디자인 원칙
│   └── design-system.md               ← 디자인 시스템 (컬러, 타이포, 간격)
│
├── qa/                                ← Quality Assurance
│   ├── PRINCIPLES.md                  ← QA 원칙
│   ├── mvp-spec.md                    ← MVP 사양 (10 기능, 138 체크박스)
│   └── test-strategy.md               ← 테스트 전략
│
└── operations/                        ← DevOps & Operations
    ├── PRINCIPLES.md                  ← 운영 원칙
    └── ci-cd.md                       ← CI/CD 파이프라인 설계
```

---

## 역할별 진입점

### Product Manager

> "어떤 제품을 만드는가? 누구를 위한 것인가?"

| 순서 | 문서 | 시간 | 핵심 내용 |
|------|------|------|-----------|
| 1 | [Product Principles](./product/PRINCIPLES.md) | 5분 | 제품 의사결정 원칙 |
| 2 | [Glossary](./glossary.md) | 5분 | 23개 핵심 용어 |
| 3 | [Market Research](./product/market-research.md) | 15분 | 8 경쟁사, 4 시장 공백 |
| 4 | [PRD](./product/prd.md) | 20분 | 2 페르소나, 8 MUST, MoSCoW |

### Engineer (Backend / Frontend)

> "어떻게 만드는가? 어떤 기술 결정이 있는가?"

| 순서 | 문서 | 시간 | 핵심 내용 |
|------|------|------|-----------|
| 1 | [Engineering Principles](./engineering/PRINCIPLES.md) | 5분 | Trust Boundary, IPC, 보안 규칙 |
| 2 | [Architecture](./engineering/architecture.md) | 30분 | C4 다이어그램, 9 컴포넌트, 3 ADR |
| 3 | [ADR Index](./engineering/adr/README.md) | 5분 | 기술 결정 기록 |
| 4 | [Spikes](./engineering/spikes/README.md) | 5분 | 3 기술 스파이크 현황 |
| 5 | [Glossary](./glossary.md) | 5분 | 용어 확인 |

### Designer (UI/UX)

> "어떤 모습이어야 하는가? 어떤 경험을 제공하는가?"

| 순서 | 문서 | 시간 | 핵심 내용 |
|------|------|------|-----------|
| 1 | [Design Principles](./design/PRINCIPLES.md) | 5분 | 터미널 중심, Dark-First, 접근성 |
| 2 | [Design System](./design/design-system.md) | 10분 | 컬러, 타이포, 간격 토큰 |
| 3 | [PRD](./product/prd.md) | 20분 | 페르소나, UI 관련 요구사항 |
| 4 | [MVP Spec](./qa/mvp-spec.md) | 25분 | Feature 10 (Theme), E2E 시나리오 |

### QA Engineer

> "어떻게 검증하는가? 무엇이 통과 기준인가?"

| 순서 | 문서 | 시간 | 핵심 내용 |
|------|------|------|-----------|
| 1 | [QA Principles](./qa/PRINCIPLES.md) | 5분 | AC 기반 테스트, NFR 검증 |
| 2 | [MVP Spec](./qa/mvp-spec.md) | 25분 | 138 체크박스, 10 AC, E2E 시나리오 |
| 3 | [Test Strategy](./qa/test-strategy.md) | 10분 | 테스트 피라미드, 도구, CI 통합 |
| 4 | [Architecture](./engineering/architecture.md) | 15분 | 4 리스크, 3 스파이크 성공 기준 |

### DevOps / Release Engineer

> "어떻게 빌드하고 배포하는가?"

| 순서 | 문서 | 시간 | 핵심 내용 |
|------|------|------|-----------|
| 1 | [Operations Principles](./operations/PRINCIPLES.md) | 5분 | 크로스 플랫폼, 번들 크기, 보안 |
| 2 | [CI/CD](./operations/ci-cd.md) | 10분 | 파이프라인, GitHub Actions Matrix |
| 3 | [Architecture](./engineering/architecture.md) | 10분 | ADR-001 (Tauri 번들 크기 근거) |

---

## 공통 참조

| 문서 | 경로 | 모든 역할이 참조 |
|------|------|------------------|
| **Glossary** | [glossary.md](./glossary.md) | 23개 핵심 용어 정의 |
| **AGENTS.md** | [AGENTS.md](../AGENTS.md) | AI Agent 조직 인덱스 (.agents/ 연결) |
| **.agents/** | [.agents/](../.agents/) | 역할별 가이드라인, 프로토콜, 레지스트리 (26개 파일) |
| **CLAUDE.md** | [CLAUDE.md](../CLAUDE.md) | AI 에이전트 설정 |
| **project-structure.md** | [project-structure.md](../project-structure.md) | 프로젝트 구조 및 규칙 |

---

## 문서 간 참조 흐름

```
Glossary (용어 기반)
    ↓
Market Research (시장 분석) → PRD (요구사항 정의)
                                  ↓
                           Architecture (기술 설계)
                              ↓           ↓
                           ADR/Spikes   Design System
                              ↓           ↓
                           MVP Spec ← Test Strategy
                              ↓
                           CI/CD (배포)
```

---

## 문서 상태

| 문서 | 상태 | 비고 |
|------|------|------|
| Glossary | ✅ Complete | 23개 용어 |
| Market Research | ✅ Complete | 8+4 경쟁사 |
| PRD | ✅ Complete | 2 페르소나, MoSCoW |
| Architecture | ✅ Complete | C4, 9 컴포넌트, 3 ADR |
| MVP Spec | ✅ Complete | 10 기능, 138 체크박스 |
| Product Principles | ✅ Complete | - |
| Engineering Principles | ✅ Complete | - |
| Design Principles | ✅ Complete | - |
| QA Principles | ✅ Complete | - |
| Operations Principles | ✅ Complete | - |
| ADR Index | ✅ Complete | 3 ADR 인덱싱 |
| Spikes Index | ✅ Complete | 3 스파이크 추적 |
| Design System | ⚠️ Skeleton | 구현 시 상세화 |
| Test Strategy | ⚠️ Skeleton | 구현 시 상세화 |
| CI/CD | ⚠️ Skeleton | 구현 시 상세화 |

---

**Last Updated**: 2026-02-08
**Version**: 2.1
