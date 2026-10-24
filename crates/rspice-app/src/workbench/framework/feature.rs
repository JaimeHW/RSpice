//! A feature: one part of the product registered with the workbench, with the
//! state it owns, the commands it declares and the context they run in.
//!
//! A feature's code never holds the whole application. Each command it
//! declares is answered and carried out in a [`Cx`]: its own state, the
//! model every feature shares, the session, and narrow read-only views of
//! the other features it reads. The shell builds that context from the
//! application's parts for each call; it is three references and a view, so
//! building one allocates nothing.
//!
//! The shell's dispatcher runs every command's prologue, whoever owns it;
//! the feature answers only whether its command can run here and what it
//! does once the prologue lets it through.

use crate::state::command_catalog::CommandKey;
use crate::workbench::app_state::Model;
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;

use super::command_registry::{CommandAvailability, CommandSpec};
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

/// Read-only views of the other features a feature reads: live references,
/// never a copy taken earlier.
#[derive(Clone, Copy)]
pub(crate) struct Peers<'a> {
    design: &'a dyn DesignPort,
}

impl<'a> Peers<'a> {
    pub(crate) fn new(design: &'a dyn DesignPort) -> Self {
        Self { design }
    }

    pub(crate) fn design(&self) -> &'a dyn DesignPort {
        self.design
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

/// Everything one feature registered, built once.
pub(crate) struct FeatureRegistry<F: Feature> {
    commands: Vec<FeatureCommand<F>>,
}

impl<F: Feature> FeatureRegistry<F> {
    /// The registry `F` fills in.
    pub(crate) fn build() -> Self {
        let mut registry = Self {
            commands: Vec::new(),
        };
        F::register(&mut registry);
        registry
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
