//! What the rules examine.
//!
//! Every fact here is read off the design's one connectivity extraction. The
//! rules never see wires, junctions, or label positions, because judging those
//! again would be a second extraction and two extractions disagree.

use crate::state::Point;

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
    /// A drawn ground symbol. Its own terminal is what binds node 0, so it is
    /// not evidence that the circuit reaches ground.
    pub is_ground_symbol: bool,
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
    /// The node name the deck writes for this terminal. It comes from the one
    /// extraction, so a rule that quotes it quotes what the engine will see.
    pub net_name: String,
    pub is_output: bool,
    /// Terminal position in schematic space.
    pub point: Point,
    /// Whether anything else in the drawing meets this terminal. Decided by
    /// the extraction, which is the only thing that knows the geometry.
    pub attached: bool,
}
