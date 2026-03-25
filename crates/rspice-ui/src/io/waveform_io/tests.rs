use super::super::cadence_psf::test_helpers::{
    build_non_windowed_array_complex_psf, build_non_windowed_array_of_struct_bare_descriptor_psf,
    build_non_windowed_array_of_struct_psf, build_non_windowed_array_real_psf,
    build_non_windowed_complex_psf, build_non_windowed_int8_psf, build_non_windowed_int32_psf,
    build_non_windowed_mixed_real_and_string_psf,
    build_non_windowed_nested_array_real_bare_descriptor_psf,
    build_non_windowed_nested_array_real_psf, build_non_windowed_real_psf,
    build_non_windowed_struct_psf, build_non_windowed_struct_with_array_psf,
    build_non_windowed_variable_length_array_psf, build_windowed_array_complex_psf,
    build_windowed_array_of_struct_bare_descriptor_psf, build_windowed_array_of_struct_psf,
    build_windowed_array_real_psf, build_windowed_nested_array_real_bare_descriptor_psf,
    build_windowed_nested_array_real_psf, build_windowed_real_psf,
    build_windowed_struct_with_array_psf, build_windowed_variable_length_array_psf,
};
use super::*;
use crate::io::binary_io::{PsfHeader, PsfWriter};
use std::io::Write;
use tempfile::{Builder, NamedTempFile};

// =========================================================================
// WaveformFormat Tests
// =========================================================================

#[test]
fn test_format_from_extension() {
    assert_eq!(
        WaveformFormat::from_extension("csv"),
        Some(WaveformFormat::Csv)
    );
    assert_eq!(
        WaveformFormat::from_extension("raw"),
        Some(WaveformFormat::Nutmeg)
    );
    assert_eq!(
        WaveformFormat::from_extension("psf"),
        Some(WaveformFormat::Psf)
    );
    assert_eq!(
        WaveformFormat::from_extension("s3p"),
        Some(WaveformFormat::Touchstone)
    );
    assert_eq!(WaveformFormat::from_extension("xyz"), None);
}

#[test]
fn test_format_capabilities() {
    assert!(WaveformFormat::Csv.can_read());
    assert!(WaveformFormat::Csv.can_write());
    assert!(WaveformFormat::Nutmeg.can_read());
    assert!(!WaveformFormat::Nutmeg.can_write());
    assert!(WaveformFormat::Psf.can_read());
    assert!(WaveformFormat::Touchstone.can_read());
    assert!(WaveformFormat::Touchstone.can_write());
}

// =========================================================================
// SignalType Tests
// =========================================================================

#[test]
fn test_signal_type_from_str() {
    assert_eq!(SignalType::from("time"), SignalType::Time);
    assert_eq!(SignalType::from("voltage"), SignalType::Voltage);
    assert_eq!(SignalType::from("unknown_type"), SignalType::Unknown);
}

#[test]
fn test_signal_type_unit() {
    assert_eq!(SignalType::Time.default_unit(), "s");
    assert_eq!(SignalType::Voltage.default_unit(), "V");
}

// =========================================================================
// WaveformSignal Tests
// =========================================================================

#[test]
fn test_signal_creation() {
    let sig = WaveformSignal::new("v(out)", SignalType::Voltage);
    assert_eq!(sig.name, "v(out)");
    assert_eq!(sig.unit, "V");
}

#[test]
fn test_signal_push() {
    let mut sig = WaveformSignal::new("test", SignalType::Voltage);
    sig.push(1.0);
    sig.push(2.0);
    sig.push(3.0);

    assert_eq!(sig.len(), 3);
    assert_eq!(sig.get(1), Some(2.0));
}

#[test]
fn test_signal_min_max() {
    let mut sig = WaveformSignal::new("test", SignalType::Voltage);
    sig.push(1.0);
    sig.push(5.0);
    sig.push(3.0);

    assert_eq!(sig.min(), Some(1.0));
    assert_eq!(sig.max(), Some(5.0));
}

// =========================================================================
// WaveformDataset Tests
// =========================================================================

#[test]
fn test_dataset_creation() {
    let dataset = WaveformDataset::new("Test Simulation");
    assert_eq!(dataset.title, "Test Simulation");
    assert!(dataset.signals.is_empty());
}

#[test]
fn test_dataset_add_signal() {
    let mut dataset = WaveformDataset::new("Test");
    dataset.add_signal(WaveformSignal::new("v(out)", SignalType::Voltage));
    dataset.add_signal(WaveformSignal::new("i(vdd)", SignalType::Current));

    assert_eq!(dataset.signal_count(), 2);
    assert!(dataset.get_signal("v(out)").is_some());
}

#[test]
fn test_dataset_signal_names() {
    let mut dataset = WaveformDataset::new("Test");
    dataset.add_signal(WaveformSignal::new("sig1", SignalType::Voltage));
    dataset.add_signal(WaveformSignal::new("sig2", SignalType::Current));

    let names = dataset.signal_names();
    assert!(names.contains(&"sig1"));
    assert!(names.contains(&"sig2"));
}

// =========================================================================
// CSV I/O Tests
// =========================================================================

#[test]
fn test_csv_roundtrip() {
    // Create dataset
    let mut dataset = WaveformDataset::new("Test");

    let mut time = WaveformSignal::new("time", SignalType::Time);
    time.data = vec![0.0, 1e-9, 2e-9, 3e-9];
    dataset.set_x(time);

    let mut vout = WaveformSignal::new("v(out)", SignalType::Voltage);
    vout.data = vec![0.0, 0.5, 1.0, 0.8];
    dataset.add_signal(vout);

    // Write
    let temp = NamedTempFile::new().unwrap();
    let writer = WaveformWriter::new(WaveformFormat::Csv);
    writer.write(&dataset, temp.path()).unwrap();

    // Read back
    let reader = WaveformReader::new(WaveformFormat::Csv);
    let loaded = reader.read(temp.path()).unwrap();

    assert_eq!(loaded.signal_count(), 1);
    assert_eq!(loaded.point_count(), 4);
}

#[test]
fn test_read_csv_with_header() {
    let mut temp = NamedTempFile::new().unwrap();
    writeln!(temp, "time,v(out),i(vdd)").unwrap();
    writeln!(temp, "0,0.0,1e-3").unwrap();
    writeln!(temp, "1e-9,0.5,0.5e-3").unwrap();
    writeln!(temp, "2e-9,1.0,0.2e-3").unwrap();

    let reader = WaveformReader::new(WaveformFormat::Csv);
    let dataset = reader.read(temp.path()).unwrap();

    assert_eq!(dataset.signal_count(), 2);
    assert_eq!(dataset.point_count(), 3);
}

// =========================================================================
// NUTMEG Format Tests
// =========================================================================

#[test]
fn test_read_nutmeg_basic() {
    let mut temp = NamedTempFile::new().unwrap();
    writeln!(temp, "Title: Test Simulation").unwrap();
    writeln!(temp, "Plotname: Transient Analysis").unwrap();
    writeln!(temp, "No. Variables: 2").unwrap();
    writeln!(temp, "No. Points: 3").unwrap();
    writeln!(temp, "Variables:").unwrap();
    writeln!(temp, "\t0\ttime\ttime").unwrap();
    writeln!(temp, "\t1\tv(out)\tvoltage").unwrap();
    writeln!(temp, "Values:").unwrap();
    writeln!(temp, "0 0.0").unwrap();
    writeln!(temp, "1e-9 0.5").unwrap();
    writeln!(temp, "2e-9 1.0").unwrap();

    let reader = WaveformReader::new(WaveformFormat::Nutmeg);
    let dataset = reader.read(temp.path()).unwrap();

    assert_eq!(dataset.title, "Test Simulation");
    assert_eq!(dataset.analysis, "Transient Analysis");
}

#[test]
fn test_write_touchstone_v1_two_port() {
    let mut dataset = WaveformDataset::new("sp");
    dataset.metadata.insert("z0".to_string(), "75".to_string());
    dataset
        .metadata
        .insert("touchstone_version".to_string(), "1".to_string());

    let mut freq = WaveformSignal::new("frequency", SignalType::Frequency);
    freq.data = vec![1.0e6, 2.0e6];
    dataset.set_x(freq);

    let mut s11_re = WaveformSignal::new("S11_RE", SignalType::SParameter);
    s11_re.data = vec![0.1, 0.2];
    let mut s11_im = WaveformSignal::new("S11_IM", SignalType::SParameter);
    s11_im.data = vec![0.01, 0.02];
    let mut s21_re = WaveformSignal::new("S21_RE", SignalType::SParameter);
    s21_re.data = vec![0.9, 0.8];
    let mut s21_im = WaveformSignal::new("S21_IM", SignalType::SParameter);
    s21_im.data = vec![0.0, -0.1];
    let mut s12_re = WaveformSignal::new("S12_RE", SignalType::SParameter);
    s12_re.data = vec![0.01, 0.02];
    let mut s12_im = WaveformSignal::new("S12_IM", SignalType::SParameter);
    s12_im.data = vec![0.0, 0.0];
    let mut s22_re = WaveformSignal::new("S22_RE", SignalType::SParameter);
    s22_re.data = vec![0.2, 0.3];
    let mut s22_im = WaveformSignal::new("S22_IM", SignalType::SParameter);
    s22_im.data = vec![-0.01, -0.02];

    dataset.add_signal(s11_re);
    dataset.add_signal(s11_im);
    dataset.add_signal(s21_re);
    dataset.add_signal(s21_im);
    dataset.add_signal(s12_re);
    dataset.add_signal(s12_im);
    dataset.add_signal(s22_re);
    dataset.add_signal(s22_im);

    let temp = NamedTempFile::new().unwrap();
    let writer = WaveformWriter::new(WaveformFormat::Touchstone);
    writer
        .write(&dataset, temp.path())
        .expect("touchstone write");

    let content = std::fs::read_to_string(temp.path()).expect("read touchstone");
    assert!(content.contains("# Hz S RI R 75"));
    assert!(
        content
            .lines()
            .any(|line| line.contains("1.000000000000e6"))
    );
    assert!(
        content
            .lines()
            .any(|line| line.contains("2.000000000000e6"))
    );
    assert!(!content.contains("[Version] 2.0"));
}

#[test]
fn test_write_touchstone_v2_two_port() {
    let mut dataset = WaveformDataset::new("spv2");
    dataset
        .metadata
        .insert("touchstone_version".to_string(), "2".to_string());

    let mut freq = WaveformSignal::new("freq", SignalType::Frequency);
    freq.data = vec![1.0e9];
    dataset.set_x(freq);

    for (name, value) in [
        ("S11_RE", 0.1),
        ("S11_IM", 0.0),
        ("S21_RE", 0.8),
        ("S21_IM", -0.1),
        ("S12_RE", 0.02),
        ("S12_IM", 0.0),
        ("S22_RE", 0.2),
        ("S22_IM", 0.01),
    ] {
        let mut signal = WaveformSignal::new(name, SignalType::SParameter);
        signal.data = vec![value];
        dataset.add_signal(signal);
    }

    let temp = NamedTempFile::new().unwrap();
    WaveformWriter::new(WaveformFormat::Touchstone)
        .write(&dataset, temp.path())
        .expect("touchstone v2 write");

    let content = std::fs::read_to_string(temp.path()).expect("read touchstone");
    assert!(content.contains("[Version] 2.0"));
    assert!(content.contains("[Number of Ports] 2"));
    assert!(content.contains("[Network Data]"));
    assert!(content.contains("[End]"));
}

#[test]
fn test_write_touchstone_v2_three_port() {
    let mut dataset = WaveformDataset::new("sp3");
    dataset
        .metadata
        .insert("touchstone_version".to_string(), "2".to_string());

    let mut freq = WaveformSignal::new("frequency", SignalType::Frequency);
    freq.data = vec![1.0e6, 2.0e6];
    dataset.set_x(freq);

    for row in 1..=3 {
        for col in 1..=3 {
            let base = format!("S{}_{}", row, col);
            let mut re = WaveformSignal::new(format!("{}_RE", base), SignalType::SParameter);
            let mut im = WaveformSignal::new(format!("{}_IM", base), SignalType::SParameter);
            re.data = vec![0.1 * row as f64, 0.2 * col as f64];
            im.data = vec![0.01 * col as f64, -0.02 * row as f64];
            dataset.add_signal(re);
            dataset.add_signal(im);
        }
    }

    let temp = NamedTempFile::new().unwrap();
    WaveformWriter::new(WaveformFormat::Touchstone)
        .write(&dataset, temp.path())
        .expect("touchstone 3-port write");

    let content = std::fs::read_to_string(temp.path()).expect("read touchstone");
    assert!(content.contains("[Number of Ports] 3"));
    assert!(
        content
            .lines()
            .any(|line| line.starts_with("1.000000000000e6 "))
    );
}

#[test]
fn test_write_touchstone_requires_complete_matrix_components() {
    let mut dataset = WaveformDataset::new("sp_missing");
    let mut freq = WaveformSignal::new("frequency", SignalType::Frequency);
    freq.data = vec![1.0e6];
    dataset.set_x(freq);

    // Intentionally missing S11_IM
    let mut s11_re = WaveformSignal::new("S11_RE", SignalType::SParameter);
    s11_re.data = vec![0.1];
    dataset.add_signal(s11_re);
    for (name, value) in [
        ("S21_RE", 0.8),
        ("S21_IM", -0.1),
        ("S12_RE", 0.02),
        ("S12_IM", 0.0),
        ("S22_RE", 0.2),
        ("S22_IM", 0.01),
    ] {
        let mut signal = WaveformSignal::new(name, SignalType::SParameter);
        signal.data = vec![value];
        dataset.add_signal(signal);
    }

    let writer = WaveformWriter::new(WaveformFormat::Touchstone);
    let err = writer
        .write(&dataset, Path::new("dummy.s2p"))
        .expect_err("missing matrix components should fail");
    assert!(err.contains("Missing Touchstone imag component for S11"));
}

#[test]
fn test_write_touchstone_v2_writes_reference_block_for_per_port_z0() {
    let mut dataset = WaveformDataset::new("sp_ref");
    dataset
        .metadata
        .insert("touchstone_version".to_string(), "2".to_string());
    dataset
        .metadata
        .insert("z0_ports".to_string(), "50,75".to_string());

    let mut freq = WaveformSignal::new("frequency", SignalType::Frequency);
    freq.data = vec![1.0e6];
    dataset.set_x(freq);

    for (name, value) in [
        ("S11_RE", 0.1),
        ("S11_IM", 0.0),
        ("S21_RE", 0.8),
        ("S21_IM", -0.1),
        ("S12_RE", 0.02),
        ("S12_IM", 0.0),
        ("S22_RE", 0.2),
        ("S22_IM", 0.01),
    ] {
        let mut signal = WaveformSignal::new(name, SignalType::SParameter);
        signal.data = vec![value];
        dataset.add_signal(signal);
    }

    let temp = NamedTempFile::new().expect("temp touchstone");
    WaveformWriter::new(WaveformFormat::Touchstone)
        .write(&dataset, temp.path())
        .expect("touchstone write should succeed");
    let content = std::fs::read_to_string(temp.path()).expect("read touchstone");

    assert!(content.contains("[Reference]"));
    assert!(content.contains("5.000000000000e1"));
    assert!(content.contains("7.500000000000e1"));
}

#[test]
fn test_write_touchstone_v1_rejects_nonuniform_per_port_z0() {
    let mut dataset = WaveformDataset::new("sp_ref_v1");
    dataset
        .metadata
        .insert("touchstone_version".to_string(), "1".to_string());
    dataset
        .metadata
        .insert("z0_ports".to_string(), "50,75".to_string());

    let mut freq = WaveformSignal::new("frequency", SignalType::Frequency);
    freq.data = vec![1.0e6];
    dataset.set_x(freq);

    for (name, value) in [
        ("S11_RE", 0.1),
        ("S11_IM", 0.0),
        ("S21_RE", 0.8),
        ("S21_IM", -0.1),
        ("S12_RE", 0.02),
        ("S12_IM", 0.0),
        ("S22_RE", 0.2),
        ("S22_IM", 0.01),
    ] {
        let mut signal = WaveformSignal::new(name, SignalType::SParameter);
        signal.data = vec![value];
        dataset.add_signal(signal);
    }

    let err = WaveformWriter::new(WaveformFormat::Touchstone)
        .write(&dataset, Path::new("dummy.s2p"))
        .expect_err("touchstone v1 must reject non-uniform per-port z0");
    assert!(err.contains("v1 does not support per-port reference impedance"));
}

#[test]
fn test_read_touchstone_v1_two_port_ri() {
    let mut temp = Builder::new()
        .suffix(".s2p")
        .tempfile()
        .expect("temp touchstone");
    writeln!(temp, "! touchstone v1").expect("write");
    writeln!(temp, "# Hz S RI R 75").expect("write");
    writeln!(temp, "1.0e6 0.1 0.01 0.9 0.0 0.02 0.0 0.2 -0.01").expect("write");
    writeln!(temp, "2.0e6 0.2 0.02 0.8 -0.1 0.03 0.0 0.3 -0.02").expect("write");

    let dataset = WaveformReader::new(WaveformFormat::Touchstone)
        .read(temp.path())
        .expect("touchstone read");

    assert_eq!(dataset.analysis, "S-Parameter");
    assert_eq!(dataset.point_count(), 2);
    assert_eq!(dataset.signal_count(), 8);
    assert_eq!(
        dataset.metadata.get("z0_ports").map(String::as_str),
        Some("75,75")
    );
    assert_eq!(
        dataset
            .get_signal("S21_RE")
            .and_then(|sig| sig.get(0))
            .unwrap_or_default(),
        0.9
    );
}

#[test]
fn test_read_touchstone_v2_db_three_port_with_reference() {
    let mut temp = Builder::new()
        .suffix(".s3p")
        .tempfile()
        .expect("temp touchstone");
    writeln!(temp, "[Version] 2.0").expect("write");
    writeln!(temp, "[Number of Ports] 3").expect("write");
    writeln!(temp, "[Number of Frequencies] 1").expect("write");
    writeln!(temp, "[Reference] 50 60 70").expect("write");
    writeln!(temp, "# GHz S DB R 50").expect("write");
    writeln!(temp, "[Network Data]").expect("write");
    writeln!(
        temp,
        "1.0 -6 0 -20 0 -30 0 -40 0 -6 0 -20 0 -40 0 -30 0 -6 0"
    )
    .expect("write");
    writeln!(temp, "[End]").expect("write");

    let dataset = WaveformReader::new(WaveformFormat::Touchstone)
        .read(temp.path())
        .expect("touchstone read");
    assert_eq!(
        dataset
            .metadata
            .get("touchstone_version")
            .map(String::as_str),
        Some("2")
    );
    assert_eq!(
        dataset.metadata.get("z0_ports").map(String::as_str),
        Some("50,60,70")
    );
    let s11_re = dataset
        .get_signal("S11_RE")
        .and_then(|sig| sig.get(0))
        .unwrap_or_default();
    assert!((s11_re - 0.501_187_233_627_272_2).abs() < 1e-12);
}

#[test]
fn test_read_touchstone_v2_lower_matrix_two_port_mirrors_upper_half() {
    let mut temp = Builder::new()
        .suffix(".s2p")
        .tempfile()
        .expect("temp touchstone");
    writeln!(temp, "[Version] 2.0").expect("write");
    writeln!(temp, "[Number of Ports] 2").expect("write");
    writeln!(temp, "[Matrix Format] Lower").expect("write");
    writeln!(temp, "# Hz S RI R 50").expect("write");
    writeln!(temp, "[Network Data]").expect("write");
    // Lower triangular entries in column-major order: S11, S21, S22.
    writeln!(temp, "1.0e6 0.1 0.01 0.8 -0.2 0.3 0.04").expect("write");
    writeln!(temp, "[End]").expect("write");

    let dataset = WaveformReader::new(WaveformFormat::Touchstone)
        .read(temp.path())
        .expect("touchstone read");
    assert_eq!(
        dataset
            .metadata
            .get("touchstone_matrix_format")
            .map(String::as_str),
        Some("lower")
    );
    let s12_re = dataset
        .get_signal("S12_RE")
        .and_then(|sig| sig.get(0))
        .unwrap_or_default();
    let s21_re = dataset
        .get_signal("S21_RE")
        .and_then(|sig| sig.get(0))
        .unwrap_or_default();
    assert!((s12_re - s21_re).abs() < 1e-15);
}

#[test]
fn test_read_touchstone_v2_upper_matrix_three_port_mirrors_lower_half() {
    let mut temp = Builder::new()
        .suffix(".s3p")
        .tempfile()
        .expect("temp touchstone");
    writeln!(temp, "[Version] 2.0").expect("write");
    writeln!(temp, "[Number of Ports] 3").expect("write");
    writeln!(temp, "[Matrix Format] Upper").expect("write");
    writeln!(temp, "# Hz S RI R 50").expect("write");
    writeln!(temp, "[Network Data]").expect("write");
    // Upper triangular entries in column-major order: S11, S12,S22, S13,S23,S33.
    writeln!(
        temp,
        "2.0e6 0.1 0.0 0.2 0.0 0.3 0.0 0.4 0.0 0.5 0.0 0.6 0.0"
    )
    .expect("write");
    writeln!(temp, "[End]").expect("write");

    let dataset = WaveformReader::new(WaveformFormat::Touchstone)
        .read(temp.path())
        .expect("touchstone read");
    assert_eq!(
        dataset
            .metadata
            .get("touchstone_matrix_format")
            .map(String::as_str),
        Some("upper")
    );

    let s13_re = dataset
        .get_signal("S13_RE")
        .and_then(|sig| sig.get(0))
        .unwrap_or_default();
    let s31_re = dataset
        .get_signal("S31_RE")
        .and_then(|sig| sig.get(0))
        .unwrap_or_default();
    let s23_re = dataset
        .get_signal("S23_RE")
        .and_then(|sig| sig.get(0))
        .unwrap_or_default();
    let s32_re = dataset
        .get_signal("S32_RE")
        .and_then(|sig| sig.get(0))
        .unwrap_or_default();
    assert!((s13_re - s31_re).abs() < 1e-15);
    assert!((s23_re - s32_re).abs() < 1e-15);
}

#[test]
fn test_read_touchstone_rejects_non_s_parameter_data() {
    let mut temp = NamedTempFile::new().expect("temp touchstone");
    writeln!(temp, "# Hz Y RI R 50").expect("write");
    writeln!(temp, "1.0e6 0.0 0.0 0.0 0.0").expect("write");

    let err = WaveformReader::new(WaveformFormat::Touchstone)
        .read(temp.path())
        .expect_err("non-S touchstone should fail");
    assert!(err.contains("only S-parameter files are supported"));
}

#[test]
fn test_read_touchstone_rejects_malformed_numeric_record_count() {
    let mut temp = Builder::new()
        .suffix(".s2p")
        .tempfile()
        .expect("temp touchstone");
    writeln!(temp, "# Hz S RI R 50").expect("write");
    // s2p requires 9 numeric values per frequency line (1 + 8),
    // this record is missing one value.
    writeln!(temp, "1.0e6 0.1 0.0 0.8 0.0 0.02 0.0 0.2").expect("write");

    let err = WaveformReader::new(WaveformFormat::Touchstone)
        .read(temp.path())
        .expect_err("malformed touchstone should fail");
    assert!(err.contains("not divisible by record width"));
}

#[test]
fn test_read_psf_lite_roundtrip() {
    let temp = NamedTempFile::new().expect("temp file");
    let header = PsfHeader::new(3, 4);

    {
        let mut writer = PsfWriter::create(temp.path()).expect("psf writer");
        writer.write_header(&header).expect("header");
        writer.write_trace(&[0.0, 1.0, 2.0, 3.0]).expect("time");
        writer.write_trace(&[0.1, 0.2, 0.3, 0.4]).expect("trace 1");
        writer.write_trace(&[1.1, 1.2, 1.3, 1.4]).expect("trace 2");
    }

    let reader = WaveformReader::new(WaveformFormat::Psf);
    let dataset = reader.read(temp.path()).expect("PSF-Lite read should work");

    assert_eq!(dataset.analysis, "PSF-Lite");
    assert_eq!(dataset.point_count(), 4);
    assert_eq!(dataset.signal_count(), 2);
    assert_eq!(
        dataset
            .x_signal
            .as_ref()
            .and_then(|x| x.get(2))
            .unwrap_or(0.0),
        2.0
    );
    assert_eq!(dataset.signals[0].data[0], 0.1);
}

#[test]
fn test_read_cadence_psf_binary_real_trace() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_non_windowed_real_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(
        dataset.metadata.get("format").map(String::as_str),
        Some("psf-binary-cadence")
    );
    assert_eq!(dataset.analysis, "Transient");
    assert_eq!(
        dataset.x_signal.as_ref().expect("x axis exists").data,
        vec![0.0, 1.0]
    );
    assert_eq!(dataset.signal_count(), 1);
    assert_eq!(
        dataset.get_signal("V(out)").expect("trace exists").data,
        vec![1.0, 2.0]
    );
}

#[test]
fn test_read_cadence_psf_binary_complex_trace() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_non_windowed_complex_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(
        dataset.metadata.get("format").map(String::as_str),
        Some("psf-binary-cadence")
    );
    assert_eq!(
        dataset
            .x_signal
            .as_ref()
            .expect("x axis exists")
            .data
            .as_slice(),
        &[0.0, 1.0]
    );
    assert_eq!(dataset.signal_count(), 2);
    assert_eq!(
        dataset
            .get_signal("V(out)_RE")
            .expect("real trace exists")
            .data,
        vec![1.0, 2.0]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)_IM")
            .expect("imag trace exists")
            .data,
        vec![0.5, -0.25]
    );
}

#[test]
fn test_read_cadence_psf_binary_int8_trace() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_non_windowed_int8_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(
        dataset.metadata.get("format").map(String::as_str),
        Some("psf-binary-cadence")
    );
    assert_eq!(
        dataset
            .x_signal
            .as_ref()
            .expect("x axis exists")
            .data
            .as_slice(),
        &[0.0, 1.0]
    );
    assert_eq!(dataset.signal_count(), 1);
    assert_eq!(
        dataset.get_signal("V(out)").expect("trace exists").data,
        vec![7.0, 255.0]
    );
}

#[test]
fn test_read_cadence_psf_binary_int32_trace() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_non_windowed_int32_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(
        dataset.metadata.get("format").map(String::as_str),
        Some("psf-binary-cadence")
    );
    assert_eq!(
        dataset
            .x_signal
            .as_ref()
            .expect("x axis exists")
            .data
            .as_slice(),
        &[0.0, 1.0]
    );
    assert_eq!(dataset.signal_count(), 1);
    assert_eq!(
        dataset.get_signal("V(out)").expect("trace exists").data,
        vec![1024.0, -2.0]
    );
}

#[test]
fn test_read_cadence_psf_binary_struct_trace_flattens_members() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_non_windowed_struct_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(
        dataset
            .x_signal
            .as_ref()
            .expect("x axis exists")
            .data
            .as_slice(),
        &[0.0, 1.0]
    );
    assert_eq!(
        dataset.get_signal("V(out).dc").expect("dc signal").data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset
            .get_signal("V(out).ac_RE")
            .expect("ac re signal")
            .data,
        vec![2.0, 2.5]
    );
    assert_eq!(
        dataset
            .get_signal("V(out).ac_IM")
            .expect("ac im signal")
            .data,
        vec![0.5, -0.25]
    );
}

#[test]
fn test_read_cadence_psf_binary_ignores_string_traces() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_non_windowed_mixed_real_and_string_psf())
        .expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(
        dataset
            .x_signal
            .as_ref()
            .expect("x axis exists")
            .data
            .as_slice(),
        &[0.0, 1.0]
    );
    assert_eq!(dataset.signal_count(), 1);
    assert_eq!(
        dataset.get_signal("V(out)").expect("real signal").data,
        vec![1.25, 2.5]
    );
    assert!(dataset.get_signal("meta").is_none());
}

#[test]
fn test_read_cadence_psf_binary_real_array_trace_expands_indices() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_non_windowed_array_real_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(
        dataset
            .x_signal
            .as_ref()
            .expect("x axis exists")
            .data
            .as_slice(),
        &[0.0, 1.0]
    );
    assert_eq!(dataset.signal_count(), 2);
    assert_eq!(
        dataset.get_signal("V(arr)[0]").expect("idx0 exists").data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset.get_signal("V(arr)[1]").expect("idx1 exists").data,
        vec![2.0, 2.5]
    );
}

#[test]
fn test_read_cadence_psf_binary_complex_array_trace_expands_indices() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_non_windowed_array_complex_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(
        dataset
            .x_signal
            .as_ref()
            .expect("x axis exists")
            .data
            .as_slice(),
        &[0.0, 1.0]
    );
    assert_eq!(dataset.signal_count(), 4);
    assert_eq!(
        dataset
            .get_signal("I(arr)[0]_RE")
            .expect("idx0 re exists")
            .data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset
            .get_signal("I(arr)[0]_IM")
            .expect("idx0 im exists")
            .data,
        vec![0.25, 0.125]
    );
    assert_eq!(
        dataset
            .get_signal("I(arr)[1]_RE")
            .expect("idx1 re exists")
            .data,
        vec![2.0, 2.5]
    );
    assert_eq!(
        dataset
            .get_signal("I(arr)[1]_IM")
            .expect("idx1 im exists")
            .data,
        vec![-0.5, -0.75]
    );
}

#[test]
fn test_read_cadence_psf_binary_struct_with_array_flattens_members() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_non_windowed_struct_with_array_psf())
        .expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(dataset.signal_count(), 3);
    assert_eq!(
        dataset.get_signal("V(out).gain").expect("gain").data,
        vec![10.0, 11.0]
    );
    assert_eq!(
        dataset.get_signal("V(out).taps[0]").expect("taps[0]").data,
        vec![0.1, 0.15]
    );
    assert_eq!(
        dataset.get_signal("V(out).taps[1]").expect("taps[1]").data,
        vec![0.2, 0.25]
    );
}

#[test]
fn test_read_cadence_psf_binary_variable_length_array_pads_nans() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_non_windowed_variable_length_array_psf())
        .expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(dataset.signal_count(), 3);
    assert_eq!(
        dataset.get_signal("V(arr)[0]").expect("idx0").data,
        vec![1.0, 1.5]
    );
    let idx1 = &dataset.get_signal("V(arr)[1]").expect("idx1").data;
    assert!(idx1[0].is_nan());
    assert_eq!(idx1[1], 2.5);
    let idx2 = &dataset.get_signal("V(arr)[2]").expect("idx2").data;
    assert!(idx2[0].is_nan());
    assert_eq!(idx2[1], 3.5);
}

#[test]
fn test_read_cadence_psf_binary_array_of_struct_expands_nested_members() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_non_windowed_array_of_struct_psf())
        .expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(dataset.signal_count(), 6);
    assert_eq!(
        dataset.get_signal("V(out)[0].dc").expect("idx0 dc").data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[0].ac_RE")
            .expect("idx0 ac re")
            .data,
        vec![2.0, 2.5]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[0].ac_IM")
            .expect("idx0 ac im")
            .data,
        vec![0.5, -0.2]
    );
    assert_eq!(
        dataset.get_signal("V(out)[1].dc").expect("idx1 dc").data,
        vec![1.1, 1.6]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[1].ac_RE")
            .expect("idx1 ac re")
            .data,
        vec![2.1, 2.6]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[1].ac_IM")
            .expect("idx1 ac im")
            .data,
        vec![0.6, -0.3]
    );
}

#[test]
fn test_read_cadence_psf_binary_array_of_struct_bare_descriptor_expands_nested_members() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(
        temp.path(),
        build_non_windowed_array_of_struct_bare_descriptor_psf(),
    )
    .expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(dataset.signal_count(), 6);
    assert_eq!(
        dataset.get_signal("V(out)[0].dc").expect("idx0 dc").data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[0].ac_RE")
            .expect("idx0 ac re")
            .data,
        vec![2.0, 2.5]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[0].ac_IM")
            .expect("idx0 ac im")
            .data,
        vec![0.5, -0.2]
    );
    assert_eq!(
        dataset.get_signal("V(out)[1].dc").expect("idx1 dc").data,
        vec![1.1, 1.6]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[1].ac_RE")
            .expect("idx1 ac re")
            .data,
        vec![2.1, 2.6]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[1].ac_IM")
            .expect("idx1 ac im")
            .data,
        vec![0.6, -0.3]
    );
}

#[test]
fn test_read_cadence_psf_binary_nested_array_real_expands_indices() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_non_windowed_nested_array_real_psf())
        .expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(dataset.signal_count(), 4);
    assert_eq!(
        dataset.get_signal("V(out)[0][0]").expect("00").data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset.get_signal("V(out)[0][1]").expect("01").data,
        vec![2.0, 2.5]
    );
    assert_eq!(
        dataset.get_signal("V(out)[1][0]").expect("10").data,
        vec![3.0, 3.5]
    );
    assert_eq!(
        dataset.get_signal("V(out)[1][1]").expect("11").data,
        vec![4.0, 4.5]
    );
}

#[test]
fn test_read_cadence_psf_binary_nested_array_real_bare_descriptor_expands_indices() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(
        temp.path(),
        build_non_windowed_nested_array_real_bare_descriptor_psf(),
    )
    .expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(dataset.signal_count(), 4);
    assert_eq!(
        dataset.get_signal("V(out)[0][0]").expect("00").data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset.get_signal("V(out)[0][1]").expect("01").data,
        vec![2.0, 2.5]
    );
    assert_eq!(
        dataset.get_signal("V(out)[1][0]").expect("10").data,
        vec![3.0, 3.5]
    );
    assert_eq!(
        dataset.get_signal("V(out)[1][1]").expect("11").data,
        vec![4.0, 4.5]
    );
}

#[test]
fn test_read_cadence_psf_binary_windowed_real_trace() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_windowed_real_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(
        dataset.metadata.get("format").map(String::as_str),
        Some("psf-binary-cadence")
    );
    assert_eq!(dataset.analysis, "Transient");
    assert_eq!(
        dataset
            .x_signal
            .as_ref()
            .expect("x axis exists")
            .data
            .as_slice(),
        &[0.0, 1.0]
    );
    assert_eq!(dataset.signal_count(), 1);
    assert_eq!(
        dataset.get_signal("V(out)").expect("trace exists").data,
        vec![1.0, 2.0]
    );
}

#[test]
fn test_read_cadence_psf_binary_windowed_real_array_trace_expands_indices() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_windowed_array_real_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(
        dataset
            .x_signal
            .as_ref()
            .expect("x axis exists")
            .data
            .as_slice(),
        &[0.0, 1.0]
    );
    assert_eq!(dataset.signal_count(), 2);
    assert_eq!(
        dataset.get_signal("V(arr)[0]").expect("idx0 exists").data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset.get_signal("V(arr)[1]").expect("idx1 exists").data,
        vec![2.0, 2.5]
    );
}

#[test]
fn test_read_cadence_psf_binary_windowed_complex_array_trace_expands_indices() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_windowed_array_complex_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(
        dataset
            .x_signal
            .as_ref()
            .expect("x axis exists")
            .data
            .as_slice(),
        &[0.0, 1.0]
    );
    assert_eq!(dataset.signal_count(), 4);
    assert_eq!(
        dataset
            .get_signal("I(arr)[0]_RE")
            .expect("idx0 re exists")
            .data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset
            .get_signal("I(arr)[0]_IM")
            .expect("idx0 im exists")
            .data,
        vec![0.25, 0.125]
    );
    assert_eq!(
        dataset
            .get_signal("I(arr)[1]_RE")
            .expect("idx1 re exists")
            .data,
        vec![2.0, 2.5]
    );
    assert_eq!(
        dataset
            .get_signal("I(arr)[1]_IM")
            .expect("idx1 im exists")
            .data,
        vec![-0.5, -0.75]
    );
}

#[test]
fn test_read_cadence_psf_binary_windowed_struct_with_array_flattens_members() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_windowed_struct_with_array_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(dataset.signal_count(), 3);
    assert_eq!(
        dataset.get_signal("V(out).gain").expect("gain").data,
        vec![10.0, 11.0]
    );
    assert_eq!(
        dataset.get_signal("V(out).taps[0]").expect("taps[0]").data,
        vec![0.1, 0.15]
    );
    assert_eq!(
        dataset.get_signal("V(out).taps[1]").expect("taps[1]").data,
        vec![0.2, 0.25]
    );
}

#[test]
fn test_read_cadence_psf_binary_windowed_variable_length_array_pads_nans() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_windowed_variable_length_array_psf())
        .expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(dataset.signal_count(), 3);
    assert_eq!(
        dataset.get_signal("V(arr)[0]").expect("idx0").data,
        vec![1.0, 1.5]
    );
    let idx1 = &dataset.get_signal("V(arr)[1]").expect("idx1").data;
    assert!(idx1[0].is_nan());
    assert_eq!(idx1[1], 2.5);
    let idx2 = &dataset.get_signal("V(arr)[2]").expect("idx2").data;
    assert!(idx2[0].is_nan());
    assert_eq!(idx2[1], 3.5);
}

#[test]
fn test_read_cadence_psf_binary_windowed_array_of_struct_expands_nested_members() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_windowed_array_of_struct_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(dataset.signal_count(), 6);
    assert_eq!(
        dataset.get_signal("V(out)[0].dc").expect("idx0 dc").data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[0].ac_RE")
            .expect("idx0 ac re")
            .data,
        vec![2.0, 2.5]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[0].ac_IM")
            .expect("idx0 ac im")
            .data,
        vec![0.5, -0.2]
    );
    assert_eq!(
        dataset.get_signal("V(out)[1].dc").expect("idx1 dc").data,
        vec![1.1, 1.6]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[1].ac_RE")
            .expect("idx1 ac re")
            .data,
        vec![2.1, 2.6]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[1].ac_IM")
            .expect("idx1 ac im")
            .data,
        vec![0.6, -0.3]
    );
}

#[test]
fn test_read_cadence_psf_binary_windowed_array_of_struct_bare_descriptor_expands_nested_members() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(
        temp.path(),
        build_windowed_array_of_struct_bare_descriptor_psf(),
    )
    .expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(dataset.signal_count(), 6);
    assert_eq!(
        dataset.get_signal("V(out)[0].dc").expect("idx0 dc").data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[0].ac_RE")
            .expect("idx0 ac re")
            .data,
        vec![2.0, 2.5]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[0].ac_IM")
            .expect("idx0 ac im")
            .data,
        vec![0.5, -0.2]
    );
    assert_eq!(
        dataset.get_signal("V(out)[1].dc").expect("idx1 dc").data,
        vec![1.1, 1.6]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[1].ac_RE")
            .expect("idx1 ac re")
            .data,
        vec![2.1, 2.6]
    );
    assert_eq!(
        dataset
            .get_signal("V(out)[1].ac_IM")
            .expect("idx1 ac im")
            .data,
        vec![0.6, -0.3]
    );
}

#[test]
fn test_read_cadence_psf_binary_windowed_nested_array_real_expands_indices() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(temp.path(), build_windowed_nested_array_real_psf()).expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(dataset.signal_count(), 4);
    assert_eq!(
        dataset.get_signal("V(out)[0][0]").expect("00").data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset.get_signal("V(out)[0][1]").expect("01").data,
        vec![2.0, 2.5]
    );
    assert_eq!(
        dataset.get_signal("V(out)[1][0]").expect("10").data,
        vec![3.0, 3.5]
    );
    assert_eq!(
        dataset.get_signal("V(out)[1][1]").expect("11").data,
        vec![4.0, 4.5]
    );
}

#[test]
fn test_read_cadence_psf_binary_windowed_nested_array_real_bare_descriptor_expands_indices() {
    let temp = Builder::new().suffix(".psf").tempfile().expect("temp psf");
    std::fs::write(
        temp.path(),
        build_windowed_nested_array_real_bare_descriptor_psf(),
    )
    .expect("write cadence psf");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("cadence psf binary read should work");

    assert_eq!(dataset.signal_count(), 4);
    assert_eq!(
        dataset.get_signal("V(out)[0][0]").expect("00").data,
        vec![1.0, 1.5]
    );
    assert_eq!(
        dataset.get_signal("V(out)[0][1]").expect("01").data,
        vec![2.0, 2.5]
    );
    assert_eq!(
        dataset.get_signal("V(out)[1][0]").expect("10").data,
        vec![3.0, 3.5]
    );
    assert_eq!(
        dataset.get_signal("V(out)[1][1]").expect("11").data,
        vec![4.0, 4.5]
    );
}

#[test]
fn test_read_psf_ascii_file_transient_records() {
    let mut temp = Builder::new()
        .suffix(".psfascii")
        .tempfile()
        .expect("temp psf ascii");
    writeln!(temp, "\"time\" 0.0").expect("write");
    writeln!(temp, "\"V(out)\" 0.0").expect("write");
    writeln!(temp, "\"time\" 1e-9").expect("write");
    writeln!(temp, "\"V(out)\" 0.5").expect("write");
    writeln!(temp, "\"time\" 2e-9").expect("write");
    writeln!(temp, "\"V(out)\" 1.0").expect("write");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("psf ascii read should work");
    assert_eq!(dataset.point_count(), 3);
    assert_eq!(dataset.signal_count(), 1);
    assert_eq!(dataset.analysis, "Transient");
    assert_eq!(
        dataset.x_signal.as_ref().expect("x").data,
        vec![0.0, 1e-9, 2e-9]
    );
    assert_eq!(
        dataset.get_signal("V(out)").expect("v(out)").data,
        vec![0.0, 0.5, 1.0]
    );
}

#[test]
fn test_read_psf_ascii_directory_from_log_file_reference() {
    let dir = tempfile::tempdir().expect("temp dir");
    let dataset_file = dir.path().join("tran.psfascii");
    std::fs::write(
        &dataset_file,
        "\"time\" 0\n\"V(out)\" 0\n\"time\" 1\n\"V(out)\" 2\n",
    )
    .expect("write dataset");
    std::fs::write(dir.path().join("logFile"), "\"tran.psfascii\"\n").expect("write logFile");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(dir.path())
        .expect("directory psf read should work");
    assert_eq!(dataset.point_count(), 2);
    assert_eq!(dataset.signal_count(), 1);
    assert_eq!(
        dataset.metadata.get("format").map(String::as_str),
        Some("psf-ascii")
    );
    assert_eq!(
        dataset.get_signal("V(out)").expect("v(out) present").data,
        vec![0.0, 2.0]
    );
}

#[test]
fn test_read_psf_ascii_complex_pair_expands_real_imag() {
    let mut temp = Builder::new()
        .suffix(".psfascii")
        .tempfile()
        .expect("temp psf ascii");
    writeln!(temp, "\"freq\" 1e3").expect("write");
    writeln!(temp, "\"S11\" (0.1 -0.2)").expect("write");
    writeln!(temp, "\"freq\" 2e3").expect("write");
    writeln!(temp, "\"S11\" (0.3 -0.4)").expect("write");

    let dataset = WaveformReader::new(WaveformFormat::Psf)
        .read(temp.path())
        .expect("psf ascii read should work");
    assert_eq!(dataset.point_count(), 2);
    assert!(
        dataset.get_signal("S11_RE").is_some(),
        "complex tuple should emit real trace"
    );
    assert!(
        dataset.get_signal("S11_IM").is_some(),
        "complex tuple should emit imag trace"
    );
    assert_eq!(
        dataset.get_signal("S11_RE").expect("re").data,
        vec![0.1, 0.3]
    );
    assert_eq!(
        dataset.get_signal("S11_IM").expect("im").data,
        vec![-0.2, -0.4]
    );
}
