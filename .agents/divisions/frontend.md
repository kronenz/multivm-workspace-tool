# 2.2 Frontend Division (프론트엔드 부문)

**인덱스**: `2.2`  
**계층**: Division (부문 계층)

---

## 관할 (Jurisdiction)

`src/` 전체 (TypeScript/JavaScript, CSS)

---

## 하위 팀 (Sub-Teams)

- **3.4** Terminal Team
- **3.5** Grid & Layout Team
- **3.6** UI Components Team

---

## 소유 파일 (Owned Files)

| 파일/디렉토리 | 소유 팀 | 상태 |
|--------------|---------|------|
| `src/main.ts` | 2.2 (직접) | ✅ |
| `src/styles.css` | 3.6 | ✅ |
| `src/terminal.ts` | 3.4 | ✅ |
| `src/workspace.ts` | 3.4 + 3.5 | ✅ |
| `src/grid.ts` | 3.5 | ✅ |
| `src/(file-browser)` | 3.6 | ⬜ 미구현 |
| `src/(markdown-viewer)` | 3.6 | ⬜ 미구현 |
| `src/(resource-monitor)` | 3.6 | ⬜ 미구현 |

---

## 부문 규칙 (Division Rules)

### 1. Frontend는 샌드박스

**절대 금지**:
- 시스템 리소스 직접 접근 (파일, SSH, OS Keystore)
- Node.js 모듈 사용 (`fs`, `child_process`, `net`)
- 직접 소켓 연결

**허용**:
- Tauri IPC (`invoke()`, `listen()`)만 사용

**위반 시**: 즉시 거부

---

### 2. 통신은 IPC만

**필수**:
- Backend 통신: `invoke()` (Command 호출)
- Backend 이벤트: `listen()` (Event 수신)

**금지**:
- 직접 HTTP fetch (외부 API 호출 금지)
- WebSocket 직접 연결

**예시**:
```typescript
import { invoke, listen } from '@tauri-apps/api/core';

// Command 호출
const worksets = await invoke<Workset[]>('list_worksets');

// Event 수신
await listen<string>('terminal-output-123', (event) => {
  terminal.write(event.payload);
});
```

---

### 3. 터미널 렌더러: WebGL 기본, Canvas 폴백

**근거**: ADR-002

**필수**:
- xterm.js WebGL 렌더러 우선 사용
- WebGL 미지원 환경에서 Canvas 폴백

**예시**:
```typescript
import { Terminal } from '@xterm/xterm';
import { WebglAddon } from '@xterm/addon-webgl';
import { CanvasAddon } from '@xterm/addon-canvas';

const term = new Terminal();
try {
  term.loadAddon(new WebglAddon());
} catch {
  term.loadAddon(new CanvasAddon());
}
```

---

### 4. 타입: TypeScript strict mode

**필수**:
- `tsconfig.json`에 `"strict": true`
- `any` 타입 금지 — 명시적 타입 정의
- `@ts-ignore` 금지

**예시**:
```typescript
// ❌ 금지
const data: any = await invoke('get_workset');

// ✅ 허용
interface Workset {
  id: string;
  name: string;
  vms: VmConfig[];
}
const data: Workset = await invoke<Workset>('get_workset');
```

---

### 5. 스타일: CSS 변수를 통한 테마 지원

**필수**:
- CSS 변수 `--color-*` 토큰 사용
- 다크/라이트 테마 전환 지원 (Feature 10)

**예시**:
```css
:root {
  --color-bg: #1e1e1e;
  --color-fg: #d4d4d4;
  --color-accent: #007acc;
}

[data-theme="light"] {
  --color-bg: #ffffff;
  --color-fg: #000000;
  --color-accent: #0066cc;
}

.terminal {
  background-color: var(--color-bg);
  color: var(--color-fg);
}
```

---

## 협업 프로토콜 (Collaboration Protocols)

### IPC 인터페이스 변경 시

**필수**: 2.1 Backend Division과 합의

**절차**:
1. Command/Event 스펙 작성 (이름, 파라미터, 반환값)
2. 2.1 + 2.2 양쪽 확인
3. Backend `capabilities/` 파일 업데이트
4. Frontend 호출 코드 업데이트
5. 양쪽 빌드 성공 확인 후 머지

---

## 관련 문서 (Related Documents)

- `docs/engineering/architecture.md` — Frontend 아키텍처
- `.agents/protocols/cross-team.md` (9.2) — 팀 간 협업
- `.agents/guidelines/naming-conventions.md` (4.2) — TypeScript 네이밍
