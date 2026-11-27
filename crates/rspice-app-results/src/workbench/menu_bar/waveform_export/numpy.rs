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
//! because `.npy` is one file, and for the same reason it holds one analysis:
//! the member of a family the form chose.
//!
//! An `.npz` archive is a ZIP of `.npy` members, and a member has a name, so
//! the names survive. What still does not survive is everything that is not an
//! array: units, the analysis domain, the producing RSpice version. RSpice's
//! own NPZ reader accepts an archive of equal-length arrays and nothing else —
//! a `manifest.json` member makes the archive unreadable to it — so the
//! metadata is stated to the reader rather than smuggled into a member that
//! would break the round trip. A reader who needs it wants the RSpice Dataset
//! Bundle. An archive of every member of a family keeps each member's arrays
//! under its parameter values, `r=2k/time`, `r=2k/V(out)`.
//!
//! # Complex data
//!
//! An AC result is published as `complex128`, not as separate real and
//! imaginary columns: the phase is then a property of the value rather than a
//! convention two columns share. In an `.npy` the whole array takes the
//! complex dtype, since one array has one dtype; in an `.npz` only the complex
//! signals do, so a mixed result costs nothing in the members that are real.
//!
//! # Written as it is read
//!
//! Either file is written on a Results job where this process writes it, a
//! block of each column at a time, so a run kept in a results store exports
//! whatever its length; a browser download takes the same bytes.

use super::job::ResultExportJob;
use super::members::{
    ExportFile, OwnedMembers, export_members, exported_members, publish_export, read_samples,
};
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::workflows::export_workflow::ExportWorkflowIo;
use crate::workbench::workflows::result_import_workflow::exported_waveform_limits;

use rspice_formats::numpy::result::{NumpyFacts, check_npy, check_npz, write_npy, write_npz};

/// NumPy has no registered media type. `application/octet-stream` is what the
/// bytes are; `.npz` is genuinely a ZIP and says so.
const NPY_MIME_TYPE: &str = "application/octet-stream";
const NPZ_MIME_TYPE: &str = "application/zip";

/// How many column names a completion message spells before it summarises.
/// The order is the only key an `.npy` reader has, so the cap is generous.
const MAX_STATED_COLUMNS: usize = 24;

/// Why an `.npy` export of a view of several analyses is refused.
const ONE_ARRAY_REFUSAL: &str = "An .npy file is one array, so it holds one analysis, and this \
     view shows several. Choose the analysis in Export result dataset\u{2026}, or choose NumPy \
     .npz to export every one.";

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

/// The sentence an `.npy` reader needs, because the file itself cannot
/// carry it.
fn column_order(export: &NumpyFacts) -> String {
    let mut order = String::from("column order = ");
    order.push_str(export.coordinate_name);
    for signal in export.signals.iter().take(MAX_STATED_COLUMNS) {
        order.push_str(", ");
        order.push_str(signal);
    }
    if export.signals.len() > MAX_STATED_COLUMNS {
        order.push_str(&format!(
            ", and {} further signals in the dataset's order",
            export.signals.len() - MAX_STATED_COLUMNS
        ));
    }
    order
}

pub(super) fn export_numpy(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
    kind: NumpyKind,
    signals: super::ExportedSignals,
) -> Option<ResultExportJob> {
    let reads = state.results_reads();
    let members = match exported_members(reads, displayed, signals) {
        Ok(members) => members,
        Err(message) => {
            state.prompt(message);
            return None;
        }
    };
    if kind == NumpyKind::Array && members.len() > 1 {
        state.prompt(ONE_ARRAY_REFUSAL);
        return None;
    }
    // A refused export never asks the reader for a destination it cannot fill.
    let checked = match kind {
        NumpyKind::Array => {
            let (analysis, waveforms) = &members[0];
            check_npy(*analysis, waveforms, exported_waveform_limits())
        }
        NumpyKind::Archive => check_npz(&export_members(&members), exported_waveform_limits()),
    };
    if let Err(error) = checked {
        state.prompt(error.to_string());
        return None;
    }
    // NumPy files carry arrays and nothing else, so which member of a family an `.npy` is goes
    // to the reader instead.
    let member = (members.len() == 1)
        .then(|| members[0].0.family_member().map(|member| member.label()))
        .flatten();
    let samples = read_samples(&members);
    let owned = OwnedMembers::of(&members);
    publish_export(
        state,
        io,
        ExportFile {
            label: kind.label(),
            title: kind.dialog_title(),
            default_name: format!("waveforms.{}", kind.extension()),
            filter_name: kind.filter_name(),
            extension: kind.extension(),
            mime_type: kind.mime_type(),
            text: false,
        },
        samples,
        move |out| {
            let members = owned.members();
            match kind {
                NumpyKind::Array => write_npy(
                    members[0].analysis,
                    &members[0].waveforms,
                    exported_waveform_limits(),
                    out,
                ),
                NumpyKind::Archive => write_npz(&members, exported_waveform_limits(), out),
            }
            .map(|facts| completion_detail(&facts, kind, member.as_deref()))
            .map_err(|error| error.to_string())
        },
    )
}

/// What the reader is told they now have. For `.npy` this is load-bearing:
/// the column order is the only key to the file.
fn completion_detail(export: &NumpyFacts, kind: NumpyKind, member: Option<&str>) -> String {
    let dtype = if export.complex {
        "complex128"
    } else {
        "float64"
    };
    let mut detail = match kind {
        NumpyKind::Array => format!(
            "one {dtype} array, {} rows x {} columns; {}. An .npy file carries no names, so this \
             order is the only key to its columns.",
            export.rows,
            export.columns(),
            column_order(export)
        ),
        NumpyKind::Archive if export.members > 1 => format!(
            "{} arrays of {} members, each member's under its parameter values: '{}' plus one per \
             signal, named. {dtype} values. Units, the analysis domain and the producing version \
             are not carried — an .npz member is an array and nothing else.",
            export.arrays, export.members, export.coordinate_name,
        ),
        NumpyKind::Archive => format!(
            "{} arrays of {} samples: '{}' plus one per signal, named. {dtype} values. Units, the \
             analysis domain and the producing version are not carried — an .npz member is an \
             array and nothing else.",
            export.arrays, export.rows, export.coordinate_name,
        ),
    };
    if let Some(member) = member {
        detail.push_str(&format!(
            " The file does not say which run it is: {member}."
        ));
    }
    detail
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
    use crate::workbench::workflows::result_import_workflow::parse_result_dataset;

    fn waveform(name: &str, x: Vec<f64>, y: Vec<f64>) -> RetainedTrace {
        RetainedTrace::new(name.to_owned(), x, y, "#4f81bd")
    }

    fn complex_waveform(
        name: &str,
        source_name: &str,
        x: Vec<f64>,
        display: Vec<f64>,
        real: Vec<f64>,
        imag: Vec<f64>,
    ) -> RetainedTrace {
        RetainedTrace::new(name.to_owned(), x, display, "#4f81bd").with_complex_components(
            source_name.to_owned(),
            real,
            imag,
        )
    }

    fn analysis(analysis_type: AnalysisType) -> AnalysisResult {
        AnalysisResult::new(1, analysis_type, "A")
    }

    /// The file `kind` writes of `waveforms`, and what it holds.
    fn written(
        kind: NumpyKind,
        analysis_type: AnalysisType,
        waveforms: &[RetainedTrace],
    ) -> Result<(Vec<u8>, NumpyFacts), String> {
        let analysis = analysis(analysis_type);
        let borrowed = waveforms.iter().collect::<Vec<_>>();
        let mut bytes = Vec::new();
        match kind {
            NumpyKind::Array => {
                write_npy(&analysis, &borrowed, exported_waveform_limits(), &mut bytes)
            }
            NumpyKind::Archive => write_npz(
                &[rspice_formats::waveform_io::result::ExportMember::new(
                    &analysis, borrowed,
                )],
                exported_waveform_limits(),
                &mut bytes,
            ),
        }
        .map(|facts| (bytes, facts))
        .map_err(|error| error.to_string())
    }

    #[test]
    fn a_real_array_puts_the_coordinate_first_and_says_so() {
        let (bytes, export) = written(
            NumpyKind::Array,
            AnalysisType::Transient,
            &[
                waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 12.0]),
                waveform("V(in)", vec![0.0, 1.0, 2.0], vec![20.0, 21.0, 22.0]),
            ],
        )
        .expect("a shared-axis transient writes");
        assert_eq!(column_order(&export), "column order = time, V(out), V(in)");

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
        let (bytes, _) = written(
            NumpyKind::Array,
            AnalysisType::Transient,
            &[
                waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 12.0]),
                waveform("V(in)", vec![0.0, 1.0, 2.0], vec![20.0, 21.0, 22.0]),
            ],
        )
        .expect("writes");
        let parsed = parse_result_dataset("waveforms.npy", &bytes).expect("re-imports");
        assert_eq!(parsed.sample_count, 3);
        assert_eq!(parsed.waveforms.len(), 2);
        assert_eq!(
            parsed.waveforms[0].x.read_whole().unwrap().as_ref(),
            [0.0, 1.0, 2.0]
        );
        assert_eq!(
            parsed.waveforms[0].y.read_whole().unwrap().as_ref(),
            [10.0, 11.0, 12.0]
        );
        assert_eq!(
            parsed.waveforms[1].y.read_whole().unwrap().as_ref(),
            [20.0, 21.0, 22.0]
        );
        // The names are gone, which is the property the export states rather
        // than hides: one array cannot carry them.
        assert_eq!(parsed.waveforms[0].name, "signal_1");
        assert_eq!(parsed.waveforms[1].name, "signal_2");
    }

    #[test]
    fn an_ac_array_is_complex128() {
        let (bytes, export) = written(
            NumpyKind::Array,
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
        .expect("writes");
        assert!(export.complex);
        let header = String::from_utf8_lossy(&bytes[10..128]);
        assert!(header.contains("'descr': '<c16'"), "{header}");
        assert!(header.contains("'shape': (3, 2, )"), "{header}");
        assert_eq!(bytes.len(), 128 + 6 * 16);
        assert_eq!(column_order(&export), "column order = frequency, V(out)");
        let reopened = parse_result_dataset("ac.npy", &bytes).expect("reopen a complex matrix");
        assert_eq!(reopened.sample_count, 3);
        assert_eq!(reopened.waveforms.len(), 1);
        assert_eq!(
            reopened.waveforms[0].x.read_whole().unwrap().as_ref(),
            &[1.0, 10.0, 100.0]
        );
        let complex = reopened.waveforms[0].complex.as_ref().unwrap();
        assert_eq!(
            complex.real.read_whole().unwrap().as_ref(),
            &[1.0, 0.5, 0.25]
        );
        assert_eq!(
            complex.imag.read_whole().unwrap().as_ref(),
            &[0.0, -0.5, -0.25]
        );
    }

    #[test]
    fn an_archive_names_every_array_and_reimports_with_those_names() {
        let (bytes, _) = written(
            NumpyKind::Archive,
            AnalysisType::Transient,
            &[
                waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 12.0]),
                waveform("I(R1)", vec![0.0, 1.0, 2.0], vec![20.0, 21.0, 22.0]),
            ],
        )
        .expect("writes");
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
        assert_eq!(out.x.read_whole().unwrap().as_ref(), [0.0, 1.0, 2.0]);
        assert_eq!(out.y.read_whole().unwrap().as_ref(), [10.0, 11.0, 12.0]);
    }

    #[test]
    fn an_ac_archive_reimports_its_complex_components() {
        let (bytes, _) = written(
            NumpyKind::Archive,
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
        .expect("writes");
        let parsed = parse_result_dataset("waveforms.npz", &bytes).expect("re-imports");
        assert_eq!(parsed.coordinate_name, "frequency");
        assert_eq!(parsed.analysis_type, AnalysisType::Ac);
        let complex = parsed.waveforms[0]
            .complex
            .as_ref()
            .expect("the complex components survive");
        assert_eq!(
            complex.real.read_whole().unwrap().as_ref(),
            [1.0, 0.5, 0.25]
        );
        assert_eq!(
            complex.imag.read_whole().unwrap().as_ref(),
            [0.0, -0.5, -0.25]
        );
    }

    #[test]
    fn the_same_dataset_publishes_the_same_bytes_twice() {
        let waveforms = [
            waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 12.0]),
            waveform("V(in)", vec![0.0, 1.0, 2.0], vec![20.0, 21.0, 22.0]),
        ];
        for kind in [NumpyKind::Array, NumpyKind::Archive] {
            assert_eq!(
                written(kind, AnalysisType::Transient, &waveforms).expect("writes"),
                written(kind, AnalysisType::Transient, &waveforms).expect("writes")
            );
        }
    }

    #[test]
    fn a_ragged_result_is_refused_rather_than_padded() {
        let error = written(
            NumpyKind::Array,
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
        let waveforms = [
            waveform("V(out)", vec![0.0, 1.0], vec![1.0, 2.0]),
            waveform("v(OUT)", vec![0.0, 1.0], vec![3.0, 4.0]),
        ];
        // One array each is fine; an archive has to name them.
        assert!(written(NumpyKind::Array, AnalysisType::Transient, &waveforms).is_ok());
        let error = written(NumpyKind::Archive, AnalysisType::Transient, &waveforms)
            .expect_err("the member names collide");
        assert!(error.to_string().contains("v(OUT)"), "{error}");
    }

    #[test]
    fn a_member_name_the_reader_refuses_is_refused_at_export() {
        let error = written(
            NumpyKind::Archive,
            AnalysisType::Transient,
            &[waveform("V(../escape)", vec![0.0, 1.0], vec![1.0, 2.0])],
        )
        .expect_err("'..' is refused by the archive reader");
        assert!(error.to_string().contains("'..'"), "{error}");
    }
}
