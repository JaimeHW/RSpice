#!/usr/bin/env python3
"""Exercise the mixed-suite fingerprint comparison the nightly gates on."""

from __future__ import annotations

from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("compare_trifamily_fingerprints.py")

BASELINE = """
running 3 tests
TRIFAMILY deck_a_points=1132
TRIFAMILY deck_a_grid_hash=94859e98f8c76df9
TRIFAMILY deck_a_volt_hash=1b54c78e1859dae5
TRIFAMILY deck_e_points=265
TRIFAMILY deck_e_grid_hash=0000000000000001
TRIFAMILY deck_e_volt_hash=0000000000000002
TRIFAMILY deck_a_op_vs_first_point_worst_node=A relative=2.4e-12 bit_exact=3/9
test result: ok. 3 passed
"""


class CompareFingerprintTests(unittest.TestCase):
    def compare(self, left: str, right: str, *args: str) -> subprocess.CompletedProcess[str]:
        with tempfile.TemporaryDirectory() as directory:
            paths = []
            for name, payload in (("a.log", left), ("b.log", right)):
                path = Path(directory) / name
                path.write_text(payload, encoding="utf-8")
                paths.append(str(path))
            return subprocess.run(
                [sys.executable, str(SCRIPT), *paths, "--min-keys", "6", *args],
                capture_output=True,
                text=True,
                check=False,
            )

    def test_identical_logs_agree(self) -> None:
        result = self.compare(BASELINE, BASELINE)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("agree on 6 fingerprint records", result.stdout)

    def test_a_moved_hash_fails_and_names_both_values(self) -> None:
        moved = BASELINE.replace("deck_a_grid_hash=94859e98f8c76df9", "deck_a_grid_hash=dead")
        result = self.compare(BASELINE, moved)
        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertIn("deck_a_grid_hash", result.stdout)
        self.assertIn("94859e98f8c76df9", result.stdout)
        self.assertIn("dead", result.stdout)

    def test_a_recorded_difference_is_allowed_and_still_printed(self) -> None:
        moved = BASELINE.replace("deck_a_grid_hash=94859e98f8c76df9", "deck_a_grid_hash=dead")
        result = self.compare(BASELINE, moved, "--allow-differing", "deck_a_grid_hash")
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertIn("recorded difference deck_a_grid_hash", result.stdout)

    def test_diagnostic_keys_are_not_gated_unless_asked(self) -> None:
        noisy = BASELINE.replace("relative=2.4e-12", "relative=9.9e-12")
        self.assertEqual(self.compare(BASELINE, noisy).returncode, 0)
        gated = self.compare(BASELINE, noisy, "--gate-all")
        self.assertEqual(gated.returncode, 1, gated.stdout)
        self.assertIn("deck_a_op_vs_first_point_worst_node", gated.stdout)

    def test_a_missing_suite_fails_rather_than_shrinking_the_comparison(self) -> None:
        short = "\n".join(
            line for line in BASELINE.splitlines() if not line.startswith("TRIFAMILY deck_e_")
        )
        result = self.compare(BASELINE, short)
        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertIn("fewer than the 6 required", result.stdout)
        self.assertIn("missing from", result.stdout)

    def test_a_required_key_must_be_present_in_both_logs(self) -> None:
        result = self.compare(BASELINE, BASELINE, "--require-key", "hardening_ring_1p_points")
        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertIn("required key hardening_ring_1p_points is missing", result.stdout)

    def test_one_run_that_disagrees_with_itself_fails(self) -> None:
        doubled = BASELINE + "TRIFAMILY deck_a_points=9999\n"
        result = self.compare(doubled, BASELINE)
        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertIn("printed with two different values in one run", result.stdout)

    def test_a_value_with_spaces_compares_as_the_whole_line(self) -> None:
        left = BASELINE + "TRIFAMILY deck_c_points=1290 deck_c_rejections=3\n"
        right = BASELINE + "TRIFAMILY deck_c_points=1290 deck_c_rejections=4\n"
        self.assertEqual(self.compare(left, left).returncode, 0)
        result = self.compare(left, right)
        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertIn("deck_c_rejections=3", result.stdout)

    def test_libtests_own_progress_prefix_does_not_swallow_a_record(self) -> None:
        """`--test-threads 1 --nocapture` puts `test <name> ... ` on the first line."""
        prefixed = BASELINE.replace(
            "TRIFAMILY deck_a_points=1132",
            "test deck_a_sequence_golden ... TRIFAMILY deck_a_points=1132",
        )
        result = self.compare(prefixed, BASELINE)
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertIn("agree on 6 fingerprint records", result.stdout)

    def test_an_empty_log_never_passes_as_agreement(self) -> None:
        result = self.compare("", "")
        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertIn("0 fingerprint records", result.stdout)

    def test_min_keys_is_mandatory(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "a.log"
            path.write_text(BASELINE, encoding="utf-8")
            result = subprocess.run(
                [sys.executable, str(SCRIPT), str(path), str(path)],
                capture_output=True,
                text=True,
                check=False,
            )
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertIn("--min-keys", result.stderr)


if __name__ == "__main__":
    unittest.main()
