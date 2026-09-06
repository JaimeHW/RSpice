# rspice-xspice-catalog-dump

Emits the executable built-in XSPICE interface registry in a reviewable form.

The registry is assembled in Rust code rather than generated from a checked-in
catalog, so nothing else proves that what a release ships is internally
coherent. This tool is that proof: every registered name resolves, ports and
parameters are named and unique, and every declared bound describes a non-empty
range.

```text
rspice-xspice-catalog-dump                  # print the compiled registry, model by model
rspice-xspice-catalog-dump --validate-only  # audit it and print one line; the form CI runs
```

Non-zero exit means the audit failed, with each defect named.

## The count is not the check

The registry had 113 interfaces when this audit was introduced
(`BUILTIN_MODEL_BASELINE_COUNT`). Deliberate additions since then are listed by
name in `BUILTIN_ADDITIONS_SINCE_BASELINE`, and each one's executable
port/parameter contract is pinned individually. Keeping the baseline separate
from the additions is what prevents a bare count bump from silently blessing an
unrelated registration.

Adding a built-in model therefore means adding its name *and* its descriptor
check here, not editing a number.

## Running

```text
cargo run -p rspice-xspice-catalog-dump -- --validate-only
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
