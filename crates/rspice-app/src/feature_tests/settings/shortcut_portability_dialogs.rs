//! The shortcut import and export windows over the product's own commands.

use crate::state::command_catalog::CommandPlatform;
use crate::state::command_catalog::ShortcutContext;
use crate::state::shortcuts::ShortcutPreferences;
use crate::state::shortcuts::ShortcutProfileLibrary;
use crate::state::shortcuts::{ShortcutBindingSlot, ShortcutSequence, ShortcutStroke};
use crate::workbench::app::dialogs::preferences::shortcut_portability_dialogs::*;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::shortcuts::artifacts::DecodedShortcutArtifact;
use crate::workbench::shortcuts::artifacts::DetectedShortcutArtifact;
use crate::workbench::shortcuts::artifacts::ImportBindingClass;
use crate::workbench::shortcuts::artifacts::ShortcutArtifactFormat;
use crate::workbench::shortcuts::artifacts::ShortcutConflictPolicy;
use crate::workbench::shortcuts::artifacts::ShortcutExportRequest;
use crate::workbench::shortcuts::artifacts::ShortcutExportScope;
use crate::workbench::shortcuts::artifacts::ShortcutImportOptions;
use crate::workbench::shortcuts::artifacts::ShortcutImportPlan;
use crate::workbench::shortcuts::artifacts::ShortcutMergePolicy;
use crate::workbench::shortcuts::artifacts::build_shortcut_reference_model;
use crate::workbench::shortcuts::artifacts::plan_shortcut_import;
use crate::workbench::shortcuts::artifacts::{
    apply_shortcut_import, decode_shortcut_artifact_json, detect_shortcut_artifact,
    serialize_shortcut_reference_json,
};
use egui::Context;

pub(crate) fn exported_artifact(profile: &ShortcutPreferences) -> DecodedShortcutArtifact {
    let model = build_shortcut_reference_model(
        profile,
        &ShortcutExportRequest {
            scope: ShortcutExportScope::UserOverrides,
            include_platform_mappings: true,
            runtime_platform: CommandPlatform::Desktop,
            operating_system: egui::os::OperatingSystem::Windows,
            current_contexts: Vec::new(),
        },
        command_registry(),
    )
    .unwrap();
    decode_shortcut_artifact_json(
        "rspice-shortcuts.json",
        &serialize_shortcut_reference_json(&model).unwrap(),
    )
    .unwrap()
}

pub(crate) fn detected(profile: &ShortcutPreferences) -> DetectedShortcutArtifact {
    let model = build_shortcut_reference_model(
        profile,
        &ShortcutExportRequest::user_overrides(
            CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
        ),
        command_registry(),
    )
    .unwrap();
    let json = serialize_shortcut_reference_json(&model).unwrap();
    detect_shortcut_artifact(
        "rspice-shortcuts.json",
        json.as_bytes(),
        crate::workbench::shortcuts::artifacts::VscodeHostPlatform::Windows,
        command_registry(),
    )
    .unwrap()
}

pub(crate) fn open_with_artifact(
    state: &mut ShortcutImportDialogState,
    base: &ShortcutProfileLibrary,
    artifact: DecodedShortcutArtifact,
) {
    state.open();
    state.artifact = Some(artifact);
    state.plan_dirty = true;
    state.refresh_plan(base, command_registry());
}

#[test]
fn option_order_and_labels_are_stable() {
    assert_eq!(
        MERGE_POLICIES.map(ShortcutMergePolicy::label),
        [
            "Merge non-conflicting bindings",
            "Replace current user bindings",
        ]
    );
    assert_eq!(
        CONFLICT_POLICIES.map(ShortcutConflictPolicy::label),
        [
            "Keep current and report",
            "Use imported binding",
            "Leave both unbound",
        ]
    );
    assert_eq!(
        IMPORT_CLASSES.map(ImportBindingClass::label),
        ["Global", "Schematic", "Results", "Simulation",]
    );
}

#[test]
fn changed_workflow_requires_a_second_explicit_discard() {
    let base = ShortcutProfileLibrary::default();
    let mut import = ShortcutImportDialogState::default();
    open_with_artifact(
        &mut import,
        &base,
        exported_artifact(&ShortcutPreferences::default()),
    );
    import.request_idle_cancel();
    assert!(import.is_open());
    assert!(import.discard_confirmation);
    import.request_idle_cancel();
    assert!(!import.is_open());

    let mut export = ShortcutExportDialogState::default();
    export.open();
    export.scope = ShortcutExportScope::CompleteResolved;
    export.request_idle_cancel();
    assert!(export.is_open());
    assert!(export.discard_confirmation);
    export.request_idle_cancel();
    assert!(!export.is_open());
}

#[test]
fn unchanged_workflow_closes_on_the_first_cancel() {
    let mut import = ShortcutImportDialogState::default();
    import.open();
    import.request_idle_cancel();
    assert!(!import.is_open());

    let mut export = ShortcutExportDialogState::default();
    export.open();
    export.request_idle_cancel();
    assert!(!export.is_open());
}

#[test]
fn source_review_is_isolated_until_the_typed_commit_effect_completes() {
    let base = ShortcutProfileLibrary::default();
    let mut imported = ShortcutPreferences::default();
    imported
        .set_binding(
            &crate::workbench::commands::vocabulary::CommandRegistry,
            Command::ZoomFit.key(),
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop],
            Some(ShortcutSequence::single(ShortcutStroke::new(
                egui::Key::F6,
                false,
                false,
                false,
            ))),
        )
        .unwrap();
    let artifact = exported_artifact(&imported);
    let mut state = ShortcutImportDialogState::default();
    open_with_artifact(&mut state, &base, artifact);

    assert_eq!(base.active(), &ShortcutPreferences::default());
    assert!(
        state
            .plan
            .as_ref()
            .is_some_and(ShortcutImportPlan::can_apply)
    );

    state.pending = ImportPending::Commit;
    let plan = state.plan.clone().unwrap();
    let mut published = base.clone();
    let receipt = apply_shortcut_import(&mut published, &plan, |_| Ok(())).unwrap();
    state.complete_commit(Ok(receipt));

    assert!(!state.is_open());
    assert!(state.last_receipt.is_some());
    assert_ne!(published.active(), base.active());
}

#[test]
fn stale_base_replans_before_commit() {
    let mut base = ShortcutProfileLibrary::default();
    let artifact = exported_artifact(&ShortcutPreferences::default());
    let mut state = ShortcutImportDialogState::default();
    open_with_artifact(&mut state, &base, artifact);
    let original_revision = state.plan.as_ref().unwrap().base_revision();
    base.edit_active(|profile| {
        profile.set_binding(
            &crate::workbench::commands::vocabulary::CommandRegistry,
            Command::ZoomFit.key(),
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop],
            None,
        )
    })
    .unwrap()
    .unwrap();
    state.synchronize_plan(&base, command_registry());
    assert_ne!(
        state.plan.as_ref().unwrap().base_revision(),
        original_revision
    );
    assert_eq!(
        state.plan.as_ref().unwrap().base_revision(),
        base.revision()
    );
}

#[test]
fn failed_commit_cancellation_keeps_ownership_until_receipt_arrives() {
    let base = ShortcutProfileLibrary::default();
    let artifact = exported_artifact(&ShortcutPreferences::default());
    let plan = plan_shortcut_import(
        &base,
        &artifact,
        &ShortcutImportOptions::default(),
        command_registry(),
    )
    .unwrap();
    let mut published = base.clone();
    let receipt = apply_shortcut_import(&mut published, &plan, |_| Ok(())).unwrap();
    let receipt_source = receipt.source_name().to_owned();
    let mut state = ShortcutImportDialogState {
        open: true,
        pending: ImportPending::Commit,
        cancel_requested: true,
        ..ShortcutImportDialogState::default()
    };

    state.commit_cancelled(Err("commit boundary reached".to_owned()));
    assert_eq!(state.pending, ImportPending::Commit);
    assert!(state.cancel_requested);
    assert!(state.is_open());
    state.complete_commit(Ok(receipt));
    assert!(!state.cancel_requested);
    assert!(!state.is_open());
    assert_eq!(
        state.last_receipt.as_ref().unwrap().source_name(),
        receipt_source
    );
}

#[test]
fn export_effect_contains_fully_prepared_private_artifact() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = ShortcutPortabilityDialogsState::default();
    state.open_export();
    let library = ShortcutProfileLibrary::default();
    let mut action = ShortcutPortabilityAction::None;
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 720.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            let _ = state.render(
                ctx,
                &library,
                ShortcutExportEnvironment {
                    runtime_platform: CommandPlatform::Desktop,
                    operating_system: egui::os::OperatingSystem::Windows,
                    current_contexts: &[ShortcutContext::EngineeringCanvas],
                },
                command_registry(),
            );
        },
    );
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 720.0),
            )),
            events: vec![egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: Some(egui::Key::Enter),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ctx| {
            action = state.render(
                ctx,
                &library,
                ShortcutExportEnvironment {
                    runtime_platform: CommandPlatform::Desktop,
                    operating_system: egui::os::OperatingSystem::Windows,
                    current_contexts: &[ShortcutContext::EngineeringCanvas],
                },
                command_registry(),
            );
        },
    );
    let ShortcutPortabilityAction::PublishExport(artifact) = action else {
        panic!("expected prepared export effect");
    };
    assert_eq!(artifact.format(), ShortcutArtifactFormat::Json);
    let text = std::str::from_utf8(artifact.bytes()).unwrap();
    assert!(!text.contains("source-path"));
    assert!(!text.contains("protected-override-acknowledgements"));
}

#[test]
fn modal_accessibility_tree_exposes_exact_titles_and_summary_table() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let library = ShortcutProfileLibrary::default();
    let artifact = exported_artifact(&ShortcutPreferences::default());
    let mut state = ShortcutPortabilityDialogsState::default();
    state.open_import();
    open_with_artifact(&mut state.import, &library, artifact);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 720.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            let _ = state.render(
                ctx,
                &library,
                ShortcutExportEnvironment {
                    runtime_platform: CommandPlatform::Desktop,
                    operating_system: egui::os::OperatingSystem::Windows,
                    current_contexts: &[],
                },
                command_registry(),
            );
        },
    );
    let nodes = output.platform_output.accesskit_update.unwrap().nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Dialog
            && node.label() == Some(IMPORT_TITLE)
            && node.is_modal()
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Table
            && node.label() == Some("Shortcut import compatibility summary")
    }));
    assert_eq!(
        nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Row)
            .count(),
        5
    );
    assert_eq!(
        nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::ColumnHeader)
            .count(),
        4
    );
    assert_eq!(
        nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Cell)
            .count(),
        16
    );
    for label in ["Source", "Current bindings", "Conflict handling"] {
        assert!(nodes.iter().any(|(_, node)| node.label() == Some(label)));
    }
}

#[test]
fn pending_import_controls_are_accessibly_disabled_and_source_keeps_picker_semantics() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let library = ShortcutProfileLibrary::default();
    let mut state = ShortcutPortabilityDialogsState::default();
    state.open_import();
    state.import.pending = ImportPending::Commit;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 720.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            let _ = state.render(
                ctx,
                &library,
                ShortcutExportEnvironment {
                    runtime_platform: CommandPlatform::Desktop,
                    operating_system: egui::os::OperatingSystem::Windows,
                    current_contexts: &[],
                },
                command_registry(),
            );
        },
    );
    let nodes = output.platform_output.accesskit_update.unwrap().nodes;
    for label in ["Source", "Current bindings", "Conflict handling"] {
        assert!(
            nodes
                .iter()
                .any(|(_, node)| { node.label() == Some(label) && node.is_disabled() })
        );
    }
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Button
            && node.label() == Some("Source")
            && node.value() == Some(IMPORT_SOURCE_PLACEHOLDER)
    }));
}

#[test]
fn pending_render_is_structurally_immutable_even_for_incomplete_restored_state() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let library = ShortcutProfileLibrary::default();
    let mut state = ShortcutImportDialogState {
        open: true,
        options: ShortcutImportOptions {
            merge_policy: ShortcutMergePolicy::ReplaceCurrentUserBindings,
            ..ShortcutImportOptions::default()
        },
        pending: ImportPending::Commit,
        ..ShortcutImportDialogState::default()
    };
    let before = state.options.clone();
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 720.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = render_import_body(ui, &mut state, &library, command_registry());
            });
        },
    );
    assert_eq!(state.options, before);
    assert_eq!(state.pending, ImportPending::Commit);
}

#[test]
fn pending_export_controls_are_accessibly_disabled() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let library = ShortcutProfileLibrary::default();
    let mut state = ShortcutPortabilityDialogsState::default();
    state.open_export();
    state.export.pending = true;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 720.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            let _ = state.render(
                ctx,
                &library,
                ShortcutExportEnvironment {
                    runtime_platform: CommandPlatform::Desktop,
                    operating_system: egui::os::OperatingSystem::Windows,
                    current_contexts: &[],
                },
                command_registry(),
            );
        },
    );
    let nodes = output.platform_output.accesskit_update.unwrap().nodes;
    for label in ["Format", "Scope", "Include platform shortcut alternatives"] {
        assert!(
            nodes
                .iter()
                .any(|(_, node)| node.label() == Some(label) && node.is_disabled())
        );
    }
}

#[test]
fn helper_artifact_is_canonical() {
    // Prevent the helper from silently drifting to a foreign adaptation.
    assert!(matches!(
        detected(&ShortcutPreferences::default()),
        DetectedShortcutArtifact::RSpice(_)
    ));
}

#[test]
fn route_loss_closes_idle_dialogs_and_waits_for_exact_async_cancellation() {
    let mut dialogs = ShortcutPortabilityDialogsState::default();
    dialogs.open_import();
    assert!(matches!(
        dialogs.request_route_close(),
        ShortcutPortabilityAction::None
    ));
    assert!(!dialogs.application_modal_open());

    dialogs.open_import();
    dialogs.import.pending = ImportPending::Source;
    assert!(matches!(
        dialogs.request_route_close(),
        ShortcutPortabilityAction::CancelImportSource
    ));
    assert!(dialogs.import.cancel_requested);
    assert!(dialogs.application_modal_open());
    assert!(matches!(
        dialogs.request_route_close(),
        ShortcutPortabilityAction::None
    ));
    dialogs.import_source_cancelled();
    assert!(!dialogs.application_modal_open());

    dialogs.open_import();
    dialogs.import.pending = ImportPending::Commit;
    assert!(matches!(
        dialogs.request_route_close(),
        ShortcutPortabilityAction::CancelImportCommit
    ));
    assert!(dialogs.application_modal_open());
    dialogs.import_commit_cancelled(Ok(()));
    assert!(!dialogs.application_modal_open());

    dialogs.open_export();
    assert!(matches!(
        dialogs.request_route_close(),
        ShortcutPortabilityAction::None
    ));
    assert!(!dialogs.application_modal_open());
}

#[test]
fn opening_a_second_portability_workflow_never_drops_the_current_owner() {
    let mut dialogs = ShortcutPortabilityDialogsState::default();
    dialogs.open_import();
    dialogs.import.pending = ImportPending::Source;
    dialogs.open_export();
    assert!(dialogs.import.is_open());
    assert!(!dialogs.export.is_open());

    dialogs.import.pending = ImportPending::None;
    dialogs.import.close();
    dialogs.open_export();
    dialogs.open_import();
    assert!(dialogs.export.is_open());
    assert!(!dialogs.import.is_open());
}
