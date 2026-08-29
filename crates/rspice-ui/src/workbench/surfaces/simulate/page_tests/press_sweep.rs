//! Every control the studio publishes, pressed.
//!
//! The rest of this suite presses controls it names: a case knows which button
//! it wants, finds it by the string it announces, and checks the one
//! transition that button is for. That leaves the surface's *other* controls —
//! the ones no case happens to name — entirely unexercised. A control wired to
//! nothing renders, announces itself, lays out inside its pane, comes to rest,
//! and passes every gate this crate has.
//!
//! So this sweeps. Every route and every overlay is opened, every node that
//! advertises a press and is not disabled is enumerated, and each one is
//! pressed on a fixture of its own.
//!
//! # What counts as handled
//!
//! A press is handled when at least one of these differs from the same
//! surface, settled, immediately before it:
//!
//! 1. **What the surface publishes** — every node's role, name, value, toggle,
//!    enabled state and bounds. A control appearing, vanishing, renaming,
//!    changing value or moving all count.
//! 2. **Which control holds focus**, as the tree update reports it.
//! 3. **What the frame asked the platform for** — clipboard writes, opened
//!    URLs, an IME region. This is the whole visible product of a control
//!    whose effect leaves the application.
//! 4. **The studio's own state** — `sim_setup` (the plan, the run space, the
//!    solver options), `workbench` (route, selections, open overlays, every
//!    receipt), `dialogs` (the reviews), and the project workspace that holds
//!    each plan's variables, outputs, groups and specifications. This is the
//!    term that catches a press whose effect the current route does not paint.
//!
//! Explicitly *not* accepted: a repaint request, a hover highlight, or a
//! handler that ran and wrote nothing. "It did not panic" is not handled.
//! [`the_handled_predicate_reports_a_press_that_moves_nothing`] holds the
//! predicate to that, by pressing something that genuinely is not a control
//! and requiring the answer `false`.
//!
//! # How a press is delivered
//!
//! By AccessKit action request, not by a synthetic pointer. It is the exact
//! action the node advertises — a control publishing a press a reader can
//! request has to answer it — and it addresses the control by identity, so
//! the sweep cannot silently press whatever happens to be painted over the
//! rectangle it aimed at.
//!
//! Text fields are the one exception, and not by choice: egui strips
//! `FAKE_PRIMARY_CLICKED` from a `TextEdit` response and gates the focus
//! request behind `response.interact_pointer_pos()`, so an AccessKit click on
//! a text field is inert *by egui's design* rather than by anything RSpice
//! did. Those are pressed with a pointer at the rectangle they announced, and
//! judged on the one thing a click on a field is for: focus has to land on
//! exactly that node. Focus landing somewhere else means the press reached a
//! different widget, which is a finding of its own.
//!
//! # What is swept
//!
//! Roles a reader acts on. Not `Role::Label`: egui gives selectable text a
//! click sense so a pointer can place a caret in it, so the studio's prose
//! advertises a press by the dozen. Those are sentences, not controls.
//!
//! Each distinct control is pressed once. Node identity is egui's own widget
//! identity, so the plan toolbar drawn on all nine routes is one control, not
//! nine — and a per-surface tally counts a control on the first surface that
//! publishes it.
//!
//! # One case per surface
//!
//! The presses are independent of one another — each opens its own fixture —
//! so putting all of them in one case bought nothing and cost the whole crate's
//! test run: the studio's surfaces were swept end to end on one thread while
//! every other core stood idle. There is a case per surface instead. Each holds
//! that surface's share of the frozen tallies, [`SWEPT_SURFACES`] records which
//! surface each case covers, and
//! [`every_swept_surface_has_a_case_that_presses_it`] is what keeps a surface
//! the studio grows from having no case at all.
//!
//! What the cases share is the enumeration — thirty-five surfaces built and
//! settled — and that is run once for the whole binary by
//! [`studio_enumeration`]. What it caches is identities, names and rectangles.
//! No `egui::Context` and no application outlive the pass that made them, and
//! every press still starts from a fixture nothing else has touched.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use egui::accesskit::{Action, ActionRequest, Node, NodeId, Role, TreeId};
use egui::{Rect, vec2};

use super::accessibility::{studio_overlays, studio_route, studio_routes};
use super::{AppState, RSpiceApp};

/// The surface size every sweep lays out at.
///
/// Measured, not guessed: [`the_sweep_viewport_reaches_every_control`] holds
/// it at the plateau where widening the surface publishes no further control
/// and nothing is drawn outside it.
const SWEEP_SIZE: (f32, f32) = (1400.0, 2600.0);

/// How many passes a surface is given to reach a fixed point.
const SETTLE_PASSES: usize = 8;

/// Roles a reader acts on.
const CONTROL_ROLES: [Role; 9] = [
    Role::Button,
    Role::CheckBox,
    Role::ComboBox,
    Role::Link,
    Role::TextInput,
    Role::MultilineTextInput,
    Role::RadioButton,
    Role::SpinButton,
    Role::Slider,
];

/// Whether this node is a control a reader can press right now.
fn is_pressable(node: &Node) -> bool {
    CONTROL_ROLES.contains(&node.role())
        && node.supports_action(Action::Click)
        && !node.is_disabled()
        && node.bounds().is_some()
}

/// Whether egui will refuse an AccessKit press on this node.
///
/// A `TextEdit` response has its fake-click flag stripped inside egui, so the
/// request reaches the widget and the widget ignores it. Pressing these with a
/// pointer is not a weaker test, it is the only one available.
fn is_text_field(node: &Node) -> bool {
    matches!(node.role(), Role::TextInput | Role::MultilineTextInput)
}

/// What a control announces, for a failure message a reader can act on.
fn describe(node: &Node) -> String {
    format!("{:?} {:?}", node.role(), announced(node))
}

/// The centre of the rectangle a node announced.
fn centre(node: &Node) -> egui::Pos2 {
    let bounds = node.bounds().expect("a pressable node carries bounds");
    egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    )
}

/// Everything observable of one rendered pass.
///
/// The four terms of the handled predicate, kept apart so a failure can say
/// which of them a press failed to move.
#[derive(PartialEq, Eq)]
struct Observation {
    tree: String,
    focus: NodeId,
    platform: String,
    state: String,
}

impl Observation {
    /// Whether anything a reader could observe moved between these two.
    fn moved_from(&self, before: &Self) -> bool {
        self != before
    }
}

/// The studio held open on one surface, pressed one control at a time.
///
/// Owns the application rather than borrowing one: a helper that took the
/// whole application mutably would be one more of exactly the parameter the
/// layering ratchet counts, and nothing outside this harness needs the fixture
/// afterwards.
struct Sweep {
    ctx: egui::Context,
    app: RSpiceApp,
}

impl Sweep {
    fn open(app: RSpiceApp) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        Self { ctx, app }
    }

    /// One rendered pass: what it published, and what could be observed of it.
    fn pass(&mut self, events: Vec<egui::Event>) -> (Observation, Vec<(NodeId, Node)>) {
        let app = &mut self.app;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(SWEEP_SIZE.0, SWEEP_SIZE.1),
                )),
                events,
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| super::super::show(ui, app));
                // The analysis catalogue and the workflow dialogs are hosted by
                // the frame, not by the surface. A sweep that ran only the
                // surface would watch every control that opens one of them
                // produce nothing at all.
                super::super::show_workflow_dialogs(ctx, app);
            },
        );
        let platform = &output.platform_output;
        // `platform_output.events` is deliberately not read: egui pushes an
        // `OutputEvent::Clicked` for whatever widget a press reached, so a
        // fingerprint holding it would report every press as handled by the
        // bare fact of having been dispatched.
        let observed_platform = format!(
            "{:?}|{:?}|{}",
            platform.commands, platform.ime, platform.mutable_text_under_cursor
        );
        let update = platform.accesskit_update.as_ref();
        let focus = update.map_or(NodeId(0), |update| update.focus);
        let mut nodes = update
            .map(|update| update.nodes.clone())
            .unwrap_or_default();
        nodes.sort_by_key(|(id, _)| *id);
        let mut tree = String::new();
        for (id, node) in &nodes {
            let bounds = node.bounds().unwrap_or(egui::accesskit::Rect {
                x0: 0.0,
                y0: 0.0,
                x1: 0.0,
                y1: 0.0,
            });
            let _ = writeln!(
                tree,
                "{}|{:?}|{}|{}|{:?}|{}|{:.1},{:.1},{:.1},{:.1}",
                id.0,
                node.role(),
                node.label().unwrap_or_default(),
                node.value().unwrap_or_default(),
                node.toggled(),
                node.is_disabled(),
                bounds.x0,
                bounds.y0,
                bounds.x1,
                bounds.y1
            );
        }
        let observation = Observation {
            tree,
            focus,
            platform: observed_platform,
            state: studio_state(&self.app),
        };
        (observation, nodes)
    }

    /// Render until the surface stops changing, and report the rest it came
    /// to.
    ///
    /// A surface that never reaches a fixed point is refused rather than
    /// swept: every press on it would read as handled by its own drift.
    fn settle(&mut self, surface: &str) -> (Observation, Vec<(NodeId, Node)>) {
        let mut previous = None;
        for pass in 1..=SETTLE_PASSES {
            let (observation, nodes) = self.pass(Vec::new());
            if previous.as_ref() == Some(&observation) {
                return (observation, nodes);
            }
            assert!(
                pass < SETTLE_PASSES,
                "{surface} never came to rest in {SETTLE_PASSES} passes; every press on it \
                 would read as handled by its own drift"
            );
            previous = Some(observation);
        }
        unreachable!("the loop returns or the assertion fires")
    }

    /// Request the press this node advertises, then let the frame that answers
    /// it complete.
    ///
    /// Two passes: the first dispatches the request, and the second is the one
    /// a two-frame handshake — a control that stages an answer for the next
    /// frame to apply — needs in order to have happened at all.
    fn press(&mut self, node: NodeId) -> Observation {
        self.pass(vec![egui::Event::AccessKitActionRequest(ActionRequest {
            action: Action::Click,
            target_tree: TreeId::ROOT,
            target_node: node,
            data: None,
        })]);
        self.pass(Vec::new()).0
    }

    /// Press with a pointer, for the fields egui refuses an AccessKit press
    /// on.
    fn pointer_press(&mut self, at: egui::Pos2) -> Observation {
        self.pass(vec![
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
        ]);
        self.pass(Vec::new()).0
    }

    /// Answer whatever is open the way Escape answers it.
    fn escape(&mut self) -> Observation {
        self.pass(vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }]);
        self.pass(Vec::new()).0
    }

    fn modal_open(&self) -> bool {
        self.app.state.application_modal_open()
    }
}

/// Everything of the studio's own state a press could move.
///
/// Rendered whole rather than compared field by field: a hand-written list is
/// a list of the fields somebody thought of, and the point of a sweep is the
/// controls nobody thought of.
fn studio_state(app: &RSpiceApp) -> String {
    let state: &AppState = &app.state;
    format!(
        "{:?}\n{:?}\n{:?}\n{:?}",
        state.sim_setup, state.workbench, state.dialogs, state.workspace
    )
}

/// A voltage source on the sheet.
///
/// The Excitations route reads the schematic rather than the plan, so on the
/// empty design every fixture in this suite starts from it publishes no
/// control at all — only the note saying no source is placed. A source gives
/// it its ledger, and each row that hops to the placed instance is a control
/// like any other.
fn place_a_source(state: &mut AppState) {
    use crate::state::{Component, ComponentType, Point};
    state.schematic.components.push(Component::new(
        1,
        ComponentType::VoltageSource,
        Point::new(120, 120),
    ));
}

/// One surface, rebuilt.
///
/// By index rather than from a list held across presses, because every press
/// starts from a fixture of its own: a route rebuilt here is the route as a
/// reader finds it, not as the previous press left it. The routes are built
/// singly; the overlays share one construction, so that one builds them all
/// and takes the one asked for.
fn sweep_surface(index: usize) -> (String, RSpiceApp) {
    let routes = studio_routes();
    if let Some((page, kind)) = routes.get(index).copied() {
        let name = kind.map_or_else(|| format!("{page:?}"), |kind| format!("{page:?}/{kind:?}"));
        let mut app = studio_route(page, kind);
        place_a_source(&mut app.state);
        return (name, app);
    }
    studio_overlays()
        .into_iter()
        .nth(index - routes.len())
        .expect("the surface index is in range")
}

/// How many surfaces the sweep covers.
fn surface_count() -> usize {
    studio_routes().len() + studio_overlays().len()
}

/// One control the sweep will press: its identity, and the name it announced.
type Target = (NodeId, String);

/// One surface's controls, as it publishes them at rest.
struct SurfacePlan {
    /// Where the surface sits in sweep order, which is how a case rebuilds it.
    index: usize,
    /// What the surface calls itself.
    surface: String,
    /// Every pressable control the surface publishes — including the ones an
    /// earlier surface published first — with what it announced and the
    /// rectangle it announced it at.
    published: Vec<(String, egui::accesskit::Rect)>,
    /// The controls this surface is the first to publish, which are the ones
    /// its case presses.
    targets: Vec<Target>,
}

/// Every distinct control the studio publishes, grouped by the surface that
/// first publishes it, enumerated once for the whole test binary.
///
/// Enumeration only — nothing is pressed here, so the gates that read the tally
/// do not pay for the sweep. It is also the one thing every case in this module
/// needs and none of them can narrow: a control is counted on the first surface
/// that publishes it, so learning which controls one surface owns means
/// settling every surface before it. Thirty-five cases each doing that would
/// cost several times what the presses do.
///
/// Caching it shares no fixture. What survives the enumeration is a list of
/// identities, names and rectangles; every `egui::Context` and every
/// application it opened is dropped with the pass that made it, and each press
/// still builds its own.
fn studio_enumeration() -> &'static [SurfacePlan] {
    static ENUMERATION: std::sync::OnceLock<Vec<SurfacePlan>> = std::sync::OnceLock::new();
    ENUMERATION.get_or_init(|| {
        let mut seen: BTreeSet<u64> = BTreeSet::new();
        let mut plan = Vec::new();
        for index in 0..surface_count() {
            let (surface, app) = sweep_surface(index);
            let mut sweep = Sweep::open(app);
            let (_, nodes) = sweep.settle(&surface);
            let published = nodes
                .iter()
                .filter(|(_, node)| is_pressable(node))
                .map(|(_, node)| {
                    (
                        describe(node),
                        node.bounds().expect("a pressable node carries bounds"),
                    )
                })
                .collect();
            let targets = nodes
                .iter()
                .filter(|(_, node)| is_pressable(node))
                .filter(|(id, _)| seen.insert(id.0))
                .map(|(id, node)| (*id, announced(node)))
                .collect();
            plan.push(SurfacePlan {
                index,
                surface,
                published,
                targets,
            });
        }
        plan
    })
}

/// The name a node announces, which is what a reader reaches it by.
fn announced(node: &Node) -> String {
    let name = node.label().unwrap_or_default();
    if name.trim().is_empty() {
        node.value().unwrap_or_default().to_owned()
    } else {
        name.to_owned()
    }
}

// -------------------------------------------------------------- the tallies
//
// Measured on the branch that introduced this module, by running the sweep and
// reading its own report. Every count is a floor the studio may rise above and
// never fall below; every list is exact, so an entry that stops being true has
// to be deleted rather than left standing and none of them can rot into a
// record of problems that no longer exist.

/// Distinct controls the sweep presses, per surface, in sweep order.
///
/// A control is counted on the first surface that publishes it: identity here
/// is egui's own widget identity, so the plan toolbar drawn on all nine setup
/// routes is one control and not nine. The zeroes are honest — nine of the ten
/// analysis-catalogue fixtures open the same window the tenth already
/// contributed, and the Outputs registry's own actions are all disabled over
/// an empty registry, so its enabled controls are the toolbar's.
const PRESSED_PER_SURFACE: &[(&str, usize)] = &[
    ("Analyses/Transient", 31),
    ("Analyses/Ac", 3),
    ("Analyses/DcSweep", 2),
    ("Analyses/Noise", 9),
    ("Analyses/Stb", 3),
    ("Analyses/Pss", 3),
    ("Analyses/Temperature", 2),
    ("Analyses/Corner", 2),
    ("Excitations", 1),
    ("Variables", 2),
    ("Outputs", 0),
    ("Specifications", 4),
    ("RunSet", 31),
    ("Models", 1),
    ("Solver", 33),
    ("Save", 6),
    ("analysis catalogue · Analyses", 29),
    ("analysis catalogue · Excitations", 0),
    ("analysis catalogue · Variables", 0),
    ("analysis catalogue · Outputs", 0),
    ("analysis catalogue · Specifications", 0),
    ("analysis catalogue · RunSet", 0),
    ("analysis catalogue · Models", 0),
    ("analysis catalogue · Solver", 0),
    ("analysis catalogue · Save", 0),
    ("analysis catalogue · Results workspace", 0),
    ("advanced options", 26),
    ("plan manager", 12),
    ("rename analysis", 4),
    ("run points", 12),
    ("design variable", 11),
    ("saved output", 9),
    ("clone plan", 8),
    ("capture group", 7),
    ("design variable import", 12),
];

/// Distinct controls the sweep presses in total.
const PRESSED_FLOOR: usize = 263;

/// Controls that are wired to nothing.
///
/// Empty, and still a list rather than a boolean, because the list is what
/// makes this a gate a reader can act on: each surface's case compares its own
/// findings against the entries naming that surface, so a control the studio
/// grows and wires to nothing fails under its own name instead of moving a
/// tally.
///
/// It held twelve entries when the sweep was first measured, and all twelve
/// were one defect rather than twelve. `page_kit::ledger_row` used
/// `Sense::click` and published
/// `WidgetInfo::selected(WidgetType::SelectableLabel, …)` for every row it
/// drew, whether or not the caller read the click — so nine solver policy rows,
/// a refused advanced option, the capture ledger's fallback group and the
/// models gate's "nothing to gate" placeholder each announced themselves to a
/// screen reader as a selectable button whose press did nothing. The row now
/// takes a `page_kit::RowPress` from its caller: a row whose click nothing
/// reads senses hover and announces itself as a label, so it advertises no
/// press to answer and this sweep no longer enumerates it at all. The twelve
/// left together, which is why four surfaces' press counts fell with them.
const DEAD_CONTROLS: &[&str] = &[];

/// How many presses the segmented-selection arm excuses.
///
/// Two, and both are a row that already announces itself as the selection: the
/// analysis stack's selected instance, and the plan manager's active plan.
/// Neither is named here because both announce a fresh identity every time the
/// fixture is built.
const HELD_SELECTION_CEILING: usize = 2;

/// Presses whose whole product leaves this process.
///
/// `page_variables.rs`'s import opens a spec-sheet picker through
/// `io::file_exchange::open_file`, and the picker's answer is delivered into
/// egui's own data store rather than into the application state. In a test
/// build the picker is answered by the scripted seam rather than by a desktop
/// dialog, and an unscripted press is a cancellation — which a reader's own
/// cancellation is too, and which by design leaves nothing behind. The press
/// reaches its handler; nothing this sweep can read records that it did.
///
/// [`the_sweep_press_that_starts_a_picker_answers_without_a_dialog`] is the
/// evidence behind that sentence: it names this exact control as the only one
/// the sweep reaches that starts a picker, and shows that a scripted sheet does
/// come back through it. Exact, like the others: if the harness ever grows a
/// way to see this, the entry has to go.
const PRESSES_THIS_HARNESS_CANNOT_SEE: &[&str] = &["Variables: Button \"Import…\""];

/// How many sweep presses open a modal.
const MODAL_OPENERS_FLOOR: usize = 7;

/// The two whole-sweep numbers above, split by the surface that produced them:
/// segments excused for holding their selection, and presses that opened a
/// modal.
///
/// A surface absent from this table excuses nothing and opens nothing, which is
/// what makes the split stronger than the totals it replaced rather than weaker
/// — an excused press that migrates to another surface now fails on both
/// surfaces instead of cancelling out.
/// [`every_swept_surface_has_a_case_that_presses_it`] holds the two columns to
/// summing to [`HELD_SELECTION_CEILING`] and [`MODAL_OPENERS_FLOOR`], so
/// neither total can be quietly re-cut.
const SEGMENTS_AND_MODALS_PER_SURFACE: &[(&str, usize, usize)] = &[
    ("Analyses/Transient", 1, 4),
    ("Analyses/Stb", 0, 1),
    ("Variables", 0, 1),
    ("Save", 0, 1),
    ("plan manager", 1, 0),
];

/// Modals a sweep press opened that Escape does not close here.
///
/// One, and it is this harness's limit rather than the studio's: "Validate
/// plan" runs `Command::PreflightChecks`, and the preflight report is drawn by
/// `workbench::frame`, which these surface-and-overlay passes do not run.
/// There is no dialog on screen for Escape to reach.
const MODALS_ESCAPE_DOES_NOT_CLOSE: &[&str] = &["Analyses/Transient: Button \"Validate plan\""];

/// How many destructive controls the sweep finds.
///
/// Two, and the number is small for a reason worth stating: the fixtures the
/// sweep opens seed no design variables, saved outputs or capture groups, so
/// each of those registries draws its `Remove` disabled and the sweep never
/// enumerates it. Those four routes are pressed exhaustively by
/// `plan_removal`, which seeds a record precisely so it can remove one. This
/// floor guards the destructive controls a *default* plan reaches, and is not
/// a claim to cover the registries.
const DESTRUCTIVE_FLOOR: usize = 2;

// ----------------------------------------------------------------- one press

/// What one press did.
enum Answer {
    /// Something a reader could observe moved.
    Handled,
    /// The control already announced itself as its group's chosen option, and
    /// the press left it chosen. A live segment of a segmented control is not
    /// dead for declining to change what it already says — clearing the
    /// selection would be the defect, and this is the arm that says so.
    HeldItsSelection,
    /// Nothing a reader could observe moved.
    Dead,
    /// A pointer press on a text field put focus somewhere else, which means it
    /// reached a different widget than the one it aimed at.
    Misdirected(NodeId),
}

/// One press, and what the studio did about it.
struct Press {
    described: String,
    answer: Answer,
    opened_a_modal: bool,
    escaped: bool,
}

/// Press one control on a fixture of its own, and report what it did.
///
/// A modal the press raised is answered with Escape here rather than in a
/// sweep of its own, because it is the same press: opening a second fixture to
/// press the same control again would double the cost of the sweep to learn
/// something this pass already knows.
fn press_one(index: usize, surface: &str, target: NodeId) -> Press {
    let (name, app) = sweep_surface(index);
    assert_eq!(
        name, surface,
        "the surface order must be stable across rebuilds, or a press lands on another route"
    );
    let mut sweep = Sweep::open(app);
    let (before, nodes) = sweep.settle(surface);
    let (_, node) = nodes
        .iter()
        .find(|(id, _)| *id == target)
        .expect("a rebuilt surface republishes the control the plan enumerated");
    let described = describe(node);
    let holds_a_selection =
        node.role() == Role::Button && node.toggled() == Some(egui::accesskit::Toggled::True);
    if is_text_field(node) {
        let at = centre(node);
        let after = sweep.pointer_press(at);
        let answer = if after.focus == target {
            Answer::Handled
        } else {
            Answer::Misdirected(after.focus)
        };
        return Press {
            described,
            answer,
            opened_a_modal: false,
            escaped: true,
        };
    }
    let modal_before = sweep.modal_open();
    let after = sweep.press(target);
    let answer = if after.moved_from(&before) {
        Answer::Handled
    } else if holds_a_selection {
        Answer::HeldItsSelection
    } else {
        Answer::Dead
    };
    let opened_a_modal = !modal_before && sweep.modal_open();
    let escaped = if opened_a_modal {
        sweep.escape();
        !sweep.modal_open()
    } else {
        true
    };
    Press {
        described,
        answer,
        opened_a_modal,
        escaped,
    }
}

// -------------------------------------------------------------------- sweeps

/// The frozen tallies one surface's case holds itself to: the controls it
/// pressed when the sweep was measured, how many of those it may excuse as a
/// segment holding its selection, and how many of them opened a modal.
fn frozen_tallies_for(surface: &str) -> (usize, usize, usize) {
    let pressed = PRESSED_PER_SURFACE
        .iter()
        .find(|&&(name, _)| name == surface)
        .map_or_else(
            || panic!("{surface} is swept and is not in PRESSED_PER_SURFACE"),
            |&(_, count)| count,
        );
    let (held, modals) = SEGMENTS_AND_MODALS_PER_SURFACE
        .iter()
        .find(|&&(name, _, _)| name == surface)
        .map_or((0, 0), |&(_, held, modals)| (held, modals));
    (pressed, held, modals)
}

/// The entries of one frozen list that name this surface.
///
/// Every entry leads with the surface that produced it and the lists are sorted
/// whole, so one surface's entries are contiguous and already in order: a case
/// comparing its own sorted findings against this slice is making exactly the
/// claim the undivided list made. An entry naming a surface nothing sweeps
/// would go unchecked, which is what
/// [`every_swept_surface_has_a_case_that_presses_it`] refuses.
fn frozen_entries_for(list: &[&'static str], surface: &str) -> Vec<&'static str> {
    let prefix = format!("{surface}: ");
    list.iter()
        .copied()
        .filter(|entry| entry.starts_with(&prefix))
        .collect()
}

/// Every control one surface publishes answers the press it advertises, and
/// every modal one of those presses raises can be escaped again.
///
/// This is the gate SS-20 exists for. Around a hundred and fifty of the
/// studio's controls had never been pressed by anything: they were rendered,
/// named, measured for overflow and watched for drift, and no test asked
/// whether pressing one reached a handler.
///
/// Both claims are measured from the one press, the way the accessibility
/// sweep measures naming and overflow from the one fixture — a control that
/// raises a modal has to be pressed to find out, and pressing it again on a
/// second fixture to ask the second question would double a sweep that already
/// knows the answer.
///
/// The body one case per surface runs. A case names its own surface as well as
/// its index so that a reordered sweep fails here rather than pressing another
/// route's controls under this one's tallies.
fn press_every_control_on(index: usize, surface: &str) {
    let plan = studio_enumeration()
        .get(index)
        .unwrap_or_else(|| panic!("the sweep no longer reaches surface {index}, {surface}"));
    assert_eq!(
        plan.surface, surface,
        "the surface order must be stable across the binary, or this case is pressing another \
         route's controls"
    );

    let mut dead = Vec::new();
    let mut misdirected = Vec::new();
    let mut unseeable = Vec::new();
    let mut stuck = Vec::new();
    let mut held = 0usize;
    let mut opened = 0usize;
    let mut pressed = 0usize;
    for (target, _) in &plan.targets {
        let press = press_one(index, surface, *target);
        let entry = format!("{surface}: {}", press.described);
        pressed += 1;
        match press.answer {
            Answer::Handled => {}
            Answer::HeldItsSelection => held += 1,
            Answer::Dead => {
                if PRESSES_THIS_HARNESS_CANNOT_SEE.contains(&entry.as_str()) {
                    unseeable.push(entry.clone());
                } else {
                    dead.push(entry.clone());
                }
            }
            Answer::Misdirected(landed) => {
                misdirected.push(format!(
                    "{entry} was pressed and focus landed on {landed:?}"
                ));
            }
        }
        if press.opened_a_modal {
            opened += 1;
            if !press.escaped {
                stuck.push(entry);
            }
        }
    }

    misdirected.sort();
    assert!(
        misdirected.is_empty(),
        "text fields whose press reached another widget:\n{}",
        misdirected.join("\n")
    );

    dead.sort();
    let dead: Vec<&str> = dead.iter().map(String::as_str).collect();
    assert_eq!(
        dead,
        frozen_entries_for(DEAD_CONTROLS, surface),
        "{surface}'s dead controls changed.\nA control listed in DEAD_CONTROLS is wired to \
         nothing: pressing it moves no state, opens nothing, stages no receipt, moves no focus \
         and asks the platform for nothing. A control that has left the list has been repaired \
         — delete its line."
    );

    unseeable.sort();
    let unseeable: Vec<&str> = unseeable.iter().map(String::as_str).collect();
    assert_eq!(
        unseeable,
        frozen_entries_for(PRESSES_THIS_HARNESS_CANNOT_SEE, surface),
        "a press this harness declared it could not see is now visible to it, or has stopped \
         being reachable; either way the exemption has to go"
    );

    stuck.sort();
    let stuck: Vec<&str> = stuck.iter().map(String::as_str).collect();
    assert_eq!(
        stuck,
        frozen_entries_for(MODALS_ESCAPE_DOES_NOT_CLOSE, surface),
        "modals a sweep press opened and Escape did not close"
    );

    let (pressed_floor, held_ceiling, modal_floor) = frozen_tallies_for(surface);
    assert!(
        pressed >= pressed_floor,
        "{surface} pressed {pressed} controls, against {pressed_floor} when it was measured; it \
         has stopped reaching controls it claims to cover"
    );
    assert!(
        held <= held_ceiling,
        "{held} of {surface}'s presses were excused as a live segment holding its selection, \
         against a measured {held_ceiling}; that arm is a carve-out for segmented controls, not \
         a place for dead ones to hide"
    );
    assert!(
        opened >= modal_floor,
        "only {opened} of {surface}'s presses opened a modal, against a measured {modal_floor}; \
         this gate is no longer reaching the controls that raise one"
    );
}

/// One case per surface, and the record of which surface each case covers.
///
/// Spelled out rather than generated, because a test function has to exist at
/// compile time and nothing here can read the enumeration before the binary is
/// built. A surface the studio grows and this list does not is a surface
/// nothing presses, which is the hole
/// [`every_swept_surface_has_a_case_that_presses_it`] stands in.
macro_rules! surface_press_cases {
    ($(($index:expr, $surface:expr, $case:ident),)*) => {
        /// Every surface a case presses, in sweep order.
        const SWEPT_SURFACES: &[(usize, &str)] = &[$(($index, $surface),)*];

        $(
            #[test]
            fn $case() {
                press_every_control_on($index, $surface);
            }
        )*
    };
}

surface_press_cases! {
    (0, "Analyses/Transient", every_control_on_the_transient_form_answers_its_press),
    (1, "Analyses/Ac", every_control_on_the_ac_form_answers_its_press),
    (2, "Analyses/DcSweep", every_control_on_the_dc_sweep_form_answers_its_press),
    (3, "Analyses/Noise", every_control_on_the_noise_form_answers_its_press),
    (4, "Analyses/Stb", every_control_on_the_stability_form_answers_its_press),
    (5, "Analyses/Pss", every_control_on_the_pss_form_answers_its_press),
    (6, "Analyses/Temperature", every_control_on_the_temperature_form_answers_its_press),
    (7, "Analyses/Corner", every_control_on_the_corner_form_answers_its_press),
    (8, "Excitations", every_control_on_the_excitations_route_answers_its_press),
    (9, "Variables", every_control_on_the_variables_route_answers_its_press),
    (10, "Outputs", every_control_on_the_outputs_route_answers_its_press),
    (11, "Specifications", every_control_on_the_specifications_route_answers_its_press),
    (12, "RunSet", every_control_on_the_run_set_route_answers_its_press),
    (13, "Models", every_control_on_the_models_route_answers_its_press),
    (14, "Solver", every_control_on_the_solver_route_answers_its_press),
    (15, "Save", every_control_on_the_save_route_answers_its_press),
    (16, "analysis catalogue · Analyses",
        every_control_on_the_catalogue_over_analyses_answers_its_press),
    (17, "analysis catalogue · Excitations",
        every_control_on_the_catalogue_over_excitations_answers_its_press),
    (18, "analysis catalogue · Variables",
        every_control_on_the_catalogue_over_variables_answers_its_press),
    (19, "analysis catalogue · Outputs",
        every_control_on_the_catalogue_over_outputs_answers_its_press),
    (20, "analysis catalogue · Specifications",
        every_control_on_the_catalogue_over_specifications_answers_its_press),
    (21, "analysis catalogue · RunSet",
        every_control_on_the_catalogue_over_the_run_set_answers_its_press),
    (22, "analysis catalogue · Models",
        every_control_on_the_catalogue_over_models_answers_its_press),
    (23, "analysis catalogue · Solver",
        every_control_on_the_catalogue_over_the_solver_answers_its_press),
    (24, "analysis catalogue · Save",
        every_control_on_the_catalogue_over_save_answers_its_press),
    (25, "analysis catalogue · Results workspace",
        every_control_on_the_catalogue_over_the_results_workspace_answers_its_press),
    (26, "advanced options", every_control_on_the_advanced_options_panel_answers_its_press),
    (27, "plan manager", every_control_on_the_plan_manager_answers_its_press),
    (28, "rename analysis", every_control_on_the_rename_analysis_dialog_answers_its_press),
    (29, "run points", every_control_on_the_run_points_picker_answers_its_press),
    (30, "design variable", every_control_on_the_design_variable_dialog_answers_its_press),
    (31, "saved output", every_control_on_the_saved_output_dialog_answers_its_press),
    (32, "clone plan", every_control_on_the_clone_plan_dialog_answers_its_press),
    (33, "capture group", every_control_on_the_capture_group_dialog_answers_its_press),
    (34, "design variable import",
        every_control_on_the_design_variable_import_dialog_answers_its_press),
}

/// The sweep is divided into cases and nothing falls between them.
///
/// Splitting one case into thirty-five moved three things that used to be
/// checked once into places that are only checked if something reaches them, so
/// each is re-made here as a claim about the division itself:
///
/// 1. Every surface the studio enumerates has a case, at the index and under
///    the name that case declares. A surface with no case would be swept by
///    nothing at all, and the frozen coverage tally would go on passing.
/// 2. Every entry of every frozen list of names belongs to a surface some case
///    presses. An entry naming a surface nothing sweeps is an exemption no
///    case can ever refuse.
/// 3. The per-surface numbers still add up to the whole-sweep numbers they were
///    cut from, so the division cannot quietly relax a total.
///
/// Enumeration only, and it shares the enumeration every case shares, so it is
/// the cheap gate a newly added surface trips first.
#[test]
fn every_swept_surface_has_a_case_that_presses_it() {
    let enumerated: Vec<(usize, &str)> = studio_enumeration()
        .iter()
        .map(|plan| (plan.index, plan.surface.as_str()))
        .collect();
    assert_eq!(
        enumerated, SWEPT_SURFACES,
        "the studio's surfaces and the cases that press them have come apart. A surface listed \
         here and not enumerated no longer exists; a surface enumerated and not listed is swept \
         by nothing."
    );

    let swept: Vec<&str> = SWEPT_SURFACES.iter().map(|&(_, name)| name).collect();
    for (list, name) in [
        (DEAD_CONTROLS, "DEAD_CONTROLS"),
        (
            PRESSES_THIS_HARNESS_CANNOT_SEE,
            "PRESSES_THIS_HARNESS_CANNOT_SEE",
        ),
        (MODALS_ESCAPE_DOES_NOT_CLOSE, "MODALS_ESCAPE_DOES_NOT_CLOSE"),
    ] {
        let partitioned: usize = swept
            .iter()
            .map(|surface| frozen_entries_for(list, surface).len())
            .sum();
        assert_eq!(
            partitioned,
            list.len(),
            "an entry of {name} names a surface no case presses, so no case can ever refuse it"
        );
    }

    let pressed: usize = PRESSED_PER_SURFACE.iter().map(|&(_, count)| count).sum();
    assert_eq!(
        pressed, PRESSED_FLOOR,
        "the per-surface press tally no longer adds up to the whole sweep's floor"
    );
    let held: usize = SEGMENTS_AND_MODALS_PER_SURFACE
        .iter()
        .map(|&(_, held, _)| held)
        .sum();
    assert_eq!(
        held, HELD_SELECTION_CEILING,
        "the per-surface segment ceilings no longer add up to the whole sweep's ceiling"
    );
    let modals: usize = SEGMENTS_AND_MODALS_PER_SURFACE
        .iter()
        .map(|&(_, _, modals)| modals)
        .sum();
    assert_eq!(
        modals, MODAL_OPENERS_FLOOR,
        "the per-surface modal floors no longer add up to the whole sweep's floor"
    );
    for &(surface, _, _) in SEGMENTS_AND_MODALS_PER_SURFACE {
        assert!(
            swept.contains(&surface),
            "{surface} carries a segment or modal tally and no case presses it"
        );
    }
}

/// Nothing in the studio removes a record without saying so.
///
/// Wave 2 put a review in front of every removal that orphans something, and
/// the four registries commit an orphan-free one on the click that asks for
/// it. Both are legitimate; committing in silence is not. So each destructive
/// control is pressed and required to have done one of the two — staged a
/// review, or written the receipt that names what it removed. A staged review
/// is then cancelled, and the plan has to be exactly what it was.
#[test]
fn every_destructive_press_is_reviewed_or_receipted() {
    let mut silent = Vec::new();
    let mut cost = Vec::new();
    let mut destructive = 0usize;
    for plan in studio_enumeration() {
        for (target, label) in &plan.targets {
            if !is_destructive(label) {
                continue;
            }
            let (name, app) = sweep_surface(plan.index);
            let mut sweep = Sweep::open(app);
            let (_, nodes) = sweep.settle(&name);
            let Some((_, node)) = nodes.iter().find(|(id, _)| id == target) else {
                continue;
            };
            let entry = format!("{}: {}", plan.surface, describe(node));
            destructive += 1;
            let plan_before = sweep.plan_records();
            let receipt_before = sweep.receipt();
            sweep.press(*target);
            let staged = sweep.review_open();
            if !staged && sweep.receipt() == receipt_before {
                silent.push(entry.clone());
            }
            if staged {
                sweep.cancel_review();
                if sweep.plan_records() != plan_before {
                    cost.push(entry);
                }
            }
        }
    }
    silent.sort();
    assert!(
        silent.is_empty(),
        "destructive controls that neither staged a review nor receipted what they did:\n{}",
        silent.join("\n")
    );
    cost.sort();
    assert!(
        cost.is_empty(),
        "removals that cost the plan something even though the review was cancelled:\n{}",
        cost.join("\n")
    );
    assert!(
        destructive >= DESTRUCTIVE_FLOOR,
        "the sweep found {destructive} destructive controls, against a measured \
         {DESTRUCTIVE_FLOOR}; it is not reaching the registries it claims to cover"
    );
}

/// The one sweep press that starts a file picker answers inside the frame.
///
/// `page_variables.rs`'s "Import…" is the only control any surface this sweep
/// opens reaches `rfd` from: every other picker in the studio is behind a route
/// the sweep's fixtures do not stand on — the plan manager's package import and
/// export are drawn only in its exchange mode, and its draft opens in Browse.
/// That one control used to raise a real Open dialog on the developer's desktop
/// and hold the test thread until a person closed it, so an unattended sweep
/// hung and an attended one recorded whichever file the person picked.
///
/// Both halves are checked from the press itself. Scripting a cancellation and
/// finding the script consumed is what says the press reached the seam rather
/// than a dialog; scripting a sheet and finding the guided import open on it is
/// what says the seam is the whole path, and not a stub that swallows the
/// press.
#[test]
fn the_sweep_press_that_starts_a_picker_answers_without_a_dialog() {
    use crate::io::file_exchange::{
        ScriptedChoice, script_next_choice, scripted_choices_remaining, take_pickers_opened,
    };
    use crate::workbench::state::SimulationWorkflowDialog;

    let index = (0..surface_count())
        .find(|&index| sweep_surface(index).0 == "Variables")
        .expect("the sweep covers the Variables route");
    fn import_control(nodes: &[(NodeId, Node)]) -> NodeId {
        nodes
            .iter()
            .find(|(_, node)| is_pressable(node) && announced(node) == "Import…")
            .map(|(id, _)| *id)
            .expect("the Variables route publishes the import control")
    }

    // A cancellation, which is what a reader who closes the picker gives.
    let (name, app) = sweep_surface(index);
    let mut sweep = Sweep::open(app);
    let (_, nodes) = sweep.settle(&name);
    let import = import_control(&nodes);
    take_pickers_opened();
    script_next_choice(ScriptedChoice::Cancelled);
    sweep.press(import);
    assert_eq!(
        take_pickers_opened(),
        ["Design variable spec sheet"],
        "the press did not reach the picker seam, so on a desktop build it reached a dialog"
    );
    assert_eq!(
        scripted_choices_remaining(),
        0,
        "the picker took the answer"
    );
    assert!(
        sweep.app.state.workbench.simulation_workflow.is_none(),
        "a cancelled pick opened a dialog"
    );

    // And a sheet, on a fixture of its own, which is the whole point of the
    // control.
    let directory =
        std::env::temp_dir().join(format!("rspice-press-sweep-picker-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("the scratch directory is made");
    let sheet = directory.join("variables.csv");
    std::fs::write(&sheet, "Name,Value\nvdd,1.8\n").expect("the scratch sheet is written");

    let (name, app) = sweep_surface(index);
    let mut sweep = Sweep::open(app);
    let (_, nodes) = sweep.settle(&name);
    let import = import_control(&nodes);
    script_next_choice(ScriptedChoice::Chose(sheet));
    sweep.press(import);
    assert_eq!(
        scripted_choices_remaining(),
        0,
        "the picker took the answer"
    );
    assert!(
        matches!(
            sweep.app.state.workbench.simulation_workflow,
            Some(SimulationWorkflowDialog::DesignVariableImport(_))
        ),
        "the picked sheet did not open the guided import"
    );

    std::fs::remove_dir_all(&directory).ok();
}

// ----------------------------------------------------------------- the gates

/// The sweep still covers every control it covered when it was measured.
///
/// Per surface, because a total alone hides a route that stopped rendering
/// behind another that grew. Enumeration only — this presses nothing, so it
/// stays cheap enough to be the gate a newly added control trips first.
///
/// It also holds the premise the tally rests on: that a node identity means
/// one control. egui derives a widget's automatic identity from where it sits
/// in its layout, so two routes drawn into the same panel could in principle
/// give their nth control the same identity under different names, and the
/// sweep would press one of them and count the other as covered. Every
/// identity the studio publishes has to answer to exactly one name.
#[test]
fn the_press_sweep_covers_every_control_it_covered_before() {
    let plan = studio_enumeration();
    let mut names: std::collections::BTreeMap<u64, BTreeSet<String>> =
        std::collections::BTreeMap::new();
    for surface in plan {
        for (id, label) in &surface.targets {
            names.entry(id.0).or_default().insert(label.clone());
        }
    }
    let shared: Vec<String> = names
        .iter()
        .filter(|(_, labels)| labels.len() > 1)
        .map(|(id, labels)| format!("{id} answers to {labels:?}"))
        .collect();
    assert!(
        shared.is_empty(),
        "one widget identity is announcing more than one name, so the sweep is pressing one \
         control and counting another as covered:\n{}",
        shared.join("\n")
    );

    let measured: Vec<(&str, usize)> = plan
        .iter()
        .map(|surface| (surface.surface.as_str(), surface.targets.len()))
        .collect();
    let mut shortfalls = Vec::new();
    for &(surface, floor) in PRESSED_PER_SURFACE {
        match measured
            .iter()
            .find(|&&(name, _)| name == surface)
            .map(|&(_, count)| count)
        {
            None => shortfalls.push(format!(
                "{surface} is in the tally and the studio no longer publishes it"
            )),
            Some(count) if count < floor => shortfalls.push(format!(
                "{surface} publishes {count} controls, and {floor} were pressed when this was \
                 measured"
            )),
            Some(_) => {}
        }
    }
    for &(surface, count) in &measured {
        if !PRESSED_PER_SURFACE.iter().any(|&(name, _)| name == surface) {
            shortfalls.push(format!(
                "{surface} publishes {count} controls and is not in the tally at all"
            ));
        }
    }
    assert!(
        shortfalls.is_empty(),
        "the press sweep's coverage moved:\n{}\n\nA new surface, or a new control on an old one, \
         is recorded in PRESSED_PER_SURFACE with the count the sweep now presses. A count that \
         fell means controls stopped being reachable.",
        shortfalls.join("\n")
    );
    let total: usize = measured.iter().map(|(_, count)| count).sum();
    assert!(
        total >= PRESSED_FLOOR,
        "the sweep enumerates {total} distinct controls, against a measured {PRESSED_FLOOR}"
    );
}

/// The surface the sweep lays out at draws every control inside itself.
///
/// The evidence behind [`SWEEP_SIZE`], kept as a test rather than as a comment
/// so a layout change that starts clipping a control at the sweep's own width
/// fails here rather than quietly narrowing what the sweep can see. A clipped
/// control is one the sweep would go on pressing by identity while no reader
/// could reach it at all, which is a worse answer than a red gate.
///
/// Measured from the rectangles [`studio_enumeration`] already recorded, which
/// are the ones the presses are aimed at: re-rendering the studio to measure
/// them again would cost a second full enumeration to answer a question the
/// first one has the answer to.
#[test]
fn the_sweep_viewport_reaches_every_control() {
    // Sub-pixel: a control resting exactly on the edge is inside it.
    const TOLERANCE: f64 = 0.5;

    let mut outside = Vec::new();
    let mut measured = 0usize;
    for plan in studio_enumeration() {
        for (described, bounds) in &plan.published {
            measured += 1;
            if bounds.x1 > f64::from(SWEEP_SIZE.0) + TOLERANCE
                || bounds.x0 < -TOLERANCE
                || bounds.y1 > f64::from(SWEEP_SIZE.1) + TOLERANCE
            {
                outside.push(format!(
                    "{}: {described} spans {:.1}..{:.1} x {:.1}..{:.1}",
                    plan.surface, bounds.x0, bounds.x1, bounds.y0, bounds.y1
                ));
            }
        }
    }
    outside.sort();
    assert!(
        outside.is_empty(),
        "controls the sweep's own surface cuts off:\n{}",
        outside.join("\n")
    );
    assert!(
        measured >= PRESSED_FLOOR,
        "the sweep laid out {measured} controls, against a measured {PRESSED_FLOOR}"
    );
}

/// The handled predicate can answer "no".
///
/// A predicate that accepts everything is a sweep that proves nothing, and
/// this one is four terms wide: any of them moving is enough. So it is held
/// against something that genuinely is not a control — a paragraph of the
/// studio's own prose, which egui gives a click sense so a pointer can select
/// text in it, and which therefore advertises `Action::Click` exactly as a
/// button does. Pressing one has to come back unhandled, and pressing a real
/// control on the same surface has to come back handled. Neither answer on its
/// own would show the two are distinguishable.
#[test]
fn the_handled_predicate_reports_a_press_that_moves_nothing() {
    let (name, app) = sweep_surface(0);
    let mut sweep = Sweep::open(app);
    let (settled, nodes) = sweep.settle(&name);
    let prose: Vec<NodeId> = nodes
        .iter()
        .filter(|(_, node)| {
            node.role() == Role::Label
                && node.supports_action(Action::Click)
                && !node.is_disabled()
                && node.bounds().is_some()
        })
        .map(|(id, _)| *id)
        .collect();
    assert!(
        !prose.is_empty(),
        "{name} publishes no clickable prose, so this gate has nothing to discriminate against"
    );
    let mut moved = Vec::new();
    for target in &prose {
        let (_, app) = sweep_surface(0);
        let mut fresh = Sweep::open(app);
        let (before, _) = fresh.settle(&name);
        if fresh.press(*target).moved_from(&before) {
            moved.push(format!("{target:?}"));
        }
    }
    assert!(
        moved.is_empty(),
        "the predicate called a press on the studio's prose handled, so it would call a dead \
         control handled too: {}",
        moved.join(", ")
    );
    let control = nodes
        .iter()
        .find(|(_, node)| is_pressable(node) && !is_text_field(node))
        .map(|(id, _)| *id)
        .expect("the first surface publishes a control");
    assert!(
        sweep.press(control).moved_from(&settled),
        "the predicate answered no to a real control, so its two answers are not \
         distinguishable and neither sweep above means anything"
    );
}

// ------------------------------------------------------ destructive controls

/// Whether this control removes a record from the plan.
///
/// By what it announces, because that is what a reader goes by too. The four
/// registries all name the action `Remove`; the other verbs are here so a
/// route that spells it differently is swept rather than skipped.
fn is_destructive(label: &str) -> bool {
    const VERBS: [&str; 5] = ["Remove", "Delete", "Archive", "Discard", "Clear"];
    VERBS
        .iter()
        .any(|verb| label == *verb || label.starts_with(&format!("{verb} ")))
}

impl Sweep {
    /// Whether a removal review is staged.
    fn review_open(&self) -> bool {
        self.app.state.dialogs.plan_removal_review.target.is_some()
    }

    /// Answer a staged review the way Cancel answers it, then run the frame the
    /// answer produces. The dialog itself is drawn by the application shell,
    /// which these surface-and-overlay passes do not run.
    fn cancel_review(&mut self) {
        self.app.state.dialogs.plan_removal_review.close();
        self.pass(Vec::new());
    }

    /// The lifecycle receipt the studio last wrote, with its sequence, so a
    /// receipt re-issued unchanged is still told from no receipt at all.
    fn receipt(&self) -> (u64, String) {
        let status = &self.app.state.workbench.analysis_lifecycle_status;
        (status.sequence(), status.message().to_owned())
    }

    /// Everything a removal could take: the plan, and the payload the project
    /// workspace holds for it.
    fn plan_records(&self) -> String {
        format!(
            "{:?}\n{:?}",
            self.app.state.sim_setup, self.app.state.workspace
        )
    }
}
