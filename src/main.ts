import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { openUrl } from '@tauri-apps/plugin-opener';
import { createLayoutToolbar } from './grid.ts';
import type { PaneState } from './workspace.ts';
import { createWorkspace, attachTerminal, destroyWorkspace, getActivePaneIndex, setPaneHostLabel, writeToPaneBuffer, updatePaneStatus } from './workspace.ts';
import { FileBrowser, type FileEntry } from './file_browser.ts';
import { installMarkdownLinkHandler, renderMarkdownToHtml } from './markdown.ts';

interface WorksetSummary {
  id: string;
  name: string;
  connection_count: number;
  updated_at: string;
}

interface ConnectionConfig {
  host: string;
  port: number;
  user: string;
  auth_method: "key" | "password" | "ssh_config";
  key_path: string | null;
  project_path: string;
  ai_cli_command: string | null;
}

interface GridLayout {
  preset: string | null;
  rows: number;
  cols: number;
}

interface Workset {
  id: string;
  name: string;
  created_at: string;
  updated_at: string;
  connections: ConnectionConfig[];
  grid_layout: GridLayout;
}

interface CreateWorksetInput {
  name: string;
  connections: ConnectionConfig[];
  grid_layout: GridLayout;
}

interface UpdateWorksetInput {
  name?: string;
  connections?: ConnectionConfig[];
  grid_layout?: GridLayout;
}

interface SessionInfo {
  session_id: string;
  connection_index: number;
  host: string;
  status: string;
}

interface ReadFileResult {
  path: string;
  bytes: number[];
  truncated: boolean;
}

interface ResourceSnapshot {
  cpu_percent: number | null;
  ram_percent: number | null;
  disk_percent: number | null;
  ts_epoch: number;
  disk_path: string;
}

let selectedWorksetId: string | null = null;
let allSummaries: WorksetSummary[] = [];
let toastTimer: ReturnType<typeof setTimeout> | null = null;
let activeWorkspace: {
  worksetId: string;
  panes: PaneState[];
  sessionInfos: SessionInfo[];
  rootPaths: string[];
} | null = null;

let eventUnlisteners: UnlistenFn[] = [];

let resourceSnapshots = new Map<string, ResourceSnapshot>();

let panelOpen = false;
let panelMode: 'files' | 'docs' = 'files';
let fileBrowser: FileBrowser | null = null;
let fileAutoRefreshTimer: ReturnType<typeof setInterval> | null = null;
let mdAutoRefreshTimer: ReturnType<typeof setInterval> | null = null;
let panelSyncTimer: ReturnType<typeof setInterval> | null = null;
let mdState: { sessionId: string; path: string; lastText: string | null } | null = null;
let mdContentEl: HTMLElement | null = null;
let mdPathEl: HTMLElement | null = null;
let mdNoteEl: HTMLElement | null = null;
let panelInitialized = false;

function $(id: string): HTMLElement {
  const el = document.getElementById(id);
  if (!el) throw new Error(`Element #${id} not found`);
  return el;
}

function escapeHtml(str: string): string {
  const div = document.createElement("div");
  div.textContent = str;
  return div.innerHTML;
}

function formatDate(iso: string): string {
  try {
    const d = new Date(iso);
    const now = new Date();
    const diff = now.getTime() - d.getTime();
    const mins = Math.floor(diff / 60000);
    if (mins < 1) return "just now";
    if (mins < 60) return `${mins}m ago`;
    const hrs = Math.floor(mins / 60);
    if (hrs < 24) return `${hrs}h ago`;
    const days = Math.floor(hrs / 24);
    if (days < 30) return `${days}d ago`;
    return d.toLocaleDateString();
  } catch {
    return iso;
  }
}

function showToast(message: string, type: "success" | "error"): void {
  let toast = document.querySelector(".toast") as HTMLElement | null;
  if (!toast) {
    toast = document.createElement("div");
    toast.className = "toast";
    document.body.appendChild(toast);
  }
  if (toastTimer) clearTimeout(toastTimer);
  toast.textContent = message;
  toast.className = `toast toast-${type}`;
  requestAnimationFrame(() => {
    toast!.classList.add("visible");
  });
  toastTimer = setTimeout(() => {
    toast!.classList.remove("visible");
  }, 3000);
}

// ── Resource Bar (MVP Feature 7) ──

function clearResourceBar(): void {
  resourceSnapshots.clear();
  const bar = document.getElementById('resource-bar');
  if (bar) {
    bar.innerHTML = '';
  }
  const ws = document.getElementById('workspace-view');
  ws?.classList.remove('has-resources');
}

function renderResourceBar(): void {
  const bar = document.getElementById('resource-bar');
  if (!bar || !activeWorkspace) return;

  const sessions = activeWorkspace.sessionInfos
    .filter((s) => Boolean(s.session_id))
    .sort((a, b) => a.connection_index - b.connection_index);

  if (sessions.length === 0) {
    bar.innerHTML = '';
    $("workspace-view").classList.remove('has-resources');
    return;
  }

  $("workspace-view").classList.add('has-resources');

  const html = sessions
    .map((s) => {
      const snap = resourceSnapshots.get(s.session_id);
      return renderResourceItem(s.host, snap);
    })
    .join('');

  bar.innerHTML = html;
}

function renderResourceItem(host: string, snap: ResourceSnapshot | undefined): string {
  const cpu = snap?.cpu_percent ?? null;
  const ram = snap?.ram_percent ?? null;
  const disk = snap?.disk_percent ?? null;

  return `
    <div class="resource-item">
      <span class="resource-host">${escapeHtml(host)}</span>
      ${renderMetric('CPU', cpu)}
      ${renderMetric('RAM', ram)}
      ${renderMetric('DSK', disk)}
    </div>
  `;
}

function renderMetric(label: string, value: number | null): string {
  const cls = metricClass(value);
  const text = value === null || Number.isNaN(value) ? 'N/A' : `${Math.round(value)}%`;
  return `<span class="metric ${cls}"><span class="k">${label}</span><span class="v">${text}</span></span>`;
}

function metricClass(value: number | null): string {
  if (value === null || Number.isNaN(value)) return 'na';
  if (value < 50) return 'ok';
  if (value < 80) return 'warn';
  return 'crit';
}

// ── Workspace Side Panel (File Browser + Markdown Viewer) ──

function initWorkspacePanel(): void {
  if (panelInitialized) return;
  panelInitialized = true;

  const filesTab = $("btn-panel-files") as HTMLButtonElement;
  const docsTab = $("btn-panel-docs") as HTMLButtonElement;
  const closeBtn = $("btn-panel-close") as HTMLButtonElement;
  const refreshBtn = $("btn-panel-refresh") as HTMLButtonElement;
  const scrim = $("workspace-panel-scrim");

  const filesView = $("file-browser-view");
  const docsView = $("markdown-viewer-view");

  // Build markdown viewer shell once.
  docsView.innerHTML = `
    <div class="md-header">
      <div style="min-width:0">
        <div class="md-path" id="md-path"></div>
        <div class="md-note" id="md-note"></div>
      </div>
    </div>
    <div class="markdown-body" id="md-content"></div>
  `;
  mdContentEl = docsView.querySelector<HTMLElement>("#md-content");
  mdPathEl = docsView.querySelector<HTMLElement>("#md-path");
  mdNoteEl = docsView.querySelector<HTMLElement>("#md-note");

  if (mdContentEl) {
    installMarkdownLinkHandler(
      mdContentEl,
      async (url: string) => {
        await openUrl(url);
      },
      () => {
        showToast('Not supported', 'error');
      },
    );
  }

  fileBrowser = new FileBrowser(
    filesView,
    async (sessionId: string, path: string) => {
      return invoke<FileEntry[]>("list_directory", { sessionId, path });
    },
    (path: string) => {
      openMarkdownForActiveSession(path);
    },
    showToast,
  );

  filesTab.addEventListener('click', () => setPanelMode('files'));
  docsTab.addEventListener('click', () => setPanelMode('docs'));
  closeBtn.addEventListener('click', () => setPanelOpen(false));
  scrim.addEventListener('click', () => setPanelOpen(false));

  refreshBtn.addEventListener('click', () => {
    if (panelMode === 'files') {
      void fileBrowser?.refresh();
    } else {
      void refreshMarkdown(true);
    }
  });

  // Default state
  setPanelMode('files');
  setPanelOpen(false);
}

function setPanelMode(mode: 'files' | 'docs'): void {
  panelMode = mode;
  const filesTab = $("btn-panel-files");
  const docsTab = $("btn-panel-docs");
  const filesView = $("file-browser-view");
  const docsView = $("markdown-viewer-view");

  if (mode === 'files') {
    filesTab.classList.add('active');
    docsTab.classList.remove('active');
    filesView.classList.add('active');
    docsView.classList.remove('active');
  } else {
    docsTab.classList.add('active');
    filesTab.classList.remove('active');
    docsView.classList.add('active');
    filesView.classList.remove('active');
  }
}

function setPanelOpen(open: boolean): void {
  panelOpen = open;
  const wsView = $("workspace-view");
  if (open) {
    wsView.classList.add('panel-open');
    syncPanelToActivePane();
    startPanelTimers();
  } else {
    wsView.classList.remove('panel-open');
    stopPanelTimers();
  }
}

function togglePanel(): void {
  initWorkspacePanel();
  setPanelOpen(!panelOpen);
}

function startPanelTimers(): void {
  if (fileAutoRefreshTimer) clearInterval(fileAutoRefreshTimer);
  fileAutoRefreshTimer = setInterval(() => {
    if (!panelOpen) return;
    void fileBrowser?.refresh();
  }, 10000);

  if (mdAutoRefreshTimer) clearInterval(mdAutoRefreshTimer);
  mdAutoRefreshTimer = setInterval(() => {
    if (!panelOpen) return;
    if (!mdState) return;
    void refreshMarkdown(false);
  }, 5000);

  if (panelSyncTimer) clearInterval(panelSyncTimer);
  panelSyncTimer = setInterval(() => {
    if (!panelOpen) return;
    syncPanelToActivePane();
  }, 500);
}

function stopPanelTimers(): void {
  if (fileAutoRefreshTimer) {
    clearInterval(fileAutoRefreshTimer);
    fileAutoRefreshTimer = null;
  }
  if (mdAutoRefreshTimer) {
    clearInterval(mdAutoRefreshTimer);
    mdAutoRefreshTimer = null;
  }
  if (panelSyncTimer) {
    clearInterval(panelSyncTimer);
    panelSyncTimer = null;
  }
}

function getActiveSessionContext(): { sessionId: string | null; rootPath: string | null } {
  if (!activeWorkspace) return { sessionId: null, rootPath: null };
  const idx = getActivePaneIndex();
  const pane = activeWorkspace.panes[idx];
  const sessionId = pane?.sessionId ?? null;
  const rootPath = activeWorkspace.rootPaths[idx] ?? null;
  return { sessionId, rootPath };
}

function syncPanelToActivePane(): void {
  if (!panelOpen) return;
  if (!fileBrowser) return;
  const { sessionId, rootPath } = getActiveSessionContext();
  fileBrowser.setContext(sessionId, rootPath);
}

function openMarkdownForActiveSession(path: string): void {
  const { sessionId } = getActiveSessionContext();
  if (!sessionId) {
    showToast('No active session for this pane', 'error');
    return;
  }
  initWorkspacePanel();
  setPanelOpen(true);
  setPanelMode('docs');
  mdState = { sessionId, path, lastText: null };
  void refreshMarkdown(true);
}

async function refreshMarkdown(force: boolean): Promise<void> {
  if (!mdState || !mdContentEl || !mdPathEl || !mdNoteEl) {
    return;
  }

  try {
    const result = await invoke<ReadFileResult>('read_file', {
      sessionId: mdState.sessionId,
      path: mdState.path,
      maxBytes: 1024 * 1024,
    });

    const bytes = new Uint8Array(result.bytes);
    const text = new TextDecoder('utf-8').decode(bytes);

    mdPathEl.textContent = mdState.path;
    mdNoteEl.textContent = result.truncated
      ? 'Showing first 1 MiB (truncated) · Auto-refresh 5s'
      : 'Auto-refresh 5s';

    if (!force && mdState.lastText === text) return;
    mdState.lastText = text;

    mdContentEl.innerHTML = renderMarkdownToHtml(text);
  } catch (err) {
    showToast(`Failed to read file: ${String(err)}`, 'error');
  }
}

function insertPanelToggleButton(toolbarContainer: HTMLElement): void {
  if (toolbarContainer.querySelector('#btn-workspace-panel')) return;
  const btn = document.createElement('button');
  btn.id = 'btn-workspace-panel';
  btn.className = 'layout-toolbar-btn';
  btn.textContent = 'Panel';

  const disconnect = toolbarContainer.querySelector('#btn-disconnect-all');
  if (disconnect && disconnect.parentElement === toolbarContainer) {
    toolbarContainer.insertBefore(btn, disconnect);
  } else {
    toolbarContainer.appendChild(btn);
  }
}

const GRID_PRESETS: Record<string, { rows: number; cols: number }> = {
  "1x1": { rows: 1, cols: 1 },
  "2x1": { rows: 2, cols: 1 },
  "2x2": { rows: 2, cols: 2 },
  "2x3": { rows: 2, cols: 3 },
  "3x3": { rows: 3, cols: 3 },
};

async function loadWorksets(): Promise<void> {
  try {
    allSummaries = await invoke<WorksetSummary[]>("list_worksets");
    renderWorksetList(allSummaries);
  } catch (err) {
    showToast(`Failed to load worksets: ${String(err)}`, "error");
    allSummaries = [];
    renderWorksetList([]);
  }
}

async function selectWorkset(id: string): Promise<void> {
  try {
    const workset = await invoke<Workset>("get_workset", { id });
    selectedWorksetId = id;
    highlightSelectedCard();
    renderWorksetDetail(workset);
  } catch (err) {
    showToast(`Failed to load workset: ${String(err)}`, "error");
  }
}

async function saveWorkset(
  formEl: HTMLFormElement,
  editId: string | null
): Promise<void> {
  const data = extractFormData(formEl);
  if (!data) return;

  try {
    if (editId) {
      const input: UpdateWorksetInput = {
        name: data.name,
        connections: data.connections,
        grid_layout: data.grid_layout,
      };
      await invoke("update_workset", { id: editId, input });
      showToast("Workset updated", "success");
    } else {
      const input: CreateWorksetInput = {
        name: data.name,
        connections: data.connections,
        grid_layout: data.grid_layout,
      };
      await invoke("create_workset", { input });
      showToast("Workset created", "success");
    }
    await loadWorksets();
    if (editId) {
      await selectWorkset(editId);
    } else {
      showEmptyState();
    }
  } catch (err) {
    showToast(`Failed to save workset: ${String(err)}`, "error");
  }
}

async function deleteWorkset(id: string): Promise<void> {
  if (!window.confirm("Delete this workset? This cannot be undone.")) return;
  try {
    await invoke("delete_workset", { id });
    showToast("Workset deleted", "success");
    selectedWorksetId = null;
    await loadWorksets();
    showEmptyState();
  } catch (err) {
    showToast(`Failed to delete workset: ${String(err)}`, "error");
  }
}

function extractFormData(
  formEl: HTMLFormElement
): CreateWorksetInput | null {
  const nameInput = formEl.querySelector<HTMLInputElement>('[name="workset-name"]');
  const name = nameInput?.value.trim() ?? "";
  if (!name) {
    nameInput?.classList.add("form-input-error");
    showToast("Workset name is required", "error");
    return null;
  }
  nameInput?.classList.remove("form-input-error");

  const presetSelect = formEl.querySelector<HTMLSelectElement>('[name="grid-preset"]');
  const presetValue = presetSelect?.value ?? "custom";
  let rows: number;
  let cols: number;
  let preset: string | null;

  if (presetValue === "custom") {
    const rowsInput = formEl.querySelector<HTMLInputElement>('[name="grid-rows"]');
    const colsInput = formEl.querySelector<HTMLInputElement>('[name="grid-cols"]');
    rows = parseInt(rowsInput?.value ?? "1", 10) || 1;
    cols = parseInt(colsInput?.value ?? "1", 10) || 1;
    rows = Math.max(1, Math.min(rows, 10));
    cols = Math.max(1, Math.min(cols, 10));
    preset = null;
  } else {
    const p = GRID_PRESETS[presetValue];
    rows = p?.rows ?? 1;
    cols = p?.cols ?? 1;
    preset = presetValue;
  }

  const connectionCards = formEl.querySelectorAll<HTMLElement>(".connection-form-card");
  if (connectionCards.length === 0) {
    showToast("At least one connection is required", "error");
    return null;
  }

  const connections: ConnectionConfig[] = [];
  let hasError = false;

  connectionCards.forEach((card) => {
    const hostInput = card.querySelector<HTMLInputElement>('[name="conn-host"]');
    const portInput = card.querySelector<HTMLInputElement>('[name="conn-port"]');
    const userInput = card.querySelector<HTMLInputElement>('[name="conn-user"]');
    const authSelect = card.querySelector<HTMLSelectElement>('[name="conn-auth"]');
    const keyInput = card.querySelector<HTMLInputElement>('[name="conn-keypath"]');
    const projInput = card.querySelector<HTMLInputElement>('[name="conn-project"]');
    const aiInput = card.querySelector<HTMLInputElement>('[name="conn-ai-cmd"]');

    const host = hostInput?.value.trim() ?? "";
    const user = userInput?.value.trim() ?? "";
    const projectPath = projInput?.value.trim() ?? "";

    [hostInput, userInput, projInput].forEach((inp) => {
      inp?.classList.remove("form-input-error");
    });

    if (!host) { hostInput?.classList.add("form-input-error"); hasError = true; }
    if (!user) { userInput?.classList.add("form-input-error"); hasError = true; }
    if (!projectPath) { projInput?.classList.add("form-input-error"); hasError = true; }

    const authMethod = (authSelect?.value ?? "ssh_config") as ConnectionConfig["auth_method"];
    const keyPath = authMethod === "key" ? (keyInput?.value.trim() || null) : null;
    const aiCmd = aiInput?.value.trim() || null;
    const port = parseInt(portInput?.value ?? "22", 10) || 22;

    connections.push({
      host,
      port,
      user,
      auth_method: authMethod,
      key_path: keyPath,
      project_path: projectPath,
      ai_cli_command: aiCmd,
    });
  });

  if (hasError) {
    showToast("Fill in all required connection fields", "error");
    return null;
  }

  return {
    name,
    connections,
    grid_layout: { preset, rows, cols },
  };
}

function showWorkspaceView(): void {
  $("empty-state").style.display = "none";
  $("workset-detail").style.display = "none";
  $("workset-form").style.display = "none";
  const wsView = $("workspace-view");
  wsView.classList.add("active");
}

function hideWorkspaceView(): void {
  const wsView = $("workspace-view");
  wsView.classList.remove("active");
}

function showEmptyState(): void {
  hideWorkspaceView();
  $("empty-state").style.display = "";
  $("workset-detail").style.display = "none";
  $("workset-form").style.display = "none";
}

function showDetailView(): void {
  hideWorkspaceView();
  $("empty-state").style.display = "none";
  $("workset-detail").style.display = "";
  $("workset-form").style.display = "none";
}

function showFormView(): void {
  hideWorkspaceView();
  $("empty-state").style.display = "none";
  $("workset-detail").style.display = "none";
  $("workset-form").style.display = "";
}

function highlightSelectedCard(): void {
  const cards = document.querySelectorAll(".workset-card");
  cards.forEach((card) => {
    const el = card as HTMLElement;
    if (el.dataset.id === selectedWorksetId) {
      el.classList.add("selected");
    } else {
      el.classList.remove("selected");
    }
  });
}

function renderWorksetList(summaries: WorksetSummary[]): void {
  const container = $("workset-list");
  if (summaries.length === 0) {
    container.innerHTML = `<div class="sidebar-empty">No worksets yet.<br>Create one to get started.</div>`;
    return;
  }

  container.innerHTML = summaries
    .map(
      (ws) =>
        `<div class="workset-card${ws.id === selectedWorksetId ? " selected" : ""}" data-id="${escapeHtml(ws.id)}">
          <div class="workset-card-name">${escapeHtml(ws.name)}</div>
          <div class="workset-card-meta">
            <span>${ws.connection_count} connection${ws.connection_count !== 1 ? "s" : ""}</span>
            <span>${formatDate(ws.updated_at)}</span>
          </div>
        </div>`
    )
    .join("");

  container.querySelectorAll(".workset-card").forEach((card) => {
    card.addEventListener("click", () => {
      const id = (card as HTMLElement).dataset.id;
      if (id) selectWorkset(id);
    });
  });
}

function renderWorksetDetail(workset: Workset): void {
  const container = $("workset-detail");
  const gridLabel = workset.grid_layout.preset ?? `${workset.grid_layout.rows}x${workset.grid_layout.cols}`;

  let connectionsHtml = "";
  workset.connections.forEach((conn, i) => {
    const authLabel =
      conn.auth_method === "key"
        ? "SSH Key"
        : conn.auth_method === "password"
          ? "Password"
          : "SSH Config";

    connectionsHtml += `
      <div class="connection-detail-card">
        <div class="connection-detail-header">Connection ${i + 1}: ${escapeHtml(conn.user)}@${escapeHtml(conn.host)}:${conn.port}</div>
        <div class="connection-detail-row">
          <span><span class="connection-detail-label">Auth:</span> ${authLabel}</span>
          ${conn.key_path ? `<span><span class="connection-detail-label">Key:</span> ${escapeHtml(conn.key_path)}</span>` : ""}
          <span><span class="connection-detail-label">Path:</span> ${escapeHtml(conn.project_path)}</span>
          ${conn.ai_cli_command ? `<span><span class="connection-detail-label">AI CLI:</span> ${escapeHtml(conn.ai_cli_command)}</span>` : ""}
        </div>
      </div>`;
  });

  container.innerHTML = `
    <div class="detail-header">
      <div>
        <h2>${escapeHtml(workset.name)}</h2>
        <div class="detail-meta">
          <span>Created ${formatDate(workset.created_at)}</span>
          <span>Updated ${formatDate(workset.updated_at)}</span>
        </div>
      </div>
      <div class="detail-header-actions">
        <button class="btn btn-primary" id="btn-activate-workset">Activate</button>
        <button class="btn btn-ghost" id="btn-edit-workset">Edit</button>
        <button class="btn btn-danger" id="btn-delete-workset">Delete</button>
      </div>
    </div>
    <div class="detail-section">
      <div class="detail-section-title">Grid Layout</div>
      <div class="detail-grid-info">
        <strong>${escapeHtml(gridLabel)}</strong>
        <span>${workset.grid_layout.rows} rows, ${workset.grid_layout.cols} cols</span>
      </div>
    </div>
    <div class="detail-section">
      <div class="detail-section-title">Connections (${workset.connections.length})</div>
      ${connectionsHtml}
    </div>`;

  showDetailView();

  $("btn-edit-workset").addEventListener("click", () => {
    showEditForm(workset);
  });
  $("btn-delete-workset").addEventListener("click", () => {
    deleteWorkset(workset.id);
  });
  $("btn-activate-workset").addEventListener("click", () => {
    if (selectedWorksetId) {
      handleActivateWorkset(selectedWorksetId);
    }
  });
}

function renderConnectionFormCard(index: number, conn?: ConnectionConfig): string {
  const host = conn?.host ?? "";
  const port = conn?.port ?? 22;
  const user = conn?.user ?? "";
  const auth = conn?.auth_method ?? "ssh_config";
  const keyPath = conn?.key_path ?? "";
  const projPath = conn?.project_path ?? "";
  const aiCmd = conn?.ai_cli_command ?? "";
  const keyDisplay = auth === "key" ? "" : "display:none;";

  return `
    <div class="connection-form-card">
      <div class="connection-form-header">
        <span>Connection ${index + 1}</span>
        <button type="button" class="btn-ghost-danger btn-remove-conn">Remove</button>
      </div>
      <div class="form-row-3">
        <div class="form-group">
          <label class="form-label">Host *</label>
          <input type="text" name="conn-host" class="form-input" placeholder="192.168.1.100" value="${escapeHtml(host)}" />
        </div>
        <div class="form-group">
          <label class="form-label">Port</label>
          <input type="number" name="conn-port" class="form-input" min="1" max="65535" value="${port}" />
        </div>
        <div class="form-group">
          <label class="form-label">User *</label>
          <input type="text" name="conn-user" class="form-input" placeholder="ubuntu" value="${escapeHtml(user)}" />
        </div>
      </div>
      <div class="form-row">
        <div class="form-group">
          <label class="form-label">Auth Method</label>
          <select name="conn-auth" class="form-select">
            <option value="ssh_config"${auth === "ssh_config" ? " selected" : ""}>SSH Config</option>
            <option value="key"${auth === "key" ? " selected" : ""}>SSH Key</option>
            <option value="password"${auth === "password" ? " selected" : ""}>Password</option>
          </select>
        </div>
        <div class="form-group conn-keypath-group" style="${keyDisplay}">
          <label class="form-label">Key Path</label>
          <input type="text" name="conn-keypath" class="form-input" placeholder="~/.ssh/id_rsa" value="${escapeHtml(keyPath)}" />
          <div class="form-hint">Path to SSH private key file</div>
        </div>
      </div>
      <div class="form-row">
        <div class="form-group">
          <label class="form-label">Project Path *</label>
          <input type="text" name="conn-project" class="form-input" placeholder="/home/user/project" value="${escapeHtml(projPath)}" />
        </div>
        <div class="form-group">
          <label class="form-label">AI CLI Command</label>
          <input type="text" name="conn-ai-cmd" class="form-input" placeholder="claude" value="${escapeHtml(aiCmd)}" />
          <div class="form-hint">Auto-launched in project directory</div>
        </div>
      </div>
    </div>`;
}

function renderWorksetForm(workset?: Workset): void {
  const container = $("workset-form");
  const isEdit = !!workset;
  const title = isEdit ? "Edit Workset" : "Create Workset";
  const editId = workset?.id ?? "";

  const name = workset?.name ?? "";
  const preset = workset?.grid_layout.preset ?? "2x2";
  const rows = workset?.grid_layout.rows ?? 2;
  const cols = workset?.grid_layout.cols ?? 2;
  const connections = workset?.connections ?? [];

  const isCustom = !preset || !GRID_PRESETS[preset];

  let presetOptions = Object.keys(GRID_PRESETS)
    .map(
      (key) =>
        `<option value="${key}"${!isCustom && preset === key ? " selected" : ""}>${key}</option>`
    )
    .join("");
  presetOptions += `<option value="custom"${isCustom ? " selected" : ""}>Custom</option>`;

  const customDisplay = isCustom ? "" : "display:none;";
  const initialConnections: Array<ConnectionConfig | undefined> =
    connections.length > 0 ? connections : [undefined];
  const connectionsHtml = initialConnections
    .map((conn, i) => renderConnectionFormCard(i, conn))
    .join("");

  container.innerHTML = `
    <form id="workset-crud-form" data-edit-id="${escapeHtml(editId)}">
      <h3 class="form-title">${title}</h3>
      <div class="form-group">
        <label class="form-label">Workset Name *</label>
        <input type="text" name="workset-name" class="form-input" placeholder="My Development Setup" value="${escapeHtml(name)}" />
      </div>
      <div class="form-row">
        <div class="form-group">
          <label class="form-label">Grid Layout Preset</label>
          <select name="grid-preset" class="form-select">${presetOptions}</select>
        </div>
        <div class="form-group custom-grid-fields" style="${customDisplay}">
          <label class="form-label">Custom Grid</label>
          <div style="display:flex;gap:8px;">
            <input type="number" name="grid-rows" class="form-input" min="1" max="10" value="${rows}" placeholder="Rows" />
            <input type="number" name="grid-cols" class="form-input" min="1" max="10" value="${cols}" placeholder="Cols" />
          </div>
        </div>
      </div>
      <div class="connections-header">
        <div>
          <span class="connections-title">Connections</span>
          <span class="connections-count" id="conn-count">(${initialConnections.length}/10)</span>
        </div>
        <button type="button" class="btn btn-ghost btn-sm" id="btn-add-connection">+ Add Connection</button>
      </div>
      <div id="connections-container">${connectionsHtml}</div>
      <div class="form-actions">
        <button type="submit" class="btn btn-primary">${isEdit ? "Save Changes" : "Create Workset"}</button>
        <button type="button" class="btn btn-ghost" id="btn-cancel-form">Cancel</button>
      </div>
    </form>`;

  showFormView();
  wireFormEvents();
}

function showCreateForm(): void {
  selectedWorksetId = null;
  highlightSelectedCard();
  renderWorksetForm();
}

function showEditForm(workset: Workset): void {
  renderWorksetForm(workset);
}

function wireFormEvents(): void {
  const form = document.getElementById("workset-crud-form") as HTMLFormElement | null;
  if (!form) return;

  form.addEventListener("submit", (e: Event) => {
    e.preventDefault();
    const editId = form.dataset.editId || null;
    saveWorkset(form, editId || null);
  });

  const cancelBtn = document.getElementById("btn-cancel-form");
  cancelBtn?.addEventListener("click", () => {
    if (selectedWorksetId) {
      selectWorkset(selectedWorksetId);
    } else {
      showEmptyState();
    }
  });

  const presetSelect = form.querySelector<HTMLSelectElement>('[name="grid-preset"]');
  const customFields = form.querySelector<HTMLElement>(".custom-grid-fields");
  presetSelect?.addEventListener("change", () => {
    if (customFields) {
      customFields.style.display = presetSelect.value === "custom" ? "" : "none";
    }
  });

  const addBtn = document.getElementById("btn-add-connection");
  addBtn?.addEventListener("click", () => {
    const container = document.getElementById("connections-container");
    if (!container) return;
    const count = container.querySelectorAll(".connection-form-card").length;
    if (count >= 10) {
      showToast("Maximum 10 connections allowed", "error");
      return;
    }
    const temp = document.createElement("div");
    temp.innerHTML = renderConnectionFormCard(count);
    const card = temp.firstElementChild as HTMLElement;
    container.appendChild(card);
    wireConnectionCard(card);
    updateConnectionCount();
  });

  form.querySelectorAll<HTMLElement>(".connection-form-card").forEach((card) => {
    wireConnectionCard(card);
  });
}

function wireConnectionCard(card: HTMLElement): void {
  const removeBtn = card.querySelector<HTMLButtonElement>(".btn-remove-conn");
  removeBtn?.addEventListener("click", () => {
    const container = document.getElementById("connections-container");
    if (!container) return;
    const remaining = container.querySelectorAll(".connection-form-card").length;
    if (remaining <= 1) {
      showToast("At least one connection is required", "error");
      return;
    }
    card.remove();
    reindexConnections();
    updateConnectionCount();
  });

  const authSelect = card.querySelector<HTMLSelectElement>('[name="conn-auth"]');
  const keyGroup = card.querySelector<HTMLElement>(".conn-keypath-group");
  authSelect?.addEventListener("change", () => {
    if (keyGroup) {
      keyGroup.style.display = authSelect.value === "key" ? "" : "none";
    }
  });
}

function reindexConnections(): void {
  const container = document.getElementById("connections-container");
  if (!container) return;
  container.querySelectorAll<HTMLElement>(".connection-form-card").forEach((card, i) => {
    const header = card.querySelector(".connection-form-header span");
    if (header) header.textContent = `Connection ${i + 1}`;
  });
}

function updateConnectionCount(): void {
  const container = document.getElementById("connections-container");
  const countEl = document.getElementById("conn-count");
  if (!container || !countEl) return;
  const count = container.querySelectorAll(".connection-form-card").length;
  countEl.textContent = `(${count}/10)`;
}

async function cleanupWorkspace(): Promise<void> {
  // Unregister all event listeners
  for (const unlisten of eventUnlisteners) {
    unlisten();
  }
  eventUnlisteners = [];

  // Stop any workspace-panel timers and close panel.
  stopPanelTimers();
  mdState = null;
  panelOpen = false;
  const wsView = document.getElementById('workspace-view');
  wsView?.classList.remove('panel-open');

  // Deactivate SSH sessions
  try {
    await invoke("deactivate_workset");
  } catch (err) {
    console.error("Failed to deactivate:", err);
  }

  // Destroy workspace UI
  if (activeWorkspace) {
    destroyWorkspace(activeWorkspace.panes);
    activeWorkspace = null;
  }

  clearResourceBar();
}

async function handleActivateWorkset(worksetId: string): Promise<void> {
  try {
    const workset = await invoke<Workset>("get_workset", { id: worksetId });
    if (!workset) {
      showToast("Workset not found", "error");
      return;
    }

    // Collect passwords for password-auth connections
    const passwords: (string | null)[] = workset.connections.map((conn) => {
      if (conn.auth_method === "password") {
        const pw = window.prompt(`Password for ${conn.user}@${conn.host}:`);
        return pw;
      }
      return null;
    });

    // If any required password was cancelled, abort
    if (workset.connections.some((c, i) => c.auth_method === "password" && passwords[i] === null)) {
      showToast("Activation cancelled", "error");
      return;
    }

    // Clean up any existing workspace
    if (activeWorkspace) {
      await cleanupWorkspace();
    }

    // Switch to workspace view
    showWorkspaceView();

    // Determine grid layout
    const rows = workset.grid_layout.rows;
    const cols = workset.grid_layout.cols;
    const preset = workset.grid_layout.preset ?? `${rows}x${cols}`;

    // Create grid and panes
    const gridContainer = $("grid-container");
    const toolbarContainer = $("layout-toolbar");
    const panes = createWorkspace(gridContainer, rows, cols, workset.connections.length);

    // Render toolbar
    createLayoutToolbar(toolbarContainer, preset, (_newPreset) => {
      // Layout preset switching not implemented in this batch
    });

    insertPanelToggleButton(toolbarContainer);

    // Set host labels and attach terminals
    for (let i = 0; i < Math.min(panes.length, workset.connections.length); i++) {
      const conn = workset.connections[i];
      setPaneHostLabel(panes[i], `${conn.user}@${conn.host}:${conn.port}`);
      attachTerminal(panes[i]);
    }

    // Store workspace state
    activeWorkspace = {
      worksetId,
      panes,
      sessionInfos: [],
      rootPaths: workset.connections.map((c) => c.project_path),
    };

    const wsState = activeWorkspace;
    if (!wsState) throw new Error('workspace state missing');

    // Invoke SSH activation via Tauri IPC
    const sessions = await invoke<SessionInfo[]>("activate_workset", {
      worksetId,
      passwords,
    });

    wsState.sessionInfos = sessions;

    // Map session IDs to panes and wire event listeners
    for (const session of sessions) {
      const pane = panes[session.connection_index];
      if (!pane) continue;

      pane.sessionId = session.session_id;

      // Skip wiring for failed connections
      if (!session.session_id) {
        updatePaneStatus(pane, "error", session.status);
        continue;
      }

      // Terminal output → write to terminal via OutputBuffer
      const unlisten1 = await listen<number[]>(
        `terminal-output-${session.session_id}`,
        (event) => {
          const data = new Uint8Array(event.payload);
          writeToPaneBuffer(pane, data);
        }
      );
      eventUnlisteners.push(unlisten1);

      // Session status → update pane status dot
      const unlisten2 = await listen<
        string | { error: string } | { reconnecting: { attempt: number; max: number } }
      >(
        `session-status-${session.session_id}`,
        (event) => {
          const payload = event.payload;
          if (typeof payload === "string") {
            if (payload === 'reconnect_failed') {
              updatePaneStatus(
                pane,
                'reconnect_failed',
                'Connection lost. Click to reconnect manually.',
              );
            } else {
              updatePaneStatus(pane, payload);
            }
          } else if (payload && typeof payload === "object") {
            if ('error' in payload) {
              updatePaneStatus(pane, 'error', payload.error);
            } else if ('reconnecting' in payload) {
              const r = payload.reconnecting;
              updatePaneStatus(pane, 'reconnecting', `Reconnecting... (${r.attempt}/${r.max})`);
            }
          }
        }
      );
      eventUnlisteners.push(unlisten2);

      // Resource updates → update bottom resource bar
      const unlisten3 = await listen<ResourceSnapshot>(
        `resource-update-${session.session_id}`,
        (event) => {
          resourceSnapshots.set(session.session_id, event.payload);
          renderResourceBar();
        }
      );
      eventUnlisteners.push(unlisten3);

      // Terminal input → send to SSH via IPC
      if (pane.terminal) {
        const inputDisposable = pane.terminal.terminal.onData((data: string) => {
          invoke("terminal_input", { sessionId: session.session_id, data }).catch(
            console.error
          );
        });
        pane.terminal.disposables.push(inputDisposable);

        // Terminal resize → send to SSH via IPC
        const resizeDisposable = pane.terminal.terminal.onResize(
          ({ cols, rows }: { cols: number; rows: number }) => {
            invoke("terminal_resize", {
              sessionId: session.session_id,
              cols,
              rows,
            }).catch(console.error);
          }
        );
        pane.terminal.disposables.push(resizeDisposable);
      }
    }

    initWorkspacePanel();
    syncPanelToActivePane();
    renderResourceBar();

    showToast("Workspace activated", "success");
  } catch (err) {
    showToast(`Activation failed: ${String(err)}`, "error");
  }
}

async function handleDisconnectAll(): Promise<void> {
  if (!activeWorkspace) return;

  await cleanupWorkspace();
  hideWorkspaceView();

  if (selectedWorksetId) {
    selectWorkset(selectedWorksetId);
  } else {
    showEmptyState();
  }

  showToast("Disconnected all sessions", "success");
}

function wireSearch(): void {
  const searchInput = document.getElementById("workset-search") as HTMLInputElement | null;
  searchInput?.addEventListener("input", () => {
    const query = searchInput.value.trim().toLowerCase();
    if (!query) {
      renderWorksetList(allSummaries);
      return;
    }
    const filtered = allSummaries.filter((ws) =>
      ws.name.toLowerCase().includes(query)
    );
    renderWorksetList(filtered);
  });
}

window.addEventListener("DOMContentLoaded", () => {
  loadWorksets();
  wireSearch();

  getCurrentWindow().onCloseRequested(async () => {
    if (activeWorkspace) {
      await cleanupWorkspace();
    }
  });

  $("btn-new-workset").addEventListener("click", () => {
    showCreateForm();
  });

  const emptyCreateBtn = document.getElementById("btn-empty-create");
  emptyCreateBtn?.addEventListener("click", () => {
    showCreateForm();
  });

  document.addEventListener("click", (e) => {
    const target = e.target as HTMLElement;
    if (target.id === "btn-disconnect-all") {
      handleDisconnectAll();
    }
    if (target.id === 'btn-workspace-panel') {
      togglePanel();
    }

    const reconnectBtn = target.closest('.btn-pane-reconnect') as HTMLButtonElement | null;
    if (reconnectBtn) {
      const paneEl = reconnectBtn.closest('.grid-pane') as HTMLElement | null;
      const idx = paneEl?.dataset.paneIndex ? parseInt(paneEl.dataset.paneIndex, 10) : NaN;
      const pane = activeWorkspace && Number.isFinite(idx) ? activeWorkspace.panes[idx] : null;
      const sessionId = pane?.sessionId;
      if (pane && sessionId) {
        updatePaneStatus(pane, 'reconnecting', 'Reconnecting... (1/3)');
        invoke('terminal_reconnect', { sessionId }).catch((err) => {
          updatePaneStatus(pane, 'error', `Reconnect failed: ${String(err)}`);
        });
      }
    }

    // If the user changed active pane while panel is open, keep file browser in sync.
    if (panelOpen && target.closest('.grid-pane')) {
      setTimeout(() => syncPanelToActivePane(), 0);
    }
  });
});
