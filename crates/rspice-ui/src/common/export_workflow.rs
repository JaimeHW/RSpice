use std::path::{Path, PathBuf};

/// Save dialog configuration for export workflows.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SaveDialogConfig<'a> {
    pub title: &'a str,
    pub default_name: &'a str,
    pub filter_name: &'a str,
    pub filter_extensions: &'a [&'a str],
}

/// Exact destination state accepted after a save picker returns.
///
/// Native publication converts this opaque token into a durable
/// compare-and-exchange precondition. Browser/test backends use
/// `BackendManaged`, because their destination is either a browser download or
/// an in-memory test sink rather than a reopenable local pathname.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ObservedExportDestination {
    path: PathBuf,
    #[cfg(not(target_arch = "wasm32"))]
    expectation: ExportDestinationExpectation,
}

impl ObservedExportDestination {
    fn backend_managed(path: PathBuf) -> Self {
        Self {
            path,
            #[cfg(not(target_arch = "wasm32"))]
            expectation: ExportDestinationExpectation::BackendManaged,
        }
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExportDestinationExpectation {
    Missing,
    Digest([u8; 32]),
    BackendManaged,
}

/// IO abstraction for export workflows.
///
/// This allows export behavior to be fully unit tested without invoking
/// native dialogs or filesystem writes.
pub(crate) trait ExportWorkflowIo {
    fn show_save_dialog(&self, config: SaveDialogConfig<'_>) -> Result<Option<PathBuf>, String>;
    fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String>;

    /// Accept the destination state returned by the save picker. Native
    /// backends override this with an exact missing/digest observation.
    fn observe_destination(&self, path: &Path) -> Result<ObservedExportDestination, String> {
        Ok(ObservedExportDestination::backend_managed(
            path.to_path_buf(),
        ))
    }

    /// Publish already-complete text bytes against the accepted destination.
    fn write_text_file_observed(
        &self,
        destination: &ObservedExportDestination,
        contents: &str,
    ) -> Result<(), String> {
        self.write_text_file(destination.path(), contents)
    }

    /// Publish a generated artifact only when its destination is still absent.
    ///
    /// Automatic exports do not have an interactive overwrite decision, so
    /// they must preserve an existing file. Test and browser backends may use
    /// their ordinary write behavior: browser downloads are copies selected by
    /// the user agent rather than writes to a reopenable local pathname.
    fn write_new_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
        self.write_text_file(path, contents)
    }

    /// Publish already-complete binary bytes against the accepted destination.
    fn write_bytes_file_observed(
        &self,
        destination: &ObservedExportDestination,
        contents: &[u8],
        mime_type: &str,
    ) -> Result<(), String> {
        let _ = (destination, contents, mime_type);
        Err("this export backend does not support binary publication".to_owned())
    }

    fn write_waveform_csv(
        &self,
        dataset: &crate::io::WaveformDataset,
        path: &Path,
    ) -> Result<(), String>;

    /// Serialize the complete waveform artifact, then publish it against the
    /// exact destination state accepted after the picker returned.
    fn write_waveform_csv_observed(
        &self,
        dataset: &crate::io::WaveformDataset,
        destination: &ObservedExportDestination,
    ) -> Result<(), String> {
        self.write_waveform_csv(dataset, destination.path())
    }

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
        let destination = self.observe_destination(path)?;
        self.write_text_file_observed(&destination, contents)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn observe_destination(&self, path: &Path) -> Result<ObservedExportDestination, String> {
        let expectation = match crate::io::durable_file::observe_expected_content(path)
            .map_err(|error| error.to_string())?
        {
            crate::io::durable_file::ExpectedContent::Missing => {
                ExportDestinationExpectation::Missing
            }
            crate::io::durable_file::ExpectedContent::Digest(digest) => {
                ExportDestinationExpectation::Digest(digest)
            }
        };
        Ok(ObservedExportDestination {
            path: path.to_path_buf(),
            expectation,
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn write_text_file_observed(
        &self,
        destination: &ObservedExportDestination,
        contents: &str,
    ) -> Result<(), String> {
        publish_observed_bytes(destination, contents.as_bytes())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn write_new_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
        crate::io::durable_file::compare_exchange_bytes(
            path,
            crate::io::durable_file::ExpectedContent::Missing,
            contents.as_bytes(),
        )
        .map_err(|error| error.to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn write_bytes_file_observed(
        &self,
        destination: &ObservedExportDestination,
        contents: &[u8],
        _mime_type: &str,
    ) -> Result<(), String> {
        publish_observed_bytes(destination, contents)
    }

    #[cfg(target_arch = "wasm32")]
    fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
        crate::common::browser_download::download_text_file(path, contents)
    }

    #[cfg(target_arch = "wasm32")]
    fn write_bytes_file_observed(
        &self,
        destination: &ObservedExportDestination,
        contents: &[u8],
        mime_type: &str,
    ) -> Result<(), String> {
        crate::common::browser_download::download_bytes_file(
            destination.path(),
            contents,
            mime_type,
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn write_waveform_csv(
        &self,
        dataset: &crate::io::WaveformDataset,
        path: &Path,
    ) -> Result<(), String> {
        let destination = self.observe_destination(path)?;
        self.write_waveform_csv_observed(dataset, &destination)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn write_waveform_csv_observed(
        &self,
        dataset: &crate::io::WaveformDataset,
        destination: &ObservedExportDestination,
    ) -> Result<(), String> {
        let contents =
            crate::io::WaveformWriter::new(crate::io::WaveformFormat::Csv).write_text(dataset)?;
        publish_observed_bytes(destination, contents.as_bytes())
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

pub(crate) fn deterministic_stored_zip(entries: &[(&str, &[u8])]) -> Result<Vec<u8>, String> {
    if entries.is_empty() || entries.len() > u16::MAX as usize {
        return Err("CI evidence package has an invalid entry count".to_owned());
    }
    let mut archive = Vec::new();
    let mut directory = Vec::new();
    for (name, contents) in entries {
        let name = name.as_bytes();
        let name_len = u16::try_from(name.len())
            .map_err(|_| "CI evidence package entry name is too long".to_owned())?;
        let content_len = u32::try_from(contents.len())
            .map_err(|_| "CI evidence package entry is too large".to_owned())?;
        let offset = u32::try_from(archive.len())
            .map_err(|_| "CI evidence package exceeds ZIP32 limits".to_owned())?;
        let crc = crc32(contents);

        push_u32(&mut archive, 0x0403_4b50);
        push_u16(&mut archive, 20);
        push_u16(&mut archive, 0x0800);
        push_u16(&mut archive, 0);
        push_u16(&mut archive, 0);
        push_u16(&mut archive, 0x0021);
        push_u32(&mut archive, crc);
        push_u32(&mut archive, content_len);
        push_u32(&mut archive, content_len);
        push_u16(&mut archive, name_len);
        push_u16(&mut archive, 0);
        archive.extend_from_slice(name);
        archive.extend_from_slice(contents);

        push_u32(&mut directory, 0x0201_4b50);
        push_u16(&mut directory, 20);
        push_u16(&mut directory, 20);
        push_u16(&mut directory, 0x0800);
        push_u16(&mut directory, 0);
        push_u16(&mut directory, 0);
        push_u16(&mut directory, 0x0021);
        push_u32(&mut directory, crc);
        push_u32(&mut directory, content_len);
        push_u32(&mut directory, content_len);
        push_u16(&mut directory, name_len);
        push_u16(&mut directory, 0);
        push_u16(&mut directory, 0);
        push_u16(&mut directory, 0);
        push_u16(&mut directory, 0);
        push_u32(&mut directory, 0);
        push_u32(&mut directory, offset);
        directory.extend_from_slice(name);
    }
    let directory_offset = u32::try_from(archive.len())
        .map_err(|_| "CI evidence package exceeds ZIP32 limits".to_owned())?;
    let directory_len = u32::try_from(directory.len())
        .map_err(|_| "CI evidence package exceeds ZIP32 limits".to_owned())?;
    archive.extend_from_slice(&directory);
    push_u32(&mut archive, 0x0605_4b50);
    push_u16(&mut archive, 0);
    push_u16(&mut archive, 0);
    push_u16(&mut archive, entries.len() as u16);
    push_u16(&mut archive, entries.len() as u16);
    push_u32(&mut archive, directory_len);
    push_u32(&mut archive, directory_offset);
    push_u16(&mut archive, 0);
    Ok(archive)
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & (0_u32.wrapping_sub(crc & 1)));
        }
    }
    !crc
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

#[cfg(not(target_arch = "wasm32"))]
fn publish_observed_bytes(
    destination: &ObservedExportDestination,
    contents: &[u8],
) -> Result<(), String> {
    let expected = match destination.expectation {
        ExportDestinationExpectation::Missing => crate::io::durable_file::ExpectedContent::Missing,
        ExportDestinationExpectation::Digest(digest) => {
            crate::io::durable_file::ExpectedContent::Digest(digest)
        }
        ExportDestinationExpectation::BackendManaged => {
            return Err(format!(
                "export destination '{}' has no native publication authorization",
                destination.path.display()
            ));
        }
    };
    crate::io::durable_file::compare_exchange_bytes(&destination.path, expected, contents)
        .map_err(|error| error.to_string())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::{ExportWorkflowIo, NativeExportWorkflowIo, deterministic_stored_zip};
    use sha2::{Digest as _, Sha256};

    fn stored_entries(bytes: &[u8]) -> Vec<(String, Vec<u8>)> {
        let read_u16 = |at: usize| u16::from_le_bytes([bytes[at], bytes[at + 1]]) as usize;
        let read_u32 = |at: usize| {
            u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]) as usize
        };
        let mut offset = 0;
        let mut entries = Vec::new();
        while bytes.get(offset..offset + 4) == Some(&0x0403_4b50_u32.to_le_bytes()) {
            let size = read_u32(offset + 18);
            let name_len = read_u16(offset + 26);
            let extra_len = read_u16(offset + 28);
            let name_start = offset + 30;
            let content_start = name_start + name_len + extra_len;
            entries.push((
                std::str::from_utf8(&bytes[name_start..name_start + name_len])
                    .expect("UTF-8 ZIP entry")
                    .to_owned(),
                bytes[content_start..content_start + size].to_vec(),
            ));
            offset = content_start + size;
        }
        entries
    }

    #[test]
    fn deterministic_ci_zip_retains_exact_entries_contents_and_digest() {
        let entries = [
            ("rspice-golden-regression.xml", b"<testsuite/>".as_slice()),
            (
                "rspice-golden-regression.tap",
                b"TAP version 13\n1..0\n".as_slice(),
            ),
        ];
        let first = deterministic_stored_zip(&entries).expect("build ZIP evidence");
        let second = deterministic_stored_zip(&entries).expect("repeat ZIP evidence");
        assert_eq!(first, second);
        assert_eq!(Sha256::digest(&first), Sha256::digest(&second));
        assert_eq!(
            stored_entries(&first),
            vec![
                (
                    "rspice-golden-regression.xml".to_owned(),
                    b"<testsuite/>".to_vec(),
                ),
                (
                    "rspice-golden-regression.tap".to_owned(),
                    b"TAP version 13\n1..0\n".to_vec(),
                ),
            ]
        );
        let changed =
            deterministic_stored_zip(&[entries[0], (entries[1].0, b"TAP version 13\n1..1\n")])
                .expect("build changed ZIP evidence");
        assert_ne!(Sha256::digest(first), Sha256::digest(changed));
    }

    #[test]
    fn automatic_text_export_never_overwrites_an_existing_artifact() {
        let root = std::env::temp_dir().join(format!(
            "rspice-export-create-only-{}",
            uuid::Uuid::new_v4()
        ));
        let path = root.join("analysis.s2p");
        let io = NativeExportWorkflowIo;

        io.write_new_text_file(&path, "first generation")
            .expect("create first automatic export");
        let conflict = io
            .write_new_text_file(&path, "second generation")
            .expect_err("existing automatic export must not be overwritten");

        assert!(conflict.contains("destination changed"));
        assert_eq!(
            std::fs::read_to_string(&path).expect("read preserved export"),
            "first generation"
        );

        std::fs::remove_dir_all(root).expect("remove isolated export test directory");
    }

    #[test]
    fn observed_export_rejects_late_external_change_without_clobbering_it() {
        let root = std::env::temp_dir().join(format!(
            "rspice-export-observed-cas-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create isolated export test directory");
        let path = root.join("waveforms.csv");
        std::fs::write(&path, "picker-time bytes").expect("write picker-time fixture");
        let io = NativeExportWorkflowIo;
        let destination = io
            .observe_destination(&path)
            .expect("capture exact picker-time destination");

        std::fs::write(&path, "external editor bytes").expect("inject late external edit");
        let conflict = io
            .write_text_file_observed(&destination, "RSpice export bytes")
            .expect_err("late external edit must revoke publication authority");

        assert!(conflict.contains("destination changed"), "{conflict}");
        assert_eq!(
            std::fs::read_to_string(&path).expect("read preserved external edit"),
            "external editor bytes"
        );

        std::fs::remove_dir_all(root).expect("remove isolated export test directory");
    }

    #[test]
    fn observed_missing_export_rejects_late_external_creation() {
        let root = std::env::temp_dir().join(format!(
            "rspice-export-observed-missing-{}",
            uuid::Uuid::new_v4()
        ));
        let path = root.join("new-export.svg");
        let io = NativeExportWorkflowIo;
        let destination = io
            .observe_destination(&path)
            .expect("capture missing picker-time destination");

        std::fs::write(&path, "late external creation").expect("inject late external creation");
        let conflict = io
            .write_text_file_observed(&destination, "RSpice export bytes")
            .expect_err("late external creation must revoke publication authority");

        assert!(conflict.contains("destination changed"), "{conflict}");
        assert_eq!(
            std::fs::read_to_string(&path).expect("read preserved external creation"),
            "late external creation"
        );

        std::fs::remove_dir_all(root).expect("remove isolated export test directory");
    }
}
