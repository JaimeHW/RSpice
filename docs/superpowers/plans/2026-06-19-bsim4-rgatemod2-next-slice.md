# BSIM4 RGATEMOD=2 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add native BSIM4 `RGATEMOD=2` support with DC, AC, transient, and noise coverage against Xyce 7.10 and ngspice-46.

**Architecture:** Keep `RGATEMOD=1` as the builder-lowered constant gate resistor. Add `RGATEMOD=2` as a native BSIM4 device path because ngspice folds the electrode resistance and channel-charge gate resistance into a bias-dependent `gcrg` branch between `gNodeExt` and `gNodePrime`. Scope this slice to `TRNQSMOD=0`, `ACNQSMOD=0`, and no `RGATEMOD=3` middle-gate node.

**Implementation note:** This slice is complete in the current worktree, and support now extends beyond the original quasi-static-only scope. Focused verification on 2026-06-19 passed `rgatemod2`, `rgatemod`, full `bsim4_native` (58/58), `bsim4v8` lib (35 passed, 1 ignored), `noise` lib (32/32), and the release-only full ngspice suite (113/113).

**Tech Stack:** Rust, `rspice-core`, local ngspice-46 source/oracle at `C:/Users/James/Desktop/ngspice-46-release/ngspice-46`, Xyce 7.10 at `C:/Users/James/Downloads/Xyce-7.10-NORAD/bin/Xyce.exe`, existing BSIM4 native tests.

---

## File Structure

- Modify `crates/rspice-core/src/device/mosfet/bsim4v8/eval.rs`: compute and expose `gcrg`, `gcrgg`, `gcrgd`, `gcrgs`, and `gcrgb` for `RGATEMOD=2`.
- Modify `crates/rspice-core/src/device/mosfet/bsim4v8/device.rs`: stamp the mode-2 gate network in DC, AC, transient companion matrix/RHS, and operating current output paths.
- Modify `crates/rspice-core/src/engine/builder.rs`: allocate a gate-prime node for `RGATEMOD=2` without lowering `m1.__rg` as a plain resistor.
- Modify `crates/rspice-core/src/device/mosfet/bsim4v8/mod.rs`: allow `RGATEMOD=2` while keeping `RGATEMOD=3` rejected.
- Modify `crates/rspice-core/src/device/mosfet/bsim4v8/params.rs`: update module comments that currently say only modes 0/1 are ported.
- Modify `crates/rspice-core/src/device/mosfet/bsim4v8/tests.rs`: replace the rejection test for mode 2 with a mode 3 rejection test.
- Modify `crates/rspice-core/tests/bsim4_native.rs`: add topology/DC, AC, transient, and noise oracle tests for mode 2.
- Modify `crates/rspice-core/src/engine/advanced/noise.rs`: add mode-2 `.rg` thermal-noise conductance if the generic resistor path no longer covers this branch.

### Task 1: Capture The RGATEMOD=2 Red Tests

**Files:**
- Modify: `crates/rspice-core/tests/bsim4_native.rs`
- Reference: `C:/Users/James/Desktop/ngspice-46-release/ngspice-46/src/spicelib/devices/bsim4/b4ld.c:2192`
- Reference: `C:/Users/James/Desktop/ngspice-46-release/ngspice-46/src/spicelib/devices/bsim4/b4ld.c:5244`
- Reference: `C:/Users/James/Desktop/ngspice-46-release/ngspice-46/src/spicelib/devices/bsim4/b4noi.c:236`

- [x] **Step 1: Add a helper deck**

Add this helper near the existing `rgatemod1` helpers:

```rust
fn rgatemod2_common_source_deck() -> String {
    let models = models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 rgatemod=2 rshg=5e8 xrcrg1=12 xrcrg2=1 xgw=0 xgl=0 ngcon=1 trnqsmod=0 acnqsmod=0",
    );
    format!(
        "* bsim4 rgatemod2 common source\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.55 ac 1 pulse(0.3 0.8 20p 5p 5p 80p 200p)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {models}\n\
         .op\n\
         .ac dec 1 1e6 1e11\n\
         .tran 0.2p 80p\n\
         .end\n"
    )
}
```

- [x] **Step 2: Add the construction red test**

```rust
#[test]
fn rgatemod2_runs_without_simplified_mos_optin() {
    let deck = rgatemod2_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine().run_dc_op(&netlist);
    assert!(
        result.is_ok(),
        "RGATEMOD=2 must build natively, got {result:?}"
    );
}
```

- [x] **Step 3: Run the red test**

Run: `cargo test -p rspice-core --test bsim4_native rgatemod2_runs_without_simplified_mos_optin -- --nocapture`

Expected before implementation: FAIL with `RGATEMOD=2 is not implemented`.

### Task 2: Compute The Mode-2 Gate Conductance State

**Files:**
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/eval.rs`

- [x] **Step 1: Add fields to `Bsim4v8Op`**

```rust
pub gcrg: Value,
pub gcrgg: Value,
pub gcrgd: Value,
pub gcrgs: Value,
pub gcrgb: Value,
```

- [x] **Step 2: Compute ngspice's `gcrg` branch**

Insert after `Ids`, `tmp1/tmp2/tmp3`, and `beta` derivatives are available, matching `b4ld.c:2192-2225`:

```rust
if model.rgate_mod > 1 || model.trnqs_mod != 0 || model.acnqs_mod != 0 {
    let t9 = p.xrcrg2 * model_temp.vtm;
    let t0 = t9 * beta;
    let dt0_dvd = (dbeta_dvd + dbeta_dvg * dvgsteff_dvd) * t9;
    let dt0_dvb = (dbeta_dvb + dbeta_dvg * dvgsteff_dvb) * t9;
    let dt0_dvg = dbeta_dvg * t9;

    op.gcrg = p.xrcrg1 * (t0 + ids);
    op.gcrgd = p.xrcrg1 * (dt0_dvd + tmp1);
    op.gcrgb = p.xrcrg1 * (dt0_dvb + tmp2) * dvbseff_dvb;
    op.gcrgg = p.xrcrg1 * (dt0_dvg + tmp3) * dvgsteff_dvg;
    if nf != 1.0 {
        op.gcrg *= nf;
        op.gcrgg *= nf;
        op.gcrgd *= nf;
        op.gcrgb *= nf;
    }
    if model.rgate_mod == 2 {
        let denom = inst.gate_conductance + op.gcrg;
        let scale = inst.gate_conductance * inst.gate_conductance / (denom * denom);
        op.gcrg = inst.gate_conductance * op.gcrg / denom;
        op.gcrgg *= scale;
        op.gcrgd *= scale;
        op.gcrgb *= scale;
    }
    op.gcrgs = -(op.gcrgg + op.gcrgd + op.gcrgb);
}
```

- [x] **Step 3: Run focused eval tests**

Run: `cargo test -p rspice-core --lib bsim4v8 -- --nocapture`

Expected: existing BSIM4 unit tests pass.

### Task 3: Allocate The Mode-2 Gate-Prime Node

**Files:**
- Modify: `crates/rspice-core/src/engine/builder.rs`
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/mod.rs`
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/tests.rs`

- [x] **Step 1: Allocate `__gint` for `RGATEMOD=2` without a resistor**

Replace the current gate allocation with:

```rust
let gate = match core.model.rgate_mod {
    1 => {
        let gint = circuit.get_or_create_node(&format!("{}.__gint", element.name));
        circuit.resistors.add(
            format!("{}.__rg", element.name),
            gate_external,
            gint,
            1.0 / (core.inst.gate_conductance * multiplier),
        );
        gint
    }
    2 => circuit.get_or_create_node(&format!("{}.__gint", element.name)),
    _ => gate_external,
};
```

- [x] **Step 2: Allow mode 2 and keep mode 3 rejected**

In `Bsim4v8::new_shared`, change the guard to:

```rust
if model.rgate_mod > 2 {
    return Err(format!(
        "BSIM4 '{name}': RGATEMOD={} is not implemented (only RGATEMOD=0, 1, or 2)",
        model.rgate_mod
    ));
}
```

- [x] **Step 3: Update the constructor rejection unit test**

In `crates/rspice-core/src/device/mosfet/bsim4v8/tests.rs`, keep:

```rust
reject("RGATEMOD", 3.0, "RGATEMOD");
```

and remove the mode-2 rejection expectation.

### Task 4: Stamp Mode-2 DC, AC, And Transient Gate Branches

**Files:**
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/device.rs`

- [x] **Step 1: Add a mode-2 branch helper**

```rust
fn rgate_mod2_terms(&self, voltages: &[Value], bias: Bsim4v8Bias, op: &Bsim4v8Op)
    -> (Value, Value, Value, Value, Value, Value)
{
    let vge = self.core.mtype
        * (Self::node_voltage(voltages, self.node_gate_external)
            - Self::node_voltage(voltages, self.node_source));
    let t0 = vge - bias.vgs;
    let gcrgd = op.gcrgd * t0;
    let gcrgg = op.gcrgg * t0;
    let gcrgs = op.gcrgs * t0;
    let gcrgb = op.gcrgb * t0;
    let ceqgcrg = -(gcrgd * bias.vds + gcrgg * bias.vgs + gcrgb * bias.vbs);
    (op.gcrg, gcrgg - op.gcrg, gcrgd, gcrgs, gcrgb, ceqgcrg)
}
```

- [x] **Step 2: Stamp DC rows for `RGATEMOD=2`**

Mirror `b4ld.c:5244-5256` in `stamp_op`:

```rust
if self.core.model.rgate_mod == 2 {
    let (gcrg, gcrgg, gcrgd, gcrgs, gcrgb, ceqgcrg) =
        self.rgate_mod2_terms(voltages, bias, op);
    stamp_rhs(matrix, self.node_gate_external, -m * ceqgcrg);
    stamp_rhs(matrix, self.node_gate, m * ceqgcrg);
    stamp(matrix, self.node_gate_external, self.node_gate_external, m * gcrg);
    stamp(matrix, self.node_gate_external, self.node_gate, m * gcrgg);
    stamp(matrix, self.node_gate_external, self.node_drain, m * gcrgd);
    stamp(matrix, self.node_gate_external, self.node_source, m * gcrgs);
    stamp(matrix, self.node_gate_external, self.node_bulk, m * gcrgb);
    stamp(matrix, self.node_gate, self.node_gate_external, -m * gcrg);
    stamp(matrix, self.node_gate, self.node_gate, -m * gcrgg);
    stamp(matrix, self.node_gate, self.node_drain, -m * gcrgd);
    stamp(matrix, self.node_gate, self.node_source, -m * gcrgs);
    stamp(matrix, self.node_gate, self.node_bulk, -m * gcrgb);
}
```

- [x] **Step 3: Extend AC and transient charge companion stamps**

Use the same matrix pattern as DC, and add it to the AC imaginary/real stamp where `b4acld.c:498-514` places `gcrg` terms. Transient RHS must match `b4ld.c:5026-5030`: gate-prime gets `+ ceqgcrg` in the existing gate RHS correction, and gate-ext gets `- ceqgcrg`.

### Task 5: Add Oracle Tests

**Files:**
- Modify: `crates/rspice-core/tests/bsim4_native.rs`

- [x] **Step 1: Add AC oracle test**

Run an AC sweep over `rgatemod2_common_source_deck()` and assert `V(out)` against Xyce 7.10 references at `1e6`, `1e8`, `1e10`, and `1e11` Hz. Generate references with:

```powershell
& "C:\Users\James\Downloads\Xyce-7.10-NORAD\bin\Xyce.exe" deck.cir
```

The test should use:

```rust
let freqs = vec![1.0e6, 1.0e8, 1.0e10, 1.0e11];
let results = engine().run_ac(&netlist, &freqs).expect("AC runs");
```

- [x] **Step 2: Add transient oracle test**

Run `engine().run_tran(&netlist, 80.0e-12, 0.1e-12)` and compare `I(vin)` at `20.5p`, `22.5p`, `25p`, `40p`, and `80p` to Xyce 7.10. Use `interp_waveform` from the same test file.

- [x] **Step 3: Add ngspice compatibility check**

Run the same deck through ngspice-46:

```powershell
& "C:\Users\James\Desktop\ngspice-46-release\Spice64\bin\ngspice_con.exe" -b deck.cir -o deck.out
```

Accept Xyce as the primary oracle and assert ngspice agrees with Xyce within the looser of `2e-3` relative or the existing BSIM4 tolerance at the chosen points.

### Task 6: Add Mode-2 Gate Noise

**Files:**
- Modify: `crates/rspice-core/src/engine/advanced/noise.rs`
- Modify: `crates/rspice-core/tests/bsim4_native.rs`

- [x] **Step 1: Add the effective `.rg` thermal source**

For `RGATEMOD=2`, use ngspice `b4noi.c:236-243`:

```rust
let t0 = 1.0 + inst.gate_conductance / op.gcrg.max(1e-300);
let effective_gate_noise_g = inst.gate_conductance / (t0 * t0);
```

Stamp the noise source between gate-prime and gate-ext with the instance multiplier applied once, matching the existing mode-1 `.rg` naming.

- [x] **Step 2: Add the noise regression**

Add a `.noise v(out) vin dec 1 1e6 1e6` deck using `RGATEMOD=2` and assert the `m1.rg` or `m1.__rg` contribution matches ngspice within `5e-3` relative.

### Task 7: Verification Gates

- [x] Run: `cargo test -p rspice-core --test bsim4_native rgatemod2 -- --nocapture`
- [x] Run: `cargo test -p rspice-core --test bsim4_native rgatemod -- --nocapture`
- [x] Run: `cargo test -p rspice-core --test bsim4_native -- --nocapture`
- [x] Run: `cargo test -p rspice-core --lib bsim4v8 -- --nocapture`
- [x] Run: `cargo test -p rspice-core --lib noise -- --nocapture`
- [x] Run: `cargo fmt --all -- --check`
- [x] Run: `git diff --check`
- [x] Run the full ngspice regression only as release:

```powershell
$env:CARGO_PROFILE_RELEASE_LTO='false'
$env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS='32'
cargo test --release -p rspice-core --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture
```

The full ngspice gate only counts when output shows `Finished release profile [optimized]` and `target\release\deps\ngspice_regression-...exe`.

## Self-Review

- Spec coverage: mode 2 construction, DC, AC, transient, and noise are covered; mode 3 and NQS are explicitly out of scope and remain rejected.
- Red-flag scan: no forbidden placeholder language remains.
- Type consistency: the plan uses existing `Bsim4v8Op`, `Bsim4v8Device`, `Bsim4v8Model`, `engine()`, `Netlist::parse`, and `interp_waveform` names from the current codebase.
