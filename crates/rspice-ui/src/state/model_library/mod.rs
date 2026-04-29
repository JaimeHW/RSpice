//! Model Library Browser
//!
//! PDK model file navigation with corner/process selection.
//!
//! # Architecture
//!
//! Matches Cadence's model library management:
//! - **Model Library**: Collection of device models (e.g., `tsmc180.lib`)
//! - **Section/Corner**: Process corner within library (tt, ff, ss, etc.)
//! - **Model**: Individual device model (nmos, pmos, npn, etc.)

mod corner;
mod library;
mod manager;
mod model;
mod types;

pub use corner::ProcessCorner;
pub use library::ModelLibrary;
pub use manager::ModelLibraryManager;
pub use model::DeviceModel;
pub use types::{ModelLevel, ModelType};
