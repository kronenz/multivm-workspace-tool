import { marked } from 'marked';
import { markedHighlight } from 'marked-highlight';
import hljs from 'highlight.js/lib/core';
import 'highlight.js/styles/github-dark.css';

import bash from 'highlight.js/lib/languages/bash';
import json from 'highlight.js/lib/languages/json';
import javascript from 'highlight.js/lib/languages/javascript';
import typescript from 'highlight.js/lib/languages/typescript';
import python from 'highlight.js/lib/languages/python';
import rust from 'highlight.js/lib/languages/rust';
import yaml from 'highlight.js/lib/languages/yaml';
import css from 'highlight.js/lib/languages/css';
import xml from 'highlight.js/lib/languages/xml';
import shell from 'highlight.js/lib/languages/shell';
import markdown from 'highlight.js/lib/languages/markdown';

hljs.registerLanguage('bash', bash);
hljs.registerLanguage('json', json);
hljs.registerLanguage('javascript', javascript);
hljs.registerLanguage('typescript', typescript);
hljs.registerLanguage('python', python);
hljs.registerLanguage('rust', rust);
hljs.registerLanguage('yaml', yaml);
hljs.registerLanguage('css', css);
hljs.registerLanguage('xml', xml);
hljs.registerLanguage('html', xml);
hljs.registerLanguage('shell', shell);
hljs.registerLanguage('sh', shell);
hljs.registerLanguage('markdown', markdown);
hljs.registerLanguage('md', markdown);

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

const ALLOWED_TAGS = new Set([
  'h1',
  'h2',
  'h3',
  'h4',
  'h5',
  'h6',
  'p',
  'br',
  'hr',
  'blockquote',
  'ul',
  'ol',
  'li',
  'dl',
  'dt',
  'dd',
  'pre',
  'code',
  'em',
  'strong',
  'del',
  's',
  'a',
  'img',
  'table',
  'thead',
  'tbody',
  'tfoot',
  'tr',
  'th',
  'td',
  'sup',
  'sub',
  'span',
  'div',
  'details',
  'summary',
  'input', // for GFM task lists
]);

const ALLOWED_ATTRS: Record<string, Set<string>> = {
  '*': new Set(['id', 'class']),
  a: new Set(['href', 'title', 'target', 'rel']),
  img: new Set(['src', 'alt', 'title', 'width', 'height']),
  input: new Set(['type', 'checked', 'disabled']),
  td: new Set(['align']),
  th: new Set(['align']),
  code: new Set(['class']), // for highlight.js language classes
};

function sanitizeHtml(html: string): string {
  const doc = new DOMParser().parseFromString(html, 'text/html');
  sanitizeNode(doc.body);
  return doc.body.innerHTML;
}

function sanitizeNode(node: Node): void {
  const children = Array.from(node.childNodes);
  for (const child of children) {
    if (child.nodeType === Node.TEXT_NODE) continue;
    if (child.nodeType === Node.COMMENT_NODE) {
      child.remove();
      continue;
    }
    if (child.nodeType !== Node.ELEMENT_NODE) {
      child.remove();
      continue;
    }
    const el = child as Element;
    const tag = el.tagName.toLowerCase();
    if (!ALLOWED_TAGS.has(tag)) {
      // Replace with text content to preserve readable text
      const text = node.ownerDocument?.createTextNode(el.textContent ?? '') ??
        document.createTextNode(el.textContent ?? '');
      node.replaceChild(text, el);
      continue;
    }

    // Strip disallowed attributes
    const globalAllowed = ALLOWED_ATTRS['*'] ?? new Set();
    const tagAllowed = ALLOWED_ATTRS[tag] ?? new Set();
    for (const attr of Array.from(el.attributes)) {
      const name = attr.name.toLowerCase();
      if (!globalAllowed.has(name) && !tagAllowed.has(name)) {
        el.removeAttribute(attr.name);
      }
    }

    // Block javascript: URLs
    if (tag === 'a') {
      const href = el.getAttribute('href') ?? '';
      if (href.trim().toLowerCase().startsWith('javascript:')) {
        el.setAttribute('href', '#');
      }
    }
    if (tag === 'img') {
      const src = el.getAttribute('src') ?? '';
      if (src.trim().toLowerCase().startsWith('javascript:')) {
        el.removeAttribute('src');
      }
    }

    sanitizeNode(el);
  }
}

export function renderMarkdownToHtml(markdown: string): string {
  const raw = marked.parse(markdown) as string;
  return sanitizeHtml(raw);
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
