# rspice-python

Python bindings for the RSpice simulation engine, imported as `rspice`. A test
suite can parse a deck, run it, read the results as NumPy arrays and fail on
its `.MEAS` checks like any other assertion, so analog regression tests run in
CI beside software tests. This package embeds RSpice in Python; the desktop
application's own Python automation is a separate surface.

## What's in here

| Module | Contents |
| :--- | :--- |
| `netlist` | `Netlist` parsing, introspection and parameter variants |
| `engine` | `Engine`: one call per analysis family, the deck runner `Engine.run`, cancellation and health |
| `config` | `SimulationConfig`, `ConvergenceConfig`, `ResourceLimits` and their enums |
| `results` | One result class per analysis family, with NumPy accessors, the shared result document and pickling |
| `measure` | `.MEAS` evaluation against existing results |
| `export` | Touchstone, SPICE raw, CSV and VCD writers |
| `abort` | Ctrl-C handling and `Engine.cancel()` |
| `errors` | The exception hierarchy |

`rspice.pyi` is the complete type stub, checked against the built module by
`mypy.stubtest` in CI. It and the docstrings are the API reference.

## Boundaries

- Analyses, devices and netlist dialects belong to `rspice-core`; the analysis
  and device tables are in the root `README.md`.
- The command-line simulator is `rspice-cli` and the browser module is
  `rspice-wasm`; all three publish the same `rspice-analysis-result` document.

## Installation

Requires CPython 3.10 or newer and `numpy>=2.0,<3`. Wheels are built against
the stable ABI (abi3-py310), so one wheel per platform covers every supported
interpreter. The package is private and is not published to PyPI: install it
from an authorized source checkout or a privately supplied build artifact. Its
use and distribution are governed by the RSpice Personal Use License.

```bash
python -m pip install "maturin==1.15.0"
cd crates/rspice-python
maturin develop --release --locked
```

`maturin build --release --locked` produces an installable abi3 wheel for
authorized testing and private delivery; it grants no redistribution rights.
CI builds wheels for manylinux2014 x86-64 and AArch64, macOS Intel and Apple
silicon, and Windows x86-64 and ARM64, plus `cp314t` wheels for free-threaded
CPython 3.14 on the same six targets, and runs the binding suite with the GIL
disabled on Linux, macOS and Windows.

A source distribution needs one repair step, because maturin builds it from
`cargo package --list`, which omits the files `rspice-core` embeds from the
workspace root and prunes the workspace lockfile:

```bash
maturin sdist --out dist
python ../../tools/release/repair_sdist_lock.py dist/rspice-*.tar.gz
```

The repair runs offline unless given `--online`, rejects any new or changed
external package identity, source or checksum, and verifies the archive with
Cargo's `--locked` mode; run `cargo fetch --locked` first on a clean machine.

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

`engine.run` executes every analysis directive in the deck, in deck order, and
returns a `RunReport`:

- `report.records` holds at least one record per directive; a directive the
  engine could not execute is listed with `skipped=True` and a reason.
- The singular accessors (`report.ac`) return the last result of a kind, and
  the `all_*` accessors return every one.
- A `.PNOISE` card around a driven carrier reports an output power spectral
  density in `report.pnoise`; around an autonomous `.PSS` carrier it reports a
  carrier-normalized dBc/Hz phase-noise spectrum in `report.oscillator_noise`.
- A record produced under a `.STEP` or `.TEMP` axis carries its `analysis_id`
  (`ac-001`, `ac-002`) and a typed `coordinate` with its stable ID and axis
  assignments.
- `report.assert_passed()` raises `MeasurementError` unless every directive
  ran, at least one measurement was evaluated and every measurement passed, so
  neither a skipped analysis nor a missing measurement passes as success.

A directive that fails is recorded and the rest of the deck still runs; pass
`continue_on_error=False` to raise on the first failure instead.
`Engine.measure` evaluates the deck's `.MEAS` statements against a
`TransientResult` or `DcSweepResult` already in hand, and raises `TypeError`
for any other result.

## Results and export

Every result family exposes NumPy accessors and, beside them, `signals()`,
`scalars()`, `device_observables()` and `document()`, the shared
`rspice-analysis-result` document. A quantity with no finite value is never a
silent zero: NumPy accessors encode it as `NaN` with a validity array beside
it where one applies, `scalars()` gives it an `unavailable_reason`, and the
document uses `null`.

```python
# Touchstone v1 for scikit-rf, ADS, or a datasheet plot
sparams.write_touchstone(f"dut.{sparams.touchstone_extension}", format="ma")
text = sparams.to_touchstone(frequency_unit="ghz", comments=["nominal corner"])

# ngspice-compatible raw files; AC is written with Flags: complex
tran.write_raw("run.raw", format="binary")
ac.write_raw("ac.raw")                          # or ac.to_raw() -> bytes

# Value Change Dump of the event timelines: the dump `rspice run -f vcd` writes
tran.write_vcd("run.vcd")                       # or tran.to_vcd() -> str

# RFC 4180 CSV; AC splits each phasor into <name>_real / <name>_imag
tran.write_csv("run.csv")
tran.export_columns                             # column order, in advance
```

Touchstone v1 carries one reference impedance, so a sweep whose ports do not
share one is refused; renormalize the ports first. Every `write_*` call stages
the complete file beside its destination and then replaces it atomically, so a
failure leaves the previous file, or no file, in place.

Netlists, configurations and most results pickle, so they cross process
boundaries:

```python
from concurrent.futures import ProcessPoolExecutor

def corner(rval):
    netlist = BASE.with_parameters({"rval": rval})
    return rval, rspice.Engine().run_dc_op(netlist).voltage("out")

with ProcessPoolExecutor() as pool:
    for rval, v_out in pool.map(corner, [1e3, 2e3, 5e3, 10e3]):
        print(rval, v_out)
```

A `Netlist` pickles as its deck text and replays the parse under the same
`ResourceLimits`. A pickled result keeps every quantity its accessors expose,
but it carries the binding's projection rather than the core result, so its
`signals()`, `scalars()`, `device_observables()` and `document()` raise
`RSpiceNotImplementedError` after a round trip; pickle `result.document()` when
the document itself must survive. `SParameterResult`, `PxfResult` and
`EnvelopeResult` do not pickle; send their `document()` instead.

## Errors, threads and cancellation

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

The last five also derive from the builtin exception a caller already expects,
so `except rspice.RSpiceError` catches everything the library raises while
`except KeyError` keeps working. Errors carry structured attributes, so
automation never parses a message: `SimulationError` exposes snake-case
`kind`, `code` and `category` tags, a conservative `retryable` flag,
`iterations` for convergence failures and `resource`, `requested` and `limit`
when a `ResourceLimits` ceiling was hit; `ParseError` carries `kind`,
`category`, the `line` and `source` of the failure and the fields specific to
its kind, all declared in the stub.

```python
try:
    report = engine.run(netlist)
except rspice.ConvergenceError:
    report = rspice.Engine(robust_config).run(netlist)   # stronger aids
except rspice.SimulationError as e:
    if e.code == "resource_limit":
        print(f"{e.resource}: requested {e.requested}, limit {e.limit}")
```

Simulation calls release the GIL, so several engines can run in parallel
threads, and the free-threaded `cp314t` wheels keep the GIL disabled with
`Netlist`, `Engine` and result objects shared across threads. A call runs on a
worker thread while the calling thread keeps servicing Python signal handlers,
so Ctrl-C raises `KeyboardInterrupt` during a long run. From another thread,
`engine.cancel()` stops every active call on that Engine (they raise
`CancelledError`, and the Engine stays usable), `engine.is_running` and
`engine.active_run_count` report activity, and `engine.progress` gives the
completed fraction when exactly one call that reports progress is active.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `veriloga-native` | Default. Compiles `.va` models at run time through the core's native Verilog-A backend; `--no-default-features` builds without it |
| `extension-module` | Builds the PyO3 extension module; maturin enables it through `pyproject.toml` |

## Building and testing

The binding suite in `tests/` runs against a development install:

```bash
cd crates/rspice-python
python -m pip install "maturin==1.15.0" "numpy>=2.0,<3" "pytest==9.1.1"
maturin develop --release --locked
python -m pytest tests/ -v
```

The Rust-side unit tests cover the interpreter-free logic. The crate is a
cdylib with `test = false`, so a bare `cargo test -p rspice-python` runs
nothing; name the lib target:

```bash
cargo test -p rspice-python --lib
```

On Windows, the Microsoft Store `python` alias can point PyO3 at a stub
executable; set `PYO3_PYTHON` to a real interpreter before any `cargo` command
on this crate:

```powershell
py -3 -m venv .venv
$env:PYO3_PYTHON = (Resolve-Path .\.venv\Scripts\python.exe).Path
cargo check -p rspice-python
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
