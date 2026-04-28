//! Signal Identification and Paths
//!
//! Core types for identifying probeable signals in the design.

use std::collections::hash_map::DefaultHasher;
use std::convert::Infallible;
use std::hash::{Hash, Hasher};

// =============================================================================
// Signal ID
// =============================================================================

/// Unique identifier for a signal
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SignalId {
    /// Unique hash-based ID
    id: u64,
    /// Original signal name
    name: String,
}

impl SignalId {
    /// Create from signal name
    pub fn new(name: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        Self {
            id: hasher.finish(),
            name: name.to_string(),
        }
    }

    /// Create from path and name
    pub fn from_path(path: &SignalPath, name: &str) -> Self {
        let full = format!("{}.{}", path.full_path(), name);
        Self::new(&full)
    }

    /// Get numeric ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get signal name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if matches a pattern (supports wildcards)
    pub fn matches(&self, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                if parts[0].is_empty() {
                    return self.name.ends_with(parts[1]);
                }
                if parts[1].is_empty() {
                    return self.name.starts_with(parts[0]);
                }
                return self.name.starts_with(parts[0]) && self.name.ends_with(parts[1]);
            }
        }
        self.name == pattern
    }
}

impl std::fmt::Display for SignalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// =============================================================================
// Signal Path
// =============================================================================

/// Hierarchical path to a signal
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SignalPath {
    /// Path components (e.g., ["top", "amp", "stage1"])
    components: Vec<String>,
}

impl SignalPath {
    /// Create empty (root) path
    pub fn new() -> Self {
        Self::default()
    }

    /// Create from dotted path string (e.g. `top.block.node`).
    pub fn from_dotted(path: &str) -> Self {
        if path.is_empty() {
            return Self::new();
        }
        let components = path.split('.').map(|s| s.to_string()).collect();
        Self { components }
    }

    /// Create from parts
    pub fn from_parts(parts: &[&str]) -> Self {
        Self {
            components: parts.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Get full path as string
    pub fn full_path(&self) -> String {
        self.components.join(".")
    }

    /// Get number of levels
    pub fn depth(&self) -> usize {
        self.components.len()
    }

    /// Is root path?
    pub fn is_root(&self) -> bool {
        self.components.is_empty()
    }

    /// Get parent path
    pub fn parent(&self) -> Option<SignalPath> {
        if self.components.len() <= 1 {
            return None;
        }
        Some(Self {
            components: self.components[..self.components.len() - 1].to_vec(),
        })
    }

    /// Get leaf name
    pub fn leaf(&self) -> Option<&str> {
        self.components.last().map(|s| s.as_str())
    }

    /// Append component
    pub fn push(&mut self, component: &str) {
        self.components.push(component.to_string());
    }

    /// Create child path
    pub fn child(&self, name: &str) -> Self {
        let mut new_path = self.clone();
        new_path.push(name);
        new_path
    }

    /// Check if this path is ancestor of other
    pub fn is_ancestor_of(&self, other: &SignalPath) -> bool {
        if self.depth() >= other.depth() {
            return false;
        }
        self.components
            .iter()
            .zip(other.components.iter())
            .all(|(a, b)| a == b)
    }

    /// Get components iterator
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.components.iter().map(|s| s.as_str())
    }
}

impl std::fmt::Display for SignalPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_path())
    }
}

impl std::str::FromStr for SignalPath {
    type Err = Infallible;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_dotted(path))
    }
}

impl From<&str> for SignalPath {
    fn from(path: &str) -> Self {
        Self::from_dotted(path)
    }
}

// =============================================================================
// Signal Type
// =============================================================================

/// Type of probeable signal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignalType {
    /// Voltage signal (V)
    Voltage,
    /// Current signal (A)
    Current,
    /// Power signal (W)
    Power,
    /// Digital signal (logic levels)
    Digital,
    /// Impedance/S-parameter
    Impedance,
    /// Generic/unknown
    Other,
}

impl SignalType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Voltage => "Voltage",
            Self::Current => "Current",
            Self::Power => "Power",
            Self::Digital => "Digital",
            Self::Impedance => "Impedance",
            Self::Other => "Signal",
        }
    }

    /// Short unit suffix
    pub fn unit(&self) -> &'static str {
        match self {
            Self::Voltage => "V",
            Self::Current => "A",
            Self::Power => "W",
            Self::Digital => "",
            Self::Impedance => "Ω",
            Self::Other => "",
        }
    }

    /// Infer type from signal name
    pub fn from_name(name: &str) -> Self {
        let lower = name.to_lowercase();
        if lower.starts_with("v(") || lower.starts_with("vn(") {
            Self::Voltage
        } else if lower.starts_with("i(") || lower.starts_with("in(") {
            Self::Current
        } else if lower.starts_with("p(") {
            Self::Power
        } else if lower.starts_with("d_") || lower.contains("clk") {
            Self::Digital
        } else if lower.starts_with("s(") || lower.starts_with("z(") {
            Self::Impedance
        } else {
            Self::Other
        }
    }
}

// =============================================================================
// Probeable Signal
// =============================================================================

/// A signal that can be probed
#[derive(Debug, Clone)]
pub struct ProbeableSignal {
    /// Unique identifier
    pub id: SignalId,
    /// Hierarchical path
    pub path: SignalPath,
    /// Signal type
    pub signal_type: SignalType,
    /// Display name
    pub display_name: String,
    /// Description/documentation
    pub description: Option<String>,
    /// Is currently selected for viewing?
    pub is_selected: bool,
    /// Is highlighted?
    pub is_highlighted: bool,
    /// Color index for display
    pub color_index: Option<usize>,
}

impl ProbeableSignal {
    /// Create new probeable signal
    pub fn new(name: &str) -> Self {
        Self {
            id: SignalId::new(name),
            path: SignalPath::new(),
            signal_type: SignalType::from_name(name),
            display_name: name.to_string(),
            description: None,
            is_selected: false,
            is_highlighted: false,
            color_index: None,
        }
    }

    /// Create with path
    pub fn with_path(name: &str, path: SignalPath) -> Self {
        Self {
            id: SignalId::from_path(&path, name),
            path,
            signal_type: SignalType::from_name(name),
            display_name: name.to_string(),
            description: None,
            is_selected: false,
            is_highlighted: false,
            color_index: None,
        }
    }

    /// Set signal type
    pub fn with_type(mut self, signal_type: SignalType) -> Self {
        self.signal_type = signal_type;
        self
    }

    /// Set description
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    /// Full qualified name (path + name)
    pub fn qualified_name(&self) -> String {
        if self.path.is_root() {
            self.display_name.clone()
        } else {
            format!("{}.{}", self.path, self.display_name)
        }
    }

    /// Select this signal
    pub fn select(&mut self) {
        self.is_selected = true;
    }

    /// Deselect this signal
    pub fn deselect(&mut self) {
        self.is_selected = false;
    }

    /// Highlight this signal
    pub fn highlight(&mut self) {
        self.is_highlighted = true;
    }

    /// Unhighlight this signal
    pub fn unhighlight(&mut self) {
        self.is_highlighted = false;
    }

    /// Toggle selection
    pub fn toggle_selection(&mut self) {
        self.is_selected = !self.is_selected;
    }
}

// =============================================================================
// Tests
// =============================================================================
