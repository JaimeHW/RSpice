//! The runs' state: what watching the simulations in flight and checking a
//! run before it starts keep between frames.

/// What the runs keep between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct RunsState {
    /// The runs seen running and the campaign outcomes not yet announced.
    ///
    /// The shell announces a completion on the *transition* into a terminal
    /// lifecycle rather than on the terminal state itself, so opening a
    /// project whose history already holds a finished run never claims a run
    /// just completed.
    pub(crate) run_watch: crate::simulation::controller::prepared_run::RunWatch,
    /// Mockup-specified simulation-preflight workflow. Validation reports are
    /// revision-bound runtime evidence and therefore never serialized.
    pub preflight: crate::workbench::app_state::run_preflight::PreflightDialogState,
    /// The deck a manual-deck run started from and the baseline the last one
    /// left. Runtime only, and boxed so a moved state stays the same size.
    pub(crate) manual_run: Box<crate::simulation::manual_run::ManualRunHandoff>,
}
