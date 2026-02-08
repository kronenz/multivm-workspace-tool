# 4.1 코드 작성 규칙

**인덱스**: `4.1`  
**계층**: Individual Contributor Guidelines (실무 가이드라인)

---

## 목적 (Purpose)

모든 IC (Individual Contributor) 에이전트가 따라야 하는 공통 코드 작성 규칙.

---

## 규칙 (Rules)

| 규칙 | 설명 | 위반 시 |
|------|------|---------|
| **Trust Boundary** | 시스템 접근은 Rust Core만 | 즉시 거부 |
| **IPC Only** | Frontend↔Backend는 Tauri Commands/Events만 | 즉시 거부 |
| **No Secret in JSON** | SSH 키 내용, 비밀번호를 JSON에 저장 금지 | 보안 위반, 즉시 거부 |
| **Type Safety** | `as any`, `@ts-ignore` 금지 (TS) / `unwrap()` 금지 (Rust) | 코드 리뷰 차단 |
| **Error Propagation** | 빈 catch 블록 금지 / Result<T,E> 반환 | 코드 리뷰 차단 |

---

## 상세 설명 (Detailed Explanation)

### 1. Trust Boundary

**원칙**: 시스템 접근은 Rust Core만

**허용**:
```rust
// Rust Core (src-tauri/)
use std::fs;
use ssh2::Session;

fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
```

**금지**:
```typescript
// Frontend (src/)
import * as fs from 'fs'; // ❌ 절대 금지

const content = fs.readFileSync('/etc/passwd'); // ❌ Trust Boundary 위반
```

**근거**: ADR-003, NFR-12, NFR-13

---

### 2. IPC Only

**원칙**: Frontend↔Backend는 Tauri Commands/Events만

**허용**:
```typescript
// Frontend
import { invoke, listen } from '@tauri-apps/api/core';

const worksets = await invoke<Workset[]>('list_worksets');

await listen<string>('terminal-output-123', (event) => {
  console.log(event.payload);
});
```

**금지**:
```typescript
// Frontend
import { WebSocket } from 'ws'; // ❌ 직접 소켓 연결 금지

const ws = new WebSocket('ws://localhost:8080'); // ❌ IPC 우회
```

---

### 3. No Secret in JSON

**원칙**: SSH 키 내용, 비밀번호를 JSON에 저장 금지

**허용**:
```json
{
  "auth": {
    "type": "key",
    "key_path": "~/.ssh/id_rsa"
  }
}
```

**금지**:
```json
{
  "auth": {
    "type": "key",
    "key_content": "-----BEGIN RSA PRIVATE KEY-----\n..." // ❌ 절대 금지
  }
}
```

**근거**: NFR-12 (SSH 키 보안), NFR-13 (비밀번호 보안)

---

### 4. Type Safety

**원칙**: TypeScript `any` 금지, Rust `unwrap()` 금지

**TypeScript 허용**:
```typescript
interface Workset {
  id: string;
  name: string;
}

const data: Workset = await invoke<Workset>('get_workset');
```

**TypeScript 금지**:
```typescript
const data: any = await invoke('get_workset'); // ❌ any 금지

// @ts-ignore // ❌ 타입 에러 무시 금지
const result = data.unknownField;
```

**Rust 허용**:
```rust
fn connect_ssh(config: SshConfig) -> Result<Session, Error> {
    let session = Session::new()?;
    session.handshake()?;
    Ok(session)
}
```

**Rust 금지**:
```rust
fn connect_ssh(config: SshConfig) -> Session {
    let session = Session::new().unwrap(); // ❌ unwrap 금지
    session.handshake().unwrap(); // ❌ 패닉 가능
    session
}
```

---

### 5. Error Propagation

**원칙**: 빈 catch 블록 금지, Result<T,E> 반환

**TypeScript 허용**:
```typescript
try {
  const worksets = await invoke<Workset[]>('list_worksets');
  return worksets;
} catch (error) {
  console.error('Failed to load worksets:', error);
  throw error; // 에러 전파
}
```

**TypeScript 금지**:
```typescript
try {
  const worksets = await invoke<Workset[]>('list_worksets');
  return worksets;
} catch (error) {
  // ❌ 빈 catch 블록 — 에러 무시
}
```

**Rust 허용**:
```rust
#[tauri::command]
async fn list_worksets() -> Result<Vec<Workset>, String> {
    let worksets = load_worksets().await?;
    Ok(worksets)
}
```

**Rust 금지**:
```rust
#[tauri::command]
async fn list_worksets() -> Vec<Workset> {
    match load_worksets().await {
        Ok(worksets) => worksets,
        Err(_) => vec![], // ❌ 에러 무시
    }
}
```

---

## 관련 문서 (Related Documents)

- `.agents/divisions/backend.md` (2.1) — Backend 부문 규칙
- `.agents/divisions/frontend.md` (2.2) — Frontend 부문 규칙
- `.agents/support/security.md` (5.2) — 보안 체크리스트
- `docs/engineering/architecture.md` — Trust Boundary 모델
