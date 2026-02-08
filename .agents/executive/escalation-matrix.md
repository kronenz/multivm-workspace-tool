# 1.3 의사결정 에스컬레이션 매트릭스

**인덱스**: `1.3`  
**계층**: Executive (전략 계층)

---

## 목적 (Purpose)

팀/부문 간 의사결정 충돌 시 **누가 최종 결정하는가**를 명확히 정의한다.

---

## 에스컬레이션 매트릭스 (Escalation Matrix)

| 상황 | 1차 결정자 | 에스컬레이션 |
|------|-----------|-------------|
| **기능 범위 논쟁** | 1.1 Product Director | — (최종 결정자) |
| **아키텍처 분쟁** | 1.2 Technical Director | 1.1과 협의 |
| **Backend vs Frontend 경계** | 2.1 + 2.2 협의 | 1.2가 중재 |
| **보안 이슈** | 5.2 Security Support | 1.2가 최종 승인 |
| **빌드 실패** | 2.4 Operations Division | 해당 부문장 |
| **일정 충돌** | 해당 부문장 | 1.1 + 1.2 합의 |

---

## 시나리오별 처리 절차 (Scenario Handling)

### 시나리오 1: 기능 범위 논쟁

**예시**: "Feature 11 (Git 통합) 추가해야 하는가?"

```
[요청자] → 1.1 Product Director
  ├─ PRD WON'T 항목 확인
  └─ 1.1이 최종 결정 (에스컬레이션 없음)
```

**결과**: 1.1이 즉시 승인 또는 거부

---

### 시나리오 2: 아키텍처 분쟁

**예시**: "Frontend에서 직접 SSH 연결해도 되는가?"

```
[요청자] → 1.2 Technical Director
  ├─ ADR-003 (Rust SSH) 위반 확인
  ├─ 1.2가 거부
  └─ 요청자가 이의 제기 시 → 1.1과 협의
```

**결과**: 1.2가 1차 결정, 이의 시 1.1과 합의

---

### 시나리오 3: Backend vs Frontend 경계

**예시**: "IPC Command 파라미터 타입을 누가 정의하는가?"

```
[요청자] → 2.1 Backend + 2.2 Frontend 협의
  ├─ 양쪽 합의 가능 → 진행
  └─ 합의 불가 → 1.2 Technical Director 중재
```

**결과**: 부문 간 협의 우선, 실패 시 1.2 중재

---

### 시나리오 4: 보안 이슈

**예시**: "SSH 키를 JSON에 저장해도 되는가?"

```
[발견자] → 5.2 Security Support
  ├─ NFR-12 위반 확인
  ├─ 5.2가 거부 권고
  └─ 1.2 Technical Director 최종 승인
```

**결과**: 5.2가 검토, 1.2가 최종 결정

---

### 시나리오 5: 빌드 실패

**예시**: "Cargo build가 깨졌다"

```
[발견자] → 2.4 Operations Division
  ├─ 원인 파일 확인 (src-tauri/)
  ├─ 2.1 Backend Division에 통보
  └─ 2.1이 해당 팀(3.x)에 수정 지시
```

**결과**: 2.4가 조율, 해당 부문장이 수정 책임

---

### 시나리오 6: 일정 충돌

**예시**: "Feature 5와 Feature 7을 동시에 구현할 리소스가 없다"

```
[부문장] → 1.1 Product + 1.2 Technical 합의
  ├─ 1.1이 우선순위 제시 (PRD 기준)
  ├─ 1.2가 기술적 실현 가능성 검토
  └─ 양쪽 합의하여 일정 조정
```

**결과**: 1.1 + 1.2 공동 결정

---

## 에스컬레이션 원칙 (Escalation Principles)

1. **최소 에스컬레이션**: 가능한 한 낮은 계층에서 해결
2. **명확한 근거**: 에스컬레이션 시 ADR, NFR, PRD 근거 제시
3. **빠른 결정**: 에스컬레이션 후 24시간 내 결정
4. **기록 의무**: 모든 에스컬레이션 결정은 커밋 메시지 또는 ADR에 기록

---

## 관련 프로토콜 (Related Protocols)

- `.agents/protocols/quick-decision.md` (9.1) — 빠른 결정 프로토콜
- `.agents/protocols/cross-team.md` (9.2) — 팀 간 협업 프로토콜
- `.agents/protocols/hotfix.md` (9.3) — 긴급 수정 프로토콜
