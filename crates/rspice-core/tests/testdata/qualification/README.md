# Core qualification baseline

`rspice-qualification-v1.json` is consumed by
[`qualification_baseline.rs`](../../qualification_baseline.rs).

It records capabilities, toolchain/features, oracle evidence, result-document
structure, and operation/storage counts. It contains no wall-clock timing or
resident-memory measurements. Each count gate declares a tolerance; capability
and structural facts are compared exactly.

Regenerate from the repository root only after reviewing an intentional
behavior change:

```sh
RSPICE_UPDATE_QUALIFICATION_BASELINE=1 \
  cargo test --locked -p rspice-core --test qualification_baseline
```

Review the resulting diff before committing. Machine-specific timing baselines
and ordinary measurements belong in ignored `target/benchmarks/` or CI artifact
storage; see the [benchmark tool](../../../../../tools/rspice-bench/README.md).
