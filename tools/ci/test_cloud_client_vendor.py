"""The vendored RSpice Cloud client crates must match their sync manifest.

crates/rspice-cloud-{domain,contract,client} are exact copies from the
RSpice-Cloud repository, pinned to one source commit by
tools/cloud/vendor-manifest.json. Their behavioural tests live in RSpice-Cloud,
where they run against the real API handlers; this workspace only compiles and
consumes them. Editing a vendored file here forks the client away from the
tree those tests qualified, so any drift must fail CI. Re-sync with
tools/cloud/sync_vendored_client.py instead of editing in place.
"""

import hashlib
import json
import unittest
from pathlib import Path

from tools.cloud.sync_vendored_client import source_origin_is_authoritative

ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = ROOT / "tools" / "cloud" / "vendor-manifest.json"


class CloudClientVendorTests(unittest.TestCase):
    def setUp(self) -> None:
        self.manifest = json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))

    def test_manifest_pins_a_full_source_commit(self) -> None:
        self.assertEqual(self.manifest["source_repository"], "JaimeHW/RSpice-Cloud")
        self.assertRegex(self.manifest["source_sha"], r"^[0-9a-f]{40}$")
        self.assertEqual(
            self.manifest["crates"],
            ["rspice-cloud-domain", "rspice-cloud-contract", "rspice-cloud-client"],
        )

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
