use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{CellView, SymbolContent, ViewContent};

/// A cell is a reusable design unit containing multiple views.
///
/// Cells represent logical design blocks like op-amps, filters, or full ICs.
/// Each cell can have multiple views (schematic, symbol, layout, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    /// Cell name
    pub name: String,
    /// Parent library name
    pub library: String,
    /// Views in this cell
    pub views: HashMap<String, CellView>,
    /// Cell category/type
    pub category: CellCategory,
    /// Cell properties
    pub properties: HashMap<String, String>,
    /// Interface definition (pins)
    pub interface: CellInterface,
}

/// Cell category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CellCategory {
    /// Analog circuit block
    #[default]
    Analog,
    /// Digital logic block
    Digital,
    /// Mixed-signal block
    MixedSignal,
    /// Power/IO cell
    PowerIO,
    /// Test structure
    Test,
    /// Top-level design
    Top,
}

/// Cell interface definition (ports/pins)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CellInterface {
    /// Interface pins
    pub pins: Vec<InterfacePin>,
}

/// Interface pin definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfacePin {
    /// Pin name
    pub name: String,
    /// Pin direction
    pub direction: PinDirection,
    /// Pin type (signal, power, ground)
    pub pin_type: PinType,
    /// Bus width (1 for single, >1 for bus)
    pub width: usize,
}

/// Pin direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PinDirection {
    #[default]
    Input,
    Output,
    InOut,
}

/// Pin electrical type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PinType {
    #[default]
    Signal,
    Power,
    Ground,
    Clock,
}

impl Cell {
    /// Create a new cell
    pub fn new(name: &str, library: &str) -> Self {
        Self {
            name: name.to_string(),
            library: library.to_string(),
            views: HashMap::new(),
            category: CellCategory::Analog,
            properties: HashMap::new(),
            interface: CellInterface::default(),
        }
    }

    /// Set category
    pub fn with_category(mut self, category: CellCategory) -> Self {
        self.category = category;
        self
    }

    /// Add a view to this cell
    pub fn add_view(&mut self, view: CellView) {
        self.views.insert(view.name.clone(), view);
    }

    /// Add view (builder pattern)
    pub fn with_view(mut self, view: CellView) -> Self {
        self.add_view(view);
        self
    }

    /// Get view by name
    pub fn get_view(&self, name: &str) -> Option<&CellView> {
        self.views.get(name)
    }

    /// Get mutable view
    pub fn get_view_mut(&mut self, name: &str) -> Option<&mut CellView> {
        self.views.get_mut(name)
    }

    /// Get schematic view (convenience)
    pub fn schematic(&self) -> Option<&CellView> {
        self.views.get("schematic")
    }

    /// Get symbol view (convenience)
    pub fn symbol(&self) -> Option<&CellView> {
        self.views.get("symbol")
    }

    /// List view names
    pub fn view_names(&self) -> Vec<&str> {
        self.views.keys().map(|s| s.as_str()).collect()
    }

    /// Add interface pin
    pub fn add_pin(&mut self, pin: InterfacePin) {
        self.interface.pins.push(pin);
    }

    /// Set a property
    pub fn set_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }

    /// Get full cell reference (library:cell)
    pub fn full_name(&self) -> String {
        format!("{}:{}", self.library, self.name)
    }

    /// Ensure this cell has a concrete symbol view derived from its interface pins.
    ///
    /// Existing non-empty symbol views are preserved to avoid overriding user-authored graphics.
    pub fn ensure_symbol_view(&mut self) {
        let should_generate = match self.views.get("symbol") {
            None => true,
            Some(view) => match &view.content {
                ViewContent::Placeholder => true,
                ViewContent::Symbol(symbol) => {
                    symbol.pins.is_empty() && !self.interface.pins.is_empty()
                }
                _ => true,
            },
        };

        if should_generate {
            let symbol = SymbolContent::generated(&self.name, &self.interface.pins);
            self.add_view(CellView::symbol(symbol));
        }
    }
}

impl InterfacePin {
    /// Create a new interface pin
    pub fn new(name: &str, direction: PinDirection) -> Self {
        Self {
            name: name.to_string(),
            direction,
            pin_type: PinType::Signal,
            width: 1,
        }
    }

    /// Create input pin
    pub fn input(name: &str) -> Self {
        Self::new(name, PinDirection::Input)
    }

    /// Create output pin
    pub fn output(name: &str) -> Self {
        Self::new(name, PinDirection::Output)
    }

    /// Create inout pin
    pub fn inout(name: &str) -> Self {
        Self::new(name, PinDirection::InOut)
    }

    /// Set as power pin
    pub fn power(mut self) -> Self {
        self.pin_type = PinType::Power;
        self
    }

    /// Set as ground pin
    pub fn ground(mut self) -> Self {
        self.pin_type = PinType::Ground;
        self
    }

    /// Set bus width
    pub fn bus(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
}
