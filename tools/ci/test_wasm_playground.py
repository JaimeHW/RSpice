import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
PLAYGROUND_DIRS = [
    ROOT / "crates" / "rspice-wasm" / "web",
    ROOT / "site" / "play",
]


class WasmPlaygroundWorkerTests(unittest.TestCase):
    def test_checked_in_playground_workers_stay_in_sync(self) -> None:
        canonical = (PLAYGROUND_DIRS[0] / "engine-worker.js").read_text(encoding="utf-8")
        deployed = (PLAYGROUND_DIRS[1] / "engine-worker.js").read_text(encoding="utf-8")
        self.assertEqual(
            deployed,
            canonical,
            "site/play worker must stay byte-identical to crates/rspice-wasm/web worker",
        )

    def test_playgrounds_run_engine_calls_in_module_worker(self) -> None:
        for playground in PLAYGROUND_DIRS:
            with self.subTest(playground=playground.relative_to(ROOT)):
                index = (playground / "index.html").read_text(encoding="utf-8")
                worker_path = playground / "engine-worker.js"

                self.assertTrue(worker_path.exists(), "missing engine-worker.js")
                worker = worker_path.read_text(encoding="utf-8")

                self.assertIn('new Worker(new URL("./engine-worker.js", import.meta.url)', index)
                self.assertIn('type: "module"', index)
                self.assertNotRegex(
                    index,
                    re.compile(r"import\s+init,\s*\{[^}]*runTransientAnalysis", re.S),
                    "main page must not import synchronous engine solve exports",
                )
                self.assertNotIn(
                    "production app will move solves into a Web Worker",
                    index,
                )
                self.assertIn("dedicated Web Worker", index)

                self.assertRegex(
                    worker,
                    re.compile(
                        r"import\s+init,\s*\{[^}]*summarizeNetlist"
                        r"[^}]*runDcOperatingPoint[^}]*runAcAnalysis"
                        r"[^}]*runTransientAnalysis",
                        re.S,
                    ),
                )
                self.assertIn('case "ac":', worker)
                self.assertIn("runAcAnalysis(payload.source, payload.frequencies)", worker)
                self.assertIn('postMessage({ type: "ready" })', worker)
                self.assertRegex(worker, re.compile(r'postMessage\(\{\s*type: "result"', re.S))
                self.assertRegex(worker, re.compile(r'postMessage\(\{\s*type: "error"', re.S))
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
                    "requests arriving before ready must share the wasm init() promise",
                )

    def test_playgrounds_expose_ac_analysis_controls(self) -> None:
        for playground in PLAYGROUND_DIRS:
            with self.subTest(playground=playground.relative_to(ROOT)):
                index = (playground / "index.html").read_text(encoding="utf-8")

                self.assertIn('id="btn-ac"', index)
                self.assertIn('id="fstart"', index)
                self.assertIn('id="fstop"', index)
                self.assertIn('id="fpoints"', index)
                self.assertIn('function acFrequencies()', index)
                self.assertIn('runEngine("ac"', index)
                self.assertIn('function doAc()', index)
                self.assertIn('drawAcPlot(response.result)', index)
                self.assertNotIn(
                    "frequencies.map(f => Math.log10(f))",
                    index,
                    "AC plots must render Hz/dB axes instead of reusing the transient seconds axis",
                )
                self.assertIn('textContent = fmtEng(f) + "Hz"', index)
                self.assertIn('textContent = db.toFixed(0)', index)
                self.assertIn('if (a >= 1e9) { return (v / 1e9).toPrecision(4) + "g"; }', index)
                self.assertIn('if (a >= 1e6) { return (v / 1e6).toPrecision(4) + "meg"; }', index)
                self.assertIn('if (a >= 1e3) { return (v / 1e3).toPrecision(4) + "k"; }', index)
                self.assertIn("const MAX_TRAN_POINTS = 200000", index)
                self.assertIn("const MAX_RENDERED_PLOT_POINTS =", index)
                self.assertIn("function decimatePlotSeries(time, values)", index)
                self.assertIn("const plotted = decimatePlotSeries(time, s.values)", index)
                self.assertNotIn(
                    "for (let i = 0; i < time.length; i++) {\n"
                    "      d += (i ? \"L\" : \"M\")",
                    index,
                    "transient SVG paths must be decimated before rendering large worker results",
                )
                self.assertIn("Math.ceil(tstop / hmax) + 1", index)
                self.assertIn("estimatedPoints > MAX_TRAN_POINTS", index)
                self.assertIn("transient request would generate about", index)
                self.assertIn("function handleWorkerFatalError(message)", index)
                self.assertIn("function handleWorkerStartupError(error)", index)
                self.assertIn("function startEngineWorker(statusText)", index)
                self.assertIn("function recreateWorkerAfterFatalError()", index)
                self.assertIn("let worker = null", index)
                self.assertIn('startEngineWorker("loading worker")', index)
                self.assertIn('message.type === "error" && message.id === 0', index)
                self.assertIn("pending.clear()", index)
                self.assertNotIn(
                    "let worker = createEngineWorker();",
                    index,
                    "worker constructor failures must be caught before startup can abort the page",
                )
                self.assertRegex(
                    index,
                    re.compile(
                        r"function\s+startEngineWorker\s*\(statusText\)\s*\{"
                        r".*try\s*\{"
                        r".*worker\s*=\s*createEngineWorker\(\)"
                        r".*\}\s*catch\s*\(error\)\s*\{"
                        r".*handleWorkerStartupError\(error\)",
                        re.S,
                    ),
                    "module Worker construction must be guarded by try/catch",
                )
                self.assertRegex(
                    index,
                    re.compile(
                        r'(?:worker|nextWorker)\.addEventListener\("error",\s*\(event\)\s*=>\s*\{'
                        r".*handleWorkerFatalError\(\{\s*error:\s*event\.message",
                        re.S,
                    ),
                    "worker error events must reject and clear pending requests",
                )
                self.assertRegex(
                    index,
                    re.compile(
                        r'(?:worker|nextWorker)\.addEventListener\("messageerror",\s*\(\)\s*=>\s*\{'
                        r".*handleWorkerFatalError\(\{\s*error:",
                        re.S,
                    ),
                    "worker messageerror events must reject and clear pending requests",
                )

    def test_playground_readme_matches_current_browser_surfaces(self) -> None:
        readme = (ROOT / "crates" / "rspice-wasm" / "web" / "README.md").read_text(
            encoding="utf-8"
        )

        self.assertNotIn("full WASM IDE", readme)
        self.assertNotIn("later milestone", readme)
        self.assertIn("experimental browser IDE", readme)
        self.assertIn("**Run .ac**", readme)


if __name__ == "__main__":
    unittest.main()
