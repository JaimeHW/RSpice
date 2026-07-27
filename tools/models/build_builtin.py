"""Generate the built-in SPICE library by selecting from the vendored packs.

The built-in library is what gets compiled into the RSpice binary, so it is the
one part of models/spice/ that must be both small and trustworthy. It is not
hand-authored: `models/spice/builtin/selection.toml` names cards that already
exist in the vendored packs, and this tool extracts them verbatim.

Two guarantees the generator enforces, both as hard errors:

  * No source file that `tools/models/license_audit.py` marks `restricted` may
    contribute a card. Those files are excluded from a distributed build, so a
    card copied out of one would smuggle the restriction into the binary.
  * Model parameters are copied byte for byte. `name` may rename a card to the
    part's canonical designation, which rewrites one token on the header line
    and nothing else.

Usage:
    python tools/models/build_builtin.py
    python tools/models/build_builtin.py --check    fail if lib/ is stale
"""

from __future__ import annotations

import argparse
import re
import sys
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
SPICE_ROOT = REPO_ROOT / "models" / "spice"
BUILTIN = SPICE_ROOT / "builtin"
SELECTION = BUILTIN / "selection.toml"
LIB_DIR = BUILTIN / "lib"
AUDIT = SPICE_ROOT / "LICENSE-AUDIT.tsv"

CATEGORY_TITLES = {
    "diode": "Diodes, rectifiers, Schottky diodes, zeners and LEDs",
    "bjt": "Bipolar junction transistors",
    "jfet": "Junction field-effect transistors",
    "mosfet": "MOSFETs, small-signal and power",
    "opamp": "Operational amplifiers",
    "ic": "Integrated circuits, timers and voltage regulators",
}


class BuildError(RuntimeError):
    """The built-in library could not be generated."""


def read_text(path: Path) -> str:
    raw = path.read_bytes()
    for encoding in ("utf-8", "cp1252", "latin-1"):
        try:
            return raw.decode(encoding)
        except UnicodeDecodeError:
            continue
    raise BuildError(f"{path}: no usable text encoding")


def restricted_sources() -> set[tuple[str, str]]:
    """(pack, path) pairs the licence audit marked as non-shippable."""
    if not AUDIT.exists():
        raise BuildError(
            f"{AUDIT} is missing; run tools/models/license_audit.py first"
        )
    blocked: set[tuple[str, str]] = set()
    for line in AUDIT.read_text(encoding="utf-8").splitlines():
        if line.startswith("#") or not line.strip():
            continue
        fields = line.split("\t")
        if len(fields) > 3 and fields[3] == "restricted":
            blocked.add((fields[0], fields[1]))
    return blocked


@dataclass
class Card:
    """One contribution to the built-in library."""

    category: str
    pack: str
    path: str
    line: int
    parts: list[str]
    note: str
    body: str
    renamed_from: str | None = None


@dataclass
class Selection:
    files: list[dict] = field(default_factory=list)
    models: list[dict] = field(default_factory=list)


def load_selection() -> Selection:
    with SELECTION.open("rb") as handle:
        data = tomllib.load(handle)
    if data.get("schema") != 1:
        raise BuildError(f"{SELECTION}: unsupported schema {data.get('schema')!r}")
    return Selection(files=data.get("file", []), models=data.get("model", []))


def source_path(pack: str, path: str) -> Path:
    resolved = SPICE_ROOT / pack / path
    if not resolved.is_file():
        raise BuildError(f"source not found: {pack}/{path}")
    return resolved


def check_allowed(pack: str, path: str, blocked: set[tuple[str, str]]) -> None:
    if (pack, path) in blocked:
        raise BuildError(
            f"{pack}/{path} is marked restricted by the licence audit and "
            f"cannot contribute to the built-in library"
        )


DEFINITION_NAME = re.compile(
    r"^(?P<lead>[ \t]*\.model[ \t]+)(?P<name>\S+)(?P<rest>.*)$",
    re.IGNORECASE,
)


def extract_model(text: str, source_name: str) -> tuple[str, int]:
    """Return a `.model` card with its continuation lines, and its line number."""
    lines = text.splitlines()
    for index, line in enumerate(lines):
        match = DEFINITION_NAME.match(line)
        if not match or match.group("name").upper() != source_name.upper():
            continue

        block = [line]
        cursor = index + 1
        while cursor < len(lines):
            following = lines[cursor]
            stripped = following.lstrip()
            # Continuation lines carry a leading '+'. Comments interleaved in a
            # card belong to it and are kept; anything else ends the card.
            if stripped.startswith("+"):
                block.append(following)
            elif stripped.startswith("*") and cursor + 1 < len(lines) and lines[
                cursor + 1
            ].lstrip().startswith("+"):
                block.append(following)
            else:
                break
            cursor += 1
        return "\n".join(block), index + 1

    raise BuildError(f"model {source_name!r} not found")


DEFINITION_HEADER = re.compile(
    r"^(?P<lead>[ \t]*\.(?:model|subckt|ends)[ \t]+)(?P<name>\S+)(?P<rest>.*)$",
    re.IGNORECASE,
)


def rename_definitions(body: str, renames: list[list[str]]) -> str:
    """Rewrite definition names inside a whole-file inclusion.

    Vendor files sometimes name a card for their own catalogue rather than for
    the part: the measured 1N4148 model is `D1N4148/TEMP`, and TI's LM358
    macromodel is `LM358/NS`. Users search for the part number, so the exposed
    name is canonicalised here.

    Only `.model`, `.subckt` and `.ends` header lines are touched; parameters
    and body text are left exactly as upstream wrote them.
    """
    if not renames:
        return body

    mapping = {old.upper(): new for old, new in renames}
    applied: set[str] = set()
    out = []
    for line in body.splitlines():
        match = DEFINITION_HEADER.match(line)
        if match:
            key = match.group("name").upper()
            if key in mapping:
                applied.add(key)
                line = f"{match.group('lead')}{mapping[key]}{match.group('rest')}"
        out.append(line)

    missing = sorted(set(mapping) - applied)
    if missing:
        raise BuildError(f"rename target(s) not found in file: {', '.join(missing)}")
    return "\n".join(out)


def rename_model(block: str, new_name: str) -> str:
    lines = block.splitlines()
    match = DEFINITION_NAME.match(lines[0])
    if not match:
        raise BuildError(f"cannot rename, unexpected header: {lines[0]!r}")
    lines[0] = f"{match.group('lead')}{new_name}{match.group('rest')}"
    return "\n".join(lines)


def collect_cards(selection: Selection, blocked: set[tuple[str, str]]) -> list[Card]:
    cards: list[Card] = []

    for entry in selection.files:
        pack, path = entry["pack"], entry["path"]
        check_allowed(pack, path, blocked)
        body = read_text(source_path(pack, path)).rstrip("\n")
        renames = [list(pair) for pair in entry.get("rename", [])]
        try:
            body = rename_definitions(body, renames)
        except BuildError as exc:
            raise BuildError(f"{pack}/{path}: {exc}") from exc

        parts = list(entry.get("parts", []))
        renamed_from = ", ".join(old for old, _ in renames) if renames else None
        cards.append(
            Card(
                category=entry["category"],
                pack=pack,
                path=path,
                line=1,
                parts=parts,
                note=entry.get("note", ""),
                body=body,
                renamed_from=renamed_from,
            )
        )

    for entry in selection.models:
        pack, path = entry["pack"], entry["path"]
        check_allowed(pack, path, blocked)
        source_name = entry["source_name"]
        text = read_text(source_path(pack, path))
        block, line = extract_model(text, source_name)

        exposed = entry.get("name", source_name)
        renamed_from = None
        if exposed.upper() != source_name.upper():
            block = rename_model(block, exposed)
            renamed_from = source_name

        cards.append(
            Card(
                category=entry["category"],
                pack=pack,
                path=path,
                line=line,
                parts=[exposed],
                note=entry.get("note", ""),
                body=block,
                renamed_from=renamed_from,
            )
        )

    return cards


def render_category(category: str, cards: list[Card]) -> str:
    title = CATEGORY_TITLES.get(category, category)
    parts = sorted({p for card in cards for p in card.parts}, key=str.upper)

    out = [
        "*" + "=" * 78,
        f"* RSpice built-in library - {title}",
        "*" + "=" * 78,
        "*",
        "* Generated by tools/models/build_builtin.py - do not edit by hand.",
        "* Every card below is copied verbatim from a pack under models/spice/;",
        "* edit models/spice/builtin/selection.toml and regenerate instead.",
        "*",
        f"* Parts: {', '.join(parts)}",
        "*",
        "",
    ]

    for card in sorted(cards, key=lambda c: (c.parts[0].upper() if c.parts else "")):
        out.append("*" + "-" * 78)
        if card.parts:
            out.append(f"* {', '.join(card.parts)}")
        if card.note:
            out.append(f"* {card.note}")
        origin = f"* Source: {card.pack}/{card.path}"
        if card.line > 1:
            origin += f":{card.line}"
        out.append(origin)
        if card.renamed_from:
            out.append(
                f"* Renamed from upstream '{card.renamed_from}'; parameters unchanged."
            )
        out.append("*" + "-" * 78)
        out.append(card.body)
        out.append("")

    return "\n".join(out).rstrip("\n") + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail if stale")
    args = parser.parse_args()

    try:
        blocked = restricted_sources()
        selection = load_selection()
        cards = collect_cards(selection, blocked)
    except BuildError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    by_category: dict[str, list[Card]] = {}
    for card in cards:
        by_category.setdefault(card.category, []).append(card)

    rendered = {
        category: render_category(category, group)
        for category, group in by_category.items()
    }

    if args.check:
        stale = []
        existing = {p.stem for p in LIB_DIR.glob("*.lib")} if LIB_DIR.is_dir() else set()
        if existing != set(rendered):
            stale.append(f"category set differs: {sorted(existing)} vs {sorted(rendered)}")
        for category, text in rendered.items():
            target = LIB_DIR / f"{category}.lib"
            # Compared as bytes: some vendored cards carry lone CR characters,
            # and text-mode newline translation would rewrite them on the
            # round-trip, reporting an identical file as stale.
            if not target.exists() or target.read_bytes() != text.encode("utf-8"):
                stale.append(f"{target.relative_to(REPO_ROOT).as_posix()} is stale")
        if stale:
            print(
                "\n".join(stale) + "\nrun: python tools/models/build_builtin.py",
                file=sys.stderr,
            )
            return 1
        print(f"up to date - {len(cards)} cards in {len(rendered)} categories")
        return 0

    LIB_DIR.mkdir(parents=True, exist_ok=True)
    for stale in LIB_DIR.glob("*.lib"):
        if stale.stem not in rendered:
            stale.unlink()
    total_bytes = 0
    for category, text in sorted(rendered.items()):
        target = LIB_DIR / f"{category}.lib"
        # Written as bytes so the vendored cards land byte for byte, including
        # any legacy line endings they carry.
        encoded = text.encode("utf-8")
        target.write_bytes(encoded)
        total_bytes += len(encoded)
        parts = sum(len(c.parts) for c in by_category[category])
        print(f"  {category:<8} {len(by_category[category]):>3} cards, {parts:>3} parts")

    print(f"{len(cards)} cards -> {LIB_DIR.relative_to(REPO_ROOT).as_posix()} "
          f"({total_bytes / 1024:.1f} KB)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
