//! Structural time-shift certification for behavioral forcing. Periodic
//! circuit variables remain symbolic; no waveform samples can prove this
//! property, because nonperiodic functions can alias on the entire grid.

use super::*;
use crate::numerics::is_integral_cycle_count;

impl BehavioralVoltageSource {
    pub(crate) fn minimum_pss_interval(&self, events_resolved: bool) -> Option<Value> {
        minimum_pss_interval(&self.ast, &self.periodicity_context(), events_resolved)
    }

    pub(crate) fn has_periodic_time_dependence(&self, period: Value, autonomous: bool) -> bool {
        time_increment(&self.ast, period, &self.periodicity_context(), autonomous) == Some(0.0)
    }

    pub(crate) fn max_authored_tone_cycles(&self, period: Value) -> Value {
        let context = self.periodicity_context();
        max_authored_tone_cycles(&self.ast, period, &context)
            .max(finite_fourier_degree(&self.ast, period, &context).unwrap_or(0.0))
    }

    pub(super) fn periodicity_context(&self) -> Context<'_> {
        Context::transient(&[], &[], 0.0)
            .with_temperature(self.temperature)
            .with_frequency(self.frequency)
            .with_gmin(self.gmin)
            .with_expression_dialect(self.expression_dialect)
    }
}

impl BehavioralCurrentSource {
    pub(crate) fn minimum_pss_interval(&self, events_resolved: bool) -> Option<Value> {
        minimum_pss_interval(&self.ast, &self.periodicity_context(), events_resolved)
    }

    pub(crate) fn has_periodic_time_dependence(&self, period: Value, autonomous: bool) -> bool {
        time_increment(&self.ast, period, &self.periodicity_context(), autonomous) == Some(0.0)
    }

    pub(crate) fn max_authored_tone_cycles(&self, period: Value) -> Value {
        let context = self.periodicity_context();
        max_authored_tone_cycles(&self.ast, period, &context)
            .max(finite_fourier_degree(&self.ast, period, &context).unwrap_or(0.0))
    }

    pub(super) fn periodicity_context(&self) -> Context<'_> {
        Context::transient(&[], &[], 0.0)
            .with_temperature(self.temperature)
            .with_frequency(self.frequency)
            .with_gmin(self.gmin)
            .with_expression_dialect(self.expression_dialect)
    }
}

fn implicit_time(function: Function) -> bool {
    matches!(
        function,
        Function::Sdt
            | Function::SpiceSin
            | Function::SpicePulse
            | Function::SpiceExp
            | Function::SpiceSffm
    )
}

fn constant_over_time(expr: &Expr) -> bool {
    match expr {
        Expr::Time | Expr::NodeVoltage(_) | Expr::BranchCurrent(_) | Expr::StringLiteral(_) => {
            false
        }
        Expr::Unary { operand, .. } => constant_over_time(operand),
        Expr::Binary { left, right, .. } => constant_over_time(left) && constant_over_time(right),
        Expr::Function { func, args } => {
            !implicit_time(*func) && args.iter().all(constant_over_time)
        }
        Expr::LookupTable { input, .. } => constant_over_time(input),
        _ => true,
    }
}

pub(super) fn constant_value(expr: &Expr, context: &Context<'_>) -> Option<Value> {
    if !constant_over_time(expr) {
        return None;
    }
    // Use the existing evaluator and resolved dialect/environment, including
    // named power functions and temperature. Do not duplicate their semantics.
    let value = Vm::new().execute(&compile(expr), context);
    value.is_finite().then_some(value)
}

fn multiply_increment(increment: Value, scale: Value) -> Option<Value> {
    let result = increment * scale;
    // Underflow is not proof of a zero time increment. Such a ramp can be
    // invisible over the shooting grid and still change on a longer run.
    (increment == 0.0 || scale == 0.0 || result != 0.0).then_some(result)
}

/// Upper bound on a table coordinate's time slope between modulo resets.
/// Unknown circuit-dependent coordinates do not provide a timing certificate.
fn time_coordinate_rate(expr: &Expr, context: &Context<'_>) -> Option<Value> {
    if let Some(increment) = affine_time_increment(expr, 1.0, context) {
        return increment.is_finite().then_some(increment.abs());
    }
    let rate = |expr: &Expr| time_coordinate_rate(expr, context);
    match expr {
        Expr::Unary {
            op: UnaryOp::Neg,
            operand,
        } => rate(operand),
        Expr::Binary { op, left, right } => match op {
            BinaryOp::Mod if constant_value(right, context).is_some_and(|value| value != 0.0) => {
                rate(left)
            }
            BinaryOp::Add | BinaryOp::Sub => Some(rate(left)? + rate(right)?),
            BinaryOp::Mul => {
                if let Some(scale) = constant_value(left, context) {
                    Some(scale.abs() * rate(right)?)
                } else {
                    Some(rate(left)? * constant_value(right, context)?.abs())
                }
            }
            BinaryOp::Div if constant_value(right, context).is_some_and(|value| value != 0.0) => {
                Some(rate(left)? / constant_value(right, context)?.abs())
            }
            _ => None,
        },
        Expr::Function { func, args } => match (func, args.as_slice()) {
            (Function::Mod, [input, divisor])
                if constant_value(divisor, context).is_some_and(|value| value != 0.0) =>
            {
                rate(input)
            }
            (Function::Sin | Function::Cos, [phase]) => rate(phase),
            _ => None,
        },
        _ => None,
    }
}

fn minimum_pss_interval(
    expr: &Expr,
    context: &Context<'_>,
    events_resolved: bool,
) -> Option<Value> {
    let interval = |expr: &Expr| minimum_pss_interval(expr, context, events_resolved);
    let shortest = |a: Option<Value>, b: Option<Value>| a.into_iter().chain(b).reduce(Value::min);
    let table_interval = |input: &Expr, points: &[(Value, Value)], has_events: bool| {
        let distance = crate::numerics::minimum_pwl_interval(points.iter().copied())?;
        let rate = time_coordinate_rate(input, context)?;
        if rate == 0.0 {
            return None;
        }
        let interval = distance / rate;
        if interval != 0.0
            && events_resolved
            && has_events
            && super::breakpoints::has_exact_table_clock(input, context)
        {
            None
        } else {
            Some(interval)
        }
    };
    match expr {
        Expr::Unary { operand, .. } => interval(operand),
        Expr::Binary { left, right, .. } => shortest(interval(left), interval(right)),
        Expr::LookupTable { input, table } => shortest(
            interval(input),
            table_interval(input, &table.points, table.transient_breakpoints),
        ),
        Expr::Function { func, args } => {
            let nested = args.iter().filter_map(interval).reduce(Value::min);
            let own = match (func, args.as_slice()) {
                (Function::Table | Function::Pwl, [input, points @ ..]) => {
                    let points = points
                        .chunks_exact(2)
                        .map(|pair| {
                            Some((
                                constant_value(&pair[0], context)?,
                                constant_value(&pair[1], context)?,
                            ))
                        })
                        .collect::<Option<Vec<_>>>();
                    points.and_then(|points| table_interval(input, &points, true))
                }
                (Function::SpicePulse, _) => args
                    .iter()
                    .map(|arg| constant_value(arg, context))
                    .collect::<Option<Vec<_>>>()
                    .and_then(|values| {
                        if events_resolved {
                            None
                        } else {
                            crate::expr::spice_waveform_minimum_interval(*func, &values)
                        }
                    }),
                _ => None,
            };
            shortest(own, nested)
        }
        _ => None,
    }
}

/// Constant phase increment for an affine function of time. Circuit variables
/// and nonlinear phase modulation cannot establish a finite Fourier degree.
pub(super) fn affine_time_increment(
    expr: &Expr,
    period: Value,
    context: &Context<'_>,
) -> Option<Value> {
    if constant_value(expr, context).is_some() {
        return Some(0.0);
    }
    let increment = |expr: &Expr| affine_time_increment(expr, period, context);
    match expr {
        Expr::Time => Some(period),
        Expr::Unary {
            op: UnaryOp::Neg,
            operand,
        } => increment(operand).map(|value| -value),
        Expr::Binary { op, left, right } => match op {
            BinaryOp::Add => Some(increment(left)? + increment(right)?),
            BinaryOp::Sub => Some(increment(left)? - increment(right)?),
            BinaryOp::Mul => {
                if let Some(scale) = constant_value(left, context) {
                    multiply_increment(increment(right)?, scale)
                } else {
                    multiply_increment(increment(left)?, constant_value(right, context)?)
                }
            }
            BinaryOp::Div => {
                let divisor = constant_value(right, context)?;
                (divisor != 0.0)
                    .then(|| increment(left).map(|value| value / divisor))
                    .flatten()
            }
            _ => None,
        },
        _ => None,
    }
}

/// Upper harmonic index of a time-only trigonometric polynomial, without
/// expanding its coefficients. Products add degrees and integer powers
/// multiply them; looking only at the inner sine clocks misses these bands.
/// None denotes an unknown/infinite band, never a zero-band certificate.
fn finite_fourier_degree(expr: &Expr, period: Value, context: &Context<'_>) -> Option<Value> {
    if constant_value(expr, context).is_some() {
        return Some(0.0);
    }
    let degree = |expr: &Expr| finite_fourier_degree(expr, period, context);
    let power = |base: &Expr, exponent: &Expr| {
        let exponent = constant_value(exponent, context)?;
        (exponent >= 0.0 && exponent.fract() == 0.0)
            .then(|| degree(base).map(|value| value * exponent))
            .flatten()
    };
    match expr {
        Expr::Unary {
            op: UnaryOp::Neg,
            operand,
        } => degree(operand),
        Expr::Binary { op, left, right } => match op {
            BinaryOp::Add | BinaryOp::Sub => Some(degree(left)?.max(degree(right)?)),
            BinaryOp::Mul => Some(degree(left)? + degree(right)?),
            BinaryOp::Div if constant_value(right, context).is_some_and(|value| value != 0.0) => {
                degree(left)
            }
            BinaryOp::Pow => power(left, right),
            _ => None,
        },
        Expr::Function { func, args } => match (func, args.as_slice()) {
            (Function::Sin | Function::Cos, [phase]) => {
                Some(affine_time_increment(phase, period, context)?.abs() / std::f64::consts::TAU)
            }
            (Function::Pow, [base, exponent])
                if context.expression_dialect == crate::config::ExpressionDialect::Xyce
                    || constant_value(exponent, context)
                        .is_some_and(|value| value.rem_euclid(2.0) == 0.0) =>
            {
                power(base, exponent)
            }
            (Function::Pwr, [base, exponent])
                if context.expression_dialect == crate::config::ExpressionDialect::Xyce
                    || constant_value(exponent, context)
                        .is_some_and(|value| value.rem_euclid(2.0) == 1.0) =>
            {
                power(base, exponent)
            }
            (Function::Pwrs, [base, exponent])
                if constant_value(exponent, context)
                    .is_some_and(|value| value.rem_euclid(2.0) == 1.0) =>
            {
                power(base, exponent)
            }
            (Function::SpiceSin, _) => {
                let values = args
                    .iter()
                    .map(|arg| constant_value(arg, context))
                    .collect::<Option<Vec<_>>>()?;
                crate::expr::spice_waveform_is_periodic(*func, &values, period, false)
                    .then(|| crate::expr::spice_waveform_max_tone_cycles(*func, &values, period))
            }
            _ => None,
        },
        _ => None,
    }
}

/// Shared affine clock geometry for source events and time-shift proofs.
pub(super) fn affine_time_coordinate(expr: &Expr, context: &Context<'_>) -> Option<(Value, Value)> {
    let rate = affine_time_increment(expr, 1.0, context)?;
    let offset = Vm::new().execute(&compile(expr), context);
    (rate.is_finite() && offset.is_finite()).then_some((rate, offset))
}

fn periodic_remainder(input: &Expr, divisor: &Expr, period: Value, context: &Context<'_>) -> bool {
    let Some((rate, offset)) = affine_time_coordinate(input, context) else {
        return false;
    };
    // A shifted affine clock that crosses zero changes the sign of `%`, so
    // its initial waveform need not repeat on later periods.
    if (rate > 0.0 && offset < 0.0) || (rate < 0.0 && offset > 0.0) {
        return false;
    }
    let Some(divisor) = constant_value(divisor, context) else {
        return false;
    };
    let Some(increment) = affine_time_increment(input, period, context) else {
        return false;
    };
    divisor != 0.0 && is_integral_cycle_count((increment / divisor).abs())
}

/// Inspect authored phase increments rather than sampled values, which can
/// all vanish when a clock aliases. Nested phase modulation retains its inner
/// clock. This is not a bound on harmonics generated by arbitrary expressions.
fn max_authored_tone_cycles(expr: &Expr, period: Value, context: &Context<'_>) -> Value {
    let cycles = |expr| max_authored_tone_cycles(expr, period, context);
    match expr {
        Expr::Unary { operand, .. } => cycles(operand),
        Expr::Binary { left, right, .. } => cycles(left).max(cycles(right)),
        Expr::LookupTable { input, .. } => cycles(input),
        Expr::Function { func, args } => {
            let nested = args.iter().map(cycles).fold(0.0, Value::max);
            let own = match (func, args.as_slice()) {
                (Function::Sin | Function::Cos | Function::Tan, [phase]) => {
                    let cycle = if *func == Function::Tan {
                        std::f64::consts::PI
                    } else {
                        std::f64::consts::TAU
                    };
                    time_increment(phase, period, context, false)
                        .map_or(0.0, |delta| delta.abs() / cycle)
                }
                _ if implicit_time(*func) => args
                    .iter()
                    .map(|arg| constant_value(arg, context))
                    .collect::<Option<Vec<_>>>()
                    .map_or(0.0, |values| {
                        crate::expr::spice_waveform_max_tone_cycles(*func, &values, period)
                    }),
                _ => 0.0,
            };
            own.max(nested)
        }
        _ => 0.0,
    }
}

/// Return a certified constant increment f(t+T)-f(t), assuming that circuit
/// voltages/currents themselves repeat. None means no certificate is known.
fn time_increment(
    expr: &Expr,
    period: Value,
    context: &Context<'_>,
    autonomous: bool,
) -> Option<Value> {
    let shift = |expr| time_increment(expr, period, context, autonomous);
    let result = match expr {
        Expr::Time => Some(period),
        Expr::StringLiteral(_) => None,
        Expr::Const(value) => value.is_finite().then_some(0.0),
        Expr::NodeVoltage(_)
        | Expr::BranchCurrent(_)
        | Expr::Frequency
        | Expr::Temperature
        | Expr::ThermalVoltage
        | Expr::Gmin => Some(0.0),
        Expr::LookupTable { input, .. } => (shift(input)? == 0.0).then_some(0.0),
        Expr::Unary {
            op: UnaryOp::Neg,
            operand,
        } => shift(operand).map(|delta| -delta),
        Expr::Unary { operand, .. } => (shift(operand)? == 0.0).then_some(0.0),
        Expr::Binary { op, left, right } => {
            let dl = shift(left)?;
            let dr = shift(right)?;
            match op {
                BinaryOp::Add => Some(dl + dr),
                BinaryOp::Sub => Some(dl - dr),
                _ if dl == 0.0 && dr == 0.0 => Some(0.0),
                BinaryOp::Mul if dl == 0.0 => {
                    multiply_increment(dr, constant_value(left, context)?)
                }
                BinaryOp::Mul if dr == 0.0 => {
                    multiply_increment(dl, constant_value(right, context)?)
                }
                BinaryOp::Div if dr == 0.0 => {
                    let result = dl / constant_value(right, context)?;
                    (result != 0.0).then_some(result)
                }
                BinaryOp::Mod if !autonomous && dr == 0.0 => {
                    periodic_remainder(left, right, period, context).then_some(0.0)
                }
                _ => None,
            }
        }
        Expr::Function { func, args } => {
            if implicit_time(*func) {
                let values = args
                    .iter()
                    .map(|arg| constant_value(arg, context))
                    .collect::<Option<Vec<_>>>()?;
                return crate::expr::spice_waveform_is_periodic(*func, &values, period, autonomous)
                    .then_some(0.0);
            }
            match (func, args.as_slice()) {
                (Function::Sin | Function::Cos | Function::Tan, [arg]) => {
                    let delta = shift(arg)?;
                    let cycle = if *func == Function::Tan {
                        std::f64::consts::PI
                    } else {
                        std::f64::consts::TAU
                    };
                    (delta == 0.0
                        || (!autonomous && is_integral_cycle_count((delta / cycle).abs())))
                    .then_some(0.0)
                }
                (Function::Floor | Function::Ceil, [arg]) => {
                    let delta = shift(arg)?;
                    (delta == 0.0 || (!autonomous && is_integral_cycle_count(delta.abs())))
                        .then_some(delta.round())
                }
                (Function::Mod, [input, divisor]) => ((shift(input) == Some(0.0)
                    && shift(divisor) == Some(0.0))
                    || (!autonomous && periodic_remainder(input, divisor, period, context)))
                .then_some(0.0),
                _ => args
                    .iter()
                    .all(|arg| shift(arg) == Some(0.0))
                    .then_some(0.0),
            }
        }
    };
    result.filter(|delta| delta.is_finite())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_source_intervals_follow_physical_time_coordinates() {
        for (expression, expected) in [
            (
                "table(time%1u,0,0,400p,0,410p,1,510p,1,520p,0,1u,0)",
                Some(1e-11),
            ),
            (
                "table(mod(time,1u)*1meg,0,0,0.0004,0,0.00041,1,0.00051,1,0.00052,0,1,0)",
                Some(1e-11),
            ),
            (
                "table((time%1u)/1n,0,0,0.4,0,0.41,1,0.51,1,0.52,0,1000,0)",
                Some(1e-11),
            ),
            ("table(time%1u,0,1,1e-300,1,1u,1)", None),
            ("table(v(out),0,0,1e-9,1,1,0)", None),
            ("spice_pulse(0,1,400p,10p,10p,100p,1u)", Some(1e-11)),
            ("spice_pulse(1,1,0,1e-300,1e-300,1e-300,1u)", None),
            ("spice_pulse(0,1,0,0,0,0,1e-300)", None),
        ] {
            let source =
                BehavioralVoltageSource::new("B1".to_owned(), 1, 0, 1, expression).unwrap();
            match (source.minimum_pss_interval(false), expected) {
                (Some(actual), Some(expected)) => assert!(
                    (actual / expected - 1.0).abs() < 1e-12,
                    "{expression}: {actual:e}"
                ),
                (None, None) => {}
                (actual, expected) => panic!("{expression}: {actual:?}, expected {expected:?}"),
            }
        }
    }

    #[test]
    fn harmonic_degree_preserves_dialect_power_semantics_and_unknown_bands() {
        use crate::config::ExpressionDialect::{Ngspice, Xyce};
        for (expression, ngspice_degree, xyce_degree) in [
            ("sin(2*pi*64meg*time)^8", Some(512.0), Some(512.0)),
            (
                "sin(2*pi*64meg*time)*cos(2*pi*3meg*time)",
                Some(67.0),
                Some(67.0),
            ),
            ("pow(sin(2*pi*64meg*time),3)", None, Some(192.0)),
            ("pow(sin(2*pi*64meg*time),4)", Some(256.0), Some(256.0)),
            ("pwr(sin(2*pi*64meg*time),3)", Some(192.0), Some(192.0)),
            ("pwr(sin(2*pi*64meg*time),4)", None, Some(256.0)),
            ("pwrs(sin(2*pi*64meg*time),3)", Some(192.0), Some(192.0)),
            ("pwrs(sin(2*pi*64meg*time),4)", None, None),
            ("sin(2*pi*64meg*time)^0.5", None, None),
            ("sin(2*pi*64meg*time)^-2", None, None),
            ("exp(sin(2*pi*64meg*time))", None, None),
            ("sin(2*pi*64meg*time+v(out))", None, None),
        ] {
            let ast = parse_expression_strict(expression).unwrap();
            for (dialect, expected) in [(Ngspice, ngspice_degree), (Xyce, xyce_degree)] {
                let context = Context::transient(&[], &[], 0.0).with_expression_dialect(dialect);
                let actual = finite_fourier_degree(&ast, 1e-6, &context);
                match (actual, expected) {
                    (Some(actual), Some(expected)) => assert!(
                        (actual - expected).abs() < 1e-10,
                        "{dialect:?}: {expression}: {actual}"
                    ),
                    (None, None) => {}
                    _ => panic!("{dialect:?}: {expression}: {actual:?}, expected {expected:?}"),
                }
            }
        }
    }

    #[test]
    fn time_shift_certificate_distinguishes_periods_aliases_and_implicit_clocks() {
        for (expression, periodic, autonomous) in [
            ("sin(2*pi*1meg*time)", true, false),
            ("cos(-2*pi*3meg*time+0.7)", true, false),
            ("sin(2*pi*1.5meg*time)", false, false),
            ("sin(2*pi*1meg*time)*exp(-time)", false, false),
            ("time-time+v(out)", true, true),
            ("time*1e-320", false, false),
            ("time%1u", true, false),
            ("mod(time,1u)", true, false),
            ("mod(time-0.5u,1u)", false, false),
            ("1meg*time-floor(1meg*time)", true, false),
            ("floor(1meg*time)", false, false),
            ("table(time%1u,0,0,0.5u,1,1u,0)", true, false),
            ("spice_sin(0,1,1meg)", true, false),
            ("spice_sin(0,1,1.5meg)", false, false),
            ("spice_sin(0,1,1meg,2u)", false, true),
            ("spice_pulse(0,1,0,0.1u,0.1u,0.3u,1u)", true, false),
            ("spice_pulse(0,1,2u,0.1u,0.1u,0.3u,1)", false, true),
            ("spice_exp(0,1,2u,1u,3u,1u)", false, true),
            ("spice_sffm(0,1,2meg,0.3,1meg)", true, false),
            ("spice_sffm(0,1,2meg,0.3,1.25meg)", false, false),
            ("sdt(1)", false, false),
        ] {
            let source =
                BehavioralVoltageSource::new("B1".to_owned(), 1, 0, 1, expression).unwrap();
            assert_eq!(
                source.has_periodic_time_dependence(1e-6, false),
                periodic,
                "{expression}"
            );
            assert_eq!(
                source.has_periodic_time_dependence(1e-6, true),
                autonomous,
                "autonomous: {expression}"
            );
            if periodic {
                let ast = parse_expression_strict(expression).unwrap();
                let program = compile(&ast);
                for fraction in [0.0, 0.12347, 0.31213, 0.62347, 0.92311] {
                    let values = [0.0, 1.0, 2.0].map(|cycle| {
                        Vm::new().execute(
                            &program,
                            &Context::transient(&[], &[], (fraction + cycle) * 1e-6),
                        )
                    });
                    assert!(
                        (values[0] - values[1]).abs() < 1e-11,
                        "{expression}: {values:?}"
                    );
                    assert!(
                        (values[0] - values[2]).abs() < 1e-11,
                        "{expression}: {values:?}"
                    );
                }
            }
        }
    }
}
