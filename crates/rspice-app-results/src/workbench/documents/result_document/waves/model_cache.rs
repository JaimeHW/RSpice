//! Retained-history invalidation and source selection for waveform projections.
use super::{
    ComplexNumberDisplay, ResultsState, SimulationState, StripModel, Tokens, build_models,
};
use rspice_results_ui::waves::ProjectionOptions;
use rspice_results_ui::waves::cache::{ModelCacheInput, apply_waveform_visibility};
use std::sync::Arc;

pub(in crate::workbench::documents::result_document) fn cached_models(
    simulation: &SimulationState,
    results: &mut ResultsState,
    complex_display: ComplexNumberDisplay,
    t: &Tokens,
) -> Arc<Vec<StripModel>> {
    results.synchronize_wave_caches(simulation);
    results.reconcile_expression_projection(simulation);
    let logic = logic_lanes_fingerprint(&results.session);
    if results.logic_lanes_source != logic {
        results.models.invalidate();
        results.logic_lanes_source = logic;
    }
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
        rspice_results_ui::waves::stable_hash(&routed)
    };
    let failed_texts: std::collections::HashSet<_> = results
        .analysis_expr_cache
        .iter()
        .filter(|(_, cached)| cached.series.is_err())
        .map(|(key, _)| key.clone())
        .collect();
    let session = &mut results.session;
    results.models.get_or_build(
        ModelCacheInput {
            active_run_idx: simulation.view.active_run_idx,
            overlay_dataset_ids: &simulation.view.overlay_dataset_ids,
            viewer: session.viewer,
            projection: ProjectionOptions {
                phase_continuous: session.presented_phase_continuous(),
                complex_display,
                ac: session.ac_display,
                selection: session.sample_selection.as_ref(),
                hidden_family_traces: &session.hidden_family_traces,
            },
            waveform_visibility: &session.waveform_visibility,
            expressions,
        },
        &simulation.retained.executed_decks,
        t,
        &mut results.plans.envelopes,
        |projection, visibility| {
            let mut built = build_models(
                simulation,
                &mut session.derived,
                t,
                projection.phase_continuous,
                projection.complex_display,
                projection.ac,
                projection.selection,
                projection.hidden_family_traces,
            );
            apply_waveform_visibility(
                &mut built,
                simulation.active_run().map(AsRef::as_ref),
                visibility,
                projection.hidden_family_traces,
                &t.color.traces,
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
                model.analysis_index,
                &expr.text,
                None,
                expr.complex_policy,
                layouts.get(&model.analysis_key),
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
            let lane = LogicLane::from_levels(label.clone(), &x, &output.waveform.data.y);
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
