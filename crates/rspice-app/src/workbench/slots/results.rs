//! The results feature's dialogs: the dataset export format question, the
//! waveform calculator and the expression diagnostics tool.

/// What the results feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct ResultsState {
    /// The format question "Export dataset…" asks, while it is open.
    pub(crate) dataset_export: Option<crate::workbench::menu_bar::DatasetExportPicker>,

    /// Waveform calculator dialog open
    pub waveform_calculator_dialog: bool,
    /// The expression and unit diagnostics tool.
    pub expression_diagnostics_dialog: bool,
}
