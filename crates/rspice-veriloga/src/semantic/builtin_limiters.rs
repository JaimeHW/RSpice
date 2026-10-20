//! The built-in `$limit` selectors.
//!
//! `$limit(V(a, c), "pnjlim", vt, vcrit)` names a limiting algorithm the
//! simulator supplies (Verilog-AMS LRM 2.4 section 9.17.3). These are the
//! SPICE3 junction and FET limiters as ngspice ships them in
//! `spicelib/devices/devsup.c` and hands to OSDI models: `pnjlim` is
//! `DEVpnjlim`, `fetlim` is `DEVfetlim`, `limvds` is `DEVlimvds` and
//! `limitlog` is `DEVlimitlog` (whose one-time reset of a NaN temperature to
//! zero, a process-wide `static` in ngspice, is not reproduced: a NaN stays
//! NaN). Xyce's names are accepted too: `pnjlim_new` is the same `DEVpnjlim`,
//! `dummy` leaves the proposal alone, and each `typed` form takes a trailing
//! type/polarity that orients the probe the way a typed source-defined
//! limiter does.
//!
//! Each body is written here as the expression the C function computes, over
//! the limiter's implicit proposed and previous values, and lowered exactly as
//! a source-defined limiter function's inlined body is. Every route therefore
//! runs it through the one named-limiter operator they already share.

use super::*;

/// The proposed value (`vnew`), previous iterate (`vold`) and forwarded
/// arguments, bound in a substitution frame. The `.` keeps the names out of
/// reach of any source identifier.
const VNEW: &str = "$limit.vnew";
const VOLD: &str = "$limit.vold";
const ARGUMENTS: [&str; 2] = ["$limit.a0", "$limit.a1"];

#[derive(Clone, Copy)]
enum Algorithm {
    PnJunction,
    Fet,
    DrainSource,
    Logarithmic,
    Identity,
}

/// A built-in selector: its algorithm, the arguments it forwards after the
/// selector and whether a trailing type/polarity follows them.
#[derive(Clone, Copy)]
pub(super) struct BuiltinLimiter {
    algorithm: Algorithm,
    forwarded: usize,
    typed: bool,
}

impl BuiltinLimiter {
    pub(super) fn named(selector: &str) -> Option<Self> {
        let (algorithm, forwarded, typed) = match selector {
            "pnjlim" | "pnjlim_new" => (Algorithm::PnJunction, 2, false),
            "typedpnjlim" | "typedpnjlim_new" => (Algorithm::PnJunction, 2, true),
            "fetlim" => (Algorithm::Fet, 1, false),
            "limvds" => (Algorithm::DrainSource, 0, false),
            "limitlog" => (Algorithm::Logarithmic, 1, false),
            "dummy" => (Algorithm::Identity, 2, false),
            "typeddummy" => (Algorithm::Identity, 2, true),
            _ => return None,
        };
        Some(Self {
            algorithm,
            forwarded,
            typed,
        })
    }

    /// The whole call's argument count: the probe, the selector, the
    /// forwarded arguments and a typed form's polarity.
    pub(super) fn arity(self) -> usize {
        2 + self.forwarded + usize::from(self.typed)
    }
}

/// The selectors [`BuiltinLimiter::named`] knows, for a diagnostic.
pub(super) const BUILTIN_LIMITER_NAMES: &str = "pnjlim, pnjlim_new, typedpnjlim, typedpnjlim_new, fetlim, limvds, limitlog, dummy, \
     typeddummy";

impl SemanticAnalyzer {
    /// Lower `$limit(probe, "<built-in>", args...)` to the named-limiter
    /// operator whose candidate is the built-in algorithm's body.
    pub(super) fn lower_builtin_limit_call(
        &mut self,
        function: &SystemFunction,
        selector: &SmolStr,
        limiter: BuiltinLimiter,
    ) -> CompileResult<Expression> {
        let span = function.span;
        let proposed = self.lower_expression(&function.args[0])?;
        let type_metadata = if limiter.typed {
            Some(Box::new(
                self.lower_expression(&function.args[2 + limiter.forwarded])?,
            ))
        } else {
            None
        };
        let implicit = |argument| {
            Expression::AnalogOperator(AnalogOperator::LimiterArgument { argument, span })
        };
        let mut frame = HashMap::new();
        frame.insert(
            SmolStr::new(VNEW),
            self.lower_expression(&implicit(LimiterArgument::Proposed))?,
        );
        frame.insert(
            SmolStr::new(VOLD),
            self.lower_expression(&implicit(LimiterArgument::Previous))?,
        );
        for (name, argument) in ARGUMENTS
            .iter()
            .zip(&function.args[2..2 + limiter.forwarded])
        {
            let value = self.lower_expression(argument)?;
            let value = self.coerce_assignment_expression(value, ValueType::Real)?.0;
            frame.insert(SmolStr::new(name), value);
        }
        let body = Body { span }.algorithm(limiter.algorithm);
        self.subst_stack.push(frame);
        let candidate = self.lower_expression(&body);
        self.subst_stack.pop();
        Ok(Expression::AnalogOperator(AnalogOperator::Limit {
            proposed: Box::new(proposed),
            candidate: Box::new(candidate?),
            type_metadata,
            selector: selector.clone(),
            span,
        }))
    }
}

/// Builds a limiter body as source-level expressions at one span.
struct Body {
    span: Span,
}

impl Body {
    fn algorithm(&self, algorithm: Algorithm) -> Expression {
        match algorithm {
            Algorithm::PnJunction => self.pnjlim(),
            Algorithm::Fet => self.fetlim(),
            Algorithm::DrainSource => self.limvds(),
            Algorithm::Logarithmic => self.limitlog(),
            Algorithm::Identity => self.name(VNEW),
        }
    }

    /// `DEVpnjlim(vnew, vold, vt, vcrit)`.
    fn pnjlim(&self) -> Expression {
        let (vnew, vold) = (|| self.name(VNEW), || self.name(VOLD));
        let (vt, vcrit) = (|| self.name(ARGUMENTS[0]), || self.name(ARGUMENTS[1]));
        let arg = || self.div(self.sub(vnew(), vold()), vt());
        // `arg > 0` with |vnew - vold| > 2 vt makes arg - 2 positive, `arg <=
        // 0` makes 2 - arg positive, and vnew > vcrit makes vnew / vt
        // positive for any physical vcrit; the floor only keeps a branch the
        // condition rejects finite on a route that evaluates both.
        let rising = self.add(
            vold(),
            self.mul(
                vt(),
                self.add(self.num(2.0), self.ln(self.sub(arg(), self.num(2.0)))),
            ),
        );
        let falling = self.sub(
            vold(),
            self.mul(
                vt(),
                self.add(self.num(2.0), self.ln(self.sub(self.num(2.0), arg()))),
            ),
        );
        let from_cutoff = self.mul(vt(), self.ln(self.div(vnew(), vt())));
        let large_step = self.cond(
            self.gt(vold(), self.num(0.0)),
            self.cond(self.gt(arg(), self.num(0.0)), rising, falling),
            from_cutoff,
        );
        let floor = self.cond(
            self.gt(vold(), self.num(0.0)),
            self.sub(self.mul(self.num(-1.0), vold()), self.num(1.0)),
            self.sub(self.mul(self.num(2.0), vold()), self.num(1.0)),
        );
        let reverse = self.cond(
            self.lt(vnew(), self.num(0.0)),
            self.cond(self.lt(vnew(), floor.clone()), floor, vnew()),
            vnew(),
        );
        let limited = self.binary(
            BinaryOp::And,
            self.gt(vnew(), vcrit()),
            self.gt(
                self.call("abs", vec![self.sub(vnew(), vold())]),
                self.add(vt(), vt()),
            ),
        );
        self.cond(limited, large_step, reverse)
    }

    /// `DEVfetlim(vnew, vold, vto)`.
    fn fetlim(&self) -> Expression {
        let (vnew, vold, vto) = (
            || self.name(VNEW),
            || self.name(VOLD),
            || self.name(ARGUMENTS[0]),
        );
        let vtsthi = || {
            self.add(
                self.call(
                    "abs",
                    vec![self.mul(self.num(2.0), self.sub(vold(), vto()))],
                ),
                self.num(2.0),
            )
        };
        let vtstlo = || {
            self.add(
                self.call("abs", vec![self.sub(vold(), vto())]),
                self.num(1.0),
            )
        };
        let vtox = || self.add(vto(), self.num(3.5));
        let delv = || self.sub(vnew(), vold());
        let falling = || self.le(delv(), self.num(0.0));
        let drop = || self.mul(self.num(-1.0), delv());

        let on_falling = self.cond(
            self.ge(vnew(), vtox()),
            self.cond(
                self.gt(drop(), vtstlo()),
                self.sub(vold(), vtstlo()),
                vnew(),
            ),
            self.max(vnew(), self.add(vto(), self.num(2.0))),
        );
        let on_rising = self.cond(
            self.ge(delv(), vtsthi()),
            self.add(vold(), vtsthi()),
            vnew(),
        );
        let on = self.cond(falling(), on_falling, on_rising);
        let middle = self.cond(
            falling(),
            self.max(vnew(), self.sub(vto(), self.num(0.5))),
            self.min(vnew(), self.add(vto(), self.num(4.0))),
        );
        let vtemp = || self.add(vto(), self.num(0.5));
        let off = self.cond(
            falling(),
            self.cond(
                self.gt(drop(), vtsthi()),
                self.sub(vold(), vtsthi()),
                vnew(),
            ),
            self.cond(
                self.le(vnew(), vtemp()),
                self.cond(
                    self.gt(delv(), vtstlo()),
                    self.add(vold(), vtstlo()),
                    vnew(),
                ),
                vtemp(),
            ),
        );
        self.cond(
            self.ge(vold(), vto()),
            self.cond(self.ge(vold(), vtox()), on, middle),
            off,
        )
    }

    /// `DEVlimvds(vnew, vold)`.
    fn limvds(&self) -> Expression {
        let (vnew, vold) = (|| self.name(VNEW), || self.name(VOLD));
        let saturated = self.cond(
            self.gt(vnew(), vold()),
            self.min(
                vnew(),
                self.add(self.mul(self.num(3.0), vold()), self.num(2.0)),
            ),
            self.cond(
                self.lt(vnew(), self.num(3.5)),
                self.max(vnew(), self.num(2.0)),
                vnew(),
            ),
        );
        let linear = self.cond(
            self.gt(vnew(), vold()),
            self.min(vnew(), self.num(4.0)),
            self.max(vnew(), self.num(-0.5)),
        );
        self.cond(self.ge(vold(), self.num(3.5)), saturated, linear)
    }

    /// `DEVlimitlog(deltemp, deltemp_old, LIM_TOL)`: past `LIM_TOL` the step
    /// grows by `log10` of its ratio to the tolerance. Either log argument
    /// exceeds one on the branch that reads it.
    fn limitlog(&self) -> Expression {
        let (new, old, tolerance) = (
            || self.name(VNEW),
            || self.name(VOLD),
            || self.name(ARGUMENTS[0]),
        );
        let ratio = |step: Expression| self.logarithm("log10", self.div(step, tolerance()));
        let rising = self.add(self.add(old(), tolerance()), ratio(self.sub(new(), old())));
        let falling = self.sub(self.sub(old(), tolerance()), ratio(self.sub(old(), new())));
        self.cond(
            self.gt(new(), self.add(old(), tolerance())),
            rising,
            self.cond(self.lt(new(), self.sub(old(), tolerance())), falling, new()),
        )
    }

    fn name(&self, name: &str) -> Expression {
        Expression::Identifier(Identifier {
            name: SmolStr::new(name),
            span: self.span,
        })
    }

    fn num(&self, value: f64) -> Expression {
        SemanticAnalyzer::number_expr(value, self.span)
    }

    fn binary(&self, op: BinaryOp, left: Expression, right: Expression) -> Expression {
        Expression::Binary(BinaryExpr {
            op,
            left: Box::new(left),
            right: Box::new(right),
            span: self.span,
        })
    }

    fn add(&self, left: Expression, right: Expression) -> Expression {
        self.binary(BinaryOp::Add, left, right)
    }

    fn sub(&self, left: Expression, right: Expression) -> Expression {
        self.binary(BinaryOp::Sub, left, right)
    }

    fn mul(&self, left: Expression, right: Expression) -> Expression {
        self.binary(BinaryOp::Mul, left, right)
    }

    fn div(&self, left: Expression, right: Expression) -> Expression {
        self.binary(BinaryOp::Div, left, right)
    }

    fn gt(&self, left: Expression, right: Expression) -> Expression {
        self.binary(BinaryOp::Gt, left, right)
    }

    fn ge(&self, left: Expression, right: Expression) -> Expression {
        self.binary(BinaryOp::Ge, left, right)
    }

    fn lt(&self, left: Expression, right: Expression) -> Expression {
        self.binary(BinaryOp::Lt, left, right)
    }

    fn le(&self, left: Expression, right: Expression) -> Expression {
        self.binary(BinaryOp::Le, left, right)
    }

    fn cond(&self, condition: Expression, then: Expression, otherwise: Expression) -> Expression {
        Expression::Conditional(ConditionalExpr {
            condition: Box::new(condition),
            then_expr: Box::new(then),
            else_expr: Box::new(otherwise),
            span: self.span,
        })
    }

    /// ngspice's `MAX(a, b)`, `(a) > (b) ? (a) : (b)`.
    fn max(&self, a: Expression, b: Expression) -> Expression {
        self.cond(self.gt(a.clone(), b.clone()), a, b)
    }

    /// ngspice's `MIN(a, b)`, `(a) < (b) ? (a) : (b)`.
    fn min(&self, a: Expression, b: Expression) -> Expression {
        self.cond(self.lt(a.clone(), b.clone()), a, b)
    }

    fn call(&self, name: &str, args: Vec<Expression>) -> Expression {
        Expression::Call(CallExpr {
            resolved_builtin: true,
            name: SmolStr::new(name),
            args,
            span: self.span,
        })
    }

    /// `ln` of an argument floored at the smallest normal double, which the
    /// branch that reads it never reaches (see [`Body::pnjlim`]).
    fn ln(&self, argument: Expression) -> Expression {
        self.logarithm("ln", argument)
    }

    fn logarithm(&self, name: &str, argument: Expression) -> Expression {
        let floor = self.num(f64::MIN_POSITIVE);
        let floored = self.cond(self.gt(argument.clone(), floor.clone()), argument, floor);
        self.call(name, vec![floored])
    }
}
