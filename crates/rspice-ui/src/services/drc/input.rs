/// Simplified component info for DRC checking.
#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub id: usize,
    pub name: String,
    pub component_type: String,
    pub pins: Vec<PinInfo>,
    pub is_voltage_source: bool,
    pub is_current_source: bool,
}

/// Simplified pin info.
#[derive(Debug, Clone)]
pub struct PinInfo {
    pub name: String,
    pub net_name: String,
    pub is_output: bool,
    /// Optional pin x-coordinate in schematic space.
    pub x: Option<f64>,
    /// Optional pin y-coordinate in schematic space.
    pub y: Option<f64>,
}

/// Simplified wire info.
#[derive(Debug, Clone)]
pub struct WireInfo {
    pub id: usize,
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
}

/// Simplified net label info.
#[derive(Debug, Clone)]
pub struct NetLabelInfo {
    pub name: String,
    pub x: f64,
    pub y: f64,
}
