#!/usr/bin/env python3
"""Run and retain the focused automated drawing-sheet qualification gates.

The resulting JSON is evidence for the automated drawing-sheet domain only.
It deliberately does not claim that physical printers, browser/OS print
dialogs, native macOS interaction, or assistive technologies were qualified.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import platform
import re
import shlex
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Sequence


ROOT = Path(__file__).resolve().parents[2]
PROCEDURE = "security/DRAWING_SHEET_RELEASE_QUALIFICATION.md"
MAX_SOURCE_LINES = 2_500
SCHEMA_VERSION = 1

SUPPORTED_TARGETS = frozenset(
    {
        "x86_64-unknown-linux-gnu",
        "aarch64-unknown-linux-gnu",
        "x86_64-pc-windows-msvc",
        "aarch64-pc-windows-msvc",
        "x86_64-apple-darwin",
        "aarch64-apple-darwin",
        "wasm32-unknown-unknown",
    }
)

SHEET_SOURCE_SCOPES = (
    "crates/rspice-ui/src/hardcopy",
    "crates/rspice-ui/src/state/design_management/drawing_sheet.rs",
    "crates/rspice-ui/src/state/design_management/drawing_sheet",
    "crates/rspice-ui/src/schematic/view/drawing_sheet.rs",
    "crates/rspice-ui/src/workbench/app/dialogs/drawing_sheet_setup.rs",
    "crates/rspice-ui/src/workbench/app/dialogs/drawing_sheet_setup",
    "crates/rspice-ui/src/workbench/app/dialogs/hardcopy.rs",
    "crates/rspice-ui/src/workbench/app/dialogs/hardcopy",
    "crates/rspice-ui/src/workbench/hardcopy_adapters",
)

QUALIFICATION_INPUTS = (
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "crates/rspice-core/Cargo.toml",
    "crates/rspice-core/tests/save_directives.rs",
    "crates/rspice-ui/Cargo.toml",
    "crates/rspice-sheet-publisher/Cargo.toml",
    "crates/rspice-sheet-publisher/src",
    PROCEDURE,
    ".github/workflows/drawing-sheet-qualification.yml",
    "tools/ci/qualify_drawing_sheet.py",
    "tools/ci/test_qualify_drawing_sheet.py",
)

# These are local orchestration workspaces, not RSpice build inputs. Git
# reports a nested repository as one untracked directory, so excluding these
# prefixes avoids binding evidence to another agent's checkout while every
# real nonignored file in this worktree remains covered.
WORKTREE_FINGERPRINT_EXCLUDED_PREFIXES = (
    ".codex-candidates/",
    ".qualification-worktrees/",
    ".worktrees/",
)

TEST_RESULT = re.compile(
    r"test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; "
    r"(\d+) ignored; (\d+) measured; (\d+) filtered out"
)
COMMIT = re.compile(r"^[0-9a-f]{40}$")


class QualificationError(ValueError):
    """The requested qualification cannot produce trustworthy evidence."""


@dataclass(frozen=True)
class Gate:
    gate_id: str
    label: str
    command: tuple[str, ...]
    minimum_passed_tests: int | None = None


@dataclass(frozen=True)
class CommandOutcome:
    returncode: int | None
    stdout: str
    stderr: str
    timed_out: bool = False


@dataclass(frozen=True)
class SourceState:
    commit: str
    branch: str
    status_porcelain: str
    input_digest: str
    input_file_count: int
    worktree_digest: str = ""
    worktree_file_count: int = 0

    @property
    def dirty(self) -> bool:
        return bool(self.status_porcelain)


CommandExecutor = Callable[[Sequence[str], Path, int], CommandOutcome]


def utc_now() -> str:
    return dt.datetime.now(dt.UTC).isoformat(timespec="milliseconds").replace("+00:00", "Z")


def sha256_bytes(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def sha256_file(path: Path) -> str:
    if not path.is_file() or path.is_symlink():
        raise QualificationError(f"qualification input must be a regular file: {path}")
    return sha256_bytes(path.read_bytes())


def canonical_json(document: object) -> bytes:
    return (json.dumps(document, indent=2, sort_keys=True) + "\n").encode("utf-8")


def collect_files(root: Path, scopes: Sequence[str]) -> tuple[list[Path], list[str]]:
    files: set[Path] = set()
    missing: list[str] = []
    for relative in scopes:
        path = root / relative
        if path.is_file() and not path.is_symlink():
            files.add(path)
        elif path.is_dir() and not path.is_symlink():
            for candidate in path.rglob("*.rs"):
                if candidate.is_symlink():
                    missing.append(f"{candidate.relative_to(root).as_posix()} (symlink)")
                elif candidate.is_file():
                    files.add(candidate)
        else:
            missing.append(relative)
    return sorted(files, key=lambda path: path.relative_to(root).as_posix()), missing


def qualification_input_fingerprint(
    root: Path, sheet_scopes: Sequence[str] = SHEET_SOURCE_SCOPES
) -> tuple[str, int]:
    files, missing = collect_files(root, tuple(sheet_scopes) + QUALIFICATION_INPUTS)
    if missing:
        raise QualificationError(
            "qualification inputs are missing or are symlinks: " + ", ".join(missing)
        )
    manifest = []
    for path in files:
        content = path.read_bytes()
        manifest.append(
            {
                "path": path.relative_to(root).as_posix(),
                "size": len(content),
                "sha256": sha256_bytes(content),
            }
        )
    return sha256_bytes(canonical_json(manifest)), len(manifest)


def git_output(root: Path, *arguments: str) -> str:
    completed = subprocess.run(
        ("git", *arguments),
        cwd=root,
        check=False,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        shell=False,
    )
    if completed.returncode != 0:
        detail = completed.stderr.strip() or completed.stdout.strip()
        raise QualificationError(f"git {' '.join(arguments)} failed: {detail}")
    return completed.stdout


def git_bytes(root: Path, *arguments: str) -> bytes:
    completed = subprocess.run(
        ("git", *arguments),
        cwd=root,
        check=False,
        capture_output=True,
        shell=False,
    )
    if completed.returncode != 0:
        detail = completed.stderr.decode("utf-8", "replace").strip()
        if not detail:
            detail = completed.stdout.decode("utf-8", "replace").strip()
        raise QualificationError(f"git {' '.join(arguments)} failed: {detail}")
    return completed.stdout


def worktree_fingerprint(root: Path) -> tuple[str, int]:
    tracked_diff = git_bytes(
        root,
        "diff",
        "--binary",
        "--no-ext-diff",
        "--no-textconv",
        "--submodule=short",
        "HEAD",
        "--",
    )
    untracked_paths = git_bytes(
        root, "ls-files", "-z", "--others", "--exclude-standard"
    ).split(b"\0")
    untracked_manifest = []
    for raw_path in untracked_paths:
        if not raw_path:
            continue
        relative = raw_path.decode("utf-8", "surrogateescape").replace("\\", "/")
        if relative.startswith(WORKTREE_FINGERPRINT_EXCLUDED_PREFIXES):
            continue
        path = root / relative
        if path.is_symlink():
            target = os.readlink(path)
            content = os.fsencode(target)
            kind = "symlink"
        elif path.is_file():
            content = path.read_bytes()
            kind = "file"
        elif path.is_dir():
            content = b""
            kind = "directory"
        else:
            raise QualificationError(
                f"untracked worktree input disappeared during inspection: {relative}"
            )
        untracked_manifest.append(
            {
                "path": relative,
                "kind": kind,
                "size": len(content),
                "sha256": sha256_bytes(content),
            }
        )
    material = {
        "tracked_diff_bytes": len(tracked_diff),
        "tracked_diff_sha256": sha256_bytes(tracked_diff),
        "untracked": untracked_manifest,
    }
    return sha256_bytes(canonical_json(material)), len(untracked_manifest)


def inspect_source(
    root: Path, sheet_scopes: Sequence[str] = SHEET_SOURCE_SCOPES
) -> SourceState:
    commit = git_output(root, "rev-parse", "HEAD").strip()
    if not COMMIT.fullmatch(commit):
        raise QualificationError("HEAD did not resolve to a full lowercase Git commit")
    branch = git_output(root, "rev-parse", "--abbrev-ref", "HEAD").strip()
    status = git_output(
        root, "status", "--porcelain=v1", "--untracked-files=all"
    ).rstrip("\r\n")
    input_digest, input_file_count = qualification_input_fingerprint(root, sheet_scopes)
    worktree_digest, worktree_file_count = worktree_fingerprint(root)
    return SourceState(
        commit,
        branch,
        status,
        input_digest,
        input_file_count,
        worktree_digest,
        worktree_file_count,
    )


def normalized_architecture(machine: str) -> str:
    value = machine.strip().lower()
    if value in {"x86_64", "amd64", "x64"}:
        return "x86_64"
    if value in {"aarch64", "arm64"}:
        return "aarch64"
    return value


def target_matches_host(target: str, system: str, machine: str) -> bool:
    if target == "wasm32-unknown-unknown":
        return True
    expected_system = (
        "Windows"
        if "windows" in target
        else "Darwin"
        if "apple-darwin" in target
        else "Linux"
        if "linux" in target
        else ""
    )
    expected_arch = "aarch64" if target.startswith("aarch64-") else "x86_64"
    return system == expected_system and normalized_architecture(machine) == expected_arch


def validate_targets(targets: Sequence[str]) -> tuple[str, ...]:
    if not targets:
        raise QualificationError("at least one --target is required")
    if len(set(targets)) != len(targets):
        raise QualificationError("each qualification target may be specified only once")
    unsupported = sorted(set(targets) - SUPPORTED_TARGETS)
    if unsupported:
        raise QualificationError("unsupported qualification target: " + ", ".join(unsupported))
    return tuple(targets)


def command_gates(
    jobs: int, targets: Sequence[str], operating_system: str | None = None
) -> list[Gate]:
    cargo_jobs = str(jobs)
    host_system = operating_system or platform.system()
    gates = [
        Gate(
            "core-save-directives",
            "Core save/print directive contract",
            (
                "cargo",
                "test",
                "--locked",
                "-p",
                "rspice-core",
                "-j",
                cargo_jobs,
                "--test",
                "save_directives",
            ),
            7,
        ),
        Gate(
            "ui-drawing-sheet",
            "Drawing-sheet geometry, state, Page Setup, and preset tests",
            (
                "cargo",
                "test",
                "--locked",
                "-p",
                "rspice-ui",
                "-j",
                cargo_jobs,
                "--lib",
                "drawing_sheet",
            ),
            125,
        ),
        Gate(
            "ui-hardcopy",
            "Hardcopy contract, source, renderer, worker, and printer tests",
            (
                "cargo",
                "test",
                "--locked",
                "-p",
                "rspice-ui",
                "-j",
                cargo_jobs,
                "--lib",
                "hardcopy",
            ),
            201 if host_system == "Windows" else 198,
        ),
        Gate(
            "publisher-tests",
            "Offline governed drawing-sheet publisher tests",
            (
                "cargo",
                "test",
                "--locked",
                "-p",
                "rspice-sheet-publisher",
                "-j",
                cargo_jobs,
            ),
            9 if host_system == "Windows" else 7,
        ),
        Gate(
            "ui-native-check",
            "Native-host UI compile check",
            ("cargo", "check", "--locked", "-p", "rspice-ui", "-j", cargo_jobs),
        ),
        Gate(
            "publisher-native-check",
            "Native-host publisher compile check",
            (
                "cargo",
                "check",
                "--locked",
                "-p",
                "rspice-sheet-publisher",
                "-j",
                cargo_jobs,
            ),
        ),
    ]
    for target in targets:
        gates.append(
            Gate(
                f"ui-target-{target}",
                f"UI compile check for {target}",
                (
                    "cargo",
                    "check",
                    "--locked",
                    "-p",
                    "rspice-ui",
                    "-j",
                    cargo_jobs,
                    "--target",
                    target,
                ),
            )
        )
        if target != "wasm32-unknown-unknown":
            gates.append(
                Gate(
                    f"publisher-target-{target}",
                    f"Publisher compile check for {target}",
                    (
                        "cargo",
                        "check",
                        "--locked",
                        "-p",
                        "rspice-sheet-publisher",
                        "-j",
                        cargo_jobs,
                        "--target",
                        target,
                    ),
                )
            )
    return gates


def run_command(command: Sequence[str], root: Path, timeout_seconds: int) -> CommandOutcome:
    environment = os.environ.copy()
    environment.update(
        {
            "CARGO_INCREMENTAL": "0",
            "CARGO_TERM_COLOR": "never",
            "RUST_BACKTRACE": "1",
        }
    )
    try:
        completed = subprocess.run(
            tuple(command),
            cwd=root,
            check=False,
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            env=environment,
            timeout=timeout_seconds,
            shell=False,
        )
    except subprocess.TimeoutExpired as error:
        stdout = error.stdout.decode("utf-8", "replace") if isinstance(error.stdout, bytes) else error.stdout
        stderr = error.stderr.decode("utf-8", "replace") if isinstance(error.stderr, bytes) else error.stderr
        return CommandOutcome(None, stdout or "", stderr or "", True)
    except OSError as error:
        return CommandOutcome(None, "", str(error), False)
    return CommandOutcome(completed.returncode, completed.stdout, completed.stderr)


def observed_test_counts(output: str) -> dict[str, int]:
    counts = {"passed": 0, "failed": 0, "ignored": 0, "measured": 0, "filtered": 0}
    for match in TEST_RESULT.finditer(output):
        for key, value in zip(counts, match.groups(), strict=True):
            counts[key] += int(value)
    return counts


def source_line_budget(
    root: Path, scopes: Sequence[str] = SHEET_SOURCE_SCOPES
) -> tuple[bool, dict[str, object], str]:
    files, missing = collect_files(root, scopes)
    entries = []
    violations = []
    for path in files:
        line_count = len(path.read_text(encoding="utf-8").splitlines())
        relative = path.relative_to(root).as_posix()
        entries.append({"path": relative, "lines": line_count})
        if line_count > MAX_SOURCE_LINES:
            violations.append({"path": relative, "lines": line_count})
    passed = bool(files) and not missing and not violations
    detail = {
        "maximum_lines": MAX_SOURCE_LINES,
        "source_file_count": len(files),
        "missing_scopes": missing,
        "violations": violations,
        "maximum_observed_lines": max((entry["lines"] for entry in entries), default=0),
    }
    log = canonical_json({"summary": detail, "files": entries}).decode("utf-8")
    return passed, detail, log


def create_only(path: Path, content: bytes) -> None:
    if path.exists() or path.is_symlink():
        raise QualificationError(f"refusing to replace existing evidence path: {path}")
    with path.open("xb") as handle:
        handle.write(content)
        handle.flush()
        os.fsync(handle.fileno())


def evidence_paths(output: Path) -> tuple[Path, Path, Path]:
    if output.suffix.lower() != ".json":
        raise QualificationError("--out must name a .json evidence record")
    checksum = Path(f"{output}.sha256")
    logs = output.with_suffix(".logs")
    candidates = (output, checksum, logs)
    if len({str(path.resolve(strict=False)).casefold() for path in candidates}) != 3:
        raise QualificationError("evidence output, checksum, and log paths must be distinct")
    for path in candidates:
        if path.exists() or path.is_symlink():
            raise QualificationError(f"refusing to replace existing evidence path: {path}")
    return output, checksum, logs


def make_log(
    *,
    gate_id: str,
    label: str,
    command: Sequence[str] | None,
    outcome: CommandOutcome | None,
    internal_output: str = "",
) -> bytes:
    sections = [f"gate: {gate_id}", f"label: {label}"]
    if command is not None:
        sections.append(f"command: {shlex.join(command)}")
    if outcome is not None:
        sections.extend(
            [
                f"returncode: {outcome.returncode}",
                f"timed_out: {str(outcome.timed_out).lower()}",
                "",
                "[stdout]",
                outcome.stdout,
                "[stderr]",
                outcome.stderr,
            ]
        )
    elif internal_output:
        sections.extend(["", "[result]", internal_output])
    return ("\n".join(sections).rstrip() + "\n").encode("utf-8")


def write_gate_log(logs: Path, index: int, gate_id: str, content: bytes) -> dict[str, object]:
    name = f"{index:02d}-{gate_id}.log"
    path = logs / name
    create_only(path, content)
    return {"path": f"{logs.name}/{name}", "bytes": len(content), "sha256": sha256_bytes(content)}


def command_version(command: Sequence[str], root: Path) -> str:
    outcome = run_command(command, root, 30)
    if outcome.returncode != 0:
        detail = outcome.stderr.strip() or outcome.stdout.strip()
        raise QualificationError(f"could not identify {' '.join(command)}: {detail}")
    return outcome.stdout.strip()


def environment_record(root: Path) -> dict[str, object]:
    return {
        "operating_system": platform.system(),
        "os_release": platform.release(),
        "os_version": platform.version(),
        "architecture": normalized_architecture(platform.machine()),
        "machine": platform.machine(),
        "python": sys.version,
        "rustc": command_version(("rustc", "-Vv"), root),
        "cargo": command_version(("cargo", "-V"), root),
    }


def source_record(state: SourceState) -> dict[str, object]:
    status_bytes = state.status_porcelain.encode("utf-8")
    return {
        "commit": state.commit,
        "branch": state.branch,
        "clean": not state.dirty,
        "status_porcelain": state.status_porcelain.splitlines(),
        "status_sha256": sha256_bytes(status_bytes),
        "qualification_input_sha256": state.input_digest,
        "qualification_input_file_count": state.input_file_count,
        "worktree_sha256": state.worktree_digest,
        "worktree_untracked_file_count": state.worktree_file_count,
    }


def run_qualification(
    *,
    root: Path,
    output: Path,
    jobs: int,
    targets: Sequence[str],
    allow_dirty: bool,
    fail_fast: bool,
    timeout_seconds: int,
    executor: CommandExecutor = run_command,
    initial_source: SourceState | None = None,
    final_source: SourceState | None = None,
    environment: dict[str, object] | None = None,
    sheet_scopes: Sequence[str] = SHEET_SOURCE_SCOPES,
) -> tuple[int, dict[str, object], Path, Path]:
    if jobs < 1:
        raise QualificationError("--jobs must be a positive integer")
    if timeout_seconds < 1:
        raise QualificationError("--timeout-seconds must be a positive integer")
    checked_targets = validate_targets(targets)
    output, checksum, logs = evidence_paths(output)
    source_before = initial_source or inspect_source(root, sheet_scopes)
    if source_before.dirty and not allow_dirty:
        raise QualificationError(
            "the worktree is dirty; release evidence requires a clean checkout "
            "(use --allow-dirty only for non-release development evidence)"
        )
    host = environment or environment_record(root)
    incompatible = [
        target
        for target in checked_targets
        if not target_matches_host(
            target, str(host["operating_system"]), str(host["architecture"])
        )
    ]
    if incompatible:
        raise QualificationError(
            "native targets must be qualified on matching native hosts: " + ", ".join(incompatible)
        )
    bound_inputs = {
        "procedure": {
            "path": PROCEDURE,
            "sha256": sha256_file(root / PROCEDURE),
        },
        "cargo_lock": {
            "path": "Cargo.lock",
            "sha256": sha256_file(root / "Cargo.lock"),
        },
        "runner": {
            "path": "tools/ci/qualify_drawing_sheet.py",
            "sha256": sha256_file(root / "tools/ci/qualify_drawing_sheet.py"),
        },
    }

    output.parent.mkdir(parents=True, exist_ok=True)
    output, checksum, logs = evidence_paths(output)
    logs.mkdir(exist_ok=False)
    started_at = utc_now()
    started_clock = time.monotonic()
    results: list[dict[str, object]] = []
    failed = False
    index = 1

    try:
        line_passed, line_detail, line_output = source_line_budget(root, sheet_scopes)
    except (OSError, UnicodeError) as error:
        line_passed = False
        line_detail = {"error": f"{type(error).__name__}: {error}"}
        line_output = canonical_json(line_detail).decode("utf-8")
    line_log = make_log(
        gate_id="sheet-source-line-budget",
        label="Drawing-sheet and hardcopy source line budget",
        command=None,
        outcome=None,
        internal_output=line_output,
    )
    results.append(
        {
            "id": "sheet-source-line-budget",
            "label": "Drawing-sheet and hardcopy source line budget",
            "status": "passed" if line_passed else "failed",
            "detail": line_detail,
            "log": write_gate_log(logs, index, "sheet-source-line-budget", line_log),
        }
    )
    index += 1
    failed = not line_passed

    gates = command_gates(jobs, checked_targets, str(host["operating_system"]))
    for gate in gates:
        if failed and fail_fast:
            results.append(
                {
                    "id": gate.gate_id,
                    "label": gate.label,
                    "status": "not-run",
                    "reason": "fail-fast",
                    "command": list(gate.command),
                    "minimum_passed_tests": gate.minimum_passed_tests,
                }
            )
            continue
        gate_started = time.monotonic()
        try:
            outcome = executor(gate.command, root, timeout_seconds)
        except Exception as error:  # Preserve evidence even for an unexpected executor failure.
            outcome = CommandOutcome(None, "", f"executor raised {type(error).__name__}: {error}")
        counts = observed_test_counts(outcome.stdout + "\n" + outcome.stderr)
        enough_tests = (
            gate.minimum_passed_tests is None
            or counts["passed"] >= gate.minimum_passed_tests
        )
        no_ignored_tests = gate.minimum_passed_tests is None or counts["ignored"] == 0
        passed = (
            outcome.returncode == 0
            and not outcome.timed_out
            and enough_tests
            and no_ignored_tests
        )
        log_content = make_log(
            gate_id=gate.gate_id,
            label=gate.label,
            command=gate.command,
            outcome=outcome,
        )
        result: dict[str, object] = {
            "id": gate.gate_id,
            "label": gate.label,
            "status": "passed" if passed else "failed",
            "command": list(gate.command),
            "returncode": outcome.returncode,
            "timed_out": outcome.timed_out,
            "duration_ms": round((time.monotonic() - gate_started) * 1_000),
            "stdout_bytes": len(outcome.stdout.encode("utf-8")),
            "stderr_bytes": len(outcome.stderr.encode("utf-8")),
            "log": write_gate_log(logs, index, gate.gate_id, log_content),
        }
        if gate.minimum_passed_tests is not None:
            result["minimum_passed_tests"] = gate.minimum_passed_tests
            result["observed_test_counts"] = counts
            if outcome.returncode == 0 and not enough_tests:
                result["failure_reason"] = "test filter executed fewer tests than its frozen minimum"
            elif outcome.returncode == 0 and not no_ignored_tests:
                result["failure_reason"] = "one or more in-scope tests were ignored"
        results.append(result)
        index += 1
        failed = failed or not passed

    try:
        source_after = final_source or inspect_source(root, sheet_scopes)
        stable = source_before == source_after
        stability_detail = {
            "initial_commit": source_before.commit,
            "final_commit": source_after.commit,
            "initial_branch": source_before.branch,
            "final_branch": source_after.branch,
            "initial_status_sha256": sha256_bytes(
                source_before.status_porcelain.encode("utf-8")
            ),
            "final_status_sha256": sha256_bytes(source_after.status_porcelain.encode("utf-8")),
            "initial_input_sha256": source_before.input_digest,
            "final_input_sha256": source_after.input_digest,
            "initial_worktree_sha256": source_before.worktree_digest,
            "final_worktree_sha256": source_after.worktree_digest,
        }
    except (QualificationError, OSError, UnicodeError) as error:
        stable = False
        stability_detail = {
            "initial_commit": source_before.commit,
            "initial_branch": source_before.branch,
            "initial_status_sha256": sha256_bytes(
                source_before.status_porcelain.encode("utf-8")
            ),
            "initial_input_sha256": source_before.input_digest,
            "initial_worktree_sha256": source_before.worktree_digest,
            "inspection_error": f"{type(error).__name__}: {error}",
        }
    stability_log = make_log(
        gate_id="source-stability",
        label="Qualification source remained unchanged during execution",
        command=None,
        outcome=None,
        internal_output=canonical_json(stability_detail).decode("utf-8"),
    )
    results.append(
        {
            "id": "source-stability",
            "label": "Qualification source remained unchanged during execution",
            "status": "passed" if stable else "failed",
            "detail": stability_detail,
            "log": write_gate_log(logs, index, "source-stability", stability_log),
        }
    )
    failed = failed or not stable

    status = "failed" if failed else "development-pass" if source_before.dirty else "automated-pass"
    record = {
        "schema_version": SCHEMA_VERSION,
        "qualification": "rspice-drawing-sheet-automated",
        "status": status,
        "release_eligible_automated_record": status == "automated-pass",
        "manual_evidence_required": [
            "native desktop interaction on every supported OS",
            "browser and operating-system print-dialog inspection",
            "physical printer, driver, spool, scale, media, and duplex matrix",
            "artifact inspection with approved external validators",
            "NVDA or JAWS, VoiceOver, and Orca accessibility qualification",
            "independent release-authority signoff",
        ],
        "started_at": started_at,
        "finished_at": utc_now(),
        "duration_ms": round((time.monotonic() - started_clock) * 1_000),
        "source": source_record(source_before),
        "environment": host,
        "targets": list(checked_targets),
        "configuration": {
            "jobs": jobs,
            "timeout_seconds_per_command": timeout_seconds,
            "fail_fast": fail_fast,
            "allow_dirty": allow_dirty,
            "locked_dependencies": True,
            "maximum_source_lines": MAX_SOURCE_LINES,
        },
        "inputs": bound_inputs,
        "gates": results,
        "summary": {
            "passed": sum(result["status"] == "passed" for result in results),
            "failed": sum(result["status"] == "failed" for result in results),
            "not_run": sum(result["status"] == "not-run" for result in results),
        },
    }
    record_bytes = canonical_json(record)
    create_only(output, record_bytes)
    digest = sha256_bytes(record_bytes)
    create_only(checksum, f"{digest}  {output.name}\n".encode("ascii"))
    return (0 if not failed else 1), record, output, checksum


def parser() -> argparse.ArgumentParser:
    argument_parser = argparse.ArgumentParser(
        description="Create an immutable automated drawing-sheet qualification record."
    )
    argument_parser.add_argument("--out", type=Path, help="new .json evidence path")
    argument_parser.add_argument(
        "--target",
        action="append",
        default=[],
        help="supported native-host or wasm32 target (repeatable)",
    )
    argument_parser.add_argument("--jobs", type=int, default=1)
    argument_parser.add_argument("--timeout-seconds", type=int, default=3_600)
    argument_parser.add_argument("--fail-fast", action="store_true")
    argument_parser.add_argument(
        "--allow-dirty",
        action="store_true",
        help="permit development evidence; the result can never be release eligible",
    )
    argument_parser.add_argument(
        "--dry-run", action="store_true", help="validate and print gates without writing or executing"
    )
    return argument_parser


def main(arguments: Sequence[str] | None = None) -> int:
    args = parser().parse_args(arguments)
    try:
        targets = validate_targets(args.target)
        if args.jobs < 1:
            raise QualificationError("--jobs must be a positive integer")
        if args.timeout_seconds < 1:
            raise QualificationError("--timeout-seconds must be a positive integer")
        if args.dry_run:
            plan = {
                "qualification": "rspice-drawing-sheet-automated",
                "dry_run": True,
                "targets": list(targets),
                "host": {
                    "operating_system": platform.system(),
                    "architecture": normalized_architecture(platform.machine()),
                },
                "source_line_budget": {
                    "maximum_lines": MAX_SOURCE_LINES,
                    "scopes": list(SHEET_SOURCE_SCOPES),
                },
                "gates": [
                    {
                        "id": gate.gate_id,
                        "command": list(gate.command),
                        "minimum_passed_tests": gate.minimum_passed_tests,
                    }
                    for gate in command_gates(args.jobs, targets)
                ],
            }
            print(json.dumps(plan, indent=2, sort_keys=True))
            return 0
        if args.out is None:
            raise QualificationError("--out is required unless --dry-run is used")
        exit_code, record, output, checksum = run_qualification(
            root=ROOT,
            output=args.out.resolve(),
            jobs=args.jobs,
            targets=targets,
            allow_dirty=args.allow_dirty,
            fail_fast=args.fail_fast,
            timeout_seconds=args.timeout_seconds,
        )
        print(f"{record['status']}: {output}")
        print(f"sha256: {checksum}")
        return exit_code
    except QualificationError as error:
        print(f"drawing-sheet qualification refused: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
