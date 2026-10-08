import sys
import tempfile
import unittest
from pathlib import Path, PurePosixPath


sys.path.insert(0, str(Path(__file__).resolve().parent))
from repair_sdist_lock import (  # noqa: E402
    RepairError,
    _embedded_workspace_files,
    _external_package_identities,
    _missing_embedded_files,
    _missing_local_patch_files,
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

    def test_unused_patch_does_not_overwrite_preceding_registry_identity(self) -> None:
        original = lockfile(REGISTRY_PACKAGE)
        repaired = original + b'\n[[patch.unused]]\nname = "zip"\nversion = "8.6.0"\n'
        _validate_reconciliation(original, repaired)
        self.assertEqual(_external_package_identities(original), _external_package_identities(repaired))


def write(root: Path, relative: str, text: str) -> None:
    path = root / relative
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


class EmbeddedFileTests(unittest.TestCase):
    """`cargo package --list` stops at the package boundary; the compile does not."""

    def archive(self, root: Path) -> None:
        write(root, "Cargo.toml", "[workspace]\n")
        write(root, "crates/core/Cargo.toml", '[package]\nname = "core"\n')
        # `assets/pack.lib` is deliberately absent: an asset outside the package
        # directory is exactly what maturin leaves out.
        write(root, "crates/core/src/inside.txt", "inside\n")

    def test_only_out_of_package_literals_from_the_compiled_tree_are_reported(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.archive(root)
            write(
                root,
                "crates/core/src/lib.rs",
                'const PACK: &str = include_str!("../../../assets/pack.lib");\n'
                'const NEAR: &str = include_str!("inside.txt");\n'
                'const GEN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/x"));\n',
            )
            # A test target is never built by an sdist install, so what it embeds
            # must not enter the archive.
            write(
                root,
                "crates/core/tests/it.rs",
                'const DECK: &str = include_str!("../../../tools/rspice-bench/circuits/deck.cir");\n',
            )

            self.assertEqual(
                _embedded_workspace_files(root),
                {PurePosixPath("assets/pack.lib"): PurePosixPath("crates/core/src/lib.rs")},
            )

    def test_an_embed_escaping_the_archive_root_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.archive(root)
            write(
                root,
                "crates/core/src/lib.rs",
                'const OUT: &str = include_str!("../../../../outside.lib");\n',
            )

            with self.assertRaisesRegex(RepairError, "escapes the archive"):
                _embedded_workspace_files(root)

    def test_a_file_the_checkout_cannot_supply_is_an_error(self) -> None:
        """Silently shipping the incomplete archive is the defect being fixed."""
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "archive"
            checkout = Path(temporary) / "checkout"
            checkout.mkdir()
            root.mkdir()
            self.archive(root)
            write(
                root,
                "crates/core/src/lib.rs",
                'const PACK: &str = include_str!("../../../assets/pack.lib");\n',
            )

            with self.assertRaisesRegex(RepairError, "neither the source distribution"):
                _missing_embedded_files(root, checkout)

            write(checkout, "assets/pack.lib", "* pack\n")
            self.assertEqual(
                _missing_embedded_files(root, checkout),
                {PurePosixPath("assets/pack.lib"): checkout / "assets" / "pack.lib"},
            )

    def test_a_file_the_archive_already_carries_is_not_added_twice(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "archive"
            checkout = Path(temporary) / "checkout"
            checkout.mkdir()
            root.mkdir()
            self.archive(root)
            write(
                root,
                "crates/core/src/lib.rs",
                'const NEAR: &str = include_str!("inside.txt");\n',
            )

            self.assertEqual(_missing_embedded_files(root, checkout), {})


class LocalPatchTests(unittest.TestCase):
    def fixture(self, temporary: str) -> tuple[Path, Path]:
        root = Path(temporary) / "archive"
        checkout = Path(temporary) / "checkout"
        write(root, "Cargo.toml", '[patch.crates-io]\nzip = { path = "vendor/zip" }\n')
        write(checkout, "vendor/zip/Cargo.toml", '[package]\nname = "zip"\n')
        write(checkout, "vendor/zip/src/lib.rs", "// patched library\n")
        write(checkout, "vendor/zip/LICENSE", "license\n")
        return root, checkout

    def test_restores_patch_tree_without_build_or_git_artifacts(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root, checkout = self.fixture(temporary)
            write(checkout, "vendor/zip/target/debug/stale", "build artifact")
            write(checkout, "vendor/zip/.git/config", "git artifact")
            self.assertEqual(
                set(_missing_local_patch_files(root, checkout)),
                {PurePosixPath(f"vendor/zip/{name}") for name in ("Cargo.toml", "src/lib.rs", "LICENSE")},
            )

    def test_existing_identical_files_are_not_added_again(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root, checkout = self.fixture(temporary)
            write(root, "vendor/zip/LICENSE", "license\n")
            self.assertNotIn(PurePosixPath("vendor/zip/LICENSE"), _missing_local_patch_files(root, checkout))
            write(root, "vendor/zip/LICENSE", "different snapshot\n")
            with self.assertRaisesRegex(RepairError, "differs from checkout"):
                _missing_local_patch_files(root, checkout)

    def test_missing_patch_and_unsafe_paths_are_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root, checkout = self.fixture(temporary)
            (checkout / "vendor/zip/Cargo.toml").unlink()
            with self.assertRaisesRegex(RepairError, "no checkout manifest"):
                _missing_local_patch_files(root, checkout)
            for path in ("../outside", "/absolute", "C:/drive", r"C:\\drive", "."):
                with self.subTest(path=path):
                    write(root, "Cargo.toml", f"[patch.crates-io]\nzip = {{ path = '{path}' }}\n")
                    with self.assertRaisesRegex(RepairError, "unsafe local patch path"):
                        _missing_local_patch_files(root, checkout)

    def test_no_local_patch_needs_no_checkout_files(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            write(root, "Cargo.toml", '[patch.crates-io]\nzip = { git = "https://example.invalid/zip" }\n')
            self.assertEqual(_missing_local_patch_files(root, root), {})


class RepositorySourceDistributionTests(unittest.TestCase):
    def test_spice_embed_is_only_the_foundation_library(self) -> None:
        root = Path(__file__).resolve().parents[2]
        spice_embeds = sorted(
            path.as_posix()
            for path in _embedded_workspace_files(root)
            if path.as_posix().startswith("models/spice/")
        )
        self.assertEqual(
            spice_embeds,
            ["models/spice/foundation/lib/foundation.lib"],
        )


if __name__ == "__main__":
    unittest.main()
