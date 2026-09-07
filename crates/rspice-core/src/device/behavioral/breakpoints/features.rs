//! Structural levels and extrema of time-only nonlinear source coordinates.
//!
//! These events expose features before shooting samples can alias them away.
//! They supplement the finite Fourier-band contract and solved-grid error
//! control; unknown coordinates do not acquire a resolution certificate.

use super::*;
use crate::expr::{TimeEnclosure, TimeInterval, compile_time_expression};

impl EventSchedule<'_> {
    fn isolated_levels(
        &mut self,
        expr: &Expr,
        target: Value,
        context: &Context<'_>,
    ) -> Result<(), BehavioralBreakpointError> {
        let window = TimeInterval {
            lower: 0.0,
            upper: self.tstop,
        };
        // A finite identical subtraction is a constant zero, not an
        // uncountable sequence of isolated roots. Preserve nonfinite refusals.
        if let Expr::Binary {
            op: BinaryOp::Sub,
            left,
            right,
        } = expr
            && left == right
        {
            let program = compile_time_expression(left, context);
            if TimeEnclosure::new(&program, self.tstop)
                .and_then(|mut bounds| bounds.evaluate(window, context))
                .is_some_and(|(value, _)| value.is_finite())
            {
                return Ok(());
            }
        }
        let program = compile_time_expression(expr, context);
        let Some(mut bounds) = TimeEnclosure::new(&program, self.tstop) else {
            return Ok(());
        };
        let Some((value, slope)) = bounds.evaluate(window, context) else {
            return Ok(());
        };
        if !value.contains(target) || (slope.lower == 0.0 && slope.upper == 0.0) {
            return Ok(());
        }
        let mut vm = Vm::new();
        let mut evaluate = |time| {
            let value = vm.execute(&program, &Context { time, ..*context });
            if value.is_finite() {
                Ok(value)
            } else {
                Err(BehavioralBreakpointError::Invalid(
                    "a time-coordinate value is non-finite",
                ))
            }
        };
        let mut pending = vec![window];
        let mut operations = program.instructions.len();
        let mut charge = || {
            self.poll()?;
            operations = operations.saturating_add(program.instructions.len());
            if operations > 16_000_000 {
                Err(BehavioralBreakpointError::Invalid(
                    "time-feature isolation exceeds its 16000000-instruction work limit",
                ))
            } else {
                Ok(())
            }
        };
        // Accumulate separately while the work-budget closure borrows this
        // schedule. The ordinary distinct-event budget remains authoritative.
        let mut roots = BTreeSet::new();
        while let Some(interval) = pending.pop() {
            charge()?;
            let (value, slope) =
                bounds
                    .evaluate(interval, context)
                    .ok_or(BehavioralBreakpointError::Invalid(
                        "a continuous time-coordinate bound is unavailable",
                    ))?;
            if !value.contains(target) {
                continue;
            }
            if !slope.contains(0.0) {
                let mut left = interval.lower;
                let mut right = interval.upper;
                charge()?;
                let left_value = evaluate(left)?;
                charge()?;
                let right_value = evaluate(right)?;
                if left_value == target {
                    roots.insert(left.to_bits());
                } else if right_value == target {
                    roots.insert(right.to_bits());
                } else if (left_value < target) != (right_value < target) {
                    while right.to_bits() - left.to_bits() > 1 {
                        charge()?;
                        let midpoint = Value::from_bits(
                            left.to_bits() + (right.to_bits() - left.to_bits()) / 2,
                        );
                        let value = evaluate(midpoint)?;
                        if value == target {
                            left = midpoint;
                            break;
                        }
                        if (value < target) == (left_value < target) {
                            left = midpoint;
                        } else {
                            right = midpoint;
                        }
                    }
                    roots.insert(left.to_bits());
                } else {
                    // An endpoint may straddle the real level within its
                    // rounded-expression enclosure. Keep it as a feature;
                    // switching boundaries are subsequently located by VM.
                    for time in [left, right] {
                        charge()?;
                        if bounds
                            .evaluate(
                                TimeInterval {
                                    lower: time,
                                    upper: time,
                                },
                                context,
                            )
                            .is_some_and(|(value, _)| value.contains(target))
                        {
                            roots.insert(time.to_bits());
                        }
                    }
                }
                ResourceLimitError::ensure(
                    ResourceKind::AnalysisPoints,
                    roots.len(),
                    self.max_points,
                )?;
                continue;
            }
            let midpoint = interval.lower + 0.5 * (interval.upper - interval.lower);
            // A multiple root can be indistinguishable from zero throughout
            // a small cluster at expression rounding precision. Keep the
            // cluster's endpoints and center as features, rather than
            // enumerating every floating-point timestamp in the plateau.
            // This supplies mesh geometry, not a simple-root derivative
            // certificate; switching still uses actual one-sided VM values.
            charge()?;
            if let Some((point, _)) = bounds.evaluate(
                TimeInterval {
                    lower: midpoint,
                    upper: midpoint,
                },
                context,
            ) {
                let uncertainty = point.upper - point.lower;
                if uncertainty.is_finite()
                    && uncertainty > 0.0
                    && point.contains(target)
                    && value.upper - value.lower <= 2.0 * uncertainty
                {
                    for time in [interval.lower, midpoint, interval.upper] {
                        roots.insert(time.to_bits());
                    }
                    ResourceLimitError::ensure(
                        ResourceKind::AnalysisPoints,
                        roots.len(),
                        self.max_points,
                    )?;
                    continue;
                }
            }
            if midpoint == interval.lower || midpoint == interval.upper {
                return Err(BehavioralBreakpointError::Invalid(
                    "a grazing time feature cannot be isolated at the available time precision",
                ));
            }
            pending.push(TimeInterval {
                lower: midpoint,
                upper: interval.upper,
            });
            pending.push(TimeInterval {
                lower: interval.lower,
                upper: midpoint,
            });
        }
        for time in roots {
            self.add(Value::from_bits(time))?;
        }
        Ok(())
    }

    fn nonlinear_phase_levels(
        &mut self,
        function: Function,
        phase: &Expr,
        target: Value,
        context: &Context<'_>,
    ) -> Result<(), BehavioralBreakpointError> {
        let program = compile_time_expression(phase, context);
        let Some((range, _)) = TimeEnclosure::new(&program, self.tstop).and_then(|mut bounds| {
            bounds.evaluate(
                TimeInterval {
                    lower: 0.0,
                    upper: self.tstop,
                },
                context,
            )
        }) else {
            return Ok(());
        };
        if !range.is_finite() {
            return Err(BehavioralBreakpointError::Invalid(
                "a nonlinear phase range is not finite",
            ));
        }
        let angle = if function == Function::Cos {
            target.acos()
        } else {
            target.asin()
        };
        let phases = [
            angle,
            if function == Function::Cos {
                -angle
            } else {
                std::f64::consts::PI - angle
            },
        ]
        .map(|phase| phase.rem_euclid(std::f64::consts::TAU));
        for (index, root) in phases.into_iter().enumerate() {
            if index == 1 && root == phases[0] {
                continue;
            }
            let first = ((range.lower - root) / std::f64::consts::TAU).ceil();
            let last = ((range.upper - root) / std::f64::consts::TAU).floor();
            if first.abs().max(last.abs()) >= (1_u64 << 53) as Value {
                return Err(BehavioralBreakpointError::Invalid(
                    "nonlinear phase cycle indices are not representable",
                ));
            }
            let count = (last - first + 1.0).max(0.0);
            if count > 1_000_000.0 {
                return Err(BehavioralBreakpointError::Invalid(
                    "nonlinear phase schedule exceeds its 1000000-cycle work limit",
                ));
            }
            for index in 0..count as usize {
                self.poll()?;
                self.level(
                    phase,
                    std::f64::consts::TAU.mul_add(first + index as Value, root),
                    context,
                )?;
            }
        }
        Ok(())
    }

    fn trigonometric_levels(
        &mut self,
        function: Function,
        rate: Value,
        offset: Value,
        target: Value,
        time_start: Value,
    ) -> Result<(), BehavioralBreakpointError> {
        if !(-1.0..=1.0).contains(&target) {
            return Ok(());
        }
        let roots = if function == Function::Cos {
            let angle = target.acos();
            [angle, -angle]
        } else {
            let angle = target.asin();
            [angle, std::f64::consts::PI - angle]
        };
        let roots = roots.map(|phase| phase.rem_euclid(std::f64::consts::TAU));
        for (index, phase) in roots.into_iter().enumerate() {
            if index != 1 || phase != roots[0] {
                self.periodic_coordinate(
                    rate,
                    offset,
                    phase,
                    std::f64::consts::TAU,
                    false,
                    time_start,
                )?;
            }
        }
        Ok(())
    }

    /// Resolve the VM's actual one-sided values, including floating-point
    /// equality plateaus around an analytic root. Bitwise bisection terminates
    /// in at most 64 trials without an absolute time epsilon.
    fn discontinuities(
        &mut self,
        expr: &Expr,
        input: &Expr,
        levels: &[Value],
        context: &Context<'_>,
    ) -> Result<(), BehavioralBreakpointError> {
        let mut candidates = EventSchedule {
            tstop: self.tstop,
            abort: self.abort,
            max_points: self.max_points,
            physical_corners: self.physical_corners,
            events: BTreeSet::new(),
        };
        for &level in levels {
            candidates.level(input, level, context)?;
        }
        let mut roots = candidates
            .events
            .into_iter()
            .map(Value::from_bits)
            .peekable();
        if roots.peek().is_none() {
            return Ok(());
        }
        let program = compile(expr);
        if !program.node_map.is_empty() || !program.branch_map.is_empty() || program.sdt_count != 0
        {
            return Ok(());
        }
        let mut vm = Vm::new();
        let mut evaluate = |time| vm.execute(&program, &Context { time, ..*context });
        let mut previous = 0.0;
        while let Some(root) = roots.next() {
            self.poll()?;
            if root == 0.0 {
                self.add(root)?;
            }
            let next = roots.peek().copied().unwrap_or(self.tstop);
            let left = previous + 0.5 * (root - previous);
            let right = root + 0.5 * (next - root);
            previous = root;
            let mut located = false;
            for (mut left, mut right) in [(left, root), (root, right)] {
                let left_value = evaluate(left);
                let right_value = evaluate(right);
                if !left_value.is_finite() || !right_value.is_finite() {
                    return Err(BehavioralBreakpointError::Invalid(
                        "a switching expression is non-finite",
                    ));
                }
                if left_value == right_value {
                    continue;
                }
                while right.to_bits() - left.to_bits() > 1 {
                    self.poll()?;
                    let midpoint =
                        Value::from_bits(left.to_bits() + (right.to_bits() - left.to_bits()) / 2);
                    let value = evaluate(midpoint);
                    if !value.is_finite() {
                        return Err(BehavioralBreakpointError::Invalid(
                            "a switching expression is non-finite",
                        ));
                    }
                    if value == left_value {
                        left = midpoint;
                    } else {
                        right = midpoint;
                    }
                }
                // An instantaneous seam uses the initial BE interval. A
                // finite equality plateau after time zero has a distinct
                // outgoing boundary and must be retained like any other.
                if left != 0.0 || right != Value::from_bits(1) {
                    self.add(left)?;
                    self.add(right)?;
                }
                located = true;
            }
            if !located {
                self.add(root)?;
            }
        }
        Ok(())
    }

    /// Invert supported scalar compositions without sampling their output.
    /// Unknown expressions contribute no roots; their children still receive
    /// their own structural feature analysis.
    fn level(
        &mut self,
        expr: &Expr,
        target: Value,
        context: &Context<'_>,
    ) -> Result<(), BehavioralBreakpointError> {
        self.poll()?;
        if !target.is_finite() {
            return Ok(());
        }
        if let Some(clock) = TimeCoordinate::from_expr(expr, context) {
            return clock.visit(target, self);
        }
        match expr {
            Expr::Unary {
                op: UnaryOp::Neg,
                operand,
            } => self.level(operand, -target, context)?,
            Expr::Binary { op, left, right } => {
                if let Some(value) = constant_value(right, context) {
                    match op {
                        BinaryOp::Add => self.level(left, target - value, context)?,
                        BinaryOp::Sub => self.level(left, target + value, context)?,
                        BinaryOp::Mul if value != 0.0 => {
                            self.level(left, target / value, context)?
                        }
                        BinaryOp::Div if value != 0.0 => {
                            self.level(left, target * value, context)?
                        }
                        BinaryOp::Pow if value > 0.0 && value.fract() == 0.0 => {
                            let even = value % 2.0 == 0.0;
                            if target >= 0.0 || !even {
                                let root = target.abs().powf(1.0 / value).copysign(target);
                                self.level(left, root, context)?;
                                if even && root != 0.0 {
                                    self.level(left, -root, context)?;
                                }
                            }
                        }
                        _ => {}
                    }
                } else if let Some(value) = constant_value(left, context) {
                    match op {
                        BinaryOp::Add => self.level(right, target - value, context)?,
                        BinaryOp::Sub => self.level(right, value - target, context)?,
                        BinaryOp::Mul if value != 0.0 => {
                            self.level(right, target / value, context)?
                        }
                        BinaryOp::Div if target != 0.0 => {
                            self.level(right, value / target, context)?
                        }
                        _ => {}
                    }
                } else {
                    self.isolated_levels(expr, target, context)?;
                }
            }
            Expr::Function { func, args } => match (func, args.as_slice()) {
                (Function::Sin | Function::Cos, [phase]) if (-1.0..=1.0).contains(&target) => {
                    if let Some((rate, offset)) = affine_time_coordinate(phase, context) {
                        self.trigonometric_levels(*func, rate, offset, target, 0.0)?;
                    } else {
                        self.nonlinear_phase_levels(*func, phase, target, context)?;
                    }
                }
                (Function::SpiceSin, _) => {
                    if let Some(values) = args
                        .iter()
                        .map(|arg| constant_value(arg, context))
                        .collect::<Option<Vec<_>>>()
                    {
                        let [bias, amplitude, frequency, delay, damping, phase] =
                            crate::expr::spice_waveform_parameters(&values);
                        if amplitude != 0.0 && damping == 0.0 {
                            let rate = std::f64::consts::TAU * frequency;
                            self.trigonometric_levels(
                                Function::Sin,
                                rate,
                                phase.to_radians() - rate * delay,
                                affine_preimage(target, bias, amplitude),
                                delay.max(0.0),
                            )?;
                        }
                    }
                }
                (Function::Abs | Function::Sqr, [input]) if target >= 0.0 => {
                    let root = if *func == Function::Sqr {
                        target.sqrt()
                    } else {
                        target
                    };
                    self.level(input, root, context)?;
                    if root != 0.0 {
                        self.level(input, -root, context)?;
                    }
                }
                (Function::Exp, [input]) if target > 0.0 => {
                    self.level(input, target.ln(), context)?
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    pub(super) fn temporal_features(
        &mut self,
        expr: &Expr,
        context: &Context<'_>,
    ) -> Result<(), BehavioralBreakpointError> {
        match expr {
            Expr::Binary { op, left, right } => match op {
                BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge
                | BinaryOp::Eq
                | BinaryOp::Ne => {
                    let levels = if matches!(op, BinaryOp::Eq | BinaryOp::Ne) {
                        [-EXPR_ZERO_TOLERANCE, EXPR_ZERO_TOLERANCE]
                    } else {
                        [0.0, 0.0]
                    };
                    if let Some(value) = constant_value(right, context) {
                        self.discontinuities(
                            expr,
                            left,
                            &levels.map(|offset| value + offset),
                            context,
                        )?;
                    } else if let Some(value) = constant_value(left, context) {
                        self.discontinuities(
                            expr,
                            right,
                            &levels.map(|offset| value + offset),
                            context,
                        )?;
                    } else {
                        let difference = Expr::Binary {
                            op: BinaryOp::Sub,
                            left: left.clone(),
                            right: right.clone(),
                        };
                        self.discontinuities(expr, &difference, &levels, context)?;
                    }
                }
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Pow => {
                    self.level(expr, 0.0, context)?;
                }
                _ => {}
            },
            Expr::Function { func, args } => match (func, args.as_slice()) {
                (Function::Sin | Function::Cos, [_]) => {
                    for level in [-1.0, 0.0, 1.0] {
                        self.level(expr, level, context)?;
                    }
                }
                (Function::SpiceSin, _) => {
                    if let Some(values) = args
                        .iter()
                        .map(|arg| constant_value(arg, context))
                        .collect::<Option<Vec<_>>>()
                    {
                        let [bias, amplitude, _, _, _, _] =
                            crate::expr::spice_waveform_parameters(&values);
                        for level in [bias - amplitude, bias, bias + amplitude] {
                            self.level(expr, level, context)?;
                        }
                    }
                }
                (Function::Abs | Function::Sqr | Function::Uramp, [input]) => {
                    self.level(input, 0.0, context)?
                }
                (
                    Function::Sign
                    | Function::Ustep
                    | Function::Gt0
                    | Function::Lt0
                    | Function::Ge0
                    | Function::Le0,
                    [input],
                ) => self.discontinuities(expr, input, &[0.0], context)?,
                (Function::Stp, [input]) => {
                    self.discontinuities(expr, input, &[EXPR_ZERO_TOLERANCE], context)?
                }
                (Function::Eq0 | Function::Ne0, [input]) => {
                    self.discontinuities(
                        expr,
                        input,
                        &[-EXPR_ZERO_TOLERANCE, EXPR_ZERO_TOLERANCE],
                        context,
                    )?;
                }
                (Function::Min | Function::Max, [left, right]) => {
                    if let Some(value) = constant_value(right, context) {
                        self.level(left, value, context)?;
                    } else if let Some(value) = constant_value(left, context) {
                        self.level(right, value, context)?;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
}
