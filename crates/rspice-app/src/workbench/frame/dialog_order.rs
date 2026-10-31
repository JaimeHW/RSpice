//! The order a frame draws its dialogs in: every step, bottom first, under
//! the slice that owns it.
//!
//! A dialog listed later sits above one listed earlier and takes the keyboard
//! from it, and a step that finishes what a dialog decided runs after that
//! dialog in the same frame. Each step draws what it owns as the shell drew
//! it before there was a list.

use crate::workbench::app::{command_palette, dialogs};
use crate::workbench::app_state::AppState;
use crate::workbench::features;
use crate::workbench::framework::dialogs::{DialogKey, DialogSpec, ModalGate, ModalSpec, Modality};
use crate::workbench::project::ProjectFeature;
use crate::workbench::{RSpiceApp, SurfaceId};

pub(in crate::workbench) const DIALOG_ORDER: &[DialogSpec<RSpiceApp, AppState>] = &[
    DialogSpec {
        key: DialogKey("shell.synchronize_route_owned_dialogs"),
        modality: Modality::None,
        is_open: None,
        render: |app, _| app.synchronize_route_owned_dialogs(),
    },
    DialogSpec {
        key: DialogKey("shell.synchronize_design_management_route"),
        modality: Modality::None,
        is_open: None,
        render: |app, _| {
            ProjectFeature::synchronize_design_management_route(&mut features::cx(app))
        },
    },
    DialogSpec {
        key: DialogKey("framework.confirmation"),
        modality: Modality::Dialog,
        is_open: Some(|state| {
            state.workbench.confirmation_dialog.visible
                || state.workbench.project_review_dialog.request.is_some()
        }),
        render: |app, ctx| dialogs::confirmation::render_confirmation_dialog(app, ctx),
    },
    DialogSpec {
        key: DialogKey("design.property"),
        modality: Modality::Application,
        is_open: Some(|state| state.design.tabbed_property_dialog.open),
        render: |app, ctx| {
            dialogs::property_dialog::render_property_dialog(ctx, &mut app.state);
        },
    },
    DialogSpec {
        key: DialogKey("models.pdk_settings"),
        modality: Modality::Application,
        is_open: Some(|state| features::dialog_open(state, DialogKey("models.pdk_settings"))),
        render: |app, ctx| features::render_dialog(app, ctx, DialogKey("models.pdk_settings")),
    },
    DialogSpec {
        key: DialogKey("settings.preferences"),
        modality: Modality::Dialog,
        is_open: Some(|state| features::dialog_open(state, DialogKey("settings.preferences"))),
        render: |app, ctx| features::render_dialog(app, ctx, DialogKey("settings.preferences")),
    },
    DialogSpec {
        key: DialogKey("cloud.license"),
        modality: Modality::Dialog,
        is_open: Some(|state| features::dialog_open(state, DialogKey("cloud.license"))),
        render: |app, ctx| features::render_dialog(app, ctx, DialogKey("cloud.license")),
    },
    DialogSpec {
        key: DialogKey("project.new_project"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.project.new_project.open),
        render: |app, ctx| app.render_new_project_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("cloud.publish_web"),
        modality: Modality::Dialog,
        is_open: Some(|state| features::dialog_open(state, DialogKey("cloud.publish_web"))),
        render: |app, ctx| features::render_dialog(app, ctx, DialogKey("cloud.publish_web")),
    },
    DialogSpec {
        key: DialogKey("cloud.live_session"),
        modality: Modality::Dialog,
        is_open: Some(|state| features::dialog_open(state, DialogKey("cloud.live_session"))),
        render: |app, ctx| features::render_dialog(app, ctx, DialogKey("cloud.live_session")),
    },
    DialogSpec {
        key: DialogKey("cloud.unpublish_web"),
        modality: Modality::Dialog,
        is_open: Some(|state| features::dialog_open(state, DialogKey("cloud.unpublish_web"))),
        render: |app, ctx| features::render_dialog(app, ctx, DialogKey("cloud.unpublish_web")),
    },
    DialogSpec {
        key: DialogKey("framework.command_palette"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.workbench.command_palette.open),
        render: |app, ctx| command_palette::render_command_palette(app, ctx),
    },
    DialogSpec {
        key: DialogKey("design.bus_tap"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.bus_tap.open),
        render: |app, ctx| app.render_bus_tap_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.builtin_xspice_placement"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.builtin_xspice_placement.open),
        render: |app, ctx| app.render_builtin_xspice_placement_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.net_label"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.net_label_placement.open),
        render: |app, ctx| app.render_net_label_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.grid_snap_routing"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.grid_snap_routing.open),
        render: |app, ctx| app.render_grid_snap_routing_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("sheets.drawing_sheet_overflow"),
        modality: Modality::Dialog,
        is_open: Some(|state| {
            features::dialog_open(state, DialogKey("sheets.drawing_sheet_overflow"))
        }),
        render: |app, ctx| {
            features::render_dialog(app, ctx, DialogKey("sheets.drawing_sheet_overflow"))
        },
    },
    DialogSpec {
        key: DialogKey("design.schematic_visibility"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.schematic_visibility_dialog.open),
        render: |app, ctx| app.render_schematic_visibility_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.descend_hierarchy"),
        modality: Modality::Modeless,
        is_open: None,
        render: |app, ctx| app.render_descend_hierarchy_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.engineering_table"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.engineering_table.open),
        render: |app, ctx| app.render_engineering_table_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.pin_port"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.pin_port.open),
        render: |app, ctx| app.render_pin_port_dialog(ctx),
    },
    // New comment hands input to the note editor in this same frame.
    DialogSpec {
        key: DialogKey("design.design_review_comments"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.design_review_comments.open),
        render: |app, ctx| app.render_design_review_comments_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.design_note"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.design_note.open),
        render: |app, ctx| app.render_design_note_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.documentation_shape"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.documentation_shape.open),
        render: |app, ctx| app.render_documentation_shape_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.array_selection"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.array_selection.open),
        render: |app, ctx| app.render_array_selection_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.replace_instance"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.replace_instance.open),
        render: |app, ctx| app.render_replace_instance_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.stimulus_link"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.stimulus_link.open),
        render: |app, ctx| dialogs::stimulus_link::render_stimulus_link_dialog(ctx, &mut app.state),
    },
    DialogSpec {
        key: DialogKey("design.create_hierarchy"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.create_hierarchy.open),
        render: |app, ctx| app.render_create_hierarchy_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("project.check_and_save"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.project.check_and_save.open),
        render: |app, ctx| app.render_check_and_save_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.connectivity_manager"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.connectivity_manager.open),
        render: |app, ctx| app.render_connectivity_manager_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("sheets.delete_sheet"),
        modality: Modality::Dialog,
        is_open: Some(|state| features::dialog_open(state, DialogKey("sheets.delete_sheet"))),
        render: |app, ctx| features::render_dialog(app, ctx, DialogKey("sheets.delete_sheet")),
    },
    DialogSpec {
        key: DialogKey("project.design_management"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.project.design_management.open),
        render: |app, ctx| app.render_design_management_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.selection_bulk_edit"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.selection_bulk_edit.open),
        render: |app, ctx| app.render_selection_bulk_edit_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("project.project_revision_history"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.project.project_revision_history.open),
        render: |app, ctx| app.render_project_revision_history_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("code.source_document"),
        modality: Modality::Modeless,
        is_open: None,
        render: |app, ctx| app.render_source_document_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("code.source_find"),
        modality: Modality::Modeless,
        is_open: None,
        render: |app, ctx| app.render_source_find_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("code.source_language_tools"),
        modality: Modality::Modeless,
        is_open: None,
        render: |app, ctx| app.render_source_language_tools_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.create_model_bound_symbol"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.create_model_bound_symbol.open),
        render: |app, ctx| app.render_create_model_bound_symbol_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.symbol_definition"),
        modality: Modality::Dialog,
        is_open: Some(|state| {
            state.design.symbol_import.open || state.design.symbol_parameter_form.open
        }),
        render: |app, ctx| app.render_symbol_definition_dialogs(ctx),
    },
    DialogSpec {
        key: DialogKey("project.configuration"),
        modality: Modality::Dialog,
        is_open: Some(|state| {
            state.project.configuration_dialog.is_open()
                || state.project.configuration_dialog.leaving.is_some()
                || state.project.configuration.pending_selection.is_some()
        }),
        render: |app, ctx| app.render_configuration_dialog(ctx),
    },
    // A new deck changes what blocks a run, so a run prepared under the
    // old one is not authorized under it.
    DialogSpec {
        key: DialogKey("design.rule_setup"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.rule_setup.open),
        render: |app, ctx| {
            if dialogs::rule_setup::render_rule_setup_dialog(ctx, &mut app.state) {
                app.invalidate_simulation_preflight();
            }
        },
    },
    DialogSpec {
        key: DialogKey("sheets.drawing_sheet_defaults"),
        modality: Modality::Dialog,
        is_open: Some(|state| {
            features::dialog_open(state, DialogKey("sheets.drawing_sheet_defaults"))
        }),
        render: |app, ctx| {
            features::render_dialog(app, ctx, DialogKey("sheets.drawing_sheet_defaults"))
        },
    },
    DialogSpec {
        key: DialogKey("sheets.drawing_sheet_presets"),
        modality: Modality::Dialog,
        is_open: Some(|state| {
            features::dialog_open(state, DialogKey("sheets.drawing_sheet_presets"))
        }),
        render: |app, ctx| {
            features::render_dialog(app, ctx, DialogKey("sheets.drawing_sheet_presets"))
        },
    },
    DialogSpec {
        key: DialogKey("sheets.drawing_sheet_support"),
        modality: Modality::Dialog,
        is_open: Some(|state| {
            features::dialog_open(state, DialogKey("sheets.drawing_sheet_support"))
        }),
        render: |app, ctx| {
            features::render_dialog(app, ctx, DialogKey("sheets.drawing_sheet_support"))
        },
    },
    DialogSpec {
        key: DialogKey("hardcopy.hardcopy"),
        modality: Modality::Dialog,
        is_open: Some(|state| features::dialog_open(state, DialogKey("hardcopy.hardcopy"))),
        render: |app, ctx| features::render_dialog(app, ctx, DialogKey("hardcopy.hardcopy")),
    },
    DialogSpec {
        key: DialogKey("design.object_properties"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.object_properties.open),
        render: |app, ctx| app.render_object_properties_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.rename_selection"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.rename_selection.open),
        render: |app, ctx| app.render_rename_selection_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.find_replace"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.find_replace.open),
        render: |app, ctx| app.render_find_replace_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("design.shared_properties"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.design.shared_properties.open),
        render: |app, ctx| app.render_shared_properties_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("framework.window_session"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.workbench.window_session_dialog.open),
        render: |app, ctx| dialogs::window_session::render_window_session_dialog(app, ctx),
    },
    DialogSpec {
        key: DialogKey("shell.help_center"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.shell.help_center.open),
        render: |app, ctx| app.render_help_center_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("shell.about"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.shell.about),
        render: |app, ctx| app.render_about_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("results.result_import"),
        modality: Modality::Application,
        is_open: Some(|state| features::dialog_open(state, DialogKey("results.result_import"))),
        render: |app, ctx| features::render_dialog(app, ctx, DialogKey("results.result_import")),
    },
    // The calculator is a modeless tool, not a modal: it renders beside
    // the dialog stack so the workspace stays live beneath it.
    DialogSpec {
        key: DialogKey("results.calculator"),
        modality: Modality::Modeless,
        is_open: None,
        render: |app, ctx| features::render_dialog(app, ctx, DialogKey("results.calculator")),
    },
    DialogSpec {
        key: DialogKey("results.expression_diagnostics"),
        modality: Modality::Modeless,
        is_open: None,
        render: |app, ctx| {
            features::render_dialog(app, ctx, DialogKey("results.expression_diagnostics"))
        },
    },
    DialogSpec {
        key: DialogKey("shell.shortcuts_help"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.shell.shortcuts_help),
        render: |app, ctx| app.render_shortcuts_help_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("models.model_browser"),
        modality: Modality::Application,
        is_open: Some(|state| features::dialog_open(state, DialogKey("models.model_browser"))),
        render: |app, ctx| features::render_dialog(app, ctx, DialogKey("models.model_browser")),
    },
    DialogSpec {
        key: DialogKey("project.new_cell"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.project.new_cell_dialog),
        render: |app, ctx| {
            ProjectFeature::process_new_cell_dialog(&mut features::cx(app), ctx);
            app.after_contribution();
        },
    },
    DialogSpec {
        key: DialogKey("project.new_view"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.project.new_view_dialog),
        render: |app, ctx| app.process_new_view_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("code.new_veriloga_device"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.code.new_veriloga_device.open),
        render: |app, ctx| app.render_new_veriloga_device_form(ctx),
    },
    DialogSpec {
        key: DialogKey("code.veriloga_import"),
        modality: Modality::Modeless,
        is_open: None,
        render: |app, ctx| app.render_veriloga_import_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("code.veriloga_replace"),
        modality: Modality::Modeless,
        is_open: None,
        render: |app, ctx| app.render_veriloga_replace_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("code.veriloga_build_settings"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.code.veriloga_build_settings.is_some()),
        render: |app, ctx| app.render_veriloga_build_settings(ctx),
    },
    DialogSpec {
        key: DialogKey("project.copy_cell"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.project.copy_cell_dialog),
        render: |app, ctx| ProjectFeature::process_copy_cell_dialog(&mut features::cx(app), ctx),
    },
    DialogSpec {
        key: DialogKey("project.rename_cell"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.project.rename_cell_dialog),
        render: |app, ctx| {
            ProjectFeature::process_rename_cell_dialog(&mut features::cx(app), ctx);
            app.after_contribution();
        },
    },
    DialogSpec {
        key: DialogKey("project.library_deletion_review"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.project.library_deletion_review.target.is_some()),
        render: |app, ctx| app.process_library_deletion_review_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("simulate.plan_removal_review"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.simulate.plan_removal_review.target.is_some()),
        render: |app, ctx| app.process_plan_removal_review_dialog(ctx),
    },
    DialogSpec {
        key: DialogKey("simulate.plan_removal_refusal"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.simulate.plan_removal_refusal.analysis.is_some()),
        render: |app, ctx| app.process_plan_removal_refusal_dialog(ctx),
    },
    #[cfg(not(target_arch = "wasm32"))]
    DialogSpec {
        key: DialogKey("services.autosave_restore"),
        modality: Modality::Dialog,
        is_open: Some(|state| state.platform.pending_autosave_restore.is_some()),
        render: |app, ctx| {
            crate::workbench::app_state::services_access::process_autosave_restore_dialog(
                &mut app.state,
                app.file_workflow_io.as_ref(),
                ctx,
            );
        },
    },
    DialogSpec {
        key: DialogKey("project.pending_library_deletions"),
        modality: Modality::None,
        is_open: None,
        render: |app, _| app.process_pending_library_deletions(),
    },
    DialogSpec {
        key: DialogKey("shell.exit_request"),
        modality: Modality::None,
        is_open: None,
        render: |app, ctx| app.process_exit_request(ctx),
    },
    // The route-owned overlays draw after the application dialogs, so a
    // workflow opened from Preferences is the top modal while the originating
    // dialog remains retained underneath and can be restored on Close.
    //
    // The simulation plan workflows are here for the same reason the account
    // and tool managers are: every one of them is opened by a command the
    // toolbar, the menus and the palette can reach from any workspace, so a
    // host that only runs on one surface would make the control dead
    // everywhere else.
    DialogSpec {
        key: DialogKey("cloud.account_organization"),
        modality: Modality::Application,
        is_open: Some(|state| {
            state.workbench.current_route().surface_id() == SurfaceId::AccountOrganization
        }),
        render: |app, ctx| {
            features::render_dialog(app, ctx, DialogKey("cloud.account_organization"));
        },
    },
    DialogSpec {
        key: DialogKey("simulate.jobs_manager"),
        modality: Modality::Application,
        is_open: Some(|state| {
            state.workbench.current_route().surface_id() == SurfaceId::JobsManager
        }),
        render: |app, ctx| crate::workbench::tools::jobs_manager::show(ctx, app),
    },
    DialogSpec {
        key: DialogKey("framework.specialist_tool_browser"),
        modality: Modality::Application,
        is_open: Some(|state| {
            state.workbench.current_route().surface_id() == SurfaceId::SpecialistToolBrowser
        }),
        render: |app, ctx| crate::workbench::tools::specialist_tool_browser::show(ctx, app),
    },
    DialogSpec {
        key: DialogKey("simulate.workflow_dialogs"),
        modality: Modality::Application,
        is_open: Some(|state| {
            state.simulate.simulation_workflow.is_some()
                || state.model.sim_setup.session.palette_open
        }),
        render: |app, ctx| crate::workbench::surfaces::show_simulation_workflow_dialogs(ctx, app),
    },
];

/// What holds a modal gate while it is open but is drawn by no step of
/// [`DIALOG_ORDER`], under the slice that hosts it. The order is not
/// behaviour: a gate is open while any of them is.
const MODAL_ONLY: &[ModalSpec<AppState>] = &[
    ModalSpec {
        key: DialogKey("project.new_library"),
        modality: Modality::Dialog,
        is_open: |state| state.project.new_library_dialog,
    },
    ModalSpec {
        key: DialogKey("project.rename_library"),
        modality: Modality::Dialog,
        is_open: |state| state.project.rename_library_dialog,
    },
    ModalSpec {
        key: DialogKey("project.delete_library"),
        modality: Modality::Dialog,
        is_open: |state| state.project.delete_library_dialog,
    },
    ModalSpec {
        key: DialogKey("project.rename_view"),
        modality: Modality::Dialog,
        is_open: |state| state.project.rename_view_dialog,
    },
    ModalSpec {
        key: DialogKey("project.technology_attachment"),
        modality: Modality::Dialog,
        is_open: |state| state.project.technology_attachment.open,
    },
    ModalSpec {
        key: DialogKey("results.dataset_export"),
        modality: Modality::Dialog,
        is_open: |state| features::dialog_open(state, DialogKey("results.dataset_export")),
    },
    ModalSpec {
        key: DialogKey("results.run_details"),
        modality: Modality::Dialog,
        is_open: |state| features::dialog_open(state, DialogKey("results.run_details")),
    },
    ModalSpec {
        key: DialogKey("project.launcher"),
        modality: Modality::Application,
        is_open: |state| state.project.project_launcher_open,
    },
    ModalSpec {
        key: DialogKey("project.launcher_route"),
        modality: Modality::Application,
        is_open: |state| state.workbench.current_route().surface_id() == SurfaceId::ProjectLauncher,
    },
    ModalSpec {
        key: DialogKey("project.examples"),
        modality: Modality::Application,
        is_open: |state| state.project.examples_selection.is_some(),
    },
    ModalSpec {
        key: DialogKey("project.design_management_route"),
        modality: Modality::Application,
        is_open: |state| {
            state.workbench.current_route().surface_id() == SurfaceId::DesignManagement
        },
    },
    ModalSpec {
        key: DialogKey("settings.preferences_route"),
        modality: Modality::Application,
        is_open: |state| state.workbench.current_route().surface_id() == SurfaceId::Preferences,
    },
    ModalSpec {
        key: DialogKey("framework.notification_center"),
        modality: Modality::Application,
        is_open: |state| state.workbench.notification_center_open,
    },
    ModalSpec {
        key: DialogKey("framework.notification_center_route"),
        modality: Modality::Application,
        is_open: |state| {
            state.workbench.current_route().surface_id() == SurfaceId::NotificationCenter
        },
    },
    // The browser draws its pending file operation under the confirmation
    // step, but only this gate counts it.
    #[cfg(target_arch = "wasm32")]
    ModalSpec {
        key: DialogKey("framework.browser_file_operation"),
        modality: Modality::Application,
        is_open: |state| {
            crate::workbench::workflows::project_workflow::browser_file_operation_label(
                &state.platform,
            )
            .is_some()
        },
    },
    ModalSpec {
        key: DialogKey("models.model_correlation"),
        modality: Modality::Application,
        is_open: |state| features::dialog_open(state, DialogKey("models.model_correlation")),
    },
    ModalSpec {
        key: DialogKey("models.models_view"),
        modality: Modality::Application,
        is_open: |state| features::dialog_open(state, DialogKey("models.models_view")),
    },
    ModalSpec {
        key: DialogKey("results.create_result_document"),
        modality: Modality::Application,
        is_open: |state| features::dialog_open(state, DialogKey("results.create_result_document")),
    },
    ModalSpec {
        key: DialogKey("results.result_document"),
        modality: Modality::Application,
        is_open: |state| features::dialog_open(state, DialogKey("results.result_document")),
    },
    ModalSpec {
        key: DialogKey("simulate.regression_baseline_picker"),
        modality: Modality::Application,
        is_open: |state| state.simulate.verification.regression_baseline_picker_open,
    },
    ModalSpec {
        key: DialogKey("simulate.tuning_review"),
        modality: Modality::Application,
        is_open: |state| state.simulate.verification.tuning_review_open,
    },
    // The Netlist document's own dialogs hold the gate only while its
    // workspace shows them.
    ModalSpec {
        key: DialogKey("code.netlist"),
        modality: Modality::Application,
        is_open: |state| {
            state.workbench.workspace == crate::workbench::state::Workspace::Netlist
                && state.code.netlist.application_modal_open()
        },
    },
];

const _: () = crate::workbench::framework::dialogs::check(DIALOG_ORDER, MODAL_ONLY);

impl ModalGate for AppState {
    type Host = RSpiceApp;
    const ORDER: &'static [DialogSpec<RSpiceApp, Self>] = DIALOG_ORDER;
    const MODAL_ONLY: &'static [ModalSpec<Self>] = MODAL_ONLY;
}

#[cfg(test)]
mod tests {
    use super::{AppState, DIALOG_ORDER, MODAL_ONLY, features};

    /// The order lists every dialog step a registered feature draws, under
    /// the key the feature draws it by and in the order that feature
    /// registers them, and lists no other step of that feature's slot. The
    /// feature registers the rest of what it draws after them: each a modal
    /// gate the modal-only list names, in the order that list names them.
    #[test]
    fn the_order_lists_exactly_the_steps_the_features_draw() {
        let registered = features::registered_dialogs();
        assert!(!registered.is_empty());
        for (feature_slot, _) in &registered {
            let drawn = registered
                .iter()
                .filter(|(slot, _)| slot == feature_slot)
                .map(|(_, key)| *key)
                .collect::<Vec<_>>();
            let listed = DIALOG_ORDER
                .iter()
                .map(|spec| spec.key)
                .filter(|key| {
                    key.0
                        .split_once('.')
                        .is_some_and(|(owner, _)| owner == *feature_slot)
                })
                .collect::<Vec<_>>();
            let gated = MODAL_ONLY
                .iter()
                .map(|spec| spec.key)
                .filter(|key| drawn.contains(key))
                .collect::<Vec<_>>();
            assert_eq!(
                [listed, gated].concat(),
                drawn,
                "the {feature_slot} feature's dialog steps"
            );
        }
        for (slot, key) in &registered {
            assert!(
                key.0
                    .strip_prefix(slot)
                    .is_some_and(|rest| rest.starts_with('.')),
                "`{}` is drawn by the feature in slot `{slot}`",
                key.0
            );
        }
    }

    /// Every dialog starts closed, so in a fresh session no step that says
    /// whether it is open says yes.
    #[test]
    fn no_dialog_step_is_open_in_a_fresh_session() {
        let state = AppState::default();
        for spec in DIALOG_ORDER {
            if let Some(is_open) = spec.is_open {
                assert!(
                    !is_open(&state),
                    "`{}` is open in a fresh session",
                    spec.key.0
                );
            }
        }
    }
}
