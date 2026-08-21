"""Audit the model trees for redistribution restrictions.

Three checks run on every invocation.

1. models/spice/ per-file findings. MANIFEST.toml records licensing at pack
   granularity; this answers "which files, exactly". It scans every file under
   models/spice/ for restriction language and writes a per-file finding table.
   Release packaging refuses an allowlisted pack if any file in that pack is
   marked restricted. For an all-RSpice-authored tree the expected result is
   zero findings, so a card pasted in from an outside source cannot carry
   restriction terms into the shipped library unnoticed.

2. models/veriloga/ provenance. Verilog-A sources are vendored third-party
   releases, so each top-level model directory must carry a PROVENANCE.md whose
   text declares the license the vendored copy arrives under. The convention is
   a line that begins with "License".

3. Non-commercial and no-derivatives terms anywhere under models/. RSpice ships
   commercially; a source carrying an NC or ND grant cannot be vendored here at
   all, promoted or not. Any hit fails the audit outright rather than being
   recorded as a finding row.

Usage:
    python tools/models/license_audit.py           write LICENSE-AUDIT.tsv
    python tools/models/license_audit.py --check   fail if the table is stale
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
MODELS_ROOT = REPO_ROOT / "models"
SPICE_ROOT = MODELS_ROOT / "spice"
VERILOGA_ROOT = MODELS_ROOT / "veriloga"
AUDIT = SPICE_ROOT / "LICENSE-AUDIT.tsv"

PROVENANCE = "PROVENANCE.md"
# The declaration convention across models/veriloga/: a line that opens with
# "License" and names the terms the vendored copy arrives under.
LICENSE_DECLARATION = re.compile(r"^[ \t]*License\b", re.IGNORECASE | re.MULTILINE)

# Non-commercial and no-derivatives grants. RSpice ships commercially, so these
# terms are disqualifying wherever they appear under models/ - there is no
# quarantine directory that makes them acceptable to vendor.
NONCOMMERCIAL_PATTERNS = (
    re.compile(r"CC[-\s]?BY[-\s]?N[CD]", re.IGNORECASE),
    re.compile(r"NonCommercial", re.IGNORECASE),
    re.compile(r"NoDerivatives", re.IGNORECASE),
    re.compile(r"\bN[CD][-\s]?4\.0\b", re.IGNORECASE),
)

SKIP_SUFFIXES = {
    ".png", ".jpg", ".jpeg", ".gif", ".svg", ".pdf", ".gds", ".gz", ".zip",
    ".7z", ".xls", ".xlsx", ".doc", ".docx", ".exe", ".dll", ".data",
}

DEFINITION = re.compile(r"^[ \t]*\.(model|subckt)\b", re.IGNORECASE)


@dataclass(frozen=True)
class Marker:
    """One restriction pattern and what a hit on it means."""

    id: str
    severity: str
    pattern: re.Pattern[str]
    description: str

    def exempt(self, line: str) -> bool:
        """Suppress hits that are boilerplate rather than a restriction."""
        if self.id != "written-permission":
            return False
        # The BSD-3 and Apache no-endorsement clauses both end in "without
        # prior written permission". That governs use of the licensor's name,
        # not use of the software, and every permissive pack carries it.
        lowered = line.lower()
        return "endorse" in lowered or "promote" in lowered


MARKERS = (
    Marker(
        id="commercial-use-restricted",
        severity="restricted",
        pattern=re.compile(r"commercial use or resale restricted", re.IGNORECASE),
        description=(
            "Symmetry/MODPEX generated model. Header asserts unpublished "
            "licensed software containing proprietary information and "
            "restricts commercial use or resale under an agreement RSpice "
            "does not hold."
        ),
    ),
    Marker(
        id="confidential",
        severity="restricted",
        pattern=re.compile(r"\bconfidential\b", re.IGNORECASE),
        description=(
            "File is marked confidential by its originator. Not material a "
            "third party may redistribute."
        ),
    ),
    Marker(
        id="written-permission",
        severity="restricted",
        pattern=re.compile(r"without\s+(?:the\s+)?(?:prior\s+)?written\s+permission", re.IGNORECASE),
        description=(
            "Terms require the originator's written permission for an act the "
            "file describes (excluding the standard no-endorsement clause)."
        ),
    ),
    Marker(
        id="unpublished-proprietary",
        severity="restricted",
        pattern=re.compile(
            r"unpublished licensed software|contains proprietary information",
            re.IGNORECASE,
        ),
        description=(
            "Explicit proprietary-rights assertion. Usually accompanies the "
            "Symmetry commercial-use restriction, but the header also appears "
            "without it — an assertion that the material is unpublished "
            "licensed software is on its own reason enough not to redistribute."
        ),
    ),
)


def read_text(path: Path) -> str | None:
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


@dataclass
class Finding:
    pack: str
    path: str
    marker: str
    severity: str
    line: int
    evidence: str
    definitions: int


def restriction_in_text(text: str) -> Marker | None:
    """The first `restricted` marker present in *text*, if any."""
    for line in text.splitlines():
        for marker in MARKERS:
            if marker.severity != "restricted":
                continue
            if marker.pattern.search(line) and not marker.exempt(line):
                return marker
    return None


def restriction_in_file(path: Path) -> Marker | None:
    """The first `restricted` marker in a file, if it is readable text."""
    text = read_text(path)
    return None if text is None else restriction_in_text(text)


def audit_file(pack: str, pack_dir: Path, path: Path) -> list[Finding]:
    text = read_text(path)
    if text is None:
        return []

    lines = text.splitlines()
    definitions = sum(1 for line in lines if DEFINITION.match(line))
    rel = path.relative_to(pack_dir).as_posix()

    findings: list[Finding] = []
    seen: set[str] = set()
    for number, line in enumerate(lines, start=1):
        for marker in MARKERS:
            if marker.id in seen or not marker.pattern.search(line):
                continue
            if marker.exempt(line):
                continue
            seen.add(marker.id)
            findings.append(
                Finding(
                    pack=pack,
                    path=rel,
                    marker=marker.id,
                    severity=marker.severity,
                    line=number,
                    evidence=" ".join(line.split())[:160],
                    definitions=definitions,
                )
            )
    return findings


def audit_tree() -> list[Finding]:
    findings: list[Finding] = []
    for pack_toml in sorted(SPICE_ROOT.rglob("pack.toml")):
        pack_dir = pack_toml.parent
        pack = pack_dir.relative_to(SPICE_ROOT).as_posix()
        for path in sorted(pack_dir.rglob("*"), key=lambda p: p.as_posix()):
            if not path.is_file() or path.suffix.lower() in SKIP_SUFFIXES:
                continue
            if path.name in {"pack.toml", "LICENSE", "LICENSE-NOTE.md", "README.md"}:
                continue
            findings.extend(audit_file(pack, pack_dir, path))
    findings.sort(key=lambda f: (f.pack, f.path, f.marker))
    return findings


def veriloga_provenance_gaps() -> list[str]:
    """Top-level models/veriloga/ directories missing a license declaration."""
    gaps: list[str] = []
    for path in sorted(VERILOGA_ROOT.iterdir(), key=lambda p: p.name):
        if not path.is_dir():
            continue
        rel = path.relative_to(REPO_ROOT).as_posix()
        provenance = path / PROVENANCE
        if not provenance.is_file():
            gaps.append(f"{rel}/ has no {PROVENANCE}")
            continue
        text = read_text(provenance)
        if text is None or not LICENSE_DECLARATION.search(text):
            gaps.append(
                f"{rel}/{PROVENANCE} declares no license "
                '(expected a line beginning with "License")'
            )
    return gaps


def noncommercial_hits() -> list[str]:
    """Every NC/ND marker anywhere under models/, as `path:line evidence`."""
    hits: list[str] = []
    for path in sorted(MODELS_ROOT.rglob("*"), key=lambda p: p.as_posix()):
        if not path.is_file() or path.suffix.lower() in SKIP_SUFFIXES:
            continue
        text = read_text(path)
        if text is None:
            continue
        rel = path.relative_to(REPO_ROOT).as_posix()
        for number, line in enumerate(text.splitlines(), start=1):
            if any(pattern.search(line) for pattern in NONCOMMERCIAL_PATTERNS):
                hits.append(f"{rel}:{number} {' '.join(line.split())[:120]}")
                break
    return hits


def restricted_files(findings: list[Finding]) -> set[tuple[str, str]]:
    """Pack-relative files that must not be shipped or embedded."""
    return {(f.pack, f.path) for f in findings if f.severity == "restricted"}


def render(findings: list[Finding]) -> str:
    out = [
        "# Generated by tools/models/license_audit.py - do not edit by hand.",
        "# Per-file redistribution findings across models/spice/.",
        "# severity=restricted means the file must not ship in a distributed",
        "# build and must not be embedded in the binary.",
        "# pack\tpath\tmarker\tseverity\tline\tdefinitions\tevidence",
    ]
    out.extend(
        f"{f.pack}\t{f.path}\t{f.marker}\t{f.severity}\t{f.line}\t{f.definitions}\t{f.evidence}"
        for f in findings
    )
    return "\n".join(out) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail if stale")
    args = parser.parse_args()

    blockers = 0
    gaps = veriloga_provenance_gaps()
    if gaps:
        blockers += 1
        print(
            "models/veriloga/ model directories without a declared license:",
            file=sys.stderr,
        )
        for gap in gaps:
            print(f"  {gap}", file=sys.stderr)

    hits = noncommercial_hits()
    if hits:
        blockers += 1
        print(
            "non-commercial or no-derivatives terms under models/ "
            "(these may not be vendored in a commercial simulator):",
            file=sys.stderr,
        )
        for hit in hits:
            print(f"  {hit}", file=sys.stderr)

    findings = audit_tree()
    rendered = render(findings)

    if blockers:
        return 1

    if args.check:
        if not AUDIT.exists() or AUDIT.read_text(encoding="utf-8") != rendered:
            print(
                f"stale {AUDIT.relative_to(REPO_ROOT).as_posix()}\n"
                "run: python tools/models/license_audit.py",
                file=sys.stderr,
            )
            return 1
        print(
            f"up to date - {len(findings)} findings; "
            "models/veriloga/ provenance and models/ NC/ND scan clean"
        )
        return 0

    AUDIT.write_text(rendered, encoding="utf-8", newline="\n")

    restricted = restricted_files(findings)
    by_pack: dict[str, int] = {}
    defs_blocked = 0
    counted: set[tuple[str, str]] = set()
    for finding in findings:
        if finding.severity != "restricted":
            continue
        key = (finding.pack, finding.path)
        if key in counted:
            continue
        counted.add(key)
        by_pack[finding.pack] = by_pack.get(finding.pack, 0) + 1
        defs_blocked += finding.definitions

    print(f"{len(findings)} findings across {len(restricted)} restricted files")
    for pack, count in sorted(by_pack.items(), key=lambda kv: -kv[1]):
        print(f"  {count:>5} restricted files  {pack}")
    print(f"  {defs_blocked} model/subcircuit definitions sit in restricted files")
    print(f"  -> {AUDIT.relative_to(REPO_ROOT).as_posix()}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
