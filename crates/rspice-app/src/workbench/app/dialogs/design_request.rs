//! What the design feature's dialogs hand the shell.

use crate::workbench::app::dialogs::connectivity_manager::RevealTarget;
use crate::workbench::app::dialogs::hierarchy::descend::DescendHierarchy;
use crate::workbench::framework::requests::{Drain, Request};

/// What a design dialog hands the shell, which carries it out over the whole
/// session as the dialog once did in place. Each step that raises one raises
/// one, and has it taken straight after it draws, in the frame the reader
/// chose it.
#[derive(Debug, Clone)]
pub(crate) enum DesignRequest {
    /// A checked Descend into hierarchy choice.
    Descend(DescendHierarchy),
    /// Open the component editor again on one instance, whose card a
    /// re-adopted stimulus has just replaced.
    ReopenPropertyEditor(u64),
    /// Reveal one finding of the connectivity report on the canvas.
    RevealConnectivity(RevealTarget),
    /// Open the generated netlist on its source map.
    OpenSourceMap,
    /// Apply the connectivity repairs and policy the manager has under review.
    ApplyConnectivityRepairs,
    /// Extract the selection into the new cell Create hierarchy describes.
    CreateHierarchy,
}

impl Request for DesignRequest {
    const DRAIN: Drain = Drain::AfterContribution;
}
