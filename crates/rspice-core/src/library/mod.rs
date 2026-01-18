//! Component Library Management
//!
//! Provides embedded SPICE component libraries with model and subcircuit definitions.
//! Libraries are compiled into the executable for seamless distribution.
//!
//! # Usage
//! ```rust
//! use rspice_core::library::LibraryManager;
//!
//! let manager = LibraryManager::new();
//! let diode_models = manager.models_by_type(ModelType::Diode);
//! ```

mod manager;
mod parser;

pub use manager::{LibraryManager, ModelDefinition, ModelType, SubcircuitDefinition};
