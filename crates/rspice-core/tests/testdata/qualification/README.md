# Core qualification data

## Structural baseline

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

## Independent coupled-line reference

`cpl_ibm2_telegrapher.tsv` is consumed by
[`cpl_telegrapher.rs`](../../cpl_telegrapher.rs). It records the independent
telegrapher-equation solution for four coupled-line port voltages at three
times, not machine-specific performance measurements or RSpice-generated output.
The fixture header records the circuit, method, and convergence evidence.

To reproduce the reference with NumPy installed, run from the repository root:

```sh
python tools/qualification/cpl_ibm2_telegrapher.py --refine
```
