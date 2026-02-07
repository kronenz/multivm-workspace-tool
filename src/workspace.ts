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

let gridResizerObserver: ResizeObserver | null = null;

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
  installGridResizers(gridContainer, rows, cols);
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

function installGridResizers(gridContainer: HTMLElement, rows: number, cols: number): void {
  // Clear previous overlay if any.
  gridContainer.querySelectorAll('.grid-resizers').forEach((n) => n.remove());
  if (gridResizerObserver) {
    gridResizerObserver.disconnect();
    gridResizerObserver = null;
  }

  if (rows <= 1 && cols <= 1) return;

  const overlay = document.createElement('div');
  overlay.className = 'grid-resizers';
  gridContainer.appendChild(overlay);

  const rowFracs = Array.from({ length: rows }, () => 1);
  const colFracs = Array.from({ length: cols }, () => 1);

  const minPx = 140;
  const handleHalf = 5;

  function applyTemplates(): void {
    gridContainer.style.gridTemplateRows = rowFracs.map((f) => `${f}fr`).join(' ');
    gridContainer.style.gridTemplateColumns = colFracs.map((f) => `${f}fr`).join(' ');
  }

  function getMetrics(): {
    innerLeft: number;
    innerTop: number;
    innerWidth: number;
    innerHeight: number;
    gap: number;
    colWidths: number[];
    rowHeights: number[];
  } {
    const rect = gridContainer.getBoundingClientRect();
    const style = getComputedStyle(gridContainer);
    const padL = parseFloat(style.paddingLeft) || 0;
    const padR = parseFloat(style.paddingRight) || 0;
    const padT = parseFloat(style.paddingTop) || 0;
    const padB = parseFloat(style.paddingBottom) || 0;
    const gap = parseFloat(style.gap) || 0;

    const innerWidth = Math.max(0, rect.width - padL - padR);
    const innerHeight = Math.max(0, rect.height - padT - padB);

    const totalColFrac = colFracs.reduce((a, b) => a + b, 0);
    const totalRowFrac = rowFracs.reduce((a, b) => a + b, 0);

    const widthAvail = Math.max(0, innerWidth - gap * Math.max(0, cols - 1));
    const heightAvail = Math.max(0, innerHeight - gap * Math.max(0, rows - 1));

    const colWidths = colFracs.map((f) => (totalColFrac ? (widthAvail * f) / totalColFrac : 0));
    const rowHeights = rowFracs.map((f) => (totalRowFrac ? (heightAvail * f) / totalRowFrac : 0));

    return {
      innerLeft: padL,
      innerTop: padT,
      innerWidth,
      innerHeight,
      gap,
      colWidths,
      rowHeights,
    };
  }

  function layoutHandles(): void {
    const m = getMetrics();

    // Vertical handles
    let x = m.innerLeft;
    for (let i = 0; i < cols - 1; i++) {
      x += m.colWidths[i];
      const left = x + m.gap / 2 - handleHalf;
      const el = overlay.querySelector<HTMLElement>(`.grid-resizer.v[data-idx="${i}"]`);
      if (el) {
        el.style.left = `${left}px`;
      }
      x += m.gap;
    }

    // Horizontal handles
    let y = m.innerTop;
    for (let i = 0; i < rows - 1; i++) {
      y += m.rowHeights[i];
      const top = y + m.gap / 2 - handleHalf;
      const el = overlay.querySelector<HTMLElement>(`.grid-resizer.h[data-idx="${i}"]`);
      if (el) {
        el.style.top = `${top}px`;
      }
      y += m.gap;
    }
  }

  function setColPair(i: number, newLeftPx: number, newRightPx: number): void {
    const pairTotal = colFracs[i] + colFracs[i + 1];
    const sumPx = newLeftPx + newRightPx;
    if (sumPx <= 0 || pairTotal <= 0) return;
    const leftFrac = (newLeftPx / sumPx) * pairTotal;
    colFracs[i] = leftFrac;
    colFracs[i + 1] = pairTotal - leftFrac;
    applyTemplates();
    layoutHandles();
  }

  function setRowPair(i: number, newTopPx: number, newBottomPx: number): void {
    const pairTotal = rowFracs[i] + rowFracs[i + 1];
    const sumPx = newTopPx + newBottomPx;
    if (sumPx <= 0 || pairTotal <= 0) return;
    const topFrac = (newTopPx / sumPx) * pairTotal;
    rowFracs[i] = topFrac;
    rowFracs[i + 1] = pairTotal - topFrac;
    applyTemplates();
    layoutHandles();
  }

  // Build handles
  for (let i = 0; i < cols - 1; i++) {
    const handle = document.createElement('div');
    handle.className = 'grid-resizer v';
    handle.dataset.idx = String(i);
    handle.style.left = '0px';
    handle.addEventListener('pointerdown', (ev) => {
      ev.preventDefault();
      handle.setPointerCapture(ev.pointerId);
      const startX = ev.clientX;
      const m = getMetrics();
      const pairWidth = m.colWidths[i] + m.colWidths[i + 1];
      let leftPx = m.colWidths[i];

      const onMove = (e: PointerEvent) => {
        const dx = e.clientX - startX;
        const nextLeft = Math.min(Math.max(leftPx + dx, minPx), pairWidth - minPx);
        setColPair(i, nextLeft, pairWidth - nextLeft);
      };
      const onUp = () => {
        window.removeEventListener('pointermove', onMove);
        window.removeEventListener('pointerup', onUp);
      };

      window.addEventListener('pointermove', onMove);
      window.addEventListener('pointerup', onUp);
    });
    overlay.appendChild(handle);
  }

  for (let i = 0; i < rows - 1; i++) {
    const handle = document.createElement('div');
    handle.className = 'grid-resizer h';
    handle.dataset.idx = String(i);
    handle.style.top = '0px';
    handle.addEventListener('pointerdown', (ev) => {
      ev.preventDefault();
      handle.setPointerCapture(ev.pointerId);
      const startY = ev.clientY;
      const m = getMetrics();
      const pairHeight = m.rowHeights[i] + m.rowHeights[i + 1];
      let topPx = m.rowHeights[i];

      const onMove = (e: PointerEvent) => {
        const dy = e.clientY - startY;
        const nextTop = Math.min(Math.max(topPx + dy, minPx), pairHeight - minPx);
        setRowPair(i, nextTop, pairHeight - nextTop);
      };
      const onUp = () => {
        window.removeEventListener('pointermove', onMove);
        window.removeEventListener('pointerup', onUp);
      };

      window.addEventListener('pointermove', onMove);
      window.addEventListener('pointerup', onUp);
    });
    overlay.appendChild(handle);
  }

  applyTemplates();
  layoutHandles();

  gridResizerObserver = new ResizeObserver(() => {
    layoutHandles();
  });
  gridResizerObserver.observe(gridContainer);
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
  if (gridResizerObserver) {
    gridResizerObserver.disconnect();
    gridResizerObserver = null;
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
