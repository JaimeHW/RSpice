#!/usr/bin/env python3
"""Stage and verify a relocatable, app-local CPython runtime for RSpice."""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import tempfile
from pathlib import Path


class StagingError(ValueError):
    """The candidate interpreter cannot become a managed RSpice payload."""


INSPECT_PROGRAM = r"""
import json, sys, sysconfig
print(json.dumps({
    "version": ".".join(map(str, sys.version_info[:3])),
    "base_prefix": sys.base_prefix,
    "executable": sys.executable,
    "cache_tag": sys.implementation.cache_tag,
    "platform": sysconfig.get_platform(),
}, sort_keys=True))
"""

EXCLUDED_DIRECTORIES = {
    ".git",
    "__pycache__",
    "ensurepip",
    "idle_test",
    "pip",
    "setuptools",
    "test",
    "tests",
}
EXCLUDED_METADATA_PREFIXES = ("pip-", "setuptools-")


def inspect_python(executable: Path) -> dict[str, str]:
    if not executable.is_file():
        raise StagingError(f"Python executable is not a regular file: {executable}")
    try:
        completed = subprocess.run(
            [str(executable), "-I", "-S", "-c", INSPECT_PROGRAM],
            check=True,
            capture_output=True,
            text=True,
            timeout=30,
        )
        inspected = json.loads(completed.stdout)
    except (OSError, subprocess.SubprocessError, json.JSONDecodeError) as error:
        raise StagingError(f"could not inspect candidate Python: {error}") from error
    required = {"version", "base_prefix", "executable", "cache_tag", "platform"}
    if not isinstance(inspected, dict) or not required.issubset(inspected):
        raise StagingError("candidate Python returned an incomplete identity")
    return {name: str(inspected[name]) for name in required}


def ensure_contained_symlinks(source_root: Path) -> None:
    canonical_root = source_root.resolve(strict=True)
    for path in source_root.rglob("*"):
        if not path.is_symlink():
            continue
        try:
            target = path.resolve(strict=True)
        except OSError as error:
            raise StagingError(f"runtime contains a broken symlink: {path}: {error}") from error
        if not target.is_relative_to(canonical_root):
            raise StagingError(f"runtime symlink escapes its installation root: {path} -> {target}")


def copy_runtime(source_root: Path, destination: Path) -> None:
    ensure_contained_symlinks(source_root)

    def ignore(_directory: str, names: list[str]) -> set[str]:
        return {
            name
            for name in names
            if name in EXCLUDED_DIRECTORIES
            or name.endswith(".pyc")
            or (
                name.lower().endswith((".dist-info", ".egg-info"))
                and name.lower().startswith(EXCLUDED_METADATA_PREFIXES)
            )
        }

    shutil.copytree(source_root, destination, symlinks=False, ignore=ignore)


def relative_member(root: Path, member: Path, label: str) -> str:
    try:
        return member.resolve(strict=True).relative_to(root.resolve(strict=True)).as_posix()
    except (OSError, ValueError) as error:
        raise StagingError(f"{label} is outside the candidate Python installation") from error


def license_members(root: Path) -> list[str]:
    members = [
        path.relative_to(root).as_posix()
        for path in root.rglob("*")
        if path.is_file() and (path.name.upper().startswith("LICENSE") or path.name == "COPYING")
    ]
    if not members:
        raise StagingError("candidate Python contains no redistributable license notice")
    return sorted(members)


def distribution_build(root: Path) -> str:
    build_file = root / "BUILD"
    if not build_file.is_file() or build_file.is_symlink():
        raise StagingError("candidate Python does not identify its standalone distribution build")
    try:
        build = build_file.read_text(encoding="utf-8").strip()
    except (OSError, UnicodeError) as error:
        raise StagingError(f"could not read candidate Python distribution build: {error}") from error
    has_control_text = any(
        ord(character) < 32 or ord(character) == 127 for character in build
    )
    if not build or len(build) > 128 or has_control_text:
        raise StagingError("candidate Python distribution build identity is malformed")
    return build


def write_metadata(
    runtime_root: Path,
    identity: dict[str, str],
    source_identity: str,
    distribution_build_identity: str,
    licenses: list[str],
) -> None:
    notice = {
        "schema": "rspice.managed-python-notice/v1",
        "component": "CPython",
        "version": identity["version"],
        "abi": identity["cache_tag"],
        "platform": identity["platform"],
        "distribution_source": source_identity,
        "distribution_build": distribution_build_identity,
        "license_files": licenses,
        "excluded_directories": sorted(EXCLUDED_DIRECTORIES)
        + ["pip-*.dist-info", "setuptools-*.dist-info"],
        "runtime_policy": "isolated-app-local-no-system-fallback",
    }
    (runtime_root / "PYTHON-RUNTIME-NOTICE.json").write_text(
        json.dumps(notice, indent=2, sort_keys=True) + "\n", encoding="utf-8", newline="\n"
    )
    sbom = {
        "bomFormat": "CycloneDX",
        "specVersion": "1.5",
        "serialNumber": f"urn:uuid:rspice-managed-cpython-{identity['version']}-{identity['platform']}",
        "version": 1,
        "components": [
            {
                "type": "application",
                "bom-ref": f"pkg:generic/cpython@{identity['version']}",
                "name": "CPython managed runtime",
                "version": identity["version"],
                "licenses": [{"license": {"id": "PSF-2.0"}}],
                "properties": [
                    {"name": "rspice:python-abi", "value": identity["cache_tag"]},
                    {"name": "rspice:platform", "value": identity["platform"]},
                    {"name": "rspice:distribution-source", "value": source_identity},
                    {
                        "name": "rspice:distribution-build",
                        "value": distribution_build_identity,
                    },
                ],
            }
        ],
    }
    (runtime_root / "PYTHON-RUNTIME-SBOM.cdx.json").write_text(
        json.dumps(sbom, indent=2, sort_keys=True) + "\n", encoding="utf-8", newline="\n"
    )


def stage_runtime(
    *,
    python_executable: Path,
    output_root: Path,
    worker: Path,
    expected_version: str,
    source_identity: str,
) -> tuple[Path, str, dict[str, str], str]:
    identity = inspect_python(python_executable)
    if identity["version"] != expected_version:
        raise StagingError(
            f"candidate Python is {identity['version']}; exactly {expected_version} is required"
        )
    if identity["cache_tag"] != "cpython-314":
        raise StagingError(f"candidate Python ABI is {identity['cache_tag']}; cpython-314 is required")
    source_root = Path(identity["base_prefix"]).resolve(strict=True)
    source_executable = Path(identity["executable"]).resolve(strict=True)
    python_relative = relative_member(source_root, source_executable, "Python executable")
    license_members(source_root)
    build_identity = distribution_build(source_root)
    if not worker.is_file() or worker.is_symlink():
        raise StagingError(f"worker bootstrap must be a regular repository file: {worker}")
    if output_root.exists():
        raise StagingError(f"runtime staging destination already exists: {output_root}")
    output_root.parent.mkdir(parents=True, exist_ok=True)
    temporary = Path(tempfile.mkdtemp(prefix=f".{output_root.name}.", dir=output_root.parent))
    staged = temporary / output_root.name
    try:
        copy_runtime(source_root, staged)
        worker_relative = "worker/rspice_worker.py"
        worker_destination = staged / worker_relative
        worker_destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(worker, worker_destination)
        staged_licenses = license_members(staged)
        if distribution_build(staged) != build_identity:
            raise StagingError("staged interpreter changed its distribution build identity")
        write_metadata(staged, identity, source_identity, build_identity, staged_licenses)
        staged_executable = staged / Path(python_relative)
        staged_identity = inspect_python(staged_executable)
        if staged_identity["version"] != expected_version:
            raise StagingError("staged interpreter did not preserve its exact Python identity")
        os.replace(staged, output_root)
    except Exception:
        shutil.rmtree(temporary, ignore_errors=True)
        raise
    shutil.rmtree(temporary, ignore_errors=True)
    return output_root, python_relative, identity, build_identity


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--python-executable", type=Path, required=True)
    parser.add_argument("--output-root", type=Path, required=True)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--expected-version", default="3.14.6")
    parser.add_argument("--source-identity", required=True)
    return parser.parse_args()


def main() -> int:
    arguments = parse_args()
    try:
        root, executable, identity, build_identity = stage_runtime(
            python_executable=arguments.python_executable.resolve(),
            output_root=arguments.output_root.resolve(),
            worker=arguments.worker.resolve(),
            expected_version=arguments.expected_version,
            source_identity=arguments.source_identity,
        )
    except StagingError as error:
        raise SystemExit(f"managed Python staging failed: {error}") from error
    print(f"runtime_root={root}")
    print(f"python_executable={executable}")
    print(f"python_version={identity['version']}")
    print(f"python_abi={identity['cache_tag'].replace('cpython-', 'cp')}")
    print(f"distribution_build={build_identity}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
