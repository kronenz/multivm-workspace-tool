# 9.4 Feature Implementation Protocol (기능 구현 절차)

**인덱스**: `9.4`  
**계층**: Protocols (조직 횡단 프로토콜)

---

## 목적 (Purpose)

Feature 5–10 구현 시 표준 워크플로우.

---

## 5단계 워크플로우 (5-Phase Workflow)

```
Phase 1: 계획 (Plan)
  └─ 해당 Feature의 mvp-spec.md Done Criteria 숙지
  └─ 관련 Architecture 컴포넌트 확인
  └─ 필요 시 기술 스파이크 수행

Phase 2: 설계 (Design)
  └─ IPC 인터페이스 설계 (Command/Event 스펙)
  └─ BE + FE 동시 설계 (9.2 Cross-Team Protocol)
  └─ 1.2 Technical Director 아키텍처 리뷰

Phase 3: 구현 (Implement)
  └─ BE 팀(3.x)이 Rust Core 구현
  └─ FE 팀(3.x)이 UI 구현
  └─ 각 팀 자체 단위 테스트

Phase 4: 통합 (Integrate)
  └─ IPC 연결 (BE → FE)
  └─ E2E 테스트 (2.3 QA)
  └─ Done Criteria 체크리스트 하나씩 검증

Phase 5: 검증 (Verify)
  └─ AC 섹션 테스트 통과
  └─ NFR 충족 확인
  └─ 기존 Feature 회귀 없음 확인
  └─ 빌드 성공 (2.4 Ops)
```

---

## Phase 1: 계획 (Plan)

### 1.1 Done Criteria 숙지

**절차**:
1. `docs/qa/mvp-spec.md` 열기
2. 해당 Feature 섹션 찾기
3. Done Criteria 체크리스트 읽기
4. AC (Acceptance Criteria) 섹션 확인

**예시** (Feature 5: File Browser):
```
Done Criteria:
□ 디렉토리 트리 표시
□ 폴더 확장/축소
□ .md 파일 클릭 시 Markdown Viewer 열기
□ 파일 아이콘 표시
□ 경로 breadcrumb 표시
□ 새로고침 버튼
```

---

### 1.2 Architecture 컴포넌트 확인

**절차**:
1. `docs/engineering/architecture.md` 열기
2. 관련 컴포넌트 찾기 (예: File Access Layer)
3. 의존성 확인 (SSH Connection Manager)

---

### 1.3 기술 스파이크 (필요 시)

**수행 조건**:
- 기술적 불확실성이 높을 때
- 성능 검증이 필요할 때
- 새 라이브러리 도입 시

**예시**:
- SPIKE-3: OS별 리소스 수집 명령어 호환성 검증

---

## Phase 2: 설계 (Design)

### 2.1 IPC 인터페이스 설계

**형식**:
```markdown
## IPC Command: `list_directory`

**파라미터**:
- `vm_id`: string
- `path`: string

**반환값**:
- `Result<Vec<FileNode>, String>`

**FileNode**:
```typescript
interface FileNode {
  name: string;
  path: string;
  type: 'file' | 'directory';
  size?: number;
  modified?: string;
}
```

**에러**:
- "VM not found"
- "Permission denied"
```

---

### 2.2 BE + FE 동시 설계

**절차**:
1. Backend 팀(3.x)이 Rust 타입 정의
2. Frontend 팀(3.x)이 TypeScript 타입 정의
3. 양쪽 합의 (9.2 Cross-Team Protocol)

---

### 2.3 아키텍처 리뷰

**절차**:
1. 설계 문서 작성
2. 1.2 Technical Director 리뷰 요청
3. ADR 위반 여부 확인
4. NFR 영향 평가

---

## Phase 3: 구현 (Implement)

### 3.1 Backend 구현

**책임**: 3.1 SSH/Connection Team (Feature 5 예시)

**구현**:
```rust
// src-tauri/src/file_access/mod.rs
use ssh2::Session;

#[derive(Serialize)]
struct FileNode {
    name: String,
    path: String,
    #[serde(rename = "type")]
    node_type: String,
    size: Option<u64>,
    modified: Option<String>,
}

#[tauri::command]
async fn list_directory(vm_id: String, path: String) -> Result<Vec<FileNode>, String> {
    let session = get_session(&vm_id).await?;
    let sftp = session.sftp().map_err(|e| e.to_string())?;
    
    let entries = sftp.readdir(&Path::new(&path)).map_err(|e| e.to_string())?;
    
    let nodes = entries.into_iter().map(|(path, stat)| {
        FileNode {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            path: path.to_string_lossy().to_string(),
            node_type: if stat.is_dir() { "directory" } else { "file" }.to_string(),
            size: Some(stat.size.unwrap_or(0)),
            modified: None,
        }
    }).collect();
    
    Ok(nodes)
}
```

---

### 3.2 Frontend 구현

**책임**: 3.6 UI Components Team (Feature 5 예시)

**구현**:
```typescript
// src/file-browser.ts
import { invoke } from '@tauri-apps/api/core';

interface FileNode {
  name: string;
  path: string;
  type: 'file' | 'directory';
  size?: number;
  modified?: string;
}

async function renderFileTree(vmId: string, path: string) {
  const nodes = await invoke<FileNode[]>('list_directory', { vmId, path });
  
  const tree = document.createElement('ul');
  tree.className = 'file-tree';
  
  nodes.forEach(node => {
    const item = document.createElement('li');
    item.textContent = node.name;
    
    if (node.type === 'directory') {
      item.classList.add('folder');
      item.addEventListener('click', async () => {
        const children = await renderFileTree(vmId, node.path);
        item.appendChild(children);
      });
    }
    
    tree.appendChild(item);
  });
  
  return tree;
}
```

---

### 3.3 단위 테스트

**Backend**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_list_directory() {
        let result = list_directory("test-vm".to_string(), "/tmp".to_string()).await;
        assert!(result.is_ok());
    }
}
```

**Frontend**:
```typescript
// src/file-browser.test.ts
import { describe, it, expect } from 'vitest';

describe('renderFileTree', () => {
  it('should render directory tree', async () => {
    const tree = await renderFileTree('test-vm', '/tmp');
    expect(tree.children.length).toBeGreaterThan(0);
  });
});
```

---

## Phase 4: 통합 (Integrate)

### 4.1 IPC 연결

**절차**:
1. Backend `lib.rs`에 Command 등록
2. `capabilities/` 파일에 권한 추가
3. Frontend에서 `invoke()` 호출
4. 양쪽 빌드 성공 확인

---

### 4.2 E2E 테스트

**책임**: 2.3 QA Division

**시나리오** (Feature 5):
```
1. 앱 실행
2. Workset 활성화
3. File Browser 열기
4. 디렉토리 트리 표시 확인
5. 폴더 클릭 → 확장 확인
6. .md 파일 클릭 → Markdown Viewer 열림 확인
```

---

### 4.3 Done Criteria 검증

**절차**:
1. `docs/qa/mvp-spec.md` § Feature 5 Done Criteria 열기
2. 각 항목을 실제로 테스트
3. 모든 항목 통과 확인

---

## Phase 5: 검증 (Verify)

### 5.1 AC 섹션 테스트

**절차**:
1. `docs/qa/mvp-spec.md` § AC-5 열기
2. 6개 AC 항목 테스트
3. 모든 항목 통과 확인

---

### 5.2 NFR 충족 확인

**예시** (Feature 5):
- NFR-1: SSH 연결 지연 ≤2초 (로컬)
- NFR-12: SSH 키 경로만 저장

---

### 5.3 회귀 테스트

**절차**:
1. Feature 1-4 테스트 재실행
2. 모든 테스트 통과 확인
3. 실패 시 즉시 롤백

---

### 5.4 빌드 성공

**책임**: 2.4 Operations Division

**검증**:
```bash
npm run build
cargo build --release
npm run tauri build
```

---

## 완료 체크리스트 (Completion Checklist)

```
□ Phase 1: Done Criteria 숙지 완료
□ Phase 2: IPC 인터페이스 설계 완료, 아키텍처 리뷰 통과
□ Phase 3: BE + FE 구현 완료, 단위 테스트 통과
□ Phase 4: IPC 연결 완료, E2E 테스트 통과, Done Criteria 검증 완료
□ Phase 5: AC 테스트 통과, NFR 충족, 회귀 없음, 빌드 성공
```

---

## 관련 문서 (Related Documents)

- `docs/qa/mvp-spec.md` — Done Criteria, AC 섹션
- `.agents/protocols/cross-team.md` (9.2) — IPC 계약 변경 절차
- `.agents/divisions/qa.md` (2.3) — QA 검증 절차
- `.agents/divisions/operations.md` (2.4) — 빌드 검증
