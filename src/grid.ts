/**
 * Grid Layout Engine
 *
 * Manages NxM pane grids and the layout-preset toolbar.
 * Does NOT contain terminal or connection logic — those are wired externally.
 */

export const GRID_PRESETS: Record<string, { rows: number; cols: number }> = {
  '1x1': { rows: 1, cols: 1 },
  '2x1': { rows: 2, cols: 1 },
  '2x2': { rows: 2, cols: 2 },
  '2x3': { rows: 2, cols: 3 },
  '3x3': { rows: 3, cols: 3 },
};

/**
 * Create an NxM CSS-grid inside `container` and return the pane elements.
 * Clears any previous content first.
 */
export function createGrid(
  container: HTMLElement,
  rows: number,
  cols: number,
): HTMLElement[] {
  container.innerHTML = '';
  container.style.gridTemplateRows = `repeat(${rows}, 1fr)`;
  container.style.gridTemplateColumns = `repeat(${cols}, 1fr)`;

  const panes: HTMLElement[] = [];
  const count = rows * cols;
  for (let i = 0; i < count; i++) {
    const pane = document.createElement('div');
    pane.className = 'grid-pane';
    pane.dataset.paneIndex = String(i);
    container.appendChild(pane);
    panes.push(pane);
  }
  return panes;
}

/**
 * Tear down the grid — remove all children and reset inline grid styles.
 */
export function destroyGrid(container: HTMLElement): void {
  container.innerHTML = '';
  container.style.gridTemplateRows = '';
  container.style.gridTemplateColumns = '';
}

/**
 * Render preset-selector buttons + a "Disconnect All" action into `container`.
 *
 * `activePreset` highlights the currently selected layout.
 * `onChange` fires when the user clicks a different preset.
 */
export function createLayoutToolbar(
  container: HTMLElement,
  activePreset: string,
  onChange: (preset: string) => void,
): void {
  container.innerHTML = '';

  for (const key of Object.keys(GRID_PRESETS)) {
    const btn = document.createElement('button');
    btn.className = `layout-toolbar-btn${key === activePreset ? ' active' : ''}`;
    btn.textContent = key;
    btn.addEventListener('click', () => {
      container
        .querySelectorAll('.layout-toolbar-btn')
        .forEach((b) => b.classList.remove('active'));
      btn.classList.add('active');
      onChange(key);
    });
    container.appendChild(btn);
  }

  const disconnectBtn = document.createElement('button');
  disconnectBtn.className = 'btn btn-danger btn-sm btn-disconnect-all';
  disconnectBtn.textContent = 'Disconnect All';
  disconnectBtn.id = 'btn-disconnect-all';
  container.appendChild(disconnectBtn);
}

/**
 * Mark `index`-th pane as active (highlighted border) and deactivate the rest.
 */
export function setActivePane(container: HTMLElement, index: number): void {
  container.querySelectorAll('.grid-pane').forEach((pane, i) => {
    if (i === index) {
      pane.classList.add('active');
    } else {
      pane.classList.remove('active');
    }
  });
}
