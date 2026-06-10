# RSpice Python Bindings

Python bindings for the RSpice circuit simulation engine, enabling automation,
scripting, and integration with the Python scientific ecosystem.

## Features

- **Full simulation API** — DC operating point, DC sweep, AC, transient, noise,
  pole-zero, Monte Carlo, sensitivity, and parametric step analysis
- **NumPy integration** — waveforms, spectra, and complex AC phasors as ndarrays
- **Releases the GIL** — long simulations run with the GIL released, so other
  Python threads (GUIs, progress reporting, parallel workers) stay live
- **Typed** — ships a complete `.pyi` stub with a `py.typed` marker for IDEs
  and type checkers
- **Typed exceptions** — a proper exception hierarchy rooted at `RSpiceError`

## Installation

### From Source

```bash
pip install maturin

cd crates/rspice-python
maturin develop --release
```

`maturin build --release` produces a redistributable abi3 wheel that works on
Python 3.8+.

## Quick Start

```python
import rspice

netlist = rspice.Netlist.parse("""
* Voltage divider
V1 1 0 10
R1 1 2 1k
R2 2 0 1k
.end
""")

engine = rspice.Engine()

# DC operating point
result = engine.run_dc_op(netlist)
print(f"V(2) = {result.voltage(2):.3f} V")     # 5.000 V

# Transient analysis (max_step defaults to stop_time / 50)
tran = engine.run_tran(netlist, stop_time=1e-3)
time = tran.time                                # NumPy array
v_out = tran.voltage_waveform(2)                # NumPy array
```

## API Overview

### Netlist

```python
netlist = rspice.Netlist.parse("V1 1 0 10\nR1 1 0 1k\n.end")

# From a file, expanding .include/.lib relative to its location;
# accepts str or os.PathLike
netlist = rspice.Netlist.parse_file(pathlib.Path("circuits") / "amplifier.sp")

# From a string, resolving includes against an explicit base path
netlist = rspice.Netlist.parse_with_includes(content, "circuits/")

netlist.num_elements, netlist.num_models, netlist.num_analyses, netlist.title
```

### Engine and Configuration

```python
engine = rspice.Engine()                      # defaults

config = rspice.SimulationConfig()
config.tolerance = 1e-12
config.temperature = 300.15                   # Kelvin
config.convergence = rspice.ConvergenceConfig.robust()
engine = rspice.Engine(config)
```

`ConvergenceConfig` exposes the DC convergence aids (GMIN stepping, source
stepping, pseudo-transient, arc-length continuation, damping strategies);
`BypassConfig` controls latent-device bypass.

### Analyses

```python
# DC operating point
op = engine.run_dc_op(netlist)
v = op.voltage("out")                          # by name or index
i = op.branch_current("V1")

# DC sweep — iterable, indexable
sweep = engine.run_dc_sweep(netlist, "V1", 0, 5, 0.1)
for v_in, sol in sweep:                        # or sweep.points()
    print(f"{v_in:.1f} V -> {sol.voltage('out'):.3f} V")
v_curve = sweep.voltage_array(2)               # NumPy array across the sweep

# AC analysis
ac = engine.run_ac(netlist, np.logspace(0, 6, 121).tolist())
gain_db = ac.voltage_db(2)                     # 20*log10|V|, NumPy array
phase = ac.voltage_phase_degrees(2)
h = ac.voltage_complex(2)                      # complex128 ndarray

# Transient
tran = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)

# Noise (temperature defaults to the engine configuration)
for r in engine.run_noise(netlist, output_node=2, frequencies=[1e3, 1e4]):
    print(f"{r.frequency:.0f} Hz: {r.output_noise_rms*1e9:.2f} nV/sqrt(Hz)")
    print(f"  dominant: {r.dominant_source()}")

# Pole-zero
pz = engine.run_pz(netlist, input_node=1, output_node=2)
print(pz.is_stable, pz.dc_gain, pz.bandwidth_hz)

# Monte Carlo
mc = engine.run_monte_carlo(netlist, num_runs=1000, seed=42)
stats = mc.get_variable("V(2)")
print(f"{stats.mean:.4f} +/- {stats.std_dev:.4f}  (p99 = {stats.percentile(99):.4f})")

# Sensitivity and parametric step
s = engine.run_sensitivity(netlist, output_node=2, param_name="R1", param_value=1e3)
for value, sol in engine.run_step(netlist, "R1", [1e3, 2e3, 5e3]):
    print(value, sol.voltage(2))
```

## Error Handling

All errors derive from `rspice.RSpiceError`:

```text
RSpiceError
├── ParseError          # netlist syntax/semantic errors
└── SimulationError     # circuit or solver failure
    └── ConvergenceError  # Newton-Raphson failed to converge
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
out-of-range node indices, `KeyError` for unknown node names.

## Threading

Simulation calls release the GIL, so a long transient can run in a worker
thread while the main thread stays responsive, and several engines can
simulate different netlists in parallel threads.

## License

RSpice Python is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).
