// Loads the app's page in a plain browser with a fake window.__TAURI__ (fake-tauri.js),
// for checking the page without the app. Not part of the app: tauri.conf.json embeds
// only ui/. Serve crates/method-schedule-app over HTTP (ES modules don't load from file://):
//   python3 scripts/serve-harness.py   (caching off; plain http.server keeps stale modules)
// then open http://localhost:8765/harness/index.html

import './fake-tauri.js';

// ?big loads big-view.json (a real 1,500-strip view written by the ignored perf test in
// src/view.rs with METHOD_SCHEDULE_VIEW_DUMP set) and times the first draw.
if (new URLSearchParams(location.search).has('big')) {
  window.__bigView = await fetch('big-view.json', { cache: 'no-store' }).then((r) => r.json());
}
window.__t0 = performance.now();

const html = await fetch('../ui/index.html').then((r) => r.text());
const doc = new DOMParser().parseFromString(html, 'text/html');
document.body.replaceChildren(...[...doc.body.childNodes].map((n) => document.importNode(n, true)));
await import('../ui/js/main.js');
