use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::state::Point;

/// An instance of a cell placed in a parent schematic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyInstance {
    /// Instance name (must be unique in parent)
    pub instance_name: String,
    /// Cell reference (library:cell)
    pub cell_ref: CellReference,
    /// Position in parent schematic (grid units)
    pub position: Point,
    /// Rotation in degrees (0, 90, 180, 270)
    pub rotation: i32,
    /// Horizontal mirror
    pub mirror_x: bool,
    /// Instance properties (override cell defaults)
    pub properties: HashMap<String, String>,
    /// Net connections (pin_name -> net_name)
    pub connections: HashMap<String, String>,
}

/// Reference to a cell (for instances)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellReference {
    /// Library name
    pub library: String,
    /// Cell name
    pub cell: String,
    /// View name (usually "symbol" for graphical, "schematic" for simulation)
    pub view: String,
}

impl CellReference {
    /// Create a cell reference
    pub fn new(library: &str, cell: &str, view: &str) -> Self {
        Self {
            library: library.to_string(),
            cell: cell.to_string(),
            view: view.to_string(),
        }
    }

    /// Create reference to schematic view
    pub fn schematic(library: &str, cell: &str) -> Self {
        Self::new(library, cell, "schematic")
    }

    /// Create reference to symbol view
    pub fn symbol(library: &str, cell: &str) -> Self {
        Self::new(library, cell, "symbol")
    }

    /// Get full path string
    pub fn full_path(&self) -> String {
        format!("{}:{}:{}", self.library, self.cell, self.view)
    }

    //-------------------------------------------------------------------------
    // Core Integration Methods
    //-------------------------------------------------------------------------

    /// Convert to a core HierarchyPath segment
    ///
    /// Creates a path segment representing this cell reference.
    /// The format is compatible with simulation hierarchy tracking.
    pub fn to_hierarchy_segment(&self) -> String {
        self.cell.clone()
    }

    /// Create from a cell name (for quick lookups)
    pub fn from_cell_name(cell: &str) -> Self {
        Self::new("", cell, "schematic")
    }
}

impl HierarchyInstance {
    /// Create a new hierarchy instance
    pub fn new(name: &str, cell_ref: CellReference, x: i32, y: i32) -> Self {
        Self {
            instance_name: name.to_string(),
            cell_ref,
            position: Point::new(x, y),
            rotation: 0,
            mirror_x: false,
            properties: HashMap::new(),
            connections: HashMap::new(),
        }
    }

    /// Set rotation
    pub fn with_rotation(mut self, degrees: i32) -> Self {
        self.rotation = degrees % 360;
        self
    }

    /// Set mirror
    pub fn with_mirror(mut self, mirror: bool) -> Self {
        self.mirror_x = mirror;
        self
    }

    /// Connect pin to net
    pub fn connect(&mut self, pin_name: &str, net_name: &str) {
        self.connections
            .insert(pin_name.to_string(), net_name.to_string());
    }

    /// Get net connected to pin
    pub fn get_connection(&self, pin_name: &str) -> Option<&str> {
        self.connections.get(pin_name).map(|s| s.as_str())
    }

    /// Set instance property
    pub fn set_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }

    //-------------------------------------------------------------------------
    // Core Integration Methods
    //-------------------------------------------------------------------------

    /// Get the full hierarchical path to this instance from a parent path
    ///
    /// Appends this instance name to the parent path to create the full
    /// simulation hierarchy path.
    ///
    /// # Arguments
    /// * `parent_path` - The hierarchical path to the parent cell
    ///
    /// # Returns
    /// Full path string in format "parent.instance_name"
    pub fn full_hierarchy_path(&self, parent_path: &str) -> String {
        if parent_path.is_empty() {
            self.instance_name.clone()
        } else {
            format!("{}.{}", parent_path, self.instance_name)
        }
    }

    /// Get parameter value with string key (for simulation)
    pub fn get_param(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }

    /// Get all properties as (name, value) pairs for ParamResolver
    pub fn param_pairs(&self) -> Vec<(String, f64)> {
        self.properties
            .iter()
            .filter_map(|(k, v)| v.parse::<f64>().ok().map(|val| (k.clone(), val)))
            .collect()
    }
}
