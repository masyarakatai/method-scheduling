# Clean-room rules

Method Schedule is an independent, clean-room film and TV scheduling tool. It is not affiliated
with Entertainment Partners or Movie Magic Scheduling.

**Allowed sources**
- How scheduling actually works as a craft: script breakdown practice, page eighths, strip
  colors by interior or exterior and time of day, day out of days conventions, union
  turnaround and holiday practice.
- Observed behavior of other scheduling tools, used normally through their UI.
- Public tutorials, public online manuals, release notes, public videos and reviews.
- Files those tools *export* in open formats (Excel, CSV, PDF), for import and comparison.
- Open screenplay formats: Fountain (an open spec) and Final Draft `.fdx` (readable XML).

**Never**
- Decompile, disassemble, debug or trace another vendor's binaries.
- Reverse engineer a proprietary native or exchange file, or research its layout. That
  covers every file another tool writes that is not plain Excel, CSV or PDF.
- Copy another vendor's code, help text, labels beyond a few-word industry term, report
  titles or layouts as designed, templates, icons, images, sample data or bundled lists.
- Quote a manual in our notes. Restate the behavior in our own words.
- Read, port or paste code from other open-source projects for features here. Their
  READMEs and docs are fine for concepts.

**Habits**
- Every research note opens with a `Source:` line and a `Clean-room:` line (behavior in our
  own words, no copied text, the source stays outside the repo).
- Screenshots and recordings of other tools live in `/Screenshots/`; raw exports and
  fetched pages live in `/imports/`. Both are gitignored and never committed.
- Test fixtures are small and synthetic: invented productions, invented people, scripts we
  write ourselves.

Our own native file format is open and documented in this repo.

**Native file conversion**, if anyone ever builds it, lives outside this repository as a
separate tool made by people who never work on this code. Its code, notes and guides must
never be read by anyone working here, and must never be copied, linked or vendored here.
