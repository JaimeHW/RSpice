//! A sealed source closure shared by virtual modules and connection libraries.

use crate::preprocessor::{PreprocessedSource, PreprocessedSourceSegment};
use crate::virtual_source;
use crate::{
    CompileError, ConnectSpecification, NoPipelineControl, PipelineControl, PipelineMetrics,
    PipelinePhase, PreparedRuntimeSource, PreparedSourceDependency, RuntimeQualificationOptions,
    VerilogACompiler, VirtualCompileLimits, VirtualRuntimeCompilation,
    VirtualRuntimeCompileFailure, VirtualSourceBundle, VirtualSourceDependency,
    VirtualSourceInclude,
};

/// An immutable, analyzed virtual root with its exact active dependency graph.
///
/// This supports standalone connection libraries with no ordinary modules.
/// Repeated module emission reuses the same analyzed tree, compiler options,
/// source identities and diagnostic map, without reprocessing the bundle.
/// Keep this object scoped to elaboration rather than a global cache.
#[derive(Debug)]
pub struct PreparedVirtualSource {
    prepared: PreparedRuntimeSource,
    segments: Vec<PreprocessedSourceSegment>,
    source_bundle: VirtualSourceBundle,
    dependency_closure: Vec<VirtualSourceDependency>,
    include_graph: Vec<VirtualSourceInclude>,
    source_bundle_identity: String,
    dependency_closure_identity: String,
    limits: VirtualCompileLimits,
}

impl PreparedVirtualSource {
    pub fn module_names(&self) -> impl Iterator<Item = &str> {
        self.prepared.module_names()
    }

    pub fn is_connect_library(&self) -> bool {
        self.prepared.is_connect_library()
    }

    pub fn connect_specification(&self) -> ConnectSpecification {
        self.prepared.connect_specification()
    }

    pub fn connection_artifact(&self) -> Option<crate::ConnectionLibraryArtifact> {
        self.prepared.connection_artifact()
    }

    pub fn dependency_closure(&self) -> &[VirtualSourceDependency] {
        &self.dependency_closure
    }

    pub fn include_graph(&self) -> &[VirtualSourceInclude] {
        &self.include_graph
    }

    pub fn metrics(&self) -> &PipelineMetrics {
        self.prepared.metrics()
    }

    pub(crate) fn diagnose(&self, error: CompileError) -> VirtualRuntimeCompileFailure {
        // Retain one expanded string during successful preparation/emission;
        // the mapped diagnostic adapter only needs a copy on an error path.
        VirtualRuntimeCompileFailure::from_compiler(
            error,
            &PreprocessedSource {
                source: self.prepared.source.clone(),
                segments: self.segments.clone(),
            },
            &self.dependency_closure,
        )
    }

    pub fn compile_runtime(
        &self,
        module_name: &str,
    ) -> Result<VirtualRuntimeCompilation, VirtualRuntimeCompileFailure> {
        self.compile_runtime_with_qualifications_and_control(
            module_name,
            RuntimeQualificationOptions::NONE,
            &NoPipelineControl,
        )
    }

    pub fn compile_runtime_with_qualifications_and_control(
        &self,
        module_name: &str,
        qualifications: RuntimeQualificationOptions,
        control: &dyn PipelineControl,
    ) -> Result<VirtualRuntimeCompilation, VirtualRuntimeCompileFailure> {
        virtual_source::validate_compile_request(&self.source_bundle, module_name, self.limits)
            .map_err(CompileError::from)
            .map_err(VirtualRuntimeCompileFailure::unmapped)?;
        let compiler = VerilogACompiler::new(self.prepared.compiler_options.clone());
        let mut measurements = crate::metrics::MetricsRecorder::with_control(
            self.prepared.source.len(),
            compiler.options.performance_budget.clone(),
            control,
        );
        *measurements.metrics_mut() = self.prepared.metrics.clone();
        let runtime = compiler
            .compile_runtime_analyzed_measured(
                &self.prepared.source_package,
                &self.prepared.source,
                &self.prepared.analyzed,
                Some(module_name),
                qualifications,
                &mut measurements,
            )
            .map_err(|error| self.diagnose(error))?;
        let compiler_contract_identity = virtual_source::compiler_contract_identity(
            &self.prepared.compiler_options,
            self.source_bundle.root_path(),
            module_name,
            &self.dependency_closure_identity,
        );
        let runtime_contract_identity =
            virtual_source::runtime_contract_identity(&compiler_contract_identity, &runtime);
        let compilation = VirtualRuntimeCompilation {
            runtime,
            root_path: self.source_bundle.root_path().to_owned(),
            selected_module: module_name.to_owned(),
            dependency_closure: self.dependency_closure.clone(),
            include_graph: self.include_graph.clone(),
            source_bundle_identity: self.source_bundle_identity.clone(),
            dependency_closure_identity: self.dependency_closure_identity.clone(),
            compiler_contract_identity,
            runtime_contract_identity,
            source_bundle: self.source_bundle.clone(),
            compiler_options: self.prepared.compiler_options.clone(),
        };
        virtual_source::validate_compilation(&compilation)
            .map_err(VirtualRuntimeCompileFailure::unmapped)?;
        Ok(compilation)
    }
}

impl VerilogACompiler {
    pub fn prepare_virtual_runtime_source(
        &self,
        bundle: &VirtualSourceBundle,
        limits: VirtualCompileLimits,
    ) -> Result<PreparedVirtualSource, VirtualRuntimeCompileFailure> {
        self.prepare_virtual_runtime_source_with_control(bundle, limits, &NoPipelineControl)
    }

    /// Prepare the same sealed source authority used for virtual compilation,
    /// while retaining diagnostics, source limits and progress/cancellation.
    pub fn prepare_virtual_runtime_source_with_control(
        &self,
        bundle: &VirtualSourceBundle,
        limits: VirtualCompileLimits,
        control: &dyn PipelineControl,
    ) -> Result<PreparedVirtualSource, VirtualRuntimeCompileFailure> {
        let input_bytes = bundle.files().iter().fold(0usize, |total, file| {
            total.saturating_add(file.source.len())
        });
        let limits = virtual_source::validate_bundle_request(bundle, limits)
            .map_err(CompileError::from)
            .map_err(VirtualRuntimeCompileFailure::unmapped)?;
        let mut measurements = crate::metrics::MetricsRecorder::with_control(
            input_bytes,
            self.options.performance_budget.clone(),
            control,
        );
        let provider = virtual_source::VirtualBundleProvider::new(bundle, limits);
        let mut preprocessor = self.configured_in_memory_preprocessor();
        measurements
            .checkpoint(PipelinePhase::Preprocess)
            .map_err(CompileError::from)
            .map_err(VirtualRuntimeCompileFailure::unmapped)?;
        let started = web_time::Instant::now();
        let preprocessed = preprocessor
            .preprocess_provider_root_mapped(&provider, std::path::Path::new(bundle.root_path()))
            .map_err(|error| {
                VirtualRuntimeCompileFailure::from_preprocessor(
                    error,
                    preprocessor.dependency_documents(),
                )
            })?;
        measurements
            .record(PipelinePhase::Preprocess, started.elapsed())
            .map_err(CompileError::from)
            .map_err(VirtualRuntimeCompileFailure::unmapped)?;
        measurements.metrics_mut().preprocessed_bytes =
            crate::metrics::usize_to_u64(preprocessed.source.len());
        let dependency_closure = virtual_source::dependencies_from_preprocessor(
            preprocessor.take_dependency_documents(),
        );
        let include_graph =
            virtual_source::includes_from_preprocessor(preprocessor.take_include_graph());
        measurements.metrics_mut().dependency_count =
            crate::metrics::usize_to_u64(dependency_closure.len());
        let analyzed = self
            .analyze_preprocessed(bundle.root_path(), &preprocessed.source, &mut measurements)
            .map_err(|error| {
                VirtualRuntimeCompileFailure::from_compiler(
                    error,
                    &preprocessed,
                    &dependency_closure,
                )
            })?;
        measurements
            .checkpoint(PipelinePhase::Semantic)
            .map_err(CompileError::from)
            .map_err(VirtualRuntimeCompileFailure::unmapped)?;
        let dependencies = dependency_closure
            .iter()
            .filter(|document| document.origin == crate::SourceDocumentOrigin::Provider)
            .map(|document| PreparedSourceDependency {
                path: document.logical_path.clone().into(),
                byte_len: document.source.len(),
                content_identity: *blake3::hash(document.source.as_bytes()).as_bytes(),
            })
            .collect();
        Ok(PreparedVirtualSource {
            prepared: PreparedRuntimeSource {
                source_package: bundle.root_path().to_owned(),
                source: preprocessed.source,
                analyzed,
                dependencies,
                compiler_options: self.options.clone(),
                metrics: measurements.finish(),
            },
            segments: preprocessed.segments,
            source_bundle: bundle.clone(),
            source_bundle_identity: virtual_source::source_bundle_identity(bundle),
            dependency_closure_identity: virtual_source::dependency_closure_identity(
                &dependency_closure,
                &include_graph,
            ),
            dependency_closure,
            include_graph,
            limits,
        })
    }
}
