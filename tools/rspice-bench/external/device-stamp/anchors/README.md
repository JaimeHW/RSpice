# Hand-written compact-model comparison

This ngspice study estimates matrix-load cost per model instance and Newton
iteration for hand-written VBIC and BSIM4. It is a diagnostic comparison,
not a release gate.

Each `<model>_loaded.cir` contains 200 model instances.
`<model>_empty.cir` preserves the node count and broad sparsity with resistors.
The script uses ngspice's `.options acct` statistics and computes:

```text
per_device_per_iteration =
  (loaded_load / loaded_iterations - empty_load / empty_iterations) / devices
```

It runs seven paired samples by default, retains the raw accounting fields,
and reports the median normalized cost. Generated decks are byte-compared
with the checked-in inputs before measurement.

## Running

Run under Bash from the repository root with a release/console ngspice binary:

```sh
RSPICE_BENCH_NGSPICE=/path/to/ngspice_con \
  bash tools/rspice-bench/external/device-stamp/anchors/run.sh
```

`RSPICE_BENCH_SAMPLES` changes the sample count. The optional first argument
sets an output path. Otherwise, the script writes a unique report under
ignored `target/benchmarks/`. Publication is atomic and refuses overwrites.
Keep reports local or in CI artifact storage.

The ngspice decks ground source/emitter terminals, while RSpice's
generated-stamp harness retains the complete live-terminal Jacobian.
Matrix representations and stamping APIs also differ. Two hundred identical
instances measure throughput rather than single-call latency, and default
model cards do not cover real process variation.
