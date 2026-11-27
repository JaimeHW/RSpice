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

use super::NO_ACTIVE_ANALYSIS_MESSAGE;
use super::job::ResultExportJob;
use super::members::{ExportFile, publish_export};
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::workflows::export_workflow::ExportWorkflowIo;

use rspice_formats::vcd::{check_result_vcd, write_result_vcd};

/// VCD is ASCII text. There is no registered media type for it, and claiming
/// one would be an invention rather than a fact about the bytes.
const MIME_TYPE: &str = "text/plain;charset=utf-8";

/// A dump is written on a Results job where this process writes the file, as
/// every export of a run is: the history is in memory beside the run, and the
/// file it makes is written out without the frame waiting on it.
pub(super) fn export_vcd(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
) -> Option<ResultExportJob> {
    let Some(analysis) = displayed.primary_analysis(state.results_reads()) else {
        state.prompt(NO_ACTIVE_ANALYSIS_MESSAGE);
        return None;
    };
    if let Err(error) = check_result_vcd(analysis) {
        state.prompt(error.to_string());
        return None;
    }
    let analysis = analysis.clone();
    publish_export(
        state,
        io,
        ExportFile {
            label: "VCD",
            title: "Export Value Change Dump",
            default_name: "events.vcd".to_owned(),
            filter_name: "Value Change Dump",
            extension: "vcd",
            mime_type: MIME_TYPE,
            text: false,
        },
        0,
        move |out| {
            write_result_vcd(&analysis, out)
                .map(|facts| {
                    format!(
                        "{} event nodes, {} changes; four bit states, XSPICE drive strength                          dropped because VCD has no strength band",
                        facts.node_count, facts.change_count
                    )
                })
                .map_err(|error| error.to_string())
        },
    )
}
