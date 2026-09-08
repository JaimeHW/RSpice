"""Exercise the production build-identity helper through actual incremental Cargo builds."""

import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
HELPER = ROOT / "crates/rspice-ui/build_identity.rs"


class BuildIdentityTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="rspice-build-identity-")
        self.addCleanup(self.temporary.cleanup)
        self.directory = Path(self.temporary.name)
        self.repo = self.directory / "repo"
        self.repo.mkdir()
        self.env = {key: value for key, value in os.environ.items()
                    if not key.startswith(("GIT_", "CARGO_", "RUST"))}
        self.env.update(GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull)

    def command(self, *args, cwd=None, env=None):
        result = subprocess.run(args, cwd=cwd or self.repo, env=env or self.env,
                                text=True, capture_output=True, check=False)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        return result.stdout.strip()

    def git(self, *args, cwd=None):
        return self.command("git", "-c", "core.hooksPath=", "-c", "commit.gpgSign=false",
                            "-c", "user.name=Build identity fixture",
                            "-c", "user.email=fixture@example.invalid", *args, cwd=cwd)

    def source(self, root):
        crate = root / "crates/ui"
        (crate / "src").mkdir(parents=True)
        (crate / "Cargo.toml").write_text(
            '[package]\nname="build_identity_fixture"\nversion="0.0.0"\nedition="2024"\n'
            '[workspace]\n', encoding="utf-8")
        shutil.copyfile(HELPER, crate / "build_identity.rs")
        (crate / "build.rs").write_text(
            'mod build_identity;\nfn main() {\n'
            'println!("cargo:rerun-if-changed=build_identity.rs");\n'
            'build_identity::emit(&std::path::PathBuf::from('
            'std::env::var_os("CARGO_MANIFEST_DIR").unwrap()));\n}\n', encoding="utf-8")
        (crate / "src/main.rs").write_text(
            'fn main() { println!("{}", env!("RSPICE_BUILD_HASH")); }\n', encoding="utf-8")
        (root / ".gitignore").write_text("Cargo.lock\n", encoding="utf-8")

    def initialize(self, *options):
        self.git("init", "--initial-branch=main", *options)
        self.source(self.repo)
        self.git("add", ".")
        self.git("commit", "-m", "Initial source")

    def build(self, root=None, env=None):
        root = root or self.repo
        target = self.directory / ("target-" + root.name)
        observed = self.command(
            "cargo", "run", "--offline", "--quiet", "--manifest-path",
            str(root / "crates/ui/Cargo.toml"), "--target-dir", str(target), cwd=root, env=env)
        outputs = list((target / "debug/build").glob("build_identity_fixture-*/output"))
        self.assertEqual(len(outputs), 1)
        # A second unchanged build must reuse the build-script output. Missing
        # watched paths otherwise hide the bug by forcing every build to rerun.
        stamp = outputs[0].stat().st_mtime_ns
        repeated = self.command(
            "cargo", "run", "--offline", "--quiet", "--manifest-path",
            str(root / "crates/ui/Cargo.toml"), "--target-dir", str(target), cwd=root, env=env)
        self.assertEqual(repeated, observed)
        self.assertEqual(outputs[0].stat().st_mtime_ns, stamp, "unchanged build reran helper")
        return observed

    def assert_current(self, root=None):
        root = root or self.repo
        self.assertEqual(self.build(root), self.git("rev-parse", "--short=9", "HEAD", cwd=root))

    def advance(self, root=None):
        self.git("commit", "--allow-empty", "-m", "Advance without source changes", cwd=root)

    def test_branch_advance_and_detached_checkout(self):
        self.initialize()
        first = self.git("rev-parse", "HEAD")
        self.assert_current()
        head = self.repo / ".git/HEAD"
        original = (head.read_bytes(), head.stat().st_mtime_ns)
        self.advance()
        self.assertEqual((head.read_bytes(), head.stat().st_mtime_ns), original)
        self.assert_current()
        self.git("checkout", "--detach", first)
        self.assert_current()
        self.advance()
        self.assert_current()

    def test_linked_worktree_uses_its_own_head_and_shared_branch(self):
        self.initialize()
        linked = self.directory / "linked"
        self.git("worktree", "add", "-b", "linked", str(linked))
        self.assertTrue((linked / ".git").is_file())
        self.assert_current(linked)
        self.advance(linked)
        self.assert_current(linked)
        self.assert_current(self.repo)

    def test_packed_branch_tracks_loose_and_packed_updates(self):
        self.initialize()
        self.git("branch", "-m", "nested/main")
        self.git("pack-refs", "--all", "--prune")
        self.assert_current()
        self.advance()
        self.assert_current()
        self.git("pack-refs", "--all", "--prune")
        self.assert_current()
        self.advance()
        self.git("pack-refs", "--all", "--prune")
        self.assert_current()

    def test_reftable_branch_and_linked_detached_head(self):
        self.initialize("--ref-format=reftable")
        self.assert_current()
        self.advance()
        self.assert_current()
        linked = self.directory / "linked"
        self.git("worktree", "add", "--detach", str(linked))
        self.assert_current(linked)
        self.advance(linked)
        self.assert_current(linked)

    def test_archive_does_not_inherit_enclosing_repository_identity(self):
        self.initialize()
        archive = self.repo / "untracked-archive"
        self.source(archive)
        self.assertEqual(self.build(archive), "unknown")
        standalone = self.directory / "archive"
        self.source(standalone)
        self.assertEqual(self.build(standalone), "unknown")

    def test_callers_git_directory_cannot_override_source_identity(self):
        self.initialize()
        other = self.directory / "other"
        other.mkdir()
        self.git("init", "--initial-branch=main", cwd=other)
        self.advance(other)
        env = dict(self.env, GIT_DIR=str(other / ".git"), GIT_WORK_TREE=str(other))
        self.assertEqual(self.build(env=env), self.git("rev-parse", "--short=9", "HEAD"))


if __name__ == "__main__":
    unittest.main()
