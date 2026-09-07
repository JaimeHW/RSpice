//! Structural levels and extrema of time-only nonlinear source coordinates.
//!
//! These events expose features before shooting samples can alias them away.
//! They supplement the finite Fourier-band contract and solved-grid error
//! control; unknown coordinates do not acquire a resolution certificate.

use super::*;

impl EventSchedule<'_> {
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
            // The first BE interval supplies the outgoing algebraic value at
            // a seam event; no derivative history exists at time zero.
            if root == 0.0 {
                self.add(root)?;
                continue;
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
                self.add(left)?;
                self.add(right)?;
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
                }
            }
            Expr::Function { func, args } => match (func, args.as_slice()) {
                (Function::Sin | Function::Cos, [phase]) if (-1.0..=1.0).contains(&target) => {
                    if let Some((rate, offset)) = affine_time_coordinate(phase, context) {
                        self.trigonometric_levels(*func, rate, offset, target, 0.0)?;
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
