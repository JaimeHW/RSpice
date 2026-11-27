//! The Sweep sheet's source: the family the shown analysis belongs to, read
//! once as a table and kept while the retained runs and the shown analysis
//! stay as they are.

use super::{AnalysisPresentationKey, ResultViewer, ResultsReads, SheetContext};
use crate::state::{RunHistoryRevision, SimulationState};
use egui::Ui;
use rspice_results::sweep_table::{self, OutputKind, SweepTable};
use rspice_results_ui::sweep as view;
use std::sync::Arc;

/// The shown analysis's family as rows, and what it was read from.
#[derive(Debug, Clone)]
pub(super) struct SweepPlan {
    source: (RunHistoryRevision, u64),
    analysis: AnalysisPresentationKey,
    table: SweepTable,
}

fn source_of(simulation: &SimulationState) -> (RunHistoryRevision, u64) {
    (
        simulation.retained.runs.revision(),
        simulation.view.data_version,
    )
}

/// The position of the shown analysis in the active run.
fn shown_index(simulation: &SimulationState) -> Option<usize> {
    let run = simulation.active_run()?;
    match simulation.view.active_analysis_idx {
        Some(index) if index < run.analyses.len() => Some(index),
        _ => (!run.analyses.is_empty()).then_some(0),
    }
}

/// The family of the shown analysis as a table, read again only when the
/// retained runs or the shown analysis changed.
pub(super) fn plan(context: &mut SheetContext<'_>) -> Option<Arc<SweepPlan>> {
    let simulation = context.simulation;
    let run = simulation.active_run()?;
    let index = shown_index(simulation)?;
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[index]);
    let source = source_of(simulation);
    if let Some(plan) = context.results.plans.sweep.as_ref()
        && plan.source == source
        && plan.analysis == key
    {
        return Some(Arc::clone(plan));
    }
    context.results.plans.sweep = None;
    let built = Arc::new(SweepPlan {
        source,
        analysis: key,
        table: sweep_table::sweep_table(&run.analyses, index)?,
    });
    context.results.plans.sweep = Some(Arc::clone(&built));
    Some(built)
}

/// Whether the shown analysis is a point of a sweep the sheet can draw.
pub(super) fn shown_is_sweep(state: ResultsReads<'_>) -> bool {
    let simulation = &state.model.simulation;
    simulation
        .active_run()
        .zip(shown_index(simulation))
        .is_some_and(|(run, index)| sweep_table::has_sweep_table(&run.analyses, index))
}

pub(super) fn domain_bar(ui: &mut Ui, context: &mut SheetContext<'_>) -> bool {
    let plan = plan(context);
    let session = &mut context.results.session;
    let before = (*session.sweep).clone();
    let drawn = view::domain_bar(
        ui,
        plan.as_ref().map(|plan| &plan.table),
        &mut session.sweep,
    );
    // A window belongs to the value and the quantity it was set on.
    if session.sweep.output != before.output || session.sweep.x != before.x {
        session.reset_plot_view(ResultViewer::Sweep, 0);
    }
    drawn
}

/// Each row's value of the output the sheet draws: what each point
/// measured, or a signal read on each member's own samples where the
/// reader asked.
fn drawn_values(
    simulation: &SimulationState,
    table: &SweepTable,
    state: &view::SweepSheetState,
) -> Vec<Option<f64>> {
    let (Some(output), _) = view::resolve(table, state) else {
        return Vec::new();
    };
    let column = &table.outputs[output];
    match column.kind {
        OutputKind::Signal => simulation.active_run().map_or_else(Vec::new, |run| {
            sweep_table::signal_values(&run.analyses, table, &column.name, state.at)
        }),
        OutputKind::Measurement | OutputKind::Sample => column.values.clone(),
    }
}

pub fn show(ui: &mut Ui, context: &mut SheetContext<'_>) {
    let plan = plan(context);
    let values = plan.as_ref().map_or_else(Vec::new, |plan| {
        drawn_values(
            context.simulation,
            &plan.table,
            &context.results.session.sweep,
        )
    });
    let kind_tag = context
        .simulation
        .active_analysis()
        .map_or_else(String::new, |analysis| {
            analysis.analysis_type.short_label().to_uppercase()
        });
    let session = &mut context.results.session;
    let pinned = session.plot_view(ResultViewer::Sweep, 0);
    let state = (*session.sweep).clone();
    let drawn = view::show(
        ui,
        plan.as_ref().map(|plan| view::SweepView {
            table: &plan.table,
            values: &values,
            state: &state,
            plot_view: pinned,
            cache_key: egui::Id::new(("sweep", plan.analysis)).value(),
            significant_digits: context.significant_digits,
            kind_tag: &kind_tag,
        }),
        &mut session.cache,
    );
    if let Some(x) = drawn.x {
        session.sweep.x = Some(x);
        session.reset_plot_view(ResultViewer::Sweep, 0);
    } else if drawn.fit {
        session.reset_plot_view(ResultViewer::Sweep, 0);
    } else if drawn.view.any() {
        session
            .plot_view_mut(ResultViewer::Sweep, 0)
            .apply(&drawn.view);
    }
    if let Some(plot) = drawn.plot.as_ref() {
        super::record_drawn_axes(context.results, ResultViewer::Sweep, plot);
    }
}

#[cfg(test)]
mod tests;
