# rspice-bench

Developer performance tools and reproducible workloads for RSpice. This binary
is a Cargo workspace member under `tools/`; shipping crates do not depend on it.

## Layout

- `src/`: process timing, solver/JIT benchmarks, generated-code measurements,
  report serialization, and provenance capture.
- `circuits/`: the single checked-in copy of the seven macro workloads.
- `external/`: ngspice/Xyce comparison inputs and reproduction scripts.

Measurement reports and same-host timing baselines belong in ignored
`target/benchmarks/` or CI artifact storage. Do not commit machine timings,
scoreboards, or dated run archives. Historical reports remain in Git history.

The host-independent correctness baseline belongs to
[`rspice-core`'s qualification tests](../../crates/rspice-core/tests/testdata/qualification/README.md).
It records capabilities and operation counts, not timing measurements.

## Build and run

Run these commands from the repository root:

```sh
cargo build --release -p rspice-cli -p rspice-bench
target/release/rspice-bench run
```

The default run scans `tools/rspice-bench/circuits/*.cir` and writes
`target/benchmarks/scoreboard.json`. It always times RSpice; set
`RSPICE_BENCH_NGSPICE` to a release/console ngspice executable to include the
comparison column. `RSPICE_BENCH_RSPICE` overrides the RSpice executable.

Create a local baseline and compare a candidate on the same machine:

```sh
target/release/rspice-bench run --out target/benchmarks/reference.json
target/release/rspice-bench run \
  --baseline target/benchmarks/reference.json \
  --out target/benchmarks/candidate.json \
  --max-regression-percent 10
```

The gate requires matching methodology, repeat count, and deck names, plus the
same OS/architecture and logical CPU count by default. That host check does
not identify the exact CPU or power configuration; control those yourself.
Use unchanged deck contents, matching release profiles, and an idle machine.
`--allow-host-mismatch` is for exploratory comparisons only.

Each deck gets one untimed warmup followed by five timed runs by default.
Wall-clock time includes process startup, parsing, solving, and output
formatting; child output is discarded. Cold filesystem caches are not
controlled. Reports contain min/median/mean; comparisons use the median.
A failed simulator run or baseline comparison produces a nonzero exit code.

Use `rspice-bench <command> --help` for all flags and defaults.

## Other commands

| Command | Purpose |
| --- | --- |
| `gen` | Regenerate RC-ladder and MOS-array workloads deterministically |
| `klu` | Measure solver analyze/factor/refactor/solve, check numerical accuracy, and apply optional fill or cost budgets |
| `native-jit` | Compare native Verilog-A execution against bytecode, checking numerical results and relative speedups |
| `generated-rust` | Authenticate generated Rust files and report source sizes and workspace/state payload counts |
| `generated-compile` | Measure isolated generated-package release checks with toolchain, host, and Git provenance |
| `generated-stamp` | Measure generated-model evaluation/stamping; requires an opt-in model feature |
| `gp-transient` | Measure public native GP exact-delay/Weil runs, accepted-work cancellation and charged peak transport storage; requires `core-transient` |

Examples:

```sh
cargo run --locked --release -p rspice-bench -- klu --out target/benchmarks/klu.json
cargo run --locked --release -p rspice-bench -- native-jit --out target/benchmarks/native-jit.json
cargo run --locked -p rspice-bench -- generated-rust --out target/benchmarks/generated-rust.json
cargo run --locked --release -p rspice-bench -- generated-compile --out target/benchmarks/generated-compile.json
cargo run --locked --release -p rspice-bench --features generated-stamp-subset -- \
  generated-stamp --out target/benchmarks/generated-stamp.json
```

`generated-compile` requires a clean release build for trusted measurements;
use `--exploratory` for development measurements. `generated-stamp-subset`
selects five models; `generated-stamp` selects the complete corpus.

CI runs KLU numerical/fill/relative-cost checks, generated-source resource
checks, native-JIT comparisons, and generated-model measurements. Nightly CI
measures the complete generated-model corpus. Workflows own their thresholds
and upload retained reports as artifacts. Absolute timing budgets are optional
and should be chosen on controlled hardware. Different compact models do not
have a universal runtime ratio.

## Native GP transient qualification

Build from a clean committed checkout and retain the exact Cargo invocation
alongside the immutable report. The command refuses a debug or dirty build
unless `--exploratory` is supplied. The common report envelope captures the
executable and lockfile hashes, current source revision, toolchain and host.

```sh
cargo run --locked --release -p rspice-bench --features core-transient-default -- \
  gp-transient --out target/benchmarks/gp-default.json
cargo run --locked --release -p rspice-bench --features core-transient -- \
  gp-transient --out target/benchmarks/gp-no-default.json
```

`core-transient-default` requests the core's default features; `core-transient`
does not. Cargo dependency features can unify, so retain `cargo tree -e features`
with comparisons. Neither enables the generated-model corpus. Additional core
features may be selected explicitly through Cargo.

Each case runs an NPN circuit with private RB/RE/RC nodes in the Ngspice dialect
with a 4 ps maximum step and adaptive refinement. It exercises both phase operators, 1 and 16 devices, and
either no checkpoints or three retained checkpoints. The full generated deck,
its hash, horizon/step policy, checkpoint times and configuration are recorded. `--devices`,
`--steps` and `--samples` control the workload. One warmup precedes the repeats.
The report retains every timing, waveform hash, sample count and abort-poll count.
Repeated waveforms and post-cancellation engine reuse must match exactly.

Cancellation is requested from the first accepted-sample callback at or beyond
half the simulation horizon, after at least one transport delay. The measured latency ends when the public
API returns, including cleanup. This avoids a trigger thread's scheduler delay;
it does not measure cancellation requested in the middle of a matrix solve or
from another process. No further sample may be accepted on these nonlinear
workloads. Portable cancellation/restart regressions live in the core tests.

Peak storage is measured through the typed transport-byte quota: follow strictly
increasing resource requests until the public run succeeds, then verify failure
at one byte less. Repeat this probe before and after timing; run at the exact
quota and require the same waveform. The figure includes charged live-record
capacities, transient copies and retained checkpoints. It excludes fixed BJT
state, results, solver allocations, allocator overhead and process RSS. Weil's
finite state therefore correctly reports zero **transport-record** bytes.

Timing includes circuit building, OP, solving, observation and checkpoint
capture/release. It excludes netlist parsing, waveform hashing/validation and
waveform release. Quota probes are not timed. Optional `--max-run-ms` (per-case
median), `--max-cancel-ms` (every measured cancellation) and
`--max-transport-bytes` fail the run after saving its report. Choose timing gates
on controlled hardware after inspecting repeatability; this harness does not
establish a universal performance limit. Its workloads complement the core's
independent accuracy oracles and do not close the complete GP qualification matrix.

## Macro workloads

| Deck | Coverage |
| --- | --- |
| `divider_ac.cir` | Process startup, parsing, and AC sweep overhead |
| `diode_rectifier.cir` | Nonlinear transient and diode limiting |
| `ring51.cir` | MOS evaluation in a ring oscillator |
| `mos_array_4096.cir` | Large MOS evaluation/stamp workload |
| `rc_ladder_100.cir` | Small sparse linear transient |
| `rc_ladder_1000.cir` | Medium sparse stamp/solve workload |
| `rc_ladder_10000.cir` | Large sparse factor/solve workload |

The three RC ladders and MOS array are generated by `src/generate.rs`.
Change the generator and run `rspice-bench gen`; review the resulting diff.
The other decks are hand-maintained in the common RSpice/ngspice dialect.
The core's `macro_benchmark_contract` tests exercise the 1k/10k ladders and
selected-current behavior independently of timing runs.

Licensed under the [RSpice Personal Use License](../../LICENSE).
