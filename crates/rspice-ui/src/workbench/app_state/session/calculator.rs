//! Waveform calculator session state.
//!
//! The expression, its recall history, and the last evaluation outcome.
//! `AppState` owns this so the calculator keeps its place while the user
//! works elsewhere; the floating tool that renders it lives at
//! `workbench::tools::calculator_tool`, which reaches down here for the
//! model and hangs its egui half off this type as a second inherent impl.

use crate::analysis::calculator::{CalcValue, SimulationContext, evaluator, parser};
use crate::state::SimulationState;
use crate::ui::plot::fmt_si_significant;

/// How many significant digits the readout shows.
///
/// Five was not enough to tell two nearby operating points apart, which is
/// most of what a calculator over simulation results is asked to do. Eight
/// is the point past which a `f64`'s decimal digits stop meaning anything a
/// reader can act on — the exact value is one click away for the rest.
const READOUT_DIGITS: usize = 8;

/// A successful evaluation: the rounded readout, and the exact number behind
/// it.
///
/// Both, because they answer different questions. The readout is for reading;
/// the `f64` is what click-to-copy puts on the clipboard, so a number taken
/// from here can be pasted somewhere that needs every bit of it.
#[derive(Clone, Debug, PartialEq)]
pub(in crate::workbench) struct CalcResult {
    /// What the readout row shows.
    pub(in crate::workbench) readout: String,
    /// The exact value the readout rounds.
    pub(in crate::workbench) exact: f64,
    /// Whether `exact` is the whole result or the last sample of a series,
    /// so the copy affordance can say which it is handing over.
    pub(in crate::workbench) exact_is_last_sample: bool,
}

impl CalcResult {
    /// Scientific notation carrying enough digits to round-trip the `f64`,
    /// matching how the Results workspace spells an exact retained value.
    pub(in crate::workbench) fn exact_text(&self) -> String {
        format!("{:.17e}", self.exact)
    }
}

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
    pub(in crate::workbench) outcome: Option<Result<CalcResult, String>>,
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
            Ok(CalcValue::Scalar(value)) => Ok(CalcResult {
                readout: format!("= {}", fmt_si_significant(value, "", READOUT_DIGITS)),
                exact: value,
                exact_is_last_sample: false,
            }),
            Ok(CalcValue::Waveform(x, y)) => {
                let last = y.last().copied().unwrap_or(f64::NAN);
                Ok(CalcResult {
                    readout: format!(
                        "= waveform · {} pts · last {}",
                        x.len(),
                        fmt_si_significant(last, "", READOUT_DIGITS)
                    ),
                    exact: last,
                    exact_is_last_sample: true,
                })
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
                Ok(value) => panic!(
                    "{text:?} produced a result instead of an error: {}",
                    value.readout
                ),
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

    #[test]
    fn the_readout_carries_eight_significant_digits() {
        let simulation = simulation_with_a_ramp();
        let mut panel = CalculatorPanel::new();
        panel.expression = "1/3".to_owned();
        panel.evaluate(&simulation);

        let result = match panel.outcome.as_ref().expect("evaluated") {
            Ok(value) => value.clone(),
            Err(error) => panic!("1/3 failed: {error}"),
        };
        assert_eq!(result.readout, "= 333.33333 m", "eight significant digits");
    }

    #[test]
    fn the_readout_keeps_the_exact_value_for_copying() {
        let simulation = simulation_with_a_ramp();

        // A scalar result copies the result itself.
        let mut panel = CalculatorPanel::new();
        panel.expression = "1/3".to_owned();
        panel.evaluate(&simulation);
        let result = panel.outcome.as_ref().expect("evaluated").clone().expect("1/3");
        assert!(!result.exact_is_last_sample);
        assert_eq!(result.exact, 1.0 / 3.0);
        assert_eq!(
            result.exact_text().parse::<f64>().expect("parses back"),
            1.0 / 3.0,
            "the copied text must round-trip the f64 the readout rounded"
        );
        assert_ne!(
            result.exact_text(),
            result.readout,
            "the readout is rounded; the copy is not"
        );

        // A series result copies its last sample, and says so.
        let mut panel = CalculatorPanel::new();
        panel.expression = "V(out) / 3".to_owned();
        panel.evaluate(&simulation);
        let result = panel
            .outcome
            .as_ref()
            .expect("evaluated")
            .clone()
            .expect("V(out) / 3");
        assert!(result.exact_is_last_sample);
        assert_eq!(result.exact, 2.0 / 3.0, "last sample of the ramp over three");
        assert_eq!(
            result.exact_text().parse::<f64>().expect("parses back"),
            2.0 / 3.0
        );
    }
}
