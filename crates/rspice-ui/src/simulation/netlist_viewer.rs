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
        } else if !trimmed.is_empty() && !trimmed.starts_with('.') {
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
        if let Some(line) = self.get_line_mut(line_number)
            && line.is_fold_start
        {
            line.is_collapsed = !line.is_collapsed;
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
