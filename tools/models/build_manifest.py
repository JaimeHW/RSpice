"""Index the SPICE model tree.

Reads every ``pack.toml`` under models/spice/, scans the packs for SPICE
definitions, and regenerates the development indexes plus two release indexes:

    models/spice/MANIFEST.toml   pack-level index for humans and packaging:
                                 provenance, license tier, redistribution flag
    models/spice/PACKS.tsv       the same pack data in the tab-separated form
                                 rspice-core reads for runtime discovery
    models/spice/CATALOG.tsv     part-level index: one row per .MODEL/.SUBCKT,
                                 giving name, kind, device type, pack and origin
    models/spice/SHIPPED-PACKS.tsv
    models/spice/SHIPPED-CATALOG.tsv
                                 runtime indexes containing only packs named by
                                 models/spice/SHIPPING.toml

SHIPPING.toml is the sole release allowlist. The complete indexes remain useful
for explicit developer corpus inspection, while normal product discovery reads
only the shipped indexes.

Run tools/models/license_audit.py FIRST. The catalog's per-file `restricted`
column is joined from LICENSE-AUDIT.tsv, so regenerating in the other order
stamps the catalog with a stale view of what may be shipped.

Usage:
    python tools/models/license_audit.py             (must run first)
    python tools/models/build_manifest.py
    python tools/models/build_manifest.py --check    fail if any is stale
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
PACKS = SPICE_ROOT / "PACKS.tsv"
CATALOG = SPICE_ROOT / "CATALOG.tsv"
SHIPPING = SPICE_ROOT / "SHIPPING.toml"
SHIPPED_PACKS = SPICE_ROOT / "SHIPPED-PACKS.tsv"
SHIPPED_CATALOG = SPICE_ROOT / "SHIPPED-CATALOG.tsv"

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
# `.ends` closes a subcircuit; the deck terminator `.end` must not match it, so
# the word boundary is load-bearing. The optional trailing name is ignored
# because nesting is tracked by depth rather than by matching names.
SUBCKT_END = re.compile(r"^[ \t]*\.ends\b", re.IGNORECASE)

# .MODEL type token -> canonical device class used by the catalog.
#
# One class per device family, not per token spelling: a family the engine
# accepts under several tokens is one thing to browse for, and splitting it
# would give the catalog a class no chip in the client offers. That is why all
# five VDMOS spellings answer `mosfet-vdmos`, and why `lpnp` answers `bjt-pnp`.
#
# Mirrored in crates/rspice-ui/src/workbench/surfaces/models/manager/
# held_parts.rs, which classifies the cards a project parsed for itself and
# never handed to this generator. A test there reads this literal and refuses
# to let the two drift, so an entry added here is added there in the same
# order and spelling.
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
    "nvdmos": "mosfet-vdmos",
    "pvdmos": "mosfet-vdmos",
    "vdmosn": "mosfet-vdmos",
    "vdmosp": "mosfet-vdmos",
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
    # Definitions at file scope, i.e. excluding those declared inside a .SUBCKT
    # body. These are the ones a user can reference from a netlist, so they are
    # what the part count in the UI and the release notes should quote.
    models_top: int = 0
    subckts_top: int = 0
    rows: list[tuple[str, str, str, str, str, int, str]] = field(default_factory=list)


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
        # Macromodels declare private helper cards in their own body — `.model DX
        # D(...)` inside an op-amp subcircuit is the dominant idiom, and `DX`
        # alone occurs thousands of times across these corpora. Those names are
        # not addressable from a netlist, so the depth at which a definition
        # appears is recorded and the catalog stays searchable.
        depth = 0
        for lineno, line in enumerate(text.splitlines(), start=1):
            match = DEFINITION.match(line)
            if match:
                kind = match.group(1).lower()
                name = match.group(2)
                scope = "nested" if depth else "top"
                # `.model Q2N3904 npn(is=6.7f ...)` is legal: the parameter list
                # may open with no separating space, so the type token ends at
                # the first paren rather than at whitespace.
                type_token = (match.group(3) or "").lower().split("(", 1)[0]
                if kind == "model":
                    index.models += 1
                    index.models_top += depth == 0
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
                    index.subckts_top += depth == 0
                    device = "subckt"
                    depth += 1
                index.rows.append((name, kind, device, index.id, rel, lineno, scope))
                continue
            if SUBCKT_END.match(line) and depth:
                depth -= 1
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


def load_shipping_ids() -> list[str]:
    with SHIPPING.open("rb") as handle:
        data = tomllib.load(handle)
    if data.get("schema") != 1:
        raise ValueError(f"{SHIPPING} must declare schema = 1")
    ids = data.get("packs")
    if not isinstance(ids, list) or not ids:
        raise ValueError(f"{SHIPPING} must contain a non-empty packs array")
    if any(not isinstance(pack_id, str) or not pack_id.strip() for pack_id in ids):
        raise ValueError(f"{SHIPPING} pack IDs must be non-empty strings")
    if len(set(ids)) != len(ids):
        raise ValueError(f"{SHIPPING} contains duplicate pack IDs")
    return ids


def select_shipped_packs(packs: list[PackIndex], shipping_ids: list[str]) -> list[PackIndex]:
    by_id = {pack.id: pack for pack in packs}
    missing = [pack_id for pack_id in shipping_ids if pack_id not in by_id]
    if missing:
        raise ValueError(
            f"{SHIPPING} names unknown pack(s): {', '.join(sorted(missing))}"
        )
    selected = [by_id[pack_id] for pack_id in shipping_ids]
    denied = [pack.id for pack in selected if not pack.redistributable]
    if denied:
        raise ValueError(
            "shipping allowlist contains pack(s) whose own metadata denies "
            f"redistribution: {', '.join(sorted(denied))}"
        )
    return selected


def _toml_str(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def _toml_list(values: list[str]) -> str:
    return "[" + ", ".join(_toml_str(v) for v in values) + "]" if values else "[]"


def render_manifest(packs: list[PackIndex], shipped_packs: list[PackIndex]) -> str:
    by_tier: dict[str, int] = {}
    for pack in packs:
        by_tier[pack.tier] = by_tier.get(pack.tier, 0) + 1

    total_models = sum(p.models for p in packs)
    total_subckts = sum(p.subckts for p in packs)
    total_models_top = sum(p.models_top for p in packs)
    total_subckts_top = sum(p.subckts_top for p in packs)
    total_files = sum(p.files for p in packs)
    total_bytes = sum(p.bytes for p in packs)
    lines = [
        "# Generated by tools/models/build_manifest.py - do not edit by hand.",
        "#",
        "# Index of every SPICE model pack under models/spice/. Each pack",
        "# directory declares itself with a pack.toml.",
        "#",
        "# SHIPPING.toml is the sole release allowlist. Pack licence fields are",
        "# descriptive and cannot add themselves to a product artifact.",
        "",
        "[summary]",
        f"packs = {len(packs)}",
        f"files = {total_files}",
        f"bytes = {total_bytes}",
        f"models = {total_models}",
        f"subcircuits = {total_subckts}",
        # The `_top` figures exclude definitions private to a .SUBCKT body.
        # Quote these as the part count: the difference is macromodel internals,
        # which no netlist can reference.
        f"models_top_level = {total_models_top}",
        f"subcircuits_top_level = {total_subckts_top}",
        f"shipped_packs = {len(shipped_packs)}",
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
            f"models_top_level = {pack.models_top}",
            f"subcircuits_top_level = {pack.subckts_top}",
            f"lib_sections = {pack.sections}",
            f"digest = {_toml_str(pack.digest)}",
            "",
        ]
    return "\n".join(lines)


def render_packs(packs: list[PackIndex], *, shipped: bool = False) -> str:
    """Pack index in the same tab-separated form as the catalog.

    MANIFEST.toml is the human- and packaging-facing document. This carries the
    identical data in the form rspice-core already reads, so pack discovery
    needs no TOML parser in the engine and the two cannot drift.
    """
    out = [
        "# Generated by tools/models/build_manifest.py - do not edit by hand.",
        (
            "# Runtime pack index for the SHIPPING.toml allowlist."
            if shipped
            else "# Full developer pack index; MANIFEST.toml carries the same data."
        ),
        "# models/subcircuits count every definition; the _top columns exclude",
        "# those private to a .SUBCKT body and are the user-facing part counts.",
        "# id\tcategory\tpath\ttier\tspdx\tredistributable\tentry\tmodels"
        "\tsubcircuits\tmodels_top\tsubcircuits_top\tfiles\tbytes\tdevices\tname",
    ]
    for pack in packs:
        out.append(
            "\t".join(
                (
                    pack.id,
                    pack.category,
                    pack.path,
                    pack.tier,
                    pack.spdx or "NOASSERTION",
                    "1" if pack.redistributable else "0",
                    pack.entry,
                    str(pack.models),
                    str(pack.subckts),
                    str(pack.models_top),
                    str(pack.subckts_top),
                    str(pack.files),
                    str(pack.bytes),
                    ",".join(pack.devices),
                    pack.name,
                )
            )
        )
    return "\n".join(out) + "\n"


def restricted_paths() -> set[tuple[str, str]]:
    """(pack path, file path) pairs the licence audit marked non-shippable.

    Read from the audit rather than recomputed so the two artifacts cannot
    disagree about which files are restricted. Absent audit means nothing is
    flagged, which is the correct behaviour for a fresh tree.
    """
    audit = SPICE_ROOT / "LICENSE-AUDIT.tsv"
    if not audit.exists():
        return set()
    blocked = set()
    for line in audit.read_text(encoding="utf-8").splitlines():
        if line.startswith("#") or not line.strip():
            continue
        fields = line.split("\t")
        if len(fields) > 3 and fields[3] == "restricted":
            blocked.add((fields[0], fields[1]))
    return blocked


def render_catalog(packs: list[PackIndex], *, shipped: bool = False) -> str:
    blocked = restricted_paths()
    by_id = {pack.id: pack.path for pack in packs}

    rows: list[tuple[str, str, str, str, str, int, str]] = []
    for pack in packs:
        rows.extend(pack.rows)
    # Sort case-insensitively by part name so lookups and diffs are stable.
    rows.sort(key=lambda r: (r[0].lower(), r[3], r[4], r[5]))

    out = [
        "# Generated by tools/models/build_manifest.py - do not edit by hand.",
        (
            "# One row per SPICE definition in SHIPPING.toml allowlisted packs."
            if shipped
            else "# One row per SPICE definition found in models/spice/."
        ),
        "# restricted=1 means the defining file is marked non-shippable by",
        "# tools/models/license_audit.py, even where its pack is otherwise usable.",
        "# scope=top is a definition at file scope, addressable from a netlist.",
        "# scope=nested is private to the enclosing .SUBCKT — retained so a card",
        "# can still be traced to its source, but not a part a user can select.",
        "# name\tkind\tdevice\tpack\tpath\tline\trestricted\tscope",
    ]
    out.extend(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}".format(
            name,
            kind,
            device,
            pack,
            path,
            line,
            "1" if (by_id.get(pack, ""), path) in blocked else "0",
            scope,
        )
        for name, kind, device, pack, path, line, scope in rows
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

    try:
        packs = load_packs()
        if not packs:
            print(f"no packs found under {SPICE_ROOT}", file=sys.stderr)
            return 2
        shipped_packs = select_shipped_packs(packs, load_shipping_ids())
    except (OSError, KeyError, TypeError, ValueError, tomllib.TOMLDecodeError) as error:
        print(f"model manifest configuration error: {error}", file=sys.stderr)
        return 2

    manifest = render_manifest(packs, shipped_packs)
    packs_index = render_packs(packs)
    catalog = render_catalog(packs)
    shipped_packs_index = render_packs(shipped_packs, shipped=True)
    shipped_catalog = render_catalog(shipped_packs, shipped=True)

    if args.check:
        stale = []
        for path, rendered in (
            (MANIFEST, manifest),
            (PACKS, packs_index),
            (CATALOG, catalog),
            (SHIPPED_PACKS, shipped_packs_index),
            (SHIPPED_CATALOG, shipped_catalog),
        ):
            # newline="" disables universal-newline translation on read. Without
            # it a CRLF copy compares equal to the LF text these tools emit, and
            # the check silently passes on a file that is not byte-identical.
            if not path.exists():
                stale.append(path.relative_to(REPO_ROOT).as_posix())
                continue
            with path.open("r", encoding="utf-8", newline="") as handle:
                if handle.read() != rendered:
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

    # newline="\n" rather than the platform default: these files are committed
    # and compared byte-for-byte by --check, so a regeneration must not depend
    # on which OS ran it.
    MANIFEST.write_text(manifest, encoding="utf-8", newline="\n")
    PACKS.write_text(packs_index, encoding="utf-8", newline="\n")
    CATALOG.write_text(catalog, encoding="utf-8", newline="\n")
    SHIPPED_PACKS.write_text(shipped_packs_index, encoding="utf-8", newline="\n")
    SHIPPED_CATALOG.write_text(shipped_catalog, encoding="utf-8", newline="\n")

    total_models = sum(p.models for p in packs)
    total_subckts = sum(p.subckts for p in packs)
    total_top = sum(p.models_top + p.subckts_top for p in packs)
    print(f"{len(packs)} packs indexed")
    print(f"  {total_models} models, {total_subckts} subcircuits")
    print(f"  {total_top} addressable at file scope; the rest are subcircuit-private")
    print(f"  -> {MANIFEST.relative_to(REPO_ROOT)}")
    print(f"  -> {PACKS.relative_to(REPO_ROOT)}")
    print(f"  -> {CATALOG.relative_to(REPO_ROOT)}")
    print(f"  -> {SHIPPED_PACKS.relative_to(REPO_ROOT)}")
    print(f"  -> {SHIPPED_CATALOG.relative_to(REPO_ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
