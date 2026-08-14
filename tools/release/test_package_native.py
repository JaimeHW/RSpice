import hashlib
import json
import os
import sys
import tarfile
import tempfile
import unittest
import zipfile
from pathlib import Path


sys.path.insert(0, str(Path(__file__).resolve().parent))
from package_native import PackageError, package_release, workspace_version  # noqa: E402


COMMIT = "0123456789abcdef" * 2 + "01234567"
EPOCH = 1_700_000_000


class NativeReleasePackageTests(unittest.TestCase):
    def fixtures(self, root: Path, target: str) -> tuple[Path, Path, Path]:
        suffix = ".exe" if "windows" in target else ""
        binary = root / f"rspice{suffix}"
        ui_binary = root / f"rspice-ui{suffix}"
        runtime = root / "python-runtime"
        (runtime / "bin").mkdir(parents=True)
        binary.write_bytes(b"deterministic-rspice-binary\x00fixture")
        ui_binary.write_bytes(b"deterministic-rspice-ui-binary\x00fixture")
        runtime_python = runtime / "bin" / f"python{suffix}"
        runtime_python.write_bytes(b"managed-python\x00fixture")
        runtime_python.chmod(0o755)
        (runtime / "runtime-manifest.json").write_text(
            '{"schema":"rspice.managed-python-runtime/v2"}\n', encoding="utf-8"
        )
        (runtime / "runtime-manifest.ed25519.json").write_text(
            '{"schema":"rspice.managed-python-signature/v1"}\n', encoding="utf-8"
        )
        (runtime / "worker").mkdir()
        (runtime / "worker" / "rspice_worker.py").write_text(
            "# governed worker fixture\n", encoding="utf-8"
        )
        return binary, ui_binary, runtime

    def package(self, root: Path, target: str) -> tuple[Path, Path]:
        binary, ui_binary, runtime = self.fixtures(root, target)
        return package_release(
            binary=binary,
            ui_binary=ui_binary,
            runtime_root=runtime,
            target=target,
            version=workspace_version(),
            commit=COMMIT,
            source_date_epoch=EPOCH,
            output_directory=root / "dist",
        )

    def assert_checksum(self, archive: Path, checksum: Path) -> None:
        expected, name = checksum.read_text(encoding="ascii").strip().split("  ")
        self.assertEqual(name, archive.name)
        self.assertEqual(expected, hashlib.sha256(archive.read_bytes()).hexdigest())

    def test_tar_package_is_deterministic_manifested_and_checksumbound(self) -> None:
        with tempfile.TemporaryDirectory() as first, tempfile.TemporaryDirectory() as second:
            archive, checksum = self.package(Path(first), "x86_64-unknown-linux-gnu")
            repeated, _ = self.package(Path(second), "x86_64-unknown-linux-gnu")
            self.assertEqual(archive.read_bytes(), repeated.read_bytes())
            self.assert_checksum(archive, checksum)

            with tarfile.open(archive, "r:gz") as package:
                names = package.getnames()
                prefix = f"rspice-{workspace_version()}-x86_64-unknown-linux-gnu"
                self.assertIn(f"{prefix}/rspice", names)
                self.assertIn(f"{prefix}/rspice-ui", names)
                self.assertIn(f"{prefix}/runtimes/python/bin/python", names)
                self.assertIn(
                    f"{prefix}/runtimes/python/runtime-manifest.ed25519.json", names
                )
                manifest = json.load(package.extractfile(f"{prefix}/RELEASE-MANIFEST.json"))
                executable = package.getmember(f"{prefix}/rspice")
                ui_executable = package.getmember(f"{prefix}/rspice-ui")
                runtime_executable = package.getmember(
                    f"{prefix}/runtimes/python/bin/python"
                )
                self.assertEqual(executable.mode, 0o755)
                self.assertEqual(ui_executable.mode, 0o755)
                if os.name != "nt":
                    self.assertEqual(runtime_executable.mode, 0o755)

            self.assertEqual(manifest["schema_version"], 1)
            self.assertEqual(manifest["source"]["commit"], COMMIT)
            self.assertTrue(manifest["build"]["locked_dependencies"])
            payload_paths = [entry["path"] for entry in manifest["files"]]
            for required in [
                "CLI-README.md",
                "Cargo.lock",
                "LICENSE",
                "NOTICE",
                "README.md",
                "rspice-ui",
                "runtimes/python/runtime-manifest.json",
                "runtimes/python/runtime-manifest.ed25519.json",
                "runtimes/python/worker/rspice_worker.py",
            ]:
                self.assertIn(required, payload_paths)

    def test_windows_zip_contains_canonical_executable_and_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            archive, checksum = self.package(Path(directory), "x86_64-pc-windows-msvc")
            self.assert_checksum(archive, checksum)
            prefix = f"rspice-{workspace_version()}-x86_64-pc-windows-msvc"
            with zipfile.ZipFile(archive) as package:
                self.assertIn(f"{prefix}/rspice.exe", package.namelist())
                self.assertIn(f"{prefix}/rspice-ui.exe", package.namelist())
                self.assertIn(
                    f"{prefix}/runtimes/python/bin/python.exe", package.namelist()
                )
                manifest = json.loads(
                    package.read(f"{prefix}/RELEASE-MANIFEST.json").decode("utf-8")
                )
            self.assertEqual(manifest["target"], "x86_64-pc-windows-msvc")

    def test_workspace_version_mismatch_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary, ui_binary, runtime = self.fixtures(
                root, "x86_64-unknown-linux-gnu"
            )
            with self.assertRaisesRegex(PackageError, "does not match workspace"):
                package_release(
                    binary=binary,
                    ui_binary=ui_binary,
                    runtime_root=runtime,
                    target="x86_64-unknown-linux-gnu",
                    version="99.0.0",
                    commit=COMMIT,
                    source_date_epoch=EPOCH,
                    output_directory=root / "dist",
                )

    def test_missing_ui_binary_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary, ui_binary, runtime = self.fixtures(
                root, "x86_64-unknown-linux-gnu"
            )
            ui_binary.unlink()
            with self.assertRaisesRegex(PackageError, "UI binary must be a regular file"):
                package_release(
                    binary=binary,
                    ui_binary=ui_binary,
                    runtime_root=runtime,
                    target="x86_64-unknown-linux-gnu",
                    version=workspace_version(),
                    commit=COMMIT,
                    source_date_epoch=EPOCH,
                    output_directory=root / "dist",
                )

    def test_unsigned_runtime_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary, ui_binary, runtime = self.fixtures(
                root, "x86_64-unknown-linux-gnu"
            )
            (runtime / "runtime-manifest.ed25519.json").unlink()
            with self.assertRaisesRegex(PackageError, "signed runtime metadata is missing"):
                package_release(
                    binary=binary,
                    ui_binary=ui_binary,
                    runtime_root=runtime,
                    target="x86_64-unknown-linux-gnu",
                    version=workspace_version(),
                    commit=COMMIT,
                    source_date_epoch=EPOCH,
                    output_directory=root / "dist",
                )

    def test_runtime_link_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary, ui_binary, runtime = self.fixtures(
                root, "x86_64-unknown-linux-gnu"
            )
            outside = root / "outside"
            outside.write_bytes(b"outside")
            link = runtime / "linked-runtime-member"
            try:
                link.symlink_to(outside)
            except OSError as error:
                self.skipTest(f"this host cannot create a test symlink: {error}")
            with self.assertRaisesRegex(PackageError, "runtime payload contains a link"):
                package_release(
                    binary=binary,
                    ui_binary=ui_binary,
                    runtime_root=runtime,
                    target="x86_64-unknown-linux-gnu",
                    version=workspace_version(),
                    commit=COMMIT,
                    source_date_epoch=EPOCH,
                    output_directory=root / "dist",
                )


if __name__ == "__main__":
    unittest.main()
