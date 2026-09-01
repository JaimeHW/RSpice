//! # RSpice Core
//!
//! The SPICE circuit simulation engine: netlist parsing, device models,
//! sparse matrix assembly, Newton-Raphson solving, and the analysis
//! algorithms. Every other crate in the workspace — the CLI, the GUI, the
//! Python bindings, the WASM bindings — is a frontend over this one.
//!
//! ## Pipeline
//!
//! A deck flows through four stages: [`netlist`] parses text into an AST,
//! [`engine`]'s builder flattens subcircuits and constructs devices into
//! [`circuit`]'s struct-of-arrays storage, [`solver`] freezes a sparsity
//! pattern once and reuses it, and [`engine`] drives the Newton loop to
//! produce [`analysis`] result types.
//!
//! ## Modules
//!
//! - [`netlist`] — lexer, parser, AST, subcircuit flattening, `.include`
//!   resolution, parameter scoping
//! - [`circuit`] — [`CircuitData`] struct-of-arrays storage, stamping,
//!   magnetic coupling, introspection
//! - [`device`] — device models: passives, diodes, BJTs, the MOSFET/FET
//!   family, sources, transmission lines, memristors, Verilog-A and FFI
//!   extension points
//! - [`solver`] — sparse LU (KLU-class and faer backends), Newton-Raphson,
//!   convergence checking, damping, arc-length continuation
//! - [`engine`] — the orchestrator: one driver per analysis family, matrix
//!   assembly, configuration, convergence aids
//! - [`analysis`] — analysis algorithms and result types, `.MEAS`
//!   evaluation, and post-processing
//! - [`expr`] — expression parser, bytecode compiler, and VM for behavioral
//!   sources and parameters
//! - [`library`] — `.lib` model-library parsing and Verilog-A pack discovery
//! - [`xspice`] — XSPICE code-model subsystem and bundled models
//! - [`io`] — SPICE RAW waveform files, read and written
//! - [`constants`] — physical and simulation constants
//! - [`diagnostics`] — structured warnings and convergence-quality metrics
//! - [`abort_signal`] — cooperative cancellation for long runs
//! - [`resource`] — resource limits for untrusted input
//! - [`time_compat`] — wall-clock shim (`wasm32` has no clock)
//! - [`simd`] — SIMD kernels, with the `simd` feature
//!
//! ## Example
//!
//! ```rust
//! use rspice_core::{Engine, Netlist};
//!
//! // The first line of a SPICE deck is the title, never an element.
//! let netlist = Netlist::parse("divider\nV1 1 0 10\nR1 1 0 1k\n.end")?;
//! let result = Engine::default().run_dc_op(&netlist)?;
//! assert_eq!(result.voltage(1), 10.0);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! [`SimulationConfig`], [`ConvergenceConfig`], [`ConvergencePreset`], and
//! [`DampingStrategy`] configure the engine. [`AbortSignal`] lets a frontend
//! cancel a long transient or sweep cooperatively — this is what backs
//! Ctrl-C in the CLI and `KeyboardInterrupt` in the Python bindings.
//!
//! ## Feature flags
//!
//! `parallel`, `simd`, and `faer-parallel` are on by default. Verilog-A
//! support (`veriloga`, `veriloga-native`, the generated `veriloga-model-*`
//! built-ins), `wasm`, and `ffi` are opt-in. See the crate README for the
//! full table.
//!
//! ## Testing
//!
//! Unit tests are excluded from the default package test target by
//! `[lib] test = false`; run them with `cargo test -p rspice-core --lib`.
//! Doctests are likewise off (`doctest = false`), so examples here are
//! checked by review rather than by `cargo test`.

#![allow(
    clippy::approx_constant,
    clippy::collapsible_if,
    clippy::derivable_impls,
    clippy::field_reassign_with_default,
    clippy::filter_map_bool_then,
    clippy::if_same_then_else,
    clippy::items_after_test_module,
    clippy::manual_contains,
    clippy::manual_is_multiple_of,
    clippy::manual_range_patterns,
    clippy::manual_strip,
    clippy::needless_lifetimes,
    clippy::needless_pub_self,
    clippy::needless_range_loop,
    clippy::never_loop,
    clippy::single_element_loop,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unnecessary_cast,
    clippy::unnecessary_map_or,
    clippy::useless_format
)]

pub mod analysis;
/// Starter model cards embedded in the binary; a leaf with no dependencies so
/// that library, netlist and engine can all share one copy.
pub(crate) mod builtin_lib;
pub mod circuit;
/// Names of the XSPICE code models compiled in; a leaf so that both the parser
/// that classifies a name and the subsystem that instantiates it read down.
pub(crate) mod codemodels;
/// Simulation configuration. Below everything that reads it, so a tolerance
/// or dialect flag is a downward reference rather than a reach into `engine`.
pub mod config;
pub mod constants;
pub mod device;
pub mod diagnostics;
pub mod engine;
pub mod execution;
pub mod expr;
pub mod io;
pub mod library;
/// SPICE naming rules: what a name means in a deck, independent of dialect.
/// A leaf — everything else depends on it, so it depends on nothing.
pub(crate) mod naming;
pub mod netlist;
pub mod numerics;
/// The vocabulary of labels a device operating-point report may carry. A leaf:
/// the device families that emit one and the circuit store that assembles the
/// report both read down into it.
pub mod op_label;
pub mod solver;
mod spice_number;
pub mod time_compat;
pub mod xspice;

/// Abort signal for cancelling long-running simulations
pub mod abort_signal;
pub mod resource;

/// SIMD-accelerated operations (optional, requires `simd` feature)
#[cfg(feature = "simd")]
pub mod simd;

// Re-export primary types for convenience
pub use abort_signal::{AbortSignal, AtomicAbort, NoAbort};
pub use analysis::{DcAnalysis, MeasureEngine, MeasureResult};
pub use circuit::CircuitData;
pub use device::{Device, DeviceModel, engine_capabilities, engine_supports_capability};
pub use engine::{
    ConvergenceConfig, ConvergencePreset, DampingStrategy, Engine, EngineHealthReport,
    JfetLevel2Model, RequestedSignalUnavailableError, ResultSchemaMismatchError, SimulationConfig,
    SimulationConfigError, SimulationConfigOverrides, SimulationError, SimulationErrorCategory,
    SimulationErrorCode, SimulationErrorDescriptor, SpiceDialect, XyceTraInterpolation,
    resolve_simulation_config,
};
#[cfg(feature = "veriloga")]
pub use engine::{
    ProjectVerilogARuntimeRegistration, VerilogACacheEntry, VerilogACachePruneReport,
    VerilogACacheStats, VerilogACacheTelemetry, clear_veriloga_cache, prune_veriloga_cache,
    register_precompiled_veriloga_model, register_precompiled_veriloga_model_with_dependencies,
    register_precompiled_veriloga_runtime_with_dependencies,
    register_project_veriloga_runtime_for_session, register_project_veriloga_runtimes_for_session,
    register_project_veriloga_runtimes_for_session_with_limits, veriloga_cache_entries,
    veriloga_cache_stats, veriloga_cache_telemetry,
};
pub use netlist::Netlist;
pub use resource::{ResourceKind, ResourceLimitError, ResourceLimits};
pub use solver::{SimulationResult, Simulator, SparseLuSolver, StaticMatrix, TripletMatrix};
pub use xspice::{CmContext, CodeModel, CodeModelRegistry, XspiceInstance};

/// Error types for the simulation engine
pub mod error {
    pub use crate::circuit::CircuitError;
    pub use crate::netlist::ParseError;
    pub use crate::solver::SolverError;
}

/// Simulation value type (using f64 for high precision)
pub type Value = f64;

/// Node identifier (0 = ground, always).
///
/// Beside [`Value`] because it is the same kind of thing: the crate's shared
/// vocabulary, named by every layer and owned by none. It sat in `circuit`,
/// two ranks above the device models that index with it, so thirty-eight
/// device files reached up for an alias to `usize`.
pub type NodeId = usize;

/// Complex value type for AC analysis
/// Re-export from num_complex for convenience
pub use num_complex::Complex64;

/// Type alias for complex values used in AC analysis
pub type ComplexValue = Complex64;
