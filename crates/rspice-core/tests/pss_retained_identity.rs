//! Semantic and payload authentication for retained shooting-PSS states.

use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::analysis::pss::PssConfig;
use rspice_core::engine::{Engine, PssOperatingPoint, SimulationConfig, SimulationError};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;

fn deck(resistance: f64, source: f64, capacitance: f64) -> String {
    format!(
        "* retained PSS identity fixture\n\
         V1 in 0 DC {source:e}\n\
         R1 in out {resistance:e}\n\
         R2 out 0 1k\n\
         C1 out 0 {capacitance:e}\n\
         .end\n"
    )
}

fn parse(resistance: f64, source: f64, capacitance: f64) -> Netlist {
    Netlist::parse(&deck(resistance, source, capacitance)).expect("PSS identity fixture parses")
}

fn pss_config() -> PssConfig {
    PssConfig::new(F0)
        .with_harmonics(4)
        .with_points_per_period(32)
        .with_tstab_periods(0)
        .with_tolerance(1.0e-7)
}

fn pac_config() -> PacConfig {
    PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e4, 1.0e4, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(0, 0)
        .with_input_source("V1")
        .with_output_node("out")
}

fn produced() -> (Engine, Netlist, PssOperatingPoint) {
    let engine = Engine::new(SimulationConfig::default());
    let netlist = parse(1.0e3, 1.0, 1.0e-12);
    let point = engine
        .run_pss_operating_point_with_abort(&netlist, pss_config(), &NoAbort)
        .expect("linear shooting-PSS fixture converges");
    (engine, netlist, point)
}

#[test]
fn retained_pss_accepts_identical_consumers_and_rejects_semantic_or_engine_drift() {
    let (engine, _, point) = produced();
    assert!(point.producer_identity().is_some());
    assert_eq!(point.shooting_state_basis(), ["C:C1"]);

    engine
        .run_pac_from_pss_with_abort(&parse(1.0e3, 1.0, 1.0e-12), pac_config(), &point, &NoAbort)
        .expect("PAC accepts an identically elaborated cloned PSS producer deck");
    engine
        .run_pnoise_from_pss_with_abort(
            &parse(1.0e3, 1.0, 1.0e-12),
            &[1.0e4],
            "out",
            None,
            Some("V1"),
            0,
            &point,
            &NoAbort,
        )
        .expect("PNoise accepts an identically elaborated cloned PSS producer deck");

    for (label, netlist) in [
        ("resistance", parse(1.1e3, 1.0, 1.0e-12)),
        ("source amplitude", parse(1.0e3, 1.1, 1.0e-12)),
        ("reactive value", parse(1.0e3, 1.0, 1.1e-12)),
    ] {
        let error = engine
            .run_pac_from_pss_with_abort(&netlist, pac_config(), &point, &NoAbort)
            .expect_err("semantic producer drift must reject retained PSS reuse");
        assert!(
            error
                .to_string()
                .contains("retained PSS semantic circuit identity"),
            "{label} drift should identify the semantic mismatch: {error}"
        );
    }

    let mut changed_config = SimulationConfig::default();
    changed_config.convergence_config.gmin_target *= 10.0;
    let error = Engine::new(changed_config)
        .run_pac_from_pss_with_abort(&parse(1.0e3, 1.0, 1.0e-12), pac_config(), &point, &NoAbort)
        .expect_err("resolved engine drift must reject retained PSS reuse");
    assert!(
        error
            .to_string()
            .contains("retained PSS resolved simulation configuration"),
        "rejection should identify resolved engine configuration drift: {error}"
    );
}

#[test]
fn identityless_pss_artifacts_parse_but_fail_closed_for_all_dependent_reuse() {
    let (engine, netlist, point) = produced();
    let legacy = PssOperatingPoint::try_from_parts(
        point.config().clone(),
        point.analysis().clone(),
        point.shooting_state().to_vec(),
    )
    .expect("legacy PSS artifact remains structurally parseable");
    assert!(legacy.producer_identity().is_none());
    assert!(legacy.shooting_state_basis().is_empty());

    let pac_error = engine
        .run_pac_from_pss_with_abort(&netlist, pac_config(), &legacy, &NoAbort)
        .expect_err("PAC must reject an identityless PSS artifact");
    assert!(
        pac_error
            .to_string()
            .contains("legacy identityless artifact")
    );

    let oscillator_config = PssConfig::autonomous()
        .with_period_guess(1.0 / F0)
        .with_harmonics(4)
        .with_points_per_period(32);
    let oscillator_error = engine
        .run_pnoise_oscillator_from_pss_with_abort(
            &netlist,
            oscillator_config,
            &[1.0e4],
            &legacy,
            &NoAbort,
        )
        .expect_err("oscillator PNoise must reject an identityless retained PSS state");
    assert!(
        oscillator_error
            .to_string()
            .contains("legacy identityless artifact"),
        "oscillator reuse should fail at producer authentication: {oscillator_error}"
    );
}

#[test]
fn copied_pss_identity_cannot_authenticate_payload_or_basis_tamper() {
    let (_, _, point) = produced();
    let identity = point
        .producer_identity()
        .cloned()
        .expect("production PSS state has producer identity");
    let reconstruct = |config: PssConfig,
                       analysis: rspice_core::engine::PssAnalysisResult,
                       basis: Vec<String>,
                       state: Vec<f64>| {
        PssOperatingPoint::try_from_authenticated_parts(
            identity.clone(),
            config,
            analysis,
            basis,
            state,
        )
    };
    let assert_payload_rejected = |result: Result<PssOperatingPoint, SimulationError>,
                                   label: &str| {
        let error = result.expect_err("tampered retained PSS payload must be rejected");
        assert!(
            error
                .to_string()
                .contains("numerical payload does not match"),
            "{label} should identify retained payload authentication failure: {error}"
        );
    };

    let mut analysis = point.analysis().clone();
    analysis.result.waveforms[0].values[0] += 0.25;
    assert_payload_rejected(
        reconstruct(
            point.config().clone(),
            analysis,
            point.shooting_state_basis().to_vec(),
            point.shooting_state().to_vec(),
        ),
        "orbit waveform",
    );

    let mut analysis = point.analysis().clone();
    analysis.monodromy[0][0] += 0.01;
    assert_payload_rejected(
        reconstruct(
            point.config().clone(),
            analysis,
            point.shooting_state_basis().to_vec(),
            point.shooting_state().to_vec(),
        ),
        "monodromy",
    );

    let mut state = point.shooting_state().to_vec();
    state[0] += 0.125;
    assert_payload_rejected(
        reconstruct(
            point.config().clone(),
            point.analysis().clone(),
            point.shooting_state_basis().to_vec(),
            state,
        ),
        "shooting state",
    );

    let mut basis = point.shooting_state_basis().to_vec();
    basis[0] = "C:COTHER".to_owned();
    assert_payload_rejected(
        reconstruct(
            point.config().clone(),
            point.analysis().clone(),
            basis,
            point.shooting_state().to_vec(),
        ),
        "shooting basis",
    );

    let mut config = point.config().clone();
    config.damping_factor = 0.75;
    assert_payload_rejected(
        reconstruct(
            config,
            point.analysis().clone(),
            point.shooting_state_basis().to_vec(),
            point.shooting_state().to_vec(),
        ),
        "PSS configuration",
    );
}
