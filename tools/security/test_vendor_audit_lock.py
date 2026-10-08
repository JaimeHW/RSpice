import hashlib
import json
from pathlib import Path
import sys
import tempfile
import tomllib
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
from vendor_audit_lock import REGISTRY, ROOT, VendorError, audit_lock, relative_path


class VendorAuditTests(unittest.TestCase):
    def fixture(self, root: Path) -> Path:
        (root / "Cargo.toml").write_text('[patch.crates-io]\nzip = {path = "vendor/zip"}\n')
        (root / "Cargo.lock").write_text('version = 4\n[[package]]\nname = "zip"\nversion = "2.2.0"\n')
        vendor = root / "vendor/zip"
        (vendor / "src").mkdir(parents=True)
        (vendor / "Cargo.toml").write_text('[package]\nname = "zip"\nversion = "2.2.0"\n')
        (vendor / "src/lib.rs").write_text("// upstream\n")
        hashes = {name: hashlib.sha256((vendor / name).read_bytes()).hexdigest()
                  for name in ("Cargo.toml", "src/lib.rs")}
        (vendor / "RSPICE_VENDOR.json").write_text(json.dumps({
            "package": "zip", "version": "2.2.0", "registry_package_sha256": "a" * 64,
            "upstream_files_sha256": hashes,
        }))
        (vendor / "src/lib.rs").write_text("// reviewed patch\n")
        (vendor / "RSPICE_PATCH.json").write_text(json.dumps({
            "modified_upstream_files": ["src/lib.rs"],
            "patched_files_sha256": {"src/lib.rs": hashlib.sha256((vendor / "src/lib.rs").read_bytes()).hexdigest()},
        }))
        (vendor / ".gitattributes").write_text("* -text\n")
        return vendor

    def test_registry_identity_is_restored_without_changing_build_graph(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.fixture(root)
            before = (root / "Cargo.lock").read_bytes()
            document = tomllib.loads(audit_lock(root))
            self.assertEqual(document["package"], [{
                "name": "zip", "version": "2.2.0", "source": REGISTRY, "checksum": "a" * 64,
            }])
            self.assertEqual((root / "Cargo.lock").read_bytes(), before)

    def test_changed_or_unrecorded_source_is_rejected(self) -> None:
        for name in ("Cargo.toml", "src/lib.rs", "src/unrecorded.rs"):
            with self.subTest(file=name), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                vendor = self.fixture(root)
                with (vendor / name).open("a") as output:
                    output.write("// unreviewed change\n")
                with self.assertRaisesRegex(VendorError, "checksum mismatch|inventory differs"):
                    audit_lock(root)

    def test_moved_build_version_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.fixture(root)
            (root / "Cargo.lock").write_text('version = 4\n[[package]]\nname = "zip"\nversion = "2.3.0"\n')
            with self.assertRaisesRegex(VendorError, "expected one local package"):
                audit_lock(root)

    def test_missing_patch_hash_cannot_silently_trust_upstream_hash(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            vendor = self.fixture(root)
            path = vendor / "RSPICE_PATCH.json"
            patch = json.loads(path.read_text())
            patch["patched_files_sha256"] = {}
            path.write_text(json.dumps(patch))
            with self.assertRaisesRegex(VendorError, "patch inventory differs"):
                audit_lock(root)

    def test_paths_cannot_escape_vendor_tree(self) -> None:
        for name in ("..", "../outside", "/absolute", "C:/drive", r"C:\drive", "."):
            with self.subTest(path=name), self.assertRaises(VendorError):
                relative_path(name)

    def test_repository_source_is_verified_and_both_audit_gates_are_wired(self) -> None:
        packages = tomllib.loads(audit_lock(ROOT))["package"]
        self.assertTrue(any(package["name"] == "zip" for package in packages))
        for name in ("security.yml", "native-release.yml"):
            workflow = (ROOT / ".github/workflows" / name).read_text(encoding="utf-8")
            self.assertIn("tools/security/vendor_audit_lock.py", workflow)
            self.assertIn("cargo audit --deny warnings --file target/security-vendor-audit.lock", workflow)


if __name__ == "__main__":
    unittest.main()
