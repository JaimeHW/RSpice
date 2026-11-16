//! The requirements the active retained run was judged against, read from the
//! model alone, for the limit lines Results draws over its waveforms.

use super::Model;
use crate::state::{SimulationRun, SpecEntry};

impl Model {
    /// The requirements the workspace's active retained dataset was judged
    /// against, owned.
    ///
    /// The ones the run sealed with its prepared receipt, or the workspace's
    /// for a run that sealed none. Reading `workspace.specs` alone put a limit
    /// edited after a completed run beside the evidence the run had been
    /// judged on.
    pub fn active_run_specifications(&self) -> Vec<SpecEntry> {
        self.simulation
            .active_run()
            .and_then(SimulationRun::prepared_receipt)
            .map_or_else(
                || self.workspace.content.specs.clone(),
                |receipt| {
                    receipt
                        .specifications()
                        .iter()
                        .map(|specification| specification.entry().clone())
                        .collect()
                },
            )
    }
}
