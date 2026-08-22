# RSpice macro-benchmarks

The deck set and operating conventions for `rspice-bench`, the WS0/M0.5
macro-benchmark rig. The rig times **whole simulator processes** — RSpice
(`rspice run <deck> -q`) against a locally installed ngspice
(`ngspice -b <deck>`) — so parsing, solving, and output formatting are all
included. It is the regression yardstick for WS3 performance work: no
optimization claim lands without a before/after scoreboard from this rig.

## Running

```
cargo build --release -p rspice-cli -p rspice-bench
target/release/rspice-bench run
```

Create and enforce a same-machine baseline:

```text
target/release/rspice-bench run --out benchmarks/results/reference.json
target/release/rspice-bench run \
  --baseline benchmarks/results/reference.json \
  --max-regression-percent 10 \
  --out benchmarks/results/candidate.json
```

- `RSPICE_BENCH_NGSPICE=<path>` — ngspice executable for the comparison
  column. Unset = the ngspice column is skipped (noted in the scoreboard).
  Use a **release/console** build (`ngspice_con.exe` on Windows); a debug
  ngspice makes RSpice look artificially fast.
- `RSPICE_BENCH_RSPICE=<path>` — RSpice executable override. Default: the
  `rspice` binary next to `rspice-bench` in `target/release/`.
- `--repeats N` — timed repetitions per deck/simulator (default 5; one
  untimed warmup always runs first).
- `--out PATH` — scoreboard destination (default
  `benchmarks/scoreboards/scoreboard.json`, the latest run on this machine).
  Point it at `benchmarks/results/` for anything you intend to keep. Do not
  date-stamp a file into `scoreboards/` or add one to `archive/`; see
  **Where results live** below.
- `--baseline PATH` — compare every RSpice median against an existing
  same-host scoreboard and fail if any deck exceeds the allowed regression.
- `--max-regression-percent PERCENT` — per-deck median budget used with
  `--baseline` (default 10). The repeat count, deck sets, and timing
  methodology must match.
- `--allow-host-mismatch` — opt out of the OS/architecture and logical-CPU
  identity check for exploratory comparisons; never use this for release gates.

The process exits non-zero if any simulator run failed or any baseline gate
failed. The current scoreboard is still written and includes the complete
`regression_gate` report so CI can retain evidence for a failed run.

## Verilog-A native JIT gate

The native x64 backend has a separate in-process release gate:

```text
cargo run --locked -p rspice-bench --release -- native-jit \
  --iterations 100000 \
  --samples 9 \
  --min-dense-speedup 1.75 \
  --min-speedup 3.00 \
  --min-full-stamp-speedup 2.00 \
  --max-native-setup-ms 10 \
  --max-native-p95-ns-per-sweep 5000 \
  --max-relative-stddev 0.25 \
  --max-native-code-bytes 16384
```

It compares dense entrypoint, public device-evaluation, and full solver-stamp
sweeps against bytecode references. The gate checks numerical checksums,
median speedups, p95 latency, timing stability, canonical-to-native setup
time, generated image size, and the expected fused-driver plan shape. Linux
and Windows CI run the same command. The absolute ceilings are regression
ratchets; they are not performance targets or representative compact-model
latencies.

## Methodology

Wall-clock of the full child process, `std::time::Instant` around spawn and
an OS-backed timed child wait (no polling interval), with stdin/stdout/stderr
attached to the null device. One untimed warmup precedes the timed repeats for
each deck/simulator pair. Cold OS file-cache effects are not controlled.
**Median** is the headline number; min and mean are recorded alongside. The
methodology string is embedded in every scoreboard so archived numbers stay
self-describing and older polling-based scoreboards cannot silently become
gate inputs.

Comparability rules:

- Same machine, same power profile, nothing heavy running concurrently
  (notably: not while another agent session is compiling).
- RSpice from the workspace's measured `--release` profile; ngspice from a
  release build. Do not mix Cargo profiles across baseline/current runs.
- Quote medians, and quote the scoreboard file, not memory.

## Deck set

| Deck | What it measures |
|---|---|
| `divider_ac.cir` | Startup floor: trivial DC + AC sweep — process overhead, parse, output. |
| `diode_rectifier.cir` | Nonlinear transient: Newton iterations with diode limiting. |
| `ring51.cir` | 51-stage MOSFET ring oscillator: device-evaluation-dominated transient. |
| `mos_array_4096.cir` | Device-evaluation tier: 4,096-stage level-1 NMOS array transient — Newton time spent in model code, not factorization. |
| `rc_ladder_100.cir` | Small linear transient (100 nodes, ~10k steps). |
| `rc_ladder_1000.cir` | Medium linear transient (1k nodes, ~10k steps): stamp + solve balance. |
| `rc_ladder_10000.cir` | Scale tier (10k nodes, ~1k steps): factorization/solve dominated. |

The RC ladders and the MOS array are generated — `rspice-bench gen`
reproduces them byte-for-byte (fixed formatting, no timestamps). Edit
`crates/rspice-bench/src/generate.rs` and regenerate; never hand-edit a
generated deck. The other decks are hand-written and use only the dialect
subset both simulators share.

Planned additions (roadmap M0.5): ring oscillators in additional
technologies, a sky130 op-amp tran/AC/noise trio, a buck converter, a
100k-node RC tier and clock tree, and an MC×500 OTA statistical workload.

## Where results live

Five directories, each with one job. Nothing moves between them by copying a
file; each promotion is a decision.

| Directory | Authority |
| :--- | :--- |
| `suites/` | The workloads themselves. A suite manifest authenticates each deck's bytes and binds it to a correctness contract; directory discovery is deliberately unsupported. Changing a deck requires a new `suite_version`. |
| `schemas/` | JSON contracts for suite projections and result envelopes, so a dashboard can reject an incomplete report without linking the benchmark crate. Schema versions are compatibility boundaries. |
| `baselines/` | Approved gates. A baseline must use the current result schema, carry raw samples and full tool provenance, pass its correctness preflight, come from a clean release build, and name an immutable suite. Promotion is an engineering approval, not a filesystem move. |
| `results/` | Ordinary run output. Write runs here and keep them local or in CI artifact storage; nothing in this directory is committed. |
| `archive/` | Frozen historical evidence from before versioned suites and provenance were required. Multiple incompatible schemas, some failed runs. Not a baseline, not quotable for commercial claims, and closed to new output. |

`scoreboards/scoreboard.json` sits outside that structure on purpose: it is the
rig's default `--out` destination — whatever ran last on this machine — not a
published record. Do not date-stamp copies of it into the repository.

A scoreboard records host info, repeat count, the methodology string, both
executable paths, and per-deck min/median/mean plus the median speedup
(`ngspice / rspice`; >1.0 means RSpice is faster). Baseline-gated runs also
record baseline/current medians, percent changes, the threshold, host match,
and per-deck plus aggregate verdicts.

`external/` holds third-party simulator studies. Each owns its decks, method,
provenance limits, and reproduction steps, and an existing study result is
never overwritten in place.
