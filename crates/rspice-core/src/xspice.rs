//! XSPICE Code Model Subsystem
//!
//! Provides mixed-signal simulation capabilities through code models.
//! This implementation follows the ngspice XSPICE specification.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                     XSPICE Subsystem                            │
//! ├─────────────────────────────────────────────────────────────────┤
//! │  CodeModelRegistry ─────► Built-in Models (gain, d_source...)   │
//! │         │                                                       │
//! │         ▼                                                       │
//! │  XspiceInstance ◄────── CmContext (runtime state)               │
//! │         │                                                       │
//! │         ▼                                                       │
//! │  Circuit Integration ──► Matrix Stamping + Event Queue         │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Code Model Types
//!
//! - **Analog Models**: Behavioral blocks (gain, summer, multiplier, etc.)
//! - **Digital Models**: Event-driven logic (gates, flip-flops, RAM)
//! - **A/D-D/A Bridges**: Interface converters between domains
//!
//! # Usage
//!
//! ```ignore
//! use rspice_core::xspice::{CodeModelRegistry, XspiceInstance};
//!
//! // Create registry with built-in models
//! let mut registry = CodeModelRegistry::new();
//! registry.register_builtins();
//!
//! // Instantiate a code model
//! let model = registry.get("gain").unwrap();
//! let instance = XspiceInstance::new("A1", model, &ports, &params);
//! ```

pub mod conformance;
mod context;
mod data_file;
mod digital;
mod event;
// The discrete-event scheduler kernel for the digital substrate.
//
// It lives here rather than under `engine` because of what the layering
// ratchet in `tests/module_layering.rs` permits: the kernel's payload is
// `EventValue`, which is this module's, and its consumers are this module's
// `instance` (rank 9) and `circuit::external_models` (rank 10). A kernel at
// `engine` (rank 12) would be an upward edge from both. Rank 9 is the only
// rank that can name `EventValue` and be named by everything that schedules.
pub mod event_scheduler;
mod external;
pub mod ifspec;
mod instance;
mod metadata;
mod registry;
// Exact counters for the settle loop's dispatch and copy-on-write structures,
// read by the `engine::xspice_settle_ratchet` CI ratchet. Rank 9 because its
// writers are this module's `instance` and `event` (rank 9) and
// `circuit::external_models` (rank 10); a counter module above either would be
// an upward edge from both.
pub(crate) mod settle_cost;
mod traits;

// Built-in code models
pub mod models;

/// Return registered virtual data-file contents for checkpoint provenance.
/// Native files are hashed as bytes by the checkpoint layer itself.
pub(crate) fn checkpoint_virtual_data_file_contents(path: &str) -> Option<std::sync::Arc<str>> {
    data_file::read_to_string_with_stamp(path)
        .ok()
        .filter(|(_, stamp)| stamp.virtual_file)
        .map(|(contents, _)| contents)
}

// Re-export primary types
pub(crate) use context::CmContextCheckpoint;
pub use context::{
    AnalogTransition, AnalogValue, AnalysisType, CallType, CmContext, EvaluationPhase,
};
pub(crate) use data_file::read_to_string as read_data_file_to_string;
pub(crate) use data_file::read_to_string_limited as read_data_file_to_string_limited;
pub use data_file::{
    clear_registered_data_files, register_data_file, register_data_file_with_limits,
    unregister_data_file,
};
pub use digital::{DigitalState, DigitalStrength, DigitalValue};
pub use event::EventValue;
pub(crate) use event::{
    SharedXspiceEventQueue, SharedXspiceEventValues, XspiceEventScheduler, XspiceEventValues,
};
pub use external::{
    DigitalCosimInputEvent, DigitalCosimRuntime, DigitalCosimRuntimeFactory, DigitalCosimSpec,
    DigitalCosimStep, DigitalProcessRuntime, DigitalProcessRuntimeFactory, DigitalProcessSpec,
    set_digital_cosim_runtime_factory, set_digital_process_runtime_factory,
};
pub use instance::{AnalogInputConnection, DigitalPortConnection, PortConnection, XspiceInstance};
pub(crate) use instance::{EventInputKind, SharedXspiceInstance, XspiceInstanceCheckpoint};
pub use registry::CodeModelRegistry;
pub use traits::{
    CmError, CmResult, CodeModel, ParamSpec, ParamType, PortDirection, PortSpec, PortType,
    XspiceCheckpointSupport,
};
