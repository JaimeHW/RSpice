//! The code feature's dialogs: creating a Verilog-A device and editing the
//! open Verilog-A view's build settings.

/// What the code feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct CodeState {
    /// New Verilog-A device: a cell whose module the reader writes next.
    pub(crate) new_veriloga_device:
        crate::workbench::app::dialogs::library::new_veriloga_device::NewVerilogADeviceFormState,
    /// Build settings: the open Verilog-A view's build profile, as a form.
    pub(crate) veriloga_build_settings: Option<crate::workbench::app::VerilogABuildSettingsDraft>,
}
