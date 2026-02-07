export interface FileEntry {
  path: string;
  name: string;
  is_dir: boolean;
  size_bytes: number;
  mtime_epoch: number | null;
}

interface FileNode {
  path: string;
  name: string;
  isDir: boolean;
  sizeBytes: number;
  mtimeEpoch: number | null;
  expanded: boolean;
  loading: boolean;
  loaded: boolean;
  children: FileNode[];
}

export class FileBrowser {
  private sessionId: string | null = null;
  private rootPath: string | null = null;
  private root: FileNode | null = null;
  private selectedPath: string | null = null;
  private loadSeq = 0;

  constructor(
    private container: HTMLElement,
    private listDirectory: (sessionId: string, path: string) => Promise<FileEntry[]>,
    private onOpenMarkdown: (path: string) => void,
    private showToast: (message: string, type: 'success' | 'error') => void,
  ) {}

  setContext(sessionId: string | null, rootPath: string | null): void {
    if (!sessionId || !rootPath) {
      this.sessionId = null;
      this.rootPath = null;
      this.root = null;
      this.selectedPath = null;
      this.renderEmpty('No active session');
      return;
    }

    const changed = this.sessionId !== sessionId || this.rootPath !== rootPath;
    this.sessionId = sessionId;
    this.rootPath = rootPath;

    if (changed) {
      this.selectedPath = null;
      this.root = {
        path: rootPath,
        name: rootPath,
        isDir: true,
        sizeBytes: 0,
        mtimeEpoch: null,
        expanded: true,
        loading: false,
        loaded: false,
        children: [],
      };
      void this.refresh();
    }
  }

  async refresh(): Promise<void> {
    if (!this.sessionId || !this.root) {
      this.renderEmpty('No active session');
      return;
    }
    // Force reload only for root; expanded children are lazy.
    this.root.loaded = false;
    await this.ensureLoaded(this.root);
    this.render();
  }

  private async ensureLoaded(node: FileNode): Promise<void> {
    if (!this.sessionId) return;
    if (!node.isDir) return;
    if (node.loaded || node.loading) return;

    node.loading = true;
    const seq = ++this.loadSeq;
    this.render();

    try {
      const entries = await this.listDirectory(this.sessionId, node.path);
      if (seq !== this.loadSeq) return;

      const children: FileNode[] = entries.map((e) => ({
        path: e.path,
        name: e.name,
        isDir: e.is_dir,
        sizeBytes: e.size_bytes,
        mtimeEpoch: e.mtime_epoch,
        expanded: false,
        loading: false,
        loaded: false,
        children: [],
      }));

      children.sort((a, b) => {
        if (a.isDir !== b.isDir) return a.isDir ? -1 : 1;
        return a.name.localeCompare(b.name);
      });

      node.children = children;
      node.loaded = true;
    } catch (err) {
      this.showToast(`Failed to list directory: ${String(err)}`, 'error');
    } finally {
      node.loading = false;
    }
  }

  private toggleDir(path: string): void {
    const node = this.findNode(path);
    if (!node || !node.isDir) return;
    node.expanded = !node.expanded;
    if (node.expanded) {
      void this.ensureLoaded(node).then(() => this.render());
    }
    this.render();
  }

  private selectFile(path: string): void {
    this.selectedPath = path;
    const lower = path.toLowerCase();
    if (lower.endsWith('.md')) {
      this.onOpenMarkdown(path);
      return;
    }
    this.showToast('Read-only file browser. Use terminal to edit files.', 'error');
    this.render();
  }

  private findNode(path: string): FileNode | null {
    if (!this.root) return null;
    const stack: FileNode[] = [this.root];
    while (stack.length) {
      const n = stack.pop()!;
      if (n.path === path) return n;
      for (const c of n.children) stack.push(c);
    }
    return null;
  }

  private renderEmpty(message: string): void {
    this.container.innerHTML = `<div class="file-empty">${escapeHtml(message)}</div>`;
  }

  private render(): void {
    if (!this.root) {
      this.renderEmpty('No active session');
      return;
    }

    const tree = document.createElement('div');
    tree.className = 'file-tree';
    this.renderNode(tree, this.root, 0);

    this.container.innerHTML = '';
    this.container.appendChild(tree);
  }

  private renderNode(parent: HTMLElement, node: FileNode, depth: number): void {
    const row = document.createElement('div');
    row.className = `file-row${node.isDir ? ' is-dir' : ''}${this.selectedPath === node.path ? ' is-selected' : ''}`;
    row.style.paddingLeft = `${4 + depth * 14}px`;
    row.dataset.path = node.path;

    const icon = document.createElement('span');
    icon.className = 'file-icon';
    icon.textContent = node.isDir ? (node.expanded ? '▾' : '▸') : '•';
    row.appendChild(icon);

    const name = document.createElement('span');
    name.className = 'file-name';
    name.textContent = node.name;
    row.appendChild(name);

    const meta = document.createElement('span');
    meta.className = 'file-meta';
    meta.textContent = node.isDir
      ? node.loading
        ? 'loading...'
        : ''
      : `${formatBytes(node.sizeBytes)}${node.mtimeEpoch ? ` · ${formatMtime(node.mtimeEpoch)}` : ''}`;
    row.appendChild(meta);

    row.addEventListener('click', (e) => {
      e.preventDefault();
      if (node.isDir) {
        this.toggleDir(node.path);
      } else {
        this.selectFile(node.path);
      }
    });

    parent.appendChild(row);

    if (node.isDir && node.expanded) {
      for (const child of node.children) {
        this.renderNode(parent, child, depth + 1);
      }
    }
  }
}

function formatBytes(bytes: number): string {
  if (!bytes) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB'];
  let val = bytes;
  let idx = 0;
  while (val >= 1024 && idx < units.length - 1) {
    val /= 1024;
    idx++;
  }
  const fixed = idx === 0 ? String(Math.round(val)) : val.toFixed(1);
  return `${fixed} ${units[idx]}`;
}

function formatMtime(epoch: number): string {
  try {
    return new Date(epoch * 1000).toLocaleString();
  } catch {
    return '';
  }
}

function escapeHtml(str: string): string {
  const div = document.createElement('div');
  div.textContent = str;
  return div.innerHTML;
}
