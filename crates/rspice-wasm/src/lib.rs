//! # RSpice WebAssembly Bindings
//!
//! This crate provides JavaScript bindings for the RSpice circuit simulator,
//! enabling browser-based circuit simulation with the same high-quality
//! SPICE engine used in the desktop application.
//!
//! ## Usage from JavaScript
//!
//! ```javascript
//! import init, { simulate, parse_netlist } from 'rspice-wasm';
//!
//! await init();
//!
//! const netlist = `
//! * Simple RC Circuit
//! V1 in 0 DC 5
//! R1 in out 1k
//! C1 out 0 1u
//! .tran 1u 10m
//! .end
//! `;
//!
//! const result = simulate(netlist);
//! console.log(result.waveforms);
//! ```

use serde::Serialize;
use wasm_bindgen::prelude::*;

// Re-export core types for advanced usage
pub use rspice_core;

/// Initialize the WASM module with better error handling
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Note: console logging is automatically set up by wasm-bindgen
    log::info!("RSpice WASM initialized");
}

/// Simulation result returned to JavaScript
#[derive(Serialize)]
pub struct SimulationResultJs {
    /// Whether simulation succeeded
    pub success: bool,
    /// Error message if simulation failed
    pub error: Option<String>,
    /// Time points (for transient analysis)
    pub time: Vec<f64>,
    /// Node voltage waveforms: { "node_name": [values...] }
    pub voltages: std::collections::HashMap<String, Vec<f64>>,
    /// Branch current waveforms: { "source_name": [values...] }
    pub currents: std::collections::HashMap<String, Vec<f64>>,
}

/// Netlist parsing result
#[derive(Serialize)]
pub struct ParseResult {
    pub valid: bool,
    pub title: String,
    pub num_elements: usize,
    pub analyses: Vec<String>,
    pub error: Option<String>,
}

/// Parse and validate a SPICE netlist without running simulation
///
/// Returns a JSON object with parsing results
#[wasm_bindgen]
pub fn parse_netlist(netlist: &str) -> Result<JsValue, JsError> {
    use rspice_core::Netlist;

    match Netlist::parse(netlist) {
        Ok(parsed) => {
            let result = ParseResult {
                valid: true,
                title: parsed.title.clone(),
                num_elements: parsed.elements.len(),
                analyses: parsed.analyses.iter().map(|a| format!("{:?}", a)).collect(),
                error: None,
            };
            Ok(serde_wasm_bindgen::to_value(&result)?)
        }
        Err(e) => {
            let result = ParseResult {
                valid: false,
                title: String::new(),
                num_elements: 0,
                analyses: Vec::new(),
                error: Some(e.to_string()),
            };
            Ok(serde_wasm_bindgen::to_value(&result)?)
        }
    }
}

/// Run a SPICE simulation and return results as JSON
///
/// # Arguments
/// * `netlist` - SPICE netlist text
///
/// # Returns
/// JSON object containing simulation results with waveform data
#[wasm_bindgen]
pub fn simulate(netlist: &str) -> Result<JsValue, JsError> {
    use rspice_core::{Engine, Netlist};

    // Parse the netlist
    let parsed =
        Netlist::parse(netlist).map_err(|e| JsError::new(&format!("Parse error: {}", e)))?;

    // Create engine and run DC operating point analysis
    let engine = Engine::default();
    let result = engine
        .run_dc_op(&parsed)
        .map_err(|e| JsError::new(&format!("Simulation error: {}", e)))?;

    // Build response with node voltages
    let mut voltages = std::collections::HashMap::new();

    // Extract node voltages from result
    for (node_id, voltage) in result.node_voltages.iter().enumerate() {
        voltages.insert(format!("V({})", node_id + 1), vec![*voltage]);
    }

    let sim_result = SimulationResultJs {
        success: true,
        error: None,
        time: vec![0.0], // DC analysis - single time point
        voltages,
        currents: std::collections::HashMap::new(),
    };

    Ok(serde_wasm_bindgen::to_value(&sim_result)?)
}

/// Model info for JavaScript
#[derive(Serialize)]
pub struct ModelInfo {
    pub name: String,
    pub model_type: String,
    pub description: Option<String>,
    pub library: String,
}

/// Get library of built-in device models
#[wasm_bindgen]
pub fn get_model_library() -> Result<JsValue, JsError> {
    use rspice_core::library::LibraryManager;

    let manager = LibraryManager::new();
    let mut models = Vec::new();

    // Get all model types and their models
    for model_type in manager.available_types() {
        for model in manager.models_of_type(model_type) {
            models.push(ModelInfo {
                name: model.name.clone(),
                model_type: format!("{:?}", model.model_type),
                description: model.description.clone(),
                library: model.library.to_string(),
            });
        }
    }

    Ok(serde_wasm_bindgen::to_value(&models)?)
}

/// Get version information
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_parse_simple_netlist() {
        let netlist = "* Test\nR1 1 0 1k\n.end";
        let result = parse_netlist(netlist);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_get_version() {
        let version = get_version();
        assert!(!version.is_empty());
    }
}
