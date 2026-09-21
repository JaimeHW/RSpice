//! Stationary signed clock resets and table resolution on the authored torus.
use super::*;

impl Lift<'_> {
    pub(super) fn remainder(&self, input: Affine, divisor: Expr) -> Result<Affine, String> {
        let modulus = self.fixed(&divisor)?.abs();
        if modulus == 0.0 {
            return Err("behavioral clock remainder requires a nonzero modulus".into());
        }
        let frequency = scaled_rate(input.rate, modulus, true)?;
        // Reduce a fixed offset before dividing: a large finite delay must
        // not overflow, and integer cycles must not erase the phase bits.
        let offset = match constant_value(&input.remainder, &self.context) {
            Some(value) => Expr::Const((value % modulus) / modulus),
            None => input.remainder / Expr::Const(modulus),
        };
        let cycles = self.phase(frequency)? / Expr::Const(TAU) + offset;
        // SPICE remainder has the dividend's sign, independent of divisor
        // sign. Once startup has passed, an affine clock's sign is its rate's
        // sign. Floor/ceil preserve that stationary extension, including zero
        // at a reset. Using Euclidean remainder for both would invert a
        // negative clock's waveform.
        let rounding = if input.rate > 0.0 {
            Function::Floor
        } else {
            Function::Ceil
        };
        Ok(Affine::stationary(
            Expr::Const(modulus) * (cycles.clone() - function(rounding, vec![cycles])),
        ))
    }

    pub(super) fn rounded(&self, func: Function, input: Affine) -> Result<Affine, String> {
        if input.rate == 0.0 {
            return Ok(Affine::stationary(function(func, vec![input.remainder])));
        }
        let rounding = match func {
            Function::Trunc if input.rate > 0.0 => Function::Floor,
            Function::Trunc => Function::Ceil,
            _ => func,
        };
        // Ties-to-even is covariant under an even integer shift, not under
        // an odd one. Retain the half-frequency clock instead of silently
        // changing tie values at alternate periods.
        let cycles = if rounding == Function::Round {
            2.0
        } else {
            1.0
        };
        let phase = self.phase(scaled_rate(input.rate, cycles, true)?)? / Expr::Const(TAU)
            * Expr::Const(cycles);
        Ok(Affine {
            rate: input.rate,
            remainder: function(rounding, vec![phase.clone() + input.remainder]) - phase,
        })
    }

    /// Bounds on explicit phase dependence between clock resets, in input
    /// units per radian, holding circuit controls fixed. Unknown V/I scaling
    /// cannot establish a bound before solving the nonlinear circuit.
    fn phase_rates(&self, expr: &Expr) -> Option<Vec<Value>> {
        let zero = || vec![0.0; self.phase_names.len()];
        if constant_value(expr, &self.context).is_some() {
            return Some(zero());
        }
        let scale = |values: Vec<Value>, scale: Value| {
            values
                .into_iter()
                .map(|v| v * scale.abs())
                .collect::<Vec<_>>()
        };
        match expr {
            Expr::NodeVoltage(name) => {
                let mut rates = zero();
                if let Some(dimension) = self.phase_names.iter().position(|phase| phase == name) {
                    rates[dimension] = 1.0;
                }
                Some(rates)
            }
            Expr::BranchCurrent(_) => Some(zero()),
            Expr::Unary {
                op: UnaryOp::Neg,
                operand,
            } => self.phase_rates(operand),
            Expr::Binary { op, left, right } => match op {
                BinaryOp::Add | BinaryOp::Sub => Some(
                    self.phase_rates(left)?
                        .into_iter()
                        .zip(self.phase_rates(right)?)
                        .map(|(a, b)| a + b)
                        .collect(),
                ),
                BinaryOp::Mul => {
                    if let Some(value) = constant_value(left, &self.context) {
                        Some(scale(self.phase_rates(right)?, value))
                    } else {
                        Some(scale(
                            self.phase_rates(left)?,
                            constant_value(right, &self.context)?,
                        ))
                    }
                }
                BinaryOp::Div => {
                    let divisor = constant_value(right, &self.context)?;
                    if divisor == 0.0 {
                        None
                    } else {
                        Some(
                            self.phase_rates(left)?
                                .into_iter()
                                .map(|v| v / divisor.abs())
                                .collect(),
                        )
                    }
                }
                _ => None,
            },
            Expr::Function { func, args } if args.len() == 1 => match func {
                Function::Sin | Function::Cos | Function::Abs => self.phase_rates(&args[0]),
                Function::Floor | Function::Ceil | Function::Trunc | Function::Round => {
                    self.phase_rates(&args[0])?;
                    Some(zero())
                }
                _ => None,
            },
            _ => None,
        }
    }

    pub(super) fn validate_table_interval(
        &self,
        input: &Expr,
        interval: Option<Value>,
    ) -> Result<(), String> {
        let Some(interval) = interval else {
            return Ok(());
        };
        let Some(rates) = self.phase_rates(input) else {
            return Ok(());
        };
        for (tone, (rate, points)) in rates.into_iter().zip(self.grid.dimensions()).enumerate() {
            let required = 2.0 * TAU * rate / interval;
            if !required.is_finite() || required > *points as Value * (1.0 + 32.0 * Value::EPSILON)
            {
                return Err(format!(
                    "behavioral table features require at least {:.0} collocation points on tone {}; configured {}",
                    required.ceil(),
                    tone + 1,
                    points
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_authored_table(&self, args: &[Expr]) -> Result<(), String> {
        let knots = args[1..]
            .chunks_exact(2)
            .map(|pair| constant_value(&pair[0], &self.context))
            .collect::<Option<Vec<_>>>();
        let Some(knots) = knots else { return Ok(()) };
        let values = args[1..]
            .chunks_exact(2)
            .map(|pair| constant_value(&pair[1], &self.context))
            .collect::<Option<Vec<_>>>();
        let interval = match values {
            Some(values) => crate::numerics::minimum_pwl_interval(knots.into_iter().zip(values)),
            // State-dependent amplitudes can make every knot significant.
            None => knots
                .windows(2)
                .map(|p| (p[1] - p[0]).abs())
                .filter(|v| *v > 0.0 && v.is_finite())
                .reduce(Value::min),
        };
        self.validate_table_interval(&args[0], interval)
    }
}
