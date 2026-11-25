//! Application policy and presentation for decoded VCD/FST results.

use super::*;
use rspice_formats::digital::{DecodedDigital, DigitalImportLimits};

/// A dump expands to at most what its bytes can hold; a declaration past that is damaged.
fn digital_import_limits() -> DigitalImportLimits {
    DigitalImportLimits {
        max_bytes: u64::try_from(source_capacity()).unwrap_or(u64::MAX),
        samples: waveform_import_limits(),
    }
}

fn present_digital(format: ResultImportFormat, decoded: DecodedDigital) -> ParsedResultDataset {
    let mut parsed = present_imported_waveforms(format, AnalysisType::Transient, decoded.data);
    parsed.coordinate_unit = Some("s".to_owned());
    parsed.notes = decoded.notes;
    parsed.event_payload = decoded.event_payload;
    parsed.event_dump = decoded.event_dump;
    parsed
}

pub(in crate::workbench::workflows) fn parse_vcd(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::super::ReadingSource::of(bytes.len());
    rspice_formats::digital::decode_vcd(bytes, format, digital_import_limits())
        .map(|decoded| present_digital(format, decoded))
        .map_err(|error| error.to_string())
}

pub(in crate::workbench::workflows) fn parse_fst(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::super::ReadingSource::of(bytes.len());
    rspice_formats::digital::decode_fst(bytes, format, digital_import_limits())
        .map(|decoded| present_digital(format, decoded))
        .map_err(|error| error.to_string())
}

#[cfg(test)]
pub(super) fn preflight_fst_for_test(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<(), String> {
    let _source = super::super::ReadingSource::of(bytes.len());
    rspice_formats::fst::preflight_fst(
        bytes,
        rspice_formats::fst::FstLimits {
            max_bytes: u64::try_from(source_capacity()).unwrap_or(u64::MAX),
            max_columns: source_capacity(),
            max_rows: source_capacity(),
            max_values: source_capacity(),
            max_signal_name_bytes: MAX_SIGNAL_NAME_BYTES,
        },
        format.label(),
    )
    .map(drop)
    .map_err(|error| error.to_string())
}
