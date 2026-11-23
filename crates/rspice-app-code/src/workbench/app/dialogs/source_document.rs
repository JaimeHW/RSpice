//! The source document dialogs: Deck and include folders over a deck, and
//! the revision history a Verilog-A file's menu on the Files rail opens.
//!
//! A Verilog-A file's operations are its row's on the Files rail; each opens
//! its own form or dialog, and what they write is carried out here, by the
//! shell, once the dialog that asked has drawn.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use egui::{Context, Ui};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, DIALOG_BODY_INSET, Dialog, DialogChoice, DialogInitialFocus, DialogSize, IconButton,
    PROBLEM_MARK_ADVANCE, PROBLEM_MARK_SIDE, Problem, band_header, control_height, name_control,
    paint_problem_mark,
};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::code::commands::{LifecycleAction, SourceDocumentRequest};
use crate::workbench::design_system::inspector::muted_inspector_copy;
use crate::workbench::design_system::{
    Fact, PANEL_INSET, fact_rows, panel_body, property_row_height, section_block_gap,
};
use crate::workbench::documents::deck_files::LineEnding;
use crate::workbench::documents::netlist_document::{self, NetlistDocumentState};
use crate::workbench::documents::source_editing::{self, SourceEditingParts, SourceEditingState};
use crate::workbench::state::{WorkbenchSession, Workspace};
use crate::workbench::{MessageCatalog, MessageId};

/// The gap between the add row's field and its buttons, and between a
/// folder row's state and its actions.
const CONTROL_GAP: f32 = 6.0;

/// The shell's `open_source_document_dialog`, over the workspace in front, the model,
/// the Code workspace's runtime state, the netlist editor's state and the
/// platform's state. Only a deck has the dialog: a Verilog-A file's
/// operations are on its row of the Files rail.
pub fn open_source_document_dialog_in(
    workspace: Workspace,
    model: &Model,
    code: &mut SourceEditingState,
    netlist: &NetlistDocumentState,
    platform: &PlatformState,
    messages: MessageCatalog,
) -> Result<(), String> {
    if workspace != Workspace::Netlist {
        return Err(messages.text(MessageId::NetlistManageNeedsDeck));
    }
    if !platform.project_lifecycle.is_open() {
        return Err(messages.text(MessageId::NetlistManageNeedsProject));
    }
    if let Some(refusal) = generated_deck_refusal(model, netlist) {
        return Err(messages.text(refusal));
    }
    code.source_document_dialog = true;
    Ok(())
}

/// Whether Deck and include folders can be opened: a deck is in front, in a
/// project that is open.
pub fn source_document_dialog_is_available(
    workspace: Workspace,
    model: &Model,
    netlist: &NetlistDocumentState,
    platform: &PlatformState,
) -> bool {
    workspace == Workspace::Netlist
        && platform.project_lifecycle.is_open()
        && generated_deck_refusal(model, netlist).is_none()
}

/// Why the netlist page has no source document to manage: the deck on it is
/// generated from the schematic and read-only, and no deck is open.
pub(crate) fn generated_deck_refusal(
    _model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<MessageId> {
    netlist
        .open_deck
        .is_none()
        .then_some(MessageId::NetlistManageGenerated)
}

pub struct NetlistLifecycleFacts {
    /// The file's name.
    logical_path: String,
    /// Where the file is in the project's folder, or on disk when it is
    /// outside it.
    project_path: String,
    line_endings: LineEnding,
    /// Where the file is on disk.
    native_path: String,
    /// Whether the file is an include the deck reads.
    include: bool,
}

/// What the dialog states of the file in front: the open deck, or an
/// include open beside it.
pub fn netlist_lifecycle_facts(
    model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<NetlistLifecycleFacts> {
    let file = netlist_document::active_deck_file_in(netlist)?;
    let native_path = file.path.display().to_string();
    let project_path = model
        .workspace
        .content
        .project
        .data_root()
        .and_then(|root| file.path.strip_prefix(root).ok())
        .map(|relative| {
            relative
                .components()
                .map(|part| part.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/")
        })
        .filter(|relative| !relative.is_empty())
        .unwrap_or_else(|| native_path.clone());
    Some(NetlistLifecycleFacts {
        logical_path: file.name(),
        project_path,
        line_endings: file.line_ending,
        native_path,
        include: netlist.front.include().is_some(),
    })
}

pub(in crate::workbench) fn leaf_name(logical_path: &str) -> String {
    logical_path
        .rsplit('/')
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or(logical_path)
        .to_owned()
}

/// Which section states a failure, chosen by the action that caused it.
#[derive(Clone, Copy)]
enum Report {
    FileTransaction,
    History,
    Console,
}

/// One spelling of a document's role, so the property row and the picker can
/// never disagree about which entry is currently selected.
///
/// The picker resolves a selection by matching the label it drew, so the label
/// has to come from the same locale the row was painted in.
pub(super) fn role_label(messages: MessageCatalog, build_profile: bool) -> String {
    messages.text(if build_profile {
        MessageId::SourceRoleBuildProfile
    } else {
        MessageId::SourceRoleOrdinary
    })
}

/// Close the netlist lifecycle dialog.
pub fn close_netlist_lifecycle(code: &mut SourceEditingState) {
    code.source_document_dialog = false;
}

/// Draw the source document dialogs over the Code workspace's runtime state,
/// the netlist editor's state, the model, the workbench session and the
/// platform's state: an open file form, else an open revision history, else
/// Deck and include folders. What a choice still needs of the whole
/// application is returned for the shell to carry out.
pub(in crate::workbench) fn render_source_document_dialog(
    code: &mut SourceEditingState,
    netlist: &mut NetlistDocumentState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    platform: &PlatformState,
    ctx: &Context,
) -> Option<SourceDocumentRequest> {
    if code.source_file_dialog.is_some() {
        return super::source_file_form::render(code, model, platform, wb.messages(), ctx);
    }
    if code.source_history.is_some() {
        return super::source_history::render(code, model, platform, wb.messages(), ctx);
    }
    if code.source_document_dialog {
        render_netlist_source_document_dialog(code, netlist, model, wb, ctx);
    }
    None
}

/// Carry out a choice on the Verilog-A document at `logical_path` that a
/// form, the revision history or the Files rail handed the shell.
pub fn apply_lifecycle_action(
    parts: &mut SourceEditingParts<'_>,
    action: LifecycleAction,
    logical_path: &str,
) {
    // Where a failure is stated is part of the action, not a guess made
    // afterwards: a rejected rename belongs beside the path field, and a
    // rejected restore beside the revision list. Deciding by whichever
    // draft happens to be open would put one inside the other's section.
    let (outcome, report) = match action {
        LifecycleAction::Commit => (
            source_editing::commit_source_file_dialog(parts).map(|_| ()),
            Report::FileTransaction,
        ),
        LifecycleAction::OpenHistory => (
            source_editing::open_source_history(parts, logical_path),
            Report::Console,
        ),
        LifecycleAction::Restore => (
            source_editing::commit_source_history_restore(parts).map(|_| ()),
            Report::History,
        ),
        // The picker and then the replacement's own confirmation take
        // over from here.
        LifecycleAction::ReplaceFromFile => (
            source_editing::request_veriloga_source_replacement(parts, logical_path),
            Report::Console,
        ),
    };
    let Err(error) = outcome else {
        return;
    };
    let stated = match report {
        Report::FileTransaction => parts
            .code
            .source_file_dialog
            .as_mut()
            .map(|draft| draft.error = Some(error.clone())),
        Report::History => parts
            .code
            .source_history
            .as_mut()
            .map(|history| history.error = Some(error.clone())),
        Report::Console => None,
    };
    // Nothing owns the failure when the draft it belonged to was cleared
    // by the same call, so the console states it instead of losing it.
    if stated.is_none() {
        parts
            .model
            .record(LogSeverity::Error, LogSource::Studio, error);
    }
}

fn render_netlist_source_document_dialog(
    code: &mut SourceEditingState,
    netlist: &NetlistDocumentState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    ctx: &Context,
) {
    let messages = wb.messages();
    let Some(facts) = netlist_lifecycle_facts(model, netlist) else {
        close_netlist_lifecycle(code);
        return;
    };
    let dialog = Dialog::new(
        messages.text(MessageId::NetlistManageDeckTitle),
        messages.text(MessageId::CommonClose),
    )
    .description(messages.format(
        MessageId::NetlistManageDeckDescription,
        &[("document", &facts.logical_path)],
    ))
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    // The rows fit at every desktop size; a bar would only reserve a
    // gutter that left the right edge wider than the left.
    .overlay_scrollbar()
    .primary_on_enter(false)
    .initial_focus(DialogInitialFocus::BodyControl);

    let mut search_paths = None;
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        search_paths = netlist_lifecycle_body(ui, code, model, &facts, messages);
        None
    });
    if let Some(paths) = search_paths {
        if let Err(error) = model
            .workspace
            .content
            .project
            .set_include_search_paths(paths)
            .map_err(|error| error.to_string())
        {
            wb.prompt(error);
        } else {
            model.workspace.content.project_metadata_dirty = true;
            // The chain decides which file every relative include
            // resolves to, so the deck's diagnostics are no longer
            // evidence about the chain that is now in force.
            model.invalidate_source_evidence();
        }
    }
    if matches!(
        choice,
        DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
    ) {
        close_netlist_lifecycle(code);
    }
}

/// The sections of a source dialog, stacked down its flush body.
pub(super) fn sections<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        add(ui)
    })
    .inner
}

/// One section of a source dialog: the dialog's band across its flush
/// body, then `body` on the dialog's inset.
fn dialog_section(ui: &mut Ui, title: &str, body: impl FnOnce(&mut Ui)) {
    band_header(ui, title, None);
    dialog_inset(ui, body);
}

/// `body` on the panel rhythm with every row on the left edge of the
/// dialog's title and footer.
pub(super) fn dialog_inset(ui: &mut Ui, body: impl FnOnce(&mut Ui)) {
    // The panel body insets its rows by its own inset; this frame makes up
    // the difference to the dialog's.
    let edge = (DIALOG_BODY_INSET - PANEL_INSET) as i8;
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(edge, 0))
        .show(ui, |ui| {
            panel_body(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                body(ui);
            });
        });
}

/// The file in front, then the include search chain; the chain as its rows
/// now read it, when one of them changed it.
fn netlist_lifecycle_body(
    ui: &mut Ui,
    code: &mut SourceEditingState,
    model: &Model,
    facts: &NetlistLifecycleFacts,
    messages: MessageCatalog,
) -> Option<Vec<std::path::PathBuf>> {
    // The dialog's body scrolls when the rows outgrow it; a scroll area of
    // its own here only withheld a gutter on the right.
    sections(ui, |ui| {
        dialog_section(ui, &messages.text(MessageId::SourceCurrentDocument), |ui| {
            let file = messages.text(if facts.include {
                MessageId::NetlistLogicalIdentity
            } else {
                MessageId::NetlistTopDeck
            });
            let line_endings = messages.text(MessageId::SourceLineEndings);
            let line_ending = facts.line_endings.label(messages);
            fact_rows(
                ui,
                &[
                    Fact::path(&file, &facts.project_path).whole(&facts.native_path),
                    Fact::new(&line_endings, &line_ending),
                ],
            );
        });
        include_search_paths(ui, code, model, messages)
    })
}

/// What a press on an include folder's row asked for.
#[derive(Clone, Copy, PartialEq, Eq)]
enum FolderAction {
    Select,
    MoveUp,
    MoveDown,
    Remove,
}

/// The project's ordered include search chain.
///
/// Order is the whole setting: the resolver walks it front to back, so which
/// of two files of the same relative name a deck gets is decided here. Every
/// row states whether its directory is present, in place, because a chain
/// entry that silently does not exist is the failure this list exists to make
/// visible.
fn include_search_paths(
    ui: &mut Ui,
    code: &mut SourceEditingState,
    model: &Model,
    messages: MessageCatalog,
) -> Option<Vec<std::path::PathBuf>> {
    let chain = rspice_simulation::netlist_preparation::IncludeSearchChain::resolve(
        model.workspace.content.project.include_search_paths(),
        model.workspace.content.project.data_root(),
    );
    let authored = model
        .workspace
        .content
        .project
        .include_search_paths()
        .to_vec();
    let unsaved = model.workspace.content.project.data_root().is_none();
    let selected = code
        .include_search_selected
        .filter(|index| *index < authored.len());
    let mut action = None;
    band_header(
        ui,
        &messages.text(MessageId::NetlistIncludeSearchPaths),
        None,
    );
    // The rows run edge to edge, as the band over them does; their words
    // keep the dialog's inset.
    for (index, entry) in chain.entries().iter().enumerate() {
        let Some(pressed) = include_folder_row(
            ui,
            index,
            authored.len(),
            entry,
            selected == Some(index),
            messages,
        ) else {
            continue;
        };
        let mut next = authored.clone();
        match pressed {
            FolderAction::Select => code.include_search_selected = Some(index),
            FolderAction::MoveUp => {
                next.swap(index - 1, index);
                code.include_search_selected = Some(index - 1);
                action = Some(next);
            }
            FolderAction::MoveDown => {
                next.swap(index, index + 1);
                code.include_search_selected = Some(index + 1);
                action = Some(next);
            }
            FolderAction::Remove => {
                next.remove(index);
                code.include_search_selected = None;
                action = Some(next);
            }
        }
    }
    dialog_inset(ui, |ui| {
        // The rows are only part of the search: the resolver looks beside
        // the deck first and goes on past them, into the library folders
        // set in Preferences and then lib/ and models/. A list that ended at
        // its last row would misstate where a name is found.
        let library_folders = !model.library_folders().is_empty();
        let whole_search = match (authored.is_empty(), library_folders) {
            (true, false) => MessageId::NetlistIncludeSearchPathEmpty,
            (true, true) => MessageId::NetlistIncludeSearchPathEmptyWithLibraryFolders,
            (false, false) => MessageId::NetlistIncludeSearchPathThen,
            (false, true) => MessageId::NetlistIncludeSearchPathThenWithLibraryFolders,
        };
        muted_inspector_copy(ui, &messages.text(whole_search));
        if unsaved && !authored.is_empty() {
            muted_inspector_copy(
                ui,
                &messages.text(MessageId::NetlistIncludeSearchPathUnsaved),
            );
        }
        section_block_gap(ui);
        if let Some(folder) = include_folder_field(ui, code, messages) {
            let mut next = authored.clone();
            next.push(folder);
            action = Some(next);
        }
    });
    action
}

/// One folder of the chain, flush to the dialog's edges: its place and its
/// path, whether it is missing, and, while the row is hovered or selected,
/// the three things that act on it alone.
fn include_folder_row(
    ui: &mut Ui,
    index: usize,
    count: usize,
    entry: &rspice_simulation::netlist_preparation::IncludeSearchEntry,
    selected: bool,
    messages: MessageCatalog,
) -> Option<FolderAction> {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let height = property_row_height(ui);
    let shown = format!("{}. {}", index + 1, entry.authored().display());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click(),
    );
    // The row is a painted galley, so nothing here reaches a screen reader
    // on its own, and a clickable sense makes it a tab stop that would
    // otherwise take focus silently and invisibly.
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            &shown,
        )
    });
    let mut pressed = response.clicked().then_some(FolderAction::Select);
    let acting = selected || ui.rect_contains_pointer(rect);
    // The row's actions, right to left from the inset; laid out only while
    // they show, so a resting list is its paths and nothing else.
    let mut right = rect.right() - DIALOG_BODY_INSET;
    if acting {
        let side = (height - 4.0).max(1.0);
        let actions = [
            (
                Icon::Trash,
                MessageId::NetlistIncludeSearchPathRemove,
                true,
                FolderAction::Remove,
            ),
            (
                Icon::ChevronDown,
                MessageId::NetlistIncludeSearchPathMoveDown,
                index + 1 < count,
                FolderAction::MoveDown,
            ),
            (
                Icon::ChevronUp,
                MessageId::NetlistIncludeSearchPathMoveUp,
                index > 0,
                FolderAction::MoveUp,
            ),
        ];
        for (icon, label, enabled, action) in actions {
            let slot = egui::Rect::from_center_size(
                egui::pos2(right - side * 0.5, rect.center().y),
                egui::Vec2::splat(side),
            );
            right -= side + 2.0;
            let label = messages.text(label);
            // A child of its own, so the row below starts where this one
            // ends rather than where its last action does.
            let mut slot_ui = ui.new_child(egui::UiBuilder::new().max_rect(slot));
            let button = IconButton::new(icon)
                .side(side)
                .enabled(enabled)
                .tooltip(&label)
                .show(&mut slot_ui);
            if button.clicked() {
                pressed = Some(action);
            }
        }
        right -= CONTROL_GAP - 2.0;
    }
    if ui.is_rect_visible(rect) {
        if selected {
            ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        }
        // A path that resolves is every path's normal state, so only one
        // that does not is marked and named.
        let missing = rspice_simulation::netlist_preparation::IncludeSearchChain::states_presence()
            && !entry.exists();
        if missing {
            let state = ui.painter().text(
                egui::pos2(right, rect.center().y),
                egui::Align2::RIGHT_CENTER,
                messages.text(MessageId::NetlistIncludeSearchPathMissing),
                font.clone(),
                t.color.text_dim,
            );
            right = state.left() - PROBLEM_MARK_ADVANCE;
            paint_problem_mark(
                ui.painter(),
                egui::Rect::from_min_size(
                    egui::pos2(right, rect.center().y - PROBLEM_MARK_SIDE * 0.5),
                    egui::Vec2::splat(PROBLEM_MARK_SIDE),
                ),
                Problem::Warning,
            );
            right -= CONTROL_GAP;
        }
        let left = rect.left() + DIALOG_BODY_INSET;
        ui.painter()
            .with_clip_rect(egui::Rect::from_x_y_ranges(
                left..=right.max(left),
                rect.y_range(),
            ))
            .text(
                egui::pos2(left, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &shown,
                font,
                t.color.text,
            );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(entry.resolved().display().to_string());
    pressed
}

/// The add row under the chain: the path field across the row, then
/// Browse… and Add at the field's height. Returns the folder it adds.
fn include_folder_field(
    ui: &mut Ui,
    code: &mut SourceEditingState,
    messages: MessageCatalog,
) -> Option<std::path::PathBuf> {
    let height = control_height(ui);
    let add_label = messages.text(MessageId::NetlistIncludeSearchPathAdd);
    let add_named = messages.text(MessageId::NetlistIncludeSearchPathAddNamed);
    let field_name = messages.text(MessageId::NetlistIncludeSearchPathField);
    let typed = !code.include_search_draft.trim().is_empty();
    let add = Button::new(&add_label)
        .accessible_label(&add_named)
        .enabled(typed);
    #[cfg(not(target_arch = "wasm32"))]
    let browse_label = messages.text(MessageId::NetlistIncludeSearchPathBrowse);
    #[cfg(not(target_arch = "wasm32"))]
    let browse = Button::new(&browse_label);
    #[cfg(not(target_arch = "wasm32"))]
    let buttons = browse.measured_width(ui) + add.measured_width(ui) + 2.0 * CONTROL_GAP;
    #[cfg(target_arch = "wasm32")]
    let buttons = add.measured_width(ui) + CONTROL_GAP;
    let mut added = None;
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), height),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = CONTROL_GAP;
            let field = ui.add_sized(
                egui::vec2((ui.available_width() - buttons).max(1.0), height),
                egui::TextEdit::singleline(&mut code.include_search_draft)
                    .font(theme::field_mono())
                    .hint_text(messages.text(MessageId::NetlistIncludeSearchPathHint))
                    .vertical_align(egui::Align::Center)
                    .margin(egui::Margin::symmetric(8, 4)),
            );
            name_control(ui, &field, &field_name);
            let entered = typed
                && field.lost_focus()
                && ui.input(|input| input.key_pressed(egui::Key::Enter));
            #[cfg(not(target_arch = "wasm32"))]
            if browse.show(ui).clicked()
                && let Some(picked) = rfd::FileDialog::new()
                    .set_title(messages.text(MessageId::NetlistIncludeSearchPaths))
                    .pick_folder()
            {
                added = Some(picked);
            }
            if add.show(ui).clicked() || entered {
                added = Some(std::path::PathBuf::from(code.include_search_draft.trim()));
                code.include_search_draft.clear();
            }
        },
    );
    added
}

#[cfg(test)]
mod tests;
