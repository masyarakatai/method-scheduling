// The page's share of the board redraw at 1,500 strips, timed under Node with no browser
// (decisions.md, entry 177). Reads the view the app sends after an edit, written by the
// Rust budget test:
//   METHOD_SCHEDULE_VIEW_DUMP=/tmp/board-view.json \
//     cargo test --release -p method-schedule-app budget_edit -- --ignored
//   METHOD_SCHEDULE_VIEW=/tmp/board-view.json node --test crates/method-schedule-app/page-tests/board-budget.mjs
//
// board.js runs as it does in the app (the row model, the lines in view, every strip's
// classes and cells) against a small stand-in for the DOM, so what is timed is the
// page's own JavaScript: parsing the view and drawing the board. The browser's real
// element creation, style and layout stay out; those are checked by hand in the page
// harness (`?big`).

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { cpSync, existsSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

/** Runs each timing this many times and keeps the median. */
const RUNS = 15;

/**
 * The budgets, in ms (entry 177): the draw takes the 5 ms target for the whole redraw,
 * though here it is the page's JavaScript alone; parsing the view and a key step get
 * about five to ten times their times on the owner's Mac.
 */
const BUDGET = { parse: 10, draw: 5, step: 2 };

const path = process.env.METHOD_SCHEDULE_VIEW;

/** The median of RUNS timings of `f`, after one run to warm up. */
function median(f) {
  f();
  const times = [];
  for (let i = 0; i < RUNS; i++) {
    const t = performance.now();
    f();
    times.push(performance.now() - t);
  }
  times.sort((a, b) => a - b);
  return times[RUNS >> 1];
}

function within(label, took, budget) {
  const ok = took <= budget;
  console.log(`budget ${label.padEnd(36)} ${took.toFixed(2).padStart(8)} ms of ${budget} ms${ok ? '' : '  OVER'}`);
  return ok;
}

// ---------------------------------------------------------------------------
// The stand-in DOM: what board.js and the modules it imports touch while drawing.

class FakeClassList {
  constructor(node) {
    this.node = node;
  }
  set() {
    return new Set(this.node.className.split(' ').filter(Boolean));
  }
  add(...c) {
    this.node.className = [...new Set([...this.set(), ...c])].join(' ');
  }
  remove(...c) {
    const s = this.set();
    for (const x of c) s.delete(x);
    this.node.className = [...s].join(' ');
  }
  toggle(c, on) {
    const has = this.set().has(c);
    const want = on === undefined ? !has : Boolean(on);
    if (want && !has) this.add(c);
    if (!want && has) this.remove(c);
    return want;
  }
  contains(c) {
    return this.set().has(c);
  }
}

class FakeNode {
  constructor(tag) {
    this.tagName = tag.toUpperCase();
    this.className = '';
    this.textContent = '';
    this.title = '';
    this.id = '';
    this.value = '';
    this.dataset = {};
    this.attributes = new Map();
    this.children = [];
    this.classList = new FakeClassList(this);
    this.scrollTop = 0;
    this.clientHeight = 900;
    this.clientTop = 0;
  }
  setAttribute(k, v) {
    this.attributes.set(k, String(v));
    if (k === 'id') this.id = String(v);
    if (k === 'value') this.value = String(v);
  }
  getAttribute(k) {
    return this.attributes.has(k) ? this.attributes.get(k) : null;
  }
  addEventListener() {}
  removeEventListener() {}
  append(...nodes) {
    this.children.push(...nodes);
  }
  replaceChildren(...nodes) {
    this.children = nodes;
  }
  getClientRects() {
    return [{ top: 0 }];
  }
  /** The scroller stays put; everything in it moves up as it scrolls. */
  getBoundingClientRect() {
    const top = this === scroller ? 0 : -scroller.scrollTop;
    return { top, bottom: top + this.clientHeight, left: 0, right: 1200 };
  }
  contains() {
    return true;
  }
  closest() {
    return null;
  }
  querySelector() {
    return null;
  }
  querySelectorAll() {
    return [];
  }
}

/** The board's scroller (the app's `.panels`). */
let scroller = null;

/** Every node under `n`, `n` included. */
function count(n) {
  return 1 + (n.children || []).reduce((a, c) => a + count(c), 0);
}

function installDom() {
  globalThis.CSSStyleSheet = class {
    replaceSync(css) {
      this.css = css;
    }
  };
  const body = new FakeNode('body');
  globalThis.document = {
    body,
    adoptedStyleSheets: [],
    activeElement: body,
    createElement: (tag) => new FakeNode(tag),
    createTextNode: (text) => ({ nodeType: 3, textContent: String(text), children: [] }),
    getElementById: () => null,
    querySelector: () => null,
    querySelectorAll: () => [],
    addEventListener() {},
    removeEventListener() {},
    elementFromPoint: () => null,
  };
  globalThis.window = {
    addEventListener() {},
    removeEventListener() {},
    localStorage: { getItem: () => null, setItem() {} },
  };
  if (typeof globalThis.navigator === 'undefined') globalThis.navigator = { platform: 'MacIntel', userAgent: 'node' };
}

/**
 * board.js and its imports, loaded as modules from a copy of ui/js beside a package.json
 * saying so (whatever Node makes of a `.js` file outside a package; entry 137).
 */
async function loadBoard() {
  const dir = mkdtempSync(join(tmpdir(), 'method-schedule-page-'));
  cpSync(fileURLToPath(new URL('../ui/js/', import.meta.url)), dir, { recursive: true });
  writeFileSync(join(dir, 'package.json'), '{ "type": "module" }\n');
  return import(pathToFileURL(join(dir, 'board.js')).href);
}

// ---------------------------------------------------------------------------

test('the board redraw at 1,500 strips stays within its budgets', async () => {
  assert.ok(path && existsSync(path), `${path || 'the view named by METHOD_SCHEDULE_VIEW'} is missing: write it with the Rust budget test (see the top of this file)`);
  const text = readFileSync(path, 'utf8');
  installDom();
  const { createBoard } = await loadBoard();
  const host = new FakeNode('div');
  host.id = 'board';
  scroller = new FakeNode('div');
  const actions = { onPick() {}, onOpen() {}, onStart() {}, onAutoBreaks() {}, onRemoveBreaks() {}, dragSet: () => [], onDrop() {} };
  const board = createBoard(host, scroller, actions);

  let view = null;
  const parse = median(() => (view = JSON.parse(text)));
  let rows = [];
  const draw = median(() => (rows = board.render(view.board, null, [], [])));
  // The board draws only what is in view: a few hundred nodes, not one per strip.
  const nodes = count(host);
  assert.ok(rows.length > 1500, `${rows.length} rows`);
  assert.ok(nodes < 2000, `${nodes} nodes drawn`);
  // A key step far down the board: select a row not drawn, which scrolls and draws it.
  let i = 0;
  let far = 0;
  const step = median(() => {
    i += 1;
    const r = rows[(i * 397) % rows.length];
    board.select(r.id, [r.id]);
    far = Math.max(far, scroller.scrollTop);
  });
  assert.ok(far > 10000, `scrolled to ${far} px`);
  console.log(`board view ${(text.length / 1024).toFixed(0)} KB, ${rows.length} rows, ${nodes} nodes drawn`);
  const ok = [within('page: parse the view', parse, BUDGET.parse), within('page: draw the board', draw, BUDGET.draw), within('page: select a far row', step, BUDGET.step)];
  assert.ok(ok.every(Boolean), 'over budget at 1,500 strips');
});
