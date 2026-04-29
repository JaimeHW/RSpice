use serde::{Deserialize, Serialize};

use crate::state::schematic::SchematicState;

use super::SymbolContent;

/// A view represents one aspect of a cell (schematic, symbol, layout, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellView {
    /// View name (e.g., "schematic", "symbol")
    pub name: String,
    /// View type
    pub view_type: ViewType,
    /// View content
    pub content: ViewContent,
    /// Last modified timestamp
    pub modified: String,
}

/// View type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ViewType {
    /// Schematic (circuit design)
    #[default]
    Schematic,
    /// Symbol (graphical representation)
    Symbol,
    /// Netlist (text-based circuit)
    Netlist,
    /// Layout (physical design metadata; editing is not implemented yet)
    Layout,
    /// Documentation
    Documentation,
}

/// View content - the actual data for each view type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewContent {
    /// Schematic content
    Schematic(Box<SchematicState>),
    /// Symbol graphics content
    Symbol(SymbolContent),
    /// Netlist text content
    Netlist(String),
    /// Placeholder payload for reserved views without concrete editable content.
    Placeholder,
}

impl CellView {
    /// Create a schematic view
    pub fn schematic(schematic: SchematicState) -> Self {
        Self {
            name: "schematic".to_string(),
            view_type: ViewType::Schematic,
            content: ViewContent::Schematic(Box::new(schematic)),
            modified: String::new(),
        }
    }

    /// Create a symbol view
    pub fn symbol(symbol: SymbolContent) -> Self {
        Self {
            name: "symbol".to_string(),
            view_type: ViewType::Symbol,
            content: ViewContent::Symbol(symbol),
            modified: String::new(),
        }
    }

    /// Create a netlist view
    pub fn netlist(content: &str) -> Self {
        Self {
            name: "netlist".to_string(),
            view_type: ViewType::Netlist,
            content: ViewContent::Netlist(content.to_string()),
            modified: String::new(),
        }
    }

    /// Create named view
    pub fn named(name: &str, view_type: ViewType) -> Self {
        Self {
            name: name.to_string(),
            view_type,
            content: ViewContent::Placeholder,
            modified: String::new(),
        }
    }

    /// Create a placeholder symbol view (for primitive components)
    ///
    /// Kept for backward compatibility. New code should use `Cell::ensure_symbol_view`.
    pub fn symbol_placeholder() -> Self {
        Self::symbol(SymbolContent::generated("symbol", &[]))
    }
}
