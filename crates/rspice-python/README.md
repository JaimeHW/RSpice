# RSpice Python Bindings

Python bindings for the RSpice circuit simulation engine, built for
automation, scripting, and automated circuit verification — running SPICE
regression tests in CI the same way you run unit tests.

## Features

- **Simulation API** — DC, AC/AC-DATA, third-order Volterra distortion,
  transient, noise, pole-zero, STB, N-port S-parameters with complex `Cy`
  noise correlation and two-port `Rn`/`NF`/`NFmin`/`Sopt`, PSS, HB, PAC,
  driven PNoise, oscillator phase noise, Monte Carlo, sensitivity, transfer
  function, Fourier/THD, and parametric analysis
- **Long-run controls** — resumable netlist-fingerprinted transient
  checkpoints and error-bounded compressed voltage waveforms
- **Verification first** — `engine.run(netlist)` executes the netlist's own
  analysis directives, evaluates `.MEAS` statements, and
  `report.assert_passed()` turns them into a CI gate
- **NumPy integration** — waveforms, spectra, and complex AC phasors as
  ndarrays
- **Releases the GIL** — long simulations run with the GIL released; other
  Python threads stay live, and engines can simulate in parallel threads
- **Ctrl-C works** — long-running DC, AC/RF, transient, noise, statistical,
  sensitivity, and periodic analyses are cooperatively cancellable with
  `KeyboardInterrupt` instead of blocking until completion
- **Application cancellation** — `engine.cancel()` safely stops every active
  call on that Engine from another Python thread and raises `CancelledError`
  in each caller
- **Readiness probe** — `engine.health_check()` exercises the configured
  parser-to-solver path without filesystem or network I/O
- **Strict error discipline** — accessors raise `IndexError`/`KeyError` for
  invalid nodes; argument errors raise `ValueError`; nothing fabricates
  silent zeros
- **Resource governance** — one `ResourceLimits` policy bounds netlist and
  dependency ingestion, hierarchy/circuit growth, analysis/result sizes,
  batch runs, external data, and shared caches across parsing and execution
- **Typed** — ships a complete `.pyi` stub with a `py.typed` marker

## Installation

The Python package is private and is not published to PyPI. Install it from an
authorized source checkout or from a privately supplied build artifact. Its
use and distribution are governed by the same RSpice Personal Use License as
the rest of the repository.

### From Source

```bash
python -m pip install "maturin==1.14.1"

cd crates/rspice-python
maturin develop --release --locked
```

`maturin build --release --locked` produces an installable abi3 wheel for
authorized testing and private delivery; it does not grant redistribution
rights. CI builds manylinux2014 x86-64/AArch64, macOS Intel/Apple-Silicon, and
Windows x86-64/ARM64 wheels as private workflow artifacts and never uploads
them to PyPI. The same six targets also produce version-specific `cp314t`
wheels for free-threaded CPython 3.14; the full binding suite runs with the GIL
disabled on Linux, macOS, and Windows.

The workspace-aware source distribution requires one post-processing step to
reconcile maturin's pruned workspace with the repository lockfile:

```bash
maturin sdist --out dist
python scripts/repair_sdist_lock.py dist/rspice-*.tar.gz
```

The repair runs offline by default, rejects any new or changed external Cargo
package identity, source, or checksum, and verifies the resulting archive with
Cargo's `--locked` mode. Run `cargo fetch --locked` first on a clean build
machine so the dependency index and sources are present locally.

## Quick Start

```python
import rspice

netlist = rspice.Netlist.parse("""
* Voltage divider
V1 in 0 10
R1 in out 1k
R2 out 0 1k
.end
""")

engine = rspice.Engine()

# DC operating point
result = engine.run_dc_op(netlist)
print(f"V(out) = {result.voltage('out'):.3f} V")     # 5.000 V

# Transient analysis (max_step defaults to stop_time / 50)
tran = engine.run_tran(netlist, stop_time=1e-3)
time = tran.time                                # NumPy array
v_out = tran.voltage_waveform("out")            # NumPy array
i_v1 = tran.branch_current_waveform("V1")       # branch currents too
```

## Automated Verification (analog CI)

Put the pass/fail criteria in the netlist as `.MEAS` statements, run the
deck, and assert on the report — from pytest, a script, or a CI job:

```python
import pytest
import rspice

DECK = """* RC step response regression
V1 in 0 PULSE(0 1 0 1n 1n 1 2)
R1 in out 1k
C1 out 0 100n
.tran 1u 1m
.meas tran t_half FIND TIME WHEN V(out)=0.5
.meas tran v_final MAX V(out)
.meas tran trise TRIG V(out) VAL=0.1 RISE=1 TARG V(out) VAL=0.9 RISE=1
.end
"""

def test_rc_step_response():
    report = rspice.Engine().run(rspice.Netlist.parse(DECK))
    report.assert_passed()                       # raises MeasurementError on failure
    assert report.measurement("trise").value == pytest.approx(219.7e-6, rel=0.02)
```

`engine.run` executes the netlist's `.op`, `.dc`, `.ac`/`.ac data`, `.disto`,
`.hb`, `.sp`, `.tran`, `.noise`, `.tf`, `.stb`, `.pz`, `.mc`, `.step`, `.temp`, DC
and AC `.sens`, and `.four` directives in order and returns a `RunReport`:

- `report.tran` / `report.ac` / `report.hb` / `report.distortion` / `report.op` /
  `report.dc` / `report.noise` / `report.s_parameters` / `report.tf` /
  `report.stb` / `report.pz` / `report.sensitivity` /
  `report.sensitivity_ac` / `report.fourier` — the analysis results
- `report.measurements`, `report.measurement(name)`, `report.failures`,
  `report.all_passed` — `.MEAS` outcomes for TRAN, DC, AC, and NOISE analyses
  (`.MEAS AC` supports magnitude, dB, phase, real, and imaginary data;
  `.MEAS NOISE` supports `ONOISE`/`INOISE` spectral densities)
- `report.records` — one record per directive; anything the engine could not
  execute is listed with `skipped=True` and a reason, never dropped silently
- `report.assert_passed()` — raises `MeasurementError` unless at least one
  measurement ran and all of them passed, so a deck whose measurements were
  skipped cannot green-wash a pipeline

Measurements can also run against transient and DC-sweep results you already
have:

```python
tran = engine.run_tran(netlist, stop_time=1e-3)
for m in engine.measure(netlist, tran):
    print(m.name, m.value, m.passed)
```

`Engine.run(...)` is the automated-verification entry point and evaluates
matching TRAN, DC, AC, and NOISE `.MEAS` statements after it executes the deck.
Standalone `Engine.measure(...)` is intentionally narrower today: it accepts
`TransientResult` and `DcSweepResult` only.

Supported `.MEAS` forms: `MAX`, `MIN`, `AVG`, `RMS`, `PP`, `INTEG`
(`FROM=`/`TO=` windows), `FIND ... AT=` / `FIND ... WHEN ...` (including
`FIND TIME WHEN ...`), and `TRIG ... TARG ...` delay measurements. Signals
address node voltages (`V(out)`) and branch currents (`I(V1)`). For AC,
plain `V(out)` / `VM(out)` measure magnitude; `VDB(out)` measures dB
magnitude, `VP(out)` phase in degrees, `VR(out)` real, and `VI(out)`
imaginary. The AC sweep axis is available as `TIME`, `FREQUENCY`, or `FREQ`;
branch currents use the same `I*` variants. For NOISE, measure `ONOISE` or
`INOISE` (also `ONOISE_SPECTRUM` / `INOISE_SPECTRUM`) against the noise
frequency axis.

## API Overview

### Netlist

```python
import pathlib
import numpy as np
import rspice

netlist = rspice.Netlist.parse("V1 1 0 10\nR1 1 0 1k\n.end")

# Raw SPICE deck: first line is always the title
netlist = rspice.Netlist.parse_spice("My Amplifier\nV1 1 0 10\n.end")

# From a file, expanding .include/.lib relative to its location;
# accepts str or os.PathLike
limits = rspice.ResourceLimits(max_netlist_bytes=8 * 1024 * 1024)
netlist = rspice.Netlist.parse_file(
    pathlib.Path("circuits") / "amplifier.sp", resource_limits=limits
)

# From a string, resolving includes against a directory
netlist = rspice.Netlist.parse_with_includes(content, "circuits/")

netlist.element_names, netlist.model_names, netlist.analyses
netlist.measurement_names, netlist.title
```

`Netlist.parse` treats its input as statements: if the first non-blank line
is not a `*` comment, a synthetic title is prepended, so a malformed first
element raises `ParseError` instead of silently becoming the title. Use
`parse_spice` when you need classic first-line-title semantics.

### Engine and Configuration

```python
engine = rspice.Engine()                      # defaults

limits = rspice.ResourceLimits(max_batch_runs=1_000)
config = rspice.SimulationConfig(
    tolerance=1e-12,
    temperature=300.15,                       # Kelvin
    integration_method=rspice.IntegrationMethod.GEAR2,
    convergence=rspice.ConvergenceConfig.robust(),
    bypass=rspice.BypassConfig(enabled=True),
    resource_limits=limits,
)
engine = rspice.Engine(config)
```

All configuration classes take keyword arguments. Property getters for
nested configs return *copies*: `config.convergence.verbose = True` and
`config.resource_limits.max_batch_runs = 10` modify temporaries and are lost —
assign the whole nested object back, or build with keywords.

`ConvergenceConfig` exposes the DC convergence aids (GMIN stepping, source
stepping, pseudo-transient, arc-length continuation, damping strategies,
tolerance knobs including `charge_abstol`); `BypassConfig` controls
latent-device bypass; `SimulationConfig` adds `transient_max_iterations`
(ITL4), `transient_trtol`, the integration method, and `ResourceLimits`.

### Analyses

```python
# DC operating point
op = engine.run_dc_op(netlist)
v = op.voltage("out")                          # by name or index
i = op.branch_current("V1")

# DC sweep — iterable, indexable
sweep = engine.run_dc_sweep(netlist, "V1", 0, 5, 0.1)
for v_in, sol in sweep:
    print(f"{v_in:.1f} V -> {sol.voltage('out'):.3f} V")
v_curve = sweep.voltage_array("out")           # NumPy array across the sweep

# AC analysis — explicit frequencies or dec/oct/lin sweeps
ac = engine.run_ac(netlist, np.logspace(0, 6, 121))
ac = engine.run_ac_sweep(netlist, "dec", 20, 1.0, 1e6)
gain_db = ac.voltage_db("out")                 # 20*log10|V|, NumPy array
phase = ac.voltage_phase_degrees("out")
h = ac.voltage_complex("out")                  # complex128 ndarray
i_in = ac.branch_current_complex("V1")         # complex branch currents

# Third-order Volterra distortion. Sources use DISTOF1 and optional DISTOF2.
distortion_netlist = rspice.Netlist.parse("""
* Biased nonlinear divider
V1 in 0 DC 0.5 DISTOF1 1m 0 DISTOF2 1m 0
R1 in out 100
D1 out 0 DM
.model DM D(IS=1e-12 N=1 CJO=1p)
.end
""")
dist = engine.run_distortion(distortion_netlist, np.logspace(3, 6, 31))
hd2 = dist.product("2f1")                     # AcResult at physical 2F1
hd3_dbc = dist.voltage_db_relative("3f1", "out")

# Two-tone mode: F2 is fixed at 0.9 * the first F1 while F1 is swept.
imd = engine.run_distortion_sweep(
    distortion_netlist, "dec", 10, 1e6, 100e6, f2_over_f1=0.9
)
im3 = imd.product("2f1-f2").voltage_complex("out")

# Transient — Ctrl-C cancellable
tran = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)
four = tran.fourier("out", fundamental=1e3)    # harmonics + THD
print(f"THD = {four.thd_percent:.2f}%")

# Long-run transient storage and continuation
compressed = engine.run_tran_compressed(netlist, stop_time=1.0,
                                        abs_tol=1e-6, rel_tol=1e-3)
segment, checkpoint = engine.run_tran_checkpointed(netlist, stop_time=0.5)
checkpoint.save("run.chk")
continued, checkpoint = engine.resume_tran(netlist, checkpoint, stop_time=1.0)

# Noise (temperature defaults to the engine configuration)
for r in engine.run_noise(netlist, "out", [1e3, 1e4]):
    print(f"{r.frequency:.0f} Hz: {r.output_noise_rms*1e9:.2f} nV/sqrt(Hz)")
    print(f"  dominant: {r.dominant_source()}")

# Pole-zero (input is a unit current: dc_gain is a transimpedance)
pz = engine.run_pz(netlist, "in", "out")
print(pz.is_stable, pz.dominant_pole_decay_hz, pz.poles_array)

# RF and periodic analyses
sparams = engine.run_s_parameters(netlist, np.logspace(6, 10, 101))
s21_db = sparams.magnitude_db(2, 1)            # one-based engineering ports
spnoise = engine.run_s_parameters(netlist, np.logspace(6, 10, 101),
                                  do_noise=True)
cy12 = spnoise.cy(1, 2)                        # complex A²/Hz correlation
print(spnoise.rn, spnoise.nf, spnoise.nfmin, spnoise.sopt)
pss = engine.run_pss(
    netlist,
    fundamental_frequency=1e9,
    harmonics=15,
    abstol=1e-13,
    integration_method=rspice.IntegrationMethod.TRAPEZOIDAL,
)
pss_spectrum = pss.harmonic_coefficients("out")
hb = engine.run_hb(netlist, fundamental_frequency=1e9, harmonics=9)
hb2 = engine.run_hb_multitone(
    mixer_netlist,
    [900e6, 800e6],
    harmonics=[5, 5],
    source_names=["VRF", "VLO"],
)
pac = engine.run_pac(netlist, 1e9, 1e3, 100e6, 20, "VRF", "out")
pnoise = engine.run_pnoise(netlist, 1e9, [1e3, 10e3], "out")
osc_noise = engine.run_oscillator_noise(
    oscillator_netlist,
    [1e3, 10e3, 100e3],
    period_guess=1e-9,
)

# Transfer function (.TF): gain, input and output impedance
tf = engine.run_transfer_function(netlist, "out", "V1")
print(f"Av={tf.gain:.3f}  Zin={tf.input_impedance:.0f}  Zout={tf.output_impedance:.0f}")

# Monte Carlo over .param values bound via {...}
mc = engine.run_monte_carlo(netlist, num_runs=1000, seed=42,
                            distribution="gaussian", spread=0.01)
stats = mc.get_variable("V(OUT)")
print(f"{stats.mean:.4f} +/- {stats.std_dev:.4f}  (p99 = {stats.percentile(99):.4f})")

# Sensitivity and parametric step vary a .param referenced via {...}
divider = rspice.Netlist.parse("""
* Parametric divider
.param rval=1k
V1 in 0 10
R1 in out {rval}
R2 out 0 1k
.end
""")
s = engine.run_sensitivity(divider, "out", "rval", 1e3)
s_dc = engine.run_sensitivity_dc_complete(divider, "out", filters=["R*"])
s_ac = engine.run_sensitivity_ac(divider, "out", "rval", 1e3, [1e3, 1e4])
complete = engine.run_sensitivity_ac_complete(
    divider,
    "out",
    [1e3, 1e4],
    filters=["R*", "RMOD:*"],
)
dr1 = complete.get("R1")
print(dr1.absolute, dr1.normalized, dr1.magnitude, dr1.phase, dr1.db)
for value, sol in engine.run_step(divider, "rval", [1e3, 2e3, 5e3]):
    print(value, sol.voltage("out"))
```

Node arguments accept names or indices everywhere, including the advanced
analyses.

Autonomous oscillator noise uses the same complete device-noise model as
stationary noise analysis, including resistor noise switches and temperature
offsets, semiconductor thermal/shot/flicker noise, tabulated and Verilog-A
sources, and correlated BSIM4 thermal noise.

## Error Handling

All errors derive from `rspice.RSpiceError`:

```text
RSpiceError
├── ParseError           # netlist syntax/semantic errors
├── SimulationError      # circuit or solver failure
│   ├── ConvergenceError # Newton-Raphson failed to converge
│   └── CancelledError   # Engine.cancel() stopped the active call
└── MeasurementError     # RunReport.assert_passed() failures
```

```python
try:
    result = engine.run_dc_op(netlist)
except rspice.ConvergenceError:
    engine = rspice.Engine(robust_config)      # retry with stronger aids
except rspice.RSpiceError as e:
    print(f"simulation failed: {e}")
```

Result accessors raise standard Python exceptions: `IndexError` for
out-of-range node indices, `KeyError` for unknown node or branch names —
in every result type, including AC. Invalid arguments (empty frequency
lists, non-positive stop times, zero sweep steps) raise `ValueError` before
the simulation starts.

## Threading and Cancellation

Simulation calls release the GIL, so a long transient can run in a worker
thread while the main thread stays responsive, and several engines can
simulate different netlists in parallel threads.

RSpice also supports free-threaded CPython 3.14. Dedicated `cp314t` wheels keep
the GIL disabled, and immutable `Netlist`, `Engine`, and result objects may be
shared across Python threads.

Applications do not need to synthesize operating-system signals. A GUI or
service thread may inspect `engine.is_running` / `engine.active_run_count` and
call `engine.cancel()`. The method returns the number of active calls signalled;
those calling threads receive `rspice.CancelledError`, and later calls on the
same Engine remain usable. When exactly one analysis is active,
`engine.progress` returns its completed fraction when that solver provides one;
it is `None` for idle Engines, concurrent calls, or analyses without a
meaningful progress scale.

DC operating points and sweeps, AC, distortion, and S-parameter sweeps,
transfer-function, STB, pole-zero, transient and checkpoint/resume runs,
noise, Monte Carlo, parameter steps, sensitivity, PSS, HB, PAC, driven PNoise, and
oscillator-noise calls poll Python signal handlers while they run. Ctrl-C
(`KeyboardInterrupt`) cancels these simulations instead of arriving only
after a completed result is returned.

## Testing

For routine Rust workspace checks, keep PyO3 and the wasm target out of the
fast path:

```bash
cargo check --workspace --exclude rspice-python --exclude rspice-wasm
```

Check the Python binding crate separately after selecting a real Python
interpreter:

```bash
cargo check -p rspice-python
```

On Windows, the Microsoft Store `python`/`python3` aliases can point PyO3 at
stub executables under `WindowsApps` and break local checks. `PYO3_PYTHON` is
the reliable override:

```powershell
py -3 -m venv .venv
$env:PYO3_PYTHON = (Resolve-Path .\.venv\Scripts\python.exe).Path
cargo check -p rspice-python
```

The binding test suite lives in `tests/` and should run through a maturin
development install:

```bash
cd crates/rspice-python
python -m pip install "maturin==1.14.1" "numpy>=2.0,<3" "pytest==9.1.1"
maturin develop --release --locked
python -m pytest tests/ -v
```

`cargo test -p rspice-python --lib` is not the recommended binding test: it
does not replace importing the built extension from pytest.

## License

RSpice Python is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).
