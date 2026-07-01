import importlib.util
import contextlib
import io
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
BUILD_SITE_PATH = ROOT / "tools" / "deploy" / "build_site.py"
SPEC = importlib.util.spec_from_file_location("build_site", BUILD_SITE_PATH)
build_site = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(build_site)


def assert_gate_fails(testcase: unittest.TestCase, func, *args) -> None:
    with contextlib.redirect_stderr(io.StringIO()):
        with testcase.assertRaises(SystemExit):
            func(*args)


class BuildSiteGateTests(unittest.TestCase):
    def test_headless_chrome_args_keep_webgpu_available(self) -> None:
        args = build_site.chrome_headless_args("chrome", "http://127.0.0.1:8000/ide/")

        self.assertNotIn("--disable-gpu", args)
        self.assertIn("--enable-unsafe-webgpu", args)
        self.assertIn("--ignore-gpu-blocklist", args)
        self.assertIn("--enable-features=Vulkan", args)
        self.assertIn("--use-vulkan=swiftshader", args)
        self.assertIn("--dump-dom", args)

    def test_ide_gate_requires_startup_error_lifecycle_hooks(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp)
            ide = out / "ide"
            ide.mkdir()
            (ide / "index.html").write_text(
                """
                <script type="module">
                window.__RSPICE_SIM_WORKER = new Worker(
                  new URL("./simulation-worker.js", import.meta.url),
                  { type: "module" },
                );
                </script>
                """,
                encoding="utf-8",
            )
            (ide / "simulation-worker.js").write_text(
                """
                import init, { runRspiceUiWorkerRequest } from "./pkg/rspice-ui.js";
                postMessage({ type: "ready" });
                postMessage({ type: "result" });
                postMessage({ type: "error" });
                """,
                encoding="utf-8",
            )

            assert_gate_fails(self, build_site.gate_ide_worker, out)

    def test_playground_gate_requires_ac_worker_route(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp)
            play = out / "play"
            play.mkdir()
            (play / "engine-worker.js").write_text(
                """
                import init, {
                  summarizeNetlist,
                  runDcOperatingPoint,
                  runTransientAnalysis,
                } from "./pkg/rspice_wasm.js";
                switch (operation) {
                  case "summary": break;
                  case "op": break;
                  case "tran": break;
                }
                """,
                encoding="utf-8",
            )

            assert_gate_fails(self, build_site.gate_playground_worker, out)

    def test_headless_dom_gate_requires_playground_solve_markers(self) -> None:
        assert_gate_fails(self, build_site.validate_playground_dom, "<html></html>")

        build_site.validate_playground_dom(
            '<html><body><span>worker ready</span><p>solved in 0.01 ms</p></body></html>'
        )

    def test_headless_dom_gate_rejects_broken_ide_route(self) -> None:
        assert_gate_fails(self, build_site.validate_ide_dom, "<html></html>")
        assert_gate_fails(
            self,
            build_site.validate_ide_dom,
            '<canvas id="rspice_canvas"></canvas><p class="err">module failed</p>',
        )

        build_site.validate_ide_dom(
            '<html><body><canvas id="rspice_canvas"></canvas></body></html>'
        )

    def test_headless_dom_gate_requires_ide_worker_smoke_solve(self) -> None:
        assert_gate_fails(self, build_site.validate_ide_worker_smoke_dom, "<html></html>")
        assert_gate_fails(
            self,
            build_site.validate_ide_worker_smoke_dom,
            '<pre id="ide-worker-smoke">ide worker error: boom</pre>',
        )

        build_site.validate_ide_worker_smoke_dom(
            '<html><body><pre id="ide-worker-smoke">ide worker solved</pre></body></html>'
        )

    def test_source_worker_contract_gate_requires_progress_bridge(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            runner_dir = root / "crates" / "rspice-ui" / "src" / "simulation" / "runner"
            runner_dir.mkdir(parents=True)
            (runner_dir / "worker_contract.rs").write_text(
                "pub(crate) struct WorkerResponse;\n",
                encoding="utf-8",
            )
            (runner_dir / "wasm_worker.rs").write_text(
                'match message_type.as_str() { "result" => handle_result_message(), _ => {} }\n',
                encoding="utf-8",
            )

            assert_gate_fails(self, build_site.gate_ide_worker_sources, root)


if __name__ == "__main__":
    unittest.main()
