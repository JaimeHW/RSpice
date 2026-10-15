//! The Verilog-A and Automation source navigators.
//!
//! Both show one file at a time out of a project-owned bundle, so the rail's
//! first job is choosing which. Before this existed the selection had no UI
//! writer at all: a bundle's build profile, run plan, environment lock, and
//! permission manifest were authored by the workspace and then unreachable,
//! because only the bundle root was ever opened.
//!
//! Rows state the role the bundle records, never a guess from the filename,
//! and only a role other than ordinary source: a label every row wears tells
//! the reader nothing. A starter name is not identity — `characterize.py` is
//! the Python entry because the bundle says so, not because of its extension.
//!
//! Below the files, the Verilog-A rail states what the file in front of the
//! reader includes and where each include resolved, what it declares, and
//! where the cell is placed.

use super::*;

use crate::state::{ProjectSourceId, ProjectSourceLanguage, ProjectSourceRole};
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::cell_usage::CellInstanceUse;
use crate::workbench::design_system::{Problem, elide_text, paint_problem_mark};
use crate::workbench::documents::code_workspace::{
    self, CodeSourceFileAction, SourceIncludeTarget, SourceNavigation, SourceOutlineGroup,
    VerilogAHeaderView, document_navigation,
};
use crate::workbench::{MessageCatalog, MessageId};

/// The rail of a Verilog-A cell view's bundle, with its file operations.
pub(super) fn veriloga(ui: &mut Ui, app: &mut RSpiceApp) {
    let language = ProjectSourceLanguage::VerilogA;
    let reference = app.state.workspace.content.active_view.clone();
    let actions = active_bundle(&app.state, language).map(|bundle| FileActions {
        bundle_id: bundle.id(),
        root: bundle.root().logical_path().to_owned(),
        writable: !app.state.active_view_read_only()
            && code_workspace::source_file_mutation_block_reason(app, language).is_none(),
    });
    let rail = Rail {
        id_salt: "workbench.veriloga.navigator",
        // One list: the build profile is a file of the bundle like any
        // other, and its row says what it is.
        files_title: MessageId::CodeNavigatorFiles,
        files: bundle_rows(&app.state, language),
        active: active_path(&app.state, language),
        language,
        actions,
        used_in: Some(
            app.state
                .cell_instances(&reference.library, &reference.cell, None),
        ),
    };
    show_rail(ui, app, rail);
}

pub(super) fn automation(ui: &mut Ui, app: &mut RSpiceApp) {
    let language = ProjectSourceLanguage::RSpiceAutomation;
    let rail = Rail {
        id_salt: "workbench.automation.navigator",
        files_title: MessageId::CodeNavigatorWorkflowFiles,
        files: bundle_rows(&app.state, language),
        active: active_path(&app.state, language),
        language,
        actions: None,
        used_in: None,
    };
    show_rail(ui, app, rail);
}

/// Everything one rail lists, gathered before it paints.
struct Rail {
    id_salt: &'static str,
    files_title: MessageId,
    files: Vec<BundleRow>,
    active: Option<String>,
    language: ProjectSourceLanguage,
    actions: Option<FileActions>,
    /// The cell's placed instances, for a rail whose bundle is a cell view's.
    used_in: Option<Vec<CellInstanceUse>>,
}

/// New, rename and delete for a bundle's files, offered by the rail that
/// lists them.
///
/// Each opens the source lifecycle dialog at its review, the same transaction
/// Manage Source Document offers: rewriting a bundle's file graph is reviewed
/// and committed, never applied by a single click in a list. The controls are
/// neutral; danger is the confirmation's to state.
struct FileActions {
    bundle_id: ProjectSourceId,
    root: String,
    writable: bool,
}

impl FileActions {
    /// Why `action` cannot apply to `path`, when it cannot. The root is the
    /// closure's entry and the service refuses to delete it, so the control
    /// says so before the click rather than after it.
    fn refusal(
        &self,
        action: CodeSourceFileAction,
        path: &str,
        messages: MessageCatalog,
    ) -> Option<String> {
        (action == CodeSourceFileAction::Delete
            && crate::state::project_source_paths_equal(path, &self.root))
        .then(|| messages.text(MessageId::CodeSourceRootDeleteBlocked))
    }
}

const FILE_ACTIONS: [(MessageId, CodeSourceFileAction, WorkbenchIcon); 3] = [
    (
        MessageId::CodeSourceNewFile,
        CodeSourceFileAction::New,
        WorkbenchIcon::Add,
    ),
    (
        MessageId::CodeSourceRename,
        CodeSourceFileAction::Rename,
        WorkbenchIcon::Text,
    ),
    (
        MessageId::CodeSourceDelete,
        CodeSourceFileAction::Delete,
        WorkbenchIcon::Trash,
    ),
];

/// The file operations as icons at the right of a section header that spans
/// `header`, acting on the file in front of the reader.
fn header_file_actions(
    ui: &mut Ui,
    header: egui::Rect,
    actions: &FileActions,
    active: &str,
    messages: MessageCatalog,
) -> Option<CodeSourceFileAction> {
    let side = (header.height() - 4.0).max(1.0);
    let mut strip = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(header.shrink2(Vec2::new(6.0, 0.0)))
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
    );
    strip.spacing_mut().item_spacing.x = 2.0;
    let mut requested = None;
    for (label, action, icon) in FILE_ACTIONS.into_iter().rev() {
        let label = messages.text(label);
        let refusal = actions.refusal(action, active, messages);
        let response = strip
            .add_enabled_ui(actions.writable && refusal.is_none(), |ui| {
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
            requested = Some(action);
        }
    }
    requested
}

/// Rename and delete for one row, from its context menu.
fn row_file_actions(
    response: &Response,
    actions: &FileActions,
    path: &str,
    messages: MessageCatalog,
) -> Option<CodeSourceFileAction> {
    let mut requested = None;
    let (popup, _) = row_context_menu(response);
    popup.show(|ui| {
        for (label, action, _) in FILE_ACTIONS.into_iter().skip(1) {
            let refusal = actions.refusal(action, path, messages);
            let item = ui.add_enabled(
                actions.writable && refusal.is_none(),
                egui::Button::new(messages.text(label)),
            );
            let item = match refusal {
                Some(reason) => item.on_disabled_hover_text(reason),
                None => item,
            };
            if item.clicked() {
                requested = Some(action);
                ui.close();
            }
        }
    });
    requested
}

/// Open the lifecycle dialog at `action`'s review for `path`.
fn open_file_action(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
    bundle_id: ProjectSourceId,
    action: CodeSourceFileAction,
    path: &str,
) {
    let opened = code_workspace::open_source_file_dialog_in_bundle(
        app, language, bundle_id, action, path, path,
    )
    .and_then(|()| crate::workbench::app::open_source_document_dialog(&mut app.state));
    if let Err(error) = opened {
        app.state.ui.code_workspace.source_file_dialog = None;
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
    }
}

/// One selectable file in a project-owned source bundle.
struct BundleRow {
    logical_path: String,
    label: String,
    role: Option<ProjectSourceRole>,
}

fn bundle_rows(state: &AppState, language: ProjectSourceLanguage) -> Vec<BundleRow> {
    let Some(bundle) = active_bundle(state, language) else {
        return Vec::new();
    };
    let root = bundle.root();
    std::iter::once((
        root.logical_path(),
        bundle.role_for_path(root.logical_path()),
    ))
    .chain(bundle.files().iter().map(|file| {
        (
            file.logical_path(),
            bundle.role_for_path(file.logical_path()),
        )
    }))
    .map(|(logical_path, role)| BundleRow {
        label: leaf_name(logical_path),
        logical_path: logical_path.to_owned(),
        role,
    })
    .collect()
}

fn leaf_name(logical_path: &str) -> String {
    logical_path
        .rsplit('/')
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or(logical_path)
        .to_owned()
}

/// What the bundle records this file as, when that is something other than
/// ordinary source.
///
/// `ProjectSourceRole::label` prefixes the language, which is noise in a rail
/// already scoped to it, so the rail states the short form. The match is
/// exhaustive on purpose: a new role has to be given a name here rather than
/// silently reading as ordinary source.
const fn role_message(role: Option<ProjectSourceRole>) -> Option<MessageId> {
    match role {
        Some(ProjectSourceRole::VerilogABuildProfile) => {
            Some(MessageId::CodeNavigatorRoleBuildProfile)
        }
        Some(ProjectSourceRole::AutomationEntry) => Some(MessageId::CodeNavigatorRoleEntry),
        Some(ProjectSourceRole::AutomationRunPlan) => Some(MessageId::CodeNavigatorRoleRunPlan),
        Some(ProjectSourceRole::AutomationEnvironmentLock) => {
            Some(MessageId::CodeNavigatorRoleEnvironmentLock)
        }
        Some(ProjectSourceRole::AutomationPermissionManifest) => {
            Some(MessageId::CodeNavigatorRolePermissions)
        }
        None => None,
    }
}

/// The bundle this rail lists: the one every editor host and command resolves,
/// so the rail can never offer the files of a bundle the editor is not showing.
fn active_bundle(
    state: &AppState,
    language: ProjectSourceLanguage,
) -> Option<&crate::state::ProjectSourceBundle> {
    code_workspace::active_source_bundle(&state.workspace, language)
}

fn active_path(state: &AppState, language: ProjectSourceLanguage) -> Option<String> {
    code_workspace::active_bundle_path(&state.workspace, &state.ui.code_workspace, language)
}

fn select(state: &mut AppState, language: ProjectSourceLanguage, logical_path: &str) {
    code_workspace::select_source_document(
        &state.workspace,
        &mut state.ui.code_workspace,
        language,
        logical_path,
    );
}

/// What a click in the rail asked for, applied once the rows are painted.
enum RailRequest {
    Select(String),
    FileAction(CodeSourceFileAction, String),
    /// Edit `path` with its caret on `line`.
    Reveal {
        path: String,
        line: usize,
    },
    /// Read a built-in header in the editor well.
    ViewHeader(&'static str),
    OpenInstance(CellInstanceUse),
}

fn show_rail(ui: &mut Ui, app: &mut RSpiceApp, rail: Rail) {
    let messages = app.state.ui.messages();
    let language = rail.language;
    let bundle = code_workspace::active_source_bundle(&app.state.workspace, language);
    let bundle_id = bundle.map(crate::state::ProjectSourceBundle::id);
    let navigation = match (rail.active.as_deref(), bundle) {
        (Some(path), Some(bundle)) => document_navigation(
            app.state.workspace.content.project.id(),
            bundle,
            &mut app.state.ui.code_workspace.source_index_cache,
            path,
        ),
        _ => SourceNavigation::default(),
    };
    let header = bundle_id
        .zip(rail.active.as_deref())
        .and_then(|(bundle_id, path)| {
            code_workspace::viewed_veriloga_header(
                &app.state.ui.code_workspace.veriloga,
                bundle_id,
                path,
            )
        })
        .map(|(identity, _)| identity);
    let mut request = None;

    ScrollArea::vertical()
        .id_salt(rail.id_salt)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            if rail.files.is_empty() {
                muted(ui, &messages.text(MessageId::CodeNavigatorNoBundle));
                return;
            }
            files_section(ui, &rail, header, messages, &mut request);
            let Some(active) = rail.active.as_deref() else {
                return;
            };
            if language == ProjectSourceLanguage::VerilogA {
                includes_section(ui, &navigation, header, messages, &mut request);
            }
            outline_section(ui, &navigation, active, messages, &mut request);
            if let Some(used_in) = &rail.used_in {
                used_in_section(ui, used_in, messages, &mut request);
            }
        });

    match request {
        None => {}
        Some(RailRequest::Select(path)) => select(&mut app.state, language, &path),
        Some(RailRequest::FileAction(action, path)) => {
            if let Some(file_actions) = rail.actions.as_ref() {
                open_file_action(app, language, file_actions.bundle_id, action, &path);
            }
        }
        Some(RailRequest::Reveal { path, line }) => {
            select(&mut app.state, language, &path);
            if let Some(bundle_id) = bundle_id {
                // The shared code editor consumes this on its next frame,
                // which is the frame it shows `path` in.
                crate::workbench::documents::text_editor_commands::queue_reveal_line(
                    ui.ctx(),
                    code_workspace::source_editor_id(language, bundle_id, &path),
                    line,
                );
            }
        }
        Some(RailRequest::ViewHeader(identity)) => {
            if let (Some(bundle_id), Some(over)) = (bundle_id, rail.active) {
                app.state.ui.code_workspace.veriloga.viewed_header = Some(VerilogAHeaderView {
                    bundle_id,
                    over,
                    identity,
                });
            }
        }
        Some(RailRequest::OpenInstance(usage)) => open_instance(&mut app.state, &usage),
    }
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
    section_header(
        ui,
        &messages.text(rail.files_title),
        rail.actions
            .is_none()
            .then(|| rail.files.len().to_string())
            .as_deref(),
    );
    if let (Some(file_actions), Some(path)) = (rail.actions.as_ref(), rail.active.as_deref()) {
        let band = egui::Rect::from_x_y_ranges(ui.max_rect().x_range(), top..=ui.cursor().top());
        if let Some(action) = header_file_actions(ui, band, file_actions, path, messages) {
            *request = Some(RailRequest::FileAction(action, path.to_owned()));
        }
    }
    for row in &rail.files {
        // A header open in the editor is what the reader is looking at; no
        // file row claims the editor then.
        let is_active =
            header.is_none() && rail.active.as_deref() == Some(row.logical_path.as_str());
        let role = role_message(row.role).map(|role| messages.text(role));
        let response = nav_row_response(
            ui,
            source_row_icon(row.role),
            &row.label,
            is_active,
            role.as_deref(),
        );
        if response.clicked() {
            *request = Some(RailRequest::Select(row.logical_path.clone()));
        }
        if let Some(file_actions) = rail.actions.as_ref()
            && let Some(action) =
                row_file_actions(&response, file_actions, &row.logical_path, messages)
        {
            *request = Some(RailRequest::FileAction(action, row.logical_path.clone()));
        }
    }
}

/// Every include the file pulls in and where it resolved. A project file
/// opens for editing, a built-in header opens read-only in the same well, and
/// a name nothing holds is marked and takes the reader to its directive.
fn includes_section(
    ui: &mut Ui,
    navigation: &SourceNavigation,
    header: Option<&'static str>,
    messages: MessageCatalog,
    request: &mut Option<RailRequest>,
) {
    section_header(
        ui,
        &messages.text(MessageId::CodeNavigatorIncludes),
        Some(&navigation.includes.len().to_string()),
    );
    for include in &navigation.includes {
        match &include.target {
            SourceIncludeTarget::Project(path) => {
                let response =
                    nav_row_response(ui, source_row_icon(None), &leaf_name(path), false, None)
                        .on_hover_text(path);
                if response.clicked() {
                    *request = Some(RailRequest::Select(path.clone()));
                }
            }
            SourceIncludeTarget::BuiltIn(identity) => {
                let response = nav_row_response(
                    ui,
                    source_row_icon(None),
                    &leaf_name(identity),
                    header == Some(*identity),
                    Some(&messages.text(MessageId::CodeNavigatorIncludeBuiltIn)),
                )
                .on_hover_text(*identity);
                if response.clicked() {
                    *request = Some(RailRequest::ViewHeader(identity));
                }
            }
            SourceIncludeTarget::Missing { including, line } => {
                let response =
                    nav_row_response(ui, source_row_icon(None), &include.requested, false, None);
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
                let response =
                    response.on_hover_text(messages.text(MessageId::CodeNavigatorIncludeNotFound));
                if response.clicked() {
                    *request = Some(RailRequest::Reveal {
                        path: including.clone(),
                        line: *line,
                    });
                }
            }
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
        &messages.text(MessageId::CodeNavigatorOutline),
        Some(&navigation.outline.len().to_string()),
    );
    let mut group = None;
    for row in &navigation.outline {
        if row.group != group {
            if let Some(next) = row.group {
                group_caption(ui, &messages.text(outline_group_title(next)));
            }
            group = row.group;
        }
        let clicked = match row.group {
            Some(kind) => {
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
                    outline_group_icon(kind),
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
            }
            // An Automation outline is a list of the lines that declare.
            None => nav_row_response(
                ui,
                WorkbenchIcon::Code,
                &if row.detail.is_empty() {
                    row.name.clone()
                } else {
                    format!("{} {}", row.name, row.detail)
                },
                false,
                None,
            )
            .clicked(),
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
        SourceOutlineGroup::Module => MessageId::CodeNavigatorOutlineModule,
        SourceOutlineGroup::Ports => MessageId::CodeNavigatorOutlinePorts,
        SourceOutlineGroup::Parameters => MessageId::CodeNavigatorOutlineParameters,
        SourceOutlineGroup::Variables => MessageId::CodeNavigatorOutlineVariables,
        SourceOutlineGroup::Nets => MessageId::CodeNavigatorOutlineNets,
        SourceOutlineGroup::Functions => MessageId::CodeNavigatorOutlineFunctions,
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
        &messages.text(MessageId::CodeNavigatorUsedIn),
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

/// Open the schematic `usage` is placed in, with the instance selected and
/// centred.
fn open_instance(state: &mut AppState, usage: &CellInstanceUse) {
    state.open_workspace_view(usage.schematic.clone());
    let position = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == usage.component_id)
        .map(|component| component.pos);
    let editor = &mut state.schematic.session.editor;
    editor.selection.select_only_component(usage.component_id);
    editor.net_highlight.clear();
    editor.center_request = position;
}

fn source_row_icon(role: Option<ProjectSourceRole>) -> WorkbenchIcon {
    match role {
        // Governed policy rather than authored logic: build settings, the
        // pinned environment, and the permission manifest.
        Some(
            ProjectSourceRole::VerilogABuildProfile
            | ProjectSourceRole::AutomationEnvironmentLock
            | ProjectSourceRole::AutomationPermissionManifest,
        ) => WorkbenchIcon::Sliders,
        Some(ProjectSourceRole::AutomationRunPlan) => WorkbenchIcon::File,
        Some(ProjectSourceRole::AutomationEntry) | None => WorkbenchIcon::Code,
    }
}

#[cfg(test)]
mod tests;
