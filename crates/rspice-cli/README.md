# RSpice CLI

**Command-line interface for automated analog circuit simulation and verification.**

The RSpice CLI provides a subcommand-based interface designed for batch simulation, CI/CD pipelines, and automated verification workflows.

---

## Installation

```bash
# Build from source
cd rspice
cargo build --release -p rspice-cli

# Binary location
./target/release/rspice --help
```

---

## Quick Start

```bash
# Run simulation
rspice run circuit.sp

# Run with output file
rspice run circuit.sp -o results.raw

# Show netlist info without simulating
rspice info circuit.sp

# Validate netlist
rspice check circuit.sp --connectivity
```

---

## Supported Analyses

| Analysis | CLI Flag | Description |
|----------|----------|-------------|
| DC Operating Point | `rspice run` | Finds DC solution with Newton-Raphson |
| DC Sweep | Built-in | Nested voltage/current/parameter sweeps |
| Transient | Built-in | Time-domain with adaptive timestepping |
| AC Small-Signal | Built-in | Frequency response |
| Noise | `--noise` | Thermal, Shot, and Flicker (1/f) noise |
| Monte Carlo | `--monte-carlo N` | Statistical yield with histogram & 3σ |
| Corner Analysis | `--corners tt,ss,ff` | PVT sweep with NMOS/PMOS scaling |
| PSS | `--pss-freq F` | Periodic Steady-State for oscillators |
| Harmonic Balance | `--hb-freq F` | RF steady-state solution |
| Pole-Zero | `--pz-input/output` | Transfer function analysis |
| Sensitivity | `--sens-*` | DC/AC sensitivity |

## Device Models

| Device | Models |
|--------|--------|
| **MOSFET** | BSIM4, BSIM3v3.24 (submicron), EKV, VDMOS (Power), Level 1-3 |
| **BJT** | Gummel-Poon (NPN/PNP) with quasi-saturation and high-injection |
| **Diode** | Shockley with junction and diffusion capacitance |
| **JFET** | Curtice model with channel length modulation |
| **GaN HEMT** | Cubic model with self-heating and trapping effects |
| **Verilog-A** | JIT-compiled behavioral models via Cranelift |

---

## Commands


### `rspice run` — Execute Simulations

Run SPICE simulations with full analysis support.

```bash
rspice run <NETLIST> [OPTIONS]
```

**Options:**

| Flag | Description |
|------|-------------|
| `-o, --output <FILE>` | Output file for results |
| `-f, --format <FORMAT>` | Output format: `raw`, `ascii`, `csv`, `json`, `tsv`, `hdf5` |
| `--temp <TEMP>` | Override temperature (Celsius) |
| `--meas` | Print `.MEAS` measurement results |
| `--progress` | Show progress bar with ETA |
| `--compress` | Enable waveform compression (10-100x memory reduction) |
| `--compress-tol <TOL>` | Compression tolerance (default: 1e-4) |
| `--monte-carlo <N>` | Run N Monte Carlo iterations |
| `--seed <SEED>` | Random seed for Monte Carlo |
| `--maxiter <N>` | Maximum Newton-Raphson iterations |
| `--abstol <TOL>` | Convergence tolerance |
| `-I, --include <DIR>` | Add include path |
| `-D, --define <NAME=VALUE>` | Define parameter |

**CI/CD Options:**

| Flag | Description |
|------|-------------|
| `--report-format <FORMAT>` | Report format: `junit`, `tap` |
| `--report-file <FILE>` | Report output file |
| `--meas-format <FORMAT>` | Measurement format: `json`, `csv` |
| `--meas-file <FILE>` | Measurement output file |
| `-q, --quiet` | Suppress non-error output |
| `-b, --batch` | Batch mode (no prompts) |

**Advanced RF/Analog Analysis:**

| Flag | Description |
|------|-------------|
| `--pss-freq <FREQ>` | Run PSS analysis at fundamental frequency |
| `--pss-harmonics <N>` | Number of PSS harmonics (default: 9) |
| `--pss-tstab <TIME>` | PSS stabilization time |
| `--hb-freq <FREQ>` | Run Harmonic Balance at fundamental frequency |
| `--hb-harmonics <N>` | Number of HB harmonics (default: 9) |
| `--pz-input <NODE>` | Pole-Zero input node |
| `--pz-output <NODE>` | Pole-Zero output node |
| `--sens-output <NODE>` | Sensitivity analysis output node |
| `--sens-param <PARAM>` | Parameter for sensitivity analysis |
| `--sens-value <VALUE>` | Nominal parameter value |
| `--corners <LIST>` | Process corners (comma-separated: tt,ss,ff,sf,fs) |
| `--corner-lib <FILE>` | Library file with corner definitions |

**Examples:**

```bash
# Basic simulation
rspice run amplifier.sp -o amp.raw

# Transient with compression for long simulations
rspice run switching.sp --compress --compress-tol 1e-5

# Monte Carlo yield analysis
rspice run circuit.sp --monte-carlo 1000 --seed 42 -v

# CI pipeline with JUnit output
rspice run circuit.sp -q --report-format junit --report-file results.xml

# Extract measurements to JSON
rspice run circuit.sp --meas --meas-format json --meas-file meas.json

# PSS (Periodic Steady-State) for oscillators
rspice run vco.sp --pss-freq 2.4e9 --pss-harmonics 15 -v

# Harmonic Balance for RF circuits
rspice run mixer.sp --hb-freq 900e6 --hb-harmonics 9

# Pole-Zero analysis
rspice run opamp.sp --pz-input 1 --pz-output 4

# Sensitivity analysis
rspice run amp.sp --sens-output 3 --sens-param R1 --sens-value 10k

# Process corner sweep
rspice run circuit.sp --corners tt,ss,ff,sf,fs -q
```

---

### `rspice info` — Netlist Information

Display netlist summary without running simulation.

```bash
rspice info <NETLIST> [OPTIONS]
```

**Options:**

| Flag | Description |
|------|-------------|
| `-d, --detailed` | Show detailed element information |
| `--models` | Show model definitions |
| `--hierarchy` | Show subcircuit hierarchy |
| `--json` | Output as JSON |

**Example:**

```bash
rspice info circuit.sp --detailed --json
```

---

### `rspice check` — Validate Netlist

Check netlist syntax and connectivity.

```bash
rspice check <NETLIST> [OPTIONS]
```

**Options:**

| Flag | Description |
|------|-------------|
| `--connectivity` | Check for floating nodes |
| `--models` | Check for undefined models |
| `--strict` | Treat warnings as errors |
| `--json` | Output as JSON |

**Example:**

```bash
rspice check circuit.sp --connectivity --strict
```

---

### `rspice compare` — Golden File Comparison

Compare simulation results against reference files for regression testing.

```bash
rspice compare <RESULT> <GOLDEN> [OPTIONS]
```

**Options:**

| Flag | Description |
|------|-------------|
| `--abstol <TOL>` | Absolute tolerance (default: 1e-9) |
| `--reltol <TOL>` | Relative tolerance (default: 1e-6) |
| `--variables <VAR>` | Compare specific variables only |
| `--fail-fast` | Stop on first difference |
| `--json` | Output differences as JSON |

**Example:**

```bash
# Compare with tolerances
rspice compare results.csv golden.csv --abstol 1e-9 --reltol 1e-6

# JSON output for CI parsing
rspice compare results.csv golden.csv --json
```

**Exit Codes:**
- `0` — Comparison passed
- `1` — Differences found

---

### `rspice compile-va` — Compile Verilog-A

Compile Verilog-A models for use in simulations.

```bash
rspice compile-va <FILE> [OPTIONS]
```

**Options:**

| Flag | Description |
|------|-------------|
| `-o, --output <FILE>` | Output compiled model |
| `-I, --include <DIR>` | Add include directory |
| `--strict` | Enable strict LRM compliance |
| `--show-usage` | Generate usage example |

---

### `rspice convert` — Format Conversion

Convert between simulation output formats.

```bash
rspice convert <INPUT> <OUTPUT> --to <FORMAT>
```

**Supported Formats:** `raw`, `ascii`, `csv`, `json`, `tsv`

**Example:**

```bash
rspice convert results.raw results.csv --to csv
```

---

## Global Options

| Flag | Description |
|------|-------------|
| `-v, --verbose` | Enable debug-level output |
| `-q, --quiet` | Suppress non-error output |
| `--config <FILE>` | Use specific config file |
| `--log-level <LEVEL>` | Set log level: `error`, `warn`, `info`, `debug`, `trace` |

---

## Configuration File

RSpice looks for configuration in `~/.rspicerc` or `./.rspicerc` (TOML format):

```toml
[simulation]
temperature = 27.0
abstol = 1e-12
reltol = 1e-6
max_iterations = 50

[output]
format = "raw"
node_names = true

[paths]
include_dirs = ["./models", "./lib"]
```

Environment variables override config file values:
- `RSPICE_TEMP` — Temperature
- `RSPICE_ABSTOL` — Absolute tolerance
- `RSPICE_INCLUDE` — Include paths (colon-separated)

---

## CI/CD Integration

### GitHub Actions

```yaml
jobs:
  analog-verification:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install RSpice
        run: cargo install --path crates/rspice-cli
      
      - name: Run Simulations
        run: rspice run circuits/amplifier.sp -q --report-format junit --report-file results.xml
      
      - name: Regression Test
        run: rspice compare output.csv golden/expected.csv --abstol 1e-9
      
      - name: Upload Results
        uses: actions/upload-artifact@v4
        with:
          name: simulation-results
          path: results.xml
```

### Jenkins

```groovy
pipeline {
    stages {
        stage('Simulate') {
            steps {
                sh 'rspice run circuit.sp -q --report-format junit --report-file results.xml'
            }
            post {
                always {
                    junit 'results.xml'
                }
            }
        }
    }
}
```

### GitLab CI

```yaml
simulate:
  script:
    - rspice run circuit.sp -q --report-format junit --report-file results.xml
  artifacts:
    reports:
      junit: results.xml
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Usage error (invalid arguments) |
| 65 | Input file error |
| 66 | Parse error |
| 70 | Simulation error |
| 73 | Output error |
| 78 | Configuration error |

---

## Examples

### Basic Workflows

```bash
# DC operating point
rspice run circuit.sp

# Transient with progress bar
rspice run circuit.sp --progress

# AC sweep with CSV output
rspice run circuit.sp -o bode.csv -f csv
```

### Advanced Analysis

```bash
# Monte Carlo (1000 runs)
rspice run circuit.sp --monte-carlo 1000 --seed 42 -v

# Compressed transient (long simulations)
rspice run power.sp --compress

# Measurements to JSON
rspice run circuit.sp --meas --meas-format json --meas-file meas.json
```

### Verification Pipeline

```bash
# Step 1: Validate netlist
rspice check circuit.sp --connectivity --strict

# Step 2: Run simulation
rspice run circuit.sp -q -o results.csv -f csv

# Step 3: Compare to golden
rspice compare results.csv golden.csv --abstol 1e-9

# Step 4: Extract measurements
rspice run circuit.sp --meas-format json --meas-file metrics.json
```

---

## License

RSpice CLI is part of the RSpice project and is licensed under the **RSpice Personal Use License**.
