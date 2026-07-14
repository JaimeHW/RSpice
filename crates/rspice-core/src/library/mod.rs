//! Component Library Management
//!
//! Provides embedded SPICE component libraries with model and subcircuit definitions.
//! Libraries are compiled into the executable for seamless distribution.
//!
//! Also supports loading external .lib files from foundry PDKs with corner selection.
//!
//! # Usage
//! ```rust
//! use rspice_core::library::{LibraryManager, ModelType};
//!
//! let manager = LibraryManager::new();
//! let diode_models = manager.models_of_type(ModelType::Diode);
//! ```

pub mod lib_parser;
mod manager;
mod parser;
mod veriloga_discovery;

pub use lib_parser::{
    LibParseResult, LibParser, LibSection, ParseError, ParsedModel, ParsedSubcircuit,
    ResolvedLibDependency, ResolvedLibSource,
};
pub use manager::{LibraryManager, ModelDefinition, ModelType, SubcircuitDefinition};
pub use veriloga_discovery::{VerilogAModelEntry, discover_veriloga_models};
