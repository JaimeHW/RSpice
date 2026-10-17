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
    /// Every severity a rule can be reported at, weakest first.
    ///
    /// Declared so a surface offering the choice offers all of it: a rule deck
    /// that could only promote, because the control listed three of the four,
    /// would be a deck a team cannot use to demote a finding they have
    /// reviewed and accepted.
    pub const ALL: [Self; 4] = [Self::Info, Self::Warning, Self::Error, Self::Critical];

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

/// Which part of the design a rule reads.
///
/// The same grouping the violation types are declared in, named so a surface
/// listing every rule can band them instead of printing twenty-eight rows in
/// one run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DrcRuleCategory {
    Connectivity,
    Electrical,
    Hierarchy,
    Naming,
    Component,
    SymbolContract,
}

impl DrcRuleCategory {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Connectivity => "Connectivity",
            Self::Electrical => "Electrical",
            Self::Hierarchy => "Hierarchy",
            Self::Naming => "Naming",
            Self::Component => "Components",
            Self::SymbolContract => "Symbol contract",
        }
    }
}

/// Type of DRC violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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
    /// A node that joins declared inputs and nothing that drives them.
    UndrivenNet,
    /// A net the drawing names as ground — a `GND!` global, or a ground
    /// label that lost its net to another name — that the deck does not
    /// write as node 0.
    GroundNotNodeZero,
    /// A global net tied to ground in one cellview — a ground symbol on the
    /// net its label names — while the rest of the design writes it as a node
    /// of its own.
    SupplyTiedToGround,

    //-------------------------------------------------------------------------
    // Hierarchy Issues
    //-------------------------------------------------------------------------
    /// A cellview reaches itself through its own instances, directly or
    /// through other cellviews. The design has no finite expansion, so nothing
    /// below the repeated occurrence is checked.
    RecursiveInstantiation,
    /// A schematic in one of the project's own libraries that the walk from
    /// the simulation root, under its configuration, never reaches. Only a
    /// check of the whole design can say so.
    UnusedCellView,

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
    /// Every rule the checker can report, in the order they are declared.
    ///
    /// Declaration order groups the rules by [`Self::category`], so a surface
    /// walking this list in order can band each run without sorting it.
    /// The array's fixed length does not make it exhaustive — a variant added
    /// to the enum and not here still compiles — so a test in `rules` holds it
    /// to the variants serde's derive declares.
    pub const ALL: [Self; 33] = [
        Self::FloatingNode,
        Self::UnconnectedPin,
        Self::UndrivenInput,
        Self::UnloadedOutput,
        Self::OrphanNetLabel,
        Self::DanglingWire,
        Self::MalformedBus,
        Self::UnnamedBus,
        Self::BusRangeConflict,
        Self::DanglingBusTap,
        Self::MixedBusTap,
        Self::DuplicateBusMemberDriver,
        Self::VectorWidthMismatch,
        Self::OffSheetConnectorWithoutPartner,
        Self::ShortedOutputs,
        Self::MissingGround,
        Self::SourceToSource,
        Self::UndrivenNet,
        Self::GroundNotNodeZero,
        Self::SupplyTiedToGround,
        Self::RecursiveInstantiation,
        Self::UnusedCellView,
        Self::DuplicateName,
        Self::EmptyName,
        Self::InvalidName,
        Self::CaseCollidingNetNames,
        Self::MissingParameter,
        Self::ValueOutOfRange,
        Self::UnknownComponent,
        Self::UnresolvedReference,
        Self::SymbolUnplacedPin,
        Self::SymbolOrphanedPin,
        Self::SymbolPinOffGrid,
    ];

    /// The part of the design this rule reads.
    pub const fn category(&self) -> DrcRuleCategory {
        match self {
            Self::FloatingNode
            | Self::UnconnectedPin
            | Self::UndrivenInput
            | Self::UnloadedOutput
            | Self::OrphanNetLabel
            | Self::DanglingWire
            | Self::MalformedBus
            | Self::UnnamedBus
            | Self::BusRangeConflict
            | Self::DanglingBusTap
            | Self::MixedBusTap
            | Self::DuplicateBusMemberDriver
            | Self::VectorWidthMismatch
            | Self::OffSheetConnectorWithoutPartner => DrcRuleCategory::Connectivity,
            Self::ShortedOutputs
            | Self::MissingGround
            | Self::SourceToSource
            | Self::UndrivenNet
            | Self::GroundNotNodeZero
            | Self::SupplyTiedToGround => DrcRuleCategory::Electrical,
            Self::RecursiveInstantiation | Self::UnusedCellView => DrcRuleCategory::Hierarchy,
            Self::DuplicateName
            | Self::EmptyName
            | Self::InvalidName
            | Self::CaseCollidingNetNames => DrcRuleCategory::Naming,
            Self::MissingParameter
            | Self::ValueOutOfRange
            | Self::UnknownComponent
            | Self::UnresolvedReference => DrcRuleCategory::Component,
            Self::SymbolUnplacedPin | Self::SymbolOrphanedPin | Self::SymbolPinOffGrid => {
                DrcRuleCategory::SymbolContract
            }
        }
    }

    /// Get the default severity for this violation type, from its entry in
    /// the rule table, where the reason for each one is written.
    ///
    /// A finding on an emitted instance the deck cannot write is reported no
    /// lower than its floor for such an instance; see
    /// [`Self::severity_for_instance`].
    pub const fn default_severity(&self) -> DrcSeverity {
        self.rule().default_severity
    }

    /// The severity this violation type carries on one emitted instance when
    /// the project's deck says nothing.
    ///
    /// `written` says whether the deck still writes the instance. A run that
    /// will not start is an error, not a note: an instance the netlist cannot
    /// carry — a cell instance whose reference is not a path segment, or whose
    /// master resolves to nothing — refuses the run. An instance the netlister
    /// writes under a name it chose still runs, and the results carry that
    /// name, so the finding is reported at the rule's default, a warning the
    /// author has to see.
    pub fn severity_for_instance(&self, written: bool) -> DrcSeverity {
        self.default_severity()
            .max(self.floor_for_instance(written))
    }

    /// The lowest severity a finding of this type on one emitted instance is
    /// reported at, whatever the project's deck sets.
    ///
    /// The deck's own floor where the deck still writes the instance; where it
    /// cannot, the rule's floor for an unwritten instance, which is an error
    /// for every rule whose finding there refuses the run. Holding only those
    /// findings there, rather than every finding of the rule, is what lets a
    /// deck lower the warning on an instance the netlister renamed without
    /// letting through one it cannot write.
    pub fn floor_for_instance(&self, written: bool) -> DrcSeverity {
        let floor = self.rule().floor;
        if written {
            floor.deck
        } else {
            floor.unwritten_instance
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
            Self::ShortedOutputs => "Multiple declared outputs connected to the same net",
            Self::UndrivenNet => "Net joins declared inputs and nothing drives it",
            Self::GroundNotNodeZero => "Net named as ground is not node 0 in the deck",
            Self::SupplyTiedToGround => "Global net is tied to ground in only one cellview",
            Self::RecursiveInstantiation => "Cellview reaches itself through its instances",
            Self::UnusedCellView => "Cellview is not reached from the simulation root",
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
            Self::ShortedOutputs => "Leave only one declared output driver on the net",
            Self::UndrivenNet => {
                "Connect a driver to the net, or declare one of its pins an output"
            }
            Self::GroundNotNodeZero => "Use a ground symbol, or a net label named 0 or GND",
            Self::SupplyTiedToGround => {
                "Remove the tie to ground, or give this net a name of its own"
            }
            Self::RecursiveInstantiation => {
                "Remove the instance that reaches its own cellview, or bind it to another view"
            }
            Self::UnusedCellView => {
                "Instantiate the cellview, delete it, or move it to a reference library"
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
    /// The waiver that covers this finding, where one does. A waived finding
    /// is still reported, at its own severity; it does not count against the
    /// design. Stated by the check from the waivers its configuration
    /// carries — see [`super::CheckWaivers::apply`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver: Option<super::DrcWaiver>,
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
            waiver: None,
        }
    }

    /// Whether this finding counts against the design: it is not waived.
    pub const fn counts(&self) -> bool {
        self.waiver.is_none()
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
    /// project's configured override is applied after this and wins, down to
    /// [`DrcViolationType::floor_for_instance`] and no further.
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
    /// A whole cellview, for a finding about the cellview itself rather than
    /// about anything drawn in it.
    CellView { reference: CellViewRef },
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
            Self::CellView { reference } => reference.display_path(),
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

    /// Get all violations, waived ones included.
    pub fn violations(&self) -> &[DrcViolation] {
        &self.violations
    }

    pub(super) fn violations_mut(&mut self) -> &mut [DrcViolation] {
        &mut self.violations
    }

    /// The errors (Error and Critical) that count against the design: a
    /// waived finding is listed, and is not one of them.
    pub fn errors(&self) -> Vec<&DrcViolation> {
        self.violations
            .iter()
            .filter(|v| v.counts() && v.severity >= DrcSeverity::Error)
            .collect()
    }

    /// The warnings that count against the design.
    pub fn warnings(&self) -> Vec<&DrcViolation> {
        self.violations
            .iter()
            .filter(|v| v.counts() && v.severity == DrcSeverity::Warning)
            .collect()
    }

    /// Whether any error counts against the design. A waived error does not:
    /// this is what the run gate refuses on.
    pub fn has_errors(&self) -> bool {
        self.violations
            .iter()
            .any(|v| v.counts() && v.severity >= DrcSeverity::Error)
    }

    /// Check if the design passed (no errors or critical)
    pub fn passed(&self) -> bool {
        !self.has_errors()
    }

    /// Get total violation count, waived findings included.
    pub fn total_count(&self) -> usize {
        self.violations.len()
    }

    /// How many findings at `severity` count against the design.
    pub fn count_by_severity(&self, severity: DrcSeverity) -> usize {
        self.violations
            .iter()
            .filter(|v| v.counts() && v.severity == severity)
            .count()
    }

    /// How many findings a waiver covers.
    pub fn waived_count(&self) -> usize {
        self.violations.iter().filter(|v| !v.counts()).count()
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
