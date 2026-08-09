"""Managed CPython protocol worker for RSpice Automation.

This file is packaged inside the signed runtime inventory. It never discovers
an interpreter, project directory, or package environment. Source arrives as
an immutable logical snapshot over the bounded native protocol.
"""

from __future__ import annotations

import json
import importlib.abc
import importlib.util
import io
import os
import ast
import re
import struct
import sys
import threading
import traceback
import types
import unicodedata
import uuid


PROTOCOL = {"major": 1, "minor": 4}
MAX_FRAME_BYTES = 72 * 1024 * 1024
MAX_OUTPUT_TEXT = 1024 * 1024
_sequence = 0
_PROTOCOL_STDIN = sys.stdin.buffer
_PROTOCOL_STDOUT = sys.stdout.buffer
_EVENT_LOCK = threading.Lock()
_ACTIVE_DEBUG_ADAPTER: _DebugAdapter | None = None


def _logical_path_key(value: str) -> str:
    """Match RSpice's locale-independent portable project-path identity."""
    return unicodedata.normalize(
        "NFC", unicodedata.normalize("NFC", value).upper().lower()
    )


class RSpiceHostError(RuntimeError):
    """A typed failure returned by the RSpice capability broker."""

    def __init__(self, code: str, message: str) -> None:
        super().__init__(f"{message} ({code})")
        self.code = code
        self.message = message


class RSpiceCancelledError(RSpiceHostError):
    pass


class _OutputBudget:
    def __init__(self, byte_limit: int) -> None:
        self.byte_limit = byte_limit
        self.bytes = 0


class _BoundedTextCapture(io.TextIOBase):
    def __init__(self, budget: _OutputBudget) -> None:
        super().__init__()
        self._budget = budget
        self._parts: list[str] = []

    @property
    def encoding(self) -> str:
        return "utf-8"

    def writable(self) -> bool:
        return True

    def write(self, value: str) -> int:
        if not isinstance(value, str):
            raise TypeError("captured output must be text")
        encoded = value.encode("utf-8")
        if self._budget.bytes + len(encoded) > self._budget.byte_limit:
            raise RSpiceHostError(
                "OUTPUT-LIMIT",
                f"Python output exceeded the authorized {self._budget.byte_limit}-byte limit",
            )
        self._budget.bytes += len(encoded)
        self._parts.append(value)
        return len(value)

    def getvalue(self) -> str:
        return "".join(self._parts)


def _request_limits(request: dict[str, object]) -> dict[str, int]:
    value = request.get("limits")
    if not isinstance(value, dict):
        raise ValueError("launch request has no resource-limit object")
    names = (
        "wall_time_ms",
        "cpu_time_ms",
        "memory_bytes",
        "output_bytes",
        "artifact_bytes",
        "max_tasks",
        "max_stack_depth",
    )
    limits: dict[str, int] = {}
    for name in names:
        item = value.get(name)
        if isinstance(item, bool) or not isinstance(item, int) or item <= 0:
            raise ValueError(f"resource limit {name!r} must be a positive integer")
        limits[name] = item
    return limits


def _apply_process_limits(limits: dict[str, int]) -> None:
    sys.setrecursionlimit(min(limits["max_stack_depth"], 100_000))
    if os.name != "posix":
        return
    try:
        import resource

        cpu_seconds = max(1, (limits["cpu_time_ms"] + 999) // 1000)
        resource.setrlimit(resource.RLIMIT_CPU, (cpu_seconds, cpu_seconds))
        if hasattr(resource, "RLIMIT_AS"):
            resource.setrlimit(
                resource.RLIMIT_AS,
                (limits["memory_bytes"], limits["memory_bytes"]),
            )
        if hasattr(resource, "RLIMIT_FSIZE"):
            file_limit = max(limits["output_bytes"], limits["artifact_bytes"])
            resource.setrlimit(resource.RLIMIT_FSIZE, (file_limit, file_limit))
        if hasattr(resource, "RLIMIT_NOFILE"):
            resource.setrlimit(resource.RLIMIT_NOFILE, (64, 64))
        if hasattr(resource, "RLIMIT_NPROC"):
            resource.setrlimit(resource.RLIMIT_NPROC, (1, 1))
    except (ImportError, OSError, ValueError) as error:
        raise RuntimeError(f"could not apply mandatory POSIX resource limits: {error}") from error


def _read_exact(count: int) -> bytes | None:
    chunks: list[bytes] = []
    remaining = count
    while remaining:
        chunk = _PROTOCOL_STDIN.read(remaining)
        if not chunk:
            if remaining == count:
                return None
            raise EOFError(f"protocol frame ended with {remaining} bytes missing")
        chunks.append(chunk)
        remaining -= len(chunk)
    return b"".join(chunks)


def _read_frame() -> dict[str, object] | None:
    header = _read_exact(4)
    if header is None:
        return None
    size = struct.unpack(">I", header)[0]
    if size == 0 or size > MAX_FRAME_BYTES:
        raise ValueError(f"protocol frame length {size} is outside the accepted range")
    payload = _read_exact(size)
    if payload is None:
        raise EOFError("protocol frame body is missing")
    value = json.loads(payload)
    if not isinstance(value, dict):
        raise ValueError("protocol envelope must be an object")
    return value


def _write_frame(value: dict[str, object]) -> None:
    payload = json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode("utf-8")
    if not payload or len(payload) > MAX_FRAME_BYTES:
        raise ValueError("outgoing protocol frame is outside the accepted range")
    _PROTOCOL_STDOUT.write(struct.pack(">I", len(payload)))
    _PROTOCOL_STDOUT.write(payload)
    _PROTOCOL_STDOUT.flush()


def _event(
    event: dict[str, object],
    *,
    request_id: int | None = None,
    session_id: str | None = None,
) -> None:
    global _sequence
    with _EVENT_LOCK:
        _sequence += 1
        _write_frame(
            {
                "protocol": PROTOCOL,
                "request_id": request_id,
                "session_id": session_id,
                "sequence": _sequence,
                "event": event,
            }
        )


def _runtime_identity() -> dict[str, object]:
    digest = os.environ.get("RSPICE_RUNTIME_DIGEST", "")
    if len(digest) != 64:
        raise RuntimeError("verified runtime digest was not supplied by the native launcher")
    platform = {
        "win32": "native-windows",
        "darwin": "native-mac-os",
    }.get(sys.platform, "native-linux")
    return {
        "managed": True,
        "platform": platform,
        "architecture": os.environ.get("RSPICE_RUNTIME_ARCH", "unknown"),
        "runtime_build": os.environ.get("RSPICE_RUNTIME_BUILD", "unknown"),
        "runtime_digest": list(bytes.fromhex(digest)),
        "python_version": ".".join(str(value) for value in sys.version_info[:3]),
        "python_abi": f"cp{sys.version_info.major}{sys.version_info.minor}",
        "rspice_api_version": os.environ.get("RSPICE_API_VERSION", "1.0.0"),
        "protocol": PROTOCOL,
    }


def _state(
    state: str,
    detail: str,
    *,
    request_id: int | None,
    session_id: str | None,
) -> None:
    _event(
        {"event": "state", "state": state, "detail": detail[:65536]},
        request_id=request_id,
        session_id=session_id,
    )


def _syntax_diagnostic(document: dict[str, object], error: SyntaxError) -> dict[str, object]:
    line = max(int(error.lineno or 1), 1)
    column = max(int(error.offset or 1), 1)
    end_line = max(int(getattr(error, "end_lineno", None) or line), line)
    end_column = max(int(getattr(error, "end_offset", None) or column), column)
    source = str(document.get("source", ""))
    lines = source.splitlines(keepends=True)

    def byte_offset(target_line: int, target_column: int) -> int:
        prefix = "".join(lines[: max(target_line - 1, 0)])
        current = lines[target_line - 1] if 0 < target_line <= len(lines) else ""
        return len((prefix + current[: max(target_column - 1, 0)]).encode("utf-8"))

    return {
        "diagnostic_id": str(uuid.uuid4()),
        "document_id": document.get("document_id"),
        "document_revision": document.get("revision"),
        "severity": "error",
        "source": "managed-cpython",
        "code": "PY-SYNTAX",
        "message": str(error.msg or "invalid Python syntax"),
        "range": {
            "start": {
                "line": line,
                "column": column,
                "byte_offset": byte_offset(line, column),
            },
            "end": {
                "line": end_line,
                "column": end_column,
                "byte_offset": byte_offset(end_line, end_column),
            },
        },
    }


def _validate_snapshot(snapshot: object) -> list[dict[str, object]]:
    if not isinstance(snapshot, dict):
        raise ValueError("launch snapshot must be an object")
    documents = snapshot.get("documents")
    if not isinstance(documents, list) or not documents or len(documents) > 10_000:
        raise ValueError("source snapshot documents are missing or exceed the limit")
    diagnostics: list[dict[str, object]] = []
    entry_count = 0
    for document in documents:
        if not isinstance(document, dict):
            raise ValueError("source document must be an object")
        role = document.get("role")
        if role == "python-entry":
            entry_count += 1
        if role not in ("python-entry", "python-module"):
            continue
        source = document.get("source")
        path = document.get("logical_path")
        if not isinstance(source, str) or not isinstance(path, str):
            raise ValueError("Python document path and source must be strings")
        try:
            compile(source, f"rspice-project://{path}", "exec", dont_inherit=True)
        except SyntaxError as error:
            diagnostics.append(_syntax_diagnostic(document, error))
    if entry_count != 1:
        raise ValueError(f"source snapshot contains {entry_count} Python entry documents")
    return diagnostics


class _HostSession:
    def __init__(
        self,
        request_id: int,
        session_id: str,
        snapshot: dict[str, object],
        mode: str,
    ) -> None:
        self.request_id = request_id
        self.session_id = session_id
        self.snapshot = snapshot
        self.mode = mode
        self.next_call_id = 1
        self.last_value: str | None = None
        self.secret_values: set[str] = set()
        capabilities = snapshot.get("capabilities", [])
        if not isinstance(capabilities, list):
            raise ValueError("source snapshot capabilities must be a list")
        self.capabilities: dict[str, dict[str, object]] = {}
        for grant in capabilities:
            if not isinstance(grant, dict):
                raise ValueError("capability grant must be an object")
            kind = grant.get("capability")
            if not isinstance(kind, str) or kind in self.capabilities:
                raise ValueError("capability grants contain an invalid or repeated kind")
            self.capabilities[kind] = grant

    def call(self, capability: str, operation: dict[str, object]) -> str | None:
        grant = self.capabilities.get(capability)
        operation_name = str(operation.get("operation", "host-operation"))
        if grant is None:
            _event(
                {
                    "event": "permission-denied",
                    "capability": capability,
                    "scope": "not-granted",
                    "operation": operation_name,
                },
                request_id=self.request_id,
                session_id=self.session_id,
            )
            raise PermissionError(
                f"RSpice capability {capability!r} was not granted for {operation_name}"
            )
        call_id = self.next_call_id
        self.next_call_id += 1
        token = grant.get("token")
        if not isinstance(token, str):
            raise ValueError("capability token is not a UUID string")
        _event(
            {
                "event": "host-call",
                "call": {
                    "call_id": call_id,
                    "capability": capability,
                    "capability_token": token,
                    "operation": operation,
                },
            },
            request_id=self.request_id,
            session_id=self.session_id,
        )
        adapter = _ACTIVE_DEBUG_ADAPTER
        if adapter is not None and adapter.session_id == self.session_id:
            handle, self.last_value = _host_response_handle(
                adapter.wait_for_host_response(call_id), call_id
            )
            return handle
        while True:
            envelope = _read_frame()
            if envelope is None:
                raise EOFError("RSpice host disconnected during a capability call")
            if envelope.get("protocol") != PROTOCOL:
                raise ValueError("host response uses an incompatible protocol version")
            request = envelope.get("request")
            if not isinstance(request, dict):
                raise ValueError("host response envelope has no request object")
            request_operation = request.get("operation")
            if request_operation == "cancel":
                if request.get("session_id") != self.session_id:
                    continue
                raise RSpiceCancelledError("CANCELLED", "Automation session was cancelled")
            if request_operation != "host-response":
                raise ValueError(
                    f"operation {request_operation!r} is not valid while a host call is pending"
                )
            if request.get("session_id") != self.session_id:
                continue
            if request.get("call_id") != call_id:
                raise ValueError("host response call identity does not match the pending call")
            response = request.get("response")
            if not isinstance(response, dict):
                raise ValueError("host response payload must be an object")
            handle, self.last_value = _host_response_handle(response, call_id)
            return handle

    def call_text(self, capability: str, operation: dict[str, object]) -> str | None:
        self.call(capability, operation)
        if capability == "environment-read" and self.last_value:
            self.secret_values.add(self.last_value)
        return self.last_value

    def redact_output(self, value: str) -> str:
        for secret in sorted(self.secret_values, key=len, reverse=True):
            value = value.replace(secret, "<redacted>")
        return value


def _host_response_handle(
    response: dict[str, object], call_id: int
) -> tuple[str | None, str | None]:
    status = response.get("status")
    if status == "success":
        handle = response.get("handle")
        if handle is not None and not isinstance(handle, str):
            raise ValueError("host response handle must be a UUID string or null")
        value = response.get("value")
        if value is not None and not isinstance(value, str):
            raise ValueError("host response value must be a string or null")
        return handle, value
    if status == "failure":
        code = response.get("code")
        message = response.get("message")
        if not isinstance(code, str) or not isinstance(message, str):
            raise ValueError("host failure code and message must be strings")
        if response.get("permission_denied") is True:
            raise PermissionError(f"{message} ({code})")
        raise RSpiceHostError(code, message)
    raise ValueError(f"unknown host response status {status!r} for call {call_id}")


def _require_handle(handle: str | None, operation: str) -> str:
    if handle is None:
        raise ValueError(f"host operation {operation} did not return its required handle")
    return handle


class ArtifactFormat:
    JUNIT = "junit"
    JSON = "summary-json"
    PDF = "verification-pdf"


class Environment:
    @staticmethod
    def get(name: str, default: str | None = None) -> str | None:
        if not isinstance(name, str) or not name:
            raise ValueError("environment variable name must be a non-empty string")
        if default is not None and not isinstance(default, str):
            raise TypeError("environment default must be a string or None")
        session = _ACTIVE_SESSION
        if session is None:
            raise RuntimeError("Environment.get() is only valid inside an RSpice session")
        value = session.call_text(
            "environment-read",
            {"operation": "read-environment", "name": name},
        )
        return default if value is None else value


class _RunPlans:
    def __init__(self, session: _HostSession, project_handle: str) -> None:
        self._session = session
        self._project_handle = project_handle

    def load(self, logical_path: str) -> "RunPlan":
        if not isinstance(logical_path, str) or not logical_path.strip():
            raise ValueError("run-plan logical path must be a non-empty string")
        documents = self._session.snapshot.get("documents", [])
        document = next(
            (
                item
                for item in documents
                if isinstance(item, dict)
                and _logical_path_key(str(item.get("logical_path", "")))
                == _logical_path_key(logical_path)
                and item.get("role") == "run-plan"
            ),
            None,
        )
        if document is None:
            raise FileNotFoundError(
                f"{logical_path!r} is not the run plan bound into this source snapshot"
            )
        document_id = document.get("document_id")
        if not isinstance(document_id, str):
            raise ValueError("bound run-plan document has no stable UUID")
        handle = self._session.call(
            "project-read",
            {
                "operation": "load-run-plan",
                "project_handle": self._project_handle,
                "document_id": document_id,
            },
        )
        return RunPlan(self._session, _require_handle(handle, "load-run-plan"))


class Project:
    def __init__(self, session: _HostSession, handle: str) -> None:
        self._session = session
        self._handle = handle
        self.run_plans = _RunPlans(session, handle)

    @classmethod
    def open(cls, selector: str) -> "Project":
        session = _ACTIVE_SESSION
        if session is None:
            raise RuntimeError("Project.open() is only valid inside an RSpice Automation session")
        if not isinstance(selector, str) or not selector.strip():
            raise ValueError("project selector must be a non-empty string")
        handle = session.call(
            "project-read",
            {"operation": "open-project", "selector": selector},
        )
        return cls(session, _require_handle(handle, "open-project"))


class RunPlan:
    def __init__(self, session: _HostSession, handle: str) -> None:
        self._session = session
        self._handle = handle

    def validate(self, *, target: str, fail_closed: bool = True) -> "RunPreview":
        if not isinstance(target, str) or not target.strip():
            raise ValueError("run target must be a non-empty string")
        if not isinstance(fail_closed, bool):
            raise TypeError("fail_closed must be a bool")
        handle = self._session.call(
            "project-read",
            {
                "operation": "validate-run-plan",
                "plan_handle": self._handle,
                "target": target,
                "fail_closed": fail_closed,
            },
        )
        return RunPreview(self._session, _require_handle(handle, "validate-run-plan"))


class RunPreview:
    def __init__(self, session: _HostSession, handle: str) -> None:
        self._session = session
        self._handle = handle

    def execute(self) -> "Run":
        handle = self._session.call(
            "simulation-execute",
            {"operation": "execute-run-plan", "preview_handle": self._handle},
        )
        return Run(self._session, _require_handle(handle, "execute-run-plan"))


class _Requirements:
    def __init__(self, run: "Run") -> None:
        self._run = run

    def evaluate(self, *, profile: str) -> None:
        if not isinstance(profile, str) or not profile.strip():
            raise ValueError("requirements profile must be a non-empty string")
        self._run._session.call(
            "result-read",
            {
                "operation": "evaluate-requirements",
                "run_handle": self._run._handle,
                "profile": profile,
            },
        )


class Run:
    def __init__(self, session: _HostSession, handle: str) -> None:
        self._session = session
        self._handle = handle
        self.requirements = _Requirements(self)

    def compare(self, *, baseline: str, waveforms: bool = True) -> None:
        if not isinstance(baseline, str) or not baseline.strip():
            raise ValueError("comparison baseline must be a non-empty string")
        if not isinstance(waveforms, bool):
            raise TypeError("waveforms must be a bool")
        self._session.call(
            "result-read",
            {
                "operation": "compare-run",
                "run_handle": self._handle,
                "baseline": baseline,
                "waveforms": waveforms,
            },
        )

    def export(self, formats: list[str] | tuple[str, ...]) -> None:
        if not isinstance(formats, (list, tuple)) or not formats:
            raise ValueError("artifact format list must not be empty")
        normalized = list(formats)
        if not all(isinstance(value, str) and value for value in normalized):
            raise TypeError("artifact formats must be strings from ArtifactFormat")
        self._session.call(
            "artifact-write",
            {
                "operation": "export-run",
                "run_handle": self._handle,
                "formats": normalized,
            },
        )


class _VirtualSourceLoader(importlib.abc.MetaPathFinder, importlib.abc.Loader):
    def __init__(self, documents: list[dict[str, object]]) -> None:
        self.modules: dict[str, tuple[dict[str, object], bool]] = {}
        self.packages: set[str] = set()
        for document in documents:
            if document.get("role") not in ("python-entry", "python-module"):
                continue
            path = str(document.get("logical_path", ""))
            if not path.endswith(".py"):
                continue
            components = path[:-3].split("/")
            is_package = components[-1] == "__init__"
            if is_package:
                components.pop()
            if not components:
                continue
            name = ".".join(components)
            self.modules[name] = (document, is_package)
            for index in range(1, len(components)):
                self.packages.add(".".join(components[:index]))

    def find_spec(self, fullname: str, path: object = None, target: object = None):
        if fullname in self.modules:
            return importlib.util.spec_from_loader(
                fullname, self, is_package=self.modules[fullname][1]
            )
        if fullname in self.packages:
            return importlib.util.spec_from_loader(fullname, self, is_package=True)
        return None

    def create_module(self, spec):
        return None

    def exec_module(self, module: types.ModuleType) -> None:
        record = self.modules.get(module.__name__)
        if record is None:
            module.__path__ = []
            return
        document, is_package = record
        source = str(document["source"])
        logical_path = str(document["logical_path"])
        module.__file__ = f"rspice-project://{logical_path}"
        if is_package:
            module.__path__ = []
        code = compile(source, module.__file__, "exec", dont_inherit=True)
        exec(code, module.__dict__)


_ACTIVE_SESSION: _HostSession | None = None
_AUDIT_GUARD_INSTALLED = False
_RUNTIME_ROOT = os.path.normcase(
    os.path.realpath(os.environ.get("RSPICE_RUNTIME_ROOT", ""))
)


def _runtime_read_path(path: object) -> bool:
    if isinstance(path, int):
        return False
    try:
        candidate = os.path.normcase(os.path.realpath(os.fsdecode(path)))
        return os.path.commonpath((_RUNTIME_ROOT, candidate)) == _RUNTIME_ROOT
    except (OSError, TypeError, ValueError):
        return False


def _install_user_audit_guard() -> None:
    """Deny ambient OS authority from Python source.

    Simulator, project, result, and artifact access is possible only through
    typed host calls. This process-local hook is defense in depth and does not
    replace the native launcher's required platform sandbox/job boundary.
    """

    global _AUDIT_GUARD_INSTALLED
    if _AUDIT_GUARD_INSTALLED:
        return

    mutation_events = {
        "os.chdir",
        "os.chmod",
        "os.chown",
        "os.link",
        "os.mkdir",
        "os.remove",
        "os.rename",
        "os.rmdir",
        "os.symlink",
        "os.truncate",
        "shutil.chown",
    }
    enumeration_events = {"os.listdir", "os.scandir"}
    process_events = {
        "os.exec",
        "os.fork",
        "os.forkpty",
        "os.kill",
        "os.posix_spawn",
        "os.spawn",
        "os.startfile",
        "os.system",
        "pty.spawn",
        "subprocess.Popen",
    }

    def audit(event: str, args: tuple[object, ...]) -> None:
        if event == "open":
            path = args[0] if args else None
            mode = args[1] if len(args) > 1 else "r"
            flags = args[2] if len(args) > 2 else 0
            write_mode = isinstance(mode, str) and any(
                marker in mode for marker in ("w", "a", "x", "+")
            )
            write_flags = isinstance(flags, int) and bool(
                flags
                & (
                    getattr(os, "O_WRONLY", 0)
                    | getattr(os, "O_RDWR", 0)
                    | getattr(os, "O_CREAT", 0)
                    | getattr(os, "O_TRUNC", 0)
                    | getattr(os, "O_APPEND", 0)
                )
            )
            if write_mode or write_flags or not _runtime_read_path(path):
                raise PermissionError(
                    "direct filesystem access is denied; use the versioned RSpice API"
                )
            return
        if event in mutation_events or event in enumeration_events:
            raise PermissionError(
                "direct filesystem access is denied; use the versioned RSpice API"
            )
        if event in process_events or event.startswith("subprocess."):
            raise PermissionError(
                "process control is denied; no process capability was brokered"
            )
        if event.startswith("socket."):
            raise PermissionError(
                "network access is denied; no network capability was brokered"
            )
        if event.startswith("ctypes.") or event.startswith("winreg."):
            raise PermissionError("native and registry access is denied in Automation")

    sys.addaudithook(audit)
    _AUDIT_GUARD_INSTALLED = True


def _install_rspice_module() -> types.ModuleType:
    module = types.ModuleType("rspice")
    module.__dict__.update(
        {
            "__version__": os.environ.get("RSPICE_API_VERSION", "1.0.0"),
            "ArtifactFormat": ArtifactFormat,
            "Environment": Environment,
            "Project": Project,
            "RSpiceHostError": RSpiceHostError,
            "RSpiceCancelledError": RSpiceCancelledError,
        }
    )
    sys.modules["rspice"] = module
    return module


class _DebugRestart(BaseException):
    pass


class _DebugAdapter:
    """Cooperative debugger for one immutable source snapshot.

    User code runs on a dedicated thread. The protocol thread remains
    responsive for pause, stop, stack, variables, watch evaluation, breakpoint
    updates, and capability-broker responses. Trace callbacks are installed
    only on the debuggee thread and only project:// source frames are exposed.
    """

    def __init__(
        self,
        request_id: int,
        session_id: str,
        snapshot: dict[str, object],
        limits: dict[str, int],
        breakpoints: object,
        exception_policy: object,
    ) -> None:
        self.request_id = request_id
        self.session_id = session_id
        self.snapshot = snapshot
        self.limits = limits
        self.exception_policy = str(exception_policy)
        if self.exception_policy not in ("all", "uncaught", "never"):
            raise ValueError("debug exception policy is invalid")
        documents = snapshot.get("documents")
        if not isinstance(documents, list):
            raise ValueError("debug source snapshot has no documents")
        self.documents: dict[str, dict[str, object]] = {}
        self.document_ids: set[str] = set()
        self.document_line_counts: dict[str, int] = {}
        for document in documents:
            if not isinstance(document, dict):
                continue
            path = document.get("logical_path")
            document_id = document.get("document_id")
            role = document.get("role")
            if (
                isinstance(path, str)
                and isinstance(document_id, str)
                and role in ("python-entry", "python-module")
            ):
                self.documents[f"rspice-project://{path}"] = document
                self.document_ids.add(document_id)
                self.document_line_counts[document_id] = len(
                    str(document.get("source", "")).splitlines()
                )
        self.condition = threading.Condition()
        self.breakpoints: list[dict[str, object]] = []
        self.breakpoint_hits: dict[str, int] = {}
        self.set_breakpoints(breakpoints)
        self.frames: dict[int, types.FrameType] = {}
        self.stack_payload: list[dict[str, object]] = []
        self.value_references: dict[int, object] = {}
        self.next_value_reference = 1_000_000_000
        self.host_responses: dict[int, dict[str, object]] = {}
        self.pause_requested = False
        self.cancel_requested = False
        self.restart_requested = False
        self.entry_pending = True
        self.resume_mode = "continue"
        self.step_depth = 0
        self.running = False
        self.paused = False
        self.thread: threading.Thread | None = None

    def start(self) -> None:
        self.thread = threading.Thread(
            target=self._run,
            name="rspice-automation-debuggee",
            daemon=True,
        )
        self.thread.start()

    def _run(self) -> None:
        global _ACTIVE_DEBUG_ADAPTER
        try:
            while True:
                self.entry_pending = True
                self.restart_requested = False
                self.cancel_requested = False
                self.running = True
                self.paused = False
                _state(
                    "running",
                    "executing governed Python under the managed debugger",
                    request_id=self.request_id,
                    session_id=self.session_id,
                )
                try:
                    _execute_snapshot(
                        self.request_id,
                        self.session_id,
                        self.snapshot,
                        "debug",
                        self.limits,
                        debugger=self,
                    )
                except _DebugRestart:
                    self._clear_pause()
                    continue
                except RSpiceCancelledError as error:
                    self._clear_pause()
                    _state(
                        "cancelled",
                        error.message,
                        request_id=self.request_id,
                        session_id=self.session_id,
                    )
                    return
                except PermissionError as error:
                    self._publish_failure(
                        "PERMISSION-DENIED",
                        str(error),
                        "a required host capability was denied",
                    )
                    return
                except BaseException as error:
                    if self.exception_policy == "uncaught":
                        frame = self._last_project_traceback_frame(error.__traceback__)
                        if frame is not None:
                            self._pause(
                                frame,
                                "exception",
                                str(error) or type(error).__name__,
                            )
                    rendered = "".join(traceback.format_exception(error))
                    _event(
                        {
                            "event": "output",
                            "channel": "stderr",
                            "category": "traceback",
                            "text": rendered[:MAX_OUTPUT_TEXT],
                        },
                        request_id=self.request_id,
                        session_id=self.session_id,
                    )
                    self._publish_failure(
                        "PYTHON-EXCEPTION",
                        str(error) or type(error).__name__,
                        "Python execution raised an exception",
                    )
                    return
                self._clear_pause()
                _state(
                    "completed",
                    "governed debug execution completed",
                    request_id=self.request_id,
                    session_id=self.session_id,
                )
                return
        finally:
            self.running = False
            self.paused = False
            with self.condition:
                self.condition.notify_all()
            if _ACTIVE_DEBUG_ADAPTER is self:
                _ACTIVE_DEBUG_ADAPTER = None

    def _publish_failure(self, code: str, message: str, detail: str) -> None:
        _event(
            {
                "event": "worker-failed",
                "code": code,
                "message": message[:65536],
                "recoverable": True,
            },
            request_id=self.request_id,
            session_id=self.session_id,
        )
        _state(
            "failed",
            detail,
            request_id=self.request_id,
            session_id=self.session_id,
        )

    def trace(self, frame: types.FrameType, event: str, arg: object):
        if self.cancel_requested:
            raise RSpiceCancelledError("CANCELLED", "Debug session was stopped")
        if self.restart_requested:
            raise _DebugRestart()
        if frame.f_code.co_filename not in self.documents:
            return self.trace
        if event == "exception" and self.exception_policy == "all":
            error = arg[1] if isinstance(arg, tuple) and len(arg) > 1 else None
            self._pause(
                frame,
                "exception",
                str(error) if error is not None else "Python exception",
            )
            return self.trace
        if event != "line":
            return self.trace
        depth = self._project_depth(frame)
        if self.entry_pending:
            self.entry_pending = False
            self._pause(frame, "entry", "Paused at the Automation entry point")
            return self.trace
        if self.pause_requested:
            self.pause_requested = False
            self._pause(frame, "pause", "Pause requested by user")
            return self.trace
        if self._apply_breakpoints(frame):
            return self.trace
        should_step = (
            self.resume_mode == "step-in"
            or (self.resume_mode == "step-over" and depth <= self.step_depth)
            or (self.resume_mode == "step-out" and depth < self.step_depth)
        )
        if should_step:
            self.resume_mode = "continue"
            self._pause(frame, "step", "Step completed")
        return self.trace

    def _apply_breakpoints(self, frame: types.FrameType) -> bool:
        document = self.documents.get(frame.f_code.co_filename)
        if document is None:
            return False
        document_id = document.get("document_id")
        stopped = False
        for breakpoint in self.breakpoints:
            if (
                breakpoint.get("enabled") is not True
                or breakpoint.get("document_id") != document_id
                or breakpoint.get("line") != frame.f_lineno
            ):
                continue
            breakpoint_id = str(breakpoint.get("breakpoint_id"))
            kind = breakpoint.get("kind")
            if not isinstance(kind, dict):
                continue
            kind_name = kind.get("kind")
            if kind_name == "logpoint":
                template = str(kind.get("template", ""))
                rendered = self._render_logpoint(template, frame)
                _event(
                    {
                        "event": "output",
                        "channel": "stdout",
                        "category": "logpoint",
                        "text": rendered[:MAX_OUTPUT_TEXT],
                    },
                    request_id=self.request_id,
                    session_id=self.session_id,
                )
                continue
            if kind_name == "conditional":
                if not bool(self._evaluate(str(kind.get("expression", "")), frame)):
                    continue
            if kind_name == "hit-count":
                hits = self.breakpoint_hits.get(breakpoint_id, 0) + 1
                self.breakpoint_hits[breakpoint_id] = hits
                if hits < int(kind.get("count", 1)):
                    continue
                condition = kind.get("condition")
                if isinstance(condition, str) and condition and not bool(
                    self._evaluate(condition, frame)
                ):
                    continue
            self._pause(frame, "breakpoint", f"Breakpoint at line {frame.f_lineno}")
            stopped = True
            break
        return stopped

    def _pause(self, frame: types.FrameType, reason: str, description: str) -> None:
        self._capture_stack(frame)
        self.running = False
        self.paused = True
        _state(
            "paused",
            description,
            request_id=self.request_id,
            session_id=self.session_id,
        )
        _event(
            {
                "event": "stopped",
                "reason": reason,
                "description": description[:65536],
                "frame_id": self.stack_payload[0]["frame_id"] if self.stack_payload else None,
            },
            request_id=self.request_id,
            session_id=self.session_id,
        )
        with self.condition:
            while self.paused and not self.cancel_requested and not self.restart_requested:
                self.condition.wait()
        self.running = True
        if self.cancel_requested:
            raise RSpiceCancelledError("CANCELLED", "Debug session was stopped")
        if self.restart_requested:
            raise _DebugRestart()

    def _capture_stack(self, frame: types.FrameType) -> None:
        self.frames.clear()
        self.stack_payload.clear()
        self.value_references.clear()
        self.next_value_reference = 1_000_000_000
        current: types.FrameType | None = frame
        frame_id = 1
        while current is not None and len(self.stack_payload) < 10_000:
            document = self.documents.get(current.f_code.co_filename)
            if document is not None:
                source_range = self._source_range(document, current.f_lineno)
                locals_reference = frame_id * 4 + 1
                globals_reference = frame_id * 4 + 2
                self.frames[frame_id] = current
                self.stack_payload.append(
                    {
                        "frame_id": frame_id,
                        "name": current.f_code.co_name or "<module>",
                        "document_id": document["document_id"],
                        "range": source_range,
                        "locals_reference": locals_reference,
                        "globals_reference": globals_reference,
                    }
                )
                frame_id += 1
            current = current.f_back

    def _clear_pause(self) -> None:
        self.frames.clear()
        self.stack_payload.clear()
        self.value_references.clear()
        self.paused = False

    def set_breakpoints(self, breakpoints: object) -> None:
        if not isinstance(breakpoints, list) or len(breakpoints) > 100_000:
            raise ValueError("debug breakpoint collection is invalid")
        validated: list[dict[str, object]] = []
        identities: set[str] = set()
        for breakpoint in breakpoints:
            if not isinstance(breakpoint, dict):
                raise ValueError("debug breakpoint must be an object")
            identity = breakpoint.get("breakpoint_id")
            document_id = breakpoint.get("document_id")
            line = breakpoint.get("line")
            column = breakpoint.get("column")
            if (
                not isinstance(identity, str)
                or identity in identities
                or document_id not in self.document_ids
                or not isinstance(line, int)
                or line <= 0
                or line > self.document_line_counts.get(str(document_id), 0)
                or not isinstance(column, int)
                or column <= 0
                or not isinstance(breakpoint.get("enabled"), bool)
                or not isinstance(breakpoint.get("kind"), dict)
            ):
                raise ValueError("debug breakpoint identity, location, or kind is invalid")
            identities.add(identity)
            validated.append(dict(breakpoint))
        with self.condition:
            self.breakpoints = validated
            self.breakpoint_hits = {
                identity: count
                for identity, count in self.breakpoint_hits.items()
                if identity in identities
            }

    def control(self, control: object) -> None:
        if not isinstance(control, str):
            raise ValueError("debug control is invalid")
        with self.condition:
            if control == "pause":
                self.pause_requested = True
                return
            if control == "stop":
                self.cancel_requested = True
                self.paused = False
                self.condition.notify_all()
                return
            if control == "restart":
                self.restart_requested = True
                self.paused = False
                self.condition.notify_all()
                return
            if control not in ("continue", "step-in", "step-over", "step-out"):
                raise ValueError(f"unsupported debug control {control!r}")
            if not self.paused:
                raise ValueError(f"debug control {control!r} requires a paused debuggee")
            self.resume_mode = control
            if control in ("step-over", "step-out") and self.stack_payload:
                frame = self.frames.get(int(self.stack_payload[0]["frame_id"]))
                self.step_depth = self._project_depth(frame) if frame is not None else 0
            self.paused = False
            self.condition.notify_all()
        _state(
            "running",
            f"debug control {control} resumed execution",
            request_id=self.request_id,
            session_id=self.session_id,
        )

    def stack(self, request_id: int, start: object, count: object) -> None:
        if not self.paused:
            raise ValueError("stack trace is available only while paused")
        start_value, count_value = self._slice(start, count)
        payload = self.stack_payload[start_value : start_value + count_value]
        _event(
            {"event": "stack", "frames": payload, "total": len(self.stack_payload)},
            request_id=request_id,
            session_id=self.session_id,
        )

    def variables(self, request_id: int, reference: object, start: object, count: object) -> None:
        if not self.paused or not isinstance(reference, int) or reference <= 0:
            raise ValueError("variable reference is invalid or the debuggee is not paused")
        values: list[tuple[str, object]]
        frame_id, scope = divmod(reference, 4)
        frame = self.frames.get(frame_id)
        if frame is not None and scope == 1:
            values = sorted(frame.f_locals.items())
        elif frame is not None and scope == 2:
            values = sorted(
                (name, value) for name, value in frame.f_globals.items() if name != "__builtins__"
            )
        elif reference in self.value_references:
            values = self._children(self.value_references[reference])
        else:
            raise ValueError("variable reference is stale or unknown")
        start_value, count_value = self._slice(start, count)
        payload = [
            self._variable(name, value)
            for name, value in values[start_value : start_value + count_value]
        ]
        _event(
            {"event": "variables", "values": payload, "total": len(values)},
            request_id=request_id,
            session_id=self.session_id,
        )

    def evaluate(self, request_id: int, frame_id: object, expression: object) -> None:
        if (
            not self.paused
            or not isinstance(frame_id, int)
            or frame_id not in self.frames
            or not isinstance(expression, str)
            or not expression.strip()
        ):
            raise ValueError("watch evaluation request is invalid or stale")
        try:
            value = self._evaluate(expression, self.frames[frame_id])
            result = self._variable(expression, value)
        except BaseException as error:
            result = {
                "name": expression,
                "type_name": "error",
                "display_value": (str(error) or type(error).__name__)[:65536],
                "variables_reference": 0,
                "redacted": False,
            }
        _event(
            {"event": "evaluated", "expression": expression, "result": result},
            request_id=request_id,
            session_id=self.session_id,
        )

    def accept_host_response(self, call_id: object, response: object) -> None:
        if not isinstance(call_id, int) or call_id <= 0 or not isinstance(response, dict):
            raise ValueError("debug host response is invalid")
        with self.condition:
            if call_id in self.host_responses:
                raise ValueError("debug host response call identity was repeated")
            self.host_responses[call_id] = response
            self.condition.notify_all()

    def wait_for_host_response(self, call_id: int) -> dict[str, object]:
        with self.condition:
            while call_id not in self.host_responses:
                if self.cancel_requested:
                    raise RSpiceCancelledError("CANCELLED", "Debug session was stopped")
                if self.restart_requested:
                    raise _DebugRestart()
                self.condition.wait()
            return self.host_responses.pop(call_id)

    def _evaluate(self, expression: str, frame: types.FrameType) -> object:
        parsed = ast.parse(expression, mode="eval")
        forbidden = (
            ast.Call,
            ast.Await,
            ast.Yield,
            ast.YieldFrom,
            ast.Lambda,
            ast.ListComp,
            ast.SetComp,
            ast.DictComp,
            ast.GeneratorExp,
            ast.NamedExpr,
        )
        if any(isinstance(node, forbidden) for node in ast.walk(parsed)):
            raise ValueError("watch expressions cannot call code or mutate debuggee state")
        return eval(compile(parsed, "<rspice-watch>", "eval"), frame.f_globals, frame.f_locals)

    def _render_logpoint(self, template: str, frame: types.FrameType) -> str:
        def replace(match: re.Match[str]) -> str:
            try:
                return self._display(self._evaluate(match.group(1), frame))
            except BaseException as error:
                return f"<error: {str(error) or type(error).__name__}>"

        return re.sub(r"\{([^{}]+)\}", replace, template)

    def _variable(self, name: str, value: object) -> dict[str, object]:
        redacted = bool(re.search(r"(?i)(secret|token|password|credential|api[_-]?key)", name))
        reference = 0 if redacted else self._register_value(value)
        return {
            "name": str(name)[:4096],
            "type_name": type(value).__name__[:4096],
            "display_value": "<redacted>" if redacted else self._display(value),
            "variables_reference": reference,
            "redacted": redacted,
        }

    def _register_value(self, value: object) -> int:
        if not self._children(value):
            return 0
        reference = self.next_value_reference
        self.next_value_reference += 1
        self.value_references[reference] = value
        return reference

    @staticmethod
    def _children(value: object) -> list[tuple[str, object]]:
        if isinstance(value, dict):
            return [(str(key), item) for key, item in list(value.items())[:100_000]]
        if isinstance(value, (list, tuple)):
            return [(f"[{index}]", item) for index, item in enumerate(value[:100_000])]
        try:
            namespace = object.__getattribute__(value, "__dict__")
        except (AttributeError, TypeError):
            namespace = None
        if isinstance(namespace, dict):
            return sorted(
                (str(name), item)
                for name, item in namespace.items()
                if not str(name).startswith("__")
            )[:100_000]
        return []

    @staticmethod
    def _display(value: object) -> str:
        if value is None or type(value) in (bool, int, float):
            return repr(value)
        if isinstance(value, str):
            rendered = repr(value)
        elif isinstance(value, bytes):
            rendered = f"bytes({len(value)})"
        elif isinstance(value, dict):
            rendered = f"dict({len(value)} items)"
        elif isinstance(value, (list, tuple, set, frozenset)):
            rendered = f"{type(value).__name__}({len(value)} items)"
        else:
            rendered = f"<{type(value).__module__}.{type(value).__name__}>"
        return rendered[:65536]

    @staticmethod
    def _slice(start: object, count: object) -> tuple[int, int]:
        if (
            not isinstance(start, int)
            or start < 0
            or not isinstance(count, int)
            or count <= 0
            or count > 100_000
        ):
            raise ValueError("debug collection slice is outside the accepted range")
        return start, count

    def _source_range(self, document: dict[str, object], line: int) -> dict[str, object]:
        source = str(document.get("source", ""))
        lines = source.splitlines(keepends=True)
        index = max(0, min(line - 1, max(len(lines) - 1, 0)))
        prefix = "".join(lines[:index])
        content = lines[index].rstrip("\r\n") if lines else ""
        start = len(prefix.encode("utf-8"))
        end = start + len(content.encode("utf-8"))
        return {
            "start": {"line": max(line, 1), "column": 1, "byte_offset": start},
            "end": {
                "line": max(line, 1),
                "column": len(content) + 1,
                "byte_offset": end,
            },
        }

    def _project_depth(self, frame: types.FrameType | None) -> int:
        depth = 0
        while frame is not None:
            if frame.f_code.co_filename in self.documents:
                depth += 1
            frame = frame.f_back
        return depth

    def _last_project_traceback_frame(self, value: types.TracebackType | None) -> types.FrameType | None:
        found = None
        while value is not None:
            if value.tb_frame.f_code.co_filename in self.documents:
                found = value.tb_frame
            value = value.tb_next
        return found


def _execute_snapshot(
    request_id: int,
    session_id: str,
    snapshot: dict[str, object],
    mode: str,
    limits: dict[str, int],
    debugger: _DebugAdapter | None = None,
) -> None:
    global _ACTIVE_SESSION
    documents = snapshot["documents"]
    if not isinstance(documents, list):
        raise ValueError("source snapshot documents must be a list")
    entry_id = snapshot.get("entry_document_id")
    entry = next(
        (
            item
            for item in documents
            if isinstance(item, dict) and item.get("document_id") == entry_id
        ),
        None,
    )
    if entry is None:
        raise ValueError("source snapshot entry document is missing")
    loader = _VirtualSourceLoader(documents)
    session = _HostSession(request_id, session_id, snapshot, mode)
    _ACTIVE_SESSION = session
    _install_rspice_module()
    _install_user_audit_guard()
    sys.meta_path.insert(0, loader)
    source_path = str(entry["logical_path"])
    package = source_path.rsplit("/", 1)[0].replace("/", ".") if "/" in source_path else None
    globals_dict = {
        "__name__": "__main__",
        "__file__": f"rspice-project://{source_path}",
        "__package__": package,
        "__builtins__": __builtins__,
    }
    output_budget = _OutputBudget(limits["output_bytes"])
    stdout = _BoundedTextCapture(output_budget)
    stderr = _BoundedTextCapture(output_budget)
    original_stdout, original_stderr = sys.stdout, sys.stderr
    failure: BaseException | None = None
    try:
        # Protocol frames always use the original binary streams; user output
        # is captured and returned as typed events.
        sys.stdout, sys.stderr = stdout, stderr
        if debugger is not None:
            sys.settrace(debugger.trace)
        code = compile(str(entry["source"]), globals_dict["__file__"], "exec", dont_inherit=True)
        exec(code, globals_dict)
    except BaseException as error:
        failure = error
    finally:
        if debugger is not None:
            sys.settrace(None)
        sys.stdout, sys.stderr = original_stdout, original_stderr
        if sys.meta_path and sys.meta_path[0] is loader:
            sys.meta_path.pop(0)
        else:
            sys.meta_path = [value for value in sys.meta_path if value is not loader]
        _ACTIVE_SESSION = None
    for channel, value in (("stdout", stdout.getvalue()), ("stderr", stderr.getvalue())):
        value = session.redact_output(value)
        if value:
            _event(
                {
                    "event": "output",
                    "channel": channel,
                    "category": "python",
                    "text": value[:MAX_OUTPUT_TEXT],
                },
                request_id=request_id,
                session_id=session_id,
            )
    if failure is not None:
        raise failure


def _launch(request_id: int, request: dict[str, object]) -> None:
    session_id = str(uuid.uuid4())
    mode = request.get("mode")
    snapshot = request.get("snapshot")
    _state(
        "validating",
        "compiling the exact source closure with managed CPython",
        request_id=request_id,
        session_id=session_id,
    )
    try:
        limits = _request_limits(request)
        _apply_process_limits(limits)
        diagnostics = _validate_snapshot(snapshot)
    except Exception as error:
        _event(
            {
                "event": "worker-failed",
                "code": "INVALID-SNAPSHOT",
                "message": str(error),
                "recoverable": False,
            },
            request_id=request_id,
            session_id=session_id,
        )
        _state(
            "failed",
            "the immutable source snapshot was rejected",
            request_id=request_id,
            session_id=session_id,
        )
        return
    for diagnostic in diagnostics:
        _event(
            {"event": "diagnostic", "diagnostic": diagnostic},
            request_id=request_id,
            session_id=session_id,
        )
    if diagnostics:
        _state(
            "failed",
            "managed CPython rejected one or more source documents",
            request_id=request_id,
            session_id=session_id,
        )
        return
    if mode == "validate":
        _state(
            "completed",
            "managed CPython accepted the exact source closure",
            request_id=request_id,
            session_id=session_id,
        )
        return

    if mode == "debug":
        global _ACTIVE_DEBUG_ADAPTER
        if not isinstance(snapshot, dict) or _ACTIVE_DEBUG_ADAPTER is not None:
            raise ValueError("a managed debugger session is already active or has no snapshot")
        adapter = _DebugAdapter(
            request_id,
            session_id,
            snapshot,
            limits,
            request.get("breakpoints"),
            request.get("exception_policy"),
        )
        _ACTIVE_DEBUG_ADAPTER = adapter
        adapter.start()
        return
    if mode not in ("run", "dry-run") or not isinstance(snapshot, dict):
        _event(
            {
                "event": "worker-failed",
                "code": "INVALID-LAUNCH-MODE",
                "message": f"launch mode {mode!r} is unsupported",
                "recoverable": False,
            },
            request_id=request_id,
            session_id=session_id,
        )
        _state(
            "failed",
            "launch mode was rejected",
            request_id=request_id,
            session_id=session_id,
        )
        return
    _state(
        "running",
        "executing governed Python through the RSpice capability broker",
        request_id=request_id,
        session_id=session_id,
    )
    try:
        _execute_snapshot(request_id, session_id, snapshot, str(mode), limits)
    except RSpiceCancelledError as error:
        _state(
            "cancelled",
            error.message,
            request_id=request_id,
            session_id=session_id,
        )
        return
    except PermissionError as error:
        _event(
            {
                "event": "worker-failed",
                "code": "PERMISSION-DENIED",
                "message": str(error),
                "recoverable": True,
            },
            request_id=request_id,
            session_id=session_id,
        )
        _state(
            "failed",
            "a required host capability was denied",
            request_id=request_id,
            session_id=session_id,
        )
        return
    except BaseException as error:
        rendered = "".join(traceback.format_exception(error))
        _event(
            {
                "event": "output",
                "channel": "stderr",
                "category": "traceback",
                "text": rendered[:MAX_OUTPUT_TEXT],
            },
            request_id=request_id,
            session_id=session_id,
        )
        _event(
            {
                "event": "worker-failed",
                "code": "PYTHON-EXCEPTION",
                "message": str(error) or type(error).__name__,
                "recoverable": True,
            },
            request_id=request_id,
            session_id=session_id,
        )
        _state(
            "failed",
            "Python execution raised an exception",
            request_id=request_id,
            session_id=session_id,
        )
        return
    _state(
        "completed",
        "governed Python execution completed",
        request_id=request_id,
        session_id=session_id,
    )


def _handle(envelope: dict[str, object]) -> bool:
    if envelope.get("protocol") != PROTOCOL:
        raise ValueError("incompatible Automation protocol version")
    request_id = envelope.get("request_id")
    request = envelope.get("request")
    if not isinstance(request_id, int) or request_id <= 0 or not isinstance(request, dict):
        raise ValueError("request envelope identity or body is invalid")
    operation = request.get("operation")
    adapter = _ACTIVE_DEBUG_ADAPTER
    if adapter is not None and operation not in ("probe", "shutdown"):
        if request.get("session_id") != adapter.session_id:
            raise ValueError("debug request has the wrong session identity")
        if operation == "set-breakpoints":
            adapter.set_breakpoints(request.get("breakpoints"))
        elif operation == "debug-control":
            adapter.control(request.get("control"))
        elif operation == "stack-trace":
            adapter.stack(request_id, request.get("start"), request.get("count"))
        elif operation == "variables":
            adapter.variables(
                request_id,
                request.get("reference"),
                request.get("start"),
                request.get("count"),
            )
        elif operation == "evaluate":
            adapter.evaluate(
                request_id,
                request.get("frame_id"),
                request.get("expression"),
            )
        elif operation == "host-response":
            adapter.accept_host_response(request.get("call_id"), request.get("response"))
        elif operation == "cancel":
            adapter.control("stop")
        else:
            raise ValueError(f"request operation {operation!r} is invalid during debugging")
        return True
    if operation == "probe":
        _event(
            {"event": "hello", "identity": _runtime_identity()},
            request_id=request_id,
        )
    elif operation == "launch":
        _launch(request_id, request)
    elif operation == "shutdown":
        if adapter is not None:
            adapter.control("stop")
        _event(
            {"event": "terminated", "exit_code": 0, "reason": "worker shutdown requested"},
            request_id=request_id,
        )
        return False
    else:
        _event(
            {
                "event": "worker-failed",
                "code": "UNSUPPORTED-REQUEST",
                "message": f"request operation {operation!r} is not valid in the current state",
                "recoverable": True,
            },
            request_id=request_id,
        )
    return True


def main() -> int:
    if sys.argv[1:] != ["--rspice-protocol-stdio"]:
        raise SystemExit("this module is an RSpice protocol worker, not a command-line tool")
    _event({"event": "hello", "identity": _runtime_identity()})
    while True:
        envelope = _read_frame()
        if envelope is None or not _handle(envelope):
            return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except SystemExit:
        raise
    except BaseException:
        traceback.print_exc(file=sys.stderr)
        raise SystemExit(70)
