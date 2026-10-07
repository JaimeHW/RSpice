//! Carry table provenance in inert RAW options, never executable commands.
//!
//! Hex chunks use only an identifier and an inert string value, and keep each
//! header line short for foreign readers with fixed-size line buffers.

use super::*;

const PREFIX: &str = "rspice_table_v";
const CHUNK_BYTES: usize = 128;

pub(super) fn write<W: std::io::Write + ?Sized, T: serde::Serialize>(
    writer: &mut W,
    version: u8,
    metadata: &T,
) -> std::io::Result<()> {
    let bytes = serde_json::to_vec(metadata).map_err(std::io::Error::other)?;
    let count = bytes.len().div_ceil(CHUNK_BYTES);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for (index, chunk) in bytes.chunks(CHUNK_BYTES).enumerate() {
        write!(writer, "Option: {PREFIX}{version}_{index}_{count} = x")?;
        let mut encoded = [0; CHUNK_BYTES * 2];
        for (byte, encoded) in chunk.iter().zip(encoded.chunks_exact_mut(2)) {
            encoded[0] = HEX[(byte >> 4) as usize];
            encoded[1] = HEX[(byte & 15) as usize];
        }
        writer.write_all(&encoded[..chunk.len() * 2])?;
        writeln!(writer)?;
    }
    Ok(())
}

#[derive(Default)]
pub(in super::super) struct MetadataOptions {
    version: Option<u8>,
    count: usize,
    next: usize,
    bytes: Vec<u8>,
}

impl MetadataOptions {
    pub(in super::super) fn read(&mut self, value: &str) -> Result<(), RawParseError> {
        if !value.starts_with(PREFIX) {
            return Ok(());
        }
        let invalid =
            || RawParseError::InvalidHeader("invalid RAW table metadata option chunks".into());
        let (name, encoded) = value.split_once('=').ok_or_else(invalid)?;
        let mut parts = name
            .trim()
            .strip_prefix(PREFIX)
            .ok_or_else(invalid)?
            .split('_');
        let version: u8 = parts
            .next()
            .and_then(|part| part.parse().ok())
            .ok_or_else(invalid)?;
        if !(1..=4).contains(&version) {
            return Err(RawParseError::UnsupportedFormat(
                "unsupported RAW table metadata version".into(),
            ));
        }
        let index: usize = parts
            .next()
            .and_then(|part| part.parse().ok())
            .ok_or_else(invalid)?;
        let count: usize = parts
            .next()
            .and_then(|part| part.parse().ok())
            .ok_or_else(invalid)?;
        if parts.next().is_some()
            || count == 0
            || index != self.next
            || index >= count
            || self
                .version
                .is_some_and(|previous| previous != version || self.count != count)
        {
            return Err(invalid());
        }
        let encoded = encoded.trim().strip_prefix('x').ok_or_else(invalid)?;
        if encoded.is_empty() || encoded.len() > CHUNK_BYTES * 2 || !encoded.len().is_multiple_of(2)
        {
            return Err(invalid());
        }
        self.bytes.try_reserve(encoded.len() / 2).map_err(|error| {
            RawParseError::DataError(format!("unable to retain RAW table metadata: {error}"))
        })?;
        for pair in encoded.as_bytes().chunks_exact(2) {
            let high = (pair[0] as char).to_digit(16).ok_or_else(invalid)?;
            let low = (pair[1] as char).to_digit(16).ok_or_else(invalid)?;
            self.bytes.push((high * 16 + low) as u8);
        }
        self.version = Some(version);
        self.count = count;
        self.next += 1;
        Ok(())
    }

    pub(in super::super) fn finish(self, header: &mut RawFileHeader) -> Result<(), RawParseError> {
        let Some(version) = self.version else {
            return Ok(());
        };
        if self.next != self.count || !header.command.is_empty() {
            return Err(RawParseError::InvalidHeader(
                "incomplete or conflicting RAW table metadata".into(),
            ));
        }
        let metadata = String::from_utf8(self.bytes).map_err(|error| {
            RawParseError::InvalidHeader(format!("RAW table metadata is not UTF-8: {error}"))
        })?;
        // Retain the established in-memory representation for all consumers.
        // Reading this field never executes it; newly written files use Options.
        header.command = format!("RSpiceTableV{version} {metadata}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Flags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 value voltage\nValues:\n0 0 1\n";

    fn metadata() -> (String, String) {
        let name = "α \"; $name `echo literal` ".repeat(40);
        let mut output = Vec::new();
        super::super::write_raw_table_layout_metadata(
            &mut output,
            &[],
            &[None, None],
            Some((
                name.as_str(),
                &[("time", "time"), (name.as_str(), "voltage")],
            )),
        )
        .unwrap();
        (String::from_utf8(output).unwrap(), name)
    }

    #[test]
    fn metadata_uses_short_inert_options_and_survives_multiple_plots() {
        let (metadata, name) = metadata();
        assert!(metadata.lines().count() > 1);
        for line in metadata.lines() {
            assert!(line.starts_with("Option: rspice_table_v4_"));
            assert!(line.len() < 320);
            let (_, encoded) = line.split_once(" = x").unwrap();
            assert!(encoded.bytes().all(|byte| byte.is_ascii_hexdigit()));
        }
        assert!(!metadata.contains("Command:"));
        let plot = format!(
            "Title: escaped\nPlotname: escaped\nOption: ordinary = value\n{metadata}{DATA}"
        );
        let file = parse_raw_plots_reader_with_limits(
            &mut std::io::Cursor::new(plot.repeat(2)),
            ResourceLimits::default(),
        )
        .unwrap();
        assert_eq!(file.plots.len(), 2);
        for plot in file.plots {
            assert_eq!(plot.header.plotname, name);
            assert_eq!(plot.variables[1].name, name);
            assert!(super::super::raw_table_has_coordinate(&plot.header).unwrap());
        }
    }

    #[test]
    fn incomplete_conflicting_reordered_and_malformed_chunks_are_rejected() {
        let (metadata, _) = metadata();
        let chunks: Vec<_> = metadata.lines().collect();
        let mut swapped = chunks.clone();
        swapped.swap(0, 1);
        let cases = [
            chunks[1..].join("\n"),
            chunks[..chunks.len() - 1].join("\n"),
            swapped.join("\n"),
            format!("{}\n{metadata}", chunks[0]),
            metadata.replacen("rspice_table_v4_", "rspice_table_v99_", 1),
            metadata.replacen(" = x", " = xz", 1),
            format!("Command: other metadata\n{metadata}"),
            "Option: rspice_table_v4_0_0 = x7b7d".into(),
            "Option: rspice_table_v4_0_1 = xff".into(),
            "Option: rspice_table_v4_0_1 = x".into(),
        ];
        for (index, metadata) in cases.iter().enumerate() {
            let source = format!("Title: invalid\nPlotname: invalid\n{metadata}\n{DATA}");
            assert!(
                parse_raw_reader(&mut std::io::Cursor::new(source)).is_err(),
                "case {index}"
            );
        }
    }
}
