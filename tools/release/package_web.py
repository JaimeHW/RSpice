#!/usr/bin/env python3
"""Assemble the browser IDE into a deterministic, content-addressed archive."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import struct
import tempfile
import time
import tomllib
import zipfile
from dataclasses import dataclass
from pathlib import Path, PurePosixPath


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_WEB_ROOT = ROOT / "crates" / "rspice-ui" / "web"
ASSET_ROOT_TOKEN = "__RSPICE_ASSET_ROOT__"
ASSET_DIGEST_DOMAIN = b"rspice.web-assets/v1"
RUNTIME_DIGEST_DOMAIN = b"rspice.browser-python-runtime/v1"
ZIP_EPOCH_FLOOR = 315_532_800  # 1980-01-01, the earliest ZIP timestamp.
ZIP_EPOCH_CEILING = 4_354_819_199  # 2107-12-31 23:59:59 UTC.
SEMVER = re.compile(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$")
COMMIT = re.compile(r"^[0-9a-f]{40}$")
TOOL_VERSION = re.compile(r"^0\.2\.\d+$")
HEX_DIGEST = re.compile(r"^[0-9a-f]{64}$")


class PackageError(ValueError):
    """Release inputs violate the immutable browser-package contract."""


@dataclass(frozen=True)
class Payload:
    path: str
    content: bytes
    mode: int = 0o644

    def manifest_entry(self) -> dict[str, object]:
        return {
            "path": self.path,
            "bytes": len(self.content),
            "sha256": hashlib.sha256(self.content).hexdigest(),
        }


def workspace_version() -> str:
    with (ROOT / "Cargo.toml").open("rb") as handle:
        return tomllib.load(handle)["workspace"]["package"]["version"]


def rust_toolchain() -> str:
    with (ROOT / "rust-toolchain.toml").open("rb") as handle:
        return tomllib.load(handle)["toolchain"]["channel"]


def locked_wasm_bindgen_version() -> str:
    with (ROOT / "Cargo.lock").open("rb") as handle:
        lock = tomllib.load(handle)
    versions = {
        package["version"]
        for package in lock["package"]
        if package.get("name") == "wasm-bindgen"
    }
    if len(versions) != 1:
        raise PackageError(
            "Cargo.lock must contain exactly one wasm-bindgen crate version"
        )
    return versions.pop()


def frame(digest: "hashlib._Hash", value: bytes) -> None:
    digest.update(struct.pack(">Q", len(value)))
    digest.update(value)


def digest_payloads(domain: bytes, payloads: list[Payload]) -> str:
    digest = hashlib.sha256(domain)
    for payload in sorted(payloads, key=lambda item: item.path):
        frame(digest, payload.path.encode("utf-8"))
        frame(digest, payload.content)
    return digest.hexdigest()


def validate_relative_path(value: object, label: str) -> str:
    if not isinstance(value, str) or not value:
        raise PackageError(f"{label} must be a non-empty relative path")
    path = PurePosixPath(value)
    if path.is_absolute() or "\\" in value or any(part in {"", ".", ".."} for part in path.parts):
        raise PackageError(f"{label} is unsafe: {value!r}")
    if path.as_posix() != value:
        raise PackageError(f"{label} is not canonical: {value!r}")
    return value


def read_regular(path: Path, label: str) -> bytes:
    if not path.is_file() or path.is_symlink():
        raise PackageError(f"{label} must be a regular, non-symlink file: {path}")
    return path.read_bytes()


def load_runtime_payloads(web_root: Path, automation_worker: bytes) -> list[Payload]:
    python_root = web_root / "python"
    bootstrap_path = python_root / "rspice_browser_bootstrap.py"
    bootstrap = read_regular(bootstrap_path, "browser Python bootstrap")

    runtime_directories = sorted(
        path
        for path in python_root.iterdir()
        if path.is_dir() and path.name.startswith("pyodide-")
    )
    if len(runtime_directories) != 1:
        raise PackageError("browser package must contain exactly one pinned Pyodide runtime")
    runtime_root = runtime_directories[0]
    if runtime_root.is_symlink():
        raise PackageError("browser Pyodide runtime cannot be a symlink")
    manifest_path = runtime_root / "rspice-runtime-manifest.json"
    manifest_bytes = read_regular(manifest_path, "browser runtime manifest")
    try:
        manifest = json.loads(manifest_bytes.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise PackageError(f"browser runtime manifest is invalid: {error}") from error

    if manifest.get("format") != "rspice-browser-python-runtime/v2":
        raise PackageError("browser runtime manifest uses an unsupported format")
    if manifest.get("runtime") != runtime_root.name:
        raise PackageError("browser runtime directory does not match its manifest identity")
    if manifest.get("external_network_required_at_runtime") is not False:
        raise PackageError("browser runtime must be complete and offline")
    expected_runtime_digest = manifest.get("runtime_digest_sha256")
    if not isinstance(expected_runtime_digest, str) or not HEX_DIGEST.fullmatch(
        expected_runtime_digest
    ):
        raise PackageError("browser runtime manifest has an invalid runtime digest")

    entries = manifest.get("files")
    if not isinstance(entries, list) or not entries:
        raise PackageError("browser runtime manifest has no closed file inventory")
    declared: dict[str, dict[str, object]] = {}
    runtime_payloads: list[Payload] = []
    execution_payloads: list[Payload] = []
    for entry in entries:
        if not isinstance(entry, dict):
            raise PackageError("browser runtime manifest contains a non-object entry")
        relative = validate_relative_path(entry.get("path"), "runtime file path")
        if "/" in relative or relative in declared:
            raise PackageError(f"browser runtime file is nested or duplicated: {relative!r}")
        declared[relative] = entry
        content = read_regular(runtime_root / relative, f"browser runtime file {relative}")
        if entry.get("bytes") != len(content):
            raise PackageError(f"browser runtime file has the wrong size: {relative}")
        digest = hashlib.sha256(content).hexdigest()
        if entry.get("sha256") != digest:
            raise PackageError(f"browser runtime file failed SHA-256 validation: {relative}")
        payload = Payload(f"python/{runtime_root.name}/{relative}", content)
        runtime_payloads.append(payload)
        if entry.get("execution_asset") is True:
            execution_payloads.append(payload)
        elif entry.get("execution_asset") is not False:
            raise PackageError(f"runtime file omits execution_asset policy: {relative}")

    actual = {
        path.name
        for path in runtime_root.iterdir()
        if path.name != manifest_path.name
    }
    if actual != set(declared):
        missing = sorted(set(declared) - actual)
        extra = sorted(actual - set(declared))
        raise PackageError(
            f"browser runtime inventory is not closed (missing={missing}, extra={extra})"
        )

    bootstrap_payload = Payload("python/rspice_browser_bootstrap.py", bootstrap)
    execution_payloads.append(bootstrap_payload)
    actual_runtime_digest = digest_payloads(RUNTIME_DIGEST_DOMAIN, execution_payloads)
    if actual_runtime_digest != expected_runtime_digest:
        raise PackageError("browser runtime aggregate digest does not match its manifest")

    worker_text = automation_worker.decode("utf-8", errors="strict")
    if f'"{expected_runtime_digest}"' not in worker_text:
        raise PackageError("automation worker does not pin the packaged runtime digest")
    for payload in execution_payloads:
        name = PurePosixPath(payload.path).name
        digest = hashlib.sha256(payload.content).hexdigest()
        if name not in worker_text or str(len(payload.content)) not in worker_text or digest not in worker_text:
            raise PackageError(f"automation worker does not pin execution asset {name}")

    runtime_payloads.extend(
        [
            Payload(
                f"python/{runtime_root.name}/{manifest_path.name}", manifest_bytes
            ),
            bootstrap_payload,
        ]
    )
    return runtime_payloads


def load_asset_payloads(web_root: Path) -> list[Payload]:
    simulation_worker = read_regular(
        web_root / "simulation-worker.js", "simulation worker"
    )
    automation_worker = read_regular(
        web_root / "automation-worker.js", "automation worker"
    )
    binding = read_regular(web_root / "pkg" / "rspice-ui.js", "wasm-bindgen module")
    wasm = read_regular(web_root / "pkg" / "rspice-ui_bg.wasm", "UI WebAssembly")
    if not wasm.startswith(b"\x00asm"):
        raise PackageError("UI WebAssembly does not have the WebAssembly magic header")
    binding_text = binding.decode("utf-8", errors="strict")
    for export in (
        "runRspiceUiWorkerRequest",
        "runRspiceUiVerilogACompileRequest",
        "runRspiceUiHardcopyRequest",
    ):
        if export not in binding_text:
            raise PackageError(f"wasm-bindgen module is missing {export}")

    payloads = [
        Payload("simulation-worker.js", simulation_worker),
        Payload("automation-worker.js", automation_worker),
        Payload("rspice-ui.js", binding),
        Payload("rspice-ui_bg.wasm", wasm),
    ]
    payloads.extend(load_runtime_payloads(web_root, automation_worker))
    paths = [payload.path for payload in payloads]
    if len(paths) != len(set(paths)):
        raise PackageError("browser asset inventory contains duplicate paths")
    return sorted(payloads, key=lambda payload: payload.path)


def validate_inputs(
    web_root: Path,
    version: str,
    commit: str,
    source_date_epoch: int,
    wasm_bindgen_version: str,
) -> None:
    if not web_root.is_dir() or web_root.is_symlink():
        raise PackageError(f"web root must be a safe directory: {web_root}")
    if not SEMVER.fullmatch(version) or version != workspace_version():
        raise PackageError(
            f"release version {version!r} does not match stable workspace version {workspace_version()!r}"
        )
    if not COMMIT.fullmatch(commit):
        raise PackageError("release commit must be 40 lowercase hexadecimal characters")
    if not ZIP_EPOCH_FLOOR <= source_date_epoch <= ZIP_EPOCH_CEILING:
        raise PackageError("source-date epoch is outside the deterministic ZIP range")
    if not TOOL_VERSION.fullmatch(wasm_bindgen_version):
        raise PackageError("wasm-bindgen version must be an exact 0.2.x version")
    locked = locked_wasm_bindgen_version()
    if wasm_bindgen_version != locked:
        raise PackageError(
            f"wasm-bindgen CLI {wasm_bindgen_version} does not match Cargo.lock {locked}"
        )


def release_manifest(
    payloads: list[Payload],
    version: str,
    commit: str,
    source_date_epoch: int,
    wasm_bindgen_version: str,
    asset_digest: str,
) -> bytes:
    value = {
        "schema": "rspice.browser-release/v1",
        "version": version,
        "source": {"commit": commit, "source_date_epoch": source_date_epoch},
        "build": {
            "locked_dependencies": True,
            "rust_toolchain": rust_toolchain(),
            "target": "wasm32-unknown-unknown",
            "wasm_bindgen": wasm_bindgen_version,
        },
        "entrypoint": "index.html",
        "asset_root": f"assets/{asset_digest}",
        "asset_digest_sha256": asset_digest,
        "files": [payload.manifest_entry() for payload in sorted(payloads, key=lambda item: item.path)],
    }
    return (json.dumps(value, indent=2, sort_keys=True) + "\n").encode("utf-8")


def zip_info(path: str, source_date_epoch: int, mode: int = 0o644) -> zipfile.ZipInfo:
    timestamp = time.gmtime(source_date_epoch)[:6]
    info = zipfile.ZipInfo(path, timestamp)
    info.compress_type = zipfile.ZIP_DEFLATED
    info.create_system = 3
    info.external_attr = (mode & 0xFFFF) << 16
    info.flag_bits |= 0x800
    return info


def assemble(
    web_root: Path,
    output_directory: Path,
    version: str,
    commit: str,
    source_date_epoch: int,
    wasm_bindgen_version: str,
) -> tuple[Path, Path, str]:
    validate_inputs(web_root, version, commit, source_date_epoch, wasm_bindgen_version)
    assets = load_asset_payloads(web_root)
    asset_digest = digest_payloads(ASSET_DIGEST_DOMAIN, assets)
    asset_root = f"assets/{asset_digest}"

    index_bytes = read_regular(web_root / "index.html", "browser entrypoint")
    try:
        index = index_bytes.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise PackageError(f"browser entrypoint is not UTF-8: {error}") from error
    if index.count(ASSET_ROOT_TOKEN) != 1:
        raise PackageError("browser entrypoint must contain exactly one release asset-root token")
    release_index = index.replace(ASSET_ROOT_TOKEN, f"./{asset_root}").encode("utf-8")

    archive_payloads = [Payload("index.html", release_index)]
    archive_payloads.extend(
        Payload(f"{asset_root}/{asset.path}", asset.content, asset.mode)
        for asset in assets
    )
    manifest = release_manifest(
        archive_payloads,
        version,
        commit,
        source_date_epoch,
        wasm_bindgen_version,
        asset_digest,
    )
    archive_payloads.append(Payload("RELEASE-MANIFEST.json", manifest))
    archive_payloads.sort(key=lambda payload: payload.path)

    output_directory.mkdir(parents=True, exist_ok=True)
    archive_path = output_directory / f"rspice-ide-{version}.zip"
    checksum_path = output_directory / f"rspice-ide-{version}.zip.sha256"
    prefix = f"rspice-ide-{version}/"
    with tempfile.NamedTemporaryFile(
        dir=output_directory, prefix=".rspice-ide-", suffix=".tmp", delete=False
    ) as temporary:
        temporary_path = Path(temporary.name)
    try:
        with zipfile.ZipFile(
            temporary_path,
            "w",
            compression=zipfile.ZIP_DEFLATED,
            compresslevel=9,
            strict_timestamps=True,
        ) as archive:
            for payload in archive_payloads:
                archive.writestr(
                    zip_info(prefix + payload.path, source_date_epoch, payload.mode),
                    payload.content,
                    compress_type=zipfile.ZIP_DEFLATED,
                    compresslevel=9,
                )
        archive_bytes = temporary_path.read_bytes()
        digest = hashlib.sha256(archive_bytes).hexdigest()
        temporary_path.replace(archive_path)
        checksum_path.write_text(
            f"{digest}  {archive_path.name}\n", encoding="ascii", newline="\n"
        )
    finally:
        temporary_path.unlink(missing_ok=True)
    return archive_path, checksum_path, digest


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--web-root", type=Path, default=DEFAULT_WEB_ROOT)
    parser.add_argument("--output-directory", type=Path, required=True)
    parser.add_argument("--version", default=workspace_version())
    parser.add_argument("--commit", required=True)
    parser.add_argument("--source-date-epoch", required=True, type=int)
    parser.add_argument("--wasm-bindgen-version", required=True)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        archive, checksum, digest = assemble(
            args.web_root.resolve(),
            args.output_directory.resolve(),
            args.version,
            args.commit,
            args.source_date_epoch,
            args.wasm_bindgen_version,
        )
    except (OSError, PackageError) as error:
        raise SystemExit(f"browser package failed: {error}") from error
    print(f"created {archive}")
    print(f"created {checksum}")
    print(f"sha256 {digest}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
