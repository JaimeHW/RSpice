//! The Verilog-A and Automation source navigators.
//!
//! Both show one file at a time out of a project-owned bundle, so the rail's
//! first job is choosing which. Before this existed the selection had no UI
//! writer at all: a bundle's build profile, run plan, environment lock, and
//! permission manifest were authored by the workspace and then unreachable,
//! because only the bundle root was ever opened.
//!
//! Rows state the role the bundle records, never a guess from the filename.
//! A starter name is not identity — `characterize.py` is the Python entry
//! because the bundle says so, not because of its extension.

use super::*;

use crate::state::{ProjectSourceId, ProjectSourceLanguage, ProjectSourceRole};
use crate::workbench::documents::code_workspace::{
    self, CodeSourceFileAction, SourceOutlineRow, VerilogAFileSelection, document_outline,
};
use crate::workbench::{MessageCatalog, MessageId};

/// The rail of a Verilog-A cell view's bundle, with its file operations.
pub(super) fn veriloga(ui: &mut Ui, app: &mut RSpiceApp) {
    let language = ProjectSourceLanguage::VerilogA;
    let active = active_path(app, language);
    let files = bundle_rows(app, language);
    // A Verilog-A bundle is module source plus the policy that builds it, and
    // the two read differently enough to earn separate sections.
    let (build, sources): (Vec<_>, Vec<_>) = files
        .into_iter()
        .partition(|row| row.role == Some(ProjectSourceRole::VerilogABuildProfile));
    let actions = active_bundle(app, language).map(|bundle| FileActions {
        bundle_id: bundle.id(),
        root: bundle.root().logical_path().to_owned(),
        writable: !app.state.active_view_read_only()
            && code_workspace::source_file_mutation_block_reason(app, language).is_none(),
    });

    show_rail(
        ui,
        app,
        "workbench.veriloga.navigator",
        &[
            (MessageId::CodeNavigatorProjectSources, sources),
            (MessageId::CodeNavigatorIncludesAndBuild, build),
        ],
        active,
        language,
        actions,
    );
}

pub(super) fn automation(ui: &mut Ui, app: &mut RSpiceApp) {
    let language = ProjectSourceLanguage::RSpiceAutomation;
    let active = active_path(app, language);
    let files = bundle_rows(app, language);
    show_rail(
        ui,
        app,
        "workbench.automation.navigator",
        &[(MessageId::CodeNavigatorWorkflowFiles, files)],
        active,
        language,
        None,
    );
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

fn bundle_rows(app: &RSpiceApp, language: ProjectSourceLanguage) -> Vec<BundleRow> {
    let Some(bundle) = active_bundle(app, language) else {
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

/// What the bundle records this file as.
///
/// `ProjectSourceRole::label` prefixes the language, which is noise in a rail
/// already scoped to it, so the rail states the short form. The match is
/// exhaustive on purpose: a new role has to be given a name here rather than
/// silently reading as ordinary source.
const fn role_message(role: Option<ProjectSourceRole>) -> MessageId {
    match role {
        Some(ProjectSourceRole::VerilogABuildProfile) => MessageId::CodeNavigatorRoleBuildProfile,
        Some(ProjectSourceRole::AutomationEntry) => MessageId::CodeNavigatorRoleEntry,
        Some(ProjectSourceRole::AutomationRunPlan) => MessageId::CodeNavigatorRoleRunPlan,
        Some(ProjectSourceRole::AutomationEnvironmentLock) => {
            MessageId::CodeNavigatorRoleEnvironmentLock
        }
        Some(ProjectSourceRole::AutomationPermissionManifest) => {
            MessageId::CodeNavigatorRolePermissions
        }
        None => MessageId::CodeNavigatorRoleSource,
    }
}

/// The bundle this rail lists: the one every editor host and command resolves,
/// so the rail can never offer the files of a bundle the editor is not showing.
fn active_bundle(
    app: &RSpiceApp,
    language: ProjectSourceLanguage,
) -> Option<&crate::state::ProjectSourceBundle> {
    crate::workbench::documents::code_workspace::active_source_bundle(
        &app.state.workspace,
        language,
    )
}

fn active_path(app: &RSpiceApp, language: ProjectSourceLanguage) -> Option<String> {
    crate::workbench::documents::code_workspace::active_bundle_path(
        &app.state.workspace,
        &app.state.ui.code_workspace,
        language,
    )
}

fn select(app: &mut RSpiceApp, language: ProjectSourceLanguage, logical_path: &str) {
    let Some(bundle_id) = active_bundle(app, language).map(crate::state::ProjectSourceBundle::id)
    else {
        return;
    };
    match language {
        ProjectSourceLanguage::VerilogA => {
            app.state.ui.code_workspace.veriloga.selected_file = Some(VerilogAFileSelection {
                bundle_id,
                logical_path: logical_path.to_owned(),
            });
        }
        ProjectSourceLanguage::RSpiceAutomation => {
            app.state.ui.code_workspace.automation.selected_file = Some(logical_path.to_owned());
        }
    }
}

fn show_rail(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    id_salt: &str,
    sections: &[(MessageId, Vec<BundleRow>)],
    active: Option<String>,
    language: ProjectSourceLanguage,
    actions: Option<FileActions>,
) {
    let messages = app.state.ui.messages();
    let mut selected = None;
    let mut requested_line = None;
    let mut requested_action = None;
    let outline = match (
        active.as_deref(),
        crate::workbench::documents::code_workspace::active_source_bundle(
            &app.state.workspace,
            language,
        ),
    ) {
        (Some(path), Some(bundle)) => document_outline(
            app.state.workspace.content.project.id(),
            bundle,
            &mut app.state.ui.code_workspace.source_index_cache,
            path,
        ),
        _ => Vec::new(),
    };

    ScrollArea::vertical()
        .id_salt(id_salt)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let mut any_file = false;
            for (index, (title, rows)) in sections.iter().enumerate() {
                if rows.is_empty() {
                    continue;
                }
                any_file = true;
                // The file operations sit in the first section's header: the
                // sources are what they create, rename and delete.
                let header_actions = actions.as_ref().filter(|_| index == 0);
                let top = ui.cursor().top();
                section_header(
                    ui,
                    &messages.text(*title),
                    header_actions
                        .is_none()
                        .then(|| rows.len().to_string())
                        .as_deref(),
                );
                if let (Some(file_actions), Some(path)) = (header_actions, active.as_deref()) {
                    let header = egui::Rect::from_x_y_ranges(
                        ui.max_rect().x_range(),
                        top..=ui.cursor().top(),
                    );
                    if let Some(action) =
                        header_file_actions(ui, header, file_actions, path, messages)
                    {
                        requested_action = Some((action, path.to_owned()));
                    }
                }
                for row in rows {
                    let is_active = active.as_deref() == Some(row.logical_path.as_str());
                    let response = nav_row_response(
                        ui,
                        source_row_icon(row.role),
                        &row.label,
                        is_active,
                        Some(&messages.text(role_message(row.role))),
                    );
                    if response.clicked() {
                        selected = Some(row.logical_path.clone());
                    }
                    if let Some(file_actions) = actions.as_ref()
                        && let Some(action) =
                            row_file_actions(&response, file_actions, &row.logical_path, messages)
                    {
                        requested_action = Some((action, row.logical_path.clone()));
                    }
                }
            }
            if !any_file {
                muted(ui, &messages.text(MessageId::CodeNavigatorNoBundle));
                return;
            }

            let Some(active) = active.as_deref() else {
                return;
            };
            section_header(
                ui,
                &messages.format(
                    MessageId::CodeNavigatorFileOutline,
                    &[("document", &leaf_name(active))],
                ),
                Some(&outline.len().to_string()),
            );
            if outline.is_empty() {
                muted(ui, &messages.text(MessageId::CodeNavigatorNoIndexedSymbol));
                return;
            }
            for row in &outline {
                if outline_row(ui, row, messages) {
                    requested_line = Some(row.line);
                }
            }
        });

    if let Some(logical_path) = selected {
        select(app, language, &logical_path);
    }
    if let (Some((action, path)), Some(file_actions)) = (requested_action, actions.as_ref()) {
        open_file_action(app, language, file_actions.bundle_id, action, &path);
    }
    if let Some(line) = requested_line
        && let Some(active) = active.as_deref()
        && let Some(editor_id) = code_editor_id(app, language, active)
    {
        // The shared code editor already consumes this; it simply had no
        // producer, which is why no outline could scroll a document.
        crate::workbench::documents::text_editor_commands::queue_reveal_line(
            ui.ctx(),
            editor_id,
            line,
        );
    }
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

fn outline_row(ui: &mut Ui, row: &SourceOutlineRow, messages: MessageCatalog) -> bool {
    let label = if row.detail.is_empty() {
        row.name.clone()
    } else {
        format!("{} {}", row.name, row.detail)
    };
    nav_row_indented(
        ui,
        WorkbenchIcon::Code,
        &label,
        false,
        Some(&messages.format(
            MessageId::CodeNavigatorOutlineLine,
            &[("line", &row.line.to_string())],
        )),
        1,
    )
}

fn code_editor_id(
    app: &RSpiceApp,
    language: ProjectSourceLanguage,
    active_path: &str,
) -> Option<egui::Id> {
    let bundle_id = active_bundle(app, language).map(crate::state::ProjectSourceBundle::id)?;
    Some(
        crate::workbench::documents::code_workspace::source_editor_id(
            language,
            bundle_id,
            active_path,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The defect this rail exists for: `selected_file` had no UI writer, so
    /// every file in a bundle except the root was unreachable. Assert the
    /// bundle's whole file set is offered, and that choosing one is what
    /// changes the selection.
    #[test]
    fn every_file_in_the_bundle_is_offered_and_selectable() {
        let mut app = RSpiceApp::test_instance();
        let language = ProjectSourceLanguage::RSpiceAutomation;
        let rows = bundle_rows(&app, language);
        assert!(
            rows.len() > 1,
            "an automation bundle carries an entry, a run plan, a lock, and a manifest"
        );

        let root = active_path(&app, language).expect("bundle root is the default document");
        let other = rows
            .iter()
            .find(|row| row.logical_path != root)
            .expect("a non-root file to select");
        let target = other.logical_path.clone();

        select(&mut app, language, &target);

        assert_eq!(
            active_path(&app, language).as_deref(),
            Some(target.as_str())
        );
    }

    /// A row states the role the bundle recorded. Nothing here is inferred
    /// from the filename.
    #[test]
    fn rows_state_the_recorded_role_rather_than_guessing_from_the_name() {
        let app = RSpiceApp::test_instance();
        let messages = MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates);
        let rows = bundle_rows(&app, ProjectSourceLanguage::RSpiceAutomation);
        let roles = rows
            .iter()
            .map(|row| messages.text(role_message(row.role)))
            .collect::<std::collections::BTreeSet<_>>();

        assert!(roles.contains("run plan"), "roles were {roles:?}");
        assert!(roles.contains("permissions"), "roles were {roles:?}");
    }
}
