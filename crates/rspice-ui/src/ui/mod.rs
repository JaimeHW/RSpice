//! RSpice UI Module
//!
//! This module contains user interface state components for the RSpice
//! schematic editor and waveform viewer.
//!
//! # Architecture
//!
//! The UI is organized into logical groups:
//!
//! - `dialog_state/` - State management for all dialogs
//! - `shortcuts` - Keyboard shortcut handling
//!
//! The main application UI components are in `crate::app`:
//! - `app` - Main application state and entry point
//! - `schematic_view` - Schematic canvas rendering
//! - `menu_bar` - Application menu bar
//! - `toolbar` - Schematic editing toolbar
//! - `panels/` - Side panels (browser, properties, console)
//! - `simulation_dialog/` - Simulation configuration dialogs
//! - `waveform/` - Waveform viewer components
//! - Various analysis viewers (bode, smith_chart, fft, etc.)

// Core state management
pub mod dialog_state;
pub mod shortcuts;

// Re-export key types for convenience
pub use dialog_state::{AnalysisDialogState, DialogState, DialogVisibility, InteractionState};
pub use shortcuts::{process_shortcuts, ShortcutCategory, ShortcutCommand, ShortcutContext};
