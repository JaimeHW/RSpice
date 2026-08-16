import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
IDE_DIRS = [
    ROOT / "crates" / "rspice-ui" / "web",
]
UI_SRC = ROOT / "crates" / "rspice-ui" / "src"
RUNNER = UI_SRC / "simulation" / "runner.rs"
WASM_WORKER = UI_SRC / "simulation" / "runner" / "wasm_worker.rs"
WORKER_CONTRACT = UI_SRC / "simulation" / "runner" / "worker_contract.rs"
MAIN = UI_SRC / "main.rs"
APP = UI_SRC / "workbench" / "app.rs"
HARDCOPY_WORKER = UI_SRC / "workbench" / "app" / "dialogs" / "hardcopy" / "worker.rs"
BROWSER_ACCESSIBILITY = UI_SRC / "workbench" / "browser" / "accessibility.rs"

# These tests assert architectural properties that live in specific files, so
# the paths above are part of the contract rather than an implementation
# detail. When rspice-ui moves one, the failure should say so plainly instead
# of surfacing as a FileNotFoundError traceback from whichever test read it
# first.
_SOURCES = {
    "RUNNER": RUNNER,
    "WASM_WORKER": WASM_WORKER,
    "WORKER_CONTRACT": WORKER_CONTRACT,
    "MAIN": MAIN,
    "APP": APP,
    "HARDCOPY_WORKER": HARDCOPY_WORKER,
    "BROWSER_ACCESSIBILITY": BROWSER_ACCESSIBILITY,
}


def read_module(module_file: Path) -> str:
    """Read a Rust module together with its submodules, as one unit.

    `worker_contract` began as a single file and has since split into
    `analysis`, `conversions`, and `transport`. The assertions below are about
    what the contract provides, not about which file inside it happens to hold
    a given symbol, so an internal reorganization must not break them.

    Test modules are excluded on purpose: a symbol that appears only in a test
    does not satisfy a claim that the contract provides it.
    """
    parts = [module_file.read_text(encoding="utf-8")]
    submodules = module_file.with_suffix("")
    if submodules.is_dir():
        for child in sorted(submodules.rglob("*.rs")):
            if child.stem == "tests" or child.stem.startswith("test_"):
                continue
            parts.append(child.read_text(encoding="utf-8"))
    return "\n".join(parts)


def setUpModule() -> None:
    """Fail once, clearly, when a pinned source moves.

    Deliberately an error rather than a skip: skipping would let CI stay green
    while these checks quietly stopped running, which is the failure mode they
    exist to prevent.
    """
    def describe(path: Path) -> str:
        try:
            return path.relative_to(ROOT).as_posix()
        except ValueError:
            # A path pointing outside the repo is itself the bug; say so
            # rather than dying in the formatting of the error message.
            return f"{path} (outside the repository)"

    missing = [
        f"  {name} -> {describe(path)}"
        for name, path in _SOURCES.items()
        if not path.is_file()
    ]
    if missing:
        raise RuntimeError(
            "rspice-ui sources these checks pin no longer exist:\n"
            + "\n".join(missing)
            + "\nThey were moved, not deleted, the last few times this fired. "
            "Find their new homes and update the paths at the top of this file "
            "— do not delete the assertions."
        )


class IdeWorkerRoutingTests(unittest.TestCase):
    def test_browser_spoken_feedback_has_a_dom_bootstrap_and_rust_bridge(self) -> None:
        for ide in IDE_DIRS:
            with self.subTest(ide=ide.relative_to(ROOT)):
                index = (ide / "index.html").read_text(encoding="utf-8")
                self.assertIn('id="rspice_spoken_feedback"', index)
                self.assertIn('aria-pressed="false"', index)
                # The claim is that the canvas element carries an accessible
                # name, not that `aria-label` is the attribute written
                # immediately after the id. `[^>]*` keeps the assertion scoped
                # to the same start tag while allowing the role, tabindex and
                # aria-describedby the canvas also declares.
                self.assertRegex(
                    index,
                    re.compile(r'id="rspice_canvas"[^>]*\saria-label=', re.S),
                )
                self.assertIn(
                    'id="rspice_loading" role="status" aria-live="polite"',
                    index,
                )
                self.assertIn('el.setAttribute("role", "alert")', index)
                self.assertIn("data-rspice-spoken-feedback", index)
                self.assertIn("rspice.web.spoken-feedback", index)
                self.assertIn("new MutationObserver", index)
                self.assertIn('new Event("resize")', index)
                self.assertNotIn("window.location.reload", index)

        bridge = BROWSER_ACCESSIBILITY.read_text(encoding="utf-8")
        app = APP.read_text(encoding="utf-8")
        self.assertIn("spoken_feedback_override", bridge)
        self.assertIn("set_spoken_feedback", bridge)
        self.assertIn("spoken_feedback_override()", app)
        main = MAIN.read_text(encoding="utf-8")
        self.assertIn('loading.set_attribute("role", "alert")', main)
        self.assertIn("loading.set_text_content(None)", main)
        self.assertNotIn('loading.set_inner_html("")', main)

    def test_browser_ide_constructs_a_simulation_worker(self) -> None:
        for ide in IDE_DIRS:
            with self.subTest(ide=ide.relative_to(ROOT)):
                index = (ide / "index.html").read_text(encoding="utf-8")
                worker_path = ide / "simulation-worker.js"

                self.assertTrue(worker_path.exists(), "missing IDE simulation worker")
                self.assertIn(
                    'const workerUrl = executableAsset("simulation-worker.js")',
                    index,
                )
                self.assertIn(
                    "window.__RSPICE_SIM_WORKER_URL = workerUrl.href", index
                )
                self.assertRegex(
                    index,
                    re.compile(r"new\s+Worker\(\s*workerUrl\s*,", re.S),
                )
                self.assertIn('type: "module"', index)
                self.assertIn("__RSPICE_SIM_WORKER", index)
                self.assertIn("data-rspice-wasm-jit-status", index)
                self.assertIn('capability.available ? "qualified" : "rejected"', index)
                self.assertIn("data-rspice-wasm-jit-solver-result", index)

                worker = worker_path.read_text(encoding="utf-8")
                self.assertIn(
                    'import(executableAsset("rspice-ui-worker.js").href)', worker
                )
                self.assertIn(
                    'executableAsset("rspice-ui-worker_bg.wasm")', worker
                )
                self.assertIn("runRspiceUiWorkerRequest", worker)
                self.assertRegex(
                    worker, re.compile(r'postMessage\(\{\s*type: "ready"', re.S)
                )
                self.assertIn("function responseTransferList(response)", worker)
                self.assertIn("const transferBuffers = new Set()", worker)
                self.assertIn("ArrayBuffer.isView(view)", worker)
                self.assertIn("view.buffer instanceof ArrayBuffer", worker)
                self.assertIn("transferBuffers.add(view.buffer)", worker)
                self.assertIn("responseTransferList(response)", worker)
                self.assertIn("runRspiceUiHardcopyRequest", worker)
                self.assertIn("runRspiceUiModelImportRequest", worker)
                self.assertIn('message.type === "run-hardcopy"', worker)
                self.assertIn('type: "hardcopy-result"', worker)
                self.assertIn('type: "hardcopy-error"', worker)
                self.assertIn(
                    "hardcopyResponseTransferList(response)",
                    worker,
                )
                self.assertNotIn("id: message.request.id", worker)
                self.assertNotIn("id: message.request?.id", worker)
                self.assertRegex(
                    worker,
                    re.compile(r'\{\s*type:\s*"result",\s*id:\s*message\.id', re.S),
                    "outer result id must echo the top-level routing id",
                )
                self.assertRegex(
                    worker,
                    re.compile(r'\{\s*type:\s*"error",\s*id:\s*message\.id\s*\?\?', re.S),
                    "outer error id must echo the top-level routing id",
                )
                self.assertRegex(
                    worker,
                    re.compile(
                        r'postMessage\(\s*\{\s*type:\s*"result".*?response\s*\}'
                        r"\s*,\s*responseTransferList\(response\)\s*,?\s*\)",
                        re.S,
                    ),
                    "result messages must transfer typed-array buffers instead of structured-cloning them",
                )
                self.assertRegex(
                    worker, re.compile(r'postMessage\(\{\s*type: "error"', re.S)
                )

    def test_worker_init_is_singleflight_and_app_startup_is_gated(self) -> None:
        for ide in IDE_DIRS:
            with self.subTest(ide=ide.relative_to(ROOT)):
                worker = (ide / "simulation-worker.js").read_text(encoding="utf-8")
                self.assertNotIn(
                    "let ready = false",
                    worker,
                    "worker readiness must be represented by a shared init promise",
                )
                self.assertIn("let initPromise = null", worker)
                self.assertRegex(
                    worker,
                    re.compile(
                        r"initializeWorkerModule\(\)\.catch\(\(error\) => \{"
                        r".*runWorkerRequest\s*=\s*null;"
                        r".*runVerilogACompileRequest\s*=\s*null;"
                        r".*runHardcopyRequest\s*=\s*null;",
                        re.S,
                    ),
                    "a failed module initialization must clear every cached wasm executor",
                )
                self.assertRegex(
                    worker,
                    re.compile(
                        r"async\s+function\s+ensureReady\s*\(\)\s*\{"
                        r".*if\s*\(!initPromise\)\s*\{"
                        r".*initPromise\s*=\s*initializeWorkerModule\(\)"
                        r".*\}"
                        r".*await\s+initPromise",
                        re.S,
                    ),
                    "requests arriving before ready must share the eager init() promise",
                )

    def test_worker_installs_verified_wasm_jit_models_with_bounded_cache(self) -> None:
        worker = (IDE_DIRS[0] / "simulation-worker.js").read_text(encoding="utf-8")
        self.assertIn('"rspiceUiWasmJitEmitterVersion"', worker)
        # The capabilities are raw WebAssembly exports, not wasm-bindgen ones,
        # so they are checked on wasmExports and must not be looked for among
        # the generated JavaScript functions.
        self.assertNotIn("rspiceUiWasmJitEvalOpV1", worker)
        self.assertNotIn("rspiceUiWasmJitMath1V1", worker)
        self.assertIn('"rspiceUiWasmJitSolverProbeArtifact"', worker)
        self.assertIn('"rspiceUiWasmJitRunSolverProbe"', worker)
        self.assertIn('"rspiceUiWasmJitKernelProbeArtifact"', worker)
        self.assertIn('"rspiceUiWasmJitRunKernelProbe"', worker)
        self.assertIn("await installWasmJitArtifact(module, solverArtifact)", worker)
        self.assertIn("await installWasmJitArtifact(module, kernelArtifact)", worker)
        self.assertIn("module.rspiceUiWasmJitRunKernelProbe()", worker)
        self.assertIn("async function installWasmJitModel(module, response)", worker)
        self.assertIn("async function installWasmJitArtifact(module, artifact)", worker)
        self.assertIn("async function prepareWasmJitSimulationRequest(module, request)", worker)
        self.assertIn("module.prepareRspiceUiWasmJitRequest(request)", worker)
        self.assertIn("workerModule.runPreparedRspiceUiWasmJitRequest(dispatchToken)", worker)
        self.assertIn("module.cancelPreparedRspiceUiWasmJitRequest(candidateToken)", worker)
        self.assertIn("function dispatchWasmJitEntry(cacheKey, exportName, frameOffset)", worker)
        self.assertIn("module.installRspiceUiWasmJitDispatcher(dispatchWasmJitEntry)", worker)
        self.assertIn("WebAssembly.compile(bytes)", worker)
        # Every generated module must be instantiated against the primary
        # module's raw exports. Binding the wasm-bindgen wrappers instead puts
        # a JavaScript frame between a model's exp() and its implementation on
        # a path that runs thousands of times per device evaluation, and the
        # answers stay correct, so nothing else here would notice.
        self.assertIn("rspice_jit: wasmJitImports(primaryWasmExports)", worker)
        self.assertIn("memory: wasmExports.memory", worker)
        for capability in ("eval_op_v1", "math1_v1", "math2_v1"):
            self.assertIn(
                f"{capability}: wasmExports.rspice_ui_wasm_jit_{capability}",
                worker,
            )
        self.assertNotIn("eval_op_v1: module.rspiceUiWasmJit", worker)
        self.assertIn("artifact.valueExports", worker)
        self.assertIn("instance.exports[artifact.assignmentExport]", worker)
        self.assertIn("instance.exports[artifact.postAssignmentExport]", worker)
        self.assertIn("const WASM_JIT_CACHE_MAX_MODELS = 64", worker)
        self.assertIn("const WASM_JIT_CACHE_MAX_BYTES = 64 * 1024 * 1024", worker)
        self.assertIn("const WASM_JIT_MODEL_MAX_BYTES = 32 * 1024 * 1024", worker)
        self.assertIn("WASM_JIT_IDENTITY.test(artifact.digest", worker)
        self.assertIn("artifact.abiVersion !== module.rspiceUiWasmJitAbiVersion()", worker)
        self.assertIn(
            "artifact.emitterVersion !== module.rspiceUiWasmJitEmitterVersion()",
            worker,
        )
        self.assertIn("uniqueValueExports.size !== artifact.valueExports.length", worker)
        self.assertIn("wasmJitCacheBytes -= oldest.moduleBytes", worker)
        self.assertIn("await installWasmJitModel(workerModule, response)", worker)
        self.assertIn("delete response.wasmJitArtifact", worker)
        self.assertIn("delete response.wasmJitError", worker)
        self.assertNotRegex(
            worker,
            re.compile(
                r'if\s*\(message\.type\s*!==\s*"run"\).*?'
                r"runWorkerRequest\(request\)",
                re.S,
            ),
            "simulation requests must consume the one decoded prepared request instead of copying transfer buffers twice",
        )

        qualification = (IDE_DIRS[0] / "wasm-jit-qualification.html").read_text(
            encoding="utf-8"
        )
        self.assertIn('data-rspice-wasm-jit-status="pending"', qualification)
        self.assertIn("capability.solverResult === 15", qualification)
        self.assertIn("capability.kernel?.contributions === 3", qualification)
        self.assertIn("capability.kernel?.jacobianEntries === 14", qualification)
        self.assertIn("data-rspice-wasm-jit-kernel-ns-per-stamp", qualification)
        self.assertIn('finish("qualified"', qualification)
        self.assertRegex(
            worker,
            re.compile(
                r"initializeWorkerModule\(\)\.catch\(\(error\) => \{"
                r".*wasmJitModelCache\.clear\(\);"
                r".*wasmJitCacheBytes\s*=\s*0;",
                re.S,
            ),
            "worker initialization failure must discard every compiled model and byte charge",
        )

    def test_hardcopy_worker_transport_is_stale_bound_and_transferable(self) -> None:
        worker = (IDE_DIRS[0] / "simulation-worker.js").read_text(encoding="utf-8")
        bridge = HARDCOPY_WORKER.read_text(encoding="utf-8")

        self.assertIn('message.type === "run-hardcopy"', worker)
        self.assertIn('{ type: "hardcopy-result", id: message.id, response }', worker)
        self.assertIn('type: "hardcopy-error"', worker)
        self.assertIn("hardcopyResponseTransferList(response)", worker)
        self.assertIn('"epoch"', bridge)
        self.assertIn('"generation"', bridge)
        self.assertIn('"operation"', bridge)
        self.assertIn("stale outer response id", bridge)
        self.assertIn("stale outer error id", bridge)
        self.assertIn("stale response id", bridge)
        self.assertIn("stale epoch", bridge)
        self.assertIn("stale generation", bridge)
        self.assertIn("returned the wrong operation", bridge)
        self.assertIn("exceeded its bounded execution deadline", bridge)

    def test_browser_executable_assets_share_one_version_identity(self) -> None:
        for ide in IDE_DIRS:
            with self.subTest(ide=ide.relative_to(ROOT)):
                index = (ide / "index.html").read_text(encoding="utf-8")
                worker = (ide / "simulation-worker.js").read_text(encoding="utf-8")
                self.assertIn(
                    'const RELEASE_ASSET_ROOT = "__RSPICE_ASSET_ROOT__"',
                    index,
                )
                for asset in (
                    "simulation-worker.js",
                    "rspice-ui.js",
                    "rspice-ui_bg.wasm",
                ):
                    self.assertIn(f'executableAsset("{asset}")', index)
                self.assertIn(
                    "window.__RSPICE_SIM_WORKER_URL = workerUrl.href", index
                )
                self.assertIn(
                    r"/\/assets\/[0-9a-f]{64}\/simulation-worker\.js$/",
                    worker,
                )
                self.assertIn('searchParams.get("v")', worker)
                self.assertIn(
                    'import(executableAsset("rspice-ui-worker.js").href)', worker
                )
                self.assertIn(
                    'executableAsset("rspice-ui-worker_bg.wasm")', worker
                )

        main = MAIN.read_text(encoding="utf-8")
        self.assertRegex(
            main,
            re.compile(
                r"let\s+Some\(window\)\s*=\s*web_sys::window\(\)\s*else\s*\{\s*return;\s*\};",
                re.S,
            ),
            "loading the wasm package inside the simulation worker must not run app startup",
        )

    def test_worker_recreation_uses_only_the_exact_page_bound_url(self) -> None:
        wasm_worker = WASM_WORKER.read_text(encoding="utf-8")
        self.assertRegex(
            wasm_worker,
            re.compile(
                r"fn\s+create_worker\(\).*?"
                r"let\s+worker_url\s*=\s*global_worker_url\(\)\?;.*?"
                r"new_with_options\(&worker_url,\s*&options\)",
                re.S,
            ),
            "worker recreation must reuse the URL retained by this exact page",
        )
        self.assertIn(
            'JsValue::from_str("__RSPICE_SIM_WORKER_URL")', wasm_worker
        )
        self.assertNotRegex(
            wasm_worker,
            re.compile(r'new_with_options\(\s*"\.?/simulation-worker\.js"'),
        )
        self.assertNotIn('Worker::new("./simulation-worker.js")', wasm_worker)

    def test_wasm_runner_no_longer_solves_inline(self) -> None:
        runner = RUNNER.read_text(encoding="utf-8")
        self.assertNotIn(
            "let result = run_simulation_thread(request, input, progress, abort_flag);",
            runner,
            "wasm32 runner path must not execute the engine inline on the UI thread",
        )
        self.assertIn("wasm_worker::start_worker_request", runner)

    def test_app_repaints_while_simulation_is_running(self) -> None:
        app = APP.read_text(encoding="utf-8")
        self.assertRegex(
            app,
            re.compile(
                r"if\s+self\.state\.simulation\.has_active_execution\(\)\s*\{\s*"
                r"ctx\.request_repaint_after\(std::time::Duration::from_millis\(\d+\)\)",
                re.S,
            ),
            "egui must keep polling browser worker/native progress while a simulation is running",
        )

    def test_worker_progress_messages_cross_to_ui_progress_state(self) -> None:
        contract = read_module(WORKER_CONTRACT)
        wasm_worker = WASM_WORKER.read_text(encoding="utf-8")

        self.assertIn("WorkerProgressSnapshot", contract)
        self.assertIn("emit_worker_progress_snapshot", contract)
        self.assertIn("WorkerResponseTransport", contract)
        self.assertIn("worker_response_transport_value", contract)
        self.assertIn("worker_response_from_value", contract)
        self.assertIn("Float64Array", contract)
        self.assertIn("validate_worker_response_id", contract)
        self.assertIn("WorkerF64Series", contract)
        self.assertRegex(
            contract,
            re.compile(r'JsValue::from_str\("progress"\)', re.S),
            "worker-side wasm must post typed progress messages while the solve runs",
        )

        self.assertIn('"progress" => handle_progress_message', wasm_worker)
        self.assertIn("active_progress", wasm_worker)
        self.assertIn("WorkerProgressSnapshot", wasm_worker)
        self.assertIn("worker_response_from_value", wasm_worker)
        self.assertIn("validate_worker_response_id", wasm_worker)
        self.assertRegex(
            wasm_worker,
            re.compile(r"snapshot\.apply_to\(&mut\s+progress\)", re.S),
            "main-thread worker bridge must apply progress snapshots to SimulationProgress",
        )

    def test_worker_startup_and_fatal_errors_clear_cached_worker(self) -> None:
        wasm_worker = WASM_WORKER.read_text(encoding="utf-8")
        self.assertRegex(
            wasm_worker,
            re.compile(
                r"fn\s+drop_cached_worker.*clear_global_worker\(\)",
                re.S,
            ),
            "fatal worker errors must drop both the handle and global cached worker",
        )
        self.assertRegex(
            wasm_worker,
            re.compile(r"set_onmessageerror\(Some", re.S),
            "messageerror events must be surfaced as worker failures",
        )

        for ide in IDE_DIRS:
            with self.subTest(ide=ide.relative_to(ROOT)):
                index = (ide / "index.html").read_text(encoding="utf-8")
                self.assertIn("try {", index)
                self.assertIn("new Worker", index)
                self.assertIn('addEventListener("error"', index)
                self.assertIn('addEventListener("messageerror"', index)
                self.assertIn("__RSPICE_SIM_WORKER_ERROR", index)
                self.assertIn("function reportSimulationWorkerStartupError(error)", index)
                self.assertIn("showStartupError(message)", index)
                self.assertRegex(
                    index,
                    re.compile(
                        r'addEventListener\("error",\s*\(event\)\s*=>\s*\{'
                        r".*reportSimulationWorkerStartupError",
                        re.S,
                    ),
                    "async worker startup errors must be surfaced visibly",
                )
                self.assertRegex(
                    index,
                    re.compile(
                        r'addEventListener\("messageerror",\s*\(\)\s*=>\s*\{'
                        r".*reportSimulationWorkerStartupError",
                        re.S,
                    ),
                    "worker message deserialization failures must be surfaced visibly",
                )
                self.assertNotIn("innerHTML =", index)


if __name__ == "__main__":
    unittest.main()
