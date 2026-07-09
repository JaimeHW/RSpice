#!/usr/bin/env python3
r"""Deploy a client commit with the current RSpice-Site main - git only, no gh.

    python3 tools/deploy/deploy.py [--ref BRANCH] [--tag NAME] [--allow-dirty]
    # Windows:  py tools\deploy\deploy.py

The deploy-site workflow triggers on a `site-v*` tag push, so the normal path
needs nothing but `git`: this pushes the selected client branch and a new
`site-vN` tag. GitHub Actions checks out RSpice-Site main, records both exact
commits, builds and gates both browser runtimes, and force-pushes cf-pages,
which Cloudflare Pages serves as production. Use workflow_dispatch when an
older or non-main site revision must be selected explicitly.

It prints the Actions URL to watch the run; if you have the GitHub CLI you can
stream it with `gh run watch`, but `gh` is never required.
"""

import argparse
import os
import re
import shutil
import subprocess
import sys


DEFAULT_SITE_REMOTE = "https://github.com/JaimeHW/RSpice-Site.git"


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


def site_main_revision():
    """Resolve the immutable RSpice-Site main commit embedded in the deploy tag."""
    remote = os.environ.get("RSPICE_SITE_REMOTE", DEFAULT_SITE_REMOTE)
    try:
        result = out(["git", "ls-remote", remote, "refs/heads/main"])
    except subprocess.CalledProcessError as error:
        sys.exit("failed to resolve RSpice-Site main from %s: %s" % (remote, error))
    fields = result.split()
    if len(fields) != 2 or fields[1] != "refs/heads/main" or not re.fullmatch(
        r"[0-9a-f]{40}", fields[0]
    ):
        sys.exit("RSpice-Site remote did not return exactly one valid main commit")
    return fields[0]


def commit_for_ref(ref):
    """Resolve the deploy ref once so push/tag target the same commit."""
    try:
        return out(["git", "rev-parse", "--verify", "%s^{commit}" % ref])
    except subprocess.CalledProcessError:
        sys.exit("ref '%s' does not resolve to a commit" % ref)


def validate_site_tag(tag):
    if not re.fullmatch(r"site-v[1-9][0-9]*", tag):
        sys.exit("deploy tag must match site-vN with N >= 1, got '%s'" % tag)


def main():
    ap = argparse.ArgumentParser(description="Push + tag to deploy the site (git only).")
    ap.add_argument("--ref", help="branch to push and deploy (default: current branch)")
    ap.add_argument("--tag", help="tag name to create (default: next site-vN)")
    ap.add_argument("--allow-non-main", action="store_true",
                    help="allow deploying a ref other than main")
    ap.add_argument("--allow-dirty", action="store_true",
                    help="proceed despite undeployed working-tree changes "
                         "(they will NOT be included; CI builds the pushed commit)")
    args = ap.parse_args()

    if not shutil.which("git"):
        sys.exit("required tool not found on PATH: git")

    branch = args.ref or out(["git", "rev-parse", "--abbrev-ref", "HEAD"])
    commit = commit_for_ref(branch)

    # Clean-tree guard. CI builds the pushed commit, never this working tree.
    # Untracked scratch files are not deploy inputs and are intentionally ignored.
    status = out(["git", "status", "--porcelain"])
    tracked_dirty = [ln for ln in status.splitlines() if not ln.startswith("??")]
    if tracked_dirty and not args.allow_dirty:
        print("Uncommitted changes to tracked files - these will NOT be deployed\n"
              "(CI builds the pushed commit, not your working tree):\n", file=sys.stderr)
        print("\n".join(tracked_dirty), file=sys.stderr)
        sys.exit("\ncommit them first, or re-run with --allow-dirty")

    if branch != "main":
        if not args.allow_non_main:
            sys.exit("ref '%s' is not main; pass --allow-non-main only for an intentional rollback or hotfix deploy" % branch)
        print("WARNING: deploying client ref '%s' (not main). This will combine "
              "that client commit with RSpice-Site main." % branch)

    tag = args.tag or next_site_tag()
    validate_site_tag(tag)
    short = out(["git", "rev-parse", "--short", commit])
    site_sha = site_main_revision()

    print("-> pushing %s to origin..." % branch)
    subprocess.run(["git", "push", "origin", branch], check=True)

    print("-> tagging %s at %s and pushing it (this is what triggers the deploy)..."
          % (tag, short))
    subprocess.run(["git", "tag", "-a", tag, commit,
                    "-m", "deploy site (%s %s)\n\nsite-source-sha: %s"
                    % (branch, short, site_sha)], check=True)
    subprocess.run(["git", "push", "origin", tag], check=True)

    url = actions_url()
    print("\nDeploy triggered by tag %s (commit %s)." % (tag, short))
    print("  pinned site:    " + site_sha)
    if url:
        print("  watch the run:  " + url)
        if shutil.which("gh"):
            print("  or stream it:   gh run watch  (gh detected, optional)")
    print("Cloudflare serves cf-pages within ~a minute after every gate passes.")
    print("  verify live:    https://rspice.app/build.json")
    print("                  client_source_sha should begin with %s" % short)
    print("                  site_source_sha identifies the assembled RSpice-Site commit")


if __name__ == "__main__":
    main()
