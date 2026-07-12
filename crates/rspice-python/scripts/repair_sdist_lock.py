#!/usr/bin/env python3
"""Repair maturin's lockfile in a Cargo-workspace source distribution.

Maturin's Cargo sdist generator prunes unrelated workspace members but currently
copies the original workspace Cargo.lock unchanged.  Cargo therefore rejects the
archive when a consumer builds it with ``--locked``.  This utility asks Cargo to
reconcile that copied lockfile against the pruned workspace, validates the result
under ``--locked``, and atomically replaces only the lockfile in the archive.
"""

from __future__ import annotations

import argparse
import gzip
import io
import os
from pathlib import Path, PurePosixPath
import shutil
import subprocess
import sys
import tarfile
import tempfile
from typing import Iterable


class RepairError(RuntimeError):
    """Raised when an sdist cannot be repaired safely."""


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


def _replace_lock_member(
    archive: Path,
    output: Path,
    lock_member_name: str,
    repaired_lock: bytes,
) -> None:
    output.parent.mkdir(parents=True, exist_ok=True)
    with archive.open("rb") as raw_input, output.open("wb") as raw_output:
        with tarfile.open(fileobj=raw_input, mode="r:gz") as source:
            with gzip.GzipFile(filename="", mode="wb", fileobj=raw_output, mtime=0) as compressed:
                with tarfile.open(fileobj=compressed, mode="w", format=tarfile.PAX_FORMAT) as target:
                    replaced = 0
                    for member in source:
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
    if replaced != 1:
        raise RepairError(f"expected one {lock_member_name!r} member, replaced {replaced}")


def repair_archive(archive: Path, cargo: str, offline: bool) -> None:
    archive = archive.resolve()
    if not archive.is_file():
        raise RepairError(f"source distribution does not exist: {archive}")

    with tempfile.TemporaryDirectory(prefix="rspice-sdist-") as temporary:
        extraction_root = Path(temporary) / "source"
        extraction_root.mkdir()
        members = _extract_regular_archive(archive, extraction_root)
        root_name, root = _workspace_root(extraction_root, members)

        # The first command is intentionally unlocked: it removes stale workspace
        # packages from the copied lockfile. Existing locked dependency versions are
        # retained. The second command proves consumers can use --locked.
        _run_cargo_metadata(root, cargo, offline=offline, locked=False)
        _run_cargo_metadata(root, cargo, offline=offline, locked=True)
        repaired_lock = (root / "Cargo.lock").read_bytes()

        temporary_archive = archive.with_name(f".{archive.name}.repairing")
        try:
            _replace_lock_member(
                archive,
                temporary_archive,
                f"{root_name}/Cargo.lock",
                repaired_lock,
            )
            with tarfile.open(temporary_archive, mode="r:gz") as verified:
                member = verified.getmember(f"{root_name}/Cargo.lock")
                extracted = verified.extractfile(member)
                if extracted is None or extracted.read() != repaired_lock:
                    raise RepairError("repacked archive failed lockfile integrity verification")
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
    return parser


def main(argv: list[str] | None = None) -> int:
    arguments = _parser().parse_args(argv)
    try:
        for archive in arguments.archives:
            repair_archive(archive, cargo=arguments.cargo, offline=not arguments.online)
            print(f"repaired and validated {archive}")
    except (OSError, tarfile.TarError, RepairError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
