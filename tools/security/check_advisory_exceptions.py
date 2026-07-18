#!/usr/bin/env python3
"""Validate that every advisory exception is explicit, owned, and unexpired."""

from __future__ import annotations

import re
import sys
import tomllib
from datetime import date
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
ID_PATTERN = re.compile(r"^RUSTSEC-\d{4}-\d{4}$")
REQUIRED_TEXT_FIELDS = (
    "dependency",
    "owner",
    "reason",
    "mitigations",
    "exit_criteria",
)


def load_toml(path: Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def deny_exception_ids(document: dict) -> set[str]:
    identifiers: set[str] = set()
    for entry in document.get("advisories", {}).get("ignore", []):
        identifier = entry.get("id") if isinstance(entry, dict) else entry
        if isinstance(identifier, str):
            identifiers.add(identifier)
    return identifiers


def audit_exception_ids(document: dict) -> set[str]:
    return {
        entry
        for entry in document.get("advisories", {}).get("ignore", [])
        if isinstance(entry, str)
    }


def main() -> int:
    errors: list[str] = []
    registry = load_toml(ROOT / "security" / "advisory-exceptions.toml")
    if registry.get("schema_version") != 1:
        errors.append("security/advisory-exceptions.toml must use schema_version = 1")

    records = registry.get("exceptions", [])
    if not isinstance(records, list):
        errors.append("security exception registry must contain [[exceptions]] records")
        records = []

    registry_ids: set[str] = set()
    today = date.today()
    for index, record in enumerate(records, start=1):
        if not isinstance(record, dict):
            errors.append(f"exception record {index} must be a TOML table")
            continue
        identifier = record.get("id")
        label = identifier if isinstance(identifier, str) else f"record {index}"
        if not isinstance(identifier, str) or not ID_PATTERN.fullmatch(identifier):
            errors.append(f"{label}: id must match RUSTSEC-YYYY-NNNN")
        elif identifier in registry_ids:
            errors.append(f"{identifier}: duplicate exception record")
        else:
            registry_ids.add(identifier)

        for field in REQUIRED_TEXT_FIELDS:
            value = record.get(field)
            if not isinstance(value, str) or not value.strip():
                errors.append(f"{label}: {field} must be non-empty text")

        review_text = record.get("review_after")
        try:
            review_after = date.fromisoformat(review_text)
        except (TypeError, ValueError):
            errors.append(f"{label}: review_after must be an ISO date")
        else:
            if review_after < today:
                errors.append(
                    f"{label}: review expired on {review_after.isoformat()} "
                    f"(today is {today.isoformat()})"
                )

    deny_ids = deny_exception_ids(load_toml(ROOT / "deny.toml"))
    audit_ids = audit_exception_ids(load_toml(ROOT / ".cargo" / "audit.toml"))
    for policy_name, policy_ids in (
        ("deny.toml", deny_ids),
        (".cargo/audit.toml", audit_ids),
    ):
        missing = sorted(policy_ids - registry_ids)
        stale = sorted(registry_ids - policy_ids)
        if missing:
            errors.append(
                f"{policy_name}: undocumented exceptions: {', '.join(missing)}"
            )
        if stale:
            errors.append(
                f"{policy_name}: registry exceptions missing from policy: "
                f"{', '.join(stale)}"
            )

    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 1

    print(
        f"validated {len(registry_ids)} advisory exception(s); "
        f"next review no later than "
        f"{min(record['review_after'] for record in records)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
