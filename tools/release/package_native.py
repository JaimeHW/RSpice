#!/usr/bin/env python3
"""Create a deterministic, self-describing native RSpice backend archive."""

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
    target: str,
    version: str,
    commit: str,
    source_date_epoch: int,
) -> None:
    if not binary.is_file() or binary.is_symlink():
        raise PackageError(f"binary must be a regular file: {binary}")
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


def release_payloads(binary: Path, target: str) -> list[Payload]:
    executable = "rspice.exe" if "windows" in target else "rspice"
    sources = [
        (executable, binary, 0o755),
        ("LICENSE", ROOT / "LICENSE", 0o644),
        ("NOTICE", ROOT / "NOTICE", 0o644),
        ("README.md", ROOT / "README.md", 0o644),
        ("CLI-README.md", ROOT / "crates" / "rspice-cli" / "README.md", 0o644),
        ("Cargo.lock", ROOT / "Cargo.lock", 0o644),
    ]
    payloads = [Payload(name, source.read_bytes(), mode) for name, source, mode in sources]
    payloads.sort(key=lambda payload: payload.path)
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
    target: str,
    version: str,
    commit: str,
    source_date_epoch: int,
    output_directory: Path,
) -> tuple[Path, Path]:
    validate_inputs(binary, target, version, commit, source_date_epoch)
    prefix = f"rspice-{version}-{target}"
    payloads = release_payloads(binary, target)
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
            binary=arguments.binary.resolve(),
            target=arguments.target,
            version=arguments.version,
            commit=arguments.commit,
            source_date_epoch=arguments.source_date_epoch,
            output_directory=arguments.out.resolve(),
        )
    except PackageError as error:
        raise SystemExit(f"release packaging failed: {error}") from error
    print(archive)
    print(checksum)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
