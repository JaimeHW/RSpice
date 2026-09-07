#!/usr/bin/env python3
"""Report browser WebAssembly size and enforce explicitly supplied budgets."""

from __future__ import annotations

import argparse
import gzip
from pathlib import Path


def positive_bytes(value: str) -> int:
    parsed = int(value)
    if parsed <= 0:
        raise argparse.ArgumentTypeError("size budget must be positive")
    return parsed


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("artifact", type=Path)
    parser.add_argument("--label", required=True)
    parser.add_argument("--max-raw", type=positive_bytes)
    parser.add_argument("--max-gzip", type=positive_bytes)
    args = parser.parse_args()

    try:
        payload = args.artifact.read_bytes()
    except OSError as error:
        parser.error(f"cannot read {args.artifact}: {error}")
    if not payload.startswith(b"\x00asm\x01\x00\x00\x00"):
        parser.error(f"{args.artifact} is not a WebAssembly version 1 module")
    compressed = gzip.compress(payload, compresslevel=9, mtime=0)
    raw_bytes = len(payload)
    gzip_bytes = len(compressed)
    print(
        f"{args.label}: raw={raw_bytes} bytes "
        f"({raw_bytes / 1024 / 1024:.2f} MiB), gzip={gzip_bytes} bytes "
        f"({gzip_bytes / 1024 / 1024:.2f} MiB)"
    )
    failures = []
    if args.max_raw is not None and raw_bytes > args.max_raw:
        failures.append(f"raw {raw_bytes} > {args.max_raw}")
    if args.max_gzip is not None and gzip_bytes > args.max_gzip:
        failures.append(f"gzip {gzip_bytes} > {args.max_gzip}")
    if failures:
        print(f"{args.label} exceeds release budget: {', '.join(failures)}")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
