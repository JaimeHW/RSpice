use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

#[test]
fn legacy_empty_text_is_readable_without_accepting_nonempty_string_arrays() {
    let mut builder = rustyhdf5::FileBuilder::new();
    builder.set_attr("title", AttrValue::String(String::new()));
    builder.set_attr(
        "array",
        AttrValue::StringArray(vec!["one".into(), "two".into()]),
    );
    let file = Hdf5File::from_bytes(builder.finish().unwrap()).unwrap();
    let attrs = file.root().attrs().unwrap();
    assert_eq!(
        read_string_attr(&attrs, "title").unwrap(),
        Some(String::new())
    );
    assert!(read_string_attr(&attrs, "array").is_err());
}

struct TestDirectory(std::path::PathBuf);

impl TestDirectory {
    fn new(tag: &str) -> Self {
        let id = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice-hdf5-atomic-{}-{id}-{tag}",
            std::process::id()
        ));
        std::fs::create_dir(&path).expect("create unique HDF5 test directory");
        Self(path)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn assert_only_destination_remains(directory: &Path, destination: &Path, exists: bool) {
    let entries: Vec<std::path::PathBuf> = std::fs::read_dir(directory)
        .expect("read HDF5 test directory")
        .map(|entry| entry.expect("read HDF5 directory entry").path())
        .collect();
    if exists {
        assert_eq!(entries, vec![destination.to_path_buf()]);
    } else {
        assert!(
            entries.is_empty(),
            "unexpected staged artifacts: {entries:?}"
        );
    }
}

#[test]
fn backend_serialization_failure_preserves_old_or_absent_destination() {
    for preexisting in [false, true] {
        let directory = TestDirectory::new("backend-failure");
        let destination = directory.0.join("result.h5");
        if preexisting {
            std::fs::write(&destination, b"old complete HDF5 artifact")
                .expect("seed existing HDF5 destination");
        }

        let mut incomplete_builder = rustyhdf5::FileBuilder::new();
        incomplete_builder.create_dataset("missing_data");
        let error = write_hdf5_staged(&destination, |file| {
            let bytes = incomplete_builder.finish()?;
            file.write_all(&bytes)?;
            Ok(())
        })
        .expect_err("incomplete dataset must fail serialization");
        assert!(matches!(error, Hdf5Error::Backend(_)));

        if preexisting {
            assert_eq!(
                std::fs::read(&destination).expect("read preserved HDF5 destination"),
                b"old complete HDF5 artifact"
            );
        } else {
            assert!(!destination.exists());
        }
        assert_only_destination_remains(&directory.0, &destination, preexisting);
    }
}

#[test]
fn nonfinite_measurement_writes_preserve_existing_artifacts() {
    let directory = TestDirectory::new("nonfinite-measurement");
    let destination = directory.0.join("result.h5");
    std::fs::write(&destination, b"complete predecessor").unwrap();
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let data = Hdf5SimulationData {
            measurements: vec![Hdf5Measurement::new("delay", value)],
            ..Hdf5SimulationData::default()
        };
        let error = write_hdf5(&destination, &data).unwrap_err();
        assert!(matches!(error, Hdf5Error::InvalidSchema(_)));
        assert!(error.to_string().contains("measurement 'delay'"));
        assert_eq!(
            std::fs::read(&destination).unwrap(),
            b"complete predecessor"
        );
        assert_only_destination_remains(&directory.0, &destination, true);
    }
}

#[test]
fn successful_hdf5_write_atomically_replaces_existing_bytes() {
    let directory = TestDirectory::new("success");
    let destination = directory.0.join("result.h5");
    std::fs::write(&destination, b"old complete HDF5 artifact")
        .expect("seed existing HDF5 destination");

    let mut data = Hdf5SimulationData::new();
    data.title = "atomic result".to_string();
    let mut transient = Hdf5WaveformSection::new("time", vec![0.0, 1.0]);
    transient.add_signal("V(out)", vec![0.0, 2.0]);
    data.transient = Some(transient);
    write_hdf5(&destination, &data).expect("write complete HDF5 destination");

    assert_eq!(
        read_hdf5(&destination).expect("read committed HDF5 destination"),
        data
    );
    assert_only_destination_remains(&directory.0, &destination, true);
}

/// The byte-identity oracle for the waveform and AC layout after it moved
/// to `rspice_core::io::hdf5`.
///
/// This spells the `FileBuilder` calls the layout makes, in the order it
/// makes them, and requires the same file back. A layout change that a
/// reader would notice fails here first, before any round trip can hide it
/// by agreeing with itself.
///
/// Re-blessed once, deliberately, when the CLI started stating units: a
/// column whose producer named a quantity now writes `signal_NNNN_unit`
/// after `signal_NNNN_type`, and `I(R1)` — added through the untyped
/// `add_signal`, which names no quantity — still writes none, so the
/// difference between *stated* and *unstated* is in these bytes too.
#[test]
fn the_moved_layout_writes_the_bytes_this_module_used_to_write() {
    let mut data = Hdf5SimulationData::new();
    data.title = "byte identity".to_string();
    let mut transient = Hdf5WaveformSection::new("time", vec![0.0, 1.0, 2.0]);
    transient.add_typed_signal(
        "V(out)",
        "voltage",
        Some("V".to_string()),
        vec![0.0, 2.0, 4.0],
    );
    transient.add_signal("I(R1)", vec![1.0, 2.0, 3.0]);
    data.transient = Some(transient);
    let mut ac = Hdf5AcSection::new(vec![1.0, 10.0]);
    ac.add_signal(
        "V(out)",
        Some("V".to_string()),
        vec![1.0, 0.5],
        vec![0.0, -0.5],
    );
    data.ac = Some(ac);
    data.measurements = vec![Hdf5Measurement::new("rise", 1.5e-9)];

    let mut moved = Vec::new();
    write_hdf5_to_writer(&mut moved, &data).expect("the moved writer publishes");

    let mut legacy = rustyhdf5::FileBuilder::new();
    legacy.set_attr("schema_version", AttrValue::String("1".to_string()));
    legacy.set_attr("simulator", AttrValue::String("RSpice".to_string()));
    legacy.set_attr("title", AttrValue::String("byte identity".to_string()));

    let mut group = legacy.create_group("transient");
    group.set_attr("section_type", AttrValue::String("transient".to_string()));
    group.set_attr("independent_name", AttrValue::String("time".to_string()));
    group.set_attr("signal_count", AttrValue::I64(2));
    group
        .create_dataset("independent")
        .with_f64_data(&[0.0, 1.0, 2.0]);
    group.set_attr("signal_0000_name", AttrValue::String("V(out)".to_string()));
    group.set_attr("signal_0000_type", AttrValue::String("voltage".to_string()));
    group.set_attr("signal_0000_unit", AttrValue::String("V".to_string()));
    group
        .create_dataset("signal_0000")
        .with_f64_data(&[0.0, 2.0, 4.0]);
    group.set_attr("signal_0001_name", AttrValue::String("I(R1)".to_string()));
    group.set_attr("signal_0001_type", AttrValue::String("value".to_string()));
    group
        .create_dataset("signal_0001")
        .with_f64_data(&[1.0, 2.0, 3.0]);
    legacy.add_group(group.finish());

    let mut group = legacy.create_group("ac");
    group.set_attr("section_type", AttrValue::String("ac".to_string()));
    group.set_attr("signal_count", AttrValue::I64(1));
    group
        .create_dataset("frequency")
        .with_f64_data(&[1.0, 10.0]);
    group.set_attr("signal_0000_name", AttrValue::String("V(out)".to_string()));
    group.set_attr("signal_0000_unit", AttrValue::String("V".to_string()));
    group
        .create_dataset("signal_0000_real")
        .with_f64_data(&[1.0, 0.5]);
    group
        .create_dataset("signal_0000_imag")
        .with_f64_data(&[0.0, -0.5]);
    legacy.add_group(group.finish());

    let mut group = legacy.create_group("measurements");
    group.set_attr("measurement_count", AttrValue::I64(1));
    group.set_attr(
        "measurement_0000_name",
        AttrValue::String("rise".to_string()),
    );
    group.set_attr("measurement_0000_value", AttrValue::F64(1.5e-9));
    legacy.add_group(group.finish());

    assert_eq!(
        moved,
        legacy.finish().expect("the old call order encodes"),
        "the moved layout changed the published bytes"
    );
}

fn fft_result(
    ordinal: usize,
    point_count: usize,
    physical_type: &str,
    value_unit: Option<&str>,
    with_metrics: bool,
) -> Hdf5FftResult {
    let bin_count = point_count / 2 + 1;
    let frequency_hz = (0..bin_count).map(|bin| bin as f64).collect::<Vec<_>>();
    let real = (0..bin_count)
        .map(|bin| if bin == 1 { 1.0 } else { 0.0 })
        .collect::<Vec<_>>();
    let imaginary = vec![0.0; bin_count];
    let magnitude = real.clone();
    let phase_degrees = vec![0.0; bin_count];
    let fundamental_bin = 1;
    let maximum_metric_bin = bin_count - 1;
    let sfdr_search_minimum_bin = fundamental_bin;
    let expected_metrics = fft_metric_expectations(
        &magnitude,
        fundamental_bin,
        maximum_metric_bin,
        sfdr_search_minimum_bin,
    )
    .expect("valid FFT metric fixture");
    let sfdr_spur_frequency_hz = expected_metrics.sfdr_spur_bin.map(|bin| frequency_hz[bin]);
    let largest_harmonics = expected_metrics
        .ranked_bins
        .iter()
        .copied()
        .enumerate()
        .map(|(index, bin)| Hdf5FftHarmonic {
            rank: index + 1,
            bin,
            frequency_hz: frequency_hz[bin],
            magnitude: magnitude[bin],
            magnitude_db: 20.0 * magnitude[bin].max(FFT_DB_NOISE_FLOOR).log10(),
            phase_degrees: phase_degrees[bin],
        })
        .collect();
    Hdf5FftResult {
        status: rspice_core::engine::TransientFftStatus::Complete,
        analysis_id: crate::commands::run::canonical_analysis_identities(
            rspice_core::execution::AnalysisKind::Fft,
            ordinal,
        )
        .expect("canonical FFT identities")
        .last()
        .expect("one identity per requested ordinal")
        .tag(),
        ordinal,
        source_kind: if physical_type == "parameter" {
            "expression".to_string()
        } else {
            "probe".to_string()
        },
        source_text: "V(OUT)".to_string(),
        authored_output: if physical_type == "parameter" {
            "{V(OUT)}".to_string()
        } else {
            "V(OUT)".to_string()
        },
        output_name: "V(OUT)".to_string(),
        physical_type: physical_type.to_string(),
        value_unit: value_unit.map(str::to_string),
        start_time_s: 0.0,
        stop_time_s: 1.0,
        sample_interval_s: 1.0 / point_count as f64,
        point_count,
        accurate_sampling: true,
        format: if ordinal == 1 {
            "normalized".to_string()
        } else {
            "unnormalized".to_string()
        },
        mode: "hspice_compatible".to_string(),
        window: if ordinal == 1 {
            "hann".to_string()
        } else {
            "rectangular".to_string()
        },
        window_name: if ordinal == 1 {
            "HANN".to_string()
        } else {
            "RECT".to_string()
        },
        alpha: 3.0,
        coherent_gain: 1.0,
        frequency_resolution_hz: 1.0,
        fundamental_bin,
        minimum_metric_bin: 0,
        maximum_metric_bin,
        sfdr_search_minimum_bin,
        bin_indices: (0..u64::try_from(bin_count).expect("bounded bin count")).collect(),
        frequency_hz,
        real,
        imaginary,
        magnitude,
        phase_degrees,
        metrics: with_metrics.then_some(Hdf5FftMetrics {
            fundamental_magnitude: expected_metrics.fundamental_magnitude,
            thd_ratio: expected_metrics.thd_ratio,
            thd_db: expected_metrics.thd_db,
            sndr_db: expected_metrics.sndr_db,
            enob_bits: expected_metrics.enob_bits,
            snr_db: expected_metrics.snr_db,
            sfdr_db: expected_metrics.sfdr_db,
            sfdr_spur_bin: expected_metrics.sfdr_spur_bin,
            sfdr_spur_frequency_hz,
            largest_harmonics,
        }),
    }
}

#[test]
fn typed_fft_section_round_trips_ragged_results_and_coordinate_metadata() {
    let directory = TestDirectory::new("fft-round-trip");
    let destination = directory.0.join("fft.h5");
    let mut data = Hdf5SimulationData::new();
    data.title = "typed FFT".to_string();
    data.fft = Some(Hdf5FftSection {
        parent_analysis_id: "tran-002".to_string(),
        coordinate: Some(Hdf5FftCoordinate {
            coordinate_id: "0123456789abcdef0123456789abcdef-001".to_string(),
            ordinal: 2,
            tag: "run-0123456789abcdef0123456789abcdef-001".to_string(),
            assignment: "PARAM gain = 2, TEMP = 75".to_string(),
        }),
        results: vec![
            fft_result(1, 8, "voltage", Some("1"), true),
            fft_result(2, 16, "parameter", None, false),
        ],
    });

    write_hdf5(&destination, &data).expect("write typed FFT HDF5 artifact");
    assert_eq!(
        read_hdf5(&destination).expect("read typed FFT HDF5 artifact"),
        data
    );
    assert_only_destination_remains(&directory.0, &destination, true);
}

/// A stated unit survives the round trip and an unstated one stays
/// unstated.
///
/// The second half is the part worth a test: `Some("")` would say the
/// quantity's symbol is the empty string, which is a claim, while `None`
/// says nobody made one. Files written before the CLI stated units read
/// back through this same branch.
#[test]
fn a_stated_unit_round_trips_and_an_unstated_one_stays_unstated() {
    let directory = TestDirectory::new("unit-round-trip");
    let destination = directory.0.join("units.h5");
    let mut data = Hdf5SimulationData::new();
    data.title = "units".to_string();

    let mut transient = Hdf5WaveformSection::new("time", vec![0.0, 1.0]);
    transient.add_typed_signal("V(out)", "voltage", Some("V".to_string()), vec![0.0, 2.0]);
    transient.add_typed_signal("I(R1)", "current", Some("A".to_string()), vec![1.0, 2.0]);
    transient.add_typed_signal("ratio", "parameter", None, vec![0.5, 0.5]);
    data.transient = Some(transient);

    let mut ac = Hdf5AcSection::new(vec![1.0, 10.0]);
    ac.add_signal(
        "V(out)",
        Some("V".to_string()),
        vec![1.0, 0.5],
        vec![0.0, -0.5],
    );
    ac.add_signal("loopgain", None, vec![2.0, 1.0], vec![0.0, 0.0]);
    data.ac = Some(ac);

    write_hdf5(&destination, &data).expect("write the unit artifact");
    let read_back = read_hdf5(&destination).expect("read the unit artifact");
    assert_eq!(read_back, data);

    let transient = read_back.transient.expect("a transient section");
    assert_eq!(transient.signals[0].unit.as_deref(), Some("V"));
    assert_eq!(transient.signals[1].unit.as_deref(), Some("A"));
    assert_eq!(transient.signals[2].unit, None);
    let ac = read_back.ac.expect("an AC section");
    assert_eq!(ac.signals[0].unit.as_deref(), Some("V"));
    assert_eq!(ac.signals[1].unit, None);
}

#[test]
fn malformed_fft_section_is_rejected_before_publication() {
    let directory = TestDirectory::new("fft-malformed");
    let destination = directory.0.join("fft.h5");
    let mut malformed = fft_result(1, 8, "voltage", Some("1"), true);
    malformed.analysis_id = "fft-002".to_string();
    let mut data = Hdf5SimulationData::new();
    data.fft = Some(Hdf5FftSection {
        parent_analysis_id: "tran-001".to_string(),
        coordinate: None,
        results: vec![malformed],
    });

    let error = write_hdf5(&destination, &data).expect_err("reject malformed FFT identity");
    assert!(matches!(error, Hdf5Error::InvalidSchema(_)));
    assert!(!destination.exists());
    assert_only_destination_remains(&directory.0, &destination, false);
}

#[test]
fn fft_units_and_normalization_are_validated_against_transform_semantics() {
    assert!(fft_source_identity_is_valid("probe", "V(OUT)", "V(OUT)"));
    assert!(fft_source_identity_is_valid(
        "expression",
        "2*V(OUT)",
        "{2*V(OUT)}"
    ));
    assert!(!fft_source_identity_is_valid("probe", "V(OUT)", "V(IN)"));
    assert!(!fft_source_identity_is_valid(
        "expression",
        "2*V(OUT)",
        "2*V(OUT)"
    ));
    assert!(
        fft_result(1, 8, "voltage", Some("1"), true)
            .validate(1, "fft-001")
            .is_ok()
    );
    assert!(
        fft_result(1, 8, "current", Some("1"), true)
            .validate(1, "fft-001")
            .is_ok()
    );
    assert!(
        fft_result(2, 8, "voltage", Some("V"), false)
            .validate(2, "fft-002")
            .is_ok()
    );
    assert!(
        fft_result(2, 8, "current", Some("A"), false)
            .validate(2, "fft-002")
            .is_ok()
    );

    assert!(
        fft_result(1, 8, "voltage", Some("V"), true)
            .validate(1, "fft-001")
            .is_err()
    );
    assert!(
        fft_result(2, 8, "current", Some("1"), false)
            .validate(2, "fft-002")
            .is_err()
    );
    assert!(
        fft_result(1, 8, "unsupported", Some("1"), false)
            .validate(1, "fft-001")
            .is_err()
    );

    let mut inconsistent_expression = fft_result(2, 8, "parameter", None, false);
    inconsistent_expression.authored_output = inconsistent_expression.source_text.clone();
    assert!(inconsistent_expression.validate(2, "fft-002").is_err());

    let mut impossible_bounds = fft_result(2, 8, "voltage", Some("V"), false);
    impossible_bounds.fundamental_bin = 2;
    impossible_bounds.minimum_metric_bin = 0;
    impossible_bounds.maximum_metric_bin = 0;
    impossible_bounds.sfdr_search_minimum_bin = 0;
    assert!(impossible_bounds.validate(2, "fft-002").is_err());

    let mut not_normalized = fft_result(1, 8, "voltage", Some("1"), false);
    not_normalized.real[1] = 0.5;
    not_normalized.magnitude[1] = 0.5;
    assert!(not_normalized.validate(1, "fft-001").is_err());

    let mut negative_sub_pico = fft_result(1, 8, "voltage", Some("1"), false);
    negative_sub_pico.magnitude[0] = -1.0e-300;
    assert!(negative_sub_pico.validate(1, "fft-001").is_err());
}

#[test]
fn fft_metric_mutations_are_rejected_against_the_spectrum() {
    let valid = fft_result(1, 8, "voltage", Some("1"), true);
    assert!(valid.validate(1, "fft-001").is_ok());

    let mut wrong_fundamental = valid.clone();
    wrong_fundamental
        .metrics
        .as_mut()
        .expect("metric fixture")
        .fundamental_magnitude += 0.25;
    assert!(wrong_fundamental.validate(1, "fft-001").is_err());

    for mutate in [
        |metrics: &mut Hdf5FftMetrics| metrics.thd_ratio += 0.25,
        |metrics: &mut Hdf5FftMetrics| metrics.thd_db += 1.0,
        |metrics: &mut Hdf5FftMetrics| metrics.sndr_db += 1.0,
        |metrics: &mut Hdf5FftMetrics| metrics.enob_bits += 1.0,
        |metrics: &mut Hdf5FftMetrics| metrics.snr_db += 1.0,
        |metrics: &mut Hdf5FftMetrics| metrics.sfdr_db += 1.0,
    ] {
        let mut malformed = valid.clone();
        mutate(malformed.metrics.as_mut().expect("metric fixture"));
        assert!(malformed.validate(1, "fft-001").is_err());
    }

    let mut wrong_spur = valid.clone();
    let metrics = wrong_spur.metrics.as_mut().expect("metric fixture");
    metrics.sfdr_spur_bin = Some(2);
    metrics.sfdr_spur_frequency_hz = Some(2.0);
    assert!(wrong_spur.validate(1, "fft-001").is_err());

    let mut wrong_harmonic = valid;
    wrong_harmonic
        .metrics
        .as_mut()
        .expect("metric fixture")
        .largest_harmonics[0]
        .magnitude += 1.0e-6;
    assert!(wrong_harmonic.validate(1, "fft-001").is_err());
}

#[test]
fn unsupported_root_and_fft_section_schemas_are_rejected() {
    let directory = TestDirectory::new("fft-future-schema");

    let future_root = directory.0.join("future-root.h5");
    let mut root_builder = rustyhdf5::FileBuilder::new();
    root_builder.set_attr("schema_version", AttrValue::String("2".to_string()));
    std::fs::write(
        &future_root,
        root_builder.finish().expect("encode future root schema"),
    )
    .expect("write future root schema");
    let root_error = read_hdf5(&future_root).expect_err("reject future root schema");
    assert!(matches!(root_error, Hdf5Error::InvalidSchema(_)));

    for (label, version) in [("old", "1"), ("previous", "2"), ("future", "4")] {
        let fft_path = directory.0.join(format!("{label}-fft.h5"));
        let mut fft_builder = rustyhdf5::FileBuilder::new();
        fft_builder.set_attr(
            "schema_version",
            AttrValue::String(SCHEMA_VERSION.to_string()),
        );
        let mut fft_group = fft_builder.create_group("fft");
        fft_group.set_attr("schema_version", AttrValue::String(version.to_string()));
        fft_builder.add_group(fft_group.finish());
        std::fs::write(
            &fft_path,
            fft_builder.finish().expect("encode unsupported FFT schema"),
        )
        .expect("write unsupported FFT schema");
        let fft_error = read_hdf5(&fft_path).expect_err("reject unsupported FFT schema");
        assert!(matches!(fft_error, Hdf5Error::InvalidSchema(_)));
    }
}
