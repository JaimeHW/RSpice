//! Bounded event scheduling in the behavioral evaluator's resolved environment.

use super::periodicity::{affine_time_coordinate, needs_time_features};
use super::*;
use crate::abort_signal::AbortSignal;
use crate::expr::constant_value;
use crate::numerics::integration::BreakpointManager;
use crate::resource::{ResourceKind, ResourceLimitError};
use std::cell::Cell;
use std::collections::BTreeSet;

mod features;

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
        schedule.periodic_coordinate(self.rate, self.offset, target, modulus, true, 0.0)
    }
}

struct EventSchedule<'a> {
    tstop: Value,
    abort: &'a dyn AbortSignal,
    max_points: usize,
    physical_corners: bool,
    // All candidate crossings of one n-ary extremum share an instruction
    // budget, including recursive level isolation and temporary schedules.
    isolation_work: Option<&'a Cell<usize>>,
    // Nonnegative finite event times have the same bit and numeric ordering.
    // One ordered union avoids quadratic Vec insertion for interleaved clocks.
    events: BTreeSet<u64>,
}

impl EventSchedule<'_> {
    fn periodic_coordinate(
        &mut self,
        rate: Value,
        offset: Value,
        target: Value,
        modulus: Value,
        signed_remainder: bool,
        time_start: Value,
    ) -> Result<(), BehavioralBreakpointError> {
        if rate == 0.0 || time_start > self.tstop {
            return Ok(());
        }
        let end = rate.mul_add(self.tstop, offset);
        let start = rate.mul_add(time_start, offset);
        let mut first = ((start.min(end) - target) / modulus).ceil() - 1.0;
        let mut last = ((start.max(end) - target) / modulus).floor() + 1.0;
        if signed_remainder && target > 0.0 {
            first = first.max(0.0);
        }
        if signed_remainder && target < 0.0 {
            last = last.min(0.0);
        }
        if !first.is_finite()
            || !last.is_finite()
            || first.abs().max(last.abs()) >= (1_u64 << 53) as Value
        {
            return Err(BehavioralBreakpointError::Invalid(
                "periodic clock cycle indices are not representable",
            ));
        }
        let count = (last - first + 1.0).max(0.0);
        if count > 1_000_004.0 {
            return Err(BehavioralBreakpointError::Invalid(
                "periodic clock schedule exceeds the exact enumeration limit of 1000000 cycles",
            ));
        }
        for index in 0..count as usize {
            self.poll()?;
            let coordinate = modulus.mul_add(first + index as Value, target);
            if coordinate < start.min(end) || coordinate > start.max(end) {
                continue;
            }
            self.add(affine_preimage(coordinate, offset, rate))?;
        }
        Ok(())
    }
    fn poll(&self) -> Result<(), BehavioralBreakpointError> {
        if self.abort.is_aborted() {
            Err(BehavioralBreakpointError::Aborted)
        } else {
            Ok(())
        }
    }

    fn charge_isolation_work(&self, cost: usize) -> Result<(), BehavioralBreakpointError> {
        if let Some(work) = self.isolation_work {
            let used = work.get().saturating_add(cost);
            work.set(used);
            if used > 16_000_000 {
                return Err(BehavioralBreakpointError::Invalid(
                    "extrema branch isolation exceeds its 16000000-instruction work limit",
                ));
            }
        }
        Ok(())
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
        features: bool,
    ) -> Result<(), BehavioralBreakpointError> {
        self.poll()?;
        if features {
            self.temporal_features(expr, context)?;
        }
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
                    self.expression(arg, context, features)?;
                }
            }
            Expr::Unary { operand, .. } => self.expression(operand, context, features)?,
            Expr::Binary { left, right, .. } => {
                self.expression(left, context, features)?;
                self.expression(right, context, features)?;
            }
            Expr::LookupTable { input, table } => {
                if table.transient_breakpoints {
                    self.table(input, table.points.iter().copied(), context)?;
                }
                self.expression(input, context, features)?;
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
            abort,
            max_points,
            physical_corners,
            isolation_work: None,
        };
        schedule.poll()?;
        for source in &self.voltage_sources {
            let context = source.periodicity_context();
            schedule.source_expression(&source.ast, &context)?;
        }
        for source in &self.current_sources {
            let context = source.periodicity_context();
            schedule.source_expression(&source.ast, &context)?;
        }
        schedule.poll()?;
        breakpoints.extend(schedule.events.into_iter().map(Value::from_bits));
        Ok(())
    }
}

#[cfg(test)]
pub(super) fn expression_transient_breakpoints(expr: &Expr, tstop: Value) -> Vec<Value> {
    let mut schedule = EventSchedule {
        tstop,
        abort: &crate::abort_signal::NoAbort,
        max_points: usize::MAX,
        physical_corners: false,
        isolation_work: None,
        events: BTreeSet::new(),
    };
    schedule
        .expression(expr, &Context::transient(&[], &[], 0.0), false)
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
    fn source_features_find_extrema_at_extreme_clock_scales() {
        for period in [1e-30, 1.0, 1e308] {
            for direction in [-1.0, 1.0] {
                let rate = direction * std::f64::consts::TAU / period;
                let expression = format!("exp(-10000*(1-cos({rate:e}*time+0.1)))");
                let source = sources(&expression);
                let events = collect(&source, period, 16, true).unwrap();
                assert_eq!(events, collect(&source, period, 16, false).unwrap());
                for cycle in -2..=2 {
                    let peak = (f64::from(cycle) * std::f64::consts::TAU - 0.1) / rate;
                    if (0.0..=period).contains(&peak) {
                        assert!(
                            contains(&events, peak),
                            "{expression}: missing {peak:e}: {events:?}"
                        );
                    }
                }
                assert!(events.len() < 10);
            }
        }
    }

    #[test]
    fn logarithm_floor_corners_and_hidden_input_jumps_share_transient_features() {
        for stop in [1e-30, 1.0, 1e300] {
            for function in ["ln", "log10", "log"] {
                let source = sources(&format!("{function}(1e-38*(4*(time/{stop:e})-2))"));
                let events = collect(&source, stop, 16, true).unwrap();
                assert!(contains(&events, 0.75 * stop), "{function}: {events:?}");
                assert_eq!(events, collect(&source, stop, 16, false).unwrap());

                let plateau = sources(&format!("{function}(-2+pwrs(time/{stop:e}-0.5,0))"));
                assert!(collect(&plateau, stop, 16, true).unwrap().is_empty());
                assert!(collect(&plateau, stop, 16, false).unwrap().is_empty());
            }
        }
    }

    #[test]
    fn square_root_floor_and_absolute_corners_preserve_shared_source_events() {
        for stop in [1e-30, 1.0, 1e300] {
            for function in ["sqrt", "abs"] {
                let source = sources(&format!("{function}(time/{stop:e}-0.3)"));
                let events = collect(&source, stop, 16, true).unwrap();
                assert!(contains(&events, 0.3 * stop), "{function}: {events:?}");
                assert_eq!(events, collect(&source, stop, 16, false).unwrap());
            }
            let plateau = sources(&format!("sqrt(-2+pwrs(time/{stop:e}-0.5,0))"));
            assert!(collect(&plateau, stop, 16, true).unwrap().is_empty());
            assert!(collect(&plateau, stop, 16, false).unwrap().is_empty());
        }
    }

    #[test]
    fn nary_extrema_find_active_corners_and_remove_clamped_intersections() {
        for stop in [1e-30, 1.0, 1e300] {
            for (function, plateau) in [("min", 0.3), ("max", 0.7)] {
                let source = sources(&format!(
                    "{function}(time/{stop:e},1-time/{stop:e},{plateau})"
                ));
                let events = collect(&source, stop, 16, true).unwrap();
                assert!(contains(&events, 0.3 * stop), "{function}: {events:?}");
                assert!(contains(&events, 0.7 * stop), "{function}: {events:?}");
                assert!(
                    !contains(&events, 0.5 * stop),
                    "the clamped intersection is inactive: {events:?}"
                );
                assert_eq!(events, collect(&source, stop, 16, false).unwrap());
            }
        }
    }

    #[test]
    fn extrema_cusp_levels_are_found_between_representable_timestamps() {
        for stop in [1e-30, 1.0, 1e300] {
            for function in ["min", "max"] {
                let phase = format!("2*pi*(time/{stop:e})+0.1");
                let source = sources(&format!(
                    "exp(-10000*{function}(cos({phase}),-cos({phase}))^2)"
                ));
                let events = collect(&source, stop, 64, true).unwrap();
                for cycle in [0.25, 0.75] {
                    let peak = (cycle - 0.1 / std::f64::consts::TAU) * stop;
                    assert!(contains(&events, peak), "{function}: {events:?}");
                }
                assert_eq!(events, collect(&source, stop, 64, false).unwrap());
            }
        }
    }

    #[test]
    fn extrema_crossings_do_not_overflow_finite_opposing_operands() {
        for stop in [1e-30, 1.0, 1e300] {
            for function in ["min", "max"] {
                let operand = format!("1e308*(2*(time/{stop:e})-1)");
                let source = sources(&format!("{function}({operand},-({operand}))"));
                let events = collect(&source, stop, 32, true).unwrap();
                assert!(contains(&events, 0.5 * stop), "{function}: {events:?}");
                assert_eq!(events, collect(&source, stop, 32, false).unwrap());
            }
        }
    }

    #[test]
    fn extrema_crossings_share_work_limits_and_preserve_failed_collection() {
        let work = Cell::new(16_000_000 - 1024);
        let mut schedule = EventSchedule {
            tstop: 1.0,
            abort: &NoAbort,
            max_points: 1024,
            physical_corners: true,
            isolation_work: Some(&work),
            events: BTreeSet::new(),
        };
        let expr = crate::expr::parse_expression_strict(
            "max(sin(8*pi*time),cos(6*pi*time),0.5*sin(2*pi*time))",
        )
        .unwrap();
        let error = schedule
            .temporal_features(&expr, &Context::transient(&[], &[], 0.0))
            .unwrap_err();
        assert!(
            error.to_string().contains("extrema branch isolation"),
            "{error}"
        );
        assert!(schedule.events.is_empty());

        // Operand-level candidates must share the same aggregate budget,
        // including analytically invertible clocks that need no bisection.
        work.set(16_000_000 - 8);
        let expr = crate::expr::parse_expression_strict(
            "sqr(max(sin(8*pi*time),cos(6*pi*time),sin(2*pi*time)))",
        )
        .unwrap();
        let error = schedule
            .temporal_features(&expr, &Context::transient(&[], &[], 0.0))
            .unwrap_err();
        assert!(
            error.to_string().contains("extrema branch isolation"),
            "{error}"
        );
        assert!(schedule.events.is_empty());

        let args = (0..1500)
            .map(|index| format!("sin(time+{index})"))
            .collect::<Vec<_>>()
            .join(",");
        let source = sources(&format!("max({args})"));
        let mut manager = BreakpointManager::new();
        manager.add(0.173);
        let error = source
            .collect_transient_breakpoints(1.0, &mut manager, &NoAbort, 1024, true)
            .unwrap_err();
        assert!(error.to_string().contains("1000000-pair"), "{error}");
        assert_eq!(manager.times(), &[0.173]);

        let repeated = std::iter::repeat_n("time", 1500)
            .collect::<Vec<_>>()
            .join(",");
        assert!(
            collect(&sources(&format!("max({repeated})")), 1.0, 16, true)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn pss_switching_events_bracket_the_actual_vm_boundary() {
        for (expression, transitions) in [
            ("if(cos(6*pi*time+0.1)>0.9999,1,0)", 6),
            ("if(spice_sin(0,1,3,0,0,90)>0.9999,1,0)", 6),
            ("if(spice_sin(0,1,3,0.25,0,90)>0.9999,1,0)", 5),
            ("eq0(cos(6*pi*time+0.1)-0.4)", 12),
            ("if(2*cos(6*pi*time+0.1)+1>2.9998,1,0)", 6),
            ("if(cos(-6*pi*time+0.1)^2>0.9999,1,0)", 12),
        ] {
            let mut source = sources(expression);
            let events = collect(&source, 1.0, 128, true).unwrap();
            let actual = events
                .windows(2)
                .filter(|pair| {
                    pair[0].next_up() == pair[1]
                        && source.voltage_sources[0].evaluate(&[], pair[0]).unwrap()
                            != source.voltage_sources[0].evaluate(&[], pair[1]).unwrap()
                })
                .count();
            assert_eq!(actual, transitions, "{expression}: {events:?}");
        }
    }

    #[test]
    fn pss_feature_enumeration_obeys_cancellation_and_resource_bounds() {
        let source = sources("exp(-10000*(1-cos(6*pi*time+0.1)))");
        assert!(matches!(
            collect(&source, 1.0, 5, true),
            Err(BehavioralBreakpointError::Resource(_))
        ));
        let mut manager = BreakpointManager::new();
        assert!(matches!(
            source.collect_transient_breakpoints(
                1.0,
                &mut manager,
                &CountingAbort::new(20),
                128,
                true,
            ),
            Err(BehavioralBreakpointError::Aborted)
        ));
        assert!(manager.times().is_empty());
        assert!(matches!(
            collect(&sources("exp(cos(1e12*time))"), 1.0, usize::MAX, true),
            Err(BehavioralBreakpointError::Invalid(_))
        ));
    }

    #[test]
    fn combined_clock_zeros_and_switching_levels_are_isolated() {
        for scale in [1e-30, 1.0, 1e300] {
            let phase = format!("6*pi*(time/{scale:e})+0.1");
            let coordinate = format!("cos({phase})+0.5*cos(2*({phase}))-0.25");
            let source = sources(&format!("exp(-1000000*({coordinate})^2)"));
            let events = collect(&source, scale, 256, true).unwrap();
            for cycle in 0..=3 {
                for angle in [-std::f64::consts::FRAC_PI_3, std::f64::consts::FRAC_PI_3] {
                    let time = ((std::f64::consts::TAU * cycle as Value + angle - 0.1)
                        / (6.0 * std::f64::consts::PI))
                        * scale;
                    if time >= 0.0 && time <= scale {
                        assert!(contains(&events, time), "missing {time:e}: {events:?}");
                    }
                }
            }
            let mut switching = sources(&format!("abs({coordinate})<0.001"));
            let events = collect(&switching, scale, 256, true).unwrap();
            let changes = events
                .windows(2)
                .filter(|pair| {
                    pair[0].next_up() == pair[1]
                        && switching.voltage_sources[0].evaluate(&[], pair[0]).unwrap()
                            != switching.voltage_sources[0].evaluate(&[], pair[1]).unwrap()
                })
                .count();
            assert_eq!(changes, 12, "scale={scale:e}: {events:?}");
        }
    }

    #[test]
    fn regular_quotient_domains_are_subdivided_before_isolating_features() {
        for scale in [1e-30, 1.0, 1e300] {
            let phase = format!("6*pi*(time/{scale:e})+0.1");
            for (denominator, gain) in [
                (format!("sqr(sin({phase}))+sqr(cos({phase}))"), 1.0),
                (format!("sin({phase})^2+cos({phase})^2"), 1.0),
                (format!("sin({phase})^2+cos({phase})^2"), 1e-310),
                (format!("pow(sin({phase}),2)+pow(cos({phase}),2)"), 1e300),
            ] {
                let coordinate = format!(
                    "({gain:e}*(cos({phase})+0.5*cos(2*({phase}))))/({gain:e}*({denominator}))-0.25"
                );
                let events = collect(
                    &sources(&format!("exp(-1000000*({coordinate})^2)")),
                    scale,
                    512,
                    true,
                )
                .unwrap();
                for cycle in 0..=3 {
                    for angle in [-std::f64::consts::FRAC_PI_3, std::f64::consts::FRAC_PI_3] {
                        let time = (std::f64::consts::TAU * cycle as Value + angle - 0.1)
                            / (6.0 * std::f64::consts::PI)
                            * scale;
                        if (0.0..=scale).contains(&time) {
                            // Subnormal operand multiplication quantizes the
                            // VM coordinate before division. Bound that input
                            // quantization separately from clock rounding.
                            let quantization = if gain < Value::MIN_POSITIVE {
                                (gain.next_up() - gain) / gain * scale
                            } else {
                                0.0
                            };
                            assert!(
                                events.iter().any(|event| (event - time).abs()
                                    <= quantization + 16.0 * Value::EPSILON * time.abs()),
                                "gain={gain:e}: missing quotient root {time:e}: {events:?}"
                            );
                        }
                    }
                }
                let mut switching = sources(&format!("abs({coordinate})<0.001"));
                let events = collect(&switching, scale, 512, true).unwrap();
                let transitions = events
                    .windows(2)
                    .filter(|pair| {
                        pair[0].next_up() == pair[1]
                            && switching.voltage_sources[0].evaluate(&[], pair[0]).unwrap()
                                != switching.voltage_sources[0].evaluate(&[], pair[1]).unwrap()
                    })
                    .count();
                assert_eq!(transitions, 12, "scale={scale:e}: {events:?}");

                let events = collect(
                    &sources(&format!(
                        "cos(({gain:e}*({phase}))/({gain:e}*({denominator})))"
                    )),
                    scale,
                    512,
                    true,
                )
                .unwrap();
                for index in 1..=6 {
                    let time = (index as Value * std::f64::consts::PI - 0.1)
                        / (6.0 * std::f64::consts::PI)
                        * scale;
                    assert!(
                        events
                            .iter()
                            .any(|event| ((event - time) / scale).abs() < 1e-6),
                        "missing quotient phase extremum {time:e}"
                    );
                }
            }
        }
    }

    #[test]
    fn power_branch_transitions_and_varying_exponents_supply_features() {
        for expression in [
            "pow(time,2)>0.25",
            "pwr(time-0.5,0)+0.1*sin(time)",
            "pwrs(time-0.5,0)+0.1*sin(time)",
        ] {
            let mut source = sources(expression);
            let events = collect(&source, 1.0, 128, true)
                .unwrap_or_else(|error| panic!("{expression}: {error}"));
            assert!(
                events.windows(2).any(|pair| pair[0].next_up() == pair[1]
                    && source.voltage_sources[0].evaluate(&[], pair[0]).unwrap()
                        != source.voltage_sources[0].evaluate(&[], pair[1]).unwrap()),
                "{expression}: {events:?}"
            );
            assert!(contains(&events, 0.5), "{expression}: {events:?}");
        }
        for expression in ["2^(sin(6*pi*time)+1)-2", "pow(cos(6*pi*time),2)-0.25"] {
            // A sharp envelope requires geometry even when the inner
            // coordinate itself has a known finite harmonic band.
            let events = collect(
                &sources(&format!("exp(-10000*({expression})^2)")),
                1.0,
                256,
                true,
            )
            .unwrap();
            for index in 1..6 {
                let time = if expression.starts_with('2') {
                    index as Value / 6.0
                } else {
                    (index as Value / 2.0 + 1.0 / 6.0) / 3.0
                };
                assert!(
                    contains(&events, time),
                    "{expression}: missing {time:e}: {events:?}"
                );
            }
        }
    }

    #[test]
    fn source_plateaus_discard_inactive_geometry_and_preserve_gaussian_peaks() {
        for gain in [1.0_f64, 1e-310] {
            let phase = "2*pi*64meg*time+0.1";
            let expression = format!(
                "exp(-1000000*(({gain:e}*(cos({phase})+0.5*cos(2*({phase}))))/({gain:e}*(sin({phase})^2+cos({phase})^2))-0.25)^2)"
            );
            let source = sources(&expression);
            let events = collect(&source, 1e-6, 10000, true).unwrap();
            let rate = std::f64::consts::TAU * 64e6;
            // The numerator-zero and peak phases are the closest distinct
            // structural features. Rounded subnormal numerator zeros must
            // not insert neighboring clocks where the final exp is zero.
            let separation =
                ((0.5 * (3.0_f64.sqrt() - 1.0)).acos() - std::f64::consts::FRAC_PI_3) / rate;
            assert!(
                events
                    .windows(2)
                    .all(|pair| pair[1] - pair[0] > 0.5 * separation)
            );
            for cycle in 0..64 {
                for phase in [
                    std::f64::consts::FRAC_PI_3,
                    5.0 * std::f64::consts::FRAC_PI_3,
                ] {
                    let peak = (std::f64::consts::TAU * cycle as Value + phase - 0.1) / rate;
                    let quantization = (gain.next_up() - gain) / gain / rate;
                    assert!(
                        events.iter().any(|&time| (time - peak).abs()
                            <= 16.0 * Value::EPSILON * 1e-6 + quantization),
                        "gain={gain:e}, missing peak={peak:e}"
                    );
                }
            }
        }
    }

    #[test]
    fn source_plateau_pruning_preserves_foreign_clocks_and_real_power_jumps() {
        let mut source = sources("exp(-1000+pwrs(time-0.5,0))");
        assert!(collect(&source, 1.0, 32, true).unwrap().is_empty());
        source.voltage_sources.push(
            BehavioralVoltageSource::new("B2".to_owned(), 2, 0, 2, "table(time,0,0,0.5,1,1,0)")
                .unwrap(),
        );
        let sentinel = 0.25;
        let mut manager = BreakpointManager::new_with_tolerance(Value::from_bits(1));
        manager.extend([sentinel]);
        source
            .collect_transient_breakpoints(1.0, &mut manager, &NoAbort, 32, true)
            .unwrap();
        assert_eq!(manager.times(), &[0.0, sentinel, 0.5, 1.0]);
        let events = collect(&sources("exp(pwrs(time-0.5,0))"), 1.0, 32, true).unwrap();
        assert!(events.contains(&0.5));
        assert!(
            events
                .iter()
                .any(|&time| time < 0.5 && time.next_up() == 0.5)
        );
    }

    #[test]
    fn uncertain_domains_keep_work_limits_and_vm_zero_division() {
        let coordinate = "sin(time)/(sqr(sin(time))+sqr(cos(time)))";
        assert!(
            collect(
                &sources(&format!("({coordinate})-({coordinate})")),
                8.0,
                256,
                true
            )
            .is_ok()
        );
        assert!(collect(&sources("sin(time)/0"), 8.0, 256, true).is_ok());
        for expression in ["sin(time)/(time-0.5)", "sin(time)/(1e-300*time)"] {
            assert!(
                matches!(
                    collect(&sources(expression), 1.0, 256, true),
                    Err(BehavioralBreakpointError::Invalid(_))
                ),
                "{expression}"
            );
        }
        let source = sources(&format!("cos(time/({coordinate}))"));
        let mut manager = BreakpointManager::new();
        assert!(matches!(
            source.collect_transient_breakpoints(
                1.0,
                &mut manager,
                &CountingAbort::new(10),
                256,
                true
            ),
            Err(BehavioralBreakpointError::Aborted)
        ));
        assert!(manager.times().is_empty());
    }

    #[test]
    fn combined_feature_isolation_is_bounded_and_does_not_enumerate_constant_zeros() {
        let source = sources("exp(-10000*(cos(6*pi*time)+0.5*cos(12*pi*time)-0.25)^2)");
        let mut manager = BreakpointManager::new();
        let abort = CountingAbort::new(100);
        assert!(matches!(
            source.collect_transient_breakpoints(1.0, &mut manager, &abort, 256, true),
            Err(BehavioralBreakpointError::Aborted)
        ));
        assert!(manager.times().is_empty());
        assert!(matches!(
            collect(&source, 1.0, 3, true),
            Err(BehavioralBreakpointError::Resource(_))
        ));
        assert!(
            collect(
                &sources("exp(cos(6*pi*time)-cos(6*pi*time))"),
                1.0,
                64,
                true
            )
            .is_ok()
        );
        assert!(matches!(
            collect(
                &sources("exp(cos(1e12*time)+sin(1e12*time))"),
                1.0,
                usize::MAX,
                true
            ),
            Err(BehavioralBreakpointError::Invalid(_))
        ));
    }

    #[test]
    fn multiple_roots_and_nonlinear_phases_remain_bounded_features() {
        for (index, expression) in [
            "exp(-10000*(cos(6*pi*time)+cos(12*pi*time)-2)^2)",
            "exp(-10000*(cos(6*pi*time)+sin(6*pi*time)-sqrt(2))^2)",
            "exp(-10000*(cos(6*pi*time+0.2*sin(6*pi*time))-0.25)^2)",
        ]
        .into_iter()
        .enumerate()
        {
            let events = collect(&sources(expression), 1.0, 512, true)
                .unwrap_or_else(|error| panic!("{expression}: {error}"));
            assert!(!events.is_empty(), "{expression}");
            if index == 1 {
                for cycle in 0..3 {
                    let peak = (cycle as Value + 0.125) / 3.0;
                    assert!(
                        events
                            .iter()
                            .any(|time| (time - peak).abs() <= 8.0 * Value::EPSILON.sqrt()),
                        "missing tangential peak {peak}: {events:?}"
                    );
                }
            } else if index == 2 {
                for sign in [-1.0, 1.0] {
                    let target = (sign * 0.25_f64.acos()).rem_euclid(std::f64::consts::TAU);
                    let mut root = target;
                    for _ in 0..10 {
                        root -= (root + 0.2 * root.sin() - target) / (1.0 + 0.2 * root.cos());
                    }
                    for cycle in 0..3 {
                        let time = (root + std::f64::consts::TAU * cycle as Value)
                            / (6.0 * std::f64::consts::PI);
                        assert!(
                            contains(&events, time),
                            "missing phase root {time}: {events:?}"
                        );
                    }
                }
            }
        }
        let mut source = sources("cos(6*pi*time)>0.5*cos(12*pi*time)+0.5");
        let events = collect(&source, 1.0, 512, true).unwrap();
        let changes = events
            .windows(2)
            .filter(|pair| {
                pair[0].next_up() == pair[1]
                    && source.voltage_sources[0].evaluate(&[], pair[0]).unwrap()
                        != source.voltage_sources[0].evaluate(&[], pair[1]).unwrap()
            })
            .count();
        assert_eq!(changes, 12, "{events:?}");
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
