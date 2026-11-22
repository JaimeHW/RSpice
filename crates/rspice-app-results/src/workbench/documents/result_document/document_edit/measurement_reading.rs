//! What a kept measurement reads.
//!
//! A measurement keeps its definition (an expression, the window it is read
//! over and, over a family, the statistic it reports), never a value. Each
//! time it is shown it is read again from the run shown, through its
//! window, as EZwave's measurement tool and ADE's outputs read theirs, so a
//! re-run, an overlaid run or a zoom changes what it reads and nothing goes
//! stale.

use rspice_results::visualization_document::{
    FamilyMeasurementSummary, FamilyStatistic, Measurement, MeasurementWindow,
};

use super::*;

/// What a kept measurement read on one run.
#[derive(Debug, Clone, PartialEq)]
pub struct KeptReading {
    /// The number it reports: its value, or over a family the statistic it
    /// names.
    pub value: f64,
    /// The unit the value reads in; `%` for a yield.
    pub unit: String,
    /// Over a family, what its members read.
    pub family: Option<FamilyMeasurementSummary>,
}

/// What `measurement`, kept by `document`, reads on `run`: on the analysis
/// of `run` its pane reads, over its window. The error states why it reads
/// nothing there.
pub fn read_kept(
    state: ResultsReads<'_>,
    document: &VisualizationDocument,
    measurement: &Measurement,
    run: &SimulationRun,
) -> Result<KeptReading, String> {
    let expression = measurement
        .expression
        .as_deref()
        .ok_or_else(|| "This measurement names no expression.".to_owned())?;
    let analysis = pane_analysis(document, measurement.pane_id, run)
        .ok_or_else(|| "This run has no such analysis.".to_owned())?;
    let unit = || {
        super::super::waves::measurement_unit(
            expression,
            analysis,
            analysis.analysis_type.axis_info().1,
        )
        .unwrap_or_default()
    };
    if let Some(family) = measurement.family {
        if measurement.window != MeasurementWindow::WholeRecord {
            return Err("A family measurement reads each member's whole record.".to_owned());
        }
        let summary = super::measurement::family_reading(run, analysis, expression, family)?;
        let value = summary
            .statistic_value()
            .ok_or_else(|| "The family has no value for that statistic.".to_owned())?;
        return Ok(KeptReading {
            value,
            unit: if family.statistic == FamilyStatistic::Yield {
                "%".to_owned()
            } else {
                unit()
            },
            family: Some(summary),
        });
    }
    let window = window(state, measurement.window, run, analysis)?;
    Ok(KeptReading {
        value: super::measurement::evaluate_over(analysis, expression, window)?,
        unit: unit(),
        family: None,
    })
}

/// The part of the x-axis a measurement over `window` reads on `analysis`
/// of `run`; `None` for every sample.
fn window(
    state: ResultsReads<'_>,
    window: MeasurementWindow,
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> Result<Option<(f64, f64)>, String> {
    let session = &state.results.session;
    match window {
        MeasurementWindow::WholeRecord => Ok(None),
        // Unzoomed, the strip shows the whole record.
        MeasurementWindow::VisibleRange => Ok(rspice_results_ui::waves::stack::shared_x_view(
            session,
            super::super::AnalysisPresentationKey::new(run.dataset_id, analysis),
        )),
        MeasurementWindow::BetweenCursors => {
            let place = run
                .analyses
                .iter()
                .position(|candidate| std::ptr::eq(candidate, analysis));
            let on_strip = session
                .cursor_strip
                .is_none_or(|strip| Some(strip) == place);
            match (session.cursors.a, session.cursors.b) {
                (Some(a), Some(b)) if on_strip && a != b => Ok(Some((a.min(b), a.max(b)))),
                _ => Err("Place cursors A and B on this strip to read between them.".to_owned()),
            }
        }
    }
}

/// What a reading over the visible range or between the cursors depends on
/// beyond the data and the view: where the strips are zoomed and where the
/// cursors stand.
pub(crate) fn window_version(state: ResultsReads<'_>) -> u64 {
    use std::hash::{Hash, Hasher};
    let session = &state.results.session;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    session.cursor_strip.hash(&mut hasher);
    for cursor in [session.cursors.a, session.cursors.b] {
        cursor.map(f64::to_bits).hash(&mut hasher);
    }
    if let Some(run) = state.model.simulation.active_run() {
        for analysis in &run.analyses {
            let key = super::super::AnalysisPresentationKey::new(run.dataset_id, analysis);
            rspice_results_ui::waves::stack::shared_x_view(session, key)
                .map(|(from, to)| (from.to_bits(), to.to_bits()))
                .hash(&mut hasher);
        }
    }
    hasher.finish()
}

/// The analysis of `run` that `document`'s pane `pane_id` reads: the one
/// its binding names, which every run of a plan solves.
pub(super) fn pane_analysis<'a>(
    document: &VisualizationDocument,
    pane_id: PaneId,
    run: &'a SimulationRun,
) -> Option<&'a AnalysisResult> {
    let binding = document
        .panes()
        .iter()
        .find(|pane| pane.id == pane_id)?
        .binding?;
    run.analyses.iter().find(|analysis| {
        super::super::create_document::source_analysis_identity(run, analysis)
            == binding.analysis_id
    })
}
