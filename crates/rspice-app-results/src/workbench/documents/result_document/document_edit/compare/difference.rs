//! The differences a comparison found, on the result's waveform sheet: each
//! drawn as `V(out) - V(out)@baseline` with its tolerance band about zero,
//! the places the runs part shaded, and the cursor stepped from place to
//! place.

use rspice_results::calculator::ast::CalculatorExpr;
use rspice_results::calculator::datasets::BASELINE;
use rspice_results::calculator::retained::find_waveform;
use rspice_results::feature_search::Direction;
use rspice_results_ui::waves::navigation::{place_stepping_cursor, stepping_from};
use rspice_results_ui::waves::pane::{ComparisonMarks, ResolvedExpr, ToleranceBand};
use rspice_results_ui::waves::{StripModel, UnitPane};

use super::*;
use crate::ui::tokens::Tokens;

const NO_COMPARISON: &str =
    "No comparison has answered yet; compare this result with a baseline first.";

/// The expression that draws `signal`'s difference from the pinned baseline.
pub(crate) fn difference_text(signal: &str) -> String {
    format!("{signal} - {signal}@{BASELINE}")
}

/// Whether `signal` reads as itself in an expression.
fn nameable(signal: &str) -> bool {
    matches!(
        crate::analysis::calculator::parser::try_parse(signal),
        Ok(CalculatorExpr::WaveformRef { signal: named, dataset: None })
            if named.eq_ignore_ascii_case(signal)
    )
}

impl Finished {
    /// The compared signals that part from the baseline, or every compared
    /// signal when none does.
    fn differing(&self) -> Vec<&str> {
        let parted: Vec<&str> = self
            .outcome
            .receipt
            .signals
            .iter()
            .zip(&self.outcome.signals)
            .filter(|(signal, _)| signal.failed_rows > 0)
            .map(|(_, name)| name.as_str())
            .collect();
        if parted.is_empty() {
            self.outcome.signals.iter().map(String::as_str).collect()
        } else {
            parted
        }
    }

    /// The tolerance band of the difference `text`, drawn by `expr`, read
    /// once for each evaluation of it.
    fn band(
        &self,
        state: ResultsReads<'_>,
        (text, signal): (&str, &str),
        expr: &ResolvedExpr,
        (fill, edge): (egui::Color32, egui::Color32),
    ) -> Option<Arc<ToleranceBand>> {
        let mut bands = self
            .bands
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some((key, band)) = bands.get(text)
            && *key == expr.cache_key
        {
            return band.clone();
        }
        // A band whose result the reader service is still reading is asked
        // for again on a later frame, not kept as none.
        let band = self
            .read_band(state, signal, expr, (fill, edge))?
            .map(Arc::new);
        bands.insert(text.to_owned(), (expr.cache_key, band.clone()));
        band
    }

    /// How far from zero the difference of `signal` may stand at each X it
    /// is drawn at: the absolute tolerance, and the relative one's share of
    /// the baseline there, which is the result less the difference. A
    /// relative band is drawn for a real signal on the result's own axis
    /// only; anything else would draw a band the comparison did not judge.
    ///
    /// The difference is evaluated, so its samples are in memory; the
    /// result's own samples are read off the frame by the reader service,
    /// and `None` says they have not arrived yet.
    fn read_band(
        &self,
        state: ResultsReads<'_>,
        signal: &str,
        expr: &ResolvedExpr,
        (fill, edge): (egui::Color32, egui::Color32),
    ) -> Option<Option<ToleranceBand>> {
        let Some(x) = expr.x.in_memory() else {
            return Some(None);
        };
        if self.relative_tolerance == 0.0 {
            let ends: Vec<f64> = [
                x.iter().find(|x| x.is_finite()),
                x.iter().rev().find(|x| x.is_finite()),
            ]
            .into_iter()
            .flatten()
            .copied()
            .collect();
            return Some(Some(ToleranceBand {
                half_width: vec![self.absolute_tolerance; ends.len()],
                x: ends,
                fill,
                edge,
            }));
        }
        let Some(own) = retained_run(state, self.candidate_id)
            .and_then(|run| self.analysis.resolve(run))
            .and_then(|(_, analysis)| find_waveform(&analysis.waveforms, signal).cloned())
        else {
            return Some(None);
        };
        let Some(difference) = expr.y.in_memory() else {
            return Some(None);
        };
        if own.complex.is_some() || own.y.len() != difference.len() || x.len() != difference.len() {
            return Some(None);
        }
        let own = match own.y.fetch_whole() {
            rspice_results::waveform::Fetched::Ready(own) => own,
            rspice_results::waveform::Fetched::Pending => return None,
            rspice_results::waveform::Fetched::Refused(_) => return Some(None),
        };
        let half_width = own
            .iter()
            .zip(difference.iter())
            .map(|(own, difference)| {
                self.absolute_tolerance + self.relative_tolerance * (own - difference).abs()
            })
            .collect();
        Some(Some(ToleranceBand {
            x: x.to_vec(),
            half_width,
            fill,
            edge,
        }))
    }
}

/// Whether a comparison has answered, whose differences can be stepped
/// through.
pub(crate) fn compared(state: ResultsReads<'_>) -> bool {
    state.results.last_comparison.is_some()
}

/// Draw the difference of each signal that parts from the baseline, every
/// compared signal when none does, on the result's strip.
pub(in super::super) fn plot_difference(state: &mut ResultsParts<'_>) -> Result<(), String> {
    let finished = state.results.last_comparison.clone().ok_or(NO_COMPARISON)?;
    let texts: Vec<String> = finished
        .differing()
        .into_iter()
        .filter(|signal| nameable(signal))
        .map(difference_text)
        .collect();
    if texts.is_empty() {
        return Err("None of the compared signals can be named in an expression.".to_owned());
    }
    for text in texts {
        super::super::super::waves::add_strip_expression(state, finished.analysis, text);
    }
    Ok(())
}

/// What the comparison that last answered marks on a pane of its result's
/// strip: the places its runs part, on a pane that draws a compared signal
/// or its difference, and the tolerance band of the first difference the
/// pane draws.
pub(crate) fn comparison_marks(
    state: &mut ResultsParts<'_>,
    model: &StripModel,
    pane: &UnitPane<'_>,
    expressions: &[ResolvedExpr],
    tokens: &Tokens,
) -> ComparisonMarks {
    let Some(finished) = state.results.last_comparison.clone() else {
        return ComparisonMarks::default();
    };
    if finished.analysis != model.analysis_key {
        return ComparisonMarks::default();
    }
    let signals = &finished.outcome.signals;
    let mut marked = vec![false; signals.len()];
    for trace in pane
        .traces
        .iter()
        .filter_map(|&index| model.traces.get(index))
        .filter(|trace| trace.visible && !trace.overlay)
    {
        if let Some(signal) = signals
            .iter()
            .position(|signal| signal.eq_ignore_ascii_case(&trace.source_waveform_name))
        {
            marked[signal] = true;
        }
    }
    let reads = state.results_reads();
    let texts = reads
        .results
        .session
        .analysis_exprs
        .get(&model.analysis_key);
    let mut tolerance = None;
    for expr in expressions {
        let Some(text) = texts
            .and_then(|texts| texts.get(expr.slot))
            .map(|trace| trace.text.as_str())
        else {
            continue;
        };
        let Some(signal) = signals
            .iter()
            .position(|signal| difference_text(signal) == text)
        else {
            continue;
        };
        marked[signal] = true;
        if tolerance.is_none() {
            tolerance = finished.band(
                reads,
                (text, &signals[signal]),
                expr,
                (
                    tokens.color.text_faint.gamma_multiply(0.14),
                    tokens.color.text_faint.gamma_multiply(0.7),
                ),
            );
        }
    }
    ComparisonMarks {
        places: finished
            .place_spans
            .iter()
            .zip(&marked)
            .filter(|(_, marked)| **marked)
            .map(|(places, _)| Arc::clone(places))
            .collect(),
        place_color: tokens.color.err.gamma_multiply(0.12),
        tolerance,
    }
}

/// Move the stepping cursor to the next or previous place the runs part by
/// more than the tolerance: onto the first sample of it outside the
/// tolerance.
pub(crate) fn step_difference(
    state: &mut ResultsParts<'_>,
    tokens: &Tokens,
    direction: Direction,
) -> Result<f64, String> {
    let finished = state.results.last_comparison.clone().ok_or(NO_COMPARISON)?;
    if finished.outcome.places.is_empty() {
        return Err("The runs part by no more than the tolerance anywhere.".to_owned());
    }
    let presentation = state.results_reads().presentation_policy();
    let models = super::super::super::waves::cached_models(
        &state.model.simulation,
        state.results,
        presentation.complex_number_display(),
        tokens,
    );
    let model = models
        .iter()
        .find(|model| model.analysis_key == finished.analysis)
        .ok_or("The compared result is not on the waveform sheet.")?;
    let from = stepping_from(&state.results.session, model);
    let firsts = finished
        .outcome
        .places
        .iter()
        .map(|place| place.first)
        .filter(|x| x.is_finite());
    let found = match direction {
        Direction::Next => firsts
            .filter(|x| from.is_none_or(|from| *x > from))
            .min_by(f64::total_cmp),
        Direction::Previous => firsts
            .filter(|x| from.is_none_or(|from| *x < from))
            .max_by(f64::total_cmp),
    };
    let x = found.ok_or(match direction {
        Direction::Next => "No place after the cursor parts by more than the tolerance.",
        Direction::Previous => "No place before the cursor parts by more than the tolerance.",
    })?;
    place_stepping_cursor(&mut state.results.session, model, x);
    Ok(x)
}
