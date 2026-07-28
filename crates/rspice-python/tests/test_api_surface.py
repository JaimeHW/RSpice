"""Pins the module's public surface against a checked-in golden.

The binding source is being reorganized into per-analysis modules. Those moves
are supposed to be pure code motion, but a lost method, a renamed keyword, or a
dropped base class compiles perfectly well and only shows up as a broken
downstream script. This test records the surface a caller can actually reach —
every exported name, every public member of every exported class, each member's
descriptor kind and signature, and the exception hierarchy — and fails on any
drift.

`stubtest` covers similar ground but runs on a single CI leg and needs a full
mypy install; this runs everywhere the suite runs, in milliseconds.

Regenerating after a *deliberate* API change:

    RSPICE_UPDATE_API_SURFACE=1 python -m pytest tests/test_api_surface.py

Review the resulting `api_surface.json` diff as carefully as the code change —
that file is the contract.
"""

from __future__ import annotations

import inspect
import json
import os
from pathlib import Path
from typing import Any

import pytest

import rspice

GOLDEN = Path(__file__).with_name("api_surface.json")

# Dunders that are part of a type's observable protocol. Everything else
# (__doc__, __module__, __dict__, ...) is noise that PyO3 or CPython supplies.
PROTOCOL_DUNDERS = frozenset(
    {
        "__bool__",
        "__contains__",
        "__copy__",
        "__deepcopy__",
        "__eq__",
        "__ge__",
        "__getitem__",
        "__gt__",
        "__hash__",
        "__iter__",
        "__le__",
        "__len__",
        "__lt__",
        "__ne__",
        "__new__",
        "__next__",
        "__reduce__",
        "__repr__",
        "__str__",
    }
)

# Values that legitimately change between builds. Their presence and type are
# pinned; their contents are not.
VOLATILE_VALUES = frozenset({"__version__", "__author__"})


def _signature(obj: Any) -> str | None:
    """Render a callable's signature, or None when it exposes none.

    PyO3 emits `__text_signature__` for most methods, but not for every
    generated `__new__` or operator slot. A missing signature is recorded as
    None rather than skipped, so gaining or losing one is itself a diff.
    """
    try:
        return str(inspect.signature(obj))
    except (TypeError, ValueError):
        return None


def _member_kind(owner: type, name: str) -> str:
    """Classify a member without invoking it.

    `getattr_static` avoids triggering property getters, which on a PyO3 class
    would raise for an unbound descriptor.
    """
    try:
        attr = inspect.getattr_static(owner, name)
    except AttributeError:  # pragma: no cover - defensive
        return "missing"
    return type(attr).__name__


def _describe_class(cls: type) -> dict[str, Any]:
    members: dict[str, Any] = {}
    for name in dir(cls):
        if name.startswith("__") and name not in PROTOCOL_DUNDERS:
            continue
        # Private helpers such as `_unpickle` are reachable by pickle but are
        # not API; stubtest ignores them for the same reason.
        if name.startswith("_") and not name.startswith("__"):
            continue
        kind = _member_kind(cls, name)
        entry: dict[str, Any] = {"kind": kind}
        if kind not in {"getset_descriptor", "member_descriptor", "property"}:
            entry["signature"] = _signature(getattr(cls, name, None))
        members[name] = entry

    return {
        "type": "class",
        # Pins the hybrid exception design: RSpiceKeyError must stay both an
        # RSpiceError and a builtin KeyError.
        "bases": [base.__name__ for base in cls.__bases__],
        "mro": [base.__name__ for base in cls.__mro__],
        "members": dict(sorted(members.items())),
    }


def build_surface() -> dict[str, Any]:
    surface: dict[str, Any] = {}
    for name in sorted(rspice.__all__):
        obj = getattr(rspice, name)
        if name in VOLATILE_VALUES:
            surface[name] = {"type": "value", "value_type": type(obj).__name__}
        elif inspect.isclass(obj):
            surface[name] = _describe_class(obj)
        elif callable(obj):
            surface[name] = {"type": "function", "signature": _signature(obj)}
        else:  # pragma: no cover - defensive
            surface[name] = {"type": "value", "value_type": type(obj).__name__}
    return {"__all__": sorted(rspice.__all__), "objects": surface}


def test_public_surface_matches_golden() -> None:
    surface = build_surface()

    if os.environ.get("RSPICE_UPDATE_API_SURFACE"):
        GOLDEN.write_text(json.dumps(surface, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        pytest.skip("api_surface.json regenerated; review the diff before committing")

    assert GOLDEN.exists(), (
        "tests/api_surface.json is missing; regenerate with "
        "RSPICE_UPDATE_API_SURFACE=1 python -m pytest tests/test_api_surface.py"
    )
    expected = json.loads(GOLDEN.read_text(encoding="utf-8"))

    assert surface["__all__"] == expected["__all__"], "rspice.__all__ changed"

    actual_objects = surface["objects"]
    expected_objects = expected["objects"]
    for name in expected_objects:
        assert name in actual_objects, f"{name} disappeared from the module surface"
        assert actual_objects[name] == expected_objects[name], (
            f"public surface of {name} changed"
        )
    assert set(actual_objects) == set(expected_objects), "module gained undeclared names"


def test_every_exported_name_is_reachable() -> None:
    """`__all__` and the real module contents must not drift apart."""
    for name in rspice.__all__:
        assert hasattr(rspice, name), f"{name} is exported but not defined"
