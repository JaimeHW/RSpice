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
    for command in vocabulary::ShellCommand::ALL {
        let id = command.spec().id;
        assert!(ids.insert(id), "duplicate command id {}", id);
        assert!(!id.is_empty());
        assert!(
            id.bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'),
            "command id is not a hyphenated product action: {id}"
        );
        assert_eq!(ShellCommand::from_stable_id(id), Some(*command));
    }
    for command in vocabulary::ShellCommand::ALL
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
    use rspice_app_state::state::command_catalog::{CommandCatalog, CommandKey, CommandPlatform};

    let catalog = vocabulary::CommandRegistry;
    let shell_commands = catalog
        .commands()
        .iter()
        .copied()
        .filter(|key| vocabulary::ShellCommand::from_key(*key).is_some())
        .collect::<Vec<CommandKey>>();
    assert_eq!(shell_commands.len(), vocabulary::ShellCommand::ALL.len());
    for (key, command) in shell_commands
        .into_iter()
        .zip(vocabulary::ShellCommand::ALL.iter().copied())
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
    let mut labels: std::collections::HashMap<&str, ShellCommand> =
        std::collections::HashMap::new();
    for command in vocabulary::ShellCommand::ALL.iter().copied() {
        let label = command.spec().label;
        if let Some(previous) = labels.insert(label, command) {
            panic!("{previous:?} and {command:?} both offer the palette row {label:?}");
        }
    }
}

#[test]
fn all_workspace_commands_are_discoverable() {
    for workspace in Workspace::ALL {
        assert!(vocabulary::ShellCommand::ALL.contains(&ShellCommand::OpenWorkspace(workspace)));
    }
}

#[test]
fn command_palette_and_help_commands_own_canonical_routes() {
    let mut palette = RSpiceApp::test_instance();
    let palette_source = palette.state.workbench.current_route();
    palette.execute_command_key(ShellCommand::CommandPalette.key());
    assert_eq!(
        palette.state.workbench.current_route(),
        rspice_app_workbench::workbench::routing::surface_route::SurfaceRoute::surface(
            rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId::CommandPalette
        )
    );
    assert!(palette.state.workbench.command_palette.open);
    assert!(palette.state.workbench.command_palette.route_owned);
    assert_eq!(
        palette.state.workbench.previous_route(),
        Some(palette_source)
    );

    let mut help = RSpiceApp::test_instance();
    let help_source = help.state.workbench.current_route();
    help.execute_command_key(ShellCommand::LegalPrivacy.key());
    assert_eq!(
        help.state.workbench.current_route(),
        rspice_app_workbench::workbench::routing::surface_route::SurfaceRoute::surface(
            rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId::HelpCenter
        )
    );
    assert!(help.state.shell.help_center.open);
    assert!(help.state.shell.help_center.route_owned);
    assert_eq!(
        help.state.shell.help_center.page,
        crate::workbench::app::HelpCenterPage::LegalPrivacy
    );
    assert_eq!(help.state.workbench.previous_route(), Some(help_source));
}
