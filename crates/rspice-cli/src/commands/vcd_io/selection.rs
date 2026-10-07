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
    select_with_buses(document, &[], requested, start, stop)
}

/// Original vector metadata plus its scalar output indices in declaration order.
/// The metadata carries no value history; expansion owns that history once.
pub(super) struct ExpandedBus {
    pub signal: VcdSignal,
    pub members: Vec<usize>,
}

pub(super) fn trace_buses(
    path: &Path,
    document: &VcdDocument,
    buses: &[DigitalBusDeclaration],
) -> Result<Vec<ExpandedBus>, CliError> {
    let indices: std::collections::HashMap<_, _> = document
        .signals
        .iter()
        .enumerate()
        .filter(|(_, signal)| signal.kind == VcdSignalKind::Logic)
        .filter_map(|(index, signal)| {
            signal
                .variables
                .first()
                .map(|variable| (variable.name.to_ascii_lowercase(), index))
        })
        .collect();
    rspice_core::engine::validate_digital_bus_table(buses, indices.keys().map(String::as_str))
        .map_err(|error| conversion_error(path, error))?;
    buses
        .iter()
        .map(|bus| {
            let members = bus
                .members
                .iter()
                .map(|member| {
                    indices
                        .get(&member.to_ascii_lowercase())
                        .copied()
                        .ok_or_else(|| {
                            conversion_error(path, format!("missing bus member '{member}'"))
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ExpandedBus {
                signal: VcdSignal {
                    identifier: String::new(),
                    variables: vec![VcdVariable {
                        scope: vec![EVENT_SCOPE.to_owned()],
                        name: format!("{} [{}:{}]", bus.name, bus.msb, bus.lsb),
                    }],
                    width: members.len() as u32,
                    kind: VcdSignalKind::Logic,
                    changes: Vec::new(),
                },
                members,
            })
        })
        .collect()
}

pub(super) fn select_with_buses(
    document: &mut VcdDocument,
    buses: &[ExpandedBus],
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
        let bus_names: Vec<_> = buses
            .iter()
            .map(|bus| column_name(&bus.signal, depth))
            .collect();
        let mut keep = vec![false; document.signals.len()];
        for want in requested {
            // Key by the selected output signals: a bit's native name and its
            // original bus alias can identify exactly the same scalar.
            let mut matches = std::collections::BTreeMap::new();
            let ambiguous = || CliError::InvalidArgument {
                message: format!("variable selector '{want}' is ambiguous"),
                suggestion: Some("use an unambiguous full signal or bus name".into()),
            };
            for (index, (signal, name)) in document.signals.iter().zip(&names).enumerate() {
                let note = match signal_selection(signal, name, want, shared_scope) {
                    Selection::No => continue,
                    Selection::Ambiguous => return Err(ambiguous()),
                    Selection::Whole => None,
                    Selection::WholeBusForOneBit { .. } if signal.width == 1 => None,
                    Selection::WholeBusForOneBit { bus, .. } => Some(format!(
                        "'{want}' names one bit of digital bus '{bus}', and a VCD vector is written whole or not at all, so the whole bus is kept; use --expand-buses or convert to a table format to select one member"
                    )),
                };
                matches.insert(vec![index], (name.as_str(), note));
            }
            for (bus, name) in buses.iter().zip(&bus_names) {
                let mut members = match signal_selection(&bus.signal, name, want, shared_scope) {
                    Selection::No => continue,
                    Selection::Ambiguous => return Err(ambiguous()),
                    Selection::Whole => bus.members.clone(),
                    Selection::WholeBusForOneBit { position, .. } => vec![bus.members[position]],
                };
                members.sort_unstable();
                matches.entry(members).or_insert((name.as_str(), None));
            }
            if matches.is_empty() {
                return Err(CliError::InvalidArgument {
                    message: format!("variable '{want}' not found in input"),
                    suggestion: Some(format!(
                        "available variables: {}",
                        names
                            .iter()
                            .chain(&bus_names)
                            .map(String::as_str)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )),
                });
            }
            if matches.len() > 1 {
                return Err(CliError::InvalidArgument {
                    message: format!("variable selector '{want}' is ambiguous"),
                    suggestion: Some(format!(
                        "use a full signal name: {}",
                        matches
                            .values()
                            .map(|(name, _)| *name)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )),
                });
            }
            let (indices, (_, note)) = matches.pop_first().expect("one matching selection");
            for index in indices {
                keep[index] = true;
            }
            if let Some(note) = note {
                notes.push(note);
            }
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
    /// Different aliases of one vector assign this bit name to different positions.
    Ambiguous,
    /// The name reaches this signal, and asks for exactly it.
    Whole,
    /// The name reaches one bit of this vector, which is kept whole.
    WholeBusForOneBit {
        /// The bus name, without its range, for the note.
        bus: String,
        /// Position within the vector, most significant first.
        position: usize,
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
    let want = if let Some(inner) = inner_name(want, DIGITAL_COLUMN_PREFIX) {
        if signal.kind != VcdSignalKind::Logic {
            return Selection::No;
        }
        inner
    } else if let Some(inner) = inner_name(want, REAL_COLUMN_PREFIX) {
        if signal.kind != VcdSignalKind::Real {
            return Selection::No;
        }
        inner
    } else {
        want
    };
    if signal_references(signal, column, shared_scope)
        .any(|reference| reference.eq_ignore_ascii_case(want))
    {
        return Selection::Whole;
    }
    bus_selection(signal, column, want, shared_scope)
}

/// Raw references, never quantity wrappers: a node literally named `D(clk)`
/// answers to `D(D(clk))`, without colliding with the digital node `clk`.
fn signal_references<'a>(
    signal: &'a VcdSignal,
    column: &'a str,
    shared_scope: &'a [String],
) -> impl Iterator<Item = String> + 'a {
    inner_name(column, DIGITAL_COLUMN_PREFIX)
        .or_else(|| inner_name(column, REAL_COLUMN_PREFIX))
        .map(str::to_owned)
        .into_iter()
        .chain(signal.variables.iter().flat_map(move |variable| {
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
        }))
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
    if signal.kind != VcdSignalKind::Logic
        || (signal.width <= 1
            && !signal
                .variables
                .iter()
                .any(|variable| split_bus_notation(&variable.name).1.is_some()))
    {
        return Selection::No;
    }
    let mut selected = Selection::No;
    for reference in signal_references(signal, column, shared_scope) {
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
            let position = msb.abs_diff(index) as usize;
            if matches!(selected, Selection::WholeBusForOneBit { position: previous, .. } if previous != position)
            {
                return Selection::Ambiguous;
            }
            selected = Selection::WholeBusForOneBit {
                bus: base.to_string(),
                position,
            };
        }
    }
    selected
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
