# 9.2 Cross-Team Protocol (팀 간 협업)

**인덱스**: `9.2`  
**계층**: Protocols (조직 횡단 프로토콜)

---

## 목적 (Purpose)

다른 팀의 파일을 수정해야 할 때의 규칙.

---

## 시나리오별 처리 (Scenario Handling)

| 시나리오 | 프로토콜 |
|---------|---------|
| **IPC 인터페이스 변경** (Command 추가/수정) | 2.1(BE) + 2.2(FE) 양쪽 합의 필요 |
| **새 IPC Event 추가** | 발행 측(BE팀)이 정의 → 소비 측(FE팀) 확인 |
| **공유 파일 수정** (lib.rs, workspace.ts) | 소유 부문장(2.1 또는 2.2) 승인 |
| **보안 관련 변경** | 5.2 Security Support 리뷰 필수 |
| **NFR 영향 변경** | 2.3 QA 부문 사전 협의 |

---

## IPC 계약 변경 절차 (IPC Contract Change Procedure)

### 5단계 절차

#### 1. 변경 요청자가 Command/Event 스펙 작성

**형식**:
```markdown
## IPC Command: `list_directory`

**파라미터**:
- `vm_id`: string — VM 식별자
- `path`: string — 디렉토리 경로

**반환값**:
- `Result<Vec<FileNode>, String>`

**FileNode 구조**:
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
- "VM not found" — VM이 존재하지 않음
- "Permission denied" — 접근 권한 없음
```

---

#### 2. BE 부문(2.1) + FE 부문(2.2) 양쪽 확인

**Backend 확인 사항**:
- Rust 타입 정의 가능한가?
- 성능 영향은?
- 보안 이슈는?

**Frontend 확인 사항**:
- TypeScript 타입 정의 가능한가?
- UI 통합 가능한가?
- 에러 핸들링 가능한가?

**확인 방법**:
- 2.1 Backend Division 리뷰
- 2.2 Frontend Division 리뷰
- 양쪽 승인 시 다음 단계

---

#### 3. `capabilities/` 파일 업데이트 (BE)

**파일**: `src-tauri/capabilities/default.json`

**추가**:
```json
{
  "permissions": [
    "core:default",
    "shell:allow-execute",
    "fs:allow-read-dir" // 추가
  ]
}
```

**책임**: 2.1 Backend Division

---

#### 4. Frontend 호출 코드 업데이트 (FE)

**파일**: `src/file-browser.ts`

**추가**:
```typescript
import { invoke } from '@tauri-apps/api/core';

interface FileNode {
  name: string;
  path: string;
  type: 'file' | 'directory';
  size?: number;
  modified?: string;
}

async function listDirectory(vmId: string, path: string): Promise<FileNode[]> {
  try {
    return await invoke<FileNode[]>('list_directory', { vmId, path });
  } catch (error) {
    console.error('Failed to list directory:', error);
    throw error;
  }
}
```

**책임**: 2.2 Frontend Division

---

#### 5. 양쪽 빌드 성공 확인 후 머지

**검증**:
```bash
# Backend 빌드
cargo build --release

# Frontend 빌드
npm run build

# 통합 테스트
npm run tauri build
```

**책임**: 2.4 Operations Division

---

## 공유 파일 수정 (Shared File Modification)

### 공유 파일 목록

| 파일 | 소유 팀 | 수정 시 승인 필요 |
|------|---------|------------------|
| `src-tauri/src/lib.rs` | 2.1 (직접) | 2.1 Backend Division |
| `src/workspace.ts` | 3.4 + 3.5 | 2.2 Frontend Division |

### 수정 절차

```
[요청자] → 소유 부문장(2.1 또는 2.2) 승인 요청
  ↓
[부문장] 영향 범위 검토
  ├─ 단일 팀 영향 → 즉시 승인
  └─ 다중 팀 영향 → 관련 팀 모두 확인
  ↓
[요청자] 수정 진행
  ↓
[부문장] 빌드 성공 확인
```

---

## 보안 관련 변경 (Security-Related Changes)

### 필수 리뷰 항목

```
□ SSH 키 내용 저장 여부 (NFR-12)
□ 비밀번호 평문 저장 여부 (NFR-13)
□ Capabilities 권한 변경 여부
□ CSP 정책 변경 여부
□ .gitignore 민감 파일 포함 여부
```

### 절차

```
[요청자] → 5.2 Security Support 리뷰 요청
  ↓
[5.2] 보안 체크리스트 검증
  ├─ 통과 → 1.2 Technical Director 최종 승인
  └─ 실패 → 수정 요청
```

---

## NFR 영향 변경 (NFR-Impacting Changes)

### NFR 영향 예시

| 변경 | 영향 NFR |
|------|---------|
| 터미널 렌더링 로직 변경 | NFR-2 (10K줄 <100ms) |
| SSH 연결 로직 변경 | NFR-1 (≤2초/≤5초) |
| 패인 리사이즈 로직 변경 | NFR-3 (<50ms) |
| Auto-Reconnect 로직 변경 | NFR-8 (≥90%, 15초) |

### 절차

```
[요청자] → 2.3 QA Division 사전 협의
  ↓
[2.3] NFR 영향 평가
  ├─ 영향 없음 → 진행
  └─ 영향 있음 → 성능 테스트 계획 수립
  ↓
[요청자] 구현
  ↓
[2.3] NFR 검증 테스트
  ├─ 통과 → 머지
  └─ 실패 → 수정 요청
```

---

## 예시 시나리오 (Example Scenarios)

### 예시 1: IPC Command 추가

**요청**: Feature 5 (File Browser) — `list_directory` Command 추가

**절차**:
1. 3.1 SSH/Connection Team이 스펙 작성
2. 2.1 Backend + 2.2 Frontend 확인
3. 2.1이 `capabilities/` 업데이트
4. 2.2가 Frontend 호출 코드 추가
5. 2.4가 빌드 성공 확인

**소요 시간**: 1-2시간

---

### 예시 2: 공유 파일 수정

**요청**: `src/workspace.ts`에 리사이즈 로직 추가

**절차**:
1. 3.5 Grid & Layout Team이 2.2 Frontend Division 승인 요청
2. 2.2가 3.4 Terminal Team 영향 확인
3. 양쪽 확인 후 승인
4. 3.5가 수정 진행
5. 2.2가 빌드 성공 확인

**소요 시간**: 30분-1시간

---

## 관련 프로토콜 (Related Protocols)

- `.agents/protocols/quick-decision.md` (9.1) — 빠른 결정
- `.agents/protocols/communication.md` (9.5) — 커뮤니케이션 규칙
- `.agents/support/security.md` (5.2) — 보안 체크리스트
