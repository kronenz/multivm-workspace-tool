# 9.3 Hotfix Protocol (긴급 수정)

**인덱스**: `9.3`  
**계층**: Protocols (조직 횡단 프로토콜)

---

## 목적 (Purpose)

빌드 깨짐 또는 크리티컬 버그 시 최우선 대응.

---

## 플로우차트 (Flowchart)

```
[발견자] → 즉시 공유
  ↓
[해당 팀(3.x)] 30분 내 수정
  ├─ 수정 완료 → 빌드 확인 → 머지
  └─ 30분 내 불가 → 2.x 부문장 에스컬레이션
       └─ 부문장이 리소스 재배치 또는 롤백 결정
```

---

## 규칙 (Rules)

### 1. Hotfix는 최소 변경만

**원칙**: 리팩토링 금지

**허용**:
```rust
// ✅ 최소 변경 — 에러 처리 추가
fn connect_ssh(config: &SshConfig) -> Result<Session, Error> {
    let session = Session::new()?;
    session.handshake()?; // 기존 코드
    Ok(session)
}
```

**금지**:
```rust
// ❌ 리팩토링 — Hotfix 범위 초과
fn connect_ssh(config: &SshConfig) -> Result<Session, Error> {
    // 전체 연결 로직 재작성
    let pool = ConnectionPool::new();
    pool.get_or_create(config)
}
```

---

### 2. 기존 기능 회귀 시 즉시 롤백

**원칙**: Feature 1-4 깨뜨리면 즉시 롤백

**절차**:
```bash
# 회귀 발견
git log --oneline -5

# 문제 커밋 확인
git show <commit-hash>

# 즉시 롤백
git revert <commit-hash>
git push origin main
```

---

### 3. Hotfix 후 반드시 원인 분석 기록

**책임**: 5.1 Documentation Support

**형식**:
```markdown
## Hotfix: {날짜} — {문제 요약}

**문제**: {무엇이 깨졌는가}

**원인**: {왜 발생했는가}

**수정**: {어떻게 고쳤는가}

**재발 방지**: {앞으로 어떻게 막을 것인가}
```

**저장 위치**: `docs/hotfix-log.md` (새 파일 생성)

---

## 시나리오별 처리 (Scenario Handling)

### 시나리오 1: 빌드 깨짐

**예시**: `cargo build` 실패

**절차**:
```
[발견자] → 2.4 Operations Division 통보
  ↓
[2.4] 원인 파일 확인 (src-tauri/src/ssh/mod.rs)
  ↓
[2.4] → 2.1 Backend Division 통보
  ↓
[2.1] → 3.1 SSH/Connection Team 수정 지시
  ↓
[3.1] 30분 내 수정
  ├─ 수정 완료 → cargo build 성공 → 머지
  └─ 30분 내 불가 → 2.1 에스컬레이션
       └─ 2.1이 다른 팀원 투입 또는 롤백 결정
```

**소요 시간**: 30분 이내

---

### 시나리오 2: 크리티컬 버그

**예시**: SSH 연결 시 앱 크래시

**절차**:
```
[발견자] → 해당 팀(3.1) 통보
  ↓
[3.1] 즉시 재현 시도
  ├─ 재현 성공 → 원인 파악 → 수정
  └─ 재현 실패 → 추가 정보 요청
  ↓
[3.1] 30분 내 수정
  ├─ 수정 완료 → 테스트 → 머지
  └─ 30분 내 불가 → 2.1 에스컬레이션
       └─ 2.1이 임시 우회 방법 또는 롤백 결정
```

**소요 시간**: 30분-1시간

---

### 시나리오 3: 보안 이슈

**예시**: SSH 키가 JSON에 평문 저장됨

**절차**:
```
[발견자] → 5.2 Security Support 통보
  ↓
[5.2] 심각도 평가 (Critical)
  ↓
[5.2] → 1.2 Technical Director 보고
  ↓
[1.2] 즉시 수정 지시
  ↓
[해당 팀] 1시간 내 수정
  ├─ 수정 완료 → 보안 검증 → 머지
  └─ 1시간 내 불가 → 기능 비활성화 또는 롤백
```

**소요 시간**: 1시간 이내 (Critical)

---

## Hotfix 체크리스트 (Hotfix Checklist)

### 수정 전

```
□ 문제를 재현할 수 있는가?
□ 원인을 파악했는가?
□ 최소 변경으로 해결 가능한가?
□ 기존 기능에 영향을 주지 않는가?
```

### 수정 후

```
□ 빌드가 성공하는가?
□ 문제가 해결되었는가?
□ 기존 Feature 1-4가 정상 작동하는가?
□ 원인 분석을 기록했는가?
```

---

## 예시 Hotfix (Example Hotfix)

### 문제: SSH 연결 타임아웃 처리 누락

**발견**:
```
[2026-02-08 14:30] 사용자 보고: SSH 연결 시 앱이 멈춤
```

**원인**:
```rust
// src-tauri/src/ssh/session.rs
fn connect(&self) -> Result<Session, Error> {
    let session = Session::new()?;
    session.handshake()?; // 타임아웃 없음 — 무한 대기
    Ok(session)
}
```

**수정**:
```rust
// src-tauri/src/ssh/session.rs
fn connect(&self) -> Result<Session, Error> {
    let session = Session::new()?;
    session.set_timeout(5000); // 5초 타임아웃 추가
    session.handshake()?;
    Ok(session)
}
```

**커밋**:
```bash
git add src-tauri/src/ssh/session.rs
git commit -m "fix(F2): add 5s timeout to SSH handshake"
git push origin main
```

**원인 분석 기록**:
```markdown
## Hotfix: 2026-02-08 — SSH 연결 타임아웃 처리 누락

**문제**: SSH 연결 시 앱이 무한 대기

**원인**: `session.handshake()`에 타임아웃 설정 누락

**수정**: `session.set_timeout(5000)` 추가 (5초)

**재발 방지**: 모든 네트워크 I/O에 타임아웃 필수 (코드 리뷰 체크리스트 추가)
```

**소요 시간**: 25분

---

## 관련 프로토콜 (Related Protocols)

- `.agents/divisions/operations.md` (2.4) — 빌드 실패 대응
- `.agents/support/security.md` (5.2) — 보안 이슈 대응
- `.agents/support/documentation.md` (5.1) — 원인 분석 기록
