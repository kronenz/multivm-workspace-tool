import { describe, expect, it, vi } from 'vitest';

import { renderMarkdownToHtml, scrollToAnchor } from '../markdown';

describe('markdown', () => {
  it('slugify: basic text -> slug conversion via heading ids', () => {
    const html = renderMarkdownToHtml('# Hello World');
    expect(html).toContain('<h1 id="hello-world">Hello World</h1>');
  });

  it('slugify: strips HTML tags when generating heading ids', () => {
    const html = renderMarkdownToHtml('# Hello <span>World</span>');
    expect(html).toContain('id="hello-world"');
    expect(html).toContain('Hello <span>World</span>');
  });

  it('slugify: handles empty string by falling back to section id', () => {
    const html = renderMarkdownToHtml('# <span></span>');
    expect(html).toContain('<h1 id="section">');
  });

  it('renderMarkdownToHtml: renders basic markdown (# heading -> <h1>)', () => {
    const html = renderMarkdownToHtml('# Title');
    expect(html).toContain('<h1');
    expect(html).toContain('>Title</h1>');
  });

  it('renderMarkdownToHtml: renders code blocks with hljs classes', () => {
    const html = renderMarkdownToHtml('```typescript\nconst x: number = 1\n```');
    expect(html).toContain('hljs');
    expect(html).toContain('language-typescript');
  });

  it('renderMarkdownToHtml: sanitizes <script> tags (removed, text preserved)', () => {
    const html = renderMarkdownToHtml('hi <script>alert("x")</script> bye');
    expect(html.toLowerCase()).not.toContain('<script');
    expect(html).toContain('hi');
    expect(html).toContain('alert("x")');
    expect(html).toContain('bye');
  });

  it('renderMarkdownToHtml: blocks javascript: URLs in <a> tags', () => {
    const html = renderMarkdownToHtml('[x](javascript:alert(1))');
    expect(html).toMatch(/<a\s+[^>]*href=\"#\"/);
    expect(html.toLowerCase()).not.toContain('javascript:');
  });

  it('renderMarkdownToHtml: allows safe tags (p, strong, em, a, img, table)', () => {
    const md = [
      '**bold** and *em* and [link](https://example.com)',
      '',
      '![alt](https://example.com/a.png)',
      '',
      '| a | b |',
      '|---|---|',
      '| 1 | 2 |',
    ].join('\n');
    const html = renderMarkdownToHtml(md);

    expect(html).toContain('<p>');
    expect(html).toContain('<strong>bold</strong>');
    expect(html).toContain('<em>em</em>');
    expect(html).toMatch(/<a\s+[^>]*href=\"https:\/\/example\.com\"/);
    expect(html).toMatch(/<img\s+[^>]*src=\"https:\/\/example\.com\/a\.png\"/);
    expect(html).toContain('<table>');
  });

  it('renderMarkdownToHtml: renders GFM tables', () => {
    const html = renderMarkdownToHtml('| a | b |\n|---|---|\n| 1 | 2 |\n');
    expect(html).toContain('<table>');
    expect(html).toContain('<thead>');
    expect(html).toContain('<tbody>');
  });

  it('cssEscape: escapes special characters via scrollToAnchor selector', () => {
    const container = document.createElement('div');
    const target = document.createElement('div');
    target.id = 'a:b.c';
    target.scrollIntoView = vi.fn();
    container.appendChild(target);

    scrollToAnchor(container, 'a:b.c');
    expect(target.scrollIntoView).toHaveBeenCalledTimes(1);
  });
});
