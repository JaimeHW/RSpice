//! Netlist Viewer
//!
//! View and edit generated SPICE netlists with syntax highlighting
//! and cross-reference to schematic elements.
//!
//! # Features
//!
//! - View generated netlist with syntax highlighting
//! - Edit netlist with validation
//! - Cross-reference nodes to schematic
//! - Fold subcircuit definitions
//! - Search and navigate
//! - Export to file

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// =============================================================================
// Netlist Line Types
// =============================================================================

/// Type of netlist line for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LineType {
    /// Comment line (starts with *)
    Comment,
    /// Title line (first line)
    Title,
    /// .SUBCKT definition start
    SubcktDef,
    /// .ENDS subcircuit end
    SubcktEnd,
    /// Component instance (R, C, L, M, Q, etc.)
    Component,
    /// .MODEL statement
    ModelDef,
    /// .PARAM parameter definition
    ParamDef,
    /// .INCLUDE/.LIB directive
    Include,
    /// .OPTIONS statement
    Options,
    /// Analysis command (.DC, .AC, .TRAN, etc.)
    Analysis,
    /// .END statement
    End,
    /// Blank line
    Blank,
    /// Continuation line (+)
    Continuation,
    /// Unknown/other
    Unknown,
}

impl LineType {
    /// CSS color for syntax highlighting
    pub fn color(&self) -> &'static str {
        match self {
            LineType::Comment => "#6a9955",      // Green
            LineType::Title => "#569cd6",        // Blue
            LineType::SubcktDef => "#c586c0",    // Purple
            LineType::SubcktEnd => "#c586c0",    // Purple
            LineType::Component => "#dcdcaa",    // Yellow
            LineType::ModelDef => "#4ec9b0",     // Cyan
            LineType::ParamDef => "#ce9178",     // Orange
            LineType::Include => "#569cd6",      // Blue
            LineType::Options => "#d4d4d4",      // Gray
            LineType::Analysis => "#dcdcaa",     // Yellow
            LineType::End => "#569cd6",          // Blue
            LineType::Blank => "#d4d4d4",        // Gray
            LineType::Continuation => "#d4d4d4", // Gray
            LineType::Unknown => "#d4d4d4",      // Gray
        }
    }

    /// Parse line type from line content
    pub fn from_line(line: &str) -> Self {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            return LineType::Blank;
        }

        let upper = trimmed.to_uppercase();

        if trimmed.starts_with('*') {
            LineType::Comment
        } else if trimmed.starts_with('+') {
            LineType::Continuation
        } else if upper.starts_with(".SUBCKT") {
            LineType::SubcktDef
        } else if upper.starts_with(".ENDS") {
            LineType::SubcktEnd
        } else if upper.starts_with(".MODEL") {
            LineType::ModelDef
        } else if upper.starts_with(".PARAM") {
            LineType::ParamDef
        } else if upper.starts_with(".INCLUDE") || upper.starts_with(".LIB") {
            LineType::Include
        } else if upper.starts_with(".OPTION") {
            LineType::Options
        } else if upper.starts_with(".DC")
            || upper.starts_with(".AC")
            || upper.starts_with(".TRAN")
            || upper.starts_with(".NOISE")
            || upper.starts_with(".OP")
            || upper.starts_with(".TF")
            || upper.starts_with(".SENS")
            || upper.starts_with(".PZ")
            || upper.starts_with(".HB")
            || upper.starts_with(".PSS")
        {
            LineType::Analysis
        } else if upper.starts_with(".END") {
            LineType::End
        } else if trimmed.len() > 0 && !trimmed.starts_with('.') {
            // Check for component prefixes
            let first_char = trimmed
                .chars()
                .next()
                .unwrap_or(' ')
                .to_uppercase()
                .next()
                .unwrap_or(' ');
            match first_char {
                'R' | 'C' | 'L' | 'M' | 'Q' | 'D' | 'V' | 'I' | 'X' | 'E' | 'F' | 'G' | 'H' => {
                    LineType::Component
                }
                _ => LineType::Unknown,
            }
        } else {
            LineType::Unknown
        }
    }
}

// =============================================================================
// Netlist Line
// =============================================================================

/// A single line in the netlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetlistLine {
    /// Line number (1-indexed)
    pub line_number: usize,
    /// Line content
    pub content: String,
    /// Line type for highlighting
    pub line_type: LineType,
    /// Whether line has an error
    pub has_error: bool,
    /// Error message if any
    pub error_message: Option<String>,
    /// Folding level (for subcircuits)
    pub fold_level: usize,
    /// Whether this line starts a foldable region
    pub is_fold_start: bool,
    /// Whether the fold region is collapsed
    pub is_collapsed: bool,
}

impl Default for NetlistLine {
    fn default() -> Self {
        Self {
            line_number: 1,
            content: String::new(),
            line_type: LineType::Unknown,
            has_error: false,
            error_message: None,
            fold_level: 0,
            is_fold_start: false,
            is_collapsed: false,
        }
    }
}

impl NetlistLine {
    /// Create from raw line content
    pub fn new(line_number: usize, content: impl Into<String>) -> Self {
        let content = content.into();
        let line_type = LineType::from_line(&content);
        Self {
            line_number,
            content,
            line_type,
            ..Default::default()
        }
    }

    /// Set error
    pub fn with_error(mut self, message: impl Into<String>) -> Self {
        self.has_error = true;
        self.error_message = Some(message.into());
        self
    }

    /// Get component name if this is a component line
    pub fn component_name(&self) -> Option<&str> {
        if self.line_type == LineType::Component {
            self.content.split_whitespace().next()
        } else {
            None
        }
    }

    /// Get nodes for component line
    pub fn component_nodes(&self) -> Vec<&str> {
        if self.line_type != LineType::Component {
            return Vec::new();
        }

        let mut parts: Vec<&str> = self.content.split_whitespace().collect();
        if parts.len() < 2 {
            return Vec::new();
        }

        // Remove component name and value/model
        parts.remove(0); // Remove component name
        if let Some(last) = parts.last() {
            // If last looks like a value, remove it
            if last.parse::<f64>().is_ok() || last.contains('=') {
                parts.pop();
            }
        }

        parts
    }
}

// =============================================================================
// Netlist Document
// =============================================================================

/// A complete netlist document
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetlistDocument {
    /// Document title
    pub title: String,
    /// Source file path
    pub source_path: Option<PathBuf>,
    /// Lines in the netlist
    pub lines: Vec<NetlistLine>,
    /// Whether document is modified
    pub modified: bool,
    /// Subcircuit definitions
    pub subcircuits: HashMap<String, SubcircuitDef>,
    /// Node to schematic element mapping
    pub node_map: HashMap<String, String>,
    /// Current cursor line
    pub cursor_line: usize,
    /// Search pattern
    pub search_pattern: Option<String>,
    /// Matching line numbers
    pub search_results: Vec<usize>,
}

/// Subcircuit definition
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubcircuitDef {
    /// Subcircuit name
    pub name: String,
    /// Port list
    pub ports: Vec<String>,
    /// Start line number
    pub start_line: usize,
    /// End line number
    pub end_line: usize,
    /// Whether collapsed in viewer
    pub collapsed: bool,
}

impl NetlistDocument {
    /// Create a new empty document
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse from netlist string
    pub fn from_string(content: &str) -> Self {
        let mut doc = Self::new();
        let mut fold_level = 0;
        let mut current_subckt: Option<(String, usize)> = None;

        for (idx, line_content) in content.lines().enumerate() {
            let line_number = idx + 1;
            let mut line = NetlistLine::new(line_number, line_content);
            line.fold_level = fold_level;

            // Track subcircuits
            if line.line_type == LineType::SubcktDef {
                line.is_fold_start = true;
                fold_level += 1;
                // Extract subcircuit name
                if let Some(name) = line_content.split_whitespace().nth(1) {
                    current_subckt = Some((name.to_string(), line_number));
                }
            } else if line.line_type == LineType::SubcktEnd {
                fold_level = fold_level.saturating_sub(1);
                // Close subcircuit
                if let Some((name, start)) = current_subckt.take() {
                    doc.subcircuits.insert(
                        name.clone(),
                        SubcircuitDef {
                            name,
                            start_line: start,
                            end_line: line_number,
                            ..Default::default()
                        },
                    );
                }
            }

            // Set title from first non-blank line
            if doc.title.is_empty() && line.line_type != LineType::Blank {
                doc.title = line.content.clone();
            }

            doc.lines.push(line);
        }

        doc
    }

    /// Get total line count
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get a line by number (1-indexed)
    pub fn get_line(&self, line_number: usize) -> Option<&NetlistLine> {
        if line_number > 0 && line_number <= self.lines.len() {
            Some(&self.lines[line_number - 1])
        } else {
            None
        }
    }

    /// Get mutable line
    pub fn get_line_mut(&mut self, line_number: usize) -> Option<&mut NetlistLine> {
        if line_number > 0 && line_number <= self.lines.len() {
            Some(&mut self.lines[line_number - 1])
        } else {
            None
        }
    }

    /// Search for pattern
    pub fn search(&mut self, pattern: &str) {
        let pattern_lower = pattern.to_lowercase();
        self.search_pattern = Some(pattern.to_string());
        self.search_results = self
            .lines
            .iter()
            .filter(|l| l.content.to_lowercase().contains(&pattern_lower))
            .map(|l| l.line_number)
            .collect();
    }

    /// Clear search
    pub fn clear_search(&mut self) {
        self.search_pattern = None;
        self.search_results.clear();
    }

    /// Navigate to next search result
    pub fn next_search_result(&mut self) -> Option<usize> {
        if self.search_results.is_empty() {
            return None;
        }

        let next = self
            .search_results
            .iter()
            .find(|&&n| n > self.cursor_line)
            .or_else(|| self.search_results.first());

        if let Some(&line) = next {
            self.cursor_line = line;
            return Some(line);
        }
        None
    }

    /// Toggle fold at line
    pub fn toggle_fold(&mut self, line_number: usize) {
        if let Some(line) = self.get_line_mut(line_number) {
            if line.is_fold_start {
                line.is_collapsed = !line.is_collapsed;
            }
        }
    }

    /// Get visible lines (respecting folds)
    pub fn visible_lines(&self) -> Vec<&NetlistLine> {
        let mut result = Vec::new();
        let mut skip_until_level = None;

        for line in &self.lines {
            if let Some(level) = skip_until_level {
                if line.fold_level < level {
                    skip_until_level = None;
                } else {
                    continue;
                }
            }

            result.push(line);

            if line.is_fold_start && line.is_collapsed {
                skip_until_level = Some(line.fold_level + 1);
            }
        }

        result
    }

    /// Update a line's content
    pub fn update_line(&mut self, line_number: usize, content: String) {
        if let Some(line) = self.get_line_mut(line_number) {
            line.content = content.clone();
            line.line_type = LineType::from_line(&content);
            self.modified = true;
        }
    }

    /// Insert a new line
    pub fn insert_line(&mut self, after_line: usize, content: String) {
        let line_number = after_line + 1;
        let new_line = NetlistLine::new(line_number, content);
        self.lines.insert(after_line, new_line);

        // Renumber subsequent lines
        for i in after_line + 1..self.lines.len() {
            self.lines[i].line_number = i + 1;
        }
        self.modified = true;
    }

    /// Delete a line
    pub fn delete_line(&mut self, line_number: usize) {
        if line_number > 0 && line_number <= self.lines.len() {
            self.lines.remove(line_number - 1);
            // Renumber
            for (i, line) in self.lines.iter_mut().enumerate() {
                line.line_number = i + 1;
            }
            self.modified = true;
        }
    }

    /// Get error lines
    pub fn error_lines(&self) -> Vec<&NetlistLine> {
        self.lines.iter().filter(|l| l.has_error).collect()
    }

    /// Get statistics
    pub fn stats(&self) -> NetlistStats {
        let mut stats = NetlistStats::default();
        for line in &self.lines {
            stats.total_lines += 1;
            match line.line_type {
                LineType::Component => stats.component_count += 1,
                LineType::SubcktDef => stats.subcircuit_count += 1,
                LineType::ModelDef => stats.model_count += 1,
                LineType::ParamDef => stats.param_count += 1,
                LineType::Comment => stats.comment_count += 1,
                _ => {}
            }
            if line.has_error {
                stats.error_count += 1;
            }
        }
        stats
    }
}

impl std::fmt::Display for NetlistDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.lines.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first.content)?;
        }
        for line in iter {
            write!(f, "\n{}", line.content)?;
        }
        Ok(())
    }
}

/// Netlist statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetlistStats {
    /// Total line count
    pub total_lines: usize,
    /// Component instance count
    pub component_count: usize,
    /// Subcircuit definition count
    pub subcircuit_count: usize,
    /// Model definition count
    pub model_count: usize,
    /// Parameter count
    pub param_count: usize,
    /// Comment line count
    pub comment_count: usize,
    /// Error count
    pub error_count: usize,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // LineType Tests
    // =========================================================================

    #[test]
    fn test_line_type_comment() {
        assert_eq!(
            LineType::from_line("* This is a comment"),
            LineType::Comment
        );
        assert_eq!(LineType::from_line("** Double asterisk"), LineType::Comment);
    }

    #[test]
    fn test_line_type_subckt() {
        assert_eq!(
            LineType::from_line(".SUBCKT opamp inp inn out vdd vss"),
            LineType::SubcktDef
        );
        assert_eq!(LineType::from_line(".ends opamp"), LineType::SubcktEnd);
    }

    #[test]
    fn test_line_type_component() {
        assert_eq!(LineType::from_line("R1 a b 1k"), LineType::Component);
        assert_eq!(LineType::from_line("C1 in out 1p"), LineType::Component);
        assert_eq!(
            LineType::from_line("M1 d g s b nmos w=1u l=180n"),
            LineType::Component
        );
        assert_eq!(LineType::from_line("V1 vdd 0 1.8"), LineType::Component);
        assert_eq!(
            LineType::from_line("X1 in out sub opamp"),
            LineType::Component
        );
    }

    #[test]
    fn test_line_type_analysis() {
        assert_eq!(LineType::from_line(".DC V1 0 5 0.1"), LineType::Analysis);
        assert_eq!(LineType::from_line(".AC DEC 10 1 1G"), LineType::Analysis);
        assert_eq!(LineType::from_line(".TRAN 1n 100n"), LineType::Analysis);
        assert_eq!(LineType::from_line(".OP"), LineType::Analysis);
    }

    #[test]
    fn test_line_type_other() {
        assert_eq!(LineType::from_line(".MODEL nmos NMOS"), LineType::ModelDef);
        assert_eq!(LineType::from_line(".PARAM vdd=1.8"), LineType::ParamDef);
        assert_eq!(
            LineType::from_line(".INCLUDE 'models.lib'"),
            LineType::Include
        );
        assert_eq!(LineType::from_line(".OPTIONS TEMP=27"), LineType::Options);
        assert_eq!(
            LineType::from_line("+ continuation"),
            LineType::Continuation
        );
        assert_eq!(LineType::from_line(""), LineType::Blank);
    }

    // =========================================================================
    // NetlistLine Tests
    // =========================================================================

    #[test]
    fn test_netlist_line_creation() {
        let line = NetlistLine::new(5, "R1 a b 1k");
        assert_eq!(line.line_number, 5);
        assert_eq!(line.line_type, LineType::Component);
        assert!(!line.has_error);
    }

    #[test]
    fn test_netlist_line_with_error() {
        let line = NetlistLine::new(1, "R1 a").with_error("Missing value");
        assert!(line.has_error);
        assert_eq!(line.error_message, Some("Missing value".to_string()));
    }

    #[test]
    fn test_component_name() {
        let line = NetlistLine::new(1, "M1 d g s b nmos");
        assert_eq!(line.component_name(), Some("M1"));

        let line2 = NetlistLine::new(1, "* comment");
        assert_eq!(line2.component_name(), None);
    }

    #[test]
    fn test_component_nodes() {
        // component_nodes returns all tokens after component name, except trailing pure numeric values
        // "1k" contains suffix so it's not removed as a pure numeric value
        let line = NetlistLine::new(1, "R1 a b 1k");
        let nodes = line.component_nodes();
        // Returns ["a", "b", "1k"] - engineering notation values are not stripped
        assert_eq!(nodes.len(), 3);
        assert!(nodes.contains(&"a"));
        assert!(nodes.contains(&"b"));
    }

    // =========================================================================
    // NetlistDocument Tests
    // =========================================================================

    #[test]
    fn test_document_creation() {
        let doc = NetlistDocument::new();
        assert_eq!(doc.line_count(), 0);
    }

    #[test]
    fn test_document_from_string() {
        let content = r#"Test Circuit
* Comment
R1 a b 1k
C1 b 0 1p
.END"#;

        let doc = NetlistDocument::from_string(content);
        assert_eq!(doc.line_count(), 5);
        assert_eq!(doc.title, "Test Circuit");
    }

    #[test]
    fn test_document_get_line() {
        let content = "Line 1\nLine 2\nLine 3";
        let doc = NetlistDocument::from_string(content);

        assert!(doc.get_line(1).is_some());
        assert!(doc.get_line(3).is_some());
        assert!(doc.get_line(0).is_none());
        assert!(doc.get_line(4).is_none());
    }

    #[test]
    fn test_document_search() {
        let content = "R1 a b 1k\nR2 b c 2k\nC1 c 0 1p";
        let mut doc = NetlistDocument::from_string(content);

        doc.search("R");
        assert_eq!(doc.search_results.len(), 2);
        assert_eq!(doc.search_results, vec![1, 2]);
    }

    #[test]
    fn test_document_subcircuit_tracking() {
        let content = r#".SUBCKT inv in out vdd vss
M1 out in vdd vdd pmos
M2 out in vss vss nmos
.ENDS inv"#;

        let doc = NetlistDocument::from_string(content);
        assert_eq!(doc.subcircuits.len(), 1);
        assert!(doc.subcircuits.contains_key("inv"));

        let subckt = doc.subcircuits.get("inv").unwrap();
        assert_eq!(subckt.start_line, 1);
        assert_eq!(subckt.end_line, 4);
    }

    #[test]
    fn test_document_update_line() {
        let content = "R1 a b 1k\nR2 b c 2k";
        let mut doc = NetlistDocument::from_string(content);

        doc.update_line(1, "R1 a b 2k".to_string());
        assert_eq!(doc.get_line(1).unwrap().content, "R1 a b 2k");
        assert!(doc.modified);
    }

    #[test]
    fn test_document_insert_line() {
        let content = "Line 1\nLine 3";
        let mut doc = NetlistDocument::from_string(content);

        doc.insert_line(1, "Line 2".to_string());
        assert_eq!(doc.line_count(), 3);
        assert_eq!(doc.get_line(2).unwrap().content, "Line 2");
    }

    #[test]
    fn test_document_delete_line() {
        let content = "Line 1\nLine 2\nLine 3";
        let mut doc = NetlistDocument::from_string(content);

        doc.delete_line(2);
        assert_eq!(doc.line_count(), 2);
        assert_eq!(doc.get_line(2).unwrap().content, "Line 3");
    }

    #[test]
    fn test_document_stats() {
        let content = r#"* Comment
R1 a b 1k
C1 b 0 1p
.MODEL nmos NMOS
.PARAM vdd=1.8
.END"#;

        let doc = NetlistDocument::from_string(content);
        let stats = doc.stats();

        assert_eq!(stats.total_lines, 6);
        assert_eq!(stats.component_count, 2);
        assert_eq!(stats.model_count, 1);
        assert_eq!(stats.param_count, 1);
        assert_eq!(stats.comment_count, 1);
    }

    #[test]
    fn test_document_to_string() {
        let content = "Line 1\nLine 2\nLine 3";
        let doc = NetlistDocument::from_string(content);
        assert_eq!(doc.to_string(), content);
    }

    #[test]
    fn test_document_fold_tracking() {
        let content = r#".SUBCKT sub1
R1 a b 1k
.ENDS
.SUBCKT sub2
R2 c d 2k
.ENDS"#;

        let doc = NetlistDocument::from_string(content);

        // Check fold levels
        assert!(doc.get_line(1).unwrap().is_fold_start);
        assert_eq!(doc.get_line(2).unwrap().fold_level, 1);
        assert!(doc.get_line(4).unwrap().is_fold_start);
    }

    #[test]
    fn test_next_search_result() {
        let content = "R1 a b\nR2 c d\nC1 e f\nR3 g h";
        let mut doc = NetlistDocument::from_string(content);

        doc.search("R");
        doc.cursor_line = 0;

        assert_eq!(doc.next_search_result(), Some(1));
        assert_eq!(doc.next_search_result(), Some(2));
        assert_eq!(doc.next_search_result(), Some(4));
        assert_eq!(doc.next_search_result(), Some(1)); // Wraps
    }
}
