//! Compare the active result with a baseline dataset, signal by signal,
//! against an absolute and relative tolerance.

use egui::{Context, RichText, Sense, Ui, vec2};
use rspice_app_types::product::DatasetId;
use rspice_results::waveform_comparison::ComparisonAlignmentDraft;
use rspice_ui_kit::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{CommandForm, Dialog, DialogChoice, DialogSize, Problem, problem_line},
};

use super::labels_and_selection;

/// The comparison being set up, then its outcome once it has run.
#[derive(Debug, Clone, PartialEq)]
pub struct CompareDraft {
    pub baseline: Option<DatasetId>,
    pub alignment: ComparisonAlignmentDraft,
    /// The signal a threshold or correlation alignment reads.
    pub signal: String,
    pub threshold: String,
    pub maximum_lag: String,
    pub absolute_tolerance: String,
    pub relative_tolerance: String,
    /// Set once Compare has run; the form then shows the outcome.
    pub outcome: Option<CompareOutcome>,
}

impl Default for CompareDraft {
    fn default() -> Self {
        Self {
            baseline: None,
            alignment: ComparisonAlignmentDraft::AbsoluteXAxis,
            signal: String::new(),
            threshold: "0".to_owned(),
            maximum_lag: "128".to_owned(),
            absolute_tolerance: "0".to_owned(),
            relative_tolerance: "0".to_owned(),
            outcome: None,
        }
    }
}

/// What the form offers, resolved by the host against the retained datasets.
pub struct CompareChoices {
    /// The candidate: the result being compared.
    pub candidate: String,
    pub baselines: Vec<(DatasetId, String)>,
    /// Signals an alignment can be read from.
    pub signals: Vec<String>,
    /// Why Compare is refused, when it is.
    pub refusal: Option<String>,
}

/// One signal's verdict. Only a failure and a pass close to its limit are
/// marked; a pass is plain text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    NearLimit,
    Fail,
}

impl Verdict {
    /// A signal fails when any compared row is outside tolerance. It passes
    /// near its limit when its worst error used more than 80% of every
    /// tolerance in force: the absolute one, the relative one, or both.
    pub fn of(
        failed_rows: usize,
        maximum_absolute_error: f64,
        maximum_relative_error: f64,
        absolute_tolerance: f64,
        relative_tolerance: f64,
    ) -> Self {
        const NEAR: f64 = 0.8;
        if failed_rows > 0 {
            return Self::Fail;
        }
        let terms = [
            (maximum_absolute_error, absolute_tolerance),
            (maximum_relative_error, relative_tolerance),
        ];
        let in_force = terms.iter().filter(|(_, tolerance)| *tolerance > 0.0);
        let mut any = false;
        let mut all_near = true;
        for (error, tolerance) in in_force {
            any = true;
            all_near &= *error > NEAR * tolerance;
        }
        if any && all_near {
            Self::NearLimit
        } else {
            Self::Pass
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::NearLimit => "near limit",
            Self::Fail => "FAIL",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignalOutcome {
    pub name: String,
    pub compared_rows: usize,
    pub failed_rows: usize,
    /// The worst absolute error, formatted with its unit.
    pub maximum_error: String,
    pub verdict: Verdict,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompareOutcome {
    pub baseline: String,
    pub rows_compared: usize,
    pub signals: Vec<SignalOutcome>,
}

/// Show the form, or the outcome once Compare has run. `Primary` on the form
/// asks the host to run the comparison; on the outcome it closes.
pub fn show(ctx: &Context, draft: &mut CompareDraft, choices: &CompareChoices) -> DialogChoice {
    if let Some(outcome) = &draft.outcome {
        return Dialog::new("Comparison result", "Done")
            .size(DialogSize::CommandForm)
            .description("The per-signal outcome of comparing the result with its baseline.")
            .show(ctx, |ui| outcome_body(ui, &choices.candidate, outcome));
    }
    let refusal = choices.refusal.as_deref();
    CommandForm::new("Compare datasets", "Compare")
        .context(Some(choices.candidate.as_str()))
        .describe(
            "Compare the result with a baseline dataset, signal by signal, within a tolerance.",
        )
        .primary_enabled(refusal.is_none())
        .status(refusal.map_or(Ok(None), Err))
        .status_lines(2)
        .show(ctx, |rows| {
            let (labels, current) = labels_and_selection(&choices.baselines, draft.baseline);
            if let Some(index) =
                rows.select("Baseline", "rspice.compare.baseline", &current, &labels)
            {
                draft.baseline = Some(choices.baselines[index].0);
            }
            let alignments = [
                ComparisonAlignmentDraft::AbsoluteXAxis,
                ComparisonAlignmentDraft::FirstThresholdCrossing,
                ComparisonAlignmentDraft::CrossCorrelation,
            ];
            let mut alignment = alignments
                .iter()
                .position(|candidate| *candidate == draft.alignment)
                .unwrap_or(0);
            if rows.segmented(
                "Align by",
                "rspice.compare.alignment",
                &["X value", "Threshold", "Correlation"],
                &mut alignment,
            ) {
                draft.alignment = alignments[alignment];
            }
            if draft.alignment != ComparisonAlignmentDraft::AbsoluteXAxis {
                if let Some(index) = rows.select(
                    "Signal",
                    "rspice.compare.signal",
                    &draft.signal,
                    &choices.signals,
                ) {
                    draft.signal = choices.signals[index].clone();
                }
            }
            match draft.alignment {
                ComparisonAlignmentDraft::FirstThresholdCrossing => {
                    rows.text(
                        "Threshold",
                        egui::Id::new("rspice.compare.threshold"),
                        &mut draft.threshold,
                        "0.5",
                    );
                }
                ComparisonAlignmentDraft::CrossCorrelation => {
                    rows.text(
                        "Max lag",
                        egui::Id::new("rspice.compare.lag"),
                        &mut draft.maximum_lag,
                        "samples",
                    );
                }
                ComparisonAlignmentDraft::AbsoluteXAxis => {}
            }
            rows.text(
                "Abs tol",
                egui::Id::new("rspice.compare.absolute"),
                &mut draft.absolute_tolerance,
                "1m",
            );
            rows.text(
                "Rel tol",
                egui::Id::new("rspice.compare.relative"),
                &mut draft.relative_tolerance,
                "1e-3",
            );
            None
        })
}

/// Column widths of the outcome table after the signal column, which takes
/// what is left.
const ROWS_COL: f32 = 64.0;
const ERROR_COL: f32 = 88.0;
const VERDICT_COL: f32 = 72.0;
const ROW_H: f32 = 24.0;

fn outcome_body(ui: &mut Ui, candidate: &str, outcome: &CompareOutcome) {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing.y = 0.0;
    ui.label(
        RichText::new(format!("{candidate} against {}", outcome.baseline))
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(6.0);
    let failed = outcome
        .signals
        .iter()
        .filter(|signal| signal.verdict == Verdict::Fail)
        .count();
    if failed > 0 {
        problem_line(
            ui,
            Problem::Failure,
            &format!(
                "{failed} of {} signals outside tolerance",
                outcome.signals.len()
            ),
        );
        ui.add_space(6.0);
    }
    table_row(ui, ["Signal", "Rows", "Max error", "Verdict"], None, true);
    for signal in &outcome.signals {
        let rows = if signal.failed_rows > 0 {
            format!("{}/{}", signal.failed_rows, signal.compared_rows)
        } else {
            signal.compared_rows.to_string()
        };
        table_row(
            ui,
            [
                &signal.name,
                &rows,
                &signal.maximum_error,
                signal.verdict.label(),
            ],
            Some(signal.verdict),
            false,
        );
    }
}

/// One 24 pt row: signal left, numbers right, verdict right. A header row is
/// dimmed; a verdict cell takes the error or warning colour only for a
/// failure or a near-limit pass.
fn table_row(ui: &mut Ui, cells: [&str; 4], verdict: Option<Verdict>, header: bool) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(width, ROW_H), Sense::hover());
    let painter = ui.painter();
    let font = if header {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    };
    let base = if header {
        t.color.text_dim
    } else {
        t.color.text
    };
    let verdict_color = match verdict {
        Some(Verdict::Fail) => t.color.err,
        Some(Verdict::NearLimit) => t.color.warn,
        Some(Verdict::Pass) | None => base,
    };
    let right = rect.right();
    // Right edges of the rows, error and verdict cells.
    let columns = [
        (right - VERDICT_COL - ERROR_COL, egui::Align2::RIGHT_CENTER),
        (right - VERDICT_COL, egui::Align2::RIGHT_CENTER),
        (right, egui::Align2::RIGHT_CENTER),
    ];
    let name_rect = egui::Rect::from_min_max(
        rect.left_top(),
        egui::pos2(columns[0].0 - ROWS_COL, rect.bottom()),
    );
    painter.with_clip_rect(name_rect).text(
        egui::pos2(rect.left(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        cells[0],
        if header {
            font.clone()
        } else {
            theme::sans(tokens::FS_0, FontWeight::Regular)
        },
        base,
    );
    for (index, (x, align)) in columns.into_iter().enumerate() {
        let color = if index == 2 { verdict_color } else { base };
        painter.text(
            egui::pos2(x, rect.center().y),
            align,
            cells[index + 1],
            font.clone(),
            color,
        );
    }
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
}

#[cfg(test)]
mod tests {
    use super::Verdict;

    #[test]
    fn only_failures_and_near_limit_passes_are_marked() {
        assert_eq!(Verdict::of(1, 0.0, 0.0, 1.0, 0.0), Verdict::Fail);
        assert_eq!(Verdict::of(0, 0.5e-3, 0.0, 1e-3, 0.0), Verdict::Pass);
        assert_eq!(Verdict::of(0, 0.9e-3, 0.0, 1e-3, 0.0), Verdict::NearLimit);
        // With both tolerances in force, a pass is near its limit only when
        // the worst error used most of each.
        assert_eq!(Verdict::of(0, 0.9e-3, 0.1e-3, 1e-3, 1e-3), Verdict::Pass);
        assert_eq!(
            Verdict::of(0, 0.9e-3, 0.9e-3, 1e-3, 1e-3),
            Verdict::NearLimit
        );
        // An exact comparison that passed had no error to be near anything.
        assert_eq!(Verdict::of(0, 0.0, 0.0, 0.0, 0.0), Verdict::Pass);
    }
}
