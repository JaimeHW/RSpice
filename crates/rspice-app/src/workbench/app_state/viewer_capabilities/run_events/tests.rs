//! Each event leaves the viewers exactly as the run tier's own writes did.
//!
//! Every expected text below was captured by running the writes the run tier
//! made before it reported through events, on the same seeded state. Only a
//! window coefficient is left out of one failure, as the FFT tests do,
//! because it is a computed float.

use super::*;
use crate::product::{AnalysisInstanceId, DatasetId};
use crate::state::SpecializedViewerCacheProvenance;
use rspice_results_ui::eye_diagram::{EyeTimebaseProvenance, EyeTrace};
use rspice_results_ui::fft::data::FftBuildError;
use rspice_results_ui::fft::{FftFailure, FftInputError, PreparedFftInput, WindowFunction};
use rspice_results_ui::nyquist::NyquistData;

fn provenance() -> SpecializedViewerCacheProvenance {
    SpecializedViewerCacheProvenance::for_prepared_analysis(
        DatasetId::new(),
        AnalysisInstanceId::new(),
    )
}

fn eye() -> EyeData {
    let mut eye = EyeData::new(1e-9, 2);
    eye.add_trace(EyeTrace::new(vec![0.0, 0.5, 1.0], vec![0.0, 1.0, 0.0]));
    eye.add_trace(EyeTrace::new(vec![0.0, 0.5, 1.0], vec![1.0, 0.0, 1.0]));
    eye
}

fn explicit() -> EyeTimebaseProvenance {
    EyeTimebaseProvenance::Explicit {
        unit_interval: 1e-9,
        incoherent: false,
    }
}

fn sine(name: &str) -> PreparedFftInput {
    PreparedFftInput {
        name: name.to_owned(),
        samples: (0..16)
            .map(|n| (std::f64::consts::TAU * n as f64 / 8.0).sin())
            .collect(),
        sample_rate: 16.0,
        original_count: 16,
        decimation_factor: 1,
    }
}

/// A sample the Hann window erases, which the FFT refuses to transform.
fn erased(name: &str) -> PreparedFftInput {
    let mut samples = vec![0.0; 16];
    samples[0] = 1.0;
    samples[1] = f64::from_bits(1);
    PreparedFftInput {
        name: name.to_owned(),
        samples,
        sample_rate: 16.0,
        original_count: 16,
        decimation_factor: 1,
    }
}

fn mismatch() -> FftInputError {
    FftInputError::LengthMismatch {
        time_count: 17,
        value_count: 16,
    }
}

fn responses(count: usize) -> BodeData {
    let mut bode = BodeData::new();
    for _ in 0..count {
        bode.add_response();
    }
    bode
}

fn locus() -> NyquistData {
    NyquistData::from_arrays(
        "L(jω)",
        &[1.0, 10.0, 100.0],
        &[2.0, 0.5, -0.1],
        &[0.0, -1.0, -0.2],
    )
}

/// Every viewer holds data owned by some other result, so each event's
/// effect shows against it.
fn seeded() -> (AppState, SpecializedViewerCacheProvenance) {
    let mut state = AppState::default();
    state.analysis.bode_plot_state.load_data(responses(1));
    state
        .analysis
        .nyquist_state
        .load_data(NyquistData::from_arrays(
            "old nyquist",
            &[1.0, 10.0],
            &[1.0, -0.5],
            &[0.0, 0.25],
        ));
    state
        .analysis
        .smith_chart_state
        .load_sparam_data("S11", &[1.0], &[0.25], &[0.0], Some(50.0))
        .expect("valid Smith fixture");
    state.analysis.eye_diagram_state.load_data(eye());
    state.analysis.fft_state.window = WindowFunction::Rectangular;
    state
        .analysis
        .fft_state
        .load_prepared_input(sine("V(old)"))
        .expect("qualified FFT fixture");
    let other = provenance();
    for viewer in [
        ActiveViewer::BodePlot,
        ActiveViewer::Nyquist,
        ActiveViewer::SmithChart,
        ActiveViewer::Fft,
        ActiveViewer::EyeDiagram,
    ] {
        state.bind_specialized_viewer_cache(viewer, other);
    }
    (state, provenance())
}

fn failure(error: Option<&FftFailure>) -> String {
    match error {
        Some(FftFailure::Build(FftBuildError::ErasedWindowedSample { index, .. })) => {
            format!("Some(Build(ErasedWindowedSample {{ index: {index}, .. }}))")
        }
        other => format!("{other:?}"),
    }
}

/// What every viewer holds and who owns it, with `owner` named as such.
fn snapshot(state: &AppState, owner: SpecializedViewerCacheProvenance) -> String {
    let who = |slot: Option<SpecializedViewerCacheProvenance>| match slot {
        None => "none",
        Some(found) if found == owner => "owner",
        Some(_) => "other",
    };
    let authority = &state.analysis.cache_authority;
    let fft = &state.analysis.fft_state;
    let eye = &state.analysis.eye_diagram_state;
    format!(
        "authority bode={} nyquist={} smith={} fft={} eye={}\n\
         fft source={:?} cache={:?} data={} analysis={} error={} revision={}\n\
         eye traces={} timebase={:?} revision={}\n\
         nyquist {:?}\n\
         bode responses={}\n\
         smith traces={}",
        who(authority.bode),
        who(authority.nyquist),
        who(authority.smith),
        who(authority.fft),
        who(authority.eye),
        fft.selected_source,
        fft.source_cache.as_ref().map(|cache| (
            cache.name.as_str(),
            cache.samples.len(),
            cache.sample_rate
        )),
        fft.data.is_some(),
        fft.analysis.is_some(),
        failure(fft.last_error.as_ref()),
        fft.spectrum_revision(),
        eye.trace_count(),
        eye.timebase_provenance(),
        eye.data_revision(),
        state
            .analysis
            .nyquist_state
            .curve()
            .map(|curve| (curve.name.as_str(), curve.points.len())),
        state.analysis.bode_plot_state.data.response_count(),
        state.analysis.smith_chart_state.traces.len(),
    )
}

fn applied(event: impl FnOnce(SpecializedViewerCacheProvenance) -> RunEvent) -> String {
    let (mut state, owner) = seeded();
    state.apply_run_event(event(owner));
    snapshot(&state, owner)
}

#[test]
fn a_replaced_transient_result_empties_the_fft_and_eye_and_frees_their_owners() {
    assert_eq!(
        applied(|_| RunEvent::TransientResultReplaced),
        "authority bode=other nyquist=other smith=other fft=none eye=none\n\
         fft source=Some(\"V(old)\") cache=None data=false analysis=false error=None revision=2\n\
         eye traces=0 timebase=None revision=2\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );
}

#[test]
fn a_chosen_fft_source_changes_only_the_selection() {
    assert_eq!(
        applied(|_| RunEvent::FftSourceChosen {
            source: "V(out)".to_owned(),
        }),
        "authority bode=other nyquist=other smith=other fft=other eye=other\n\
         fft source=Some(\"V(out)\") cache=Some((\"V(old)\", 16, 16.0)) data=true analysis=true error=None revision=1\n\
         eye traces=2 timebase=None revision=1\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );
}

#[test]
fn a_folded_eye_is_loaded_with_its_timebase_and_owned_only_when_something_folded() {
    assert_eq!(
        applied(|owner| RunEvent::EyeFolded {
            owner: Some(owner),
            eye: Some(eye()),
            timebase: explicit(),
        }),
        "authority bode=other nyquist=other smith=other fft=other eye=owner\n\
         fft source=Some(\"V(old)\") cache=Some((\"V(old)\", 16, 16.0)) data=true analysis=true error=None revision=1\n\
         eye traces=2 timebase=Some(Explicit { unit_interval: 1e-9, incoherent: false }) revision=2\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );
    assert_eq!(
        applied(|owner| RunEvent::EyeFolded {
            owner: Some(owner),
            eye: None,
            timebase: explicit(),
        }),
        "authority bode=other nyquist=other smith=other fft=other eye=other\n\
         fft source=Some(\"V(old)\") cache=Some((\"V(old)\", 16, 16.0)) data=true analysis=true error=None revision=1\n\
         eye traces=0 timebase=Some(Explicit { unit_interval: 1e-9, incoherent: false }) revision=2\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );
}

#[test]
fn a_changed_bit_period_empties_the_eye_and_frees_its_owner() {
    assert_eq!(
        applied(|_| RunEvent::EyeTimebaseChanged),
        "authority bode=other nyquist=other smith=other fft=other eye=none\n\
         fft source=Some(\"V(old)\") cache=Some((\"V(old)\", 16, 16.0)) data=true analysis=true error=None revision=1\n\
         eye traces=0 timebase=None revision=2\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );
}

#[test]
fn an_outdated_eye_is_emptied_and_its_owner_left_for_synchronization() {
    assert_eq!(
        applied(|_| RunEvent::EyeOutdated),
        "authority bode=other nyquist=other smith=other fft=other eye=other\n\
         fft source=Some(\"V(old)\") cache=Some((\"V(old)\", 16, 16.0)) data=true analysis=true error=None revision=1\n\
         eye traces=0 timebase=None revision=2\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );
}

#[test]
fn a_prepared_fft_input_is_owned_only_when_the_viewer_accepts_it() {
    assert_eq!(
        applied(|owner| RunEvent::FftInputPrepared {
            owner: Some(owner),
            input: Ok(sine("V(out)")),
        }),
        "authority bode=other nyquist=other smith=other fft=owner eye=other\n\
         fft source=Some(\"V(old)\") cache=Some((\"V(out)\", 16, 16.0)) data=true analysis=true error=None revision=2\n\
         eye traces=2 timebase=None revision=1\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );

    let (mut state, owner) = seeded();
    state.analysis.fft_state.window = WindowFunction::Hanning;
    state.apply_run_event(RunEvent::FftInputPrepared {
        owner: Some(owner),
        input: Ok(erased("V(out)")),
    });
    assert_eq!(
        snapshot(&state, owner),
        "authority bode=other nyquist=other smith=other fft=none eye=other\n\
         fft source=Some(\"V(old)\") cache=Some((\"V(out)\", 16, 16.0)) data=false analysis=false error=Some(Build(ErasedWindowedSample { index: 1, .. })) revision=2\n\
         eye traces=2 timebase=None revision=1\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );

    assert_eq!(
        applied(|owner| RunEvent::FftInputPrepared {
            owner: Some(owner),
            input: Err(mismatch()),
        }),
        "authority bode=other nyquist=other smith=other fft=none eye=other\n\
         fft source=Some(\"V(old)\") cache=None data=false analysis=false error=Some(Input(LengthMismatch { time_count: 17, value_count: 16 })) revision=2\n\
         eye traces=2 timebase=None revision=1\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );
}

#[test]
fn a_lost_fft_worker_is_recorded_and_frees_the_owner() {
    assert_eq!(
        applied(|_| RunEvent::FftWorkerLost),
        "authority bode=other nyquist=other smith=other fft=none eye=other\n\
         fft source=Some(\"V(old)\") cache=None data=false analysis=false error=Some(WorkerDisconnected) revision=2\n\
         eye traces=2 timebase=None revision=1\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );
}

#[test]
fn an_outdated_fft_is_cleared_and_its_owner_left_for_synchronization() {
    assert_eq!(
        applied(|_| RunEvent::FftOutdated),
        "authority bode=other nyquist=other smith=other fft=other eye=other\n\
         fft source=Some(\"V(old)\") cache=None data=false analysis=false error=None revision=2\n\
         eye traces=2 timebase=None revision=1\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );
}

#[test]
fn an_emptied_fft_frees_only_its_owner() {
    assert_eq!(
        applied(|_| RunEvent::FftEmptied),
        "authority bode=other nyquist=other smith=other fft=none eye=other\n\
         fft source=Some(\"V(old)\") cache=Some((\"V(old)\", 16, 16.0)) data=true analysis=true error=None revision=1\n\
         eye traces=2 timebase=None revision=1\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );
}

#[test]
fn a_confirmed_cache_changes_only_its_owner() {
    assert_eq!(
        applied(|owner| RunEvent::ViewerCacheConfirmed {
            viewer: ActiveViewer::Fft,
            owner,
        }),
        "authority bode=other nyquist=other smith=other fft=owner eye=other\n\
         fft source=Some(\"V(old)\") cache=Some((\"V(old)\", 16, 16.0)) data=true analysis=true error=None revision=1\n\
         eye traces=2 timebase=None revision=1\n\
         nyquist Some((\"old nyquist\", 2))\n\
         bode responses=1\n\
         smith traces=1"
    );
}

#[test]
fn a_frequency_response_replaces_bode_nyquist_and_smith_and_owns_what_it_loaded() {
    assert_eq!(
        applied(|owner| RunEvent::FrequencyResponseReplaced {
            owner: Some(owner),
            bode: responses(2),
            loop_gain: Some(locus()),
        }),
        "authority bode=owner nyquist=owner smith=none fft=other eye=other\n\
         fft source=Some(\"V(old)\") cache=Some((\"V(old)\", 16, 16.0)) data=true analysis=true error=None revision=1\n\
         eye traces=2 timebase=None revision=1\n\
         nyquist Some((\"L(jω)\", 3))\n\
         bode responses=2\n\
         smith traces=0"
    );
    assert_eq!(
        applied(|owner| RunEvent::FrequencyResponseReplaced {
            owner: Some(owner),
            bode: responses(0),
            loop_gain: None,
        }),
        "authority bode=none nyquist=none smith=none fft=other eye=other\n\
         fft source=Some(\"V(old)\") cache=Some((\"V(old)\", 16, 16.0)) data=true analysis=true error=None revision=1\n\
         eye traces=2 timebase=None revision=1\n\
         nyquist None\n\
         bode responses=0\n\
         smith traces=0"
    );
}

#[test]
fn collected_completions_release_every_cache_the_active_result_does_not_own() {
    assert_eq!(
        applied(|_| RunEvent::CompletionsCollected),
        "authority bode=none nyquist=none smith=none fft=none eye=none\n\
         fft source=Some(\"V(old)\") cache=None data=false analysis=false error=None revision=2\n\
         eye traces=0 timebase=None revision=2\n\
         nyquist None\n\
         bode responses=0\n\
         smith traces=0"
    );
}
