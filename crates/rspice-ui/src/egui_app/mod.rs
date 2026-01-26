//! RSpice egui Application
//!
//! Commercial-grade GPU-native UI built with egui + eframe.
//! This provides 60fps rendering with direct wgpu integration.
//!
//! # Architecture
//!
//! The application follows a professional EDA architecture:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                     RSpiceApp                            │
//! │  ┌─────────────────────────────────────────────────────┐│
//! │  │ MenuBar                                             ││
//! │  └─────────────────────────────────────────────────────┘│
//! │  ┌─────────────────────────────────────────────────────┐│
//! │  │ Toolbar                                             ││
//! │  └─────────────────────────────────────────────────────┘│
//! │  ┌────────────┬────────────────────────┬───────────────┐│
//! │  │ Component  │    Schematic View      │  Properties   ││
//! │  │ Panel      │    (wgpu rendered)     │  Panel        ││
//! │  │            │                        │               ││
//! │  │            ├────────────────────────┤               ││
//! │  │            │    Waveform View       │               ││
//! │  │            │    (wgpu rendered)     │               ││
//! │  └────────────┴────────────────────────┴───────────────┘│
//! │  ┌─────────────────────────────────────────────────────┐│
//! │  │ Status Bar                                          ││
//! │  └─────────────────────────────────────────────────────┘│
//! └─────────────────────────────────────────────────────────┘
//! ```

pub mod app;
pub mod bode;
pub mod cross_probe;
pub mod eye_diagram;
pub mod fft;
pub mod histogram;
pub mod integrated_viewer;
pub mod menu_bar;
pub mod nyquist;
pub mod panels;
pub mod pole_zero;
pub mod properties_dialog;
pub mod schematic_export;
pub mod schematic_view;
pub mod simulation;
pub mod simulation_controller;
pub mod simulation_dialog;
pub mod smith_chart;
pub mod status_bar;
pub mod symbol_library;
pub mod theme;
pub mod toolbar;
pub mod waveform;

// Re-exports for convenient access
pub use app::RSpiceApp;
pub use integrated_viewer::{ActiveViewer, IntegratedViewerState, ViewerStates};
pub use theme::RSpiceTheme;
