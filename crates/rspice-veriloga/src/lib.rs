//! # RSpice Verilog-A/AMS Compiler
//!
//! A Verilog-A compiler targeting the supported analog subset of the
//! Verilog-AMS Language Reference Manual (LRM) 2.4 for RSpice.
//!
//! ## Architecture
//!
//! The compiler is organized into the following stages:
//!
//! 1. **Lexical Analysis** ([`lexer`]) - Tokenizes Verilog-A/AMS source code
//! 2. **Parsing** ([`parser`]) - Produces Abstract Syntax Tree from tokens
//! 3. **Semantic Analysis** ([`semantic`]) - Type checking and symbol resolution
//! 4. **IR Generation** ([`ir`]) - Device equations and auto-differentiation
//! 5. **Code Generation** ([`codegen`]) - Generates simulator-ready device models
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
//! ## Verilog-A Language Support
//!
//! Targets the Verilog-A subset of the Verilog-AMS LRM 2.4. Currently
//! supported:
//!
//! - Analog operators: `ddt`, `idt`, `idtmod` (backward Euler), `ddx`,
//!   `limexp`, `absdelay`, `transition`, `slew`, `laplace_zp/zd/np/nd`,
//!   `zi_nd/zp/zd/np` (sampled-data), `$limit`, `$table_model`
//! - Noise sources (`white_noise`, `flicker_noise`, `noise_table`,
//!   `noise_table_log`) injected into `.noise` with amplitude scaling and
//!   mode gating
//! - Indirect contributions (`V(x): lhs == rhs`) as constraint rows on a
//!   branch unknown
//! - System functions: `$temperature`, `$vt`, `$abstime`, `$simparam`,
//!   `$param_given`, `$port_connected`, `$mfactor` (with automatic
//!   multiplicity scaling), `$bound_step`, `$discontinuity`
//! - 1-D array variables (compile-time and runtime indexes, shadowed
//!   derivatives), runtime-bounded loops
//! - Parameters with dependent defaults, ranges, and exclusions;
//!   localparams; attribute instances (`(* desc, units *)`)
//! - Internal nodes, named branches, ground nets, user disciplines
//!   (thermal, mechanical, ...), ANSI and non-ANSI port styles
//! - Control flow lowered to guarded dataflow: if/else, case,
//!   compile-time-bounded for/repeat loops, event controls
//!   (`initial_step`, `cross`, `above`, `timer`)
//! - User-defined analog functions (inlined)
//!
//! Known limitations (clean compile errors, never silent):
//!
//! - `noise_table` file input (inline the pair list), correlated noise
//! - Parameter-dependent `zi_*` sample periods
//! - Multi-dimensional arrays; array locals in analog functions

pub mod ast;
pub mod canonical_ir;
pub mod codegen;
pub mod disciplines;
pub mod error;
pub mod expr_converter;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod preprocessor;
pub mod semantic;
pub mod source;
pub mod stdlib;
pub mod types;

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
pub use parser::Parser;
pub use preprocessor::{Preprocessor, PreprocessorError};
pub use semantic::SemanticAnalyzer;
pub use source::{SourceId, SourceMap, Span};
pub use types::{FunctionRegistry, ParameterRange, ValueType};

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
}

/// Result of compiling a Verilog-A source file to canonical IR from disk.
///
/// Includes the canonical HIR/MIR/OptIR artifact and canonical dependency
/// paths discovered during preprocessing (`include` expansion).
#[derive(Debug, Clone)]
pub struct CanonicalIrFile {
    /// Canonical HIR/MIR/OptIR artifact for the selected module.
    pub artifact: canonical_ir::CanonicalIrArtifact,
    /// Canonical source/include dependencies captured at compile time.
    pub dependencies: Vec<std::path::PathBuf>,
}

/// Main compiler entry point
pub struct VerilogACompiler {
    options: CompilerOptions,
}

/// Compiler configuration options
#[derive(Debug, Clone, Default)]
pub struct CompilerOptions {
    /// Enable Verilog-AMS mixed-signal support
    pub enable_ams: bool,
    /// Include paths for `include directives
    pub include_paths: Vec<std::path::PathBuf>,
    /// Define macros for preprocessor
    pub defines: Vec<(String, Option<String>)>,
    /// Enable strict LRM compliance (errors on extensions)
    pub strict_mode: bool,
    /// Target integration method compatibility
    pub integration_order: IntegrationOrder,
}

/// Integration order for `idt` and `ddt` operators
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum IntegrationOrder {
    /// First-order (Gear-1, Backward Euler)
    First,
    /// Second-order (Gear-2, Trapezoidal)
    #[default]
    Second,
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
        let mut pp = self.configured_preprocessor();
        let preprocessed = pp
            .preprocess_source(source)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;
        self.compile_preprocessed(&preprocessed, module_name)
    }

    /// Compile Verilog-A source code to the canonical HIR/MIR/OptIR artifact.
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

    /// Compile one module of a Verilog-A source to canonical HIR/MIR/OptIR.
    ///
    /// See [`Self::compile_module`] for module selection rules.
    pub fn compile_canonical_ir_module(
        &self,
        source: &str,
        module_name: Option<&str>,
    ) -> CompileResult<canonical_ir::CanonicalIrArtifact> {
        let mut pp = self.configured_preprocessor();
        let preprocessed = pp
            .preprocess_source(source)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;
        self.compile_canonical_ir_preprocessed(&preprocessed, module_name)
    }

    /// Compile already-preprocessed Verilog-A source.
    fn compile_preprocessed(
        &self,
        source: &str,
        module_name: Option<&str>,
    ) -> CompileResult<CompiledModel> {
        // Phase 1: Lexical analysis
        let source_map = SourceMap::new();
        let source_id = source_map.add_source("<input>", source);
        let tokens = Lexer::new(source, source_id).collect_tokens()?;

        // Phase 2: Parsing
        let source_file = Parser::new(&tokens).parse()?;

        // Phase 3: Semantic analysis
        let analyzed = SemanticAnalyzer::new().analyze(&source_file)?;

        // Phase 4 & 5: IR generation and code generation
        let model = CodeGenerator::new().generate_module(&analyzed, module_name)?;

        Ok(model)
    }

    /// Compile already-preprocessed Verilog-A source to canonical IR.
    fn compile_canonical_ir_preprocessed(
        &self,
        source: &str,
        module_name: Option<&str>,
    ) -> CompileResult<canonical_ir::CanonicalIrArtifact> {
        self.compile_canonical_ir_preprocessed_with_metadata("<input>", source, module_name)
    }

    /// Compile already-preprocessed Verilog-A source to canonical IR with
    /// caller-provided source package metadata.
    fn compile_canonical_ir_preprocessed_with_metadata(
        &self,
        source_package: &str,
        source: &str,
        module_name: Option<&str>,
    ) -> CompileResult<canonical_ir::CanonicalIrArtifact> {
        // Phase 1: Lexical analysis
        let source_map = SourceMap::new();
        let source_id = source_map.add_source(source_package, source);
        let tokens = Lexer::new(source, source_id).collect_tokens()?;

        // Phase 2: Parsing
        let source_file = Parser::new(&tokens).parse()?;

        // Phase 3: Semantic analysis
        let analyzed = SemanticAnalyzer::new().analyze(&source_file)?;
        let module = self.select_analyzed_module(&analyzed, module_name)?;

        // Phase 4: Canonical HIR/MIR/OptIR lowering
        let metadata = canonical_ir::CanonicalMetadata::for_source(source_package, source);
        let hir = canonical_ir::HirModel::from_analyzed_module(&metadata, module);
        let mir = canonical_ir::MirModel::from_hir(&hir).map_err(Self::canonical_ir_error)?;
        let opt = canonical_ir::OptModel::from_mir(&mir).map_err(Self::canonical_ir_error)?;

        canonical_ir::CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
            .map_err(Self::canonical_ir_error)
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
        let mut pp = self.configured_preprocessor();

        // Preprocess the file (handles `include, `define, `ifdef, etc.)
        let preprocessed = pp
            .preprocess_file(path)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;
        let dependencies = pp.take_dependencies();

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
        let model = self.compile_preprocessed(&preprocessed, module_name)?;
        Ok(CompiledFile {
            model,
            dependencies,
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
        let mut pp = self.configured_preprocessor();

        let preprocessed = pp
            .preprocess_file(path)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;
        let dependencies = pp.take_dependencies();
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
        let artifact = self.compile_canonical_ir_preprocessed_with_metadata(
            &source_package,
            &preprocessed,
            module_name,
        )?;

        Ok(CanonicalIrFile {
            artifact,
            dependencies,
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
