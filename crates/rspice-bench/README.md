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
timing is `std::time::Instant` around child-process spawn/wait. For
isolated solver-kernel numbers, use rspice-core's
`cargo run --release -p rspice-core --example klu_bench` instead.

## Layout

| File | Contents |
| :--- | :--- |
| `src/main.rs` | CLI entry point: `gen` and `run` subcommands |
| `src/runner.rs` | The `run` subcommand: locates executables, runs warmup + timed repeats per deck/simulator, computes min/median/mean and the median speedup, writes the scoreboard, prints the table |
| `src/generate.rs` | The `gen` subcommand: deterministically regenerates the generated decks (RC ladders, MOS array) with byte-stable output — fixed formatting, no timestamps |
| `src/error.rs` | `BenchError` with full context on every failure path |

No feature flags; dependencies are just `clap`, `serde`, and `serde_json`.

## Building and running

```bash
# Build the simulator under test and the rig together
cargo build --release -p rspice-cli -p rspice-bench

# Run the suite (writes benchmarks/scoreboards/scoreboard.json)
target/release/rspice-bench run

# Regenerate the generated decks (verify with git diff afterwards)
target/release/rspice-bench gen
```

### `rspice-bench run`

| Flag | Default | Meaning |
| :--- | :--- | :--- |
| `--repeats <N>` | 5 (min 1) | Timed repetitions per deck/simulator pair; one untimed warmup always runs first |
| `--out <PATH>` | `benchmarks/scoreboards/scoreboard.json` | Scoreboard destination. The rig never date-stamps; archive a run by passing an explicit dated path |
| `--circuits <DIR>` | `benchmarks/circuits` | Deck directory (non-recursive scan for `*.cir`) |
| `--timeout-secs <N>` | 120 (min 1) | Per-run wall-clock cap; an exceeded run is killed and marked failed |

Environment variables:

| Variable | Meaning |
| :--- | :--- |
| `RSPICE_BENCH_RSPICE` | RSpice executable override. Default: the `rspice` binary next to `rspice-bench` in `target/release/` |
| `RSPICE_BENCH_NGSPICE` | ngspice executable for the comparison column. Unset = the ngspice column is skipped (noted in the scoreboard, not a failure). Use a release/console build — a debug ngspice makes RSpice look artificially fast |

The process exits non-zero if any simulator run failed, so CI can gate on
deck health even when timings are not being compared.

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
