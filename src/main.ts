import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Terminal } from "@xterm/xterm";
import { WebglAddon } from "@xterm/addon-webgl";
import { FitAddon } from "@xterm/addon-fit";
import "@xterm/xterm/css/xterm.css";
import { LatencyRecorder } from "./latency";

interface EchoResponse {
  ch: string;
  received_at_us: number;
}

interface FloodSummary {
  flood_id: number;
  lines: number;
  elapsed_ms: number;
}

const keystrokeLatency = new LatencyRecorder();
const floodLatency = new LatencyRecorder();

let term: Terminal;
let fitAddon: FitAddon;
let webglAddon: WebglAddon | null = null;
let rendererType = "canvas";

let lastKeydownAt: number | null = null;

let echoQueue: Promise<void> = Promise.resolve();

let outputBuffer = "";
let outputFlushScheduled = false;

let floodInProgress = false;
let floodStartAt = 0;
let floodRustSummary: FloodSummary | null = null;

let lastStatsUpdateAt = 0;
let statsUpdateScheduled = false;

function $(id: string): HTMLElement {
  const el = document.getElementById(id);
  if (!el) throw new Error(`Element #${id} not found`);
  return el;
}

function updateStats(): void {
  $("keystroke-stats").textContent = keystrokeLatency.summary();
  $("flood-stats").textContent = floodLatency.summary();
  $("renderer-info").textContent = rendererType;
  $("sample-count").textContent = String(keystrokeLatency.count);
}

function scheduleStatsUpdate(): void {
  const now = performance.now();
  if (now - lastStatsUpdateAt < 100) return;
  if (statsUpdateScheduled) return;
  statsUpdateScheduled = true;
  requestAnimationFrame(() => {
    statsUpdateScheduled = false;
    lastStatsUpdateAt = performance.now();
    updateStats();
  });
}

function scheduleOutputFlush(): void {
  if (outputFlushScheduled) return;
  outputFlushScheduled = true;
  requestAnimationFrame(() => {
    outputFlushScheduled = false;
    if (outputBuffer.length === 0) return;
    const payload = outputBuffer;
    outputBuffer = "";
    term.write(payload);
  });
}

function initTerminal(): void {
  term = new Terminal({
    cursorBlink: true,
    fontSize: 14,
    fontFamily: "monospace",
    theme: {
      background: "#1a1a2e",
      foreground: "#e0e0e0",
      cursor: "#00d4ff",
    },
    scrollback: 15000,
    convertEol: false,
  });

  fitAddon = new FitAddon();
  term.loadAddon(fitAddon);

  const container = $("terminal-container");
  term.open(container);

  container.addEventListener(
    "keydown",
    () => {
      lastKeydownAt = performance.now();
    },
    true
  );

  try {
    webglAddon = new WebglAddon();
    webglAddon.onContextLoss(() => {
      webglAddon?.dispose();
      webglAddon = null;
      rendererType = "canvas (webgl context lost)";
      updateStats();
    });
    term.loadAddon(webglAddon);
    rendererType = "webgl";
  } catch {
    rendererType = "canvas (webgl unavailable)";
  }

  fitAddon.fit();
  updateStats();

  term.writeln("SPIKE-1: Tauri + xterm.js Latency Prototype");
  term.writeln("--------------------------------------------");
  term.writeln("Type characters to measure keystroke->echo latency.");
  term.writeln("Use controls above to run flood/stream tests.");
  term.writeln("");
}

function wireKeystrokeEcho(): void {
  term.onData((data: string) => {
    const startedAt = lastKeydownAt ?? performance.now();
    lastKeydownAt = null;

    echoQueue = echoQueue
      .then(async () => {
        const resp: EchoResponse = await invoke("echo_key", { ch: data });

        await new Promise<void>((resolve) => {
          term.write(resp.ch, () => {
            requestAnimationFrame(() => {
              requestAnimationFrame(() => {
                resolve();
              });
            });
          });
        });

        const latencyMs = performance.now() - startedAt;
        keystrokeLatency.record(latencyMs);
        scheduleStatsUpdate();
      })
      .catch((err: unknown) => {
        term.writeln(`\r\n[echo error] ${String(err)}`);
      });
  });
}

async function wireTerminalOutput(): Promise<void> {
  await listen<string>("terminal-output", (event) => {
    outputBuffer += event.payload;
    if (outputBuffer.length > 64 * 1024) {
      scheduleOutputFlush();
    } else {
      scheduleOutputFlush();
    }
  });

  await listen<{ flood_id: number; lines: number; elapsed_ms: number }>(
    "terminal-flood-done",
    (event) => {
      if (!floodInProgress) return;
      floodInProgress = false;
      ( $("btn-flood") as HTMLButtonElement ).disabled = false;

      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          const totalMs = performance.now() - floodStartAt;
          floodLatency.record(totalMs);

          const rustMs = floodRustSummary?.elapsed_ms;
          const rustText = typeof rustMs === "number" ? rustMs.toFixed(1) : "n/a";
          term.writeln(
            `\r\nFlood complete: ${event.payload.lines} lines | ` +
              `frontend=${totalMs.toFixed(1)}ms | rust=${rustText}ms`
          );
          scheduleStatsUpdate();
        });
      });
    }
  );
}

function wireControls(): void {
  $("btn-flood").addEventListener("click", async () => {
    const lines = parseInt(($("flood-lines") as HTMLInputElement).value, 10) || 10000;
    keystrokeLatency.clear();
    floodLatency.clear();
    scheduleStatsUpdate();

    if (floodInProgress) return;
    floodInProgress = true;
    ( $("btn-flood") as HTMLButtonElement ).disabled = true;
    floodRustSummary = null;

    term.writeln(`\r\nStarting ${lines}-line flood...`);
    floodStartAt = performance.now();

    invoke<FloodSummary>("start_flood", { lines })
      .then((summary) => {
        floodRustSummary = summary;
      })
      .catch((err: unknown) => {
        floodInProgress = false;
        ( $("btn-flood") as HTMLButtonElement ).disabled = false;
        term.writeln(`\r\n[flood error] ${String(err)}`);
      });
  });

  $("btn-stream").addEventListener("click", async () => {
    const lines = parseInt(($("stream-lines") as HTMLInputElement).value, 10) || 100;
    const delayMsRaw = parseInt(($("stream-delay") as HTMLInputElement).value, 10);
    const delayMs = Number.isNaN(delayMsRaw) ? 50 : delayMsRaw;
    const delayUs = Math.max(0, delayMs) * 1000;

    term.writeln(`\r\nStarting stream: ${lines} lines, ${delayUs}us delay...`);
    await invoke("start_stream", { lines, delayUs });
    term.writeln("\r\nStream complete.");
  });

  $("btn-clear-stats").addEventListener("click", () => {
    keystrokeLatency.clear();
    floodLatency.clear();
    scheduleStatsUpdate();
  });

  $("btn-clear-terminal").addEventListener("click", () => {
    term.clear();
  });

  window.addEventListener("resize", () => {
    fitAddon.fit();
  });
}

window.addEventListener("DOMContentLoaded", async () => {
  initTerminal();
  wireKeystrokeEcho();
  await wireTerminalOutput();
  wireControls();
});
