import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
IDE_DIRS = [
    ROOT / "crates" / "rspice-ui" / "web",
]
RUNNER = ROOT / "crates" / "rspice-ui" / "src" / "simulation" / "runner" / "mod.rs"
WASM_WORKER = (
    ROOT / "crates" / "rspice-ui" / "src" / "simulation" / "runner" / "wasm_worker.rs"
)
WORKER_CONTRACT = (
    ROOT / "crates" / "rspice-ui" / "src" / "simulation" / "runner" / "worker_contract.rs"
)
MAIN = ROOT / "crates" / "rspice-ui" / "src" / "main.rs"
APP = ROOT / "crates" / "rspice-ui" / "src" / "common" / "app" / "mod.rs"
BROWSER_ACCESSIBILITY = (
    ROOT
    / "crates"
    / "rspice-ui"
    / "src"
    / "common"
    / "browser_accessibility.rs"
)


class IdeWorkerRoutingTests(unittest.TestCase):
    def test_browser_spoken_feedback_has_a_dom_bootstrap_and_rust_bridge(self) -> None:
        for ide in IDE_DIRS:
            with self.subTest(ide=ide.relative_to(ROOT)):
                index = (ide / "index.html").read_text(encoding="utf-8")
                self.assertIn('id="rspice_spoken_feedback"', index)
                self.assertIn('aria-pressed="false"', index)
                self.assertRegex(
                    index,
                    re.compile(r'id="rspice_canvas"\s+aria-label=', re.S),
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

                worker = worker_path.read_text(encoding="utf-8")
                self.assertIn(
                    'import(executableAsset("rspice-ui.js").href)', worker
                )
                self.assertIn(
                    'executableAsset("rspice-ui_bg.wasm")', worker
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
                        r"async\s+function\s+ensureReady\s*\(\)\s*\{"
                        r".*if\s*\(!initPromise\)\s*\{"
                        r".*initPromise\s*=\s*initializeWorkerModule\(\)"
                        r".*\}"
                        r".*await\s+initPromise",
                        re.S,
                    ),
                    "requests arriving before ready must share the eager init() promise",
                )

    def test_browser_executable_assets_share_one_version_identity(self) -> None:
        assembler = (ROOT / "tools" / "deploy" / "build_site.py").read_text(
            encoding="utf-8"
        )
        self.assertIn(
            'ASSET_ROOT_PLACEHOLDER = "__RSPICE_ASSET_ROOT__"',
            assembler,
        )
        self.assertIn("def executable_asset_identity(asset_root):", assembler)
        self.assertIn("def package_ide_executable_assets(ide):", assembler)
        self.assertIn("require_clean_client_checkout(root)", assembler)
        self.assertIn(
            'ide_asset_identity = package_ide_executable_assets(out / "ide")',
            assembler,
        )

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
                    'import(executableAsset("rspice-ui.js").href)', worker
                )
                self.assertIn(
                    'executableAsset("rspice-ui_bg.wasm")', worker
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
                r"if\s+self\.state\.simulation\.is_running\s*\{\s*"
                r"ctx\.request_repaint_after\(std::time::Duration::from_millis\(\d+\)\)",
                re.S,
            ),
            "egui must keep polling browser worker/native progress while a simulation is running",
        )

    def test_worker_progress_messages_cross_to_ui_progress_state(self) -> None:
        contract = WORKER_CONTRACT.read_text(encoding="utf-8")
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
