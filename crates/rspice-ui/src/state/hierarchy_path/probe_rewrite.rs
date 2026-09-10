//! Lossless instance-path edits in V/I probe calls. Arithmetic, function
//! arguments, quoted text outside probes, and unrelated node names stay authored.

use super::{InstancePath, ProbeTarget};

pub(crate) fn remap_instance_probes(
    expression: &str,
    from: &InstancePath,
    to: &InstancePath,
) -> Result<Option<String>, String> {
    remap_instance_probes_many(expression, &[(from.clone(), to.clone())])
}

/// Apply a simultaneous identity map once to each original probe argument.
/// Destinations already include ancestor edits; longest-prefix matching keeps
/// a child rename from being lost when its parent is renamed in the same edit.
pub(crate) fn remap_instance_probes_many(
    expression: &str,
    mappings: &[(InstancePath, InstancePath)],
) -> Result<Option<String>, String> {
    let mut chars = expression.char_indices().peekable();
    let mut edits = Vec::new();
    while let Some((start, character)) = chars.next() {
        if character == '"' {
            // A quoted signal/alias is not an executable probe call.
            while let Some((_, character)) = chars.next() {
                if character == '\\' {
                    chars.next();
                } else if character == '"' {
                    break;
                }
            }
            continue;
        }
        if !character.is_alphabetic() && character != '_' {
            continue;
        }
        let mut end = start + character.len_utf8();
        while let Some(&(offset, character)) = chars.peek() {
            if !character.is_alphanumeric() && !matches!(character, '_' | '.' | ':') {
                break;
            }
            end = offset + character.len_utf8();
            chars.next();
        }
        let name = &expression[start..end];
        if !name.eq_ignore_ascii_case("v") && !name.eq_ignore_ascii_case("i") {
            continue;
        }
        while chars.peek().is_some_and(|(_, c)| c.is_whitespace()) {
            chars.next();
        }
        if chars.peek().is_none_or(|(_, c)| *c != '(') {
            continue;
        }
        let (open, _) = chars.next().expect("observed opening parenthesis");
        let mut argument_start = open + 1;
        let mut quoted = false;
        let mut escaped = false;
        for (offset, character) in chars.by_ref() {
            if escaped {
                escaped = false;
                continue;
            }
            if quoted && character == '\\' {
                escaped = true;
                continue;
            }
            if character == '"' {
                quoted = !quoted;
            } else if !quoted && matches!(character, ',' | ')') {
                if let Some(replacement) = remap_argument(
                    &expression[argument_start..offset],
                    mappings,
                    name.eq_ignore_ascii_case("i"),
                )? {
                    edits.push((argument_start..offset, replacement));
                }
                argument_start = offset + 1;
                if character == ')' {
                    break;
                }
            } else if !quoted && character == '(' {
                // Nested arithmetic is not a valid raw probe argument.
                break;
            }
        }
    }
    if edits.is_empty() {
        return Ok(None);
    }
    let mut result = String::with_capacity(expression.len());
    let mut copied = 0;
    for (range, replacement) in edits {
        result.push_str(&expression[copied..range.start]);
        result.push_str(&replacement);
        copied = range.end;
    }
    result.push_str(&expression[copied..]);
    Ok(Some(result))
}

fn remap_argument(
    argument: &str,
    mappings: &[(InstancePath, InstancePath)],
    current: bool,
) -> Result<Option<String>, String> {
    let trimmed = argument.trim();
    let quoted = trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2;
    let raw = if quoted {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    };
    let Ok(target) = ProbeTarget::parse(raw) else {
        return Ok(None);
    };
    let device = current
        .then(|| target.scope.child(&target.leaf).ok())
        .flatten();
    let Some((from, to)) = mappings
        .iter()
        .filter(|(from, _)| {
            target.scope.starts_with(from)
                || device
                    .as_ref()
                    .is_some_and(|device| device.fold_key() == from.fold_key())
        })
        .max_by_key(|(from, _)| from.depth())
    else {
        return Ok(None);
    };
    let moved = if let Some(tail) = target.scope.strip_prefix(from) {
        ProbeTarget {
            scope: to.join(&tail).map_err(|error| error.to_string())?,
            leaf: target.leaf,
        }
    } else if current
        && target
            .scope
            .child(&target.leaf)
            .is_ok_and(|path| path.fold_key() == from.fold_key())
    {
        let Some((leaf, _)) = to.segments().split_last() else {
            return Err("A device probe cannot be moved onto the design root.".to_owned());
        };
        ProbeTarget {
            scope: to.parent().expect("non-root target"),
            leaf: leaf.clone(),
        }
    } else {
        return Ok(None);
    };
    let moved = moved.to_string();
    if moved == raw {
        return Ok(None);
    }
    let leading = argument.len() - argument.trim_start().len() + usize::from(quoted);
    let trailing = argument.len() - argument.trim_end().len() + usize::from(quoted);
    Ok(Some(format!(
        "{}{moved}{}",
        &argument[..leading],
        &argument[argument.len() - trailing..]
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simultaneous_probe_swaps_and_nested_renames_use_original_arguments_once() {
        let mappings = [
            ("/V1", "/V2"),
            ("/V2", "/V1"),
            ("/X1", "/X9"),
            ("/X1/V1", "/X9/V3"),
        ]
        .map(|(from, to)| {
            (
                InstancePath::parse(from).unwrap(),
                InstancePath::parse(to).unwrap(),
            )
        });
        assert_eq!(
            remap_instance_probes_many("I(V1)-I(V2)+I(/X1/V1)+V(/X1/n)+V(V1)", &mappings,)
                .unwrap()
                .as_deref(),
            Some("I(V2)-I(V1)+I(/X9/V3)+V(/X9/n)+V(V1)")
        );
        let reversed = mappings.into_iter().rev().collect::<Vec<_>>();
        assert_eq!(
            remap_instance_probes_many("I(/X1/V1)", &reversed)
                .unwrap()
                .as_deref(),
            Some("I(/X9/V3)")
        );
    }

    #[test]
    fn only_probe_instance_paths_change_and_authored_formatting_survives() {
        let from = InstancePath::parse("/X1").unwrap();
        let to = InstancePath::parse("/X9").unwrap();
        for (input, expected) in [
            ("V(/X1/n) - V(/X2/n)", "V(/X9/n) - V(/X2/n)"),
            ("V( /X1/n , /X1/p )", "V( /X9/n , /X9/p )"),
            ("abs(V(/X1/n))", "abs(V(/X9/n))"),
            ("(I(X1) + V(X1)) * 2", "(I(X9) + V(X1)) * 2"),
            ("I ( \"X1\" ) + v(\"x1.n\")", "I ( \"X9\" ) + v(\"/X9/n\")"),
            ("param(X1) + avg(I(X1))", "param(X1) + avg(I(X9))"),
        ] {
            assert_eq!(
                remap_instance_probes(input, &from, &to).unwrap().as_deref(),
                Some(expected),
                "{input}"
            );
        }
        for input in [
            "(1+2)*V(out)",
            "param(X1)",
            "S(X1, X1)",
            "V(X1)",
            "\"I(X1)\"",
            "someI(X1)",
        ] {
            assert_eq!(
                remap_instance_probes(input, &from, &to).unwrap(),
                None,
                "{input}"
            );
        }
    }
}
