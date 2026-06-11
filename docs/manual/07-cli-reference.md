# 7 · CLI reference

```
rspice [--verbose] [--quiet] [--config FILE] [--log-level LEVEL] <command>
```

| Command | Purpose |
|---|---|
| `run NETLIST` | Simulate a deck (all analysis cards, multi-run plans). |
| `check NETLIST` | Parse + structural validation, no simulation. |
| `info NETLIST` | Deck summary: elements, nodes, models, analyses. |
| `compile-va FILE.va` | Compile a Verilog-A model. |
| `convert` | Translate waveform files between formats. |
| `compare` | Diff results against a golden file with tolerances. |
| `completions SHELL` | Shell-completion script on stdout. |

A user/project config file (`--config`, or the default location printed
by `--help`) supplies defaults for output format, search paths, and
simulation options; CLI flags override it.

## `rspice run`

**Input/output**

| Flag | Meaning |
|---|---|
| `-o, --output FILE` | Waveform output file (multi-analysis and multi-run outputs are tagged). |
| `-f, --format FMT` | `raw`, `csv`, `json`, … (`--help` lists all). |
| `-I, --include DIR` | Extra `.include`/`.lib` search directories (repeatable). |
| `-D NAME=VALUE` | Parameter override, wins over deck `.param` (repeatable; SPICE suffixes accepted). |
| `--meas` | Print `.meas` results. |
| `--meas-file FILE` / `--meas-format json\|csv` | Measurement report for CI. |
| `--report-file FILE` / `--report-format junit\|tap` | Run report for CI. |
| `--progress` | Spinner with elapsed time. |

**Simulation control**

| Flag | Meaning |
|---|---|
| `--temp C` | Simulation temperature. |
| `--maxiter N`, `--abstol`, `--reltol`, `--residual-reltol` | Newton tolerances. |
| `--min-step`, `--max-step` | Transient step bounds. |
| `--convergence PRESET` | Convergence preset selection. |
| `--compress [--compress-tol TOL]` | Waveform compression for long transients. |
| `--checkpoint FILE` | Save end-of-run transient state. |
| `--restore FILE` | Resume a transient from a checkpoint (conflicts with `--compress`). |

**Analysis triggers (beyond deck cards)**

| Flag | Meaning |
|---|---|
| `--monte-carlo N [--seed S]` | Monte Carlo over the deck's analyses. |
| `--pss-freq F [--pss-harmonics N] [--pss-tstab T]` | Periodic steady state (shooting). |
| `--hb-freq F [--hb-harmonics N]` | Harmonic balance. |
| `--pz-input N --pz-output M` | Pole-zero between nodes. |
| `--sens-output N [--sens-param NAME]` | Sensitivity analysis. |

## Exit status

`0` on success. Non-zero when parsing fails, any analysis fails, or any
run of a multi-run plan fails (the remaining runs still execute and the
reports cover all of them).

## Environment

| Variable | Effect |
|---|---|
| `RSPICE_SOLVER=klu` | Experimental KLU-class linear-solver backend (see [chapter 8](08-performance.md)). |
| `RUST_LOG=debug` | Engine diagnostics (timestep control, convergence, rescue ladders). |
