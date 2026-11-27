//! SPICE rawfile publication: the displayed analysis as one rawfile plot.
//!
//! The bytes are [`rspice_formats::spice_raw::result`]'s, which writes through
//! the core RAW writer the CLI's `--format raw` uses. A file published here
//! therefore loads in ngspice and LTspice as an ordinary rawfile, and reopens
//! in this product with its units and exact names.
//!
//! # What a rawfile carries
//!
//! One plot per analysis: a transient, an AC sweep as a complex plot of its
//! retained rectangular samples, or a DC sweep. An export of every member of
//! a `.step` sweep or Monte Carlo study writes one plot per member, in family
//! order, each titled by the member it is, as ngspice writes one plot per
//! analysis it runs. The traces are the ones the export
//! publishes, and a transient's digital and real lanes — the plotted ones, or
//! every one when the export writes all signals — follow the analysis plot as
//! the engine's own per-node event plots, so a mixed run's logic travels
//! exactly, strength and all, where a VCD keeps four bit states.
//!
//! # Written as it is read
//!
//! Where the file is one this process writes, the rawfile is written on a
//! Results job as its samples are read, a block of each column at a time, so
//! a run kept in a results store exports whatever its length. A browser
//! download takes the same bytes.

use super::job::ResultExportJob;
use super::members::{ExportFile, OwnedMembers, exported_members, publish_export, read_samples};
use super::{ExportedSignals, event_nodes};
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use crate::workbench::workflows::export_workflow::ExportWorkflowIo;
use crate::workbench::workflows::result_import_workflow::exported_waveform_limits;

use rspice_core::io::RawFormat;
use rspice_formats::spice_raw::result::{RawPlotFacts, check_result_raw, write_result_raw};

const EXTENSION: &str = "raw";

/// No media type is registered for a SPICE rawfile. Binary values are bytes;
/// ASCII values are text, and claiming more than that would be an invention.
const fn mime_type(format: RawFormat) -> &'static str {
    match format {
        RawFormat::Binary => "application/octet-stream",
        RawFormat::Ascii => "text/plain;charset=utf-8",
    }
}

const fn label(format: RawFormat) -> &'static str {
    match format {
        RawFormat::Binary => "SPICE raw",
        RawFormat::Ascii => "SPICE raw (ASCII)",
    }
}

/// Whether the event lane of `node` is on the plot: its `D()` or `E()`
/// column's visibility, the reader's override winning, and a node the run
/// keeps only as event history off the plot until the reader plots it — the
/// rule the strip draws its lanes by.
pub(super) fn plotted_event_lane(
    state: ResultsReads<'_>,
    dataset_id: crate::product::DatasetId,
    analysis: &crate::state::AnalysisResult,
    node: &str,
) -> bool {
    use crate::workbench::documents::result_document::AnalysisPresentationKey;
    use rspice_results::result_presentation::SourceWaveformPresentationKey;

    let analysis_key = AnalysisPresentationKey::new(dataset_id, analysis);
    ["D", "E"].into_iter().any(|prefix| {
        let column = format!("{prefix}({node})");
        let retained = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.name == column)
            .is_some_and(|waveform| waveform.visible);
        state.results.session.waveform_visibility(
            &SourceWaveformPresentationKey::new(analysis_key, &column),
            retained,
        )
    })
}

pub(super) fn export_raw(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
    format: RawFormat,
    signals: ExportedSignals,
) -> Option<ResultExportJob> {
    let reads = state.results_reads();
    let members = match exported_members(reads, displayed, signals) {
        Ok(members) => members,
        Err(message) => {
            state.prompt(message);
            return None;
        }
    };
    // A refused export never asks the reader for a destination it cannot fill.
    if let Some(refusal) = members.iter().find_map(|(analysis, waveforms)| {
        check_result_raw(*analysis, waveforms, exported_waveform_limits()).err()
    }) {
        state.prompt(refusal.to_string());
        return None;
    }
    // What the job writes is fixed now: each analysis, the signals by their place in it, and
    // the event lanes that ride along.
    let lanes = members
        .iter()
        .map(|(analysis, _)| {
            event_nodes(analysis)
                .into_iter()
                .filter(|node| {
                    signals == ExportedSignals::All
                        || plotted_event_lane(reads, displayed.dataset_id, analysis, node)
                })
                .map(str::to_owned)
                .collect::<std::collections::BTreeSet<_>>()
        })
        .collect::<Vec<_>>();
    let samples = read_samples(&members);
    let owned = OwnedMembers::of(&members);
    publish_export(
        state,
        io,
        ExportFile {
            label: label(format),
            title: "Export SPICE Rawfile",
            default_name: format!("waveforms.{EXTENSION}"),
            filter_name: "SPICE Rawfile",
            extension: EXTENSION,
            mime_type: mime_type(format),
            text: false,
        },
        samples,
        move |out| {
            let mut plots = Vec::new();
            for (member, lanes) in owned.members().iter().zip(&lanes) {
                plots.push(
                    write_result_raw(
                        member.analysis,
                        &member.waveforms,
                        &|node| lanes.contains(node),
                        format,
                        exported_waveform_limits(),
                        out,
                    )
                    .map_err(|error| error.to_string())?,
                );
            }
            Ok(match plots.as_slice() {
                [plot] => completion_detail(plot),
                plots => format!(
                    "{} plots, one per member, each titled by the member it is; the first {}",
                    plots.len(),
                    completion_detail(&plots[0])
                ),
            })
        },
    )
}

/// What the file holds, in the terms a rawfile reader lists it by.
fn completion_detail(export: &RawPlotFacts) -> String {
    let mut detail = format!(
        "plot '{}': {} points x {} variables on '{}'{}",
        export.plot_name,
        export.points,
        export.variables,
        export.scale_name,
        if export.complex { ", complex" } else { "" }
    );
    if export.event_plots > 0 {
        detail.push_str(&format!(
            "; {} event node plot{} after it",
            export.event_plots,
            if export.event_plots == 1 { "" } else { "s" }
        ));
        if export.bus_plots > 0 {
            detail.push_str(&format!(
                " and {} bus plot{}",
                export.bus_plots,
                if export.bus_plots == 1 { "" } else { "s" }
            ));
        }
    }
    detail
}
