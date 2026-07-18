import hashlib
import json
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
    def package(self, root: Path, target: str) -> tuple[Path, Path]:
        binary = root / ("rspice.exe" if "windows" in target else "rspice")
        binary.write_bytes(b"deterministic-rspice-binary\x00fixture")
        return package_release(
            binary=binary,
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
                manifest = json.load(package.extractfile(f"{prefix}/RELEASE-MANIFEST.json"))
                executable = package.getmember(f"{prefix}/rspice")
                self.assertEqual(executable.mode, 0o755)

            self.assertEqual(manifest["schema_version"], 1)
            self.assertEqual(manifest["source"]["commit"], COMMIT)
            self.assertTrue(manifest["build"]["locked_dependencies"])
            payload_paths = [entry["path"] for entry in manifest["files"]]
            for required in [
                "Cargo.lock",
                "NATIVE-RELEASE.md",
                "PRODUCTION-RUNBOOK.md",
                "production.toml",
            ]:
                self.assertIn(required, payload_paths)

    def test_windows_zip_contains_canonical_executable_and_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            archive, checksum = self.package(Path(directory), "x86_64-pc-windows-msvc")
            self.assert_checksum(archive, checksum)
            prefix = f"rspice-{workspace_version()}-x86_64-pc-windows-msvc"
            with zipfile.ZipFile(archive) as package:
                self.assertIn(f"{prefix}/rspice.exe", package.namelist())
                manifest = json.loads(
                    package.read(f"{prefix}/RELEASE-MANIFEST.json").decode("utf-8")
                )
            self.assertEqual(manifest["target"], "x86_64-pc-windows-msvc")

    def test_workspace_version_mismatch_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary = root / "rspice"
            binary.write_bytes(b"fixture")
            with self.assertRaisesRegex(PackageError, "does not match workspace"):
                package_release(
                    binary=binary,
                    target="x86_64-unknown-linux-gnu",
                    version="99.0.0",
                    commit=COMMIT,
                    source_date_epoch=EPOCH,
                    output_directory=root / "dist",
                )


if __name__ == "__main__":
    unittest.main()
