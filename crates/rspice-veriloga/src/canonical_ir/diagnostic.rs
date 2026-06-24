use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpanRef {
    pub source_file_id: u32,
    pub start: u32,
    pub end: u32,
}

impl From<crate::source::Span> for SourceSpanRef {
    fn from(value: crate::source::Span) -> Self {
        Self {
            source_file_id: value.source.raw(),
            start: value.start,
            end: value.end,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompilerPhase {
    HirLowering,
    HirValidation,
    MirLowering,
    MirValidation,
    OptLowering,
    OptValidation,
    Scheduling,
    Artifact,
    BackendLowering,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IrDiagnostic {
    pub severity: DiagnosticSeverity,
    pub phase: CompilerPhase,
    pub message: String,
    pub span: Option<SourceSpanRef>,
}

impl IrDiagnostic {
    pub fn error(phase: CompilerPhase, message: impl Into<String>, span: SourceSpanRef) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            phase,
            message: message.into(),
            span: Some(span),
        }
    }

    pub fn global_error(phase: CompilerPhase, message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            phase,
            message: message.into(),
            span: None,
        }
    }
}

impl fmt::Display for IrDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(span) = self.span {
            write!(
                f,
                "{:?} {:?} at {}:{}-{}: {}",
                self.severity, self.phase, span.source_file_id, span.start, span.end, self.message
            )
        } else {
            write!(
                f,
                "{:?} {:?} at global: {}",
                self.severity, self.phase, self.message
            )
        }
    }
}

pub type IrValidationResult = Result<(), Vec<IrDiagnostic>>;
