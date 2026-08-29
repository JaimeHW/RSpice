//! Dependent-analysis contracts for an exact nonlinear-HB operating point.
//!
//! The one-microohm gate shunt makes a fixed one-microohm Norton replacement
//! observably wrong: it biases the gate at 0.5 V instead of enforcing VIN's
//! ideal 1 V constraint. Ordinary AC/noise provide an independent MNA oracle
//! for direct PAC/PNoise and for consumers of a retained HB state.

use num_complex::Complex64;
use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::HbConfig;
use rspice_core::analysis::NoiseContributionProbe;
use rspice_core::analysis::ac::AcResult;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::constants::K_BOLTZMANN;
use rspice_core::engine::{Engine, PacAnalysisResult, PnoiseAnalysisResult, SimulationConfig};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;
const OFFSET: f64 = 1.0e4;
const TEMPERATURE: f64 = 300.15;
const RSH: f64 = 1.0e-6;
const RD: f64 = 1.0e3;
const GM: f64 = 5.0e-4;
const GAIN: f64 = -GM * RD;

fn static_mos_deck(with_ac_drive: bool) -> String {
    let input = if with_ac_drive {
        "VIN gate 0 DC 1 AC 1"
    } else {
        "VIN gate 0 DC 1"
    };
    format!(
        "* exact-MNA static MOS dependent-analysis oracle\n\
         {input}\n\
         VDD vdd 0 DC 2\n\
         RSH gate 0 {RSH:e}\n\
         RD vdd drain {RD:e}\n\
         M1 drain gate 0 0 NMOD L=1u W=1u\n\
         .model NMOD NMOS LEVEL=1 VTO=0.5 KP=1m LAMBDA=0 \
                 CGSO=0 CGDO=0 CGBO=0 CJ=0 CJSW=0 KF=0\n\
         .end\n"
    )
}

fn parse(with_ac_drive: bool) -> Netlist {
    Netlist::parse(&static_mos_deck(with_ac_drive)).expect("static MOS fixture parses")
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn ac_voltage(point: &AcResult, node: &str) -> Complex64 {
    let index = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("missing AC node '{node}' in {:?}", point.node_names));
    point.voltages[index]
}

fn pac_gain(analysis: &PacAnalysisResult) -> Complex64 {
    analysis
        .result
        .conversion_matrix
        .get(0, 0, 0)
        .expect("PAC central conversion gain is materialized")
}

fn pac_config() -> PacConfig {
    PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(OFFSET, OFFSET, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(0, 0)
        .with_input_source("VIN")
        .with_output_node("drain")
}

fn assert_complex_relative(actual: Complex64, expected: Complex64, reltol: f64, label: &str) {
    let error = (actual - expected).norm();
    let tolerance = reltol * expected.norm().max(1.0);
    assert!(
        error <= tolerance,
        "{label}: actual={actual}, expected={expected}, error={error:.3e}, tolerance={tolerance:.3e}"
    );
}

fn assert_relative(actual: f64, expected: f64, reltol: f64, label: &str) {
    let error = (actual - expected).abs();
    let tolerance = reltol * expected.abs().max(f64::MIN_POSITIVE);
    assert!(
        actual.is_finite() && error <= tolerance,
        "{label}: actual={actual:.17e}, expected={expected:.17e}, relative error={:.3e}",
        error / expected.abs().max(f64::MIN_POSITIVE)
    );
}

fn branch_index(names: &[String], name: &str) -> usize {
    names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("missing branch '{name}' in {names:?}"))
}

fn assert_pac_exact_source_evidence(analysis: &PacAnalysisResult) {
    assert_eq!(analysis.result.branch_names, ["VIN", "VDD"]);
    let gate = analysis.result.node_index("gate").expect("PAC gate node");
    let sideband = analysis
        .result
        .get_sideband_data(0, 0)
        .expect("PAC central sideband is retained");
    assert_complex_relative(
        sideband.node_voltages[gate],
        Complex64::new(1.0, 0.0),
        1.0e-12,
        "PAC ideal input constraint",
    );

    let vin = sideband.branch_currents[branch_index(&analysis.result.branch_names, "VIN")];
    let vdd = sideband.branch_currents[branch_index(&analysis.result.branch_names, "VDD")];
    assert_relative(vin.re, -1.0 / RSH, 1.0e-10, "PAC VIN branch current");
    assert_relative(vdd.re, GAIN / RD, 1.0e-9, "PAC VDD branch current");
}

fn ordinary_ac_oracle() -> Complex64 {
    let point = engine()
        .run_ac(&parse(true), &[OFFSET])
        .expect("ordinary AC completes")
        .pop()
        .expect("one ordinary AC point");
    assert_eq!(point.branch_names, ["VIN", "VDD"]);
    assert_complex_relative(
        ac_voltage(&point, "gate"),
        Complex64::new(1.0, 0.0),
        1.0e-12,
        "ordinary AC ideal input constraint",
    );
    let gain = ac_voltage(&point, "drain");
    assert_complex_relative(
        gain,
        Complex64::new(GAIN, 0.0),
        1.0e-9,
        "ordinary AC common-source gain",
    );
    gain
}

#[test]
fn direct_pac_matches_the_exact_static_mos_ac_oracle() {
    let ac_gain = ordinary_ac_oracle();
    let pac = engine()
        .run_pac(&parse(false), pac_config())
        .expect("direct PAC completes");

    assert!(pac.converged);
    assert_complex_relative(
        pac_gain(&pac),
        Complex64::new(GAIN, 0.0),
        1.0e-9,
        "direct PAC common-source gain",
    );
    assert_complex_relative(pac_gain(&pac), ac_gain, 1.0e-11, "direct PAC/AC parity");
    assert_pac_exact_source_evidence(&pac);
}

#[test]
fn retained_hb_pac_preserves_exact_clamps_branches_and_ac_gain() {
    let netlist = parse(false);
    let simulator = engine();
    let hb = simulator
        .run_hb(&netlist, HbConfig::new(F0).with_harmonics(8))
        .expect("static nonlinear HB completes");
    let retained = simulator
        .run_pac_from_hb_with_abort(&netlist, pac_config(), &hb.operating_point, &NoAbort)
        .expect("PAC consumes exact retained HB state");
    let direct = simulator
        .run_pac(&netlist, pac_config())
        .expect("direct PAC completes for retained-state parity");

    let gate = hb
        .operating_point
        .node_names()
        .iter()
        .position(|name| name.eq_ignore_ascii_case("gate"))
        .expect("retained HB gate node");
    assert_complex_relative(
        hb.operating_point.spectral_state()[gate][0],
        Complex64::new(1.0, 0.0),
        1.0e-12,
        "retained HB ideal input constraint",
    );
    assert_eq!(hb.operating_point.mna_branch_names(), ["VIN", "VDD"]);
    let vin = branch_index(hb.operating_point.mna_branch_names(), "VIN");
    let vdd = branch_index(hb.operating_point.mna_branch_names(), "VDD");
    assert_eq!(hb.operating_point.mna_branch_spectral_state()[vin].len(), 9);
    assert_relative(
        hb.operating_point.mna_branch_spectral_state()[vin][0].re,
        -1.0 / RSH,
        1.0e-10,
        "retained HB VIN DC branch current",
    );
    assert_relative(
        hb.operating_point.mna_branch_spectral_state()[vdd][0].re,
        -1.25e-4,
        1.0e-8,
        "retained HB VDD DC branch current",
    );
    assert!(
        hb.operating_point
            .mna_branch_spectral_state()
            .iter()
            .flat_map(|spectrum| spectrum.iter().skip(1))
            .all(|coefficient| coefficient.norm() <= 1.0e-12),
        "a static HB fixture must retain zero non-DC branch harmonics"
    );

    assert_pac_exact_source_evidence(&retained);
    assert_complex_relative(
        pac_gain(&retained),
        Complex64::new(GAIN, 0.0),
        1.0e-9,
        "retained-HB PAC common-source gain",
    );
    assert_complex_relative(
        pac_gain(&retained),
        pac_gain(&direct),
        1.0e-12,
        "direct/retained PAC parity",
    );
}

fn expected_noise() -> (f64, f64, f64) {
    let resistor = 4.0 * K_BOLTZMANN * TEMPERATURE * RD;
    let channel = (8.0 / 3.0) * K_BOLTZMANN * TEMPERATURE * GM * RD * RD;
    (resistor, channel, resistor + channel)
}

fn ordinary_noise_oracle() -> (f64, f64, f64) {
    let result = engine()
        .run_noise_named_with_input_source(
            &parse(true),
            "drain",
            None,
            "VIN",
            &[OFFSET],
            TEMPERATURE,
        )
        .expect("ordinary input-referred noise completes")
        .pop()
        .expect("one ordinary noise point");
    let output = |probe: &str| {
        result
            .contribution(&NoiseContributionProbe::parse(probe).expect("DNO probe parses"))
            .unwrap_or_else(|error| panic!("ordinary contribution {probe} is available: {error}"))
    };
    let (expected_rd, expected_channel, expected_total) = expected_noise();

    assert_relative(output("DNO(RD)"), expected_rd, 1.0e-8, "ordinary RD noise");
    assert_relative(
        output("DNO(M1,ID)"),
        expected_channel,
        1.0e-8,
        "ordinary MOS channel noise",
    );
    assert_eq!(
        output("DNO(RSH)"),
        0.0,
        "the ideal VIN clamp must reject all RSH noise at the drain"
    );
    assert_relative(
        result.output_noise_density,
        expected_total,
        1.0e-8,
        "ordinary total output noise",
    );
    assert_relative(
        result.input_gain_squared,
        GAIN * GAIN,
        1.0e-8,
        "ordinary input gain squared",
    );
    assert_relative(
        result.input_referred_density,
        expected_total / (GAIN * GAIN),
        1.0e-8,
        "ordinary input-referred noise",
    );
    (
        result.output_noise_density,
        result.input_referred_density,
        result.input_gain_squared,
    )
}

fn pnoise_contribution(result: &PnoiseAnalysisResult, name: &str) -> f64 {
    result
        .contributors
        .iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| {
            panic!(
                "missing PNoise contributor '{name}' in {:?}",
                result.contributors
            )
        })
        .1[0]
}

fn assert_pnoise_oracle(result: &PnoiseAnalysisResult, label: &str) {
    let (expected_rd, expected_channel, expected_total) = expected_noise();
    assert!(result.converged, "{label}: operating point must converge");
    assert_relative(
        pnoise_contribution(result, "RD thermal"),
        expected_rd,
        1.0e-8,
        &format!("{label} RD noise"),
    );
    assert_relative(
        pnoise_contribution(result, "Nmos#0 channel thermal"),
        expected_channel,
        1.0e-8,
        &format!("{label} MOS channel noise"),
    );
    assert_eq!(
        pnoise_contribution(result, "RSH thermal"),
        0.0,
        "{label}: the ideal VIN clamp must reject all RSH noise at the drain"
    );
    assert_relative(
        result.output_noise[0],
        expected_total,
        1.0e-8,
        &format!("{label} total output noise"),
    );
    assert_relative(
        result.input_noise.as_ref().expect("input-referred PNoise")[0],
        expected_total / (GAIN * GAIN),
        1.0e-8,
        &format!("{label} input-referred noise"),
    );
}

#[test]
fn direct_pnoise_matches_ordinary_noise_and_the_static_mos_oracle() {
    let (ordinary_output, ordinary_input, _) = ordinary_noise_oracle();
    let direct = engine()
        .run_pnoise(&parse(false), F0, &[OFFSET], "drain", None, Some("VIN"), 0)
        .expect("direct PNoise completes");

    assert_pnoise_oracle(&direct, "direct PNoise");
    assert_relative(
        direct.output_noise[0],
        ordinary_output,
        1.0e-11,
        "ordinary/direct output-noise parity",
    );
    assert_relative(
        direct.input_noise.as_ref().expect("direct input noise")[0],
        ordinary_input,
        1.0e-11,
        "ordinary/direct input-noise parity",
    );
}

#[test]
fn retained_hb_pnoise_preserves_contributors_and_matches_direct() {
    let netlist = parse(false);
    let simulator = engine();
    let hb = simulator
        .run_hb(&netlist, HbConfig::new(F0).with_harmonics(8))
        .expect("static nonlinear HB completes");
    let retained = simulator
        .run_pnoise_from_hb_with_abort(
            &netlist,
            &[OFFSET],
            "drain",
            None,
            Some("VIN"),
            0,
            &hb.operating_point,
            &NoAbort,
        )
        .expect("PNoise consumes exact retained HB state");

    // Qualify retained-state noise before asking the direct path for parity,
    // so a direct-path regression cannot hide the retained evidence.
    assert_pnoise_oracle(&retained, "retained-HB PNoise");
    let direct = simulator
        .run_pnoise(&netlist, F0, &[OFFSET], "drain", None, Some("VIN"), 0)
        .expect("direct PNoise completes for retained-state parity");

    assert_eq!(retained.frequencies, direct.frequencies);
    assert_eq!(retained.contributors.len(), direct.contributors.len());
    for ((retained_name, retained_values), (direct_name, direct_values)) in
        retained.contributors.iter().zip(&direct.contributors)
    {
        assert_eq!(retained_name, direct_name);
        assert_relative(
            retained_values[0],
            direct_values[0],
            1.0e-12,
            &format!("direct/retained contribution parity for {retained_name}"),
        );
    }
    assert_relative(
        retained.output_noise[0],
        direct.output_noise[0],
        1.0e-12,
        "direct/retained output-noise parity",
    );
    assert_relative(
        retained.input_noise.as_ref().expect("retained input noise")[0],
        direct.input_noise.as_ref().expect("direct input noise")[0],
        1.0e-12,
        "direct/retained input-noise parity",
    );
}
