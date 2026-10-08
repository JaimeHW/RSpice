//! HDF5 publication: the displayed analysis as one section of named columns.
//!
//! The layout is [`rspice_core::io::hdf5`]'s, which is also what the CLI's
//! `--format hdf5` writes and what RSpice's own HDF5 reader expects. A file
//! published here therefore reopens in this product with its names, units and
//! complex pairs intact, and in `h5py` or MATLAB as ordinary `f64` arrays.
//!
//! # What an HDF5 dataset carries
//!
//! One analysis, under one of the three section names the layout defines:
//! `transient`, `dc_sweep` and `ac`. A result whose domain is none of those —
//! an operating point, a noise spectrum, a pole-zero set, a stack of several
//! displayed analyses — is refused by name. Publishing it under one of those
//! headings would not lose a number, but it would make every reader, this
//! product's included, call the result something it is not.
//!
//! Digital event timelines are not carried either: they are an irregular
//! schedule rather than a table on a shared coordinate, and VCD is the format
//! that holds them.
//!
//! # Complex data
//!
//! Real and complex traces retain their original representation in every
//! sampled domain. An all-complex AC result uses the native spectral layout;
//! mixed representations use the shared typed table layout.

use super::{
    ALL_TRACES_HIDDEN_MESSAGE, NO_ACTIVE_ANALYSIS_MESSAGE, NO_SAMPLES_MESSAGE, exported_waveforms,
    note_result_export_failure, note_result_export_success,
};
use crate::workbench::app_state::AppState;
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
use crate::workbench::workflows::result_import_workflow::waveform_import_limits;

use rspice_core::io::write_hdf5;
use rspice_formats::hdf5::result::{Hdf5Export, prepare_hdf5};

/// Registered by The HDF Group for HDF5 files.
const HDF5_MIME_TYPE: &str = "application/x-hdf5";

const LABEL: &str = "HDF5";
const EXTENSION: &str = "h5";

/// How many column names a completion message spells before it summarises.
const MAX_STATED_COLUMNS: usize = 12;

/// The bytes of a prepared document.
pub(super) fn encode_hdf5(export: &Hdf5Export) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    write_hdf5(&mut bytes, &export.document)
        .map_err(|error| format!("the HDF5 document could not be written: {error}"))?;
    Ok(bytes)
}

pub(super) fn export_hdf5(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
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
                prepare_hdf5(analysis, &waveforms, waveform_import_limits())
                    .map_err(|error| error.to_string())
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
    let bytes = match encode_hdf5(&prepared) {
        Ok(bytes) => bytes,
        Err(error) => {
            note_result_export_failure(state, format!("{LABEL} encoding failed: {error}"));
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "{LABEL} export failed before destination selection: {error}"
            )));
            return;
        }
    };

    let default_name = format!("waveforms.{EXTENSION}");
    let (published_path, export) = match io.show_save_dialog(SaveDialogConfig {
        title: "Export HDF5 Dataset",
        default_name: &default_name,
        filter_name: "HDF5 Dataset",
        filter_extensions: &[EXTENSION],
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, EXTENSION);
            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_bytes_file_observed(&destination, &bytes, HDF5_MIME_TYPE)
            });
            (path, export)
        }
        Ok(None) => return,
        Err(error) => (std::path::PathBuf::from(default_name), Err(error)),
    };
    match export {
        Ok(()) => {
            note_result_export_success(state, LABEL);
            state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                crate::workbench::workflows::export_workflow::export_completion_message(
                    LABEL,
                    &published_path,
                    Some(completion_detail(&prepared)),
                    io,
                ),
            ));
        }
        Err(error) => {
            note_result_export_failure(state, format!("{LABEL} export failed: {error}"));
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "{LABEL} export failed: {error}"
            )));
        }
    }
}

/// What the reader is told they now have: where the numbers are in the file,
/// so they can open it without guessing.
fn completion_detail(export: &Hdf5Export) -> String {
    let mut detail = format!(
        "group /{}: {} rows x {} signals on '{}'",
        export.section, export.rows, export.columns, export.coordinate_name
    );
    let names = export
        .document
        .groups
        .first()
        .map(|group| {
            group
                .attributes
                .iter()
                .filter(|(name, _)| name.ends_with("_name"))
                .filter_map(|(_, value)| match value {
                    rspice_core::io::Hdf5Attribute::Text(name) => Some(name.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    detail.push_str(" — ");
    detail.push_str(&names[..names.len().min(MAX_STATED_COLUMNS)].join(", "));
    if names.len() > MAX_STATED_COLUMNS {
        detail.push_str(&format!(", and {} more", names.len() - MAX_STATED_COLUMNS));
    }
    detail.push_str(
        ". Signal names, quantities and units travel as group attributes; complex signals are \
         real and imaginary datasets.",
    );
    detail
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, WaveformData};
    use crate::workbench::workflows::result_import_workflow::parse_result_dataset;

    fn waveform(name: &str, x: Vec<f64>, y: Vec<f64>) -> WaveformData {
        WaveformData::new(name.to_owned(), x, y, "#4f81bd")
    }

    fn analysis(analysis_type: AnalysisType) -> AnalysisResult {
        AnalysisResult::new(1, analysis_type, "A")
    }

    fn prepared(
        analysis_type: AnalysisType,
        waveforms: &[WaveformData],
    ) -> Result<Hdf5Export, String> {
        let analysis = analysis(analysis_type);
        let borrowed = waveforms.iter().collect::<Vec<_>>();
        prepare_hdf5(&analysis, &borrowed, waveform_import_limits())
            .map_err(|error| error.to_string())
    }

    #[test]
    fn a_transient_round_trips_its_names_units_and_values() {
        let waveforms = [
            waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 12.0]).with_unit("V"),
            waveform("I(R1)", vec![0.0, 1.0, 2.0], vec![20.0, 21.0, 22.0]).with_unit("A"),
        ];
        let export = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        let bytes = encode_hdf5(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.h5", &bytes).expect("re-imports");
        assert_eq!(parsed.analysis_type, AnalysisType::Transient);
        assert_eq!(parsed.coordinate_name, "time");
        assert_eq!(parsed.sample_count, 3);
        assert_eq!(parsed.waveforms.len(), 2);
        assert_eq!(parsed.waveforms[0].name, "V(out)");
        assert_eq!(parsed.waveforms[0].x.as_slice(), [0.0, 1.0, 2.0]);
        assert_eq!(parsed.waveforms[0].y.as_slice(), [10.0, 11.0, 12.0]);
        assert_eq!(parsed.waveforms[0].unit.as_deref(), Some("V"));
        assert_eq!(parsed.waveforms[1].name, "I(R1)");
        assert_eq!(parsed.waveforms[1].y.as_slice(), [20.0, 21.0, 22.0]);
        assert_eq!(parsed.waveforms[1].unit.as_deref(), Some("A"));
    }

    /// A one-point result publishes and reopens.
    ///
    /// A `.OP` is a single sample by construction, and the importer's
    /// `MIN_RESULT_ROWS` was two — so this writer published files this product
    /// then refused to read. An export the product cannot reopen is worse than
    /// a small file.
    #[test]
    fn a_single_sample_result_publishes_and_reopens() {
        let waveforms = [waveform("V(out)", vec![0.0], vec![2.5]).with_unit("V")];
        let export = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        assert_eq!(export.rows, 1);
        let bytes = encode_hdf5(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.h5", &bytes).expect("re-imports");
        assert_eq!(parsed.sample_count, 1);
        assert_eq!(parsed.waveforms[0].x.as_slice(), [0.0]);
        assert_eq!(parsed.waveforms[0].y.as_slice(), [2.5]);
        assert_eq!(parsed.waveforms[0].unit.as_deref(), Some("V"));
    }

    #[test]
    fn an_ac_result_round_trips_its_complex_pairs() {
        let waveforms = [WaveformData::new(
            "V(out) magnitude".to_owned(),
            vec![1.0, 10.0, 100.0],
            vec![1.0, 0.5, 0.25],
            "#4f81bd",
        )
        .with_unit("V")
        .with_complex_components(
            "V(out)".to_owned(),
            vec![1.0, 0.5, 0.25],
            vec![0.0, -0.5, -0.25],
        )];
        let export = prepared(AnalysisType::Ac, &waveforms).expect("prepares");
        let bytes = encode_hdf5(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.h5", &bytes).expect("re-imports");
        assert_eq!(parsed.analysis_type, AnalysisType::Ac);
        assert_eq!(parsed.coordinate_name, "frequency");
        // The importer names a complex trace by the magnitude it draws and
        // keeps the published name on the components it drew it from.
        assert_eq!(parsed.waveforms[0].name, "|V(out)|");
        assert_eq!(parsed.waveforms[0].unit.as_deref(), Some("V"));
        let complex = parsed.waveforms[0]
            .complex
            .as_ref()
            .expect("the complex components survive");
        assert_eq!(complex.source_name, "V(out)");
        assert_eq!(complex.real.as_slice(), [1.0, 0.5, 0.25]);
        assert_eq!(complex.imag.as_slice(), [0.0, -0.5, -0.25]);
    }

    #[test]
    fn a_dc_sweep_publishes_the_dc_sweep_section() {
        let waveforms = [waveform("V(out)", vec![0.0, 0.5, 1.0], vec![0.0, 1.0, 2.0])];
        let export = prepared(AnalysisType::DcSweep, &waveforms).expect("prepares");
        assert_eq!(export.section, "dc_sweep");
        let bytes = encode_hdf5(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.h5", &bytes).expect("re-imports");
        assert_eq!(parsed.analysis_type, AnalysisType::DcSweep);
        assert_eq!(parsed.waveforms[0].y.as_slice(), [0.0, 1.0, 2.0]);
    }

    #[test]
    fn an_ac_trace_with_no_retained_phase_remains_real() {
        let waveforms = [waveform("V(out)", vec![1.0, 10.0], vec![1.0, 0.5])];
        let export = prepared(AnalysisType::Ac, &waveforms).expect("prepares");
        let bytes = encode_hdf5(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.h5", &bytes).expect("re-imports");
        assert_eq!(parsed.waveforms[0].name, "V(out)");
        assert!(parsed.waveforms[0].complex.is_none());
        assert_eq!(parsed.waveforms[0].y.as_slice(), [1.0, 0.5]);
    }

    #[test]
    fn the_same_dataset_publishes_the_same_bytes_twice() {
        let waveforms = [waveform(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![10.0, 11.0, 12.0],
        )];
        let first = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        let second = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        assert_eq!(
            encode_hdf5(&first).expect("encodes"),
            encode_hdf5(&second).expect("encodes")
        );
    }

    #[test]
    fn a_domain_the_layout_has_no_section_for_is_refused_by_name() {
        let waveforms = [waveform("V(onoise)", vec![1.0, 10.0], vec![1e-9, 2e-9])];
        let error = prepared(AnalysisType::Noise, &waveforms)
            .expect_err("HDF5 carries three sections, and noise is not one");
        assert!(
            error.contains("transient, a DC sweep or an AC sweep"),
            "{error}"
        );
        assert!(error.contains("RSpice bundle"), "{error}");
    }

    #[test]
    fn a_ragged_result_is_refused_rather_than_padded() {
        let waveforms = [
            waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 12.0]),
            waveform("V(in)", vec![0.0, 1.0], vec![20.0, 21.0]),
        ];
        let error = prepared(AnalysisType::Transient, &waveforms)
            .expect_err("an HDF5 section is one table");
        assert!(error.contains("'V(in)'"), "{error}");
        assert!(error.contains("same coordinate samples"), "{error}");
    }

    #[test]
    fn an_unstated_unit_comes_back_unstated_rather_than_dimensionless() {
        let waveforms = [waveform("V(out)", vec![0.0, 1.0], vec![1.0, 2.0])];
        let export = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        let bytes = encode_hdf5(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.h5", &bytes).expect("re-imports");
        assert_eq!(parsed.waveforms[0].unit, None);
    }
}
