# 4.4 작업 시작 전 체크리스트

**인덱스**: `4.4`  
**계층**: Individual Contributor Guidelines (실무 가이드라인)

---

## 목적 (Purpose)

작업 시작 전 필수 확인 사항을 체크하여 실수를 방지한다.

---

## 체크리스트 (Checklist)

```
□ 해당 Feature의 Done Criteria를 읽었는가? (docs/qa/mvp-spec.md)
□ 소유 팀(3.x)이 맞는가? 다른 팀 파일을 건드리지 않는가?
□ Trust Boundary를 위반하지 않는가?
□ 관련 ADR을 확인했는가?
□ 기존 Feature 1-4를 깨뜨리지 않는가?
```

---

## 상세 설명 (Detailed Explanation)

### 1. Done Criteria 확인

**목적**: 구현 범위와 완료 기준을 명확히 한다.

**절차**:
1. `docs/qa/mvp-spec.md` 열기
2. 해당 Feature 섹션 찾기 (예: Feature 5 — File Browser)
3. Done Criteria 체크리스트 읽기
4. AC (Acceptance Criteria) 섹션 확인

**예시**:
```
Feature 5: File Browser
Done Criteria:
□ 디렉토리 트리 표시
□ 폴더 확장/축소
□ .md 파일 클릭 시 Markdown Viewer 열기
□ 파일 아이콘 표시
□ 경로 breadcrumb 표시
□ 새로고침 버튼
```

---

### 2. 소유 팀 확인

**목적**: 다른 팀의 파일을 수정하지 않는다.

**절차**:
1. `.agents/registry.json` 열기
2. `file_ownership` 섹션에서 수정할 파일 검색
3. 소유 팀 인덱스 확인
4. 자신의 팀 인덱스와 일치하는지 확인

**예시**:
```json
{
  "file_ownership": {
    "src-tauri/src/ssh/mod.rs": "3.1",
    "src/terminal.ts": "3.4"
  }
}
```

**질문**:
- 나는 3.1 SSH/Connection Team인가?
- `src-tauri/src/ssh/mod.rs`를 수정해도 되는가? → ✅ Yes
- `src/terminal.ts`를 수정해도 되는가? → ❌ No (3.4 Terminal Team 소유)

---

### 3. Trust Boundary 확인

**목적**: 시스템 접근 규칙을 위반하지 않는다.

**체크 항목**:
- Frontend에서 파일 시스템 직접 접근? → ❌ 금지
- Frontend에서 SSH 직접 연결? → ❌ 금지
- Frontend에서 OS Keystore 직접 접근? → ❌ 금지
- Backend에서 시스템 리소스 접근? → ✅ 허용

**참조**: `.agents/guidelines/code-rules.md` (4.1) — Trust Boundary 규칙

---

### 4. ADR 확인

**목적**: 아키텍처 결정을 위반하지 않는다.

**체크 항목**:
- ADR-001: Tauri 사용 (Electron 금지)
- ADR-002: xterm.js 사용 (WebGL 기본, Canvas 폴백)
- ADR-003: SSH는 Rust Core에서만

**절차**:
1. `docs/engineering/architecture.md` § ADR 섹션 읽기
2. 작업 내용이 ADR을 위반하지 않는지 확인

**예시**:
- "Frontend에서 SSH 연결 구현" → ❌ ADR-003 위반
- "xterm.js Canvas 렌더러만 사용" → ❌ ADR-002 위반 (WebGL 우선)

---

### 5. 회귀 방지 확인

**목적**: 기존 Feature 1-4를 깨뜨리지 않는다.

**체크 항목**:
- Feature 1 (Workset CRUD) 영향?
- Feature 2 (SSH Connection) 영향?
- Feature 3 (Terminal) 영향?
- Feature 4 (Grid Layout) 영향?

**절차**:
1. 수정할 파일이 Feature 1-4와 관련 있는지 확인
2. 관련 있다면 변경 후 해당 Feature 테스트 재실행
3. 테스트 실패 시 즉시 롤백

**예시**:
- `src-tauri/src/ssh/mod.rs` 수정 → Feature 2 영향 → 테스트 재실행 필요
- `src/grid.ts` 수정 → Feature 4 영향 → 테스트 재실행 필요

---

## 체크리스트 사용 예시 (Usage Example)

### 시나리오: Feature 5 (File Browser) 구현 시작

```
□ Done Criteria 확인
  → docs/qa/mvp-spec.md § Feature 5 읽음 ✅

□ 소유 팀 확인
  → 나는 3.1 SSH/Connection Team (Backend)
  → src-tauri/src/file_access/ 수정 예정
  → .agents/registry.json 확인: "src-tauri/src/file_access/": "3.1" ✅

□ Trust Boundary 확인
  → Backend에서 SFTP 사용 → ✅ 허용

□ ADR 확인
  → ADR-003: SSH는 Rust Core에서만 → ✅ 준수

□ 회귀 방지 확인
  → Feature 2 (SSH Connection) 영향 가능
  → 구현 후 Feature 2 테스트 재실행 예정 ✅
```

---

## 관련 문서 (Related Documents)

- `docs/qa/mvp-spec.md` — Done Criteria, AC 섹션
- `.agents/registry.json` — 파일 소유권 매핑
- `.agents/guidelines/code-rules.md` (4.1) — Trust Boundary 규칙
- `docs/engineering/architecture.md` — ADR 섹션
