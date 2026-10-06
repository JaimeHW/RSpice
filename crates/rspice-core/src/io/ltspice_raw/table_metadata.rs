//! Optional RSpice provenance for real columns in an otherwise complex RAW plot.
//!
//! RAW's numeric encoding is plot-wide. A standard Command header records the
//! original real columns so reopening does not turn frequencies or indices into
//! complex signals. Other readers can ignore the header and read ordinary RAW.
use super::*;

const PREFIX: &str = "RSpiceTableV1 ";

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Metadata {
    real_variables: Vec<usize>,
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
    let Some(encoded) = header.command.strip_prefix(PREFIX) else {
        return Ok(());
    };
    let metadata: Metadata = serde_json::from_str(encoded).map_err(|error| {
        RawParseError::InvalidHeader(format!("invalid RAW table metadata: {error}"))
    })?;
    if !header.is_complex
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
