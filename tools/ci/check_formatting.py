#!/usr/bin/env python3
"""Check or apply workspace formatting without overflowing a command line."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

# Generated Verilog-A model crates hold machine-written sources that rustfmt is
# not asked to own.
EXCLUDED_MEMBERS = {"rspice-veriloga-models"}
EXCLUDED_MEMBER_PREFIXES = ("rspice-veriloga-model-",)

# The workspace has not had fewer members than this since the application
# crates were extracted. Without the floor, a metadata read that returned a
# handful of packages would report success for having checked almost nothing.
MINIMUM_MEMBERS = 41

# Windows caps a command line at 32767 bytes for the whole line, and cargo fmt
# hands every target of a package to one rustfmt process. rspice-core declares
# enough targets to exceed that on its own, which is why `cargo fmt -p
# rspice-core` cannot start there while the same command succeeds on Linux.
# Batching to a budget keeps one invocation per package in the common case and
# splits only the packages that would not fit.
ARGUMENT_BUDGET = 24000

DIFF_LINE = re.compile(r"^Diff in (.+?):\d+:$")


def excluded(member: str) -> bool:
    """Report whether the workflow has always skipped this member."""
    return member in EXCLUDED_MEMBERS or member.startswith(EXCLUDED_MEMBER_PREFIXES)


def entry_points(metadata: dict) -> dict[tuple[str, str], list[str]]:
    """Group every target rustfmt must be given by member and edition.

    rustfmt follows `mod` declarations, so a target's entry file covers the
    whole module tree beneath it. One row per target therefore reaches the same
    sources cargo fmt would, and a package that declares two targets over one
    file is only formatted once.
    """
    grouped: dict[tuple[str, str], list[str]] = {}
    for package in metadata["packages"]:
        member = package["name"]
        if excluded(member):
            continue
        for target in package["targets"]:
            key = (member, target["edition"])
            paths = grouped.setdefault(key, [])
            if target["src_path"] not in paths:
                paths.append(target["src_path"])
    return {key: sorted(paths) for key, paths in sorted(grouped.items())}


def batches(paths: list[str], budget: int = ARGUMENT_BUDGET) -> list[list[str]]:
    """Split paths into runs that fit one command line.

    A single path longer than the budget still goes out on its own: refusing it
    would skip a file, and the shorter line may well be accepted.
    """
    runs: list[list[str]] = []
    current: list[str] = []
    width = 0
    for path in paths:
        cost = len(path) + 3
        if current and width + cost > budget:
            runs.append(current)
            current = []
            width = 0
        current.append(path)
        width += cost
    if current:
        runs.append(current)
    return runs


def reportable(path: str) -> str:
    """Name a file the way the rest of the repository's checks name one.

    On Windows rustfmt reports an extended-length path, which no amount of
    `relative_to` will place inside the workspace until the `\\\\?\\` prefix is
    off. A path that still does not sit under the root is printed as rustfmt
    gave it rather than discarded.
    """
    cleaned = path.removeprefix("\\\\?\\")
    try:
        return Path(cleaned).relative_to(ROOT).as_posix()
    except ValueError:
        return Path(cleaned).as_posix()


def offenders(output: str) -> list[str]:
    """The files rustfmt reported a diff for, in the order it reported them."""
    found: list[str] = []
    for line in output.splitlines():
        match = DIFF_LINE.match(line.strip())
        if match:
            path = reportable(match.group(1))
            if path not in found:
                found.append(path)
    return found


def metadata() -> dict:
    """Describe the workspace, keeping cargo's own words when it refuses.

    `--locked` is deliberate: a gate checks the workspace the lockfile pins
    rather than quietly resolving a new one. It also means a tree whose
    `Cargo.lock` is out of step with its manifests fails here, which is the
    ordinary state of a landing tree that merged two branches each adding a
    dependency. Capturing stderr is what lets the caller say the tree is
    stale instead of raising a traceback that reads as a broken gate.
    """
    return json.loads(
        subprocess.check_output(
            ("cargo", "metadata", "--locked", "--no-deps", "--format-version", "1"),
            cwd=ROOT,
            text=True,
            stderr=subprocess.PIPE,
        )
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--write",
        action="store_true",
        help="format in place instead of reporting what is unformatted",
    )
    arguments = parser.parse_args()

    try:
        grouped = entry_points(metadata())
    except subprocess.CalledProcessError as refusal:
        print(f"cargo could not describe the workspace:\n{(refusal.stderr or '').strip()}")
        print(
            "This gate reads the workspace under --locked, so Cargo.lock must "
            "match the manifests. Run a cargo command without --locked to "
            "update it and commit the lockfile with the manifest that moved."
        )
        return 1
    members = {member for member, _ in grouped}
    if len(members) < MINIMUM_MEMBERS:
        print(
            f"cargo metadata named {len(members)} formattable member(s), "
            f"fewer than the {MINIMUM_MEMBERS} this workspace has"
        )
        return 1

    unformatted: list[str] = []
    targets = 0
    for (member, edition), paths in grouped.items():
        targets += len(paths)
        for run in batches(paths):
            command = ["rustfmt", "--edition", edition]
            if not arguments.write:
                command.append("--check")
            completed = subprocess.run(
                [*command, *run],
                cwd=ROOT,
                text=True,
                capture_output=True,
                check=False,
            )
            for path in offenders(completed.stdout):
                if path not in unformatted:
                    unformatted.append(path)
            if completed.returncode != 0 and not completed.stdout:
                # rustfmt failed without reporting a diff: a parse error, a
                # missing file, or a command line it still could not take.
                print(f"rustfmt could not check {member}: {completed.stderr.strip()}")
                return 1

    if unformatted:
        for path in unformatted:
            print(f"unformatted: {path}")
        print(f"{len(unformatted)} file(s) need rustfmt; run with --write to format them")
        return 1

    action = "formatted" if arguments.write else "checked"
    print(f"rustfmt {action} {targets} target(s) across {len(members)} workspace member(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
