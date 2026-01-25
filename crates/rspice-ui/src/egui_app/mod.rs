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
//!
//! # Usage
//!
//! ```bash
//! cargo run -p rspice-ui --features egui --no-default-features
//! ```

#[cfg(feature = "egui")]
pub mod app;

#[cfg(feature = "egui")]
pub mod menu_bar;

#[cfg(feature = "egui")]
pub mod toolbar;

#[cfg(feature = "egui")]
pub mod panels;

#[cfg(feature = "egui")]
pub mod schematic_view;

#[cfg(feature = "egui")]
pub mod status_bar;

#[cfg(feature = "egui")]
pub mod theme;

// Re-exports for convenient access
#[cfg(feature = "egui")]
pub use app::RSpiceApp;

#[cfg(feature = "egui")]
pub use theme::RSpiceTheme;
