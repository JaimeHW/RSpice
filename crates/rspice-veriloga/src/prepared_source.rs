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

/// A sealed source closure with reusable parsing and declaration analysis.
///
/// This can describe a standalone connect library with no ordinary module.
/// Preparation freezes the source, macros, physical definitions and compiler
/// options. Compiling further modules does not consult the filesystem again.
/// Bodies needing occurrence context are analyzed after module selection.
/// Keep preparation scoped to elaboration; it retains the parsed syntax tree.
#[derive(Debug)]
pub struct PreparedRuntimeSource {
    pub(crate) source_package: String,
    pub(crate) replay_module: Option<smol_str::SmolStr>,
    pub(crate) replay_configuration: Option<Box<crate::ConnectionConfiguration>>,
    pub(crate) source: String,
    pub(crate) analyzed: crate::semantic::AnalyzedFile,
    pub(crate) dependencies: Vec<PreparedSourceDependency>,
    pub(crate) compiler_options: CompilerOptions,
    pub(crate) metrics: PipelineMetrics,
    pub(crate) diagnostics: Vec<crate::SourceCompileDiagnostic>,
    pub(crate) source_map: crate::prepared_diagnostics::PreparedSourceMap,
}

impl PreparedRuntimeSource {
    pub fn preprocessed_source(&self) -> &str {
        &self.source
    }

    /// Input identity for a selected runtime, including preserved assignments.
    pub fn runtime_source_identity(&self, module: Option<&str>) -> CompileResult<[u8; 32]> {
        let name = self.resolved_module(module)?;
        Ok(runtime_source_identity(
            &crate::canonical_ir::source_identity(&self.source),
            name,
            &crate::parameter_override::specialization_identity(
                &self.analyzed.source_specialization,
            ),
        ))
    }

    // Root assignments were applied during artifact preparation. They cannot be
    // interpreted as overrides for another module in the retained source.
    fn selected_module<'a>(&'a self, module: Option<&'a str>) -> CompileResult<Option<&'a str>> {
        if let (Some(requested), Some(original)) = (module, self.replay_module.as_deref())
            && requested != original
        {
            return Err(crate::CompileError::ModuleSelection(format!(
                "artifact preparation belongs to module '{original}'; cannot select '{requested}'"
            )));
        }
        Ok(module.or(self.replay_module.as_deref()))
    }

    pub(crate) fn resolved_module<'a>(
        &'a self,
        requested: Option<&'a str>,
    ) -> CompileResult<&'a str> {
        let requested = self.selected_module(requested)?;
        let ordinary: Vec<_> = self.module_names().collect();
        if let Some(name) = requested {
            let connect = self.analyzed.source.items.iter().any(|item| matches!(item, crate::ast::Item::ConnectModule(module) if module.name == name));
            if connect && ordinary.contains(&name) {
                return Err(crate::CompileError::ModuleSelection(format!(
                    "'{name}' names both an ordinary module and a connect module"
                )));
            }
            if connect || ordinary.contains(&name) {
                return Ok(name);
            }
            return Err(crate::CompileError::ModuleSelection(format!(
                "module '{name}' not found; the file declares: {}",
                ordinary.join(", ")
            )));
        }
        match ordinary.as_slice() {
            [name] => Ok(*name),
            [] => Err(crate::CompileError::ModuleSelection(
                "no modules found in source".into(),
            )),
            names => Err(crate::CompileError::ModuleSelection(format!(
                "the file declares multiple modules: {}; select one by name",
                names.join(", ")
            ))),
        }
    }

    pub(crate) fn analysis_for_module(
        &self,
        name: &str,
        measurements: &mut crate::metrics::MetricsRecorder,
    ) -> CompileResult<std::borrow::Cow<'_, crate::semantic::AnalyzedFile>> {
        let contextual = self.analyzed.source.items.iter().any(|item| {
            matches!(item,
            crate::ast::Item::Module(module) | crate::ast::Item::ConnectModule(module)
            if module.reference_sources.is_some())
        });
        if !self.analyzed.deferred_hierarchy
            && (!contextual || self.analyzed.selected_root.as_deref() == Some(name))
        {
            return Ok(std::borrow::Cow::Borrowed(&self.analyzed));
        }
        measurements.checkpoint(crate::PipelinePhase::Semantic)?;
        let started = web_time::Instant::now();
        let mut analyzed = crate::semantic::SemanticAnalyzer::new()
            .analyze_selected(&self.analyzed.source, Some(name))?;
        analyzed.source_specialization = self.analyzed.source_specialization.clone();
        measurements.record(crate::PipelinePhase::Semantic, started.elapsed())?;
        measurements.metrics_mut().module_count =
            crate::metrics::usize_to_u64(analyzed.modules.len());
        Ok(std::borrow::Cow::Owned(analyzed))
    }

    pub fn is_connect_library(&self) -> bool {
        self.module_names().next().is_none()
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
            source: self
                .analyzed
                .connect_rules
                .has_declarations()
                .then(|| std::sync::Arc::from(self.source.as_str())),
            builtin_delegations: crate::connect::library::equivalent_declarations(
                &self.source,
                &self.analyzed.source,
            ),
            rules: self.analyzed.connect_rules.clone(),
            disciplines: self.analyzed.disciplines.clone(),
            declares_module: self.module_names().next().is_some(),
        }
    }

    /// Retain active connection declarations and authored bodies without
    /// requiring an ordinary device module or generating executable code.
    pub fn connection_artifact(&self) -> Option<crate::ConnectionLibraryArtifact> {
        self.analyzed.connect_rules.has_declarations().then(|| {
            crate::ConnectionLibraryArtifact::from_prepared(&self.source_package, &self.source)
        })
    }

    /// Select from this already analyzed closure without parsing it again.
    pub fn connection_configuration(
        &self,
        block: &str,
    ) -> Result<crate::ConnectionConfiguration, String> {
        self.analyzed
            .connect_rules
            .select_block(block)
            .map_err(|error| error.to_string())?;
        Ok(crate::ConnectionConfiguration::selected(
            crate::ConnectionLibraryArtifact::from_prepared(&self.source_package, &self.source),
            block,
        ))
    }

    /// Elaborate with an explicit design configuration. The immutable original
    /// preparation remains reusable for a different selection.
    pub fn compile_runtime_with_connections(
        &self,
        module: Option<&str>,
        configuration: &crate::ConnectionConfiguration,
        control: &dyn PipelineControl,
    ) -> CompileResult<CompiledRuntimeFile> {
        VerilogACompiler::new(self.compiler_options.clone()).compile_prepared_runtime_with_control(
            self,
            self.selected_module(module)?,
            Some(configuration),
            control,
        )
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
        match self.replay_configuration.as_deref() {
            Some(configuration) => {
                self.diagnostics_for_error_with_connections(configuration, error)
            }
            None => self.source_map.diagnostics(&self.source, error),
        }
    }

    /// Map configured-compilation errors against the frozen device or library.
    /// External library locations refer to its retained preprocessed document;
    /// original include/macro coordinates are not part of library transport.
    pub fn diagnostics_for_error_with_connections(
        &self,
        configuration: &crate::ConnectionConfiguration,
        error: &crate::CompileError,
    ) -> Vec<crate::SourceCompileDiagnostic> {
        let original = self.source_map.diagnostics(&self.source, error);
        let external =
            crate::compile_diagnostics(configuration.library().preprocessed_source(), error);
        original
            .into_iter()
            .zip(external)
            .map(|(original, external)| {
                if external
                    .span
                    .as_ref()
                    .is_some_and(|span| span.source_id == 1)
                {
                    crate::connection_configuration::library_diagnostic(configuration, external)
                } else {
                    original
                }
            })
            .collect()
    }

    /// Compile one selected module with the options frozen at preparation.
    /// The returned metrics include the shared preparation prefix as provenance;
    /// progress callbacks only announce work actually performed by this call.
    pub fn compile_runtime_with_control(
        &self,
        module: Option<&str>,
        control: &dyn PipelineControl,
    ) -> CompileResult<CompiledRuntimeFile> {
        VerilogACompiler::new(self.compiler_options.clone()).compile_prepared_runtime_with_control(
            self,
            self.selected_module(module)?,
            self.replay_configuration.as_deref(),
            control,
        )
    }
}

pub(crate) fn runtime_source_identity(
    source: &str,
    module: &str,
    assignments: &[u8; 32],
) -> [u8; 32] {
    let mut hash = blake3::Hasher::new();
    hash.update(b"rspice.runtime-source\0");
    hash.update(&(source.len() as u64).to_le_bytes());
    hash.update(source.as_bytes());
    hash.update(&(module.len() as u64).to_le_bytes());
    hash.update(module.as_bytes());
    hash.update(assignments);
    *hash.finalize().as_bytes()
}

impl VerilogACompiler {
    /// Recover the exact retained input of an already validated runtime. This
    /// never opens a source path; its containing registration owns provenance.
    /// The preparation is bound to the artifact's root module and assignments.
    /// Default compilation reuses its captured connection configuration; an
    /// explicit configuration replaces that selection.
    pub fn prepare_artifact_runtime_source(
        &self,
        artifact: &crate::canonical_ir::CanonicalIrArtifact,
        control: &dyn PipelineControl,
    ) -> CompileResult<PreparedRuntimeSource> {
        artifact.validate().map_err(Self::canonical_ir_error)?;
        let source = artifact
            .connections
            .source()
            .or(artifact.parameter_source.as_deref())
            .ok_or_else(|| {
                crate::CompileError::ModuleSelection(
                    "runtime has no retained hierarchy source; recompile it".into(),
                )
            })?;
        let mut measurements = crate::metrics::MetricsRecorder::with_control(
            source.len(),
            self.options.performance_budget.clone(),
            control,
        );
        let parameters: Vec<_> = artifact
            .source_specialization
            .iter()
            .map(|(name, value)| (name.as_str(), value.clone()))
            .collect();
        let analyzed = self.analyze_preprocessed_with_parameters(
            &artifact.metadata.source_package,
            source,
            Some(&artifact.hir.module_name),
            &parameters,
            &mut measurements,
        )?;
        let source_map = crate::prepared_diagnostics::PreparedSourceMap::from_preprocessed(
            &artifact.metadata.source_package,
            source,
        );
        let diagnostics = source_map.warnings(source, &analyzed.warnings);
        Ok(PreparedRuntimeSource {
            source_package: artifact.metadata.source_package.to_string(),
            replay_module: Some(artifact.hir.module_name.clone()),
            replay_configuration: artifact.connections.configuration().cloned().map(Box::new),
            source: source.to_owned(),
            analyzed,
            dependencies: Vec::new(),
            compiler_options: self.options.clone(),
            metrics: measurements.finish(),
            diagnostics,
            source_map,
        })
    }
}
