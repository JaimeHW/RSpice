//! Source findings shared by compiler services and diagnostic views.

use std::sync::Arc;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum DiagnosticSeverity {
    #[serde(rename = "hint")]
    Hint,
    #[serde(rename = "information")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SourceDiagnostic {
    #[serde(flatten)]
    pub canonical: super::CanonicalDiagnosticMetadata,
    pub severity: DiagnosticSeverity,
    pub message: Arc<str>,
    #[serde(rename = "details")]
    pub detail: Arc<str>,
    /// Stable logical source identity for multi-file diagnostics.
    #[serde(rename = "logicalPath")]
    pub source_path: Option<Arc<str>>,
    /// Exact retained UTF-8 source paired with `source_path`.
    #[serde(rename = "sourceText")]
    pub source: Option<Arc<str>>,
    #[serde(rename = "byteRange")]
    pub byte_range: Option<std::ops::Range<usize>>,
    #[serde(rename = "sourceStartLine")]
    pub line: Option<usize>,
    #[serde(rename = "sourceStartColumn")]
    pub column: Option<usize>,
}

impl SourceDiagnostic {
    #[allow(
        clippy::too_many_arguments,
        reason = "Retain the complete diagnostic source location and identity in one record"
    )]
    pub fn current(
        producer: impl Into<String>,
        code: impl Into<String>,
        severity: DiagnosticSeverity,
        message: impl Into<Arc<str>>,
        detail: impl Into<Arc<str>>,
        source_path: Option<String>,
        source: Option<String>,
        byte_range: Option<std::ops::Range<usize>>,
        line: Option<usize>,
        column: Option<usize>,
    ) -> Self {
        let message = message.into();
        let source_path = source_path.map(Arc::<str>::from);
        let source = source.map(Arc::<str>::from);
        let document_id = source_path
            .clone()
            .unwrap_or_else(|| Arc::<str>::from("workspace"));
        let range = super::diagnostic_range(byte_range.as_ref(), line, column);
        Self {
            canonical: super::CanonicalDiagnosticMetadata::current(
                Arc::<str>::from(producer.into()),
                Arc::<str>::from(code.into()),
                document_id,
                range,
                0,
                "unbound-validation",
                message.as_ref(),
            ),
            severity,
            message,
            detail: detail.into(),
            source_path,
            source,
            byte_range,
            line,
            column,
        }
    }

    /// The producer's stable diagnostic code, as shown beside the message.
    ///
    /// A surface never composes this from a phase or a producer name: the code
    /// is identity owned by whatever raised the diagnostic.
    pub fn code(&self) -> &str {
        self.canonical.code.as_ref()
    }

    pub fn bind_validation(
        mut self,
        document_id: impl Into<Arc<str>>,
        revision: u64,
        validation_id: impl Into<String>,
    ) -> Self {
        let old = self.canonical;
        self.canonical = super::CanonicalDiagnosticMetadata::current(
            old.source.clone(),
            old.code.clone(),
            document_id,
            old.range,
            revision,
            Arc::<str>::from(validation_id.into()),
            self.message.as_ref(),
        );
        self.canonical.related_locations = old.related_locations;
        self.canonical.quick_fixes = old.quick_fixes;
        self.canonical.suppression = old.suppression;
        self.canonical.currentness = old.currentness;
        self.canonical.affected_consumers = old.affected_consumers;
        self
    }
}
