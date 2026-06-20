# VDMOS Transient Xyce Oracle Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add native transient charge integration for RSpice VDMOS so Xyce MOS LEVEL=18 switching behavior is validated against the official Xyce VDMOS transient regression.

**Architecture:** Reuse the existing transient companion pattern already used by MOSFET/JFET/diode devices. Keep VDMOS as a native device, add a VDMOS-specific history and companion stamp path, and validate branch current waveforms against Xyce gold data.

**Implementation note:** The completed implementation intentionally covers the full native VDMOS transient charge set rather than only the original three-branch sketch: `qgs`, `qgd`, `qgb`, `qds`, `qbs`, `qbd`, and `qd1`. Subagent review follow-ups were also completed: mixed BJT/JFET/MOSFET+VDMOS decks no longer defer generic voltage LTE through a family-only shortcut, and PVDMOS transient charge companion slots now follow the polarity-normalized branch orientation.

**Tech Stack:** Rust, `rspice-core`, Xyce 7.10 regression data, ngspice 46 release regression gate.

---

### Task 1: Add The RED Xyce VDMOS Transient Test

**Files:**
- Modify: `crates/rspice-core/tests/vdmos_native.rs`

- [x] **Step 1: Add interpolation and transient waveform helpers**

```rust
fn transient_node_series<'a>(
    names: &[String],
    voltages: &'a [Vec<f64>],
    want: &str,
) -> &'a [f64] {
    let idx = names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(want))
        .unwrap_or_else(|| panic!("missing node {want} in {names:?}"));
    &voltages[idx]
}

fn interpolate(time: &[f64], values: &[f64], t: f64) -> f64 {
    let idx = time.partition_point(|x| *x < t);
    if idx == 0 {
        return values[0];
    }
    if idx >= time.len() {
        return *values.last().unwrap();
    }
    let (t0, t1) = (time[idx - 1], time[idx]);
    let (v0, v1) = (values[idx - 1], values[idx]);
    v0 + (v1 - v0) * (t - t0) / (t1 - t0)
}
```

- [x] **Step 2: Add the Xyce regression deck**

```rust
fn xyce_irf130_tran_deck() -> &'static str {
    "\
IRF130 Test Circuit
VD 3 1 0.5
VS 2 0 0
VG 4 0 10 pulse(0 10 300ns 50ns 50ns 400ns 1000ns)
VID 0 1 DC 0
M1 3 4 2 0 IRF130 W=0.386 L=2.5u
.MODEL IRF130 NMOS LEVEL=18
+ CV=1
+ CVE=1
+ VTO=3.5
+ RD= 0
+ RS= 0.005
+ LAMBDA=0
+ M=3
+ SIGMA0=0
+ UO=230
+ VMAX=4e4
+ DELTA=5
+ TOX=50nm
.TRAN 0.5n 1u 0u 2n
.PRINT TRAN precision=10 width=19 V(3) V(4) {I(VID)+0.5}
.options timeint reltol=1.0e-2 abstol=1.0e-7
.END
"
}
```

- [x] **Step 3: Add failing test against representative Xyce points**

```rust
#[test]
fn xyce_level18_irf130_transient_switching_current_matches_xyce_gold() {
    let netlist = Netlist::parse(xyce_irf130_tran_deck()).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 1.0e-6, 2.0e-9)
        .expect("Xyce IRF130 transient solves");

    let v4 = transient_node_series(&result.node_names, &result.voltages, "4");
    let i_vid = result
        .try_branch_current_waveform_named("vid")
        .unwrap_or_else(|| panic!("missing VID branch in {:?}", result.branch_names));

    let oracle = [
        (3.1024804688e-7, 2.0496093750, 0.5000000000),
        (3.1799434538e-7, 3.5988690761, 0.51973737192),
        (3.2502793402e-7, 5.0055868039, 1.7259214736),
        (3.5000000000e-7, 10.000000000, 3.6608820456),
        (4.0056275235e-7, 10.000000000, 3.6931714117),
        (7.7496941475e-7, 5.0061170503, 1.8121702232),
        (8.0000000000e-7, 0.0000000000, 0.5000000000),
    ];

    for (t, expected_vg, expected_expr) in oracle {
        let got_vg = interpolate(&result.time, v4, t);
        let got_expr = interpolate(&result.time, i_vid, t) + 0.5;
        assert!((got_vg - expected_vg).abs() < 8.0e-2);
        assert!((got_expr - expected_expr).abs() < 8.0e-2 * expected_expr.abs().max(1.0));
    }
}
```

- [x] **Step 4: Run RED test**

Run: `cargo test -p rspice-core --test vdmos_native xyce_level18_irf130_transient_switching_current_matches_xyce_gold -- --nocapture`

Expected: FAIL before production changes because VDMOS transient charge/capacitance is not in the transient companion path.

### Task 2: Add Native VDMOS Transient Companion Path

**Files:**
- Modify: `crates/rspice-core/src/engine/transient.rs`
- Modify: `crates/rspice-core/src/engine/transient/state.rs`
- Modify: `crates/rspice-core/src/engine/transient/residual.rs`
- Modify: `crates/rspice-core/src/device/mosfet/vdmos/device.rs`

- [x] **Step 1: Add `VdmosTransientHistory` near the other transient histories**

```rust
#[derive(Debug, Clone, Default)]
struct VdmosTransientHistory {
    vgs_prev: Vec<Value>,
    vgs_prev_prev: Vec<Value>,
    qgs_prev: Vec<Value>,
    qgs_prev_prev: Vec<Value>,
    cqgs_prev: Vec<Value>,
    vgd_prev: Vec<Value>,
    vgd_prev_prev: Vec<Value>,
    qgd_prev: Vec<Value>,
    qgd_prev_prev: Vec<Value>,
    cqgd_prev: Vec<Value>,
    vds_prev: Vec<Value>,
    vds_prev_prev: Vec<Value>,
    qds_prev: Vec<Value>,
    qds_prev_prev: Vec<Value>,
    cqds_prev: Vec<Value>,
    accepted_dt_prev: Value,
    accepted_dt_prev_prev: Value,
}
```

- [x] **Step 2: Add VDMOS charge helper methods**

```rust
pub(crate) fn transient_charge_branch_voltages_at(
    &self,
    voltages: &[Value],
) -> (Value, Value, Value) {
    let g = self.gate;
    let di = self.drain_int.unwrap_or(self.drain);
    let si = self.source_int.unwrap_or(self.source);
    let vg = if g > 0 { voltages[g - 1] } else { 0.0 };
    let vd = if di > 0 { voltages[di - 1] } else { 0.0 };
    let vs = if si > 0 { voltages[si - 1] } else { 0.0 };
    (vg - vs, vg - vd, vd - vs)
}
```

- [x] **Step 3: Link three VDMOS companion branches**

Use `TwoTerminalStampSlots::link(matrix, vdmos.gate, source_int)`, `TwoTerminalStampSlots::link(matrix, vdmos.gate, drain_int)`, and `TwoTerminalStampSlots::link(matrix, drain_int, source_int)`.

- [x] **Step 4: Stamp VDMOS transient companions**

Use the existing `jfet_companion_terms` helper for the three effective capacitances returned by `vdmos.capacitances(vgs, vds)`, and stamp through `stamp_two_terminal_companion_direct`.

- [x] **Step 5: Update VDMOS history on every accepted transient point**

Mirror the MOSFET accepted-history pattern for `qgs`, `qgd`, and `qds`, setting `accepted_dt_prev_prev` and `accepted_dt_prev`.

- [x] **Step 6: Wire the history and slots through transient Newton/residual paths**

Pass `vdmos_history` and `vdmos_companion_slots` wherever transient companion stamping is called, including residual checks and gmin rescue contexts.

### Task 3: Verify And Gate

**Files:**
- No additional files expected.

- [x] **Step 1: Run targeted VDMOS tests**

Run: `cargo test -p rspice-core --test vdmos_native -- --nocapture`

Expected: PASS.

- [x] **Step 2: Run policy regression**

Run: `cargo test -p rspice-core --test mos_level_policy native_levels_unaffected -- --nocapture`

Expected: PASS.

- [x] **Step 3: Run formatting and diff checks**

Run: `cargo fmt --all -- --check`

Run: `git diff --check`

Expected: PASS, except existing CRLF warnings may appear in unrelated files.

- [x] **Step 4: Run full ngspice regression suite in release mode only**

Run: `$env:CARGO_PROFILE_RELEASE_LTO='false'; $env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS='32'; cargo test --release -p rspice-core --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture`

Expected: `TOTAL 113 tests | 113 passed | 0 failed | 0 skipped`.
