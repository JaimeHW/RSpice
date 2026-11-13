//! The Verilog-A source editor.
//!
//! A Verilog-A module is a view of a cell, so its editor is a Design-workspace
//! document: the shell's `show_in_design` draws it, editor only, while the
//! inspector dock states the module's facts. It edits the bundle
//! `ProjectWorkspace::active_source_owner` names, which is the bundle every
//! command, dock and dialog resolves too.

use egui::{Align, Layout, Sense, Stroke, Ui, Vec2};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::code::CodeFeature;
use crate::workbench::documents::text_editor_commands::ActiveSourceEditor;
use crate::workbench::framework::feature::Cx;
use crate::workbench::{MessageId, WorkbenchSession};

use super::super::design_system::{PROBLEM_MARK_GAP, PROBLEM_MARK_SIDE, paint_problem_mark};

use crate::state::code_diagnostics::CodeDiagnosticCollection;
use crate::workbench::app::actions::veriloga_placement::veriloga_placement_target;
use crate::workbench::documents::code_workspace::{
    CodeEditorLanguage, CodeWorkspaceRuntimeState, EditorFindings, show_veriloga_document_versioned,
};

const EDITOR_TOOLBAR_HEIGHT: f32 = 33.0;
const EDITOR_SCROLLBAR_INSET: f32 = 14.0;

/// Before the page draws, in the code feature's context: put the page's
/// editor in front, before the shell takes the dispatcher's answers the
/// page's context holds, so they are the answers the editor gets. The page
/// publishes it again as it draws it.
pub(crate) fn prepare(cx: &mut Cx<'_, CodeFeature>) {
    use crate::workbench::documents::code_workspace::{
        active_veriloga_file_path, selected_veriloga_source_in, viewed_veriloga_header,
    };

    let Ok(selected) = selected_veriloga_source_in(cx.model) else {
        return;
    };
    let active_path = active_veriloga_file_path(&cx.state.code_workspace, &selected);
    let header = viewed_veriloga_header(
        &cx.state.code_workspace.veriloga,
        selected.bundle().id(),
        &active_path,
    );
    let shown_path =
        header.map_or_else(|| active_path.clone(), |(identity, _)| identity.to_owned());
    cx.wb.active_source_editor = Some(page_editor(
        cx.model,
        &cx.state.code_workspace,
        cx.platform,
        &selected,
        &shown_path,
        editor_language(&selected, &active_path, header.is_some()),
        header.is_none(),
    ));
}

/// Whether the page's editor's right-click menu is open, or may open on
/// what this frame carries, read from the editor the page put in front.
/// The page's context holds the menu's answers only then, so this is read
/// before the context is lent.
pub(crate) fn source_menu_open(ctx: &egui::Context, wb: &WorkbenchSession) -> bool {
    wb.active_source_editor.is_some_and(|editor| {
        crate::workbench::documents::virtual_text_editor::context_menu_may_show(ctx, editor.id)
    })
}

/// The language the editor reads the file it shows in: the build profile
/// beside the module is TOML; its sources, and a built-in header read in
/// place of one, are Verilog-A.
fn editor_language(
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
    active_path: &str,
    header_shown: bool,
) -> CodeEditorLanguage {
    if !header_shown
        && selected.bundle().role_for_path(active_path)
            == Some(crate::state::ProjectSourceRole::VerilogABuildProfile)
    {
        CodeEditorLanguage::Toml
    } else {
        CodeEditorLanguage::VerilogA
    }
}

/// The page's editor of `shown_path`, as the Edit menu and the source menu
/// act on it. `editable` is false for a built-in header.
fn page_editor(
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    platform: &PlatformState,
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
    shown_path: &str,
    editor_language: CodeEditorLanguage,
    editable: bool,
) -> ActiveSourceEditor {
    let language = crate::state::ProjectSourceLanguage::VerilogA;
    let bundle_id = selected.bundle().id();
    ActiveSourceEditor {
        id: crate::workbench::documents::code_workspace::source_editor_id(
            language, bundle_id, shown_path,
        ),
        // A built-in header is read here whatever the bundle allows.
        editable: editable
            && crate::workbench::documents::code_workspace::source_bundle_document_is_editable_in(
                model, code, platform, language, bundle_id, shown_path,
            ),
        // The build profile beside the module is TOML, which the language
        // service does not index, and a built-in header is not part of the
        // bundle it indexes.
        navigable: editable && editor_language == CodeEditorLanguage::VerilogA,
    }
}

/// Draw a Verilog-A cell view in the code feature's context, and say what
/// the reader pressed on the interface strip.
pub(crate) fn show(ui: &mut Ui, cx: &mut Cx<'_, CodeFeature>) -> Option<InterfaceAction> {
    // Import and compile results are consumed by the frame loop, so a result
    // that lands while the editor is off screen is still published. Only what
    // needs this surface's pointer and context stays here.
    handle_veriloga_file_drop(ui.ctx(), cx);
    let t = Tokens::get(ui.ctx());
    let selected =
        crate::workbench::documents::code_workspace::selected_veriloga_source_in(cx.model);
    let mut pressed = None;
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        // No heading above the editor: the tab names the module, the file row
        // names its source, and the toolbar holds Compile at every width.
        let selected = match selected {
            Ok(selected) => selected,
            Err(error) => {
                missing_source(ui, cx, &error);
                return;
            }
        };
        let active_path = crate::workbench::documents::code_workspace::active_veriloga_file_path(
            &cx.state.code_workspace,
            &selected,
        );
        // A built-in header the navigator opened is read in this same well,
        // in place of the file, and never edited.
        let header = crate::workbench::documents::code_workspace::viewed_veriloga_header(
            &cx.state.code_workspace.veriloga,
            selected.bundle().id(),
            &active_path,
        );
        let editor_language = editor_language(&selected, &active_path, header.is_some());
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
        let diagnostics = current_diagnostics(cx, &selected);
        // The inspector dock states the module's facts, so the editor takes
        // the whole document well at every width. A second copy of them beside
        // or below it would only push the source off a phone.
        let available = ui.available_rect_before_wrap();
        let mut editor_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(available)
                .layout(Layout::top_down(Align::Min)),
        );
        pressed = source_editor(
            &mut editor_ui,
            cx,
            &selected,
            &shown_path,
            &mut source,
            diagnostics.as_deref(),
            editor_language,
            header.is_none(),
        );
        ui.allocate_rect(available, Sense::hover());
    });
    pressed
}

fn handle_veriloga_file_drop(ctx: &egui::Context, cx: &mut Cx<'_, CodeFeature>) {
    if cx.application_modal_open() {
        return;
    }
    let dropped = ctx.input(|input| input.raw.dropped_files.clone());
    if dropped.is_empty() {
        return;
    }
    // One root and its includes, or several roots: every dropped file is
    // read, and the import sorts roots from the files they include.
    let result = dropped
        .iter()
        .map(dropped_source)
        .collect::<Result<Vec<_>, _>>()
        .and_then(|sources| {
            crate::workbench::documents::code_workspace::import_dropped_veriloga_sources(
                cx.model,
                &mut cx.state.code_workspace.veriloga,
                sources,
            )
        });
    if let Err(error) = result {
        cx.model.record(
            LogSeverity::Error,
            LogSource::Studio,
            format!("Verilog-A/AMS drop import failed: {error}"),
        );
    }
}

/// One dropped file's name, text and folder.
fn dropped_source(
    file: &egui::DroppedFile,
) -> Result<crate::workbench::documents::code_workspace::PickedSource, String> {
    let name = (!file.name.trim().is_empty())
        .then(|| file.name.clone())
        .or_else(|| {
            file.path
                .as_deref()
                .and_then(std::path::Path::file_name)
                .and_then(std::ffi::OsStr::to_str)
                .map(str::to_owned)
        })
        .ok_or_else(|| "A dropped file has no name.".to_owned())?;
    let bytes = if let Some(bytes) = file.bytes.as_ref() {
        Ok(bytes.to_vec())
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            file.path
                .as_deref()
                .ok_or_else(|| format!("{name} has neither bytes nor a native path."))
                .and_then(|path| std::fs::read(path).map_err(|error| format!("{name}: {error}")))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Err(format!(
                "The browser did not provide the contents of {name}."
            ))
        }
    }?;
    if bytes.len() > crate::state::MAX_PROJECT_CODE_SOURCE_BYTES {
        return Err(format!(
            "{name} exceeds the supported {}-byte limit.",
            crate::state::MAX_PROJECT_CODE_SOURCE_BYTES
        ));
    }
    let contents =
        String::from_utf8(bytes).map_err(|error| format!("{name} is not valid UTF-8: {error}"))?;
    Ok(crate::workbench::documents::code_workspace::PickedSource {
        folder: file
            .path
            .as_deref()
            .and_then(std::path::Path::parent)
            .map(std::path::Path::to_path_buf),
        name,
        contents,
    })
}

/// The editor and the rows above it, and what the reader pressed on the
/// interface strip.
fn source_editor(
    ui: &mut Ui,
    cx: &mut Cx<'_, CodeFeature>,
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
    active_path: &str,
    source: &mut String,
    // `None` when the source changed after its last check.
    diagnostics: Option<&CodeDiagnosticCollection>,
    editor_language: CodeEditorLanguage,
    // False for a built-in header: it is read here, and nothing that writes
    // a file, searches the bundle, or records a place in it applies.
    editable: bool,
) -> Option<InterfaceAction> {
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
    let messages = cx.wb.messages();
    let toolbar_content_right = toolbar.right() - EDITOR_SCROLLBAR_INSET;
    if cx.state.code_workspace.veriloga.pending.is_some() {
        // The strip counts the seconds of a running compile.
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(250));
    }
    // Neutral text, and a mark only where the reader has to act. A clean
    // compile leaves the corner empty and gives the whole row to the path.
    let status_left = crate::workbench::documents::code_workspace::veriloga_compile_status(
        &cx.state.code_workspace.veriloga,
        cx.model.workspace.content.project.id(),
        selected,
        messages,
    )
    .map_or(toolbar_content_right, |(status, problem)| {
        let galley = ui.painter().layout_no_wrap(
            status.clone(),
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.text_dim,
        );
        let mut left = toolbar_content_right - galley.size().x - 8.0;
        let status_rect = egui::Rect::from_min_size(
            egui::pos2(left, toolbar.center().y - galley.size().y * 0.5),
            galley.size(),
        );
        ui.painter()
            .galley(status_rect.min, galley, t.color.text_dim);
        // A screen reader can read the strip as a status, but it is not a live
        // region: the status bar already announces the compile result once.
        let status_response = ui.interact(
            status_rect,
            editor_id.with("compile-status"),
            Sense::hover(),
        );
        ui.ctx().accesskit_node_builder(status_response.id, |node| {
            node.set_role(egui::accesskit::Role::Status);
            node.set_label(status);
        });
        if let Some(problem) = problem {
            left -= PROBLEM_MARK_GAP;
            paint_problem_mark(
                ui.painter(),
                egui::Rect::from_center_size(
                    egui::pos2(left - PROBLEM_MARK_SIDE / 2.0, toolbar.center().y),
                    Vec2::splat(PROBLEM_MARK_SIDE),
                ),
                problem,
            );
            left -= PROBLEM_MARK_SIDE;
        }
        left
    });
    // Keep even unusually long logical paths out of the status. The full path
    // remains available from the file tree.
    let path_right = (status_left - 9.0).max(path_clip.left());
    let path_clip =
        egui::Rect::from_min_max(path_clip.min, egui::pos2(path_right, path_clip.bottom()));
    ui.painter().with_clip_rect(path_clip).text(
        egui::pos2(toolbar.left() + 8.0, toolbar.center().y),
        egui::Align2::LEFT_CENTER,
        active_path,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    let pressed = interface_strip(ui, cx, selected);
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
    let editor = page_editor(
        cx.model,
        &cx.state.code_workspace,
        cx.platform,
        selected,
        active_path,
        editor_language,
        editable,
    );
    super::netlist::publish_source_editor(ui.ctx(), cx.wb, editor);
    // What an include in this file may name: the bundle's other sources.
    let include_files = selected
        .bundle()
        .files()
        .iter()
        .map(crate::state::ProjectSourceFile::logical_path)
        .chain([selected.bundle().root().logical_path()])
        .filter(|path| {
            !crate::state::project_source_paths_equal(path, active_path)
                && selected.bundle().role_for_path(path).is_none()
        })
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let interaction = egui::Frame::new()
        .fill(t.color.bg_inset)
        .show(ui, |ui| {
            let interaction = show_veriloga_document_versioned(
                ui,
                (
                    "workbench.veriloga.source",
                    selected.bundle().id(),
                    active_path,
                ),
                source,
                selected.bundle().revision().get(),
                editor_language,
                diagnostics.map_or(EditorFindings::Outdated, EditorFindings::Current),
                Some(active_path),
                editable,
                messages,
                &include_files,
            );
            if editable && interaction.changed {
                match crate::workbench::documents::code_workspace::replace_selected_veriloga_file(
                    &mut CodeFeature::workspace_parts(cx),
                    selected,
                    active_path,
                    source.clone(),
                ) {
                    Ok(true) => {}
                    Ok(false) => {}
                    Err(error) => {
                        cx.wb
                            .prompt(format!("Could not update {active_path}: {error}"));
                        // The editor goes back to the source as it is stored.
                        crate::workbench::documents::virtual_text_editor::resynchronize(
                            ui.ctx(),
                            crate::workbench::documents::code_workspace::source_editor_id(
                                crate::state::ProjectSourceLanguage::VerilogA,
                                selected.bundle().id(),
                                active_path,
                            ),
                        );
                    }
                }
            }
            interaction
        })
        .inner;
    // A built-in header keeps the menu for reading: its writes are disabled
    // and its language group hidden through the editor published above. The
    // rows read the page's context, which holds the menu's answers while the
    // menu is open or opening; the row chosen runs once the page has drawn.
    let choice = crate::workbench::chrome::title_bar::source_editor_context_menu_over(
        &interaction.response,
        |ctx| {
            super::netlist::source_menu::rows(crate::workbench::code::SOURCE_EDITOR_MENU, cx, ctx)
        },
    );
    if let Some(choice) = choice {
        cx.execute(choice.command());
    }
    if let Some(announcement) = interaction.announcement.clone() {
        cx.wb.prompt(announcement);
    }
    if !editable {
        return pressed;
    }
    // Every request below names the bundle this editor is showing, never a
    // language-wide default: Find, Format and the language tools must act on
    // the cell view in front of the reader.
    let bundle_id = selected.bundle().id();
    if let Some(step) = interaction.unanswered_history {
        match cx.state.code_workspace.source_edit_history.step(
            &mut cx.model.workspace.content,
            bundle_id,
            step,
        ) {
            Ok(true) => crate::workbench::documents::code_workspace::invalidate_veriloga_evidence(
                &mut CodeFeature::workspace_parts(cx),
                bundle_id,
            ),
            Ok(false) => {}
            Err(error) => cx.wb.prompt(error),
        }
    }
    cx.state.code_workspace.source_carets.insert(
        (bundle_id, active_path.to_owned()),
        interaction.cursor_char_index,
    );
    if editor_language == CodeEditorLanguage::VerilogA {
        let project_id = cx.model.workspace.content.project.id();
        crate::workbench::documents::code_workspace::drive_veriloga_live_check(
            ui.ctx(),
            &mut cx.state.code_workspace.veriloga,
            project_id,
            selected,
        );
    }
    let selection_key = (bundle_id, active_path.to_owned());
    if let Some(range) = interaction.selected_char_range {
        cx.state
            .code_workspace
            .source_selections
            .insert(selection_key, range);
    } else {
        cx.state
            .code_workspace
            .source_selections
            .remove(&selection_key);
    }
    if crate::workbench::documents::text_editor_commands::take_format_document_request(
        ui, editor_id,
    ) && let Err(error) = crate::workbench::documents::code_workspace::format_language_document(
        &mut CodeFeature::workspace_parts(cx),
        crate::state::ProjectSourceLanguage::VerilogA,
        bundle_id,
        active_path,
    ) {
        cx.wb.prompt(error);
    }
    drop(preceding_input);
    if let Some(input) = search_input {
        // Bind the search to source revisions and selection after preceding edits.
        if let Err(error) =
            crate::workbench::documents::code_workspace::open_source_search_in_bundle(
                &mut CodeFeature::workspace_parts(cx),
                crate::state::ProjectSourceLanguage::VerilogA,
                bundle_id,
                active_path,
            )
        {
            cx.wb.prompt(error);
        }
        input.route(
            ui.ctx(),
            crate::workbench::documents::code_workspace::source_search_query_id(),
        );
    }
    pressed
}

/// The line under the toolbar that states a finding about the module's
/// interface: its ports differ from the cell's schematic, they could not be
/// written, or a hand-edited symbol has drifted from them, with Update symbol
/// and Open symbol beside the drift. With nothing to say it takes no room at
/// all. It says what the reader pressed.
fn interface_strip(
    ui: &mut Ui,
    cx: &Cx<'_, CodeFeature>,
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
) -> Option<InterfaceAction> {
    use crate::workbench::app_state::model::interface_mismatch_detail;
    use crate::workbench::documents::code_workspace::{
        VerilogAInterfaceFinding, interface_finding, symbol_drift_text, veriloga_receipt,
    };

    let reference = selected.view();
    let t = Tokens::get(ui.ctx());
    let messages = cx.wb.messages();
    let veriloga = &cx.state.code_workspace.veriloga;
    let receipt = veriloga_receipt(veriloga, cx.model.workspace.content.project.id(), selected);
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
            symbol_drift_text(drift, messages),
            t.color.warn,
            None,
            if drift.updatable() {
                StripAction::UpdateSymbol
            } else {
                StripAction::None
            },
        ),
        // With nothing to fix, the compile result of a device New Verilog-A
        // device… created hands it back to the schematic it came from.
        None => {
            let (receipt, target) = receipt.and_then(|receipt| {
                let handoff = receipt.placement.as_ref()?;
                (&handoff.view == reference)
                    .then(|| veriloga_placement_target(cx.model, handoff))
                    .flatten()
                    .map(|target| (receipt, target))
            })?;
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
        return None;
    }
    match action {
        StripAction::None => None,
        StripAction::UpdateSymbol => Some(InterfaceAction::UpdateSymbol),
        StripAction::Place(_) => Some(InterfaceAction::Place),
    }
}

/// What the reader pressed on the interface strip.
pub(crate) enum InterfaceAction {
    /// Update symbol, beside a hand-edited symbol's drift.
    UpdateSymbol,
    /// Place the new device in the schematic the strip named.
    Place,
}

/// What the interface strip offers.
enum StripAction {
    /// The strip states a finding with nothing to press.
    None,
    /// Bring a hand-edited symbol up to date and take its author to it.
    UpdateSymbol,
    /// Place the new device in the schematic named by the label.
    Place(String),
}

fn current_diagnostics(
    cx: &Cx<'_, CodeFeature>,
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
) -> Option<std::sync::Arc<CodeDiagnosticCollection>> {
    crate::workbench::documents::code_workspace::veriloga_current_diagnostics(
        &cx.state.code_workspace.veriloga,
        cx.model.workspace.content.project.id(),
        selected,
    )
}

/// The view has no bundle to show.
///
/// A Verilog-A cell view without its bundle is a defect of that view, and the
/// resolver's error names it exactly. Nothing here offers a remedy that is not
/// one: there is no project-wide source to fall back to.
fn missing_source(ui: &mut Ui, cx: &Cx<'_, CodeFeature>, error: &str) {
    super::super::design_system::empty_state(
        ui,
        super::super::design_system::WorkbenchIcon::VerilogA,
        &cx.wb.messages().text(MessageId::VerilogANoSource),
        error,
    );
}
