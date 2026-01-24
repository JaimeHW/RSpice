//! Dialogs Module
//!
//! Domain-specific dialog components for the RSpice UI application.
//! These are separated from primitive UI components to improve organization.
//!
//! ## Modules
//!
//! - `simulation_dialog` - Tabbed simulation configuration dialog
//! - `component_edit` - Component property editor dialog
//! - `veriloga_dialog` - Verilog-A module editor dialog
//! - `veriloga_inspector` - Verilog-A code inspector
//! - `confirm_modal` - Generic confirmation modal
//! - `model_types` - Device model types and parameter definitions
//! - `model_editor` - Advanced device model editor dialog
//! - `simulation_options` - Simulation options configuration

pub mod component_edit;
pub mod confirm_modal;
pub mod model_editor;
pub mod model_types;
pub mod simulation_dialog;
pub mod simulation_options;
pub mod veriloga_dialog;
pub mod veriloga_inspector;

// Re-export main dialog components
pub use component_edit::ComponentEditModal;
pub use confirm_modal::{SaveDialogResult, UnsavedChangesModal};
pub use model_editor::ModelEditorState;
pub use model_types::{
    BjtModelType, DeviceModelType, DiodeModelType, ModelParameter, MosfetModelType,
    ParameterCategory, TechnologyNode,
};
pub use simulation_dialog::SimulationDialog;
pub use simulation_options::{
    AcOptions, AdvancedOptions, ConvergenceOptions, DcConvergenceAid, DcOptions, FrequencyScale,
    IntegrationMethod, MatrixSolver, OptionCategory, PivotStrategy, SimulationOptions,
    SimulationOptionsDialog, SimulationOptionsDialogProps, TransientOptions,
};
pub use veriloga_dialog::VerilogAImportDialog;
pub use veriloga_inspector::{ParameterInfo, VerilogAInspector, VerilogAModelInfo};
