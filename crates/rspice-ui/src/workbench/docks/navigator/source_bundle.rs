//! The Verilog-A and Automation source navigators.
//!
//! Both pages show one file at a time out of a project-owned bundle, so the
//! rail's first job is choosing which. Before this existed the selection had
//! no UI writer at all: a bundle's build profile, run plan, environment lock,
//! and permission manifest were authored by the workspace and then
//! unreachable, because only the bundle root was ever opened.
//!
//! Rows state the role the bundle records, never a guess from the filename.
//! A starter name is not identity — `characterize.py` is the Python entry
//! because the bundle says so, not because of its extension.

use super::*;

use crate::state::{ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceRole};
use crate::workbench::documents::code_workspace::{
    SourceOutlineRow, VerilogAFileSelection, document_outline,
};

pub(super) fn veriloga(ui: &mut Ui, app: &mut RSpiceApp) {
    let language = ProjectSourceLanguage::VerilogA;
    let active = active_path(app, language);
    let files = bundle_rows(app, language);
    // A Verilog-A bundle is module source plus the policy that builds it, and
    // the two read differently enough to earn separate sections.
    let (build, sources): (Vec<_>, Vec<_>) = files
        .into_iter()
        .partition(|row| row.role == Some(ProjectSourceRole::VerilogABuildProfile));

    show_rail(
        ui,
        app,
        "workbench.veriloga.navigator",
        &[("Project sources", sources), ("Includes & build", build)],
        active,
        language,
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
        &[("Workflow files", files)],
        active,
        language,
    );
}

/// One selectable file in a project-owned source bundle.
struct BundleRow {
    logical_path: String,
    label: String,
    role: Option<ProjectSourceRole>,
}

fn bundle_rows(app: &RSpiceApp, language: ProjectSourceLanguage) -> Vec<BundleRow> {
    let owner = ProjectSourceOwner::code_workspace(language);
    let Some(bundle) = app.state.workspace.project_sources.bundle_for_owner(&owner) else {
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
fn role_label(role: Option<ProjectSourceRole>) -> &'static str {
    match role {
        Some(ProjectSourceRole::VerilogABuildProfile) => "build profile",
        Some(ProjectSourceRole::AutomationEntry) => "entry",
        Some(ProjectSourceRole::AutomationRunPlan) => "run plan",
        Some(ProjectSourceRole::AutomationEnvironmentLock) => "environment lock",
        Some(ProjectSourceRole::AutomationPermissionManifest) => "permissions",
        None => "source",
    }
}

fn active_path(app: &RSpiceApp, language: ProjectSourceLanguage) -> Option<String> {
    let explicit = match language {
        ProjectSourceLanguage::VerilogA => app
            .state
            .ui
            .code_workspace
            .veriloga
            .selected_file
            .as_ref()
            .map(|selection| selection.logical_path.clone()),
        ProjectSourceLanguage::RSpiceAutomation => {
            app.state.ui.code_workspace.automation.selected_file.clone()
        }
    };
    explicit.or_else(|| {
        let owner = ProjectSourceOwner::code_workspace(language);
        app.state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .map(|bundle| bundle.root().logical_path().to_owned())
    })
}

fn select(app: &mut RSpiceApp, language: ProjectSourceLanguage, logical_path: &str) {
    let owner = ProjectSourceOwner::code_workspace(language);
    let Some(bundle_id) = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .map(crate::state::ProjectSourceBundle::id)
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
    sections: &[(&str, Vec<BundleRow>)],
    active: Option<String>,
    language: ProjectSourceLanguage,
) {
    let mut selected = None;
    let mut requested_line = None;
    let outline = active
        .as_deref()
        .map(|path| {
            document_outline(
                app.state.workspace.project.id(),
                &app.state.workspace.project_sources,
                &mut app.state.ui.code_workspace.source_index_cache,
                language,
                path,
            )
        })
        .unwrap_or_default();

    ScrollArea::vertical()
        .id_salt(id_salt)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let mut any_file = false;
            for (title, rows) in sections {
                if rows.is_empty() {
                    continue;
                }
                any_file = true;
                section_header(ui, title, Some(&rows.len().to_string()));
                for row in rows {
                    let is_active = active.as_deref() == Some(row.logical_path.as_str());
                    if nav_row(
                        ui,
                        source_row_icon(row.role),
                        &row.label,
                        is_active,
                        Some(role_label(row.role)),
                    ) {
                        selected = Some(row.logical_path.clone());
                    }
                }
            }
            if !any_file {
                muted(
                    ui,
                    "This project owns no source bundle for this page yet. Create or import one from the document well.",
                );
                return;
            }

            let Some(active) = active.as_deref() else {
                return;
            };
            section_header(
                ui,
                &format!("Outline · {}", leaf_name(active)),
                Some(&outline.len().to_string()),
            );
            if outline.is_empty() {
                muted(ui, "No indexed symbol is declared in this document.");
                return;
            }
            for row in &outline {
                if outline_row(ui, row) {
                    requested_line = Some(row.line);
                }
            }
        });

    if let Some(logical_path) = selected {
        select(app, language, &logical_path);
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

fn outline_row(ui: &mut Ui, row: &SourceOutlineRow) -> bool {
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
        Some(&format!("line {}", row.line)),
        1,
    )
}

/// The id the visible page's document well gives its editor.
///
/// It has to be reconstructed exactly, including the bundle identity the
/// Verilog-A well mixes in, or the reveal request lands on an editor that is
/// not on screen and the row silently does nothing.
fn code_editor_id(
    app: &RSpiceApp,
    language: ProjectSourceLanguage,
    active_path: &str,
) -> Option<egui::Id> {
    match language {
        ProjectSourceLanguage::VerilogA => {
            let owner = ProjectSourceOwner::code_workspace(language);
            let bundle_id = app
                .state
                .workspace
                .project_sources
                .bundle_for_owner(&owner)
                .map(crate::state::ProjectSourceBundle::id)?;
            Some(egui::Id::new((
                "workbench.veriloga.source",
                bundle_id,
                active_path,
            )))
        }
        ProjectSourceLanguage::RSpiceAutomation => {
            Some(egui::Id::new(("workbench.automation.source", active_path)))
        }
    }
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

        assert_eq!(active_path(&app, language).as_deref(), Some(target.as_str()));
    }

    /// A row states the role the bundle recorded. Nothing here is inferred
    /// from the filename.
    #[test]
    fn rows_state_the_recorded_role_rather_than_guessing_from_the_name() {
        let app = RSpiceApp::test_instance();
        let rows = bundle_rows(&app, ProjectSourceLanguage::RSpiceAutomation);
        let roles = rows
            .iter()
            .map(|row| role_label(row.role))
            .collect::<std::collections::BTreeSet<_>>();

        assert!(roles.contains("run plan"), "roles were {roles:?}");
        assert!(roles.contains("permissions"), "roles were {roles:?}");
    }
}
