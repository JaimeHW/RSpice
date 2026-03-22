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
#[allow(dead_code)]
pub(crate) mod document;
pub mod hierarchy;
pub mod history;
pub mod junction_cache;
pub mod label_cache;
pub mod leader_line;
pub mod library_browser;
pub mod model_library;
pub mod pdk_config;
pub mod preferences;
pub mod property_types;
pub mod recent_files;
pub mod render_context;
mod schematic;
pub mod schematic_file;
#[allow(dead_code)]
pub(crate) mod session_state;
pub mod shortcuts;
mod simulation;
pub mod spatial_index;
pub mod viewport;

pub use dc_annotation::{AnnotationKind, AnnotationMode, DcAnnotationState};
pub use display_settings::{PinNameVisibility, SchematicDisplaySettings};
pub use history::SchematicHistory;
pub use library_browser::{Cell, Library, LibraryManager, View, ViewType};
pub use model_library::ModelLibraryManager;
pub use pdk_config::{ConfigError, DiscoveredFile, LibraryPathEntry, PdkConfig};
pub use property_types::{
    format_engineering, DisplayMode, PropertyDefinition, PropertyRegistry, PropertySheet,
    PropertyType, PropertyValue, VisibilityCondition,
};
pub use schematic::*;
pub use simulation::{
    AnalysisResult, AnalysisType, CrossProbeMapping, DcOpResult, OperatingPointValue,
    SimulationRun, SimulationState, WaveformData,
};
pub use viewport::{BoundingBox, Viewport, VisibilityFilter};
