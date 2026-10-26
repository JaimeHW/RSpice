//! A feature: one part of the product registered with the workbench, with the
//! state it owns, the commands it declares and the context they run in.
//!
//! A feature's code never holds the whole application. Each command it
//! declares is answered and carried out in a [`Cx`]: its own state, the
//! model every feature shares, the session, the platform services' state,
//! and narrow views of the other features it reads, read-only but for
//! letting go of design's canvas gesture. The shell builds that context
//! from the application's parts for each call; it is four references and
//! a view, so building one allocates nothing.
//!
//! The shell's dispatcher runs every command's prologue, whoever owns it;
//! the feature answers only whether its command can run here and what it
//! does once the prologue lets it through.

use crate::state::command_catalog::CommandKey;
use crate::ui::panels::WorkbenchIcon;
use crate::workbench::app_state::{Model, PlatformState};
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;

use super::command_registry::{CommandAvailability, CommandSpec};
use super::dialogs::DialogKey;
use super::ports::ResultsPort;
use super::requests::{Drain, Outbox, Request};

#[cfg(test)]
pub(crate) mod harness;

/// The session a feature's context holds. The framework asks of it only
/// where requests wait.
pub(crate) trait Session: 'static {
    fn outbox(&mut self) -> &mut Outbox;
}

impl Session for crate::workbench::state::workbench_session::WorkbenchSession {
    fn outbox(&mut self) -> &mut Outbox {
        &mut self.outbox
    }
}

/// One part of the product registered with the workbench.
pub(crate) trait Feature: Sized + 'static {
    /// What the feature owns, held in its slot of the application state.
    type State: 'static;
    /// The session its context holds.
    type Session: Session;

    /// Hand the registry everything the feature declares.
    fn register(registry: &mut FeatureRegistry<Self>);
}

/// What design shows that another feature reads.
pub(crate) trait DesignPort {
    /// Which review markers the drawing shows, part of the view the drawing
    /// is read through.
    fn review_markers(&self) -> SchematicReviewMarkerVisibility;
}

/// What the cloud shows that another feature reads.
pub(crate) trait CloudPort {
    /// Whether the license dialog is open, above whatever opened it.
    fn license_dialog_open(&self) -> bool;
}

/// What another feature may ask of the gesture under way on design's canvas:
/// to let go of it. Letting go rolls back only the operation the gesture
/// holds, in whichever buffer holds it, so a sheet switch or a drawing sync
/// asks first, inside the same call, and the gesture's partial edit never
/// reaches the project.
pub(crate) trait CanvasGesturePort {
    fn let_go_of_canvas_gesture(&mut self, model: &mut Model);
}

/// Design as another feature reaches it: what it shows, and its canvas
/// gesture to let go of.
pub(crate) trait DesignPeer: DesignPort + CanvasGesturePort {}

impl<T: DesignPort + CanvasGesturePort> DesignPeer for T {}

/// Everything the caller decides about a publication snapshot; the snapshot
/// builder derives the rest from project state. `created_utc` is supplied
/// here so the builder itself stays clock-free and deterministic. A feature
/// that asks another to build a snapshot names it, so it sits below both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublicationDraft {
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

/// The other features a feature reads, as live references, never a copy
/// taken earlier; and the one thing it may ask of one, letting go of
/// design's canvas gesture.
pub(crate) struct Peers<'a> {
    design: &'a mut dyn DesignPeer,
    cloud: &'a dyn CloudPort,
}

impl<'a> Peers<'a> {
    pub(crate) fn new(design: &'a mut dyn DesignPeer, cloud: &'a dyn CloudPort) -> Self {
        Self { design, cloud }
    }

    pub(crate) fn design(&self) -> &dyn DesignPort {
        &*self.design
    }

    pub(crate) fn cloud(&self) -> &'a dyn CloudPort {
        self.cloud
    }
}

impl CanvasGesturePort for Peers<'_> {
    fn let_go_of_canvas_gesture(&mut self, model: &mut Model) {
        self.design.let_go_of_canvas_gesture(model);
    }
}

/// The context a feature's command runs in.
pub(crate) struct Cx<'a, F: Feature> {
    pub(crate) state: &'a mut F::State,
    pub(crate) model: &'a mut Model,
    pub(crate) wb: &'a mut F::Session,
    /// What the platform services keep between frames: the autosave
    /// interval and the recent files among them.
    pub(crate) platform: &'a mut PlatformState,
    pub(crate) peers: Peers<'a>,
}

impl<'a, F: Feature> Cx<'a, F> {
    pub(crate) fn new(
        state: &'a mut F::State,
        model: &'a mut Model,
        wb: &'a mut F::Session,
        platform: &'a mut PlatformState,
        peers: Peers<'a>,
    ) -> Self {
        Self {
            state,
            model,
            wb,
            platform,
            peers,
        }
    }

    /// Run the command `key` names once this context's call returns, behind
    /// the prologue every command shares.
    pub(crate) fn execute(&mut self, key: CommandKey) {
        self.wb.outbox().push(Execute(key));
    }
}

/// What a feature reads to answer whether one of its commands can run.
pub(crate) struct Reads<'a, F: Feature> {
    pub(crate) state: &'a F::State,
    pub(crate) model: &'a Model,
    pub(crate) wb: &'a F::Session,
    /// Whether a project is open, and whether an operation on it is still
    /// in progress.
    pub(crate) platform: &'a PlatformState,
    /// What the results workspace shows.
    pub(crate) results: &'a dyn ResultsPort,
}

impl<'a, F: Feature> Reads<'a, F> {
    pub(crate) fn new(
        state: &'a F::State,
        model: &'a Model,
        wb: &'a F::Session,
        platform: &'a PlatformState,
        results: &'a dyn ResultsPort,
    ) -> Self {
        Self {
            state,
            model,
            wb,
            platform,
            results,
        }
    }
}

/// A command another command raised, run once the one that raised it
/// returns.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Execute(pub(crate) CommandKey);

impl Request for Execute {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// The commands one feature declares, one value per command.
pub(crate) trait CommandSet: Copy + 'static {
    type Feature: Feature;
    /// Every command of the set the registry lists.
    const REGISTERED: &'static [Self];
    /// The commands of the set the registry leaves out: a container holds
    /// each by key, and no menu, shortcut profile, palette or Repeat reaches
    /// it.
    const UNLISTED: &'static [Self] = &[];

    /// What the command declares about itself.
    fn spec(self) -> CommandSpec;

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
pub(crate) struct FeatureCommand<F: Feature> {
    pub(crate) spec: CommandSpec,
    index: usize,
    availability: fn(usize, &Reads<'_, F>) -> CommandAvailability,
    execute: fn(usize, &mut Cx<'_, F>),
    checked: fn(usize, &Reads<'_, F>) -> Option<bool>,
}

impl<F: Feature> FeatureCommand<F> {
    pub(crate) fn availability(&self, reads: &Reads<'_, F>) -> CommandAvailability {
        (self.availability)(self.index, reads)
    }

    pub(crate) fn execute(&self, cx: &mut Cx<'_, F>) {
        (self.execute)(self.index, cx);
    }

    pub(crate) fn checked(&self, reads: &Reads<'_, F>) -> Option<bool> {
        (self.checked)(self.index, reads)
    }
}

/// A feature's part in a routed command: a workbench command, such as Copy
/// or Zoom in, that the shell sends to whichever part the reader is working
/// in. The shell asks the feature where its own precedence puts the
/// feature's turn, so a routed command reaches its owners in the order it
/// always has.
pub(crate) struct RoutedCommand<F: Feature> {
    pub(crate) key: CommandKey,
    /// Whether the command is the feature's here.
    pub(crate) takes: fn(&Reads<'_, F>) -> bool,
    /// Whether the feature can carry the command out, where it takes it.
    pub(crate) enabled: fn(&Reads<'_, F>) -> bool,
    /// Carry the command out, where the feature takes it.
    pub(crate) execute: fn(&mut Cx<'_, F>),
}

/// A step of the frame's dialogs a feature draws in its context. The shell
/// lists it by `key` in its dialog order, which says where it is drawn and
/// how it holds the workbench.
pub(crate) struct FeatureDialog<F: Feature> {
    pub(crate) key: DialogKey,
    /// Whether what the step draws is open, as the modal gates ask it.
    pub(crate) is_open: fn(&F::State) -> bool,
    /// Run the step for this frame: draw what of it is open, and nothing
    /// when none of it is.
    pub(crate) render: fn(&mut Cx<'_, F>, &egui::Context),
}

/// The inspector section a feature draws in its context, in the body the
/// inspector container keeps for the section the shell resolves to it.
pub(crate) struct FeatureInspector<F: Feature> {
    /// Draw the section in the container's `Ui`.
    pub(crate) show: fn(&mut Cx<'_, F>, &mut egui::Ui),
    /// Commit what the section holds typed and not yet committed, because
    /// another section shows in its place.
    pub(crate) commit_pending_edit: fn(&mut Cx<'_, F>, &egui::Context),
}

/// The surface a feature draws in its context, on the stage of the route the
/// shell resolves to it.
pub(crate) struct FeatureSurface<F: Feature> {
    /// Draw the surface in the stage's `Ui`.
    pub(crate) show: fn(&mut Cx<'_, F>, &mut egui::Ui),
}

/// One tool a feature puts on the context toolbar while its scope shows: a
/// command it declares, which the toolbar draws as a labelled button and
/// lays out, answers, dispatches and folds into its overflow like any other.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ToolbarTool {
    pub(crate) key: CommandKey,
    pub(crate) icon: WorkbenchIcon,
    pub(crate) label: &'static str,
}

/// Everything one feature registered, built once.
pub(crate) struct FeatureRegistry<F: Feature> {
    commands: Vec<FeatureCommand<F>>,
    routed: &'static [RoutedCommand<F>],
    dialogs: &'static [FeatureDialog<F>],
    inspector: Option<FeatureInspector<F>>,
    surface: Option<FeatureSurface<F>>,
    toolbar: &'static [ToolbarTool],
}

impl<F: Feature> FeatureRegistry<F> {
    /// The registry `F` fills in.
    pub(crate) fn build() -> Self {
        let mut registry = Self {
            commands: Vec::new(),
            routed: &[],
            dialogs: &[],
            inspector: None,
            surface: None,
            toolbar: &[],
        };
        F::register(&mut registry);
        registry
    }

    /// Register the feature's dialog steps.
    pub(crate) fn dialogs(&mut self, dialogs: &'static [FeatureDialog<F>]) {
        self.dialogs = dialogs;
    }

    /// Register the feature's inspector section.
    pub(crate) fn inspector(&mut self, inspector: FeatureInspector<F>) {
        self.inspector = Some(inspector);
    }

    /// The inspector section the feature registered, if it draws one.
    pub(crate) fn inspector_section(&self) -> Option<&FeatureInspector<F>> {
        self.inspector.as_ref()
    }

    /// Register the surface the feature draws on its route's stage.
    pub(crate) fn surface(&mut self, surface: FeatureSurface<F>) {
        self.surface = Some(surface);
    }

    /// The surface the feature registered, if it draws one.
    pub(crate) fn surface_section(&self) -> Option<&FeatureSurface<F>> {
        self.surface.as_ref()
    }

    /// Register the tools the feature puts on the context toolbar, in the
    /// order the toolbar lays them out.
    pub(crate) fn toolbar(&mut self, tools: &'static [ToolbarTool]) {
        self.toolbar = tools;
    }

    /// The tools the feature registered for the context toolbar.
    pub(crate) fn toolbar_tools(&self) -> &'static [ToolbarTool] {
        self.toolbar
    }

    /// The registered dialog step `key` names, if the feature draws one.
    pub(crate) fn dialog(&self, key: DialogKey) -> Option<&FeatureDialog<F>> {
        self.dialogs.iter().find(|dialog| dialog.key == key)
    }

    /// Every dialog step the feature registered.
    #[cfg(test)]
    pub(crate) fn registered_dialogs(&self) -> &'static [FeatureDialog<F>] {
        self.dialogs
    }

    /// Register every command of `C`, the ones the registry leaves out too,
    /// so the key a container holds reaches its command.
    pub(crate) fn commands<C: CommandSet<Feature = F>>(&mut self) {
        for (index, command) in C::REGISTERED.iter().enumerate() {
            self.commands.push(FeatureCommand {
                spec: command.spec(),
                index,
                availability: |index, reads| C::REGISTERED[index].availability(reads),
                execute: |index, cx| C::REGISTERED[index].execute(cx),
                checked: |index, reads| C::REGISTERED[index].checked(reads),
            });
        }
        for (index, command) in C::UNLISTED.iter().enumerate() {
            self.commands.push(FeatureCommand {
                spec: command.spec(),
                index,
                availability: |index, reads| C::UNLISTED[index].availability(reads),
                execute: |index, cx| C::UNLISTED[index].execute(cx),
                checked: |index, reads| C::UNLISTED[index].checked(reads),
            });
        }
    }

    /// The registered command `key` names, if the feature declares one.
    pub(crate) fn command(&self, key: CommandKey) -> Option<&FeatureCommand<F>> {
        self.commands.iter().find(|command| command.spec.key == key)
    }

    /// Register the feature's parts in routed commands.
    pub(crate) fn routed(&mut self, routed: &'static [RoutedCommand<F>]) {
        self.routed = routed;
    }

    /// The feature's part in the routed command `key`, if it has one.
    pub(crate) fn routed_command(&self, key: CommandKey) -> Option<&RoutedCommand<F>> {
        self.routed.iter().find(|routed| routed.key == key)
    }

    /// Every routed command the feature takes a part in.
    #[cfg(test)]
    pub(crate) fn registered_routed(&self) -> &'static [RoutedCommand<F>] {
        self.routed
    }
}
