#!/usr/bin/env python3
r"""Take committed site/ changes live on rspice.app — git only, no gh.

    python3 tools/deploy/deploy.py [--ref BRANCH] [--tag NAME] [--allow-dirty]
    # Windows:  py tools\deploy\deploy.py

The deploy-site workflow triggers on a `site-v*` tag push, so shipping needs
nothing but `git`: this pushes your branch and a new `site-vN` tag. GitHub
Actions then builds (tools/deploy/build_site.py), gates, and force-pushes
cf-pages, which Cloudflare Pages serves as production. The build stays in CI
for a clean-room, reproducible result — nothing is built or published locally.

It prints the Actions URL to watch the run; if you have the GitHub CLI you can
stream it with `gh run watch`, but `gh` is never required.
"""

import argparse
import re
import shutil
import subprocess
import sys


def out(cmd):
    return subprocess.run(cmd, capture_output=True, text=True, check=True).stdout.strip()


def next_site_tag():
    """Next `site-vN`, one past the highest existing numbered tag (local+fetched)."""
    highest = 0
    for tag in out(["git", "tag", "--list", "site-v*"]).splitlines():
        m = re.fullmatch(r"site-v(\d+)", tag.strip())
        if m:
            highest = max(highest, int(m.group(1)))
    return "site-v%d" % (highest + 1)


def actions_url():
    """https://github.com/<owner>/<repo>/actions/... from the origin remote."""
    try:
        remote = out(["git", "remote", "get-url", "origin"])
    except subprocess.CalledProcessError:
        return None
    m = re.search(r"github\.com[:/](.+?)(?:\.git)?$", remote)
    return ("https://github.com/%s/actions/workflows/deploy-site.yml" % m.group(1)
            if m else None)


def commit_for_ref(ref):
    """Resolve the deploy ref once so push/tag target the same commit."""
    try:
        return out(["git", "rev-parse", "--verify", "%s^{commit}" % ref])
    except subprocess.CalledProcessError:
        sys.exit("ref '%s' does not resolve to a commit" % ref)


def main():
    ap = argparse.ArgumentParser(description="Push + tag to deploy the site (git only).")
    ap.add_argument("--ref", help="branch to push and deploy (default: current branch)")
    ap.add_argument("--tag", help="tag name to create (default: next site-vN)")
    ap.add_argument("--allow-dirty", action="store_true",
                    help="proceed despite uncommitted tracked changes "
                         "(they will NOT be included — CI builds the pushed commit)")
    args = ap.parse_args()

    if not shutil.which("git"):
        sys.exit("required tool not found on PATH: git")

    branch = args.ref or out(["git", "rev-parse", "--abbrev-ref", "HEAD"])
    commit = commit_for_ref(branch)

    # clean-tree guard — untracked files are fine (they never deploy);
    # uncommitted edits to *tracked* files are the foot-gun.
    status = out(["git", "status", "--porcelain"])
    tracked_dirty = [ln for ln in status.splitlines() if not ln.startswith("??")]
    if tracked_dirty and not args.allow_dirty:
        print("Uncommitted changes to tracked files - these will NOT be deployed\n"
              "(CI builds the pushed commit, not your working tree):\n", file=sys.stderr)
        print("\n".join(tracked_dirty), file=sys.stderr)
        sys.exit("\ncommit them first, or re-run with --allow-dirty")

    if branch != "main":
        print("WARNING: deploying ref '%s' (not main). The site's source of truth "
              "is main; this will publish %s's site/ to production." % (branch, branch))

    tag = args.tag or next_site_tag()
    short = out(["git", "rev-parse", "--short", commit])

    print("-> pushing %s to origin..." % branch)
    subprocess.run(["git", "push", "origin", branch], check=True)

    print("-> tagging %s at %s and pushing it (this is what triggers the deploy)..."
          % (tag, short))
    subprocess.run(["git", "tag", "-a", tag, commit,
                    "-m", "deploy site (%s %s)" % (branch, short)], check=True)
    subprocess.run(["git", "push", "origin", tag], check=True)

    url = actions_url()
    print("\nDeploy triggered by tag %s (commit %s)." % (tag, short))
    if url:
        print("  watch the run:  " + url)
        if shutil.which("gh"):
            print("  or stream it:   gh run watch  (gh detected, optional)")
    print("Cloudflare serves cf-pages within ~a minute after every gate passes.")
    print("  verify live:    https://rspice.app/build.json   (source_sha should be %s)" % short)


if __name__ == "__main__":
    main()
