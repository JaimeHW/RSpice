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
//! ## Verilog-A LRM 2.4 Compliance
//!
//! This compiler implements the full Verilog-A Language Reference Manual 2.4,
//! including:
//!
//! - All analog operators (`ddt`, `idt`, `ddx`, `limexp`, `absdelay`, etc.)
//! - Noise functions (`white_noise`, `flicker_noise`, `noise_table`)
//! - All system functions (`$temperature`, `$vt`, `$abstime`, etc.)
//! - Parameter declarations with ranges and defaults
//! - Hierarchical modules and instances
//! - Conditional and event-driven analog statements
//!
//! ## Verilog-AMS Support
//!
//! With the `ams` feature enabled, the compiler also supports:
//!
//! - Digital modules and primitives
//! - Mixed-signal connect modules
//! - Cross-domain event detection

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

/// End-to-end integration tests
#[cfg(test)]
mod integration_tests;

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

    /// Create a compiler with default options
    pub fn default() -> Self {
        Self::new(CompilerOptions::default())
    }

    /// Compile Verilog-A source code to a device model
    pub fn compile(&self, source: &str) -> CompileResult<CompiledModel> {
        // Phase 1: Lexical analysis
        let source_map = SourceMap::new();
        let source_id = source_map.add_source("<input>", source);
        let tokens = Lexer::new(source, source_id).collect_tokens()?;

        // Phase 2: Parsing
        let source_file = Parser::new(&tokens, &source_map).parse()?;

        // Phase 3: Semantic analysis
        let analyzed = SemanticAnalyzer::new(&self.options).analyze(&source_file)?;

        // Phase 4 & 5: IR generation and code generation
        let model = CodeGenerator::new(&self.options).generate(&analyzed)?;

        Ok(model)
    }

    /// Compile a source file from disk with preprocessing
    pub fn compile_file(&self, path: &std::path::Path) -> CompileResult<CompiledModel> {
        // Create preprocessor with options
        let mut pp = Preprocessor::new();

        // Add include paths from options
        for inc_path in &self.options.include_paths {
            pp.add_include_path(inc_path);
        }

        // Add defines from options
        for (name, value) in &self.options.defines {
            let def = preprocessor::MacroDef::simple(value.as_deref().unwrap_or(""));
            pp.define(name, def);
        }

        // Preprocess the file (handles `include, `define, `ifdef, etc.)
        let preprocessed = pp
            .preprocess_file(path)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;

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
        self.compile(&preprocessed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_creation() {
        let compiler = VerilogACompiler::default();
        assert!(!compiler.options.enable_ams);
        assert!(!compiler.options.strict_mode);
    }

    #[test]
    fn test_options_builder() {
        let options = CompilerOptions {
            enable_ams: true,
            strict_mode: true,
            integration_order: IntegrationOrder::First,
            ..Default::default()
        };
        assert!(options.enable_ams);
        assert!(options.strict_mode);
        assert_eq!(options.integration_order, IntegrationOrder::First);
    }
}
