//! The generated netlist leaving the project: its export dialog.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::MessageId;
use crate::workbench::code::SourceFeature;
use crate::workbench::framework::feature::Cx;

pub fn export_generated_dialog_window(ctx: &egui::Context, cx: &mut Cx<'_, SourceFeature>) {
    if !cx.state.netlist.export_dialog.open {
        return;
    }
    let mut dialog = cx.state.netlist.export_dialog.clone();
    let current = cx.model.netlist_artifacts.generation_error.is_none()
        && cx.model.netlist_artifacts.generated_document.is_some()
        && cx.model.netlist_artifacts.generated_input_digest
            == cx.model.netlist_artifacts.current_generation_input_digest;
    let (dependency_count, dependencies_sealed) = cx
        .model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .map(|document| {
            (
                document.dependencies().len(),
                document.dependency_graph_is_sealed(),
            )
        })
        .unwrap_or_default();
    // A deck that includes other files exports them with it, as a .zip.
    let requires_bundle = dependency_count > 0;
    if requires_bundle {
        dialog.bundle_dependencies = true;
    }
    let messages = cx.wb.messages();
    let netlist = &cx.model.netlist_artifacts;
    // Each refusal is one line beside Cancel and Export… in the footer.
    let refusal = if netlist.generation_error.is_some() {
        Some(MessageId::NetlistExportHasErrors)
    } else if netlist.generated_document.is_none() {
        Some(MessageId::NetlistExportNotGenerated)
    } else if !current {
        Some(MessageId::NetlistExportOutOfDate)
    } else if dialog.bundle_dependencies && !dependencies_sealed {
        Some(MessageId::NetlistExportIncludeMissing)
    } else {
        None
    }
    .map(|refusal| messages.text(refusal));
    let error = dialog.error.clone();
    let status = match (error.as_deref(), refusal.as_deref()) {
        (Some(error), _) => Err(error),
        (None, Some(refusal)) => Err(refusal),
        (None, None) => Ok(None),
    };
    let bundle_label = messages.text(MessageId::NetlistExportDependencyBundle);
    let source_map_label = messages.text(MessageId::NetlistExportSourceMap);
    let bundled_includes = requires_bundle.then(|| {
        messages.counted(
            dependency_count,
            MessageId::NetlistExportIncludesSingular,
            MessageId::NetlistExportIncludes,
        )
    });
    let include_column = messages.text(MessageId::NetlistExportIncludeColumn);
    let before = (dialog.bundle_dependencies, dialog.include_source_map);
    let choice = CommandForm::new(
        &messages.text(MessageId::NetlistExportTitle),
        &messages.text(MessageId::NetlistExportDeck),
    )
    .option_columns()
    .describe(&messages.text(MessageId::NetlistExportDescription))
    .primary_enabled(refusal.is_none())
    .status(status)
    .show(ctx, |rows| {
        let mut items: Vec<(&str, &mut bool)> = Vec::new();
        if !requires_bundle {
            items.push((bundle_label.as_str(), &mut dialog.bundle_dependencies));
        }
        items.push((source_map_label.as_str(), &mut dialog.include_source_map));
        rows.check_columns(&include_column, &mut items);
        if let Some(bundled) = &bundled_includes {
            rows.derived(bundled);
        }
        None
    });
    // A source map travels in the bundle: asking for one asks for the
    // bundle, and leaving the bundle leaves the map.
    if dialog.include_source_map && !before.1 {
        dialog.bundle_dependencies = true;
    } else if !dialog.bundle_dependencies && before.0 {
        dialog.include_source_map = false;
    }
    if (dialog.bundle_dependencies, dialog.include_source_map) != before {
        dialog.error = None;
    }
    match choice {
        DialogChoice::Primary => {
            if crate::workbench::menu_bar::action_export_generated_netlist_with_options(
                cx.model,
                cx.wb,
                dialog.bundle_dependencies,
                dialog.include_source_map,
                cx.handles.export_io,
            ) {
                dialog.open = false;
                dialog.error = None;
            } else {
                dialog.error = Some(messages.text(MessageId::NetlistExportFailed));
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => dialog.open = false,
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    cx.state.netlist.export_dialog = dialog;
}
