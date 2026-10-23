//! Carry result provenance in inert RAW options, never executable commands.
//!
//! Hex chunks use only an identifier and an inert string value, and keep each
//! header line short for foreign readers with fixed-size line buffers.

use super::*;

const TABLE_PREFIX: &str = "rspice_table_v";
const METADATA_PREFIX: &str = "rspice_metadata_v";
const CHUNK_BYTES: usize = 128;

#[derive(Clone, Copy, PartialEq, Eq)]
enum MetadataKind {
    Table(u8),
    Opaque,
}

/// Store opaque UTF-8 result metadata in bounded, inert RAW Option chunks.
/// The reader restores it verbatim to `RawFileHeader::command`, preserving
/// existing result-specific decoders without emitting executable Command lines.
/// The metadata must be nonempty; schema validation belongs to its consumer.
pub fn write_raw_metadata_options<W: std::io::Write + ?Sized>(
    writer: &mut W,
    metadata: &str,
) -> std::io::Result<()> {
    write_chunks(writer, MetadataKind::Opaque, metadata.as_bytes())
}

pub(super) fn write_table<W: std::io::Write + ?Sized, T: serde::Serialize>(
    writer: &mut W,
    version: u8,
    metadata: &T,
) -> std::io::Result<()> {
    let bytes = serde_json::to_vec(metadata).map_err(std::io::Error::other)?;
    write_chunks(writer, MetadataKind::Table(version), &bytes)
}

fn write_chunks<W: std::io::Write + ?Sized>(
    writer: &mut W,
    kind: MetadataKind,
    bytes: &[u8],
) -> std::io::Result<()> {
    if bytes.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "RAW metadata must not be empty",
        ));
    }
    let (prefix, version) = match kind {
        MetadataKind::Table(version) => (TABLE_PREFIX, version),
        MetadataKind::Opaque => (METADATA_PREFIX, 1),
    };
    let count = bytes.len().div_ceil(CHUNK_BYTES);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for (index, chunk) in bytes.chunks(CHUNK_BYTES).enumerate() {
        write!(writer, "Option: {prefix}{version}_{index}_{count} = x")?;
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
pub(super) struct MetadataOptions {
    kind: Option<MetadataKind>,
    count: usize,
    next: usize,
    bytes: Vec<u8>,
}

impl MetadataOptions {
    pub(super) fn read(&mut self, value: &str) -> Result<(), RawParseError> {
        let (suffix, table) = if let Some(suffix) = value.strip_prefix(TABLE_PREFIX) {
            (suffix, true)
        } else if let Some(suffix) = value.strip_prefix(METADATA_PREFIX) {
            (suffix, false)
        } else {
            return Ok(());
        };
        let invalid = || RawParseError::InvalidHeader("invalid RAW metadata option chunks".into());
        let (name, encoded) = suffix.split_once('=').ok_or_else(invalid)?;
        let mut parts = name.trim().split('_');
        let version: u8 = parts
            .next()
            .and_then(|part| part.parse().ok())
            .ok_or_else(invalid)?;
        if (table && !(1..=4).contains(&version)) || (!table && version != 1) {
            return Err(RawParseError::UnsupportedFormat(
                "unsupported RAW metadata option version".into(),
            ));
        }
        let kind = if table {
            MetadataKind::Table(version)
        } else {
            MetadataKind::Opaque
        };
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
                .kind
                .is_some_and(|previous| previous != kind || self.count != count)
        {
            return Err(invalid());
        }
        let encoded = encoded.trim().strip_prefix('x').ok_or_else(invalid)?;
        if encoded.is_empty() || encoded.len() > CHUNK_BYTES * 2 || !encoded.len().is_multiple_of(2)
        {
            return Err(invalid());
        }
        self.bytes.try_reserve(encoded.len() / 2).map_err(|error| {
            RawParseError::DataError(format!("unable to retain RAW metadata: {error}"))
        })?;
        for pair in encoded.as_bytes().chunks_exact(2) {
            let high = (pair[0] as char).to_digit(16).ok_or_else(invalid)?;
            let low = (pair[1] as char).to_digit(16).ok_or_else(invalid)?;
            self.bytes.push((high * 16 + low) as u8);
        }
        self.kind = Some(kind);
        self.count = count;
        self.next += 1;
        Ok(())
    }

    pub(super) fn finish(self, header: &mut RawFileHeader) -> Result<(), RawParseError> {
        let Some(kind) = self.kind else {
            return Ok(());
        };
        if self.next != self.count || !header.command.is_empty() {
            return Err(RawParseError::InvalidHeader(
                "incomplete or conflicting RAW metadata".into(),
            ));
        }
        let metadata = String::from_utf8(self.bytes).map_err(|error| {
            RawParseError::InvalidHeader(format!("RAW metadata is not UTF-8: {error}"))
        })?;
        // Retain the established in-memory representation for all consumers.
        // Reading this field never executes it; newly written files use Options.
        header.command = match kind {
            MetadataKind::Table(version) => format!("RSpiceTableV{version} {metadata}"),
            MetadataKind::Opaque => metadata,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Flags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 value voltage\nValues:\n0 0 1\n";

    #[test]
    fn opaque_metadata_preserves_utf8_and_control_characters_in_both_encodings() {
        let metadata = "α \"; $literal `text`\r\n\0 ".repeat(50);
        for binary in [false, true] {
            let mut bytes = b"Title: metadata\nPlotname: metadata\n".to_vec();
            write_raw_metadata_options(&mut bytes, &metadata).unwrap();
            let header = std::str::from_utf8(&bytes).unwrap();
            assert!(!header.contains("Command:"));
            for line in header.lines().filter(|line| line.starts_with("Option:")) {
                assert!(line.starts_with("Option: rspice_metadata_v1_"));
                assert!(line.len() < 320);
                let (_, encoded) = line.split_once(" = x").unwrap();
                assert!(encoded.bytes().all(|byte| byte.is_ascii_hexdigit()));
            }
            if binary {
                bytes.extend_from_slice(b"Flags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 value voltage\nBinary:\n");
                bytes.extend_from_slice(&0.0_f64.to_le_bytes());
                bytes.extend_from_slice(&1.0_f64.to_le_bytes());
            } else {
                bytes.extend_from_slice(DATA.as_bytes());
            }
            let parsed = parse_raw_reader(&mut std::io::Cursor::new(bytes)).unwrap();
            assert_eq!(parsed.header.command, metadata);
            assert_eq!(parsed.waveforms[1].y, vec![1.0]);
            assert!(!raw_table_has_coordinate(&parsed.header).unwrap());
        }
        let mut output = Vec::new();
        assert!(write_raw_metadata_options(&mut output, "").is_err());
        assert!(output.is_empty());
    }

    #[test]
    fn opaque_metadata_rejects_mixed_families_and_unsupported_versions() {
        let mut output = Vec::new();
        write_raw_metadata_options(&mut output, &"metadata".repeat(100)).unwrap();
        let metadata = String::from_utf8(output).unwrap();
        let mixed = metadata.replacen("rspice_metadata_v1_", "rspice_table_v1_", 1);
        let future = metadata.replacen("rspice_metadata_v1_", "rspice_metadata_v2_", 1);
        for metadata in [
            mixed,
            future,
            "Option: rspice_metadata_v1_0_1 = xff\n".into(),
        ] {
            let source = format!("Title: invalid\nPlotname: invalid\n{metadata}{DATA}");
            assert!(parse_raw_reader(&mut std::io::Cursor::new(source)).is_err());
        }
    }

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
