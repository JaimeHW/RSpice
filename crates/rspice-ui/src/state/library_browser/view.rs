use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// View Types
// =============================================================================

/// Standard view types matching Cadence conventions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ViewType {
    /// Schematic view (circuit diagram)
    #[default]
    Schematic,
    /// Symbol view (block symbol for hierarchical use)
    Symbol,
    /// Layout view (physical design)
    Layout,
    /// Testbench view (simulation setup)
    Testbench,
    /// Verilog behavioral model
    Verilog,
    /// VerilogA behavioral model for SPICE
    VerilogA,
    /// SPICE netlist
    Spice,
    /// Documentation
    Document,
    /// Extracted view (post-layout)
    Extracted,
    /// Abstract view (timing/power)
    Abstract,
    /// Configuration view
    Config,
    /// Custom/user-defined view
    Custom,
}

impl ViewType {
    /// Display name for view type
    pub fn display_name(&self) -> &'static str {
        match self {
            ViewType::Schematic => "schematic",
            ViewType::Symbol => "symbol",
            ViewType::Layout => "layout",
            ViewType::Testbench => "testbench",
            ViewType::Verilog => "verilog",
            ViewType::VerilogA => "veriloga",
            ViewType::Spice => "spice",
            ViewType::Document => "doc",
            ViewType::Extracted => "extracted",
            ViewType::Abstract => "abstract",
            ViewType::Config => "config",
            ViewType::Custom => "custom",
        }
    }

    /// Parse view type from string
    pub fn from_name(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "schematic" | "sch" => ViewType::Schematic,
            "symbol" | "sym" => ViewType::Symbol,
            "layout" | "lay" => ViewType::Layout,
            "testbench" | "tb" => ViewType::Testbench,
            "verilog" | "v" => ViewType::Verilog,
            "veriloga" | "va" => ViewType::VerilogA,
            "spice" | "sp" => ViewType::Spice,
            "doc" | "document" => ViewType::Document,
            "extracted" | "ext" => ViewType::Extracted,
            "abstract" | "abs" => ViewType::Abstract,
            "config" | "cfg" => ViewType::Config,
            _ => ViewType::Custom,
        }
    }

    /// Icon for this view type
    pub fn icon(&self) -> &'static str {
        match self {
            ViewType::Schematic => "📋",
            ViewType::Symbol => "🔲",
            ViewType::Layout => "🗺️",
            ViewType::Testbench => "🧪",
            ViewType::Verilog => "📝",
            ViewType::VerilogA => "📝",
            ViewType::Spice => "⚡",
            ViewType::Document => "📄",
            ViewType::Extracted => "🔍",
            ViewType::Abstract => "📊",
            ViewType::Config => "⚙️",
            ViewType::Custom => "📁",
        }
    }

    /// All standard view types
    pub const ALL: [ViewType; 12] = [
        ViewType::Schematic,
        ViewType::Symbol,
        ViewType::Layout,
        ViewType::Testbench,
        ViewType::Verilog,
        ViewType::VerilogA,
        ViewType::Spice,
        ViewType::Document,
        ViewType::Extracted,
        ViewType::Abstract,
        ViewType::Config,
        ViewType::Custom,
    ];
}

// =============================================================================
// View
// =============================================================================

/// A single view within a cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    /// View name (e.g., "schematic", "symbol")
    pub name: String,
    /// View type
    pub view_type: ViewType,
    /// Path to view file on disk
    pub file_path: Option<PathBuf>,
    /// Last modified timestamp (Unix epoch)
    pub modified_time: Option<u64>,
    /// Whether view has unsaved changes
    pub modified: bool,
    /// Whether view is currently open
    pub is_open: bool,
    /// View-specific metadata
    pub metadata: HashMap<String, String>,
}

impl Default for View {
    fn default() -> Self {
        Self {
            name: String::new(),
            view_type: ViewType::Schematic,
            file_path: None,
            modified_time: None,
            modified: false,
            is_open: false,
            metadata: HashMap::new(),
        }
    }
}

impl View {
    /// Create a new view
    pub fn new(name: impl Into<String>, view_type: ViewType) -> Self {
        Self {
            name: name.into(),
            view_type,
            ..Default::default()
        }
    }

    /// Create from file path
    pub fn from_path(path: PathBuf) -> Self {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        let view_type = ViewType::from_name(&name);
        Self {
            name,
            view_type,
            file_path: Some(path),
            ..Default::default()
        }
    }

    /// Set file path
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.file_path = Some(path);
        self
    }
}

// =============================================================================
