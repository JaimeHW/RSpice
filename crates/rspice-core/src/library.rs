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
mod spice_packs;
mod veriloga_discovery;

pub use lib_parser::{
    LibParseResult, LibParser, LibSection, ParseError, ParsedModel, ParsedSubcircuit,
    ResolvedLibDependency, ResolvedLibSource,
};
pub use manager::{LibraryManager, ModelDefinition, ModelType, SubcircuitDefinition};
// The Spectre model-library adapter is source text in, canonical SPICE source
// text out — a dialect front-end, so it lives in the parsing layer beside the
// include expander that also runs it. It keeps its published path here because
// this is where a library consumer looks for it.
pub use crate::netlist::spectre_adapter::{SpectreModelAdapterError, adapt_spectre_model_library};
pub use spice_packs::{
    CatalogDefinitionPreview, CatalogEntry, CatalogSubcircuitInterface, LicenseTier,
    MODELS_DIR_ENV, SpiceLibraryIndex, SpicePack,
};
pub use veriloga_discovery::{VerilogAModelEntry, discover_veriloga_models};
