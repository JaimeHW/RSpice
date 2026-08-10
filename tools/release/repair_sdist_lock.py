#!/usr/bin/env python3
"""Repair maturin's Cargo-workspace source distribution so consumers can build it.

Maturin's Cargo sdist generator leaves the archive unbuildable in two ways.  It
fills the archive from ``cargo package --list``, which never reports a file
outside a package directory, so a crate that ``include_str!``s an asset from the
workspace root ships an archive whose compile fails on a missing file; and it
prunes unrelated workspace members but copies the original workspace Cargo.lock
unchanged, so Cargo rejects the archive under ``--locked``.  This utility adds
the embedded files the compiled sources name, asks Cargo to reconcile the copied
lockfile against the pruned workspace, validates the result under ``--locked``,
and rewrites the archive once with both corrections.
"""

from __future__ import annotations

import argparse
import gzip
import io
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
from typing import Iterable


class RepairError(RuntimeError):
    """Raised when an sdist cannot be repaired safely."""


_LOCK_STRING = re.compile(
    r'^(name|version|source|checksum)\s*=\s*("(?:[^"\\]|\\.)*")\s*$'
)

# Installing an sdist compiles each package's `src` tree and nothing else, so an
# embed reached from `tests`, `benches` or `examples` is out of scope: those
# targets are never built by the consumer this archive exists for.
_COMPILED_TREE = "src"

_EMBED_CALL = re.compile(r"\binclude_(?:str|bytes)!\s*\(\s*")
_EMBED_LITERAL = re.compile(r'"((?:[^"\\]|\\.)*)"')


def _external_package_identities(lockfile: bytes) -> frozenset[tuple[str, str, str, str | None]]:
    """Return immutable identities for every registry or Git lockfile package."""

    try:
        text = lockfile.decode("utf-8")
    except UnicodeDecodeError as exc:
        raise RepairError("Cargo.lock is not valid UTF-8") from exc

    packages: list[dict[str, str]] = []
    package: dict[str, str] | None = None
    for line in text.splitlines():
        if line.strip() == "[[package]]":
            if package is not None:
                packages.append(package)
            package = {}
            continue
        if package is None:
            continue
        match = _LOCK_STRING.match(line.strip())
        if match is None:
            continue
        try:
            value = json.loads(match.group(2))
        except json.JSONDecodeError as exc:
            raise RepairError(f"invalid quoted value in Cargo.lock: {line!r}") from exc
        if not isinstance(value, str):
            raise RepairError(f"non-string package identity in Cargo.lock: {line!r}")
        package[match.group(1)] = value
    if package is not None:
        packages.append(package)

    identities: set[tuple[str, str, str, str | None]] = set()
    for fields in packages:
        source = fields.get("source")
        if source is None:
            # Workspace and path packages intentionally have no immutable source.
            continue
        try:
            identity = (fields["name"], fields["version"], source, fields.get("checksum"))
        except KeyError as exc:
            raise RepairError(
                f"external Cargo.lock package is missing {exc.args[0]!r}: {fields!r}"
            ) from exc
        if identity in identities:
            raise RepairError(f"duplicate external Cargo.lock package identity: {identity!r}")
        identities.add(identity)
    return frozenset(identities)


def _validate_reconciliation(original_lock: bytes, repaired_lock: bytes) -> None:
    """Prove unlocked reconciliation did not introduce or alter dependencies."""

    original = _external_package_identities(original_lock)
    repaired = _external_package_identities(repaired_lock)
    introduced = sorted(repaired - original)
    if introduced:
        details = "\n".join(
            f"  {name} {version} ({source}, checksum={checksum or 'none'})"
            for name, version, source, checksum in introduced
        )
        raise RepairError(
            "Cargo lockfile reconciliation introduced or changed external packages:\n"
            f"{details}"
        )


def _safe_member_path(root: Path, member: tarfile.TarInfo) -> Path:
    name = PurePosixPath(member.name)
    if name.is_absolute() or not name.parts or any(part in {"", ".", ".."} for part in name.parts):
        raise RepairError(f"unsafe archive member path: {member.name!r}")

    destination = root.joinpath(*name.parts)
    try:
        destination.resolve().relative_to(root.resolve())
    except ValueError as exc:
        raise RepairError(f"archive member escapes extraction root: {member.name!r}") from exc
    return destination


def _extract_regular_archive(archive: Path, destination: Path) -> list[tarfile.TarInfo]:
    members: list[tarfile.TarInfo] = []
    seen: set[str] = set()
    with tarfile.open(archive, mode="r:gz") as source:
        for member in source:
            if member.name in seen:
                raise RepairError(f"duplicate archive member: {member.name!r}")
            seen.add(member.name)
            target = _safe_member_path(destination, member)

            if member.isdir():
                target.mkdir(parents=True, exist_ok=True)
            elif member.isreg():
                target.parent.mkdir(parents=True, exist_ok=True)
                payload = source.extractfile(member)
                if payload is None:
                    raise RepairError(f"could not read archive member: {member.name!r}")
                with target.open("wb") as output:
                    shutil.copyfileobj(payload, output)
                os.chmod(target, member.mode & 0o777)
            else:
                raise RepairError(
                    f"unsupported non-regular archive member {member.name!r}; "
                    "links and device entries are intentionally rejected"
                )
            members.append(member)

    return members


def _workspace_root(destination: Path, members: Iterable[tarfile.TarInfo]) -> tuple[str, Path]:
    roots = {PurePosixPath(member.name).parts[0] for member in members}
    if len(roots) != 1:
        raise RepairError(f"expected exactly one top-level directory, found: {sorted(roots)!r}")
    root_name = roots.pop()
    root = destination / root_name
    for required in ("Cargo.toml", "Cargo.lock"):
        if not (root / required).is_file():
            raise RepairError(f"source distribution is missing top-level {required}")
    return root_name, root


def _normalize_relative(parts: Iterable[str]) -> PurePosixPath | None:
    """Resolve ``.`` and ``..`` textually, or return None if the path escapes."""

    resolved: list[str] = []
    for part in parts:
        if part in {"", "."}:
            continue
        if part == "..":
            if not resolved:
                return None
            resolved.pop()
        else:
            resolved.append(part)
    return PurePosixPath(*resolved) if resolved else None


def _package_directories(root: Path) -> list[PurePosixPath]:
    return [
        PurePosixPath(manifest.parent.relative_to(root).as_posix())
        for manifest in root.rglob("Cargo.toml")
    ]


def _embedded_workspace_files(root: Path) -> dict[PurePosixPath, PurePosixPath]:
    """Map each file the compiled sources embed from outside their own package.

    Values are the source file naming the target, for diagnostics.  Only string
    literals are resolved; the remaining call sites build their argument with
    ``concat!(env!(..), ..)`` and are gated to ``wasm32`` or to ``cfg(test)``,
    neither of which an sdist install compiles.
    """

    packages = _package_directories(root)
    embedded: dict[PurePosixPath, PurePosixPath] = {}
    for path in sorted(root.rglob("*.rs")):
        relative = PurePosixPath(path.relative_to(root).as_posix())
        owners = [package for package in packages if package in relative.parents]
        if not owners:
            continue
        owner = max(owners, key=lambda package: len(package.parts))
        within = relative.parts[len(owner.parts) :]
        if not within or within[0] != _COMPILED_TREE:
            continue

        text = path.read_text(encoding="utf-8", errors="replace")
        for call in _EMBED_CALL.finditer(text):
            literal = _EMBED_LITERAL.match(text, call.end())
            if literal is None:
                continue
            target = _normalize_relative(
                [*relative.parent.parts, *PurePosixPath(literal.group(1)).parts]
            )
            if target is None:
                raise RepairError(
                    f"{relative} embeds '{literal.group(1)}', which escapes the archive"
                )
            if owner not in target.parents:
                embedded.setdefault(target, relative)
    return embedded


def _missing_embedded_files(root: Path, checkout: Path) -> dict[PurePosixPath, Path]:
    """Locate every embedded file the archive lacks in the repository checkout."""

    missing: dict[PurePosixPath, Path] = {}
    for target, named_by in sorted(_embedded_workspace_files(root).items()):
        if root.joinpath(*target.parts).is_file():
            continue
        source = checkout.joinpath(*target.parts)
        if not source.is_file():
            raise RepairError(
                f"{named_by} embeds '{target}', which is in neither the source "
                f"distribution nor the checkout at {checkout}"
            )
        missing[target] = source
    return missing


def _run_cargo_metadata(root: Path, cargo: str, offline: bool, locked: bool) -> None:
    command = [
        cargo,
        "metadata",
        "--format-version",
        "1",
        "--manifest-path",
        str(root / "Cargo.toml"),
    ]
    if offline:
        command.append("--offline")
    if locked:
        command.append("--locked")

    completed = subprocess.run(
        command,
        cwd=root,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    if completed.returncode != 0:
        mode = "locked validation" if locked else "lockfile reconciliation"
        detail = completed.stderr.strip() or f"Cargo exited with status {completed.returncode}"
        raise RepairError(f"Cargo {mode} failed:\n{detail}")


def _rewrite_archive(
    archive: Path,
    output: Path,
    lock_member_name: str,
    repaired_lock: bytes,
    added: dict[str, Path],
) -> None:
    output.parent.mkdir(parents=True, exist_ok=True)
    with archive.open("rb") as raw_input, output.open("wb") as raw_output:
        with tarfile.open(fileobj=raw_input, mode="r:gz") as source:
            with gzip.GzipFile(filename="", mode="wb", fileobj=raw_output, mtime=0) as compressed:
                with tarfile.open(fileobj=compressed, mode="w", format=tarfile.PAX_FORMAT) as target:
                    replaced = 0
                    carried: set[str] = set()
                    for member in source:
                        carried.add(member.name)
                        payload = source.extractfile(member) if member.isreg() else None
                        if member.name == lock_member_name:
                            member.size = len(repaired_lock)
                            member.mtime = 0
                            target.addfile(member, io.BytesIO(repaired_lock))
                            replaced += 1
                        elif payload is None:
                            target.addfile(member)
                        else:
                            target.addfile(member, payload)

                    collisions = sorted(carried & set(added))
                    if collisions:
                        raise RepairError(f"embedded files are already archived: {collisions}")
                    for name, embedded in sorted(added.items()):
                        payload_bytes = embedded.read_bytes()
                        info = tarfile.TarInfo(name)
                        info.size = len(payload_bytes)
                        info.mode = 0o644
                        info.mtime = 0
                        target.addfile(info, io.BytesIO(payload_bytes))
    if replaced != 1:
        raise RepairError(f"expected one {lock_member_name!r} member, replaced {replaced}")


def _verify_rewritten_archive(
    archive: Path,
    lock_member_name: str,
    repaired_lock: bytes,
    added: dict[str, Path],
) -> None:
    with tarfile.open(archive, mode="r:gz") as verified:
        expected = {lock_member_name: repaired_lock}
        expected.update((name, source.read_bytes()) for name, source in added.items())
        for name, payload in sorted(expected.items()):
            try:
                member = verified.getmember(name)
            except KeyError as exc:
                raise RepairError(f"repacked archive is missing {name!r}") from exc
            extracted = verified.extractfile(member)
            if extracted is None or extracted.read() != payload:
                raise RepairError(f"repacked archive failed integrity verification for {name!r}")


def repair_archive(archive: Path, cargo: str, offline: bool, checkout: Path) -> None:
    archive = archive.resolve()
    if not archive.is_file():
        raise RepairError(f"source distribution does not exist: {archive}")
    if not (checkout / "Cargo.toml").is_file():
        raise RepairError(f"checkout has no workspace manifest: {checkout}")

    with tempfile.TemporaryDirectory(prefix="rspice-sdist-") as temporary:
        extraction_root = Path(temporary) / "source"
        extraction_root.mkdir()
        members = _extract_regular_archive(archive, extraction_root)
        root_name, root = _workspace_root(extraction_root, members)
        original_lock = (root / "Cargo.lock").read_bytes()

        # Restore the embedded files before Cargo runs, so the tree Cargo
        # validates below is the tree the archive ships.
        missing = _missing_embedded_files(root, checkout)
        for target, source in missing.items():
            destination = root.joinpath(*target.parts)
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)

        # The first command is intentionally unlocked: it removes stale workspace
        # packages from the copied lockfile. The identity comparison proves it did
        # not add or alter any external dependency; removals are allowed because the
        # sdist prunes unrelated workspace members. The second Cargo command proves
        # consumers can use --locked.
        _run_cargo_metadata(root, cargo, offline=offline, locked=False)
        repaired_lock = (root / "Cargo.lock").read_bytes()
        _validate_reconciliation(original_lock, repaired_lock)
        _run_cargo_metadata(root, cargo, offline=offline, locked=True)

        added = {f"{root_name}/{target}": source for target, source in missing.items()}
        temporary_archive = archive.with_name(f".{archive.name}.repairing")
        try:
            _rewrite_archive(
                archive,
                temporary_archive,
                f"{root_name}/Cargo.lock",
                repaired_lock,
                added,
            )
            _verify_rewritten_archive(
                temporary_archive, f"{root_name}/Cargo.lock", repaired_lock, added
            )
            os.replace(temporary_archive, archive)
        finally:
            temporary_archive.unlink(missing_ok=True)


def _parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("archives", nargs="+", type=Path, help="maturin-generated .tar.gz files")
    parser.add_argument(
        "--cargo",
        default=os.environ.get("CARGO", "cargo"),
        help="Cargo executable (default: $CARGO or cargo)",
    )
    parser.add_argument(
        "--online",
        action="store_true",
        help="allow Cargo network access while reconciling the existing lockfile",
    )
    parser.add_argument(
        "--checkout",
        default=Path(__file__).resolve().parents[2],
        type=Path,
        help="repository the embedded files are restored from (default: this checkout)",
    )
    return parser


def main(argv: list[str] | None = None) -> int:
    arguments = _parser().parse_args(argv)
    checkout = arguments.checkout.resolve()
    try:
        for archive in arguments.archives:
            repair_archive(
                archive,
                cargo=arguments.cargo,
                offline=not arguments.online,
                checkout=checkout,
            )
            print(f"repaired and validated {archive}")
    except (OSError, tarfile.TarError, RepairError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
