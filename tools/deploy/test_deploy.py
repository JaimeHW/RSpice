#!/usr/bin/env python3
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DEPLOY = ROOT / "tools" / "deploy" / "deploy.py"


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


if __name__ == "__main__":
    unittest.main()
