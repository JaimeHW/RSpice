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

    assert_rel_close(
        "I(V_C)",
        source_current(&result, "vc"),
        -7.07666883e-04,
        5e-4,
    );
    assert_rel_close(
        "I(V_B)",
        source_current(&result, "vb"),
        -5.21020532e-06,
        5e-4,
    );
    assert_rel_close(
        "I(V_E)",
        source_current(&result, "ve"),
        7.12877089e-04,
        5e-4,
    );
    assert_rel_close(
        "I(V_S)",
        source_current(&result, "vs"),
        5.45880309e-20,
        1e-2,
    );
}
