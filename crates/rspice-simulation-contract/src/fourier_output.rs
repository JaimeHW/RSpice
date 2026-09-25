//! Canonical output spelling and parsing for authored Fourier cards.

pub fn fourier_output_is_current(output_node: &str) -> bool {
    let output = output_node.trim();
    output
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("I("))
        && output.ends_with(')')
}

pub fn validate_fourier_output_accessor(
    output_node: &str,
    output_ref: Option<&str>,
) -> Result<(), String> {
    let output = output_node.trim();
    if output.is_empty() {
        return Err("Fourier output node must be specified".to_owned());
    }
    if fourier_output_is_current(output) {
        let device = output[2..output.len() - 1].trim();
        if device.is_empty() || device.contains(',') || device.contains('(') || device.contains(')')
        {
            return Err("Fourier current output must identify exactly one device".to_owned());
        }
        if output_ref.is_some_and(|reference| !reference.trim().is_empty()) {
            return Err("Fourier current output must not specify a voltage reference".to_owned());
        }
        return Ok(());
    }

    let node = if output
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V("))
        && output.ends_with(')')
    {
        output[2..output.len() - 1].trim()
    } else {
        if output.contains('(') || output.contains(')') {
            return Err(
                "Fourier output must be a node, V(node), or qualified I(device)".to_owned(),
            );
        }
        output
    };
    if node.is_empty() || node.contains(',') || node.contains('(') || node.contains(')') {
        return Err("Fourier voltage output must identify exactly one positive node".to_owned());
    }
    if let Some(reference) = output_ref {
        let reference = reference.trim();
        if reference.contains(',') || reference.contains('(') || reference.contains(')') {
            return Err("Fourier voltage reference must identify exactly one node".to_owned());
        }
    }
    Ok(())
}

/// Split one card-spelled `.FOUR` output into the positive accessor and the
/// voltage reference the run config carries separately.
///
/// The card has no reference field: a differential output is `V(out,ref)`
/// inside the output itself, which is the one spelling every route reads. The
/// reference comes back as `"0"` for a single-ended output, which every reader
/// downstream already treats as ground.
pub fn split_fourier_output(output: &str) -> Result<(String, String), String> {
    let trimmed = output.trim();
    if fourier_output_is_current(trimmed) {
        let device = trimmed[2..trimmed.len() - 1].trim();
        if device.is_empty() || device.contains(',') {
            return Err("a current output must identify exactly one device".to_owned());
        }
        return Ok((format!("I({device})"), String::new()));
    }
    if trimmed.len() < 4
        || !trimmed
            .get(..2)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V("))
        || !trimmed.ends_with(')')
    {
        return Err("use V(node), V(node+, node-), or I(device)".to_owned());
    }
    let nodes = trimmed[2..trimmed.len() - 1]
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>();
    match nodes.as_slice() {
        [node] if !node.is_empty() => Ok(((*node).to_owned(), "0".to_owned())),
        [positive, reference] if !positive.is_empty() && !reference.is_empty() => {
            Ok(((*positive).to_owned(), (*reference).to_owned()))
        }
        _ => Err("a voltage output carries one node or one differential node pair".to_owned()),
    }
}

/// Spell one output the way the `.FOUR` card writes it.
pub fn fourier_card_output(output_node: &str, output_ref: &str) -> String {
    let output = output_node.trim();
    if fourier_output_is_current(output) {
        return format!("I({})", output[2..output.len() - 1].trim());
    }
    let node = if output
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V("))
        && output.ends_with(')')
    {
        output[2..output.len() - 1].trim()
    } else {
        output
    };
    let reference = output_ref.trim();
    if reference.is_empty() || reference.eq_ignore_ascii_case("0") {
        format!("V({node})")
    } else {
        format!("V({node},{reference})")
    }
}
