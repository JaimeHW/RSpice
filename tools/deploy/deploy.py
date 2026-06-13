#!/usr/bin/env python3
r"""Take committed site/ changes live on rspice.app — one command.

    python3 tools/deploy/deploy.py [--ref BRANCH] [--allow-dirty]
    # Windows:  py tools\deploy\deploy.py

This does NOT build or publish locally. The build stays in CI for a
clean-room, reproducible result: GitHub Actions runs
tools/deploy/build_site.py from the pushed commit, gates it, and
force-pushes _site/ to the gh-pages branch, which Cloudflare Pages serves
as production. This wrapper just drives that pipeline:

  1. refuse a dirty tree   — uncommitted edits would NOT be deployed
                             (CI builds the pushed commit, not your worktree)
  2. git push              — land your commits on origin
  3. gh workflow run       — dispatch deploy-site on the pushed ref
  4. gh run watch          — stream the run; Cloudflare picks up gh-pages
                             ~1 min after every gate passes

Needs the GitHub CLI (`gh`) authenticated with workflow scope.
"""

import argparse
import shutil
import subprocess
import sys
import time

WORKFLOW = "deploy-site.yml"


def out(cmd):
    return subprocess.run(cmd, capture_output=True, text=True, check=True).stdout.strip()


def need(tool):
    if not shutil.which(tool):
        sys.exit("required tool not found on PATH: " + tool)


def latest_run_id():
    return out(["gh", "run", "list", "--workflow", WORKFLOW, "--limit", "1",
                "--json", "databaseId", "-q", ".[0].databaseId"])


def main():
    ap = argparse.ArgumentParser(description="Push + trigger + watch the site deploy.")
    ap.add_argument("--ref", help="branch to push and deploy (default: current branch)")
    ap.add_argument("--allow-dirty", action="store_true",
                    help="proceed despite uncommitted tracked changes "
                         "(they will NOT be included — CI builds the pushed commit)")
    args = ap.parse_args()

    need("git")
    need("gh")

    branch = args.ref or out(["git", "rev-parse", "--abbrev-ref", "HEAD"])

    # 1 · clean-tree guard — untracked files are fine (they never deploy);
    #     uncommitted edits to *tracked* files are the foot-gun.
    status = out(["git", "status", "--porcelain"])
    tracked_dirty = [ln for ln in status.splitlines() if not ln.startswith("??")]
    if tracked_dirty and not args.allow_dirty:
        print("Uncommitted changes to tracked files - these will NOT be deployed\n"
              "(CI builds the pushed commit, not your working tree):\n",
              file=sys.stderr)
        print("\n".join(tracked_dirty), file=sys.stderr)
        sys.exit("\ncommit them first, or re-run with --allow-dirty")

    if branch != "main":
        print("WARNING: deploying ref '%s' (not main). The site's source of truth "
              "is main; this will publish %s's site/ to production." % (branch, branch))

    # 2 · push
    print("-> pushing %s to origin..." % branch)
    subprocess.run(["git", "push", "origin", branch], check=True)

    # 3 · dispatch (note the run id beforehand so we watch the NEW one)
    before = latest_run_id()
    print("-> dispatching %s on %s..." % (WORKFLOW, branch))
    subprocess.run(["gh", "workflow", "run", WORKFLOW, "--ref", branch], check=True)

    run_id = None
    for _ in range(20):
        time.sleep(2)
        current = latest_run_id()
        if current and current != before:
            run_id = current
            break
    if not run_id:
        sys.exit("dispatched, but the new run did not appear — check:\n"
                 "  gh run list --workflow %s" % WORKFLOW)

    # 4 · watch (the run keeps going even if you Ctrl-C this)
    print("-> watching run %s ..." % run_id)
    watched = subprocess.run(["gh", "run", "watch", run_id, "--exit-status"])
    if watched.returncode != 0:
        sys.exit("deploy run %s failed — inspect:\n"
                 "  gh run view %s --log-failed" % (run_id, run_id))

    print("\nDeployed. Cloudflare serves gh-pages within ~a minute.")
    print("  verify:  https://rspice.app/build.json   (source_sha should match the pushed HEAD)")


if __name__ == "__main__":
    main()
