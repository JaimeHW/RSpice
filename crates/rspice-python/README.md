# rspice-python

Python bindings for the RSpice circuit simulation engine, built for
automation, scripting, and automated circuit verification: running SPICE
regression tests in CI the same way you run unit tests.

These bindings put RSpice inside your Python. The desktop IDE has the reverse,
Python automation inside RSpice, and the two are separate.

## What is here

- **Every analysis the engine has**: DC, AC and AC-DATA, third-order Volterra
  distortion, transient, noise, pole-zero, STB, N-port S-parameters with
  complex `Cy` noise correlation and two-port `Rn`/`NF`/`NFmin`/`Sopt`, PSS,
  HB, PAC, PXF, envelope, driven PNoise, oscillator phase noise, Monte Carlo,
  sensitivity, transfer function, Fourier/THD, and parametric analysis.
- **Verification first**: `engine.run(netlist)` executes the netlist's own
  analysis directives, evaluates `.MEAS` statements, and
  `report.assert_passed()` turns them into a CI gate.
- **NumPy throughout**: waveforms, spectra, and complex AC phasors as ndarrays.
- **Runtime Verilog-A**: default builds compile `.va` models through the core
  native backend. AC results retain the solved frequency prefix when a model
  requests `$finish`, and AC-DATA applies each row's parameter values.
- **Long-run controls**: resumable netlist-fingerprinted transient checkpoints
  and error-bounded compressed voltage waveforms.
- **Export**: Touchstone v1, ngspice-compatible SPICE raw (ASCII and binary,
  real and complex), RFC 4180 CSV, and VCD.
- **Netlist introspection**: walk elements, nodes, and `.PARAM` values, and
  derive parameter variants without string-editing the deck.
- **Cancellable and thread-friendly**: simulation calls release the GIL, Ctrl-C
  raises `KeyboardInterrupt` instead of blocking, and `engine.cancel()` stops
  every active call on an Engine from another thread.
- **Strict error discipline**: accessors raise `IndexError`/`KeyError` for
  invalid nodes, argument errors raise `ValueError`, and nothing fabricates a
  silent zero.
- **Resource governance**: one `ResourceLimits` policy bounds netlist and
  dependency ingestion, hierarchy and circuit growth, analysis and result
  sizes, batch runs, external data, and shared caches, across parsing and
  execution alike.
- **Typed**: a complete `rspice.pyi` stub, installed with a `py.typed` marker.

The full API is in that stub and in the docstrings, which carry more than two
hundred runnable examples. This README covers what `help()` cannot: how to use
the bindings for verification, and the contracts that are not visible from a
signature.

## Installation

Requires CPython 3.10 or newer and `numpy>=2.0,<3`. Release wheels are built
against the stable ABI (abi3-py310), so one wheel per platform covers every
supported interpreter.

The Python package is private and is not published to PyPI. Install it from an
authorized source checkout or from a privately supplied build artifact. Its use
and distribution are governed by the same RSpice Personal Use License as the
rest of the repository.

```bash
python -m pip install "maturin==1.15.0"

cd crates/rspice-python
maturin develop --release --locked
```

The default `veriloga-native` Cargo feature enables runtime Verilog-A model
compilation. `--no-default-features` builds the bindings without that backend.

`maturin build --release --locked` produces an installable abi3 wheel for
authorized testing and private delivery; it does not grant redistribution
rights. CI builds manylinux2014 x86-64/AArch64, macOS Intel/Apple-Silicon, and
Windows x86-64/ARM64 wheels as private workflow artifacts. The same six targets
also produce version-specific `cp314t` wheels for free-threaded CPython 3.14;
the full binding suite runs with the GIL disabled on Linux, macOS, and Windows.

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

## Quick start

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

result = engine.run_dc_op(netlist)
print(f"V(out) = {result.voltage('out'):.3f} V")

# max_step defaults to the output window / 50
tran = engine.run_tran(netlist, stop_time=1e-3)
time = tran.time                                # NumPy array
v_out = tran.voltage_waveform("out")            # NumPy array
i_v1 = tran.branch_current_waveform("V1")       # branch currents too
```

`Netlist.parse` takes statements only: no title line, so nothing is swallowed.
`Netlist.parse_spice` takes a raw deck, whose first line is always the title.

## Automated verification

Put the pass/fail criteria in the netlist as `.MEAS` statements, run the deck,
and assert on the report, from pytest, a script, or a CI job:

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

`engine.run` executes every analysis directive the deck contains, in deck
order, and returns a `RunReport`. What matters about that report is not the
accessor list (the stub has it) but its discipline:

- **Nothing is dropped silently.** `report.records` holds at least one record
  per directive; anything the engine could not execute is listed with
  `skipped=True` and a reason. `.four` adds one record per output, and
  `.sp donoise` adds a noise record.
- **Repeated directives are all kept.** The `all_*` accessors return every
  result of a kind; the singular accessors are the last of each.
- **Different quantities get different fields.** A `.pnoise` card around a
  driven carrier reports an output power spectral density
  (`report.pnoise`); the same card around an autonomous `.pss` carrier
  reports a carrier-normalized dBc/Hz phase-noise spectrum with the
  phase-diffusion constant beside it (`report.oscillator_noise`). One field
  with a mode flag would hide that.
- **Run axes are identified, not flattened.** Each record produced under a run
  axis carries `analysis_id` (`ac-001`, `ac-002`) and a typed `coordinate`
  with its stable ID and axis assignments. An authored `.TEMP` wraps the deck's
  physical analyses at each temperature; it does not add an unrelated
  operating-point sweep.
- **`report.assert_passed()` cannot be green-washed.** It raises
  `MeasurementError` unless every directive ran, at least one measurement was
  evaluated, and all of them passed, so neither a skipped analysis nor a
  skipped measurement passes as success.

A directive that fails is recorded and the rest of the deck still runs, so one
unconverged sweep does not cost you the results and `.MEAS` outcomes of
everything else. Pass `continue_on_error=False` to abort on the first failure
and raise it instead.

Measurements can also run against results you already have, though
`Engine.measure` is intentionally narrower than `Engine.run`: it accepts
`TransientResult` and `DcSweepResult` only, and raises `TypeError` for
anything else.

```python
tran = engine.run_tran(netlist, stop_time=1e-3)
for m in engine.measure(netlist, tran):
    print(m.name, m.value, m.passed)
```

### Supported `.MEAS` forms

- Windowed statistics over `FROM=`/`TO=`: `MAX`, `MIN`, `PP`, `AVG`, `RMS`,
  `INTEG` (`INTEGRAL`)
- Point queries: `FIND ... AT=` / `FIND ... WHEN ...` (including
  `FIND TIME WHEN ...`), standalone `WHEN`, and `DERIV` (`DERIVATIVE`)
- `TRIG ... TARG ...` delay measurements
- Expressions over earlier results: `PARAM='expr'`, and `EQN` for Xyce
  continuous equation measures re-evaluated at every accepted point
- Waveform comparison: `ERR`/`ERR1` (RMS norm) and `ERR2` (mean-absolute norm)
  between two signals, and `ERROR ... FILE=` against a column of an external
  Xyce PRN, CSV, or CSDF table

Signals address node voltages (`V(out)`) and branch currents (`I(V1)`). For AC,
plain `V(out)` and `VM(out)` measure magnitude; `VDB(out)` measures dB
magnitude, `VP(out)` phase in degrees, `VR(out)` real, and `VI(out)` imaginary.
The AC sweep axis is available as `TIME`, `FREQUENCY`, or `FREQ`; branch
currents use the same `I*` variants. For NOISE, measure `ONOISE` or `INOISE`
(also `ONOISE_SPECTRUM` / `INOISE_SPECTRUM`) against the noise frequency axis.

## The shared result document

Beyond the flat accessors, every result family exposes `signals()`,
`scalars()`, `device_observables()`, and `document()`: the same
`rspice-analysis-result` document the CLI, the browser, and the cloud
engine-adapter publish.

The difference is what they can say. A flat accessor has to return a float;
`scalars()` reports a quantity the analysis proved has no finite value, such as
the gain margin of a loop whose phase never reaches −180°, as `value is None`
with an `unavailable_reason`. A signal descriptor carries canonical name, kind,
unit, owner, and an `availability` tag (`available`, `not_projected`,
`absent_at_coordinate`), so a channel the authored output projection did not
retain is distinguishable from one that does not exist. `document()` names the
authored card the result came from and, for a `.STEP` or `.TEMP` run, the
coordinate it was solved at.

A result restored from `pickle` carries this binding's own projection rather
than the core result these are built from, and says so with
`RSpiceNotImplementedError`. Pickle `result.document()` when the document
itself has to survive a round trip.

## Exporting results

```python
# Touchstone v1 for scikit-rf, ADS, or a datasheet plot
sparams.write_touchstone(f"dut.{sparams.touchstone_extension}", format="ma")
text = sparams.to_touchstone(frequency_unit="ghz", comments=["nominal corner"])

# ngspice-compatible raw files; AC is written with Flags: complex
tran.write_raw("run.raw", format="binary")
ac.write_raw("ac.raw")                          # or ac.to_raw() -> bytes

# Value Change Dump of the event timelines, for a logic viewer. The same bytes
# `rspice run -f vcd` publishes; VCD keeps four bit states and no strength.
tran.write_vcd("run.vcd")                       # or tran.to_vcd() -> str

# RFC 4180 CSV; AC splits each phasor into <name>_real / <name>_imag
tran.write_csv("run.csv")
print(sweep.to_csv())
tran.export_columns                             # column order, in advance
```

Touchstone v1 carries a single reference impedance. A sweep whose ports do not
all share one is refused rather than written with an `R` that misdescribes it;
renormalize the ports first.

Every native result-file publication (`write_csv`, `write_raw`,
`write_touchstone`, `write_vcd`) is transactional: RSpice stages and
synchronizes the complete file beside its destination before atomically
replacing it. A serialization, flush, or commit failure therefore preserves the
previous complete artifact, or leaves an absent destination absent.

## Parallelism

Simulation calls release the GIL, so threads work directly. For process-based
parallelism, netlists, configurations, and most results pickle:

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

A pickleable result carries the state behind everything its own accessors
expose, so each readable quantity, and each quantity derived from one such as
`PssResult.thd_percent` or `HbResult.is_valid`, is unchanged across a round
trip. **`SParameterResult`, `PxfResult`, and `EnvelopeResult` do not implement
the pickle protocol**; move those across a process boundary as
`result.document()` instead.

Compressed transient pickles preserve retained step sizes, every
descriptor-keyed channel with its unit, owner and per-sample validity mask, the
XSPICE digital and real event traces, the parent analysis/coordinate/topology
identity, and the typed `.FFT`, `.FOUR` and `.MEASURE` post-results. A pickle
whose sample is neither a number nor a typed absence, or whose channel role,
unit or absence reason this build does not know, is rejected rather than
repaired. Transient FFT state is explicitly versioned and is identical in full
and compressed transient pickles. Legacy transient pickles from bindings that
discarded FFT products are rejected because they cannot prove whether an empty
FFT list is genuine; rerun and repickle those analyses with the current schema.
FFT state version 2 adds completion status; earlier FFT pickle states are also
rejected. `FftResult.status` is `"complete"` for a computed spectrum or
`"incomplete-history"` when the requested sample record extends beyond the
retained trajectory, including checkpoint resume without earlier samples.
`incomplete_history` gives the available `(start, stop)` times for the latter.
These requests retain their configured window and transform length, with empty
bin arrays and no metrics. Their shared result document uses schema version 4,
with the same status and zero spectral points.

## Error handling

All errors derive from `rspice.RSpiceError`:

```text
RSpiceError
├── ParseError                   # netlist syntax/semantic errors
├── SimulationError              # circuit or solver failure
│   ├── ConvergenceError         # Newton-Raphson failed to converge
│   └── CancelledError           # Engine.cancel() stopped the active call
│       └── TimeoutError         # the time budget expired
├── MeasurementError             # RunReport.assert_passed() failures
├── RSpiceKeyError               # also a KeyError: unknown node/branch/device
├── RSpiceIndexError             # also an IndexError: out-of-range result index
├── RSpiceValueError             # also a ValueError: invalid argument value
├── RSpiceTypeError              # also a TypeError: invalid argument type
└── RSpiceNotImplementedError    # also a NotImplementedError
```

The last five derive from both `RSpiceError` and the builtin exception a caller
already expects, so `except rspice.RSpiceError` catches everything the library
raises while `except KeyError` keeps working unchanged.

Both error families carry structured attributes so automation never has to
parse a display message. `SimulationError` and its subclasses expose the stable
snake-case `kind`, `code`, and `category` tags, a conservative `retryable`
flag, `iterations` for convergence failures, and `resource`/`requested`/`limit`
when a `ResourceLimits` ceiling was hit. `ParseError` adds `kind`, `category`,
and the source provenance for the failure: `line`/`source`, the
`primary_*`/`related_*` pair for two-location errors, and error-specific fields
such as `unresolved_output_symbols`. The `.pyi` stub declares the full set.

```python
try:
    report = engine.run(netlist)
except rspice.ConvergenceError:
    report = rspice.Engine(robust_config).run(netlist)   # stronger aids
except rspice.SimulationError as e:
    if e.code == "resource_limit":
        print(f"{e.resource}: requested {e.requested}, limit {e.limit}")
    elif e.retryable:
        report = engine.run(netlist)
```

Result accessors raise standard Python exceptions: `IndexError` for
out-of-range node indices, `KeyError` for unknown node or branch names, in
every result type including AC. Invalid arguments (empty frequency lists,
non-positive stop times, zero sweep steps) raise `ValueError` before the
simulation starts.

## Threading and cancellation

Simulation calls release the GIL, so a long transient can run in a worker
thread while the main thread stays responsive, and several engines can simulate
different netlists in parallel threads.

RSpice also supports free-threaded CPython 3.14. Dedicated `cp314t` wheels keep
the GIL disabled, and immutable `Netlist`, `Engine`, and result objects may be
shared across Python threads.

Applications do not need to synthesize operating-system signals. A GUI or
service thread may inspect `engine.is_running` and `engine.active_run_count`
and call `engine.cancel()`. The method returns the number of active calls
signalled; those calling threads receive `rspice.CancelledError`, and later
calls on the same Engine remain usable. When exactly one analysis is active,
`engine.progress` returns its completed fraction if that solver provides one;
it is `None` for idle Engines, concurrent calls, or analyses with no meaningful
progress scale.

DC operating points and sweeps, AC, distortion, and S-parameter sweeps,
transfer-function, STB, pole-zero, transient and checkpoint/resume runs, noise,
Monte Carlo, parameter steps, sensitivity, PSS, HB, PAC, driven PNoise, and
oscillator-noise calls poll Python signal handlers while they run. Ctrl-C
cancels these simulations with `KeyboardInterrupt` instead of arriving only
after a completed result is returned.

## Testing

For routine Rust workspace checks, keep PyO3 and the wasm target out of the
fast path, then check this crate separately after selecting a real interpreter:

```bash
cargo check --workspace --exclude rspice-python --exclude rspice-wasm
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

The binding test suite lives in `tests/` and runs through a maturin development
install:

```bash
cd crates/rspice-python
python -m pip install "maturin==1.15.0" "numpy>=2.0,<3" "pytest==9.1.1"
maturin develop --release --locked
python -m pytest tests/ -v
```

There are also Rust-side unit tests covering the interpreter-free logic
(signal-spec parsing, the export formats, error mapping). A bare
`cargo test -p rspice-python` runs nothing, because a cdylib cannot link into a
test harness; they need the lib target named explicitly:

```bash
cargo test -p rspice-python --lib
```

That is a complement to the pytest suite, not a substitute for it.

Licensed under the [RSpice Personal Use License](../../LICENSE).
