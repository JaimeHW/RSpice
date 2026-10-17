//! Violation severities, kinds, and locations.

use serde::{Deserialize, Serialize};

use rspice_app_types::hierarchy_path::InstancePath;
use rspice_design_model::Point;
use rspice_design_model::cell_view::CellViewRef;

/// Severity level for DRC violations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DrcSeverity {
    /// Informational note (won't affect simulation)
    Info,
    /// Warning (may affect simulation results)
    Warning,
    /// Error (will likely cause simulation failure)
    Error,
    /// Critical error (simulation cannot proceed)
    Critical,
}

impl DrcSeverity {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Warning => "Warning",
            Self::Error => "Error",
            Self::Critical => "Critical",
        }
    }
}

/// Type of DRC violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DrcViolationType {
    //-------------------------------------------------------------------------
    // Connectivity Issues
    //-------------------------------------------------------------------------
    /// Node with only one connection (floating)
    FloatingNode,
    /// An unattached terminal no interface classified: a passive leg, a
    /// primitive device terminal, or a pin declared `inout`.
    ///
    /// Three findings cover an unattached terminal, and this is the one that
    /// claims nothing about direction. It keeps its name because that is what
    /// every surface already asks for when it means "an open terminal nobody
    /// declared anything about".
    UnconnectedPin,
    /// A pin the bound cell interface declares an input or a supply, which no
    /// conductor reaches: nothing drives it, or nothing feeds its rail.
    UndrivenInput,
    /// A pin the bound cell interface declares an output — or an independent
    /// voltage source's `+` terminal — which no conductor reaches: it drives
    /// nothing.
    UnloadedOutput,
    /// Net label with no matching connections
    OrphanNetLabel,
    /// Wire segment not connected to anything
    DanglingWire,
    /// Bus geometry or declaration is structurally invalid.
    MalformedBus,
    /// Bus has no typed declaration.
    UnnamedBus,
    /// Connected bus declarations or a tap selector disagree.
    BusRangeConflict,
    /// Bus tap does not terminate on its required target kind.
    DanglingBusTap,
    /// Scalar and vector connectivity are mixed at a bus tap.
    MixedBusTap,
    /// More than one output drives the same typed scalar bus member.
    DuplicateBusMemberDriver,
    /// A vector port, pin, or instance terminal meets a bus that declares a
    /// different set of conductors.
    VectorWidthMismatch,
    /// An off-sheet connector's name is declared on only one sheet.
    OffSheetConnectorWithoutPartner,
    /// A net label, pin or bus-tap end sits exactly where two wires cross
    /// with no junction, so it does not say which wire it joins.
    AmbiguousCrossing,

    //-------------------------------------------------------------------------
    // Electrical Issues
    //-------------------------------------------------------------------------
    /// Multiple hierarchy-declared output pins on the same net.
    ShortedOutputs,
    /// Missing ground/reference node
    MissingGround,
    /// Independent voltage sources that close a loop among themselves, which
    /// leaves the loop's current undetermined.
    SourceToSource,

    //-------------------------------------------------------------------------
    // Hierarchy Issues
    //-------------------------------------------------------------------------
    /// A cellview reaches itself through its own instances, directly or
    /// through other cellviews. The design has no finite expansion, so nothing
    /// below the repeated occurrence is checked.
    RecursiveInstantiation,

    //-------------------------------------------------------------------------
    // Naming Issues
    //-------------------------------------------------------------------------
    /// Duplicate component reference designator
    DuplicateName,
    /// Empty component name
    EmptyName,
    /// Invalid characters in name
    InvalidName,
    /// Two authored net names that differ only by ASCII case. The deck folds
    /// them into one node, so the drawing shows a net the simulation does not
    /// have.
    CaseCollidingNetNames,

    //-------------------------------------------------------------------------
    // Component Issues
    //-------------------------------------------------------------------------
    /// Component with missing required parameter
    MissingParameter,
    /// Component value out of expected range
    ValueOutOfRange,
    /// Unknown component type
    UnknownComponent,
    /// An instance names another instance — a current-controlled source its
    /// controlling element — by a reference that names no single instance of
    /// its cellview, so the deck has no element to write for it.
    UnresolvedReference,

    //-------------------------------------------------------------------------
    // Symbol Contract Issues
    //-------------------------------------------------------------------------
    /// Symbol pin required by the cell interface has no terminal location.
    SymbolUnplacedPin,
    /// Symbol pin no longer exists in the paired schematic interface.
    SymbolOrphanedPin,
    /// Symbol pin terminal is not on the required terminal grid.
    SymbolPinOffGrid,
}

impl DrcViolationType {
    /// Get the default severity for this violation type.
    ///
    /// Three of them have no answer that holds for every object that can raise
    /// them: an empty reference, an invalid one, and a type that resolves to
    /// nothing each matter only as far as they change what the deck writes.
    /// This returns what they are worth on an object that writes nothing;
    /// [`Self::severity_for_instance`] is what a rule holding the instance
    /// asks.
    pub fn default_severity(&self) -> DrcSeverity {
        match self {
            // Critical - simulation cannot run
            Self::MissingGround => DrcSeverity::Critical,
            Self::DuplicateName => DrcSeverity::Critical,
            // A cellview that reaches itself has no finite expansion, so there
            // is no deck to run.
            Self::RecursiveInstantiation => DrcSeverity::Critical,

            // Error - likely to cause simulation failure
            // An input nothing drives is a terminal the solver has no source
            // for. It is a stronger claim than an open passive leg because the
            // interface said what the pin is for.
            Self::UndrivenInput => DrcSeverity::Error,
            Self::ShortedOutputs => DrcSeverity::Error,
            Self::SourceToSource => DrcSeverity::Error,
            Self::MissingParameter => DrcSeverity::Error,
            // The engine refuses a card that names an element it does not have.
            Self::UnresolvedReference => DrcSeverity::Error,
            Self::CaseCollidingNetNames => DrcSeverity::Error,
            // The deck would have to guess which wire was meant.
            Self::AmbiguousCrossing => DrcSeverity::Error,
            Self::MalformedBus
            | Self::BusRangeConflict
            | Self::DanglingBusTap
            | Self::MixedBusTap
            | Self::DuplicateBusMemberDriver
            | Self::VectorWidthMismatch => DrcSeverity::Error,
            Self::SymbolUnplacedPin | Self::SymbolOrphanedPin | Self::SymbolPinOffGrid => {
                DrcSeverity::Error
            }

            // Warning - may affect results
            //
            // A node with one connection is almost always a wiring slip, but
            // it never stops the solver: a dangling resistor leg sits at the
            // potential of its other end, and a dangling capacitor plate is an
            // unforced island the conditioning shunt settles. The engine
            // refuses the islands that cannot be solved on its own account,
            // so an error here would refuse runs it completes.
            Self::FloatingNode => DrcSeverity::Warning,
            Self::UnconnectedPin => DrcSeverity::Warning,
            // An output nothing loads still solves; the design just loses the
            // signal it drew.
            Self::UnloadedOutput => DrcSeverity::Warning,
            Self::DanglingWire => DrcSeverity::Warning,
            Self::OrphanNetLabel => DrcSeverity::Warning,
            Self::ValueOutOfRange => DrcSeverity::Warning,
            Self::UnnamedBus => DrcSeverity::Warning,

            // Info
            Self::OffSheetConnectorWithoutPartner => DrcSeverity::Info,

            // Info on an object that emits no instance. See
            // `severity_for_instance` for one that does.
            Self::EmptyName => DrcSeverity::Info,
            Self::InvalidName => DrcSeverity::Info,
            Self::UnknownComponent => DrcSeverity::Info,
        }
    }

    /// The severity this violation type carries on one emitted instance.
    ///
    /// `written` says whether the deck still writes the instance. A run that
    /// will not start is an error, not a note: an instance the netlist cannot
    /// carry — a cell instance whose reference is not a path segment, or whose
    /// master resolves to nothing — refuses the run. An instance the netlister
    /// writes under a name it chose still runs, and the results carry that
    /// name, so the finding is a warning the author has to see.
    pub fn severity_for_instance(&self, written: bool) -> DrcSeverity {
        match self {
            Self::EmptyName | Self::InvalidName | Self::UnknownComponent if written => {
                DrcSeverity::Warning
            }
            Self::EmptyName | Self::InvalidName | Self::UnknownComponent => DrcSeverity::Error,
            _ => self.default_severity(),
        }
    }

    /// Get a description of this violation type
    pub fn description(&self) -> &'static str {
        match self {
            Self::FloatingNode => "Node has only one connection",
            Self::UnconnectedPin => "Component pin is not connected",
            Self::UndrivenInput => "Declared input pin has no driver",
            Self::UnloadedOutput => "Declared output pin drives nothing",
            Self::OrphanNetLabel => "Net label has no connections",
            Self::DanglingWire => "Wire segment is not connected to anything",
            Self::MalformedBus => "Bus geometry or declaration is malformed",
            Self::UnnamedBus => "Bus has no typed declaration",
            Self::BusRangeConflict => "Connected bus ranges or selectors are incompatible",
            Self::DanglingBusTap => "Bus tap does not terminate on the required target",
            Self::MixedBusTap => "Bus tap mixes scalar and vector connectivity",
            Self::DuplicateBusMemberDriver => "Typed bus member has multiple output drivers",
            Self::VectorWidthMismatch => "Vector connection widths disagree",
            Self::OffSheetConnectorWithoutPartner => {
                "Off-sheet connector has no partner on another sheet"
            }
            Self::AmbiguousCrossing => "Label, pin or bus tap on an unmarked wire crossing",
            Self::ShortedOutputs => "Multiple declared outputs connected to the same net",
            Self::RecursiveInstantiation => "Cellview reaches itself through its instances",
            Self::MissingGround => "Circuit has no ground reference (node 0)",
            Self::SourceToSource => "Voltage sources form a closed loop",
            Self::DuplicateName => "Multiple components have the same name",
            Self::EmptyName => "Component has no reference designator",
            Self::InvalidName => "Component name contains invalid characters",
            Self::CaseCollidingNetNames => "Two net names differ only by case",
            Self::MissingParameter => "Required component parameter is missing",
            Self::ValueOutOfRange => "Component value is outside expected range",
            Self::UnknownComponent => "Component type is not recognized",
            Self::UnresolvedReference => "Instance reference names no single instance",
            Self::SymbolUnplacedPin => "Symbol pin is not placed",
            Self::SymbolOrphanedPin => "Symbol pin is not in the schematic interface",
            Self::SymbolPinOffGrid => "Symbol pin is off the terminal grid",
        }
    }

    /// Get suggested fix for this violation
    pub fn suggested_fix(&self) -> &'static str {
        match self {
            Self::FloatingNode => "Connect the node to another component or remove it",
            Self::UnconnectedPin => "Connect this pin or verify that it is intentionally open",
            Self::UndrivenInput => {
                "Connect a driver to this input, or declare the pin bidirectional"
            }
            Self::UnloadedOutput => "Connect this output or verify that it is intentionally open",
            Self::OrphanNetLabel => "Connect wires to both ends of the net or remove the label",
            Self::DanglingWire => "Complete the wire connection or delete the segment",
            Self::MalformedBus => "Repair or redraw the invalid bus geometry and declaration",
            Self::UnnamedBus => "Assign a valid bus name and range",
            Self::BusRangeConflict => "Make connected bus declarations and tap ranges identical",
            Self::DanglingBusTap => "Connect the tap to the required scalar wire or typed bus",
            Self::MixedBusTap => "Separate scalar wires from vector bus connectivity",
            Self::DuplicateBusMemberDriver => "Leave only one output driver on this bus member",
            Self::VectorWidthMismatch => {
                "Declare the same range on both ends, or tap an explicit slice of the bus"
            }
            Self::OffSheetConnectorWithoutPartner => {
                "Place the matching connector on the other sheet, or make this an ordinary net label"
            }
            Self::AmbiguousCrossing => {
                "Place a junction to join the crossing wires, or move it onto one wire"
            }
            Self::ShortedOutputs => "Leave only one declared output driver on the net",
            Self::RecursiveInstantiation => {
                "Remove the instance that reaches its own cellview, or bind it to another view"
            }
            Self::MissingGround => "Add a ground symbol (GND) to the circuit",
            Self::SourceToSource => {
                "Put a series element in the loop, or remove one of its sources"
            }
            Self::DuplicateName => "Rename components to have unique identifiers",
            Self::EmptyName => "Assign a reference designator to the component",
            Self::InvalidName => "Use only alphanumeric characters and underscores",
            Self::CaseCollidingNetNames => {
                "Spell both names the same way, or rename one of the two nets"
            }
            Self::MissingParameter => "Set the required parameter value",
            Self::ValueOutOfRange => "Check the parameter value is within valid range",
            Self::UnknownComponent => "Check the component type or define the model",
            Self::UnresolvedReference => "Name exactly one instance of this cellview",
            Self::SymbolUnplacedPin => "Place the symbol pin on the terminal grid",
            Self::SymbolOrphanedPin => "Delete the orphaned pin or add the schematic port back",
            Self::SymbolPinOffGrid => "Move the symbol pin onto the terminal grid",
        }
    }
}

//=============================================================================
// DRC Violation
//=============================================================================

/// A single DRC violation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrcViolation {
    /// Unique identifier for this violation
    pub id: usize,
    /// Type of violation
    pub violation_type: DrcViolationType,
    /// Severity level
    pub severity: DrcSeverity,
    /// Human-readable message
    pub message: String,
    /// Location in schematic (component ID, wire ID, or coordinates)
    pub location: DrcLocation,
    /// Related component names (for cross-referencing)
    pub related_items: Vec<String>,
}

impl DrcViolation {
    /// Create a new violation
    pub fn new(
        id: usize,
        violation_type: DrcViolationType,
        message: impl Into<String>,
        location: DrcLocation,
    ) -> Self {
        Self {
            id,
            severity: violation_type.default_severity(),
            violation_type,
            message: message.into(),
            location,
            related_items: Vec::new(),
        }
    }

    /// Add related items
    pub fn with_related(mut self, items: Vec<String>) -> Self {
        self.related_items = items;
        self
    }

    /// Re-base this finding's severity on the instance that raised it.
    ///
    /// `written` says whether the deck still writes that instance. Severity
    /// stays a property of the finding rather than of its type alone, because
    /// a naming or binding failure stops a run only where it costs the deck
    /// the instance: see [`DrcViolationType::severity_for_instance`]. A
    /// project's configured override is applied after this and still wins.
    pub fn with_instance_severity(mut self, written: bool) -> Self {
        self.severity = self.violation_type.severity_for_instance(written);
        self
    }
}

/// Location of a DRC violation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DrcLocation {
    /// At a specific schematic coordinate
    Point { x: f64, y: f64 },
    /// On a component
    Component { id: u64, name: String },
    /// On one terminal of a placed component.
    ///
    /// A four-terminal device raises its own per-terminal findings, and a
    /// component-level location cannot say which terminal a finding is about.
    /// The point is the terminal position the extraction placed, so a reader
    /// and the canvas are sent to the pin rather than to the symbol's origin.
    Terminal {
        component_id: u64,
        /// The authored reference, which may legitimately be empty. The
        /// display falls back to the component's identity, exactly as a
        /// message does.
        component_name: String,
        pin_name: String,
        point: Point,
    },
    /// Inside an instantiated cellview, at `inner` within that cellview.
    ///
    /// `path` is the chain of instance references from the checked subject down
    /// to the cellview `inner` belongs to, which is the only thing that tells
    /// `X1/M3` from `X2/M3`. The subject's own findings are never wrapped.
    InsideInstance {
        path: InstancePath,
        view: CellViewRef,
        inner: Box<DrcLocation>,
    },
    /// On a wire
    Wire { id: u64 },
    /// On a bus.
    Bus { id: u64 },
    /// On a bus tap.
    BusTap { id: u64 },
    /// On a net label
    NetLabel { name: String },
    /// On a node (by net name)
    Node { net_name: String },
    /// Global (no specific location)
    Global,
    /// On a symbol pin in a cell's symbol view.
    SymbolPin {
        reference: CellViewRef,
        pin_name: String,
        point: Option<Point>,
    },
}

impl DrcLocation {
    /// Get display string for this location
    pub fn display(&self) -> String {
        match self {
            Self::Point { x, y } => format!("({:.1}, {:.1})", x, y),
            Self::Component { name, .. } => format!("Component {}", name),
            Self::Terminal {
                component_id,
                component_name,
                pin_name,
                ..
            } => {
                if component_name.trim().is_empty() {
                    format!("Pin component #{component_id}.{pin_name}")
                } else {
                    format!("Pin {component_name}.{pin_name}")
                }
            }
            Self::InsideInstance { path, inner, .. } => {
                format!("{path} \u{00b7} {}", inner.display())
            }
            Self::Wire { id } => format!("Wire #{}", id),
            Self::Bus { id } => format!("Bus #{}", id),
            Self::BusTap { id } => format!("Bus tap #{}", id),
            Self::NetLabel { name } => format!("Net '{}'", name),
            Self::Node { net_name } => format!("Node '{}'", net_name),
            Self::Global => "Global".to_string(),
            Self::SymbolPin {
                reference,
                pin_name,
                ..
            } => format!("{} pin {}", reference.display_path(), pin_name),
        }
    }
}

//=============================================================================
// DRC Result
//=============================================================================

/// Result of a DRC check.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DrcResult {
    /// All violations found
    violations: Vec<DrcViolation>,
    /// Whether the check completed successfully
    pub completed: bool,
    /// Time taken in milliseconds
    pub duration_ms: u64,
}

impl DrcResult {
    /// Create a new empty result
    pub fn new() -> Self {
        Self {
            violations: Vec::new(),
            completed: false,
            duration_ms: 0,
        }
    }

    /// Add a violation
    pub fn add_violation(&mut self, violation: DrcViolation) {
        self.violations.push(violation);
    }

    /// Get all violations
    pub fn violations(&self) -> &[DrcViolation] {
        &self.violations
    }

    /// Get only errors (Error and Critical)
    pub fn errors(&self) -> Vec<&DrcViolation> {
        self.violations
            .iter()
            .filter(|v| v.severity >= DrcSeverity::Error)
            .collect()
    }

    /// Get only warnings
    pub fn warnings(&self) -> Vec<&DrcViolation> {
        self.violations
            .iter()
            .filter(|v| v.severity == DrcSeverity::Warning)
            .collect()
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.violations
            .iter()
            .any(|v| v.severity >= DrcSeverity::Error)
    }

    /// Check if the design passed (no errors or critical)
    pub fn passed(&self) -> bool {
        !self.has_errors()
    }

    /// Get total violation count
    pub fn total_count(&self) -> usize {
        self.violations.len()
    }

    /// Get count by severity
    pub fn count_by_severity(&self, severity: DrcSeverity) -> usize {
        self.violations
            .iter()
            .filter(|v| v.severity == severity)
            .count()
    }

    /// Get summary statistics
    pub fn summary(&self) -> DrcSummary {
        DrcSummary {
            total: self.violations.len(),
            critical: self.count_by_severity(DrcSeverity::Critical),
            errors: self.count_by_severity(DrcSeverity::Error),
            warnings: self.count_by_severity(DrcSeverity::Warning),
            info: self.count_by_severity(DrcSeverity::Info),
            passed: self.passed(),
        }
    }
}

/// Summary of DRC results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrcSummary {
    pub total: usize,
    pub critical: usize,
    pub errors: usize,
    pub warnings: usize,
    pub info: usize,
    pub passed: bool,
}
