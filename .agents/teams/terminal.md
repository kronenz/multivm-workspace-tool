# 3.4 Terminal Team

**인덱스**: `3.4` | **부문**: 2.2 Frontend | **담당 기능**: F3

---

## 소유 컴포넌트 (Owned Components)

- Terminal Emulator UI

---

## 소유 파일 (Owned Files)

| 파일 경로 | 설명 | 상태 |
|----------|------|------|
| `src/terminal.ts` | xterm.js WebGL/Canvas 렌더러 (79줄) | ✅ |
| `src/workspace.ts` | Grid-Terminal 통합, OutputBuffer (206줄) | ✅ (3.5와 공동 소유) |

---

## 기술 가이드라인 (Technical Guidelines)

### 1. 렌더러: WebGL 기본, Canvas 폴백

**근거**: ADR-002

**구현**:
```typescript
import { Terminal } from '@xterm/xterm';
import { WebglAddon } from '@xterm/addon-webgl';
import { CanvasAddon } from '@xterm/addon-canvas';

const term = new Terminal();
try {
  term.loadAddon(new WebglAddon());
  console.log('Using WebGL renderer');
} catch (e) {
  console.warn('WebGL not supported, falling back to Canvas');
  term.loadAddon(new CanvasAddon());
}
```

---

### 2. 스크롤백: 10,000줄

**근거**: NFR-2

**설정**:
```typescript
const term = new Terminal({
  scrollback: 10000,
  // ... 기타 옵션
});
```

---

### 3. 색상: 256색 + truecolor (24-bit RGB)

**설정**:
```typescript
const term = new Terminal({
  allowTransparency: false,
  theme: {
    background: '#1e1e1e',
    foreground: '#d4d4d4',
    cursor: '#ffffff',
    // ... 256색 팔레트
  },
});
```

---

### 4. Copy/Paste 단축키

**플랫폼별**:
- **Linux/Windows**: Ctrl+Shift+C / Ctrl+Shift+V
- **macOS**: Cmd+C / Cmd+V

**구현**:
```typescript
term.attachCustomKeyEventHandler((event) => {
  const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
  const modifier = isMac ? event.metaKey : event.ctrlKey && event.shiftKey;
  
  if (modifier && event.key === 'c') {
    document.execCommand('copy');
    return false;
  }
  if (modifier && event.key === 'v') {
    navigator.clipboard.readText().then(text => term.paste(text));
    return false;
  }
  return true;
});
```

---

### 5. 대량 출력 최적화: OutputBuffer

**문제**: 초당 수천 줄 출력 시 렌더링 지연

**해결**: `requestAnimationFrame` 기반 배치 렌더링

**구현**:
```typescript
class OutputBuffer {
  private buffer: string[] = [];
  private rafId: number | null = null;
  
  constructor(private term: Terminal) {}
  
  write(data: string) {
    this.buffer.push(data);
    
    if (this.rafId === null) {
      this.rafId = requestAnimationFrame(() => {
        const chunk = this.buffer.join('');
        this.buffer = [];
        this.term.write(chunk);
        this.rafId = null;
      });
    }
  }
}
```

---

### 6. 리사이즈: FitAddon

**요구사항**: NFR-3 — 패인 리사이즈 <50ms 응답

**구현**:
```typescript
import { FitAddon } from '@xterm/addon-fit';

const fitAddon = new FitAddon();
term.loadAddon(fitAddon);

// ResizeObserver로 패인 크기 변경 감지
const resizeObserver = new ResizeObserver(() => {
  fitAddon.fit();
  
  // Backend에 터미널 크기 전달
  const { cols, rows } = term;
  invoke('terminal_resize', { sessionId, cols, rows });
});

resizeObserver.observe(terminalContainer);
```

---

## Done Criteria 참조

→ `docs/qa/mvp-spec.md` § Feature 3 (AC-3)

---

## 관련 문서 (Related Documents)

- `.agents/divisions/frontend.md` (2.2) — Frontend 부문 규칙
- `.agents/teams/grid-layout.md` (3.5) — Grid & Layout Team (workspace.ts 공동 소유)
- `docs/engineering/architecture.md` — Terminal Emulator 아키텍처
