/// Simplified component info for DRC checking.
#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub id: u64,
    /// Authored reference designator. Structural objects may legitimately
    /// leave this empty; emitted instances may not.
    pub name: String,
    pub component_type: String,
    pub pins: Vec<PinInfo>,
    pub is_voltage_source: bool,
    pub is_current_source: bool,
    /// Whether this component emits a SPICE instance and therefore owns a
    /// required reference designator.
    pub reference_required: bool,
    /// Exact schema validation failure for a non-empty authored reference.
    /// Extraction computes this with the component's effective primitive or
    /// model-bound prefix; the checker never guesses from display text.
    pub reference_error: Option<String>,
    /// Exact type-resolution status. `None` means the extraction boundary did
    /// not have enough hierarchy authority to decide; fail-open is mandatory
    /// in that case so flat compatibility callers do not invent unknown
    /// project-cell findings.
    pub component_known: Option<bool>,
    /// Required effective properties that the canonical component schema
    /// proves are absent. Defaults accepted by the property editor are
    /// effective values and therefore never appear here.
    pub missing_parameters: Vec<String>,
    /// Definite numeric constants outside explicit canonical schema bounds.
    /// Symbolic expressions and values without a registered range are omitted.
    pub out_of_range_parameters: Vec<ParameterRangeIssue>,
}

/// One exact parameter-range failure extracted from the canonical property
/// schema. Keeping the structured values avoids parsing human error text in
/// the checker and makes messages deterministic.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterRangeIssue {
    pub name: String,
    pub display_name: String,
    pub value: f64,
    pub min: Option<f64>,
    pub max: Option<f64>,
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
    pub id: u64,
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
    /// Synthetic labels represent already-validated bus taps or ground
    /// symbols. They participate in net naming but are never reported as
    /// orphaned user annotations.
    pub synthetic: bool,
    /// A synthetic label that is itself a real electrical endpoint (typed bus
    /// tap or ground symbol), rather than presentation-only naming.
    pub electrical_anchor: bool,
}

/// Simplified explicit-junction position for connectivity checking.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JunctionInfo {
    pub x: f64,
    pub y: f64,
}

impl JunctionInfo {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
