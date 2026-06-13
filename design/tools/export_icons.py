#!/usr/bin/env python3
"""Rasterize the RSpice brand SVGs into the app-icon export set.

The standard-cut sizes 256/128/64/48 are NOT screenshotted individually — they
are DERIVED by high-quality Lanczos downscaling of the single 512 master. The
old per-size approach drove headless Chrome once per size, and Chrome's
`--screenshot` capture races the SVG paint on large canvases: it silently
shipped a horizontally shifted, right-clipped `icon-256.png` (the running-app
taskbar icon, embedded by crates/rspice-ui/src/main.rs::load_window_icon) and a
near-blank `icon-128.png`. Dimensions were correct, so the old size-only check
never caught it. Rendering one master per distinct cut and resampling down is
deterministic and pixel-stable, and every frame is now validated for centering.

Masters (one per distinct artwork):
  run-icon.svg      @512  standard cut  -> icon-512 + derived 256/128/64/48
  run-icon-dark.svg @512  dark tile     -> icon-dark-512 + derived dark-256
  run-icon-32.svg   @32   dot-tip cut   -> icon-32 (rings would smear shut here)
  run-icon-16.svg   ...   bare cut      -> icon-16 is reused on disk, not rendered
  og-card.html      1200x630 social card

Every render is validated (exact size, sane opaque coverage, centred bounding
box) and retried; a master that never validates aborts the run before any good
asset is overwritten. Pass --reuse-masters to skip Chrome entirely and rebuild
the derived set + ICO from the master PNGs already on disk (icon-512,
icon-dark-512, icon-32, icon-16) — a deterministic repair that cannot race.

Re-run after any change to the brand SVGs or the OG card.
"""

import argparse
import os
import struct
import subprocess
import sys
import tempfile

from PIL import Image

TOOLS = os.path.dirname(os.path.abspath(__file__))
BRAND = os.path.normpath(os.path.join(TOOLS, "..", "brand"))
EXPORT = os.path.join(BRAND, "export")

CHROME_CANDIDATES = [
    r"C:\Program Files\Google\Chrome\Application\chrome.exe",
    r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
    os.path.expandvars(r"%LOCALAPPDATA%\Google\Chrome\Application\chrome.exe"),
    r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
]

# Masters rendered from SVG (cut, source, pixel size, output filename).
STD_MASTER = ("run-icon.svg", 512, "icon-512.png")
DARK_MASTER = ("run-icon-dark.svg", 512, "icon-dark-512.png")
DOT_MASTER = ("run-icon-32.svg", 32, "icon-32.png")

# Sizes derived by downscaling a validated master.
STD_DERIVED = [256, 128, 64, 48]   # from icon-512.png
DARK_DERIVED = [256]               # from icon-dark-512.png (-> icon-dark-256.png)

# ICO frames, largest first; 16 is the bare cut reused on disk.
ICO_SIZES = [256, 128, 64, 48, 32, 16]

RENDER_RETRIES = 4


def find_chrome():
    for p in CHROME_CANDIDATES:
        if os.path.isfile(p):
            return p
    return None


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
            # Force the compositor to finish every stage before the capture —
            # the single most effective guard against the paint/screenshot race.
            "--run-all-compositor-stages-before-draw",
            "--allow-file-access-from-files",
            "--virtual-time-budget=%d" % budget_ms,
            "--user-data-dir=" + profile,
            url,
        ]
        if transparent:
            cmd.insert(3, "--default-background-color=00000000")
        subprocess.run(cmd, check=True, capture_output=True)


def validate_tile(img, size):
    """Reject the screenshot race: a full-bleed tile must fill the canvas and
    sit centred. Returns None if OK, else a human-readable reason."""
    if img.size != (size, size):
        return "wrong size %s" % (img.size,)
    alpha = img.getchannel("A")
    bbox = alpha.getbbox()  # bounds of non-transparent pixels
    if bbox is None:
        return "fully transparent (blank render)"
    # Opaque coverage: a rounded full-bleed tile covers ~96% of the square.
    hist = alpha.histogram()
    opaque = sum(hist[16:])  # alpha > 15
    coverage = opaque / float(size * size)
    if coverage < 0.80:
        return "opaque coverage %.2f < 0.80 (clipped/partial render)" % coverage
    # Centred: a full-bleed tile's content box must hug all four edges. A
    # shifted/clipped capture leaves a wide transparent margin on one side.
    left, top, right, bottom = bbox
    margin = max(left, top, size - right, size - bottom)
    if margin > size * 0.06:
        return "off-centre: content bbox %s leaves %dpx margin" % (bbox, margin)
    return None


def render_master(chrome, src, size, out_path, transparent=True):
    """Render one SVG master to out_path, validated + retried. Aborts on
    persistent failure rather than shipping a raced frame."""
    src_path = os.path.join(BRAND, src)
    last = "no attempt"
    for attempt in range(1, RENDER_RETRIES + 1):
        shoot(chrome, file_url(src_path), out_path, size, size,
              transparent=transparent, budget_ms=1500)
        with Image.open(out_path) as im:
            im.load()
            problem = validate_tile(im, size)
        if problem is None:
            print("ok  %-18s %dx%d  master from %s" % (
                os.path.basename(out_path), size, size, src))
            return
        last = problem
        print("..  %-18s attempt %d/%d rejected: %s" % (
            os.path.basename(out_path), attempt, RENDER_RETRIES, problem))
    sys.exit("%s never rendered cleanly: %s" % (os.path.basename(out_path), last))


def load_valid_master(out_path, size):
    """--reuse-masters: load an existing master PNG and confirm it is good."""
    if not os.path.isfile(out_path):
        sys.exit("reuse-masters: missing master " + out_path)
    with Image.open(out_path) as im:
        im.load()
        master = im.convert("RGBA")
    problem = validate_tile(master, size)
    if problem is not None:
        sys.exit("reuse-masters: %s is itself bad (%s) — re-render from SVG"
                 % (os.path.basename(out_path), problem))
    print("ok  %-18s %dx%d  master (reused, validated)" % (
        os.path.basename(out_path), size, size))
    return master


def derive(master, sizes, name_fmt):
    """Downscale a validated master to each size (Lanczos, alpha-preserving)."""
    for s in sizes:
        out = os.path.join(EXPORT, name_fmt % s)
        small = master.resize((s, s), Image.LANCZOS)
        problem = validate_tile(small, s)
        if problem is not None:
            sys.exit("derived %s failed validation: %s" % (name_fmt % s, problem))
        small.save(out)
        print("ok  %-18s %dx%d  derived" % (os.path.basename(out), s, s))


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
    ap = argparse.ArgumentParser(description="Export the RSpice app-icon set.")
    ap.add_argument("--reuse-masters", action="store_true",
                    help="rebuild derived sizes + ICO from on-disk master PNGs "
                         "(no Chrome); deterministic repair of a raced set.")
    args = ap.parse_args()

    if args.reuse_masters:
        print("mode: reuse-masters (no Chrome)")
        std_master = load_valid_master(os.path.join(EXPORT, STD_MASTER[2]), STD_MASTER[1])
        dark_master = load_valid_master(os.path.join(EXPORT, DARK_MASTER[2]), DARK_MASTER[1])
        load_valid_master(os.path.join(EXPORT, DOT_MASTER[2]), DOT_MASTER[1])
    else:
        chrome = find_chrome()
        if chrome is None:
            sys.exit("no headless-capable Chrome/Edge found "
                     "(use --reuse-masters to rebuild from on-disk masters)")
        print("renderer: " + chrome)

        # One render per distinct cut; derivatives come from the 512 masters.
        for src, size, out in (STD_MASTER, DARK_MASTER, DOT_MASTER):
            render_master(chrome, src, size, os.path.join(EXPORT, out))
        with Image.open(os.path.join(EXPORT, STD_MASTER[2])) as im:
            std_master = im.convert("RGBA")
        with Image.open(os.path.join(EXPORT, DARK_MASTER[2])) as im:
            dark_master = im.convert("RGBA")

        og_src = os.path.join(BRAND, "og-card.html")
        og_out = os.path.join(EXPORT, "og-card.png")
        for attempt in range(1, RENDER_RETRIES + 1):
            shoot(chrome, file_url(og_src), og_out, 1200, 630,
                  transparent=False, budget_ms=3000)
            with Image.open(og_out) as im:
                im.load()
                ok = im.size == (1200, 630) and im.convert("L").getextrema()[0] \
                    != im.convert("L").getextrema()[1]
            if ok:
                print("ok  og-card.png        1200x630")
                break
            print("..  og-card.png        attempt %d/%d rejected" % (attempt, RENDER_RETRIES))
        else:
            sys.exit("og-card.png never rendered cleanly")

    derive(std_master, STD_DERIVED, "icon-%d.png")
    derive(dark_master, DARK_DERIVED, "icon-dark-%d.png")

    frames = [(s, os.path.join(EXPORT, "icon-%d.png" % s)) for s in ICO_SIZES]
    for s, p in frames:
        if not os.path.isfile(p):
            sys.exit("missing ICO frame source: " + p)
    build_ico(os.path.join(EXPORT, "rspice.ico"), frames)
    print("ok  rspice.ico         frames: " + ", ".join(str(s) for s in ICO_SIZES))


if __name__ == "__main__":
    main()
