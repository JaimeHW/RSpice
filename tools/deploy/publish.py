#!/usr/bin/env python3
r"""Force-publish a built site directory as a single orphan commit to a branch.

Cloudflare Pages watches that branch (cf-pages) and serves it as production.
This replaces the third-party publish action: same force-orphan behaviour --
each deploy is exactly one commit, so the multi-MB wasm bundles never accrete
history on the branch -- but plain git, all-Python, and no external dependency.

CI usage (see .github/workflows/deploy-site.yml):
    GH_TOKEN=<token> python3 tools/deploy/publish.py --branch cf-pages --dir _site

Environment:
  GH_TOKEN / GITHUB_TOKEN   push credential (required unless --remote is given)
  GITHUB_REPOSITORY         owner/repo, used to build the push URL
  GITHUB_SHA                source commit, used in the commit message
`--remote` overrides the constructed URL (used by the local self-test against a
bare repo, so the publish path is exercised without touching the real remote).
"""

import argparse
import os
import shutil
import stat
import subprocess
import sys


def git(args, cwd):
    subprocess.run(["git", *args], cwd=cwd, check=True)


def force_rmtree(path):
    """Remove a tree even if it holds read-only files. git marks loose objects
    0444, and on Windows the read-only bit blocks plain shutil.rmtree; clear it
    first so the throwaway .git is always gone (version-proof, no onerror/onexc)."""
    if not os.path.exists(path):
        return
    for root, dirs, files in os.walk(path):
        for name in dirs + files:
            try:
                os.chmod(os.path.join(root, name), stat.S_IWRITE)
            except OSError:
                pass
    shutil.rmtree(path, ignore_errors=True)


def main():
    ap = argparse.ArgumentParser(description="Orphan force-push a site dir to a branch.")
    ap.add_argument("--branch", default="cf-pages", help="target branch (default: cf-pages)")
    ap.add_argument("--dir", default="_site", help="built site directory (default: _site)")
    ap.add_argument("--remote", help="push URL override (default: built from env token+repo)")
    ap.add_argument("--message", help="commit message (default: 'deploy <GITHUB_SHA>')")
    args = ap.parse_args()

    site = os.path.abspath(args.dir)
    if not os.path.isdir(site):
        sys.exit("%s not found - run build_site.py first" % site)

    remote = args.remote
    if not remote:
        token = os.environ.get("GH_TOKEN") or os.environ.get("GITHUB_TOKEN")
        repo = os.environ.get("GITHUB_REPOSITORY")
        if not token or not repo:
            sys.exit("need --remote, or GH_TOKEN/GITHUB_TOKEN + GITHUB_REPOSITORY in the environment")
        remote = "https://x-access-token:%s@github.com/%s.git" % (token, repo)

    message = args.message or ("deploy %s" % os.environ.get("GITHUB_SHA", "")).strip()

    # A throwaway repo rooted at the build dir: stage everything, one commit,
    # force-push. Wiping any prior .git guarantees a single commit even if this
    # runs twice on the same checkout. The build dir is gitignored by the outer
    # repo, so the nested .git is harmless.
    force_rmtree(os.path.join(site, ".git"))
    ident = ["-c", "user.name=rspice-deploy", "-c", "user.email=deploy@rspice.app"]
    git(["init", "-q", "-b", args.branch], site)
    git(["add", "-A"], site)
    git(ident + ["commit", "-q", "-m", message], site)
    git(["push", "--force", "--quiet", remote, args.branch], site)
    force_rmtree(os.path.join(site, ".git"))

    print("published %s as a single commit to '%s' (%s)" % (args.dir, args.branch, message))


if __name__ == "__main__":
    main()
