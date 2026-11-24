//! The S-parameter sheet: a network's coefficients against frequency on
//! rectangular axes, through the same strip stack as Waves and Bode.
//!
//! ADS opens an S-parameter dataset on rectangular plots of dB(S(i,j))
//! against frequency, and ViVA's Direct Plot form for an SP run plots any
//! S, Y, Z or H parameter, group delay, VSWR, the stability factors and the
//! gains against frequency. The run's retained coefficients are the sheet's
//! traces; every other function is an expression on the analysis' strip,
//! kept by the run's working view as any expression is.

use egui::Ui;
use rspice_results::network_matrix::trace_identity;
use rspice_results_ui::s_parameters::{self as view, NetworkOffer, PlotGroup};
use rspice_results_ui::selection::ResultExpressionPresentationKey;

use super::{
    AnalysisPresentationKey, ResultViewer, ResultsParts, ResultsReads,
    SourceWaveformPresentationKey, ViewerAvailability,
};
use crate::state::{AnalysisResult, AnalysisResultFamilyMetadata};

/// The sheet: the run's strips, with each network's noise columns (the
/// correlation matrix, F, Fmin, Rn, Sopt) unticked until the reader ticks
/// them. They are other quantities in other units, and the correlation
/// matrix in decibels would crush the coefficients' pane; NF, NFmin and Rn
/// are in the Plot menu.
pub(super) fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    untick_noise_columns(state);
    super::waves::show_bode(ui, state);
}

/// Gives every non-coefficient column of the run's networks a hidden
/// visibility, unless the reader has already chosen one, so the browser
/// and the strip agree on what is drawn.
fn untick_noise_columns(state: &mut ResultsParts<'_>) {
    let Some(run) = state.model.simulation.active_run() else {
        return;
    };
    let visibility = &mut state.results.session.waveform_visibility;
    for analysis in run
        .analyses
        .iter()
        .filter(|analysis| analysis.analysis_type.is_network_response())
    {
        let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
        for waveform in &analysis.waveforms {
            let key = SourceWaveformPresentationKey::new(key, &waveform.name);
            if !view::is_coefficient(&waveform.name) && !visibility.contains_key(&key) {
                visibility.set(key, Some(false));
            }
        }
    }
}

/// Whether the active run holds a network the sheet can draw.
pub(super) fn availability(state: ResultsReads<'_>) -> ViewerAvailability {
    let drawable = state.model.simulation.active_run().is_some_and(|run| {
        run.analyses.iter().any(|analysis| {
            analysis.success
                && analysis.analysis_type.is_network_response()
                && !analysis.waveforms.is_empty()
                && super::analysis_evidence_is_valid(state, run.dataset_id, analysis)
        })
    });
    if drawable {
        ViewerAvailability::available("S-parameters are available to plot against frequency")
    } else {
        ViewerAvailability::unavailable("Requires an SP, PSP or HBSP analysis")
    }
}

/// The Plot menu at the head of the sheet's bar. `false` on any other sheet,
/// or when the run holds no network to plot a function of.
pub(super) fn plot_menu(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    if state.results.session.viewer != ResultViewer::SParameters {
        return false;
    }
    let Some((key, groups)) = offer(state.results_reads()) else {
        return false;
    };
    let exprs = &state.results.session.analysis_exprs;
    let shown = |expression: &str| {
        exprs.get(&key).is_some_and(|traces| {
            traces
                .iter()
                .any(|trace| trace.text == expression && trace.visible)
        })
    };
    if let Some(expression) = view::plot_menu(ui, &groups, shown) {
        toggle(state, key, expression);
    }
    true
}

/// The network analysis the menu plots from, and what it offers: the
/// selected analysis when it is a network, else the run's first.
fn offer(state: ResultsReads<'_>) -> Option<(AnalysisPresentationKey, Vec<PlotGroup>)> {
    let run = state.model.simulation.active_run()?;
    let network = |analysis: &&AnalysisResult| {
        analysis.success && analysis.analysis_type.is_network_response()
    };
    let analysis = state
        .model
        .simulation
        .active_analysis()
        .filter(network)
        .or_else(|| run.analyses.iter().find(network))?;
    let coefficients = analysis
        .waveforms
        .iter()
        .filter_map(|waveform| {
            let name = waveform.complex.as_ref()?.source_name.trim();
            let identity = trace_identity(name).filter(|identity| identity.physical_ports)?;
            (!name.contains('['))
                .then(|| (identity.output_port, identity.input_port, name.to_owned()))
        })
        .collect::<Vec<_>>();
    let references = match &analysis.family_metadata {
        Some(AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm,
            ..
        }) => reference_impedances_ohm.clone(),
        _ => Vec::new(),
    };
    let kept = |name: &str| {
        analysis
            .waveforms
            .iter()
            .any(|waveform| waveform.name == name)
    };
    let groups = view::plot_groups(&NetworkOffer {
        coefficients: &coefficients,
        references: &references,
        noise: kept("F") && kept("Fmin") && kept("Rn"),
    });
    (!groups.is_empty()).then(|| {
        (
            AnalysisPresentationKey::new(run.dataset_id, analysis),
            groups,
        )
    })
}

/// Adds the function to the analysis' strip, or shows or hides it when it
/// is there already.
fn toggle(state: &mut ResultsParts<'_>, key: AnalysisPresentationKey, expression: String) {
    let present = state
        .results
        .session
        .analysis_exprs
        .get(&key)
        .is_some_and(|traces| traces.iter().any(|trace| trace.text == expression));
    if !present {
        super::waves::add_strip_expression(state, key, expression);
        return;
    }
    let identity = ResultExpressionPresentationKey::new(key, expression);
    match state
        .results
        .toggle_expression_visibility_by_key(&state.model.simulation, &identity)
    {
        Ok(()) => state.model.workspace.content.visualization_documents_dirty = true,
        Err(error) => state.workbench.prompt(error),
    }
}

#[cfg(test)]
mod tests;
