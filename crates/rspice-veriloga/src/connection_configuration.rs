//! Immutable design-level connection selection, replayed before elaboration.

use crate::ast::{
    DefaultDisciplineDirective, DefaultTransitionDirective, Expression, Item, NumberLit,
};
use crate::disciplines::DisciplineDb;
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::metrics::MetricsRecorder;
use crate::semantic::{AnalyzedFile, SemanticAnalyzer};
use crate::source::{SourceId, Span};
use crate::{ConnectionLibraryArtifact, Lexer, Parser, PipelinePhase};
use serde::{Deserialize, Serialize};

/// An exact connection library and one case-sensitive `connectrules` block.
///
/// This is independent of the device source identity. Canonical artifacts retain
/// both sources so cached execution and parameter specialization use the same
/// selection without reopening a file or consulting ambient configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectionConfiguration {
    library: ConnectionLibraryArtifact,
    block: String,
}

impl ConnectionConfiguration {
    pub fn new(library: ConnectionLibraryArtifact, block: &str) -> Result<Self, String> {
        library
            .connect_specification()?
            .rules
            .select_block(block)
            .map_err(|error| error.to_string())?;
        Ok(Self::selected(library, block))
    }

    pub(crate) fn selected(library: ConnectionLibraryArtifact, block: &str) -> Self {
        Self {
            library,
            block: block.to_owned(),
        }
    }

    pub fn library(&self) -> &ConnectionLibraryArtifact {
        &self.library
    }
    pub fn block(&self) -> &str {
        &self.block
    }

    pub fn identity(&self) -> [u8; 32] {
        let mut hash = blake3::Hasher::new();
        hash.update(b"rspice.connection-configuration\0");
        hash.update(self.library.identity());
        hash.update(&(self.block.len() as u64).to_le_bytes());
        hash.update(self.block.as_bytes());
        *hash.finalize().as_bytes()
    }

    pub fn validate_integrity(&self) -> Result<(), String> {
        self.library.validate_integrity()?;
        if self.block.is_empty() || self.block.contains('\0') {
            return Err(
                "connection configuration must select a nonempty rule block without NUL".into(),
            );
        }
        Ok(())
    }

    pub(crate) fn apply(
        &self,
        source: &str,
        analyzed: &AnalyzedFile,
        measurements: &mut MetricsRecorder,
    ) -> CompileResult<AnalyzedFile> {
        self.validate_integrity()
            .map_err(CompileError::ModuleSelection)?;
        measurements.checkpoint(PipelinePhase::Semantic)?;
        let started = web_time::Instant::now();
        let mut configured = if source == self.library.preprocessed_source() {
            // Keep effective top-level parameter values already applied to this
            // AST. Re-parsing the library would reset those overrides.
            let mut configured = analyzed.clone();
            configured.connect_rules = analyzed
                .connect_rules
                .select_block(&self.block)
                .map_err(|error| CompileError::ModuleSelection(error.to_string()))?;
            retain_selected_rules(&mut configured.source.items, &self.block);
            configured
        } else {
            measurements.checkpoint(PipelinePhase::Lex)?;
            let tokens = Lexer::new(self.library.preprocessed_source(), SourceId::new(1))
                .collect_tokens()?;
            measurements.checkpoint(PipelinePhase::Parse)?;
            let mut library = Parser::new(&tokens).parse()?;
            measurements.checkpoint(PipelinePhase::Semantic)?;
            let library_analysis = SemanticAnalyzer::new().analyze(&library)?;
            library_analysis
                .connect_rules
                .select_block(&self.block)
                .map_err(|error| CompileError::ModuleSelection(error.to_string()))?;
            compatible_physics(&analyzed.disciplines, &library_analysis.disciplines)?;
            retain_selected_rules(&mut library.items, &self.block);
            let mut combined = analyzed.source.clone();
            // The selected external library owns connection declarations. The
            // device retains its ordinary hierarchy, physical definitions and
            // effective parameter overrides.
            combined
                .items
                .retain(|item| !matches!(item, Item::ConnectModule(_) | Item::ConnectRules(_)));
            let reset = Span::new(SourceId::new(1), 0, 0);
            combined
                .items
                .push(Item::DefaultDiscipline(DefaultDisciplineDirective {
                    discipline: None,
                    span: reset,
                }));
            combined
                .items
                .push(Item::DefaultTransition(DefaultTransitionDirective {
                    value: Expression::Number(NumberLit {
                        value: SemanticAnalyzer::SIMULATOR_DEFAULT_TRANSITION,
                        raw: "1e-9".into(),
                        span: reset,
                    }),
                    span: reset,
                }));
            for item in library.items {
                match &item {
                    Item::Nature(nature)
                        if analyzed
                            .disciplines
                            .natures
                            .contains_key(nature.name.as_str()) =>
                    {
                        continue;
                    }
                    Item::Discipline(discipline)
                        if analyzed
                            .disciplines
                            .disciplines
                            .contains_key(discipline.name.as_str()) =>
                    {
                        continue;
                    }
                    Item::Module(module) if analyzed.modules.contains_key(&module.name) => {
                        return Err(conflict(
                            format!(
                                "ordinary module '{}' is declared by both the device source and selected connection library '{}'",
                                module.name,
                                self.library.source_package(),
                            ),
                            module.span,
                        ));
                    }
                    _ => {}
                }
                combined.items.push(item);
            }
            SemanticAnalyzer::new().analyze(&combined)?
        };
        configured.connection_configuration = Some(Box::new(self.clone()));
        measurements.record(PipelinePhase::Semantic, started.elapsed())?;
        Ok(configured)
    }
}

fn retain_selected_rules(items: &mut Vec<Item>, block: &str) {
    items.retain(|item| !matches!(item, Item::ConnectRules(rules) if rules.name != block));
}

fn conflict(message: String, span: Span) -> CompileError {
    CompileError::Semantic(SemanticError::new(
        SemanticErrorKind::UnsupportedFeature(message),
        span,
    ))
}

/// A shared physical name must retain its meaning in both source closures.
/// Source locations are provenance and do not participate in equality.
fn compatible_physics(device: &DisciplineDb, library: &DisciplineDb) -> CompileResult<()> {
    for (name, right) in &library.natures {
        let Some(left) = device.natures.get(name) else {
            continue;
        };
        if left.base != right.base
            || left.units != right.units
            || left.abstol.to_bits() != right.abstol.to_bits()
            || left.access != right.access
            || left.idt_nature != right.idt_nature
            || left.ddt_nature != right.ddt_nature
        {
            return Err(conflict(
                format!(
                    "nature '{name}' has conflicting definitions in the device source and selected connection library",
                ),
                right.span.or(left.span).unwrap_or_else(Span::dummy),
            ));
        }
    }
    for (name, right) in &library.disciplines {
        let Some(left) = device.disciplines.get(name) else {
            continue;
        };
        if left.domain != right.domain
            || left.potential != right.potential
            || left.flow != right.flow
        {
            return Err(conflict(
                format!(
                    "discipline '{name}' has conflicting definitions in the device source and selected connection library",
                ),
                right.span.or(left.span).unwrap_or_else(Span::dummy),
            ));
        }
    }
    Ok(())
}

pub(crate) fn library_diagnostic(
    configuration: &ConnectionConfiguration,
    diagnostic: crate::CompileDiagnostic,
) -> crate::SourceCompileDiagnostic {
    let span = diagnostic.span;
    crate::SourceCompileDiagnostic {
        severity: diagnostic.severity,
        phase: diagnostic.phase,
        code: diagnostic.code,
        message: diagnostic.message,
        path: Some(format!(
            "{} (preprocessed)",
            configuration.library().source_package()
        )),
        byte_start: span.as_ref().map(|span| span.byte_start as usize),
        byte_end: span.as_ref().map(|span| span.byte_end as usize),
        line: span
            .as_ref()
            .and_then(|span| span.start.map(|position| position.line as usize)),
        column: span
            .as_ref()
            .and_then(|span| span.start.map(|position| position.column as usize)),
    }
}
