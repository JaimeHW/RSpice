use std::path::{Path, PathBuf};

/// Save dialog configuration for export workflows.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SaveDialogConfig<'a> {
    pub title: &'a str,
    pub default_name: &'a str,
    pub filter_name: &'a str,
    pub filter_extensions: &'a [&'a str],
}

/// IO abstraction for export workflows.
///
/// This allows export behavior to be fully unit tested without invoking
/// native dialogs or filesystem writes.
pub(crate) trait ExportWorkflowIo {
    fn show_save_dialog(&self, config: SaveDialogConfig<'_>) -> Option<PathBuf>;
    fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String>;
    fn write_waveform_csv(
        &self,
        dataset: &crate::io::WaveformDataset,
        path: &Path,
    ) -> Result<(), String>;
}

/// Production IO backend using native dialogs and filesystem operations.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct NativeExportWorkflowIo;

impl ExportWorkflowIo for NativeExportWorkflowIo {
    #[cfg(not(target_arch = "wasm32"))]
    fn show_save_dialog(&self, config: SaveDialogConfig<'_>) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter(config.filter_name, config.filter_extensions)
            .set_file_name(config.default_name)
            .set_title(config.title)
            .save_file()
    }

    /// Browser builds will route exports through a download blob instead of a
    /// native save dialog; until that lands the action declines gracefully.
    #[cfg(target_arch = "wasm32")]
    fn show_save_dialog(&self, _config: SaveDialogConfig<'_>) -> Option<PathBuf> {
        None
    }

    fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
        std::fs::write(path, contents).map_err(|err| err.to_string())
    }

    fn write_waveform_csv(
        &self,
        dataset: &crate::io::WaveformDataset,
        path: &Path,
    ) -> Result<(), String> {
        crate::io::WaveformWriter::new(crate::io::WaveformFormat::Csv).write(dataset, path)
    }
}
