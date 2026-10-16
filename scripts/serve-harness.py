#!/usr/bin/env python3
"""Serves crates/method-schedule-app for the page harness with caching turned off.

Usage: python3 scripts/serve-harness.py [PORT]   (default 8765, localhost only)
Then open http://localhost:PORT/harness/index.html

Python's own http.server lets the browser cache ES modules, so after a JS change the
page can load a stale module and fail with false import errors. Every response here
says no-store, so a reload always gets the files on disk.
"""

import functools
import http.server
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent / "crates" / "method-schedule-app"


class NoStore(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()


def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8765
    handler = functools.partial(NoStore, directory=str(ROOT))
    with http.server.ThreadingHTTPServer(("127.0.0.1", port), handler) as httpd:
        print(f"serving {ROOT} on http://localhost:{port}/harness/index.html")
        httpd.serve_forever()


if __name__ == "__main__":
    main()
