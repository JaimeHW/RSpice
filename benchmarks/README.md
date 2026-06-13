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

- `RSPICE_BENCH_NGSPICE=<path>` — ngspice executable for the comparison
  column. Unset = the ngspice column is skipped (noted in the scoreboard).
  Use a **release/console** build (`ngspice_con.exe` on Windows); a debug
  ngspice makes RSpice look artificially fast.
- `RSPICE_BENCH_RSPICE=<path>` — RSpice executable override. Default: the
  `rspice` binary next to `rspice-bench` in `target/release/`.
- `--repeats N` — timed repetitions per deck/simulator (default 5; one
  untimed warmup always runs first).
- `--out PATH` — scoreboard destination (default
  `benchmarks/scoreboards/scoreboard.json`). The rig never date-stamps the
  file itself; archive a run by passing an explicit dated path, e.g.
  `--out benchmarks/scoreboards/2026-06-11-baseline.json`.

The process exits non-zero if any simulator run failed, so the rig can gate
deck health in CI even when timings are not being compared.

## Methodology

Wall-clock of the full child process, `std::time::Instant` around
spawn/wait, stdin/stdout/stderr attached to the null device. One untimed
warmup precedes the timed repeats for each deck/simulator pair. Cold OS
file-cache effects are not controlled. **Median** is the headline number;
min and mean are recorded alongside. The methodology string is embedded in
every scoreboard so archived numbers stay self-describing.

Comparability rules:

- Same machine, same power profile, nothing heavy running concurrently
  (notably: not while another agent session is compiling).
- RSpice from `--release` (fat LTO); ngspice from a release build.
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

## Scoreboards

`scoreboards/` (this directory) holds the published scoreboards:

- `scoreboard.json` — the latest run on the reference machine.
- `YYYY-MM-DD-<tag>.json` — archived baselines (pre/post optimization).

A scoreboard records host info, repeat count, the methodology string, both
executable paths, and per-deck min/median/mean plus the median speedup
(`ngspice / rspice`; >1.0 means RSpice is faster).
