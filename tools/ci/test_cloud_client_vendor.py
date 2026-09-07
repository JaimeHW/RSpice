"""The vendored RSpice Cloud crates must match their sync manifest.

crates/rspice-pack and crates/rspice-cloud-{domain,contract,client} are exact
copies from the RSpice-Cloud repository plus a reviewed downstream patch,
pinned by tools/cloud/vendor-manifest.json. API integration tests live in RSpice-Cloud,
where they run against the real API handlers; this workspace only compiles and
consumes them. Editing a vendored file here forks the client away from the
tree those tests qualified, so any drift must fail CI. Re-sync with
tools/cloud/sync_vendored_client.py instead of editing in place.
"""

import hashlib
import importlib.util
import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = ROOT / "tools" / "cloud" / "vendor-manifest.json"


def load_sync_tool():
    """Load the sync tool by path.

    CI runs this file as a script, so the repository root is never on
    ``sys.path`` and ``tools`` is not an importable package. Loading by
    location is what the other CI tests do for the tool they cover.
    """
    tool_path = ROOT / "tools" / "cloud" / "sync_vendored_client.py"
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location("sync_vendored_client", tool_path)
    if spec is None or spec.loader is None:
        raise AssertionError(f"cannot load {tool_path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


sync_tool = load_sync_tool()
source_origin_is_authoritative = sync_tool.source_origin_is_authoritative


class CloudClientVendorTests(unittest.TestCase):
    def setUp(self) -> None:
        self.manifest = json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))

    def test_manifest_pins_a_full_source_commit(self) -> None:
        self.assertEqual(self.manifest["source_repository"], "JaimeHW/RSpice-Cloud")
        self.assertRegex(self.manifest["source_sha"], r"^[0-9a-f]{40}$")
        self.assertEqual(
            self.manifest["crates"],
            [
                "rspice-pack",
                "rspice-cloud-domain",
                "rspice-cloud-contract",
                "rspice-cloud-client",
            ],
        )

    def test_downstream_patch_is_pinned_and_reconstructs_the_upstream_bytes(self) -> None:
        patch = self.manifest["patch"]
        self.assertEqual(patch["path"], sync_tool.PATCH_RELATIVE)
        patch_path = ROOT / patch["path"]
        self.assertEqual(hashlib.sha256(patch_path.read_bytes()).hexdigest(), patch["sha256"])
        with tempfile.TemporaryDirectory() as directory:
            staging = Path(directory)
            for relative in self.manifest["files"]:
                target = staging / relative
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / relative, target)
            subprocess.run(
                ["git", "-c", "core.autocrlf=false", "apply", "--no-index", "--reverse", str(patch_path)],
                cwd=staging, check=True, capture_output=True,
            )
            recovered = {
                path.relative_to(staging).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest()
                for path in staging.rglob("*") if path.is_file()
            }
            self.assertEqual(recovered, self.manifest["upstream_files"])
            sync_tool.apply_vendor_patch(staging, patch_path)
            for relative, digest in self.manifest["files"].items():
                self.assertEqual(hashlib.sha256((staging / relative).read_bytes()).hexdigest(), digest)

    def test_patch_rejects_incompatible_upstream_without_partial_application(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            staging = Path(directory)
            source = staging / "source.txt"
            source.write_text("upstream changed\n", encoding="utf-8", newline="\n")
            patch = staging / "change.patch"
            patch.write_text(
                "--- a/source.txt\n+++ b/source.txt\n@@ -1 +1 @@\n-old\n+new\n",
                encoding="utf-8", newline="\n",
            )
            with self.assertRaises(subprocess.CalledProcessError):
                sync_tool.apply_vendor_patch(staging, patch)
            self.assertEqual(source.read_text(encoding="utf-8"), "upstream changed\n")

    def test_sync_admits_only_the_authoritative_credential_free_origin(self) -> None:
        for admitted in (
            "https://github.com/JaimeHW/RSpice-Cloud.git",
            "git@github.com:JaimeHW/RSpice-Cloud.git",
            "ssh://git@github.com/JaimeHW/RSpice-Cloud.git",
        ):
            with self.subTest(origin=admitted):
                self.assertTrue(source_origin_is_authoritative(admitted))
        for rejected in (
            "https://user:token@github.com/JaimeHW/RSpice-Cloud.git",
            "https://github.com/other/RSpice-Cloud.git",
            "C:/Users/example/RSpice-Cloud",
            "",
        ):
            with self.subTest(origin=rejected):
                self.assertFalse(source_origin_is_authoritative(rejected))

    def test_every_vendored_file_matches_its_recorded_digest(self) -> None:
        for relative, digest in self.manifest["files"].items():
            with self.subTest(file=relative):
                path = ROOT / relative
                self.assertTrue(path.is_file(), f"{relative} is missing; re-sync")
                actual = hashlib.sha256(path.read_bytes()).hexdigest()
                self.assertEqual(
                    actual,
                    digest,
                    f"{relative} was edited in place; vendored trees are "
                    "read-only — change RSpice-Cloud and re-sync",
                )

    def test_vendored_trees_contain_no_unrecorded_files(self) -> None:
        recorded = set(self.manifest["files"])
        roots = [ROOT / "crates" / crate for crate in self.manifest["crates"]]
        roots.append(ROOT / "testdata")
        for vendored_root in roots:
            for path in sorted(p for p in vendored_root.rglob("*") if p.is_file()):
                relative = path.relative_to(ROOT).as_posix()
                with self.subTest(file=relative):
                    self.assertIn(
                        relative,
                        recorded,
                        f"{relative} is not part of the vendored sync; "
                        "vendored trees are read-only",
                    )


if __name__ == "__main__":
    unittest.main()
