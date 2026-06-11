//! # RSpice Verilog-A/AMS Compiler
//!
//! A full Verilog-A Language Reference Manual (LRM) 2.4 compliant compiler
//! with Verilog-AMS mixed-signal support for RSpice circuit simulator.
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
//!   `$limit`, `$table_model`
//! - Noise source declarations (`white_noise`, `flicker_noise`) - parsed
//!   and carried in the IR; noise-analysis integration is pending
//! - System functions: `$temperature`, `$vt`, `$abstime`, `$simparam`,
//!   `$param_given`, `$port_connected`, `$mfactor`
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
//! - Noise-analysis integration of declared noise sources is pending
//! - Z-domain (`zi_*`) filters, `noise_table`, indirect contributions,
//!   array-valued variables in expressions

pub mod ast;
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
    /// identically whether compiling from a string or from a file.
    pub fn compile(&self, source: &str) -> CompileResult<CompiledModel> {
        let mut pp = self.configured_preprocessor();
        let preprocessed = pp
            .preprocess_source(source)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;
        self.compile_preprocessed(&preprocessed)
    }

    /// Compile already-preprocessed Verilog-A source.
    fn compile_preprocessed(&self, source: &str) -> CompileResult<CompiledModel> {
        // Phase 1: Lexical analysis
        let source_map = SourceMap::new();
        let source_id = source_map.add_source("<input>", source);
        let tokens = Lexer::new(source, source_id).collect_tokens()?;

        // Phase 2: Parsing
        let source_file = Parser::new(&tokens).parse()?;

        // Phase 3: Semantic analysis
        let analyzed = SemanticAnalyzer::new().analyze(&source_file)?;

        // Phase 4 & 5: IR generation and code generation
        let model = CodeGenerator::new().generate(&analyzed)?;

        Ok(model)
    }

    /// Compile a source file from disk with preprocessing and dependency metadata.
    pub fn compile_file_with_metadata(
        &self,
        path: &std::path::Path,
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
        let model = self.compile_preprocessed(&preprocessed)?;
        Ok(CompiledFile {
            model,
            dependencies,
        })
    }

    /// Compile a source file from disk with preprocessing
    pub fn compile_file(&self, path: &std::path::Path) -> CompileResult<CompiledModel> {
        self.compile_file_with_metadata(path)
            .map(|compiled| compiled.model)
    }
}

impl Default for VerilogACompiler {
    fn default() -> Self {
        Self::new(CompilerOptions::default())
    }
}
