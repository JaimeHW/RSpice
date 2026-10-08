use super::*;
use rustyhdf5::{AttrValue, FileBuilder};

type Column<'a> = (&'a str, &'a str, Option<&'a str>, &'a [f64]);

#[test]
fn generic_sweep_coordinates_require_an_explicit_dc_analysis() {
    for (analysis, coordinate_type, accepted) in [
        (Some("dc_sweep"), "value", true),
        (Some("dc_sweep"), "index", false),
        (Some("transient"), "value", false),
        (Some("report"), "value", false),
        (None, "value", false),
    ] {
        let mut file = FileBuilder::new();
        let mut group = file.create_group("selected");
        for (key, value) in [
            ("section_type", "table"),
            ("coordinate_type", coordinate_type),
            ("independent_name", "control"),
            ("signal_0000_name", "out"),
            ("signal_0000_type", "value"),
        ] {
            group.set_attr(key, AttrValue::String(value.into()));
        }
        if let Some(analysis) = analysis {
            group.set_attr("analysis", AttrValue::String(analysis.into()));
        }
        group.set_attr("signal_count", AttrValue::I64(1));
        group
            .create_dataset("independent")
            .with_f64_data(&[1.0, 2.0]);
        group
            .create_dataset("signal_0000")
            .with_f64_data(&[3.0, 4.0]);
        file.add_group(group.finish());
        let result = decode_hdf5(
            &file.finish().unwrap(),
            Hdf5Limits {
                max_columns: 4,
                max_values: 16,
                coordinate_names: &["time", "frequency"],
            },
            "hdf5",
        );
        if accepted {
            let decoded = result.unwrap();
            assert_eq!(decoded.domain, crate::WaveformDomain::DcSweep);
            assert_eq!(decoded.coordinate_name, "control");
            assert_eq!(decoded.coordinate_unit, None);
            assert_eq!(decoded.coordinate, [1.0, 2.0]);
        } else {
            assert!(result.is_err(), "{analysis:?} {coordinate_type}");
        }
    }
}

fn section_container(sections: &[(&str, Option<AttrValue>)]) -> Vec<u8> {
    let mut file = FileBuilder::new();
    for (name, kind) in sections {
        let mut group = file.create_group(name);
        if let Some(kind) = kind {
            group.set_attr("section_type", kind.clone());
        }
        group.set_attr("independent_name", AttrValue::String("time".into()));
        group.set_attr("signal_count", AttrValue::I64(1));
        group.set_attr("signal_0000_name", AttrValue::String("V(out)".into()));
        group
            .create_dataset("independent")
            .with_f64_data(&[0.0, 1.0]);
        group
            .create_dataset("signal_0000")
            .with_f64_data(&[2.0, 3.0]);
        file.add_group(group.finish());
    }
    file.finish().unwrap()
}

#[test]
fn unsupported_sections_are_never_silently_skipped_beside_supported_waveforms() {
    let limits = Hdf5Limits {
        max_columns: 4,
        max_values: 16,
        coordinate_names: &["time"],
    };
    for kind in [
        "noise",
        "operating_point",
        "distortion",
        "fft",
        "future_result",
    ] {
        let bytes = section_container(&[
            ("tran1", Some(AttrValue::String("transient".into()))),
            ("another_result", Some(AttrValue::String(kind.into()))),
        ]);
        let error = decode_hdf5(&bytes, limits, "hdf5").unwrap_err();
        assert!(
            matches!(&error.reason, Hdf5ReadFailure::MultipleSections(names)
            if names.len() == 2 && names.iter().any(|name| name == "another_result")),
            "{error}"
        );
    }
    let legacy = section_container(&[("transient", None), ("noise", None)]);
    assert!(matches!(
        decode_hdf5(&legacy, limits, "hdf5").unwrap_err().reason,
        Hdf5ReadFailure::MultipleSections(_)
    ));
}

#[test]
fn invalid_and_unsupported_section_declarations_cannot_fall_back_to_a_waveform() {
    let limits = Hdf5Limits {
        max_columns: 4,
        max_values: 16,
        coordinate_names: &["time"],
    };
    for kind in [
        "noise",
        "operating_point",
        "distortion",
        "fft",
        "future_result",
    ] {
        let bytes = section_container(&[("transient", Some(AttrValue::String(kind.into())))]);
        let error = decode_hdf5(&bytes, limits, "hdf5").unwrap_err();
        assert!(
            matches!(&error.reason, Hdf5ReadFailure::UnsupportedSection { section, kind: actual }
            if section == "transient" && actual == kind),
            "{error}"
        );
    }
    for kind in [AttrValue::I64(1), AttrValue::String("".into())] {
        let bytes = section_container(&[("transient", Some(kind))]);
        assert!(
            matches!(decode_hdf5(&bytes, limits, "hdf5").unwrap_err().reason,
            Hdf5ReadFailure::StringAttribute { name, .. } if name == "section_type")
        );
    }
    let missing_coordinate =
        section_container(&[("converted", Some(AttrValue::String("table".into())))]);
    assert!(
        matches!(decode_hdf5(&missing_coordinate, limits, "hdf5").unwrap_err().reason,
        Hdf5ReadFailure::MissingAttribute { name } if name == "coordinate_type")
    );
}

#[test]
fn ordinary_metadata_groups_do_not_count_as_result_sections() {
    let limits = Hdf5Limits {
        max_columns: 4,
        max_values: 16,
        coordinate_names: &["time"],
    };
    let legacy = section_container(&[("transient", None), ("metadata", None)]);
    let decoded = decode_hdf5(&legacy, limits, "hdf5").unwrap();
    assert_eq!(decoded.coordinate, [0.0, 1.0]);
    assert_eq!(decoded.signals[0].real, [2.0, 3.0]);

    let mut file = FileBuilder::new();
    file.create_dataset("time").with_f64_data(&[0.0, 1.0]);
    file.create_dataset("V(out)").with_f64_data(&[2.0, 3.0]);
    let mut metadata = file.create_group("metadata");
    metadata.set_attr("title", AttrValue::String("Generic root table".into()));
    file.add_group(metadata.finish());
    let decoded = decode_hdf5(&file.finish().unwrap(), limits, "hdf5").unwrap();
    assert_eq!(decoded.coordinate, [0.0, 1.0]);
    assert_eq!(decoded.signals[0].real, [2.0, 3.0]);
}

fn read_table(
    columns: &[Column<'_>],
    coordinate_unit: Option<AttrValue>,
) -> Result<DecodedNumericDataset, Hdf5ReadError> {
    let mut file = FileBuilder::new();
    let mut group = file.create_group("converted");
    for (key, value) in [
        ("section_type", "table"),
        ("coordinate_type", "time"),
        ("independent_name", "time"),
    ] {
        group.set_attr(key, AttrValue::String(value.to_owned()));
    }
    if let Some(unit) = coordinate_unit {
        group.set_attr("coordinate_unit", unit);
    }
    group.set_attr("signal_count", AttrValue::I64(columns.len() as i64));
    group
        .create_dataset("independent")
        .with_f64_data(&[0.0, 1.0]);
    for (index, (name, kind, unit, values)) in columns.iter().enumerate() {
        let prefix = format!("signal_{index:04}");
        group.set_attr(
            &format!("{prefix}_name"),
            AttrValue::String((*name).to_owned()),
        );
        group.set_attr(
            &format!("{prefix}_type"),
            AttrValue::String((*kind).to_owned()),
        );
        if let Some(unit) = unit {
            group.set_attr(
                &format!("{prefix}_unit"),
                AttrValue::String((*unit).to_owned()),
            );
        }
        group.create_dataset(&prefix).with_f64_data(values);
    }
    file.add_group(group.finish());
    decode_hdf5(
        &file.finish().unwrap(),
        Hdf5Limits {
            max_columns: 8,
            max_values: 64,
            coordinate_names: &["time", "frequency"],
        },
        "hdf5",
    )
}

#[test]
fn nullable_masks_require_exact_padding_flags_lengths_and_matching_declarations() {
    let values = [0.0, 1.0];
    let flags = [0.0, 1.0];
    let value = ("out", "nullable_real:voltage", Some("V"), values.as_slice());
    let mask = (
        "mask",
        "nullable_validity:voltage",
        Some("1"),
        flags.as_slice(),
    );
    let decoded = read_table(&[value, mask], None).unwrap();
    assert!(decoded.signals[0].real[0].is_nan());
    assert_eq!(decoded.signals[0].real[1], 1.0);
    assert_eq!(decoded.signals.len(), 1);
    for (real, validity) in [
        (vec![1.0, 1.0], vec![0.0, 1.0]),
        (vec![0.0, 1.0], vec![0.5, 1.0]),
        (vec![0.0, 1.0], vec![0.0]),
        (vec![f64::NAN, 1.0], vec![0.0, 1.0]),
        (vec![0.0, f64::INFINITY], vec![0.0, 1.0]),
    ] {
        assert!(
            read_table(
                &[
                    (value.0, value.1, value.2, &real),
                    (mask.0, mask.1, mask.2, &validity)
                ],
                None
            )
            .is_err()
        );
    }
    for (kind, unit) in [
        ("nullable_validity:current", Some("1")),
        ("nullable_validity:voltage", None),
        ("voltage", Some("1")),
    ] {
        assert!(read_table(&[value, (mask.0, kind, unit, &flags)], None).is_err());
    }
    assert!(read_table(&[value], None).is_err());
    assert!(read_table(&[mask], None).is_err());
}

#[test]
fn complex_nullable_samples_share_availability_and_component_units() {
    let real = (
        "Re(out)",
        "complex_real:nullable_complex:voltage",
        Some("V"),
        &[0.0, 2.0][..],
    );
    let imag = (
        "Im(out)",
        "complex_imag:nullable_complex:voltage",
        Some("V"),
        &[0.0, -0.0][..],
    );
    let mask = (
        "mask",
        "nullable_validity:voltage",
        Some("1"),
        &[0.0, 1.0][..],
    );
    let decoded = read_table(&[real, imag, mask], None).unwrap();
    let signal = &decoded.signals[0];
    assert_eq!(signal.name, "out");
    assert_eq!(signal.real[1], 2.0);
    assert!(signal.real[0].is_nan());
    assert!(signal.imag.as_ref().unwrap()[0].is_nan());
    assert_eq!(
        signal.imag.as_ref().unwrap()[1].to_bits(),
        (-0.0_f64).to_bits()
    );
    for (kind, unit, values) in [
        (imag.1, Some("A"), &[0.0, 0.0][..]),
        (imag.1, imag.2, &[1.0, 0.0][..]),
        (imag.1, imag.2, &[0.0][..]),
        (
            "complex_imag:nullable_complex:current",
            imag.2,
            &[0.0, 0.0][..],
        ),
    ] {
        assert!(read_table(&[real, (imag.0, kind, unit, values), mask], None).is_err());
    }
    assert!(read_table(&[real, mask], None).is_err());
    assert!(read_table(&[imag, mask], None).is_err());
}

#[test]
fn declared_coordinate_units_cannot_be_ignored_when_invalid() {
    let columns = [("out", "voltage", Some("V"), &[1.0, 2.0][..])];
    assert!(
        read_table(&columns, None)
            .unwrap()
            .coordinate_unit
            .is_none()
    );
    for unit in [
        AttrValue::I64(1),
        AttrValue::String("".into()),
        AttrValue::String("V".into()),
        AttrValue::String("unknown".into()),
    ] {
        assert!(read_table(&columns, Some(unit)).is_err());
    }
    let decoded = read_table(&columns, Some(AttrValue::String("ns".into()))).unwrap();
    assert_eq!(decoded.coordinate, [0.0, 1e-9]);
    assert_eq!(decoded.coordinate_unit.as_deref(), Some("s"));
}
