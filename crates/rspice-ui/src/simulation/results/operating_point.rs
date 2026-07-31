//! DC operating point results.
//!
//! Node voltages, branch currents, and the small-signal device parameters
//! the operating point implies.

use std::collections::HashMap;

//=============================================================================
// DC Operating Point Result
//=============================================================================

/// DC operating point result
#[derive(Debug, Clone, Default)]
pub struct DcOpResult {
    /// Exact solve, annotation, and retention contract applied to this result.
    pub configuration: crate::simulation::dialog::OpConfig,

    /// Number of authored startup directives validated before any selected
    /// ignore/validate-only execution filtering was applied.
    pub validated_startup_directives: usize,

    /// Exact core MNA ordering and values. Ground is omitted; node values are
    /// followed by branch values. This is retained so a later compatible OP
    /// can use the converged state without reconstructing order from maps.
    pub mna_node_names: Vec<String>,
    pub mna_branch_names: Vec<String>,
    pub mna_solution: Vec<f64>,

    /// Node voltages
    pub node_voltages: HashMap<String, f64>,

    /// Branch currents
    pub branch_currents: HashMap<String, f64>,

    /// Device operating points
    pub device_ops: HashMap<String, DeviceOpPoint>,

    /// Per-device operating-point report from the engine (bias and
    /// small-signal parameters with regions, in netlist order) — the data
    /// behind the OP inspector.
    pub device_report: Option<rspice_core::circuit::DeviceOpReport>,
}

impl DcOpResult {
    /// Get voltage at a node
    pub fn voltage(&self, node: &str) -> Option<f64> {
        self.node_voltages.get(node).copied().or_else(|| {
            self.node_voltages
                .iter()
                .find_map(|(name, value)| name.eq_ignore_ascii_case(node).then_some(*value))
        })
    }
}

/// Operating point data for a device
#[derive(Debug, Clone)]
pub struct DeviceOpPoint {
    /// Device type (R, C, M, Q, etc.)
    pub device_type: String,

    /// Operating point parameters
    pub parameters: HashMap<String, f64>,
}
