#!/usr/bin/env python3
"""Check the manifests of the rspice-core crate family against each other.

The engine is crates/rspice-core plus every crates/rspice-core-<name> beside
it. A feature one crate declares and another does not forward reads false in
the second crate without any error, so a conformance suite can pass with the
code it means to test compiled out. Every family manifest is read and these
fail:

- a `[features]` key set other than rspice-core's, and a crate below
  rspice-core without its own `test-support`;
- a feature, other than `default` and `test-support`, that does not forward to
  the same feature of every family crate the crate depends on;
- a dependency on a family crate without `default-features = false`;
- `rspice-veriloga` named by base, netlist or devices, even optionally;
- a dependency of any kind on a family crate that is not below the dependent
  (a crate may dev-depend on itself);
- `test-support` enabled anywhere but a dev-dependency or another crate's
  `test-support` feature;
- a family `clippy.toml` that differs from rspice-core's.

This only reads: the tables are edited by hand. Finding no family manifest is
a failure, so a moved tree cannot turn the check into a check of nothing.
"""

from __future__ import annotations

import argparse
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

CORE = "rspice-core"
TEST_SUPPORT = "test-support"
# A crate may depend only on crates of a lower rank.
RANKS = {
    "rspice-core-base": 0,
    "rspice-core-netlist": 1,
    "rspice-core-devices": 2,
    "rspice-core-xspice": 2,
    "rspice-core-analysis": 3,
    CORE: 4,
}
# Feature unification would put these back behind the Verilog-A compiler.
VERILOGA = "rspice-veriloga"
VERILOGA_FREE = ("rspice-core-base", "rspice-core-netlist", "rspice-core-devices")
DEPENDENCY_TABLES = ("dependencies", "dev-dependencies", "build-dependencies")


def is_family(package: str) -> bool:
    return package == CORE or package.startswith(CORE + "-")


def family_manifests(root: Path) -> dict[str, tuple[Path, dict]]:
    """Each family crate's name, directory and parsed manifest."""
    found: dict[str, tuple[Path, dict]] = {}
    crates = root / "crates"
    if not crates.is_dir():
        return found
    for directory in sorted(crates.iterdir()):
        manifest = directory / "Cargo.toml"
        if not is_family(directory.name) or not manifest.is_file():
            continue
        parsed = tomllib.loads(manifest.read_text(encoding="utf-8"))
        name = parsed.get("package", {}).get("name", directory.name)
        found[name] = (directory, parsed)
    return found


def dependency_edges(manifest: dict):
    """(table, key, spec) for every dependency table, target tables included."""
    for table in DEPENDENCY_TABLES:
        for key, spec in manifest.get(table, {}).items():
            yield table, key, spec
    for target, tables in manifest.get("target", {}).items():
        for table in DEPENDENCY_TABLES:
            for key, spec in tables.get(table, {}).items():
                yield f"target.'{target}'.{table}", key, spec


def resolve(key: str, spec, workspace: dict) -> tuple[dict, str | None]:
    """The edge as cargo reads it: package, default features and features."""
    if isinstance(spec, str):
        spec = {"version": spec}
    if not spec.get("workspace"):
        return {
            "package": spec.get("package", key),
            "default-features": spec.get("default-features", True),
            "features": list(spec.get("features", [])),
        }, None
    if key not in workspace:
        return {"package": key, "default-features": True, "features": []}, (
            f"inherits `{key}` from the workspace, which declares no such dependency"
        )
    inherited = workspace[key]
    if isinstance(inherited, str):
        inherited = {"version": inherited}
    # An inherited edge can turn default features back on, never off: cargo
    # ignores a member's `default-features = false` when the workspace entry
    # leaves them on.
    default = inherited.get("default-features", True) or spec.get("default-features", False)
    return {
        "package": inherited.get("package", key),
        "default-features": default,
        "features": list(inherited.get("features", [])) + list(spec.get("features", [])),
    }, None


def check(root: Path) -> list[str]:
    family = family_manifests(root)
    if not family:
        return [f"no rspice-core family manifest under {root / 'crates'}; the check read nothing"]
    if CORE not in family:
        return [f"{CORE} is missing from the family under {root / 'crates'}"]

    root_manifest = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
    workspace = root_manifest.get("workspace", {}).get("dependencies", {})
    problems: list[str] = []

    for name in family:
        if name not in RANKS:
            problems.append(f"{name}: not in the family's crate order; give it a rank in RANKS")
    if problems:
        return problems

    core_features = family[CORE][1].get("features", {})
    expected = set(core_features) - {TEST_SUPPORT}
    core_clippy = family[CORE][0] / "clippy.toml"

    for name, (directory, manifest) in family.items():
        features = manifest.get("features", {})
        keys = set(features) - {TEST_SUPPORT}
        for feature in sorted(expected - keys):
            problems.append(f"{name}: does not declare the family feature `{feature}`")
        for feature in sorted(keys - expected):
            problems.append(f"{name}: declares `{feature}`, which {CORE} does not")
        if name != CORE and TEST_SUPPORT not in features:
            problems.append(f"{name}: does not declare `{TEST_SUPPORT}`")

        for feature, enables in features.items():
            if feature == TEST_SUPPORT:
                continue
            for entry in enables:
                if entry == TEST_SUPPORT or entry.endswith("/" + TEST_SUPPORT):
                    problems.append(
                        f"{name}: feature `{feature}` enables `{entry}`; "
                        f"`{TEST_SUPPORT}` is enabled only from dev-dependencies"
                    )

        for table, key, spec in dependency_edges(manifest):
            edge, problem = resolve(key, spec, workspace)
            package = edge["package"]
            if problem:
                problems.append(f"{name}: [{table}] {problem}")
            if package == VERILOGA and name in VERILOGA_FREE:
                problems.append(
                    f"{name}: [{table}] names `{VERILOGA}`, which only xspice, analysis "
                    f"and {CORE} may depend on"
                )
            if not is_family(package):
                continue
            dev = table.endswith("dev-dependencies")
            if package not in RANKS:
                problems.append(f"{name}: [{table}] `{package}` is not in the family's crate order")
                continue
            if not (dev and package == name) and RANKS[package] >= RANKS[name]:
                problems.append(
                    f"{name}: [{table}] depends on `{package}`, which is not below it"
                    + ("; its tests would link a second copy of the crate" if dev else "")
                )
            if edge["default-features"]:
                problems.append(
                    f"{name}: [{table}] `{key}` leaves default features on; a family edge "
                    "carries `default-features = false`"
                )
            if not dev and TEST_SUPPORT in edge["features"]:
                problems.append(
                    f"{name}: [{table}] `{key}` enables `{TEST_SUPPORT}`, which only "
                    "dev-dependencies may"
                )
            if table.endswith("build-dependencies") or dev:
                continue
            # A feature outside the family's set is already reported above.
            for feature in sorted((expected & keys) - {"default"}):
                enables = features[feature]
                if f"{key}/{feature}" not in enables and f"{key}?/{feature}" not in enables:
                    problems.append(f"{name}: feature `{feature}` does not forward to `{key}/{feature}`")

        clippy = directory / "clippy.toml"
        if name != CORE and core_clippy.is_file():
            if not clippy.is_file():
                problems.append(f"{name}: has no clippy.toml; the family shares {CORE}'s")
            elif clippy.read_bytes() != core_clippy.read_bytes():
                problems.append(f"{name}: clippy.toml differs from {CORE}'s")

    return problems


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    parser.add_argument("--root", type=Path, default=ROOT, help="workspace root")
    args = parser.parse_args()
    problems = check(args.root)
    if problems:
        print("rspice-core family manifests disagree:", file=sys.stderr)
        for problem in problems:
            print(f"  {problem}", file=sys.stderr)
        return 1
    names = ", ".join(family_manifests(args.root))
    print(f"rspice-core family manifests agree: {names}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
