//! The code feature's dialogs, creating a Verilog-A device and editing the
//! open Verilog-A view's build settings, and its netlist and Code & Automation
//! editors' session, with the netlist documents opened and the outline groups
//! collapsed.

/// What the code feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct CodeState {
    /// New Verilog-A device: a cell whose module the reader writes next.
    pub(crate) new_veriloga_device:
        crate::workbench::app::dialogs::library::new_veriloga_device::NewVerilogADeviceFormState,
    /// Build settings: the open Verilog-A view's build profile, as a form.
    pub(crate) veriloga_build_settings: Option<crate::workbench::app::VerilogABuildSettingsDraft>,
    /// Netlist editor state (diagnostics, diff pips, tuner mode).
    pub netlist: crate::workbench::documents::netlist_document::NetlistDocumentState,
    /// Visible Code & Automation page and revision-bound operation receipts.
    pub code_workspace: crate::workbench::documents::code_workspace::CodeWorkspaceRuntimeState,
    /// Netlist secondary documents explicitly opened by the user. Resolved
    /// include closures can contain thousands of files, so availability alone
    /// must never materialize every include as a tab.
    pub netlist_open_documents:
        std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
    /// Netlist outline groups the engineer has collapsed. Collapsed rather
    /// than expanded, so a deck that gains a category discloses it instead of
    /// hiding it behind a preference nobody set.
    pub netlist_outline_collapsed: std::collections::BTreeSet<crate::state::OutlineSectionKind>,
}
