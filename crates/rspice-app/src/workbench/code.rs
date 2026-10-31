//! The code feature: the Netlist and Automation workspaces, the Verilog-A
//! views the Design workspace opens, and the source editors they share.
//!
//! What it has not registered yet, the shell's legacy tables still serve.

pub(crate) mod commands;

use crate::workbench::framework::feature::{Feature, FeatureRegistry, NoHandles};
use crate::workbench::slots::CodeState;
use crate::workbench::state::WorkbenchSession;

/// The code feature.
pub(crate) struct CodeFeature;

impl Feature for CodeFeature {
    type State = CodeState;
    type Session = WorkbenchSession;
    type Handles = NoHandles;

    fn register(_registry: &mut FeatureRegistry<Self>) {}
}
