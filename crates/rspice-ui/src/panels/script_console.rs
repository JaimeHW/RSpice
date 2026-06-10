//! Automation console state — the command history and executor behind the
//! shell console's Automation tab (`crate::shell::console`).

use crate::simulation::automation::{CommandOutput, ScriptExecutor};

/// State for the scripting console
#[derive(Clone, Default)]
pub struct ScriptConsoleState {
    pub input_buffer: String,
    pub history: Vec<ConsoleHistoryItem>,
    pub executor: ScriptExecutor,
}

#[derive(Clone, Default)]
pub struct ConsoleHistoryItem {
    pub command: String,
    pub output: CommandOutput,
}
