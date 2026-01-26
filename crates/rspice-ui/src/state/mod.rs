//! State Management Module
//!
//! Application state for simulation, project, and UI state.

pub mod dc_annotation;
pub mod dc_annotation_placement;
pub mod digital_waveform;
pub mod display_settings;
pub mod document;
pub mod fft;
pub mod hierarchy;
pub mod history;
pub mod junction_cache;
pub mod label_cache;
pub mod leader_line;
pub mod preferences;
pub mod recent_files;
pub mod render_context;
mod schematic;
pub mod schematic_file;
pub mod shortcuts;
mod simulation;
pub mod simulation_command;
pub mod spatial_index;
pub mod viewport;
pub mod waveform_math;

pub use dc_annotation::{AnnotationKind, AnnotationMode, DcAnnotationState};
pub use display_settings::{PinNameVisibility, SchematicDisplaySettings};
pub use document::{Document, DocumentManager};
#[allow(unused_imports)]
pub use fft::{compute_fft, FftResult, WindowFunction};
pub use history::SchematicHistory;
pub use schematic::*;
pub use simulation::{ConsoleMessage, MessageSeverity, SimulationState, WaveformData};
pub use viewport::{BoundingBox, Viewport, VisibilityFilter};
#[allow(unused_imports)]
pub use waveform_math::eval_expression;

// Re-export services from their new locations
pub use crate::services::netlist_generator::{generate_netlist, NetlistResult};
pub use crate::services::simulation_runner::{
    run_simulation, run_simulation_with_options, SimulationResult, SimulationStats,
};
