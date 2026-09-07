"""Coordinate and evidence-integrity regressions for real-browser qualification."""

import json
import hashlib
from pathlib import Path
import tempfile
import unittest

from browser_workbench import WorkbenchBrowser, controls
from check_browser_workbench import verify_checkpoint, verify_recovery_copy


class BrowserWorkbenchTests(unittest.TestCase):
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
