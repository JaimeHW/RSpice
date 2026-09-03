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
- **Result export** — Touchstone v1, ngspice-compatible SPICE raw (ASCII and
  binary, real and complex), and RFC 4180 CSV
- **Netlist introspection** — walk elements, nodes, and `.PARAM` values, and
  derive parameter variants without string-editing the deck
- **Process-parallel** — netlists, configurations, and results pickle, so
  `multiprocessing` works on every supported interpreter
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

Requires CPython 3.10 or newer and `numpy>=2.0,<3`. Release wheels are built
against the stable ABI (abi3-py310), so one wheel per platform covers every
supported interpreter.

The Python package is private and is not published to PyPI. Install it from an
authorized source checkout or from a privately supplied build artifact. Its
use and distribution are governed by the same RSpice Personal Use License as
the rest of the repository.

### From Source

```bash
python -m pip install "maturin==1.15.0"

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

The workspace-aware source distribution requires one post-processing step. It
reconciles maturin's pruned workspace with the repository lockfile, and it
restores the files `rspice-core` embeds from the workspace root: maturin fills
the archive from `cargo package --list`, which never reports a file outside a
package directory, so without them the archive does not compile at all.

```bash
maturin sdist --out dist
python ../../tools/release/repair_sdist_lock.py dist/rspice-*.tar.gz
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

# Transient analysis (max_step defaults to the output window / 50)
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

`engine.run` executes the netlist's `.op`, `.dc`, `.tran`, `.ac`/`.ac data`,
`.disto`, `.hb`, `.sp`, `.noise`/`.noise data`, `.tf`, `.stb`, `.pz`, `.mc`,
`.step`, `.temp`, DC and AC `.sens`, and `.four` directives in deck order and
returns a `RunReport`:

- `report.all_op` / `report.all_dc` / `report.all_tran` / `report.all_ac` /
  `report.all_noise` — every result when a deck repeats a directive kind;
  the singular accessors below are the last of each
- `report.op` / `report.dc` / `report.tran` / `report.ac` /
  `report.distortion` / `report.hb` / `report.s_parameters` / `report.noise` /
  `report.tf` / `report.stb` / `report.pz` / `report.monte_carlo` /
  `report.step` / `report.temperature` / `report.sensitivity` /
  `report.sensitivity_ac` / `report.fourier` — the analysis results
- `report.measurements`, `report.measurement(name)`, `report.failures`,
  `report.all_passed` — `.MEAS` outcomes for TRAN, DC, AC, and NOISE analyses
  (`.MEAS AC` supports magnitude, dB, phase, real, and imaginary data;
  `.MEAS NOISE` supports `ONOISE`/`INOISE` spectral densities)
- `report.records` / `report.analyses_run` / `report.skipped` — at least one
  record per directive (`.four` adds one per output, `.sp donoise` adds a
  noise record); anything the engine could not execute is listed with
  `skipped=True` and a reason, never dropped silently
- Each record produced under a run axis carries `analysis_id` (for example,
  `ac-001` or `ac-002`) and a typed `coordinate` with its stable ID and axis
  assignments. An authored `.TEMP` wraps the deck's physical analyses at each
  temperature; it does not add an unrelated operating-point sweep.
- `report.assert_passed()` — raises `MeasurementError` unless every directive
  ran, at least one measurement was evaluated, and all of them passed, so
  neither a skipped analysis nor a skipped measurement can green-wash a
  pipeline

A directive that fails is recorded and the rest of the deck still runs, so
one unconverged sweep does not cost you the results and `.MEAS` outcomes of
everything else. Pass `continue_on_error=False` to abort on the first failure
and raise it instead:

```python
report = engine.run(netlist)                       # record failures, keep going
report = engine.run(netlist, continue_on_error=False)   # raise the first one
```

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

Supported `.MEAS` forms:

- Windowed statistics over `FROM=`/`TO=`: `MAX`, `MIN`, `PP`, `AVG`, `RMS`,
  `INTEG` (`INTEGRAL`)
- Point queries: `FIND ... AT=` / `FIND ... WHEN ...` (including
  `FIND TIME WHEN ...`), standalone `WHEN`, and `DERIV` (`DERIVATIVE`)
- `TRIG ... TARG ...` delay measurements
- Expressions over earlier results: `PARAM='expr'`, and `EQN` for Xyce
  continuous equation measures re-evaluated at every accepted point
- Waveform comparison: `ERR`/`ERR1` (RMS norm) and `ERR2` (mean-absolute
  norm) between two signals, and `ERROR ... FILE=` against a column of an
  external Xyce PRN, CSV, or CSDF table

Signals address node voltages (`V(out)`) and branch currents (`I(V1)`). For AC,
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

# Statements only: no title line, so V1 stays a source (netlist.num_elements == 2)
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

netlist.element_names, netlist.model_names, netlist.subcircuit_names
netlist.analyses, netlist.measurement_names, netlist.measurement_specs
netlist.title, netlist.is_global("vdd")
netlist.num_elements, netlist.num_models, netlist.num_subcircuits
netlist.num_analyses, netlist.num_measurements

# Structure: every instance, the authored node set, and .PARAM values
for element in netlist.elements:
    print(element.name, element.kind, element.nodes, element.value, element.model)
m1 = netlist.element("M1")                     # case-insensitive; KeyError if absent
m1.instance_params["W"]                        # resolved instance parameters
netlist.node_names                             # authored nodes, ground excluded
netlist.parameters                             # {'RVAL': 1000.0} - canonical names
netlist.parameter("rval")                      # case-insensitive lookup
netlist.source, netlist.source_path            # the deck this was parsed from
```

`Netlist.with_parameters` derives an independent netlist with different
`.PARAM` values. Top-level assignments are rewritten in place and the deck is
re-parsed, so the result does not depend on the redefinition policy in force.
Definitions inside a `.SUBCKT` are left alone, because those are scoped to the
subcircuit and share only a name:

```python
for rval in (1e3, 2e3, 5e3):
    corner = netlist.with_parameters({"rval": rval})
    print(rval, engine.run_dc_op(corner).voltage("out"))
```

`Netlist.parse` treats its input as statements: if the first non-blank line
is not a `*` comment, a synthetic title is prepended, so a malformed first
element raises `ParseError` instead of silently becoming the title. Use
`parse_spice` when you need classic first-line-title semantics.

Syntax that parsed but was downgraded or ignored surfaces as non-fatal
diagnostics rather than warnings on stderr. `netlist.diagnostics` carries
`line`/`severity`/`code`/`message` entries, and `netlist.startup_diagnostics`
adds structured `.IC`/`.NODESET` findings whose `code`, `stage`, `directive`,
`origins`, `scopes`, and `canonical_nodes` are stable API values, so a linting
job can branch on them without matching message text.

### Engine and Configuration

```python
engine = rspice.Engine()                      # defaults

limits = rspice.ResourceLimits(max_batch_runs=1_000, max_parallel_workers=8)
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
Its transient timestep ceiling is unbounded by default; set a finite
`max_timestep` when the embedding application requires one.

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

# The general .DC form: linear, list, decade/octave, and nested sweeps
vgs = rspice.DcSweep("VG", start=0, stop=1.8, step=0.05)
vds = rspice.DcSweep("VD", values=[0.5, 1.0, 1.8])
curves = engine.run_dc_sweep_spec(netlist, vgs, sweep2=vds)
curves.shape                                   # (outer_points, inner_points)
decade = engine.run_dc_sweep_spec(
    netlist, rspice.DcSweep("V1", 1, 1e3, mode="dec", points=10)
)

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

# Probes address node voltages, differential pairs, and branch currents
tran.signal("V(out)"), tran.signal("V(outp,outn)"), tran.signal("I(V1)")
tran.fourier("outp", 1e3, reference="outn")    # differential .FOUR
tran.fourier_current("V1", 1e3)                # branch-current .FOUR

# When the netlist contains .FFT, each directive executes with the transient
# and retains source/configuration metadata and calibrated complex bins.
fft = tran.fft(0)                               # same objects as tran.fft_results
fft.source, fft.window, fft.format, fft.mode
fft.physical_type, fft.value_unit               # e.g. ("voltage", "V")
fft.frequencies, fft.complex_bins               # float64 and complex128 arrays
# Normalized spectra retain physical_type provenance but use value_unit "1";
# an unnormalized parameter/expression has value_unit None.
# .OPTIONS FFT FFTOUT=1 additionally populates fft.metrics.
if fft.metrics is not None:
    print(fft.metrics.thd_ratio, fft.metrics.largest_harmonics)

# Long-run transient storage and continuation
compressed = engine.run_tran_compressed(netlist, stop_time=1.0,
                                        abs_tol=1e-6, rel_tol=1e-3)
# The retained grid carries its exact step sizes plus every analog family.
compressed.branch_current_waveform("V1")
compressed.device_parameter_waveform("M1", "gm")
compressed.store_waveform("YMEMRISTOR!MR1:R")
# Channels are keyed by descriptor: canonical name, unit and availability.
compressed.channel_names            # ["v(out)", "i(v1)", "@m1[gm]", ...]
compressed.channel_unit("v(out)")   # "volt"
# A sample the run could not record is absent with a reason, never a number.
compressed.channel_absence("v(out)")  # [None, None, "non-finite", ...]
# Event traces are never decimated, and post-process products are computed on
# the exact accepted trajectory rather than on the retained grid.
compressed.digital_trace("d")
compressed.real_trace("rnode")
compressed.measurements
compressed.fourier_results
compressed.fft_results
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
# Two-port stability and gain (None for any other port count)
print(sparams.k_factor, sparams.mu_factor, abs(sparams.delta))
print(sparams.unconditionally_stable, sparams.max_available_gain_db)
circles = sparams.stability_circles(0)         # centres, radii, stable side
sparams.write_touchstone("dut.s2p", format="ma")
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

# Solve the periodic operating point once and reuse it: PAC and PNoise each
# linearize around a PSS solution, which otherwise dominates an RF sweep.
op = engine.run_pss_operating_point(netlist, 1e9)
pac = engine.run_pac(netlist, 1e9, 1e3, 100e6, 20, "VRF", "out", pss=op)
pnoise = engine.run_pnoise(netlist, 1e9, [1e3, 10e3], "out", pss=op)

# Continue a transient from a converged orbit or an HB envelope, skipping
# the settling interval a cold start has to integrate through.
pss, state = engine.run_pss_continuation(netlist, 1e9)
tran, checkpoint = engine.run_tran_from_pss(netlist, state, duration=1e-6)
hb, envelope = engine.run_hb_envelope(netlist, 1e9, frozen_sources=["VMOD"])
tran, checkpoint = engine.run_tran_from_hb_envelope(netlist, envelope, 1e-6)
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

Node arguments accept a name or a node index wherever the signature is typed
`int | str` — every result accessor, plus `run_noise`, `run_pz`, and the
sensitivity runners. The RF and periodic entry points (`run_pac`,
`run_pnoise`, `run_transfer_function`, `run_stb`) and the `HbResult` /
`PacResult` accessors take names only.

Autonomous oscillator noise uses the same complete device-noise model as
stationary noise analysis, including resistor noise switches and temperature
offsets, semiconductor thermal/shot/flicker noise, tabulated and Verilog-A
sources, and correlated BSIM4 thermal noise.

## Exporting Results

Every result serializes to the formats the rest of an EDA flow reads, either
as an in-memory string for a CI job or straight to disk:

```python
# Touchstone v1 for scikit-rf, ADS, or a datasheet plot
sparams.write_touchstone(f"dut.{sparams.touchstone_extension}", format="ma")
text = sparams.to_touchstone(frequency_unit="ghz", comments=["nominal corner"])

# ngspice-compatible raw files; AC is written with Flags: complex
tran.write_raw("run.raw", format="binary")
ac.write_raw("ac.raw")                          # or ac.to_raw() -> bytes

# RFC 4180 CSV; AC splits each phasor into <name>_real / <name>_imag
tran.write_csv("run.csv")
print(sweep.to_csv())
tran.export_columns                             # column order, in advance
```

Touchstone v1 carries a single reference impedance. A sweep whose ports do not
all share one is refused rather than written with an `R` that misdescribes it;
renormalize the ports first.

Native result-file publication (`write_csv`, `write_raw`, and
`write_touchstone`) is transactional: RSpice stages and synchronizes the
complete file beside its destination before atomically replacing it. A
serialization, flush, or commit failure therefore preserves the previous
complete artifact, or leaves an absent destination absent.

## Parallelism

Simulation calls release the GIL, so threads work directly. For process-based
parallelism, netlists, configurations, and results all pickle:

```python
from concurrent.futures import ProcessPoolExecutor

def corner(rval):
    netlist = BASE.with_parameters({"rval": rval})
    return rval, rspice.Engine().run_dc_op(netlist).voltage("out")

with ProcessPoolExecutor() as pool:
    for rval, v_out in pool.map(corner, [1e3, 2e3, 5e3, 10e3]):
        print(rval, v_out)
```

A `Netlist` pickles by replaying its parse from the deck text it retains, so
the payload stays the size of the source rather than the whole AST, and the
`ResourceLimits` it was parsed under are reapplied.

Every result type pickles, including the periodic and RF families. A result
carries the state behind everything its own accessors expose, so each readable
quantity — and each quantity derived from one, such as `PssResult.thd_percent`
or `HbResult.is_valid` — is unchanged across a round trip. Internal traces with
no accessor on the class holding them are not carried; `PacResult` MNA branch
currents currently fall in that group. Compressed transient pickles preserve
retained step sizes, every descriptor-keyed channel with its unit, owner and
per-sample validity mask, the XSPICE digital and real event traces, the parent
analysis/coordinate/topology identity, and the typed `.FFT`, `.FOUR` and
`.MEASURE` post-results. A pickle whose sample is neither a number nor a typed
absence, or whose channel role, unit or absence reason this build does not
know, is rejected rather than repaired.
Transient FFT state is explicitly versioned and is identical in full and
compressed transient pickles. Legacy transient pickles from bindings that
discarded FFT products are rejected because they cannot prove whether an empty
FFT list is genuine; rerun and repickle those analyses with the current schema.

## Error Handling

All errors derive from `rspice.RSpiceError`:

```text
RSpiceError
├── ParseError           # netlist syntax/semantic errors
├── SimulationError      # circuit or solver failure
│   ├── ConvergenceError # Newton-Raphson failed to converge
│   └── CancelledError   # Engine.cancel() stopped the active call
├── MeasurementError     # RunReport.assert_passed() failures
├── RSpiceKeyError       # also a KeyError   - unknown node/branch/device
├── RSpiceIndexError     # also an IndexError - out-of-range result index
├── RSpiceValueError     # also a ValueError  - invalid argument value
└── RSpiceTypeError      # also a TypeError   - invalid argument type
```

The last four derive from both `RSpiceError` and the builtin exception a
caller already expects, so `except rspice.RSpiceError` catches everything the
library raises while `except KeyError` keeps working unchanged.

```python
try:
    result = engine.run_dc_op(netlist)
except rspice.ConvergenceError:
    engine = rspice.Engine(robust_config)      # retry with stronger aids
except rspice.RSpiceError as e:
    print(f"simulation failed: {e}")
```

Both error families carry structured attributes so automation never has to
parse a display message. `SimulationError` (and its subclasses) expose the
stable snake-case `kind`, `code`, and `category` tags, a conservative
`retryable` flag, `iterations` for convergence failures, and
`resource`/`requested`/`limit` when a `ResourceLimits` ceiling was hit.
`ParseError` adds `kind`, `category`, and the source provenance for the
failure — `line`/`source`, the `primary_*`/`related_*` pair for two-location
errors, and error-specific fields such as `unresolved_output_symbols`. The
`.pyi` stub declares the full set.

```python
try:
    report = engine.run(netlist)
except rspice.SimulationError as e:
    if e.code == "resource_limit":
        print(f"{e.resource}: requested {e.requested}, limit {e.limit}")
    elif e.retryable:
        report = engine.run(netlist)
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
python -m pip install "maturin==1.15.0" "numpy>=2.0,<3" "pytest==9.1.1"
maturin develop --release --locked
python -m pytest tests/ -v
```

There are also Rust-side unit tests covering the interpreter-free logic
(signal-spec parsing, the export formats, error mapping). A bare
`cargo test -p rspice-python` runs nothing, because a cdylib cannot link into
a test harness; they need the lib target named explicitly:

```bash
cargo test -p rspice-python --lib
```

That is a complement to the pytest suite, not a substitute for it.

## License

RSpice Python is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).
