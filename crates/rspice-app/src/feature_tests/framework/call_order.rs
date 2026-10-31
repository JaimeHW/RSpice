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
const FRAME: &str = include_str!("../../workbench/frame.rs");
const DIALOG_ORDER: &str = include_str!("../../workbench/frame/dialog_order.rs");
const DISPATCH: &str = include_str!("../../workbench/dispatch.rs");
const EXPORT_IMAGE: &str = include_str!("../../workbench/app/actions/export_image.rs");
const SHELL_HOST: &str = include_str!("../../workbench/shell_host.rs");
const FEATURES: &str = include_str!("../../workbench/features.rs");
const RESULTS: &str = include_str!("../../../../rspice-app-results/src/workbench/results.rs");
const STATUS_ORDER: &str = include_str!("../../workbench/shell_host/status_order.rs");
const PROJECT_TRANSACTIONS: &str = include_str!("../../workbench/session/project_transactions.rs");

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
    (
        "5.6",
        &["self.terminate_automation_runtime_after_project_change()"],
    ),
    ("5.7", &["self.automation_runtime.poll_discovery()"]),
    ("5.8", &["code_workspace::poll_veriloga_import(self)"]),
    ("5.9", &["code_workspace::poll_veriloga_compile(self)"]),
    (
        "5.10",
        &["code_workspace::consume_veriloga_compile_request(self, ctx)"],
    ),
    ("5.11", &["code_workspace::refresh_symbol_standing("]),
    (
        "5.12",
        &["code_workspace::poll_automation_source_import(self)"],
    ),
    ("5.13", &["code_workspace::poll_automation_workflow(self)"]),
    (
        "5.14",
        &["code_workspace::settle_pending_python_execute(self)"],
    ),
    (
        "5.15",
        &["code_workspace::poll_managed_automation_runtime(self)"],
    ),
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
            "project_workflow::poll_browser_project_import(",
            "self.restore_workspace_after_project_load()",
        ],
    ),
    (
        "5.25",
        &[
            "project_workflow::poll_browser_project_save(",
            "self.handle_save_continuation_event(event)",
        ],
    ),
    (
        "5.26",
        &[
            "file_workflow::poll_browser_schematic_import(",
            "self.state.results.analysis.clear_transient_specialized_viewer_data()",
        ],
    ),
    (
        "5.27",
        &["netlist_workflow::poll_browser_netlist_workflow("],
    ),
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
            "app.state.open_project_launcher()",
            "tools::project_launcher::show(ctx, app)",
        ],
    ),
    (
        "11.25",
        &["tools::examples_browser::show(ctx, &mut app.state)"],
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
            "show_toasts(ctx, &mut app.state, layout)",
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
    ("synchronize_design_management_route", "shell"),
    (
        "dialogs::confirmation::render_confirmation_dialog",
        "framework",
    ),
    ("dialogs::property_dialog::render_property_dialog", "design"),
    ("process_pdk_settings_dialog", "models"),
    ("features::render_dialog", "settings"),
    ("features::render_dialog", "cloud"),
    ("render_new_project_dialog", "project"),
    ("features::render_dialog", "cloud"),
    ("features::render_dialog", "cloud"),
    ("features::render_dialog", "cloud"),
    ("command_palette::render_command_palette", "framework"),
    ("render_bus_tap_dialog", "design"),
    ("render_builtin_xspice_placement_dialog", "design"),
    ("render_net_label_dialog", "design"),
    ("render_grid_snap_routing_dialog", "design"),
    ("features::render_dialog", "sheets"),
    ("render_schematic_visibility_dialog", "design"),
    ("render_descend_hierarchy_dialog", "design"),
    ("render_engineering_table_dialog", "design"),
    ("render_pin_port_dialog", "design"),
    ("render_design_review_comments_dialog", "design"),
    ("render_design_note_dialog", "design"),
    ("render_documentation_shape_dialog", "design"),
    ("render_array_selection_dialog", "design"),
    ("render_replace_instance_dialog", "design"),
    (
        "dialogs::stimulus_link::render_stimulus_link_dialog",
        "design",
    ),
    ("render_create_hierarchy_dialog", "design"),
    ("render_check_and_save_dialog", "project"),
    ("render_connectivity_manager_dialog", "design"),
    ("features::render_dialog", "sheets"),
    ("render_design_management_dialog", "project"),
    ("render_selection_bulk_edit_dialog", "design"),
    ("render_project_revision_history_dialog", "project"),
    ("render_source_document_dialog", "code"),
    ("render_source_find_dialog", "code"),
    ("render_source_language_tools_dialog", "code"),
    ("render_create_model_bound_symbol_dialog", "design"),
    ("render_symbol_definition_dialogs", "design"),
    ("render_configuration_dialog", "project"),
    ("dialogs::rule_setup::render_rule_setup_dialog", "design"),
    ("invalidate_simulation_preflight", "shell"),
    ("features::render_dialog", "sheets"),
    ("features::render_dialog", "sheets"),
    ("features::render_dialog", "sheets"),
    ("features::render_dialog", "hardcopy"),
    ("render_object_properties_dialog", "design"),
    ("render_rename_selection_dialog", "design"),
    ("render_find_replace_dialog", "design"),
    ("render_shared_properties_dialog", "design"),
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
    ("models::render_model_browser_dialog", "models"),
    ("process_new_cell_dialog", "project"),
    ("process_new_view_dialog", "project"),
    ("render_new_veriloga_device_form", "code"),
    ("render_veriloga_import_dialog", "code"),
    ("render_veriloga_replace_dialog", "code"),
    ("render_veriloga_build_settings", "code"),
    ("process_copy_cell_dialog", "project"),
    ("process_rename_cell_dialog", "project"),
    ("process_library_deletion_review_dialog", "project"),
    ("process_plan_removal_review_dialog", "simulate"),
    ("process_plan_removal_refusal_dialog", "simulate"),
    (
        "app_state::services_access::process_autosave_restore_dialog",
        "services",
    ),
    ("process_pending_library_deletions", "project"),
    ("process_exit_request", "shell"),
    ("show_route_overlays", "shell"),
];

/// The steps of [`DIALOG_STEPS`] that draw no dialog of their own: they keep
/// the routes and the dialogs they own in step, carry one dialog's outcome to
/// the run, finish what a dialog decided, or draw the route overlays.
const SYNTHETIC_DIALOG_STEPS: &[&str] = &[
    "synchronize_route_owned_dialogs",
    "synchronize_design_management_route",
    "invalidate_simulation_preflight",
    "process_pending_library_deletions",
    "process_exit_request",
    "show_route_overlays",
];

/// What `show_route_overlays`, the last dialog step, drew, and the slice that
/// owns each: above every dialog, so a workflow opened from one of them is
/// the top modal. The dialog order lists each as a step of its own.
const ROUTE_OVERLAYS: &[(&str, &str)] = &[
    ("features::render_dialog", "cloud"),
    ("tools::jobs_manager::show", "simulate"),
    ("tools::specialist_tool_browser::show", "framework"),
    ("surfaces::show_simulation_workflow_dialogs", "simulate"),
];

/// `RSpiceApp::dispatch_feature_command`: a feature's command runs behind
/// the shared prologue, and what it raised is answered straight after it, in
/// the same dispatch.
const DISPATCH_FEATURE_COMMAND: &[Step] = &[
    ("1", &["held_back(flags, &self.state.platform).is_some()"]),
    ("2", &["self.state.deny_read_only_edit()"]),
    ("3", &["self.state.commit_pending_inspector_edit()"]),
    ("4", &["features::execute(self, spec.key)"]),
    ("5", &["self.after_contribution()"]),
];

/// `RSpiceApp::after_contribution`, in the order a feature's call has what
/// it raised answered. The commands it ran next go first, so a report's
/// Save is carried out before anything else the call raised.
const AFTER_CONTRIBUTION: &[Step] = &[
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
            "app.state.simulate.active_analysis_instance = instance",
        ],
    ),
    (
        "8",
        &[
            "|app, RevealExecutedDeck(sequence)|",
            "open_task_deck(&mut app.state, sequence)",
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
            "if app.state.highlight_active_failure_sites() {",
            "app.state.results.results.marked_failure_run = run_sequence;",
        ],
    ),
    (
        "13",
        &["|app, CloseForLiveMirror| app.close_for_live_mirror()"],
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
            "Workspace::Simulate => simulate::show(ui, app),",
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
/// context: none answers anything there, as none did when the shell drew
/// them, so a request another point left waits for the answer it waited for.
const MODELS_CONTRIBUTIONS: &[Step] = &[
    (
        "navigator",
        &["Workspace::Verify => verify(ui, app),\
           Workspace::Models => features::show_navigator::<ModelsFeature>(app, ui),\
           }"],
    ),
    (
        "workspace inspector",
        &["Workspace::Verify => verify(ui, app),\
           Workspace::Models => features::show_inspector::<ModelsFeature>(app, ui),\
           Workspace::Netlist =>"],
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
            "apply_live_project_snapshot(state, snapshot.bytes(), snapshot.host_label())",
            "live_session.mirror_snapshot_applied(&mut live_host(state), snapshot, outcome)",
        ],
    ),
    ("6", &["live_session.pump_outbound(&mut live_host(state))"]),
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
                renders: top_level_calls(render),
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
        76,
        "the frame's 73 dialog steps, the route overlays' 4 owners in place of their host"
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
            SYNTHETIC_DIALOG_STEPS.contains(&first),
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
    assert_eq!(
        DIALOG_STEPS.len() - SYNTHETIC_DIALOG_STEPS.len(),
        67,
        "the frame draws 67 dialog steps beside its synthetic ones"
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
        13,
        "`after_contribution` answers exactly the thirteen requests it lists"
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
