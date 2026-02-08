# 4.2 네이밍 컨벤션

**인덱스**: `4.2`  
**계층**: Individual Contributor Guidelines (실무 가이드라인)

---

## 목적 (Purpose)

프로젝트 전체에서 일관된 네이밍 규칙을 적용한다.

---

## 네이밍 규칙 (Naming Rules)

| 대상 | 규칙 | 예시 |
|------|------|------|
| **Rust 모듈** | snake_case | `ssh_connection.rs` |
| **Rust 구조체/타입** | PascalCase | `SshSession`, `WorksetProfile` |
| **Rust 함수** | snake_case | `connect_ssh()` |
| **TypeScript 함수** | camelCase | `formatBytes()` |
| **IPC Commands** | snake_case | `connect_ssh`, `list_worksets` |
| **IPC Events** | kebab + 변수 | `terminal-output-{session_id}` |
| **JSON 파일** | kebab-case | `my-project.json` |
| **기획 문서** | kebab-case | `market-research.md` |

---

## 상세 설명 (Detailed Explanation)

### Rust 모듈 (snake_case)

**규칙**: 소문자 + 언더스코어

**예시**:
- ✅ `ssh_connection.rs`
- ✅ `workset_store.rs`
- ❌ `SshConnection.rs` (PascalCase)
- ❌ `ssh-connection.rs` (kebab-case)

---

### Rust 구조체/타입 (PascalCase)

**규칙**: 각 단어 첫 글자 대문자

**예시**:
- ✅ `SshSession`
- ✅ `WorksetProfile`
- ✅ `GridLayout`
- ❌ `ssh_session` (snake_case)
- ❌ `worksetProfile` (camelCase)

---

### Rust 함수 (snake_case)

**규칙**: 소문자 + 언더스코어

**예시**:
- ✅ `connect_ssh()`
- ✅ `list_worksets()`
- ✅ `get_cpu_usage()`
- ❌ `connectSsh()` (camelCase)
- ❌ `ConnectSsh()` (PascalCase)

---

### TypeScript 함수 (camelCase)

**규칙**: 첫 단어 소문자, 이후 단어 첫 글자 대문자

**예시**:
- ✅ `formatBytes()`
- ✅ `renderTerminal()`
- ✅ `applyGridLayout()`
- ❌ `format_bytes()` (snake_case)
- ❌ `FormatBytes()` (PascalCase)

---

### IPC Commands (snake_case)

**규칙**: 소문자 + 언더스코어

**예시**:
- ✅ `connect_ssh`
- ✅ `list_worksets`
- ✅ `terminal_input`
- ❌ `connectSsh` (camelCase)
- ❌ `connect-ssh` (kebab-case)

**이유**: Rust 함수명과 일치 (Tauri Command는 Rust 함수로 정의)

---

### IPC Events (kebab-case + 변수)

**규칙**: 소문자 + 하이픈 + 중괄호 변수

**예시**:
- ✅ `terminal-output-{session_id}`
- ✅ `session-status-{session_id}`
- ✅ `resource-update`
- ❌ `terminal_output_{session_id}` (snake_case)
- ❌ `terminalOutput{sessionId}` (camelCase)

**이유**: 이벤트명은 문자열이므로 kebab-case 사용 (웹 표준)

---

### JSON 파일 (kebab-case)

**규칙**: 소문자 + 하이픈

**예시**:
- ✅ `my-project.json`
- ✅ `dev-cluster.json`
- ✅ `staging-env.json`
- ❌ `MyProject.json` (PascalCase)
- ❌ `my_project.json` (snake_case)

**구현**:
```rust
fn to_kebab_case(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}
```

---

### 기획 문서 (kebab-case)

**규칙**: 소문자 + 하이픈

**예시**:
- ✅ `market-research.md`
- ✅ `mvp-spec.md`
- ✅ `architecture.md`
- ❌ `MarketResearch.md` (PascalCase)
- ❌ `market_research.md` (snake_case)

---

## 예외 사항 (Exceptions)

| 항목 | 규칙 | 예시 |
|------|------|------|
| 상수 (Rust) | SCREAMING_SNAKE_CASE | `MAX_RETRIES`, `DEFAULT_PORT` |
| 환경 변수 | SCREAMING_SNAKE_CASE | `RUST_LOG`, `DATABASE_URL` |
| CSS 클래스 | kebab-case | `.terminal-container`, `.file-tree` |
| CSS 변수 | kebab-case | `--color-bg`, `--color-accent` |

---

## 관련 문서 (Related Documents)

- `.agents/divisions/backend.md` (2.1) — Backend 부문 규칙 (IPC Command 네이밍)
- `.agents/divisions/frontend.md` (2.2) — Frontend 부문 규칙 (TypeScript 네이밍)
- `.agents/guidelines/git-conventions.md` (4.3) — Git 브랜치/커밋 네이밍
