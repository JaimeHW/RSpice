# MEXTRAM504 Native DC Implementation Plan

> **Superseded on 2026-06-24:** Do not continue this as a hand-native CMC model implementation plan. CMC models with Verilog-A sources under `models/veriloga/cmc/` are now implemented through the Verilog-A to Rust transpiler strategy in `docs/superpowers/plans/2026-06-24-cmc-veriloga-transpiler-strategy.md`. Any hand-native code/tests from this slice should be removed from active code paths; keep only historical notes or external validation data, and target new model coverage at generated Rust from Verilog-A.

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the first native Xyce-primary MEXTRAM 504.12.1 DC path for `.model ... NPN LEVEL=504`, with no Verilog-A runtime fallback.

**Architecture:** MEXTRAM 504 is a separate advanced BJT compact model, not a VBIC or legacy Gummel-Poon selector. The first slice creates an independent native `Mextram504` device with its own internal MNA nodes, parameter storage, DC evaluator, and circuit storage, then validates source currents against Xyce 7.10 MEXTRAM regression decks. `LEVEL=505` self-heated MEXTRAM, transient charge, AC, noise, and PNP support remain separate follow-up slices.

**Tech Stack:** Rust, `rspice-core`, Xyce 7.10 NORAD binary at `C:/Users/James/Downloads/Xyce-7.10-NORAD/bin/Xyce.exe`, Xyce ADMS MEXTRAM source at `C:/Users/James/Downloads/Xyce-7.10.0/Xyce-7.10/utils/ADMS/examples/mextram_504.12.1`, Xyce regression decks at `C:/Users/James/Downloads/Xyce_Regression-master/Xyce_Regression-master/Netlists/MEXTRAM`, release-only ngspice regression verification.

---

### Context Map

**Current Evidence**
- Xyce `README_Xyce` says `bjt504` is registered as Xyce BJT device `LEVEL=504` named `MEXTRAM 504.12.1`; `bjt504t` is `LEVEL=505`.
- Xyce `bjt504.va` declares external pins `(c, b, e, s)` and internal electrical nodes `e1`, `b1`, `b2`, `c1`, `c2`, `c3`, and `c4`.
- Xyce `evaluate.inc` contributes DC branch currents on `c1-c2`, `c2-e1`, `b1-e1`, `b2-e1`, `b1-s`, `b-s`, `s-c1`, `b1-b2`, `b2-c2`, `e-e1`, `b-b1`, `b-c3` or `b-c4`, `c-c3` or `c-c4`, `c4-c1`, `b1-c4`, and `c3-c4`. Dynamic `ddt(...)` terms are ignored in this DC slice.
- The Xyce FGummel cards set `RCBLX=0` and `RCBLI=0`; Xyce still declares `c3` and `c4`, but `evaluate.inc` collapses the distributed collector branch toward `c1` in this case. Keep the native nodes present so nonzero `RCBLX/RCBLI` decks do not require a topology rewrite later.
- The FGummel model card parameter surface is `LEVEL,TREF,DTA,EXMOD,EXPHI,EXAVL,EXSUB,IS,IK,VER,VEF,BF,IBF,MLF,XIBI,IZEB,NZEB,BRI,IBR,VLR,XEXT,WAVL,VAVL,SFH,RE,RBC,RBV,RCC,RCBLX,RCBLI,RCV,SCRCV,IHC,AXI,CJE,VDE,PE,XCJE,CBEO,CJC,VDC,PC,XP,MC,XCJC,CBCO,MTAU,TAUE,TAUB,TEPI,TAUR,DEG,XREC,AQBO,AE,AB,AEX,AEPI,AC,ACBL,DAIS,DVGBF,DVGBR,VGB,VGC,VGJ,VGZEB,AVGEB,TVGEB,DVGTE,AF,KF,KFN,KAVL,ISS,ICSS,IKS,CJS,VDS,PS,VGS,AS,ASUB,MULT`. Store all of these in the native model; dynamic and noise fields can remain inactive in DC, but they must parse and round-trip through the evaluator inputs.
- Xyce accepts `MULT` as both a model and instance parameter and aliases `M` to `MULT`. Instance `M`/`MULT` overrides model `M`/`MULT`; model `MULT=1.5` matters for the FGummel cards.
- Xyce adds `trunc_ev` limiting for `V(b2,c1)`, `V(b2,c2)`, `V(b1,e1)`, and `V(b1,b2)` to improve operating-point convergence on CMC QA tests.
- Xyce clean direct one-point oracle, generated without unsupported `.options`, is:
  - Deck: direct `q1 c b e s mymodel`, `LEVEL=504`, `Vc=1.0`, `Vb=0.8`, `Ve=0`, `Vs=0`, `.dc v_b 0.8 0.8 0.1`.
  - Xyce device summary: `Q level 504 (MEXTRAM 504.12.1) 1`.
  - Source currents: `I(V_C)=-7.07666883e-04`, `I(V_B)=-5.21020532e-06`, `I(V_E)=7.12877089e-04`, `I(V_S)=5.45880309e-20`.
- Existing `crates/rspice-core/tests/bjt_level_policy.rs` currently requires `LEVEL=504` to fail closed with "no native implementation". That test must be narrowed once native MEXTRAM support exists.
- `crates/rspice-core/src/engine/builder.rs` currently accepts only legacy GP (`LEVEL` absent or `LEVEL=1`) and VBIC (`LEVEL=4/11/12`) through BJT routing.

**Non-Goals For This First Slice**
- Do not route MEXTRAM through `Bjt`, `BjtChargeModel::Vbic`, or legacy Gummel-Poon.
- Do not approximate MEXTRAM with fitted current sources.
- Do not add a Verilog-A runtime fallback.
- Do not implement `LEVEL=505`, self-heating, transient `ddt(...)` charge, AC, noise, or PNP in this slice.
- Do not treat any non-`--release` full ngspice regression-suite run as verification evidence.

---

### Task 1: Add The RED MEXTRAM 504 Xyce Oracle Test

**Files:**
- Create: `crates/rspice-core/tests/mextram504_native.rs`
- Read: `C:/Users/James/Downloads/Xyce_Regression-master/Xyce_Regression-master/Netlists/MEXTRAM/FGummel_Ib.cir`
- Read: `C:/Users/James/Downloads/Xyce_Regression-master/Xyce_Regression-master/Netlists/MEXTRAM/FGummel_Ic.cir`

- [x] **Step 1: Write the failing test**

Create `crates/rspice-core/tests/mextram504_native.rs`:

```rust
//! Engine-level validation for native MEXTRAM 504.12.1 (`BJT LEVEL=504`).
//!
//! The first oracle is a direct one-device Xyce 7.10 MEXTRAM operating point
//! derived from the official `MEXTRAM/FGummel_Ib.cir` regression card. This
//! must fail before native MEXTRAM routing exists because `LEVEL=504` is
//! currently rejected by the BJT level policy.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use rspice_core::solver::SimulationResult;

const MEXTRAM504_XYCE_MODEL: &str = r#"
.model mymodel npn level=504
+ LEVEL=504.0
+ TREF=25.00
+ DTA=0.000
+ EXMOD=1.000
+ EXPHI=1.000
+ EXAVL=0.000
+ EXSUB=0.000
+ IS=22.0E-18
+ IK=0.1
+ VER=2.5
+ VEF=44.0
+ BF=215.0
+ IBF=2.7E-15
+ MLF=2.000
+ XIBI=0.0
+ IZEB=0.0
+ NZEB=22.0
+ BRI=7.00
+ IBR=1.0E-015
+ VLR=0.2
+ XEXT=0.63
+ WAVL=1.1E-006
+ VAVL=3.0
+ SFH=0.3
+ RE=5.0
+ RBC=23.0
+ RBV=18.0
+ RCC=12.0
+ RCBLX=0.000
+ RCBLI=0.000
+ RCV=150.0
+ SCRCV=1250.0
+ IHC=4.000E-003
+ AXI=0.3
+ CJE=73.0E-015
+ VDE=950.0E-003
+ PE=400.0E-003
+ XCJE=400.0E-003
+ CBEO=0.000
+ CJC=78.0E-015
+ VDC=680.0E-003
+ PC=500.0E-003
+ XP=350.0E-003
+ MC=500.0E-003
+ XCJC=32.0E-003
+ CBCO=0.000
+ MTAU=1.0
+ TAUE=2.0E-012
+ TAUB=4.2E-12
+ TEPI=41.0E-12
+ TAUR=520.0E-012
+ DEG=0.01
+ XREC=0.1
+ AQBO=300.0E-003
+ AE=0.0E-003
+ AB=1.0
+ AEX=620.0E-003
+ AEPI=2.5
+ AC=2.0
+ ACBL=2.0
+ DAIS=0.000
+ DVGBF=50.0E-003
+ DVGBR=45.00E-003
+ VGB=1.17
+ VGC=1.18
+ VGJ=1.15
+ VGZEB=1.15
+ AVGEB=4.73E-4
+ TVGEB=636.0
+ DVGTE=50.0E-003
+ AF=2.000
+ KF=20.0E-012
+ KFN=20.0E-012
+ KAVL=1.000
+ ISS=48.0E-18
+ ICSS=-1.0
+ IKS=250.E-006
+ CJS=315.0E-015
+ VDS=620.0E-003
+ PS=340.0E-003
+ VGS=1.20
+ AS=1.580
+ ASUB=2.0
+ MULT=1.5
"#;

fn mextram504_direct_op_deck(vb: f64, vc: f64) -> String {
    format!(
        "* Xyce 7.10 direct MEXTRAM 504.12.1 DC oracle\n\
         vc c 0 dc {vc:.15}\n\
         vb b 0 dc {vb:.15}\n\
         ve e 0 dc 0\n\
         vs s 0 dc 0\n\
         q1 c b e s mymodel\n\
         {MEXTRAM504_XYCE_MODEL}\n\
         .op\n\
         .end\n"
    )
}

fn run_op(deck: &str) -> SimulationResult {
    let netlist = Netlist::parse(deck).expect("MEXTRAM deck parses");
    Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("MEXTRAM OP converges natively")
}

fn source_current(result: &SimulationResult, branch: &str) -> f64 {
    result
        .branch_current_named(branch)
        .unwrap_or_else(|| panic!("missing branch {branch} in {:?}", result.branch_names))
}

fn assert_rel_close(label: &str, got: f64, expected: f64, rel_tol: f64) {
    let rel = (got - expected).abs() / expected.abs().max(1e-30);
    assert!(
        rel <= rel_tol,
        "{label}: got {got:.12e}, expected {expected:.12e}, rel {rel:.3e} > {rel_tol:.3e}"
    );
}

#[test]
fn mextram504_level504_direct_operating_point_matches_xyce710() {
    let result = run_op(&mextram504_direct_op_deck(0.8, 1.0));

    assert_rel_close("I(V_C)", source_current(&result, "vc"), -7.07666883e-04, 5e-4);
    assert_rel_close("I(V_B)", source_current(&result, "vb"), -5.21020532e-06, 5e-4);
    assert_rel_close("I(V_E)", source_current(&result, "ve"), 7.12877089e-04, 5e-4);
    assert_rel_close("I(V_S)", source_current(&result, "vs"), 5.45880309e-20, 1e-2);
}
```

- [x] **Step 2: Run the RED test**

Run:

```powershell
cargo test -p rspice-core --test mextram504_native mextram504_level504_direct_operating_point_matches_xyce710 -- --nocapture
```

Expected: FAIL with an error containing `BJT 'q1'`, `LEVEL=504`, and `no native implementation`.

Observed: FAIL with `BJT 'Q1': model 'MYMODEL' requests LEVEL=504, which has no native implementation`.

---

### Task 2: Add Native MEXTRAM Module And Parameter Storage

**Files:**
- Create: `crates/rspice-core/src/device/semiconductor/mextram504.rs`
- Create: `crates/rspice-core/src/device/semiconductor/mextram504/params.rs`
- Create: `crates/rspice-core/src/device/semiconductor/mextram504/temp.rs`
- Create: `crates/rspice-core/src/device/semiconductor/mextram504/eval.rs`
- Modify: `crates/rspice-core/src/device/semiconductor/mod.rs`
- Modify: `crates/rspice-core/src/device/mod.rs`

**Implementation note:** The checked-in implementation intentionally goes beyond the abbreviated code sketch below by storing the full Xyce source parameter surface needed by the first native model skeleton: the FGummel card parameters plus source defaults for `XQB`, `KC`, `KE`, `FTAUN`, `TYPE`, and `GMIN`. It also includes a default-value regression test against Xyce `parameters.inc`; use `crates/rspice-core/src/device/semiconductor/mextram504/params.rs` as the source of truth for this completed task.

- [x] **Step 1: Add the parameter unit test**

Create `crates/rspice-core/src/device/semiconductor/mextram504/params.rs` with this test first:

```rust
use crate::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mextram504Polarity {
    Npn,
    Pnp,
}

impl Mextram504Polarity {
    pub fn type_sign(self) -> Value {
        match self {
            Self::Npn => 1.0,
            Self::Pnp => -1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mextram504Model {
    pub polarity: Mextram504Polarity,
    pub tref: Value,
    pub dta: Value,
    pub exmod: Value,
    pub exphi: Value,
    pub exavl: Value,
    pub exsub: Value,
    pub is_: Value,
    pub ik: Value,
    pub ver: Value,
    pub vef: Value,
    pub bf: Value,
    pub ibf: Value,
    pub mlf: Value,
    pub xibi: Value,
    pub izeb: Value,
    pub nzeb: Value,
    pub bri: Value,
    pub ibr: Value,
    pub vlr: Value,
    pub xext: Value,
    pub wavl: Value,
    pub vavl: Value,
    pub sfh: Value,
    pub re: Value,
    pub rbc: Value,
    pub rbv: Value,
    pub rcc: Value,
    pub rcblx: Value,
    pub rcbli: Value,
    pub rcv: Value,
    pub scrcv: Value,
    pub ihc: Value,
    pub axi: Value,
    pub cje: Value,
    pub vde: Value,
    pub pe: Value,
    pub xcje: Value,
    pub cbeo: Value,
    pub cjc: Value,
    pub vdc: Value,
    pub pc: Value,
    pub xp: Value,
    pub mc: Value,
    pub xcjc: Value,
    pub cbco: Value,
    pub mtau: Value,
    pub taue: Value,
    pub taub: Value,
    pub tepi: Value,
    pub taur: Value,
    pub deg: Value,
    pub xrec: Value,
    pub aqbo: Value,
    pub ae: Value,
    pub ab: Value,
    pub aex: Value,
    pub aepi: Value,
    pub ac: Value,
    pub acbl: Value,
    pub dais: Value,
    pub dvgbf: Value,
    pub dvgbr: Value,
    pub vgb: Value,
    pub vgc: Value,
    pub vgj: Value,
    pub vgzeb: Value,
    pub avgeb: Value,
    pub tvgeb: Value,
    pub dvgte: Value,
    pub af: Value,
    pub kf: Value,
    pub kfn: Value,
    pub kavl: Value,
    pub iss: Value,
    pub icss: Value,
    pub iks: Value,
    pub cjs: Value,
    pub vds: Value,
    pub ps: Value,
    pub vgs: Value,
    pub as_: Value,
    pub asub: Value,
    pub mult: Value,
}

impl Mextram504Model {
    pub fn from_params(
        params: &HashMap<String, Value>,
        instance_params: &HashMap<String, Value>,
        polarity: Mextram504Polarity,
    ) -> Self {
        let get_model = |name: &str, default: Value| params.get(name).copied().unwrap_or(default);
        let mult = instance_params
            .get("M")
            .or_else(|| instance_params.get("MULT"))
            .copied()
            .or_else(|| params.get("M").or_else(|| params.get("MULT")).copied())
            .unwrap_or(1.0)
            .max(1e-30);
        Self {
            polarity,
            tref: get_model("TREF", 25.0),
            dta: get_model("DTA", 0.0),
            exmod: get_model("EXMOD", 1.0),
            exphi: get_model("EXPHI", 1.0),
            exavl: get_model("EXAVL", 0.0),
            exsub: get_model("EXSUB", 0.0),
            is_: get_model("IS", 1e-16).max(0.0),
            ik: get_model("IK", 0.0).max(0.0),
            ver: get_model("VER", 1e30),
            vef: get_model("VEF", 1e30),
            bf: get_model("BF", 100.0).max(1e-30),
            ibf: get_model("IBF", 1e-16).max(0.0),
            mlf: get_model("MLF", 1.0),
            xibi: get_model("XIBI", 0.0),
            izeb: get_model("IZEB", 0.0).max(0.0),
            nzeb: get_model("NZEB", 1.0).max(1e-30),
            bri: get_model("BRI", 1.0).max(1e-30),
            ibr: get_model("IBR", 1e-16).max(0.0),
            vlr: get_model("VLR", 0.0),
            xext: get_model("XEXT", 1.0),
            wavl: get_model("WAVL", 0.0),
            vavl: get_model("VAVL", 1.0).max(1e-30),
            sfh: get_model("SFH", 0.0),
            re: get_model("RE", 0.0).max(0.0),
            rbc: get_model("RBC", 0.0).max(0.0),
            rbv: get_model("RBV", 0.0).max(0.0),
            rcc: get_model("RCC", 0.0).max(0.0),
            rcblx: get_model("RCBLX", 0.0).max(0.0),
            rcbli: get_model("RCBLI", 0.0).max(0.0),
            rcv: get_model("RCV", 0.0).max(0.0),
            scrcv: get_model("SCRCV", 0.0).max(0.0),
            ihc: get_model("IHC", 0.0).max(0.0),
            axi: get_model("AXI", 0.0),
            cje: get_model("CJE", 0.0).max(0.0),
            vde: get_model("VDE", 0.75).max(1e-30),
            pe: get_model("PE", 0.5),
            xcje: get_model("XCJE", 1.0),
            cbeo: get_model("CBEO", 0.0).max(0.0),
            cjc: get_model("CJC", 0.0).max(0.0),
            vdc: get_model("VDC", 0.75).max(1e-30),
            pc: get_model("PC", 0.5),
            xp: get_model("XP", 0.0),
            mc: get_model("MC", 0.5),
            xcjc: get_model("XCJC", 1.0),
            cbco: get_model("CBCO", 0.0).max(0.0),
            mtau: get_model("MTAU", 1.0),
            taue: get_model("TAUE", 0.0).max(0.0),
            taub: get_model("TAUB", 0.0).max(0.0),
            tepi: get_model("TEPI", 0.0).max(0.0),
            taur: get_model("TAUR", 0.0).max(0.0),
            deg: get_model("DEG", 0.0),
            xrec: get_model("XREC", 0.0),
            aqbo: get_model("AQBO", 0.0),
            ae: get_model("AE", 0.0),
            ab: get_model("AB", 1.0),
            aex: get_model("AEX", 0.0),
            aepi: get_model("AEPI", 1.0),
            ac: get_model("AC", 1.0),
            acbl: get_model("ACBL", 1.0),
            dais: get_model("DAIS", 0.0),
            dvgbf: get_model("DVGBF", 0.0),
            dvgbr: get_model("DVGBR", 0.0),
            vgb: get_model("VGB", 1.17),
            vgc: get_model("VGC", 1.17),
            vgj: get_model("VGJ", 1.17),
            vgzeb: get_model("VGZEB", 1.17),
            avgeb: get_model("AVGEB", 0.0),
            tvgeb: get_model("TVGEB", 0.0),
            dvgte: get_model("DVGTE", 0.0),
            af: get_model("AF", 1.0),
            kf: get_model("KF", 0.0).max(0.0),
            kfn: get_model("KFN", 0.0).max(0.0),
            kavl: get_model("KAVL", 0.0),
            iss: get_model("ISS", 0.0).max(0.0),
            icss: get_model("ICSS", 0.0),
            iks: get_model("IKS", 0.0).max(0.0),
            cjs: get_model("CJS", 0.0).max(0.0),
            vds: get_model("VDS", 0.75).max(1e-30),
            ps: get_model("PS", 0.5),
            vgs: get_model("VGS", 1.17),
            as_: get_model("AS", 1.0),
            asub: get_model("ASUB", 1.0),
            mult,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xyce_fgummel_card_parses_core_params_and_mult_alias() {
        let model_params = HashMap::from([
            ("IS".to_string(), 22.0e-18),
            ("IK".to_string(), 0.1),
            ("BF".to_string(), 215.0),
            ("VER".to_string(), 2.5),
            ("VEF".to_string(), 44.0),
            ("RE".to_string(), 5.0),
            ("RBC".to_string(), 23.0),
            ("RBV".to_string(), 18.0),
            ("RCC".to_string(), 12.0),
            ("RCV".to_string(), 150.0),
            ("MULT".to_string(), 1.5),
        ]);
        let model = Mextram504Model::from_params(
            &model_params,
            &HashMap::new(),
            Mextram504Polarity::Npn,
        );
        assert_eq!(model.polarity, Mextram504Polarity::Npn);
        assert_eq!(model.is_, 22.0e-18);
        assert_eq!(model.ik, 0.1);
        assert_eq!(model.bf, 215.0);
        assert_eq!(model.rcblx, 0.0);
        assert_eq!(model.rcbli, 0.0);
        assert_eq!(model.mult, 1.5);

        let instance_override = HashMap::from([("M".to_string(), 2.0)]);
        let overridden = Mextram504Model::from_params(
            &model_params,
            &instance_override,
            Mextram504Polarity::Npn,
        );
        assert_eq!(overridden.mult, 2.0);
    }
}
```

- [x] **Step 2: Run the parameter test**

Run:

```powershell
cargo test -p rspice-core --lib mextram504::params::tests::xyce_fgummel_card_parses_core_params_and_mult_alias -- --nocapture
```

Expected: PASS once `mextram504` is exported by `semiconductor/mod.rs` and `device/mod.rs`.

Observed: `cargo test -p rspice-core --lib mextram504 -- --nocapture` passes 5 tests, including Xyce defaults, `M`/`MULT` precedence, discrete-flag canonicalization, native polarity `TYPE`, and operating-point cache behavior.

- [x] **Step 3: Add the module shell**

Create `crates/rspice-core/src/device/semiconductor/mextram504.rs`:

```rust
//! Native MEXTRAM 504.12.1 compact model.
//!
//! This is a Rust port of the Xyce 7.10 ADMS MEXTRAM source. The first slice
//! supports DC operating point for `LEVEL=504` NPN devices with substrate pin.

mod eval;
mod params;
mod temp;

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};
use std::cell::Cell;

pub use params::{Mextram504Model, Mextram504Polarity};

#[derive(Debug, Clone, Copy)]
pub struct Mextram504Nodes {
    pub c: NodeId,
    pub b: NodeId,
    pub e: NodeId,
    pub s: NodeId,
    pub e1: NodeId,
    pub b1: NodeId,
    pub b2: NodeId,
    pub c1: NodeId,
    pub c2: NodeId,
    pub c3: NodeId,
    pub c4: NodeId,
}

#[derive(Debug, Clone, Default)]
pub struct Mextram504Indices {
    pub slots: Vec<(NodeId, NodeId, Option<CscIndex>)>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Mextram504Op {
    pub source_c: Value,
    pub source_b: Value,
    pub source_e: Value,
    pub source_s: Value,
}

#[derive(Debug, Clone)]
pub struct Mextram504 {
    pub name: String,
    pub nodes: Mextram504Nodes,
    pub model: Mextram504Model,
    pub indices: Mextram504Indices,
    last_op: Cell<Mextram504Op>,
}

impl Mextram504 {
    pub fn new(name: String, nodes: Mextram504Nodes, model: Mextram504Model) -> Self {
        Self {
            name,
            nodes,
            model,
            indices: Mextram504Indices::default(),
            last_op: Cell::new(Mextram504Op::default()),
        }
    }

    pub fn op_at_solution(&self, voltages: &[Value]) -> Mextram504Op {
        eval::evaluate_dc(&self.model, self.nodes, voltages)
    }
}
```

---

### Task 3: Add Native Storage, Internal Nodes, And LEVEL=504 Routing

**Files:**
- Modify: `crates/rspice-core/src/circuit/mod.rs`
- Modify: `crates/rspice-core/src/circuit/construction.rs`
- Modify: `crates/rspice-core/src/circuit/nonlinear.rs`
- Modify: `crates/rspice-core/src/circuit/storage/nonlinear.rs`
- Modify: `crates/rspice-core/src/circuit/introspection.rs`
- Modify: `crates/rspice-core/src/engine/builder.rs`
- Modify: `crates/rspice-core/src/engine/matrix.rs`
- Test: `crates/rspice-core/tests/bjt_level_policy.rs`

- [ ] **Step 1: Add storage**

Add a `Mextram504s` storage type beside `Bjts` in `crates/rspice-core/src/circuit/storage/nonlinear.rs`:

```rust
#[derive(Debug, Clone, Default)]
pub struct Mextram504s {
    pub devices: Vec<crate::device::Mextram504>,
}

impl Mextram504s {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, device: crate::device::Mextram504) { self.devices.push(device); }
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

Add `pub(crate) mextram504s: Mextram504s` to `CircuitData`, initialize it with `Mextram504s::new()`, include it in nonlinear snapshots, and include it in `has_nonlinear_devices`, `has_physical_nonlinear_devices`, `requires_conservative_solution_damping`, update, convergence, link, and direct-stamp passes.

- [ ] **Step 3: Allocate MEXTRAM internal nodes**

In `crates/rspice-core/src/engine/builder.rs`, when an NPN BJT model has exact `LEVEL=504`, allocate seven internal nodes named with the instance prefix:

```rust
let e1 = circuit.get_or_create_node(&format!("{name}#e1"));
let b1 = circuit.get_or_create_node(&format!("{name}#b1"));
let b2 = circuit.get_or_create_node(&format!("{name}#b2"));
let c1 = circuit.get_or_create_node(&format!("{name}#c1"));
let c2 = circuit.get_or_create_node(&format!("{name}#c2"));
let c3 = circuit.get_or_create_node(&format!("{name}#c3"));
let c4 = circuit.get_or_create_node(&format!("{name}#c4"));
```

Use these nodes to build `Mextram504Nodes { c, b, e, s, e1, b1, b2, c1, c2, c3, c4 }`.

- [ ] **Step 4: Reserve matrix positions**

In `crates/rspice-core/src/engine/matrix.rs`, reserve a dense conductance block over the 11 MEXTRAM nodes:

```rust
for device in &circuit.mextram504s.devices {
    let nodes = [
        device.nodes.c,
        device.nodes.b,
        device.nodes.e,
        device.nodes.s,
        device.nodes.e1,
        device.nodes.b1,
        device.nodes.b2,
        device.nodes.c1,
        device.nodes.c2,
        device.nodes.c3,
        device.nodes.c4,
    ];
    for &row in &nodes {
        for &col in &nodes {
            if row != 0 && col != 0 {
                add_position(row, col);
            }
        }
    }
}
```

- [ ] **Step 5: Route only exact native MEXTRAM**

Add helper logic near existing BJT level helpers in `crates/rspice-core/src/engine/builder.rs`:

```rust
fn is_native_mextram504_level(level: f64) -> bool {
    bjt_level_matches(level, 504.0)
}
```

Handle the MEXTRAM route before `validate_bjt_model_level(...)` rejects unsupported advanced levels. For this first slice:
- Accept only `crate::netlist::BjtType::Npn`.
- Require a four-terminal BJT instance so the substrate pin is explicit.
- Build a `Mextram504` instance and push it into `circuit.mextram504s`.
- Return a clear circuit error for PNP `LEVEL=504`.
- Keep `LEVEL=505` rejected until the self-heated slice is implemented.

- [ ] **Step 6: Update policy tests**

In `crates/rspice-core/tests/bjt_level_policy.rs`, change `unsupported_advanced_bjt_level_is_rejected` so it still checks HICUM `LEVEL=234` and MEXTRAM self-heated `LEVEL=505`, but no longer expects `LEVEL=504` to fail.

Add:

```rust
#[test]
fn mextram504_level504_routes_to_native_device() {
    let deck = "* MEXTRAM504 policy route\n\
                vc c 0 dc 1.0\n\
                vb b 0 dc 0.8\n\
                ve e 0 dc 0\n\
                vs s 0 dc 0\n\
                q1 c b e s qmod\n\
                .model qmod NPN (LEVEL=504 IS=22e-18 BF=215 IK=0.1 RE=5 RBC=23 RBV=18 RCC=12 RCV=150 MULT=1.5)\n\
                .op\n\
                .end\n";
    let built = build(deck).expect("LEVEL=504 must build as native MEXTRAM");
    assert_eq!(built.device_summary().count_by_kind("MEXTRAM504"), 1);
}
```

If `device_summary().count_by_kind(...)` is not available, use the local introspection helper already used in the adjacent policy tests; do not add a public API just for this assertion.

---

### Task 4: Port The DC Evaluator And Stamping

**Files:**
- Modify: `crates/rspice-core/src/device/semiconductor/mextram504/eval.rs`
- Modify: `crates/rspice-core/src/device/semiconductor/mextram504.rs`
- Test: `crates/rspice-core/tests/mextram504_native.rs`

**Current status note:** The evaluator is intentionally still fail-closed. It now panics if called directly while incomplete so no future routing can silently simulate zero MEXTRAM currents. Completed scaffolding includes Xyce `expLin`, `trunc_ev`, typed limiting primitives, 12-node static order including `noi`, static branch-contribution projection for all collector-collapse cases, the Xyce noise-node ground stamp, and DC temperature/MULT scaling for the branch values used by the first stamp path.

- [ ] **Step 1: Add evaluator RED test points**

Add this unit test in `crates/rspice-core/src/device/semiconductor/mextram504/eval.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::semiconductor::mextram504::{
        Mextram504Model, Mextram504Nodes, Mextram504Polarity,
    };
    use std::collections::HashMap;

    fn fgummel_model() -> Mextram504Model {
        Mextram504Model::from_params(
            &HashMap::from([
                ("IS".to_string(), 22.0e-18),
                ("IK".to_string(), 0.1),
                ("BF".to_string(), 215.0),
                ("VER".to_string(), 2.5),
                ("VEF".to_string(), 44.0),
                ("RE".to_string(), 5.0),
                ("RBC".to_string(), 23.0),
                ("RBV".to_string(), 18.0),
                ("RCC".to_string(), 12.0),
                ("RCV".to_string(), 150.0),
                ("MULT".to_string(), 1.5),
            ]),
            &HashMap::new(),
            Mextram504Polarity::Npn,
        )
    }

    fn direct_nodes() -> Mextram504Nodes {
        Mextram504Nodes {
            c: 1,
            b: 2,
            e: 0,
            s: 0,
            e1: 3,
            b1: 4,
            b2: 5,
            c1: 6,
            c2: 7,
            c3: 8,
            c4: 9,
        }
    }

    #[test]
    fn direct_operating_point_currents_match_xyce710_after_newton_solve() {
        let voltages = vec![1.0, 0.8, 0.0, 0.8, 0.8, 1.0, 1.0, 1.0, 1.0];
        let op = evaluate_dc(&fgummel_model(), direct_nodes(), &voltages);
        assert!((op.source_c - -7.07666883e-04).abs() / 7.07666883e-04 < 5e-4);
        assert!((op.source_b - -5.21020532e-06).abs() / 5.21020532e-06 < 5e-4);
    }
}
```

Expected initial result: FAIL because `evaluate_dc` returns zero or is not implemented. If the exact internal-node voltages differ after the native Newton solve, keep the engine-level Xyce oracle as the authoritative RED/GREEN test and convert this evaluator unit test into a residual/Jacobian consistency test instead.

- [ ] **Step 2: Port Xyce DC include files in order**

Partial progress completed before the full evaluator port:
- [x] Xyce `frontdef.inc` `expLin` behavior with `VEXLIM=400`.
- [x] Xyce `bjt504.va` `trunc_ev` limiter behavior, including typed NPN/PNP polarity handling.
- [x] Xyce static `I(pos,neg)` branch projection for common branches and all `RCBLX`/`RCBLI` collector topology cases.
- [x] Xyce static node order includes `noi`; `I(noi,GND)` is represented as a positive noise-node ground contribution.
- [x] Xyce `initialize.inc`/`tscaling.inc` DC temperature/MULT scaling foothold for Kelvin temperature, `DTA`, resistances, conductances, and fixed overlap capacitance scaling.
- [x] Xyce generated probe extraction conventions for raw solution voltages, including typed pre-multiply only for `V(b1,b2)`, `V(b1,e1)`, `V(b2,c2)`, and `V(b2,c1)`.
- [x] Xyce lead-current projection that sums static node contributions into external `c/b/e/s` leads through a node map with no sign flip.

Port these Xyce files into Rust functions in `eval.rs` and `temp.rs` in the listed order:

```text
C:/Users/James/Downloads/Xyce-7.10.0/Xyce-7.10/utils/ADMS/examples/mextram_504.12.1/frontdef.inc
C:/Users/James/Downloads/Xyce-7.10.0/Xyce-7.10/utils/ADMS/examples/mextram_504.12.1/initialize.inc
C:/Users/James/Downloads/Xyce-7.10.0/Xyce-7.10/utils/ADMS/examples/mextram_504.12.1/tscaling.inc
C:/Users/James/Downloads/Xyce-7.10.0/Xyce-7.10/utils/ADMS/examples/mextram_504.12.1/evaluate.inc
```

Implement `temp::scale_dc` from `tscaling.inc` and call it from `evaluate_dc` before evaluating branch currents. Implement `evaluate_dc` so it returns all static branch contributions from `evaluate.inc` and omits only lines whose contribution is explicitly `ddt(...)`, noise, or self-heating `dt`. Preserve the Xyce `trunc_ev` limiter behavior for the four limited junction voltages. Preserve the `expLin` behavior from `frontdef.inc` with `VEXLIM=400`. Use finite-difference partials over the same evaluator for the first slice, then replace with analytic derivatives once DC parity is stable.

- [ ] **Step 3: Stamp static branches**

In `mextram504.rs`, implement `NonlinearDevice` for `Mextram504`. Stamp each branch current as a two-terminal nonlinear current between the same nodes named in Xyce `evaluate.inc`. The first DC branch list is:

```text
c1-c2: Ic1c2
c2-e1: In
b1-e1: Ib1_s
b2-e1: Ib1 + Ib2 - Izteb
b1-s: Isub
b-s: XIsub
s-c1: Isf
b1-b2: Ib1b2
b2-c2: -Iavl
e-e1: Vee1 / RE_TM
b-b1: Vbb1 / RBC_TM
b-c3 or b-c4: XIex
c-c3 or c-c4: Vcc3 * GCCxx_TM
c4-c1: Vc4c1 * GCCin_TM
b1-c4 or b1-c1: Ib3 + Iex
c3-c4 or c3-c1: Vc3c4 * GCCex_TM
```

For every stamped nonlinear current, add conductance entries from finite-difference partials with respect to the 11-node voltage vector and an equivalent RHS current so Newton sees a consistent linearization.

- [ ] **Step 4: Run the GREEN engine oracle**

Run:

```powershell
cargo test -p rspice-core --test mextram504_native mextram504_level504_direct_operating_point_matches_xyce710 -- --nocapture
```

Expected: PASS with relative current errors under `5e-4`.

---

### Task 5: Expand Xyce DC Coverage

**Files:**
- Modify: `crates/rspice-core/tests/mextram504_native.rs`

- [ ] **Step 1: Add selected sweep oracle points**

Add this oracle table to `mextram504_native.rs` after the direct one-point test passes:

```rust
const XYCE_FGUMMEL_SELECTED_POINTS: &[(f64, f64, f64, f64)] = &[
    // temp_c, vb, direct source I(V_C), direct source I(V_B)
    (25.0, 0.5, -7.27826372e-09, -1.11556048e-10),
    (25.0, 0.7, -1.53806093e-05, -1.07489865e-07),
    (25.0, 0.8, -7.07666883e-04, -5.21020532e-06),
    (25.0, 1.2, -5.79467939e-02, -2.46698041e-03),
    (100.0, 0.7, -9.98823506e-04, -6.16700449e-06),
];
```

Before using the sweep points as strict assertions, regenerate direct Xyce decks with `.options temp=<temp>` or `.step temp list 25 100` and direct branch probes, because the original FGummel wrapper prints probe currents with a different sign convention. Keep the direct `Vb=0.8`, `Vc=1.0`, `25C` constants above as the first authoritative assertion.

- [ ] **Step 2: Add direct Xyce regeneration command to test comments**

Add this comment above the oracle table:

```rust
// Oracle regeneration:
// $xyce = 'C:\Users\James\Downloads\Xyce-7.10-NORAD\bin\Xyce.exe'
// Use the model card from Xyce_Regression-master/Netlists/MEXTRAM/FGummel_Ib.cir.
// Run a direct q1 c b e s mymodel deck with .print dc V(b) I(v_c) I(v_b) I(v_e) I(v_s).
```

---

### Task 6: Final Verification

**Files:**
- Modify: `docs/superpowers/plans/2026-06-20-mextram504-native-dc.md`

- [ ] **Step 1: Run focused tests**

Run:

```powershell
cargo test -p rspice-core --test mextram504_native -- --nocapture
cargo test -p rspice-core --test bjt_level_policy -- --nocapture
```

Expected: PASS.

- [ ] **Step 2: Run formatting and whitespace checks**

Run:

```powershell
cargo fmt --all -- --check
git diff --check
```

Expected: PASS. If `git diff --check` reports pre-existing warnings outside the MEXTRAM files, record them and verify no new MEXTRAM whitespace errors were added.

- [ ] **Step 3: Run the full ngspice suite in release mode only**

Run:

```powershell
$env:CARGO_PROFILE_RELEASE_LTO = 'false'
$env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS = '32'
$env:NGSPICE_EXE = 'C:\Users\James\Desktop\ngspice-46-release\Spice64\bin\ngspice_con.exe'
cargo test --release -p rspice-core --test ngspice_regression test_full_ngspice_suite_summary -- --nocapture
```

Expected: the full checked-in ngspice regression suite passes in the `release` profile. Non-release full-suite runs are invalid for this gate.

---

### Follow-Up Slices After This Plan

- Add direct Xyce sweep regeneration for all `FGummel_Ib.cir` and `FGummel_Ic.cir` points.
- Add low-current and reverse-Gummel Xyce MEXTRAM decks.
- Add `LEVEL=505` self-heated `bjt504t` support.
- Add transient charge from the `ddt(...)` branches in `evaluate.inc`.
- Add AC and noise from `noise.inc` and the Xyce AC regression decks.
- Add PNP/no-flip cards once NPN DC is stable.

### Self-Review

- Spec coverage: the plan targets native MEXTRAM 504 support, uses Xyce as the primary oracle, keeps ngspice as a release-only regression gate, and explicitly forbids Verilog-A runtime fallback.
- Red-flag scan: no deferred-work tokens remain. The only large task is explicitly bounded to named Xyce include files and concrete static branch contributions.
- Type consistency: `Mextram504`, `Mextram504Model`, `Mextram504Polarity`, `Mextram504Nodes`, `Mextram504Op`, and `Mextram504s` names are consistent across tasks.
