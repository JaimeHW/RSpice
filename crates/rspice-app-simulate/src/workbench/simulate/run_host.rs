//! The run tier's host over a simulate command's context.
//!
//! A run started from one of simulate's commands reads and reports through
//! the context the command runs in, through the framework's
//! [`RunView`]: code's two facts are read from code's peer when the host is
//! built.

pub use crate::workbench::framework::run_view::{FrozenRun, RunView};

use super::SimulateFeature;
use crate::simulation::SimulationController;
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::run_view::RunCodeFacts;

/// The run tier's host over `cx`, and beside it the run controller it drives.
pub fn run_host<'v, 'a>(
    cx: &'v mut Cx<'a, SimulateFeature>,
) -> (RunView<'v, 'a>, &'v mut SimulationController) {
    let code = cx.peers.code();
    let code = RunCodeFacts {
        veriloga_compilation: code.veriloga_compilation(),
        netlist_shown: code.netlist_shown(),
    };
    let host = RunView::new(
        &mut *cx.model,
        &mut *cx.runs,
        &mut *cx.wb,
        &*cx.platform,
        &mut cx.peers,
        code,
        cx.commands,
    );
    (host, &mut *cx.handles.run_controller)
}
