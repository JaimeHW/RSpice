# BSIM4 RGATEMOD=3 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add native BSIM4 v4.8 `RGATEMOD=3` support so level 14/54 decks with a middle-gate electrode run natively and match Xyce/ngspice reference behavior for the quasi-static, non-NQS slice.

**Architecture:** Reuse the existing BSIM4 `gcrg` evaluator introduced for `RGATEMOD=2`. `RGATEMOD=3` adds one middle-gate node between the external gate and gate-prime node; the constant electrode conductance connects external gate to middle gate, and the bias-dependent `gcrg` branch connects middle gate to gate-prime. NQS (`TRNQSMOD/ACNQSMOD=1`) remains rejected and becomes a separate charge-deficit-node slice.

**Implementation note:** This slice is complete in the current worktree, and support now extends beyond the original quasi-static-only scope. Focused verification on 2026-06-19 passed `rgatemod3`, `rgatemod`, full `bsim4_native` (58/58), `bsim4v8` lib (35 passed, 1 ignored), `noise` lib (32/32), and the release-only full ngspice suite (113/113).

**Tech Stack:** Rust, `rspice-core`, native BSIM4 v4.8 port, existing Xyce/ngspice oracle-style integration tests.

---

### Task 1: RED Construction And Oracle Tests

**Files:**
- Modify: `crates/rspice-core/tests/bsim4_native.rs`
- Reference: `crates/rspice-core/src/device/mosfet/bsim4v8/mod.rs`

- [x] **Step 1: Add RGATEMOD=3 deck helpers**

Add helpers next to the existing `models45_rgatemod2()` and `rgatemod2_common_source_deck()` helpers:

```rust
fn models45_rgatemod3() -> String {
    models45().replace(
        ".model n45 nmos level=54 version=4.8",
        ".model n45 nmos level=54 version=4.8 \
         rgatemod=3 rshg=5e8 xrcrg1=12 xrcrg2=1 xgw=0 xgl=0 ngcon=1 \
         trnqsmod=0 acnqsmod=0",
    )
}

fn rgatemod3_common_source_deck() -> String {
    format!(
        "* bsim4 rgatemod=3 common source\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.55 ac 1 pulse(0.3 0.8 20p 5p 5p 80p 200p)\n\
         vb b 0 dc 0\n\
         rd vdd out 2k\n\
         m1 out in 0 b n45 w=2u l=45n nf=2 ad=0.2p as=0.2p pd=4.4u ps=4.4u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models45_rgatemod3()
    )
}
```

- [x] **Step 2: Add the construction red test**

```rust
#[test]
fn rgatemod3_runs_without_simplified_mos_optin() {
    let deck = rgatemod3_common_source_deck();
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine().run_dc_op(&netlist);
    assert!(
        result.is_ok(),
        "RGATEMOD=3 must build and run natively, got {result:?}"
    );
}
```

- [x] **Step 3: Run the construction red test**

Run: `cargo test -p rspice-core --test bsim4_native rgatemod3_runs_without_simplified_mos_optin -- --nocapture`

Expected: FAIL with `RGATEMOD=3 is not implemented`.

- [x] **Step 4: Add AC/transient oracle tests**

Add tests mirroring the existing `rgatemod2_common_source_ac_matches_xyce710_and_ngspice46()` and `rgatemod2_transient_gate_current_matches_xyce710_and_ngspice46()` shapes, with constants generated from Xyce 7.10 and ngspice 46 for the deck above.

Expected before production changes: FAIL at construction for `RGATEMOD=3`.

### Task 2: Add Gate-Mid Topology

**Files:**
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/device.rs`
- Modify: `crates/rspice-core/src/engine/builder.rs`
- Modify: `crates/rspice-core/src/engine/matrix.rs`
- Modify: `crates/rspice-core/src/circuit/construction.rs`

- [x] **Step 1: Add the middle-gate node field**

Add `node_gate_mid: NodeId` to `Bsim4v8Device`. The constructor receives `node_gate_external`, `node_gate_mid`, and `node_gate` where `node_gate` remains gate-prime.

- [x] **Step 2: Allocate nodes in the builder**

Update `build_bsim4v8()` gate handling:

```rust
let (gate_mid, gate) = match core.model.rgate_mod {
    1 => {
        let gint = circuit.get_or_create_node(&format!("{}.__gint", element.name));
        circuit.resistors.add(
            format!("{}.__rg", element.name),
            gate_external,
            gint,
            1.0 / (core.inst.gate_conductance * multiplier),
        );
        (gate_external, gint)
    }
    2 => {
        let gint = circuit.get_or_create_node(&format!("{}.__gint", element.name));
        (gate_external, gint)
    }
    3 => {
        let gmid = circuit.get_or_create_node(&format!("{}.__gmid", element.name));
        let gint = circuit.get_or_create_node(&format!("{}.__gint", element.name));
        (gmid, gint)
    }
    _ => (gate_external, gate_external),
};
```

Pass `gate_mid` into `Bsim4v8Device::new`.

- [x] **Step 3: Reserve sparse topology**

Include `node_gate_mid` in the BSIM4 topology set in `engine/matrix.rs`, and remap it in `circuit/construction.rs`.

### Task 3: Stamp RGATEMOD=3

**Files:**
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/device.rs`
- Modify: `crates/rspice-core/src/device/mosfet/bsim4v8/mod.rs`

- [x] **Step 1: Allow mode 3 while preserving NQS rejection**

Change `validate_model()` so `model.rgate_mod > 3` is rejected and the error says `RGATEMOD=0, 1, 2, or 3`. Keep `TRNQSMOD/ACNQSMOD=1` rejected.

- [x] **Step 2: Generalize the existing gcrg terms**

Rename `rgate_mod2_terms_from()` to a mode-neutral helper such as `rgate_gcrg_terms_from()`. Keep the current PMOS and reverse-mode unit tests intact after the rename.

- [x] **Step 3: Stamp the mode-3 DC rows**

For `RGATEMOD=3`, stamp ngspice/Xyce's middle-gate branch:

- RHS: gate-mid receives `+ceqgcrg`; gate-prime receives `-ceqgcrg`.
- Conductance: gate-mid row gets `geltd + gcrg` on gate-mid, `-geltd` to gate-external, and `gcrg*` derivatives to gate-prime/drain/source/body. Gate-prime row gets `-gcrg` to gate-mid and the opposite `gcrg*` derivative contributions.
- Keep the existing intrinsic gate-current stamps on gate-prime.

Implement this using the existing `stamp()`/`stamp_rhs()` helpers and the existing `op.gcrg/gcrg*` fields.

- [x] **Step 4: Extend noise contribution**

Update `engine/advanced/noise.rs` so `RGATEMOD=3` reports the thermal noise source on the external-to-middle gate electrode branch (`m1.__rg` equivalent name or a mode-specific source name), using `inst.gate_conductance * multiplier`.

### Task 4: Verification

**Files:**
- Verify: `crates/rspice-core/tests/bsim4_native.rs`
- Verify: `crates/rspice-core/src/device/mosfet/bsim4v8/device.rs`
- Verify: `crates/rspice-core/src/engine/builder.rs`
- Verify: `crates/rspice-core/src/engine/matrix.rs`
- Verify: `crates/rspice-core/src/circuit/construction.rs`
- Verify: `crates/rspice-core/src/engine/advanced/noise.rs`

- [x] **Step 1: Run focused RGATEMOD tests**

Run: `cargo test -p rspice-core --test bsim4_native rgatemod -- --nocapture`

Expected: PASS.

- [x] **Step 2: Run BSIM4 native integration**

Run: `cargo test -p rspice-core --test bsim4_native -- --nocapture`

Expected: PASS.

- [x] **Step 3: Run BSIM4 lib and noise filters**

Run: `cargo test -p rspice-core --lib bsim4v8 -- --nocapture`

Run: `cargo test -p rspice-core --lib noise -- --nocapture`

Expected: PASS.

- [x] **Step 4: Run formatting and diff checks**

Run: `cargo fmt --all -- --check`

Run: `git diff --check`

Expected: PASS, except pre-existing CRLF warnings if they are already present and outside this slice.

- [x] **Step 5: Run full ngspice regression only in release mode**

Only after behavior changes are green locally, run:

```powershell
$env:CARGO_PROFILE_RELEASE_LTO='false'
$env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS='32'
cargo test --release -p rspice-core --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture
```

Valid evidence requires `Finished release profile [optimized]` and `TOTAL 113 tests | 113 passed`.

---

## Notes

- This slice intentionally excludes `TRNQSMOD=1` and `ACNQSMOD=1`; those require the BSIM4 charge-deficit `qNode`, `qdef/qcheq` history, and AC/transient NQS stamps.
- Do not add Verilog-A defaults or fallback behavior. `RGATEMOD=3` must be native.
- Do not commit unless the user explicitly asks; this shared worktree already has many unrelated changes.
