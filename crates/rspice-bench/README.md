# RSpice Bench

The macro-benchmark rig: a standalone binary that times **whole simulator
processes** — `rspice run <deck> -q` against a locally installed
`ngspice -b <deck>` — over the shared deck set in
[`benchmarks/circuits/`](../../benchmarks/circuits/), and emits a JSON
scoreboard plus a human-readable table. It is deliberately macro, not
micro: parsing, solving, and output formatting are all inside the measured
wall-clock, so the numbers reflect what a user actually waits for. It is
the regression yardstick for performance work — the convention is that no
optimization claim lands without a before/after scoreboard from this rig.

It is **not** a Criterion/Divan harness and has no `[[bench]]` targets;
timing is `std::time::Instant` around child-process spawn and an OS-backed
timed wait, without polling-interval quantization. Alongside it, the
in-process subcommands isolate one layer each — `klu` for the solver kernels,
`native-jit` for generated Verilog-A entrypoints — so an optimization there is
attributable to a phase rather than diluted across a whole process.

## Layout

| File | Contents |
| :--- | :--- |
| `src/main.rs` | CLI entry point: `gen`, `generated-rust`, `generated-compile`, `generated-stamp`, `klu`, `native-jit`, and `run` subcommands |
| `src/runner.rs` | The `run` subcommand: locates executables, runs warmup + timed repeats per deck/simulator, computes min/median/mean and the median speedup, writes the scoreboard, prints the table |
| `src/generate.rs` | The `gen` subcommand: deterministically regenerates the generated decks (RC ladders, MOS array) with byte-stable output — fixed formatting, no timestamps |
| `src/generated_rust.rs` | Authenticated source-resource report and budget gate for checked-in Verilog-A Rust kernels |
| `src/generated_compile.rs` | Reproducible generated-catalog compile-time measurement with package-only rebuild auditing |
| `src/provenance.rs` | Shared source, toolchain, target, host, and repository provenance capture |
| `src/report.rs` | Deterministic JSON report serialization and fail-closed verdict helpers |
| `src/klu.rs` | The `klu` subcommand: in-process KLU kernel benchmark — analyze/factor/refactor/solve medians and fill per circuit-shaped pattern, with optional per-nonzero budgets |
| `src/native_jit.rs` | The `native-jit` subcommand: in-process Verilog-A native JIT benchmark gate with median speedup and optional p95 budgets |
| `src/error.rs` | `BenchError` with full context on every failure path |

The macro runner has no feature flags. The `native-jit` subcommand links the
`rspice-veriloga` native feature so it can benchmark generated native x64
entrypoints without exposing the low-level JIT ABI outside the compiler crate.
The `klu` subcommand depends on `rspice-matrix` directly rather than on the
`rspice-core` re-export, which keeps it clear of the optional corpus-building
dependency and off the ~40-minute build path.

## Building and running

```bash
# Build the simulator under test and the rig together
cargo build --release -p rspice-cli -p rspice-bench

# Run the suite (writes benchmarks/scoreboards/scoreboard.json)
target/release/rspice-bench run

# Gate a candidate on the same host and with the same repeat count/deck set
target/release/rspice-bench run \
  --baseline benchmarks/scoreboards/reference.json \
  --out benchmarks/scoreboards/candidate.json

# Run the native Verilog-A JIT gate
target/release/rspice-bench native-jit

# Measure the KLU solver kernels in isolation
target/release/rspice-bench klu --out benchmarks/scoreboards/klu.json

# Authenticate and measure the checked-in generated Rust bundle
target/release/rspice-bench generated-rust \
  --out benchmarks/scoreboards/generated-rust.json

# Measure an isolated release check of the generated catalog
target/release/rspice-bench generated-compile \
  --out benchmarks/scoreboards/generated-compile.json

# Measure five representative generated models against hand-written BSIM4
cargo run -p rspice-bench --release --features generated-stamp-subset -- \
  generated-stamp --max-corpus-median-reference-ratio 1.25

# Regenerate the generated decks (verify with git diff afterwards)
target/release/rspice-bench gen
```

### `rspice-bench generated-rust`

This deterministic gate reads the v3 generator manifest, authenticates every
generated file, and reports total source bytes/lines, noise-source bytes,
category totals, and the largest files and models. Optional limits turn each
metric into a non-zero-exit release budget.

| Flag | Default | Meaning |
| :--- | :--- | :--- |
| `--generated-root <DIR>` | `crates/rspice-veriloga-models` | Generated bundle to authenticate and measure |
| `--max-source-bytes <N>` | unset | Maximum total generated-source bytes |
| `--max-noise-source-bytes <N>` | unset | Maximum source bytes attributable to noise kernels |
| `--max-model-source-bytes <N>` | unset | Maximum source bytes for any one generated model |
| `--max-file-count <N>` | unset | Maximum generated-file count |
| `--max-pooled-workspace-payload-bytes <N>` | unset | Maximum pooled scratch-workspace payload for any model on one worker thread |
| `--max-stamp-state-payload-bytes <N>` | unset | Maximum persistent DDT/IDT stamp-state payload for any model instance |
| `--top <N>` | 20 | Largest files and models retained in output |
| `--out <PATH>` | unset | Optional deterministic JSON report |

### `rspice-bench generated-compile`

This gate prepares dependencies in an isolated target directory, then forces
and times repeat release checks of representative generated leaf packages.
Cargo JSON messages are audited so dependency rebuilds cannot silently pollute
the samples. The JSON report authenticates the source bundle and captures the
Rust toolchain, target, host, and Git provenance.

### `rspice-bench generated-stamp`

This in-process gate measures generated model evaluation and matrix/RHS
stamping in one call, then measures the hand-written BSIM4 implementation in
the same process. Same-run ratios are the portable release signal; absolute
nanosecond limits are available for controlled hardware only. Use
`--features generated-stamp-subset` for the five-model routine tier or
`--features generated-stamp` for the complete 42-model corpus.

| Flag | Default | Meaning |
| :--- | :--- | :--- |
| `--iterations <N>` | 2000 | Stamp calls per timed sample |
| `--samples <N>` | 7 | Independent timed samples |
| `--model <NAME>` | all compiled-in models | Repeatable model filter |
| `--max-median-ns-per-stamp <NS>` | unset | Controlled-host absolute median limit for every model |
| `--max-corpus-median-reference-ratio <X>` | unset | Maximum corpus median divided by the same-run hand-written reference |
| `--max-model-reference-ratio <X>` | unset | Maximum per-model median divided by the same-run hand-written reference |
| `--out <PATH>` | unset | JSON evidence including every ratio, configured budget, failure, and verdict |

### `rspice-bench run`

| Flag | Default | Meaning |
| :--- | :--- | :--- |
| `--repeats <N>` | 5 (min 1) | Timed repetitions per deck/simulator pair; one untimed warmup always runs first |
| `--out <PATH>` | `benchmarks/scoreboards/scoreboard.json` | Scoreboard destination. The rig never date-stamps; archive a run by passing an explicit dated path |
| `--circuits <DIR>` | `benchmarks/circuits` | Deck directory (non-recursive scan for `*.cir`) |
| `--timeout-secs <N>` | 120 (min 1) | Per-run wall-clock cap; an exceeded run is killed and marked failed |
| `--baseline <PATH>` | unset | Same-host scoreboard used to gate every RSpice deck median |
| `--max-regression-percent <PERCENT>` | 10 | Largest allowed per-deck median slowdown versus `--baseline` |
| `--allow-host-mismatch` | false | Permit exploratory cross-host comparisons; unsafe for release gates |

Environment variables:

| Variable | Meaning |
| :--- | :--- |
| `RSPICE_BENCH_RSPICE` | RSpice executable override. Default: the `rspice` binary next to `rspice-bench` in `target/release/` |
| `RSPICE_BENCH_NGSPICE` | ngspice executable for the comparison column. Unset = the ngspice column is skipped (noted in the scoreboard, not a failure). Use a release/console build — a debug ngspice makes RSpice look artificially fast |

The process exits non-zero if any simulator run or baseline comparison fails.
Baseline comparison requires the same repeat count, exact deck set, and
methodology and, by default, the same OS/architecture and logical CPU count.
The scoreboard is still written on a regression and carries a machine-readable
`regression_gate` report with every median, delta, and verdict.

### `rspice-bench klu`

This isolates the solver kernels so an optimization is attributable to a
phase. It times `analyze`, `factor`, `refactor` and `solve` separately on
circuit-shaped matrices — a banded RC-ladder pattern and a denser ring-like
pattern with off-diagonal couplings, both diagonally dominant, with a
Newton-style value drift between refactors — and reports fill as
`(L+U) nnz / A nnz`. Each case is reseeded from its own identity rather than
from sweep position, so changing `--sizes` cannot silently perturb another
case's matrix. Drift is applied outside the timed region: it is fixture work,
not solver work.

A random expander is measured as the pathological reference row and is never
gated. Real circuit matrices are local; the expander is worst-case fill under
*any* ordering, and at n=1000 it runs ~25x worse per nonzero than the circuit
patterns.

Budgets are normalized per `(L+U)` nonzero rather than absolute per iteration,
because a sparse direct solve is proportional to factor nonzeros. That holds in
practice — measured `refactor` cost is ~2.14, 2.13, 2.16 ns/nnz for the ladder
at n=100, 1000, 10000 — so one threshold covers the whole sweep.

| Flag | Default | Meaning |
| :--- | :--- | :--- |
| `--sizes <N,...>` | 100,1000,10000 | Dimensions swept for each circuit-shaped pattern |
| `--refactors <N>` | 400 | Refactor and solve iterations per timed sample |
| `--samples <N>` | 7 | Timed samples per case; the report gates on the median |
| `--expander-size <N>` | 1000 | Dimension of the ungated reference case |
| `--expander-refactors <N>` | 50 | Iterations per sample for the reference case |
| `--max-refactor-ns-per-lu-nnz <NS>` | unset | Refactor budget per factor nonzero |
| `--max-solve-ns-per-lu-nnz <NS>` | unset | Solve budget per factor nonzero |
| `--max-fill-ratio <RATIO>` | unset | Fill budget as `(L+U) nnz / A nnz` |
| `--out <PATH>` | unset | Optional JSON report path |

Every budget is off unless passed explicitly, so an unqualified run measures
and reports without ever failing. This is a deliberate difference from
`native-jit`, whose defaults enforce: KLU budgets cannot be chosen before
baselines exist, and the solver is still moving. Use release builds — debug
numbers are a smoke test of the machinery only.

### `rspice-bench native-jit`

This is an in-process native Verilog-A JIT benchmark gate, not a full-circuit
macro benchmark. It compiles a dense synthetic Verilog-A model, runs generated
native entrypoint sweeps plus public `VerilogADevice::try_evaluate` sweeps
against bytecode VM reference paths, and gates each case on median native
speedup. It also records p95 so a single lucky fastest sample cannot hide a
regression.

| Flag | Default | Meaning |
| :--- | :--- | :--- |
| `--iterations <N>` | 200000 | Entry-point sweeps per timed sample |
| `--samples <N>` | 7 | Timed samples; report computes min/median/p95/mean |
| `--min-speedup <X>` | 3.00 | Required `bytecode_median / native_median`; below this exits non-zero |
| `--max-native-p95-ns-per-sweep <NS>` | unset | Optional absolute native p95 budget |
| `--out <PATH>` | unset | Optional JSON report path |

Use release builds for comparable numbers. Debug runs are useful only as a
functional smoke test of the benchmark machinery.

### `rspice-bench gen`

| Flag | Default | Meaning |
| :--- | :--- | :--- |
| `--dir <DIR>` | `benchmarks/circuits` | Where to write the generated decks |

The generated decks (the three RC ladders and the 4,096-stage MOS array)
are checked in; never hand-edit them — change
`src/generate.rs` and regenerate.

## The deck set

Seven decks, spanning startup overhead to 10k-node matrices (the
hand-written ones use only the SPICE dialect subset both simulators
share):

| Deck | What it measures |
| :--- | :--- |
| `divider_ac.cir` | Startup floor: trivial DC + AC sweep — process overhead, parse, output |
| `diode_rectifier.cir` | Nonlinear transient: Newton iterations with diode limiting |
| `ring51.cir` | 51-stage MOSFET ring oscillator: device-evaluation-dominated transient |
| `mos_array_4096.cir` | Device-evaluation tier: 4,096 level-1 NMOS stages — Newton time in model code, not factorization |
| `rc_ladder_100.cir` | Small linear transient (100 nodes, ~10k steps) |
| `rc_ladder_1000.cir` | Medium linear transient (1k nodes, ~10k steps): stamp + solve balance |
| `rc_ladder_10000.cir` | Scale tier (10k nodes, ~1k steps): factorization/solve dominated |

## Methodology and scoreboards

[`benchmarks/README.md`](../../benchmarks/README.md) is the canonical
methodology document — comparability rules (same machine, release builds,
nothing heavy running concurrently), the warmup convention, and the
scoreboard archive layout (`scoreboards/scoreboard.json` for the latest
run, `YYYY-MM-DD-<tag>.json` for archived baselines). The headline number
is the **median**; min and mean are recorded alongside, and each
scoreboard embeds the methodology string, host info, repeat count, and
both executable paths so archived numbers stay self-describing. The
speedup column is `ngspice_median / rspice_median` — greater than 1.0
means RSpice is faster. Benchmark runs are invoked manually; CI does not
run timing comparisons.

## License

RSpice Bench is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).
