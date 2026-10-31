"""Vendor epaint 0.35.0 with the right-to-left glyph-count fix.

egui 0.35.0 lays out a right-to-left run with one extra zero-width glyph per
character after the first (emilk/egui#8576), so a text cursor in Arabic or
Hebrew lands past the end of the text and "previous word" underflows. RSpice
edits right-to-left text, so it builds against crates.io epaint 0.35.0 with
that one fix applied, through `[patch.crates-io]` in the root manifest.

`vendor/epaint` is the crates.io package byte for byte (without the files
cargo writes when it packages a crate), egui's two licence files from the
commit 0.35.0 was released from, and `epaint.patch` applied on top. The
manifest records the package checksum, the patch digest and every upstream
file's digest, so `--check` can prove offline that the tree is exactly that.

Delete `vendor/epaint`, this directory and the two manifest lines at the first
egui upgrade whose epaint ships the fix.

Usage:
    python tools/vendor/vendor_epaint.py --crate <epaint-0.35.0.crate> --licences <dir>
    python tools/vendor/vendor_epaint.py --check [--crate <...> --licences <dir>]
"""

from __future__ import annotations

import argparse
import hashlib
import io
import json
import os
import shutil
import subprocess
import sys
import tarfile
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
VENDOR = ROOT / "vendor" / "epaint"
HERE = Path(__file__).resolve().parent
MANIFEST = HERE / "epaint-manifest.json"
PATCH = HERE / "epaint.patch"
LICENCES = ("LICENSE-APACHE", "LICENSE-MIT")
# Written by `cargo package`, not part of the crate's sources.
PACKAGING = {".cargo_vcs_info.json", "Cargo.lock", "Cargo.toml.orig"}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def tree_digests(root: Path) -> dict[str, str]:
    return {
        path.relative_to(root).as_posix(): sha256(path.read_bytes())
        for path in sorted(root.rglob("*"))
        if path.is_file()
    }


def load_manifest() -> dict:
    return json.loads(MANIFEST.read_text(encoding="utf-8"))


def apply_patch(tree: Path, *, reverse: bool) -> None:
    """Apply the patch to `tree` exactly: no fuzz, no line-ending translation.

    Inside a repository, git apply resolves paths from the repository root and
    silently skips the ones outside the working directory, so the search for
    a repository stops at the tree.
    """
    command = ["git", "-c", "core.autocrlf=false", "apply", "--no-index"]
    if reverse:
        command.append("--reverse")
    environment = {**os.environ, "GIT_CEILING_DIRECTORIES": str(tree.resolve().parent)}
    subprocess.run(
        [*command, str(PATCH)], cwd=tree, env=environment, check=True, capture_output=True
    )


def unpack_upstream(crate: Path, licences: Path, manifest: dict, into: Path) -> None:
    """Write the upstream tree: the package's sources plus egui's licences."""
    data = crate.read_bytes()
    if sha256(data) != manifest["crate_sha256"]:
        raise SystemExit(f"{crate} is not crates.io {manifest['crate']} {manifest['version']}")
    prefix = f"{manifest['crate']}-{manifest['version']}/"
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as archive:
        for member in archive.getmembers():
            if not member.isfile() or not member.name.startswith(prefix):
                continue
            relative = member.name[len(prefix):]
            if relative in PACKAGING:
                continue
            target = into / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            source = archive.extractfile(member)
            assert source is not None
            target.write_bytes(source.read())
    for name in LICENCES:
        shutil.copyfile(licences / name, into / name)


def check_tree(tree: Path, manifest: dict) -> list[str]:
    """Every way `tree` differs from the upstream tree plus the patch."""
    problems = []
    if sha256(PATCH.read_bytes()) != manifest["patch_sha256"]:
        problems.append("epaint.patch is not the recorded patch")
    with tempfile.TemporaryDirectory() as directory:
        staging = Path(directory) / "epaint"
        shutil.copytree(tree, staging)
        try:
            apply_patch(staging, reverse=True)
        except subprocess.CalledProcessError as error:
            problems.append(f"the patch does not reverse out: {error.stderr.decode().strip()}")
            return problems
        recovered = tree_digests(staging)
    expected = manifest["upstream_files"]
    for relative in sorted(set(recovered) | set(expected)):
        if relative not in expected:
            problems.append(f"{relative} is not part of the upstream package")
        elif relative not in recovered:
            problems.append(f"{relative} is missing")
        elif recovered[relative] != expected[relative]:
            problems.append(f"{relative} differs from upstream outside the patch")
    return problems


def derive(crate: Path, licences: Path, into: Path, manifest: dict) -> None:
    unpack_upstream(crate, licences, manifest, into)
    apply_patch(into, reverse=False)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--crate", type=Path, help="the crates.io epaint-0.35.0.crate")
    parser.add_argument("--licences", type=Path, help="egui's LICENSE-APACHE and LICENSE-MIT")
    parser.add_argument("--check", action="store_true", help="verify vendor/epaint")
    arguments = parser.parse_args()
    manifest = load_manifest()

    if arguments.check:
        problems = check_tree(VENDOR, manifest)
        if arguments.crate and arguments.licences:
            with tempfile.TemporaryDirectory() as directory:
                derived = Path(directory) / "epaint"
                derived.mkdir()
                derive(arguments.crate, arguments.licences, derived, manifest)
                if tree_digests(derived) != tree_digests(VENDOR):
                    problems.append("vendor/epaint differs from a fresh derivation")
        for problem in problems:
            print(f"vendor/epaint: {problem}", file=sys.stderr)
        return 1 if problems else 0

    if not (arguments.crate and arguments.licences):
        parser.error("--crate and --licences are required to vendor")
    with tempfile.TemporaryDirectory() as directory:
        upstream = Path(directory) / "upstream"
        upstream.mkdir()
        unpack_upstream(arguments.crate, arguments.licences, manifest, upstream)
        manifest["upstream_files"] = tree_digests(upstream)
    manifest["patch_sha256"] = sha256(PATCH.read_bytes())
    if VENDOR.exists():
        shutil.rmtree(VENDOR)
    VENDOR.mkdir(parents=True)
    derive(arguments.crate, arguments.licences, VENDOR, manifest)
    MANIFEST.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8", newline="\n")
    if tree_digests(VENDOR) == manifest["upstream_files"]:
        raise SystemExit("the patch changed nothing")
    return 0


if __name__ == "__main__":
    sys.exit(main())
