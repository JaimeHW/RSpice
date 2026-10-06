//! Diagnostics resolved against the original preprocessing dependency closure.

use crate::preprocessor::{PreprocessedDependency, PreprocessedSource};
use crate::{
    CompileDiagnosticPhase, CompileDiagnosticSeverity, CompileDiagnosticSpan, CompileError,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A compiler diagnostic with locations in the source the author supplied.
///
/// Byte ranges are half-open UTF-8 ranges in `path`, not in expanded compiler
/// input. Lines and Unicode scalar columns are one-based. When preprocessing
/// changed a line (for example, a macro invocation), its entire original line
/// is selected instead of presenting an expanded token as an authored range.
/// Diagnostics without a source span retain `None` locations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceCompileDiagnostic {
    pub severity: CompileDiagnosticSeverity,
    pub phase: CompileDiagnosticPhase,
    pub code: String,
    pub message: String,
    /// Display path, with non-Unicode filesystem names represented lossily.
    pub path: Option<String>,
    pub byte_start: Option<usize>,
    pub byte_end: Option<usize>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

impl std::fmt::Display for SourceCompileDiagnostic {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity = match self.severity {
            CompileDiagnosticSeverity::Error => "Error",
            CompileDiagnosticSeverity::Warning => "Warning",
        };
        write!(formatter, "{severity}: ")?;
        if let Some(path) = &self.path {
            formatter.write_str(path)?;
        }
        if let Some(line) = self.line {
            write!(formatter, ":{line}")?;
            if let Some(column) = self.column {
                write!(formatter, ":{column}")?;
            }
        }
        if self.path.as_ref().is_some_and(|path| !path.is_empty()) || self.line.is_some() {
            formatter.write_str(": ")?;
        }
        write!(formatter, "[{}] {}", self.code, self.message)
    }
}

impl From<&crate::PreprocessorError> for SourceCompileDiagnostic {
    fn from(error: &crate::PreprocessorError) -> Self {
        Self {
            severity: CompileDiagnosticSeverity::Error,
            phase: CompileDiagnosticPhase::Input,
            code: if error.cancelled {
                "VA-INPUT-CANCELLED"
            } else if error.resource_limit.is_some() {
                "VA-INPUT-RESOURCE-LIMIT"
            } else if error.io_error.is_some() {
                "VA-INPUT-IO"
            } else {
                "VA-INPUT-PREPROCESS"
            }
            .into(),
            message: error.message.clone(),
            path: error.file.as_ref().map(|path| path.display().to_string()),
            byte_start: None,
            byte_end: None,
            line: (error.line > 0).then_some(error.line),
            column: None,
        }
    }
}

pub(crate) fn provider_diagnostics(
    error: &CompileError,
    preprocessed: &PreprocessedSource,
    dependencies: &[PreprocessedDependency],
) -> Vec<SourceCompileDiagnostic> {
    map_diagnostics(
        crate::compile_diagnostics(&preprocessed.source, error),
        preprocessed,
        dependencies,
    )
}

pub(crate) fn map_diagnostics(
    diagnostics: Vec<crate::CompileDiagnostic>,
    preprocessed: &PreprocessedSource,
    dependencies: &[PreprocessedDependency],
) -> Vec<SourceCompileDiagnostic> {
    map_diagnostics_with_sources(diagnostics, preprocessed, |path| {
        dependencies
            .iter()
            .find(|dependency| dependency.logical_path == path)
            .map(|dependency| dependency.source.as_str())
    })
}

pub(crate) fn map_diagnostics_with_sources<'a>(
    diagnostics: Vec<crate::CompileDiagnostic>,
    preprocessed: &'a PreprocessedSource,
    source_for_path: impl Fn(&Path) -> Option<&'a str>,
) -> Vec<SourceCompileDiagnostic> {
    diagnostics
        .into_iter()
        .map(|diagnostic| {
            let mapped = diagnostic
                .span
                .as_ref()
                .and_then(|span| preprocessed.map_span(span, &source_for_path));
            SourceCompileDiagnostic {
                severity: diagnostic.severity,
                phase: diagnostic.phase,
                code: diagnostic.code,
                message: diagnostic.message,
                path: mapped.as_ref().map(|span| span.path.display().to_string()),
                byte_start: mapped.as_ref().map(|span| span.byte_start),
                byte_end: mapped.as_ref().map(|span| span.byte_end),
                line: mapped.as_ref().map(|span| span.line),
                column: mapped.as_ref().map(|span| span.column),
            }
        })
        .collect()
}

/// Borrows the exact source snapshot, so editor consumers can authenticate it
/// while filesystem consumers need not clone entire documents into each error.
pub(crate) struct MappedSourceSpan<'a> {
    pub path: &'a Path,
    pub source: &'a str,
    pub byte_start: usize,
    pub byte_end: usize,
    pub line: usize,
    pub column: usize,
}

impl PreprocessedSource {
    pub(crate) fn map_span<'a>(
        &'a self,
        span: &CompileDiagnosticSpan,
        source_for_path: impl FnOnce(&Path) -> Option<&'a str>,
    ) -> Option<MappedSourceSpan<'a>> {
        let offset = usize::try_from(span.byte_start).ok()?;
        let segment = self.segment_at(offset)?;
        let source = source_for_path(&segment.logical_path)?;
        let line_range = source_line_range(source, segment.source_line)?;
        let expanded_line = &self.source[segment.expanded_start..segment.expanded_end];
        let original_line = &source[line_range.clone()];
        let exact_line = expanded_line.trim_end_matches(['\r', '\n'])
            == original_line.trim_end_matches(['\r', '\n']);
        let local_start = offset.saturating_sub(segment.expanded_start);
        let local_end = usize::try_from(span.byte_end)
            .ok()?
            .saturating_sub(segment.expanded_start);
        let start = line_range
            .start
            .saturating_add(local_start)
            .min(line_range.end);
        let end = line_range
            .start
            .saturating_add(local_end)
            .min(line_range.end)
            .max(start);
        let (byte_start, byte_end, column) =
            if exact_line && source.is_char_boundary(start) && source.is_char_boundary(end) {
                (
                    start,
                    end,
                    source[line_range.start..start].chars().count() + 1,
                )
            } else {
                (line_range.start, line_range.end, 1)
            };
        Some(MappedSourceSpan {
            path: &segment.logical_path,
            source,
            byte_start,
            byte_end,
            line: segment.source_line,
            column,
        })
    }
}

pub(crate) fn source_line_range(
    source: &str,
    one_based_line: usize,
) -> Option<std::ops::Range<usize>> {
    if one_based_line == 0 {
        return None;
    }
    let mut start = 0usize;
    for (index, line) in source.split_inclusive('\n').enumerate() {
        let end = start.saturating_add(line.len());
        if index + 1 == one_based_line {
            return Some(start..end);
        }
        start = end;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{ParseError, ParseErrorKind};
    use crate::source::{SourceId, Span};
    use crate::virtual_source::VirtualBundleProvider;
    use crate::{Preprocessor, VirtualCompileLimits, VirtualSourceBundle};

    #[test]
    fn collected_diagnostics_keep_each_original_document_and_unicode_position() {
        let first = "// header\r\n\u{3b1} @\r\n";
        let second = "`define BAD #\n`BAD\n";
        let bundle = VirtualSourceBundle::from_sources(
            "root.va",
            [
                ("root.va", "`include \"first.va\"\n`include \"second.va\"\n"),
                ("first.va", first),
                ("second.va", second),
            ],
        )
        .unwrap();
        let provider = VirtualBundleProvider::new(&bundle, VirtualCompileLimits::default());
        let mut pp = Preprocessor::new();
        let preprocessed = pp
            .preprocess_provider_root_mapped(&provider, Path::new("root.va"))
            .unwrap();
        let [first_error, second_error] = ['@', '#'].map(|token| {
            let offset = preprocessed.source.find(token).unwrap() as u32;
            CompileError::Parser(ParseError::new(
                ParseErrorKind::InvalidExpression,
                Span::new(SourceId::new(0), offset, offset + 1),
            ))
        });
        let error = CompileError::Multiple(vec![
            first_error,
            CompileError::Multiple(vec![second_error]),
            CompileError::ModuleSelection("missing module".into()),
        ]);
        let diagnostics = provider_diagnostics(&error, &preprocessed, pp.dependency_documents());
        assert_eq!(diagnostics.len(), 3);
        let exact = &diagnostics[0];
        assert_eq!(exact.path.as_deref(), Some("first.va"));
        assert_eq!(exact.line, Some(2));
        assert_eq!(exact.column, Some(3));
        assert_eq!(
            &first[exact.byte_start.unwrap()..exact.byte_end.unwrap()],
            "@"
        );
        let expanded = &diagnostics[1];
        assert_eq!(expanded.path.as_deref(), Some("second.va"));
        assert_eq!(expanded.line, Some(2));
        assert_eq!(expanded.column, Some(1));
        assert_eq!(
            &second[expanded.byte_start.unwrap()..expanded.byte_end.unwrap()],
            "`BAD\n"
        );
        assert_eq!(
            diagnostics[2].phase,
            CompileDiagnosticPhase::ModuleSelection
        );
        assert!(diagnostics[2].path.is_none());
        assert!(diagnostics[2].line.is_none());
        assert!(diagnostics[2].byte_start.is_none());
    }
}
