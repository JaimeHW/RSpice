import hashlib
import json
import os
import plistlib
import subprocess
import sys
import tarfile
import tempfile
import unittest
import zipfile
from pathlib import Path
from unittest import mock


sys.path.insert(0, str(Path(__file__).resolve().parent))
import package_native  # noqa: E402
from package_native import (  # noqa: E402
    PackageError,
    model_tree_payloads,
    package_release,
    workspace_version,
)


COMMIT = "0123456789abcdef" * 2 + "01234567"
EPOCH = 1_700_000_000


class NativeReleasePackageTests(unittest.TestCase):
    def fixtures(self, root: Path, target: str) -> tuple[Path, Path, Path]:
        suffix = ".exe" if "windows" in target else ""
        binary = root / f"rspice{suffix}"
        ui_binary = root / f"rspice-app{suffix}"
        runtime = root / "python-runtime"
        (runtime / "bin").mkdir(parents=True)
        binary.write_bytes(b"deterministic-rspice-binary\x00fixture")
        ui_binary.write_bytes(b"deterministic-rspice-app-binary\x00fixture")
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
                self.assertIn(f"{prefix}/rspice-app", names)
                self.assertIn(f"{prefix}/runtimes/python/bin/python", names)
                self.assertIn(
                    f"{prefix}/runtimes/python/runtime-manifest.ed25519.json", names
                )
                manifest = json.load(package.extractfile(f"{prefix}/RELEASE-MANIFEST.json"))
                executable = package.getmember(f"{prefix}/rspice")
                ui_executable = package.getmember(f"{prefix}/rspice-app")
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
                "rspice-app",
                "runtimes/python/runtime-manifest.json",
                "runtimes/python/runtime-manifest.ed25519.json",
                "runtimes/python/worker/rspice_worker.py",
            ]:
                self.assertIn(required, payload_paths)
            model_paths = [
                path for path in payload_paths if path.startswith("models/spice/")
            ]
            self.assertEqual(
                model_paths,
                [
                    "models/spice/SHIPPED-CATALOG.tsv",
                    "models/spice/SHIPPED-PACKS.tsv",
                    "models/spice/SHIPPING.toml",
                    "models/spice/foundation/lib/foundation.lib",
                    "models/spice/foundation/pack.toml",
                ],
            )
            self.assertFalse(
                any(
                    f"models/spice/{category}/" in path
                    for path in model_paths
                    for category in (
                        "academic",
                        "builtin",
                        "community",
                        "foundry",
                        "vendor",
                    )
                )
            )

    def test_macos_tar_carries_the_application_bundle_and_its_launcher(self) -> None:
        target = "aarch64-apple-darwin"
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            archive, checksum = self.package(root, target)
            self.assert_checksum(archive, checksum)
            prefix = f"rspice-{workspace_version()}-{target}"
            contents = f"{prefix}/RSpice.app/Contents"
            with tarfile.open(archive, "r:gz") as package:
                ui_executable = package.getmember(f"{contents}/MacOS/rspice-app")
                launcher = package.getmember(f"{prefix}/rspice-app")
                info = plistlib.load(package.extractfile(f"{contents}/Info.plist"))
                icon = package.extractfile(f"{contents}/Resources/RSpice.icns").read()
                manifest = json.load(package.extractfile(f"{prefix}/RELEASE-MANIFEST.json"))
                package.extractall(root / "installed", filter="data")
            self.assertEqual(ui_executable.mode, 0o755)
            self.assertEqual(launcher.mode, 0o755)
            self.assertEqual(info["CFBundleIdentifier"], "app.rspice.RSpice")
            self.assertEqual(info["CFBundleExecutable"], "rspice-app")
            self.assertEqual(info["CFBundlePackageType"], "APPL")
            self.assertEqual(info["CFBundleShortVersionString"], workspace_version())
            self.assertEqual(info["CFBundleVersion"], workspace_version())
            self.assertEqual(info["CFBundleIconFile"], "RSpice.icns")
            self.assertTrue(info["NSHighResolutionCapable"])

            self.assertEqual(icon[:4], b"icns")
            self.assertEqual(int.from_bytes(icon[4:8], "big"), len(icon))
            slots, offset = [], 8
            while offset < len(icon):
                slot = icon[offset : offset + 4]
                length = int.from_bytes(icon[offset + 4 : offset + 8], "big")
                self.assertTrue(icon[offset + 8 :].startswith(b"\x89PNG\r\n\x1a\n"))
                slots.append(slot)
                offset += length
            self.assertEqual(offset, len(icon))
            self.assertEqual(
                slots,
                [b"icp4", b"icp5", b"ic11", b"icp6", b"ic12", b"ic07", b"ic08", b"ic13"],
            )

            payload_paths = [entry["path"] for entry in manifest["files"]]
            for required in [
                "RSpice.app/Contents/Info.plist",
                "RSpice.app/Contents/MacOS/rspice-app",
                "RSpice.app/Contents/Resources/RSpice.icns",
                "rspice-app",
                "models/spice/SHIPPING.toml",
                "runtimes/python/runtime-manifest.json",
            ]:
                self.assertIn(required, payload_paths)

            if os.name != "nt":
                installed = root / "installed" / prefix
                bundled = installed / "RSpice.app" / "Contents" / "MacOS" / "rspice-app"
                bundled.write_text('#!/bin/sh\nprintf "%s|" "$@"\n', encoding="ascii")
                bundled.chmod(0o755)
                opened = subprocess.run(
                    [str(installed / "rspice-app"), "a", "b c"],
                    cwd=root,
                    capture_output=True,
                    check=True,
                    text=True,
                )
                self.assertEqual(opened.stdout, "a|b c|")

    def test_windows_zip_contains_canonical_executable_and_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            archive, checksum = self.package(Path(directory), "x86_64-pc-windows-msvc")
            self.assert_checksum(archive, checksum)
            prefix = f"rspice-{workspace_version()}-x86_64-pc-windows-msvc"
            with zipfile.ZipFile(archive) as package:
                self.assertIn(f"{prefix}/rspice.exe", package.namelist())
                self.assertIn(f"{prefix}/rspice-app.exe", package.namelist())
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

    def test_restricted_file_in_allowlisted_pack_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            spice = root / "models" / "spice"
            source = spice / "foundation" / "lib" / "foundation.lib"
            source.parent.mkdir(parents=True)
            source.write_text(".model D D\n", encoding="utf-8")
            (spice / "foundation" / "pack.toml").write_text("", encoding="utf-8")
            (spice / "SHIPPING.toml").write_text(
                'schema = 1\npacks = ["rspice-foundation"]\n', encoding="utf-8"
            )
            (spice / "SHIPPED-PACKS.tsv").write_text(
                "rspice-foundation\tbuiltin\tfoundation\town\tLicenseRef-RSpice\t1\t"
                "lib/foundation.lib\t1\t0\t1\t0\t1\t11\tdiode\tFoundation\n",
                encoding="utf-8",
            )
            (spice / "SHIPPED-CATALOG.tsv").write_text(
                "D\tmodel\tdiode\trspice-foundation\tlib/foundation.lib\t1\t0\ttop\n",
                encoding="utf-8",
            )
            (spice / "LICENSE-AUDIT.tsv").write_text(
                "foundation\tlib/foundation.lib\tmarker\trestricted\t1\t1\tevidence\n",
                encoding="utf-8",
            )

            with mock.patch.object(package_native, "ROOT", root):
                with self.assertRaisesRegex(PackageError, "restricted model source"):
                    model_tree_payloads()

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
