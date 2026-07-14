//! Netlist Line Classification
//!
//! Classifies SPICE deck lines for syntax highlighting in the netlist view
//! (`crate::workbench::netlist_document`), which maps each [`LineType`] onto the
//! active design tokens.

/// Type of netlist line for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
