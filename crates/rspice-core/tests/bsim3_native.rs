//! Engine-level validation of the native BSIM3v3.3 (MOS LEVEL=8/9/49) wiring.
//!
//! The model module (`device/mosfet/bsim3v3`) is pinned against ngspice-46
//! standalone; these tests prove the *engine* reproduces those values
//! through its own Newton solve, DC sweep, and transient integration:
//!
//! - a single-NMOS `.op` must hit the module's pinned oracle table;
//! - a CMOS inverter VTC `.dc` sweep is compared point-by-point against an
//!   ngspice-46 run of the same deck;
//! - a 3-stage ring oscillator `.tran` must oscillate with the period
//!   ngspice produces for the same deck (5% tolerance);
//! - LEVEL=49 and Xyce-compatible LEVEL=9 cards build and run natively, without the
//!   `allow_simplified_mos` escape hatch.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

/// The module's own oracle model cards (n018/p018, LEVEL=49, CAPMOD=3).
fn models018() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/device/mosfet/bsim3v3/testdata/models018.lib"
    );
    std::fs::read_to_string(path).expect("read models018.lib")
}

fn models018_capmod2(xpart: f64) -> String {
    models018()
        .replace("capmod=3", "capmod=2")
        .replace("xpart=0.5", &format!("xpart={xpart}"))
}

fn models018_capmod(capmod: i32) -> String {
    models018().replace("capmod=3", &format!("capmod={capmod}"))
}

fn models018_acnqsmod1() -> String {
    models018().replace("capmod=3 xpart=0.5", "capmod=3 xpart=0.5 acnqsmod=1")
}

fn models018_acm1() -> String {
    models018().replace(
        ".model n018 nmos level=49",
        ".model n018 nmos level=49 acm=1 hdif=0.2u wmlt=1.1 xw=0.02u",
    )
}

fn models018_acm1_rseries() -> String {
    models018()
        .replace(
            ".model n018 nmos level=49",
            ".model n018 nmos level=49 acm=1 hdif=0.2u wmlt=1.2 xw=0.05u ld=0.015u ldif=0.025u rd=400 rs=300 rdc=25 rsc=15",
        )
        .replace("rsh=0 mobmod=1", "rsh=60 mobmod=1")
}

fn models018_nqsmod(nqs_mod: i32) -> String {
    models018().replace(
        "capmod=3 xpart=0.5",
        &format!("capmod=3 xpart=0.5 nqsmod={nqs_mod} acnqsmod=0"),
    )
}

fn models018_invalid_selector(selector: &str, value: &str) -> String {
    match selector {
        "MOBMOD" => models018().replace("rsh=0 mobmod=1", &format!("rsh=0 mobmod={value}")),
        "CAPMOD" => models018().replace("capmod=3 xpart=0.5", &format!("capmod={value} xpart=0.5")),
        "NQSMOD" | "ACNQSMOD" => models018().replace(
            "capmod=3 xpart=0.5",
            &format!(
                "capmod=3 xpart=0.5 {}={value}",
                selector.to_ascii_lowercase()
            ),
        ),
        _ => models018().replace(
            ".model n018 nmos level=49",
            &format!(
                ".model n018 nmos level=49 {}={value}",
                selector.to_ascii_lowercase()
            ),
        ),
    }
}

fn models018_level9_acnqsmod1() -> String {
    models018_acnqsmod1().replace("level=49", "level=9 version=3.2.2")
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn bsim3_nmos_op_id_with_models(models: &str, instance_suffix: &str) -> Result<f64, String> {
    let deck = format!(
        "* bsim3 native selector op\n\
         vd d 0 dc 1.2\n\
         vg g 0 dc 1.2\n\
         m1 d g 0 0 n018 w=10u l=0.18u ad=4.2p as=4.2p pd=20.84u ps=20.84u nrd=0 nrs=0 {instance_suffix}\n\
         {models}\n\
         .op\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("BSIM3 op deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .map_err(|error| error.to_string())?;
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    Ok(entry
        .params
        .iter()
        .find(|(key, _)| *key == "id")
        .map(|(_, value)| *value)
        .expect("m1 id op param"))
}

fn bsim3_nmos_op_id(instance_suffix: &str) -> Result<f64, String> {
    bsim3_nmos_op_id_with_models(&models018(), instance_suffix)
}

#[test]
fn native_bsim3_rejects_unresolved_model_params_before_defaulting() {
    let models = models018().replace(
        ".model n018 nmos level=49",
        ".model n018 nmos level=49 nqsmod={native_nqs}",
    );
    let deck = format!(
        "* bsim3 unresolved native model param policy\n\
         vd d 0 dc 1.2\n\
         vg g 0 dc 1.2\n\
         m1 d g 0 0 n018 w=10u l=0.18u\n\
         {models}\n\
         .op\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let message = engine()
        .run_dc_op_with_report(&netlist)
        .expect_err("unresolved BSIM3 model parameter must not fall back to the default")
        .to_string();

    assert!(
        message.contains("BSIM3") && message.contains("NQSMOD"),
        "error should identify the unresolved native BSIM3 model parameter: {message}"
    );
    assert!(
        message.contains("unresolved") && message.contains("finite numeric literal"),
        "error should explain native BSIM3 params must be numeric: {message}"
    );
}

#[test]
fn native_bsim3_rejects_invalid_integer_model_selectors_without_defaulting() {
    for (selector, value) in [
        ("MOBMOD", "1.9"),
        ("CAPMOD", "3.9"),
        ("NOIMOD", "7"),
        ("NOIMOD", "2.9"),
        ("BINUNIT", "1.5"),
        ("PARAMCHK", "0.9"),
        ("CALCACM", "1.5"),
    ] {
        let models = models018_invalid_selector(selector, value);
        let deck = format!(
            "* bsim3 invalid integer selector policy\n\
             vd d 0 dc 1.2\n\
             vg g 0 dc 1.2\n\
             m1 d g 0 0 n018 w=10u l=0.18u\n\
             {models}\n\
             .op\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let message = match engine().run_dc_op_with_report(&netlist) {
            Ok(_) => panic!("{selector}={value} must not be accepted"),
            Err(error) => error.to_string(),
        };

        assert!(
            message.contains("BSIM3") && message.contains(selector),
            "error should identify the invalid native BSIM3 selector {selector}={value}: {message}"
        );
        assert!(
            message.contains("finite integer"),
            "error should explain native BSIM3 selectors must be finite integers: {message}"
        );
    }
}

#[test]
fn native_bsim3_out_of_range_nqs_selectors_reset_to_qs_like_ngspice46() {
    let base_id = bsim3_nmos_op_id_with_models(&models018(), "").expect("BSIM3 baseline op");

    for (selector, value) in [
        ("NQSMOD", "2"),
        ("NQSMOD", "-1"),
        ("ACNQSMOD", "2"),
        ("ACNQSMOD", "-1"),
    ] {
        let models = models018_invalid_selector(selector, value);
        let id = bsim3_nmos_op_id_with_models(&models, "")
            .unwrap_or_else(|err| panic!("{selector}={value} should reset like ngspice: {err}"));
        let rel = (id - base_id).abs() / base_id.abs().max(1e-30);

        assert!(
            rel < 1e-12,
            "BSIM3 {selector}={value} must reset to QS baseline like ngspice-46: id={id:.9e}, base={base_id:.9e}, rel={rel:.3e}"
        );
    }

    let rel = (base_id - 2.389567960e-03).abs() / 2.389567960e-03;
    assert!(
        rel < 1e-6,
        "BSIM3 QS baseline drifted from the ngspice-46 oracle: id={base_id:.9e}, rel={rel:.3e}"
    );
}

#[test]
fn native_bsim3_rejects_nonfinite_model_selectors_before_defaulting() {
    for selector in ["MOBMOD", "CAPMOD", "NQSMOD", "NOIMOD"] {
        let models = models018_invalid_selector(selector, "1e309");
        let deck = format!(
            "* bsim3 non-finite selector policy\n\
             vd d 0 dc 1.2\n\
             vg g 0 dc 1.2\n\
             m1 d g 0 0 n018 w=10u l=0.18u\n\
             {models}\n\
             .op\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let message = match engine().run_dc_op_with_report(&netlist) {
            Ok(_) => panic!("{selector}=1e309 must not be accepted"),
            Err(error) => error.to_string(),
        };

        assert!(
            message.contains("BSIM3") && message.contains(selector),
            "error should identify the non-finite native BSIM3 selector {selector}=1e309: {message}"
        );
        assert!(
            message.contains("finite numeric literal"),
            "error should explain native BSIM3 selector values must be finite numeric literals: {message}"
        );
    }
}

#[test]
fn native_bsim3_mult_alias_matches_m_multiplier() {
    let base_id = bsim3_nmos_op_id("").expect("BSIM3 default multiplier op converges");
    let m_id = bsim3_nmos_op_id("m=3").expect("BSIM3 M=3 op converges");
    let mult_id = bsim3_nmos_op_id("mult=3").expect("BSIM3 MULT=3 op converges");
    let rel = (mult_id - m_id).abs() / m_id.abs().max(1e-30);
    let m_ratio = (m_id - 3.0 * base_id).abs() / m_id.abs().max(1e-30);
    let mult_ratio = (mult_id - 3.0 * base_id).abs() / mult_id.abs().max(1e-30);

    assert!(
        rel < 1e-12,
        "BSIM3 MULT=3 must match M=3: MULT id={mult_id:.9e}, M id={m_id:.9e}, rel={rel:.3e}"
    );
    assert!(
        m_ratio < 1e-12 && mult_ratio < 1e-12,
        "BSIM3 M/MULT=3 must scale default current by 3: base={base_id:.9e}, M={m_id:.9e}, MULT={mult_id:.9e}"
    );
}

#[test]
fn native_bsim3_rejects_invalid_multiplicity_aliases() {
    for suffix in [
        "M=0",
        "MULT=0",
        "M=-1",
        "MULT=-1",
        "M=3 MULT=0",
        "MULT=3 M=0",
    ] {
        let message =
            bsim3_nmos_op_id(suffix).expect_err("invalid BSIM3 multiplicity must fail closed");
        assert!(
            message.contains("BSIM3") && message.contains("finite"),
            "unexpected invalid BSIM3 {suffix} error: {message}"
        );
    }
}

#[test]
fn single_nmos_op_matches_module_oracle() {
    // m1 = 10u/0.18u at vds=1.2, vgs=1.2, vbs=0, T=27C: the exact bias of
    // the module's ngspice_pinned_nmos_idvg_saturation row. The engine must
    // reproduce what the module produced standalone — that proves the
    // builder/stamp/solve wiring, not just the device math.
    let deck = format!(
        "* bsim3 native op\n\
         vd d 0 dc 1.2\n\
         vg g 0 dc 1.2\n\
         m1 d g 0 0 n018 w=10u l=0.18u ad=4.2p as=4.2p pd=20.84u ps=20.84u nrd=0 nrs=0\n\
         {}\n\
         .op\n\
         .end\n",
        models018()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("op converges");
    let entry = report
        .entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(
        entry.device_kind, "BSIM3",
        "native port, not an approximation"
    );
    assert_eq!(entry.region, Some("saturation"));
    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| panic!("missing op param {key}"))
    };
    // ngspice-46 references (module tests.rs, 9 significant digits). The
    // module matches them at <= 4.9e-9 relative; through the engine solve
    // the only extra error is the Newton stopping tolerance on the
    // (source-driven, exactly-biased) terminals — keep 1e-6.
    let assert_rel = |what: &str, ours: f64, reference: f64| {
        let rel = (ours - reference).abs() / reference.abs();
        assert!(
            rel < 1e-6,
            "{what}: engine={ours:.9e} oracle={reference:.9e} rel={rel:.2e}"
        );
    };
    assert_rel("id", get("id"), 2.389567960e-03);
    assert_rel("gm", get("gm"), 4.543583450e-03);
    assert_rel("gds", get("gds"), 2.234402370e-04);
    assert_rel("gmb", get("gmb"), 1.134341020e-03);
    assert_rel("vth", get("vth"), 4.967558170e-01);
    assert_rel("vdsat", get("vdsat"), 3.733014340e-01);
}

#[test]
fn level9_default_bsim3_current_matches_xyce710() {
    // Xyce 7.10 registers MOSFET_B3 as LEVEL=9/49 and defaults VERSION to
    // 3.2.2. This one-point deck pins the LEVEL=9 compatibility front against
    // Xyce's DC current while still using RSpice's native BSIM3 evaluator.
    let deck = "* bsim3 level9 xyce oracle\n\
                vd d 0 dc 1.2\n\
                vg g 0 dc 1.2\n\
                m1 d g 0 0 n9 w=10u l=0.18u ad=4.2p as=4.2p pd=20.84u ps=20.84u nrd=0 nrs=0\n\
                .model n9 nmos level=9 version=3.2.2 tox=4.1n nch=2.35e17 vth0=0.5 capmod=3\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("LEVEL=9 deck runs natively");
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(
        entry.device_kind, "BSIM3",
        "LEVEL=9 must use the native BSIM3 port, not a simplified fallback"
    );
    let id = entry
        .params
        .iter()
        .find(|(key, _)| *key == "id")
        .map(|(_, value)| *value)
        .expect("BSIM3 op id");

    // XyceNF 7.10.0, same deck with `.print dc I(Vd)`: I(Vd) =
    // -1.29424026e-03 A, so drain current into the MOSFET is positive.
    let reference = 1.29424026e-03;
    let rel = (id - reference).abs() / reference;
    assert!(
        rel < 1e-6,
        "LEVEL=9 id: RSpice={id:.9e} Xyce7.10={reference:.9e} rel={rel:.2e}"
    );
}

#[test]
fn level49_version324_fails_closed_until_native_bsim3v32_port_exists() {
    let deck = "* BSIM3v32 NMOS DC Operating Point\n\
                V1 drain 0 1.0\n\
                V2 gate 0 1.0\n\
                M1 drain gate 0 0 NMOD W=1u L=100n\n\
                .model NMOD NMOS LEVEL=49 VERSION=3.24 TNOM=27\n\
                + TOX=9E-9 VTH0=0.5 K1=0.53 K2=-0.03\n\
                + NCH=2.3E17 NSUB=1E16\n\
                + U0=300 VSAT=1.5E5\n\
                + UA=-1.4E-9 UB=2.3E-18 UC=-4.6E-11\n\
                + RDSW=200 PRWG=0.5 PRWB=0.2\n\
                + WR=1 WINT=5E-9 LINT=2E-8\n\
                + DWG=-2E-8 DWB=1E-8\n\
                + VOFF=-0.1 NFACTOR=1.5 ETA0=0.08 ETAB=-0.07\n\
                + DSUB=0.56 PCLM=1.3 PDIBLC1=0.39 PDIBLC2=0.0086\n\
                + DROUT=0.56 PSCBE1=4.24E8 PSCBE2=1E-5 PVAG=0.1\n\
                + DELTA=0.01 ALPHA0=0 BETA0=30\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("BSIM3v32 deck parses");
    let message = engine()
        .run_dc_op_with_report(&netlist)
        .expect_err("LEVEL=49 VERSION=3.24 must not masquerade as BSIM3v3.3")
        .to_string();
    assert!(
        message.contains("BSIM3v32")
            && message.contains("LEVEL=49")
            && message.contains("VERSION=3.24")
            && message.contains("distinct native BSIM3v32 port"),
        "BSIM3v32 fail-closed error should identify the unsupported native port: {message}"
    );
    // Future native BSIM3v32 port target for this deck:
    // ngspice 46 reports id=2.36367e-04, gm=4.87393e-04,
    // gds=1.67152e-04, vth=0.456369, vdsat=0.296196.
}

#[test]
fn unsupported_pre33_bsim3_version_still_fails_closed() {
    let deck = "* unsupported BSIM3 pre-3.3 version\n\
                V1 drain 0 1.0\n\
                V2 gate 0 1.0\n\
                M1 drain gate 0 0 NMOD W=1u L=100n\n\
                .model NMOD NMOS LEVEL=49 VERSION=3.1 TOX=9E-9 VTH0=0.5\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("BSIM3 deck parses");
    let message = engine()
        .run_dc_op_with_report(&netlist)
        .expect_err("unsupported pre-3.3 BSIM3 version fails closed")
        .to_string();
    assert!(
        message.contains("unsupported BSIM3 pre-3.3")
            && message.contains("LEVEL=49")
            && message.contains("VERSION=3.1"),
        "unsupported version diagnostic should identify the rejected card: {message}"
    );
}

/// The inverter used by the VTC and ring tests: 2u/0.18u PMOS over
/// 1u/0.18u NMOS on a 1.8 V rail, junction geometry spelled out.
fn inverter_pair(name: &str, input: &str, output: &str) -> String {
    format!(
        "mp{name} {output} {input} vdd vdd p018 w=2u l=0.18u ad=0.84p as=0.84p pd=4.84u ps=4.84u nrd=0 nrs=0\n\
         mn{name} {output} {input} 0 0 n018 w=1u l=0.18u ad=0.42p as=0.42p pd=2.84u ps=2.84u nrd=0 nrs=0\n"
    )
}

fn nqsmod_common_source_deck(nqs_mod: i32) -> String {
    format!(
        "* bsim3 nqsmod={nqs_mod} common source transient\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12 method=gear maxord=2\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 pulse(0.3 1.2 20p 2p 2p 80p 200p)\n\
         vb b 0 dc 0\n\
         rd vdd out 4k\n\
         m1 out in 0 b n018 w=1u l=0.18u ad=0.42p as=0.42p pd=2.84u ps=2.84u nrd=0 nrs=0\n\
         {}\n\
         .tran 0.1p 80p\n\
         .end\n",
        models018_nqsmod(nqs_mod)
    )
}

fn nqsmod_common_source_instance_deck(model_nqs_mod: i32, instance_suffix: &str) -> String {
    format!(
        "* bsim3 instance nqs common source transient\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12 method=gear maxord=2\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 pulse(0.3 1.2 20p 2p 2p 80p 200p)\n\
         vb b 0 dc 0\n\
         rd vdd out 4k\n\
         m1 out in 0 b n018 w=1u l=0.18u ad=0.42p as=0.42p pd=2.84u ps=2.84u nrd=0 nrs=0 {instance_suffix}\n\
         {}\n\
         .tran 0.1p 80p\n\
         .end\n",
        models018_nqsmod(model_nqs_mod)
    )
}

#[test]
fn cmos_inverter_vtc_matches_ngspice() {
    let deck = format!(
        "* bsim3 inverter vtc\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 dc 0\n\
         {}\
         {}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models018()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let results = engine()
        .run_dc_sweep(&netlist, "vin", 0.0, 1.8, 0.1)
        .expect("vtc sweep converges");

    // ngspice-46 reference: `ngspice -b` on this same deck (models018.lib,
    // `dc vin 0 1.8 0.1` with `.option reltol=1e-6`, wrdata v(out)), run
    // 2026-06-12 from the local ngspice-46 source build. The tightened
    // reltol matters: at its default 1e-3, ngspice's sweep-continuation
    // Newton leaves a step-size-dependent residue of up to ~1.4 mV on the
    // low-gain shoulders (its own standalone `.op` at vin=0.7 agrees with
    // the tight sweep, not the loose one).
    let reference: &[(f64, f64)] = &[
        (0.3, 1.79997200e+00),
        (0.5, 1.79598045e+00),
        (0.7, 1.71660112e+00),
        (0.8, 1.59036404e+00),
        (0.9, 6.77698565e-01), // steepest point of the transition
        (1.0, 1.30887788e-01),
        (1.2, 2.55494996e-02),
        (1.5, 1.47447798e-04),
        (1.8, 7.02511507e-08),
    ];
    for &(vin, vout_ref) in reference {
        let (_, result) = results
            .iter()
            .find(|(v, _)| (v - vin).abs() < 1e-9)
            .unwrap_or_else(|| panic!("sweep point vin={vin} present"));
        let node = result.node_index_named("out").expect("node out in result");
        let vout = result.voltage(node);
        // The device math matches ngspice to ~5e-9; the budget here is the
        // two solvers' Newton stopping criteria. The high-gain transition
        // point is input-error amplified, so it gets a wider (still
        // sub-mV) gate.
        let tol = if (vin - 0.9).abs() < 1e-9 {
            5e-4
        } else {
            1e-5 * vout_ref.abs() + 1e-6
        };
        println!(
            "VTC vin={vin:.1}: engine={vout:.9e} ngspice={vout_ref:.9e} delta={:.2e}",
            (vout - vout_ref).abs()
        );
        assert!(
            (vout - vout_ref).abs() < tol,
            "VTC at vin={vin}: engine={vout:.9e} ngspice={vout_ref:.9e} (tol {tol:.1e})"
        );
    }
}

/// Rising-edge crossing times of `threshold`, linearly interpolated.
fn rising_crossings(time: &[f64], wave: &[f64], threshold: f64) -> Vec<f64> {
    let mut crossings = Vec::new();
    for i in 1..time.len() {
        if wave[i - 1] < threshold && wave[i] >= threshold {
            let f = (threshold - wave[i - 1]) / (wave[i] - wave[i - 1]);
            crossings.push(time[i - 1] + f * (time[i] - time[i - 1]));
        }
    }
    crossings
}

fn interp_waveform(time: &[f64], wave: &[f64], target: f64) -> f64 {
    assert_eq!(time.len(), wave.len(), "time/waveform length mismatch");
    if target <= time[0] {
        return wave[0];
    }
    for i in 1..time.len() {
        if time[i] >= target {
            let f = (target - time[i - 1]) / (time[i] - time[i - 1]);
            return wave[i - 1] + f * (wave[i] - wave[i - 1]);
        }
    }
    *wave.last().expect("nonempty waveform")
}

#[test]
fn ring_oscillator_period_matches_ngspice() {
    // 3-stage ring, CAPMOD=3 intrinsic + overlap + junction charges as the
    // only load. `.ic v(n1)=0` kicks it off the metastable rail.
    let deck = format!(
        "* bsim3 ring oscillator\n\
         vdd vdd 0 dc 1.8\n\
         {}\
         {}\
         {}\
         .ic v(n1)=0\n\
         {}\n\
         .tran 0.5p 4n\n\
         .end\n",
        inverter_pair("1", "n1", "n2"),
        inverter_pair("2", "n2", "n3"),
        inverter_pair("3", "n3", "n1"),
        models018()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 4e-9, 2e-12)
        .expect("ring transient runs");
    let wave = result
        .try_voltage_waveform_named("n1")
        .expect("n1 waveform");
    let crossings = rising_crossings(&result.time, wave, 0.9);
    assert!(
        crossings.len() >= 8,
        "ring must oscillate: only {} rising crossings of 0.9 V in 4 ns",
        crossings.len()
    );
    let periods: Vec<f64> = crossings.windows(2).map(|w| w[1] - w[0]).collect();
    let tail = &periods[periods.len().saturating_sub(5)..];
    let period = tail.iter().sum::<f64>() / tail.len() as f64;

    // ngspice-46 reference: same deck (`.tran 0.5p 4n`, `.ic v(n1)=0`),
    // run 2026-06-12; the period settles to 2.2217e-10 s (4.50 GHz) with
    // <0.01% cycle-to-cycle spread after the first cycle.
    let reference = 2.2217e-10;
    let rel = (period - reference).abs() / reference;
    println!(
        "ring: {} crossings, period engine={period:.6e} ngspice={reference:.4e} rel={rel:.4}",
        crossings.len()
    );
    assert!(
        rel < 0.05,
        "ring period: engine={period:.4e} ngspice={reference:.4e} rel={rel:.3}"
    );
}

#[test]
fn nqsmod1_common_source_transient_matches_ngspice46() {
    // ngspice-46 console reference for the BSIM3 charge-deficit transient NQS
    // topology (`NQSMOD=1`, `ACNQSMOD=0`). Xyce 7.10's regression tree has
    // BSIM3 ACNQS coverage, but no matching transient NQS oracle; its local
    // BSIM3 source also keeps "nqsMod=1 is not ready yet" guards in several
    // transient paths, so ngspice is the physics oracle for this selector.
    let deck = nqsmod_common_source_deck(1);
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 80.0e-12, 0.1e-12)
        .expect("NQSMOD=1 transient runs natively");
    let vout = result
        .try_voltage_waveform_named("out")
        .expect("out waveform");

    let qs_deck = nqsmod_common_source_deck(0);
    let qs_netlist = Netlist::parse(&qs_deck).expect("QS deck parses");
    let qs_result = engine()
        .run_tran(&qs_netlist, 80.0e-12, 0.1e-12)
        .expect("NQSMOD=0 transient runs");
    let qs_vout = qs_result
        .try_voltage_waveform_named("out")
        .expect("QS out waveform");

    let reference: &[(f64, f64, f64)] = &[
        (20.5e-12, 1.900_800_684_161, -1.279_221_177_014e-2),
        (21.0e-12, 1.993_415_522_894, -4.593_396_012_749e-2),
        (21.5e-12, 2.065_573_906_354, -7.473_266_985_363e-2),
        (22.0e-12, 2.111_525_790_000, -9.529_635_000_000e-2),
        (22.5e-12, 2.038_683_686_375, -6.452_367_015_376e-2),
        (23.0e-12, 1.966_591_681_099, -4.135_735_948_028e-2),
        (24.0e-12, 1.828_490_009_922, -1.127_009_894_238e-2),
        (25.0e-12, 1.702_111_376_065, 4.805_536_592_500e-3),
        (30.0e-12, 1.264_027_054_269, 1.498_800_548_199e-2),
        (40.0e-12, 9.593_658_667_354e-1, 3.610_276_377_525e-3),
    ];
    let mut max_qs_delta: f64 = 0.0;
    for &(time, expected, expected_delta_vs_qs) in reference {
        let got = interp_waveform(&result.time, vout, time);
        let abs_err = (got - expected).abs();
        assert!(
            abs_err < 1.0e-3,
            "NQSMOD=1 BSIM3 v(out) at {time:.3e}s: rspice={got:.9e} ngspice={expected:.9e} abs_err={abs_err:.3e}"
        );

        let qs = interp_waveform(&qs_result.time, qs_vout, time);
        let qs_delta = got - qs;
        max_qs_delta = max_qs_delta.max(qs_delta.abs());
        let delta_err = (qs_delta - expected_delta_vs_qs).abs();
        assert!(
            delta_err < 1.5e-3,
            "NQSMOD=1 BSIM3 delta vs QS at {time:.3e}s: rspice={qs_delta:.9e} ngspice={expected_delta_vs_qs:.9e} abs_err={delta_err:.3e}"
        );
    }
    assert!(
        max_qs_delta > 1.0e-2,
        "NQSMOD=1 must not silently degrade to QS; max |delta v(out)|={max_qs_delta:.3e}"
    );
}

#[test]
fn instance_nqsmod1_common_source_transient_matches_ngspice46_global_override() {
    let deck = nqsmod_common_source_instance_deck(0, "nqsmod=1");
    let netlist = Netlist::parse(&deck).expect("instance NQSMOD deck parses");
    let result = engine()
        .run_tran(&netlist, 80.0e-12, 0.1e-12)
        .expect("instance NQSMOD=1 transient runs natively");
    let vout = result
        .try_voltage_waveform_named("out")
        .expect("out waveform");

    let qs_deck = nqsmod_common_source_instance_deck(0, "");
    let qs_netlist = Netlist::parse(&qs_deck).expect("QS deck parses");
    let qs_result = engine()
        .run_tran(&qs_netlist, 80.0e-12, 0.1e-12)
        .expect("instance-default NQSMOD=0 transient runs");
    let qs_vout = qs_result
        .try_voltage_waveform_named("out")
        .expect("QS out waveform");

    let reference: &[(f64, f64, f64)] = &[
        (20.5e-12, 1.900_800_684_161, -1.279_221_177_014e-2),
        (21.0e-12, 1.993_415_522_894, -4.593_396_012_749e-2),
        (21.5e-12, 2.065_573_906_354, -7.473_266_985_363e-2),
        (22.0e-12, 2.111_525_790_000, -9.529_635_000_000e-2),
        (22.5e-12, 2.038_683_686_375, -6.452_367_015_376e-2),
        (23.0e-12, 1.966_591_681_099, -4.135_735_948_028e-2),
    ];
    let mut max_qs_delta: f64 = 0.0;
    for &(time, expected, expected_delta_vs_qs) in reference {
        let got = interp_waveform(&result.time, vout, time);
        let abs_err = (got - expected).abs();
        assert!(
            abs_err < 1.0e-3,
            "instance NQSMOD=1 BSIM3 v(out) at {time:.3e}s: rspice={got:.9e} ngspice={expected:.9e} abs_err={abs_err:.3e}"
        );

        let qs = interp_waveform(&qs_result.time, qs_vout, time);
        let qs_delta = got - qs;
        max_qs_delta = max_qs_delta.max(qs_delta.abs());
        let delta_err = (qs_delta - expected_delta_vs_qs).abs();
        assert!(
            delta_err < 1.5e-3,
            "instance NQSMOD=1 BSIM3 delta vs QS at {time:.3e}s: rspice={qs_delta:.9e} ngspice={expected_delta_vs_qs:.9e} abs_err={delta_err:.3e}"
        );
    }
    assert!(
        max_qs_delta > 1.0e-2,
        "instance NQSMOD=1 must not silently degrade to QS; max |delta v(out)|={max_qs_delta:.3e}"
    );
}

#[test]
fn fractional_instance_nqs_selectors_round_then_reset_like_ngspice46() {
    let rounded_deck = nqsmod_common_source_instance_deck(0, "nqsmod=0.5");
    let exact_deck = nqsmod_common_source_instance_deck(0, "nqsmod=1");

    let rounded_netlist = Netlist::parse(&rounded_deck).expect("fractional NQSMOD deck parses");
    let exact_netlist = Netlist::parse(&exact_deck).expect("exact NQSMOD deck parses");
    let rounded = engine()
        .run_tran(&rounded_netlist, 25.0e-12, 0.1e-12)
        .expect("instance NQSMOD=0.5 rounds to NQSMOD=1 like ngspice");
    let exact = engine()
        .run_tran(&exact_netlist, 25.0e-12, 0.1e-12)
        .expect("instance NQSMOD=1 transient runs");

    let rounded_vout = rounded
        .try_voltage_waveform_named("out")
        .expect("rounded out waveform");
    let exact_vout = exact
        .try_voltage_waveform_named("out")
        .expect("exact out waveform");
    for time in [21.0e-12, 22.0e-12, 23.0e-12] {
        let got = interp_waveform(&rounded.time, rounded_vout, time);
        let expected = interp_waveform(&exact.time, exact_vout, time);
        assert!(
            (got - expected).abs() < 1.0e-10,
            "instance NQSMOD=0.5 should round to NQSMOD=1 at {time:.3e}s: got={got:.9e}, expected={expected:.9e}"
        );
    }

    for suffix in ["nqsmod=1.5", "nqsmod=-0.6", "acnqsmod=1.5"] {
        let deck = nqsmod_common_source_instance_deck(0, suffix);
        let netlist = Netlist::parse(&deck).expect("rounded-out instance selector deck parses");
        engine()
            .run_dc_op_with_report(&netlist)
            .unwrap_or_else(|err| {
                panic!("BSIM3 instance {suffix} should reset like ngspice: {err}")
            });
    }
}

#[test]
fn inverter_ac_response_matches_ngspice() {
    // Small-signal check of the AC path: DC linearization (gm/gds/gmbs +
    // junction conductances) on the real axis, the mode-assembled BSIM3
    // capacitance matrix (intrinsic CAPMOD=3 + overlaps + capbd/capbs) on
    // the imaginary axis. The inverter is biased mid-transition with a
    // 10 fF load so the pole sits inside the sweep.
    let deck = format!(
        "* bsim3 inverter ac\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 dc 0.9 ac 1\n\
         {}\
         cl out 0 10f\n\
         {}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models018()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    // ngspice-46 reference: `ac dec 2 1e6 1e11` on this deck with
    // `.option reltol=1e-6`, run 2026-06-12 (vdb(out), ph(out)).
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 2.462154e+01, 3.139444e+00),
        (1.000000e7, 2.461961e+01, 3.120110e+00),
        (1.000000e8, 2.443028e+01, 2.929833e+00),
        (1.000000e9, 1.721812e+01, 1.984708e+00),
        (1.000000e10, -1.63000e+00, 1.359302e+00),
        (1.000000e11, -1.28853e+01, 3.661702e-01),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, db_ref, ph_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let ph = v.arg();
        println!("AC f={freq:.3e}: engine ({db:.5} dB, {ph:.5} rad) ngspice ({db_ref}, {ph_ref})");
        assert!(
            (db - db_ref).abs() < 1e-3,
            "AC magnitude at {freq:.3e} Hz: engine={db:.5} dB ngspice={db_ref} dB"
        );
        assert!(
            (ph - ph_ref).abs() < 1e-3,
            "AC phase at {freq:.3e} Hz: engine={ph:.5} ngspice={ph_ref}"
        );
    }
}

#[test]
fn inverter_ac_response_with_capmod2_matches_ngspice_and_xpart04() {
    // Same inverter as the CAPMOD=3 AC oracle, but using the BSIM3 CAPMOD=2
    // Meyer-inspired charge branch and XPART=0.4 to exercise the 40/60
    // partition path used by Xyce's BSIM3 regression decks.
    let deck = format!(
        "* bsim3 capmod2 inverter ac\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 dc 0.9 ac 1\n\
         {}\
         cl out 0 10f\n\
         {}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models018_capmod2(0.4)
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    // ngspice-46 reference: same deck with `.option reltol=1e-6`,
    // `ngspice_con.exe -b`, `ac dec 1 1e6 1e11` (vdb(out), ph(out)).
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 2.46215429e+01, 3.13944439e+00),
        (1.000000e7, 2.46196062e+01, 3.12011321e+00),
        (1.000000e8, 2.44301714e+01, 2.92987138e+00),
        (1.000000e9, 1.72158867e+01, 1.98555971e+00),
        (1.000000e10, -1.65277947e+00, 1.36829769e+00),
        (1.000000e11, -1.31679139e+01, 3.78589518e-01),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, db_ref, ph_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let ph = v.arg();
        assert!(
            (db - db_ref).abs() < 1e-3,
            "CAPMOD=2 AC magnitude at {freq:.3e} Hz: engine={db:.5} dB ngspice={db_ref} dB"
        );
        assert!(
            (ph - ph_ref).abs() < 1e-3,
            "CAPMOD=2 AC phase at {freq:.3e} Hz: engine={ph:.5} ngspice={ph_ref}"
        );
    }
}

#[test]
fn inverter_ac_response_with_capmod0_and_1_runs_natively() {
    for capmod in [0, 1] {
        let deck = format!(
            "* bsim3 capmod{capmod} inverter ac\n\
             vdd vdd 0 dc 1.8\n\
             vin in 0 dc 0.9 ac 1\n\
             {}\
             cl out 0 10f\n\
             {}\n\
             .end\n",
            inverter_pair("1", "in", "out"),
            models018_capmod(capmod)
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let results = engine()
            .run_ac(&netlist, &[1.0e6, 1.0e9, 1.0e11])
            .unwrap_or_else(|err| panic!("CAPMOD={capmod} AC should run natively: {err}"));
        for result in &results {
            let idx = result
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case("out"))
                .expect("out in ac result");
            let v = result.voltages[idx];
            assert!(
                v.re.is_finite() && v.im.is_finite(),
                "CAPMOD={capmod} finite v(out), got {v:?}"
            );
        }
    }
}

#[test]
fn common_source_acm1_op_and_ac_match_ngspice46() {
    // ACM=1 derives source/drain junction area/perimeter from HDIF/WMLT/XW
    // rather than the instance AD/AS/PD/PS fields. This biased common-source
    // stage makes the body diode current and drain-bulk capacitance visible.
    let deck = format!(
        "* bsim3 acm1 common source\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.8\n\
         rd vdd out 4k\n\
         vin in 0 dc 0.9 ac 1\n\
         vb b 0 dc 0.2\n\
         m1 out in 0 b n018 w=1u l=0.18u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        models018_acm1()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");

    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("ACM=1 op converges");
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| panic!("missing op param {key}"))
    };
    let assert_rel = |what: &str, ours: f64, reference: f64| {
        let rel = (ours - reference).abs() / reference.abs().max(1e-30);
        assert!(
            rel < 2e-5,
            "{what}: engine={ours:.9e} ngspice={reference:.9e} rel={rel:.3e}"
        );
    };
    assert_rel("id", get("id"), 1.293330554337343e-04);
    assert_rel("gm", get("gm"), 4.230119510284859e-04);
    assert_rel("gds", get("gds"), 1.629544678813967e-05);
    assert_rel("gmb", get("gmb"), 7.669405620390512e-05);

    let reference: &[(f64, f64, f64)] = &[
        (1.0e6, -3.98038151093600e-03, 7.943065348433849e-02),
        (1.0e7, -3.85210026774544e-05, 7.962825491589793e-03),
        (1.0e8, 9.966480492601029e-07, 7.963023589527024e-04),
        (1.0e9, 1.391834485980957e-06, 7.963025570556171e-05),
        (1.0e10, 1.395786351341136e-06, 7.963025590366467e-06),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine()
        .run_ac(&netlist, &freqs)
        .expect("ACM=1 AC runs natively");
    for ((freq, real_ref, imag_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        assert!(
            (v.re - real_ref).abs() <= 2e-5 * real_ref.abs().max(1e-9),
            "ACM=1 AC real at {freq:.3e} Hz: engine={:.9e} ngspice={:.9e}",
            v.re,
            real_ref
        );
        assert!(
            (v.im - imag_ref).abs() <= 2e-5 * imag_ref.abs().max(1e-9),
            "ACM=1 AC imag at {freq:.3e} Hz: engine={:.9e} ngspice={:.9e}",
            v.im,
            imag_ref
        );
    }
}

#[test]
fn common_source_acm1_series_resistance_matches_ngspice46() {
    let deck = format!(
        "* bsim3 acm1 common source with series resistance\n\
         .option reltol=1e-7 abstol=1e-15 vntol=1e-9 chgtol=1e-16 gmin=1e-12\n\
         vdd vdd 0 dc 1.8\n\
         rdload vdd out 3k\n\
         vin in 0 dc 0.95 ac 1\n\
         vb b 0 dc 0.2\n\
         m1 out in 0 b n018 w=1u l=0.18u nrd=3 nrs=4\n\
         {}\n\
         .end\n",
        models018_acm1_rseries()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");

    let (op, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("ACM=1 series-resistance op converges");
    let out = op
        .node_index_named("out")
        .map(|node| op.voltage(node))
        .expect("out in op result");
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| panic!("missing op param {key}"))
    };
    let assert_rel = |what: &str, ours: f64, reference: f64| {
        let rel = (ours - reference).abs() / reference.abs().max(1e-30);
        assert!(
            rel < 2e-5,
            "{what}: engine={ours:.9e} ngspice={reference:.9e} rel={rel:.3e}"
        );
    };
    assert_rel("v(out)", out, 1.389230626530001e+00);
    assert_rel("id", get("id"), 1.369231210805875e-04);
    assert_rel("gm", get("gm"), 4.383635283208747e-04);
    assert_rel("gds", get("gds"), 1.694346660945246e-05);
    assert_rel("gmb", get("gmb"), 8.356978654885963e-05);

    let reference: &[(f64, f64, f64)] = &[
        (1.0e6, -6.19444853565072e-03, 6.249385643499959e-02),
        (1.0e7, -6.76297024298875e-05, 6.413601019897896e-03),
        (1.0e8, 4.294825441277538e-07, 6.415544439130715e-04),
        (1.0e9, 1.110888270110707e-06, 6.415563909484882e-05),
        (1.0e10, 1.117702408919968e-06, 6.415564104192049e-06),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine()
        .run_ac(&netlist, &freqs)
        .expect("ACM=1 series-resistance AC runs natively");
    for ((freq, real_ref, imag_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        assert!(
            (v.re - real_ref).abs() <= 2e-5 * real_ref.abs().max(1e-9),
            "ACM=1 series AC real at {freq:.3e} Hz: engine={:.9e} ngspice={:.9e}",
            v.re,
            real_ref
        );
        assert!(
            (v.im - imag_ref).abs() <= 2e-5 * imag_ref.abs().max(1e-9),
            "ACM=1 series AC imag at {freq:.3e} Hz: engine={:.9e} ngspice={:.9e}",
            v.im,
            imag_ref
        );
    }
}

#[test]
fn acm_outside_supported_integer_set_is_rejected() {
    for selector in ["0.5", "1.5", "2"] {
        let models = models018().replace(
            ".model n018 nmos level=49",
            &format!(".model n018 nmos level=49 acm={selector}"),
        );
        let deck = format!(
            "* bsim3 unsupported acm selector\n\
             vdd vdd 0 dc 1.8\n\
             rd vdd out 4k\n\
             vin in 0 dc 0.9\n\
             m1 out in 0 0 n018 w=1u l=0.18u\n\
             {models}\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let err = engine()
            .run_dc_op_with_report(&netlist)
            .expect_err("unsupported ACM should reject before simulation");
        let message = err.to_string();
        assert!(
            message.contains("ACM"),
            "ACM={selector}: unexpected error: {message}"
        );
    }
}

#[test]
fn inverter_ac_response_with_acnqsmod1_matches_ngspice46() {
    // BSIM3 ACNQSMOD=1 is implemented in ngspice's b3acld.c and b3ld.c.
    // Xyce 7.10 accepts ACNQSMOD on BSIM3 LEVEL=9 decks but produces the
    // same AC output with ACNQSMOD on and off for this circuit, so it is
    // coverage-only here rather than a matching AC-NQS physics oracle.
    let deck = format!(
        "* bsim3 acnqs inverter ac\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 dc 0.9 ac 1\n\
         {}\
         cl out 0 10f\n\
         {}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models018_acnqsmod1()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    // ngspice-46 reference: same deck with `.option reltol=1e-6`,
    // `.ac dec 1 1e6 1e11`, and `wrdata v(out)`.
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, -1.70245705e+01, 3.65615895e-02),
        (1.000000e7, -1.70169605e+01, 3.65455847e-01),
        (1.000000e8, -1.62881907e+01, 3.50128856e+00),
        (1.000000e9, -2.84420288e+00, 6.73825002e+00),
        (1.000000e10, 3.13948842e-01, 7.83344665e-01),
        (1.000000e11, 1.92673406e-01, -6.65831480e-03),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, real_ref, imag_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let real_err = (v.re - real_ref).abs();
        let imag_err = (v.im - imag_ref).abs();
        println!(
            "ACNQSMOD=1 BSIM3 f={freq:.3e}: rspice=({:.9e}, {:.9e}) ngspice=({real_ref:.9e}, {imag_ref:.9e})",
            v.re, v.im
        );
        assert!(
            real_err <= 5e-10 + 3e-3 * real_ref.abs(),
            "ACNQSMOD=1 BSIM3 AC real(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={real_ref:.9e} abs_err={real_err:.3e}",
            v.re
        );
        assert!(
            imag_err <= 5e-10 + 3e-3 * imag_ref.abs(),
            "ACNQSMOD=1 BSIM3 AC imag(vout) at {freq:.3e} Hz: rspice={:.9e} ngspice={imag_ref:.9e} abs_err={imag_err:.3e}",
            v.im
        );
    }
}

#[test]
fn level9_acnqsmod1_deck_runs_natively_for_xyce_compatibility() {
    // Xyce regression modelcards in RINGS/INIT_CONDS/IC_AND_NODESET contain
    // LEVEL=9 BSIM3 with `Acnqsmod=1 elm=3`. Xyce 7.10 ignores the AC delta
    // for this parameter, but RSpice must still accept and run these decks
    // natively instead of falling back to a simplified or Verilog model.
    let deck = format!(
        "* bsim3 level9 acnqs xyce-compat ac\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 dc 0.9 ac 1\n\
         {}\
         cl out 0 10f\n\
         {}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models018_level9_acnqsmod1()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let results = engine()
        .run_ac(&netlist, &[1.0e6, 1.0e9, 1.0e11])
        .expect("LEVEL=9 ACNQSMOD=1 deck runs natively");
    for result in &results {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        assert!(
            v.re.is_finite() && v.im.is_finite(),
            "LEVEL=9 ACNQSMOD=1 finite v(out), got {v:?}"
        );
    }
}

#[test]
fn acnqsmod1_is_rejected_for_pole_zero_until_charge_deficit_state_exists() {
    let deck = format!(
        "* bsim3 acnqs pz rejection\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 dc 0.9 ac 1\n\
         {}\
         cl out 0 10f\n\
         {}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models018_acnqsmod1()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let circuit = engine().build_circuit(&netlist).expect("circuit builds");
    let input = circuit.get_node_by_name("in").expect("input node");
    let output = circuit.get_node_by_name("out").expect("output node");
    let err = engine()
        .run_pz(&netlist, input, output)
        .expect_err("ACNQSMOD=1 is rational and must not use G+sC PZ extraction");
    let message = err.to_string();
    assert!(
        message.contains("Pole-zero")
            && message.contains("BSIM3")
            && message.contains("ACNQSMOD=1")
            && message.contains("charge-deficit"),
        "typed PZ rejection should name BSIM3 ACNQSMOD=1 and the missing state: {message}"
    );
}

#[test]
fn inverter_ac_response_with_capmod2_matches_xyce710() {
    // Xyce 7.10 LEVEL=9 BSIM3 uses CAPMOD=2 in several regression decks
    // (for example TR_TRAN/tr.cir). This pins RSpice's native evaluator
    // against the same circuit-level AC behavior while keeping ngspice as the
    // intrinsic-charge oracle above.
    let models = models018_capmod2(0.4).replace("level=49", "level=9 version=3.2.2");
    let deck = format!(
        "* bsim3 capmod2 inverter ac xyce\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 dc 0.9 ac 1\n\
         {}\
         cl out 0 10f\n\
         {models}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    // XyceNF 7.10.0 reference, same deck with `.print ac vm(out) vp(out)`.
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 1.70246141e+01, 1.79876914e+02),
        (1.000000e7, 1.70208185e+01, 1.78769318e+02),
        (1.000000e8, 1.66536216e+01, 1.67869260e+02),
        (1.000000e9, 7.25762182e+00, 1.13764183e+02),
        (1.000000e10, 8.26724866e-01, 7.83976809e+01),
        (1.000000e11, 2.19585824e-01, 2.16915803e+01),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, mag_ref, phase_deg_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let mag = v.norm();
        let phase_deg = v.arg().to_degrees();
        let mag_rel = (mag - mag_ref).abs() / mag_ref;
        assert!(
            mag_rel < 1e-3,
            "CAPMOD=2 AC magnitude vs Xyce at {freq:.3e} Hz: engine={mag:.9e} xyce={mag_ref:.9e} rel={mag_rel:.3e}"
        );
        assert!(
            (phase_deg - phase_deg_ref).abs() < 5e-2,
            "CAPMOD=2 AC phase vs Xyce at {freq:.3e} Hz: engine={phase_deg:.6} deg xyce={phase_deg_ref:.6} deg"
        );
    }
}

#[test]
fn level49_runs_without_simplified_mos_optin() {
    // The full LEVEL=49 card must build and solve natively: no
    // `.options allow_simplified_mos`, no rejection, and the OP report
    // names the BSIM3 port for every instance.
    let deck = format!(
        "* level 49 native\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 dc 0.9\n\
         {}\
         {}\n\
         .op\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models018()
    );
    assert!(
        !deck.contains("allow_simplified_mos"),
        "deck must not opt into the approximation"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("LEVEL=49 deck runs natively");
    let bsim3_count = report
        .entries
        .iter()
        .filter(|e| e.device_kind == "BSIM3")
        .count();
    assert_eq!(bsim3_count, 2, "both transistors use the native port");
}
