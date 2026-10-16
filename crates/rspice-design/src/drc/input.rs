//! What the rules examine.
//!
//! Every fact here is read off the design's one connectivity extraction. The
//! rules never see wires, junctions, or label positions, because judging those
//! again would be a second extraction and two extractions disagree.

use crate::schematic::port::PortDirection;
use rspice_design_model::Point;

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
    /// The name the deck writes this instance under, or `None` when the
    /// netlister cannot write it at all.
    ///
    /// The two differ for a reference the schema refuses: the netlister names
    /// an unnamed primitive `<prefix><id>` and prefixes one that starts with
    /// the wrong letter, so the run still starts, under a name the results
    /// will carry. A cell instance is addressed by its occurrence path as
    /// well, so one whose reference is not a path segment cannot be written.
    pub deck_name: Option<String>,
    /// A placed interface port. Its terminal is where the cellview's node
    /// continues into the parent, which is a connection no terminal inside
    /// the cellview represents.
    pub is_interface_port: bool,
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
    /// References this instance makes to another instance by name that do
    /// not name exactly one instance of its cellview, so the deck cannot
    /// write them as an element the engine has.
    pub broken_references: Vec<BrokenReference>,
}

/// One reference an instance makes to another by name — a current-controlled
/// source's controlling element — that names no single instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrokenReference {
    /// The parameter that carries the reference.
    pub parameter: String,
    /// The reference as authored.
    pub reference: String,
    /// More than one instance answers to it, rather than none.
    pub ambiguous: bool,
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
    /// What a declaration says this terminal is, where anything declares it.
    ///
    /// `None` is the primitive case, and it is the honest answer rather than a
    /// missing one: a resistor leg, a capacitor plate and a MOSFET terminal
    /// have no interface to read a direction off, so nothing may conclude that
    /// such a terminal is an input. `InOut` is the interface's own
    /// "bidirectional or unclassified" and claims no more than `None` does.
    pub direction: Option<PortDirection>,
    /// Terminal position in schematic space.
    pub point: Point,
    /// Whether anything else in the drawing meets this terminal. Decided by
    /// the extraction, which is the only thing that knows the geometry.
    pub attached: bool,
    /// The terminal may legally be left open: the netlister writes `null` for
    /// it when nothing meets it, as it does for the set and reset inputs of
    /// the built-in XSPICE flip-flops and latches.
    pub optional: bool,
    /// The node this terminal stands on is a global the deck declares, so it
    /// continues into every other cellview that names it.
    pub on_global: bool,
}

impl PinInfo {
    /// Whether this terminal drives the node it stands on.
    ///
    /// Only a declared output does. A terminal nothing classified is not a
    /// driver and is not a load either — it is simply unclassified, which is
    /// why no rule may read the absence of a declaration as a direction.
    pub fn drives(&self) -> bool {
        self.direction == Some(PortDirection::Out)
    }
}
