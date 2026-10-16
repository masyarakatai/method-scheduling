#!/usr/bin/env python3
"""Is docs/releases/<version>.md ready to publish? (docs/release-notes-template.md)

    python3 scripts/notes-check.py 0.2.0

Refuses a file with a `{{...}}` placeholder left, a title that isn't
"# Method Schedule <version>", no headline, no section of what's new, no `## Known gaps`
or `## Thanks` (the suite's closing sections, SUITE-STANDARDS.md 7.2), an em dash or an
exclamation mark, or more than 60,000 characters. Exits 1 and says why.
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CLOSING = ["Fixed", "Known gaps", "Thanks"]


def problems(version: str, text: str) -> list[str]:
    out = []
    lines = text.splitlines()
    if not lines or lines[0].strip() != f"# Method Schedule {version}":
        out.append(f'the first line must be "# Method Schedule {version}"')
    if "{{" in text or "}}" in text:
        out.append("a {{placeholder}} is left")
    body = [l for l in lines[1:] if l.strip()]
    if not body or body[0].startswith(("#", "-")):
        out.append("there is no headline paragraph under the title")
    sections = [l[3:].strip() for l in lines if l.startswith("## ")]
    new = [s for s in sections if s not in CLOSING]
    if not new:
        out.append("there is no section of what's new")
    for must in ["Known gaps", "Thanks"]:
        if must not in sections:
            out.append(f"there is no ## {must} section")
    prose = re.sub(r"<!--.*?-->", "", text, flags=re.S)
    if "—" in prose:
        out.append("it has an em dash")
    if re.search(r"!(?!\[)", re.sub(r"`[^`]*`", "", prose)):
        out.append("it has an exclamation mark")
    if len(text) > 60000:
        out.append("it is over 60,000 characters")
    return out


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: python3 scripts/notes-check.py VERSION", file=sys.stderr)
        return 2
    version = sys.argv[1]
    path = ROOT / "docs" / "releases" / f"{version}.md"
    if not path.is_file():
        print(f"notes-check: no {path.relative_to(ROOT)}")
        return 1
    found = problems(version, path.read_text())
    for p in found:
        print(f"notes-check: {p}")
    if not found:
        print(f"notes-check: docs/releases/{version}.md is ready")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
