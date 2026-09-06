#!/usr/bin/env python3
"""Fail closed when the AArch64 build of a crate errors or warns.

The ARM CI job is the only place the AArch64 machine backend compiles, and it
builds with `-D warnings`. Nothing local reproduced it: its test binary stopped
compiling on 2026-09-02 and three days of merges went in over the red, because
every gate a change is run through here is a host-architecture gate. This is
that missing gate.

No ARM runner is needed. The Android NDK ships an `aarch64-linux-android`
toolchain that cross-compiles the crate's own C runtime (`aarch64_runtime.c`),
so an x86-64 host with an NDK type-checks the whole AArch64 tree, tests
included. Without an NDK — or without the Rust target — this reports what is
missing and skips, so it is an addition to the local gate rather than a new
dependency for it.

Usage:
    python3 tools/ci/check_aarch64_cross.py
    python3 tools/ci/check_aarch64_cross.py -p rspice-veriloga --features native
"""

from __future__ import annotations

import argparse
import os
import platform
import re
import subprocess
import sys
from pathlib import Path

TARGET = "aarch64-linux-android"

# The API level the toolchain wrapper is named for. 21 is the floor the NDK
# still ships an `aarch64` wrapper for, and nothing here runs on a device: the
# level only selects which wrapper script exists.
API_LEVEL = 21


def ndk_search_roots() -> list[Path]:
    """Where an NDK installation may be, most explicit first."""
    roots = []
    for variable in ("ANDROID_NDK_HOME", "ANDROID_NDK_ROOT", "ANDROID_NDK"):
        value = os.environ.get(variable)
        if value:
            # These name one NDK, not the directory that holds several.
            roots.append(Path(value).parent)
    sdk = os.environ.get("ANDROID_SDK_ROOT") or os.environ.get("ANDROID_HOME")
    if sdk:
        roots.append(Path(sdk) / "ndk")
    home = Path.home()
    local_app_data = os.environ.get("LOCALAPPDATA")
    if local_app_data:
        roots.append(Path(local_app_data) / "Android" / "Sdk" / "ndk")
    roots.append(home / "Library" / "Android" / "sdk" / "ndk")
    roots.append(home / "Android" / "Sdk" / "ndk")
    return roots


def version_key(name: str) -> tuple[int, ...]:
    return tuple(int(part) for part in re.findall(r"\d+", name)) or (0,)


def newest_ndk_bin() -> Path | None:
    """The `bin` directory of the newest installed NDK, if there is one."""
    candidates = []
    for root in ndk_search_roots():
        if not root.is_dir():
            continue
        for installed in sorted(root.iterdir(), key=lambda path: version_key(path.name)):
            # One prebuilt host toolchain per NDK; the host tag varies
            # (`windows-x86_64`, `darwin-x86_64`, `linux-x86_64`), so it is
            # matched rather than spelled.
            candidates.extend(sorted(installed.glob("toolchains/llvm/prebuilt/*/bin")))
    for bin_dir in reversed(candidates):
        if compiler_path(bin_dir) is not None and archiver_path(bin_dir) is not None:
            return bin_dir
    return None


def compiler_path(bin_dir: Path) -> Path | None:
    for name in (
        f"{TARGET}{API_LEVEL}-clang.cmd",
        f"{TARGET}{API_LEVEL}-clang",
        f"{TARGET}{API_LEVEL}-clang.exe",
    ):
        candidate = bin_dir / name
        if candidate.is_file():
            return candidate
    return None


def archiver_path(bin_dir: Path) -> Path | None:
    for name in ("llvm-ar.exe", "llvm-ar"):
        candidate = bin_dir / name
        if candidate.is_file():
            return candidate
    return None


def rust_target_installed() -> bool:
    result = subprocess.run(
        ["rustup", "target", "list", "--installed"],
        capture_output=True,
        text=True,
        check=False,
    )
    return result.returncode == 0 and TARGET in result.stdout.split()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-p",
        "--package",
        action="append",
        default=None,
        help="package to check (repeatable); defaults to rspice-veriloga",
    )
    parser.add_argument(
        "--features",
        default="native",
        help="comma-separated features to enable (default: native)",
    )
    parser.add_argument(
        "--require-toolchain",
        action="store_true",
        help="fail instead of skipping when the NDK or the Rust target is absent",
    )
    args = parser.parse_args()
    packages = args.package or ["rspice-veriloga"]

    if platform.machine().lower() in {"arm64", "aarch64"}:
        print(
            f"{TARGET} check: this host is already AArch64; "
            "the ordinary host gates cover it"
        )
        return 0

    missing = []
    bin_dir = newest_ndk_bin()
    if bin_dir is None:
        missing.append(
            "no Android NDK with an "
            f"{TARGET}{API_LEVEL} toolchain was found (searched "
            f"{', '.join(str(root) for root in ndk_search_roots())}); install one "
            "with `sdkmanager --install ndk-bundle`, or set ANDROID_NDK_HOME"
        )
    if not rust_target_installed():
        missing.append(f"the Rust target is absent; run `rustup target add {TARGET}`")
    if missing:
        report = "\n  ".join(missing)
        if args.require_toolchain:
            print(f"{TARGET} check cannot run:\n  {report}")
            return 1
        print(f"{TARGET} check skipped:\n  {report}")
        return 0

    compiler = compiler_path(bin_dir)
    archiver = archiver_path(bin_dir)
    assert compiler is not None and archiver is not None

    environment = dict(os.environ)
    environment[f"CC_{TARGET.replace('-', '_')}"] = str(compiler)
    environment[f"AR_{TARGET.replace('-', '_')}"] = str(archiver)
    # The ARM job's own setting. Registry dependencies are built with
    # `--cap-lints allow`, so this is a gate on this workspace's code only.
    rustflags = environment.get("RUSTFLAGS", "")
    environment["RUSTFLAGS"] = f"{rustflags} -D warnings".strip()

    command = ["cargo", "check", "--locked"]
    for package in packages:
        command += ["-p", package]
    if args.features:
        command += ["--features", args.features]
    command += ["--lib", "--tests", "--target", TARGET]

    # Flushed, so the header stays above the output of the build it describes.
    print(f"{TARGET} check: {' '.join(command)}")
    print(f"  CC={compiler}")
    print(f"  AR={archiver}", flush=True)
    completed = subprocess.run(command, env=environment, check=False)
    if completed.returncode != 0:
        print(
            f"{TARGET} check failed: the AArch64 build errors or warns. CI builds "
            "this tree with `-D warnings` and nothing else compiles it, so a "
            "warning here is a red ARM job.",
            file=sys.stderr,
        )
        return completed.returncode
    print(f"{TARGET} check: clean")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
