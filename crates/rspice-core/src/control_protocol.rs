//! Shared protocol between the control-session machine and its command host.
//!
//! The execution layer produces commands through this protocol and the engine
//! consumes them. Keeping the vocabulary below both sides prevents either one
//! from owning an interface the other must reach upward to use.

use crate::Value;
use crate::netlist::expr::ParamContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlErrorKind {
    Syntax,
    Expression,
    ResourceLimit,
    Aborted,
    Host,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlError {
    pub line: usize,
    pub kind: ControlErrorKind,
    pub message: String,
}

impl ControlError {
    pub(crate) fn new(line: usize, kind: ControlErrorKind, message: impl Into<String>) -> Self {
        Self {
            line,
            kind,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ControlError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "control line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for ControlError {}

/// One command for the analysis/output host, after variable substitution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlCommand {
    pub line: usize,
    pub name: String,
    pub arguments: String,
}

/// Hosts may extend scalar evaluation with values from completed datasets.
pub trait ControlScalarEvaluator {
    fn evaluate_scalar(
        &mut self,
        expression: &str,
        variables: &ParamContext,
        line: usize,
    ) -> Result<Value, ControlError>;
}
