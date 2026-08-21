//! Code Generator for Verilog-A
//!
//! Generates simulator-ready device models from IR.
//! Produces bytecode programs for efficient simulation.

use crate::ast::BinaryOp;
use crate::error::{CodeGenError, CodeGenErrorKind, CompileError, CompileResult};
use crate::ir::{BranchEquation, DerivativeWrt, DeviceIR, IrExpr, IrFunction};
use crate::laplace::StateSpaceFilter;
use crate::semantic::{AnalyzedFile, AnalyzedModule};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

mod generator;
mod lookup_table;
mod model;

pub use model::{
    AssignmentProgram, AssignmentStep, BytecodeProgram, CodeGenerator, ColumnAxis,
    CompiledBranchSource, CompiledModel, CompiledNoiseSource, CompiledParameter, Instruction,
    JacobianEntry, LookupTable, StampIndex, StampLocation, StampProgram,
};
