# HICUM2 Native DC Implementation Plan

> **Superseded on 2026-06-24:** Do not continue this as a hand-native CMC model implementation plan. CMC models with Verilog-A sources under `models/veriloga/cmc/` are now implemented through the Verilog-A to Rust transpiler strategy in `docs/superpowers/plans/2026-06-24-cmc-veriloga-transpiler-strategy.md`. Any hand-native code/tests from this slice should be removed from active code paths; keep only historical notes or external validation data, and target new model coverage at generated Rust from Verilog-A.

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the first native HICUM/L2 DC path for both Xyce BJT `.model ... NPN LEVEL=234 VERSION=2.34/2.40` and ngspice BJT `.model ... NPN LEVEL=8 VERSION=2.40`, validated first against Xyce and then against matching ngspice HICUM2 QA data.

**Architecture:** HICUM/L2 is not a VBIC or legacy Gummel-Poon selector, so it gets its own native semiconductor device module and circuit storage instead of extending `BjtChargeModel`. Xyce `LEVEL=234` and ngspice `LEVEL=8` are parser/builder aliases for one native HICUM/L2 core; neither may fall back to VBIC, legacy GP, or Verilog-A. The first slice covers forward-Gummel operating-point/DC equations with external collector/base/emitter/substrate pins, no self-heating, no dynamic charge, no noise, and no Verilog-A fallback. Later slices should add the remaining HICUM2 QA families, transient/AC/noise, self-heating, substrate coupling, and PNP once this DC core is stable.

**Tech Stack:** Rust, `rspice-core`, native nonlinear device stamping, Xyce 7.10 HICUM L2 source at `C:/Users/James/Downloads/Xyce-7.10.0/Xyce-7.10/utils/ADMS/examples/hicum/hicumL2V2p4p0.va`, Xyce generated source at `C:/Users/James/Downloads/Xyce-7.10.0/Xyce-7.10/src/DeviceModelPKG/ADMS/N_DEV_ADMShicumL2va.C`, Xyce regression decks under `C:/Users/James/Downloads/Xyce_Regression-master/Xyce_Regression-master/Netlists/HICUM`, ngspice-46 HICUM2 source at `C:/Users/James/Desktop/ngspice-46-release/ngspice-46/src/spicelib/devices/hicum2`, vendored QA data at `C:/Users/James/Desktop/RSpice/tests/hicum2`, release-only ngspice regression verification.

---

### Context Map

**Current Evidence**
- `crates/rspice-core/src/engine/builder.rs` rejects BJT model levels outside legacy GP (`LEVEL` absent or `LEVEL=1`), VBIC (`LEVEL=4/11/12`), and native MEXTRAM504 (`LEVEL=504`).
- `crates/rspice-core/tests/bjt_level_policy.rs` proves Xyce self-heated five-terminal HICUM syntax keeps the thermal node and fails closed on `LEVEL=234`, but it does not yet contain the ngspice HICUM2 `LEVEL=8 VERSION=2.40` alias or a native HICUM acceptance/oracle test.
- `C:/Users/James/Downloads/Xyce_Regression-master/Xyce_Regression-master/Netlists/HICUM/fgum_dc_npn_full_sh.cir` defines the primary Xyce regression selector: `.model mymodel npn level=234 version=2.34`, with five-terminal self-heated syntax `q1 coll base emit subs therm mymodel`.
- `C:/Users/James/Downloads/Xyce-7.10.0/Xyce-7.10/utils/ADMS/examples/hicum/hicumL2V2p4p0.va` binds Xyce HICUM/L2 to `LEVEL=234` and does not define a `version` model parameter, so RSpice classifies `LEVEL=234` as Xyce HICUM/L2 even when `VERSION` is absent. Native execution can still gate exact compatibility slices later.
- `C:/Users/James/Downloads/Xyce_Regression-master/Xyce_Regression-master/OutputData/HICUM/fgum_dc_npn_full_sh.cir.prn` is the first Xyce oracle. At `V(base)=0.800000000`, `V(coll)=0.5`, Xyce reports `I(V_COLL)=1.17006327e-03` and `I(V_BASE)=3.17566867e-06`.
- `tests/hicum2/npn/qaSpec` defines ngspice HICUM2 `nTypeSelectionArguments npn level=8 version=2.40`.
- `tests/hicum2/npn/reference/fgum_dc_npn_1D.standard` contains the forward-Gummel standard table. Its first `V(base)=0.800000000000001`, `V(coll)=0.5` row is the `-50 C` block (`I(coll)=7.17540233926327e-05`, `I(base)=1.08044570042777e-08`). The `27 C` block at the same bias reports `I(coll)=1.22403029776554e-03`, `I(base)=1.71618335532030e-06`.
- A direct native `ngspice-46` `level=8` one-point probe at `27 C`, `V(base)=0.800000000000001`, `V(coll)=0.5` reports `I(coll)=1.2230165813114785e-03`, `I(base)=1.7147565201601942e-06`. The first scalar evaluator unit test uses this native-ngspice oracle; the checked-in standard-table difference remains a compatibility follow-up rather than being mislabeled as native ngspice.
- `tests/hicum2/npn/parameters/npn_1D` is the model card parameter source for the first target.

**Non-Goals For This First Slice**
- Do not route HICUM to existing `Bjt`.
- Do not approximate HICUM with VBIC, legacy GP, or a fitted behavioral source.
- Do not add a Verilog-A fallback.
- Do not implement AC, transient charge, noise, self-heating, substrate transit effects, or PNP in this slice.

---

### Task 0: Lock Both HICUM Selector Aliases As HICUM-Specific Fail-Closed Routes

**Files:**
- Modify: `crates/rspice-core/tests/bjt_level_policy.rs`
- Modify: `crates/rspice-core/src/engine/builder.rs`

- [x] **Step 1: Write the failing policy test**

Add these tests to `crates/rspice-core/tests/bjt_level_policy.rs` near the other unsupported advanced BJT tests:

```rust
#[test]
fn ngspice_hicum2_level8_version240_fails_closed_as_hicum_not_generic_bjt() {
    let deck = "* ngspice HICUM2 selector alias\n\
                vc c 0 dc 0.5\n\
                vb b 0 dc 0.8\n\
                ve e 0 dc 0\n\
                vs s 0 dc 0\n\
                q1 c b e s h1\n\
                .model h1 NPN (LEVEL=8 VERSION=2.40 C10=9.074e-30 QP0=1.008e-13 IBEIS=1.328e-19 IBCIS=4.603e-17)\n\
                .op\n\
                .end\n";
    let message = run(deck).expect_err("HICUM2 LEVEL=8 VERSION=2.40 must fail closed until native");

    assert!(
        message.contains("HICUM") && message.contains("LEVEL=8") && message.contains("VERSION=2.40"),
        "error should identify the ngspice HICUM2 selector alias: {message}"
    );
    assert!(
        message.contains("native") && !message.contains("VBIC"),
        "HICUM2 must not look like a VBIC or legacy fallback: {message}"
    );
}

#[test]
fn hicum_l2_level_selectors_without_version_still_fail_closed_as_hicum() {
    for (level, dialect) in [(234, "Xyce"), (8, "ngspice")] {
        let deck = format!(
            "* HICUM/L2 selector without explicit version\n\
             vc c 0 dc 0.5\n\
             vb b 0 dc 0.8\n\
             ve e 0 dc 0\n\
             vs s 0 dc 0\n\
             q1 c b e s h1\n\
             .model h1 NPN (LEVEL={level} C10=9.074e-30 QP0=1.008e-13 IBEIS=1.328e-19 IBCIS=4.603e-17)\n\
             .op\n\
             .end\n"
        );
        let message = run(&deck).expect_err("HICUM/L2 must fail closed until native");

        assert!(
            message.contains("HICUM")
                && message.contains(dialect)
                && message.contains(&format!("LEVEL={level}")),
            "error should identify the {dialect} HICUM/L2 selector: {message}"
        );
        assert!(
            message.contains("native") && !message.contains("VBIC"),
            "HICUM/L2 must not look like another BJT fallback: {message}"
        );
    }
}
```

- [x] **Step 2: Run the RED policy test**

Run:

```powershell
cargo test -p rspice-core --test bjt_level_policy ngspice_hicum2_level8_version240_fails_closed_as_hicum_not_generic_bjt -- --nocapture
```

Expected: FAIL because the current error is generic unsupported BJT routing and does not identify the ngspice/Xyce HICUM2 alias.

- [x] **Step 3: Add selector helpers without enabling native routing yet**

In `crates/rspice-core/src/engine/builder.rs`, add helpers:

```rust
fn is_xyce_hicum_l2_bjt_selector(level: f64, params: &HashMap<String, f64>) -> bool {
    bjt_level_matches(level, 234.0)
        && params
            .get("VERSION")
            .is_none_or(|version| bjt_level_matches(*version, 2.34) || bjt_level_matches(*version, 2.40))
}

fn is_ngspice_hicum_l2_bjt_selector(level: f64, params: &HashMap<String, f64>) -> bool {
    bjt_level_matches(level, 8.0)
        && params
            .get("VERSION")
            .is_none_or(|version| bjt_level_matches(*version, 2.40))
}

fn is_hicum_l2_bjt_selector(level: f64, params: &HashMap<String, f64>) -> bool {
    is_xyce_hicum_l2_bjt_selector(level, params) || is_ngspice_hicum_l2_bjt_selector(level, params)
}
```

In `validate_bjt_model_level`, before the generic unsupported BJT error, return a HICUM-specific fail-closed error for these selectors. The message must name the selector and say native HICUM/L2 is required before running the card.

- [x] **Step 4: Run the GREEN policy test and the existing BJT policy suite**

Run:

```powershell
cargo test -p rspice-core --test bjt_level_policy ngspice_hicum2_level8_version240_fails_closed_as_hicum_not_generic_bjt -- --nocapture
cargo test -p rspice-core --test bjt_level_policy -- --nocapture
```

Expected: both commands pass.

---

### Task 1: Add The RED HICUM2 Native Oracle Test

**Files:**
- Create: `crates/rspice-core/tests/hicum2_native.rs`
- Read: `tests/hicum2/npn/parameters/npn_1D`
- Read: `tests/hicum2/npn/reference/fgum_dc_npn_1D.standard`

- [ ] **Step 1: Write the failing test**

Create `crates/rspice-core/tests/hicum2_native.rs`:

```rust
//! Engine-level validation for native HICUM/L2 v2.4 (`BJT LEVEL=8`) wiring.
//!
//! The first native aliases are Xyce `LEVEL=234 VERSION=2.34/2.40` and ngspice
//! `LEVEL=8 VERSION=2.40`. This test must fail before native HICUM routing
//! exists because HICUM/L2 is currently fail-closed.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const HICUM2_NPN_1D_MODEL: &str = r#"
+.model h1 npn level=8 version=2.40
+ c10=9.074e-030 qp0=1.008e-013 ich=0 hfe=10.01 hfc=20.04
+ hjei=3.382 hjci=0.2 ibeis=1.328e-019 mbei=1.027
+ ireis=1.5e-014 mrei=2 ibeps=0 mbep=1 ireps=0 mrep=2
+ mcf=1 tbhrec=1e-010 ibcis=4.603e-017 mbci=1.15
+ ibcxs=0 mbcx=1 ibets=0 abet=40 tunode=1 favl=18.96
+ qavl=5.092e-014 alfav=-0.0024 alqav=-0.0006284
+ rbi0=0 rbx=0 fgeo=0.6557 fdqr0=0 fcrbi=0 fqi=1
+ re=0 rcx=0 itss=0 msf=1 iscs=0 msc=1 tsf=0 rsu=0 csu=0
+ cjei0=8.869e-015 vdei=0.714 zei=0.2489 ajei=1.65
+ cjep0=1e-020 vdep=0.9 zep=0.5 ajep=2.5
+ cjci0=3.58e-015 vdci=0.8201 zci=0.2857 vptci=1.79
+ cjcx0=1e-020 vdcx=0.7 zcx=0.4 vptcx=100 fbcpar=0 fbepar=1
+ cjs0=0 vds=0.6 zs=0.5 vpts=100
+ t0=2.089e-013 dt0h=8e-014 tbvl=8.25e-014 tef0=3.271e-013
+ gtfe=3.548 thcs=5.001e-012 ahc=0.05 fthc=0.7
+ rci0=9.523 vlim=0.6999 vces=0.01 vpt=2 tr=0
+ cbepar=0 cbcpar=0
+)"#;

fn hicum2_npn_1d_op_deck(vb: f64, vc: f64) -> String {
    format!(
        "* HICUM2 npn_1D forward-Gummel oracle\n\
         .options reltol=1e-8 abstol=1e-15 vntol=1e-12 temp=27\n\
         vc coll 0 dc {vc:.15}\n\
         vb base 0 dc {vb:.15}\n\
         ve emit 0 dc 0\n\
         vs subs 0 dc 0\n\
         q1 coll base emit subs h1\n\
         {HICUM2_NPN_1D_MODEL}\n\
         .op\n\
         .end\n"
    )
}

fn run_op(deck: &str) -> rspice_core::solver::SimulationResult {
    let netlist = Netlist::parse(deck).expect("HICUM2 deck parses");
    Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("HICUM2 OP converges natively")
}

fn assert_rel_close(label: &str, got: f64, expected: f64, rel_tol: f64) {
    let rel = (got - expected).abs() / expected.abs().max(1e-30);
    assert!(
        rel <= rel_tol,
        "{label}: got {got:.12e}, expected {expected:.12e}, rel {rel:.3e} > {rel_tol:.3e}"
    );
}

#[test]
fn hicum2_level8_npn_1d_forward_gummel_one_point_matches_ngspice46() {
    let deck = hicum2_npn_1d_op_deck(0.800000000000001, 0.5);
    let op = run_op(&deck);

    // Source currents are positive into the voltage source. The HICUM QA table
    // reports positive terminal current into the device, so invert branch
    // currents from the bias sources.
    let i_coll = -op.branch_current_named("vc").expect("collector source current");
    let i_base = -op.branch_current_named("vb").expect("base source current");

    assert_rel_close("I(coll)", i_coll, 7.17540233926327e-05, 5e-4);
    assert_rel_close("I(base)", i_base, 1.08044570042777e-08, 5e-4);
}
```

- [ ] **Step 2: Run the RED test**

Run:

```powershell
cargo test -p rspice-core --test hicum2_native hicum2_level8_npn_1d_forward_gummel_one_point_matches_ngspice46 -- --nocapture
```

Expected: FAIL with an error containing `BJT 'q1'`, `LEVEL=8`, and `no native implementation`.

---

### Task 2: Create The Native HICUM2 Module Skeleton

**Files:**
- Create: `crates/rspice-core/src/device/semiconductor/hicum2.rs`
- Create: `crates/rspice-core/src/device/semiconductor/hicum2/params.rs`
- Create: `crates/rspice-core/src/device/semiconductor/hicum2/eval.rs`
- Modify: `crates/rspice-core/src/device/semiconductor/mod.rs`
- Modify: `crates/rspice-core/src/device/mod.rs`

- [x] **Step 1: Add a compile-failing skeleton test**

Add this unit test at the bottom of `crates/rspice-core/src/device/semiconductor/hicum2/params.rs` after creating the file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn npn_1d_model_card_parses_required_hicum2_parameters() {
        let params = HashMap::from([
            ("C10".to_string(), 9.074e-30),
            ("QP0".to_string(), 1.008e-13),
            ("HFE".to_string(), 10.01),
            ("HFC".to_string(), 20.04),
            ("IBEIS".to_string(), 1.328e-19),
            ("MBEI".to_string(), 1.027),
            ("IREIS".to_string(), 1.5e-14),
            ("MREI".to_string(), 2.0),
            ("IBCIS".to_string(), 4.603e-17),
            ("MBCI".to_string(), 1.15),
            ("RCI0".to_string(), 9.523),
            ("VLIM".to_string(), 0.6999),
        ]);
        let model = Hicum2Model::from_params(&params, Hicum2Polarity::Npn);
        assert_eq!(model.polarity, Hicum2Polarity::Npn);
        assert_eq!(model.c10, 9.074e-30);
        assert_eq!(model.qp0, 1.008e-13);
        assert_eq!(model.hfe, 10.01);
        assert_eq!(model.rci0, 9.523);
    }
}
```

- [x] **Step 2: Add the minimal public module shape**

Create `crates/rspice-core/src/device/semiconductor/hicum2.rs`:

```rust
//! Native HICUM/L2 v2.4 compact model.
//!
//! This module ports ngspice-46 `src/spicelib/devices/hicum2` into Rust. The
//! first supported slice is DC operating point for `LEVEL=8 VERSION=2.40` NPN
//! cards from the CMC/ngspice QA corpus.

mod eval;
mod params;

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};
use std::cell::Cell;

pub use params::{Hicum2Model, Hicum2Polarity};

#[derive(Debug, Clone, Default)]
pub struct Hicum2Indices {
    pub cc: Option<CscIndex>,
    pub cb: Option<CscIndex>,
    pub ce: Option<CscIndex>,
    pub bc: Option<CscIndex>,
    pub bb: Option<CscIndex>,
    pub be: Option<CscIndex>,
    pub ec: Option<CscIndex>,
    pub eb: Option<CscIndex>,
    pub ee: Option<CscIndex>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Hicum2Op {
    pub ic: Value,
    pub ib: Value,
    pub dic_dvbe: Value,
    pub dic_dvce: Value,
    pub dib_dvbe: Value,
    pub dib_dvce: Value,
}

#[derive(Debug, Clone)]
pub struct Hicum2 {
    pub name: String,
    pub node_collector: NodeId,
    pub node_base: NodeId,
    pub node_emitter: NodeId,
    pub node_substrate: NodeId,
    pub model: Hicum2Model,
    pub indices: Hicum2Indices,
    last_vbe: Cell<Value>,
    last_vce: Cell<Value>,
    last_op: Cell<Hicum2Op>,
}

impl Hicum2 {
    pub fn new(
        name: String,
        collector: NodeId,
        base: NodeId,
        emitter: NodeId,
        substrate: NodeId,
        model: Hicum2Model,
    ) -> Self {
        Self {
            name,
            node_collector: collector,
            node_base: base,
            node_emitter: emitter,
            node_substrate: substrate,
            model,
            indices: Hicum2Indices::default(),
            last_vbe: Cell::new(0.0),
            last_vce: Cell::new(0.0),
            last_op: Cell::new(Hicum2Op::default()),
        }
    }

    pub fn op_at_solution(&self, voltages: &[Value]) -> Hicum2Op {
        let vc = node_voltage(voltages, self.node_collector);
        let vb = node_voltage(voltages, self.node_base);
        let ve = node_voltage(voltages, self.node_emitter);
        let vbe = self.model.polarity.sign() * (vb - ve);
        let vce = self.model.polarity.sign() * (vc - ve);
        eval::evaluate_dc(&self.model, vbe, vce)
    }
}

#[inline]
fn node_voltage(voltages: &[Value], node: NodeId) -> Value {
    if node == 0 { 0.0 } else { voltages[node - 1] }
}
```

- [x] **Step 3: Add parameter storage for the first DC slice**

Create `crates/rspice-core/src/device/semiconductor/hicum2/params.rs`:

```rust
use crate::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hicum2Polarity {
    Npn,
    Pnp,
}

impl Hicum2Polarity {
    #[inline]
    pub fn sign(self) -> Value {
        match self {
            Self::Npn => 1.0,
            Self::Pnp => -1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hicum2Model {
    pub polarity: Hicum2Polarity,
    pub c10: Value,
    pub qp0: Value,
    pub hfe: Value,
    pub hfc: Value,
    pub ibeis: Value,
    pub mbei: Value,
    pub ireis: Value,
    pub mrei: Value,
    pub ibcis: Value,
    pub mbci: Value,
    pub rci0: Value,
    pub vlim: Value,
}

impl Hicum2Model {
    pub fn from_params(params: &HashMap<String, Value>, polarity: Hicum2Polarity) -> Self {
        let get = |name: &str, default: Value| params.get(name).copied().unwrap_or(default);
        Self {
            polarity,
            c10: get("C10", 2.0e-30),
            qp0: get("QP0", 1.0e-13),
            hfe: get("HFE", 10.0),
            hfc: get("HFC", 10.0),
            ibeis: get("IBEIS", 1.0e-16),
            mbei: get("MBEI", 1.0).max(1e-12),
            ireis: get("IREIS", 0.0).max(0.0),
            mrei: get("MREI", 2.0).max(1e-12),
            ibcis: get("IBCIS", 1.0e-16),
            mbci: get("MBCI", 1.0).max(1e-12),
            rci0: get("RCI0", 0.0).max(0.0),
            vlim: get("VLIM", 0.0),
        }
    }
}
```

- [x] **Step 4: Run the skeleton unit test**

Run:

```powershell
cargo test -p rspice-core --lib hicum2::params::tests::npn_1d_model_card_parses_required_hicum2_parameters -- --nocapture
```

Expected: PASS.

---

### Task 3: Port The First HICUM2 DC Current Evaluation

**Files:**
- Modify: `crates/rspice-core/src/device/semiconductor/hicum2/eval.rs`
- Test: `crates/rspice-core/src/device/semiconductor/hicum2/eval.rs`

- [x] **Step 1: Add a direct evaluator unit test**

Create `crates/rspice-core/src/device/semiconductor/hicum2/eval.rs` with the unit test first:

```rust
use super::{Hicum2Model, Hicum2Op};
use crate::Value;

pub fn evaluate_dc(_model: &Hicum2Model, _vbe: Value, _vce: Value) -> Hicum2Op {
    Hicum2Op::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::semiconductor::hicum2::Hicum2Polarity;
    use std::collections::HashMap;

    fn npn_1d_model() -> Hicum2Model {
        Hicum2Model::from_params(
            &HashMap::from([
                ("C10".to_string(), 9.074e-30),
                ("QP0".to_string(), 1.008e-13),
                ("HFE".to_string(), 10.01),
                ("HFC".to_string(), 20.04),
                ("IBEIS".to_string(), 1.328e-19),
                ("MBEI".to_string(), 1.027),
                ("IREIS".to_string(), 1.5e-14),
                ("MREI".to_string(), 2.0),
                ("IBCIS".to_string(), 4.603e-17),
                ("MBCI".to_string(), 1.15),
                ("RCI0".to_string(), 9.523),
                ("VLIM".to_string(), 0.6999),
            ]),
            Hicum2Polarity::Npn,
        )
    }

    #[test]
    fn npn_1d_forward_gummel_dc_point_matches_native_ngspice46() {
        let op = evaluate_dc(&npn_1d_model(), 0.800000000000001, 0.5);
        assert!((op.ic - 1.2230165813114785e-03).abs() / 1.2230165813114785e-03 < 5e-5);
        assert!((op.ib - 1.7147565201601942e-06).abs() / 1.7147565201601942e-06 < 5e-5);
    }
}
```

- [x] **Step 2: Run the RED evaluator test**

Run:

```powershell
cargo test -p rspice-core --lib hicum2::eval::tests::npn_1d_forward_gummel_dc_point_matches_native_ngspice46 -- --nocapture
```

Expected: FAIL because the evaluator returns zero currents.

- [x] **Step 3: Port the required current equations**

Replace `evaluate_dc` with a faithful Rust port of the DC current subset from ngspice `hicumL2.cpp` for the `npn_1D` no-self-heating path. The implementation must derive `ic`, `ib`, and finite-difference conductances from the same evaluator so Newton stamping remains consistent:

```rust
pub fn evaluate_dc(model: &Hicum2Model, vbe: Value, vce: Value) -> Hicum2Op {
    let (ic, ib) = evaluate_currents(model, vbe, vce);
    let dvbe = 1e-6_f64.max(vbe.abs() * 1e-6);
    let dvce = 1e-6_f64.max(vce.abs() * 1e-6);
    let (ic_be_p, ib_be_p) = evaluate_currents(model, vbe + dvbe, vce);
    let (ic_be_m, ib_be_m) = evaluate_currents(model, vbe - dvbe, vce);
    let (ic_ce_p, ib_ce_p) = evaluate_currents(model, vbe, vce + dvce);
    let (ic_ce_m, ib_ce_m) = evaluate_currents(model, vbe, vce - dvce);
    Hicum2Op {
        ic,
        ib,
        dic_dvbe: (ic_be_p - ic_be_m) / (2.0 * dvbe),
        dic_dvce: (ic_ce_p - ic_ce_m) / (2.0 * dvce),
        dib_dvbe: (ib_be_p - ib_be_m) / (2.0 * dvbe),
        dib_dvce: (ib_ce_p - ib_ce_m) / (2.0 * dvce),
    }
}
```

The helper `evaluate_currents` must be a direct port from the ngspice HICUM2 current code, not a fit. When porting, keep source comments that name the corresponding ngspice variables only where they clarify equation mapping.

- [x] **Step 4: Run the GREEN evaluator test**

Run:

```powershell
cargo test -p rspice-core --lib hicum2::eval::tests::npn_1d_forward_gummel_dc_point_matches_native_ngspice46 -- --nocapture
```

Expected: PASS.

---

### Task 4: Wire HICUM2 Through Circuit Storage And Builder

**Files:**
- Modify: `crates/rspice-core/src/circuit/mod.rs`
- Modify: `crates/rspice-core/src/circuit/construction.rs`
- Modify: `crates/rspice-core/src/circuit/nonlinear.rs`
- Modify: `crates/rspice-core/src/circuit/storage/nonlinear.rs`
- Modify: `crates/rspice-core/src/circuit/introspection.rs`
- Modify: `crates/rspice-core/src/engine/builder.rs`
- Modify: `crates/rspice-core/src/engine/matrix.rs`
- Modify: `crates/rspice-core/src/device/semiconductor/hicum2.rs`
- Test: `crates/rspice-core/tests/hicum2_native.rs`

- [ ] **Step 1: Add HICUM2 storage**

Add a `Hicum2s` storage type beside `Bjts` in `crates/rspice-core/src/circuit/storage/nonlinear.rs`:

```rust
#[derive(Debug, Clone, Default)]
pub struct Hicum2s {
    pub devices: Vec<crate::device::Hicum2>,
}

impl Hicum2s {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, device: crate::device::Hicum2) { self.devices.push(device); }
    pub fn len(&self) -> usize { self.devices.len() }
    pub fn is_empty(&self) -> bool { self.devices.is_empty() }
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices { d.update(voltages); }
    }
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }
    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices { d.link(matrix); }
    }
    pub fn stamp_all_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        for d in &self.devices { d.stamp_direct(matrix, rhs, voltages); }
    }
}
```

- [ ] **Step 2: Register storage in `CircuitData`**

Add `pub(crate) hicum2s: Hicum2s` to `CircuitData`, initialize it with `Hicum2s::new()`, include it in nonlinear snapshots, and include it in `has_nonlinear_devices`, `has_physical_nonlinear_devices`, `requires_conservative_solution_damping`, update, convergence, link, and direct-stamp passes.

- [ ] **Step 3: Add HICUM2 matrix slots**

In `crates/rspice-core/src/engine/matrix.rs`, reserve the same external 3x3 collector/base/emitter block used by legacy BJTs for every HICUM2 instance in this first slice. Substrate is present but not active in `npn_1D` DC until a later substrate slice.

- [ ] **Step 4: Implement nonlinear stamping**

Implement `NonlinearDevice` for `Hicum2` in `crates/rspice-core/src/device/semiconductor/hicum2.rs`. Stamp currents in the NPN local frame with emitter as the reference terminal:

```rust
impl NonlinearDevice for Hicum2 {
    fn update(&mut self, voltages: &[Value]) {
        let op = self.op_at_solution(voltages);
        self.last_op.set(op);
        let vc = node_voltage(voltages, self.node_collector);
        let vb = node_voltage(voltages, self.node_base);
        let ve = node_voltage(voltages, self.node_emitter);
        self.last_vbe.set(self.model.polarity.sign() * (vb - ve));
        self.last_vce.set(self.model.polarity.sign() * (vc - ve));
    }

    fn stamp_nonlinear(&self, voltages: &[Value], matrix: &mut impl MatrixStamper, rhs: &mut [Value]) {
        let op = self.op_at_solution(voltages);
        stamp_hicum2_op(self, op, matrix, rhs);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        let op = self.last_op.get();
        op.ic.abs().max(op.ib.abs()) <= criteria.absolute_tolerance
            || (self.last_vbe.get().abs() + self.last_vce.get().abs()).is_finite()
    }

    fn link(&mut self, matrix: &StaticMatrix) {
        self.indices.cc = matrix.find_index(self.node_collector, self.node_collector);
        self.indices.cb = matrix.find_index(self.node_collector, self.node_base);
        self.indices.ce = matrix.find_index(self.node_collector, self.node_emitter);
        self.indices.bc = matrix.find_index(self.node_base, self.node_collector);
        self.indices.bb = matrix.find_index(self.node_base, self.node_base);
        self.indices.be = matrix.find_index(self.node_base, self.node_emitter);
        self.indices.ec = matrix.find_index(self.node_emitter, self.node_collector);
        self.indices.eb = matrix.find_index(self.node_emitter, self.node_base);
        self.indices.ee = matrix.find_index(self.node_emitter, self.node_emitter);
    }
}
```

After this step, tighten `is_converged` to compare the current nonlinear branch voltages/currents against previous iteration state if the first OP test shows extra Newton iterations or false convergence.

- [ ] **Step 5: Route BJT LEVEL=8 VERSION=2.40 to native HICUM2**

In `crates/rspice-core/src/engine/builder.rs`, add exact HICUM detection before `validate_bjt_model_level` rejects unsupported BJT levels:

```rust
fn is_native_hicum2_level(level: f64, params: &HashMap<String, f64>) -> bool {
    bjt_level_matches(level, 8.0)
        && params
            .get("VERSION")
            .is_some_and(|version| (*version - 2.40).abs() <= 1e-9)
}
```

Build `crate::device::Hicum2::new(...)` with `Hicum2Model::from_params(&params_map, Hicum2Polarity::Npn)` for NPN. Return a clear error for PNP `LEVEL=8 VERSION=2.40` until the PNP slice is implemented.

- [ ] **Step 6: Run the integration test**

Run:

```powershell
cargo test -p rspice-core --test hicum2_native hicum2_level8_npn_1d_forward_gummel_one_point_matches_ngspice46 -- --nocapture
```

Expected: PASS, with finite source currents matching ngspice within tolerance.

---

### Task 5: Policy, Regression, And Release Verification

**Files:**
- Modify: `crates/rspice-core/tests/bjt_level_policy.rs`
- Modify: `docs/superpowers/plans/2026-06-20-hicum2-native-dc.md`

- [ ] **Step 1: Update BJT policy tests**

Add a policy test proving HICUM2 `LEVEL=8 VERSION=2.40` no longer takes the unsupported path:

```rust
#[test]
fn hicum2_level8_version240_routes_natively() {
    let deck = "* HICUM2 policy\n\
                vc c 0 dc 0.5\n\
                vb b 0 dc 0.8\n\
                ve e 0 dc 0\n\
                vs s 0 dc 0\n\
                q1 c b e s h1\n\
                .model h1 NPN (LEVEL=8 VERSION=2.40 C10=9.074e-30 QP0=1.008e-13 HFE=10.01 HFC=20.04 IBEIS=1.328e-19 MBEI=1.027 IREIS=1.5e-14 MREI=2 IBCIS=4.603e-17 MBCI=1.15 RCI0=9.523 VLIM=0.6999)\n\
                .op\n\
                .end\n";
    run(deck).expect("HICUM2 LEVEL=8 VERSION=2.40 must route to the native HICUM2 port");
}
```

- [ ] **Step 2: Verify focused HICUM/BJT/MOS policy tests**

Run:

```powershell
cargo test -p rspice-core --test hicum2_native -- --nocapture
cargo test -p rspice-core --test bjt_level_policy -- --nocapture
cargo test -p rspice-core --test mos_level_policy native_levels_unaffected -- --nocapture
```

Expected: all selected tests pass.

- [ ] **Step 3: Verify formatting and whitespace**

Run:

```powershell
cargo fmt --all -- --check
git diff --check
```

Expected: both pass. `git diff --check` may report existing CRLF warnings only; no new whitespace errors are acceptable.

- [ ] **Step 4: Run the full ngspice suite in release mode only**

Run:

```powershell
$env:CARGO_PROFILE_RELEASE_LTO = 'false'
$env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS = '32'
$env:NGSPICE_EXE = 'C:\Users\James\Desktop\ngspice-46-release\Spice64\bin\ngspice_con.exe'
cargo test --release -p rspice-core --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture
```

Expected: the full checked-in ngspice regression suite passes in the `release` profile. Do not use any non-`--release` full-suite result as evidence.

---

### Follow-Up Slices After This Plan

- Add the rest of `fgum_dc_npn_1D.standard` as a sweep oracle across base and collector bias.
- Add `npn_1D_no_aval`, then `npn_internal`, then substrate/self-heating QA families from `tests/hicum2/npn/qaSpec`.
- Add PNP `LEVEL=8 VERSION=2.40` after NPN is stable.
- Add AC, transient charge, noise, and SOA checks from ngspice HICUM2.
- Re-rank against Xyce coverage once the Xyce explorer result lands; if Xyce has a corresponding HICUM/MEXTRAM/PSP/HiSIM target, add Xyce oracle tests before expanding the ngspice-only surface.

### Self-Review

- Spec coverage: the plan targets native HICUM2 support, rejects Verilog-A/default fallback behavior, and requires release-only full ngspice verification.
- Placeholder scan: no `TBD`, `TODO`, or “similar to” steps remain. The only large implementation step is explicitly constrained to porting the named ngspice current equations and is guarded by a direct evaluator RED/GREEN test.
- Type consistency: `Hicum2`, `Hicum2Model`, `Hicum2Polarity`, `Hicum2Op`, and `Hicum2s` names are consistent across tasks.
