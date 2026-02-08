# 9.5 Communication Rules (커뮤니케이션 규칙)

**인덱스**: `9.5`  
**계층**: Protocols (조직 횡단 프로토콜)

---

## 목적 (Purpose)

팀/부문 간 효율적인 커뮤니케이션을 위한 규칙.

---

## 5가지 규칙 (5 Rules)

| 규칙 | 설명 |
|------|------|
| **즉시 공유** | 빌드 실패, 보안 이슈, 블로커 발견 시 즉시 알릴 것 |
| **결정 기록** | 모든 아키텍처 결정은 ADR로, 나머지는 커밋 메시지로 |
| **가정 금지** | 다른 팀의 코드를 읽지 않고 가정하지 말 것 |
| **최소 컨텍스트** | 질문/요청 시: [현재 상황] + [원하는 것] + [시도한 것] |
| **No 블로킹** | 응답 대기로 작업을 멈추지 말 것 — 병렬 가능한 다른 작업 진행 |

---

## 상세 설명 (Detailed Explanation)

### 1. 즉시 공유

**원칙**: 빌드 실패, 보안 이슈, 블로커 발견 시 즉시 알릴 것

**즉시 공유 대상**:
- 빌드 실패 → 2.4 Operations Division
- 보안 이슈 → 5.2 Security Support
- 블로커 (작업 진행 불가) → 해당 팀 또는 부문장

**예시**:
```
[발견자] "cargo build 실패 — src-tauri/src/ssh/mod.rs:127 타입 에러"
  → 즉시 2.4 Operations Division 통보
```

**금지**:
- "나중에 알려드릴게요" (지연)
- "혼자 해결해보겠습니다" (블로킹)

---

### 2. 결정 기록

**원칙**: 모든 아키텍처 결정은 ADR로, 나머지는 커밋 메시지로

**ADR 기록 대상**:
- 프레임워크 선택 (Tauri vs Electron)
- 라이브러리 선택 (xterm.js)
- 아키텍처 패턴 (Trust Boundary)

**커밋 메시지 기록 대상**:
- 기능 구현
- 버그 수정
- 리팩토링

**예시**:
```markdown
<!-- ADR-003: SSH in Rust Core -->
**결정**: SSH 연결은 Rust Core에서만 처리

**근거**:
- Trust Boundary 원칙
- 보안 (NFR-12, NFR-13)
- 성능 (네이티브 SSH2 라이브러리)
```

```bash
# 커밋 메시지
git commit -m "feat(F5): add SFTP file listing"
```

---

### 3. 가정 금지

**원칙**: 다른 팀의 코드를 읽지 않고 가정하지 말 것

**금지**:
- "아마 이렇게 동작할 거야" (가정)
- "이 함수는 이런 역할일 거야" (추측)

**허용**:
- 코드 읽기 → 확인
- 해당 팀에게 질문 → 확인

**예시**:
```
❌ "터미널 렌더링은 Canvas를 쓸 거야"
✅ "src/terminal.ts를 읽어보니 WebGL을 우선 사용하고 Canvas는 폴백이네요"
```

---

### 4. 최소 컨텍스트

**원칙**: 질문/요청 시 [현재 상황] + [원하는 것] + [시도한 것]

**형식**:
```
[현재 상황]
Feature 5 (File Browser) 구현 중입니다.
SFTP로 디렉토리 목록을 가져오려고 합니다.

[원하는 것]
`list_directory` IPC Command를 추가하고 싶습니다.

[시도한 것]
src-tauri/src/file_access/mod.rs에 함수를 작성했지만,
lib.rs에 등록하는 방법을 모르겠습니다.
```

**금지**:
```
❌ "IPC Command 어떻게 추가해요?"
❌ "도와주세요"
```

---

### 5. No 블로킹

**원칙**: 응답 대기로 작업을 멈추지 말 것 — 병렬 가능한 다른 작업 진행

**시나리오**:
```
[3.1 SSH/Connection Team]
  ├─ Backend IPC Command 구현 (진행 중)
  └─ Frontend 호출 코드 대기 (블로킹 ❌)
       → 대신 단위 테스트 작성 (병렬 작업 ✅)
```

**예시**:
```
[3.1] "Frontend 팀의 타입 정의를 기다리는 동안,
       Backend 단위 테스트를 먼저 작성하겠습니다."
```

---

## 커뮤니케이션 채널 (Communication Channels)

| 상황 | 채널 | 응답 시간 |
|------|------|----------|
| 긴급 (빌드 실패, 보안) | 즉시 통보 | <5분 |
| 블로커 (작업 진행 불가) | 해당 팀/부문장 | <30분 |
| 질문 (일반) | 해당 팀 | <1시간 |
| 제안 (개선 아이디어) | 해당 부문장 | <1일 |

---

## 예시 대화 (Example Dialogue)

### 예시 1: 즉시 공유

**발견자**: "cargo build 실패 — src-tauri/src/ssh/mod.rs:127 타입 에러"  
**2.4 Ops**: "확인했습니다. 2.1 Backend Division에 통보합니다."  
**2.1 Backend**: "3.1 SSH/Connection Team, 즉시 수정 부탁드립니다."  
**3.1 SSH**: "수정 중입니다. 10분 내 완료 예정입니다."

**소요 시간**: 5분

---

### 예시 2: 최소 컨텍스트

**요청자**:
```
[현재 상황]
Feature 5 구현 중, SFTP 디렉토리 목록 가져오기 구현 완료

[원하는 것]
lib.rs에 IPC Command 등록 방법 확인

[시도한 것]
lib.rs를 열어봤지만 어디에 추가해야 할지 모르겠습니다.
```

**2.1 Backend**:
```
lib.rs의 `invoke_handler!` 매크로에 추가하세요:

invoke_handler![
  list_worksets,
  list_directory, // 여기에 추가
]
```

**소요 시간**: 2분

---

### 예시 3: No 블로킹

**3.1 SSH**:
```
Frontend 팀의 TypeScript 타입 정의를 기다리는 동안,
Backend 단위 테스트를 먼저 작성하겠습니다.
```

**3.6 UI**:
```
TypeScript 타입 정의 완료했습니다.
src/file-browser.ts에 추가했습니다.
```

**3.1 SSH**:
```
감사합니다. 단위 테스트도 완료했으니 이제 통합 테스트 진행하겠습니다.
```

**소요 시간**: 병렬 작업으로 시간 절약

---

## 관련 프로토콜 (Related Protocols)

- `.agents/protocols/quick-decision.md` (9.1) — 빠른 결정
- `.agents/protocols/cross-team.md` (9.2) — 팀 간 협업
- `.agents/protocols/hotfix.md` (9.3) — 긴급 수정
- `.agents/executive/escalation-matrix.md` (1.3) — 에스컬레이션
