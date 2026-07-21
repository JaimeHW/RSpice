use num_complex::Complex64;
use rspice_core::AtomicAbort;
use rspice_core::analysis::advanced::harmonic_balance::{
    HbConfig, HbContinuationLimitation, HbReactiveKind,
};
use rspice_core::engine::{Engine, SimulationConfig, SimulationError};
use rspice_core::netlist::Netlist;
use std::f64::consts::{PI, TAU};

fn run(deck: &str, fundamental: f64, harmonics: usize) -> rspice_core::engine::HbAnalysisResult {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .run_hb(
            &netlist,
            HbConfig::new(fundamental).with_harmonics(harmonics),
        )
        .expect("HB completes")
}

fn evaluate(coefficients: &[Complex64], phase: f64) -> f64 {
    coefficients.iter().enumerate().skip(1).fold(
        coefficients[0].re,
        |value, (harmonic, coefficient)| {
            value + (*coefficient * Complex64::from_polar(1.0, harmonic as f64 * phase)).re
        },
    )
}

#[test]
fn linear_hb_retains_named_mna_current_and_exact_capacitor_state() {
    let f0 = 1.0e6;
    let capacitance = 1.0 / (TAU * f0 * 1.0e3);
    let deck = format!(
        "retained HB state\n\
         VDRIVE in 0 sin(0 1 {f0})\n\
         R1 in out 1k\n\
         CLOAD out 0 {capacitance:e}\n\
         .end\n"
    );
    let analysis = run(&deck, f0, 4);
    assert!(analysis.converged);
    let result = &analysis.result;

    let source = result
        .mna_branch_currents
        .iter()
        .find(|branch| branch.device_name.eq_ignore_ascii_case("VDRIVE"))
        .expect("actual MNA source branch retained by authored name");
    assert_eq!(source.coefficients.len(), 5);
    assert!(source.coefficients[1].norm() > 1.0e-6);

    let capacitor = result
        .reactive_spectra
        .iter()
        .find(|branch| branch.device_name.eq_ignore_ascii_case("CLOAD"))
        .expect("capacitor state retained");
    assert_eq!(capacitor.kind, HbReactiveKind::Capacitor);
    assert!(capacitor.dc_current_is_exact);
    let expected_current =
        Complex64::new(0.0, TAU * f0 * capacitance) * capacitor.voltage_coefficients[1];
    assert!((capacitor.current_coefficients[1] - expected_current).norm() < 1.0e-14);
    assert!(result.continuation_limitations.is_empty());

    let phase = PI / 3.0;
    let projected = result.project_phase(phase).expect("finite phase projects");
    assert!(result.project_phase(f64::NAN).is_err());
    assert!(projected.is_complete());
    let projected_capacitor = projected
        .reactive_states
        .iter()
        .find(|state| state.device_name.eq_ignore_ascii_case("CLOAD"))
        .expect("projected capacitor state");
    assert!(
        (projected_capacitor.voltage - evaluate(&capacitor.voltage_coefficients, phase)).abs()
            < 1.0e-14
    );
    assert!(
        (projected_capacitor.current - evaluate(&capacitor.current_coefficients, phase)).abs()
            < 1.0e-14
    );
}

#[test]
fn inductor_projection_is_constitutive_and_fails_closed_on_dc_exactness() {
    let f0 = 100.0e3;
    let inductance = 1.0e-3;
    let deck = format!(
        "inductor HB state\n\
         V1 in 0 sin(0 1 {f0})\n\
         R1 in out 100\n\
         LSTATE out 0 {inductance}\n\
         .end\n"
    );
    let analysis = run(&deck, f0, 3);
    assert!(analysis.converged);
    let result = &analysis.result;
    let inductor = result
        .reactive_spectra
        .iter()
        .find(|branch| branch.device_name.eq_ignore_ascii_case("LSTATE"))
        .expect("inductor state retained");
    assert_eq!(inductor.kind, HbReactiveKind::Inductor);
    assert!(!inductor.dc_current_is_exact);
    let expected_current =
        inductor.voltage_coefficients[1] / Complex64::new(0.0, TAU * f0 * inductance);
    assert!((inductor.current_coefficients[1] - expected_current).norm() < 1.0e-14);
    assert_eq!(
        result.continuation_limitations,
        vec![HbContinuationLimitation::InductorDcCurrentUsesShortSurrogate]
    );
    assert!(
        !result
            .project_phase(0.0)
            .expect("finite phase projects")
            .is_complete()
    );
}

#[test]
fn nonlinear_norton_source_never_claims_an_mna_branch_spectrum() {
    let f0 = 1.0e6;
    let deck = format!(
        "nonlinear HB state\n\
         V1 in 0 sin(0 0.05 {f0})\n\
         R1 in out 1k\n\
         D1 out 0 DMOD\n\
         C1 out 0 1p\n\
         .model DMOD D IS=1e-14\n\
         .end\n"
    );
    let analysis = run(&deck, f0, 3);
    assert!(analysis.converged);
    assert!(analysis.result.mna_branch_currents.is_empty());
    assert!(
        analysis
            .result
            .continuation_limitations
            .contains(&HbContinuationLimitation::NonlinearVoltageSourcesUseNortonEquivalent)
    );
    assert!(
        !analysis
            .result
            .project_phase(0.0)
            .expect("finite phase projects")
            .is_complete()
    );
}

fn envelope_deck() -> Netlist {
    Netlist::parse(
        "HB Envelope continuation\n\
         Vcarrier carrier 0 SIN(0 1 1meg)\n\
         Vmod mod 0 DC 0 AC 2 PULSE(0 1 250n 20n 20n 2u 10u)\n\
         Rcarrier carrier out 1k\n\
         Rmod mod out 2k\n\
         Cout out 0 160p\n\
         .end\n",
    )
    .expect("Envelope deck parses")
}

fn envelope_hb_config() -> HbConfig {
    HbConfig::new(1.0e6).with_harmonics(4)
}

#[test]
fn hb_envelope_state_freezes_authenticates_restores_and_reactivates() {
    let netlist = envelope_deck();
    let engine = Engine::new(SimulationConfig::default());
    assert!(
        engine.run_hb(&netlist, envelope_hb_config()).is_err(),
        "the original slow PULSE is not carrier-periodic and must not be used as the HB initializer"
    );

    let (hb, state) = engine
        .run_hb_envelope_continuation_state(&netlist, envelope_hb_config(), &["vMoD".to_string()])
        .expect("selected slow source is frozen before the carrier solve");
    assert!(hb.converged);
    let frozen_modulation = hb
        .result
        .spectral_voltages
        .iter()
        .find(|voltage| voltage.node_name.eq_ignore_ascii_case("mod"))
        .expect("modulation source node retained");
    assert!(
        frozen_modulation
            .coefficients
            .iter()
            .all(|coefficient| coefficient.norm() < 1.0e-14),
        "the selected modulation waveform and its AC phasor must be frozen at the exact t=0 value during HB"
    );
    assert_eq!(state.time_origin(), 0.0);
    assert_eq!(state.fundamental_freq(), 1.0e6);
    assert_eq!(state.num_harmonics(), 4);
    assert_eq!(state.canonical_frozen_sources(), &["VMOD".to_string()]);
    assert_eq!(
        state.guarantee(),
        rspice_core::engine::HbEnvelopeStateGuarantee::ExactLinearRcMnaV1
    );
    assert_eq!(state.original_netlist_identity().len(), 64);
    assert_eq!(state.resolved_simulation_identity().len(), 64);
    assert_eq!(state.hb_config_identity().len(), 64);
    assert!(state.history_step().is_finite() && state.history_step() > 0.0);

    let phase_zero = hb
        .result
        .project_phase(0.0)
        .expect("zero carrier phase projects");
    let expected_out = phase_zero
        .node_voltages
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("out"))
        .map(|(_, voltage)| *voltage)
        .expect("out node projected");
    let (transient, _) = engine
        .run_tran_from_hb_envelope_state(
            &netlist,
            &envelope_hb_config(),
            &["VMOD".to_string()],
            &state,
            600.0e-9,
            10.0e-9,
        )
        .expect("original source waveforms reactivate at slow-time origin zero");
    assert_eq!(transient.time.first().copied(), Some(0.0));
    let out_index = transient
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node present");
    assert!(
        (transient.voltages[out_index][0] - expected_out).abs() < 1.0e-12,
        "transient must start from the exact projected HB node state"
    );
    let mod_index = transient
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("mod"))
        .expect("mod node present");
    assert!(transient.voltages[mod_index][0].abs() < 1.0e-12);
    assert!(
        transient.voltages[mod_index]
            .iter()
            .copied()
            .fold(0.0_f64, f64::max)
            > 0.99,
        "the original PULSE waveform must be active after the HB seam"
    );
}

#[test]
fn hb_envelope_artifact_rejects_every_identity_mismatch() {
    let netlist = envelope_deck();
    let engine = Engine::new(SimulationConfig::default());
    let (_, state) = engine
        .run_hb_envelope_continuation_state(&netlist, envelope_hb_config(), &["Vmod".to_string()])
        .expect("continuation state");

    let changed_hb = HbConfig::new(1.0e6).with_harmonics(5);
    let hb_error = engine
        .run_tran_from_hb_envelope_state(
            &netlist,
            &changed_hb,
            &["Vmod".to_string()],
            &state,
            100.0e-9,
            10.0e-9,
        )
        .expect_err("HB configuration mismatch must fail closed");
    assert!(hb_error.to_string().contains("different HB configuration"));

    let source_error = engine
        .run_tran_from_hb_envelope_state(
            &netlist,
            &envelope_hb_config(),
            &["Vcarrier".to_string()],
            &state,
            100.0e-9,
            10.0e-9,
        )
        .expect_err("frozen-source mismatch must fail closed");
    assert!(source_error.to_string().contains("frozen-source set"));

    let changed_netlist = Netlist::parse(
        "changed HB Envelope deck\n\
         Vcarrier carrier 0 SIN(0 1 1meg)\n\
         Vmod mod 0 PULSE(0 1 250n 20n 20n 2u 10u)\n\
         Rcarrier carrier out 1.1k\n\
         Rmod mod out 2k\n\
         Cout out 0 160p\n\
         .end\n",
    )
    .expect("changed deck parses");
    let netlist_error = engine
        .run_tran_from_hb_envelope_state(
            &changed_netlist,
            &envelope_hb_config(),
            &["Vmod".to_string()],
            &state,
            100.0e-9,
            10.0e-9,
        )
        .expect_err("semantic netlist mismatch must fail closed");
    assert!(
        netlist_error
            .to_string()
            .contains("different original netlist")
    );

    let mut changed_simulation = SimulationConfig::default();
    changed_simulation.temperature += 5.0;
    let config_error = Engine::new(changed_simulation)
        .run_tran_from_hb_envelope_state(
            &netlist,
            &envelope_hb_config(),
            &["Vmod".to_string()],
            &state,
            100.0e-9,
            10.0e-9,
        )
        .expect_err("resolved simulation configuration mismatch must fail closed");
    assert!(
        config_error
            .to_string()
            .contains("different resolved simulation configuration")
    );
}

#[test]
fn hb_envelope_source_selection_and_circuit_subset_fail_closed() {
    let netlist = envelope_deck();
    let engine = Engine::new(SimulationConfig::default());
    let duplicate = engine
        .run_hb_envelope_continuation_state(
            &netlist,
            envelope_hb_config(),
            &["Vmod".to_string(), "vMOD".to_string()],
        )
        .expect_err("case-insensitive duplicates must fail");
    assert!(duplicate.to_string().contains("duplicate source 'vmod'"));
    let unknown = engine
        .run_hb_envelope_continuation_state(
            &netlist,
            envelope_hb_config(),
            &["Vmissing".to_string()],
        )
        .expect_err("unknown source must fail");
    assert!(unknown.to_string().contains("unknown independent source"));

    for (label, deck) in [
        (
            "inductor",
            "unsupported inductor\nV1 in 0 SIN(0 1 1meg)\nR1 in out 1k\nL1 out 0 1m\n.end\n",
        ),
        (
            "nonlinear",
            "unsupported diode\nV1 in 0 SIN(0 0.01 1meg)\nR1 in out 1k\nD1 out 0 DMOD\nC1 out 0 1p\n.model DMOD D\n.end\n",
        ),
    ] {
        let unsupported = Netlist::parse(deck).expect("unsupported deck parses");
        let error = engine
            .run_hb_envelope_continuation_state(&unsupported, envelope_hb_config(), &[])
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("exact initializer currently supports only"),
            "{label} rejection was not the strict subset gate: {error}"
        );
    }
}

#[test]
fn hb_envelope_initializer_and_resume_are_abort_aware() {
    let netlist = envelope_deck();
    let engine = Engine::new(SimulationConfig::default());
    let abort = AtomicAbort::new();
    abort.set();
    let creation = engine.run_hb_envelope_continuation_state_with_abort(
        &netlist,
        envelope_hb_config(),
        &["Vmod".to_string()],
        &abort,
    );
    assert!(matches!(creation, Err(SimulationError::Aborted)));

    let (_, state) = engine
        .run_hb_envelope_continuation_state(&netlist, envelope_hb_config(), &["Vmod".to_string()])
        .expect("continuation state");
    let resume = engine.run_tran_from_hb_envelope_state_with_abort(
        &netlist,
        &envelope_hb_config(),
        &["Vmod".to_string()],
        &state,
        100.0e-9,
        10.0e-9,
        &abort,
    );
    assert!(matches!(resume, Err(SimulationError::Aborted)));
}
