//! A frame's timing checks (IEEE 1364-2005 section 15) and the module paths
//! that delay their delayed signals (section 15.8).

use super::*;
use crate::ast::{SpecifyTimingCheck, TimingCheckEvent, TimingCheckKind};
use crate::canonical_ir::digital::{
    DigitalCheckCondition, DigitalCheckEvent, DigitalModulePath, DigitalPathDelay,
    DigitalTimingCheck, DigitalTimingCheckKind,
};
use crate::numeric_literal::NumericLiteralValue::{Integer, Real};

/// Lower `checks`, their names resolved in `scope` and their messages
/// naming `instance` (empty for the compiled module itself).
pub(super) fn lower_timing_checks(
    checks: &[SpecifyTimingCheck],
    (scope, instance): (&HashMap<&str, DigitalSignalId>, &str),
    constants: (&ResolvedConstants, crate::time_scale::ModuleTimeScale),
    precision: i8,
    (lowered, paths): (&mut Vec<DigitalTimingCheck>, &mut Vec<DigitalModulePath>),
) -> Result<(), DigitalLoweringDiagnostic> {
    for check in checks {
        let resolve = |name: &SmolStr, span: crate::source::Span| {
            scope.get(name.as_str()).copied().ok_or_else(|| {
                DigitalLoweringDiagnostic::refusal(
                    format!("timing check terminal `{name}` names no discrete signal"),
                    span.into(),
                )
            })
        };
        let event = |event: &TimingCheckEvent| -> Result<_, DigitalLoweringDiagnostic> {
            Ok(DigitalCheckEvent {
                signal: resolve(&event.terminal.name, event.terminal.span)?,
                edges: event.edges,
                condition: event
                    .condition
                    .as_ref()
                    .map(|condition| {
                        Ok::<_, DigitalLoweringDiagnostic>(DigitalCheckCondition {
                            signal: resolve(&condition.signal.name, condition.signal.span)?,
                            enabling: condition.enabling,
                            unknown_enables: condition.unknown_enables,
                        })
                    })
                    .transpose()?,
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
        let [first, second] = limits;
        // The delayed signals' delays: the reference's makes a negative
        // setup (removal) limit 0, the data's a negative hold (recovery) one.
        let (before, after) = match check.kind {
            TimingCheckKind::SetupHold => (first, second),
            TimingCheckKind::RecRem => (second, first),
            _ => (0, 0),
        };
        for (delayed, source, delay) in [
            (&check.delayed_reference, Some(&check.reference), before),
            (&check.delayed_data, check.data.as_ref(), after),
        ] {
            let (Some(delayed), Some(source), Ok(ticks)) = (delayed, source, u64::try_from(-delay))
            else {
                continue;
            };
            if ticks == 0 {
                continue;
            }
            paths.push(DigitalModulePath {
                source: resolve(&source.terminal.name, source.terminal.span)?,
                destination: resolve(&delayed.name, delayed.span)?,
                edge: None,
                condition: None,
                ifnone: false,
                delay: DigitalPathDelay {
                    transitions: [ticks; 12],
                    exponent: precision,
                },
                // A delayed signal is a copy of every change (transport).
                pulse_limits: Some([0, 0]),
                on_detect: false,
                show_cancelled: false,
                delayed_signal: true,
            });
        }
        let describe = |event: &TimingCheckEvent| {
            let mut text = match &event.edge_text {
                Some(edge) => format!("{edge} {}", event.terminal.name),
                None => event.terminal.name.to_string(),
            };
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
                TimingCheckKind::Period => DigitalTimingCheckKind::Period,
                TimingCheckKind::Width => DigitalTimingCheckKind::Width,
                TimingCheckKind::NoChange => DigitalTimingCheckKind::NoChange,
            },
            reference: event(&check.reference)?,
            data: check.data.as_ref().map(event).transpose()?,
            limits,
            exponent: precision,
            limit_zeroed,
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
    Ok(())
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
