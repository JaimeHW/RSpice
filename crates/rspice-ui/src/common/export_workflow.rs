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
    fn show_save_dialog(&self, config: SaveDialogConfig<'_>) -> Result<Option<PathBuf>, String>;
    fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String>;
    fn write_waveform_csv(
        &self,
        dataset: &crate::io::WaveformDataset,
        path: &Path,
    ) -> Result<(), String>;

    fn saved_paths_are_reopenable(&self) -> bool {
        true
    }
}

/// Production IO backend using native dialogs and filesystem operations.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct NativeExportWorkflowIo;

impl ExportWorkflowIo for NativeExportWorkflowIo {
    #[cfg(not(target_arch = "wasm32"))]
    fn show_save_dialog(&self, config: SaveDialogConfig<'_>) -> Result<Option<PathBuf>, String> {
        Ok(rfd::FileDialog::new()
            .add_filter(config.filter_name, config.filter_extensions)
            .set_file_name(config.default_name)
            .set_title(config.title)
            .save_file())
    }

    #[cfg(target_arch = "wasm32")]
    fn show_save_dialog(&self, config: SaveDialogConfig<'_>) -> Result<Option<PathBuf>, String> {
        let default_name = config.default_name.trim();
        if default_name.is_empty() {
            return Err(format!(
                "{} export requires a default file name",
                config.title
            ));
        }
        if config.filter_name.trim().is_empty() || config.filter_extensions.is_empty() {
            return Err(format!(
                "{} export is missing file-type metadata",
                config.title
            ));
        }
        Ok(Some(PathBuf::from(default_name)))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
        std::fs::write(path, contents).map_err(|err| err.to_string())
    }

    #[cfg(target_arch = "wasm32")]
    fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
        crate::common::browser_download::download_text_file(path, contents)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn write_waveform_csv(
        &self,
        dataset: &crate::io::WaveformDataset,
        path: &Path,
    ) -> Result<(), String> {
        crate::io::WaveformWriter::new(crate::io::WaveformFormat::Csv).write(dataset, path)
    }

    #[cfg(target_arch = "wasm32")]
    fn write_waveform_csv(
        &self,
        dataset: &crate::io::WaveformDataset,
        path: &Path,
    ) -> Result<(), String> {
        let contents =
            crate::io::WaveformWriter::new(crate::io::WaveformFormat::Csv).write_text(dataset)?;
        crate::common::browser_download::download_text_file(path, &contents)
    }

    fn saved_paths_are_reopenable(&self) -> bool {
        !cfg!(target_arch = "wasm32")
    }
}

pub(crate) fn export_completion_message(
    label: &str,
    path: &Path,
    detail: Option<String>,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> String {
    let detail = detail.filter(|detail| !detail.trim().is_empty());
    match (io.saved_paths_are_reopenable(), detail) {
        (true, Some(detail)) => format!("Exported {label}: {} ({detail})", path.display()),
        (true, None) => format!("Exported {label}: {}", path.display()),
        (false, Some(detail)) => format!(
            "{label} download started: {} ({detail}; confirm the browser accepted the download)",
            path.display()
        ),
        (false, None) => format!(
            "{label} download started: {} (confirm the browser accepted the download)",
            path.display()
        ),
    }
}
