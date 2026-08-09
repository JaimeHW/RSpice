"""Pinned Pyodide-side implementation of the RSpice Automation API.

This module is evaluated only inside the dedicated browser Automation worker.
All simulator authority crosses the typed JavaScript capability bridge; project
source is supplied as an immutable logical snapshot and is never mounted from a
user filesystem.
"""

from __future__ import annotations

import importlib.abc
import importlib.util
import ast
import io
import json
import re
import sys
import traceback
import types
import unicodedata

from pyodide.ffi import run_sync
from rspice_bridge import debug_checkpoint, debug_exchange, host_call


def _logical_path_key(value: str) -> str:
    """Match RSpice's locale-independent portable project-path identity."""
    return unicodedata.normalize(
        "NFC", unicodedata.normalize("NFC", value).upper().lower()
    )


class RSpiceHostError(RuntimeError):
    def __init__(self, code: str, message: str) -> None:
        super().__init__(f"{message} ({code})")
        self.code = code
        self.message = message


class RSpiceCancelledError(RSpiceHostError):
    pass


class _OutputBudget:
    def __init__(self, limit: int) -> None:
        self.limit = limit
        self.used = 0


class _Capture(io.TextIOBase):
    def __init__(self, budget: _OutputBudget) -> None:
        self._budget = budget
        self._parts: list[str] = []

    @property
    def encoding(self) -> str:
        return "utf-8"

    def writable(self) -> bool:
        return True

    def write(self, value: str) -> int:
        encoded = value.encode("utf-8")
        if self._budget.used + len(encoded) > self._budget.limit:
            raise RSpiceHostError(
                "OUTPUT-LIMIT",
                f"Python output exceeded the authorized {self._budget.limit}-byte limit",
            )
        self._budget.used += len(encoded)
        self._parts.append(value)
        return len(value)

    def getvalue(self) -> str:
        return "".join(self._parts)


class _HostSession:
    def __init__(self, snapshot: dict[str, object], mode: str) -> None:
        self.snapshot = snapshot
        self.mode = mode
        self.last_value: str | None = None
        self.secret_values: set[str] = set()
        grants = snapshot.get("capabilities")
        if not isinstance(grants, list):
            raise ValueError("source snapshot capabilities must be a list")
        self.capabilities: dict[str, dict[str, object]] = {}
        for grant in grants:
            if not isinstance(grant, dict):
                raise ValueError("capability grant must be an object")
            kind = grant.get("capability")
            if not isinstance(kind, str) or kind in self.capabilities:
                raise ValueError("capability grants contain an invalid or repeated kind")
            self.capabilities[kind] = grant

    def call(self, capability: str, operation: dict[str, object]) -> str | None:
        grant = self.capabilities.get(capability)
        if grant is None:
            raise PermissionError(f"RSpice capability {capability!r} was not granted")
        token = grant.get("token")
        if not isinstance(token, str):
            raise ValueError("capability token is not a UUID string")
        request = json.dumps(
            {
                "capability": capability,
                "capability_token": token,
                "operation": operation,
            },
            separators=(",", ":"),
        )
        response = json.loads(str(run_sync(host_call(request))))
        status = response.get("status")
        if status == "success":
            handle = response.get("handle")
            if handle is not None and not isinstance(handle, str):
                raise ValueError("host response handle must be a UUID string or null")
            value = response.get("value")
            if value is not None and not isinstance(value, str):
                raise ValueError("host response value must be a string or null")
            self.last_value = value
            return handle
        if status == "failure":
            code = response.get("code")
            message = response.get("message")
            if not isinstance(code, str) or not isinstance(message, str):
                raise ValueError("host failure code and message must be strings")
            if response.get("permission_denied") is True:
                raise PermissionError(f"{message} ({code})")
            if code == "CANCELLED":
                raise RSpiceCancelledError(code, message)
            raise RSpiceHostError(code, message)
        raise ValueError("host returned an unknown response status")

    def call_text(self, capability: str, operation: dict[str, object]) -> str | None:
        self.call(capability, operation)
        if capability == "environment-read" and self.last_value:
            self.secret_values.add(self.last_value)
        return self.last_value

    def redact_output(self, value: str) -> str:
        for secret in sorted(self.secret_values, key=len, reverse=True):
            value = value.replace(secret, "<redacted>")
        return value


def _required_handle(handle: str | None, operation: str) -> str:
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
        if _ACTIVE_SESSION is None:
            raise RuntimeError("Environment.get() requires an active RSpice session")
        value = _ACTIVE_SESSION.call_text(
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
                f"{logical_path!r} is not the selected run plan in this snapshot"
            )
        document_id = document.get("document_id")
        if not isinstance(document_id, str):
            raise ValueError("selected run plan has no stable document identity")
        handle = self._session.call(
            "project-read",
            {
                "operation": "load-run-plan",
                "project_handle": self._project_handle,
                "document_id": document_id,
            },
        )
        return RunPlan(self._session, _required_handle(handle, "load-run-plan"))


class Project:
    def __init__(self, session: _HostSession, handle: str) -> None:
        self._session = session
        self._handle = handle
        self.run_plans = _RunPlans(session, handle)

    @classmethod
    def open(cls, selector: str) -> "Project":
        if _ACTIVE_SESSION is None:
            raise RuntimeError("Project.open() requires an active RSpice session")
        if not isinstance(selector, str) or not selector.strip():
            raise ValueError("project selector must be a non-empty string")
        handle = _ACTIVE_SESSION.call(
            "project-read", {"operation": "open-project", "selector": selector}
        )
        return cls(_ACTIVE_SESSION, _required_handle(handle, "open-project"))


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
        return RunPreview(self._session, _required_handle(handle, "validate-run-plan"))


class RunPreview:
    def __init__(self, session: _HostSession, handle: str) -> None:
        self._session = session
        self._handle = handle

    def execute(self) -> "Run":
        handle = self._session.call(
            "simulation-execute",
            {"operation": "execute-run-plan", "preview_handle": self._handle},
        )
        return Run(self._session, _required_handle(handle, "execute-run-plan"))


class _Requirements:
    def __init__(self, run: "Run") -> None:
        self._run = run

    def evaluate(self, *, profile: str) -> None:
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
        values = list(formats)
        if not values or not all(isinstance(value, str) and value for value in values):
            raise ValueError("artifact formats must be a non-empty list of format names")
        self._session.call(
            "artifact-write",
            {
                "operation": "export-run",
                "run_handle": self._handle,
                "formats": values,
            },
        )


class _VirtualLoader(importlib.abc.MetaPathFinder, importlib.abc.Loader):
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

    def find_spec(self, fullname: str, path=None, target=None):
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
        logical_path = str(document["logical_path"])
        module.__file__ = f"rspice-project://{logical_path}"
        if is_package:
            module.__path__ = []
        exec(
            compile(str(document["source"]), module.__file__, "exec", dont_inherit=True),
            module.__dict__,
        )


_ACTIVE_SESSION: _HostSession | None = None
_AUDIT_GUARD_INSTALLED = False


def validate_browser_snapshot(snapshot_json: str) -> str:
    import uuid

    snapshot = json.loads(snapshot_json)
    documents = snapshot.get("documents")
    if not isinstance(documents, list) or not documents:
        raise ValueError("source snapshot documents are missing")
    entries = [item for item in documents if item.get("role") == "python-entry"]
    if len(entries) != 1:
        raise ValueError("source snapshot must contain exactly one Python entry")
    diagnostics: list[dict[str, object]] = []
    for document in documents:
        if document.get("role") not in ("python-entry", "python-module"):
            continue
        source = str(document.get("source", ""))
        path = str(document.get("logical_path", "<unknown>"))
        try:
            compile(source, f"rspice-project://{path}", "exec", dont_inherit=True)
        except SyntaxError as error:
            line = max(1, error.lineno or 1)
            column = max(1, error.offset or 1)
            end_line = max(line, error.end_lineno or line)
            end_column = max(column, error.end_offset or column + 1)

            def byte_offset(line_number: int, column_number: int) -> int:
                lines = source.splitlines(keepends=True)
                prefix = "".join(lines[: max(0, line_number - 1)])
                current = lines[line_number - 1] if 0 < line_number <= len(lines) else ""
                return len((prefix + current[: max(0, column_number - 1)]).encode("utf-8"))

            diagnostics.append(
                {
                    "diagnostic_id": str(uuid.uuid4()),
                    "document_id": document.get("document_id"),
                    "document_revision": document.get("revision"),
                    "severity": "error",
                    "source": "managed-pyodide",
                    "code": "PY-SYNTAX",
                    "message": error.msg,
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
            )
    return json.dumps(diagnostics, separators=(",", ":"))


def _install_public_module() -> None:
    module = types.ModuleType("rspice")
    module.__dict__.update(
        {
            "__version__": "1.0.0",
            "ArtifactFormat": ArtifactFormat,
            "Environment": Environment,
            "Project": Project,
            "RSpiceHostError": RSpiceHostError,
            "RSpiceCancelledError": RSpiceCancelledError,
        }
    )
    sys.modules["rspice"] = module


def _install_audit_guard() -> None:
    global _AUDIT_GUARD_INSTALLED
    if _AUDIT_GUARD_INSTALLED:
        return
    blocked = {"js", "pyodide", "rspice_bridge"}

    def audit(event: str, args: tuple[object, ...]) -> None:
        if event == "import" and args:
            root = str(args[0]).partition(".")[0]
            if root in blocked:
                raise PermissionError(
                    "direct browser/JavaScript access is denied; use the RSpice API"
                )
        if event in {"os.system", "subprocess.Popen"} or event.startswith("socket."):
            raise PermissionError("ambient process and network access is denied")

    sys.addaudithook(audit)
    _AUDIT_GUARD_INSTALLED = True


class _BrowserDebugRestart(BaseException):
    pass


class _BrowserDebugAdapter:
    def __init__(
        self,
        snapshot: dict[str, object],
        breakpoints: object,
        exception_policy: str,
    ) -> None:
        self.documents: dict[str, dict[str, object]] = {}
        self.document_ids: set[str] = set()
        self.document_line_counts: dict[str, int] = {}
        for document in snapshot.get("documents", []):
            if not isinstance(document, dict):
                continue
            path = document.get("logical_path")
            document_id = document.get("document_id")
            if (
                isinstance(path, str)
                and isinstance(document_id, str)
                and document.get("role") in ("python-entry", "python-module")
            ):
                self.documents[f"rspice-project://{path}"] = document
                self.document_ids.add(document_id)
                self.document_line_counts[document_id] = len(
                    str(document.get("source", "")).splitlines()
                )
        if exception_policy not in ("all", "uncaught", "never"):
            raise ValueError("browser debugger exception policy is invalid")
        self.exception_policy = exception_policy
        self.breakpoints: list[dict[str, object]] = []
        self.breakpoint_hits: dict[str, int] = {}
        self.set_breakpoints(breakpoints)
        self.frames: dict[int, types.FrameType] = {}
        self.stack_payload: list[dict[str, object]] = []
        self.value_references: dict[int, object] = {}
        self.next_value_reference = 1_000_000_000
        self.entry_pending = True
        self.resume_mode = "continue"
        self.step_depth = 0
        self.trace_events = 0

    def trace(self, frame: types.FrameType, event: str, arg: object):
        if frame.f_code.co_filename not in self.documents:
            return self.trace
        if event == "exception" and self.exception_policy == "all":
            error = arg[1] if isinstance(arg, tuple) and len(arg) > 1 else None
            self._pause(frame, "exception", str(error) if error is not None else "Python exception")
            return self.trace
        if event != "line":
            return self.trace
        self.trace_events += 1
        if self.trace_events % 64 == 0:
            command = json.loads(str(run_sync(debug_checkpoint())))
            if self._apply_async_control(command, frame):
                return self.trace
        depth = self._project_depth(frame)
        if self.entry_pending:
            self.entry_pending = False
            self._pause(frame, "entry", "Paused at the Automation entry point")
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

    def pause_uncaught(self, error: BaseException) -> None:
        if self.exception_policy != "uncaught":
            return
        frame = None
        value = error.__traceback__
        while value is not None:
            if value.tb_frame.f_code.co_filename in self.documents:
                frame = value.tb_frame
            value = value.tb_next
        if frame is not None:
            self._pause(frame, "exception", str(error) or type(error).__name__)

    def _pause(self, frame: types.FrameType, reason: str, description: str) -> None:
        self._capture_stack(frame)
        outbound = [
            {
                "event": {
                    "event": "state",
                    "state": "paused",
                    "detail": description,
                }
            },
            {
                "event": {
                    "event": "stopped",
                    "reason": reason,
                    "description": description,
                    "frame_id": self.stack_payload[0]["frame_id"] if self.stack_payload else None,
                }
            },
        ]
        while True:
            command = self._exchange(outbound)
            outbound = []
            operation = command.get("operation")
            request_id = command.get("request_id")
            if operation == "set-breakpoints":
                self.set_breakpoints(command.get("breakpoints"))
            elif operation == "stack-trace":
                start, count = self._slice(command.get("start"), command.get("count"))
                outbound.append(
                    {
                        "request_id": request_id,
                        "event": {
                            "event": "stack",
                            "frames": self.stack_payload[start : start + count],
                            "total": len(self.stack_payload),
                        },
                    }
                )
            elif operation == "variables":
                outbound.append(
                    {
                        "request_id": request_id,
                        "event": self._variables_event(
                            command.get("reference"),
                            command.get("start"),
                            command.get("count"),
                        ),
                    }
                )
            elif operation == "evaluate":
                outbound.append(
                    {
                        "request_id": request_id,
                        "event": self._evaluated_event(
                            command.get("frame_id"), command.get("expression")
                        ),
                    }
                )
            elif operation == "pause":
                continue
            elif operation == "stop":
                raise RSpiceCancelledError("CANCELLED", "Debug session was stopped")
            elif operation == "restart":
                raise _BrowserDebugRestart()
            elif operation in ("continue", "step-in", "step-over", "step-out"):
                self.resume_mode = str(operation)
                if operation in ("step-over", "step-out") and self.stack_payload:
                    active = self.frames.get(int(self.stack_payload[0]["frame_id"]))
                    self.step_depth = self._project_depth(active)
                self._exchange(
                    [
                        {
                            "event": {
                                "event": "state",
                                "state": "running",
                                "detail": f"debug control {operation} resumed execution",
                            }
                        }
                    ],
                    wait=False,
                )
                self.frames.clear()
                self.stack_payload.clear()
                self.value_references.clear()
                return
            else:
                raise ValueError(f"unsupported browser debugger command {operation!r}")

    def _exchange(self, events: list[dict[str, object]], *, wait: bool = True) -> dict[str, object]:
        payload = json.dumps({"events": events, "wait": wait}, separators=(",", ":"))
        response = json.loads(str(run_sync(debug_exchange(payload))))
        if not isinstance(response, dict):
            raise ValueError("browser debug bridge returned an invalid command")
        return response

    def _apply_async_control(self, command: dict[str, object], frame: types.FrameType) -> bool:
        operation = command.get("operation")
        if operation in (None, "none"):
            return False
        if operation == "set-breakpoints":
            self.set_breakpoints(command.get("breakpoints"))
            return False
        if operation == "pause":
            self._pause(frame, "pause", "Pause requested by user")
            return True
        if operation == "stop":
            raise RSpiceCancelledError("CANCELLED", "Debug session was stopped")
        if operation == "restart":
            raise _BrowserDebugRestart()
        return False

    def _apply_breakpoints(self, frame: types.FrameType) -> bool:
        document = self.documents.get(frame.f_code.co_filename)
        if document is None:
            return False
        for breakpoint in self.breakpoints:
            if (
                breakpoint.get("enabled") is not True
                or breakpoint.get("document_id") != document.get("document_id")
                or breakpoint.get("line") != frame.f_lineno
            ):
                continue
            kind = breakpoint.get("kind")
            if not isinstance(kind, dict):
                continue
            name = kind.get("kind")
            if name == "logpoint":
                template = str(kind.get("template", ""))
                rendered = re.sub(
                    r"\{([^{}]+)\}",
                    lambda match: self._log_value(match.group(1), frame),
                    template,
                )
                self._exchange(
                    [
                        {
                            "event": {
                                "event": "output",
                                "channel": "stdout",
                                "category": "logpoint",
                                "text": rendered[: 1024 * 1024],
                            }
                        }
                    ],
                    wait=False,
                )
                continue
            if name == "conditional" and not bool(
                self._evaluate(str(kind.get("expression", "")), frame)
            ):
                continue
            if name == "hit-count":
                identity = str(breakpoint.get("breakpoint_id"))
                hits = self.breakpoint_hits.get(identity, 0) + 1
                self.breakpoint_hits[identity] = hits
                if hits < int(kind.get("count", 1)):
                    continue
                condition = kind.get("condition")
                if isinstance(condition, str) and condition and not bool(
                    self._evaluate(condition, frame)
                ):
                    continue
            self._pause(frame, "breakpoint", f"Breakpoint at line {frame.f_lineno}")
            return True
        return False

    def set_breakpoints(self, breakpoints: object) -> None:
        if not isinstance(breakpoints, list) or len(breakpoints) > 100_000:
            raise ValueError("browser breakpoint collection is invalid")
        validated = []
        identities = set()
        for breakpoint in breakpoints:
            if not isinstance(breakpoint, dict):
                raise ValueError("browser breakpoint must be an object")
            identity = breakpoint.get("breakpoint_id")
            if (
                not isinstance(identity, str)
                or identity in identities
                or breakpoint.get("document_id") not in self.document_ids
                or not isinstance(breakpoint.get("line"), int)
                or int(breakpoint.get("line", 0)) <= 0
                or int(breakpoint.get("line", 0))
                > self.document_line_counts.get(str(breakpoint.get("document_id")), 0)
                or not isinstance(breakpoint.get("kind"), dict)
            ):
                raise ValueError("browser breakpoint identity or location is invalid")
            identities.add(identity)
            validated.append(dict(breakpoint))
        self.breakpoints = validated

    def _capture_stack(self, frame: types.FrameType) -> None:
        self.frames.clear()
        self.stack_payload.clear()
        self.value_references.clear()
        self.next_value_reference = 1_000_000_000
        frame_id = 1
        current: types.FrameType | None = frame
        while current is not None and len(self.stack_payload) < 10_000:
            document = self.documents.get(current.f_code.co_filename)
            if document is not None:
                locals_reference = frame_id * 4 + 1
                globals_reference = frame_id * 4 + 2
                self.frames[frame_id] = current
                self.stack_payload.append(
                    {
                        "frame_id": frame_id,
                        "name": current.f_code.co_name or "<module>",
                        "document_id": document["document_id"],
                        "range": self._source_range(document, current.f_lineno),
                        "locals_reference": locals_reference,
                        "globals_reference": globals_reference,
                    }
                )
                frame_id += 1
            current = current.f_back

    def _variables_event(self, reference: object, start: object, count: object) -> dict[str, object]:
        if not isinstance(reference, int) or reference <= 0:
            raise ValueError("browser variable reference is invalid")
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
            raise ValueError("browser variable reference is stale")
        start_value, count_value = self._slice(start, count)
        return {
            "event": "variables",
            "values": [
                self._variable(name, value)
                for name, value in values[start_value : start_value + count_value]
            ],
            "total": len(values),
        }

    def _evaluated_event(self, frame_id: object, expression: object) -> dict[str, object]:
        if (
            not isinstance(frame_id, int)
            or frame_id not in self.frames
            or not isinstance(expression, str)
            or not expression.strip()
        ):
            raise ValueError("browser watch request is invalid")
        try:
            result = self._variable(expression, self._evaluate(expression, self.frames[frame_id]))
        except BaseException as error:
            result = {
                "name": expression,
                "type_name": "error",
                "display_value": (str(error) or type(error).__name__)[:65536],
                "variables_reference": 0,
                "redacted": False,
            }
        return {"event": "evaluated", "expression": expression, "result": result}

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

    def _log_value(self, expression: str, frame: types.FrameType) -> str:
        try:
            return self._display(self._evaluate(expression, frame))
        except BaseException as error:
            return f"<error: {str(error) or type(error).__name__}>"

    def _variable(self, name: str, value: object) -> dict[str, object]:
        redacted = bool(re.search(r"(?i)(secret|token|password|credential|api[_-]?key)", str(name)))
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
            raise ValueError("browser debug collection slice is invalid")
        return start, count

    @staticmethod
    def _source_range(document: dict[str, object], line: int) -> dict[str, object]:
        source = str(document.get("source", ""))
        lines = source.splitlines(keepends=True)
        index = max(0, min(line - 1, max(len(lines) - 1, 0)))
        prefix = "".join(lines[:index])
        content = lines[index].rstrip("\r\n") if lines else ""
        start = len(prefix.encode("utf-8"))
        return {
            "start": {"line": max(line, 1), "column": 1, "byte_offset": start},
            "end": {
                "line": max(line, 1),
                "column": len(content) + 1,
                "byte_offset": start + len(content.encode("utf-8")),
            },
        }

    def _project_depth(self, frame: types.FrameType | None) -> int:
        depth = 0
        while frame is not None:
            if frame.f_code.co_filename in self.documents:
                depth += 1
            frame = frame.f_back
        return depth


def execute_browser_snapshot(
    snapshot_json: str,
    mode: str,
    output_limit: int,
    max_stack_depth: int,
    breakpoints_json: str = "[]",
    exception_policy: str = "uncaught",
) -> str:
    global _ACTIVE_SESSION
    snapshot = json.loads(snapshot_json)
    documents = snapshot.get("documents")
    if not isinstance(documents, list) or not documents:
        raise ValueError("source snapshot documents are missing")
    entries = [item for item in documents if item.get("role") == "python-entry"]
    if len(entries) != 1:
        raise ValueError("source snapshot must contain exactly one Python entry")
    for document in documents:
        if document.get("role") in ("python-entry", "python-module"):
            compile(
                str(document.get("source", "")),
                f"rspice-project://{document.get('logical_path', '<unknown>')}",
                "exec",
                dont_inherit=True,
            )
    entry = entries[0]
    loader = _VirtualLoader(documents)
    debugger = (
        _BrowserDebugAdapter(snapshot, json.loads(breakpoints_json), exception_policy)
        if mode == "debug"
        else None
    )
    session = _HostSession(snapshot, mode)
    _ACTIVE_SESSION = session
    _install_public_module()
    _install_audit_guard()
    sys.meta_path.insert(0, loader)
    budget = _OutputBudget(output_limit)
    stdout, stderr = _Capture(budget), _Capture(budget)
    original_stdout, original_stderr = sys.stdout, sys.stderr
    original_recursion_limit = sys.getrecursionlimit()
    error = None
    rendered_traceback = None
    restart = False
    cancelled = False
    try:
        sys.setrecursionlimit(min(max_stack_depth, 100_000))
        sys.stdout, sys.stderr = stdout, stderr
        if debugger is not None:
            sys.settrace(debugger.trace)
        logical_path = str(entry["logical_path"])
        globals_dict = {
            "__name__": "__main__",
            "__file__": f"rspice-project://{logical_path}",
            "__package__": (
                logical_path.rsplit("/", 1)[0].replace("/", ".")
                if "/" in logical_path
                else None
            ),
            "__builtins__": __builtins__,
        }
        exec(
            compile(str(entry["source"]), globals_dict["__file__"], "exec", dont_inherit=True),
            globals_dict,
        )
    except _BrowserDebugRestart:
        restart = True
    except RSpiceCancelledError:
        cancelled = True
    except BaseException as exception:
        try:
            if debugger is not None:
                debugger.pause_uncaught(exception)
        except _BrowserDebugRestart:
            restart = True
        except RSpiceCancelledError:
            cancelled = True
        else:
            error = str(exception) or type(exception).__name__
            rendered_traceback = "".join(traceback.format_exception(exception))
    finally:
        if debugger is not None:
            sys.settrace(None)
        sys.stdout, sys.stderr = original_stdout, original_stderr
        sys.setrecursionlimit(original_recursion_limit)
        sys.meta_path = [item for item in sys.meta_path if item is not loader]
        _ACTIVE_SESSION = None
    rendered_stdout = stdout.getvalue()
    rendered_stderr = stderr.getvalue()
    rendered_stdout = session.redact_output(rendered_stdout)
    rendered_stderr = session.redact_output(rendered_stderr)
    return json.dumps(
        {
            "ok": error is None,
            "stdout": rendered_stdout,
            "stderr": rendered_stderr,
            "error": error,
            "traceback": rendered_traceback,
            "restart": restart,
            "cancelled": cancelled,
        },
        separators=(",", ":"),
    )
