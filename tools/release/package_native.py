#!/usr/bin/env python3
"""Create a deterministic, self-describing native RSpice product archive."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import io
import json
import os
import re
import stat
import tarfile
import tempfile
import tomllib
import zipfile
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
REPOSITORY = "https://github.com/JaimeHW/RSpice"
SEMVER = re.compile(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$")
TARGET = re.compile(r"^[A-Za-z0-9_.-]+$")
COMMIT = re.compile(r"^[0-9a-f]{40}$")
ZIP_EPOCH_FLOOR = 315_532_800  # 1980-01-01, the earliest ZIP timestamp.


class PackageError(ValueError):
    """Release inputs violate the immutable packaging contract."""


@dataclass(frozen=True)
class Payload:
    path: str
    content: bytes
    mode: int

    def manifest_entry(self) -> dict[str, object]:
        return {
            "path": self.path,
            "sha256": hashlib.sha256(self.content).hexdigest(),
            "size": len(self.content),
        }


def workspace_version() -> str:
    with (ROOT / "Cargo.toml").open("rb") as handle:
        return tomllib.load(handle)["workspace"]["package"]["version"]


def rust_toolchain() -> str:
    with (ROOT / "rust-toolchain.toml").open("rb") as handle:
        return tomllib.load(handle)["toolchain"]["channel"]


def validate_inputs(
    binary: Path,
    ui_binary: Path,
    runtime_root: Path,
    target: str,
    version: str,
    commit: str,
    source_date_epoch: int,
) -> None:
    if not binary.is_file() or binary.is_symlink():
        raise PackageError(f"binary must be a regular file: {binary}")
    if not ui_binary.is_file() or ui_binary.is_symlink():
        raise PackageError(f"UI binary must be a regular file: {ui_binary}")
    if not runtime_root.is_dir() or is_link_like(runtime_root):
        raise PackageError(f"runtime root must be a local directory: {runtime_root}")
    for required in ("runtime-manifest.json", "runtime-manifest.ed25519.json"):
        member = runtime_root / required
        if not member.is_file() or is_link_like(member):
            raise PackageError(f"signed runtime metadata is missing: {member}")
    if not TARGET.fullmatch(target):
        raise PackageError(f"invalid Rust target triple: {target!r}")
    if not SEMVER.fullmatch(version):
        raise PackageError(f"release version must be stable SemVer: {version!r}")
    expected_version = workspace_version()
    if version != expected_version:
        raise PackageError(
            f"release version {version!r} does not match workspace {expected_version!r}"
        )
    if not COMMIT.fullmatch(commit):
        raise PackageError("release commit must be 40 lowercase hexadecimal characters")
    if source_date_epoch < ZIP_EPOCH_FLOOR:
        raise PackageError("source-date epoch must be on or after 1980-01-01")


def model_tree_payloads() -> list[Payload]:
    """Ship models/spice beside the executable.

    The engine locates the tree relative to the binary (see
    rspice-core's library::spice_packs), so the archive layout is the contract:
    `models/spice/...` next to the executable is what `SpiceLibraryIndex`
    discovers.

    Everything under models/spice is shipped. That is safe because restricted
    material never enters the repository in the first place — it is dropped at
    the vendoring boundary by tools/models/sync_packs.py — so there is no
    filtering decision left to make here. The licence audit gates that
    invariant in CI, and the guard below refuses to build a release if it ever
    fails.
    """
    spice_root = ROOT / "models" / "spice"
    if not spice_root.is_dir() or is_link_like(spice_root):
        raise PackageError(f"model tree missing: {spice_root}")

    audit = spice_root / "LICENSE-AUDIT.tsv"
    if not audit.exists():
        raise PackageError(
            "models/spice/LICENSE-AUDIT.tsv is missing; run "
            "tools/models/license_audit.py before packaging"
        )
    for line in audit.read_text(encoding="utf-8").splitlines():
        if line.startswith("#") or not line.strip():
            continue
        fields = line.split("\t")
        if len(fields) > 3 and fields[3] == "restricted":
            raise PackageError(
                f"refusing to package: {fields[0]}/{fields[1]} is marked "
                f"restricted ({fields[2]}). Re-run tools/models/sync_packs.py "
                f"to drop it from the tree."
            )

    payloads = []
    for path in sorted(spice_root.rglob("*"), key=lambda p: p.as_posix()):
        if is_link_like(path):
            raise PackageError(f"model payload contains a link: {path}")
        if path.is_dir():
            continue
        if not path.is_file():
            raise PackageError(f"model payload contains a non-regular file: {path}")
        relative = path.relative_to(ROOT).as_posix()
        payloads.append(Payload(relative, path.read_bytes(), 0o644))
    return payloads


def is_link_like(path: Path) -> bool:
    """Reject symlinks and Windows junctions from immutable release inputs."""
    return path.is_symlink() or (
        hasattr(path, "is_junction") and path.is_junction()
    )


def runtime_tree_payloads(runtime_root: Path) -> list[Payload]:
    payloads = []
    for path in sorted(runtime_root.rglob("*"), key=lambda candidate: candidate.as_posix()):
        if is_link_like(path):
            raise PackageError(f"runtime payload contains a link: {path}")
        if path.is_dir():
            continue
        if not path.is_file():
            raise PackageError(f"runtime payload contains a non-regular file: {path}")
        relative = path.relative_to(runtime_root)
        if any(
            part in {"", ".", ".."}
            or any(ord(character) < 32 or ord(character) == 127 for character in part)
            for part in relative.parts
        ):
            raise PackageError(f"runtime payload contains an unsafe path: {relative}")
        mode = 0o755 if path.stat(follow_symlinks=False).st_mode & 0o111 else 0o644
        payloads.append(
            Payload(f"runtimes/python/{relative.as_posix()}", path.read_bytes(), mode)
        )
    return payloads


def file_payload(path: str, source: Path, mode: int) -> Payload:
    if not source.is_file() or is_link_like(source):
        raise PackageError(f"release payload source must be a regular file: {source}")
    return Payload(path, source.read_bytes(), mode)


def release_payloads(
    binary: Path, ui_binary: Path, runtime_root: Path, target: str
) -> list[Payload]:
    executable = "rspice.exe" if "windows" in target else "rspice"
    ui_executable = "rspice-ui.exe" if "windows" in target else "rspice-ui"
    sources = [
        (executable, binary, 0o755),
        (ui_executable, ui_binary, 0o755),
        ("LICENSE", ROOT / "LICENSE", 0o644),
        ("NOTICE", ROOT / "NOTICE", 0o644),
        ("README.md", ROOT / "README.md", 0o644),
        ("CLI-README.md", ROOT / "crates" / "rspice-cli" / "README.md", 0o644),
        ("Cargo.lock", ROOT / "Cargo.lock", 0o644),
    ]
    payloads = [file_payload(name, source, mode) for name, source, mode in sources]
    payloads.extend(model_tree_payloads())
    payloads.extend(runtime_tree_payloads(runtime_root))
    payloads.sort(key=lambda payload: payload.path)
    paths = [payload.path for payload in payloads]
    if len(paths) != len(set(paths)):
        raise PackageError("release payload contains duplicate archive paths")
    return payloads


def release_manifest(
    payloads: list[Payload],
    target: str,
    version: str,
    commit: str,
    source_date_epoch: int,
) -> bytes:
    document = {
        "schema_version": 1,
        "product": "rspice",
        "version": version,
        "target": target,
        "source": {
            "repository": REPOSITORY,
            "commit": commit,
            "source_date_epoch": source_date_epoch,
        },
        "build": {
            "profile": "release",
            "rust_toolchain": rust_toolchain(),
            "locked_dependencies": True,
        },
        "files": [payload.manifest_entry() for payload in payloads],
    }
    return (json.dumps(document, indent=2, sort_keys=True) + "\n").encode("utf-8")


def zip_timestamp(source_date_epoch: int) -> tuple[int, int, int, int, int, int]:
    import datetime

    moment = datetime.datetime.fromtimestamp(source_date_epoch, datetime.UTC)
    # ZIP stores seconds at two-second precision.
    return (moment.year, moment.month, moment.day, moment.hour, moment.minute, moment.second // 2 * 2)


def zip_archive(prefix: str, payloads: list[Payload], source_date_epoch: int) -> bytes:
    output = io.BytesIO()
    timestamp = zip_timestamp(source_date_epoch)
    with zipfile.ZipFile(
        output, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9
    ) as archive:
        for payload in payloads:
            info = zipfile.ZipInfo(f"{prefix}/{payload.path}", timestamp)
            info.create_system = 3
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = (stat.S_IFREG | payload.mode) << 16
            archive.writestr(info, payload.content, compresslevel=9)
    return output.getvalue()


def tar_gzip_archive(prefix: str, payloads: list[Payload], source_date_epoch: int) -> bytes:
    tar_bytes = io.BytesIO()
    with tarfile.open(fileobj=tar_bytes, mode="w", format=tarfile.PAX_FORMAT) as archive:
        for payload in payloads:
            info = tarfile.TarInfo(f"{prefix}/{payload.path}")
            info.size = len(payload.content)
            info.mode = payload.mode
            info.mtime = source_date_epoch
            info.uid = 0
            info.gid = 0
            info.uname = ""
            info.gname = ""
            archive.addfile(info, io.BytesIO(payload.content))

    output = io.BytesIO()
    with gzip.GzipFile(filename="", mode="wb", fileobj=output, mtime=source_date_epoch) as stream:
        stream.write(tar_bytes.getvalue())
    return output.getvalue()


def atomic_write(path: Path, content: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(
        prefix=f".{path.name}.", suffix=".tmp", dir=path.parent
    )
    temporary = Path(temporary_name)
    try:
        with os.fdopen(descriptor, "wb") as handle:
            handle.write(content)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(temporary, path)
    finally:
        temporary.unlink(missing_ok=True)


def package_release(
    *,
    binary: Path,
    ui_binary: Path,
    runtime_root: Path,
    target: str,
    version: str,
    commit: str,
    source_date_epoch: int,
    output_directory: Path,
) -> tuple[Path, Path]:
    validate_inputs(
        binary,
        ui_binary,
        runtime_root,
        target,
        version,
        commit,
        source_date_epoch,
    )
    prefix = f"rspice-{version}-{target}"
    payloads = release_payloads(binary, ui_binary, runtime_root, target)
    payloads.append(
        Payload(
            "RELEASE-MANIFEST.json",
            release_manifest(payloads, target, version, commit, source_date_epoch),
            0o644,
        )
    )
    payloads.sort(key=lambda payload: payload.path)

    if "windows" in target:
        archive_name = f"{prefix}.zip"
        archive_bytes = zip_archive(prefix, payloads, source_date_epoch)
    else:
        archive_name = f"{prefix}.tar.gz"
        archive_bytes = tar_gzip_archive(prefix, payloads, source_date_epoch)

    archive_path = output_directory / archive_name
    checksum_path = output_directory / f"{archive_name}.sha256"
    digest = hashlib.sha256(archive_bytes).hexdigest()
    atomic_write(archive_path, archive_bytes)
    atomic_write(checksum_path, f"{digest}  {archive_name}\n".encode("ascii"))
    return archive_path, checksum_path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--ui-binary", type=Path, required=True)
    parser.add_argument("--runtime-root", type=Path, required=True)
    parser.add_argument("--target", required=True)
    parser.add_argument("--version", required=True)
    parser.add_argument("--commit", required=True)
    parser.add_argument("--source-date-epoch", type=int, required=True)
    parser.add_argument("--out", type=Path, required=True)
    return parser.parse_args()


def main() -> int:
    arguments = parse_args()
    try:
        archive, checksum = package_release(
            binary=arguments.binary.absolute(),
            ui_binary=arguments.ui_binary.absolute(),
            runtime_root=arguments.runtime_root.absolute(),
            target=arguments.target,
            version=arguments.version,
            commit=arguments.commit,
            source_date_epoch=arguments.source_date_epoch,
            output_directory=arguments.out.resolve(),
        )
    except (OSError, PackageError) as error:
        raise SystemExit(f"release packaging failed: {error}") from error
    print(archive)
    print(checksum)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
