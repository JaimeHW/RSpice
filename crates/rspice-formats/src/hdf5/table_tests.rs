use super::*;
use rustyhdf5::{AttrValue, FileBuilder};

type Column<'a> = (&'a str, &'a str, Option<&'a str>, &'a [f64]);

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
