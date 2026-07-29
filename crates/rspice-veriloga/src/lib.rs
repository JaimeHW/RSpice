//! # RSpice Verilog-A/AMS Compiler
//!
//! A Verilog-A compiler targeting the supported analog subset of the
//! Verilog-AMS Language Reference Manual (LRM) 2.4 for RSpice.
//!
//! ## Architecture
//!
//! Source text is lowered through a fixed front end and then fans out to
//! three independent backends:
//!
//! 1. **Preprocessing** ([`preprocessor`]) - `` `include ``/`` `define ``
//!    expansion, with the standard VAMS headers built in ([`stdlib`])
//! 2. **Lexical Analysis** ([`lexer`]) - Tokenizes Verilog-A/AMS source code
//! 3. **Parsing** ([`parser`]) - Produces an Abstract Syntax Tree ([`ast`])
//! 4. **Semantic Analysis** ([`semantic`]) - Type checking, symbol and
//!    discipline resolution
//! 5. **IR Generation** ([`ir`], [`expr_converter`]) - Device equations plus
//!    forward-mode symbolic derivatives, so Jacobians are analytic
//! 6. **Canonical IR** ([`canonical_ir`]) - The validated, content-digested
//!    HIR/MIR artifact that the backends consume
//!
//! The backends are [`codegen`], emitting a bytecode [`CompiledModel`] run by
//! [`vm`]; `native`, a JIT behind the `native` feature; and [`rust_backend`],
//! an offline emitter that turns canonical IR into Rust source compiled
//! directly into `rspice-core`. The first two are driven in-process through
//! [`device::VerilogADevice`]; the third runs ahead of the build.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use rspice_veriloga::{VerilogACompiler, CompilerOptions};
//!
//! let source = r#"
//!     `include "disciplines.vams"
//!     module resistor(p, n);
//!         inout p, n;
//!         electrical p, n;
//!         parameter real r = 1.0 from (0:inf);
//!         analog I(p, n) <+ V(p, n) / r;
//!     endmodule
//! "#;
//!
//! let compiler = VerilogACompiler::new(CompilerOptions::default());
//! let model = compiler.compile(source)?;
//! ```
//!
//! [`VerilogACompiler`] has three further families of entry point:
//! `compile_canonical_ir*` for the canonical artifact alone, `compile_runtime`
//! / `compile_file_runtime_with_metadata` for both artifacts from a single
//! parse, and `compile_virtual_runtime*` for sealed [`VirtualSourceBundle`]s
//! that never touch the file system.
//!
//! ## Verilog-A language support
//!
//! The supported subset, the constructs that are accepted but inert, and the
//! constructs that are rejected outright are enumerated in the crate README.
//! Unsupported input is always a [`CompileError`] naming the construct and its
//! span — the compiler does not silently drop what it cannot lower, and
//! [`semantic`] carries tests pinning that.

#![allow(
    clippy::assertions_on_constants,
    clippy::collapsible_if,
    clippy::filter_map_bool_then,
    clippy::if_same_then_else,
    clippy::items_after_test_module,
    clippy::manual_is_multiple_of,
    clippy::map_identity,
    clippy::match_like_matches_macro,
    clippy::needless_borrow,
    clippy::needless_lifetimes,
    clippy::ptr_arg,
    clippy::question_mark,
    clippy::redundant_guards,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unnecessary_map_or,
    clippy::unnecessary_unwrap
)]

pub mod ast;
pub mod canonical_ir;
pub mod codegen;
pub mod disciplines;
pub mod error;
pub mod expr_converter;
pub mod ir;
pub mod lexer;
pub mod metrics;
pub mod parser;
pub mod preprocessor;
pub mod runtime_report;
pub mod rust_backend;
pub mod semantic;
pub mod source;
pub mod stdlib;
pub mod types;
pub mod virtual_source;

/// Laplace (s-domain) filters for transient analysis
pub mod laplace;

/// Z-domain (sampled-data) filters for the zi_* operators
pub mod zfilter;

/// Virtual machine for bytecode execution
pub mod vm;

/// Device interface for circuit simulation
pub mod device;

/// Native code generation and compilation (feature-gated)
#[cfg(feature = "native")]
pub mod native;

// Re-export primary types
pub use ast::{Module, SourceFile};
pub use codegen::{CodeGenerator, CompiledModel};
pub use error::{CompileError, CompileResult};
pub use lexer::{Lexer, Token, TokenKind};
pub use metrics::{
    Measured, NoPipelineControl, PerformanceBudget, PerformanceBudgetExceeded, PhaseTiming,
    PipelineCancelled, PipelineControl, PipelineMetrics, PipelinePhase,
};
pub use parser::Parser;
pub use preprocessor::{
    FileSystemSourceProvider, PreprocessedDependency, PreprocessedInclude, Preprocessor,
    PreprocessorError, SourceDocument, SourceDocumentOrigin, SourceProvider, SourceProviderLimits,
};
pub use runtime_report::{
    CompileDiagnostic, CompileDiagnosticPhase, CompileDiagnosticSeverity, CompileDiagnosticSpan,
    CompileSourcePosition, RuntimeAbiParameter, RuntimeAbiPort, RuntimeAbiSummary,
    RuntimeArtifactIntegrityError, RuntimeCompileReport, RuntimeQualificationOptions,
    RuntimeTarget, RuntimeTargetMaturity, RuntimeTargetQualification, RuntimeTargetQualifications,
    RuntimeTargetReadiness, compile_diagnostics,
};
pub use semantic::SemanticAnalyzer;
pub use source::{SourceId, SourceMap, Span};
pub use types::{FunctionRegistry, ParameterRange, ValueType};
pub use virtual_source::{
    VirtualCompileLimits, VirtualRuntimeCompilation, VirtualRuntimeCompileFailure,
    VirtualSourceBundle, VirtualSourceDependency, VirtualSourceDiagnostic, VirtualSourceError,
    VirtualSourceFile, VirtualSourceInclude,
};

/// Result of compiling a Verilog-A source file from disk.
///
/// Includes the compiled model artifact and canonical dependency paths
/// discovered during preprocessing (`include` expansion).
#[derive(Debug, Clone)]
pub struct CompiledFile {
    /// Compiled model artifact used by the simulation engine.
    pub model: CompiledModel,
    /// Canonical source/include dependencies captured at compile time.
    pub dependencies: Vec<std::path::PathBuf>,
    /// Structured phase timings and work-size counters.
    pub metrics: PipelineMetrics,
}

/// Result of compiling a Verilog-A source file to canonical IR from disk.
///
/// Includes the canonical HIR/MIR artifact and canonical dependency
/// paths discovered during preprocessing (`include` expansion).
#[derive(Debug, Clone)]
pub struct CanonicalIrFile {
    /// Canonical HIR/MIR artifact for the selected module.
    pub artifact: canonical_ir::CanonicalIrArtifact,
    /// Canonical source/include dependencies captured at compile time.
    pub dependencies: Vec<std::path::PathBuf>,
    /// Structured phase timings and work-size counters.
    pub metrics: PipelineMetrics,
}

/// Result of compiling a Verilog-A source file for the runtime from disk.
///
/// Includes the bytecode-era compiled model, the canonical HIR/MIR
/// artifact that native JIT backends consume, and canonical dependency paths
/// discovered during preprocessing (`include` expansion).
#[derive(Debug, Clone)]
pub struct CompiledRuntimeFile {
    /// Compiled model artifact used by existing simulation metadata paths.
    pub model: CompiledModel,
    /// Canonical HIR/MIR artifact for the selected module.
    pub canonical_ir: canonical_ir::CanonicalIrArtifact,
    /// Canonical source/include dependencies captured at compile time.
    pub dependencies: Vec<std::path::PathBuf>,
    /// Structured phase timings and work-size counters.
    pub metrics: PipelineMetrics,
}

/// Main compiler entry point
pub struct VerilogACompiler {
    options: CompilerOptions,
}

/// Compiler configuration options.
///
/// Only the three preprocessor fields ([`include_paths`](Self::include_paths),
/// [`defines`](Self::defines), [`undefines`](Self::undefines)) change generated
/// artifacts. [`performance_budget`](Self::performance_budget) is an
/// operational policy and is deliberately excluded from artifact and compiler
/// contract identities. The remaining fields are reserved and are folded into
/// the compiler-contract identity.
#[derive(Debug, Clone, Default)]
pub struct CompilerOptions {
    /// Reserved for Verilog-AMS mixed-signal support. Gates no behavior today;
    /// the analog subset is always accepted and digital constructs are always
    /// rejected regardless of this flag.
    pub enable_ams: bool,
    /// Directories searched by `` `include `` directives, in order. Consulted
    /// only by the file-system-backed entry points; the sealed in-memory paths
    /// ([`VerilogACompiler::compile_runtime`] and the virtual-bundle APIs)
    /// deliberately ignore them.
    pub include_paths: Vec<std::path::PathBuf>,
    /// Macros predefined before preprocessing, as `(name, value)`. A `None`
    /// value defines the macro as the empty string.
    pub defines: Vec<(String, Option<String>)>,
    /// Standard preprocessor macros to remove before [`Self::defines`] is
    /// applied, letting a caller replace a built-in definition rather than
    /// redefine it.
    pub undefines: Vec<String>,
    /// Reserved for strict LRM compliance (erroring on vendor extensions).
    /// Gates no behavior today.
    pub strict_mode: bool,
    /// Reserved for selecting the companion-model integration rule. Gates no
    /// behavior today: `ddt`/`idt` compile to integration-agnostic state slots
    /// and the engine supplies the coefficients per timestep through
    /// [`vm::IntegrationCoefficients`].
    pub integration_order: IntegrationOrder,
    /// Optional wall-clock limits. Empty by default, so ordinary compilation
    /// is never rejected due to host load or timer resolution.
    pub performance_budget: PerformanceBudget,
}

/// Integration order for the `idt` and `ddt` companion models.
///
/// Reserved — see [`CompilerOptions::integration_order`]. The effective
/// integration rule is chosen by the engine at run time, not here.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum IntegrationOrder {
    /// First-order (Gear-1, Backward Euler)
    First,
    /// Second-order (Gear-2, Trapezoidal)
    #[default]
    Second,
}

fn compiler_phase_trace_enabled() -> bool {
    std::env::var_os("RSPICE_VERILOGA_PHASE_TRACE").is_some()
        || std::env::var_os("RSPICE_VERILOGA_CANONICAL_IR_PHASE_TRACE").is_some()
}

fn trace_compiler_phase(
    enabled: bool,
    target: &str,
    phase: &str,
    elapsed: Option<std::time::Duration>,
    detail: Option<String>,
) {
    if !enabled {
        return;
    }
    match (elapsed, detail) {
        (Some(elapsed), Some(detail)) => {
            eprintln!("{target}: finished {phase} in {elapsed:.2?} ({detail})");
        }
        (Some(elapsed), None) => {
            eprintln!("{target}: finished {phase} in {elapsed:.2?}");
        }
        (None, Some(detail)) => {
            eprintln!("{target}: starting {phase} ({detail})");
        }
        (None, None) => {
            eprintln!("{target}: starting {phase}");
        }
    }
    use std::io::Write as _;
    let _ = std::io::stderr().flush();
}

fn trace_canonical_ir_phase(
    enabled: bool,
    module_name: &str,
    phase: &str,
    elapsed: Option<std::time::Duration>,
) {
    let target = format!("canonical IR {module_name}");
    trace_compiler_phase(enabled, &target, phase, elapsed, None);
}

impl VerilogACompiler {
    /// Create a new compiler with the given options
    pub fn new(options: CompilerOptions) -> Self {
        Self { options }
    }

    /// Build a preprocessor configured from the compiler options.
    fn configured_preprocessor(&self) -> Preprocessor {
        let mut pp = Preprocessor::new();

        for inc_path in &self.options.include_paths {
            pp.add_include_path(inc_path);
        }

        for name in &self.options.undefines {
            pp.undefine(name);
        }

        for (name, value) in &self.options.defines {
            let def = preprocessor::MacroDef::simple(value.as_deref().unwrap_or(""));
            pp.define(name, def);
        }

        pp
    }

    /// Build the deterministic preprocessor used by source-only APIs.
    ///
    /// Project callers resolve their own virtual include graph before calling
    /// [`Self::compile_runtime`]. Standard built-in VAMS headers remain
    /// available, while configured disk include paths are intentionally not
    /// consulted.
    fn configured_in_memory_preprocessor(&self) -> Preprocessor {
        let mut pp = Preprocessor::new();

        for name in &self.options.undefines {
            pp.undefine(name);
        }

        for (name, value) in &self.options.defines {
            let def = preprocessor::MacroDef::simple(value.as_deref().unwrap_or(""));
            pp.define(name, def);
        }

        pp
    }

    /// Compile Verilog-A source code to a device model
    ///
    /// The source is preprocessed first, so `include/`define/`ifdef work
    /// identically whether compiling from a string or from a file. The
    /// source must contain exactly one module; multi-module sources
    /// require [`Self::compile_module`].
    pub fn compile(&self, source: &str) -> CompileResult<CompiledModel> {
        self.compile_module(source, None)
    }

    /// Compile a single-module source and retain structured measurements.
    pub fn compile_measured(&self, source: &str) -> CompileResult<Measured<CompiledModel>> {
        self.compile_module_measured(source, None)
    }

    /// Compile one module of a (possibly multi-module) Verilog-A source
    ///
    /// Foundry releases ship several modules per file; `module_name`
    /// selects which one to compile. With `None` the source must contain
    /// exactly one module - anything else is an error listing the
    /// declared module names.
    pub fn compile_module(
        &self,
        source: &str,
        module_name: Option<&str>,
    ) -> CompileResult<CompiledModel> {
        self.compile_module_measured(source, module_name)
            .map(|compiled| compiled.output)
    }

    /// Compile one module and retain structured phase measurements.
    pub fn compile_module_measured(
        &self,
        source: &str,
        module_name: Option<&str>,
    ) -> CompileResult<Measured<CompiledModel>> {
        self.compile_module_measured_with_control(source, module_name, &NoPipelineControl)
    }

    /// Cancellable, progress-observable form of [`Self::compile_module_measured`].
    pub fn compile_module_measured_with_control(
        &self,
        source: &str,
        module_name: Option<&str>,
        control: &dyn PipelineControl,
    ) -> CompileResult<Measured<CompiledModel>> {
        let mut measurements = metrics::MetricsRecorder::with_control(
            source.len(),
            self.options.performance_budget.clone(),
            control,
        );
        let mut pp = self.configured_preprocessor();
        measurements.checkpoint(PipelinePhase::Preprocess)?;
        let phase_started = web_time::Instant::now();
        let preprocessed = pp
            .preprocess_source(source)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;
        measurements.record(PipelinePhase::Preprocess, phase_started.elapsed())?;
        measurements.metrics_mut().preprocessed_bytes = metrics::usize_to_u64(preprocessed.len());
        let output =
            self.compile_preprocessed_measured(&preprocessed, module_name, &mut measurements)?;
        Ok(Measured {
            output,
            metrics: measurements.finish(),
        })
    }

    /// Compile source text once for every simulator runtime surface.
    ///
    /// Preprocessing, lexing, parsing, and semantic analysis each run once.
    /// The compiled bytecode model and canonical IR are then emitted from the
    /// same analyzed module, cross-validated, and qualified against the
    /// available runtime backends. No source or generated artifact is written
    /// to the file system.
    pub fn compile_runtime(
        &self,
        source: &str,
        module_name: Option<&str>,
    ) -> CompileResult<RuntimeCompileReport> {
        self.compile_runtime_with_qualifications(
            source,
            module_name,
            RuntimeQualificationOptions::NONE,
        )
    }

    /// Compile source text and explicitly request optional, expensive backend
    /// qualification products.
    ///
    /// Ordinary runtime/editor compilation should use [`Self::compile_runtime`].
    /// Generated Rust and native qualification are intended for offline
    /// validation and are never produced unless selected here.
    pub fn compile_runtime_with_qualifications(
        &self,
        source: &str,
        module_name: Option<&str>,
        qualifications: RuntimeQualificationOptions,
    ) -> CompileResult<RuntimeCompileReport> {
        self.compile_runtime_with_qualifications_and_control(
            source,
            module_name,
            qualifications,
            &NoPipelineControl,
        )
    }

    /// Cancellable, progress-observable runtime compilation.
    pub fn compile_runtime_with_qualifications_and_control(
        &self,
        source: &str,
        module_name: Option<&str>,
        qualifications: RuntimeQualificationOptions,
        control: &dyn PipelineControl,
    ) -> CompileResult<RuntimeCompileReport> {
        let mut measurements = metrics::MetricsRecorder::with_control(
            source.len(),
            self.options.performance_budget.clone(),
            control,
        );
        let mut pp = self.configured_in_memory_preprocessor();
        measurements.checkpoint(PipelinePhase::Preprocess)?;
        let phase_started = web_time::Instant::now();
        let preprocessed = pp
            .preprocess_source(source)
            .map_err(|error| CompileError::io_error(format!("Preprocessor error: {error}")))?;
        measurements.record(PipelinePhase::Preprocess, phase_started.elapsed())?;
        measurements.metrics_mut().preprocessed_bytes = metrics::usize_to_u64(preprocessed.len());
        self.compile_runtime_preprocessed_measured(
            "<input>",
            &preprocessed,
            module_name,
            qualifications,
            &mut measurements,
        )
    }

    /// Compile one explicitly selected module from a sealed virtual source
    /// bundle without consulting the file system.
    ///
    /// The result retains the exact active dependency closure, portable
    /// BLAKE3 identities for the source/compiler/runtime contracts, and the
    /// compiler-derived ABI evidence exposed by [`RuntimeCompileReport`].
    pub fn compile_virtual_runtime(
        &self,
        bundle: &VirtualSourceBundle,
        module_name: &str,
        limits: VirtualCompileLimits,
    ) -> CompileResult<VirtualRuntimeCompilation> {
        self.compile_virtual_runtime_with_qualifications(
            bundle,
            module_name,
            limits,
            RuntimeQualificationOptions::NONE,
        )
    }

    /// Compile a sealed virtual source bundle with explicit optional backend
    /// qualifications.
    pub fn compile_virtual_runtime_with_qualifications(
        &self,
        bundle: &VirtualSourceBundle,
        module_name: &str,
        limits: VirtualCompileLimits,
        qualifications: RuntimeQualificationOptions,
    ) -> CompileResult<VirtualRuntimeCompilation> {
        self.compile_virtual_runtime_diagnosed_with_qualifications(
            bundle,
            module_name,
            limits,
            qualifications,
        )
        .map_err(|failure| failure.error)
    }

    /// Compile a sealed virtual source bundle while retaining source-authentic
    /// diagnostics when preprocessing or a later compiler phase fails.
    pub fn compile_virtual_runtime_diagnosed(
        &self,
        bundle: &VirtualSourceBundle,
        module_name: &str,
        limits: VirtualCompileLimits,
    ) -> Result<VirtualRuntimeCompilation, VirtualRuntimeCompileFailure> {
        self.compile_virtual_runtime_diagnosed_with_qualifications(
            bundle,
            module_name,
            limits,
            RuntimeQualificationOptions::NONE,
        )
    }

    /// Diagnosed virtual compilation with explicit optional backend
    /// qualifications.
    pub fn compile_virtual_runtime_diagnosed_with_qualifications(
        &self,
        bundle: &VirtualSourceBundle,
        module_name: &str,
        limits: VirtualCompileLimits,
        qualifications: RuntimeQualificationOptions,
    ) -> Result<VirtualRuntimeCompilation, VirtualRuntimeCompileFailure> {
        let input_bytes = bundle.files().iter().fold(0_usize, |total, file| {
            total.saturating_add(file.source.len())
        });
        let mut measurements =
            metrics::MetricsRecorder::new(input_bytes, self.options.performance_budget.clone());
        let limits = virtual_source::validate_compile_request(bundle, module_name, limits)
            .map_err(CompileError::from)
            .map_err(VirtualRuntimeCompileFailure::unmapped)?;
        let provider = virtual_source::VirtualBundleProvider::new(bundle, limits);
        let mut preprocessor = self.configured_in_memory_preprocessor();
        measurements
            .checkpoint(PipelinePhase::Preprocess)
            .map_err(CompileError::from)
            .map_err(VirtualRuntimeCompileFailure::unmapped)?;
        let phase_started = web_time::Instant::now();
        let preprocessed = preprocessor
            .preprocess_provider_root_mapped(&provider, std::path::Path::new(bundle.root_path()))
            .map_err(|error| {
                VirtualRuntimeCompileFailure::from_preprocessor(
                    error,
                    preprocessor.dependency_documents(),
                )
            })?;
        measurements
            .record(PipelinePhase::Preprocess, phase_started.elapsed())
            .map_err(CompileError::from)
            .map_err(VirtualRuntimeCompileFailure::unmapped)?;
        measurements.metrics_mut().preprocessed_bytes =
            metrics::usize_to_u64(preprocessed.source.len());
        let dependency_closure = virtual_source::dependencies_from_preprocessor(
            preprocessor.take_dependency_documents(),
        );
        let include_graph =
            virtual_source::includes_from_preprocessor(preprocessor.take_include_graph());
        measurements.metrics_mut().dependency_count =
            metrics::usize_to_u64(dependency_closure.len());
        let source_bundle_identity = virtual_source::source_bundle_identity(bundle);
        let dependency_closure_identity =
            virtual_source::dependency_closure_identity(&dependency_closure, &include_graph);
        let compiler_contract_identity = virtual_source::compiler_contract_identity(
            &self.options,
            bundle.root_path(),
            module_name,
            &dependency_closure_identity,
        );
        let runtime = self
            .compile_runtime_preprocessed_measured(
                bundle.root_path(),
                &preprocessed.source,
                Some(module_name),
                qualifications,
                &mut measurements,
            )
            .map_err(|error| {
                VirtualRuntimeCompileFailure::from_compiler(
                    error,
                    &preprocessed,
                    &dependency_closure,
                )
            })?;
        let runtime_contract_identity =
            virtual_source::runtime_contract_identity(&compiler_contract_identity, &runtime);
        let compilation = VirtualRuntimeCompilation {
            runtime,
            root_path: bundle.root_path().to_owned(),
            selected_module: module_name.to_owned(),
            dependency_closure,
            include_graph,
            source_bundle_identity,
            dependency_closure_identity,
            compiler_contract_identity,
            runtime_contract_identity,
            source_bundle: bundle.clone(),
            compiler_options: self.options.clone(),
        };
        virtual_source::validate_compilation(&compilation)
            .map_err(VirtualRuntimeCompileFailure::unmapped)?;
        Ok(compilation)
    }

    fn compile_runtime_preprocessed_measured(
        &self,
        source_package: &str,
        preprocessed: &str,
        module_name: Option<&str>,
        qualifications: RuntimeQualificationOptions,
        measurements: &mut metrics::MetricsRecorder,
    ) -> CompileResult<RuntimeCompileReport> {
        let analyzed = self.analyze_preprocessed(source_package, preprocessed, measurements)?;
        let source_digest = canonical_ir::StableDigest::from_text(&preprocessed).as_hex();
        measurements.checkpoint(PipelinePhase::BytecodeGeneration)?;
        let phase_started = web_time::Instant::now();
        let model = CodeGenerator::new().generate_module_with_source_digest(
            &analyzed,
            module_name,
            source_digest,
        )?;
        measurements.record(PipelinePhase::BytecodeGeneration, phase_started.elapsed())?;
        let canonical_ir = self.build_canonical_ir_artifact(
            source_package,
            preprocessed,
            &analyzed,
            module_name,
            measurements,
        )?;
        measurements.checkpoint(PipelinePhase::RuntimeQualification)?;
        let phase_started = web_time::Instant::now();
        let mut report = RuntimeCompileReport::from_artifacts(model, canonical_ir, qualifications);
        measurements.record(PipelinePhase::RuntimeQualification, phase_started.elapsed())?;
        measurements.checkpoint(PipelinePhase::IntegrityValidation)?;
        let phase_started = web_time::Instant::now();
        report.validate_integrity().map_err(|error| {
            CompileError::CodeGen(error::CodeGenError::new(error::CodeGenErrorKind::Internal(
                format!("runtime artifact integrity validation failed: {error}"),
            )))
        })?;
        measurements.record(PipelinePhase::IntegrityValidation, phase_started.elapsed())?;
        report.metrics = measurements.metrics().clone();
        Ok(report)
    }

    /// Compile Verilog-A source code to the canonical HIR/MIR artifact.
    ///
    /// The source is preprocessed first, so `include/`define/`ifdef work
    /// identically to [`Self::compile`]. The source must contain exactly one
    /// module; multi-module sources require
    /// [`Self::compile_canonical_ir_module`].
    pub fn compile_canonical_ir(
        &self,
        source: &str,
    ) -> CompileResult<canonical_ir::CanonicalIrArtifact> {
        self.compile_canonical_ir_module(source, None)
    }

    /// Compile a single-module source to canonical IR and retain measurements.
    pub fn compile_canonical_ir_measured(
        &self,
        source: &str,
    ) -> CompileResult<Measured<canonical_ir::CanonicalIrArtifact>> {
        self.compile_canonical_ir_module_measured(source, None)
    }

    /// Compile one module of a Verilog-A source to canonical HIR/MIR.
    ///
    /// See [`Self::compile_module`] for module selection rules.
    pub fn compile_canonical_ir_module(
        &self,
        source: &str,
        module_name: Option<&str>,
    ) -> CompileResult<canonical_ir::CanonicalIrArtifact> {
        self.compile_canonical_ir_module_measured(source, module_name)
            .map(|compiled| compiled.output)
    }

    /// Compile one selected module to canonical IR and retain measurements.
    pub fn compile_canonical_ir_module_measured(
        &self,
        source: &str,
        module_name: Option<&str>,
    ) -> CompileResult<Measured<canonical_ir::CanonicalIrArtifact>> {
        self.compile_canonical_ir_module_measured_with_control(
            source,
            module_name,
            &NoPipelineControl,
        )
    }

    /// Cancellable, progress-observable canonical-IR compilation.
    pub fn compile_canonical_ir_module_measured_with_control(
        &self,
        source: &str,
        module_name: Option<&str>,
        control: &dyn PipelineControl,
    ) -> CompileResult<Measured<canonical_ir::CanonicalIrArtifact>> {
        let mut measurements = metrics::MetricsRecorder::with_control(
            source.len(),
            self.options.performance_budget.clone(),
            control,
        );
        let mut pp = self.configured_preprocessor();
        measurements.checkpoint(PipelinePhase::Preprocess)?;
        let phase_started = web_time::Instant::now();
        let preprocessed = pp
            .preprocess_source(source)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;
        measurements.record(PipelinePhase::Preprocess, phase_started.elapsed())?;
        measurements.metrics_mut().preprocessed_bytes = metrics::usize_to_u64(preprocessed.len());
        let output = self.compile_canonical_ir_preprocessed_with_metadata_measured(
            "<input>",
            &preprocessed,
            module_name,
            &mut measurements,
        )?;
        Ok(Measured {
            output,
            metrics: measurements.finish(),
        })
    }

    fn compile_preprocessed_measured(
        &self,
        source: &str,
        module_name: Option<&str>,
        measurements: &mut metrics::MetricsRecorder,
    ) -> CompileResult<CompiledModel> {
        let analyzed = self.analyze_preprocessed("<input>", source, measurements)?;

        // Phase 4 & 5: IR generation and code generation
        let source_digest = canonical_ir::StableDigest::from_text(source).as_hex();
        measurements.checkpoint(PipelinePhase::BytecodeGeneration)?;
        let phase_started = web_time::Instant::now();
        let model = CodeGenerator::new().generate_module_with_source_digest(
            &analyzed,
            module_name,
            source_digest,
        )?;
        measurements.record(PipelinePhase::BytecodeGeneration, phase_started.elapsed())?;

        Ok(model)
    }

    fn analyze_preprocessed(
        &self,
        source_package: &str,
        source: &str,
        measurements: &mut metrics::MetricsRecorder,
    ) -> CompileResult<semantic::AnalyzedFile> {
        let trace = compiler_phase_trace_enabled();
        let target = format!("Verilog-A compiler {source_package}");

        // Phase 1: Lexical analysis
        measurements.checkpoint(PipelinePhase::Lex)?;
        trace_compiler_phase(
            trace,
            &target,
            "lex",
            None,
            Some(format!("{} bytes", source.len())),
        );
        let phase_started = web_time::Instant::now();
        let source_map = SourceMap::new();
        let source_id = source_map.add_source(source_package, source);
        let tokens = Lexer::new(source, source_id).collect_tokens()?;
        measurements.record(PipelinePhase::Lex, phase_started.elapsed())?;
        measurements.metrics_mut().token_count = metrics::usize_to_u64(tokens.len());
        trace_compiler_phase(
            trace,
            &target,
            "lex",
            Some(phase_started.elapsed()),
            Some(format!("{} tokens", tokens.len())),
        );

        // Phase 2: Parsing
        measurements.checkpoint(PipelinePhase::Parse)?;
        trace_compiler_phase(trace, &target, "parse", None, None);
        let phase_started = web_time::Instant::now();
        let source_file = Parser::new(&tokens).parse()?;
        measurements.record(PipelinePhase::Parse, phase_started.elapsed())?;
        measurements.metrics_mut().top_level_item_count =
            metrics::usize_to_u64(source_file.items.len());
        trace_compiler_phase(
            trace,
            &target,
            "parse",
            Some(phase_started.elapsed()),
            Some(format!("{} top-level items", source_file.items.len())),
        );

        // Phase 3: Semantic analysis
        measurements.checkpoint(PipelinePhase::Semantic)?;
        trace_compiler_phase(trace, &target, "semantic", None, None);
        let phase_started = web_time::Instant::now();
        let analyzed = SemanticAnalyzer::new().analyze(&source_file)?;
        measurements.record(PipelinePhase::Semantic, phase_started.elapsed())?;
        measurements.metrics_mut().module_count = metrics::usize_to_u64(analyzed.modules.len());
        trace_compiler_phase(
            trace,
            &target,
            "semantic",
            Some(phase_started.elapsed()),
            Some(format!("{} modules", analyzed.modules.len())),
        );
        Ok(analyzed)
    }

    /// Compile already-preprocessed Verilog-A source to canonical IR with
    /// caller-provided source package metadata.
    fn compile_canonical_ir_preprocessed_with_metadata_measured(
        &self,
        source_package: &str,
        source: &str,
        module_name: Option<&str>,
        measurements: &mut metrics::MetricsRecorder,
    ) -> CompileResult<canonical_ir::CanonicalIrArtifact> {
        let analyzed = self.analyze_preprocessed(source_package, source, measurements)?;
        self.build_canonical_ir_artifact(
            source_package,
            source,
            &analyzed,
            module_name,
            measurements,
        )
    }

    fn build_canonical_ir_artifact(
        &self,
        source_package: &str,
        source: &str,
        analyzed: &semantic::AnalyzedFile,
        module_name: Option<&str>,
        measurements: &mut metrics::MetricsRecorder,
    ) -> CompileResult<canonical_ir::CanonicalIrArtifact> {
        let module = self.select_analyzed_module(analyzed, module_name)?;
        let trace = compiler_phase_trace_enabled();
        let metadata = canonical_ir::CanonicalMetadata::for_source(source_package, source);
        measurements.checkpoint(PipelinePhase::HirLowering)?;
        trace_canonical_ir_phase(trace, &module.name, "hir", None);
        let phase_started = web_time::Instant::now();
        let mut hir = canonical_ir::HirModel::from_analyzed_module(&metadata, module);
        measurements.record(PipelinePhase::HirLowering, phase_started.elapsed())?;
        trace_canonical_ir_phase(trace, &module.name, "hir", Some(phase_started.elapsed()));
        measurements.checkpoint(PipelinePhase::MirLowering)?;
        trace_canonical_ir_phase(trace, &module.name, "mir", None);
        let phase_started = web_time::Instant::now();
        let mut mir = canonical_ir::MirModel::from_hir(&hir).map_err(Self::canonical_ir_error)?;
        measurements.record(PipelinePhase::MirLowering, phase_started.elapsed())?;
        trace_canonical_ir_phase(trace, &module.name, "mir", Some(phase_started.elapsed()));
        measurements.checkpoint(PipelinePhase::CanonicalNoisePlanning)?;
        let phase_started = web_time::Instant::now();
        let noise_sources =
            canonical_ir::CanonicalNoiseSourcePlan::from_hir_and_mir(&mut hir, &mut mir)
                .map_err(Self::canonical_ir_error)?;
        measurements.record(
            PipelinePhase::CanonicalNoisePlanning,
            phase_started.elapsed(),
        )?;
        measurements.checkpoint(PipelinePhase::IntegrityValidation)?;
        let phase_started = web_time::Instant::now();
        let artifact = canonical_ir::CanonicalIrArtifact::from_parts_with_noise_plan(
            metadata,
            hir,
            mir,
            noise_sources,
        )
        .map_err(Self::canonical_ir_error)?;
        measurements.record(PipelinePhase::IntegrityValidation, phase_started.elapsed())?;
        Ok(artifact)
    }

    /// Resolve which analyzed module to compile.
    fn select_analyzed_module<'a>(
        &self,
        analyzed: &'a semantic::AnalyzedFile,
        module_name: Option<&str>,
    ) -> CompileResult<&'a semantic::AnalyzedModule> {
        // The modules map iterates in arbitrary order; list candidates in
        // declaration order so diagnostics are deterministic.
        let declared: Vec<&str> = analyzed
            .source
            .items
            .iter()
            .filter_map(|item| match item {
                ast::Item::Module(module) => Some(module.name.as_str()),
                _ => None,
            })
            .collect();

        match module_name {
            Some(name) => analyzed.modules.get(name).ok_or_else(|| {
                let candidates = if declared.is_empty() {
                    "none".to_string()
                } else {
                    declared.join(", ")
                };
                CompileError::ModuleSelection(format!(
                    "module '{}' not found; the file declares: {}",
                    name, candidates
                ))
            }),
            None => match declared.as_slice() {
                [] => Err(CompileError::ModuleSelection(
                    "no modules found in source".into(),
                )),
                [name] => analyzed.modules.get(*name).ok_or_else(|| {
                    error::CodeGenError::new(error::CodeGenErrorKind::Internal(format!(
                        "module '{}' was parsed but not analyzed",
                        name
                    )))
                    .into()
                }),
                names => Err(CompileError::ModuleSelection(format!(
                    "the file declares multiple modules: {}; select one by name",
                    names.join(", ")
                ))),
            },
        }
    }

    fn canonical_ir_error(diagnostics: Vec<canonical_ir::IrDiagnostic>) -> CompileError {
        let details = diagnostics
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("; ");
        CompileError::CodeGen(error::CodeGenError::new(error::CodeGenErrorKind::Internal(
            format!("canonical IR validation failed: {details}"),
        )))
    }

    /// Compile a source file from disk with preprocessing and dependency metadata.
    ///
    /// The file must contain exactly one module; multi-module files
    /// require [`Self::compile_file_module_with_metadata`].
    pub fn compile_file_with_metadata(
        &self,
        path: &std::path::Path,
    ) -> CompileResult<CompiledFile> {
        self.compile_file_module_with_metadata(path, None)
    }

    /// Compile one module of a source file from disk with preprocessing
    /// and dependency metadata. See [`Self::compile_module`] for the
    /// module selection rules.
    pub fn compile_file_module_with_metadata(
        &self,
        path: &std::path::Path,
        module_name: Option<&str>,
    ) -> CompileResult<CompiledFile> {
        self.compile_file_module_with_metadata_and_control(path, module_name, &NoPipelineControl)
    }

    /// Cancellable, progress-observable file compilation.
    pub fn compile_file_module_with_metadata_and_control(
        &self,
        path: &std::path::Path,
        module_name: Option<&str>,
        control: &dyn PipelineControl,
    ) -> CompileResult<CompiledFile> {
        let input_bytes = std::fs::metadata(path)
            .ok()
            .and_then(|metadata| usize::try_from(metadata.len()).ok())
            .unwrap_or(0);
        let mut measurements = metrics::MetricsRecorder::with_control(
            input_bytes,
            self.options.performance_budget.clone(),
            control,
        );
        let mut pp = self.configured_preprocessor();

        // Preprocess the file (handles `include, `define, `ifdef, etc.)
        measurements.checkpoint(PipelinePhase::Preprocess)?;
        let phase_started = web_time::Instant::now();
        let preprocessed = pp
            .preprocess_file(path)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;
        let dependencies = pp.take_dependencies();
        measurements.record(PipelinePhase::Preprocess, phase_started.elapsed())?;
        measurements.metrics_mut().preprocessed_bytes = metrics::usize_to_u64(preprocessed.len());
        measurements.metrics_mut().dependency_count = metrics::usize_to_u64(dependencies.len());

        // DEBUG: Dump preprocessed content to file for debugging
        if std::env::var("RSPICE_DEBUG_PP").is_ok() {
            let debug_path = path.with_extension("pp.va");
            let _ = std::fs::write(&debug_path, &preprocessed);
            eprintln!(
                "DEBUG: Preprocessed output written to {}",
                debug_path.display()
            );
        }

        // Compile the preprocessed source
        let model =
            self.compile_preprocessed_measured(&preprocessed, module_name, &mut measurements)?;
        Ok(CompiledFile {
            model,
            dependencies,
            metrics: measurements.finish(),
        })
    }

    /// Compile one module of a source file from disk to canonical IR with
    /// preprocessing and dependency metadata.
    ///
    /// See [`Self::compile_module`] for the module selection rules.
    pub fn compile_file_canonical_ir_with_metadata(
        &self,
        path: &std::path::Path,
        module_name: Option<&str>,
    ) -> CompileResult<CanonicalIrFile> {
        self.compile_file_canonical_ir_with_metadata_and_control(
            path,
            module_name,
            &NoPipelineControl,
        )
    }

    /// Cancellable, progress-observable canonical-IR file compilation.
    pub fn compile_file_canonical_ir_with_metadata_and_control(
        &self,
        path: &std::path::Path,
        module_name: Option<&str>,
        control: &dyn PipelineControl,
    ) -> CompileResult<CanonicalIrFile> {
        let trace = compiler_phase_trace_enabled();
        let trace_target = format!("Verilog-A compiler {}", path.display());
        let input_bytes = std::fs::metadata(path)
            .ok()
            .and_then(|metadata| usize::try_from(metadata.len()).ok())
            .unwrap_or(0);
        let mut measurements = metrics::MetricsRecorder::with_control(
            input_bytes,
            self.options.performance_budget.clone(),
            control,
        );
        let mut pp = self.configured_preprocessor();

        measurements.checkpoint(PipelinePhase::Preprocess)?;
        trace_compiler_phase(trace, &trace_target, "preprocess", None, None);
        let phase_started = web_time::Instant::now();
        let preprocessed = pp
            .preprocess_file(path)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;
        let dependencies = pp.take_dependencies();
        measurements.record(PipelinePhase::Preprocess, phase_started.elapsed())?;
        measurements.metrics_mut().preprocessed_bytes = metrics::usize_to_u64(preprocessed.len());
        measurements.metrics_mut().dependency_count = metrics::usize_to_u64(dependencies.len());
        trace_compiler_phase(
            trace,
            &trace_target,
            "preprocess",
            Some(phase_started.elapsed()),
            Some(format!(
                "{} bytes, {} dependencies",
                preprocessed.len(),
                dependencies.len()
            )),
        );
        let source_package_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        if std::env::var("RSPICE_DEBUG_PP").is_ok() {
            let debug_path = path.with_extension("pp.va");
            let _ = std::fs::write(&debug_path, &preprocessed);
            eprintln!(
                "DEBUG: Preprocessed output written to {}",
                debug_path.display()
            );
        }

        // Keep canonical IR metadata aligned with the root file, not the
        // lexicographic dependency order used by the preprocessor.
        let source_package = source_package_path.display().to_string();
        let artifact = self.compile_canonical_ir_preprocessed_with_metadata_measured(
            &source_package,
            &preprocessed,
            module_name,
            &mut measurements,
        )?;

        Ok(CanonicalIrFile {
            artifact,
            dependencies,
            metrics: measurements.finish(),
        })
    }

    /// Compile one module of a source file from disk to both runtime artifacts
    /// with preprocessing and dependency metadata.
    ///
    /// This API preprocesses, parses, and analyzes the file once, then emits the
    /// compiled model and canonical IR artifact from the same selected module.
    /// See [`Self::compile_module`] for the module selection rules.
    pub fn compile_file_runtime_with_metadata(
        &self,
        path: &std::path::Path,
        module_name: Option<&str>,
    ) -> CompileResult<CompiledRuntimeFile> {
        self.compile_file_runtime_with_metadata_and_control(path, module_name, &NoPipelineControl)
    }

    /// Cancellable, progress-observable file compilation of both runtime
    /// artifacts from one front-end pass.
    pub fn compile_file_runtime_with_metadata_and_control(
        &self,
        path: &std::path::Path,
        module_name: Option<&str>,
        control: &dyn PipelineControl,
    ) -> CompileResult<CompiledRuntimeFile> {
        let input_bytes = std::fs::metadata(path)
            .ok()
            .and_then(|metadata| usize::try_from(metadata.len()).ok())
            .unwrap_or(0);
        let mut measurements = metrics::MetricsRecorder::with_control(
            input_bytes,
            self.options.performance_budget.clone(),
            control,
        );
        let mut pp = self.configured_preprocessor();

        measurements.checkpoint(PipelinePhase::Preprocess)?;
        let phase_started = web_time::Instant::now();
        let preprocessed = pp
            .preprocess_file(path)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;
        let dependencies = pp.take_dependencies();
        measurements.record(PipelinePhase::Preprocess, phase_started.elapsed())?;
        measurements.metrics_mut().preprocessed_bytes = metrics::usize_to_u64(preprocessed.len());
        measurements.metrics_mut().dependency_count = metrics::usize_to_u64(dependencies.len());
        let source_package_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        if std::env::var("RSPICE_DEBUG_PP").is_ok() {
            let debug_path = path.with_extension("pp.va");
            let _ = std::fs::write(&debug_path, &preprocessed);
            eprintln!(
                "DEBUG: Preprocessed output written to {}",
                debug_path.display()
            );
        }

        let source_package = source_package_path.display().to_string();
        let analyzed =
            self.analyze_preprocessed(&source_package, &preprocessed, &mut measurements)?;
        let source_digest = canonical_ir::StableDigest::from_text(&preprocessed).as_hex();
        measurements.checkpoint(PipelinePhase::BytecodeGeneration)?;
        let phase_started = web_time::Instant::now();
        let model = CodeGenerator::new().generate_module_with_source_digest(
            &analyzed,
            module_name,
            source_digest,
        )?;
        measurements.record(PipelinePhase::BytecodeGeneration, phase_started.elapsed())?;
        let canonical_ir = self.build_canonical_ir_artifact(
            &source_package,
            &preprocessed,
            &analyzed,
            module_name,
            &mut measurements,
        )?;

        Ok(CompiledRuntimeFile {
            model,
            canonical_ir,
            dependencies,
            metrics: measurements.finish(),
        })
    }

    /// Compile a source file from disk with preprocessing
    ///
    /// The file must contain exactly one module; multi-module files
    /// require [`Self::compile_file_module`].
    pub fn compile_file(&self, path: &std::path::Path) -> CompileResult<CompiledModel> {
        self.compile_file_with_metadata(path)
            .map(|compiled| compiled.model)
    }

    /// Compile one module of a source file from disk. See
    /// [`Self::compile_module`] for the module selection rules.
    pub fn compile_file_module(
        &self,
        path: &std::path::Path,
        module_name: Option<&str>,
    ) -> CompileResult<CompiledModel> {
        self.compile_file_module_with_metadata(path, module_name)
            .map(|compiled| compiled.model)
    }
}

impl Default for VerilogACompiler {
    fn default() -> Self {
        Self::new(CompilerOptions::default())
    }
}
