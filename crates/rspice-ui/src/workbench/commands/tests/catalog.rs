//! Catalog-wide command identity: what the product declares it can do.
//!
//! These claims hold over the whole registry rather than over one command. A
//! stable id is a contract with shortcuts, automation and the palette, so a
//! duplicate id, a title naming two actions at once, or an id that drifted
//! from the one a mockup fixed breaks something no single command's own test
//! can see.

use super::*;

#[test]
fn command_catalog_has_unique_stable_ids() {
    let mut ids = std::collections::HashSet::new();
    for command in vocabulary::COMMAND_REGISTRY {
        let id = command.spec().id;
        assert!(ids.insert(id), "duplicate command id {}", id);
        assert!(!id.is_empty());
        assert!(
            id.bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'),
            "command id is not a hyphenated product action: {id}"
        );
        assert_eq!(Command::from_stable_id(id), Some(*command));
    }
    for command in vocabulary::COMMAND_REGISTRY
        .iter()
        .copied()
        .filter(|command| !command.shortcut_bindings().is_empty())
    {
        assert!(
            ids.contains(command.stable_id()),
            "bindable command is missing a unique stable ID: {command:?}"
        );
    }
}

/// The palette is a title-matched list, so two commands sharing a title are two
/// rows the user cannot tell apart — and picking the wrong one goes somewhere
/// else entirely. Unique IDs do not cover this: they are never displayed.
#[test]
fn command_catalog_titles_name_exactly_one_action() {
    let mut labels: std::collections::HashMap<&str, Command> = std::collections::HashMap::new();
    for command in vocabulary::COMMAND_REGISTRY.iter().copied() {
        let label = command.spec().label;
        if let Some(previous) = labels.insert(label, command) {
            panic!("{previous:?} and {command:?} both offer the palette row {label:?}");
        }
    }
}

#[test]
fn legacy_model_metadata_audit_identity_migrates_to_qualification() {
    assert_eq!(
        Command::from_stable_id("model-metadata-audit"),
        Some(Command::ModelsPage(ModelsPage::Qualification))
    );
    assert_eq!(
        Command::ModelsPage(ModelsPage::Qualification).stable_id(),
        "model-qualification"
    );
}

#[test]
fn all_workspace_commands_are_discoverable() {
    for workspace in Workspace::ALL {
        assert!(vocabulary::COMMAND_REGISTRY.contains(&Command::OpenWorkspace(workspace)));
    }
}

#[test]
fn protected_commands_keep_the_exact_mockup_action_ids() {
    assert_eq!(Command::CommandPalette.spec().id, "command-palette");
    assert_eq!(Command::ToggleFocusMode.spec().id, "toggle-focus-mode");
    assert_eq!(Command::RunSimulation.spec().id, "start-run");
    assert_eq!(Command::StopSimulation.spec().id, "stop-run");
    assert_eq!(Command::OpenProject.spec().id, "open-project");
    assert_eq!(Command::OpenNetlist.spec().id, "open-netlist");
    assert_eq!(Command::NewProject.spec().id, "new-project");
    assert_eq!(Command::Save.spec().id, "save-project");
    assert_eq!(Command::CloseActiveDocument.spec().id, "close-document");
    assert_eq!(Command::ToggleFullScreen.spec().id, "full-screen");
    assert_eq!(Command::GenerateNetlist.spec().id, "generated-netlist");
    assert_eq!(Command::ToggleConsole.spec().id, "console");
    assert_eq!(Command::PreviousDocument.spec().id, "previous-document");
    assert_eq!(Command::NextDocument.spec().id, "next-document");
    assert_eq!(
        Command::CloseOtherDocuments.spec().id,
        "close-other-documents"
    );
    assert_eq!(Command::CloseAllDocuments.spec().id, "close-all-documents");
    assert_eq!(Command::WorkspaceLayouts.spec().id, "workspace-layouts");
    assert_eq!(Command::WindowManager.spec().id, "window-manager");
    assert_eq!(Command::HelpCenter.spec().id, "help-center");
    assert_eq!(Command::ReleaseNotes.spec().id, "release-notes");
    assert_eq!(Command::MigrationGuide.spec().id, "migration-guide");
    assert_eq!(Command::SystemDiagnostics.spec().id, "system-diagnostics");
    assert_eq!(Command::SupportBundle.spec().id, "support-bundle");
    assert_eq!(Command::LegalPrivacy.spec().id, "legal-privacy-center");
    assert_eq!(Command::OpenConsole.spec().id, "open-console");
    assert_eq!(Command::OpenProblems.spec().id, "open-problems");
    assert_eq!(
        Command::ToggleConsoleMaximized.spec().id,
        "console-maximize"
    );
    assert_eq!(Command::ClearConsole.spec().id, "console-clear");
    assert_eq!(
        Command::FeatureAvailability.spec(),
        CommandSpec {
            id: "feature-availability",
            label: "Product capability and platform matrix…",
            group: "Help",
        }
    );
    assert_eq!(
        Command::InteroperabilityMatrix.spec(),
        CommandSpec {
            id: "interoperability-matrix",
            label: "Interoperability and format matrix…",
            group: "Help",
        }
    );
    assert_eq!(
        Command::OpenWorkspace(Workspace::Results).spec().id,
        "results"
    );
    assert_eq!(
        Command::OpenWorkspace(Workspace::Verify).spec().id,
        "verify"
    );
    assert_eq!(
        Command::OpenWorkspace(Workspace::Models).spec().id,
        "models"
    );
    assert_eq!(
        Command::OpenWorkspace(Workspace::Netlist).spec().label,
        "Open Netlist & Script Editor"
    );
    assert_eq!(
        Command::ModelsPage(ModelsPage::Models).spec().label,
        "Model & library catalog"
    );
}
