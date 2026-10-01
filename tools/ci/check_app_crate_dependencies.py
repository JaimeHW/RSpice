#!/usr/bin/env python3
"""Keep extracted application crates below their declared owners."""

from __future__ import annotations

import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

# Direct application-layer edges. Existing engine/trust crates are checked by
# their own boundaries; these rows forbid a new extracted crate reaching up.
ALLOWED: dict[str, set[str]] = {
    "rspice-units": set(),
    "rspice-app-types": set(),
    "rspice-results": {"rspice-app-types", "rspice-units"},
    "rspice-model-library": {"rspice-app-types", "rspice-results"},
    "rspice-simulation-contract": {
        "rspice-app-types", "rspice-results", "rspice-model-library",
    },
    "rspice-hardcopy-contract": {"rspice-app-types", "rspice-results"},
    "rspice-design": {
        "rspice-app-types", "rspice-model-library", "rspice-hardcopy-contract",
    },
    "rspice-project-contract": {
        "rspice-app-types", "rspice-design", "rspice-model-library",
    },
    "rspice-project": {
        "rspice-project-contract",
        "rspice-app-types", "rspice-design", "rspice-model-library",
        "rspice-simulation-contract", "rspice-results", "rspice-hardcopy-contract",
        "rspice-formats",
    },
    "rspice-formats": {"rspice-app-types", "rspice-results", "rspice-model-library"},
    "rspice-simulation": {
        "rspice-project-contract",
        "rspice-app-types", "rspice-design", "rspice-model-library",
        "rspice-simulation-contract", "rspice-results", "rspice-formats",
    },
    "rspice-hardcopy": {
        "rspice-app-types", "rspice-hardcopy-contract", "rspice-design", "rspice-project-contract",
        "rspice-results", "rspice-formats",
    },
    "rspice-ui-kit": {"rspice-app-types", "rspice-results"},
    "rspice-schematic-editor": {
        "rspice-app-types", "rspice-design", "rspice-model-library",
        "rspice-ui-kit", "rspice-simulation-contract",
    },
    "rspice-results-ui": {"rspice-app-types", "rspice-results", "rspice-ui-kit"},
    "rspice-worker": {
        "rspice-app-types", "rspice-results", "rspice-model-library",
        "rspice-simulation-contract", "rspice-hardcopy-contract", "rspice-design",
        "rspice-project-contract", "rspice-formats", "rspice-simulation", "rspice-hardcopy",
    },
}
HEADLESS = set(ALLOWED) - {
    "rspice-ui-kit", "rspice-schematic-editor", "rspice-results-ui",
}
GUI_PACKAGES = {"eframe", "winit", "rfd", "wgpu"}
ENGINE_PACKAGES = {
    "rspice-core", "rspice-matrix", "rspice-veriloga",
    "rspice-veriloga-runtime", "rspice-veriloga-models",
}
PACKAGE_LINE = re.compile(r"^([A-Za-z0-9_-]+) v\d")


def violations(
    direct: dict[str, set[str]], closures: dict[str, set[str]],
    portable_formats: set[str] | None = None,
    native_bundle_formats: set[str] | None = None,
    table_schema_formats: set[str] | None = None,
) -> list[str]:
    """Validate the package graph; absent planned crates need no placeholder."""
    issues = []
    application_names = set(ALLOWED) | {"rspice-app"}
    for name, deps in sorted(direct.items()):
        unexpected = (deps & application_names) - ALLOWED[name]
        for dep in sorted(unexpected):
            issues.append(f"{name} has forbidden application dependency {dep}")
    for name, closure in sorted(closures.items()):
        if name == "rspice-worker" and "rspice-app" in closure:
            issues.append("rspice-worker reaches application package rspice-app")
        if name in HEADLESS:
            for dep in sorted(dep for dep in closure if dep.startswith("egui") or dep in GUI_PACKAGES):
                issues.append(f"{name} reaches GUI package {dep}")
        if name in {"rspice-simulation", "rspice-project-contract", "rspice-worker"} and "rspice-project" in closure:
            issues.append(f"{name} reaches project aggregate rspice-project")
        if name in {"rspice-app-types", "rspice-units"}:
            for dep in sorted(
                dep for dep in closure
                if dep in ENGINE_PACKAGES or dep.startswith("rspice-veriloga-model-")
            ):
                issues.append(f"{name} reaches simulator package {dep}")
        if name == "rspice-ui-kit":
            forbidden = (
                application_names - {name, "rspice-units"} - ALLOWED[name]
            ) | ENGINE_PACKAGES | GUI_PACKAGES
            for dep in sorted(
                dep for dep in closure
                if dep in forbidden or dep.startswith("rspice-veriloga-model-")
            ):
                issues.append(f"rspice-ui-kit reaches non-presentation package {dep}")
    for label, dependencies in (
        ("without optional features", portable_formats),
        ("with only native-bundle", native_bundle_formats),
        ("with only table-schema", table_schema_formats),
    ):
        for dep in sorted(dependencies or set()):
            if dep in ENGINE_PACKAGES or dep.startswith("rspice-veriloga-model-"):
                issues.append(f"rspice-formats {label} reaches simulator package {dep}")
    return issues


def run(*args: str) -> str:
    return subprocess.check_output(args, cwd=ROOT, text=True)


def main() -> int:
    metadata = json.loads(run("cargo", "metadata", "--locked", "--no-deps", "--format-version", "1"))
    direct = {}
    closures = {}
    for package in metadata["packages"]:
        name = package["name"]
        if name not in ALLOWED:
            continue
        direct[name] = {
            dep["name"] for dep in package["dependencies"]
            if dep["kind"] != "dev" or name in {"rspice-worker", "rspice-ui-kit"}
        }
        tree = run(
            "cargo", "tree", "--locked", "-p", name, "--all-features",
            "--target", "all", "--edges",
            "normal,build,dev" if name in {"rspice-worker", "rspice-ui-kit"} else "normal,build",
            "--prefix", "none",
            "--format", "{p}",
        )
        closures[name] = {
            match.group(1)
            for line in tree.splitlines()
            if (match := PACKAGE_LINE.match(line))
        }
    portable_formats = {}
    if "rspice-formats" in direct:
        for feature in ("", "native-bundle", "table-schema"):
            selection = ("--features", feature) if feature else ()
            tree = run(
                "cargo", "tree", "--locked", "-p", "rspice-formats", "--no-default-features",
                "--target", "all", "--edges", "normal,build", "--prefix", "none",
                "--format", "{p}", *selection,
            )
            portable_formats[feature] = {
                match.group(1) for line in tree.splitlines()
                if (match := PACKAGE_LINE.match(line))
            }
    issues = violations(
        direct, closures, portable_formats.get(""), portable_formats.get("native-bundle"),
        portable_formats.get("table-schema"),
    )
    if issues:
        print("\n".join(issues))
        return 1
    print(f"Application crate dependency policy: {len(direct)} extracted crate(s) checked")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
