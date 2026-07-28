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
    pub fn evaluate(&mut self, simulation: &SimulationState) {
        let text = self.expression.trim();
        if text.is_empty() {
            self.outcome = None;
            return;
        }
        let expr = parser::parse(text);
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
