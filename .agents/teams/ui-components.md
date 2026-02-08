# 3.6 UI Components Team

**인덱스**: `3.6` | **부문**: 2.2 Frontend | **담당 기능**: F5, F6, F7, F10

---

## 소유 컴포넌트 (Owned Components)

- File Browser UI
- Markdown Viewer UI
- Resource Monitor UI
- Workset Manager UI

---

## 소유 파일 (Owned Files)

| 파일 경로 | 설명 | 상태 |
|----------|------|------|
| `src/styles.css` | 글로벌 스타일 (670줄) | ✅ |
| `src/(file-browser)` | 파일 브라우저 UI | ⬜ Feature 5 |
| `src/(markdown-viewer)` | Markdown 렌더러 | ⬜ Feature 6 |
| `src/(resource-monitor)` | 리소스 모니터 UI | ⬜ Feature 7 |

---

## 기술 가이드라인 (Technical Guidelines)

### 1. 다크 테마 기본 + 라이트 테마 토글

**근거**: Feature 10

**구현**:
```css
:root {
  --color-bg: #1e1e1e;
  --color-fg: #d4d4d4;
  --color-accent: #007acc;
  --color-border: #3e3e3e;
}

[data-theme="light"] {
  --color-bg: #ffffff;
  --color-fg: #000000;
  --color-accent: #0066cc;
  --color-border: #cccccc;
}
```

**토글**:
```typescript
function toggleTheme() {
  const root = document.documentElement;
  const currentTheme = root.getAttribute('data-theme');
  const newTheme = currentTheme === 'light' ? 'dark' : 'light';
  root.setAttribute('data-theme', newTheme);
  localStorage.setItem('theme', newTheme);
}
```

---

### 2. CSS 변수 `--color-*` 토큰으로 테마 전환

**토큰 목록**:
- `--color-bg`: 배경색
- `--color-fg`: 전경색 (텍스트)
- `--color-accent`: 강조색 (버튼, 링크)
- `--color-border`: 테두리색
- `--color-success`: 성공 상태 (녹색)
- `--color-warning`: 경고 상태 (노란색)
- `--color-error`: 에러 상태 (빨간색)

**사용 예시**:
```css
.button {
  background-color: var(--color-accent);
  color: var(--color-bg);
  border: 1px solid var(--color-border);
}

.button:hover {
  opacity: 0.8;
}
```

---

### 3. Markdown 렌더링: syntax highlighting

**라이브러리**: `marked` + `highlight.js`

**구현**:
```typescript
import { marked } from 'marked';
import hljs from 'highlight.js';

marked.setOptions({
  highlight: (code, lang) => {
    if (lang && hljs.getLanguage(lang)) {
      return hljs.highlight(code, { language: lang }).value;
    }
    return hljs.highlightAuto(code).value;
  },
});

async function renderMarkdown(filePath: string) {
  const content = await invoke<string>('read_file', { filePath });
  const html = marked(content);
  
  const viewer = document.getElementById('markdown-viewer');
  viewer.innerHTML = html;
}
```

---

### 4. 리소스 표시: 색상 코딩

**규칙**:
- **Green** (<50%): 정상
- **Yellow** (50-80%): 주의
- **Red** (>80%): 위험

**구현**:
```typescript
function getResourceColor(usage: number): string {
  if (usage < 50) return 'var(--color-success)';
  if (usage < 80) return 'var(--color-warning)';
  return 'var(--color-error)';
}

function renderResourceBar(usage: number) {
  const bar = document.createElement('div');
  bar.className = 'resource-bar';
  bar.style.width = `${usage}%`;
  bar.style.backgroundColor = getResourceColor(usage);
  return bar;
}
```

---

### 5. File Browser: 트리 뷰

**기능**:
- 폴더 확장/축소
- `.md` 클릭 시 Markdown Viewer로 전환

**구현**:
```typescript
interface FileNode {
  name: string;
  path: string;
  type: 'file' | 'directory';
  children?: FileNode[];
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
    } else if (node.name.endsWith('.md')) {
      item.classList.add('markdown-file');
      item.addEventListener('click', () => {
        renderMarkdown(node.path);
      });
    }
    
    tree.appendChild(item);
  });
  
  return tree;
}
```

---

## Done Criteria 참조

→ `docs/qa/mvp-spec.md` § Feature 5 (AC-5), Feature 6 (AC-6), Feature 7 (AC-7), Feature 10 (AC-10)

---

## 관련 문서 (Related Documents)

- `.agents/divisions/frontend.md` (2.2) — Frontend 부문 규칙
- `docs/engineering/architecture.md` — UI Components 아키텍처
