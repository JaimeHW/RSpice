import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


sys.path.insert(0, str(Path(__file__).resolve().parent))
from qualify_drawing_sheet import (  # noqa: E402
    CommandOutcome,
    QualificationError,
    SourceState,
    SUPPORTED_TARGETS,
    command_gates,
    create_only,
    evidence_paths,
    observed_test_counts,
    run_qualification,
    source_line_budget,
    target_matches_host,
    validate_targets,
    worktree_fingerprint,
)


COMMIT = "0123456789abcdef" * 2 + "01234567"


class DrawingSheetQualificationTests(unittest.TestCase):
    def test_real_git_fingerprint_detects_tracked_and_untracked_byte_changes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)

            def git(*arguments: str) -> None:
                subprocess.run(
                    ("git", *arguments),
                    cwd=root,
                    check=True,
                    capture_output=True,
                    text=True,
                    encoding="utf-8",
                )

            git("init", "--initial-branch=main")
            tracked = root / "tracked.txt"
            tracked.write_text("first\n", encoding="utf-8")
            git("add", "tracked.txt")
            git(
                "-c",
                "user.name=Qualification Test",
                "-c",
                "user.email=qualification@example.invalid",
                "commit",
                "-m",
                "fixture",
            )

            clean, clean_untracked = worktree_fingerprint(root)
            self.assertEqual(clean_untracked, 0)
            tracked.write_text("second\n", encoding="utf-8")
            tracked_changed, _ = worktree_fingerprint(root)
            self.assertNotEqual(clean, tracked_changed)

            untracked = root / "new.txt"
            untracked.write_text("one\n", encoding="utf-8")
            untracked_first, untracked_count = worktree_fingerprint(root)
            self.assertEqual(untracked_count, 1)
            untracked.write_text("two\n", encoding="utf-8")
            untracked_second, _ = worktree_fingerprint(root)
            self.assertNotEqual(untracked_first, untracked_second)

            excluded = root / ".codex-candidates" / "other" / "source.rs"
            excluded.parent.mkdir(parents=True)
            excluded.write_text("first\n", encoding="utf-8")
            excluded_first, excluded_count = worktree_fingerprint(root)
            excluded.write_text("second\n", encoding="utf-8")
            excluded_second, repeated_count = worktree_fingerprint(root)
            self.assertEqual(excluded_count, repeated_count)
            self.assertEqual(excluded_first, excluded_second)

    def test_workflow_covers_every_desktop_architecture_and_wasm_without_dirty_override(self) -> None:
        root = Path(__file__).resolve().parents[2]
        workflow = (root / ".github/workflows/drawing-sheet-qualification.yml").read_text(
            encoding="utf-8"
        )
        for target in SUPPORTED_TARGETS:
            self.assertIn(f"target: {target}", workflow)
        self.assertIn("workflow_dispatch:", workflow)
        self.assertRegex(workflow, r"(?m)^permissions:\n  contents: read$")
        self.assertNotIn("--allow-dirty", workflow)
        for action in re.findall(r"uses:\s*([^#\s]+)", workflow):
            reference = action.rsplit("@", 1)[-1]
            self.assertRegex(reference, r"^[0-9a-f]{40}$")

    def test_gate_ids_are_unique_locked_and_test_filters_are_nonvacuous(self) -> None:
        gates = command_gates(2, ("x86_64-unknown-linux-gnu", "wasm32-unknown-unknown"))
        self.assertEqual(len(gates), len({gate.gate_id for gate in gates}))
        for gate in gates:
            self.assertEqual(gate.command[0], "cargo")
            self.assertIn("--locked", gate.command)
            self.assertNotIn("shell", gate.command)
            if "test" in gate.command:
                self.assertIsNotNone(gate.minimum_passed_tests)
                self.assertGreater(gate.minimum_passed_tests, 0)

    def test_targets_are_an_allowlist_and_duplicates_are_rejected(self) -> None:
        self.assertEqual(
            validate_targets(("wasm32-unknown-unknown",)),
            ("wasm32-unknown-unknown",),
        )
        with self.assertRaisesRegex(QualificationError, "at least one"):
            validate_targets(())
        with self.assertRaisesRegex(QualificationError, "only once"):
            validate_targets(("wasm32-unknown-unknown", "wasm32-unknown-unknown"))
        with self.assertRaisesRegex(QualificationError, "unsupported"):
            validate_targets(("x86_64-unknown-linux-gnu; Remove-Item *",))

    def test_native_targets_require_the_matching_operating_system_and_architecture(self) -> None:
        self.assertTrue(target_matches_host("x86_64-pc-windows-msvc", "Windows", "AMD64"))
        self.assertTrue(target_matches_host("aarch64-apple-darwin", "Darwin", "arm64"))
        self.assertFalse(target_matches_host("x86_64-apple-darwin", "Windows", "AMD64"))
        self.assertFalse(target_matches_host("aarch64-unknown-linux-gnu", "Linux", "x86_64"))
        self.assertTrue(target_matches_host("wasm32-unknown-unknown", "Windows", "AMD64"))

    def test_rust_test_summary_is_counted_across_test_targets(self) -> None:
        output = (
            "test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\n"
            "test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out\n"
        )
        self.assertEqual(
            observed_test_counts(output),
            {"passed": 9, "failed": 0, "ignored": 1, "measured": 0, "filtered": 0},
        )

    def test_line_budget_detects_missing_and_oversized_sources(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "scope" / "unit.rs"
            source.parent.mkdir()
            source.write_text("fn small() {}\n", encoding="utf-8")
            passed, detail, _ = source_line_budget(root, ("scope",))
            self.assertTrue(passed)
            self.assertEqual(detail["source_file_count"], 1)

            source.write_text("\n".join("// line" for _ in range(2_501)), encoding="utf-8")
            passed, detail, _ = source_line_budget(root, ("scope",))
            self.assertFalse(passed)
            self.assertEqual(detail["violations"][0]["lines"], 2_501)

            passed, detail, _ = source_line_budget(root, ("missing",))
            self.assertFalse(passed)
            self.assertEqual(detail["missing_scopes"], ["missing"])

    def test_output_paths_and_writes_are_create_only_and_symlink_safe(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            output = root / "record.json"
            evidence_paths(output)
            create_only(output, b"first")
            with self.assertRaisesRegex(QualificationError, "replace existing"):
                create_only(output, b"second")

            if hasattr(os, "symlink"):
                target = root / "target.json"
                target.write_bytes(b"target")
                link = root / "linked.json"
                try:
                    link.symlink_to(target)
                except OSError:
                    return
                with self.assertRaisesRegex(QualificationError, "replace existing"):
                    evidence_paths(link)

    def _fixture_root(self, root: Path) -> tuple[str, ...]:
        (root / "scope").mkdir(parents=True)
        (root / "scope" / "sheet.rs").write_text("fn sheet() {}\n", encoding="utf-8")
        for relative, content in (
            ("Cargo.lock", "# lock\n"),
            ("security/DRAWING_SHEET_RELEASE_QUALIFICATION.md", "# procedure\n"),
            ("tools/ci/qualify_drawing_sheet.py", "# runner\n"),
        ):
            path = root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content, encoding="utf-8")
        return ("scope",)

    def _source(self, *, dirty: bool = False) -> SourceState:
        return SourceState(
            commit=COMMIT,
            branch="test",
            status_porcelain=" M fixture" if dirty else "",
            input_digest="a" * 64,
            input_file_count=1,
        )

    def _environment(self) -> dict[str, object]:
        return {
            "operating_system": "Windows",
            "architecture": "x86_64",
            "machine": "AMD64",
            "python": "test",
            "rustc": "rustc test",
            "cargo": "cargo test",
        }

    def _executor(self, command, _root, _timeout) -> CommandOutcome:
        minimum = next(
            (
                gate.minimum_passed_tests
                for gate in command_gates(1, ("wasm32-unknown-unknown",))
                if gate.command == tuple(command)
            ),
            None,
        )
        stdout = ""
        if minimum is not None:
            stdout = (
                f"test result: ok. {minimum} passed; 0 failed; 0 ignored; "
                "0 measured; 0 filtered out\n"
            )
        return CommandOutcome(0, stdout, "")

    def test_clean_all_pass_record_is_checksumbound(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            scopes = self._fixture_root(root)
            output = root / "evidence" / "record.json"
            exit_code, record, written, checksum = run_qualification(
                root=root,
                output=output,
                jobs=1,
                targets=("wasm32-unknown-unknown",),
                allow_dirty=False,
                fail_fast=False,
                timeout_seconds=10,
                executor=self._executor,
                initial_source=self._source(),
                final_source=self._source(),
                environment=self._environment(),
                sheet_scopes=scopes,
            )
            self.assertEqual(exit_code, 0)
            self.assertEqual(record["status"], "automated-pass")
            self.assertTrue(record["release_eligible_automated_record"])
            expected, name = checksum.read_text(encoding="ascii").strip().split("  ")
            self.assertEqual(name, written.name)
            self.assertEqual(expected, hashlib.sha256(written.read_bytes()).hexdigest())
            self.assertTrue(written.with_suffix(".logs").is_dir())

    def test_dirty_allowed_record_is_never_release_eligible(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            scopes = self._fixture_root(root)
            _, record, _, _ = run_qualification(
                root=root,
                output=root / "record.json",
                jobs=1,
                targets=("wasm32-unknown-unknown",),
                allow_dirty=True,
                fail_fast=False,
                timeout_seconds=10,
                executor=self._executor,
                initial_source=self._source(dirty=True),
                final_source=self._source(dirty=True),
                environment=self._environment(),
                sheet_scopes=scopes,
            )
            self.assertEqual(record["status"], "development-pass")
            self.assertFalse(record["release_eligible_automated_record"])

    def test_dirty_release_attempt_is_refused_before_any_output_is_created(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            scopes = self._fixture_root(root)
            output = root / "evidence" / "record.json"
            with self.assertRaisesRegex(QualificationError, "worktree is dirty"):
                run_qualification(
                    root=root,
                    output=output,
                    jobs=1,
                    targets=("wasm32-unknown-unknown",),
                    allow_dirty=False,
                    fail_fast=False,
                    timeout_seconds=10,
                    executor=self._executor,
                    initial_source=self._source(dirty=True),
                    final_source=self._source(dirty=True),
                    environment=self._environment(),
                    sheet_scopes=scopes,
                )
            self.assertFalse(output.parent.exists())

    def test_source_drift_during_execution_seals_a_failed_record(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            scopes = self._fixture_root(root)
            final_source = SourceState(
                commit=COMMIT,
                branch="test",
                status_porcelain=" M changed-during-run",
                input_digest="b" * 64,
                input_file_count=1,
            )
            exit_code, record, written, checksum = run_qualification(
                root=root,
                output=root / "record.json",
                jobs=1,
                targets=("wasm32-unknown-unknown",),
                allow_dirty=False,
                fail_fast=False,
                timeout_seconds=10,
                executor=self._executor,
                initial_source=self._source(),
                final_source=final_source,
                environment=self._environment(),
                sheet_scopes=scopes,
            )
            self.assertEqual(exit_code, 1)
            self.assertEqual(record["status"], "failed")
            stability = next(item for item in record["gates"] if item["id"] == "source-stability")
            self.assertEqual(stability["status"], "failed")
            self.assertTrue(written.is_file())
            self.assertTrue(checksum.is_file())

    def test_dirty_file_byte_drift_is_detected_even_when_git_status_is_unchanged(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            scopes = self._fixture_root(root)
            initial = self._source(dirty=True)
            final = SourceState(
                commit=initial.commit,
                branch=initial.branch,
                status_porcelain=initial.status_porcelain,
                input_digest=initial.input_digest,
                input_file_count=initial.input_file_count,
                worktree_digest="changed-worktree-bytes",
                worktree_file_count=initial.worktree_file_count,
            )
            exit_code, record, _, _ = run_qualification(
                root=root,
                output=root / "record.json",
                jobs=1,
                targets=("wasm32-unknown-unknown",),
                allow_dirty=True,
                fail_fast=False,
                timeout_seconds=10,
                executor=self._executor,
                initial_source=initial,
                final_source=final,
                environment=self._environment(),
                sheet_scopes=scopes,
            )
            self.assertEqual(exit_code, 1)
            stability = next(item for item in record["gates"] if item["id"] == "source-stability")
            self.assertEqual(stability["status"], "failed")
            self.assertNotEqual(
                stability["detail"]["initial_worktree_sha256"],
                stability["detail"]["final_worktree_sha256"],
            )

    def test_command_failure_produces_failed_evidence_instead_of_false_pass(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            scopes = self._fixture_root(root)

            def failing_executor(command, command_root, timeout):
                if "rspice-ui" in command and "drawing_sheet" in command:
                    return CommandOutcome(1, "", "intentional failure")
                return self._executor(command, command_root, timeout)

            exit_code, record, _, _ = run_qualification(
                root=root,
                output=root / "record.json",
                jobs=1,
                targets=("wasm32-unknown-unknown",),
                allow_dirty=False,
                fail_fast=False,
                timeout_seconds=10,
                executor=failing_executor,
                initial_source=self._source(),
                final_source=self._source(),
                environment=self._environment(),
                sheet_scopes=scopes,
            )
            self.assertEqual(exit_code, 1)
            self.assertEqual(record["status"], "failed")
            drawing_sheet = next(gate for gate in record["gates"] if gate["id"] == "ui-drawing-sheet")
            self.assertEqual(drawing_sheet["status"], "failed")

    def test_too_few_filtered_tests_is_a_failure_even_with_zero_exit(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            scopes = self._fixture_root(root)

            def vacuous_executor(command, _root, _timeout):
                if "test" in command:
                    return CommandOutcome(
                        0,
                        "test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\n",
                        "",
                    )
                return CommandOutcome(0, "", "")

            exit_code, record, _, _ = run_qualification(
                root=root,
                output=root / "record.json",
                jobs=1,
                targets=("wasm32-unknown-unknown",),
                allow_dirty=False,
                fail_fast=True,
                timeout_seconds=10,
                executor=vacuous_executor,
                initial_source=self._source(),
                final_source=self._source(),
                environment=self._environment(),
                sheet_scopes=scopes,
            )
            self.assertEqual(exit_code, 1)
            self.assertEqual(record["summary"]["not_run"], len(command_gates(1, ("wasm32-unknown-unknown",))) - 1)

    def test_ignored_in_scope_test_is_not_a_pass(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            scopes = self._fixture_root(root)

            def ignored_executor(command, command_root, timeout):
                outcome = self._executor(command, command_root, timeout)
                if "rspice-ui" in command and "drawing_sheet" in command:
                    return CommandOutcome(
                        0,
                        outcome.stdout.replace(
                            "0 ignored;", "1 ignored;"
                        ),
                        "",
                    )
                return outcome

            exit_code, record, _, _ = run_qualification(
                root=root,
                output=root / "record.json",
                jobs=1,
                targets=("wasm32-unknown-unknown",),
                allow_dirty=False,
                fail_fast=False,
                timeout_seconds=10,
                executor=ignored_executor,
                initial_source=self._source(),
                final_source=self._source(),
                environment=self._environment(),
                sheet_scopes=scopes,
            )
            self.assertEqual(exit_code, 1)
            gate = next(item for item in record["gates"] if item["id"] == "ui-drawing-sheet")
            self.assertEqual(gate["failure_reason"], "one or more in-scope tests were ignored")


if __name__ == "__main__":
    unittest.main()
