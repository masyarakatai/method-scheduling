#!/usr/bin/env python3
"""Write an invented .fdx script for ZTEST checks in the reference app.

Usage: python3 scripts/make-ztest-fdx.py OUT.fdx            # DOOD run (runs 1 and 2)
       python3 scripts/make-ztest-fdx.py --lengths OUT.fdx  # page length run (run 3)

DOOD run: 25 scenes, numbered 1 to 25. With a day break after every strip, scene N is
shoot day N. Each invented character speaks on exactly the days its check needs
(docs/research/engine-checks.md). Every scene has its own set so nothing merges.

Length run: one scene per entry in LENGTHS, each with its Length attribute written in a
different form, to see which forms the reference app reads (E-I4) and to drive automatic
day breaks over mixed lengths (E-B1, E-B2, E-B6).
"""

import sys
from xml.sax.saxutils import escape

# Character -> shoot days. One element per check, so one schedule covers several.
CAST = {
    "ADA": [1, 11, 21],    # E-D2: holds + drops, minimum 7: one drop or two?
    "BRUNO": [4, 7],       # E-D5: holds on, drops off; Thu Mar 4 and Tue Mar 9
    "CLEO": [1, 11],       # E-D1: holds + drops, minimum 12 / 13 / 14
    "DARIO": [1, 11],      # holds on, drops off: holds across a long gap
    "EVE": [1, 2, 3],      # holds on: SW, W, WF in a row
    "FINN": [5],           # one day: SWF
}


# Length attribute as written, one scene each.
LENGTHS = ["8/8", "1 0/8", "1", "4/8", "1 2/8", "2/8", "3/8", "5/8", "4/8", "4/8", "10/8", "2 1/8"]


def para(kind, text, extra=""):
    return f'<Paragraph Type="{kind}"{extra}>\n<Text>{escape(text)}</Text>\n</Paragraph>\n'


def main(out, lengths=False):
    body = []
    scenes = len(LENGTHS) if lengths else 25
    for n in range(1, scenes + 1):
        length = LENGTHS[n - 1] if lengths else "1"
        body.append(
            f'<Paragraph Number="{n}" Type="Scene Heading">\n'
            f'<SceneProperties Length="{length}" Page="{n}" Title=""/>\n'
            f"<Text>INT. ROOM {n} - DAY</Text>\n</Paragraph>\n"
        )
        body.append(para("Action", f"Room {n}. Nothing happens."))
        for name, days in ({} if lengths else CAST).items():
            if n in days:
                body.append(para("Character", name))
                body.append(para("Dialogue", f"Line in room {n}."))
    xml = (
        '<?xml version="1.0" encoding="UTF-8" standalone="no" ?>\n'
        '<FinalDraft DocumentType="Script" Template="No" Version="5">\n<Content>\n'
        + "".join(body)
        + "</Content>\n</FinalDraft>\n"
    )
    with open(out, "w") as f:
        f.write(xml)


if __name__ == "__main__":
    if sys.argv[1] == "--lengths":
        main(sys.argv[2], lengths=True)
    else:
        main(sys.argv[1])
