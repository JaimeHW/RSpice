# RSpice CLI

Command-line interface for the RSpice simulation engine. Designed for scripted runs, batch simulation, regression testing, and CI pipelines.

## Building

```bash
cargo build --release -p rspice-cli

# Binary location
./target/release/rspice --help
```

## Quick Start

```bash
# Run all analyses requested by the netlist
rspice run circuit.sp

# Write results to a file
rspice run circuit.sp -o results.raw

# Inspect a netlist without simulating
rspice info circuit.sp

# Validate syntax and connectivity
rspice check circuit.sp --connectivity

# Verify the deployed parser-to-solver path
rspice health --json
```

## How Analyses Are Selected

`rspice run` executes every analysis card found in the netlist, in order:

`.OP`, `.DC`, `.TRAN`, `.AC`, `.HB`, `.SP`, `.STB`, `.DISTO`, `.NOISE`, `.TF`, `.SENS`, `.PZ`, `.STEP`, `.FOUR`, `.TEMP`, and Monte Carlo cards. `.AC` and `.NOISE` additionally accept the `DATA=<table>` form, sweeping the frequencies listed in a `.DATA` table instead of a generated sweep. If the netlist contains no analysis cards, a DC operating point is run by default.

Frequency-domain analysis notes:

- `.DISTO` runs the third-order Volterra solver in harmonic or two-tone mode and exports each physical product (`2f1`, `3f1`, `f1+f2`, `f1-f2`, or `2f1-f2`) as an actual sinusoidal peak phasor with magnitude, phase, product frequency, and an explicit magnitude ratio to the F1 response. Two-tone cards use the SPICE ratio contract `0 < f2/f1 < 1`, with F2 fixed relative to the first swept F1 point.
- `.SP` exports S-parameters only. It needs voltage sources annotated `portnum=<n> [z0=<ohms>]`, numbered densely from 1, and the optional ngspice SP-noise flag parses without producing noise output.

A handful of analyses can instead be requested from the command line. When one of these flags is present, it runs **instead of** the netlist's analysis cards; if several are given, the first match in this order wins:

| Mode | Trigger |
| :--- | :--- |
| Monte Carlo | `--monte-carlo N` |
| Periodic steady-state (PSS) | `--pss-freq F` |
| Harmonic balance | `--hb-freq F` |
| Pole-zero | `--pz-input NODE --pz-output NODE [--pz-transfer voltage|current]` |
| DC sensitivity | `--sens-output NODE --sens-param NAME` |
| Two-port S-parameters | `--sparam "P1+,P1-,P2+,P2-"` (needs a `.AC` card for the sweep) |
| Process corners | `--corners tt,ss,ff` |

Each mode's tuning flags are listed under **Analysis-mode options** below.

Numeric flag values accept SPICE magnitude suffixes everywhere: `--pss-freq 2.4G`, `--max-step 1u`, `-D RLOAD=4.7k`.

### Decks that expand into several runs

HSPICE `.ALTER` and `.DATA` constructs expand into a plan of concrete decks, each parsed and solved independently. Every run tags its own output files so a later run cannot overwrite an earlier one: `.ALTER` runs are named `base` and then by block title, `.DATA` runs by table row, each reduced to a file-safe tag (`out.csv` → `out.base.csv`, `out.hot.csv`, `out.tbl_row_1.csv`). `-j` spreads the plan across workers. A failing run does not abort the rest — HSPICE semantics — so each one lands in the reports and the process exit status reflects the whole plan. `resources.max_batch_runs` bounds how large a plan may get.

`.PREPROCESS ADDRESISTORS` writes its derived Xyce-compatible deck alongside the input as `<input>_xyce.cir`. It needs a file-backed netlist, and it is rejected in a multi-run deck, where one sibling name cannot represent several rewritten decks.

### Measurements and the exit status

`.MEAS` statements evaluate for every analysis that produces data:

- **TRAN** against the transient result — node voltages, branch currents as `I(name)`, and the time axis as `TIME`, so `FIND TIME WHEN V(out)=...` works
- **DC** against the sweep, with the swept value as the abscissa
- **AC** against the derived real series — `V(x)`/`VM(x)` magnitude, `VDB(x)`, `VP(x)` phase in degrees, `VR(x)`/`VI(x)`, and the matching `I…` forms for branch currents — with the frequency axis addressable as `FREQUENCY`, `FREQ`, or `TIME`
- **NOISE** against the spectral densities `ONOISE`/`INOISE` (also `*_SPECTRUM`)

Any statement may add `GOAL=value [TOL=value]`: a computed value that misses its goal fails the measurement (TOL defaults to max(1% of |goal|, 1e-12)). Results print under `--meas` and are always collected for report files.

Xyce `TRAN_CONT`, `DC_CONT`, and `AC_CONT` measurements may add `FAILVALUE=value` when the run selects `--spice-dialect xyce`. The threshold is checked independently for every retained record with the exact inclusive contract `abs(raw_value) >= FAILVALUE`. A non-finite raw value or threshold fails closed. The stream passes only when evaluation succeeds, produces at least one record, and every record passes; failed records remain in the stream instead of being replaced by a single aggregate failure.

Continuous rows are serialized additively. JSON and CSV retain `record_index`, raw value, threshold, per-record verdict, event or trigger/target coordinates, and `aggregate_policy=all_records_must_pass`. JUnit and TAP emit one named case per row (`name[record N]`) and include the same contract metadata in their diagnostics. `--allow-failed-meas` changes only the process exit code; it does not change or remove any verdict.

**A failed measurement fails the run with exit code 3** — a missed GOAL, an unevaluated statement, or a measurement whose analysis never ran. Pass `--allow-failed-meas` to restore exit 0. Results containing NaN/Inf are a simulation error (exit 1) unless `--allow-nonfinite` is given.

### Output files for every mode

With `-o`, every run mode writes machine-readable results. When a deck runs several analyses, each writes its own tagged file (`out.csv` → `out.op.csv`, `out.tran.csv`, ...). The `.OP`, `.DC`, `.TRAN`, `.AC`, and `.NOISE` cards write the expected node/branch tables under the `op`, `dc`, `tran`, `ac`, and `noise` tags. The rest have mode-specific shapes:

| Mode | Tag | Contents |
| :--- | :--- | :--- |
| `.STEP` | `step` | One row per step value, node voltages as columns |
| `.FOUR` | `four` | One row per harmonic and output: frequency, magnitude, phase, DC component, THD |
| `.TEMP` | `temp` | One row per temperature point, node voltages as columns |
| Monte Carlo | `mc` | Per-run samples; JSON adds mean/std/min/max, seed, failure count |
| PSS | `pss` | One period of the steady-state waveforms (time domain) |
| HB | `hb` | Complex spectrum per node over the harmonic frequencies |
| `.STB` | `stb` | Complex `loopgain` plus `loopgain_mag_db` and `loopgain_phase_deg` |
| `.TF` | `tf` | Gain, input impedance, output impedance |
| Pole-zero | `pz` | `pole(i)`/`zero(i)` complex columns |
| `.SENS` | `sens` | `dV/d(param)` columns (DC: single point; AC: series over frequency) |
| `.SP` | `sp` | `S_i_j` complex columns for the deck's N ports (Touchstone instead when `-o` ends in a matching `.sNp`) |
| `--sparam` | `sparam` | `S11`/`S21`/`S12`/`S22` complex columns over frequency (Touchstone instead when `-o` ends in `.s2p`) |

An implicit `.STEP` whose topology and complete signal schema are identical at
every coordinate retains the single wide `step` table above. If a conditional
changes either contract, RSpice writes one coordinate-local artifact
(`out.step_000001.csv`, and so on) plus `out.step_schema.json`. The companion
manifest records each deterministic coordinate ID, its topology fingerprint,
the union signal schema, its artifact path, and a validity bitmap. A signal
absent at one coordinate is omitted from that coordinate's artifact and marked
invalid in the bitmap; it is never inferred from the first coordinate or
fabricated as zero. This policy applies to every selected output format; the
companion manifest is JSON.

For a stepped transient, `--checkpoint state.chk` and `--resume state.chk`
resolve one state file per coordinate (`state.step_000001.chk`, and so on).
Outer `.ALTER`/textual-`.DATA` labels are composed into the same filename, so
no run can overwrite or resume another run's solver state. Checkpoint options
on a `.STEP` deck without an authored `.TRAN` are rejected before execution.

TF, pole-zero, and sensitivity tables have no natural HDF5 section and reject `-f hdf5` with a clear error; use `csv`, `json`, or `raw`.

`.FOUR` and `.TEMP` are harmonic and sweep tables rather than waveforms, so they do not use the waveform writers. `.FOUR` honors `csv`/`tsv` and writes JSON for every other format; `.TEMP` honors `csv` and `json`, and writes a plain-text dump otherwise. Ask for `csv` or `json` explicitly with these two.

## Commands

### `rspice health` — Backend Health

`rspice health` is a deployment probe. The default readiness mode validates the effective engine configuration and executes a deterministic, bounded in-memory circuit through parsing, construction, matrix assembly, and a linear DC solve. It performs no filesystem or network I/O. A failed check exits nonzero.

```bash
# Readiness: exercise the complete parser-to-solver path
rspice health --json

# Liveness: validate startup and engine configuration without admitting work
rspice health --mode liveness --json
```

The versioned JSON response includes `status`, `ready`, probe duration, build/runtime identity, a per-check verdict, and the process `run_id` used by JSON logs and fatal diagnostics. Readiness honors the configured resource policy; liveness deliberately skips the synthetic workload so an intentionally narrow admission policy does not make the process appear dead.

### `rspice run` — Execute Simulations

```bash
rspice run <NETLIST> [OPTIONS]
```

Accepts `.sp`, `.cir`, `.net`, and `.spice` netlists, or `-` to read the netlist from stdin (includes then resolve against the working directory).

**Output options:**

| Flag | Description |
| :--- | :--- |
| `-o, --output <FILE>` | Output file for results. With several analysis cards in one deck, each analysis writes its own tagged file: `out.csv` → `out.op.csv`, `out.tran.csv`, ... |
| `-f, --format <FORMAT>` | Output format: `raw`, `ascii`, `csv`, `json`, `tsv`, `hdf5` (default: config `output.format`, else `raw`) |
| `--save <SIGNAL>` | Limit exported signals, replacing the netlist `.SAVE`/`.PROBE`/`.PRINT`/`.PLOT` selection: `V(out)`, `V(a,b)`, `I(v1)`, `@m1[id]`, `all` (repeatable) |
| `--meas` | Print `.MEAS` measurement results |
| `--summary <FILE>` | Write a versioned JSON run summary — build/run identity, execution counts and timing, effective resource limits, typed failures, every measurement, result files, and overall verdict — to FILE, or stdout with `-` |
| `--progress` | Show a live percentage bar during transient analysis (the engine reports its completed fraction; elapsed time, no ETA) |
| `--compress` | Enable waveform compression for long simulations |
| `--compress-tol <TOL>` | Compression tolerance (default: config `compression_tolerance`, else 1e-4; requires `--compress`) |

**Run-discipline options:**

| Flag | Description |
| :--- | :--- |
| `--timeout <SECONDS>` | Abort the run after this budget; exits 124. Transient and DC sweeps stop at the next safe point. Ctrl-C stops the same way and exits 130 |
| `--allow-failed-meas` | Exit 0 even when `.MEAS` measurements fail (default: exit 3) |
| `--allow-nonfinite` | Export results containing NaN/Inf instead of failing (default: simulation error) |
| `--checkpoint <FILE>` | Save the transient integrator state when the run completes, for later `--resume` |
| `--resume <FILE>` | Continue a transient from a saved checkpoint; the deck must be byte-identical (fingerprint-checked) and a segmented run reproduces the uninterrupted waveform |
| `--tran-stop <TIME>` | Override the `.TRAN` stop time without editing the deck, so checkpoint segments share identical source |
| `-j, --jobs <N>` | Parallel workers for `.ALTER`/`.DATA` multi-run plans and corner sweeps (default: 1; `0` = all cores). Outputs are tagged per run or corner so workers never collide, and results are byte-identical to a serial sweep. Above one worker, per-run console output reduces to status lines — files and reports carry the data. Requests above `resources.max_parallel_workers` are rejected rather than silently clamped |

**Simulation options:**

| Flag | Description |
| :--- | :--- |
| `--spice-dialect <DIALECT>` | Select consistent parser and simulator compatibility semantics: `best`, `ngspice`, or `xyce`. `best` uses RSpice's preferred device evaluators with ngspice-compatible expressions |
| `--temp <TEMP>` | Override simulation temperature (Celsius) |
| `--maxiter <N>` | Maximum Newton-Raphson iterations |
| `--abstol <TOL>` | Absolute convergence tolerance |
| `--reltol <TOL>` | Relative convergence tolerance |
| `--residual-reltol <TOL>` | Relative residual tolerance for equation convergence checks |
| `--voltage-abstol <TOL>` | Voltage absolute tolerance (VNTOL) |
| `--current-abstol <TOL>` | Current absolute tolerance |
| `--charge-abstol <TOL>` | Charge absolute tolerance (CHGTOL) |
| `--min-step <TIME>` | Minimum transient timestep |
| `--max-step <TIME>` | Maximum transient timestep |
| `--integration-method <M>` | Transient integrator: `trap`, `gear`, `trapgear` (default), `euler` |
| `--trtol <TOL>` | Transient truncation-error tolerance (TRTOL) |
| `--gmin <G>` | Initial GMIN conductance for convergence aids |
| `--convergence <MODE>` | DC convergence preset: `fast`, `default`, `robust` |
| `-I, --include <DIR>` | Add a search directory for `.include`/`.lib` references (repeatable) |
| `-D, --define <NAME=VALUE>` | Override or define a netlist parameter; values accept SPICE suffixes, e.g. `-D RLOAD=4.7k` (repeatable) |

**Analysis-mode options** (see [How Analyses Are Selected](#how-analyses-are-selected)):

| Flag | Description |
| :--- | :--- |
| `--monte-carlo <N>` | Run N Monte Carlo iterations (operating-point variation). Runs solve in parallel across cores automatically — bounded by `resources.max_parallel_workers`, not by `-j` — with seed-stable sampling, so statistics match a serial sweep exactly. Non-converging runs are dropped from the statistics and counted; all runs failing is an error |
| `--seed <SEED>` | Random seed for Monte Carlo; the default seed 1 makes runs reproducible (requires `--monte-carlo`) |
| `--mc-distribution <D>` | Variation distribution: `gaussian` (default), `uniform`, `worst-case` |
| `--mc-spread <S>` | Relative spread: sigma for gaussian, tolerance otherwise (default: 0.01) |
| `--mc-param <NAME>` | Restrict variation to specific parameters (repeatable) |
| `--pss-freq <FREQ>` | PSS fundamental frequency in Hz |
| `--pss-harmonics <N>` | Number of PSS harmonics (default: 9) |
| `--pss-tstab <TIME>` | PSS stabilization time before the shooting method (default: auto) |
| `--hb-freq <FREQ>` | Harmonic balance fundamental frequency in Hz |
| `--hb-harmonics <N>` | Number of HB harmonics (default: 9) |
| `--pz-input <NODE>` | Pole-zero input node (name or index) |
| `--pz-output <NODE>` | Pole-zero output node (requires `--pz-input`) |
| `--pz-transfer <TYPE>` | Input excitation for the pole-zero transfer: `voltage` (default) or `current` (requires `--pz-input`) |
| `--sens-output <NODE>` | Sensitivity analysis output node (name or index) |
| `--sens-param <PARAM>` | Parameter name for sensitivity analysis (requires `--sens-output`) |
| `--sens-value <VALUE>` | Nominal parameter value (default: 1.0; requires `--sens-param`) |
| `--sparam <NODES>` | Two-port S-parameter extraction over the deck's `.AC` sweep: four port nodes as `"P1+,P1-,P2+,P2-"` (use `0` for grounded references). Writes Touchstone when `-o` ends in `.s2p`, else the standard complex table formats |
| `--sparam-z0 <OHMS>` | S-parameter reference impedance (default: 50) |
| `--corners <LIST>` | Process corners, comma-separated, e.g. `tt,ss,ff` |
| `--corner-lib <FILE>` | Library with one `.lib <corner> ... .endl` section per corner; each corner re-elaborates the deck with its section applied (requires `--corners`) |

Corner runs write per-corner tagged outputs (`res.csv` → `res.tt.csv`, `res.ss.csv`) and exit nonzero if any corner fails. Without `--corner-lib`, every corner runs nominal models and the sweep only checks convergence.

**Reporting options:**

| Flag | Description |
| :--- | :--- |
| `--report-format <FORMAT>` | CI report format: `junit`, `tap` |
| `--report-file <FILE>` | CI report output file (requires `--report-format`) |
| `--meas-format <FORMAT>` | Machine-readable `.MEAS` format: `json`, `csv` |
| `--meas-file <FILE>` | `.MEAS` output file (defaults to JSON if `--meas-format` is omitted) |

**Examples:**

```bash
# Basic simulation with raw output
rspice run amplifier.sp -o amp.raw

# Transient with compression for long simulations
rspice run switching.sp --compress --compress-tol 1e-5

# Monte Carlo yield analysis
rspice run circuit.sp --monte-carlo 1000 --seed 42 -v

# CI pipeline with JUnit output
rspice run circuit.sp -q --report-format junit --report-file results.xml

# Extract measurements to JSON
rspice run circuit.sp --meas --meas-format json --meas-file meas.json

# PSS for oscillators
rspice run vco.sp --pss-freq 2.4e9 --pss-harmonics 15 -v

# Harmonic balance for RF circuits
rspice run mixer.sp --hb-freq 900e6 --hb-harmonics 9

# Pole-zero voltage-transfer analysis (the CLI default)
rspice run opamp.sp --pz-input 1 --pz-output 4

# Pole-zero transimpedance analysis with a current input
rspice run transimpedance.sp --pz-input 1 --pz-output 4 --pz-transfer current

# DC sensitivity
rspice run amp.sp --sens-output 3 --sens-param R1 --sens-value 10k

# Process corner sweep with per-corner model sections
rspice run circuit.sp --corners tt,ss,ff --corner-lib pdk/corners.lib -q

# Override a parameter for a CI sweep
rspice run filter.sp -D CLOAD=2.2p -o fast.csv -f csv
```

### `rspice info` — Netlist Information

Display a netlist summary without running a simulation. Accepts `-` for stdin.

```bash
rspice info <NETLIST> [OPTIONS]
```

| Flag | Description |
| :--- | :--- |
| `-d, --detailed` | Show detailed element information (text output only) |
| `--models` | Show model definitions |
| `--hierarchy` | Show subcircuit hierarchy |
| `--params` | Show parameter values |
| `--json` | Output as JSON |

The JSON document always carries the title, per-kind element counts, analysis and measurement counts, and any parser diagnostics; `--models`, `--params`, and `--hierarchy` add their arrays. `--detailed` has no JSON counterpart.

### `rspice check` — Validate Netlist

Check netlist syntax and topology. Accepts `-` for stdin.

Four checks always run, before any flag is considered:

- **Parse diagnostics** from the netlist reader are reported as warnings.
- **Output symbols** referenced by `.PRINT`/`.PLOT`/`.SAVE`/`.PROBE` must resolve; an undefined `V(x)` or `I(rbogus)` is an error.
- **Singular topology**: a loop of ideal voltage sources/inductors is an error (the DC system cannot be solved), and a node connected only to current sources warns about its undefined voltage.
- **XSPICE build**: a deck containing XSPICE devices is built into a circuit with external runtimes stubbed out, so a model that cannot be constructed fails here rather than at run time.

```bash
rspice check <NETLIST> [OPTIONS]
```

| Flag | Description |
| :--- | :--- |
| `--connectivity` | Warn about floating nodes |
| `--models` | Check for undefined models |
| `--strict` | Treat warnings as errors |
| `--json` | Output as JSON |

Errors exit 65; `--strict` turns a warning-only deck into a usage failure, which exits 2. The JSON document reports both verdicts separately — `valid` tracks the non-strict exit status, and `strict_valid` stays false whenever there are warnings — alongside the `errors` and `warnings` arrays.

### `rspice compare` — Golden File Comparison

Compare simulation results against a reference file for regression testing. Both files may be in any supported result format — rawfile (binary or ASCII), CSV, TSV, JSON, or HDF5, auto-detected by extension — so a binary rawfile result can be checked directly against a CSV golden. Complex AC data compares value-for-value as `Re(..)`/`Im(..)` series.

The golden file defines the contract: golden variables missing from the result fail, point-count mismatches fail (a result truncated by a crashed run cannot pass on the overlap it wrote), and NaN never matches anything. A passing comparison exits `0`; mismatches exit `3` (verification failure).

```bash
rspice compare <RESULT> <GOLDEN> [OPTIONS]
```

| Flag | Description |
| :--- | :--- |
| `--abstol <TOL>` | Absolute tolerance (default: 1e-9) |
| `--reltol <TOL>` | Relative tolerance (default: 1e-6) |
| `--variables <VAR>` | Compare specific variables only (repeatable; default: all) |
| `--fail-fast` | Stop on first difference |
| `--json` | Output differences as JSON |
| `--allow-truncated` | Tolerate point-count mismatches and compare the overlap |
| `--ignore-missing` | Tolerate golden variables missing from the result |
| `--bless` | Accept the result as the new reference: copies it over the golden file when they differ, or creates the golden file if it does not exist |
| `--interpolate` | Linearly resample the result onto the golden file's scale, so runs with different time grids compare point-for-point (never extrapolates) |

### `rspice convert` — Format Conversion

Convert between simulation output formats: `raw`, `ascii`, `csv`, `json`, `tsv`, `hdf5`. Complex AC data is preserved across every round trip (`Re(..)`/`Im(..)` column pairs in CSV/TSV, `Flags: complex` in rawfiles, real/imag arrays in JSON and HDF5).

```bash
rspice convert <INPUT> <OUTPUT> --to <FORMAT> [OPTIONS]
```

| Flag | Description |
| :--- | :--- |
| `--to <FORMAT>` | Output format (required) |
| `--from <FORMAT>` | Input format (auto-detected from the extension if omitted) |
| `--variables <VAR>` | Variables to keep, by full (`V(out)`) or bare (`out`) name (repeatable; default: all) |
| `--start <VALUE>` | Keep points with scale ≥ this time/frequency |
| `--stop <VALUE>` | Keep points with scale ≤ this time/frequency |

```bash
rspice convert results.raw results.csv --to csv
rspice convert results.csv results.h5 --to hdf5
rspice convert tran.raw window.csv --to csv --variables "V(out)" --start 1u --stop 5u
```

### `rspice compile-va` — Compile Verilog-A

Compile a Verilog-A model and report its interface. Terminals, internal node count, and the parameter table (with defaults and bounds) always print to stdout; a compile failure exits 1.

```bash
rspice compile-va <FILE> [OPTIONS]
```

| Flag | Description |
| :--- | :--- |
| `-o, --output <FILE>` | Write a JSON interface summary: model name, terminals, internal node count, parameters with defaults and bounds |
| `-I, --include <DIR>` | Add an include directory (repeatable) |
| `--strict` | Enable strict LRM compliance mode |
| `--detailed` | Also print per-branch stamp and Jacobian entry counts |
| `--show-usage` | Print an example `.VERILOGA` line, instantiation, and `.MODEL` card for the compiled model |

Includes are searched in order: `-I` directories, then config `paths.veriloga_includes`, then the source file's own directory.

### `rspice completions` — Shell Completions

Emit a completion script on stdout for `bash`, `zsh`, `fish`, `powershell`, or `elvish`.

```bash
rspice completions bash > /etc/bash_completion.d/rspice
rspice completions powershell >> $PROFILE
```

## Global Options

Available on every subcommand:

| Flag | Description |
| :--- | :--- |
| `-v, --verbose` | Enable debug-level output |
| `-q, --quiet` | Suppress non-error output |
| `--config <FILE>` | Use this configuration file instead of the discovered user and project files |
| `--log-level <LEVEL>` | Set log level: `off`, `error`, `warn`, `info`, `debug`, `trace` |
| `--log-format <FORMAT>` | Log format: `text` (default) or newline-delimited `json` with timestamp, source, process, thread, and `run_id` |
| `--error-format <FORMAT>` | Fatal diagnostic format: `text` (default) or versioned `json` with stable code/category, retry policy, exit code, and numeric resource/convergence details |

`rspice --version` reports the crate version, build target, profile, and exact
source commit. The same commit appears in health documents, structured fatal
diagnostics, and run summaries so operators can correlate an installed binary
with its release provenance. The commit is read from `git rev-parse HEAD` at
build time; set `RSPICE_BUILD_COMMIT` to a full 40-character hash to stamp
provenance when building from a source archive with no git checkout.

Every JSON document the CLI emits — health, fatal diagnostics, `--summary`,
`--log-format json` records — carries the process `run_id`, so one run's logs,
failure, and summary can be correlated after the fact.

## Configuration File

Configuration is loaded and merged in order of increasing priority:

1. Built-in defaults
2. User config: `~/.config/rspice/config.toml`, falling back to `~/.rspicerc`
3. Project config: `./.rspicerc`
4. Environment variables
5. Command-line arguments

`--config <FILE>` replaces layers 2 and 3 with that single file. Scalar keys override the layer below; the `[paths]` lists accumulate, so a project file adds search directories rather than discarding the user's.

All files use TOML. Unknown keys are rejected rather than ignored, and every value is range-checked on load (`min_timestep <= max_timestep`, positive tolerances, known enum names) — a bad file or environment override exits 78 naming the offending key. The full set of recognized keys, with their defaults:

```toml
[simulation]
temperature = 27.0            # Celsius
max_iterations = 50
abstol = 1e-12
reltol = 1e-3
residual_reltol = 1e-3
min_timestep = 1e-12
max_timestep = 1e-3
compress_waveforms = false
compression_tolerance = 1e-4
convergence_mode = "default"  # "fast" | "default" | "robust"

[output]
format = "raw"                   # default for -f/--format
show_progress = false            # default for --progress
# output_directory = "results"   # relative -o paths land here (created on demand)

[paths]
include_paths = ["./models", "./lib"]   # extra .include/.lib search dirs for run
library_paths = []                      # extra .include/.lib search dirs for run
veriloga_includes = []                  # extra include dirs for compile-va

[resources]
max_netlist_bytes = 67108864             # 64 MiB root deck
max_netlist_lines = 2000000
max_expanded_source_bytes = 268435456    # includes and retained multi-run decks
max_dependency_source_bytes = 268435456
max_external_data_bytes = 268435456
max_external_data_values = 25000000
max_shared_cache_bytes = 536870912
max_include_depth = 64
max_hierarchy_depth = 100
max_flattened_elements = 250000
max_circuit_nodes = 250000
max_matrix_unknowns = 250000
max_analysis_points = 2000000
max_result_values = 25000000
max_parallel_workers = 64               # cap batch/analysis worker fan-out
max_batch_runs = 10000
```

The same resource policy is applied consistently to `run`, `check`, and
`info`, including stdin, include expansion, `.ALTER`/`.DATA` materialization,
derived corner/S-parameter decks, circuit construction, and result retention.
The worker ceiling also bounds automatic `--jobs 0` fan-out; an explicit
`--jobs` request that would exceed it is rejected. Lower these ceilings for a
service with a fixed CPU or memory budget.

Environment variable overrides:

| Variable | Effect |
| :--- | :--- |
| `RSPICE_TEMPERATURE` | Default simulation temperature (Celsius) |
| `RSPICE_OUTPUT_FORMAT` | Default output format |
| `RSPICE_INCLUDE_PATH` | Include search paths, platform path separator (`;` on Windows, `:` elsewhere) |
| `RSPICE_LIBRARY_PATH` | Model library paths, platform path separator |
| `RSPICE_MAX_<RESOURCE>` | Override any `[resources]` key in uppercase, for example `RSPICE_MAX_BATCH_RUNS` or `RSPICE_MAX_NETLIST_BYTES` |

## Exit Codes

The exit status is the verification contract — a deck whose measurements fail, or whose results are non-finite, does not exit 0.

Every nonzero code is derived from one **failure category**, and for anything the engine produced that category is the engine's own (`rspice_core::SimulationErrorCategory`), so the exit status, the `--error-format json` `category` field, and the `--summary` report always agree. Automation can branch on the number alone:

| Code | Category | Meaning |
| :--- | :--- | :--- |
| 0 | — | Success: simulation ran and every check passed |
| 1 | `compilation`, `conversion` | A failure with no engine category: Verilog-A compile failure, result-format conversion failure |
| 2 | `usage` | Usage error (invalid arguments, or warnings under `check --strict`) |
| 3 | `verification` | A `.MEAS` failed or did not evaluate, or `compare` found mismatches |
| 65 | `netlist` | Invalid authored input: netlist parse failure, singular topology from `check` |
| 66 | `input_not_found` | Input file not found |
| 69 | `capability` | The deck is well formed and this build does not execute it — an unsupported analysis/device combination, model family, or netlist construct |
| 70 | `internal` | Internal error |
| 73 | `output_commit` | The run produced correct results and publishing them failed; the previous artifact is intact unless the message says otherwise |
| 74 | `io` | I/O error outside a publication transaction (failed to read input) |
| 75 | `resource_limit` | A configured resource budget was exceeded; the same workload succeeds under a larger budget |
| 76 | `persistence` | A checkpoint or other persisted artifact was written by an incompatible format version |
| 78 | `configuration` | Configuration error |
| 80 | `simulation` | Circuit construction or device evaluation failed |
| 81 | `solver` | The numerical solver failed |
| 82 | `convergence` | An iterative analysis exhausted its convergence strategy |
| 83 | `signal_unavailable` | A valid authored output symbol is absent from the produced result |
| 84 | `result_schema` | A produced result violates its own published schema |
| 85 | `materialization` | A materialized `.STEP`/`.TEMP` run disagrees with the plan that produced it |
| 124 | `timeout` | Run exceeded `--timeout` |
| 130 | `cancellation` | Interrupted (Ctrl-C) |

65-78 keep their `sysexits.h` meanings; 80-85 are an RSpice block for engine-domain outcomes `sysexits` has no name for. No engine category exits 1 — a `1` means only the two frontend failures listed above.

## CI Integration

`--report-format junit` writes JUnit XML that most CI systems ingest directly as test results: one test suite per run, holding a `simulation` case plus one case per `.MEAS` statement, so a missed goal shows up as a named failing test rather than a log line. `--report-format tap` reports the same content as TAP. Publish the file as a build artifact and the analog checks appear alongside the software tests.

A typical verification pipeline:

```bash
# 1. Validate the netlist (syntax + singular-topology checks)
rspice check circuit.sp --connectivity --strict

# 2. Run with a time budget; failed .MEAS checks exit 3, NaN results exit 1
rspice run circuit.sp -q -o results.csv -f csv --timeout 600 \
        --summary summary.json

# 3. Compare against the golden reference (mismatches exit 3)
rspice compare results.csv golden.csv --abstol 1e-9

# 4. After a reviewed, intentional change: accept the new waveforms
rspice compare results.csv golden.csv --bless
```

Because failed measurements, non-finite results, comparison mismatches, and timeouts all map to distinct nonzero exit codes, `rspice run deck.sp && deploy` is safe without parsing any output. The `--summary` JSON carries the same verdict plus every measurement value for archiving.

## License

RSpice CLI is part of the RSpice project and is licensed under the [RSpice Personal Use License](../../LICENSE).
