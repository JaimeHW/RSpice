//! Native EKV 2.6 validation against Xyce 7.10.

use num_complex::Complex64;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use rspice_core::solver::SimulationResult;
use std::sync::Arc;

const EKV26_NMOS_MODEL: &str = r#".MODEL N NMOS (
+ LEVEL  = 260
+ TNOM = 27.00
+ COX = 4.379E-3
+ XJ = 22.53n
+ VTO = 570.6E-3
+ TCV = 1.194E-3
+ GAMMA = 670.7E-3
+ PHI = 450.0E-3
+ KP = 232.1E-6
+ BEX =-1.828
+ THETA=0.0
+ E0 = 42.216MEG
+ UCRIT = 3.146E6
+ UCEX=0.8
+ LAMBDA = 228.3E-3
+ DL =-60.86n
+ DW =-209.7n
+ WETA = 2.001
+ LETA = 264.6E-3
+ Q0 = 230e-6
+ LK=0.4e-6
+ IBA = 0.000
+ IBB = 300.0E6
+ IBBT = 800.0E-6
+ IBN = 1.000
+ RSH = 0.000
+ HDIF = 0.000
+ AVTO = 0.000
+ AKP = 1e-6
+ )"#;

const EKV26_PMOS_MODEL: &str = r#".MODEL P PMOS (
+ LEVEL  = 260
+ TNOM = 27.00
+ COX = 4.379E-3
+ XJ = 22.53n
+ VTO =-570.6e-3
+ TCV =-1.194e-3
+ GAMMA = 670.7e-3
+ PHI = 450.0E-3
+ KP = 232.1e-6
+ BEX =-1.828
+ THETA=0
+ E0 = 42.216MEG
+ UCRIT = 3.146E6
+ UCEX=0.8
+ LAMBDA = 228.3e-3
+ DL =-60.86n
+ DW =-209.7n
+ WETA=2.001
+ LETA=264.6e-3
+ Q0 = 230e-6
+ LK=0.4e-6
+ IBA = 0.000
+ IBB = 300.0E6
+ IBBT = 800.0E-6
+ IBN = 1.000
+ RSH = 0.000
+ HDIF = 0.000
+ AVTO = 0.000
+ AKP = 1e-6
+ )"#;

fn ekv26_nmos_op_deck(vg: f64, vd: f64) -> String {
    format!(
        "* Xyce Regression EKV26/test_ekv26_nmos.cir selected operating point\n\
         .options temp=27 gmin=0\n\
         {EKV26_NMOS_MODEL}\n\
         v1 g 0 dc {vg:.15e}\n\
         v2 d1 0 dc {vd:.15e}\n\
         vdmon d1 d 0\n\
         mfoo d g 0 0 n l=.35u w=.8u as=1e-12 ad=1e-12 ps=2e-6 pd=2e-6\n\
         .op\n\
         .end\n"
    )
}

fn ekv26_pmos_op_deck(vg: f64, vd: f64) -> String {
    format!(
        "* Xyce Regression EKV26/test_ekv26_pmos.cir selected operating point\n\
         .options temp=27 gmin=0\n\
         {EKV26_PMOS_MODEL}\n\
         v1 g 0 dc {vg:.15e}\n\
         v2 d1 0 dc {vd:.15e}\n\
         vdmon d1 d 0\n\
         mfoo d g 0 0 p l=.45u w=.8u as=1e-12 ad=1e-12 ps=2e-6 pd=2e-6\n\
         .op\n\
         .end\n"
    )
}

fn branch_current(result: &SimulationResult, branch: &str) -> f64 {
    result
        .branch_current_named(branch)
        .unwrap_or_else(|| panic!("missing branch {branch} in {:?}", result.branch_names))
}

fn ac_branch_current(result: &rspice_core::analysis::ac::AcResult, branch: &str) -> Complex64 {
    let index = result
        .branch_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(branch))
        .unwrap_or_else(|| panic!("missing AC branch {branch} in {:?}", result.branch_names));
    result.currents[index]
}

fn assert_abs_or_rel_close(label: &str, got: f64, expected: f64, rel_tol: f64, abs_tol: f64) {
    let abs = (got - expected).abs();
    let tol = abs_tol.max(rel_tol * expected.abs());
    assert!(
        abs <= tol,
        "{label}: got {got:.12e}, expected {expected:.12e}, abs {abs:.3e} > {tol:.3e}"
    );
}

fn assert_complex_abs_or_rel_close(
    label: &str,
    got: Complex64,
    expected: Complex64,
    rel_tol: f64,
    abs_tol: f64,
) {
    let abs = (got - expected).norm();
    let tol = abs_tol.max(rel_tol * expected.norm());
    assert!(
        abs <= tol,
        "{label}: got ({:.12e},{:.12e}), expected ({:.12e},{:.12e}), abs {:.3e} > {:.3e}",
        got.re,
        got.im,
        expected.re,
        expected.im,
        abs,
        tol
    );
}

fn interpolate(time: &[f64], values: &[f64], target: f64) -> f64 {
    assert_eq!(time.len(), values.len(), "time and value vectors align");
    if target <= time[0] {
        return values[0];
    }
    for index in 1..time.len() {
        if time[index] >= target {
            let t0 = time[index - 1];
            let t1 = time[index];
            let y0 = values[index - 1];
            let y1 = values[index];
            if (t1 - t0).abs() <= f64::EPSILON {
                return y1;
            }
            let alpha = (target - t0) / (t1 - t0);
            return y0 + alpha * (y1 - y0);
        }
    }
    *values.last().expect("nonempty waveform")
}

#[test]
fn ekv26_level260_ac_junction_depletion_currents_match_xyce26_oracle() {
    let deck = "* Xyce EKV 2.6 Verilog-A junction depletion AC oracle\n\
         .options temp=27 gmin=0\n\
         .model N NMOS (LEVEL=260 TNOM=27 VTO=0.5 TCV=0 GAMMA=0 PHI=0.5 KP=1e-30 \
         COX=1e-18 XJ=1n DL=0 DW=0 WETA=0 LETA=0 Q0=0 LAMBDA=0 IBA=0 \
         RSH=0 HDIF=0 XD_JS=0 XD_JSW=0 XD_JSWG=0 XD_GMIN=0 \
         XD_CJ=2e-3 XD_CJSW=3e-10 XD_CJSWG=4e-10 \
         XD_MJ=0.45 XD_MJSW=0.35 XD_MJSWG=0.25 \
         XD_PB=0.8 XD_PBSW=0.6 XD_PBSWG=0.55)\n\
         M1 D G S B n W=5u L=1u AD=2e-12 PD=3e-6 AS=0 PS=0\n\
         Vd D 0 DC 0.25 AC 1\n\
         Vg G 0 DC 0 AC 0\n\
         Vs S 0 DC 0 AC 0\n\
         Vb B 0 DC 0 AC 0\n\
         .end\n";
    let netlist = Netlist::parse(deck).expect("EKV26 junction AC charge deck parses");
    let freqs = [1.0e8];
    let results = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &freqs)
        .expect("native EKV26 junction AC charge solve runs");

    // Xyce 7.10 EKV 2.6 uses qjd=(csb_d+cssw_d+csswg_d)*Vdb,
    // I(d,b)<+ddt(qjd)*TYPE*M.  At Vdb=0.25 V the generated Xyce
    // derivative is dqjd/dVdb=5.553649682205238e-15 C/V, so the
    // voltage-source current convention gives -j*2*pi*f*dqjd/dVdb.
    let result = &results[0];
    assert_abs_or_rel_close(
        "Xyce EKV26 junction AC frequency",
        result.frequency,
        1.0e8,
        1.0e-12,
        1.0e-6,
    );
    assert_complex_abs_or_rel_close(
        "Xyce EKV26 junction AC I(VD)",
        ac_branch_current(result, "vd"),
        Complex64::new(0.0, -3.489_461_008_445_452e-6),
        2.0e-4,
        1.0e-11,
    );
    assert_complex_abs_or_rel_close(
        "Xyce EKV26 junction AC I(VB)",
        ac_branch_current(result, "vb"),
        Complex64::new(0.0, 3.489_461_008_445_452e-6),
        2.0e-4,
        1.0e-11,
    );
    assert_complex_abs_or_rel_close(
        "Xyce EKV26 junction AC I(VG)",
        ac_branch_current(result, "vg"),
        Complex64::new(0.0, 0.0),
        2.0e-4,
        1.0e-11,
    );
    assert_complex_abs_or_rel_close(
        "Xyce EKV26 junction AC I(VS)",
        ac_branch_current(result, "vs"),
        Complex64::new(0.0, 0.0),
        2.0e-4,
        1.0e-11,
    );
}

#[test]
fn ekv26_level260_ac_intrinsic_charge_currents_match_xyce26_charge_oracle() {
    let deck = format!(
        "* Xyce EKV 2.6 Verilog-A intrinsic AC terminal-charge oracle\n\
         .options temp=27 gmin=0\n\
         {EKV26_NMOS_MODEL}\n\
         M1 D G S B n W=10u L=1u AS=0 AD=0 PS=0 PD=0\n\
         Vd D 0 DC 1 AC 0\n\
         Vg G 0 DC 0.8 AC 1\n\
         Vs S 0 DC 0 AC 0\n\
         Vb B 0 DC -1 AC 0\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("EKV26 AC charge deck parses");
    let freqs = [1.0e6, 1.0e8, 1.0e9];
    let results = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &freqs)
        .expect("native EKV26 AC charge solve runs");

    // Hard numeric points from the Xyce 7.10 EKV 2.6 Verilog-A current
    // equations plus the intrinsic terminal-charge Jacobian at
    // V(D,G,S,B)=(1,0.8,0,-1). Voltage-source current convention is the
    // negative of the EKV terminal-current row response.
    let oracle = [
        (
            1.0e6,
            Complex64::new(-1.860_118_827_445e-4, 5.388_843_440_055e-8),
            Complex64::new(0.0, -1.587_664_565_478e-7),
            Complex64::new(1.860_118_827_445e-4, 8.386_413_194_165e-8),
            Complex64::new(0.0, 2.101_389_019_257e-8),
        ),
        (
            1.0e8,
            Complex64::new(-1.860_118_827_445e-4, 5.388_843_440_055e-6),
            Complex64::new(0.0, -1.587_664_565_478e-5),
            Complex64::new(1.860_118_827_445e-4, 8.386_413_194_165e-6),
            Complex64::new(0.0, 2.101_389_019_257e-6),
        ),
        (
            1.0e9,
            Complex64::new(-1.860_118_827_445e-4, 5.388_843_440_055e-5),
            Complex64::new(0.0, -1.587_664_565_478e-4),
            Complex64::new(1.860_118_827_445e-4, 8.386_413_194_165e-5),
            Complex64::new(0.0, 2.101_389_019_257e-5),
        ),
    ];

    for (result, (freq, vd, vg, vs, vb)) in results.iter().zip(oracle) {
        assert_abs_or_rel_close(
            "Xyce EKV26 AC frequency",
            result.frequency,
            freq,
            1.0e-12,
            1.0e-6,
        );
        assert_complex_abs_or_rel_close(
            &format!("Xyce EKV26 AC I(VD) at {freq:.3e} Hz"),
            ac_branch_current(result, "vd"),
            vd,
            2.0e-4,
            2.0e-10,
        );
        assert_complex_abs_or_rel_close(
            &format!("Xyce EKV26 AC I(VG) at {freq:.3e} Hz"),
            ac_branch_current(result, "vg"),
            vg,
            2.0e-4,
            2.0e-10,
        );
        assert_complex_abs_or_rel_close(
            &format!("Xyce EKV26 AC I(VS) at {freq:.3e} Hz"),
            ac_branch_current(result, "vs"),
            vs,
            2.0e-4,
            2.0e-10,
        );
        assert_complex_abs_or_rel_close(
            &format!("Xyce EKV26 AC I(VB) at {freq:.3e} Hz"),
            ac_branch_current(result, "vb"),
            vb,
            2.0e-4,
            2.0e-10,
        );
    }
}

#[test]
fn ekv26_level260_nmos_idvd_matches_xyce710_selected_rows() {
    // Xyce 7.10 `EKV26/test_ekv26_nmos.cir.prn`, printed as `ID(Mfoo)`.
    for (vg, vd, expected_id) in [
        (1.0, 0.1, 4.477_071_06e-5),
        (1.0, 1.0, 1.847_307_89e-4),
        (1.4, 1.0, 2.104_232_51e-4),
        (1.8, 3.0, 2.593_874_94e-4),
        (3.0, 3.0, 3.432_412_38e-4),
    ] {
        let deck = ekv26_nmos_op_deck(vg, vd);
        let netlist = Netlist::parse(&deck).expect("EKV26 selected-row deck parses");
        let result = Engine::new(SimulationConfig::default())
            .run_dc_op(&netlist)
            .expect("native EKV26 LEVEL=260 NMOS DC operating point solves");
        let got = branch_current(&result, "vdmon");
        assert_abs_or_rel_close(
            &format!("Xyce EKV26 NMOS V(G)={vg:.1} V(D)={vd:.1} ID(Mfoo)"),
            got,
            expected_id,
            2.0e-3,
            5.0e-12,
        );
    }
}

#[test]
fn ekv26_level260_exact_zero_inert_model_params_match_xyce710_selected_row() {
    let model = EKV26_NMOS_MODEL.replace(
        ".MODEL N NMOS (",
        ".MODEL N NMOS (\n+ FNOIMOD=0\n+ NOIA=0\n+ CGSO=0\n+ CGDO=0\n+ CGBO=0",
    );
    let deck = ekv26_nmos_op_deck(1.0, 1.0).replace(EKV26_NMOS_MODEL, &model);
    let netlist = Netlist::parse(&deck).expect("EKV26 zero inert-param deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("native EKV26 LEVEL=260 exact-zero inert model params solve");
    let got = branch_current(&result, "vdmon");

    // XyceNF 7.10.0 accepts these unsupported EKV26 model parameters as
    // ignored when their values are zero, so the selected row stays on the
    // baseline current.
    assert_abs_or_rel_close(
        "Xyce EKV26 exact-zero inert model params V(G)=1.0 V(D)=1.0 ID(Mfoo)",
        got,
        1.847_307_89e-4,
        2.0e-3,
        5.0e-12,
    );
}

#[test]
fn ekv26_level260_pmos_idvd_matches_xyce710_selected_rows() {
    // Xyce 7.10 `EKV26/test_ekv26_pmos.cir.prn`, printed as `ID(Mfoo)`.
    for (vg, vd, expected_id) in [
        (-3.0, -0.1, -2.400_109_17e-6),
        (-3.0, -1.0, -2.968_808_15e-6),
        (-3.4, -1.0, -1.814_617_37e-5),
        (-4.2, -3.0, -7.439_167_50e-5),
        (-5.0, -3.0, -1.416_728_14e-4),
    ] {
        let deck = ekv26_pmos_op_deck(vg, vd);
        let netlist = Netlist::parse(&deck).expect("EKV26 PMOS selected-row deck parses");
        let result = Engine::new(SimulationConfig::default())
            .run_dc_op(&netlist)
            .expect("native EKV26 LEVEL=260 PMOS DC operating point solves");
        let got = branch_current(&result, "vdmon");
        assert_abs_or_rel_close(
            &format!("Xyce EKV26 PMOS V(G)={vg:.1} V(D)={vd:.1} ID(Mfoo)"),
            got,
            expected_id,
            2.0e-3,
            5.0e-12,
        );
    }
}

#[test]
fn ekv26_level260_inverter_transient_matches_xyce710_selected_rows() {
    let deck = r#"* Xyce Regression EKV26/inverter_transient_ekv26.cir
.model nmos1 nmos level=260 TNOM=27
.model pmos1 pmos level=260 TNOM=27
vdd supply 0 dc 3.0
vsig vi 0 dc 0.5 sin (0 2.5 1MEG)
.subckt mg_inv vin vout vdd gnd
Mp1 vout vin vdd vdd pmos1 W=1u L=1u AD=1e-12 AS=1e-12 PS=2e-6 PD=2e-6
Mn1 vout vin gnd gnd nmos1 W=1u L=1u AD=1e-12 AS=1e-12 PS=2e-6 PD=2e-6
.ends
Xinv1 vi 1 supply 0 mg_inv
R1 1 0 1MEG
.tran 10n 5u
.end
"#;
    let reference = [
        (0.0, 0.000_000_00, 2.991_529_90),
        (7.0e-8, 1.064_448_23, 2.942_416_83),
        (1.226_225_85e-7, 1.741_164_27, 2.618_478_71),
        (1.448_549_91e-7, 1.973_990_63, 1.786_921_49),
        (1.526_429_21e-7, 2.046_664_35, 9.880_835_99e-1),
        (2.519_420_90e-7, 2.499_813_88, 1.919_379_01e-1),
        (3.509_088_54e-7, 2.014_118_19, 1.326_321_08),
        (3.704_549_35e-7, 1.817_522_29, 2.513_865_74),
    ];

    let netlist = Netlist::parse(deck).expect("EKV26 inverter transient deck parses");
    let config = SimulationConfig {
        locked_time_grid: Some(Arc::new(
            reference.iter().skip(1).map(|row| row.0).collect(),
        )),
        ..SimulationConfig::default()
    };
    let result = Engine::new(config)
        .run_tran(&netlist, 5.0e-6, 10.0e-9)
        .expect("native EKV26 inverter transient solves");
    let vi = result
        .try_voltage_waveform_named("vi")
        .unwrap_or_else(|| panic!("missing VI waveform in {:?}", result.node_names));
    let vout = result
        .try_voltage_waveform_named("1")
        .unwrap_or_else(|| panic!("missing output waveform in {:?}", result.node_names));

    for &(time, expected_vi, expected_vout) in &reference {
        let got_vi = interpolate(&result.time, vi, time);
        let got_vout = interpolate(&result.time, vout, time);
        assert_abs_or_rel_close(
            &format!("Xyce EKV26 inverter transient V(VI) at {time:.3e}s"),
            got_vi,
            expected_vi,
            1.0e-4,
            2.0e-5,
        );
        assert_abs_or_rel_close(
            &format!("Xyce EKV26 inverter transient V(1) at {time:.3e}s"),
            got_vout,
            expected_vout,
            2.0e-2,
            2.0e-2,
        );
    }
}

#[test]
fn ekv26_level260_gate_displacement_current_matches_xyce26_charge_oracle() {
    let deck = format!(
        "* Xyce EKV 2.6 Verilog-A intrinsic terminal-charge oracle\n\
         .options temp=27 gmin=0\n\
         {EKV26_NMOS_MODEL}\n\
         M1 D G S B n W=10u L=1u AS=0 AD=0 PS=0 PD=0\n\
         Vd D 0 DC 1\n\
         Vg G 0 SIN(0.8 0.1 100e6 0 0 0)\n\
         Vs S 0 DC 0\n\
         Vb B 0 DC -1\n\
         .tran 0.05n 20n\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("EKV26 gate-current transient deck parses");
    let config = SimulationConfig {
        locked_time_grid: Some(Arc::new(
            (1..=400).map(|idx| idx as f64 * 0.05e-9).collect(),
        )),
        ..SimulationConfig::default()
    };
    let result = Engine::new(config)
        .run_tran(&netlist, 20.0e-9, 0.05e-9)
        .expect("native EKV26 gate-current transient solves");
    let gate = result
        .try_voltage_waveform_named("g")
        .unwrap_or_else(|| panic!("missing G waveform in {:?}", result.node_names));
    let i_vg = result
        .try_branch_current_waveform_named("vg")
        .unwrap_or_else(|| panic!("missing VG branch in {:?}", result.branch_names));

    // Hard numeric points derived from the Xyce 7.10 EKV 2.6 Verilog-A
    // intrinsic charge equations (`ddt(QG)`) for the deck above. The branch
    // current uses RSpice/Xyce voltage-source convention, so it is `-I(g)`.
    let reference = [
        (2.35e-9, 8.995_561_96e-1, -1.580_413_28e-7),
        (4.75e-9, 8.156_434_47e-1, 1.587_362_92e-6),
        (7.846_824_09e-9, 7.023_649_88e-1, -2.983_947_88e-7),
        (1.035_539_08e-8, 8.221_447_55e-1, -1.574_454_96e-6),
        (1.474_538_26e-8, 8.159_299_28e-1, 1.586_956_52e-6),
        (2.0e-8, 8.000_000_00e-1, -1.587_664_57e-6),
    ];

    for &(time, expected_gate, expected_i_vg) in &reference {
        let got_gate = interpolate(&result.time, gate, time);
        let got_i_vg = interpolate(&result.time, i_vg, time);
        assert_abs_or_rel_close(
            &format!("Xyce EKV26 gate drive V(G) at {time:.3e}s"),
            got_gate,
            expected_gate,
            1.0e-4,
            2.0e-5,
        );
        assert_abs_or_rel_close(
            &format!("Xyce EKV26 intrinsic charge I(VG) at {time:.3e}s"),
            got_i_vg,
            expected_i_vg,
            5.0e-2,
            8.0e-8,
        );
    }
}

/// Cross-coupled EKV 2.6 pair with two stable DC operating points. Which one
/// the solver reports is exactly what the `OFF` instance keyword is there to
/// decide. EKV has no ngspice counterpart, so the generic SPICE rule applies:
/// mos1load.c, b3ld.c:217, b4ld.c:316 and vdmosload.c:116 all evaluate a
/// marked instance at zero junction bias on the first load, outside any
/// compatibility gate, and the four RSpice ports of that arm agree.
fn ekv26_bistable_deck(off_instance: &str) -> String {
    let annotate = |instance: &str| {
        if instance == off_instance { " OFF" } else { "" }
    };
    format!(
        "* cross-coupled EKV26 bistable steered by the OFF keyword\n\
         {EKV26_NMOS_MODEL}\n\
         vdd vdd 0 dc 0.75\n\
         r1 vdd d1 10meg\n\
         r2 vdd d2 10meg\n\
         rs1 d1 0 10meg\n\
         rs2 d2 0 10meg\n\
         m1 d1 d2 0 0 n w=100u l=10u{}\n\
         m2 d2 d1 0 0 n w=100u l=10u{}\n\
         .op\n\
         .end\n",
        annotate("m1"),
        annotate("m2")
    )
}

/// How far the routed EKV 2.6 implementation may sit from the native oracle
/// values below.
///
/// `LEVEL=260` is served by two independent implementations of the same model,
/// and which one a deck gets is a build decision, not a deck decision: the
/// native evaluator when the generated catalog is absent, and the compiled
/// Xyce ADMS artifact `ekv_va` — which the router prefers — when it is. Both
/// must select the same branch from the same `OFF` keyword, which is what this
/// test exists to hold. They then settle it 3.2e-6 apart at most (measured:
/// +1.891e-6 on the conducting drain, -3.162e-6 on the cut-off one, 5e-5
/// relative), which is the two ports' own residual on this model and two orders
/// tighter than the 2e-3 relative window the Xyce `ID(VD)` oracle rows above
/// run under. The bound below keeps 3x margin on that and still sits ~33000x
/// inside the 0.335 V gap between the branches, so no wrong-branch regression
/// can hide in it.
#[cfg(feature = "veriloga-model-ekv-va")]
const OFF_BRANCH_TOLERANCE: f64 = 1.0e-5;
/// The native evaluator produced these values, so it is held to them exactly.
#[cfg(not(feature = "veriloga-model-ekv-va"))]
const OFF_BRANCH_TOLERANCE: f64 = 1.0e-7;

fn ekv26_dc_node_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("EKV26 bistable deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("EKV26 bistable operating point converges");
    result
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("missing voltage for node {node}"))
}

#[test]
fn ekv26_off_keyword_selects_the_bistable_operating_point_branch() {
    // Unmarked, both drains settle on the symmetric root. Marking either
    // instance cuts it off on the first load and selects the opposite branch.
    // Until this landed the keyword was not merely dropped here, it was a
    // construction error, so any deck carrying it failed outright.
    let symmetric_d1 = ekv26_dc_node_voltage(&ekv26_bistable_deck(""), "d1");
    let symmetric_d2 = ekv26_dc_node_voltage(&ekv26_bistable_deck(""), "d2");
    // The claim is that the two drains land together, and the pair's equations
    // are symmetric, so the residual here is rounding, not solver accuracy:
    // 2.2e-16 measured, against the 3e-4 a `RELTOL` of 1e-3 would allow at this
    // node. A bound seven orders above the measured residual and five below the
    // convergence contract still catches a pair that has split.
    assert!(
        (symmetric_d1 - symmetric_d2).abs() < 1.0e-9,
        "unmarked pair left the symmetric root: d1 {symmetric_d1}, d2 {symmetric_d2}"
    );
    // The symmetric root is the pair's *unstable* equilibrium, so its last
    // digits are set by the host libm rather than by this port: glibc's
    // exp/log/pow put it 5.5e-7 (1.8 ppm) from where MSVCRT does, bit-stable on
    // each. The 1e-5 window is still ~7500x inside the 0.0748 gap to the
    // nearest stable branch, so a wrong-branch regression cannot hide in it.
    assert!(
        (symmetric_d1 - 0.299_884_9).abs() < 1.0e-5,
        "unmarked symmetric root moved: {symmetric_d1}"
    );

    let m1_off_d1 = ekv26_dc_node_voltage(&ekv26_bistable_deck("m1"), "d1");
    let m1_off_d2 = ekv26_dc_node_voltage(&ekv26_bistable_deck("m1"), "d2");
    let m2_off_d1 = ekv26_dc_node_voltage(&ekv26_bistable_deck("m2"), "d1");
    let m2_off_d2 = ekv26_dc_node_voltage(&ekv26_bistable_deck("m2"), "d2");

    for (label, got, expected) in [
        ("m1 OFF d1", m1_off_d1, 0.374_714_415_0),
        ("m1 OFF d2", m1_off_d2, 0.039_654_175_9),
        ("m2 OFF d1", m2_off_d1, 0.039_654_175_9),
        ("m2 OFF d2", m2_off_d2, 0.374_714_415_0),
    ] {
        assert!(
            (got - expected).abs() < OFF_BRANCH_TOLERANCE,
            "{label}: got {got} expected {expected}"
        );
    }

    // Opposite branches, not merely two values near a reference: a regression
    // back to the symmetric root would otherwise have to be caught by tolerance
    // alone.
    assert!(
        m1_off_d1 - m1_off_d2 > 0.3 && m2_off_d2 - m2_off_d1 > 0.3,
        "each marking must cut its own device off: m1 OFF ({m1_off_d1}, {m1_off_d2}), \
         m2 OFF ({m2_off_d1}, {m2_off_d2})"
    );
}

/// The generated route must admit the same card tail the native route does,
/// and refuse the rest by name.
///
/// `LEVEL=260` reaches the compiled `ekv_va` artifact whenever this build
/// carries it, which is what the branch-selection test above then runs on. This
/// is the other end of that contract: a card key the module cannot honour has
/// to say which key, on which card, and why — never fail as an unknown
/// parameter naming a token the deck did not contain, and never be dropped.
#[cfg(feature = "veriloga-model-ekv-va")]
#[test]
fn ekv26_generated_route_admits_the_card_tail_it_can_honour_and_names_the_rest() {
    let solve = |tail: &str| {
        let deck = format!(
            "* generated EKV26 instance-tail admission\n\
             .options temp=27 gmin=0\n\
             {EKV26_NMOS_MODEL}\n\
             vd d 0 dc 0.5\n\
             vg g 0 dc 0.5\n\
             m1 d g 0 0 n w=10u l=1u{tail}\n\
             .op\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("EKV26 admission deck parses");
        Engine::new(SimulationConfig::default()).run_dc_op(&netlist)
    };

    solve(" OFF").expect("the generated route accepts the OFF keyword");
    solve(" m=2").expect("instance multiplicity reaches the generated route");
    solve(" as=1e-12 ad=1e-12 ps=2e-6 pd=2e-6")
        .expect("declared geometry keys reach the generated route");

    for (tail, fragments) in [
        (" IC=0.2,0.3", ["M1", "IC=", "ekv_va"]),
        (" TEMP=85", ["M1", "TEMP", "ekv_va"]),
        (" DTEMP=10", ["M1", "DTEMP", "ekv_va"]),
        (" NRD=3", ["M1", "NRD", "ekv_va"]),
    ] {
        let message = solve(tail)
            .expect_err("ekv_va cannot honour this card key")
            .to_string();
        for fragment in fragments {
            assert!(
                message.contains(fragment),
                "'{tail}' must be refused naming '{fragment}', got: {message}"
            );
        }
    }
}
