//! The streamed import against the whole-file import of the same rawfile.

use super::*;

/// A one-plot binary rawfile of doubles, written to `path`.
fn write_rawfile(
    path: &Path,
    plot: &str,
    flags: &str,
    variables: &[(&str, &str)],
    rows: &[Vec<f64>],
) {
    let mut bytes = format!(
        "Title: streamed\nDate: today\nPlotname: {plot}\nFlags: {flags}\nNo. Variables: {}\nNo. Points: {}\nVariables:\n",
        variables.len(),
        rows.len()
    )
    .into_bytes();
    for (index, (name, kind)) in variables.iter().enumerate() {
        bytes.extend(format!("\t{index}\t{name}\t{kind}\n").bytes());
    }
    bytes.extend(b"Binary:\n");
    for row in rows {
        for value in row {
            bytes.extend(value.to_le_bytes());
        }
    }
    std::fs::write(path, bytes).unwrap();
}

/// The store each series of `parsed` reads, by digest and column, and the window it reads.
fn storage(parsed: &ParsedResultDataset) -> Vec<(Option<([u8; 32], u64)>, StoredWindow)> {
    parsed
        .waveforms
        .iter()
        .flat_map(|trace| {
            let complex = trace
                .complex
                .iter()
                .flat_map(|complex| [&complex.real, &complex.imag]);
            [&trace.x, &trace.y].into_iter().chain(complex)
        })
        .map(|values| {
            let (source, window) = values.stored_window().expect("a stored series");
            (source.identity(), window)
        })
        .collect()
}

/// A folder of its own for one test's files, removed when the test ends.
struct Folder(std::path::PathBuf);

impl Folder {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("rspice-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A transient, an AC plot of complex values and a DC sweep run downwards each stream into the
/// very segment the whole-file import keeps for the same file, with the same traces, names,
/// units and review facts: the streamed import is the whole-file import, holding one block.
#[test]
fn a_binary_rawfile_streams_into_the_segment_the_whole_file_import_keeps() {
    let folder = Folder::new("streamed-raw-import");
    let transient = (0..5_000)
        .map(|row| {
            let time = row as f64 * 1.0e-9;
            vec![time, (time * 1.0e8).sin(), -(time * 1.0e8).cos()]
        })
        .collect::<Vec<_>>();
    let ac = (1..200)
        .map(|row| {
            let frequency = row as f64 * 1.0e3;
            vec![frequency, 0.0, 1.0 / row as f64, -(row as f64).sqrt()]
        })
        .collect::<Vec<_>>();
    let sweep = (0..50)
        .map(|row| vec![5.0 - row as f64 * 0.1, row as f64 * 1.0e-3])
        .collect::<Vec<_>>();
    let cases: [(&str, &str, &str, &[(&str, &str)], &[Vec<f64>]); 3] = [
        (
            "transient.raw",
            "Transient Analysis",
            "real double",
            &[
                ("time", "time"),
                ("V(out)", "voltage"),
                ("I(R1)", "device_current"),
            ],
            &transient,
        ),
        (
            "ac.raw",
            "AC Analysis",
            "complex double",
            &[("frequency", "frequency"), ("V(out)", "voltage")],
            &ac,
        ),
        (
            "sweep.raw",
            "DC transfer characteristic",
            "real double",
            &[("v-sweep", "voltage"), ("I(V1)", "current")],
            &sweep,
        ),
    ];
    for (name, plot, flags, variables, rows) in cases {
        let path = folder.0.join(name);
        write_rawfile(&path, plot, flags, variables, rows);
        let job = JobContext::detached();
        let streamed = stream_spice_raw(&job, &path, name)
            .unwrap_or_else(|_| panic!("{name} streams"))
            .unwrap_or_else(|| panic!("{name} is read as it arrives"));
        let size = std::fs::metadata(&path).unwrap().len();
        assert_eq!(job.reported(), ("Reading".to_owned(), size, size), "{name}");
        let bytes = std::fs::read(&path).unwrap();
        let whole = stage_source(&job, name, &bytes, None)
            .unwrap_or_else(|_| panic!("{name} imports whole"));
        let (streamed, whole) = (&streamed.parsed, &whole.parsed);
        assert_eq!(streamed.analysis_type, whole.analysis_type, "{name}");
        assert_eq!(streamed.coordinate_name, whole.coordinate_name, "{name}");
        assert_eq!(streamed.coordinate_unit, whole.coordinate_unit, "{name}");
        assert_eq!(streamed.sample_count, whole.sample_count, "{name}");
        assert_eq!(streamed.notes, whole.notes, "{name}");
        let traces = |parsed: &ParsedResultDataset| {
            parsed
                .waveforms
                .iter()
                .map(|trace| {
                    (
                        trace.name.clone(),
                        trace.unit.clone(),
                        trace
                            .complex
                            .as_ref()
                            .map(|complex| complex.source_name.clone()),
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(traces(streamed), traces(whole), "{name}");
        assert_eq!(storage(streamed), storage(whole), "{name}");
        assert!(
            storage(streamed)
                .iter()
                .all(|(identity, _)| identity.is_some())
        );
    }
}

/// The review facts gathered as the rows pass are those read from the kept samples.
#[test]
fn the_review_facts_are_gathered_as_the_rows_pass() {
    let folder = Folder::new("streamed-raw-facts");
    let rows = (1..30)
        .map(|row| vec![row as f64, 0.0, if row == 3 { f64::NAN } else { 0.5 }, 1.0])
        .collect::<Vec<_>>();
    let path = folder.0.join("facts.raw");
    write_rawfile(
        &path,
        "AC Analysis",
        "complex double",
        &[("frequency", "frequency"), ("V(out)", "voltage")],
        &rows,
    );
    // A NaN in a binary rawfile is refused by the reader; a finite file gives the facts.
    let job = JobContext::detached();
    let refused = stream_spice_raw(&job, &path, "facts.raw").err();
    assert!(
        matches!(&refused, Some(JobStop::Failed(reason)) if reason.contains("Non-finite binary raw value")),
        "{refused:?}"
    );
    let rows = (1..30)
        .map(|row| vec![row as f64, 0.0, 0.5 * row as f64, -1.0])
        .collect::<Vec<_>>();
    write_rawfile(
        &path,
        "AC Analysis",
        "complex double",
        &[("frequency", "frequency"), ("V(out)", "voltage")],
        &rows,
    );
    let streamed = stream_spice_raw(&job, &path, "facts.raw").unwrap().unwrap();
    assert_eq!(
        streamed.facts,
        ResultImportSampleFacts::read(&streamed.parsed.waveforms)
    );
    assert_eq!(streamed.facts.frequency_grid, vec![true]);
}

/// A rawfile the streamed reader does not read, and a source that is no rawfile, are left to the
/// whole-file import. (Cancel during a streamed import is the generated-rawfile import test's.)
#[test]
fn other_sources_are_read_whole() {
    let folder = Folder::new("streamed-raw-whole");
    let rows = (0..10).map(|row| vec![row as f64, 1.0]).collect::<Vec<_>>();
    let path = folder.0.join("stepped.raw");
    write_rawfile(
        &path,
        "Transient Analysis",
        "real double stepped",
        &[("time", "time"), ("V(out)", "voltage")],
        &rows,
    );
    let job = JobContext::detached();
    assert!(
        stream_spice_raw(&job, &path, "stepped.raw")
            .unwrap()
            .is_none()
    );
    let csv = folder.0.join("table.csv");
    std::fs::write(&csv, "time,V(out)\n0,1\n1,2\n").unwrap();
    assert!(stream_spice_raw(&job, &csv, "table.csv").unwrap().is_none());
}
