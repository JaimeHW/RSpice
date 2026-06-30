# Analog CI Example

This example shows the supported pattern for putting an analog design under
continuous integration with the RSpice Python bindings. It lives under
`crates/rspice-python` because it is a Python API example and is verified by
the Python binding workflow.

The contract is intentionally executable:

- Acceptance criteria live in the SPICE deck as `.MEAS` statements.
- `rspice.Engine().run(...)` executes the deck and evaluates measurements.
- `report.assert_passed()` fails the test if measurements fail or are skipped.
- Pytest assertions compare key results against analytic circuit behavior.

## Circuit Pattern

```spice
* RC lowpass regression deck
V1 in 0 PULSE(0 1 0 1n 1n 1 2)
R1 in out 1k
C1 out 0 100n
.tran 1u 800u
.meas tran t_half  FIND TIME WHEN V(out)=0.5
.meas tran v_final MAX V(out)
.meas tran trise   TRIG V(out) VAL=0.1 RISE=1 TARG V(out) VAL=0.9 RISE=1
.end
```

```python
report = rspice.Engine().run(rspice.Netlist.parse(DECK))
report.assert_passed()
assert report.measurement("trise").value == pytest.approx(219.7e-6, rel=0.02)
```

`assert_passed()` is strict by design: it raises if no measurements were
evaluated, so a deck whose `.MEAS` statements were silently skipped cannot
green-wash a pipeline.

## Running Locally

From the repository root:

```bash
cd crates/rspice-python
python -m pip install maturin numpy pytest
maturin develop --release
python -m pytest examples/analog_ci -v
```

## Running in GitHub Actions

This repository runs the example from `.github/workflows/python.yml` after
installing the extension with `maturin develop --release`. A downstream project
can use the same shape:

```yaml
jobs:
  circuit-regression:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with: { python-version: "3.12" }
      - run: pip install rspice-<version>-cp38-abi3-*.whl numpy pytest
      - run: python -m pytest circuits/ -v
```

## Contents

- `test_rc_filter.py` - a complete regression suite for a first-order RC
  filter: step-response measurements checked against analytic theory,
  frequency response and pole location, and a seeded Monte Carlo bound for a
  toleranced divider.
