# 2.1 Backend Division (백엔드 부문)

**인덱스**: `2.1`  
**계층**: Division (부문 계층)

---

## 관할 (Jurisdiction)

`src-tauri/` 전체 (Rust Core)

---

## 하위 팀 (Sub-Teams)

- **3.1** SSH/Connection Team
- **3.2** Workset Store Team
- **3.3** Process & Resource Team

---

## 소유 파일 (Owned Files)

| 파일/디렉토리 | 소유 팀 | 상태 |
|--------------|---------|------|
| `src-tauri/src/lib.rs` | 2.1 (직접) | ✅ |
| `src-tauri/src/main.rs` | 2.1 (직접) | ✅ |
| `src-tauri/src/ssh/` | 3.1 | ✅ |
| `src-tauri/src/workset/` | 3.2 | ✅ |
| `src-tauri/src/process/` | 3.3 | ⬜ 미구현 |
| `src-tauri/src/resource/` | 3.3 | ⬜ 미구현 |
| `src-tauri/src/file_access/` | 3.1 | ⬜ 미구현 |
| `src-tauri/Cargo.toml` | 2.1 (직접) | ✅ |

---

## 부문 규칙 (Division Rules)

### 1. 모든 시스템 접근은 Rust Core에서만

**근거**: ADR-003, NFR-12, NFR-13

- SSH 연결
- 파일 시스템 접근
- OS Keystore (macOS Keychain, Linux Secret Service, Windows Credential Manager)

**위반 시**: 즉시 거부

---

### 2. IPC Command 네이밍: `snake_case`

**예시**:
- `connect_ssh`
- `list_worksets`
- `terminal_input`

**금지**:
- `connectSSH` (camelCase)
- `ConnectSsh` (PascalCase)

---

### 3. 에러 핸들링: `Result<T, E>` 반환

**필수**:
- 모든 IPC Command는 `Result<T, E>` 반환
- `unwrap()` 금지 — `?` 연산자 또는 `match` 사용
- 에러를 Frontend까지 전파

**예시**:
```rust
#[tauri::command]
async fn connect_ssh(config: SshConfig) -> Result<SessionId, String> {
    let session = SshSession::connect(config).await?;
    Ok(session.id)
}
```

---

### 4. 비동기: tokio 런타임 사용

**필수**:
- 모든 I/O는 비동기 (`async`/`await`)
- 블로킹 I/O는 `spawn_blocking`으로 격리

**예시**:
```rust
use tokio::task;

async fn read_large_file(path: &str) -> Result<String, io::Error> {
    task::spawn_blocking(move || {
        std::fs::read_to_string(path)
    }).await?
}
```

---

### 5. 새 IPC Command 추가 시

**절차**:
1. `src-tauri/src/lib.rs`에 Command 등록
2. `src-tauri/capabilities/` 파일에 권한 정의
3. Frontend에서 `invoke()` 호출 코드 추가
4. 양쪽 빌드 성공 확인

**참조**: `.agents/protocols/cross-team.md` (9.2) — IPC 계약 변경 절차

---

## 협업 프로토콜 (Collaboration Protocols)

### IPC 인터페이스 변경 시

**필수**: 2.2 Frontend Division과 합의

**절차**:
1. Command/Event 스펙 작성 (이름, 파라미터, 반환값)
2. 2.1 + 2.2 양쪽 확인
3. `capabilities/` 파일 업데이트 (BE)
4. Frontend 호출 코드 업데이트 (FE)
5. 양쪽 빌드 성공 확인 후 머지

---

## 관련 문서 (Related Documents)

- `docs/engineering/architecture.md` — Rust Core 아키텍처
- `.agents/protocols/cross-team.md` (9.2) — 팀 간 협업
- `.agents/support/security.md` (5.2) — 보안 체크리스트
