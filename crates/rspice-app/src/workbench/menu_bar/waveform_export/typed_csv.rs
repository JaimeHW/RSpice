//! Application filenames and messages for retained result tables.

use super::PreparedTypedResultCsv;
use rspice_formats::result_csv::{TypedCsvSummary, encode_typed_result_csv};

pub(super) fn prepare_typed_result_csv(
    analysis: &crate::state::AnalysisResult,
) -> Option<PreparedTypedResultCsv> {
    let encoded = encode_typed_result_csv(analysis)?;
    let (default_name, detail) = match encoded.summary {
        TypedCsvSummary::Qpss {
            coordinate_count,
            tuple_count,
        } => (
            "qpss-spectrum.csv",
            format!(
                "{} MNA coordinates, {} signed tone tuples; complex Fourier coefficients",
                coordinate_count, tuple_count
            ),
        ),
        TypedCsvSummary::DcMismatch {
            retained_contributors,
            evaluated_contributors,
        } => (
            "dc-mismatch.csv",
            format!(
                "{} of {} exact DC mismatch contributors",
                retained_contributors, evaluated_contributors
            ),
        ),
        TypedCsvSummary::OperatingPoint => (
            "operating-point-contract.csv",
            "exact operating-point execution and retention contract".to_owned(),
        ),
        TypedCsvSummary::PoleZero {
            pole_count,
            pole_status,
            zero_count,
            zero_status,
            gain_retained,
        } => (
            "pole-zero.csv",
            format!(
                "{} poles ({}), {} zeros ({}), DC gain {}",
                pole_count,
                pole_status,
                zero_count,
                zero_status,
                if gain_retained {
                    "retained"
                } else {
                    "unavailable"
                }
            ),
        ),
        TypedCsvSummary::PssFloquet {
            multiplier_count,
            complete,
        } => (
            "pss-floquet-evidence.csv",
            format!(
                "{} with retained qualification evidence",
                floquet_export_count_label(multiplier_count, complete, "PSS Floquet multipliers")
            ),
        ),
        TypedCsvSummary::PstbFloquet {
            mode_count,
            complete,
        } => (
            "pstb-floquet-evidence.csv",
            format!(
                "{} with retained qualification and stability evidence",
                floquet_export_count_label(mode_count, complete, "PSTB Floquet modes")
            ),
        ),
        TypedCsvSummary::Sensitivity { row_count } => (
            "sensitivity.csv",
            format!("{} exact sensitivity rows", row_count),
        ),
        TypedCsvSummary::Scalars { value_count } => (
            "scalar-results.csv",
            format!("{} exact scalar values", value_count),
        ),
        TypedCsvSummary::TransferFunction { value_count } => (
            "transfer-function.csv",
            format!("{value_count} exact transfer-function values"),
        ),
        TypedCsvSummary::Soa {
            rule_count,
            event_count,
        } => (
            "soa-evidence.csv",
            format!(
                "{} evaluated rules, {} warning/violation events",
                rule_count, event_count
            ),
        ),
        TypedCsvSummary::Events {
            node_count,
            event_count,
        } => (
            "event-history.csv",
            format!("{} event nodes, {event_count} committed events", node_count),
        ),
        TypedCsvSummary::CurrentEvents {
            node_count,
            current_history_count,
            impulse_count,
        } => (
            "event-history.csv",
            format!(
                "{} event nodes, {} current histories, {impulse_count} current impulses",
                node_count, current_history_count
            ),
        ),
        TypedCsvSummary::Qpac {
            probe_offset_count,
            tuple_count,
        } => (
            "qpac-response.csv",
            format!(
                "{} probe offsets × {} signed tuples; full complex MNA response and selected differential transfer",
                probe_offset_count, tuple_count
            ),
        ),
        TypedCsvSummary::Qpnoise {
            output_count,
            frequency_count,
            mechanism_count,
        } => (
            "qpnoise-results.csv",
            format!(
                "{} outputs; {} frequencies; {} noise mechanisms; PSD, referral, integration, ranking, covariance and complete adjoints",
                output_count, frequency_count, mechanism_count
            ),
        ),
        TypedCsvSummary::Qpxf {
            source_count,
            input_tuple_count,
            output_frequency_count,
        } => (
            "qpxf-transfers.csv",
            format!(
                "{} sources × {} input tuples × {} output frequencies; unit transfers, sampled delay statuses and complete complex adjoints",
                source_count, input_tuple_count, output_frequency_count
            ),
        ),
    };
    Some(PreparedTypedResultCsv {
        default_name,
        contents: encoded.contents,
        detail,
    })
}

fn floquet_export_count_label(count: usize, complete: bool, noun: &str) -> String {
    if complete {
        format!("{count} complete {noun}")
    } else {
        format!("{count} retained {noun}; completeness unavailable")
    }
}
