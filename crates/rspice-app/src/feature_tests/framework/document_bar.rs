use crate::product::ObjectRevision;
use crate::state::ViewType;
use crate::state::{
    DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput, NetlistDocument,
    NetlistDocumentId, SimulationRun, SourceLocator,
};
use crate::ui::panels::{WorkbenchIcon, elide_text};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens;
use crate::workbench::chrome::document_bar::code::netlist_workspace_documents;
use crate::workbench::chrome::document_bar::design::icon_for_view;
use crate::workbench::chrome::document_bar::*;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::workbench_session::WorkspaceDocument;
use crate::workbench::state::{Workspace, WorkspaceDocumentId};
use crate::workbench::{AppState, RSpiceApp};

fn multi_document_netlist_state() -> AppState {
    const ROOT: &str = "tabs\n.include \"models/resistor.inc\"\n.end\n";
    const INCLUDE: &str = ".param r=1k\n";
    let locator = SourceLocator::try_new("models/resistor.inc", "resistor.inc").unwrap();
    let dependency = DependencyMetadata::unresolved_direct_to(0, "models/resistor.inc", locator)
        .unwrap()
        .resolve_utf8(INCLUDE.as_bytes().to_vec())
        .unwrap();
    let provenance = GeneratedProvenance::try_new(
        "document-bar-test",
        GenerationInput::new(
            ObjectRevision::INITIAL,
            crate::state::content_digest("document-bar-input"),
        ),
    )
    .unwrap();
    let artifact = GeneratedArtifact::try_from_utf8(
        provenance,
        ROOT.as_bytes().to_vec(),
        vec![dependency],
        Vec::new(),
    )
    .unwrap();
    let generated = NetlistDocument::from_generated(NetlistDocumentId::new(), artifact).unwrap();
    let owned = generated
        .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
        .unwrap();
    let mut state = AppState::default();
    state.workbench.workspace = Workspace::Netlist;
    state.code.code_workspace.page =
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist;
    state.model.netlist_artifacts.generated_source = ROOT.to_owned();
    state.model.netlist_artifacts.generated_document = Some(generated);
    state.code.netlist.owned_document = Some(owned.clone());
    state.model.workspace.content.netlist_source = Some(ROOT.to_owned());
    state.model.workspace.content.netlist_document = Some(owned);
    state.model.simulation.source.netlist_content = ROOT.to_owned();
    state
}

/// Every Design tab is a cell view, so the icon is what tells a module
/// from the schematic and the symbol beside it. A Verilog-A module wore
/// the netlist mark, which named a deck that is not what the tab opens.
#[test]
fn a_veriloga_tab_wears_its_own_icon() {
    let icon = icon_for_view(ViewType::VerilogA);
    assert_eq!(icon, WorkbenchIcon::VerilogA);
    for other in [
        ViewType::Schematic,
        ViewType::Symbol,
        ViewType::Spice,
        ViewType::Verilog,
    ] {
        assert_ne!(icon, icon_for_view(other), "{other:?} shares the mark");
    }
}

#[test]
fn design_strip_retains_the_single_active_cell_view() {
    assert!(!document_strip_visible(Workspace::Design, 0));
    assert!(document_strip_visible(Workspace::Design, 1));
    assert!(document_strip_visible(Workspace::Design, 2));
}

#[test]
fn models_strip_retains_its_single_pinned_root_document() {
    assert!(!document_strip_visible(Workspace::Models, 0));
    assert!(document_strip_visible(Workspace::Models, 1));

    let mut state = AppState::default();
    state.workbench.workspace = Workspace::Models;
    let descriptors = document_descriptors(&state);
    assert_eq!(descriptors.len(), 1);
    assert_eq!(descriptors[0].id, WorkspaceDocumentId::Models);
    assert_eq!(descriptors[0].label, "Models & PDKs");
    assert!(descriptors[0].open);
    assert!(!descriptors[0].closable);
}

#[test]
fn non_design_strips_remain_reserved_for_multiple_open_documents() {
    assert!(!document_strip_visible(Workspace::Project, 0));
    assert!(!document_strip_visible(Workspace::Project, 1));
    assert!(document_strip_visible(Workspace::Project, 2));
}

#[test]
fn netlist_roots_and_dependencies_are_independent_stable_tabs() {
    let mut state = multi_document_netlist_state();
    assert_eq!(
        document_descriptors(&state).len(),
        2,
        "resolved closures are available in the navigator, not opened as thousands of tabs"
    );
    crate::workbench::documents::netlist_document::open_netlist_dependency_from_root(
        &mut state,
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated,
        "models/resistor.inc",
    )
    .unwrap();
    crate::workbench::documents::netlist_document::open_netlist_dependency_from_root(
        &mut state,
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource,
        "models/resistor.inc",
    )
    .unwrap();
    crate::workbench::documents::netlist_document::open_generated_primary(&mut state);
    let descriptors = document_descriptors(&state);
    assert_eq!(descriptors.len(), 4);
    assert!(matches!(
        descriptors[0].id,
        WorkspaceDocumentId::NetlistGenerated(_)
    ));
    assert!(matches!(
        descriptors[1].id,
        WorkspaceDocumentId::NetlistDependency { .. }
    ));
    assert!(matches!(
        descriptors[2].id,
        WorkspaceDocumentId::NetlistOwned(_)
    ));
    assert!(matches!(
        descriptors[3].id,
        WorkspaceDocumentId::NetlistDependency { .. }
    ));
    assert!(descriptors[0].active);

    let owned_dependency = descriptors[3].id.clone();
    assert!(activate_document(&mut state, &owned_dependency));
    assert_eq!(
        state.code.netlist.active_document,
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
    );
    assert_eq!(
        state.code.netlist.active_dependency_identity.as_deref(),
        Some("models/resistor.inc")
    );
    assert_eq!(active_document_id(&state), Some(owned_dependency.clone()));

    let generated_dependency = descriptors[1].id.clone();
    let documents = netlist_workspace_documents(&state);
    assert!(close_document(
        &mut state,
        &generated_dependency,
        &documents
    ));
    assert_eq!(
        active_document_id(&state),
        Some(owned_dependency),
        "closing an inactive include must not switch the active editor"
    );
}

#[test]
fn registry_scopes_active_documents_by_workspace() {
    let mut registry = crate::workbench::state::WorkspaceDocumentRegistry::default();
    let dataset = crate::product::DatasetId::new();

    registry.activate(WorkspaceDocumentId::SimulationPlan);
    registry.activate(WorkspaceDocumentId::ResultDataset(dataset));

    assert_eq!(
        registry.active(Workspace::Simulate),
        Some(&WorkspaceDocumentId::SimulationPlan)
    );
    assert_eq!(
        registry.active(Workspace::Results),
        Some(&WorkspaceDocumentId::ResultDataset(dataset))
    );
    assert!(registry.active(Workspace::Design).is_none());
}

#[test]
fn activating_a_result_document_does_not_force_design_workspace() {
    let mut state = AppState::default();
    state.workbench.workspace = Workspace::Results;
    let run = SimulationRun::new(7);
    let dataset = run.dataset_id;
    state.model.simulation.retained.runs.push(run);

    assert!(activate_document(
        &mut state,
        &WorkspaceDocumentId::ResultDataset(dataset)
    ));
    assert_eq!(state.workbench.workspace, Workspace::Results);
    assert_eq!(state.model.simulation.view.active_run_idx, Some(0));
}

/// The document bar re-activates the open document every frame. Doing so
/// must be inert for the dataset that is already active: re-selecting its
/// run resynchronizes waveforms, and the resulting data-version change
/// retires cursors, the selected trace, the active pane, and pinned
/// readouts — which would make those states impossible to hold at all.
#[test]
fn reactivating_the_active_dataset_disturbs_no_result_state() {
    let mut state = AppState::default();
    state.workbench.workspace = Workspace::Results;
    let mut first = SimulationRun::new(1);
    first.add_analysis(crate::state::AnalysisResult::new(
        1,
        crate::state::AnalysisType::Transient,
        "TRAN",
    ));
    first.add_analysis(crate::state::AnalysisResult::new(
        2,
        crate::state::AnalysisType::Ac,
        "AC",
    ));
    let dataset = first.dataset_id;
    state.model.simulation.retained.runs = vec![first].into();
    assert!(state.model.simulation.select_run(0));
    assert!(state.model.simulation.select_analysis(1));
    let version = state.model.simulation.view.data_version;

    for _ in 0..3 {
        assert!(activate_document_by_id(
            &mut state,
            &WorkspaceDocumentId::ResultDataset(dataset)
        ));
    }

    assert_eq!(
        state.model.simulation.view.data_version, version,
        "re-activating the active dataset must not advance the data version"
    );
    assert_eq!(
        state.model.simulation.view.active_analysis_idx,
        Some(1),
        "re-activation must not snap the analysis selection back to the first"
    );
}

#[test]
fn closing_a_result_tab_keeps_dataset_and_selects_adjacent_document() {
    let mut state = AppState::default();
    state.workbench.workspace = Workspace::Results;
    let first = SimulationRun::new(1);
    let first_dataset = first.dataset_id;
    let second = SimulationRun::new(2);
    let second_dataset = second.dataset_id;
    state.model.simulation.retained.runs = vec![first, second].into();
    assert!(state.model.simulation.select_run(1));
    let documents = visible_documents(&state);

    assert!(close_document(
        &mut state,
        &WorkspaceDocumentId::ResultDataset(second_dataset),
        &documents
    ));
    assert_eq!(
        state.model.simulation.retained.runs.len(),
        2,
        "closing never deletes result data"
    );
    assert_eq!(state.model.simulation.view.active_run_idx, Some(0));
    assert_eq!(
        state.model.simulation.active_run().unwrap().dataset_id,
        first_dataset
    );
    assert!(
        state
            .workbench
            .documents
            .is_closed(&WorkspaceDocumentId::ResultDataset(second_dataset))
    );
    assert_eq!(visible_documents(&state).len(), 1);
}

#[test]
fn document_cycle_and_bulk_close_are_presentation_only() {
    let mut state = AppState::default();
    state.workbench.workspace = Workspace::Results;
    let first = SimulationRun::new(1);
    let first_dataset = first.dataset_id;
    let second = SimulationRun::new(2);
    let second_dataset = second.dataset_id;
    let third = SimulationRun::new(3);
    let third_dataset = third.dataset_id;
    state.model.simulation.retained.runs = vec![first, second, third].into();
    assert!(state.model.simulation.select_run(0));

    assert!(cycle_document(&mut state, false));
    assert_eq!(
        state
            .model
            .simulation
            .active_run()
            .map(|run| run.dataset_id),
        Some(second_dataset)
    );
    assert_eq!(close_other_documents(&mut state), 1);
    assert!(
        state
            .workbench
            .documents
            .is_closed(&WorkspaceDocumentId::ResultDataset(third_dataset))
    );
    assert_eq!(state.model.simulation.retained.runs.len(), 3);

    assert_eq!(close_all_documents(&mut state), 1);
    assert_eq!(
        state
            .model
            .simulation
            .active_run()
            .map(|run| run.dataset_id),
        Some(first_dataset)
    );
    assert!(
        state
            .workbench
            .documents
            .is_closed(&WorkspaceDocumentId::ResultDataset(second_dataset))
    );
    assert_eq!(state.model.simulation.retained.runs.len(), 3);
}

#[test]
fn dirty_and_close_suffixes_never_overlap() {
    assert_eq!(suffix_width(false, false), 0.0);
    assert_eq!(suffix_width(true, false), 13.0);
    assert_eq!(suffix_width(false, true), 25.0);
    assert_eq!(suffix_width(true, true), 38.0);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn long_document_labels_are_measured_and_ellipsized() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut fitted = String::new();
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let font = theme::sans(tokens::FS_1, FontWeight::Regular);
            fitted = elide_text(ui, "precision_sensor_front_end · schematic", &font, 72.0);
            let width = ui
                .painter()
                .layout_no_wrap(fitted.clone(), font, egui::Color32::WHITE)
                .size()
                .x;
            assert!(width <= 72.0);
        });
    });
    assert!(fitted.ends_with('\u{2026}'));
    assert_ne!(fitted, "precision_sensor_front_end · schematic");
}

/// The Simulation Studio is one document, whatever the plan holds.
///
/// It used to open a "<name> \u{b7} setup" tab per analysis instance, and
/// activating one only selected that instance on the Analyses page --
/// which the plan's analysis list on the left does anyway. So a plan with
/// three analyses showed four tabs that said nothing the list did not, and
/// the strip is gone: one document, and `document_strip_visible` hides a
/// lone non-Design document.
#[test]
fn the_simulate_workspace_is_one_document_whatever_the_plan_holds() {
    use crate::simulation::plan::AnalysisKind;

    let mut state = AppState::default();
    state.workbench.workspace = Workspace::Simulate;
    let plan = state
        .model
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("a default state owns a stable analysis plan");
    plan.insert(AnalysisKind::Transient)
        .expect("a plan takes a second transient");
    plan.insert(AnalysisKind::Ac)
        .expect("a plan takes a small-signal sweep");
    assert_eq!(
        plan.instances().len(),
        3,
        "the fixture holds the instances the strip used to tab"
    );

    let documents = available_documents_for_workspace(&state, Workspace::Simulate);
    assert_eq!(
        documents
            .iter()
            .map(|document| document.label.as_str())
            .collect::<Vec<_>>(),
        ["Simulation plan"]
    );
    assert_eq!(documents[0].id, WorkspaceDocumentId::SimulationPlan);
    assert!(
        !document_strip_visible(Workspace::Simulate, documents.len()),
        "one document paints no strip"
    );
}

/// A Design session with the top schematic and `extra` further schematics
/// open, then a Verilog-A module, which is the active document.
#[cfg(not(target_arch = "wasm32"))]
fn design_session(extra: usize) -> RSpiceApp {
    use crate::state::CellViewRef;

    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    let library = app
        .state
        .model
        .workspace
        .content
        .project
        .root_library
        .clone();
    for index in 0..extra {
        app.state.model.workspace.open_view(
            CellViewRef::new(&library, format!("stage_{index}"), "schematic"),
            ViewType::Schematic,
        );
    }
    app.state.model.workspace.open_view(
        CellViewRef::new(&library, "testdev", "veriloga"),
        ViewType::VerilogA,
    );
    app
}

/// Draw the strip alone across a 1440 pt window, as `frame::show` does.
#[cfg(not(target_arch = "wasm32"))]
fn strip_frame(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    let size = egui::vec2(1440.0, 900.0);
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            events,
            ..Default::default()
        },
        |ui| {
            let layout = LayoutSpec::resolve(size.x, size.y, &app.state.workbench);
            show(ui, app, layout);
        },
    )
}

/// A press on a tab opens its document and leaves keyboard focus where it
/// was. Taking focus on a press painted the keyboard focus ring around
/// every tab the pointer chose.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_pressed_tab_activates_without_taking_keyboard_focus() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = design_session(0);
    let top = app.state.model.workspace.content.open_views[0]
        .reference
        .clone();
    assert_ne!(app.state.model.workspace.content.active_view, top);
    strip_frame(&ctx, &mut app, Vec::new());

    // The top schematic is the first tab, at the strip's left edge.
    let at = egui::pos2(40.0, 15.0);
    strip_frame(
        &ctx,
        &mut app,
        vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    strip_frame(
        &ctx,
        &mut app,
        vec![egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    strip_frame(&ctx, &mut app, Vec::new());

    assert_eq!(app.state.model.workspace.content.active_view, top);
    assert_eq!(ctx.memory(|memory| memory.focused()), None);
}

/// The active tab is scrolled into view when the strip holds more tabs
/// than it can show. With the overflow menu gone, a document activated
/// off the strip's edge would otherwise have no visible tab.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_active_tab_past_the_strips_edge_is_scrolled_into_view() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = design_session(14);

    let nodes = settled_strip(&ctx, &mut app);
    let active = nodes
        .iter()
        .find(|node| node.role() == egui::accesskit::Role::Tab && node.is_selected() == Some(true))
        .and_then(egui::accesskit::Node::bounds)
        .expect("the active tab is in the tree");
    let button = hidden_button(&nodes)
        .and_then(egui::accesskit::Node::bounds)
        .expect("tabs are out of view, so the strip lists them");
    assert!(
        active.x0 >= 0.0 && active.x1 <= button.x0 + 0.5,
        "the active tab spans {}..{}, outside the tabs' track 0..{}",
        active.x0,
        active.x1,
        button.x0
    );
}

/// Frame the strip until its scroll has settled, and return what the last
/// frame named for a screen reader.
#[cfg(not(target_arch = "wasm32"))]
fn settled_strip(ctx: &egui::Context, app: &mut RSpiceApp) -> Vec<egui::accesskit::Node> {
    let mut output = strip_frame(ctx, app, Vec::new());
    // The scroll animates; give it the frames it takes.
    for _ in 0..60 {
        output = strip_frame(ctx, app, Vec::new());
    }
    output
        .platform_output
        .accesskit_update
        .expect("accessibility tree")
        .nodes
        .into_iter()
        .map(|(_, node)| node)
        .collect()
}

/// What a screen reader calls `node`: a label, or a plain label's text.
#[cfg(not(target_arch = "wasm32"))]
fn name(node: &egui::accesskit::Node) -> Option<&str> {
    node.label().or_else(|| node.value())
}

#[cfg(not(target_arch = "wasm32"))]
fn hidden_button(nodes: &[egui::accesskit::Node]) -> Option<&egui::accesskit::Node> {
    nodes.iter().find(|node| {
        name(node).is_some_and(|label| {
            label.ends_with(" more open documents") || label == "1 more open document"
        })
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn tab_nodes(nodes: &[egui::accesskit::Node]) -> Vec<&egui::accesskit::Node> {
    nodes
        .iter()
        .filter(|node| node.role() == egui::accesskit::Role::Tab)
        .collect()
}

#[test]
fn a_squeezed_cell_view_tab_shows_its_cell_before_cutting_the_name() {
    let label = "stage_10 · schematic";
    assert_eq!(tab_caption(label, Some("stage_10"), 170.0, 170.0), label);
    assert_eq!(
        tab_caption(label, Some("stage_10"), 170.0, 140.0),
        "stage_10"
    );
    assert_eq!(
        tab_caption("Generated netlist", None, 170.0, 140.0),
        "Generated netlist",
        "a document with no short form is cut as it is"
    );

    let document = WorkspaceDocument {
        id: WorkspaceDocumentId::CellView(crate::state::CellViewRef::new(
            "lib",
            "stage_10",
            "schematic",
        )),
        label: label.to_owned(),
        icon: WorkbenchIcon::Design,
        dirty: false,
    };
    assert_eq!(document.short_label(), Some("stage_10"));
}

#[test]
fn tabs_that_fit_keep_their_own_widths() {
    assert_eq!(shared_width_cap(&[100.0, 120.0], 400.0), None);
    assert_eq!(
        StripFit::resolve(&[100.0, 120.0], 400.0, 40.0).tabs_width,
        400.0
    );
}

#[test]
fn tabs_that_do_not_fit_share_the_strip_and_the_narrow_keep_theirs() {
    assert_eq!(shared_width_cap(&[190.0, 190.0, 190.0], 450.0), Some(150.0));
    // The narrow tab keeps its 60; the other two split what is left.
    assert_eq!(shared_width_cap(&[60.0, 190.0, 190.0], 360.0), Some(150.0));
}

#[test]
fn tabs_never_shrink_past_the_floor_and_the_strip_scrolls_instead() {
    let natural = [190.0; 10];
    assert_eq!(shared_width_cap(&natural, 600.0), Some(TAB_MIN_WIDTH));
    let fit = StripFit::resolve(&natural, 1000.0, 40.0);
    assert!(fit.overflowing);
    assert_eq!(
        fit.tabs_width, 960.0,
        "the hidden-tabs button takes its room"
    );
    assert_eq!(fit.cap, Some(TAB_MIN_WIDTH));

    let fit = StripFit::resolve(&natural[..6], 1000.0, 40.0);
    assert!(!fit.overflowing, "six tabs fit at 166 pt each");
    assert_eq!(fit.tabs_width, 1000.0, "no button, so no room given up");
}

/// Ten tabs too wide for the window together shrink until they fit, so
/// every one stays in view and nothing is offered as hidden.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn tabs_shrink_to_stay_in_view_before_the_strip_scrolls() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = design_session(8);

    let nodes = settled_strip(&ctx, &mut app);
    let tabs = tab_nodes(&nodes);
    assert_eq!(tabs.len(), 10);
    let widest = tabs
        .iter()
        .filter_map(|tab| tab.bounds())
        .map(|bounds| bounds.x1 - bounds.x0)
        .fold(0.0, f64::max);
    assert!(
        widest < f64::from(TAB_MAX_WIDTH) - 1.0,
        "the tabs shrank: the widest is {widest} pt"
    );
    for tab in &tabs {
        let bounds = tab.bounds().expect("a tab has bounds");
        assert!(
            bounds.x0 >= 0.0 && bounds.x1 <= 1440.5,
            "{:?} spans {}..{}",
            tab.label(),
            bounds.x0,
            bounds.x1
        );
    }
    assert!(hidden_button(&nodes).is_none(), "nothing is hidden");
}

/// Past the floor, the strip scrolls and "+N" counts exactly the tabs out
/// of view; choosing one from its menu opens that document.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_hidden_tabs_button_lists_the_tabs_out_of_view_and_opens_one() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = design_session(14);
    let top = app.state.model.workspace.content.open_views[0]
        .reference
        .clone();

    let nodes = settled_strip(&ctx, &mut app);
    let button = hidden_button(&nodes).expect("sixteen tabs overflow 1440 pt");
    let track_end = button.bounds().expect("the button has bounds").x0;
    let out_of_view = tab_nodes(&nodes)
        .into_iter()
        .filter(|tab| {
            let bounds = tab.bounds().expect("a tab has bounds");
            bounds.x0 < -0.5 || bounds.x1 > track_end + 0.5
        })
        .count();
    assert!(out_of_view > 0);
    assert_eq!(
        name(button),
        Some(format!("{out_of_view} more open documents").as_str())
    );

    let press = |ctx: &egui::Context, app: &mut RSpiceApp, at: egui::Pos2| {
        strip_frame(
            ctx,
            app,
            vec![
                egui::Event::PointerMoved(at),
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        strip_frame(
            ctx,
            app,
            vec![egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }],
        )
    };
    let bounds = button.bounds().expect("the button has bounds");
    let at = egui::pos2(
        ((bounds.x0 + bounds.x1) * 0.5) as f32,
        ((bounds.y0 + bounds.y1) * 0.5) as f32,
    );
    press(&ctx, &mut app, at);
    let row = strip_frame(&ctx, &mut app, Vec::new())
        .platform_output
        .accesskit_update
        .expect("accessibility tree")
        .nodes
        .into_iter()
        .map(|(_, node)| node)
        .find(|node| {
            node.role() != egui::accesskit::Role::Tab && name(node) == Some("top · schematic")
        })
        .expect("the scrolled-out first tab is in the menu");
    let bounds = row.bounds().expect("the menu row has bounds");
    let at = egui::pos2(
        ((bounds.x0 + bounds.x1) * 0.5) as f32,
        ((bounds.y0 + bounds.y1) * 0.5) as f32,
    );
    press(&ctx, &mut app, at);
    assert_eq!(app.state.model.workspace.content.active_view, top);

    let nodes = settled_strip(&ctx, &mut app);
    let first = tab_nodes(&nodes)
        .into_iter()
        .find(|tab| name(tab) == Some("top · schematic"))
        .and_then(egui::accesskit::Node::bounds)
        .expect("the opened document has a tab");
    assert!(
        first.x0 >= -0.5,
        "the opened document's tab scrolled into view"
    );
}

/// A session whose Studio tab was a retired setup document opens on the
/// plan.
///
/// The registry is where an activated document identity was written, so a
/// reader upgrading across the deletion has one in their session file.
/// `ron` refuses an unknown enum variant outright and
/// `restore_eframe_session` discards the whole AppState on any error, so
/// the retired identity is read and dropped rather than refused, and the
/// active document falls back to the one document the Studio now has.
#[test]
fn a_session_naming_the_retired_setup_document_opens_on_the_plan() {
    const SESSION: &str = r#"(
        active: {
            Simulate: AnalysisSetup("9f1b6f6c-0f1a-4f2e-9c6d-2a1b3c4d5e6f"),
            Models: Models,
        },
        closed: [AnalysisSetup("9f1b6f6c-0f1a-4f2e-9c6d-2a1b3c4d5e6f")],
    )"#;

    let registry: crate::workbench::state::WorkspaceDocumentRegistry =
        ron::from_str(SESSION).expect("a retired identity costs the reader nothing");
    assert!(
        registry.active(Workspace::Simulate).is_none(),
        "the retired identity is dropped, not restored"
    );
    assert_eq!(
        registry.active(Workspace::Models),
        Some(&WorkspaceDocumentId::Models),
        "its neighbours in the same session survive it"
    );

    let mut state = AppState::default();
    state.workbench.workspace = Workspace::Simulate;
    state.workbench.documents = registry;
    assert_eq!(
        active_document_id(&state),
        Some(WorkspaceDocumentId::SimulationPlan)
    );
}
