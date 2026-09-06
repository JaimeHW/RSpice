//! Cross-format result-import adapter tests and binary fixture builders.

use super::*;
use arrow_array::{ArrayRef, Float64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use npyz::WriterBuilder as _;
use std::io::Write as _;
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

#[test]
fn native_export_schema_is_deterministic_and_round_trips_real_and_complex() {
    use crate::workbench::workflows::native_result_bundle::{
        NativeBundleAnalysis, NativeBundleDataset, NativeBundleKind, NativeBundleSignal,
        NativeBundleSignalValues, encode_native_bundle,
    };

    let coordinate = [1.0e3, 2.0e3, 4.0e3];
    let real_values = [0.25, 0.5, 1.0];
    let complex_real = [1.0, -2.0, 0.5];
    let complex_imag = [0.125, 0.25, -0.75];
    let dataset = NativeBundleDataset {
        analysis: NativeBundleAnalysis::Ac,
        coordinate_name: "frequency",
        coordinate: &coordinate,
        signals: vec![
            NativeBundleSignal {
                name: "gain",
                unit: None,
                values: NativeBundleSignalValues::Real(&real_values),
            },
            NativeBundleSignal {
                name: "V(out)",
                unit: Some("V"),
                values: NativeBundleSignalValues::Complex {
                    real: &complex_real,
                    imag: &complex_imag,
                },
            },
        ],
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
        let bytes = encode_native_bundle(kind, &dataset).expect("native bundle encode");
        assert_eq!(
            bytes,
            encode_native_bundle(kind, &dataset).expect("repeat deterministic encode")
        );

        let mut archive = zip::ZipArchive::new(Cursor::new(bytes.as_slice())).unwrap();
        assert_eq!(archive.len(), 2);
        let manifest_bytes = read_zip_member(&mut archive, "manifest.json", format).unwrap();
        let dataset_bytes = read_zip_member(&mut archive, "dataset.json", format).unwrap();
        let manifest: serde_json::Value = serde_json::from_slice(&manifest_bytes).unwrap();
        let document: serde_json::Value = serde_json::from_slice(&dataset_bytes).unwrap();
        assert_eq!(manifest["schema"], kind.manifest_schema());
        assert_eq!(manifest["dataset_member"], "dataset.json");
        assert_eq!(document["schema"], "rspice-waveform-dataset/1");
        assert_eq!(document["analysis"], "ac");
        assert_eq!(document["signals"][0]["values"][1], 0.5);
        assert_eq!(document["signals"][1]["real"][1], -2.0);
        assert_eq!(document["signals"][1]["imag"][2], -0.75);
        use sha2::Digest as _;
        assert_eq!(
            manifest["dataset_sha256"],
            format!("{:x}", sha2::Sha256::digest(&dataset_bytes))
        );

        let parsed = parse_native_bundle(&bytes, format).expect("exporter/importer round-trip");
        assert_eq!(parsed.analysis_type, AnalysisType::Ac);
        assert_eq!(parsed.coordinate_name, "frequency");
        assert_eq!(parsed.waveforms.len(), 2);
        assert_eq!(parsed.waveforms[0].name, "gain");
        assert_eq!(parsed.waveforms[0].y.as_ref(), real_values.as_slice());
        let complex = parsed.waveforms[1]
            .complex
            .as_ref()
            .expect("complex identity retained");
        assert_eq!(complex.source_name, "V(out)");
        assert_eq!(complex.real.as_ref(), complex_real.as_slice());
        assert_eq!(complex.imag.as_ref(), complex_imag.as_slice());

        let mut tampered_manifest = manifest;
        tampered_manifest["dataset_sha256"] = serde_json::Value::String("00".repeat(32));
        let tampered_manifest = serde_json::to_vec(&tampered_manifest).unwrap();
        let tampered = zip_bytes(&[
            ("manifest.json", &tampered_manifest),
            ("dataset.json", &dataset_bytes),
        ]);
        let error = parse_native_bundle(&tampered, format).expect_err("digest tamper");
        assert!(error.contains("SHA-256"), "{error}");
    }
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
    let mut bytes = Vec::new();
    let mut writer = npyz::WriteOptions::new()
        .default_dtype()
        .shape(shape)
        .writer(&mut bytes)
        .begin_nd()
        .expect("NPY writer");
    writer.extend(values.iter().copied()).expect("NPY values");
    writer.finish().expect("NPY finish");
    bytes
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

#[test]
fn spice_raw_and_psf_ascii_import_real_waveforms() {
    let raw = b"Title: fixture\nDate: now\nPlotname: Transient Analysis\nFlags: real double\nNo. Variables: 2\nNo. Points: 3\nVariables:\n\t0\ttime\ttime\n\t1\tV(out)\tvoltage\nValues:\n0\t0\n\t1\n1\t1e-9\n\t2\n2\t2e-9\n\t3\n";
    assert_basic(
        parse_spice_raw(raw, ResultImportFormat::SpiceRaw).expect("SPICE RAW"),
        ResultImportFormat::SpiceRaw,
    );
    let psf = b"HEADER\n\"analysis\" \"tran\"\nSWEEP\n\"time\" \"s\"\nTRACE\n\"V(out)\" \"V\"\nVALUE\n0 1\n1e-9 2\n2e-9 3\nEND\n";
    assert_basic(
        parse_psf_ascii(psf, ResultImportFormat::PsfAscii).expect("PSF ASCII"),
        ResultImportFormat::PsfAscii,
    );
    assert!(parse_spice_raw(&raw[..40], ResultImportFormat::SpiceRaw).is_err());
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
    assert_eq!(parsed.waveforms[1].y.as_slice(), &[0.0, 1.0, 2.0]);
    assert_basic(parsed, ResultImportFormat::Vcd);
}

#[test]
fn vcd_imports_unknown_and_high_impedance_at_the_projection_level() {
    let vcd = b"$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! a $end\n$var wire 2 \" bus $end\n$upscope $end\n$enddefinitions $end\n#0\nx!\nb0x \"\n#1\n1!\nb01 \"\n#2\nz!\nb11 \"\n";
    let parsed = parse_vcd(vcd, ResultImportFormat::Vcd).expect("four-state VCD imports");
    assert_eq!(parsed.waveforms[0].y.as_slice(), &[0.5, 1.0, 0.5]);
    assert_eq!(
        parsed.waveforms[1].y.as_slice(),
        &[0.5, 1.0, 3.0],
        "a vector with any unknown bit denotes no integer"
    );
    let note = &parsed.notes[0];
    assert!(
        note.starts_with("3 sampled values were unknown (x) or high impedance (z)"),
        "unexpected note: {note}"
    );
    assert!(note.contains("0.5"), "unexpected note: {note}");
    assert!(
        note.contains("keeps the four-state code the file recorded"),
        "the level is the grid's decision, not a loss: {note}"
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
    assert_eq!(parsed.waveforms[0].y.as_slice(), &[0.0, 1.0]);
    assert!(
        parsed.notes.iter().any(
            |note| note.starts_with("1 vector variable is wider than 53 bits")
                && note.contains("one column per bit")
        ),
        "unexpected notes: {:?}",
        parsed.notes
    );
}

#[test]
fn vcd_aliases_share_one_timeline_under_their_own_names() {
    let vcd = b"$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! clk $end\n$var wire 1 ! clock $end\n$upscope $end\n$enddefinitions $end\n#0\n0!\n#5\n1!\n";
    let parsed = parse_vcd(vcd, ResultImportFormat::Vcd).expect("aliased VCD");
    assert_eq!(
        parsed
            .waveforms
            .iter()
            .map(|waveform| waveform.name.as_str())
            .collect::<Vec<_>>(),
        vec!["top.clk", "top.clock"]
    );
    assert_eq!(parsed.waveforms[0].y, parsed.waveforms[1].y);
    assert!(Arc::ptr_eq(&parsed.waveforms[0].y, &parsed.waveforms[1].y));
}

#[test]
fn vcd_keeps_the_changes_a_dumpoff_block_records() {
    let vcd = b"$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! a $end\n$upscope $end\n$enddefinitions $end\n#0\n$dumpvars\n0!\n$end\n#5\n1!\n#10\n$dumpoff\nx!\n$end\n";
    let parsed = parse_vcd(vcd, ResultImportFormat::Vcd).expect("VCD with a dump block");
    assert_eq!(
        parsed.waveforms[0].y.as_slice(),
        &[0.0, 1.0, 0.5],
        "a $dumpoff block records that the signals stopped being dumped"
    );
    assert_eq!(parsed.sample_count, 3);
}

fn generated_fst() -> Vec<u8> {
    use fst_writer::{
        FstFileType, FstInfo, FstScopeType, FstSignalType, FstVarDirection, FstVarType,
    };
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rspice-result-import-{}-{nonce}.fst",
        std::process::id()
    ));
    let info = FstInfo {
        start_time: 0,
        timescale_exponent: -9,
        version: "RSpice import fixture".to_owned(),
        date: "2026-08-31".to_owned(),
        file_type: FstFileType::Verilog,
    };
    let mut header = fst_writer::open_fst(&path, &info).expect("open FST fixture");
    header
        .scope("top", "top", FstScopeType::Module)
        .expect("FST scope");
    let clock = header
        .var(
            "clock",
            FstSignalType::bit_vec(1),
            FstVarType::Wire,
            FstVarDirection::Implicit,
            None,
        )
        .expect("FST signal");
    header.up_scope().expect("FST upscope");
    let mut body = header.finish().expect("FST header");
    for tick in 0..96_u64 {
        body.time_change(tick).expect("FST time change");
        body.signal_change(clock, if tick & 1 == 0 { b"0" } else { b"1" })
            .expect("FST signal change");
    }
    body.finish().expect("FST finish");
    let bytes = std::fs::read(&path).expect("read FST fixture");
    let _ = std::fs::remove_file(&path);
    bytes
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
    let mut header = vec![0_u8; (FST_HEADER_SECTION_BYTES - 8) as usize];
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
    data.extend_from_slice(&MAX_RESULT_DATASET_BYTES.to_be_bytes());
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
    assert!(looks_like_fst(&bytes));
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
    let too_large = MAX_RESULT_DATASET_BYTES + 1;

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
    let oversized = test_uleb(MAX_RESULT_DATASET_BYTES + 1);
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
    duo.extend(test_uleb(MAX_RESULT_DATASET_BYTES + 1));
    let duo = fst_test_block(7, &duo);
    let error = preflight_fst(&duo, ResultImportFormat::Fst).expect_err("LZ4 duo bomb");
    assert!(error.contains("LZ4-duo"), "{error}");
}

#[test]
fn fst_preflight_bounds_lz4_fastlz_and_zlib_signal_expansion() {
    for (name, pack_type) in [("LZ4", b'4'), ("FastLZ", b'F'), ("zlib", b'Z')] {
        let bytes = fst_with_packed_signal(pack_type, MAX_RESULT_DATASET_BYTES + 1);
        let error = preflight_fst(&bytes, ResultImportFormat::Fst)
            .expect_err("packed FST signal bomb must reject");
        assert!(error.contains(name), "{name}: {error}");
        assert!(error.contains("limit"), "{name}: {error}");
    }
}

/// The exact digital event evidence an import retained, for the tests that
/// read it: `(node, [(time_s, value_code)])` and the declarations over them.
fn imported_events(
    parsed: &ParsedResultDataset,
) -> (
    Vec<(String, Vec<(f64, u8)>)>,
    Vec<(String, i64, i64, Vec<String>)>,
) {
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
        parsed.waveforms[0].y.as_slice(),
        &[0.5, 0.5],
        "the grid still projects an unresolved bit at its own level"
    );
}
