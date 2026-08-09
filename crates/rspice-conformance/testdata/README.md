# Conformance-owned fixtures

`veriloga-golden/` contains the complete, deterministic stamp fingerprints for
the generated compact-model corpus. They are correctness artifacts, not timing
baselines.

Verify them through the owning crate:

```text
cargo run --locked --release -p rspice-conformance \
  --features veriloga-builtins-models \
  --bin rspice-veriloga-golden -- verify
```

Capture requires an explicit, nonexistent output directory and publishes the
complete model set transactionally:

```text
cargo run --locked --release -p rspice-conformance \
  --features veriloga-builtins-models \
  --bin rspice-veriloga-golden -- capture --out target/veriloga-golden-candidate
```

There is no in-place replacement mode. Promote a candidate only after reviewing
the complete numerical diff and running the independent derivative audit. A
capture produced by the implementation under test is a snapshot, not proof that
the captured answer is correct.
