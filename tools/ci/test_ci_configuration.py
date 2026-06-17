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


if __name__ == "__main__":
    unittest.main()
