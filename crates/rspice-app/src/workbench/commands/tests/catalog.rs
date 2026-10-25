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

/// A shortcut profile reads its command facts by key through the registry's
/// catalog. Every registered command answers there as it answers itself, in
/// registry order, and a retired id resolves to the command it became.
#[test]
fn the_registry_catalog_answers_for_every_command_as_the_command_does() {
    use crate::state::command_catalog::{CommandCatalog, CommandKey, CommandPlatform};

    let catalog = vocabulary::CommandRegistry;
    let shell_commands = catalog
        .commands()
        .iter()
        .copied()
        .filter(|key| vocabulary::legacy_command(*key).is_some())
        .collect::<Vec<CommandKey>>();
    assert_eq!(shell_commands.len(), vocabulary::COMMAND_REGISTRY.len());
    for (key, command) in shell_commands
        .into_iter()
        .zip(vocabulary::COMMAND_REGISTRY.iter().copied())
    {
        assert_eq!(key, command.key());
        assert_eq!(key.as_str(), command.stable_id());
        assert_eq!(catalog.resolve(command.stable_id()), Some(key));
        assert_eq!(catalog.context(key), Some(command.shortcut_context()));
        assert_eq!(catalog.default_bindings(key), command.shortcut_bindings());
        for platform in CommandPlatform::ALL {
            assert_eq!(
                catalog.primary_is_reserved_on(key, platform),
                command.primary_is_reserved_on(platform),
                "{command:?} on {platform:?}"
            );
        }
    }
    for &(alias, live) in super::rows::LEGACY_ALIASES {
        assert_eq!(catalog.resolve(alias).map(CommandKey::as_str), Some(live));
    }
    let unknown = CommandKey::new("future-command");
    assert_eq!(catalog.resolve(unknown.as_str()), None);
    assert_eq!(catalog.context(unknown), None);
    assert!(catalog.default_bindings(unknown).is_empty());
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
    assert_eq!(
        SettingsCommand::WorkspaceLayouts.key().as_str(),
        "workspace-layouts"
    );
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
    // A shortcut profile names its commands by these ids.
    assert_eq!(Command::MoveSelection.spec().id, "move-selection");
    assert_eq!(Command::StretchSelection.spec().id, "stretch-selection");
    assert_eq!(Command::ArraySelection.spec().id, "array-selection");
    assert_eq!(Command::ReplaceInstance.spec().id, "replace-instance");
    assert_eq!(Command::RenameSelection.spec().id, "rename-selection");
    assert_eq!(Command::PlaceBus.spec().id, "place-bus");
    assert_eq!(Command::PlaceBusTap.spec().id, "place-bus-tap");
    assert_eq!(Command::PlacePin.spec().id, "place-pin");
    assert_eq!(Command::PlaceText.spec().id, "place-text");
    assert_eq!(Command::PlaceShape.spec().id, "place-shape");
    assert_eq!(
        Command::DesignSpecialistWorkspaces.spec().id,
        "specialist-tools-design"
    );
    assert_eq!(Command::ConfigurationSets.spec().id, "configuration-sets");
    assert_eq!(Command::DesignManagement.spec().id, "design-management");
    assert_eq!(
        Command::ConnectivityManager.spec().id,
        "design-connectivity-tools"
    );
    assert_eq!(Command::SelectionBulkEdit.spec().id, "design-bulk-tools");
    assert_eq!(SheetsCommand::NextSheet.key().as_str(), "next-sheet");
    assert_eq!(
        SheetsCommand::PreviousSheet.key().as_str(),
        "previous-sheet"
    );
    assert_eq!(SheetsCommand::NewSheet.key().as_str(), "new-sheet");
    assert_eq!(SheetsCommand::DeleteSheet.key().as_str(), "delete-sheet");
    assert_eq!(Command::ShowInNetlist.spec().id, "show-in-netlist");
    assert_eq!(Command::ModelEditor.spec().id, "model-editor");
    assert_eq!(
        ResultsCommand::ImportResultDataset.key().as_str(),
        "import-dataset"
    );
    // The capability and interoperability matrices were deleted. A shortcut
    // profile that still names them keeps the entry but resolves nothing.
    assert_eq!(Command::from_stable_id("feature-availability"), None);
    assert_eq!(Command::from_stable_id("interoperability-matrix"), None);
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
        Command::OpenWorkspace(Workspace::Stimulus).spec(),
        CommandSpec {
            id: "stimulus",
            label: "Open Stimulus Library",
            group: "Navigate",
        }
    );
    assert_eq!(
        Command::ModelsPage(ModelsPage::Models).spec().label,
        "Model & library catalog"
    );
}

#[test]
fn command_palette_and_help_commands_own_canonical_routes() {
    let mut palette = RSpiceApp::test_instance();
    let palette_source = palette.state.workbench.current_route();
    Command::CommandPalette.execute(&mut palette);
    assert_eq!(
        palette.state.workbench.current_route(),
        crate::workbench::SurfaceRoute::surface(crate::workbench::SurfaceId::CommandPalette)
    );
    assert!(palette.state.workbench.command_palette.open);
    assert!(palette.state.workbench.command_palette.route_owned);
    assert_eq!(
        palette.state.workbench.previous_route(),
        Some(palette_source)
    );

    let mut help = RSpiceApp::test_instance();
    let help_source = help.state.workbench.current_route();
    Command::ReleaseNotes.execute(&mut help);
    assert_eq!(
        help.state.workbench.current_route(),
        crate::workbench::SurfaceRoute::surface(crate::workbench::SurfaceId::HelpCenter)
    );
    assert!(help.state.shell.help_center.open);
    assert!(help.state.shell.help_center.route_owned);
    assert_eq!(
        help.state.shell.help_center.page,
        crate::workbench::app::HelpCenterPage::ReleaseNotes
    );
    assert_eq!(help.state.workbench.previous_route(), Some(help_source));
}
