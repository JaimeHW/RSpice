#!/usr/bin/env python3
"""Build RSpice Symbols, the engineering-symbol face at the end of every font chain.

IBM Plex has no diameter sign, no ground symbol, no GD&T characteristic and none
of the geometric marks the workbench draws, so those characters would otherwise
depend on whatever the viewer's machine has. This script takes each listed code
point from the first source face that has it (IBM Plex Math first, which shares
Plex's metrics and design; then Noto Sans Symbols 2 and Noto Sans Math), copies
its outline without hinting, and writes one small TrueType face.

The result is a Modified Version under the SIL Open Font License 1.1, so its
names do not use the Reserved Font Name "Plex", and its licence file carries the
copyright line of every source a glyph was taken from.

The sources are pinned by release URL and SHA-256 in assets/fonts/SOURCES.tsv
(rows whose role is "build-input"); the build records the outputs' hashes in the
"derived" rows. Licence texts are hashed with LF line endings, as git stores
them. The output is deterministic: rebuilding from the pinned sources with the
same fontTools version reproduces the committed bytes.

Usage:
    py -3 tools/fonts/build_rspice_symbols.py fetch DIR   download and verify the sources into DIR
    py -3 tools/fonts/build_rspice_symbols.py build DIR   rebuild the face from the sources in DIR
    py -3 tools/fonts/build_rspice_symbols.py check       verify the committed face
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import io
import sys
import unicodedata
import urllib.request
import zipfile
from pathlib import Path

from fontTools.fontBuilder import FontBuilder
from fontTools.misc.timeTools import timestampFromString
from fontTools.pens.recordingPen import DecomposingRecordingPen
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.ttLib import TTFont

ROOT = Path(__file__).resolve().parents[2]
FONTS = ROOT / "assets" / "fonts"
MANIFEST = FONTS / "SOURCES.tsv"
OUTPUT = FONTS / "RSpiceSymbols-Regular.ttf"
LICENCE = FONTS / "OFL-RSpiceSymbols.txt"
PLEX_LICENCE = FONTS / "OFL-IBMPlex.txt"

FAMILY = "RSpice Symbols"
POSTSCRIPT = "RSpiceSymbols-Regular"
VERSION = "1.000"
# Fixed so the build is byte-reproducible; change it with VERSION.
TIMESTAMP = timestampFromString("Fri Oct 30 00:00:00 2026")
SIZE_BUDGET = 60 * 1024

# Source faces in chain order. The licence line is the copyright line of that
# source's own OFL file, which is what a Modified Version must keep.
SOURCES = (
    ("IBMPlexMath-Regular.ttf", 'Copyright © 2017 IBM Corp. with Reserved Font Name "Plex"'),
    ("NotoSansSymbols2-Regular.ttf", "Copyright 2022 The Noto Project Authors (https://github.com/notofonts/symbols)"),
    ("NotoSansMath-Regular.ttf", "Copyright 2022 The Noto Project Authors (https://github.com/notofonts/math)"),
)

# Every character the face guarantees. None of them is in any IBM Plex Sans or
# Mono face. U+25FB is also the glyph shown for a character no face covers.
SYMBOLS = (
    ("electrical", "⏚⏛⎓∿⏦⏧⌁⎍⎎⏻⏼⏽⏾⭘⚡℧"),
    ("math and units", "∅∇∝∓≡⋅℃⁻∠⊥∥"),
    (
        "GD&T (ASME Y14.5, ISO 1101)",
        "⌀⏤⏥○⌭⌒⌓⟂⌖◎⌯⌰⌴⌵↧□"
        "ⓂⓁⒻⓉⓅⓈ⌲⌳⌱",
    ),
    ("interface marks", "⌄▸▾▴◂★☆✕✗●◻◼■△▽◇⊞⚠ⓘ⇒"),
    ("keys", "⌘⌥⇧⌃⏎⌫⎋⇥⌦⇪"),
)

# Name IDs that name the font, which a Modified Version may not give the
# Reserved Font Name: family, subfamily, unique ID, full name, PostScript name,
# typographic family and subfamily, WWS family and subfamily.
FONT_NAME_IDS = (1, 2, 3, 4, 6, 16, 17, 21, 22)


def code_points() -> list[int]:
    points = [ord(c) for _, chars in SYMBOLS for c in chars]
    if len(points) != len(set(points)):
        raise SystemExit("the symbol list repeats a code point")
    return points


def sha256(path: Path) -> str:
    """Hash a file as git stores it: text files with LF line endings."""
    data = path.read_bytes()
    if path.suffix == ".txt":
        data = data.replace(b"\r\n", b"\n")
    return hashlib.sha256(data).hexdigest()


def manifest_rows(role: str) -> list[dict[str, str]]:
    with MANIFEST.open(encoding="utf-8", newline="") as handle:
        return [row for row in csv.DictReader(handle, delimiter="\t") if row["role"] == role]


def pinned_inputs() -> dict[str, dict[str, str]]:
    rows = {row["file"]: row for row in manifest_rows("build-input")}
    missing = [name for name, _ in SOURCES if name not in rows]
    if missing:
        raise SystemExit(f"{MANIFEST.name} pins no build input for {', '.join(missing)}")
    return rows


def pin_derived() -> None:
    """Record the built files' SHA-256 and size in their manifest rows."""
    with MANIFEST.open(encoding="utf-8", newline="") as handle:
        reader = csv.DictReader(handle, delimiter="\t")
        fields, rows = reader.fieldnames, list(reader)
    for row in rows:
        if row["role"] == "derived" and row["file"] in (OUTPUT.name, LICENCE.name):
            path = FONTS / row["file"]
            row["sha256"], row["bytes"] = sha256(path), str(path.stat().st_size)
    with MANIFEST.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fields, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def fetch(directory: Path) -> None:
    directory.mkdir(parents=True, exist_ok=True)
    for name, row in pinned_inputs().items():
        target = directory / name
        if target.exists() and sha256(target) == row["sha256"]:
            print(f"have    {name}")
            continue
        with urllib.request.urlopen(row["release_url"]) as response:
            archive = zipfile.ZipFile(io.BytesIO(response.read()))
        data = archive.read(row["member"])
        digest = hashlib.sha256(data).hexdigest()
        if digest != row["sha256"]:
            raise SystemExit(f"{name}: release member hashes to {digest}, the manifest pins {row['sha256']}")
        target.write_bytes(data)
        print(f"fetched {name}")


def load_sources(directory: Path) -> list[tuple[str, str, TTFont]]:
    pins = pinned_inputs()
    sources = []
    for name, licence_line in SOURCES:
        path = directory / name
        if not path.exists():
            raise SystemExit(f"{path} is missing; run `fetch {directory}` first")
        if sha256(path) != pins[name]["sha256"]:
            raise SystemExit(f"{path} does not match the SHA-256 pinned in {MANIFEST.name}")
        font = TTFont(path)
        if font["head"].unitsPerEm != 1000 or "glyf" not in font:
            raise SystemExit(f"{name}: expected a TrueType face with 1000 units per em")
        sources.append((name, licence_line, font))
    return sources


def outline(font: TTFont, glyph_name: str):
    glyph_set = font.getGlyphSet()
    recording = DecomposingRecordingPen(glyph_set)
    glyph_set[glyph_name].draw(recording)
    pen = TTGlyphPen(None)
    recording.replay(pen)
    return pen.glyph(), glyph_set[glyph_name].width


def build(directory: Path) -> None:
    sources = load_sources(directory)
    primary = sources[0][2]
    picks: dict[int, tuple[str, str]] = {}
    glyphs, widths = {}, {}
    glyphs[".notdef"], widths[".notdef"] = outline(primary, ".notdef")
    used = []
    for point in code_points():
        for name, licence_line, font in sources:
            source_glyph = font.getBestCmap().get(point)
            if source_glyph is None:
                continue
            glyph_name = f"uni{point:04X}"
            glyphs[glyph_name], widths[glyph_name] = outline(font, source_glyph)
            picks[point] = (name, glyph_name)
            if licence_line not in used:
                used.append(licence_line)
            break
        else:
            raise SystemExit(f"no source covers U+{point:04X} {unicodedata.name(chr(point), '')}")

    builder = FontBuilder(1000, isTTF=True)
    order = [".notdef"] + [picks[point][1] for point in sorted(picks)]
    builder.setupGlyphOrder(order)
    builder.setupCharacterMap({point: glyph_name for point, (_, glyph_name) in picks.items()})
    builder.setupGlyf(glyphs)
    glyf = builder.font["glyf"]
    builder.setupHorizontalMetrics({name: (widths[name], getattr(glyf[name], "xMin", 0)) for name in order})

    # Plex Math shares Plex Sans's vertical metrics, so a symbol borrowed into
    # a Plex line sits on the same baseline and leaves the line height alone.
    hhea, os2 = primary["hhea"], primary["OS/2"]
    builder.setupHorizontalHeader(ascent=hhea.ascent, descent=hhea.descent, lineGap=hhea.lineGap)
    pins = pinned_inputs()
    credits = [
        f"{font['name'].getDebugName(1)} ({pins[name]['version']})" for name, line, font in sources if line in used
    ]
    builder.setupNameTable(
        {
            "copyright": "; ".join(used),
            "familyName": FAMILY,
            "styleName": "Regular",
            "uniqueFontIdentifier": f"{VERSION};{POSTSCRIPT}",
            "fullName": f"{FAMILY} Regular",
            "version": f"Version {VERSION}",
            "psName": POSTSCRIPT,
            "description": "Engineering symbols for RSpice, taken from " + " and ".join(credits) + ".",
            "licenseDescription": "This Font Software is licensed under the SIL Open Font License, Version 1.1.",
            "licenseInfoURL": "https://openfontlicense.org",
        },
        mac=False,
    )
    builder.setupOS2(
        version=4,
        usWeightClass=400,
        usWidthClass=5,
        fsType=0,
        # REGULAR, keeping the source's USE_TYPO_METRICS bit.
        fsSelection=0x40 | (os2.fsSelection & 0x80),
        achVendID="    ",
        sTypoAscender=os2.sTypoAscender,
        sTypoDescender=os2.sTypoDescender,
        sTypoLineGap=os2.sTypoLineGap,
        usWinAscent=os2.usWinAscent,
        usWinDescent=os2.usWinDescent,
        sxHeight=os2.sxHeight,
        sCapHeight=os2.sCapHeight,
        usDefaultChar=0,
        usMaxContext=0,
    )
    builder.setupPost(keepGlyphNames=True)
    if "maxp" not in builder.font:
        builder.setupMaxp()
    builder.updateHead(created=TIMESTAMP, modified=TIMESTAMP, fontRevision=float(VERSION))

    font = builder.font
    font["OS/2"].recalcAvgCharWidth(font)
    font["OS/2"].recalcUnicodeRanges(font)
    font["OS/2"].recalcCodePageRanges(font)
    buffer = io.BytesIO()
    font.save(buffer)
    OUTPUT.write_bytes(buffer.getvalue())

    body = PLEX_LICENCE.read_text(encoding="utf-8").replace("\r\n", "\n")
    body = body[body.index("This Font Software is licensed") :]
    LICENCE.write_text("\n".join(used) + "\n\n" + body, encoding="utf-8", newline="\n")
    pin_derived()

    for point in code_points():
        print(f"U+{point:04X} {unicodedata.name(chr(point), '')}: {picks[point][0]}")
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({OUTPUT.stat().st_size} bytes, sha256 {sha256(OUTPUT)})")
    check()


def check() -> None:
    problems = []
    font = TTFont(OUTPUT)
    cmap = font.getBestCmap()
    glyf = font["glyf"]
    for point in code_points():
        glyph_name = cmap.get(point)
        if glyph_name is None:
            problems.append(f"U+{point:04X} {unicodedata.name(chr(point), '')} is not covered")
        elif glyf[glyph_name].numberOfContours == 0:
            problems.append(f"U+{point:04X} maps to an empty glyph")
    for record in font["name"].names:
        if record.nameID in FONT_NAME_IDS and "plex" in record.toUnicode().lower():
            problems.append(f"name {record.nameID} uses the Reserved Font Name: {record.toUnicode()!r}")
    if font["name"].getDebugName(1) != FAMILY:
        problems.append(f"family name is {font['name'].getDebugName(1)!r}, not {FAMILY!r}")
    size = OUTPUT.stat().st_size
    if size > SIZE_BUDGET:
        problems.append(f"{OUTPUT.name} is {size} bytes, over the {SIZE_BUDGET}-byte budget")
    if not LICENCE.exists():
        problems.append(f"{LICENCE.name} is missing")
    else:
        licence = LICENCE.read_text(encoding="utf-8")
        for line in font["name"].getDebugName(0).split("; "):
            source = next((entry for entry in SOURCES if entry[1] == line), None)
            if source is None or line not in licence:
                problems.append(f"{LICENCE.name} does not carry the copyright line {line!r}")
    derived = {row["file"]: row["sha256"] for row in manifest_rows("derived")}
    for path in (OUTPUT, LICENCE):
        if path.exists() and derived.get(path.name) != sha256(path):
            problems.append(f"{MANIFEST.name} does not pin the SHA-256 of {path.name}")
    if problems:
        print("\n".join(problems), file=sys.stderr)
        raise SystemExit(1)
    print(f"{OUTPUT.name}: {len(code_points())} code points covered, {size} bytes")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("fetch").add_argument("directory", type=Path)
    commands.add_parser("build").add_argument("directory", type=Path)
    commands.add_parser("check")
    arguments = parser.parse_args()
    if arguments.command == "fetch":
        fetch(arguments.directory)
    elif arguments.command == "build":
        build(arguments.directory)
    else:
        check()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
