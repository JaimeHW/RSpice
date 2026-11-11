//! The last outcome any plan command announced, and the two operations
//! that announce one.
//!
//! The outcome is the model's because the commands that produce it are model
//! transactions on the plan, and more than one feature commits them: the
//! Simulation Studio's rows and dialogs, the toolbar's corner selector and
//! the schematic's probe all hand back a receipt. They announce it here, in
//! the same synchronous step as the commit.

use super::Model;

/// Whether a Simulation Studio plan command changed the plan or was refused.
///
/// A receipt and a refusal are told apart here rather than by reading the
/// message, because the two are shown differently and only one of them has to
/// follow the reader off the Analyses page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum AnalysisLifecycleSeverity {
    /// Everything the command asked for was committed, and the message is the
    /// transaction's receipt.
    #[default]
    Receipt,
    /// Something the command asked for was refused, and the message names what
    /// and why. A command that commits one step and is refused the next is a
    /// refusal: the reader has to be told about the part that did not happen.
    Refusal,
}

/// The last outcome any Simulation Studio plan command announced.
///
/// Only the Analyses route draws this; the Console holds the record of each
/// outcome. An outcome is new only when it *changes*, not on every
/// restatement: three of the writers sit on the render path rather than in a
/// click handler, and recording each restatement would record the same
/// refusal forever.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisLifecycleOutcome {
    severity: AnalysisLifecycleSeverity,
    message: String,
}

impl Default for AnalysisLifecycleOutcome {
    /// The field is runtime-only, so a restored session gets this rather than
    /// the outcome it was saved with. It has to read as a real status line.
    fn default() -> Self {
        Self {
            severity: AnalysisLifecycleSeverity::Receipt,
            message: "No lifecycle command has been committed this session.".to_owned(),
        }
    }
}

impl AnalysisLifecycleOutcome {
    pub fn message(&self) -> &str {
        &self.message
    }

    pub const fn is_refusal(&self) -> bool {
        matches!(self.severity, AnalysisLifecycleSeverity::Refusal)
    }

    /// Announce a committed change. Returns whether this is a new outcome, so
    /// that a reporter which also records it elsewhere records it once per
    /// event.
    pub fn record_receipt(&mut self, message: impl Into<String>) -> bool {
        self.record(AnalysisLifecycleSeverity::Receipt, message)
    }

    /// Announce a command that changed nothing, and why. Returns whether this
    /// is a new outcome, as [`Self::record_receipt`] does.
    pub fn record_refusal(&mut self, message: impl Into<String>) -> bool {
        self.record(AnalysisLifecycleSeverity::Refusal, message)
    }

    /// Whether the announced outcome differs from the one already held.
    ///
    /// Three announcing sites sit on the render path rather than in a click
    /// handler, so a standing refusal is restated every frame. Restating it is
    /// not an event, and a reporter reading the answer does not report it
    /// again.
    fn record(&mut self, severity: AnalysisLifecycleSeverity, message: impl Into<String>) -> bool {
        let message = message.into();
        if self.severity == severity && self.message == message {
            return false;
        }
        self.severity = severity;
        self.message = message;
        true
    }
}

impl Model {
    /// Announce a committed plan command, and keep its receipt in the log as
    /// a plan entry when it is a new outcome.
    ///
    /// The Analyses route's status line reads the outcome, and the Console
    /// reads the log afterwards. [`LogSource::Plan`] keeps the second from
    /// becoming noise: the activity mirror never lifts a `Plan` entry, so a
    /// receipt cannot turn a routine registry edit into a notification. A
    /// standing refusal restated every frame is not a new outcome, so two
    /// consecutive identical receipts leave one record.
    ///
    /// [`LogSource::Plan`]: crate::diagnostics::LogSource::Plan
    pub fn record_plan_receipt(&mut self, message: impl Into<String>) {
        let message = message.into();
        if self
            .analysis_lifecycle_status
            .record_receipt(message.as_str())
        {
            self.log_buffer.log(
                crate::diagnostics::LogSeverity::Info,
                crate::diagnostics::LogSource::Plan,
                message,
            );
        }
    }

    /// Announce a refused plan command, as [`Self::record_plan_receipt`]
    /// does, at warning severity. Returns the sentence to answer the gesture
    /// with on the status bar when the outcome is new.
    ///
    /// The status line is drawn only on the Analyses route, and a plan
    /// command can be refused from the plan manager in any workspace, so the
    /// prompt is what answers the reader wherever they are. It is due with
    /// the record, when the outcome changes, because a standing refusal
    /// restated every frame would otherwise hold the prompt up forever.
    pub fn record_plan_refusal(&mut self, message: impl Into<String>) -> Option<String> {
        let message = message.into();
        if !self
            .analysis_lifecycle_status
            .record_refusal(message.as_str())
        {
            return None;
        }
        self.log_buffer.log(
            crate::diagnostics::LogSeverity::Warning,
            crate::diagnostics::LogSource::Plan,
            message.clone(),
        );
        Some(message)
    }
}
