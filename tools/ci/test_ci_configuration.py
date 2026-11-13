import ast
import importlib.util
import re
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
_XYCE_EXCLUSION_FIXTURE = None

# The `veriloga`-gated rspice-core integration test files that no pull-request
# lane runs, each with the reason. Every other gated file must build into a
# target both Verilog-A steps of `ci.yml` name; see
# `CiConfigurationTests.test_veriloga_gated_core_suites_run_on_both_routes`.
VERILOGA_GATED_CORE_TEST_EXCLUSIONS = {
    "public_paths_snapshot": (
        "holds no test: it only names public paths, so a route would run "
        "nothing. Its Verilog-A paths are compiled by the all-features Clippy "
        "in ci.yml's native-feature-lint job."
    ),
}

# Core test files that are not gated on `veriloga` but that a route's steps run
# as well, by route feature. The default-feature core lane runs each of them
# already; the route runs it again because the feature changes what it
# measures.
VERILOGA_ROUTE_UNGATED_CORE_TEST_FILES = {
    "veriloga": {
        "expression_stack_safety": (
            "bounds the stack a deeply nested expression may take on a 1 MiB "
            "thread, and the feature deepens the elaboration frames beneath the "
            "parser. The shipped application carries the feature."
        ),
    },
    "veriloga-native": {
        "expression_stack_safety": (
            "sits in the veriloga folder target both routes run, and the "
            "native feature deepens the same elaboration frames beneath the "
            "parser."
        ),
    },
}

# The two routes a Verilog-A module can reach the solver through, and the steps
# that run the gated core suites on each. Both lower the same modules, so a
# suite passing on one says nothing about the other.
#
# Each route takes two cargo invocations, not one. The tri-family and
# generated-route suites need `veriloga-model-diode-cmc` compiled in, and that
# feature reroutes cards the other suites assert about through the generated
# device, so the two sets cannot share a feature list. A gated file may be
# named by either invocation; what is checked is the union.
VERILOGA_GATED_CORE_TEST_STEPS = (
    (
        "test-linux-veriloga",
        "veriloga",
        (
            "Test Verilog-A-gated core suites through the bytecode interpreter",
            "Test the tri-family and generated-route suites through the bytecode interpreter",
        ),
    ),
    (
        "test-linux-native",
        "veriloga-native",
        (
            "Test Verilog-A-gated core suites through the native JIT (Linux x64)",
            "Test the tri-family and generated-route suites through the native JIT (Linux x64)",
        ),
    ),
)

# `#[cfg(...)]` and `#![cfg(...)]`, but deliberately NOT the `cfg!(...)` macro:
# `qualification_baseline` records `cfg!(feature = "veriloga")` as a fact inside
# the baseline document and is not gated by it, so a scan that read the macro
# would demand a lane for a suite the feature makes fail.
_CFG_ATTRIBUTE = re.compile(r"#!?\[\s*cfg(?:_attr)?\s*\(")


def read_text(relative_path: str) -> str:
    return (ROOT / relative_path).read_text(encoding="utf-8")


def workflow_job_body(workflow: str, job: str) -> str:
    """The YAML of one workflow job, up to the next job's key."""
    body = workflow.split(f"\n  {job}:\n", 1)
    if len(body) != 2:
        raise AssertionError(f"workflow has no job named {job!r}")
    return re.split(r"\n  [a-z][a-z0-9-]*:", body[1], maxsplit=1)[0]


def workflow_step_body(job: str, name: str) -> str:
    """The YAML of one step of a job, up to the next step's `- name:`."""
    body = job.split(f"- name: {name}\n", 1)
    if len(body) != 2:
        raise AssertionError(f"job has no step named {name!r}")
    return body[1].split("- name:", 1)[0]


def folded_run_command(step: str) -> str:
    """The command in a step's `run: >-` scalar, without what follows the block.

    The block ends at a blank line, a dedent, or a comment. Splitting on the
    next `- name:` instead leaves any comment written above the following step
    inside the text, and one apostrophe in such a comment is enough to make
    `shlex` report an unterminated quote over the whole command.
    """
    tail = step.split("run: >-", 1)[1].split("\n")[1:]
    indent = None
    collected = []
    for line in tail:
        if not line.strip() or line.lstrip().startswith("#"):
            break
        width = len(line) - len(line.lstrip())
        if indent is None:
            indent = width
        if width < indent:
            break
        collected.append(line.strip())
    return " ".join(collected)


def cfg_predicates(source: str) -> list[str]:
    """The predicate text of every `cfg` attribute in a Rust source file.

    Parentheses are balanced rather than matched with a regex, because the
    gates that matter are nested: `generated_route_parity` is gated on
    `all(feature = "veriloga", feature = "veriloga-model-diode-cmc")`, and a
    scan that only recognised the flat spelling would have missed it.
    """
    predicates = []
    for match in _CFG_ATTRIBUTE.finditer(source):
        depth = 1
        index = match.end()
        while index < len(source) and depth:
            if source[index] == "(":
                depth += 1
            elif source[index] == ")":
                depth -= 1
            index += 1
        predicates.append(source[match.end() : index - 1])
    return predicates


def targets_gated_on(directory: Path, feature: str) -> set[str]:
    """Integration targets in `directory` that a `cfg` attribute gates on `feature`."""
    wanted = re.compile(r'feature\s*=\s*"%s"' % re.escape(feature))
    return {
        path.stem
        for path in sorted(directory.glob("*.rs"))
        if any(
            wanted.search(predicate)
            for predicate in cfg_predicates(path.read_text(encoding="utf-8"))
        )
    }


_MOD_DECLARATION = re.compile(r"((?:[ \t]*#\[[^\n]*\n)*)[ \t]*(?:pub )?mod (\w+);")


def core_test_files() -> dict[str, tuple[str, str]]:
    """Every rspice-core integration test file: stem -> (target, cfg text).

    A top-level `tests/<stem>.rs` is its own target. A `tests/<folder>/main.rs`
    builds one target out of the files it declares as modules, and the
    attributes on a file's `mod` line gate it as an inner attribute would, so
    they are part of the text the scan reads. A `mod` with a `#[path]` is a
    shared helper, not a test file.
    """
    directory = ROOT / "crates" / "rspice-core" / "tests"
    files: dict[str, tuple[str, str]] = {}
    for path in sorted(directory.glob("*.rs")):
        files[path.stem] = (path.stem, path.read_text(encoding="utf-8"))
    for main in sorted(directory.glob("*/main.rs")):
        folder = main.parent
        for attributes, name in _MOD_DECLARATION.findall(main.read_text(encoding="utf-8")):
            if "#[path" in attributes:
                continue
            source = (folder / f"{name}.rs").read_text(encoding="utf-8")
            if name in files:
                raise AssertionError(f"core test file {name} is declared twice")
            files[name] = (folder.name, attributes + source)
    return files


def veriloga_gated_core_test_files() -> set[str]:
    """Every rspice-core integration test file `--features veriloga` switches on.

    Matched on the `cfg` predicate rather than the inner-attribute spelling: a
    file gated item by item, as `transient_checkpoint` is, links under the
    default feature set but runs a shorter suite there, which is exactly the
    silent coverage hole this scan exists to close.
    """
    wanted = re.compile(r'feature\s*=\s*"veriloga"')
    return {
        name
        for name, (_, text) in core_test_files().items()
        if any(wanted.search(predicate) for predicate in cfg_predicates(text))
    }


def contract_gated_veriloga_test_targets() -> set[str]:
    """rspice-veriloga integration targets gated on the contract-test feature.

    These exist only under `native-bytecode-contract-tests`, so no default
    `cargo test -p rspice-veriloga` links them at all.
    """
    return targets_gated_on(
        ROOT / "crates" / "rspice-veriloga" / "tests",
        "native-bytecode-contract-tests",
    )


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


class BrowserJitGateTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        spec = importlib.util.spec_from_file_location(
            "check_wasm_jit_browser", ROOT / "tools/ci/check_wasm_jit_browser.py"
        )
        assert spec is not None and spec.loader is not None
        cls.gate = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.gate)

    def test_worker_abi_matches_the_independent_release_contract(self) -> None:
        source = read_text("crates/rspice-veriloga/src/wasm_jit/mod.rs")
        version = re.search(r"pub const WASM_JIT_ABI_VERSION: u32 = (\d+);", source)
        self.assertIsNotNone(version)
        self.assertEqual(int(version.group(1)), self.gate.EXPECTED_WASM_JIT_ABI_VERSION)

    def test_independent_helper_fixture_uses_the_reviewed_abi(self) -> None:
        source = read_text("tools/ci/wasm_jit_abi_probe.mjs")
        version = re.search(r"const ABI = (\d+);", source)
        self.assertIsNotNone(version)
        self.assertEqual(int(version.group(1)), self.gate.EXPECTED_WASM_JIT_ABI_VERSION)

    def test_abi_fixture_uses_the_actual_source_and_immutable_worker_assets(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            for folder, compressed in [(root, False), (root / "assets" / ("a" * 64), True)]:
                worker = folder / "simulation-worker.js"
                assets = self.gate.qualification_assets(worker)
                package = folder if compressed else folder / "pkg"
                self.assertEqual(assets, {
                    "bindings": package / "rspice-ui-worker.js",
                    "loader": folder / "wasm-loader.js",
                    "wasm": package / ("rspice-ui-worker_bg.wasm.gz" if compressed else "rspice-ui-worker_bg.wasm"),
                })
                for path in (worker, *assets.values()):
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_bytes(b"fixture")
                relative = worker.relative_to(root)
                self.assertEqual(self.gate.qualification_worker(root, str(relative)), relative)
                assets["wasm"].unlink()
                with self.assertRaisesRegex(ValueError, "qualification is missing"):
                    self.gate.qualification_worker(root, str(relative))
            with self.assertRaisesRegex(ValueError, "must stay inside"):
                self.gate.qualification_worker(root, "../simulation-worker.js")

    def test_only_finite_positive_measurements_can_qualify(self) -> None:
        for reported in (None, "", "invalid", "0", "-1", "NaN", "inf", "-inf"):
            with self.subTest(reported=reported), self.assertRaises(SystemExit):
                self.gate.read_nanoseconds_per_stamp(reported)
        self.assertEqual(self.gate.read_nanoseconds_per_stamp("123.5"), 123.5)


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
            ".github/workflows/security.yml",
            ".github/workflows/coverage.yml",
            ".github/workflows/native-release.yml",
        ]

        for workflow_path in read_only_workflows:
            with self.subTest(workflow=workflow_path):
                workflow = read_text(workflow_path)
                self.assertRegex(workflow, r"(?m)^permissions:\n  contents: read\n")

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
            "rspice-app",
            "rspice-app-state",
            "rspice-app-services",
            "rspice-app-runs",
            "rspice-app-workbench",
            "rspice-app-sheets",
            "rspice-app-settings",
            "rspice-app-results",
            "rspice-app-hardcopy",
            "rspice-app-cloud",
            "rspice-app-models",
            "rspice-app-simulate",
            "rspice-core",
            "rspice-core-base",
            "rspice-core-netlist",
            "rspice-core-devices",
            "rspice-publication-contract",
            "rspice-fonts",
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

    def test_conformance_crate_forwards_core_features_it_branches_on(self) -> None:
        """Every `feature = "x"` the suites test must be declared here too.

        The suites branch on rspice-core features to decide whether a deck
        should run or be marked unsupported. An unforwarded feature silently
        reads `false` in this crate, so the suite takes the "unsupported"
        branch and passes vacuously even when the models were compiled in —
        a conformance suite quietly skipping cases is the one outcome it
        must never have.
        """
        manifest = read_text("tools/rspice-conformance/Cargo.toml")
        declared = set(re.findall(r"(?m)^([a-z0-9-]+) = \[", manifest))

        # Only real gates count. A bare `feature = "..."` also occurs in prose
        # inside doc comments, so require it to sit inside a `cfg(...)`.
        gate = re.compile(r'cfg!?\([^)]*feature = "([a-z0-9-]+)"')
        sources = sorted((ROOT / "tools" / "rspice-conformance").rglob("*.rs"))
        self.assertTrue(sources, "the scan read no rspice-conformance source")
        used = set()
        for path in sources:
            used.update(gate.findall(path.read_text(encoding="utf-8", errors="ignore")))

        missing = sorted(used - declared)
        self.assertEqual(
            missing,
            [],
            "rspice-conformance branches on undeclared features (they read false): "
            + ", ".join(missing),
        )

    def test_all_ci_and_release_python_harnesses_are_gated(self) -> None:
        workflow = read_text(".github/workflows/ci.yml")
        for directory in ("tools/ci", "tools/release"):
            for path in (ROOT / directory).glob("test_*.py"):
                self.assertIn(path.relative_to(ROOT).as_posix(), workflow)

    def test_headless_chrome_profiles_survive_a_cleanup_race(self) -> None:
        """Deleting a scratch profile does not get to decide a browser check.

        Chromium's children outlive the process the harness terminates, and a
        late write into the profile makes its removal fail with ENOTEMPTY. That
        failed the Monte Carlo checkpoint on a run whose worker had already
        returned a passing verdict, and took the whole browser job with it.
        """
        scanned, offenders = [], []
        for path in sorted((ROOT / "tools" / "ci").glob("*.py")):
            # Suites name the flag to talk about it; only harnesses pass it.
            if path.name.startswith("test_"):
                continue
            source = path.read_text(encoding="utf-8")
            if "--user-data-dir" not in source:
                continue
            scanned.append(path.name)
            for node in ast.walk(ast.parse(source)):
                if not isinstance(node, ast.Call):
                    continue
                if ast.unparse(node.func) != "tempfile.TemporaryDirectory":
                    continue
                if not any(
                    keyword.arg == "ignore_cleanup_errors"
                    and isinstance(keyword.value, ast.Constant)
                    and keyword.value.value is True
                    for keyword in node.keywords
                ):
                    offenders.append(f"{path.name}:{node.lineno}")

        self.assertLessEqual(
            {
                "browser_workbench.py",
                "check_browser_monte_carlo_checkpoint.py",
                "check_wasm_jit_browser.py",
            },
            set(scanned),
            "a headless-Chrome harness was renamed; this guard no longer reads it",
        )
        self.assertEqual(
            [],
            offenders,
            "a temporary directory handed to Chromium as --user-data-dir must be "
            "built with ignore_cleanup_errors=True",
        )

    def test_veriloga_gated_core_suites_run_on_both_routes(self) -> None:
        """A `veriloga`-gated core suite cannot be added without a lane.

        `cargo test -p rspice-core` resolves the default feature set, under
        which every one of these files compiles to an empty test binary. The
        core lane therefore linked and ran nothing for any of them, and the
        only lane that reached them was nightly's `--workspace` run. Roughly
        nine thousand lines of mixed-signal engine code landed with these
        suites never executed on a pull request, and `connect_module_insertion`
        was already red on `main` when this gate was written.

        The list of files is scanned out of the test directory rather than
        written down here, because a hand-maintained list is the failure being
        fixed: a new gated file would join it only if someone remembered. Each
        route gets the whole set -- the bytecode interpreter and the x64 JIT
        are separate lowerings of the same modules -- minus the explicit
        exclusions above, each of which must still be a gated file, so an
        exclusion for a file that no longer exists fails instead of rotting.
        A step names targets; a target folder brings every file it declares,
        so the files a route runs are exactly the gated set plus the ungated
        files listed for it, and no excluded file rides along in a folder.
        """
        import shlex

        workflow = read_text(".github/workflows/ci.yml")
        files = core_test_files()
        gated = veriloga_gated_core_test_files()

        # A scan that silently collapses to nothing would pass every assertion
        # below, so it must find the suites known to be gated.
        self.assertIn("connect_module_insertion", gated)
        self.assertIn("transient_checkpoint", gated)
        self.assertIn("generated_route_parity", gated)

        for name, reason in VERILOGA_GATED_CORE_TEST_EXCLUSIONS.items():
            self.assertIn(
                name,
                gated,
                f"{name} is excluded but is no longer a Verilog-A-gated file",
            )
            self.assertTrue(reason.strip(), f"{name} needs a written reason")
        gated_expected = gated - set(VERILOGA_GATED_CORE_TEST_EXCLUSIONS)

        for job_name, feature, step_names in VERILOGA_GATED_CORE_TEST_STEPS:
            extras = VERILOGA_ROUTE_UNGATED_CORE_TEST_FILES[feature]
            for name, reason in extras.items():
                with self.subTest(route=feature, extra=name):
                    self.assertIn(
                        name,
                        files,
                        f"{name} is run on {feature} but is no longer a core test file",
                    )
                    self.assertNotIn(
                        name,
                        gated,
                        f"{name} is gated now; list it as a gated target instead",
                    )
                    self.assertTrue(reason.strip(), f"{name} needs a written reason")
            expected = gated_expected | set(extras)
            job = workflow_job_body(workflow, job_name)
            covered: list[str] = []
            for step_name in step_names:
                with self.subTest(step=step_name):
                    step = workflow_step_body(job, step_name)
                    command = shlex.split(folded_run_command(step))

                    self.assertEqual(
                        command[:5],
                        ["cargo", "test", "--locked", "-p", "rspice-core"],
                    )
                    features = command[command.index("--features") + 1].split(",")
                    self.assertIn(
                        feature,
                        features,
                        f"{step_name} must select {feature}, got {features}",
                    )
                    # One broken suite must not hide the findings of the
                    # independent ones behind it.
                    self.assertIn("--no-fail-fast", command)
                    covered.extend(
                        command[index + 1]
                        for index, word in enumerate(command)
                        if word == "--test"
                    )

            with self.subTest(route=feature):
                self.assertEqual(
                    len(covered),
                    len(set(covered)),
                    f"{job_name} compiles a target twice: {sorted(covered)}",
                )
                targets = {target for target, _ in files.values()}
                self.assertEqual(
                    set(covered) - targets,
                    set(),
                    f"{job_name} names targets rspice-core does not build",
                )
                run = {name for name, (target, _) in files.items() if target in covered}
                self.assertEqual(
                    run,
                    expected,
                    f"{job_name} does not run every Verilog-A-gated core "
                    f"suite; missing={sorted(expected - run)}, "
                    f"unexpected={sorted(run - expected)}",
                )

    def test_contract_gated_veriloga_suites_have_an_executing_lane(self) -> None:
        """A suite that exists only under a feature needs a step naming it.

        `native-bytecode-contract-tests` reaches constructors production native
        callers cannot use, so these targets link in no default build and no
        `--all-targets` lint reaches them either. `route_parity` -- which holds
        the bytecode and x64 lowerings of one model to each other -- landed with
        no lane at all, which is the same shape of hole as the `veriloga`-gated
        core suites above.
        """
        import shlex

        workflow = read_text(".github/workflows/ci.yml")
        gated = contract_gated_veriloga_test_targets()
        self.assertIn("route_parity", gated, "the contract-gated target scan found nothing")

        named: set[str] = set()
        for line in workflow.splitlines():
            stripped = line.strip()
            if "cargo test" not in stripped or "-p rspice-veriloga" not in stripped:
                continue
            text = stripped.split("run:", 1)[1] if stripped.startswith("run:") else stripped
            try:
                command = shlex.split(text)
            except ValueError:
                continue
            if "--features" not in command:
                continue
            if "native-bytecode-contract-tests" not in command[
                command.index("--features") + 1
            ].split(","):
                continue
            named.update(
                command[index + 1]
                for index, word in enumerate(command)
                if word == "--test"
            )

        self.assertEqual(
            gated - named,
            set(),
            f"contract-gated suites that no step runs: {sorted(gated - named)}",
        )

    def test_model_gated_core_library_units_have_an_executing_build(self) -> None:
        """A core-library unit gated on a generated model runs somewhere.

        The generated-route suites name integration targets, which leave the
        library out, so no job ran these units: VBIC13's static-DAE probe test
        stood red for six weeks unseen, and R2_CMC's task test never
        compiled under its own feature because it leaned on imports only
        DIODE_CMC's tests brought in. Per push, every model the device
        library's units name gets a build carrying that model alone, read from
        the source so a new gate cannot arrive without one.
        """
        model_gate = re.compile(r'feature\s*=\s*"(veriloga-model-[a-z0-9-]+)"')
        device_models = {
            model
            for path in sorted((ROOT / "crates/rspice-core-devices/src").rglob("*.rs"))
            for predicate in cfg_predicates(path.read_text(encoding="utf-8"))
            for model in model_gate.findall(predicate)
        }
        self.assertIn("veriloga-model-vbic13", device_models, "the gate scan found nothing")

        per_push = workflow_step_body(
            workflow_job_body(read_text(".github/workflows/ci.yml"), "generated-veriloga"),
            "Test the device library against each model its units name",
        )
        builds = [
            line.strip()
            for line in per_push.splitlines()
            if line.strip().startswith("cargo test")
        ]
        self.assertEqual(
            builds,
            [
                f"cargo test --locked -p rspice-core-devices --lib --features {model}"
                for model in sorted(device_models)
            ],
            "one build per model, carrying that model alone",
        )

    def test_macos_hardened_jit_uses_only_narrow_execution_entitlements(self) -> None:
        import plistlib

        entitlement_path = ROOT / "crates/rspice-app/macos/RSpice.entitlements"
        with entitlement_path.open("rb") as source:
            entitlements = plistlib.load(source)
        self.assertEqual(
            entitlements,
            {
                "com.apple.security.cs.allow-jit": True,
                "com.apple.security.cs.jit-write-allowlist": True,
            },
        )

    def test_ui_enables_accessibility_backend_for_each_runtime(self) -> None:
        manifest = read_text("crates/rspice-app/Cargo.toml")

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

        app = read_text("crates/rspice-app/src/workbench/app.rs")
        self.assertIn(
            "options.screen_reader = self.state.settings.browser_spoken_feedback",
            app,
        )

    def test_custom_ui_interactions_have_semantics_and_visible_focus(self) -> None:
        """Painter-backed controls must not bypass egui's accessibility contract."""
        click_pattern = re.compile(
            r"(?<![A-Za-z0-9_])(?:egui::)?Sense::(?:click|click_and_drag)\(\)"
        )
        offenders = []

        for path in sorted(
            path
            for crate in (
                "rspice-app",
                "rspice-app-state",
                "rspice-app-services",
                "rspice-app-runs",
                "rspice-app-workbench",
                "rspice-app-sheets",
                "rspice-app-settings",
                "rspice-app-results",
                "rspice-app-hardcopy",
                "rspice-app-cloud",
                "rspice-app-models",
                "rspice-app-simulate",
            )
            for path in (ROOT / "crates" / crate / "src").rglob("*.rs")
        ):
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
