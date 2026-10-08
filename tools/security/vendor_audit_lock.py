#!/usr/bin/env python3
"""Verify local registry patches and expose their upstream versions to RustSec.

cargo-audit skips path packages. This supplemental, audit-only lockfile records
their original registry identities so vendoring cannot hide upstream advisories.
It must never be used for dependency resolution or builds: local patches differ
from the registry archives. Cargo.lock remains the authoritative build graph.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import sys
import tomllib


ROOT = Path(__file__).resolve().parents[2]
REGISTRY = "registry+https://github.com/rust-lang/crates.io-index"
SHA256 = re.compile(r"[0-9a-f]{64}\Z")
RECORDS = {"RSPICE_VENDOR.json", "RSPICE_PATCH.json", ".gitattributes"}


class VendorError(RuntimeError):
    """A local dependency's reviewed identity or source no longer matches."""


def relative_path(raw: str) -> Path:
    path = PurePosixPath(raw)
    if (not path.parts or path.is_absolute() or ".." in path.parts
            or "\\" in raw or ":" in raw):
        raise VendorError(f"unsafe vendor path: {raw!r}")
    return Path(*path.parts)


def read_toml(path: Path) -> dict:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def verify_source(directory: Path) -> dict:
    provenance = json.loads((directory / "RSPICE_VENDOR.json").read_text(encoding="utf-8"))
    patch = json.loads((directory / "RSPICE_PATCH.json").read_text(encoding="utf-8"))
    original = provenance["upstream_files_sha256"]
    changed = patch["patched_files_sha256"]
    if (set(changed) != set(patch["modified_upstream_files"])
            or not set(changed) <= set(original)):
        raise VendorError(f"{directory}: patch inventory differs from provenance")
    expected = original | changed
    present: set[str] = set()
    for parent, dirs, files in os.walk(directory, followlinks=False):
        dirs[:] = sorted(name for name in dirs if name not in {"target", ".git"})
        for name in [*dirs, *files]:
            path = Path(parent) / name
            if path.is_symlink():
                raise VendorError(f"vendor source contains a symlink: {path}")
        for name in files:
            present.add((Path(parent) / name).relative_to(directory).as_posix())
    if present != set(expected) | RECORDS:
        raise VendorError(f"{directory}: source file inventory differs from provenance")
    for name, checksum in expected.items():
        path = directory / relative_path(name)
        if not SHA256.fullmatch(checksum) or hashlib.sha256(path.read_bytes()).hexdigest() != checksum:
            raise VendorError(f"vendor source checksum mismatch: {path}")
    package = read_toml(directory / "Cargo.toml")["package"]
    if (package["name"], package["version"]) != (provenance["package"], provenance["version"]):
        raise VendorError(f"{directory}: package identity differs from provenance")
    if not SHA256.fullmatch(provenance["registry_package_sha256"]):
        raise VendorError(f"{directory}: invalid upstream package checksum")
    return provenance


def audit_lock(root: Path) -> str:
    manifest = read_toml(root / "Cargo.toml")
    packages = read_toml(root / "Cargo.lock")["package"]
    lines = [
        "# Generated for advisory scanning only. DO NOT use for builds.",
        "# These are upstream identities; reviewed local changes are hash-verified.",
        "version = 4",
    ]
    seen: set[tuple[str, str]] = set()
    for spec in manifest.get("patch", {}).get("crates-io", {}).values():
        if not isinstance(spec, dict) or "path" not in spec:
            continue
        directory = root / relative_path(spec["path"])
        if not directory.resolve().is_relative_to(root.resolve()) or directory.is_symlink():
            raise VendorError(f"local patch escapes checkout or is a symlink: {directory}")
        provenance = verify_source(directory)
        identity = (provenance["package"], provenance["version"])
        matches = [p for p in packages if (p["name"], p["version"]) == identity]
        if len(matches) != 1 or "source" in matches[0]:
            raise VendorError(f"{identity}: expected one local package in Cargo.lock")
        if identity in seen:
            raise VendorError(f"duplicate local patch identity: {identity}")
        seen.add(identity)
        lines += [
            "", "[[package]]",
            f"name = {json.dumps(identity[0])}",
            f"version = {json.dumps(identity[1])}",
            f"source = {json.dumps(REGISTRY)}",
            f"checksum = {json.dumps(provenance['registry_package_sha256'])}",
        ]
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--output", type=Path, default=Path("target/security-vendor-audit.lock"))
    args = parser.parse_args()
    try:
        contents = audit_lock(args.root)
        output = args.output if args.output.is_absolute() else args.root / args.output
        if output.resolve() == (args.root / "Cargo.lock").resolve():
            raise VendorError("refusing to overwrite the build lockfile")
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(contents, encoding="utf-8", newline="\n")
        print(f"verified local registry patches; wrote audit identities to {output}")
    except (OSError, ValueError, KeyError, TypeError, VendorError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
