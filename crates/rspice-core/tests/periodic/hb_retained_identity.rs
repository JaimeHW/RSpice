//! Authentication contracts for retained harmonic-balance operating points.

use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::HbConfig;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::engine::{Engine, HbOperatingPoint, SimulationConfig, SimulationError};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;

#[derive(Clone, Copy)]
struct DeckValues {
    resistance: f64,
    inductance: f64,
    source_amplitude: f64,
    model_vto: f64,
    temperature: f64,
    rshunt: f64,
}

const BASE: DeckValues = DeckValues {
    resistance: 100.0,
    inductance: 1.0e-6,
    source_amplitude: 1.0,
    model_vto: 0.5,
    temperature: 27.0,
    rshunt: 1.0e9,
};

fn deck(values: DeckValues) -> String {
    format!(
        "* retained HB semantic identity fixture\n\
         VIN in 0 DC {source:e}\n\
         R1 in mid {resistance:e}\n\
         L1 mid out {inductance:e}\n\
         RLOAD out 0 1k\n\
         .model NMOD NMOS LEVEL=1 VTO={model_vto:e} KP=1m\n\
         .options temp={temperature:e} rshunt={rshunt:e}\n\
         .end\n",
        source = values.source_amplitude,
        resistance = values.resistance,
        inductance = values.inductance,
        model_vto = values.model_vto,
        temperature = values.temperature,
        rshunt = values.rshunt,
    )
}

fn parse(values: DeckValues) -> Netlist {
    Netlist::parse(&deck(values)).expect("semantic identity fixture parses")
}

fn hb_config() -> HbConfig {
    HbConfig::new(F0).with_harmonics(8)
}

fn pac_config() -> PacConfig {
    PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e4, 1.0e4, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(0, 0)
        .with_input_source("VIN")
        .with_output_node("out")
}

fn assert_identity_rejection(error: impl std::fmt::Display, label: &str) {
    let message = error.to_string();
    assert!(
        message.contains("retained HB semantic circuit identity"),
        "{label} must fail on retained semantic identity, got: {message}"
    );
}

#[test]
fn retained_hb_accepts_an_identical_clone_and_rejects_same_name_semantic_drift() {
    let engine = Engine::new(SimulationConfig::default());
    let producer_deck = parse(BASE);
    let analysis = engine
        .run_hb(&producer_deck, hb_config())
        .expect("linear HB producer solve completes");
    assert!(analysis.operating_point.producer_identity().is_some());

    let identical_clone = parse(BASE);
    engine
        .run_pac_from_hb_with_abort(
            &identical_clone,
            pac_config(),
            &analysis.operating_point,
            &NoAbort,
        )
        .expect("an identically elaborated cloned deck accepts the retained state");

    let mut drifts = Vec::new();
    let mut values = BASE;
    values.resistance = 101.0;
    drifts.push(("resistance", values));
    values = BASE;
    values.inductance = 1.1e-6;
    drifts.push(("inductance", values));
    values = BASE;
    values.source_amplitude = 1.1;
    drifts.push(("source amplitude", values));
    values = BASE;
    values.model_vto = 0.55;
    drifts.push(("model parameter", values));
    values = BASE;
    values.temperature = 35.0;
    drifts.push(("TEMP", values));
    values = BASE;
    values.rshunt = 2.0e9;
    drifts.push(("RSHUNT", values));

    for (label, drifted_values) in drifts {
        let error = engine
            .run_pac_from_hb_with_abort(
                &parse(drifted_values),
                pac_config(),
                &analysis.operating_point,
                &NoAbort,
            )
            .expect_err("same-name semantic drift must not reuse a retained HB state");
        assert_identity_rejection(error, label);
    }
}

#[test]
fn retained_hb_pnoise_authenticates_the_current_deck() {
    let engine = Engine::new(SimulationConfig::default());
    let producer_deck = parse(BASE);
    let analysis = engine
        .run_hb(&producer_deck, hb_config())
        .expect("linear HB producer solve completes");

    engine
        .run_pnoise_from_hb_with_abort(
            &parse(BASE),
            &[1.0e4],
            "out",
            None,
            Some("VIN"),
            0,
            &analysis.operating_point,
            &NoAbort,
        )
        .expect("pnoise accepts an identically elaborated cloned deck");

    let mut drifted = BASE;
    drifted.resistance = 101.0;
    let error = engine
        .run_pnoise_from_hb_with_abort(
            &parse(drifted),
            &[1.0e4],
            "out",
            None,
            Some("VIN"),
            0,
            &analysis.operating_point,
            &NoAbort,
        )
        .expect_err("pnoise must reject retained state from a changed deck");
    assert_identity_rejection(error, "pnoise resistance");
}

#[test]
fn retained_hb_rejects_an_identical_deck_under_a_different_resolved_engine_config() {
    let producer = Engine::new(SimulationConfig::default());
    let netlist = parse(BASE);
    let analysis = producer
        .run_hb(&netlist, hb_config())
        .expect("linear HB producer solve completes");

    let mut changed_config = SimulationConfig::default();
    changed_config.convergence_config.gmin_target *= 10.0;
    let consumer = Engine::new(changed_config);
    let error = consumer
        .run_pac_from_hb_with_abort(
            &parse(BASE),
            pac_config(),
            &analysis.operating_point,
            &NoAbort,
        )
        .expect_err("different resolved engine configuration must not reuse retained HB state");
    assert!(
        error
            .to_string()
            .contains("retained HB resolved simulation configuration"),
        "rejection should identify resolved engine configuration drift: {error}"
    );
}

#[test]
fn identityless_compatibility_artifacts_fail_closed_at_dependent_solve() {
    let engine = Engine::new(SimulationConfig::default());
    let netlist = parse(BASE);
    let analysis = engine
        .run_hb(&netlist, hb_config())
        .expect("linear HB producer solve completes");
    let point = &analysis.operating_point;
    let legacy = HbOperatingPoint::try_from_parts_with_mna_branches(
        point.config().clone(),
        point.node_names().to_vec(),
        point.spectral_state().to_vec(),
        point.mna_branch_names().to_vec(),
        point.mna_branch_spectral_state().to_vec(),
        point.iterations(),
        point.residual_norm(),
    )
    .expect("legacy artifact remains structurally parseable");
    assert!(legacy.producer_identity().is_none());

    let error = engine
        .run_pac_from_hb_with_abort(&netlist, pac_config(), &legacy, &NoAbort)
        .expect_err("identityless retained state must fail closed");
    assert!(
        error.to_string().contains("legacy identityless artifact"),
        "typed rejection should identify the trust failure: {error}"
    );
}

#[test]
fn copied_identity_cannot_authenticate_tampered_retained_payloads() {
    let engine = Engine::new(SimulationConfig::default());
    let netlist = parse(BASE);
    let analysis = engine
        .run_hb(&netlist, hb_config())
        .expect("linear HB producer solve completes");
    let point = &analysis.operating_point;
    let identity = point
        .producer_identity()
        .cloned()
        .expect("production HB state has producer identity");

    let reconstruct = |config: HbConfig,
                       node_names: Vec<String>,
                       spectral_state: Vec<Vec<num_complex::Complex64>>,
                       branch_names: Vec<String>,
                       branch_state: Vec<Vec<num_complex::Complex64>>| {
        HbOperatingPoint::try_from_authenticated_parts_with_mna_branches(
            identity.clone(),
            config,
            node_names,
            spectral_state,
            branch_names,
            branch_state,
            point.iterations(),
            point.residual_norm(),
        )
    };
    let assert_payload_rejected = |result: Result<HbOperatingPoint, SimulationError>,
                                   label: &str| {
        let error = result.expect_err("tampered retained payload must be rejected");
        assert!(
            error
                .to_string()
                .contains("numerical payload does not match"),
            "{label} should identify the authenticated payload mismatch: {error}"
        );
    };

    let mut node_state = point.spectral_state().to_vec();
    node_state[0][0].re += 0.25;
    assert_payload_rejected(
        reconstruct(
            point.config().clone(),
            point.node_names().to_vec(),
            node_state,
            point.mna_branch_names().to_vec(),
            point.mna_branch_spectral_state().to_vec(),
        ),
        "node coefficient",
    );

    let mut node_names = point.node_names().to_vec();
    node_names[0] = "renamed".to_owned();
    assert_payload_rejected(
        reconstruct(
            point.config().clone(),
            node_names,
            point.spectral_state().to_vec(),
            point.mna_branch_names().to_vec(),
            point.mna_branch_spectral_state().to_vec(),
        ),
        "node name",
    );

    let mut branch_state = point.mna_branch_spectral_state().to_vec();
    branch_state[0][0].re += 0.25;
    assert_payload_rejected(
        reconstruct(
            point.config().clone(),
            point.node_names().to_vec(),
            point.spectral_state().to_vec(),
            point.mna_branch_names().to_vec(),
            branch_state,
        ),
        "branch coefficient",
    );

    let mut branch_names = point.mna_branch_names().to_vec();
    branch_names[0] = "VOTHER".to_owned();
    assert_payload_rejected(
        reconstruct(
            point.config().clone(),
            point.node_names().to_vec(),
            point.spectral_state().to_vec(),
            branch_names,
            point.mna_branch_spectral_state().to_vec(),
        ),
        "branch name",
    );

    let mut config = point.config().clone();
    config.damping = 0.75;
    assert_payload_rejected(
        reconstruct(
            config,
            point.node_names().to_vec(),
            point.spectral_state().to_vec(),
            point.mna_branch_names().to_vec(),
            point.mna_branch_spectral_state().to_vec(),
        ),
        "HB configuration",
    );

    assert_payload_rejected(
        HbOperatingPoint::try_from_authenticated_parts_with_mna_branches(
            identity.clone(),
            point.config().clone(),
            point.node_names().to_vec(),
            point.spectral_state().to_vec(),
            point.mna_branch_names().to_vec(),
            point.mna_branch_spectral_state().to_vec(),
            point.iterations().saturating_add(1),
            point.residual_norm(),
        ),
        "iteration count",
    );
    assert_payload_rejected(
        HbOperatingPoint::try_from_authenticated_parts_with_mna_branches(
            identity,
            point.config().clone(),
            point.node_names().to_vec(),
            point.spectral_state().to_vec(),
            point.mna_branch_names().to_vec(),
            point.mna_branch_spectral_state().to_vec(),
            point.iterations(),
            point.residual_norm() + f64::EPSILON,
        ),
        "residual norm",
    );
}
