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

fn nmos_mobmod_device(mob_mod: i32, temp_k: Value) -> Bsim4v8 {
    let mut card = nmos45();
    card.insert("MOBMOD".to_string(), mob_mod as Value);
    // Keep the high-k Coulombic branch in a numerically useful range; the
    // default n45 UD value was tuned for the legacy mobility branches.
    card.insert("UD".to_string(), 1.0e-3);
    let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
    Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), temp_k).expect("mobMod device")
}

#[test]
fn fractional_model_selectors_round_like_ngspice46_before_validation() {
    for (param, raw, expected) in [
        ("MOBMOD", 0.5, 1),
        ("MOBMOD", 1.5, 2),
        ("GEOMOD", 0.5, 1),
        ("GEOMOD", 1.5, 2),
        ("ACNQSMOD", 0.5, 1),
        ("ACNQSMOD", 1.4, 1),
    ] {
        let mut card = nmos45();
        card.insert(param.to_string(), raw);
        let model = Bsim4v8Model::try_from_params(&card, false, T300)
            .unwrap_or_else(|err| panic!("{param}={raw} should round like ngspice-46: {err}"));
        let got = match param {
            "MOBMOD" => model.mob_mod,
            "GEOMOD" => model.geo_mod,
            "ACNQSMOD" => model.acnqs_mod,
            _ => unreachable!(),
        };
        assert_eq!(got, expected, "{param}={raw} effective selector");
    }

    for (param, raw) in [("MOBMOD", 6.5), ("GEOMOD", 10.5), ("ACNQSMOD", 1.5)] {
        let mut card = nmos45();
        card.insert(param.to_string(), raw);
        let err = Bsim4v8Model::try_from_params(&card, false, T300)
            .expect_err("rounded out-of-range selectors remain fail-closed");
        assert!(
            err.contains(param),
            "{param}={raw}: error should name selector, got {err}"
        );
    }
}

fn nmos_geomod_device(geo_mod: i32, nf: Value, min_sd: i32) -> Bsim4v8 {
    let mut card = nmos45();
    card.insert("GEOMOD".to_string(), geo_mod as Value);
    let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
    Bsim4v8::new(
        "m1".to_string(),
        model,
        Bsim4v8Geometry {
            l: 45e-9,
            w: 1e-6,
            nf,
            min_sd,
            ..Bsim4v8Geometry::default()
        },
        T300,
    )
    .expect("geoMod device")
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
    assert!(
        dev.inst.vfbzb.is_finite() && dev.inst.vfbzb < 0.0,
        "vfbzb={}",
        dev.inst.vfbzb
    );
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
fn transient_nqs_mode_combinations_are_scoped() {
    let mut trnqs_card = nmos45();
    trnqs_card.insert("TRNQSMOD".to_string(), 1.0);
    trnqs_card.insert("ACNQSMOD".to_string(), 0.0);
    trnqs_card.insert("RGATEMOD".to_string(), 0.0);
    trnqs_card.insert("RBODYMOD".to_string(), 0.0);
    trnqs_card.insert("RDSMOD".to_string(), 0.0);
    let trnqs_model = Arc::new(Bsim4v8Model::from_params(&trnqs_card, false, T300));
    let trnqs = Bsim4v8::new("m1".to_string(), trnqs_model, geom(1e-6, 45e-9, 1.0), T300)
        .expect("TRNQSMOD=1 canonical transient topology constructs natively");
    assert_eq!(trnqs.model.trnqs_mod, 1);

    let mut trnqs_acnqs_card = trnqs_card.clone();
    trnqs_acnqs_card.insert("ACNQSMOD".to_string(), 1.0);
    let trnqs_acnqs_model = Arc::new(Bsim4v8Model::from_params(&trnqs_acnqs_card, false, T300));
    let trnqs_acnqs = Bsim4v8::new(
        "m1".to_string(),
        trnqs_acnqs_model,
        geom(1e-6, 45e-9, 1.0),
        T300,
    )
    .expect("TRNQSMOD=1 with ACNQSMOD=1 constructs natively");
    assert_eq!(trnqs_acnqs.model.trnqs_mod, 1);
    assert_eq!(trnqs_acnqs.model.acnqs_mod, 1);

    let mut trnqs_rgatemod1_card = trnqs_card.clone();
    trnqs_rgatemod1_card.insert("RGATEMOD".to_string(), 1.0);
    let trnqs_rgatemod1_model = Arc::new(Bsim4v8Model::from_params(
        &trnqs_rgatemod1_card,
        false,
        T300,
    ));
    let trnqs_rgatemod1 = Bsim4v8::new(
        "m1".to_string(),
        trnqs_rgatemod1_model,
        geom(1e-6, 45e-9, 1.0),
        T300,
    )
    .expect("TRNQSMOD=1 with RGATEMOD=1 constructs natively");
    assert_eq!(trnqs_rgatemod1.model.trnqs_mod, 1);
    assert_eq!(trnqs_rgatemod1.model.rgate_mod, 1);

    let mut trnqs_rgatemod2_card = trnqs_card.clone();
    trnqs_rgatemod2_card.insert("RGATEMOD".to_string(), 2.0);
    let trnqs_rgatemod2_model = Arc::new(Bsim4v8Model::from_params(
        &trnqs_rgatemod2_card,
        false,
        T300,
    ));
    let trnqs_rgatemod2 = Bsim4v8::new(
        "m1".to_string(),
        trnqs_rgatemod2_model,
        geom(1e-6, 45e-9, 1.0),
        T300,
    )
    .expect("TRNQSMOD=1 with RGATEMOD=2 constructs natively");
    assert_eq!(trnqs_rgatemod2.model.trnqs_mod, 1);
    assert_eq!(trnqs_rgatemod2.model.rgate_mod, 2);

    let mut trnqs_rgatemod3_card = trnqs_card.clone();
    trnqs_rgatemod3_card.insert("RGATEMOD".to_string(), 3.0);
    let trnqs_rgatemod3_model = Arc::new(Bsim4v8Model::from_params(
        &trnqs_rgatemod3_card,
        false,
        T300,
    ));
    let trnqs_rgatemod3 = Bsim4v8::new(
        "m1".to_string(),
        trnqs_rgatemod3_model,
        geom(1e-6, 45e-9, 1.0),
        T300,
    )
    .expect("TRNQSMOD=1 with RGATEMOD=3 constructs natively");
    assert_eq!(trnqs_rgatemod3.model.trnqs_mod, 1);
    assert_eq!(trnqs_rgatemod3.model.rgate_mod, 3);

    let mut trnqs_rdsmod1_card = trnqs_card.clone();
    trnqs_rdsmod1_card.insert("RDSMOD".to_string(), 1.0);
    let trnqs_rdsmod1_model = Arc::new(Bsim4v8Model::from_params(&trnqs_rdsmod1_card, false, T300));
    let trnqs_rdsmod1 = Bsim4v8::new(
        "m1".to_string(),
        trnqs_rdsmod1_model,
        geom(1e-6, 45e-9, 1.0),
        T300,
    )
    .expect("TRNQSMOD=1 with RDSMOD=1 constructs natively");
    assert_eq!(trnqs_rdsmod1.model.trnqs_mod, 1);
    assert_eq!(trnqs_rdsmod1.model.rds_mod, 1);

    for rbody_mod in [1.0, 2.0] {
        let mut card = trnqs_card.clone();
        card.insert("RBODYMOD".to_string(), rbody_mod);
        let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
        let dev = Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
            .expect("TRNQSMOD=1 with RBODYMOD=1/2 constructs natively");
        assert_eq!(dev.model.trnqs_mod, 1);
        assert_eq!(dev.model.rbody_mod as Value, rbody_mod);
    }

    let mut acnqs_card = nmos45();
    acnqs_card.insert("ACNQSMOD".to_string(), 1.0);
    acnqs_card.insert("TRNQSMOD".to_string(), 0.0);
    acnqs_card.insert("RGATEMOD".to_string(), 0.0);
    acnqs_card.insert("RBODYMOD".to_string(), 0.0);
    acnqs_card.insert("RDSMOD".to_string(), 0.0);
    let acnqs_model = Arc::new(Bsim4v8Model::from_params(&acnqs_card, false, T300));
    let acnqs = Bsim4v8::new("m1".to_string(), acnqs_model, geom(1e-6, 45e-9, 1.0), T300)
        .expect("ACNQSMOD=1 canonical AC topology constructs natively");
    assert_eq!(acnqs.model.acnqs_mod, 1);

    let mut acnqs_rgate2_card = nmos45();
    acnqs_rgate2_card.insert("ACNQSMOD".to_string(), 1.0);
    acnqs_rgate2_card.insert("TRNQSMOD".to_string(), 0.0);
    acnqs_rgate2_card.insert("RGATEMOD".to_string(), 2.0);
    acnqs_rgate2_card.insert("RBODYMOD".to_string(), 0.0);
    acnqs_rgate2_card.insert("RDSMOD".to_string(), 0.0);
    let acnqs_rgate2_model = Arc::new(Bsim4v8Model::from_params(&acnqs_rgate2_card, false, T300));
    let acnqs_rgate2 = Bsim4v8::new(
        "m1".to_string(),
        acnqs_rgate2_model,
        geom(1e-6, 45e-9, 1.0),
        T300,
    )
    .expect("ACNQSMOD=1 with RGATEMOD=2 constructs natively");
    assert_eq!(acnqs_rgate2.model.acnqs_mod, 1);
    assert_eq!(acnqs_rgate2.model.rgate_mod, 2);

    let mut acnqs_rgate1_card = nmos45();
    acnqs_rgate1_card.insert("ACNQSMOD".to_string(), 1.0);
    acnqs_rgate1_card.insert("TRNQSMOD".to_string(), 0.0);
    acnqs_rgate1_card.insert("RGATEMOD".to_string(), 1.0);
    acnqs_rgate1_card.insert("RBODYMOD".to_string(), 0.0);
    acnqs_rgate1_card.insert("RDSMOD".to_string(), 0.0);
    let acnqs_rgate1_model = Arc::new(Bsim4v8Model::from_params(&acnqs_rgate1_card, false, T300));
    let acnqs_rgate1 = Bsim4v8::new(
        "m1".to_string(),
        acnqs_rgate1_model,
        geom(1e-6, 45e-9, 1.0),
        T300,
    )
    .expect("ACNQSMOD=1 with RGATEMOD=1 constructs natively");
    assert_eq!(acnqs_rgate1.model.acnqs_mod, 1);
    assert_eq!(acnqs_rgate1.model.rgate_mod, 1);

    let mut acnqs_rgate3_card = nmos45();
    acnqs_rgate3_card.insert("ACNQSMOD".to_string(), 1.0);
    acnqs_rgate3_card.insert("TRNQSMOD".to_string(), 0.0);
    acnqs_rgate3_card.insert("RGATEMOD".to_string(), 3.0);
    acnqs_rgate3_card.insert("RBODYMOD".to_string(), 0.0);
    acnqs_rgate3_card.insert("RDSMOD".to_string(), 0.0);
    let acnqs_rgate3_model = Arc::new(Bsim4v8Model::from_params(&acnqs_rgate3_card, false, T300));
    let acnqs_rgate3 = Bsim4v8::new(
        "m1".to_string(),
        acnqs_rgate3_model,
        geom(1e-6, 45e-9, 1.0),
        T300,
    )
    .expect("ACNQSMOD=1 with RGATEMOD=3 constructs natively");
    assert_eq!(acnqs_rgate3.model.acnqs_mod, 1);
    assert_eq!(acnqs_rgate3.model.rgate_mod, 3);

    let mut acnqs_rdsmod1_card = nmos45();
    acnqs_rdsmod1_card.insert("ACNQSMOD".to_string(), 1.0);
    acnqs_rdsmod1_card.insert("TRNQSMOD".to_string(), 0.0);
    acnqs_rdsmod1_card.insert("RGATEMOD".to_string(), 0.0);
    acnqs_rdsmod1_card.insert("RBODYMOD".to_string(), 0.0);
    acnqs_rdsmod1_card.insert("RDSMOD".to_string(), 1.0);
    let acnqs_rdsmod1_model = Arc::new(Bsim4v8Model::from_params(&acnqs_rdsmod1_card, false, T300));
    let acnqs_rdsmod1 = Bsim4v8::new(
        "m1".to_string(),
        acnqs_rdsmod1_model,
        geom(1e-6, 45e-9, 1.0),
        T300,
    )
    .expect("ACNQSMOD=1 with RDSMOD=1 constructs natively");
    assert_eq!(acnqs_rdsmod1.model.acnqs_mod, 1);
    assert_eq!(acnqs_rdsmod1.model.rds_mod, 1);

    for rbody_mod in [1.0, 2.0] {
        let mut card = nmos45();
        card.insert("ACNQSMOD".to_string(), 1.0);
        card.insert("TRNQSMOD".to_string(), 0.0);
        card.insert("RGATEMOD".to_string(), 0.0);
        card.insert("RBODYMOD".to_string(), rbody_mod);
        card.insert("RDSMOD".to_string(), 0.0);
        let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
        let dev = Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
            .expect("ACNQSMOD=1 with RBODYMOD=1/2 constructs natively");
        assert_eq!(dev.model.acnqs_mod, 1);
        assert_eq!(dev.model.rbody_mod as Value, rbody_mod);
    }

    let mut rgate3_card = nmos45();
    rgate3_card.insert("RGATEMOD".to_string(), 3.0);
    let rgate3_model = Arc::new(Bsim4v8Model::from_params(&rgate3_card, false, T300));
    let rgate3 = Bsim4v8::new("m1".to_string(), rgate3_model, geom(1e-6, 45e-9, 1.0), T300)
        .expect("RGATEMOD=3 constructs natively");
    assert_eq!(rgate3.model.rgate_mod, 3);

    let mut mtrl_card = nmos45();
    mtrl_card.insert("MTRLMOD".to_string(), 1.0);
    let mtrl_model = Arc::new(Bsim4v8Model::from_params(&mtrl_card, false, T300));
    let mtrl = Bsim4v8::new("m1".to_string(), mtrl_model, geom(1e-6, 45e-9, 1.0), T300)
        .expect("MTRLMOD=1/MTRLCOMPATMOD=0 EOT iteration constructs natively");
    assert_eq!(mtrl.model.mtrl_mod, 1);
    assert_eq!(mtrl.model.mtrl_compat_mod, 0);

    let mut rdsmod_card = nmos45();
    rdsmod_card.insert("RDSMOD".to_string(), 1.0);
    let rdsmod_model = Arc::new(Bsim4v8Model::from_params(&rdsmod_card, false, T300));
    let rdsmod = Bsim4v8::new("m1".to_string(), rdsmod_model, geom(1e-6, 45e-9, 1.0), T300)
        .expect("RDSMOD=1 external source/drain resistance constructs natively");
    assert_eq!(rdsmod.model.rds_mod, 1);

    let mut rbodymod_card = nmos45();
    rbodymod_card.insert("RBODYMOD".to_string(), 1.0);
    let rbodymod_model = Arc::new(Bsim4v8Model::from_params(&rbodymod_card, false, T300));
    let rbodymod = Bsim4v8::new(
        "m1".to_string(),
        rbodymod_model,
        geom(1e-6, 45e-9, 1.0),
        T300,
    )
    .expect("RBODYMOD=1 substrate resistance network constructs natively");
    assert_eq!(rbodymod.model.rbody_mod, 1);

    let mut rbodymod2_card = nmos45();
    rbodymod2_card.insert("RBODYMOD".to_string(), 2.0);
    let rbodymod2_model = Arc::new(Bsim4v8Model::from_params(&rbodymod2_card, false, T300));
    let rbodymod2 = Bsim4v8::new(
        "m1".to_string(),
        rbodymod2_model,
        geom(1e-6, 45e-9, 1.0),
        T300,
    )
    .expect("RBODYMOD=2 geometry-scaled substrate resistance network constructs natively");
    assert_eq!(rbodymod2.model.rbody_mod, 2);

    let mut card = nmos45();
    card.insert("CVCHARGEMOD".to_string(), 2.0);
    let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
    let dev = Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
        .expect("CVCHARGEMOD=2 constructs natively");
    let bias = Bsim4v8Bias {
        vds: 0.5,
        vgs: 0.8,
        vbs: 0.0,
    };
    assert!(dev.eval(bias, GMIN, false).is_ok());
    dev.eval(bias, GMIN, true)
        .expect("CVCHARGEMOD=2 uses the native nonzero charge path")
        .charge
        .expect("CVCHARGEMOD=2 charges");

    let mut card = nmos45();
    card.insert("CVCHARGEMOD".to_string(), 3.0);
    let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
    let dev = Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
        .expect("CVCHARGEMOD=3 constructs natively");
    assert!(dev.eval(bias, GMIN, false).is_ok());
    dev.eval(bias, GMIN, true)
        .expect("CVCHARGEMOD=3 uses the native nonzero charge path")
        .charge
        .expect("CVCHARGEMOD=3 charges");

    for invalid in [2.5] {
        let mut card = nmos45();
        card.insert("CVCHARGEMOD".to_string(), invalid);
        let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
        let dev = Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
            .unwrap_or_else(|err| panic!("CVCHARGEMOD={invalid} should construct for DC: {err}"));
        assert!(dev.eval(bias, GMIN, false).is_ok());
        let err = match dev.eval(bias, GMIN, true) {
            Ok(_) => panic!("CVCHARGEMOD={invalid} should reject charge eval"),
            Err(err) => err,
        };
        assert!(
            err.contains("CVCHARGEMOD"),
            "CVCHARGEMOD={invalid}: unexpected error text: {err}"
        );
    }

    let mut card = nmos45();
    card.insert("CAPMOD".to_string(), 0.0);
    card.insert("XPART".to_string(), -1.0);
    let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
    let dev = Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
        .expect("XPART<0 CAPMOD=0 constructs for DC");
    let bias = Bsim4v8Bias {
        vds: 0.5,
        vgs: 0.8,
        vbs: 0.0,
    };
    assert!(dev.eval(bias, GMIN, false).is_ok());
    let charge = dev
        .eval(bias, GMIN, true)
        .expect("XPART<0 CAPMOD=0 charge request uses native overlap/junction path")
        .charge
        .expect("charges");
    assert_eq!(charge.qgate, 0.0);
    assert_eq!(charge.qbulk, 0.0);
    assert_eq!(charge.qdrn, 0.0);
    assert_eq!(charge.qsrc, 0.0);
}

#[test]
fn stress_geometry_adjusts_instance_temperature_tail() {
    let mut card = nmos45();
    card.insert("SAREF".to_string(), 1.0e-6);
    card.insert("SBREF".to_string(), 1.0e-6);
    card.insert("KU0".to_string(), 2.0e-8);
    card.insert("KVSAT".to_string(), 0.25);
    card.insert("KVTH0".to_string(), 1.5e-9);
    card.insert("STK2".to_string(), 2.0e-9);
    card.insert("STETA0".to_string(), 2.0e-10);
    let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));

    let baseline = Bsim4v8::new(
        "m_base".to_string(),
        Arc::clone(&model),
        geom(1e-6, 45e-9, 1.0),
        T300,
    )
    .expect("baseline BSIM4 device");
    let stressed = Bsim4v8::new(
        "m_stress".to_string(),
        model,
        Bsim4v8Geometry {
            sa: 0.2e-6,
            sb: 0.4e-6,
            ..geom(1e-6, 45e-9, 1.0)
        },
        T300,
    )
    .expect("stress-enabled BSIM4 device");

    assert_ne!(stressed.inst.u0temp, baseline.inst.u0temp);
    assert_ne!(stressed.inst.vsattemp, baseline.inst.vsattemp);
    assert_ne!(stressed.inst.vth0, baseline.inst.vth0);
    assert_ne!(stressed.inst.eta0, baseline.inst.eta0);
    assert_ne!(stressed.inst.k2, baseline.inst.k2);
}

#[test]
fn geomod1_implicit_diffusions_follow_bsim4_paeffgeo() {
    // Berkeley BSIM4PAeffGeo case 1: source keeps isolated end diffusion,
    // drain uses shared diffusion for end+internal fingers.
    let dev = nmos_geomod_device(1, 3.0, 0);
    let dmcg_eff = dev.model.dmcg - dev.model.dmcgt;
    let dmci_eff = dev.model.dmci;
    let t0 = dmcg_eff + dmci_eff;
    let p_iso = t0 + t0 + dev.size.weff_cj;
    let p_sha = dmcg_eff + dmcg_eff;
    let a_iso = t0 * dev.size.weff_cj;
    let a_sha = dmcg_eff * dev.size.weff_cj;

    assert_rel(dev.inst.pseff, p_iso + 2.0 * p_sha, "geomod1 pseff");
    assert_rel(dev.inst.pdeff, 3.0 * p_sha, "geomod1 pdeff");
    assert_rel(dev.inst.aseff, a_iso + 2.0 * a_sha, "geomod1 aseff");
    assert_rel(dev.inst.adeff, 3.0 * a_sha, "geomod1 adeff");

    let geo0 = nmos_geomod_device(0, 3.0, 0);
    assert!(
        (geo0.inst.pdeff - dev.inst.pdeff).abs() > 1.0e-9,
        "fixture must distinguish GEOMOD=1 from GEOMOD=0"
    );
}

#[test]
fn rgeomod1_implicit_resistance_uses_bsim4_rdseffgeo() {
    let mut card = nmos45();
    card.insert("GEOMOD".to_string(), 4.0);
    card.insert("DMDG".to_string(), 8.0e-8);
    let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
    let dev = Bsim4v8::new(
        "m_rgeo".to_string(),
        model,
        Bsim4v8Geometry {
            l: 45e-9,
            w: 1e-6,
            nf: 3.0,
            rgeo_mod: 1,
            ..Bsim4v8Geometry::default()
        },
        T300,
    )
    .expect("rgeoMod device");

    let weff_cj = dev.size.weff_cj;
    let rsh = dev.model.sheet_resistance;
    let dmcg = dev.model.dmcg - dev.model.dmcgt;
    let dmdg = dev.model.dmdg - dev.model.dmcgt;
    let rint = rsh * dmcg / (weff_cj * 2.0);
    let source_rend = rsh * dmcg / weff_cj;
    let drain_rend = rsh * dmdg / weff_cj;
    let source_r = rint * source_rend / (rint + source_rend);
    let drain_r = rint * drain_rend / (rint + drain_rend);

    assert_rel(
        dev.inst.source_conductance,
        1.0 / source_r,
        "rgeomod1 source conductance",
    );
    assert_rel(
        dev.inst.drain_conductance,
        1.0 / drain_r,
        "rgeomod1 drain conductance",
    );
    assert!(
        (dev.inst.source_conductance - dev.inst.drain_conductance).abs()
            > 0.1 * dev.inst.source_conductance.abs(),
        "fixture must distinguish source and drain rgeo paths"
    );
}

#[test]
fn rgeomod_selectors_1_to_8_are_supported() {
    let base = {
        let mut card = nmos45();
        card.insert("GEOMOD".to_string(), 0.0);
        Arc::new(Bsim4v8Model::from_params(&card, false, T300))
    };

    for rgeo_mod in 1..=8 {
        let dev = Bsim4v8::new(
            format!("m_rgeo{rgeo_mod}"),
            Arc::clone(&base),
            Bsim4v8Geometry {
                l: 45e-9,
                w: 1e-6,
                nf: 3.0,
                rgeo_mod,
                ..Bsim4v8Geometry::default()
            },
            T300,
        )
        .unwrap_or_else(|err| panic!("RGEOMOD={rgeo_mod} should construct: {err}"));

        assert!(
            dev.inst.source_conductance.is_finite() && dev.inst.source_conductance > 0.0,
            "RGEOMOD={rgeo_mod} source conductance={}",
            dev.inst.source_conductance
        );
        assert!(
            dev.inst.drain_conductance.is_finite() && dev.inst.drain_conductance > 0.0,
            "RGEOMOD={rgeo_mod} drain conductance={}",
            dev.inst.drain_conductance
        );
    }
}

#[test]
fn rgeomod_zero_end_diffusion_matches_ngspice_conductance_fallbacks() {
    let mut card = nmos45();
    card.insert("GEOMOD".to_string(), 5.0);
    card.insert("DMDG".to_string(), 8.0e-8);
    let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
    let geom = Bsim4v8Geometry {
        l: 45e-9,
        w: 1e-6,
        nf: 4.0,
        min_sd: 0,
        rgeo_mod: 1,
        ..Bsim4v8Geometry::default()
    };
    let rdsmod0 = Bsim4v8::new("m_rgeo_edge0".to_string(), Arc::clone(&model), geom, T300)
        .expect("RDSMOD=0 edge device");
    assert!(
        rdsmod0.inst.source_conductance.is_finite() && rdsmod0.inst.source_conductance > 0.0,
        "source side should still have positive geometry conductance"
    );
    assert_eq!(
        rdsmod0.inst.drain_conductance, 0.0,
        "ngspice setup does not lower a fixed resistor when RdseffGeo is non-positive/NaN"
    );

    let mut rdsmod1_card = nmos45();
    rdsmod1_card.insert("GEOMOD".to_string(), 5.0);
    rdsmod1_card.insert("DMDG".to_string(), 8.0e-8);
    rdsmod1_card.insert("RDSMOD".to_string(), 1.0);
    let rdsmod1_model = Arc::new(Bsim4v8Model::from_params(&rdsmod1_card, false, T300));
    let rdsmod1 = Bsim4v8::new("m_rgeo_edge1".to_string(), rdsmod1_model, geom, T300)
        .expect("RDSMOD=1 edge device");
    assert!(
        rdsmod1.inst.source_conductance.is_finite() && rdsmod1.inst.source_conductance > 0.0,
        "source side should still have positive geometry conductance"
    );
    assert_eq!(
        rdsmod1.inst.drain_conductance, 1.0e3,
        "RDSMOD=1 forced prime node uses ngspice's 1000 mho fallback"
    );
}

#[test]
fn ngspice_pinned_nmos_mobmod3_to_6() {
    // Same n45 geometry as the default DC oracle, but with UD reduced so the
    // high-k Coulombic term is observable without swamping the branch. The
    // references come from ngspice-46 `ngspice_con.exe -b` with @m1[id/gm/
    // gds/gmbs/vth/vdsat] at three bias points.
    #[derive(Clone, Copy)]
    struct Ref {
        mob_mod: i32,
        vds: Value,
        vgs: Value,
        vbs: Value,
        id: Value,
        gm: Value,
        gds: Value,
        gmbs: Value,
        vth: Value,
        vdsat: Value,
    }

    let refs: &[Ref] = &[
        Ref {
            mob_mod: 3,
            vds: 0.05,
            vgs: 0.9,
            vbs: 0.0,
            id: 2.51773358e-4,
            gm: 2.01537138e-4,
            gds: 4.58712168e-3,
            gmbs: 2.01972523e-5,
            vth: 3.96082238e-1,
            vdsat: 2.41702085e-1,
        },
        Ref {
            mob_mod: 3,
            vds: 1.1,
            vgs: 0.6,
            vbs: 0.0,
            id: 5.27286296e-4,
            gm: 2.12945681e-3,
            gds: 2.42527664e-4,
            gmbs: -1.69926892e-3,
            vth: 3.16523792e-1,
            vdsat: 1.54738832e-1,
        },
        Ref {
            mob_mod: 3,
            vds: 1.1,
            vgs: 1.1,
            vbs: -0.45,
            id: 2.34321512e-3,
            gm: 2.18386352e-3,
            gds: 1.23250984e-3,
            gmbs: -1.83230034e-3,
            vth: -4.43531872e-2,
            vdsat: 4.56395979e-1,
        },
        Ref {
            mob_mod: 4,
            vds: 0.05,
            vgs: 0.9,
            vbs: 0.0,
            id: 1.90989193e-4,
            gm: 1.59461868e-4,
            gds: 3.52498649e-3,
            gmbs: 1.62889383e-5,
            vth: 3.96082238e-1,
            vdsat: 2.86094941e-1,
        },
        Ref {
            mob_mod: 4,
            vds: 1.1,
            vgs: 0.6,
            vbs: 0.0,
            id: 4.36989042e-4,
            gm: 1.90881105e-3,
            gds: 2.15053371e-4,
            gmbs: -1.52594975e-3,
            vth: 3.16523792e-1,
            vdsat: 1.81614627e-1,
        },
        Ref {
            mob_mod: 4,
            vds: 1.1,
            vgs: 1.1,
            vbs: -0.45,
            id: 2.04318359e-3,
            gm: 1.86328104e-3,
            gds: 1.08503394e-3,
            gmbs: -1.56837552e-3,
            vth: -4.43531872e-2,
            vdsat: 5.63081965e-1,
        },
        Ref {
            mob_mod: 5,
            vds: 0.05,
            vgs: 0.9,
            vbs: 0.0,
            id: 1.90989193e-4,
            gm: 1.59461868e-4,
            gds: 3.52498649e-3,
            gmbs: 1.83144516e-5,
            vth: 3.96082238e-1,
            vdsat: 2.86094941e-1,
        },
        Ref {
            mob_mod: 5,
            vds: 1.1,
            vgs: 0.6,
            vbs: 0.0,
            id: 4.36989042e-4,
            gm: 1.90881105e-3,
            gds: 2.15053371e-4,
            gmbs: -1.52203080e-3,
            vth: 3.16523792e-1,
            vdsat: 1.81614627e-1,
        },
        Ref {
            mob_mod: 5,
            vds: 1.1,
            vgs: 1.1,
            vbs: -0.45,
            id: 2.03878177e-3,
            gm: 1.86022711e-3,
            gds: 1.08353708e-3,
            gmbs: -1.55591925e-3,
            vth: -4.43531872e-2,
            vdsat: 5.64435086e-1,
        },
        Ref {
            mob_mod: 6,
            vds: 0.05,
            vgs: 0.9,
            vbs: 0.0,
            id: 2.62997083e-9,
            gm: 2.63641627e-9,
            gds: 4.99534542e-8,
            gmbs: -1.56265107e-11,
            vth: 3.96082238e-1,
            vdsat: 4.96289162e-1,
        },
        Ref {
            mob_mod: 6,
            vds: 1.1,
            vgs: 0.6,
            vbs: 0.0,
            id: 6.68275633e-9,
            gm: 3.69329106e-8,
            gds: 3.78483378e-9,
            gmbs: -3.04686000e-8,
            vth: 3.16523792e-1,
            vdsat: 2.90364309e-1,
        },
        Ref {
            mob_mod: 6,
            vds: 1.1,
            vgs: 1.1,
            vbs: -0.45,
            id: 4.12411377e-8,
            gm: 3.41102986e-8,
            gds: 2.13074349e-8,
            gmbs: -3.49502071e-8,
            vth: -4.43531872e-2,
            vdsat: 1.08302467,
        },
    ];

    for r in refs {
        let dev = nmos_mobmod_device(r.mob_mod, T300);
        let op = op_at(&dev, r.vds, r.vgs, r.vbs);
        let label = format!(
            "mobMod={} vds={} vgs={} vbs={}",
            r.mob_mod, r.vds, r.vgs, r.vbs
        );
        assert_rel(op.cd, r.id, &format!("{label} id"));
        assert_rel(op.gm, r.gm, &format!("{label} gm"));
        assert_rel(op.gds, r.gds, &format!("{label} gds"));
        assert_rel(op.gmbs, r.gmbs, &format!("{label} gmbs"));
        assert_rel(op.von, r.vth, &format!("{label} vth"));
        assert_rel(op.vdsat, r.vdsat, &format!("{label} vdsat"));
    }
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
    assert!(
        ok(op.ggidld, fd_d),
        "ggidld {:.6e} vs FD {fd_d:.6e}",
        op.ggidld
    );
    let fd_g = (f(igidl, vds, vgs + h, vbs) - f(igidl, vds, vgs - h, vbs)) / (2.0 * h);
    assert!(
        ok(op.ggidlg, fd_g),
        "ggidlg {:.6e} vs FD {fd_g:.6e}",
        op.ggidlg
    );
    let fd_b = (f(igidl, vds, vgs, vbs + h) - f(igidl, vds, vgs, vbs - h)) / (2.0 * h);
    assert!(
        ok(op.ggidlb, fd_b),
        "ggidlb {:.6e} vs FD {fd_b:.6e}",
        op.ggidlb
    );
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
        assert!(
            ok(op.gbs, fd),
            "({vds},{vgs},{vbs}): gbs {:.8e} vs FD {fd:.8e}",
            op.gbs
        );
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
    assert!(
        vb > v0 + 0.03,
        "body effect: von(vbs=-0.9)={vb:.4} von(0)={v0:.4}"
    );
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
fn nondefault_diode_models_match_ngspice46_reverse_breakdown() {
    let device = |dio_mod: Value| {
        let mut card = nmos45();
        card.insert("DIOMOD".to_string(), dio_mod);
        let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
        Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
            .expect("non-default diode model constructs")
    };
    let cases = [
        // ngspice-46 reference, `models45.lib` n45 with `diomod=0`, `as=0.1p`,
        // `ps=2.2u`, `ad=pd=0`, and Vbs = -12 V:
        // @m1[ibs] = -3.39274e+13 A, @m1[gbs] = 1.249287e+15 S.
        (0.0, -3.39274e13, 1.249287e15),
        // Same deck with `diomod=2`, exercising the reverse-breakdown
        // linearization that keeps the current finite:
        // @m1[ibs] = -7.21345e-01 A, @m1[gbs] = 7.364476e-01 S.
        (2.0, -7.21345e-1, 7.364476e-1),
    ];
    for (mode, ibs_ref, gbs_ref) in cases {
        let dev = device(mode);
        let op = op_at(&dev, 0.0, 0.0, -12.0);
        let close = |a: Value, b: Value| (a - b).abs() <= 2e-5 * a.abs().max(b.abs()).max(1.0);
        assert!(
            close(op.cbs, ibs_ref),
            "dioMod={mode}: ibs/cbs rspice={:.8e} ngspice={ibs_ref:.8e}",
            op.cbs
        );
        assert!(
            close(op.gbs, gbs_ref),
            "dioMod={mode}: gbs rspice={:.8e} ngspice={gbs_ref:.8e}",
            op.gbs
        );
    }
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
        assert!(
            sum.abs() < 1e-25,
            "intrinsic charges not neutral: {sum:.3e}"
        );
        // Row sums of the 4x4 completion are zero by construction.
        assert!((c0.cggb + c0.cdgb + c0.cbgb + c0.csgb).abs() < 1e-25);
        assert!((c0.cgdb + c0.cddb + c0.cbdb + c0.csdb).abs() < 1e-25);

        // FD of the intrinsic gate charge vs the gate row, in the mode
        // frame (normal mode only; h in the mode-swapped vgs direction).
        if vds >= 0.0 {
            let h = 1e-7;
            let fd_g =
                (charge(vds, vgs + h, vbs).qgate - charge(vds, vgs - h, vbs).qgate) / (2.0 * h);
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
    let old = Bsim4v8Bias {
        vds: 0.0,
        vgs: 0.0,
        vbs: 0.0,
    };
    // A huge forward body jump must be clipped by pnjlim.
    let (lim, check) = dev.limit_voltages(
        Bsim4v8Bias {
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
        Bsim4v8Bias {
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
fn pmos_polarity_mirrors_nmos_conventions() {
    let dev = pmos_device(1e-6, 90e-9, T300);
    // Node-space PMOS bias: vgs=-1.0, vds=-0.5 -> folded internal +1.0/+0.5.
    let op = dev
        .eval_polarity(-0.5, -1.0, 0.0, GMIN, false)
        .expect("eval");
    assert_eq!(op.mode, 1);
    assert!(
        op.cd > 0.0,
        "internal-frame current positive, got {}",
        op.cd
    );
}

// ===================== pinned ngspice references =====================
//
// Values produced by a local ngspice-46 build running the decks in
// testdata/ (`wrdata`, 9 significant digits). The live oracle test below
// regenerates and re-checks every sweep; these pinned rows keep the oracle
// agreement in the default test suite. Comparison at 1e-6 relative — the
// port matched the full 12k-point oracle at <= 4.94e-9 relative (print
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
    // m1 = 1u/45n, vds=0.05, vbs=0, T=27C (nmos_idvg_vd50m_vb0).
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);
    let table: [(Value, Value, Value, Value, Value); 5] = [
        (
            0.0,
            3.12868811e-10,
            8.80428212e-09,
            2.54449222e-09,
            1.82391109e-09,
        ),
        (
            0.3,
            1.17939846e-06,
            2.99566556e-05,
            9.56158860e-06,
            4.95180212e-06,
        ),
        (
            0.6,
            1.17111258e-04,
            4.79554116e-04,
            1.92100141e-03,
            7.75030642e-05,
        ),
        (
            0.9,
            1.98523688e-04,
            1.60162691e-04,
            3.66700074e-03,
            3.30176312e-05,
        ),
        (
            1.1,
            2.24835254e-04,
            1.09457144e-04,
            4.24171044e-03,
            2.38798480e-05,
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
    assert_rel(op.von, 3.96082238e-01, "vth");
    assert_rel(op.vdsat, 2.80112388e-01, "vdsat(0.9)");
    assert_rel(op.cbd, -5.00003187e-14, "ibd");
    assert_rel(
        op_at(&dev, 0.05, 1.1, 0.0).vdsat,
        3.67445684e-01,
        "vdsat(1.1)",
    );

    // Body bias vbs=-0.9 at vgs=0.9 (nmos_idvg_vd50m_vbm09); the reverse
    // junctions carry the TAT + gmin leakage.
    let op = op_at(&dev, 0.05, 0.9, -0.9);
    assert_rel(op.cd, 1.72951222e-04, "id(vbs=-0.9)");
    assert_rel(op.gm, 2.15379091e-04, "gm(vbs=-0.9)");
    assert_rel(op.gds, 3.39957450e-03, "gds(vbs=-0.9)");
    assert_rel(op.gmbs, 2.47469449e-05, "gmbs(vbs=-0.9)");
    assert_rel(op.von, 5.01802278e-01, "vth(vbs=-0.9)");
    assert_rel(op.cbs, -9.00001154e-13, "ibs(vbs=-0.9)");
    assert_rel(op.cbd, -9.50001234e-13, "ibd(vbs=-0.9)");
}

#[test]
fn ngspice_pinned_nmos_idvg_saturation() {
    // m1 = 1u/45n, vds=1.1, vbs=0 (nmos_idvg_vd1100m_vb0). This card's
    // 22.6nm Leff puts gmbs negative in saturation (DIBL/DITS dominated) —
    // ngspice agrees point by point.
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);

    let op = op_at(&dev, 1.1, 0.0, 0.0);
    assert_rel(op.cd, 6.22105666e-09, "id(0)");
    assert_rel(op.gm, 1.74352960e-07, "gm(0)");
    assert_rel(op.gds, 1.66753226e-08, "gds(0)");
    assert_rel(op.gmbs, -1.36299333e-07, "gmbs(0)");
    assert_rel(op.von, 3.16523792e-01, "vth(0)");
    assert_rel(op.vdsat, 3.77590033e-02, "vdsat(0)");
    assert_rel(op.csub, 8.65521888e-14, "isub(0)");
    assert_rel(op.igidl, 1.02705712e-27, "igidl(0)");
    assert_rel(op.vgsteff, 4.64929501e-06, "vgsteff(0)");
    assert_rel(op.vdseff, 3.73373751e-02, "vdseff(0)");

    let op = op_at(&dev, 1.1, 0.6, 0.0);
    assert_rel(op.cd, 4.68109509e-04, "id(0.6)");
    assert_rel(op.gm, 1.97228540e-03, "gm(0.6)");
    assert_rel(op.gds, 2.39110397e-04, "gds(0.6)");
    assert_rel(op.gmbs, -1.73098462e-03, "gmbs(0.6)");
    assert_rel(op.vdsat, 1.72844541e-01, "vdsat(0.6)");
    assert_rel(op.csub, 1.13760643e-09, "isub(0.6)");
    assert_rel(op.vgsteff, 2.73384386e-01, "vgsteff(0.6)");
    assert_rel(op.vdseff, 1.70641200e-01, "vdseff(0.6)");

    let op = op_at(&dev, 1.1, 1.1, 0.0);
    assert_rel(op.cd, 1.40891935e-03, "id(1.1)");
    assert_rel(op.gm, 1.87452469e-03, "gm(1.1)");
    assert_rel(op.gds, 3.04461834e-04, "gds(1.1)");
    assert_rel(op.gmbs, -1.78776023e-03, "gmbs(1.1)");
    assert_rel(op.vdsat, 3.92689365e-01, "vdsat(1.1)");
    assert_rel(op.csub, 5.35400314e-11, "isub(1.1)");
    assert_rel(op.vgsteff, 7.83228617e-01, "vgsteff(1.1)");
    assert_rel(op.vdseff, 3.86196871e-01, "vdseff(1.1)");

    // vbs=-0.9 (nmos_idvg_vd1100m_vbm09): the DVT2/K2 chain drives the
    // operating-point von negative at this Leff; pinned as ngspice prints it.
    let op = op_at(&dev, 1.1, 1.1, -0.9);
    assert_rel(op.cd, 3.15052501e-03, "id(vbs=-0.9)");
    assert_rel(op.gm, 2.15484576e-03, "gm(vbs=-0.9)");
    assert_rel(op.gds, 2.35589192e-03, "gds(vbs=-0.9)");
    assert_rel(op.gmbs, -2.04349261e-03, "gmbs(vbs=-0.9)");
    assert_rel(op.von, -4.18954541e-01, "vth(vbs=-0.9)");
    assert_rel(op.vdsat, 5.81275439e-01, "vdsat(vbs=-0.9)");
    assert_rel(op.csub, 2.80024230e-13, "isub(vbs=-0.9)");
}

#[test]
fn ngspice_pinned_nmos_idvd() {
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);

    // vgs=0.5, vbs=0 (nmos_idvd_vg500m_vb0).
    let table: [(Value, Value, Value); 3] = [
        (0.1, 8.58989331e-05, 3.35711528e-04),
        (0.6, 1.73265569e-04, 1.73385984e-04),
        (1.1, 2.68791324e-04, 2.15459318e-04),
    ];
    for &(vds, id, gds) in &table {
        let op = op_at(&dev, vds, 0.5, 0.0);
        assert_rel(op.cd, id, &format!("vg0.5 id(vds={vds})"));
        assert_rel(op.gds, gds, &format!("vg0.5 gds(vds={vds})"));
    }
    let op = op_at(&dev, 0.6, 0.5, 0.0);
    assert_rel(op.gm, 1.70419811e-03, "vg0.5 gm(0.6)");
    assert_rel(op.gmbs, -6.45399696e-04, "vg0.5 gmbs(0.6)");
    let op = op_at(&dev, 1.1, 0.5, 0.0);
    assert_rel(op.vdsat, 1.21948090e-01, "vg0.5 vdsat(1.1)");
    assert_rel(op.csub, 1.33030640e-09, "vg0.5 isub(1.1)");

    // vgs=0.8, vbs=0 (nmos_idvd_vg800m_vb0).
    let table: [(Value, Value, Value); 3] = [
        (0.05, 1.80270519e-04, 3.26856982e-03),
        (0.55, 7.09764381e-04, 2.71993307e-04),
        (1.1, 8.49159220e-04, 2.62699651e-04),
    ];
    for &(vds, id, gds) in &table {
        let op = op_at(&dev, vds, 0.8, 0.0);
        assert_rel(op.cd, id, &format!("vg0.8 id(vds={vds})"));
        assert_rel(op.gds, gds, &format!("vg0.8 gds(vds={vds})"));
    }
    let op = op_at(&dev, 0.55, 0.8, 0.0);
    assert_rel(op.gm, 1.75716925e-03, "vg0.8 gm(0.55)");
    assert_rel(op.gmbs, -6.18437815e-04, "vg0.8 gmbs(0.55)");
    assert_rel(op.vdsat, 2.49306771e-01, "vg0.8 vdsat(0.55)");
    assert_rel(
        op_at(&dev, 1.1, 0.8, 0.0).csub,
        4.67711508e-10,
        "vg0.8 isub(1.1)",
    );

    // vgs=1.1, vbs=-0.45 (nmos_idvd_vg1100m_vbm045).
    let op = op_at(&dev, 0.6, 1.1, -0.45);
    assert_rel(op.cd, 1.60056174e-03, "vg1.1 id(0.6)");
    assert_rel(op.gds, 1.37048267e-03, "vg1.1 gds(0.6)");
    let op = op_at(&dev, 1.1, 1.1, -0.45);
    assert_rel(op.cd, 2.25187339e-03, "vg1.1 id(1.1)");
    assert_rel(op.gds, 1.30051995e-03, "vg1.1 gds(1.1)");
    assert_rel(op.gmbs, -1.94437507e-03, "vg1.1 gmbs(1.1)");
    assert_rel(op.vdsat, 4.93209795e-01, "vg1.1 vdsat(1.1)");
    assert_rel(op.csub, 5.15686982e-12, "vg1.1 isub(1.1)");
}

#[test]
fn ngspice_pinned_nmos_geometry_variants() {
    // Rows at (vds=0.05, vgs=0.9) and (vds=1.1, vgs=1.1) for the other
    // three deck geometries: m2 = 2u/0.2u, m3 = 1u/45n nf=4 (PAeffGeo
    // diffusions), m4 = 4u/1u.
    let m2 = nmos_device(2e-6, 0.2e-6, 1.0, T300);
    let op = op_at(&m2, 0.05, 0.9, 0.0);
    assert_rel(op.cd, 9.63138520e-05, "m2 lin id");
    assert_rel(op.gm, 1.51388172e-04, "m2 lin gm");
    assert_rel(op.gds, 1.78180914e-03, "m2 lin gds");
    assert_rel(op.gmbs, 7.14478375e-05, "m2 lin gmbs");
    let op = op_at(&m2, 1.1, 1.1, 0.0);
    assert_rel(op.cd, 7.29480675e-04, "m2 sat id");
    assert_rel(op.gm, 1.55044160e-03, "m2 sat gm");
    assert_rel(op.gds, 5.24445642e-05, "m2 sat gds");
    assert_rel(op.gmbs, 5.62222957e-04, "m2 sat gmbs");

    let m3 = nmos_device(1e-6, 45e-9, 4.0, T300);
    let op = op_at(&m3, 0.05, 0.9, 0.0);
    assert_rel(op.cd, 1.96889988e-04, "m3 lin id");
    assert_rel(op.gm, 1.59825444e-04, "m3 lin gm");
    assert_rel(op.gds, 3.63852589e-03, "m3 lin gds");
    assert_rel(op.gmbs, 3.38428801e-05, "m3 lin gmbs");
    let op = op_at(&m3, 1.1, 1.1, 0.0);
    assert_rel(op.cd, 1.40523471e-03, "m3 sat id");
    assert_rel(op.gm, 1.88614311e-03, "m3 sat gm");
    assert_rel(op.gds, 3.04728150e-04, "m3 sat gds");
    assert_rel(op.gmbs, -1.78288359e-03, "m3 sat gmbs");
    assert_rel(
        op_at(&m3, 0.05, 0.9, -0.9).cd,
        1.70665584e-04,
        "m3 id(vbs=-0.9)",
    );

    let m4 = nmos_device(4e-6, 1e-6, 1.0, T300);
    let op = op_at(&m4, 0.05, 0.9, 0.0);
    assert_rel(op.cd, 4.10171169e-05, "m4 lin id");
    assert_rel(op.gm, 6.94262578e-05, "m4 lin gm");
    assert_rel(op.gds, 7.62922032e-04, "m4 lin gds");
    assert_rel(op.gmbs, 3.41325298e-05, "m4 lin gmbs");
    let op = op_at(&m4, 1.1, 1.1, 0.0);
    assert_rel(op.cd, 3.26762653e-04, "m4 sat id");
    assert_rel(op.gm, 7.50560632e-04, "m4 sat gm");
    assert_rel(op.gds, 1.39084542e-05, "m4 sat gds");
    assert_rel(op.gmbs, 3.03053461e-04, "m4 sat gmbs");
}

#[test]
fn ngspice_pinned_nmos_temperature() {
    // m1 = 1u/45n at .temp 125 and .temp -40 (nmos_oracle_temp.sp /
    // nmos_oracle_tm40.sp); exercises tempMod=0 plus the diode/TAT
    // temperature chain (ibd carries the temp-scaled saturation current).
    let hot = nmos_device(1e-6, 45e-9, 1.0, 125.0 + 273.15);
    assert_rel(
        op_at(&hot, 0.05, 0.0, 0.0).cd,
        1.54225677e-08,
        "T125 leakage id",
    );
    let op = op_at(&hot, 0.05, 0.9, 0.0);
    assert_rel(op.cd, 1.75100485e-04, "T125 id");
    assert_rel(op.gm, 1.41093291e-04, "T125 gm");
    assert_rel(op.gds, 3.27180360e-03, "T125 gds");
    assert_rel(op.gmbs, 2.75781664e-05, "T125 gmbs");
    assert_rel(op.von, 3.33085360e-01, "T125 vth");
    assert_rel(op.cbd, -8.24653081e-14, "T125 ibd");
    let op = op_at(&hot, 1.1, 1.1, 0.0);
    assert_rel(op.cd, 1.37611607e-03, "T125 sat id");
    assert_rel(op.gm, 1.73509940e-03, "T125 sat gm");
    assert_rel(op.gds, 2.93735071e-04, "T125 sat gds");
    assert_rel(op.gmbs, -1.70440132e-03, "T125 sat gmbs");
    assert_rel(op.von, 2.52585567e-01, "T125 sat vth");
    assert_rel(op.vdsat, 4.46840617e-01, "T125 sat vdsat");
    assert_rel(op.csub, 1.26924564e-11, "T125 sat isub");
    let m3_hot = nmos_device(1e-6, 45e-9, 4.0, 125.0 + 273.15);
    assert_rel(
        op_at(&m3_hot, 1.1, 1.1, 0.0).cd,
        1.37410526e-03,
        "T125 m3 sat id",
    );

    let cold = nmos_device(1e-6, 45e-9, 1.0, -40.0 + 273.15);
    assert_rel(
        op_at(&cold, 0.05, 0.0, 0.0).cd,
        3.15304865e-12,
        "Tm40 leakage id",
    );
    let op = op_at(&cold, 0.05, 0.9, 0.0);
    assert_rel(op.cd, 2.17481206e-04, "Tm40 id");
    assert_rel(op.gm, 1.79190141e-04, "Tm40 gm");
    assert_rel(op.gds, 3.97173031e-03, "Tm40 gds");
    assert_rel(op.gmbs, 3.68166094e-05, "Tm40 gmbs");
    assert_rel(op.von, 4.39194452e-01, "Tm40 vth");
    assert_rel(op.cbd, -5.00000195e-14, "Tm40 ibd");
    let op = op_at(&cold, 1.1, 1.1, 0.0);
    assert_rel(op.cd, 1.42413165e-03, "Tm40 sat id");
    assert_rel(op.gm, 1.98495077e-03, "Tm40 sat gm");
    assert_rel(op.gds, 3.10565969e-04, "Tm40 sat gds");
    assert_rel(op.gmbs, -1.82829677e-03, "Tm40 sat gmbs");
    assert_rel(op.von, 3.60277676e-01, "Tm40 sat vth");
    assert_rel(op.vdsat, 3.54832273e-01, "Tm40 sat vdsat");
    assert_rel(op.csub, 1.30141955e-10, "Tm40 sat isub");
    let m3_cold = nmos_device(1e-6, 45e-9, 4.0, -40.0 + 273.15);
    assert_rel(
        op_at(&m3_cold, 1.1, 1.1, 0.0).cd,
        1.41930478e-03,
        "Tm40 m3 sat id",
    );
}

#[test]
fn ngspice_pinned_nmos_charges_capmod2() {
    // m1 = 1u/45n intrinsic charges + capacitance matrix as `@m1[qg]`/
    // `@m1[c**]` report them (here->BSIM4qgate/c***, b4ask.c), CAPMOD=2.
    let dev = nmos_device(1e-6, 45e-9, 1.0, T300);
    let charge_at = |vds: Value, vgs: Value| {
        dev.eval(Bsim4v8Bias { vds, vgs, vbs: 0.0 }, GMIN, true)
            .expect("charge eval")
            .charge
            .expect("charges")
    };

    // Linear, strong inversion (nmos_idvg_vd50m_vb0 at vgs=0.9).
    let c = charge_at(0.05, 0.9);
    assert_rel(c.qgate, 5.56112662e-16, "lin qg");
    assert_rel(c.qbulk, -3.17363989e-16, "lin qb");
    assert_rel(c.qdrn, -1.11984308e-16, "lin qd");
    assert_rel(c.cggb, 6.59136638e-16, "lin cgg");
    assert_rel(c.cgdb, -1.87972596e-16, "lin cgd");
    assert_rel(c.cgsb, -4.27469041e-16, "lin cgs");
    assert_rel(c.cdgb, -3.12752276e-16, "lin cdg");
    assert_rel(c.cddb, 4.63915823e-16, "lin cdd");
    assert_rel(c.cdsb, -1.03865067e-16, "lin cds");
    assert_rel(c.cbgb, -2.40833408e-17, "lin cbg");
    assert_rel(c.cbdb, -4.71918220e-16, "lin cbd");
    assert_rel(c.cbsb, 3.55843819e-16, "lin cbs");
    assert_rel(c.capbd, 3.92729344e-16, "lin capbd");
    assert_rel(c.capbs, 4.01240000e-16, "lin capbs");

    // Saturation (nmos_idvg_vd1100m_vb0 at vgs=1.1) with the derived
    // completion rows of the 4x4 matrix.
    let c = charge_at(1.1, 1.1);
    assert_rel(c.qgate, 6.99204478e-16, "sat qg");
    assert_rel(c.qbulk, -3.85615596e-16, "sat qb");
    assert_rel(c.qdrn, -1.25493936e-16, "sat qd");
    assert_rel(c.qsrc, -1.88094945e-16, "sat qs");
    assert_rel(c.cggb, 6.15702104e-16, "sat cgg");
    assert_rel(c.cgdb, 3.67808440e-17, "sat cgd");
    assert_rel(c.cgsb, -1.45454506e-16, "sat cgs");
    assert_rel(c.cdgb, -1.83546997e-16, "sat cdg");
    assert_rel(c.cddb, -1.36515831e-17, "sat cdd");
    assert_rel(c.cdsb, 5.38777248e-17, "sat cds");
    assert_rel(c.cbgb, -1.57142098e-16, "sat cbg");
    assert_rel(c.cbdb, -2.36179433e-18, "sat cbd");
    assert_rel(c.cbsb, 1.05350960e-17, "sat cbs");
    assert_rel(c.csgb, -2.75013009e-16, "sat csg");
    assert_rel(c.csdb, -2.07674666e-17, "sat csd");
    assert_rel(c.cssb, 8.10416850e-17, "sat css");
    assert_rel(c.cgbb, -5.07028442e-16, "sat cgb");
    assert_rel(c.cdbb, 1.43320855e-16, "sat cdb");
    assert_rel(c.csbb, 2.14738790e-16, "sat csb");
    assert_rel(c.cbbb, 1.48968797e-16, "sat cbb");

    // Subthreshold/depletion (vgs=0, vds=1.1): the gate/bulk pair carries
    // the charge, the drain partition is ~zero.
    let c = charge_at(1.1, 0.0);
    assert_rel(c.qgate, 2.24991507e-16, "dep qg");
    assert_rel(c.qbulk, -2.24987895e-16, "dep qb");
    assert_rel(c.qdrn, -1.44531650e-21, "dep qd");
    assert_rel(c.cggb, 1.55231298e-16, "dep cgg");
    assert_rel(c.cbgb, -1.55146657e-16, "dep cbg");

    // Id-Vd at vgs=0.8 across linear-to-saturation
    // (nmos_idvd_vg800m_vb0): capbd tracks the drain junction bias.
    let c = charge_at(0.05, 0.8);
    assert_rel(c.qgate, 4.90989796e-16, "vd50m qg");
    assert_rel(c.qbulk, -3.14408340e-16, "vd50m qb");
    assert_rel(c.qdrn, -8.15103680e-17, "vd50m qd");
    assert_rel(c.cggb, 6.42175595e-16, "vd50m cgg");
    assert_rel(c.cddb, 3.84859556e-16, "vd50m cdd");
    assert_rel(c.cbdb, -3.90351250e-16, "vd50m cbd");
    assert_rel(c.capbd, 3.92729344e-16, "vd50m capbd");
    let c = charge_at(0.55, 0.8);
    assert_rel(c.qgate, 4.97787297e-16, "vd550m qg");
    assert_rel(c.qdrn, -6.40628943e-17, "vd550m qd");
    assert_rel(c.cggb, 5.92253961e-16, "vd550m cgg");
    assert_rel(c.cddb, -1.24783324e-17, "vd550m cdd");
    assert_rel(c.capbd, 3.33456107e-16, "vd550m capbd");
}

#[test]
fn ngspice_pinned_nmos_charges_cvchargemod1() {
    // Same saturation bias and geometry as the CAPMOD=2 oracle above, with
    // only CVCHARGEMOD=1 added to the n45 model card. Reference generated
    // from local ngspice-46 (`ngspice_con.exe -b`) using `@m1[q*]` and
    // `@m1[c**]` probes.
    let mut card = nmos45();
    card.insert("CVCHARGEMOD".to_string(), 1.0);
    let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
    let dev = Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
        .expect("cvchargeMod=1 constructs");
    let charge = dev
        .eval(
            Bsim4v8Bias {
                vds: 1.1,
                vgs: 1.1,
                vbs: 0.0,
            },
            GMIN,
            true,
        )
        .expect("cvchargeMod=1 charge eval")
        .charge
        .expect("charges");

    assert_rel(charge.qgate, 7.23241861e-16, "cv1 qg");
    assert_rel(charge.qbulk, -3.86973253e-16, "cv1 qb");
    assert_rel(charge.qdrn, -1.34572090e-16, "cv1 qd");
    assert_rel(charge.qsrc, -2.01696518e-16, "cv1 qs");
    assert_rel(charge.cggb, 6.22177426e-16, "cv1 cgg");
    assert_rel(charge.cgdb, 3.70873112e-17, "cv1 cgd");
    assert_rel(charge.cgsb, -1.46331544e-16, "cv1 cgs");
    assert_rel(charge.cdgb, -1.85825441e-16, "cv1 cdg");
    assert_rel(charge.cddb, -1.37924792e-17, "cv1 cdd");
    assert_rel(charge.cdsb, 5.43320145e-17, "cv1 cds");
    assert_rel(charge.cbgb, -1.57943823e-16, "cv1 cbg");
    assert_rel(charge.cbdb, -2.27867934e-18, "cv1 cbd");
    assert_rel(charge.cbsb, 1.02431828e-17, "cv1 cbs");
}

#[test]
fn cvchargemod2_matches_ngspice_nonzero_charge_path() {
    // ngspice-46's BSIM4load distinguishes cvchargeMod == 0 from the
    // nonzero path; selector 2 therefore follows the same equations as 1.
    let charge_at = |selector: Value| {
        let mut card = nmos45();
        card.insert("CVCHARGEMOD".to_string(), selector);
        let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
        let dev = Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
            .unwrap_or_else(|err| panic!("CVCHARGEMOD={selector} constructs: {err}"));
        dev.eval(
            Bsim4v8Bias {
                vds: 1.1,
                vgs: 1.1,
                vbs: 0.0,
            },
            GMIN,
            true,
        )
        .unwrap_or_else(|err| panic!("CVCHARGEMOD={selector} charge eval: {err}"))
        .charge
        .expect("charges")
    };

    let cv1 = charge_at(1.0);
    let cv2 = charge_at(2.0);
    assert_rel(cv2.qgate, cv1.qgate, "cv2 qg");
    assert_rel(cv2.qbulk, cv1.qbulk, "cv2 qb");
    assert_rel(cv2.qdrn, cv1.qdrn, "cv2 qd");
    assert_rel(cv2.qsrc, cv1.qsrc, "cv2 qs");
    assert_rel(cv2.cggb, cv1.cggb, "cv2 cgg");
    assert_rel(cv2.cgdb, cv1.cgdb, "cv2 cgd");
    assert_rel(cv2.cgsb, cv1.cgsb, "cv2 cgs");
    assert_rel(cv2.cdgb, cv1.cdgb, "cv2 cdg");
    assert_rel(cv2.cddb, cv1.cddb, "cv2 cdd");
    assert_rel(cv2.cdsb, cv1.cdsb, "cv2 cds");
    assert_rel(cv2.cbgb, cv1.cbgb, "cv2 cbg");
    assert_rel(cv2.cbdb, cv1.cbdb, "cv2 cbd");
    assert_rel(cv2.cbsb, cv1.cbsb, "cv2 cbs");
}

#[test]
fn ngspice_pinned_nmos_charges_capmod1() {
    // Same saturation bias and geometry as the CAPMOD=2 oracle above, with
    // CAPMOD=1 selecting BSIM4's Meyer-like intrinsic charge model.
    // Reference generated from local ngspice-46 (`ngspice_con.exe -b`) using
    // `@m1[q*]` and `@m1[c**]` probes.
    let mut card = nmos45();
    card.insert("CAPMOD".to_string(), 1.0);
    let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
    let dev = Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
        .expect("CAPMOD=1 constructs");
    let charge = dev
        .eval(
            Bsim4v8Bias {
                vds: 1.1,
                vgs: 1.1,
                vbs: 0.0,
            },
            GMIN,
            true,
        )
        .expect("CAPMOD=1 charge eval")
        .charge
        .expect("charges");

    assert_rel(charge.qgate, 6.15439443e-16, "cap1 qg");
    assert_rel(charge.qbulk, -3.43898072e-16, "cap1 qb");
    assert_rel(charge.qdrn, -1.08669721e-16, "cap1 qd");
    assert_rel(charge.qsrc, -1.62871649e-16, "cap1 qs");
    assert_rel(charge.cggb, 4.97332878e-16, "cap1 cgg");
    assert_rel(charge.cgdb, 2.87802524e-17, "cap1 cgd");
    assert_rel(charge.cgsb, -1.14027307e-16, "cap1 cgs");
    assert_rel(charge.cdgb, -1.48217285e-16, "cap1 cdg");
    assert_rel(charge.cddb, -1.09920562e-17, "cap1 cdd");
    assert_rel(charge.cdsb, 4.34748979e-17, "cap1 cds");
    assert_rel(charge.cbgb, -1.27063074e-16, "cap1 cbg");
    assert_rel(charge.cbdb, -1.02835896e-18, "cap1 cbd");
    assert_rel(charge.cbsb, 5.12574604e-18, "cap1 cbs");
}

#[test]
fn ngspice_pinned_nmos_charges_capmod0() {
    // Same saturation bias and geometry as the CAPMOD=1/2 oracles above,
    // with CAPMOD=0 selecting BSIM4's older Meyer-like charge model and
    // linear overlap capacitances. Reference generated from local ngspice-46
    // (`ngspice_con.exe -b`) using `@m1[q*]` and `@m1[c**]` probes.
    let mut card = nmos45();
    card.insert("CAPMOD".to_string(), 0.0);
    let model = Arc::new(Bsim4v8Model::from_params(&card, false, T300));
    let dev = Bsim4v8::new("m1".to_string(), model, geom(1e-6, 45e-9, 1.0), T300)
        .expect("CAPMOD=0 constructs");
    let charge = dev
        .eval(
            Bsim4v8Bias {
                vds: 1.1,
                vgs: 1.1,
                vbs: 0.0,
            },
            GMIN,
            true,
        )
        .expect("CAPMOD=0 charge eval")
        .charge
        .expect("charges");

    assert_rel(charge.qgate, 6.24533876e-16, "cap0 qg");
    assert_rel(charge.qbulk, -3.50291923e-16, "cap0 qb");
    assert_rel(charge.qdrn, -1.09696781e-16, "cap0 qd");
    assert_rel(charge.qsrc, -1.64545172e-16, "cap0 qs");
    assert_rel(charge.cggb, 4.95309698e-16, "cap0 cgg");
    assert_rel(charge.cgdb, 0.0, "cap0 cgd");
    assert_rel(charge.cgsb, -4.81495299e-16, "cap0 cgs");
    assert_rel(charge.cdgb, -1.47997141e-16, "cap0 cdg");
    assert_rel(charge.cddb, 0.0, "cap0 cdd");
    assert_rel(charge.cdsb, 1.86287800e-16, "cap0 cds");
    assert_rel(charge.cbgb, -1.25316847e-16, "cap0 cbg");
    assert_rel(charge.cbdb, 0.0, "cap0 cbd");
    assert_rel(charge.cbsb, 1.57757996e-17, "cap0 cbs");
    assert_rel(charge.capbd, 2.95502205e-16, "cap0 capbd");
    assert_rel(charge.capbs, 4.01240000e-16, "cap0 capbs");
}

#[test]
fn ngspice_pinned_pmos() {
    // p90 m1 = 1u/90n; node-space biases (PMOS), internal polarity folded
    // by eval_polarity (pmos_oracle.sp sweeps). vth/vdsat/charges are the
    // internal-frame `here->BSIM4*` values, positive as ngspice prints them.
    let dev = pmos_device(1e-6, 90e-9, T300);
    let table: [(Value, Value, Value, Value, Value); 3] = [
        (
            -0.4,
            2.73567022e-06,
            3.47860239e-05,
            3.50738362e-05,
            7.33896156e-06,
        ),
        (
            -0.8,
            2.01011962e-05,
            3.56661498e-05,
            3.73779975e-04,
            1.10310990e-05,
        ),
        (
            -1.1,
            2.89375622e-05,
            2.41763513e-05,
            5.53359794e-04,
            1.02506581e-05,
        ),
    ];
    for &(vgs_node, id, gm, gds, gmbs) in &table {
        let op = dev
            .eval_polarity(-0.05, vgs_node, 0.0, GMIN, false)
            .expect("eval");
        assert_rel(op.cd, id, &format!("pmos id(vgs={vgs_node})"));
        assert_rel(op.gm, gm, &format!("pmos gm(vgs={vgs_node})"));
        assert_rel(op.gds, gds, &format!("pmos gds(vgs={vgs_node})"));
        assert_rel(op.gmbs, gmbs, &format!("pmos gmbs(vgs={vgs_node})"));
    }
    let op = dev
        .eval_polarity(-0.05, -0.8, 0.0, GMIN, true)
        .expect("eval");
    assert_rel(op.von, 3.57331527e-01, "pmos vth");
    assert_rel(op.vdsat, 3.72908219e-01, "pmos vdsat(-0.8)");
    assert_rel(op.cbd, -5.00003159e-14, "pmos ibd");
    let c = op.charge.expect("charges");
    assert_rel(c.qgate, 1.29938876e-15, "pmos qg");
    assert_rel(c.qbulk, -6.85141267e-16, "pmos qb");
    assert_rel(c.qdrn, -2.91905191e-16, "pmos qd");
    assert_rel(c.cggb, 1.60930964e-15, "pmos cgg");
    assert_rel(c.cgdb, -6.18695879e-16, "pmos cgd");
    assert_rel(c.cgsb, -9.54372079e-16, "pmos cgs");
    assert_rel(c.cdgb, -7.79581392e-16, "pmos cdg");
    assert_rel(c.cddb, 1.06011611e-15, "pmos cdd");
    assert_rel(c.cdsb, -1.26436629e-16, "pmos cds");
    assert_rel(c.cbgb, -3.62087121e-17, "pmos cbg");
    assert_rel(c.cbdb, -9.27844984e-16, "pmos cbd");
    assert_rel(c.cbsb, 6.13474377e-16, "pmos cbs");
    assert_rel(c.capbd, 4.93775177e-16, "pmos capbd");
    assert_rel(c.capbs, 5.04680000e-16, "pmos capbs");

    // Second geometry m2 = 2u/0.25u at vgs=-0.8.
    let m2 = {
        let model = Arc::new(Bsim4v8Model::from_params(&pmos90(), true, T300));
        Bsim4v8::new("m2".to_string(), model, geom(2e-6, 0.25e-6, 1.0), T300).expect("pmos m2")
    };
    let op = m2
        .eval_polarity(-0.05, -0.8, 0.0, GMIN, false)
        .expect("eval");
    assert_rel(op.cd, 1.54873022e-05, "pmos m2 id");
    assert_rel(op.gm, 2.91541495e-05, "pmos m2 gm");
    assert_rel(op.gds, 2.88223652e-04, "pmos m2 gds");
    assert_rel(op.gmbs, 9.60404821e-06, "pmos m2 gmbs");

    // Forward body bias vbs=+0.9 (node frame) at vgs=-1.1.
    let op = dev
        .eval_polarity(-0.05, -1.1, 0.9, GMIN, false)
        .expect("eval");
    assert_rel(op.cd, 2.18769310e-05, "pmos id(vbs=+0.9)");
    assert_rel(op.gm, 2.57849895e-05, "pmos gm(vbs=+0.9)");
    assert_rel(op.gds, 4.16132466e-04, "pmos gds(vbs=+0.9)");
    assert_rel(op.gmbs, 6.17421134e-06, "pmos gmbs(vbs=+0.9)");
    assert_rel(op.von, 5.05587021e-01, "pmos vth(vbs=+0.9)");
    assert_rel(op.vdsat, 5.19456781e-01, "pmos vdsat(vbs=+0.9)");

    // Saturation Id-Vg point (vds=-1.1).
    let op = dev
        .eval_polarity(-1.1, -1.1, 0.0, GMIN, false)
        .expect("eval");
    assert_rel(op.cd, 2.07956178e-04, "pmos sat id");
    assert_rel(op.gm, 3.69279162e-04, "pmos sat gm");
    assert_rel(op.gds, 4.75388615e-05, "pmos sat gds");
    assert_rel(op.gmbs, 8.97400018e-05, "pmos sat gmbs");
    assert_rel(op.von, 3.49387775e-01, "pmos sat vth");
    assert_rel(op.vdsat, 5.85213980e-01, "pmos sat vdsat");
    assert_rel(op.vgsteff, 7.50458863e-01, "pmos sat vgsteff");
    assert_rel(op.vdseff, 5.74289900e-01, "pmos sat vdseff");
    let op = m2
        .eval_polarity(-1.1, -1.1, 0.0, GMIN, false)
        .expect("eval");
    assert_rel(op.cd, 1.61082620e-04, "pmos m2 sat id");
    assert_rel(op.gds, 1.48834421e-05, "pmos m2 sat gds");

    // Id-Vd row at vds=-0.6 (pmos_idvd_vgm1100m_vb0) with charges.
    let op = dev
        .eval_polarity(-0.6, -1.1, 0.0, GMIN, true)
        .expect("eval");
    assert_rel(op.cd, 1.91001736e-04, "pmos idvd id");
    assert_rel(op.gm, 3.29293129e-04, "pmos idvd gm");
    assert_rel(op.gds, 7.02272806e-05, "pmos idvd gds");
    assert_rel(op.gmbs, 8.32418473e-05, "pmos idvd gmbs");
    assert_rel(op.vdsat, 5.82941887e-01, "pmos idvd vdsat");
    let c = op.charge.expect("charges");
    assert_rel(c.qgate, 1.67602355e-15, "pmos idvd qg");
    assert_rel(c.qbulk, -8.71646161e-16, "pmos idvd qb");
    assert_rel(c.qdrn, -3.22684859e-16, "pmos idvd qd");
    assert_rel(c.cggb, 1.45543507e-15, "pmos idvd cgg");
    assert_rel(c.cgdb, 3.64232786e-18, "pmos idvd cgd");
    assert_rel(c.cgsb, -1.40924578e-15, "pmos idvd cgs");
    assert_rel(c.cddb, 6.43493136e-18, "pmos idvd cdd");
    let op = m2
        .eval_polarity(-0.6, -1.1, 0.0, GMIN, false)
        .expect("eval");
    assert_rel(op.cd, 1.51688249e-04, "pmos m2 idvd id");
    assert_rel(op.gds, 5.19704692e-05, "pmos m2 idvd gds");
}

// ===================== live ngspice oracle =====================
//
// Set NGSPICE_EXE to a local ngspice-46 build and run
// `cargo test -p rspice-core --lib bsim4v8 -- --ignored --nocapture`.
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

    pub struct DeviceSpec {
        pub name: &'static str,
        pub w: Value,
        pub l: Value,
        pub nf: Value,
        /// Explicit AD/AS/PD/PS, or None for the BSIM4PAeffGeo diffusions.
        pub diff: Option<(Value, Value, Value, Value)>,
    }

    pub struct DeckSpec {
        pub deck: &'static str,
        pub temp_c: Value,
        pub is_pmos: bool,
        pub devices: &'static [DeviceSpec],
        pub sweeps: &'static [Sweep],
    }

    pub fn deck_specs() -> Vec<DeckSpec> {
        vec![
            DeckSpec {
                deck: "nmos_oracle.sp",
                temp_c: 27.0,
                is_pmos: false,
                devices: &[
                    DeviceSpec {
                        name: "m1",
                        w: 1e-6,
                        l: 45e-9,
                        nf: 1.0,
                        diff: Some((0.1e-12, 0.1e-12, 2.2e-6, 2.2e-6)),
                    },
                    DeviceSpec {
                        name: "m2",
                        w: 2e-6,
                        l: 0.2e-6,
                        nf: 1.0,
                        diff: Some((0.2e-12, 0.2e-12, 4.2e-6, 4.2e-6)),
                    },
                    DeviceSpec {
                        name: "m3",
                        w: 1e-6,
                        l: 45e-9,
                        nf: 4.0,
                        diff: None,
                    },
                    DeviceSpec {
                        name: "m4",
                        w: 4e-6,
                        l: 1e-6,
                        nf: 1.0,
                        diff: Some((0.4e-12, 0.4e-12, 8.2e-6, 8.2e-6)),
                    },
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
                        file: "nmos_idvg_vd1100m_vb0",
                        sweep_is_vg: true,
                        vd: 1.1,
                        vg: 0.0,
                        vb: 0.0,
                    },
                    Sweep {
                        file: "nmos_idvg_vd1100m_vbm09",
                        sweep_is_vg: true,
                        vd: 1.1,
                        vg: 0.0,
                        vb: -0.9,
                    },
                    Sweep {
                        file: "nmos_idvd_vg500m_vb0",
                        sweep_is_vg: false,
                        vd: 0.0,
                        vg: 0.5,
                        vb: 0.0,
                    },
                    Sweep {
                        file: "nmos_idvd_vg800m_vb0",
                        sweep_is_vg: false,
                        vd: 0.0,
                        vg: 0.8,
                        vb: 0.0,
                    },
                    Sweep {
                        file: "nmos_idvd_vg1100m_vbm045",
                        sweep_is_vg: false,
                        vd: 0.0,
                        vg: 1.1,
                        vb: -0.45,
                    },
                ],
            },
            DeckSpec {
                deck: "nmos_oracle_temp.sp",
                temp_c: 125.0,
                is_pmos: false,
                devices: &[
                    DeviceSpec {
                        name: "m1",
                        w: 1e-6,
                        l: 45e-9,
                        nf: 1.0,
                        diff: Some((0.1e-12, 0.1e-12, 2.2e-6, 2.2e-6)),
                    },
                    DeviceSpec {
                        name: "m3",
                        w: 1e-6,
                        l: 45e-9,
                        nf: 4.0,
                        diff: None,
                    },
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
                        file: "nmos_idvg_vd1100m_t125",
                        sweep_is_vg: true,
                        vd: 1.1,
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
                    DeviceSpec {
                        name: "m1",
                        w: 1e-6,
                        l: 45e-9,
                        nf: 1.0,
                        diff: Some((0.1e-12, 0.1e-12, 2.2e-6, 2.2e-6)),
                    },
                    DeviceSpec {
                        name: "m3",
                        w: 1e-6,
                        l: 45e-9,
                        nf: 4.0,
                        diff: None,
                    },
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
                        file: "nmos_idvg_vd1100m_tm40",
                        sweep_is_vg: true,
                        vd: 1.1,
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
                    DeviceSpec {
                        name: "m1",
                        w: 1e-6,
                        l: 90e-9,
                        nf: 1.0,
                        diff: Some((0.1e-12, 0.1e-12, 2.2e-6, 2.2e-6)),
                    },
                    DeviceSpec {
                        name: "m2",
                        w: 2e-6,
                        l: 0.25e-6,
                        nf: 1.0,
                        diff: Some((0.2e-12, 0.2e-12, 4.2e-6, 4.2e-6)),
                    },
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
                        file: "pmos_idvg_vdm1100m_vb0",
                        sweep_is_vg: true,
                        vd: -1.1,
                        vg: 0.0,
                        vb: 0.0,
                    },
                    Sweep {
                        file: "pmos_idvd_vgm1100m_vb0",
                        sweep_is_vg: false,
                        vd: 0.0,
                        vg: -1.1,
                        vb: 0.0,
                    },
                ],
            },
        ]
    }

    pub fn build_device(spec: &DeviceSpec, is_pmos: bool, temp_k: Value) -> Bsim4v8 {
        let card = if is_pmos { pmos90() } else { nmos45() };
        let model = Arc::new(Bsim4v8Model::from_params(&card, is_pmos, T300));
        let mut geom = Bsim4v8Geometry {
            l: spec.l,
            w: spec.w,
            nf: spec.nf,
            ..Bsim4v8Geometry::default()
        };
        if let Some((ad, asr, pd, ps)) = spec.diff {
            geom.drain_area = ad;
            geom.drain_area_given = true;
            geom.source_area = asr;
            geom.source_area_given = true;
            geom.drain_perimeter = pd;
            geom.drain_perimeter_given = true;
            geom.source_perimeter = ps;
            geom.source_perimeter_given = true;
        }
        Bsim4v8::new(spec.name.to_string(), model, geom, temp_k).expect("oracle device")
    }

    pub fn testdata_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/device/mosfet/bsim4v8/testdata")
    }

    pub fn run_deck(exe: &Path, deck: &str) -> PathBuf {
        let work = std::env::temp_dir().join(format!(
            "rspice-b4v8-oracle-{}-{}",
            std::process::id(),
            deck.trim_end_matches(".sp")
        ));
        let _ = fs::remove_dir_all(&work);
        fs::create_dir_all(&work).expect("create oracle work dir");
        for f in ["models45.lib", deck] {
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

    /// Map an oracle column quantity onto the port's op/charge state. The
    /// `@m[q*]`/`@m[c**]` probes report the intrinsic `here->BSIM4*` values
    /// (b4ask.c), not the CKTstate compositions.
    pub fn extract(op: &Bsim4v8Op, quantity: &str) -> Value {
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
            "isub" => op.csub,
            "igidl" => op.igidl,
            "igisl" => op.igisl,
            "vgsteff" => op.vgsteff,
            "vdseff" => op.vdseff,
            "qg" => c().qgate,
            "qb" => c().qbulk,
            "qd" => c().qdrn,
            "qs" => c().qsrc,
            "cgg" => c().cggb,
            "cgd" => c().cgdb,
            "cgs" => c().cgsb,
            "cdg" => c().cdgb,
            "cdd" => c().cddb,
            "cds" => c().cdsb,
            "cbg" => c().cbgb,
            "cbd" => c().cbdb,
            "cbs" => c().cbsb,
            "csg" => c().csgb,
            "csd" => c().csdb,
            "css" => c().cssb,
            "cgb" => c().cgbb,
            "cdb" => c().cdbb,
            "csb" => c().csbb,
            "cbb" => c().cbbb,
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
        eprintln!("Skipping: set NGSPICE_EXE to run the live BSIM4 oracle.");
        return;
    };

    let mut points = 0usize;
    let mut worst: (Value, String) = (0.0, String::new());
    for spec in oracle::deck_specs() {
        let work = oracle::run_deck(&exe, spec.deck);
        let temp_k = spec.temp_c + 273.15;

        let devices: HashMap<String, Bsim4v8> = spec
            .devices
            .iter()
            .map(|d| {
                (
                    d.name.to_string(),
                    oracle::build_device(d, spec.is_pmos, temp_k),
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
                let mut ops: HashMap<&str, Bsim4v8Op> = HashMap::new();
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
        "live BSIM4 oracle: {points} comparisons passed; worst rel err {:.3e} ({})",
        worst.0, worst.1
    );
}
// ===================== Verilog-A cross-check =====================
//
// An optional Verilog-A BSIM4.8 source, when supplied locally, is a sibling
// implementation rather than the transcription source for this native port.
// This cross-check reports native-vs-VA deltas at the same fixture biases
// without forcing agreement; the hard bound only guards against gross
// divergence.

#[cfg(feature = "veriloga")]
mod veriloga_cross_check {
    use super::*;
    use std::path::{Path, PathBuf};

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
        let netlist = crate::Netlist::parse(&deck).expect("parse");
        let result = crate::Engine::default()
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
    /// Run with `cargo test -p rspice-core --lib bsim4v8 --features veriloga
    /// -- --ignored --nocapture` to see the delta table.
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
