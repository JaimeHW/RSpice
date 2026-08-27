//! Waveform calculator session state.
//!
//! The expression, its recall history, and the last evaluation outcome.
//! `AppState` owns this so the calculator keeps its place while the user
//! works elsewhere; the floating tool that renders it lives at
//! `workbench::tools::calculator_tool`, which reaches down here for the
//! model and hangs its egui half off this type as a second inherent impl.

use crate::analysis::calculator::{CalcValue, SimulationContext, evaluator, parser};
use crate::state::SimulationState;
use crate::ui::plot::fmt_si;

#[derive(Default, Clone)]
pub struct CalculatorPanel {
    /// Current expression text.
    pub expression: String,
    /// Successful expressions, most recent first.
    pub(in crate::workbench) history: Vec<String>,
    /// Position while cycling history with ↑/↓ (None = live text).
    pub(in crate::workbench) history_at: Option<usize>,
    /// Live text stashed while cycling history.
    pub(in crate::workbench) stash: String,
    /// Last evaluation outcome.
    pub(in crate::workbench) outcome: Option<Result<String, String>>,
    /// Selected function category.
    pub(in crate::workbench) category: FunctionCategory,
    /// Signal list filter.
    pub(in crate::workbench) signal_filter: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub(in crate::workbench) enum FunctionCategory {
    #[default]
    Math,
    Signal,
    Measure,
}

impl CalculatorPanel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Footer hint for the dialog: what the expression evaluates against.
    pub fn context_hint(&self, simulation: &SimulationState) -> String {
        match simulation.active_run() {
            Some(run) => format!("evaluates against run #{}", run.id),
            None => "no run yet — signals resolve after a simulation".to_owned(),
        }
    }

    /// Evaluate the current expression against the live waveform set.
    ///
    /// A syntax error is an error, never a number. The parser used to
    /// recover a malformed expression to the literal `0`, so a typo evaluated
    /// cleanly, printed `= 0.0000` in the success colour, and joined the
    /// recall history — a wrong answer indistinguishable from a right one.
    pub fn evaluate(&mut self, simulation: &SimulationState) {
        let text = self.expression.trim();
        if text.is_empty() {
            self.outcome = None;
            return;
        }
        let expr = match parser::try_parse(text) {
            Ok(expr) => expr,
            Err(error) => {
                self.outcome = Some(Err(format!("syntax error: {error}")));
                return;
            }
        };
        let ctx = SimulationContext::new(simulation);
        self.outcome = Some(match evaluator::evaluate(&expr, &ctx) {
            Ok(CalcValue::Scalar(value)) => Ok(format!("= {}", fmt_si(value, "", 4))),
            Ok(CalcValue::Waveform(x, y)) => {
                let last = y.last().copied().unwrap_or(0.0);
                Ok(format!(
                    "= waveform · {} pts · last {}",
                    x.len(),
                    fmt_si(last, "", 3)
                ))
            }
            Err(error) => Err(error.to_string()),
        });
        if matches!(self.outcome, Some(Ok(_))) {
            let owned = text.to_owned();
            self.history.retain(|h| h != &owned);
            self.history.insert(0, owned);
            self.history.truncate(16);
        }
    }

    /// Clear the editor and outcome.
    pub fn clear(&mut self) {
        self.expression.clear();
        self.outcome = None;
        self.history_at = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WaveformData;

    fn simulation_with_a_ramp() -> SimulationState {
        let mut simulation = SimulationState::default();
        simulation.waveforms = vec![WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![0.0, 1.0, 2.0],
            "#00aaff",
        )];
        simulation
    }

    #[test]
    fn a_syntax_error_never_reports_a_numeric_result() {
        let simulation = simulation_with_a_ramp();
        for text in ["V(out) +", "avg(V(out)", "1 2", "V()", "*3"] {
            let mut panel = CalculatorPanel::new();
            panel.expression = text.to_owned();
            panel.evaluate(&simulation);

            let outcome = panel.outcome.as_ref().expect("evaluation reports something");
            let message = match outcome {
                Ok(value) => panic!("{text:?} produced a result instead of an error: {value}"),
                Err(message) => message.clone(),
            };
            assert!(
                !message.contains('='),
                "{text:?} error must not read as a result: {message}"
            );
            assert!(
                panel.history.is_empty(),
                "{text:?} must not enter the recall history"
            );
        }
    }
}
