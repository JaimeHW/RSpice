import sys
import unittest
from pathlib import Path


sys.path.insert(0, str(Path(__file__).resolve().parent))
from repair_sdist_lock import (  # noqa: E402
    RepairError,
    _external_package_identities,
    _validate_reconciliation,
)


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


class ReconciliationTests(unittest.TestCase):
    def test_reconciliation_allows_only_package_removal(self) -> None:
        """Pruning a workspace member is the whole point; that must pass."""
        original = lockfile(REGISTRY_PACKAGE, GIT_PACKAGE, WORKSPACE_PACKAGE)
        repaired = lockfile(REGISTRY_PACKAGE)

        _validate_reconciliation(original, repaired)

    def test_reconciliation_rejects_new_or_changed_external_packages(self) -> None:
        """A repair that moves a dependency is a supply-chain change, not a repair.

        Regenerating the lockfile is allowed to drop pruned workspace members
        and nothing else. If it bumps a version, rewrites a checksum, swaps the
        registry, or moves a git revision, the published sdist would build
        against something other than what was tested.
        """
        for label, changed in (
            ("version bump", REGISTRY_PACKAGE.replace('version = "2.0.0"', 'version = "2.0.1"')),
            ("checksum swap", REGISTRY_PACKAGE.replace("0123456789abcdef", "fedcba9876543210")),
            ("registry swap", REGISTRY_PACKAGE.replace("crates.io-index", "untrusted.invalid/index")),
            ("git revision move", GIT_PACKAGE.replace("#abc", "#def")),
        ):
            with self.subTest(change=label):
                original = lockfile(REGISTRY_PACKAGE, GIT_PACKAGE, WORKSPACE_PACKAGE)
                repaired = lockfile(REGISTRY_PACKAGE, GIT_PACKAGE, changed)

                with self.assertRaisesRegex(
                    RepairError, "introduced or changed external packages"
                ):
                    _validate_reconciliation(original, repaired)

    def test_lock_parser_rejects_invalid_utf8(self) -> None:
        with self.assertRaisesRegex(RepairError, "not valid UTF-8"):
            _external_package_identities(b"\xff")


if __name__ == "__main__":
    unittest.main()
