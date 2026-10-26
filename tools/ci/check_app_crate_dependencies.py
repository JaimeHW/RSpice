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
    "rspice-vocabulary": set(),
    "rspice-results": {"rspice-vocabulary", "rspice-units"},
    "rspice-model-library": {"rspice-vocabulary", "rspice-results"},
    "rspice-simulation-contract": {
        "rspice-vocabulary", "rspice-results", "rspice-model-library",
    },
    "rspice-hardcopy-contract": {"rspice-vocabulary", "rspice-results"},
    "rspice-design": {
        "rspice-vocabulary", "rspice-model-library", "rspice-hardcopy-contract",
    },
    "rspice-project-contract": {
        "rspice-vocabulary", "rspice-design", "rspice-model-library",
    },
    "rspice-project": {
        "rspice-project-contract",
        "rspice-vocabulary", "rspice-design", "rspice-model-library",
        "rspice-simulation-contract", "rspice-results", "rspice-hardcopy-contract",
        "rspice-formats",
    },
    "rspice-formats": {"rspice-vocabulary", "rspice-results", "rspice-model-library"},
    "rspice-simulation": {
        "rspice-project-contract",
        "rspice-vocabulary", "rspice-design", "rspice-model-library",
        "rspice-simulation-contract", "rspice-results", "rspice-formats",
    },
    "rspice-hardcopy": {
        "rspice-vocabulary", "rspice-hardcopy-contract", "rspice-design", "rspice-project-contract",
        "rspice-results", "rspice-formats",
    },
    "rspice-ui-kit": {"rspice-vocabulary", "rspice-results"},
    "rspice-schematic-editor": {
        "rspice-vocabulary", "rspice-design", "rspice-model-library",
        "rspice-ui-kit", "rspice-simulation-contract",
    },
    "rspice-results-ui": {"rspice-vocabulary", "rspice-results", "rspice-ui-kit"},
    # The persisted model: every lower crate the application's state is made of,
    # and no host or window crate.
    "rspice-app-state": {
        "rspice-vocabulary", "rspice-ui-kit", "rspice-schematic-editor", "rspice-results-ui",
        "rspice-design", "rspice-project", "rspice-results", "rspice-hardcopy-contract",
        "rspice-model-library", "rspice-formats", "rspice-simulation-contract",
        "rspice-simulation",
    },
    # The platform services: the persisted model and the lower crates they read
    # and write it with, and no feature, window or shell crate.
    "rspice-app-services": {
        "rspice-app-state", "rspice-vocabulary", "rspice-ui-kit", "rspice-schematic-editor",
        "rspice-results-ui", "rspice-design", "rspice-project", "rspice-results",
        "rspice-hardcopy-contract", "rspice-model-library", "rspice-formats",
        "rspice-simulation-contract", "rspice-simulation",
    },
    # The run tier: the persisted model, the platform services, and the engine
    # and result crates a run is prepared, dispatched and published with; no
    # feature, window or shell crate. The schematic editor is a test-support
    # dependency only (the run test host's review-marker visibility).
    "rspice-app-runs": {
        "rspice-app-state", "rspice-app-services", "rspice-vocabulary", "rspice-design",
        "rspice-results", "rspice-results-ui", "rspice-model-library", "rspice-formats",
        "rspice-simulation-contract", "rspice-simulation", "rspice-schematic-editor",
    },
    "rspice-worker": {
        "rspice-vocabulary", "rspice-results", "rspice-model-library",
        "rspice-simulation-contract", "rspice-hardcopy-contract", "rspice-design",
        "rspice-project-contract", "rspice-formats", "rspice-simulation", "rspice-hardcopy",
    },
}
HEADLESS = set(ALLOWED) - {
    "rspice-ui-kit", "rspice-schematic-editor", "rspice-results-ui", "rspice-app-state",
    "rspice-app-services", "rspice-app-runs",
}
GUI_PACKAGES = {"eframe", "winit", "rfd", "wgpu"}
ENGINE_PACKAGES = {
    "rspice-core", "rspice-core-base", "rspice-core-netlist", "rspice-core-devices",
    "rspice-matrix", "rspice-veriloga",
    "rspice-veriloga-runtime", "rspice-veriloga-models",
}
INCLUDE_DEV = {"rspice-worker", "rspice-ui-kit", "rspice-schematic-editor", "rspice-results-ui"}
PACKAGE_LINE = re.compile(r"^([A-Za-z0-9_-]+) v\d")


def violations(
    direct: dict[str, set[str]], closures: dict[str, set[str]],
    portable_formats: set[str] | None = None,
    native_bundle_formats: set[str] | None = None,
    table_schema_formats: set[str] | None = None,
    normal_direct: dict[str, set[str]] | None = None,
) -> list[str]:
    """Validate the package graph; absent planned crates need no placeholder."""
    issues = []
    # The viewers name the engine's result types through rspice-results; only
    # their tests check what they build against the engine itself.
    for dep in sorted((normal_direct or {}).get("rspice-results-ui", set()) & ENGINE_PACKAGES):
        issues.append(f"rspice-results-ui names engine package {dep} as a normal dependency")
    application_names = set(ALLOWED) | {"rspice-app"}
    for name, deps in sorted(direct.items()):
        unexpected = (deps & application_names) - ALLOWED[name]
        for dep in sorted(unexpected):
            issues.append(f"{name} has forbidden application dependency {dep}")
    for name, closure in sorted(closures.items()):
        if name == "rspice-worker":
            for dep in sorted(
                closure & {"rspice-app", "rspice-app-state", "rspice-app-services", "rspice-app-runs"}
            ):
                issues.append(f"rspice-worker reaches application package {dep}")
        if name == "rspice-app-state":
            for dep in sorted(closure & ({"rspice-app", "rspice-worker"} | GUI_PACKAGES)):
                issues.append(f"rspice-app-state reaches application or host package {dep}")
        if name == "rspice-app-services":
            # The native file pickers are a platform service; the window is not.
            for dep in sorted(closure & ({"rspice-app", "rspice-worker"} | (GUI_PACKAGES - {"rfd"}))):
                issues.append(f"rspice-app-services reaches application or host package {dep}")
        if name == "rspice-app-runs":
            # The services' file pickers come with them; the window does not.
            for dep in sorted(closure & ({"rspice-app", "rspice-worker"} | (GUI_PACKAGES - {"rfd"}))):
                issues.append(f"rspice-app-runs reaches application or host package {dep}")
        if name in HEADLESS:
            for dep in sorted(dep for dep in closure if dep.startswith("egui") or dep in GUI_PACKAGES):
                issues.append(f"{name} reaches GUI package {dep}")
        if name in {"rspice-simulation", "rspice-project-contract", "rspice-worker"} and "rspice-project" in closure:
            issues.append(f"{name} reaches project aggregate rspice-project")
        if name in {"rspice-vocabulary", "rspice-units"}:
            for dep in sorted(
                dep for dep in closure
                if dep in ENGINE_PACKAGES or dep.startswith("rspice-veriloga-model-")
            ):
                issues.append(f"{name} reaches simulator package {dep}")
        if name in {"rspice-schematic-editor", "rspice-results-ui"}:
            forbidden = {
                "rspice-app", "rspice-app-state", "rspice-app-services", "rspice-app-runs",
                "rspice-project", "rspice-project-contract", "rspice-worker", "rspice-simulation",
            } | GUI_PACKAGES
            for dep in sorted(closure & forbidden):
                issues.append(f"{name} reaches application service or host package {dep}")
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
    normal_direct = {}
    closures = {}
    for package in metadata["packages"]:
        name = package["name"]
        if name not in ALLOWED:
            continue
        direct[name] = {
            dep["name"] for dep in package["dependencies"]
            if dep["kind"] != "dev" or name in INCLUDE_DEV
        }
        normal_direct[name] = {
            dep["name"] for dep in package["dependencies"] if dep["kind"] is None
        }
        tree = run(
            "cargo", "tree", "--locked", "-p", name, "--all-features",
            "--target", "all", "--edges",
            "normal,build,dev" if name in INCLUDE_DEV else "normal,build",
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
        portable_formats.get("table-schema"), normal_direct,
    )
    if issues:
        print("\n".join(issues))
        return 1
    print(f"Application crate dependency policy: {len(direct)} extracted crate(s) checked")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
