"""Regression tests for the workspace formatting gate."""

import contextlib
import io
import subprocess
import sys
import unittest
from unittest import mock

import check_formatting
from check_formatting import (
    ARGUMENT_BUDGET,
    MINIMUM_MEMBERS,
    batches,
    entry_points,
    excluded,
    metadata,
    offenders,
    reportable,
)


def package(name: str, *targets: tuple[str, str]) -> dict:
    return {
        "name": name,
        "targets": [
            {"edition": edition, "src_path": src_path} for edition, src_path in targets
        ],
    }


class EntryPointTests(unittest.TestCase):
    def test_only_the_generated_model_crates_are_skipped(self) -> None:
        self.assertTrue(excluded("rspice-veriloga-models"))
        self.assertTrue(excluded("rspice-veriloga-model-bsim4"))
        for member in ("rspice-core", "rspice-app", "rspice-veriloga", "rspice-output"):
            self.assertFalse(excluded(member), member)

    def test_every_target_is_offered_and_a_shared_file_only_once(self) -> None:
        """A target's entry file is what rustfmt needs, not just the library.

        cargo fmt formats a package's tests, benches and build script as well
        as its library, so a gate built only from `src/lib.rs` would silently
        stop covering every other target.
        """
        grouped = entry_points(
            {
                "packages": [
                    package(
                        "rspice-app",
                        ("2024", "crates/rspice-app/src/lib.rs"),
                        ("2024", "crates/rspice-app/src/main.rs"),
                        ("2024", "crates/rspice-app/build.rs"),
                        ("2024", "crates/rspice-app/src/lib.rs"),
                    ),
                    package("rspice-veriloga-models", ("2024", "generated/lib.rs")),
                ]
            }
        )
        self.assertEqual(
            grouped,
            {
                ("rspice-app", "2024"): [
                    "crates/rspice-app/build.rs",
                    "crates/rspice-app/src/lib.rs",
                    "crates/rspice-app/src/main.rs",
                ]
            },
        )

    def test_a_member_with_two_editions_is_checked_under_each(self) -> None:
        grouped = entry_points(
            {
                "packages": [
                    package(
                        "rspice-core",
                        ("2024", "crates/rspice-core/src/lib.rs"),
                        ("2015", "crates/rspice-core/build.rs"),
                    )
                ]
            }
        )
        self.assertEqual(
            grouped,
            {
                ("rspice-core", "2015"): ["crates/rspice-core/build.rs"],
                ("rspice-core", "2024"): ["crates/rspice-core/src/lib.rs"],
            },
        )


class BatchTests(unittest.TestCase):
    def test_a_short_list_stays_one_invocation(self) -> None:
        paths = ["a.rs", "b.rs", "c.rs"]
        self.assertEqual(batches(paths), [paths])

    def test_no_run_exceeds_the_budget_and_no_path_is_lost(self) -> None:
        """Absolute paths are what rustfmt is given, and they are not short.

        cargo metadata reports `src_path` absolute, so the length that matters
        is the whole checkout path and not the repository-relative tail.
        """
        from check_formatting import ROOT

        paths = [
            str(ROOT / "crates" / "rspice-core" / "tests" / f"case_{index:04}.rs")
            for index in range(400)
        ]
        runs = batches(paths)
        self.assertGreater(len(runs), 1, "400 absolute target paths cannot fit one line")
        self.assertEqual([path for run in runs for path in run], paths)
        for run in runs:
            self.assertLessEqual(sum(len(path) + 3 for path in run), ARGUMENT_BUDGET)

    def test_one_path_over_the_budget_still_goes_out(self) -> None:
        """Refusing it would skip a file, which is worse than a long line."""
        huge = "x" * (ARGUMENT_BUDGET + 10) + ".rs"
        self.assertEqual(batches([huge]), [[huge]])
        self.assertEqual(batches(["a.rs", huge]), [["a.rs"], [huge]])


class ReportingTests(unittest.TestCase):
    def test_a_diff_names_its_file_once(self) -> None:
        output = (
            "Diff in /work/crates/rspice-core/src/lib.rs:12:\n"
            "-    let x = ( 1 );\n"
            "+    let x = (1);\n"
            "Diff in /work/crates/rspice-core/src/lib.rs:40:\n"
            "Diff in /work/crates/rspice-app/src/main.rs:3:\n"
        )
        self.assertEqual(
            offenders(output),
            [
                "/work/crates/rspice-core/src/lib.rs",
                "/work/crates/rspice-app/src/main.rs",
            ],
        )

    def test_clean_output_names_nothing(self) -> None:
        self.assertEqual(offenders(""), [])
        self.assertEqual(offenders("rustfmt 1.8.0-stable\n"), [])

    def test_a_windows_extended_length_path_is_named_from_the_root(self) -> None:
        """rustfmt reports `\\\\?\\C:\\...` on Windows, which is not a subpath.

        The gate printed a traceback instead of the offending file until the
        prefix was taken off, which is a poor way to learn that a file needs
        formatting.
        """
        from check_formatting import ROOT

        extended = "\\\\?\\" + str(ROOT / "crates" / "rspice-core" / "src" / "lib.rs")
        self.assertEqual(reportable(extended), "crates/rspice-core/src/lib.rs")

    def test_a_path_outside_the_workspace_is_reported_as_given(self) -> None:
        self.assertEqual(reportable("/elsewhere/foo.rs"), "/elsewhere/foo.rs")


class WorkspaceTests(unittest.TestCase):
    """The gate's floor and its reason for existing, against real metadata."""

    def test_the_member_floor_equals_the_hand_written_member_count(self) -> None:
        """A gate that enumerates can collapse; the floor is what catches it.

        The step this replaced asserted `test "$checked" -ge 41` for the same
        reason: an enumeration that silently returned a handful of packages
        would otherwise report success for checking almost nothing.
        """
        grouped = entry_points(metadata())
        members = {member for member, _ in grouped}
        self.assertEqual(
            MINIMUM_MEMBERS,
            len(members),
            "MINIMUM_MEMBERS must equal the number of hand-written members, so "
            "adding a crate raises the floor instead of leaving it behind",
        )

    def test_the_engine_crate_cannot_be_formatted_in_one_invocation(self) -> None:
        """Why this script exists rather than `cargo fmt -p <member>`.

        cargo fmt puts every target of a package on one rustfmt command line.
        rspice-core declares enough targets that the line exceeds the 32767
        bytes Windows allows, so `cargo fmt -p rspice-core` cannot start there
        and a formatting violation in the engine was invisible off CI.
        """
        grouped = entry_points(metadata())
        core = [paths for (member, _), paths in grouped.items() if member == "rspice-core"]
        self.assertTrue(core, "rspice-core must be covered by the formatting gate")
        widest = max(sum(len(path) + 3 for path in paths) for paths in core)
        self.assertGreater(
            widest,
            ARGUMENT_BUDGET,
            "if rspice-core now fits one command line, this gate may be simplified",
        )
        for paths in core:
            for run in batches(paths):
                self.assertLessEqual(sum(len(path) + 3 for path in run), ARGUMENT_BUDGET)


class WorkspaceDescriptionTests(unittest.TestCase):
    """What the gate says when cargo will not describe the workspace."""

    def run_main(self, failure: Exception) -> tuple[int, str]:
        printed = io.StringIO()
        with (
            mock.patch.object(
                check_formatting.subprocess, "check_output", side_effect=failure
            ),
            mock.patch.object(sys, "argv", ["check_formatting.py"]),
            contextlib.redirect_stdout(printed),
        ):
            return check_formatting.main(), printed.getvalue()

    def test_a_refusal_is_stated_in_cargos_own_words(self) -> None:
        """The gate used to raise `CalledProcessError` out of `main`.

        That reads as the gate itself being broken. Only cargo knows what it
        could not load, so its words are what the reader gets, with no guess
        at the cause added to them.
        """
        refusal = subprocess.CalledProcessError(101, ("cargo", "metadata"))
        refusal.stderr = (
            "error: failed to parse manifest at `/w/crates/x/Cargo.toml`\n"
        )
        code, printed = self.run_main(refusal)
        self.assertEqual(code, 1)
        self.assertIn("failed to parse manifest", printed)
        self.assertNotIn("Traceback", printed)

    def test_a_refusal_with_nothing_on_stderr_still_exits_rather_than_raising(
        self,
    ) -> None:
        """`stderr` is `None` when a caller captured it elsewhere."""
        code, printed = self.run_main(
            subprocess.CalledProcessError(101, ("cargo", "metadata"))
        )
        self.assertEqual(code, 1)
        self.assertIn("could not describe the workspace", printed)
        self.assertIn("exit status 101", printed)


class RustfmtOutcomeTests(unittest.TestCase):
    """What one rustfmt run's exit status and output make of the gate."""

    def run_main(self, completed: subprocess.CompletedProcess) -> tuple[int, str]:
        workspace = {
            "packages": [
                package(f"member-{index:02}", ("2024", f"/w/member-{index:02}/src/lib.rs"))
                for index in range(MINIMUM_MEMBERS)
            ]
        }
        printed = io.StringIO()
        with (
            mock.patch.object(check_formatting, "metadata", return_value=workspace),
            mock.patch.object(check_formatting.subprocess, "run", return_value=completed),
            mock.patch.object(sys, "argv", ["check_formatting.py"]),
            contextlib.redirect_stdout(printed),
        ):
            return check_formatting.main(), printed.getvalue()

    @staticmethod
    def outcome(returncode: int, stdout: str = "", stderr: str = "") -> subprocess.CompletedProcess:
        return subprocess.CompletedProcess(("rustfmt",), returncode, stdout, stderr)

    def test_a_clean_run_passes(self) -> None:
        code, printed = self.run_main(self.outcome(0))
        self.assertEqual(code, 0)
        self.assertIn(f"checked {MINIMUM_MEMBERS} target(s)", printed)

    def test_a_diff_names_the_file_and_fails(self) -> None:
        code, printed = self.run_main(
            self.outcome(1, stdout="Diff in /w/member-00/src/lib.rs:1:\n-a\n+b\n")
        )
        self.assertEqual(code, 1)
        self.assertIn("unformatted: /w/member-00/src/lib.rs", printed)

    def test_a_parse_error_beside_a_diff_is_still_reported(self) -> None:
        """rustfmt prints both and exits 1 when one run holds both files.

        Measured with rustfmt 1.8: the diff goes to stdout, the parse error to
        stderr. Reading only stdout told the reader to run --write, which does
        not fix a file that does not parse.
        """
        code, printed = self.run_main(
            self.outcome(
                1,
                stdout="Diff in /w/member-00/src/lib.rs:1:\n-a\n+b\n",
                stderr="error: this file contains an unclosed delimiter\n",
            )
        )
        self.assertEqual(code, 1)
        self.assertIn("unclosed delimiter", printed)

    def test_a_failure_that_names_no_file_is_not_a_pass(self) -> None:
        code, printed = self.run_main(self.outcome(1, stdout="something unexpected\n"))
        self.assertEqual(code, 1)
        self.assertIn("could not check", printed)
        self.assertIn("exit status 1", printed)


if __name__ == "__main__":
    unittest.main()
