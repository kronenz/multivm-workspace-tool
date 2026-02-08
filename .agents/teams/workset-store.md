# 3.2 Workset Store Team

**인덱스**: `3.2` | **부문**: 2.1 Backend | **담당 기능**: F1

---

## 소유 컴포넌트 (Owned Components)

- Workset Store

---

## 소유 파일 (Owned Files)

| 파일 경로 | 설명 | 상태 |
|----------|------|------|
| `src-tauri/src/workset/mod.rs` | 데이터 모델 + JSON CRUD (420줄) | ✅ |

---

## IPC Commands 소유

| Command | 설명 | 상태 |
|---------|------|------|
| `list_worksets` | Workset 목록 조회 | ✅ |
| `get_workset` | 단건 조회 (ID 기준) | ✅ |
| `create_workset` | 새 Workset 생성 | ✅ |
| `update_workset` | 기존 Workset 수정 | ✅ |
| `delete_workset` | Workset 삭제 | ✅ |

---

## 기술 가이드라인 (Technical Guidelines)

### 1. 저장 경로

**위치**: `~/.config/multivm-workspace/worksets/`

**파일명**: kebab-case (예: `my-project.json`)

**예시**:
```
~/.config/multivm-workspace/worksets/
├── my-project.json
├── dev-cluster.json
└── staging-env.json
```

---

### 2. 포맷: JSON (serde_json)

**직렬화/역직렬화**:
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WorksetProfile {
    id: String,
    name: String,
    vms: Vec<VmConfig>,
    grid_layout: GridLayout,
    ai_cli_command: Option<String>,
}
```

---

### 3. 스키마 검증

**필수 필드**:
- `name`: Workset 이름
- `vms`: VM 설정 배열
  - `host`: SSH 호스트
  - `port`: SSH 포트
  - `user`: SSH 사용자
  - `auth`: 인증 정보

**검증 로직**:
```rust
fn validate_workset(profile: &WorksetProfile) -> Result<(), String> {
    if profile.name.is_empty() {
        return Err("name is required".to_string());
    }
    if profile.vms.is_empty() {
        return Err("at least one VM required".to_string());
    }
    for vm in &profile.vms {
        if vm.host.is_empty() {
            return Err("VM host is required".to_string());
        }
        // ... 추가 검증
    }
    Ok(())
}
```

---

### 4. 비밀번호는 OS Keystore만 사용

**근거**: NFR-13

**허용**: OS Keystore 참조
```json
{
  "auth": {
    "type": "password",
    "keystore_key": "multivm-workspace/my-project/vm1"
  }
}
```

**금지**: JSON에 평문 저장
```json
{
  "auth": {
    "type": "password",
    "password": "secret123" // ❌ 절대 금지
  }
}
```

**OS Keystore 사용**:
```rust
use keyring::Entry;

// 저장
let entry = Entry::new("multivm-workspace", "my-project/vm1")?;
entry.set_password("secret123")?;

// 조회
let password = entry.get_password()?;
```

---

### 5. 파일명: kebab-case

**규칙**: 소문자 + 하이픈

**예시**:
- ✅ `my-project.json`
- ✅ `dev-cluster.json`
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

## Done Criteria 참조

→ `docs/qa/mvp-spec.md` § Feature 1 (AC-1)

---

## 관련 문서 (Related Documents)

- `.agents/divisions/backend.md` (2.1) — Backend 부문 규칙
- `.agents/support/security.md` (5.2) — 보안 체크리스트 (NFR-13)
- `docs/engineering/architecture.md` — Workset Store 아키텍처
