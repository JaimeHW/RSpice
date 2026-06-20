# BSIM4 RGATEMOD=1 Next-Slice Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add native BSIM4 `RGATEMOD=1` gate-resistance support as the next commercial/Xyce coverage slice.

**Architecture:** Keep intrinsic BSIM4 evaluation on the existing gate-prime math and add an external gate node connected by the ngspice/Xyce constant electrode gate resistance. Scope this first slice to model-level `RGATEMOD=1`; leave `RGATEMOD=2/3` and instance override support for later explicit tests.

**Tech Stack:** Rust, `rspice-core`, ngspice-46 BSIM4 source/oracles, Xyce 7.10 regression decks/goldens, existing RSpice BSIM4 native tests.

---

### Task 1: Capture The RGATEMOD=1 Topology Gap

**Files:**
- Modify: `crates/rspice-core/tests/bsim4_native.rs`
- Reference: `C:/Users/James/Desktop/ngspice-46-release/ngspice-46/src/spicelib/devices/bsim4/b4temp.c`
- Reference: `C:/Users/James/Desktop/ngspice-46-release/ngspice-46/src/spicelib/devices/bsim4/b4ld.c`

- [x] **Step 1: Write the failing topology test**

Add a test that builds a simple one-device deck with `RGATEMOD=1`, `MTRLMOD=0`, `RBODYMOD=0`, `TRNQSMOD=0`, and `ACNQSMOD=0`. The red behavior was the current typed construction error; the green behavior proves the lowered `m1.__rg` conductance matches ngspice's `grgeltd` formula:

```rust
#[test]
fn rgatemod1_creates_external_gate_resistance() {
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 rgatemod=1 rshg=5 xgw=0 xgl=0 ngcon=1",
    );
    let deck = format!(
        "* bsim4 rgatemod=1 gate resistance topology\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         vb b 0 dc 0\n\
         m1 d g 0 b n45 w=1u l=45n nf=2 ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {models}\n\
         .op\n\
         .end\n"
    );
    let wnew = 1.0e-6 / 2.0;
    let weff_cj = wnew - 2.0 * 2.0e-9;
    let lnew = 45.0e-9 - 20.0e-9;
    let rgeltd = 5.0 * (weff_cj / 3.0) / (2.0 * lnew);
    let expected_g = 1.0 / rgeltd;
    let conductance = bsim4_resistor_conductance(&deck, "m1.__rg");
    assert!(((conductance - expected_g).abs() / expected_g) < 1e-12);
}
```

- [x] **Step 2: Run the red test**

Run: `cargo test -p rspice-core --test bsim4_native rgatemod1_creates_external_gate_resistance -- --nocapture`

Observed red before implementation: FAILS for the existing `RGATEMOD=1 is not implemented` path.

### Task 2: Add Mode-1 Gate Resistance State And Topology

**Files:**
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/temp.rs`
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/device.rs`
- Modify: `crates/rspice-core/src/engine/builder.rs`
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/mod.rs`

- [x] **Step 1: Store `grgeltd` in the instance temp tail**

Use the ngspice formula:

```text
Rgeltd = rshg * (xgw + weffCJ / (3 * ngcon)) / (ngcon * nf * (Lnew - xgl))
grgeltd = 1 / Rgeltd when Rgeltd > 0, else 1e3
```

- [x] **Step 2: Allocate a gate-prime node for `RGATEMOD=1`**

Keep external `g` as the netlist gate. Route intrinsic BSIM4 branch voltages through the prime gate node. Stamp only the linear gate resistor between external gate and prime gate for mode 1.

- [x] **Step 3: Remove only the `RGATEMOD=1` rejection**

Keep `RGATEMOD=2/3` rejected until their middle-gate/NQS paths have tests.

### Task 3: Add AC/Noise Coverage

**Files:**
- Modify: `crates/rspice-core/tests/bsim4_native.rs`
- Modify: `crates/rspice-core/src/engine/advanced/noise.rs` if the gate resistor noise source is not already represented by the generic resistor storage path.

- [x] **Step 1: Add a reduced gain-stage AC test**

Use the Xyce regression seed `C:/Users/James/Downloads/Xyce_Regression-master/Xyce_Regression-master/Netlists/ACtests/bsim4/gstage.cir`, but force the reduced card to `RGATEMOD=1 MTRLMOD=0 RBODYMOD=0 TRNQSMOD=0 ACNQSMOD=0`.

- [x] **Step 2: Add the noise source check**

Verify that `.noise` includes thermal noise for the external-gate-to-gate-prime resistor, matching ngspice's `.rg` source behavior for mode 1.

### Task 4: Verification Gates

- [x] Run: `cargo test -p rspice-core --test bsim4_native rgatemod -- --nocapture`
- [x] Run: `cargo test -p rspice-core --test bsim4_native -- --nocapture`
- [x] Run: `cargo test -p rspice-core --lib bsim4v8 -- --nocapture`
- [x] Run: `cargo test -p rspice-core --lib noise -- --nocapture`
- [x] Run: `cargo fmt --all -- --check`
- [x] Run: `git diff --check`
- [x] Run the full ngspice gate only as release:

```powershell
$env:CARGO_PROFILE_RELEASE_LTO='false'
$env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS='32'
cargo test --release -p rspice-core --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture
```

The full ngspice gate only counts when output shows `Finished release profile [optimized]` and `target\release\deps\ngspice_regression-...exe`.
