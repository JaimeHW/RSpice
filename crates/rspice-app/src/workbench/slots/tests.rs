//! Which retained dialog owners block background shortcuts.
//!
//! A modal that leaves shortcuts live lets a keystroke edit the design behind
//! it. The modeless waveform calculator is the deliberate exception, and it is
//! asserted here so that "every retained owner blocks" stays a rule with one
//! named hole rather than an unstated pattern.

use super::*;
use crate::workbench::app::HelpCenterPage;
use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::dialogs::library::pending_deletions::LibraryDeletionTarget;
use crate::workbench::app::dialogs::object_properties::ObjectPropertiesDialogState;
use crate::workbench::app::dialogs::plan_removal::PlanRemovalTarget;
use crate::workbench::app::dialogs::replace_instance::ReplaceInstanceOpen;
use crate::workbench::app::dialogs::selection::commands::floating::{
    FloatingPaste, FloatingPasteOrigin,
};
use crate::workbench::app::dialogs::window_session::WindowSessionPage;
use crate::workbench::docks::RunDetailsDialogState;
use crate::workbench::lifecycle::project_lifecycle::ProjectCloseDestination;
use crate::workbench::menu_bar::DatasetExportPicker;
use rspice_schematic_editor::object_properties::ObjectPropertiesDraft;

fn edit_authority() -> SchematicEditAuthority {
    SchematicEditAuthority::capture(&crate::workbench::app_state::AppState::default())
}

fn assert_blocks_shortcuts(configure: impl FnOnce(&mut AppState)) {
    let mut dialogs = AppState::default();
    assert!(!dialogs.dialog_modal_open());
    configure(&mut dialogs);
    assert!(dialogs.dialog_modal_open());
}

#[test]
fn every_retained_dialog_owner_blocks_background_shortcuts() {
    assert_blocks_shortcuts(|dialogs| dialogs.shell.about = true);
    assert_blocks_shortcuts(|dialogs| dialogs.shell.shortcuts_help = true);
    assert_blocks_shortcuts(|dialogs| dialogs.shell.help_center.open(HelpCenterPage::Help));
    assert_blocks_shortcuts(|dialogs| {
        dialogs
            .workbench
            .window_session_dialog
            .open(WindowSessionPage::Documents);
    });
    assert_blocks_shortcuts(|dialogs| dialogs.project.new_cell_dialog = true);
    assert_blocks_shortcuts(|dialogs| dialogs.project.new_view_dialog = true);
    assert_blocks_shortcuts(|dialogs| dialogs.project.copy_cell_dialog = true);
    assert_blocks_shortcuts(|dialogs| dialogs.project.rename_cell_dialog = true);
    assert_blocks_shortcuts(|dialogs| dialogs.design.rename_selection.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.project.design_management.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.design.create_model_bound_symbol.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.design.symbol_import.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.design.symbol_parameter_form.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.design.engineering_table.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.settings.preferences_open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.sheets.drawing_sheet_support.manager.open = true);
    assert_blocks_shortcuts(|dialogs| {
        dialogs
            .settings
            .workspace_layout_manager
            .open(crate::workbench::WorkspacePreset::Engineering);
    });
    assert_blocks_shortcuts(|dialogs| dialogs.settings.shortcut_portability.open_import());
    assert_blocks_shortcuts(|dialogs| dialogs.settings.shortcut_portability.open_export());
    assert_blocks_shortcuts(|dialogs| dialogs.settings.shortcut_editor.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.cloud.license_dialog.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.cloud.publish_web.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.cloud.live_session.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.cloud.unpublish_web.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.workbench.command_palette.open = true);
    assert_blocks_shortcuts(|dialogs| {
        dialogs.design.bus_tap.open(
            crate::workbench::app::schematic_editor_request_source(
                &crate::workbench::app_state::AppState::default(),
            ),
            String::new(),
        );
    });
    assert_blocks_shortcuts(|dialogs| dialogs.design.builtin_xspice_placement.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.design.net_label_placement.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.design.schematic_visibility_dialog.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.sheets.drawing_sheet_overflow_open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.design.grid_snap_routing.open = true);
    assert_blocks_shortcuts(|dialogs| {
        dialogs.design.pin_port.open(
            "EN".to_owned(),
            crate::state::PlacementAuthority::new(
                crate::workbench::app::schematic_editor_request_source(
                    &crate::workbench::app_state::AppState::default(),
                ),
            ),
        );
    });
    assert_blocks_shortcuts(|dialogs| {
        dialogs
            .design
            .design_note
            .open(crate::workbench::app::schematic_editor_request_source(
                &crate::workbench::app_state::AppState::default(),
            ));
    });
    assert_blocks_shortcuts(|dialogs| {
        dialogs.design.documentation_shape.open(
            crate::workbench::app::schematic_editor_request_source(
                &crate::workbench::app_state::AppState::default(),
            ),
            Vec::new(),
        );
    });
    assert_blocks_shortcuts(|dialogs| {
        dialogs.design.array_selection.open(
            edit_authority(),
            "8 \u{00d7} 1".to_owned(),
            "U4\u{2026}U11 \u{00b7} DATA[0]\u{2026}DATA[7]".to_owned(),
        );
    });
    assert_blocks_shortcuts(|dialogs| {
        dialogs.design.replace_instance.open(ReplaceInstanceOpen {
            authority: edit_authority(),
            replacement_authority: {
                let mut schematic = crate::state::SchematicState::default();
                schematic
                    .document_mut_for_test()
                    .components
                    .push(crate::state::Component::new(
                        1,
                        crate::state::ComponentType::VoltageSource,
                        crate::state::Point::origin(),
                    ));
                schematic.session.editor.selection.select_only_component(1);
                schematic
                    .replacement_authority()
                    .expect("valid replacement review fixture")
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
            .design
            .object_properties
            .open_bus(&bus, edit_authority().source, String::new());
    });
    assert_blocks_shortcuts(|dialogs| dialogs.design.selection_bulk_edit.open = true);
    assert_blocks_shortcuts(|dialogs| dialogs.project.technology_attachment.open = true);
    assert_blocks_shortcuts(|dialogs| {
        dialogs.project.library_deletion_review.target = Some(LibraryDeletionTarget::Cell {
            library: "work".to_owned(),
            cell: "filter".to_owned(),
        });
    });
    assert_blocks_shortcuts(|dialogs| dialogs.workbench.confirmation_dialog.visible = true);
    assert_blocks_shortcuts(|dialogs| {
        dialogs
            .workbench
            .project_review_dialog
            .show_close_project(ProjectCloseDestination::EmptyWorkbench);
    });
}

fn open_drawing_sheet_presets_alone(dialogs: &mut AppState) {
    let mut state = crate::workbench::app_state::AppState::default();
    assert!(
        crate::workbench::app::dialogs::drawing_sheet_presets::open_custom_sheet_size_library(
            &mut state
        )
    );
    dialogs.sheets.drawing_sheet_presets = state.sheets.drawing_sheet_presets.clone();
}

fn open_veriloga_build_settings_alone(dialogs: &mut AppState) {
    use crate::workbench::app::veriloga_design_tests::{
        add_veriloga_cell, open_in_design, open_project_app,
    };

    let mut app = open_project_app();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    crate::workbench::app::dialogs::veriloga_build_settings::open_veriloga_build_settings(&mut app)
        .expect("Build settings opens on a Verilog-A view");
    dialogs.code.veriloga_build_settings = app.state.code.veriloga_build_settings.take();
}

/// A dialog by name, and the least that opens it on a closed dialog state.
type DialogRow = (&'static str, fn(&mut AppState));

/// One row per term of [`AppState::dialog_modal_open`], in the order
/// the gate lists them.
const DIALOG_GATE: &[DialogRow] = &[
    ("about", |dialogs| dialogs.shell.about = true),
    ("shortcuts_help", |dialogs| {
        dialogs.shell.shortcuts_help = true
    }),
    ("help_center", |dialogs| {
        dialogs.shell.help_center.open(HelpCenterPage::Help);
    }),
    ("window_session", |dialogs| {
        dialogs
            .workbench
            .window_session_dialog
            .open(WindowSessionPage::Documents);
    }),
    ("new_cell_dialog", |dialogs| {
        dialogs.project.new_cell_dialog = true
    }),
    ("new_view_dialog", |dialogs| {
        dialogs.project.new_view_dialog = true
    }),
    ("new_veriloga_device", |dialogs| {
        dialogs.code.new_veriloga_device.open = true;
    }),
    (
        "veriloga_build_settings",
        open_veriloga_build_settings_alone,
    ),
    ("copy_cell_dialog", |dialogs| {
        dialogs.project.copy_cell_dialog = true
    }),
    ("rename_cell_dialog", |dialogs| {
        dialogs.project.rename_cell_dialog = true
    }),
    ("rename_view_dialog", |dialogs| {
        dialogs.project.rename_view_dialog = true
    }),
    ("new_library_dialog", |dialogs| {
        dialogs.project.new_library_dialog = true
    }),
    ("rename_library_dialog", |dialogs| {
        dialogs.project.rename_library_dialog = true;
    }),
    ("delete_library_dialog", |dialogs| {
        dialogs.project.delete_library_dialog = true;
    }),
    ("library_deletion_review", |dialogs| {
        dialogs.project.library_deletion_review.target = Some(LibraryDeletionTarget::Cell {
            library: "work".to_owned(),
            cell: "filter".to_owned(),
        });
    }),
    ("plan_removal_review", |dialogs| {
        dialogs.simulate.plan_removal_review.target = Some(PlanRemovalTarget::Analysis(
            crate::product::AnalysisInstanceId::new(),
        ));
    }),
    ("plan_removal_refusal", |dialogs| {
        dialogs.simulate.plan_removal_refusal.analysis =
            Some(crate::product::AnalysisInstanceId::new());
    }),
    ("dataset_export", |dialogs| {
        dialogs.results.dataset_export = Some(DatasetExportPicker {
            format: 0,
            several_analyses: false,
        });
    }),
    ("preferences_open", |dialogs| {
        dialogs.settings.preferences_open = true
    }),
    ("hardcopy", |dialogs| dialogs.hardcopy.hardcopy.open = true),
    ("drawing_sheet_defaults", |dialogs| {
        dialogs.sheets.drawing_sheet_defaults.open = true;
    }),
    ("drawing_sheet_presets", open_drawing_sheet_presets_alone),
    ("drawing_sheet_support", |dialogs| {
        dialogs.sheets.drawing_sheet_support.manager.open = true;
    }),
    ("workspace_layout_manager", |dialogs| {
        dialogs
            .settings
            .workspace_layout_manager
            .open(crate::workbench::WorkspacePreset::Engineering);
    }),
    ("shortcut_portability (import)", |dialogs| {
        dialogs.settings.shortcut_portability.open_import();
    }),
    ("shortcut_portability (export)", |dialogs| {
        dialogs.settings.shortcut_portability.open_export();
    }),
    ("shortcut_editor", |dialogs| {
        dialogs.settings.shortcut_editor.open = true
    }),
    ("license_dialog", |dialogs| {
        dialogs.cloud.license_dialog.open = true
    }),
    ("new_project", |dialogs| {
        dialogs.project.new_project.open = true
    }),
    ("publish_web", |dialogs| {
        dialogs.cloud.publish_web.open = true
    }),
    ("live_session", |dialogs| {
        dialogs.cloud.live_session.open = true
    }),
    ("unpublish_web", |dialogs| {
        dialogs.cloud.unpublish_web.open = true
    }),
    ("run_details", |dialogs| {
        let run = crate::state::SimulationRun::new(1);
        dialogs.simulate.run_details = Some(RunDetailsDialogState {
            run_id: run.id,
            manifest: crate::workbench::documents::result_document::manifest::manifest_for_run(
                &run,
            ),
        });
    }),
    ("command_palette", |dialogs| {
        dialogs.workbench.command_palette.open = true
    }),
    ("bus_tap", |dialogs| {
        dialogs.design.bus_tap.open(
            crate::workbench::app::schematic_editor_request_source(
                &crate::workbench::app_state::AppState::default(),
            ),
            String::new(),
        );
    }),
    ("builtin_xspice_placement", |dialogs| {
        dialogs.design.builtin_xspice_placement.open = true;
    }),
    ("net_label_placement", |dialogs| {
        dialogs.design.net_label_placement.open = true;
    }),
    ("schematic_visibility", |dialogs| {
        dialogs.design.schematic_visibility_dialog.open = true;
    }),
    ("drawing_sheet_overflow_open", |dialogs| {
        dialogs.sheets.drawing_sheet_overflow_open = true;
    }),
    ("grid_snap_routing", |dialogs| {
        dialogs.design.grid_snap_routing.open = true
    }),
    ("engineering_table", |dialogs| {
        dialogs.design.engineering_table.open = true
    }),
    ("pin_port", |dialogs| {
        dialogs.design.pin_port.open(
            "EN".to_owned(),
            crate::state::PlacementAuthority::new(
                crate::workbench::app::schematic_editor_request_source(
                    &crate::workbench::app_state::AppState::default(),
                ),
            ),
        );
    }),
    ("design_note", |dialogs| {
        dialogs
            .design
            .design_note
            .open(crate::workbench::app::schematic_editor_request_source(
                &crate::workbench::app_state::AppState::default(),
            ));
    }),
    ("documentation_shape", |dialogs| {
        dialogs.design.documentation_shape.open(
            crate::workbench::app::schematic_editor_request_source(
                &crate::workbench::app_state::AppState::default(),
            ),
            Vec::new(),
        );
    }),
    ("array_selection", |dialogs| {
        dialogs.design.array_selection.open(
            edit_authority(),
            "8 \u{00d7} 1".to_owned(),
            "U4\u{2026}U11".to_owned(),
        );
    }),
    ("replace_instance", |dialogs| {
        let mut schematic = crate::state::SchematicState::default();
        schematic
            .document_mut_for_test()
            .components
            .push(crate::state::Component::new(
                1,
                crate::state::ComponentType::VoltageSource,
                crate::state::Point::origin(),
            ));
        schematic.session.editor.selection.select_only_component(1);
        dialogs.design.replace_instance.open(ReplaceInstanceOpen {
            authority: edit_authority(),
            replacement_authority: schematic
                .replacement_authority()
                .expect("valid replacement review fixture"),
            source_component_id: 1,
            current: "U1".to_owned(),
            replacement: "OPA188".to_owned(),
            initial_target_identity: "work/OPA188/schematic".to_owned(),
            initial_target_spec: crate::state::SchematicReplacementTargetSpec::primitive(
                crate::state::ComponentType::Resistor,
            ),
            mapping: "5 / 5 pins".to_owned(),
        });
    }),
    ("stimulus_link", |dialogs| {
        dialogs.design.stimulus_link.open = true
    }),
    ("create_hierarchy", |dialogs| {
        dialogs.design.create_hierarchy.open = true
    }),
    ("check_and_save", |dialogs| {
        dialogs.project.check_and_save.open = true
    }),
    ("connectivity_manager", |dialogs| {
        dialogs.design.connectivity_manager.open = true;
    }),
    ("delete_sheet", |dialogs| {
        dialogs.sheets.delete_sheet = Some(
            crate::workbench::app::dialogs::delete_sheet::PendingSheetDelete {
                sheet: crate::state::SheetId::new(),
                name: "Sheet 2".to_owned(),
                objects: 3,
                destination: crate::state::SheetId::new(),
                destination_name: "Sheet 1".to_owned(),
            },
        );
    }),
    ("design_management", |dialogs| {
        dialogs.project.design_management.open = true;
    }),
    ("selection_bulk_edit", |dialogs| {
        dialogs.design.selection_bulk_edit.open = true;
    }),
    ("design_review_comments", |dialogs| {
        dialogs.design.design_review_comments.open = true;
    }),
    ("project_revision_history", |dialogs| {
        dialogs.project.project_revision_history.open = true;
    }),
    ("create_model_bound_symbol", |dialogs| {
        dialogs.design.create_model_bound_symbol.open = true;
    }),
    ("symbol_import", |dialogs| {
        dialogs.design.symbol_import.open = true
    }),
    ("symbol_parameter_form", |dialogs| {
        dialogs.design.symbol_parameter_form.open = true;
    }),
    ("configuration_sets", |dialogs| {
        dialogs.simulate.configuration_sets.open = true;
    }),
    ("rule_setup", |dialogs| {
        dialogs.design.rule_setup.open = true
    }),
    ("object_properties", |dialogs| {
        let bus = crate::state::Bus::segment(
            1,
            crate::state::Point::new(0, 0),
            crate::state::Point::new(10, 0),
            None,
        )
        .expect("a valid bus fixture");
        dialogs
            .design
            .object_properties
            .open_bus(&bus, edit_authority().source, String::new());
    }),
    ("rename_selection", |dialogs| {
        dialogs.design.rename_selection.open = true
    }),
    ("find_replace", |dialogs| {
        dialogs.design.find_replace.open = true
    }),
    ("shared_properties", |dialogs| {
        dialogs.design.shared_properties.open = true;
    }),
    ("technology_attachment", |dialogs| {
        dialogs.project.technology_attachment.open = true;
    }),
    ("confirmation_dialog", |dialogs| {
        dialogs.workbench.confirmation_dialog.visible = true;
    }),
    ("project_review_dialog", |dialogs| {
        dialogs
            .workbench
            .project_review_dialog
            .show_close_project(ProjectCloseDestination::EmptyWorkbench);
    }),
];

/// The gate's one term that exists only where a native autosave can be
/// found beside the file being opened.
#[cfg(not(target_arch = "wasm32"))]
const NATIVE_DIALOG_GATE: &[DialogRow] = &[("pending_autosave_restore", |dialogs| {
    use crate::workbench::lifecycle::recovery_checkpoint::{
        AutosaveRestoreCandidate, CheckpointBinding, CheckpointIdentity, CheckpointOwnership,
    };
    dialogs.platform.pending_autosave_restore = Some(AutosaveRestoreCandidate {
        source: std::path::PathBuf::from("afe.rspiceproj"),
        checkpoint: std::path::PathBuf::from("afe.rspiceproj.autosave"),
        binding: CheckpointBinding {
            identity: CheckpointIdentity {
                content_digest: crate::product::ContentDigest::from_bytes([0x5a; 32]),
                byte_len: 1,
                modified_unix_ms: 1,
            },
            ownership: CheckpointOwnership::Legacy,
            source_snapshot: None,
        },
    });
})];

#[cfg(target_arch = "wasm32")]
const NATIVE_DIALOG_GATE: &[DialogRow] = &[];

/// Each term of the dialog gate makes the application modal on its own, and
/// nothing does when every dialog is closed.
///
/// Every dialog that sets one of these is about to move to its own owner.
/// A term lost on the way would leave shortcuts live behind that dialog,
/// which is how a keystroke edits the design under a form the reader is
/// still filling in.
#[test]
fn each_term_of_the_dialog_gate_makes_the_application_modal_on_its_own() {
    assert!(
        !AppState::default().dialog_modal_open(),
        "with every dialog closed nothing is modal"
    );
    for (term, open) in DIALOG_GATE.iter().chain(NATIVE_DIALOG_GATE) {
        let mut dialogs = AppState::default();
        open(&mut dialogs);
        assert!(
            dialogs.dialog_modal_open(),
            "`{term}` alone makes the application modal"
        );
    }
}

/// What is drawn while it is open but is not a term of the gate today, so
/// a keystroke still reaches the drawing beneath it. This records the gate
/// as it stands rather than ruling on each; a change to any row is a
/// behaviour change and is made on purpose.
const OUTSIDE_THE_DIALOG_GATE: &[DialogRow] = &[
    ("waveform_calculator_dialog", |dialogs| {
        dialogs.results.waveform_calculator_dialog = true;
    }),
    ("expression_diagnostics_dialog", |dialogs| {
        dialogs.results.expression_diagnostics_dialog = true;
    }),
    ("descend_hierarchy", |dialogs| {
        dialogs.design.descend_hierarchy.open = true;
    }),
    ("move_selection", |dialogs| {
        dialogs.design.move_selection.armed = true
    }),
    ("stretch_selection", |dialogs| {
        dialogs.design.stretch_selection.armed = true;
    }),
    ("floating_paste", |dialogs| {
        dialogs.design.floating_paste = Some(FloatingPaste {
            objects: rspice_design::schematic::clipboard::ClipboardData::default(),
            start: crate::state::Point::origin(),
            grab: None,
            delta: crate::state::Point::origin(),
            view_path: String::new(),
            sheet: None,
            origin: FloatingPasteOrigin::Paste,
            preview: None,
        });
    }),
];

#[test]
fn what_is_drawn_outside_the_dialog_gate_leaves_the_application_modeless() {
    for (term, open) in OUTSIDE_THE_DIALOG_GATE {
        let mut dialogs = AppState::default();
        open(&mut dialogs);
        assert!(
            !dialogs.dialog_modal_open(),
            "`{term}` is not a term of the gate today"
        );
    }
}

#[test]
fn modeless_waveform_calculator_does_not_block_workspace_shortcuts() {
    let mut dialogs = AppState::default();
    dialogs.results.waveform_calculator_dialog = true;
    assert!(
        !dialogs.dialog_modal_open(),
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
    dialog.open_bus(
        &bus,
        edit_authority().source,
        "work/top/schematic".to_owned(),
    );

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
}
