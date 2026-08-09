import hashlib
import json
import sys
import tempfile
import unittest
import zipfile
from pathlib import Path


sys.path.insert(0, str(Path(__file__).resolve().parent))
from package_web import (  # noqa: E402
    ASSET_DIGEST_DOMAIN,
    PackageError,
    Payload,
    RUNTIME_DIGEST_DOMAIN,
    assemble,
    digest_payloads,
    locked_wasm_bindgen_version,
    workspace_version,
)


COMMIT = "0123456789abcdef" * 2 + "01234567"
EPOCH = 1_700_000_000


class BrowserReleasePackageTests(unittest.TestCase):
    def write_web_fixture(self, root: Path) -> Path:
        web = root / "web"
        package = web / "pkg"
        runtime = web / "python" / "pyodide-314.0.2"
        package.mkdir(parents=True)
        runtime.mkdir(parents=True)

        (web / "index.html").write_text(
            '<script>const RELEASE_ASSET_ROOT = "__RSPICE_ASSET_ROOT__";</script>\n',
            encoding="utf-8",
        )
        (web / "simulation-worker.js").write_text(
            "// immutable simulation worker\n", encoding="utf-8"
        )
        (package / "rspice-ui.js").write_text(
            "export function runRspiceUiWorkerRequest() {}\n"
            "export function runRspiceUiVerilogACompileRequest() {}\n"
            "export function runRspiceUiHardcopyRequest() {}\n",
            encoding="utf-8",
        )
        (package / "rspice-ui_bg.wasm").write_bytes(b"\x00asm\x01\x00\x00\x00")

        runtime_files = {
            "pyodide.mjs": b"export const pyodide = true;\n",
            "python_stdlib.zip": b"fixture-stdlib",
            "LICENSE.pyodide.txt": b"fixture license\n",
        }
        for name, content in runtime_files.items():
            (runtime / name).write_bytes(content)
        bootstrap = b"# trusted browser bootstrap\n"
        (web / "python" / "rspice_browser_bootstrap.py").write_bytes(bootstrap)

        execution = [
            Payload("python/pyodide-314.0.2/pyodide.mjs", runtime_files["pyodide.mjs"]),
            Payload(
                "python/pyodide-314.0.2/python_stdlib.zip",
                runtime_files["python_stdlib.zip"],
            ),
            Payload("python/rspice_browser_bootstrap.py", bootstrap),
        ]
        runtime_digest = digest_payloads(RUNTIME_DIGEST_DOMAIN, execution)
        entries = []
        for name, content in runtime_files.items():
            entries.append(
                {
                    "path": name,
                    "bytes": len(content),
                    "sha256": hashlib.sha256(content).hexdigest(),
                    "execution_asset": name != "LICENSE.pyodide.txt",
                }
            )
        manifest = {
            "format": "rspice-browser-python-runtime/v2",
            "runtime": "pyodide-314.0.2",
            "runtime_digest_sha256": runtime_digest,
            "external_network_required_at_runtime": False,
            "files": entries,
        }
        (runtime / "rspice-runtime-manifest.json").write_text(
            json.dumps(manifest), encoding="utf-8"
        )

        pinned = [f'"{runtime_digest}"']
        for payload in execution:
            pinned.extend(
                [
                    Path(payload.path).name,
                    str(len(payload.content)),
                    hashlib.sha256(payload.content).hexdigest(),
                ]
            )
        (web / "automation-worker.js").write_text(
            "\n".join(pinned) + "\n", encoding="utf-8"
        )
        return web

    def package(self, web: Path, output: Path):
        return assemble(
            web,
            output,
            workspace_version(),
            COMMIT,
            EPOCH,
            locked_wasm_bindgen_version(),
        )

    def test_archive_is_deterministic_closed_and_content_addressed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            web = self.write_web_fixture(root)
            first, first_checksum, first_digest = self.package(web, root / "first")
            second, _, second_digest = self.package(web, root / "second")

            self.assertEqual(first.read_bytes(), second.read_bytes())
            self.assertEqual(first_digest, second_digest)
            self.assertEqual(
                first_checksum.read_text(encoding="ascii"),
                f"{first_digest}  {first.name}\n",
            )

            with zipfile.ZipFile(first) as archive:
                names = archive.namelist()
                prefix = f"rspice-ide-{workspace_version()}/"
                manifest = json.loads(
                    archive.read(prefix + "RELEASE-MANIFEST.json")
                )
                asset_root = manifest["asset_root"]
                self.assertRegex(asset_root, r"^assets/[0-9a-f]{64}$")
                index = archive.read(prefix + "index.html").decode("utf-8")
                self.assertIn(f'"./{asset_root}"', index)
                self.assertNotIn("__RSPICE_ASSET_ROOT__", index)
                self.assertIn(prefix + f"{asset_root}/rspice-ui_bg.wasm", names)
                self.assertIn(
                    prefix
                    + f"{asset_root}/python/pyodide-314.0.2/python_stdlib.zip",
                    names,
                )
                self.assertNotIn(prefix + f"{asset_root}/rspice-ui.d.ts", names)
                self.assertEqual(manifest["schema"], "rspice.browser-release/v1")
                self.assertEqual(manifest["source"]["commit"], COMMIT)
                self.assertTrue(manifest["build"]["locked_dependencies"])

    def test_asset_digest_authenticates_paths_as_well_as_content(self) -> None:
        original = [Payload("a", b"bc"), Payload("d", b"ef")]
        reframed = [Payload("ab", b"c"), Payload("d", b"ef")]
        self.assertNotEqual(
            digest_payloads(ASSET_DIGEST_DOMAIN, original),
            digest_payloads(ASSET_DIGEST_DOMAIN, reframed),
        )

    def test_tampered_or_extra_runtime_assets_fail_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            web = self.write_web_fixture(root)
            runtime = web / "python" / "pyodide-314.0.2"
            (runtime / "pyodide.mjs").write_bytes(b"tampered")
            with self.assertRaisesRegex(PackageError, "wrong size|SHA-256"):
                self.package(web, root / "tampered-output")

            web = self.write_web_fixture(root / "extra")
            (web / "python" / "pyodide-314.0.2" / "undeclared.js").write_text(
                "surprise", encoding="utf-8"
            )
            with self.assertRaisesRegex(PackageError, "inventory is not closed"):
                self.package(web, root / "extra-output")

    def test_entrypoint_and_wasm_exports_are_mandatory(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            web = self.write_web_fixture(root)
            (web / "index.html").write_text("no release token\n", encoding="utf-8")
            with self.assertRaisesRegex(PackageError, "exactly one"):
                self.package(web, root / "missing-token")

            web = self.write_web_fixture(root / "export")
            (web / "pkg" / "rspice-ui.js").write_text(
                "export default function init() {}\n", encoding="utf-8"
            )
            with self.assertRaisesRegex(PackageError, "missing runRspiceUiWorkerRequest"):
                self.package(web, root / "missing-export")

    def test_lockfile_mismatched_binding_generator_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            web = self.write_web_fixture(root)
            with self.assertRaisesRegex(PackageError, "does not match Cargo.lock"):
                assemble(
                    web,
                    root / "output",
                    workspace_version(),
                    COMMIT,
                    EPOCH,
                    "0.2.1",
                )


if __name__ == "__main__":
    unittest.main()
