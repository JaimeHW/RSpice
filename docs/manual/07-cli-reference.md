# 7 · CLI reference

```
rspice [GLOBAL] <command> [COMMAND OPTIONS]
```

Global options are available on every subcommand:

| Flag | Meaning |
|---|---|
| `-v, --verbose` | Enable debug-level output. |
| `-q, --quiet` | Suppress non-error output. |
| `--config FILE` | Load a specific config file. |
| `--log-level LEVEL` | `off`, `error`, `warn`, `info`, `debug`, or `trace`. |

`rspice --version` reports the crate version plus build target/profile.
Numeric CLI values accept SPICE suffixes (`4.7k`, `2.4G`, `1u`, `100meg`).

## Commands

| Command | Purpose |
|---|---|
| `run NETLIST` | Simulate a deck. Executes all deck analysis cards unless an analysis-mode flag is supplied. |
| `check NETLIST` | Parse and structurally validate a deck, without simulation. |
| `info NETLIST` | Print element, node, model, hierarchy, and parameter summaries. |
| `compile-va FILE.va` | Compile a Verilog-A model and optionally emit an interface summary. |
| `convert INPUT OUTPUT --to FORMAT` | Convert waveform/result files. |
| `compare RESULT GOLDEN` | Diff results against a golden file with tolerances. |
| `completions SHELL` | Emit shell completions for `bash`, `zsh`, `fish`, `powershell`, or `elvish`. |

`run`, `check`, and `info` accept `-` for stdin; includes then resolve against
the working directory.

## `rspice run`

**Input and output**

| Flag | Meaning |
|---|---|
| `-o, --output FILE` | Waveform/result output. Multi-analysis and multi-run outputs are tagged (`out.csv` -> `out.tran.csv`, `out.tt.csv`, etc.). |
| `-f, --format FORMAT` | `raw`, `ascii`, `csv`, `json`, `tsv`, or `hdf5` (default: config `output.format`, else `raw`). |
| `--save SIGNAL` | Override deck `.save`/`.probe`/`.print` selection; repeatable (`V(out)`, `V(a,b)`, `I(v1)`, `@m1[id]`, `all`). |
| `-I, --include DIR` | Extra `.include`/`.lib` search directory; repeatable. |
| `-D, --define NAME=VALUE` | Override or define a deck parameter; repeatable. |
| `--meas` | Print `.meas` results. |
| `--meas-file FILE` | Write machine-readable `.meas` output. |
| `--meas-format FORMAT` | `json` or `csv` (defaults to JSON when `--meas-file` is used). |
| `--report-format FORMAT` | CI report format: `junit` or `tap`. |
| `--report-file FILE` | CI report output; requires `--report-format`. |
| `--summary FILE` | JSON run summary with tool version, per-run status, measurements, result files, and verdict; use `-` for stdout. |
| `--progress` | Show live progress for transient runs. |

**Run discipline**

| Flag | Meaning |
|---|---|
| `--timeout SECONDS` | Abort after the budget; exits `124`. Ctrl-C exits `130`. |
| `--allow-failed-meas` | Exit `0` even when `.meas GOAL/TOL` checks fail (default: exit `3`). |
| `--allow-nonfinite` | Export results containing NaN/Inf instead of treating them as simulation errors. |
| `--checkpoint FILE` | Save transient integrator state at end of run. |
| `--resume FILE` | Continue from a checkpoint; the deck is fingerprint-checked. |
| `--tran-stop TIME` | Override `.tran` stop time without changing source text, useful for checkpoint segments. |
| `--compress` | Enable waveform compression. |
| `--compress-tol TOL` | Compression tolerance; requires `--compress`. |
| `-j, --jobs N` | Parallel workers for `.alter`/`.data` multi-run plans and corner sweeps; `0` means all cores. |

**Simulation controls**

| Flag | Meaning |
|---|---|
| `--temp TEMP` | Simulation temperature in Celsius. |
| `--maxiter N` | Maximum Newton iterations. |
| `--abstol TOL` | Absolute convergence tolerance. |
| `--reltol TOL` | Relative convergence tolerance. |
| `--residual-reltol TOL` | Equation-residual convergence tolerance. |
| `--voltage-abstol TOL` | Voltage absolute tolerance (`VNTOL`). |
| `--current-abstol TOL` | Current absolute tolerance. |
| `--charge-abstol TOL` | Charge absolute tolerance (`CHGTOL`). |
| `--min-step TIME` | Minimum transient timestep. |
| `--max-step TIME` | Maximum transient timestep. |
| `--integration-method METHOD` | `trap`, `gear`, `trapgear`, or `euler`. |
| `--trtol TOL` | Transient truncation-error tolerance. |
| `--gmin G` | Initial GMIN conductance for convergence aids. |
| `--convergence MODE` | `fast`, `default`, or `robust`. |

**Analysis-mode flags**

When one of these mode selectors is present, it runs that mode instead of
the deck's ordinary analysis-card sequence.

| Flag | Meaning |
|---|---|
| `--monte-carlo N` | Run N operating-point Monte Carlo parameter-variation iterations. |
| `--seed SEED` | Monte Carlo seed; requires `--monte-carlo`. |
| `--mc-distribution DIST` | `gaussian`, `uniform`, or `worst-case`; requires `--monte-carlo`. |
| `--mc-spread SPREAD` | Non-negative relative sigma/tolerance; `0` repeats nominal samples, default `0.01`. |
| `--mc-param PARAM` | Restrict Monte Carlo variation to specific parameters; repeatable. |
| `--pss-freq FREQ` | Periodic steady-state fundamental frequency. |
| `--pss-harmonics N` | PSS harmonics; default `9`. |
| `--pss-tstab TIME` | PSS stabilization time before shooting; default auto. |
| `--hb-freq FREQ` | Harmonic-balance fundamental frequency. |
| `--hb-harmonics N` | HB harmonics; default `9`. |
| `--pz-input NODE` | Pole-zero input node. |
| `--pz-output NODE` | Pole-zero output node; requires `--pz-input`. |
| `--sens-output NODE` | Sensitivity output node. |
| `--sens-param PARAM` | Sensitivity parameter name; requires `--sens-output`. |
| `--sens-value VALUE` | Sensitivity nominal value; requires `--sens-param`. |
| `--corners LIST` | Comma-separated process corners such as `tt,ss,ff`. |
| `--corner-lib FILE` | Library file with matching `.lib <corner>` sections; requires `--corners`. |
| `--sparam NODES` | Two-port extraction over the deck `.ac` sweep; four nodes as `"P1+,P1-,P2+,P2-"`, with `0` for ground. |
| `--sparam-z0 OHMS` | S-parameter reference impedance; default `50`. |

## Other Commands

### `rspice info NETLIST`

| Flag | Meaning |
|---|---|
| `-d, --detailed` | Show element details. |
| `--models` | Show model definitions. |
| `--hierarchy` | Show subcircuit hierarchy. |
| `--params` | Show parameter values. |
| `--json` | Emit JSON. |

### `rspice check NETLIST`

| Flag | Meaning |
|---|---|
| `--connectivity` | Warn about floating nodes. |
| `--models` | Check undefined models. |
| `--strict` | Treat warnings as errors. |
| `--json` | Emit JSON. |

### `rspice compile-va FILE.va`

| Flag | Meaning |
|---|---|
| `-o, --output FILE` | Write JSON interface summary. |
| `-I, --include DIR` | Extra Verilog-A include directory; repeatable. |
| `--strict` | Enable strict LRM mode. |
| `--detailed` | Show compilation details. |
| `--show-usage` | Include an example instance/model usage block. |

### `rspice convert INPUT OUTPUT --to FORMAT`

| Flag | Meaning |
|---|---|
| `--to FORMAT` | Output format: `raw`, `ascii`, `csv`, `json`, `tsv`, or `hdf5`. |
| `--from FORMAT` | Input format; auto-detected when omitted. |
| `--variables VAR` | Keep selected variables only; repeatable. |
| `--start VALUE` | Keep points at or after this scale value. |
| `--stop VALUE` | Keep points at or before this scale value. |

### `rspice compare RESULT GOLDEN`

| Flag | Meaning |
|---|---|
| `--abstol TOL` | Absolute tolerance; default `1e-9`. |
| `--reltol TOL` | Relative tolerance; default `1e-6`. |
| `--variables VAR` | Compare selected variables only; repeatable. |
| `--fail-fast` | Stop on first mismatch. |
| `--json` | Emit differences as JSON. |
| `--allow-truncated` | Compare only overlap when point counts differ. |
| `--ignore-missing` | Tolerate golden variables missing from the result. |
| `--bless` | Accept the current result as the new golden file. |
| `--interpolate` | Resample result data onto the golden scale before comparing. |

## Configuration and Environment

Configuration is merged in this order: built-in defaults, user config
(`~/.config/rspice/config.toml`, then `~/.rspicerc`), project `./.rspicerc`,
environment variables, and finally command-line arguments.

| Variable | Effect |
|---|---|
| `RSPICE_TEMPERATURE` | Default simulation temperature. |
| `RSPICE_OUTPUT_FORMAT` | Default output format. |
| `RSPICE_INCLUDE_PATH` | Include search paths (`;` on Windows, `:` elsewhere). |
| `RSPICE_LIBRARY_PATH` | Model library paths. |
| `RSPICE_SOLVER=faer` | Opt out of the default KLU-class real solver and use the faer sparse path. |
| `RUST_LOG=debug` | Engine and CLI diagnostics. |

## Exit Status

| Code | Meaning |
|---|---|
| `0` | Success. |
| `1` | Simulation, conversion, or Verilog-A compilation error. |
| `2` | Usage error / invalid arguments. |
| `3` | Verification failure: failed or unevaluated `.meas`, or `compare` mismatch. |
| `65` | Netlist/input format error. |
| `66` | Input file not found. |
| `70` | Internal error. |
| `74` | I/O error. |
| `78` | Configuration error. |
| `124` | `--timeout` expired. |
| `130` | Interrupted by Ctrl-C. |
