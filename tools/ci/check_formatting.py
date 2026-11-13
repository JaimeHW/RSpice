#!/usr/bin/env python3
"""Check or apply workspace formatting without overflowing a command line."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

# Generated Verilog-A model crates hold machine-written sources that rustfmt is
# not asked to own.
EXCLUDED_MEMBERS = {"rspice-veriloga-models"}
EXCLUDED_MEMBER_PREFIXES = ("rspice-veriloga-model-",)

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


def missing_members(described: dict, root: Path = ROOT) -> list[str]:
    """The workspace members `Cargo.toml` declares that cargo did not describe.

    The declared set is read from the manifest's `members` globs, not from
    cargo, so a metadata read that returned a handful of packages cannot report
    success for having checked almost nothing.
    """
    workspace = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8")).get(
        "workspace", {}
    )
    skipped = {(root / entry).resolve() for entry in workspace.get("exclude", [])}
    declared = {
        directory.resolve()
        for pattern in workspace.get("members", [])
        for directory in root.glob(pattern)
        if (directory / "Cargo.toml").is_file()
    } - skipped
    found = {
        Path(package["manifest_path"]).resolve().parent for package in described["packages"]
    }
    return sorted(
        directory.relative_to(root.resolve()).as_posix() for directory in declared - found
    )


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


def run_rustfmt(command: list[str]) -> subprocess.CompletedProcess:
    """Run one rustfmt batch, reading its output as the UTF-8 it is.

    rustfmt writes the source lines of a diff, and this workspace's sources
    are UTF-8. Decoding with the locale's codec instead, which is cp1252 on a
    Windows checkout, fails on the first typographic quote or box-drawing
    character a diff quotes, and the gate would crash rather than name the
    file.
    """
    return subprocess.run(
        command,
        cwd=ROOT,
        text=True,
        encoding="utf-8",
        errors="replace",
        capture_output=True,
        check=False,
    )


def format_members(
    grouped: dict[tuple[str, str], list[str]],
    *,
    write: bool,
    runner=None,
) -> tuple[list[str], list[str], int]:
    """Check or format every batch, and report everything that went wrong.

    Every batch runs whatever an earlier one reported, because a gate that
    stops at its first failure hides the rest, which is the defect this script
    replaced. Returns the unformatted files, a line per batch rustfmt could
    not check, and the number of targets it was given.
    """
    runner = runner or run_rustfmt
    unformatted: list[str] = []
    failures: list[str] = []
    targets = 0
    for (member, edition), paths in grouped.items():
        targets += len(paths)
        for run in batches(paths):
            command = ["rustfmt", "--edition", edition]
            if not write:
                command.append("--check")
            completed = runner([*command, *run])
            stdout = completed.stdout or ""
            found = offenders(stdout)
            for path in found:
                if path not in unformatted:
                    unformatted.append(path)
            problem = (completed.stderr or "").strip()
            if completed.returncode != 0 and (problem or not found):
                # A diff is reported on stdout and nothing else; anything on
                # stderr is a parse error, a missing file or a command line
                # rustfmt could not take, and a diff in the same run must not
                # hide it behind advice to run --write. A failure that named
                # no file at all is not a pass either, and what it did print
                # is kept, because it is the only account of what went wrong.
                said = problem or stdout.strip()
                failures.append(
                    f"rustfmt could not check {member} (exit status {completed.returncode})"
                    + (f": {said}" if said else "")
                )
    return unformatted, failures, targets


def metadata() -> dict:
    """Describe the workspace, keeping cargo's own words when it refuses.

    `--no-deps` resolves nothing, so cargo does not read `Cargo.lock` here and
    `--locked` cannot trip; it is passed because every cargo call in CI is.
    What does make this fail is a manifest cargo cannot load. Capturing stderr
    is what lets the caller print cargo's reason instead of raising a
    traceback that reads as a broken gate.
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
        described = metadata()
    except subprocess.CalledProcessError as refusal:
        reason = (refusal.stderr or "").strip() or f"exit status {refusal.returncode}"
        print(f"cargo could not describe the workspace:\n{reason}")
        return 1
    missing = missing_members(described)
    if missing:
        print("cargo metadata did not describe these workspace members:")
        for member in missing:
            print(f"  {member}")
        return 1
    grouped = entry_points(described)
    members = {member for member, _ in grouped}
    if not members:
        print("cargo metadata named no formattable workspace member")
        return 1

    unformatted, failures, targets = format_members(grouped, write=arguments.write)

    for path in unformatted:
        print(f"unformatted: {path}")
    for failure in failures:
        print(failure)
    if unformatted and not failures:
        print(f"{len(unformatted)} file(s) need rustfmt; run with --write to format them")
    elif unformatted:
        # --write cannot fix a file that does not parse, so the advice is
        # given only alongside the failures that say what has to come first.
        print(
            f"{len(unformatted)} file(s) need rustfmt, and rustfmt could not check "
            f"{len(failures)} batch(es); fix those first, then run with --write"
        )
    if unformatted or failures:
        return 1

    action = "formatted" if arguments.write else "checked"
    print(f"rustfmt {action} {targets} target(s) across {len(members)} workspace member(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
