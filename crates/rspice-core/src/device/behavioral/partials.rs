//! Every analytic input partial of one expression, in one pass.
//!
//! A linearization needs the expression's slope against each voltage and
//! current it reads. [`super::analytic_expression_partial`] answers for one
//! input at a time: a walk of the whole tree per input, recomputing every
//! value on the way, and a string-keyed lookup at every reference. This
//! answers for all of them at once — one pass over the tree, flattened to
//! postfix with each reference resolved to its local index, carrying one
//! derivative per input beside the one value.
//!
//! Each input's derivative is computed exactly as the per-input walk computes
//! it. A lane holds the derivative that walk would hold at the same node, and
//! every operation applies the same function to the same operands:
//! [`super::eval_binary_with_derivative`] for an operator, and the recursive
//! evaluator itself, with that input as its target, for any subtree this pass
//! does not flatten (a function call, a table, the analysis variables). The
//! value is shared because it does not depend on the target: none of those
//! functions reads a derivative to produce a value. A lane the walk would have
//! abandoned with `None` is abandoned at the same node.

use super::{
    BehavioralDerivativeContext, BehavioralEnvironment, BinaryOp, CompiledExpr, Derivative,
    DerivativeTarget, Expr, Function, UnaryOp, Value, eval_behavioral_expr_with_derivative,
    eval_binary_with_derivative, normalize_expression_boundary,
};
use std::sync::Arc;

/// The expression in postfix, references resolved.
#[derive(Debug)]
struct Program {
    /// The tree this was flattened from, kept to recognize it: a source whose
    /// expression is rewritten gets a new tree, and this is rebuilt for it.
    source: Arc<Expr>,
    /// The compiled expression whose maps resolved the references.
    compiled: Arc<CompiledExpr>,
    ops: Vec<Op>,
}

#[derive(Debug)]
enum Op {
    Const(Value),
    Node(usize),
    Branch(usize),
    Unary(UnaryOp),
    Binary(BinaryOp),
    /// `if(condition, then, otherwise)`, its condition already on the stack.
    ///
    /// The per-input walk evaluates the condition, abandons the input if the
    /// condition was abandoned, and then evaluates exactly one branch, chosen
    /// by the condition's value — which no input changes. So does this.
    Select {
        then: Vec<Op>,
        otherwise: Vec<Op>,
    },
    /// Any other subtree, evaluated by the recursive evaluator once per lane.
    Subtree(Box<Expr>),
}

impl Program {
    /// Flatten `source`, or `None` when nothing would be flattened — the
    /// whole tree a single subtree the per-input walk evaluates anyway.
    fn compile(source: &Arc<Expr>, compiled: &Arc<CompiledExpr>) -> Option<Self> {
        let mut ops = Vec::new();
        emit(source, compiled, &mut ops);
        if matches!(ops.as_slice(), [Op::Subtree(_)]) {
            return None;
        }
        Some(Self {
            source: Arc::clone(source),
            compiled: Arc::clone(compiled),
            ops,
        })
    }

    fn compiled_from(&self, source: &Arc<Expr>, compiled: &Arc<CompiledExpr>) -> bool {
        Arc::ptr_eq(&self.source, source) && Arc::ptr_eq(&self.compiled, compiled)
    }
}

fn emit(expr: &Expr, compiled: &CompiledExpr, ops: &mut Vec<Op>) {
    match expr {
        Expr::Const(value) => ops.push(Op::Const(*value)),
        Expr::NodeVoltage(name) => ops.push(match compiled.node_map.get(name) {
            Some(&index) => Op::Node(index),
            None => Op::Subtree(Box::new(expr.clone())),
        }),
        Expr::BranchCurrent(name) => ops.push(match compiled.branch_map.get(name) {
            Some(&index) => Op::Branch(index),
            None => Op::Subtree(Box::new(expr.clone())),
        }),
        Expr::Unary { op, operand } => {
            emit(operand, compiled, ops);
            ops.push(Op::Unary(*op));
        }
        Expr::Binary { op, left, right } => {
            emit(left, compiled, ops);
            emit(right, compiled, ops);
            ops.push(Op::Binary(*op));
        }
        Expr::Function {
            func: Function::If,
            args,
        } if args.len() == 3 => {
            emit(&args[0], compiled, ops);
            let mut then = Vec::new();
            emit(&args[1], compiled, &mut then);
            let mut otherwise = Vec::new();
            emit(&args[2], compiled, &mut otherwise);
            ops.push(Op::Select { then, otherwise });
        }
        _ => ops.push(Op::Subtree(Box::new(expr.clone()))),
    }
}

/// The most inputs one pass answers for. A source reading more keeps the
/// per-input walk, which is no slower than it was.
const MAX_LANES: usize = 16;

/// The stacks one pass runs on.
///
/// Storage, never state: every pass empties them before it reads them. Held
/// per thread rather than per source because the transient's rollback copies
/// sources wholesale, and a copy that brought no stacks of its own would ask
/// the allocator for them at its next linearization.
#[derive(Default)]
struct Stacks {
    /// One value per stack slot.
    values: Vec<Value>,
    /// `lanes` derivatives per stack slot; `None` is a lane the per-input walk
    /// would have abandoned.
    derivatives: Vec<Option<Derivative>>,
}

thread_local! {
    static STACKS: std::cell::RefCell<Stacks> = std::cell::RefCell::new(Stacks::default());
}

/// One behavioral source's single-pass partials: the flattened expression and
/// the last pass's answers.
#[derive(Clone, Debug, Default)]
pub(super) struct AnalyticPartials {
    program: Option<Arc<Program>>,
    /// The finished partial of each lane: node inputs first, then branch
    /// currents.
    partials: [Option<Value>; MAX_LANES],
    /// How many node lanes `partials` holds; the branch lanes follow.
    node_lanes: usize,
    /// How many lanes `partials` holds.
    lanes: usize,
    /// Whether `partials` holds this linearization's lanes.
    filled: bool,
}

impl AnalyticPartials {
    /// Compute every lane's partial at the inputs given. Afterwards
    /// [`Self::node`] and [`Self::branch`] answer for each input, or `None`
    /// when they have no answer and the caller must ask the per-input walk.
    pub(super) fn evaluate(
        &mut self,
        source: &Arc<Expr>,
        compiled: &Arc<CompiledExpr>,
        node_values: &[Value],
        branch_values: &[Value],
        environment: BehavioralEnvironment,
    ) {
        self.filled = false;
        let node_lanes = node_values.len();
        let lanes = node_lanes + branch_values.len();
        if lanes > MAX_LANES {
            return;
        }
        if !self
            .program
            .as_ref()
            .is_some_and(|program| program.compiled_from(source, compiled))
        {
            self.program = Program::compile(source, compiled).map(Arc::new);
        }
        let Some(program) = self.program.as_deref() else {
            return;
        };
        let partials = &mut self.partials[..lanes];
        let ran = STACKS.with(|stacks| {
            let Ok(mut stacks) = stacks.try_borrow_mut() else {
                return None;
            };
            let Stacks {
                values,
                derivatives,
            } = &mut *stacks;
            values.clear();
            derivatives.clear();
            let pass = Pass {
                compiled: &program.compiled,
                node_values,
                branch_values,
                environment,
                lanes,
            };
            if !(pass.run(&program.ops, values, derivatives) && values.len() == 1) {
                partials.fill(None);
                return Some(());
            }
            let dialect = environment.expression_dialect;
            let top = &derivatives[derivatives.len() - lanes..];
            for (partial, derivative) in partials.iter_mut().zip(top) {
                *partial = derivative.and_then(|derivative| {
                    let derivative = normalize_expression_boundary(derivative.binary64(), dialect);
                    derivative.is_finite().then_some(derivative)
                });
            }
            Some(())
        });
        self.node_lanes = node_lanes;
        self.lanes = lanes;
        self.filled = ran.is_some();
    }

    /// The partial against node input `index`: `Some(answer)` when this pass
    /// computed it, `None` when the caller must take the per-input walk.
    pub(super) fn node(&self, index: usize) -> Option<Option<Value>> {
        (self.filled && index < self.node_lanes).then(|| self.partials[index])
    }

    /// [`Self::node`] for branch-current input `index`.
    pub(super) fn branch(&self, index: usize) -> Option<Option<Value>> {
        let lane = self.node_lanes + index;
        (self.filled && lane < self.lanes).then(|| self.partials[lane])
    }
}

/// One pass's inputs.
struct Pass<'a> {
    compiled: &'a CompiledExpr,
    node_values: &'a [Value],
    branch_values: &'a [Value],
    environment: BehavioralEnvironment,
    lanes: usize,
}

impl Pass<'_> {
    /// Run `ops` onto the stacks. `false` when every lane was abandoned — or a
    /// reference fell outside the inputs, which the per-input walk also
    /// abandons.
    fn run(
        &self,
        ops: &[Op],
        values: &mut Vec<Value>,
        derivatives: &mut Vec<Option<Derivative>>,
    ) -> bool {
        let Self {
            compiled,
            node_values,
            branch_values,
            environment,
            lanes,
        } = *self;
        let node_lanes = node_values.len();
        let dialect = environment.expression_dialect;
        for op in ops {
            match op {
                Op::Const(value) => {
                    values.push(*value);
                    derivatives.extend((0..lanes).map(|_| Some(Derivative::from(0.0))));
                }
                Op::Node(index) => {
                    let Some(value) = node_values.get(*index) else {
                        return false;
                    };
                    values.push(*value);
                    derivatives.extend((0..lanes).map(|lane| {
                        Some(Derivative::from(if lane == *index { 1.0 } else { 0.0 }))
                    }));
                }
                Op::Branch(index) => {
                    let Some(value) = branch_values.get(*index) else {
                        return false;
                    };
                    values.push(*value);
                    derivatives.extend((0..lanes).map(|lane| {
                        Some(Derivative::from(if lane == node_lanes + *index {
                            1.0
                        } else {
                            0.0
                        }))
                    }));
                }
                Op::Unary(op) => {
                    let slot = values.len() - 1;
                    let operand = values[slot];
                    values[slot] = match op {
                        UnaryOp::Neg => -operand,
                        UnaryOp::Not => {
                            if operand == 0.0 {
                                1.0
                            } else {
                                0.0
                            }
                        }
                    };
                    for derivative in derivatives[slot * lanes..].iter_mut().flatten() {
                        *derivative = match op {
                            UnaryOp::Neg => -*derivative,
                            UnaryOp::Not => Derivative::from(0.0),
                        };
                    }
                }
                Op::Binary(op) => {
                    let right = values.pop().expect("a binary operator has two operands");
                    let slot = values.len() - 1;
                    let left = values[slot];
                    let (lower, upper) = derivatives.split_at_mut((slot + 1) * lanes);
                    let mut value = None;
                    for (left_derivative, right_derivative) in
                        lower[slot * lanes..].iter_mut().zip(upper.iter())
                    {
                        let (Some(d_left), Some(d_right)) = (*left_derivative, *right_derivative)
                        else {
                            *left_derivative = None;
                            continue;
                        };
                        *left_derivative =
                            eval_binary_with_derivative(*op, left, d_left, right, d_right, dialect)
                                .map(|(result, derivative)| {
                                    debug_assert!(value.is_none_or(|value: Value| {
                                        value.to_bits() == result.to_bits()
                                    }));
                                    value = Some(result);
                                    derivative
                                });
                    }
                    derivatives.truncate((slot + 1) * lanes);
                    let Some(value) = value else {
                        return false;
                    };
                    values[slot] = value;
                }
                Op::Subtree(expr) => {
                    let mut value = None;
                    for lane in 0..lanes {
                        let target = if lane < node_lanes {
                            DerivativeTarget::Node(lane)
                        } else {
                            DerivativeTarget::Branch(lane - node_lanes)
                        };
                        let context = BehavioralDerivativeContext {
                            program: compiled,
                            node_values,
                            branch_values,
                            time: environment.time,
                            frequency: environment.frequency,
                            temperature: environment.temperature,
                            gmin: environment.gmin,
                            expression_dialect: dialect,
                            logarithm_domain: environment.logarithm_domain,
                            target,
                        };
                        derivatives.push(eval_behavioral_expr_with_derivative(expr, &context).map(
                            |(result, derivative)| {
                                debug_assert!(value.is_none_or(|value: Value| {
                                    value.to_bits() == result.to_bits()
                                }));
                                value = Some(result);
                                derivative
                            },
                        ));
                    }
                    let Some(value) = value else {
                        return false;
                    };
                    values.push(value);
                }
                Op::Select { then, otherwise } => {
                    let condition = values.pop().expect("a selection has a condition");
                    let start = derivatives.len() - lanes;
                    let mut live = [false; MAX_LANES];
                    for (live, derivative) in live.iter_mut().zip(&derivatives[start..]) {
                        *live = derivative.is_some();
                    }
                    derivatives.truncate(start);
                    if !live.contains(&true) {
                        return false;
                    }
                    let branch = if condition != 0.0 { then } else { otherwise };
                    if !self.run(branch, values, derivatives) {
                        return false;
                    }
                    for (live, derivative) in live.iter().zip(&mut derivatives[start..]) {
                        if !live {
                            *derivative = None;
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::super::analytic_expression_partial;
    use super::*;
    use crate::config::ExpressionDialect;
    use crate::expr::{LogarithmDomain, compile, parse_expression_strict};

    /// The single pass is only a faster way to ask the per-input walk its
    /// question: every lane, at every input, must be the walk's answer to the
    /// bit — including where the walk has none.
    #[test]
    fn every_lane_is_the_per_input_walk_to_the_bit() {
        let expressions = [
            "v(a)*(1.8-v(b))/1.8",
            "1.8-v(b)",
            "i(vx)^1",
            "-2*1e6*i(vx)^2",
            "v(a)^v(b)",
            "v(a)/v(b)",
            "v(a)%v(b)",
            "-(v(a)-v(b))*!v(a)",
            "(v(a)<v(b))+(v(a)>=v(b))*v(b)",
            "exp(v(a))*v(b)+sin(v(b))*i(vx)",
            "v(a)*time+temper-v(b)/v(a)",
            "abs(v(a))*sqrt(v(b))-tanh(i(vx))",
            "v(a)>0 ? v(a)*v(b) : -v(b)",
            "if(v(a)>0.9,2,0)+if(v(b)>0.9,4,0)+if(i(vx)>0.9,8,0)",
            "if(v(a),v(a)*v(b),-v(b)/v(a))",
            "if(v(a)^v(b),v(a)*v(b),v(b))",
            "if(v(a)>v(b),if(i(vx)<0,v(a)^2,i(vx)),sqrt(v(b)))*v(a)",
            "if(v(a)>0,v(a)/v(b),v(b)/v(a))+exp(v(b))",
            "max(v(a),v(b))*pwr(v(b),2.5)+uramp(v(a))",
            "v(a)/(v(b)-v(b))",
            "v(a)*v(a)*v(a)-v(0)",
            "sin(v(a))",
        ];
        let samples = [
            -2.5,
            -0.0,
            0.0,
            1.0e-300,
            0.7,
            1.8,
            3.0,
            1.0e300,
            f64::INFINITY,
            f64::NAN,
        ];
        let bits = |value: Option<Value>| value.map(f64::to_bits);
        let mut lanes_checked = 0_usize;
        for text in expressions {
            let source = Arc::new(parse_expression_strict(text).expect(text));
            let compiled = Arc::new(compile(&source));
            let nodes = compiled.node_map.len();
            let inputs = nodes + compiled.branch_map.len();
            for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
                let environment = BehavioralEnvironment {
                    time: 1.0e-6,
                    frequency: 0.0,
                    temperature: 27.0,
                    gmin: 1.0e-12,
                    expression_dialect: dialect,
                    logarithm_domain: LogarithmDomain::Ieee,
                };
                let mut partials = AnalyticPartials::default();
                for combination in 0..samples.len().pow(inputs as u32) {
                    let mut rest = combination;
                    let mut point = Vec::with_capacity(inputs);
                    for _ in 0..inputs {
                        point.push(samples[rest % samples.len()]);
                        rest /= samples.len();
                    }
                    let (node_values, branch_values) = point.split_at(nodes);
                    partials.evaluate(&source, &compiled, node_values, branch_values, environment);
                    for lane in 0..inputs {
                        let target = if lane < nodes {
                            DerivativeTarget::Node(lane)
                        } else {
                            DerivativeTarget::Branch(lane - nodes)
                        };
                        let walk = analytic_expression_partial(
                            &source,
                            &compiled,
                            node_values,
                            branch_values,
                            environment,
                            target,
                        );
                        let answer = if lane < nodes {
                            partials.node(lane)
                        } else {
                            partials.branch(lane - nodes)
                        };
                        // Only a lone subtree is left to the walk.
                        assert_eq!(answer.is_some(), partials.program.is_some(), "{text}");
                        let pass = answer.unwrap_or(walk);
                        assert_eq!(
                            bits(pass),
                            bits(walk),
                            "{text} ({dialect:?}) lane {lane} at {point:?}"
                        );
                        lanes_checked += 1;
                    }
                }
            }
        }
        assert!(lanes_checked > 10_000, "{lanes_checked} lanes checked");
    }

    /// A source whose expression is rewritten gets a new tree, and the pass
    /// follows it rather than answering for the old one.
    #[test]
    fn a_rewritten_expression_is_flattened_again() {
        let environment = BehavioralEnvironment {
            time: 0.0,
            frequency: 0.0,
            temperature: 27.0,
            gmin: 1.0e-12,
            expression_dialect: ExpressionDialect::Ngspice,
            logarithm_domain: LogarithmDomain::Ieee,
        };
        let mut partials = AnalyticPartials::default();
        let first = Arc::new(parse_expression_strict("2*v(a)").unwrap());
        let first_compiled = Arc::new(compile(&first));
        partials.evaluate(&first, &first_compiled, &[1.0], &[], environment);
        assert_eq!(partials.node(0), Some(Some(2.0)));
        let second = Arc::new(parse_expression_strict("3*v(a)").unwrap());
        let second_compiled = Arc::new(compile(&second));
        partials.evaluate(&second, &second_compiled, &[1.0], &[], environment);
        assert_eq!(partials.node(0), Some(Some(3.0)));
    }
}
