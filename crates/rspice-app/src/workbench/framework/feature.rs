//! A feature: one part of the product registered with the workbench, with the
//! state it owns, the commands it declares and the context they run in.
//!
//! A feature's code never holds the whole application. Each command it
//! declares is answered and carried out in a [`Cx`]: its own state, the
//! model every feature shares, the session, and narrow views of the other
//! features it reads, read-only but for letting go of design's canvas
//! gesture. The shell builds that context from the application's parts for
//! each call; it is three references and a view, so building one allocates
//! nothing.
//!
//! The shell's dispatcher runs every command's prologue, whoever owns it;
//! the feature answers only whether its command can run here and what it
//! does once the prologue lets it through.

use crate::state::command_catalog::CommandKey;
use crate::workbench::app_state::Model;
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;

use super::command_registry::{CommandAvailability, CommandSpec};
use super::dialogs::DialogKey;
use super::requests::{Drain, Outbox, Request};

#[cfg(test)]
pub(crate) mod harness;

/// The session a feature's context holds. The framework asks of it only
/// where requests wait.
pub(crate) trait Session: 'static {
    fn outbox(&mut self) -> &mut Outbox;
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

/// The other features a feature reads, as live references, never a copy
/// taken earlier; and the one thing it may ask of one, letting go of
/// design's canvas gesture.
pub(crate) struct Peers<'a> {
    design: &'a mut dyn DesignPeer,
}

impl<'a> Peers<'a> {
    pub(crate) fn new(design: &'a mut dyn DesignPeer) -> Self {
        Self { design }
    }

    pub(crate) fn design(&self) -> &dyn DesignPort {
        &*self.design
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
    pub(crate) peers: Peers<'a>,
}

impl<'a, F: Feature> Cx<'a, F> {
    pub(crate) fn new(
        state: &'a mut F::State,
        model: &'a mut Model,
        wb: &'a mut F::Session,
        peers: Peers<'a>,
    ) -> Self {
        Self {
            state,
            model,
            wb,
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
}

impl<'a, F: Feature> Reads<'a, F> {
    pub(crate) fn new(state: &'a F::State, model: &'a Model, wb: &'a F::Session) -> Self {
        Self { state, model, wb }
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

    /// What the command declares about itself.
    fn spec(self) -> CommandSpec;

    /// Whether the command can run here, once the shell's own guards (a
    /// project operation in flight, no project open) have passed.
    fn availability(self, reads: &Reads<'_, Self::Feature>) -> CommandAvailability;

    /// Carry the command out. The shell's prologue has already let it
    /// through.
    fn execute(self, cx: &mut Cx<'_, Self::Feature>);
}

/// One registered command: what it declares, and its answers as plain
/// function pointers into its set.
pub(crate) struct FeatureCommand<F: Feature> {
    pub(crate) spec: CommandSpec,
    index: usize,
    availability: fn(usize, &Reads<'_, F>) -> CommandAvailability,
    execute: fn(usize, &mut Cx<'_, F>),
}

impl<F: Feature> FeatureCommand<F> {
    pub(crate) fn availability(&self, reads: &Reads<'_, F>) -> CommandAvailability {
        (self.availability)(self.index, reads)
    }

    pub(crate) fn execute(&self, cx: &mut Cx<'_, F>) {
        (self.execute)(self.index, cx);
    }
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

/// Everything one feature registered, built once.
pub(crate) struct FeatureRegistry<F: Feature> {
    commands: Vec<FeatureCommand<F>>,
    dialogs: &'static [FeatureDialog<F>],
    inspector: Option<FeatureInspector<F>>,
}

impl<F: Feature> FeatureRegistry<F> {
    /// The registry `F` fills in.
    pub(crate) fn build() -> Self {
        let mut registry = Self {
            commands: Vec::new(),
            dialogs: &[],
            inspector: None,
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

    /// The registered dialog step `key` names, if the feature draws one.
    pub(crate) fn dialog(&self, key: DialogKey) -> Option<&FeatureDialog<F>> {
        self.dialogs.iter().find(|dialog| dialog.key == key)
    }

    /// Every dialog step the feature registered.
    #[cfg(test)]
    pub(crate) fn registered_dialogs(&self) -> &'static [FeatureDialog<F>] {
        self.dialogs
    }

    /// Register every command of `C`.
    pub(crate) fn commands<C: CommandSet<Feature = F>>(&mut self) {
        for (index, command) in C::REGISTERED.iter().enumerate() {
            self.commands.push(FeatureCommand {
                spec: command.spec(),
                index,
                availability: |index, reads| C::REGISTERED[index].availability(reads),
                execute: |index, cx| C::REGISTERED[index].execute(cx),
            });
        }
    }

    /// The registered command `key` names, if the feature declares one.
    pub(crate) fn command(&self, key: CommandKey) -> Option<&FeatureCommand<F>> {
        self.commands.iter().find(|command| command.spec.key == key)
    }
}
