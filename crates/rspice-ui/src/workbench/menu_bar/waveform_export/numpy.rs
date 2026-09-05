//! NumPy publication: one array, or an archive of named arrays.
//!
//! The two differ in exactly one way that matters to a reader, and it is the
//! reason both exist.
//!
//! An `.npy` file is *one array*. The format has nowhere to put a signal name,
//! so a single array cannot say which column is which. This encoder does not
//! pretend otherwise: it writes the coordinate as column 0 and the signals
//! after it in the dataset's own order, and it states that order back to the
//! reader on every publication. A sidecar naming the columns is not an option,
//! because `.npy` is one file.
//!
//! An `.npz` archive is a ZIP of `.npy` members, and a member has a name, so
//! the names survive. What still does not survive is everything that is not an
//! array: units, the analysis domain, the producing RSpice version. RSpice's
//! own NPZ reader accepts an archive of equal-length arrays and nothing else —
//! a `manifest.json` member makes the archive unreadable to it — so the
//! metadata is stated to the reader rather than smuggled into a member that
//! would break the round trip. A reader who needs it wants the RSpice Dataset
//! Bundle.
//!
//! # Complex data
//!
//! An AC result is published as `complex128`, not as separate real and
//! imaginary columns: the phase is then a property of the value rather than a
//! convention two columns share. In an `.npy` the whole array takes the
//! complex dtype, since one array has one dtype; in an `.npz` only the complex
//! signals do, so a mixed result costs nothing in the members that are real.

use std::collections::BTreeSet;

use super::{
    ALL_TRACES_HIDDEN_MESSAGE, NO_ACTIVE_ANALYSIS_MESSAGE, NO_SAMPLES_MESSAGE, exported_waveforms,
    note_result_export_failure, note_result_export_success,
};
use crate::workbench::app_state::AppState;
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::workflows::export_workflow::{
    ExportWorkflowIo, SaveDialogConfig, deterministic_stored_zip,
};

use npyz::WriterBuilder as _;
use num_complex::Complex64;

/// `result_import_workflow::MAX_RESULT_COLUMNS`, and
/// `result_import_adapters::MAX_ARCHIVE_MEMBERS`, which are the same number.
/// Both are private to their modules; an export above either is a file this
/// product refuses to read, so the ceiling is enforced here rather than
/// discovered on re-import.
const MAX_COLUMNS: usize = 1_024;

/// NumPy has no registered media type. `application/octet-stream` is what the
/// bytes are; `.npz` is genuinely a ZIP and says so.
const NPY_MIME_TYPE: &str = "application/octet-stream";
const NPZ_MIME_TYPE: &str = "application/zip";

/// How many column names a completion message spells before it summarises.
/// The order is the only key an `.npy` reader has, so the cap is generous.
const MAX_STATED_COLUMNS: usize = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NumpyKind {
    /// One array, `.npy`.
    Array,
    /// One named array per signal, `.npz`.
    Archive,
}

impl NumpyKind {
    const fn extension(self) -> &'static str {
        match self {
            Self::Array => "npy",
            Self::Archive => "npz",
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Array => "NumPy .npy",
            Self::Archive => "NumPy .npz",
        }
    }

    const fn mime_type(self) -> &'static str {
        match self {
            Self::Array => NPY_MIME_TYPE,
            Self::Archive => NPZ_MIME_TYPE,
        }
    }

    const fn dialog_title(self) -> &'static str {
        match self {
            Self::Array => "Export NumPy Array",
            Self::Archive => "Export NumPy Archive",
        }
    }

    const fn filter_name(self) -> &'static str {
        match self {
            Self::Array => "NumPy Array",
            Self::Archive => "NumPy Archive",
        }
    }
}

#[derive(Debug)]
struct NumpySignal {
    name: String,
    real: Vec<f64>,
    imag: Option<Vec<f64>>,
}

/// One rectangular table, with the complex signals still complex.
#[derive(Debug)]
pub(super) struct NumpyExport {
    coordinate_name: &'static str,
    coordinate: Vec<f64>,
    signals: Vec<NumpySignal>,
}

impl NumpyExport {
    fn is_complex(&self) -> bool {
        self.signals.iter().any(|signal| signal.imag.is_some())
    }

    fn columns(&self) -> usize {
        self.signals.len() + 1
    }

    /// The sentence an `.npy` reader needs, because the file itself cannot
    /// carry it.
    fn column_order(&self) -> String {
        let mut order = String::from("column order = ");
        order.push_str(self.coordinate_name);
        for signal in self.signals.iter().take(MAX_STATED_COLUMNS) {
            order.push_str(", ");
            order.push_str(&signal.name);
        }
        if self.signals.len() > MAX_STATED_COLUMNS {
            order.push_str(&format!(
                ", and {} further signals in the dataset's order",
                self.signals.len() - MAX_STATED_COLUMNS
            ));
        }
        order
    }
}

/// The coordinate name each analysis domain publishes under.
///
/// These three are exactly the spellings RSpice's own NPZ reader recognises as
/// a coordinate, and the ones it maps back onto an analysis domain. Publishing
/// any other name would produce an archive this product could not reopen.
fn coordinate_name(analysis_type: crate::state::AnalysisType) -> &'static str {
    match analysis_type {
        crate::state::AnalysisType::Transient => "time",
        crate::state::AnalysisType::Ac => "frequency",
        _ => "sweep",
    }
}

pub(super) fn prepare_numpy(
    analysis: &crate::state::AnalysisResult,
    waveforms: &[&crate::state::WaveformData],
) -> Result<NumpyExport, String> {
    let reference = waveforms
        .iter()
        .filter(|waveform| !waveform.x.is_empty())
        .max_by_key(|waveform| waveform.x.len())
        .ok_or_else(|| NO_SAMPLES_MESSAGE.to_owned())?;
    let coordinate = reference.x.as_ref().to_vec();

    let mut signals = Vec::with_capacity(waveforms.len());
    for waveform in waveforms {
        // A NumPy array is rectangular, so a column that does not stand on the
        // shared coordinate has no honest place in it.
        if waveform.x.as_ref() != coordinate.as_slice() {
            return Err(format!(
                "A NumPy export is one rectangular table, so every column must stand on the same \
                 coordinate samples. '{}' carries its own x-axis samples. Export this result as \
                 CSV or an RSpice bundle instead.",
                waveform.name
            ));
        }
        if let Some(complex) = &waveform.complex {
            signals.push(NumpySignal {
                name: complex.source_name.clone(),
                real: complex.real.as_ref().to_vec(),
                imag: Some(complex.imag.as_ref().to_vec()),
            });
        } else {
            signals.push(NumpySignal {
                name: waveform.name.clone(),
                real: waveform.y.as_ref().to_vec(),
                imag: None,
            });
        }
    }
    if signals.is_empty() {
        return Err(NO_SAMPLES_MESSAGE.to_owned());
    }
    for signal in &signals {
        if signal.real.len() != coordinate.len()
            || signal
                .imag
                .as_ref()
                .is_some_and(|imag| imag.len() != coordinate.len())
        {
            return Err(format!(
                "'{}' has {} samples against {} coordinate samples; the export is refused rather \
                 than padded or truncated.",
                signal.name,
                signal.real.len(),
                coordinate.len()
            ));
        }
    }
    let export = NumpyExport {
        coordinate_name: coordinate_name(analysis.analysis_type),
        coordinate,
        signals,
    };
    if export.columns() > MAX_COLUMNS {
        return Err(format!(
            "This result has {} columns; RSpice reads at most {MAX_COLUMNS} from a NumPy source, \
             so publishing it would produce a file this build could not reopen. Hide traces, or \
             export an RSpice bundle.",
            export.columns()
        ));
    }
    Ok(export)
}

fn npy_error(error: std::io::Error) -> String {
    format!("The NumPy array could not be written: {error}")
}

fn encode_real_array(shape: &[u64], values: &[f64]) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    let mut writer = npyz::WriteOptions::<f64>::new()
        .default_dtype()
        .shape(shape)
        .writer(&mut bytes)
        .begin_nd()
        .map_err(npy_error)?;
    writer.extend(values.iter().copied()).map_err(npy_error)?;
    writer.finish().map_err(npy_error)?;
    Ok(bytes)
}

fn encode_complex_array(shape: &[u64], values: &[Complex64]) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    let mut writer = npyz::WriteOptions::<Complex64>::new()
        .default_dtype()
        .shape(shape)
        .writer(&mut bytes)
        .begin_nd()
        .map_err(npy_error)?;
    writer.extend(values.iter().copied()).map_err(npy_error)?;
    writer.finish().map_err(npy_error)?;
    Ok(bytes)
}

/// One 2-D array, C order, coordinate first.
pub(super) fn encode_npy(export: &NumpyExport) -> Result<Vec<u8>, String> {
    let rows = export.coordinate.len();
    let columns = export.columns();
    let shape = [rows as u64, columns as u64];
    if export.is_complex() {
        let mut values = Vec::with_capacity(rows * columns);
        for row in 0..rows {
            values.push(Complex64::new(export.coordinate[row], 0.0));
            for signal in &export.signals {
                values.push(Complex64::new(
                    signal.real[row],
                    signal.imag.as_ref().map_or(0.0, |imag| imag[row]),
                ));
            }
        }
        encode_complex_array(&shape, &values)
    } else {
        let mut values = Vec::with_capacity(rows * columns);
        for row in 0..rows {
            values.push(export.coordinate[row]);
            for signal in &export.signals {
                values.push(signal.real[row]);
            }
        }
        encode_real_array(&shape, &values)
    }
}

/// A member name RSpice's own archive reader will accept.
///
/// The reader refuses a member whose name is absolute, contains `..`, or
/// contains a backslash, and it identifies an array by the file stem, so two
/// signals whose names differ only in case collide. Every one of those is
/// refused here, by name, rather than published as an archive this product
/// would not reopen.
fn archive_member_name(name: &str) -> Result<String, String> {
    if name.trim().is_empty() {
        return Err(
            "A signal with no name cannot become an archive member; export CSV instead.".to_owned(),
        );
    }
    if name.starts_with('/') || name.contains("..") || name.contains('\\') {
        return Err(format!(
            "'{name}' cannot be an archive member name: RSpice refuses an archive member that is \
             absolute, contains '..', or contains a backslash. Export CSV or an RSpice bundle."
        ));
    }
    Ok(format!("{name}.npy"))
}

/// One `.npy` member per signal, plus the coordinate, in a stored ZIP.
pub(super) fn encode_npz(export: &NumpyExport) -> Result<Vec<u8>, String> {
    let members = export.columns();
    if members > MAX_COLUMNS {
        return Err(format!(
            "This result needs {members} archive members; RSpice reads at most {MAX_COLUMNS}."
        ));
    }
    let rows = [export.coordinate.len() as u64];
    let mut names = Vec::with_capacity(members);
    let mut seen = BTreeSet::new();
    names.push(archive_member_name(export.coordinate_name)?);
    seen.insert(export.coordinate_name.to_ascii_lowercase());
    for signal in &export.signals {
        let member = archive_member_name(&signal.name)?;
        if !seen.insert(signal.name.to_ascii_lowercase()) {
            return Err(format!(
                "Two signals both claim the archive member '{}'. An archive names its arrays, so \
                 the names have to differ; RSpice compares them without regard to case.",
                signal.name
            ));
        }
        names.push(member);
    }

    let mut payloads = Vec::with_capacity(members);
    payloads.push(encode_real_array(&rows, &export.coordinate)?);
    for signal in &export.signals {
        let bytes = match &signal.imag {
            Some(imag) => {
                let values = signal
                    .real
                    .iter()
                    .zip(imag)
                    .map(|(re, im)| Complex64::new(*re, *im))
                    .collect::<Vec<_>>();
                encode_complex_array(&rows, &values)?
            }
            None => encode_real_array(&rows, &signal.real)?,
        };
        payloads.push(bytes);
    }

    let entries = names
        .iter()
        .map(String::as_str)
        .zip(payloads.iter().map(Vec::as_slice))
        .collect::<Vec<_>>();
    deterministic_stored_zip(&entries)
}

pub(super) fn export_numpy(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
    kind: NumpyKind,
) {
    let prepared = match displayed.primary_analysis(state) {
        Some(analysis) => {
            let waveforms = exported_waveforms(state, displayed.dataset_id, analysis);
            if waveforms.is_empty() {
                Err(if analysis.waveforms.is_empty() {
                    NO_SAMPLES_MESSAGE.to_owned()
                } else {
                    ALL_TRACES_HIDDEN_MESSAGE.to_owned()
                })
            } else {
                prepare_numpy(analysis, &waveforms)
            }
        }
        None => Err(NO_ACTIVE_ANALYSIS_MESSAGE.to_owned()),
    };
    let prepared = match prepared {
        Ok(prepared) => prepared,
        Err(message) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(message));
            return;
        }
    };
    // Encode before the picker opens: a refused export never asks the reader
    // for a destination it cannot fill.
    let bytes = match match kind {
        NumpyKind::Array => encode_npy(&prepared),
        NumpyKind::Archive => encode_npz(&prepared),
    } {
        Ok(bytes) => bytes,
        Err(error) => {
            note_result_export_failure(state, format!("{} encoding failed: {error}", kind.label()));
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "{} export failed before destination selection: {error}",
                kind.label()
            )));
            return;
        }
    };

    let extension = kind.extension();
    let default_name = format!("waveforms.{extension}");
    let filter_extensions = [extension];
    let (published_path, export) = match io.show_save_dialog(SaveDialogConfig {
        title: kind.dialog_title(),
        default_name: &default_name,
        filter_name: kind.filter_name(),
        filter_extensions: &filter_extensions,
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, extension);
            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_bytes_file_observed(&destination, &bytes, kind.mime_type())
            });
            (path, export)
        }
        Ok(None) => return,
        Err(error) => (std::path::PathBuf::from(default_name), Err(error)),
    };
    match export {
        Ok(()) => {
            note_result_export_success(state, kind.label());
            state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                crate::workbench::workflows::export_workflow::export_completion_message(
                    kind.label(),
                    &published_path,
                    Some(completion_detail(&prepared, kind)),
                    io,
                ),
            ));
        }
        Err(error) => {
            note_result_export_failure(state, format!("{} export failed: {error}", kind.label()));
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "{} export failed: {error}",
                kind.label()
            )));
        }
    }
}

/// What the reader is told they now have. For `.npy` this is load-bearing:
/// the column order is the only key to the file.
fn completion_detail(export: &NumpyExport, kind: NumpyKind) -> String {
    let dtype = if export.is_complex() {
        "complex128"
    } else {
        "float64"
    };
    match kind {
        NumpyKind::Array => format!(
            "one {dtype} array, {} rows x {} columns; {}. An .npy file carries no names, so this \
             order is the only key to its columns.",
            export.coordinate.len(),
            export.columns(),
            export.column_order()
        ),
        NumpyKind::Archive => format!(
            "{} arrays of {} samples: '{}' plus one per signal, named. {dtype} values. Units, the \
             analysis domain and the producing version are not carried — an .npz member is an \
             array and nothing else.",
            export.columns(),
            export.coordinate.len(),
            export.coordinate_name,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, WaveformData};
    use crate::workbench::workflows::result_import_workflow::parse_result_dataset;

    fn waveform(name: &str, x: Vec<f64>, y: Vec<f64>) -> WaveformData {
        WaveformData::new(name.to_owned(), x, y, "#4f81bd")
    }

    fn complex_waveform(
        name: &str,
        source_name: &str,
        x: Vec<f64>,
        display: Vec<f64>,
        real: Vec<f64>,
        imag: Vec<f64>,
    ) -> WaveformData {
        WaveformData::new(name.to_owned(), x, display, "#4f81bd").with_complex_components(
            source_name.to_owned(),
            real,
            imag,
        )
    }

    fn analysis(analysis_type: AnalysisType) -> AnalysisResult {
        AnalysisResult::new(1, analysis_type, "A")
    }

    fn prepared(
        analysis_type: AnalysisType,
        waveforms: &[WaveformData],
    ) -> Result<NumpyExport, String> {
        let analysis = analysis(analysis_type);
        let borrowed = waveforms.iter().collect::<Vec<_>>();
        prepare_numpy(&analysis, &borrowed)
    }

    #[test]
    fn a_real_array_puts_the_coordinate_first_and_says_so() {
        let export = prepared(
            AnalysisType::Transient,
            &[
                waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 12.0]),
                waveform("V(in)", vec![0.0, 1.0, 2.0], vec![20.0, 21.0, 22.0]),
            ],
        )
        .expect("a shared-axis transient prepares");
        assert_eq!(export.column_order(), "column order = time, V(out), V(in)");

        let bytes = encode_npy(&export).expect("encodes");
        // `\x93NUMPY`, version 1.0, then a C-order 3x3 float64 header.
        assert_eq!(&bytes[..8], b"\x93NUMPY\x01\x00");
        let header = String::from_utf8_lossy(&bytes[10..128]);
        assert!(header.contains("'descr': '<f8'"), "{header}");
        assert!(header.contains("'fortran_order': False"), "{header}");
        assert!(header.contains("'shape': (3, 3, )"), "{header}");
        assert_eq!(bytes.len(), 128 + 9 * 8);
        // Row 0 is the coordinate then the two signals, in that order.
        let first_row = bytes[128..128 + 24]
            .chunks_exact(8)
            .map(|chunk| f64::from_le_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>();
        assert_eq!(first_row, [0.0, 10.0, 20.0]);
    }

    #[test]
    fn an_npy_matrix_reimports_with_its_values_in_column_order() {
        let export = prepared(
            AnalysisType::Transient,
            &[
                waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 12.0]),
                waveform("V(in)", vec![0.0, 1.0, 2.0], vec![20.0, 21.0, 22.0]),
            ],
        )
        .expect("prepares");
        let bytes = encode_npy(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.npy", &bytes).expect("re-imports");
        assert_eq!(parsed.sample_count, 3);
        assert_eq!(parsed.waveforms.len(), 2);
        assert_eq!(parsed.waveforms[0].x.as_slice(), [0.0, 1.0, 2.0]);
        assert_eq!(parsed.waveforms[0].y.as_slice(), [10.0, 11.0, 12.0]);
        assert_eq!(parsed.waveforms[1].y.as_slice(), [20.0, 21.0, 22.0]);
        // The names are gone, which is the property the export states rather
        // than hides: one array cannot carry them.
        assert_eq!(parsed.waveforms[0].name, "signal_1");
        assert_eq!(parsed.waveforms[1].name, "signal_2");
    }

    #[test]
    fn an_ac_array_is_complex128() {
        let export = prepared(
            AnalysisType::Ac,
            &[complex_waveform(
                "V(out) magnitude",
                "V(out)",
                vec![1.0, 10.0, 100.0],
                vec![1.0, 0.5, 0.25],
                vec![1.0, 0.5, 0.25],
                vec![0.0, -0.5, -0.25],
            )],
        )
        .expect("prepares");
        assert!(export.is_complex());
        let bytes = encode_npy(&export).expect("encodes");
        let header = String::from_utf8_lossy(&bytes[10..128]);
        assert!(header.contains("'descr': '<c16'"), "{header}");
        assert!(header.contains("'shape': (3, 2, )"), "{header}");
        assert_eq!(bytes.len(), 128 + 6 * 16);
        assert_eq!(export.column_order(), "column order = frequency, V(out)");
    }

    #[test]
    fn an_archive_names_every_array_and_reimports_with_those_names() {
        let export = prepared(
            AnalysisType::Transient,
            &[
                waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 12.0]),
                waveform("I(R1)", vec![0.0, 1.0, 2.0], vec![20.0, 21.0, 22.0]),
            ],
        )
        .expect("prepares");
        let bytes = encode_npz(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.npz", &bytes).expect("re-imports");
        assert_eq!(parsed.coordinate_name, "time");
        assert_eq!(parsed.analysis_type, AnalysisType::Transient);
        let mut names = parsed
            .waveforms
            .iter()
            .map(|waveform| waveform.name.clone())
            .collect::<Vec<_>>();
        names.sort();
        assert_eq!(names, ["I(R1)", "V(out)"]);
        let out = parsed
            .waveforms
            .iter()
            .find(|waveform| waveform.name == "V(out)")
            .expect("named");
        assert_eq!(out.x.as_slice(), [0.0, 1.0, 2.0]);
        assert_eq!(out.y.as_slice(), [10.0, 11.0, 12.0]);
    }

    #[test]
    fn an_ac_archive_reimports_its_complex_components() {
        let export = prepared(
            AnalysisType::Ac,
            &[complex_waveform(
                "V(out) magnitude",
                "V(out)",
                vec![1.0, 10.0, 100.0],
                vec![1.0, 0.5, 0.25],
                vec![1.0, 0.5, 0.25],
                vec![0.0, -0.5, -0.25],
            )],
        )
        .expect("prepares");
        let bytes = encode_npz(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.npz", &bytes).expect("re-imports");
        assert_eq!(parsed.coordinate_name, "frequency");
        assert_eq!(parsed.analysis_type, AnalysisType::Ac);
        let complex = parsed.waveforms[0]
            .complex
            .as_ref()
            .expect("the complex components survive");
        assert_eq!(complex.real.as_slice(), [1.0, 0.5, 0.25]);
        assert_eq!(complex.imag.as_slice(), [0.0, -0.5, -0.25]);
    }

    #[test]
    fn the_same_dataset_publishes_the_same_bytes_twice() {
        let waveforms = [
            waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 12.0]),
            waveform("V(in)", vec![0.0, 1.0, 2.0], vec![20.0, 21.0, 22.0]),
        ];
        let first = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        let second = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        assert_eq!(
            encode_npy(&first).expect("encodes"),
            encode_npy(&second).expect("encodes")
        );
        assert_eq!(
            encode_npz(&first).expect("encodes"),
            encode_npz(&second).expect("encodes")
        );
    }

    #[test]
    fn a_ragged_result_is_refused_rather_than_padded() {
        let error = prepared(
            AnalysisType::Transient,
            &[
                waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 12.0]),
                waveform("V(in)", vec![0.0, 1.0], vec![20.0, 21.0]),
            ],
        )
        .expect_err("a NumPy array is rectangular");
        assert!(error.contains("'V(in)'"), "{error}");
        assert!(error.contains("rectangular"), "{error}");
    }

    #[test]
    fn two_signals_that_differ_only_in_case_cannot_share_one_archive() {
        let export = prepared(
            AnalysisType::Transient,
            &[
                waveform("V(out)", vec![0.0, 1.0], vec![1.0, 2.0]),
                waveform("v(OUT)", vec![0.0, 1.0], vec![3.0, 4.0]),
            ],
        )
        .expect("prepares");
        // One array each is fine; an archive has to name them.
        assert!(encode_npy(&export).is_ok());
        let error = encode_npz(&export).expect_err("the member names collide");
        assert!(error.contains("v(OUT)"), "{error}");
    }

    #[test]
    fn a_member_name_the_reader_refuses_is_refused_at_export() {
        let export = prepared(
            AnalysisType::Transient,
            &[waveform("V(../escape)", vec![0.0, 1.0], vec![1.0, 2.0])],
        )
        .expect("prepares");
        let error = encode_npz(&export).expect_err("'..' is refused by the archive reader");
        assert!(error.contains("'..'"), "{error}");
    }
}
