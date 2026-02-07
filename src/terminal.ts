import '@xterm/xterm/css/xterm.css';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { WebglAddon } from '@xterm/addon-webgl';

export interface TerminalInstance {
  terminal: Terminal;
  fitAddon: FitAddon;
  disposables: Array<{ dispose(): void }>;
}

export const TERMINAL_THEME = {
  background: '#0f0f1a',
  foreground: '#e0e0e0',
  cursor: '#00d4ff',
  cursorAccent: '#0f0f1a',
  selectionBackground: 'rgba(0, 212, 255, 0.3)',
  selectionForeground: '#ffffff',
  black: '#1a1a2e',
  red: '#ff4757',
  green: '#2ed573',
  yellow: '#ffa502',
  blue: '#3742fa',
  magenta: '#a55eea',
  cyan: '#00d4ff',
  white: '#e0e0e0',
  brightBlack: '#6a6a7a',
  brightRed: '#ff6b81',
  brightGreen: '#7bed9f',
  brightYellow: '#ffda79',
  brightBlue: '#70a1ff',
  brightMagenta: '#c56cf0',
  brightCyan: '#34e7e4',
  brightWhite: '#ffffff',
};

export function createTerminal(container: HTMLElement): TerminalInstance {
  const fitAddon = new FitAddon();

  const terminal = new Terminal({
    fontFamily: 'JetBrains Mono, Menlo, Monaco, Consolas, Courier New, monospace',
    fontSize: 14,
    cursorBlink: true,
    scrollback: 10000,
    allowProposedApi: true,
    theme: TERMINAL_THEME,
  });

  terminal.loadAddon(fitAddon);
  terminal.open(container);
  fitAddon.fit();

  // Try WebGL renderer, fall back to Canvas
  try {
    const webglAddon = new WebglAddon();
    webglAddon.onContextLoss(() => {
      webglAddon.dispose();
    });
    terminal.loadAddon(webglAddon);
  } catch (e) {
    console.warn('[Terminal] WebGL not available, using Canvas renderer', e);
  }

  return { terminal, fitAddon, disposables: [] };
}

export function destroyTerminal(instance: TerminalInstance): void {
  for (const d of instance.disposables) {
    d.dispose();
  }
  instance.disposables.length = 0;
  instance.fitAddon.dispose();
  instance.terminal.dispose();
}

export function writeToTerminal(instance: TerminalInstance, data: Uint8Array | string): void {
  instance.terminal.write(data);
}
