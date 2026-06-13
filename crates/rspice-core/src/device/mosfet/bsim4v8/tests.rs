//! Unit tests for the BSIM4 v4.8 (level 14/54) model module.
//!
//! Two layers:
//! - analytic/self-consistency tests that run everywhere: temperature-setup
//!   physicality, derivative-vs-finite-difference agreement, smoothing
//!   continuity, source/drain swap symmetry, subthreshold slope, and a
//!   no-NaN sweep grid;
//! - ngspice-pinned tests: bias tables produced by a local ngspice-46 build
//!   from the decks in `testdata/` (see the `#[ignore]`d live-oracle test
//!   which regenerates them when `NGSPICE_EXE` is set).

use super::*;
use crate::Value;
use std::collections::HashMap;

const T300: Value = 300.15;
const GMIN: Value = 1e-12;

/// 45nm-style NMOS card (own values, written for this test suite; not a
/// foundry deck). Mirrors testdata/models45.lib `n45`.
fn nmos45() -> HashMap<String, Value> {
    let pairs: &[(&str, Value)] = &[
        ("TNOM", 27.0),
        ("TOXE", 1.4e-9),
        ("TOXP", 1.0e-9),
        ("TOXM", 1.4e-9),
        ("TOXREF", 1.4e-9),
        ("EPSROX", 3.9),
        ("XJ", 1.4e-8),
        ("NDEP", 3.0e18),
        ("NSD", 2.0e20),
        ("NGATE", 1.0e23),
        ("VTH0", 0.46),
        ("K1", 0.49),
        ("K2", 0.01),
        ("K3", 0.0),
        ("K3B", 0.0),
        ("W0", 2.5e-6),
        ("LPE0", 2.0e-9),
        ("LPEB", 1.0e-9),
        ("DVT0", 1.1),
        ("DVT1", 0.55),
        ("DVT2", -0.03),
        ("DVT0W", 0.0),
        ("DVT1W", 5.3e6),
        ("DVT2W", -0.032),
        ("DVTP0", 1.0e-8),
        ("DVTP1", 0.12),
        ("DSUB", 0.095),
        ("ETA0", 0.0042),
        ("ETAB", -0.05),
        ("MINV", 0.05),
        ("VOFF", -0.11),
        ("VOFFL", 1.0e-9),
        ("NFACTOR", 1.7),
        ("CDSC", 4.0e-4),
        ("CDSCB", 1.0e-5),
        ("CDSCD", 1.0e-5),
        ("CIT", 1.0e-5),
        ("MOBMOD", 0.0),
        ("U0", 0.045),
        ("UA", 5.0e-10),
        ("UB", 1.3e-18),
        ("UC", 8.0e-11),
        ("UD", 1.0e15),
        ("EU", 1.67),
        ("VSAT", 1.2e5),
        ("A0", 1.1),
        ("AGS", 0.5),
        ("A1", 0.0),
        ("A2", 1.0),
        ("B0", 0.0),
        ("B1", 0.0),
        ("KETA", 0.035),
        ("KETAC", 0.04),
        ("DWG", -3.0e-9),
        ("DWB", 4.0e-9),
        ("PCLM", 0.06),
        ("PDIBLC1", 0.012),
        ("PDIBLC2", 4.0e-3),
        ("PDIBLCB", -0.05),
        ("DROUT", 0.45),
        ("PVAG", 0.2),
        ("DELTA", 0.012),
        ("PSCBE1", 8.0e8),
        ("PSCBE2", 1.0e-7),
        ("FPROUT", 0.2),
        ("PDITS", 0.15),
        ("PDITSD", 0.22),
        ("PDITSL", 2.0e6),
        ("RDSW", 150.0),
        ("RDSWMIN", 10.0),
        ("PRWG", 0.3),
        ("PRWB", -0.1),
        ("WR", 1.0),
        ("WINT", 2.0e-9),
        ("LINT", 1.2e-9),
        ("XL", -2.0e-8),
        ("ALPHA0", 6.0e-9),
        ("ALPHA1", 0.8),
        ("BETA0", 12.0),
        ("AGIDL", 4.0e-10),
        ("BGIDL", 2.1e9),
        ("CGIDL", 0.4),
        ("EGIDL", 0.9),
        ("AGISL", 3.0e-10),
        ("BGISL", 2.2e9),
        ("CGISL", 0.5),
        ("EGISL", 1.0),
        ("UTE", -1.4),
        ("UA1", 2.5e-9),
        ("UB1", -2.0e-18),
        ("UC1", -1.0e-10),
        ("UD1", 0.0),
        ("AT", 2.6e4),
        ("PRT", 10.0),
        ("KT1", -0.26),
        ("KT1L", 1.0e-9),
        ("KT2", 0.022),
        ("TVOFF", 8.0e-4),
        ("TNFACTOR", 0.1),
        ("TETA0", 1.0e-4),
        ("XPART", 0.0),
        ("CGSO", 1.0e-10),
        ("CGDO", 1.0e-10),
        ("CGBO", 1.0e-12),
        ("CGSL", 1.5e-10),
        ("CGDL", 1.5e-10),
        ("CKAPPAS", 0.55),
        ("CKAPPAD", 0.55),
        ("ACDE", 0.9),
        ("MOIN", 12.0),
        ("NOFF", 1.2),
        ("VOFFCV", 0.05),
        ("CLC", 6.0e-8),
        ("CLE", 0.58),
        ("CF", 1.1e-10),
        ("RSH", 5.0),
        ("JSS", 1.2e-7),
        ("JSWS", 2.0e-13),
        ("JSWGS", 1.0e-13),
        ("NJS", 1.05),
        ("XTIS", 3.0),
        ("IJTHSFWD", 0.02),
        ("IJTHSREV", 0.02),
        ("CJS", 8.0e-4),
        ("MJS", 0.42),
        ("PBS", 0.85),
        ("CJSWS", 6.0e-11),
        ("MJSWS", 0.3),
        ("PBSWS", 0.75),
        ("CJSWGS", 2.5e-10),
        ("MJSWGS", 0.35),
        ("PBSWGS", 0.8),
        ("TPB", 1.2e-3),
        ("TCJ", 8.0e-4),
        ("TPBSW", 1.0e-3),
        ("TCJSW", 6.0e-4),
        ("TPBSWG", 9.0e-4),
        ("TCJSWG", 5.0e-4),
        ("JTSS", 1.0e-7),
        ("JTSD", 1.0e-7),
        ("JTSSWS", 1.0e-13),
        ("JTSSWGS", 1.0e-13),
        ("NJTS", 18.0),
        ("NJTSSW", 22.0),
        ("NJTSSWG", 20.0),
        ("VTSS", 8.0),
        ("VTSD", 8.0),
        ("VTSSWS", 8.0),
        ("VTSSWGS", 8.0),
        ("XTSS", 0.022),
        ("XTSD", 0.022),
        ("XTSSWS", 0.022),
        ("XTSSWGS", 0.022),
        ("TNJTS", 0.5),
        ("TNJTSSW", 0.6),
        ("TNJTSSWG", 0.55),
        ("DMCG", 4.0e-8),
        ("DMCI", 5.0e-8),
    ];
    pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
}

/// 90nm-style PMOS card (own values; mirrors testdata/models45.lib `p90`).
fn pmos90() -> HashMap<String, Value> {
    let pairs: &[(&str, Value)] = &[
        ("TNOM", 27.0),
        ("TOXE", 2.1e-9),
        ("TOXP", 1.6e-9),
        ("TOXM", 2.1e-9),
        ("TOXREF", 2.1e-9),
        ("XJ", 2.0e-8),
        ("NDEP", 1.8e18),
        ("NSD", 1.5e20),
        ("NGATE", 8.0e22),
        ("VTH0", -0.35),
        ("K1", 0.45),
        ("K2", -0.02),
        ("K3", 1.0),
        ("K3B", 0.5),
        ("W0", 1.0e-6),
        ("LPE0", 5.0e-9),
        ("DVT0", 1.8),
        ("DVT1", 0.52),
        ("DVT2", -0.028),
        ("DVTP0", 6.0e-9),
        ("DVTP1", 0.1),
        ("DSUB", 0.45),
        ("ETA0", 0.15),
        ("ETAB", -0.12),
        ("MINV", 0.0),
        ("VOFF", -0.13),
        ("NFACTOR", 2.1),
        ("CDSC", 2.4e-4),
        ("MOBMOD", 1.0),
        ("U0", 0.009),
        ("UA", 9.0e-10),
        ("UB", 1.0e-18),
        ("UC", -5.0e-11),
        ("VSAT", 9.0e4),
        ("A0", 1.3),
        ("AGS", 0.3),
        ("A1", 0.0),
        ("A2", 1.0),
        ("KETA", -0.02),
        ("PCLM", 0.12),
        ("PDIBLC1", 0.02),
        ("PDIBLC2", 6.0e-3),
        ("DROUT", 0.5),
        ("PVAG", 1.0),
        ("DELTA", 0.01),
        ("PSCBE1", 4.24e8),
        ("PSCBE2", 1.0e-5),
        ("PDITS", 0.1),
        ("PDITSD", 0.2),
        ("RDSW", 320.0),
        ("PRWG", 0.4),
        ("PRWB", 0.1),
        ("WINT", 3.0e-9),
        ("LINT", 1.5e-9),
        ("AGIDL", 2.0e-10),
        ("BGIDL", 2.5e9),
        ("CGIDL", 0.6),
        ("EGIDL", 0.8),
        ("UTE", -1.2),
        ("UA1", 4.0e-9),
        ("UB1", -1.5e-18),
        ("UC1", -1.0e-10),
        ("AT", 3.0e4),
        ("KT1", -0.22),
        ("KT2", 0.025),
        ("XPART", 0.0),
        ("CGSO", 1.6e-10),
        ("CGDO", 1.6e-10),
        ("CGBO", 1.0e-12),
        ("CGSL", 1.0e-10),
        ("CGDL", 1.0e-10),
        ("CKAPPAS", 0.6),
        ("ACDE", 1.0),
        ("MOIN", 15.0),
        ("NOFF", 1.5),
        ("VOFFCV", -0.02),
        ("CLC", 1.0e-7),
        ("CLE", 0.6),
        ("CF", 1.2e-10),
        ("RSH", 6.0),
        ("JSS", 2.0e-7),
        ("JSWS", 3.0e-13),
        ("NJS", 1.1),
        ("XTIS", 3.0),
        ("IJTHSFWD", 0.03),
        ("CJS", 1.1e-3),
        ("MJS", 0.45),
        ("PBS", 0.9),
        ("CJSWS", 8.0e-11),
        ("MJSWS", 0.32),
        ("PBSWS", 0.8),
        ("CJSWGS", 3.0e-10),
        ("MJSWGS", 0.38),
        ("PBSWGS", 0.85),
        ("TPB", 1.0e-3),
        ("TCJ", 7.0e-4),
    ];
    pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
}

fn geom(w: Value, l: Value, nf: Value) -> Bsim4v8Geometry {
    // The oracle decks give per-device AD/AS/PD/PS for nf=1 only; nf>1
    // devices use the geoMod=0 BSIM4PAeffGeo diffusions like the deck.
    if nf == 1.0 {
        Bsim4v8Geometry {
            l,
            w,
            nf,
            drain_area: w * 0.1e-6,
            drain_area_given: true,
            source_area: w * 0.1e-6,
            source_area_given: true,
            drain_perimeter: 2.0 * (w + 0.1e-6),
            drain_perimeter_given: true,
            source_perimeter: 2.0 * (w + 0.1e-6),
            source_perimeter_given: true,
            ..Bsim4v8Geometry::default()
        }
    } else {
        Bsim4v8Geometry {
            l,
            w,
            nf,
            ..Bsim4v8Geometry::default()
        }
    }
}

fn nmos_device(w: Value, l: Value, nf: Value, temp_k: Value) -> Bsim4v8 {
    let model = Arc::new(Bsim4v8Model::from_params(&nmos45(), false, T300));
    Bsim4v8::new("m1".to_string(), model, geom(w, l, nf), temp_k).expect("nmos device")
}

fn pmos_device(w: Value, l: Value, temp_k: Value) -> Bsim4v8 {
    let model = Arc::new(Bsim4v8Model::from_params(&pmos90(), true, T300));
    Bsim4v8::new("m1".to_string(), model, geom(w, l, 1.0), temp_k).expect("pmos device")
}

fn op_at(dev: &Bsim4v8, vds: Value, vgs: Value, vbs: Value) -> Bsim4v8Op {
    dev.eval(Bsim4v8Bias { vds, vgs, vbs }, GMIN, false)
        .expect("dc eval")
}

#[test]
fn temp_setup_is_finite_and_physical() {
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);
    let p = &dev.size;
    assert!(p.phi > 0.8 && p.phi < 1.2, "phi={}", p.phi);
    assert!(p.vbi > p.phi, "vbi={} phi={}", p.vbi, p.phi);
    assert!(p.u0temp > 0.0 && p.u0temp < 0.1, "u0temp={}", p.u0temp);
    assert!(p.vsattemp > 1e4, "vsattemp={}", p.vsattemp);
    assert!(p.rds0 > 0.0, "rds0={}", p.rds0);
    assert!(p.k1ox > 0.0 && p.vth0 > 0.0);
    assert!(dev.model_temp.vtm > 0.025 && dev.model_temp.vtm < 0.027);
    assert!(dev.model_temp.vcrit > 0.5);
    assert!(dev.inst.vfbzb.is_finite() && dev.inst.vfbzb < 0.0, "vfbzb={}", dev.inst.vfbzb);
    assert!(dev.inst.vtfbphi1 > 0.0 && dev.inst.vtfbphi2 > 0.0);
    assert!(dev.inst.source_sat_current > 0.0);
    assert!(dev.inst.vjsm_fwd.is_some());
}

#[test]
fn size_cache_reuses_knots() {
    let model = Arc::new(Bsim4v8Model::from_params(&nmos45(), false, T300));
    let mt = Bsim4v8ModelTemp::new(&model, T300);
    let mut cache = SizeDepCache::new();
    let a = cache.get(&model, &mt, 45e-9, 1e-6, 1.0).expect("knot");
    let b = cache.get(&model, &mt, 45e-9, 1e-6, 1.0).expect("knot");
    let c = cache.get(&model, &mt, 45e-9, 1e-6, 4.0).expect("knot");
    assert!(Arc::ptr_eq(&a, &b));
    assert!(!Arc::ptr_eq(&a, &c));
}

#[test]
fn unported_mode_knobs_are_rejected() {
    let reject = |key: &str, val: Value, what: &str| {
        let mut card = nmos45();
        card.insert(key.to_string(), val);
        let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
        let err = Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
            .expect_err(&format!("{what} should be rejected"));
        assert!(err.contains(what), "{what}: unexpected error text: {err}");
    };
    reject("RDSMOD", 1.0, "RDSMOD");
    reject("RGATEMOD", 2.0, "RGATEMOD");
    reject("RBODYMOD", 1.0, "RBODYMOD");
    reject("TRNQSMOD", 1.0, "TRNQSMOD");
    reject("MOBMOD", 3.0, "MOBMOD");
    reject("DIOMOD", 2.0, "DIOMOD");
    reject("IGCMOD", 1.0, "IGCMOD");
    reject("IGBMOD", 1.0, "IGBMOD");
    reject("CAPMOD", 0.0, "CAPMOD");
    reject("MTRLMOD", 1.0, "MTRLMOD");
    reject("WPEMOD", 1.0, "WPEMOD");

    // Stress model: SA/SB on the instance.
    let model = Arc::new(Bsim4v8Model::from_params(&nmos45(), false, T300));
    let g = Bsim4v8Geometry {
        sa: 1.0e-6,
        sb: 1.0e-6,
        ..geom(1e-6, 45e-9, 1.0)
    };
    let err = Bsim4v8::new("m1".to_string(), model, g, T300).expect_err("stress");
    assert!(err.contains("stress"), "unexpected error text: {err}");

    // cvchargeMod=1: DC fine, charges a typed error.
    let mut card = nmos45();
    card.insert("CVCHARGEMOD".to_string(), 1.0);
    let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
    let dev = Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
        .expect("cvchargeMod=1 constructs");
    let bias = Bsim4v8Bias { vds: 0.5, vgs: 0.8, vbs: 0.0 };
    assert!(dev.eval(bias, GMIN, false).is_ok());
    let err = dev.eval(bias, GMIN, true).unwrap_err();
    assert!(err.contains("CVCHARGEMOD"), "unexpected error text: {err}");
}

#[test]
fn gm_gds_gmbs_match_finite_differences() {
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);
    let h = 1e-6;
    let biases: [(Value, Value, Value); 8] = [
        (0.05, 0.6, 0.0),
        (1.0, 0.6, 0.0),
        (1.1, 1.1, 0.0),
        (0.5, 0.45, -0.4),
        (0.8, 0.2, 0.0),   // subthreshold
        (0.02, 1.1, -0.8), // deep linear, body bias
        (1.0, 0.46, 0.2),  // near Vth, forward body
        (0.3, 0.9, -1.0),
    ];
    for &(vds, vgs, vbs) in &biases {
        let op = op_at(&dev, vds, vgs, vbs);
        let id = |vds: Value, vgs: Value, vbs: Value| op_at(&dev, vds, vgs, vbs).cd;
        let fd_gm = (id(vds, vgs + h, vbs) - id(vds, vgs - h, vbs)) / (2.0 * h);
        let fd_gds = (id(vds + h, vgs, vbs) - id(vds - h, vgs, vbs)) / (2.0 * h);
        let fd_gmb = (id(vds, vgs, vbs + h) - id(vds, vgs, vbs - h)) / (2.0 * h);
        let ok = |a: Value, b: Value| (a - b).abs() <= 1e-4 * a.abs().max(b.abs()) + 1e-12;
        assert!(ok(op.gm, fd_gm), "({vds},{vgs},{vbs}): gm {:.8e} vs FD {fd_gm:.8e}", op.gm);
        assert!(ok(op.gds, fd_gds), "({vds},{vgs},{vbs}): gds {:.8e} vs FD {fd_gds:.8e}", op.gds);
        assert!(ok(op.gmbs, fd_gmb), "({vds},{vgs},{vbs}): gmbs {:.8e} vs FD {fd_gmb:.8e}", op.gmbs);
    }
}

#[test]
fn substrate_and_gidl_jacobians_match_finite_differences() {
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);
    let h = 1e-6;

    let f = |q: fn(&Bsim4v8Op) -> Value, vds: Value, vgs: Value, vbs: Value| {
        q(&op_at(&dev, vds, vgs, vbs))
    };
    let ok = |a: Value, b: Value| (a - b).abs() <= 1e-3 * a.abs().max(b.abs()) + 1e-16;

    // Substrate current at a strong-inversion, high-Vds point.
    let (vds, vgs, vbs) = (1.1, 0.7, -0.2);
    let op = op_at(&dev, vds, vgs, vbs);
    assert!(op.csub > 0.0, "Isub should be active: {:.3e}", op.csub);
    let csub = |o: &Bsim4v8Op| o.csub;
    let fd_g = (f(csub, vds, vgs + h, vbs) - f(csub, vds, vgs - h, vbs)) / (2.0 * h);
    let fd_d = (f(csub, vds + h, vgs, vbs) - f(csub, vds - h, vgs, vbs)) / (2.0 * h);
    let fd_b = (f(csub, vds, vgs, vbs + h) - f(csub, vds, vgs, vbs - h)) / (2.0 * h);
    assert!(ok(op.gbgs, fd_g), "gbgs {:.6e} vs FD {fd_g:.6e}", op.gbgs);
    assert!(ok(op.gbds, fd_d), "gbds {:.6e} vs FD {fd_d:.6e}", op.gbds);
    assert!(ok(op.gbbs, fd_b), "gbbs {:.6e} vs FD {fd_b:.6e}", op.gbbs);

    // GIDL needs vds - vgs > EGIDL; bias accordingly. The Ggidl* of b4ld.c
    // are complete raw-frame partials w.r.t. (vds, vgs, vbs) — the vbd
    // chain of the body factor is already folded into Ggidld/Ggidlb.
    let (vds, vgs, vbs) = (1.1, 0.05, 0.0);
    let op = op_at(&dev, vds, vgs, vbs);
    assert!(op.igidl > 0.0, "Igidl should be active: {:.3e}", op.igidl);
    let igidl = |o: &Bsim4v8Op| o.igidl;
    let fd_d = (f(igidl, vds + h, vgs, vbs) - f(igidl, vds - h, vgs, vbs)) / (2.0 * h);
    assert!(ok(op.ggidld, fd_d), "ggidld {:.6e} vs FD {fd_d:.6e}", op.ggidld);
    let fd_g = (f(igidl, vds, vgs + h, vbs) - f(igidl, vds, vgs - h, vbs)) / (2.0 * h);
    assert!(ok(op.ggidlg, fd_g), "ggidlg {:.6e} vs FD {fd_g:.6e}", op.ggidlg);
    let fd_b = (f(igidl, vds, vgs, vbs + h) - f(igidl, vds, vgs, vbs - h)) / (2.0 * h);
    assert!(ok(op.ggidlb, fd_b), "ggidlb {:.6e} vs FD {fd_b:.6e}", op.ggidlb);
}

#[test]
fn diode_conductances_match_finite_differences() {
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);
    let h = 1e-7;
    for &(vds, vgs, vbs) in &[(0.5, 0.0, -0.6), (0.5, 0.0, 0.3), (0.0, 0.0, 0.9)] {
        let op = op_at(&dev, vds, vgs, vbs);
        let cbs = |vbs: Value| op_at(&dev, vds, vgs, vbs).cbs;
        let fd = (cbs(vbs + h) - cbs(vbs - h)) / (2.0 * h);
        let ok = |a: Value, b: Value| (a - b).abs() <= 1e-4 * a.abs().max(b.abs()) + 1e-15;
        assert!(ok(op.gbs, fd), "({vds},{vgs},{vbs}): gbs {:.8e} vs FD {fd:.8e}", op.gbs);
    }
}

#[test]
fn subthreshold_slope_is_physical() {
    // Deep subthreshold: the slope must be ~n*vtm*ln(10) per decade, with
    // n between 1 and 2 for this card.
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);
    let vds = 0.05;
    let i1 = op_at(&dev, vds, 0.10, 0.0).cd;
    let i2 = op_at(&dev, vds, 0.20, 0.0).cd;
    assert!(i1 > 0.0 && i2 > i1, "i1={i1:.3e} i2={i2:.3e}");
    let slope_mv_per_dec = 0.1 / (i2.log10() - i1.log10()) * 1e3;
    let vt_ln10 = dev.model_temp.vtm * (10.0_f64).ln() * 1e3;
    assert!(
        slope_mv_per_dec > vt_ln10 && slope_mv_per_dec < 2.2 * vt_ln10,
        "slope {slope_mv_per_dec:.1} mV/dec outside (1..2.2)*{vt_ln10:.1}"
    );
}

#[test]
fn body_bias_raises_vth() {
    let dev = nmos_device(1e-6, 0.5e-6, 1.0, T300);
    let v0 = op_at(&dev, 0.05, 0.6, 0.0).von;
    let vb = op_at(&dev, 0.05, 0.6, -0.9).von;
    assert!(vb > v0 + 0.03, "body effect: von(vbs=-0.9)={vb:.4} von(0)={v0:.4}");
}

#[test]
fn continuity_across_vgsteff_transition() {
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);
    let vds = 0.55;
    let mut prev: Option<Bsim4v8Op> = None;
    let mut vgs = 0.0;
    while vgs <= 1.1 + 1e-12 {
        let op = op_at(&dev, vds, vgs, 0.0);
        assert!(op.cd.is_finite() && op.gm.is_finite() && op.gds.is_finite());
        assert!(op.cd >= 0.0);
        if let Some(prev_op) = &prev {
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
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);
    let vgs = 0.8;
    let mut prev: Option<Bsim4v8Op> = None;
    let mut vds = 0.0;
    while vds <= 1.1 + 1e-12 {
        let op = op_at(&dev, vds, vgs, 0.0);
        if let Some(prev_op) = &prev {
            let dvds = 1e-3;
            let didl = (op.cd - prev_op.cd).abs();
            let bound = 1.5 * dvds * op.gds.max(prev_op.gds).max(1e-18) + 1e-15;
            assert!(
                didl <= bound,
                "Id jump at vds={vds:.3}: dI={didl:.3e} bound={bound:.3e}"
            );
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
    // With identical S/D areas and symmetric S/D model cards, swapping
    // source and drain mirrors the device.
    let dev = nmos_device(1e-6, 0.5e-6, 1.0, T300);
    let fwd = op_at(&dev, 0.7, 1.0, 0.0);
    let rev = op_at(&dev, -0.7, 0.3, -0.7); // vgd=1.0, vbd=0 in swapped frame
    assert_eq!(fwd.mode, 1);
    assert_eq!(rev.mode, -1);
    let rel = (fwd.cd - rev.cd).abs() / fwd.cd.abs();
    assert!(rel < 1e-12, "fwd {:.10e} vs rev {:.10e}", fwd.cd, rev.cd);
}

#[test]
fn junction_diode_matches_ijth_linearization() {
    let dev = nmos_device(1e-6, 0.5e-6, 1.0, T300);
    // Slope continuity at the ijth anchor.
    let (vjsm, _) = dev.inst.vjsm_fwd.expect("ijth anchors");
    let below = op_at(&dev, 0.0, 0.0, vjsm - 1e-9).cbs;
    let above = op_at(&dev, 0.0, 0.0, vjsm + 1e-9).cbs;
    assert!(
        (above - below).abs() < 1e-6 * above.abs(),
        "ijth linearization discontinuous at vjsm: {below:.8e} vs {above:.8e}"
    );
    // Far above the anchor the current is linear in vbs.
    let i1 = op_at(&dev, 0.0, 0.0, vjsm + 0.2).cbs;
    let i2 = op_at(&dev, 0.0, 0.0, vjsm + 0.3).cbs;
    let i3 = op_at(&dev, 0.0, 0.0, vjsm + 0.4).cbs;
    let d1 = i2 - i1;
    let d2 = i3 - i2;
    assert!(
        (d1 - d2).abs() < 1e-9 * d1.abs(),
        "post-ijth current not linear: {d1:.6e} vs {d2:.6e}"
    );
}

#[test]
fn charge_matrix_consistency_and_fd() {
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);
    let charge = |vds: Value, vgs: Value, vbs: Value| {
        dev.eval(Bsim4v8Bias { vds, vgs, vbs }, GMIN, true)
            .expect("charge eval")
            .charge
            .expect("charge state")
    };
    let biases: [(Value, Value, Value); 6] = [
        (0.05, 0.9, 0.0),
        (1.0, 0.9, 0.0),
        (0.5, 0.2, 0.0),
        (1.1, 1.1, -0.6),
        (0.001, 0.7, -0.2),
        (-0.6, 0.4, -0.6), // inverse mode
    ];
    for &(vds, vgs, vbs) in &biases {
        let c0 = charge(vds, vgs, vbs);
        assert!(c0.qgate.is_finite() && c0.qbulk.is_finite() && c0.qdrn.is_finite());
        assert!(c0.capbs > 0.0 && c0.capbd > 0.0);
        // Charge neutrality of the intrinsic partition.
        let sum = c0.qgate + c0.qbulk + c0.qdrn + c0.qsrc;
        assert!(sum.abs() < 1e-25, "intrinsic charges not neutral: {sum:.3e}");
        // Row sums of the 4x4 completion are zero by construction.
        assert!((c0.cggb + c0.cdgb + c0.cbgb + c0.csgb).abs() < 1e-25);
        assert!((c0.cgdb + c0.cddb + c0.cbdb + c0.csdb).abs() < 1e-25);

        // FD of the intrinsic gate charge vs the gate row, in the mode
        // frame (normal mode only; h in the mode-swapped vgs direction).
        if vds >= 0.0 {
            let h = 1e-7;
            let fd_g = (charge(vds, vgs + h, vbs).qgate - charge(vds, vgs - h, vbs).qgate)
                / (2.0 * h);
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
fn no_nan_across_wide_sweep() {
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);
    for vg_i in 0..=11 {
        for vd_i in -11..=11 {
            for vb_i in -3..=1 {
                let bias = Bsim4v8Bias {
                    vds: 0.1 * vd_i as Value,
                    vgs: 0.1 * vg_i as Value,
                    vbs: 0.3 * vb_i as Value,
                };
                let op = dev.eval(bias, GMIN, true).expect("eval");
                assert!(op.cd.is_finite(), "cd NaN at {bias:?}");
                assert!(op.gm.is_finite() && op.gds.is_finite() && op.gmbs.is_finite());
                assert!(op.cbs.is_finite() && op.cbd.is_finite());
                assert!(op.csub.is_finite() && op.igidl.is_finite() && op.igisl.is_finite());
                let c = op.charge.expect("charges");
                assert!(c.qgate.is_finite() && c.qbulk.is_finite() && c.qdrn.is_finite());
                assert!(c.cggb.is_finite() && c.cddb.is_finite() && c.cbsb.is_finite());
                assert!(c.qg_state().is_finite() && c.qd_state().is_finite());
            }
        }
    }
}

#[test]
fn limiting_follows_b4ld_sequence() {
    let dev = nmos_device(1e-6, 0.5e-6, 1.0, T300);
    let old = Bsim4v8Bias { vds: 0.0, vgs: 0.0, vbs: 0.0 };
    // A huge forward body jump must be clipped by pnjlim.
    let (lim, check) = dev.limit_voltages(
        Bsim4v8Bias { vds: 0.1, vgs: 0.5, vbs: 5.0 },
        old,
        0.0,
    );
    assert!(check, "pnjlim should flag the clipped body voltage");
    assert!(lim.vbs < 1.0, "vbs should be limited, got {}", lim.vbs);
    // A small step passes through unchanged.
    let (lim, check) = dev.limit_voltages(
        Bsim4v8Bias { vds: 0.01, vgs: 0.02, vbs: -0.01 },
        old,
        0.0,
    );
    assert!(!check);
    assert!((lim.vds - 0.01).abs() < 1e-15 && (lim.vgs - 0.02).abs() < 1e-15);
}

#[test]
fn pmos_polarity_mirrors_nmos_conventions() {
    let dev = pmos_device(1e-6, 90e-9, T300);
    // Node-space PMOS bias: vgs=-1.0, vds=-0.5 -> folded internal +1.0/+0.5.
    let op = dev.eval_polarity(-0.5, -1.0, 0.0, GMIN, false).expect("eval");
    assert_eq!(op.mode, 1);
    assert!(op.cd > 0.0, "internal-frame current positive, got {}", op.cd);
}
