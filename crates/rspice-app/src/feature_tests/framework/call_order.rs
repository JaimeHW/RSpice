//! The order one frame runs its steps in, and the order the frame's dialogs
//! draw in, read back from the frame loop's shipped source.
//!
//! Both orders are behaviour. A dialog drawn later sits above one drawn
//! earlier and takes the keyboard from it; a request one step raises is
//! answered only by a step that runs after it in the same frame. Moving the
//! steps into hooks, tables or containers has to leave both orders exactly as
//! they are, so each list here is the order as it stands, and a test fails
//! the moment the source stops following it.

// Refactor scaffolding: deleted when the split completes.

const APP: &str = include_str!("../../workbench/app.rs");
const CLOUD: &str = include_str!("../../../../rspice-app-cloud/src/workbench/cloud.rs");
const CODE: &str = include_str!("../../workbench/code.rs");
const FRAME: &str = include_str!("../../workbench/frame.rs");
const DIALOG_ORDER: &str = include_str!("../../workbench/frame/dialog_order.rs");
const DISPATCH: &str = include_str!("../../workbench/dispatch.rs");
const COMMAND_HELPERS: &str = include_str!("../../workbench/commands/command_helpers.rs");
const CONFIGURATION_PAGE: &str = include_str!("../../workbench/surfaces/project/configuration.rs");
const EXPORT_IMAGE: &str = include_str!("../../workbench/app/actions/export_image.rs");
const SHELL_HOST: &str = include_str!("../../workbench/shell_host.rs");
const FEATURES: &str = include_str!("../../workbench/features.rs");
const MODELS: &str = include_str!("../../../../rspice-app-models/src/workbench/models.rs");
const PROJECT: &str = include_str!("../../workbench/project.rs");
const RESULTS: &str = include_str!("../../../../rspice-app-results/src/workbench/results.rs");
const SERIALIZATION: &str = include_str!("../../workbench/app_state/session/serialization.rs");
const STATUS_ORDER: &str = include_str!("../../workbench/shell_host/status_order.rs");
const PROJECT_TRANSACTIONS: &str = include_str!("../../workbench/session/project_transactions.rs");
const LIBRARY_SURFACE: &str = include_str!("../../workbench/surfaces/project/library.rs");
const LIBRARY_HOST: &str = include_str!("../../workbench/shell_host/project/library.rs");
const TECHNOLOGY_DIALOG: &str = include_str!("../../workbench/surfaces/project/technology.rs");

/// One step of a frame function: its number in the frame-order census, and
/// the calls that make it up, in the order they run. A call is matched with
/// its whitespace removed, so rewrapping a line never fails the order.
type Step = (&'static str, &'static [&'static str]);

/// `RSpiceApp::ui`, the frame eframe runs.
const UI: &[Step] = &[
    ("1", &["self.state.assert_project_open_fact()"]),
    ("2", &["FrameSimulationInput::capture(&self.state)"]),
    (
        "3",
        &["ui::viewport::capture_root_viewport(&ctx, ui.max_rect())"],
    ),
    (
        "4",
        &[
            "accessibility::spoken_feedback_override()",
            "options.screen_reader = self.state.settings.browser_spoken_feedback",
        ],
    ),
    ("5", &["self.prepare_frame(&ctx)"]),
    ("6", &["self.poll_cloud_account(&ctx)"]),
    ("7", &["self.record_publish_receipt()"]),
    ("8", &["self.pump_live_session()"]),
    (
        "9",
        &[
            "framework::requests::drain(self,",
            "ClipboardText(text)| ctx.copy_text(text)",
            "framework::requests::drain(self,",
            "ClipboardImage(image)| ctx.copy_image(image)",
        ],
    ),
    ("10", &["autosave::autosave_tick("]),
    ("11", &["self.render_frame_chrome(ui)"]),
    ("12", &["self.render_secondary_application_windows(&ctx)"]),
    ("13", &["self.refresh_live_checks(&ctx)"]),
    ("14", &["self.render_frame_dialogs(&ctx)"]),
    (
        "15",
        &["self.state.preferences_file_parts().persist_preferences_file(&ctx)"],
    ),
    ("16", &["self.end_frame(&simulation_input_before_frame)"]),
    (
        "17",
        &[
            "if !self.state.model.notices.is_empty()",
            "ctx.request_repaint()",
        ],
    ),
];

/// `RSpiceApp::prepare_frame`, step 5 of the frame.
const PREPARE_FRAME: &[Step] = &[
    ("5.1", &["self.state.drain_notices(ctx)"]),
    ("5.2", &["self.state.reconcile_schematic_drag(ctx, false)"]),
    ("5.3", &["ctx.set_zoom_factor(desired_zoom)"]),
    ("5.4", &["self.state.workbench.theme.apply(ctx)"]),
    ("5.5", &["if initializing_frame", "ctx.request_repaint()"]),
    ("5.6", &["self.poll_code(ctx)"]),
    ("5.13", &["self.poll_automation_workflow()"]),
    (
        "5.14",
        &["code_workspace::settle_pending_python_execute(&mut self.code_workspace_parts(),)"],
    ),
    ("5.15", &["self.poll_managed_automation_runtime()"]),
    (
        "5.16",
        &[
            "automation.execution.is_active()",
            "ctx.request_repaint_after(std::time::Duration::from_millis(50))",
        ],
    ),
    (
        "5.17",
        &["synchronize_browser_surface_navigation(&mut self.state)"],
    ),
    (
        "5.18",
        &[
            "self.state.shortcut_library_parts().poll_shortcut_library_persistence()",
            "self.state.take_shortcut_resolver_reset()",
        ],
    ),
    ("5.19", &["self.poll_browser_pdk_persistence(ctx)"]),
    ("5.20", &["self.poll_browser_pack_restore(ctx)"]),
    ("5.21", &["self.handle_shortcuts(ctx)"]),
    (
        "5.22",
        &["file_import::register_text_import_repaint_context(ctx)"],
    ),
    (
        "5.23",
        &["project_lifecycle::poll_browser_binding_restore("],
    ),
    (
        "5.24",
        &[
            "self.poll_browser_project_import()",
            "self.state.open_browser_project(open)",
            "self.restore_workspace_after_project_load()",
        ],
    ),
    (
        "5.25",
        &[
            "self.state.poll_browser_project_save()",
            "self.handle_save_continuation_event(event)",
        ],
    ),
    (
        "5.26",
        &[
            "file_workflow::poll_browser_schematic_import(",
            "self.state.open_browser_schematic(opened)",
            "self.state.results.analysis.clear_transient_specialized_viewer_data()",
        ],
    ),
    ("5.27", &["self.state.poll_browser_netlist_workflow()"]),
    (
        "5.28",
        &["result_import_workflow::poll_browser_result_dataset_import("],
    ),
    (
        "5.29",
        &["self.simulation_controller.update(&mut self.state, self.export_workflow_io.as_ref())"],
    ),
    (
        "5.29.1",
        &[".refresh_yield_evidence(&model.sim_setup, &model.workspace.content)"],
    ),
    ("5.30", &["accessibility::publish_workspace_context("]),
    (
        "5.31",
        &[
            "self.state.runs.preflight.checking.is_some()",
            "ctx.request_repaint_after(std::time::Duration::from_millis(50))",
        ],
    ),
    (
        "5.32",
        &[".set_active_dirty(self.state.model.schematic.session.is_dirty)"],
    ),
    ("5.33", &["self.sync_window_title(ctx)"]),
    ("5.34", &["self.handle_image_export()"]),
];

/// `workbench::show`, the chrome, docks and surface of step 11.
const SHOW: &[Step] = &[
    ("11.1", &["app.state.reconcile_schematic_drag(ctx, false)"]),
    ("11.2", &["app.state.apply_schematic_editing_preferences()"]),
    ("11.3", &["reconcile_platform_full_screen(app)"]),
    (
        "11.4",
        &["preflight::poll(&mut app.state, &mut app.simulation_controller)"],
    ),
    ("11.5", &["app.state.drain_notices(ctx)"]),
    ("11.6", &["announce_run_completion(ctx, app)"]),
    ("11.7", &["synchronize_activity_stream(ctx, app)"]),
    (
        "11.8",
        &[
            "pointer_is_coarse(ctx, app.state.workbench.coarse_pointer)",
            "LayoutSpec::resolve_for_shell(",
            "docks::synchronize_panel_memory(ctx, app, layout)",
        ],
    ),
    (
        "11.9",
        &["app.state.results.follow_preferences(&app.state.model.preferences)"],
    ),
    (
        "11.10",
        &[
            ".apply_responsive_metrics_with_target(",
            "style.animation_time =",
            "app.state.workbench.reconcile_drawer_mode(",
        ],
    ),
    (
        "11.11",
        &[
            "app.state.exit_full_screen_presentation()",
            "show_full_screen_presentation(root, app, layout)",
        ],
    ),
    ("11.12", &["chrome::status_bar::show(root, app, layout)"]),
    (
        "11.13",
        &["chrome::phone_navigation::show(root, app, layout)"],
    ),
    ("11.14", &["chrome::title_bar::show(root, app, layout)"]),
    ("11.15", &["chrome::toolbar::show(root, app, layout)"]),
    ("11.16", &["chrome::activity_rail::show(root, app)"]),
    ("11.17", &["docks::show_navigator(root, app, layout)"]),
    ("11.18", &["docks::show_inspector(root, app, layout)"]),
    ("11.19", &["chrome::document_bar::show(root, app, layout)"]),
    (
        "11.20",
        &["SheetsFeature::show_sheet_strip(&mut features::cx::<SheetsFeature>(app), root, layout)"],
    ),
    ("11.21", &["docks::show_console(root, app, layout)"]),
    (
        "11.22",
        &["CentralPanel::default()", "surfaces::show(ui, app)"],
    ),
    ("11.23", &["docks::show_drawers(root, app, layout)"]),
    (
        "11.24",
        &[
            "crate::workbench::project::open_project_launcher(",
            "shell_host::project::launcher::show(ctx, app)",
        ],
    ),
    (
        "11.25",
        &[
            "open_confirmed(&mut app.state)",
            "tools::examples_browser::show(ctx, &mut features::cx(app))",
            "request_example(&mut app.state, example)",
        ],
    ),
    (
        "11.26",
        &[
            "app.state.runs.preflight.take_run_and_queue_request()",
            "preflight::start(&mut app.state, &mut app.simulation_controller, true)",
        ],
    ),
    ("11.27", &["diagnostic_routes::apply_pending(ctx, app)"]),
    (
        "11.28",
        &["features::render_dialog(app, ctx, DialogKey(\"results.run_details\"))"],
    ),
    ("11.29", &["app.state.drain_notices(ctx)"]),
    (
        "11.30",
        &["tools::notification_center::show(ctx, app, layout.title_bar_height)"],
    ),
    ("11.31", &["answer_result_requests(ctx, app)"]),
    (
        "11.32",
        &["app.state.design.follow_workspace(app.state.workbench.workspace)"],
    ),
    (
        "11.33",
        &[
            "show_toasts(ctx, &mut app.state, layout, document.response.rect)",
            "ResultsFeature::perform_notification_action(&mut features::cx(app), action)",
            "app.after_contribution()",
        ],
    ),
    ("11.34", &["app.state.reconcile_schematic_drag(ctx, true)"]),
    ("11.35", &["apply_platform_full_screen_request(ctx, app)"]),
];

/// The frame's dialog steps as `render_frame_dialogs` wrote them out before
/// they became the dialog order: each step's call, as the source wrote it
/// less `self.` and the shell's own path, and the slice that owns it. Every
/// dialog the frame draws is drawn by one of these, bottom first. A step a
/// registered feature now draws is `features::render_dialog`, under the key
/// its spec names.
const DIALOG_STEPS: &[(&str, &str)] = &[
    ("synchronize_route_owned_dialogs", "shell"),
    (
        "ProjectFeature::synchronize_design_management_route",
        "shell",
    ),
    (
        "dialogs::confirmation::render_confirmation_dialog",
        "framework",
    ),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "models"),
    ("features::render_dialog", "settings"),
    ("features::render_dialog", "cloud"),
    ("features::render_dialog", "cloud"),
    ("features::render_dialog", "project"),
    ("features::render_dialog", "cloud"),
    ("features::render_dialog", "cloud"),
    ("features::render_dialog", "cloud"),
    ("command_palette::render_command_palette", "framework"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "sheets"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "project"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "sheets"),
    ("features::render_dialog", "project"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "project"),
    ("features::render_dialog", "code"),
    ("features::render_dialog", "code"),
    ("features::render_dialog", "code"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "project"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "sheets"),
    ("features::render_dialog", "sheets"),
    ("features::render_dialog", "sheets"),
    ("features::render_dialog", "hardcopy"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    ("features::render_dialog", "design"),
    (
        "dialogs::window_session::render_window_session_dialog",
        "framework",
    ),
    ("render_help_center_dialog", "shell"),
    ("render_about_dialog", "shell"),
    ("features::render_dialog", "results"),
    ("features::render_dialog", "results"),
    ("features::render_dialog", "results"),
    ("render_shortcuts_help_dialog", "shell"),
    ("features::render_dialog", "models"),
    ("features::render_dialog", "project"),
    ("features::render_dialog", "project"),
    ("features::render_dialog", "code"),
    ("features::render_dialog", "code"),
    ("features::render_dialog", "code"),
    ("features::render_dialog", "code"),
    ("features::render_dialog", "project"),
    ("features::render_dialog", "project"),
    ("features::render_dialog", "project"),
    ("SimulateFeature::render_plan_removal_review", "simulate"),
    ("SimulateFeature::render_plan_removal_refusal", "simulate"),
    (
        "app_state::services_access::process_autosave_restore_dialog",
        "services",
    ),
    ("features::render_dialog", "project"),
    ("process_exit_request", "shell"),
    ("show_route_overlays", "shell"),
];

/// The steps of [`DIALOG_STEPS`] that draw no dialog of their own: they keep
/// the routes and the dialogs they own in step, finish what a dialog decided,
/// or draw the route overlays.
const SYNTHETIC_DIALOG_STEPS: &[&str] = &[
    "synchronize_route_owned_dialogs",
    "ProjectFeature::synchronize_design_management_route",
    "process_exit_request",
    "show_route_overlays",
];

/// The steps a registered feature runs that draw no dialog of their own,
/// named by the key their spec draws them by, since each is drawn through
/// the registry like any feature dialog: the project feature's staged
/// deletion, which finishes what the deletion review decided.
const SYNTHETIC_FEATURE_STEPS: &[&str] = &["project.pending_library_deletions"];

/// What `show_route_overlays`, the last dialog step, drew, and the slice that
/// owns each: above every dialog, so a workflow opened from one of them is
/// the top modal. The dialog order lists each as a step of its own.
const ROUTE_OVERLAYS: &[(&str, &str)] = &[
    ("features::render_dialog", "cloud"),
    ("SimulateFeature::render_jobs_manager", "simulate"),
    ("SimulateFeature::render_workflow_dialogs", "simulate"),
];

/// `RSpiceApp::dispatch_feature_command`: a feature's command runs behind
/// the shared prologue, and what it raised is answered straight after it, in
/// the same dispatch.
const DISPATCH_FEATURE_COMMAND: &[Step] = &[
    ("1", &["held_back(flags, &self.state.platform).is_some()"]),
    (
        "2",
        &["crate::workbench::app::deny_read_only_edit(\
           &self.state.model,\
           &mut self.state.workbench,\
           )"],
    ),
    ("3", &["self.state.commit_pending_inspector_edit()"]),
    ("4", &["features::execute(self, spec.key)"]),
    ("5", &["self.after_contribution()"]),
];

/// `RSpiceApp::after_contribution`, in the order a feature's call has what
/// it raised answered. The netlist editor's commit, undo and redo go before
/// all of it, as when the editor made them while the page drew. What the
/// schematic canvas asked of the results workspace goes straight after, as
/// when the canvas wrote it in place. The commands the call ran next go next, so a report's Save is carried out
/// before anything else the call raised.
const AFTER_CONTRIBUTION: &[Step] = &[
    (
        "-1",
        &["|app, CommitSheetSection(ctx)| {\
           app.commit_inspector_section(InspectorScope::Document(Workspace::Design), &ctx);\
           }"],
    ),
    (
        "0",
        &["|app, request: NetlistEditorRequest| answer_netlist_editor_request(app, request)"],
    ),
    (
        "0.1",
        &["|app, probed: SelectProbedTrace| select_probed_trace(&mut app.state, probed)"],
    ),
    (
        "0.2",
        &["|app, request: DesignSurfaceRequest| answer_surface_request(&mut app.state, request)"],
    ),
    ("1", &["|app, Execute(key)| app.execute_command_key(key)"]),
    (
        "1.1",
        &["|app, PromptAfterCommands(text)| app.state.workbench.prompt(text)"],
    ),
    (
        "2",
        &[
            "|app, PrintActiveView|",
            "app.open_requested_hardcopy_workflow(HardcopyWorkflow::Print)",
        ],
    ),
    (
        "3",
        &[
            "|app, FollowSchematicGrid|",
            "state.design.follow_schematic_grid(&state.model)",
        ],
    ),
    ("4", &["self.state.take_shortcut_resolver_reset()"]),
    (
        "5",
        &[
            "|app, ApplyWaveformDefaults|",
            "state.results.start_waveform_viewer_session(&state.model.preferences)",
        ],
    ),
    (
        "6",
        &["|app, ExportDataset| file_action(app, FileMenuAction::ExportCsvWaveforms)"],
    ),
    (
        "7",
        &[
            "|app, SelectAnalysisInstance(instance)|",
            "crate::workbench::simulate::select_analysis_instance(",
            "&mut app.state.simulate,",
        ],
    ),
    (
        "8",
        &[
            "|app, RevealExecutedDeck { seq, close_manager_on_success, }|",
            "if open_task_deck(&mut app.state, seq) && close_manager_on_success {",
            "crate::workbench::simulate::close_jobs_manager(&mut app.state.workbench);",
        ],
    ),
    (
        "8.1",
        &[
            "|app, OpenPropertyEditor { component_id, occurrence, }|",
            "crate::workbench::app::actions::reveal::placed_instance(",
            "crate::workbench::app::open_property_editor(&mut app.state, component_id);",
        ],
    ),
    (
        "8.2",
        &[
            "|app, OpenStimulusDefinition(definition)|",
            "crate::workbench::app::show_stimulus_definition(\
             &mut app.state.design.selected_stimulus_definition,\
             &mut app.state.workbench,\
             &definition,\
             );",
        ],
    ),
    (
        "8.3",
        &[
            "|app, StageComponentTuning { component_id }|",
            "crate::workbench::simulate::SimulateFeature::stage_component_tuning(",
            "app.execute_command_key(ids::TUNING);",
        ],
    ),
    (
        "9",
        &[
            "|app, ShowWaveforms { visibility, selected, }|",
            "results.reveal_waveforms(visibility);",
            "results.session.selected_trace = selected;",
            "results.session.clear_cursors();",
        ],
    ),
    (
        "10",
        &[
            "|app, SetFamilyFilter(filter)|",
            "crate::workbench::documents::result_document::set_family_filter(",
        ],
    ),
    (
        "11",
        &[
            "|app, ShowSchematicAnnotations(annotations)|",
            "app.state.design.schematic_visibility.annotations = annotations;",
        ],
    ),
    (
        "12",
        &[
            "|app, MarkFailureSites(run_sequence)|",
            "if crate::workbench::app::highlight_active_failure_sites(\
             &mut app.state.model,\
             &mut app.state.workbench,\
             ) {",
            "app.state.results.results.marked_failure_run = run_sequence;",
        ],
    ),
    (
        "13",
        &["|app, CloseForLiveMirror| app.close_for_live_mirror()"],
    ),
    (
        "14",
        &[
            "|app, ArmPartPlacement(placement)|",
            "crate::workbench::app::arm_pack_part(&mut app.state.model, placement);",
        ],
    ),
    (
        "15",
        &[
            "|app, BindComponentModel { component_id, library, model, }|",
            "crate::workbench::design::inspector::bind_component_model_from_catalog(",
            "app.state.models.models_view.note_receipt(result);",
        ],
    ),
    (
        "16",
        &[
            "|app, request: OpenSymbolAuthoring| match request {",
            "crate::workbench::app::open_create_model_bound_symbol_dialog(\
             &mut app.state.design.create_model_bound_symbol,\
             &app.state.model,\
             &mut app.state.workbench,\
             app.state.models.selected_model.as_deref(),\
             );",
            "crate::workbench::app::open_symbol_parameter_form_dialog(\
             &mut app.state.design.symbol_parameter_form,\
             &app.state.model,\
             &mut app.state.workbench,\
             &app.state.platform,\
             );",
            "crate::workbench::app::open_symbol_parameter_form_dialog_for(\
             &mut app.state.design.symbol_parameter_form,\
             &app.state.model,\
             &mut app.state.workbench,\
             &app.state.platform,\
             target,\
             );",
            "crate::workbench::app::open_symbol_import_dialog(\
             &mut app.state.design.symbol_import,\
             &app.state.model,\
             &mut app.state.workbench,\
             &app.state.platform,\
             );",
            "crate::workbench::app::open_symbol_import_dialog_for(\
             &mut app.state.design.symbol_import,\
             &app.state.model,\
             &mut app.state.workbench,\
             &app.state.platform,\
             Some(&binding_source),\
             );",
            "crate::workbench::app::open_create_subcircuit_bound_symbol_dialog(",
            "app.state.models.models_view.note_receipt(result);",
        ],
    ),
    (
        "17",
        &[
            "|app, OpenInDesign(reference)|",
            "app.state.open_workspace_view(reference);",
            ".activate(crate::workbench::state::Workspace::Design);",
        ],
    ),
    (
        "18",
        &[
            "|app, ShowProjectModels|",
            "app.state.models.models_view.catalog_scope =",
            "app.execute_command_key(ids::MODELS_CATALOG);",
        ],
    ),
    (
        "19",
        &[
            "|app,",
            "ConfirmPackAttach {",
            "crate::workbench::models::ModelsFeature::confirm_pack_attach(",
            "&mut features::cx(app),",
            "app.execute_command_key(ids::MODELS_CATALOG);",
        ],
    ),
    (
        "20",
        &[
            "|app, RevealModel { library, model }|",
            "crate::workbench::models::ModelsFeature::reveal_model(",
            "&mut features::cx(app),",
        ],
    ),
    (
        "20.01",
        &["|app, ClearSelectedModel| app.state.models.clear_selected_model()"],
    ),
    (
        "20.02",
        &["|app, VoidVerilogAEvidence| answer_void_veriloga_evidence(app)"],
    ),
    (
        "20.1",
        &["|app, request: CodeRequest| answer_code_request(app, request)"],
    ),
    (
        "21",
        &["|app, request: DesignRequest| app.answer_design_request(request)"],
    ),
    (
        "22",
        &["|app, request: ProjectRequest| answer_project_request(app, request)"],
    ),
    (
        "23",
        &["|app, action: ProjectFileAction| run_project_file_action(app, action)"],
    ),
];

/// The shell's `contribute`, at the points that draw the results workspace
/// or a page that hands off into it: each has what it raised answered once
/// it has drawn, before the next point draws. The surface answers after
/// every workspace's page, the others after the results workspace's own.
const RESULTS_CONTRIBUTIONS: &[Step] = &[
    (
        "surface",
        &[
            "simulate::show(",
            "Workspace::Results => features::show_surface::<ResultsFeature>(app, ui),",
            "app.after_contribution();",
            "Point::SplitResults =>",
        ],
    ),
    (
        "split",
        &[
            "features::show_split::<ResultsFeature>(app, ui);\
             app.after_contribution();",
            "Point::Navigator(workspace) =>",
        ],
    ),
    (
        "navigator",
        &["Workspace::Results => {\
             features::show_navigator::<ResultsFeature>(app, ui);\
             app.after_contribution(); }"],
    ),
    (
        "inspector beside any workspace",
        &["Point::Inspector(InspectorScope::AnyWorkspace) => {\
             features::show_inspector::<ResultsFeature>(app, ui);\
             app.after_contribution(); }"],
    ),
    (
        "workspace inspector",
        &["Workspace::Results => {\
             features::show_inspector::<ResultsFeature>(app, ui);\
             app.after_contribution(); }"],
    ),
];

/// The shell's `contribute`, at the points the models feature draws in its
/// context. The workspace's surface has what it raised answered with every
/// workspace's page, once it has drawn. Its routes, navigator and inspector
/// answer nothing there, as none did when the shell drew them, so a request
/// another point left waits for the answer it waited for.
const MODELS_CONTRIBUTIONS: &[Step] = &[
    (
        "model editor route",
        &["== super::SurfaceId::ModelEditor\
           {\
           ModelsFeature::show_model_editor(&mut features::cx(app), ui);\
           return;\
           }"],
    ),
    (
        "measurement correlation route",
        &["== super::SurfaceId::ModelCorrelation\
           {\
           ModelsFeature::show_model_correlation(&mut features::cx(app), ui);\
           return;\
           }"],
    ),
    (
        "workspace surface",
        &[
            "Workspace::Models => features::show_surface::<ModelsFeature>(app, ui),",
            "app.after_contribution();",
            "Point::SplitResults =>",
        ],
    ),
    (
        "navigator",
        &[
            "Workspace::Verify => features::show_navigator::<SimulateFeature>(app, ui),\
           Workspace::Models => features::show_navigator::<ModelsFeature>(app, ui),\
           }",
        ],
    ),
    (
        "workspace inspector",
        &[
            "Workspace::Verify => features::show_inspector::<SimulateFeature>(app, ui),\
           Workspace::Models => features::show_inspector::<ModelsFeature>(app, ui),\
           Workspace::Netlist =>",
        ],
    ),
];

/// The shell's `contribute`, at the design docks that hand off into the
/// Models workspace: each has what it raised answered once it has drawn,
/// before the next point draws.
const DESIGN_CONTRIBUTIONS: &[Step] = &[
    (
        "navigator",
        &["Workspace::Design => {\
           let ctx = ui.ctx().clone();\
           features::show_navigator_reading::<DesignFeature>(app, ui, |_| {\
           design::row_menu_may_show(&ctx)\
           });\
           app.after_contribution();\
           }\
           Workspace::Results => {\
           features::show_navigator::<ResultsFeature>(app, ui);"],
    ),
    (
        "workspace inspector",
        &["Workspace::Design => {\
           design::prepare(&mut features::cx(app));\
           app.after_contribution();\
           features::show_inspector_modal::<DesignFeature>(app, ui, |_| true);\
           app.after_contribution();\
           if std::mem::take(&mut app.state.design.inspector_yields_to_sheet) {\
           app.contribute(\
           Point::Inspector(InspectorScope::Document(Workspace::Design)),\
           ui,\
           );\
           }\
           }\
           Workspace::Simulate => {"],
    ),
];

/// The shell's `contribute`, at the Design stage. While a deck owns the
/// project with no schematic drawn, its landing stands in for the document.
/// Otherwise the design feature draws the document in its context, then
/// code's Verilog-A tab draws a Verilog-A view's source under it, then the
/// touch strip draws over the canvas the document drew, in a lend of its
/// own taken where it draws.
const DESIGN_STAGE: &[Step] = &[
    (
        "netlist-first landing",
        &[
            "Workspace::Design if app.state.is_netlist_first_without_schematic() => {\
           design::show_netlist_first(ui, &mut app.state);\
           }",
        ],
    ),
    (
        "document",
        &["features::show_surface::<DesignFeature>(app, ui);"],
    ),
    (
        "Verilog-A source",
        &["code_adapters::show_in_design(ui, app);"],
    ),
    (
        "touch strip",
        &[
            "app.state.design.touch_strip_canvas.take()",
            "show_mobile_canvas_controls(",
            "features::surface_cx::<DesignFeature>(app,",
        ],
    ),
];

/// The shell's `contribute`, at the simulate feature's points. Both surfaces
/// draw in the feature's context and have the command they raised answered
/// once they have drawn, before the next point draws; the Simulate surface's
/// carried-over transactions land before the shell takes the answers it
/// reads. The navigator and the inspector draw in the feature's context on
/// both its workspaces: on Simulate each has the command it raised answered
/// once it has drawn; on Verify they raise nothing, so they answer nothing
/// where they draw. The toolbar's trailing group draws in the feature's
/// context with the dispatcher's answers, and has what it raised answered
/// once it has drawn.
const SIMULATE_CONTRIBUTIONS: &[Step] = &[
    (
        "workspace surface",
        &[
            "Workspace::Simulate => {\
           simulate::prepare(&mut features::cx(app));\
           let page = app.state.simulate.simulation_page;\
           simulate::show(\
           ui,\
           &mut features::cx_with_foreign_answers(app, |app| {\
           simulate::reads_foreign_answers(&app.state.model, page)\
           }),\
           );\
           }",
            "Workspace::Verify => verify::show(ui, &mut features::cx(app)),",
            "app.after_contribution();",
            "Point::SplitResults =>",
        ],
    ),
    (
        "navigator",
        &[
            "Workspace::Simulate => {\
           features::show_navigator::<SimulateFeature>(app, ui);\
           app.after_contribution();\
           }",
            "Workspace::Verify => features::show_navigator::<SimulateFeature>(app, ui),",
        ],
    ),
    (
        "workspace inspector",
        &[
            "Workspace::Simulate => {\
           features::show_inspector::<SimulateFeature>(app, ui);\
           app.after_contribution();\
           }",
            "Workspace::Verify => features::show_inspector::<SimulateFeature>(app, ui),",
        ],
    ),
    (
        "trailing",
        &["Point::Toolbar(ToolbarSlot::Trailing, layout) => {\
           crate::workbench::simulate::chrome::show_trailing(\
           &mut features::cx_with_foreign_answers(app, |_| true),\
           ui,\
           layout,\
           );\
           app.after_contribution();\
           }"],
    ),
];

/// The shell's `contribute`, at the code feature's points. The deck's page
/// prepares in the feature's context, then draws in it, holding the menu's
/// answers only while the source editor's menu is open or opening; the
/// Automation page does the same once its breakpoints have reached the
/// debugger. What either page raised is answered once it has drawn. The
/// navigator draws in the feature's context on the Netlist workspace and on a
/// Verilog-A view the Design workspace shows, and on both has what it raised
/// answered once it has drawn, before the next point draws. The inspector
/// draws in the feature's context on the same two: a Verilog-A view's raises
/// nothing, so it answers nothing where it draws; the Netlist inspector,
/// holding the dispatcher's answers while the deck's page is in front, has
/// the command it raised answered once it has drawn.
const CODE_CONTRIBUTIONS: &[Step] = &[
    (
        "netlist page",
        &[
            "netlist::prepare_workspace(&mut features::cx(app));",
            "let ctx = ui.ctx().clone();\
             netlist::show_prepared(\
             ui,\
             &mut features::cx_with_menu_answers(app, |app| {\
             netlist::source_menu_open(&ctx, &app.state.model, &app.state.code.netlist,)\
             }),\
             );",
            "sync_automation_debugger(&mut app.code_workspace_parts());\
             automation::prepare(&mut features::cx(app));\
             let ctx = ui.ctx().clone();\
             automation::show(\
             ui,\
             &mut features::cx_with_menu_answers(app, |app| {\
             automation::source_menu_open(&ctx, &app.state.model, &app.state.code.code_workspace,)\
             }),\
             ); } } } } app.after_contribution();",
        ],
    ),
    (
        "navigator",
        &[
            "Workspace::Netlist => {\
             features::show_navigator::<CodeFeature>(app, ui);\
             app.after_contribution();\
             }",
            "== ViewType::VerilogA =>\
             {\
             features::show_navigator::<CodeFeature>(app, ui);\
             app.after_contribution();\
             }",
        ],
    ),
    (
        "workspace inspector",
        &[
            "== crate::state::ViewType::VerilogA =>\
         {\
         features::show_inspector::<CodeFeature>(app, ui);\
         }",
            "Workspace::Netlist => {\
         features::show_inspector_reading::<CodeFeature>(app, ui, |app| {\
         app.state.code.code_workspace.page\
         == crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist\
         });\
         app.after_contribution();\
         }",
        ],
    ),
];

/// The shell's `contribute`, at the project feature's points: the open
/// workspace's chrome, the navigator and the inspector draw in the feature's
/// context and the toolbars over the workbench host. The chrome has what it
/// raised answered before the page in front draws, in the one fill the two
/// share, and the page has its own answered as it returns; the inspector has
/// what it raised answered once it has drawn, before the next point draws.
/// The navigator and the toolbars raise nothing, so they answer nothing
/// where they draw. The Library and cellview manager's route has what it
/// raised, a symbol dialog to open, answered as it returns, so the dialog
/// draws in the frame that asked for it.
const PROJECT_CONTRIBUTIONS: &[Step] = &[
    (
        "library and cellview manager route",
        &["== super::SurfaceId::LibraryCellviewManager\
           {\
           library_cellview_manager::show(ui, app);\
           app.after_contribution();\
           return;\
           }"],
    ),
    (
        "workspace surface",
        &[
            "Workspace::Project => {",
            "egui::Frame::new().fill(fill).show(ui, |ui| {",
            "ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);\
           project::chrome(&mut features::cx_with_foreign_answers(app, |_| true), ui,);\
           app.after_contribution();\
           self::project::page(ui, app);\
           app.after_contribution();\
           });",
            "project::show_technology_attachment_dialog(&ctx, &mut features::cx(app));\
           }",
        ],
    ),
    (
        "navigator",
        &[
            "Workspace::Project => features::show_navigator::<ProjectFeature>(app, ui),\
           Workspace::Simulate => {",
        ],
    ),
    (
        "workspace inspector",
        &["match workspace {\
           Workspace::Project => {\
           features::show_inspector::<ProjectFeature>(app, ui);\
           app.after_contribution();\
           }\
           Workspace::Design"],
    ),
    (
        "no-project toolbar",
        &[
            "ToolbarScope::NoProject => no_project_tools(ui, app, layout),\
           ToolbarScope::Surface(SurfaceOwner::ReportAuthoring) =>",
        ],
    ),
    (
        "workspace toolbar",
        &["ToolbarScope::Workspace(workspace) => match workspace {\
           Workspace::Project => project_tools(ui, app, layout),"],
    ),
];

/// `answer_result_requests`, step 11.31 of the frame: the export requests
/// the result documents raised, then Export dataset…, drawn between the
/// request that opens it and the plan hop.
const ANSWER_RESULT_REQUESTS: &[Step] = &[
    (
        "1",
        &[
            "|app, ExportResultQuantities(keys)|",
            "action_export_result_selection_with_io(",
        ],
    ),
    (
        "2",
        &[
            "|app, ExportCsv|",
            "open_dataset_export_picker(&mut app.state)",
        ],
    ),
    (
        "3",
        &["features::render_dialog(app, ctx, DialogKey(\"results.dataset_export\"))"],
    ),
    (
        "4",
        &[
            "|app, OpenProducingPlan|",
            "app.execute_command_key(ResultsCommand::OpenProducingPlan.key())",
        ],
    ),
];

/// The results feature's surface: the Results stage, then the result
/// document's two forms, the last things the stage draws.
const RESULTS_SURFACE: &[Step] = &[
    ("stage", &["surface::show("]),
    (
        "forms",
        &[
            "render_create_result_document(cx, ui.ctx())",
            "render_result_document_dialog(cx, ui.ctx())",
        ],
    ),
];

/// `RSpiceApp::handle_image_export`, the last step of the frame's
/// preparation: the figure a result document asked to export while it was
/// drawn opens the hardcopy feature's export workflow, in its context.
const HANDLE_IMAGE_EXPORT: &[Step] = &[(
    "1",
    &[
        "|app, ExportFigure|",
        "app.open_requested_hardcopy_workflow(",
        "HardcopyWorkflow::Export",
    ],
)];

/// `CloudFeature::poll_account`, step 6 of the frame: what the account
/// service settled since the last frame, taken in this order.
const CLOUD_ACCOUNT_POLL: &[Step] = &[
    ("6.1", &["account.poll_for_lost_session()"]),
    ("6.2", &["account.take_local_error()"]),
    ("6.3", &["account.take_storage_failure()"]),
    ("6.4", &["account.take_browser_request()"]),
];

/// `CodeFeature::poll`, step 5.6 of the frame: what the code feature settles
/// before any surface draws, in the order the frame settled it.
const CODE_POLL: &[Step] = &[
    ("5.6.1", &["Self::terminate_after_project_change(cx)"]),
    ("5.6.2", &["cx.own.automation_runtime.poll_discovery()"]),
    (
        "5.6.3",
        &["poll_veriloga_import(cx.model, &mut cx.state.code_workspace, cx.wb)"],
    ),
    (
        "5.6.4",
        &["poll_veriloga_compile(cx.model, &mut cx.state.code_workspace, cx.wb)"],
    ),
    (
        "5.6.5",
        &[
            "let shown = cx.state.netlist_shown()",
            "consume_veriloga_compile_request(cx.model, &mut cx.state.code_workspace, shown, ctx)",
        ],
    ),
    ("5.6.6", &["refresh_symbol_standing("]),
    ("5.6.7", &["poll_automation_source_import("]),
];

/// `pump_live_session`, step 8 of the frame: the live-session engine's
/// phases, with the project transactions they ask for between them where the
/// engine ran them itself.
const LIVE_SESSION_PUMP: &[Step] = &[
    (
        "1",
        &[
            "live_session.reconcile_role(&mut live_host(state), account)",
            "close_project_discard(state, ProjectCloseDestination::EmptyWorkbench)",
            "live_session.mirror_retired(closed)",
        ],
    ),
    (
        "2",
        &[
            "live_session.mirror_discard_due(&live_host(state))",
            "close_project_discard(state, ProjectCloseDestination::EmptyWorkbench)",
            "live_session.mirror_discard_settled(closed)",
        ],
    ),
    (
        "3",
        &["if !live_session.pump_inbound(&mut live_host(state), account) {"],
    ),
    (
        "4",
        &[
            "requests::drain(",
            "|state, AdoptLiveOwnedSource(document)|",
            "adopt_live_owned_source(&mut state.code.netlist, document)",
        ],
    ),
    (
        "5",
        &[
            "live_session.take_mirror_snapshot()",
            "state.apply_live_project_snapshot(snapshot.bytes(), snapshot.host_label())",
            "live_session.mirror_snapshot_applied(&mut live_host(state), snapshot, outcome)",
        ],
    ),
    ("6", &["live_session.pump_outbound(&mut live_host(state))"]),
];

/// `end_frame`, step 16 of the frame: the models feature's authoring
/// recovery point is captured once the frame's edits and cross-probes have
/// settled, and before a project the frame replaced takes its automation
/// authority with it.
const END_FRAME: &[Step] = &[
    (
        "16.1",
        &["self.invalidate_simulation_preflight_after_frame_edit(simulation_input_before_frame)"],
    ),
    (
        "16.2",
        &[
            "crate::workbench::synchronize_schematic_cross_probe(",
            "project_cross_probe_trace(projection.released_trace, projection.trace)",
            "project_cross_probe_line(projection.released_line, projection.line)",
        ],
    ),
    ("16.3", &["self.state.models.capture_authoring_recovery()"]),
    (
        "16.4",
        &["self.terminate_code_runtime_after_project_change()"],
    ),
];

/// `sync_window_title`, step 5.33 of the frame: the title's dirty mark and
/// the browser's leave-page guard read the models feature's unsaved
/// authoring beside the project's own unsaved changes.
const SYNC_WINDOW_TITLE: &[Step] = &[
    (
        "5.33.1",
        &["project_lifecycle::has_unsaved_changes(&self.state)"],
    ),
    (
        "5.33.2",
        &["ModelsPort::has_unsaved_authoring_changes(&*self.state.models)"],
    ),
    (
        "5.33.3",
        &[
            "should_warn_before_browser_unload(",
            "project_has_unsaved_changes,",
            "authoring_has_unsaved_changes,",
        ],
    ),
    (
        "5.33.4",
        &["if project_has_unsaved_changes || authoring_has_unsaved_changes"],
    ),
];

/// eframe's `save`: the models feature's authoring recovery point is
/// captured after the active schematic is synced into the workspace, so the
/// session stored after both carries both.
const SAVE: &[Step] = &[
    (
        "1",
        &["self.state.preferences_file_parts().flush_preferences_file()"],
    ),
    (
        "2",
        &[
            "if !should_persist_application_state(&self.state)",
            "return;",
        ],
    ),
    ("3", &["self.state.sync_active_schematic_to_workspace()"]),
    ("4", &["self.state.models.capture_authoring_recovery()"]),
    ("5", &["save_global_veriloga_library("]),
    ("6", &["self.state.store_session(storage)"]),
];

/// `RSpiceApp::new`: once the session's project is open, the browser's
/// stores start reading in this order, the models feature's PDK and pack
/// stores after the surface navigation and before the license backfill.
const STARTUP: &[Step] = &[
    ("1", &["state.open_session_project()"]),
    (
        "2",
        &["initialize_browser_surface_navigation(&mut state, &cc.egui_ctx)"],
    ),
    (
        "3",
        &["pdk_persistence::initialize_browser_pdk_persistence(&cc.egui_ctx)"],
    ),
    (
        "4",
        &["pack_persistence::initialize_browser_pack_restore(&cc.egui_ctx)"],
    ),
    ("5", &["state.cloud.restore_stored_license()"]),
];

/// The session's deserialization, once its stores are restored: the
/// restore's warnings are logged, then the models feature reopens the model
/// editor's unsaved draft, then the active schematic is saved into the
/// workspace.
const SESSION_RESTORE: &[Step] = &[
    (
        "1",
        &["state.record(LogSeverity::Warning, LogSource::Studio, warning);"],
    ),
    (
        "2",
        &["state.models.reopen_authoring_recovery(&mut state.model);"],
    ),
    (
        "3",
        &[
            ".save_active_schematic(&state.model.schematic);",
            "Ok(state)",
        ],
    ),
];

/// The body of the one function `signature` opens in `source`'s shipped half,
/// without its comment lines.
fn body(source: &str, signature: &str) -> String {
    let shipped = crate::source_guard::production_source(source)
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    let starts = shipped
        .match_indices(signature)
        .map(|(at, _)| at)
        .collect::<Vec<_>>();
    assert_eq!(
        starts.len(),
        1,
        "`{signature}` opens exactly one function in the shipped source"
    );
    let open = starts[0]
        + shipped[starts[0]..]
            .find('{')
            .unwrap_or_else(|| panic!("`{signature}` has a body"));
    let mut depth = 0_usize;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, character) in shipped[open..].char_indices() {
        if in_string {
            match character {
                _ if escaped => escaped = false,
                '\\' => escaped = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }
        match character {
            '"' => in_string = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return shipped[open + 1..open + offset].to_owned();
                }
            }
            _ => {}
        }
    }
    panic!("the braces of `{signature}` never close")
}

fn without_whitespace(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Every call of `steps` is found in `body`, each after the one before it.
fn assert_runs_in_order(function: &str, body: &str, steps: &[Step]) {
    let body = without_whitespace(body);
    let mut from = 0;
    let mut previous = "the start of the function".to_owned();
    for (step, calls) in steps {
        for call in *calls {
            let call = without_whitespace(call);
            let Some(at) = body[from..].find(&call) else {
                panic!(
                    "`{function}` step {step}: `{call}` is gone, or no longer runs \
                     after {previous}"
                );
            };
            from += at + call.len();
            previous = format!("step {step}");
        }
    }
}

/// The calls a function body makes at the top level, in order: each call
/// whose parentheses are not inside another call's, as the source spells its
/// callee, less `self.` (or the `app.` a dialog spec is handed) and the
/// shell's own path. A step's arguments and the closures it is handed are not
/// steps.
fn top_level_calls(body: &str) -> Vec<String> {
    let code = body
        .lines()
        .filter(|line| !line.trim_start().starts_with("#["))
        .collect::<Vec<_>>()
        .join("\n");
    let is_path = |c: &char| c.is_alphanumeric() || matches!(c, '_' | ':' | '.');
    let mut calls = Vec::new();
    let mut depth = 0_usize;
    let mut in_string = false;
    let mut escaped = false;
    for (at, character) in code.char_indices() {
        if in_string {
            match character {
                _ if escaped => escaped = false,
                '\\' => escaped = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }
        match character {
            '"' => in_string = true,
            '(' => {
                if depth == 0 {
                    let mut callee = code[..at]
                        .chars()
                        .rev()
                        .take_while(is_path)
                        .collect::<Vec<_>>();
                    callee.reverse();
                    let callee = callee.into_iter().collect::<String>();
                    let callee = callee
                        .strip_prefix("self.")
                        .or_else(|| callee.strip_prefix("app."))
                        .unwrap_or(&callee);
                    let callee = callee.strip_prefix("crate::workbench::").unwrap_or(callee);
                    calls.push(callee.to_owned());
                }
                depth += 1;
            }
            ')' => depth -= 1,
            _ => {}
        }
    }
    calls
}

/// `found` is `expected` call for call; the first difference is the report.
fn assert_calls(function: &str, found: &[String], expected: &[(&str, &str)]) {
    for (index, (call, owner)) in expected.iter().enumerate() {
        match found.get(index) {
            Some(found) if found == call => {}
            Some(found) => panic!(
                "`{function}` step {}: expected `{call}` ({owner}), found `{found}`",
                index + 1
            ),
            None => panic!(
                "`{function}` ends after {} steps; step {} was `{call}` ({owner})",
                found.len(),
                index + 1
            ),
        }
    }
    assert_eq!(
        found.len(),
        expected.len(),
        "`{function}` has steps after `{}`: {:?}",
        expected.last().map_or("", |(call, _)| call),
        &found[expected.len().min(found.len())..]
    );
}

/// One spec of the dialog order, as its source writes it.
struct WrittenSpec {
    key: String,
    /// The `Modality` variant it names.
    modality: String,
    /// The calls its `render` makes at the top level.
    renders: Vec<String>,
}

/// Each spec of the dialog order in `source`, in order.
fn dialog_specs(source: &str) -> Vec<WrittenSpec> {
    let shipped = crate::source_guard::production_source(source)
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    let order = shipped
        .split_once("DIALOG_ORDER")
        .and_then(|(_, rest)| rest.split_once("];"))
        .map(|(order, _)| order)
        .expect("the dialog order is one list");
    let field = |spec: &str, opens: &str, closes: char| {
        spec.split_once(opens)
            .and_then(|(_, rest)| rest.split_once(closes))
            .map(|(value, _)| value.to_owned())
            .unwrap_or_else(|| panic!("a dialog spec has no `{opens}`: {spec}"))
    };
    order
        .split("DialogSpec {")
        .skip(1)
        .map(|spec| {
            let (_, render) = spec
                .split_once("render:")
                .expect("every dialog spec renders");
            WrittenSpec {
                key: field(spec, "DialogKey(\"", '"'),
                modality: field(spec, "modality: Modality::", ','),
                // Answering what a step raised, straight after it, draws
                // nothing: it is the step's own drain, not a step.
                renders: top_level_calls(render)
                    .into_iter()
                    .filter(|call| call != "after_contribution")
                    .collect(),
            }
        })
        .collect()
}

#[test]
fn the_frame_runs_its_top_level_steps_in_todays_order() {
    assert_runs_in_order("RSpiceApp::ui", &body(APP, "fn ui("), UI);
    assert_eq!(
        top_level_calls(&body(APP, "fn render_frame_chrome(")),
        ["show"],
        "the frame's chrome step draws the workbench frame and nothing else"
    );
}

#[test]
fn the_live_session_runs_its_phases_and_project_transactions_in_todays_order() {
    let pump = body(
        PROJECT_TRANSACTIONS,
        "pub(crate) fn pump_live_session(state",
    );
    assert_runs_in_order("pump_live_session", &pump, LIVE_SESSION_PUMP);
    assert_eq!(
        pump.matches("requests::drain(").count(),
        1,
        "the live session answers only the netlist source adoption in its pump"
    );
}

#[test]
fn the_frame_prepares_itself_in_todays_order() {
    assert_runs_in_order(
        "RSpiceApp::prepare_frame",
        &body(APP, "fn prepare_frame("),
        PREPARE_FRAME,
    );
}

#[test]
fn the_workbench_frame_draws_in_todays_order() {
    assert_runs_in_order("workbench::show", &body(FRAME, "pub fn show("), SHOW);
}

#[test]
fn the_frame_dialogs_draw_in_todays_order() {
    let walk = body(APP, "fn render_frame_dialogs(");
    assert_eq!(
        top_level_calls(&walk),
        ["framework::dialogs::render"],
        "the frame's dialogs are one walk and nothing else"
    );
    assert!(
        without_whitespace(&walk).contains("frame::dialog_order::DIALOG_ORDER"),
        "the frame walks the shell's dialog order"
    );

    let recorded = DIALOG_STEPS
        .iter()
        .flat_map(|step| {
            if step.0 == "show_route_overlays" {
                ROUTE_OVERLAYS
            } else {
                std::slice::from_ref(step)
            }
        })
        .copied()
        .collect::<Vec<_>>();
    assert_eq!(
        recorded.len(),
        75,
        "the frame's 73 dialog steps, the route overlays' 3 owners in place of their host"
    );

    let specs = dialog_specs(DIALOG_ORDER);
    let calls = specs
        .iter()
        .flat_map(|spec| spec.renders.iter().cloned())
        .collect::<Vec<_>>();
    assert_calls("DIALOG_ORDER", &calls, &recorded);

    let mut drawn = 0;
    for WrittenSpec {
        key,
        modality,
        renders,
    } in &specs
    {
        let (owner, _) = key
            .split_once('.')
            .unwrap_or_else(|| panic!("`{key}` names the slice that owns it"));
        assert!(!renders.is_empty(), "`{key}` draws a step of the frame");
        let (first, recorded_owner) = recorded[drawn];
        assert_eq!(
            owner, recorded_owner,
            "`{key}` draws `{first}`, which {recorded_owner} owns"
        );
        for handed_to in renders.iter().skip(1) {
            assert!(
                SYNTHETIC_DIALOG_STEPS.contains(&handed_to.as_str()),
                "`{key}` hands its outcome on in the same frame only to a step that draws no \
                 dialog, not to `{handed_to}`"
            );
        }
        assert_eq!(
            modality == "None",
            SYNTHETIC_DIALOG_STEPS.contains(&first)
                || SYNTHETIC_FEATURE_STEPS.contains(&key.as_str()),
            "`{key}` is a step that draws no dialog exactly when `{first}` is one"
        );
        drawn += renders.len();
    }

    for synthetic in SYNTHETIC_DIALOG_STEPS {
        assert!(
            DIALOG_STEPS.iter().any(|(call, _)| call == synthetic),
            "`{synthetic}` is a step of the frame's dialogs"
        );
    }
    for synthetic in SYNTHETIC_FEATURE_STEPS {
        assert!(
            specs.iter().any(|spec| spec.key == *synthetic),
            "`{synthetic}` is a step of the frame's dialogs"
        );
    }
    assert_eq!(
        DIALOG_STEPS.len() - SYNTHETIC_DIALOG_STEPS.len() - SYNTHETIC_FEATURE_STEPS.len(),
        68,
        "the frame draws 68 dialog steps beside its synthetic ones"
    );
}

#[test]
fn a_feature_command_has_what_it_raised_answered_in_the_same_dispatch() {
    let dispatch = body(DISPATCH, "fn dispatch_feature_command(");
    assert_runs_in_order(
        "RSpiceApp::dispatch_feature_command",
        &dispatch,
        DISPATCH_FEATURE_COMMAND,
    );
    assert!(
        without_whitespace(&dispatch).ends_with(&without_whitespace("self.after_contribution();")),
        "nothing runs after a feature's command has what it raised answered"
    );

    let after = body(DISPATCH, "pub(crate) fn after_contribution(");
    assert_runs_in_order("RSpiceApp::after_contribution", &after, AFTER_CONTRIBUTION);
    assert_eq!(
        after.matches("requests::drain(").count(),
        33,
        "`after_contribution` answers exactly the thirty-three requests it lists"
    );
}

#[test]
fn another_part_opens_a_hardcopy_workflow_in_the_feature_context_where_it_did() {
    let open = body(DISPATCH, "fn open_requested_hardcopy_workflow(");
    assert_eq!(
        without_whitespace(&open),
        without_whitespace("hardcopy::open_workflow(&mut features::cx(self), workflow);"),
        "a request another part raised opens the workflow in the hardcopy feature's context"
    );

    let pump = body(EXPORT_IMAGE, "fn handle_image_export(");
    assert_runs_in_order("RSpiceApp::handle_image_export", &pump, HANDLE_IMAGE_EXPORT);
    assert_eq!(
        pump.matches("requests::drain(").count(),
        1,
        "the frame's preparation answers only the figure export there"
    );
}

#[test]
fn the_cloud_feature_takes_its_account_and_publish_news_where_the_frame_did() {
    for (signature, call) in [
        (
            "fn poll_cloud_account(",
            "CloudFeature::poll_account(&mut features::cx(self), ctx);",
        ),
        (
            "fn record_publish_receipt(",
            "CloudFeature::record_publish_receipt(&mut features::cx(self));",
        ),
    ] {
        assert_eq!(
            without_whitespace(&body(FRAME, signature)),
            without_whitespace(call),
            "the frame's `{signature}` step runs in the cloud feature's context, and only that"
        );
    }
    assert_runs_in_order(
        "CloudFeature::poll_account",
        &body(CLOUD, "fn poll_account("),
        CLOUD_ACCOUNT_POLL,
    );
}

#[test]
fn the_code_feature_settles_its_frame_where_the_frame_did() {
    for (signature, call) in [
        (
            "fn poll_code(",
            "CodeFeature::poll(&mut features::cx(self), ctx);",
        ),
        (
            "fn terminate_code_runtime_after_project_change(",
            "CodeFeature::terminate_after_project_change(&mut features::cx(self));",
        ),
        (
            "fn terminate_code_runtime(",
            "CodeFeature::terminate_runtime(&mut features::cx(self));",
        ),
    ] {
        assert_eq!(
            without_whitespace(&body(FRAME, signature)),
            without_whitespace(call),
            "the frame's `{signature}` step runs in the code feature's context, and only that"
        );
    }
    assert_runs_in_order("CodeFeature::poll", &body(CODE, "fn poll("), CODE_POLL);
}

#[test]
fn a_results_contribution_has_what_it_raised_answered_once_it_draws() {
    assert_runs_in_order(
        "RSpiceApp::contribute",
        &body(SHELL_HOST, "fn contribute("),
        RESULTS_CONTRIBUTIONS,
    );
}

#[test]
fn a_models_contribution_answers_nothing_where_it_draws() {
    assert_runs_in_order(
        "RSpiceApp::contribute",
        &body(SHELL_HOST, "fn contribute("),
        MODELS_CONTRIBUTIONS,
    );
}

#[test]
fn a_design_dock_has_what_it_raised_answered_once_it_draws() {
    assert_runs_in_order(
        "RSpiceApp::contribute",
        &body(SHELL_HOST, "fn contribute("),
        DESIGN_CONTRIBUTIONS,
    );
}

#[test]
fn the_design_stage_draws_the_document_then_code_s_source_then_the_strip() {
    assert_runs_in_order(
        "RSpiceApp::contribute",
        &body(SHELL_HOST, "fn contribute("),
        DESIGN_STAGE,
    );
}

#[test]
fn a_simulate_surface_or_dock_has_the_command_it_raised_answered_once_it_draws() {
    assert_runs_in_order(
        "RSpiceApp::contribute",
        &body(SHELL_HOST, "fn contribute("),
        SIMULATE_CONTRIBUTIONS,
    );
}

#[test]
fn a_code_dock_has_what_it_raised_answered_once_it_draws() {
    assert_runs_in_order(
        "RSpiceApp::contribute",
        &body(SHELL_HOST, "fn contribute("),
        CODE_CONTRIBUTIONS,
    );
}

#[test]
fn of_the_project_contributions_the_chrome_the_page_and_the_inspector_answer_what_they_raised() {
    assert_runs_in_order(
        "RSpiceApp::contribute",
        &body(SHELL_HOST, "fn contribute("),
        PROJECT_CONTRIBUTIONS,
    );
}

/// Where a configuration the Project inspector used for runs is refused, the
/// order from the inspector's drawing to the Configuration page's refusal
/// strip: the docked inspector draws before the stage, the project feature
/// commits the configuration it raised in the inspector's own step and writes
/// the refusal there, and the stage's Configuration page states it. A docked
/// inspector's refusal is stated in the frame its button was pressed in; an
/// inspector in its drawer draws after the stage, so its refusal is stated in
/// the next, as it always was.
const CONFIGURATION_REFUSAL: &[(&str, &str, &str, &[&str])] = &[
    (
        "workbench::show",
        FRAME,
        "pub fn show(",
        &[
            "docks::show_inspector(root, app, layout)",
            "CentralPanel::default()",
            "surfaces::show(ui, app)",
        ],
    ),
    (
        "RSpiceApp::contribute",
        SHELL_HOST,
        "fn contribute(",
        &["features::show_inspector::<ProjectFeature>(app, ui);\
             app.after_contribution();"],
    ),
    (
        "ProjectFeature's inspector",
        PROJECT,
        "fn show_inspector(",
        &[
            "Some(ProjectInspectorAction::UseForRuns(target)) => {",
            "ProjectFeature::commit_configuration(cx, ConfigurationTransaction::UseForRuns(target))",
        ],
    ),
    (
        "ProjectFeature::commit_configuration",
        PROJECT,
        "fn commit_configuration(",
        &[
            "let refusal = commit_configuration(",
            "cx.state.configuration.refusal = refusal;",
        ],
    ),
    (
        "the Configuration page",
        CONFIGURATION_PAGE,
        "pub(crate) fn configuration(",
        &[
            "if let Some(refusal) = cx.state.configuration.refusal.as_deref() {\
             refusal_strip(ui, refusal);",
        ],
    ),
];

#[test]
fn the_docked_project_inspector_has_a_configuration_refused_for_runs_stated_in_the_same_frame() {
    for (function, source, signature, calls) in CONFIGURATION_REFUSAL {
        let steps = calls
            .iter()
            .map(|call| (*function, std::slice::from_ref(call)))
            .collect::<Vec<Step>>();
        // The Configuration page declares its test module above its code, so
        // each file is read with its test items blanked where they sit.
        let shipped = crate::source_guard::without_test_items(source);
        assert_runs_in_order(function, &body(&shipped, signature), &steps);
    }
}

/// What creating a cell, or a view a schematic or symbol editor edits, hands
/// the shell is carried out in the order the creation once did it: the new
/// documents, the reserved revision, the creation's line in the log, the
/// view it opens, the Verilog-A receipt only once that view is the one open,
/// and the Verilog-A library saved.
const CELL_VIEWS_CREATED: &[Step] = &[(
    "CellViewsCreated",
    &[
        "ProjectRequest::CellViewsCreated {",
        "let seeded = app.state.new_schematic_document();",
        ".insert_schematic_editor(reference.key(), seeded.clone());",
        "publish_reserved_revision(app, mutation);",
        "app.state.record(LogSeverity::Info, LogSource::Studio, notice);",
        "app.state.open_workspace_view(view);",
        "&& app.state.model.workspace.content.active_view == view",
        "app.state.workbench.activate(Workspace::Design);",
        "app.state.code.code_workspace.veriloga.receipt = Some(receipt);",
        "persist_global_veriloga_library(&mut app.state.model);",
    ],
)];

/// The project feature registers `render` as its dialog step `key`, so the
/// registry draws `key` with it in the feature's context.
fn assert_project_registers(key: &str, render: &str) {
    let project = without_whitespace(PROJECT);
    let dialog = project
        .split("FeatureDialog{")
        .find(|dialog| dialog.starts_with(&format!("key:DialogKey(\"{key}\")")))
        .unwrap_or_else(|| panic!("the project feature registers `{key}`"));
    assert!(
        dialog.contains(&format!("render:{},", without_whitespace(render))),
        "the project feature draws `{key}` with `{render}`"
    );
}

/// The shell's dialog order draws the project step `key` through the
/// feature registry, which answers what the step raised straight after it
/// draws, in the step that drew it; and the feature registers `render` as
/// that step.
fn assert_project_step(key: &str, render: &str) {
    let order = without_whitespace(DIALOG_ORDER);
    let spec = order
        .split("DialogSpec{")
        .find(|spec| spec.contains(&format!("key:DialogKey(\"{key}\")")))
        .unwrap_or_else(|| panic!("the shell's dialog order draws `{key}`"));
    assert!(
        spec.contains(&format!(
            "features::render_dialog(app,ctx,DialogKey(\"{key}\"))"
        )),
        "`{key}` draws through the feature registry, which answers what it raised straight after"
    );
    assert_project_registers(key, render);
}

/// The project's cell dialogs draw through the feature registry, in the
/// feature's context, and have what they raised answered straight after
/// they draw, in the step that drew them. Copy Cell raises nothing, so its
/// answer finds nothing to carry out.
#[test]
fn the_cell_dialogs_draw_in_the_project_context_and_what_they_raise_is_answered_as_they_return() {
    assert_project_step(
        "project.rename_cell",
        "ProjectFeature::process_rename_cell_dialog",
    );
    assert_project_step(
        "project.copy_cell",
        "ProjectFeature::process_copy_cell_dialog",
    );
    assert_project_step(
        "project.new_cell",
        "ProjectFeature::process_new_cell_dialog",
    );

    let shipped = crate::source_guard::without_test_items(COMMAND_HELPERS);
    assert_runs_in_order(
        "answer_project_request",
        &body(&shipped, "fn answer_project_request("),
        CELL_VIEWS_CREATED,
    );
}

/// What creating a Layout or a Verilog-A view hands the shell is carried out
/// in the order the creation once did it: the reserved revision, the view
/// opened, the Verilog-A receipt with the Design workspace in front, and the
/// creation's line in the log, with the Verilog-A library saved last.
const VIEW_OPENED: &[Step] = &[(
    "ViewOpened",
    &[
        "ProjectRequest::ViewOpened {",
        "publish_reserved_revision(app, mutation);",
        "app.state.open_workspace_view(view);",
        "app.state.workbench.activate(Workspace::Design);",
        "app.state.code.code_workspace.veriloga.receipt = Some(*receipt);",
        "app.state.record(LogSeverity::Info, LogSource::Studio, notice);",
        "persist_global_veriloga_library(&mut app.state.model);",
    ],
)];

/// New View draws through the feature registry, in the project feature's
/// context, and has what it raised answered straight after it draws, in the
/// step that drew it. A created view's Verilog-A library is saved inside
/// that answer, after everything else it carries out, so nothing is saved
/// ahead of the creation it saves.
#[test]
fn the_new_view_dialog_draws_in_the_project_context_and_what_it_raises_is_answered_as_it_returns() {
    assert_project_step(
        "project.new_view",
        "ProjectFeature::process_new_view_dialog",
    );

    let shipped = crate::source_guard::without_test_items(COMMAND_HELPERS);
    let answer = body(&shipped, "fn answer_project_request(");
    assert_runs_in_order("answer_project_request", &answer, CELL_VIEWS_CREATED);
    assert_runs_in_order("answer_project_request", &answer, VIEW_OPENED);
    let answer = without_whitespace(&answer);
    let view_opened = answer
        .split("ProjectRequest::")
        .find(|arm| arm.starts_with("ViewOpened{"))
        .expect("the shell answers a view opened");
    assert!(
        view_opened
            .trim_end_matches('}')
            .ends_with("persist_global_veriloga_library(&mutapp.state.model);"),
        "a created view's Verilog-A library is saved last"
    );
}

/// What deleting a cell or a view hands the shell is carried out in the
/// order the deletion once did it: the compile evidence naming a source it
/// removed dropped, what the catalog change left, the reserved revision, the
/// placements removed when the reader chose that, the deletion's line in the
/// log, then the Verilog-A library saved.
const CELL_OR_VIEW_DELETED: &[Step] = &[(
    "CellOrViewDeleted",
    &[
        "ProjectRequest::CellOrViewDeleted {",
        "void_veriloga_evidence(&mut app.state, &removed_sources);",
        "apply_workspace_pruned(&mut app.state, pruned);",
        "publish_reserved_revision(app, mutation);",
        "app.state.model.remove_deleted_master_placements(",
        "app.state.record(LogSeverity::Info, LogSource::Studio, notice);",
        "persist_global_veriloga_library(&mut app.state.model);",
    ],
)];

/// The deletion review draws through the feature registry, in the project
/// feature's context, and raises nothing, so the answer after it finds
/// nothing to carry out. The staged deletion's step, a step that draws no
/// dialog, runs the same way, and has what it raised answered straight
/// after it, before the next step draws.
#[test]
fn the_library_deletion_steps_run_in_the_project_context_and_a_deletion_is_answered_as_it_returns()
{
    assert_project_step(
        "project.library_deletion_review",
        "ProjectFeature::process_library_deletion_review_dialog",
    );
    assert_project_step(
        "project.pending_library_deletions",
        "|cx, _| ProjectFeature::process_pending_library_deletions(cx)",
    );

    let shipped = crate::source_guard::without_test_items(COMMAND_HELPERS);
    assert_runs_in_order(
        "answer_project_request",
        &body(&shipped, "fn answer_project_request("),
        CELL_OR_VIEW_DELETED,
    );
}

/// What New project hands the shell is carried out as the dialog once did
/// it in place: the identity the held-open dialog names is created, then
/// the outcome is written back into the dialog.
const PROJECT_CREATED: &[Step] = &[(
    "CreateProject",
    &[
        "ProjectRequest::CreateProject => {",
        "let params = app.state.project.new_project.params();",
        "match app.state.create_new_project_with(&params) {",
        "Ok(()) => app.state.project.new_project = NewProjectDialogState::default(),",
        "Err(error) => app.state.project.new_project.error = Some(error),",
    ],
)];

/// New project draws through the feature registry, in the project feature's
/// context, and the creation it hands over is answered straight after it
/// draws, in the step that drew it, so the dialogs drawn after it in the
/// frame see the new project.
#[test]
fn the_new_project_dialog_draws_in_the_project_context_and_its_creation_is_answered_as_it_returns()
{
    assert_project_step(
        "project.new_project",
        "ProjectFeature::render_new_project_dialog",
    );

    let shipped = crate::source_guard::without_test_items(COMMAND_HELPERS);
    assert_runs_in_order(
        "answer_project_request",
        &body(&shipped, "fn answer_project_request("),
        PROJECT_CREATED,
    );
}

/// What Design management hands the shell is design's Create hierarchy,
/// opened as the dialog once opened it in place.
const CREATE_HIERARCHY_OPENED: &[Step] = &[(
    "OpenCreateHierarchy",
    &[
        "ProjectRequest::OpenCreateHierarchy => {",
        "crate::workbench::app::open_create_hierarchy_dialog(
            &mut app.state.design.create_hierarchy,
            &app.state.model,
            &mut app.state.workbench,
        );",
    ],
)];

/// Design management draws through the feature registry, in the project
/// feature's context, and the Create hierarchy it hands over is answered
/// straight after it draws, in the step that drew it, so it opens in the
/// frame it was asked for.
#[test]
fn the_design_management_dialog_draws_in_the_project_context_and_what_it_raises_is_answered_as_it_returns()
 {
    assert_project_step(
        "project.design_management",
        "ProjectFeature::render_design_management_dialog",
    );

    let shipped = crate::source_guard::without_test_items(COMMAND_HELPERS);
    assert_runs_in_order(
        "answer_project_request",
        &body(&shipped, "fn answer_project_request("),
        CREATE_HIERARCHY_OPENED,
    );
}

/// What Check and save hands the shell is saving the validated revision the
/// dialog reviewed, carried out as the dialog once carried it out in place.
const CHECK_AND_SAVE_COMMITTED: &[Step] = &[(
    "CommitCheckAndSave",
    &["ProjectRequest::CommitCheckAndSave => app.commit_check_and_save(),"],
)];

/// Check and save opens in the project feature's context, inside the call
/// of the command that asks for it, raising nothing; it draws there, through
/// the feature registry, and the save it hands over is answered straight
/// after it draws, in the step that drew it.
#[test]
fn the_check_and_save_dialog_draws_in_the_project_context_and_what_it_raises_is_answered_as_it_returns()
 {
    let execute = body(PROJECT, "fn execute(self, cx: &mut Cx<'_, ProjectFeature>)");
    let opens = without_whitespace(
        "Self::CheckAndSave => open_check_and_save_dialog(&mut check_and_save_parts(cx)),",
    );
    assert!(
        without_whitespace(&execute).contains(&opens),
        "Check and save opens in the project feature's context, in the command's own call"
    );

    assert_project_step(
        "project.check_and_save",
        "ProjectFeature::render_check_and_save_dialog",
    );

    let shipped = crate::source_guard::without_test_items(COMMAND_HELPERS);
    assert_runs_in_order(
        "answer_project_request",
        &body(&shipped, "fn answer_project_request("),
        CHECK_AND_SAVE_COMMITTED,
    );
}

/// What the technology attachment dialog's migration copy hands the shell,
/// carried out as the dialog once did it: the copy saved as a new project,
/// then its line in the log once it is saved.
const MIGRATION_COPY_SAVED: &[Step] = &[(
    "SaveMigrationCopy",
    &[
        "ProjectRequest::SaveMigrationCopy => {",
        "let saved = app.state.save_project_as();",
        "if saved {",
        "app.state.record(",
        "\"Saved a migration copy that keeps the current technology; no technology was attached.\",",
    ],
)];

/// The technology attachment dialog draws in the project feature's context
/// once the open project's page has drawn. Its migration copy closes the
/// dialog, hands the save to the shell and returns, and the workspace's
/// closing drain answers it straight after.
#[test]
fn the_technology_dialogs_migration_copy_is_answered_as_the_dialog_returns() {
    let dialog = without_whitespace(&body(
        TECHNOLOGY_DIALOG,
        "fn show_technology_attachment_dialog(",
    ));
    assert!(
        dialog.contains(&without_whitespace(
            "cx.state.technology_attachment.close();
             cx.wb.outbox.push(ProjectRequest::SaveMigrationCopy);
             return;"
        )),
        "the migration copy closes the dialog, then hands the save over and returns"
    );
    assert_eq!(
        dialog.matches("outbox.push(").count(),
        1,
        "the dialog hands the shell one request"
    );
    assert_runs_in_order(
        "RSpiceApp::contribute",
        &body(SHELL_HOST, "fn contribute("),
        &[(
            "technology dialog",
            &[
                "project::show_technology_attachment_dialog(&ctx, &mut features::cx(app));",
                "app.after_contribution();",
                "Point::SplitResults =>",
            ],
        )],
    );
    let shipped = crate::source_guard::without_test_items(COMMAND_HELPERS);
    assert_runs_in_order(
        "answer_project_request",
        &body(&shipped, "fn answer_project_request("),
        MIGRATION_COPY_SAVED,
    );
}

/// What the configuration dialogs hand the shell is going on leaving the
/// project once its configuration edits are settled, as the dialog once went
/// on in place: the question about unsaved work, or the action carried out.
const LEAVING_GONE_ON: &[Step] = &[(
    "GoOnLeaving",
    &[
        "ProjectRequest::GoOnLeaving { action, recent } => {",
        "go_on_leaving(&mut app.state, Leaving { action, recent })",
        "app.execute_pending_action(action, path, kind, None);",
    ],
)];

/// The configuration dialogs draw through the feature registry, in the
/// project feature's context, and going on leaving, which they hand over, is
/// answered straight after they draw, in the step that drew them.
#[test]
fn the_configuration_dialogs_draw_in_the_project_context_and_what_they_raise_is_answered_as_they_return()
 {
    assert_project_step(
        "project.configuration",
        "ProjectFeature::render_configuration_dialog",
    );

    let shipped = crate::source_guard::without_test_items(COMMAND_HELPERS);
    assert_runs_in_order(
        "answer_project_request",
        &body(&shipped, "fn answer_project_request("),
        LEAVING_GONE_ON,
    );
}

/// The Library page's calls in the project feature's context, each intent
/// carried out as its call returns: the detail reads the selection the
/// browser's intent writes, so it draws after that intent is carried out.
/// With the built-in library open, its source and its band take the detail's
/// place, in that order. Then the reports.
const LIBRARY_PAGE: &[Step] = &[
    (
        "browser",
        &[
            "let (intent, source_height) = library_browser(ui, cx);",
            "if let Some(intent) = intent {\
             apply_library_intent(&ctx, cx, intent);\
             }",
        ],
    ),
    (
        "built-in source and band",
        &[
            "if cx.state.builtin_models.open {",
            "if let Some(intent) = builtin_source(ui, cx, source_height) {\
             apply_library_intent(&ctx, cx, intent);\
             }",
            "if let Some(intent) = builtin_detail(ui, cx) {\
             apply_library_intent(&ctx, cx, intent);\
             }",
        ],
    ),
    (
        "detail",
        &["} else if let Some(intent) = library_detail(ui, cx) {\
             apply_library_intent(&ctx, cx, intent);\
             }"],
    ),
    (
        "reports",
        &[
            "show_library_usage_report(&ctx);",
            "show_library_audit_history(&ctx, cx.model);",
        ],
    ),
];

/// The Library surface as the shell composes it: the page in the project
/// feature's context, then what it raised (a view to open, a cell to create)
/// answered as it returns, before the publication dialogs draw.
const LIBRARY_COMPOSITION: &[Step] = &[
    ("page", &["page(ui, &mut features::cx(app));"]),
    ("what the page raised", &["app.after_contribution();"]),
    (
        "publication",
        &["super::library_publication::show(&ctx, app);"],
    ),
];

/// The library and view membership dialogs the Library surface draws, in
/// today's order, each through the feature registry, in the project
/// feature's context, which answers what each raised as it returns. New
/// Library raises nothing, so the answer after it finds nothing to carry out.
const LIBRARY_DIALOGS: &[Step] = &[
    (
        "New Library",
        &["render_dialog(app, &ctx, DialogKey(\"project.new_library\"));"],
    ),
    (
        "Rename Library",
        &["render_dialog(app, &ctx, DialogKey(\"project.rename_library\"));"],
    ),
    (
        "Delete Library",
        &["render_dialog(app, &ctx, DialogKey(\"project.delete_library\"));"],
    ),
    (
        "Rename View",
        &["render_dialog(app, &ctx, DialogKey(\"project.rename_view\"));"],
    ),
];

/// What a catalog change hands the shell is carried out in the order the
/// change once did it: what it left outside the model, then the revision a
/// deletion reserved, then the change's own line in the log.
const CATALOG_CHANGED: &[Step] = &[(
    "CatalogChanged",
    &[
        "ProjectRequest::CatalogChanged {",
        "apply_workspace_pruned(&mut app.state, pruned);",
        "publish_reserved_revision(app, mutation);",
        "app.state.record(LogSeverity::Info, LogSource::Studio, notice);",
    ],
)];

/// A deletion's reserved revision is answered as the dialog that reserved it
/// returns, so no copy of the session can be taken while it waits.
#[test]
fn the_library_dialogs_draw_in_the_project_context_and_a_change_is_answered_as_it_returns() {
    assert_runs_in_order(
        "surfaces::project::library::page",
        &body(LIBRARY_SURFACE, "pub(crate) fn page("),
        LIBRARY_PAGE,
    );
    let library = body(LIBRARY_HOST, "pub(super) fn library(");
    assert_runs_in_order(
        "shell_host::project::library",
        &library,
        &[LIBRARY_COMPOSITION, LIBRARY_DIALOGS].concat(),
    );
    assert!(
        without_whitespace(&library).ends_with(&without_whitespace(
            "render_dialog(app, &ctx, DialogKey(\"project.rename_view\"));"
        )),
        "the Library surface draws nothing after its last dialog has what it raised answered"
    );
    assert_project_registers(
        "project.new_library",
        "ProjectFeature::process_new_library_dialog",
    );
    assert_project_registers(
        "project.rename_library",
        "ProjectFeature::process_rename_library_dialog",
    );
    assert_project_registers(
        "project.delete_library",
        "ProjectFeature::process_delete_library_dialog",
    );
    assert_project_registers(
        "project.rename_view",
        "ProjectFeature::process_rename_view_dialog",
    );

    let shipped = crate::source_guard::without_test_items(COMMAND_HELPERS);
    assert_runs_in_order(
        "answer_project_request",
        &body(&shipped, "fn answer_project_request("),
        CATALOG_CHANGED,
    );
}

#[test]
fn the_project_feature_takes_the_browser_project_import_where_the_frame_did() {
    assert_eq!(
        without_whitespace(&body(FRAME, "fn poll_browser_project_import(")),
        without_whitespace(
            "use crate::workbench::project::ProjectFeature;\
             ProjectFeature::poll_browser_project_import(&mut features::cx(self))"
        ),
        "the frame's browser project import step runs in the project feature's context, \
         and only that"
    );
    assert_runs_in_order(
        "ProjectFeature::poll_browser_project_import",
        &body(PROJECT, "fn poll_browser_project_import("),
        &[(
            "5.24",
            &["project_workflow::poll_browser_project_import(cx.model, cx.platform,)"],
        )],
    );
}

#[test]
fn the_models_feature_takes_browser_pdk_storage_news_where_the_frame_did() {
    assert_eq!(
        without_whitespace(&body(FRAME, "fn poll_browser_pdk_persistence(")),
        without_whitespace(
            "ModelsFeature::poll_browser_pdk_persistence(&mut features::cx(self), ctx);"
        ),
        "the frame's browser PDK storage step runs in the models feature's context, and only that"
    );
    assert_runs_in_order(
        "ModelsFeature::poll_browser_pdk_persistence",
        &body(MODELS, "fn poll_browser_pdk_persistence("),
        &[(
            "5.19",
            &["poll_browser_pdk_persistence(\
               &mut BrowserPdkParts {\
               model: cx.model,\
               workbench: cx.wb,\
               platform: cx.platform,\
               preflight: &mut cx.runs.preflight,\
               run_controller: cx.handles.run_controller,\
               },\
               ctx,\
               );"],
        )],
    );
}

#[test]
fn the_models_feature_takes_the_browser_pack_restore_where_the_frame_did() {
    assert_eq!(
        without_whitespace(&body(FRAME, "fn poll_browser_pack_restore(")),
        without_whitespace(
            "ModelsFeature::poll_browser_pack_restore(&mut features::cx(self), ctx);"
        ),
        "the frame's browser pack restore step runs in the models feature's context, and only that"
    );
    assert_runs_in_order(
        "ModelsFeature::poll_browser_pack_restore",
        &body(MODELS, "fn poll_browser_pack_restore("),
        &[(
            "5.20",
            &["poll_browser_pack_restore(\
                 &mut PackRestoreParts {\
                 models_view: &mut cx.state.models_view,\
                 model: cx.model,\
                 model_hub: cx.handles.model_hub,\
                 },\
                 ctx,\
                 );"],
        )],
    );
}

#[test]
fn the_models_feature_keeps_its_authoring_recovery_where_the_shell_did() {
    assert_runs_in_order(
        "RSpiceApp::end_frame",
        &body(APP, "fn end_frame("),
        END_FRAME,
    );
    assert_runs_in_order("RSpiceApp::save", &body(APP, "fn save("), SAVE);
    assert_runs_in_order(
        "AppState::deserialize",
        &body(SERIALIZATION, "fn deserialize<D>("),
        SESSION_RESTORE,
    );
}

#[test]
fn the_window_title_reads_the_unsaved_authoring_where_it_did() {
    assert_runs_in_order(
        "RSpiceApp::sync_window_title",
        &body(APP, "fn sync_window_title("),
        SYNC_WINDOW_TITLE,
    );
}

#[test]
fn the_browser_stores_start_reading_in_todays_order() {
    assert_runs_in_order("RSpiceApp::new", &body(APP, "fn new("), STARTUP);
}

#[test]
fn a_results_dialog_has_what_it_raised_answered_once_it_draws() {
    let render = body(FEATURES, "fn render_dialog(");
    assert_runs_in_order(
        "features::render_dialog",
        &render,
        &[(
            "1",
            &[
                "(dialog.render)(&mut cx::<$feature>(app), ctx);",
                "app.after_contribution();",
            ],
        )],
    );
    assert!(
        without_whitespace(&render).ends_with("app.after_contribution();"),
        "nothing runs after a dialog step has what it raised answered"
    );

    let answer = body(FRAME, "fn answer_result_requests(");
    assert_runs_in_order("answer_result_requests", &answer, ANSWER_RESULT_REQUESTS);
    assert_eq!(
        answer.matches("requests::drain(").count(),
        3,
        "`answer_result_requests` answers exactly the three requests it lists"
    );

    let surface = body(RESULTS, "fn show_surface(");
    assert_runs_in_order("ResultsFeature::show_surface", &surface, RESULTS_SURFACE);
    assert!(
        without_whitespace(&surface).ends_with(&without_whitespace(
            "render_result_document_dialog(cx, ui.ctx());"
        )),
        "the result document's forms are the last things the Results stage draws"
    );
}

#[test]
fn a_hop_into_results_has_what_it_raised_answered_where_it_is_taken() {
    let offer: &[Step] = &[(
        "offer",
        &[
            "ResultsFeature::perform_notification_action(&mut features::cx(app), action);",
            "app.after_contribution();",
        ],
    )];
    for (function, signature) in [
        ("workbench::show", "pub fn show("),
        (
            "show_full_screen_presentation",
            "fn show_full_screen_presentation(",
        ),
        ("show_secondary", "pub(crate) fn show_secondary("),
    ] {
        assert_runs_in_order(function, &body(FRAME, signature), offer);
    }
    assert_eq!(
        without_whitespace(&body(SHELL_HOST, "fn perform_notification_action(")),
        without_whitespace(
            "ResultsFeature::perform_notification_action(&mut features::cx(self), action);             self.after_contribution();"
        ),
        "the notification centre's offer is answered as the toast's is"
    );
    assert_eq!(
        without_whitespace(&body(STATUS_ORDER, "fn open_newest_retained_run(")),
        without_whitespace(
            "let opened = ResultsFeature::open_newest_retained_run(&mut features::cx(self));             self.after_contribution();             opened"
        ),
        "the engine chip's hop has what it raised answered before it returns"
    );
}
