//! The Verilog-A source editor.
//!
//! A Verilog-A module is a view of a cell, so its editor is a Design-workspace
//! document: [`show_in_design`] draws it, editor only, while the inspector
//! dock states the module's facts. It edits the bundle
//! `code_workspace::active_source_owner` names, which is the bundle every
//! command, dock and dialog resolves too.

use egui::{Align, Layout, Sense, Stroke, Ui, Vec2};

use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::{MessageId, RSpiceApp};

use crate::workbench::documents::code_workspace::{
    CodeDiagnosticCollection, CodeEditorLanguage, show_code_document_interaction_versioned,
};

const EDITOR_TOOLBAR_HEIGHT: f32 = 33.0;
const EDITOR_SCROLLBAR_INSET: f32 = 14.0;

/// A Verilog-A cell view, as the Design workspace's document.
pub fn show_in_design(ui: &mut Ui, app: &mut RSpiceApp) {
    // Import and compile results are consumed by the frame loop, so a result
    // that lands while the editor is off screen is still published. Only what
    // needs this surface's pointer and context stays here.
    handle_veriloga_file_drop(ui.ctx(), app);
    let t = Tokens::get(ui.ctx());
    let selected = crate::workbench::documents::code_workspace::selected_veriloga_source(app);
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        // No heading above the editor: the tab names the module, the file row
        // names its source, and the toolbar holds Compile at every width.
        let selected = match selected {
            Ok(selected) => selected,
            Err(error) => {
                missing_source(ui, app, &error);
                return;
            }
        };
        let active_path =
            crate::workbench::documents::code_workspace::active_veriloga_file_path(app, &selected);
        // A built-in header the navigator opened is read in this same well,
        // in place of the file, and never edited.
        let header = crate::workbench::documents::code_workspace::viewed_veriloga_header(
            &app.state.ui.code_workspace.veriloga,
            selected.bundle().id(),
            &active_path,
        );
        let editor_language = if header.is_none()
            && selected.bundle().role_for_path(&active_path)
                == Some(crate::state::ProjectSourceRole::VerilogABuildProfile)
        {
            CodeEditorLanguage::Toml
        } else {
            CodeEditorLanguage::VerilogA
        };
        let (shown_path, mut source) = match header {
            Some((identity, text)) => (identity.to_owned(), text.to_owned()),
            None => (
                active_path.clone(),
                selected
                    .bundle()
                    .file_content(&active_path)
                    .unwrap_or_else(|| selected.document().content())
                    .to_owned(),
            ),
        };
        let diagnostics = current_diagnostics(app, &selected);
        // The inspector dock states the module's facts, so the editor takes
        // the whole document well at every width. A second copy of them beside
        // or below it would only push the source off a phone.
        let available = ui.available_rect_before_wrap();
        let mut editor_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(available)
                .layout(Layout::top_down(Align::Min)),
        );
        source_editor(
            &mut editor_ui,
            app,
            &selected,
            &shown_path,
            &mut source,
            &diagnostics,
            editor_language,
            header.is_none(),
        );
        ui.allocate_rect(available, Sense::hover());
    });
}

fn handle_veriloga_file_drop(ctx: &egui::Context, app: &mut RSpiceApp) {
    if app.state.application_modal_open() {
        return;
    }
    let dropped = ctx.input(|input| input.raw.dropped_files.clone());
    if dropped.is_empty() {
        return;
    }
    if dropped.len() != 1 {
        let message = app
            .state
            .ui
            .messages()
            .text(MessageId::VerilogADropOneSource);
        app.state
            .push_user_message(ConsoleMessage::warning(message));
        return;
    }
    let file = &dropped[0];
    let name = (!file.name.trim().is_empty())
        .then(|| file.name.clone())
        .or_else(|| {
            file.path
                .as_deref()
                .and_then(std::path::Path::file_name)
                .and_then(std::ffi::OsStr::to_str)
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "dropped.va".to_owned());
    let bytes = if let Some(bytes) = file.bytes.as_ref() {
        Ok(bytes.to_vec())
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            file.path
                .as_deref()
                .ok_or_else(|| "Dropped file has neither bytes nor a native path.".to_owned())
                .and_then(|path| std::fs::read(path).map_err(|error| error.to_string()))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Err("Browser drop did not provide immutable file bytes.".to_owned())
        }
    };
    let result = bytes.and_then(|bytes| {
        if bytes.len() > crate::state::MAX_PROJECT_CODE_SOURCE_BYTES {
            return Err(format!(
                "Dropped source exceeds the supported {}-byte limit.",
                crate::state::MAX_PROJECT_CODE_SOURCE_BYTES
            ));
        }
        let contents = String::from_utf8(bytes)
            .map_err(|error| format!("Dropped source is not valid UTF-8: {error}"))?;
        crate::workbench::documents::code_workspace::import_dropped_veriloga_source(
            app, name, contents,
        )
    });
    if let Err(error) = result {
        app.state.push_user_message(ConsoleMessage::error(format!(
            "Verilog-A drop import failed: {error}"
        )));
    }
}

fn source_editor(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
    active_path: &str,
    source: &mut String,
    diagnostics: &CodeDiagnosticCollection,
    editor_language: CodeEditorLanguage,
    // False for a built-in header: it is read here, and nothing that writes
    // a file, searches the bundle, or records a place in it applies.
    editable: bool,
) {
    let t = Tokens::get(ui.ctx());
    let editor_id = egui::Id::new((
        "workbench.veriloga.source",
        selected.bundle().id(),
        active_path,
    ));
    let (toolbar, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), EDITOR_TOOLBAR_HEIGHT),
        Sense::hover(),
    );
    // The page's tone, which the active document tab above wears too: the
    // tab opens onto this row.
    ui.painter().rect_filled(toolbar, 0.0, t.color.bg_app);
    ui.painter().hline(
        toolbar.x_range(),
        toolbar.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let path_clip = egui::Rect::from_min_max(
        egui::pos2(toolbar.left() + 8.0, toolbar.top()),
        egui::pos2(toolbar.right() - 8.0, toolbar.bottom()),
    );
    let messages = app.state.ui.messages();
    let (status, color) = crate::workbench::documents::code_workspace::veriloga_compile_status(
        &app.state.ui.code_workspace.veriloga,
        app.state.workspace.content.project.id(),
        selected,
        messages,
        &t,
    );
    let status_galley =
        ui.painter()
            .layout_no_wrap(status, theme::mono(tokens::FS_0, FontWeight::Medium), color);
    let toolbar_content_right = toolbar.right() - EDITOR_SCROLLBAR_INSET;
    let status_dot_x = toolbar_content_right - status_galley.size().x - 17.0;
    ui.painter()
        .circle_filled(egui::pos2(status_dot_x, toolbar.center().y), 2.5, color);
    ui.painter().galley(
        egui::pos2(
            toolbar_content_right - status_galley.size().x - 8.0,
            toolbar.center().y - status_galley.size().y * 0.5,
        ),
        status_galley.clone(),
        color,
    );
    // Keep even unusually long logical paths out of the status. The full path
    // remains available from the file tree.
    let path_right = (status_dot_x - 13.0).max(path_clip.left());
    let path_clip =
        egui::Rect::from_min_max(path_clip.min, egui::pos2(path_right, path_clip.bottom()));
    ui.painter().with_clip_rect(path_clip).text(
        egui::pos2(toolbar.left() + 8.0, toolbar.center().y),
        egui::Align2::LEFT_CENTER,
        active_path,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    interface_strip(ui, &mut app.state, selected);
    let search_input = editable
        .then(|| {
            crate::workbench::documents::text_editor_commands::take_find_in_source_bundle_request(
                ui, editor_id,
            )
        })
        .flatten();
    let preceding_input = search_input
        .as_ref()
        .map(|input| input.preceding_scope(ui.ctx()));
    // A built-in header is read here whatever the bundle allows.
    let writable = editable
        && crate::workbench::documents::code_workspace::source_bundle_document_is_editable(
            app,
            crate::state::ProjectSourceLanguage::VerilogA,
            selected.bundle().id(),
            active_path,
        );
    super::publish_source_editor(
        ui.ctx(),
        app,
        crate::workbench::documents::text_editor_commands::ActiveSourceEditor {
            id: editor_id,
            editable: writable,
            // The build profile beside the module is TOML, which the
            // language service does not index, and a built-in header is
            // not part of the bundle it indexes.
            navigable: editable && editor_language == CodeEditorLanguage::VerilogA,
        },
    );
    let interaction = egui::Frame::new()
        .fill(t.color.bg_inset)
        .show(ui, |ui| {
            let interaction = show_code_document_interaction_versioned(
                ui,
                (
                    "workbench.veriloga.source",
                    selected.bundle().id(),
                    active_path,
                ),
                source,
                selected.bundle().revision().get(),
                editor_language,
                diagnostics,
                Some(active_path),
                editable,
                messages,
            );
            if editable && interaction.changed {
                match crate::workbench::documents::code_workspace::replace_selected_veriloga_file(
                    app,
                    selected,
                    active_path,
                    source.clone(),
                ) {
                    Ok(true) => {}
                    Ok(false) => {}
                    Err(error) => {
                        app.state
                            .push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                                "Could not update {}: {error}",
                                active_path
                            )))
                    }
                }
            }
            interaction
        })
        .inner;
    // A built-in header keeps the menu for reading: its writes are disabled
    // and its language group hidden through the editor published above.
    if let Some(response) = interaction.response.as_ref() {
        crate::workbench::chrome::title_bar::source_editor_context_menu(response, app);
    }
    if !editable {
        return;
    }
    // Every request below names the bundle this editor is showing, never a
    // language-wide default: Find, Format and the language tools must act on
    // the cell view in front of the reader.
    let bundle_id = selected.bundle().id();
    app.state.ui.code_workspace.source_carets.insert(
        (bundle_id, active_path.to_owned()),
        interaction.cursor_char_index,
    );
    let selection_key = (bundle_id, active_path.to_owned());
    if let Some(range) = interaction.selected_char_range {
        app.state
            .ui
            .code_workspace
            .source_selections
            .insert(selection_key, range);
    } else {
        app.state
            .ui
            .code_workspace
            .source_selections
            .remove(&selection_key);
    }
    if crate::workbench::documents::text_editor_commands::take_format_document_request(
        ui, editor_id,
    ) {
        match crate::workbench::documents::code_workspace::open_language_tools_in_bundle(
            app,
            crate::state::ProjectSourceLanguage::VerilogA,
            bundle_id,
            active_path,
            interaction.cursor_char_index,
        ) {
            Ok(()) => {
                if let Some(tools) = app.state.ui.code_workspace.language_tools.as_mut() {
                    tools.view =
                        crate::workbench::documents::code_workspace::LanguageToolView::CodeActions;
                }
            }
            Err(error) => app
                .state
                .push_user_message(crate::diagnostics::ConsoleMessage::error(error)),
        }
    }
    drop(preceding_input);
    if let Some(input) = search_input {
        // Bind the search to source revisions and selection after preceding edits.
        if let Err(error) =
            crate::workbench::documents::code_workspace::open_source_search_in_bundle(
                app,
                crate::state::ProjectSourceLanguage::VerilogA,
                bundle_id,
                active_path,
            )
        {
            app.state
                .push_user_message(crate::diagnostics::ConsoleMessage::error(error));
        }
        input.route(
            ui.ctx(),
            crate::workbench::documents::code_workspace::source_search_query_id(),
        );
    }
}

/// The line under the toolbar that states a finding about the module's
/// interface: its ports differ from the cell's schematic, they could not be
/// written, or a hand-edited symbol has drifted from them, with Update symbol
/// beside the drift. With nothing to say it takes no room at all.
fn interface_strip(
    ui: &mut Ui,
    state: &mut crate::workbench::app_state::AppState,
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
) {
    use crate::workbench::documents::code_workspace::{
        VerilogAInterfaceFinding, interface_finding, interface_mismatch_detail, symbol_drift_text,
        veriloga_receipt,
    };

    let crate::state::ProjectSourceOwner::CellView { reference } = selected.bundle().owner() else {
        return;
    };
    let t = Tokens::get(ui.ctx());
    let messages = state.ui.messages();
    let veriloga = &state.ui.code_workspace.veriloga;
    let receipt = veriloga_receipt(veriloga, state.workspace.content.project.id(), selected);
    let (text, tone, detail, action) = match interface_finding(veriloga, receipt, reference) {
        Some(VerilogAInterfaceFinding::SchematicMismatch { schematic, module }) => (
            messages.text(MessageId::VerilogAInterfaceMismatchStrip),
            t.color.err,
            Some(interface_mismatch_detail(reference, schematic, module)),
            StripAction::None,
        ),
        Some(VerilogAInterfaceFinding::Refused(reason)) => (
            messages.format(
                MessageId::VerilogAInterfaceRefusedStrip,
                &[("reason", reason)],
            ),
            t.color.warn,
            None,
            StripAction::None,
        ),
        Some(VerilogAInterfaceFinding::SymbolDrift(drift)) => (
            messages.format(
                MessageId::VerilogASymbolStrip,
                &[("drift", &symbol_drift_text(drift, messages))],
            ),
            t.color.warn,
            None,
            StripAction::UpdateSymbol,
        ),
        // With nothing to fix, the compile result of a device New Verilog-A
        // device… created hands it back to the schematic it came from.
        None => {
            let Some((receipt, target)) = receipt.and_then(|receipt| {
                let handoff = receipt.placement.as_ref()?;
                (&handoff.view == reference)
                    .then(|| state.veriloga_placement_target(handoff))
                    .flatten()
                    .map(|target| (receipt, target))
            }) else {
                return;
            };
            let ports = receipt.compilation.report().abi.formal_ports.len();
            let text = if ports == 1 {
                messages.format(
                    MessageId::VerilogAPlaceReadyOne,
                    &[("cell", &reference.cell)],
                )
            } else {
                messages.format(
                    MessageId::VerilogAPlaceReady,
                    &[("cell", &reference.cell), ("count", &ports.to_string())],
                )
            };
            let place = messages.format(MessageId::VerilogAPlaceIn, &[("schematic", &target.cell)]);
            (text, t.color.text_dim, None, StripAction::Place(place))
        }
    };
    let (strip, response) = ui.allocate_exact_size(
        Vec2::new(
            ui.available_width(),
            EDITOR_TOOLBAR_HEIGHT.max(t.metrics.ctl_h),
        ),
        Sense::hover(),
    );
    // The strip shares the toolbar's fill, so the rule above it is what keeps
    // a finding from reading as a second row of tools.
    ui.painter().rect_filled(strip, 0.0, t.color.bg_app);
    for y in [strip.top() + 0.5, strip.bottom()] {
        ui.painter()
            .hline(strip.x_range(), y, Stroke::new(1.0, t.color.border));
    }
    // The toolbar's left inset, and its status text's right edge.
    let content = strip.shrink2(Vec2::new(8.0, 0.0));
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(egui::Rect::from_min_max(
                content.min,
                egui::pos2(content.right() - EDITOR_SCROLLBAR_INSET, content.bottom()),
            ))
            .layout(Layout::right_to_left(Align::Center)),
    );
    let clicked = match &action {
        StripAction::None => false,
        StripAction::UpdateSymbol => Button::new(&messages.text(MessageId::VerilogAUpdateSymbol))
            .show(&mut actions)
            .clicked(),
        StripAction::Place(label) => Button::new(label).show(&mut actions).clicked(),
    };
    let text_right = actions.min_rect().left().min(content.right()) - 8.0;
    let icon = egui::Rect::from_center_size(
        egui::pos2(content.left() + 7.0, content.center().y),
        Vec2::splat(14.0),
    );
    let glyph = match action {
        StripAction::Place(_) => crate::workbench::design_system::WorkbenchIcon::Instance,
        StripAction::None | StripAction::UpdateSymbol => {
            crate::workbench::design_system::WorkbenchIcon::Warning
        }
    };
    glyph.paint(ui.painter(), icon, tone);
    let text_clip = egui::Rect::from_min_max(
        egui::pos2(icon.right() + 7.0, strip.top()),
        egui::pos2(text_right.max(icon.right() + 7.0), strip.bottom()),
    );
    ui.painter().with_clip_rect(text_clip).text(
        egui::pos2(text_clip.left(), strip.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        tone,
    );
    if let Some(detail) = detail {
        response.on_hover_text(detail);
    }
    if !clicked {
        return;
    }
    match action {
        StripAction::None => {}
        StripAction::UpdateSymbol => state.update_active_veriloga_symbol(),
        StripAction::Place(_) => match state.place_offered_veriloga_device() {
            Ok(armed) => {
                state.push_user_message(ConsoleMessage::info(armed));
                // The canvas takes the keyboard, as the shelf leaves it, so
                // R, M and Esc work before the pointer reaches the sheet.
                rspice_schematic_editor::view::canvas::request_focus(ui.ctx());
            }
            Err(error) => state.push_user_message(ConsoleMessage::warning(error)),
        },
    }
}

/// What the interface strip's one button does.
enum StripAction {
    /// The strip states a finding with nothing to press.
    None,
    /// Reconcile a hand-edited symbol with the module's ports.
    UpdateSymbol,
    /// Place the new device in the schematic named by the label.
    Place(String),
}

fn current_diagnostics(
    app: &RSpiceApp,
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
) -> std::sync::Arc<CodeDiagnosticCollection> {
    crate::workbench::documents::code_workspace::veriloga_diagnostics(
        &app.state.ui.code_workspace.veriloga,
        app.state.workspace.content.project.id(),
        selected,
    )
}

/// The view has no bundle to show.
///
/// A Verilog-A cell view without its bundle is a defect of that view, and the
/// resolver's error names it exactly. Nothing here offers a remedy that is not
/// one: there is no project-wide source to fall back to.
fn missing_source(ui: &mut Ui, app: &RSpiceApp, error: &str) {
    super::super::design_system::empty_state(
        ui,
        super::super::design_system::WorkbenchIcon::VerilogA,
        &app.state.ui.messages().text(MessageId::VerilogANoSource),
        error,
    );
}
