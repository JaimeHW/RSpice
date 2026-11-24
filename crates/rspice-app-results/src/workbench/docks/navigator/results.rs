//! The Results workspace's data browser: its tab band, toolbar and facets,
//! the dataset rows, and the row model the quantity list is built from.

use egui::{Response, ScrollArea, Ui};

use crate::product::DatasetId;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::docks::RunDetailsDialogState;
use crate::workbench::documents::result_document::browser_plot::result_quantity_ticked;
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, ResultsParts, retained_evidence_is_valid,
};
use crate::workbench::framework::requests::{ExportResultQuantities, FocusNavigatorSearch};
use crate::workbench::results::commands::ResultsCommand;
use rspice_results::result_presentation::{
    ResultArtifactPresentationKey, SourceWaveformPresentationKey,
};
use rspice_results_ui::selection::{ResultBrowserSelectionKey, ResultExpressionPresentationKey};

use super::super::super::commands::result_navigation::{open_run_log, ticked_result_quantities};
use super::super::super::design_system::WorkbenchIcon;
use super::results_actions::{
    compare_enabled, execute, locate_on_schematic_menu, select_result_analysis_by_key,
    select_result_browser_key, set_result_quantities_ticked,
};
use super::results_catalogue::{ListedArtifact, ListedSignals, ListingAsk, Quantity};
use super::results_rows::{
    ResultCompletenessClass, artifact_hierarchical_name, artifact_signal_kind, expression_header,
    is_plottable_artifact, result_analysis_browser_status, result_analysis_completeness,
    result_analysis_domain, result_browser_manifest_row, result_run_completeness,
    result_run_operational_label, retained_result_artifacts, show_virtualized_result_signals,
    signal_row, signal_rows,
};
use super::results_sets::{save_ticked_as_set, signal_sets_menu};
use super::results_tree::{BrowserQuery, SignalKind};
use rspice_app_workbench::workbench::docks::navigator::muted;
use rspice_app_workbench::workbench::docks::navigator::{
    PANEL_SEARCH_MARGIN_X, search_focus_requested,
};
use rspice_vocabulary::natural_order::natural_cmp;

/// Clipboard generation is synchronous on every supported target. Keep the
/// operation bounded and direct larger evidence sets to the streaming export
/// workflow instead of allocating an unbounded browser/WASM string.
pub(super) const RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT: usize = 100_000;
const RESULT_BROWSER_CLIPBOARD_BYTE_LIMIT: usize = 8_000_000;
/// The gap between the header's columns and rows, and around the controls
/// of the selection bar.
const BROWSER_GUTTER: f32 = 5.0;

pub(in crate::workbench) fn results(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    run_details: &mut Option<Box<RunDetailsDialogState>>,
) {
    if !state.results.session.checked_result_quantities.is_empty()
        && !egui::Popup::is_any_open(ui.ctx())
        && ui.input(|input| input.key_pressed(egui::Key::Escape))
    {
        state.results.session.clear_checked_signals();
    }
    let query = BrowserQuery::parse(state.workbench.navigator_filter());
    let ticked = ticked_quantities(ui.ctx(), state);
    let active_browser_tab = results_browser_active_tab(ui.ctx());
    // Kind/layout read the values the toolbar stored last frame — the row
    // renders below the tab band while filtering happens here, the same
    // one-frame contract the viewer-tab chevrons use.
    let stored_kind = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserKind>(results_browser_kind_id()))
        .unwrap_or_default();
    let layout = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserLayout>(results_browser_layout_id()))
        .unwrap_or_default();
    let stored_scope = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserScope>(results_browser_scope_id()))
        .unwrap_or_default();
    let stored_order = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserOrder>(results_browser_order_id()))
        .unwrap_or_default();
    // Internal nodes stay behind the kind list's switch whatever tab is open:
    // they are hidden from the browser, not filtered out of one listing.
    let show_internal = results_browser_shows_internal(ui.ctx());
    let stored_producer_facet = ui.ctx().data(|data| {
        data.get_temp::<Option<AnalysisPresentationKey>>(results_browser_producer_id())
            .flatten()
    });
    // Every facet belongs to the quantity list, so every one of them yields on
    // the tabs that do not list quantities.
    let kind = if active_browser_tab == ResultsBrowserTab::Signals {
        stored_kind
    } else {
        ResultsBrowserKind::All
    };
    let scope = if active_browser_tab == ResultsBrowserTab::Signals {
        stored_scope
    } else {
        ResultsBrowserScope::All
    };
    let order = if active_browser_tab == ResultsBrowserTab::Signals {
        stored_order
    } else {
        ResultsBrowserOrder::Name
    };
    let producer_facet = if active_browser_tab == ResultsBrowserTab::Signals {
        stored_producer_facet
    } else {
        None
    };
    let active_run = state
        .model
        .simulation
        .view
        .active_run_idx
        .or_else(|| state.model.simulation.retained.runs.len().checked_sub(1));
    let active_analysis = state.model.simulation.view.active_analysis_idx.or_else(|| {
        active_run.and_then(|run| {
            state
                .model
                .simulation
                .retained
                .runs
                .get(run)
                .is_some_and(|run| !run.analyses.is_empty())
                .then_some(0)
        })
    });
    let active_analysis_key = active_run
        .and_then(|run| state.model.simulation.retained.runs.get(run))
        .zip(active_analysis)
        .and_then(|(run, analysis_index)| {
            run.analyses
                .get(analysis_index)
                .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
        });
    let selected_trace = state
        .results
        .valid_selected_trace(&state.model.simulation)
        .cloned();
    let selected_artifact = state
        .results
        .session
        .selected_result_artifact
        .clone()
        .filter(|key| key.resolve(&state.model.simulation.retained.runs).is_some());
    let expression_source = active_analysis_key.map_or_else(Vec::new, |analysis| {
        state
            .results
            .expression_entries_for_analysis(&state.model.simulation, analysis)
    });
    let expressions = expression_source
        .iter()
        .enumerate()
        .filter(|(_, (_, expression))| query.matches_name(&expression.text))
        .map(|(expression_index, (identity, expression))| {
            (identity.clone(), expression_index, expression.clone())
        })
        .collect::<Vec<_>>();
    let expression_query_match = !query.is_empty() && !expressions.is_empty();
    let analysis_keys = state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .flat_map(|run| {
            run.analyses
                .iter()
                .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
        })
        .collect::<Vec<_>>();
    let analysis_integrity = analysis_keys
        .into_iter()
        .map(|key| {
            let valid = retained_evidence_is_valid(state.results_reads(), key);
            (key, valid)
        })
        .collect::<std::collections::HashMap<_, _>>();
    // Each analysis's signals are read from its catalogue, which a large
    // analysis fills a slice a frame, and listed away from the frame when the
    // analysis is large; the generation moves as either arrives.
    let catalogued = super::results_catalogue::advance(
        ui.ctx(),
        state.model.simulation.retained.runs.iter().flat_map(|run| {
            run.analyses.iter().map(move |analysis| {
                (
                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                    analysis,
                )
            })
        }),
    );
    // The row model is kept until something it is built from changes: the
    // retained runs or the state of one of their analyses, the query and the
    // facets, what was recent, or a catalogue or listing that arrived.
    // Building it reads each analysis, never each of its signals: a row reads
    // its signal from the listing, and its tick, as it is drawn.
    let fingerprint = browser_rows_fingerprint(
        &*state,
        &analysis_integrity,
        (kind, layout, order, scope, producer_facet, show_internal),
        (active_run, expression_query_match, catalogued),
    );
    let ctx = ui.ctx().clone();
    let model = browser_rows(ui.ctx(), &*state, fingerprint, || {
        BrowserModel {
        loaded: loaded_signals(&ctx, &*state, show_internal),
        runs: state.model.simulation
        .retained.runs
        .iter()
        .enumerate()
        .filter_map(|(run_index, run)| {
            let analyses = run
                .analyses
                .iter()
                .enumerate()
                .filter_map(|(analysis_index, analysis)| {
                    let presentation_key =
                        AnalysisPresentationKey::new(run.dataset_id, analysis);
                    let evidence_valid = analysis_integrity
                        .get(&presentation_key)
                        .copied()
                        .unwrap_or(false);
                    let source_currentness = crate::workbench::documents::result_document::operational_state::analysis_currentness(
                        state.results_reads(),
                        run,
                        analysis,
                        evidence_valid,
                    );
                    let currentness_label = source_currentness.id();
                    let completeness_class = result_analysis_completeness(run, analysis);
                    if producer_facet.is_some_and(|producer| producer != presentation_key) {
                        return None;
                    }
                    let (integrity, completeness) = result_analysis_browser_status(
                        run,
                        analysis,
                        evidence_valid,
                    );
                    let analysis_query_matches = query.is_empty()
                        || query.matches_word(&run.label)
                        || query.matches_word(&analysis.label)
                        || query.matches_word(analysis.analysis_type.display_name())
                        || query.matches_word(analysis.analysis_type.short_label())
                        || query.matches_word(&analysis.id.to_string())
                        || analysis.provenance().is_some_and(|provenance| {
                            query.matches_word(&provenance.source_instance_id().to_string())
                        })
                        || query.matches_word(integrity)
                        || query.matches_word(completeness)
                        || query.matches_word(currentness_label);
                    // The row's meta line stays typed metadata: kind, shape,
                    // sample count. Data state belongs to the producing
                    // analysis, so appending it here would repeat one word down
                    // every row the analysis owns; the head above them and the
                    // inspector beside them each state it once.
                    let all_artifacts = retained_result_artifacts(analysis, presentation_key);
                    let total_artifacts = all_artifacts.len();
                    let mut artifacts = all_artifacts
                        .into_iter()
                        .filter(|artifact| kind.admits_artifact(artifact.kind, &artifact.unit))
                        .filter(|artifact| {
                            analysis_query_matches
                                || query.matches_name(artifact_hierarchical_name(artifact))
                                || query.matches_word(&artifact.name)
                                || query.matches_word(artifact.identity.canonical_name())
                                || query.matches_word(artifact.kind.label())
                                || query.matches_word(&artifact.meta)
                        })
                        .filter(|artifact| match scope {
                            ResultsBrowserScope::All => true,
                            ResultsBrowserScope::Recent => state.results
                                .session.recent_result_artifact_rank(&artifact.identity)
                                .is_some(),
                        })
                        .collect::<Vec<_>>();
                    if scope == ResultsBrowserScope::Recent {
                        // Recent means recency order; the sort facet yields.
                        artifacts.sort_by_key(|artifact| {
                            state.results
                                .session.recent_result_artifact_rank(&artifact.identity)
                                .unwrap_or(usize::MAX)
                        });
                    } else if layout == ResultsBrowserLayout::Flat {
                        order.sort_artifacts(&mut artifacts);
                    }
                    // The signals the query and the facets let through, made
                    // from the analysis's catalogue: on the frame for a small
                    // analysis, away from it for a large one.
                    let signals = super::results_catalogue::listed(
                        &ctx,
                        presentation_key,
                        analysis,
                        ListingAsk {
                            query: query.clone(),
                            analysis_matches: analysis_query_matches,
                            kind,
                            show_internal,
                            order,
                            layout,
                            recent: (scope == ResultsBrowserScope::Recent).then(|| {
                                state
                                    .results
                                    .session
                                    .recent_signals
                                    .iter()
                                    .filter(|recent| recent.analysis() == presentation_key)
                                    .map(|recent| recent.source_name().to_owned())
                                    .collect()
                            }),
                            artifacts: if layout == ResultsBrowserLayout::Tree {
                                artifacts
                                    .iter()
                                    .map(|artifact| ListedArtifact {
                                        path: artifact_hierarchical_name(artifact).to_owned(),
                                        unit: artifact.unit.clone(),
                                        kind_rank: artifact_signal_kind(artifact)
                                            .map_or(u8::MAX, SignalKind::rank),
                                    })
                                    .collect()
                            } else {
                                Vec::new()
                            },
                        },
                    );
                    // An analysis whose listing is still being made stays in
                    // view, saying so, until it is.
                    let matches_analysis = analysis_query_matches
                        || signals.pending()
                        || !signals.is_empty()
                        || !artifacts.is_empty();
                    // A deck that labelled its analysis with the same code the
                    // kind glyph carries would print that code twice; the
                    // canonical name is the useful title in that case.
                    let label = if analysis
                        .label
                        .eq_ignore_ascii_case(analysis.analysis_type.short_label())
                    {
                        analysis.kind_display_name().to_owned()
                    } else {
                        analysis.label.clone()
                    };
                    matches_analysis.then(|| ResultAnalysis {
                        domain: result_analysis_domain(
                            analysis.analysis_type,
                            analysis.waveforms.first(),
                        ),
                        // Hidden internal nodes are not part of what the
                        // browser lists, so the head does not count them as
                        // filtered away.
                        total_signals: super::results_catalogue::loaded(
                            &ctx,
                            presentation_key,
                            analysis,
                            show_internal,
                        ) + total_artifacts,
                        data_state: result_analysis_data_state(
                            source_currentness,
                            completeness_class,
                        ),
                        analysis_index,
                        presentation_key,
                        label,
                        short_label: analysis.analysis_type.short_label(),
                        // A running analysis has not failed.
                        success: analysis.success || analysis.is_live_partial(),
                        signals,
                        artifacts,
                    })
                })
                .collect::<Vec<_>>();
            let run_currentness = crate::workbench::documents::result_document::operational_state::run_currentness(
                state.results_reads(),
                run,
                |analysis| {
                    analysis_integrity
                        .get(&AnalysisPresentationKey::new(run.dataset_id, analysis))
                        .copied()
                        .unwrap_or(false)
                },
            );
            let run_currentness_label = run_currentness.id();
            let matches_run = query.is_empty()
                || query.matches_word(&run.label)
                || !analyses.is_empty()
                || (active_run == Some(run_index) && expression_query_match);
            let run_integrity = if run.analyses.iter().all(|analysis| {
                analysis_integrity
                    .get(&AnalysisPresentationKey::new(run.dataset_id, analysis))
                    .copied()
                    .unwrap_or(false)
            }) {
                "integrity verified"
            } else {
                "corrupted"
            };
            let run_state = result_run_operational_label(run);
            // Every state word stays searchable even though the rows no longer
            // print them: a reader who types "corrupted" is asking a question
            // about the dataset, not about a label.
            let matches_run = matches_run
                || query.matches_word(run_integrity)
                || query.matches_word(run_state)
                || query.matches_word(run_currentness_label)
                || query.matches_word(&run.dataset_id.to_string());
            matches_run.then(|| ResultRun {
                run_index,
                dataset_id: run.dataset_id,
                label: run.label.clone(),
                name: run.name().to_owned(),
                success: run.success,
                data_state: result_analysis_data_state(
                    run_currentness,
                    result_run_completeness(run),
                ),
                analyses,
            })
        })
        .collect::<Vec<_>>(),
    }
    });
    let runs = &model.runs;

    let signal_count = runs
        .iter()
        .flat_map(|run| run.analyses.iter())
        .map(|analysis| analysis.signals.len() + analysis.artifacts.len())
        .sum::<usize>();
    let signal_list = signal_rows(
        ui.ctx(),
        runs,
        active_run,
        active_analysis,
        !query.is_empty(),
    );
    let visible_result_keys = signal_list.visible_keys(runs);
    let tab = results_browser_tab_band(ui, [signal_count, runs.len(), expressions.len()]);
    // How much of the tab's inventory a filter is holding back: two numbers
    // at the query's end, which is where the reader asked for less.
    let count_copy = {
        let shown = match tab {
            ResultsBrowserTab::Signals => signal_count,
            ResultsBrowserTab::Datasets => runs.len(),
            ResultsBrowserTab::Expressions => expressions.len(),
        };
        let loaded = match tab {
            ResultsBrowserTab::Signals => model.loaded,
            ResultsBrowserTab::Datasets => state.model.simulation.retained.runs.len(),
            ResultsBrowserTab::Expressions => expression_source.len(),
        };
        // Two numbers, no noun: the tab band above already names what is being
        // counted and the scope control below already says which working set
        // is in force, so spelling either one again costs the query the width
        // it reads in at the dock's minimum. With everything on screen the
        // tab's own count has already said it, so the field says nothing.
        // A query states its match count even when everything matches, so
        // a pattern that should have narrowed the list says that it did not.
        (shown != loaded || !query.is_empty()).then(|| format!("{shown} / {loaded}"))
    };
    // The browser's header: the query, then the producing analysis beside
    // the kind, then the scope beside the sets, on one grid. The facets
    // belong to the quantity list, so Datasets and Expressions keep the query
    // alone.
    results_browser_toolbar(
        ui,
        state,
        BrowserFacets {
            kind: stored_kind,
            layout,
            producer: stored_producer_facet,
            order,
            scope,
        },
        (tab == ResultsBrowserTab::Signals).then_some(&ticked),
        count_copy.as_deref(),
    );
    // What a batch action acts on: the ticked quantities, which for a
    // waveform are the curves on the plot. The bar stands at the foot of the
    // dock, under the rows: over them, the first tick brought it in and
    // moved every row down under the pointer, so the next click there
    // unticked what had just been ticked.
    if tab == ResultsBrowserTab::Signals && !ticked.is_empty() {
        let t = Tokens::get(ui.ctx());
        egui::Panel::bottom("workbench.results.selection-bar")
            .resizable(false)
            .show_separator_line(false)
            .exact_size(t.metrics.ctl_h + 2.0 * BROWSER_GUTTER)
            .frame(egui::Frame::new().fill(t.color.bg_panel))
            .show(ui, |ui| {
                // The bar opens on a rule, as the header closes on one.
                ui.painter().hline(
                    ui.max_rect().x_range(),
                    ui.max_rect().top() + 0.5,
                    egui::Stroke::new(1.0, t.color.border),
                );
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.add_space(PANEL_SEARCH_MARGIN_X);
                        ui.label(
                            egui::RichText::new(format!("{} ticked", ticked.len()))
                                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                                .color(t.color.text),
                        );
                        ui.add_space(2.0 * BROWSER_GUTTER);
                        let clear = ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("Clear")
                                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                        .color(t.color.text_dim),
                                )
                                .frame(false)
                                .min_size(egui::vec2(0.0, t.metrics.ctl_h)),
                            )
                            .on_hover_text("Untick every signal");
                        clear.widget_info(|| {
                            egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "Clear ticks")
                        });
                        if clear.clicked() {
                        let ordered = ticked.ordered(state);
                        set_result_quantities_ticked(state, ordered, false);
                        state.results.session.clear_checked_signals();
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(PANEL_SEARCH_MARGIN_X);
                            let actions = rspice_ui_kit::widgets::Button::new("Actions")
                                .menu()
                                .show(ui);
                            button_menu(&actions, egui::PopupCloseBehavior::CloseOnClickOutside, |ui| {
                // What the actions need to know of the ticked quantities is
                // read while the menu is open, not on every frame.
                let TickedFacts {
                    exact_validation_error,
                    clipboard_sample_count,
                    datasets,
                    names,
                } = ticked.facts(state).clone();
                let exact_evidence_available = exact_validation_error.is_none();
                let clipboard_exact_available = exact_evidence_available
                    && !ticked.is_empty()
                    && clipboard_sample_count <= RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT;
                let compare_selection_compatible =
                    exact_evidence_available && ticked.len() >= 2 && datasets >= 2 && names == 1;
                if ui
                    .add_enabled(
                        signal_list.has_quantities(),
                        egui::Button::new("Tick visible quantities"),
                    )
                    .clicked()
                {
                    let visible = visible_result_keys.keys();
                    set_result_quantities_ticked(state, visible, true);
                    if let Some(last) = visible.last() {
                        state.results.session.set_browser_range_anchor(last.clone());
                    }
                    ui.close();
                }
                if ui
                    .add_enabled(
                        !ticked.is_empty(),
                        egui::Button::new("Copy names"),
                    )
                    .clicked()
                {
                    let result = ticked
                        .ordered(state)
                        .iter()
                        .map(|key| {
                            crate::workbench::documents::result_document::result_browser_selection_canonical_name(
                                key,
                                &state.model.simulation.retained.runs,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .map(|names| names.join("\n"));
                    match result {
                        Ok(names) => ui.ctx().copy_text(names),
                        Err(error) => state.workbench.prompt(error),
                    }
                    ui.close();
                }
                let copy_exact = ui.add_enabled(
                    clipboard_exact_available,
                    egui::Button::new("Copy data"),
                );
                if !ticked.is_empty() && !clipboard_exact_available {
                    copy_exact.clone().on_disabled_hover_text(
                        exact_validation_error.clone().unwrap_or_else(|| format!(
                            "The waveform portion holds {clipboard_sample_count} samples; clipboard copy is limited to {RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT}. Export it instead."
                        )),
                    );
                }
                if copy_exact.clicked() {
                    match crate::workbench::documents::result_document::exact_result_browser_selection_bundle(
                        ticked.ordered(state),
                        &state.model.simulation.retained.runs,
                    ) {
                        Ok(exact) if exact.len() <= RESULT_BROWSER_CLIPBOARD_BYTE_LIMIT => {
                            ui.ctx().copy_text(exact);
                        }
                        Ok(exact) => state.workbench.prompt(format!(
                            "The selection is {} bytes; clipboard copy is limited to {RESULT_BROWSER_CLIPBOARD_BYTE_LIMIT} bytes. Export it instead.",
                            exact.len()
                        )),
                        Err(error) => state.workbench.prompt(error),
                    }
                    ui.close();
                }
                let export_exact = ui.add_enabled(
                    exact_evidence_available && !ticked.is_empty(),
                    egui::Button::new("Export\u{2026}"),
                );
                if let Some(error) = exact_validation_error.as_deref() {
                    export_exact.clone().on_disabled_hover_text(error);
                }
                if export_exact.clicked() {
                    let ordered = ticked.ordered(state).to_vec();
                    state.workbench
                        .outbox
                        .push(ExportResultQuantities(ordered));
                    ui.close();
                }
                ui.separator();
                let compare = ui.add_enabled(
                    compare_selection_compatible
                        && compare_enabled(state),
                    egui::Button::new("Compare across datasets\u{2026}"),
                );
                if !ticked.is_empty() && !compare_selection_compatible {
                    compare.clone().on_disabled_hover_text(
                        exact_validation_error.as_deref().unwrap_or(
                            "Select the same signal in at least two runs.",
                        ),
                    );
                }
                if compare.clicked() {
                    if let Some(first) = ticked.ordered(state).first()
                        && select_result_browser_key(state, first)
                    {
                        execute(state, ResultsCommand::CompareResultDatasets.key());
                    }
                    ui.close();
                }
                ui.separator();
                rspice_ui_kit::widgets::menu_button(ui, "Save ticked as set\u{2026}", |ui| {
                    save_ticked_as_set(ui, state, &ticked);
                });
                            });
                        });
                    },
                );
            });
    }
    if tab == ResultsBrowserTab::Signals {
        show_virtualized_result_signals(
            ui,
            state,
            runs,
            &signal_list,
            active_run,
            active_analysis,
            selected_trace.as_ref(),
            selected_artifact.as_ref(),
            &visible_result_keys,
            &ticked,
            !query.is_empty(),
            kind,
            scope,
        );
        return;
    }
    ScrollArea::vertical()
        .id_salt("workbench.results.navigator")
        .auto_shrink([false, true])
        .show(ui, |ui| match tab {
            ResultsBrowserTab::Signals => {
                unreachable!("signal rows return through the virtualized result browser")
            }
            ResultsBrowserTab::Datasets => {
                if runs.is_empty() {
                    muted(
                        ui,
                        if state.model.simulation.retained.runs.is_empty() {
                            "No results yet — run a simulation"
                        } else {
                            "No dataset or analysis matches this filter."
                        },
                    );
                }
                for run in runs.iter() {
                    let run_active = active_run == Some(run.run_index);
                    // A run whose results store no folder holds, or holds damaged, says so
                    // before anything about its data.
                    let unread = state
                        .model
                        .simulation
                        .retained
                        .runs
                        .get(run.run_index)
                        .and_then(crate::state::SimulationRun::unread_results);
                    let checking = state
                        .model
                        .simulation
                        .retained
                        .runs
                        .get(run.run_index)
                        .and_then(crate::state::SimulationRun::checking_results);
                    let checking_meta;
                    let run_meta = if let Some((done, total)) = checking {
                        checking_meta = match done.saturating_mul(100).checked_div(total) {
                            Some(percent) => format!("checking {percent}%"),
                            None => "checking".to_owned(),
                        };
                        checking_meta.as_str()
                    } else if unread.is_some() {
                        "results not found"
                    } else {
                        data_state_meta(run.data_state)
                    };
                    let overlaid = state.model.simulation.is_dataset_overlaid(run.dataset_id);
                    let pinned = state
                        .model
                        .simulation
                        .retained
                        .runs
                        .get(run.run_index)
                        .is_some_and(|retained| retained.retention().is_pinned());
                    let pinned_meta;
                    let run_meta = match (pinned, run_meta.is_empty()) {
                        (false, _) => run_meta,
                        (true, true) => "pinned",
                        (true, false) => {
                            pinned_meta = format!("pinned \u{b7} {run_meta}");
                            pinned_meta.as_str()
                        }
                    };
                    let responses = result_dataset_row(
                        ui,
                        if run.success {
                            WorkbenchIcon::Success
                        } else {
                            WorkbenchIcon::Warning
                        },
                        &run.label,
                        run_active,
                        run_meta,
                        overlaid,
                    );
                    if responses.selection.clicked() {
                        // A plan's run is shown through the plan's view and
                        // stays shown, as choosing it on the toolbar's run
                        // selector does.
                        if let Err(error) =
                            crate::workbench::chrome::toolbar::results::run_selector::show_run(
                                state,
                                run.dataset_id,
                            )
                        {
                            state
                                .workbench
                                .prompt(format!("Could not show {}: {error}", run.label));
                        }
                        state.results.session.selected_trace = None;
                        state.results.session.selected_result_artifact = None;
                    }
                    if let Some(reason) = &unread {
                        responses.selection.clone().on_hover_text(format!(
                            "The results of {} cannot be read: {reason}",
                            run.label
                        ));
                    }
                    let mut details = false;
                    let mut log = false;
                    let mut locate = false;
                    responses.selection.context_menu(|ui| {
                        super::results_runs::run_menu(ui, state, run.dataset_id);
                        ui.separator();
                        if unread.is_some() && ui.button("Locate results\u{2026}").clicked() {
                            locate = true;
                            ui.close();
                        }
                        if ui.button("Run details\u{2026}").clicked() {
                            details = true;
                            ui.close();
                        }
                        if ui.button("Open run log").clicked() {
                            log = true;
                            ui.close();
                        }
                    });
                    if locate {
                        locate_results(state);
                    }
                    if let Some(run_id) = state
                        .model
                        .simulation
                        .retained
                        .runs
                        .get(run.run_index)
                        .map(|retained| retained.id)
                    {
                        if details {
                            super::super::inspector::run_details::open(
                                run_details,
                                &state.model.simulation,
                                run_id,
                            );
                        }
                        if log {
                            open_run_log(state.model, state.workbench, run_id);
                        }
                    }
                    if responses.overlay.is_some_and(|response| response.clicked()) {
                        let enabled = state
                            .model
                            .simulation
                            .toggle_dataset_overlay(run.dataset_id);
                        state.workbench.prompt(if enabled {
                            format!("Overlaying {} on the active result sheet.", run.label)
                        } else {
                            format!("Removed {} from the active result sheet.", run.label)
                        });
                    }
                    if !run_active && query.is_empty() {
                        continue;
                    }
                    for analysis in &run.analyses {
                        let analysis_active =
                            run_active && active_analysis == Some(analysis.analysis_index);
                        if result_browser_manifest_row(
                            ui,
                            analysis.short_label,
                            &analysis.label,
                            &analysis.domain,
                            analysis_active,
                            analysis.success,
                        )
                        .clicked()
                        {
                            select_result_analysis_by_key(state, analysis.presentation_key);
                        }
                    }
                }
            }
            ResultsBrowserTab::Expressions => {
                // Both ways of having no analysis read the same to the
                // reader: there is one to pick and they have not picked it.
                if state.model.simulation.view.active_analysis_idx.is_none()
                    || active_analysis_key.is_none()
                {
                    muted(ui, "Select an analysis to see its expressions.");
                    return;
                }
                expression_header(ui, state);
                let mut toggled_expression: Option<ResultExpressionPresentationKey> = None;
                for (identity, expression_index, expression) in &expressions {
                    let t = Tokens::get(ui.ctx());
                    let row_id =
                        ui.id()
                            .with(("result-expression", identity.analysis(), identity.text()));
                    let responses = signal_row(
                        ui,
                        row_id,
                        &expression.text,
                        // The tab the row sits under is the only place that
                        // has to say these are expressions.
                        None,
                        t.color.traces[*expression_index % t.color.traces.len()],
                        false,
                        expression.visible,
                    );
                    if responses.visibility.clicked() {
                        toggled_expression = Some(identity.clone());
                    }
                    locate_on_schematic_menu(&responses.selection, state, &expression.text);
                }
                if let Some(identity) = toggled_expression {
                    match state
                        .results
                        .toggle_expression_visibility_by_key(&state.model.simulation, &identity)
                    {
                        Ok(()) => {
                            state.model.workspace.content.visualization_documents_dirty = true;
                        }
                        Err(error) => state.workbench.prompt(error),
                    }
                }
            }
        });
}

/// The ticked quantities, kept between frames with what they were read
/// from: reading them walks every signal ticked, so a tick changes them by
/// the signals it edited instead. The order the runs hold them in, and what
/// the band of batch actions needs to know of them, are read when an action
/// asks for them, never on the frame a tick changes them.
#[derive(Clone)]
pub(super) struct TickedQuantities {
    revision: crate::state::RunHistoryRevision,
    /// The reader's visibility overrides, compared by revision.
    visibility: rspice_vocabulary::source_revision::SourceRevision,
    fingerprint: u64,
    pub(super) ticked: std::sync::Arc<std::collections::HashSet<ResultBrowserSelectionKey>>,
    /// Read the first time an action asks after the ticks changed.
    listed: std::sync::Arc<std::sync::OnceLock<TickedList>>,
}

/// The ticked quantities in the order the runs hold them, each resolved
/// once, and what the band of batch actions needs to know of them.
struct TickedList {
    ordered: Vec<ResultBrowserSelectionKey>,
    facts: TickedFacts,
}

impl TickedQuantities {
    pub(super) fn len(&self) -> usize {
        self.ticked.len()
    }

    pub(super) fn is_empty(&self) -> bool {
        self.ticked.is_empty()
    }

    pub(super) fn contains(&self, key: &ResultBrowserSelectionKey) -> bool {
        self.ticked.contains(key)
    }

    /// The ticked quantities in the order the runs hold them: what a set
    /// saves and the band's actions act on.
    pub(super) fn ordered(&self, state: &ResultsParts<'_>) -> &[ResultBrowserSelectionKey] {
        &self.listed(state).ordered
    }

    fn facts(&self, state: &ResultsParts<'_>) -> &TickedFacts {
        &self.listed(state).facts
    }

    fn listed(&self, state: &ResultsParts<'_>) -> &TickedList {
        self.listed.get_or_init(|| {
            let selection =
                crate::workbench::documents::result_document::resolve_result_browser_selection(
                    &self.ticked,
                    &state.model.simulation.retained.runs,
                    (&state.results.session.signal_indices, &self.revision),
                );
            let facts = TickedFacts::of(&selection, state);
            TickedList {
                ordered: selection.into_iter().map(|(key, _)| key).collect(),
                facts,
            }
        })
    }
}

/// The ticked quantities, read again whole only when the retained runs or
/// something else they are read from changed since the frame that last read
/// them, or the reader's ticks changed in ways no longer known; otherwise
/// changed by the signals whose ticks were edited since.
pub(super) fn ticked_quantities(ctx: &egui::Context, state: &ResultsParts<'_>) -> TickedQuantities {
    let id = egui::Id::new("rspice.results.browser.ticked");
    let revision = state.model.simulation.retained.runs.revision();
    let visibility = state.results.session.waveform_visibility.revision();
    let fingerprint = ticked_fingerprint(state);
    // Taken out of the frames' memory, so an edit changes the set they hold
    // in place rather than a copy of it.
    let kept = ctx
        .data_mut(|data| {
            data.get_temp_mut_or_default::<Option<TickedQuantities>>(id)
                .take()
        })
        .filter(|kept| kept.revision == revision && kept.fingerprint == fingerprint);
    let kept = match kept {
        Some(kept) if kept.visibility == visibility => kept,
        Some(kept) => follow_ticks(kept, state, visibility.clone())
            .unwrap_or_else(|| read_ticked(state, revision, visibility, fingerprint)),
        None => read_ticked(state, revision, visibility, fingerprint),
    };
    ctx.data_mut(|data| data.insert_temp(id, Some(kept.clone())));
    kept
}

/// Every ticked quantity, read whole.
pub(super) fn read_ticked(
    state: &ResultsParts<'_>,
    revision: crate::state::RunHistoryRevision,
    visibility: rspice_vocabulary::source_revision::SourceRevision,
    fingerprint: u64,
) -> TickedQuantities {
    let mut ticked = ticked_result_quantities(state.model, state.results);
    // An event net's or a bus's row is ticked while its lane is on the plot,
    // as a curve's row is while the curve is.
    ticked.extend(plotted_event_rows(state));
    TickedQuantities {
        revision,
        visibility,
        fingerprint,
        ticked: std::sync::Arc::new(ticked),
        listed: std::sync::Arc::default(),
    }
}

/// `kept`, changed by the reader's ticks of the signals edited since it was
/// read: each edited signal of the active run is ticked again as a whole
/// read ticks it, with the curves it overlays on the runs overlaid, and an
/// event analysis an edit touched has its rows read again. `None` when the
/// edits are no longer known or an edited analysis has no index yet: only a
/// whole read follows.
pub(super) fn follow_ticks(
    mut kept: TickedQuantities,
    state: &ResultsParts<'_>,
    visibility: rspice_vocabulary::source_revision::SourceRevision,
) -> Option<TickedQuantities> {
    use std::collections::HashSet;
    let edited: Vec<&SourceWaveformPresentationKey> = state
        .results
        .session
        .waveform_visibility
        .edits_since(&kept.visibility)?
        .collect();
    let simulation = &state.model.simulation;
    let checked = &state.results.session.checked_result_quantities;
    if let Some(active) = simulation.active_run() {
        let revision = simulation.retained.runs.revision();
        let indices = &state.results.session.signal_indices;
        let overlaid = simulation
            .view
            .overlay_dataset_ids
            .iter()
            .filter_map(|dataset| simulation.retained.run_by_dataset_id(*dataset))
            .collect::<Vec<_>>();
        // Each analysis of the active run with its index, and the analyses of
        // the runs overlaid on it with theirs, as a whole read pairs them.
        let analyses = active
            .analyses
            .iter()
            .map(|analysis| {
                let key = AnalysisPresentationKey::new(active.dataset_id, analysis);
                let overlays = overlaid
                    .iter()
                    .filter_map(|run| {
                        let other = rspice_results_ui::waves::matching_overlay_analysis(
                            analysis.as_ref(),
                            run,
                        )?;
                        let other_key = AnalysisPresentationKey::new(run.dataset_id, other);
                        Some((other_key, other, indices.index(&revision, other_key, other)))
                    })
                    .collect::<Vec<_>>();
                (
                    key,
                    analysis,
                    indices.index(&revision, key, &analysis.data),
                    overlays,
                )
            })
            .collect::<Vec<_>>();
        // Whether a whole read ticks `signal` of the active run's analysis
        // `at` by its column `column`: that column is on the plot.
        let column_plotted =
            |at: usize, signal: &SourceWaveformPresentationKey, column: usize| -> bool {
                let waveform = &analyses[at].1.waveforms[column];
                crate::workbench::documents::result_document::waveform_plotted(
                    state.model,
                    state.results,
                    signal,
                    &waveform.name,
                    waveform.visible,
                )
            };
        // Whether a whole read ticks `signal` of the active run's analysis
        // `at`: one of its columns of that name is on the plot.
        let plotted = |at: usize, signal: &SourceWaveformPresentationKey| -> Option<bool> {
            let (_, analysis, index, _) = &analyses[at];
            Some(
                index
                    .as_ref()?
                    .columns_named(&analysis.data, signal.source_name())
                    .any(|column| column_plotted(at, signal, column)),
            )
        };
        // The edited signals of each analysis of the active run, by its place.
        let mut edits = vec![Vec::new(); analyses.len()];
        for signal in edited {
            // Another run's ticks are not followed: a whole read reads the
            // active run's.
            if let Some(at) = analyses
                .iter()
                .position(|(key, ..)| *key == signal.analysis())
            {
                edits[at].push(signal);
            }
        }
        let ticked = std::sync::Arc::make_mut(&mut kept.ticked);
        let mut events = HashSet::new();
        for (at, signals) in edits.into_iter().enumerate() {
            if signals.is_empty() {
                continue;
            }
            let (_, analysis, index, overlays) = &analyses[at];
            let index = index.as_ref()?;
            if matches!(
                analysis.result_payload,
                Some(crate::state::AnalysisResultPayload::TransientEvents { .. })
            ) {
                events.insert(at);
            }
            // Each signal's column, and whether each overlaid analysis has a
            // signal of its name, found together; a name no column or more
            // than one has is looked for column by column.
            let names: Vec<&str> = signals.iter().map(|signal| signal.source_name()).collect();
            let found = index.columns(&analysis.data, &names);
            let overlaid = overlays
                .iter()
                .map(|(_, other, other_index)| match other_index {
                    Some(index) => index
                        .first_columns(other, &names)
                        .into_iter()
                        .map(|column| column.is_some())
                        .collect(),
                    None => names
                        .iter()
                        .map(|name| {
                            other
                                .waveforms
                                .iter()
                                .any(|waveform| waveform.name == *name)
                        })
                        .collect::<Vec<_>>(),
                })
                .collect::<Vec<_>>();
            for (place, (signal, found)) in signals.into_iter().zip(found).enumerate() {
                let name = signal.source_name();
                let on = match found {
                    Some(column) => column_plotted(at, signal, column),
                    None => plotted(at, signal)?,
                };
                let own = ResultBrowserSelectionKey::Waveform(signal.clone());
                if on {
                    ticked.insert(own);
                } else if !checked.contains(&own) {
                    ticked.remove(&own);
                }
                for ((other_key, ..), present) in overlays.iter().zip(&overlaid) {
                    if !present[place] {
                        continue;
                    }
                    let overlay = ResultBrowserSelectionKey::Waveform(
                        SourceWaveformPresentationKey::new(*other_key, name),
                    );
                    // Another analysis of the active run on the same overlaid
                    // one may still plot the signal there.
                    let still =
                        on || checked.contains(&overlay)
                            || analyses.iter().enumerate().any(
                                |(other_place, (key, _, _, others))| {
                                    other_place != at
                                        && others.iter().any(|(other, ..)| other == other_key)
                                        && plotted(
                                            other_place,
                                            &SourceWaveformPresentationKey::new(*key, name),
                                        ) == Some(true)
                                },
                            );
                    if still {
                        ticked.insert(overlay);
                    } else {
                        ticked.remove(&overlay);
                    }
                }
            }
        }
        // An event analysis's rows are read again as a whole read reads
        // them: an edit of a net may tick or untick a bus that holds it.
        for at in events {
            let (key, analysis, ..) = &analyses[at];
            for artifact in retained_result_artifacts(analysis, *key) {
                if !is_plottable_artifact(artifact.identity.canonical_name()) {
                    continue;
                }
                let row = ResultBrowserSelectionKey::Artifact(artifact.identity);
                if result_quantity_ticked(state, &row) || checked.contains(&row) {
                    ticked.insert(row);
                } else {
                    ticked.remove(&row);
                }
            }
        }
    }
    kept.visibility = visibility;
    kept.listed = std::sync::Arc::default();
    Some(kept)
}

/// Everything besides the retained runs themselves that what is ticked is
/// read from, as one number: the data version, the active run and its
/// overlays, the quantities ticked by hand, and the persistent pane whose
/// traces decide a curve's tick. The reader's visibility overrides are
/// compared by their revision instead.
fn ticked_fingerprint(state: &ResultsParts<'_>) -> u64 {
    use std::hash::{Hash, Hasher};
    let stable = rspice_results_ui::waves::stable_hash;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let view = &state.model.simulation.view;
    (view.data_version, view.active_run_idx).hash(&mut hasher);
    view.overlay_dataset_ids.hash(&mut hasher);
    let session = &state.results.session;
    // A set's or a map's order is its hasher's: combine its members in any
    // order.
    session
        .checked_result_quantities
        .iter()
        .fold(0_u64, |combined, key| combined ^ stable(key))
        .hash(&mut hasher);
    session.checked_result_quantities.len().hash(&mut hasher);
    if let Some(context) = session.persistent_pane_context {
        (context.document_id, context.pane_id, context.analysis).hash(&mut hasher);
        state
            .model
            .workspace
            .content
            .visualization_document(context.document_id)
            .map(|document| document.revision())
            .hash(&mut hasher);
    }
    hasher.finish()
}

/// What the band of batch actions needs to know of the ticked quantities.
#[derive(Clone)]
struct TickedFacts {
    /// Why the first quantity that gives no exact values gives none.
    exact_validation_error: Option<String>,
    /// The samples copying the quantities would put on the clipboard.
    clipboard_sample_count: usize,
    /// The runs the quantities come from.
    datasets: usize,
    /// The distinct names the quantities have.
    names: usize,
}

impl TickedFacts {
    fn of(
        selection: &[(
            ResultBrowserSelectionKey,
            Option<crate::workbench::documents::result_document::ResolvedSignal<'_>>,
        )],
        state: &ResultsParts<'_>,
    ) -> Self {
        let exact_validation_error = selection.iter().find_map(|(key, signal)| {
            crate::workbench::documents::result_document::resolved_selection_evidence(
                key,
                *signal,
                state.results,
                state.model,
            )
            .err()
        });
        let clipboard_sample_count = selection
            .iter()
            .filter_map(|(_, signal)| signal.map(|(.., waveform)| waveform.x.len()))
            .sum::<usize>();
        let datasets = selection
            .iter()
            .map(|(key, _)| key.dataset_id())
            .collect::<std::collections::HashSet<_>>()
            .len();
        let names = selection
            .iter()
            .filter_map(|(key, signal)| match key {
                ResultBrowserSelectionKey::Waveform(_) => {
                    signal.map(|(.., waveform)| waveform.name.as_str())
                }
                ResultBrowserSelectionKey::Artifact(artifact) => Some(artifact.canonical_name()),
            })
            .collect::<std::collections::HashSet<_>>()
            .len();
        Self {
            exact_validation_error,
            clipboard_sample_count,
            datasets,
            names,
        }
    }
}

/// Which content the results data browser shows. Stored in egui memory like
/// the pane log toggles: browser focus is a session gesture, not document
/// state.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ResultsBrowserTab {
    #[default]
    Signals,
    Datasets,
    Expressions,
}

/// The data browser's row model, kept between frames with what it was built
/// from.
#[derive(Clone)]
struct BrowserRows {
    revision: crate::state::RunHistoryRevision,
    fingerprint: u64,
    model: std::sync::Arc<BrowserModel>,
}

/// The rows the filters let through, and how many quantities there are
/// before them.
struct BrowserModel {
    runs: Vec<ResultRun>,
    /// Every retained analysis's quantities the browser lists before its
    /// filters: its signals, less internal nodes unless they are shown, and
    /// its other results.
    loaded: usize,
}

/// The quantities the browser lists before its filters, across every
/// retained analysis.
fn loaded_signals(ctx: &egui::Context, state: &ResultsParts<'_>, show_internal: bool) -> usize {
    state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .flat_map(|run| {
            run.analyses.iter().map(move |analysis| {
                let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
                super::results_catalogue::loaded(ctx, key, analysis, show_internal)
                    + retained_result_artifacts(analysis, key).len()
            })
        })
        .sum()
}

/// The row model `build` makes, built again only when the retained runs or
/// `fingerprint` changed since the frame that last built it.
fn browser_rows(
    ctx: &egui::Context,
    state: &ResultsParts<'_>,
    fingerprint: u64,
    build: impl FnOnce() -> BrowserModel,
) -> std::sync::Arc<BrowserModel> {
    let id = egui::Id::new("rspice.results.browser.row-model");
    let revision = state.model.simulation.retained.runs.revision();
    if let Some(kept) = ctx.data(|data| data.get_temp::<BrowserRows>(id))
        && kept.revision == revision
        && kept.fingerprint == fingerprint
    {
        return kept.model;
    }
    crate::workbench::documents::result_document::frame_work::note(
        crate::workbench::documents::result_document::frame_work::DatasetWalk::BrowserRows,
    );
    let model = std::sync::Arc::new(build());
    ctx.data_mut(|data| {
        data.insert_temp(
            id,
            BrowserRows {
                revision,
                fingerprint,
                model: std::sync::Arc::clone(&model),
            },
        );
    });
    model
}

/// Everything besides the retained runs themselves that the row model reads,
/// as one number: the data version, the query, the facets, the reader's
/// recent signals where the Recent scope lists them, the active run, each
/// analysis's integrity and currentness, and the generation of the
/// catalogues and listings.
fn browser_rows_fingerprint(
    state: &ResultsParts<'_>,
    integrity: &std::collections::HashMap<AnalysisPresentationKey, bool>,
    (kind, layout, order, scope, producer, show_internal): (
        ResultsBrowserKind,
        ResultsBrowserLayout,
        ResultsBrowserOrder,
        ResultsBrowserScope,
        Option<AnalysisPresentationKey>,
        bool,
    ),
    (active_run, expression_query_match, catalogued): (Option<usize>, bool, u64),
) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    state.model.simulation.view.data_version.hash(&mut hasher);
    state.workbench.navigator_filter().hash(&mut hasher);
    (kind as u8, layout as u8, order as u8, scope as u8).hash(&mut hasher);
    (producer, show_internal, active_run, expression_query_match).hash(&mut hasher);
    catalogued.hash(&mut hasher);
    // Only the Recent scope lists by recency: elsewhere a signal ticked, and
    // so used, leaves the rows as they were.
    if scope == ResultsBrowserScope::Recent {
        state.results.session.recent_signals.hash(&mut hasher);
        state
            .results
            .session
            .recent_result_artifacts
            .hash(&mut hasher);
    }
    let reads = state.results_reads();
    for run in state.model.simulation.retained.runs.iter() {
        let valid = |analysis: &crate::state::AnalysisResult| {
            integrity
                .get(&AnalysisPresentationKey::new(run.dataset_id, analysis))
                .copied()
                .unwrap_or(false)
        };
        crate::workbench::documents::result_document::operational_state::run_currentness(
            reads, run, valid,
        )
        .id()
        .hash(&mut hasher);
        for analysis in &run.analyses {
            let valid = valid(analysis);
            (
                valid,
                crate::workbench::documents::result_document::operational_state::analysis_currentness(
                    reads, run, analysis, valid,
                )
                .id(),
            )
                .hash(&mut hasher);
        }
    }
    hasher.finish()
}

/// Quantity-kind facet of the browser toolbar.
///
/// Waveform kinds come from their exact engineering unit. Typed producer
/// artifacts carry their own scalar/array/event/contribution classification,
/// so those facets never guess from a display label.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ResultsBrowserKind {
    #[default]
    All,
    Voltage,
    Current,
    Power,
    NoiseDensity,
    Scalar,
    Array,
    EventStream,
    Contribution,
    OtherUnits,
}

impl ResultsBrowserKind {
    const ALL: [Self; 10] = [
        Self::All,
        Self::Voltage,
        Self::Current,
        Self::Power,
        Self::NoiseDensity,
        Self::Scalar,
        Self::Array,
        Self::EventStream,
        Self::Contribution,
        Self::OtherUnits,
    ];

    /// The base electrical dimensions. The facet closes with one negation over
    /// this set, so "anything that is not a volt, an amp, a watt, or a noise
    /// density" stays askable without a second unit control beside this one.
    pub const BASE_UNITS: [&'static str; 15] = [
        "V", "A", "W", "ns/√Hz", "s/√Hz", "s²/Hz", "s^2/Hz", "nV/√Hz", "nA/√Hz", "V/√Hz", "A/√Hz",
        "V^2/Hz", "A^2/Hz", "V²/Hz", "A²/Hz",
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::All => "All kinds",
            Self::Voltage => "Voltage",
            Self::Current => "Current",
            Self::Power => "Power",
            Self::NoiseDensity => "Noise density",
            Self::Scalar => "Scalar",
            Self::Array => "Array",
            Self::EventStream => "Event stream",
            Self::Contribution => "Contribution",
            Self::OtherUnits => "Other units",
        }
    }

    /// Whether a signal reading in `unit` belongs to this kind.
    pub fn admits(self, unit: &str) -> bool {
        match self {
            Self::All => true,
            Self::Voltage => unit == "V",
            Self::Current => unit == "A",
            Self::Power => unit == "W",
            Self::Scalar | Self::Array | Self::EventStream | Self::Contribution => false,
            Self::NoiseDensity => matches!(
                unit,
                "ns/√Hz"
                    | "s/√Hz"
                    | "s²/Hz"
                    | "s^2/Hz"
                    | "nV/√Hz"
                    | "nA/√Hz"
                    | "V/√Hz"
                    | "A/√Hz"
                    | "V^2/Hz"
                    | "A^2/Hz"
                    | "V²/Hz"
                    | "A²/Hz"
            ),
            Self::OtherUnits => !Self::BASE_UNITS.contains(&unit),
        }
    }

    /// A typed artifact answers to its own shape class, and to the closing
    /// negation through the unit it carries.
    pub fn admits_artifact(self, kind: ResultArtifactKind, unit: &str) -> bool {
        match self {
            Self::All => true,
            Self::Scalar => kind == ResultArtifactKind::Scalar,
            Self::Array => kind == ResultArtifactKind::Array,
            Self::EventStream => kind == ResultArtifactKind::EventStream,
            Self::Contribution => kind == ResultArtifactKind::Contribution,
            Self::Voltage | Self::Current | Self::Power | Self::NoiseDensity => false,
            Self::OtherUnits => !Self::BASE_UNITS.contains(&unit),
        }
    }
}

/// How the signals tab lays its quantities out: as the design's instance
/// tree, or as one flat list of full names — EZwave's "hierarchical (tree)
/// or flat (list) format" — each in the [`ResultsBrowserOrder`] in force.
/// Both count numbers in names (`L2` before `L10`). Recency is the Recent
/// scope on the band below: one owner for it, not a further order that
/// silently competes with it.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub(super) enum ResultsBrowserLayout {
    #[default]
    Tree,
    Flat,
}

impl ResultsBrowserLayout {
    const ALL: [Self; 2] = [Self::Tree, Self::Flat];

    const fn label(self) -> &'static str {
        match self {
            Self::Tree => "Tree",
            Self::Flat => "Flat",
        }
    }
}

/// Working-set scope of the signals tab: everything, or the signals touched
/// most recently, from the real recency state on `ResultsState`. A working
/// set the reader wants to keep is a signal set, saved with the project.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub(super) enum ResultsBrowserScope {
    #[default]
    All,
    Recent,
}

/// One reading per producing analysis, merged from the one retained-evidence
/// read that currentness, integrity, and completeness all resolve from.
///
/// It is stated, never filtered on. Within a dataset binding every producer is
/// visible at once, so a facet over these words would narrow a list the reader
/// can already read; and because integrity failure *is* corrupted currentness,
/// separate controls could be aimed at each other to name an empty set. The
/// word has three homes and no fourth: the dataset row states its binding's
/// condition, the analysis head's retained fraction carries the reason on its
/// tooltip, and the inspector states it for one selected quantity.
/// The word and its consequence travel together, so no caller can look one up
/// from the other's display text.
#[derive(Clone, Copy)]
pub struct ResultAnalysisDataState {
    pub word: &'static str,
    pub note: &'static str,
}

/// A dataset row's state word, only when it asks something of the reader: a
/// current dataset is the normal case and its row says nothing about it.
///
/// Nor does an unresolved one. "Unresolved" means the run records no source
/// revision to compare with the design: it came from a netlist the reader
/// ran, a file they imported, or a session from before runs recorded one.
/// The results are exact and nothing about them can be brought up to date
/// from here, so the word asked a question the reader had no way to answer.
/// It stays searchable, as every state word does.
pub(super) fn data_state_meta(state: ResultAnalysisDataState) -> &'static str {
    match state.word {
        "current" | "unresolved" => "",
        word => word,
    }
}

pub const fn result_analysis_data_state(
    currentness: crate::workbench::documents::result_document::operational_state::ResultCurrentness,
    completeness: ResultCompletenessClass,
) -> ResultAnalysisDataState {
    use crate::workbench::documents::result_document::operational_state::ResultCurrentness;

    const fn state(word: &'static str, note: &'static str) -> ResultAnalysisDataState {
        ResultAnalysisDataState { word, note }
    }

    // Severity first: an unreadable payload outranks a short retained scope,
    // which outranks any claim about which source revision produced it.
    match (currentness, completeness) {
        (ResultCurrentness::Corrupted, _) => state(
            "corrupted",
            "Payload validation failed on load. Metadata stays readable; numeric actions stay blocked.",
        ),
        (_, ResultCompletenessClass::Failed) => state(
            "failed",
            "The analysis stopped before it finished. Only the points it computed are kept; nothing is extrapolated.",
        ),
        (_, ResultCompletenessClass::Cancelled) => state(
            "cancelled",
            "The run was cancelled. Complete sample chunks were retained; nothing partial was synthesized.",
        ),
        (_, ResultCompletenessClass::Loading) => state(
            "loading",
            "Sample pages are still arriving. Counts grow; stable row identities are preserved.",
        ),
        (_, ResultCompletenessClass::Partial) => state(
            "partial",
            "Retained scope is narrower than the declared inventory. Absent quantities are named, never invented.",
        ),
        (ResultCurrentness::Stale, _) => state(
            "stale",
            "The design changed after this run. These results stay open for review; actions that need current results are unavailable.",
        ),
        (ResultCurrentness::Superseded, _) => state(
            "superseded",
            "A later run covers this same analysis instance. The dataset itself is unchanged.",
        ),
        (ResultCurrentness::Unresolved, _) => state(
            "unresolved",
            "The relationship to the current source revision is unknown, so neither current nor stale may be claimed.",
        ),
        (ResultCurrentness::Recovered, _) => state(
            "recovered",
            "Reopened from a verified replica. Row identities were restored.",
        ),
        (ResultCurrentness::Current | ResultCurrentness::Partial, _) => state(
            "current",
            "Matches the current source revision, verified on load, complete against its declared inventory.",
        ),
    }
}

/// One way back when a narrowing has hidden everything.
///
/// A reader who filters to nothing has to be able to undo it without
/// remembering which control did it, so this resets every narrowing the
/// browser owns and names them all on its tooltip. Nothing is offered when the
/// emptiness is the dataset's own — there would be no filter to clear.
pub(super) fn results_browser_clear_filters(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    kind: ResultsBrowserKind,
    scope: ResultsBrowserScope,
) {
    let producer_filtering = ui.ctx().data(|data| {
        data.get_temp::<Option<AnalysisPresentationKey>>(results_browser_producer_id())
            .flatten()
            .is_some()
    });
    let filtering = !state.workbench.navigator_filter().trim().is_empty()
        || kind != ResultsBrowserKind::All
        || scope != ResultsBrowserScope::All
        || producer_filtering;
    if !filtering {
        return;
    }
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.add_space(PANEL_SEARCH_MARGIN_X);
        if ui
            .small_button("Clear filters")
            .on_hover_text("Reset the query, the analysis, the kind, and the scope")
            .clicked()
        {
            state.workbench.clear_navigator_filter();
            ui.ctx().data_mut(|data| {
                data.insert_temp(results_browser_kind_id(), ResultsBrowserKind::All);
                data.insert_temp(results_browser_scope_id(), ResultsBrowserScope::All);
                data.insert_temp::<Option<AnalysisPresentationKey>>(
                    results_browser_producer_id(),
                    None,
                );
            });
        }
    });
}

/// What the header's facets read this frame.
#[derive(Clone, Copy)]
struct BrowserFacets {
    kind: ResultsBrowserKind,
    layout: ResultsBrowserLayout,
    producer: Option<AnalysisPresentationKey>,
    order: ResultsBrowserOrder,
    scope: ResultsBrowserScope,
}

/// The data browser's header, on one grid: every control at the control
/// height, inside the dock's 8 px insets, with a 5 px gutter between the rows
/// and between the two columns the lower rows share.
///
/// The query fills the first row, with the view menu square at its end; the
/// producing analysis and the kind share the second, as the three facets of
/// the quantity list; the scope and the signal sets share the third. What a
/// filter holds back is counted at the query's end, inside the field. Given
/// no `signals` (Datasets and Expressions) the header keeps the query alone:
/// a manifest binding has no quantity kind to filter and no hierarchy to
/// order, and a saved expression is found by name.
fn results_browser_toolbar(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    facets: BrowserFacets,
    signals: Option<&TickedQuantities>,
    count: Option<&str>,
) {
    let t = Tokens::get(ui.ctx());
    let workspace = state.workbench.workspace;
    let side = t.metrics.ctl_h;
    let row_width = (ui.available_width() - 2.0 * PANEL_SEARCH_MARGIN_X).max(1.0);
    let column = ((row_width - BROWSER_GUTTER) / 2.0).max(1.0);
    let count_font = theme::mono(tokens::FS_MICRO, FontWeight::Medium);
    let count_room = count.map_or(0.0, |count| {
        ui.painter()
            .layout_no_wrap(count.to_owned(), count_font.clone(), t.color.text_faint)
            .size()
            .x
            + 6.0
    });
    ui.add_space(7.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(PANEL_SEARCH_MARGIN_X);
        let menu_room = if signals.is_some() {
            side + BROWSER_GUTTER
        } else {
            0.0
        };
        let field_width = (row_width - menu_room).max(1.0);
        let placeholder = "Find signal…";
        let field_id = ui.make_persistent_id("workbench.navigator.filter");
        if search_focus_requested::<FocusNavigatorSearch>(ui, &mut state.workbench.outbox) {
            ui.memory_mut(|memory| memory.request_focus(field_id));
        }
        let response = ui.add_sized(
            [field_width, side],
            egui::TextEdit::singleline(state.workbench.navigator_trees.filter_mut(workspace))
                .id(field_id)
                .hint_text(placeholder)
                .font(theme::field_sans())
                .margin(egui::Margin {
                    left: 29,
                    right: (8.0 + count_room) as i8,
                    top: 5,
                    bottom: 5,
                }),
        );
        const QUERY_HELP: &str = "Text finds a word anywhere. A pattern with * ? or [a-z] \
                                  matches whole hierarchical names, case-blind: x1.* or V(*out)";
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label(placeholder);
            node.set_description(QUERY_HELP);
        });
        let response = response.on_hover_text(QUERY_HELP);
        WorkbenchIcon::Search.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(response.rect.left() + 15.0, response.rect.center().y),
                egui::vec2(16.0, 16.0),
            ),
            t.color.text_faint,
        );
        if let Some(count) = count {
            // A label of its own, laid over the field's right margin, so
            // assistive tools read the count as the eye does.
            let room = egui::Rect::from_min_max(
                egui::pos2(
                    response.rect.right() - 8.0 - count_room,
                    response.rect.top(),
                ),
                egui::pos2(response.rect.right() - 8.0, response.rect.bottom()),
            );
            ui.put(
                room,
                egui::Label::new(
                    egui::RichText::new(count)
                        .font(count_font.clone())
                        .color(t.color.text_faint),
                )
                .selectable(false),
            );
        }
        if signals.is_some() {
            ui.add_space(BROWSER_GUTTER);
            results_browser_view_menu(ui, facets);
        }
    });
    if let Some(ticked) = signals {
        // Every analysis the session retained, under one leading "all"
        // entry that carries the count, so a reader can see how many
        // producers the tree spans without opening the list.
        let producers = state
            .model
            .simulation
            .retained
            .runs
            .iter()
            .flat_map(|run| {
                run.analyses.iter().map(move |analysis| {
                    (
                        AnalysisPresentationKey::new(run.dataset_id, analysis),
                        format!("{} · {}", run.name(), analysis.label),
                    )
                })
            })
            .collect::<Vec<_>>();
        let selected_producer = facets.producer.and_then(|selected| {
            producers
                .iter()
                .find(|(key, _)| *key == selected)
                .map(|(_, label)| label.clone())
        });
        // The count rides as the first entry's detail, which gives way
        // before its words do in a narrow dock.
        let mut producer_options = vec!["All analyses".to_owned()];
        producer_options.extend(producers.iter().map(|(_, label)| label.clone()));
        let mut producer_details = vec![producers.len().to_string()];
        producer_details.resize(producer_options.len(), String::new());
        let kind_options = ResultsBrowserKind::ALL
            .map(|value| value.label().to_owned())
            .to_vec();
        ui.add_space(BROWSER_GUTTER);
        ui.horizontal(|ui| {
            // Spacing is added explicitly: an `add_space` inside a horizontal
            // with item spacing pays for both, and the columns drift off the
            // query's inset.
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.add_space(PANEL_SEARCH_MARGIN_X);
            // The design system's select allocates exactly the width it is
            // given and shortens a value too long for it, naming it whole on
            // hover.
            if let Some(picked) = crate::ui::widgets::select_with_details(
                ui,
                "workbench.results.browser-producer",
                "Producing analysis",
                selected_producer.as_deref().unwrap_or(&producer_options[0]),
                &producer_options,
                &producer_details,
                column,
            )
            .picked
            {
                let key = picked
                    .checked_sub(1)
                    .and_then(|index| producers.get(index).map(|(key, _)| *key));
                ui.ctx()
                    .data_mut(|data| data.insert_temp(results_browser_producer_id(), key));
            }
            ui.add_space(BROWSER_GUTTER);
            results_browser_kind_control(ui, facets.kind, &kind_options, column);
        });
        ui.add_space(BROWSER_GUTTER);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.add_space(PANEL_SEARCH_MARGIN_X);
            // One scope is in force at a time, so the two read as one control
            // rather than two independent marks.
            const SCOPES: [ResultsBrowserScope; 2] =
                [ResultsBrowserScope::All, ResultsBrowserScope::Recent];
            let mut index = SCOPES
                .iter()
                .position(|candidate| *candidate == facets.scope)
                .unwrap_or(0);
            let changed = ui
                .allocate_ui_with_layout(
                    egui::vec2(column, side),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.set_width(column);
                        crate::ui::widgets::segmented(
                            ui,
                            "workbench.results.browser-scope",
                            &["All", "Recent"],
                            &mut index,
                            crate::ui::widgets::SegmentedWidth::Equal,
                        )
                    },
                )
                .inner;
            if changed {
                let scope_now = SCOPES[index];
                ui.ctx().data_mut(|data| {
                    data.insert_temp(results_browser_scope_id(), scope_now);
                });
            }
            ui.add_space(BROWSER_GUTTER);
            signal_sets_menu(ui, state, ticked, column);
        });
    }
    ui.add_space(7.0);
    // The header is one block, so it closes with a rule like the tab band
    // above it rather than bleeding into the rows below.
    ui.painter().hline(
        egui::Rangef::new(ui.max_rect().left(), ui.max_rect().right()),
        ui.cursor().top() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
}

/// Open `content` as a menu from `button`, a button the kit drew: a click on
/// it opens or closes the menu, and the menu closes as `close_behavior` says.
pub(super) fn button_menu<R>(
    button: &Response,
    close_behavior: egui::PopupCloseBehavior,
    content: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    use egui::containers::menu::MenuConfig;
    let config = MenuConfig::new().close_behavior(close_behavior);
    egui::Popup::menu(button)
        .close_behavior(close_behavior)
        .style(config.style.clone())
        .info(
            egui::UiStackInfo::new(egui::UiKind::Menu)
                .with_tag_value(MenuConfig::MENU_CONFIG_TAG, config),
        )
        .show(content)
        .map(|shown| shown.inner)
}

/// The active run's event-net and bus rows whose lanes are on the plot.
pub(super) fn plotted_event_rows(state: &ResultsParts<'_>) -> Vec<ResultBrowserSelectionKey> {
    let Some(run) = state.model.simulation.active_run() else {
        return Vec::new();
    };
    run.analyses
        .iter()
        .filter(|analysis| {
            matches!(
                analysis.result_payload,
                Some(crate::state::AnalysisResultPayload::TransientEvents { .. })
            )
        })
        .flat_map(|analysis| {
            retained_result_artifacts(
                analysis,
                AnalysisPresentationKey::new(run.dataset_id, analysis),
            )
        })
        .filter(|artifact| is_plottable_artifact(artifact.identity.canonical_name()))
        .map(|artifact| ResultBrowserSelectionKey::Artifact(artifact.identity))
        .filter(|key| result_quantity_ticked(state, key))
        .collect()
}

/// The kind list, closing with the switch that shows internal nodes: as in
/// SimVision's and EZwave's browsers, the nodes the simulator generated for
/// its devices stay out of the tree until the reader asks for them.
fn results_browser_kind_control(
    ui: &mut Ui,
    kind: ResultsBrowserKind,
    options: &[String],
    width: f32,
) {
    let show_internal = results_browser_shows_internal(ui.ctx());
    let (picked, switched) = crate::ui::widgets::select_with_switch(
        ui,
        "workbench.results.browser-kind",
        "Quantity kind",
        kind.label(),
        options,
        crate::ui::widgets::SelectSwitch {
            label: "Show internal nodes",
            on: show_internal,
        },
        width,
    );
    if let Some(picked) = picked {
        let kind_now = ResultsBrowserKind::ALL[picked];
        ui.ctx()
            .data_mut(|data| data.insert_temp(results_browser_kind_id(), kind_now));
    }
    if switched {
        ui.ctx().data_mut(|data| {
            data.insert_temp(results_browser_internal_id(), !show_internal);
        });
    }
}

fn results_browser_kind_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-kind-facet")
}

fn results_browser_internal_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-internal-nodes")
}

/// Whether the browser lists the nodes the simulator generated. They are
/// hidden until the reader turns them on.
pub(super) fn results_browser_shows_internal(ctx: &egui::Context) -> bool {
    ctx.data(|data| data.get_temp::<bool>(results_browser_internal_id()))
        .unwrap_or(false)
}

fn results_browser_layout_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-layout")
}

fn results_browser_scope_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-scope")
}

fn results_browser_producer_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-producer-facet")
}

fn results_browser_order_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-order")
}

/// The view menu, square at the query's end: the order the signals read in
/// and whether they read as a tree or as a flat list. Recent lists what was
/// used most recently first, so the orders wait while it is in force.
fn results_browser_view_menu(ui: &mut Ui, facets: BrowserFacets) {
    let t = Tokens::get(ui.ctx());
    let side = t.metrics.ctl_h;
    let recent = facets.scope == ResultsBrowserScope::Recent;
    const RECENT: &str = "Recent lists what was used most recently first.";
    let heading = |ui: &mut Ui, text: &str| {
        ui.label(
            egui::RichText::new(text)
                .font(theme::sans(tokens::FS_MICRO, FontWeight::Medium))
                .color(t.color.text_faint),
        );
    };
    let response = ui
        .scope(|ui| {
            ui.spacing_mut().button_padding = egui::Vec2::ZERO;
            egui::containers::menu::MenuButton::from_button(
                egui::Button::new("").min_size(egui::vec2(side, side)),
            )
            .ui(ui, |ui| {
                heading(ui, "Sort by");
                ui.add_enabled_ui(!recent, |ui| {
                    for candidate in ResultsBrowserOrder::ALL {
                        let row = ui.radio(candidate == facets.order, candidate.label());
                        if row.clicked() {
                            ui.ctx().data_mut(|data| {
                                data.insert_temp(results_browser_order_id(), candidate);
                            });
                            ui.close();
                        }
                        row.on_disabled_hover_text(RECENT);
                    }
                });
                ui.separator();
                heading(ui, "Show as");
                for candidate in ResultsBrowserLayout::ALL {
                    if ui
                        .radio(candidate == facets.layout, candidate.label())
                        .clicked()
                    {
                        ui.ctx().data_mut(|data| {
                            data.insert_temp(results_browser_layout_id(), candidate);
                        });
                        ui.close();
                    }
                }
            })
            .0
        })
        .inner;
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), "View options")
    });
    WorkbenchIcon::Sliders.paint(
        ui.painter(),
        egui::Rect::from_center_size(response.rect.center(), egui::vec2(16.0, 16.0)),
        t.color.text_dim,
    );
    let order = if recent {
        "Most recently used first"
    } else {
        facets.order.stated()
    };
    let shown = match facets.layout {
        ResultsBrowserLayout::Tree => "as a tree",
        ResultsBrowserLayout::Flat => "as a flat list",
    };
    response.on_hover_text(format!("{order}, {shown}"));
}

fn results_browser_active_tab(ctx: &egui::Context) -> ResultsBrowserTab {
    ctx.data(|data| {
        data.get_temp::<ResultsBrowserTab>(egui::Id::new("workbench.results.browser-tab"))
    })
    .unwrap_or_default()
}

pub fn result_browser_dataset_expanded(
    ctx: &egui::Context,
    dataset: DatasetId,
    default_open: bool,
) -> bool {
    ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new((
            "workbench.results.browser-dataset",
            dataset,
        )))
    })
    .unwrap_or(default_open)
}

pub fn set_result_browser_dataset_expanded(
    ctx: &egui::Context,
    dataset: DatasetId,
    expanded: bool,
) {
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(("workbench.results.browser-dataset", dataset)),
            expanded,
        );
    });
}

/// Disclosure state for one analysis group, defaulting to open for the
/// active analysis and closed for the rest: a run with a hundred retained
/// quantities should open as an index of its analyses, not one long list.
pub fn result_browser_group_expanded(
    ctx: &egui::Context,
    analysis: AnalysisPresentationKey,
    active: bool,
) -> bool {
    ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new(("workbench.results.browser-group", analysis)))
    })
    .unwrap_or(active)
}

pub fn set_result_browser_group_expanded(
    ctx: &egui::Context,
    analysis: AnalysisPresentationKey,
    expanded: bool,
) {
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(("workbench.results.browser-group", analysis)),
            expanded,
        );
    });
}

/// Whether an instance of the tree is open. Instances fold by default, so an
/// analysis opens as its top-level signals and the instances under them.
pub(super) fn result_browser_instance_expanded(
    ctx: &egui::Context,
    analysis: AnalysisPresentationKey,
    instance: &str,
) -> bool {
    ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new((
            "workbench.results.browser-instance",
            analysis,
            instance,
        )))
    })
    .unwrap_or(false)
}

pub(super) fn set_result_browser_instance_expanded(
    ctx: &egui::Context,
    analysis: AnalysisPresentationKey,
    instance: &str,
    expanded: bool,
) {
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(("workbench.results.browser-instance", analysis, instance)),
            expanded,
        );
    });
}

/// The mockup's data-browser tab band: three equal columns on one 30 px
/// row, label and mono count sharing the cell, a 2 px accent baseline on
/// the active tab. Returns the tab to render this frame.
fn results_browser_tab_band(ui: &mut Ui, counts: [usize; 3]) -> ResultsBrowserTab {
    let t = Tokens::get(ui.ctx());
    let band_id = egui::Id::new("workbench.results.browser-tab");
    let mut tab = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserTab>(band_id))
        .unwrap_or_default();
    let tabs = [
        (ResultsBrowserTab::Signals, "Signals", counts[0]),
        (ResultsBrowserTab::Datasets, "Datasets", counts[1]),
        (ResultsBrowserTab::Expressions, "Expressions", counts[2]),
    ];
    let (band, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 30.0), egui::Sense::hover());
    let painter = ui.painter_at(band);
    let column_width = band.width() / tabs.len() as f32;
    for (index, (candidate, label, count)) in tabs.into_iter().enumerate() {
        let cell = egui::Rect::from_min_max(
            egui::pos2(band.left() + column_width * index as f32, band.top()),
            egui::pos2(
                band.left() + column_width * (index + 1) as f32,
                band.bottom(),
            ),
        );
        let response = ui
            .interact(
                cell,
                ui.id().with(("results-browser-tab", index)),
                egui::Sense::click(),
            )
            .on_hover_text(format!("Show {label}"));
        let selected = tab == candidate;
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                selected,
                format!("{label}, {count}"),
            )
        });
        if response.clicked() {
            tab = candidate;
            ui.ctx().data_mut(|data| data.insert_temp(band_id, tab));
        }
        if response.hovered() && !selected {
            painter.rect_filled(cell, 0.0, t.color.bg_hover);
        }
        let label_color = if selected || response.hovered() {
            t.color.text
        } else {
            t.color.text_dim
        };
        let count_color = if selected {
            t.color.accent
        } else {
            t.color.text_faint
        };
        let label_galley = painter.layout_no_wrap(
            label.to_owned(),
            theme::sans(tokens::FS_MICRO, FontWeight::SemiBold),
            label_color,
        );
        let count_galley = painter.layout_no_wrap(
            count.to_string(),
            theme::mono(tokens::FS_MICRO, FontWeight::Medium),
            count_color,
        );
        let total = label_galley.rect.width() + 5.0 + count_galley.rect.width();
        let start_x = cell.center().x - total / 2.0;
        let label_pos = egui::pos2(start_x, cell.center().y - label_galley.rect.height() / 2.0);
        let count_pos = egui::pos2(
            start_x + label_galley.rect.width() + 5.0,
            cell.center().y - count_galley.rect.height() / 2.0,
        );
        painter.galley(label_pos, label_galley, label_color);
        painter.galley(count_pos, count_galley, count_color);
        if selected {
            painter.rect_filled(
                egui::Rect::from_min_max(egui::pos2(cell.left(), cell.bottom() - 2.0), cell.max),
                0.0,
                t.color.accent,
            );
        }
        theme::paint_focus_ring(ui, &response, cell);
    }
    painter.hline(
        band.x_range(),
        band.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    tab
}

pub(super) struct ResultRun {
    pub(super) run_index: usize,
    pub(super) dataset_id: DatasetId,
    pub(super) label: String,
    /// The run's name, its label without the start it carries.
    pub(super) name: String,
    pub(super) success: bool,
    /// The binding's own merged reading. A dataset row is where a condition
    /// that holds for every analysis under it is stated, once.
    pub(super) data_state: ResultAnalysisDataState,
    pub(super) analyses: Vec<ResultAnalysis>,
}

struct ResultDatasetRowResponses {
    selection: Response,
    overlay: Option<Response>,
}

/// Locate results: the folder the reader picks is searched for the results store of every run
/// that cannot read its samples.
fn locate_results(state: &mut ResultsParts<'_>) {
    let Some(folder) = pick_results_folder() else {
        return;
    };
    #[cfg(not(target_arch = "wasm32"))]
    {
        let located =
            crate::io::result_segments::locate_results(&mut state.model.simulation, &folder);
        state.workbench.prompt(match located {
            0 => format!("{} holds no results store of these runs.", folder.display()),
            1 => "The results of 1 run were found.".to_owned(),
            runs => format!("The results of {runs} runs were found."),
        });
    }
    #[cfg(target_arch = "wasm32")]
    let _ = (state, folder);
}

/// The folder Locate results searches: none in a test, which never opens a dialog, nor in the
/// browser, which has no folders.
#[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
fn pick_results_folder() -> Option<std::path::PathBuf> {
    None
}

#[cfg(not(any(test, feature = "test-support", target_arch = "wasm32")))]
fn pick_results_folder() -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .set_title("Locate results")
        .pick_folder()
}

fn result_dataset_row(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: &str,
    overlaid: bool,
) -> ResultDatasetRowResponses {
    let t = Tokens::get(ui.ctx());
    let height = t.metrics.row_h;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    let overlay_width = if selected { 0.0 } else { 30.0 };
    let selection_rect = egui::Rect::from_min_max(
        rect.left_top(),
        egui::pos2(rect.right() - overlay_width, rect.bottom()),
    );
    let selection = ui.interact(
        selection_rect,
        ui.id().with(("result-dataset-selection", label)),
        egui::Sense::click(),
    );
    selection.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        )
    });
    let overlay = (!selected).then(|| {
        let overlay_rect = egui::Rect::from_min_max(
            egui::pos2(selection_rect.right(), rect.top()),
            rect.right_bottom(),
        );
        let response = ui.interact(
            overlay_rect,
            ui.id().with(("result-dataset-overlay", label)),
            egui::Sense::click(),
        );
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Button,
                ui.is_enabled(),
                overlaid,
                format!("Overlay {label}"),
            )
        });
        let dot = egui::Rect::from_center_size(overlay_rect.center(), egui::vec2(10.0, 10.0));
        ui.painter().circle(
            dot.center(),
            4.5,
            if overlaid {
                t.color.accent
            } else {
                egui::Color32::TRANSPARENT
            },
            egui::Stroke::new(
                1.0,
                if response.hovered() || overlaid {
                    t.color.accent
                } else {
                    t.color.text_faint
                },
            ),
        );
        theme::paint_focus_ring(ui, &response, overlay_rect);
        response
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .on_hover_text(if overlaid {
                format!("Remove {label} from the active result sheet")
            } else {
                format!("Overlay {label} on the active result sheet")
            })
    });
    if selected || selection.hovered() {
        ui.painter().rect_filled(
            selection_rect,
            0.0,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }
    let caret_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 12.0, rect.center().y),
        egui::vec2(9.0, 9.0),
    );
    if selected {
        WorkbenchIcon::ChevronDown.paint(ui.painter(), caret_rect, t.color.text_dim);
    } else {
        let stroke = egui::Stroke::new(1.0, t.color.text_faint);
        ui.painter().line_segment(
            [
                egui::pos2(caret_rect.left() + 2.5, caret_rect.top() + 1.5),
                caret_rect.center(),
            ],
            stroke,
        );
        ui.painter().line_segment(
            [
                caret_rect.center(),
                egui::pos2(caret_rect.left() + 2.5, caret_rect.bottom() - 1.5),
            ],
            stroke,
        );
    }
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 31.0, rect.center().y),
            egui::vec2(15.0, 15.0),
        ),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    let meta_width = ui
        .painter()
        .layout_no_wrap(
            meta.to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        )
        .size()
        .x;
    let label_left = rect.left() + 45.0;
    let label_right = selection_rect.right() - 14.0 - meta_width;
    ui.painter()
        .with_clip_rect(egui::Rect::from_x_y_ranges(
            label_left..=label_right.max(label_left),
            rect.y_range(),
        ))
        .text(
            egui::pos2(label_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if selected {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
    ui.painter().text(
        egui::pos2(selection_rect.right() - 8.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        meta,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    theme::paint_focus_ring(ui, &selection, selection_rect);
    ResultDatasetRowResponses { selection, overlay }
}

pub(super) struct ResultAnalysis {
    pub(super) analysis_index: usize,
    pub(super) presentation_key:
        crate::workbench::documents::result_document::AnalysisPresentationKey,
    pub(super) label: String,
    pub(super) short_label: &'static str,
    /// Caption under the label: the domain this analysis swept. Repeating
    /// the analysis name here would state the head's own title twice.
    pub(super) domain: String,
    /// Quantities the analysis retained, before the browser's filters. The
    /// head shows `shown / retained` whenever the two differ.
    pub(super) total_signals: usize,
    /// The one merged reading of this producer's retained evidence. The head
    /// paints no word for it; it explains the retained fraction on the tooltip.
    pub(super) data_state: ResultAnalysisDataState,
    pub(super) success: bool,
    /// The signals the filters let through, with the instance tree over them
    /// and `artifacts` in the tree layout.
    pub(super) signals: ListedSignals,
    pub(super) artifacts: Vec<ResultArtifact>,
}

/// A listed signal's row, made when the row is drawn.
pub(super) struct ResultSignal {
    pub(super) waveform_index: usize,
    pub(super) identity: SourceWaveformPresentationKey,
    pub(super) name: String,
    pub(super) color: String,
    pub(super) value: Option<String>,
    /// The row's typed metadata, empty when there is nothing to say.
    pub(super) meta: String,
    pub(super) kind: SignalKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultArtifactKind {
    Scalar,
    Array,
    EventStream,
    Contribution,
}

impl ResultArtifactKind {
    /// The mark an artifact row wears when it is not a net or a bus.
    pub(super) const fn glyph(self) -> &'static str {
        match self {
            Self::Scalar => "#",
            Self::Array => "[]",
            Self::EventStream => ">",
            Self::Contribution => "S",
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Scalar => "Scalar",
            Self::Array => "Array",
            Self::EventStream => "Event stream",
            Self::Contribution => "Contribution",
        }
    }
}

pub struct ResultArtifact {
    pub identity: ResultArtifactPresentationKey,
    pub name: String,
    pub kind: ResultArtifactKind,
    pub(super) unit: String,
    pub meta: String,
    pub value: Option<String>,
    pub viewer: crate::workbench::ResultViewer,
}

#[derive(Clone, Copy)]
pub(super) enum ResultSignalVirtualRow {
    Dataset {
        run: usize,
        expanded: bool,
    },
    DatasetOmission {
        run: usize,
    },
    Analysis {
        run: usize,
        analysis: usize,
        expanded: bool,
    },
    Omission {
        run: usize,
        analysis: usize,
    },
    /// An instance of the design, by its index in the frame's instance
    /// list; it folds what it owns.
    Instance {
        instance: usize,
        depth: usize,
        expanded: bool,
    },
    /// A signal, `depth` instances down the tree; zero in the flat layout
    /// and at the top of the design.
    Signal {
        run: usize,
        analysis: usize,
        signal: usize,
        depth: usize,
    },
    Artifact {
        run: usize,
        analysis: usize,
        artifact: usize,
        depth: usize,
    },
}

/// The order the signals tab lists each level's quantities in: by name, by
/// unit, in the order the run wrote them, or by sample count, as ViVA's and
/// EZwave's browsers sort. Numbers in names count (`L2` before `L10`), and
/// a tie reads by name.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub(super) enum ResultsBrowserOrder {
    #[default]
    Name,
    Unit,
    RunOrder,
    Samples,
}

impl ResultsBrowserOrder {
    pub(super) const ALL: [Self; 4] = [Self::Name, Self::Unit, Self::RunOrder, Self::Samples];

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Name => "Name",
            Self::Unit => "Unit",
            Self::RunOrder => "Run order",
            Self::Samples => "Samples",
        }
    }

    /// The order said in full, for the menu's tooltip.
    const fn stated(self) -> &'static str {
        match self {
            Self::Name => "Sorted by name",
            Self::Unit => "Sorted by unit, then by name",
            Self::RunOrder => "In the order the run wrote them",
            Self::Samples => "Most samples first, then by name",
        }
    }

    /// How two signals of a flat list read in this order. The run's order
    /// is a stable sort's: every pair ties.
    pub(super) fn compare(self, left: Quantity<'_>, right: Quantity<'_>) -> std::cmp::Ordering {
        let by_name = || natural_cmp(left.name, right.name);
        match self {
            Self::Name => by_name(),
            Self::Unit => natural_cmp(left.unit, right.unit).then_with(by_name),
            Self::Samples => right.samples.cmp(&left.samples).then_with(by_name),
            Self::RunOrder => std::cmp::Ordering::Equal,
        }
    }

    /// Put a flat list's other results in this order. They hold no samples
    /// to count, so the sample order reads them by name.
    pub(super) fn sort_artifacts(self, artifacts: &mut [ResultArtifact]) {
        let by_name =
            |left: &ResultArtifact, right: &ResultArtifact| natural_cmp(&left.name, &right.name);
        match self {
            Self::Name | Self::Samples => artifacts.sort_by(by_name),
            Self::Unit => artifacts.sort_by(|left, right| {
                natural_cmp(&left.unit, &right.unit).then_with(|| by_name(left, right))
            }),
            Self::RunOrder => {}
        }
    }

    /// The groups a tree level reads in under this order, before names:
    /// kinds, units, or sample counts.
    /// `signals` are each listed signal's unit and samples, `artifact_units`
    /// each listed artifact's unit.
    pub(super) fn groups<'a>(
        self,
        signals: impl IntoIterator<Item = (&'a str, usize)>,
        artifact_units: impl IntoIterator<Item = &'a str>,
    ) -> OrderGroups {
        let mut units: Vec<String> = Vec::new();
        let mut counts: Vec<usize> = Vec::new();
        match self {
            Self::Unit => {
                let mut seen = std::collections::HashSet::new();
                units = signals
                    .into_iter()
                    .map(|(unit, _)| unit)
                    .chain(artifact_units)
                    .filter(|unit| seen.insert(*unit))
                    .map(str::to_owned)
                    .collect();
                units.sort_by(|left, right| natural_cmp(left, right));
            }
            Self::Samples => {
                let mut seen = std::collections::HashSet::new();
                counts = signals
                    .into_iter()
                    .map(|(_, samples)| samples)
                    .filter(|samples| seen.insert(*samples))
                    .collect();
                counts.sort_unstable_by(|left, right| right.cmp(left));
            }
            Self::Name | Self::RunOrder => {}
        }
        OrderGroups {
            order: self,
            units,
            counts,
        }
    }
}

/// Where each quantity of a tree level falls under a [`ResultsBrowserOrder`]:
/// the rank its group reads at.
pub(super) struct OrderGroups {
    order: ResultsBrowserOrder,
    units: Vec<String>,
    counts: Vec<usize>,
}

impl OrderGroups {
    fn rank(position: Option<usize>) -> u8 {
        position.map_or(u8::MAX, |position| {
            u8::try_from(position).unwrap_or(u8::MAX - 1)
        })
    }

    /// The group of a signal reading in `unit`, holding `samples`, of `kind`.
    pub(super) fn of_signal(&self, unit: &str, samples: usize, kind: SignalKind) -> u8 {
        match self.order {
            ResultsBrowserOrder::Unit => {
                Self::rank(self.units.iter().position(|held| held == unit))
            }
            ResultsBrowserOrder::Samples => {
                Self::rank(self.counts.iter().position(|count| *count == samples))
            }
            ResultsBrowserOrder::Name | ResultsBrowserOrder::RunOrder => kind.rank(),
        }
    }

    /// The group of an artifact reading in `unit` whose kind ranks at
    /// `kind_rank`.
    pub(super) fn of_artifact(&self, unit: &str, kind_rank: u8) -> u8 {
        match self.order {
            ResultsBrowserOrder::Unit => {
                Self::rank(self.units.iter().position(|held| held == unit))
            }
            // No samples to count: after the signals, by name.
            ResultsBrowserOrder::Samples => u8::MAX,
            ResultsBrowserOrder::Name | ResultsBrowserOrder::RunOrder => kind_rank,
        }
    }
}
