import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


sys.path.insert(0, str(Path(__file__).resolve().parent))
from stage_managed_python import StagingError, stage_runtime  # noqa: E402


class ManagedPythonStagingTests(unittest.TestCase):
    def fixture(self, root: Path) -> tuple[Path, Path, dict[str, str]]:
        source = root / "source-python"
        executable = source / ("python.exe" if sys.platform == "win32" else "bin/python3.14")
        executable.parent.mkdir(parents=True)
        executable.write_bytes(b"relocatable-python-fixture")
        (source / "BUILD").write_text("fixture-build-20260805\n", encoding="utf-8")
        (source / "LICENSE").write_text("Python fixture license\n", encoding="utf-8")
        stdlib = source / "lib/python3.14"
        stdlib.mkdir(parents=True)
        (stdlib / "json.py").write_text("# fixture\n", encoding="utf-8")
        cache = stdlib / "__pycache__"
        cache.mkdir()
        (cache / "json.pyc").write_bytes(b"cache")
        pip_metadata = stdlib / "site-packages/pip-99.0.dist-info"
        pip_metadata.mkdir(parents=True)
        (pip_metadata / "LICENSE").write_text("removed metadata\n", encoding="utf-8")
        worker = root / "rspice_worker.py"
        worker.write_text("print('worker')\n", encoding="utf-8")
        identity = {
            "version": "3.14.6",
            "base_prefix": str(source),
            "executable": str(executable),
            "cache_tag": "cpython-314",
            "platform": "fixture-platform",
        }
        return executable, worker, identity

    def test_stage_is_atomic_stripped_self_describing_and_worker_complete(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            executable, worker, identity = self.fixture(root)
            output = root / "stage/runtimes/python"
            with mock.patch("stage_managed_python.inspect_python", side_effect=[identity, identity]):
                staged, relative, observed, build = stage_runtime(
                    python_executable=executable,
                    output_root=output,
                    worker=worker,
                    expected_version="3.14.6",
                    source_identity="python-build-standalone@fixture",
                )
            self.assertEqual(staged, output)
            self.assertEqual(observed, identity)
            self.assertEqual(build, "fixture-build-20260805")
            self.assertTrue((staged / relative).is_file())
            self.assertTrue((staged / "worker/rspice_worker.py").is_file())
            self.assertFalse((staged / "lib/python3.14/__pycache__").exists())
            self.assertFalse((staged / "lib/python3.14/site-packages/pip-99.0.dist-info").exists())
            notice = json.loads((staged / "PYTHON-RUNTIME-NOTICE.json").read_text())
            self.assertEqual(notice["version"], "3.14.6")
            self.assertEqual(notice["distribution_build"], "fixture-build-20260805")
            self.assertEqual(notice["license_files"], ["LICENSE"])
            sbom = json.loads((staged / "PYTHON-RUNTIME-SBOM.cdx.json").read_text())
            self.assertEqual(sbom["bomFormat"], "CycloneDX")

    def test_wrong_patch_and_existing_destination_fail_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            executable, worker, identity = self.fixture(root)
            wrong = {**identity, "version": "3.14.5"}
            with mock.patch("stage_managed_python.inspect_python", return_value=wrong):
                with self.assertRaisesRegex(StagingError, "exactly 3.14.6"):
                    stage_runtime(
                        python_executable=executable,
                        output_root=root / "runtime",
                        worker=worker,
                        expected_version="3.14.6",
                        source_identity="fixture",
                    )

            existing = root / "existing"
            existing.mkdir()
            with mock.patch("stage_managed_python.inspect_python", return_value=identity):
                with self.assertRaisesRegex(StagingError, "already exists"):
                    stage_runtime(
                        python_executable=executable,
                        output_root=existing,
                        worker=worker,
                        expected_version="3.14.6",
                        source_identity="fixture",
                    )


if __name__ == "__main__":
    unittest.main()
