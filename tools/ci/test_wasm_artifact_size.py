#!/usr/bin/env python3
"""Exercise size reporting and explicit deployment-budget failure behavior."""

from __future__ import annotations

import gzip
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("check_wasm_artifact_size.py")
EMPTY_MODULE = b"\x00asm\x01\x00\x00\x00"


class ArtifactSizeTests(unittest.TestCase):
    def run_report(self, payload: bytes | None, *args: str) -> subprocess.CompletedProcess[str]:
        with tempfile.TemporaryDirectory() as directory:
            artifact = Path(directory) / "module.wasm"
            if payload is not None:
                artifact.write_bytes(payload)
            return subprocess.run(
                [sys.executable, str(SCRIPT), str(artifact), "--label", "test", *args],
                capture_output=True,
                text=True,
                check=False,
            )

    def test_reports_without_a_budget(self) -> None:
        result = self.run_report(EMPTY_MODULE)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("test: raw=8 bytes", result.stdout)
        self.assertIn("gzip=", result.stdout)

    def test_missing_empty_and_invalid_artifacts_fail(self) -> None:
        for payload in (None, b"", b"\x00asm", b"not a wasm artifact"):
            with self.subTest(payload=payload):
                self.assertNotEqual(self.run_report(payload).returncode, 0)

    def test_explicit_budgets_accept_the_boundary_and_reject_excess(self) -> None:
        for flag, size in (
            ("--max-raw", len(EMPTY_MODULE)),
            ("--max-gzip", len(gzip.compress(EMPTY_MODULE, compresslevel=9, mtime=0))),
        ):
            with self.subTest(flag=flag):
                self.assertEqual(self.run_report(EMPTY_MODULE, flag, str(size)).returncode, 0)
                result = self.run_report(EMPTY_MODULE, flag, str(size - 1))
                self.assertEqual(result.returncode, 1, result.stderr)
                self.assertIn("exceeds release budget", result.stdout)


if __name__ == "__main__":
    unittest.main()
