//! The Results workspace's data browser: its tab band, toolbar and facets,
//! the dataset rows, and the row model the quantity list is built from.

use egui::{Response, ScrollArea, Ui};

use crate::product::DatasetId;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, analysis_default_unit, browser_signal_unit, retained_evidence_is_valid,
};
use crate::workbench::framework::requests::{ExportResultQuantities, FocusNavigatorSearch};
use crate::workbench::results::commands::ResultsCommand;
use rspice_results::result_presentation::{
    ResultArtifactPresentationKey, SourceWaveformPresentationKey,
};
use rspice_results_ui::selection::{ResultBrowserSelectionKey, ResultExpressionPresentationKey};

use super::super::super::commands::result_navigation::open_run_log;
use super::super::super::design_system::WorkbenchIcon;
use super::results_actions::{
    locate_on_schematic_menu, select_result_analysis_by_key, select_result_browser_key,
    select_result_dataset, set_checked_signal_visibility,
};
use super::results_rows::{
    ResultCompletenessClass, expression_header, result_analysis_browser_status,
    result_analysis_completeness, result_analysis_domain, result_browser_manifest_row,
    result_quantity_kind_label, result_run_completeness, result_run_operational_label,
    retained_result_artifacts, show_virtualized_result_signals, signal_row,
};
use super::{PANEL_SEARCH_MARGIN_X, muted, panel_search_field_width, search_focus_requested};

/// The content width the pair still reads at: the navigator dock's own minimum
/// of 220 px less its 8 px insets. Above it the typed facets stay side by side
/// as the mockup draws them, rather than stacking into a column of selects at
/// every ordinary dock width. Below it — the tablet and phone drawers — they
/// stack, so a right-to-left row can never enlarge beyond its host and resize
/// stays monotonic.
pub(super) const RESULT_BROWSER_STACKED_FACET_MAX_WIDTH: f32 = 204.0;
/// Clipboard generation is synchronous on every supported target. Keep the
/// operation bounded and direct larger evidence sets to the streaming export
/// workflow instead of allocating an unbounded browser/WASM string.
pub(super) const RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT: usize = 100_000;
const RESULT_BROWSER_CLIPBOARD_BYTE_LIMIT: usize = 8_000_000;
/// `.result-browser-status { padding: 3px 8px }` — the padding that makes the
/// scope control a band rather than a bare row. Without it the segments are
/// exactly the band, so they sit hard against the toolbar rule above them.
const RESULT_BROWSER_BAND_PAD_Y: f32 = 3.0;

pub(super) fn result_browser_facets_stack(available_width: f32) -> bool {
    available_width < RESULT_BROWSER_STACKED_FACET_MAX_WIDTH
}

pub(in crate::workbench) fn results(ui: &mut Ui, app: &mut RSpiceApp) {
    if !app
        .state
        .results
        .results
        .session
        .checked_result_quantities
        .is_empty()
        && !egui::Popup::is_any_open(ui.ctx())
        && ui.input(|input| input.key_pressed(egui::Key::Escape))
    {
        app.state.results.results.session.clear_checked_signals();
    }
    let query = app.state.workbench.navigator_filter().trim().to_lowercase();
    let active_browser_tab = results_browser_active_tab(ui.ctx());
    // Kind/sort read the values the toolbar stored last frame — the row
    // renders below the tab band while filtering happens here, the same
    // one-frame contract the viewer-tab chevrons use.
    let stored_kind = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserKind>(results_browser_kind_id()))
        .unwrap_or_default();
    let sort = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserSort>(results_browser_sort_id()))
        .unwrap_or_default();
    let stored_scope = ui
        .ctx()
        .data(|data| data.get_temp::<ResultsBrowserScope>(results_browser_scope_id()))
        .unwrap_or_default();
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
    let producer_facet = if active_browser_tab == ResultsBrowserTab::Signals {
        stored_producer_facet
    } else {
        None
    };
    // A snapshot keeps the favorites predicate borrow-free inside the run map;
    // recency reads its owning accessor, which states the rank once.
    let favorite_signals = app.state.results.results.session.favorite_signals.clone();
    let active_run = app.state.model.simulation.view.active_run_idx.or_else(|| {
        app.state
            .model
            .simulation
            .retained
            .runs
            .len()
            .checked_sub(1)
    });
    let active_analysis = app
        .state
        .model
        .simulation
        .view
        .active_analysis_idx
        .or_else(|| {
            active_run.and_then(|run| {
                app.state
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
        .and_then(|run| app.state.model.simulation.retained.runs.get(run))
        .zip(active_analysis)
        .and_then(|(run, analysis_index)| {
            run.analyses
                .get(analysis_index)
                .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
        });
    let selected_trace = app
        .state
        .results
        .results
        .valid_selected_trace(&app.state.model.simulation)
        .cloned();
    let selected_artifact = app
        .state
        .results
        .results
        .session
        .selected_result_artifact
        .clone()
        .filter(|key| {
            key.resolve(&app.state.model.simulation.retained.runs)
                .is_some()
        });
    let expression_source = active_analysis_key.map_or_else(Vec::new, |analysis| {
        app.state
            .results
            .results
            .expression_entries_for_analysis(&app.state.model.simulation, analysis)
    });
    let expressions = expression_source
        .iter()
        .enumerate()
        .filter(|(_, (_, expression))| {
            query.is_empty() || expression.text.to_lowercase().contains(&query)
        })
        .map(|(expression_index, (identity, expression))| {
            (identity.clone(), expression_index, expression.clone())
        })
        .collect::<Vec<_>>();
    let expression_query_match = !query.is_empty() && !expressions.is_empty();
    let analysis_keys = app
        .state
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
            let valid = retained_evidence_is_valid(app.state.results_reads(), key);
            (key, valid)
        })
        .collect::<std::collections::HashMap<_, _>>();
    let runs = app
        .state
        .model.simulation
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
                        &app.state,
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
                        || run.label.to_lowercase().contains(&query)
                        || analysis.label.to_lowercase().contains(&query)
                        || analysis
                            .analysis_type
                            .display_name()
                            .to_lowercase()
                            .contains(&query)
                        || analysis
                            .analysis_type
                            .short_label()
                            .to_lowercase()
                            .contains(&query)
                        || analysis.id.to_string().contains(&query)
                        || analysis.provenance().is_some_and(|provenance| {
                            provenance
                                .source_instance_id()
                                .to_string()
                                .to_lowercase()
                                .contains(&query)
                        })
                        || integrity.contains(&query)
                        || completeness.contains(&query)
                        || currentness_label.contains(&query);
                    let mut signals = analysis
                        .waveforms
                        .iter()
                        .enumerate()
                        .filter(|(_, waveform)| {
                            // The unit the signal actually reads in decides
                            // its kind. Treating "not a current" as a voltage
                            // filed noise densities and decibel magnitudes
                            // under Voltage.
                            let unit = browser_signal_unit(
                                &waveform.name,
                                waveform.unit.as_deref(),
                                analysis_default_unit(analysis.analysis_type),
                            );
                            kind.admits(unit)
                        })
                        .filter_map(|(waveform_index, waveform)| {
                            let unit = browser_signal_unit(
                                &waveform.name,
                                waveform.unit.as_deref(),
                                analysis.analysis_type.axis_info().3,
                            );
                            let quantity_kind =
                                result_quantity_kind_label(&waveform.name, unit);
                            if !analysis_query_matches
                                && !waveform.name.to_lowercase().contains(&query)
                                && !unit.to_lowercase().contains(&query)
                                && !quantity_kind.to_lowercase().contains(&query)
                                && !"vector".contains(&query)
                            {
                                return None;
                            }
                            let identity = SourceWaveformPresentationKey::new(
                                presentation_key,
                                waveform.name.clone(),
                            );
                            let in_scope = match scope {
                                ResultsBrowserScope::All => true,
                                ResultsBrowserScope::Favorites => {
                                    favorite_signals.contains(&identity)
                                }
                                ResultsBrowserScope::Recent => app
                                    .state
                                    .results
                                    .results
                                    .session.recent_signal_rank(&identity)
                                    .is_some(),
                            };
                            if !in_scope {
                                return None;
                            }
                            let samples = waveform.y.len();
                            Some(ResultSignal {
                                waveform_index,
                                visible: app
                                    .state
                                    .results
                                    .results
                                    .session.waveform_visibility(&identity, waveform.visible),
                                identity,
                                name: waveform.name.clone(),
                                color: waveform.color.clone(),
                                // Nothing: every row of a waveform list is a
                                // vector, the unit rides the value, and the
                                // sample count is not something a reader acts
                                // on from here.
                                meta: String::new(),
                                // A swept quantity has no one value: the last
                                // sample is the end of the sweep, and read
                                // against the row's name it passes for "the
                                // value of V(out)". A solved scalar does have
                                // one, and states it.
                                value: (samples == 1)
                                    .then(|| waveform.y.iter().copied().find(|v| v.is_finite()))
                                    .flatten()
                                    .map(|value| {
                                        // Degrees carry no SI prefix; a
                                        // near-zero phase as "f°" misreads.
                                        if unit == "°" {
                                            format!("{value:.3} °")
                                        } else {
                                            crate::ui::plot::fmt_si(value, unit, 3)
                                        }
                                    }),
                            })
                        })
                        .collect::<Vec<_>>();
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
                                || artifact.name.to_lowercase().contains(&query)
                                || artifact
                                    .identity
                                    .canonical_name()
                                    .to_lowercase()
                                    .contains(&query)
                                || artifact.kind.label().to_lowercase().contains(&query)
                                || artifact.meta.to_lowercase().contains(&query)
                        })
                        .filter(|artifact| match scope {
                            ResultsBrowserScope::All => true,
                            ResultsBrowserScope::Favorites => app
                                .state
                                .results
                                .results
                                .session.is_favorite_result_artifact(&artifact.identity),
                            ResultsBrowserScope::Recent => app
                                .state
                                .results
                                .results
                                .session.recent_result_artifact_rank(&artifact.identity)
                                .is_some(),
                        })
                        .collect::<Vec<_>>();
                    if scope == ResultsBrowserScope::Recent {
                        // Recent means recency order; the sort facet yields.
                        signals.sort_by_key(|signal| {
                            app.state
                                .results
                                .results
                                .session.recent_signal_rank(&signal.identity)
                                .unwrap_or(usize::MAX)
                        });
                        artifacts.sort_by_key(|artifact| {
                            app.state
                                .results
                                .results
                                .session.recent_result_artifact_rank(&artifact.identity)
                                .unwrap_or(usize::MAX)
                        });
                    } else if sort == ResultsBrowserSort::Name {
                        signals.sort_by(|left, right| {
                            left.name
                                .to_ascii_lowercase()
                                .cmp(&right.name.to_ascii_lowercase())
                                .then_with(|| left.name.cmp(&right.name))
                        });
                        artifacts.sort_by(|left, right| {
                            left.name
                                .to_ascii_lowercase()
                                .cmp(&right.name.to_ascii_lowercase())
                                .then_with(|| left.name.cmp(&right.name))
                        });
                    }
                    let matches_analysis =
                        analysis_query_matches || !signals.is_empty() || !artifacts.is_empty();
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
                        total_signals: analysis.waveforms.len() + total_artifacts,
                        data_state: result_analysis_data_state(
                            source_currentness,
                            completeness_class,
                        ),
                        analysis_index,
                        presentation_key,
                        label,
                        short_label: analysis.analysis_type.short_label(),
                        success: analysis.success,
                        signals,
                        artifacts,
                    })
                })
                .collect::<Vec<_>>();
            let run_currentness = crate::workbench::documents::result_document::operational_state::run_currentness(
                &app.state,
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
                || run.label.to_lowercase().contains(&query)
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
                || run_integrity.contains(&query)
                || run_state.contains(&query)
                || run_currentness_label.contains(&query)
                || run.dataset_id.to_string().to_lowercase().contains(&query);
            matches_run.then(|| ResultRun {
                run_index,
                dataset_id: run.dataset_id,
                label: run.label.clone(),
                success: run.success,
                data_state: result_analysis_data_state(
                    run_currentness,
                    result_run_completeness(run),
                ),
                analyses,
            })
        })
        .collect::<Vec<_>>();

    let signal_count = runs
        .iter()
        .flat_map(|run| run.analyses.iter())
        .map(|analysis| analysis.signals.len() + analysis.artifacts.len())
        .sum::<usize>();
    let visible_result_keys =
        runs.iter()
            .filter(|run| {
                !query.is_empty()
                    || result_browser_dataset_expanded(
                        ui.ctx(),
                        run.dataset_id,
                        active_run == Some(run.run_index) || runs.len() == 1,
                    )
            })
            .flat_map(|run| {
                run.analyses.iter().filter(|analysis| {
                    !query.is_empty()
                        || result_browser_group_expanded(
                            ui.ctx(),
                            analysis.presentation_key,
                            active_run == Some(run.run_index)
                                && active_analysis == Some(analysis.analysis_index),
                        )
                })
            })
            .flat_map(|analysis| {
                analysis
                    .signals
                    .iter()
                    .map(|signal| ResultBrowserSelectionKey::Waveform(signal.identity.clone()))
                    .chain(analysis.artifacts.iter().map(|artifact| {
                        ResultBrowserSelectionKey::Artifact(artifact.identity.clone())
                    }))
            })
            .collect::<Vec<_>>();
    let tab = results_browser_tab_band(ui, [signal_count, runs.len(), expressions.len()]);
    // The mockup's browser toolbar: the query over this tab, then the analysis,
    // kind, and sort facets. The facets are absent on Datasets and Expressions —
    // a manifest binding has no quantity kind to filter and no hierarchy to
    // order, and a saved expression is found by name.
    results_browser_toolbar(
        ui,
        app,
        stored_kind,
        sort,
        stored_producer_facet,
        tab == ResultsBrowserTab::Signals,
    );
    // The scope band: the signals tab owns the scope control on the left —
    // All | Favorites | Recent over the session's real star and recency
    // state — and the count on the right states how much of the tab's
    // inventory is being held back, which is nothing to say while all of it
    // is on screen.
    {
        let t = Tokens::get(ui.ctx());
        let shown = match tab {
            ResultsBrowserTab::Signals => signal_count,
            ResultsBrowserTab::Datasets => runs.len(),
            ResultsBrowserTab::Expressions => expressions.len(),
        };
        let loaded = match tab {
            ResultsBrowserTab::Signals => app
                .state
                .model
                .simulation
                .retained
                .runs
                .iter()
                .map(|run| {
                    run.analyses
                        .iter()
                        .map(|analysis| {
                            analysis.waveforms.len()
                                + retained_result_artifacts(
                                    analysis,
                                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                                )
                                .len()
                        })
                        .sum::<usize>()
                })
                .sum(),
            ResultsBrowserTab::Datasets => app.state.model.simulation.retained.runs.len(),
            ResultsBrowserTab::Expressions => expression_source.len(),
        };
        // Two numbers, no noun: the tab band above already names what is being
        // counted and the scope control beside it already says which working
        // set is in force, so spelling either one again costs the row the width
        // the count needs at the dock's minimum. With everything on screen the
        // tab's own count has already said it, so the band says nothing.
        let count_copy = (shown != loaded).then(|| format!("{shown} / {loaded}"));
        // Nothing held back and no scope control to hold: the band would be an
        // empty strip under a rule.
        if count_copy.is_some() || tab == ResultsBrowserTab::Signals {
            ui.add_space(RESULT_BROWSER_BAND_PAD_Y);
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), t.metrics.ctl_h.max(28.0)),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.add_space(8.0);
                    if tab == ResultsBrowserTab::Signals {
                        // One scope is in force at a time, so the three read as
                        // one control rather than three independent marks.
                        const SCOPES: [ResultsBrowserScope; 3] = [
                            ResultsBrowserScope::All,
                            ResultsBrowserScope::Favorites,
                            ResultsBrowserScope::Recent,
                        ];
                        let mut index = SCOPES
                            .iter()
                            .position(|candidate| *candidate == scope)
                            .unwrap_or(0);
                        let changed = crate::ui::widgets::segmented(
                            ui,
                            "workbench.results.browser-scope",
                            &["All", "Favorites", "Recent"],
                            &mut index,
                            crate::ui::widgets::SegmentedWidth::Natural,
                        );
                        if changed {
                            let scope_now = SCOPES[index];
                            ui.ctx().data_mut(|data| {
                                data.insert_temp(results_browser_scope_id(), scope_now);
                            });
                        }
                    }
                    // The count is the band's last register: how many of the
                    // tab's rows the filter in force is letting through.
                    if let Some(count_copy) = count_copy {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(count_copy)
                                    .font(theme::mono(tokens::FS_MICRO, FontWeight::Medium))
                                    .color(t.color.text_faint),
                            )
                            .on_hover_text("Matching rows of the rows this tab holds");
                        });
                    }
                },
            );
            ui.add_space(RESULT_BROWSER_BAND_PAD_Y);
            // `.result-browser-status` closes on its own rule, like the toolbar
            // above it, so the scope control reads as a band of its own instead
            // of as the first row of the list below.
            ui.painter().hline(
                egui::Rangef::new(ui.max_rect().left(), ui.max_rect().right()),
                ui.cursor().top() - 0.5,
                egui::Stroke::new(1.0, t.color.border),
            );
        }
    }
    // What a batch action would act on, with only the actions RSpice actually
    // performs on a set of quantities — plot membership, and letting the set
    // go. The band belongs to a selection: with nothing checked it held one
    // button, and the row checkboxes under it are how a selection starts.
    if tab == ResultsBrowserTab::Signals
        && !app
            .state
            .results
            .results
            .session
            .checked_result_quantities
            .is_empty()
    {
        let t = Tokens::get(ui.ctx());
        let checked = app
            .state
            .results
            .results
            .session
            .checked_result_quantities
            .clone();
        let checked_ordered =
            ordered_checked_result_keys(&checked, &app.state.model.simulation.retained.runs);
        let exact_validation_error = checked_ordered.iter().find_map(|key| {
            crate::workbench::documents::result_document::validate_result_browser_selection_evidence(
                key,
                &app.state.model.simulation.retained.runs,
            )
            .err()
        });
        let exact_evidence_available = exact_validation_error.is_none();
        let plot_compatible = !checked.is_empty()
            && exact_evidence_available
            && checked.iter().all(|quantity| quantity.waveform().is_some());
        let clipboard_sample_count = checked_ordered
            .iter()
            .filter_map(ResultBrowserSelectionKey::waveform)
            .filter_map(|key| {
                key.resolve(&app.state.model.simulation.retained.runs)
                    .map(|(.., waveform)| waveform.x.len())
            })
            .sum::<usize>();
        let clipboard_exact_available = exact_evidence_available
            && !checked_ordered.is_empty()
            && clipboard_sample_count <= RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT;
        let compare_selection_compatible = exact_evidence_available
            && checked_ordered.len() >= 2
            && checked_ordered
                .iter()
                .map(ResultBrowserSelectionKey::dataset_id)
                .collect::<std::collections::HashSet<_>>()
                .len()
                >= 2
            && checked_ordered
                .iter()
                .filter_map(|key| {
                    crate::workbench::documents::result_document::result_browser_selection_canonical_name(
                        key,
                        &app.state.model.simulation.retained.runs,
                    )
                    .ok()
                })
                .collect::<std::collections::HashSet<_>>()
                .len()
                == 1;
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 29.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.painter()
                    .rect_filled(ui.available_rect_before_wrap(), 0.0, t.color.accent_dim);
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(format!("{} selected", checked.len()))
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(6.0);
                    ui.menu_button("Actions...", |ui| {
                        if ui
                            .add_enabled(
                                !visible_result_keys.is_empty(),
                                egui::Button::new("Select visible quantities"),
                            )
                            .clicked()
                        {
                            app.state
                                .results
                                .results
                                .session.select_visible_signals(&visible_result_keys);
                            ui.close();
                        }
                        if ui.button("Clear selection").clicked() {
                            app.state.results.results.session.clear_checked_signals();
                            ui.close();
                        }
                        let show = ui.add_enabled(
                            plot_compatible,
                            egui::Button::new("Show selected waveforms"),
                        );
                        if !plot_compatible {
                            show.clone().on_disabled_hover_text(
                                exact_validation_error.as_deref().unwrap_or(
                                    "Only waveforms can be plotted",
                                ),
                            );
                        }
                        if show.clicked() {
                            set_checked_signal_visibility(app, &checked, true);
                            ui.close();
                        }
                        let hide = ui.add_enabled(
                            plot_compatible,
                            egui::Button::new("Hide selected waveforms"),
                        );
                        if !plot_compatible {
                            hide.clone().on_disabled_hover_text(
                                exact_validation_error.as_deref().unwrap_or(
                                    "Only waveforms can be plotted",
                                ),
                            );
                        }
                        if hide.clicked() {
                            set_checked_signal_visibility(app, &checked, false);
                            ui.close();
                        }
                        ui.separator();
                        if ui
                            .add_enabled(
                                !checked_ordered.is_empty(),
                                egui::Button::new("Copy names"),
                            )
                            .clicked()
                        {
                            let result = checked_ordered
                                .iter()
                                .map(|key| {
                                    crate::workbench::documents::result_document::result_browser_selection_canonical_name(
                                        key,
                                        &app.state.model.simulation.retained.runs,
                                    )
                                })
                                .collect::<Result<Vec<_>, _>>()
                                .map(|names| names.join("\n"));
                            match result {
                                Ok(names) => ui.ctx().copy_text(names),
                                Err(error) => app.state.prompt(error),
                            }
                            ui.close();
                        }
                        let copy_exact = ui.add_enabled(
                            clipboard_exact_available,
                            egui::Button::new("Copy data"),
                        );
                        if !checked_ordered.is_empty() && !clipboard_exact_available {
                            copy_exact.clone().on_disabled_hover_text(
                                exact_validation_error.clone().unwrap_or_else(|| format!(
                                    "The waveform portion holds {clipboard_sample_count} samples; clipboard copy is limited to {RESULT_BROWSER_CLIPBOARD_SAMPLE_LIMIT}. Export it instead."
                                )),
                            );
                        }
                        if copy_exact.clicked() {
                            match crate::workbench::documents::result_document::exact_result_browser_selection_bundle(
                                &checked_ordered,
                                &app.state.model.simulation.retained.runs,
                            ) {
                                Ok(exact) if exact.len() <= RESULT_BROWSER_CLIPBOARD_BYTE_LIMIT => {
                                    ui.ctx().copy_text(exact);
                                }
                                Ok(exact) => app.state.prompt(format!(
                                    "The selection is {} bytes; clipboard copy is limited to {RESULT_BROWSER_CLIPBOARD_BYTE_LIMIT} bytes. Export it instead.",
                                    exact.len()
                                )),
                                Err(error) => app.state.prompt(error),
                            }
                            ui.close();
                        }
                        let export_exact = ui.add_enabled(
                            exact_evidence_available && !checked_ordered.is_empty(),
                            egui::Button::new("Export\u{2026}"),
                        );
                        if let Some(error) = exact_validation_error.as_deref() {
                            export_exact.clone().on_disabled_hover_text(error);
                        }
                        if export_exact.clicked() {
                            app.state
                                .workbench
                                .outbox
                                .push(ExportResultQuantities(checked_ordered.clone()));
                            ui.close();
                        }
                        ui.separator();
                        let compare = ui.add_enabled(
                            compare_selection_compatible
                                && app.command_key_enabled(ResultsCommand::CompareResultDatasets.key()),
                            egui::Button::new("Compare selected quantity across datasets..."),
                        );
                        if !checked_ordered.is_empty() && !compare_selection_compatible {
                            compare.clone().on_disabled_hover_text(
                                exact_validation_error.as_deref().unwrap_or(
                                    "Select the same signal in at least two runs.",
                                ),
                            );
                        }
                        if compare.clicked() {
                            if let Some(first) = checked_ordered.first()
                                && select_result_browser_key(app, first)
                            {
                                app.execute_command_key(ResultsCommand::CompareResultDatasets.key());
                            }
                            ui.close();
                        }
                    });
                });
            },
        );
        // `.result-browser-selection` closes on a rule too, so the selection
        // band and the list under it do not run together.
        ui.painter().hline(
            egui::Rangef::new(ui.max_rect().left(), ui.max_rect().right()),
            ui.cursor().top() - 0.5,
            egui::Stroke::new(1.0, t.color.border),
        );
    }
    if tab == ResultsBrowserTab::Signals {
        show_virtualized_result_signals(
            ui,
            app,
            &runs,
            active_run,
            active_analysis,
            selected_trace.as_ref(),
            selected_artifact.as_ref(),
            &visible_result_keys,
            &query,
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
                        if app.state.model.simulation.retained.runs.is_empty() {
                            "No results yet. Run a simulation."
                        } else {
                            "No dataset or analysis matches this filter."
                        },
                    );
                }
                for run in runs {
                    let run_active = active_run == Some(run.run_index);
                    let run_meta = data_state_meta(run.data_state);
                    let overlaid = app
                        .state
                        .model
                        .simulation
                        .is_dataset_overlaid(run.dataset_id);
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
                        select_result_dataset(app, run.run_index);
                    }
                    let mut details = false;
                    let mut log = false;
                    responses.selection.context_menu(|ui| {
                        if ui.button("Run details\u{2026}").clicked() {
                            details = true;
                            ui.close();
                        }
                        if ui.button("Open run log").clicked() {
                            log = true;
                            ui.close();
                        }
                    });
                    if let Some(run_id) = app
                        .state
                        .model
                        .simulation
                        .retained
                        .runs
                        .get(run.run_index)
                        .map(|retained| retained.id)
                    {
                        if details {
                            super::super::inspector::run_details::open(&mut app.state, run_id);
                        }
                        if log {
                            open_run_log(&app.state.model, &mut app.state.workbench, run_id);
                        }
                    }
                    if responses.overlay.is_some_and(|response| response.clicked()) {
                        let enabled = app
                            .state
                            .model
                            .simulation
                            .toggle_dataset_overlay(run.dataset_id);
                        app.state.prompt(if enabled {
                            format!("Overlaying {} on the active result sheet.", run.label)
                        } else {
                            format!("Removed {} from the active result sheet.", run.label)
                        });
                    }
                    if !run_active && query.is_empty() {
                        continue;
                    }
                    for analysis in run.analyses {
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
                            select_result_analysis_by_key(app, analysis.presentation_key);
                        }
                    }
                }
            }
            ResultsBrowserTab::Expressions => {
                // Both ways of having no analysis read the same to the
                // reader: there is one to pick and they have not picked it.
                if app
                    .state
                    .model
                    .simulation
                    .view
                    .active_analysis_idx
                    .is_none()
                    || active_analysis_key.is_none()
                {
                    muted(ui, "Select an analysis to see its expressions.");
                    return;
                }
                expression_header(ui, app);
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
                    locate_on_schematic_menu(&responses.selection, app, &expression.text);
                }
                if let Some(identity) = toggled_expression {
                    match app
                        .state
                        .results
                        .results
                        .toggle_expression_visibility_by_key(&app.state.model.simulation, &identity)
                    {
                        Ok(()) => {
                            app.state
                                .model
                                .workspace
                                .content
                                .visualization_documents_dirty = true;
                        }
                        Err(error) => app.state.prompt(error),
                    }
                }
            }
        });
}

fn ordered_checked_result_keys(
    checked: &std::collections::HashSet<ResultBrowserSelectionKey>,
    runs: &[crate::state::SimulationRun],
) -> Vec<ResultBrowserSelectionKey> {
    let mut keys = checked.iter().cloned().collect::<Vec<_>>();
    keys.sort_by(|left, right| {
        let left =
            crate::workbench::documents::result_document::result_browser_selection_stable_path(
                left, runs,
            )
            .unwrap_or_default();
        let right =
            crate::workbench::documents::result_document::result_browser_selection_stable_path(
                right, runs,
            )
            .unwrap_or_default();
        left.cmp(&right)
    });
    keys
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

/// Quantity-kind facet of the browser toolbar.
///
/// Waveform kinds come from their exact engineering unit. Typed producer
/// artifacts carry their own scalar/array/event/contribution classification,
/// so those facets never guess from a display label.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub(super) enum ResultsBrowserKind {
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
    pub(super) const BASE_UNITS: [&'static str; 15] = [
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
    pub(super) fn admits(self, unit: &str) -> bool {
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
    pub(super) fn admits_artifact(self, kind: ResultArtifactKind, unit: &str) -> bool {
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

/// Row ordering facet: retained hierarchy order, or name.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ResultsBrowserSort {
    #[default]
    Hierarchy,
    Name,
}

impl ResultsBrowserSort {
    const ALL: [Self; 2] = [Self::Hierarchy, Self::Name];

    const fn label(self) -> &'static str {
        match self {
            // Labels stay inside the facet's own share of the row: the pair is
            // sized to the panel, not to its longest string.
            Self::Hierarchy => "Hierarchy",
            // The mockup's third option, "Recently used", is the Recent
            // scope on the status band below: one owner for recency, not a
            // sort that silently competes with it.
            Self::Name => "Name",
        }
    }
}

/// Working-set scope of the signals tab: everything, the user's starred
/// signals, or the signals touched most recently. Backed by the real
/// favorite/recency state on `ResultsState`, never a static filter.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub(super) enum ResultsBrowserScope {
    #[default]
    All,
    Favorites,
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
pub(super) struct ResultAnalysisDataState {
    pub(super) word: &'static str,
    pub(super) note: &'static str,
}

/// A dataset row's state word, only when the data is not current: a current
/// dataset is the normal case and its row says nothing about it.
pub(super) fn data_state_meta(state: ResultAnalysisDataState) -> &'static str {
    if state.word == "current" {
        ""
    } else {
        state.word
    }
}

pub(super) const fn result_analysis_data_state(
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
    app: &mut RSpiceApp,
    kind: ResultsBrowserKind,
    scope: ResultsBrowserScope,
) {
    let producer_filtering = ui.ctx().data(|data| {
        data.get_temp::<Option<AnalysisPresentationKey>>(results_browser_producer_id())
            .flatten()
            .is_some()
    });
    let filtering = !app.state.workbench.navigator_filter().trim().is_empty()
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
            app.state.workbench.clear_navigator_filter();
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

/// The data browser's own toolbar: the query, the producing analysis, then the
/// kind and sort pair.
///
/// The mockup lays this out as one bordered block under the tab band with an
/// 8 px inset and a 5 px gutter, in three registers widest-first. The analysis
/// facet takes a whole row because its option strings are the longest in the
/// panel and because it is the one control that changes what the tree contains
/// rather than how it reads; the typed pair below splits unevenly, since kind
/// carries the longer option names and sort carries three short ones. Facets
/// are sized to the panel rather than to their own labels, so the block reads
/// as one control stack at any dock width.
fn results_browser_toolbar(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    kind: ResultsBrowserKind,
    sort: ResultsBrowserSort,
    producer: Option<AnalysisPresentationKey>,
    show_quantity_facets: bool,
) {
    const GUTTER: f32 = 5.0;

    let t = Tokens::get(ui.ctx());
    let workspace = app.state.workbench.workspace;
    ui.add_space(7.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(PANEL_SEARCH_MARGIN_X);
        let field_width = panel_search_field_width(ui.available_width() + PANEL_SEARCH_MARGIN_X);
        let placeholder = "Find signal…";
        let field_id = ui.make_persistent_id("workbench.navigator.filter");
        if search_focus_requested::<FocusNavigatorSearch>(ui, &mut app.state.workbench.outbox) {
            ui.memory_mut(|memory| memory.request_focus(field_id));
        }
        let response = ui.add_sized(
            [field_width, t.metrics.ctl_h],
            egui::TextEdit::singleline(app.state.workbench.navigator_trees.filter_mut(workspace))
                .id(field_id)
                .hint_text(placeholder)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .margin(egui::Margin {
                    left: 29,
                    right: 8,
                    top: 5,
                    bottom: 5,
                }),
        );
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label(placeholder);
            node.set_description("Filter the quantities of the selected browser tab");
        });
        WorkbenchIcon::Search.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(response.rect.left() + 15.0, response.rect.center().y),
                egui::vec2(16.0, 16.0),
            ),
            t.color.text_faint,
        );
    });
    if show_quantity_facets {
        // The producing analysis, on its own row. Every analysis the session
        // retained is offered under one leading "all" entry that carries the
        // count, so a reader can see how many producers the tree spans without
        // opening the list.
        let producers = app
            .state
            .model
            .simulation
            .retained
            .runs
            .iter()
            .flat_map(|run| {
                run.analyses.iter().map(move |analysis| {
                    (
                        AnalysisPresentationKey::new(run.dataset_id, analysis),
                        format!("{} · {}", run.label, analysis.label),
                    )
                })
            })
            .collect::<Vec<_>>();
        let selected_producer = producer.and_then(|selected| {
            producers
                .iter()
                .find(|(key, _)| *key == selected)
                .map(|(_, label)| label.clone())
        });
        let mut producer_options = vec![format!("All analyses · {}", producers.len())];
        producer_options.extend(producers.iter().map(|(_, label)| label.clone()));
        ui.add_space(GUTTER);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.add_space(PANEL_SEARCH_MARGIN_X);
            let row_width = ui.available_width() - PANEL_SEARCH_MARGIN_X;
            if let Some(picked) = crate::ui::widgets::select(
                ui,
                "workbench.results.browser-producer",
                "Producing analysis",
                selected_producer.as_deref().unwrap_or(&producer_options[0]),
                &producer_options,
                row_width,
            ) {
                let key = picked
                    .checked_sub(1)
                    .and_then(|index| producers.get(index).map(|(key, _)| *key));
                ui.ctx()
                    .data_mut(|data| data.insert_temp(results_browser_producer_id(), key));
            }
        });
        ui.add_space(GUTTER);
        ui.horizontal(|ui| {
            // Spacing is added explicitly: an `add_space` inside a horizontal
            // with item spacing pays for both, and the facets drift off the
            // query field's inset.
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.add_space(PANEL_SEARCH_MARGIN_X);
            // The design system's select allocates exactly the width it is
            // given, so the pair can be split against the row. A raw combo box
            // pads itself past the width it is asked for, which pushed the sort
            // facet off the query's inset and clipped it at the panel edge.
            let row_width = ui.available_width() - PANEL_SEARCH_MARGIN_X;
            let kind_options = ResultsBrowserKind::ALL
                .map(|value| value.label().to_owned())
                .to_vec();
            let sort_options = ResultsBrowserSort::ALL
                .map(|value| value.label().to_owned())
                .to_vec();
            if result_browser_facets_stack(row_width) {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = GUTTER;
                    if let Some(picked) = crate::ui::widgets::select(
                        ui,
                        "workbench.results.browser-kind",
                        "Quantity kind",
                        kind.label(),
                        &kind_options,
                        row_width,
                    ) {
                        let kind_now = ResultsBrowserKind::ALL[picked];
                        ui.ctx()
                            .data_mut(|data| data.insert_temp(results_browser_kind_id(), kind_now));
                    }
                    if let Some(picked) = crate::ui::widgets::select(
                        ui,
                        "workbench.results.browser-sort",
                        "Quantity sort",
                        sort.label(),
                        &sort_options,
                        row_width,
                    ) {
                        let sort_now = ResultsBrowserSort::ALL[picked];
                        ui.ctx()
                            .data_mut(|data| data.insert_temp(results_browser_sort_id(), sort_now));
                    }
                });
            } else {
                // Kind takes the wider share of the pair: its option list holds
                // the long names, sort holds three short ones.
                let pair_width = row_width - GUTTER;
                let kind_width = (pair_width * 0.554).max(48.0);
                let sort_width = (pair_width - kind_width).max(48.0);
                if let Some(picked) = crate::ui::widgets::select(
                    ui,
                    "workbench.results.browser-kind",
                    "Quantity kind",
                    kind.label(),
                    &kind_options,
                    kind_width,
                ) {
                    let kind_now = ResultsBrowserKind::ALL[picked];
                    ui.ctx()
                        .data_mut(|data| data.insert_temp(results_browser_kind_id(), kind_now));
                }
                ui.add_space(GUTTER);
                if let Some(picked) = crate::ui::widgets::select(
                    ui,
                    "workbench.results.browser-sort",
                    "Quantity sort",
                    sort.label(),
                    &sort_options,
                    sort_width,
                ) {
                    let sort_now = ResultsBrowserSort::ALL[picked];
                    ui.ctx()
                        .data_mut(|data| data.insert_temp(results_browser_sort_id(), sort_now));
                }
            }
        });
    }
    ui.add_space(7.0);
    // The toolbar is one block, so it closes with a rule like the tab band
    // above it rather than bleeding into the status band below.
    ui.painter().hline(
        egui::Rangef::new(ui.max_rect().left(), ui.max_rect().right()),
        ui.cursor().top() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
}

fn results_browser_kind_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-kind-facet")
}

fn results_browser_sort_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-sort-facet")
}

fn results_browser_scope_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-scope")
}

fn results_browser_producer_id() -> egui::Id {
    egui::Id::new("workbench.results.browser-producer-facet")
}

fn results_browser_active_tab(ctx: &egui::Context) -> ResultsBrowserTab {
    ctx.data(|data| {
        data.get_temp::<ResultsBrowserTab>(egui::Id::new("workbench.results.browser-tab"))
    })
    .unwrap_or_default()
}

pub(super) fn result_browser_dataset_expanded(
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

pub(super) fn set_result_browser_dataset_expanded(
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
pub(super) fn result_browser_group_expanded(
    ctx: &egui::Context,
    analysis: AnalysisPresentationKey,
    active: bool,
) -> bool {
    ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new(("workbench.results.browser-group", analysis)))
    })
    .unwrap_or(active)
}

pub(super) fn set_result_browser_group_expanded(
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
    pub(super) signals: Vec<ResultSignal>,
    pub(super) artifacts: Vec<ResultArtifact>,
}

pub(super) struct ResultSignal {
    pub(super) waveform_index: usize,
    pub(super) identity: SourceWaveformPresentationKey,
    pub(super) name: String,
    pub(super) color: String,
    pub(super) visible: bool,
    pub(super) value: Option<String>,
    /// The row's second register, empty when there is nothing to put in it.
    pub(super) meta: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ResultArtifactKind {
    Scalar,
    Array,
    EventStream,
    Contribution,
}

impl ResultArtifactKind {
    const fn label(self) -> &'static str {
        match self {
            Self::Scalar => "Scalar",
            Self::Array => "Array",
            Self::EventStream => "Event stream",
            Self::Contribution => "Contribution",
        }
    }
}

pub(super) struct ResultArtifact {
    pub(super) identity: ResultArtifactPresentationKey,
    pub(super) name: String,
    pub(super) kind: ResultArtifactKind,
    pub(super) unit: String,
    pub(super) meta: String,
    pub(super) value: Option<String>,
    pub(super) viewer: crate::workbench::ResultViewer,
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
    Signal {
        run: usize,
        analysis: usize,
        signal: usize,
    },
    Artifact {
        run: usize,
        analysis: usize,
        artifact: usize,
    },
}
