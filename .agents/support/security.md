# 5.2 Security Support (보안 지원)

**인덱스**: `5.2`  
**계층**: Support (지원 계층)

---

## 책임 (Responsibilities)

- 보안 규칙 집행 (NFR-12, NFR-13)
- Tauri Capabilities 파일 검토
- CSP (Content Security Policy) 관리
- 의존성 보안 감사

---

## 보안 체크리스트 (Security Checklist)

| 체크 항목 | 근거 |
|----------|------|
| SSH 키 **내용**이 JSON에 없는가? | NFR-12 |
| 비밀번호가 OS Keystore에만 저장되는가? | NFR-13 |
| Capabilities에 필요 최소 권한만 정의했는가? | ADR-001 |
| CSP가 외부 리소스 접근을 차단하는가? | Tauri 보안 모델 |
| `.env`, 인증 파일이 `.gitignore`에 포함되는가? | 기본 보안 |

---

## 상세 설명 (Detailed Explanation)

### 1. SSH 키 내용 저장 금지 (NFR-12)

**원칙**: SSH 키 **경로만** 저장, 내용 저장 절대 금지

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
    "key_content": "-----BEGIN RSA PRIVATE KEY-----\nMIIEpAIBAAKCAQEA..." // ❌ 절대 금지
  }
}
```

**검증 방법**:
```bash
# JSON 파일에서 "BEGIN" 문자열 검색
grep -r "BEGIN.*PRIVATE KEY" ~/.config/multivm-workspace/worksets/
# 결과가 있으면 위반
```

---

### 2. 비밀번호는 OS Keystore만 (NFR-13)

**원칙**: 비밀번호를 JSON에 평문 저장 금지, OS Keystore만 사용

**OS별 Keystore**:
- **macOS**: Keychain
- **Linux**: Secret Service (libsecret)
- **Windows**: Credential Manager

**허용**:
```rust
use keyring::Entry;

// 저장
let entry = Entry::new("multivm-workspace", "my-project/vm1")?;
entry.set_password("secret123")?;

// 조회
let password = entry.get_password()?;
```

**금지**:
```json
{
  "auth": {
    "type": "password",
    "password": "secret123" // ❌ 절대 금지
  }
}
```

**검증 방법**:
```bash
# JSON 파일에서 "password" 필드 검색
grep -r '"password"' ~/.config/multivm-workspace/worksets/
# 결과가 있으면 위반 (keystore_key는 허용)
```

---

### 3. Tauri Capabilities 최소 권한 (ADR-001)

**원칙**: 필요한 권한만 정의

**예시** (`src-tauri/capabilities/default.json`):
```json
{
  "permissions": [
    "core:default",
    "shell:allow-execute",
    "fs:allow-read-text-file",
    "fs:allow-write-text-file"
  ]
}
```

**금지**:
```json
{
  "permissions": [
    "core:default",
    "shell:allow-all", // ❌ 과도한 권한
    "fs:allow-all"     // ❌ 과도한 권한
  ]
}
```

**검증 절차**:
1. `src-tauri/capabilities/` 파일 검토
2. `allow-all` 권한 사용 여부 확인
3. 각 권한의 필요성 검증

---

### 4. CSP (Content Security Policy)

**원칙**: 외부 리소스 접근 차단

**설정** (`src-tauri/tauri.conf.json`):
```json
{
  "security": {
    "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:;"
  }
}
```

**금지**:
```json
{
  "security": {
    "csp": "default-src *; script-src *;" // ❌ 모든 출처 허용
  }
}
```

**검증 방법**:
1. `tauri.conf.json` § security.csp 확인
2. `default-src 'self'` 포함 여부 확인
3. 외부 도메인 허용 여부 검토

---

### 5. `.gitignore`에 민감 파일 포함

**필수 항목**:
```gitignore
# 환경 변수
.env
.env.local

# 인증 파일
*.pem
*.key
*.crt
credentials.json

# OS Keystore 백업
keychain-backup/
```

**검증 방법**:
```bash
# .gitignore 파일 확인
cat .gitignore | grep -E "\.env|\.pem|\.key|credentials"
```

---

## 보안 감사 절차 (Security Audit Procedure)

### 코드 리뷰 시

```
□ SSH 키 내용이 JSON에 없는가?
□ 비밀번호가 OS Keystore에만 저장되는가?
□ Capabilities에 과도한 권한이 없는가?
□ CSP가 외부 리소스를 차단하는가?
□ 민감 파일이 .gitignore에 포함되는가?
```

### 의존성 추가 시

```
□ 라이선스가 MIT/Apache 2.0 호환인가?
□ 보안 취약점이 없는가? (cargo audit / npm audit)
□ 번들 크기 영향이 허용 범위인가? (<10MB)
```

---

## 보안 이슈 대응 (Security Issue Response)

### 발견 시

```
[발견자] → 5.2 Security Support 통보
  ↓
[5.2] 심각도 평가 (Critical / High / Medium / Low)
  ↓
[5.2] 1.2 Technical Director에게 보고
  ↓
[1.2] 최종 승인 (수정 / 롤백 / 예외 허용)
```

### 심각도 기준

| 심각도 | 예시 | 대응 시간 |
|--------|------|----------|
| Critical | SSH 키 평문 저장 | 즉시 (1시간 이내) |
| High | 비밀번호 JSON 저장 | 24시간 이내 |
| Medium | 과도한 Capabilities | 1주일 이내 |
| Low | .gitignore 누락 | 다음 릴리스 |

---

## 관련 문서 (Related Documents)

- `.agents/executive/technical-director.md` (1.2) — 보안 이슈 최종 승인
- `.agents/executive/escalation-matrix.md` (1.3) — 보안 이슈 에스컬레이션
- `.agents/guidelines/code-rules.md` (4.1) — No Secret in JSON 규칙
- `docs/engineering/architecture.md` — NFR-12, NFR-13
