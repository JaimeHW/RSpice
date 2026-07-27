"""Index the vendored SPICE model tree.

Reads every ``pack.toml`` under models/spice/, scans the packs for SPICE
definitions, and regenerates two committed artifacts:

    models/spice/MANIFEST.toml   pack-level index: provenance, license tier,
                                 redistribution flag, file and definition counts
    models/spice/CATALOG.tsv     part-level index: one row per .MODEL/.SUBCKT,
                                 giving name, kind, device type, pack and origin

MANIFEST.toml is what a packaging step reads to decide which packs may ship.
CATALOG.tsv is what the application reads to browse and resolve parts without
opening 140 MB of model files.

Usage:
    python tools/models/build_manifest.py
    python tools/models/build_manifest.py --check    fail if either is stale
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
MANIFEST = SPICE_ROOT / "MANIFEST.toml"
CATALOG = SPICE_ROOT / "CATALOG.tsv"

# Files that can hold SPICE definitions. The tree carries documentation,
# measurement data and tool scripts alongside the model cards.
SKIP_SUFFIXES = {
    ".png", ".jpg", ".jpeg", ".gif", ".svg", ".pdf", ".gds", ".gz", ".zip",
    ".7z", ".xls", ".xlsx", ".doc", ".docx", ".exe", ".dll", ".data",
}

DEFINITION = re.compile(
    r"^[ \t]*\.(model|subckt)[ \t]+(\S+)(?:[ \t]+(\S+))?",
    re.IGNORECASE,
)
LIB_SECTION = re.compile(r"^[ \t]*\.lib[ \t]+(\S+)[ \t]*$", re.IGNORECASE)

# .MODEL type token -> canonical device class used by the catalog.
DEVICE_CLASS = {
    "d": "diode",
    "npn": "bjt-npn",
    "pnp": "bjt-pnp",
    "lpnp": "bjt-pnp",
    "njf": "jfet-n",
    "pjf": "jfet-p",
    "nmos": "mosfet-n",
    "pmos": "mosfet-p",
    "nmf": "mesfet-n",
    "pmf": "mesfet-p",
    "nsw": "switch",
    "sw": "switch",
    "vswitch": "switch",
    "csw": "switch-current",
    "iswitch": "switch-current",
    "r": "resistor",
    "res": "resistor",
    "c": "capacitor",
    "cap": "capacitor",
    "l": "inductor",
    "ind": "inductor",
    "k": "coupling",
    "core": "magnetic-core",
    "vdmos": "mosfet-vdmos",
    "txl": "transmission-line",
    "cpl": "transmission-line",
    "ltra": "transmission-line",
    "urc": "distributed-rc",
    "pzt": "piezo",
}


@dataclass
class PackIndex:
    id: str
    name: str
    category: str
    originator: str
    spdx: str
    tier: str
    redistributable: bool
    url: str
    pin: str
    path: str
    files: int
    bytes: int
    digest: str
    entry: str
    devices: list[str]
    corners: list[str]
    models: int = 0
    subckts: int = 0
    sections: int = 0
    rows: list[tuple[str, str, str, str, str, int]] = field(default_factory=list)


def read_text(path: Path) -> str | None:
    """Read a model file, tolerating the legacy encodings in these corpora."""
    try:
        raw = path.read_bytes()
    except OSError:
        return None
    if b"\x00" in raw[:4096]:
        return None
    for encoding in ("utf-8", "cp1252", "latin-1"):
        try:
            return raw.decode(encoding)
        except UnicodeDecodeError:
            continue
    return None


def scan_pack(pack_dir: Path, index: PackIndex, root_name: str) -> None:
    root = pack_dir / root_name
    if not root.is_dir():
        return
    for path in sorted(root.rglob("*"), key=lambda p: p.as_posix()):
        if not path.is_file():
            continue
        # File and byte counts describe the whole pack, so they are taken from
        # the walk rather than from pack.toml, which locally authored packs do
        # not have a sync step to keep honest.
        index.files += 1
        index.bytes += path.stat().st_size
        if path.suffix.lower() in SKIP_SUFFIXES:
            continue
        text = read_text(path)
        if text is None:
            continue
        rel = path.relative_to(pack_dir).as_posix()
        for lineno, line in enumerate(text.splitlines(), start=1):
            match = DEFINITION.match(line)
            if match:
                kind = match.group(1).lower()
                name = match.group(2)
                # `.model Q2N3904 npn(is=6.7f ...)` is legal: the parameter list
                # may open with no separating space, so the type token ends at
                # the first paren rather than at whitespace.
                type_token = (match.group(3) or "").lower().split("(", 1)[0]
                if kind == "model":
                    index.models += 1
                    device = DEVICE_CLASS.get(type_token)
                    if device is None:
                        # PSpice digital primitives (d_and, d_dff, ...) and the
                        # XSPICE u-prefixed behavioural family are numerous
                        # enough to fold into two classes rather than enumerate.
                        if type_token.startswith("d_"):
                            device = "digital"
                        elif type_token.startswith("u"):
                            device = "digital-behavioral"
                        else:
                            device = type_token or "unknown"
                else:
                    index.subckts += 1
                    device = "subckt"
                index.rows.append((name, kind, device, index.id, rel, lineno))
                continue
            if LIB_SECTION.match(line):
                index.sections += 1


def load_packs() -> list[PackIndex]:
    packs: list[PackIndex] = []
    for pack_toml in sorted(SPICE_ROOT.rglob("pack.toml")):
        with pack_toml.open("rb") as handle:
            data = tomllib.load(handle)
        pack = data["pack"]
        lic = data["license"]
        prov = data["provenance"]
        contents = data.get("contents", {})
        index = PackIndex(
            id=pack["id"],
            name=pack["name"],
            category=pack["category"],
            originator=pack.get("originator", ""),
            spdx=lic.get("spdx", ""),
            tier=lic.get("tier", ""),
            redistributable=bool(lic.get("redistributable", False)),
            url=prov.get("url", ""),
            pin=prov.get("commit") or prov.get("sha256", ""),
            path=pack_toml.parent.relative_to(SPICE_ROOT).as_posix(),
            files=0,
            bytes=0,
            digest=contents.get("digest", ""),
            entry=contents.get("entry", ""),
            devices=list(contents.get("devices", [])),
            corners=list(contents.get("corners", [])),
        )
        # Vendored packs keep upstream files under `upstream/`; packs authored
        # in this repo name their own content directory.
        scan_pack(pack_toml.parent, index, contents.get("root", "upstream"))
        packs.append(index)
    packs.sort(key=lambda p: (p.category, p.id))
    return packs


def _toml_str(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def _toml_list(values: list[str]) -> str:
    return "[" + ", ".join(_toml_str(v) for v in values) + "]" if values else "[]"


def render_manifest(packs: list[PackIndex]) -> str:
    by_tier: dict[str, int] = {}
    for pack in packs:
        by_tier[pack.tier] = by_tier.get(pack.tier, 0) + 1

    total_models = sum(p.models for p in packs)
    total_subckts = sum(p.subckts for p in packs)
    total_files = sum(p.files for p in packs)
    total_bytes = sum(p.bytes for p in packs)
    shippable = [p for p in packs if p.redistributable]

    lines = [
        "# Generated by tools/models/build_manifest.py - do not edit by hand.",
        "#",
        "# Index of every SPICE model pack vendored under models/spice/.",
        "# Packs are declared in sources.toml and materialized by sync_packs.py.",
        "#",
        "# license.tier and license.redistributable govern packaging: a build that",
        "# ships model data must include only packs it is entitled to redistribute.",
        "",
        "[summary]",
        f"packs = {len(packs)}",
        f"files = {total_files}",
        f"bytes = {total_bytes}",
        f"models = {total_models}",
        f"subcircuits = {total_subckts}",
        f"redistributable_packs = {len(shippable)}",
        f"tiers = {{ " + ", ".join(
            f"{tier} = {count}" for tier, count in sorted(by_tier.items())
        ) + " }",
        "",
    ]

    for pack in packs:
        lines += [
            "[[pack]]",
            f"id = {_toml_str(pack.id)}",
            f"name = {_toml_str(pack.name)}",
            f"category = {_toml_str(pack.category)}",
            f"originator = {_toml_str(pack.originator)}",
            f"path = {_toml_str(pack.path)}",
            f"url = {_toml_str(pack.url)}",
            f"pin = {_toml_str(pack.pin)}",
            f"spdx = {_toml_str(pack.spdx or 'NOASSERTION')}",
            f"tier = {_toml_str(pack.tier)}",
            f"redistributable = {str(pack.redistributable).lower()}",
            f"devices = {_toml_list(pack.devices)}",
            f"corners = {_toml_list(pack.corners)}",
            f"entry = {_toml_str(pack.entry)}",
            f"files = {pack.files}",
            f"bytes = {pack.bytes}",
            f"models = {pack.models}",
            f"subcircuits = {pack.subckts}",
            f"lib_sections = {pack.sections}",
            f"digest = {_toml_str(pack.digest)}",
            "",
        ]
    return "\n".join(lines)


def render_catalog(packs: list[PackIndex]) -> str:
    rows: list[tuple[str, str, str, str, str, int]] = []
    for pack in packs:
        rows.extend(pack.rows)
    # Sort case-insensitively by part name so lookups and diffs are stable.
    rows.sort(key=lambda r: (r[0].lower(), r[3], r[4], r[5]))

    out = [
        "# Generated by tools/models/build_manifest.py - do not edit by hand.",
        "# One row per SPICE definition found in models/spice/.",
        "# name\tkind\tdevice\tpack\tpath\tline",
    ]
    out.extend(
        f"{name}\t{kind}\t{device}\t{pack}\t{path}\t{line}"
        for name, kind, device, pack, path, line in rows
    )
    return "\n".join(out) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail if the committed artifacts are stale",
    )
    args = parser.parse_args()

    packs = load_packs()
    if not packs:
        print(f"no packs found under {SPICE_ROOT}", file=sys.stderr)
        return 2

    manifest = render_manifest(packs)
    catalog = render_catalog(packs)

    if args.check:
        stale = []
        for path, rendered in ((MANIFEST, manifest), (CATALOG, catalog)):
            if not path.exists() or path.read_text(encoding="utf-8") != rendered:
                stale.append(path.relative_to(REPO_ROOT).as_posix())
        if stale:
            print(
                "stale generated artifacts: " + ", ".join(stale)
                + "\nrun: python tools/models/build_manifest.py",
                file=sys.stderr,
            )
            return 1
        print(f"up to date - {len(packs)} packs")
        return 0

    MANIFEST.write_text(manifest, encoding="utf-8")
    CATALOG.write_text(catalog, encoding="utf-8")

    total_models = sum(p.models for p in packs)
    total_subckts = sum(p.subckts for p in packs)
    print(f"{len(packs)} packs indexed")
    print(f"  {total_models} models, {total_subckts} subcircuits")
    print(f"  -> {MANIFEST.relative_to(REPO_ROOT)}")
    print(f"  -> {CATALOG.relative_to(REPO_ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
