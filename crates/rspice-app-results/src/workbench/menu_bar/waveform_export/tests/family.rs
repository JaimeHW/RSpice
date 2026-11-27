//! Every member of a `.step` family, written as each format holds a family: one long table with
//! a column per swept parameter, an array, structure or group per member named by its parameter
//! values, a plot per member, or every member in a bundle. A format that holds one analysis
//! writes the member the reader chose.

use super::*;

use rspice_results::provenance::{AnalysisResultFamilyMember, AnalysisResultStepCoordinate};
use rspice_results::result_import::{ResultImportFormat, ResultImportSource};

/// The swept resistance of each member, and the label each is named by.
const MEMBERS: [(f64, &str); 3] = [(1000.0, "r=1k"), (2000.0, "r=2k"), (3000.0, "r=3k")];

/// A run of the three members of a `.step` of `r`, each a transient whose V(out) ends at its
/// resistance in kilohms, all of them shown.
fn stepped_state() -> ResultsTestHost {
    let mut run = SimulationRun::new(7);
    for (index, (r, _)) in MEMBERS.into_iter().enumerate() {
        let member = AnalysisResultFamilyMember::StepPoint {
            index,
            coordinate: vec![AnalysisResultStepCoordinate {
                name: "r".to_owned(),
                value: r,
                unit: String::new(),
            }],
        };
        let mut analysis = AnalysisResult::new(
            index as u64 + 1,
            AnalysisType::Transient,
            member.titled("Transient"),
        )
        .with_waveforms(vec![waveform(
            "V(out)",
            vec![0.0, 1.0e-6, 2.0e-6],
            vec![0.0, 0.5e-3 * r, 1.0e-3 * r],
        )]);
        analysis.import_source = Some(ResultImportSource {
            source_name: "stepped.raw".to_owned(),
            format: ResultImportFormat::SpiceRaw,
            coordinate: None,
            family_member: Some(member),
            event_dump: None,
        });
        run.add_analysis(analysis);
    }
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![run].into();
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);
    state
}

/// What exporting the stepped view in `format` wrote, the member chosen when `analysis` names
/// one, and the prompt it left.
fn exported(format: usize, analysis: Option<usize>) -> (MockExportWorkflowIo, String) {
    let mut state = stepped_state();
    state
        .model
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, format)
        .expect("an export preference");
    let io = MockExportWorkflowIo::default();
    action_export_dataset_with_io(
        &mut state.results_parts(),
        &io,
        &DatasetExportChoice {
            analysis,
            signals: ExportedSignals::All,
            trace: None,
        },
    );
    let prompt = state.prompt_text().unwrap_or_default().to_owned();
    (io, prompt)
}

fn contains(bytes: &[u8], text: &str) -> bool {
    bytes
        .windows(text.len())
        .any(|window| window == text.as_bytes())
}

/// CSV and TSV write one long table of every member, with a column for the swept parameter
/// stating each member's value on each of its rows.
#[test]
fn a_table_of_every_member_states_each_members_parameter() {
    for (format, delimiter) in [(0, ','), (2, '\t')] {
        let (io, prompt) = exported(format, None);
        let files = io.text_files.borrow();
        assert_eq!(files.len(), 1, "{prompt}");
        let text = &files[0].1;
        let header = text.lines().next().expect("a header");
        let columns = header.split(delimiter).collect::<Vec<_>>();
        assert_eq!(
            columns,
            [
                "dataset_id",
                "analysis_sequence",
                "analysis_label",
                "analysis_type",
                "r",
                "trace",
                "component",
                "sample_index",
                "x",
                "y"
            ],
            "{text}"
        );
        for (r, label) in MEMBERS {
            let rows = text
                .lines()
                .skip(1)
                .filter(|line| line.split(delimiter).nth(4) == Some(&r.to_string()))
                .collect::<Vec<_>>();
            assert_eq!(rows.len(), 3, "{label}: {text}");
            assert!(rows.iter().all(|row| row.contains(label)), "{label}");
        }
    }
}

/// NumPy's archive, MATLAB and HDF5 hold an array, structure or group per member, named by its
/// parameter values; the rawfiles hold a plot per member and the bundles every member, each
/// read back with its parameter values.
#[test]
fn every_member_is_written_by_each_format_that_holds_a_family() {
    let (npz, prompt) = exported(7, None);
    let bytes = &npz.byte_files.borrow()[0].1;
    for (_, label) in MEMBERS {
        assert!(
            contains(bytes, &format!("{label}/V(out).npy")),
            "{label}: {prompt}"
        );
    }

    let (mat, prompt) = exported(9, None);
    let bytes = &mat.byte_files.borrow()[0].1;
    for (_, label) in MEMBERS {
        let structure = label.replace('=', "_");
        assert!(
            contains(bytes, &format!("{structure} = {label}")),
            "{label}: {prompt}"
        );
    }

    let (hdf5, prompt) = exported(8, None);
    let bytes = &hdf5.byte_files.borrow()[0].1;
    assert!(contains(bytes, "parameter_0000_name"), "{prompt}");
    for (_, label) in MEMBERS {
        assert!(contains(bytes, label), "{label}: {prompt}");
    }

    for (format, name) in [(10, "waveforms.raw"), (11, "waveforms.raw")] {
        let (raw, prompt) = exported(format, None);
        let bytes = &raw.byte_files.borrow()[0].1;
        let parsed =
            crate::workbench::workflows::result_import_workflow::parse_result_dataset(name, bytes)
                .unwrap_or_else(|error| panic!("{format}: {error} ({prompt})"));
        let plots = parsed.plots.as_ref().expect("a plot per member");
        assert_eq!(plots.further.len(), 2, "{format}");
        // Each plot's title names its member.
        for (_, label) in MEMBERS {
            assert!(
                contains(bytes, &format!("Title: Transient \u{00b7} {label}")),
                "{format}: {label}"
            );
        }
        assert_eq!(
            plots.further[1].waveforms[0].y.read_whole().unwrap().as_ref(),
            &[0.0, 1.5, 3.0]
        );
    }

    for (format, name) in [
        (3, "waveforms.rspiceresult"),
        (4, "waveforms.rspicedata"),
    ] {
        let (bundle, prompt) = exported(format, None);
        let bytes = &bundle.byte_files.borrow()[0].1;
        let parsed =
            crate::workbench::workflows::result_import_workflow::parse_result_dataset(name, bytes)
                .unwrap_or_else(|error| panic!("{format}: {error} ({prompt})"));
        let plots = parsed.plots.as_ref().expect("every member re-imports");
        let members = std::iter::once(plots.reviewed_member.clone())
            .chain(plots.further.iter().map(|plot| plot.family_member.clone()))
            .map(|member| member.map(|member| member.label()))
            .collect::<Vec<_>>();
        assert_eq!(
            members,
            MEMBERS.map(|(_, label)| Some(label.to_owned())),
            "{format}"
        );
        assert_eq!(
            plots.further[0].waveforms[0].y.read_whole().unwrap().as_ref(),
            &[0.0, 1.0, 2.0]
        );
    }
}

/// One NumPy array and a PWL source hold one analysis: of a family they write the member the
/// reader chose, and one array refuses every member at once rather than write one of them.
#[test]
fn a_one_analysis_format_writes_the_chosen_member() {
    let (npy, prompt) = exported(6, Some(1));
    let bytes = &npy.byte_files.borrow()[0].1;
    let parsed =
        crate::workbench::workflows::result_import_workflow::parse_result_dataset("m.npy", bytes)
            .unwrap_or_else(|error| panic!("{error} ({prompt})"));
    assert_eq!(
        parsed.waveforms[0].y.read_whole().unwrap().as_ref(),
        &[0.0, 1.0, 2.0]
    );

    let (pwl, prompt) = exported(12, Some(2));
    let text = String::from_utf8(pwl.byte_files.borrow()[0].1.clone()).expect("a text source");
    assert!(text.contains("r=3k"), "{prompt}: {text}");
    assert!(text.contains("+ 2e-6 3e0\n"), "{text}");

    let (npy, prompt) = exported(6, None);
    assert!(npy.byte_files.borrow().is_empty());
    assert!(prompt.contains("one"), "{prompt}");
}
