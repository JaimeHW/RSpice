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
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use library::is_foreign_platform_absolute_path;
pub use library::{ModelLibrary, ModelSourceContent, ModelSourceEdge, ModelSourcePin};
pub(crate) use library::{first_unreachable_source, is_portable_absolute_path};
pub use manager::{ModelLibraryManager, SealedModelExecutionSources};
pub use model::DeviceModel;
pub use types::{ModelLevel, ModelType};
