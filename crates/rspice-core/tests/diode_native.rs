//! Native diode validation against Cadence PSpice, ngspice 46, and Xyce 7.10.

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;
use rspice_core::solver::SimulationResult;

#[test]
fn diode_dc_preserves_resolved_currents_near_zero_bias() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(config);
        let nvt = 300.15
            * if dialect == SpiceDialect::Xyce {
                1.3806226e-23 / 1.6021918e-19
            } else {
                1.38064852e-23 / 1.6021766208e-19
            };
        for voltage in [-1e-20, 1e-20, -1e-200, 1e-200] {
            let deck=Netlist::parse(&format!("Small diode bias\nV1 anode 0 {voltage:e}\nD1 anode 0 DM\n.model DM D(IS=0.01)\n.end\n")).unwrap();
            let result = engine.run_dc_op(&deck).unwrap();
            let current = branch_current(&result, "V1");
            let expected = -0.01 / nvt;
            assert!(
                (current / voltage - expected).abs() < 2e-14 * expected.abs(),
                "{dialect:?}, {voltage:e}: {current:e}"
            );
            let diode = result.try_dc_observable_named("I(D1)").unwrap();
            assert!((diode / voltage + expected).abs() < 2e-14 * expected.abs());
        }
    }
}

#[test]
fn diode_charge_only_reactive_state_matches_the_analytic_rc_response() {
    // A reverse-biased M=0 junction has Q=CJO*V. With negligible IS, this
    // circuit is exactly an RC low-pass with no explicit capacitor. Checking
    // the entire waveform also checks phase and the startup charge history.
    let tau = 1e3 * 1e-9;
    let omega = std::f64::consts::TAU * 1e6;
    let ratio: f64 = omega * tau;
    let amplitude = 0.01 / (1.0 + ratio * ratio).sqrt();
    for method in ["trap", "gear"] {
        let netlist = Netlist::parse(&format!(
            "diode charge RC oracle\n\
             V1 in 0 SIN(-1 0.01 1meg)\n\
             R1 in out 1k\n\
             D1 out 0 dm\n\
             .model dm D(IS=1e-30 CJO=1n M=0 TT=0)\n\
             .options method={method}\n\
             .tran 1n 5u\n\
             .end\n"
        ))
        .expect("charge-only diode deck parses");
        let result = Engine::default()
            .run_tran(&netlist, 5e-6, 1e-9)
            .expect("charge-only diode transient converges");
        let output = result.try_voltage_waveform_named("OUT").unwrap();
        let mut max_error = 0.0_f64;
        for (&time, &voltage) in result.time.iter().zip(output) {
            let phase = omega * time;
            let expected = -1.0
                + 0.01 / (1.0 + ratio * ratio)
                    * (phase.sin() - ratio * phase.cos() + ratio * (-time / tau).exp());
            max_error = max_error.max((voltage - expected).abs());
        }
        assert!(
            max_error < amplitude * 0.002,
            "{method}: maximum charge waveform error {max_error:e}, amplitude {amplitude:e}"
        );
    }
}

fn branch_current(result: &SimulationResult, branch: &str) -> f64 {
    result
        .branch_current_named(branch)
        .unwrap_or_else(|| panic!("missing branch {branch} in {:?}", result.branch_names))
}

fn ac_branch_current(deck: &str, branch: &str, frequency: f64) -> rspice_core::Complex64 {
    let netlist = Netlist::parse(deck).expect("diode AC deck parses");
    let point = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[frequency])
        .expect("diode AC converges")
        .pop()
        .expect("one AC point");
    let index = point
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(branch))
        .unwrap_or_else(|| panic!("missing branch {branch} in {:?}", point.branch_names));
    point.currents[index]
}

fn op_branch_current(model_tail: &str, voltage: f64) -> f64 {
    let deck = format!(
        "* diode high-injection knee oracle\n\
         .options gmin=0\n\
         V1 anode 0 {voltage:.15e}\n\
         D1 anode 0 DK\n\
         .model DK D(IS=1e-14 N=1 RS=0 CJO=0 {model_tail})\n\
         .op\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("diode deck parses");
    let mut config = SimulationConfig::default();
    // Isolate the model-card high-injection knee from RSpice's internal
    // nodal conditioning floor; the deck already sets junction GMIN to zero.
    config.convergence_config.gmin_target = 0.0;
    let result = Engine::new(config)
        .run_dc_op(&netlist)
        .expect("diode op converges");
    branch_current(&result, "v1")
}

fn assert_close(label: &str, got: f64, expected: f64, rel_tol: f64, abs_tol: f64) {
    let abs = (got - expected).abs();
    let tol = abs_tol.max(rel_tol * expected.abs().max(got.abs()));
    assert!(
        abs <= tol,
        "{label}: rspice={got:.12e} ngspice46={expected:.12e} abs={abs:.3e} tol={tol:.3e}"
    );
}

#[test]
fn diode_ikf_limits_forward_high_injection_current_like_ngspice46() {
    // ngspice-46, same one-diode .op deck, `IKF=1e-3`:
    // i(V1) = -1.551419616134975e-02.  Without the high-injection knee
    // rolloff this bias point is -2.708299612085749e-01 A.
    assert_close(
        "IKF-limited diode source current",
        op_branch_current("IKF=1e-3", 0.8),
        -1.551_419_616_134_975e-2,
        2.0e-8,
        1.0e-10,
    );
}

#[test]
fn diode_tiny_ikf_is_disabled_like_ngspice46() {
    // ngspice-46 prints `Warning: ... IKF too small - model effect disabled!`
    // and returns the no-knee current for this vendor-model corner.
    assert_close(
        "tiny IKF disabled diode source current",
        op_branch_current("IKF=1e-186", 0.8),
        -2.708_299_612_085_749e-1,
        2.0e-8,
        1.0e-10,
    );
}

#[test]
fn diode_ik_alias_limits_forward_high_injection_current_like_ngspice46() {
    assert_close(
        "IK alias-limited diode source current",
        op_branch_current("IK=1e-3", 0.8),
        -1.551_419_616_134_975e-2,
        2.0e-8,
        1.0e-10,
    );
}

#[test]
fn diode_ikr_limits_reverse_high_injection_current_like_ngspice46() {
    // ngspice-46, same one-diode .op deck, `IKR=1e-14` at Vd=-0.2:
    // i(V1) = 4.989091488361717e-15.  Without the reverse knee this is
    // 9.970924740527615e-15 A.
    assert_close(
        "IKR-limited diode source current",
        op_branch_current("IKR=1e-14", -0.2),
        4.989_091_488_361_717e-15,
        2.0e-6,
        1.0e-20,
    );
}

fn isr_reverse_branch_current(dialect: SpiceDialect, voltage: f64) -> f64 {
    let deck = format!(
        "* diode ISR/NR dialect oracle\n\
         .options gmin=0\n\
         V1 anode 0 {voltage:.15e}\n\
         D1 anode 0 DM\n\
         .model DM D(IS=1e-30 N=1.0136 RS=0 CJO=0 M=.55916 VJ=1.0542 ISR=564.09e-9 NR=4.9950)\n\
         .op\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("ISR dialect deck parses");
    let mut config = SimulationConfig::default().with_spice_dialect(dialect);
    config.convergence_config.gmin_target = 0.0;
    let result = Engine::new(config)
        .run_dc_op(&netlist)
        .expect("ISR dialect operating point converges");
    branch_current(&result, "v1")
}

fn pspice_recombination_current(vd: f64) -> f64 {
    let vt = (1.38064852e-23 / 1.6021766208e-19) * 300.15;
    let isr = 564.09e-9;
    let nr = 4.9950;
    let vj = 1.0542;
    let grading = 0.55916;
    let irec = isr * ((vd / (nr * vt)).exp() - 1.0);
    let kgen = ((1.0 - vd / vj).powi(2) + 0.005).powf(grading / 2.0);
    irec * kgen
}

#[test]
fn diode_isr_reverse_current_keeps_pspice_ngspice_and_xyce_semantics_distinct() {
    let vd = -10.0;
    let pspice_expected = -pspice_recombination_current(vd);
    assert_close(
        "best-available PSpice ISR reverse current",
        isr_reverse_branch_current(SpiceDialect::BestAvailable, vd),
        pspice_expected,
        2.0e-10,
        1.0e-18,
    );

    let vt = (1.38064852e-23 / 1.6021766208e-19) * 300.15;
    let ngspice_boundary = -3.0 * 1.0136 * vt;
    let ngspice_expected = -pspice_recombination_current(ngspice_boundary);
    assert_close(
        "explicit ngspice ISR reverse-current clamp",
        isr_reverse_branch_current(SpiceDialect::Ngspice, vd),
        ngspice_expected,
        2.0e-10,
        1.0e-18,
    );

    assert_close(
        "explicit Xyce ISR reverse-current omission",
        isr_reverse_branch_current(SpiceDialect::Xyce, vd),
        1.0e-30,
        0.0,
        1.0e-28,
    );
}

#[test]
fn diode_grading_above_one_retains_reverse_bias_capacitance_like_ngspice46() {
    let frequency = 1.0e6;
    let deck = "* M > 1 diode depletion-capacitance oracle\n\
                V1 anode 0 DC -4 AC 1\n\
                D1 anode 0 DM\n\
                .model DM D(IS=1e-14 N=1 RS=0 CJO=463.53p VJ=9.99 M=1.2861 TT=0 TNOM=25)\n\
                .temp 25\n\
                .end\n";
    let current = ac_branch_current(deck, "V1", frequency);
    let capacitance = current.im.abs() / (std::f64::consts::TAU * frequency);
    let expected = 463.53e-12 * (1.0_f64 + 4.0 / 9.99).powf(-1.2861);

    assert_close(
        "M > 1 reverse-bias capacitance",
        capacitance,
        expected,
        2.0e-10,
        1.0e-21,
    );
}

/// A diode across a negative resistance has two DC roots: the junction's
/// low-forward-bias root and a deeply reverse-biased one where the reverse
/// saturation current balances the same line. Which root the solve reports is
/// exactly what the `OFF` instance keyword is there to decide.
fn negative_resistance_bistable_deck(off: bool) -> String {
    format!(
        "* diode bistable steered by the OFF keyword\n\
         vs vs 0 dc -0.3\n\
         rn n vs -1k\n\
         d1 n 0 dmod{}\n\
         .model dmod D(IS=1e-3 N=1)\n\
         .op\n\
         .end\n",
        if off { " OFF" } else { "" }
    )
}

fn op_node_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("diode deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("diode bistable operating point converges");
    result
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("missing voltage for node {node}"))
}

#[test]
fn diode_off_keyword_selects_the_bistable_operating_point_branch() {
    // dioload.c evaluates an OFF instance at exactly vd = 0 on MODEINITJCT, in
    // every compatibility mode, which lands this network on its forward root.
    // ngspice-46 reports v(n) = 6.923413e-03 there. An instance whose IS is
    // large enough for the pnjlim reference to still conduct is where that
    // differs from merely limiting the raw bias against zero, and a simulator
    // that drops the keyword reports the reverse root near -1.3 V instead.
    let off_root = op_node_voltage(&negative_resistance_bistable_deck(true), "n");
    assert_close(
        "OFF diode bistable v(n)",
        off_root,
        6.923_413e-3,
        1.0e-5,
        0.0,
    );
}

/// The same network at a default saturation current, where `tVcrit` is a
/// three-quarter-volt forward bias rather than the 75 mV a milliamp `IS`
/// gives. Here the two MODEINITJCT arms land on genuinely different roots, so
/// the deck reads the startup bias directly rather than inferring it.
fn standard_bistable_deck(off: bool) -> String {
    format!(
        "* diode bistable steered by the MODEINITJCT startup bias\n\
         vs vs 0 dc -1.5\n\
         rn n vs -1k\n\
         d1 n 0 dmod{}\n\
         .model dmod D(IS=1e-14 N=1)\n\
         .op\n\
         .end\n",
        if off { " OFF" } else { "" }
    )
}

#[test]
fn diode_startup_bias_selects_the_bistable_operating_point_branch() {
    // Both roots are genuine equilibria of this network: the junction's
    // forward root near 0.68 V, and the one where the resistor line meets the
    // reverse saturation current a hair below the supply. Which one a solve
    // reports is decided entirely by where dioload.c's MODEINITJCT arms open
    // the junction — `vd = tVcrit` (dioload.c:162-166) for an unmarked
    // instance, `vd = 0` (dioload.c:158-161) for one the deck marked OFF.
    //
    // ngspice-46 reports v(n) = 6.752190e-01 and -1.50000e+00 respectively.
    // A simulator that instead limits a zero-referenced raw bias opens both
    // instances at cutoff and reports the reverse root for both, which makes
    // the unmarked diode's operating point disagree with every other SPICE.
    let unmarked = op_node_voltage(&standard_bistable_deck(false), "n");
    assert_close(
        "unmarked diode bistable v(n)",
        unmarked,
        6.752_190e-1,
        1.0e-5,
        0.0,
    );

    let off = op_node_voltage(&standard_bistable_deck(true), "n");
    assert_close("OFF diode bistable v(n)", off, -1.5, 1.0e-6, 1.0e-9);
}

#[test]
fn diode_xyce_high_injection_matches_current_and_ac_charge_oracles() {
    // Xyce 7.10 N_DEV_Diode.C: Inorm*Khi, Khi=sqrt(IKF/(IKF+Inorm)).
    // Its GMIN contribution is inside Inorm, before the injection factor.
    let vt: f64 = 300.15 * 1.380_622_6e-23 / 1.602_191_8e-19;
    for gmin in [0.0, 1e-3] {
        for ratio in [1.0_f64, 0.01, 100.0] {
            let voltage = vt * (1.0 + ratio * 1e-3 / 1e-14).ln();
            let normal = 1e-14 * (voltage / vt).exp_m1() + gmin * voltage;
            let normal_g = 1e-14 * (voltage / vt).exp() / vt + gmin;
            let r = normal / 1e-3;
            let expected_i = normal / (1.0 + r).sqrt();
            let expected_g = normal_g * (1.0 + 0.5 * r) / (1.0 + r).powf(1.5);
            let deck = Netlist::parse(&format!("Xyce high injection\nV1 n 0 DC {voltage:.17e} AC 1\nD1 n 0 dm\n.model dm D(IS=1e-14 N=1 IKF=1m CJO=0 TT=2n)\n.options GMIN={gmin:.17e} RELTOL=1e-8 ABSTOL=1e-15 VNTOL=1e-12\n.end\n")).unwrap();
            let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
            config.convergence_config.gmin_target = 0.0;
            let engine = Engine::new(config);
            let point = engine.run_dc_op(&deck).unwrap();
            let source_i = -branch_current(&point, "v1");
            let observed_i = point.try_dc_observable_named("I(D1)").unwrap();
            assert!(
                (source_i - expected_i).abs() < 1e-14 + 1e-9 * expected_i.abs(),
                "GMIN={gmin:e} ratio={ratio}: source {source_i:e}, expected {expected_i:e}"
            );
            assert!(
                (observed_i - expected_i).abs() < 1e-14 + 1e-9 * expected_i.abs(),
                "GMIN={gmin:e} ratio={ratio}: I(D1)={observed_i:e}, expected {expected_i:e}"
            );
            for frequency in [1e6, 1e9] {
                let result = engine.run_ac(&deck, &[frequency]).unwrap();
                let index = result[0]
                    .branch_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case("v1"))
                    .unwrap();
                let current = -result[0].currents[index];
                let susceptance = std::f64::consts::TAU * frequency * 2e-9 * expected_g;
                assert!(
                    (current.re - expected_g).abs() < 1e-13 + 1e-9 * expected_g.abs(),
                    "AC GMIN={gmin:e} ratio={ratio}: {current:?}, expected G={expected_g:e}"
                );
                assert!((current.im - susceptance).abs() < 1e-13 + 1e-9 * susceptance.abs());
            }
        }
    }
}

#[test]
fn diode_xyce_injection_keeps_recombination_and_sidewall_outside_the_knee() {
    let vt: f64 = 300.15 * 1.380_622_6e-23 / 1.602_191_8e-19;
    for separate in [false, true] {
        for gmin in [0.0, 1e-3] {
            let v: f64 = 0.67;
            let isat = if separate { 1e-14 } else { 1e-14 + 2.0 * 3e-14 };
            let nvt = 1.1 * vt;
            let normal = isat * (v / nvt).exp_m1() + gmin * v;
            let normal_g = isat * (v / nvt).exp() / nvt + gmin;
            let ratio = normal / 1e-3;
            let mut expected_i = normal / (1.0 + ratio).sqrt();
            let mut expected_g = normal_g * (1.0 + 0.5 * ratio) / (1.0 + ratio).powf(1.5);
            let rec = 1e-10 * (v / (2.0 * vt)).exp_m1();
            let rec_g = 1e-10 * (v / (2.0 * vt)).exp() / (2.0 * vt);
            let base = (1.0 - v).powi(2) + 0.005;
            let generation = base.powf(0.25);
            let generation_g = -0.5 * (1.0 - v) * base.powf(-0.75);
            expected_i += rec * generation;
            expected_g += rec_g * generation + rec * generation_g;
            if separate {
                expected_i += 6e-14 * (v / (1.3 * vt)).exp_m1();
                expected_g += 6e-14 * (v / (1.3 * vt)).exp() / (1.3 * vt);
            }
            let sidewall = if separate { "NS=1.3" } else { "" };
            let deck = Netlist::parse(&format!("Xyce composite diode\nV1 n 0 DC {v} AC 1\nD1 n 0 dm PJ=2\n.model dm D(IS=1e-14 N=1.1 IKF=1m ISR=1e-10 NR=2 JSW=3e-14 {sidewall} VJ=1 M=.5 CJO=0 TT=2n)\n.options GMIN={gmin:e} RELTOL=1e-8 ABSTOL=1e-15 VNTOL=1e-12\n.end\n")).unwrap();
            let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
            config.convergence_config.gmin_target = 0.0;
            let engine = Engine::new(config);
            let point = engine.run_dc_op(&deck).unwrap();
            for current in [
                -branch_current(&point, "v1"),
                point.try_dc_observable_named("I(D1)").unwrap(),
            ] {
                assert!(
                    (current - expected_i).abs() < 1e-14 + 1e-9 * expected_i.abs(),
                    "NS={separate} GMIN={gmin}: {current:e} vs {expected_i:e}"
                );
            }
            let result = engine.run_ac(&deck, &[1e6]).unwrap();
            let index = result[0]
                .branch_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case("v1"))
                .unwrap();
            let actual = -result[0].currents[index];
            assert!((actual.re - expected_g).abs() < 1e-13 + 1e-9 * expected_g.abs());
            let expected_c = std::f64::consts::TAU * 1e6 * 2e-9 * expected_g;
            assert!((actual.im - expected_c).abs() < 1e-13 + 1e-9 * expected_c.abs());
        }
    }
}

#[test]
fn diode_xyce_tikf_controls_temperature_resolved_current_and_charge() {
    // Xyce 7.10 updateTemperature resolves tIKF before evaluating F/G/TT.
    for (temperature, nominal, coefficient) in [
        (127.0, 27.0, 0.01),
        (-23.0, 27.0, 0.01),
        (27.0, 27.0, 0.01),
        (127.0, 27.0, -0.005),
        (127.0, 27.0, -0.01),
        (127.0, 27.0, -0.02),
        (77.0, 77.0, 0.02),
        (127.0, 77.0, 0.02),
        (127.0, 27.0, 0.0),
    ] {
        let vt = (temperature + 273.15) * 1.380_622_6e-23 / 1.602_191_8e-19;
        let ratio: f64 = (temperature + 273.15) / (nominal + 273.15);
        let saturation = 1e-14 * ((ratio - 1.0) * 1.11 / vt + 3.0 * ratio.ln()).exp();
        let voltage = vt * (1.0_f64 + 1e-3 / saturation).ln();
        let knee = 1e-3 * (1.0 + coefficient * (temperature - nominal));
        for gmin in [0.0, 1e-3] {
            let normal = saturation * (voltage / vt).exp_m1() + gmin * voltage;
            let normal_g = saturation * (voltage / vt).exp() / vt + gmin;
            let (expected_i, expected_g) = if knee > 0.0 {
                let ratio = normal / knee;
                (
                    normal / (1.0 + ratio).sqrt(),
                    normal_g * (1.0 + 0.5 * ratio) / (1.0 + ratio).powf(1.5),
                )
            } else {
                (normal, normal_g)
            };
            for route in 0..3 {
                let (instance, global) = match route {
                    0 => (format!("TEMP={temperature}"), String::new()),
                    1 => (format!("DTEMP={}", temperature - 27.0), String::new()),
                    _ => (String::new(), format!(".temp {temperature}")),
                };
                let deck = Netlist::parse_with_options(
                    &format!("Xyce diode temperature knee\nV1 n 0 DC {voltage:.17e} AC 1\nD1 n 0 dm {instance}\n.model dm D(IS=1e-14 N=1 EG=1.11 XTI=3 IKF=1m TIKF={coefficient} CJO=0 TT=2n TNOM={nominal})\n{global}\n.options GMIN={gmin} RELTOL=1e-8 ABSTOL=1e-15 VNTOL=1e-12\n.end\n"),
                    rspice_core::netlist::NetlistParseOptions {
                        expression_dialect: rspice_core::config::ExpressionDialect::Xyce,
                        ..Default::default()
                    },
                ).unwrap();
                assert!(deck.diagnostics.is_empty());
                let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
                config.convergence_config.gmin_target = 0.0;
                let engine = Engine::new(config);
                let point = engine.run_dc_op(&deck).unwrap();
                for current in [
                    -branch_current(&point, "V1"),
                    point.try_dc_observable_named("I(D1)").unwrap(),
                ] {
                    assert!(
                        (current - expected_i).abs() < 1e-14 + 1e-9 * expected_i.abs(),
                        "route={route} TEMP={temperature} TNOM={nominal} TIKF={coefficient} GMIN={gmin}: {current:e} vs {expected_i:e}"
                    );
                }
                let ac = engine.run_ac(&deck, &[1e7]).unwrap();
                let index = ac[0]
                    .branch_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case("v1"))
                    .unwrap();
                let actual = -ac[0].currents[index];
                let expected_b = std::f64::consts::TAU * 1e7 * 2e-9 * expected_g;
                assert!((actual.re - expected_g).abs() < 1e-13 + 1e-9 * expected_g.abs());
                assert!((actual.im - expected_b).abs() < 1e-13 + 1e-9 * expected_b.abs());
            }
        }
    }
}
