//! A frame's timing checks (IEEE 1364-2005 section 15) and the module paths
//! that delay their delayed signals (sections 15.5.1 and 15.8).

use super::*;
use crate::ast::{SpecifyTimingCheck, TimingCheckEvent, TimingCheckKind};
use crate::canonical_ir::digital::{
    DigitalCheckCondition, DigitalCheckEvent, DigitalModulePath, DigitalPathDelay,
    DigitalTimingCheck, DigitalTimingCheckKind,
};
use crate::numeric_literal::NumericLiteralValue::{Integer, Real};
use std::collections::{BTreeMap, BTreeSet};

/// Lower `checks`, their names resolved in `scope` and their messages
/// naming `instance` (empty for the compiled module itself).
pub(super) fn lower_timing_checks(
    checks: &[SpecifyTimingCheck],
    (scope, instance, signals): (&HashMap<&str, DigitalSignalId>, &str, &[DigitalSignal]),
    constants: (&ResolvedConstants, crate::time_scale::ModuleTimeScale),
    precision: i8,
    (lowered, paths): (&mut Vec<DigitalTimingCheck>, &mut Vec<DigitalModulePath>),
) -> Result<(), DigitalLoweringDiagnostic> {
    let resolve = |name: &SmolStr, span: crate::source::Span| {
        scope.get(name.as_str()).copied().ok_or_else(|| {
            DigitalLoweringDiagnostic::refusal(
                format!("timing check terminal `{name}` names no discrete signal"),
                span.into(),
            )
        })
    };
    let first = lowered.len();
    // Each check's delayed reference and data: the signal copied and the net
    // its copy drives.
    let mut copies = Vec::with_capacity(checks.len());
    for check in checks {
        let condition = |condition: Option<&crate::ast::TimingCheckCondition>| {
            condition
                .map(|condition| {
                    Ok::<_, DigitalLoweringDiagnostic>(DigitalCheckCondition {
                        signal: resolve(&condition.signal.name, condition.signal.span)?,
                        enabling: condition.enabling,
                        unknown_enables: condition.unknown_enables,
                    })
                })
                .transpose()
        };
        let event = |event: &TimingCheckEvent| -> Result<_, DigitalLoweringDiagnostic> {
            let signal = resolve(&event.terminal.name, event.terminal.span)?;
            Ok(DigitalCheckEvent {
                signal,
                edges: event.edges,
                condition: condition(event.condition.as_ref())?,
                bits: selected_bits(
                    &event.terminal,
                    signals.iter().find(|declared| declared.id == signal),
                    constants,
                )?,
            })
        };
        let signed = matches!(
            check.kind,
            TimingCheckKind::SetupHold | TimingCheckKind::RecRem | TimingCheckKind::NoChange
        );
        let mut limits = [0_i64; 2];
        let mut written = Vec::new();
        for (slot, value) in limits.iter_mut().zip(&check.limits) {
            let (ticks, text) = limit_ticks(value, constants, precision, signed)?;
            *slot = ticks;
            written.push(text);
        }
        // Section 15.8: a negative limit shifts the window off the
        // reference. One that leaves no window at all is taken as 0, as the
        // commercial simulators take it; the host warns of it.
        let limit_zeroed = matches!(
            check.kind,
            TimingCheckKind::SetupHold | TimingCheckKind::RecRem
        ) && limits.iter().any(|limit| *limit < 0)
            && limits[0].saturating_add(limits[1]) <= 0;
        if limit_zeroed {
            for limit in &mut limits {
                *limit = (*limit).max(0);
            }
        }
        if check.kind == TimingCheckKind::NoChange && limits[1] < 0 {
            return Err(DigitalLoweringDiagnostic::refusal(
                "a `$nochange` with a negative end edge offset, whose window would close before \
                 its trailing edge is known (IEEE 1364-2005 section 15.3)"
                    .to_string(),
                check.span.into(),
            ));
        }
        let mut copied: [Option<DelayedCopy>; 2] = [None, None];
        for (slot, (delayed, source)) in copied.iter_mut().zip([
            (&check.delayed_reference, Some(&check.reference)),
            (&check.delayed_data, check.data.as_ref()),
        ]) {
            if let (Some(delayed), Some(source)) = (delayed, source) {
                *slot = Some(DelayedCopy {
                    source: resolve(&source.terminal.name, source.terminal.span)?,
                    delayed: (
                        resolve(&delayed.name, delayed.span)?,
                        match &delayed.select {
                            Some(crate::ast::TerminalSelect::Bit { text, .. }) => {
                                Some(text.clone())
                            }
                            _ => None,
                        },
                    ),
                    span: delayed.span,
                });
            }
        }
        copies.push(copied);
        let describe = |event: &TimingCheckEvent| {
            let mut text = match &event.edge_text {
                Some(edge) => format!("{edge} {}", event.terminal.name),
                None => event.terminal.name.to_string(),
            };
            if let Some(
                crate::ast::TerminalSelect::Bit { text: select, .. }
                | crate::ast::TerminalSelect::Part { text: select, .. },
            ) = &event.terminal.select
            {
                text.push('[');
                text.push_str(select);
                text.push(']');
            }
            if let Some(condition) = &event.condition {
                text.push_str(" &&& ");
                text.push_str(&condition.text);
            }
            text
        };
        let mut arguments: Vec<String> = match (check.kind, &check.data) {
            (TimingCheckKind::Setup, Some(data)) => {
                vec![describe(data), describe(&check.reference)]
            }
            (_, data) => std::iter::once(describe(&check.reference))
                .chain(data.as_ref().map(describe))
                .collect(),
        };
        arguments.extend(written);
        let label = format!("{}({})", check.kind.name(), arguments.join(", "));
        lowered.push(DigitalTimingCheck {
            kind: match check.kind {
                TimingCheckKind::Setup => DigitalTimingCheckKind::Setup,
                TimingCheckKind::Hold => DigitalTimingCheckKind::Hold,
                TimingCheckKind::SetupHold => DigitalTimingCheckKind::SetupHold,
                TimingCheckKind::Recovery => DigitalTimingCheckKind::Recovery,
                TimingCheckKind::Removal => DigitalTimingCheckKind::Removal,
                TimingCheckKind::RecRem => DigitalTimingCheckKind::RecRem,
                TimingCheckKind::Skew => DigitalTimingCheckKind::Skew,
                TimingCheckKind::TimeSkew => DigitalTimingCheckKind::TimeSkew,
                TimingCheckKind::FullSkew => DigitalTimingCheckKind::FullSkew,
                TimingCheckKind::Period => DigitalTimingCheckKind::Period,
                TimingCheckKind::Width => DigitalTimingCheckKind::Width,
                TimingCheckKind::NoChange => DigitalTimingCheckKind::NoChange,
            },
            event_based: flag(check.event_based_flag.as_ref(), constants)?,
            remain_active: flag(check.remain_active_flag.as_ref(), constants)?,
            reference: event(&check.reference)?,
            data: check.data.as_ref().map(event).transpose()?,
            limits,
            exponent: precision,
            limit_zeroed,
            delays: [0, 0],
            limits_inconsistent: false,
            timestamp_condition: condition(check.timestamp_condition.as_ref())?,
            timecheck_condition: condition(check.timecheck_condition.as_ref())?,
            notifier: check
                .notifier
                .as_ref()
                .map(|notifier| resolve(&notifier.name, notifier.span))
                .transpose()?,
            label: if instance.is_empty() {
                label
            } else {
                format!("{instance}: {label}")
            },
        });
    }
    let named: Vec<_> = copies
        .iter()
        .map(|copied| {
            [
                copied[0].as_ref().map(|copy| copy.delayed.clone()),
                copied[1].as_ref().map(|copy| copy.delayed.clone()),
            ]
        })
        .collect();
    let delays = delayed_signal_delays(&mut lowered[first..], &named);
    // One transport-delayed copy (every change passes) from each signal to
    // each delayed net that copies it.
    let mut emitted = BTreeSet::new();
    for copy in copies.iter().flatten().flatten() {
        let Some(ticks) = delays
            .get(&copy.delayed)
            .and_then(|delay| u64::try_from(*delay).ok())
            .filter(|ticks| *ticks > 0)
        else {
            continue;
        };
        let (net, bit) = &copy.delayed;
        if bit.is_some() {
            return Err(DigitalLoweringDiagnostic::refusal(
                "a delayed signal that is one bit of a net, delayed by a negative timing check \
                 limit: the module path that delays it would delay the whole net (IEEE \
                 1364-2005 section 15.8)"
                    .to_string(),
                copy.span.into(),
            ));
        }
        if !emitted.insert((copy.source, *net)) {
            continue;
        }
        paths.push(DigitalModulePath {
            source: copy.source,
            destination: *net,
            edge: None,
            condition: None,
            ifnone: false,
            delay: DigitalPathDelay {
                transitions: [ticks; 12],
                exponent: precision,
            },
            pulse_limits: Some([0, 0]),
            on_detect: false,
            show_cancelled: false,
            delayed_signal: true,
        });
    }
    Ok(())
}

/// A delayed signal: a net, and the bit of it a check names as written.
type Delayed = (DigitalSignalId, Option<SmolStr>);

/// A check's delayed signal and the signal it copies.
struct DelayedCopy {
    source: DigitalSignalId,
    delayed: Delayed,
    span: crate::source::Span,
}

/// A `$setuphold`'s or `$recrem`'s window over a data event's tick less its
/// reference event's, `(-before, after)`: setup and hold, or removal and
/// recovery.
fn window(check: &DigitalTimingCheck) -> Option<(i64, i64)> {
    let [first, second] = check.limits;
    match check.kind {
        DigitalTimingCheckKind::SetupHold => Some((first, second)),
        DigitalTimingCheckKind::RecRem => Some((second, first)),
        _ => None,
    }
}

/// The delays of a frame's delayed signals (IEEE 1364-2005 section 15.5.1),
/// written into each `$setuphold` and `$recrem` of `checks`, and the delay
/// of each delayed net `named` gives a check's reference and data.
///
/// Delaying the reference by `r` and the data by `d` takes the window
/// `(-before, after)` to `(-before - r + d, after - r + d)` between the
/// delayed signals, whose limits are both non-negative when
/// `-before <= r - d <= after`. Checks naming the same delayed net share its
/// delay, so the delays are the least non-negative ones that satisfy every
/// check naming any of them; a delayed signal a check leaves unnamed is its
/// own. Where no delays satisfy a set of checks, the section takes the
/// smallest negative limit among them as 0 and computes the delays again,
/// until delays exist, which they do once no limit is negative. Each check
/// whose limit is taken as 0 is marked for the warning the section requires.
fn delayed_signal_delays(
    checks: &mut [DigitalTimingCheck],
    named: &[[Option<Delayed>; 2]],
) -> BTreeMap<Delayed, i64> {
    let mut variables = BTreeMap::new();
    let mut count = 0_usize;
    let mut fresh = || {
        count += 1;
        count - 1
    };
    let slots: Vec<Option<[usize; 2]>> = checks
        .iter()
        .zip(named)
        .map(|(check, names)| {
            window(check)?;
            Some(names.clone().map(|name| match name {
                Some(net) => *variables.entry(net).or_insert_with(&mut fresh),
                None => fresh(),
            }))
        })
        .collect();
    // The sets of checks that share delayed signals, by union-find over the
    // delays.
    let mut parent: Vec<usize> = (0..count).collect();
    fn root(parent: &mut [usize], mut at: usize) -> usize {
        while parent[at] != at {
            parent[at] = parent[parent[at]];
            at = parent[at];
        }
        at
    }
    for [reference, data] in slots.iter().flatten() {
        let (reference, data) = (root(&mut parent, *reference), root(&mut parent, *data));
        parent[reference] = data;
    }
    let mut sets: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (index, slot) in slots.iter().enumerate() {
        if let Some([reference, _]) = slot {
            sets.entry(root(&mut parent, *reference))
                .or_default()
                .push(index);
        }
    }
    let mut delays = vec![0_i64; count];
    for members in sets.values() {
        // The least solution of the difference constraints, relaxed from 0;
        // one still rising after a round per delay has none.
        let solved = loop {
            let mut trial = delays.clone();
            let mut settled = false;
            for _ in 0..=count {
                let mut raised = false;
                for &index in members {
                    let (Some((before, after)), Some([r, d])) =
                        (window(&checks[index]), slots[index])
                    else {
                        continue;
                    };
                    for (later, earlier, limit) in [(r, d, before), (d, r, after)] {
                        let least = trial[earlier].saturating_sub(limit);
                        if least > trial[later] {
                            trial[later] = least;
                            raised = true;
                        }
                    }
                }
                if !raised {
                    settled = true;
                    break;
                }
            }
            if settled {
                break trial;
            }
            let Some(smallest) = members
                .iter()
                .flat_map(|&index| checks[index].limits)
                .filter(|limit| *limit < 0)
                .min()
            else {
                break trial;
            };
            for &index in members {
                let check = &mut checks[index];
                for limit in &mut check.limits {
                    if *limit == smallest {
                        *limit = 0;
                        check.limits_inconsistent = true;
                    }
                }
            }
        };
        for &index in members {
            if let Some([r, d]) = slots[index] {
                delays[r] = solved[r];
                delays[d] = solved[d];
                checks[index].delays = [solved[r], solved[d]];
            }
        }
    }
    variables
        .into_iter()
        .map(|(net, variable)| (net, delays[variable]))
        .collect()
}

/// Whether a `$timeskew` or `$fullskew` flag is set: written, and a constant
/// other than zero (IEEE 1364-2005 section 15.3 and A.7.5.2).
fn flag(
    written: Option<&Expression>,
    (resolved, time_scale): (&ResolvedConstants, crate::time_scale::ModuleTimeScale),
) -> Result<bool, DigitalLoweringDiagnostic> {
    let Some(written) = written else {
        return Ok(false);
    };
    match constants::scalar(written, resolved, time_scale) {
        Some(Integer(value)) => Ok(value != 0),
        Some(Real(value)) => Ok(value != 0.0),
        None => Err(DigitalLoweringDiagnostic::refusal(
            "a timing check flag that is not a constant expression (IEEE 1364-2005 A.7.5.2)"
                .to_string(),
            written.span().into(),
        )),
    }
}

/// The storage bits a timing check event's bit- or part-select names (IEEE
/// 1364-2005 A.7.5.3 `constant_range_expression`), or `None` for a terminal
/// that names its whole signal. The event is then a change of those bits
/// only, and an edge of the least significant of them.
///
/// `[i +: w]` names indices `i` to `i + w - 1` and `[i -: w]` indices
/// `i - w + 1` to `i` (section 5.2.1), whichever way the signal is declared;
/// `[m:l]` must run the way the declaration does.
fn selected_bits(
    terminal: &crate::ast::SpecifyTerminal,
    signal: Option<&DigitalSignal>,
    (resolved, time_scale): (&ResolvedConstants, crate::time_scale::ModuleTimeScale),
) -> Result<Option<crate::canonical_ir::digital::DigitalCheckBits>, DigitalLoweringDiagnostic> {
    use crate::ast::TerminalSelect;
    let (Some(select), Some(signal)) = (&terminal.select, signal) else {
        return Ok(None);
    };
    let refuse = |what: String| {
        DigitalLoweringDiagnostic::refusal(
            format!(
                "timing check event `{}` {what} (IEEE 1364-2005 A.7.5.3 and section 5.2.1)",
                terminal.name
            ),
            terminal.span.into(),
        )
    };
    let index = |expression: &Expression| match constants::scalar(expression, resolved, time_scale)
    {
        Some(Integer(value)) => Ok(value),
        _ => Err(refuse(
            "selects bits by an expression that is not a constant integer".to_string(),
        )),
    };
    let range = signal.declared_range();
    let (left, right, text) = match select {
        TerminalSelect::Bit { index: at, text } => {
            let at = index(at)?;
            (at, at, text)
        }
        TerminalSelect::Part {
            first,
            second,
            indexed,
            text,
        } => {
            let (base, other) = (index(first)?, index(second)?);
            match indexed {
                None => {
                    if base != other && (base > other) != (range.msb > range.lsb) {
                        return Err(refuse(format!(
                            "selects `[{text}]` against the direction of its declaration \
                             `[{}:{}]`",
                            range.msb, range.lsb
                        )));
                    }
                    (base, other, text)
                }
                Some(ascending) => {
                    if other <= 0 {
                        return Err(refuse(format!(
                            "selects `[{text}]`, whose width is not positive"
                        )));
                    }
                    let span = other - 1;
                    let far = if *ascending {
                        base.checked_add(span)
                    } else {
                        base.checked_sub(span)
                    };
                    let Some(far) = far else {
                        return Err(refuse(format!("selects `[{text}]` beyond any index")));
                    };
                    (base, far, text)
                }
            }
        }
    };
    if !range.contains(left) || !range.contains(right) {
        return Err(refuse(format!(
            "selects `[{text}]`, outside its declaration `[{}:{}]`",
            range.msb, range.lsb
        )));
    }
    let (first, second) = (range.position_of(left), range.position_of(right));
    let (low, high) = (first.min(second), first.max(second));
    Ok(Some(crate::canonical_ir::digital::DigitalCheckBits {
        low: u32::try_from(low).map_err(|_| refuse(format!("selects `[{text}]`")))?,
        width: u32::try_from(high - low + 1).map_err(|_| refuse(format!("selects `[{text}]`")))?,
    }))
}

/// A timing check limit in ticks of `precision`, signed where the check
/// takes a negative limit, and as its message writes it.
fn limit_ticks(
    value: &Expression,
    (resolved, time_scale): (&ResolvedConstants, crate::time_scale::ModuleTimeScale),
    precision: i8,
    signed: bool,
) -> Result<(i64, String), DigitalLoweringDiagnostic> {
    let refuse = |what: &str| {
        DigitalLoweringDiagnostic::refusal(
            format!("{what} (IEEE 1364-2005 section 15)"),
            value.span().into(),
        )
    };
    let (negative, converted, text) = match constants::scalar(value, resolved, time_scale) {
        Some(Integer(units)) => (
            units < 0,
            time_scale.integer_delay_ticks(units.unsigned_abs(), precision),
            units.to_string(),
        ),
        Some(Real(units)) => (
            units < 0.0,
            time_scale.delay_ticks(units.abs(), precision),
            units.to_string(),
        ),
        None => {
            return Err(refuse(
                "a timing check limit that is not a constant expression",
            ));
        }
    };
    if negative && !signed {
        return Err(refuse("a negative limit of a timing check that takes none"));
    }
    match converted {
        Ok(ticks) if ticks != crate::time_scale::DELAY_BEYOND_HORIZON => {
            Ok((if negative { -ticks } else { ticks }, text))
        }
        _ => Err(refuse(
            "a timing check limit too long for the design's time",
        )),
    }
}
