//! MATLAB v5 publication: the displayed analysis as named column vectors.
//!
//! The bytes are [`writer`]'s, which implements MathWorks' *MAT-File Format*
//! directly. The format projection also owns coordinate names, variable
//! naming and provenance. This module selects the displayed evidence,
//! publishes its bytes and presents the format's limitations.
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
//! Each retained complex signal is one MATLAB complex array in any analysis
//! domain. Real traces remain real, including an AC magnitude without retained
//! phase. MATLAB's complex class makes operations such as `abs(V_out)` work
//! directly on the original rectangular samples.
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

use rspice_formats::matlab as writer;

use super::job::ResultExportJob;
use super::members::{
    ExportFile, OwnedMembers, estimated_bytes, export_members, exported_members, publish_export,
};
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::workflows::export_workflow::ExportWorkflowIo;
use crate::workbench::workflows::result_import_workflow::exported_waveform_limits;
use writer::result::{MatlabFacts, check_matlab, write_matlab};

/// The media type the result-data contract states for `matlab-v5`.
const MATLAB_MIME_TYPE: &str = "application/x-matlab-data";

const LABEL: &str = "MATLAB v5";
const EXTENSION: &str = "mat";

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

pub(super) fn export_matlab(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
    signals: super::ExportedSignals,
) -> Option<ResultExportJob> {
    let reads = state.results_reads();
    let members = match exported_members(reads, displayed, signals) {
        Ok(members) => members,
        Err(message) => {
            let refusal = displayed
                .analyses(reads)
                .find(|analysis| analysis.waveforms.is_empty())
                .and_then(event_only_refusal);
            state.prompt(refusal.unwrap_or(message));
            return None;
        }
    };
    // A refused export never asks the reader for a destination it cannot fill.
    if let Err(error) = check_matlab(&export_members(&members), exported_waveform_limits()) {
        state.prompt(error.to_string());
        return None;
    }
    let bytes = estimated_bytes(&members, 8);
    let owned = OwnedMembers::of(&members);
    publish_export(
        state,
        io,
        ExportFile {
            label: LABEL,
            title: "Export MATLAB v5 File",
            default_name: format!("waveforms.{EXTENSION}"),
            filter_name: "MATLAB v5 File",
            extension: EXTENSION,
            mime_type: MATLAB_MIME_TYPE,
            text: false,
        },
        bytes,
        move |out| {
            write_matlab(&owned.members(), exported_waveform_limits(), out)
                .map(|facts| completion_detail(&facts))
                .map_err(|error| error.to_string())
        },
    )
}

/// What the reader is told they now have — including the two things the
/// format cannot keep, stated here because this is the only copy of them.
fn completion_detail(export: &MatlabFacts) -> String {
    let mut detail = if export.members > 1 {
        format!(
            "{} structures, one per member, named by its parameter values, each holding '{}' and \
             its signals as double column vectors. MAT v5 has no unit slot and accepts only \
             MATLAB identifiers as names, so both are stated here: {}.",
            export.members, export.coordinate, export.note
        )
    } else {
        let signals = export.variables - 1;
        format!(
            "'{}' and {signals} signals as double column vectors, {} samples each. MAT v5 has no \
             unit slot and accepts only MATLAB identifiers as names, so both are stated here: {}.",
            export.coordinate, export.rows, export.note
        )
    };
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
    use crate::state::{AnalysisResult, AnalysisResultPayload, AnalysisType, RetainedTrace};
    use crate::workbench::workflows::result_import_workflow::parse_result_dataset;
    use writer::publication::{ELLIPSIS, created_on, matlab_identifier};
    use writer::{
        HEADER_BYTES, HEADER_SIGNATURE, HEADER_TEXT_BYTES, MAX_NAME_CHARS, is_matlab_identifier,
    };

    fn waveform(name: &str, x: Vec<f64>, y: Vec<f64>) -> RetainedTrace {
        RetainedTrace::new(name.to_owned(), x, y, "#4f81bd")
    }

    /// A fixed timestamp: the header text is the dataset's creation time, so
    /// a test that pins bytes has to state one.
    fn analysis(analysis_type: AnalysisType) -> AnalysisResult {
        let mut analysis = AnalysisResult::new(1, analysis_type, "A");
        analysis.timestamp = 1_767_225_845.0;
        analysis
    }

    /// A written file and what it holds.
    #[derive(Debug)]
    struct Written {
        facts: MatlabFacts,
        bytes: Vec<u8>,
    }

    impl std::ops::Deref for Written {
        type Target = MatlabFacts;

        fn deref(&self) -> &MatlabFacts {
            &self.facts
        }
    }

    fn prepared(analysis_type: AnalysisType, waveforms: &[RetainedTrace]) -> Result<Written, String> {
        let analysis = analysis(analysis_type);
        let borrowed = waveforms.iter().collect::<Vec<_>>();
        let mut bytes = Vec::new();
        write_matlab(
            &[rspice_formats::waveform_io::result::ExportMember::new(&analysis, borrowed)],
            exported_waveform_limits(),
            &mut bytes,
        )
        .map(|facts| Written { facts, bytes })
        .map_err(|error| error.to_string())
    }

    fn encode_matlab(export: &Written) -> Result<Vec<u8>, String> {
        Ok(export.bytes.clone())
    }

    /// The names of the file's variables, as MATLAB reads them.
    fn variable_names(export: &Written) -> Vec<String> {
        matfile::MatFile::parse(std::io::Cursor::new(&export.bytes))
            .expect("MATLAB parses it")
            .arrays()
            .iter()
            .map(|array| array.name().to_owned())
            .collect()
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
        assert_eq!(
            parsed.waveforms[0].x.read_whole().unwrap().as_ref(),
            [0.0, 1.0, 2.0]
        );
        assert_eq!(
            parsed.waveforms[0].y.read_whole().unwrap().as_ref(),
            [10.0, 11.5, 12.25]
        );
        assert_eq!(parsed.waveforms[1].name, "I_R1_");
        assert_eq!(
            parsed.waveforms[1].y.read_whole().unwrap().as_ref(),
            [-2.0e-3, 0.0, 2.0e-3]
        );
    }

    #[test]
    fn an_ac_result_round_trips_its_complex_values() {
        let waveforms = [RetainedTrace::new(
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
    fn a_dc_sweep_publishes_the_coordinate_the_importer_reads_a_sweep_by() {
        let waveforms = [waveform("V(out)", vec![0.0, 0.5, 1.0], vec![0.0, 1.0, 2.0])];
        let export = prepared(AnalysisType::DcSweep, &waveforms).expect("prepares");
        assert_eq!(export.coordinate, "sweep");
        let bytes = encode_matlab(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.mat", &bytes).expect("re-imports");
        assert_eq!(parsed.analysis_type, AnalysisType::DcSweep);
        assert_eq!(parsed.coordinate_name, "sweep");
        assert_eq!(
            parsed.waveforms[0].y.read_whole().unwrap().as_ref(),
            [0.0, 1.0, 2.0]
        );
    }

    /// Every name this writer publishes a coordinate under is one the reader
    /// recognises as a coordinate.
    ///
    /// The reader's list was written out once per format and the writer held a
    /// fourth copy of it in prose. A coordinate published under a name the
    /// readers did not know would have come back as an anonymous signal on an
    /// invented sweep, and nothing would have failed until someone reopened
    /// the file — so the two sides now read the one constant, and this
    /// iterates it.
    #[test]
    fn every_coordinate_this_writer_publishes_is_one_the_importer_reads() {
        use crate::workbench::workflows::result_import_workflow::RESULT_COORDINATE_NAMES;

        assert_eq!(
            RESULT_COORDINATE_NAMES,
            ["time", "frequency", "freq", "sweep", "x"],
            "the shared list moved; every reader and this writer follow it"
        );
        let published = [
            AnalysisType::Transient,
            AnalysisType::DcSweep,
            AnalysisType::Ac,
        ]
        .map(|analysis| {
            prepared(analysis, &[waveform("V(out)", vec![0.0], vec![1.0])])
                .unwrap_or_else(|error| {
                    panic!("{analysis:?} is one of the three MAT carries: {error}")
                })
                .coordinate
        });
        for name in published {
            assert!(
                RESULT_COORDINATE_NAMES.contains(&name),
                "'{name}' is published but not read back as a coordinate"
            );
        }
        assert_eq!(published, ["time", "sweep", "frequency"]);
    }

    /// A one-point result publishes and reopens. See the HDF5 case: the
    /// importer's floor was two rows, so the three table writers published
    /// files this product then refused to read.
    #[test]
    fn a_single_sample_result_publishes_and_reopens() {
        let waveforms = [waveform("V(out)", vec![0.0], vec![2.5])];
        let export = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        assert_eq!(export.rows, 1);
        let bytes = encode_matlab(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.mat", &bytes).expect("re-imports");
        assert_eq!(parsed.sample_count, 1);
        assert_eq!(parsed.waveforms[0].x.read_whole().unwrap().as_ref(), [0.0]);
        assert_eq!(parsed.waveforms[0].y.read_whole().unwrap().as_ref(), [2.5]);
    }

    #[test]
    fn an_ac_trace_with_no_retained_phase_remains_real() {
        let waveforms = [waveform("V(out)", vec![1.0, 10.0], vec![1.0, 0.5])];
        let export = prepared(AnalysisType::Ac, &waveforms).expect("prepares");
        let bytes = encode_matlab(&export).expect("encodes");
        let parsed = parse_result_dataset("waveforms.mat", &bytes).expect("re-imports");
        assert!(parsed.waveforms[0].complex.is_none());
        assert_eq!(
            parsed.waveforms[0].y.read_whole().unwrap().as_ref(),
            [1.0, 0.5]
        );
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
            ("end", "xend"),
            ("function", "xfunction"),
            ("End", "End"),
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
        let names = variable_names(&export);
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
        assert_eq!(
            parsed.waveforms[4].y.read_whole().unwrap().as_ref(),
            [9.0, 10.0]
        );
    }

    #[test]
    fn a_suffixed_name_stays_within_namelengthmax() {
        let long = "V(".to_owned() + &"n".repeat(MAX_NAME_CHARS) + ")";
        let waveforms = [
            waveform(&long, vec![0.0, 1.0], vec![1.0, 2.0]),
            waveform(&long, vec![0.0, 1.0], vec![3.0, 4.0]),
        ];
        let export = prepared(AnalysisType::Transient, &waveforms).expect("prepares");
        let names = variable_names(&export);
        for name in &names {
            assert!(name.len() <= MAX_NAME_CHARS, "{name}");
            assert!(is_matlab_identifier(name), "{name}");
        }
        assert!(names[2].ends_with("_2"));
        assert_ne!(names[1], names[2]);
    }

    #[test]
    fn an_event_only_transient_is_refused_by_what_a_mat_file_carries() {
        let mut events = analysis(AnalysisType::Transient);
        events.result_payload = Some(AnalysisResultPayload::TransientEvents {
            voltage_impulses: None,
            current_impulses: None,
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
            hdl_sources: Vec::new(),
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
