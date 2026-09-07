# Conformance-owned fixtures

`veriloga-golden/` contains the complete, deterministic stamp fingerprints for
the generated compact-model corpus. They are correctness artifacts, not timing
baselines.

Verify them through the owning crate:

```text
cargo run --locked --release -p rspice-conformance \
  --features veriloga-builtins-models \
  --bin rspice-veriloga-golden -- verify
```

Capture requires an explicit, nonexistent output directory and publishes the
complete model set transactionally:

```text
cargo run --locked --release -p rspice-conformance \
  --features veriloga-builtins-models \
  --bin rspice-veriloga-golden -- capture --out target/veriloga-golden-candidate
```

There is no in-place replacement mode. Promote a candidate only after reviewing
the complete numerical diff and running the independent derivative audit. A
capture produced by the implementation under test is a snapshot, not proof that
the captured answer is correct.

The 2026-09-07 update to ASM-ESD, ASM-ESD diode and BSIM-IMG replaces only the
zero-bias NaN stamp entries left by the former power-rule differentiation.
Every finite entry and all five nonzero-bias records remain byte-identical.
The corrected equilibrium residual is zero; the ASM thermal diagonal is
`1 / rth0 = 2000`, with the remaining formerly non-finite partials zero.
The independent current and charge derivative audits pass the corpus's existing
criteria, including BSIM-IMG's separately recorded nonzero-bias derivative
deviation. The replay no longer permits matching non-finite values.

HiSIM-SOTB's six-point fingerprint was also refreshed after comparing every RHS,
Jacobian and capacitance entry with two runtime routes compiled from the
unchanged upstream Verilog-A source. The native JIT matched the generated Rust
arrays exactly. The bytecode interpreter's maximum absolute differences were
`1.90e-19` for RHS, `7.05e-19` for Jacobian and `1.07e-27` for capacitance.
The six-point derivative audit retained only the already recorded equilibrium
kink; no replay tolerance or derivative exception was widened. This replaces
the earlier generated-backend snapshot, whose nonzero-bias currents also
differed from both runtime routes.
