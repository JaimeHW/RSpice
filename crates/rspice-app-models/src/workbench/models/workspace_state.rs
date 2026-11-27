//! The Models & PDKs workspace's session state: the catalog's facets, sort
//! and selections, what the last model operation concluded, and the
//! unPDK draft kept for recovery.
//!
//! None of it is engineering data. Models, sources, symbols and PDK records
//! stay in the project's model library and the PDK configuration; this is
//! what the workspace shows them through.

use serde::{Deserialize, Serialize};

use crate::workbench::state::WorkbenchSession;
use rspice_app_state::workbench::app_state::model::Model;

/// Select a model library, stating on the status bar the model library
/// manager's refusal of a name the project no longer holds. The refusal is
/// about the gesture, so it is prompted, never logged.
pub fn select_library_or_prompt(model: &mut Model, workbench: &mut WorkbenchSession, name: &str) {
    if let Err(refusal) = model.model_library_manager.select_library(name) {
        workbench.prompt(refusal);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectModelFacet {
    #[default]
    All,
    Bound,
    Pinned,
    Review,
    /// Models compiled into the engine.
    BuiltIn,
}

impl ProjectModelFacet {
    pub const ALL: [Self; 5] = [
        Self::All,
        Self::Bound,
        Self::Pinned,
        Self::Review,
        Self::BuiltIn,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Bound => "Bound",
            Self::Pinned => "Pinned",
            Self::Review => "Review",
            Self::BuiltIn => "Built-in",
        }
    }
}

/// Which column of the project model catalog decides its order.
///
/// Presentation only: it reorders rows the page has already derived and never
/// changes which rows those are. The default is the order the table has always
/// opened in — model name, case-folded, library breaking ties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelsCatalogSortKey {
    #[default]
    Model,
    Family,
    Source,
    UsedBy,
    Status,
}

/// A sortable table's ordering: which column, and which direction.
///
/// Durable, because it is a reading preference rather than an engineering
/// fact — a reader who ordered the catalog by status wants it that way again
/// when the project reopens. Both halves carry `serde(default)` so a session
/// written before this existed restores the default order instead of failing
/// the whole view state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ModelsTableSort {
    #[serde(default)]
    pub key: ModelsCatalogSortKey,
    /// `false` is ascending, which is what a column first clicked gives.
    #[serde(default)]
    pub descending: bool,
}

impl ModelsTableSort {
    /// Apply a click on `key`: the same column toggles direction, a different
    /// one takes over ascending.
    pub fn toggle(&mut self, key: ModelsCatalogSortKey) {
        if self.key == key {
            self.descending = !self.descending;
        } else {
            self.key = key;
            self.descending = false;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RSpicePartFacet {
    #[default]
    All,
    Mosfet,
    Bipolar,
    Diode,
    JfetAndHemt,
    Passive,
    IcAndMacro,
}

impl RSpicePartFacet {
    pub const ALL: [Self; 7] = [
        Self::All,
        Self::Mosfet,
        Self::Bipolar,
        Self::Diode,
        Self::JfetAndHemt,
        Self::Passive,
        Self::IcAndMacro,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All classes",
            Self::Mosfet => "MOSFET",
            Self::Bipolar => "Bipolar",
            Self::Diode => "Diode",
            Self::JfetAndHemt => "JFET & HEMT",
            Self::Passive => "Passive",
            Self::IcAndMacro => "IC & macro",
        }
    }

    #[must_use]
    pub const fn device_filters(self) -> &'static [&'static str] {
        match self {
            Self::All => &[],
            Self::Mosfet => &[
                "mosfet-n",
                "mosfet-p",
                "mosfet-vdmos",
                "fdsoin",
                "fdsoip",
                "nsoi",
                "psoi",
                "psp103_va",
                "psp103va",
                "pspnqs103va",
                "mosvar",
            ],
            Self::Bipolar => &["bjt-npn", "bjt-pnp"],
            Self::Diode => &["diode", "sidiode"],
            // Both MESFET polarities. `mesfet-p` was missing, so every
            // p-channel MESFET the catalog classified — the foundation's own
            // among them — was reachable from no class chip at all.
            Self::JfetAndHemt => &["jfet-n", "jfet-p", "gasfet", "mesfet-n", "mesfet-p"],
            Self::Passive => &[
                "resistor",
                "capacitor",
                "inductor",
                "magnetic-core",
                "lcouple",
                "transmission-line",
            ],
            Self::IcAndMacro => &["subckt"],
        }
    }
}

/// Models-manager selections and filters that do not own engineering data.
/// Stable strings are used only for presentation; every mutation re-resolves
/// the live library/source identity and fails closed if it changed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelsWorkbenchViewState {
    #[serde(default)]
    pub catalog_scope: ModelsCatalogScope,
    #[serde(default)]
    pub project_facet: ProjectModelFacet,
    #[serde(default)]
    pub part_facet: RSpicePartFacet,
    #[serde(default)]
    pub catalog_query: String,
    /// How the project catalog table is ordered.
    #[serde(default)]
    pub catalog_sort: ModelsTableSort,
    #[serde(default)]
    pub selected_part: Option<String>,
    /// Zero-based offset into the exact current part-catalog query.
    #[serde(default)]
    pub part_catalog_offset: usize,
    #[serde(default)]
    pub selected_symbol: Option<String>,
    #[serde(default)]
    pub selected_corner: Option<String>,
    #[serde(default)]
    pub selected_bin_family: Option<String>,
    #[serde(default)]
    pub include_selected_source: Option<String>,
    #[serde(default)]
    pub include_direct_only: bool,
    #[serde(default)]
    pub include_definition_query: String,
    /// Last real workflow result. It is deliberately not durable: restored
    /// sessions must derive health from retained sources, not trust a toast.
    #[serde(skip)]
    pub action_receipt: Option<Result<String, String>>,
    /// One authenticated source import may parse at a time. Parsing is owned
    /// by a native background thread or a dedicated browser worker; these
    /// fields are presentation only and never restore as engineering state.
    #[serde(skip)]
    pub model_import_in_progress: bool,
    #[serde(skip)]
    pub model_import_label: Option<String>,
    #[serde(skip)]
    pub dialog: Option<ModelsWorkbenchDialog>,
}

impl Default for ModelsWorkbenchViewState {
    fn default() -> Self {
        Self {
            catalog_scope: ModelsCatalogScope::default(),
            project_facet: ProjectModelFacet::default(),
            part_facet: RSpicePartFacet::default(),
            catalog_query: String::new(),
            catalog_sort: ModelsTableSort::default(),
            selected_part: None,
            part_catalog_offset: 0,
            selected_symbol: None,
            selected_corner: None,
            selected_bin_family: None,
            include_selected_source: None,
            include_direct_only: false,
            include_definition_query: String::new(),
            action_receipt: None,
            model_import_in_progress: false,
            model_import_label: None,
            dialog: None,
        }
    }
}

impl ModelsWorkbenchViewState {
    /// Put an action's outcome on the workspace's banner.
    pub fn note_receipt(&mut self, result: Result<String, String>) {
        self.action_receipt = Some(result);
    }
}

/// Corpus projection selected in the Models & PDKs catalog. This is
/// presentation state only: attaching a source or adding a project model is a
/// separate guarded engineering transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelsCatalogScope {
    #[default]
    Project,
    RSpiceLibrary,
}

impl ModelsCatalogScope {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::RSpiceLibrary => "RSpice library",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelsWorkbenchDialog {
    /// Models > New model: the device type, by its index in the engine's
    /// table of new-model defaults, and the name the card will carry.
    NewModel {
        device: usize,
        name: String,
    },
    #[cfg(target_arch = "wasm32")]
    SelectBrowserImportRoot {
        candidates: Vec<String>,
        selected: usize,
    },
    SourcePreview {
        title: String,
        subtitle: String,
        source: String,
        /// Whether this source can be authored, which offers a route into
        /// Model Editor. The preview itself is never a writable buffer — it
        /// holds a copy of the retained bytes with nowhere to write back to.
        editable: bool,
    },
    CompareModels {
        left_library: String,
        left_model: String,
        /// The chosen counterpart as `(library, model)`. `None` means the
        /// dialog is still asking: a comparison against an arbitrary
        /// definition is not a comparison, so there is no default beyond the
        /// same card in another library.
        right: Option<(String, String)>,
    },
    AuthorTechnologySymbolVariant {
        package_id: String,
        source_cell: String,
        target_library: String,
        target_cell: String,
    },
    DefinitionConflict {
        definition: String,
        scope: crate::state::model_library::ModelConsumerScope,
        providers: Vec<String>,
        selected_provider: String,
        reason: String,
        error: Option<String>,
    },
    BindingTrace {
        model: String,
        consumers: Vec<String>,
    },
    AddCorner {
        library: String,
        name: String,
        temperature_c: String,
        supply_factor: String,
    },
    EditCorner {
        library: String,
        original_name: String,
        duplicate: bool,
        name: String,
        description: String,
        nmos_corner: String,
        pmos_corner: String,
        temperature_c: String,
        supply_factor: String,
        minimum_temperature_c: String,
        maximum_temperature_c: String,
        required_domains: Vec<crate::state::model_library::CornerSectionDomain>,
        make_default: bool,
    },
    ConfirmDeleteCorner {
        library: String,
        corner: String,
    },
    BindCornerSection {
        library: String,
        corner: String,
        domain: crate::state::model_library::CornerSectionDomain,
        section: String,
    },
    /// The findings one library's last source-drift scan produced, and the
    /// one repair. It names the library rather than carrying the findings so
    /// the dialog and the pages behind it read the same retained report.
    ResolveDrift {
        library: String,
    },
}

#[cfg(test)]
mod tests;
