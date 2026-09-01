//! Which retained dialog owners block background shortcuts.
//!
//! A modal that leaves shortcuts live lets a keystroke edit the design behind
//! it. The modeless waveform calculator is the deliberate exception, and it is
//! asserted here so that "every retained owner blocks" stays a rule with one
//! named hole rather than an unstated pattern.

use super::*;

fn assert_blocks_shortcuts(configure: impl FnOnce(&mut DialogState)) {
    let mut dialogs = DialogState::default();
    assert!(!dialogs.application_modal_open());
    configure(&mut dialogs);
    assert!(dialogs.application_modal_open());
}

#[test]
fn every_retained_dialog_owner_blocks_background_shortcuts() {
    assert_blocks_shortcuts(|dialogs| dialogs.about = true);
    assert_blocks_shortcuts(|dialogs| dialogs.shortcuts_help = true);
    assert_blocks_shortcuts(|dialogs| dialogs.help_center.open(HelpCenterPage::Help));
    assert_blocks_shortcuts(|dialogs| {
        dialogs.window_session.open(WindowSessionPage::Documents);
    });
    assert_blocks_shortcuts(|dialogs| dialogs.new_cell_dialog = true);
    assert_blocks_shortcuts(|dialogs| dialogs.new_view_dialog = true);
    assert_blocks_shortcuts(|dialogs| dialogs.copy_cell_dialog = true);
    assert_blocks_shortcuts(|dialogs| dialogs.rename_cell_dialog = true);
    assert_blocks_shortcuts(|dialogs| dialogs.rename_selection.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.design_management.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.create_model_bound_symbol.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.symbol_import.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.symbol_parameter_form.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.engineering_table.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.preferences_open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.drawing_sheet_setup.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.drawing_sheet_support.manager.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.drawing_sheet_support.title_fields.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.managed_preference_policy_open = true);
    assert_blocks_shortcuts(|dialogs| {
        dialogs
            .workspace_layout_manager
            .open(crate::workbench::WorkspacePreset::Engineering);
    });
    assert_blocks_shortcuts(|dialogs| dialogs.shortcut_portability.open_import());
    assert_blocks_shortcuts(|dialogs| dialogs.shortcut_portability.open_export());
    assert_blocks_shortcuts(|dialogs| dialogs.shortcut_editor.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.license_dialog.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.publish_web.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.live_session.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.unpublish_web.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.command_palette.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.bus_tap.open());
    assert_blocks_shortcuts(|dialogs| dialogs.builtin_xspice_placement.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.net_label_placement.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.schematic_visibility.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.drawing_sheet_layers.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.drawing_sheet_overflow_open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.grid_snap_routing.open = true);
    assert_blocks_shortcuts(|dialogs| {
        dialogs.pin_port.open(
            "BIAS_EN".to_owned(),
            0,
            0,
            0,
            "user/top/schematic".to_owned(),
        );
    });
    assert_blocks_shortcuts(|dialogs| {
        dialogs
            .design_note
            .open(0, 0, 0, "user/top/schematic".to_owned());
    });
    assert_blocks_shortcuts(|dialogs| {
        dialogs
            .documentation_shape
            .open(0, 0, 0, "user/top/schematic".to_owned(), Vec::new());
    });
    assert_blocks_shortcuts(|dialogs| dialogs.selection_workflow.open = true);
    assert_blocks_shortcuts(|dialogs| {
        dialogs.move_selection.open(super::SchematicEditAuthority {
            design_execution_epoch: 0,
            active_schematic_epoch: 0,
            topology_version: 0,
            view_path: "user/top/schematic".to_owned(),
            grid_size: 10,
            document_policy: crate::state::SchematicDocumentPolicy::default(),
            snapshot: crate::state::SchematicSnapshot::capture(
                &crate::state::SchematicState::default(),
            ),
            selection: crate::state::Selection::default(),
        });
    });
    assert_blocks_shortcuts(|dialogs| {
        dialogs.stretch_selection.open(
            super::SchematicEditAuthority {
                design_execution_epoch: 0,
                active_schematic_epoch: 0,
                topology_version: 0,
                view_path: "user/top/schematic".to_owned(),
                grid_size: 10,
                document_policy: crate::state::SchematicDocumentPolicy::default(),
                snapshot: crate::state::SchematicSnapshot::capture(
                    &crate::state::SchematicState::default(),
                ),
                selection: crate::state::Selection::default(),
            },
            crate::state::StretchTarget::WireSegment {
                wire_id: 1,
                segment_index: 0,
            },
        );
    });
    assert_blocks_shortcuts(|dialogs| {
        dialogs.array_selection.open(
            super::SchematicEditAuthority {
                design_execution_epoch: 0,
                active_schematic_epoch: 0,
                topology_version: 0,
                view_path: "user/top/schematic".to_owned(),
                grid_size: 10,
                document_policy: crate::state::SchematicDocumentPolicy::default(),
                snapshot: crate::state::SchematicSnapshot::capture(
                    &crate::state::SchematicState::default(),
                ),
                selection: crate::state::Selection::default(),
            },
            "8 \u{00d7} 1".to_owned(),
            "U4\u{2026}U11 \u{00b7} DATA[0]\u{2026}DATA[7]".to_owned(),
        );
    });
    assert_blocks_shortcuts(|dialogs| {
        dialogs.replace_instance.open(ReplaceInstanceOpen {
            authority: super::SchematicEditAuthority {
                design_execution_epoch: 0,
                active_schematic_epoch: 0,
                topology_version: 0,
                view_path: "user/top/schematic".to_owned(),
                grid_size: 10,
                document_policy: crate::state::SchematicDocumentPolicy::default(),
                snapshot: crate::state::SchematicSnapshot::capture(
                    &crate::state::SchematicState::default(),
                ),
                selection: crate::state::Selection::default(),
            },
            replacement_authority: crate::state::SchematicReplacementAuthority {
                component_id: 1,
                topology_version: 0,
                source_component: crate::state::Component::new(
                    1,
                    crate::state::ComponentType::VoltageSource,
                    crate::state::Point::origin(),
                ),
                source_spec: crate::state::SchematicReplacementSourceSpec::new(
                    Vec::new(),
                    Vec::<String>::new(),
                ),
            },
            source_component_id: 1,
            current: "U1 \u{00b7} OPA189".to_owned(),
            replacement: "OPA188 \u{00b7} 5 pin".to_owned(),
            initial_target_identity: "work/OPA188/schematic".to_owned(),
            initial_target_spec: crate::state::SchematicReplacementTargetSpec::primitive(
                crate::state::ComponentType::Resistor,
            ),
            mapping: "5 / 5 pins \u{00b7} 6 / 8 parameters".to_owned(),
        });
    });
    assert_blocks_shortcuts(|dialogs| {
        let bus = crate::state::Bus::segment(
            1,
            crate::state::Point::new(0, 0),
            crate::state::Point::new(10, 0),
            None,
        )
        .unwrap();
        dialogs
            .object_properties
            .open_bus(&bus, 0, 0, 0, String::new());
    });
    assert_blocks_shortcuts(|dialogs| dialogs.selection_bulk_edit.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.technology_attachment.open = true);
    assert_blocks_shortcuts(|dialogs| {
        dialogs.interaction.schematic_delete_confirmation_open = true;
    });
    assert_blocks_shortcuts(|dialogs| {
        dialogs.library_deletion_review.target = Some(LibraryDeletionTarget::Cell {
            library: "work".to_owned(),
            cell: "filter".to_owned(),
        });
    });
    assert_blocks_shortcuts(|dialogs| dialogs.confirmation_dialog.visible = true);
    assert_blocks_shortcuts(|dialogs| dialogs.project_review_dialog.show_close_project());
}

#[test]
fn modeless_waveform_calculator_does_not_block_workspace_shortcuts() {
    let mut dialogs = DialogState::default();
    dialogs.waveform_calculator_dialog = true;
    assert!(
        !dialogs.application_modal_open(),
        "the modeless calculator must leave plot and workspace shortcuts active"
    );
}

#[test]
fn generic_property_dirty_state_clears_after_semantic_revert() {
    let bus = crate::state::Bus::segment(
        7,
        crate::state::Point::new(0, 0),
        crate::state::Point::new(10, 0),
        Some(crate::state::BusDeclaration::parse("DATA[7:0]").unwrap()),
    )
    .unwrap();
    let mut dialog = ObjectPropertiesDialogState::default();
    dialog.open_bus(&bus, 1, 2, 3, "work/top/schematic".to_owned());

    let Some(ObjectPropertiesDraft::Bus(draft)) = dialog.draft.as_mut() else {
        unreachable!()
    };
    draft.declaration = "ADDR[7:0]".to_owned();
    dialog.mark_edited();
    assert!(dialog.dirty);

    let Some(ObjectPropertiesDraft::Bus(draft)) = dialog.draft.as_mut() else {
        unreachable!()
    };
    draft.declaration = " DATA[7:0] ".to_owned();
    dialog.mark_edited();
    assert!(!dialog.dirty);
    assert!(!dialog.discard_confirm);
}
