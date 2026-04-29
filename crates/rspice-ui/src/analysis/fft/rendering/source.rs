use super::*;

pub(super) fn collect_fft_source_names(app_state: &AppState) -> Vec<String> {
    if !fft_supported_for_active_analysis(app_state) {
        return Vec::new();
    }

    let mut names: Vec<String> = app_state
        .simulation
        .waveforms
        .iter()
        .map(|wf| wf.name.clone())
        .collect();
    names.sort();
    names.dedup();
    names
}

pub(super) fn current_fft_source_time_bounds(app_state: &AppState) -> Option<(f64, f64)> {
    let selected = app_state
        .analysis
        .fft_state
        .selected_source
        .as_ref()
        .or_else(|| {
            app_state
                .analysis
                .fft_state
                .source_cache
                .as_ref()
                .map(|src| &src.name)
        })?;
    let waveform = app_state
        .simulation
        .waveforms
        .iter()
        .find(|wf| wf.name == *selected)?;
    waveform_time_bounds(waveform)
}

fn waveform_time_bounds(waveform: &crate::state::WaveformData) -> Option<(f64, f64)> {
    let start = waveform.x.iter().copied().find(|x| x.is_finite())?;
    let end = waveform.x.iter().copied().rfind(|x| x.is_finite())?;
    if end > start {
        Some((start, end))
    } else {
        None
    }
}

pub(super) fn fft_supported_for_active_analysis(app_state: &AppState) -> bool {
    matches!(
        app_state
            .simulation
            .active_analysis()
            .map(|analysis| analysis.analysis_type),
        Some(
            AnalysisType::Transient
                | AnalysisType::Pss
                | AnalysisType::Envelope
                | AnalysisType::Soa
        )
    )
}

pub(super) fn refresh_fft_from_source_waveform(app_state: &mut AppState, source_name: &str) {
    app_state
        .analysis
        .fft_state
        .set_selected_source(Some(source_name.to_string()));
    let Some(waveform) = app_state
        .simulation
        .waveforms
        .iter()
        .find(|wf| wf.name == source_name)
    else {
        app_state.analysis.fft_state.clear();
        return;
    };

    let input_options = app_state
        .analysis
        .fft_state
        .input_options_for_waveform(&waveform.x);
    if let Some(prepared) = crate::analysis::fft::prepare_fft_input_with_options(
        source_name,
        &waveform.x,
        &waveform.y,
        input_options,
    ) {
        app_state.analysis.fft_state.load_prepared_input(prepared);
    } else {
        app_state.analysis.fft_state.clear();
    }
}
