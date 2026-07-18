# Native backend release runbook

The `Native backend release` workflow creates immutable, checksummed, attested
archives for six native targets and publishes a CycloneDX SBOM. This document
is the operator procedure for producing, verifying, repairing, and rolling
back a release.

## Release contract

A native release must satisfy all of these invariants:

- The Git reference is an annotated `v<workspace-version>` tag.
- The tag resolves to one exact 40-character commit that is an ancestor of
  `origin/main`.
- Dependencies are locked and Rust warnings fail the release build.
- The binary reports the tagged version, release profile, target, and exact
  source commit, then passes the parser-to-solver readiness probe.
- Archives and checksum sidecars are deterministic for the tagged commit.
- Six target archives, seven checksum sidecars, a CycloneDX SBOM, and its
  checksum are present and verified before publication.
- Native packages and SBOM artifacts carry GitHub artifact attestations.
- An existing release asset is never overwritten. A rerun may upload a missing
  asset only after every same-named asset compares byte-for-byte.

Supported targets are x86_64 and arm64/aarch64 on Linux, macOS, and Windows.
Each archive contains the binary, licenses/notices, CLI and project READMEs,
the lockfile, the production worker config, both operations runbooks, and a
release manifest with exact source/build identity and payload hashes.

## Preflight

1. Land the intended release on `main`; do not release from an unmerged branch.
2. Update the workspace version in `Cargo.toml`, refresh `Cargo.lock`, and land
   that change through normal review.
3. Confirm the `CI`, `Nightly`, `Security`, Python, coverage, and relevant web
   release gates are green for the exact commit.
4. On the fixed benchmark host, pass the same-host macro regression gate in
   [`benchmarks/README.md`](../../benchmarks/README.md). Archive both baseline
   and candidate scoreboards with the release evidence.
5. Review open incidents, advisory exceptions, conformance regressions, and
   dependency/license changes. Do not release with an unexplained gate waiver.
6. Confirm the working tree is clean and `main` matches `origin/main`.

Useful local policy checks are:

```bash
cargo deny check advisories bans licenses sources
cargo audit
python tools/security/check_advisory_exceptions.py
python tools/release/test_package_native.py
rspice --config config/production.toml health --json
```

## Tag and publish

Create a signed annotated tag when maintainer signing is available; an
annotated tag is the minimum enforced contract.

```bash
git switch main
git pull --ff-only origin main
git status --short
git tag -s vX.Y.Z -m "RSpice X.Y.Z"
git push origin vX.Y.Z
```

If signing is not configured, use `git tag -a` after the release approver has
verified the commit. Never use or move a lightweight tag for a native release.

The tag push starts `.github/workflows/native-release.yml`. The validation job
resolves and pins the source commit before any matrix build. Do not publish
manually while the workflow is running.

## Verify the published release

The publish job verifies all sidecars before creating or repairing the GitHub
Release. Independently verify at least one artifact for every platform shipped
to production, plus the SBOM:

```bash
gh release download vX.Y.Z --repo JaimeHW/RSpice --dir release-vX.Y.Z
cd release-vX.Y.Z
sha256sum --check --strict *.sha256
gh attestation verify rspice-X.Y.Z-x86_64-unknown-linux-gnu.tar.gz \
  --repo JaimeHW/RSpice
gh attestation verify rspice-X.Y.Z.cdx.json --repo JaimeHW/RSpice
```

On Windows, use `Get-FileHash -Algorithm SHA256` to compare an archive with its
sidecar if `sha256sum` is unavailable. Extract the package and verify:

```bash
./rspice --version
./rspice --config production.toml health --json
```

Confirm that version, target, profile, and commit match the release manifest
and the annotated tag. Retain the verification output with release approval
evidence.

## Recover an interrupted publish

A failed publish may leave a GitHub Release with only some assets. Rerun the
workflow against the original tag:

```bash
gh workflow run native-release.yml --ref vX.Y.Z --repo JaimeHW/RSpice
```

The workflow rebuilds from the tag's exact commit. For each expected asset it
downloads any existing same-named asset and compares bytes. Identical assets
are retained; missing assets are uploaded; any mismatch fails with
`refusing to replace immutable release asset`.

If a mismatch occurs, stop. Preserve both candidates and investigate runner,
toolchain, source-date, or supply-chain drift. Do not use `--clobber`, delete
the release, or move/recreate the tag. Fix the deterministic build defect and
publish a new patch version.

## Revoke or roll back

Published artifacts and tags are an audit record and remain immutable. For a
bad release:

1. Stop new deployment and route new jobs to the last verified binary/config/
   model-bundle tuple.
2. Mark the affected release clearly in operator-facing release notes and the
   incident record; preserve its assets for forensic verification unless legal
   or security response requires otherwise.
3. Do not resume its checkpoints on an older binary unless that path was
   explicitly tested. Prefer finishing them on an isolated affected-version
   worker or restarting from immutable inputs.
4. Land the regression test and correction on `main`, increment the patch
   version, and publish a new annotated tag through the full workflow.

There is no mutable backend database in the native package. Deployment
rollback consists of selecting the prior verified artifact tuple and draining
the suspect workers as described in
[`production-runbook.md`](production-runbook.md).
