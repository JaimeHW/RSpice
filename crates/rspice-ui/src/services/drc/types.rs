use serde::{Deserialize, Serialize};

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

    /// Get icon for UI display
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Info => "ℹ️",
            Self::Warning => "⚠️",
            Self::Error => "❌",
            Self::Critical => "🛑",
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
    /// Component pin not connected to any wire
    UnconnectedPin,
    /// Net label with no matching connections
    OrphanNetLabel,
    /// Wire segment not connected to anything
    DanglingWire,

    //-------------------------------------------------------------------------
    // Electrical Issues
    //-------------------------------------------------------------------------
    /// Multiple voltage sources on same net
    ShortedOutputs,
    /// Missing ground/reference node
    MissingGround,
    /// Direct short circuit (wire from net to same net)
    ShortCircuit,
    /// Voltage source directly connected to another voltage source
    SourceToSource,

    //-------------------------------------------------------------------------
    // Naming Issues
    //-------------------------------------------------------------------------
    /// Duplicate component reference designator
    DuplicateName,
    /// Empty component name
    EmptyName,
    /// Invalid characters in name
    InvalidName,

    //-------------------------------------------------------------------------
    // Component Issues
    //-------------------------------------------------------------------------
    /// Component with missing required parameter
    MissingParameter,
    /// Component value out of expected range
    ValueOutOfRange,
    /// Unknown component type
    UnknownComponent,
}

impl DrcViolationType {
    /// Get the default severity for this violation type
    pub fn default_severity(&self) -> DrcSeverity {
        match self {
            // Critical - simulation cannot run
            Self::MissingGround => DrcSeverity::Critical,
            Self::DuplicateName => DrcSeverity::Critical,

            // Error - likely to cause simulation failure
            Self::FloatingNode => DrcSeverity::Error,
            Self::ShortedOutputs => DrcSeverity::Error,
            Self::ShortCircuit => DrcSeverity::Error,
            Self::SourceToSource => DrcSeverity::Error,
            Self::MissingParameter => DrcSeverity::Error,

            // Warning - may affect results
            Self::UnconnectedPin => DrcSeverity::Warning,
            Self::DanglingWire => DrcSeverity::Warning,
            Self::OrphanNetLabel => DrcSeverity::Warning,
            Self::ValueOutOfRange => DrcSeverity::Warning,

            // Info
            Self::EmptyName => DrcSeverity::Info,
            Self::InvalidName => DrcSeverity::Info,
            Self::UnknownComponent => DrcSeverity::Info,
        }
    }

    /// Get a description of this violation type
    pub fn description(&self) -> &'static str {
        match self {
            Self::FloatingNode => "Node has only one connection",
            Self::UnconnectedPin => "Component pin is not connected",
            Self::OrphanNetLabel => "Net label has no connections",
            Self::DanglingWire => "Wire segment is not connected to anything",
            Self::ShortedOutputs => "Multiple voltage sources connected to same net",
            Self::MissingGround => "Circuit has no ground reference (node 0)",
            Self::ShortCircuit => "Direct short circuit detected",
            Self::SourceToSource => "Voltage source outputs directly connected",
            Self::DuplicateName => "Multiple components have the same name",
            Self::EmptyName => "Component has no reference designator",
            Self::InvalidName => "Component name contains invalid characters",
            Self::MissingParameter => "Required component parameter is missing",
            Self::ValueOutOfRange => "Component value is outside expected range",
            Self::UnknownComponent => "Component type is not recognized",
        }
    }

    /// Get suggested fix for this violation
    pub fn suggested_fix(&self) -> &'static str {
        match self {
            Self::FloatingNode => "Connect the node to another component or remove it",
            Self::UnconnectedPin => "Connect a wire to this pin or mark as intentionally NC",
            Self::OrphanNetLabel => "Connect wires to both ends of the net or remove the label",
            Self::DanglingWire => "Complete the wire connection or delete the segment",
            Self::ShortedOutputs => "Connect only one voltage source per net",
            Self::MissingGround => "Add a ground symbol (GND) to the circuit",
            Self::ShortCircuit => "Remove the short or check intended connectivity",
            Self::SourceToSource => "Add a resistor between voltage sources",
            Self::DuplicateName => "Rename components to have unique identifiers",
            Self::EmptyName => "Assign a reference designator to the component",
            Self::InvalidName => "Use only alphanumeric characters and underscores",
            Self::MissingParameter => "Set the required parameter value",
            Self::ValueOutOfRange => "Check the parameter value is within valid range",
            Self::UnknownComponent => "Check the component type or define the model",
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

    /// Override severity
    pub fn with_severity(mut self, severity: DrcSeverity) -> Self {
        self.severity = severity;
        self
    }
}

/// Location of a DRC violation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DrcLocation {
    /// At a specific schematic coordinate
    Point { x: f64, y: f64 },
    /// On a component
    Component { id: usize, name: String },
    /// On a wire
    Wire { id: usize },
    /// On a net label
    NetLabel { name: String },
    /// On a node (by net name)
    Node { net_name: String },
    /// Global (no specific location)
    Global,
}

impl DrcLocation {
    /// Get display string for this location
    pub fn display(&self) -> String {
        match self {
            Self::Point { x, y } => format!("({:.1}, {:.1})", x, y),
            Self::Component { name, .. } => format!("Component {}", name),
            Self::Wire { id } => format!("Wire #{}", id),
            Self::NetLabel { name } => format!("Net '{}'", name),
            Self::Node { net_name } => format!("Node '{}'", net_name),
            Self::Global => "Global".to_string(),
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

    /// Get violations filtered by severity
    pub fn violations_by_severity(&self, severity: DrcSeverity) -> Vec<&DrcViolation> {
        self.violations
            .iter()
            .filter(|v| v.severity == severity)
            .collect()
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

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        self.violations
            .iter()
            .any(|v| v.severity == DrcSeverity::Warning)
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

impl DrcSummary {
    /// Get display string
    pub fn display(&self) -> String {
        if self.passed {
            if self.warnings > 0 {
                format!("Passed with {} warning(s)", self.warnings)
            } else {
                "Passed - no issues found".to_string()
            }
        } else {
            format!(
                "Failed: {} critical, {} error(s), {} warning(s)",
                self.critical, self.errors, self.warnings
            )
        }
    }
}
