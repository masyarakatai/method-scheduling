#!/usr/bin/env python3
"""Write THIRD-PARTY-NOTICES.md: every third-party crate the app and the command line ship.

    python3 scripts/notices.py           rewrite THIRD-PARTY-NOTICES.md
    python3 scripts/notices.py --check   exit 1 when it is out of date (CI, the deny job)

Reads `cargo metadata --locked` (run `cargo fetch --locked` first on a fresh machine, so
each crate's own license files are on disk). Follows normal dependencies only from
method-schedule-app and method-schedule-cli, for every platform. For each crate it names
the license we take it under (the first of PREFER its expression offers) and the
copyright lines from its own license files (its authors when the files have none). Each
license's full text then appears once at the end. Decision 209.
"""

import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "THIRD-PARTY-NOTICES.md"
SHIPPED = ("method-schedule-app", "method-schedule-cli")
# The license we take a crate under, when its expression offers a choice.
PREFER = ["MIT", "Apache-2.0", "Zlib", "ISC", "BSD-3-Clause", "Unicode-3.0", "MPL-2.0", "CDLA-Permissive-2.0"]
# How each license's text is recognized among the crates' own license files.
SIGNS = {
    "MIT": ["Permission is hereby granted, free of charge"],
    "Apache-2.0": ["Apache License", "Version 2.0, January 2004"],
    "Zlib": ["This software is provided 'as-is'", "altered source versions"],
    "ISC": ["Permission to use, copy, modify, and/or distribute this software for any"],
    "BSD-3-Clause": ["Redistribution and use in source and binary forms", "Neither the name"],
    "Unicode-3.0": ["UNICODE LICENSE V3"],
    "MPL-2.0": ["Mozilla Public License Version 2.0"],
    "CDLA-Permissive-2.0": ["Community Data License Agreement - Permissive - Version 2.0"],
}
LICENSE_FILE = re.compile(r"^(LICEN[CS]E|COPYING|NOTICE|UNLICENSE)", re.I)
# A holder's line: "Copyright (c) 2019 Someone", not license prose ("copyright notice ...").
COPYRIGHT = re.compile(r"^\s*(?:(?:Copyright|COPYRIGHT)\s*(?:\(c\)|\(C\)|©)?\s*(?:\d{4}|[A-Z(])|(?:\(c\)|\(C\)|©)\s*\d{4})")
PROSE = re.compile(r"^\s*Copyright\s+(notice|license|holder|owner|and\b|law|statement)", re.I)
TEMPLATE = re.compile(r"\[(yyyy|year|name)|<(year|copyright holders)>|\{yyyy\}|\{name of", re.I)


def metadata():
    out = subprocess.run(["cargo", "metadata", "--format-version", "1", "--locked"], cwd=ROOT, check=True, capture_output=True, text=True)
    return json.loads(out.stdout)


def shipped(meta):
    pkgs = {p["id"]: p for p in meta["packages"]}
    nodes = {n["id"]: n for n in meta["resolve"]["nodes"]}
    todo = [p["id"] for p in meta["packages"] if p["name"] in SHIPPED]
    seen = set()
    while todo:
        i = todo.pop()
        if i in seen:
            continue
        seen.add(i)
        for d in nodes[i]["deps"]:
            if any(k["kind"] in (None, "normal") for k in d["dep_kinds"]):
                todo.append(d["pkg"])
    return sorted((pkgs[i] for i in seen if pkgs[i]["source"]), key=lambda p: (p["name"], p["version"]))


def taken(expr):
    """The licenses we take a crate under: one per AND part, the first PREFER offers."""
    expr = (expr or "").replace("/", " OR ")
    picked = []
    for part in re.split(r"\s+AND\s+", expr):
        options = [o.strip(" ()") for o in re.split(r"\s+OR\s+", part.strip(" ()"))]
        options = [o for o in options if o]
        best = next((p for p in PREFER if p in options), options[0] if options else "unknown")
        if best not in picked:
            picked.append(best)
    return picked


def license_files(pkg):
    folder = Path(pkg["manifest_path"]).parent
    return sorted(f for f in folder.iterdir() if f.is_file() and LICENSE_FILE.match(f.name))


def copyrights(pkg):
    lines = []
    for f in license_files(pkg):
        for line in f.read_text(errors="replace").splitlines():
            line = " ".join(line.split())
            if COPYRIGHT.match(line) and not PROSE.match(line) and not TEMPLATE.search(line) and len(line) < 200 and line not in lines:
                lines.append(line)
    if lines:
        return lines
    authors = [re.sub(r"\s*<[^>]*>", "", a).strip() for a in pkg.get("authors") or []]
    authors = [a for a in authors if a]
    return [f"Authors: {', '.join(authors)}"] if authors else []


def texts(pkgs, needed):
    """One full text per license, from a crate taken under that license (any crate after)."""
    found = {}
    for lic in needed:
        under = [p for p in pkgs if lic in taken(p["license"])]
        for pkg in under + [p for p in pkgs if p not in under]:
            for f in license_files(pkg):
                body = f.read_text(errors="replace")
                flat = " ".join(body.replace("\u2019", "'").split()).lower()
                if all(x.lower() in flat for x in SIGNS.get(lic, ["\0"])):
                    found[lic] = canonical(lic, body)
                    break
            if lic in found:
                break
    return found


def canonical(lic, body):
    lines = body.replace("\r\n", "\n").strip("\n").split("\n")
    if lic == "MIT":
        # The permission text alone; each crate's copyright line is listed with the crate.
        start = next(i for i, l in enumerate(lines) if "Permission is hereby granted" in l)
        lines = lines[start:]
    if lic in ("Zlib", "ISC", "BSD-3-Clause"):
        # The terms alone; each crate's own copyright lines are listed with the crate.
        lines = [l for l in lines if not COPYRIGHT.match(l)]
    if lic == "Apache-2.0":
        end = next((i for i, l in enumerate(lines) if "END OF TERMS AND CONDITIONS" in l), len(lines) - 1)
        lines = lines[: end + 1]
    return "\n".join(l.rstrip() for l in lines).strip("\n")


def render(pkgs):
    rows = []
    needed = []
    for p in pkgs:
        lic = taken(p["license"])
        for x in lic:
            if x not in needed:
                needed.append(x)
        rows.append((p, lic))
    found = texts(pkgs, needed)
    out = [
        "# Third-party notices",
        "",
        "Method Schedule's own code is licensed under the MIT License or the Apache License 2.0,",
        "at your option: see [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).",
        "Copyright (c) 2026 Method Production Suite Team and the Method Schedule contributors.",
        "",
        "The app (`method-schedule-app`) and the command line (`method-schedule`) are built with",
        f"the {len(pkgs)} third-party Rust crates below, each listed with the license we take it",
        "under (where a crate offers a choice) and its copyright lines. Each license's full text",
        "follows the list. The app draws its window with the system's own web view (WebKit on",
        "macOS and Linux, WebView2 on Windows), which is part of the operating system and not",
        "shipped with it.",
        "",
        "This file is written by `scripts/notices.py` from `Cargo.lock`; CI checks it is current.",
        "",
        "## Crates",
        "",
    ]
    for p, lic in rows:
        where = p.get("repository") or f"https://crates.io/crates/{p['name']}"
        out.append(f"### {p['name']} {p['version']}")
        out.append("")
        out.append(f"License: {' and '.join(lic)} (offered: {p['license'] or 'not stated'}). Source: {where}")
        for c in copyrights(p):
            out.append("")
            out.append(c)
        out.append("")
    out += ["## License texts", ""]
    for lic in needed:
        out.append(f"### {lic}")
        out.append("")
        if lic in found:
            out.append("```text")
            out.append(found[lic])
            out.append("```")
        else:
            out.append(f"The text is at https://spdx.org/licenses/{lic}.html")
        out.append("")
    return "\n".join(out).rstrip("\n") + "\n"


def main():
    text = render(shipped(metadata()))
    if "--check" in sys.argv[1:]:
        if not OUT.exists() or OUT.read_text() != text:
            print("notices.py: THIRD-PARTY-NOTICES.md is out of date; run python3 scripts/notices.py")
            return 1
        print("notices.py: THIRD-PARTY-NOTICES.md is up to date")
        return 0
    OUT.write_text(text)
    print(f"wrote {OUT.name}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
