"""Audit the SPICE model tree for per-file redistribution restrictions.

MANIFEST.toml records licensing at pack granularity. This tool answers "which
files, exactly": it scans every file under models/spice/ for restriction
language and writes a per-file finding table. Release packaging refuses an
allowlisted pack if any file in that pack is marked restricted.

For an all-RSpice-authored tree the expected result is zero findings. The scan
remains a CI gate so that a card pasted in from an outside source cannot carry
restriction terms into the shipped library unnoticed.

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
SPICE_ROOT = REPO_ROOT / "models" / "spice"
AUDIT = SPICE_ROOT / "LICENSE-AUDIT.tsv"

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

    findings = audit_tree()
    rendered = render(findings)

    if args.check:
        if not AUDIT.exists() or AUDIT.read_text(encoding="utf-8") != rendered:
            print(
                f"stale {AUDIT.relative_to(REPO_ROOT).as_posix()}\n"
                "run: python tools/models/license_audit.py",
                file=sys.stderr,
            )
            return 1
        print(f"up to date - {len(findings)} findings")
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
