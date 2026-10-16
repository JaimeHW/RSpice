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
# A yanked release is named the way cargo-deny's ignore list takes it: one
# package spec, `name@version`, so an exception can never cover a later yank.
CRATE_PATTERN = re.compile(r"^[A-Za-z0-9_-]+@\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.+-]+)?$")
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


def deny_exceptions(document: dict) -> tuple[set[str], set[str]]:
    """Split cargo-deny's ignore list into advisory ids and yanked releases."""
    identifiers: set[str] = set()
    crates: set[str] = set()
    for entry in document.get("advisories", {}).get("ignore", []):
        if isinstance(entry, dict):
            if isinstance(entry.get("crate"), str):
                crates.add(entry["crate"])
                continue
            entry = entry.get("id")
        if isinstance(entry, str):
            (crates if "@" in entry else identifiers).add(entry)
    return identifiers, crates


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
    registry_crates: set[str] = set()
    today = date.today()
    for index, record in enumerate(records, start=1):
        if not isinstance(record, dict):
            errors.append(f"exception record {index} must be a TOML table")
            continue
        identifier = record.get("id")
        crate = record.get("crate")
        label = next(
            (value for value in (identifier, crate) if isinstance(value, str)),
            f"record {index}",
        )
        if (identifier is None) == (crate is None):
            errors.append(
                f"{label}: give exactly one of id (an advisory) "
                "or crate (a yanked release)"
            )
        elif identifier is not None:
            if not isinstance(identifier, str) or not ID_PATTERN.fullmatch(identifier):
                errors.append(f"{label}: id must match RUSTSEC-YYYY-NNNN")
            elif identifier in registry_ids:
                errors.append(f"{identifier}: duplicate exception record")
            else:
                registry_ids.add(identifier)
        elif not isinstance(crate, str) or not CRATE_PATTERN.fullmatch(crate):
            errors.append(f"{label}: crate must name one yanked release as name@version")
        elif crate in registry_crates:
            errors.append(f"{crate}: duplicate exception record")
        else:
            registry_crates.add(crate)

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

    deny = load_toml(ROOT / "deny.toml")
    audit = load_toml(ROOT / ".cargo" / "audit.toml")
    deny_ids, deny_crates = deny_exceptions(deny)
    audit_ids = audit_exception_ids(audit)
    for policy_name, policy_ids, documented in (
        ("deny.toml", deny_ids, registry_ids),
        ("deny.toml", deny_crates, registry_crates),
        (".cargo/audit.toml", audit_ids, registry_ids),
    ):
        missing = sorted(policy_ids - documented)
        stale = sorted(documented - policy_ids)
        if missing:
            errors.append(
                f"{policy_name}: undocumented exceptions: {', '.join(missing)}"
            )
        if stale:
            errors.append(
                f"{policy_name}: registry exceptions missing from policy: "
                f"{', '.join(stale)}"
            )

    # cargo audit cannot scope a yanked exception to one release, so while one
    # exists its yanked check is off and cargo-deny is the only gate on yanked
    # releases. That is sound only while cargo-deny fails on every yanked
    # release it was not told about, and the check must come back with the
    # last exception.
    audit_checks_yanked = audit.get("yanked", {}).get("enabled", True)
    if not audit_checks_yanked and deny.get("advisories", {}).get("yanked") != "deny":
        errors.append(
            '.cargo/audit.toml disables yanked checks, so deny.toml must keep yanked = "deny"'
        )
    if registry_crates and audit_checks_yanked:
        errors.append(
            ".cargo/audit.toml must set [yanked] enabled = false while a yanked "
            "release is accepted; cargo audit cannot scope the exception"
        )
    if not registry_crates and not audit_checks_yanked:
        errors.append(
            ".cargo/audit.toml disables yanked checks with no accepted yanked release"
        )

    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 1

    print(
        f"validated {len(registry_ids)} advisory and "
        f"{len(registry_crates)} yanked-release exception(s); "
        f"next review no later than "
        f"{min(record['review_after'] for record in records)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
