# rspice-bench

Developer performance tools and reproducible workloads for RSpice. This binary
is a Cargo workspace member under `tools/`; shipping crates do not depend on it.

## Layout

- `src/`: process timing, solver/JIT benchmarks, generated-code measurements,
  report serialization, and provenance capture.
- `circuits/`: the hand-written macro workloads.

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

The default run generates the parametric decks into `target/benchmarks/decks/`,
times them with the hand-written `tools/rspice-bench/circuits/*.cir`, and writes
`target/benchmarks/scoreboard.json`. `--circuits <DIR>` times only that directory. It always times RSpice; set
`RSPICE_BENCH_NGSPICE` to a release/console ngspice executable to include the
comparison column. `RSPICE_BENCH_RSPICE` overrides the RSpice executable.
ngspice runs as `ngspice -b -r <raw> <deck>`: its batch mode runs no analysis
for a deck without an output request, so its column includes writing a
rawfile, which `rspice run -q` does not.

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

## Named populations

`run --suite <name>` and `paired --suite <name>` time a fixed population
instead of the default deck set. Each run writes the suite's generated decks
fresh into `target/benchmarks/decks/<suite>/`; corpus decks run in place under
`tests/`, because they include model files by relative path.

| Suite | Workloads |
| --- | --- |
| `startup` | Spawn floor, `--version`, and `check`, `run -q` and `run -q -o x.raw` on a five-line deck |
| `flat` | `check` and `run -q` on a flat 40,002-element diode ladder |
| `neospice` | 34 generated shapes: dividers (op, dc sweep, noise), RC/RLC/PULSE transients, resistor meshes, diode ladders, RC and diode-RC grids (ac, tran, noise), and op+ac op-amp cases |
| `gf180` | One GF180MCU BSIM4 I-V deck |
| `iscas` | ISCAS85 c499 and c880 gate-level BSIM4 transients (minute-scale; c432, the smallest, writes plot files beside itself) |
| `mixed` | Three mixed-signal corpus designs (op and ac) |
| `all` | Every suite above |

The op-amp cases use the transistor-level `opampal` from
`tests/xyce/Netlists/MCNC_BJT_OPAMPAL/`, read at generation time, in place of
vendor macromodels that cannot be shipped. PSP and BSIM-CMG ring oscillators
are not in the population: they need the generated Verilog-A model catalog,
which the default build refuses.

The spawn floor is `rspice-bench --spawn-floor`, which exits before doing
anything: a minimal Rust process, the same on every platform. Every report
records it, and `run` prints each RSpice median above it.

`--filter <text>` keeps the workloads whose id contains the text; a filter
that keeps nothing is an error. Minute-scale workloads take no warm-up and run
`--heavy-repeats` times in `run` (default 1). `--label <text>` records the
machine state. On Linux and macOS every workload also records the load
averages it started under, and the table prints the one-minute figure;
Windows has no load average, so there the label is the only record.
`RSPICE_BENCH_ESPICE` adds a column for an ESpice executable run as
`<exe> <deck>` on simulating workloads.

## Paired A/B timing

`paired` is the way to measure a change: it times two RSpice executables
against each other, interleaved, over one population.

```sh
target/release/rspice-bench paired \
  --a target/benchmarks/rspice-main --b target/release/rspice \
  --suite neospice --label "idle box"
```

For each workload, A and B first run once writing a rawfile each. The rawfiles
must be byte-identical (or, with `--tolerance <rel>`, agree within that
relative difference per value); a workload whose outputs differ is
DISQUALIFIED, reported, and excluded from every summary. Those runs also warm
the file cache. `--warmups` untimed pairs (default 1) and `--pairs` timed pairs
(default 10) follow, in alternating order (AB, BA, ...); a minute-scale
workload takes no warm-up pairs and `--heavy-pairs` timed pairs (default 3).
Each pair gives a B/A ratio; the table shows each workload's median ratio with
its min and max and, where the platform reports one, the one-minute load
average, and ends with the geometric mean over the workloads that counted. A
run that fails or times out (`--timeout-secs`) marks its workload FAILED, also
excluded; the table lists every excluded workload with the reason. The JSON
report goes to `target/benchmarks/paired.json` (`--out`); any disqualified or
failed workload makes the exit code nonzero.

Run `paired` with the same executable as A and B first: the spread of that
A/A run is the harness noise on the machine at that moment, the floor under
any B/A difference worth reporting.

## Other commands

| Command | Purpose |
| --- | --- |
| `klu` | Measure solver analyze/factor/refactor/solve, check numerical accuracy, and apply optional fill or cost budgets |
| `native-jit` | Compare native Verilog-A execution against bytecode, checking numerical results and relative speedups |
| `generated-rust` | Authenticate generated Rust files and report source sizes and workspace/state payload counts |
| `generated-stamp` | Measure generated-model evaluation/stamping; requires an opt-in model feature |

Examples:

```sh
cargo run --locked --release -p rspice-bench -- klu --out target/benchmarks/klu.json
cargo run --locked --release -p rspice-bench -- native-jit --out target/benchmarks/native-jit.json
cargo run --locked -p rspice-bench -- generated-rust --out target/benchmarks/generated-rust.json
cargo run --locked --release -p rspice-bench --features generated-stamp-subset -- \
  generated-stamp --out target/benchmarks/generated-stamp.json
```

`generated-stamp-subset` selects five models; `generated-stamp` selects the
complete corpus.

CI runs KLU numerical/fill/relative-cost checks, generated-source resource
checks, native-JIT comparisons, and generated-model measurements. Nightly CI
measures the complete generated-model corpus. Workflows own their thresholds
and upload retained reports as artifacts. Absolute timing budgets are optional
and should be chosen on controlled hardware. Different compact models do not
have a universal runtime ratio.

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

The three RC ladders and MOS array are generated by `src/generate.rs` on every
default run. The other decks are hand-maintained in the common RSpice/ngspice
dialect. The core's `macro_benchmark_contract` tests build the same 1k/10k
ladders and check them and selected-current behavior independently of timing
runs.

Licensed under the [RSpice Personal Use License](../../LICENSE).
