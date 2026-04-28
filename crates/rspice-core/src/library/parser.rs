//! Library Content Parser
//!
//! Parses SPICE library files to extract .MODEL and .SUBCKT definitions.
//! Handles multi-line continuations and extracts descriptions from comments.

use super::manager::{ModelDefinition, ModelType, SubcircuitDefinition};

/// Parse library content and extract all model and subcircuit definitions
pub fn parse_library_content(
    content: &'static str,
    library_name: &'static str,
) -> (Vec<ModelDefinition>, Vec<SubcircuitDefinition>) {
    let mut models = Vec::new();
    let mut subcircuits = Vec::new();

    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    let mut last_comment = String::new();

    // Track subcircuit nesting depth - models inside subcircuits are internal
    // and should not be exposed in the library browser
    let mut subckt_depth: usize = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Track comments for descriptions
        if line.starts_with('*') && !line.starts_with("*=") {
            let comment_text = line.trim_start_matches('*').trim();
            if !comment_text.is_empty() {
                last_comment = comment_text.to_string();
            }
        }

        let upper = line.to_uppercase();

        // Track subcircuit blocks
        if upper.starts_with(".SUBCKT") {
            // Parse subcircuit at top level only
            if subckt_depth == 0
                && let Some(subckt) = parse_subckt_line(line, library_name, &last_comment)
            {
                subcircuits.push(subckt);
            }
            subckt_depth += 1;
            last_comment.clear();
        } else if upper.starts_with(".ENDS") {
            subckt_depth = subckt_depth.saturating_sub(1);
        }
        // Parse .MODEL directive - only at top level (outside subcircuits)
        else if upper.starts_with(".MODEL") && subckt_depth == 0 {
            if let Some(model) = parse_model_line(line, library_name, &last_comment) {
                models.push(model);
            }
            last_comment.clear();
        }
        // Non-directive line - clear comment tracking if not a continuation
        else if !line.starts_with('+') && !line.starts_with('*') && !line.is_empty() {
            last_comment.clear();
        }

        i += 1;
    }

    (models, subcircuits)
}

/// Parse a .MODEL directive line
/// Format: .MODEL name type(params...)
fn parse_model_line(
    line: &str,
    library_name: &'static str,
    description: &str,
) -> Option<ModelDefinition> {
    // Remove .MODEL prefix
    let rest = line
        .strip_prefix(".MODEL")
        .or_else(|| line.strip_prefix(".model"))
        .or_else(|| line.strip_prefix(".Model"))?
        .trim();

    // Split into parts
    let mut parts = rest.split_whitespace();
    let name = parts.next()?.to_string();

    // Get model type - may have parameters attached like "D(" or may be separate
    let type_part = parts.next()?;
    let model_type_str = type_part.split('(').next()?.trim();

    let model_type = ModelType::from_spice_type(model_type_str);

    let mut model = ModelDefinition::new(name, model_type, library_name);

    if !description.is_empty() {
        model = model.with_description(description);
    }

    Some(model)
}

/// Parse a .SUBCKT directive line
/// Format: .SUBCKT name pin1 pin2 pin3 ...
fn parse_subckt_line(
    line: &str,
    library_name: &'static str,
    description: &str,
) -> Option<SubcircuitDefinition> {
    // Remove .SUBCKT prefix
    let rest = line
        .strip_prefix(".SUBCKT")
        .or_else(|| line.strip_prefix(".subckt"))
        .or_else(|| line.strip_prefix(".Subckt"))?
        .trim();

    // Split into parts
    let mut parts = rest.split_whitespace();
    let name = parts.next()?.to_string();

    // Remaining parts are pin names
    let pins: Vec<String> = parts.map(|s| s.to_string()).collect();

    if pins.is_empty() {
        return None;
    }

    let mut subckt = SubcircuitDefinition::new(name, pins, library_name);

    if !description.is_empty() {
        subckt = subckt.with_description(description);
    }

    Some(subckt)
}

//=============================================================================
// Tests
//=============================================================================

