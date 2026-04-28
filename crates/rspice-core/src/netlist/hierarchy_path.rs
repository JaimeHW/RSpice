//! Hierarchical Path Management
//!
//! Provides unified path representation for hierarchical circuit navigation,
//! following industry conventions for instance addressing.
//!
//! # Path Format
//!
//! Hierarchical paths use dot notation by default: `TOP.X1.X2.R1`
//!
//! Special path segments:
//! - Empty path represents the root/top-level design
//! - Global nodes (ground, power rails) are never prefixed
//!
//! # Usage
//!
//! ```ignore
//! let path = HierarchyPath::root();
//! let child = path.child("X1");
//! let grandchild = child.child("X2");
//! assert_eq!(grandchild.to_string(), "X1.X2");
//!
//! // Parse from string
//! let parsed = HierarchyPath::parse("X1.X2.X3", '.');
//! assert_eq!(parsed.depth(), 3);
//! ```

// Note: serde support not needed here - serialization handled at UI level
use std::fmt;
use std::hash::{Hash, Hasher};

//=============================================================================
// Hierarchy Path Configuration
//=============================================================================

/// Configuration for hierarchy path formatting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HierarchyPathConfig {
    /// Separator character between path segments
    pub separator: char,
    /// Maximum allowed hierarchy depth (prevents infinite recursion)
    pub max_depth: usize,
    /// Whether to preserve full paths or use compressed names
    pub preserve_full_path: bool,
}

impl Default for HierarchyPathConfig {
    fn default() -> Self {
        Self {
            separator: '.',
            max_depth: 100,
            preserve_full_path: true,
        }
    }
}

impl HierarchyPathConfig {
    /// SPICE-style configuration (underscore separator)
    pub fn spice_style() -> Self {
        Self {
            separator: '_',
            max_depth: 100,
            preserve_full_path: true,
        }
    }

    /// Standard style configuration (dot separator)
    pub fn spectre_style() -> Self {
        Self {
            separator: '.',
            max_depth: 256,
            preserve_full_path: true,
        }
    }
}

//=============================================================================
// Hierarchy Path
//=============================================================================

/// A hierarchical path representing the location of an instance or node
/// within a circuit design hierarchy.
///
/// This is the canonical representation used throughout the simulator
/// for tracking instance relationships, parameter scopes, and result mapping.
#[derive(Debug, Clone)]
pub struct HierarchyPath {
    /// Path segments from root to current location (instance names)
    segments: Vec<String>,
    /// Separator character for string formatting
    separator: char,
}

impl Default for HierarchyPath {
    fn default() -> Self {
        Self::root()
    }
}

impl PartialEq for HierarchyPath {
    fn eq(&self, other: &Self) -> bool {
        // Equality based on segments only, ignoring separator choice
        self.segments == other.segments
    }
}

impl Eq for HierarchyPath {}

impl Hash for HierarchyPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.segments.hash(state);
    }
}

impl HierarchyPath {
    //-------------------------------------------------------------------------
    // Constructors
    //-------------------------------------------------------------------------

    /// Create a root (top-level) path with no segments
    pub fn root() -> Self {
        Self {
            segments: Vec::new(),
            separator: '.',
        }
    }

    /// Create a root path with custom separator
    pub fn root_with_separator(separator: char) -> Self {
        Self {
            segments: Vec::new(),
            separator,
        }
    }

    /// Create a path from a single instance name
    pub fn from_instance(name: impl Into<String>) -> Self {
        Self {
            segments: vec![name.into()],
            separator: '.',
        }
    }

    /// Create a path from a vector of segments
    pub fn from_segments(segments: Vec<String>) -> Self {
        Self {
            segments,
            separator: '.',
        }
    }

    /// Create a path from segments with custom separator
    pub fn from_segments_with_separator(segments: Vec<String>, separator: char) -> Self {
        Self {
            segments,
            separator,
        }
    }

    /// Parse a path from a string representation
    ///
    /// # Arguments
    /// * `path` - The path string to parse
    /// * `separator` - The separator character to split on
    ///
    /// # Examples
    /// ```ignore
    /// let path = HierarchyPath::parse("X1.X2.R1", '.');
    /// assert_eq!(path.segments(), &["X1", "X2", "R1"]);
    /// ```
    pub fn parse(path: &str, separator: char) -> Self {
        if path.is_empty() {
            return Self::root_with_separator(separator);
        }

        let segments: Vec<String> = path
            .split(separator)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        Self {
            segments,
            separator,
        }
    }

    //-------------------------------------------------------------------------
    // Navigation
    //-------------------------------------------------------------------------

    /// Create a child path by appending an instance name
    ///
    /// Returns a new path with the given name appended as a child segment.
    pub fn child(&self, name: impl Into<String>) -> Self {
        let mut new_segments = self.segments.clone();
        new_segments.push(name.into());
        Self {
            segments: new_segments,
            separator: self.separator,
        }
    }

    /// Create a child path for a nested node
    ///
    /// This appends a node name that is internal to the current instance.
    /// The result is a fully-qualified hierarchical node name.
    pub fn node(&self, node_name: &str) -> String {
        if self.is_root() {
            node_name.to_string()
        } else {
            format!("{}{}{}", self, self.separator, node_name)
        }
    }

    /// Get the parent path, or None if this is the root
    pub fn parent(&self) -> Option<Self> {
        if self.segments.is_empty() {
            None
        } else {
            let mut parent_segments = self.segments.clone();
            parent_segments.pop();
            Some(Self {
                segments: parent_segments,
                separator: self.separator,
            })
        }
    }

    /// Get ancestors from root to this path (not including this path)
    pub fn ancestors(&self) -> Vec<Self> {
        let mut result = Vec::with_capacity(self.depth());
        let mut current = self.clone();

        while let Some(parent) = current.parent() {
            result.push(parent.clone());
            current = parent;
        }

        result.reverse();
        result
    }

    //-------------------------------------------------------------------------
    // Queries
    //-------------------------------------------------------------------------

    /// Check if this is the root path (no segments)
    pub fn is_root(&self) -> bool {
        self.segments.is_empty()
    }

    /// Get the number of segments in the path (hierarchy depth)
    pub fn depth(&self) -> usize {
        self.segments.len()
    }

    /// Get the path segments
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// Get the last segment (leaf instance name), or None if root
    pub fn leaf(&self) -> Option<&str> {
        self.segments.last().map(|s| s.as_str())
    }

    /// Get the first segment (top-level instance), or None if root
    pub fn top(&self) -> Option<&str> {
        self.segments.first().map(|s| s.as_str())
    }

    /// Get the separator character
    pub fn separator(&self) -> char {
        self.separator
    }

    /// Set the separator character
    pub fn with_separator(mut self, separator: char) -> Self {
        self.separator = separator;
        self
    }

    /// Check if this path is an ancestor of another path
    pub fn is_ancestor_of(&self, other: &Self) -> bool {
        if self.depth() >= other.depth() {
            return false;
        }

        self.segments
            .iter()
            .zip(other.segments.iter())
            .all(|(a, b)| a == b)
    }

    /// Check if this path is a descendant of another path
    pub fn is_descendant_of(&self, other: &Self) -> bool {
        other.is_ancestor_of(self)
    }

    /// Check if this path starts with another path (is equal or descendant)
    pub fn starts_with(&self, prefix: &Self) -> bool {
        if prefix.depth() > self.depth() {
            return false;
        }

        prefix
            .segments
            .iter()
            .zip(self.segments.iter())
            .all(|(a, b)| a == b)
    }

    /// Get the relative path from an ancestor to this path
    ///
    /// Returns None if `ancestor` is not actually an ancestor.
    pub fn relative_to(&self, ancestor: &Self) -> Option<Self> {
        if !self.starts_with(ancestor) {
            return None;
        }

        let relative_segments = self.segments[ancestor.depth()..].to_vec();
        Some(Self {
            segments: relative_segments,
            separator: self.separator,
        })
    }

    //-------------------------------------------------------------------------
    // Name Generation
    //-------------------------------------------------------------------------

    /// Generate a unique element name within this hierarchy
    ///
    /// Combines the path with an element name to create a fully-qualified name.
    pub fn qualified_name(&self, element_name: &str) -> String {
        if self.is_root() {
            element_name.to_string()
        } else {
            format!("{}{}{}", self, self.separator, element_name)
        }
    }

    /// Check if a node name is a global node (ground variants)
    ///
    /// Global nodes are never prefixed with hierarchy path.
    pub fn is_global_node(node: &str) -> bool {
        // Standard ground representations
        node == "0"
            || node.eq_ignore_ascii_case("gnd")
            || node.eq_ignore_ascii_case("ground")
            || node.eq_ignore_ascii_case("vss")
            // Common global supply names
            || node.eq_ignore_ascii_case("vdd")
            || node.eq_ignore_ascii_case("vcc")
            // Global prefix
            || node.starts_with('!')
    }

    /// Qualify a node name with this hierarchy path
    ///
    /// Global nodes are returned unchanged; local nodes get the full path prefix.
    pub fn qualify_node(&self, node: &str) -> String {
        if Self::is_global_node(node) || self.is_root() {
            // Ground is never renamed, global power rails preserved
            if node == "0" || node.eq_ignore_ascii_case("gnd") {
                "0".to_string()
            } else {
                node.to_string()
            }
        } else {
            format!("{}{}{}", self, self.separator, node)
        }
    }
}

impl fmt::Display for HierarchyPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.segments.is_empty() {
            write!(f, "")
        } else {
            let separator_str = self.separator.to_string();
            write!(f, "{}", self.segments.join(&separator_str))
        }
    }
}

//=============================================================================
// Path Iterator
//=============================================================================

/// Iterator over path segments from root to leaf
pub struct HierarchyPathIter<'a> {
    path: &'a HierarchyPath,
    index: usize,
}

impl<'a> Iterator for HierarchyPathIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.path.segments.len() {
            let segment = &self.path.segments[self.index];
            self.index += 1;
            Some(segment)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a HierarchyPath {
    type Item = &'a str;
    type IntoIter = HierarchyPathIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        HierarchyPathIter {
            path: self,
            index: 0,
        }
    }
}

//=============================================================================
// Tests
//=============================================================================
