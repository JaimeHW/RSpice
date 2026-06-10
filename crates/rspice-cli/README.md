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

A handful of analyses can instead be requested from the command line. When one of these flags is present, it runs **instead of** the netlist's analysis cards:

| Mode | Flags |
| :--- | :--- |
| Monte Carlo | `--monte-carlo N` (optional `--seed`) |
| Periodic steady-state (PSS) | `--pss-freq F` (optional `--pss-harmonics`, `--pss-tstab`) |
| Harmonic balance | `--hb-freq F` (optional `--hb-harmonics`) |
| Pole-zero | `--pz-input NODE --pz-output NODE` |
| DC sensitivity | `--sens-output NODE --sens-param NAME` (optional `--sens-value`) |
| Process corners | `--corners tt,ss,ff` (optional `--corner-lib`) |

## Commands

### `rspice run` — Execute Simulations

```bash
rspice run <NETLIST> [OPTIONS]
```

Accepts `.sp`, `.cir`, `.net`, and `.spice` netlists.

**Output options:**

| Flag | Description |
| :--- | :--- |
| `-o, --output <FILE>` | Output file for results |
| `-f, --format <FORMAT>` | Output format: `raw` (default), `ascii`, `csv`, `json`, `tsv`, `hdf5` |
| `--meas` | Print `.MEAS` measurement results |
| `--progress` | Show a progress bar with ETA for transient analysis |
| `--compress` | Enable waveform compression for long simulations |
| `--compress-tol <TOL>` | Compression tolerance (default: 1e-4; requires `--compress`) |

**Simulation options:**

| Flag | Description |
| :--- | :--- |
| `--temp <TEMP>` | Override simulation temperature (Celsius) |
| `--maxiter <N>` | Maximum Newton-Raphson iterations |
| `--abstol <TOL>` | Absolute convergence tolerance |
| `--reltol <TOL>` | Relative convergence tolerance |
| `--residual-reltol <TOL>` | Relative residual tolerance for equation convergence checks |
| `--min-step <TIME>` | Minimum transient timestep |
| `--max-step <TIME>` | Maximum transient timestep |
| `--convergence <MODE>` | DC convergence preset: `fast`, `default`, `robust` |
| `-I, --include <DIR>` | Add a search directory for `.include`/`.lib` references (repeatable) |
| `-D, --define <NAME=VALUE>` | Override or define a netlist parameter; values accept SPICE suffixes, e.g. `-D RLOAD=4.7k` (repeatable) |

**Analysis-mode options** (see [How Analyses Are Selected](#how-analyses-are-selected)):

| Flag | Description |
| :--- | :--- |
| `--monte-carlo <N>` | Run N Monte Carlo iterations |
| `--seed <SEED>` | Random seed for Monte Carlo (requires `--monte-carlo`) |
| `--pss-freq <FREQ>` | PSS fundamental frequency in Hz |
| `--pss-harmonics <N>` | Number of PSS harmonics (default: 9) |
| `--pss-tstab <TIME>` | PSS stabilization time before the shooting method (default: auto) |
| `--hb-freq <FREQ>` | Harmonic balance fundamental frequency in Hz |
| `--hb-harmonics <N>` | Number of HB harmonics (default: 9) |
| `--pz-input <NODE>` | Pole-zero input node index |
| `--pz-output <NODE>` | Pole-zero output node index (requires `--pz-input`) |
| `--sens-output <NODE>` | Sensitivity analysis output node index |
| `--sens-param <PARAM>` | Parameter name for sensitivity analysis (requires `--sens-output`) |
| `--sens-value <VALUE>` | Nominal parameter value (default: 1.0; requires `--sens-param`) |
| `--corners <LIST>` | Process corners, comma-separated: `tt,ss,ff,sf,fs` |
| `--corner-lib <FILE>` | Library file with corner definitions (requires `--corners`) |

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

# Process corner sweep
rspice run circuit.sp --corners tt,ss,ff,sf,fs -q
```

### `rspice info` — Netlist Information

Display a netlist summary without running a simulation.

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

Check netlist syntax and connectivity.

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

Compare simulation results against a reference file for regression testing. Exits with code `0` if the comparison passes and `1` if differences are found.

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

### `rspice convert` — Format Conversion

Convert between simulation output formats: `raw`, `ascii`, `csv`, `json`, `tsv`, `hdf5`.

```bash
rspice convert <INPUT> <OUTPUT> --to <FORMAT> [OPTIONS]
```

| Flag | Description |
| :--- | :--- |
| `--to <FORMAT>` | Output format (required) |
| `--from <FORMAT>` | Input format (auto-detected if omitted) |
| `--variables <VAR>` | Variables to include (repeatable; default: all) |
| `--start <VALUE>` | Time/frequency range start |
| `--stop <VALUE>` | Time/frequency range end |

```bash
rspice convert results.raw results.csv --to csv
rspice convert results.csv results.h5 --to hdf5
```

### `rspice compile-va` — Compile Verilog-A

Compile a Verilog-A model for use in simulations.

```bash
rspice compile-va <FILE> [OPTIONS]
```

| Flag | Description |
| :--- | :--- |
| `-o, --output <FILE>` | Output compiled model (optional, for caching) |
| `-I, --include <DIR>` | Add an include directory (repeatable) |
| `--strict` | Enable strict LRM compliance mode |
| `--detailed` | Show detailed compilation information |
| `--show-usage` | Generate a usage example in the output |

## Global Options

Available on every subcommand:

| Flag | Description |
| :--- | :--- |
| `-v, --verbose` | Enable debug-level output |
| `-q, --quiet` | Suppress non-error output |
| `--config <FILE>` | Use a specific configuration file |
| `--log-level <LEVEL>` | Set log level: `error`, `warn`, `info`, `debug`, `trace` |

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
format = "raw"
show_progress = false
include_node_names = false
# output_directory = "results"   # default: same directory as input

[paths]
include_paths = ["./models", "./lib"]
library_paths = []
veriloga_includes = []
```

Environment variable overrides:

| Variable | Effect |
| :--- | :--- |
| `RSPICE_TEMPERATURE` | Default simulation temperature (Celsius) |
| `RSPICE_OUTPUT_FORMAT` | Default output format |
| `RSPICE_INCLUDE_PATH` | Include paths, semicolon-separated |
| `RSPICE_LIBRARY_PATH` | Model library paths, semicolon-separated |

## Exit Codes

Exit codes follow BSD `sysexits` conventions:

| Code | Meaning |
| :--- | :--- |
| 0 | Success |
| 1 | General error (simulation failure, Verilog-A compile failure, conversion failure, comparison differences) |
| 2 | Usage error (invalid arguments) |
| 65 | Input format error (netlist parse failure) |
| 66 | Input file not found |
| 70 | Internal error |
| 74 | I/O error (failed to read input or write output) |
| 78 | Configuration error |

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
# 1. Validate the netlist
rspice check circuit.sp --connectivity --strict

# 2. Run the simulation
rspice run circuit.sp -q -o results.csv -f csv

# 3. Compare against the golden reference
rspice compare results.csv golden.csv --abstol 1e-9

# 4. Extract measurements
rspice run circuit.sp --meas-format json --meas-file metrics.json
```

## License

RSpice CLI is part of the RSpice project and is licensed under the [RSpice Personal Use License](../../LICENSE).
