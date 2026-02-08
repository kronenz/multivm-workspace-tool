# 3.1 SSH/Connection Team

**인덱스**: `3.1` | **부문**: 2.1 Backend | **담당 기능**: F2, F5, F9

---

## 소유 컴포넌트 (Owned Components)

- SSH Connection Manager
- File Access Layer

---

## 소유 파일 (Owned Files)

| 파일 경로 | 설명 | 상태 |
|----------|------|------|
| `src-tauri/src/ssh/mod.rs` | SshConnectionManager (127줄) | ✅ |
| `src-tauri/src/ssh/session.rs` | SSH Session Worker (328줄) | ✅ |
| `src-tauri/src/ssh/reconnect.rs` | Auto-Reconnect 로직 | ⬜ Feature 9 |
| `src-tauri/src/file_access/` | SFTP/SSH exec 파일 접근 | ⬜ Feature 5 |

---

## IPC Commands 소유

| Command | 설명 | 상태 |
|---------|------|------|
| `activate_workset` | SSH 연결 → PTY 시작 | ✅ |
| `deactivate_workset` | 모든 SSH 세션 종료 | ✅ |
| `terminal_input` | 키 입력 전달 | ✅ |
| `terminal_resize` | 터미널 크기 변경 | ✅ |
| `list_directory` | 디렉토리 목록 조회 | ⬜ Feature 5 |
| `read_file` | 파일 내용 읽기 | ⬜ Feature 5 |

---

## IPC Events 소유

| Event | 설명 | 상태 |
|-------|------|------|
| `terminal-output-{session_id}` | PTY 출력 스트림 | ✅ |
| `session-status-{session_id}` | 연결 상태 변경 (connected, disconnected, reconnecting) | ✅ |

---

## 기술 가이드라인 (Technical Guidelines)

### 1. SSH 인증 3가지 지원

- **Key-based**: `~/.ssh/id_rsa`, `~/.ssh/id_ed25519` 등
- **Password**: OS Keystore에 저장 (NFR-13)
- **`~/.ssh/config`**: 기존 SSH 설정 파일 파싱

---

### 2. 연결 풀링

**원칙**: 하나의 VM에 Terminal + SFTP + exec 채널 공유

**구현**:
```rust
struct SshConnectionManager {
    connections: HashMap<VmId, Arc<Session>>,
}
```

**이점**:
- 중복 연결 방지
- 리소스 절약
- 연결 상태 일관성

---

### 3. Keepalive

**설정**: `ServerAliveInterval` 기반 연결 상태 감지

**구현**:
```rust
session.set_keepalive(true, 30); // 30초 간격
```

**목적**: 네트워크 끊김 조기 감지

---

### 4. Auto-Reconnect

**요구사항**: NFR-8 — 90% 성공률, 15초 이내

**전략**:
- 3회 재시도
- 지수 백오프: 1초, 2초, 4초
- Jitter: ±20% 랜덤 지연

**구현**:
```rust
async fn reconnect_with_backoff(config: &SshConfig) -> Result<Session, Error> {
    for attempt in 1..=3 {
        let delay = 2u64.pow(attempt - 1);
        let jitter = rand::random::<f64>() * 0.4 - 0.2; // ±20%
        let wait = Duration::from_secs(delay) * (1.0 + jitter);
        
        tokio::time::sleep(wait).await;
        
        match Session::connect(config).await {
            Ok(session) => return Ok(session),
            Err(e) if attempt == 3 => return Err(e),
            _ => continue,
        }
    }
}
```

---

### 5. SSH 키 내용 저장 절대 금지

**근거**: NFR-12

**허용**: 경로만 저장
```json
{
  "auth": {
    "type": "key",
    "key_path": "~/.ssh/id_rsa"
  }
}
```

**금지**: 키 내용 저장
```json
{
  "auth": {
    "type": "key",
    "key_content": "-----BEGIN RSA PRIVATE KEY-----\n..." // ❌ 절대 금지
  }
}
```

---

## Done Criteria 참조

→ `docs/qa/mvp-spec.md` § Feature 2 (AC-2), Feature 5 (AC-5), Feature 9 (AC-9)

---

## 관련 문서 (Related Documents)

- `.agents/divisions/backend.md` (2.1) — Backend 부문 규칙
- `.agents/support/security.md` (5.2) — 보안 체크리스트
- `docs/engineering/architecture.md` — SSH Connection Manager 아키텍처
