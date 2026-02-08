# 3.5 Grid & Layout Team

**인덱스**: `3.5` | **부문**: 2.2 Frontend | **담당 기능**: F4

---

## 소유 컴포넌트 (Owned Components)

- Grid Layout Engine

---

## 소유 파일 (Owned Files)

| 파일 경로 | 설명 | 상태 |
|----------|------|------|
| `src/grid.ts` | CSS Grid, 5개 프리셋 (96줄) | ✅ |
| `src/workspace.ts` | ResizeObserver (206줄) | ✅ (3.4와 공동 소유) |

---

## 기술 가이드라인 (Technical Guidelines)

### 1. 5개 프리셋

| 프리셋 | 레이아웃 | 패인 수 |
|--------|---------|---------|
| 1x1 | 단일 패인 | 1 |
| 2x1 | 좌우 2분할 | 2 |
| 2x2 | 4분할 | 4 |
| 2x3 | 6분할 (2행 3열) | 6 |
| 3x2 | 6분할 (3행 2열) | 6 |

---

### 2. CSS Grid 기반 레이아웃

**구현**:
```typescript
function applyGridLayout(preset: GridPreset) {
  const container = document.getElementById('workspace-grid');
  
  switch (preset) {
    case '1x1':
      container.style.gridTemplateColumns = '1fr';
      container.style.gridTemplateRows = '1fr';
      break;
    case '2x1':
      container.style.gridTemplateColumns = '1fr 1fr';
      container.style.gridTemplateRows = '1fr';
      break;
    case '2x2':
      container.style.gridTemplateColumns = '1fr 1fr';
      container.style.gridTemplateRows = '1fr 1fr';
      break;
    case '2x3':
      container.style.gridTemplateColumns = 'repeat(3, 1fr)';
      container.style.gridTemplateRows = '1fr 1fr';
      break;
    case '3x2':
      container.style.gridTemplateColumns = '1fr 1fr';
      container.style.gridTemplateRows = 'repeat(3, 1fr)';
      break;
  }
}
```

---

### 3. 패인 리사이즈: <50ms 응답

**근거**: NFR-3

**구현**: ResizeObserver + debounce

```typescript
let resizeTimeout: number | null = null;

const resizeObserver = new ResizeObserver((entries) => {
  if (resizeTimeout) {
    clearTimeout(resizeTimeout);
  }
  
  resizeTimeout = setTimeout(() => {
    entries.forEach(entry => {
      const pane = entry.target as HTMLElement;
      const terminal = pane.querySelector('.terminal');
      
      if (terminal) {
        // 터미널 리사이즈 (3.4 Terminal Team)
        fitAddon.fit();
      }
    });
  }, 50); // 50ms 이내 응답
});

document.querySelectorAll('.pane').forEach(pane => {
  resizeObserver.observe(pane);
});
```

---

### 4. 각 패인에 콘텐츠 타입 할당

**지원 타입**:
- Terminal
- File Browser
- Markdown Viewer

**구현**:
```typescript
interface Pane {
  id: string;
  type: 'terminal' | 'file-browser' | 'markdown-viewer';
  vmId?: string; // Terminal인 경우
  filePath?: string; // Markdown Viewer인 경우
}

function renderPane(pane: Pane, container: HTMLElement) {
  switch (pane.type) {
    case 'terminal':
      renderTerminal(pane.vmId!, container);
      break;
    case 'file-browser':
      renderFileBrowser(container);
      break;
    case 'markdown-viewer':
      renderMarkdownViewer(pane.filePath!, container);
      break;
  }
}
```

---

### 5. 레이아웃 상태를 Workset JSON에 저장/복원

**저장**:
```typescript
interface WorksetProfile {
  id: string;
  name: string;
  vms: VmConfig[];
  grid_layout: {
    preset: GridPreset;
    panes: Pane[];
  };
}

async function saveWorkset(profile: WorksetProfile) {
  await invoke('update_workset', { profile });
}
```

**복원**:
```typescript
async function loadWorkset(id: string) {
  const profile = await invoke<WorksetProfile>('get_workset', { id });
  
  applyGridLayout(profile.grid_layout.preset);
  
  profile.grid_layout.panes.forEach((pane, index) => {
    const container = document.querySelector(`.pane:nth-child(${index + 1})`);
    renderPane(pane, container as HTMLElement);
  });
}
```

---

## Done Criteria 참조

→ `docs/qa/mvp-spec.md` § Feature 4 (AC-4)

---

## 관련 문서 (Related Documents)

- `.agents/divisions/frontend.md` (2.2) — Frontend 부문 규칙
- `.agents/teams/terminal.md` (3.4) — Terminal Team (workspace.ts 공동 소유)
- `docs/engineering/architecture.md` — Grid Layout Engine 아키텍처
