use std::collections::HashMap;

//=============================================================================
// DC Operating Point Result
//=============================================================================

/// DC operating point result
#[derive(Debug, Clone, Default)]
pub struct DcOpResult {
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
        self.node_voltages.get(node).copied()
    }

    /// Get current through a branch
    pub fn current(&self, branch: &str) -> Option<f64> {
        self.branch_currents.get(branch).copied()
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
