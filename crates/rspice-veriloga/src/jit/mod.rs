//! Architecture-neutral JIT lowering and optimization pipeline.
//!
//! Every executable backend consumes these exact plans. Target backends may
//! choose different ABIs, instruction encodings, and publication mechanisms,
//! but they must not maintain independent semantic lowering implementations.

#[path = "../native/assignment.rs"]
pub(crate) mod assignment;
#[path = "../native/cfg_lanes.rs"]
pub(crate) mod cfg_lanes;
#[path = "../native/cfg_program.rs"]
pub(crate) mod cfg_program;
pub(crate) mod coverage;
pub(crate) mod current_dependencies;
#[path = "../native/error.rs"]
mod error;
#[path = "../native/expr.rs"]
pub(crate) mod expr;
#[path = "../native/model_plan.rs"]
pub(crate) mod model_plan;
pub(crate) mod plan_builder;
#[path = "../native/ssa.rs"]
pub(crate) mod ssa;
#[path = "../native/value_cache.rs"]
pub(crate) mod value_cache;

pub use error::{JitError, JitResult};
