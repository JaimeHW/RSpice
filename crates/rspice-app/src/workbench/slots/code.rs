//! The code feature's dialogs, creating a Verilog-A device and editing the
//! open Verilog-A view's build settings, and its netlist and Code & Automation
//! editors' session.

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
}
