use super::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_FFT_TEST: AtomicU64 = AtomicU64::new(0);

/// The canonical `fft-NNN` identities a deck with `count` authored `.FFT`
/// requests publishes under, minted the same way the executor mints them.
fn fft_test_identities(count: usize) -> Vec<String> {
    crate::commands::run::canonical_analysis_identities(
        rspice_core::execution::AnalysisKind::Fft,
        count,
    )
    .expect("canonical FFT identities")
    .iter()
    .map(|id| id.tag())
    .collect()
}

struct FftTestDirectory(PathBuf);

impl FftTestDirectory {
    fn new() -> Self {
        let id = NEXT_FFT_TEST.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("rspice-cli-fft-raw-{}-{id}", std::process::id()));
        std::fs::create_dir(&path).expect("create FFT RAW test directory");
        Self(path)
    }
}

impl Drop for FftTestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn fft_value_units_preserve_physical_type_and_transform_semantics() {
    use rspice_core::netlist::FftFormat::{Normalized, Unnormalized};

    assert_eq!(fft_value_unit("voltage", Normalized), Ok(Some("1")));
    assert_eq!(fft_value_unit("current", Normalized), Ok(Some("1")));
    assert_eq!(fft_value_unit("parameter", Normalized), Ok(Some("1")));
    assert_eq!(fft_value_unit("voltage", Unnormalized), Ok(Some("V")));
    assert_eq!(fft_value_unit("current", Unnormalized), Ok(Some("A")));
    assert_eq!(fft_value_unit("parameter", Unnormalized), Ok(None));
    assert!(fft_value_unit("unsupported", Normalized).is_err());
}

fn fft_publication_fixture() -> (
    rspice_core::Netlist,
    Vec<rspice_core::engine::TransientFftResult>,
) {
    let netlist = rspice_core::Netlist::parse(
        "typed FFT publication validation\n\
         V1 out 0 SIN(0 1 3k)\n\
         R1 out 0 1k\n\
         .options fft fftout=1\n\
         .tran 1u 1m\n\
         .fft v(out) np=8 format=unorm window=rect freq=3k\n\
         .fft {2*v(out)} np=16 window=hann freq=3k fmin=1k\n\
         .end\n",
    )
    .expect("parse FFT publication fixture");
    let transient = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
        .run_tran_with_abort(&netlist, 1.0e-3, 1.0e-6, &rspice_core::NoAbort)
        .expect("run FFT publication fixture");
    (netlist, transient.fft_results)
}

#[test]
fn every_fft_json_float_has_a_matching_precision_checked_wire_path() {
    use rspice_core::io::json::NumericJsonDocument;

    fn floats(value: &serde_json::Value, path: &str, output: &mut Vec<String>) {
        match value {
            serde_json::Value::Number(number) if number.is_f64() => output.push(path.to_owned()),
            serde_json::Value::Array(values) => {
                for (index, value) in values.iter().enumerate() {
                    floats(value, &format!("{path}/{index}"), output);
                }
            }
            serde_json::Value::Object(values) => {
                for (key, value) in values {
                    let key = key.replace('~', "~0").replace('/', "~1");
                    floats(value, &format!("{path}/{key}"), output);
                }
            }
            _ => {}
        }
    }

    let (netlist, original) = fft_publication_fixture();
    let directory = FftTestDirectory::new();
    let path = directory.0.join("fft.json");
    let mut checked = 0;
    for incomplete in [false, true] {
        let mut results = original.clone();
        if incomplete {
            results[0].status = rspice_core::engine::TransientFftStatus::IncompleteHistory {
                available_start: 0.0,
                available_stop: 0.1e-3,
            };
            results[0].bins.clear();
            results[0].metrics = None;
        }
        write_fft_output(
            &path,
            OutputFormat::Json,
            "tran-001",
            &fft_test_identities(2),
            None,
            &results,
            &netlist,
            None,
        )
        .unwrap();
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let raw = fft_raw_metadata(
            OutputFormat::Raw,
            "tran-001",
            &fft_test_identities(2),
            None,
            &results,
            &netlist.fft_analyses,
        )
        .unwrap();
        for (is_raw, value) in [(false, json), (true, serde_json::to_value(raw).unwrap())] {
            let mut paths = Vec::new();
            floats(&value, "", &mut paths);
            assert!(!paths.is_empty());
            for pointer in paths {
                for (literal, message) in [
                    ("9007199254740993", "cannot be represented exactly"),
                    ("1e-999", "underflow"),
                ] {
                    let mut corrupted = value.clone();
                    *corrupted.pointer_mut(&pointer).unwrap() =
                        serde_json::json!("numeric-test-marker");
                    let text = serde_json::to_string(&corrupted)
                        .unwrap()
                        .replace("\"numeric-test-marker\"", literal);
                    let error = if is_raw {
                        FftRawMetadata::decode_numeric_json(&text, &rspice_core::NoAbort)
                            .unwrap_err()
                            .to_string()
                    } else {
                        FftBundle::from_json(
                            &path,
                            &text,
                            serde_json::from_str(&text).unwrap(),
                            rspice_core::ResourceLimits::default(),
                        )
                        .err()
                        .expect("precision refusal")
                        .to_string()
                    };
                    assert!(error.contains(message), "{pointer}: {error}");
                    checked += 1;
                }
            }
        }
    }
    assert!(
        checked > 400,
        "checked {checked} field/literal combinations"
    );
}

#[test]
fn fft_decimal_underflow_is_refused_in_metadata_bins_and_harmonics() {
    use crate::commands::waveform_io::parse_delimited_record;
    let (netlist, original) = fft_publication_fixture();
    let directory = FftTestDirectory::new();
    for (format, separator) in [(OutputFormat::Csv, ','), (OutputFormat::Tsv, '\t')] {
        for incomplete in [false, true] {
            let path = directory.0.join("source");
            let mut results = original.clone();
            if incomplete {
                results[0].status = rspice_core::engine::TransientFftStatus::IncompleteHistory {
                    available_start: 0.0,
                    available_stop: 0.1e-3,
                };
                results[0].bins.clear();
                results[0].metrics = None;
            }
            write_fft_output(
                &path,
                format,
                "tran-001",
                &fft_test_identities(2),
                None,
                &results,
                &netlist,
                None,
            )
            .unwrap();
            let text = std::fs::read_to_string(&path).unwrap();
            FftBundle::from_delimited(
                &path,
                &text,
                separator,
                rspice_core::ResourceLimits::default(),
            )
            .unwrap();
            let rows = text
                .lines()
                .map(|row| parse_delimited_record(row, separator).unwrap())
                .collect::<Vec<_>>();
            let fields: &[&str] = if incomplete {
                &["available_start_s", "available_stop_s"]
            } else {
                &[
                    "start_time_s",
                    "alpha",
                    "frequency_hz",
                    "imaginary",
                    "thd_db",
                    "sfdr_spur_frequency_hz",
                    "harmonic_phase_degrees",
                ]
            };
            for field in fields {
                let column = rows[0].iter().position(|name| name == field).unwrap();
                let mut corrupted = rows.clone();
                let mut modified = false;
                for row in corrupted.iter_mut().skip(1) {
                    if !row[column].is_empty() {
                        row[column] = "1e-999".into();
                        modified = true;
                        // Repeated metadata must remain consistent across rows;
                        // sample and harmonic fields need only one bad value.
                        if column >= 44 {
                            break;
                        }
                    }
                }
                assert!(modified, "populated numeric field {field}");
                let mut writer = csv::WriterBuilder::new()
                    .delimiter(separator as u8)
                    .from_writer(Vec::new());
                for row in corrupted {
                    writer.write_record(row).unwrap();
                }
                let corrupted = String::from_utf8(writer.into_inner().unwrap()).unwrap();
                let error = FftBundle::from_delimited(
                    &path,
                    &corrupted,
                    separator,
                    rspice_core::ResourceLimits::default(),
                )
                .err()
                .expect("underflow refusal");
                assert!(
                    error.to_string().contains("underflow") && error.to_string().contains(field),
                    "{field}: {error}"
                );
            }
        }
    }
}

fn assert_fft_publication_rejected_for_every_format(
    directory: &FftTestDirectory,
    label: &str,
    results: &[rspice_core::engine::TransientFftResult],
    netlist: &rspice_core::Netlist,
) {
    for (format, extension) in [
        (OutputFormat::Json, "json"),
        (OutputFormat::Csv, "csv"),
        (OutputFormat::Tsv, "tsv"),
        (OutputFormat::Raw, "raw"),
        (OutputFormat::RawAscii, "ascii.raw"),
        (OutputFormat::Hdf5, "h5"),
    ] {
        let path = directory.0.join(format!("{label}.{extension}"));
        write_fft_output(
            &path,
            format,
            "tran-001",
            &fft_test_identities(netlist.fft_analyses.len()),
            None,
            results,
            netlist,
            None,
        )
        .expect_err("malformed FFT publication must fail closed");
        assert!(!path.exists(), "malformed {format:?} FFT was published");
    }
    let entries = std::fs::read_dir(&directory.0)
        .expect("read rejected FFT publication directory")
        .map(|entry| entry.expect("read rejected FFT entry").file_name())
        .collect::<Vec<_>>();
    assert!(
        entries.iter().all(|name| !name
            .to_string_lossy()
            .contains(rspice_output::STAGING_MARKER)),
        "rejected FFT publication left a staging artifact: {entries:?}"
    );
}

#[test]
fn fft_publication_rejects_count_identity_and_core_invariant_corruption_in_every_format() {
    let (netlist, results) = fft_publication_fixture();
    let directory = FftTestDirectory::new();

    assert!(validate_fft_result_count(&[], &netlist.fft_analyses).is_err());
    assert_fft_publication_rejected_for_every_format(&directory, "missing-results", &[], &netlist);

    let mut reordered = results.clone();
    reordered.swap(0, 1);
    assert_fft_publication_rejected_for_every_format(&directory, "reordered", &reordered, &netlist);

    let mut wrong_mode = results.clone();
    wrong_mode[0].mode = rspice_core::netlist::XyceFftMode::SpectreCompatible;
    assert_fft_publication_rejected_for_every_format(
        &directory,
        "wrong-mode",
        &wrong_mode,
        &netlist,
    );

    let mut wrong_sampling_policy = results.clone();
    wrong_sampling_policy[0].accurate_sampling = false;
    assert_fft_publication_rejected_for_every_format(
        &directory,
        "wrong-sampling-policy",
        &wrong_sampling_policy,
        &netlist,
    );

    let mut missing_metrics = results.clone();
    missing_metrics[0].metrics = None;
    assert_fft_publication_rejected_for_every_format(
        &directory,
        "missing-metrics",
        &missing_metrics,
        &netlist,
    );

    let transient_path = directory.0.join("reordered.tran.json");
    let fft_path = directory.0.join("reordered.fft.json");
    let transient = TransientOutputDocument::Table {
        table: Box::new(crate::commands::export_table::ExportTable {
            scale_unit: None,
            analysis: "transient".to_string(),
            plot_name: "Transient Analysis".to_string(),
            scale_name: "time".to_string(),
            scale_type: "time".to_string(),
            scale: vec![0.0],
            columns: Vec::new(),
        }),
        events: Vec::new(),
        buses: Vec::new(),
    };
    write_transient_fft_output_pair(
        &transient_path,
        &transient,
        &fft_path,
        OutputFormat::Json,
        "tran-001",
        &fft_test_identities(netlist.fft_analyses.len()),
        None,
        &reordered,
        &netlist,
        None,
        u64::MAX,
    )
    .expect_err("malformed FFT pair must fail before staging either sibling");
    assert!(!transient_path.exists());
    assert!(!fft_path.exists());

    let mut impossible_results = results.clone();
    let mut impossible_requests = netlist.fft_analyses.clone();
    impossible_requests[0].fundamental_frequency = Some(2.0e3);
    impossible_requests[0].minimum_frequency = Some(0.0);
    impossible_requests[0].maximum_frequency = Some(1.0);
    impossible_results[0].fundamental_bin = 2;
    impossible_results[0].minimum_metric_bin = 0;
    impossible_results[0].maximum_metric_bin = 0;
    impossible_results[0].metrics = None;
    let mut impossible_netlist = netlist.clone();
    impossible_netlist.fft_analyses = impossible_requests;
    assert_fft_publication_rejected_for_every_format(
        &directory,
        "impossible-bounds",
        &impossible_results,
        &impossible_netlist,
    );

    let mut not_normalized = results.clone();
    not_normalized[1].metrics = None;
    let peak = not_normalized[1]
        .bins
        .iter()
        .enumerate()
        .max_by(|(_, left), (_, right)| left.magnitude.total_cmp(&right.magnitude))
        .map(|(index, _)| index)
        .expect("normalized FFT has a bin");
    not_normalized[1].bins[peak].real *= 0.5;
    not_normalized[1].bins[peak].imaginary *= 0.5;
    not_normalized[1].bins[peak].magnitude *= 0.5;
    assert_fft_publication_rejected_for_every_format(
        &directory,
        "not-normalized",
        &not_normalized,
        &netlist,
    );

    let mut stale_metrics = results;
    stale_metrics[0]
        .metrics
        .as_mut()
        .expect("FFTOUT fixture")
        .thd_ratio += 0.25;
    assert_fft_publication_rejected_for_every_format(
        &directory,
        "stale-metrics",
        &stale_metrics,
        &netlist,
    );
}

#[test]
fn incomplete_fft_requests_remain_visible_in_every_publication_format() {
    use rspice_core::engine::TransientFftStatus;
    let (netlist, reference) = fft_publication_fixture();
    let directory = FftTestDirectory::new();
    let incomplete = TransientFftStatus::IncompleteHistory {
        available_start: 0.0,
        available_stop: 0.1e-3,
    };
    for all_incomplete in [false, true] {
        let mut results = reference.clone();
        for (index, result) in results.iter_mut().enumerate() {
            if index == 0 || all_incomplete {
                result.status = incomplete;
                result.bins.clear();
                result.metrics = None;
            }
        }
        for (index, format) in [
            OutputFormat::Json,
            OutputFormat::Csv,
            OutputFormat::Tsv,
            OutputFormat::Raw,
            OutputFormat::RawAscii,
            OutputFormat::Hdf5,
        ]
        .into_iter()
        .enumerate()
        {
            let path = directory
                .0
                .join(format!("incomplete_{all_incomplete}_{index}"));
            write_fft_output(
                &path,
                format,
                "tran-001",
                &fft_test_identities(results.len()),
                None,
                &results,
                &netlist,
                None,
            )
            .unwrap();
            match format {
                OutputFormat::Json => {
                    let document: serde_json::Value =
                        serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
                    assert_eq!(document["schema_version"], FFT_ARTIFACT_SCHEMA_VERSION);
                    assert_eq!(
                        document["results"][0]["status"]["kind"],
                        "incomplete-history"
                    );
                    assert!(
                        document["results"][0]["spectrum"]["bins"]
                            .as_array()
                            .unwrap()
                            .is_empty()
                    );
                }
                OutputFormat::Csv | OutputFormat::Tsv => {
                    let text = std::fs::read_to_string(&path).unwrap();
                    assert_eq!(
                        text.lines()
                            .filter(|line| line.contains("unavailable"))
                            .count(),
                        if all_incomplete { 2 } else { 1 }
                    );
                    assert!(text.contains("incomplete-history"));
                }
                OutputFormat::Raw | OutputFormat::RawAscii => {
                    let decoded = read_fft_raw_artifact(&path).unwrap();
                    assert_eq!(decoded.metadata.results[0].status, incomplete);
                    assert_eq!(decoded.metadata.results[1].status, results[1].status);
                    assert_eq!(decoded.bins.len(), results[1].bins.len());
                }
                OutputFormat::Hdf5 => {
                    let document = crate::hdf5::read_hdf5(&path).unwrap();
                    let fft = document.fft.unwrap();
                    assert_eq!(fft.results[0].status, incomplete);
                    assert_eq!(fft.results[1].status, results[1].status);
                    assert!(fft.results[0].real.is_empty());
                }
                OutputFormat::Vcd => unreachable!(),
            }
        }
    }
}

#[test]
fn binary_and_ascii_fft_raw_artifacts_round_trip_typed_metadata_and_ragged_bins() {
    let netlist = rspice_core::Netlist::parse(
        "typed FFT RAW round trip\n\
         V1 out 0 SIN(0 1 3k)\n\
         R1 out 0 1k\n\
         .options fft fftout=1\n\
         .tran 1u 1m\n\
         .fft v(out) np=8 format=unorm window=rect freq=3k\n\
         .fft {2*v(out)} np=16 window=hann freq=3k fmin=1k\n\
         .end\n",
    )
    .expect("parse typed FFT RAW test deck");
    let transient = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
        .run_tran_with_abort(&netlist, 1.0e-3, 1.0e-6, &rspice_core::NoAbort)
        .expect("run typed FFT RAW test deck");
    assert_eq!(transient.fft_results.len(), 2);
    let directory = FftTestDirectory::new();

    for (format, extension, selected) in [
        (OutputFormat::Raw, "raw", "raw"),
        (OutputFormat::RawAscii, "ascii.raw", "ascii"),
    ] {
        let path = directory.0.join(format!("fft.{extension}"));
        write_fft_output(
            &path,
            format,
            "tran-007",
            &fft_test_identities(netlist.fft_analyses.len()),
            None,
            &transient.fft_results,
            &netlist,
            None,
        )
        .expect("write typed FFT RAW artifact");
        let decoded = read_fft_raw_artifact(&path).expect("decode typed FFT RAW artifact");
        assert_eq!(decoded.metadata.selected_format, selected);
        assert_eq!(decoded.metadata.parent_analysis_id, "tran-007");
        assert_eq!(decoded.metadata.result_count, 2);
        assert_eq!(decoded.metadata.frequency_unit, "Hz");
        assert_eq!(decoded.metadata.phase_unit, "degree");
        assert_eq!(decoded.metadata.complex_representation, "cartesian");
        assert_eq!(decoded.metadata.results[0].analysis_id, "fft-001");
        assert_eq!(decoded.metadata.results[1].analysis_id, "fft-002");
        assert_eq!(decoded.metadata.results[0].signal.physical_type, "voltage");
        assert_eq!(
            decoded.metadata.results[0].signal.unit.as_deref(),
            Some("V")
        );
        assert_eq!(
            decoded.metadata.results[1].signal.physical_type,
            "parameter"
        );
        assert_eq!(
            decoded.metadata.results[1].signal.unit.as_deref(),
            Some("1")
        );
        assert_eq!(
            decoded.metadata.results[0]
                .transform
                .sfdr_search_minimum_bin,
            decoded.metadata.results[0].transform.fundamental_bin
        );
        assert_eq!(
            decoded.metadata.results[1]
                .transform
                .sfdr_search_minimum_bin,
            decoded.metadata.results[1].transform.minimum_metric_bin
        );
        assert_ne!(
            decoded.metadata.results[1].transform.fundamental_bin,
            decoded.metadata.results[1].transform.minimum_metric_bin
        );
        assert!(decoded.metadata.results[0].metrics.is_some());
        assert!(decoded.metadata.results[1].metrics.is_some());
        assert_eq!(decoded.bins[0].analysis_id, "fft-001");
        let second_start = transient.fft_results[0].bins.len();
        assert_eq!(decoded.bins[second_start].analysis_id, "fft-002");
        assert_eq!(decoded.bins[second_start].index, 0);
        assert_eq!(decoded.bins[second_start].frequency_hz, 0.0);
        assert_eq!(
            decoded.bins.len(),
            transient
                .fft_results
                .iter()
                .map(|result| result.bins.len())
                .sum::<usize>()
        );
        assert!((decoded.bins[1].real - transient.fft_results[0].bins[1].real).abs() < 1e-14);
        assert!(
            (decoded.bins[1].imaginary - transient.fft_results[0].bins[1].imaginary).abs() < 1e-14
        );
        assert!(
            (decoded.bins[1].magnitude - transient.fft_results[0].bins[1].magnitude).abs() < 1e-14
        );
        assert!(
            (decoded.bins[1].phase_degrees - transient.fft_results[0].bins[1].phase_degrees).abs()
                < 1e-14
        );
        let entries = std::fs::read_dir(&directory.0)
            .expect("read FFT RAW test directory")
            .map(|entry| entry.expect("read FFT RAW entry").file_name())
            .collect::<Vec<_>>();
        assert!(
            entries.iter().all(|name| !name
                .to_string_lossy()
                .contains(rspice_output::STAGING_MARKER)),
            "atomic FFT RAW staging artifact remained: {entries:?}"
        );

        if matches!(format, OutputFormat::Raw) {
            let mut old = decoded.metadata.clone();
            old.schema_version = 1;
            assert!(validate_fft_raw_metadata(&old).is_err());

            let mut future = decoded.metadata.clone();
            future.schema_version = FFT_ARTIFACT_SCHEMA_VERSION + 1;
            assert!(validate_fft_raw_metadata(&future).is_err());

            let mut malformed = decoded.metadata.clone();
            malformed.results[0].analysis_id = "fft-999".to_string();
            assert!(validate_fft_raw_metadata(&malformed).is_err());

            let mut unknown_enum = decoded.metadata.clone();
            unknown_enum.results[0].transform.window = "future_window".to_string();
            assert!(validate_fft_raw_metadata(&unknown_enum).is_err());

            let mut inconsistent_source = decoded.metadata.clone();
            inconsistent_source.results[0].source.kind = "expression".to_string();
            assert!(validate_fft_raw_metadata(&inconsistent_source).is_err());

            let mut impossible_bounds = decoded.metadata.clone();
            impossible_bounds.results[0].transform.fundamental_bin = 2;
            impossible_bounds.results[0].transform.minimum_metric_bin = 0;
            impossible_bounds.results[0].transform.maximum_metric_bin = 0;
            impossible_bounds.results[0]
                .transform
                .sfdr_search_minimum_bin = 0;
            impossible_bounds.results[0].metrics = None;
            assert!(validate_fft_raw_metadata(&impossible_bounds).is_err());

            let first_bin_count = decoded.metadata.results[0].sampling.point_count / 2 + 1;
            let first_bins = &decoded.bins[..first_bin_count];
            let mut wrong_thd = decoded.metadata.results[0].clone();
            wrong_thd
                .metrics
                .as_mut()
                .expect("metric fixture")
                .thd_ratio += 0.25;
            assert!(validate_fft_raw_metrics_against_bins(&wrong_thd, first_bins).is_err());

            let mut wrong_spur = decoded.metadata.results[0].clone();
            let metrics = wrong_spur.metrics.as_mut().expect("metric fixture");
            metrics.sfdr_spur_bin = Some(2);
            metrics.sfdr_spur_frequency_hz =
                Some(2.0 * wrong_spur.transform.frequency_resolution_hz);
            assert!(validate_fft_raw_metrics_against_bins(&wrong_spur, first_bins).is_err());

            let mut wrong_harmonic = decoded.metadata.results[0].clone();
            wrong_harmonic
                .metrics
                .as_mut()
                .expect("metric fixture")
                .largest_harmonics[0]
                .magnitude += 1.0e-6;
            assert!(validate_fft_raw_metrics_against_bins(&wrong_harmonic, first_bins).is_err());

            let original = std::fs::read(&path).expect("read binary FFT RAW bytes");
            let mut corrupt_schema = original.clone();
            let schema_offset = corrupt_schema
                .windows(b"fft_real\tvalue".len())
                .position(|window| window == b"fft_real\tvalue")
                .expect("find FFT RAW variable type")
                + b"fft_real\t".len();
            corrupt_schema[schema_offset..schema_offset + 5].copy_from_slice(b"bogus");
            std::fs::write(&path, &corrupt_schema).expect("write corrupt variable schema");
            assert!(read_fft_raw_artifact(&path).is_err());

            let mut non_finite = original.clone();
            let binary_offset = non_finite
                .windows(b"Binary:\n".len())
                .position(|window| window == b"Binary:\n")
                .expect("find FFT RAW binary payload")
                + b"Binary:\n".len();
            non_finite[binary_offset..binary_offset + 8].copy_from_slice(&f64::NAN.to_le_bytes());
            std::fs::write(&path, &non_finite).expect("write non-finite FFT RAW row");
            assert!(read_fft_raw_artifact(&path).is_err());

            let mut wrong_phase = original.clone();
            let second_row_phase = binary_offset + 7 * 8 + 4 * 8;
            wrong_phase[second_row_phase..second_row_phase + 8]
                .copy_from_slice(&123.0_f64.to_le_bytes());
            std::fs::write(&path, &wrong_phase).expect("write inconsistent FFT RAW phase");
            assert!(read_fft_raw_artifact(&path).is_err());

            let mut not_normalized = original.clone();
            let normalized_peak = decoded.bins[second_start..]
                .iter()
                .enumerate()
                .max_by(|(_, left), (_, right)| left.magnitude.total_cmp(&right.magnitude))
                .map(|(index, _)| second_start + index)
                .expect("normalized FFT has bins");
            for (column, value) in [
                (1, decoded.bins[normalized_peak].real * 0.5),
                (2, decoded.bins[normalized_peak].imaginary * 0.5),
                (3, decoded.bins[normalized_peak].magnitude * 0.5),
            ] {
                let offset = binary_offset + (normalized_peak * 7 + column) * 8;
                not_normalized[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
            }
            std::fs::write(&path, &not_normalized).expect("write non-normalized FFT RAW spectrum");
            assert!(read_fft_raw_artifact(&path).is_err());

            let mut negative_sub_pico = original.clone();
            let first_row_magnitude = binary_offset + 3 * 8;
            negative_sub_pico[first_row_magnitude..first_row_magnitude + 8]
                .copy_from_slice(&(-1.0e-300_f64).to_le_bytes());
            std::fs::write(&path, &negative_sub_pico)
                .expect("write negative sub-pico FFT RAW magnitude");
            assert!(read_fft_raw_artifact(&path).is_err());
            std::fs::write(&path, original).expect("restore binary FFT RAW fixture");
        }
    }

    let hdf5_path = directory.0.join("fft.h5");
    write_fft_output(
        &hdf5_path,
        OutputFormat::Hdf5,
        "tran-007",
        &fft_test_identities(netlist.fft_analyses.len()),
        None,
        &transient.fft_results,
        &netlist,
        None,
    )
    .expect("write typed FFT HDF5 artifact");
    let hdf5 = crate::hdf5::read_hdf5(&hdf5_path).expect("decode typed FFT HDF5 artifact");
    let hdf5_fft = hdf5.fft.expect("typed FFT HDF5 section");
    assert_eq!(hdf5_fft.results[0].physical_type, "voltage");
    assert_eq!(hdf5_fft.results[0].value_unit.as_deref(), Some("V"));
    assert_eq!(hdf5_fft.results[1].physical_type, "parameter");
    assert_eq!(hdf5_fft.results[1].value_unit.as_deref(), Some("1"));
    assert_eq!(
        hdf5_fft.results[0].sfdr_search_minimum_bin,
        hdf5_fft.results[0].fundamental_bin
    );
    assert_eq!(
        hdf5_fft.results[1].sfdr_search_minimum_bin,
        hdf5_fft.results[1].minimum_metric_bin
    );
}
