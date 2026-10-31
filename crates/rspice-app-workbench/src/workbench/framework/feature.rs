//! A feature: one part of the product registered with the workbench, with the
//! state it owns, the commands it declares and the context they run in.
//!
//! A feature's code never holds the whole application. Each command it
//! declares is answered and carried out in a [`Cx`]: its own state and the
//! services it runs, the model every feature shares, the session, the
//! platform services' state, the application's handles on the services
//! below the workbench, and narrow views of the other features it reads,
//! read-only but for letting go of design's canvas gesture. The shell builds that context from the
//! application's parts for each call; it is references and views only, so
//! building one allocates nothing.
//!
//! The shell's dispatcher runs every command's prologue, whoever owns it;
//! the feature answers only whether its command can run here and what it
//! does once the prologue lets it through.

use crate::state::command_catalog::CommandKey;
use crate::ui::panels::WorkbenchIcon;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::state::workbench_session::WorkbenchSession;
use rspice_app_runs::simulation::{NetlistShown, ProjectCompileReceipt};
use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;

use super::command_registry::{CommandAvailability, CommandFlags, CommandSpec, Registry};
use super::dialogs::DialogKey;
use super::ports::{ModelGateFact, ResultsPeer, ResultsPort, RunEventPort};
use super::requests::{Drain, Outbox, Request};

#[cfg(any(test, feature = "test-support"))]
pub mod harness;

/// The session a feature's context holds. The framework asks of it only
/// where requests wait.
pub trait Session: 'static {
    fn outbox(&mut self) -> &mut Outbox;
}

impl Session for WorkbenchSession {
    fn outbox(&mut self) -> &mut Outbox {
        &mut self.outbox
    }
}

/// One part of the product registered with the workbench.
pub trait Feature: Sized + 'static {
    /// What the feature owns, held in its slot of the application state.
    type State: 'static;
    /// The session its context holds.
    type Session: Session;
    /// What the feature runs on the services below the workbench, held by
    /// the application beside its state and lent with its context;
    /// [`NoHandles`] for a feature that runs none.
    type Handles: 'static;

    /// Hand the registry everything the feature declares.
    fn register(registry: &mut FeatureRegistry<Self>);
}

/// What a feature that runs no services of its own is lent beside its
/// context: nothing. It is an empty array because an empty array is the one
/// value a context can borrow mutably with nothing holding it.
pub type NoHandles = [(); 0];

/// What design shows that another feature reads.
pub trait DesignPort {
    /// Which review markers the drawing shows, part of the view the drawing
    /// is read through.
    fn review_markers(&self) -> SchematicReviewMarkerVisibility;
    /// Where the pointer rests on the canvas, in the sheet's coordinates,
    /// while it rests there.
    fn canvas_hover(&self) -> Option<(f64, f64)>;
}

/// What code shows that another feature reads.
pub trait CodePort {
    /// Whether the netlist editor shows the project's own netlist source.
    fn owned_source_active(&self) -> bool;
    /// Whether the netlist editor shows the project's own root source, and
    /// not a file that source includes.
    fn shows_owned_root(&self) -> bool;
    /// The zero-based line holding the netlist editor's caret.
    fn netlist_cursor_line(&self) -> usize;
    /// Which netlist document the editor shows, as a manual-deck run reads it.
    fn netlist_shown(&self) -> NetlistShown;
    /// The receipt of the project's Verilog-A compilation, when one is
    /// current. Read here while code's state holds the receipt; the read
    /// ends when the receipt moves to the model.
    fn veriloga_compilation(&self) -> Option<&ProjectCompileReceipt>;
}

/// What the cloud shows that another feature reads.
pub trait CloudPort {
    /// Whether the license dialog is open, above whatever opened it.
    fn license_dialog_open(&self) -> bool;
    /// Whom this machine's license names, when it holds one.
    fn license_holder(&self) -> Option<&str>;
}

/// What the sessions the cloud runs show that another feature reads: the
/// signed-in account and the live session's peers.
pub trait CloudSessionPort {
    /// The name a reply, a resolution and "My assignments" use for the
    /// person at this application, by the cloud's rule: the account's name,
    /// else the license holder `cloud` shows, else the operating system's
    /// user.
    fn review_actor(&self, cloud: &dyn CloudPort) -> String;
    /// Whom the account button names: the signed-in account's display name
    /// or email, else the license holder `cloud` shows; `None` when neither
    /// names anyone.
    fn account_holder(&self, cloud: &dyn CloudPort) -> Option<String>;
    /// Where the live session's peers point on the canvas of `doc`, the
    /// document a cursor names.
    fn live_cursors(&self, doc: &str) -> Vec<LiveCursor>;
}

/// Where a live-session peer points on a canvas, and the name it goes by.
#[derive(Debug, Clone, PartialEq)]
pub struct LiveCursor {
    /// The peer's principal, which picks the colour its cursor wears.
    pub principal: u128,
    pub name: String,
    /// The point, in the sheet's coordinates.
    pub x: f32,
    pub y: f32,
}

/// What another feature may ask of the gesture under way on design's canvas:
/// to let go of it. Letting go rolls back only the operation the gesture
/// holds, in whichever buffer holds it, so a sheet switch or a drawing sync
/// asks first, inside the same call, and the gesture's partial edit never
/// reaches the project.
pub trait CanvasGesturePort {
    fn let_go_of_canvas_gesture(&mut self, model: &mut Model);
}

/// Design as another feature reaches it: what it shows, and its canvas
/// gesture to let go of.
pub trait DesignPeer: DesignPort + CanvasGesturePort {}

impl<T: DesignPort + CanvasGesturePort> DesignPeer for T {}

/// Everything the caller decides about a publication snapshot; the snapshot
/// builder derives the rest from project state. `created_utc` is supplied
/// here so the builder itself stays clock-free and deterministic. A feature
/// that asks another to build a snapshot names it, so it sits below both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationDraft {
    pub title: String,
    pub description: String,
    pub author_display: String,
    pub created_utc: String,
    pub license: rspice_publication_contract::ContentLicense,
    pub overview_narrative: String,
    pub specification_label: String,
    pub specification_value: String,
    pub specification_unit: String,
    pub include_schematic: bool,
    pub include_results: bool,
    pub include_netlist: bool,
    pub activate_featured: bool,
}

/// What the hardcopy feature builds that another feature reads.
pub trait HardcopyPort {
    /// The validated publication snapshot `draft` describes, built from the
    /// open project in `model` as `workbench`, `results` and `runs` show it:
    /// every printable schematic sheet and plot pane, the active run's
    /// analyses, datasets and measurements, and the effective deck. A refusal
    /// names the boundary that refused.
    fn build_publication_snapshot(
        &self,
        model: &Model,
        workbench: &WorkbenchSession,
        results: &dyn ResultsPort,
        runs: &RunsState,
        draft: &PublicationDraft,
    ) -> Result<rspice_publication_contract::PublicationSnapshot, String>;

    /// The UTC RFC 3339 stamp a publication records as its creation time,
    /// read from the clock as the hardcopy pipeline reads it.
    fn publication_timestamp_utc(&self) -> String;
}

impl Default for PublicationDraft {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            author_display: String::new(),
            created_utc: String::new(),
            license: rspice_publication_contract::ContentLicense::default(),
            overview_narrative: String::new(),
            specification_label: String::new(),
            specification_value: String::new(),
            specification_unit: String::new(),
            include_schematic: true,
            include_results: true,
            include_netlist: true,
            activate_featured: false,
        }
    }
}

/// What simulate shows that another feature reads.
pub trait SimulatePort {
    /// The specification the reader selected in the plan, if any.
    fn selected_specification(&self) -> Option<&str>;
}

/// What the models feature shows that another part reads: whether its
/// authoring holds unsaved work, and each model's release gate.
pub trait ModelsPort {
    /// Whether the model editor or unsigned PDK authoring holds changes not
    /// yet saved.
    fn has_unsaved_authoring_changes(&self) -> bool;
    /// Whether the model editor's open candidate holds changes not yet saved.
    fn model_editor_has_unsaved_changes(&self) -> bool;
    /// Every model in `model`'s library closure, with its release gate as a
    /// reading surface may know it.
    fn model_gate_facts(&self, model: &Model) -> Vec<ModelGateFact>;
}

/// The other features a feature reads, as live references, never a copy
/// taken earlier; and the one thing it may ask of one, letting go of
/// design's canvas gesture.
pub struct Peers<'a> {
    design: &'a mut dyn DesignPeer,
    /// `None` in the cloud feature's own context, as `results` is in the
    /// results feature's.
    cloud: Option<&'a dyn CloudPort>,
    /// `None` in the hardcopy feature's own context, as `results` is in the
    /// results feature's.
    hardcopy: Option<&'a dyn HardcopyPort>,
    /// `None` in the results feature's own context: a feature reads its own
    /// state, never itself as a peer. Lent mutably, as design is, for the
    /// run tier's reports alone.
    results: Option<&'a mut dyn ResultsPeer>,
    /// `None` in simulate's own context, as `results` is in the results
    /// feature's.
    simulate: Option<&'a dyn SimulatePort>,
    code: &'a dyn CodePort,
    /// `None` in the models feature's own context, as `results` is in the
    /// results feature's.
    models: Option<&'a dyn ModelsPort>,
}

impl<'a> Peers<'a> {
    pub fn new(
        design: &'a mut dyn DesignPeer,
        cloud: Option<&'a dyn CloudPort>,
        hardcopy: Option<&'a dyn HardcopyPort>,
        results: Option<&'a mut dyn ResultsPeer>,
        simulate: Option<&'a dyn SimulatePort>,
        code: &'a dyn CodePort,
        models: Option<&'a dyn ModelsPort>,
    ) -> Self {
        Self {
            design,
            cloud,
            hardcopy,
            results,
            simulate,
            code,
            models,
        }
    }

    pub fn design(&self) -> &dyn DesignPort {
        &*self.design
    }

    /// What the cloud shows. The cloud feature's own context holds no such
    /// peer, because the feature reads its own state; its code asking for one
    /// is a fault in that code.
    pub fn cloud(&self) -> &'a dyn CloudPort {
        self.cloud
            .expect("the cloud feature reads its own state, never itself as a peer")
    }

    /// What the hardcopy feature builds. The hardcopy feature's own context
    /// holds no such peer, because the feature reads its own state; its code
    /// asking for one is a fault in that code.
    pub fn hardcopy(&self) -> &'a dyn HardcopyPort {
        self.hardcopy
            .expect("the hardcopy feature reads its own state, never itself as a peer")
    }

    /// What the results workspace shows. The results feature's own context
    /// holds no such peer, because the feature reads its own state; its code
    /// asking for one is a fault in that code.
    pub fn results(&self) -> &dyn ResultsPort {
        self.results
            .as_deref()
            .expect("the results feature reads its own state, never itself as a peer")
    }

    /// Where the run tier's reports reach the results workspace. The results
    /// feature's own context holds no such peer, as for [`Self::results`].
    pub fn run_events(&mut self) -> &mut dyn RunEventPort {
        self.results
            .as_deref_mut()
            .expect("the results feature reads its own state, never itself as a peer")
    }

    /// What simulate shows. Simulate's own context holds no such peer,
    /// because the feature reads its own state; its code asking for one is
    /// a fault in that code.
    pub fn simulate(&self) -> &'a dyn SimulatePort {
        self.simulate
            .expect("simulate reads its own state, never itself as a peer")
    }

    /// What code shows.
    pub fn code(&self) -> &'a dyn CodePort {
        self.code
    }

    /// What the models feature shows. The models feature's own context holds
    /// no such peer, because the feature reads its own state; its code asking
    /// for one is a fault in that code.
    pub fn models(&self) -> &'a dyn ModelsPort {
        self.models
            .expect("the models feature reads its own state, never itself as a peer")
    }
}

/// The application's handles on the services below the workbench that a
/// feature's command reaches through: the export IO that asks where a file
/// goes and writes it, the run controller, which holds the prepared run and
/// loads the transient viewers' derived data, and the Model Hub service,
/// which holds the model-pack store and the hub's catalog. The shell lends
/// them beside the application state for the one call.
pub struct LowerHandles<'a> {
    pub export_io:
        &'a dyn rspice_app_services::workbench::workflows::export_workflow::ExportWorkflowIo,
    pub run_controller: &'a mut rspice_app_runs::simulation::SimulationController,
    pub model_hub: &'a mut rspice_app_services::services::model_hub::ModelHubService,
}

impl CanvasGesturePort for Peers<'_> {
    fn let_go_of_canvas_gesture(&mut self, model: &mut Model) {
        self.design.let_go_of_canvas_gesture(model);
    }
}

/// The context a feature's command runs in.
pub struct Cx<'a, F: Feature> {
    pub state: &'a mut F::State,
    /// What the feature runs, lent for the one call.
    pub own: &'a mut F::Handles,
    pub model: &'a mut Model,
    pub wb: &'a mut F::Session,
    /// What the platform services keep between frames: the autosave
    /// interval and the recent files among them.
    pub platform: &'a mut PlatformState,
    /// What the run tier keeps between frames: the runs in flight, the run
    /// preflight, and the deck the last manual run executed.
    pub runs: &'a mut RunsState,
    pub handles: LowerHandles<'a>,
    pub peers: Peers<'a>,
    /// Every command the application registers, the other features' too:
    /// what a shortcut profile is checked against and a command is labelled
    /// by.
    pub commands: &'static Registry,
}

impl<'a, F: Feature> Cx<'a, F> {
    pub fn new(
        state: &'a mut F::State,
        own: &'a mut F::Handles,
        model: &'a mut Model,
        wb: &'a mut F::Session,
        platform: &'a mut PlatformState,
        runs: &'a mut RunsState,
        handles: LowerHandles<'a>,
        peers: Peers<'a>,
        commands: &'static Registry,
    ) -> Self {
        Self {
            state,
            own,
            model,
            wb,
            platform,
            runs,
            handles,
            peers,
            commands,
        }
    }

    /// Run the command `key` names once this context's call returns, behind
    /// the prologue every command shares.
    pub fn execute(&mut self, key: CommandKey) {
        self.wb.outbox().push(Execute(key));
    }
}

/// What a feature reads to answer whether one of its commands can run.
pub struct Reads<'a, F: Feature> {
    pub state: &'a F::State,
    /// What the feature runs.
    pub own: &'a F::Handles,
    pub model: &'a Model,
    pub wb: &'a F::Session,
    /// Whether a project is open, and whether an operation on it is still
    /// in progress.
    pub platform: &'a PlatformState,
    /// What the run tier keeps between frames: the runs in flight and the
    /// run preflight among them.
    pub runs: &'a RunsState,
    /// What the results workspace shows.
    pub results: &'a dyn ResultsPort,
    /// Every command the application registers, the other features' too.
    pub commands: &'static Registry,
}

impl<'a, F: Feature> Reads<'a, F> {
    pub fn new(
        state: &'a F::State,
        own: &'a F::Handles,
        model: &'a Model,
        wb: &'a F::Session,
        platform: &'a PlatformState,
        runs: &'a RunsState,
        results: &'a dyn ResultsPort,
        commands: &'static Registry,
    ) -> Self {
        Self {
            state,
            own,
            model,
            wb,
            platform,
            runs,
            results,
            commands,
        }
    }
}

/// A command another command raised, run once the one that raised it
/// returns.
#[derive(Clone, Copy, Debug)]
pub struct Execute(pub CommandKey);

impl Request for Execute {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Why the dispatcher's prologue holds back a command that declares `flags`
/// before its owner is asked: a project operation in flight holds back one
/// it blocks, and one that needs an open project needs one open. `None`
/// when the prologue lets it through to its owner.
pub fn held_back(flags: CommandFlags, platform: &PlatformState) -> Option<&'static str> {
    if flags.contains(CommandFlags::BLOCKED_BY_PROJECT_OPERATION)
        && rspice_app_services::workbench::lifecycle::project_lifecycle::operation_in_progress(
            platform,
        )
    {
        return Some("project operation is still in progress");
    }
    if flags.contains(CommandFlags::REQUIRES_OPEN_PROJECT) && !platform.project_lifecycle.is_open()
    {
        return Some("no project is open");
    }
    None
}

/// The commands one feature declares, one value per command.
pub trait CommandSet: Copy + 'static {
    type Feature: Feature;
    /// Every command of the set the registry lists.
    const REGISTERED: &'static [Self];
    /// The commands of the set the registry leaves out: a container holds
    /// each by key, and no menu, shortcut profile, palette or Repeat reaches
    /// it.
    const UNLISTED: &'static [Self] = &[];

    /// What the command declares about itself.
    fn spec(self) -> CommandSpec;

    /// Whether the command is offered here at all, asked before the shell's
    /// own guards: one that is not is hidden, whatever they would answer.
    fn offered(self, _reads: &Reads<'_, Self::Feature>) -> bool {
        true
    }

    /// Whether the command can run here, once the shell's own guards (a
    /// project operation in flight, no project open) have passed.
    fn availability(self, reads: &Reads<'_, Self::Feature>) -> CommandAvailability;

    /// Carry the command out. The shell's prologue has already let it
    /// through.
    fn execute(self, cx: &mut Cx<'_, Self::Feature>);

    /// Whether the state the command switches is on right now, for a command
    /// that switches one; `None` for a command that acts once.
    fn checked(self, _reads: &Reads<'_, Self::Feature>) -> Option<bool> {
        None
    }
}

/// One registered command: what it declares, and its answers as plain
/// function pointers into its set.
pub struct FeatureCommand<F: Feature> {
    pub spec: CommandSpec,
    index: usize,
    offered: fn(usize, &Reads<'_, F>) -> bool,
    availability: fn(usize, &Reads<'_, F>) -> CommandAvailability,
    execute: fn(usize, &mut Cx<'_, F>),
    checked: fn(usize, &Reads<'_, F>) -> Option<bool>,
}

impl<F: Feature> FeatureCommand<F> {
    pub fn offered(&self, reads: &Reads<'_, F>) -> bool {
        (self.offered)(self.index, reads)
    }

    pub fn availability(&self, reads: &Reads<'_, F>) -> CommandAvailability {
        (self.availability)(self.index, reads)
    }

    pub fn execute(&self, cx: &mut Cx<'_, F>) {
        (self.execute)(self.index, cx);
    }

    pub fn checked(&self, reads: &Reads<'_, F>) -> Option<bool> {
        (self.checked)(self.index, reads)
    }
}

/// A feature's part in a routed command: a workbench command, such as Copy
/// or Zoom in, that the shell sends to whichever part the reader is working
/// in. The shell asks the feature where its own precedence puts the
/// feature's turn, so a routed command reaches its owners in the order it
/// always has.
pub struct RoutedCommand<F: Feature> {
    pub key: CommandKey,
    /// Whether the command is the feature's here.
    pub takes: fn(&Reads<'_, F>) -> bool,
    /// Whether the feature can carry the command out, where it takes it.
    pub enabled: fn(&Reads<'_, F>) -> bool,
    /// Carry the command out, where the feature takes it.
    pub execute: fn(&mut Cx<'_, F>),
}

/// A step of the frame's dialogs a feature draws in its context. The shell
/// lists it by `key` in its dialog order, which says where it is drawn and
/// how it holds the workbench.
pub struct FeatureDialog<F: Feature> {
    pub key: DialogKey,
    /// Whether what the step draws is open, as the modal gates ask it.
    pub is_open: fn(&F::State) -> bool,
    /// Run the step for this frame: draw what of it is open, and nothing
    /// when none of it is.
    pub render: fn(&mut Cx<'_, F>, &egui::Context),
}

/// The inspector section a feature draws in its context, in the body the
/// inspector container keeps for the section the shell resolves to it.
pub struct FeatureInspector<F: Feature> {
    /// Draw the section in the container's `Ui`.
    pub show: fn(&mut Cx<'_, F>, &mut egui::Ui),
    /// Commit what the section holds typed and not yet committed, because
    /// another section shows in its place.
    pub commit_pending_edit: fn(&mut Cx<'_, F>, &egui::Context),
}

/// The surface a feature draws in its context, on the stage of the route the
/// shell resolves to it.
pub struct FeatureSurface<F: Feature> {
    /// Draw the surface in the stage's `Ui`.
    pub show: fn(&mut Cx<'_, F>, &mut egui::Ui),
}

/// The navigator section a feature draws in its context while its workspace
/// shows, below the heading and the filter field the navigator keeps.
pub struct FeatureNavigator<F: Feature> {
    /// Draw the section in the navigator's `Ui`.
    pub show: fn(&mut Cx<'_, F>, &mut egui::Ui),
}

/// The compact projection a feature draws in its context beside another
/// workspace's document, when the shell splits the stage for it.
pub struct FeatureSplit<F: Feature> {
    /// Draw the projection in the split's `Ui`.
    pub show: fn(&mut Cx<'_, F>, &mut egui::Ui),
}

/// One tool a feature puts on the context toolbar while its scope shows: a
/// command it declares, which the toolbar draws as a labelled button and
/// lays out, answers, dispatches and folds into its overflow like any other.
#[derive(Clone, Copy, Debug)]
pub struct ToolbarTool {
    pub key: CommandKey,
    pub icon: WorkbenchIcon,
    pub label: &'static str,
}

/// Everything one feature registered, built once.
pub struct FeatureRegistry<F: Feature> {
    commands: Vec<FeatureCommand<F>>,
    routed: &'static [RoutedCommand<F>],
    dialogs: &'static [FeatureDialog<F>],
    inspector: Option<FeatureInspector<F>>,
    surface: Option<FeatureSurface<F>>,
    navigator: Option<FeatureNavigator<F>>,
    split: Option<FeatureSplit<F>>,
    toolbar: &'static [ToolbarTool],
}

impl<F: Feature> FeatureRegistry<F> {
    /// The registry `F` fills in.
    pub fn build() -> Self {
        let mut registry = Self {
            commands: Vec::new(),
            routed: &[],
            dialogs: &[],
            inspector: None,
            surface: None,
            navigator: None,
            split: None,
            toolbar: &[],
        };
        F::register(&mut registry);
        registry
    }

    /// Register the feature's dialog steps.
    pub fn dialogs(&mut self, dialogs: &'static [FeatureDialog<F>]) {
        self.dialogs = dialogs;
    }

    /// Register the feature's inspector section.
    pub fn inspector(&mut self, inspector: FeatureInspector<F>) {
        self.inspector = Some(inspector);
    }

    /// The inspector section the feature registered, if it draws one.
    pub fn inspector_section(&self) -> Option<&FeatureInspector<F>> {
        self.inspector.as_ref()
    }

    /// Register the surface the feature draws on its route's stage.
    pub fn surface(&mut self, surface: FeatureSurface<F>) {
        self.surface = Some(surface);
    }

    /// The surface the feature registered, if it draws one.
    pub fn surface_section(&self) -> Option<&FeatureSurface<F>> {
        self.surface.as_ref()
    }

    /// Register the navigator section the feature draws in its workspace.
    pub fn navigator(&mut self, navigator: FeatureNavigator<F>) {
        self.navigator = Some(navigator);
    }

    /// The navigator section the feature registered, if it draws one.
    pub fn navigator_section(&self) -> Option<&FeatureNavigator<F>> {
        self.navigator.as_ref()
    }

    /// Register the projection the feature draws beside another workspace's
    /// document.
    pub fn split(&mut self, split: FeatureSplit<F>) {
        self.split = Some(split);
    }

    /// The split projection the feature registered, if it draws one.
    pub fn split_section(&self) -> Option<&FeatureSplit<F>> {
        self.split.as_ref()
    }

    /// Register the tools the feature puts on the context toolbar, in the
    /// order the toolbar lays them out.
    pub fn toolbar(&mut self, tools: &'static [ToolbarTool]) {
        self.toolbar = tools;
    }

    /// The tools the feature registered for the context toolbar.
    pub fn toolbar_tools(&self) -> &'static [ToolbarTool] {
        self.toolbar
    }

    /// The registered dialog step `key` names, if the feature draws one.
    pub fn dialog(&self, key: DialogKey) -> Option<&FeatureDialog<F>> {
        self.dialogs.iter().find(|dialog| dialog.key == key)
    }

    /// Every dialog step the feature registered.
    #[cfg(any(test, feature = "test-support"))]
    pub fn registered_dialogs(&self) -> &'static [FeatureDialog<F>] {
        self.dialogs
    }

    /// Register every command of `C`, the ones the registry leaves out too,
    /// so the key a container holds reaches its command.
    pub fn commands<C: CommandSet<Feature = F>>(&mut self) {
        for (index, command) in C::REGISTERED.iter().enumerate() {
            self.commands.push(FeatureCommand {
                spec: command.spec(),
                index,
                offered: |index, reads| C::REGISTERED[index].offered(reads),
                availability: |index, reads| C::REGISTERED[index].availability(reads),
                execute: |index, cx| C::REGISTERED[index].execute(cx),
                checked: |index, reads| C::REGISTERED[index].checked(reads),
            });
        }
        for (index, command) in C::UNLISTED.iter().enumerate() {
            self.commands.push(FeatureCommand {
                spec: command.spec(),
                index,
                offered: |index, reads| C::UNLISTED[index].offered(reads),
                availability: |index, reads| C::UNLISTED[index].availability(reads),
                execute: |index, cx| C::UNLISTED[index].execute(cx),
                checked: |index, reads| C::UNLISTED[index].checked(reads),
            });
        }
    }

    /// The registered command `key` names, if the feature declares one.
    pub fn command(&self, key: CommandKey) -> Option<&FeatureCommand<F>> {
        self.commands.iter().find(|command| command.spec.key == key)
    }

    /// Register the feature's parts in routed commands.
    pub fn routed(&mut self, routed: &'static [RoutedCommand<F>]) {
        self.routed = routed;
    }

    /// The feature's part in the routed command `key`, if it has one.
    pub fn routed_command(&self, key: CommandKey) -> Option<&RoutedCommand<F>> {
        self.routed.iter().find(|routed| routed.key == key)
    }

    /// Every routed command the feature takes a part in.
    #[cfg(any(test, feature = "test-support"))]
    pub fn registered_routed(&self) -> &'static [RoutedCommand<F>] {
        self.routed
    }
}
