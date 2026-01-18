//! Schematic File Operations
//!
//! Save and load schematic files in JSON format.

use crate::state::SchematicState;
use std::path::Path;

/// File extension for schematic files
pub const SCHEMATIC_EXT: &str = "rsch";

/// Save schematic state to a JSON file
pub fn save_schematic(state: &SchematicState, path: &Path) -> Result<(), String> {
    let json =
        serde_json::to_string_pretty(state).map_err(|e| format!("Serialization error: {}", e))?;

    std::fs::write(path, json).map_err(|e| format!("File write error: {}", e))
}

/// Load schematic state from a JSON file
pub fn load_schematic(path: &Path) -> Result<SchematicState, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("File read error: {}", e))?;

    let mut state: SchematicState =
        serde_json::from_str(&content).map_err(|e| format!("Parse error: {}", e))?;

    // Recalculate runtime state (next_id, component_counters) from loaded data
    // This prevents ID collisions when adding new components/wires
    state.recalculate_runtime_state();

    Ok(state)
}
