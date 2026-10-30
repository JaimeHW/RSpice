//! Value Change Dump publication.
//!
//! A dump is the transient's *event* timelines, not its waveform table: the
//! sparse schedule the event solver accepted, at the times it accepted it. It
//! is therefore written from the retained `TransientEvents` evidence and never
//! from the analog grid, and a result that carries no event history is refused
//! by name rather than handed an empty file.
//!
//! # Agreement with the command line
//!
//! `rspice run -f vcd` and this export are the same bytes for the same run.
//! Both project through [`rspice_core::execution::event_vcd_document`] under one `$scope module
//! events` and both serialise with [`rspice_core::io::write_vcd`], so the timescale rule,
//! identifier assignment and byte layout are the core's single implementation
//! rather than two that happen to agree today.
//!
//! # What a dump does not carry
//!
//! VCD has four bit states and no drive strength, so the twelve XSPICE
//! resolved states collapse onto `0`, `1`, `x` and `z`: a resistive one and a
//! strong one are both `1`. That is a property of the format, not of this
//! encoder, and it is stated to the reader on every successful publication.
//! A reader who needs the strength band wants the RSpice Result Bundle.

use super::{NO_ACTIVE_ANALYSIS_MESSAGE, note_result_export_failure, note_result_export_success};
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

use rspice_formats::vcd::encode_result_vcd;

/// VCD is ASCII text. There is no registered media type for it, and claiming
/// one would be an invention rather than a fact about the bytes.
const MIME_TYPE: &str = "text/plain;charset=utf-8";

pub(super) fn export_vcd(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
) {
    let prepared = match displayed.primary_analysis(state.results_reads()) {
        Some(analysis) => encode_result_vcd(analysis)
            .map_err(|error| error.to_string())
            .and_then(|mut prepared| {
                prepared.bytes = super::admit_result_export_bytes(prepared.bytes)?;
                Ok(prepared)
            }),
        None => Err(NO_ACTIVE_ANALYSIS_MESSAGE.to_owned()),
    };
    let prepared = match prepared {
        Ok(prepared) => prepared,
        Err(message) => {
            state.prompt(message);
            return;
        }
    };

    let (published_path, export) = match io.show_save_dialog(SaveDialogConfig {
        title: "Export Value Change Dump",
        default_name: "events.vcd",
        filter_name: "Value Change Dump",
        filter_extensions: &["vcd"],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, "vcd");
            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_bytes_file_observed(&destination, &prepared.bytes, MIME_TYPE)
            });
            (path, export)
        }
        Ok(None) => return,
        Err(error) => (std::path::PathBuf::from("events.vcd"), Err(error)),
    };
    match export {
        Ok(()) => {
            let detail = format!(
                "{} event nodes, {} changes; four bit states, XSPICE drive strength dropped \
                 because VCD has no strength band",
                prepared.node_count, prepared.change_count
            );
            note_result_export_success(
                state,
                "VCD",
                crate::workbench::workflows::export_workflow::export_completion_message(
                    "VCD",
                    &published_path,
                    Some(detail),
                    io,
                ),
            );
        }
        Err(error) => {
            note_result_export_failure(state, format!("VCD export failed: {error}"));
            state.report_failure("VCD export failed", error.to_string());
        }
    }
}
