//! Optional RSpice provenance for table column kinds, units and exact labels.
//!
//! RAW's numeric encoding is plot-wide. Standard Option headers record the
//! original column kinds, units, layout and labels. Legacy Command metadata is
//! still read, but never emitted: ngspice executes Command headers when loading.
use super::*;

mod options;
pub(super) use options::MetadataOptions;

const PREFIX: &str = "RSpiceTableV1 ";
const UNITS_PREFIX: &str = "RSpiceTableV2 ";
const TEXT_PREFIX: &str = "RSpiceTableV3 ";
const LAYOUT_PREFIX: &str = "RSpiceTableV4 ";

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Metadata {
    real_variables: Vec<usize>,
    #[serde(default)]
    units: Option<Vec<Option<String>>>,
    #[serde(default)]
    text: Option<TextMetadata>,
    #[serde(default)]
    layout: Option<TableLayout>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum TableLayout {
    CoordinateFirst,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct TextMetadata {
    plot_name: String,
    variables: Vec<(String, String)>,
}

/// Declare an ordinary table whose first variable is its real coordinate.
/// V4 makes the layout authoritative even when the display plot name resembles
/// an operating point, FFT or event carrier. Optional text retains labels that
/// need escaping in the ordinary RAW header; units include the coordinate.
pub fn write_raw_table_layout_metadata<W: std::io::Write + ?Sized>(
    writer: &mut W,
    real_variables: &[usize],
    units: &[Option<String>],
    text: Option<(&str, &[(&str, &str)])>,
) -> std::io::Result<()> {
    #[derive(serde::Serialize)]
    struct TextRef<'a> {
        plot_name: &'a str,
        variables: &'a [(&'a str, &'a str)],
    }
    #[derive(serde::Serialize)]
    struct MetadataRef<'a> {
        real_variables: &'a [usize],
        units: &'a [Option<String>],
        layout: &'static str,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<TextRef<'a>>,
    }
    options::write(
        writer,
        4,
        &MetadataRef {
            real_variables,
            units,
            layout: "coordinate-first",
            text: text.map(|(plot_name, variables)| TextRef {
                plot_name,
                variables,
            }),
        },
    )
}

/// Whether validated table provenance explicitly declares a real first coordinate.
/// Legacy metadata only describes units/column kinds, leaving plot-name inference
/// to the consumer. V4 explicitly takes precedence over that inference.
pub fn raw_table_has_coordinate(header: &RawFileHeader) -> Result<bool, RawParseError> {
    Ok(read_metadata(header)?.is_some_and(|metadata| metadata.layout.is_some()))
}

/// Write exact table labels alongside real-column and unit provenance.
/// V3 keeps the original plot name and `(name, type)` pairs in variable order,
/// including the independent variable. Callers can then use whitespace-free
/// declarations without losing labels that RAW's header syntax cannot express.
pub fn write_raw_table_metadata_with_text<W: std::io::Write + ?Sized>(
    writer: &mut W,
    real_variables: &[usize],
    units: &[Option<String>],
    plot_name: &str,
    variables: &[(&str, &str)],
) -> std::io::Result<()> {
    options::write(
        writer,
        3,
        &serde_json::json!({
            "real_variables": real_variables,
            "units": units,
            "text": {"plot_name": plot_name, "variables": variables},
        }),
    )
}

/// Write table provenance including explicit unit symbols in RAW variable order.
/// V2 retains the V1 real-column contract and adds case-sensitive unit metadata.
pub fn write_raw_table_metadata_with_units<W: std::io::Write + ?Sized>(
    writer: &mut W,
    real_variables: &[usize],
    units: &[Option<String>],
) -> std::io::Result<()> {
    options::write(
        writer,
        2,
        &serde_json::json!({
            "real_variables": real_variables,
            "units": units,
        }),
    )
}

fn read_metadata(header: &RawFileHeader) -> Result<Option<Metadata>, RawParseError> {
    let (encoded, version) = if let Some(encoded) = header.command.strip_prefix(PREFIX) {
        (encoded, 1)
    } else if let Some(encoded) = header.command.strip_prefix(UNITS_PREFIX) {
        (encoded, 2)
    } else if let Some(encoded) = header.command.strip_prefix(TEXT_PREFIX) {
        (encoded, 3)
    } else if let Some(encoded) = header.command.strip_prefix(LAYOUT_PREFIX) {
        (encoded, 4)
    } else if header.command.starts_with("RSpiceTableV") {
        return Err(RawParseError::UnsupportedFormat(
            "unsupported RAW table metadata version".into(),
        ));
    } else {
        return Ok(None);
    };
    let metadata: Metadata = serde_json::from_str(encoded).map_err(|error| {
        RawParseError::InvalidHeader(format!("invalid RAW table metadata: {error}"))
    })?;
    if (version >= 2) != metadata.units.is_some()
        || (version < 3 && metadata.text.is_some())
        || (version == 3 && metadata.text.is_none())
        || (version == 4) != metadata.layout.is_some()
        || (version == 4 && header.is_complex && metadata.real_variables.first() != Some(&0))
        || metadata
            .text
            .as_ref()
            .is_some_and(|text| text.variables.len() != header.no_variables)
        || metadata.units.as_ref().is_some_and(|units| {
            units.len() != header.no_variables
                || units.iter().flatten().any(|unit| unit.trim().is_empty())
        })
    {
        return Err(RawParseError::InvalidHeader(
            "RAW table metadata must match its version and variable count, use nonempty unit symbols and declare a real table coordinate".into(),
        ));
    }
    Ok(Some(metadata))
}

/// Read explicitly stated units; legacy RAW plots have no unit metadata.
pub fn raw_table_units(
    header: &RawFileHeader,
) -> Result<Option<Vec<Option<String>>>, RawParseError> {
    Ok(read_metadata(header)?.and_then(|metadata| metadata.units))
}

/// Write optional metadata for a mixed real/complex result table.
/// Indices refer to the RAW variable order, including the independent variable.
pub fn write_raw_table_metadata<W: std::io::Write + ?Sized>(
    writer: &mut W,
    real_variables: &[usize],
) -> std::io::Result<()> {
    #[derive(serde::Serialize)]
    struct MetadataRef<'a> {
        real_variables: &'a [usize],
    }
    options::write(writer, 1, &MetadataRef { real_variables })
}

pub(super) fn restore_table_metadata(
    header: &mut RawFileHeader,
    variables: &mut [RawVariable],
    waveforms: &mut [RawWaveform],
) -> Result<(), RawParseError> {
    let Some(metadata) = read_metadata(header)? else {
        return Ok(());
    };
    if (!header.is_complex && (metadata.units.is_none() || !metadata.real_variables.is_empty()))
        || metadata
            .real_variables
            .windows(2)
            .any(|indices| indices[0] >= indices[1])
    {
        return Err(RawParseError::InvalidHeader(
            "RAW table metadata needs a complex plot and strictly ordered real variable indices"
                .into(),
        ));
    }
    for index in metadata.real_variables {
        let waveform = waveforms.get_mut(index).ok_or_else(|| {
            RawParseError::InvalidHeader("RAW table metadata names an absent variable".into())
        })?;
        if waveform
            .y_imag
            .as_ref()
            .is_none_or(|imaginary| imaginary.iter().any(|value| *value != 0.0))
        {
            return Err(RawParseError::DataError(format!(
                "RAW table variable '{}' is declared real but has a nonzero imaginary component",
                waveform.name
            )));
        }
        waveform.y_imag = None;
    }
    if let Some(text) = metadata.text {
        header.plotname = text.plot_name;
        for ((variable, waveform), (name, var_type)) in
            variables.iter_mut().zip(waveforms).zip(text.variables)
        {
            waveform.name.clone_from(&name);
            variable.name = name;
            variable.var_type = var_type;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn explicit_table_layout_retains_coordinate_kind_and_optional_text() {
        for complex in [false, true] {
            for with_text in [false, true] {
                let mut bytes = b"Title: table\nPlotname: DC Operating Point\n".to_vec();
                let labels = [(" time ", "time"), (" V(out) ", "voltage")];
                write_raw_table_layout_metadata(
                    &mut bytes,
                    if complex { &[0] } else { &[] },
                    &[None, None],
                    with_text.then_some(("", labels.as_slice())),
                )
                .unwrap();
                let (flags, values) = if complex {
                    ("complex", "0,0 1,2")
                } else {
                    ("real", "0 1")
                };
                writeln!(bytes, "Flags: {flags}\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 {values}").unwrap();
                let parsed = parse_raw_reader(&mut std::io::Cursor::new(bytes)).unwrap();
                assert!(raw_table_has_coordinate(&parsed.header).unwrap());
                assert_eq!(
                    parsed.header.plotname,
                    if with_text { "" } else { "DC Operating Point" }
                );
                assert_eq!(
                    parsed.variables[0].name,
                    if with_text { " time " } else { "time" }
                );
                assert!(parsed.waveforms[0].y_imag.is_none());
                assert_eq!(parsed.waveforms[1].y_imag.is_some(), complex);
            }
        }
    }

    #[test]
    fn layout_metadata_rejects_unsupported_versions_missing_layout_and_complex_coordinates() {
        for (prefix, layout, indices, flags, values) in [
            (
                "RSpiceTableV99 ",
                "\"coordinate-first\"",
                "[]",
                "real",
                "0 1",
            ),
            (LAYOUT_PREFIX, "null", "[]", "real", "0 1"),
            (LAYOUT_PREFIX, "\"unknown\"", "[]", "real", "0 1"),
            (UNITS_PREFIX, "\"coordinate-first\"", "[]", "real", "0 1"),
            (
                LAYOUT_PREFIX,
                "\"coordinate-first\"",
                "[]",
                "complex",
                "0,0 1,2",
            ),
            (
                LAYOUT_PREFIX,
                "\"coordinate-first\"",
                "[0]",
                "complex",
                "0,1 1,2",
            ),
        ] {
            let source = format!(
                "Title: table\nPlotname: table\nCommand: {prefix}{{\"real_variables\":{indices},\"units\":[null,null],\"layout\":{layout}}}\nFlags: {flags}\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 {values}\n"
            );
            assert!(
                parse_raw_reader(&mut std::io::Cursor::new(source)).is_err(),
                "{prefix}{layout} {indices} {values}"
            );
        }
    }

    #[test]
    fn exact_labels_restore_both_variable_descriptors_and_waveforms() {
        for complex in [false, true] {
            let mut bytes = Vec::new();
            writeln!(bytes, "Title: escaped\nPlotname: escaped").unwrap();
            write_raw_table_metadata_with_text(
                &mut bytes,
                if complex { &[0] } else { &[] },
                &[Some("ms".into()), None],
                " plot\r\nname ",
                &[
                    (" time ", "elapsed time"),
                    ("V(α\n\"out\")", "custom\ttype"),
                ],
            )
            .unwrap();
            let (flags, values) = if complex {
                ("complex", "0,0 1,2")
            } else {
                ("real", "0 1")
            };
            writeln!(bytes, "Flags: {flags}\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 escaped_time time\n1 escaped_name value\nValues:\n0 {values}").unwrap();
            let parsed = parse_raw_reader(&mut std::io::Cursor::new(bytes)).unwrap();
            assert_eq!(parsed.header.plotname, " plot\r\nname ");
            assert_eq!(parsed.variables[0].name, " time ");
            assert_eq!(parsed.variables[0].var_type, "elapsed time");
            assert_eq!(parsed.variables[1].name, "V(α\n\"out\")");
            assert_eq!(parsed.variables[1].var_type, "custom\ttype");
            for (variable, waveform) in parsed.variables.iter().zip(&parsed.waveforms) {
                assert_eq!(waveform.name, variable.name);
            }
            assert_eq!(
                raw_table_units(&parsed.header).unwrap(),
                Some(vec![Some("ms".into()), None])
            );
            assert!(parsed.waveforms[0].y_imag.is_none());
            assert_eq!(parsed.waveforms[1].y_imag.is_some(), complex);
        }
    }

    #[test]
    fn text_metadata_requires_its_version_and_complete_variable_order() {
        let valid = serde_json::json!({
            "real_variables": [], "units": [null, "V"],
            "text": {"plot_name": "original", "variables": [["time", "time"], ["V(out)", "voltage"]]},
        });
        let mut cases = vec![(PREFIX, valid.clone()), (UNITS_PREFIX, valid.clone())];
        for value in [
            serde_json::Value::Null,
            serde_json::json!({"plot_name": "original", "variables": [["time", "time"]]}),
            serde_json::json!({"plot_name": "original", "variables": [["time", "time"], ["V(out)", 5]]}),
        ] {
            let mut invalid = valid.clone();
            invalid["text"] = value;
            cases.push((TEXT_PREFIX, invalid));
        }
        let mut missing = valid.clone();
        missing.as_object_mut().unwrap().remove("text");
        cases.push((TEXT_PREFIX, missing));
        let mut missing_units = valid;
        missing_units.as_object_mut().unwrap().remove("units");
        cases.push((TEXT_PREFIX, missing_units));
        for (prefix, metadata) in cases {
            let source = format!(
                "Title: invalid\nPlotname: escaped\nCommand: {prefix}{metadata}\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0 1\n"
            );
            assert!(
                parse_raw_reader(&mut std::io::Cursor::new(source)).is_err(),
                "{prefix}{metadata}"
            );
        }
    }

    #[test]
    fn explicit_units_roundtrip_for_real_and_mixed_tables_and_reject_malformed_metadata() {
        for complex in [false, true] {
            let mut bytes = Vec::new();
            writeln!(bytes, "Title: units\nPlotname: units").unwrap();
            write_raw_table_metadata_with_units(
                &mut bytes,
                if complex { &[0] } else { &[] },
                &[Some("ms".into()), Some("mV".into())],
            )
            .unwrap();
            let (flags, values) = if complex {
                ("complex", "0,0 1,2")
            } else {
                ("real", "0 1")
            };
            writeln!(bytes, "Flags: {flags}\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 {values}").unwrap();
            let parsed = parse_raw_reader(&mut std::io::Cursor::new(bytes)).unwrap();
            assert_eq!(
                raw_table_units(&parsed.header).unwrap(),
                Some(vec![Some("ms".into()), Some("mV".into())])
            );
            assert!(parsed.waveforms[0].y_imag.is_none());
            assert_eq!(parsed.waveforms[1].y_imag.is_some(), complex);
        }
        for metadata in [
            r#"{"real_variables":[],"units":["s"]}"#,
            r#"{"real_variables":[],"units":["s",""]}"#,
            r#"{"real_variables":[],"units":["s",5]}"#,
            r#"{"real_variables":[],"units":null}"#,
            r#"{"real_variables":[0],"units":["s","V"]}"#,
        ] {
            let source = format!(
                "Title: invalid\nPlotname: units\nCommand: {UNITS_PREFIX}{metadata}\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0 1\n"
            );
            assert!(
                parse_raw_reader(&mut std::io::Cursor::new(source)).is_err(),
                "{metadata}"
            );
        }
    }

    #[test]
    fn mixed_tables_restore_real_columns_and_reject_inconsistent_metadata() {
        let make = |indices: &str, imaginary: &str| {
            format!(
                "Title: mixed\nPlotname: mixed\nCommand: {PREFIX}{{\"real_variables\":{indices}}}\nFlags: complex\nNo. Variables: 3\nNo. Points: 1\nVariables:\n0 index index\n1 frequency frequency\n2 V(out) voltage\nValues:\n0 0,0 1000,{imaginary} 1,0\n"
            )
        };
        let parsed = parse_raw_reader(&mut std::io::Cursor::new(make("[0,1]", "0"))).unwrap();
        assert!(parsed.waveforms[0].y_imag.is_none());
        assert!(parsed.waveforms[1].y_imag.is_none());
        assert_eq!(parsed.waveforms[2].y_imag, Some(vec![0.0]));
        for (indices, imaginary) in [("[0,1]", "1"), ("[1,1]", "0"), ("[1,0]", "0"), ("[3]", "0")] {
            assert!(parse_raw_reader(&mut std::io::Cursor::new(make(indices, imaginary))).is_err());
        }
    }
}
