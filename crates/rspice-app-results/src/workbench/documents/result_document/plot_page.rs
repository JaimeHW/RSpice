//! The dock's Plot page, on the pages that plot in the dock: one analysis of
//! the plotted run, its panes fitted to the dock. Laying several analyses
//! out is the Results workspace's, a press of Open in Results away.

use super::{
    AnalysisPresentationKey, ResultViewer, ResultsParts, ResultsReads, create_document, instrument,
    operational_state, waves,
};
use crate::product::{AnalysisInstanceId, DatasetId};
use crate::ui::tokens::Tokens;
use egui::Ui;
use rspice_results::result_presentation::SourceWaveformPresentationKey;
use rspice_results_ui::presentation::well_hint;
use rspice_vocabulary::source_revision::SourceRevision;

/// The dock's Plot page: its toolbar band, then the shown analysis' plot,
/// flush to the dock's sides and foot. This is the same canonical retained
/// result state Results draws, not a second result document, so Open in
/// Results shows the same view.
pub fn show_plot_page(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let offered = offered_analyses(state);
    settle_shown(state, &offered);
    // The page plots; the run's other sheets (its table, spectrum, eye and
    // manifest) are in Results, a press of the band's Open in Results away.
    let sheets = pane_sheets(state.results_reads());
    if let Some(&first) = sheets.first()
        && !sheets.contains(&state.results.session.viewer)
    {
        state.results.session.viewer = first;
    }
    super::prepare_quick_view(ui, state);
    let shown = shown_analysis(state.results_reads());
    let open_pressed = rspice_app_workbench::workbench::docks::console::toolbar(ui, |ui| {
        band(ui, state, &sheets, &offered)
    });
    crate::ui::plot::set_interaction_mode(
        ui.ctx(),
        state.results.session.plot_tool.interaction_mode(),
    );
    let well = ui.available_rect_before_wrap();
    ui.allocate_ui(well.size(), |ui| {
        ui.set_min_height(well.height());
        if sheets.is_empty() && state.model.simulation.retained.has_results() {
            let t = Tokens::get(ui.ctx());
            ui.painter().rect_filled(well, 0.0, t.color.canvas_bg);
            // An analysis that failed has nothing to plot for that reason:
            // its card takes the well, as on its sheet in Results.
            let viewer = state.results.session.viewer;
            let status = operational_state::classify_viewer(state, viewer);
            if !(status.blocks_visuals
                && operational_state::show_result_operational_status(ui, state, &status))
            {
                well_hint(
                    ui,
                    "Nothing to plot from this run: its values are on the drawing, and Open in Results shows every sheet",
                );
            }
        } else if super::viewer_well_is_open(ui, state) {
            waves::show_compact(ui, state, shown);
        }
    });
    super::working_view::capture_at_frame_end(state);
    waves::marker_dialog::show(ui.ctx(), state);
    if open_pressed {
        open_in_results(state, shown);
    }
}

/// The Plot page's band: the result's state when it is not complete, the
/// analysis when the run has more than one with something plotted, the
/// sheet when the analysis has more than one to plot on, the cursor tools,
/// zoom and fit, and at its right end what the cursors read and Open in
/// Results. Returns whether Open in Results was pressed.
fn band(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    sheets: &[ResultViewer],
    offered: &[OfferedAnalysis],
) -> bool {
    let mut open_in_results = false;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = PLOT_BAND_GAP;
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            open_in_results = crate::ui::widgets::Button::new("Open in Results")
                .compact()
                .show(ui)
                .on_hover_text("Open this plot in Results: measurements, cursors and every sheet")
                .clicked();
            let t = Tokens::get(ui.ctx());
            if let Some(readout) = waves::inline_cursor_readout(state, &t) {
                ui.label(
                    egui::RichText::new(readout)
                        .font(crate::ui::theme::mono(
                            crate::ui::tokens::FS_0,
                            crate::ui::theme::FontWeight::Regular,
                        ))
                        .color(t.color.text_dim),
                );
            }
            let rest = ui.available_size();
            ui.allocate_ui_with_layout(
                rest,
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.set_clip_rect(ui.max_rect().intersect(ui.clip_rect()));
                    if operational_state::show_status_mark(ui, state) {
                        instrument::separator(ui);
                    }
                    if offered.len() > 1 {
                        analysis_picker(ui, state, offered);
                        instrument::separator(ui);
                    }
                    if sheets.len() > 1 {
                        let labels: Vec<&str> =
                            sheets.iter().map(|sheet| sheet.tab_label()).collect();
                        let mut chosen = sheets
                            .iter()
                            .position(|&sheet| sheet == state.results.session.viewer)
                            .unwrap_or(0);
                        if rspice_ui_kit::widgets::segmented(
                            ui,
                            "results.plot-page.sheet",
                            &labels,
                            &mut chosen,
                            rspice_ui_kit::widgets::SegmentedWidth::Natural,
                        ) {
                            state.results.session.viewer = sheets[chosen];
                        }
                        instrument::separator(ui);
                    }
                    instrument::show_plot_tools(ui, state);
                },
            );
        });
    });
    open_in_results
}

/// The gap between two items of the Plot page's band.
const PLOT_BAND_GAP: f32 = 6.0;

/// The select that picks the analysis the page plots, each named as the data
/// browser names it.
fn analysis_picker(ui: &mut Ui, state: &mut ResultsParts<'_>, offered: &[OfferedAnalysis]) {
    let labels: Vec<String> = offered
        .iter()
        .map(|analysis| analysis.label.clone())
        .collect();
    let shown = state.model.simulation.view.active_analysis_idx;
    let selected = offered
        .iter()
        .position(|analysis| Some(analysis.index) == shown)
        .map_or("", |position| labels[position].as_str());
    let width = rspice_ui_kit::widgets::compact_select_mono_width(ui, &labels);
    let output = rspice_ui_kit::widgets::compact_select_mono_with_response(
        ui,
        "results.plot-page.analysis",
        "Analysis",
        selected,
        &labels,
        width,
    );
    output
        .response
        .on_hover_text("The analysis the page plots; Results lays out every one");
    if let Some(picked) = output.picked {
        show_on_plot_page(state, offered[picked].key);
    }
}

/// An analysis of the shown run the Plot page offers: one a sheet of the
/// page draws, with a signal or an expression of it plotted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OfferedAnalysis {
    /// Its place in the run.
    index: usize,
    key: AnalysisPresentationKey,
    /// The authored analysis it solved, which the plan's next run solves
    /// too.
    authored: AnalysisInstanceId,
    /// What the data browser calls it.
    label: String,
}

/// The analyses of the shown run a sheet of the Plot page draws, each with
/// whether a signal of it is plotted, and what that was read from: the run,
/// its samples and the signals the reader plotted. Read again only when one
/// of those changes, so an idle frame walks no run's signals.
#[derive(Debug, Clone)]
pub(crate) struct PlotPageOffer {
    dataset: DatasetId,
    history: crate::state::RunHistoryRevision,
    data_version: u64,
    visibility: SourceRevision,
    analyses: Vec<(OfferedAnalysis, bool)>,
}

/// The analyses of the shown run the Plot page offers, in the run's order.
fn offered_analyses(state: &mut ResultsParts<'_>) -> Vec<OfferedAnalysis> {
    let Some(run) = state.model.simulation.active_run() else {
        return Vec::new();
    };
    let (dataset, history, data_version, visibility) = (
        run.dataset_id,
        state.model.simulation.retained.runs.revision(),
        state.model.simulation.view.data_version,
        state.results.session.waveform_visibility.revision(),
    );
    let current = state.results.plot_page_offer.as_ref().is_some_and(|offer| {
        offer.dataset == dataset
            && offer.history == history
            && offer.data_version == data_version
            && offer.visibility == visibility
    });
    if !current {
        let analyses = read_offer(state.results_reads());
        state.results.plot_page_offer = Some(Box::new(PlotPageOffer {
            dataset,
            history,
            data_version,
            visibility,
            analyses,
        }));
    }
    let session = &state.results.session;
    state
        .results
        .plot_page_offer
        .as_ref()
        .map(|offer| {
            offer
                .analyses
                .iter()
                .filter(|(analysis, plotted)| {
                    *plotted
                        || session
                            .analysis_exprs
                            .get(&analysis.key)
                            .is_some_and(|exprs| exprs.iter().any(|expr| expr.visible))
                })
                .map(|(analysis, _)| analysis.clone())
                .collect()
        })
        .unwrap_or_default()
}

/// Each analysis of the shown run a sheet of the page draws, with whether a
/// signal of it is plotted.
fn read_offer(state: ResultsReads<'_>) -> Vec<(OfferedAnalysis, bool)> {
    let Some(run) = state.model.simulation.active_run() else {
        return Vec::new();
    };
    run.analyses
        .iter()
        .enumerate()
        .filter(|(_, analysis)| {
            ResultViewer::all()
                .filter(|&sheet| pane_sheet(sheet))
                .any(|sheet| {
                    rspice_results::renderability::viewer_can_render(sheet, analysis, |gate| {
                        super::analysis_answers_structural_gate(
                            state,
                            run.dataset_id,
                            analysis,
                            gate,
                        )
                    })
                })
        })
        .map(|(index, analysis)| {
            let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
            let plotted = analysis.waveforms.iter().any(|waveform| {
                state.results.session.waveform_visibility(
                    &SourceWaveformPresentationKey::new(key, &waveform.name),
                    waveform.visible,
                )
            });
            let offered = OfferedAnalysis {
                index,
                key,
                authored: create_document::source_analysis_identity(run, analysis),
                label: crate::workbench::docks::navigator::results::analysis_label(analysis),
            };
            (offered, plotted)
        })
        .collect()
}

/// Settle the shown run on the analysis the page shows, when the run has not
/// settled on one, or shows one the page does not offer: by the page's
/// rule, [`chosen`]. While the run writes and the rule's pick is a stand-in
/// for an analysis not written yet, the page shows the stand-in and waits.
fn settle_shown(state: &mut ResultsParts<'_>, offered: &[OfferedAnalysis]) {
    let Some(run) = state.model.simulation.active_run() else {
        return;
    };
    let (dataset, writing) = (run.dataset_id, !run.lifecycle.is_terminal());
    let shown = state.model.simulation.view.active_analysis_idx;
    let shown_offered = shown.is_some_and(|index| offered.iter().any(|a| a.index == index));
    if shown_offered && state.results.settled_runs.contains(&dataset) {
        return;
    }
    let Some((index, first_choice)) = chosen(state.results_reads(), offered) else {
        return;
    };
    if first_choice || !writing {
        super::settle_shown_analysis(state.results, &mut state.model.simulation, index);
    } else if shown != Some(index) {
        state.model.simulation.select_analysis(index);
    }
}

/// The analysis the page shows of a run the reader has not chosen one in:
/// the one last probed, plotted or picked on the page, as the authored
/// analysis this run solved; else the run's lead analysis, the one the
/// toolbar's run chip names; else the first offered. With whether it is the
/// first of those, which a run still writing does not wait past.
fn chosen(state: ResultsReads<'_>, offered: &[OfferedAnalysis]) -> Option<(usize, bool)> {
    let followed = state.results.plot_page_followed;
    if let Some(found) = followed.and_then(|followed| {
        offered
            .iter()
            .find(|analysis| analysis.authored == followed)
    }) {
        return Some((found.index, true));
    }
    let leading = state.model.sim_setup.leading_analysis_instances();
    for (rank, lead) in leading.iter().enumerate() {
        if let Some(found) = offered
            .iter()
            .find(|analysis| analysis.authored == lead.id())
        {
            return Some((found.index, followed.is_none() && rank == 0));
        }
    }
    offered.first().map(|analysis| (analysis.index, false))
}

/// Show `analysis` on the Plot page, as a probe, a plot into the dock or the
/// page's analysis select asks: the shown run shows it from now on, and the
/// plan's next run opens the page on it. It is the page's choice for the
/// session, not the run's view's: the project keeps the data browser's.
/// Whether the shown run holds the analysis.
pub fn show_on_plot_page(state: &mut ResultsParts<'_>, analysis: AnalysisPresentationKey) -> bool {
    let Some(run) = state.model.simulation.active_run() else {
        return false;
    };
    let Some((index, found)) = analysis.resolve(run) else {
        return false;
    };
    state.results.plot_page_followed = Some(create_document::source_analysis_identity(run, found));
    super::settle_shown_analysis(state.results, &mut state.model.simulation, index);
    true
}

/// Open Results on `analysis`, the one the Plot page shows.
pub(crate) fn open_in_results(
    state: &mut ResultsParts<'_>,
    analysis: Option<AnalysisPresentationKey>,
) {
    if let Some(analysis) = analysis
        && let Some(run) = state.model.simulation.active_run()
        && let Some((index, _)) = analysis.resolve(run)
    {
        super::settle_shown_analysis(state.results, &mut state.model.simulation, index);
    }
    state
        .workbench
        .activate(rspice_app_state::workbench::workspace::Workspace::Results);
}

/// The analysis of the shown run the page plots.
fn shown_analysis(state: ResultsReads<'_>) -> Option<AnalysisPresentationKey> {
    let simulation = &state.model.simulation;
    let run = simulation.active_run()?;
    let analysis = simulation.active_analysis()?;
    Some(AnalysisPresentationKey::new(run.dataset_id, analysis))
}

/// The sheets the dock's Plot page can show for the shown analysis: the
/// ones that plot it against time, a sweep or frequency.
pub(crate) fn pane_sheets(state: ResultsReads<'_>) -> Vec<ResultViewer> {
    ResultViewer::all()
        .filter(|&viewer| pane_sheet(viewer) && super::viewer_availability(state, viewer).available)
        .collect()
}

/// Whether the dock's Plot page draws `viewer`: the sheets that plot a run
/// against time, a sweep or frequency.
const fn pane_sheet(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
    )
}

/// The state of the result the dock's Plot page draws, when it is one the
/// reader should notice, for the page's tab to carry.
pub fn plot_page_status(
    state: &mut ResultsParts<'_>,
) -> Option<rspice_results_ui::operational::ResultOperationalStatus> {
    let viewer = plot_page_viewer(state.results_reads())?;
    operational_state::notable_status(state, viewer)
}

/// The sheet the Plot page draws: the one shown when the page can draw it,
/// else the first the run can plot.
pub(crate) fn plot_page_viewer(state: ResultsReads<'_>) -> Option<ResultViewer> {
    let viewer = state.results.session.viewer;
    let sheets = pane_sheets(state);
    if sheets.contains(&viewer) {
        Some(viewer)
    } else {
        sheets.first().copied()
    }
}
