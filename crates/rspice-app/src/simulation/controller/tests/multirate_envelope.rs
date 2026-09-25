//! Controller tests preserve envelope controls, exact events, and source limits.

use super::*;
use crate::simulation::dialog::EnvelopeDialogState;
use crate::simulation::plan::AnalysisDraft;
use crate::simulation::runner::worker_contract::{
    WorkerAnalysisSpec, round_trip_response_for_test,
};

fn draft() -> EnvelopeDialogState {
    let mut draft = EnvelopeDialogState::default();
    draft.ensure_initialized();
    draft.multirate_enabled = true;
    draft.carrier_tones = "1Meg".into();
    draft.stop_time = "2m".into();
    draft.envelope_step = "200u".into();
    draft.harmonic_order = "1".into();
    draft.modulation_sources = "Vmod".into();
    draft.adaptive_mode_idx = 1;
    draft.multirate.adaptive = false;
    draft.multirate.bdf2 = false;
    draft.multirate.dc_initialization = true;
    draft.multirate.linear_method =
        rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Direct;
    draft.multirate.collocation_points = "8".into();
    // Disabled buffers persist, but never affect an active solver choice.
    draft.multirate.oversample = "incomplete".into();
    draft.multirate.minimum_step = "incomplete".into();
    draft.multirate.linear_restart = "incomplete".into();
    draft.initialization.reltol = "incomplete".into();
    draft.initial_periodic_solve_idx = usize::MAX;
    draft
}

fn spec(draft: &EnvelopeDialogState) -> AnalysisSpec {
    let saved = serde_json::to_value(draft).unwrap();
    let mut restored =
        AnalysisDraft::Envelope(Box::new(serde_json::from_value(saved.clone()).unwrap()));
    restored.prepare_after_restore();
    assert_eq!(
        serde_json::to_value(&restored).unwrap(),
        serde_json::json!({"kind": "envelope", "draft": saved.clone()})
    );
    let AnalysisDraft::Envelope(restored_draft) = &restored else {
        unreachable!()
    };
    assert_eq!(serde_json::to_value(restored_draft).unwrap(), saved);
    let mut state = AppState::default();
    state.sim_setup.apply_analysis_draft_projection(&restored);
    let spec = SimulationController::new()
        .build_envelope_spec(&state)
        .unwrap();
    spec.validate().unwrap();
    let worker = WorkerAnalysisSpec::try_from(&spec).unwrap();
    let worker: WorkerAnalysisSpec =
        serde_json::from_value(serde_json::to_value(worker).unwrap()).unwrap();
    let restored = AnalysisSpec::from(worker);
    assert_eq!(spec, restored);
    restored
}

const DECK: &str = "Multirate Studio\nVmod input 0 PWL(0 0 .73m 0 .73m 1 2m 1 2m 2)\nVrf rf 0 SIN(0 .4 1Meg 0 0 90)\nRin input out 1k\nRrf rf out 1k\nCout out 0 1u\nCshunt input 0 2u\n.end\n";

#[test]
fn multirate_envelope_saved_controls_execute_and_retain_physical_results() {
    let draft = draft();
    let spec = spec(&draft);
    let line = draft.to_config().unwrap().to_spice();
    assert!(line.contains("multirate="));
    let run = crate::simulation::runner::pvt_point_evidence::run_declaration(
        DECK,
        "Envelope",
        QueuedAnalysis {
            spec,
            analysis_line: line,
            config: None,
            spec_options: Default::default(),
            numeric_override: None,
        },
        27.0,
        crate::simulation::execution::SavePolicy::RetainEngineProducedResults,
        &[],
    )
    .unwrap();
    let result = &run.analyses[0];
    assert!(result.success, "{:?}", result.error_message);
    assert_eq!(result.analysis_type, AnalysisType::Envelope);
    let trace = result
        .waveforms
        .iter()
        .find(|w| w.name.to_ascii_lowercase().starts_with("env(v(out); k=[0]"))
        .unwrap();
    assert_eq!(trace.unit.as_deref(), Some("V"));
    assert_eq!(trace.x.first(), Some(&0.0));
    assert_eq!(trace.x.last(), Some(&0.002));
    assert_eq!(trace.x.iter().filter(|t| **t == 0.00073).count(), 2);
    let mut expected = 0.0;
    let mut previous = 0.0;
    for (&time, &value) in trace.x.iter().zip(trace.y.iter()) {
        let step = time - previous;
        if step > 0.0 {
            let source = if previous >= 0.00073 { 0.001 } else { 0.0 };
            expected = (source + 1e-6 / step * expected) / (0.002 + 1e-6 / step);
        }
        assert!(
            (value - expected).abs() < 1e-8,
            "{time}: {value} != {expected}"
        );
        previous = time;
    }
    assert!(
        result
            .waveforms
            .iter()
            .any(|w| w.name.to_ascii_lowercase().starts_with("env(i(vmod)")
                && w.unit.as_deref() == Some("A"))
    );
    let impulse = result
        .measurements
        .iter()
        .find(|m| {
            m.name
                .to_ascii_lowercase()
                .contains("impulse i(vmod) k=[0]")
                && m.name.ends_with("real")
                && m.event_axis == Some(0.00073)
        })
        .unwrap();
    assert!((impulse.value.unwrap() + 2e-6).abs() < 1e-12);
    assert_eq!(impulse.units.as_ref().unwrap().value.symbol(), Some("C"));
    assert_eq!(impulse.units.as_ref().unwrap().axis.symbol(), Some("s"));
    let steps = result
        .measurements
        .iter()
        .find(|m| m.name == "Envelope accepted steps")
        .unwrap()
        .value
        .unwrap();
    assert!(steps < 20.0);
}

#[test]
fn multirate_envelope_independent_tones_adapt_and_transfer_exact_event_measurements() {
    let mut draft = draft();
    draft.carrier_tones = "1Meg, 1.414213562373095Meg".into();
    draft.multirate.adaptive = true;
    draft.multirate.minimum_step = "1n".into();
    draft.multirate.maximum_step = "500u".into();
    draft.multirate.bdf2 = true;
    draft.multirate.max_mixing_order = "1".into();
    draft.multirate.source_tones = "Vac=2".into();
    draft.multirate.dc_initialization = false;
    draft.adaptive_mode_idx = 2;
    let spec = spec(&draft);
    let deck = DECK.replace(".end", "Vac second 0 AC .2 90\nRac second out 1k\n.options output outputtimepoints=0,.001,.002\n.end");
    let result = crate::simulation::runner::pvt_point_evidence::run_standalone_spec(&deck, spec);
    let result = round_trip_response_for_test(result);
    let crate::simulation::results::SimulationResult::Transient {
        time,
        waveforms,
        measurements,
        ..
    } = result
    else {
        panic!("envelope result")
    };
    assert_eq!(time, vec![0.0, 0.00073, 0.00073, 0.001, 0.002, 0.002]);
    let dc = waveforms
        .values()
        .find(|w| {
            w.name
                .to_ascii_lowercase()
                .starts_with("env(v(out); k=[0,0]")
        })
        .unwrap();
    let expected = (1.0 - (-0.00127_f64 / (1e-6 / 0.003)).exp()) / 3.0;
    assert!((dc.y_values.last().unwrap() - expected).abs() < 2e-3);
    let carrier = waveforms
        .values()
        .find(|w| {
            w.name
                .to_ascii_lowercase()
                .starts_with("env(v(out); k=[0,1]")
        })
        .unwrap();
    let expected = num_complex::Complex64::new(0.0, 0.0002)
        / num_complex::Complex64::new(0.003, std::f64::consts::TAU * 1.414213562373095);
    for (&re, &im) in carrier
        .y_values
        .iter()
        .zip(carrier.y_imag.as_ref().unwrap())
    {
        assert!((num_complex::Complex64::new(re, im) - expected).norm() < 1e-8);
    }
    assert!(measurements.iter().any(|m| m.name.contains("impulse")
        && m.event_axis == Some(0.00073)
        && m.units.as_ref().unwrap().value.symbol() == Some("C")));
    assert!(
        measurements
            .iter()
            .find(|m| m.name == "Envelope rejected steps")
            .unwrap()
            .value
            .unwrap()
            > 0.0
    );
}

#[test]
fn multirate_envelope_strobe_preserves_incoming_and_outgoing_source_limits() {
    let spec = spec(&draft());
    let deck = DECK.replace(".end", ".options output initial_interval=.65m\n.end");
    let result = crate::simulation::runner::pvt_point_evidence::run_standalone_spec(&deck, spec);
    let crate::simulation::results::SimulationResult::Transient {
        time, waveforms, ..
    } = result
    else {
        panic!("envelope result")
    };
    let interval = rspice_core::Netlist::parse(&deck)
        .unwrap()
        .options
        .output_interval_schedule
        .unwrap()
        .initial_interval;
    assert!(time.contains(&interval), "{time:?}");
    let source = waveforms
        .values()
        .find(|w| {
            w.name
                .to_ascii_lowercase()
                .starts_with("env(v(input); k=[0]")
        })
        .unwrap();
    assert_eq!(time.iter().filter(|t| **t == 0.00073).count(), 2);
    for (index, (&time, &value)) in time.iter().zip(&source.y_values).enumerate() {
        let expected = if time < 0.00073
            || (time == 0.00073 && source.x_values.get(index + 1) == Some(&time))
        {
            0.0
        } else if time < 0.002 || source.x_values.get(index + 1) == Some(&time) {
            1.0
        } else {
            2.0
        };
        assert!((value - expected).abs() < 1e-8, "{time}: {value}");
    }
}
