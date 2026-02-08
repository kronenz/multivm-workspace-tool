# 5.1 Documentation Support (문서 지원)

**인덱스**: `5.1`  
**계층**: Support (지원 계층)

---

## 책임 (Responsibilities)

- `docs/` 내 문서 정합성 유지
- 용어 일관성 검증 (`docs/glossary.md` 기준 23개 핵심 용어)
- 문서 간 참조 링크 무결성 확인
- 변경 이력(Revision History) 갱신

---

## 가이드라인 (Guidelines)

### 1. 모든 용어는 `docs/glossary.md` 정의를 따를 것

**목적**: 용어 일관성 유지

**절차**:
1. 새 용어 사용 전 `docs/glossary.md` 확인
2. 정의되지 않은 용어는 추가 후 사용
3. 기존 용어와 다른 의미로 사용 금지

**예시**:
- ✅ "Workset" (glossary에 정의됨)
- ✅ "VM" (glossary에 정의됨)
- ❌ "Workspace" (Workset과 혼동 가능 — glossary 확인 필요)

---

### 2. 파일명: 영문 소문자 + 하이픈 (kebab-case)

**규칙**: kebab-case

**예시**:
- ✅ `market-research.md`
- ✅ `mvp-spec.md`
- ✅ `architecture.md`
- ❌ `MarketResearch.md` (PascalCase)
- ❌ `market_research.md` (snake_case)

---

### 3. 문서 간 참조: 상대 경로

**규칙**: 상대 경로 사용

**예시**:
```markdown
<!-- ✅ 허용 -->
자세한 내용은 [PRD](./docs/product/prd.md)를 참조하세요.

<!-- ❌ 금지 -->
자세한 내용은 [PRD](/home/user/project/docs/product/prd.md)를 참조하세요.
```

**이유**: 프로젝트 경로 변경 시에도 링크 유지

---

### 4. ADR 기록: `docs/engineering/architecture.md` § ADR 섹션

**목적**: 아키텍처 결정 추적

**절차**:
1. 새 ADR 추가 시 `docs/engineering/architecture.md` § ADR 섹션에 기록
2. ADR 번호 부여 (ADR-001, ADR-002, ...)
3. 결정 내용, 근거, 상태 (Accepted/Rejected/Superseded) 명시

**형식**:
```markdown
### ADR-004: {결정 제목}

**상태**: Accepted  
**날짜**: 2026-02-08

**결정**: {무엇을 결정했는가}

**근거**:
- {이유 1}
- {이유 2}

**영향**:
- {영향 1}
- {영향 2}
```

---

## 문서 정합성 체크리스트 (Document Integrity Checklist)

### 새 문서 추가 시

```
□ 파일명이 kebab-case인가?
□ 용어가 docs/glossary.md와 일치하는가?
□ 참조 링크가 상대 경로인가?
□ Revision History가 있는가?
□ docs/README.md에 추가했는가?
```

### 기존 문서 수정 시

```
□ 용어 변경이 glossary와 일치하는가?
□ 참조 링크가 깨지지 않았는가?
□ Revision History를 갱신했는가?
□ 관련 문서도 함께 업데이트했는가?
```

---

## 문서 구조 (Document Structure)

### `docs/` 폴더 구조

```
docs/
├── README.md                      # 문서 가이드
├── glossary.md                    # 용어 정의 (23개)
├── product/
│   ├── prd.md                     # 제품 요구사항
│   └── market-research.md         # 시장 분석
├── engineering/
│   └── architecture.md            # 아키텍처 (C4, ADR)
└── qa/
    └── mvp-spec.md                # MVP 명세 (Done Criteria, AC)
```

---

## 용어 일관성 검증 (Terminology Consistency)

### 핵심 용어 (23개)

| 용어 | 정의 | 사용 예시 |
|------|------|----------|
| Workset | VM 그룹 + 설정 프로필 | "Workset을 활성화한다" |
| VM | Virtual Machine | "4개 VM을 2x2 그리드로 배치" |
| Grid Layout | 패인 배치 레이아웃 | "2x2 Grid Layout 선택" |
| Pane | 그리드 내 개별 영역 | "각 Pane에 터미널 할당" |
| SSH Session | SSH 연결 세션 | "SSH Session이 끊어졌다" |

**전체 목록**: `docs/glossary.md` 참조

---

## 변경 이력 갱신 (Revision History Update)

### 형식

```markdown
## Revision History

| Date | Version | Changes |
|------|---------|---------|
| 2026-02-08 | 3.0 | 조직 계층 구조 전면 재설계 |
| 2026-02-07 | 2.0 | MVP Feature 1–4 구현 완료 반영 |
| 2026-02-07 | 1.0 | Initial project knowledge base |
```

### 규칙

- 날짜: YYYY-MM-DD 형식
- 버전: Semantic Versioning (Major.Minor)
- 변경 내용: 간결하게 요약

---

## 관련 문서 (Related Documents)

- `docs/glossary.md` — 23개 핵심 용어 정의
- `docs/README.md` — 문서 가이드
- `.agents/guidelines/naming-conventions.md` (4.2) — 파일명 규칙
