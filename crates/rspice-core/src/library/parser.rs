//! The `.SUBCKT` header grammar of a library file: where the name ends, where
//! the formal ports end, and which tokens open the parameter declarations.

/// Structural pieces of a library `.SUBCKT` header. `tail` is never
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
