//! What a click on this surface actually does.
//!
//! Every other test module here judges what the studio *says*: a route is
//! rendered and the text it painted is read back. That leaves the whole of
//! each control's handler unexercised, and the tests that do reach a command
//! call it directly — which proves the command works and says nothing about
//! whether anything on screen reaches it. A control bound to the wrong
//! command, or to none, passes all of them.
//!
//! So these press the pointer. Each case renders the studio until its layout
//! settles, finds the control by the announcement it publishes to AccessKit —
//! the same string a screen reader is given, so a control that reaches nobody
//! cannot be clicked here either — and presses in the middle of the rectangle
//! that announcement was published with.
//!
//! What each case asserts is the state transition, not that a handler ran: a
//! click on a stack row's switch has to leave the plan holding that change
//! *and* a receipt recording it, because the two together are what
//! `apply_analysis_action` is for.

use egui::vec2;

use crate::workbench::RSpiceApp;
use crate::workbench::state::SimulationPage;

/// A control as its accessibility tree published it: what it announces, and
/// where it is.
type Control = (String, egui::Rect);

/// A press and a release at one point, which is what egui reads as a click.
fn click_events(at: egui::Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(at),
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        },
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        },
    ]
}

/// A studio held open across several clicks on one route.
///
/// It owns the application rather than borrowing one per call: the whole point
/// of a click test is that the frame before the press and the frame that
/// dispatches it are the same session, and a handle that took `&mut RSpiceApp`
/// at every step would be four more places able to mutate every subsystem for
/// the sake of a helper.
struct Studio {
    ctx: egui::Context,
    app: RSpiceApp,
    controls: Vec<Control>,
    size: (f32, f32),
    shell: bool,
}

impl Studio {
    /// Open `page` and settle its layout.
    fn open(app: RSpiceApp, page: SimulationPage, size: (f32, f32)) -> Self {
        Self::opened(app, page, size, false)
    }

    /// Open `page` inside the shell the frame puts around it: the navigator
    /// dock on one side, and the overlay host that draws the plan dialogs.
    ///
    /// Opt-in, and not because the dock is expensive. Drawing an overlay in the
    /// same pass as the synthetic click that opened it lets the modal read that
    /// click as a dismissal — its press and its release arrive in one frame
    /// here, which is not how a pointer behaves — so a case that only wants to
    /// press something on the surface is better off without it. The cases that
    /// need it are the ones about a control on one surface opening a window
    /// hosted by another, which is precisely what cannot be checked otherwise.
    fn open_with_shell(app: RSpiceApp, page: SimulationPage, size: (f32, f32)) -> Self {
        Self::opened(app, page, size, true)
    }

    fn opened(mut app: RSpiceApp, page: SimulationPage, size: (f32, f32), shell: bool) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        if shell {
            // The navigator draws the tree of whichever workspace is active, so
            // a studio shown beside it has to actually be the active one.
            app.state
                .workbench
                .activate(crate::workbench::state::Workspace::Simulate);
        }
        app.state.workbench.simulation_page = page;
        let mut studio = Self {
            ctx,
            app,
            controls: Vec::new(),
            size,
            shell,
        };
        // Twice: the first pass builds the font set and the second lays out
        // against it, and a rectangle measured before the fonts exist is not
        // the rectangle the control ends up in.
        studio.pass(Vec::new());
        studio.pass(Vec::new());
        studio
    }

    /// One rendered pass, and the controls it published.
    fn pass(&mut self, events: Vec<egui::Event>) {
        use crate::workbench::layout::LayoutSpec;

        let app = &mut self.app;
        let shell = self.shell;
        let size = self.size;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(size.0, size.1),
                )),
                events,
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |root| {
                        if shell {
                            // The shell's own dock, resolved the way the shell
                            // resolves it, so the panel this presses is the
                            // panel the product draws.
                            let layout = LayoutSpec::resolve(size.0, size.1, &app.state.workbench);
                            crate::workbench::docks::show_navigator(root, app, layout);
                        }
                        super::show(root, app);
                    });
                if shell {
                    // Where the plan dialogs and the analysis catalogue are
                    // actually drawn. A harness that ran only the surface could
                    // press the control that opens one and see nothing appear,
                    // which is the defect these cases exist to hold shut.
                    super::show_workflow_dialogs(ctx, app);
                }
            },
        );
        self.controls = output
            .platform_output
            .accesskit_update
            .map(|update| {
                update
                    .nodes
                    .iter()
                    .filter_map(|(_, node)| {
                        let label = node.label()?.to_owned();
                        let bounds = node.bounds()?;
                        Some((
                            label,
                            egui::Rect::from_min_max(
                                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                            ),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default();
    }

    /// Click the one control whose announcement satisfies `matches`, then
    /// settle the frame the click produced.
    ///
    /// An ambiguous match is refused rather than resolved by taking the first:
    /// two controls answering one description means the test is not saying
    /// which of them it pressed.
    fn click(&mut self, matches: impl Fn(&str) -> bool) {
        let hits = self
            .controls
            .iter()
            .filter(|(label, _)| matches(label))
            .collect::<Vec<_>>();
        assert_eq!(
            hits.len(),
            1,
            "expected exactly one matching control, found {}; the route announces: {:#?}",
            hits.len(),
            self.controls
                .iter()
                .map(|(label, _)| label.as_str())
                .collect::<Vec<_>>()
        );
        let at = hits[0].1.center();
        self.pass(click_events(at));
        self.pass(Vec::new());
    }

    /// Whether the route currently announces a control matching `matches`.
    fn announces(&self, matches: impl Fn(&str) -> bool) -> bool {
        self.controls.iter().any(|(label, _)| matches(label))
    }
}

/// The desktop size these routes are laid out at.
const DESKTOP: (f32, f32) = (1280.0, 1400.0);

/// Wide enough that the navigator dock and the surface beside it both lay out
/// at their desktop widths rather than at their stacked breakpoints.
const DESKTOP_WITH_SHELL: (f32, f32) = (1600.0, 1400.0);

// ----------------------------------------------------------------- stack rows

/// Selecting an analysis and enabling one are two different clicks on one row.
///
/// The row is a hit target with a second one inside it, so neither can be
/// judged by "the row was clicked". They dispatch different commands —
/// `StackAction::Select` writes the surface's selection, `StackAction::
/// SetEnabled` goes through `apply_analysis_action`, which commits to the plan
/// *and* records a lifecycle receipt — and a row that sent one where it meant
/// the other would still look like it worked.
#[test]
fn a_click_on_a_stack_row_selects_it_and_a_click_on_its_switch_enables_it() {
    let mut app = RSpiceApp::test_instance();
    let (first, second) = {
        let plan = app
            .state
            .sim_setup
            .stable_analysis_plan_mut()
            .expect("the test instance has a stable plan");
        // Two rows, so selecting one and enabling one can be told apart. A
        // second transient needs no prerequisite of its own, which keeps this
        // fixture about the stack rather than about dependency binding.
        while plan.instances().len() < 2 {
            plan.insert(crate::simulation::plan::AnalysisKind::Transient)
                .expect("a transient inserts");
        }
        let rows = plan
            .instances()
            .iter()
            .map(|instance| (instance.id(), instance.enabled()))
            .collect::<Vec<_>>();
        (rows[0], rows[1])
    };
    app.state.workbench.active_analysis_instance = Some(first.0);

    let mut studio = Studio::open(app, SimulationPage::Analyses, DESKTOP);
    let target = format!("instance {}", second.0);
    studio.click(|label| label.starts_with("Select ") && label.contains(&target));
    assert_eq!(
        studio.app.state.workbench.active_analysis_instance,
        Some(second.0),
        "a click on the row body selects that instance"
    );
    assert_eq!(
        studio
            .app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("plan")
            .instance(second.0)
            .expect("instance")
            .enabled(),
        second.1,
        "and selecting a row must not also enable or disable it"
    );

    let receipts_before = studio
        .app
        .state
        .workbench
        .analysis_lifecycle_status
        .sequence();
    studio.click(|label| label.starts_with("Enable ") && label.contains(&target));
    assert_eq!(
        studio
            .app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("plan")
            .instance(second.0)
            .expect("instance")
            .enabled(),
        !second.1,
        "a click on the switch moves the plan"
    );
    assert!(
        studio
            .app
            .state
            .workbench
            .analysis_lifecycle_status
            .sequence()
            > receipts_before,
        "and it goes through the lifecycle command, which receipts what it did"
    );
}

// ------------------------------------------------------------- run-set toolbar

/// Every toolbar action on the run-set route reaches the transaction.
///
/// The two history controls and the preview are this page's only route into
/// `page_runset::commit`, which is where a refused edit is recorded and a
/// preview receipt is written. The model-level tests exercise the actions
/// themselves; nothing exercised the wrapper, so a button bound to the wrong
/// variant would have passed every one of them.
#[test]
fn the_run_set_toolbar_undoes_redoes_and_previews_the_declaration() {
    use crate::simulation::run_set::{RunSetAction, RunSetDimensionKind, dispatch};

    let mut app = RSpiceApp::test_instance();
    // Something to undo, authored through the model so the toolbar is the only
    // thing under test.
    let axis = app
        .state
        .sim_setup
        .run_set
        .dimensions
        .iter()
        .find(|dimension| dimension.kind == RunSetDimensionKind::Temperature)
        .expect("the default run set declares a temperature axis")
        .id
        .clone();
    let declared = |app: &RSpiceApp| {
        app.state
            .sim_setup
            .run_set
            .declared_temperatures_celsius(app.state.sim_setup.reference_pvt)
    };
    let authored = declared(&app).expect("the default axis is a list of temperatures");
    let transaction = dispatch(
        &mut app.state.sim_setup.run_set,
        RunSetAction::SetValues {
            id: axis,
            text: "-55\n27\n150".to_owned(),
        },
        1,
    );
    assert!(transaction.was_adopted(), "{:?}", transaction.receipt);
    assert_eq!(declared(&app), Some(vec![-55.0, 27.0, 150.0]));

    let mut studio = Studio::open(app, SimulationPage::RunSet, DESKTOP);
    studio.click(|label| label == "Undo");
    assert_eq!(
        declared(&studio.app),
        Some(authored.clone()),
        "Undo restores the declaration the edit replaced"
    );

    studio.click(|label| label == "Redo");
    assert_eq!(
        declared(&studio.app),
        Some(vec![-55.0, 27.0, 150.0]),
        "and Redo puts it back"
    );

    assert!(
        studio.app.state.sim_setup.run_set.preview.is_none(),
        "nothing has been forecast yet"
    );
    studio.click(|label| label == "Validate and preview");
    assert!(
        studio.app.state.sim_setup.run_set.preview.is_some(),
        "the preview action freezes a forecast"
    );
}

// ----------------------------------------------------------- save groups card

/// Every control on the capture-groups card, pressed.
///
/// The card is five actions over a ledger, and each routes through
/// `apply_group_command` — the one place a group edit becomes a plan change
/// and a receipt. Nothing called it: the one reorder that was covered was
/// covered by a test that inlined the handler's body, which proves the
/// workspace method works and says nothing about the button above it.
#[test]
fn every_capture_group_command_is_reachable_from_the_card() {
    use crate::state::{
        CaptureGroup, CaptureGroupRule, InstancePath, SavedOutput, SavedOutputCompatibility,
        SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
    };
    use crate::workbench::state::SimulationWorkflowDialog;

    let mut app = RSpiceApp::test_instance();
    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance has a stable plan")
        .id();
    for (name, expression) in [("core_n", "V(x1.n)"), ("edge_n", "V(x2.n)")] {
        let output = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            name,
            expression,
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::SelectedAndFinalPoints,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("valid saved output");
        app.state
            .workspace
            .add_saved_output(plan, output)
            .expect("the plan accepts the output");
    }
    let mut ids = Vec::new();
    for (name, scope) in [("Core rails", "/x1"), ("Edge rails", "/x2")] {
        let mut group = CaptureGroup::new(name).expect("group name");
        group.rules.push(CaptureGroupRule::for_scope(
            InstancePath::parse_legacy(scope).expect("scope"),
        ));
        ids.push(
            app.state
                .workspace
                .add_capture_group(plan, group)
                .expect("the plan accepts the group"),
        );
    }
    let order = |app: &RSpiceApp| {
        app.state
            .workspace
            .plan_data(plan)
            .expect("payload")
            .capture_groups
            .iter()
            .map(|group| group.name.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(order(&app), vec!["Core rails", "Edge rails"]);

    let mut studio = Studio::open(app, SimulationPage::Save, (1400.0, 2400.0));

    // Selecting a row is what arms the four commands that act on one group.
    studio.click(|label| label.contains("Core rails"));
    assert_eq!(
        studio.app.state.workbench.selected_capture_group,
        Some(ids[0]),
        "a click on the ledger row selects that group"
    );

    studio.click(|label| label == "Lower");
    assert_eq!(
        order(&studio.app),
        vec!["Edge rails", "Core rails"],
        "Lower moves the selected group later in resolution order"
    );
    assert_eq!(
        studio.app.state.workbench.selected_capture_group,
        Some(ids[0]),
        "and the selection follows the group it moved"
    );

    studio.click(|label| label == "Raise");
    assert_eq!(
        order(&studio.app),
        vec!["Core rails", "Edge rails"],
        "Raise undoes it"
    );

    studio.click(|label| label == "Edit");
    assert!(
        matches!(
            &studio.app.state.workbench.simulation_workflow,
            Some(SimulationWorkflowDialog::CaptureGroup(draft)) if draft.name == "Core rails"
        ),
        "Edit opens the group editor on the selected group"
    );
    studio.app.state.workbench.simulation_workflow = None;

    studio.click(|label| label == "Add group");
    assert!(
        matches!(
            &studio.app.state.workbench.simulation_workflow,
            Some(SimulationWorkflowDialog::CaptureGroup(draft)) if draft.name.is_empty()
        ),
        "Add group opens the same editor on an unnamed draft"
    );
    studio.app.state.workbench.simulation_workflow = None;

    studio.click(|label| label == "Remove");
    assert_eq!(
        order(&studio.app),
        vec!["Edge rails"],
        "Remove takes the selected group out of the plan"
    );
    assert_eq!(
        studio.app.state.workbench.selected_capture_group, None,
        "and clears a selection that no longer names anything"
    );
    assert!(
        !studio.announces(|label| label.contains("Core rails")),
        "and the ledger stops listing a group the plan no longer holds"
    );
}

// ------------------------------------------------------- analysis editor rows

/// The four editor actions that had no coverage anywhere, pressed.
///
/// `AnalysisAction::Clone`, `Earlier`, `Later` and `Remove` appear exactly
/// once each in the crate — at the button that raises them — and nowhere in
/// any test. Each of them commits to the plan and writes a receipt, so a
/// button bound to the wrong one moves a different instance and says so in
/// the lifecycle strip, confidently.
#[test]
fn the_editor_clones_reorders_and_removes_the_instance_it_is_open_on() {
    use crate::simulation::plan::AnalysisKind;

    let mut app = RSpiceApp::test_instance();
    let names = |app: &RSpiceApp| {
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("plan")
            .instances()
            .iter()
            .map(|instance| instance.id())
            .collect::<Vec<_>>()
    };
    let selected = {
        let plan = app
            .state
            .sim_setup
            .stable_analysis_plan_mut()
            .expect("the test instance has a stable plan");
        while plan.instances().len() < 2 {
            plan.insert(AnalysisKind::Transient)
                .expect("a transient inserts");
        }
        plan.instances()[1].id()
    };
    app.state.workbench.active_analysis_instance = Some(selected);
    let before = names(&app);

    let mut studio = Studio::open(app, SimulationPage::Analyses, DESKTOP);

    studio.click(|label| label == "Clone");
    let cloned = names(&studio.app);
    assert_eq!(
        cloned.len(),
        before.len() + 1,
        "Clone adds one instance to the plan"
    );
    let clone_id = studio
        .app
        .state
        .workbench
        .active_analysis_instance
        .expect("the clone is what the editor is now open on");
    assert!(
        !before.contains(&clone_id),
        "and the editor follows the copy rather than staying on the original"
    );

    let position = |app: &RSpiceApp, id| {
        names(app)
            .iter()
            .position(|candidate| *candidate == id)
            .expect("the instance is in the plan")
    };
    let at = position(&studio.app, clone_id);
    assert!(at > 0, "the clone starts after the instance it copied");
    studio.click(|label| label == "Earlier");
    assert_eq!(
        position(&studio.app, clone_id),
        at - 1,
        "Earlier moves it one place up the plan"
    );
    studio.click(|label| label == "Later");
    assert_eq!(
        position(&studio.app, clone_id),
        at,
        "and Later puts it back where it was"
    );

    studio.click(|label| label == "Remove");
    assert_eq!(
        names(&studio.app),
        before,
        "Remove takes the instance the editor was open on, and only it"
    );
}

// ------------------------------------------------------- the analysis catalogue

/// Adding an analysis from a route that is not the one it lands on.
///
/// The navigator's creating action is drawn on all nine setup routes, and the
/// catalogue it arms was drawn by the Analyses rail — so on the other eight the
/// press set `palette_open` with nothing to render it. That flag is a term of
/// `AppState::application_modal_open`, which gates the whole shortcut
/// dispatcher, so a reader who pressed it from Solver lost every keyboard
/// shortcut in the application and had no painted modal to press Escape on.
///
/// Both halves are asserted here, because either alone would pass over the
/// defect: that the press paints the catalogue, and that choosing from it
/// leaves the plan holding the instance, the reader on the route that edits
/// it, and no modal claim standing behind them.
#[test]
fn adding_an_analysis_from_another_route_opens_the_catalogue_and_lands_on_the_new_instance() {
    use crate::workbench::commands::vocabulary::Command;

    let app = RSpiceApp::test_instance();
    let before = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance has a stable plan")
        .instances()
        .len();

    let mut studio = Studio::open_with_shell(app, SimulationPage::Solver, DESKTOP_WITH_SHELL);
    assert!(
        !studio.announces(|label| label == super::ANALYSIS_CATALOG_SEARCH_LABEL),
        "nothing is open before the press"
    );

    studio.click(|label| label == Command::AddAnalysis.spec().label);
    assert!(
        studio.app.state.sim_setup.palette_open,
        "the press arms the catalogue"
    );
    assert!(
        studio.announces(|label| label == super::ANALYSIS_CATALOG_SEARCH_LABEL),
        "and the frame draws it, from the Solver route it was pressed on"
    );

    studio.click(|label| label == "Add Transient analysis instance");
    let plan = studio
        .app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("the plan still resolves");
    assert_eq!(
        plan.instances().len(),
        before + 1,
        "the row commits the insert"
    );
    let added = studio
        .app
        .state
        .workbench
        .active_analysis_instance
        .expect("the insert selects what it added");
    assert!(
        plan.instance(added).is_some(),
        "and the selection names an instance the plan holds"
    );
    assert_eq!(
        studio.app.state.workbench.simulation_page,
        SimulationPage::Analyses,
        "the reader lands on the one route that can configure it"
    );
    assert!(
        !studio.app.state.sim_setup.palette_open,
        "the catalogue closes behind the choice"
    );
    assert!(
        !studio.app.state.application_modal_open(),
        "and nothing is left claiming exclusive keyboard intent"
    );
}
