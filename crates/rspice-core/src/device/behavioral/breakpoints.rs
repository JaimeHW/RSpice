//! Bounded event scheduling in the behavioral evaluator's resolved environment.

use super::periodicity::{affine_time_coordinate, constant_value};
use super::*;
use crate::abort_signal::AbortSignal;
use crate::numerics::integration::BreakpointManager;
use crate::resource::{ResourceKind, ResourceLimitError};
use std::collections::BTreeSet;

#[derive(Debug, Error)]
pub(crate) enum BehavioralBreakpointError {
    #[error("behavioral source-event collection was cancelled")]
    Aborted,
    #[error(transparent)]
    Resource(#[from] ResourceLimitError),
    #[error("behavioral source-event schedule is invalid: {0}")]
    Invalid(&'static str),
}

/// An affine clock, optionally reduced by floating remainder and transformed
/// again. Keeping the remainder explicit preserves negative-time semantics.
#[derive(Clone, Copy)]
struct TimeCoordinate {
    rate: Value,
    offset: Value,
    modulus: Option<Value>,
    scale: Value,
    shift: Value,
}

pub(super) fn has_exact_table_clock(input: &Expr, context: &Context<'_>) -> bool {
    TimeCoordinate::from_expr(input, context).is_some()
}

fn affine_preimage(value: Value, offset: Value, rate: Value) -> Value {
    let difference = value - offset;
    if difference.is_infinite() && value.is_finite() && offset.is_finite() {
        // Opposite finite endpoints can overflow before division even when
        // their time separation is ordinary (e.g. -1e308 to +1e308 at 1e308/s).
        ((0.5 * value - 0.5 * offset) / rate) * 2.0
    } else {
        difference / rate
    }
}

impl TimeCoordinate {
    fn from_expr(expr: &Expr, context: &Context<'_>) -> Option<Self> {
        if let Some((rate, offset)) = affine_time_coordinate(expr, context) {
            return Some(Self {
                rate,
                offset,
                modulus: None,
                scale: 1.0,
                shift: 0.0,
            });
        }
        let transformed = |input: &Expr, scale: Value, shift: Value| {
            let mut coordinate = Self::from_expr(input, context)?;
            coordinate.scale *= scale;
            coordinate.shift = coordinate.shift * scale + shift;
            (coordinate.scale.is_finite() && coordinate.shift.is_finite()).then_some(coordinate)
        };
        let modulo = |input: &Expr, divisor: &Expr| {
            let mut coordinate = Self::from_expr(input, context)?;
            let modulus = constant_value(divisor, context)?.abs();
            if coordinate.modulus.is_some() || modulus == 0.0 {
                return None;
            }
            coordinate.offset = coordinate.offset * coordinate.scale + coordinate.shift;
            coordinate.rate *= coordinate.scale;
            coordinate.modulus = Some(modulus);
            coordinate.scale = 1.0;
            coordinate.shift = 0.0;
            Some(coordinate)
        };
        match expr {
            Expr::Unary {
                op: UnaryOp::Neg,
                operand,
            } => transformed(operand, -1.0, 0.0),
            Expr::Binary {
                op: BinaryOp::Mod,
                left,
                right,
            } => modulo(left, right),
            Expr::Function {
                func: Function::Mod,
                args,
            } if args.len() == 2 => modulo(&args[0], &args[1]),
            Expr::Binary { op, left, right } => {
                if let Some(value) = constant_value(right, context) {
                    match op {
                        BinaryOp::Add => transformed(left, 1.0, value),
                        BinaryOp::Sub => transformed(left, 1.0, -value),
                        BinaryOp::Mul => transformed(left, value, 0.0),
                        BinaryOp::Div if value != 0.0 => transformed(left, 1.0 / value, 0.0),
                        _ => None,
                    }
                } else if let Some(value) = constant_value(left, context) {
                    match op {
                        BinaryOp::Add => transformed(right, 1.0, value),
                        BinaryOp::Sub => transformed(right, -1.0, value),
                        BinaryOp::Mul => transformed(right, value, 0.0),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn visit(
        &self,
        target: Value,
        schedule: &mut EventSchedule<'_>,
    ) -> Result<(), BehavioralBreakpointError> {
        if self.rate == 0.0 || self.scale == 0.0 {
            return Ok(());
        }
        let target = affine_preimage(target, self.shift, self.scale);
        let Some(modulus) = self.modulus else {
            let end = self.rate.mul_add(schedule.tstop, self.offset);
            if target < self.offset.min(end) || target > self.offset.max(end) {
                return Ok(());
            }
            return schedule.add(affine_preimage(target, self.offset, self.rate));
        };
        // Floating remainder has the dividend's sign and never reaches either
        // modulus endpoint. Both one-sided endpoints are covered by resets.
        if target <= -modulus || target >= modulus {
            return Ok(());
        }
        let end = self.offset + self.rate * schedule.tstop;
        let mut first = ((self.offset.min(end) - target) / modulus).ceil() - 1.0;
        let mut last = ((self.offset.max(end) - target) / modulus).floor() + 1.0;
        if target > 0.0 {
            first = first.max(0.0);
        }
        if target < 0.0 {
            last = last.min(0.0);
        }
        if !first.is_finite()
            || !last.is_finite()
            || first.abs().max(last.abs()) >= (1_u64 << 53) as Value
        {
            return Err(BehavioralBreakpointError::Invalid(
                "modulo cycle indices are not representable",
            ));
        }
        let count = (last - first + 1.0).max(0.0);
        if count > 1_000_004.0 {
            return Err(BehavioralBreakpointError::Invalid(
                "modulo schedule exceeds the exact enumeration limit of 1000000 cycles",
            ));
        }
        for index in 0..count as usize {
            schedule.poll()?;
            let coordinate = modulus.mul_add(first + index as Value, target);
            schedule.add(affine_preimage(coordinate, self.offset, self.rate))?;
        }
        Ok(())
    }
}

struct EventSchedule<'a> {
    tstop: Value,
    breakpoints: &'a mut BreakpointManager,
    abort: &'a dyn AbortSignal,
    max_points: usize,
    physical_corners: bool,
    // Nonnegative finite event times have the same bit and numeric ordering.
    // One ordered union avoids quadratic Vec insertion for interleaved clocks.
    events: BTreeSet<u64>,
}

impl EventSchedule<'_> {
    fn poll(&self) -> Result<(), BehavioralBreakpointError> {
        if self.abort.is_aborted() {
            Err(BehavioralBreakpointError::Aborted)
        } else {
            Ok(())
        }
    }

    fn add(&mut self, time: Value) -> Result<(), BehavioralBreakpointError> {
        self.poll()?;
        if !time.is_finite() {
            return Err(BehavioralBreakpointError::Invalid(
                "an event time is not finite",
            ));
        }
        if (0.0..=self.tstop).contains(&time) {
            self.events
                .insert(if time == 0.0 { 0 } else { time.to_bits() });
            ResourceLimitError::ensure(
                ResourceKind::AnalysisPoints,
                self.events.len(),
                self.max_points,
            )?;
        }
        Ok(())
    }

    fn repeat(
        &mut self,
        first: Value,
        period: Option<Value>,
    ) -> Result<(), BehavioralBreakpointError> {
        let Some(period) = period else {
            return self.add(first);
        };
        let first = if first < 0.0 {
            first.rem_euclid(period)
        } else {
            first
        };
        // Refuse obviously excessive schedules before retaining any events;
        // the exact loop below remains authoritative at the count boundary.
        if (self.tstop - first) / period > 1_000_001.0 {
            return Err(BehavioralBreakpointError::Invalid(
                "pulse schedule exceeds the exact enumeration limit of 1000000 cycles",
            ));
        }
        let mut previous = None;
        for cycle in 0..=1_000_000 {
            self.poll()?;
            let time = period.mul_add(cycle as Value, first);
            if time > self.tstop {
                return Ok(());
            }
            if cycle == 1_000_000 {
                return Err(BehavioralBreakpointError::Invalid(
                    "pulse schedule exceeds the exact enumeration limit of 1000000 cycles",
                ));
            }
            if previous.is_some_and(|previous| time <= previous) {
                return Err(BehavioralBreakpointError::Invalid(
                    "pulse period cannot advance the event clock",
                ));
            }
            self.add(time)?;
            previous = Some(time);
        }
        unreachable!("the final cycle returns an enumeration error")
    }

    fn table(
        &mut self,
        input: &Expr,
        points: impl IntoIterator<Item = (Value, Value)>,
        context: &Context<'_>,
    ) -> Result<(), BehavioralBreakpointError> {
        let Some(coordinate) = TimeCoordinate::from_expr(input, context) else {
            return Ok(());
        };
        for (knot, _) in crate::numerics::pwl_event_points(points, !self.physical_corners) {
            self.poll()?;
            coordinate.visit(knot, self)?;
        }
        if coordinate.modulus.is_some() {
            // The remainder resets even when no table knot lies at zero.
            coordinate.visit(coordinate.shift, self)?;
        }
        Ok(())
    }

    fn expression(
        &mut self,
        expr: &Expr,
        context: &Context<'_>,
    ) -> Result<(), BehavioralBreakpointError> {
        self.poll()?;
        match expr {
            Expr::Function { func, args } => {
                match func {
                    Function::Table | Function::Pwl if !args.is_empty() => {
                        let points = args[1..].chunks_exact(2).filter_map(|pair| {
                            Some((
                                constant_value(&pair[0], context)?,
                                constant_value(&pair[1], context).unwrap_or(Value::NAN),
                            ))
                        });
                        self.table(&args[0], points, context)?;
                    }
                    Function::SpicePulse | Function::SpiceSin | Function::SpiceExp => {
                        let values = args
                            .iter()
                            .map(|arg| constant_value(arg, context))
                            .collect::<Option<Vec<_>>>();
                        if let Some(values) = values {
                            match func {
                                Function::SpicePulse if (1..=7).contains(&values.len()) => {
                                    let [v1, v2, delay, rise, fall, width, period] =
                                        crate::expr::spice_waveform_parameters(&values);
                                    if v1 != v2 {
                                        let period = (period > 0.0).then_some(period);
                                        for offset in [0.0, rise, rise + width, rise + width + fall]
                                        {
                                            if offset >= 0.0
                                                && period.is_none_or(|period| offset < period)
                                            {
                                                self.repeat(delay + offset, period)?;
                                            }
                                        }
                                    }
                                }
                                Function::SpiceSin if (3..=6).contains(&values.len()) => {
                                    let [_, _, _, delay, _, _] =
                                        crate::expr::spice_waveform_parameters(&values);
                                    self.add(delay)?;
                                }
                                Function::SpiceExp if (2..=6).contains(&values.len()) => {
                                    let [_, _, delay1, _, delay2, _] =
                                        crate::expr::spice_exp_parameters(&values);
                                    self.add(delay1)?;
                                    self.add(delay2)?;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
                for arg in args {
                    self.expression(arg, context)?;
                }
            }
            Expr::Unary { operand, .. } => self.expression(operand, context)?,
            Expr::Binary { left, right, .. } => {
                self.expression(left, context)?;
                self.expression(right, context)?;
            }
            Expr::LookupTable { input, table } => {
                if table.transient_breakpoints {
                    self.table(input, table.points.iter().copied(), context)?;
                }
                self.expression(input, context)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl BehavioralSources {
    pub(crate) fn collect_transient_breakpoints(
        &self,
        tstop: Value,
        breakpoints: &mut BreakpointManager,
        abort: &dyn AbortSignal,
        max_points: usize,
        physical_corners: bool,
    ) -> Result<(), BehavioralBreakpointError> {
        if abort.is_aborted() {
            return Err(BehavioralBreakpointError::Aborted);
        }
        if self.voltage_sources.is_empty() && self.current_sources.is_empty() {
            return Ok(());
        }
        let mut events = BTreeSet::new();
        for (index, &time) in breakpoints.times().iter().enumerate() {
            if index.is_multiple_of(256) && abort.is_aborted() {
                return Err(BehavioralBreakpointError::Aborted);
            }
            events.insert(if time == 0.0 { 0 } else { time.to_bits() });
        }
        let mut schedule = EventSchedule {
            tstop,
            events,
            breakpoints,
            abort,
            max_points,
            physical_corners,
        };
        schedule.poll()?;
        for source in &self.voltage_sources {
            schedule.expression(&source.ast, &source.periodicity_context())?;
        }
        for source in &self.current_sources {
            schedule.expression(&source.ast, &source.periodicity_context())?;
        }
        schedule.poll()?;
        schedule
            .breakpoints
            .extend(schedule.events.into_iter().map(Value::from_bits));
        Ok(())
    }
}

#[cfg(test)]
pub(super) fn expression_transient_breakpoints(expr: &Expr, tstop: Value) -> Vec<Value> {
    let mut breakpoints = BreakpointManager::new_with_tolerance(Value::from_bits(1));
    let mut schedule = EventSchedule {
        tstop,
        breakpoints: &mut breakpoints,
        abort: &crate::abort_signal::NoAbort,
        max_points: usize::MAX,
        physical_corners: false,
        events: BTreeSet::new(),
    };
    schedule
        .expression(expr, &Context::transient(&[], &[], 0.0))
        .unwrap();
    schedule.events.into_iter().map(Value::from_bits).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, NoAbort};

    fn sources(expression: &str) -> BehavioralSources {
        let mut sources = BehavioralSources::new();
        sources
            .voltage_sources
            .push(BehavioralVoltageSource::new("B1".to_owned(), 1, 0, 1, expression).unwrap());
        sources
    }

    fn collect(
        sources: &BehavioralSources,
        stop: Value,
        limit: usize,
        physical: bool,
    ) -> Result<Vec<Value>, BehavioralBreakpointError> {
        let mut manager = BreakpointManager::new_with_tolerance(Value::from_bits(1));
        sources.collect_transient_breakpoints(stop, &mut manager, &NoAbort, limit, physical)?;
        Ok(manager.times().to_vec())
    }

    fn contains(times: &[Value], target: Value) -> bool {
        times
            .iter()
            .any(|&time| (time - target).abs() <= 16.0 * Value::EPSILON * target.abs())
    }

    #[test]
    fn affine_modulo_forms_schedule_the_same_small_scale_knots() {
        for scale in [1e-6, 1e-18, 1e-30] {
            for coordinate in [
                format!("time%{scale:e}"),
                format!("mod(time,{scale:e})"),
                format!("mod(time/{scale:e},1)*{scale:e}"),
            ] {
                let expression = format!("table({coordinate},0,0,{:e},1,{scale:e},0)", scale * 0.3);
                let events = collect(&sources(&expression), 2.5 * scale, 16, false).unwrap();
                for phase in [0.0, 0.3, 1.0, 1.3, 2.0, 2.3] {
                    assert!(
                        contains(&events, phase * scale),
                        "{expression}: {events:?}, phase={phase}"
                    );
                }
                assert_eq!(events.len(), 6, "{expression}: {events:?}");
            }
        }
    }

    #[test]
    fn affine_events_ignore_distant_knots_without_overflowing_finite_preimages() {
        for expression in [
            "table(time*1e-300,0,0,1e300,1)",
            "table(-time*1e-300,-1e300,1,0,0)",
        ] {
            assert_eq!(collect(&sources(expression), 1.0, 2, false).unwrap(), [0.0]);
        }
        let events = collect(
            &sources("table(1e308*(time-1),-1e308,0,0,1,1e308,0)"),
            2.0,
            3,
            false,
        )
        .unwrap();
        assert_eq!(events, [0.0, 1.0, 2.0]);
    }

    #[test]
    fn modulo_events_follow_signed_remainder_on_both_sides_of_zero() {
        let events = collect(
            &sources("table((time-40n)%120n,-120n,0,-20n,1,0,0,60n,1,120n,0)"),
            300e-9,
            20,
            false,
        )
        .unwrap();
        for target in [20e-9, 40e-9, 100e-9, 160e-9, 220e-9, 280e-9] {
            assert!(contains(&events, target), "missing {target:e}: {events:?}");
        }
        assert!(
            !contains(&events, 140e-9),
            "a positive dividend cannot have negative remainder"
        );
    }

    #[test]
    fn scaled_modulo_periodicity_requires_a_clock_that_keeps_its_sign() {
        for (coordinate, periodic) in [
            ("mod(time*1e12,1e6)", true),
            ("(-time*1e12)%-1e6", true),
            ("mod(time+0.25u,1u)", true),
            ("mod(-time-0.25u,1u)", true),
            ("mod(time-0.25u,1u)", false),
            ("mod(-time+0.25u,1u)", false),
        ] {
            let source = sources(coordinate);
            assert_eq!(
                source.voltage_sources[0].has_periodic_time_dependence(1e-6, false),
                periodic,
                "{coordinate}"
            );
        }
    }

    #[test]
    fn source_events_use_resolved_temperature_and_implicit_waveform_defaults() {
        let mut thermal = sources("table(time%120n,0,0,temper*1n,1,120n,0)");
        thermal.voltage_sources[0].set_temperature(60.0);
        let events = collect(&thermal, 200e-9, 16, false).unwrap();
        assert!(
            contains(&events, 60e-9) && contains(&events, 180e-9),
            "{events:?}"
        );
        for (expression, expected) in [
            ("spice_pulse(0,1,0,1n,2n)", vec![0.0, 1e-9, 3e-9]),
            ("spice_pulse(0,1,0,-1n,1n,3n)", vec![0.0, 2e-9, 3e-9]),
            ("spice_exp(0,1,2n,1n)", vec![2e-9]),
        ] {
            let events = collect(&sources(expression), 10e-9, 16, false).unwrap();
            assert_eq!(events.len(), expected.len(), "{expression}: {events:?}");
            assert!(
                expected.iter().all(|&time| contains(&events, time)),
                "{expression}: {events:?}"
            );
        }
    }

    #[test]
    fn only_physical_corner_mode_discards_redundant_flat_knots() {
        let source = sources("table(time,0,0,1e-300,0,0.25,0,0.5,1,1,0)");
        let authored = collect(&source, 1.0, 5, false).unwrap();
        let physical = collect(&source, 1.0, 4, true).unwrap();
        assert_eq!(authored, vec![0.0, 1e-300, 0.25, 0.5, 1.0]);
        assert_eq!(physical, vec![0.0, 0.25, 0.5, 1.0]);
    }

    #[test]
    fn event_schedules_enforce_union_limits_cancellation_and_enumeration_bounds() {
        let mut repeated = sources("table(time%1,0,0,0.5,1,1,0)");
        repeated
            .voltage_sources
            .push(repeated.voltage_sources[0].clone());
        assert_eq!(
            collect(&repeated, 2.0, 5, false).unwrap(),
            vec![0.0, 0.5, 1.0, 1.5, 2.0]
        );
        assert!(matches!(
            collect(&repeated, 2.0, 4, false),
            Err(BehavioralBreakpointError::Resource(_))
        ));
        let mut manager = BreakpointManager::new();
        assert!(matches!(
            repeated.collect_transient_breakpoints(
                2.0,
                &mut manager,
                &CountingAbort::new(0),
                5,
                false
            ),
            Err(BehavioralBreakpointError::Aborted)
        ));
        assert!(manager.times().is_empty());
        for expression in [
            "table(time%1p,0,0,0.5p,1,1p,0)",
            "spice_pulse(0,1,0,1p,1p,1p,4p)",
        ] {
            assert!(
                matches!(
                    collect(&sources(expression), 1.0, usize::MAX, false),
                    Err(BehavioralBreakpointError::Invalid(_))
                ),
                "{expression}"
            );
        }
    }
}
