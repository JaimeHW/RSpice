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
//!   family, sources, transmission lines, memristors, and Verilog-A devices
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
//! `parallel`, `simd`, and `faer-parallel` are on by default. `wasm` and the
//! Verilog-A features are opt-in: `veriloga`, `veriloga-native`,
//! `veriloga-wasm-jit`, and the generated built-ins (`veriloga-builtins-base`,
//! one `veriloga-model-*` flag per model, `veriloga-builtins-noise`, and the
//! `veriloga-builtins-models` and `veriloga-builtins` umbrellas). The crate
//! README's feature-flag table says what each one does.
//!
//! ## Testing
//!
//! Unit tests are excluded from the default package test target by
//! `[lib] test = false`; run them with `cargo test -p rspice-core --lib`.
//! Doctests are likewise off (`doctest = false`), so examples here are
//! checked by review rather than by `cargo test`.

pub mod analysis;
/// Starter model cards embedded in the binary; a leaf with no dependencies so
/// that library, netlist and engine can all share one copy.
pub(crate) use rspice_core_base::builtin_lib;
/// The typed refusal of a well-formed construct this build does not execute.
/// Below configuration and XSPICE, which raise one; published through
/// `analysis::error` and the crate root.
use rspice_core_base::capability;
pub mod circuit;
/// Names of the XSPICE code models compiled in; a leaf so that both the parser
/// that classifies a name and the subsystem that instantiates it read down.
pub(crate) use rspice_core_base::codemodels;
/// Simulation configuration. Below everything that reads it, so a tolerance
/// or dialect flag is a downward reference rather than a reach into `engine`.
pub use rspice_core_base::config;
pub use rspice_core_base::constants;
/// Shared vocabulary between the control-session machine and its engine host.
use rspice_core_netlist::control_protocol;
/// Device models. They compile in the family's devices crate; the compiled
/// Verilog-A device needs the compiler, so it is this crate's.
pub mod device {
    pub use rspice_core_devices::device::*;
    #[cfg(feature = "veriloga")]
    pub mod veriloga;
}
pub use rspice_core_base::diagnostics;
pub mod engine;
/// The names an event-only net is refused and published under.
use rspice_core_base::event_net;
pub mod execution;
pub use rspice_core_base::expr;
#[doc(hidden)]
pub use rspice_core_base::family_features;
/// Stable analysis and run-coordinate identities, named by both the engine's
/// error taxonomy and the deck planner. A leaf, so neither has to reach for
/// the other to say which analysis or sweep point a value belongs to.
pub use rspice_core_base::identity;
pub use rspice_core_base::io;
/// Which run a log record belongs to, carried onto the engine's worker threads.
/// A leaf: the analyses that hand work to another thread read down into it.
pub use rspice_core_base::log_scope;
/// SPICE naming rules: what a name means in a deck, independent of dialect.
/// A leaf — everything else depends on it, so it depends on nothing.
pub(crate) use rspice_core_base::naming;
/// What a run raised and went on to complete with, in a shape a reader can
/// act on: the advisories the log states as prose.
pub use rspice_core_base::notice;
pub use rspice_core_base::numerics;
/// The vocabulary of labels a device operating-point report may carry. A leaf:
/// the device families that emit one and the circuit store that assembles the
/// report both read down into it.
pub use rspice_core_base::op_label;
/// Typed per-analysis scalars, carried by analysis results, control-script
/// PRINT and result documents alike. Published under
/// `execution::result_document`.
use rspice_core_base::result_scalar;
/// Physical units carried by result schemas and control-expression vectors.
use rspice_core_base::signal_unit;
pub use rspice_core_base::solver;
use rspice_core_base::spice_number;
pub use rspice_core_base::time_compat;
use rspice_core_base::transient_observation;
pub use rspice_core_netlist::library;
pub use rspice_core_netlist::netlist;
pub mod xspice;

/// Abort signal for cancelling long-running simulations
pub use rspice_core_base::abort_signal;
pub use rspice_core_base::resource;

// Re-export primary types for convenience
pub use abort_signal::{
    AbortReason, AbortSignal, AtomicAbort, ModelFinish, ModelFinishPoint, NoAbort,
    SimulationOutcome,
};
pub use analysis::{DcAnalysis, MeasureEngine, MeasureResult};
pub use circuit::CircuitData;
pub use device::{engine_capabilities, engine_supports_capability};
pub use engine::{
    ContinuationAid, ContinuationFailure, ContinuationStage, ConvergenceConfig, ConvergencePreset,
    DampingStrategy, ElaborationError, ElaborationErrorKind, Engine, EngineHealthReport,
    GpTransientPhaseModel, JfetLevel2Model, MaterializationMismatchError, OutputCommitError,
    OutputCommitPhase, PersistenceIncompatibleError, RequestedSignalUnavailableError,
    ResultSchemaMismatchError, SimulationConfig, SimulationConfigError, SimulationConfigOverrides,
    SimulationError, SimulationErrorCategory, SimulationErrorCode, SimulationErrorDescriptor,
    SpiceDialect, UnsupportedCapabilityError, XyceTraInterpolation, resolve_simulation_config,
};
#[cfg(feature = "veriloga")]
pub use engine::{
    HeldVerilogASources, ProjectVerilogAConnectionLibraryRegistration,
    ProjectVerilogARuntimeRegistration, ProjectVerilogASourceRegistration, VerilogACacheTelemetry,
    hold_project_veriloga_sources_for_session, register_precompiled_veriloga_model,
    register_precompiled_veriloga_model_with_dependencies,
    register_precompiled_veriloga_runtime_with_dependencies, veriloga_cache_telemetry,
};
pub use netlist::Netlist;
pub use resource::{ResourceKind, ResourceLimitError, ResourceLimits};
pub use solver::{SimulationResult, SparseLuSolver, StaticMatrix, TripletMatrix};
pub use transient_observation::{
    CurrentImpulseDerivative, CurrentImpulseOwner, CurrentImpulsePoint, CurrentImpulseTrace,
    ImpulseDerivative, ImpulsePointValue, ImpulseTraceRef, VoltageImpulseDerivative,
    VoltageImpulsePoint, VoltageImpulseTrace,
};
pub use xspice::{CmContext, CodeModel, CodeModelRegistry, XspiceInstance};

family_features!(rspice_core_base, rspice_core_netlist, rspice_core_devices);

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

/// `family_features!` is defined, and tests its own table, in the family's
/// lowest crate. These hold this crate's manifest and root to that table.
#[cfg(test)]
mod family_features_tests {
    use std::collections::BTreeSet;

    /// Every feature this crate's manifest declares, `default` and
    /// `test-support` aside: neither is a flag.
    fn manifest_features() -> BTreeSet<String> {
        let mut in_features = false;
        let mut features = BTreeSet::new();
        for line in include_str!("../Cargo.toml").lines() {
            if line.starts_with('[') {
                in_features = line == "[features]";
            } else if in_features && line.starts_with(|c: char| c.is_ascii_lowercase()) {
                let (name, _) = line.split_once(" = ").expect("a feature definition");
                features.insert(name.to_owned());
            }
        }
        features.remove("default");
        features.remove("test-support");
        features
    }

    /// Every flag `FAMILY_FEATURES` states, spelled as the feature it reads.
    fn stated_flags() -> BTreeSet<String> {
        let stated = format!("{:?}", crate::FAMILY_FEATURES);
        let fields = stated
            .strip_prefix("FamilyFeatures { ")
            .and_then(|fields| fields.strip_suffix(" }"))
            .expect("the derived `Debug` form of `FamilyFeatures`");
        fields
            .split(", ")
            .map(|field| field.split_once(": ").expect("a `flag: bool` field").0)
            .map(|flag| flag.replace('_', "-"))
            .collect()
    }

    #[test]
    fn every_feature_of_the_manifest_has_one_flag_named_for_it() {
        assert_eq!(stated_flags(), manifest_features());
    }

    #[test]
    fn the_crate_root_states_the_features_of_this_build_as_base_does() {
        let stated = crate::FAMILY_FEATURES;
        assert_eq!(stated, rspice_core_base::FAMILY_FEATURES);
        assert_eq!(stated.parallel, cfg!(feature = "parallel"));
        assert_eq!(stated.simd, cfg!(feature = "simd"));
        assert_eq!(stated.veriloga, cfg!(feature = "veriloga"));
        assert_eq!(stated.veriloga_native, cfg!(feature = "veriloga-native"));
        assert_eq!(
            stated.veriloga_builtins,
            cfg!(feature = "veriloga-builtins")
        );
    }
}
