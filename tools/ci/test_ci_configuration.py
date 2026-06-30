import re
import subprocess
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


def read_text(relative_path: str) -> str:
    return (ROOT / relative_path).read_text(encoding="utf-8")


class CiConfigurationTests(unittest.TestCase):
    def test_rust_workflows_use_pinned_toolchain(self) -> None:
        workflows = [
            ".github/workflows/ci.yml",
            ".github/workflows/nightly.yml",
            ".github/workflows/python.yml",
            ".github/workflows/deploy-site.yml",
        ]

        for workflow_path in workflows:
            with self.subTest(workflow=workflow_path):
                workflow = read_text(workflow_path)
                self.assertNotIn("dtolnay/rust-toolchain@stable", workflow)
                self.assertIn("dtolnay/rust-toolchain@1.94.0", workflow)

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
        self.assertIn("python3 tools/ci/test_ci_configuration.py", workflow)
        self.assertIn("python3 tools/ci/test_wasm_playground.py", workflow)
        self.assertIn("python3 tools/ci/test_ide_worker.py", workflow)
        self.assertIn("python3 tools/deploy/test_build_site.py", workflow)
        self.assertIn("python3 tools/deploy/test_deploy.py", workflow)
        self.assertRegex(
            workflow,
            r"- name: Clear check artifacts before tests\s+run: cargo clean",
        )
        self.assertNotIn(
            "cargo test --workspace --exclude rspice-python --exclude rspice-wasm\n"
            "          -- --skip test_ngspice_ --skip test_full_ngspice",
            workflow,
            "Linux fast CI should not link every workspace test target in one cargo invocation",
        )
        self.assertIn("Test core integration tests (fast tier)", workflow)
        self.assertIn("cargo test -p rspice-core --tests", workflow)
        self.assertIn("Test non-UI crates (fast tier)", workflow)
        self.assertIn("cargo test -p rspice-cli -p rspice-veriloga -p rspice-bench", workflow)
        self.assertIn("Test Verilog-A native JIT units (Linux x64)", workflow)
        self.assertIn(
            "cargo test -p rspice-veriloga --features native native:: -- --test-threads=1",
            workflow,
        )
        self.assertIn("Test Verilog-A native JIT contracts (Linux x64)", workflow)
        self.assertIn(
            "cargo test -p rspice-veriloga --features native --test native_contract -- --test-threads=1",
            workflow,
        )
        self.assertIn("Test Verilog-A native multiplicity contracts (Linux x64)", workflow)
        self.assertIn(
            "cargo test -p rspice-veriloga --features native --test mfactor -- --test-threads=1",
            workflow,
        )
        self.assertIn("Test Verilog-A native contract (Linux x64 release)", workflow)
        self.assertIn(
            "cargo test -p rspice-veriloga --release --features native --test native_contract -- --test-threads=1",
            workflow,
        )
        self.assertIn("Test UI library (Linux)", workflow)
        self.assertIn("cargo test -p rspice-ui --lib", workflow)
        self.assertGreaterEqual(
            workflow.count("run: cargo clean"),
            3,
            "Linux fast CI should clean between heavy test groups to stay within runner disk",
        )
        self.assertRegex(
            workflow,
            r"- name: Format\s+run: cargo fmt --all -- --check",
        )

    def test_core_lib_tests_are_explicitly_gated(self) -> None:
        ci_workflow = read_text(".github/workflows/ci.yml")
        nightly_workflow = read_text(".github/workflows/nightly.yml")

        self.assertIn("cargo test -p rspice-core --lib", ci_workflow)
        self.assertIn("cargo test -p rspice-core --lib --release", nightly_workflow)

    def test_linux_ci_runs_clippy_warning_clean(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn(
            "cargo clippy --workspace --exclude rspice-python --exclude rspice-wasm --all-targets --message-format short -- -D warnings",
            workflow,
        )

    def test_windows_ci_runs_native_veriloga_jit_tests(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn("runs-on: windows-latest", workflow)
        self.assertIn("Verilog-A native JIT unit tests", workflow)
        self.assertIn("Verilog-A native JIT contract tests", workflow)
        self.assertIn("Verilog-A native multiplicity tests", workflow)
        self.assertIn(
            "cargo test -p rspice-veriloga --features native native:: -- --test-threads=1",
            workflow,
        )
        self.assertIn(
            "cargo test -p rspice-veriloga --features native --test native_contract -- --test-threads=1",
            workflow,
        )
        self.assertIn(
            "cargo test -p rspice-veriloga --features native --test mfactor -- --test-threads=1",
            workflow,
        )

    def test_wasm_ci_checks_ui_and_bindings_warning_clean(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn("cargo check -p rspice-wasm --target wasm32-unknown-unknown", workflow)
        self.assertIn("cargo check -p rspice-ui --target wasm32-unknown-unknown", workflow)
        self.assertIn(
            "cargo build -p rspice-wasm --lib --target wasm32-unknown-unknown --release",
            workflow,
        )
        self.assertGreaterEqual(
            workflow.count("RUSTFLAGS: -D warnings"),
            2,
            "wasm checks should deny warnings for both wasm crates",
        )

    def test_native_desktop_claim_has_macos_ui_cli_ci_coverage(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn("runs-on: macos-latest", workflow)
        self.assertIn("cargo check -p rspice-cli -p rspice-ui", workflow)

    def test_python_workflow_runs_when_examples_change(self) -> None:
        workflow = read_text(".github/workflows/python.yml")

        self.assertIn('- "examples/python/**"', workflow)
        self.assertEqual(
            workflow.count('- "examples/python/**"'),
            2,
            "python workflow must include examples/python/** for push and pull_request filters",
        )
        self.assertIn("working-directory: examples/python", workflow)

    def test_platform_support_matrix_documents_evidence_and_mobile_limits(self) -> None:
        matrix = read_text("docs/platform-support.md")

        self.assertIn("Native desktop", matrix)
        self.assertIn("Browser IDE", matrix)
        self.assertIn("WASM playground", matrix)
        self.assertRegex(matrix, r"macOS.*cargo check -p rspice-cli -p rspice-ui")
        self.assertRegex(matrix.lower(), r"mobile/tablet.*experimental")

    def test_ui_readme_matches_current_feature_flags_and_modules(self) -> None:
        readme = read_text("crates/rspice-ui/README.md")
        manifest = read_text("crates/rspice-ui/Cargo.toml")

        self.assertIn("desktop = []", manifest)
        self.assertNotIn("utils/file_ops.rs", readme)
        self.assertNotIn("FileError::NotSupported", readme)
        self.assertIn(
            "Compatibility marker for native desktop builds; desktop-only behavior is selected by target-specific dependencies",
            readme,
        )

    def test_browser_surface_docs_distinguish_ide_and_playground(self) -> None:
        ui_readme = read_text("crates/rspice-ui/README.md")
        playground_readme = read_text("crates/rspice-wasm/web/README.md")
        site_index = read_text("site/index.html")

        self.assertNotIn("later milestone", playground_readme)
        self.assertNotIn("not this crate", ui_readme)
        self.assertIn("experimental browser IDE", ui_readme)
        self.assertIn("experimental browser IDE", playground_readme)
        self.assertIn("rspice.app/play", site_index)

    def test_site_mobile_copy_matches_experimental_support_matrix(self) -> None:
        index = read_text("site/index.html")
        download = read_text("site/download.html")

        self.assertIn("Experimental tablet/mobile preview", index)
        self.assertIn("repeatable device matrix", index)
        self.assertNotIn("ipad &amp; android browsers", index)
        self.assertNotIn("available today", download)
        self.assertIn("web/source available", download)
        self.assertIn("mobile browser preview", download)

    def test_site_validation_copy_does_not_overclaim_release_data(self) -> None:
        parity = read_text("site/parity.html")
        changelog = read_text("site/changelog.html")

        self.assertNotIn("Every release", parity)
        self.assertNotIn("Every release", changelog)
        self.assertIn("pre-release validation snapshot", parity)
        self.assertIn("static engineering snapshot", parity)

    def test_notice_includes_vendored_compact_model_attributions(self) -> None:
        notice = read_text("NOTICE")

        for required in [
            "PSP104.1.0_vacode",
            "JUNCAP200",
            "r3_cmc_release1.1.2_2023Jun16",
            "R3_CMC",
            "diode_cmc_3.0_20250714",
            "BSIM-CMG_112.1.0_04282026",
            "Si2 Compact Model Coalition",
        ]:
            with self.subTest(required=required):
                self.assertIn(required, notice)

        normalized_notice = re.sub(r"\s+", " ", notice)
        self.assertIn("Educational Community License, Version 2.0", normalized_notice)

    def test_klu_solver_provenance_audit_is_current(self) -> None:
        audit = read_text("docs/legal/ngspice-provenance-audit.md")
        core_readme = read_text("crates/rspice-core/README.md")

        self.assertIn("KLU-class", core_readme)
        self.assertIn("2026-06-18 KLU-class solver addendum", audit)
        self.assertNotIn("not re-audited", audit)
        self.assertIn("no SuiteSparse KLU source or binding is vendored", audit)

    def test_referenced_source_files_are_tracked(self) -> None:
        source_like_paths = [
            "crates/rspice-core/src/netlist/source_map.rs",
            "crates/rspice-core/tests/common/mod.rs",
            "crates/rspice-ui/src/common/browser_download.rs",
            "crates/rspice-ui/src/common/browser_file_import.rs",
            "crates/rspice-ui/src/common/logging.rs",
            "crates/rspice-ui/src/common/netlist_workflow.rs",
            "crates/rspice-ui/src/shell/views/netlist/baseline.rs",
            "crates/rspice-ui/src/shell/views/netlist/diagnostics.rs",
            "crates/rspice-ui/src/shell/views/netlist/summary.rs",
            "crates/rspice-ui/src/simulation/controller/manual_deck.rs",
            "crates/rspice-ui/src/simulation/runner/wasm_worker.rs",
            "crates/rspice-ui/src/simulation/runner/worker_contract.rs",
            "crates/rspice-ui/src/state/simulation/ac_bode.rs",
            "crates/rspice-veriloga/tests/support/mod.rs",
            "docs/platform-support.md",
        ]

        for path in source_like_paths:
            with self.subTest(path=path):
                result = subprocess.run(
                    ["git", "ls-files", "--error-unmatch", path],
                    cwd=ROOT,
                    capture_output=True,
                    text=True,
                )
                self.assertEqual(
                    result.returncode,
                    0,
                    f"{path} is referenced by tracked code/docs but is not tracked",
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
