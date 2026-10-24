//! The code toolbars: the Verilog-A cell view's build and source tools, and
//! the Code & Automation workspace's netlist and automation pages.

use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::{WorkbenchIcon, icon_button, labeled_icon_button_sized};
use crate::workbench::layout::LayoutSpec;

use super::{
    context_separator, explicit_label_width, overflow, take_projected_tool_slot,
    toolbar_icon_button_size, toolbar_icon_command, toolbar_text_command,
};

/// Toolbar of an open Verilog-A cell view.
///
/// Compile and the cross-link to the generated deck, then the build settings
/// Compile reads, then the source tools, Format first. The toolbar is the only
/// place the first two actions live, so they come first: on a phone, Compile
/// is the one tool the row keeps, and the deck heads More tools. Build settings, Format
/// and every source tool are in the command palette as well, and Format in the
/// editor's own menu. A schematic's placement and transform tools have nothing
/// to act on here.
pub(in crate::workbench) fn veriloga_tools(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    layout: LayoutSpec,
) {
    toolbar_text_command(
        ui,
        app,
        Command::CompileVerilogA,
        WorkbenchIcon::Run,
        "Compile Verilog-A module",
        layout,
    );
    generated_netlist_tool(ui, app, layout);
    toolbar_icon_command(
        ui,
        app,
        Command::VerilogABuildSettings,
        WorkbenchIcon::Sliders,
        layout,
    );
    context_separator(ui, layout);
    veriloga_format_tool(ui, app, layout);
    toolbar_icon_command(
        ui,
        app,
        Command::FindCodeDocument,
        WorkbenchIcon::Search,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::SourceLanguageTools,
        WorkbenchIcon::Target,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::ManageSourceDocument,
        WorkbenchIcon::Folder,
        layout,
    );
}

/// The generated deck, in the Netlist workspace: what the design this module
/// is placed in netlists to.
fn generated_netlist_tool(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let label = app
        .state
        .workbench
        .messages()
        .text(crate::workbench::MessageId::CodeGeneratedNetlist);
    if !take_projected_tool_slot(ui, layout) {
        let action = overflow::OverflowAction::Run(
            crate::workbench::documents::netlist_document::show_generated_deck,
        );
        let (icon, selected) = (WorkbenchIcon::Netlist, false);
        overflow::set_aside(
            ui,
            overflow::OverflowTool {
                action,
                icon,
                label,
                selected,
            },
        );
        return;
    }
    let response = if layout.toolbar_labels {
        labeled_icon_button_sized(
            ui,
            WorkbenchIcon::Netlist,
            &label,
            false,
            explicit_label_width(&label),
            layout.toolbar_control_height,
        )
    } else {
        icon_button(
            ui,
            WorkbenchIcon::Netlist,
            &label,
            false,
            toolbar_icon_button_size(layout),
        )
    };
    if response.clicked() {
        crate::workbench::documents::netlist_document::show_generated_deck(&mut app.state);
    }
}

/// Format the source the editor is showing.
///
/// Formatting is the editor's own transaction — it is parse-gated and lands
/// as a reviewable document action — so the tool hands the request to the
/// visible editor rather than rewriting text itself. Offered only when that
/// file may be edited.
fn veriloga_format_tool(ui: &mut egui::Ui, app: &RSpiceApp, layout: LayoutSpec) {
    use crate::workbench::documents::code_workspace;

    if !take_projected_tool_slot(ui, layout) {
        return;
    }
    let language = crate::state::ProjectSourceLanguage::VerilogA;
    let editor = code_workspace::selected_veriloga_editor_snapshot(app)
        .ok()
        .filter(|snapshot| {
            code_workspace::source_bundle_document_is_editable(
                app,
                language,
                snapshot.bundle_id(),
                snapshot.active_path(),
            )
        })
        .map(|snapshot| {
            code_workspace::source_editor_id(language, snapshot.bundle_id(), snapshot.active_path())
        });
    let response = ui
        .add_enabled_ui(editor.is_some(), |ui| {
            icon_button(
                ui,
                WorkbenchIcon::Format,
                "Format document",
                false,
                toolbar_icon_button_size(layout),
            )
        })
        .inner;
    if response.clicked()
        && let Some(editor) = editor
    {
        crate::workbench::documents::text_editor_commands::queue_format_document(ui.ctx(), editor);
    }
}

/// The Code & Automation workspace hosts two different documents, so its
/// toolbar is chosen by the visible page.
///
/// Find is deliberately absent: the editor's own toolbar sits directly under
/// this one and owns it. Offering it twice was the mockup's own earlier
/// mistake, and here it was worse than redundant — the find window is drawn
/// only by the netlist page, so the button silently did nothing on the other
/// two and then ambushed the user on return.
pub(in crate::workbench) fn netlist_tools(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    layout: LayoutSpec,
) {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    match app.state.code.code_workspace.page {
        CodeWorkspacePage::Netlist => netlist_page_tools(ui, app, layout),
        CodeWorkspacePage::Automation => {
            toolbar_text_command(
                ui,
                app,
                Command::AutomationConsole,
                WorkbenchIcon::Terminal,
                "Automation console",
                layout,
            );
            source_page_tools(ui, app, layout);
        }
    }
    toolbar_icon_command(ui, app, Command::RunSimulation, WorkbenchIcon::Run, layout);
}

/// The Automation page's source tools: navigate the index, manage the
/// document, search the closure.
fn source_page_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    context_separator(ui, layout);
    toolbar_icon_command(
        ui,
        app,
        Command::SourceLanguageTools,
        WorkbenchIcon::Target,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::ManageSourceDocument,
        WorkbenchIcon::Folder,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::FindCodeDocument,
        WorkbenchIcon::Search,
        layout,
    );
}

/// The netlist page's set follows the document it is showing: a generated
/// primary is exported and compared, an owned deck is saved and run.
fn netlist_page_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let generated = app.state.code.netlist.active_document
        == crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated;
    if generated {
        toolbar_text_command(
            ui,
            app,
            Command::ExportNetlist(crate::io::NetlistFormat::Spice),
            WorkbenchIcon::Export,
            "Export generated deck",
            layout,
        );
        toolbar_text_command(
            ui,
            app,
            Command::ValidateCodeDocument,
            WorkbenchIcon::Check,
            "Validate generated netlist",
            layout,
        );
        context_separator(ui, layout);
        toolbar_text_command(
            ui,
            app,
            Command::CompareGeneratedRevisions,
            WorkbenchIcon::Compare,
            "Compare generated revisions",
            layout,
        );
        return;
    }
    toolbar_text_command(
        ui,
        app,
        Command::Save,
        WorkbenchIcon::Save,
        "Save deck",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::ValidateCodeDocument,
        WorkbenchIcon::Check,
        "Validate source",
        layout,
    );
    context_separator(ui, layout);
    toolbar_text_command(
        ui,
        app,
        Command::ExportNetlist(crate::io::NetlistFormat::Spice),
        WorkbenchIcon::Export,
        "Export deck copy",
        layout,
    );
}
