//! A Verify cell's verdict over the kit's shared table: the word a pass/fail
//! column states and the tone a judged value is inked in.
//!
//! The table itself knows tones, not verdicts. Verify's columns and its
//! judgement of a value stay here, so the kit names nothing of verification.

use crate::ui::widgets::table::{Cell, Tone};

/// A result's judgment of one value.
///
/// A failure is painted in the error tone and a value near its limit in the
/// warning tone. A pass is neutral: success is the absence of a problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Verdict {
    Pass,
    Near,
    Fail,
}

impl Verdict {
    /// The word a pass/fail column states.
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Pass => "Pass",
            Self::Near => "Near",
            Self::Fail => "Fail",
        }
    }

    /// The tone the verdict inks a value in, which a status sort orders by:
    /// a failure before a near miss before a pass.
    const fn tone(self) -> Tone {
        match self {
            Self::Pass => Tone::Neutral,
            Self::Near => Tone::Caution,
            Self::Fail => Tone::Error,
        }
    }
}

/// A verdict on the kit's cell.
pub(super) trait VerdictCell {
    /// A pass/fail column's word for `verdict`.
    fn verdict(verdict: Verdict) -> Self;
    /// The same words, judged: coloured only when the verdict is not a pass.
    fn judged(self, verdict: Verdict) -> Self;
}

impl VerdictCell for Cell {
    fn verdict(verdict: Verdict) -> Self {
        Self::text(verdict.label()).judged(verdict)
    }

    fn judged(self, verdict: Verdict) -> Self {
        self.toned(verdict.tone())
    }
}
