# 4.3 커밋 & 브랜치 규칙

**인덱스**: `4.3`  
**계층**: Individual Contributor Guidelines (실무 가이드라인)

---

## 목적 (Purpose)

Git 브랜치 네이밍, 커밋 메시지 형식, 커밋 단위를 표준화한다.

---

## 규칙 (Rules)

| 항목 | 규칙 |
|------|------|
| **브랜치 네이밍** | `feature/F{번호}-{설명}`, `fix/F{번호}-{설명}`, `spike/{이름}` |
| **커밋 메시지** | `feat(F7): add resource poller`, `fix(F2): handle auth timeout` |
| **커밋 단위** | 하나의 논리적 변경 = 하나의 커밋 (atomic commit) |
| **금지** | `--force push` to main, `--no-verify` |

---

## 브랜치 네이밍 (Branch Naming)

### 형식

```
{타입}/{Feature 번호}-{설명}
```

### 타입

| 타입 | 용도 | 예시 |
|------|------|------|
| `feature/` | 새 기능 구현 | `feature/F5-file-browser` |
| `fix/` | 버그 수정 | `fix/F2-ssh-timeout` |
| `spike/` | 기술 스파이크 | `spike/xterm-webgl` |
| `refactor/` | 리팩토링 | `refactor/ssh-connection-pool` |
| `docs/` | 문서 작업 | `docs/update-readme` |

### 예시

- ✅ `feature/F5-file-browser`
- ✅ `fix/F2-handle-auth-timeout`
- ✅ `spike/resource-polling-strategy`
- ❌ `feature-5` (설명 없음)
- ❌ `file-browser` (타입 없음)

---

## 커밋 메시지 (Commit Message)

### 형식

```
{타입}(F{번호}): {설명}
```

### 타입

| 타입 | 용도 | 예시 |
|------|------|------|
| `feat` | 새 기능 추가 | `feat(F7): add resource poller` |
| `fix` | 버그 수정 | `fix(F2): handle auth timeout` |
| `refactor` | 리팩토링 | `refactor(F3): optimize terminal rendering` |
| `docs` | 문서 변경 | `docs: update AGENTS.md` |
| `test` | 테스트 추가/수정 | `test(F1): add workset CRUD tests` |
| `chore` | 빌드/설정 변경 | `chore: update dependencies` |

### 예시

- ✅ `feat(F7): add resource poller with 5s interval`
- ✅ `fix(F2): handle SSH auth timeout gracefully`
- ✅ `refactor(F3): use OutputBuffer for terminal rendering`
- ❌ `add resource poller` (타입 없음)
- ❌ `feat: add feature` (Feature 번호 없음)

---

## 커밋 단위 (Commit Unit)

### 원칙: Atomic Commit

**하나의 논리적 변경 = 하나의 커밋**

### 좋은 예시

```bash
# 커밋 1: Backend IPC Command 추가
git add src-tauri/src/resource/mod.rs
git commit -m "feat(F7): add start_polling IPC command"

# 커밋 2: Frontend UI 추가
git add src/resource-monitor.ts
git commit -m "feat(F7): add resource monitor UI component"

# 커밋 3: 통합
git add src/workspace.ts
git commit -m "feat(F7): integrate resource monitor into workspace"
```

### 나쁜 예시

```bash
# ❌ 여러 변경을 하나의 커밋에
git add src-tauri/src/resource/ src/resource-monitor.ts src/workspace.ts
git commit -m "feat(F7): add resource monitoring"
```

---

## 금지 사항 (Forbidden Operations)

### 1. `--force push` to main

**절대 금지**:
```bash
git push --force origin main # ❌
```

**허용** (feature 브랜치만):
```bash
git push --force origin feature/F5-file-browser # ✅ (주의해서 사용)
```

---

### 2. `--no-verify`

**절대 금지**:
```bash
git commit --no-verify -m "feat(F7): add resource poller" # ❌
```

**이유**: pre-commit hook 우회 (린트, 테스트 건너뛰기)

---

## 커밋 메시지 템플릿 (Commit Message Template)

### 설정

```bash
git config commit.template .gitmessage
```

### `.gitmessage` 파일

```
# {타입}(F{번호}): {설명}
#
# 타입: feat, fix, refactor, docs, test, chore
# Feature 번호: F1-F10
#
# 예시:
# feat(F7): add resource poller with 5s interval
# fix(F2): handle SSH auth timeout gracefully
#
# 상세 설명 (선택):
# - 변경 이유
# - 구현 방법
# - 영향 범위
```

---

## 관련 문서 (Related Documents)

- `.agents/guidelines/naming-conventions.md` (4.2) — 네이밍 컨벤션
- `.agents/protocols/feature-implementation.md` (9.4) — 기능 구현 절차
