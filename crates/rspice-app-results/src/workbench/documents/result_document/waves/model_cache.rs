//! Retained-history invalidation and source selection for waveform projections.
use super::{ComplexNumberDisplay, ResultsState, SimulationState, StripModel, Tokens};
use rspice_results::result_presentation::SourceWaveformPresentationKey;
use rspice_results_ui::derived::DerivedSeries;
use rspice_results_ui::waves::ProjectionOptions;
use rspice_results_ui::waves::cache::{
    ModelCacheInput, ModelsMaker, ModelsView, apply_waveform_visibility,
};
use std::sync::Arc;

pub(in crate::workbench::documents::result_document) fn cached_models(
    simulation: &SimulationState,
    results: &mut ResultsState,
    complex_display: ComplexNumberDisplay,
    t: &Tokens,
) -> Arc<Vec<StripModel>> {
    let models = built_models(simulation, results, complex_display, t);
    if give_own_strip(results, &models) {
        return built_models(simulation, results, complex_display, t);
    }
    models
}

/// Give the curve a Command-click probe plotted a strip of its own, once the
/// models draw it. Whether the strips were arranged anew.
fn give_own_strip(results: &mut ResultsState, models: &[StripModel]) -> bool {
    let Some(key) = results.own_strip_pending.as_ref() else {
        return false;
    };
    let Some((model, index)) = models
        .iter()
        .filter(|model| model.analysis_key == key.analysis())
        .find_map(|model| {
            model
                .traces
                .iter()
                .position(|trace| {
                    !trace.overlay
                        && !trace.is_expression()
                        && trace.source_waveform_name == key.source_name()
                })
                .map(|index| (model, index))
        })
    else {
        return false;
    };
    results.own_strip_pending = None;
    let curve = rspice_results_ui::waves::strip_layout::Curve::Trace(index);
    // A curve that is alone on its strip already keeps it.
    let Ok(strips) = rspice_results_ui::waves::strip_layout::move_to_new_strip(model, curve) else {
        return false;
    };
    results
        .session
        .strip_layouts
        .insert(model.analysis_key, strips);
    results.models.invalidate();
    true
}

fn built_models(
    simulation: &SimulationState,
    results: &mut ResultsState,
    complex_display: ComplexNumberDisplay,
    t: &Tokens,
) -> Arc<Vec<StripModel>> {
    results.synchronize_wave_caches(simulation);
    results.reconcile_expression_projection(simulation);
    let failed = |analysis: super::AnalysisPresentationKey, text: &str| {
        results
            .analysis_expr_cache
            .get(&(analysis, text.to_owned()))
            .is_some_and(|cached| cached.series.is_err())
    };
    let expressions = {
        let mut routed: Vec<_> = results
            .session
            .analysis_exprs
            .iter()
            .flat_map(|(analysis, exprs)| {
                exprs.iter().map(|expr| {
                    (
                        analysis.order_key(),
                        expr.text.as_str(),
                        expr.visible,
                        results
                            .session
                            .expression_units
                            .get(&(*analysis, expr.text.clone())),
                        failed(*analysis, &expr.text),
                    )
                })
            })
            .collect();
        routed.sort();
        // The `digital()` ones are logic lanes, coloured by their place.
        let logic = logic_lanes_fingerprint(&results.session);
        rspice_results_ui::waves::stable_hash(&(routed, logic))
    };
    let failed_texts: std::collections::HashSet<_> = results
        .analysis_expr_cache
        .iter()
        .filter(|(_, cached)| cached.series.is_err())
        .map(|(key, _)| key.clone())
        .collect();
    let session = &mut results.session;
    let strips = &session.strip_layouts;
    let overlaid = simulation.overlaid_dataset_ids();
    let revision = simulation.retained.runs.revision();
    results.models.get_or_build(
        ModelCacheInput {
            view: ModelsView::drawn(session),
            active_run_idx: simulation.view.active_run_idx,
            overlay_dataset_ids: &overlaid,
            viewer: session.viewer,
            projection: ProjectionOptions {
                phase_continuous: session.presented_phase_continuous(),
                complex_display,
                ac: session.ac_display,
                selection: session.sample_selection.as_ref(),
                hidden_family_traces: &session.hidden_family_traces,
                waveform_visibility: &session.waveform_visibility,
                signals: Some(rspice_results_ui::waves::SignalLookup {
                    indices: &session.signal_indices,
                    revision: &revision,
                }),
            },
            waveform_visibility: &session.waveform_visibility,
            waveform_colors: &session.waveform_colors,
            waveform_looks: &session.waveform_looks,
            expressions,
            x_scales: &session.x_scales,
            strips,
            derived: &mut session.derived,
        },
        &simulation.retained.executed_decks,
        t,
        &mut results.plans.envelopes,
        ModelsMaker {
            build: |derived: &mut DerivedSeries,
                    projection: ProjectionOptions<'_>,
                    visibility: &rspice_results_ui::session::VisibilityOverrides| {
            // Whether each analysis is complete, from the memo of its
            // evidence: the validator reads every column of the run.
            let display_runs = simulation.display_runs();
            let validity = &results.retained_evidence_validity;
            let checks = &results.evidence_checks;
            let mut built = rspice_results_ui::waves::build_models(
                &display_runs,
                &simulation.retained.executed_decks,
                derived,
                t,
                projection,
                |index| {
                    let run = display_runs[0];
                    crate::workbench::documents::result_document::remembered_incomplete_reason(
                        validity,
                        checks,
                        simulation,
                        run.dataset_id,
                        &run.analyses[index],
                    )
                },
            );
            // The forms the reader's strips show a signal in, for every
            // member and overlay run the stack draws.
            rspice_results_ui::waves::project_strips(
                &mut built,
                &display_runs,
                strips,
                projection.selection,
            );
            // A running transient's strip frames its run from start to stop while it runs.
            if let Some(run) = simulation.active_run() {
                for model in &mut built {
                    if run
                        .analyses
                        .get(model.analysis_index)
                        .is_some_and(|analysis| analysis.is_live_partial())
                    {
                        model.live_frame = simulation.execution.live_window;
                    }
                }
            }
            apply_waveform_visibility(
                &mut built,
                simulation.active_run().map(AsRef::as_ref),
                visibility,
                &session.waveform_colors,
                projection.hidden_family_traces,
                &t.color.traces,
            );
            rspice_results_ui::waves::cache::apply_waveform_looks(
                &mut built,
                &session.waveform_looks,
            );
            add_logic_lanes(
                &mut built,
                simulation,
                &session.analysis_exprs,
                &session.event_lanes,
                t,
            );
            super::expressions::route_expressions(
                &mut built,
                simulation,
                &session.analysis_exprs,
                &session.expression_units,
                |analysis, text| failed_texts.contains(&(analysis, text.to_owned())),
            );
            built
        },
            retick: |derived: &mut DerivedSeries,
                     models: &mut [StripModel],
                     edited: &[&SourceWaveformPresentationKey],
                     projection: ProjectionOptions<'_>| {
                // A strip that draws the reader's expressions routes them by
                // its curves' units, and is built afresh.
                if edited.iter().any(|key| {
                    session
                        .analysis_exprs
                        .get(&key.analysis())
                        .is_some_and(|exprs| !exprs.is_empty())
                }) {
                    return None;
                }
                rspice_results_ui::waves::retick(
                    models,
                    edited,
                    derived,
                    &rspice_results_ui::waves::Retick {
                        run: simulation.active_run()?.as_ref(),
                        options: projection,
                        colors: &session.waveform_colors,
                        looks: &session.waveform_looks,
                        palette: &t.color.traces,
                        ticks: &session.waveform_visibility,
                    },
                )
            },
        },
    )
}

/// The `digital()` expressions of every strip, with their visibility.
fn logic_lanes_fingerprint(session: &rspice_results_ui::session::ResultViewerState) -> u64 {
    session
        .analysis_exprs
        .iter()
        .flat_map(|(analysis, exprs)| {
            exprs
                .iter()
                .enumerate()
                .filter(|(_, expr)| super::expressions::draws_as_logic(&expr.text))
                .map(move |(slot, expr)| {
                    rspice_results_ui::waves::stable_hash(&(
                        analysis,
                        slot,
                        &expr.text,
                        expr.visible,
                    ))
                })
        })
        .fold(0, |fingerprint, entry| fingerprint ^ entry)
}

/// Each strip's `digital()` expressions as logic lanes of its event history,
/// so they are rows of its lane stack like any `D()` net: read at the cursor,
/// stepped along, grouped into a bus and ordered. An expression that does not
/// evaluate draws no lane; its strip reports why.
fn add_logic_lanes(
    models: &mut [StripModel],
    simulation: &SimulationState,
    exprs: &std::collections::HashMap<
        rspice_results::result_presentation::AnalysisPresentationKey,
        Vec<rspice_results::result_presentation::ExprTrace>,
    >,
    layouts: &std::collections::HashMap<
        rspice_results::result_presentation::AnalysisPresentationKey,
        rspice_results_ui::waves::lanes::EventLaneLayout,
    >,
    t: &Tokens,
) {
    use rspice_results_ui::waves::logic::{EventLanes, LogicLane};
    use rspice_results_ui::waves::{LaneTrace, LaneTraceKind};

    for model in models.iter_mut() {
        let Some(list) = exprs.get(&model.analysis_key) else {
            continue;
        };
        let mut lanes: Option<EventLanes> = None;
        for (slot, expr) in list.iter().enumerate() {
            if !super::expressions::draws_as_logic(&expr.text) {
                continue;
            }
            let Ok(outputs) = super::evaluate_expression_reading(
                simulation,
                (model.analysis_index, None),
                &expr.text,
                None,
                expr.complex_policy,
                layouts.get(&model.analysis_key),
                &super::Places::default(),
            ) else {
                continue;
            };
            let Some(output) = outputs.into_iter().next() else {
                continue;
            };
            let x = output.waveform.data.x;
            // The gutter has room for a short name; the expression's own
            // text stays the row's identity.
            let label = rspice_results_ui::waves::logic::convert::digital_label(&expr.text);
            // An evaluated expression's samples, held in memory.
            let lane = LogicLane::from_levels(
                label.clone(),
                x.in_memory().unwrap_or_default(),
                output.waveform.data.y.in_memory().unwrap_or_default(),
            );
            lanes
                .get_or_insert_with(|| model.event_lanes.as_deref().cloned().unwrap_or_default())
                .logic
                .push(lane);
            model.lane_traces.push(LaneTrace {
                waveform_index: None,
                source_waveform_name: expr.text.clone(),
                net: label,
                kind: LaneTraceKind::Logic,
                unit: None,
                color: super::expressions::expr_color(
                    t,
                    super::expressions::expr_palette_slot(model, slot),
                ),
                x,
                visible: expr.visible,
            });
        }
        if let Some(lanes) = lanes {
            model.event_lanes = Some(Arc::new(lanes));
        }
    }
}
