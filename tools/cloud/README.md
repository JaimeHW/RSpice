# Vendored client provenance

`rspice-pack`, `rspice-cloud-domain`, `rspice-cloud-contract` and
`rspice-cloud-client` are vendored, not authored here. They live in the private
RSpice-Cloud repository beside the API they are tested against, and this
workspace cannot take a git dependency on it: `deny.toml` admits only
crates.io sources. The vendored trees are read-only here; behavioural authority
stays in RSpice-Cloud.

Run `python tools/cloud/sync_vendored_client.py --source <clean-checkout>` to
import those four crates and their test fixtures. Native licensing fixtures
live beside their module in
`crates/rspice-cloud-client/src/native_licensing/testdata/` and are included
with the crate sources. The sync records the exact upstream commit and upstream
file hashes, applies `vendor.patch`, and records the resulting file hashes and patch
hash. `--check` verifies the materialized result. Re-sync after reviewing any
patch change; do not update hashes to conceal unexplained drift.

The downstream patch preserves three existing application changes:

- Protocols 3 and 4 both use revision content digest 2. Protocol 4 changes the
  result document, not the request identity. The regression tests retain the
  acceptance of 3/4 and rejection of incompatible versions.
- Locally authored crate READMEs describe application integration.
- `rspice-cloud-client` enables reqwest's `stream` feature for native targets
  only. The browser build streams through its own Fetch adapter and the
  workspace `wasm-streams`; reqwest's feature there would link a second
  `wasm-streams` with the same exported JavaScript symbols.

These changes are not claimed to originate at the upstream commit. Patch
application fails when upstream context changes, requiring a compatibility
review during the next sync. Drop a patch hunk when its change lands upstream.
