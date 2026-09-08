//! Structural levels and extrema of time-only nonlinear source coordinates.
//!
//! These events expose features before shooting samples can alias them away.
//! They supplement the finite Fourier-band contract and solved-grid error
//! control; unknown coordinates do not acquire a resolution certificate.

use super::*;
use crate::expr::{TimeEnclosure, TimeInterval, compile_time_expression};

impl EventSchedule<'_> {
    pub(super) fn source_expression(
        &mut self,
        expr: &Expr,
        context: &Context<'_>,
    ) -> Result<(), BehavioralBreakpointError> {
        if !needs_time_features(expr, self.tstop, context) {
            return self.expression(expr, context, false);
        }
        let program = compile_time_expression(expr, context);
        let Some(mut bounds) = TimeEnclosure::new(&program, self.tstop) else {
            return self.expression(expr, context, true);
        };
        let mut operations: usize = 0;
        let mut vm = Vm::new();
        let mut charge = || {
            self.poll()?;
            operations = operations.saturating_add(program.instructions.len());
            if operations > 16_000_000 {
                return Err(BehavioralBreakpointError::Invalid(
                    "source-feature qualification exceeds its 16000000-instruction work limit",
                ));
            }
            Ok(())
        };
        let mut is_constant = |lower, upper| -> Result<bool, BehavioralBreakpointError> {
            charge()?;
            let Some(domain) = bounds
                .evaluate(TimeInterval { lower, upper }, context)
                .filter(|domain| domain.value.is_finite())
            else {
                return Ok(false);
            };
            if domain.continuous && domain.value.lower == domain.value.upper {
                return Ok(true);
            }
            if !domain.vm_monotone {
                return Ok(false);
            }
            charge()?;
            let left = vm.execute(
                &program,
                &Context {
                    time: lower,
                    ..*context
                },
            );
            charge()?;
            let right = vm.execute(
                &program,
                &Context {
                    time: upper,
                    ..*context
                },
            );
            Ok(left.is_finite() && left == right)
        };
        if is_constant(0.0, self.tstop)? {
            return Ok(());
        }
        // Qualify only this source's internal geometry. A flat source must
        // never remove another source's clocks or pre-existing breakpoints.
        let mut source = EventSchedule {
            events: BTreeSet::new(),
            ..*self
        };
        source.expression(expr, context, true)?;
        let mut retained = BTreeSet::new();
        let mut previous = 0.0;
        let mut candidates = source.events.into_iter().peekable();
        while let Some(bits) = candidates.next() {
            let time = Value::from_bits(bits);
            let next = candidates
                .peek()
                .map_or(self.tstop, |&bits| Value::from_bits(bits));
            let mut lower = previous + 0.5 * (time - previous);
            let mut upper = time + 0.5 * (next - time);
            // Adjacent timestamps still require a nonzero neighborhood.
            // This preserves an actual VM jump on either side of a clock.
            if lower == time {
                lower = previous;
            }
            if upper == time {
                upper = next;
            }
            if lower == upper || !is_constant(lower, upper)? {
                retained.insert(bits);
            }
            previous = time;
        }
        for bits in retained {
            self.add(Value::from_bits(bits))?;
        }
        Ok(())
    }

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
        // Authenticate finite identical operands on each resolved domain;
        // an initially uncertain quotient need not invalidate the identity.
        let identical_difference = if let Expr::Binary {
            op: BinaryOp::Sub,
            left,
            right,
        } = expr
            && left == right
        {
            Some(left.as_ref())
        } else {
            None
        };
        let program = compile_time_expression(identical_difference.unwrap_or(expr), context);
        self.charge_isolation_work(program.instructions.len())?;
        let Some(mut bounds) = TimeEnclosure::new(&program, self.tstop) else {
            return Ok(());
        };
        // A zero of a-b is exactly a==b for finite VM operands. Compare the
        // operands directly: their subtraction can overflow while both
        // sources and their branch crossing remain perfectly finite.
        let operands = if target == 0.0
            && identical_difference.is_none()
            && let Expr::Binary {
                op: BinaryOp::Sub,
                left,
                right,
            } = expr
        {
            let left = compile_time_expression(left, context);
            let right = compile_time_expression(right, context);
            self.charge_isolation_work(
                left.instructions
                    .len()
                    .saturating_add(right.instructions.len()),
            )?;
            Some((left, right))
        } else {
            None
        };
        let mut operand_bounds = operands.as_ref().and_then(|(left, right)| {
            Some((
                TimeEnclosure::new(left, self.tstop)?,
                TimeEnclosure::new(right, self.tstop)?,
            ))
        });
        let mut vm = Vm::new();
        let mut evaluate = |time| {
            let point = Context { time, ..*context };
            let value = if let Some((left, right)) = &operands {
                let a = vm.execute(left, &point);
                let b = vm.execute(right, &point);
                if !a.is_finite() || !b.is_finite() {
                    Value::NAN
                } else if a < b {
                    -1.0
                } else if a > b {
                    1.0
                } else {
                    0.0
                }
            } else {
                vm.execute(&program, &point)
            };
            if value.is_finite() {
                Ok(value)
            } else {
                Err(BehavioralBreakpointError::Invalid(
                    "a time-coordinate value is non-finite",
                ))
            }
        };
        let mut pending = vec![window];
        let mut operations: usize = 0;
        // Direct comparisons evaluate both operands even when compilation of
        // the difference reused a sibling. Charge enough for either route.
        let evaluation_cost = operands
            .as_ref()
            .map_or(program.instructions.len(), |(a, b)| {
                program
                    .instructions
                    .len()
                    .max(a.instructions.len().saturating_add(b.instructions.len()))
            });
        let mut charge = || {
            self.poll()?;
            self.charge_isolation_work(evaluation_cost)?;
            operations = operations.saturating_add(evaluation_cost);
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
            let Some(domain) = bounds.evaluate(interval, context) else {
                // A denominator enclosure containing zero is uncertainty,
                // not evidence of an absent feature or an actual pole.
                // Resolve its domain with the same bounded subdivision.
                subdivide_time_domain(interval, &mut pending)?;
                continue;
            };
            if !domain.value.is_finite() {
                let finite_operands = if let Some((left, right)) = &mut operand_bounds {
                    // The shared cost covers both operand programs.
                    charge()?;
                    left.evaluate(interval, context)
                        .is_some_and(|bounds| bounds.value.is_finite())
                        && right
                            .evaluate(interval, context)
                            .is_some_and(|bounds| bounds.value.is_finite())
                } else {
                    false
                };
                if !finite_operands {
                    subdivide_time_domain(interval, &mut pending)?;
                    continue;
                }
            }
            let (value, slope) = (domain.value, domain.slope);
            if identical_difference.is_some()
                || !value.contains(target)
                || (domain.continuous && slope.lower == 0.0 && slope.upper == 0.0)
            {
                continue;
            }
            if domain.continuous && !slope.contains(0.0) {
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
                            .is_some_and(|bounds| bounds.value.contains(target))
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
            if let Some(point) = bounds.evaluate(
                TimeInterval {
                    lower: midpoint,
                    upper: midpoint,
                },
                context,
            ) {
                let uncertainty = point.value.upper - point.value.lower;
                if domain.continuous
                    && point.continuous
                    && uncertainty.is_finite()
                    && uncertainty > 0.0
                    && point.value.contains(target)
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
                charge()?;
                let left = evaluate(interval.lower)?;
                charge()?;
                let right = evaluate(interval.upper)?;
                if !domain.continuous
                    || left == target
                    || right == target
                    || (left < target) != (right < target)
                {
                    // A finite crossing between adjacent VM timestamps is
                    // an observable feature even at a nonsmooth power branch.
                    // Retain both sides; no smooth derivative is certified.
                    roots.insert(interval.lower.to_bits());
                    if interval.lower != 0.0 || interval.upper != Value::from_bits(1) {
                        roots.insert(interval.upper.to_bits());
                    }
                    ResourceLimitError::ensure(
                        ResourceKind::AnalysisPoints,
                        roots.len(),
                        self.max_points,
                    )?;
                    continue;
                }
            }
            subdivide_time_domain(interval, &mut pending)?;
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
        let Some(bounds) = TimeEnclosure::new(&program, self.tstop).and_then(|mut bounds| {
            bounds.evaluate(
                TimeInterval {
                    lower: 0.0,
                    upper: self.tstop,
                },
                context,
            )
        }) else {
            if function == Function::Tan {
                // sin(phase-atan(level)) locates every tangent level without
                // treating an unrelated tangent pole as a root to isolate.
                return self.isolated_levels(
                    &Expr::Function {
                        func: Function::Sin,
                        args: vec![Expr::Binary {
                            op: BinaryOp::Sub,
                            left: Box::new(phase.clone()),
                            right: Box::new(Expr::Const(target.atan())),
                        }],
                    },
                    0.0,
                    context,
                );
            }
            // Direct sine/cosine isolation can resolve a regular phase's
            // denominator locally when its whole-window range is unknown.
            return self.isolated_levels(
                &Expr::Function {
                    func: function,
                    args: vec![phase.clone()],
                },
                target,
                context,
            );
        };
        let range = bounds.value;
        if !range.is_finite() {
            return Err(BehavioralBreakpointError::Invalid(
                "a nonlinear phase range is not finite",
            ));
        }
        let cycle = if function == Function::Tan {
            std::f64::consts::PI
        } else {
            std::f64::consts::TAU
        };
        let angle = if function == Function::Tan {
            target.atan()
        } else if function == Function::Cos {
            target.acos()
        } else {
            target.asin()
        };
        let phases = [
            angle,
            if function == Function::Tan {
                angle
            } else if function == Function::Cos {
                -angle
            } else {
                std::f64::consts::PI - angle
            },
        ]
        .map(|phase| phase.rem_euclid(cycle));
        for (index, root) in phases.into_iter().enumerate() {
            if index == 1 && root == phases[0] {
                continue;
            }
            let first = ((range.lower - root) / cycle).ceil();
            let last = ((range.upper - root) / cycle).floor();
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
                self.level(phase, cycle.mul_add(first + index as Value, root), context)?;
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
        if function == Function::Tan {
            return self.periodic_coordinate(
                rate,
                offset,
                target.atan(),
                std::f64::consts::PI,
                false,
                time_start,
            );
        }
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
            isolation_work: self.isolation_work,
            events: BTreeSet::new(),
        };
        for &level in levels {
            candidates.level(input, level, context)?;
        }
        self.jump_boundaries(expr, candidates.events, context, false)
    }

    fn polar_corners(
        &mut self,
        expr: &Expr,
        y: &Expr,
        x: &Expr,
        context: &Context<'_>,
    ) -> Result<(), BehavioralBreakpointError> {
        let work = Cell::new(0);
        let mut candidates = EventSchedule {
            events: BTreeSet::new(),
            isolation_work: Some(self.isolation_work.unwrap_or(&work)),
            ..*self
        };
        for coordinate in [y, x] {
            candidates.level(coordinate, 0.0, context)?;
            // A numerically zero coordinate can still change its zero sign
            // through a product or quotient. Its internal clocks expose the
            // candidates for the VM's actual angular branch transition.
            candidates.expression(coordinate, context, true)?;
        }
        self.jump_boundaries(expr, candidates.events, context, true)
    }

    fn jump_boundaries(
        &mut self,
        expr: &Expr,
        roots: BTreeSet<u64>,
        context: &Context<'_>,
        angular: bool,
    ) -> Result<(), BehavioralBreakpointError> {
        let mut roots = roots.into_iter().map(Value::from_bits).peekable();
        if roots.peek().is_none() {
            return Ok(());
        }
        let program = compile(expr);
        if !program.node_map.is_empty() || !program.branch_map.is_empty() || program.sdt_count != 0
        {
            return Ok(());
        }
        let mut vm = Vm::new();
        let work = Cell::new(0);
        let budget = self.isolation_work.unwrap_or(&work);
        let mut evaluate = |time| -> Result<Value, BehavioralBreakpointError> {
            Self::charge_feature_work(budget, program.instructions.len())?;
            Ok(vm.execute(&program, &Context { time, ..*context }))
        };
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
                let left_value = evaluate(left)?;
                let right_value = evaluate(right)?;
                if !left_value.is_finite() || !right_value.is_finite() {
                    return Err(BehavioralBreakpointError::Invalid(
                        "a switching expression is non-finite",
                    ));
                }
                if angular && (left_value - right_value).abs() <= std::f64::consts::PI {
                    continue;
                }
                let project = |value: Value| {
                    if angular {
                        if value.is_sign_negative() { -1.0 } else { 1.0 }
                    } else {
                        value
                    }
                };
                let left_value = project(left_value);
                let right_value = project(right_value);
                if left_value == right_value {
                    continue;
                }
                while right.to_bits() - left.to_bits() > 1 {
                    self.poll()?;
                    let midpoint =
                        Value::from_bits(left.to_bits() + (right.to_bits() - left.to_bits()) / 2);
                    let value = evaluate(midpoint)?;
                    if !value.is_finite() {
                        return Err(BehavioralBreakpointError::Invalid(
                            "a switching expression is non-finite",
                        ));
                    }
                    if project(value) == left_value {
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
        self.charge_isolation_work(1)?;
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
                        _ => self.isolated_levels(expr, target, context)?,
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
                        _ => self.isolated_levels(expr, target, context)?,
                    }
                } else {
                    self.isolated_levels(expr, target, context)?;
                }
            }
            Expr::Function { func, args } => match (func, args.as_slice()) {
                (Function::Sin | Function::Cos | Function::Tan, [phase])
                    if *func == Function::Tan || (-1.0..=1.0).contains(&target) =>
                {
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
                (Function::Asin, [input]) if target.abs() <= std::f64::consts::FRAC_PI_2 => {
                    self.level(input, target.sin(), context)?;
                }
                (Function::Acos, [input]) if (0.0..=std::f64::consts::PI).contains(&target) => {
                    self.level(input, target.cos(), context)?;
                }
                (Function::Sinh, [input]) => self.level(input, target.asinh(), context)?,
                (Function::Asinh, [input]) => self.level(input, target.sinh(), context)?,
                (Function::Cosh, [input]) if target >= 1.0 => {
                    let root = target.acosh();
                    self.level(input, root, context)?;
                    if root != 0.0 {
                        self.level(input, -root, context)?;
                    }
                }
                (Function::Acosh, [input]) if target >= 0.0 => {
                    self.level(input, target.cosh(), context)?
                }
                (Function::Tanh, [input]) if target.abs() < 1.0 => {
                    self.level(input, target.atanh(), context)?
                }
                (Function::Atanh, [input]) => self.level(input, target.tanh(), context)?,
                (Function::Atan, [input]) if target.abs() < std::f64::consts::FRAC_PI_2 => {
                    self.level(input, target.tan(), context)?
                }
                (Function::Min | Function::Max, args) => {
                    // An extremum reaches a level only through one of its
                    // operands. Isolate those candidates directly: a cusp
                    // of the envelope need not have an exact VM zero or a
                    // sign change at adjacent representable timestamps.
                    let work = Cell::new(0);
                    let mut levels = EventSchedule {
                        events: BTreeSet::new(),
                        isolation_work: Some(self.isolation_work.unwrap_or(&work)),
                        ..*self
                    };
                    for arg in args {
                        levels.poll()?;
                        levels.charge_isolation_work(
                            compile_time_expression(arg, context).instructions.len(),
                        )?;
                        levels.level(arg, target, context)?;
                    }
                    for bits in levels.events {
                        self.add(Value::from_bits(bits))?;
                    }
                }
                _ => self.isolated_levels(expr, target, context)?,
            },
            _ => {}
        }
        Ok(())
    }

    fn extrema_corners(
        &mut self,
        args: &[Expr],
        context: &Context<'_>,
    ) -> Result<(), BehavioralBreakpointError> {
        if args.len() < 2 || args[1..].iter().all(|arg| arg == &args[0]) {
            return Ok(());
        }
        // Pairwise equality contains every possible change of active branch.
        // Bound this enumeration before cloning any operand trees.
        if args.len().saturating_mul(args.len() - 1) / 2 > 1_000_000 {
            return Err(BehavioralBreakpointError::Invalid(
                "extrema branch enumeration exceeds its 1000000-pair work limit",
            ));
        }
        let work = Cell::new(0);
        let mut corners = EventSchedule {
            events: BTreeSet::new(),
            isolation_work: Some(self.isolation_work.unwrap_or(&work)),
            ..*self
        };
        let mut operand_work = 0_usize;
        for arg in args {
            corners.poll()?;
            let cost = compile_time_expression(arg, context).instructions.len();
            corners.charge_isolation_work(cost)?;
            operand_work = operand_work.saturating_add(cost);
        }
        // Each operand participates in n-1 pairs. Reserve the tree-copy work
        // as well as charging actual root evaluations to the shared budget.
        corners.charge_isolation_work(operand_work.saturating_mul(args.len() - 1))?;
        for (index, left) in args.iter().enumerate() {
            for right in &args[index + 1..] {
                corners.poll()?;
                corners.charge_isolation_work(1)?;
                if left == right {
                    continue;
                }
                if let Some(value) = constant_value(right, context) {
                    corners.level(left, value, context)?;
                } else if let Some(value) = constant_value(left, context) {
                    corners.level(right, value, context)?;
                } else {
                    corners.level(
                        &Expr::Binary {
                            op: BinaryOp::Sub,
                            left: Box::new(left.clone()),
                            right: Box::new(right.clone()),
                        },
                        0.0,
                        context,
                    )?;
                }
            }
        }
        for bits in corners.events {
            self.add(Value::from_bits(bits))?;
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
                (Function::Pow | Function::Pwr | Function::Pwrs, [input, exponent])
                    if constant_value(exponent, context) == Some(0.0) =>
                {
                    self.discontinuities(expr, input, &[0.0], context)?;
                }
                (Function::Sin | Function::Cos, [_]) => {
                    for level in [-1.0, 0.0, 1.0] {
                        self.level(expr, level, context)?;
                    }
                }
                (Function::Tan, [phase]) => {
                    self.level(expr, 0.0, context)?;
                    if let Some((rate, offset)) = affine_time_coordinate(phase, context) {
                        self.trigonometric_levels(
                            Function::Tan,
                            rate,
                            offset,
                            Value::INFINITY,
                            0.0,
                        )?;
                    } else {
                        self.nonlinear_phase_levels(
                            Function::Tan,
                            phase,
                            Value::INFINITY,
                            context,
                        )?;
                    }
                }
                (Function::Asin | Function::Acos, [input]) => {
                    self.level(input, -1.0, context)?;
                    self.level(input, 1.0, context)?;
                }
                (Function::Atan2, [y, x]) => {
                    self.polar_corners(expr, y, x, context)?;
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
                (Function::Abs | Function::Sqr | Function::Sqrt | Function::Uramp, [input]) => {
                    self.level(input, 0.0, context)?
                }
                (Function::Ln | Function::Log10 | Function::Log, [input]) => {
                    self.level(input, crate::expr::LOGARITHM_MIN_ARGUMENT, context)?
                }
                (Function::Cosh, [input]) => self.level(input, 0.0, context)?,
                (Function::Acosh, [input]) => self.level(input, 1.0, context)?,
                (Function::Atanh, [input]) => {
                    let limit = if context.expression_dialect == ExpressionDialect::Xyce {
                        1.0 - crate::expr::XYCE_ATANH_EPSILON
                    } else {
                        1.0
                    };
                    self.level(input, -limit, context)?;
                    self.level(input, limit, context)?;
                }
                (Function::Tanh, [input])
                    if context.expression_dialect == ExpressionDialect::Xyce =>
                {
                    self.level(input, -crate::expr::XYCE_TANH_SATURATION_THRESHOLD, context)?;
                    self.level(input, crate::expr::XYCE_TANH_SATURATION_THRESHOLD, context)?;
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
                (Function::Min | Function::Max, args) => {
                    self.extrema_corners(args, context)?;
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
}

fn subdivide_time_domain(
    interval: TimeInterval,
    pending: &mut Vec<TimeInterval>,
) -> Result<(), BehavioralBreakpointError> {
    let midpoint = interval.lower + 0.5 * (interval.upper - interval.lower);
    if midpoint == interval.lower || midpoint == interval.upper {
        return Err(BehavioralBreakpointError::Invalid(
            "a time feature or its continuous domain cannot be isolated at the available time precision",
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
    Ok(())
}
