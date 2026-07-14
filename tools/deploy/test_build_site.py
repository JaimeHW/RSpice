import importlib.util
import contextlib
import io
import shutil
import subprocess
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
    @staticmethod
    def make_site_source(path: Path) -> None:
        (path / "assets").mkdir(parents=True)
        (path / "index.html").write_text("index", encoding="utf-8")
        (path / "404.html").write_text("missing", encoding="utf-8")

    @staticmethod
    def make_ide_build_outputs(ide: Path) -> None:
        pkg = ide / "pkg"
        pkg.mkdir(parents=True)
        shutil.copy2(
            ROOT / "crates" / "rspice-ui" / "web" / "index.html",
            ide / "index.html",
        )
        shutil.copy2(
            ROOT / "crates" / "rspice-ui" / "web" / "simulation-worker.js",
            ide / "simulation-worker.js",
        )
        (pkg / "rspice-ui.js").write_text(
            "export { initSync, __wbg_init as default };\n", encoding="utf-8"
        )
        (pkg / "rspice-ui_bg.wasm").write_bytes(b"\x00asm\x01\x00\x00\x00")

    def test_site_source_boundary_rejects_client_runtime_routes(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            site = Path(tmp) / "public"
            self.make_site_source(site)

            build_site.validate_site_source(site)
            (site / "ide").mkdir()
            assert_gate_fails(self, build_site.validate_site_source, site)

    def test_site_source_boundary_requires_expected_path_types(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            site = Path(tmp) / "public"
            self.make_site_source(site)
            (site / "index.html").unlink()
            (site / "index.html").mkdir()

            assert_gate_fails(self, build_site.validate_site_source, site)

    def test_output_boundary_rejects_checkout_source_overlap_and_external_paths(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            base = Path(tmp)
            root = base / "rspice"
            site_parent = root / "_site-source"
            site = site_parent / "dist"
            root.mkdir()
            self.make_site_source(site)
            source_directory = root / "crates"
            source_directory.mkdir()
            non_directory_output = root / "_site-file"
            non_directory_output.write_text("not a directory", encoding="utf-8")

            for output in (
                root,
                site_parent,
                site,
                site / "nested",
                source_directory,
                non_directory_output,
                base / "external",
            ):
                assert_gate_fails(
                    self,
                    build_site.validated_output_path,
                    root,
                    site,
                    output,
                )

    def test_output_boundary_allows_release_controller_staging_layout(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            release = Path(tmp) / "release"
            root = release / "_sources" / "rspice"
            site = release / "_sources" / "site" / "dist"
            root.mkdir(parents=True)
            self.make_site_source(site)

            output = build_site.validated_output_path(
                root, site, release / "dist"
            )

            self.assertEqual(output, (release / "dist").resolve())

            for protected in (release / ".git", release / "_sources"):
                protected.mkdir(exist_ok=True)
                assert_gate_fails(
                    self,
                    build_site.validated_output_path,
                    root,
                    site,
                    protected,
                )

    def test_assembly_refuses_checkout_root_without_deleting_it(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "rspice"
            site = root / "site-source"
            root.mkdir()
            self.make_site_source(site)
            sentinel = root / "keep.txt"
            sentinel.write_text("keep", encoding="utf-8")

            assert_gate_fails(
                self, build_site.assemble_site_sources, root, site, root
            )

            self.assertEqual(sentinel.read_text(encoding="utf-8"), "keep")

    def test_assembly_overlays_only_canonical_client_shell_files(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            site = root / "site-source" / "public"
            out = root / "_site-test"
            ui = root / "crates" / "rspice-ui" / "web"
            play = root / "crates" / "rspice-wasm" / "web"
            self.make_site_source(site)
            ui.mkdir(parents=True)
            play.mkdir(parents=True)

            (site / "index.html").write_text("marketing", encoding="utf-8")
            (site / "assets" / "site.css").write_text("body{}", encoding="utf-8")
            (ui / "index.html").write_text("ide", encoding="utf-8")
            (ui / "simulation-worker.js").write_text("ide worker", encoding="utf-8")
            (ui / "README.md").write_text("do not deploy", encoding="utf-8")
            (play / "index.html").write_text("play", encoding="utf-8")
            (play / "engine-worker.js").write_text("play worker", encoding="utf-8")
            (play / ".gitignore").write_text("pkg", encoding="utf-8")

            build_site.assemble_site_sources(root, site, out)

            self.assertEqual((out / "index.html").read_text(encoding="utf-8"), "marketing")
            self.assertEqual((out / "ide" / "index.html").read_text(encoding="utf-8"), "ide")
            self.assertEqual((out / "play" / "index.html").read_text(encoding="utf-8"), "play")
            self.assertFalse((out / "ide" / "README.md").exists())
            self.assertFalse((out / "play" / ".gitignore").exists())

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

    def test_executable_asset_identity_changes_with_any_payload_byte(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            asset_root = Path(tmp)
            (asset_root / "simulation-worker.js").write_text("worker", encoding="utf-8")
            (asset_root / "rspice-ui.js").write_text("module", encoding="utf-8")
            wasm = asset_root / "rspice-ui_bg.wasm"
            wasm.write_bytes(b"\x00asm\x01\x00\x00\x00")

            first = build_site.executable_asset_identity(asset_root)
            self.assertEqual(first, build_site.executable_asset_identity(asset_root))
            wasm.write_bytes(wasm.read_bytes() + b"\x01")
            second = build_site.executable_asset_identity(asset_root)

            self.assertRegex(first, r"^[0-9a-f]{64}$")
            self.assertNotEqual(first, second)

    def test_packaging_removes_mutable_outputs_and_gate_accepts_exact_digest(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp)
            ide = out / "ide"
            self.make_ide_build_outputs(ide)

            identity = build_site.package_ide_executable_assets(ide)

            self.assertEqual(build_site.gate_ide_worker(out), identity)
            self.assertTrue((ide / "assets" / identity).is_dir())
            self.assertFalse((ide / "simulation-worker.js").exists())
            self.assertFalse((ide / "pkg").exists())
            self.assertIn(
                './assets/%s' % identity,
                (ide / "index.html").read_text(encoding="utf-8"),
            )

    def test_ide_gate_rejects_absent_and_mismatched_digest_directories(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp)
            ide = out / "ide"
            self.make_ide_build_outputs(ide)
            identity = build_site.package_ide_executable_assets(ide)
            asset_root = ide / "assets" / identity
            absent_name = "0" * 64
            index = ide / "index.html"
            source = index.read_text(encoding="utf-8")
            index.write_text(source.replace(identity, absent_name), encoding="utf-8")

            assert_gate_fails(self, build_site.gate_ide_worker, out)

            asset_root.rename(ide / "assets" / absent_name)
            assert_gate_fails(
                self,
                build_site.gate_ide_worker,
                out,
            )

    def test_ide_gate_rejects_tampered_content_and_every_mutable_alias(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp)
            ide = out / "ide"
            self.make_ide_build_outputs(ide)
            identity = build_site.package_ide_executable_assets(ide)
            worker = ide / "assets" / identity / "simulation-worker.js"
            worker.write_text(worker.read_text(encoding="utf-8") + "\n// tampered\n")
            assert_gate_fails(self, build_site.gate_ide_worker, out)

        for alias in (
            "simulation-worker.js",
            "rspice-ui.js",
            "rspice-ui_bg.wasm",
            "pkg",
        ):
            with self.subTest(alias=alias), tempfile.TemporaryDirectory() as tmp:
                out = Path(tmp)
                ide = out / "ide"
                self.make_ide_build_outputs(ide)
                build_site.package_ide_executable_assets(ide)
                mutable = ide / alias
                if alias == "pkg":
                    mutable.mkdir()
                else:
                    mutable.write_bytes(b"mutable alias")
                assert_gate_fails(self, build_site.gate_ide_worker, out)

    def test_packaging_rejects_unexpected_wasm_bindgen_outputs(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            ide = Path(tmp) / "ide"
            self.make_ide_build_outputs(ide)
            (ide / "pkg" / "rspice-ui.d.ts").write_text(
                "unexpected", encoding="utf-8"
            )

            assert_gate_fails(
                self,
                build_site.package_ide_executable_assets,
                ide,
            )

    def test_release_identity_gate_requires_a_clean_checkout(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            checkout = Path(tmp)
            subprocess.run(["git", "init", "-q", str(checkout)], check=True)
            tracked = checkout / "tracked.txt"
            tracked.write_text("accepted", encoding="utf-8")
            subprocess.run(["git", "-C", str(checkout), "add", "tracked.txt"], check=True)
            subprocess.run(
                [
                    "git",
                    "-C",
                    str(checkout),
                    "-c",
                    "user.name=RSpice Test",
                    "-c",
                    "user.email=rspice-test@example.invalid",
                    "commit",
                    "-qm",
                    "fixture",
                ],
                check=True,
            )

            build_site.require_clean_client_checkout(checkout)
            tracked.write_text("dirty", encoding="utf-8")
            assert_gate_fails(
                self,
                build_site.require_clean_client_checkout,
                checkout,
            )

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
