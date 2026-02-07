import { marked } from 'marked';
import { markedHighlight } from 'marked-highlight';
import hljs from 'highlight.js/lib/core';

import bash from 'highlight.js/lib/languages/bash';
import json from 'highlight.js/lib/languages/json';
import javascript from 'highlight.js/lib/languages/javascript';
import typescript from 'highlight.js/lib/languages/typescript';
import python from 'highlight.js/lib/languages/python';

hljs.registerLanguage('bash', bash);
hljs.registerLanguage('json', json);
hljs.registerLanguage('javascript', javascript);
hljs.registerLanguage('typescript', typescript);
hljs.registerLanguage('python', python);

marked.use(
  markedHighlight({
    langPrefix: 'hljs language-',
    highlight(code: string, lang: string | undefined) {
      const language = lang && hljs.getLanguage(lang) ? lang : 'plaintext';
      return hljs.highlight(code, { language }).value;
    },
  })
);

interface MarkedRendererThis {
  parser: {
    parseInline(tokens: unknown[]): string;
  };
}

function slugify(text: string): string {
  return text
    .replace(/<[^>]+>/g, '')
    .trim()
    .toLowerCase()
    .replace(/[^\w]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

const renderer = {
  heading(this: MarkedRendererThis, token: { tokens: unknown[]; depth: number }) {
    const text = this.parser.parseInline(token.tokens);
    const id = slugify(text) || 'section';
    return `<h${token.depth} id="${id}">${text}</h${token.depth}>`;
  },
};

marked.use({ renderer });
marked.use({ gfm: true, breaks: true });

export function renderMarkdownToHtml(markdown: string): string {
  return marked.parse(markdown) as string;
}

export function installMarkdownLinkHandler(
  container: HTMLElement,
  openExternalUrl: (url: string) => Promise<void>,
  showNotSupported: () => void,
): void {
  container.addEventListener('click', (event) => {
    const target = event.target as HTMLElement;
    const link = target.closest('a') as HTMLAnchorElement | null;
    if (!link) return;

    const href = link.getAttribute('href');
    if (!href) return;

    event.preventDefault();

    if (href.startsWith('http://') || href.startsWith('https://')) {
      openExternalUrl(href).catch(() => {
        // Ignore; caller will toast/log.
      });
      return;
    }

    if (href.startsWith('#')) {
      scrollToAnchor(container, href.slice(1));
      return;
    }

    showNotSupported();
  });
}

export function scrollToAnchor(container: HTMLElement, id: string): void {
  if (!id) return;
  const el = container.querySelector<HTMLElement>(`#${cssEscape(id)}`);
  el?.scrollIntoView({ behavior: 'smooth', block: 'start' });
}

function cssEscape(id: string): string {
  // Minimal escape suitable for ids produced by marked.
  return id.replace(/[^a-zA-Z0-9_-]/g, (m) => `\\${m}`);
}
