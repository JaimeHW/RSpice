import re
import subprocess
import sys
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


def read_text(relative_path: str) -> str:
    return (ROOT / relative_path).read_text(encoding="utf-8")


class CiConfigurationTests(unittest.TestCase):
    def test_rust_workflows_install_pinned_toolchain_without_toolchain_action(self) -> None:
        workflows = [
            ".github/workflows/ci.yml",
            ".github/workflows/nightly.yml",
            ".github/workflows/python.yml",
            ".github/workflows/security.yml",
            ".github/workflows/coverage.yml",
            ".github/workflows/native-release.yml",
        ]

        for workflow_path in workflows:
            with self.subTest(workflow=workflow_path):
                workflow = read_text(workflow_path)
                self.assertNotIn("dtolnay/rust-toolchain", workflow)
                self.assertIn('RUST_TOOLCHAIN: "1.94.0"', workflow)
                self.assertIn("rustup toolchain install", workflow)

    def test_github_actions_are_pinned_to_full_commit_shas(self) -> None:
        workflows = sorted((ROOT / ".github" / "workflows").glob("*.yml"))

        for workflow_path in workflows:
            workflow = workflow_path.read_text(encoding="utf-8")
            for action in re.findall(r"uses:\s*([^#\s]+)", workflow):
                if action.startswith("./"):
                    continue
                with self.subTest(workflow=workflow_path.name, action=action):
                    self.assertIn("@", action)
                    ref = action.rsplit("@", 1)[1]
                    self.assertRegex(
                        ref,
                        r"^[0-9a-f]{40}$",
                        "third-party actions must be pinned to immutable commits",
                    )

    def test_workflows_set_minimum_token_permissions(self) -> None:
        read_only_workflows = [
            ".github/workflows/ci.yml",
            ".github/workflows/nightly.yml",
            ".github/workflows/python.yml",
            ".github/workflows/release-trigger.yml",
            ".github/workflows/security.yml",
            ".github/workflows/coverage.yml",
            ".github/workflows/native-release.yml",
        ]

        for workflow_path in read_only_workflows:
            with self.subTest(workflow=workflow_path):
                workflow = read_text(workflow_path)
                self.assertRegex(workflow, r"(?m)^permissions:\n  contents: read\n")

    def test_rspice_python_generates_windows_abi3_import_library(self) -> None:
        manifest = read_text("crates/rspice-python/Cargo.toml")

        pyo3 = re.search(
            r"pyo3\s*=\s*\{[^}]*features\s*=\s*\[([^\]]*)\]",
            manifest,
            flags=re.DOTALL,
        )

        self.assertIsNotNone(pyo3, "rspice-python must declare PyO3 features")
        features = set(re.findall(r'"([^"]+)"', pyo3.group(1)))
        self.assertIn("abi3-py310", features)
        self.assertIn("generate-import-lib", features)

    def test_rspice_python_windows_runtime_detection_supports_py_launcher(self) -> None:
        build_script = read_text("crates/rspice-python/build.rs")

        self.assertIn('OsString::from("python")', build_script)
        self.assertIn('OsString::from("py")', build_script)
        self.assertIn('&["-3"]', build_script)
        self.assertIn("query_python_base_prefix", build_script)

    def test_native_jit_has_no_cranelift_dependency_or_source_references(self) -> None:
        active_paths = [ROOT / "Cargo.toml", ROOT / "Cargo.lock"]
        active_paths.extend((ROOT / "crates").rglob("Cargo.toml"))
        active_paths.extend((ROOT / "crates").rglob("*.rs"))

        offenders = []
        for path in active_paths:
            text = path.read_text(encoding="utf-8", errors="ignore")
            if re.search(r"cranelift", text, flags=re.IGNORECASE):
                offenders.append(path.relative_to(ROOT).as_posix())

        self.assertEqual(
            [],
            offenders,
            "native JIT must not reintroduce Cranelift in active manifests or Rust source",
        )

    def test_bytecode_native_compiler_surface_is_contract_test_only(self) -> None:
        native_mod = read_text("crates/rspice-veriloga/src/native/mod.rs")
        x64_mod = read_text("crates/rspice-veriloga/src/native/x64/mod.rs")

        self.assertRegex(
            native_mod,
            r'#\[cfg\(feature = "native-bytecode-contract-tests"\)\]\s+pub fn compile_native\(',
            "bytecode-native compiler entry must stay out of production native builds",
        )
        self.assertRegex(
            x64_mod,
            r'#\[cfg\(feature = "native-bytecode-contract-tests"\)\]\s+pub\(crate\) fn compile_model\(',
            "x64 bytecode-native lowering must stay contract-test only",
        )

    def test_linux_fast_ci_reduces_test_artifact_pressure(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn('CARGO_BUILD_JOBS: "2"', workflow)
        self.assertIn('CARGO_PROFILE_DEV_DEBUG: "0"', workflow)
        self.assertIn('CARGO_PROFILE_TEST_DEBUG: "0"', workflow)
        self.assertIn("python3 tools/ci/test_ci_configuration.py", workflow)
        self.assertIn('ACTIONLINT_VERSION: "1.7.12"', workflow)
        self.assertIn(
            'go install "github.com/rhysd/actionlint/cmd/actionlint@v${ACTIONLINT_VERSION}"',
            workflow,
        )
        self.assertIn('"$GOBIN/actionlint" .github/workflows/*.yml', workflow)
        self.assertIn("python3 tools/ci/test_wasm_playground.py", workflow)
        self.assertIn("python3 tools/ci/test_ide_worker.py", workflow)
        self.assertIn("python3 tools/deploy/test_build_site.py", workflow)
        self.assertIn("python3 tools/release/test_package_native.py", workflow)
        self.assertRegex(
            workflow,
            r"- name: Clear check artifacts before tests\s+run: cargo clean",
        )
        self.assertNotIn(
            "cargo test --locked --workspace --exclude rspice-python --exclude rspice-wasm\n"
            "          -- --skip test_ngspice_ --skip test_full_ngspice",
            workflow,
            "Linux fast CI should not link every workspace test target in one cargo invocation",
        )
        self.assertIn("Test core integration tests (fast tier)", workflow)
        self.assertIn("cargo test --locked -p rspice-core --tests", workflow)
        self.assertIn("Test non-UI crates (fast tier)", workflow)
        self.assertIn("cargo test --locked -p rspice-cli -p rspice-veriloga -p rspice-bench", workflow)
        self.assertIn("Test Verilog-A native JIT units (Linux x64)", workflow)
        self.assertRegex(
            workflow,
            r"cargo test --locked -p rspice-veriloga --features native native::\s+-- --test-threads=1",
        )
        self.assertIn("Test Verilog-A native JIT contracts (Linux x64)", workflow)
        self.assertIn(
            "cargo test --locked -p rspice-veriloga --features native-bytecode-contract-tests --test native_contract -- --test-threads=1",
            workflow,
        )
        self.assertIn("Test Verilog-A native multiplicity contracts (Linux x64)", workflow)
        self.assertIn(
            "cargo test --locked -p rspice-veriloga --features native-bytecode-contract-tests --test mfactor -- --test-threads=1",
            workflow,
        )
        self.assertIn("Test Verilog-A native contract (Linux x64 release)", workflow)
        self.assertIn(
            "cargo test --locked -p rspice-veriloga --release --features native-bytecode-contract-tests --test native_contract -- --test-threads=1",
            workflow,
        )
        self.assertIn("Smoke Verilog-A native JIT benchmark gate (Linux x64)", workflow)
        self.assertIn(
            "cargo run --locked -p rspice-bench --release -- native-jit --iterations 10000 --samples 7 --min-speedup 1.10",
            workflow,
        )
        self.assertIn("Test UI library (Linux)", workflow)
        self.assertIn("cargo test --locked -p rspice-ui --lib", workflow)
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

        self.assertIn("cargo test --locked -p rspice-core --lib", ci_workflow)
        self.assertIn("cargo test --locked -p rspice-core --lib --release", nightly_workflow)

    def test_linux_ci_runs_clippy_warning_clean(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn(
            "cargo clippy --locked --workspace --exclude rspice-python --exclude rspice-wasm --all-targets --message-format short -- -D warnings",
            workflow,
        )

    def test_windows_ci_runs_native_veriloga_jit_tests(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn("runs-on: windows-latest", workflow)
        self.assertIn("Verilog-A native JIT unit tests", workflow)
        self.assertIn("Verilog-A native JIT contract tests", workflow)
        self.assertIn("Verilog-A native multiplicity tests", workflow)
        self.assertIn("Smoke Verilog-A native JIT benchmark gate", workflow)
        self.assertRegex(
            workflow,
            r"cargo test --locked -p rspice-veriloga --features native native::\s+-- --test-threads=1",
        )
        self.assertIn(
            "cargo test --locked -p rspice-veriloga --features native-bytecode-contract-tests --test native_contract -- --test-threads=1",
            workflow,
        )
        self.assertIn(
            "cargo test --locked -p rspice-veriloga --features native-bytecode-contract-tests --test mfactor -- --test-threads=1",
            workflow,
        )
        self.assertIn(
            "cargo run --locked -p rspice-bench --release -- native-jit --iterations 10000 --samples 7 --min-speedup 1.10",
            workflow,
        )

    def test_wasm_ci_checks_ui_and_bindings_warning_clean(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn("cargo check --locked -p rspice-wasm --target wasm32-unknown-unknown", workflow)
        self.assertIn("cargo check --locked -p rspice-ui --target wasm32-unknown-unknown", workflow)
        self.assertIn(
            "cargo build --locked -p rspice-wasm --lib --target wasm32-unknown-unknown --release",
            workflow,
        )
        self.assertGreaterEqual(
            workflow.count("RUSTFLAGS: -D warnings"),
            2,
            "wasm checks should deny warnings for both wasm crates",
        )

    def test_ci_smokes_the_same_browser_release_assembly_used_in_production(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")
        build_script = read_text("tools/deploy/build_site.py")
        release_trigger = read_text(".github/workflows/release-trigger.yml")

        self.assertIn("Browser site smoke (wasm)", workflow)
        self.assertIn("repository: JaimeHW/RSpice-Site", workflow)
        self.assertIn(
            "python3 tools/build_site.py --out dist --rspice-source ..",
            workflow,
        )
        self.assertIn(
            "python3 tools/deploy/build_site.py --site-source _site-source/dist --out _site-ci",
            workflow,
        )
        self.assertIn("wasm-bindgen-cli (pinned to Cargo.lock)", workflow)
        self.assertIn(
            'cargo install wasm-bindgen-cli --version "$VERSION" --locked', workflow
        )
        self.assertNotIn("curl -sSfL", workflow)
        self.assertIn("site-smoke-bundle", workflow)
        self.assertIn("if-no-files-found: error", workflow)
        self.assertIn("RSpice-Release", release_trigger)
        self.assertIn("github.event.workflow_run.head_sha", release_trigger)
        self.assertIn("--field channel=production", release_trigger)
        self.assertIn('"tablet", 820, 1180', build_script)
        self.assertIn('"phone", 390, 844', build_script)
        self.assertIn("cargo\", \"build\", \"--locked\"", build_script)

    def test_native_release_is_immutable_attested_and_recoverable(self) -> None:
        workflow = read_text(".github/workflows/native-release.yml")

        self.assertIn('tags: ["v*"]', workflow)
        self.assertIn('if [ "$GITHUB_REF_TYPE" != "tag" ]', workflow)
        self.assertIn("release tags must be annotated", workflow)
        self.assertIn("git merge-base --is-ancestor", workflow)
        self.assertEqual(
            len(re.findall(r"(?m)^\s+target: [A-Za-z0-9_.-]+$", workflow)), 6
        )
        for target in [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
            "aarch64-apple-darwin",
            "x86_64-apple-darwin",
            "x86_64-pc-windows-msvc",
            "aarch64-pc-windows-msvc",
        ]:
            self.assertIn(target, workflow)
        self.assertIn("cargo build --locked --release -p rspice-cli", workflow)
        self.assertIn(
            '"$binary" --config config/production.toml health --json', workflow
        )
        self.assertIn("tools/release/package_native.py", workflow)
        self.assertIn("cargo deny check advisories bans licenses sources", workflow)
        self.assertIn("cargo audit", workflow)
        self.assertIn("cargo cyclonedx --format json --spec-version 1.5", workflow)
        self.assertEqual(workflow.count("actions/attest@"), 2)
        self.assertIn("sha256sum --check --strict *.sha256", workflow)
        self.assertIn("gh release upload", workflow)
        self.assertNotIn("--clobber", workflow)
        self.assertIn("cmp --silent", workflow)
        self.assertIn("refusing to replace immutable release asset", workflow)
        self.assertIn("gh release create", workflow)
        self.assertIn("--verify-tag", workflow)

    def test_security_and_coverage_workflows_are_present(self) -> None:
        security = read_text(".github/workflows/security.yml")
        coverage = read_text(".github/workflows/coverage.yml")
        nightly = read_text(".github/workflows/nightly.yml")
        deny = read_text("deny.toml")
        dependabot = read_text(".github/dependabot.yml")

        self.assertIn("actions/dependency-review-action@", security)
        self.assertIn("cargo deny check advisories bans licenses sources", security)
        self.assertIn("cargo audit", security)
        self.assertIn("cargo cyclonedx --format json --target all", security)
        self.assertIn(
            "python3 tools/security/check_advisory_exceptions.py", security
        )
        self.assertIn("if-no-files-found: error", security)
        self.assertIn("cargo llvm-cov --locked --workspace", coverage)
        self.assertIn("mkdir -p target/coverage", coverage)
        self.assertLess(
            coverage.index("mkdir -p target/coverage"),
            coverage.index("--lcov --output-path target/coverage/lcov.info"),
            "coverage output directory must exist before cargo-llvm-cov writes the report",
        )
        self.assertIn("if-no-files-found: error", coverage)
        self.assertIn("actions/attest@", nightly)
        self.assertIn("attestations: write", nightly)
        self.assertIn("subject-path: release-artifacts/*", nightly)
        self.assertIn("unknown-registry = \"deny\"", deny)
        self.assertIn("unknown-git = \"deny\"", deny)
        self.assertIn("package-ecosystem: \"cargo\"", dependabot)
        self.assertIn("package-ecosystem: \"github-actions\"", dependabot)
        self.assertIn("package-ecosystem: \"pip\"", dependabot)

    def test_security_exceptions_are_owned_scoped_and_unexpired(self) -> None:
        result = subprocess.run(
            [sys.executable, "tools/security/check_advisory_exceptions.py"],
            cwd=ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(
            result.returncode,
            0,
            f"advisory exception policy failed:\n{result.stdout}\n{result.stderr}",
        )

        security_policy = read_text("SECURITY.md")
        codeowners = read_text(".github/CODEOWNERS")
        self.assertIn("security/advisory-exceptions.toml", security_policy)
        self.assertIn("security/advisory-exceptions.toml", codeowners)
        self.assertIn("tools/security/", codeowners)

    def test_python_ci_matches_supported_abi_and_smokes_built_wheels(self) -> None:
        workflow = read_text(".github/workflows/python.yml")
        pyproject = read_text("crates/rspice-python/pyproject.toml")
        constraints = read_text("crates/rspice-python/ci-constraints.txt")

        self.assertIn('requires-python = ">=3.10"', pyproject)
        self.assertIn('requires = ["maturin==1.14.1"]', pyproject)
        self.assertIn(
            'python: ["3.10", "3.11", "3.12", "3.13", "3.14", "3.14t"]',
            workflow,
        )
        self.assertIn("maturin develop --release --locked", workflow)
        self.assertIn("args: --release --locked --compatibility pypi --out dist", workflow)
        self.assertIn("python -m pip install --force-reinstall -c ci-constraints.txt dist/*.whl", workflow)
        self.assertIn("wheel smoke passed", workflow)
        self.assertIn("Python 3.10 abi3 smoke passed", workflow)
        self.assertIn(
            "cargo clippy --locked -p rspice-core -p rspice-python --all-targets --all-features -- -D warnings",
            workflow,
        )
        self.assertIn("python scripts/repair_sdist_lock.py", workflow)
        self.assertIn("CARGO_NET_OFFLINE=true", workflow)
        self.assertIn("cp314t wheel (${{ matrix.platform.name }})", workflow)
        self.assertIn("-i python3.14t", workflow)
        self.assertIn("assert not sys._is_gil_enabled()", workflow)
        self.assertEqual(workflow.count("actions/attest@"), 3)
        self.assertIn("subject-path: crates/rspice-python/dist/*.whl", workflow)
        self.assertIn("subject-path: crates/rspice-python/dist/*.tar.gz", workflow)
        self.assertIn("maturin==1.14.1", constraints)
        self.assertIn("numpy==2.2.6", constraints)
        self.assertIn("numpy==2.5.0", constraints)

    def test_native_desktop_claim_has_macos_ui_cli_ci_coverage(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn("runs-on: macos-latest", workflow)
        self.assertIn("cargo check --locked -p rspice-cli -p rspice-ui", workflow)
        self.assertIn("cargo test --locked -p rspice-cli --tests", workflow)
        self.assertIn("cargo test --locked -p rspice-ui --lib", workflow)

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

    def test_ui_enables_accessibility_backend_for_each_runtime(self) -> None:
        manifest = read_text("crates/rspice-ui/Cargo.toml")

        native_dependencies = re.search(
            r'^\[target\.\'cfg\(not\(target_arch = "wasm32"\)\)\'\.dependencies\]'
            r"(.*?)^\[",
            manifest,
            flags=re.DOTALL | re.MULTILINE,
        )
        wasm_dependencies = re.search(
            r'^\[target\.\'cfg\(target_arch = "wasm32"\)\'\.dependencies\]'
            r"(.*?)\Z",
            manifest,
            flags=re.DOTALL | re.MULTILINE,
        )

        self.assertIsNotNone(native_dependencies)
        self.assertIsNotNone(wasm_dependencies)

        native_eframe = re.search(r"^eframe\s*=\s*\{([^}]*)\}", native_dependencies.group(1), re.MULTILINE)
        wasm_eframe = re.search(r"^eframe\s*=\s*\{([^}]*)\}", wasm_dependencies.group(1), re.MULTILINE)
        self.assertIsNotNone(native_eframe)
        self.assertIsNotNone(wasm_eframe)

        native_features = set(re.findall(r'"([^"]+)"', native_eframe.group(1)))
        wasm_features = set(re.findall(r'"([^"]+)"', wasm_eframe.group(1)))
        self.assertIn("accesskit", native_features)
        self.assertNotIn("web_screen_reader", native_features)
        self.assertIn("web_screen_reader", wasm_features)
        self.assertNotIn("accesskit", wasm_features)

        app = read_text("crates/rspice-ui/src/common/app/mod.rs")
        preferences = read_text(
            "crates/rspice-ui/src/common/app/app_preferences_dialog.rs"
        )
        self.assertIn(
            "options.screen_reader = self.state.ui.browser_spoken_feedback",
            app,
        )
        self.assertIn("Speak control changes (browser)", preferences)
        readme = read_text("crates/rspice-ui/README.md")
        self.assertIn(
            "browser backend does not expose that AccessKit tree through",
            readme,
        )
        self.assertIn("remains a release gate", readme)

    def test_browser_surface_docs_distinguish_ide_and_playground(self) -> None:
        ui_readme = read_text("crates/rspice-ui/README.md")
        playground_readme = read_text("crates/rspice-wasm/web/README.md")
        workflow = read_text(".github/workflows/ci.yml")
        release_trigger = read_text(".github/workflows/release-trigger.yml")

        self.assertNotIn("later milestone", playground_readme)
        self.assertNotIn("not this crate", ui_readme)
        self.assertIn("experimental browser IDE", ui_readme)
        self.assertIn("experimental browser IDE", playground_readme)
        self.assertIn("repository: JaimeHW/RSpice-Site", workflow)
        self.assertIn("--site-source _site-source/dist", workflow)
        self.assertIn("RSpice-Release", release_trigger)

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

    def test_referenced_source_files_are_tracked(self) -> None:
        source_like_paths = [
            "crates/rspice-core/src/netlist/source_map.rs",
            "crates/rspice-core/tests/common/mod.rs",
            "crates/rspice-ui/src/common/browser_download.rs",
            "crates/rspice-ui/src/common/browser_file_import.rs",
            "crates/rspice-ui/src/common/logging.rs",
            "crates/rspice-ui/src/common/netlist_workflow.rs",
            "crates/rspice-ui/src/workbench/netlist_document/baseline.rs",
            "crates/rspice-ui/src/workbench/netlist_document/diagnostics.rs",
            "crates/rspice-ui/src/workbench/netlist_document/summary.rs",
            "crates/rspice-ui/src/simulation/controller/manual_deck.rs",
            "crates/rspice-ui/src/simulation/runner/wasm_worker.rs",
            "crates/rspice-ui/src/simulation/runner/worker_contract.rs",
            "crates/rspice-ui/src/state/simulation/ac_bode.rs",
            "crates/rspice-veriloga/tests/support/mod.rs",
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

    def test_veriloga_docs_do_not_overclaim_language_support(self) -> None:
        veriloga_lib = read_text("crates/rspice-veriloga/src/lib.rs")

        self.assertIn("supported analog subset", veriloga_lib)
        self.assertNotIn("full Verilog-A Language Reference Manual", veriloga_lib)

    def test_custom_ui_interactions_have_semantics_and_visible_focus(self) -> None:
        """Painter-backed controls must not bypass egui's accessibility contract."""
        click_pattern = re.compile(
            r"(?<![A-Za-z0-9_])(?:egui::)?Sense::(?:click|click_and_drag)\(\)"
        )
        offenders = []

        for path in sorted((ROOT / "crates" / "rspice-ui" / "src").rglob("*.rs")):
            source = path.read_text(encoding="utf-8")
            custom_clicks = len(click_pattern.findall(source))
            if custom_clicks == 0:
                continue

            pointer_only_shims = source.count("accessibility-pointer-shim")
            semantic_contracts = source.count(".widget_info(")
            focus_contracts = source.count("paint_focus_ring(")
            required = custom_clicks - pointer_only_shims
            if semantic_contracts < required or focus_contracts < required:
                offenders.append(
                    (
                        path.relative_to(ROOT).as_posix(),
                        custom_clicks,
                        pointer_only_shims,
                        semantic_contracts,
                        focus_contracts,
                    )
                )

        self.assertEqual(
            [],
            offenders,
            "every custom click/drag target needs WidgetInfo semantics and a visible "
            "focus ring; pointer-only forwarding shims must be explicitly annotated",
        )


if __name__ == "__main__":
    unittest.main()
