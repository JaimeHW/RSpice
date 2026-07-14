//! XSPICE Code Model Subsystem
//!
//! Provides mixed-signal simulation capabilities through code models.
//! This implementation follows the ngspice XSPICE specification.
//!
// Allow dead code - XSPICE has infrastructure for digital/event simulation
// that will be connected as the subsystem matures
#![allow(dead_code)]

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
mod external;
pub mod ifspec;
mod instance;
mod metadata;
mod registry;
mod traits;

// Built-in code models
pub mod models;

// Re-export primary types
pub(crate) use context::CmContextCheckpoint;
pub use context::{AnalogValue, AnalysisType, CallType, CmContext, EvaluationPhase};
pub(crate) use data_file::read_to_string as read_data_file_to_string;
pub use data_file::{clear_registered_data_files, register_data_file, unregister_data_file};
pub use digital::{DigitalState, DigitalStrength, DigitalValue};
pub use event::{Event, EventQueue, EventValue};
pub use external::{
    DigitalCosimInputEvent, DigitalCosimRuntime, DigitalCosimRuntimeFactory, DigitalCosimSpec,
    DigitalCosimStep, DigitalProcessRuntime, DigitalProcessRuntimeFactory, DigitalProcessSpec,
    set_digital_cosim_runtime_factory, set_digital_process_runtime_factory,
};
pub(crate) use instance::XspiceInstanceCheckpoint;
pub use instance::{AnalogInputConnection, DigitalPortConnection, PortConnection, XspiceInstance};
pub use registry::CodeModelRegistry;
pub use traits::{
    CmError, CmResult, CodeModel, ParamSpec, ParamType, PortDirection, PortSpec, PortType,
    XspiceCheckpointSupport,
};
