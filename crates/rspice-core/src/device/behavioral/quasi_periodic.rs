//! Lift authored clocks onto independent phase inputs. Physical node/branch
//! bindings keep their original coordinates; phase inputs are read-only and
//! never become MNA unknowns or physical Jacobian columns.

use super::*;
use crate::analysis::quasi_periodic::QuasiPeriodicGrid;
use crate::expr::{constant_value, function_uses_implicit_time};
use std::collections::HashMap;
use std::f64::consts::{PI, TAU};

mod clock;

struct Lift<'a> {
    grid: &'a QuasiPeriodicGrid,
    context: Context<'a>,
    phase_names: Vec<String>,
}

// Until a periodic function consumes it, preserve the exact symbolic form
// rate*time + remainder. In particular, do not fit a clock from sampled values.
struct Affine {
    rate: Value,
    remainder: Expr,
}

impl Affine {
    fn stationary(remainder: Expr) -> Self {
        Self {
            rate: 0.0,
            remainder,
        }
    }
    fn finish(self) -> Result<Expr, String> {
        if self.rate == 0.0 {
            Ok(self.remainder)
        } else {
            Err("nonperiodic explicit time cannot be represented on independent phases".into())
        }
    }
}

fn function(func: Function, args: Vec<Expr>) -> Expr {
    Expr::Function { func, args }
}
fn sine(arg: Expr) -> Expr {
    function(Function::Sin, vec![arg])
}
fn select_less(left: Expr, right: Value, yes: Expr, no: Expr) -> Expr {
    function(
        Function::If,
        vec![
            Expr::Binary {
                op: BinaryOp::Lt,
                left: Box::new(left),
                right: Box::new(Expr::Const(right)),
            },
            yes,
            no,
        ],
    )
}

fn scaled_rate(rate: Value, scale: Value, divide: bool) -> Result<Value, String> {
    let result = if divide { rate / scale } else { rate * scale };
    if !result.is_finite() || (result == 0.0 && rate != 0.0 && (divide || scale != 0.0)) {
        Err("behavioral clock slope exceeds the representable range".into())
    } else {
        Ok(result)
    }
}

impl Lift<'_> {
    fn phase(&self, frequency: Value) -> Result<Expr, String> {
        if frequency == 0.0 {
            return Ok(Expr::Const(0.0));
        }
        let tuple = self
            .grid
            .clock_tuple(frequency)
            .map_err(|e| e.to_string())?;
        Ok(tuple
            .iter()
            .zip(&self.phase_names)
            .filter(|(n, _)| **n != 0)
            .fold(Expr::Const(0.0), |sum, (&n, name)| {
                sum + Expr::Const(n as Value) * Expr::NodeVoltage(name.clone())
            }))
    }

    fn fixed(&self, expr: &Expr) -> Result<Value, String> {
        constant_value(expr, &self.context)
            .filter(|v| v.is_finite())
            .ok_or_else(|| {
                "clock frequency and waveform timing need finite constant parameters".into()
            })
    }

    fn implicit(&self, func: Function, args: Vec<Expr>) -> Result<Expr, String> {
        let arg = |i: usize| args.get(i).cloned().unwrap_or(Expr::Const(0.0));
        match func {
            Function::SpiceSin => {
                let frequency = self.fixed(&arg(2))?;
                let delay = self.fixed(&arg(3))?;
                if self.fixed(&arg(4))? != 0.0 {
                    return Err(
                        "damped behavioral SIN is not stationary on a quasiperiodic torus".into(),
                    );
                }
                // Like an independent QPSS SIN, delay is the steady-state
                // phase shift, not the initial transient before the source starts.
                let phase = self.phase(frequency)? - Expr::Const(TAU * frequency * delay)
                    + arg(5) * Expr::Const(PI / 180.0);
                Ok(arg(0) + arg(1) * sine(phase))
            }
            Function::SpiceSffm => {
                let carrier = self.phase(self.fixed(&arg(2))?)?;
                let modulation = self.phase(self.fixed(&arg(4))?)?;
                Ok(arg(0) + arg(1) * sine(carrier + arg(3) * sine(modulation)))
            }
            Function::SpicePulse => {
                let delay = self.fixed(&arg(2))?;
                let rise = self.fixed(&arg(3))?;
                let fall = self.fixed(&arg(4))?;
                let width = self.fixed(&arg(5))?;
                let period = self.fixed(&arg(6))?;
                if period <= 0.0
                    || [rise, fall, width].iter().any(|v| *v < 0.0)
                    || !(rise + fall + width).is_finite()
                {
                    return Err("behavioral PULSE needs a positive period and nonnegative finite edge/width durations".into());
                }
                let tuple = self
                    .grid
                    .clock_tuple(period.recip())
                    .map_err(|e| e.to_string())?;
                let interval = [rise, fall, width, period - rise - fall - width]
                    .into_iter()
                    .filter(|v| *v > 0.0)
                    .reduce(Value::min)
                    .unwrap_or(period);
                let resolution = tuple
                    .iter()
                    .zip(self.grid.dimensions())
                    .filter(|(n, _)| **n != 0)
                    .map(|(n, points)| *points as Value / n.unsigned_abs() as Value)
                    .fold(0.0, Value::max);
                if resolution < 2.0 * period / interval {
                    return Err(
                        "behavioral PULSE features need more per-tone collocation points".into(),
                    );
                }
                let cycles = self.phase(period.recip())? / Expr::Const(TAU)
                    - Expr::Const(delay.rem_euclid(period) / period);
                let elapsed = Expr::Const(period)
                    * (cycles.clone() - function(Function::Floor, vec![cycles]));
                let low = arg(0);
                let high = arg(1);
                let mut value = low.clone();
                if fall > 0.0 {
                    let ramp = high.clone()
                        + (low.clone() - high.clone())
                            * ((elapsed.clone() - Expr::Const(rise + width)) / Expr::Const(fall));
                    value = select_less(elapsed.clone(), rise + width + fall, ramp, value);
                }
                value = select_less(elapsed.clone(), rise + width, high.clone(), value);
                if rise > 0.0 {
                    let ramp = low.clone() + (high - low) * (elapsed.clone() / Expr::Const(rise));
                    value = select_less(elapsed, rise, ramp, value);
                }
                Ok(value)
            }
            _ => Err(
                "nonstationary behavioral waveform cannot be represented on independent phases"
                    .into(),
            ),
        }
    }

    fn lift(&self, expr: &Expr) -> Result<Affine, String> {
        let stationary = Affine::stationary;
        let result = match expr {
            Expr::Time => Affine {
                rate: 1.0,
                remainder: Expr::Const(0.0),
            },
            Expr::Unary {
                op: UnaryOp::Neg,
                operand,
            } => {
                let a = self.lift(operand)?;
                Affine {
                    rate: -a.rate,
                    remainder: -a.remainder,
                }
            }
            Expr::Unary { op, operand } => stationary(Expr::Unary {
                op: *op,
                operand: Box::new(self.lift(operand)?.finish()?),
            }),
            Expr::Binary { op, left, right } => {
                let a = self.lift(left)?;
                let b = self.lift(right)?;
                if *op == BinaryOp::Mod && a.rate != 0.0 {
                    return self.remainder(a, b.finish()?);
                }
                let rate = match op {
                    BinaryOp::Add => a.rate + b.rate,
                    BinaryOp::Sub => a.rate - b.rate,
                    BinaryOp::Mul if a.rate != 0.0 && b.rate == 0.0 => scaled_rate(a.rate, self.fixed(&b.remainder)?, false)?,
                    BinaryOp::Mul if b.rate != 0.0 && a.rate == 0.0 => scaled_rate(b.rate, self.fixed(&a.remainder)?, false)?,
                    BinaryOp::Div if b.rate == 0.0 && a.rate != 0.0 => scaled_rate(a.rate, self.fixed(&b.remainder)?, true)?,
                    _ if a.rate == 0.0 && b.rate == 0.0 => 0.0,
                    _ => return Err("behavioral clock is not affine in time; a chirp or state-dependent frequency has no fixed tone tuple".into()),
                };
                Affine {
                    rate,
                    remainder: Expr::Binary {
                        op: *op,
                        left: Box::new(a.remainder),
                        right: Box::new(b.remainder),
                    },
                }
            }
            Expr::Function {
                func: Function::Sin | Function::Cos | Function::Tan,
                args,
            } if args.len() == 1 => {
                let Expr::Function { func, .. } = expr else {
                    unreachable!()
                };
                let a = self.lift(&args[0])?;
                let phase = if a.rate == 0.0 {
                    a.remainder
                } else {
                    let period = if *func == Function::Tan { PI } else { TAU };
                    self.phase(scaled_rate(a.rate, period, true)?)? * Expr::Const(period / TAU)
                        + a.remainder
                };
                stationary(function(*func, vec![phase]))
            }
            Expr::Function {
                func: Function::Mod,
                args,
            } if args.len() == 2 => {
                let a = self.lift(&args[0])?;
                let divisor = self.lift(&args[1])?.finish()?;
                if a.rate == 0.0 {
                    stationary(function(Function::Mod, vec![a.remainder, divisor]))
                } else {
                    self.remainder(a, divisor)?
                }
            }
            Expr::Function {
                func: Function::Floor | Function::Ceil | Function::Trunc | Function::Round,
                args,
            } if args.len() == 1 => {
                let Expr::Function { func, .. } = expr else {
                    unreachable!()
                };
                self.rounded(*func, self.lift(&args[0])?)?
            }
            Expr::Function { func, args } => {
                let args = args
                    .iter()
                    .map(|a| self.lift(a)?.finish())
                    .collect::<Result<Vec<_>, _>>()?;
                if matches!(func, Function::Table | Function::Pwl) && args.len() >= 3 {
                    self.validate_authored_table(&args)?;
                }
                stationary(if function_uses_implicit_time(*func) {
                    self.implicit(*func, args)?
                } else {
                    function(*func, args)
                })
            }
            Expr::LookupTable { input, table } => {
                let input = self.lift(input)?.finish()?;
                self.validate_table_interval(
                    &input,
                    crate::numerics::minimum_pwl_interval(table.points.iter().copied()),
                )?;
                stationary(Expr::LookupTable {
                    input: Box::new(input),
                    table: table.clone(),
                })
            }
            _ => stationary(expr.clone()),
        };
        if !result.rate.is_finite() {
            return Err("behavioral clock slope is not finite".into());
        }
        Ok(result)
    }
}

// Apply identical binding rules to voltage and current sources without
// converting ground into a phase input or renumbering physical references.
macro_rules! lift_source {
    ($kind:ty) => {
        impl $kind {
            pub(crate) fn lift_quasi_periodic(
                &mut self,
                grid: &QuasiPeriodicGrid,
                unknowns: usize,
            ) -> Result<(), String> {
                if self.is_frequency_dependent() {
                    return Err(
                        "behavioral live-frequency equations need a quasiperiodic frequency model"
                            .into(),
                    );
                }
                self.validate_periodic_integral_rates()?;
                let mut names = Vec::new();
                for dimension in 0..grid.dimensions().len() {
                    let mut name = format!("\0qpss_phase_{dimension}");
                    while self.program.node_map.contains_key(&name) {
                        name.push('_');
                    }
                    names.push(name);
                }
                if let Some(equations) = &self.integral_equations {
                    let lift = Lift {
                        grid,
                        context: self.periodicity_context(),
                        phase_names: names.clone(),
                    };
                    let equations =
                        equations.lifted_phases(&names, |ast| lift.lift(ast)?.finish())?;
                    self.integral_equations = Some(equations);
                    // The original compiler's occurrence order and physical
                    // bindings remain authoritative. Only the detached F/Q
                    // rate/output programs read the appended phase scalars.
                    return Ok(());
                }
                let ast = Lift {
                    grid,
                    context: self.periodicity_context(),
                    phase_names: names.clone(),
                }
                .lift(&self.ast)?
                .finish()?;
                let nodes: HashMap<_, _> = self
                    .program
                    .node_map
                    .iter()
                    .map(|(name, &index)| (name.clone(), self.node_bindings[index]))
                    .collect();
                let branches: HashMap<_, _> = self
                    .program
                    .branch_map
                    .iter()
                    .map(|(name, &index)| (name.clone(), self.branch_bindings[index]))
                    .collect();
                self.ast = ast;
                self.program = compile(&self.ast);
                self.vm = Vm::new();
                self.bind_references(
                    |name| {
                        names
                            .iter()
                            .position(|phase| phase == name)
                            .map(|i| unknowns + i + 1)
                            .or_else(|| nodes.get(name).map(|binding| binding.map_or(0, |i| i + 1)))
                    },
                    |name| {
                        branches.get(name).copied().flatten().map_or(
                            BehavioralBranchResolution::MissingDevice,
                            BehavioralBranchResolution::Branch,
                        )
                    },
                )
                .map_err(|e| e.to_string())?;
                Ok(())
            }

            pub(crate) fn has_quasi_periodic_equation(&self, dimensions: usize) -> bool {
                !self.is_frequency_dependent()
                    && self.integral_equations.as_ref().map_or_else(
                        || self.has_memoryless_periodic_equation(),
                        |equations| equations.has_phase_basis(dimensions),
                    )
            }
        }
    };
}
lift_source!(BehavioralVoltageSource);
lift_source!(BehavioralCurrentSource);
