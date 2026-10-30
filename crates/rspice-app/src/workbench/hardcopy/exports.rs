//! The hardcopy feature's file exports: the open schematic as SVG, and the
//! sealed publication snapshot.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::hardcopy_adapters::publication_snapshot::{
    PublicationReads, publication_timestamp_utc,
};
use crate::workbench::hardcopy_adapters::sources::SourceReads;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

use super::HardcopyFeature;

/// Export the open schematic as SVG, through the export IO.
pub fn action_export_svg_with_io(model: &mut Model, io: &(impl ExportWorkflowIo + ?Sized)) {
    use rspice_schematic_editor::export::{
        SvgDesignContext, SvgExportConfig, export_to_svg_with_symbol_resolver_and_context,
    };

    let config = SvgExportConfig::default();
    let resolver = rspice_design::symbol_resolver::SymbolResolver::new(
        model.library_manager.catalog(),
        &model.workspace.content.schematic_buffers,
    );
    let view_path = model.workspace.content.active_view.display_path();
    let sheet_key = model.workspace.content.active_schematic_reference().key();
    let svg_content = export_to_svg_with_symbol_resolver_and_context(
        model.schematic.document(),
        &config,
        &resolver,
        SvgDesignContext {
            view_path: &view_path,
            // What the canvas shows: the active sheet.
            sheet_catalog: model
                .workspace
                .content
                .design_management
                .sheet_catalog(&sheet_key),
        },
    );

    let default_name = format!("{}.svg", model.export_file_stem("schematic"));

    match io.show_save_dialog(SaveDialogConfig {
        title: "Export SVG",
        default_name: &default_name,
        filter_name: "SVG Image",
        filter_extensions: &["svg"],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, "svg");

            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &svg_content));
            match export {
                Ok(()) => {
                    model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            "SVG", &path, None, io,
                        ),
                    );
                }
                Err(e) => model.report_failure("SVG export failed", e.to_string()),
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(e) => model.report_failure("SVG export failed", e.to_string()),
    }
}

/// Export the sealed publication snapshot: the exact canonical bytes the
/// publish pipeline consumes, built from the current sheets, plot panes,
/// active-run results, and effective deck.
pub(super) fn action_export_publication_snapshot_with_io(cx: &mut Cx<'_, HardcopyFeature>) {
    let io = cx.handles.export_io;
    let stem = cx
        .model
        .schematic
        .session
        .current_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|s| s.to_string_lossy().into_owned());
    let draft = crate::workbench::framework::feature::PublicationDraft {
        title: stem
            .clone()
            .unwrap_or_else(|| "Untitled project".to_string()),
        description: String::new(),
        author_display: "Local export".to_string(),
        created_utc: publication_timestamp_utc(),
        license: rspice_publication_contract::ContentLicense::AllRightsReserved,
        ..Default::default()
    };
    let built =
        crate::workbench::hardcopy_adapters::publication_snapshot::build_publication_snapshot(
            publication_reads(cx),
            &draft,
        )
        .map_err(|error| error.to_string())
        .and_then(|snapshot| {
            snapshot
                .canonical_bytes()
                .map_err(|error| error.to_string())
        })
        .and_then(|bytes| String::from_utf8(bytes).map_err(|error| error.to_string()));
    let content = match built {
        Ok(content) => content,
        Err(error) => {
            cx.model
                .report_failure("Publication snapshot export failed", error);
            return;
        }
    };
    let default_name = stem
        .map(|s| format!("{s}.rspicepub"))
        .unwrap_or_else(|| "publication.rspicepub".to_string());

    match io.show_save_dialog(SaveDialogConfig {
        title: "Export publication snapshot",
        default_name: &default_name,
        filter_name: "RSpice publication snapshot",
        filter_extensions: &["rspicepub"],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, "rspicepub");
            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &content));
            match export {
                Ok(()) => {
                    cx.model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            "publication snapshot",
                            &path,
                            None,
                            io,
                        ),
                    );
                }
                Err(error) => {
                    cx.model
                        .report_failure("Publication snapshot export failed", error.to_string());
                }
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(error) => {
            cx.model
                .report_failure("Publication snapshot export failed", error.to_string());
        }
    }
}

/// What the snapshot is built from, read from the feature's context.
fn publication_reads<'a>(cx: &'a Cx<'_, HardcopyFeature>) -> PublicationReads<'a> {
    PublicationReads {
        sources: SourceReads {
            model: cx.model,
            workbench: cx.wb,
            report_authoring: &cx.state.report_authoring,
            results: cx.peers.results(),
        },
        runs: cx.runs,
    }
}
