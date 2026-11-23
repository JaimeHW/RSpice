//! Formatting the deck or include the netlist editor shows, answered for the
//! source feature's request.

use crate::workbench::code::SourceFeature;
use crate::workbench::framework::feature::Cx;
use crate::workbench::workflows::netlist_workflow::formatted_deck_text;

/// Whether the file the netlist editor shows is a Spectre file, which a `+`
/// does not continue.
fn shows_spectre(cx: &Cx<'_, SourceFeature>) -> bool {
    crate::workbench::documents::netlist_document::dialect::shown_source_in(
        cx.model,
        &cx.state.netlist,
    )
    .is_some_and(|shown| {
        matches!(
            shown.syntax(),
            crate::workbench::documents::text_editor_commands::EditorSyntax::Spectre
        )
    })
}

/// Format the deck or the include the netlist editor shows, when it is
/// editable and its check finds no error: trailing blanks come off every
/// card but the title, and the text ends with a line end.
pub fn format_owned_netlist(cx: &mut Cx<'_, SourceFeature>) {
    if !crate::workbench::documents::netlist_document::active_netlist_source_is_editable_in(
        cx.model,
        &cx.state.netlist,
    ) {
        return;
    }
    let has_errors = cx.state.netlist.diagnostics.iter().any(|diagnostic| {
        diagnostic.is_current()
            && diagnostic.severity
                == crate::workbench::documents::netlist_document::DiagnosticSeverity::Error
    });
    if has_errors {
        cx.wb.prompt(
            cx.wb
                .messages()
                .text(crate::workbench::MessageId::NetlistFormatHasErrors),
        );
        return;
    }
    let source = cx.model.simulation.source.netlist_content.clone();
    let formatted = formatted_deck_text(&source, shows_spectre(cx));
    if formatted == source {
        cx.wb.prompt(
            cx.wb
                .messages()
                .text(crate::workbench::MessageId::NetlistFormatUnchanged),
        );
    } else if crate::workbench::documents::netlist_document::edit_deck_file(
        cx.model,
        &mut cx.state.netlist,
        formatted,
    ) {
        cx.wb.prompt(
            cx.wb
                .messages()
                .text(crate::workbench::MessageId::NetlistFormatted),
        );
    }
}
