#!/usr/bin/env python3
"""Compare two emitted `TRIFAMILY` fingerprint sets from the mixed suites.

The mixed-signal suites print every pinned quantity as a `TRIFAMILY` line when
`RSPICE_TRI_FAMILY_EMIT=1` is set, asserting none of them. Running a suite
twice under two builds and diffing those lines is how the nightly asks the one
question no pin can ask itself: does this engine land on the same accepted grid
when the parallel reductions underneath it are gone?

A line is one record. The key is what precedes the first `=` after the
`TRIFAMILY ` marker; the value is the rest of the line, so a diagnostic line
that carries several `key=value` pairs compares as a whole and a value with
spaces in it (a printed vector) survives. The marker is looked for anywhere in
the line rather than at its start: under `--test-threads 1 --nocapture`,
libtest writes `test <name> ... ` with no newline, so each test's first printed
line arrives with that prefix attached and an anchored match would drop exactly
one record per test.

What the comparison gates on, by default, is the fingerprint channel: keys
ending in `_points`, `_grid_hash`, `_volt_hash` or `_cross_process_children`.
Everything else a suite prints under `TRIFAMILY` is diagnostic context — a
worst-case relative difference, a list of digital instants — and a gate that
failed on those would be measuring the diagnostics rather than the answer.
`--gate-all` compares every key instead.
"""

from __future__ import annotations

import argparse
from pathlib import Path

# The suffixes that make a key part of the determinism channel rather than
# diagnostic context.
FINGERPRINT_SUFFIXES = ("_points", "_grid_hash", "_volt_hash", "_cross_process_children")

PREFIX = "TRIFAMILY "


def positive(value: str) -> int:
    parsed = int(value)
    if parsed <= 0:
        raise argparse.ArgumentTypeError("must be positive")
    return parsed


def is_gated(key: str, gate_all: bool) -> bool:
    return gate_all or key.endswith(FINGERPRINT_SUFFIXES)


def read_records(text: str, gate_all: bool) -> tuple[dict[str, str], list[str]]:
    """Every gated record in a log, plus the keys that disagreed inside it.

    A key printed twice with two values in ONE run is non-determinism within a
    single process, which is a stronger finding than any difference between two
    builds and is reported as its own failure.
    """
    records: dict[str, str] = {}
    inconsistent: list[str] = []
    for line in text.splitlines():
        stripped = line.strip()
        if PREFIX not in stripped:
            continue
        record = stripped.split(PREFIX, 1)[1]
        key, separator, value = record.partition("=")
        if not separator:
            continue
        key = key.strip()
        value = value.strip()
        if not is_gated(key, gate_all):
            continue
        if key in records and records[key] != value:
            if key not in inconsistent:
                inconsistent.append(key)
        records[key] = value
    return records, inconsistent


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("log_a", type=Path)
    parser.add_argument("log_b", type=Path)
    parser.add_argument("--label-a", default="A")
    parser.add_argument("--label-b", default="B")
    parser.add_argument(
        "--min-keys",
        type=positive,
        required=True,
        help="fail unless each log carries at least this many gated records, so a "
        "run that emitted nothing cannot pass as agreement",
    )
    parser.add_argument(
        "--require-key",
        action="append",
        default=[],
        metavar="KEY",
        help="a key both logs must carry; name one per suite so a suite that did "
        "not run fails instead of shrinking the comparison",
    )
    parser.add_argument(
        "--allow-differing",
        action="append",
        default=[],
        metavar="KEY",
        help="a key whose two values are a recorded, explained difference rather "
        "than a regression; the two values are still printed",
    )
    parser.add_argument("--gate-all", action="store_true")
    args = parser.parse_args()

    failures: list[str] = []
    sides = []
    for label, path in ((args.label_a, args.log_a), (args.label_b, args.log_b)):
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError as error:
            parser.error(f"cannot read {path}: {error}")
        records, inconsistent = read_records(text, args.gate_all)
        for key in inconsistent:
            failures.append(f"{label}: {key} was printed with two different values in one run")
        if len(records) < args.min_keys:
            failures.append(
                f"{label}: {len(records)} fingerprint records, fewer than the "
                f"{args.min_keys} required; the suites did not all run"
            )
        for key in args.require_key:
            if key not in records:
                failures.append(f"{label}: required key {key} is missing")
        sides.append((label, records))

    (label_a, left), (label_b, right) = sides
    for key in sorted(set(left) - set(right)):
        failures.append(f"{key}: present in {label_a}, missing from {label_b}")
    for key in sorted(set(right) - set(left)):
        failures.append(f"{key}: present in {label_b}, missing from {label_a}")

    shared = sorted(set(left) & set(right))
    allowed = set(args.allow_differing)
    differing = [key for key in shared if left[key] != right[key]]
    for key in differing:
        line = f"{key}: {label_a}={left[key]!r} {label_b}={right[key]!r}"
        if key in allowed:
            print(f"recorded difference {line}")
        else:
            failures.append(line)

    if failures:
        print(f"{label_a} and {label_b} do not agree:")
        for failure in failures:
            print(f"  {failure}")
        return 1
    print(
        f"{label_a} and {label_b} agree on {len(shared)} fingerprint records "
        f"({len(allowed & set(shared))} recorded difference(s) allowed)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
