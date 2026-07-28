# Hand-written compact-model anchors

The Verilog-A backend gate in `design/VERILOGA_BACKEND_PLAN.md` is stated against
hand-written C: *corpus median at or below 1.00x ngspice C, no model above 1.50x*.
This directory is what makes that number reproducible instead of remembered.

## What is measured

ngspice's `.options acct` reports **`Matrix load time`** — the wall time spent
evaluating devices and stamping their contributions, separate from reorder,
factor and solve. That is the same span a generated RSpice device covers in one
`stamp` call, which is why it is the quantity being compared.

`acct` also reports an iteration count (printed as *"Transient iterations for the
last time point"*, which is the running iteration counter regardless of the
analysis). Load time divided by that count is time per Newton iteration for the
whole circuit.

## Why two decks per model

A deck's load time includes every device in it, sources and parasitic resistors
included, and its iteration count depends on how hard the circuit is to converge.
Neither is separable from a single run. Each anchor therefore ships as a pair:

- `<model>_loaded.cir` — `N` instances of the device under test
- `<model>_empty.cir` — identical topology, identical node count, identical
  sweep, with each device under test replaced by a linear resistor

The resistor keeps the matrix the same size and the sparsity pattern comparable,
so subtracting removes the harness and leaves the model:

```
per_device_per_iteration = (load_loaded / iters_loaded - load_empty / iters_empty) / N
```

Each side is normalised by its *own* iteration count before subtracting, because
the two decks do not converge in the same number of iterations and pretending
they do silently charges the difference to the device.

## Reading the result

These are order-of-magnitude anchors, not a controlled benchmark:

- **Terminals are grounded here and floating in RSpice's harness.** These decks
  tie the emitter/source to ground, as a real circuit does. `rspice-core`'s stamp
  harness deliberately leaves every terminal ungrounded so the Jacobian block is
  structurally dense — the worst case, and more importantly the *same* case for
  every model and revision, which is what makes its numbers comparable to each
  other. It is not the same case as this. RSpice's side is pessimistic by
  whatever the extra live derivative lanes cost, so a ratio computed across the
  two is an upper bound on RSpice's true cost, not an estimate of it.
- ngspice and RSpice do not use the same matrix format, so stamping costs differ
  by a constant that is not the model's.
- `N` is large enough that per-call overhead is amortised, which flatters both
  sides equally but means the number is throughput, not latency.
- Model cards are ngspice defaults, matching the convention in
  `rspice-core`'s stamp harness: what a backend's cost depends on is operation
  count and derivative width, and no card is equally representative across
  forty-odd models.

Read the ratio, not the difference. For the exact same-model figure, compare the
scalar and kernel tiers of one model family inside RSpice's own harness.

## Running

```bash
RSPICE_BENCH_NGSPICE=/path/to/ngspice_con ./run.sh
```

Writes `anchors.json` next to the decks. The environment variable matches the one
`rspice-bench run` already uses, so a machine configured for one is configured
for the other.
