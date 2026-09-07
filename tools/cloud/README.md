# Vendored client provenance

Run `python tools/cloud/sync_vendored_client.py --source <clean-checkout>` to
import the four crates and test fixtures from the authoritative RSpice-Cloud
repository. The sync records the exact upstream commit and upstream file
hashes, applies `vendor.patch`, and records the resulting file hashes and patch
hash. `--check` verifies the materialized result. Re-sync after reviewing any
patch change; do not update hashes to conceal unexplained drift.

The downstream patch preserves two existing application changes:

- Protocols 3 and 4 both use revision content digest 2. Protocol 4 changes the
  result document, not the request identity. The regression tests retain the
  acceptance of 3/4 and rejection of incompatible versions.
- Locally authored crate READMEs describe application integration.

These changes are not claimed to originate at the upstream commit. Patch
application fails when upstream context changes, requiring a compatibility
review during the next sync. Drop a patch hunk when its change lands upstream.
