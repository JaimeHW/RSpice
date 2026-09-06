//! MATLAB v5 publication: the displayed analysis as named column vectors.
//!
//! The bytes are [`writer`]'s, which implements MathWorks' *MAT-File Format*
//! directly. This module decides what goes in them: which variable is the
//! coordinate, what each signal is called once MATLAB's naming rules have had
//! their say, and what the reader has to be told because the format has no
//! place to keep it.
//!
//! # What a `.mat` file carries
//!
//! One analysis, as one `double` column vector per column of it. The
//! coordinate is published under the name RSpice's own importer identifies a
//! coordinate by — `time` for a transient, `sweep` for a DC sweep,
//! `frequency` for an AC sweep — and every other variable is a signal. A
//! result whose domain is none of the three is refused by name: MAT has no
//! header that would say what a variable *is*, so a noise spectrum published
//! here would come back as an anonymous sweep.
//!
//! An AC signal is one complex array rather than two real ones, because
//! MATLAB has a complex numeric class and using it is what makes `abs(V_out)`
//! work in the reader's hands.
//!
//! # What it does not carry, and is told instead
//!
//! MAT v5 has **no unit slot** — a numeric array is a name, a shape and
//! numbers — and it accepts **only MATLAB identifiers** as names, so `V(out)`
//! cannot travel as itself. Inventing a `units` variable would put RSpice's
//! private convention in a file that claims to be a MATLAB file, and no
//! reader but this one would know to look for it.
//!
//! So neither is invented. Both are *stated*: one provenance note maps every
//! published variable back to the name the reader typed and to the unit it
//! was drawn in, and that note is written twice — verbatim into the export
//! confirmation, and into the header's 116-byte descriptive text, truncated
//! there with an ellipsis when it does not fit and said to be.

mod writer;

use std::collections::HashSet;

use super::{
    ALL_TRACES_HIDDEN_MESSAGE, NO_ACTIVE_ANALYSIS_MESSAGE, NO_SAMPLES_MESSAGE, exported_waveforms,
    note_result_export_failure, note_result_export_success,
};
use crate::workbench::app_state::AppState;
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
use writer::{HEADER_SIGNATURE, HEADER_TEXT_BYTES, MAX_NAME_CHARS, MatVariable, write_mat_v5};

/// `result_import_workflow::MAX_RESULT_COLUMNS`. An export above it is a file
/// this product refuses to read, so the ceiling is enforced here rather than
/// discovered on re-import.
const MAX_COLUMNS: usize = 1_024;

/// `result_import_workflow::MAX_RESULT_ROWS`, on the same argument.
const MAX_ROWS: usize = 1_000_000;

/// The media type the result-data contract states for `matlab-v5`.
const MATLAB_MIME_TYPE: &str = "application/x-matlab-data";

const LABEL: &str = "MATLAB v5";
const EXTENSION: &str = "mat";

/// What a header text says when it is cut to fit its field.
const ELLIPSIS: &str = "...";

/// One prepared file, and what the reader is owed about it.
#[derive(Debug)]
pub(super) struct MatlabExport {
    header_text: String,
    variables: Vec<MatVariable>,
    /// The variable the coordinate was published under.
    coordinate: &'static str,
    rows: usize,
    /// Every published variable, mapped back to what it came from.
    note: String,
    /// The note did not fit the header's descriptive-text field.
    note_truncated: bool,
    /// Signals published with a zero imaginary part because the displayed
    /// trace retained none.
    zeroed_imaginary: Vec<String>,
}

/// The variable name an analysis publishes its coordinate under, and whether
/// its signals are complex.
///
/// These are exactly the names `result_import_adapters::parse_matlab_v5`
/// recognises a coordinate by, so a file written here reopens as the analysis
/// it was.
const fn coordinate_variable(analysis: crate::state::AnalysisType) -> Option<(&'static str, bool)> {
    match analysis {
        crate::state::AnalysisType::Transient => Some(("time", false)),
        crate::state::AnalysisType::DcSweep => Some(("sweep", false)),
        crate::state::AnalysisType::Ac => Some(("frequency", true)),
        _ => None,
    }
}

/// A MATLAB identifier built from a name an engineer typed.
///
/// A letter first, then letters, digits and underscores, at most
/// `namelengthmax` characters. Everything else becomes an underscore, so
/// `V(out)` is `V_out_` and the shape of the original is still readable —
/// which matters, because the reader will see this name and not the other.
fn matlab_identifier(source: &str) -> String {
    let mut name = String::with_capacity(source.len());
    for character in source.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            name.push(character);
        } else {
            name.push('_');
        }
    }
    if !name.starts_with(|character: char| character.is_ascii_alphabetic()) {
        name.insert(0, 'x');
    }
    // Every character is ASCII by now, so this is a character count.
    name.truncate(MAX_NAME_CHARS);
    name
}

/// `candidate`, or the first `candidate_N` no other variable has taken.
///
/// Comparison is case-insensitive even though MATLAB's own is not: RSpice's
/// importer refuses two signals whose names differ only in case, and a file
/// this product writes has to be one it can read.
///
/// This terminates: suffixes of one digit width produce distinct names, there
/// are nine of width one and ninety of width two, and `taken` is finite.
fn unique_identifier(candidate: String, taken: &mut HashSet<String>) -> String {
    if taken.insert(candidate.to_ascii_lowercase()) {
        return candidate;
    }
    let mut suffix = 2_u32;
    loop {
        let tail = format!("_{suffix}");
        let stem = &candidate[..candidate.len().min(MAX_NAME_CHARS - tail.len())];
        let name = format!("{stem}{tail}");
        if taken.insert(name.to_ascii_lowercase()) {
            return name;
        }
        suffix += 1;
    }
}

/// The dataset's own creation time, as UTC.
///
/// It is the analysis timestamp rather than the wall clock, so exporting the
/// same result twice writes the same bytes. A timestamp no calendar can hold
/// is stated as unstated rather than guessed at.
fn created_on(timestamp: f64) -> String {
    let seconds = timestamp.trunc();
    if !seconds.is_finite() || seconds < i64::MIN as f64 || seconds > i64::MAX as f64 {
        return "unstated".to_owned();
    }
    // The range is checked immediately above, so this cast is exact.
    let Ok(stamp) = time::OffsetDateTime::from_unix_timestamp(seconds as i64) else {
        return "unstated".to_owned();
    };
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        stamp.year(),
        u8::from(stamp.month()),
        stamp.day(),
        stamp.hour(),
        stamp.minute(),
        stamp.second()
    )
}

/// One line of the provenance note: what a variable is, and in what.
fn note_entry(variable: &str, source: &str, unit: Option<&str>) -> String {
    let mut entry = variable.to_owned();
    if variable != source {
        entry.push_str(" = ");
        entry.push_str(source);
    }
    if let Some(unit) = unit {
        entry.push_str(" in ");
        entry.push_str(unit);
    }
    entry
}

/// The header's descriptive text, and whether the note had to be cut to fit.
///
/// The signature comes first because RSpice's own importer identifies a
/// `.mat` file by it, and MATLAB shows this field verbatim.
fn header_text(created_on: &str, note: &str) -> (String, bool) {
    let mut text =
        format!("{HEADER_SIGNATURE}, Platform: RSpice, Created on: {created_on}; {note}");
    if text.len() <= HEADER_TEXT_BYTES {
        return (text, false);
    }
    let mut kept = HEADER_TEXT_BYTES - ELLIPSIS.len();
    while !text.is_char_boundary(kept) {
        kept -= 1;
    }
    text.truncate(kept);
    text.push_str(ELLIPSIS);
    (text, true)
}

/// A transient whose retained evidence is an event schedule rather than a
/// table of samples.
fn event_only_refusal(analysis: &crate::state::AnalysisResult) -> Option<String> {
    matches!(
        analysis.result_payload,
        Some(crate::state::AnalysisResultPayload::TransientEvents { .. })
    )
    .then(|| {
        format!(
            "A {LABEL} file carries numeric column vectors standing on one coordinate. '{}' \
             retained an event schedule instead — the times the event solver accepted, which is \
             not a table and has no coordinate every node shares. Export VCD, which is the \
             format that holds an event history, or an RSpice bundle.",
            analysis.label
        )
    })
}

pub(super) fn prepare_matlab(
    analysis: &crate::state::AnalysisResult,
    waveforms: &[&crate::state::WaveformData],
) -> Result<MatlabExport, String> {
    let Some((coordinate_name, spectral)) = coordinate_variable(analysis.analysis_type) else {
        return Err(format!(
            "A {LABEL} file carries one sampled analysis: a transient on 'time', a DC sweep on \
             'sweep' or an AC sweep on 'frequency'. '{}' is none of those, and MAT has no header \
             in which a variable could say what it is, so the file would reopen as an anonymous \
             sweep. Export CSV, or an RSpice bundle, which carries this analysis whole.",
            analysis.label
        ));
    };
    let reference = waveforms
        .iter()
        .filter(|waveform| !waveform.x.is_empty())
        .max_by_key(|waveform| waveform.x.len())
        .ok_or_else(|| NO_SAMPLES_MESSAGE.to_owned())?;
    let coordinate = reference.x.as_ref().to_vec();
    if coordinate.len() > MAX_ROWS {
        return Err(format!(
            "This result has {} samples; RSpice reads at most {MAX_ROWS} from a MATLAB source, \
             so publishing it would produce a file this build could not reopen.",
            coordinate.len()
        ));
    }
    if waveforms.len() + 1 > MAX_COLUMNS {
        return Err(format!(
            "This result has {} columns; RSpice reads at most {MAX_COLUMNS} from a MATLAB \
             source. Hide traces, or export an RSpice bundle.",
            waveforms.len() + 1
        ));
    }

    // The coordinate claims its name first: it is written first, and the
    // importer takes the first variable whose name it recognises.
    let mut taken = HashSet::with_capacity(waveforms.len() + 1);
    taken.insert(coordinate_name.to_owned());
    let rows = coordinate.len();
    let mut variables = vec![MatVariable {
        name: coordinate_name.to_owned(),
        real: coordinate.clone(),
        imag: None,
    }];
    let coordinate_source = super::axis_signal_for_analysis_type(analysis.analysis_type).0;
    let mut entries = vec![note_entry(coordinate_name, &coordinate_source, None)];
    let mut zeroed_imaginary = Vec::new();
    for waveform in waveforms {
        // Every variable stands on the one coordinate, because that is what
        // the importer reads a MAT file back as. Publishing a column with its
        // own x-axis writes a file this product would refuse.
        if waveform.x.as_ref() != coordinate.as_slice() {
            return Err(format!(
                "A {LABEL} export is one table: every signal stands on the same coordinate \
                 samples. '{}' carries its own x-axis samples. Export this result as CSV or an \
                 RSpice bundle instead.",
                waveform.name
            ));
        }
        let (source, real, imag) = match (spectral, &waveform.complex) {
            (true, Some(complex)) => (
                complex.source_name.clone(),
                complex.real.as_ref().to_vec(),
                Some(complex.imag.as_ref().to_vec()),
            ),
            (true, None) => {
                zeroed_imaginary.push(waveform.name.clone());
                (
                    waveform.name.clone(),
                    waveform.y.as_ref().to_vec(),
                    Some(vec![0.0; rows]),
                )
            }
            (false, _) => (waveform.name.clone(), waveform.y.as_ref().to_vec(), None),
        };
        let name = unique_identifier(matlab_identifier(&source), &mut taken);
        entries.push(note_entry(&name, &source, waveform.unit.as_deref()));
        variables.push(MatVariable { name, real, imag });
    }
    if variables.len() == 1 {
        return Err(NO_SAMPLES_MESSAGE.to_owned());
    }

    // Every published variable is named, not just the first few: the unit is
    // recoverable from nowhere else, so a note that covered some of them
    // would be a note that quietly lost the rest.
    let note = entries.join("; ");
    let (header_text, note_truncated) = header_text(&created_on(analysis.timestamp), &note);
    Ok(MatlabExport {
        header_text,
        variables,
        coordinate: coordinate_name,
        rows,
        note,
        note_truncated,
        zeroed_imaginary,
    })
}

/// The bytes of a prepared file.
pub(super) fn encode_matlab(export: &MatlabExport) -> Result<Vec<u8>, String> {
    write_mat_v5(&export.header_text, &export.variables)
        .map_err(|error| format!("the MATLAB v5 file could not be written: {error}"))
}

pub(super) fn export_matlab(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
) {
    // A `.mat` file holds one analysis's variables in one flat namespace, so
    // a stack of displayed strips has nowhere to go and is refused before a
    // picker opens.
    if displayed.analysis_indices.len() > 1 {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
            "A {LABEL} file names its variables in one flat namespace, and this view shows \
             several analyses. Maximize one displayed strip, or export an RSpice bundle, which \
             carries them all."
        )));
        return;
    }
    let prepared = match displayed.primary_analysis(state) {
        Some(analysis) => {
            let waveforms = exported_waveforms(state, displayed.dataset_id, analysis);
            if waveforms.is_empty() {
                Err(event_only_refusal(analysis).unwrap_or_else(|| {
                    if analysis.waveforms.is_empty() {
                        NO_SAMPLES_MESSAGE.to_owned()
                    } else {
                        ALL_TRACES_HIDDEN_MESSAGE.to_owned()
                    }
                }))
            } else {
                prepare_matlab(analysis, &waveforms)
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
    let bytes = match encode_matlab(&prepared) {
        Ok(bytes) => bytes,
        Err(error) => {
            note_result_export_failure(state, format!("{LABEL} encoding failed: {error}"));
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "{LABEL} export failed before destination selection: {error}"
            )));
            return;
        }
    };
    if !prepared.zeroed_imaginary.is_empty() {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
            "An AC signal is published as a complex array. {} retained no imaginary part and \
             were written with a zero one: {}.",
            prepared.zeroed_imaginary.len(),
            prepared.zeroed_imaginary.join(", ")
        )));
    }

    let default_name = format!("waveforms.{EXTENSION}");
    let (published_path, export) = match io.show_save_dialog(SaveDialogConfig {
        title: "Export MATLAB v5 File",
        default_name: &default_name,
        filter_name: "MATLAB v5 File",
        filter_extensions: &[EXTENSION],
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, EXTENSION);
            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_bytes_file_observed(&destination, &bytes, MATLAB_MIME_TYPE)
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

/// What the reader is told they now have — including the two things the
/// format cannot keep, stated here because this is the only copy of them.
fn completion_detail(export: &MatlabExport) -> String {
    let signals = export.variables.len() - 1;
    let mut detail = format!(
        "'{}' and {signals} signals as double column vectors, {} samples each. MAT v5 has no \
         unit slot and accepts only MATLAB identifiers as names, so both are stated here: {}.",
        export.coordinate, export.rows, export.note
    );
    detail.push_str(if export.note_truncated {
        " The file's header text carries as much of that note as its 116 characters hold."
    } else {
        " The file's header text carries the same note."
    });
    detail
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisResultPayload, AnalysisType, WaveformData};
    use crate::workbench::workflows::result_import_workflow::parse_result_dataset;
    use writer::{HEADER_BYTES, is_matlab_identifier};

    fn waveform(name: &str, x: Vec<f64>, y: Vec<f64>) -> WaveformData {
        WaveformData::new(name.to_owned(), x, y, "#4f81bd")
    }

    /// A fixed timestamp: the header text is the dataset's creation time, so
    /// a test that pins bytes has to state one.
    fn analysis(analysis_type: AnalysisType) -> AnalysisResult {
        let mut analysis = AnalysisResult::new(1, analysis_type, "A");
        analysis.timestamp = 1_767_225_845.0;
        analysis
    }

    fn prepared(
        analysis_type: AnalysisType,
        waveforms: &[WaveformData],
    ) -> Result<MatlabExport, String> {
        let analysis = analysis(analysis_type);
        let borrowed = waveforms.iter().collect::<Vec<_>>();
        prepare_matlab(&analysis, &borrowed)
    }

    #[test]
    fn a_transient_round_trips_its_sanitised_names_and_exact_values() {
        let waveforms = [
            waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.5, 12.25]).with_unit("V"),
            waveform("I(R1)", vec![0.0, 1.0, 2.0], vec![-2.0e-3, 0.0, 2.0e-3]).with_unit("A"),
        ];
        let export = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        let bytes = encode_matlab(&export).expect("encodes");

        // The oracle first: the crate that reads MAT files, not RSpice's
        // adapter on top of it.
        let file = matfile::MatFile::parse(std::io::Cursor::new(&bytes)).expect("MATLAB parses it");
        let names = file
            .arrays()
            .iter()
            .map(matfile::Array::name)
            .collect::<Vec<_>>();
        assert_eq!(names, ["time", "V_out_", "I_R1_"]);
        for array in file.arrays() {
            assert_eq!(array.size(), &vec![3, 1], "{}", array.name());
        }
        let matfile::NumericData::Double { real, imag } = file
            .find_by_name("I_R1_")
            .expect("the second signal")
            .data()
        else {
            panic!("a double array");
        };
        assert_eq!(real.as_slice(), [-2.0e-3, 0.0, 2.0e-3]);
        assert_eq!(*imag, None);

        let parsed = parse_result_dataset("waveforms.mat", &bytes).expect("re-imports");
        assert_eq!(parsed.analysis_type, AnalysisType::Transient);
        assert_eq!(parsed.coordinate_name, "time");
        assert_eq!(parsed.sample_count, 3);
        assert_eq!(parsed.waveforms.len(), 2);
        assert_eq!(parsed.waveforms[0].name, "V_out_");
        assert_eq!(parsed.waveforms[0].x.as_slice(), [0.0, 1.0, 2.0]);
        assert_eq!(parsed.waveforms[0].y.as_slice(), [10.0, 11.5, 12.25]);
        assert_eq!(parsed.waveforms[1].name, "I_R1_");
        assert_eq!(parsed.waveforms[1].y.as_slice(), [-2.0e-3, 0.0, 2.0e-3]);
    }

    #[test]
    fn an_ac_result_round_trips_its_complex_values() {
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
        assert!(export.zeroed_imaginary.is_empty());
        let bytes = encode_matlab(&export).expect("encodes");

        let file = matfile::MatFile::parse(std::io::Cursor::new(&bytes)).expect("MATLAB parses it");
        let matfile::NumericData::Double { real, imag } =
            file.find_by_name("V_out_").expect("the signal").data()
        else {
            panic!("a double array");
        };
        assert_eq!(real.as_slice(), [1.0, 0.5, 0.25]);
        assert_eq!(
            imag.as_ref().expect("a complex array").as_slice(),
            [0.0, -0.5, -0.25]
        );

        let parsed = parse_result_dataset("waveforms.mat", &bytes).expect("re-imports");
        assert_eq!(parsed.analysis_type, AnalysisType::Ac);
        assert_eq!(parsed.coordinate_name, "frequency");
        // The importer names a complex trace by the magnitude it draws and
        // keeps the published name on the components it drew it from.
        assert_eq!(parsed.waveforms[0].name, "|V_out_|");
        let complex = parsed.waveforms[0]
            .complex
            .as_ref()
            .expect("the complex components survive");
        assert_eq!(complex.source_name, "V_out_");
        assert_eq!(complex.real.as_slice(), [1.0, 0.5, 0.25]);
        assert_eq!(complex.imag.as_slice(), [0.0, -0.5, -0.25]);
    }

    #[test]
    fn a_dc_sweep_publishes_the_coordinate_the_importer_reads_a_sweep_by() {
        let waveforms = [waveform("V(out)", vec![0.0, 0.5, 1.0], vec![0.0, 1.0, 2.0])];
        let export = prepared(AnalysisType::DcSweep, &waveforms).expect("prepares");
        assert_eq!(export.coordinate, "sweep");
        let bytes = encode_matlab(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.mat", &bytes).expect("re-imports");
        assert_eq!(parsed.analysis_type, AnalysisType::DcSweep);
        assert_eq!(parsed.coordinate_name, "sweep");
        assert_eq!(parsed.waveforms[0].y.as_slice(), [0.0, 1.0, 2.0]);
    }

    #[test]
    fn an_ac_trace_with_no_retained_phase_is_published_with_zero_and_named() {
        let waveforms = [waveform("V(out)", vec![1.0, 10.0], vec![1.0, 0.5])];
        let export = prepared(AnalysisType::Ac, &waveforms).expect("prepares");
        assert_eq!(export.zeroed_imaginary, ["V(out)"]);
        let bytes = encode_matlab(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.mat", &bytes).expect("re-imports");
        let complex = parsed.waveforms[0].complex.as_ref().expect("a pair");
        assert_eq!(complex.imag.as_slice(), [0.0, 0.0]);
    }

    #[test]
    fn the_same_dataset_publishes_the_same_bytes_twice() {
        let waveforms = [waveform("V(out)", vec![0.0, 1.0, 2.0], vec![1.0, 2.0, 3.0])];
        let first = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        let second = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        assert_eq!(
            encode_matlab(&first).expect("encodes"),
            encode_matlab(&second).expect("encodes")
        );
    }

    #[test]
    fn the_header_states_the_creation_time_of_the_dataset_not_the_wall_clock() {
        // The fixed timestamp the helper sets.
        assert_eq!(created_on(1_767_225_845.0), "2026-01-01T00:04:05Z");
        assert_eq!(created_on(0.0), "1970-01-01T00:00:00Z");
        // A timestamp no calendar holds is stated as unstated, not guessed.
        assert_eq!(created_on(f64::NAN), "unstated");
        assert_eq!(created_on(f64::INFINITY), "unstated");
        assert_eq!(created_on(1.0e30), "unstated");

        let waveforms = [waveform("V(out)", vec![0.0, 1.0], vec![1.0, 2.0])];
        let export = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        assert!(
            export.header_text.starts_with(
                "MATLAB 5.0 MAT-file, Platform: RSpice, Created on: 2026-01-01T00:04:05Z"
            ),
            "{}",
            export.header_text
        );
    }

    #[test]
    fn the_first_128_bytes_are_the_header_the_specification_lays_out() {
        let waveforms = [waveform("V(out)", vec![0.0, 1.0], vec![1.0, 2.0])];
        let export = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        let bytes = encode_matlab(&export).expect("encodes");
        assert!(bytes.starts_with(HEADER_SIGNATURE.as_bytes()));
        assert_eq!(bytes[HEADER_TEXT_BYTES..HEADER_BYTES - 4], [0_u8; 8]);
        assert_eq!(bytes[HEADER_BYTES - 4..HEADER_BYTES - 2], [0x00, 0x01]);
        assert_eq!(&bytes[HEADER_BYTES - 2..HEADER_BYTES], b"IM");
        // The whole text field is written: the note, then spaces.
        assert_eq!(
            std::str::from_utf8(&bytes[..HEADER_TEXT_BYTES])
                .expect("the header text is text")
                .trim_end(),
            export.header_text
        );
    }

    #[test]
    fn the_note_states_every_variable_its_source_name_and_its_unit() {
        let waveforms = [
            waveform("V(out)", vec![0.0, 1.0], vec![1.0, 2.0]).with_unit("V"),
            waveform("I(R1)", vec![0.0, 1.0], vec![3.0, 4.0]).with_unit("A"),
            waveform("ratio", vec![0.0, 1.0], vec![5.0, 6.0]),
        ];
        let export = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        assert_eq!(
            export.note,
            "time; V_out_ = V(out) in V; I_R1_ = I(R1) in A; ratio"
        );
        // The confirmation carries the note verbatim.
        let detail = completion_detail(&export);
        assert!(detail.contains(&export.note), "{detail}");
        assert!(detail.contains("no unit slot"), "{detail}");
    }

    #[test]
    fn a_note_longer_than_the_header_field_is_cut_with_an_ellipsis_and_said_to_be() {
        let waveforms = (0..8)
            .map(|index| {
                waveform(
                    &format!("V(node_number_{index})"),
                    vec![0.0, 1.0],
                    vec![1.0, 2.0],
                )
                .with_unit("V")
            })
            .collect::<Vec<_>>();
        let export = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        assert!(export.note_truncated);
        assert_eq!(export.header_text.len(), HEADER_TEXT_BYTES);
        assert!(
            export.header_text.ends_with(ELLIPSIS),
            "{}",
            export.header_text
        );
        // The whole note survives in the confirmation, which is the copy the
        // reader is meant to keep.
        let detail = completion_detail(&export);
        assert!(detail.contains(&export.note), "{detail}");
        assert!(detail.contains("as its 116 characters hold"), "{detail}");
    }

    #[test]
    fn every_name_an_engineer_can_type_becomes_a_matlab_identifier() {
        for (source, expected) in [
            ("V(out)", "V_out_"),
            ("I(R1)", "I_R1_"),
            ("1st", "x1st"),
            ("_leading", "x_leading"),
            ("with space", "with_space"),
            ("v.out", "v_out"),
            ("x[3]", "x_3_"),
            ("V(\u{b5})", "V___"),
            ("", "x"),
            ("###", "x___"),
            ("already_fine9", "already_fine9"),
        ] {
            let name = matlab_identifier(source);
            assert_eq!(name, expected, "{source}");
            assert!(is_matlab_identifier(&name), "{source} -> {name}");
        }
        // namelengthmax, counted in characters.
        let long = matlab_identifier(&"v".repeat(MAX_NAME_CHARS + 10));
        assert_eq!(long.len(), MAX_NAME_CHARS);
        assert!(is_matlab_identifier(&long));
        // A prefixed name is still cut to namelengthmax.
        let prefixed = matlab_identifier(&"9".repeat(MAX_NAME_CHARS + 10));
        assert_eq!(prefixed.len(), MAX_NAME_CHARS);
        assert!(prefixed.starts_with("x9"));
    }

    #[test]
    fn names_that_sanitise_alike_are_suffixed_rather_than_collided() {
        let waveforms = [
            waveform("V(out)", vec![0.0, 1.0], vec![1.0, 2.0]),
            waveform("V[out]", vec![0.0, 1.0], vec![3.0, 4.0]),
            waveform("V out ", vec![0.0, 1.0], vec![5.0, 6.0]),
            // Case-only differences collide for RSpice's own importer, so
            // they are suffixed here too.
            waveform("v_OUT_", vec![0.0, 1.0], vec![7.0, 8.0]),
            // A signal that sanitises onto the coordinate's own name.
            waveform("time", vec![0.0, 1.0], vec![9.0, 10.0]),
        ];
        let export = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        let names = export
            .variables
            .iter()
            .map(|variable| variable.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            [
                "time", "V_out_", "V_out__2", "V_out__3", "v_OUT__4", "time_2"
            ]
        );
        let bytes = encode_matlab(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.mat", &bytes).expect("re-imports");
        // The coordinate is the first variable, so the suffixed `time_2` is
        // read back as an ordinary signal.
        assert_eq!(parsed.coordinate_name, "time");
        assert_eq!(parsed.waveforms.len(), 5);
        assert_eq!(parsed.waveforms[4].name, "time_2");
        assert_eq!(parsed.waveforms[4].y.as_slice(), [9.0, 10.0]);
    }

    #[test]
    fn a_suffixed_name_stays_within_namelengthmax() {
        let long = "V(".to_owned() + &"n".repeat(MAX_NAME_CHARS) + ")";
        let waveforms = [
            waveform(&long, vec![0.0, 1.0], vec![1.0, 2.0]),
            waveform(&long, vec![0.0, 1.0], vec![3.0, 4.0]),
        ];
        let export = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        for variable in &export.variables {
            assert!(variable.name.len() <= MAX_NAME_CHARS, "{}", variable.name);
            assert!(is_matlab_identifier(&variable.name), "{}", variable.name);
        }
        assert!(export.variables[2].name.ends_with("_2"));
        assert_ne!(export.variables[1].name, export.variables[2].name);
        encode_matlab(&export).expect("the writer accepts every name");
    }

    #[test]
    fn a_domain_the_layout_has_no_coordinate_for_is_refused_by_name() {
        let waveforms = [waveform("V(onoise)", vec![1.0, 10.0], vec![1e-9, 2e-9])];
        let error = prepared(AnalysisType::Noise, &waveforms)
            .expect_err("MAT names its coordinate or says nothing");
        assert!(error.contains("'time'"), "{error}");
        assert!(error.contains("'sweep'"), "{error}");
        assert!(error.contains("'frequency'"), "{error}");
        assert!(error.contains("RSpice bundle"), "{error}");
    }

    #[test]
    fn an_event_only_transient_is_refused_by_what_a_mat_file_carries() {
        let mut events = analysis(AnalysisType::Transient);
        events.result_payload = Some(AnalysisResultPayload::TransientEvents {
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
        });
        let refusal = event_only_refusal(&events).expect("an event schedule is not a table");
        assert!(refusal.contains("event schedule"), "{refusal}");
        assert!(refusal.contains("VCD"), "{refusal}");
        // A transient that retained samples is not refused for this reason.
        assert!(event_only_refusal(&analysis(AnalysisType::Transient)).is_none());
    }

    #[test]
    fn a_ragged_result_is_refused_rather_than_padded() {
        let waveforms = [
            waveform("V(out)", vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 12.0]),
            waveform("V(in)", vec![0.0, 1.0], vec![20.0, 21.0]),
        ];
        let error =
            prepared(AnalysisType::Transient, &waveforms).expect_err("a MAT export is one table");
        assert!(error.contains("'V(in)'"), "{error}");
        assert!(error.contains("same coordinate samples"), "{error}");
    }
}
