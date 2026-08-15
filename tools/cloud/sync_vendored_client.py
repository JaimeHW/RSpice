"""Vendor the RSpice Cloud client and pack-trust crates into this workspace.

The RSpice application consumes RSpice Cloud through the typed
`rspice-cloud-client` crate, and admits a downloaded model pack or catalog
snapshot only through `rspice-pack`. Both live in the private RSpice-Cloud
repository next to the API they are tested against. This workspace cannot take
a git dependency on that repository (the dependency-source policy in deny.toml
admits only crates.io, and every build would need repository credentials), so
the crate closure is vendored: an exact copy of the library sources, pinned to
the Cloud commit it was taken from.

Vendored trees are read-only in this repository. Behavioural authority stays
in RSpice-Cloud, where the crates are exercised against the real API handlers;
this workspace only compiles and consumes them. `--check` verifies the
vendored trees still match the recorded manifest, so any local edit or partial
sync fails CI instead of silently forking the client.

Usage:
    python tools/cloud/sync_vendored_client.py --source <RSpice-Cloud checkout>
    python tools/cloud/sync_vendored_client.py --check
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = ROOT / "tools" / "cloud" / "vendor-manifest.json"

# The vendored trust closure, in dependency order. `rspice-pack` depends on
# nothing internal and is not reachable from the client: it is vendored on its
# own terms, because a pack archive and a catalog snapshot must reach the same
# verdict in this application as they do in the service that signed them.
VENDORED_CRATES = (
    "rspice-pack",
    "rspice-cloud-domain",
    "rspice-cloud-contract",
    "rspice-cloud-client",
)

# Repository-level fixtures the crates' in-source unit tests include by
# relative path (../../../testdata/...). Test-only key material, not secrets.
VENDORED_TESTDATA = (
    "testdata/README.md",
    "testdata/license-test-private-key.pem",
    "testdata/license-test-public-jwks.json",
)

SOURCE_REPOSITORY = "JaimeHW/RSpice-Cloud"
SOURCE_ORIGINS = frozenset(
    {
        "https://github.com/jaimehw/rspice-cloud",
        "git@github.com:jaimehw/rspice-cloud",
        "ssh://git@github.com/jaimehw/rspice-cloud",
    }
)
CANONICAL_COMMIT = re.compile(r"^[0-9a-f]{40}$")

VENDOR_MARKER = (
    "# VENDORED from {repository}@{sha} — do not edit.\n"
    "# Re-sync with: python tools/cloud/sync_vendored_client.py --source <checkout>\n"
    "# The client's native dev-dependency section is stripped: its integration\n"
    "# tests stay in RSpice-Cloud, where they run against the real API handlers.\n"
)

# The client's integration tests (tests/, axum loopback servers) are not
# vendored, so the native dev-dependency section that exists only for them is
# stripped. Every other dev-dependency stays: the crates' in-source unit-test
# modules need them to compile under `--all-targets`.
NATIVE_DEV_DEPENDENCIES_HEADER = re.compile(
    r"^\[target\.'cfg\(not\(target_arch = \"wasm32\"\)\)'\.dev-dependencies\]\s*$"
)
SECTION_HEADER = re.compile(r"^\[")


def strip_native_dev_dependencies(manifest_text: str) -> str:
    """Remove the native-only dev-dependencies section from a Cargo.toml."""
    kept: list[str] = []
    skipping = False
    for line in manifest_text.splitlines(keepends=True):
        if NATIVE_DEV_DEPENDENCIES_HEADER.match(line):
            skipping = True
            continue
        if skipping and SECTION_HEADER.match(line):
            skipping = False
        if not skipping:
            kept.append(line)
    stripped = "".join(kept)
    # Collapse the trailing blank lines a removed final section leaves behind.
    return stripped.rstrip("\n") + "\n"


def crate_files(crate_root: Path) -> list[Path]:
    """The files a vendored crate consists of: manifest, sources, README."""
    files = [crate_root / "Cargo.toml"]
    readme = crate_root / "README.md"
    if readme.is_file():
        files.append(readme)
    src = crate_root / "src"
    if not src.is_dir():
        raise SystemExit(f"{crate_root} has no src/ directory")
    files.extend(sorted(path for path in src.rglob("*") if path.is_file()))
    return files


def sha256_bytes(payload: bytes) -> str:
    return hashlib.sha256(payload).hexdigest()


def source_origin_is_authoritative(value: str) -> bool:
    """Admit only credential-free canonical transports for the source repo."""
    normalized = value.strip().removesuffix("/").removesuffix(".git").lower()
    return normalized in SOURCE_ORIGINS


def git_output(source: Path, *arguments: str) -> str:
    return subprocess.run(
        ["git", "-C", str(source), *arguments],
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()


def admit_source(source: Path) -> str:
    """Bind a clean authoritative checkout to one canonical commit."""
    try:
        root = Path(git_output(source, "rev-parse", "--show-toplevel")).resolve()
        source_sha = git_output(source, "rev-parse", "HEAD")
        origin = git_output(source, "remote", "get-url", "origin")
        dirty = git_output(source, "status", "--porcelain=v1", "--untracked-files=all")
    except subprocess.CalledProcessError as error:
        raise SystemExit("source checkout failed Git provenance admission") from error
    if root != source:
        raise SystemExit("source path must be the RSpice-Cloud checkout root")
    if not CANONICAL_COMMIT.fullmatch(source_sha):
        raise SystemExit("source checkout HEAD is not a canonical commit")
    if not source_origin_is_authoritative(origin):
        raise SystemExit(f"source checkout origin is not {SOURCE_REPOSITORY}")
    if dirty:
        raise SystemExit("source checkout has uncommitted or untracked files; commit it before re-sync")
    return source_sha


def sync(source: Path) -> int:
    if not (source / "crates").is_dir():
        raise SystemExit(f"{source} does not look like an RSpice-Cloud checkout")
    source_sha = admit_source(source)

    marker = VENDOR_MARKER.format(repository=SOURCE_REPOSITORY, sha=source_sha)
    recorded: dict[str, str] = {}
    for crate in VENDORED_CRATES:
        crate_source = source / "crates" / crate
        crate_target = ROOT / "crates" / crate
        if crate_target.exists():
            shutil.rmtree(crate_target)
        for path in crate_files(crate_source):
            relative = path.relative_to(crate_source)
            # Every vendored file is text. Normalize to LF so the recorded
            # digests match both the git object store and Linux checkouts;
            # .gitattributes pins the vendored trees to eol=lf for Windows.
            payload = path.read_bytes().replace(b"\r\n", b"\n")
            if relative == Path("Cargo.toml"):
                manifest_text = payload.decode("utf-8")
                if crate == "rspice-cloud-client":
                    manifest_text = strip_native_dev_dependencies(manifest_text)
                payload = (marker + manifest_text).encode("utf-8")
            target = crate_target / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(payload)
            recorded[f"crates/{crate}/{relative.as_posix()}"] = sha256_bytes(payload)

    for relative in VENDORED_TESTDATA:
        payload = (source / relative).read_bytes().replace(b"\r\n", b"\n")
        target = ROOT / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(payload)
        recorded[relative] = sha256_bytes(payload)

    if admit_source(source) != source_sha:
        raise SystemExit("source checkout changed during vendoring; discard the partial sync and retry")

    manifest = {
        "source_repository": SOURCE_REPOSITORY,
        "source_sha": source_sha,
        "crates": list(VENDORED_CRATES),
        "files": dict(sorted(recorded.items())),
    }
    MANIFEST_PATH.write_text(
        json.dumps(manifest, indent=2) + "\n", encoding="utf-8"
    )
    print(f"vendored {len(recorded)} files from {SOURCE_REPOSITORY}@{source_sha}")
    return 0


def check() -> int:
    manifest = json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))
    failures: list[str] = []
    expected = manifest["files"]
    for relative, digest in expected.items():
        path = ROOT / relative
        if not path.is_file():
            failures.append(f"missing: {relative}")
            continue
        if sha256_bytes(path.read_bytes()) != digest:
            failures.append(f"modified: {relative}")
    vendored_roots = [ROOT / "crates" / crate for crate in manifest["crates"]]
    vendored_roots.append(ROOT / "testdata")
    for vendored_root in vendored_roots:
        for path in sorted(p for p in vendored_root.rglob("*") if p.is_file()):
            relative = path.relative_to(ROOT).as_posix()
            if relative not in expected:
                failures.append(f"unexpected: {relative}")
    if failures:
        for failure in failures:
            print(failure, file=sys.stderr)
        print(
            "vendored cloud client trees have drifted from "
            f"{manifest['source_repository']}@{manifest['source_sha']}; "
            "re-run tools/cloud/sync_vendored_client.py --source <checkout> "
            "instead of editing them in place",
            file=sys.stderr,
        )
        return 1
    print(
        f"vendored trees match {manifest['source_repository']}"
        f"@{manifest['source_sha']} ({len(expected)} files)"
    )
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument(
        "--source",
        type=Path,
        help="path to an RSpice-Cloud checkout to vendor from",
    )
    group.add_argument(
        "--check",
        action="store_true",
        help="verify the vendored trees match the recorded manifest",
    )
    arguments = parser.parse_args()
    if arguments.check:
        return check()
    return sync(arguments.source.resolve())


if __name__ == "__main__":
    raise SystemExit(main())
