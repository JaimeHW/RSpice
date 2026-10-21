//! A session with every persisted field off its default, and the round trips
//! it must survive unchanged.
//!
//! The session's wire is three structs: the application's own fields, the
//! `ui_session` subset of the document engines' state, and the workbench's
//! persisted layout and selections. A wire mismatch in any of them does not
//! fail a JSON test; it costs the reader the whole session at the next launch,
//! which has happened twice. So the session here is saved and read back the
//! way eframe does it, read strictly, read as the safe-mode JSON snapshot, and
//! kept without its runs, and each must come back as it was written.

use super::tests::{MemoryStorage, relaunch};
use super::*;

use crate::state::ModelsPage;
use crate::workbench::state::{
    ConsolePage, DesignPanel, LibraryCellviewPage, ProjectPage, SimulationPage, VerificationPage,
    Workspace, WorkspaceDocumentId,
};

/// A session in mid-work with every persisted field moved off its default:
/// a closed project that left its documents, a run and an unsaved model edit
/// behind, and a workbench, editor and preferences the reader has arranged.
///
/// Add a field to the session's wire and this must set it, or
/// [`the_populated_session_moves_every_persisted_field_off_its_default`]
/// fails.
///
/// The round trips compare the session as a value, so a hash map may hold
/// any number of entries. A hash set is written as a list in the order it
/// happens to iterate, so none here holds more than one.
pub(super) fn populated_session() -> AppState {
    let mut state = AppState::default();

    // The project, its drawing and its library browser.
    state
        .workspace
        .content
        .project
        .rename("Precision AFE")
        .expect("valid project name");
    state.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::new(17, 29),
    );
    state.workspace.save_active_schematic(&state.schematic);
    state.library_manager.filter_text = "afe".to_owned();

    // The plan, through the isolated execution context: an operating point
    // ahead of the transient, and a project-owned model.
    let operating_point = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("a new session owns a stable plan")
        .insert_at(crate::simulation::plan::AnalysisKind::OperatingPoint, 0)
        .expect("an operating point inserts ahead of the transient")
        .0;
    state
        .model_library_manager
        .create_project_model(
            "owned-models",
            &crate::state::model_library::ProjectModelDefinition {
                name: "nch_owned".to_owned(),
                spice_type: "NMOS".to_owned(),
                description: "Project model".to_owned(),
                numeric_parameters: std::collections::BTreeMap::new(),
                string_parameters: std::collections::BTreeMap::new(),
            },
        )
        .expect("a project model is created");

    // An unsaved edit of that model, kept for recovery.
    state
        .workbench
        .model_editor
        .open(
            &state.model_library_manager,
            "owned-models",
            "nch_owned",
            state.workspace.content.project.revision(),
        )
        .expect("the model editor opens on the project model");
    state
        .workbench
        .model_editor
        .draft
        .as_mut()
        .expect("an open draft")
        .description = "Unsaved description".to_owned();
    state.workbench.capture_authoring_recovery();

    // One retained run, finished: a run still in flight when the session is
    // saved restores as interrupted.
    let mut run = crate::state::SimulationRun::new(3);
    run.mark_running().expect("a new run starts");
    run.finish_lifecycle(crate::state::SimulationRunLifecycle::Completed)
        .expect("a running run completes");
    run.label = "Run 3 (bias sweep)".to_owned();
    run.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![crate::state::RetainedTrace::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 0.75, 1.5],
                "#00aaff",
            )]),
    );
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .expect("legacy fixture provenance seals");
    state.simulation.retained.runs = vec![run].into();
    state.simulation.retained.next_run_id = 3;
    state.simulation.view.active_run_idx = Some(0);
    state.simulation.view.active_analysis_idx = Some(0);

    // Authority over the canonical project, on each host.
    let project_id = state.workspace.content.project.id().to_string();
    state.native_project_binding_receipt = Some(
        crate::workbench::lifecycle::project_lifecycle::NativeBindingReceipt {
            canonical_path: std::path::PathBuf::from(r"C:\projects\precision-afe.rspiceproj"),
            project_id: project_id.clone(),
            accepted_digest: crate::product::ContentDigest::from_bytes([0x5a; 32]),
        },
    );
    state.browser_project_binding_receipt = Some(
        crate::workbench::lifecycle::project_lifecycle::BrowserBindingReceipt {
            binding_id: uuid::Uuid::from_u128(0xc23c_8916_2865_430a_a612_ecbb_111b_3ce1),
            project_id,
            accepted_generation: 23,
            accepted_digest: "ab".repeat(32).parse().expect("valid digest fixture"),
            backend: crate::workbench::lifecycle::project_lifecycle::BrowserBindingBackend::Opfs,
        },
    );
    state
        .recent_files
        .push(crate::services::recent_files::RecentFile {
            kind: crate::services::recent_files::RecentKind::Project,
            path: std::path::PathBuf::from(r"C:\projects\precision-afe.rspiceproj"),
            opened_at_unix_ms: 1_760_000_000_000,
            pinned: true,
            owner: Some("Analog team".to_owned()),
            tags: vec!["afe".to_owned()],
        });
    state.license_key = Some("RSPICE-SESSION-FIXTURE".to_owned());

    populate_ui_session(&mut state);
    populate_workbench(&mut state, operating_point);

    // Closed last: Close Project keeps the documents it closed over.
    state.project_lifecycle.authority.close_project();
    state
}

/// Every field `UiSessionWire` writes.
fn populate_ui_session(state: &mut AppState) {
    let ui = &mut state.ui;
    ui.theme.density = crate::ui::Density::Relaxed;
    // Grid off, with lines the style a toggle brings back.
    ui.set_grid_style(crate::state::GridStyle::Lines);
    ui.set_grid_style(crate::state::GridStyle::Off);
    ui.schematic_selection_filter.instances = false;
    ui.schematic_bulk_edit_filters.active.query = "OPA*".to_owned();
    ui.schematic_visibility.review_markers = crate::state::SchematicReviewMarkerVisibility::All;
    ui.drawing_sheet_layers.zones = false;
    ui.schematic_snap.enabled = false;
    ui.schematic_routing_mode = crate::state::WireRoutingMode::FortyFiveDegree;
    ui.duplicate_external_nets = crate::state::DuplicateExternalNets::PreserveNamedNetAttachment;
    let dataset = crate::state::engineering_table::active_schematic_dataset(&Default::default());
    let view = crate::state::engineering_table::EngineeringTableView::for_dataset(&dataset);
    ui.engineering_table_views
        .save(
            "My device view",
            crate::state::engineering_table::EngineeringViewScope::Personal,
            view,
            true,
            &dataset,
        )
        .expect("valid personal view");
    ui.autosave_minutes = 10;
    let wheel = crate::workbench::preferences::TogglePreference::InvertWheelZoom;
    let inverted = !ui.preferences.toggle(wheel);
    ui.preferences.set_toggle(wheel, inverted);
    // Read through an untagged storage enum, which keeps a domain it cannot
    // read as an untyped value.
    ui.preferences
        .workspace_mut()
        .expect("the shipped workspace preferences are current")
        .set_background_task_attention(
            crate::workbench::preferences::BackgroundTaskAttention::Silent,
        );
    ui.preferences
        .shortcut_profiles_mut()
        .replace_active(customized_shortcuts())
        .expect("the shipped shortcut library is compatible");
    ui.browser_spoken_feedback = true;
    ui.text_locale = crate::workbench::UiTextLocale::PseudoExpanded;
    ui.results.session.viewer = rspice_results::result_presentation::ResultViewer::Bode;
    ui.results.session.eye_timebase.insert(
        rspice_results_ui::eye_diagram::EyeTimebaseKey::Prepared(
            crate::product::AnalysisInstanceId::new(),
        ),
        rspice_results_ui::eye_diagram::EyeTimebase::Explicit {
            unit_interval: 1e-9,
        },
    );
}

/// The shortcut profile in use moved off its shipped state: every binding
/// policy changed, and Save moved to a chord with every modifier.
///
/// The profile reads through an untyped value, so a value the session
/// writes in a form that value cannot hold is lost on the way back.
fn customized_shortcuts() -> crate::state::shortcuts::ShortcutPreferences {
    use crate::state::shortcuts::{
        ChordTimeoutPolicy, CommandShortcutOverride, ContextPrecedencePolicy,
        ProfileShortcutBinding, ProtectedShortcutPolicy, ShortcutBindingSlot, ShortcutPreferences,
        ShortcutSequence, ShortcutStroke, SingleKeyCanvasPolicy,
    };

    // A browser keeps Save's shipped chord for itself, so Save needs an
    // alternate there.
    let everywhere = |slot, key| {
        ProfileShortcutBinding::new(
            slot,
            serde_json::from_value(serde_json::json!(["desktop", "browser", "tablet", "phone"]))
                .expect("every platform by its stored name"),
            ShortcutSequence::single(ShortcutStroke::new(key, true, true, true)),
        )
        .expect("a binding on every platform")
    };
    let save = CommandShortcutOverride::new(vec![
        everywhere(ShortcutBindingSlot::Primary, egui::Key::F6),
        everywhere(ShortcutBindingSlot::Alternate, egui::Key::F7),
    ])
    .expect("one primary and one alternate binding");
    let mut profile: ShortcutPreferences = serde_json::from_value(serde_json::json!({
        "commands": { "save-project": save },
    }))
    .expect("a profile that rebinds Save");
    let policies = profile.policies_mut();
    policies.set_single_key_canvas(SingleKeyCanvasPolicy::RequireAlt);
    policies.set_chord_timeout(ChordTimeoutPolicy::NoTimeout);
    policies.set_protected_shortcuts(ProtectedShortcutPolicy::AllowWithConfirmation);
    policies.set_context_precedence(ContextPrecedencePolicy::EditorModalWorkspaceGlobal);
    assert!(
        profile
            .audit(&crate::workbench::commands::vocabulary::CommandRegistry)
            .is_valid(),
        "the customized profile is one the reader could have saved: {:?}",
        profile.audit(&crate::workbench::commands::vocabulary::CommandRegistry)
    );
    profile
}

/// Every field `WorkbenchState` persists. The model-editor recovery is set
/// by [`populated_session`], which owns the model it recovers.
fn populate_workbench(state: &mut AppState, operating_point: crate::product::AnalysisInstanceId) {
    let workbench = &mut state.workbench;
    // Navigating records the route, the workspace and the layout left behind,
    // and applies the destination's, so the layout below comes after it.
    workbench.activate(Workspace::Simulate);
    workbench.window_session.set_clamp_restored_windows(false);
    workbench.set_workspace_preset(crate::workbench::WorkspacePreset::Canvas);
    workbench.navigator_visible = false;
    workbench.inspector_visible = false;
    workbench.console_visible = true;
    workbench.console_maximized = true;
    workbench.focus_mode = true;
    workbench.split_with_results = true;
    workbench.navigator_width = 311.0;
    workbench.navigator_width_custom = true;
    workbench.inspector_width = 287.0;
    workbench.inspector_width_custom = true;
    workbench.console_height = 244.0;
    workbench.console_page = ConsolePage::Problems;
    workbench.design_panel = DesignPanel::ComponentShelf;
    workbench
        .documents
        .activate(WorkspaceDocumentId::SimulationPlan);
    workbench
        .netlist_open_documents
        .insert(WorkspaceDocumentId::NetlistComparison);
    workbench.project_page = ProjectPage::Recovery;
    workbench.project_library_filter = "afe".to_owned();
    workbench.project_dependency_filter = "pdk".to_owned();
    workbench.project_dependency_selection = Some("models/afe.lib".to_owned());
    workbench.project_checkpoint_selection = Some("checkpoint-2".to_owned());
    workbench.library_cellview_page = LibraryCellviewPage::SymbolForm;
    workbench.verification_page = VerificationPage::Corners;
    workbench.simulation_page = SimulationPage::Outputs;
    workbench.verification.corner_compare_nominal = true;
    workbench.verification.regression_baseline_run = Some(crate::product::RunId::new());
    workbench.models_page = ModelsPage::Bins;
    workbench.models_view.catalog_query = "nch".to_owned();
    workbench.active_analysis = 3;
    workbench.active_analysis_instance = Some(operating_point);
    workbench.selected_specification = Some("gain".to_owned());
    workbench.selected_design_variable = Some("rload".to_owned());
    workbench.selected_saved_output = Some("vout".to_owned());
    workbench.selected_capture_group = Some(crate::product::CaptureGroupId::new());
    workbench.selected_run_set_dimension = Some("temperature".to_owned());
    workbench.selected_model = Some("nch".to_owned());
    workbench
        .netlist_outline_collapsed
        .insert(crate::state::OutlineSectionKind::Parameters);
    workbench.command_query = "probe".to_owned();
    workbench
        .specialist_tool_browser
        .toggle_favorite(crate::workbench::SurfaceId::RfWorkbench);
    workbench.report_authoring.selected_document = Some(crate::product::ResultDocumentId::new());
    workbench.pdk_technology_authoring.dirty = true;
}

/// `state` as a restore leaves it before anything else is read back.
///
/// Every restore reconciles the project's library model, and reconciling
/// advances the library catalog's revision whether or not it changes
/// anything. So no session saves again exactly as it was read: the one it
/// saves is the one written, after this step.
fn as_restored(mut state: AppState) -> AppState {
    state
        .workspace
        .ensure_library_model(&mut state.library_manager);
    state
}

/// Save the session the way the application does on exit.
fn stored(state: &mut AppState) -> String {
    let mut storage = MemoryStorage::default();
    state.store_session(&mut storage);
    eframe::Storage::get_string(&storage, eframe::APP_KEY).expect("the session is stored")
}

/// The session as a value: what `encode` wrote, with the execution context,
/// which the session holds as JSON text, read as a value as well.
///
/// Text cannot be compared. A hash map writes its entries in an order that
/// differs from one instance to the next, and the library catalog, the
/// documents and the built-in model catalog are all hash maps. A value's
/// maps compare by key.
fn session_value(encode: impl serde::Serialize) -> serde_json::Value {
    let mut session = serde_json::to_value(encode).expect("the session encodes as JSON");
    let context = session
        .get_mut("execution_context_json")
        .expect("the session writes its execution context");
    let parsed = serde_json::from_str(context.as_str().expect("the context is held as text"))
        .expect("the execution context is JSON");
    *context = parsed;
    session
}

/// Fail on the first place two sessions part company, naming where, rather
/// than showing two whole sessions.
fn assert_same_session(written: &serde_json::Value, read: &serde_json::Value, encoding: &str) {
    if let Some((at, written, read)) = first_difference(written, read, "") {
        panic!(
            "the {encoding} session is not what was written, at {at}:\n\
             written: {written}\n\
             read:    {read}"
        );
    }
}

fn first_difference(
    written: &serde_json::Value,
    read: &serde_json::Value,
    at: &str,
) -> Option<(String, String, String)> {
    use serde_json::Value;

    let shown = |value: Option<&Value>| {
        let text = value.map_or_else(|| "(absent)".to_owned(), Value::to_string);
        match text.char_indices().nth(400) {
            Some((end, _)) => format!("{}…", &text[..end]),
            None => text,
        }
    };
    match (written, read) {
        (Value::Object(written), Value::Object(read)) => written
            .keys()
            .chain(read.keys())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .find_map(|key| {
                let at = format!("{at}/{key}");
                match (written.get(key), read.get(key)) {
                    (Some(written), Some(read)) => first_difference(written, read, &at),
                    (written, read) => Some((at, shown(written), shown(read))),
                }
            }),
        (Value::Array(written), Value::Array(read)) if written.len() == read.len() => written
            .iter()
            .zip(read)
            .enumerate()
            .find_map(|(index, (written, read))| {
                first_difference(written, read, &format!("{at}/{index}"))
            }),
        _ if written == read => None,
        _ => {
            let at = if at.is_empty() { "/" } else { at };
            Some((at.to_owned(), shown(Some(written)), shown(Some(read))))
        }
    }
}

/// Saved through eframe and read back at the next launch, the session saves
/// again as it was written.
#[test]
fn a_populated_session_restores_and_saves_again_unchanged() {
    let mut state = populated_session();
    let saved = stored(&mut state);

    let restored = relaunch(saved);

    assert_eq!(restored.workspace.content.project.name(), "Precision AFE");
    assert!(!restored.project_lifecycle.is_open());
    assert_eq!(restored.simulation.retained.run_count(), 1);
    assert!(
        restored.workbench.model_editor.draft.is_some(),
        "the unsaved model edit reopens"
    );
    let shortcuts = restored.ui.preferences.shortcuts();
    assert!(
        shortcuts
            .audit(&crate::workbench::commands::vocabulary::CommandRegistry)
            .is_valid(),
        "the shortcut profile comes back as one this build understands: {:?}",
        shortcuts.audit(&crate::workbench::commands::vocabulary::CommandRegistry)
    );
    assert_eq!(shortcuts.policies(), customized_shortcuts().policies());
    assert_same_session(
        &session_value(as_restored(state)),
        &session_value(&restored),
        "eframe RON",
    );
}

/// The production reader accepts a plain value where an option is expected,
/// which is what lets an older session through. A strict reader does not,
/// so it is the one that sees a field this build writes in a shape it does
/// not read.
#[test]
fn a_populated_session_reads_back_strictly() {
    let saved = stored(&mut populated_session());

    ron::from_str::<Box<AppState>>(&saved).expect("strict RON reads the session this build wrote");
}

/// The safe-mode snapshot is the session as JSON, and it is read back
/// through the same deserializer.
#[test]
fn a_populated_session_survives_the_json_snapshot_unchanged() {
    let state = populated_session();
    let written = serde_json::to_string(&state).expect("the session encodes as JSON");

    let restored: AppState = serde_json::from_str(&written).expect("the JSON session reads back");

    assert_same_session(
        &session_value(as_restored(state)),
        &session_value(&restored),
        "JSON",
    );
}

/// A store too small for the runs keeps the session without them; that
/// session restores everything else.
#[test]
fn a_populated_session_kept_without_its_runs_restores_everything_else() {
    let lean = |state: &AppState| {
        ron::ser::to_string(&WithoutRuns(state)).expect("the lean session encodes")
    };
    let state = populated_session();

    let restored = relaunch(lean(&state));

    assert_eq!(restored.simulation.retained.run_count(), 0);
    assert_eq!(
        restored.simulation.retained.next_run_id, 3,
        "the next run is not numbered as the one left out"
    );
    let state = as_restored(state);
    assert_same_session(
        &session_value(WithoutRuns(&state)),
        &session_value(WithoutRuns(&restored)),
        "lean RON",
    );
    // Only the runs are left out.
    let without_runs = |state: &AppState| {
        let mut value = session_value(state);
        value
            .as_object_mut()
            .expect("the session is an object")
            .remove("simulation_results");
        value
    };
    assert_same_session(
        &without_runs(&state),
        &without_runs(&restored),
        "lean RON beside the whole",
    );
}

/// Every field the session writes holds something other than its default in
/// [`populated_session`], so a field added to the wire is covered by the
/// round trips above the day it is added, or this fails naming it.
///
/// The walk covers the application's own fields and the two structs nested
/// in it, `ui_session` and `workbench`. A field written only when it holds a
/// value is seen once the populated session sets it.
#[test]
fn the_populated_session_moves_every_persisted_field_off_its_default() {
    // These name the schema this build writes. A session holding any other
    // value is migrated as it is read, so it cannot round-trip.
    const SCHEMA_VERSIONS: [&str; 2] = [
        "workbench.navigation_schema_version",
        "workbench.layout_schema_version",
    ];

    let default = serde_json::to_value(AppState::default()).expect("the session encodes");
    let populated = serde_json::to_value(populated_session()).expect("the session encodes");

    let mut at_default = Vec::new();
    let mut walked = 0_usize;
    for (scope, default, populated) in [
        ("", &default, &populated),
        (
            "ui_session.",
            &default["ui_session"],
            &populated["ui_session"],
        ),
        ("workbench.", &default["workbench"], &populated["workbench"]),
    ] {
        let default = default.as_object().expect("a wire struct is an object");
        let populated = populated.as_object().expect("a wire struct is an object");
        let keys = default
            .keys()
            .chain(populated.keys())
            .collect::<std::collections::BTreeSet<_>>();
        for key in keys {
            let path = format!("{scope}{key}");
            if SCHEMA_VERSIONS.contains(&path.as_str()) {
                continue;
            }
            walked += 1;
            match (default.get(key), populated.get(key)) {
                (_, None) => at_default.push(format!("{path} (not written)")),
                (Some(default), Some(populated)) if default == populated => at_default.push(path),
                _ => {}
            }
        }
    }

    assert!(walked > 0, "the walk reached no persisted field");
    assert!(
        at_default.is_empty(),
        "populated_session() leaves these persisted fields at their defaults, so no round trip \
         covers them:\n  {}",
        at_default.join("\n  ")
    );
}
