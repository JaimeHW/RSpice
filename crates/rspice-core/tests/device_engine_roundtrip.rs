//! Device-model checks whose evidence is a deck the engine solves.
//!
//! The model-level assertions stay in each device module's own unit tests.
//! A check that needs `Engine` to elaborate and solve a deck lives here,
//! outside the crate, so `device` never reaches up to the engine to check
//! itself.

use rspice_core::{Engine, Netlist, SimulationConfig};

/// The floating-body DC equilibrium must rise into forward bias as the gate
/// turns the device on with the drain at 1.5 V (the RampVg2 bias), driven by
/// impact ionization charging the body until the source diode clamps it.
/// This is the DC anchor the RampVg2 transient relaxes toward.
#[test]
fn floating_body_dc_equilibrium_is_forward_biased() {
    let model = n1_card_text();
    let solve_vb = |vg: f64| -> f64 {
        let deck = format!(
            "*ramp op\nvd d 0 dc 1.5\nvs s 0 dc 0\nve e 0 dc 0\nvg g 0 dc {vg}\nm1 d g s e n1 w=10u l=0.25u\n.option gmin=1e-20 itl1=500\n.op\n{model}\n.end\n"
        );
        let netlist = Netlist::parse(&deck).expect("parse");
        let engine = Engine::new(SimulationConfig::default());
        let res = engine.run_dc_op(&netlist).expect("dc op");
        res.try_voltage_named("m1.__body.internal")
            .expect("body node")
    };
    // Off (Vg=0): body near the t=0 RampVg2 reference (~0.092 V).
    let vb0 = solve_vb(0.0);
    assert!((vb0 - 0.0917).abs() < 5e-3, "Vb(Vg=0)={vb0:.4e}");
    // Saturation (Vg=1): strong impact ionization forward-biases the body.
    let vb1 = solve_vb(1.0);
    assert!(vb1 > 0.4 && vb1 < 0.7, "Vb(Vg=1)={vb1:.4e}");
}

/// End-to-end check: a tied-body NMOS at the t4 first operating point
/// (Vg=0, Vd=0.05, Vb=-0.3) must reproduce the checked-in ngspice reference
/// drain current to within 0.1%. Exercises the builder dispatch, the SOI
/// stamping consistency (cdreq vs. linearized conductances), and the
/// `VOFF=-.14` model-card sign parsing all at once.
#[test]
fn t4_first_point_matches_ngspice_reference() {
    let model = n1_card_text();
    let deck = format!(
        "*t4 single point\nvd d 0 dc 0.05\nvs s 0 dc 0\nve e 0 dc 0\nvg g 0 dc 0\nvb b 0 dc -0.3\nm1 d g s e b n1 w=10u l=0.25u\n.option gmin=1e-25 itl1=500\n.op\n{model}\n.end\n"
    );
    let netlist = Netlist::parse(&deck).expect("parse t4 single point deck");
    let engine = Engine::new(SimulationConfig::default());
    let res = engine.run_dc_op(&netlist).expect("dc op");
    // vs is the 2nd voltage source (branch index 1); i(vs) == drain current.
    let i_vs = res.branch_currents[1];
    // ngspice t4.out row 0: vs#branch = 4.146227e-12 at Vg=0, Vb=-0.3.
    let reference = 4.146227e-12;
    assert!(
        (i_vs - reference).abs() <= 1e-3 * reference.abs(),
        "i(vs)={i_vs:.6e} vs reference {reference:.6e}"
    );
}

/// The `N1` NMOS model card text from `tests/bsim3soidd/nmosdd.mod`, kept in
/// sync with `n1_params` in the B3SOIDD unit tests; written out here so the
/// decks above do not depend on the test working directory.
fn n1_card_text() -> &'static str {
    ".Model N1 NMOS Level=56\n\
     +TNOM=27 TOX=4.5E-09 TSI=5e-8 TBOX=8E-08\n\
     +MOBMOD=0 CAPMOD=3 SHMOD=0\n\
     +PARAMCHK=0 WINT=0 LINT=-2E-08\n\
     +VTH0=.52 K1=.39 K2=.1 K3=0\n\
     +KB1=.95 K3B=2.2 NLX=7.2E-08\n\
     +DVT0=.55 DVT1=.28 DVT2=-1.4\n\
     +DVT0W=0 DVT1W=0 DVT2W=0\n\
     +NCH=3.3E+17 NSUB=1E+15 NGATE=1E+20\n\
     +DVBD0=60.0 DVBD1=1.1 VBSA=0.0\n\
     +KB3=2.2 DELP=0.02\n\
     +ABP=0.9 MXC=0.9 ADICE0=0.93\n\
     +KBJT1=1.0E-08 EDL=.0000005\n\
     +NDIODE=1.13 NTUN=14.0\n\
     +ISBJT=2e-6 ISDIF=1e-6 ISTUN=0.0 ISREC=1e-5\n\
     +XBJT=0.01 XDIF=0.01 XREC=0.01 XTUN=0.001\n\
     +U0=352 UA=1.3E-11 UB=1.7E-18 UC=-4E-10\n\
     +W0=1.16E-06 AGS=.25 A1=0 A2=1\n\
     +B0=.01 B1=10\n\
     +RDSW=700 PRWG=0 PRWB=-.2 WR=1\n\
     +RBODY=0.0 RBSH=0.0\n\
     +A0=1.4 KETA=-.67 VSAT=135000\n\
     +DWG=0 DWB=0\n\
     +ALPHA0=0.0 ALPHA1=1.5 BETA0=20.5\n\
     +AII=1.2 BII=0.1e-7 CII=0.8 DII=0.6\n\
     +VOFF=-.14 NFACTOR=.7 CDSC=.00002 CDSCB=0\n\
     +CDSCD=0 CIT=0\n\
     +PCLM=2.9 PVAG=12 PDIBLC1=.18 PDIBLC2=.004\n\
     +PDIBLCB=-.234 DROUT=.2\n\
     +DELTA=.01 ETA0=.01 ETAB=0\n\
     +DSUB=.3 RTH0=.006\n\
     +CLC=.0000001 CLE=.6 CF=1E-20 CKAPPA=.6\n\
     +CGDL=1E-20 CGSL=1E-20 KT1=-.3 KT1L=0\n\
     +KT2=.022 UTE=-1.5 UA1=4.31E-09 UB1=-7.61E-18\n\
     +UC1=-5.6E-11 PRT=760 AT=22400\n\
     +CGSO=1e-10 CGDO=1e-10 CJSWG=5e-10 TT=3e-10\n\
     +ASD=0.3 CSDESW=1e-12\n"
}

// ===================== Verilog-A cross-check =====================
//
// An optional Verilog-A BSIM4.8 source, when supplied locally, is a sibling
// implementation rather than the transcription source for the native port.
// This cross-check reports native-vs-VA deltas at the same fixture biases
// without forcing agreement; the hard bound only guards against gross
// divergence.

#[cfg(feature = "veriloga")]
mod bsim4_veriloga_cross_check {
    use rspice_core::device::mosfet::bsim4v8::{
        Bsim4v8, Bsim4v8Bias, Bsim4v8Geometry, Bsim4v8Model,
    };
    use rspice_core::{Engine, Netlist, Value};
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    const T300: Value = 300.15;
    const GMIN: Value = 1e-12;

    const BSIM4_VA_ENV: &str = "RSPICE_BSIM4_VA";

    fn bsim4_va_path() -> Option<PathBuf> {
        if let Some(raw) = std::env::var_os(BSIM4_VA_ENV) {
            let path = PathBuf::from(raw);
            assert!(
                path.is_file(),
                "{BSIM4_VA_ENV} must point at an externally supplied BSIM4 Verilog-A source file: {}",
                path.display()
            );
            return Some(path);
        }

        let fallback = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models/veriloga/bsim4.va");
        fallback.is_file().then_some(fallback)
    }

    /// Terminal drain current -i(vd) of the VA model at a bias point, the
    /// exact recipe of tests/veriloga_bsim4_oracle.rs.
    fn va_drain_current(model: &Path, vgs: Value, vds: Value, temp_c: Option<Value>) -> Value {
        let temp_line = match temp_c {
            Some(t) => format!(".options temp={t}\n"),
            None => String::new(),
        };
        let deck = format!(
            "* bsim4 va bias point\n\
             {temp_line}\
             vg g 0 {vgs}\n\
             vd d 0 {vds}\n\
             XM1 d g 0 0 bsim4va l=1e-7 w=1e-6\n\
             .va \"{}\" bsim4va\n\
             .end\n",
            model.display().to_string().replace('\\', "/")
        );
        let netlist = Netlist::parse(&deck).expect("parse");
        let result = Engine::default()
            .run_tran(&netlist, 2e-9, 1e-9)
            .expect("bias point must converge");
        let idx = result
            .branch_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("vd"))
            .expect("vd branch");
        -result.branch_currents[idx]
            .last()
            .copied()
            .expect("samples")
    }

    /// Terminal drain current of the native port with the all-defaults
    /// card (the VA deck gives no model parameters): channel current plus
    /// the reverse drain-junction current, i.e. what -i(vd) measures.
    fn native_drain_current(tnom_k: Value, temp_k: Value, vgs: Value, vds: Value) -> Value {
        let model = Arc::new(Bsim4v8Model::from_params(&HashMap::new(), false, tnom_k));
        let dev = Bsim4v8::new(
            "m1".to_string(),
            model,
            Bsim4v8Geometry {
                l: 0.1e-6,
                w: 1e-6,
                ..Bsim4v8Geometry::default()
            },
            temp_k,
        )
        .expect("native default device");
        let op = dev
            .eval(Bsim4v8Bias { vds, vgs, vbs: 0.0 }, GMIN, false)
            .expect("eval");
        op.cd - op.cbd
    }

    /// Native (C-transcription) vs Verilog-A at the fixture bias points.
    /// Run with `cargo test -p rspice-core --features veriloga --test
    /// device_engine_roundtrip -- --ignored --nocapture` to see the delta
    /// table.
    #[test]
    #[ignore = "requires an externally supplied models/veriloga/bsim4.va and the veriloga engine"]
    fn native_vs_veriloga_bias_points() {
        let Some(model) = bsim4_va_path() else {
            eprintln!("bsim4.va not present; skipping VA cross-check");
            return;
        };

        // (label, vgs, vds, temp_c). The temperature rows mirror the VA
        // fixture's tnom=25 protocol; the rest run at the 27C default.
        let points: [(&str, Value, Value, Option<Value>); 10] = [
            ("idvg deep subthreshold", 0.0, 1.2, None),
            ("idvg subthreshold", 0.3, 1.2, None),
            ("idvg moderate inversion", 0.6, 1.2, None),
            ("idvg onset", 0.9, 1.2, None),
            ("idvg strong inversion", 1.2, 1.2, None),
            ("idvd linear", 1.2, 0.1, None),
            ("idvd saturation onset", 1.2, 0.6, None),
            ("temp -40C", 1.0, 1.2, Some(-40.0)),
            ("temp 27C", 1.0, 1.2, Some(27.0)),
            ("temp 125C", 1.0, 1.2, Some(125.0)),
        ];

        println!(
            "{:<26} {:>13} {:>13} {:>9}",
            "bias", "native", "veriloga", "delta"
        );
        let mut worst: Value = 0.0;
        for (label, vgs, vds, temp_c) in points {
            let (tnom_k, temp_k) = match temp_c {
                // VA temperature fixtures: tnom=25, swept device temp.
                Some(t) => (25.0 + 273.15, t + 273.15),
                None => (T300, T300),
            };
            let native = native_drain_current(tnom_k, temp_k, vgs, vds);
            let va = va_drain_current(&model, vgs, vds, temp_c);
            let delta = (va - native) / native.abs().max(1e-30);
            worst = worst.max(delta.abs());
            println!(
                "{label:<26} {native:>13.6e} {va:>13.6e} {:>+8.2}%",
                delta * 100.0
            );
            // Documented variant gap is 0.6-1% strong inversion, ~5% deep
            // subthreshold; anything past 15% would mean one of the two
            // models is broken, not a default deviation.
            assert!(
                delta.abs() < 0.15,
                "{label}: native {native:.6e} vs VA {va:.6e} ({:+.2}%)",
                delta * 100.0
            );
        }
        println!("worst |delta| = {:.2}%", worst * 100.0);
    }
}
