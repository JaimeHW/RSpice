import importlib.util
import re
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
_XYCE_EXCLUSION_FIXTURE = None


def read_text(relative_path: str) -> str:
    return (ROOT / relative_path).read_text(encoding="utf-8")


def cargo_tree_for_target(package: str, target: str) -> str:
    result = subprocess.run(
        [
            "cargo",
            "tree",
            "--locked",
            "-p",
            package,
            "--target",
            target,
            "-e",
            "features",
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        raise AssertionError(
            f"cargo tree failed for {package} on {target}:\n{result.stderr}"
        )
    return result.stdout


def xyce_exclusion_fixture():
    global _XYCE_EXCLUSION_FIXTURE
    if _XYCE_EXCLUSION_FIXTURE is not None:
        return _XYCE_EXCLUSION_FIXTURE
    tool_path = ROOT / "tools/xyce/sync_upstream_exclusions.py"
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location("sync_upstream_exclusions", tool_path)
    if spec is None or spec.loader is None:
        raise AssertionError(f"cannot load {tool_path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    exclusions = module.recover_exclusions(ROOT)
    manifest = ROOT / module.MANIFEST_RELATIVE_PATH
    promotions = module.parse_existing_promotions(manifest)
    expected = module.render_manifest(exclusions, promotions)
    _XYCE_EXCLUSION_FIXTURE = (module, exclusions, manifest, expected)
    return _XYCE_EXCLUSION_FIXTURE


class CiConfigurationTests(unittest.TestCase):
    def test_xyce_upstream_exclusion_manifest_is_byte_exact_and_reproducible(self) -> None:
        module, _, manifest, expected = xyce_exclusion_fixture()
        module.verify_manifest_bytes(manifest, expected)

    def test_xyce_upstream_exclusion_check_rejects_noncanonical_bytes(self) -> None:
        module, _, _, expected = xyce_exclusion_fixture()
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "manifest.tsv"
            for label, mutation in [
                ("crlf", expected.replace("\n", "\r\n")),
                (
                    "source-case substitution",
                    expected.replace(
                        "Netlists/ABM_FREQ/RC_data.cir",
                        "Netlists/ABM_FREQ/RC_Data.cir",
                        1,
                    ),
                ),
            ]:
                with self.subTest(mutation=label):
                    path.write_bytes(mutation.encode("utf-8"))
                    with self.assertRaisesRegex(RuntimeError, "byte-for-byte"):
                        module.verify_manifest_bytes(path, expected)

    def test_xyce_upstream_exclusion_report_digest_mismatch_fails_closed(self) -> None:
        module, exclusions, _, _ = xyce_exclusion_fixture()
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "census.tsv"
            path.write_text("relative_path\tpassed\n", encoding="utf-8", newline="\n")
            with self.assertRaisesRegex(RuntimeError, "SHA-256"):
                module.parse_classification_report(path, exclusions)

    def test_xyce_upstream_exclusion_historical_baseline_is_an_exact_subset(self) -> None:
        module, _, _, _ = xyce_exclusion_fixture()
        current = {
            "netlists/old.cir": "historical_contract",
            "netlists/new.cir": "later_reviewed_contract",
        }
        module.validate_classification_baseline(
            current, {"netlists/old.cir": "historical_contract"}
        )

        with self.assertRaisesRegex(RuntimeError, "missing=1, changed=0"):
            module.validate_classification_baseline(
                {"netlists/new.cir": "later_reviewed_contract"},
                {"netlists/old.cir": "historical_contract"},
            )
        with self.assertRaisesRegex(RuntimeError, "missing=0, changed=1"):
            module.validate_classification_baseline(
                current, {"netlists/old.cir": "changed_contract"}
            )

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

    def test_no_shipping_crate_depends_on_the_conformance_crate(self) -> None:
        """Nothing a user installs may depend on rspice-conformance.

        The suites live outside rspice-core so the compiler enforces that they
        see only its public API — a suite able to assert on private state
        passes while the simulator a user gets regresses. A single
        `rspice-conformance = { path = ... }` line in a shipping crate would
        quietly undo that, and drag a test harness into a shipped binary.

        rspice-bench is deliberately exempt: it is a benchmark rig, not a
        product, and the Verilog-A stamp bench and golden-fingerprint capture
        live with that suite rather than inside the library they measure.
        """
        shipping = (
            "rspice-cli",
            "rspice-ui",
            "rspice-python",
            "rspice-wasm",
            "rspice-core",
            "rspice-publication-contract",
        )
        offenders = []
        for crate in shipping:
            manifest = ROOT / "crates" / crate / "Cargo.toml"
            if not manifest.exists():
                continue
            text = manifest.read_text(encoding="utf-8", errors="ignore")
            # Ignore prose in comments; only a real dependency entry counts.
            if any(
                re.match(r"\s*rspice-conformance\s*(=|\.)", line)
                for line in text.splitlines()
            ):
                offenders.append(crate)

        self.assertEqual(
            offenders,
            [],
            "these shipping crates depend on rspice-conformance: " + ", ".join(offenders),
        )

    def test_no_conformance_harness_remains_inside_rspice_core(self) -> None:
        """rspice-core ships no test harness and no conformance corpus runner.

        Both suites live in rspice-conformance. A harness back inside the
        library would be compiled into every shipping frontend and, worse,
        would regain access to `pub(crate)` internals — assertions on private
        state survive real numerical regressions, because they never travel
        the path a user's deck takes.
        """
        core_src = ROOT / "crates" / "rspice-core" / "src"
        self.assertFalse(
            (core_src / "testing").exists(),
            "rspice-core/src/testing must not come back; suites belong in rspice-conformance",
        )
        self.assertNotIn(
            "pub mod testing",
            read_text("crates/rspice-core/src/lib.rs"),
            "rspice-core must not re-declare a `testing` module",
        )
        self.assertNotRegex(
            read_text("crates/rspice-core/Cargo.toml"),
            r"(?m)^conformance = ",
            "the transitional `conformance` feature is obsolete once the harness moved out",
        )

        offenders = [
            path.relative_to(ROOT).as_posix()
            for path in sorted(core_src.rglob("*.rs"))
            if re.search(r"\b(xyce|ngspice)_runner\b", path.read_text(encoding="utf-8", errors="ignore"))
        ]
        self.assertEqual(offenders, [], "conformance runner code reappeared inside rspice-core")

    def test_engine_component_publication_is_conformance_gated(self) -> None:
        """A signed adapter may exist only after exact-source conformance."""
        workflow = read_text(".github/workflows/engine-component-release.yml")
        conformance_job = workflow.split("  conformance:", 1)[1].split("\n  publish:", 1)[0]
        publish_job = workflow.split("\n  publish:", 1)[1]

        self.assertIn("ref: ${{ github.sha }}", conformance_job)
        self.assertIn('test "$(git rev-parse HEAD)" = "$GITHUB_SHA"', conformance_job)
        self.assertIn("-p rspice-core --lib --tests", conformance_job)
        self.assertIn("-p rspice-engine-adapter", conformance_job)
        for target in (
            "ngspice_oracle_audit",
            "y_device_policy",
            "xyce_regression",
            "gf180mcu_devices",
            "execution_corpora",
            "ngspice_regression",
        ):
            self.assertIn(target, conformance_job)
        self.assertIn('grep -E "^TOTAL +113 tests"', conformance_job)
        self.assertIn('test "$failed" -eq 0', conformance_job)
        self.assertNotIn("continue-on-error", conformance_job)
        self.assertRegex(publish_job, r"(?m)^    needs: conformance$")
        self.assertNotIn("rspice-qualification-corpus", read_text("Cargo.toml"))
        self.assertFalse(
            (ROOT / ".github/workflows/simulation-qualification-release.yml").exists()
        )

    def test_conformance_crate_forwards_core_features_it_branches_on(self) -> None:
        """Every `feature = "x"` the suites test must be declared here too.

        The suites branch on rspice-core features to decide whether a deck
        should run or be marked unsupported. An unforwarded feature silently
        reads `false` in this crate, so the suite takes the "unsupported"
        branch and passes vacuously even when the models were compiled in —
        a conformance suite quietly skipping cases is the one outcome it
        must never have.
        """
        manifest = read_text("crates/rspice-conformance/Cargo.toml")
        declared = set(re.findall(r"(?m)^([a-z0-9-]+) = \[", manifest))

        # Only real gates count. A bare `feature = "..."` also occurs in prose
        # inside doc comments, so require it to sit inside a `cfg(...)`.
        gate = re.compile(r'cfg!?\([^)]*feature = "([a-z0-9-]+)"')
        used = set()
        for path in sorted((ROOT / "crates" / "rspice-conformance").rglob("*.rs")):
            used.update(gate.findall(path.read_text(encoding="utf-8", errors="ignore")))

        missing = sorted(used - declared)
        self.assertEqual(
            missing,
            [],
            "rspice-conformance branches on undeclared features (they read false): "
            + ", ".join(missing),
        )

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
        self.assertIn(
            "cargo test --locked -p rspice-matrix -p rspice-cli -p rspice-veriloga -p rspice-bench\n"
            "          -p rspice-publication-contract -p rspice-publish -p rspice-viewer",
            workflow,
        )
        self.assertIn("Gate generated Verilog-A Rust resources", workflow)
        self.assertIn("Check granular generated Verilog-A features", workflow)
        self.assertIn(
            "--features veriloga-model-hisimhv-va,veriloga-builtins-noise",
            workflow,
        )
        self.assertIn(
            "cargo run --locked -p rspice-bench -- generated-rust",
            workflow,
        )
        self.assertIn(
            "--features generated-stamp-subset -- generated-stamp",
            workflow,
        )
        self.assertIn("--max-corpus-median-reference-ratio 1.25", workflow)
        self.assertIn("--max-model-reference-ratio 4.25", workflow)
        for budget in [
            "--max-source-bytes 56000000",
            "--max-noise-source-bytes 17000000",
            "--max-model-source-bytes 4700000",
            "--max-file-count 380",
            "--max-pooled-workspace-payload-bytes 0",
            "--max-stamp-state-payload-bytes 6200",
        ]:
            self.assertIn(budget, workflow)
        self.assertIn("Compile Verilog-A native JIT tests (Linux x64)", workflow)
        self.assertIn(
            "cargo test --locked -p rspice-veriloga --features native native:: --no-run",
            workflow,
        )
        self.assertIn(
            "cargo test --locked -p rspice-veriloga --release --features native native:: --no-run",
            workflow,
        )
        self.assertIn("Test Verilog-A native JIT units (Linux x64)", workflow)
        self.assertRegex(
            workflow,
            r"cargo test --locked -p rspice-veriloga --features native native::\s+-- --test-threads=1",
        )
        self.assertRegex(
            workflow,
            r"cargo test --locked -p rspice-veriloga --release --features native native::\s+-- --test-threads=1",
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
        self.assertRegex(
            workflow,
            r"cargo run --locked -p rspice-bench --release -- native-jit\s+"
            r"--iterations 100000\s+"
            r"--samples 9\s+"
            r"--min-dense-speedup 1\.75\s+"
            r"--min-speedup 3\.00\s+"
            r"--min-full-stamp-speedup 2\.00\s+"
            r"--max-native-setup-ms 10\s+"
            r"--max-native-p95-ns-per-sweep 5000\s+"
            r"--max-relative-stddev 0\.25\s+"
            r"--max-native-code-bytes 16384",
        )
        self.assertIn("Test UI library (Linux)", workflow)
        self.assertIn("cargo test --locked -p rspice-ui --lib", workflow)
        self.assertGreaterEqual(
            workflow.count("run: cargo clean"),
            3,
            "Linux fast CI should clean between heavy test groups to stay within runner disk",
        )
        # The generated Verilog-A model crates are validated against a content
        # digest of their own bytes, so rustfmt must not touch them. `cargo fmt`
        # has no --exclude, so the gate names the hand-written members instead.
        # Assert every one of them is covered, or a new crate could silently
        # drop out of the format gate.
        self.assertRegex(workflow, r"- name: Format\s+run: >\s+cargo fmt")
        format_step = workflow.split("- name: Format", 1)[1].split("- name:", 1)[0]
        for package in (
            "rspice-matrix",
            "rspice-veriloga-runtime",
            "rspice-core",
            "rspice-cli",
            "rspice-publication-contract",
            "rspice-publish",
            "rspice-viewer",
            "rspice-ui",
            "rspice-veriloga",
            "rspice-python",
            "rspice-wasm",
            "rspice-bench",
            "rspice-conformance",
        ):
            self.assertIn(f"-p {package}", format_step)
        self.assertNotIn("-p rspice-veriloga-models", format_step)
        self.assertIn("-- --check", format_step)

    def test_core_lib_tests_are_explicitly_gated(self) -> None:
        ci_workflow = read_text(".github/workflows/ci.yml")
        nightly_workflow = read_text(".github/workflows/nightly.yml")

        self.assertIn("cargo test --locked -p rspice-core --lib", ci_workflow)
        self.assertIn("cargo test --locked -p rspice-core --lib --release", nightly_workflow)

    def test_digital_verilog_suites_are_named_in_the_fast_tier(self) -> None:
        """Both digital Verilog conformance targets run on every push.

        The conformance step names its test targets explicitly rather than
        using `--tests`, so a new target that nobody adds here is a suite that
        exists and never runs. Two of them are digital: `verilog_oracles` is
        the language corpus and `verilog_scale` is the gate-level scale suite,
        and neither needs an external simulator installed to be worth running
        — each checks its designs against expectations derived without one.
        """
        workflow = read_text(".github/workflows/ci.yml")
        step = workflow.split("- name: Conformance suite unit tests", 1)[1].split(
            "- name:", 1
        )[0]

        for target in ("--test verilog_oracles", "--test verilog_scale"):
            self.assertIn(target, step)
        self.assertEqual(step.count("--test verilog_scale"), 1)

    def test_nightly_qualifies_shipped_models_through_native_x64(self) -> None:
        nightly_workflow = read_text(".github/workflows/nightly.yml")

        self.assertIn(
            "Shipped models execute through the native x64 device route",
            nightly_workflow,
        )
        self.assertIn(
            "native_x64_shipped_model_devices_run_without_interpreter_fallback",
            nightly_workflow,
        )
        self.assertIn(
            "Shipped native x64 finite entries match bytecode",
            nightly_workflow,
        )
        self.assertIn(
            "native_x64_shipped_model_finite_entries_match_bytecode_reference",
            nightly_workflow,
        )
        self.assertGreaterEqual(
            nightly_workflow.count(
                "cargo test --locked --release -p rspice-veriloga --features native --lib"
            ),
            2,
            "shipped-model probes should compile only their library test target",
        )
        self.assertGreaterEqual(
            nightly_workflow.count('RUST_MIN_STACK: "67108864"'),
            3,
            "all full-corpus Verilog-A gates need the raised test-thread stack",
        )
        self.assertIn("--features generated-stamp -- generated-stamp", nightly_workflow)
        self.assertIn("--max-corpus-median-reference-ratio 5.75", nightly_workflow)
        self.assertIn("--max-model-reference-ratio 16.50", nightly_workflow)
        self.assertIn("target/veriloga-stamp-qualification.json", nightly_workflow)

    def test_generated_veriloga_has_desktop_browser_and_mobile_portability_rows(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertGreaterEqual(
            workflow.count(
                "--no-default-features --features veriloga-model-diode-cmc,veriloga-builtins-noise"
            ),
            3,
            "Linux, Windows, and macOS must compile the direct portable shard",
        )
        for target in [
            "wasm32-unknown-unknown",
            "aarch64-linux-android",
            "aarch64-apple-ios",
        ]:
            self.assertIn(target, workflow)
        self.assertIn("veriloga-mobile:", workflow)
        self.assertIn("Configure Android NDK C toolchain", workflow)
        self.assertIn(
            "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin",
            workflow,
        )
        self.assertIn("aarch64-linux-android21-clang", workflow)
        self.assertIn("CC_aarch64_linux_android", workflow)
        self.assertIn("AR_aarch64_linux_android", workflow)
        self.assertIn("CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER", workflow)

    def test_linux_ci_runs_clippy_warning_clean(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn(
            "cargo clippy --locked --workspace --exclude rspice-python --exclude rspice-wasm --all-targets --message-format short -- -D warnings",
            workflow,
        )

    def test_windows_ci_runs_native_veriloga_jit_tests(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn("runs-on: windows-latest", workflow)
        self.assertIn("Compile Verilog-A native JIT tests", workflow)
        self.assertIn("Verilog-A native JIT unit tests", workflow)
        self.assertIn("Verilog-A native JIT contract tests", workflow)
        self.assertIn("Verilog-A native multiplicity tests", workflow)
        self.assertIn("Smoke Verilog-A native JIT benchmark gate", workflow)
        self.assertRegex(
            workflow,
            r"cargo test --locked -p rspice-veriloga --features native native::\s+-- --test-threads=1",
        )
        self.assertRegex(
            workflow,
            r"cargo test --locked -p rspice-veriloga --release --features native native::\s+-- --test-threads=1",
        )
        self.assertIn(
            "cargo test --locked -p rspice-veriloga --features native-bytecode-contract-tests --test native_contract -- --test-threads=1",
            workflow,
        )
        self.assertIn(
            "cargo test --locked -p rspice-veriloga --features native-bytecode-contract-tests --test mfactor -- --test-threads=1",
            workflow,
        )
        self.assertRegex(
            workflow,
            r"cargo run --locked -p rspice-bench --release -- native-jit\s+"
            r"--iterations 100000\s+"
            r"--samples 9\s+"
            r"--min-dense-speedup 1\.75\s+"
            r"--min-speedup 3\.00\s+"
            r"--min-full-stamp-speedup 2\.00\s+"
            r"--max-native-setup-ms 10\s+"
            r"--max-native-p95-ns-per-sweep 5000\s+"
            r"--max-relative-stddev 0\.25\s+"
            r"--max-native-code-bytes 16384",
        )

    def test_macos_intel_ci_executes_native_veriloga_jit(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn("test-macos-x64-native:", workflow)
        self.assertIn("runs-on: macos-15-intel", workflow)
        self.assertIn('test "$(uname -m)" = "x86_64"', workflow)
        self.assertIn("Test Verilog-A native JIT (macOS Intel x64)", workflow)
        self.assertIn(
            "cargo test --locked -p rspice-veriloga --features native native:: -- --test-threads=1",
            workflow,
        )
        self.assertIn(
            "cargo test --locked -p rspice-veriloga --release --features native native:: -- --test-threads=1",
            workflow,
        )
        self.assertIn("Gate Verilog-A native JIT performance", workflow)

    def test_aarch64_desktop_ci_executes_native_veriloga_jit(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")
        aarch64_job = workflow.split("  test-aarch64-native:", maxsplit=1)[1].split(
            "\n  check-macos-desktop:", maxsplit=1
        )[0]

        self.assertIn("test-aarch64-native:", workflow)
        for runner in ["ubuntu-24.04-arm", "macos-15", "windows-11-arm"]:
            self.assertIn(f"os: {runner}", workflow)
        self.assertIn("Test AArch64 encoder, runtime, unwind, and model JIT", workflow)
        self.assertIn("Test public host-native qualification report", workflow)
        self.assertIn("Qualify full shipped device census through the AArch64 JIT", workflow)
        self.assertIn("Gate AArch64 native JIT performance", workflow)
        # The two machine backends encode, allocate, and verify independently,
        # so the census qualifies only the architecture it runs on.
        self.assertIn(
            "Qualify full shipped device census through the x64 JIT (Linux x64)",
            workflow,
        )
        self.assertEqual(workflow.count("--test native_shipped_models"), 2)
        self.assertEqual(workflow.count('RUST_MIN_STACK: "67108864"'), 2)
        self.assertIn("--min-dense-speedup 1.50", aarch64_job)
        self.assertIn("--min-speedup 3.00", aarch64_job)
        self.assertIn("--min-full-stamp-speedup 2.00", aarch64_job)

    def test_macos_hardened_jit_uses_only_narrow_execution_entitlements(self) -> None:
        import plistlib

        workflow = read_text(".github/workflows/ci.yml")
        entitlement_path = ROOT / "crates/rspice-ui/macos/RSpice.entitlements"
        with entitlement_path.open("rb") as source:
            entitlements = plistlib.load(source)

        self.assertEqual(
            entitlements,
            {
                "com.apple.security.cs.allow-jit": True,
                "com.apple.security.cs.jit-write-allowlist": True,
            },
        )
        self.assertIn("Test macOS hardened-runtime JIT entitlements", workflow)
        self.assertIn("Test Intel macOS hardened-runtime JIT entitlements", workflow)
        self.assertIn("--options runtime", workflow)
        # Workflow paths are POSIX; formatting with the host separator makes
        # this gate unrunnable on Windows, where it fails for a reason that has
        # nothing to do with the entitlements it checks.
        self.assertIn(entitlement_path.relative_to(ROOT).as_posix(), workflow)
        self.assertIn("codesign --verify --strict", workflow)
        self.assertIn(
            "registered_dwarf_cfi_unwinds_through_generated_helper_frame",
            workflow,
        )

    def test_shipping_frontends_enable_jit_on_qualified_desktop_architectures(self) -> None:
        cli_manifest = read_text("crates/rspice-cli/Cargo.toml")
        ui_manifest = read_text("crates/rspice-ui/Cargo.toml")
        architecture = 'any(target_arch = "x86_64", target_arch = "aarch64")'
        desktop_os = 'any(target_os = "macos", target_os = "linux", windows)'
        qualified = f"all({architecture}, {desktop_os})"
        cli_native = f"[target.'cfg({qualified})'.dependencies]"
        cli_portable = f"[target.'cfg(not({qualified}))'.dependencies]"
        ui_native = (
            f"[target.'cfg(all(not(target_arch = \"wasm32\"), "
            f"{architecture}, {desktop_os}))'.dependencies]"
        )
        ui_portable = (
            f"[target.'cfg(all(not(target_arch = \"wasm32\"), not({qualified})))'.dependencies]"
        )

        self.assertIn(cli_native, cli_manifest)
        self.assertIn(cli_portable, cli_manifest)
        self.assertIn(ui_native, ui_manifest)
        self.assertIn(ui_portable, ui_manifest)

        self.assertIn(
            'rspice-core = { workspace = true, features = ["veriloga-native"] }',
            cli_manifest.split(cli_native, maxsplit=1)[1].split(cli_portable, maxsplit=1)[0],
        )
        self.assertIn(
            'rspice-core = { workspace = true, features = ["veriloga"] }',
            cli_manifest.split(cli_portable, maxsplit=1)[1],
        )
        self.assertIn(
            'features = ["veriloga-native"]',
            ui_manifest.split(ui_native, maxsplit=1)[1].split(ui_portable, maxsplit=1)[0],
        )
        self.assertIn(
            'features = ["veriloga"]',
            ui_manifest.split(ui_portable, maxsplit=1)[1],
        )

        for mobile_target in ["aarch64-linux-android", "x86_64-linux-android"]:
            tree = cargo_tree_for_target("rspice-ui", mobile_target)
            self.assertNotIn('rspice-veriloga feature "native"', tree)
            self.assertIn('rspice-core feature "veriloga"', tree)

    def test_wasm_ci_checks_ui_and_bindings_warning_clean(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn("cargo check --locked -p rspice-wasm --target wasm32-unknown-unknown", workflow)
        self.assertIn(
            "cargo check --locked -p rspice-ui --features generated-veriloga-catalog --target wasm32-unknown-unknown",
            workflow,
        )
        self.assertIn(
            "cargo check --locked -p rspice-ui --bin rspice-ui-worker --features browser-worker --target wasm32-unknown-unknown",
            workflow,
        )
        self.assertIn(
            "cargo build --locked -p rspice-wasm --lib --target wasm32-unknown-unknown --release",
            workflow,
        )
        self.assertIn(
            "cargo build --locked --profile web-release -p rspice-ui --bin rspice-ui --target wasm32-unknown-unknown",
            workflow,
        )
        self.assertIn("--bin rspice-ui-worker --features browser-worker", workflow)
        self.assertIn("tools/ci/check_wasm_artifact_size.py", workflow)
        self.assertIn("--max-raw 67108864", workflow)
        self.assertIn("--max-raw 25165824", workflow)
        self.assertGreaterEqual(
            workflow.count("RUSTFLAGS: -D warnings"),
            2,
            "wasm checks should deny warnings for both wasm crates",
        )

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
        self.assertIn('"$binary" health --json', workflow)
        self.assertIn("tools/release/package_native.py", workflow)
        self.assertIn("cargo deny check advisories bans licenses sources", workflow)
        self.assertIn("cargo audit", workflow)
        self.assertIn("cargo cyclonedx --format json --spec-version 1.5", workflow)
        self.assertEqual(workflow.count("actions/attest@"), 2)
        self.assertIn("sha256sum --check --strict ./*.sha256", workflow)
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
            "cargo clippy --locked -p rspice-core --all-targets --all-features -- -D warnings",
            workflow,
        )
        self.assertIn(
            "cargo clippy --locked -p rspice-python --all-targets --all-features -- -D warnings",
            workflow,
        )
        self.assertIn("native-feature-lint:", workflow)
        self.assertIn('CARGO_BUILD_JOBS: "1"', workflow)
        self.assertIn('CARGO_PROFILE_DEV_DEBUG: "0"', workflow)
        self.assertIn('CARGO_PROFILE_TEST_DEBUG: "0"', workflow)
        self.assertIn("shared-key: python-native-feature-lint-linux", workflow)
        test_job = workflow.split("  native-feature-lint:", maxsplit=1)[0]
        native_lint_job = workflow.split("  native-feature-lint:", maxsplit=1)[1].split(
            "  wheels:", maxsplit=1
        )[0]
        self.assertIn("timeout-minutes: 60", native_lint_job)
        self.assertNotIn(
            "Lint every native feature combination",
            test_job,
            "all-feature Clippy must not run after building every Python matrix variant",
        )
        self.assertIn("python ../../tools/release/repair_sdist_lock.py", workflow)
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

    def test_python_binding_modules_stay_navigable(self) -> None:
        """No binding source file may exceed the ceiling, and none may grow.

        The binding layer had accumulated two monoliths (`results.rs` at 7363
        lines carrying 32 pyclasses, `engine.rs` at 4231 with a 45-method
        dispatch block). They are being split per analysis family. This ratchet
        holds the result: a file over the ceiling must appear in the allowlist
        with its current size, and a file may never exceed its allowlisted
        size, so the only permitted direction is down. An allowlisted file that
        has dropped under the ceiling must be removed from the allowlist, which
        keeps the list from rotting into a permanent exemption.
        """
        ceiling = 900

        # path -> current line count. Shrink an entry when a split lands; never
        # raise one.
        #
        # engine/mod.rs is the one structural exception rather than debt. PyO3
        # permits a single `#[pymethods]` block per type unless the
        # `multiple-pymethods` feature is enabled, which pulls in `inventory`
        # for a dependency this project does not want. That block is the whole
        # public Engine surface, and most of its bulk is the Python-facing
        # docstrings, which belong beside the signatures they document.
        # Everything that can leave has: the validators, the RF configuration
        # builders, the value types, the private helper impl, and the directive
        # runner all live in sibling modules.
        allowlist = {
            "crates/rspice-python/src/engine/mod.rs": 1940,
        }

        source_root = ROOT / "crates" / "rspice-python" / "src"
        over_ceiling = {}
        for path in sorted(source_root.rglob("*.rs")):
            relative = path.relative_to(ROOT).as_posix()
            lines = len(path.read_text(encoding="utf-8").splitlines())
            if lines > ceiling:
                over_ceiling[relative] = lines

        for relative, lines in sorted(over_ceiling.items()):
            with self.subTest(module=relative):
                self.assertIn(
                    relative,
                    allowlist,
                    f"{relative} is {lines} lines, over the {ceiling}-line ceiling; "
                    "split it per analysis family rather than adding it here",
                )
                self.assertLessEqual(
                    lines,
                    allowlist[relative],
                    f"{relative} grew to {lines} lines from an allowlisted "
                    f"{allowlist[relative]}; binding debt may only shrink",
                )

        stale = sorted(set(allowlist) - set(over_ceiling))
        self.assertEqual(
            [],
            stale,
            f"these modules are now under the {ceiling}-line ceiling and must be "
            "dropped from the allowlist",
        )

    def test_native_desktop_claim_has_macos_ui_cli_ci_coverage(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")

        self.assertIn("runs-on: macos-latest", workflow)
        self.assertIn("cargo check --locked -p rspice-cli -p rspice-ui", workflow)
        self.assertIn("cargo test --locked -p rspice-cli --tests", workflow)
        self.assertIn("cargo test --locked -p rspice-ui --lib", workflow)

    def test_ui_readme_matches_current_feature_flags_and_modules(self) -> None:
        readme = read_text("crates/rspice-ui/README.md")
        manifest = read_text("crates/rspice-ui/Cargo.toml")

        # The flags the crate declares, and the flags the README documents,
        # have to be the same set. A flag no `cfg` reads is not a compatibility
        # marker, it is a switch that silently does nothing, so the guard is
        # phrased as an equality rather than as a list of names to find.
        features_block = re.search(
            r"^\[features\]\n(.*?)(?:^\[|\Z)", manifest, flags=re.DOTALL | re.MULTILINE
        )
        self.assertIsNotNone(features_block)
        declared = set(re.findall(r"^([\w-]+)\s*=", features_block.group(1), re.MULTILINE))
        self.assertEqual(
            declared,
            {"default", "generated-veriloga-catalog", "browser-worker"},
            "rspice-ui declares a Cargo feature the README does not describe; add "
            "the row, or drop the flag if no cfg reads it",
        )
        for feature in declared - {"default"}:
            self.assertIn(f"| `{feature}` |", readme)
        self.assertNotIn("| `desktop` |", readme)
        self.assertNotIn("| `veriloga` |", readme)
        self.assertNotIn("utils/file_ops.rs", readme)
        self.assertNotIn("FileError::NotSupported", readme)

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

        app = read_text("crates/rspice-ui/src/workbench/app.rs")
        preferences = read_text(
            "crates/rspice-ui/src/workbench/app/dialogs/preferences/preference_pages.rs"
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

        self.assertNotIn("later milestone", playground_readme)
        self.assertNotIn("not this crate", ui_readme)
        self.assertIn("experimental browser IDE", ui_readme)
        self.assertIn("experimental browser IDE", playground_readme)

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
            # These moved out of `common/` and `workbench/netlist_document/`
            # during the module reorganization. The paths are updated rather than
            # dropped: each one was added here because it was once written but
            # never committed, and that is still worth guarding. `summary.rs`
            # left the list when the module was deleted as test-only.
            "crates/rspice-ui/src/workbench/browser/download.rs",
            "crates/rspice-ui/src/workbench/browser/file_import.rs",
            "crates/rspice-ui/src/workbench/logging.rs",
            "crates/rspice-ui/src/workbench/workflows/netlist_workflow.rs",
            "crates/rspice-ui/src/workbench/documents/netlist_document/baseline.rs",
            "crates/rspice-ui/src/workbench/documents/netlist_document/diagnostics.rs",
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
            focus_contracts = source.count("paint_focus_ring(") + source.count(
                "paint_focus_ring_outset("
            )
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
