"""vendor/epaint must stay crates.io epaint 0.35.0 plus the reviewed patch.

The workspace builds every egui crate against vendor/epaint through
`[patch.crates-io]`. The tree is the published package byte for byte with one
fix applied (tools/vendor/epaint.patch), so an edit anywhere else forks epaint
silently. tools/vendor/vendor_epaint.py re-derives the tree; this proves
offline that it still is what that tool writes.
"""

import importlib.util
import re
import shutil
import sys
import tempfile
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def load_tool():
    """Load the vendoring tool by path; CI runs this file as a script."""
    tool_path = ROOT / "tools" / "vendor" / "vendor_epaint.py"
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location("vendor_epaint", tool_path)
    if spec is None or spec.loader is None:
        raise AssertionError(f"cannot load {tool_path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


tool = load_tool()


class VendoredEpaintTests(unittest.TestCase):
    def setUp(self) -> None:
        self.manifest = tool.load_manifest()

    def test_vendored_tree_is_the_package_plus_the_patch(self) -> None:
        self.assertEqual(tool.check_tree(tool.VENDOR, self.manifest), [])

    def test_the_patch_changes_only_the_text_layout(self) -> None:
        patch = tool.PATCH.read_text(encoding="utf-8")
        self.assertEqual(
            re.findall(r"^\+\+\+ b/(\S+)$", patch, flags=re.MULTILINE),
            ["src/text/text_layout.rs"],
        )

    def test_an_edit_outside_the_patch_is_caught(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            copy = Path(directory) / "epaint"
            shutil.copytree(tool.VENDOR, copy)
            with (copy / "src" / "lib.rs").open("a", encoding="utf-8", newline="\n") as file:
                file.write("// local change\n")
            self.assertEqual(
                tool.check_tree(copy, self.manifest),
                ["src/lib.rs differs from upstream outside the patch"],
            )

    def test_an_edit_inside_the_patched_lines_is_caught(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            copy = Path(directory) / "epaint"
            shutil.copytree(tool.VENDOR, copy)
            layout = copy / "src" / "text" / "text_layout.rs"
            text = layout.read_bytes().replace(b"let is_rtl = ", b"let is_rtl = !", 1)
            layout.write_bytes(text)
            problems = tool.check_tree(copy, self.manifest)
            self.assertEqual(len(problems), 1, problems)
            self.assertIn("does not reverse out", problems[0])

    def test_an_added_file_is_caught(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            copy = Path(directory) / "epaint"
            shutil.copytree(tool.VENDOR, copy)
            (copy / "src" / "extra.rs").write_text("\n", encoding="utf-8")
            self.assertEqual(
                tool.check_tree(copy, self.manifest),
                ["src/extra.rs is not part of the upstream package"],
            )

    def test_checkouts_keep_the_vendored_bytes(self) -> None:
        attributes = (ROOT / ".gitattributes").read_text(encoding="utf-8").splitlines()
        self.assertIn("vendor/epaint/** -text", attributes)
        self.assertIn("tools/vendor/epaint.patch -text", attributes)

    def test_the_workspace_builds_egui_against_the_vendored_epaint(self) -> None:
        manifest = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
        self.assertEqual(
            manifest["patch"]["crates-io"]["epaint"], {"path": "vendor/epaint"}
        )
        self.assertIn("vendor/epaint", manifest["workspace"]["exclude"])
        vendored = tomllib.loads((tool.VENDOR / "Cargo.toml").read_text(encoding="utf-8"))
        self.assertEqual(vendored["package"]["version"], self.manifest["version"])

    def test_egui_is_still_the_release_the_patch_was_made_for(self) -> None:
        # At an egui upgrade the patch stops applying; vendor that release's
        # epaint with the fix, or delete the tree if the release ships it.
        lock = tomllib.loads((ROOT / "Cargo.lock").read_text(encoding="utf-8"))
        egui = {package["version"] for package in lock["package"] if package["name"] == "egui"}
        self.assertEqual(egui, {self.manifest["version"]})
        epaint = [package for package in lock["package"] if package["name"] == "epaint"]
        self.assertEqual(len(epaint), 1)
        self.assertNotIn("source", epaint[0], "epaint must resolve to vendor/epaint")


if __name__ == "__main__":
    unittest.main()
