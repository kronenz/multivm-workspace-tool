# Product Principles — 제품 원칙

> **역할**: Product Manager, Product Owner
> **적용 범위**: 요구사항 정의, 기능 우선순위, 사용자 경험 의사결정

---

## 핵심 원칙

### 1. 사용자 중심 설계

모든 기능 결정은 두 페르소나를 기준으로 판단한다:

| 페르소나 | 핵심 니즈 | 판단 기준 |
|----------|-----------|-----------|
| **Alex Chen** (Solo Dev) | 2-3개 VM, 컨텍스트 스위칭 감소 | "이 기능이 일일 20분 절감에 기여하는가?" |
| **Jordan Kim** (Startup Lead) | 5-7개 VM, 병렬 AI 리팩토링 | "이 기능이 팀 온보딩 2시간→15분에 기여하는가?" |

**규칙**: 두 페르소나 모두에게 가치가 없으면 MVP에 포함하지 않는다.

### 2. MoSCoW 우선순위 엄수

| 분류 | 의미 | MVP 포함 |
|------|------|----------|
| **MUST** | 없으면 제품 가치 없음 | ✅ 반드시 |
| **SHOULD** | 중요하지만 대안 존재 | ⚠️ 검토 후 |
| **COULD** | 있으면 좋지만 없어도 됨 | ❌ 제외 |
| **WON'T** | 명시적 제외 | ❌ 절대 제외 |

SHOULD→MUST 승격은 팀 합의 후에만 가능하다 (예: Dark/Light Theme은 SHOULD-1에서 승격됨).

### 3. "실행" ≠ "오케스트레이션"

이 제품의 AI CLI 관련 범위를 절대 혼동하지 않는다:

- ✅ **실행**: SSH 접속 → `cd` → `claude-code` 자동 실행
- ❌ **오케스트레이션**: AI 에이전트 간 작업 분배, 조율, 결과 병합

이 경계를 넘는 기능 요청은 즉시 거부한다.

### 4. Scope Guardrails (Must NOT Have)

PRD에 정의된 15개 Scope Guardrails를 항상 준수한다:

- 구현 타임라인/스프린트 계획을 PRD에 넣지 않는다
- 특정 Rust crate/JS 라이브러리를 PRD에서 추천하지 않는다
- 와이어프레임/디자인 목업은 PRD에 포함하지 않는다
- TAM/SAM/SOM 재무 모델링을 하지 않는다

### 5. 용어 일관성

모든 문서와 커뮤니케이션에서 [Glossary](../glossary.md)의 23개 핵심 용어를 일관되게 사용한다. 새 용어 도입 시 반드시 Glossary를 먼저 업데이트한다.

---

## 의사결정 프레임워크

기능 추가/변경 요청 시:

```
1. 두 페르소나 중 누구에게 가치가 있는가?
2. MoSCoW 분류는 무엇인가?
3. 기존 10개 MVP 기능과 충돌하는가?
4. 10개 Explicit Exclusions에 해당하는가?
5. 15개 Must NOT Have에 위반하는가?
```

하나라도 부적합하면 **거부** 또는 **Post-MVP로 연기**한다.

---

## 참조 문서

| 문서 | 경로 | 용도 |
|------|------|------|
| PRD | [prd.md](./prd.md) | 요구사항, 페르소나, MoSCoW |
| Market Research | [market-research.md](./market-research.md) | 경쟁 분석, 시장 공백 |
| Glossary | [glossary.md](../glossary.md) | 용어 정의 |
| MVP Spec | [mvp-spec.md](../qa/mvp-spec.md) | MVP 범위, 수락 기준 |

---

**Last Updated**: 2026-02-07
