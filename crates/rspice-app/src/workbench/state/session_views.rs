//! The parameter tuner's candidate drafts, which a design instance's Value row
//! stages for the Verify workspace's tuning sandbox.
//!
//! They are proposals held for the active session only: the design variables
//! and the plan stay authoritative until the tuning review commits the whole
//! candidate set, and nothing here is durable.

/// Runtime-only candidate value owned by the non-destructive parameter tuner.
/// The persisted `DesignVariable` remains authoritative until the user commits
/// the complete candidate set as one simulation-plan revision.
#[derive(Debug, Clone, PartialEq)]
pub struct TuningVariableDraft {
    pub variable_id: crate::product::DesignVariableId,
    pub baseline_expression: String,
    pub candidate_expression: String,
    pub validation_error: Option<String>,
    /// A proposed variable does not exist in the authoritative plan yet.
    /// Its candidate expression is still initialized from the selected
    /// instance literal, so this bit participates in dirty detection.
    pub proposed: bool,
}

impl TuningVariableDraft {
    pub fn is_dirty(&self) -> bool {
        self.proposed || self.candidate_expression.trim() != self.baseline_expression.trim()
    }
}

/// Runtime-only bridge between a selected schematic instance and the tuning
/// sandbox.  The proposed variable and binding are deliberately kept outside
/// both authoritative stores until the review dialog commits them together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuningInstanceBindingDraft {
    pub component_id: u64,
    pub component_name: String,
    pub source_view: crate::state::CellViewRef,
    pub source_topology_version: u64,
    pub source_value: String,
    pub binding_expression: String,
    pub variable: crate::state::DesignVariable,
    pub creates_variable: bool,
}

impl TuningInstanceBindingDraft {
    #[must_use]
    pub fn requires_schematic_edit(&self) -> bool {
        self.source_value.trim() != self.binding_expression
    }
}
