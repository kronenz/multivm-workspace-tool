# 9.1 Quick Decision Protocol (빠른 결정)

**인덱스**: `9.1`  
**계층**: Protocols (조직 횡단 프로토콜)

---

## 목적 (Purpose)

30초 안에 결정, 혹은 에스컬레이션.

---

## 플로우차트 (Flowchart)

```
[요청자] → [소유 팀(3.x)]에 질문
  ├─ 소유 팀이 즉시 답변 가능 → 즉시 결정, 진행
  ├─ 다른 팀 영역과 겹침 → 9.2 Cross-Team Protocol 발동
  └─ 아키텍처 변경 필요 → 1.2 Technical Director 에스컬레이션
```

---

## 원칙 (Principles)

### 1. 기본값은 "Yes, 진행"

**원칙**: 명확한 거부 사유가 없으면 진행

**예시**:
- **요청**: "터미널에 복사/붙여넣기 단축키 추가"
- **판단**: Feature 3 Done Criteria에 포함 → 즉시 승인

---

### 2. 거부는 반드시 근거 제시

**필수**: ADR, NFR, 보안 규칙 근거

**예시**:
- **요청**: "Frontend에서 직접 SSH 연결"
- **거부 근거**: ADR-003 위반 (SSH는 Rust Core만)

---

### 3. "확인 후 답변" 금지

**원칙**: 지금 모르면 "모르니 1.2에게 물어보자" 라고 할 것

**금지**:
- "확인해보고 알려드릴게요" (블로킹)
- "나중에 답변드리겠습니다" (지연)

**허용**:
- "모르겠습니다. 1.2 Technical Director에게 에스컬레이션합니다."

---

## 시나리오별 처리 (Scenario Handling)

### 시나리오 1: 즉시 답변 가능

**예시**:
- **요청**: "터미널 스크롤백을 20,000줄로 늘려도 되나요?"
- **소유 팀**: 3.4 Terminal Team
- **판단**: NFR-2 (10,000줄) 충족, 성능 영향 미미 → 즉시 승인

**응답 시간**: <30초

---

### 시나리오 2: 다른 팀 영역과 겹침

**예시**:
- **요청**: "IPC Command 파라미터 타입 변경"
- **소유 팀**: 3.1 SSH/Connection Team (Backend)
- **판단**: Frontend 영향 있음 → 9.2 Cross-Team Protocol 발동

**응답**:
> "이 변경은 Frontend에도 영향을 줍니다. 9.2 Cross-Team Protocol에 따라 2.2 Frontend Division과 협의가 필요합니다."

---

### 시나리오 3: 아키텍처 변경 필요

**예시**:
- **요청**: "WebSocket으로 Backend-Frontend 통신 변경"
- **소유 팀**: 2.1 Backend Division
- **판단**: ADR-001 (Tauri IPC) 위반 가능 → 1.2 에스컬레이션

**응답**:
> "이 변경은 아키텍처 결정(ADR-001)에 영향을 줍니다. 1.2 Technical Director에게 에스컬레이션합니다."

---

## 의사결정 트리 (Decision Tree)

```
질문 받음
  ↓
내 팀 소유 파일인가?
  ├─ Yes → 즉시 답변 가능한가?
  │         ├─ Yes → 즉시 결정 (30초 이내)
  │         └─ No → 아키텍처 영향?
  │                  ├─ Yes → 1.2 에스컬레이션
  │                  └─ No → 다른 팀 협의 (9.2)
  └─ No → 소유 팀에게 전달
```

---

## 예시 대화 (Example Dialogue)

### 예시 1: 즉시 승인

**요청자**: "터미널에 256색 지원 추가해도 되나요?"  
**3.4 Terminal Team**: "Feature 3 Done Criteria에 포함되어 있습니다. 즉시 진행하세요."

**소요 시간**: 10초

---

### 예시 2: 근거 제시 거부

**요청자**: "Frontend에서 파일 시스템 직접 접근해도 되나요?"  
**2.2 Frontend Division**: "ADR-003 위반입니다. Trust Boundary 원칙에 따라 시스템 접근은 Rust Core만 가능합니다. 거부합니다."

**소요 시간**: 15초

---

### 예시 3: 에스컬레이션

**요청자**: "Electron으로 프레임워크 변경하면 어떨까요?"  
**2.1 Backend Division**: "ADR-001 (Tauri 선택)에 영향을 줍니다. 1.2 Technical Director에게 에스컬레이션합니다."

**소요 시간**: 20초

---

## 관련 프로토콜 (Related Protocols)

- `.agents/executive/escalation-matrix.md` (1.3) — 에스컬레이션 매트릭스
- `.agents/protocols/cross-team.md` (9.2) — 팀 간 협업
- `.agents/protocols/communication.md` (9.5) — 커뮤니케이션 규칙
