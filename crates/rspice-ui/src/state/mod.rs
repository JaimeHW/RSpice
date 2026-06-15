//! State Management Module
//!
//! Application state for simulation, project, and UI state.
//! Core data structures that are shared across multiple modules.

pub mod library_browser;
pub mod model_library;
pub mod pdk_config;
pub mod property_types;
mod schematic;
mod simulation;
pub mod workspace;

pub use library_browser::{Cell, Library, LibraryManager, NavColumn, View, ViewType};
pub use model_library::ModelLibraryManager;
pub use pdk_config::{ConfigError, DiscoveredFile, LibraryPathEntry, PdkConfig};
pub use property_types::{
    DisplayMode, PropertyDefinition, PropertyRegistry, PropertySheet, PropertyType, PropertyValue,
    VisibilityCondition, format_engineering,
};
pub use schematic::*;
pub use simulation::{
    AnalysisResult, AnalysisType, CrossProbeMapping, DcOpResult, NoiseContributorRow, NoiseSummary,
    OperatingPointValue, SimulationRun, SimulationState, WaveformData,
};
pub use workspace::{CellViewRef, OpenCellView, ProjectDescriptor, ProjectWorkspace, SpecEntry};
