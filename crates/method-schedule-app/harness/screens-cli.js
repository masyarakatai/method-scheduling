// Draws /screens-data/cli.json (commands and their real output, from
// scripts/capture-screens.mjs) as a terminal. Text through textContent only.
const term = document.getElementById('term');
try {
  const runs = await fetch('/screens-data/cli.json', { cache: 'no-store' }).then((r) => r.json());
  runs.forEach((run, i) => {
    if (i) term.append(Object.assign(document.createElement('div'), { className: 'gap' }));
    term.append(Object.assign(document.createElement('div'), { className: 'cmd', textContent: run.command }));
    term.append(Object.assign(document.createElement('div'), { className: 'out', textContent: run.output.replace(/\n$/, '') }));
  });
  window.__screensReady = true;
} catch (e) {
  window.__screensFailed = String(e);
}
