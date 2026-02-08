import { describe, expect, it } from 'vitest';

import { GRID_PRESETS, createGrid, destroyGrid, setActivePane } from '../grid';

describe('grid', () => {
  it('GRID_PRESETS has correct presets (1x1, 2x1, 2x2, 2x3, 3x3)', () => {
    expect(GRID_PRESETS).toEqual({
      '1x1': { rows: 1, cols: 1 },
      '2x1': { rows: 2, cols: 1 },
      '2x2': { rows: 2, cols: 2 },
      '2x3': { rows: 2, cols: 3 },
      '3x3': { rows: 3, cols: 3 },
    });
  });

  it('createGrid returns correct number of elements (rows * cols)', () => {
    const container = document.createElement('div');
    const panes = createGrid(container, 2, 3);
    expect(panes).toHaveLength(6);
    expect(container.querySelectorAll('.grid-pane')).toHaveLength(6);
  });

  it('createGrid sets gridTemplateRows and gridTemplateColumns on container', () => {
    const container = document.createElement('div');
    createGrid(container, 2, 3);
    expect(container.style.gridTemplateRows).toBe('repeat(2, 1fr)');
    expect(container.style.gridTemplateColumns).toBe('repeat(3, 1fr)');
  });

  it('createGrid each pane has data-pane-index attribute', () => {
    const container = document.createElement('div');
    const panes = createGrid(container, 2, 2);

    expect(panes[0]?.dataset.paneIndex).toBe('0');
    expect(panes[1]?.dataset.paneIndex).toBe('1');
    expect(panes[2]?.dataset.paneIndex).toBe('2');
    expect(panes[3]?.dataset.paneIndex).toBe('3');
  });

  it('createGrid clears previous content', () => {
    const container = document.createElement('div');
    container.innerHTML = '<div class="old">old</div>';
    expect(container.querySelectorAll('.old')).toHaveLength(1);

    createGrid(container, 1, 1);
    expect(container.querySelectorAll('.old')).toHaveLength(0);
    expect(container.querySelectorAll('.grid-pane')).toHaveLength(1);
  });

  it('destroyGrid clears container', () => {
    const container = document.createElement('div');
    createGrid(container, 2, 2);
    expect(container.children).toHaveLength(4);

    destroyGrid(container);
    expect(container.children).toHaveLength(0);
    expect(container.style.gridTemplateRows).toBe('');
    expect(container.style.gridTemplateColumns).toBe('');
  });

  it("setActivePane adds 'active' class to correct pane", () => {
    const container = document.createElement('div');
    createGrid(container, 2, 2);

    setActivePane(container, 2);
    const panes = Array.from(container.querySelectorAll<HTMLElement>('.grid-pane'));
    expect(panes[2]?.classList.contains('active')).toBe(true);
  });

  it("setActivePane removes 'active' from others", () => {
    const container = document.createElement('div');
    createGrid(container, 2, 2);

    setActivePane(container, 0);
    setActivePane(container, 3);

    const panes = Array.from(container.querySelectorAll<HTMLElement>('.grid-pane'));
    expect(panes[3]?.classList.contains('active')).toBe(true);
    expect(panes[0]?.classList.contains('active')).toBe(false);
    expect(panes[1]?.classList.contains('active')).toBe(false);
    expect(panes[2]?.classList.contains('active')).toBe(false);
  });
});
