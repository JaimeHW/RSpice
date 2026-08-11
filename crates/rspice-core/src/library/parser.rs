//! Library Content Parser
//!
//! Parses SPICE library files to extract .MODEL and .SUBCKT definitions.
//! Handles multi-line continuations and extracts descriptions from comments.

use super::manager::{ModelDefinition, ModelType, SubcircuitDefinition};
use std::sync::Arc;

/// Structural pieces shared by both library importers. `tail` is never
/// rewritten, so parentheses belonging to parameter expressions remain
/// distinct from the optional outer formal-port delimiters.
pub(super) struct LibrarySubcircuitHeader<'a> {
    pub(super) name: &'a str,
    pub(super) parenthesized_ports: Option<&'a str>,
    pub(super) tail: &'a str,
}

pub(super) fn split_library_subcircuit_header(
    header: &str,
) -> Result<LibrarySubcircuitHeader<'_>, &'static str> {
    let header = header.trim();
    let mut cursor = 0usize;
    while cursor < header.len() {
        let character = header[cursor..]
            .chars()
            .next()
            .expect("cursor remains on a character boundary");
        if character.is_whitespace() || character == ',' {
            break;
        }
        cursor += character.len_utf8();
    }
    let name = &header[..cursor];
    if name.is_empty() {
        return Err("declaration is missing its subcircuit name");
    }

    let wrapper_may_follow = header[cursor..]
        .chars()
        .next()
        .is_some_and(char::is_whitespace);
    cursor = skip_header_separators(header, cursor);
    let Some(first_after_name) = header[cursor..].chars().next() else {
        return Ok(LibrarySubcircuitHeader {
            name,
            parenthesized_ports: None,
            tail: "",
        });
    };
    if first_after_name == ')' {
        return Err("closing ')' has no matching parenthesized formal-port list");
    }
    if first_after_name != '(' || !wrapper_may_follow {
        return Ok(LibrarySubcircuitHeader {
            name,
            parenthesized_ports: None,
            tail: &header[cursor..],
        });
    }

    let mut closing_parenthesis = None;
    let opening_parenthesis = cursor;
    cursor += first_after_name.len_utf8();
    for (relative_offset, character) in header[cursor..].char_indices() {
        match character {
            '(' => return Err("nested parentheses are not valid in a formal-port list"),
            ')' => {
                closing_parenthesis = Some(cursor + relative_offset);
                break;
            }
            _ => {}
        }
    }

    let closing_parenthesis =
        closing_parenthesis.ok_or("parenthesized formal-port list is missing its closing ')'")?;
    let ports = header[opening_parenthesis + '('.len_utf8()..closing_parenthesis].trim();
    if split_library_formal_ports(ports).next().is_none() {
        return Err("parenthesized formal-port list cannot be empty");
    }

    let after_closing = &header[closing_parenthesis + ')'.len_utf8()..];
    if after_closing
        .chars()
        .next()
        .is_some_and(|character| !character.is_whitespace())
    {
        return Err("content after a parenthesized formal-port list must be whitespace-separated");
    }
    let tail = after_closing.trim_start();
    if !tail.is_empty() && !is_subcircuit_interface_boundary(tail) {
        return Err(
            "only OPTIONAL or parameter declarations may follow a parenthesized formal-port list",
        );
    }

    Ok(LibrarySubcircuitHeader {
        name,
        parenthesized_ports: Some(ports),
        tail,
    })
}

fn skip_header_separators(source: &str, mut cursor: usize) -> usize {
    while cursor < source.len() {
        let character = source[cursor..]
            .chars()
            .next()
            .expect("cursor remains on a character boundary");
        if !character.is_whitespace() && character != ',' {
            break;
        }
        cursor += character.len_utf8();
    }
    cursor
}

pub(super) fn split_library_formal_ports(source: &str) -> impl Iterator<Item = &str> {
    source
        .split(|character: char| character.is_whitespace() || character == ',')
        .filter(|field| !field.is_empty())
}

pub(super) fn is_subcircuit_params_marker(field: &str) -> bool {
    [
        "param",
        "param:",
        "params",
        "params:",
        "parameters",
        "parameters:",
    ]
    .iter()
    .any(|marker| field.eq_ignore_ascii_case(marker))
}

pub(super) fn is_subcircuit_optional_marker(field: &str) -> bool {
    field.eq_ignore_ascii_case("optional") || field.eq_ignore_ascii_case("optional:")
}

pub(super) fn is_subcircuit_interface_boundary(source: &str) -> bool {
    let mut fields = source.split_whitespace();
    let Some(first) = fields.next() else {
        return false;
    };
    is_subcircuit_params_marker(first)
        || is_subcircuit_optional_marker(first)
        || first.contains('=')
        || fields.next().is_some_and(|field| field == "=")
}

fn directive_rest<'a>(line: &'a str, directive: &str) -> Option<&'a str> {
    let line = line.trim();
    let token_end = line.find(char::is_whitespace).unwrap_or(line.len());
    line[..token_end]
        .eq_ignore_ascii_case(directive)
        .then(|| line[token_end..].trim())
}

fn fold_library_continuations(content: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for physical_line in content.lines() {
        let line = physical_line.trim();
        if let Some(continuation) = line.strip_prefix('+')
            && let Some(previous) = lines.last_mut()
            && !previous.is_empty()
        {
            previous.push(' ');
            previous.push_str(continuation.trim());
            continue;
        }
        lines.push(line.to_owned());
    }
    lines
}

/// Parse library content and extract all model and subcircuit definitions
pub fn parse_library_content(
    content: &'static str,
    library_name: &str,
) -> (Vec<ModelDefinition>, Vec<SubcircuitDefinition>) {
    let mut models = Vec::new();
    let mut subcircuits = Vec::new();
    let library_name: Arc<str> = Arc::from(library_name);

    let lines = fold_library_continuations(content);
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

        // Track subcircuit blocks
        if directive_rest(line, ".subckt").is_some() {
            // Parse subcircuit at top level only
            if subckt_depth == 0
                && let Some(subckt) =
                    parse_subckt_line(line, Arc::clone(&library_name), &last_comment)
            {
                subcircuits.push(subckt);
            }
            subckt_depth += 1;
            last_comment.clear();
        } else if directive_rest(line, ".ends").is_some() {
            subckt_depth = subckt_depth.saturating_sub(1);
        }
        // Parse .MODEL directive - only at top level (outside subcircuits)
        else if directive_rest(line, ".model").is_some() && subckt_depth == 0 {
            if let Some(model) = parse_model_line(line, Arc::clone(&library_name), &last_comment) {
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
    library_name: Arc<str>,
    description: &str,
) -> Option<ModelDefinition> {
    // Remove the complete .MODEL directive token, case-insensitively.
    let rest = directive_rest(line, ".model")?;

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
    library_name: Arc<str>,
    description: &str,
) -> Option<SubcircuitDefinition> {
    // Remove the complete .SUBCKT directive token, case-insensitively.
    let rest = directive_rest(line, ".subckt")?;

    let header = split_library_subcircuit_header(rest).ok()?;
    let name = header.name.to_string();

    // Stop the public interface at OPTIONAL/PARAMS/default-assignment
    // boundaries. Optional defaults are metadata, never formal terminals.
    let pins: Vec<String> = if let Some(formals) = header.parenthesized_ports {
        let fields = split_library_formal_ports(formals);
        let mut pins = Vec::new();
        for field in fields {
            if is_subcircuit_params_marker(field)
                || is_subcircuit_optional_marker(field)
                || field.contains(['=', '(', ')'])
            {
                return None;
            }
            pins.push(field.to_string());
        }
        pins
    } else {
        let fields: Vec<&str> = split_library_formal_ports(header.tail).collect();
        let mut pins = Vec::new();
        let mut index = 0usize;
        while index < fields.len() {
            let field = fields[index];
            if is_subcircuit_params_marker(field)
                || is_subcircuit_optional_marker(field)
                || field.contains('=')
                || fields.get(index + 1).is_some_and(|field| *field == "=")
            {
                break;
            }
            if field.contains(['(', ')']) {
                return None;
            }
            pins.push(field.to_string());
            index += 1;
        }
        pins
    };

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_index_accepts_one_parenthesized_formal_port_list() {
        let (_, subcircuits) = parse_library_content(
            ".SuBcKt resistor\n+ (1 2)\nr1 1 2 1k\n.eNdS resistor\n",
            "fixture",
        );

        assert_eq!(subcircuits.len(), 1);
        assert_eq!(subcircuits[0].name, "resistor");
        assert_eq!(subcircuits[0].pins, ["1", "2"]);
    }

    #[test]
    fn library_index_keeps_parameter_parentheses_out_of_the_interface() {
        let (_, subcircuits) = parse_library_content(
            ".subckt filter (input, output) OPTIONAL: reference=0 params: CURVE=lookup(1, 2)\n\
             r1 input output 1k\n\
             .ends filter\n",
            "fixture",
        );

        assert_eq!(subcircuits.len(), 1);
        assert_eq!(subcircuits[0].pins, ["input", "output"]);
    }

    #[test]
    fn library_index_preserves_parentheses_inside_subcircuit_names() {
        let (_, subcircuits) = parse_library_content(
            ".subckt S861(C1)_5000/SIE input output\n.ends S861(C1)_5000/SIE\n",
            "fixture",
        );

        assert_eq!(subcircuits.len(), 1);
        assert_eq!(subcircuits[0].name, "S861(C1)_5000/SIE");
        assert_eq!(subcircuits[0].pins, ["input", "output"]);
    }

    #[test]
    fn subckt_prefix_lookalike_does_not_change_library_scope() {
        let (models, subcircuits) = parse_library_content(
            ".SUBCKTfoo vendor extension\n\
             .MoDeL visible D\n",
            "fixture",
        );

        assert!(subcircuits.is_empty());
        assert_eq!(models.len(), 1, "lookalike must not open subcircuit scope");
        assert_eq!(models[0].name, "visible");
    }

    #[test]
    fn library_index_plain_interfaces_stop_at_all_metadata_boundaries() {
        for declaration in [
            ".subckt optional input output OPTIONAL: reference=0 PARAMS: GAIN=(1+2)\n\
             .ends optional\n",
            ".subckt bare input output GAIN=(1+2)\n.ends bare\n",
            ".subckt param input output PARAM GAIN=(1+2)\n.ends param\n",
            ".subckt param_colon input output PARAM: GAIN=(1+2)\n.ends param_colon\n",
            ".subckt parameters input output PARAMETERS GAIN=(1+2)\n.ends parameters\n",
            ".subckt parameters_colon input output PARAMETERS: GAIN=(1+2)\n\
             .ends parameters_colon\n",
        ] {
            let (_, subcircuits) = parse_library_content(declaration, "fixture");
            assert_eq!(subcircuits.len(), 1, "{declaration:?}");
            assert_eq!(subcircuits[0].pins, ["input", "output"]);
        }
    }

    #[test]
    fn library_index_rejects_malformed_parenthesized_formal_lists() {
        for declaration in [
            ".subckt empty ()\n.ends empty\n",
            ".subckt nested ((a b))\n.ends nested\n",
            ".subckt unclosed (a b\n.ends unclosed\n",
            ".subckt trailing (a b) c\n.ends trailing\n",
            ".subckt assigned (a P=1)\n.ends assigned\n",
            ".subckt adjacent(a b)\n.ends adjacent\n",
        ] {
            let (_, subcircuits) = parse_library_content(declaration, "fixture");
            assert!(
                subcircuits.is_empty(),
                "malformed declaration must fail closed: {declaration:?}"
            );
        }
    }
}
