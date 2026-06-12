#!/usr/bin/env python3
"""Rasterize the RSpice brand SVGs into the app-icon export set.

Renders each source SVG with headless Chrome at the exact target pixel size
(transparent rounded corners preserved), re-renders the OG card, then
assembles the multi-size Windows ICO from the individually rendered PNGs
(PNG-compressed entries, largest first).

Cut tiering (per design/brand/run-mark-refined.html):
  512-48  run-icon.svg      port-ring terminals, equal pins
  32      run-icon-32.svg   dot-tip terminals on the heavy stroke
  16      run-icon-16.svg   bare heavy cut - NOT re-rendered here; the
                            shipped icon-16.png is reused for the ICO frame.
"""

import os
import struct
import subprocess
import sys
import tempfile

TOOLS = os.path.dirname(os.path.abspath(__file__))
BRAND = os.path.normpath(os.path.join(TOOLS, "..", "brand"))
EXPORT = os.path.join(BRAND, "export")

CHROME_CANDIDATES = [
    r"C:\Program Files\Google\Chrome\Application\chrome.exe",
    r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
    os.path.expandvars(r"%LOCALAPPDATA%\Google\Chrome\Application\chrome.exe"),
    r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
]

PNG_JOBS = [
    ("run-icon.svg", 512, "icon-512.png"),
    ("run-icon.svg", 256, "icon-256.png"),
    ("run-icon.svg", 128, "icon-128.png"),
    ("run-icon.svg", 64, "icon-64.png"),
    ("run-icon.svg", 48, "icon-48.png"),
    ("run-icon-32.svg", 32, "icon-32.png"),
    ("run-icon-dark.svg", 512, "icon-dark-512.png"),
    ("run-icon-dark.svg", 256, "icon-dark-256.png"),
]

ICO_SIZES = [256, 128, 64, 48, 32, 16]


def find_chrome():
    for p in CHROME_CANDIDATES:
        if os.path.isfile(p):
            return p
    sys.exit("no headless-capable Chrome/Edge found")


def file_url(path):
    return "file:///" + os.path.abspath(path).replace("\\", "/")


def shoot(chrome, url, out_png, width, height, transparent, budget_ms):
    with tempfile.TemporaryDirectory() as profile:
        cmd = [
            chrome,
            "--headless=new",
            "--screenshot=" + out_png,
            "--window-size=%d,%d" % (width, height),
            "--hide-scrollbars",
            "--force-device-scale-factor=1",
            "--disable-gpu",
            "--allow-file-access-from-files",
            "--virtual-time-budget=%d" % budget_ms,
            "--user-data-dir=" + profile,
            url,
        ]
        if transparent:
            cmd.insert(3, "--default-background-color=00000000")
        subprocess.run(cmd, check=True, capture_output=True)


def png_size(path):
    with open(path, "rb") as f:
        head = f.read(24)
    if head[:8] != b"\x89PNG\r\n\x1a\n" or head[12:16] != b"IHDR":
        sys.exit(path + ": not a PNG")
    w, h = struct.unpack(">II", head[16:24])
    return w, h


def build_ico(out_path, frames):
    """frames: list of (size, png_path), written largest-first."""
    blobs = [(size, open(p, "rb").read()) for size, p in frames]
    header = struct.pack("<HHH", 0, 1, len(blobs))
    offset = len(header) + 16 * len(blobs)
    entries, body = b"", b""
    for size, blob in blobs:
        dim = 0 if size >= 256 else size
        entries += struct.pack("<BBBBHHII", dim, dim, 0, 0, 1, 32, len(blob), offset)
        body += blob
        offset += len(blob)
    with open(out_path, "wb") as f:
        f.write(header + entries + body)


def main():
    chrome = find_chrome()
    print("renderer: " + chrome)

    for src, size, out in PNG_JOBS:
        src_path = os.path.join(BRAND, src)
        out_path = os.path.join(EXPORT, out)
        shoot(chrome, file_url(src_path), out_path, size, size,
              transparent=True, budget_ms=1000)
        w, h = png_size(out_path)
        if (w, h) != (size, size):
            sys.exit("%s rendered %dx%d, wanted %d" % (out, w, h, size))
        print("ok  %-18s %dx%d  from %s" % (out, w, h, src))

    og_src = os.path.join(BRAND, "og-card.html")
    og_out = os.path.join(EXPORT, "og-card.png")
    shoot(chrome, file_url(og_src), og_out, 1200, 630,
          transparent=False, budget_ms=3000)
    w, h = png_size(og_out)
    if (w, h) != (1200, 630):
        sys.exit("og-card.png rendered %dx%d, wanted 1200x630" % (w, h))
    print("ok  og-card.png        1200x630")

    frames = [(s, os.path.join(EXPORT, "icon-%d.png" % s)) for s in ICO_SIZES]
    for s, p in frames:
        if not os.path.isfile(p):
            sys.exit("missing ICO frame source: " + p)
    build_ico(os.path.join(EXPORT, "rspice.ico"), frames)
    print("ok  rspice.ico         frames: " + ", ".join(str(s) for s in ICO_SIZES))


if __name__ == "__main__":
    main()
