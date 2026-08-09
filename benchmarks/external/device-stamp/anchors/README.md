# Hand-written compact-model anchors

This focused ngspice study estimates matrix-load cost per model instance and
Newton iteration for hand-written VBIC and BSIM4. It supplies an external order
of magnitude for RSpice's generated-stamp work; topology and matrix differences
mean it is not a direct release gate.

## Measurement

ngspice `.options acct` reports matrix-load wall time separately from reorder,
factor, and solve. Each model has a paired deck:

- `<model>_loaded.cir` contains 200 instances of the model under test.
- `<model>_empty.cir` preserves the node count and broad sparsity with linear
  resistors.

Each side is normalized by its own iteration count before subtraction:

```text
per_device_per_iteration =
  (loaded_load / loaded_iterations - empty_load / empty_iterations) / devices
```

The reproduction script runs seven paired samples by default, retains the raw
load times and iteration counts, and reports the median normalized cost. It
regenerates decks in a temporary directory and byte-compares them with the
checked-in inputs, so running it cannot rewrite the study definition.

## Comparability limits

- The ngspice decks ground source/emitter terminals; RSpice's generated-stamp
  harness leaves terminals live to retain the complete Jacobian block.
- The simulators use different matrix representations and stamping APIs.
- Two hundred identical instances measure throughput, not single-call latency.
- Default model cards do not represent every real process configuration.

Use these anchors to detect an order-of-magnitude backend problem. Do not quote
a cross-simulator ratio as same-model, same-topology latency.

## Running

Run under Bash with an explicitly selected release/console ngspice binary:

```text
RSPICE_BENCH_NGSPICE=/path/to/ngspice_con \
  bash benchmarks/external/device-stamp/anchors/run.sh
```

`RSPICE_BENCH_SAMPLES` changes the sample count. The optional first argument is
an output path. Without it, the script chooses a unique file under
`benchmarks/results/`. Publication is atomic and no-clobber.

The checked-in `anchors.json` is a historical ngspice 46 observation produced
before the current provenance and repeated-sampling policy. Preserve it as
context; use a new immutable result for current claims.
