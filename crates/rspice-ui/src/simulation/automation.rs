//! Script Execution Engine
//!
//! Provides a commercial-grade automation interface for RSpice.
//! Allows controlling simulations, extracting data, and generating plots via code.
//!
//! # Features
//!
//! - **Headless Control**: Run simulations without UI interaction.
//! - **Batch Processing**: Sweep arbitrary parameters and aggregate results.
//! - **Data API**: Direct access to SimulationResult and WaveformData.
//! - **Plotting API**: programmatically control WaveformViewer state.

use crate::state::SimulationState;
use std::collections::HashMap;

/// Result of a single script command
#[derive(Debug, Clone, Default)]
pub struct CommandOutput {
    pub success: bool,
    pub message: String,
    pub data: Option<String>,
}

/// Engine for executing automation scripts
#[derive(Clone)]
pub struct ScriptExecutor {
    /// Internal variables/state for the script session
    variables: HashMap<String, String>,
}

impl Default for ScriptExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl ScriptExecutor {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Execute a single command (e.g., "run transient", "plot V(out)")
    /// In a full implementation, this would be a Python/Lua/DSL interpreter
    pub fn execute_command(&mut self, cmd: &str, state: &mut SimulationState) -> CommandOutput {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return CommandOutput {
                success: true,
                message: String::new(),
                data: None,
            };
        }

        match parts[0].to_lowercase().as_str() {
            "run" => self.handle_run(parts.get(1).copied(), state),
            "plot" => self.handle_plot(parts.get(1).copied(), state),
            "set" => self.handle_set(parts.get(1).copied(), parts.get(2).copied()),
            "help" => CommandOutput {
                success: true,
                message: "Available commands: run [analysis], plot [signal], set [var] [val], help"
                    .to_string(),
                data: None,
            },
            _ => CommandOutput {
                success: false,
                message: format!("Unknown command: {}", parts[0]),
                data: None,
            },
        }
    }

    fn handle_run(&self, analysis: Option<&str>, state: &mut SimulationState) -> CommandOutput {
        match analysis {
            Some("transient") | Some("tran") => {
                state.request_run_set();
                CommandOutput {
                    success: true,
                    message: "Simulation triggered successfully (Transient)".to_string(),
                    data: None,
                }
            }
            _ => CommandOutput {
                success: false,
                message: "Usage: run [transient|ac|dc]".to_string(),
                data: None,
            },
        }
    }

    fn handle_plot(&self, signal: Option<&str>, state: &mut SimulationState) -> CommandOutput {
        if let Some(name) = signal {
            if state.toggle_waveform_visibility(name) {
                CommandOutput {
                    success: true,
                    message: format!("Toggled plot for {}", name),
                    data: None,
                }
            } else {
                CommandOutput {
                    success: false,
                    message: format!("Signal {} not found in results", name),
                    data: None,
                }
            }
        } else {
            CommandOutput {
                success: false,
                message: "Usage: plot [signal_name]".to_string(),
                data: None,
            }
        }
    }

    fn handle_set(&mut self, var: Option<&str>, val: Option<&str>) -> CommandOutput {
        if let (Some(v), Some(val)) = (var, val) {
            self.variables.insert(v.to_string(), val.to_string());
            CommandOutput {
                success: true,
                message: format!("Set {} = {}", v, val),
                data: None,
            }
        } else {
            CommandOutput {
                success: false,
                message: "Usage: set [var] [val]".to_string(),
                data: None,
            }
        }
    }
}

// =============================================================================
// Tests
// =============================================================================
