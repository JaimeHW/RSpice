//! The design feature: the schematic and symbol canvases, the stimulus
//! library, and the dialogs that edit what they draw.
//!
//! What it has not registered yet, the shell's legacy tables still serve.

use crate::workbench::framework::feature::{Feature, FeatureRegistry, NoHandles};
use crate::workbench::slots::DesignState;
use crate::workbench::state::WorkbenchSession;

/// The design feature.
pub(crate) struct DesignFeature;

impl Feature for DesignFeature {
    type State = DesignState;
    type Session = WorkbenchSession;
    type Handles = NoHandles;

    fn register(_registry: &mut FeatureRegistry<Self>) {}
}
