import { Terminal } from '@xterm/xterm';
import { createGrid, setActivePane } from './grid.ts';
import { createTerminal, destroyTerminal, TerminalInstance } from './terminal.ts';

// ── Types ──

export interface PaneState {
  index: number;
  sessionId: string | null;
  terminal: TerminalInstance | null;
  container: HTMLElement;
  statusEl: HTMLElement | null;
  hostLabel: string;
  resizeObserver: ResizeObserver | null;
  outputBuffer: OutputBuffer | null;
}

// ── Output Buffer (rAF batching) ──

class OutputBuffer {
  private chunks: Uint8Array[] = [];
  private scheduled = false;

  constructor(private terminal: Terminal) {}

  write(data: Uint8Array): void {
    this.chunks.push(data);
    if (!this.scheduled) {
      this.scheduled = true;
      requestAnimationFrame(() => {
        const merged = this.mergeChunks();
        this.terminal.write(merged);
        this.chunks = [];
        this.scheduled = false;
      });
    }
  }

  private mergeChunks(): Uint8Array {
    if (this.chunks.length === 1) return this.chunks[0];
    const total = this.chunks.reduce((sum, c) => sum + c.length, 0);
    const merged = new Uint8Array(total);
    let offset = 0;
    for (const chunk of this.chunks) {
      merged.set(chunk, offset);
      offset += chunk.length;
    }
    return merged;
  }
}

// ── Active Pane Tracking ──

let activePaneIndex = 0;

export function getActivePaneIndex(): number {
  return activePaneIndex;
}

// ── Core Functions ──

export function createWorkspace(
  gridContainer: HTMLElement,
  rows: number,
  cols: number,
  connectionCount: number,
): PaneState[] {
  const paneElements = createGrid(gridContainer, rows, cols);
  const panes: PaneState[] = [];

  for (let i = 0; i < paneElements.length; i++) {
    const container = paneElements[i];
    const pane: PaneState = {
      index: i,
      sessionId: null,
      terminal: null,
      container,
      statusEl: null,
      hostLabel: '',
      resizeObserver: null,
      outputBuffer: null,
    };

    if (i < connectionCount) {
      container.addEventListener('click', () => {
        activePaneIndex = i;
        setActivePane(gridContainer, i);
        if (pane.terminal) {
          pane.terminal.terminal.focus();
        }
      });
    } else {
      container.classList.add('grid-pane-empty');
      container.textContent = 'No connection';
    }

    panes.push(pane);
  }

  if (panes.length > 0) {
    activePaneIndex = 0;
    setActivePane(gridContainer, 0);
  }

  return panes;
}

export function attachTerminal(pane: PaneState): void {
  if (pane.terminal) return;

  const termWrapper = document.createElement('div');
  termWrapper.style.position = 'absolute';
  termWrapper.style.top = '24px';
  termWrapper.style.left = '0';
  termWrapper.style.right = '0';
  termWrapper.style.bottom = '0';
  pane.container.appendChild(termWrapper);

  const statusBar = document.createElement('div');
  statusBar.className = 'pane-status-bar';
  statusBar.innerHTML = `
    <span class="pane-status-dot connecting"></span>
    <span class="pane-host-label">${escapeText(pane.hostLabel)}</span>
    <span class="pane-status-text"></span>
    <button class="btn-pane-reconnect" type="button" style="display:none;">Reconnect</button>
  `;
  pane.container.appendChild(statusBar);
  pane.statusEl = statusBar;

  const instance = createTerminal(termWrapper);
  pane.terminal = instance;
  pane.outputBuffer = new OutputBuffer(instance.terminal);

  let resizeTimer: ReturnType<typeof setTimeout> | null = null;
  const observer = new ResizeObserver(() => {
    if (resizeTimer) clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => {
      if (pane.terminal) {
        pane.terminal.fitAddon.fit();
      }
    }, 100);
  });
  observer.observe(pane.container);
  pane.resizeObserver = observer;
}

export function detachTerminal(pane: PaneState): void {
  if (pane.resizeObserver) {
    pane.resizeObserver.disconnect();
    pane.resizeObserver = null;
  }

  if (pane.terminal) {
    destroyTerminal(pane.terminal);
    pane.terminal = null;
  }

  pane.outputBuffer = null;
  pane.statusEl = null;

  while (pane.container.firstChild) {
    pane.container.removeChild(pane.container.firstChild);
  }
}

export function destroyWorkspace(panes: PaneState[]): void {
  for (const pane of panes) {
    detachTerminal(pane);
  }
  activePaneIndex = 0;
}

export function writeToPaneBuffer(pane: PaneState, data: Uint8Array): void {
  if (pane.outputBuffer) {
    pane.outputBuffer.write(data);
  }
}

// ── Status Management ──

export function updatePaneStatus(pane: PaneState, status: string, statusText?: string): void {
  if (!pane.statusEl) return;
  const dot = pane.statusEl.querySelector('.pane-status-dot');
  if (dot) {
    dot.className = 'pane-status-dot';
    const statusClass = typeof status === 'string' ? status.toLowerCase() : 'disconnected';
    dot.classList.add(statusClass);
  }

  const textEl = pane.statusEl.querySelector<HTMLElement>('.pane-status-text');
  if (textEl) {
    textEl.textContent = statusText ?? defaultStatusText(status);
  }

  const btn = pane.statusEl.querySelector<HTMLButtonElement>('.btn-pane-reconnect');
  if (btn) {
    const s = status.toLowerCase();
    const show = s === 'reconnect_failed' || s === 'error';
    btn.style.display = show ? '' : 'none';
  }
}

export function setPaneHostLabel(pane: PaneState, label: string): void {
  pane.hostLabel = label;
  if (pane.statusEl) {
    const labelEl = pane.statusEl.querySelector('.pane-host-label');
    if (labelEl) {
      labelEl.textContent = label;
    }
  }
}

// ── Utilities ──

function escapeText(str: string): string {
  const div = document.createElement('div');
  div.textContent = str;
  return div.innerHTML;
}

function defaultStatusText(status: string): string {
  const s = status.toLowerCase();
  if (s === 'connecting') return 'Connecting...';
  if (s === 'connected') return 'Connected';
  if (s === 'reconnecting') return 'Reconnecting...';
  if (s === 'reconnect_failed') return 'Connection lost. Click to reconnect manually.';
  if (s === 'disconnected') return 'Disconnected';
  if (s === 'error') return 'Error';
  return '';
}
