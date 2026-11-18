//! View types.
//!
//! What a view of a cell can be — schematic, symbol, netlist, Verilog-A —
//! and which of them a given cell is allowed to have.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// View Types
// =============================================================================

/// Standard view types matching Cadence conventions
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
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
    /// Verilog-AMS mixed-signal model: analog and discrete behaviour in one
    /// module.
    VerilogAms,
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
            ViewType::VerilogAms => "verilogams",
            ViewType::Spice => "spice",
            ViewType::Document => "doc",
            ViewType::Extracted => "extracted",
            ViewType::Abstract => "abstract",
            ViewType::Config => "config",
            ViewType::Custom => "custom",
        }
    }

    /// How the view type reads in product text. [`Self::display_name`] is also
    /// the default view name and the canonical word in a `lib/cell/view` path,
    /// so it stays as stored; this is only ever a caption.
    pub const fn caption(&self) -> &'static str {
        match self {
            ViewType::VerilogA => "Verilog-A",
            ViewType::Schematic => "schematic",
            ViewType::Symbol => "symbol",
            ViewType::Layout => "layout",
            ViewType::Testbench => "testbench",
            ViewType::Verilog => "Verilog",
            ViewType::VerilogAms => "Verilog-AMS",
            ViewType::Spice => "SPICE",
            ViewType::Document => "document",
            ViewType::Extracted => "extracted",
            ViewType::Abstract => "abstract",
            ViewType::Config => "configuration",
            ViewType::Custom => "custom",
        }
    }

    /// A tab or title caption for the view named `name`: a view keeps the
    /// name it was given, and a source view under its type's default name
    /// reads as its language.
    pub fn caption_for<'a>(&self, name: &'a str) -> &'a str {
        if self.is_hdl_source() && name.eq_ignore_ascii_case(self.display_name()) {
            self.caption()
        } else {
            name
        }
    }

    /// Whether this is a Verilog-family source view: Verilog, Verilog-A or
    /// Verilog-AMS. One compiler reads all three (Verilog-AMS LRM 2.4 over
    /// IEEE 1364-2005), so a view of any of them that owns a project source
    /// compiles, draws its symbol from its module's ports and netlists the
    /// same way.
    pub const fn is_hdl_source(&self) -> bool {
        matches!(
            self,
            ViewType::Verilog | ViewType::VerilogA | ViewType::VerilogAms
        )
    }

    /// The extension of the root file a new source view of this type is
    /// created with, for the Verilog family.
    pub const fn hdl_root_extension(&self) -> Option<&'static str> {
        match self {
            ViewType::Verilog => Some(".v"),
            ViewType::VerilogA => Some(".va"),
            ViewType::VerilogAms => Some(".vams"),
            _ => None,
        }
    }

    /// The Verilog-family view type a root file of this name is the source
    /// of, by its extension, case-insensitively: `.v` Verilog, `.va`
    /// Verilog-A and `.vams` Verilog-AMS. The inverse of
    /// [`Self::hdl_root_extension`]; any other file is the root of none.
    pub fn of_hdl_root(file_name: &str) -> Option<Self> {
        let (_, extension) = file_name.rsplit_once('.')?;
        [ViewType::Verilog, ViewType::VerilogA, ViewType::VerilogAms]
            .into_iter()
            .find(|view_type| {
                view_type
                    .hdl_root_extension()
                    .is_some_and(|root| root[1..].eq_ignore_ascii_case(extension))
            })
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
            "verilogams" | "vams" => ViewType::VerilogAms,
            "spice" | "sp" => ViewType::Spice,
            "doc" | "document" => ViewType::Document,
            "extracted" | "ext" => ViewType::Extracted,
            "abstract" | "abs" => ViewType::Abstract,
            "config" | "cfg" => ViewType::Config,
            _ => ViewType::Custom,
        }
    }

    /// All standard view types
    pub const ALL: [ViewType; 13] = [
        ViewType::Schematic,
        ViewType::Symbol,
        ViewType::Layout,
        ViewType::Testbench,
        ViewType::Verilog,
        ViewType::VerilogA,
        ViewType::VerilogAms,
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
    /// Whether view has unsaved changes
    pub modified: bool,
    /// When a save last wrote a change to the view, in milliseconds since
    /// the epoch. Like the unsaved-changes mark it is no part of the view's
    /// content: recording it makes nothing dirty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved_at: Option<u64>,
    /// View-specific metadata
    pub metadata: HashMap<String, String>,
}

impl Default for View {
    fn default() -> Self {
        Self {
            name: String::new(),
            view_type: ViewType::Schematic,
            file_path: None,
            modified: false,
            saved_at: None,
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

    /// Set file path
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.file_path = Some(path);
        self
    }
}

// =============================================================================

#[cfg(test)]
mod caption_tests {
    use super::ViewType;

    #[test]
    fn a_verilog_a_view_reads_as_its_type_until_it_is_given_a_name_of_its_own() {
        assert_eq!(ViewType::VerilogA.caption_for("veriloga"), "Verilog-A");
        assert_eq!(ViewType::VerilogA.caption_for("behavioral"), "behavioral");
        assert_eq!(ViewType::Schematic.caption_for("schematic"), "schematic");
        assert_eq!(ViewType::VerilogA.caption(), "Verilog-A");
        assert_eq!(ViewType::Verilog.caption_for("verilog"), "Verilog");
        assert_eq!(
            ViewType::VerilogAms.caption_for("verilogams"),
            "Verilog-AMS"
        );
        assert_eq!(ViewType::VerilogAms.caption_for("mixed"), "mixed");
    }

    #[test]
    fn a_verilog_ams_view_keeps_its_name_and_reads_back_from_it() {
        assert_eq!(ViewType::VerilogAms.display_name(), "verilogams");
        assert_eq!(ViewType::from_name("verilogams"), ViewType::VerilogAms);
        assert_eq!(ViewType::from_name("vams"), ViewType::VerilogAms);
        assert!(ViewType::ALL.contains(&ViewType::VerilogAms));
        let family = ViewType::ALL
            .into_iter()
            .filter(ViewType::is_hdl_source)
            .map(|view| view.hdl_root_extension())
            .collect::<Vec<_>>();
        assert_eq!(family, [Some(".v"), Some(".va"), Some(".vams")]);
    }

    /// A root file's extension names the view it is the source of, the
    /// inverse of the extension a new view's root is created with.
    #[test]
    fn a_root_file_states_its_view_type_by_its_extension() {
        assert_eq!(ViewType::of_hdl_root("sar8.v"), Some(ViewType::Verilog));
        assert_eq!(ViewType::of_hdl_root("cmp.VA"), Some(ViewType::VerilogA));
        assert_eq!(ViewType::of_hdl_root("tb.vams"), Some(ViewType::VerilogAms));
        assert_eq!(ViewType::of_hdl_root("body.inc"), None);
        assert_eq!(ViewType::of_hdl_root("vams"), None);
    }
}
