//! Application policy and presentation for decoded VCD/FST results.

use super::*;
use rspice_formats::digital::{DecodedDigital, DigitalImportLimits};

fn digital_import_limits() -> DigitalImportLimits {
    DigitalImportLimits {
        max_bytes: MAX_RESULT_DATASET_BYTES,
        samples: waveform_import_limits(),
    }
}

fn present_digital(format: ResultImportFormat, decoded: DecodedDigital) -> ParsedResultDataset {
    let mut parsed = present_imported_waveforms(format, AnalysisType::Transient, decoded.data);
    parsed.notes = decoded.notes;
    parsed.event_payload = decoded.event_payload;
    parsed
}

pub(in crate::workbench::workflows) fn parse_vcd(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    rspice_formats::digital::decode_vcd(bytes, format, digital_import_limits())
        .map(|decoded| present_digital(format, decoded))
}

pub(in crate::workbench::workflows) fn parse_fst(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    rspice_formats::digital::decode_fst(bytes, format, digital_import_limits())
        .map(|decoded| present_digital(format, decoded))
}

#[cfg(test)]
pub(super) fn preflight_fst_for_test(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<(), String> {
    rspice_formats::fst::preflight_fst(
        bytes,
        rspice_formats::fst::FstLimits {
            max_bytes: MAX_RESULT_DATASET_BYTES,
            max_columns: MAX_RESULT_COLUMNS,
            max_rows: MAX_RESULT_ROWS,
            max_values: MAX_RESULT_VALUES,
            max_signal_name_bytes: MAX_SIGNAL_NAME_BYTES,
        },
        format.canonical_id(),
    )
    .map(drop)
}
