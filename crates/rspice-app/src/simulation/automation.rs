//! The result of one console command.
//!
//! The console dock at `workbench::docks::console` owns the commands
//! themselves — parsing, dispatch, and the project queries they answer. This
//! is only the shape of what a command hands back, shared so
//! `ScriptConsoleState` can retain a transcript of them.

/// Result of a single script command
#[derive(Debug, Clone, Default)]
pub struct CommandOutput {
    pub success: bool,
    pub message: String,
    pub data: Option<String>,
}
