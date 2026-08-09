//! Exact, transactional source-file lifecycle for the Code Workspace tree.

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    ProjectSourceBundle, ProjectSourceDependency, ProjectSourceFile, ProjectSourceLanguage,
    ProjectSourceOwner, ProjectSourceRole, ProjectSourceRoleBinding,
};
use crate::workbench::RSpiceApp;
use crate::workbench::workflows::export_workflow::SaveDialogConfig;

use super::{
    CodeSourceEditorBufferIdentity, CodeSourceFileAction, CodeSourceFileDialogState,
    CodeSourceHistoryState, CodeSourceImportState, VerilogAFileSelection,
};

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct BrowserAutomationImportCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    result: Result<Option<crate::workbench::browser::file_import::PickedTextFile>, String>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_AUTOMATION_IMPORT_COMPLETION: std::cell::RefCell<Option<BrowserAutomationImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

pub(crate) fn source_file_mutation_block_reason(
    app: &RSpiceApp,
    language: ProjectSourceLanguage,
) -> Option<&'static str> {
    if !app.state.project_lifecycle.project_open {
        return Some("Open a project before changing project source files.");
    }
    if app.state.workbench.safe_mode.project_read_only() {
        return Some("The project is open read-only.");
    }
    match language {
        ProjectSourceLanguage::VerilogA
            if app.state.ui.code_workspace.veriloga.pending.is_some() =>
        {
            Some("Wait for the exact Verilog-A compile to finish or cancel it first.")
        }
        ProjectSourceLanguage::RSpiceAutomation => {
            let runtime = &app.state.ui.code_workspace.automation;
            (runtime.pending_runtime_validation.is_some()
                || runtime.runtime_execution.is_some()
                || runtime.execution.is_active())
            .then_some("Stop the managed Automation validation or execution before changing files.")
        }
        _ => None,
    }
}

pub(crate) fn open_source_workspace_dialog(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
) -> Result<(), String> {
    if let Some(reason) = source_file_mutation_block_reason(app, language) {
        return Err(reason.to_owned());
    }
    let owner = ProjectSourceOwner::code_workspace(language);
    if app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .is_some()
    {
        return Err(format!(
            "This project already owns a {} source workspace.",
            language.label()
        ));
    }
    let netlist_source_path = app
        .state
        .workspace
        .netlist_descriptor
        .as_ref()
        .map(|descriptor| descriptor.artifact_name.clone())
        .unwrap_or_else(|| "top.sp".to_owned());
    app.state.ui.code_workspace.source_workspace_dialog =
        Some(super::CodeSourceWorkspaceDialogState {
            project_id: app.state.workspace.project.id(),
            language,
            root_path: match language {
                ProjectSourceLanguage::VerilogA => "model.va",
                ProjectSourceLanguage::RSpiceAutomation => "automation.py",
            }
            .to_owned(),
            build_profile_path: "veriloga-build.toml".to_owned(),
            run_plan_path: "runplan.yaml".to_owned(),
            environment_lock_path: "environment.lock".to_owned(),
            permission_manifest_path: "permissions.toml".to_owned(),
            netlist_source_path,
            error: None,
        });
    Ok(())
}

pub(crate) fn commit_source_workspace_dialog(app: &mut RSpiceApp) -> Result<String, String> {
    let draft = app
        .state
        .ui
        .code_workspace
        .source_workspace_dialog
        .clone()
        .ok_or_else(|| "No source-workspace transaction is open.".to_owned())?;
    if let Some(reason) = source_file_mutation_block_reason(app, draft.language) {
        return Err(reason.to_owned());
    }
    if draft.project_id != app.state.workspace.project.id() {
        return Err(
            "The active project changed while the source-workspace transaction was open."
                .to_owned(),
        );
    }
    let owner = ProjectSourceOwner::code_workspace(draft.language);
    if app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .is_some()
    {
        return Err("The source workspace was created by another operation. Review the current project source tree."
            .to_owned());
    }

    let bundle = match draft.language {
        ProjectSourceLanguage::VerilogA => {
            new_veriloga_workspace_bundle(&draft.root_path, &draft.build_profile_path)?
        }
        ProjectSourceLanguage::RSpiceAutomation => new_automation_workspace_bundle(&draft)?,
    };
    let root_path = bundle.root().logical_path().to_owned();
    let bundle_id = bundle.id();
    app.state
        .workspace
        .insert_project_source_bundle(bundle)
        .map_err(|error| error.to_string())?;
    match draft.language {
        ProjectSourceLanguage::VerilogA => {
            app.state.ui.code_workspace.veriloga = Default::default();
            app.state.ui.code_workspace.veriloga.selected_file = Some(VerilogAFileSelection {
                bundle_id,
                logical_path: root_path.clone(),
            });
        }
        ProjectSourceLanguage::RSpiceAutomation => {
            app.state.ui.code_workspace.automation = Default::default();
            app.state.ui.code_workspace.automation.selected_file = Some(root_path.clone());
        }
    }
    app.state.ui.code_workspace.source_workspace_dialog = None;
    let message = format!(
        "Created a project-owned {} source workspace rooted at '{root_path}'.",
        draft.language.label()
    );
    app.state
        .push_user_message(ConsoleMessage::info(message.clone()));
    Ok(message)
}

fn new_veriloga_workspace_bundle(
    root_path: &str,
    build_profile_path: &str,
) -> Result<ProjectSourceBundle, String> {
    let root_path = root_path.trim();
    let build_profile_path = build_profile_path.trim();
    if crate::state::project_source_paths_equal(root_path, build_profile_path) {
        return Err(
            "The root source and Verilog-A build profile must use distinct project paths."
                .to_owned(),
        );
    }
    let module = verilog_a_module_name(root_path);
    let source = format!(
        "`include \"constants.vams\"\n`include \"disciplines.vams\"\n\nmodule {module}(p, n);\n    inout p, n;\n    electrical p, n;\n\n    analog begin\n    end\nendmodule\n"
    );
    new_veriloga_workspace_bundle_with_source(root_path, source, build_profile_path, &module)
}

pub(super) fn new_imported_veriloga_workspace_bundle(
    root_path: &str,
    source: String,
) -> Result<ProjectSourceBundle, String> {
    let module = verilog_a_module_name(root_path);
    let mut profile = super::veriloga_profile::VerilogABuildProfile::starter(&module);
    // Imported source is arbitrary project content. The file stem is useful
    // as a package label, but it is not evidence of the declared module. An
    // empty entry list lets the compiler select exactly one declared module
    // and requires an explicit project choice for multi-module sources.
    profile.entry_modules.clear();
    let profile = profile.to_toml().map_err(|error| error.to_string())?;
    ProjectSourceBundle::try_new_with_roles(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        root_path,
        source,
        [
            ProjectSourceFile::try_new(".rspice/veriloga-build.toml", profile)
                .map_err(|error| error.to_string())?,
        ],
        [
            ProjectSourceDependency::try_new(root_path, ".rspice/veriloga-build.toml")
                .map_err(|error| error.to_string())?,
        ],
        [ProjectSourceRoleBinding::try_new(
            ".rspice/veriloga-build.toml",
            ProjectSourceRole::VerilogABuildProfile,
        )
        .map_err(|error| error.to_string())?],
    )
    .map_err(|error| error.to_string())
}

fn new_veriloga_workspace_bundle_with_source(
    root_path: &str,
    source: String,
    build_profile_path: &str,
    module: &str,
) -> Result<ProjectSourceBundle, String> {
    let profile = super::veriloga_profile::VerilogABuildProfile::starter(&module)
        .to_toml()
        .map_err(|error| error.to_string())?;
    ProjectSourceBundle::try_new_with_roles(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        root_path,
        source,
        [ProjectSourceFile::try_new(build_profile_path, profile)
            .map_err(|error| error.to_string())?],
        [
            ProjectSourceDependency::try_new(root_path, build_profile_path)
                .map_err(|error| error.to_string())?,
        ],
        [ProjectSourceRoleBinding::try_new(
            build_profile_path,
            ProjectSourceRole::VerilogABuildProfile,
        )
        .map_err(|error| error.to_string())?],
    )
    .map_err(|error| error.to_string())
}

fn verilog_a_module_name(root_path: &str) -> String {
    let stem = root_path
        .rsplit('/')
        .next()
        .unwrap_or(root_path)
        .split('.')
        .next()
        .unwrap_or_default();
    let mut module = stem
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    if module.is_empty() || module.starts_with(|character: char| character.is_ascii_digit()) {
        module.insert_str(0, "model_");
    }
    module
}

pub(super) fn verilog_a_module_name_for_profile(root_path: &str) -> String {
    verilog_a_module_name(root_path)
}

fn new_automation_workspace_bundle(
    draft: &super::CodeSourceWorkspaceDialogState,
) -> Result<ProjectSourceBundle, String> {
    let paths = [
        draft.root_path.trim(),
        draft.run_plan_path.trim(),
        draft.environment_lock_path.trim(),
        draft.permission_manifest_path.trim(),
    ];
    let mut unique = std::collections::HashSet::new();
    if paths.iter().any(|path| !unique.insert(path.to_lowercase())) {
        return Err(
            "Every Automation role must be bound to a distinct project source path.".to_owned(),
        );
    }
    if !draft.root_path.trim().to_ascii_lowercase().ends_with(".py") {
        return Err("The Automation entry document must be a Python source file.".to_owned());
    }
    let netlist_source = draft.netlist_source_path.trim();
    if netlist_source.is_empty() {
        return Err(
            "Enter the portable project-relative netlist source used by the starter run plan."
                .to_owned(),
        );
    }
    let plan_literal = serde_json::to_string(draft.run_plan_path.trim())
        .map_err(|error| format!("Could not encode the run-plan path: {error}"))?;
    let entry = format!(
        "from rspice import Project\n\nproject = Project.open(\".\")\nplan = project.run_plans.load({plan_literal})\npreview = plan.validate(target=\"local\", fail_closed=True)\nrun = preview.execute()\n"
    );
    let plan = format!(
        "schema: rspice.run-plan/v1\nname: New automation workflow\nsource: {netlist_source}\nanalyses: [op]\ntarget: local\n"
    );
    let lock = crate::automation_workflow::DEFAULT_ENVIRONMENT_LOCK;
    let permissions = "project_files = \"read\"\nartifact_directory = \"write\"\nnetwork = \"deny\"\nprocess_spawn = \"deny\"\nenvironment = []\nsecret_logging = \"redact\"\n";
    let documents = [
        crate::automation_workflow::AutomationSourceDocument {
            path: draft.root_path.trim(),
            source: &entry,
        },
        crate::automation_workflow::AutomationSourceDocument {
            path: draft.run_plan_path.trim(),
            source: &plan,
        },
        crate::automation_workflow::AutomationSourceDocument {
            path: draft.environment_lock_path.trim(),
            source: lock,
        },
        crate::automation_workflow::AutomationSourceDocument {
            path: draft.permission_manifest_path.trim(),
            source: permissions,
        },
    ];
    let compile_roles = [
        crate::automation_workflow::AutomationRoleBinding {
            path: draft.run_plan_path.trim(),
            role: crate::automation_workflow::AutomationSourceRole::RunPlan,
        },
        crate::automation_workflow::AutomationRoleBinding {
            path: draft.environment_lock_path.trim(),
            role: crate::automation_workflow::AutomationSourceRole::EnvironmentLock,
        },
        crate::automation_workflow::AutomationRoleBinding {
            path: draft.permission_manifest_path.trim(),
            role: crate::automation_workflow::AutomationSourceRole::PermissionManifest,
        },
    ];
    if let Err(diagnostics) = crate::automation_workflow::compile_automation_documents(
        draft.root_path.trim(),
        &documents,
        &compile_roles,
    ) {
        let Some(diagnostic) = diagnostics.first() else {
            return Err(
                "Automation source compilation failed without a structured diagnostic.".to_owned(),
            );
        };
        return Err(format!(
            "{}:{}:{} {}: {}",
            diagnostic.path,
            diagnostic.line,
            diagnostic.column,
            diagnostic.code,
            diagnostic.message
        ));
    }
    let files = [
        ProjectSourceFile::try_new(draft.run_plan_path.trim(), plan)
            .map_err(|error| error.to_string())?,
        ProjectSourceFile::try_new(draft.environment_lock_path.trim(), lock)
            .map_err(|error| error.to_string())?,
        ProjectSourceFile::try_new(draft.permission_manifest_path.trim(), permissions)
            .map_err(|error| error.to_string())?,
    ];
    let dependencies = [
        ProjectSourceDependency::try_new(draft.root_path.trim(), draft.run_plan_path.trim())
            .map_err(|error| error.to_string())?,
        ProjectSourceDependency::try_new(
            draft.root_path.trim(),
            draft.environment_lock_path.trim(),
        )
        .map_err(|error| error.to_string())?,
        ProjectSourceDependency::try_new(
            draft.root_path.trim(),
            draft.permission_manifest_path.trim(),
        )
        .map_err(|error| error.to_string())?,
    ];
    let roles = [
        ProjectSourceRoleBinding::try_new(
            draft.root_path.trim(),
            ProjectSourceRole::AutomationEntry,
        )
        .map_err(|error| error.to_string())?,
        ProjectSourceRoleBinding::try_new(
            draft.run_plan_path.trim(),
            ProjectSourceRole::AutomationRunPlan,
        )
        .map_err(|error| error.to_string())?,
        ProjectSourceRoleBinding::try_new(
            draft.environment_lock_path.trim(),
            ProjectSourceRole::AutomationEnvironmentLock,
        )
        .map_err(|error| error.to_string())?,
        ProjectSourceRoleBinding::try_new(
            draft.permission_manifest_path.trim(),
            ProjectSourceRole::AutomationPermissionManifest,
        )
        .map_err(|error| error.to_string())?,
    ];
    ProjectSourceBundle::try_new_with_roles(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation),
        ProjectSourceLanguage::RSpiceAutomation,
        draft.root_path.trim(),
        entry,
        files,
        dependencies,
        roles,
    )
    .map_err(|error| error.to_string())
}

pub(crate) fn assign_automation_source_role(
    app: &mut RSpiceApp,
    logical_path: &str,
    role: Option<ProjectSourceRole>,
) -> Result<String, String> {
    if let Some(reason) =
        source_file_mutation_block_reason(app, ProjectSourceLanguage::RSpiceAutomation)
    {
        return Err(reason.to_owned());
    }
    let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation);
    let bundle = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .ok_or_else(|| "The Automation source bundle no longer exists.".to_owned())?;
    let bundle_id = bundle.id();
    let changed = app
        .state
        .workspace
        .set_project_source_bundle_non_entry_role(bundle_id, logical_path, role)
        .map_err(|error| error.to_string())?;
    if changed {
        super::automation::invalidate_automation_evidence(app);
    }
    let role_label = role.map_or("unassigned", ProjectSourceRole::label);
    let message = if changed {
        format!("Set Automation source '{logical_path}' role to {role_label}.")
    } else {
        format!("Automation source '{logical_path}' already has the {role_label} role.")
    };
    app.state
        .push_user_message(ConsoleMessage::info(message.clone()));
    Ok(message)
}

/// Resolve edit authority from persisted document ownership and semantic
/// roles. File extensions and demonstration filenames are never authority.
pub(crate) fn source_document_is_editable(
    app: &RSpiceApp,
    language: ProjectSourceLanguage,
    logical_path: &str,
) -> bool {
    let Some(bundle_id) = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(language))
        .map(ProjectSourceBundle::id)
    else {
        return false;
    };
    source_bundle_document_is_editable(app, language, bundle_id, logical_path)
}

/// Resolve edit authority for an exact source bundle. Verilog-A bundles may
/// belong either to the singleton Code workspace or to a first-class cell
/// view; Automation remains confined to its singleton project workspace.
pub(crate) fn source_bundle_document_is_editable(
    app: &RSpiceApp,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    logical_path: &str,
) -> bool {
    if source_file_mutation_block_reason(app, language).is_some() {
        return false;
    }
    source_bundle_contains_document(app, language, bundle_id, logical_path)
        && (language != ProjectSourceLanguage::RSpiceAutomation
            || app
                .state
                .workspace
                .project_sources
                .get_bundle(bundle_id)
                .and_then(|bundle| bundle.role_for_path(logical_path))
                != Some(crate::state::ProjectSourceRole::AutomationEnvironmentLock))
}

/// Whether an exact retained document belongs to an authenticated source
/// owner in the currently open project. Read-only policy is intentionally not
/// part of this predicate so find, navigation, and language inspection remain
/// available in safe mode.
pub(crate) fn source_bundle_contains_document(
    app: &RSpiceApp,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    logical_path: &str,
) -> bool {
    if !app.state.project_lifecycle.project_open {
        return false;
    }
    app.state
        .workspace
        .project_sources
        .get_bundle(bundle_id)
        .is_some_and(|bundle| {
            let owner_matches = match bundle.owner() {
                ProjectSourceOwner::CodeWorkspace {
                    language: owner_language,
                } => *owner_language == language,
                ProjectSourceOwner::CellView { .. } => language == ProjectSourceLanguage::VerilogA,
            };
            bundle.language() == language && owner_matches && bundle.contains_file(logical_path)
        })
}

/// Commit one Automation editor buffer only when the exact project, bundle,
/// revision, closure, path, and edit authority that produced it are still
/// current. Presentation code must not call the registry mutation API
/// directly because safe-mode and governed-role policy live above it.
pub(crate) fn replace_automation_editor_file(
    app: &mut RSpiceApp,
    expected: &CodeSourceEditorBufferIdentity,
    contents: String,
) -> Result<bool, String> {
    let language = ProjectSourceLanguage::RSpiceAutomation;
    if let Some(reason) = source_file_mutation_block_reason(app, language) {
        return Err(reason.to_owned());
    }
    if !source_bundle_document_is_editable(
        app,
        language,
        expected.bundle_id,
        expected.path.as_ref(),
    ) {
        return Err(
            "The selected Automation document is governed read-only; no source was changed."
                .to_owned(),
        );
    }
    if app.state.workspace.project.id() != expected.project_id {
        return Err("The active project changed before this editor update committed.".to_owned());
    }
    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(expected.bundle_id)
        .ok_or_else(|| "The Automation source bundle no longer exists.".to_owned())?;
    if bundle.language() != language
        || bundle.revision().get() != expected.bundle_revision
        || bundle.closure_digest() != expected.closure_digest
        || !bundle.contains_file(expected.path.as_ref())
    {
        return Err(
            "The Automation source changed before this editor update committed.".to_owned(),
        );
    }
    let changed = app
        .state
        .workspace
        .replace_project_source_bundle_file(expected.bundle_id, expected.path.as_ref(), contents)
        .map_err(|error| error.to_string())?;
    if changed {
        super::automation::invalidate_automation_evidence(app);
    }
    Ok(changed)
}

fn source_document_blocks_graph_mutation(
    bundle: &ProjectSourceBundle,
    language: ProjectSourceLanguage,
    logical_path: &str,
) -> bool {
    language == ProjectSourceLanguage::RSpiceAutomation
        && bundle.role_for_path(logical_path) == Some(ProjectSourceRole::AutomationEnvironmentLock)
}

#[cfg(test)]
pub(crate) fn open_source_file_dialog(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
    action: CodeSourceFileAction,
    source_path: &str,
    importer_path: &str,
) -> Result<(), String> {
    let owner = crate::state::ProjectSourceOwner::code_workspace(language);
    let bundle_id = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .map(ProjectSourceBundle::id)
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    open_source_file_dialog_in_bundle(app, language, bundle_id, action, source_path, importer_path)
}

pub(crate) fn open_source_file_dialog_in_bundle(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    action: CodeSourceFileAction,
    source_path: &str,
    importer_path: &str,
) -> Result<(), String> {
    if let Some(reason) = source_file_mutation_block_reason(app, language) {
        return Err(reason.to_owned());
    }
    let authority_path = if action == CodeSourceFileAction::New {
        importer_path
    } else {
        source_path
    };
    if !source_bundle_contains_document(app, language, bundle_id, authority_path) {
        return Err(format!(
            "The selected source file '{authority_path}' no longer belongs to this project source bundle."
        ));
    }
    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(bundle_id)
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    if action != CodeSourceFileAction::New && !bundle.contains_file(source_path) {
        return Err(format!(
            "The selected source file '{source_path}' no longer exists."
        ));
    }
    if (matches!(
        action,
        CodeSourceFileAction::Rename | CodeSourceFileAction::Move | CodeSourceFileAction::Delete
    ) && source_document_blocks_graph_mutation(bundle, language, source_path))
        || (action == CodeSourceFileAction::New
            && source_document_blocks_graph_mutation(bundle, language, importer_path))
    {
        return Err(
            "The managed Automation environment lock is read-only. Reassign its semantic role or duplicate it before changing the source graph."
                .to_owned(),
        );
    }
    if action == CodeSourceFileAction::Delete
        && crate::state::project_source_paths_equal(source_path, bundle.root().logical_path())
    {
        return Err("The bundle root cannot be deleted; replace or rename it instead.".to_owned());
    }
    let proposed_path = match action {
        CodeSourceFileAction::New => {
            unique_suggestion(bundle, importer_path, default_file_name(language))
        }
        CodeSourceFileAction::Duplicate => {
            unique_suggestion(bundle, source_path, duplicate_file_name(source_path))
        }
        CodeSourceFileAction::Rename
        | CodeSourceFileAction::Move
        | CodeSourceFileAction::Delete => source_path.to_owned(),
    };
    app.state.ui.code_workspace.source_file_dialog = Some(CodeSourceFileDialogState {
        action,
        project_id: app.state.workspace.project.id(),
        bundle_id,
        bundle_revision: bundle.revision().get(),
        closure_digest: bundle.closure_digest(),
        language,
        source_path: source_path.to_owned(),
        importer_path: importer_path.to_owned(),
        proposed_path,
        error: None,
    });
    Ok(())
}

pub(crate) fn commit_source_file_dialog(app: &mut RSpiceApp) -> Result<String, String> {
    let draft = app
        .state
        .ui
        .code_workspace
        .source_file_dialog
        .clone()
        .ok_or_else(|| "No source-file transaction is open.".to_owned())?;
    if let Some(reason) = source_file_mutation_block_reason(app, draft.language) {
        return Err(reason.to_owned());
    }
    if draft.project_id != app.state.workspace.project.id() {
        return Err(
            "The active project changed while the source-file transaction was open.".to_owned(),
        );
    }
    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(draft.bundle_id)
        .ok_or_else(|| {
            "The source bundle was removed while the transaction was open.".to_owned()
        })?;
    let authority_path = if draft.action == CodeSourceFileAction::New {
        draft.importer_path.as_str()
    } else {
        draft.source_path.as_str()
    };
    if !source_bundle_contains_document(app, draft.language, draft.bundle_id, authority_path) {
        return Err(
            "The source-file transaction no longer targets an authenticated project source bundle."
                .to_owned(),
        );
    }
    if bundle.language() != draft.language
        || bundle.revision().get() != draft.bundle_revision
        || bundle.closure_digest() != draft.closure_digest
    {
        return Err(
            "The source bundle changed while the transaction was open. Review the current tree and try again."
                .to_owned(),
        );
    }
    if (matches!(
        draft.action,
        CodeSourceFileAction::Rename | CodeSourceFileAction::Move | CodeSourceFileAction::Delete
    ) && source_document_blocks_graph_mutation(bundle, draft.language, &draft.source_path))
        || (draft.action == CodeSourceFileAction::New
            && source_document_blocks_graph_mutation(bundle, draft.language, &draft.importer_path))
    {
        return Err(
            "The managed Automation environment lock became read-only before the source-file transaction committed."
                .to_owned(),
        );
    }

    let proposed = draft.proposed_path.trim();
    let changed = match draft.action {
        CodeSourceFileAction::New => {
            let file =
                ProjectSourceFile::try_new(proposed, new_file_template(draft.language, proposed))
                    .map_err(|error| error.to_string())?;
            app.state
                .workspace
                .add_project_source_bundle_file(draft.bundle_id, &draft.importer_path, file)
                .map_err(|error| error.to_string())?
        }
        CodeSourceFileAction::Duplicate => {
            let content = bundle
                .file_content(&draft.source_path)
                .ok_or_else(|| {
                    format!("The source file '{}' no longer exists.", draft.source_path)
                })?
                .to_owned();
            let file =
                ProjectSourceFile::try_new(proposed, content).map_err(|error| error.to_string())?;
            app.state
                .workspace
                .add_project_source_bundle_file(draft.bundle_id, &draft.importer_path, file)
                .map_err(|error| error.to_string())?
        }
        CodeSourceFileAction::Rename | CodeSourceFileAction::Move => app
            .state
            .workspace
            .rename_project_source_bundle_file(draft.bundle_id, &draft.source_path, proposed)
            .map_err(|error| error.to_string())?,
        CodeSourceFileAction::Delete => app
            .state
            .workspace
            .remove_project_source_bundle_file(draft.bundle_id, &draft.source_path)
            .map_err(|error| error.to_string())?,
    };
    if !changed {
        app.state.ui.code_workspace.source_file_dialog = None;
        return Ok("No source-file changes were required.".to_owned());
    }

    let selected_path = match draft.action {
        CodeSourceFileAction::Delete => draft.importer_path.clone(),
        _ => proposed.to_owned(),
    };
    match draft.language {
        ProjectSourceLanguage::VerilogA => {
            app.state.ui.code_workspace.veriloga.selected_file = Some(VerilogAFileSelection {
                bundle_id: draft.bundle_id,
                logical_path: selected_path,
            });
            super::veriloga::invalidate_veriloga_evidence(app);
        }
        ProjectSourceLanguage::RSpiceAutomation => {
            app.state.ui.code_workspace.automation.selected_file = Some(selected_path);
            super::automation::invalidate_automation_evidence(app);
        }
    }
    app.state.ui.code_workspace.source_file_dialog = None;
    let message = match draft.action {
        CodeSourceFileAction::New => format!("Created project source '{proposed}'."),
        CodeSourceFileAction::Duplicate => {
            format!("Duplicated '{}' as '{proposed}'.", draft.source_path)
        }
        CodeSourceFileAction::Rename => {
            format!("Renamed '{}' to '{proposed}'.", draft.source_path)
        }
        CodeSourceFileAction::Move => {
            format!("Moved '{}' to '{proposed}'.", draft.source_path)
        }
        CodeSourceFileAction::Delete => format!("Deleted project source '{}'.", draft.source_path),
    };
    app.state
        .push_user_message(ConsoleMessage::info(message.clone()));
    Ok(message)
}

#[cfg(test)]
pub(crate) fn open_source_history(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
    source_path: &str,
) -> Result<(), String> {
    let owner = crate::state::ProjectSourceOwner::code_workspace(language);
    let bundle_id = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .map(ProjectSourceBundle::id)
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    open_source_history_in_bundle(app, language, bundle_id, source_path)
}

pub(crate) fn open_source_history_in_bundle(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    source_path: &str,
) -> Result<(), String> {
    if !source_bundle_contains_document(app, language, bundle_id, source_path) {
        return Err(format!(
            "The selected source file '{source_path}' no longer belongs to this project source bundle."
        ));
    }
    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(bundle_id)
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    if !bundle.contains_file(source_path) {
        return Err(format!(
            "The selected source file '{source_path}' no longer exists."
        ));
    }
    app.state.ui.code_workspace.source_history = Some(CodeSourceHistoryState {
        project_id: app.state.workspace.project.id(),
        bundle_id,
        bundle_revision: bundle.revision().get(),
        closure_digest: bundle.closure_digest(),
        language,
        source_path: source_path.to_owned(),
        selected_revision: bundle
            .revision_history()
            .last()
            .map(|snapshot| snapshot.revision().get()),
        error: None,
    });
    Ok(())
}

pub(crate) fn export_source_copy_in_bundle(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    source_path: &str,
) -> Result<String, String> {
    if !source_bundle_contains_document(app, language, bundle_id, source_path) {
        return Err(format!(
            "The selected source file '{source_path}' no longer belongs to this project source bundle."
        ));
    }
    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(bundle_id)
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    let source = bundle
        .file_content(source_path)
        .ok_or_else(|| format!("The selected source file '{source_path}' no longer exists."))?
        .to_owned();
    let default_name = source_path.rsplit('/').next().unwrap_or(source_path);
    let extensions: &[&str] = match language {
        ProjectSourceLanguage::VerilogA => &["va", "vams"],
        ProjectSourceLanguage::RSpiceAutomation => &["py", "yaml", "yml", "toml", "lock"],
    };
    let Some(path) = app.export_workflow_io.show_save_dialog(SaveDialogConfig {
        title: "Save project source copy",
        default_name,
        filter_name: language.label(),
        filter_extensions: extensions,
    })?
    else {
        return Ok("Source copy cancelled; no file was written.".to_owned());
    };
    let destination = app.export_workflow_io.observe_destination(&path)?;
    app.export_workflow_io
        .write_text_file_observed(&destination, &source)?;
    let message = if app.export_workflow_io.saved_paths_are_reopenable() {
        format!(
            "Saved an exact copy of '{source_path}' to {}.",
            path.display()
        )
    } else {
        format!("Downloaded an exact copy of '{source_path}'.")
    };
    app.state
        .push_user_message(ConsoleMessage::info(message.clone()));
    Ok(message)
}

pub(crate) fn request_automation_source_import(
    app: &mut RSpiceApp,
    importer_path: &str,
) -> Result<(), String> {
    let language = ProjectSourceLanguage::RSpiceAutomation;
    if let Some(reason) = source_file_mutation_block_reason(app, language) {
        return Err(reason.to_owned());
    }
    if app.state.ui.code_workspace.source_import.is_some() {
        return Err("An Automation source import is already pending.".to_owned());
    }
    let owner = crate::state::ProjectSourceOwner::code_workspace(language);
    let bundle = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .ok_or_else(|| "The Automation source bundle no longer exists.".to_owned())?;
    if !bundle.contains_file(importer_path) {
        return Err(format!("The importer '{importer_path}' no longer exists."));
    }
    if source_document_blocks_graph_mutation(bundle, language, importer_path) {
        return Err(
            "The managed Automation environment lock cannot own imported project dependencies. Select an editable project source first."
                .to_owned(),
        );
    }
    app.state.ui.code_workspace.source_import = Some(CodeSourceImportState {
        project_id: app.state.workspace.project.id(),
        bundle_id: bundle.id(),
        bundle_revision: bundle.revision().get(),
        closure_digest: bundle.closure_digest(),
        language,
        importer_path: importer_path.to_owned(),
        picker_started: false,
    });
    Ok(())
}

pub(crate) fn poll_automation_source_import(app: &mut RSpiceApp) {
    #[cfg(target_arch = "wasm32")]
    poll_browser_automation_import_completion(app);

    let Some(request) = app.state.ui.code_workspace.source_import.as_mut() else {
        return;
    };
    if request.language != ProjectSourceLanguage::RSpiceAutomation || request.picker_started {
        return;
    }
    request.picker_started = true;
    if let Some(reason) =
        source_file_mutation_block_reason(app, ProjectSourceLanguage::RSpiceAutomation)
    {
        app.state.ui.code_workspace.source_import = None;
        app.state.push_user_message(ConsoleMessage::error(format!(
            "Automation source import cancelled: {reason}"
        )));
        return;
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(request) = app.state.ui.code_workspace.source_import.take() else {
            return;
        };
        let Some(path) = rfd::FileDialog::new()
            .add_filter(
                "Automation source",
                &["py", "yaml", "yml", "toml", "lock", "json", "csv"],
            )
            .set_title("Import project-owned Automation source")
            .pick_file()
        else {
            return;
        };
        let result = (|| {
            let metadata = std::fs::metadata(&path).map_err(|error| error.to_string())?;
            if metadata.len() > crate::state::MAX_PROJECT_CODE_SOURCE_BYTES as u64 {
                return Err(format!(
                    "Selected source exceeds the supported {}-byte limit",
                    crate::state::MAX_PROJECT_CODE_SOURCE_BYTES
                ));
            }
            let bytes = std::fs::read(&path).map_err(|error| error.to_string())?;
            if bytes.len() > crate::state::MAX_PROJECT_CODE_SOURCE_BYTES {
                return Err(format!(
                    "Selected source exceeds the supported {}-byte limit",
                    crate::state::MAX_PROJECT_CODE_SOURCE_BYTES
                ));
            }
            let contents = String::from_utf8(bytes)
                .map_err(|error| format!("Selected source is not valid UTF-8: {error}"))?;
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .filter(|name| !name.trim().is_empty())
                .ok_or_else(|| "Selected source has no valid UTF-8 file name".to_owned())?;
            Ok((file_name.to_owned(), contents))
        })();
        match result {
            Ok((file_name, contents)) => {
                if let Err(error) =
                    apply_automation_source_import(app, request, file_name, contents)
                {
                    app.state.push_user_message(ConsoleMessage::error(format!(
                        "Automation source import failed: {error}"
                    )));
                }
            }
            Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
                "Automation source import failed: {error}"
            ))),
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let token = match crate::workbench::browser::file_import::try_begin_text_import(
            crate::workbench::browser::file_import::BrowserTextImportKind::Automation,
        ) {
            Ok(token) => token,
            Err(error) => {
                app.state.ui.code_workspace.source_import = None;
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Automation source import failed: {error}"
                )));
                return;
            }
        };
        crate::workbench::browser::file_import::pick_text_file(
            "Automation source",
            &["py", "yaml", "yml", "toml", "lock", "json", "csv"],
            move |result| {
                if crate::workbench::browser::file_import::text_import_is_current(token) {
                    BROWSER_AUTOMATION_IMPORT_COMPLETION.with(|slot| {
                        *slot.borrow_mut() =
                            Some(BrowserAutomationImportCompletion { token, result });
                    });
                }
            },
        );
    }
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_automation_import_completion(app: &mut RSpiceApp) {
    let Some(completion) =
        BROWSER_AUTOMATION_IMPORT_COMPLETION.with(|slot| slot.borrow_mut().take())
    else {
        return;
    };
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        return;
    }
    let Some(request) = app.state.ui.code_workspace.source_import.take() else {
        return;
    };
    match completion.result {
        Ok(Some(file)) => {
            if let Err(error) =
                apply_automation_source_import(app, request, file.name, file.contents)
            {
                app.state.push_user_message(ConsoleMessage::error(format!(
                    "Automation source import failed: {error}"
                )));
            }
        }
        Ok(None) => {}
        Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
            "Automation source import failed: {error}"
        ))),
    }
}

fn apply_automation_source_import(
    app: &mut RSpiceApp,
    request: CodeSourceImportState,
    file_name: String,
    contents: String,
) -> Result<String, String> {
    if let Some(reason) =
        source_file_mutation_block_reason(app, ProjectSourceLanguage::RSpiceAutomation)
    {
        return Err(reason.to_owned());
    }
    if app.state.workspace.project.id() != request.project_id {
        return Err("The active project changed while the source picker was open.".to_owned());
    }
    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(request.bundle_id)
        .ok_or_else(|| "The Automation source bundle no longer exists.".to_owned())?;
    if bundle.revision().get() != request.bundle_revision
        || bundle.closure_digest() != request.closure_digest
        || !bundle.contains_file(&request.importer_path)
    {
        return Err(
            "The Automation source bundle changed while the source picker was open.".to_owned(),
        );
    }
    if source_document_blocks_graph_mutation(
        bundle,
        ProjectSourceLanguage::RSpiceAutomation,
        &request.importer_path,
    ) {
        return Err(
            "The selected Automation importer is a read-only managed environment lock.".to_owned(),
        );
    }
    let logical_path = unique_suggestion(bundle, &request.importer_path, file_name);
    let file =
        ProjectSourceFile::try_new(&logical_path, contents).map_err(|error| error.to_string())?;
    app.state
        .workspace
        .add_project_source_bundle_file(request.bundle_id, &request.importer_path, file)
        .map_err(|error| error.to_string())?;
    app.state.ui.code_workspace.automation.selected_file = Some(logical_path.clone());
    super::automation::invalidate_automation_evidence(app);
    let message = format!("Imported project-owned Automation source '{logical_path}'.");
    app.state
        .push_user_message(ConsoleMessage::info(message.clone()));
    Ok(message)
}

pub(crate) fn import_dropped_automation_source(
    app: &mut RSpiceApp,
    file_name: String,
    contents: String,
) -> Result<String, String> {
    let language = ProjectSourceLanguage::RSpiceAutomation;
    if let Some(reason) = source_file_mutation_block_reason(app, language) {
        return Err(reason.to_owned());
    }
    if contents.len() > crate::state::MAX_PROJECT_CODE_SOURCE_BYTES {
        return Err(format!(
            "Dropped source exceeds the supported {}-byte limit.",
            crate::state::MAX_PROJECT_CODE_SOURCE_BYTES
        ));
    }
    let owner = ProjectSourceOwner::code_workspace(language);
    let bundle = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .ok_or_else(|| "The Automation source bundle no longer exists.".to_owned())?;
    let importer_path = app
        .state
        .ui
        .code_workspace
        .automation
        .selected_file
        .as_deref()
        .filter(|path| bundle.contains_file(path))
        .unwrap_or_else(|| bundle.root().logical_path())
        .to_owned();
    let request = CodeSourceImportState {
        project_id: app.state.workspace.project.id(),
        bundle_id: bundle.id(),
        bundle_revision: bundle.revision().get(),
        closure_digest: bundle.closure_digest(),
        language,
        importer_path,
        picker_started: true,
    };
    apply_automation_source_import(app, request, file_name, contents)
}

pub(crate) fn commit_source_history_restore(app: &mut RSpiceApp) -> Result<String, String> {
    let draft = app
        .state
        .ui
        .code_workspace
        .source_history
        .clone()
        .ok_or_else(|| "No source-history transaction is open.".to_owned())?;
    if app.state.workspace.project.id() != draft.project_id {
        return Err("The active project changed while source history was open.".to_owned());
    }
    if let Some(reason) = source_file_mutation_block_reason(app, draft.language) {
        return Err(reason.to_owned());
    }
    let retained_revision = draft
        .selected_revision
        .ok_or_else(|| "Select a retained revision to restore.".to_owned())?;
    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(draft.bundle_id)
        .ok_or_else(|| "The source bundle no longer exists.".to_owned())?;
    if !source_bundle_contains_document(app, draft.language, draft.bundle_id, &draft.source_path) {
        return Err(
            "Source history no longer targets an authenticated project source bundle.".to_owned(),
        );
    }
    if bundle.revision().get() != draft.bundle_revision
        || bundle.closure_digest() != draft.closure_digest
    {
        return Err(
            "The source bundle changed while history was open. Reopen history before restoring."
                .to_owned(),
        );
    }
    let expected_current = bundle.revision();
    let retained_revision = crate::product::ObjectRevision::new(retained_revision)
        .map_err(|error| format!("The retained revision is invalid: {error}"))?;
    let changed = app
        .state
        .workspace
        .restore_project_source_bundle_revision(
            draft.bundle_id,
            expected_current,
            retained_revision,
        )
        .map_err(|error| error.to_string())?;
    if !changed {
        app.state.ui.code_workspace.source_history = None;
        return Ok("The selected revision already matches the current source graph.".to_owned());
    }

    let restored_bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(draft.bundle_id)
        .ok_or_else(|| "The restored source bundle is unavailable.".to_owned())?;
    let selected_path = if restored_bundle.contains_file(&draft.source_path) {
        draft.source_path.clone()
    } else {
        restored_bundle.root().logical_path().to_owned()
    };
    match draft.language {
        ProjectSourceLanguage::VerilogA => {
            app.state.ui.code_workspace.veriloga.selected_file = Some(VerilogAFileSelection {
                bundle_id: draft.bundle_id,
                logical_path: selected_path,
            });
            super::veriloga::invalidate_veriloga_evidence(app);
        }
        ProjectSourceLanguage::RSpiceAutomation => {
            app.state.ui.code_workspace.automation.selected_file = Some(selected_path);
            super::automation::invalidate_automation_evidence(app);
        }
    }
    app.state.ui.code_workspace.source_history = None;
    let message = format!(
        "Restored {} source bundle revision {} as a new revision.",
        draft.language.label(),
        retained_revision.get()
    );
    app.state
        .push_user_message(ConsoleMessage::info(message.clone()));
    Ok(message)
}

fn default_file_name(language: ProjectSourceLanguage) -> &'static str {
    match language {
        ProjectSourceLanguage::VerilogA => "untitled.va",
        ProjectSourceLanguage::RSpiceAutomation => "helper.py",
    }
}

fn duplicate_file_name(path: &str) -> String {
    let (directory, file) = path
        .rsplit_once('/')
        .map_or(("", path), |(dir, file)| (dir, file));
    let (stem, extension) = file
        .rsplit_once('.')
        .map_or((file, ""), |(stem, ext)| (stem, ext));
    let file = if extension.is_empty() {
        format!("{stem}_copy")
    } else {
        format!("{stem}_copy.{extension}")
    };
    if directory.is_empty() {
        file
    } else {
        format!("{directory}/{file}")
    }
}

fn unique_suggestion(
    bundle: &crate::state::ProjectSourceBundle,
    anchor: &str,
    file_name: impl AsRef<str>,
) -> String {
    let directory = anchor
        .rsplit_once('/')
        .map_or("", |(directory, _)| directory);
    let requested = file_name.as_ref();
    let (stem, extension) = requested
        .rsplit_once('.')
        .map_or((requested, ""), |(stem, extension)| (stem, extension));
    for index in 0_u32..10_000 {
        let suffix = if index == 0 {
            String::new()
        } else {
            format!("_{index}")
        };
        let file = if extension.is_empty() {
            format!("{stem}{suffix}")
        } else {
            format!("{stem}{suffix}.{extension}")
        };
        let candidate = if directory.is_empty() {
            file
        } else {
            format!("{directory}/{file}")
        };
        if !bundle.contains_file(&candidate) {
            return candidate;
        }
    }
    format!("{requested}.new")
}

fn new_file_template(language: ProjectSourceLanguage, path: &str) -> String {
    match language {
        ProjectSourceLanguage::VerilogA if path.to_ascii_lowercase().ends_with(".vams") => {
            "// Project-owned Verilog-A definitions.\n".to_owned()
        }
        ProjectSourceLanguage::VerilogA => {
            "`include \"constants.vams\"\n`include \"disciplines.vams\"\n\n// Add project-owned Verilog-A declarations here.\n".to_owned()
        }
        ProjectSourceLanguage::RSpiceAutomation
            if path.to_ascii_lowercase().ends_with(".yaml")
                || path.to_ascii_lowercase().ends_with(".yml") =>
        {
            "# Project-owned Automation configuration.\n".to_owned()
        }
        ProjectSourceLanguage::RSpiceAutomation
            if path.to_ascii_lowercase().ends_with(".toml")
                || path.to_ascii_lowercase().ends_with(".lock") =>
        {
            "# Project-owned Automation configuration.\n".to_owned()
        }
        ProjectSourceLanguage::RSpiceAutomation => {
            "\"\"\"Project-owned RSpice Automation helper module.\"\"\"\n".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_source_lifecycle_has_no_panic_based_state_transitions() {
        let source = include_str!("source_files.rs");
        let production = source
            .split("#[cfg(test)]\nmod tests")
            .next()
            .expect("the production source precedes this test module");
        for forbidden in [".expect(", ".unwrap(", "panic!("] {
            assert!(
                !production.contains(forbidden),
                "production source lifecycle contains panic shortcut {forbidden}"
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn verilog_a_bundle(app: &RSpiceApp) -> &crate::state::ProjectSourceBundle {
        app.state
            .workspace
            .project_sources
            .bundle_for_owner(&crate::state::ProjectSourceOwner::code_workspace(
                ProjectSourceLanguage::VerilogA,
            ))
            .expect("the test project has a Verilog-A bundle")
    }

    #[test]
    fn duplicate_names_preserve_directory_and_extension() {
        assert_eq!(duplicate_file_name("models/gain.va"), "models/gain_copy.va");
        assert_eq!(duplicate_file_name("README"), "README_copy");
    }

    #[test]
    fn workspace_creation_rejects_unicode_case_alias_paths() {
        let error = new_veriloga_workspace_bundle("Models/étage.va", "MODELS/ÉTAGE.VA")
            .expect_err("one logical path cannot own both root and profile roles");
        assert!(error.contains("must use distinct project paths"), "{error}");
    }

    #[test]
    fn cell_view_file_transactions_retain_the_exact_veriloga_bundle() {
        let bundle = ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(crate::state::CellViewRef::new(
                "behavioral",
                "gain",
                "veriloga",
            )),
            ProjectSourceLanguage::VerilogA,
            "cell/gain.va",
            "module gain; endmodule\n",
            [],
            [],
        )
        .unwrap();
        let bundle_id = bundle.id();
        let mut app = RSpiceApp::test_instance();
        app.state
            .workspace
            .project_sources
            .insert_bundle(bundle)
            .unwrap();

        open_source_file_dialog_in_bundle(
            &mut app,
            ProjectSourceLanguage::VerilogA,
            bundle_id,
            CodeSourceFileAction::Duplicate,
            "cell/gain.va",
            "cell/gain.va",
        )
        .unwrap();
        app.state
            .ui
            .code_workspace
            .source_file_dialog
            .as_mut()
            .unwrap()
            .proposed_path = "cell/gain_copy.va".to_owned();
        commit_source_file_dialog(&mut app).unwrap();

        let exact = app
            .state
            .workspace
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert!(exact.contains_file("cell/gain_copy.va"));
        assert_eq!(
            app.state
                .ui
                .code_workspace
                .veriloga
                .selected_file
                .as_ref()
                .map(|selection| selection.bundle_id),
            Some(bundle_id)
        );
        open_source_history_in_bundle(
            &mut app,
            ProjectSourceLanguage::VerilogA,
            bundle_id,
            "cell/gain.va",
        )
        .unwrap();
        assert_eq!(
            app.state
                .ui
                .code_workspace
                .source_history
                .as_ref()
                .map(|history| history.bundle_id),
            Some(bundle_id)
        );
    }

    #[test]
    fn dropped_automation_source_uses_the_transactional_project_import_path() {
        let mut app = RSpiceApp::test_instance();
        let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation);
        let before = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .expect("test Automation bundle")
            .revision()
            .get();

        let message = import_dropped_automation_source(
            &mut app,
            "helpers.py".to_owned(),
            "def characterize():\n    return 42\n".to_owned(),
        )
        .expect("drop import");
        let logical_path = app
            .state
            .ui
            .code_workspace
            .automation
            .selected_file
            .clone()
            .expect("imported selection");
        assert!(message.contains(&logical_path));
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .unwrap();
        assert!(bundle.revision().get() > before);
        assert_eq!(
            bundle.file_content(&logical_path),
            Some("def characterize():\n    return 42\n")
        );
        assert!(app.state.workspace.project_sources_dirty);
    }

    #[test]
    fn editor_authority_rejects_closed_read_only_and_governed_sources() {
        let mut app = RSpiceApp::test_instance();
        let automation_owner =
            ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation);
        let automation = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&automation_owner)
            .expect("test Automation bundle");
        let automation_root = automation.root().logical_path().to_owned();
        let automation_bundle_id = automation.id();
        let automation_revision = automation.revision().get();
        let automation_digest = automation.closure_digest();
        let lock = automation
            .paths_for_role(ProjectSourceRole::AutomationEnvironmentLock)
            .next()
            .expect("managed environment lock")
            .to_owned();
        let verilog_owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA);
        let verilog_root = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&verilog_owner)
            .expect("test Verilog-A bundle")
            .root()
            .logical_path()
            .to_owned();

        assert!(source_document_is_editable(
            &app,
            ProjectSourceLanguage::VerilogA,
            &verilog_root
        ));
        assert!(source_document_is_editable(
            &app,
            ProjectSourceLanguage::RSpiceAutomation,
            &automation_root
        ));
        assert!(!source_document_is_editable(
            &app,
            ProjectSourceLanguage::RSpiceAutomation,
            &lock
        ));
        assert!(!source_document_is_editable(
            &app,
            ProjectSourceLanguage::VerilogA,
            "missing/model.va"
        ));
        assert!(!source_document_is_editable(
            &app,
            ProjectSourceLanguage::RSpiceAutomation,
            "missing/automation.py"
        ));

        app.state
            .ui
            .code_workspace
            .automation
            .pending_runtime_validation =
            Some(super::super::page::PendingAutomationRuntimeValidation {
                request_id: 41,
                bundle_id: automation_bundle_id,
                token: super::super::SourceOperationToken {
                    project_id: app.state.workspace.project.id(),
                    revision: automation_revision,
                    content_digest: automation_digest,
                },
            });
        assert!(!source_document_is_editable(
            &app,
            ProjectSourceLanguage::RSpiceAutomation,
            &automation_root
        ));
        app.state
            .ui
            .code_workspace
            .automation
            .pending_runtime_validation = None;

        app.state.project_lifecycle.project_open = false;
        assert!(!source_document_is_editable(
            &app,
            ProjectSourceLanguage::VerilogA,
            &verilog_root
        ));
        assert!(!source_document_is_editable(
            &app,
            ProjectSourceLanguage::RSpiceAutomation,
            &automation_root
        ));

        app.state.project_lifecycle.project_open = true;
        app.state.workbench.safe_mode.activate(
            crate::workbench::state::LocalSafeModeOptions {
                open_project_read_only: true,
                ..Default::default()
            },
            String::new(),
        );
        assert!(!source_document_is_editable(
            &app,
            ProjectSourceLanguage::VerilogA,
            &verilog_root
        ));
        assert!(!source_document_is_editable(
            &app,
            ProjectSourceLanguage::RSpiceAutomation,
            &automation_root
        ));
    }

    #[test]
    fn automation_editor_transaction_rechecks_read_only_authority_without_mutation() {
        let mut app = RSpiceApp::test_instance();
        let language = ProjectSourceLanguage::RSpiceAutomation;
        let owner = ProjectSourceOwner::code_workspace(language);
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .expect("test Automation bundle");
        let identity = CodeSourceEditorBufferIdentity {
            project_id: app.state.workspace.project.id(),
            bundle_id: bundle.id(),
            bundle_revision: bundle.revision().get(),
            closure_digest: bundle.closure_digest(),
            path: std::sync::Arc::from(bundle.root().logical_path()),
        };
        let original = bundle.root().content().to_owned();

        app.state.workbench.safe_mode.activate(
            crate::workbench::state::LocalSafeModeOptions {
                open_project_read_only: true,
                ..Default::default()
            },
            String::new(),
        );
        let error = replace_automation_editor_file(
            &mut app,
            &identity,
            "raise RuntimeError('must not commit')\n".to_owned(),
        )
        .unwrap_err();
        assert!(error.contains("read-only"));
        let unchanged = app
            .state
            .workspace
            .project_sources
            .get_bundle(identity.bundle_id)
            .expect("same Automation bundle");
        assert_eq!(unchanged.revision().get(), identity.bundle_revision);
        assert_eq!(unchanged.root().content(), original);
    }

    #[test]
    fn automation_surface_never_bypasses_edit_authority_for_legacy_migration() {
        let source = include_str!("../../surfaces/automation.rs");
        assert!(
            !source.contains("legacy || editable"),
            "legacy Automation presentation must not override project read-only authority"
        );
    }

    #[test]
    fn managed_environment_lock_blocks_graph_mutation_but_allows_an_editable_copy() {
        let mut app = RSpiceApp::test_instance();
        let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation);
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .expect("test Automation bundle");
        let bundle_id = bundle.id();
        let root = bundle.root().logical_path().to_owned();
        let lock = bundle
            .paths_for_role(ProjectSourceRole::AutomationEnvironmentLock)
            .next()
            .expect("managed environment lock")
            .to_owned();
        let revision = bundle.revision().get();

        for action in [
            CodeSourceFileAction::Rename,
            CodeSourceFileAction::Move,
            CodeSourceFileAction::Delete,
        ] {
            let error = open_source_file_dialog(
                &mut app,
                ProjectSourceLanguage::RSpiceAutomation,
                action,
                &lock,
                &root,
            )
            .unwrap_err();
            assert!(error.contains("environment lock is read-only"));
            assert!(app.state.ui.code_workspace.source_file_dialog.is_none());
        }

        let error = open_source_file_dialog(
            &mut app,
            ProjectSourceLanguage::RSpiceAutomation,
            CodeSourceFileAction::New,
            &lock,
            &lock,
        )
        .unwrap_err();
        assert!(error.contains("environment lock is read-only"));

        let error = request_automation_source_import(&mut app, &lock).unwrap_err();
        assert!(error.contains("cannot own imported project dependencies"));

        app.state.ui.code_workspace.automation.selected_file = Some(lock.clone());
        let error = import_dropped_automation_source(
            &mut app,
            "should-not-import.py".to_owned(),
            "value = 1\n".to_owned(),
        )
        .unwrap_err();
        assert!(error.contains("read-only managed environment lock"));

        open_source_file_dialog(
            &mut app,
            ProjectSourceLanguage::RSpiceAutomation,
            CodeSourceFileAction::Duplicate,
            &lock,
            &root,
        )
        .expect("an exact project-owned copy remains available");
        let draft = app
            .state
            .ui
            .code_workspace
            .source_file_dialog
            .as_ref()
            .expect("duplicate transaction");
        assert_eq!(draft.bundle_id, bundle_id);
        assert_eq!(draft.action, CodeSourceFileAction::Duplicate);
        assert_eq!(draft.source_path, lock);
        app.state
            .ui
            .code_workspace
            .source_file_dialog
            .as_mut()
            .expect("duplicate transaction")
            .action = CodeSourceFileAction::Rename;
        let error = commit_source_file_dialog(&mut app).unwrap_err();
        assert!(error.contains("became read-only"));
        assert_eq!(
            app.state
                .workspace
                .project_sources
                .get_bundle(bundle_id)
                .expect("same exact bundle")
                .revision()
                .get(),
            revision
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn source_file_actions_are_persisted_transactionally() {
        let mut app = RSpiceApp::test_instance();
        app.state.workspace.mark_project_sources_clean();
        let root = verilog_a_bundle(&app).root().logical_path().to_owned();

        open_source_file_dialog(
            &mut app,
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::New,
            &root,
            &root,
        )
        .unwrap();
        app.state
            .ui
            .code_workspace
            .source_file_dialog
            .as_mut()
            .unwrap()
            .proposed_path = "models/helper.va".to_owned();
        commit_source_file_dialog(&mut app).unwrap();
        assert!(verilog_a_bundle(&app).contains_file("models/helper.va"));
        assert!(app.state.workspace.project_sources_dirty);

        open_source_file_dialog(
            &mut app,
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::Rename,
            "models/helper.va",
            &root,
        )
        .unwrap();
        app.state
            .ui
            .code_workspace
            .source_file_dialog
            .as_mut()
            .unwrap()
            .proposed_path = "models/renamed.va".to_owned();
        commit_source_file_dialog(&mut app).unwrap();
        assert!(!verilog_a_bundle(&app).contains_file("models/helper.va"));
        assert!(verilog_a_bundle(&app).contains_file("models/renamed.va"));

        open_source_file_dialog(
            &mut app,
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::Move,
            "models/renamed.va",
            &root,
        )
        .unwrap();
        app.state
            .ui
            .code_workspace
            .source_file_dialog
            .as_mut()
            .unwrap()
            .proposed_path = "library/renamed.va".to_owned();
        commit_source_file_dialog(&mut app).unwrap();
        assert!(!verilog_a_bundle(&app).contains_file("models/renamed.va"));
        assert!(verilog_a_bundle(&app).contains_file("library/renamed.va"));

        open_source_file_dialog(
            &mut app,
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::Duplicate,
            "library/renamed.va",
            &root,
        )
        .unwrap();
        let duplicate = app
            .state
            .ui
            .code_workspace
            .source_file_dialog
            .as_ref()
            .unwrap()
            .proposed_path
            .clone();
        commit_source_file_dialog(&mut app).unwrap();
        assert!(verilog_a_bundle(&app).contains_file(&duplicate));

        open_source_file_dialog(
            &mut app,
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::Delete,
            &duplicate,
            &root,
        )
        .unwrap();
        commit_source_file_dialog(&mut app).unwrap();
        assert!(!verilog_a_bundle(&app).contains_file(&duplicate));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn source_file_commit_rejects_a_stale_bundle_transaction() {
        let mut app = RSpiceApp::test_instance();
        let root = verilog_a_bundle(&app).root().logical_path().to_owned();
        let bundle_id = verilog_a_bundle(&app).id();
        let before = verilog_a_bundle(&app).closure_digest();
        open_source_file_dialog(
            &mut app,
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::New,
            &root,
            &root,
        )
        .unwrap();
        app.state
            .workspace
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module changed; endmodule\n".to_owned(),
            )
            .unwrap();

        let error = commit_source_file_dialog(&mut app).unwrap_err();
        assert!(error.contains("changed while the transaction was open"));
        assert_ne!(verilog_a_bundle(&app).closure_digest(), before);
        assert!(!verilog_a_bundle(&app).contains_file("untitled.va"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn source_history_restores_the_exact_bundle_as_a_new_revision() {
        let mut app = RSpiceApp::test_instance();
        let bundle_id = verilog_a_bundle(&app).id();
        let root = verilog_a_bundle(&app).root().logical_path().to_owned();
        let original = verilog_a_bundle(&app).root().content().to_owned();
        app.state
            .workspace
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module changed; endmodule\n".to_owned(),
            )
            .unwrap();
        let edited_revision = verilog_a_bundle(&app).revision();

        open_source_history(&mut app, ProjectSourceLanguage::VerilogA, &root).unwrap();
        commit_source_history_restore(&mut app).unwrap();

        assert_eq!(verilog_a_bundle(&app).root().content(), original);
        assert!(verilog_a_bundle(&app).revision().get() > edited_revision.get());
        assert!(app.state.ui.code_workspace.source_history.is_none());
        assert!(app.state.workspace.project_sources_dirty);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn source_history_restore_rejects_a_newer_source_closure() {
        let mut app = RSpiceApp::test_instance();
        let bundle_id = verilog_a_bundle(&app).id();
        let root = verilog_a_bundle(&app).root().logical_path().to_owned();
        app.state
            .workspace
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module first; endmodule\n".to_owned(),
            )
            .unwrap();
        open_source_history(&mut app, ProjectSourceLanguage::VerilogA, &root).unwrap();
        app.state
            .workspace
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module newer; endmodule\n".to_owned(),
            )
            .unwrap();

        let error = commit_source_history_restore(&mut app).unwrap_err();
        assert!(error.contains("changed while history was open"));
        assert_eq!(
            verilog_a_bundle(&app).root().content(),
            "module newer; endmodule\n"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn bundle_root_delete_is_rejected_before_a_dialog_opens() {
        let mut app = RSpiceApp::test_instance();
        let root = verilog_a_bundle(&app).root().logical_path().to_owned();
        let error = open_source_file_dialog(
            &mut app,
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::Delete,
            &root,
            &root,
        )
        .unwrap_err();
        assert!(error.contains("bundle root cannot be deleted"));
        assert!(app.state.ui.code_workspace.source_file_dialog.is_none());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn automation_import_applies_to_the_exact_owned_bundle() {
        let mut app = RSpiceApp::test_instance();
        let language = ProjectSourceLanguage::RSpiceAutomation;
        let owner = crate::state::ProjectSourceOwner::code_workspace(language);
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .unwrap();
        let bundle_id = bundle.id();
        let importer = bundle.root().logical_path().to_owned();
        let logical_path = unique_suggestion(bundle, &importer, "analysis_helpers.py".to_owned());

        request_automation_source_import(&mut app, &importer).unwrap();
        let request = app.state.ui.code_workspace.source_import.take().unwrap();
        let message = apply_automation_source_import(
            &mut app,
            request,
            "analysis_helpers.py".to_owned(),
            "def gain(value):\n    return value * 2\n".to_owned(),
        )
        .unwrap();

        let bundle = app
            .state
            .workspace
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert_eq!(
            bundle.file_content(&logical_path),
            Some("def gain(value):\n    return value * 2\n")
        );
        assert_eq!(
            app.state.ui.code_workspace.automation.selected_file,
            Some(logical_path.clone())
        );
        assert!(message.contains(&logical_path));
        assert!(app.state.workspace.project_sources_dirty);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn automation_import_rejects_a_stale_picker_result() {
        let mut app = RSpiceApp::test_instance();
        let language = ProjectSourceLanguage::RSpiceAutomation;
        let owner = crate::state::ProjectSourceOwner::code_workspace(language);
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .unwrap();
        let bundle_id = bundle.id();
        let importer = bundle.root().logical_path().to_owned();

        request_automation_source_import(&mut app, &importer).unwrap();
        let request = app.state.ui.code_workspace.source_import.take().unwrap();
        app.state
            .workspace
            .replace_project_source_bundle_file(
                bundle_id,
                &importer,
                "# changed while picker was open\n".to_owned(),
            )
            .unwrap();

        let error = apply_automation_source_import(
            &mut app,
            request,
            "stale.py".to_owned(),
            "raise RuntimeError('must not be imported')\n".to_owned(),
        )
        .unwrap_err();
        assert!(error.contains("changed while the source picker was open"));
        assert!(
            !app.state
                .workspace
                .project_sources
                .get_bundle(bundle_id)
                .unwrap()
                .contains_file("stale.py")
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn automation_import_rechecks_read_only_authority_after_picker_open() {
        let mut app = RSpiceApp::test_instance();
        let language = ProjectSourceLanguage::RSpiceAutomation;
        let owner = ProjectSourceOwner::code_workspace(language);
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .unwrap();
        let bundle_id = bundle.id();
        let importer = bundle.root().logical_path().to_owned();
        let before_revision = bundle.revision().get();

        request_automation_source_import(&mut app, &importer).unwrap();
        let request = app.state.ui.code_workspace.source_import.take().unwrap();
        app.state.workbench.safe_mode.activate(
            crate::workbench::state::LocalSafeModeOptions {
                open_project_read_only: true,
                ..Default::default()
            },
            String::new(),
        );
        let error = apply_automation_source_import(
            &mut app,
            request,
            "must-not-import.py".to_owned(),
            "raise RuntimeError('must not commit')\n".to_owned(),
        )
        .unwrap_err();
        assert!(error.contains("read-only"));
        let unchanged = app
            .state
            .workspace
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert_eq!(unchanged.revision().get(), before_revision);
        assert!(!unchanged.contains_file("must-not-import.py"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn automation_editability_follows_persisted_role_not_filename() {
        let mut app = RSpiceApp::test_instance();
        let owner = crate::state::ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        );
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .unwrap();
        let bundle_id = bundle.id();
        let environment_path = bundle
            .paths_for_role(crate::state::ProjectSourceRole::AutomationEnvironmentLock)
            .next()
            .unwrap()
            .to_owned();
        app.state
            .workspace
            .rename_project_source_bundle_file(
                bundle_id,
                &environment_path,
                "environment/runtime-policy.toml",
            )
            .unwrap();
        assert!(!source_document_is_editable(
            &app,
            ProjectSourceLanguage::RSpiceAutomation,
            "environment/runtime-policy.toml",
        ));

        let root = app
            .state
            .workspace
            .project_sources
            .get_bundle(bundle_id)
            .unwrap()
            .root()
            .logical_path()
            .to_owned();
        app.state
            .workspace
            .add_project_source_bundle_file(
                bundle_id,
                &root,
                ProjectSourceFile::try_new("data/user-owned.lock", "value = 1\n").unwrap(),
            )
            .unwrap();
        assert!(source_document_is_editable(
            &app,
            ProjectSourceLanguage::RSpiceAutomation,
            "data/user-owned.lock",
        ));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn empty_project_creates_arbitrary_role_bound_automation_workspace() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .workspace
            .remove_project_source(ProjectSourceLanguage::RSpiceAutomation);
        app.state.workspace.mark_project_sources_clean();

        open_source_workspace_dialog(&mut app, ProjectSourceLanguage::RSpiceAutomation).unwrap();
        let draft = app
            .state
            .ui
            .code_workspace
            .source_workspace_dialog
            .as_mut()
            .unwrap();
        draft.root_path = "flows/nightly_entry.py".to_owned();
        draft.run_plan_path = "plans/qualification.data".to_owned();
        draft.environment_lock_path = "environment/pinned-runtime.toml".to_owned();
        draft.permission_manifest_path = "security/least-privilege.data".to_owned();
        draft.netlist_source_path = "circuits/owned-deck.cir".to_owned();

        commit_source_workspace_dialog(&mut app).unwrap();
        let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation);
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .unwrap();
        assert_eq!(bundle.root().logical_path(), "flows/nightly_entry.py");
        assert_eq!(
            bundle.role_for_path("plans/qualification.data"),
            Some(ProjectSourceRole::AutomationRunPlan)
        );
        assert_eq!(
            bundle.role_for_path("environment/pinned-runtime.toml"),
            Some(ProjectSourceRole::AutomationEnvironmentLock)
        );
        assert_eq!(
            bundle.role_for_path("security/least-privilege.data"),
            Some(ProjectSourceRole::AutomationPermissionManifest)
        );
        assert!(bundle.root().content().contains("plans/qualification.data"));
        assert!(
            bundle
                .file_content("plans/qualification.data")
                .unwrap()
                .contains("source: circuits/owned-deck.cir")
        );
        assert!(app.state.workspace.project_sources_dirty);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn automation_singleton_role_moves_in_one_monotonic_revision() {
        let mut app = RSpiceApp::test_instance();
        let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation);
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .unwrap();
        let bundle_id = bundle.id();
        let root = bundle.root().logical_path().to_owned();
        let old_lock = bundle
            .paths_for_role(ProjectSourceRole::AutomationEnvironmentLock)
            .next()
            .unwrap()
            .to_owned();
        app.state
            .workspace
            .add_project_source_bundle_file(
                bundle_id,
                &root,
                ProjectSourceFile::try_new(
                    "environment/next-runtime.data",
                    crate::automation_workflow::DEFAULT_ENVIRONMENT_LOCK,
                )
                .unwrap(),
            )
            .unwrap();
        let before = app
            .state
            .workspace
            .project_sources
            .get_bundle(bundle_id)
            .unwrap()
            .revision()
            .get();

        assign_automation_source_role(
            &mut app,
            "environment/next-runtime.data",
            Some(ProjectSourceRole::AutomationEnvironmentLock),
        )
        .unwrap();
        let bundle = app
            .state
            .workspace
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert_eq!(bundle.revision().get(), before + 1);
        assert_eq!(bundle.role_for_path(&old_lock), None);
        assert_eq!(
            bundle.role_for_path("environment/next-runtime.data"),
            Some(ProjectSourceRole::AutomationEnvironmentLock)
        );
        assert!(source_document_is_editable(
            &app,
            ProjectSourceLanguage::RSpiceAutomation,
            &old_lock,
        ));
        assert!(!source_document_is_editable(
            &app,
            ProjectSourceLanguage::RSpiceAutomation,
            "environment/next-runtime.data",
        ));
    }
}
