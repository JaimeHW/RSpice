//! Unit tests for the B3SOIFD (level 55) device.
//!
//! Mirror the DD tests (temp-setup physicality, monotonic/NaN-free Ids,
//! charge-matrix consistency) but additionally assert the FD-specific body
//! pinning: the body voltage is algebraically fixed to `Vbs0eff` and the body
//! currents are zero, so a floating FD device exposes no body node and `cbody`
//! reduces to the `minIsub` leakage.

use super::*;
use crate::circuit::NodeId;
use crate::device::mosfet::b3soi::common::B3SoiDialect;
use crate::device::mosfet::b3soi::fd::params::B3SoiFdModel;
use crate::device::mosfet::b3soi::fd::temp::{B3SoiFdGeometry, B3SoiFdSized};
use crate::device::traits::MatrixStamper;
use eval::B3SoiFdBias;
use std::collections::HashMap;
use std::sync::Arc;

/// The `N1` NMOS card from `tests/bsim3soifd/nmosfd.mod` (BSIMFD2.0).
fn n1_params() -> HashMap<String, Value> {
    let pairs: &[(&str, Value)] = &[
        ("LEVEL", 55.0),
        ("TNOM", 27.0),
        ("TOX", 4.5e-9),
        ("TSI", 5e-8),
        ("TBOX", 8e-8),
        ("MOBMOD", 0.0),
        ("CAPMOD", 3.0),
        ("SHMOD", 0.0),
        ("PARAMCHK", 0.0),
        ("WINT", 0.0),
        ("LINT", -2e-8),
        ("VTH0", 0.52),
        ("K1", 0.39),
        ("K2", 0.1),
        ("K3", 0.0),
        ("KB1", 0.95),
        ("K3B", 2.2),
        ("NLX", 7.2e-8),
        ("DVT0", 0.55),
        ("DVT1", 0.28),
        ("DVT2", -1.4),
        ("NCH", 3.3e17),
        ("NSUB", 1e15),
        ("NGATE", 1e20),
        ("DVBD0", 60.0),
        ("DVBD1", 1.1),
        ("VBSA", 0.0),
        ("KB3", 2.2),
        ("DELP", 0.02),
        ("ABP", 0.9),
        ("MXC", 0.9),
        ("ADICE0", 0.93),
        ("KBJT1", 1e-8),
        ("EDL", 5e-7),
        ("NDIODE", 1.13),
        ("NTUN", 14.0),
        ("ISBJT", 2e-6),
        ("ISDIF", 1e-6),
        ("ISTUN", 0.0),
        ("ISREC", 1e-5),
        ("XBJT", 0.01),
        ("XDIF", 0.01),
        ("XREC", 0.01),
        ("XTUN", 0.001),
        ("U0", 352.0),
        ("UA", 1.3e-11),
        ("UB", 1.7e-18),
        ("UC", -4e-10),
        ("W0", 1.16e-6),
        ("AGS", 0.25),
        ("A1", 0.0),
        ("A2", 1.0),
        ("B0", 0.01),
        ("B1", 10.0),
        ("RDSW", 700.0),
        ("PRWG", 0.0),
        ("PRWB", -0.2),
        ("WR", 1.0),
        ("RBODY", 0.0),
        ("RBSH", 0.0),
        ("A0", 1.4),
        ("KETA", -0.67),
        ("VSAT", 135000.0),
        ("ALPHA0", 0.0),
        ("ALPHA1", 1.5),
        ("BETA0", 20.5),
        ("AII", 1.2),
        ("BII", 0.1e-7),
        ("CII", 0.8),
        ("DII", 0.6),
        ("VOFF", -0.14),
        ("NFACTOR", 0.7),
        ("CDSC", 2e-5),
        ("PCLM", 2.9),
        ("PVAG", 12.0),
        ("PDIBLC1", 0.18),
        ("PDIBLC2", 0.004),
        ("PDIBLCB", -0.234),
        ("DROUT", 0.2),
        ("DELTA", 0.01),
        ("ETA0", 0.01),
        ("ETAB", 0.0),
        ("DSUB", 0.3),
        ("RTH0", 0.006),
        ("CLC", 1e-7),
        ("CLE", 0.6),
        ("CF", 1e-20),
        ("CKAPPA", 0.6),
        ("CGDL", 1e-20),
        ("CGSL", 1e-20),
        ("KT1", -0.3),
        ("KT2", 0.022),
        ("UTE", -1.5),
        ("UA1", 4.31e-9),
        ("UB1", -7.61e-18),
        ("UC1", -5.6e-11),
        ("PRT", 760.0),
        ("AT", 22400.0),
        ("CGSO", 1e-10),
        ("CGDO", 1e-10),
        ("CJSWG", 5e-10),
        ("TT", 3e-10),
        ("ASD", 0.3),
        ("CSDESW", 1e-12),
    ];
    pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
}

fn geom() -> B3SoiFdGeometry {
    B3SoiFdGeometry {
        l: 0.25e-6,
        w: 10e-6,
        drain_area: 0.0,
        source_area: 0.0,
        drain_squares: 0.0,
        source_squares: 0.0,
        drain_perimeter: 0.0,
        source_perimeter: 0.0,
        body_squares: 0.0,
        rth0: 0.006,
        cth0: 0.0,
    }
}

#[derive(Default)]
struct RecordingStamper {
    rhs: HashMap<NodeId, Value>,
    matrix: HashMap<(NodeId, NodeId), Value>,
}

impl RecordingStamper {
    fn rhs_at(&self, node: NodeId) -> Value {
        self.rhs.get(&node).copied().unwrap_or(0.0)
    }

    fn matrix_at(&self, row: NodeId, col: NodeId) -> Value {
        self.matrix.get(&(row, col)).copied().unwrap_or(0.0)
    }
}

impl MatrixStamper for RecordingStamper {
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        *self.matrix.entry((row, col)).or_insert(0.0) += value;
    }

    fn stamp_rhs(&mut self, index: NodeId, value: Value) {
        *self.rhs.entry(index).or_insert(0.0) += value;
    }
}

fn device_for_stamp_test(is_pmos: bool) -> B3SoiFd {
    let model = Arc::new(B3SoiFdModel::from_params(&n1_params(), is_pmos, 300.15));
    B3SoiFd::new(
        "m1".to_string(),
        1,
        2,
        3,
        4,
        0,
        0,
        BodyMode::Floating,
        model,
        geom(),
        300.15,
    )
    .expect("fd device")
}

fn self_heating_device_for_stamp_test() -> B3SoiFd {
    let mut params = n1_params();
    params.insert("SHMOD".to_string(), 1.0);
    let model = Arc::new(B3SoiFdModel::from_params(&params, false, 300.15));
    let mut geometry = geom();
    geometry.cth0 = 2.0;
    B3SoiFd::new(
        "m1".to_string(),
        1,
        2,
        3,
        4,
        5,
        0,
        BodyMode::Floating,
        model,
        geometry,
        300.15,
    )
    .expect("self-heating FD device builds with a temp node")
}

fn model_consts(m: &B3SoiFdModel) -> ModelConsts {
    ModelConsts {
        cap_mod: m.cap_mod,
        cox: m.cox,
        cbox: m.cbox,
        csi: m.csi,
        csieff: m.csieff,
        qsi: m.qsi,
        qsieff: m.qsieff,
        adice: m.adice,
        tox: m.tox,
        tsi: m.tsi,
        xj: m.xj,
        charge_q: crate::device::mosfet::b3soi::common::CHARGE_Q,
        mob_mod: m.mob_mod,
        cboxt: m.cboxt,
        xpart: m.xpart,
        mtype: m.mtype,
    }
}

#[test]
fn charge_companion_folds_body_charge_linearization_into_source_rhs() {
    let charge = eval::B3SoiFdCharge {
        gcbgb: 3.0,
        gcbdb: 5.0,
        gcbsb: 7.0,
        gcbeb: 11.0,
        ..Default::default()
    };
    let ag0 = 2.0;
    let cqb = 11.0;
    let voltages = [0.8, 1.2, 0.0, -0.4];

    for is_pmos in [false, true] {
        let dev = device_for_stamp_test(is_pmos);
        let mut stamper = RecordingStamper::default();
        dev.stamp_charge_companion(
            &charge,
            ag0,
            0.0,
            cqb,
            0.0,
            0.0,
            0.0,
            &voltages,
            &mut stamper,
        );

        let mt = if is_pmos { -1.0 } else { 1.0 };
        let vbs = mt * (0.0 - voltages[2]);
        let vgs = mt * (voltages[1] - voltages[2]);
        let vds = mt * (voltages[0] - voltages[2]);
        let ves = mt * (voltages[3] - voltages[2]);
        let vgb = vgs - vbs;
        let vbd = vbs - vds;
        let veb = ves - vbs;
        let mut ceqqb =
            cqb - charge.gcbgb * ag0 * vgb + charge.gcbdb * ag0 * vbd + charge.gcbsb * ag0 * vbs
                - charge.gcbeb * ag0 * veb;
        if is_pmos {
            ceqqb = -ceqqb;
        }

        assert!(
            (stamper.rhs_at(3) - ceqqb).abs() <= 1e-12,
            "source RHS should include full ceqqb for is_pmos={is_pmos}: got {}, expected {ceqqb}",
            stamper.rhs_at(3)
        );
        assert_eq!(stamper.rhs_at(0), 0.0, "FD must not stamp a body-node RHS");
    }
}

#[test]
fn self_heating_charge_companion_stamps_qth_capacitance() {
    let dev = self_heating_device_for_stamp_test();
    let cth = dev.thermal_capacitance();
    assert!(cth > 0.0, "cth={cth:.6e}");

    let ag0 = 7.0;
    let del_temp = 0.25;
    let voltages = [0.5, 1.2, 0.0, 0.0, del_temp];
    let mut stamper = RecordingStamper::default();

    dev.stamp_charge_companion(
        &eval::B3SoiFdCharge::default(),
        ag0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        &voltages,
        &mut stamper,
    );

    let expected_gc_tt = ag0 * cth;
    let expected_rhs = expected_gc_tt * del_temp;
    assert!(
        (stamper.matrix_at(5, 5) - expected_gc_tt).abs() <= 1e-15,
        "temp-node companion conductance {:.6e} vs {expected_gc_tt:.6e}",
        stamper.matrix_at(5, 5)
    );
    assert!(
        (stamper.rhs_at(5) - expected_rhs).abs() <= 1e-15,
        "temp-node companion RHS {:.6e} vs {expected_rhs:.6e}",
        stamper.rhs_at(5)
    );
}

#[test]
fn self_heating_update_limits_del_temp_step_like_ngspice() {
    let mut dev = self_heating_device_for_stamp_test();

    dev.update(&[0.5, 1.2, 0.0, 0.0, 0.0]);
    dev.update(&[0.5, 1.2, 0.0, 0.0, 100.0]);

    assert!(
        (dev.bias.del_temp - 5.0).abs() <= 1e-12,
        "FD self-heating delTemp should be limited to a 5 K step, got {:.9e}",
        dev.bias.del_temp
    );
}

#[test]
fn capmod3_fd_dynamic_charges_ignore_intrinsic_source_drain_junction_storage() {
    let mut no_junction = n1_params();
    no_junction.insert("CAPMOD".to_string(), 3.0);
    no_junction.insert("CJSWG".to_string(), 0.0);
    no_junction.insert("TT".to_string(), 0.0);
    let model_no_junction = B3SoiFdModel::from_params(&no_junction, false, 300.15);
    let sized_no_junction = B3SoiFdSized::new(&model_no_junction, &geom(), 300.15).expect("sized");
    let mc_no_junction = model_consts(&model_no_junction);

    let mut stressed_junction = no_junction;
    stressed_junction.insert("CJSWG".to_string(), 5e-6);
    stressed_junction.insert("TT".to_string(), 3e-7);
    let model_stressed_junction = B3SoiFdModel::from_params(&stressed_junction, false, 300.15);
    let sized_stressed_junction =
        B3SoiFdSized::new(&model_stressed_junction, &geom(), 300.15).expect("sized");
    let mc_stressed_junction = model_consts(&model_stressed_junction);

    let bias = B3SoiFdBias {
        vbs: 0.0,
        vgs: 1.2,
        vds: 0.8,
        ves: 0.0,
        vps: 0.0,
        ..Default::default()
    };
    let baseline = eval::eval(&sized_no_junction, &mc_no_junction, bias, 1.0, true)
        .charge
        .expect("charge");
    let stressed = eval::eval(
        &sized_stressed_junction,
        &mc_stressed_junction,
        bias,
        1.0,
        true,
    )
    .charge
    .expect("charge");

    let same = |label: &str, lhs: Value, rhs: Value| {
        assert!(
            (lhs - rhs).abs() <= 1e-24,
            "{label} changed through FD intrinsic junction storage: baseline={lhs:.12e}, stressed={rhs:.12e}"
        );
    };
    same("qb", baseline.qb, stressed.qb);
    same("qd", baseline.qd, stressed.qd);
    same("gcddb", baseline.gcddb, stressed.gcddb);
    same("gcdsb", baseline.gcdsb, stressed.gcdsb);
    same("gcbdb", baseline.gcbdb, stressed.gcbdb);
    same("gcbsb", baseline.gcbsb, stressed.gcbsb);
}

#[test]
fn temp_setup_is_finite_and_physical() {
    let model = B3SoiFdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiFdSized::new(&model, &geom(), 300.15).expect("sized");
    assert!(sized.phi > 0.5 && sized.phi < 1.2, "phi={}", sized.phi);
    assert!(sized.vtm > 0.02 && sized.vtm < 0.03, "vtm={}", sized.vtm);
    assert!(sized.u0temp > 0.0 && sized.u0temp < 1.0);
    assert!(sized.vth0.is_finite());
    assert!(sized.rds0 >= 0.0);
    assert_eq!(model.dialect, B3SoiDialect::Ngspice);
    assert_eq!(
        sized.abulk_cv_factor,
        (1.0 + sized.clc / sized.leff).powf(sized.cle)
    );
    assert!(sized.jbjt > 0.0 && sized.jrec > 0.0 && sized.jdif > 0.0);
}

#[test]
fn temp_setup_uses_xyce_abulk_cv_factor_for_level_10_origin() {
    let mut params = n1_params();
    params.insert("LEVEL".to_string(), 10.0);
    let model = B3SoiFdModel::from_params(&params, false, 300.15);
    let sized = B3SoiFdSized::new(&model, &geom(), 300.15).expect("sized");

    assert_eq!(model.dialect, B3SoiDialect::Xyce);
    assert_eq!(
        sized.abulk_cv_factor,
        1.0 + (sized.clc / sized.leff).powf(sized.cle)
    );
}

#[test]
fn eval_dc_strong_inversion_is_sane() {
    let model = B3SoiFdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiFdSized::new(&model, &geom(), 300.15).expect("sized");
    let mc = model_consts(&model);
    let bias = B3SoiFdBias {
        vbs: 0.0,
        vgs: 3.0,
        vds: 0.05,
        ves: 0.0,
        vps: 0.0,
        ..Default::default()
    };
    let op = eval::eval_dc(&sized, &mc, bias, 1.0);
    assert!(op.ids.is_finite() && op.ids > 0.0, "ids={}", op.ids);
    assert!(op.gds.is_finite() && op.gds > 0.0, "gds={}", op.gds);
    assert!(op.gm.is_finite() && op.gm >= 0.0, "gm={}", op.gm);
    assert!(op.von.is_finite());
    assert!(op.ids < 1.0, "ids unreasonably large: {}", op.ids);
}

#[test]
fn fd_body_currents_are_zero() {
    // FD disables impact ionization, GIDL and the body diodes (b3soifdld.c:
    // 2121-2145). Across a strong-inversion sweep the only body term is the
    // minIsub leakage, so cbody must equal the linearized minIsub residual and
    // never diverge from zero by more than that tiny amount.
    let model = B3SoiFdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiFdSized::new(&model, &geom(), 300.15).expect("sized");
    let mc = model_consts(&model);
    for vg_i in 0..=15 {
        for vd_i in 0..=20 {
            let bias = B3SoiFdBias {
                vbs: 0.0,
                vgs: 0.1 * vg_i as Value,
                vds: 0.1 * vd_i as Value,
                ves: 0.0,
                vps: 0.0,
                ..Default::default()
            };
            let op = eval::eval_dc(&sized, &mc, bias, 1.0);
            // No impact ionization / GIDL / diode body conductances.
            assert_eq!(op.gbgs, 0.0, "gbgs nonzero at {bias:?}");
            assert_eq!(op.gbds, 0.0, "gbds nonzero at {bias:?}");
            assert_eq!(op.gbes, 0.0, "gbes nonzero at {bias:?}");
            // cbody is at most the minIsub leakage scale.
            assert!(
                op.cbody.abs() <= sized.min_isub.max(1e-30) * 2.0 + 1e-18,
                "cbody={} exceeds minIsub at {bias:?}",
                op.cbody
            );
        }
    }
}

#[test]
fn eval_dc_monotonic_in_vg() {
    let model = B3SoiFdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiFdSized::new(&model, &geom(), 300.15).expect("sized");
    let mc = model_consts(&model);
    let mk = |vg: Value| {
        eval::eval_dc(
            &sized,
            &mc,
            B3SoiFdBias {
                vbs: 0.0,
                vgs: vg,
                vds: 0.05,
                ves: 0.0,
                vps: 0.0,
                ..Default::default()
            },
            1.0,
        )
        .ids
    };
    let i_low = mk(0.3);
    let i_mid = mk(0.8);
    let i_high = mk(1.5);
    assert!(i_low < i_mid, "{} !< {}", i_low, i_mid);
    assert!(i_mid < i_high, "{} !< {}", i_mid, i_high);
    assert!(i_low >= 0.0);
}

#[test]
fn eval_dc_no_nan_across_sweep() {
    let model = B3SoiFdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiFdSized::new(&model, &geom(), 300.15).expect("sized");
    let mc = model_consts(&model);
    for vg_i in 0..=15 {
        for vd_i in 0..=30 {
            for ve_i in -4..=4 {
                let bias = B3SoiFdBias {
                    vbs: 0.0,
                    vgs: 0.1 * vg_i as Value,
                    vds: 0.1 * vd_i as Value,
                    ves: ve_i as Value,
                    vps: 0.0,
                    ..Default::default()
                };
                let op = eval::eval_dc(&sized, &mc, bias, 1.0);
                assert!(op.ids.is_finite(), "ids NaN at {bias:?}");
                assert!(op.cbody.is_finite(), "cbody NaN at {bias:?}");
                assert!(op.gds.is_finite() && op.gm.is_finite());
                assert!(op.cjd.is_finite() && op.cjs.is_finite());
            }
        }
    }
}

fn assert_charge_matrix_is_consistent_with_charges_for_capmod(cap_mod: i32) {
    let mut params = n1_params();
    params.insert("CAPMOD".to_string(), cap_mod as Value);
    let model = B3SoiFdModel::from_params(&params, false, 300.15);
    let sized = B3SoiFdSized::new(&model, &geom(), 300.15).expect("sized");
    let mc = model_consts(&model);
    let charge = |vg: Value, vd: Value, ve: Value| {
        eval::eval(
            &sized,
            &mc,
            B3SoiFdBias {
                vbs: 0.0,
                vgs: vg,
                vds: vd,
                ves: ve,
                vps: 0.0,
                ..Default::default()
            },
            1.0,
            true,
        )
        .charge
        .unwrap()
    };
    let (vg, vd, ve) = (1.2_f64, 0.8, 0.0);
    let h = 1e-6;
    let c0 = charge(vg, vd, ve);
    let cp = charge(vg + h, vd, ve);
    let cm = charge(vg - h, vd, ve);
    let dqg_dvg = (cp.qg - cm.qg) / (2.0 * h);
    let ok = |analytic: Value, fd: Value| {
        (analytic - fd).abs() <= 1e-2 * analytic.abs().max(fd.abs()) + 1e-14
    };
    assert!(ok(c0.gcggb, dqg_dvg), "cggb {} vs FD {}", c0.gcggb, dqg_dvg);
    let total = c0.qg + c0.qb + c0.qd + c0.qe;
    assert!(total.is_finite(), "charge sum not finite: {total}");
}

#[test]
fn capmod3_charge_matrix_is_consistent_with_charges() {
    assert_charge_matrix_is_consistent_with_charges_for_capmod(3);
}

#[test]
fn capmod2_charge_matrix_is_consistent_with_charges() {
    assert_charge_matrix_is_consistent_with_charges_for_capmod(2);
}
