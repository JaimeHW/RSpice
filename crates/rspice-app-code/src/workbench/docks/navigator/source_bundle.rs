//! The Verilog-A source navigator.
//!
//! It shows one file at a time out of a project-owned bundle, so the rail's
//! first job is choosing which. Before this existed the selection had no UI
//! writer at all: a bundle's included files and build profile were authored
//! and then unreachable, because only the bundle root was ever opened.
//!
//! Rows state the role the bundle records, never a guess from the filename,
//! and only a role other than ordinary source: a label every row wears tells
//! the reader nothing.
//!
//! Every operation on a file is its row's: its menu holds them all, and F2
//! and Delete do the two a list's keys always do. Below the files, the
//! Verilog-A rail states what the file in front of the reader includes and
//! where each include resolved, which files include it, what it declares,
//! and where the cell is placed.

use super::rail;
use super::*;
use egui::{Key, Modifiers, Response, ScrollArea, Ui, Vec2};

use crate::ui::panels::{WorkbenchIcon, section_header};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use crate::state::{ProjectSourceBundle, ProjectSourceId};
use crate::workbench::app::dialogs::source_document::leaf_name;
use crate::workbench::app::dialogs::source_file_form::delete_refusal;
use crate::workbench::app_state::PlatformState;
use crate::workbench::app_state::cell_usage::CellInstanceUse;
use crate::workbench::code::commands::LifecycleAction;
use crate::workbench::design_system::{PANEL_SECTION_H, Problem, elide_text, paint_problem_mark};
use crate::workbench::documents::source_editing::{
    self, SourceEditingState, SourceFileAction, SourceIncludeTarget, SourceNavigation,
    SourceOutlineGroup, VerilogAHeaderView, document_navigation,
};
use crate::workbench::{MessageCatalog, MessageId};
use rspice_app_state::workbench::app_state::model::Model;

/// What a source bundle rail reads, and writes in place: the model, the
/// Verilog-A editor's state, the platform's and the message catalogue.
pub(crate) struct SourceRail<'a> {
    pub(crate) model: &'a Model,
    pub(crate) source_editing: &'a mut SourceEditingState,
    pub(crate) platform: &'a PlatformState,
    pub(crate) messages: MessageCatalog,
}

/// What a rail asked of the whole application once it had drawn.
#[derive(Debug)]
pub(crate) enum RailAction {
    /// Say why the rail's choice could not be carried out.
    Prompt(String),
    /// Open `action`'s form for `path`.
    FileAction {
        bundle_id: ProjectSourceId,
        action: SourceFileAction,
        path: String,
    },
    /// Ask the shell for `action` on the file at `path`: its revision
    /// history, or replacing it from a picked file.
    Document {
        action: LifecycleAction,
        path: String,
    },
    /// Open the schematic an instance of the cell is placed in.
    OpenInstance(CellInstanceUse),
}

/// The rail of a Verilog-A cell view's bundle, with its file operations.
pub(super) fn veriloga(ui: &mut Ui, parts: &mut SourceRail<'_>) -> Option<RailAction> {
    let reference = parts.model.workspace.content.active_view.clone();
    let messages = parts.messages;
    let actions = active_bundle(parts.model).map(|bundle| FileActions {
        bundle_id: bundle.id(),
        delete_refusals: std::iter::once(bundle.root().logical_path())
            .chain(bundle.files().iter().map(|file| file.logical_path()))
            .filter_map(|path| {
                delete_refusal(bundle, path, messages).map(|reason| (path.to_owned(), reason))
            })
            .collect(),
        held: source_editing::source_file_mutation_block_reason_in(
            parts.model,
            parts.source_editing,
            parts.platform,
            messages,
        )
        .or_else(|| {
            parts
                .model
                .active_view_read_only()
                .then(|| messages.text(MessageId::SourceReadOnlyDocumentDetail))
        }),
    });
    let rail = Rail {
        id_salt: "workbench.veriloga.navigator",
        // One list: the build profile is a file of the bundle like any
        // other, and its row says what it is.
        files_title: MessageId::SourceNavigatorFiles,
        files: bundle_rows(parts.model),
        active: active_path(parts.model, parts.source_editing),
        actions,
        used_in: Some(
            parts
                .model
                .cell_instances(&reference.library, &reference.cell, None),
        ),
    };
    show_rail(ui, parts, rail)
}

/// Everything one rail lists, gathered before it paints.
struct Rail {
    id_salt: &'static str,
    files_title: MessageId,
    files: Vec<BundleRow>,
    active: Option<String>,
    actions: Option<FileActions>,
    /// The cell's placed instances, for a rail whose bundle is a cell view's.
    used_in: Option<Vec<CellInstanceUse>>,
}

/// The operations on a bundle's files, offered by the rail that lists them:
/// New, Rename and Delete in its header, and every operation on a file in
/// its row's menu.
///
/// Each opens its own form or dialog: rewriting a bundle's file graph is
/// confirmed and committed, never applied by a single click in a list. The
/// controls are neutral; danger is the form's to state.
struct FileActions {
    bundle_id: ProjectSourceId,
    /// The files the bundle refuses to delete, and why.
    delete_refusals: Vec<(String, String)>,
    /// Why no file can change now, when none can: what holds every file
    /// operation, or a cell that is read-only.
    held: Option<String>,
}

impl FileActions {
    /// Why `operation` cannot apply to `path`, when it cannot. Reading the
    /// history changes nothing, so only what writes is held. The bundle
    /// refuses to delete its main file, a file a role binds and a file that
    /// still includes another, so the control says so before the click
    /// rather than after it.
    fn refusal(&self, operation: FileOperation, path: &str) -> Option<String> {
        if operation == FileOperation::History {
            return None;
        }
        if let Some(held) = &self.held {
            return Some(held.clone());
        }
        if operation != FileOperation::File(SourceFileAction::Delete) {
            return None;
        }
        self.delete_refusals
            .iter()
            .find(|(refused, _)| crate::state::project_source_paths_equal(refused, path))
            .map(|(_, reason)| reason.clone())
    }
}

/// One operation on a file of the rail.
#[derive(Clone, Copy, PartialEq, Eq)]
enum FileOperation {
    /// A transaction with a form of its own.
    File(SourceFileAction),
    /// The revision history of the file's cell.
    History,
    /// Replace the file's text from a picked file.
    ReplaceFromFile,
}

impl FileOperation {
    /// What a press on the operation asks the rail for, on `path`.
    fn request(self, path: &str) -> RailRequest {
        let path = path.to_owned();
        match self {
            Self::File(action) => RailRequest::FileAction(action, path),
            Self::History => RailRequest::Document(LifecycleAction::OpenHistory, path),
            Self::ReplaceFromFile => RailRequest::Document(LifecycleAction::ReplaceFromFile, path),
        }
    }
}

/// A file row's menu, top to bottom: New file, which is included from the
/// row's file; the operations that give the file another name, place or
/// copy; its history and its text; and Delete last, past a separator. Each
/// names the key that does it from the row, when one does.
const FILE_MENU: [(MessageId, FileOperation, Option<MessageId>); 7] = [
    (
        MessageId::SourceNewFile,
        FileOperation::File(SourceFileAction::New),
        None,
    ),
    (
        MessageId::SourceRename,
        FileOperation::File(SourceFileAction::Rename),
        Some(MessageId::SourceRenameKey),
    ),
    (
        MessageId::SourceMove,
        FileOperation::File(SourceFileAction::Move),
        None,
    ),
    (
        MessageId::SourceDuplicate,
        FileOperation::File(SourceFileAction::Duplicate),
        None,
    ),
    (
        MessageId::SourceRevisionHistory,
        FileOperation::History,
        None,
    ),
    (
        MessageId::SourceReplaceFromFile,
        FileOperation::ReplaceFromFile,
        None,
    ),
    (
        MessageId::SourceDelete,
        FileOperation::File(SourceFileAction::Delete),
        Some(MessageId::SourceDeleteKey),
    ),
];

/// The height at which the controls are sized for a finger.
const TOUCH_CONTROL_HEIGHT: f32 = 44.0;

const FILE_ACTIONS: [(MessageId, SourceFileAction, WorkbenchIcon); 3] = [
    (
        MessageId::SourceNewFile,
        SourceFileAction::New,
        WorkbenchIcon::Add,
    ),
    (
        MessageId::SourceRename,
        SourceFileAction::Rename,
        WorkbenchIcon::Text,
    ),
    (
        MessageId::SourceDelete,
        SourceFileAction::Delete,
        WorkbenchIcon::Trash,
    ),
];

/// The side of a rail's icon: the height of the header it sits in with a
/// mouse, and as large as every other control where a finger is the pointer.
fn file_action_side(ctx: &egui::Context) -> f32 {
    let control = Tokens::get(ctx).metrics.ctl_h;
    if control >= TOUCH_CONTROL_HEIGHT {
        control
    } else {
        PANEL_SECTION_H
    }
}

/// The file operations as icons at the right of a section header that spans
/// `header`, acting on the file in front of the reader, and Add files, which
/// adds files from disk beside it.
fn header_file_actions(
    ui: &mut Ui,
    header: egui::Rect,
    actions: &FileActions,
    active: &str,
    messages: MessageCatalog,
) -> Option<RailRequest> {
    let side = file_action_side(ui.ctx());
    let mut strip = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(header.shrink2(Vec2::new(6.0, 0.0)))
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
    );
    strip.spacing_mut().item_spacing.x = 2.0;
    let mut requested = None;
    for (label, action, icon) in FILE_ACTIONS.into_iter().rev() {
        let label = messages.text(label);
        let refusal = actions.refusal(FileOperation::File(action), active);
        let response = strip
            .add_enabled_ui(refusal.is_none(), |ui| {
                crate::workbench::design_system::icon_button(
                    ui,
                    icon,
                    &label,
                    false,
                    Vec2::splat(side),
                )
            })
            .inner;
        let response = match refusal {
            Some(reason) => response.on_disabled_hover_text(reason),
            None => response,
        };
        if response.clicked() {
            requested = Some(RailRequest::FileAction(action, active.to_owned()));
        }
    }
    let add = strip
        .add_enabled_ui(actions.held.is_none(), |ui| {
            crate::workbench::design_system::icon_button(
                ui,
                WorkbenchIcon::Folder,
                &messages.text(MessageId::SourceAddFiles),
                false,
                Vec2::splat(side),
            )
        })
        .inner;
    let add = match &actions.held {
        Some(reason) => add.on_disabled_hover_text(reason),
        None => add,
    };
    if add.clicked() {
        requested = Some(RailRequest::AddFiles(active.to_owned()));
    }
    requested
}

/// Every operation on one file, from its row's menu. A refused one is held,
/// and its hover says why.
fn row_file_menu(
    response: &Response,
    actions: &FileActions,
    path: &str,
    messages: MessageCatalog,
) -> Option<RailRequest> {
    let mut requested = None;
    let (popup, _) = row_context_menu(response);
    popup.show(|ui| {
        for (label, operation, key) in FILE_MENU {
            if operation == FileOperation::File(SourceFileAction::Delete) {
                ui.separator();
            }
            let refusal = actions.refusal(operation, path);
            let mut item = egui::Button::new(messages.text(label));
            if let Some(key) = key {
                item = item.shortcut_text(messages.text(key));
            }
            let item = ui.add_enabled(refusal.is_none(), item);
            let item = match refusal {
                Some(reason) => item.on_disabled_hover_text(reason),
                None => item,
            };
            if item.clicked() {
                requested = Some(operation.request(path));
                ui.close();
            }
        }
    });
    requested
}

/// One selectable file in a project-owned source bundle.
struct BundleRow {
    logical_path: String,
    label: String,
    build_profile: bool,
}

fn bundle_rows(model: &Model) -> Vec<BundleRow> {
    let Some(bundle) = active_bundle(model) else {
        return Vec::new();
    };
    let root = bundle.root();
    std::iter::once(root.logical_path())
        .chain(bundle.files().iter().map(|file| file.logical_path()))
        .map(|logical_path| BundleRow {
            label: leaf_name(logical_path),
            logical_path: logical_path.to_owned(),
            build_profile: bundle.is_build_profile(logical_path),
        })
        .collect()
}

/// What the bundle records this file as, when that is something other than
/// ordinary source: the build profile, in the short form a rail already
/// scoped to Verilog-A states.
const fn role_message(build_profile: bool) -> Option<MessageId> {
    if build_profile {
        Some(MessageId::SourceNavigatorRoleBuildProfile)
    } else {
        None
    }
}

/// The bundle this rail lists: the one every editor host and command resolves,
/// so the rail can never offer the files of a bundle the editor is not showing.
fn active_bundle(model: &Model) -> Option<&crate::state::ProjectSourceBundle> {
    model.workspace.active_source_bundle()
}

fn active_path(model: &Model, code: &SourceEditingState) -> Option<String> {
    source_editing::active_bundle_path(&model.workspace, code)
}

fn select(model: &Model, code: &mut SourceEditingState, logical_path: &str) {
    source_editing::select_source_document(&model.workspace, code, logical_path);
}

/// What a click in the rail asked for, applied once the rows are painted.
enum RailRequest {
    Select(String),
    FileAction(SourceFileAction, String),
    /// Ask the shell for a choice on the file at the path.
    Document(LifecycleAction, String),
    /// Say why a key's file operation is refused.
    Refused(String),
    /// Add files from disk to the bundle, beside `path`.
    AddFiles(String),
    /// Edit `path` with its caret on `line`.
    Reveal {
        path: String,
        line: usize,
    },
    /// Read a built-in header in the editor well.
    ViewHeader(&'static str),
    OpenInstance(CellInstanceUse),
}

fn show_rail(ui: &mut Ui, parts: &mut SourceRail<'_>, rail: Rail) -> Option<RailAction> {
    let messages = parts.messages;
    let bundle = parts.model.workspace.active_source_bundle();
    let bundle_id = bundle.map(crate::state::ProjectSourceBundle::id);
    let navigation = match (rail.active.as_deref(), bundle) {
        (Some(path), Some(bundle)) => document_navigation(
            parts.model.workspace.content.project.id(),
            bundle,
            &mut parts.source_editing.source_index_cache,
            path,
        ),
        _ => SourceNavigation::default(),
    };
    let importers = match (rail.active.as_deref(), bundle) {
        (Some(path), Some(bundle)) => importers(bundle, path),
        _ => Vec::new(),
    };
    let header = bundle_id
        .zip(rail.active.as_deref())
        .and_then(|(bundle_id, path)| {
            source_editing::viewed_veriloga_header(&parts.source_editing.veriloga, bundle_id, path)
        })
        .map(|(identity, _)| identity);
    let mut request = None;

    rail::open(ui.ctx());
    ScrollArea::vertical()
        .id_salt(rail.id_salt)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            'rows: {
                if rail.files.is_empty() {
                    muted(ui, &messages.text(MessageId::SourceNavigatorNoBundle));
                    break 'rows;
                }
                files_section(ui, &rail, header, messages, &mut request);
                let Some(active) = rail.active.as_deref() else {
                    break 'rows;
                };
                includes_section(ui, &navigation, &importers, header, messages, &mut request);
                outline_section(ui, &navigation, active, messages, &mut request);
                if let Some(used_in) = &rail.used_in {
                    used_in_section(ui, used_in, messages, &mut request);
                }
            }
            // Inside the area and below the last row: the keys need the whole
            // rail, and the row they land on is scrolled to by this area.
            let _ = rail::traverse(ui, false);
        });

    match request {
        None => {}
        Some(RailRequest::Select(path)) => {
            select(parts.model, parts.source_editing, &path);
        }
        Some(RailRequest::Refused(reason)) => return Some(RailAction::Prompt(reason)),
        Some(RailRequest::FileAction(action, path)) => {
            if let Some(file_actions) = rail.actions.as_ref() {
                return Some(RailAction::FileAction {
                    bundle_id: file_actions.bundle_id,
                    action,
                    path,
                });
            }
        }
        Some(RailRequest::Document(action, path)) => {
            return Some(RailAction::Document { action, path });
        }
        Some(RailRequest::AddFiles(path)) => {
            if let Some(file_actions) = rail.actions.as_ref() {
                let requested = match source_editing::source_file_mutation_block_reason_in(
                    parts.model,
                    parts.source_editing,
                    parts.platform,
                    parts.messages,
                ) {
                    Some(reason) => Err(reason),
                    None => source_editing::request_veriloga_add_files(
                        parts.model,
                        &mut parts.source_editing.veriloga,
                        file_actions.bundle_id,
                        &path,
                    ),
                };
                if let Err(error) = requested {
                    return Some(RailAction::Prompt(error));
                }
            }
        }
        Some(RailRequest::Reveal { path, line }) => {
            select(parts.model, parts.source_editing, &path);
            if let Some(bundle_id) = bundle_id {
                // The shared code editor consumes this on its next frame,
                // which is the frame it shows `path` in.
                crate::workbench::documents::text_editor_commands::queue_reveal_line(
                    ui.ctx(),
                    source_editing::source_editor_id(bundle_id, &path),
                    line,
                );
            }
        }
        Some(RailRequest::ViewHeader(identity)) => {
            if let (Some(bundle_id), Some(over)) = (bundle_id, rail.active) {
                parts.source_editing.veriloga.viewed_header = Some(VerilogAHeaderView {
                    bundle_id,
                    over,
                    identity,
                });
            }
        }
        Some(RailRequest::OpenInstance(usage)) => return Some(RailAction::OpenInstance(usage)),
    }
    None
}

/// The bundle's files, with the file operations in the section's header: the
/// files are what they create, rename and delete.
fn files_section(
    ui: &mut Ui,
    rail: &Rail,
    header: Option<&'static str>,
    messages: MessageCatalog,
    request: &mut Option<RailRequest>,
) {
    let top = ui.cursor().top();
    // An icon sized for a finger is taller than the header, so the difference
    // goes either side of the title and the two stay level.
    let has_icons = rail.actions.is_some() && rail.active.is_some();
    let lead = if has_icons {
        ((file_action_side(ui.ctx()) - PANEL_SECTION_H) * 0.5).max(0.0)
    } else {
        0.0
    };
    ui.add_space(lead);
    section_header(
        ui,
        &messages.text(rail.files_title),
        rail.actions
            .is_none()
            .then(|| rail.files.len().to_string())
            .as_deref(),
    );
    ui.add_space(lead);
    if let (Some(file_actions), Some(path)) = (rail.actions.as_ref(), rail.active.as_deref()) {
        let band = egui::Rect::from_x_y_ranges(ui.max_rect().x_range(), top..=ui.cursor().top());
        if let Some(action) = header_file_actions(ui, band, file_actions, path, messages) {
            *request = Some(action);
        }
    }
    for row in &rail.files {
        // A header open in the editor is what the reader is looking at; no
        // file row claims the editor then.
        let is_active =
            header.is_none() && rail.active.as_deref() == Some(row.logical_path.as_str());
        let role = role_message(row.build_profile).map(|role| messages.text(role));
        let response = nav_row_response(
            ui,
            source_row_icon(row.build_profile),
            &row.label,
            is_active,
            role.as_deref(),
        );
        if response.clicked() {
            *request = Some(RailRequest::Select(row.logical_path.clone()));
        }
        if let Some(file_actions) = rail.actions.as_ref() {
            if let Some(chosen) =
                row_file_menu(&response, file_actions, &row.logical_path, messages)
            {
                *request = Some(chosen);
            }
            if let Some(keyed) = row_file_keys(ui, &response, file_actions, &row.logical_path) {
                *request = Some(keyed);
            }
        }
    }
}

/// F2 renames the file row holding the keyboard and Delete deletes it, as in
/// every file list. A refused one says why rather than doing nothing.
fn row_file_keys(
    ui: &Ui,
    response: &Response,
    actions: &FileActions,
    path: &str,
) -> Option<RailRequest> {
    if !response.has_focus() {
        return None;
    }
    let action = if ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::F2)) {
        SourceFileAction::Rename
    } else if ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Delete)) {
        SourceFileAction::Delete
    } else {
        return None;
    };
    Some(match actions.refusal(FileOperation::File(action), path) {
        Some(reason) => RailRequest::Refused(reason),
        None => RailRequest::FileAction(action, path.to_owned()),
    })
}

/// The files of `bundle` that include `logical_path`, as the bundle records
/// them: persisted dependency records, never a re-parse of the text, so the
/// rail cannot claim an edge the project does not hold.
fn importers(bundle: &ProjectSourceBundle, logical_path: &str) -> Vec<String> {
    let mut importers: Vec<String> = bundle
        .dependencies()
        .iter()
        .filter(|dependency| {
            crate::state::project_source_paths_equal(dependency.imported(), logical_path)
        })
        .map(|dependency| dependency.importer().to_owned())
        .collect();
    importers.sort();
    importers.dedup();
    importers
}

/// Every include the file pulls in and where it resolved, then the files
/// that include it. A project file opens for editing, a built-in header
/// opens read-only in the same well, and a name nothing holds is marked and
/// takes the reader to its directive. The header counts the files included;
/// the files that include this one are listed under their own caption.
fn includes_section(
    ui: &mut Ui,
    navigation: &SourceNavigation,
    importers: &[String],
    header: Option<&'static str>,
    messages: MessageCatalog,
    request: &mut Option<RailRequest>,
) {
    section_header(
        ui,
        &messages.text(MessageId::SourceNavigatorIncludes),
        Some(&navigation.includes.len().to_string()),
    );
    for include in &navigation.includes {
        match &include.target {
            SourceIncludeTarget::Project(path) => {
                let response =
                    nav_row_response(ui, source_row_icon(false), &leaf_name(path), false, None)
                        .on_hover_text(path);
                if response.clicked() {
                    *request = Some(RailRequest::Select(path.clone()));
                }
            }
            SourceIncludeTarget::BuiltIn(identity) => {
                let response = nav_row_response(
                    ui,
                    source_row_icon(false),
                    &leaf_name(identity),
                    header == Some(*identity),
                    Some(&messages.text(MessageId::SourceNavigatorIncludeBuiltIn)),
                )
                .on_hover_text(*identity);
                if response.clicked() {
                    *request = Some(RailRequest::ViewHeader(identity));
                }
            }
            SourceIncludeTarget::Missing { including, line } => {
                let response =
                    nav_row_response(ui, source_row_icon(false), &include.requested, false, None);
                // The mark carries the severity at the row's trailing edge,
                // where the other rows state where their include resolved;
                // the name beside it stays neutral.
                let rect = response.rect;
                paint_problem_mark(
                    ui.painter(),
                    egui::Rect::from_center_size(
                        egui::pos2(rect.right() - rect.height() * 0.5, rect.center().y),
                        Vec2::splat(rect.height() * 0.4),
                    ),
                    Problem::Failure,
                );
                let announced = Problem::Failure.announce(&include.requested);
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::SelectableLabel,
                        ui.is_enabled(),
                        &announced,
                    )
                });
                let response = response
                    .on_hover_text(messages.text(MessageId::SourceNavigatorIncludeNotFound));
                if response.clicked() {
                    *request = Some(RailRequest::Reveal {
                        path: including.clone(),
                        line: *line,
                    });
                }
            }
        }
    }
    if importers.is_empty() {
        return;
    }
    group_caption(ui, &messages.text(MessageId::SourceNavigatorIncludedBy));
    for importer in importers {
        let response = nav_row_response(
            ui,
            source_row_icon(false),
            &leaf_name(importer),
            false,
            None,
        )
        .on_hover_text(importer);
        if response.clicked() {
            *request = Some(RailRequest::Select(importer.clone()));
        }
    }
}

/// What the file declares, a row per name, grouped by kind under a caption.
fn outline_section(
    ui: &mut Ui,
    navigation: &SourceNavigation,
    active: &str,
    messages: MessageCatalog,
    request: &mut Option<RailRequest>,
) {
    section_header(
        ui,
        &messages.text(MessageId::SourceNavigatorOutline),
        Some(&navigation.outline.len().to_string()),
    );
    let mut group = None;
    for row in &navigation.outline {
        if group != Some(row.group) {
            group_caption(ui, &messages.text(outline_group_title(row.group)));
            group = Some(row.group);
        }
        let clicked = {
            // The name is the row's identity and reads whole; the detail
            // takes what is left beside it and elides. A quarter of the
            // row is the icon column and the gaps around the detail.
            let name_width = ui
                .painter()
                .layout_no_wrap(
                    row.name.clone(),
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    egui::Color32::PLACEHOLDER,
                )
                .size()
                .x;
            let detail = elide_text(
                ui,
                &row.detail,
                &theme::mono(tokens::FS_0, FontWeight::Regular),
                ui.available_width() * 0.75 - name_width,
            );
            let response = nav_row_response(
                ui,
                outline_group_icon(row.group),
                &row.name,
                false,
                (!detail.is_empty()).then_some(detail.as_str()),
            );
            let response = if detail == row.detail {
                response
            } else {
                response.on_hover_text(&row.detail)
            };
            response.clicked()
        };
        if clicked {
            *request = Some(RailRequest::Reveal {
                path: active.to_owned(),
                line: row.line,
            });
        }
    }
}

/// A group's caption, taking exactly one row of the list.
///
/// The caption is the navigator's own label row, which reserves a minimum
/// height of its own a point taller than a compact row. Laid into a band of
/// the list's row height, it keeps the outline on the pitch of every other
/// row in the rail instead of drifting a point per group; centred on the
/// band at the height it measured last frame, its text sits on the line a
/// row's text does.
fn group_caption(ui: &mut Ui, text: &str) {
    let band = egui::Rect::from_min_size(
        ui.cursor().min,
        Vec2::new(ui.available_width(), Tokens::get(ui.ctx()).metrics.row_h),
    );
    let measured = ui.id().with("group-caption-height");
    let height = ui
        .data(|data| data.get_temp::<f32>(measured))
        .unwrap_or(band.height());
    let mut caption = ui.new_child(
        egui::UiBuilder::new().max_rect(egui::Rect::from_center_size(
            band.center(),
            Vec2::new(band.width(), height),
        )),
    );
    empty_navigator_row(&mut caption, text);
    let drawn = caption.min_rect().height();
    ui.data_mut(|data| data.insert_temp(measured, drawn));
    ui.advance_cursor_after_rect(band);
}

const fn outline_group_title(group: SourceOutlineGroup) -> MessageId {
    match group {
        SourceOutlineGroup::Module => MessageId::SourceNavigatorOutlineModule,
        SourceOutlineGroup::Ports => MessageId::SourceNavigatorOutlinePorts,
        SourceOutlineGroup::Parameters => MessageId::SourceNavigatorOutlineParameters,
        SourceOutlineGroup::Variables => MessageId::SourceNavigatorOutlineVariables,
        SourceOutlineGroup::Nets => MessageId::SourceNavigatorOutlineNets,
        SourceOutlineGroup::Functions => MessageId::SourceNavigatorOutlineFunctions,
    }
}

/// A kind's own glyph where the icon set has one that fits. Variables and
/// functions have none, and share the code mark; their caption names them.
const fn outline_group_icon(group: SourceOutlineGroup) -> WorkbenchIcon {
    match group {
        SourceOutlineGroup::Module => WorkbenchIcon::VerilogA,
        SourceOutlineGroup::Ports => WorkbenchIcon::Pin,
        SourceOutlineGroup::Parameters => WorkbenchIcon::Sliders,
        SourceOutlineGroup::Nets => WorkbenchIcon::Wire,
        SourceOutlineGroup::Variables | SourceOutlineGroup::Functions => WorkbenchIcon::Code,
    }
}

/// The cell's instances across the project's schematics. Its header counts
/// them, which is the whole answer when there are none.
fn used_in_section(
    ui: &mut Ui,
    used_in: &[CellInstanceUse],
    messages: MessageCatalog,
    request: &mut Option<RailRequest>,
) {
    section_header(
        ui,
        &messages.text(MessageId::SourceNavigatorUsedIn),
        Some(&used_in.len().to_string()),
    );
    for usage in used_in {
        let response = nav_row_response(
            ui,
            WorkbenchIcon::Instance,
            &format!("{} / {}", usage.schematic.cell, usage.instance),
            false,
            None,
        )
        .on_hover_text(usage.schematic.display_path());
        if response.clicked() {
            *request = Some(RailRequest::OpenInstance(usage.clone()));
        }
    }
}

fn source_row_icon(build_profile: bool) -> WorkbenchIcon {
    if build_profile {
        // Build settings rather than authored logic.
        WorkbenchIcon::Sliders
    } else {
        WorkbenchIcon::Code
    }
}

#[cfg(test)]
mod tests;
