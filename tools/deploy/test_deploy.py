#!/usr/bin/env python3
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DEPLOY = ROOT / "tools" / "deploy" / "deploy.py"
PUBLISH = ROOT / "tools" / "deploy" / "publish.py"


def git(cwd, *args, check=True):
    return subprocess.run(
        ["git", *args],
        cwd=cwd,
        capture_output=True,
        text=True,
        check=check,
    )


def write_file(root, rel, text):
    path = root / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


class DeployScriptTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.base = Path(self.tmp.name)
        self.remote = self.base / "origin.git"
        self.repo = self.base / "repo"

        subprocess.run(["git", "init", "--bare", str(self.remote)], check=True)
        subprocess.run(["git", "init", "--initial-branch=main", str(self.repo)], check=True)
        git(self.repo, "config", "user.name", "Deploy Test")
        git(self.repo, "config", "user.email", "deploy-test@example.invalid")
        git(self.repo, "remote", "add", "origin", str(self.remote))

        write_file(self.repo, "site/index.html", "main\n")
        git(self.repo, "add", "site/index.html")
        git(self.repo, "commit", "-m", "initial site")
        git(self.repo, "push", "origin", "main")

    def deploy(self, *args):
        env = os.environ.copy()
        env["PYTHONDONTWRITEBYTECODE"] = "1"
        return subprocess.run(
            [sys.executable, str(DEPLOY), *args],
            cwd=self.repo,
            capture_output=True,
            text=True,
            env=env,
        )

    def test_ref_tags_the_requested_ref_commit(self):
        main_commit = git(self.repo, "rev-parse", "HEAD").stdout.strip()
        git(self.repo, "switch", "-c", "site-work")
        write_file(self.repo, "site/index.html", "site work\n")
        git(self.repo, "add", "site/index.html")
        git(self.repo, "commit", "-m", "site work")
        target_commit = git(self.repo, "rev-parse", "HEAD").stdout.strip()
        git(self.repo, "switch", "main")

        result = self.deploy("--ref", "site-work", "--tag", "site-v999")

        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        tag_commit = git(self.remote, "rev-parse", "refs/tags/site-v999^{}").stdout.strip()
        self.assertEqual(target_commit, tag_commit)
        self.assertNotEqual(main_commit, tag_commit)

    def test_untracked_site_files_block_deploy(self):
        write_file(self.repo, "site/new-page.html", "untracked\n")

        result = self.deploy("--tag", "site-v1000")

        self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("Untracked site files", result.stderr)
        self.assertIn("site/new-page.html", result.stderr.replace("\\", "/"))
        tag = git(self.remote, "rev-parse", "--verify", "refs/tags/site-v1000", check=False)
        self.assertNotEqual(tag.returncode, 0)


class PublishScriptTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.base = Path(self.tmp.name)
        self.remote = self.base / "origin.git"
        self.site = self.base / "_site"

        subprocess.run(["git", "init", "--bare", str(self.remote)], check=True)
        write_file(self.site, "index.html", "v1\n")
        write_file(self.site, "assets/app.js", "console.log('v1');\n")

    def publish(self, *args, env=None):
        run_env = os.environ.copy()
        run_env["PYTHONDONTWRITEBYTECODE"] = "1"
        if env:
            run_env.update(env)
        return subprocess.run(
            [sys.executable, str(PUBLISH), *args],
            cwd=self.base,
            capture_output=True,
            text=True,
            env=run_env,
        )

    def test_publish_force_pushes_single_orphan_commit(self):
        result = self.publish(
            "--remote",
            str(self.remote),
            "--branch",
            "cf-pages",
            "--dir",
            str(self.site),
            "--message",
            "deploy first",
        )

        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertFalse((self.site / ".git").exists(), "throwaway .git must be removed")
        first_commit = git(self.remote, "rev-parse", "cf-pages").stdout.strip()
        first_parents = git(self.remote, "rev-list", "--parents", "-n", "1", "cf-pages").stdout
        self.assertEqual(len(first_parents.split()), 1, "publish commit must be orphaned")
        tree = git(self.remote, "ls-tree", "-r", "--name-only", "cf-pages").stdout
        self.assertIn("index.html", tree)
        self.assertIn("assets/app.js", tree)

        write_file(self.site, "index.html", "v2\n")
        result = self.publish(
            "--remote",
            str(self.remote),
            "--branch",
            "cf-pages",
            "--dir",
            str(self.site),
            "--message",
            "deploy second",
        )

        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        second_commit = git(self.remote, "rev-parse", "cf-pages").stdout.strip()
        self.assertNotEqual(first_commit, second_commit)
        self.assertEqual(git(self.remote, "rev-list", "--count", "cf-pages").stdout.strip(), "1")

    def test_publish_cleans_throwaway_git_when_push_fails(self):
        missing_remote = self.base / "missing.git"

        result = self.publish(
            "--remote",
            str(missing_remote),
            "--branch",
            "cf-pages",
            "--dir",
            str(self.site),
        )

        self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertFalse(
            (self.site / ".git").exists(),
            "failed publish must not leave a nested git repository in _site",
        )


if __name__ == "__main__":
    unittest.main()
