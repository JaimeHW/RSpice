//! One feature on its own, for its tests: its state, a model and a session,
//! with every peer it reads showing what a fresh session shows.
//!
//! The harness answers and runs the feature's own commands as the feature
//! does, in the same context. The shell's prologue (a project operation in
//! flight, no project open, a read-only drawing, a pending inspector edit)
//! reads the whole application, so a test of the prologue runs in the shell.
//! A command the feature raises waits in the session's outbox; nothing runs
//! it here.

use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;

use crate::state::command_catalog::CommandKey;
use crate::workbench::app_state::Model;
use crate::workbench::framework::command_registry::CommandAvailability;

use super::{
    CanvasGesturePort, Cx, DesignPort, Feature, FeatureCommand, FeatureRegistry, Peers, Reads,
};

/// Peers as a fresh session shows them.
struct NullPorts;

impl DesignPort for NullPorts {
    fn review_markers(&self) -> SchematicReviewMarkerVisibility {
        SchematicReviewMarkerVisibility::default()
    }
}

/// A fresh session holds no canvas gesture to let go of.
impl CanvasGesturePort for NullPorts {
    fn let_go_of_canvas_gesture(&mut self, _model: &mut Model) {}
}

/// A feature, its own state, and the model and session its commands run
/// over.
pub(crate) struct FeatureHarness<F: Feature> {
    pub(crate) state: F::State,
    pub(crate) model: Model,
    pub(crate) wb: F::Session,
}

impl<F: Feature> FeatureHarness<F>
where
    F::State: Default,
{
    /// The feature with nothing of its own open, over `model` and `wb`.
    pub(crate) fn with_model(model: Model, wb: F::Session) -> Self {
        Self {
            state: F::State::default(),
            model,
            wb,
        }
    }

    /// Whether the feature's command `key` can run here, as the feature
    /// answers it.
    pub(crate) fn availability(&self, key: CommandKey) -> CommandAvailability {
        command(&FeatureRegistry::<F>::build(), key).availability(&Reads::new(
            &self.state,
            &self.model,
            &self.wb,
        ))
    }

    /// Carry out the feature's command `key`, as the prologue would have let
    /// it through.
    pub(crate) fn execute(&mut self, key: CommandKey) {
        command(&FeatureRegistry::<F>::build(), key).execute(&mut Cx::new(
            &mut self.state,
            &mut self.model,
            &mut self.wb,
            Peers::new(&mut NullPorts),
        ));
    }
}

/// The command `key` names in `registry`; a test that names another
/// feature's command is wrong, so it fails.
fn command<F: Feature>(registry: &FeatureRegistry<F>, key: CommandKey) -> &FeatureCommand<F> {
    registry
        .command(key)
        .unwrap_or_else(|| panic!("the feature declares no {key:?}"))
}
