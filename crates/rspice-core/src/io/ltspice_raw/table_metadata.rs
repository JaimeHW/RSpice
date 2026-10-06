//! Optional RSpice provenance for real columns in an otherwise complex RAW plot.
//!
//! RAW's numeric encoding is plot-wide. A standard Command header records the
//! original real columns so reopening does not turn frequencies or indices into
//! complex signals. Other readers can ignore the header and read ordinary RAW.
use super::*;

const PREFIX: &str = "RSpiceTableV1 ";
const UNITS_PREFIX: &str = "RSpiceTableV2 ";

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Metadata {
    real_variables: Vec<usize>,
    #[serde(default)]
    units: Option<Vec<Option<String>>>,
}

/// Write table provenance including explicit unit symbols in RAW variable order.
/// V2 retains the V1 real-column contract and adds case-sensitive unit metadata.
pub fn write_raw_table_metadata_with_units<W: std::io::Write + ?Sized>(
    writer: &mut W,
    real_variables: &[usize],
    units: &[Option<String>],
) -> std::io::Result<()> {
    write!(writer, "Command: {UNITS_PREFIX}")?;
    serde_json::to_writer(
        &mut *writer,
        &serde_json::json!({
            "real_variables": real_variables,
            "units": units,
        }),
    )
    .map_err(std::io::Error::other)?;
    writeln!(writer)
}

fn read_metadata(header: &RawFileHeader) -> Result<Option<Metadata>, RawParseError> {
    let (encoded, with_units) = if let Some(encoded) = header.command.strip_prefix(PREFIX) {
        (encoded, false)
    } else if let Some(encoded) = header.command.strip_prefix(UNITS_PREFIX) {
        (encoded, true)
    } else {
        return Ok(None);
    };
    let metadata: Metadata = serde_json::from_str(encoded).map_err(|error| {
        RawParseError::InvalidHeader(format!("invalid RAW table metadata: {error}"))
    })?;
    if with_units != metadata.units.is_some()
        || metadata.units.as_ref().is_some_and(|units| {
            units.len() != header.no_variables
                || units.iter().flatten().any(|unit| unit.trim().is_empty())
        })
    {
        return Err(RawParseError::InvalidHeader(
            "RAW table unit metadata must match its version and variable count and use nonempty symbols".into(),
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

/// Write the optional Command header for a mixed real/complex result table.
/// Indices refer to the RAW variable order, including the independent variable.
pub fn write_raw_table_metadata<W: std::io::Write + ?Sized>(
    writer: &mut W,
    real_variables: &[usize],
) -> std::io::Result<()> {
    #[derive(serde::Serialize)]
    struct MetadataRef<'a> {
        real_variables: &'a [usize],
    }
    write!(writer, "Command: {PREFIX}")?;
    serde_json::to_writer(&mut *writer, &MetadataRef { real_variables })
        .map_err(std::io::Error::other)?;
    writeln!(writer)
}

pub(super) fn restore_real_columns(
    header: &RawFileHeader,
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
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
