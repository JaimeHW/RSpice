//! VCD signal and bus selection, preserving scopes, aliases and quantity.
use super::*;

// -------------------------------------------------------------------------
// Selection

/// Apply `convert`'s `--variables`, `--start` and `--stop` to a dump.
///
/// A variable is named the way any of the CLI's spellings of it reads: the
/// column spelling `D(node)`, the scoped name a viewer shows, or the bare node
/// name. Clipping retains changes inside the range and carries each signal's
/// held value to the start of the range, refining the timescale if necessary.
///
/// # Buses
///
/// A vector variable answers to its bus name (`data`), to its declared range
/// in either spelling (`data[7:0]`, `data [7:0]`), and to any one of its bits
/// (`data[3]`). The last of those selects the *whole* vector, because a VCD
/// vector is all-or-nothing — a `$var` is as wide as it is declared, and there
/// is no dump that carries one bit of one. That widening is returned as a note
/// rather than done silently, since the caller asked for less than it got.
///
/// The returned notes are informational; an unknown name is still refused.
#[must_use = "the notes say where a selection was widened past what was asked"]
pub(crate) fn select_and_clip(
    document: &mut VcdDocument,
    requested: &[String],
    start: Option<f64>,
    stop: Option<f64>,
) -> Result<Vec<String>, CliError> {
    let mut notes = Vec::new();
    if !requested.is_empty() {
        let names = column_names(document);
        let depth = shared_scope_depth(
            document
                .signals
                .iter()
                .filter_map(|signal| signal.variables.first()),
        );
        let shared_scope = document
            .signals
            .first()
            .and_then(|signal| signal.variables.first())
            .map(|variable| &variable.scope[..depth])
            .unwrap_or_default();
        let mut keep = vec![false; document.signals.len()];
        for want in requested {
            let mut matches = Vec::new();
            for (index, (signal, name)) in document.signals.iter().zip(&names).enumerate() {
                match signal_selection(signal, name, want, shared_scope) {
                    Selection::No => continue,
                    Selection::Whole => {}
                    Selection::WholeBusForOneBit { bus } => {
                        notes.push(format!(
                            "'{want}' names one bit of digital bus '{bus}', and a VCD vector is \
                             written whole or not at all, so the whole bus is kept; convert to a \
                             table format to select one member column"
                        ));
                    }
                }
                matches.push(index);
            }
            if matches.is_empty() {
                return Err(CliError::InvalidArgument {
                    message: format!("variable '{want}' not found in input"),
                    suggestion: Some(format!("available variables: {}", names.join(", "))),
                });
            }
            if matches.len() > 1 {
                return Err(CliError::InvalidArgument {
                    message: format!("variable selector '{want}' is ambiguous"),
                    suggestion: Some(format!(
                        "use a full signal name: {}",
                        matches
                            .iter()
                            .map(|&index| names[index].as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )),
                });
            }
            keep[matches[0]] = true;
        }
        let mut keep = keep.into_iter();
        document.signals.retain(|_| keep.next().unwrap_or(false));
    }

    clipping::clip(document, start, stop)?;

    Ok(notes)
}

/// What one `--variables` name asks of one signal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Selection {
    /// The name does not reach this signal.
    No,
    /// The name reaches this signal, and asks for exactly it.
    Whole,
    /// The name reaches one bit of this vector, which is kept whole.
    WholeBusForOneBit {
        /// The bus name, without its range, for the note.
        bus: String,
    },
}

/// Whether one `--variables` name selects this signal, and what it cost.
pub(super) fn signal_selection(
    signal: &VcdSignal,
    column: &str,
    want: &str,
    shared_scope: &[String],
) -> Selection {
    let want = want.trim();
    if column.eq_ignore_ascii_case(want) {
        return Selection::Whole;
    }
    if let Some(inner) = inner_name(want, DIGITAL_COLUMN_PREFIX) {
        return bus_selection(signal, column, inner, shared_scope);
    }
    if inner_name(want, REAL_COLUMN_PREFIX).is_some() {
        return Selection::No;
    }
    if signal_matches(signal, column, want) {
        return Selection::Whole;
    }
    bus_selection(signal, column, want, shared_scope)
}

/// Whether one `--variables` name selects this signal by its exact spelling.
fn signal_matches(signal: &VcdSignal, column: &str, want: &str) -> bool {
    if column.eq_ignore_ascii_case(want) {
        return true;
    }
    if inner_name(column, DIGITAL_COLUMN_PREFIX)
        .or_else(|| inner_name(column, REAL_COLUMN_PREFIX))
        .is_some_and(|inner| inner.eq_ignore_ascii_case(want))
    {
        return true;
    }
    signal.variables.iter().any(|variable| {
        variable.scoped_name().eq_ignore_ascii_case(want)
            || variable.name.eq_ignore_ascii_case(want)
    })
}

/// Whether one `--variables` name reaches this signal as a bus.
///
/// A vector variable is declared `name [msb:lsb]`, so its bare name is not one
/// of its spellings and neither is the closed-up `name[msb:lsb]` a user is at
/// least as likely to type. Both are resolved here through the same
/// [`split_bus_notation`] grammar core writes the reference with, and so is a
/// bit-select of an index the range actually covers.
fn bus_selection(
    signal: &VcdSignal,
    column: &str,
    want: &str,
    shared_scope: &[String],
) -> Selection {
    if signal.kind != VcdSignalKind::Logic || signal.width <= 1 {
        return Selection::No;
    }
    let references = inner_name(column, DIGITAL_COLUMN_PREFIX)
        .map(str::to_owned)
        .into_iter()
        .chain(signal.variables.iter().flat_map(|variable| {
            let scope = variable
                .scope
                .strip_prefix(shared_scope)
                .unwrap_or(&variable.scope);
            let relative = scope
                .iter()
                .map(String::as_str)
                .chain(std::iter::once(variable.name.as_str()))
                .collect::<Vec<_>>()
                .join(".");
            [variable.scoped_name(), relative, variable.name.clone()]
        }));
    for reference in references {
        let (base, range) = split_bus_notation(&reference);
        let (msb, lsb) = range.unwrap_or((i64::from(signal.width) - 1, 0));
        // The bus by name, or by its range in either spelling.
        let (wanted_base, wanted_range) = split_bus_notation(want);
        if wanted_base.eq_ignore_ascii_case(base)
            && (wanted_range.is_none() || wanted_range == Some((msb, lsb)))
        {
            return Selection::Whole;
        }
        // One of its bits. `split_bus_notation` deliberately leaves a
        // bit-select whole — it is the name of a conductor, not a range — so
        // the index is read here.
        if let Some(index) = bit_select_index(want, base)
            && (msb.min(lsb)..=msb.max(lsb)).contains(&index)
        {
            return Selection::WholeBusForOneBit {
                bus: base.to_string(),
            };
        }
    }
    Selection::No
}

/// The `k` of `base[k]`, when `want` is spelled that way for this base.
fn bit_select_index(want: &str, base: &str) -> Option<i64> {
    let trimmed = want.trim_end();
    let open = trimmed.rfind('[')?;
    if !trimmed[..open].trim_end().eq_ignore_ascii_case(base) {
        return None;
    }
    trimmed[open + 1..].strip_suffix(']')?.trim().parse().ok()
}
