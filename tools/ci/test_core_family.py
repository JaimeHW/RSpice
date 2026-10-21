"""Regression tests for the rspice-core family manifest check."""

from __future__ import annotations

import copy
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from core_family import check  # noqa: E402

CLIPPY = "allow-unwrap-in-tests = true\n"


def toml_value(value) -> str:
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, str):
        return json.dumps(value)
    if isinstance(value, list):
        return "[" + ", ".join(toml_value(item) for item in value) + "]"
    return "{ " + ", ".join(f"{json.dumps(k)} = {toml_value(v)}" for k, v in value.items()) + " }"


def toml_document(tables: dict) -> str:
    lines = []
    for table, entries in tables.items():
        lines.append(f"[{table}]")
        lines.extend(f"{json.dumps(key)} = {toml_value(value)}" for key, value in entries.items())
        lines.append("")
    return "\n".join(lines)


def family_edge(name: str, **extra) -> dict:
    return {"path": f"../{name}", "default-features": False, **extra}


def baseline() -> dict:
    """A consistent three-crate family: base <- netlist <- core."""
    return {
        "workspace": {
            "workspace": {"members": ["crates/*"]},
            "workspace.dependencies": {
                "rspice-core-base": {"path": "crates/rspice-core-base", "default-features": False},
            },
        },
        "rspice-core-base": {
            "package": {"name": "rspice-core-base"},
            "features": {
                "default": ["parallel"],
                "parallel": ["dep:rayon"],
                "veriloga": [],
                "test-support": [],
            },
            "dependencies": {"rayon": {"version": "1", "optional": True}},
        },
        "rspice-core-netlist": {
            "package": {"name": "rspice-core-netlist"},
            "features": {
                "default": ["parallel"],
                "parallel": ["rspice-core-base/parallel"],
                "veriloga": ["rspice-core-base/veriloga"],
                "test-support": ["rspice-core-base/test-support"],
            },
            "dependencies": {"rspice-core-base": {"workspace": True}},
            "dev-dependencies": {
                "rspice-core-netlist": {"path": ".", "default-features": False, "features": ["test-support"]},
            },
        },
        "rspice-core": {
            "package": {"name": "rspice-core"},
            "features": {
                "default": ["parallel"],
                "parallel": ["rspice-core-base/parallel", "rspice-core-netlist/parallel"],
                "veriloga": ["dep:rspice-veriloga", "rspice-core-base/veriloga", "rspice-core-netlist/veriloga"],
            },
            "dependencies": {
                "rspice-core-base": {"workspace": True},
                "rspice-core-netlist": family_edge("rspice-core-netlist"),
                "rspice-veriloga": {"path": "../rspice-veriloga", "optional": True},
            },
            "dev-dependencies": {
                "rspice-core-base": {"workspace": True, "features": ["test-support"]},
                "rspice-core-netlist": family_edge("rspice-core-netlist", features=["test-support"]),
            },
        },
    }


def problems(family: dict, clippy: dict[str, str] | None = None) -> list[str]:
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        (root / "Cargo.toml").write_text(toml_document(family["workspace"]), encoding="utf-8")
        for name, tables in family.items():
            if name == "workspace":
                continue
            crate = root / "crates" / name
            crate.mkdir(parents=True)
            (crate / "Cargo.toml").write_text(toml_document(tables), encoding="utf-8")
            (crate / "clippy.toml").write_text((clippy or {}).get(name, CLIPPY), encoding="utf-8")
        return check(root)


class CoreFamilyTests(unittest.TestCase):
    def assertOneProblem(self, family: dict, fragment: str, clippy: dict[str, str] | None = None) -> None:
        found = problems(family, clippy)
        self.assertEqual(len(found), 1, found)
        self.assertIn(fragment, found[0])

    def test_a_consistent_family_passes(self) -> None:
        self.assertEqual(problems(baseline()), [])

    def test_the_single_crate_of_today_passes(self) -> None:
        family = baseline()
        del family["rspice-core-base"], family["rspice-core-netlist"]
        core = family["rspice-core"]
        core["features"] = {"default": ["parallel"], "parallel": [], "veriloga": ["dep:rspice-veriloga"]}
        core["dependencies"] = {"rspice-veriloga": {"path": "../rspice-veriloga", "optional": True}}
        del core["dev-dependencies"]
        self.assertEqual(problems(family), [])

    def test_no_family_manifest_fails(self) -> None:
        family = {"workspace": baseline()["workspace"]}
        found = problems(family)
        self.assertEqual(len(found), 1)
        self.assertIn("no rspice-core family manifest", found[0])

    def test_a_family_without_rspice_core_fails(self) -> None:
        family = baseline()
        del family["rspice-core"]
        self.assertOneProblem(family, "rspice-core is missing")

    def test_a_crate_outside_the_crate_order_fails(self) -> None:
        family = baseline()
        family["rspice-core-extra"] = copy.deepcopy(family["rspice-core-base"])
        family["rspice-core-extra"]["package"]["name"] = "rspice-core-extra"
        self.assertOneProblem(family, "rspice-core-extra: not in the family's crate order")

    def test_a_missing_or_extra_feature_key_fails(self) -> None:
        family = baseline()
        del family["rspice-core-base"]["features"]["veriloga"]
        family["rspice-core-netlist"]["features"]["simd"] = []
        found = problems(family)
        self.assertIn("rspice-core-base: does not declare the family feature `veriloga`", found)
        self.assertIn("rspice-core-netlist: declares `simd`, which rspice-core does not", found)
        # netlist's `veriloga` now forwards to a feature base lacks; that is
        # the key-set failure above, not a second kind.
        self.assertEqual(len(found), 2, found)

    def test_a_crate_below_core_without_test_support_fails(self) -> None:
        family = baseline()
        del family["rspice-core-base"]["features"]["test-support"]
        self.assertOneProblem(family, "rspice-core-base: does not declare `test-support`")

    def test_an_unforwarded_feature_fails(self) -> None:
        family = baseline()
        family["rspice-core"]["features"]["veriloga"].remove("rspice-core-netlist/veriloga")
        self.assertOneProblem(
            family, "rspice-core: feature `veriloga` does not forward to `rspice-core-netlist/veriloga`"
        )

    def test_an_unforwarded_feature_on_a_renamed_or_target_edge_fails(self) -> None:
        family = baseline()
        netlist = family["rspice-core-netlist"]
        del netlist["dependencies"]
        netlist["target.'cfg(windows)'.dependencies"] = {
            "base": family_edge("rspice-core-base", package="rspice-core-base"),
        }
        netlist["features"]["parallel"] = ["base/parallel"]
        netlist["features"]["veriloga"] = ["base?/veriloga"]
        self.assertEqual(problems(family), [])
        netlist["features"]["veriloga"] = ["rspice-core-base/veriloga"]
        self.assertOneProblem(family, "feature `veriloga` does not forward to `base/veriloga`")

    def test_an_internal_edge_with_default_features_fails(self) -> None:
        family = baseline()
        family["rspice-core"]["dependencies"]["rspice-core-netlist"]["default-features"] = True
        self.assertOneProblem(family, "`rspice-core-netlist` leaves default features on")

        family = baseline()
        del family["workspace"]["workspace.dependencies"]["rspice-core-base"]["default-features"]
        found = problems(family)
        # Every edge that inherits the workspace entry: netlist's and core's
        # normal dependencies and core's dev-dependency.
        self.assertEqual(len(found), 3, found)
        self.assertTrue(all("`rspice-core-base` leaves default features on" in p for p in found))

        # Cargo ignores a member's `default-features = false` when the
        # workspace entry leaves them on.
        family["rspice-core-netlist"]["dependencies"]["rspice-core-base"]["default-features"] = False
        self.assertEqual(len(problems(family)), 3)

    def test_an_edge_inherited_from_a_missing_workspace_entry_fails(self) -> None:
        family = baseline()
        family["rspice-core-netlist"]["dependencies"]["rspice-core-base"] = {"workspace": True}
        del family["workspace"]["workspace.dependencies"]["rspice-core-base"]
        found = problems(family)
        self.assertTrue(any("which declares no such dependency" in p for p in found), found)

    def test_veriloga_below_xspice_fails_even_optionally(self) -> None:
        for name, table in (
            ("rspice-core-base", "dependencies"),
            ("rspice-core-netlist", "dev-dependencies"),
            ("rspice-core-base", "target.'cfg(unix)'.dependencies"),
        ):
            with self.subTest(name=name, table=table):
                family = baseline()
                family[name].setdefault(table, {})["rspice-veriloga"] = {
                    "path": "../rspice-veriloga",
                    "optional": True,
                }
                self.assertOneProblem(family, f"{name}: [{table}] names `rspice-veriloga`")

    def test_veriloga_in_xspice_passes(self) -> None:
        family = baseline()
        xspice = copy.deepcopy(family["rspice-core-netlist"])
        xspice["package"]["name"] = "rspice-core-xspice"
        del xspice["dev-dependencies"]
        xspice["dependencies"]["rspice-veriloga"] = {"path": "../rspice-veriloga", "optional": True}
        family["rspice-core-xspice"] = xspice
        core = family["rspice-core"]
        core["dependencies"]["rspice-core-xspice"] = family_edge("rspice-core-xspice")
        for feature in ("parallel", "veriloga"):
            core["features"][feature].append(f"rspice-core-xspice/{feature}")
        self.assertEqual(problems(family), [])

    def test_an_upward_or_sideways_edge_fails(self) -> None:
        family = baseline()
        base = family["rspice-core-base"]
        base["dependencies"]["rspice-core-netlist"] = family_edge("rspice-core-netlist")
        for feature in ("parallel", "veriloga"):
            base["features"][feature].append(f"rspice-core-netlist/{feature}")
        self.assertOneProblem(family, "rspice-core-base: [dependencies] depends on `rspice-core-netlist`")

        family = baseline()
        for name in ("rspice-core-devices", "rspice-core-xspice"):
            crate = copy.deepcopy(family["rspice-core-netlist"])
            crate["package"]["name"] = name
            del crate["dev-dependencies"]
            family[name] = crate
        devices = family["rspice-core-devices"]
        devices["dependencies"]["rspice-core-xspice"] = family_edge("rspice-core-xspice")
        for feature in ("parallel", "veriloga"):
            devices["features"][feature].append(f"rspice-core-xspice/{feature}")
        self.assertOneProblem(family, "rspice-core-devices: [dependencies] depends on `rspice-core-xspice`")

    def test_a_dev_dependency_on_a_crate_above_fails(self) -> None:
        family = baseline()
        family["rspice-core-base"]["dev-dependencies"] = {"rspice-core": family_edge("rspice-core")}
        self.assertOneProblem(family, "would link a second copy of the crate")

    def test_test_support_enabled_by_a_normal_or_build_edge_fails(self) -> None:
        for table in ("dependencies", "build-dependencies", "target.'cfg(unix)'.dependencies"):
            with self.subTest(table=table):
                family = baseline()
                netlist = family["rspice-core-netlist"]
                if table == "dependencies":
                    netlist[table]["rspice-core-base"]["features"] = ["test-support"]
                else:
                    netlist[table] = {"rspice-core-base": family_edge("rspice-core-base", features=["test-support"])}
                    if table.startswith("target."):
                        del netlist["dependencies"]
                self.assertOneProblem(
                    family, f"rspice-core-netlist: [{table}] `rspice-core-base` enables `test-support`"
                )

    def test_test_support_enabled_by_another_feature_fails(self) -> None:
        family = baseline()
        family["rspice-core-netlist"]["features"]["parallel"].append("rspice-core-base/test-support")
        self.assertOneProblem(family, "feature `parallel` enables `rspice-core-base/test-support`")

        family = baseline()
        family["rspice-core-base"]["features"]["default"].append("test-support")
        self.assertOneProblem(family, "rspice-core-base: feature `default` enables `test-support`")

    def test_test_support_inherited_into_a_normal_edge_fails(self) -> None:
        family = baseline()
        family["workspace"]["workspace.dependencies"]["rspice-core-base"]["features"] = ["test-support"]
        found = problems(family)
        # netlist's and core's normal edges; core's dev-dependency may.
        self.assertEqual(len(found), 2, found)
        self.assertTrue(all("enables `test-support`" in p for p in found))

    def test_a_differing_or_missing_clippy_configuration_fails(self) -> None:
        self.assertOneProblem(
            baseline(),
            "rspice-core-base: clippy.toml differs from rspice-core's",
            clippy={"rspice-core-base": CLIPPY + "too-many-arguments-threshold = 9\n"},
        )
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            family = baseline()
            (root / "Cargo.toml").write_text(toml_document(family.pop("workspace")), encoding="utf-8")
            for name, tables in family.items():
                crate = root / "crates" / name
                crate.mkdir(parents=True)
                (crate / "Cargo.toml").write_text(toml_document(tables), encoding="utf-8")
                if name != "rspice-core-netlist":
                    (crate / "clippy.toml").write_text(CLIPPY, encoding="utf-8")
            self.assertEqual(check(root), ["rspice-core-netlist: has no clippy.toml; the family shares rspice-core's"])


if __name__ == "__main__":
    unittest.main()
