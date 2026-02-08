import { execSync, spawn } from 'node:child_process';
import net from 'node:net';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const repoRoot = path.resolve(__dirname, '..');
const tauriBinaryPath = path.resolve(repoRoot, 'src-tauri/target/debug/multivm-workspace-tool');

const capabilities: any[] = [
  {
    browserName: 'wry',
    'tauri:options': {
      application: tauriBinaryPath
    }
  }
];

let tauriDriverProcess: ReturnType<typeof spawn> | null = null;

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitForPortOpen(host: string, port: number, timeoutMs: number): Promise<void> {
  const startedAt = Date.now();
  // eslint-disable-next-line no-constant-condition
  while (true) {
    try {
      await new Promise<void>((resolve, reject) => {
        const socket = net
          .createConnection({ host, port }, () => {
            socket.end();
            resolve();
          })
          .on('error', reject);
      });
      return;
    } catch {
      if (Date.now() - startedAt > timeoutMs) {
        throw new Error(`Timed out waiting for ${host}:${port} to open`);
      }
      await sleep(100);
    }
  }
}

export const config = {
  runner: 'local',
  hostname: '127.0.0.1',
  port: 4444,

  specs: ['./specs/**/*.spec.ts'],
  maxInstances: 1,

  capabilities,

  logLevel: 'info',
  framework: 'mocha',
  reporters: ['spec'],

  mochaOpts: {
    ui: 'bdd',
    timeout: 60_000
  },

  autoCompileOpts: {
    autoCompile: true,
    tsNodeOpts: {
      project: path.resolve(__dirname, './tsconfig.json'),
      transpileOnly: true
    }
  },

  onPrepare: () => {
    // Build the debug binary used by tauri-driver.
    execSync('npm run tauri build -- --debug --no-bundle', {
      cwd: repoRoot,
      stdio: 'inherit'
    });
  },

  beforeSession: async () => {
    // Start tauri-driver so WebdriverIO can create a session.
    tauriDriverProcess = spawn('tauri-driver', ['--host', '127.0.0.1', '--port', '4444'], {
      stdio: 'inherit'
    });

    await waitForPortOpen('127.0.0.1', 4444, 10_000);
  },

  afterSession: async () => {
    if (!tauriDriverProcess) return;

    const proc = tauriDriverProcess;
    tauriDriverProcess = null;

    if (proc.killed) return;

    proc.kill('SIGTERM');
    await sleep(500);
    if (!proc.killed) proc.kill('SIGKILL');
  }
};
