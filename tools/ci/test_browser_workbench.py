"""Coordinate and evidence-integrity regressions for real-browser qualification."""

import json
import hashlib
from pathlib import Path
import tempfile
import unittest
import functools
import http.server
import threading
import urllib.request
from unittest.mock import Mock

from browser_workbench import WorkbenchBrowser, controls
from check_browser_workbench import verify_checkpoint, verify_recovery_copy
from check_browser_release import ReleaseHandler, startup_ready, verify_worker_urls
from check_wasm_jit_browser import qualification_worker


class BrowserWorkbenchTests(unittest.TestCase):
    def test_packaged_headers_apply_to_real_and_virtual_browser_pages(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "_headers").write_text("/*\n  Content-Security-Policy: script-src 'self' 'wasm-unsafe-eval'\n/assets/*\n  Cache-Control: public, max-age=3600\n", encoding="utf-8")
            (root / "assets").mkdir()
            (root / "assets/probe.js").write_text("export const probe = true;", encoding="utf-8")
            handler = functools.partial(ReleaseHandler, directory=str(root))
            with http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler) as server:
                for path, cached in (("/assets/probe.js?v=1", True), ("/__rspice_qualification__/automation.html", False)):
                    thread = threading.Thread(target=server.handle_request)
                    thread.start()
                    try:
                        with urllib.request.urlopen(f"http://127.0.0.1:{server.server_port}{path}", timeout=5) as response:
                            self.assertEqual(response.headers["Content-Security-Policy"], "script-src 'self' 'wasm-unsafe-eval'")
                            self.assertEqual(response.headers.get("Cache-Control"), "public, max-age=3600" if cached else None)
                    finally:
                        thread.join(timeout=5)

    def test_static_canvas_is_not_successful_application_startup(self):
        ready = {"canvas": True, "loading": False, "worker_ready": True}
        self.assertTrue(startup_ready(ready))
        for changed in ({"canvas": False}, {"loading": True}, {"worker_ready": False}):
            self.assertFalse(startup_ready({**ready, **changed}))
        with self.assertRaisesRegex(AssertionError, "startup failed"):
            startup_ready({**ready, "error": "startup failed"})

    def test_release_workers_share_the_exact_packaged_cohort(self):
        base = "https://example.test/ide/assets/" + "a" * 64 + "/"
        state = {"simulation_worker": base + "simulation-worker.js",
                 "automation_worker": base + "automation-worker.js"}
        verify_worker_urls(state, base)
        for key in state:
            for value in (None, state[key].replace("/ide/assets/", "/assets/"),
                          state[key].replace("a" * 64, "b" * 64)):
                with self.assertRaises(AssertionError):
                    verify_worker_urls({**state, key: value}, base)

    def test_worker_qualification_finds_development_and_immutable_modules(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for prefix, compressed in (("", False), ("ide/assets/" + "a" * 64, True)):
                assets = root / prefix
                package = assets if compressed else assets / "pkg"
                package.mkdir(parents=True, exist_ok=True)
                worker = assets / "simulation-worker.js"
                wasm = package / ("rspice-ui-worker_bg.wasm.gz" if compressed else "rspice-ui-worker_bg.wasm")
                for path in (worker, assets / "wasm-loader.js", package / "rspice-ui-worker.js", wasm):
                    path.write_bytes(b"fixture")
                relative = worker.relative_to(root)
                self.assertEqual(qualification_worker(root, str(relative)), relative)
                wasm.unlink()
                with self.assertRaisesRegex(ValueError, "missing"):
                    qualification_worker(root, str(relative))
            with self.assertRaisesRegex(ValueError, "inside the served tree"):
                qualification_worker(root, "../simulation-worker.js")

    def test_diagnostic_collection_cannot_erase_the_original_console_failure(self):
        with tempfile.TemporaryDirectory() as directory:
            browser = WorkbenchBrowser.__new__(WorkbenchBrowser)
            browser.output = Path(directory)
            failure = {"level": "SEVERE", "message": "original worker failure"}
            browser.call = Mock(side_effect=[[failure], []])
            browser.script = Mock(side_effect=[[], []])
            for _ in range(2):
                with self.assertRaisesRegex(AssertionError, "original worker failure"):
                    browser.assert_no_errors()
            self.assertEqual(json.loads((browser.output / "console.json").read_text()), [failure])

    def test_nested_control_transforms_preserve_exact_node_identity(self):
        root, parent, child = 2**63, 2**63 + 1, 2**63 + 2
        snapshot = {"tree": {"tree": {"root": root}, "nodes": [
            [root, {"role": "window", "properties": {
                "children": [parent], "transform": [2, 0, 0, 2, 0, 0]}}],
            [parent, {"role": "group", "properties": {
                "children": [child], "transform": [1, 0, 0, 1, 10, 20]}}],
            [child, {"role": "button", "actions": 1, "properties": {
                "label": "Save project", "transform": [0, 1, -1, 0, 0, 0],
                "bounds": {"x0": 0, "y0": 0, "x1": 10, "y1": 20}}}],
        ]}}
        observed = controls(json.loads(json.dumps(snapshot)))
        self.assertEqual(len(observed), 1)
        self.assertEqual(observed[0]["id"], str(child))
        self.assertEqual(observed[0]["label"], "Save project")
        self.assertEqual(observed[0]["center"], (0, 50))

    def test_duplicate_or_cyclic_controls_are_not_click_authority(self):
        snapshot = {"tree": {"tree": {"root": 1}, "nodes": [
            [1, {"role": "window", "properties": {"children": [1]}}],
        ]}}
        with self.assertRaisesRegex(AssertionError, "repeats a node"):
            controls(snapshot)

    def test_retained_artifacts_cannot_satisfy_a_new_run(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            (output / "previous-result.json").write_text("{}", encoding="utf-8")
            with self.assertRaises(FileExistsError):
                WorkbenchBrowser(output, output, driver="unused")

    def test_checkpoint_evidence_requires_fresh_time_exact_bytes_and_one_pair(self):
        raw = b'{"version":1,"workspace":{"project":{"id":"source","name":"Source","revision":1}}}'
        manifest = {"schema_version": 1, "reason": "manual", "created_unix_ms": 105,
                    "project_id": "source", "project_name": "Source", "project_revision": 1,
                    "snapshot_byte_len": len(raw), "snapshot_digest": hashlib.sha256(raw).hexdigest()}
        records = {"checkpoint.manifest": json.dumps(manifest), "checkpoint.snapshot": list(raw)}
        self.assertEqual(verify_checkpoint(records, 100, 110)[1], raw)
        for changed, start, end in (
            ({**records, "checkpoint.snapshot": list(raw + b" ")}, 100, 110),
            ({**records, "checkpoint.snapshot": list(raw.replace(b"1", b"2"))}, 100, 110),
            ({**records, "old.manifest": json.dumps(manifest)}, 100, 110),
            (records, 106, 110), (records, 100, 104),
        ):
            with self.subTest(records=changed, start=start, end=end):
                with self.assertRaises(AssertionError):
                    verify_checkpoint(changed, start, end)

    def test_recovery_copy_requires_new_identity_and_exact_content(self):
        import copy
        original = {"workspace": {"project": {"id": "88ab3c24-8fbd-4317-82eb-f512fd0ab512", "path": None},
                                  "circuit": {"resistance": "10k"}}}
        recovered = copy.deepcopy(original)
        recovered["workspace"]["project"].update(
            id="2438fdce-9c26-4e7b-9a0c-6e6dfec6c23b", path="recovered.rspiceproj")
        verify_recovery_copy(original, recovered, "recovered.rspiceproj")
        for wrong in (original, {**recovered, "unexpected": True}):
            with self.assertRaises(AssertionError):
                verify_recovery_copy(original, wrong, "recovered.rspiceproj")
        recovered["workspace"]["circuit"]["resistance"] = "1k"
        with self.assertRaises(AssertionError):
            verify_recovery_copy(original, recovered, "recovered.rspiceproj")
        self.assertEqual(original["workspace"]["circuit"]["resistance"], "10k")


if __name__ == "__main__":
    unittest.main()
