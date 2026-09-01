//! Periodic-AC validation against closed-form answers.
//!
//! Two analytic gates pin the conversion-matrix solve:
//! 1. A linear RC low-pass under PAC must reproduce the ordinary AC transfer
//!    at every SIGNED sideband frequency offset + k*f0, with exactly zero
//!    sideband conversion.
//! 2. A switch chopper driven by a sinusoidal LO is the classic textbook
//!    mixer: a 50% duty square-wave conductance whose +/-1-harmonic
//!    conversion gain is A/pi and whose direct feedthrough is A/2.

use num_complex::Complex64;
use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::PssConfig;
use rspice_core::analysis::pac::PacConfig;
use rspice_core::engine::{Engine, PacAnalysisResult, SimulationConfig};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;

fn run_pac(deck: &str, config: PacConfig) -> PacAnalysisResult {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    engine.run_pac(&netlist, config).expect("PAC completes")
}

fn one_point_pac_config(input: &str, output: &str) -> PacConfig {
    PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e4, 1.0e4, 1)
        .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
        .with_sidebands(0, 0)
        .with_input_source(input)
        .with_output_node(output)
}

#[test]
fn direct_pac_resolves_deck_rshunt_before_circuit_elaboration() {
    let deck = "\
* RSHUNT is the only physical return path
iin 0 out dc 0
.options rshunt=2k
.end
";
    let analysis = run_pac(deck, one_point_pac_config("iin", "out"));
    let transfer = analysis
        .result
        .conversion_matrix
        .get(0, 0, 0)
        .expect("the one-point PAC transfer is retained");
    assert!(
        (transfer - Complex64::new(2.0e3, 0.0)).norm() <= 1.0e-12,
        "deck RSHUNT must produce its exact transimpedance, got {transfer}"
    );
}

#[test]
fn direct_pac_applies_hb_local_options_and_accepts_typed_initializers() {
    let base = "\
* PAC HB-local option gate
iin 0 out dc 0
r1 out 0 1k
.end
";
    let mut zero_budget = Netlist::parse(base).expect("base deck parses");
    zero_budget.options.nonlin_hb_maxstep = Some(0);
    let error = Engine::new(SimulationConfig::default())
        .run_pac(&zero_budget, one_point_pac_config("iin", "out"))
        .expect_err("a zero NONLIN-HB MAXSTEP must fail at the PAC boundary");
    assert!(error.to_string().contains("MAXSTEP must be at least 1"));

    let transient_tahb = Netlist::parse(&base.replace(".end", ".options hbint tahb=1\n.end"))
        .expect("typed TAHB deck parses");
    Engine::new(SimulationConfig::default())
        .run_pac(&transient_tahb, one_point_pac_config("iin", "out"))
        .expect("a supported HB initializer remains analysis-local for direct PAC");
}

#[test]
fn linear_rc_pac_matches_the_ac_transfer_at_signed_sideband_frequencies() {
    // RC corner exactly at 1 MHz: H(f) = 1 / (1 + j f/fc) with fc = 1 MHz.
    let deck = "\
* linear rc low-pass
vin in 0 dc 0 ac 1
r1 in out 1k
c1 out 0 159.154943091895p
.end
";
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e5, 5.0e5, 3)
        .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
        .with_sidebands(-2, 2)
        .with_input_source("vin")
        .with_output_node("out");

    let analysis = run_pac(deck, config);
    let result = &analysis.result;
    let out_idx = result.node_index("out").expect("out node present");

    let fc = 1.0e6;
    for (freq_idx, &offset) in result.frequencies.clone().iter().enumerate() {
        for m in -2..=2 {
            // The input excited at sideband m responds only at sideband m,
            // with the ordinary AC transfer at the signed frequency.
            let f_abs = offset + (m as f64) * F0;
            let expected = Complex64::new(1.0, 0.0) / Complex64::new(1.0, f_abs / fc);

            let got = result
                .conversion_matrix
                .get(freq_idx, m, m)
                .expect("conversion value is materialized");
            assert!(
                (got - expected).norm() < 1e-3 * expected.norm(),
                "diagonal transfer at offset {offset:.3e}, sideband {m}: got {got}, want {expected}"
            );

            for k in -2..=2 {
                if k == m {
                    continue;
                }
                let leak = result
                    .conversion_matrix
                    .get(freq_idx, k, m)
                    .expect("conversion value is materialized")
                    .norm();
                assert!(
                    leak < 1e-9,
                    "linear circuit must not convert sidebands: |H[{k},{m}]| = {leak:.3e}"
                );
            }
        }

        // The m = 0 column doubles as the per-node sideband spectra.
        let direct = result
            .voltage(out_idx, freq_idx, 0)
            .expect("PAC node voltage is retained");
        let expected = Complex64::new(1.0, 0.0) / Complex64::new(1.0, offset / fc);
        assert!(
            (direct - expected).norm() < 1e-3 * expected.norm(),
            "sideband data at offset {offset:.3e}: got {direct}, want {expected}"
        );
    }
}

#[test]
fn pac_preserves_a_high_impedance_physical_transfer() {
    let deck = "\
* PAC must not place an implicit shunt across this 100 TOhm load
iin 0 out dc 0
r1 out 0 1e14
.end
";
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e4, 1.0e4, 1)
        .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
        .with_sidebands(0, 0)
        .with_input_source("iin")
        .with_output_node("out");

    let analysis = run_pac(deck, config);
    let transfer = analysis
        .result
        .conversion_matrix
        .get(0, 0, 0)
        .expect("conversion value is materialized");
    assert!(
        (transfer.norm() - 1.0e14).abs() <= 1.0e2,
        "a unit PAC current through 100 TOhm must produce 1e14 V, got {transfer}"
    );
}

#[test]
fn pac_conversion_matrix_measures_the_configured_differential_output() {
    let deck = "\
* unequal dividers produce a known differential output
vin in 0 dc 0 ac 1
r1 in outp 1k
r2 outp 0 1k
r3 in outn 1k
r4 outn 0 3k
.end
";
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e4, 1.0e4, 1)
        .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
        .with_sidebands(-1, 1)
        .with_input_source("vin")
        .with_output_node("outp")
        .with_output_ref("outn");

    let analysis = run_pac(deck, config);
    let differential = analysis
        .result
        .conversion_matrix
        .get(0, 0, 0)
        .expect("conversion value is materialized");

    // V(outp) = 1/2 and V(outn) = 3/4 for a unit input.
    assert!(
        (differential - Complex64::new(-0.25, 0.0)).norm() < 1.0e-9,
        "differential PAC response = {differential}"
    );
}

#[test]
fn pac_uses_exact_voltage_source_branches_at_signed_and_zero_frequencies() {
    let deck = "\
* exact ideal-source periodic MNA
vin in 0 dc 0 ac 7 37
rcouple in clamp 1u
vclamp clamp 0 dc 0
.end
";
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(5.0e5, 1.0e6, 2)
        .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
        .with_sidebands(-1, 1)
        .with_input_source("vin")
        .with_output_node("in")
        .with_output_ref("clamp");

    let analysis = run_pac(deck, config);
    let result = &analysis.result;
    for frequency_index in 0..result.frequencies.len() {
        for input_sideband in -1..=1 {
            for output_sideband in -1..=1 {
                let actual = result
                    .conversion_matrix
                    .get(frequency_index, output_sideband, input_sideband)
                    .expect("conversion value is materialized");
                let expected = if output_sideband == input_sideband {
                    Complex64::new(1.0, 0.0)
                } else {
                    Complex64::new(0.0, 0.0)
                };
                assert!(
                    (actual - expected).norm() <= 1.0e-12,
                    "H[{output_sideband},{input_sideband}] at frequency index {frequency_index} = {actual}, expected {expected}"
                );
            }
        }
    }

    let input = result.node_index("in").expect("input node retained");
    let clamp = result.node_index("clamp").expect("clamp node retained");
    for frequency_index in 0..result.frequencies.len() {
        let input_voltage = result
            .voltage(input, frequency_index, 0)
            .expect("central input voltage retained");
        let clamp_voltage = result
            .voltage(clamp, frequency_index, 0)
            .expect("central clamp voltage retained");
        assert!((input_voltage - Complex64::new(1.0, 0.0)).norm() <= 1.0e-12);
        assert!(clamp_voltage.norm() <= 1.0e-12);
    }
}

#[test]
fn pac_uses_an_exact_inductor_branch_at_negative_zero_and_positive_frequency() {
    let deck = "\
* exact inductor periodic MNA
iin 0 in dc 0
l1 in out 1u
r1 out 0 1k
.end
";
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(5.0e5, 1.0e6, 2)
        .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
        .with_sidebands(-1, 1)
        .with_input_source("iin")
        .with_output_node("in")
        .with_output_ref("out");

    let analysis = run_pac(deck, config);
    let result = &analysis.result;
    let inductance = 1.0e-6;
    for (frequency_index, &offset) in result.frequencies.iter().enumerate() {
        for input_sideband in -1..=1 {
            for output_sideband in -1..=1 {
                let actual = result
                    .conversion_matrix
                    .get(frequency_index, output_sideband, input_sideband)
                    .expect("conversion value is materialized");
                let expected = if output_sideband == input_sideband {
                    let frequency = offset + f64::from(input_sideband) * F0;
                    Complex64::new(0.0, 2.0 * std::f64::consts::PI * frequency * inductance)
                } else {
                    Complex64::new(0.0, 0.0)
                };
                assert!(
                    (actual - expected).norm() <= 1.0e-10,
                    "inductor H[{output_sideband},{input_sideband}] at offset {offset:.6e} = {actual}, expected {expected}"
                );
            }
        }
    }
}

#[test]
fn pac_retains_interleaved_canonical_branch_currents_and_orientation() {
    let deck = "\
* interleaved canonical MNA branches
VDRIVE in 0 dc 0
R1 in mid 100
LSERIES mid out 1m
VCLAMP out 0 dc 0
.end
";
    let offset = 1.0e4;
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(offset, offset, 1)
        .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
        .with_sidebands(-1, 1)
        .with_input_source("VDRIVE")
        .with_output_node("in");

    let analysis = run_pac(deck, config);
    let result = &analysis.result;
    assert_eq!(result.branch_names, ["VDRIVE", "LSERIES", "VCLAMP"]);
    let impedance = Complex64::new(100.0, std::f64::consts::TAU * offset * 1.0e-3);
    let path_current = Complex64::new(1.0, 0.0) / impedance;

    for sideband in -1..=1 {
        let data = result
            .get_sideband_data(0, sideband)
            .expect("every configured sideband is retained");
        assert_eq!(data.branch_currents.len(), result.branch_names.len());
        assert!(
            data.branch_currents
                .iter()
                .all(|value| value.re.is_finite() && value.im.is_finite())
        );
        if sideband == 0 {
            for (actual, expected) in data.branch_currents.iter().copied().zip([
                -path_current,
                path_current,
                path_current,
            ]) {
                assert!(
                    (actual - expected).norm() <= 1.0e-10 * expected.norm().max(1.0),
                    "retained branch current {actual} differs from {expected}"
                );
            }
        } else {
            assert!(
                data.branch_currents
                    .iter()
                    .all(|value| value.norm() <= 1.0e-12),
                "an LTI circuit must not create off-sideband branch current: {:?}",
                data.branch_currents
            );
        }
    }
}

#[test]
fn pac_exact_mna_fails_closed_for_unrepresented_branch_coupling() {
    let cases = [
        (
            "\
* coupled inductors require a mutual branch block
iin 0 a dc 0
l1 a 0 1u
l2 b 0 1u
r2 b 0 50
k12 l1 l2 0.9
.end
",
            "coupled-inductor",
        ),
        (
            "\
* current switch requires the actual control branch current spectrum
iin 0 out dc 0
vctrl ctrl 0 dc 0
w1 out 0 vctrl csw
.model csw iswitch (ron=1 roff=1meg ion=1 ioff=0)
r1 out 0 1k
.end
",
            "current-controlled switches",
        ),
        (
            "\
* solution-dependent capacitance requires its charge linearization
iin 0 out dc 0
vctrl ctrl 0 dc 0.5
c1 out 0 C={1p*(1+V(ctrl))}
r1 out 0 1k
.end
",
            "solution-dependent capacitor",
        ),
    ];
    for (deck, expected) in cases {
        let netlist = Netlist::parse(deck).expect("unsupported deck still parses");
        let config = PacConfig::new()
            .with_fundamental(F0)
            .with_sweep(1.0e4, 1.0e4, 1)
            .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
            .with_sidebands(0, 0)
            .with_input_source("iin")
            .with_output_node("out");
        let error = Engine::new(SimulationConfig::default())
            .run_pac(&netlist, config)
            .expect_err("an incomplete branch model must not publish PAC data");
        assert!(
            error.to_string().contains(expected),
            "failure must identify the unsupported branch family: {error}"
        );
    }
}

#[test]
fn pac_without_an_output_reports_conversion_data_as_unavailable() {
    let deck = "\
* per-node PAC without an output metric
iin 0 out dc 0
r1 out 0 1k
.end
";
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e4, 1.0e4, 1)
        .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
        .with_sidebands(-1, 1)
        .with_input_source("iin");

    let analysis = run_pac(deck, config);
    let result = &analysis.result;
    assert!(result.conversion_matrix.get(0, 0, 0).is_err());
    assert!(result.conversion_gain(0, 0, 0).is_err());
    assert!(result.conversion_gain_db(0, 0, 0).is_err());
    assert!(result.get_transfer(0, 0).is_err());
    assert!(result.image_rejection_db(0).is_err());
    let out = result.node_index("out").expect("output node is retained");
    let voltage = result
        .voltage(out, 0, 0)
        .expect("no-output PAC still retains its physical node spectrum");
    assert!(
        (voltage - Complex64::new(1.0e3, 0.0)).norm() < 1.0e-9,
        "unit current through 1 kohm must retain 1 kV, got {voltage}"
    );
}

#[test]
fn pac_rejects_a_nonrepresentable_differential_transfer() {
    let deck = "\
* each node voltage is finite, but their differential exceeds binary64
iin outn outp dc 0
r1 outp 0 1e308
r2 outn 0 1e308
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let base_config = || {
        PacConfig::new()
            .with_fundamental(F0)
            .with_sweep(1.0e4, 1.0e4, 1)
            .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
            .with_sidebands(0, 0)
            .with_input_source("iin")
    };
    for output in ["outp", "outn"] {
        let single_ended = engine
            .run_pac(&netlist, base_config().with_output_node(output))
            .expect("each single-ended extreme response remains representable");
        let value = single_ended
            .result
            .conversion_matrix
            .get(0, 0, 0)
            .expect("single-ended conversion value is materialized");
        assert!(
            value.re.is_finite() && value.im.is_finite(),
            "single-ended {output} response must be finite, got {value}"
        );
    }
    let error = engine
        .run_pac(
            &netlist,
            base_config()
                .with_output_node("outp")
                .with_output_ref("outn"),
        )
        .expect_err("an unrepresentable differential must fail closed");
    assert!(
        error
            .to_string()
            .contains("PAC differential output is non-representable"),
        "unexpected error: {error}"
    );
}

#[test]
fn pac_drives_a_physical_rf_port_through_its_reference_impedance() {
    let deck = "\
* 50 ohm series network between matched RF ports
P1 p1 0 PORT=1 Z0=50
R1 p1 p2 50
C1 p1 0 1e-18
P2 p2 0 PORT=2 Z0=50
.end
";
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e4, 1.0e4, 1)
        .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
        .with_sidebands(-1, 1)
        .with_input_source("P1")
        .with_output_node("P1");

    let analysis = run_pac(deck, config);
    let voltage = analysis
        .result
        .conversion_matrix
        .get(0, 0, 0)
        .expect("conversion value is materialized");
    assert!(
        (voltage - Complex64::new(2.0 / 3.0, 0.0)).norm() < 1.0e-8,
        "port-plane voltage = {voltage}"
    );
}

#[test]
fn retained_pss_pac_drives_a_physical_rf_port_through_z0() {
    let deck = "\
* dynamic state plus 50 ohm series network between matched RF ports
P1 p1 0 PORT=1 Z0=50
R1 p1 p2 50
C1 p1 0 1e-18
P2 p2 0 PORT=2 Z0=50
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let operating_point = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(F0)
                .with_tstab_periods(1)
                .with_points_per_period(32)
                .with_harmonics(4)
                .with_tolerance(1.0e-7),
            &NoAbort,
        )
        .expect("PSS converges");
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e4, 1.0e4, 1)
        .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
        .with_sidebands(-1, 1)
        .with_input_source("P1")
        .with_output_node("P1");

    let analysis = engine
        .run_pac_from_pss_with_abort(&netlist, config, &operating_point, &NoAbort)
        .expect("PAC consumes PSS");
    let voltage = analysis
        .result
        .conversion_matrix
        .get(0, 0, 0)
        .expect("conversion value is materialized");
    assert!(
        (voltage - Complex64::new(2.0 / 3.0, 0.0)).norm() < 1.0e-8,
        "port-plane voltage = {voltage}"
    );
}

#[test]
fn retained_hb_pac_drives_a_physical_rf_port_through_z0() {
    let deck = "\
* 50 ohm series network between matched RF ports
P1 p1 0 PORT=1 Z0=50
R1 p1 p2 50
C1 p1 0 1e-18
P2 p2 0 PORT=2 Z0=50
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let hb = engine
        .run_hb(
            &netlist,
            rspice_core::analysis::HbConfig::new(F0).with_harmonics(8),
        )
        .expect("HB converges");
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e4, 1.0e4, 1)
        .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
        .with_sidebands(-1, 1)
        .with_input_source("P1")
        .with_output_node("P1");

    let analysis = engine
        .run_pac_from_hb_with_abort(&netlist, config, &hb.operating_point, &NoAbort)
        .expect("PAC consumes retained HB state");
    let voltage = analysis
        .result
        .conversion_matrix
        .get(0, 0, 0)
        .expect("conversion value is materialized");
    assert!(
        (voltage - Complex64::new(2.0 / 3.0, 0.0)).norm() < 1.0e-8,
        "port-plane voltage = {voltage}"
    );
}

#[test]
fn switch_chopper_conversion_gain_converges_to_the_square_wave_coefficients() {
    // LO-driven series switch: the small-signal transfer rfin -> out toggles
    // between Rload/(Rload + ron) ~ 0.999 and ~1e-6 at 50% duty. The exact
    // LTV answer has |H[+/-1, 0]| = A/pi and H[0,0] = A/2; the truncated
    // conversion matrix approaches it as the sideband count grows (hard
    // switching converges ~1/K, exactly as in commercial conversion-matrix
    // solvers), so the test pins both the K=16 values and the convergence.
    let deck = "\
* series chopper mixer
vlo ctl 0 sin(0 1 1meg)
vrf rfin 0 dc 0
s1 rfin out ctl 0 swmod
rload out 0 1k
cload out 0 1f
.model swmod sw vt=0 ron=1 roff=1e9 smooth=1m
.end
";
    let chopper_h = |k: i32| -> (f64, f64, f64) {
        let config = PacConfig::new()
            .with_fundamental(F0)
            .with_sweep(1.0e4, 1.0e4, 1)
            .with_sweep_type(rspice_core::analysis::pac::PacSweepType::Linear)
            .with_sidebands(-k, k)
            .with_input_source("vrf")
            .with_output_node("out");
        let analysis = run_pac(deck, config);
        assert!(analysis.converged, "operating point must converge");
        let cm = &analysis.result.conversion_matrix;
        (
            cm.get(0, 1, 0)
                .expect("conversion value is materialized")
                .norm(),
            cm.get(0, -1, 0)
                .expect("conversion value is materialized")
                .norm(),
            cm.get(0, 0, 0)
                .expect("conversion value is materialized")
                .norm(),
        )
    };

    let a = 1000.0 / 1001.0; // on-state divider ratio
    let fundamental_exact = a / std::f64::consts::PI;
    let direct_exact = a / 2.0;

    let (h1_coarse, _, h0_coarse) = chopper_h(3);
    let (h1, h1_down, h0) = chopper_h(16);

    assert!(
        (h1 - fundamental_exact).abs() < 0.01 * fundamental_exact,
        "K=16 upper-sideband conversion must be within 1% of A/pi: got {h1:.5}, want {fundamental_exact:.5}"
    );
    assert!(
        (h1_down - fundamental_exact).abs() < 0.01 * fundamental_exact,
        "K=16 lower-sideband conversion must be within 1% of A/pi: got {h1_down:.5}"
    );
    assert!(
        (h0 - direct_exact).abs() < 0.08 * direct_exact,
        "K=16 direct feedthrough must approach A/2: got {h0:.5}, want {direct_exact:.5}"
    );

    // Truncation error must shrink as sidebands are added.
    assert!(
        (h1 - fundamental_exact).abs() < (h1_coarse - fundamental_exact).abs() / 3.0,
        "fundamental conversion must converge with sideband count: K=3 {h1_coarse:.5}, K=16 {h1:.5}"
    );
    assert!(
        (h0 - direct_exact).abs() < (h0_coarse - direct_exact).abs() / 2.0,
        "direct feedthrough must converge with sideband count: K=3 {h0_coarse:.5}, K=16 {h0:.5}"
    );
}
