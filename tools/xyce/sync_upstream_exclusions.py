#!/usr/bin/env python3
"""Reproduce RSpice's retained Xyce upstream-exclusion manifest.

The exclusion inventory comes from the exact Git tree immediately before the
vendored Xyce harness files were trimmed. Promotion contracts come from a
clean-HEAD classification report, or from the checked-in manifest when the
tool is used in audit mode without ``--classification-report``.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import os
import re
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path


SOURCE_COMMIT = "80115a9277c0ddb3409acceb3d4e745fd11cddd4"
SOURCE_NETLISTS_TREE = "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d"
QUALIFICATION_COMMIT = "855c573762329b184ba566b63305a0f6e98f79b6"
QUALIFICATION_REPORT_SHA256 = (
    "74895b402cb215e0cb05be30b5f99bf69bbfb672ea83bb6f2b201dde709bf8b1"
)
RETAINED_RECORD_COUNT = 1_143
QUALIFIED_RECORD_COUNT = 210
TOTAL_RETAINED_CIRCUIT_COUNT = 4_055
MANIFEST_RELATIVE_PATH = Path("tests/xyce/RSPICE-UPSTREAM-EXCLUSIONS.tsv")
CONTRACT_RE = re.compile(r"^[a-z0-9_]+$")


@dataclass(frozen=True)
class Exclusion:
    deck: str
    source: str


def run_git(repo: Path, *args: str) -> str:
    process = subprocess.run(
        ["git", *args],
        cwd=repo,
        check=False,
        capture_output=True,
        text=True,
        encoding="utf-8",
    )
    if process.returncode != 0:
        raise RuntimeError(
            f"git {' '.join(args)} failed with exit code {process.returncode}: "
            f"{process.stderr.strip()}"
        )
    return process.stdout


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def current_decks(repo: Path) -> dict[str, str]:
    corpus = repo / "tests/xyce"
    decks: dict[str, str] = {}
    for path in sorted((corpus / "Netlists").rglob("*")):
        if path.suffix.casefold() != ".cir":
            continue
        if path.is_symlink() or not path.is_file():
            raise RuntimeError(f"retained deck must be a regular non-symlink file: {path}")
        relative = path.relative_to(corpus).as_posix()
        key = relative.casefold()
        previous = decks.setdefault(key, relative)
        if previous != relative:
            raise RuntimeError(
                f"case-insensitive retained-deck collision: {previous!r} and {relative!r}"
            )
    if len(decks) != TOTAL_RETAINED_CIRCUIT_COUNT:
        raise RuntimeError(
            f"discovered {len(decks)} retained circuits; "
            f"expected {TOTAL_RETAINED_CIRCUIT_COUNT}"
        )
    return decks


def ensure_source_commit(repo: Path) -> None:
    """Make the pinned pre-trim commit available in shallow checkouts.

    CI checks the repository out at depth 1, which drops the historical
    commit this inventory is reproduced from. Fetching the pinned hash
    directly stays exact: every digest check downstream still fails closed
    if the fetched history does not match.
    """
    probe = subprocess.run(
        ["git", "cat-file", "-e", f"{SOURCE_COMMIT}^{{commit}}"],
        cwd=repo,
        check=False,
        capture_output=True,
    )
    if probe.returncode == 0:
        return
    fetch = subprocess.run(
        ["git", "fetch", "--depth=1", "origin", SOURCE_COMMIT],
        cwd=repo,
        check=False,
        capture_output=True,
        text=True,
        encoding="utf-8",
    )
    if fetch.returncode != 0:
        raise RuntimeError(
            f"source commit {SOURCE_COMMIT} is absent from this clone and "
            f"fetching it from origin failed with exit code {fetch.returncode}: "
            f"{fetch.stderr.strip()}"
        )


def recover_exclusions(repo: Path) -> list[Exclusion]:
    ensure_source_commit(repo)
    actual_tree = run_git(repo, "rev-parse", f"{SOURCE_COMMIT}:tests/xyce/Netlists").strip()
    if actual_tree != SOURCE_NETLISTS_TREE:
        raise RuntimeError(
            f"source Netlists tree is {actual_tree}, expected {SOURCE_NETLISTS_TREE}"
        )
    retained = current_decks(repo)
    paths = run_git(
        repo,
        "ls-tree",
        "-r",
        "--name-only",
        SOURCE_COMMIT,
        "--",
        "tests/xyce/Netlists",
    ).splitlines()
    exclude_files = [path for path in paths if path.endswith("/exclude")]
    recovered: dict[str, Exclusion] = {}
    for source_path in exclude_files:
        content = run_git(repo, "show", f"{SOURCE_COMMIT}:{source_path}")
        source = source_path.removeprefix("tests/xyce/")
        directory = source.rsplit("/", 1)[0]
        for raw_line in content.splitlines():
            entry = re.sub(r"\s+", "", raw_line.split("#", 1)[0])
            if not entry:
                continue
            candidate = f"{directory}/{entry}"
            if not candidate.casefold().endswith(".cir"):
                continue
            deck = retained.get(candidate.casefold())
            if deck is None:
                continue
            exclusion = Exclusion(deck=deck, source=source)
            previous = recovered.setdefault(deck.casefold(), exclusion)
            if previous != exclusion:
                raise RuntimeError(
                    f"conflicting exclusion provenance for {deck}: "
                    f"{previous.source} and {source}"
                )
    rows = sorted(recovered.values(), key=lambda row: row.deck)
    if len(rows) != RETAINED_RECORD_COUNT:
        raise RuntimeError(
            f"recovered {len(rows)} retained exclusions; expected {RETAINED_RECORD_COUNT}"
        )
    return rows


def parse_existing_promotions(path: Path) -> dict[str, str]:
    promotions: dict[str, str] = {}
    if not path.is_file():
        return promotions
    for raw_line in path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        fields = [field.strip() for field in line.split("\t")]
        if len(fields) == 4 and fields[2] == "rspice_independently_qualified":
            promotions[fields[0].casefold()] = fields[3]
    return promotions


def parse_classification_report(
    path: Path, exclusions: list[Exclusion]
) -> dict[str, str]:
    actual_digest = sha256(path)
    if actual_digest != QUALIFICATION_REPORT_SHA256:
        raise RuntimeError(
            f"classification report SHA-256 is {actual_digest}, "
            f"expected {QUALIFICATION_REPORT_SHA256}"
        )
    with path.open("r", encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream, delimiter="\t")
        required = {
            "relative_path",
            "passed",
            "expected_unsupported",
            "contract",
            "process_status",
        }
        missing = required.difference(reader.fieldnames or ())
        if missing:
            raise RuntimeError(
                f"classification report is missing columns: {', '.join(sorted(missing))}"
            )
        classified: dict[str, tuple[bool, bool, str]] = {}
        for row in reader:
            deck = row["relative_path"]
            key = deck.casefold()
            if key in classified:
                raise RuntimeError(f"classification report duplicates {deck}")
            if row["process_status"] != "completed":
                raise RuntimeError(
                    f"classification for {deck} has process status {row['process_status']!r}"
                )
            try:
                passed = {"true": True, "false": False}[row["passed"]]
                unsupported = {
                    "true": True,
                    "false": False,
                }[row["expected_unsupported"]]
            except KeyError as error:
                raise RuntimeError(
                    f"classification for {deck} has invalid boolean {error.args[0]!r}"
                ) from error
            classified[key] = (passed, unsupported, row["contract"])

    expected = {row.deck.casefold() for row in exclusions}
    actual = set(classified)
    if expected != actual:
        raise RuntimeError(
            "classification inventory differs from recovered exclusions: "
            f"missing={len(expected - actual)}, extra={len(actual - expected)}"
        )
    promotions = {
        key: contract
        for key, (passed, unsupported, contract) in classified.items()
        if passed and not unsupported
    }
    if len(promotions) != QUALIFIED_RECORD_COUNT:
        raise RuntimeError(
            f"classification produced {len(promotions)} promotions; "
            f"expected {QUALIFIED_RECORD_COUNT}"
        )
    return promotions


def render_manifest(exclusions: list[Exclusion], promotions: dict[str, str]) -> str:
    unknown = set(promotions).difference(row.deck.casefold() for row in exclusions)
    if unknown:
        raise RuntimeError(f"manifest contains {len(unknown)} promotions for unknown decks")
    if len(promotions) != QUALIFIED_RECORD_COUNT:
        raise RuntimeError(
            f"manifest contains {len(promotions)} promotions; expected {QUALIFIED_RECORD_COUNT}"
        )
    lines = [
        "# RSpice Xyce upstream-exclusion provenance and qualification manifest",
        "# Reproduce with tools/xyce/sync_upstream_exclusions.py.",
        f"# qualification_commit={QUALIFICATION_COMMIT}",
        f"# qualification_report_sha256={QUALIFICATION_REPORT_SHA256}",
        "# Fields: deck, trimmed upstream exclude file, disposition, optional exact contract.",
        "schema_version\t1",
        f"source_commit\t{SOURCE_COMMIT}",
        f"source_netlists_tree\t{SOURCE_NETLISTS_TREE}",
    ]
    for exclusion in exclusions:
        contract = promotions.get(exclusion.deck.casefold())
        if contract is None:
            lines.append(
                f"{exclusion.deck}\t{exclusion.source}\tupstream_excluded"
            )
            continue
        if not CONTRACT_RE.fullmatch(contract):
            raise RuntimeError(
                f"promotion for {exclusion.deck} has invalid contract {contract!r}"
            )
        lines.append(
            f"{exclusion.deck}\t{exclusion.source}\t"
            f"rspice_independently_qualified\t{contract}"
        )
    return "\n".join(lines) + "\n"


def atomic_write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(
        dir=path.parent, prefix=f".{path.name}.", suffix=".tmp"
    )
    temporary = Path(temporary_name)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8", newline="\n") as stream:
            stream.write(content)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, path)
    finally:
        temporary.unlink(missing_ok=True)


def verify_manifest_bytes(path: Path, expected: str) -> None:
    if path.read_bytes() != expected.encode("utf-8"):
        raise RuntimeError(f"{path} is not byte-for-byte reproducible")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=Path(__file__).resolve().parents[2],
        help="RSpice repository root",
    )
    parser.add_argument(
        "--classification-report",
        type=Path,
        help="clean-HEAD census TSV used to refresh exact promotion contracts",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="verify the checked-in manifest without modifying it",
    )
    args = parser.parse_args()

    repo = args.repo_root.resolve()
    manifest_path = repo / MANIFEST_RELATIVE_PATH
    exclusions = recover_exclusions(repo)
    promotions = (
        parse_classification_report(args.classification_report.resolve(), exclusions)
        if args.classification_report
        else parse_existing_promotions(manifest_path)
    )
    expected = render_manifest(exclusions, promotions)
    if args.check:
        verify_manifest_bytes(manifest_path, expected)
        print(
            f"verified {len(exclusions)} exclusions with {len(promotions)} promotions"
        )
        return 0
    atomic_write(manifest_path, expected)
    print(f"wrote {len(exclusions)} exclusions with {len(promotions)} promotions")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
