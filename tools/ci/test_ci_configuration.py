import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


def read_text(relative_path: str) -> str:
    return (ROOT / relative_path).read_text(encoding="utf-8")


class CiConfigurationTests(unittest.TestCase):
    def test_rspice_python_generates_windows_abi3_import_library(self) -> None:
        manifest = read_text("crates/rspice-python/Cargo.toml")

        pyo3 = re.search(
            r"pyo3\s*=\s*\{[^}]*features\s*=\s*\[([^\]]*)\]",
            manifest,
            flags=re.DOTALL,
        )

        self.assertIsNotNone(pyo3, "rspice-python must declare PyO3 features")
        features = set(re.findall(r'"([^"]+)"', pyo3.group(1)))
        self.assertIn("abi3-py38", features)
        self.assertIn("generate-import-lib", features)

    def test_linux_fast_ci_reduces_test_artifact_pressure(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn('CARGO_BUILD_JOBS: "2"', workflow)
        self.assertIn('CARGO_PROFILE_DEV_DEBUG: "0"', workflow)
        self.assertIn('CARGO_PROFILE_TEST_DEBUG: "0"', workflow)
        self.assertRegex(
            workflow,
            r"- name: Clear check artifacts before tests\s+run: cargo clean",
        )

    def test_site_validation_copy_does_not_overclaim_release_data(self) -> None:
        parity = read_text("site/parity.html")
        changelog = read_text("site/changelog.html")

        for page in (parity, changelog):
            self.assertNotIn("Every release", page)

        self.assertIn("pre-release validation snapshot", parity)
        self.assertIn("static engineering snapshot", parity)
        self.assertNotIn("sample data", parity)
        self.assertNotIn("wired to CI", parity)

        self.assertIn("Release candidates", changelog)
        self.assertIn("Release targets and history", changelog)

    def test_public_copy_uses_bounded_pre_release_claims(self) -> None:
        index = read_text("site/index.html")
        parity = read_text("site/parity.html")
        download = read_text("site/download.html")
        veriloga_lib = read_text("crates/rspice-veriloga/src/lib.rs")

        public_pages = "\n".join([index, parity, download])
        for overclaim in (
            "111 / 113",
            "111/113",
            "2.4 M",
            "< 1e-9 V",
            "the same deck gives the same answer, every time",
            "takes your foundry's .va decks, NDA and all",
            "bring your foundry's .va decks",
            "Cloud simulation runners",
            "On-prem cloud runners",
            "SLAs",
            "escrow",
            "rspice-1.0.0",
        ):
            self.assertNotIn(overclaim, public_pages)

        self.assertIn("supported Verilog-A modules", index)
        self.assertIn("documented build and thread settings", index)
        self.assertIn("example manifest format", download)
        self.assertIn("supported analog subset", veriloga_lib)
        self.assertNotIn("full Verilog-A Language Reference Manual", veriloga_lib)


if __name__ == "__main__":
    unittest.main()
