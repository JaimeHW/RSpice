# JFET2 Strict Upgrade Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add native JFET level-2 support for both ngspice Parker-Skellern and Xyce modified-Shockley semantics, defaulting to the richer ngspice Parker-Skellern model.

**Architecture:** Keep the existing level-1 JFET path unchanged and add explicit level-2 variants inside the existing `Jfet` device boundary. The default `SimulationConfig` uses `SpiceDialect::BestAvailable`, which routes `NJF`/`PJF LEVEL=2` to ngspice's richer Parker-Skellern model; an explicit Xyce compatibility mode routes the same cards to Xyce's modified-Shockley model. Validation is kept separate: ngspice Parker-Skellern tests prove the default model and Xyce regression decks prove the Xyce variant.

**Tech Stack:** Rust, `rspice-core`, ngspice-46 reference source/executable, Xyce 7.10 source/regression decks, existing RSpice `Engine` integration tests.

---

### Task 1: Capture JFET2 Failure And Ngspice Oracle

**Files:**
- Create: `crates/rspice-core/tests/jfet2_native.rs`
- Reference: `C:/Users/James/Desktop/ngspice-46-release/ngspice-46/src/spicelib/devices/jfet2/jfet2load.c`
- Reference: `C:/Users/James/Desktop/ngspice-46-release/ngspice-46/src/spicelib/devices/jfet2/psmodel.c`

- [x] **Step 1: Write the failing OP test**

```rust
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn op_current(deck: &str, source_name: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("op converges");
    let idx = result
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(source_name))
        .unwrap_or_else(|| panic!("missing branch {source_name:?}: {:?}", result.branch_names));
    -result.branch_currents[idx]
}

#[test]
fn jfet2_level2_op_matches_ngspice46() {
    let deck = "\
* JFET2 level-2 Parker-Skellern OP
vd d 0 dc 5
vg g 0 dc -0.25
vs s 0 dc 0
j1 d g s psmod area=1
.model psmod NJF(level=2 beta=1e-3 vt0=-2 lambda=0.02 vbi=1 is=1e-14 n=1 \
                 p=2 q=2 xi=1000 z=1 vst=0.1 mvst=0.05 mxi=0.0 \
                 lfgam=0.01 lfg1=0.002 lfg2=0.001 ibd=1e-12 vbd=10)
.op
.end
";
    let id = op_current(deck, "vd");
    let ngspice46_id = 3.472_026_20e-3;
    let rel = (id - ngspice46_id).abs() / ngspice46_id.abs();
    assert!(
        rel < 2e-4,
        "JFET2 OP drain current mismatch: rspice={id:.9e} ngspice={ngspice46_id:.9e} rel={rel:.3e}"
    );
}
```

- [x] **Step 2: Run the test to verify it fails**

Run: `cargo test -p rspice-core --test jfet2_native jfet2_level2_op_matches_ngspice46 -- --nocapture`

Expected: FAIL because RSpice still uses the level-1 JFET path and does not match the ngspice-46 Parker-Skellern drain current.

### Task 2: Add JFET2 Parameter And Temperature State

**Files:**
- Modify: `crates/rspice-core/src/device/mosfet/jfet.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/params.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/construction/model_params.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/construction/defaults.rs`
- Create: `crates/rspice-core/src/device/mosfet/jfet/jfet2.rs`

- [x] **Step 1: Add `JfetChannelModel::ParkerSkellern`**

Extend the channel-model enum and `JfetParams` with the ngspice JFET2 parameters: `acgam`, `capds`, `delta`, `hfeta`, `hfe1`, `hfe2`, `hfgam`, `hfg1`, `hfg2`, `ibd`, `lfgam`, `lfg1`, `lfg2`, `mvst`, `mxi`, `p`, `q`, `taud`, `taug`, `vbd`, `ver`, `vst`, `xc`, `xi`, and `z`.

- [x] **Step 2: Add derived temperature helpers**

Provide the ngspice temperature-adjusted values through focused helpers: `t_sat_cur`, `t_gate_pot`, `t_cgs`, `t_cgd`, `cor_dep_cap`, `vcrit`, `xiwoo`, `d3`, `alpha`, `za`, and `pave`.

- [x] **Step 3: Map `.model` aliases**

Accept ngspice aliases exactly: `VT0`/`VTO`, `VBI`/`PB`, `KF`, `AF`, `CDS`, `CGS`, `CGD`, and all JFET2 names from `jfet2parm.h`.

### Task 3: Port DC Operating Equations

**Files:**
- Modify: `crates/rspice-core/src/device/mosfet/jfet/jfet2.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/bias.rs`

- [x] **Step 1: Implement `jfet2_temperature_update`**

Translate `jfet2temp.c` into Rust and call it when model or instance temperature changes. Preserve ngspice constants and clamp only for non-finite numerical protection.

- [x] **Step 2: Implement `jfet2_ps_ids`**

Translate `PSids` from ngspice `psmodel.c` for DC operating point mode. Return `(ids, igs, igd, ggs, ggd, gm, gds)` in RSpice's external sign convention.

- [x] **Step 3: Wire `compute_operating_terms`**

Dispatch `JfetChannelModel::ParkerSkellern` to the new JFET2 evaluator, preserving reverse-mode behavior from `jfet2load.c`.

- [x] **Step 4: Run the OP test to verify it passes**

Run: `cargo test -p rspice-core --test jfet2_native jfet2_level2_op_matches_ngspice46 -- --nocapture`

Expected: PASS.

### Task 4: Wire Builder And Reports

**Files:**
- Modify: `crates/rspice-core/src/engine/builder.rs`
- Modify: `crates/rspice-core/src/circuit/introspection.rs`
- Modify: `crates/rspice-core/tests/jfet2_native.rs`

- [x] **Step 1: Remove the downgrade warning path**

When `NJF`/`PJF` has `LEVEL=2`, call `enable_jfet2_model()` before applying model params and do not warn about level-1 fallback.

- [x] **Step 2: Add an OP report assertion**

Extend the test to call `run_dc_op_with_report` and assert that `j1` reports a JFET2/Parker-Skellern kind rather than generic level-1 JFET.

### Task 5: Add AC And Transient Coverage

**Files:**
- Modify: `crates/rspice-core/src/device/mosfet/jfet/jfet2.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/capacitance.rs`
- Modify: `crates/rspice-core/src/engine/ac.rs`
- Modify: `crates/rspice-core/src/engine/transient/state.rs`
- Modify: `crates/rspice-core/tests/jfet2_native.rs`

- [x] **Step 1: Port `PScharge`/`qgg`**

Return JFET2 `Cgs`, `Cgd`, and `Cds` using ngspice's Parker-Skellern charge model.

- [x] **Step 2: Add AC oracle test**

Use a fixed-bias common-source deck and compare `V(out)` magnitude/phase at three frequencies against ngspice-46.

- [x] **Step 3: Add transient oracle test**

Use a small pulse-driven common-source deck with `CGS`, `CGD`, and `CDS`; compare selected time points or crossing times against ngspice-46.

### Task 6: Validate And Document

**Files:**
- Modify: `README.md`
- Modify: `crates/rspice-core/README.md`
- Modify: `docs/manual/02-netlists.md`

- [x] **Step 1: Update docs**

Replace the current `LEVEL=2` fallback language with native JFET2 support notes and list the supported parameter set.

- [x] **Step 2: Run focused validation**

Run:

```powershell
cargo test -p rspice-core --test jfet2_native -- --nocapture
cargo test -p rspice-core --test ngspice_regression test_ngspice_jfet_suite -- --nocapture
cargo test -p rspice-core --test mos_level_policy native_levels_unaffected
cargo test -p rspice-core
```

Expected: all focused tests pass; full `rspice-core` test suite passes or any unrelated pre-existing failures are documented with exact output.

### Task 7: Tighten Parker-Skellern Fidelity Gaps

**Files:**
- Modify: `crates/rspice-core/tests/jfet2_native.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/construction/defaults.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/construction/model_params.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/bias.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/capacitance.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/jfet2.rs`
- Modify: `crates/rspice-core/src/engine/builder.rs`
- Modify: `crates/rspice-core/src/engine/transient.rs`
- Modify: `crates/rspice-core/src/engine/transient/state.rs`
- Modify: `crates/rspice-core/src/engine/transient/truncation.rs`

- [x] **Step 1: Add failing tests for review gaps**

Add tests for `AREA != 1`, circuit `.options temp`, PJF AC feedback, and a stronger charge/memory transient deck.

Run: `cargo test -p rspice-core --test jfet2_native -- --nocapture`

Expected: FAIL for AREA, temperature, and charge/memory transient before implementation changes.

- [x] **Step 2: Fix AREA and temperature routing**

Scale Parker-Skellern channel `BETA` by instance `AREA`, store the builder-resolved analysis temperature on `Jfet`, convert JFET model-card `TNOM` to Kelvin, and evaluate JFET2 DC/AC/transient helpers with that analysis temperature unless instance `TEMP` overrides it.

- [x] **Step 3: Port exact `PScharge` history**

Return exact `(qgs, qgd, cgs, cgd)` from the ngspice `qgg`/`PScharge` equations. In transient, use previous accepted `QGS/QGD` and previous `VGS/VGD` to partition charge instead of using generic `q = C * V`.

- [x] **Step 4: Port TAUG and TAUD transient memory**

Use ngspice's fourth-power `h = (tau / (tau + dt/4))^4` in Parker-Skellern threshold feedback and thermal power averaging. Store previous accepted power in `JfetTransientHistory`.

- [x] **Step 5: Fix PJF AC feedback sign and guards**

Use internal-polarity drain current in Parker-Skellern AC thermal feedback and guard singular denominators explicitly while preserving base conductance terms.

- [x] **Step 6: Verify Parker-Skellern tests pass**

Run: `cargo test -p rspice-core --test jfet2_native -- --nocapture`

Expected: PASS.

### Task 8: Add Xyce Modified-Shockley JFET2 Variant

**Files:**
- Modify: `crates/rspice-core/src/engine/config.rs`
- Modify: `crates/rspice-core/src/engine/config_resolver.rs`
- Modify: `crates/rspice-core/src/engine/builder.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/construction/defaults.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/bias.rs`
- Modify: `crates/rspice-core/src/device/mosfet/jfet/capacitance.rs`
- Modify: `crates/rspice-core/src/circuit/introspection.rs`
- Modify: `crates/rspice-core/tests/jfet2_native.rs`

- [x] **Step 1: Add internal dialect knob**

Add a public `SpiceDialect` enum to `SimulationConfig` with default `SpiceDialect::BestAvailable`. Add `SimulationConfigOverrides::spice_dialect` so frontends can request `SpiceDialect::Xyce`.

- [x] **Step 2: Add Xyce JFET2 channel variant**

Add `JfetChannelModel::XyceModifiedShockley` and an `enable_xyce_jfet2_model()` constructor. Keep `enable_jfet2_model()` as the ngspice Parker-Skellern default.

- [x] **Step 3: Route LEVEL=2 by dialect**

In the builder, route `NJF`/`PJF LEVEL=2` to Parker-Skellern when the resolved JFET2 selector follows `BestAvailable`/`Ngspice`, and to Xyce modified-Shockley when it follows `Xyce`.

- [x] **Step 4: Port Xyce DC equations and add capacitance oracle coverage**

Port the level-2 branch from `Xyce-7.10/src/DeviceModelPKG/OpenModels/N_DEV_JFET.C` into a focused helper that supports DC operating point and AC capacitance stamps. The DC path is covered by the Xyce `NJFET_DC/njfet-2109.cir` and `PJFET_DC/pjfet-2108.cir` decks; add a non-zero-capacitance oracle before marking this fully complete.

- [x] **Step 5: Add Xyce-variant tests**

Add tests that construct `Engine::new(SimulationConfig { spice_dialect: SpiceDialect::Xyce, ..Default::default() })` and verify that `LEVEL=2` reports `JFET2 (Xyce modified Shockley)` and produces the Xyce regression DC currents for `njfet-2109.cir` and `pjfet-2108.cir` once the local Xyce binary is available.

### Task 9: Final Verification

**Files:**
- Test-only command outputs.

- [ ] **Step 1: Run focused RSpice verification**

Run:

```powershell
cargo test -p rspice-core --test jfet2_native -- --nocapture
cargo test -p rspice-core --test ngspice_regression test_ngspice_jfet_suite -- --nocapture
cargo test -p rspice-core --test mos_level_policy native_levels_unaffected -- --nocapture
cargo fmt --all -- --check
```

Expected: PASS.

- [ ] **Step 2: Run full core and release ngspice verification**

Run:

```powershell
cargo test -p rspice-core
cargo test -p rspice-core --release --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture
```

Expected: PASS, including `TOTAL 113 tests | 113 passed`.

- [x] **Step 3: Run Xyce corresponding regression decks**

Build Xyce if needed, then run and verify:

```powershell
Xyce.exe C:\Users\James\Downloads\Xyce_Regression-master\Xyce_Regression-master\Netlists\NJFET_DC\njfet-2109.cir
Xyce.exe C:\Users\James\Downloads\Xyce_Regression-master\Xyce_Regression-master\Netlists\PJFET_DC\pjfet-2108.cir
```

Compare generated `.prn` files with the suite's `OutputData/NJFET_DC/njfet-2109.cir.prn` and `OutputData/PJFET_DC/pjfet-2108.cir.prn` using the Xyce regression verifier.
