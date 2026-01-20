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

mod context;
mod digital;
mod event;
mod instance;
mod registry;
mod traits;

// Built-in code models
pub mod models;

// Re-export primary types
pub use context::{AnalysisType, CallType, CmContext};
pub use digital::{DigitalState, DigitalStrength, DigitalValue};
pub use event::{Event, EventQueue};
pub use instance::{PortConnection, XspiceInstance};
pub use registry::CodeModelRegistry;
pub use traits::{
    CmError, CmResult, CodeModel, ParamSpec, ParamType, PortDirection, PortSpec, PortType,
};
