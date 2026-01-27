//! State Management Module
//!
//! Application state for simulation, project, and UI state.
//! Core data structures that are shared across multiple modules.
//!
//! Note: Domain-specific state has been moved to respective modules:
//! - FFT compute utilities are in `crate::analysis::fft::compute`
//! - Digital waveform state is in `crate::waveform::digital`
//! - Waveform math is in `crate::waveform::math`

pub mod dc_annotation;
pub mod dc_annotation_placement;
pub mod display_settings;
pub mod document;
pub mod hierarchy;
pub mod history;
pub mod junction_cache;
pub mod label_cache;
pub mod leader_line;
pub mod library_browser;
pub mod model_library;
pub mod preferences;
pub mod property_types;
pub mod recent_files;
pub mod render_context;
mod schematic;
pub mod schematic_file;
pub mod session_state;
pub mod shortcuts;
mod simulation;
pub mod simulation_command;
pub mod spatial_index;
pub mod viewport;

pub use dc_annotation::{AnnotationKind, AnnotationMode, DcAnnotationState};
pub use display_settings::{PinNameVisibility, SchematicDisplaySettings};
pub use document::{Document, DocumentManager};
pub use history::SchematicHistory;
pub use library_browser::{Cell, Library, LibraryManager, View, ViewType};
pub use property_types::{
    PropertyDefinition, PropertyRegistry, PropertySheet, PropertyType, PropertyValue,
};
pub use schematic::*;
pub use simulation::{
    ConsoleMessage, CrossProbeMapping, MessageSeverity, SimulationState, WaveformData,
};
pub use viewport::{BoundingBox, Viewport, VisibilityFilter};

// Re-export from domain modules for backward compatibility
#[allow(unused_imports)]
pub use crate::analysis::fft::{compute_fft, FftResult, WindowFunction};
#[allow(unused_imports)]
pub use crate::waveform::math::eval_expression;

// Re-export simulation utilities from their module
pub use crate::services::simulation_runner::{
    run_simulation, run_simulation_with_options, SimulationResult, SimulationStats,
};
pub use crate::simulation::{generate_netlist, NetlistGenerator, NetlistResult};
