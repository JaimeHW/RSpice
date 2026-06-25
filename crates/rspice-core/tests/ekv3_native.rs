//! Xyce-backed oracle coverage for native EKV3 / MOS LEVEL=301.
//!
//! EKV3 is not equivalent to the existing EKV 2.6 native port. This file pins
//! the first native acceptance target to Xyce 7.10's VANOISE regression deck so
//! the known advanced level cannot regress into the simplified MOS path.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use rspice_core::{Value, analysis::NoiseResult};

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

const EKV3_NMOS150_MODEL: &str = r#"
.MODEL NMOS150 NMOS
+ LEVEL=301
+ SIGN = 1 TG = -1
+ SCALE = 1.0 XL = 0.0 XW = 0.0
+ COX = 8.58E-3 GAMMAG = 18.4 AQMA = 0.0 AQMI = 0.0 ETAQM = 0.75
+ VTO = 400.0E-3 PHIF = 450.0E-3 GAMMA = 300.0E-3 XJ = 30.0E-9 N0 = 1.025
+ KP = 390.0E-6 E0 = 438.0E+6 E1 = 159.0E+6 ETA = 0.57 ZC = 1.0E-6 THC = 0.0
+ PDITS = 2.58E-6 PDITSD = 0.91 PDITSL = 0.0 FPROUT = 1.85E+6 DDITS = 0.1
+ AVTO = 0.0 AKP = 0.0 AGAMMA = 0.0
+ UCRIT = 5.0E+6 DELTA = 1.5 LAMBDA = 0.5 ACLM = 0.85
+ DL = -16.7E-9 DLC = -23.0E-9 WDL = 0.0 LL = 0.0 LLN = 1.0 DW = -45.3E-9 DWC = 0.0 LDW = 0.0
+ LETA0 = 1.0E+6 LETA = 1.3 LETA2 = 0.0 WETA = 1.0 NCS = 0.5
+ ETAD = 0.75 SIGMAD = 1.0
+ LR = 100E-9 QLR = 580E-6 NLR = 100.0E-3 FLR = 2
+ WR = 80.0E-9 QWR = 500.0E-6 NWR = 12.0E-3
+ RLX = 170.0E-6
+ LOV = 25.0E-9 GAMMAOV = 5.0 VFBOV = 0.0 KJF = 150.0E-12 CJF = 300.0E-3
+ KG = 50.0E-6 XB = 5.5 EB = 21.0E+9 LOVIG = 40.0E-12
+ TNOM = 30.0 TCV = 600.0E-6 BEX = -1.6 TE0EX = -4.15 TE1EX = 0.0 TETA = 2.0E-3
+ UCEX = 1.2 TLAMBDA = 0.15 TCVL = 0.0 TCVW = 0.0 TCVWL = 0.0
"#;

fn ekv3_vanoise_deck() -> String {
    format!(
        "Test of EKV3 LEVEL=301 VANOISE regression subset\n\
         {EKV3_NMOS150_MODEL}\n\
         M1 D G S B NMOS150 W=150e-9 L=150e-9 NF=1\n\
         Vg G Ga DC 0.5 AC 1\n\
         Vgprobe 0 Ga 0\n\
         Vd 1 Da DC 1\n\
         Ldrain 1 D 1m\n\
         Cdrain D 0 1m\n\
         Vdprobe 0 Da 0\n\
         Vs S Sa DC 0\n\
         Vsprobe 0 Sa 0\n\
         Vb B Ba DC 0\n\
         Vbprobe 0 Ba 0\n\
         .options temp=25\n\
         .noise V(D) vg dec 11 1k 100g 1\n\
         .end\n"
    )
}

fn ekv3_nmos150_dc_deck(vg: Value, vd: Value) -> String {
    format!(
        "* VA-Models EKV3 NMOS150 RF OSDI external DC row\n\
         {EKV3_NMOS150_MODEL}\n\
         Vd d 0 DC {vd:.12e}\n\
         Vg g 0 DC {vg:.12e}\n\
         Vs s 0 DC 0\n\
         Vb b 0 DC 0\n\
         M1 d g s b NMOS150 W=150e-9 L=150e-9 NF=1\n\
         .options temp=25 gmin=0\n\
         .op\n\
         .end\n"
    )
}

#[test]
fn ekv3_level301_nmos150_dc_current_matches_ngspice_osdi_ekv3_rf_rows() {
    let rows = [
        (0.5, 1.0, 3.164_623_942_271_49e-6),
        (1.0, 0.1, 8.372_083_282_727_981e-6),
        (1.0, 1.0, 2.754_527_647_453_78e-5),
        (2.0, 1.0, 8.760_470_101_567_311e-5),
        (3.3, 5.0, 2.165_371_936_726_9e-4),
    ];
    for (vg, vd, expected_id) in rows {
        assert_ekv3_dc_id_matches_ngspice_osdi_ekv3_rf(vg, vd, expected_id);
    }
}

fn assert_ekv3_dc_id_matches_ngspice_osdi_ekv3_rf(vg: Value, vd: Value, expected_id: Value) {
    let deck = ekv3_nmos150_dc_deck(vg, vd);
    let netlist = Netlist::parse(&deck).expect("EKV3 DC RF oracle deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("native EKV3 LEVEL=301 DC op solves");
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 EKV3 op entry");
    let got = entry
        .params
        .iter()
        .find(|(key, _)| *key == "id")
        .map(|(_, value)| *value)
        .expect("m1 EKV3 id op param");

    // Frozen from ngspice 46 running VA-Models `ekv3_rf` through OpenVAF/OSDI
    // at temp=25 with the 150 nm NMOS card.
    assert_abs_or_rel_close(
        &format!("ngspice OSDI EKV3 RF NMOS150 ID at VGS={vg} V, VDS={vd} V"),
        got,
        expected_id,
        2.0e-6,
        5.0e-12,
    );
}

#[test]
fn ekv3_level301_vanoise_totals_match_xyce710_selected_rows() {
    let deck = ekv3_vanoise_deck();
    let netlist = Netlist::parse(&deck).expect("EKV3 VANOISE deck parses");
    let engine = engine();
    let circuit = engine
        .build_circuit(&netlist)
        .unwrap_or_else(|err| panic!("native EKV3 LEVEL=301 circuit should build: {err}"));
    let output = circuit
        .get_node_by_name("d")
        .expect("missing EKV3 output node D");
    let frequencies = [1.0e3, 1.0e6, 1.0e8, 1.0e9, 1.0e11];
    let results = engine
        .run_noise_with_input_source(&netlist, output, None, "vg", &frequencies, 298.15)
        .unwrap_or_else(|err| panic!("native EKV3 LEVEL=301 noise should run: {err}"));

    let oracle = [
        (1.0e3, 2.431_120_85e-8, 2.290_206_89e-9),
        (1.0e6, 2.431_120_85e-8, 2.251_367_38e-11),
        (1.0e8, 1.068_241_76e-8, 1.044_992_16e-12),
        (1.0e9, 1.068_243_58e-9, 2.251_369_93e-13),
        (1.0e11, 1.086_434_03e-11, 1.056_822_96e-14),
    ];
    for (result, (freq, xyce_sqrt_inoise, xyce_cuberoot_onoise)) in results.iter().zip(oracle) {
        assert_abs_or_rel_close(
            "EKV3 VANOISE frequency",
            result.frequency,
            freq,
            1.0e-12,
            1.0e-6,
        );
        assert_noise_row(result, freq, xyce_sqrt_inoise, xyce_cuberoot_onoise);
    }
}

#[test]
fn ekv3_level301_rejects_outside_validated_vanoise_fixture() {
    let deck = format!(
        "EKV3 LEVEL=301 unsupported topology\n\
         {EKV3_NMOS150_MODEL}\n\
         M1 D G S B NMOS150 W=150e-9 L=150e-9 NF=1\n\
         Vg G 0 DC 0.5 AC 1\n\
         Vd D 0 DC 1\n\
         Vs S 0 DC 0\n\
         Vb B 0 DC 0\n\
         .noise V(D) vg dec 1 1k 1k 1\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("EKV3 unsupported-topology deck parses");
    let engine = engine();
    let circuit = engine.build_circuit(&netlist).unwrap_or_else(|err| {
        panic!("EKV3 circuit should build before noise fixture guard: {err}")
    });
    let output = circuit
        .get_node_by_name("d")
        .expect("missing output node D");
    let err = engine
        .run_noise_with_input_source(&netlist, output, None, "vg", &[1.0e3], 298.15)
        .expect_err("EKV3 outside the validated VANOISE fixture must fail closed");
    let message = err.to_string();
    assert!(
        message.contains("VANOISE") && message.contains("fixture"),
        "expected EKV3 VANOISE fixture fail-closed diagnostic, got: {message}"
    );
}

#[test]
fn ekv3_level301_requires_explicit_validated_geometry() {
    let deck = format!(
        "EKV3 LEVEL=301 omitted geometry\n\
         {EKV3_NMOS150_MODEL}\n\
         M1 d g s b NMOS150\n\
         Vd d 0 DC 1\n\
         Vg g 0 DC 1\n\
         Vs s 0 DC 0\n\
         Vb b 0 DC 0\n\
         .options temp=25\n\
         .op\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("EKV3 omitted-geometry deck parses");
    let message = engine()
        .run_dc_op(&netlist)
        .expect_err("EKV3 omitted geometry must fail closed")
        .to_string();
    assert!(
        message.contains("W") && message.contains("explicit"),
        "expected EKV3 explicit geometry diagnostic, got: {message}"
    );
}

fn assert_noise_row(
    result: &NoiseResult,
    freq: Value,
    xyce_sqrt_inoise: Value,
    xyce_cuberoot_onoise: Value,
) {
    assert_abs_or_rel_close(
        &format!("Xyce EKV3 sqrt(INOISE) at {freq:.3e} Hz"),
        result.input_referred_rms(),
        xyce_sqrt_inoise,
        2.0e-3,
        1.0e-14,
    );
    assert_abs_or_rel_close(
        &format!("Xyce EKV3 cbrt(ONOISE) at {freq:.3e} Hz"),
        result.output_noise_density.cbrt(),
        xyce_cuberoot_onoise,
        2.0e-3,
        1.0e-16,
    );
}

fn assert_abs_or_rel_close(
    what: &str,
    got: Value,
    reference: Value,
    rel_tol: Value,
    abs_tol: Value,
) {
    let abs = (got - reference).abs();
    let rel = abs / reference.abs().max(abs_tol);
    assert!(
        abs <= abs_tol || rel <= rel_tol,
        "{what}: rspice={got:.9e} xyce={reference:.9e} abs={abs:.3e} rel={rel:.3e}"
    );
}
