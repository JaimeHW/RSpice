# RSpice Python Bindings

Commercial-grade Python bindings for the RSpice circuit simulation engine, enabling 
automation, scripting, and integration with the Python scientific ecosystem.

## Features

- **Full Simulation API** — DC, AC, Transient, and sweep analysis
- **NumPy Integration** — Zero-copy waveform access as NumPy arrays
- **Pythonic API** — Clean, intuitive interface following Python conventions
- **Type Hints** — Full type annotation support for IDE integration
- **Exception Handling** — Proper Python exceptions for error conditions

## Installation

### From Source (Development)

```bash
# Install maturin build tool
pip install maturin

# Build and install in development mode
cd crates/rspice-python
maturin develop --release
```

### With pip (Future)

```bash
pip install rspice
```

## Quick Start

```python
import rspice
import numpy as np

# Parse a netlist
netlist = rspice.Netlist.parse("""
V1 1 0 10
R1 1 2 1k
R2 2 0 1k
.end
""")

# Create simulation engine
engine = rspice.Engine()

# Run DC operating point
result = engine.run_dc_op(netlist)
print(f"V(2) = {result.voltage(2):.3f} V")  # Expected: 5.0 V

# Run transient analysis
tran = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)
time = tran.time  # NumPy array
voltage = tran.voltage_waveform(2)  # NumPy array
```

## API Reference

### Netlist

```python
# Parse from string
netlist = rspice.Netlist.parse("V1 1 0 10\nR1 1 0 1k\n.end")

# Parse from file with include resolution
netlist = rspice.Netlist.parse_file("circuit.sp")
```

### Engine

```python
# Default configuration
engine = rspice.Engine()

# Custom configuration
config = rspice.SimulationConfig()
config.tolerance = 1e-12
config.temperature = 300.15
engine = rspice.Engine(config)

# DC analysis
dc_result = engine.run_dc_op(netlist)
sweep_results = engine.run_dc_sweep(netlist, "V1", 0, 5, 0.1)

# AC analysis
ac_result = engine.run_ac(netlist, [10, 100, 1000, 10000])

# Transient analysis
tran_result = engine.run_tran(netlist, stop_time=1e-3, max_step=1e-6)
```

### Results

```python
# DC result
v = result.voltage(1)           # By node index
v = result.voltage("out")       # By node name
i = result.branch_current("V1") # Current through source

# Transient result (NumPy arrays)
t = tran.time                   # Time points array
v = tran.voltage_waveform(1)    # Voltage waveform array
n = tran.num_points             # Number of time points

# AC result (NumPy arrays)
f = ac.frequencies              # Frequency array
mag = ac.voltage_magnitude(1)   # Magnitude array
phase = ac.voltage_phase(1)     # Phase array (radians)
```

## License

See the LICENSE file in the repository root.
