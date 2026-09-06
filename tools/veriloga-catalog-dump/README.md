# rspice-veriloga-catalog-dump

Validates and emits the generated Verilog-A catalog compiled into a release.
Unlike its XSPICE sibling this is a release utility, not just a development aid:
it depends on `rspice-core` with `veriloga-builtins`, so running it *is* a proof
that the full catalog links.

```text
rspice-veriloga-catalog-dump                  # print every compiled model descriptor
rspice-veriloga-catalog-dump --validate-only  # audit and print one line; the form CI runs
```

Non-zero exit means the audit failed, with each defect named.

## What it checks

- Every descriptor declares `GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION`, so a
  stale generated artifact cannot ride along in a release.
- No two models share a case-insensitive name.
- The compiled catalog contains exactly `EXPECTED_SHIPPED_MODEL_COUNT` models
  (43). A shipped catalog is not allowed to be a subset by accident.
- The workspace `README.md` states that same count, in each of the specific
  claims the release documentation makes about it. Documentation that contradicts
  the binary is treated as a release defect rather than a cosmetic one.

## Running

```text
cargo run -p rspice-veriloga-catalog-dump -- --validate-only
```

This builds the complete 43-model corpus. Expect a long, memory-hungry compile
the first time; see [rspice-veriloga-models](../../crates/rspice-veriloga-models)
for why the corpus is split across independent packages.

Licensed under the [RSpice Personal Use License](../../LICENSE).
