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
    let mut config = SimulationConfig::default();
    config.locked_time_grid = Some(Arc::new(
        reference.iter().skip(1).map(|row| row.0).collect(),
    ));
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
    let mut config = SimulationConfig::default();
    config.locked_time_grid = Some(Arc::new(
        (1..=400).map(|idx| idx as f64 * 0.05e-9).collect(),
    ));
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
