//! Unit tests for the B3SOIPD (level 57) device.
//!
//! PD is the partially-depleted variant: it keeps a real floating-body node and
//! the full body-diode + parasitic-BJT current set, so the tests assert
//! temperature-setup physicality, NaN-free / monotonic channel current, the
//! presence of nonzero body coupling under forward bias, and charge-matrix
//! consistency.

use super::*;
use crate::device::mosfet::b3soi::pd::params::B3SoiPdModel;
use crate::device::mosfet::b3soi::pd::temp::{B3SoiPdGeometry, B3SoiPdSized};
use eval::B3SoiPdBias;
use std::collections::HashMap;

/// The `N1` NMOS card from `tests/bsim3soipd/nmospd.mod` (BSIMPD2.0).
fn n1_params() -> HashMap<String, Value> {
    let pairs: &[(&str, Value)] = &[
        ("LEVEL", 57.0),
        ("TNOM", 27.0),
        ("TOX", 4.5e-9),
        ("TSI", 1e-7),
        ("TBOX", 8e-8),
        ("MOBMOD", 0.0),
        ("CAPMOD", 2.0),
        ("SHMOD", 0.0),
        ("PARAMCHK", 0.0),
        ("WINT", 0.0),
        ("LINT", -2e-8),
        ("VTH0", 0.42),
        ("K1", 0.49),
        ("K2", 0.1),
        ("K3", 0.0),
        ("K3B", 2.2),
        ("NLX", 2e-7),
        ("DVT0", 10.0),
        ("DVT1", 0.55),
        ("DVT2", -1.4),
        ("NCH", 4.7e17),
        ("NSUB", -1e15),
        ("NGATE", 1e20),
        ("AGIDL", 1e-15),
        ("BGIDL", 1e9),
        ("NGIDL", 1.1),
        ("NDIODE", 1.13),
        ("NTUN", 14.0),
        ("NRECF0", 2.5),
        ("NRECR0", 4.0),
        ("VREC0", 1.2),
        ("NTRECF", 0.1),
        ("NTRECR", 0.2),
        ("ISBJT", 1e-4),
        ("ISDIF", 1e-5),
        ("ISTUN", 2e-5),
        ("ISREC", 4e-2),
        ("XBJT", 0.9),
        ("XDIF", 0.9),
        ("XREC", 0.9),
        ("XTUN", 0.01),
        ("AHLI", 1e-9),
        ("LBJT0", 0.2e-6),
        ("LN", 2e-6),
        ("NBJT", 0.8),
        ("NDIF", -1.0),
        ("AELY", 1e8),
        ("VABJT", 0.0),
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
        ("RDSW", 0.0),
        ("PRWG", 0.0),
        ("PRWB", -0.2),
        ("WR", 1.0),
        ("RBODY", 1.0),
        ("RBSH", 0.0),
        ("A0", 1.4),
        ("KETA", 0.1),
        ("KETAS", 0.2),
        ("VSAT", 135000.0),
        ("ALPHA0", 1e-8),
        ("BETA0", 0.0),
        ("BETA1", 0.05),
        ("BETA2", 0.07),
        ("VDSATII0", 0.8),
        ("ESATII", 1e7),
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
        ("ETA0", 0.05),
        ("ETAB", 0.0),
        ("DSUB", 0.2),
        ("RTH0", 0.005),
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
        ("CJSWG", 1e-12),
        ("TT", 3e-10),
        ("ASD", 0.3),
        ("CSDESW", 1e-12),
        ("TCJSWG", 1e-4),
        ("MJSWG", 0.5),
        ("PBSWG", 1.0),
    ];
    pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
}

fn geom() -> B3SoiPdGeometry {
    B3SoiPdGeometry {
        l: 0.25e-6,
        w: 10e-6,
        drain_area: 0.0,
        source_area: 0.0,
        drain_squares: 0.0,
        source_squares: 0.0,
        drain_perimeter: 0.0,
        source_perimeter: 0.0,
        body_squares: 0.0,
        rth0: 0.005,
        cth0: 0.0,
    }
}

fn model_consts(m: &B3SoiPdModel) -> ModelConsts {
    ModelConsts {
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
        tt: m.tt,
        mjswg: m.body_jct_gate_side_grading_coeff,
        phibswg: m.gate_sidewall_jct_potential.max(0.1),
        cjswg: m.unit_length_gate_sidewall_jct_cap,
        mtype: m.mtype,
    }
}

#[test]
fn temp_setup_is_finite_and_physical() {
    let model = B3SoiPdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiPdSized::new(&model, &geom(), 300.15).expect("sized");
    assert!(sized.phi > 0.5 && sized.phi < 1.2, "phi={}", sized.phi);
    assert!(sized.vtm > 0.02 && sized.vtm < 0.03, "vtm={}", sized.vtm);
    assert!(sized.u0temp > 0.0 && sized.u0temp < 1.0);
    assert!(sized.vth0.is_finite());
    assert!(sized.rds0 >= 0.0);
    assert!(sized.jbjt > 0.0 && sized.jrec > 0.0 && sized.jdif > 0.0);
}

#[test]
fn eval_dc_strong_inversion_is_sane() {
    let model = B3SoiPdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiPdSized::new(&model, &geom(), 300.15).expect("sized");
    let mc = model_consts(&model);
    let bias = B3SoiPdBias {
        vbs: 0.0,
        vgs: 3.0,
        vds: 0.05,
        ves: 0.0,
        vps: 0.0,
    };
    let op = eval::eval_dc(&sized, &mc, bias, 1.0);
    assert!(op.ids.is_finite() && op.ids > 0.0, "ids={}", op.ids);
    assert!(op.gds.is_finite() && op.gds > 0.0, "gds={}", op.gds);
    assert!(op.gm.is_finite() && op.gm >= 0.0, "gm={}", op.gm);
    assert!(op.von.is_finite());
    assert!(op.ids < 1.0, "ids unreasonably large: {}", op.ids);
}

#[test]
fn eval_dc_monotonic_in_vg() {
    let model = B3SoiPdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiPdSized::new(&model, &geom(), 300.15).expect("sized");
    let mc = model_consts(&model);
    let mk = |vg: Value| {
        eval::eval_dc(
            &sized,
            &mc,
            B3SoiPdBias {
                vbs: 0.0,
                vgs: vg,
                vds: 0.05,
                ves: 0.0,
                vps: 0.0,
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
    let model = B3SoiPdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiPdSized::new(&model, &geom(), 300.15).expect("sized");
    let mc = model_consts(&model);
    for vg_i in 0..=15 {
        for vd_i in 0..=30 {
            for vbs_i in 0..=4 {
                let bias = B3SoiPdBias {
                    vbs: 0.1 * vbs_i as Value,
                    vgs: 0.1 * vg_i as Value,
                    vds: 0.1 * vd_i as Value,
                    ves: 0.0,
                    vps: 0.0,
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

#[test]
fn forward_bias_has_body_coupling() {
    // PD keeps the body diodes + BJT active, so a forward-biased body must
    // produce nonzero body-junction conductance (unlike FD which zeroes them).
    let model = B3SoiPdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiPdSized::new(&model, &geom(), 300.15).expect("sized");
    let mc = model_consts(&model);
    let bias = B3SoiPdBias {
        vbs: 0.6,
        vgs: 1.0,
        vds: 0.5,
        ves: 0.0,
        vps: 0.0,
    };
    let op = eval::eval_dc(&sized, &mc, bias, 1.0);
    assert!(op.cbody.is_finite());
    assert!(
        op.gbbs.abs() > 0.0 || op.cjs.abs() > 0.0 || op.cjd.abs() > 0.0,
        "expected nonzero body coupling under forward bias"
    );
}

#[test]
fn charge_matrix_is_consistent_with_charges() {
    let model = B3SoiPdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiPdSized::new(&model, &geom(), 300.15).expect("sized");
    let mc = model_consts(&model);
    let charge = |vg: Value, vd: Value, vb: Value, ve: Value| {
        eval::eval(
            &sized,
            &mc,
            B3SoiPdBias {
                vbs: vb,
                vgs: vg,
                vds: vd,
                ves: ve,
                vps: 0.0,
            },
            1.0,
            true,
        )
        .charge
        .unwrap()
    };
    let (vg, vd, vb, ve) = (1.2_f64, 0.8, 0.05, 0.0);
    let h = 1e-6;
    let c0 = charge(vg, vd, vb, ve);
    let cp = charge(vg + h, vd, vb, ve);
    let cm = charge(vg - h, vd, vb, ve);
    let dqg_dvg = (cp.qg - cm.qg) / (2.0 * h);
    let ok = |analytic: Value, fd: Value| {
        (analytic - fd).abs() <= 1e-2 * analytic.abs().max(fd.abs()) + 1e-14
    };
    assert!(ok(c0.gcggb, dqg_dvg), "cggb {} vs FD {}", c0.gcggb, dqg_dvg);
    let total = c0.qg + c0.qb + c0.qd + c0.qe;
    assert!(total.is_finite(), "charge sum not finite: {total}");
}

/// The stamped body-row conductances must be the Jacobian of the body current:
/// `gbbs = d(-cb)/dVbs`, `gbgs = d(-cb)/dVgs`, `gbds = d(-cb)/dVds`, where
/// `-cb = Iii + Idgidl + Isgidl - Ibs - Ibd` is the current INTO the body
/// node. A sign error here lets impact-ionization/GIDL currents drive the
/// floating body to a runaway with Newton unable to respond (the conductance
/// pulls the wrong way). Checked across switching-edge biases where II and
/// GIDL are active.
#[test]
fn body_current_jacobian_matches_conductances() {
    let model = B3SoiPdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiPdSized::new(&model, &geom(), 300.15).expect("sized");
    let mc = model_consts(&model);
    let op_at = |vg: Value, vd: Value, vb: Value, ve: Value| {
        eval::eval(
            &sized,
            &mc,
            B3SoiPdBias {
                vbs: vb,
                vgs: vg,
                vds: vd,
                ves: ve,
                vps: 0.0,
            },
            1.0,
            false,
        )
    };

    let biases: [(Value, Value, Value, Value); 6] = [
        (1.2, 0.8, 0.05, 0.0),
        (0.1, 1.8, 0.2, 0.0),
        (0.0, 2.0, 0.4, 0.0),
        (2.0, 1.5, 0.6, 0.0),
        (0.5, 1.0, -0.3, 0.0),
        (1.0, 0.01, 0.3, 0.0),
    ];
    let h = 1e-7;
    let ok = |analytic: Value, fd: Value| {
        (analytic - fd).abs() <= 2e-2 * analytic.abs().max(fd.abs()) + 1e-12
    };

    for &(vg, vd, vb, ve) in &biases {
        let o0 = op_at(vg, vd, vb, ve);
        let into_body = |o: &eval::B3SoiPdOp| -o.cb;

        let fd_b = (into_body(&op_at(vg, vd, vb + h, ve)) - into_body(&op_at(vg, vd, vb - h, ve)))
            / (2.0 * h);
        let fd_g = (into_body(&op_at(vg + h, vd, vb, ve)) - into_body(&op_at(vg - h, vd, vb, ve)))
            / (2.0 * h);
        let fd_d = (into_body(&op_at(vg, vd + h, vb, ve)) - into_body(&op_at(vg, vd - h, vb, ve)))
            / (2.0 * h);

        assert!(
            ok(o0.gbbs, fd_b),
            "bias {:?}: gbbs {:.6e} vs FD {fd_b:.6e}",
            (vg, vd, vb, ve),
            o0.gbbs,
        );
        assert!(
            ok(o0.gbgs, fd_g),
            "bias {:?}: gbgs {:.6e} vs FD {fd_g:.6e}",
            (vg, vd, vb, ve),
            o0.gbgs,
        );
        assert!(
            ok(o0.gbds, fd_d),
            "bias {:?}: gbds {:.6e} vs FD {fd_d:.6e}",
            (vg, vd, vb, ve),
            o0.gbds,
        );
    }
}

/// The stamped capacitance matrix must be the Jacobian of the node charges:
/// `gcXgb = dqX/dVg`, `gcXdb = dqX/dVd`, `gcXeb = dqX/dVe`, and
/// `gcXsb = dqX/dVs = -(dqX/dVg + dqX/dVd + dqX/dVb + dqX/dVe)`. Any mismatch
/// makes the transient Newton diverge even though the charges themselves look
/// sane. Checked by central finite differences across bias regimes covering
/// subthreshold, strong inversion, forward-biased body (a switching SOI ring),
/// vds near zero, and the inverse mode.
#[test]
fn charge_matrix_matches_charge_jacobian() {
    let model = B3SoiPdModel::from_params(&n1_params(), false, 300.15);
    let sized = B3SoiPdSized::new(&model, &geom(), 300.15).expect("sized");
    let mc = model_consts(&model);
    let charge = |vg: Value, vd: Value, vb: Value, ve: Value| {
        eval::eval(
            &sized,
            &mc,
            B3SoiPdBias {
                vbs: vb,
                vgs: vg,
                vds: vd,
                ves: ve,
                vps: 0.0,
            },
            1.0,
            true,
        )
        .charge
        .unwrap()
    };

    let biases: [(Value, Value, Value, Value); 6] = [
        (1.2, 0.8, 0.05, 0.0),
        (0.6, 0.01, 0.4, 0.0),
        (2.0, 0.4, 0.3, 0.0),
        (0.3, 1.5, 0.5, 0.0),
        (1.5, 0.002, 0.45, 0.0),
        (1.2, -0.8, 0.05, 0.0),
    ];
    let h = 1e-7;
    // FD noise on fF-scale charges sits near 1e-17; tolerate 2% + 2e-16.
    let ok = |analytic: Value, fd: Value| {
        (analytic - fd).abs() <= 2e-2 * analytic.abs().max(fd.abs()) + 2e-16
    };

    for &(vg, vd, vb, ve) in &biases {
        let c0 = charge(vg, vd, vb, ve);
        let q = |c: &eval::B3SoiPdCharge| [c.qg, c.qb, c.qd, c.qe];

        let dg = {
            let (p, m) = (charge(vg + h, vd, vb, ve), charge(vg - h, vd, vb, ve));
            [0, 1, 2, 3].map(|k| (q(&p)[k] - q(&m)[k]) / (2.0 * h))
        };
        let dd = {
            let (p, m) = (charge(vg, vd + h, vb, ve), charge(vg, vd - h, vb, ve));
            [0, 1, 2, 3].map(|k| (q(&p)[k] - q(&m)[k]) / (2.0 * h))
        };
        let db = {
            let (p, m) = (charge(vg, vd, vb + h, ve), charge(vg, vd, vb - h, ve));
            [0, 1, 2, 3].map(|k| (q(&p)[k] - q(&m)[k]) / (2.0 * h))
        };
        let de = {
            let (p, m) = (charge(vg, vd, vb, ve + h), charge(vg, vd, vb, ve - h));
            [0, 1, 2, 3].map(|k| (q(&p)[k] - q(&m)[k]) / (2.0 * h))
        };

        let rows = [
            ("qg", c0.gcggb, c0.gcgdb, c0.gcgsb, c0.gcgeb),
            ("qb", c0.gcbgb, c0.gcbdb, c0.gcbsb, c0.gcbeb),
            ("qd", c0.gcdgb, c0.gcddb, c0.gcdsb, c0.gcdeb),
            ("qe", c0.gcegb, c0.gcedb, c0.gcesb, c0.gceeb),
        ];
        for (k, (name, gcg, gcd, gcs, gce)) in rows.into_iter().enumerate() {
            let ds = -(dg[k] + dd[k] + db[k] + de[k]);
            assert!(
                ok(gcg, dg[k]),
                "bias {:?}: gc{name}gb {gcg:.6e} vs FD {:.6e}",
                (vg, vd, vb, ve),
                dg[k]
            );
            assert!(
                ok(gcd, dd[k]),
                "bias {:?}: gc{name}db {gcd:.6e} vs FD {:.6e}",
                (vg, vd, vb, ve),
                dd[k]
            );
            assert!(
                ok(gce, de[k]),
                "bias {:?}: gc{name}eb {gce:.6e} vs FD {:.6e}",
                (vg, vd, vb, ve),
                de[k]
            );
            assert!(
                ok(gcs, ds),
                "bias {:?}: gc{name}sb {gcs:.6e} vs FD {ds:.6e}",
                (vg, vd, vb, ve),
            );
        }
    }
}
