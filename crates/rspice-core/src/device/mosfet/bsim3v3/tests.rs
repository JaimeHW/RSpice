//! Unit tests for the BSIM3v3.3 (level 8/9/49) model module.
//!
//! Two layers:
//! - analytic/self-consistency tests that run everywhere: temperature-setup
//!   physicality, long-channel square-law limit, subthreshold slope,
//!   derivative-vs-finite-difference agreement, and continuity across the
//!   `Vgsteff` and `Vds = Vdsat` transitions;
//! - ngspice-pinned tests: bias tables produced by a local ngspice-46 build
//!   from the decks in `testdata/` (see the `#[ignore]`d live-oracle test
//!   which regenerates them when `NGSPICE_EXE` is set).

use super::*;
use crate::Value;
use std::collections::HashMap;

/// 0.18um-style NMOS card (own values, written for this test suite; not a
/// foundry deck). Mirrors testdata/nmos018.mod.
fn nmos018() -> HashMap<String, Value> {
    let pairs: &[(&str, Value)] = &[
        ("LEVEL", 49.0),
        ("TNOM", 27.0),
        ("TOX", 4.1e-9),
        ("XJ", 1.0e-7),
        ("NCH", 2.35e17),
        ("VTH0", 0.37),
        ("K1", 0.59),
        ("K2", 0.0026),
        ("K3", 1e-3),
        ("K3B", 2.3),
        ("W0", 1e-7),
        ("NLX", 1.8e-7),
        ("DVT0W", 0.0),
        ("DVT1W", 0.0),
        ("DVT2W", 0.0),
        ("DVT0", 1.4),
        ("DVT1", 0.41),
        ("DVT2", 0.027),
        ("U0", 270.0),
        ("UA", -1.4e-9),
        ("UB", 2.4e-18),
        ("UC", 6.2e-11),
        ("VSAT", 9.5e4),
        ("A0", 1.9),
        ("AGS", 0.41),
        ("B0", -1e-8),
        ("B1", 0.0),
        ("KETA", 0.04),
        ("A1", 0.0),
        ("A2", 1.0),
        ("RDSW", 110.0),
        ("PRWG", 0.5),
        ("PRWB", -0.2),
        ("WR", 1.0),
        ("WINT", 5e-9),
        ("LINT", 1.5e-8),
        ("DWG", -4e-9),
        ("DWB", 6e-9),
        ("VOFF", -0.092),
        ("NFACTOR", 2.2),
        ("CIT", 0.0),
        ("CDSC", 2.4e-4),
        ("CDSCD", 0.0),
        ("CDSCB", 0.0),
        ("ETA0", 2.5e-3),
        ("ETAB", 5e-5),
        ("DSUB", 1.5e-2),
        ("PCLM", 0.72),
        ("PDIBLC1", 0.21),
        ("PDIBLC2", 2.5e-3),
        ("PDIBLCB", -0.1),
        ("DROUT", 0.78),
        ("PSCBE1", 8.5e8),
        ("PSCBE2", 5e-7),
        ("PVAG", 0.01),
        ("DELTA", 0.01),
        ("RSH", 0.0),
        ("MOBMOD", 1.0),
        ("PRT", 0.0),
        ("UTE", -1.5),
        ("KT1", -0.11),
        ("KT1L", 0.0),
        ("KT2", 0.022),
        ("UA1", 4.31e-9),
        ("UB1", -7.61e-18),
        ("UC1", -5.6e-11),
        ("AT", 3.3e4),
        ("WL", 0.0),
        ("WLN", 1.0),
        ("WW", 0.0),
        ("WWN", 1.0),
        ("WWL", 0.0),
        ("LL", 0.0),
        ("LLN", 1.0),
        ("LW", 0.0),
        ("LWN", 1.0),
        ("LWL", 0.0),
        ("CAPMOD", 3.0),
        ("XPART", 0.5),
        ("CGDO", 7.9e-10),
        ("CGSO", 7.9e-10),
        ("CGBO", 1e-12),
        ("CJ", 9.5e-4),
        ("PB", 0.8),
        ("MJ", 0.38),
        ("CJSW", 2.4e-10),
        ("PBSW", 0.8),
        ("MJSW", 0.13),
        ("CJSWG", 3.3e-10),
        ("PBSWG", 0.8),
        ("MJSWG", 0.13),
        ("CF", 0.0),
        ("CLC", 1e-7),
        ("CLE", 0.6),
        ("VOFFCV", -0.02),
        ("NOFF", 1.8),
        ("ACDE", 0.6),
        ("MOIN", 15.0),
        ("JS", 1.5e-6),
        ("JSW", 2.5e-12),
        ("NJ", 1.0),
        ("XTI", 3.0),
    ];
    pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
}

const T300: Value = 300.15;
const GMIN: Value = 1e-12;

fn device(l: Value, w: Value) -> Bsim3v3 {
    let model = Arc::new(Bsim3v3Model::from_params(&nmos018(), false, T300));
    let geom = Bsim3v3Geometry {
        l,
        w,
        drain_area: w * 0.42e-6,
        source_area: w * 0.42e-6,
        drain_perimeter: 2.0 * (w + 0.42e-6),
        source_perimeter: 2.0 * (w + 0.42e-6),
        ..Bsim3v3Geometry::default()
    };
    Bsim3v3::new("m1".to_string(), model, geom, T300).expect("device")
}

fn device_with_card(card: HashMap<String, Value>, l: Value, w: Value) -> Bsim3v3 {
    let model = Arc::new(Bsim3v3Model::from_params(&card, false, T300));
    let geom = Bsim3v3Geometry {
        l,
        w,
        drain_area: w * 0.42e-6,
        source_area: w * 0.42e-6,
        drain_perimeter: 2.0 * (w + 0.42e-6),
        source_perimeter: 2.0 * (w + 0.42e-6),
        ..Bsim3v3Geometry::default()
    };
    Bsim3v3::new("m1".to_string(), model, geom, T300).expect("device")
}

fn construct_device_with_card(card: HashMap<String, Value>) -> Result<Bsim3v3, String> {
    let model = Arc::new(Bsim3v3Model::from_params(&card, false, T300));
    let geom = Bsim3v3Geometry {
        l: 0.18e-6,
        w: 10e-6,
        drain_area: 4.2e-12,
        source_area: 4.2e-12,
        drain_perimeter: 20.84e-6,
        source_perimeter: 20.84e-6,
        ..Bsim3v3Geometry::default()
    };
    Bsim3v3::new("m1".to_string(), model, geom, T300)
}

fn op_at(dev: &Bsim3v3, vds: Value, vgs: Value, vbs: Value) -> Bsim3v3Op {
    dev.eval(Bsim3v3Bias { vds, vgs, vbs }, GMIN, false)
        .expect("dc eval")
}

#[test]
fn temp_setup_is_finite_and_physical() {
    let dev = device(0.18e-6, 10e-6);
    let p = &dev.size;
    assert!(p.phi > 0.7 && p.phi < 1.1, "phi={}", p.phi);
    assert!(p.vbi > p.phi, "vbi={} phi={}", p.vbi, p.phi);
    assert!(p.u0temp > 0.0 && p.u0temp < 0.1, "u0temp={}", p.u0temp);
    assert!(p.vsattemp > 1e4, "vsattemp={}", p.vsattemp);
    assert!(p.rds0 > 0.0, "rds0={}", p.rds0);
    assert!(p.k1ox > 0.0 && p.vth0 > 0.0);
    assert!(dev.model_temp.vtm > 0.025 && dev.model_temp.vtm < 0.027);
    assert!(dev.model_temp.vcrit > 0.5);
    assert!(p.vfbzb.is_finite() && p.vfbzb < 0.0, "vfbzb={}", p.vfbzb);
}

#[test]
fn size_cache_reuses_knots() {
    let model = Arc::new(Bsim3v3Model::from_params(&nmos018(), false, T300));
    let mt = Bsim3v3ModelTemp::new(&model, T300);
    let mut cache = SizeDepCache::new();
    let a = cache.get(&model, &mt, 0.18e-6, 10e-6).expect("knot");
    let b = cache.get(&model, &mt, 0.18e-6, 10e-6).expect("knot");
    let c = cache.get(&model, &mt, 0.5e-6, 10e-6).expect("knot");
    assert!(Arc::ptr_eq(&a, &b));
    assert!(!Arc::ptr_eq(&a, &c));
}

#[test]
fn long_channel_square_law_limit() {
    // 10u/10u long channel at low Vds: Id ~ ueff Cox W/L (Vgst - Abulk/2 Vds) Vds.
    let dev = device(10e-6, 10e-6);
    let vgs = 1.5;
    let vds = 0.05;
    let op = op_at(&dev, vds, vgs, 0.0);
    let p = &dev.size;
    let expected = op.ueff * dev.model.cox * p.weff / p.leff
        * (op.vgsteff - 0.5 * op.abulk * op.vdseff)
        * op.vdseff
        / (1.0 + op.vdseff / (2.0 * p.vsattemp / op.ueff * p.leff));
    // Same formula modulo Rds and the Early factors; at vds=50mV they are
    // sub-percent for this geometry.
    let rel = (op.cd - expected).abs() / expected.abs();
    assert!(
        rel < 0.02,
        "Id={:.6e} square-law={:.6e} rel={rel:.3e}",
        op.cd,
        expected
    );
}

#[test]
fn subthreshold_slope_matches_n_vtm_ln10() {
    // Deep subthreshold: d(log10 Id)/dVgs = 1 / (n * vtm * ln 10).
    let dev = device(1e-6, 10e-6);
    let vds = 0.1;
    let i1 = op_at(&dev, vds, 0.05, 0.0).cd;
    let i2 = op_at(&dev, vds, 0.15, 0.0).cd;
    assert!(i1 > 0.0 && i2 > i1, "i1={i1:.3e} i2={i2:.3e}");
    let slope_mv_per_dec = 0.1 / (i2.log10() - i1.log10()) * 1e3;

    // The model n at this bias.
    let p = &dev.size;
    let mt = &dev.model_temp;
    // Reconstruct n exactly as eval does at Vbseff=0 (vbs=0): Xdep = Xdep0.
    let theta0 = {
        let lt1 = mt.factor1 * p.xdep0.sqrt();
        let t0: Value = -0.5 * p.dvt1 * p.leff / lt1;
        let t1 = t0.exp();
        t1 * (1.0 + 2.0 * t1)
    };
    let n = 1.0
        + (p.nfactor * super::common::EPSSI / p.xdep0 + (p.cdsc + p.cdscd * vds) * theta0 + p.cit)
            / dev.model.cox;
    let expected = n * mt.vtm * (10.0_f64).ln() * 1e3;
    let rel = (slope_mv_per_dec - expected).abs() / expected;
    assert!(
        rel < 0.02,
        "slope {slope_mv_per_dec:.2} mV/dec vs n*vt*ln10 {expected:.2} mV/dec"
    );
}

#[test]
fn vth_rolloff_sign_short_channel() {
    // DVT0/DVT1 charge sharing: von falls as L shrinks once the NLX
    // reverse-short-channel term is removed (the full card's halo term
    // legitimately raises Vth at 0.18u, so isolate the roll-off here).
    let mut card = nmos018();
    card.insert("NLX".to_string(), 0.0);
    let dev_at = |l: Value| {
        let model = Arc::new(Bsim3v3Model::from_params(&card, false, T300));
        Bsim3v3::new(
            "m1".to_string(),
            model,
            Bsim3v3Geometry {
                l,
                w: 10e-6,
                ..Bsim3v3Geometry::default()
            },
            T300,
        )
        .expect("device")
    };
    let long = op_at(&dev_at(10e-6), 0.05, 1.0, 0.0).von;
    let short = op_at(&dev_at(0.18e-6), 0.05, 1.0, 0.0).von;
    assert!(
        short < long,
        "Vth roll-off sign: von(0.18u)={short:.4} !< von(10u)={long:.4}"
    );
    // And the full card's NLX term raises Vth back up at short L (RSCE).
    let with_nlx = op_at(&device(0.18e-6, 10e-6), 0.05, 1.0, 0.0).von;
    assert!(with_nlx > short, "NLX should raise short-channel Vth");
}

#[test]
fn body_bias_raises_vth() {
    let dev = device(0.5e-6, 10e-6);
    let v0 = op_at(&dev, 0.05, 1.0, 0.0).von;
    let vb = op_at(&dev, 0.05, 1.0, -0.9).von;
    assert!(
        vb > v0 + 0.05,
        "body effect: von(vbs=-0.9)={vb:.4} von(0)={v0:.4}"
    );
}

#[test]
fn gm_gds_gmbs_match_finite_differences() {
    let dev = device(0.18e-6, 10e-6);
    let h = 1e-6;
    let biases: [(Value, Value, Value); 8] = [
        (0.05, 0.8, 0.0),
        (1.2, 0.8, 0.0),
        (1.8, 1.8, 0.0),
        (0.6, 0.45, -0.5),
        (1.0, 0.25, 0.0),  // subthreshold
        (0.02, 1.8, -0.9), // deep linear, body bias
        (1.5, 0.37, 0.3),  // near Vth, forward body
        (0.4, 1.2, -1.5),
    ];
    for &(vds, vgs, vbs) in &biases {
        let op = op_at(&dev, vds, vgs, vbs);
        // Drain current as stamped: Id = cd + csub? No — cd is the channel
        // current; gm/gds/gmbs are its derivatives in the mode frame.
        let id = |vds: Value, vgs: Value, vbs: Value| op_at(&dev, vds, vgs, vbs).cd;
        let fd_gm = (id(vds, vgs + h, vbs) - id(vds, vgs - h, vbs)) / (2.0 * h);
        let fd_gds = (id(vds + h, vgs, vbs) - id(vds - h, vgs, vbs)) / (2.0 * h);
        let fd_gmb = (id(vds, vgs, vbs + h) - id(vds, vgs, vbs - h)) / (2.0 * h);
        let ok = |a: Value, b: Value| (a - b).abs() <= 1e-4 * a.abs().max(b.abs()) + 1e-12;
        assert!(
            ok(op.gm, fd_gm),
            "({vds},{vgs},{vbs}): gm {:.8e} vs FD {fd_gm:.8e}",
            op.gm
        );
        assert!(
            ok(op.gds, fd_gds),
            "({vds},{vgs},{vbs}): gds {:.8e} vs FD {fd_gds:.8e}",
            op.gds
        );
        assert!(
            ok(op.gmbs, fd_gmb),
            "({vds},{vgs},{vbs}): gmbs {:.8e} vs FD {fd_gmb:.8e}",
            op.gmbs
        );
    }
}

#[test]
fn substrate_current_jacobian_matches() {
    let mut card = nmos018();
    card.insert("ALPHA0".to_string(), 1e-8);
    card.insert("ALPHA1".to_string(), 0.5);
    card.insert("BETA0".to_string(), 15.0);
    let model = Arc::new(Bsim3v3Model::from_params(&card, false, T300));
    let dev = Bsim3v3::new(
        "m1".to_string(),
        model,
        Bsim3v3Geometry {
            l: 0.18e-6,
            w: 10e-6,
            ..Bsim3v3Geometry::default()
        },
        T300,
    )
    .expect("device");
    let h = 1e-6;
    let isub = |vds: Value, vgs: Value, vbs: Value| op_at(&dev, vds, vgs, vbs).csub;
    let (vds, vgs, vbs) = (1.8, 1.0, 0.0);
    let op = op_at(&dev, vds, vgs, vbs);
    assert!(op.csub > 0.0, "Isub should be active: {:.3e}", op.csub);
    let fd_g = (isub(vds, vgs + h, vbs) - isub(vds, vgs - h, vbs)) / (2.0 * h);
    let fd_d = (isub(vds + h, vgs, vbs) - isub(vds - h, vgs, vbs)) / (2.0 * h);
    let fd_b = (isub(vds, vgs, vbs + h) - isub(vds, vgs, vbs - h)) / (2.0 * h);
    let ok = |a: Value, b: Value| (a - b).abs() <= 1e-3 * a.abs().max(b.abs()) + 1e-15;
    assert!(ok(op.gbgs, fd_g), "gbgs {:.6e} vs FD {fd_g:.6e}", op.gbgs);
    assert!(ok(op.gbds, fd_d), "gbds {:.6e} vs FD {fd_d:.6e}", op.gbds);
    assert!(ok(op.gbbs, fd_b), "gbbs {:.6e} vs FD {fd_b:.6e}", op.gbbs);
}

#[test]
fn continuity_across_vgsteff_transition() {
    // Sweep Vgs through the subthreshold-to-strong-inversion transition and
    // assert Id, gm, gds move without jumps (relative step-to-step change
    // bounded; Id spans decades so compare against the local trapezoid).
    let dev = device(0.18e-6, 10e-6);
    let vds = 0.5;
    let mut prev: Option<Bsim3v3Op> = None;
    let mut vgs = 0.0;
    while vgs <= 1.8 + 1e-12 {
        let op = op_at(&dev, vds, vgs, 0.0);
        assert!(op.cd.is_finite() && op.gm.is_finite() && op.gds.is_finite());
        assert!(op.cd >= 0.0);
        if let Some(prev_op) = &prev {
            // gm bounds the current change over the 1 mV step (smoothness):
            let dvgs = 1e-3;
            let didl = (op.cd - prev_op.cd).abs();
            let bound = 1.5 * dvgs * op.gm.max(prev_op.gm).max(1e-18) + 1e-15;
            assert!(
                didl <= bound,
                "Id jump at vgs={vgs:.3}: dI={didl:.3e} bound={bound:.3e}"
            );
        }
        prev = Some(op);
        vgs += 1e-3;
    }
}

#[test]
fn continuity_across_vdsat() {
    let dev = device(0.18e-6, 10e-6);
    let vgs = 1.0;
    let mut prev: Option<Bsim3v3Op> = None;
    let mut vds = 0.0;
    while vds <= 1.8 + 1e-12 {
        let op = op_at(&dev, vds, vgs, 0.0);
        if let Some(prev_op) = &prev {
            let dvds = 1e-3;
            let didl = (op.cd - prev_op.cd).abs();
            let bound = 1.5 * dvds * op.gds.max(prev_op.gds).max(1e-18) + 1e-15;
            assert!(
                didl <= bound,
                "Id jump at vds={vds:.3}: dI={didl:.3e} bound={bound:.3e}"
            );
            // gds itself must be continuous to ~1% per mV through Vdsat.
            let dg = (op.gds - prev_op.gds).abs();
            assert!(
                dg <= 0.05 * op.gds.abs().max(prev_op.gds.abs()) + 1e-15,
                "gds jump at vds={vds:.3}"
            );
        }
        prev = Some(op);
        vds += 1e-3;
    }
}

#[test]
fn inverse_mode_is_symmetric_for_symmetric_junctions() {
    // With identical S/D areas, swapping source and drain mirrors the device:
    // Id(vds) in normal mode equals Id(-vds) re-expressed, junction terms swap.
    let dev = device(0.5e-6, 10e-6);
    let fwd = op_at(&dev, 0.7, 1.2, 0.0);
    let rev = op_at(&dev, -0.7, 0.5, -0.7); // vgd=1.2, vbd=0 in swapped frame
    assert_eq!(fwd.mode, 1);
    assert_eq!(rev.mode, -1);
    let rel = (fwd.cd - rev.cd).abs() / fwd.cd.abs();
    assert!(rel < 1e-12, "fwd {:.10e} vs rev {:.10e}", fwd.cd, rev.cd);
}

#[test]
fn junction_diode_matches_ijth_linearization() {
    let dev = device(0.5e-6, 10e-6);
    // Reverse bias: -Is*(1) + gmin*v.
    let op = op_at(&dev, 0.0, 0.0, -1.0);
    let is_s = dev.inst.source_sat_current;
    let expected = is_s * ((-1.0_f64 / dev.model_temp.vtm).exp() - 1.0) - GMIN;
    let rel = (op.cbs - expected).abs() / expected.abs();
    assert!(rel < 1e-12, "cbs={:.6e} expected={:.6e}", op.cbs, expected);

    // Far above vjsm the current is linearized: slope continuous at vjsm.
    let (vjsm, _) = dev.inst.vjsm.expect("ijth anchors");
    let below = op_at(&dev, 0.0, 0.0, vjsm - 1e-9).cbs;
    let above = op_at(&dev, 0.0, 0.0, vjsm + 1e-9).cbs;
    assert!(
        (above - below).abs() < 1e-6 * above.abs(),
        "ijth linearization discontinuous at vjsm: {below:.8e} vs {above:.8e}"
    );
}

#[test]
fn charge_model_capmod3_jacobian_consistency() {
    let dev = device(0.18e-6, 10e-6);
    let charge = |vds: Value, vgs: Value, vbs: Value| {
        dev.eval(Bsim3v3Bias { vds, vgs, vbs }, GMIN, true)
            .expect("charge eval")
            .charge
            .expect("charge state")
    };
    let biases: [(Value, Value, Value); 6] = [
        (0.05, 1.2, 0.0),
        (1.2, 1.2, 0.0),
        (0.6, 0.3, 0.0),
        (1.8, 1.8, -0.9),
        (0.001, 0.9, -0.3),
        (-0.7, 0.5, -0.7), // inverse mode
    ];
    let h = 1e-7;
    // Total node charge must be neutral: qg + qb + qd + qs = 0 with
    // qs = -(qgate+qbulk+qdrn); intrinsic+overlap is closed by construction,
    // so check finiteness + the intrinsic matrix against FD of the charges.
    for &(vds, vgs, vbs) in &biases {
        let c0 = charge(vds, vgs, vbs);
        assert!(c0.qgate.is_finite() && c0.qbulk.is_finite() && c0.qdrn.is_finite());
        assert!(c0.capbs > 0.0 && c0.capbd > 0.0);

        // The c*** matrix is d(q_intrinsic)/dV in the mode frame; validate
        // the gate row via total qgate minus overlap parts.
        let qg_int = |vds: Value, vgs: Value, vbs: Value| {
            let c = charge(vds, vgs, vbs);
            let vgd = vgs - vds;
            let vgb = vgs - vbs;
            // Strip the overlap lump added at assembly to recover the
            // intrinsic gate charge (qgdo/qgso reconstructed like the eval).
            let p = &dev.size;
            let t0 = vgd + super::common::DELTA_1;
            let t1 = (t0 * t0 + 4.0 * super::common::DELTA_1).sqrt();
            let t2 = 0.5 * (t0 - t1);
            let t3 = p.weff_cv * p.cgdl;
            let t4 = (1.0 - 4.0 * t2 / p.ckappa).sqrt();
            let qgdo = (p.cgdo + t3) * vgd - t3 * (t2 + 0.5 * p.ckappa * (t4 - 1.0));
            let t0 = vgs + super::common::DELTA_1;
            let t1 = (t0 * t0 + 4.0 * super::common::DELTA_1).sqrt();
            let t2 = 0.5 * (t0 - t1);
            let t3 = p.weff_cv * p.cgsl;
            let t4 = (1.0 - 4.0 * t2 / p.ckappa).sqrt();
            let qgso = (p.cgso + t3) * vgs - t3 * (t2 + 0.5 * p.ckappa * (t4 - 1.0));
            c.qgate - qgdo - qgso - p.cgbo * vgb
        };
        // FD in the mode frame: for mode>0, d/dVg at fixed Vd,Vs,Vb is
        // d/dvgs; cggb is dQg/dVg with the b3ld sign/frame conventions.
        if vds >= 0.0 {
            let fd_g = (qg_int(vds, vgs + h, vbs) - qg_int(vds, vgs - h, vbs)) / (2.0 * h);
            let ok = |a: Value, b: Value| (a - b).abs() <= 2e-2 * a.abs().max(b.abs()) + 1e-16;
            assert!(
                ok(c0.cggb, fd_g),
                "({vds},{vgs},{vbs}): cggb {:.6e} vs FD {fd_g:.6e}",
                c0.cggb
            );
        }
    }
}

#[test]
fn ngspice_pinned_nmos_charges_capmod2() {
    let mut card = nmos018();
    card.insert("CAPMOD".to_string(), 2.0);
    let dev = device_with_card(card, 0.18e-6, 10e-6);
    let charge_at = |vds: Value, vgs: Value| {
        dev.eval(Bsim3v3Bias { vds, vgs, vbs: 0.0 }, GMIN, true)
            .expect("CAPMOD=2 charge eval")
            .charge
            .expect("charges")
    };

    // ngspice-46, nmos_oracle.sp with models018.lib capmod=2, `ngspice_con.exe -b`.
    let c = charge_at(0.05, 0.9);
    assert_rel(c.qg_state(), 2.553329460e-14, "capmod2 qg(0.9)");
    assert_rel(c.qb_state(), -7.575462320e-15, "capmod2 qb(0.9)");
    assert_rel(c.qd_state(), -8.536083460e-15, "capmod2 qd(0.9)");
    assert_rel(c.cggb, 1.252411080e-14, "capmod2 cgg(0.9)");
    assert_rel(c.cgdb, -4.709989470e-15, "capmod2 cgd(0.9)");
    assert_rel(c.cgsb, -7.248269520e-15, "capmod2 cgs(0.9)");
    assert_rel(c.cdgb, -6.149079910e-15, "capmod2 cdg(0.9)");
    assert_rel(c.cddb, 5.439817160e-15, "capmod2 cdd(0.9)");
    assert_rel(c.cdsb, 2.336574110e-15, "capmod2 cds(0.9)");
    assert_rel(c.cbgb, -2.259509250e-16, "capmod2 cbg(0.9)");
    assert_rel(c.cbdb, -6.169644860e-15, "capmod2 cbd(0.9)");
    assert_rel(c.cbsb, 2.575121300e-15, "capmod2 cbs(0.9)");
    assert_rel(c.capbd, 9.753509820e-15, "capmod2 capbd(0.9)");
    assert_rel(c.capbs, 9.890700000e-15, "capmod2 capbs(0.9)");

    let c = charge_at(1.2, 1.0);
    assert_rel(c.qg_state(), 1.868390600e-14, "capmod2 qg(vd=1.2 vg=1.0)");
    assert_rel(c.qb_state(), -1.851946500e-14, "capmod2 qb(vd=1.2 vg=1.0)");
    assert_rel(c.qd_state(), 9.929873520e-15, "capmod2 qd(vd=1.2 vg=1.0)");
    assert_rel(c.cggb, 1.083387950e-14, "capmod2 cgg(vd=1.2 vg=1.0)");
    assert_rel(c.cgdb, 5.280069540e-17, "capmod2 cgd(vd=1.2 vg=1.0)");
    assert_rel(c.cgsb, -9.973132910e-15, "capmod2 cgs(vd=1.2 vg=1.0)");
    assert_rel(c.cdgb, -4.207864430e-15, "capmod2 cdg(vd=1.2 vg=1.0)");
    assert_rel(c.cddb, -2.787440400e-17, "capmod2 cdd(vd=1.2 vg=1.0)");
    assert_rel(c.cdsb, 5.369810250e-15, "capmod2 cds(vd=1.2 vg=1.0)");
    assert_rel(c.cbgb, -2.418150640e-15, "capmod2 cbg(vd=1.2 vg=1.0)");
    assert_rel(c.cbdb, 2.948112450e-18, "capmod2 cbd(vd=1.2 vg=1.0)");
    assert_rel(c.cbsb, -7.664875970e-16, "capmod2 cbs(vd=1.2 vg=1.0)");
}

#[test]
fn ngspice_pinned_nmos_charges_capmod0() {
    let mut card = nmos018();
    card.insert("CAPMOD".to_string(), 0.0);
    let dev = device_with_card(card, 0.18e-6, 10e-6);
    let charge_at = |vds: Value, vgs: Value| {
        dev.eval(Bsim3v3Bias { vds, vgs, vbs: 0.0 }, GMIN, true)
            .expect("CAPMOD=0 charge eval")
            .charge
            .expect("charges")
    };

    // ngspice-46, models018.lib with capmod=0, `ngspice_con.exe -b`.
    let c = charge_at(0.05, 0.9);
    assert_rel(c.qg_state(), 2.66511147e-14, "capmod0 qg(0.9)");
    assert_rel(c.qb_state(), -7.78252513e-15, "capmod0 qb(0.9)");
    assert_rel(c.qd_state(), -8.99146211e-15, "capmod0 qd(0.9)");
    assert_rel(c.cggb, 1.25891850e-14, "capmod0 cgg(0.9)");
    assert_rel(c.cgdb, -5.72168893e-15, "capmod0 cgd(0.9)");
    assert_rel(c.cgsb, -6.85937461e-15, "capmod0 cgs(0.9)");
    assert_rel(c.cdgb, -6.27411899e-15, "capmod0 cdg(0.9)");
    assert_rel(c.cddb, 6.56848067e-15, "capmod0 cdd(0.9)");
    assert_rel(c.cdsb, 1.66223474e-15, "capmod0 cds(0.9)");
    assert_rel(c.cbgb, -4.09469770e-17, "capmod0 cbg(0.9)");
    assert_rel(c.cbdb, -7.41527240e-15, "capmod0 cbd(0.9)");
    assert_rel(c.cbsb, 3.53490512e-15, "capmod0 cbs(0.9)");
    assert_rel(c.capbd, 9.75350982e-15, "capmod0 capbd(0.9)");
    assert_rel(c.capbs, 9.89070000e-15, "capmod0 capbs(0.9)");

    let c = charge_at(1.2, 1.0);
    assert_rel(c.qg_state(), 1.96280929e-14, "capmod0 qg(vd=1.2 vg=1.0)");
    assert_rel(c.qb_state(), -1.88658981e-14, "capmod0 qb(vd=1.2 vg=1.0)");
    assert_rel(c.qd_state(), 9.63099659e-15, "capmod0 qd(vd=1.2 vg=1.0)");
    assert_rel(c.cggb, 1.07884898e-14, "capmod0 cgg(vd=1.2 vg=1.0)");
    assert_rel(c.cgdb, 0.0, "capmod0 cgd(vd=1.2 vg=1.0)");
    assert_rel(c.cgsb, -1.03402151e-14, "capmod0 cgs(vd=1.2 vg=1.0)");
    assert_rel(c.cdgb, -4.20692667e-15, "capmod0 cdg(vd=1.2 vg=1.0)");
    assert_rel(c.cddb, 0.0, "capmod0 cdd(vd=1.2 vg=1.0)");
    assert_rel(c.cdsb, 5.54615958e-15, "capmod0 cds(vd=1.2 vg=1.0)");
    assert_rel(c.cbgb, -2.37463645e-15, "capmod0 cbg(vd=1.2 vg=1.0)");
    assert_rel(c.cbdb, 0.0, "capmod0 cbd(vd=1.2 vg=1.0)");
    assert_rel(c.cbsb, -7.52104044e-16, "capmod0 cbs(vd=1.2 vg=1.0)");
}

#[test]
fn ngspice_pinned_nmos_charges_capmod1() {
    let mut card = nmos018();
    card.insert("CAPMOD".to_string(), 1.0);
    let dev = device_with_card(card, 0.18e-6, 10e-6);
    let charge_at = |vds: Value, vgs: Value| {
        dev.eval(Bsim3v3Bias { vds, vgs, vbs: 0.0 }, GMIN, true)
            .expect("CAPMOD=1 charge eval")
            .charge
            .expect("charges")
    };

    // ngspice-46, models018.lib with capmod=1, `ngspice_con.exe -b`.
    let c = charge_at(0.05, 0.9);
    assert_rel(c.qg_state(), 2.55914642e-14, "capmod1 qg(0.9)");
    assert_rel(c.qb_state(), -7.71596505e-15, "capmod1 qb(0.9)");
    assert_rel(c.qd_state(), -8.49491691e-15, "capmod1 qd(0.9)");
    assert_rel(c.cggb, 1.25566096e-14, "capmod1 cgg(0.9)");
    assert_rel(c.cgdb, -5.51479743e-15, "capmod1 cgd(0.9)");
    assert_rel(c.cgsb, -6.53236098e-15, "capmod1 cgs(0.9)");
    assert_rel(c.cdgb, -6.24506572e-15, "capmod1 cdg(0.9)");
    assert_rel(c.cddb, 6.36365191e-15, "capmod1 cdd(0.9)");
    assert_rel(c.cdsb, 1.52453868e-15, "capmod1 cds(0.9)");
    assert_rel(c.cbgb, -6.64781661e-17, "capmod1 cbg(0.9)");
    assert_rel(c.cbdb, -7.21250638e-15, "capmod1 cbd(0.9)");
    assert_rel(c.cbsb, 3.48328362e-15, "capmod1 cbs(0.9)");
    assert_rel(c.capbd, 9.75350982e-15, "capmod1 capbd(0.9)");
    assert_rel(c.capbs, 9.89070000e-15, "capmod1 capbs(0.9)");

    let c = charge_at(1.2, 1.0);
    assert_rel(c.qg_state(), 1.87722498e-14, "capmod1 qg(vd=1.2 vg=1.0)");
    assert_rel(c.qb_state(), -1.86095468e-14, "capmod1 qb(vd=1.2 vg=1.0)");
    assert_rel(c.qd_state(), 9.93074251e-15, "capmod1 qd(vd=1.2 vg=1.0)");
    assert_rel(c.cggb, 1.07858214e-14, "capmod1 cgg(vd=1.2 vg=1.0)");
    assert_rel(c.cgdb, 5.42146046e-17, "capmod1 cgd(vd=1.2 vg=1.0)");
    assert_rel(c.cgsb, -9.95964875e-15, "capmod1 cgs(vd=1.2 vg=1.0)");
    assert_rel(c.cdgb, -4.20547097e-15, "capmod1 cdg(vd=1.2 vg=1.0)");
    assert_rel(c.cddb, -2.95763327e-17, "capmod1 cdd(vd=1.2 vg=1.0)");
    assert_rel(c.cdsb, 5.36852121e-15, "capmod1 cds(vd=1.2 vg=1.0)");
    assert_rel(c.cbgb, -2.37487949e-15, "capmod1 cbg(vd=1.2 vg=1.0)");
    assert_rel(c.cbdb, 4.93806074e-18, "capmod1 cbd(vd=1.2 vg=1.0)");
    assert_rel(c.cbsb, -7.77393676e-16, "capmod1 cbs(vd=1.2 vg=1.0)");
}

#[test]
#[allow(clippy::excessive_precision)]
fn ngspice_pinned_nmos_charges_capmod0_and_1_xpart_branches() {
    let cases = [
        // ngspice-46, models018.lib with CAPMOD/XPART rewritten, high-precision `print`.
        (
            0.0,
            0.4,
            0.05,
            0.9,
            -8.87114004339493e-15,
            -6.27221382721914e-15,
            8.956079350300603e-15,
            -7.42672104956259e-16,
        ),
        (
            0.0,
            0.4,
            1.2,
            1.0,
            1.013121487698900e-14,
            -3.36554133365854e-15,
            0.0,
            4.436927660965850e-15,
        ),
        (
            0.0,
            1.0,
            0.05,
            0.9,
            -8.29874074912211e-15,
            -6.20157696773521e-15,
            1.970544200405465e-14,
            -1.16517744987191e-14,
        ),
        (0.0, 1.0, 1.2, 1.0, 1.213208802179343e-14, 0.0, 0.0, 0.0),
        (
            1.0,
            0.4,
            0.05,
            0.9,
            -8.37479965952086e-15,
            -6.24160029899201e-15,
            8.737187674974698e-15,
            -8.68105048316137e-16,
        ),
        (
            1.0,
            0.4,
            1.2,
            1.0,
            1.037101161427719e-14,
            -3.36437677781087e-15,
            -2.36610661359743e-17,
            4.294816968590364e-15,
        ),
        (
            1.0,
            1.0,
            0.05,
            0.9,
            -7.80921688428095e-15,
            -6.13703529477321e-15,
            1.917955636119728e-14,
            -1.15079225667818e-14,
        ),
        (
            1.0,
            1.0,
            1.2,
            1.0,
            1.213208802179343e-14,
            1.577721810442024e-30,
            0.0,
            0.0,
        ),
    ];

    for &(capmod, xpart, vds, vgs, qd, cdg, cdd, cds) in &cases {
        let mut card = nmos018();
        card.insert("CAPMOD".to_string(), capmod);
        card.insert("XPART".to_string(), xpart);
        let dev = device_with_card(card, 0.18e-6, 10e-6);
        let c = dev
            .eval(Bsim3v3Bias { vds, vgs, vbs: 0.0 }, GMIN, true)
            .unwrap_or_else(|err| panic!("CAPMOD={capmod} XPART={xpart} charge eval: {err}"))
            .charge
            .expect("charges");

        let label = format!("capmod{capmod} xpart{xpart} vd={vds} vg={vgs}");
        assert_rel(c.qd_state(), qd, &format!("{label} qd"));
        assert_rel(c.cdgb, cdg, &format!("{label} cdg"));
        assert_rel(c.cddb, cdd, &format!("{label} cdd"));
        assert_rel(c.cdsb, cds, &format!("{label} cds"));
    }
}

#[test]
#[allow(clippy::excessive_precision)]
fn ngspice_pinned_nmos_charges_capmod0_and_1_inverse_mode() {
    let cases = [
        // ngspice-46, models018.lib with CAPMOD rewritten, vds=-50 mV.
        (
            0.0,
            2.806964788877441e-14,
            -6.60470996619538e-15,
            -1.11788227823833e-14,
            1.259669744494825e-14,
            -5.79877434198245e-15,
            -6.79235222922769e-15,
            -6.28257161661318e-15,
            6.698314312556863e-15,
            1.431273465992502e-15,
            -3.15542117218833e-17,
            -7.59785428313127e-15,
            3.929805297242683e-15,
            1.003340568750000e-14,
            9.890700000000001e-15,
        ),
        (
            1.0,
            2.698652390448081e-14,
            -6.54742778743705e-15,
            -1.06659018796156e-14,
            1.257955746655105e-14,
            -5.62640505875731e-15,
            -6.55313424221581e-15,
            -6.26615196756301e-15,
            6.531908261996990e-15,
            1.282972424884014e-15,
            -4.72535314250278e-17,
            -7.43741146523667e-15,
            3.987189392447785e-15,
            1.003340568750000e-14,
            9.890700000000001e-15,
        ),
    ];

    for &(capmod, qg, qb, qd, cgg, cgd, cgs, cdg, cdd, cds, cbg, cbd, cbs, capbd, capbs) in &cases {
        let mut card = nmos018();
        card.insert("CAPMOD".to_string(), capmod);
        let dev = device_with_card(card, 0.18e-6, 10e-6);
        let c = dev
            .eval(
                Bsim3v3Bias {
                    vds: -0.05,
                    vgs: 0.9,
                    vbs: 0.0,
                },
                GMIN,
                true,
            )
            .unwrap_or_else(|err| panic!("CAPMOD={capmod} inverse charge eval: {err}"))
            .charge
            .expect("charges");

        let label = format!("capmod{capmod} inverse");
        assert_rel(c.qg_state(), qg, &format!("{label} qg"));
        assert_rel(c.qb_state(), qb, &format!("{label} qb"));
        assert_rel(c.qd_state(), qd, &format!("{label} qd"));
        assert_rel(c.cggb, cgg, &format!("{label} cgg"));
        assert_rel(c.cgdb, cgd, &format!("{label} cgd"));
        assert_rel(c.cgsb, cgs, &format!("{label} cgs"));
        assert_rel(c.cdgb, cdg, &format!("{label} cdg"));
        assert_rel(c.cddb, cdd, &format!("{label} cdd"));
        assert_rel(c.cdsb, cds, &format!("{label} cds"));
        assert_rel(c.cbgb, cbg, &format!("{label} cbg"));
        assert_rel(c.cbdb, cbd, &format!("{label} cbd"));
        assert_rel(c.cbsb, cbs, &format!("{label} cbs"));
        assert_rel(c.capbd, capbd, &format!("{label} capbd"));
        assert_rel(c.capbs, capbs, &format!("{label} capbs"));
    }
}

#[test]
#[allow(clippy::excessive_precision)]
fn ngspice_pinned_nmos_charges_xpart_negative_suppresses_intrinsic_charge() {
    for capmod in [0.0, 1.0] {
        let mut card = nmos018();
        card.insert("CAPMOD".to_string(), capmod);
        card.insert("XPART".to_string(), -1.0);
        let dev = device_with_card(card, 0.18e-6, 10e-6);
        let c = dev
            .eval(
                Bsim3v3Bias {
                    vds: 0.05,
                    vgs: 0.9,
                    vbs: 0.0,
                },
                GMIN,
                true,
            )
            .unwrap_or_else(|err| panic!("CAPMOD={capmod} XPART<0 charge eval: {err}"))
            .charge
            .expect("charges");

        let label = format!("capmod{capmod} xpart<0");
        assert_rel(c.qg_state(), 1.381131000000000e-14, &format!("{label} qg"));
        assert_rel(c.qb_state(), -4.91195350563735e-16, &format!("{label} qb"));
        assert_rel(c.qd_state(), -6.21722464943626e-15, &format!("{label} qd"));
        assert_rel(c.cggb, 0.0, &format!("{label} cgg"));
        assert_rel(c.cgdb, 0.0, &format!("{label} cgd"));
        assert_rel(c.cgsb, 0.0, &format!("{label} cgs"));
        assert_rel(c.cdgb, 0.0, &format!("{label} cdg"));
        assert_rel(c.cddb, 0.0, &format!("{label} cdd"));
        assert_rel(c.cdsb, 0.0, &format!("{label} cds"));
        assert_rel(c.cbgb, 0.0, &format!("{label} cbg"));
        assert_rel(c.cbdb, 0.0, &format!("{label} cbd"));
        assert_rel(c.cbsb, 0.0, &format!("{label} cbs"));
        assert_rel(c.capbd, 9.753509824241641e-15, &format!("{label} capbd"));
        assert_rel(c.capbs, 9.890700000000001e-15, &format!("{label} capbs"));
    }
}

#[test]
fn eval_rejects_invalid_capmod_even_when_xpart_suppresses_intrinsic_charge() {
    let mut card = nmos018();
    card.insert("CAPMOD".to_string(), 99.0);
    card.insert("XPART".to_string(), -1.0);
    let model = Bsim3v3Model::from_params(&card, false, T300);
    let mt = Bsim3v3ModelTemp::new(&model, T300);
    let size = Bsim3v3SizeDep::new(&model, &mt, 0.18e-6, 10e-6).expect("size");
    let geom = Bsim3v3Geometry {
        l: 0.18e-6,
        w: 10e-6,
        drain_area: 4.2e-12,
        source_area: 4.2e-12,
        drain_perimeter: 20.84e-6,
        source_perimeter: 20.84e-6,
        ..Bsim3v3Geometry::default()
    };
    let inst = Bsim3v3InstTemp::new(&model, &mt, &size, &geom);
    let err = eval::eval(
        &model,
        &mt,
        &size,
        &inst,
        Bsim3v3Bias {
            vds: 0.05,
            vgs: 0.9,
            vbs: 0.0,
        },
        GMIN,
        true,
    )
    .expect_err("invalid CAPMOD must be rejected before xpart suppression");
    assert!(
        err.contains("CAPMOD=99") && err.contains("invalid"),
        "unexpected invalid CAPMOD error: {err}"
    );
}

#[test]
fn acnqsmod_and_transient_nqs_construct_but_invalid_selectors_reject() {
    let mut ac_card = nmos018();
    ac_card.insert("ACNQSMOD".to_string(), 1.0);
    let ac_dev = construct_device_with_card(ac_card)
        .unwrap_or_else(|err| panic!("ACNQSMOD=1 should construct natively: {err}"));
    assert_eq!(ac_dev.model.acnqs_mod, 1);
    assert_eq!(ac_dev.model.nqs_mod, 0);

    let mut transient_card = nmos018();
    transient_card.insert("NQSMOD".to_string(), 1.0);
    let transient_dev = construct_device_with_card(transient_card)
        .unwrap_or_else(|err| panic!("NQSMOD=1 should construct natively: {err}"));
    assert_eq!(transient_dev.model.nqs_mod, 1);

    let mut invalid_card = nmos018();
    invalid_card.insert("NQSMOD".to_string(), 2.0);
    let err =
        construct_device_with_card(invalid_card).expect_err("invalid NQSMOD selector must reject");
    assert!(
        err.contains("NQSMOD=2") && err.contains("invalid"),
        "typed rejection should name invalid NQSMOD selector: {err}"
    );
}

#[test]
fn limiting_follows_b3ld_sequence() {
    let dev = device(0.5e-6, 10e-6);
    let old = Bsim3v3Bias {
        vds: 0.0,
        vgs: 0.0,
        vbs: 0.0,
    };
    // A huge forward body jump must be clipped by pnjlim.
    let (lim, check) = dev.limit_voltages(
        Bsim3v3Bias {
            vds: 0.1,
            vgs: 0.5,
            vbs: 5.0,
        },
        old,
        0.0,
    );
    assert!(check, "pnjlim should flag the clipped body voltage");
    assert!(lim.vbs < 1.0, "vbs should be limited, got {}", lim.vbs);
    // A small step passes through unchanged.
    let (lim, check) = dev.limit_voltages(
        Bsim3v3Bias {
            vds: 0.01,
            vgs: 0.02,
            vbs: -0.01,
        },
        old,
        0.0,
    );
    assert!(!check);
    assert!((lim.vds - 0.01).abs() < 1e-15 && (lim.vgs - 0.02).abs() < 1e-15);
}

#[test]
fn no_nan_across_wide_sweep() {
    let dev = device(0.18e-6, 10e-6);
    for vg_i in 0..=18 {
        for vd_i in -18..=18 {
            for vb_i in -3..=1 {
                let bias = Bsim3v3Bias {
                    vds: 0.1 * vd_i as Value,
                    vgs: 0.1 * vg_i as Value,
                    vbs: 0.3 * vb_i as Value,
                };
                let op = dev.eval(bias, GMIN, true).expect("eval");
                assert!(op.cd.is_finite(), "cd NaN at {bias:?}");
                assert!(op.gm.is_finite() && op.gds.is_finite() && op.gmbs.is_finite());
                assert!(op.cbs.is_finite() && op.cbd.is_finite());
                let c = op.charge.expect("charges");
                assert!(c.qgate.is_finite() && c.qbulk.is_finite() && c.qdrn.is_finite());
                assert!(c.cggb.is_finite() && c.cddb.is_finite() && c.cbsb.is_finite());
            }
        }
    }
}

// ===================== pinned ngspice references =====================
//
// Values produced by a local ngspice-46 build (Mar 2025 sources) running the
// decks in testdata/ (`wrdata`, 9 significant digits). The live oracle test
// below regenerates and re-checks every sweep; these pinned rows keep the
// oracle agreement in the default test suite. Comparison at 1e-6 relative —
// the port matched the full 10k-point oracle at <= 4.9e-9 relative (print
// quantization), so any drift past 1e-6 is a real regression.

fn assert_rel(ours: Value, reference: Value, what: &str) {
    let tol = 1e-6 * reference.abs().max(ours.abs()) + 1e-22;
    assert!(
        (ours - reference).abs() <= tol,
        "{what}: ours={ours:.9e} ngspice={reference:.9e}"
    );
}

#[test]
fn ngspice_pinned_nmos_idvg_linear() {
    // m1 = 10u/0.18u, vds=0.05, vbs=0, T=27C (nmos_idvg_vd50m_vb0).
    let dev = device(0.18e-6, 10e-6);
    let table: [(Value, Value, Value, Value, Value); 6] = [
        (
            0.0,
            1.888412160e-11,
            5.173372990e-10,
            1.023756720e-10,
            1.701520030e-10,
        ),
        (
            0.3,
            6.554309470e-08,
            1.735273730e-06,
            3.562440100e-07,
            4.889285300e-07,
        ),
        (
            0.6,
            5.074543660e-05,
            6.568843820e-04,
            6.547648060e-04,
            1.636396300e-04,
        ),
        (
            0.9,
            2.881440310e-04,
            7.283444000e-04,
            5.056831740e-03,
            1.921903430e-04,
        ),
        (
            1.2,
            4.767365090e-04,
            5.314142340e-04,
            8.737667630e-03,
            1.589392620e-04,
        ),
        (
            1.8,
            6.899502130e-04,
            2.010887270e-04,
            1.307953140e-02,
            1.091537670e-04,
        ),
    ];
    for &(vgs, id, gm, gds, gmbs) in &table {
        let op = op_at(&dev, 0.05, vgs, 0.0);
        assert_rel(op.cd, id, &format!("id(vgs={vgs})"));
        assert_rel(op.gm, gm, &format!("gm(vgs={vgs})"));
        assert_rel(op.gds, gds, &format!("gds(vgs={vgs})"));
        assert_rel(op.gmbs, gmbs, &format!("gmbs(vgs={vgs})"));
    }
    let op = op_at(&dev, 0.05, 0.9, 0.0);
    assert_rel(op.von, 5.048435630e-01, "vth");
    assert_rel(op.vdsat, 2.517733870e-01, "vdsat");

    // Body bias vbs=-0.9 at vgs=0.9 (nmos_idvg_vd50m_vbm09).
    let op = op_at(&dev, 0.05, 0.9, -0.9);
    assert_rel(op.cd, 1.292135310e-04, "id(vbs=-0.9)");
    assert_rel(op.gm, 8.092530300e-04, "gm(vbs=-0.9)");
    assert_rel(op.gds, 2.065615380e-03, "gds(vbs=-0.9)");
    assert_rel(op.gmbs, 1.616952000e-04, "gmbs(vbs=-0.9)");
    assert_rel(op.von, 7.109931570e-01, "vth(vbs=-0.9)");
}

#[test]
fn ngspice_pinned_nmos_idvg_saturation() {
    // m1 = 10u/0.18u, vds=1.2, vbs=0 (nmos_idvg_vd1200m_vb0).
    let dev = device(0.18e-6, 10e-6);
    let table: [(Value, Value, Value, Value, Value, Value, Value); 3] = [
        (
            0.6,
            1.225790620e-04,
            1.745467450e-03,
            4.231575200e-05,
            4.274622410e-04,
            4.967558170e-01,
            9.841355690e-02,
        ),
        (
            1.2,
            2.389567960e-03,
            4.543583450e-03,
            2.234402370e-04,
            1.134341020e-03,
            4.967558170e-01,
            3.733014340e-01,
        ),
        (
            1.8,
            5.118486640e-03,
            4.396534130e-03,
            3.517967690e-04,
            1.192932080e-03,
            4.967558170e-01,
            5.923702990e-01,
        ),
    ];
    for &(vgs, id, gm, gds, gmbs, vth, vdsat) in &table {
        let op = op_at(&dev, 1.2, vgs, 0.0);
        assert_rel(op.cd, id, &format!("id(vgs={vgs})"));
        assert_rel(op.gm, gm, &format!("gm(vgs={vgs})"));
        assert_rel(op.gds, gds, &format!("gds(vgs={vgs})"));
        assert_rel(op.gmbs, gmbs, &format!("gmbs(vgs={vgs})"));
        assert_rel(op.von, vth, &format!("vth(vgs={vgs})"));
        assert_rel(op.vdsat, vdsat, &format!("vdsat(vgs={vgs})"));
    }
}

#[test]
fn ngspice_pinned_nmos_geometry_variants() {
    // vds=0.05, vgs=1.2, vbs=0; long channel 10u/10u and narrow 0.6u/0.18u.
    let long = device(10e-6, 10e-6);
    let op = op_at(&long, 0.05, 1.2, 0.0);
    assert_rel(op.cd, 1.083333230e-05, "long id");
    assert_rel(op.gm, 1.247999170e-05, "long gm");
    assert_rel(op.gds, 2.073654580e-04, "long gds");
    assert_rel(op.gmbs, 4.520030240e-06, "long gmbs");

    let model = Arc::new(Bsim3v3Model::from_params(&nmos018(), false, T300));
    let narrow = Bsim3v3::new(
        "m4".to_string(),
        model,
        Bsim3v3Geometry {
            l: 0.18e-6,
            w: 0.6e-6,
            drain_area: 0.252e-12,
            source_area: 0.252e-12,
            drain_perimeter: 2.04e-6,
            source_perimeter: 2.04e-6,
            ..Bsim3v3Geometry::default()
        },
        T300,
    )
    .expect("narrow device");
    let op = op_at(&narrow, 0.05, 1.2, 0.0);
    assert_rel(op.cd, 2.837500060e-05, "narrow id");
    assert_rel(op.gm, 3.187345790e-05, "narrow gm");
    assert_rel(op.gds, 5.202146900e-04, "narrow gds");
    assert_rel(op.gmbs, 9.337682850e-06, "narrow gmbs");
}

#[test]
fn ngspice_pinned_nmos_temperature() {
    // m1 = 10u/0.18u at T=125C and T=-40C (vds=0.05, vgs=0.9).
    let model = Arc::new(Bsim3v3Model::from_params(&nmos018(), false, T300));
    let geom = Bsim3v3Geometry {
        l: 0.18e-6,
        w: 10e-6,
        drain_area: 4.2e-12,
        source_area: 4.2e-12,
        drain_perimeter: 20.84e-6,
        source_perimeter: 20.84e-6,
        ..Bsim3v3Geometry::default()
    };
    let hot = Bsim3v3::new("m1".to_string(), Arc::clone(&model), geom, 125.0 + 273.15)
        .expect("hot device");
    let op = op_at(&hot, 0.05, 0.9, 0.0);
    assert_rel(op.cd, 1.746001170e-04, "T125 id");
    assert_rel(op.gm, 4.610805720e-04, "T125 gm");
    assert_rel(op.gds, 3.146490720e-03, "T125 gds");
    assert_rel(op.gmbs, 1.098196500e-04, "T125 gmbs");
    assert_rel(op.von, 4.689281870e-01, "T125 vth");
    assert_rel(op.cbd, -9.931620370e-12, "T125 ibd");
    let op0 = op_at(&hot, 0.05, 0.0, 0.0);
    assert_rel(op0.cd, 5.621104480e-10, "T125 leakage id");

    let cold = Bsim3v3::new("m1".to_string(), model, geom, -40.0 + 273.15).expect("cold device");
    let op = op_at(&cold, 0.05, 0.9, 0.0);
    assert_rel(op.cd, 4.303247600e-04, "Tm40 id");
    assert_rel(op.gm, 1.061098670e-03, "Tm40 gm");
    assert_rel(op.gds, 7.310324640e-03, "Tm40 gds");
    assert_rel(op.gmbs, 3.011834420e-04, "Tm40 gmbs");
    assert_rel(op.von, 5.293979530e-01, "Tm40 vth");
    assert_rel(op.cbd, -5.000000000e-14, "Tm40 ibd");
}

#[test]
fn ngspice_pinned_nmos_charges_capmod3() {
    // m1 = 10u/0.18u, vds=0.05, vbs=0; CKTstate charges + intrinsic matrix
    // (nmos_idvg_vd50m_vb0 charge columns).
    let dev = device(0.18e-6, 10e-6);
    let charge_at = |vds: Value, vgs: Value| {
        dev.eval(Bsim3v3Bias { vds, vgs, vbs: 0.0 }, GMIN, true)
            .expect("charge eval")
            .charge
            .expect("charges")
    };

    let c = charge_at(0.05, 0.9);
    assert_rel(c.qg_state(), 2.352133940e-14, "qg(0.9)");
    assert_rel(c.qb_state(), -6.297545760e-15, "qb(0.9)");
    assert_rel(c.qd_state(), -8.169064120e-15, "qd(0.9)");
    assert_rel(c.cggb, 1.117357730e-14, "cgg(0.9)");
    assert_rel(c.cgdb, -4.302830190e-15, "cgd(0.9)");
    assert_rel(c.cgsb, -6.637790760e-15, "cgs(0.9)");
    assert_rel(c.cdgb, -5.468159430e-15, "cdg(0.9)");
    assert_rel(c.cddb, 4.970269500e-15, "cdd(0.9)");
    assert_rel(c.cdsb, 1.927017650e-15, "cds(0.9)");
    assert_rel(c.cbgb, -2.372584130e-16, "cbg(0.9)");
    assert_rel(c.cbdb, -5.637708810e-15, "cbd(0.9)");
    assert_rel(c.cbsb, 2.783755460e-15, "cbs(0.9)");
    assert_rel(c.capbd, 9.753509820e-15, "capbd(0.9)");
    assert_rel(c.capbs, 9.890700000e-15, "capbs(0.9)");

    let c = charge_at(0.05, 1.8);
    assert_rel(c.qg_state(), 4.818723100e-14, "qg(1.8)");
    assert_rel(c.qb_state(), -6.377323740e-15, "qb(1.8)");
    assert_rel(c.qd_state(), -2.046212100e-14, "qd(1.8)");
    assert_rel(c.cggb, 1.185931500e-14, "cgg(1.8)");
    assert_rel(c.cgdb, -5.552100720e-15, "cgd(1.8)");
    assert_rel(c.cgsb, -6.225148060e-15, "cgs(1.8)");
    assert_rel(c.cdgb, -5.910659050e-15, "cdg(1.8)");
    assert_rel(c.cddb, 6.406125830e-15, "cdd(1.8)");
    assert_rel(c.cdsb, 1.013519610e-15, "cds(1.8)");
    assert_rel(c.cbgb, -3.799686540e-17, "cbg(1.8)");
    assert_rel(c.cbdb, -7.260150940e-15, "cbd(1.8)");
    assert_rel(c.cbsb, 4.198108840e-15, "cbs(1.8)");

    // Id-Vd at vgs=1.2: charge state across linear-to-saturation
    // (nmos_idvd_vg1200m_vb0).
    let table: [(Value, Value, Value, Value, Value); 4] = [
        (
            0.1,
            8.767909750e-04,
            7.292463720e-03,
            3.104280690e-14,
            1.145796740e-14,
        ),
        (
            0.5,
            2.166427230e-03,
            6.902089060e-04,
            2.728467320e-14,
            9.985041030e-15,
        ),
        (
            1.0,
            2.343182810e-03,
            2.428145710e-04,
            2.335256890e-14,
            9.936896910e-15,
        ),
        (
            1.8,
            2.516572950e-03,
            2.047779910e-04,
            1.707814390e-14,
            9.935598650e-15,
        ),
    ];
    for &(vds, id, gds, qg, cgg) in &table {
        let op = dev
            .eval(
                Bsim3v3Bias {
                    vds,
                    vgs: 1.2,
                    vbs: 0.0,
                },
                GMIN,
                true,
            )
            .expect("eval");
        assert_rel(op.cd, id, &format!("id(vds={vds})"));
        assert_rel(op.gds, gds, &format!("gds(vds={vds})"));
        let c = op.charge.expect("charges");
        assert_rel(c.qg_state(), qg, &format!("qg(vds={vds})"));
        assert_rel(c.cggb, cgg, &format!("cgg(vds={vds})"));
    }
}

#[test]
fn ngspice_pinned_pmos() {
    // p018 m1 = 10u/0.18u; node-space biases (PMOS), internal polarity folded
    // by eval_polarity (pmos_idvg_vdm50m_vb0 and friends).
    let model = Arc::new(Bsim3v3Model::from_params(&pmos018(), true, T300));
    let dev = Bsim3v3::new(
        "m1".to_string(),
        model,
        Bsim3v3Geometry {
            l: 0.18e-6,
            w: 10e-6,
            drain_area: 4.2e-12,
            source_area: 4.2e-12,
            drain_perimeter: 20.84e-6,
            source_perimeter: 20.84e-6,
            ..Bsim3v3Geometry::default()
        },
        T300,
    )
    .expect("pmos device");
    let table: [(Value, Value, Value, Value, Value, Value, Value); 3] = [
        (
            -0.6,
            2.190448330e-05,
            2.130193240e-04,
            3.420662180e-04,
            6.462213390e-05,
            4.802688480e-01,
            1.345257640e-14,
        ),
        (
            -1.2,
            1.417817060e-04,
            1.571937100e-04,
            2.720356250e-03,
            6.626002240e-05,
            4.802688480e-01,
            2.683623020e-14,
        ),
        (
            -1.8,
            2.162855600e-04,
            9.544283270e-05,
            4.229858380e-03,
            5.423827330e-05,
            4.802688480e-01,
            4.063189500e-14,
        ),
    ];
    for &(vgs_node, id, gm, gds, gmbs, vth, qg) in &table {
        let op = dev
            .eval_polarity(-0.05, vgs_node, 0.0, GMIN, true)
            .expect("eval");
        assert_rel(op.cd, id, &format!("pmos id(vgs={vgs_node})"));
        assert_rel(op.gm, gm, &format!("pmos gm(vgs={vgs_node})"));
        assert_rel(op.gds, gds, &format!("pmos gds(vgs={vgs_node})"));
        assert_rel(op.gmbs, gmbs, &format!("pmos gmbs(vgs={vgs_node})"));
        assert_rel(op.von, vth, &format!("pmos vth(vgs={vgs_node})"));
        let c = op.charge.expect("charges");
        assert_rel(c.qg_state(), qg, &format!("pmos qg(vgs={vgs_node})"));
    }
    // Saturation Id-Vd point and forward body bias.
    let op = dev
        .eval_polarity(-1.0, -1.2, 0.0, GMIN, false)
        .expect("eval");
    assert_rel(op.cd, 1.088345900e-03, "pmos idvd id");
    assert_rel(op.gds, 2.104459930e-04, "pmos idvd gds");
    assert_rel(op.vdsat, 5.909648480e-01, "pmos idvd vdsat");
    let op = dev
        .eval_polarity(-0.05, -1.2, 0.9, GMIN, false)
        .expect("eval");
    assert_rel(op.cd, 9.084630350e-05, "pmos id(vbs=+0.9)");
    assert_rel(op.von, 7.132288490e-01, "pmos vth(vbs=+0.9)");
}

// ===================== live ngspice oracle =====================
//
// Set NGSPICE_EXE to a local ngspice-46 binary and run
// `cargo test -p rspice-core --lib bsim3v3 -- --ignored --nocapture`.
// Each deck in testdata/ is executed with `ngspice -b`; every wrdata table is
// then replayed through this port at the same bias and compared column by
// column at 1e-6 relative (the math is a transcription, so anything looser
// is a porting bug; the reference itself is printed with 9 significant
// digits).

mod oracle {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    pub const REL_TOL: Value = 1e-6;

    pub struct Sweep {
        pub file: &'static str,
        /// true: vg swept; false: vd swept.
        pub sweep_is_vg: bool,
        pub vd: Value,
        pub vg: Value,
        pub vb: Value,
    }

    pub struct DeckSpec {
        pub deck: &'static str,
        pub temp_c: Value,
        pub is_pmos: bool,
        /// (name, w, l, ad, as, pd, ps)
        pub devices: &'static [(&'static str, Value, Value, Value, Value, Value, Value)],
        pub sweeps: &'static [Sweep],
    }

    pub fn deck_specs() -> Vec<DeckSpec> {
        vec![
            DeckSpec {
                deck: "nmos_oracle.sp",
                temp_c: 27.0,
                is_pmos: false,
                devices: &[
                    ("m1", 10e-6, 0.18e-6, 4.2e-12, 4.2e-12, 20.84e-6, 20.84e-6),
                    ("m2", 10e-6, 10e-6, 4.2e-12, 4.2e-12, 20.84e-6, 20.84e-6),
                    ("m3", 10e-6, 0.5e-6, 4.2e-12, 4.2e-12, 20.84e-6, 20.84e-6),
                    (
                        "m4", 0.6e-6, 0.18e-6, 0.252e-12, 0.252e-12, 2.04e-6, 2.04e-6,
                    ),
                ],
                sweeps: &[
                    Sweep {
                        file: "nmos_idvg_vd50m_vb0",
                        sweep_is_vg: true,
                        vd: 0.05,
                        vg: 0.0,
                        vb: 0.0,
                    },
                    Sweep {
                        file: "nmos_idvg_vd50m_vbm09",
                        sweep_is_vg: true,
                        vd: 0.05,
                        vg: 0.0,
                        vb: -0.9,
                    },
                    Sweep {
                        file: "nmos_idvg_vd1200m_vb0",
                        sweep_is_vg: true,
                        vd: 1.2,
                        vg: 0.0,
                        vb: 0.0,
                    },
                    Sweep {
                        file: "nmos_idvd_vg1200m_vb0",
                        sweep_is_vg: false,
                        vd: 0.0,
                        vg: 1.2,
                        vb: 0.0,
                    },
                    Sweep {
                        file: "nmos_idvd_vg600m_vb0",
                        sweep_is_vg: false,
                        vd: 0.0,
                        vg: 0.6,
                        vb: 0.0,
                    },
                    Sweep {
                        file: "nmos_idvd_vg900m_vbm045",
                        sweep_is_vg: false,
                        vd: 0.0,
                        vg: 0.9,
                        vb: -0.45,
                    },
                ],
            },
            DeckSpec {
                deck: "nmos_oracle_temp.sp",
                temp_c: 125.0,
                is_pmos: false,
                devices: &[
                    ("m1", 10e-6, 0.18e-6, 4.2e-12, 4.2e-12, 20.84e-6, 20.84e-6),
                    ("m2", 10e-6, 0.5e-6, 4.2e-12, 4.2e-12, 20.84e-6, 20.84e-6),
                ],
                sweeps: &[
                    Sweep {
                        file: "nmos_idvg_vd50m_t125",
                        sweep_is_vg: true,
                        vd: 0.05,
                        vg: 0.0,
                        vb: 0.0,
                    },
                    Sweep {
                        file: "nmos_idvg_vd1200m_t125",
                        sweep_is_vg: true,
                        vd: 1.2,
                        vg: 0.0,
                        vb: 0.0,
                    },
                ],
            },
            DeckSpec {
                deck: "nmos_oracle_tm40.sp",
                temp_c: -40.0,
                is_pmos: false,
                devices: &[
                    ("m1", 10e-6, 0.18e-6, 4.2e-12, 4.2e-12, 20.84e-6, 20.84e-6),
                    ("m2", 10e-6, 0.5e-6, 4.2e-12, 4.2e-12, 20.84e-6, 20.84e-6),
                ],
                sweeps: &[
                    Sweep {
                        file: "nmos_idvg_vd50m_tm40",
                        sweep_is_vg: true,
                        vd: 0.05,
                        vg: 0.0,
                        vb: 0.0,
                    },
                    Sweep {
                        file: "nmos_idvg_vd1200m_tm40",
                        sweep_is_vg: true,
                        vd: 1.2,
                        vg: 0.0,
                        vb: 0.0,
                    },
                ],
            },
            DeckSpec {
                deck: "pmos_oracle.sp",
                temp_c: 27.0,
                is_pmos: true,
                devices: &[
                    ("m1", 10e-6, 0.18e-6, 4.2e-12, 4.2e-12, 20.84e-6, 20.84e-6),
                    ("m2", 10e-6, 0.5e-6, 4.2e-12, 4.2e-12, 20.84e-6, 20.84e-6),
                ],
                sweeps: &[
                    Sweep {
                        file: "pmos_idvg_vdm50m_vb0",
                        sweep_is_vg: true,
                        vd: -0.05,
                        vg: 0.0,
                        vb: 0.0,
                    },
                    Sweep {
                        file: "pmos_idvg_vdm50m_vb09",
                        sweep_is_vg: true,
                        vd: -0.05,
                        vg: 0.0,
                        vb: 0.9,
                    },
                    Sweep {
                        file: "pmos_idvg_vdm1200m_vb0",
                        sweep_is_vg: true,
                        vd: -1.2,
                        vg: 0.0,
                        vb: 0.0,
                    },
                    Sweep {
                        file: "pmos_idvd_vgm1200m_vb0",
                        sweep_is_vg: false,
                        vd: 0.0,
                        vg: -1.2,
                        vb: 0.0,
                    },
                ],
            },
        ]
    }

    pub fn testdata_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/device/mosfet/bsim3v3/testdata")
    }

    pub fn run_deck(exe: &Path, deck: &str) -> PathBuf {
        let work = std::env::temp_dir().join(format!(
            "rspice-b3v3-oracle-{}-{}",
            std::process::id(),
            deck.trim_end_matches(".sp")
        ));
        let _ = fs::remove_dir_all(&work);
        fs::create_dir_all(&work).expect("create oracle work dir");
        for f in ["models018.lib", deck] {
            fs::copy(testdata_dir().join(f), work.join(f)).expect("copy deck");
        }
        let out = Command::new(exe)
            .arg("-b")
            .arg(deck)
            .current_dir(&work)
            .output()
            .expect("run ngspice");
        assert!(
            out.status.success(),
            "ngspice failed on {deck}:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
        work
    }

    /// Parse a `wrdata` table (wr_vecnames + wr_singlescale).
    pub fn parse_wrdata(path: &Path) -> (Vec<String>, Vec<Vec<Value>>) {
        let text =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let mut lines = text.lines();
        let header: Vec<String> = lines
            .next()
            .expect("wrdata header")
            .split_whitespace()
            .map(str::to_string)
            .collect();
        let rows = lines
            .filter(|l| !l.trim().is_empty())
            .map(|l| {
                l.split_whitespace()
                    .map(|t| t.parse::<Value>().expect("wrdata number"))
                    .collect::<Vec<_>>()
            })
            .collect();
        (header, rows)
    }

    /// Map an oracle column quantity onto the port's op/charge state.
    pub fn extract(op: &Bsim3v3Op, quantity: &str) -> Value {
        let c = || op.charge.as_ref().expect("charge state");
        match quantity {
            "id" => op.cd,
            "gm" => op.gm,
            "gds" => op.gds,
            "gmbs" => op.gmbs,
            "vth" => op.von,
            "vdsat" => op.vdsat,
            "ibs" => op.cbs,
            "ibd" => op.cbd,
            "qg" => c().qg_state(),
            "qb" => c().qb_state(),
            "qd" => c().qd_state(),
            "cgg" => c().cggb,
            "cgd" => c().cgdb,
            "cgs" => c().cgsb,
            "cdg" => c().cdgb,
            "cdd" => c().cddb,
            "cds" => c().cdsb,
            "cbg" => c().cbgb,
            "cbd" => c().cbdb,
            "cbs" => c().cbsb,
            "capbd" => c().capbd,
            "capbs" => c().capbs,
            other => panic!("unmapped oracle quantity {other}"),
        }
    }
}

/// Live oracle: requires a local ngspice-46 build.
#[test]
#[ignore = "requires NGSPICE_EXE pointing at a local ngspice-46 binary"]
fn live_ngspice_oracle_dc_and_charge() {
    let Some(exe) = std::env::var_os("NGSPICE_EXE").map(std::path::PathBuf::from) else {
        eprintln!("Skipping: set NGSPICE_EXE to run the live BSIM3 oracle.");
        return;
    };

    let mut points = 0usize;
    let mut worst: (Value, String) = (0.0, String::new());
    for spec in oracle::deck_specs() {
        let work = oracle::run_deck(&exe, spec.deck);
        let temp_k = spec.temp_c + 273.15;

        // Build one device per geometry of this deck.
        let card = if spec.is_pmos { pmos018() } else { nmos018() };
        let model = Arc::new(Bsim3v3Model::from_params(&card, spec.is_pmos, T300));
        let devices: HashMap<String, Bsim3v3> = spec
            .devices
            .iter()
            .map(|&(name, w, l, ad, asr, pd, ps)| {
                let geom = Bsim3v3Geometry {
                    l,
                    w,
                    drain_area: ad,
                    source_area: asr,
                    drain_perimeter: pd,
                    source_perimeter: ps,
                    ..Bsim3v3Geometry::default()
                };
                (
                    name.to_string(),
                    Bsim3v3::new(name.to_string(), Arc::clone(&model), geom, temp_k)
                        .expect("oracle device"),
                )
            })
            .collect();

        for sweep in spec.sweeps {
            let (header, rows) = oracle::parse_wrdata(&work.join(sweep.file));
            assert!(rows.len() > 10, "{}: too few rows", sweep.file);
            for row in &rows {
                let x = row[0];
                let (vd, vg, vb) = if sweep.sweep_is_vg {
                    (sweep.vd, x, sweep.vb)
                } else {
                    (x, sweep.vg, sweep.vb)
                };
                // One charge-bearing eval per device per row.
                let mut ops: HashMap<&str, Bsim3v3Op> = HashMap::new();
                for (col, name) in header.iter().enumerate().skip(1) {
                    let (dev_name, quantity) = name
                        .trim_start_matches('@')
                        .split_once('[')
                        .map(|(d, q)| (d, q.trim_end_matches(']')))
                        .expect("oracle column name");
                    let dev = &devices[dev_name];
                    let op = ops.entry(dev_name).or_insert_with(|| {
                        dev.eval_polarity(vd, vg, vb, 1e-12, true).expect("eval")
                    });
                    let ours = oracle::extract(op, quantity);
                    let reference = row[col];
                    let tol = oracle::REL_TOL * reference.abs().max(ours.abs()) + 1e-22;
                    let err = (ours - reference).abs();
                    if reference.abs() > 0.0 {
                        let rel = err / reference.abs().max(1e-30);
                        if rel > worst.0 && err > 1e-22 {
                            worst = (
                                rel,
                                format!(
                                    "{}:{} {} at (vd={vd},vg={vg},vb={vb}): ours={ours:.9e} ref={reference:.9e}",
                                    sweep.file, dev_name, quantity
                                ),
                            );
                        }
                    }
                    assert!(
                        err <= tol,
                        "{} {}[{}] at (vd={vd}, vg={vg}, vb={vb}): ours={ours:.9e} ngspice={reference:.9e} err={err:.3e}",
                        sweep.file,
                        dev_name,
                        quantity,
                    );
                    points += 1;
                }
            }
        }
        let _ = std::fs::remove_dir_all(&work);
    }
    println!(
        "live BSIM3 oracle: {points} comparisons passed; worst rel err {:.3e} ({})",
        worst.0, worst.1
    );
}

/// PMOS card matching testdata/models018.lib `p018`.
fn pmos018() -> HashMap<String, Value> {
    let pairs: &[(&str, Value)] = &[
        ("TNOM", 27.0),
        ("TOX", 4.1e-9),
        ("XJ", 1.0e-7),
        ("NCH", 4.1e17),
        ("VTH0", -0.39),
        ("K1", 0.55),
        ("K2", 0.032),
        ("K3", 0.0),
        ("K3B", 7.5),
        ("W0", 1e-6),
        ("NLX", 1.1e-7),
        ("DVT0W", 0.0),
        ("DVT1W", 0.0),
        ("DVT2W", 0.0),
        ("DVT0", 0.58),
        ("DVT1", 0.24),
        ("DVT2", 0.07),
        ("U0", 110.0),
        ("UA", 1.3e-9),
        ("UB", 1.0e-21),
        ("UC", -1.0e-10),
        ("VSAT", 1.9e5),
        ("A0", 1.6),
        ("AGS", 0.27),
        ("B0", 2.5e-7),
        ("B1", 8e-7),
        ("KETA", 0.025),
        ("A1", 0.4),
        ("A2", 0.4),
        ("RDSW", 250.0),
        ("PRWG", 0.5),
        ("PRWB", 0.05),
        ("WR", 1.0),
        ("WINT", 0.0),
        ("LINT", 2.5e-8),
        ("DWG", -2.5e-8),
        ("DWB", 1e-8),
        ("VOFF", -0.1),
        ("NFACTOR", 1.9),
        ("CIT", 0.0),
        ("CDSC", 2.4e-4),
        ("CDSCD", 0.0),
        ("CDSCB", 0.0),
        ("ETA0", 0.02),
        ("ETAB", -0.012),
        ("DSUB", 0.06),
        ("PCLM", 1.2),
        ("PDIBLC1", 8e-4),
        ("PDIBLC2", 0.022),
        ("PDIBLCB", -1e-3),
        ("DROUT", 0.08),
        ("PSCBE1", 2.7e9),
        ("PSCBE2", 9.5e-10),
        ("PVAG", 15.0),
        ("DELTA", 0.01),
        ("RSH", 0.0),
        ("MOBMOD", 1.0),
        ("PRT", 0.0),
        ("UTE", -1.1),
        ("KT1", -0.11),
        ("KT1L", 0.0),
        ("KT2", 0.022),
        ("UA1", 4.31e-9),
        ("UB1", -7.61e-18),
        ("UC1", -5.6e-11),
        ("AT", 3.3e4),
        ("WL", 0.0),
        ("WLN", 1.0),
        ("WW", 0.0),
        ("WWN", 1.0),
        ("WWL", 0.0),
        ("LL", 0.0),
        ("LLN", 1.0),
        ("LW", 0.0),
        ("LWN", 1.0),
        ("LWL", 0.0),
        ("CAPMOD", 3.0),
        ("XPART", 0.5),
        ("CGDO", 6.4e-10),
        ("CGSO", 6.4e-10),
        ("CGBO", 1e-12),
        ("CJ", 1.15e-3),
        ("PB", 0.85),
        ("MJ", 0.41),
        ("CJSW", 1.9e-10),
        ("PBSW", 0.85),
        ("MJSW", 0.34),
        ("CJSWG", 2.8e-10),
        ("PBSWG", 0.85),
        ("MJSWG", 0.34),
        ("CF", 0.0),
        ("CLC", 1e-7),
        ("CLE", 0.6),
        ("VOFFCV", -0.05),
        ("NOFF", 2.5),
        ("ACDE", 0.85),
        ("MOIN", 15.0),
        ("JS", 1.1e-6),
        ("JSW", 2.2e-12),
        ("NJ", 1.0),
        ("XTI", 3.0),
    ];
    pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
}

#[test]
fn pmos_polarity_mirrors_nmos_conventions() {
    let model = Arc::new(Bsim3v3Model::from_params(&nmos018(), true, T300));
    let dev = Bsim3v3::new(
        "mp".to_string(),
        model,
        Bsim3v3Geometry {
            l: 0.5e-6,
            w: 10e-6,
            ..Bsim3v3Geometry::default()
        },
        T300,
    )
    .expect("pmos device");
    // Node-space PMOS bias: vgs=-1.2, vds=-0.6 -> folded internal +1.2/+0.6.
    let op = dev
        .eval_polarity(-0.6, -1.2, 0.0, GMIN, false)
        .expect("eval");
    assert_eq!(op.mode, 1);
    assert!(
        op.cd > 0.0,
        "internal-frame current positive, got {}",
        op.cd
    );
}
