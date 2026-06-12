# Analog design CI with RSpice

This directory shows the pattern for putting circuits under continuous
integration: acceptance criteria live in the netlist as `.MEAS` statements,
pytest runs the deck, and a failed measurement fails the pipeline like any
other broken test.

## The pattern

1. Write the circuit and its acceptance criteria in one deck:

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

2. Run it and assert in pytest:

   ```python
   report = rspice.Engine().run(rspice.Netlist.parse(DECK))
   report.assert_passed()   # any failed/skipped measurement fails the test
   assert report.measurement("trise").value == pytest.approx(219.7e-6, rel=0.02)
   ```

`assert_passed()` is strict by design: it raises if *no* measurements were
evaluated, so a deck whose `.MEAS` statements were silently skipped cannot
green-wash a pipeline.

## Running locally

```bash
pip install maturin numpy pytest
(cd crates/rspice-python && maturin develop --release)
python -m pytest examples/python/ -v
```

## Running in GitHub Actions

See [.github/workflows/python.yml](../../.github/workflows/python.yml) for
the full matrix this repository uses (3 platforms × 2 Python versions). The
minimal job for a project that *uses* RSpice:

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

## What's in here

- [test_rc_filter.py](test_rc_filter.py) — a complete regression suite for a
  first-order RC filter: step-response measurements checked against analytic
  theory, frequency response and pole location, and a Monte Carlo bound on
  corner-frequency drift with toleranced components.
