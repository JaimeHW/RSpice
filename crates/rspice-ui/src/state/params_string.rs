//! SPICE parameter-string parsing.
//!
//! `params="m=2 tc1=0.01"` appears on instance lines, in netlist generation,
//! and in the property bridge. The parser is a self-contained state machine
//! over the text, so it lives with the design model rather than with the
//! property-editing dialogs that happened to host it.

use std::collections::HashMap;

/// Parses a SPICE-format parameter string into a key-value HashMap.
///
/// Supports multiple formats commonly used in SPICE:
/// - Space-separated: `m=2 tc1=0.01 tc2=0.001`
/// - Comma-separated: `m=2, tc1=0.01, tc2=0.001`
/// - Mixed: `m=2 tc1=0.01, dtemp=25`
/// - Quoted values: `model="nmos_3p3" region="saturation"`
///
/// # Arguments
/// * `params` - The parameter string to parse
///
/// # Returns
/// HashMap of parameter name to string value
///
/// # Example
/// ```ignore
/// let params = parse_params_string("m=2 tc1=0.01 tc2=0.001");
/// assert_eq!(params.get("m"), Some(&"2".to_string()));
/// assert_eq!(params.get("tc1"), Some(&"0.01".to_string()));
/// ```
pub fn parse_params_string(params: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    if params.trim().is_empty() {
        return result;
    }

    // State machine for parsing
    let mut current_key = String::new();
    let mut current_value = String::new();
    let mut in_value = false;
    let mut in_quotes = false;
    let mut quote_char = '"';

    for ch in params.chars() {
        match ch {
            // Quote handling
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = ch;
            }
            c if c == quote_char && in_quotes => {
                in_quotes = false;
            }
            // Equals sign starts value
            '=' if !in_quotes && !in_value => {
                in_value = true;
            }
            // Separators end current key-value pair
            ' ' | ',' | '\t' | '\n' if !in_quotes && in_value => {
                let key = current_key.trim().to_lowercase();
                let value = current_value.trim().to_string();
                if !key.is_empty() && !value.is_empty() {
                    result.insert(key, value);
                }
                current_key.clear();
                current_value.clear();
                in_value = false;
            }
            // Skip separators when not in value
            ' ' | ',' | '\t' | '\n' if !in_quotes && !in_value => {
                // Skip leading separators
            }
            // Accumulate characters
            _ => {
                if in_value {
                    current_value.push(ch);
                } else {
                    current_key.push(ch);
                }
            }
        }
    }

    // Handle final key-value pair
    let key = current_key.trim().to_lowercase();
    let value = current_value.trim().to_string();
    if !key.is_empty() && !value.is_empty() {
        result.insert(key, value);
    }

    result
}
/// ```ignore
/// let mut params = HashMap::new();
/// params.insert("m".to_string(), "2".to_string());
/// params.insert("tc1".to_string(), "0.01".to_string());
/// let result = format_params_string(&params);
/// // Result: "m=2 tc1=0.01" (order may vary)
/// ```
pub fn format_params_string(params: &HashMap<String, String>) -> String {
    let mut pairs: Vec<_> = params
        .iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(k, v)| {
            // Quote values containing spaces or special characters
            if v.contains(' ') || v.contains(',') || v.contains('=') {
                format!("{}=\"{}\"", k, v)
            } else {
                format!("{}={}", k, v)
            }
        })
        .collect();

    // Sort for deterministic output (important for testing and diffs)
    pairs.sort();
    pairs.join(" ")
}
