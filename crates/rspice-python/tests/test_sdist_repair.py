from __future__ import annotations

import importlib.util
from pathlib import Path

import pytest


SCRIPT = Path(__file__).parents[1] / "scripts" / "repair_sdist_lock.py"
SPEC = importlib.util.spec_from_file_location("repair_sdist_lock", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
repair_sdist_lock = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(repair_sdist_lock)


def lockfile(*packages: str) -> bytes:
    return ("version = 4\n\n" + "\n\n".join(packages) + "\n").encode()


REGISTRY_PACKAGE = "\n".join(
    (
        "[[package]]",
        'name = "numpy-equivalent"',
        'version = "2.0.0"',
        'source = "registry+https://github.com/rust-lang/crates.io-index"',
        'checksum = "0123456789abcdef"',
    )
)

GIT_PACKAGE = "\n".join(
    (
        "[[package]]",
        'name = "git-model"',
        'version = "1.2.3"',
        'source = "git+https://example.invalid/model?rev=abc#abc"',
    )
)

WORKSPACE_PACKAGE = "\n".join(
    (
        "[[package]]",
        'name = "rspice-unused-member"',
        'version = "0.1.0"',
    )
)


def test_reconciliation_allows_only_package_removal() -> None:
    original = lockfile(REGISTRY_PACKAGE, GIT_PACKAGE, WORKSPACE_PACKAGE)
    repaired = lockfile(REGISTRY_PACKAGE)

    repair_sdist_lock._validate_reconciliation(original, repaired)


@pytest.mark.parametrize(
    "changed",
    [
        REGISTRY_PACKAGE.replace('version = "2.0.0"', 'version = "2.0.1"'),
        REGISTRY_PACKAGE.replace("0123456789abcdef", "fedcba9876543210"),
        REGISTRY_PACKAGE.replace("crates.io-index", "untrusted.invalid/index"),
        GIT_PACKAGE.replace("#abc", "#def"),
    ],
)
def test_reconciliation_rejects_new_or_changed_external_packages(changed: str) -> None:
    original = lockfile(REGISTRY_PACKAGE, GIT_PACKAGE, WORKSPACE_PACKAGE)
    repaired = lockfile(REGISTRY_PACKAGE, GIT_PACKAGE, changed)

    with pytest.raises(
        repair_sdist_lock.RepairError,
        match="introduced or changed external packages",
    ):
        repair_sdist_lock._validate_reconciliation(original, repaired)


def test_lock_parser_rejects_invalid_utf8() -> None:
    with pytest.raises(repair_sdist_lock.RepairError, match="not valid UTF-8"):
        repair_sdist_lock._external_package_identities(b"\xff")
