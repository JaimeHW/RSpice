//! Immutable front-end preparation shared by module and connection elaboration.

use crate::{
    CompileResult, CompiledRuntimeFile, CompilerOptions, ConnectSpecification, NoPipelineControl,
    PipelineControl, PipelineMetrics, VerilogACompiler,
};
use std::path::PathBuf;

/// Identity of the exact provider document consumed during preprocessing.
/// Built-in headers are covered by compiler/source identity and have no disk
/// dependency. A later filesystem read must not redefine this identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedSourceDependency {
    pub path: PathBuf,
    pub byte_len: usize,
    pub content_identity: [u8; 32],
}

/// A source closure analyzed once, before selecting executable modules.
///
/// This can describe a standalone connect library with no ordinary module.
/// Preparation freezes the source, macros, physical definitions and compiler
/// options. Compiling further modules does not consult the filesystem again.
/// Keep preparation scoped to elaboration; it retains the analyzed syntax tree.
#[derive(Debug)]
pub struct PreparedRuntimeSource {
    pub(crate) source_package: String,
    pub(crate) source: String,
    pub(crate) analyzed: crate::semantic::AnalyzedFile,
    pub(crate) dependencies: Vec<PreparedSourceDependency>,
    pub(crate) compiler_options: CompilerOptions,
    pub(crate) metrics: PipelineMetrics,
    pub(crate) diagnostics: Vec<crate::SourceCompileDiagnostic>,
    pub(crate) source_map: crate::prepared_diagnostics::PreparedSourceMap,
}

impl PreparedRuntimeSource {
    pub fn is_connect_library(&self) -> bool {
        self.analyzed.modules.is_empty()
            && self.analyzed.source.items.iter().any(|item| {
                matches!(
                    item,
                    crate::ast::Item::ConnectModule(_) | crate::ast::Item::ConnectRules(_)
                )
            })
    }

    /// Ordinary modules in declaration order, excluding connect modules.
    pub fn module_names(&self) -> impl Iterator<Item = &str> {
        self.analyzed
            .source
            .items
            .iter()
            .filter_map(|item| match item {
                crate::ast::Item::Module(module) => Some(module.name.as_str()),
                _ => None,
            })
    }

    pub fn dependencies(&self) -> &[PreparedSourceDependency] {
        &self.dependencies
    }

    pub fn metrics(&self) -> &PipelineMetrics {
        &self.metrics
    }

    /// Nonfatal findings mapped to the exact source closure prepared here.
    pub fn diagnostics(&self) -> &[crate::SourceCompileDiagnostic] {
        &self.diagnostics
    }

    pub fn connect_specification(&self) -> ConnectSpecification {
        ConnectSpecification {
            source_identity: crate::canonical_ir::source_identity(&self.source),
            rules: self.analyzed.connect_rules.clone(),
            disciplines: self.analyzed.disciplines.clone(),
            declares_module: !self.analyzed.modules.is_empty(),
        }
    }

    /// Retain active connection declarations and authored bodies without
    /// requiring an ordinary device module or generating executable code.
    pub fn connection_artifact(&self) -> Option<crate::ConnectionLibraryArtifact> {
        self.analyzed.connect_rules.has_declarations().then(|| {
            crate::ConnectionLibraryArtifact::from_prepared(&self.source_package, &self.source)
        })
    }

    pub fn compile_runtime(&self, module: Option<&str>) -> CompileResult<CompiledRuntimeFile> {
        self.compile_runtime_with_control(module, &NoPipelineControl)
    }

    /// Resolve an error returned by this preparation's compilation methods
    /// against its original source documents. Do not pass an error from a
    /// different source snapshot. Mapping never rereads the filesystem and
    /// leaves locations absent when the compiler did not supply a span.
    pub fn diagnostics_for_error(
        &self,
        error: &crate::CompileError,
    ) -> Vec<crate::SourceCompileDiagnostic> {
        self.source_map.diagnostics(&self.source, error)
    }

    /// Compile one selected module with the options frozen at preparation.
    /// The returned metrics include the shared preparation prefix as provenance;
    /// progress callbacks only announce work actually performed by this call.
    pub fn compile_runtime_with_control(
        &self,
        module: Option<&str>,
        control: &dyn PipelineControl,
    ) -> CompileResult<CompiledRuntimeFile> {
        VerilogACompiler::new(self.compiler_options.clone())
            .compile_prepared_runtime_with_control(self, module, control)
    }
}
