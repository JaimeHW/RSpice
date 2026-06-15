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
```

## How Analyses Are Selected

`rspice run` executes every analysis card found in the netlist, in order:

`.OP`, `.DC`, `.TRAN`, `.AC`, `.DISTO`, `.NOISE`, `.SENS`, `.PZ`, `.STEP`, `.FOUR`, `.TEMP`, and Monte Carlo cards. If the netlist contains no analysis cards, a DC operating point is run by default.

Compatibility note: `.DISTO` is accepted as a deck card, but the CLI currently maps it to the corresponding linearized AC sweep and does not emit Volterra distortion products. Use `.FOUR`/THD/IMD post-processing for distortion metrics.

A handful of analyses can instead be requested from the command line. When one of these flags is present, it runs **instead of** the netlist's analysis cards:

| Mode | Flags |
| :--- | :--- |
| Monte Carlo | `--monte-carlo N` (optional `--seed`, `--mc-distribution`, `--mc-spread`, `--mc-param`) |
| Periodic steady-state (PSS) | `--pss-freq F` (optional `--pss-harmonics`, `--pss-tstab`) |
| Harmonic balance | `--hb-freq F` (optional `--hb-harmonics`) |
| Pole-zero | `--pz-input NODE --pz-output NODE` (node names or indices) |
| DC sensitivity | `--sens-output NODE --sens-param NAME` (optional `--sens-value`) |
| Process corners | `--corners tt,ss,ff` (optional `--corner-lib`, `-j` parallel workers) |
| Two-port S-parameters | `--sparam "P1+,P1-,P2+,P2-"` (optional `--sparam-z0`; needs a `.AC` card) |

Numeric flag values accept SPICE magnitude suffixes everywhere: `--pss-freq 2.4G`, `--max-step 1u`, `-D RLOAD=4.7k`.

### Measurements and the exit status

`.MEAS` statements evaluate for every analysis that produces data:

- **TRAN** against the transient result — node voltages, branch currents as `I(name)`, and the time axis as `TIME`, so `FIND TIME WHEN V(out)=...` works
- **DC** against the sweep, with the swept value as the abscissa
- **AC** against the derived real series — `V(x)`/`VM(x)` magnitude, `VDB(x)`, `VP(x)` phase in degrees, `VR(x)`/`VI(x)` — with the frequency axis addressable as `FREQUENCY`, `FREQ`, or `TIME`
- **NOISE** against the spectral densities `ONOISE`/`INOISE` (also `*_SPECTRUM`)

Any statement may add `GOAL=value [TOL=value]`: a computed value that misses its goal fails the measurement (TOL defaults to max(1% of |goal|, 1e-12)). Results print under `--meas` and are always collected for report files.

**A failed measurement fails the run with exit code 3** — a missed GOAL, an unevaluated statement, or a measurement whose analysis never ran. Pass `--allow-failed-meas` to restore exit 0. Results containing NaN/Inf are a simulation error (exit 1) unless `--allow-nonfinite` is given.

### Output files for every mode

With `-o`, every run mode writes machine-readable results. When a deck runs several analyses, each writes its own tagged file (`out.csv` → `out.op.csv`, `out.tran.csv`, ...). Mode-specific shapes:

| Mode | Tag | Contents |
| :--- | :--- | :--- |
| `.STEP` | `step` | One row per step value, node voltages as columns |
| Monte Carlo | `mc` | Per-run samples; JSON adds mean/std/min/max, seed, failure count |
| PSS | `pss` | One period of the steady-state waveforms (time domain) |
| HB | `hb` | Complex spectrum per node over the harmonic frequencies |
| `.TF` | `tf` | Gain, input impedance, output impedance |
| Pole-zero | `pz` | `pole(i)`/`zero(i)` complex columns |
| `.SENS` | `sens` | `dV/d(param)` columns (DC: single point; AC: series over frequency) |
| S-parameters | `sparam` | `S11`/`S21`/`S12`/`S22` complex columns over frequency (Touchstone instead when `-o` ends in `.s2p`) |

TF, pole-zero, and sensitivity tables have no natural HDF5 section and reject `-f hdf5` with a clear error; use `csv`, `json`, or `raw`.

## Commands

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
| `--save <SIGNAL>` | Limit exported signals, replacing the netlist `.SAVE`/`.PROBE` selection: `V(out)`, `V(a,b)`, `I(v1)`, `@m1[id]`, `all` (repeatable) |
| `--meas` | Print `.MEAS` measurement results |
| `--summary <FILE>` | Write a JSON run summary — tool version, per-run status, every measurement, the result files written, overall verdict — to FILE, or stdout with `-` |
| `--progress` | Show a live percentage bar during transient analysis (the engine reports its completed fraction) |
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

**Simulation options:**

| Flag | Description |
| :--- | :--- |
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
| `--monte-carlo <N>` | Run N Monte Carlo iterations (operating-point variation); independent runs solve in parallel across cores with seed-stable sampling |
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
| `--sens-output <NODE>` | Sensitivity analysis output node (name or index) |
| `--sens-param <PARAM>` | Parameter name for sensitivity analysis (requires `--sens-output`) |
| `--sens-value <VALUE>` | Nominal parameter value (default: 1.0; requires `--sens-param`) |
| `--sparam <NODES>` | Two-port S-parameter extraction over the deck's `.AC` sweep: four port nodes as `"P1+,P1-,P2+,P2-"` (use `0` for grounded references). Writes Touchstone when `-o` ends in `.s2p`, else the standard complex table formats |
| `--sparam-z0 <OHMS>` | S-parameter reference impedance (default: 50) |
| `--corners <LIST>` | Process corners, comma-separated, e.g. `tt,ss,ff` |
| `--corner-lib <FILE>` | Library with one `.lib <corner> ... .endl` section per corner; each corner re-elaborates the deck with its section applied (requires `--corners`) |
| `-j, --jobs <N>` | Parallel workers for corner sweeps (default: 1). Corner outputs are tagged per corner so workers never collide; results are byte-identical to a serial sweep |

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

# Pole-zero analysis
rspice run opamp.sp --pz-input 1 --pz-output 4

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
| `-d, --detailed` | Show detailed element information |
| `--models` | Show model definitions |
| `--hierarchy` | Show subcircuit hierarchy |
| `--params` | Show parameter values |
| `--json` | Output as JSON |

### `rspice check` — Validate Netlist

Check netlist syntax and topology. Accepts `-` for stdin. Two singular-topology checks always run: a loop of ideal voltage sources/inductors is an error (the DC system cannot be solved), and a node connected only to current sources warns about its undefined voltage.

```bash
rspice check <NETLIST> [OPTIONS]
```

| Flag | Description |
| :--- | :--- |
| `--connectivity` | Warn about floating nodes |
| `--models` | Check for undefined models |
| `--strict` | Treat warnings as errors |
| `--json` | Output as JSON |

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

Compile a Verilog-A model for use in simulations.

```bash
rspice compile-va <FILE> [OPTIONS]
```

| Flag | Description |
| :--- | :--- |
| `-o, --output <FILE>` | Write a JSON interface summary: model name, terminals, internal node count, parameters with defaults and bounds |
| `-I, --include <DIR>` | Add an include directory (repeatable) |
| `--strict` | Enable strict LRM compliance mode |
| `--detailed` | Show detailed compilation information |
| `--show-usage` | Generate a usage example in the output |

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
| `--config <FILE>` | Use a specific configuration file |
| `--log-level <LEVEL>` | Set log level: `off`, `error`, `warn`, `info`, `debug`, `trace` |

`rspice --version` reports the crate version with the build target and profile.

## Configuration File

Configuration is loaded and merged in order of increasing priority:

1. Built-in defaults
2. User config: `~/.config/rspice/config.toml`, falling back to `~/.rspicerc`
3. Project config: `./.rspicerc`
4. Environment variables
5. Command-line arguments

All files use TOML. The full set of recognized keys, with their defaults:

```toml
[simulation]
temperature = 27.0            # Celsius
max_iterations = 50
abstol = 1e-12
reltol = 1e-3
residual_reltol = 1e-3
min_timestep = 1e-15
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
```

Environment variable overrides:

| Variable | Effect |
| :--- | :--- |
| `RSPICE_TEMPERATURE` | Default simulation temperature (Celsius) |
| `RSPICE_OUTPUT_FORMAT` | Default output format |
| `RSPICE_INCLUDE_PATH` | Include search paths, platform path separator (`;` on Windows, `:` elsewhere) |
| `RSPICE_LIBRARY_PATH` | Model library paths, platform path separator |

## Exit Codes

The exit status is the verification contract — a deck whose measurements fail, or whose results are non-finite, does not exit 0:

| Code | Meaning |
| :--- | :--- |
| 0 | Success: simulation ran and every check passed |
| 1 | Simulation error (convergence failure, non-finite results, Verilog-A compile failure, conversion failure) |
| 2 | Usage error (invalid arguments) |
| 3 | Verification failure: a `.MEAS` failed or did not evaluate, or `compare` found mismatches |
| 65 | Input format error (netlist parse failure, singular topology from `check`) |
| 66 | Input file not found |
| 70 | Internal error |
| 74 | I/O error (failed to read input or write output) |
| 78 | Configuration error |
| 124 | Run exceeded `--timeout` |
| 130 | Interrupted (Ctrl-C) |

## CI Integration

`--report-format junit` produces JUnit XML, which most CI systems can ingest directly. Example with GitHub Actions:

```yaml
jobs:
  analog-verification:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install RSpice
        run: cargo install --path crates/rspice-cli

      - name: Run simulations
        run: rspice run circuits/amplifier.sp -q --report-format junit --report-file results.xml

      - name: Regression check
        run: rspice compare output.csv golden/expected.csv --abstol 1e-9

      - name: Upload results
        uses: actions/upload-artifact@v4
        with:
          name: simulation-results
          path: results.xml
```

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
