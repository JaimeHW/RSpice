import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
IDE_DIRS = [
    ROOT / "crates" / "rspice-ui" / "web",
    ROOT / "site" / "ide",
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


class IdeWorkerRoutingTests(unittest.TestCase):
    def test_checked_in_ide_workers_stay_in_sync(self) -> None:
        canonical = (IDE_DIRS[0] / "simulation-worker.js").read_text(encoding="utf-8")
        deployed = (IDE_DIRS[1] / "simulation-worker.js").read_text(encoding="utf-8")
        self.assertEqual(
            deployed,
            canonical,
            "site/ide worker must stay byte-identical to crates/rspice-ui/web worker",
        )

    def test_browser_ide_constructs_a_simulation_worker(self) -> None:
        for ide in IDE_DIRS:
            with self.subTest(ide=ide.relative_to(ROOT)):
                index = (ide / "index.html").read_text(encoding="utf-8")
                worker_path = ide / "simulation-worker.js"

                self.assertTrue(worker_path.exists(), "missing IDE simulation worker")
                self.assertRegex(
                    index,
                    re.compile(
                        r"new\s+Worker\s*\(\s*new\s+URL\("
                        r"\"\.\/simulation-worker\.js\",\s*import\.meta\.url\)",
                        re.S,
                    ),
                )
                self.assertIn('type: "module"', index)
                self.assertIn("__RSPICE_SIM_WORKER", index)

                worker = worker_path.read_text(encoding="utf-8")
                self.assertIn('from "./pkg/rspice-ui.js"', worker)
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
                        r".*initPromise\s*=\s*init\(\)"
                        r".*\}"
                        r".*await\s+initPromise",
                        re.S,
                    ),
                    "requests arriving before ready must share the eager init() promise",
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
