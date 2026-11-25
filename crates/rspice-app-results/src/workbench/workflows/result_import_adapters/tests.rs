//! Cross-format result-import adapter tests and binary fixture builders.

use super::*;
use crate::workbench::documents::result_document::test_fixtures::generated_fst;
use arrow_array::{ArrayRef, Float64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use std::collections::HashMap;
use std::io::{Cursor, Write as _};
use std::sync::Arc;

fn assert_basic(parsed: ParsedResultDataset, format: ResultImportFormat) {
    assert_eq!(parsed.source_format, format);
    assert!(parsed.sample_count >= 2);
    assert!(!parsed.waveforms.is_empty());
    assert!(parsed.waveforms.iter().all(|waveform| {
        waveform.x.len() == parsed.sample_count && waveform.y.len() == parsed.sample_count
    }));
}

fn zip_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for (name, contents) in entries {
        writer.start_file(*name, options).expect("start ZIP member");
        writer.write_all(contents).expect("write ZIP member");
    }
    writer.finish().expect("finish ZIP").into_inner()
}

fn native_bundle(format: ResultImportFormat) -> Vec<u8> {
    let dataset = br#"{"schema":"rspice-waveform-dataset/1","analysis":"ac","coordinate":{"name":"frequency","values":[1.0,2.0]},"signals":[{"name":"V(out)","real":[1.0,2.0],"imag":[0.5,-0.5]}]}"#;
    use sha2::Digest as _;
    let schema = if format == ResultImportFormat::RSpiceResultBundle {
        "rspice-result-bundle/1"
    } else {
        "rspice-dataset-bundle/1"
    };
    let manifest = format!(
        "{{\"schema\":\"{schema}\",\"dataset_member\":\"dataset.json\",\"dataset_sha256\":\"{:x}\"}}",
        sha2::Sha256::digest(dataset)
    );
    zip_bytes(&[
        ("manifest.json", manifest.as_bytes()),
        ("dataset.json", dataset),
    ])
}

/// A bundle of one member of a `.step` sweep imports as that member, titled the way every route
/// titles one, so a member exported and read back is still that member of its family.
#[test]
fn a_member_bundle_imports_as_that_member() {
    use rspice_formats::native_bundle::{
        NativeBundleDataset, NativeBundleKind, NativeBundleMember, NativeBundleSignal,
        NativeBundleSignalValues, NativeBundleStepValue, encode_native_bundle,
    };
    let member = NativeBundleMember::StepPoint {
        index: 1,
        coordinate: vec![NativeBundleStepValue {
            name: "r".to_owned(),
            value: 2000.0,
            unit: String::new(),
        }],
    };
    for (kind, format) in [
        (
            NativeBundleKind::Result,
            ResultImportFormat::RSpiceResultBundle,
        ),
        (
            NativeBundleKind::Dataset,
            ResultImportFormat::RSpiceDatasetBundle,
        ),
    ] {
        let bytes = encode_native_bundle(
            kind,
            &NativeBundleDataset {
                analysis: rspice_formats::WaveformDomain::Transient,
                coordinate_name: "time",
                coordinate_unit: None,
                coordinate: vec![0.0, 1.0e-6].into(),
                signals: vec![NativeBundleSignal {
                    name: "V(out)",
                    unit: None,
                    values: NativeBundleSignalValues::Real(vec![0.5, 1.5].into()),
                }],
                member: Some(member.clone()),
            },
            1_000_000,
        )
        .expect("a member bundle");
        let parsed = parse_native_bundle(&bytes, format).expect("a member bundle reads");
        let kept = keep_reviewed_draft("member.bundle", parsed).expect("a member bundle imports");
        let analysis = &kept.analyses[0];
        assert_eq!(
            analysis.family_member(),
            Some(&member.clone().into_family_member()),
            "{format:?}"
        );
        assert_eq!(analysis.label, "Imported \u{00b7} Transient \u{00b7} r=2k");
    }
}

/// A size no small test source can hold: a declaration of it is damage, as a bomb's is.
const DECLARED_PAST_THE_SOURCE: u64 = 64 * 1024 * 1024;

#[test]
fn native_result_and_dataset_bundles_verify_digest_and_complex_samples() {
    for format in [
        ResultImportFormat::RSpiceResultBundle,
        ResultImportFormat::RSpiceDatasetBundle,
    ] {
        let parsed = parse_native_bundle(&native_bundle(format), format).expect("native bundle");
        assert_basic(parsed, format);
    }
    let mut bad = native_bundle(ResultImportFormat::RSpiceDatasetBundle);
    let last = bad.len() - 1;
    bad[last] ^= 0x01;
    assert!(parse_native_bundle(&bad, ResultImportFormat::RSpiceDatasetBundle).is_err());
}

fn generic_hdf5() -> Vec<u8> {
    let mut builder = rustyhdf5::FileBuilder::new();
    builder
        .create_dataset("time")
        .with_f64_data(&[0.0, 1e-9, 2e-9]);
    builder
        .create_dataset("V(out)")
        .with_f64_data(&[0.0, 1.0, 0.0]);
    builder.finish().expect("HDF5 fixture")
}

#[test]
fn hdf5_signed_zero_coordinates_must_be_numerically_distinct() {
    for coordinate in [
        vec![-0.0, 0.0],
        vec![0.0, -0.0],
        vec![-1.0, -0.0, 1.0],
        vec![1.0, -0.0, -1.0],
    ] {
        let mut builder = rustyhdf5::FileBuilder::new();
        builder.create_dataset("time").with_f64_data(&coordinate);
        builder
            .create_dataset("out")
            .with_f64_data(&vec![-0.0; coordinate.len()]);
        let bytes = builder.finish().unwrap();
        for (name, format) in [
            ("signed-zero.h5", ResultImportFormat::Hdf5),
            ("signed-zero.mat", ResultImportFormat::MatlabV73),
        ] {
            let parsed = crate::workbench::workflows::result_import_workflow::parse_result_dataset(
                name, &bytes,
            );
            if coordinate.len() == 2 {
                let error = parsed.unwrap_err();
                assert!(error.contains("coordinate repeats"), "{error}");
            } else {
                let parsed = parsed.unwrap();
                assert_eq!(parsed.source_format, format);
                assert_eq!(
                    parsed.waveforms[0].x.read_whole().unwrap()[1].to_bits(),
                    (-0.0_f64).to_bits()
                );
                assert!(
                    parsed.waveforms[0]
                        .y
                        .read_whole()
                        .unwrap()
                        .iter()
                        .all(|v| v.to_bits() == (-0.0_f64).to_bits())
                );
            }
        }
    }
}

#[test]
fn hdf5_and_matlab_v73_import_real_root_vectors() {
    let bytes = generic_hdf5();
    assert_basic(
        parse_hdf5(&bytes, ResultImportFormat::Hdf5).expect("HDF5"),
        ResultImportFormat::Hdf5,
    );
    assert_basic(
        parse_matlab_v73(&bytes, ResultImportFormat::MatlabV73).expect("MATLAB 7.3"),
        ResultImportFormat::MatlabV73,
    );
    assert!(parse_hdf5(&bytes[..16], ResultImportFormat::Hdf5).is_err());
}

/// One RSpice HDF5 document holding a single section, named `group` and
/// declaring `section_type`.
///
/// Written through `rspice_core::io`, which is the one encoder both the CLI's
/// `--format hdf5` and this product's own export come through, so the fixture
/// is a file this product actually produces rather than a hand-built shape.
fn rspice_hdf5_section(group: &str, section_type: &str) -> Vec<u8> {
    rspice_hdf5_sections(&[(group, section_type)])
}

fn rspice_hdf5_sections(sections: &[(&str, &str)]) -> Vec<u8> {
    use rspice_core::io::{Hdf5Column, Hdf5Coordinate, Hdf5Document, Hdf5Table, write_hdf5};

    let mut document = Hdf5Document::new("fixture".to_owned());
    for (group, section_type) in sections {
        document
            .add_table(&Hdf5Table {
                group: (*group).to_owned(),
                section_type: (*section_type).to_owned(),
                coordinate: Hdf5Coordinate::Independent {
                    name: "time".to_owned(),
                    values: vec![0.0, 1e-9, 2e-9],
                },
                columns: vec![Hdf5Column::Real {
                    name: "V(out)".to_owned(),
                    quantity: "voltage".to_owned(),
                    unit: Some("V".to_owned()),
                    values: vec![0.0, 1.0, 0.0],
                }],
            })
            .expect("the fixture table stands on one coordinate");
    }
    let mut bytes = Vec::new();
    write_hdf5(&mut bytes, &document).expect("fixture bytes");
    bytes
}

/// A section group is read by what it declares it is, not by what it is called.
///
/// The layout contract says the group name is the producer's choice and
/// `section_type` names the family. The command line names its one section
/// group after the analysis instance that produced it — `tran1` — so a reader
/// keyed on the name alone read every identity-named file as an anonymous
/// root, fell through to the generic reader and refused a file this product
/// had just written.
#[test]
fn an_hdf5_section_is_read_from_what_it_declares_not_what_it_is_called() {
    for group in ["tran1", "transient"] {
        let parsed = parse_hdf5(
            &rspice_hdf5_section(group, "transient"),
            ResultImportFormat::Hdf5,
        )
        .unwrap_or_else(|error| panic!("a section group named '{group}': {error}"));
        assert_eq!(parsed.analysis_type, AnalysisType::Transient);
        assert_eq!(parsed.coordinate_name, "time");
        assert_eq!(parsed.waveforms.len(), 1);
        assert_eq!(parsed.waveforms[0].name, "V(out)");
        assert_eq!(
            parsed.waveforms[0].y.read_whole().unwrap().as_ref(),
            [0.0, 1.0, 0.0]
        );
        assert_eq!(parsed.waveforms[0].unit.as_deref(), Some("V"));
    }

    // A DC sweep under an instance name is the same claim on the other family,
    // and it must not be read as a transient because it is not called `ac`.
    let sweep = parse_hdf5(
        &rspice_hdf5_section("dc3", "dc_sweep"),
        ResultImportFormat::Hdf5,
    )
    .expect("an identity-named DC sweep");
    assert_eq!(sweep.analysis_type, AnalysisType::DcSweep);

    // A family outside the sampled domains is identified explicitly.
    let error = parse_hdf5(
        &rspice_hdf5_section("op1", "operating_point"),
        ResultImportFormat::Hdf5,
    )
    .expect_err("an operating point is not one of the three sampled domains");
    assert!(
        error.contains("section /op1 declares \"operating_point\"")
            && error.contains("cannot be represented as a waveform"),
        "{error}"
    );
}

#[test]
fn hdf5_import_refuses_to_discard_sections_outside_the_waveform_domains() {
    let error = parse_hdf5(
        &rspice_hdf5_sections(&[("tran1", "transient"), ("noise1", "noise")]),
        ResultImportFormat::Hdf5,
    )
    .expect_err("import must not discard the noise section");
    for detail in [
        "multiple result sections",
        "tran1",
        "noise1",
        "rspice convert --section",
    ] {
        assert!(error.contains(detail), "{error}");
    }
}

/// A result of one sample is a result, and reopens.
///
/// `MIN_RESULT_ROWS` was two, so a `.OP` — a single point by construction —
/// exported to HDF5, MAT or NumPy and then refused on the way back in. The
/// command line settled the rule at one first (`MIN_RESULT_SAMPLES`); this is
/// the same rule on this side of the boundary a file crosses.
///
/// Zero is still not a result, and the sentence that says so is pinned where
/// the reader can reach it: `parse_delimited_result_dataset`'s own case in
/// `result_import_workflow`.
#[test]
fn a_single_sample_source_is_a_result_and_imports() {
    let one_sample = {
        let mut builder = rustyhdf5::FileBuilder::new();
        builder.create_dataset("time").with_f64_data(&[0.0]);
        builder.create_dataset("V(out)").with_f64_data(&[2.5]);
        builder.finish().expect("HDF5 fixture")
    };
    let parsed = parse_hdf5(&one_sample, ResultImportFormat::Hdf5).expect("one point is a result");
    assert_eq!(parsed.sample_count, 1);
    assert_eq!(parsed.waveforms[0].x.read_whole().unwrap().as_ref(), [0.0]);
    assert_eq!(parsed.waveforms[0].y.read_whole().unwrap().as_ref(), [2.5]);
}

fn arrow_batch() -> (Arc<Schema>, RecordBatch) {
    let mut metadata = HashMap::new();
    metadata.insert("rspice.coordinate".to_owned(), "frequency".to_owned());
    metadata.insert("rspice.analysis".to_owned(), "ac".to_owned());
    let schema = Arc::new(Schema::new_with_metadata(
        vec![
            Field::new("frequency", DataType::Float64, false),
            Field::new("V(out)__real", DataType::Float64, false),
            Field::new("V(out)__imag", DataType::Float64, false),
        ],
        metadata,
    ));
    let arrays: Vec<ArrayRef> = vec![
        Arc::new(Float64Array::from(vec![1.0, 2.0])),
        Arc::new(Float64Array::from(vec![1.0, 2.0])),
        Arc::new(Float64Array::from(vec![0.5, -0.5])),
    ];
    let batch = RecordBatch::try_new(Arc::clone(&schema), arrays).expect("record batch");
    (schema, batch)
}

#[test]
fn arrow_file_and_stream_import_complex_columns() {
    let (schema, batch) = arrow_batch();
    let mut file_bytes = Vec::new();
    {
        let mut writer = arrow_ipc::writer::FileWriter::try_new(&mut file_bytes, &schema)
            .expect("Arrow file writer");
        writer.write(&batch).expect("Arrow file batch");
        writer.finish().expect("Arrow file finish");
    }
    assert_basic(
        parse_arrow_ipc(&file_bytes, ResultImportFormat::ArrowIpc).expect("Arrow file"),
        ResultImportFormat::ArrowIpc,
    );

    let mut stream_bytes = Vec::new();
    {
        let mut writer = arrow_ipc::writer::StreamWriter::try_new(&mut stream_bytes, &schema)
            .expect("Arrow stream writer");
        writer.write(&batch).expect("Arrow stream batch");
        writer.finish().expect("Arrow stream finish");
    }
    assert_basic(
        parse_arrow_ipc(&stream_bytes, ResultImportFormat::ArrowIpc).expect("Arrow stream"),
        ResultImportFormat::ArrowIpc,
    );
    assert!(parse_arrow_ipc(&file_bytes[..20], ResultImportFormat::ArrowIpc).is_err());
}

#[test]
fn parquet_imports_complex_columns_and_rejects_truncation() {
    use parquet::arrow::ArrowWriter;
    let (schema, batch) = arrow_batch();
    let mut writer = ArrowWriter::try_new(Vec::new(), schema, None).expect("Parquet writer");
    writer.write(&batch).expect("Parquet batch");
    let bytes = writer.into_inner().expect("Parquet finish");
    assert_basic(
        parse_parquet(&bytes, ResultImportFormat::Parquet).expect("Parquet"),
        ResultImportFormat::Parquet,
    );
    assert!(parse_parquet(&bytes[..20], ResultImportFormat::Parquet).is_err());
}

fn npy_f64(shape: &[u64], values: &[f64]) -> Vec<u8> {
    rspice_formats::numpy::encode_real_array(shape, values).expect("NPY fixture")
}

#[test]
fn numpy_npy_and_npz_import_real_arrays() {
    let npy = npy_f64(&[3, 2], &[0.0, 1.0, 1.0, 2.0, 2.0, 3.0]);
    assert_basic(
        parse_npy(&npy, ResultImportFormat::NumpyNpy).expect("NPY"),
        ResultImportFormat::NumpyNpy,
    );
    let time = npy_f64(&[3], &[0.0, 1.0, 2.0]);
    let out = npy_f64(&[3], &[1.0, 2.0, 3.0]);
    let npz = zip_bytes(&[("time.npy", &time), ("V(out).npy", &out)]);
    assert_basic(
        parse_npz(&npz, ResultImportFormat::NumpyNpz).expect("NPZ"),
        ResultImportFormat::NumpyNpz,
    );
    assert!(parse_npy(&npy[..12], ResultImportFormat::NumpyNpy).is_err());
    assert!(parse_npz(&npz[..20], ResultImportFormat::NumpyNpz).is_err());
}

fn mat_element(kind: u32, data: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    output.extend_from_slice(&kind.to_le_bytes());
    output.extend_from_slice(&(data.len() as u32).to_le_bytes());
    output.extend_from_slice(data);
    output.resize((output.len() + 7) & !7, 0);
    output
}

fn mat_matrix(name: &str, values: &[f64]) -> Vec<u8> {
    let mut body = Vec::new();
    let mut flags = Vec::new();
    flags.extend_from_slice(&6_u32.to_le_bytes());
    flags.extend_from_slice(&0_u32.to_le_bytes());
    body.extend(mat_element(6, &flags));
    let mut dimensions = Vec::new();
    dimensions.extend_from_slice(&(values.len() as i32).to_le_bytes());
    dimensions.extend_from_slice(&1_i32.to_le_bytes());
    body.extend(mat_element(5, &dimensions));
    body.extend(mat_element(1, name.as_bytes()));
    let value_bytes = values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect::<Vec<_>>();
    body.extend(mat_element(9, &value_bytes));
    mat_element(14, &body)
}

fn matlab_v5() -> Vec<u8> {
    let mut bytes = vec![b' '; 128];
    let description = b"MATLAB 5.0 MAT-file, RSpice import fixture";
    bytes[..description.len()].copy_from_slice(description);
    bytes[116..124].fill(0);
    bytes[124..126].copy_from_slice(&0x0100_u16.to_le_bytes());
    bytes[126..128].copy_from_slice(b"IM");
    bytes.extend(mat_matrix("time", &[0.0, 1.0, 2.0]));
    bytes.extend(mat_matrix("V_out", &[1.0, 2.0, 3.0]));
    bytes
}

#[test]
fn matlab_v5_imports_numeric_vectors_and_rejects_truncation() {
    let bytes = matlab_v5();
    assert_basic(
        parse_matlab_v5(&bytes, ResultImportFormat::MatlabV5).expect("MATLAB v5"),
        ResultImportFormat::MatlabV5,
    );
    assert!(parse_matlab_v5(&bytes[..140], ResultImportFormat::MatlabV5).is_err());
}

/// One Level 4 variable as SciPy's `savemat(format='4')` writes a 1-D array: header words
/// `[0, 1, n, 0, name + NUL]` (little-endian full doubles, one row), the name, the values.
fn mat_v4_row(name: &str, values: &[f64]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for word in [0, 1, values.len() as i32, 0, name.len() as i32 + 1] {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    bytes.extend_from_slice(name.as_bytes());
    bytes.push(0);
    bytes.extend(values.iter().flat_map(|value| value.to_le_bytes()));
    bytes
}

/// A Level 4 MAT-file is read as itself. It was taken for a Level 5 file, or, before the
/// FST signature was tightened, for an FST file. The values are SciPy's own, read back.
#[test]
fn a_level_four_mat_file_imports_as_itself() {
    let time = [0.0, 1e-6, 2e-6, 3e-6, 4e-6];
    let vout = [
        0.0,
        0.587785252292473,
        0.9510565162951535,
        0.9510565162951536,
        0.5877852522924736,
    ];
    let mut bytes = mat_v4_row("time", &time);
    bytes.extend(mat_v4_row("vout", &vout));
    let parsed = parse_result_dataset("scope.mat", &bytes).expect("MAT v4");
    assert_eq!(parsed.source_format, ResultImportFormat::MatlabV4);
    assert_eq!(parsed.analysis_type, AnalysisType::Transient);
    assert_eq!(parsed.waveforms[0].data.name, "vout");
    assert_eq!(parsed.waveforms[0].x.read_whole().unwrap().as_ref(), &time);
    assert_eq!(parsed.waveforms[0].y.read_whole().unwrap().as_ref(), &vout);

    let error = parse_result_dataset("scope.mat", &bytes[..bytes.len() - 3])
        .expect_err("a cut file is refused");
    assert!(error.contains("cut short inside variable 2"), "{error}");
}

/// A workspace saved with a setting beside its signals imports the signals and says, in the
/// review, that the setting was left out.
#[test]
fn a_matlab_workspace_scalar_beside_the_signals_is_left_out_with_a_note() {
    let mut bytes = matlab_v5();
    bytes.extend(mat_matrix("gain", &[2.0]));
    let parsed = parse_result_dataset("workspace.mat", &bytes).expect("signals import");
    let names: Vec<_> = parsed
        .waveforms
        .iter()
        .map(|trace| &trace.data.name)
        .collect();
    assert_eq!(names, ["V_out"]);
    assert_eq!(
        parsed.notes,
        [
            "The MATLAB variable 'gain' holds one value, not a signal over 'time', so it is left out."
        ]
    );
}

/// The op-then-tran rawfile ngspice-46 writes under `-r` for an RC divider
/// stepped from 0.5 V, in its ASCII encoding: its header lines verbatim, cut
/// to the first three transient rows.
const NGSPICE_OP_THEN_TRAN: &[u8] = b"Title: rc op then tran\nDate: Fri Nov 13 06:07:38  2026\nCommand: ngspice-46, Build \nPlotname: Operating Point\nFlags: real\nNo. Variables: 3\nNo. Points: 1       \nVariables:\n\t0\tv(in)\tvoltage\n\t1\tv(out)\tvoltage\n\t2\ti(v1)\tcurrent\nValues:\n0\t\t5.000000000000000e-01\n\t3.750000000000000e-01\n\t-1.250000000000000e-04\n\
Title: rc op then tran\nDate: Fri Nov 13 06:07:38  2026\nCommand: ngspice-46, Build \nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 4\nNo. Points: 3       \nVariables:\n\t0\ttime\ttime\n\t1\tv(in)\tvoltage\n\t2\tv(out)\tvoltage\n\t3\ti(v1)\tcurrent\nValues:\n\
0\t\t0.000000000000000e+00\n\t5.000000000000000e-01\n\t3.750000000000000e-01\n\t-1.250000000000000e-04\n\
1\t\t4.000000000000001e-12\n\t5.002000000000000e-01\n\t3.750000799573561e-01\n\t-1.251999200426440e-04\n\
2\t\t8.000000000000001e-12\n\t5.004000000000000e-01\n\t3.750002398294470e-01\n\t-1.253997601705530e-04\n";

/// Every plot of an ngspice rawfile comes in, one analysis each, in one run:
/// the transient is the one reviewed, and the operating point lands beside
/// it as written.
#[test]
fn an_ngspice_op_then_tran_rawfile_imports_as_one_run_of_both_analyses() {
    use crate::workbench::documents::result_document::test_host::ResultsTestHost;

    let parsed = parse_spice_raw(NGSPICE_OP_THEN_TRAN, None, ResultImportFormat::SpiceRaw)
        .expect("a multi-plot rawfile imports");
    assert_eq!(parsed.analysis_type, AnalysisType::Transient);
    assert_eq!(parsed.sample_count, 3);
    let plots = parsed
        .plots
        .as_deref()
        .expect("the source holds two analyses");
    assert_eq!(plots.reviewed, "Transient Analysis");
    let [op] = plots.further.as_slice() else {
        panic!("one further analysis, got {}", plots.further.len());
    };
    assert_eq!(op.name, "Operating Point");
    assert_eq!(op.analysis_type, AnalysisType::DcSweep);
    assert_eq!((op.coordinate_name.as_str(), op.sample_count), ("point", 1));

    let mut state = ResultsTestHost::default();
    state.platform.project_lifecycle.authority.open_session();
    stage_imported_result_dataset(
        &mut state.results_parts(),
        "optran.raw",
        NGSPICE_OP_THEN_TRAN,
    )
    .unwrap();
    assert_eq!(state.results.result_import.plots.as_deref(), Some(plots));
    commit_result_import_draft(&mut state.results_parts()).unwrap();
    let run = state
        .model
        .simulation
        .active_run()
        .expect("the import is shown");
    let analyses = run
        .analyses
        .iter()
        .map(|analysis| (analysis.label.as_str(), analysis.analysis_type))
        .collect::<Vec<_>>();
    assert_eq!(
        analyses,
        [
            ("Imported · Transient Analysis", AnalysisType::Transient),
            ("Imported · Operating Point", AnalysisType::DcSweep),
        ]
    );
    let op_values = run.analyses[1]
        .waveforms
        .iter()
        .map(|waveform| {
            (
                waveform.name.as_str(),
                waveform.y.samples(..).collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        op_values,
        [
            ("v(in)", vec![0.5]),
            ("v(out)", vec![0.375]),
            ("i(v1)", vec![-0.000125]),
        ]
    );
    assert_eq!(run.analyses[0].waveforms[1].name, "v(out)");
    assert_eq!(
        run.analyses[0].waveforms[1]
            .y
            .read_whole()
            .unwrap()
            .as_ref(),
        [0.375, 0.3750000799573561, 0.375000239829447]
    );
    let message = &state.model.log_buffer.entries().last().unwrap().message;
    assert!(message.contains("and 1 further analysis"), "{message}");
}

#[test]
fn spice_raw_and_psf_ascii_import_real_waveforms() {
    let raw = b"Title: fixture\nDate: now\nPlotname: Transient Analysis\nFlags: real double\nNo. Variables: 2\nNo. Points: 3\nVariables:\n\t0\ttime\ttime\n\t1\tV(out)\tvoltage\nValues:\n0\t0\n\t1\n1\t1e-9\n\t2\n2\t2e-9\n\t3\n";
    assert_basic(
        parse_spice_raw(raw, None, ResultImportFormat::SpiceRaw).expect("SPICE RAW"),
        ResultImportFormat::SpiceRaw,
    );
    let psf = b"HEADER\n\"analysis\" \"tran\"\nSWEEP\n\"time\" \"s\"\nTRACE\n\"V(out)\" \"V\"\nVALUE\n0 1\n1e-9 2\n2e-9 3\nEND\n";
    assert_basic(
        parse_psf_ascii(psf, ResultImportFormat::PsfAscii).expect("PSF ASCII"),
        ResultImportFormat::PsfAscii,
    );
    assert!(parse_spice_raw(&raw[..40], None, ResultImportFormat::SpiceRaw).is_err());
    assert!(parse_psf_ascii(&psf[..30], ResultImportFormat::PsfAscii).is_err());
}

#[test]
fn vcd_imports_initialized_digital_events() {
    let vcd = b"$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! clk $end\n$var wire 2 \" bus $end\n$upscope $end\n$enddefinitions $end\n#0\n0!\nb00 \"\n#5\n1!\nb01 \"\n#10\n0!\nb10 \"\n";
    let parsed = parse_vcd(vcd, ResultImportFormat::Vcd).expect("VCD");
    assert_eq!(
        parsed.notes,
        vec![
            "1 vector variable is retained as a declared digital bus over its member traces."
                .to_owned()
        ],
        "a two-state file decides nothing about its samples; it does state what it declared"
    );
    assert_eq!(
        parsed
            .waveforms
            .iter()
            .map(|waveform| waveform.name.as_str())
            .collect::<Vec<_>>(),
        vec!["top.clk", "top.bus"]
    );
    assert_eq!(
        parsed.waveforms[1].y.read_whole().unwrap().as_ref(),
        &[0.0, 1.0, 2.0]
    );
    assert_basic(parsed, ResultImportFormat::Vcd);
}

/// A dump of more ticks than the million rows an import once stopped at
/// imports whole: the size of real data is never a reason to refuse it.
#[test]
fn a_vcd_of_more_than_a_million_ticks_imports_every_tick() {
    const TICKS: usize = 1_000_003;
    let mut vcd = String::from(
        "$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! clk $end\n\
         $upscope $end\n$enddefinitions $end\n",
    );
    for tick in 0..TICKS {
        vcd.push_str(&format!("#{tick}\n{}!\n", tick % 2));
    }
    let parsed = parse_vcd(vcd.as_bytes(), ResultImportFormat::Vcd).expect("a long dump imports");
    assert_eq!(parsed.sample_count, TICKS);
    let clock = &parsed.waveforms[0];
    assert_eq!(clock.x.len(), TICKS);
    assert_eq!(
        clock.x.read_range(TICKS - 1..TICKS).unwrap()[0],
        (TICKS - 1) as f64 * 1.0e-9
    );
    assert_eq!(
        clock.y.read_range(TICKS - 2..TICKS).unwrap().as_ref(),
        &[1.0, 0.0]
    );
}

/// A vector no `f64` sample can hold used to be refused outright, taking the
/// rest of the file with it. It imports: the word is retained whole as a
/// declared bus, and the grid — which is where the f64 limit lives — carries
/// one column per bit instead of one rounded integer.
#[test]
fn a_vector_no_f64_sample_can_hold_reaches_the_grid_one_bit_at_a_time() {
    let mut vcd = String::from(
        "$timescale 1 ns $end\n$scope module top $end\n$var wire 96 ! wide $end\n$upscope $end\n$enddefinitions $end\n#0\nb",
    );
    vcd.push_str(&"0".repeat(96));
    vcd.push_str(" !\n#1\nb1");
    vcd.push_str(&"0".repeat(95));
    vcd.push_str(" !\n");
    let parsed =
        parse_vcd(vcd.as_bytes(), ResultImportFormat::Vcd).expect("a 96-bit vector imports");
    assert_eq!(parsed.waveforms.len(), 96);
    assert_eq!(parsed.waveforms[0].name, "top.wide[95]");
    assert_eq!(
        parsed.waveforms[0].y.read_whole().unwrap().as_ref(),
        &[0.0, 1.0]
    );
    assert!(
        parsed.notes.iter().any(
            |note| note.starts_with("1 vector variable is wider than 53 bits")
                && note.contains("one column per bit")
        ),
        "unexpected notes: {:?}",
        parsed.notes
    );
}

fn fst_test_blocks(bytes: &[u8]) -> Vec<(u8, usize, usize)> {
    let mut blocks = Vec::new();
    let mut cursor = 0_usize;
    while cursor < bytes.len() {
        let block = cursor;
        let block_type = bytes[cursor];
        cursor += 1;
        let length = u64::from_be_bytes(bytes[cursor..cursor + 8].try_into().unwrap());
        if block_type == 255 && length == 0 {
            break;
        }
        let end = cursor + usize::try_from(length).unwrap();
        blocks.push((block_type, block, end));
        cursor = end;
    }
    blocks
}

fn set_fst_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_be_bytes());
}

fn fst_test_block(block_type: u8, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(9 + payload.len());
    bytes.push(block_type);
    bytes.extend_from_slice(&(8_u64 + payload.len() as u64).to_be_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn test_uleb(mut value: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        let next = value >> 7;
        bytes.push((value as u8 & 0x7f) | if next == 0 { 0 } else { 0x80 });
        value = next;
        if value == 0 {
            break;
        }
    }
    bytes
}

fn fst_with_packed_signal(pack_type: u8, expanded_size: u64) -> Vec<u8> {
    let mut header = vec![0_u8; (rspice_formats::fst::FST_HEADER_SECTION_BYTES - 8) as usize];
    header[16..24].copy_from_slice(&std::f64::consts::E.to_be_bytes());
    set_fst_u64(&mut header, 32, 1); // scopes
    set_fst_u64(&mut header, 40, 1); // variables
    set_fst_u64(&mut header, 48, 1); // handles
    set_fst_u64(&mut header, 56, 1); // value-change sections
    let mut bytes = fst_test_block(0, &header);

    let mut geometry = Vec::new();
    geometry.extend_from_slice(&1_u64.to_be_bytes());
    geometry.extend_from_slice(&1_u64.to_be_bytes());
    geometry.push(1); // one-bit signal
    bytes.extend(fst_test_block(3, &geometry));

    let mut hierarchy = Vec::new();
    hierarchy.extend_from_slice(&1_u64.to_be_bytes());
    hierarchy.push(0); // declaration is enough for a preflight failure fixture
    bytes.extend(fst_test_block(6, &hierarchy));

    let signal = test_uleb(expanded_size);
    let mut data = Vec::new();
    data.extend_from_slice(&0_u64.to_be_bytes()); // start
    data.extend_from_slice(&1_u64.to_be_bytes()); // end
    data.extend_from_slice(&1_024_u64.to_be_bytes()); // an allocation the source can hold
    data.extend([1, 1, 1, b'0']); // direct one-byte frame, one handle
    data.push(1); // value-change handle count
    data.push(pack_type);
    data.extend_from_slice(&signal);
    data.push(3); // DynamicAlias2 first signal offset is one byte after pack type
    data.extend_from_slice(&1_u64.to_be_bytes()); // offset-table byte count
    data.push(1); // direct one-byte time delta stream
    data.extend_from_slice(&1_u64.to_be_bytes());
    data.extend_from_slice(&1_u64.to_be_bytes());
    data.extend_from_slice(&1_u64.to_be_bytes());
    bytes.extend(fst_test_block(8, &data));
    bytes
}

#[test]
fn fst_reader_imports_a_real_generated_container_and_rejects_truncation() {
    let bytes = generated_fst();
    assert!(rspice_formats::fst::looks_like_fst(&bytes));
    preflight_fst(&bytes, ResultImportFormat::Fst).expect("ordinary FST preflight");
    assert_basic(
        parse_fst(&bytes, ResultImportFormat::Fst).expect("FST import"),
        ResultImportFormat::Fst,
    );
    assert!(parse_fst(&bytes[..64], ResultImportFormat::Fst).is_err());
}

#[test]
fn fst_preflight_rejects_huge_fixed_allocation_declarations() {
    let baseline = generated_fst();
    let blocks = fst_test_blocks(&baseline);
    let geometry = blocks.iter().find(|block| block.0 == 3).copied().unwrap();
    let hierarchy = blocks
        .iter()
        .find(|block| matches!(block.0, 4 | 6 | 7))
        .copied()
        .unwrap();
    let data = blocks
        .iter()
        .find(|block| matches!(block.0, 1 | 5 | 8))
        .copied()
        .unwrap();
    let time_compressed = usize::try_from(u64::from_be_bytes(
        baseline[data.2 - 16..data.2 - 8].try_into().unwrap(),
    ))
    .unwrap();
    let offset_table_length = data.2 - 24 - time_compressed - 8;
    let too_large = DECLARED_PAST_THE_SOURCE + 1;

    for (label, offset) in [
        ("header signal count", 1 + 8 + 48),
        ("geometry expanded", geometry.1 + 1 + 8),
        ("hierarchy expanded", hierarchy.1 + 1 + 8),
        ("data allocation", data.1 + 1 + 24),
        ("offset-table compressed", offset_table_length),
        ("time-table expanded", data.2 - 24),
        ("time-table compressed", data.2 - 16),
        ("time-table item count", data.2 - 8),
    ] {
        let mut bytes = baseline.clone();
        set_fst_u64(&mut bytes, offset, too_large);
        let error = preflight_fst(&bytes, ResultImportFormat::Fst)
            .expect_err("oversized FST declaration must reject");
        assert!(
            error.contains("limit") || error.contains("count"),
            "{label}: {error}"
        );
    }
}

#[test]
fn fst_preflight_rejects_huge_frame_and_signal_count_varints() {
    let baseline = generated_fst();
    let data = fst_test_blocks(&baseline)
        .into_iter()
        .find(|block| matches!(block.0, 1 | 5 | 8))
        .unwrap();
    let oversized = test_uleb(DECLARED_PAST_THE_SOURCE + 1);
    for (label, offset) in [
        ("initial-frame expanded", data.1 + 33),
        ("initial-frame compressed", data.1 + 34),
        ("initial-frame signal count", data.1 + 35),
        ("value-change signal count", data.1 + 37),
    ] {
        let mut bytes = baseline.clone();
        bytes[offset..offset + oversized.len()].copy_from_slice(&oversized);
        let error = preflight_fst(&bytes, ResultImportFormat::Fst)
            .expect_err("oversized FST variable integer must reject");
        assert!(
            error.contains("limit") || error.contains("count"),
            "{label}: {error}"
        );
    }
}

#[test]
fn fst_preflight_rejects_wrappers_unknown_blocks_overflow_and_truncation() {
    let mut invalid_header = generated_fst();
    invalid_header[1 + 8 + 16..1 + 8 + 24].fill(0);
    let error =
        preflight_fst(&invalid_header, ResultImportFormat::Fst).expect_err("invalid endian marker");
    assert!(error.contains("endian marker"), "{error}");

    let mut reversed_time = generated_fst();
    set_fst_u64(&mut reversed_time, 1 + 8, 2);
    set_fst_u64(&mut reversed_time, 1 + 8 + 8, 1);
    let error = preflight_fst(&reversed_time, ResultImportFormat::Fst)
        .expect_err("reversed header time range");
    assert!(error.contains("precedes"), "{error}");

    let wrapper = fst_test_block(254, &u64::MAX.to_be_bytes());
    let error = preflight_fst(&wrapper, ResultImportFormat::Fst).expect_err("gzip bomb");
    assert!(error.contains("gzip wrapper expanded"), "{error}");

    let bounded_wrapper = fst_test_block(254, &16_u64.to_be_bytes());
    let error = preflight_fst(&bounded_wrapper, ResultImportFormat::Fst)
        .expect_err("nested gzip cannot be preflighted");
    assert!(error.contains("nested framing"), "{error}");

    let unknown = fst_test_block(9, &[]);
    assert!(
        preflight_fst(&unknown, ResultImportFormat::Fst)
            .expect_err("unknown block")
            .contains("unknown")
    );

    let mut overflow = vec![0];
    overflow.extend_from_slice(&u64::MAX.to_be_bytes());
    assert!(preflight_fst(&overflow, ResultImportFormat::Fst).is_err());
    assert!(preflight_fst(&[0, 0, 0], ResultImportFormat::Fst).is_err());
}

#[test]
fn fst_preflight_rejects_compressed_geometry_and_duo_intermediate_bombs() {
    let mut compressed_geometry = generated_fst();
    let geometry = fst_test_blocks(&compressed_geometry)
        .into_iter()
        .find(|block| block.0 == 3)
        .unwrap();
    let declared = u64::from_be_bytes(
        compressed_geometry[geometry.1 + 9..geometry.1 + 17]
            .try_into()
            .unwrap(),
    );
    set_fst_u64(
        &mut compressed_geometry,
        geometry.1 + 9,
        declared.saturating_add(1),
    );
    let error = preflight_fst(&compressed_geometry, ResultImportFormat::Fst)
        .expect_err("compressed geometry must fail closed");
    assert!(error.contains("compressed FST geometry"), "{error}");

    let mut duo = Vec::new();
    duo.extend_from_slice(&1_u64.to_be_bytes());
    duo.extend(test_uleb(DECLARED_PAST_THE_SOURCE + 1));
    let duo = fst_test_block(7, &duo);
    let error = preflight_fst(&duo, ResultImportFormat::Fst).expect_err("LZ4 duo bomb");
    assert!(error.contains("LZ4-duo"), "{error}");
}

#[test]
fn fst_preflight_bounds_lz4_fastlz_and_zlib_signal_expansion() {
    for (name, pack_type) in [("LZ4", b'4'), ("FastLZ", b'F'), ("zlib", b'Z')] {
        let bytes = fst_with_packed_signal(pack_type, DECLARED_PAST_THE_SOURCE + 1);
        let error = preflight_fst(&bytes, ResultImportFormat::Fst)
            .expect_err("packed FST signal bomb must reject");
        assert!(error.contains(name), "{name}: {error}");
        assert!(error.contains("limit"), "{name}: {error}");
    }
}

/// One retained digital trace as `(node, [(time_s, value_code)])`.
type ImportedTrace = (String, Vec<(f64, u8)>);
/// One retained declaration as `(name, msb, lsb, members)`.
type ImportedBus = (String, i64, i64, Vec<String>);

/// The exact digital event evidence an import retained, for the tests that
/// read it: the member histories and the declarations over them.
fn imported_events(parsed: &ParsedResultDataset) -> (Vec<ImportedTrace>, Vec<ImportedBus>) {
    let Some(crate::state::AnalysisResultPayload::TransientEvents {
        digital_traces,
        digital_buses,
        ..
    }) = parsed.event_payload.as_ref()
    else {
        panic!("a digital import retains its event history");
    };
    (
        digital_traces
            .iter()
            .map(|trace| {
                (
                    trace.node_name.clone(),
                    trace
                        .points
                        .iter()
                        .map(|point| (point.time_s, point.value_code))
                        .collect(),
                )
            })
            .collect(),
        digital_buses
            .iter()
            .map(|bus| (bus.name.clone(), bus.msb, bus.lsb, bus.members.clone()))
            .collect(),
    )
}

/// Two vector variables import as two `Import` declarations over their
/// members, and every member keeps its own four-state history.
#[test]
fn a_two_bus_dump_imports_as_two_declarations_over_their_members() {
    let vcd = b"$timescale 1 ns $end\n$scope module top $end\n$var wire 2 ! addr [1:0] $end\n$var wire 2 \" data [1:0] $end\n$upscope $end\n$enddefinitions $end\n#0\nb00 !\nb01 \"\n#5\nb01 !\nb10 \"\n#10\nb11 !\nb11 \"\n";
    let parsed = parse_vcd(vcd, ResultImportFormat::Vcd).expect("a two-bus dump imports");
    let (traces, buses) = imported_events(&parsed);
    assert_eq!(
        buses,
        vec![
            (
                "addr".to_owned(),
                1,
                0,
                vec!["addr[1]".to_owned(), "addr[0]".to_owned()]
            ),
            (
                "data".to_owned(),
                1,
                0,
                vec!["data[1]".to_owned(), "data[0]".to_owned()]
            ),
        ]
    );
    assert_eq!(
        traces
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>(),
        vec!["addr[1]", "addr[0]", "data[1]", "data[0]"]
    );
    assert!(
        parsed
            .notes
            .iter()
            .any(|note| note == "2 vector variables are retained as declared digital buses over their member traces."),
        "unexpected notes: {:?}",
        parsed.notes
    );
}

/// A 64-bit vector was refused outright — no f64 holds its integer. It now
/// imports: the evidence carries the whole word as a declared bus, and the
/// sampled grid spreads it over one column per bit rather than rounding it.
#[test]
fn a_vector_wider_than_an_exact_f64_integer_imports_as_a_bus() {
    let zeros = "0".repeat(64);
    let mut ones = "1".to_owned();
    ones.push_str(&"0".repeat(63));
    let vcd = format!(
        "$timescale 1 ns $end\n$scope module top $end\n$var wire 64 ! wide [63:0] $end\n$upscope $end\n$enddefinitions $end\n#0\nb{zeros} !\n#1\nb{ones} !\n"
    );
    let parsed =
        parse_vcd(vcd.as_bytes(), ResultImportFormat::Vcd).expect("a 64-bit vector imports");
    let (traces, buses) = imported_events(&parsed);
    assert_eq!(buses.len(), 1);
    assert_eq!(
        buses[0].0, "wide",
        "reading a dump back drops the scope path every variable shares, which is what \
         `rspice convert` and `--variables` see too"
    );
    assert_eq!(buses[0].3.len(), 64);
    assert_eq!(traces.len(), 64);
    assert_eq!(
        parsed
            .waveforms
            .iter()
            .map(|waveform| waveform.name.as_str())
            .take(2)
            .collect::<Vec<_>>(),
        vec!["top.wide[63]", "top.wide[62]"],
        "the grid spreads a word no f64 holds over one column per bit"
    );
    assert_eq!(parsed.waveforms.len(), 64);
    assert!(
        parsed.notes.iter().any(|note| note.starts_with(
            "1 vector variable is wider than 53 bits, which no f64 sample holds exactly"
        )),
        "unexpected notes: {:?}",
        parsed.notes
    );
}

/// The grid's 0.5 is a property of the grid. The evidence keeps the code the
/// file recorded, so an unknown bit stays unknown all the way to the sheet.
#[test]
fn an_unknown_or_high_impedance_change_is_retained_as_its_code() {
    let vcd = b"$timescale 1 ns $end\n$scope module top $end\n$var wire 2 ! bus [1:0] $end\n$upscope $end\n$enddefinitions $end\n#0\nb0x !\n#1\nbz1 !\n";
    let parsed = parse_vcd(vcd, ResultImportFormat::Vcd).expect("a four-state vector imports");
    let (traces, _) = imported_events(&parsed);
    // Codes: 0 is strong zero, 1 is strong one, 2 is unknown, 12 is high-Z.
    assert_eq!(
        traces,
        vec![
            ("bus[1]".to_owned(), vec![(0.0, 0), (1.0e-9, 12)]),
            ("bus[0]".to_owned(), vec![(0.0, 2), (1.0e-9, 1)]),
        ]
    );
    assert_eq!(
        parsed.waveforms[0].y.read_whole().unwrap().as_ref(),
        &[0.5, 0.5],
        "the grid still projects an unresolved bit at its own level"
    );
}

fn ltspice_fixture(name: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../rspice-core-base/tests/fixtures/ltspice")
        .join(name);
    std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// Every layout LTspice 17.2.4 writes imports, UTF-16 header and all.
#[test]
fn rawfiles_ltspice_wrote_import() {
    for (name, analysis_type, samples) in [
        ("rc_tran.raw", AnalysisType::Transient, 23),
        ("rc_tran_double.raw", AnalysisType::Transient, 23),
        ("rc_fast.raw", AnalysisType::Transient, 277),
        ("rc_ac.raw", AnalysisType::Ac, 121),
        ("rc_noise.raw", AnalysisType::Ac, 61),
        ("rc_dc.raw", AnalysisType::DcSweep, 21),
        ("rc_op.raw", AnalysisType::DcSweep, 1),
    ] {
        let parsed = parse_result_dataset(name, &ltspice_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(
            (parsed.analysis_type, parsed.sample_count),
            (analysis_type, samples),
            "{name}"
        );
    }
}

/// An LTspice `.step` rawfile holds every run in one plot; with its log
/// beside it, each run imports as its own analysis named by its values, and
/// the runs of the import are one family.
#[test]
fn an_ltspice_stepped_rawfile_imports_one_analysis_per_run_named_by_its_log() {
    use crate::workbench::documents::result_document::test_host::ResultsTestHost;
    use rspice_results::provenance::AnalysisResultFamilyMember;

    let mut state = ResultsTestHost::default();
    state.platform.project_lifecycle.authority.open_session();
    stage_imported_result_dataset_beside(
        &mut state.results_parts(),
        "rc_step.raw",
        &ltspice_fixture("rc_step.raw"),
        Some(&ltspice_fixture("rc_step.log")),
    )
    .unwrap();
    commit_result_import_draft(&mut state.results_parts()).unwrap();
    let run = state
        .model
        .simulation
        .active_run()
        .expect("the import is shown");
    let analyses: Vec<_> = run
        .analyses
        .iter()
        .map(|analysis| {
            let Some(AnalysisResultFamilyMember::StepPoint { index, coordinate }) =
                analysis.family_member()
            else {
                panic!("{} names no run", analysis.label);
            };
            (
                analysis.label.as_str(),
                *index,
                coordinate[0].value,
                analysis.waveforms[0].x.len(),
            )
        })
        .collect();
    assert_eq!(
        analyses,
        [
            (
                "Imported \u{b7} Transient Analysis \u{b7} r=1k",
                0,
                1000.0,
                55
            ),
            (
                "Imported \u{b7} Transient Analysis \u{b7} r=2k",
                1,
                2000.0,
                31
            ),
            (
                "Imported \u{b7} Transient Analysis \u{b7} r=4k",
                2,
                4000.0,
                19
            ),
        ]
    );
}

/// An HSPICE ASCII waveform file: the header's counts and version, then from byte 256 the
/// column types and names, wrapped at 80 columns as HSPICE writes it, then `values`.
fn hspice_ascii(columns: &[(u32, &str)], sweep: Option<&str>, values: &str) -> Vec<u8> {
    let mut header = format!(
        "{:04}{:04}{:04}0000{:<8}{:<64}",
        columns.len(),
        0,
        usize::from(sweep.is_some()),
        "9601",
        "* swept rc"
    );
    header.push_str(&" ".repeat(256 - header.len()));
    for (kind, _) in columns {
        header.push_str(&format!("{kind:<8}"));
    }
    for name in columns.iter().map(|(_, name)| *name).chain(sweep) {
        header.push_str(&format!("{name:<16}"));
    }
    header.push_str("$&%#");
    let mut bytes = Vec::new();
    for line in header.as_bytes().chunks(80) {
        bytes.extend_from_slice(line);
        bytes.push(b'\n');
    }
    bytes.extend_from_slice(values.as_bytes());
    bytes
}

fn step_values(
    member: Option<&rspice_results::provenance::AnalysisResultFamilyMember>,
) -> Vec<(String, f64)> {
    let Some(rspice_results::provenance::AnalysisResultFamilyMember::StepPoint {
        coordinate, ..
    }) = member
    else {
        panic!("{member:?} is no run of a sweep");
    };
    coordinate
        .iter()
        .map(|point| (point.name.clone(), point.value))
        .collect()
}

/// A swept HSPICE transient imports one analysis per sweep value, as one family.
#[test]
fn an_hspice_swept_transient_imports_one_analysis_per_sweep_value() {
    let values = "0.25000E+020.00000E+000.00000E+000.10000E-080.50000E+000.10000E+31\n\
                  0.85000E+020.00000E+000.00000E+000.10000E-080.40000E+000.10000E+31\n";
    let bytes = hspice_ascii(&[(1, "TIME"), (1, "v(out")], Some("temp"), values);
    let parsed = parse_result_dataset("rc.tr0", &bytes).expect("HSPICE transient");
    assert_eq!(parsed.source_format, ResultImportFormat::Hspice);
    assert_eq!(parsed.analysis_type, AnalysisType::Transient);
    assert_eq!(parsed.waveforms[0].data.name, "v(out)");
    assert_eq!(
        parsed.waveforms[0].y.read_whole().unwrap().as_ref(),
        &[0.0, 0.5]
    );
    let plots = parsed.plots.as_deref().expect("two runs");
    assert_eq!(
        step_values(plots.reviewed_member.as_ref()),
        [("temp".to_owned(), 25.0)]
    );
    assert_eq!(plots.further.len(), 1);
    assert_eq!(
        step_values(plots.further[0].family_member.as_ref()),
        [("temp".to_owned(), 85.0)]
    );

    let error = parse_result_dataset("rc.tr0", &bytes[..bytes.len() - 12]).expect_err("cut");
    assert_eq!(
        error,
        "HSPICE waveform import: the file ends inside a table, before the value that closes it"
    );
}

/// A `.measure` table imports as a scalar result: one value per measurement and row.
#[test]
fn an_hspice_measurement_table_imports_as_a_scalar_result() {
    let table = "$DATA1 SOURCE='HSPICE' VERSION='P-2019.06-SP1-1 win64' PARAM_COUNT=0\n\
                 .TITLE '**********carry ripple adder***************'\n \
                 delay_cout       delay_s8         power_avg        temper          \n \
                 alter#          \n  \
                 2.367e-10        4.291e-10       -1.548e-05         25.0000       \n\
                 1               \n";
    let parsed = parse_result_dataset("ca3.mt0", table.as_bytes()).expect("measurements");
    assert_eq!(parsed.source_format, ResultImportFormat::HspiceMeasure);
    assert_eq!(parsed.coordinate_name, "point");
    assert_eq!(parsed.sample_count, 1);
    let values: Vec<_> = parsed
        .waveforms
        .iter()
        .map(|waveform| {
            (
                waveform.data.name.as_str(),
                waveform.y.read_whole().unwrap()[0],
            )
        })
        .collect();
    assert_eq!(
        values,
        [
            ("delay_cout", 2.367e-10),
            ("delay_s8", 4.291e-10),
            ("power_avg", -1.548e-05),
            ("temper", 25.0),
            ("alter#", 1.0),
        ]
    );
}

/// A stepped `.qraw` imports one analysis per run, named by its parameter columns.
#[test]
fn a_qspice_stepped_qraw_imports_one_analysis_per_run_named_by_its_parameters() {
    let source = "Title: * step\nPlotname: Transient Analysis\nFlags: real stepped\n\
                  No. Variables: 3\nNo. Points: 4                    \n\
                  Command: QSPICE64, Build Sep 24 2023 18:58:57\n.param temp=27\nVariables:\n\
                  \t0\tTime\ttime\n\t1\tV(out)\tvoltage\n\t2\tVIN\tparameter\nValues:\n\
                  0\t\t0.0\n\t0.0\n\t1.0\n1\t\t1.0e-3\n\t0.5\n\t1.0\n\
                  2\t\t0.0\n\t0.0\n\t10.0\n3\t\t1.0e-3\n\t5.0\n\t10.0\n";
    let parsed = parse_result_dataset("TRAN - STEP.qraw", source.as_bytes()).expect("QSPICE");
    assert_eq!(parsed.source_format, ResultImportFormat::QspiceRaw);
    let names: Vec<_> = parsed
        .waveforms
        .iter()
        .map(|waveform| waveform.data.name.as_str())
        .collect();
    assert_eq!(names, ["V(out)"]);
    let plots = parsed.plots.as_deref().expect("two runs");
    assert_eq!(
        step_values(plots.reviewed_member.as_ref()),
        [("VIN".to_owned(), 1.0)]
    );
    assert_eq!(
        step_values(plots.further[0].family_member.as_ref()),
        [("VIN".to_owned(), 10.0)]
    );
}

/// A log that names fewer runs than the rawfile holds cannot name them; the review says so.
#[test]
fn a_log_that_disagrees_with_the_rawfile_says_the_runs_could_not_be_named() {
    let log: Vec<u8> = "LTspice 17.2.4 for MacOS\n.step r=1000\n.step r=2000\nTotal elapsed time: 0.005 seconds.\n"
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect();
    let parsed = parse_spice_raw(
        &ltspice_fixture("rc_step.raw"),
        Some(&log),
        ResultImportFormat::SpiceRaw,
    )
    .expect("the runs import, numbered");
    assert_eq!(
        parsed.notes,
        [
            "The log beside the rawfile names 2 runs but the rawfile holds 3, so the runs' step \
          values could not be named and the runs are numbered instead."
        ]
    );
}

/// The browser reads the one file the reader picks, so a stepped rawfile arrives without its
/// log: the review says its runs are numbered, and the log the reader then chooses names them.
#[test]
fn a_stepped_rawfile_read_without_its_log_names_its_runs_from_the_log_chosen_after() {
    use crate::workbench::documents::result_document::test_host::ResultsTestHost;

    let mut state = ResultsTestHost::default();
    state.platform.project_lifecycle.authority.open_session();
    stage_imported_result_dataset(
        &mut state.results_parts(),
        "rc_step.raw",
        &ltspice_fixture("rc_step.raw"),
    )
    .unwrap();
    assert_eq!(state.results.result_import.notes, [UNNAMED_RUNS_NOTE]);
    assert_eq!(unnamed_result_import_runs(&state.results.result_import), 3);

    let two_runs: Vec<u8> = "LTspice 17.2.4\n.step r=1000\n.step r=2000\n"
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect();
    assert_eq!(
        name_result_import_runs(&mut state.results.result_import, &two_runs),
        Err("The log names 2 runs but the rawfile holds 3, so it cannot name them.".to_owned())
    );

    name_result_import_runs(
        &mut state.results.result_import,
        &ltspice_fixture("rc_step.log"),
    )
    .unwrap();
    assert!(state.results.result_import.notes.is_empty());
    commit_result_import_draft(&mut state.results_parts()).unwrap();
    let run = state
        .model
        .simulation
        .active_run()
        .expect("the import is shown");
    let labels: Vec<_> = run
        .analyses
        .iter()
        .map(|analysis| analysis.label.as_str())
        .collect();
    assert_eq!(
        labels,
        [
            "Imported \u{b7} Transient Analysis \u{b7} r = 1000",
            "Imported \u{b7} Transient Analysis \u{b7} r = 2000",
            "Imported \u{b7} Transient Analysis \u{b7} r = 4000",
        ]
    );
}

/// A damaged or hostile file of each format, as a person might pick one: the refusal says what
/// is wrong in words, never a parser's debug form or an internal limit's key.
#[test]
fn every_refusal_of_a_damaged_file_is_in_plain_words() {
    let npy = |descr: &str| {
        let mut header = format!("{{'descr': '{descr}', 'fortran_order': False, 'shape': (2,), }}");
        while (10 + header.len() + 1) % 64 != 0 {
            header.push(' ');
        }
        header.push('\n');
        let mut bytes = b"\x93NUMPY\x01\x00".to_vec();
        bytes.extend_from_slice(&u16::try_from(header.len()).unwrap().to_le_bytes());
        bytes.extend_from_slice(header.as_bytes());
        bytes.extend_from_slice(&[0; 16]);
        bytes
    };
    let npz = zip_bytes(&[("time.npy", &npy("<f8")), ("v.npy", &npy("<f8"))]);
    let bundle = native_bundle(ResultImportFormat::RSpiceResultBundle);
    let fst = generated_fst();
    let raw_header = "Title: t\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\n\
                      No. Points: 5\nVariables:\n\t0\ttime\ttime\n\t1\tV(a)\tvoltage\nBinary:\n";
    let with_tail = |head: &str, tail: &[u8]| [head.as_bytes(), tail].concat();
    let hostile: Vec<(&str, Vec<u8>)> = vec![
        ("table.parquet", b"PAR1 not a footer PAR1".to_vec()),
        ("table.arrow", b"ARROW1\0\0 not a schema".to_vec()),
        (
            "table.feather",
            b"\xff\xff\xff\xff\x10\0\0\0 not a message".to_vec(),
        ),
        ("objects.npy", npy("|O")),
        ("text.npy", npy("<U8")),
        ("dates.npy", npy("<M8[ns]")),
        ("cut.npy", npy("<f8")[..40].to_vec()),
        ("cut.npz", npz[..npz.len() - 20].to_vec()),
        ("cut.rspiceresult", bundle[..bundle.len() / 2].to_vec()),
        ("cut.fst", fst[..fst.len() / 2].to_vec()),
        ("cut.raw", with_tail(raw_header, &[0; 12])),
        ("cut.qraw", with_tail(raw_header, &[0; 12])),
        (
            "noise.raw",
            b"Title: t\nPlotname: x\nVariables:\n0 a\n".to_vec(),
        ),
        ("cut.tr0", b"0002000000000000    9601* title".to_vec()),
        (
            "letters.tr0",
            with_tail(
                &format!(
                    "{:<256}1       1       TIME            v(a             $&%#",
                    "00010001000000009601"
                ),
                b"0.10000E+01abcdefghijk",
            ),
        ),
        (
            "words.mt0",
            b"$DATA1 SOURCE='HSPICE'\n.TITLE 'x'\nalpha beta\none\n".to_vec(),
        ),
        (
            "bad.vcd",
            b"$timescale 1 ns $end\n$var wire 1 ! a $end\n$enddefinitions $end\n#0\nq!\n".to_vec(),
        ),
        (
            "bad.mat",
            [
                b"MATLAB 5.0 MAT-file".as_slice(),
                &[b' '; 109],
                &[0, 1, b'I', b'M'],
                &[0xff; 24],
            ]
            .concat(),
        ),
        (
            "bad.h5",
            [b"\x89HDF\r\n\x1a\n".as_slice(), &[0xff; 40]].concat(),
        ),
        (
            "bad.psfascii",
            b"HEADER\n\"PSFversion\" \"1.00\"\nSWEEP\n".to_vec(),
        ),
        ("bad.csv", b"time,V(out)\n0,abc\n1,2\n".to_vec()),
        ("bad.s2p", b"# Hz S MA R 50\n1 2 3\n".to_vec()),
        ("binary.txt", vec![0, 159, 146, 150, 0, 1, 2]),
    ];
    // What only a parser's debug form or a crate's internals would spell.
    let debug = [
        "::",
        "Error(",
        "Error {",
        "Some(",
        "None",
        "Callback",
        "external_data",
        "result_values",
        "Object0",
        "ZipCentral",
        "Utf8Error",
        "kind: ",
        "{:?}",
        "..=",
        "f64",
        "u64",
        "usize",
    ];
    for (name, bytes) in hostile {
        let error = match parse_result_dataset(name, &bytes) {
            Ok(_) => continue,
            Err(error) => error,
        };
        for marker in debug {
            assert!(!error.contains(marker), "{name}: '{marker}' in: {error}");
        }
        for format in ResultImportFormat::ALL {
            let id = format.canonical_id();
            assert!(
                !error.contains(&format!("{id} import")),
                "{name}: '{id}' in: {error}"
            );
        }
    }
}
