//! Persisted hardcopy contracts, authenticated sources, and pagination.
//!
//! Rendering and platform print integration remain with application adapters.

pub mod contract;
pub mod mappings;
pub mod sources;

pub use contract::*;
pub use mappings::{
    PrintMappingCatalogOwner, PrintMappingPersistenceError, PrintMappingPresetCatalog,
    PrintMappingSaveDisposition, PrintMappingSaveReceipt,
};
