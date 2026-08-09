"""Offline qualification for the shipped browser Python runtime."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import re
import struct
import sys
import types
import unittest
from unittest import mock
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
WEB = ROOT / "crates" / "rspice-ui" / "web"
RUNTIME = WEB / "python" / "pyodide-314.0.2"
MANIFEST = RUNTIME / "rspice-runtime-manifest.json"
WORKER = WEB / "automation-worker.js"
BOOTSTRAP = WEB / "python" / "rspice_browser_bootstrap.py"
NATIVE_WORKER = ROOT / "assets" / "automation_runtime" / "rspice_worker.py"
RUST_BROWSER_RUNTIME = ROOT / "crates" / "rspice-ui" / "src" / "automation_runtime_browser.rs"
RUST_NATIVE_RUNTIME = ROOT / "crates" / "rspice-ui" / "src" / "automation_runtime.rs"
NATIVE_RUNTIME_LIBRARY = ROOT / "crates" / "rspice-automation-runtime" / "src" / "lib.rs"
NATIVE_RUNTIME_QUALIFIER = (
    ROOT
    / "crates"
    / "rspice-automation-runtime"
    / "src"
    / "bin"
    / "rspice-managed-runtime-qualifier.rs"
)
NATIVE_RELEASE = ROOT / ".github" / "workflows" / "native-release.yml"
DEBUGGER_HARNESS = ROOT / "tools" / "ci" / "automation_browser_debugger_harness.html"
PROJECT_CHECKPOINT = (
    ROOT
    / "crates"
    / "rspice-ui"
    / "src"
    / "workbench"
    / "lifecycle"
    / "project_checkpoint.rs"
)
BROWSER_PERSISTENCE = (
    ROOT
    / "crates"
    / "rspice-ui"
    / "src"
    / "workbench"
    / "lifecycle"
    / "project_lifecycle"
    / "persistence"
    / "browser.rs"
)
ENVIRONMENT_DIGEST = (
    "d445b1443965be4e6b1b191ee023176dbd35430ac3cd00603458384ea03b8518"
)
EXECUTION_ASSETS = (
    "python/pyodide-314.0.2/pyodide.mjs",
    "python/pyodide-314.0.2/pyodide.asm.mjs",
    "python/pyodide-314.0.2/pyodide.asm.wasm",
    "python/pyodide-314.0.2/python_stdlib.zip",
    "python/pyodide-314.0.2/pyodide-lock.json",
    "python/rspice_browser_bootstrap.py",
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def frame(digest: "hashlib._Hash", value: bytes) -> None:
    digest.update(struct.pack(">Q", len(value)))
    digest.update(value)


def read_unsigned_leb128(data: bytes, offset: int) -> tuple[int, int]:
    value = 0
    shift = 0
    while offset < len(data) and shift <= 63:
        byte = data[offset]
        offset += 1
        value |= (byte & 0x7F) << shift
        if not byte & 0x80:
            return value, offset
        shift += 7
    raise ValueError("invalid unsigned LEB128")


def defined_wasm_memories(data: bytes) -> list[tuple[int, int | None]]:
    if data[:8] != b"\x00asm\x01\x00\x00\x00":
        raise ValueError("invalid WebAssembly header")
    memories: list[tuple[int, int | None]] = []
    offset = 8
    while offset < len(data):
        section_id = data[offset]
        offset += 1
        section_size, offset = read_unsigned_leb128(data, offset)
        section_end = offset + section_size
        if section_end > len(data):
            raise ValueError("truncated WebAssembly section")
        if section_id == 5:
            count, cursor = read_unsigned_leb128(data, offset)
            for _ in range(count):
                flags, cursor = read_unsigned_leb128(data, cursor)
                minimum, cursor = read_unsigned_leb128(data, cursor)
                maximum = None
                if flags & 1:
                    maximum, cursor = read_unsigned_leb128(data, cursor)
                memories.append((minimum, maximum))
            if cursor != section_end:
                raise ValueError("unexpected WebAssembly memory-section payload")
        offset = section_end
    return memories


def runtime_digest() -> str:
    digest = hashlib.sha256(b"rspice.browser-python-runtime/v1")
    for relative in sorted(EXECUTION_ASSETS):
        frame(digest, relative.encode("utf-8"))
        frame(digest, (WEB / relative).read_bytes())
    return digest.hexdigest()


class BrowserAutomationRuntimeAssets(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
        cls.worker = WORKER.read_text(encoding="utf-8")

    def test_manifest_is_closed_and_every_file_matches(self) -> None:
        declared = {entry["path"]: entry for entry in self.manifest["files"]}
        actual = {
            path.name
            for path in RUNTIME.iterdir()
            if path.name != MANIFEST.name
        }
        self.assertEqual(actual, set(declared))
        for name, entry in declared.items():
            path = RUNTIME / name
            self.assertEqual(path.stat().st_size, entry["bytes"], name)
            self.assertEqual(sha256(path), entry["sha256"], name)

    def test_runtime_identity_authenticates_every_execution_asset(self) -> None:
        expected = self.manifest["runtime_digest_sha256"]
        self.assertEqual(runtime_digest(), expected)
        self.assertRegex(expected, r"^[0-9a-f]{64}$")
        self.assertIn(f'"{expected}"', self.worker)
        rust = RUST_BROWSER_RUNTIME.read_text(encoding="utf-8")
        match = re.search(
            r"const EXPECTED_RUNTIME_DIGEST: \[u8; 32\] = \[(.*?)\];",
            rust,
            re.DOTALL,
        )
        self.assertIsNotNone(match)
        encoded = bytes(
            int(value, 16)
            for value in re.findall(r"0x([0-9a-f]{2})", match.group(1))
        )
        self.assertEqual(encoded.hex(), expected)

    def test_worker_pins_versions_environment_and_each_core_asset(self) -> None:
        self.assertIn('const PYODIDE_VERSION = "314.0.2";', self.worker)
        self.assertIn('const PYTHON_VERSION = "3.14.2";', self.worker)
        self.assertIn(f'"{ENVIRONMENT_DIGEST}"', self.worker)
        self.assertIn('python/pyodide-314.0.2/', self.worker)
        for entry in self.manifest["files"]:
            if not entry["execution_asset"]:
                continue
            self.assertIn(entry["path"], self.worker)
            self.assertIn(str(entry["bytes"]), self.worker)
            self.assertIn(entry["sha256"], self.worker)

    def test_worker_pins_and_verifies_the_trusted_bootstrap(self) -> None:
        match = re.search(
            r'const BOOTSTRAP = Object\.freeze\(\[\s*"([^"]+)",\s*'
            r"(\d+),\s*\"([0-9a-f]{64})\"",
            self.worker,
        )
        self.assertIsNotNone(match)
        name, length, digest = match.groups()
        self.assertEqual(name, BOOTSTRAP.name)
        self.assertEqual(int(length), BOOTSTRAP.stat().st_size)
        self.assertEqual(digest, sha256(BOOTSTRAP))

    def test_run_plan_lookup_uses_portable_project_path_identity(self) -> None:
        expected = '_logical_path_key(str(item.get("logical_path", "")))'
        for adapter in (BOOTSTRAP, NATIVE_WORKER):
            source = adapter.read_text(encoding="utf-8")
            self.assertIn(expected, source, adapter.as_posix())
            self.assertIn("== _logical_path_key(logical_path)", source, adapter.as_posix())
            self.assertIn("unicodedata.normalize", source, adapter.as_posix())
            self.assertNotIn(
                'item.get("logical_path") == logical_path',
                source,
                adapter.as_posix(),
            )

    def test_run_plan_lookup_executes_with_unicode_project_path_identity(self) -> None:
        pyodide = types.ModuleType("pyodide")
        pyodide_ffi = types.ModuleType("pyodide.ffi")
        pyodide_ffi.run_sync = lambda value: value
        bridge = types.ModuleType("rspice_bridge")
        bridge.debug_checkpoint = lambda *args: None
        bridge.debug_exchange = lambda *args: None
        bridge.host_call = lambda *args: None
        stubs = {
            "pyodide": pyodide,
            "pyodide.ffi": pyodide_ffi,
            "rspice_bridge": bridge,
        }

        class Session:
            def __init__(self, retained_path: str) -> None:
                self.snapshot = {
                    "documents": [
                        {
                            "logical_path": retained_path,
                            "role": "run-plan",
                            "document_id": "qualified-run-plan-id",
                        }
                    ]
                }
                self.calls = []

            def call(self, capability, operation):
                self.calls.append((capability, operation))
                return "qualified-run-plan-handle"

        for index, adapter in enumerate((BOOTSTRAP, NATIVE_WORKER)):
            name = f"rspice_qualified_adapter_{index}"
            spec = importlib.util.spec_from_file_location(name, adapter)
            self.assertIsNotNone(spec)
            self.assertIsNotNone(spec.loader)
            module = importlib.util.module_from_spec(spec)
            with mock.patch.dict(sys.modules, stubs):
                spec.loader.exec_module(module)

            for retained_path, alias in (
                ("Plans/\u00c9t\u00e9.yaml", "PLANS/\u00e9T\u00c9.YAML"),
                ("Plans/Stra\u00dfe.yaml", "PLANS/STRASSE.YAML"),
                ("Plans/caf\u00e9.yaml", "PLANS/CAFE\u0301.YAML"),
            ):
                session = Session(retained_path)
                run_plan = module._RunPlans(session, "qualified-project").load(alias)
                self.assertEqual(run_plan._handle, "qualified-run-plan-handle")
                self.assertEqual(
                    session.calls,
                    [
                        (
                            "project-read",
                            {
                                "operation": "load-run-plan",
                                "project_handle": "qualified-project",
                                "document_id": "qualified-run-plan-id",
                            },
                        )
                    ],
                    adapter.as_posix(),
                )

    def test_release_provenance_and_offline_runtime_policy_are_explicit(self) -> None:
        upstream = self.manifest["upstream_release"]
        self.assertEqual(upstream["bytes"], 6_767_493)
        self.assertEqual(
            upstream["sha256"],
            "86e3d5e0cbd39b1def1e424b3f1abdcc9edc66ae200fa5280ae8825bf71799ec",
        )
        self.assertFalse(self.manifest["external_network_required_at_runtime"])
        self.assertFalse((WEB / "python" / "pyodide-0.26.4").exists())

    def test_debugger_qualification_covers_the_stateful_adapter_contract(self) -> None:
        harness = DEBUGGER_HARNESS.read_text(encoding="utf-8")
        for operation in (
            'operation: "launch"',
            'mode: "debug"',
            'operation: "stack-trace"',
            'operation: "variables"',
            'operation: "evaluate"',
            'control: "step-over"',
            'control: "continue"',
        ):
            self.assertIn(operation, harness)
        self.assertIn("RSPICE_AUTOMATION_DEBUGGER_QUALIFICATION", harness)
        self.assertIn('kind: { kind: "stop" }', harness)

    def test_cold_runtime_startup_is_bounded_separately_from_user_code(self) -> None:
        source = RUST_BROWSER_RUNTIME.read_text(encoding="utf-8")
        self.assertIn("const RUNTIME_STARTUP_LIMIT_MS: f64 = 120_000.0;", source)
        self.assertIn("runtime_ready: bool", source)
        self.assertIn("runtime_ready: self.runtime_ready", source)
        self.assertIn("if self.runtime_ready", source)
        self.assertIn("limits.wall_time_ms as f64", source)
        self.assertIn("RUNTIME_STARTUP_LIMIT_MS", source)
        self.assertIn("self.runtime_ready = true;", source)
        self.assertIn("self.runtime_ready = false;", source)
        self.assertIn("did not initialize within", source)
        self.assertIn("resource clock failed closed", source)
        self.assertIn('return Err("a browser Python launch is already active"', source)
        self.assertIn("let _ = self.terminate();", source)

    def test_browser_resource_policy_hard_bounds_the_verified_pyodide_memory(self) -> None:
        wasm = (RUNTIME / "pyodide.asm.wasm").read_bytes()
        self.assertEqual(defined_wasm_memories(wasm), [(480, 65_536)])
        self.assertEqual(
            self.manifest["execution_policy"],
            {
                "wasm_memory_minimum_pages": 480,
                "upstream_wasm_memory_maximum_pages": 65_536,
                "rspice_wasm_memory_maximum_pages": 32_768,
                "rspice_wasm_memory_maximum_bytes": 2_147_483_648,
                "application": "after SHA-256 verification, before WebAssembly instantiation",
            },
        )
        for token in (
            "const BROWSER_MEMORY_LIMIT_BYTES = 2 * 1024 * 1024 * 1024;",
            "const UPSTREAM_PYODIDE_MAXIMUM_MEMORY_PAGES = 65_536;",
            "const MAX_BROWSER_OUTPUT_BYTES = 16 * 1024 * 1024;",
            "const MAX_BROWSER_ARTIFACT_BYTES = 512 * 1024 * 1024;",
            "const MAX_BROWSER_STACK_DEPTH = 4_000;",
            "function applyPyodideMemoryLimit(source)",
            "applyPyodideMemoryLimit(verifiedAssets[2])",
            "createUpstreamPyodideModule({ ...settings, wasmBinary })",
            'limits.memory_bytes !== BROWSER_MEMORY_LIMIT_BYTES',
            'limits.cpu_time_ms !== limits.wall_time_ms',
            'limits.max_tasks !== 1',
            'limits[field] > maximum',
        ):
            self.assertIn(token, self.worker)

        bootstrap = BOOTSTRAP.read_text(encoding="utf-8")
        self.assertIn("max_stack_depth: int", bootstrap)
        self.assertIn("sys.setrecursionlimit(min(max_stack_depth, 100_000))", bootstrap)
        self.assertIn("sys.setrecursionlimit(original_recursion_limit)", bootstrap)

        source = RUST_BROWSER_RUNTIME.parent.joinpath(
            "workbench", "documents", "code_workspace", "automation.rs"
        ).read_text(encoding="utf-8")
        browser_validation = source.split(
            '#[cfg(target_arch = "wasm32")]\nfn begin_managed_python_validation', 1
        )[1]
        self.assertIn("cpu_time_ms: 30_000", browser_validation)
        self.assertIn("memory_bytes: 2 * 1024 * 1024 * 1024", browser_validation)

    def test_browser_checkpoint_catalog_streams_project_sized_snapshots(self) -> None:
        persistence = BROWSER_PERSISTENCE.read_text(encoding="utf-8")
        checkpoint = PROJECT_CHECKPOINT.read_text(encoding="utf-8")
        self.assertIn(
            "mut visit: impl FnMut(String, Option<String>, String, Option<Vec<u8>>)",
            persistence,
        )
        self.assertIn(
            "visit(manifest_key, manifest, snapshot_key, snapshot);",
            persistence,
        )
        self.assertNotIn("let mut records = Vec::with_capacity(manifest_keys.len())", persistence)
        self.assertIn("fn visit_browser_checkpoint_record(", checkpoint)
        self.assertIn("BrowserCheckpointAccumulator", checkpoint)
        self.assertIn("std::mem::take(&mut accumulator.checkpoints)", checkpoint)

    def test_browser_cancellation_has_cooperative_and_hard_termination_paths(self) -> None:
        runtime = RUST_BROWSER_RUNTIME.read_text(encoding="utf-8")
        automation = RUST_BROWSER_RUNTIME.parent.joinpath(
            "workbench", "documents", "code_workspace", "automation.rs"
        ).read_text(encoding="utf-8")
        self.assertIn("self.worker = None;", runtime)
        self.assertIn("self.active_limits = None;", runtime)
        self.assertIn("self.events.borrow_mut().clear();", runtime)
        self.assertIn("let _ = app.automation_runtime.terminate();", automation)
        self.assertIn('case "cancel":', self.worker)
        self.assertIn('enqueueDebugCommand({ operation: "stop" });', self.worker)
        self.assertIn(
            'pending.reject(new Error("Automation session was cancelled."));',
            self.worker,
        )

    def test_native_runtime_uses_one_fresh_resource_boundary_per_launch(self) -> None:
        transport = RUST_NATIVE_RUNTIME.read_text(encoding="utf-8")
        runtime = NATIVE_RUNTIME_LIBRARY.read_text(encoding="utf-8")
        automation = RUST_BROWSER_RUNTIME.parent.joinpath(
            "workbench", "documents", "code_workspace", "automation.rs"
        ).read_text(encoding="utf-8")
        qualifier = NATIVE_RUNTIME_QUALIFIER.read_text(encoding="utf-8")
        release = NATIVE_RELEASE.read_text(encoding="utf-8")

        self.assertIn('return Err("a managed Python launch is already active"', transport)
        self.assertIn("partially installed", transport)
        self.assertIn("let _ = self.terminate();", transport)
        self.assertIn("resource_limits_applied: bool", runtime)
        self.assertIn("RuntimeError::ResourceLimitsAlreadyApplied", runtime)
        self.assertIn('cfg(not(target_arch = "wasm32"))', automation)
        self.assertIn("always launch execution in", automation)
        for token in (
            "qualify_completed_launch",
            "qualify_hard_cancellation",
            "qualify_watchdog_termination",
            "ResourceLimitsAlreadyApplied",
            "system_python_path_used",
        ):
            self.assertIn(token, qualifier)
        self.assertIn("rspice-managed-runtime-qualifier", release)
        self.assertIn("Qualify signed managed CPython execution and cancellation", release)


if __name__ == "__main__":
    unittest.main()
