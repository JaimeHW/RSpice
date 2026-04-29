//! Hierarchical Design Support
//!
//! Implements a Library/Cell/View hierarchy for managing complex
//! designs with reusable blocks. This follows the standard Open Access model
//! used by industry-standard IC design tools.

mod cell;
mod instance;
mod library;
mod manager;
mod symbol;
mod view;

pub use self::cell::{Cell, CellCategory, CellInterface, InterfacePin, PinDirection, PinType};
pub use self::instance::{CellReference, HierarchyInstance};
pub use self::library::{Library, LibraryMetadata, LibraryType};
pub use self::manager::HierarchyManager;
pub use self::symbol::{
    DrawingPrimitive, PinOrientation, SymbolContent, SymbolGraphics, SymbolPin, TextAnchor,
};
pub use self::view::{CellView, ViewContent, ViewType};
