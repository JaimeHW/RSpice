//! The runs' state: what watching the simulations in flight keeps between
//! frames.

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
}
