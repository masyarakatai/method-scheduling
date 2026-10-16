#!/usr/bin/env python3
"""App icons for crates/method-schedule-app: the mark in white on a slate blue rounded square.

Usage: python3 scripts/make-icons.py [OUT_DIR]   (default: crates/method-schedule-app/icons)

Writes 32x32.png, 128x128.png, 128x128@2x.png, icon.png (512), icon.icns and icon.ico,
the files tauri-build and the bundler read. The rows are scripts/make-brand.py's, not a
copy: the 64-unit mark from 48 pixels up, and the 16-unit favicon drawing at 16 and 32
pixels, where whole-unit rows land on whole pixels and stay sharp. The square fills the
drawing's grid (corner radius 14.3 of 64); from 48 pixels up it sits inside the macOS
icon grid's margin (100 of 1024 on each side), smaller sizes are full bleed. Rules for
the icon are in docs/BRAND.md. Stdlib only, and the output is the same bytes every run.
"""

import importlib.util
import os
import struct
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def load_brand():
    spec = importlib.util.spec_from_file_location("make_brand", os.path.join(ROOT, "scripts", "make-brand.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


brand = load_brand()

WHITE = (255, 255, 255)
SQUARE_RADIUS = 14.3 / 64  # of the square's side, as Apple's icon shape
INSET = 100 / 1024  # macOS icon grid: the square is 824 of 1024
STRIP = 0.55  # the strips' white on slate blue; the day break is full white
SMALL = 32  # this size and under use the favicon drawing


def coverage_rounded_rect(x, y, x0, y0, x1, y1, r):
    """How much of the pixel centred at (x, y) lies inside the rounded rectangle, 0..1."""
    cx = min(max(x, x0 + r), x1 - r)
    cy = min(max(y, y0 + r), y1 - r)
    if x0 + r <= x <= x1 - r or y0 + r <= y <= y1 - r:
        # Beside a straight edge: the area of the pixel on the inside of each edge.
        fx = min(max(x - x0 + 0.5, 0.0), 1.0) * min(max(x1 - x + 0.5, 0.0), 1.0)
        fy = min(max(y - y0 + 0.5, 0.0), 1.0) * min(max(y1 - y + 0.5, 0.0), 1.0)
        return fx * fy
    d = ((x - cx) ** 2 + (y - cy) ** 2) ** 0.5 - r
    return min(max(0.5 - d, 0.0), 1.0)


def render(size):
    """PNG scanlines (filter byte, then RGBA) for one icon size."""
    if size <= SMALL:
        units, rows, inset = 16.0, brand.fav_rows(), 0.0
    else:
        units, rows, inset = 64.0, brand.mark_rows(), size * INSET
    k = (size - 2 * inset) / units  # pixels per unit
    side = units * k
    square = (inset, inset, inset + side, inset + side, side * SQUARE_RADIUS)
    shapes = [
        (inset + x * k, inset + y * k, inset + (x + w) * k, inset + (y + h) * k, r * k,
         1.0 if strength == 1 else STRIP)
        for x, y, w, h, r, strength in rows
    ]
    back = brand.hex_rgb(brand.ACCENT)
    out = bytearray()
    for py in range(size):
        out.append(0)
        y = py + 0.5
        band = [s for s in shapes if s[1] - 1 <= y <= s[3] + 1]
        for px in range(size):
            x = px + 0.5
            a = coverage_rounded_rect(x, y, *square)
            if a <= 0.0:
                out += b"\x00\x00\x00\x00"
                continue
            white = 0.0
            for x0, y0, x1, y1, r, strength in band:
                if x0 - 1 <= x <= x1 + 1:
                    white += coverage_rounded_rect(x, y, x0, y0, x1, y1, r) * strength
            out += bytes(round(c + (w - c) * white) for c, w in zip(back, WHITE))
            out.append(round(a * 255))
    return bytes(out)


def icns(pngs):
    """An .icns holding PNG images: ic07 (128), ic08 (256), ic09 (512), ic10 (1024)."""
    body = b""
    for kind, size in ((b"ic07", 128), (b"ic08", 256), (b"ic09", 512), (b"ic10", 1024)):
        data = pngs[size]
        body += kind + struct.pack(">I", len(data) + 8) + data
    return b"icns" + struct.pack(">I", len(body) + 8) + body


def main():
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(ROOT, "crates", "method-schedule-app", "icons")
    os.makedirs(out, exist_ok=True)
    pngs = {size: brand.png(size, render(size)) for size in (16, 32, 48, 128, 256, 512, 1024)}
    files = {
        "32x32.png": pngs[32],
        "128x128.png": pngs[128],
        "128x128@2x.png": pngs[256],
        "icon.png": pngs[512],
        "icon.icns": icns(pngs),
        "icon.ico": brand.ico([(size, pngs[size]) for size in (16, 32, 48, 256)]),
    }
    for name, data in files.items():
        with open(os.path.join(out, name), "wb") as f:
            f.write(data)
        print(f"wrote {os.path.join(out, name)} ({len(data)} bytes)")


if __name__ == "__main__":
    main()
