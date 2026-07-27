//! Array-form chain rules for the width-parameterized lowering.
//!
//! Where the scalarized lowering materializes one value per live lane, this
//! emits one `[f64; L]` binding per differentiated value and lets LLVM unroll
//! it. The rules here mirror `ScalarGraphBuilder::lower_*_derivatives` in
//! `canonical_ir::opt` operation for operation — same factors, same order of
//! operands — because matching that exactly is what keeps the two backends
//! numerically interchangeable while both exist.
//!
//! ## Uniform width instead of per-lane presence
//!
//! The scalarized rules track which lanes an operand actually has and emit only
//! those: `a - b` with a derivative on the right alone yields `-db`, not
//! `0 - db`. Here every value carries all `L` lanes, with absent ones held at
//! zero, so the same case yields `0.0 - db`.
//!
//! Those agree on every finite value. They differ only in the sign of zero —
//! `0.0 - 0.0` is `+0.0` where `-(0.0)` is `-0.0` — which no matrix stamp can
//! observe, since both add the same quantity to the same entry. The wasted
//! arithmetic on absent lanes is real but small: the probe in
//! `benchmarks/reference/lowering-probe` measured the array form scaling
//! sub-linearly in `L`, which is why uniform width beats tracking presence.
//!
//! Unused lanes must hold exactly `0.0` rather than being left undefined. Add,
//! sub and mul propagate zeros; division does too as long as the primal is
//! finite. A garbage lane would otherwise turn into a NaN that spreads to
//! every downstream lane.

use std::collections::HashSet;

use crate::canonical_ir::{OptBinaryOp, OptModel, OptUnaryOp, OptValueKind, ValueId};

/// Where a value's derivative comes from, structurally.
///
/// Derived from the value's own operator rather than from a `derivatives`
/// list, because the primal graph the packed lowering reads has none — that is
/// the whole point of reading it before the expansion pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Dependence {
    /// Independent of every unknown; costs one binding, not two.
    None,
    /// A seed: this value *is* an unknown, so its array is a unit vector.
    Seed,
    /// Differentiated exactly when its listed operands are.
    Propagates,
    /// Differentiated regardless of its operands.
    ///
    /// `$limit` is the case: it contributes an affine correction lane whether
    /// or not the proposed value depends on an unknown.
    Always,
}

/// How a value's derivative relates to its operands, before any lane arithmetic.
pub(super) fn dependence(kind: &OptValueKind) -> Dependence {
    match kind {
        // The three ways an unknown enters a model.
        OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::BranchUnknownFlow { .. } => Dependence::Seed,

        OptValueKind::Limit { .. } => Dependence::Always,

        // Runtime loop carriers keep a derivative on every lane, because the
        // trip count is not known until the model runs.
        OptValueKind::RuntimeLoopVariable { .. } | OptValueKind::RuntimeLoopResult { .. } => {
            Dependence::Always
        }

        OptValueKind::Unary { op, .. } => {
            if unary_rule(*op, "", "") == LaneRule::Zero {
                Dependence::None
            } else {
                Dependence::Propagates
            }
        }
        OptValueKind::Binary { op, .. } => {
            if binary_rule(*op, "", "", "") == LaneRule::Zero {
                Dependence::None
            } else {
                Dependence::Propagates
            }
        }

        OptValueKind::Select { .. }
        | OptValueKind::Ddt { .. }
        | OptValueKind::CountedSum { .. }
        | OptValueKind::SimParam { .. } => Dependence::Propagates,

        // Constants, model parameters, ambient quantities, and the values that
        // deliberately break the derivative chain. `Ddx` is among them: it
        // reports a derivative rather than carrying one, and `LimitPrevious`
        // holds the previous iterate, which Newton treats as fixed.
        OptValueKind::RealConstant(_)
        | OptValueKind::BooleanConstant(_)
        | OptValueKind::Parameter { .. }
        | OptValueKind::ParamGiven { .. }
        | OptValueKind::SimParamGiven { .. }
        | OptValueKind::Temperature
        | OptValueKind::ThermalVoltage
        | OptValueKind::Multiplicity
        | OptValueKind::Time
        | OptValueKind::Analysis { .. }
        | OptValueKind::Ddx { .. }
        | OptValueKind::DdtScale
        | OptValueKind::LimitPrevious { .. }
        | OptValueKind::LoopIndex { .. }
        | OptValueKind::RuntimeLoopVariableDerivative { .. }
        | OptValueKind::RuntimeLoopResultDerivative { .. }
        | OptValueKind::EquationValue { .. } => Dependence::None,
    }
}

/// Operands whose derivative feeds this value's.
///
/// Only the operands the chain rule actually reads: a `Select`'s condition is
/// excluded because it is boolean, and `Limit`'s candidate because the affine
/// correction is formed from the primal displacement rather than a derivative.
fn differentiating_operands(kind: &OptValueKind) -> Vec<ValueId> {
    match kind {
        OptValueKind::Unary { input, .. } => vec![*input],
        OptValueKind::Binary { left, right, .. } => vec![*left, *right],
        OptValueKind::Select {
            then_value,
            else_value,
            ..
        } => vec![*then_value, *else_value],
        OptValueKind::Ddt { input, .. } => vec![*input],
        OptValueKind::SimParam { fallback, .. } => vec![*fallback],
        OptValueKind::CountedSum { initial, term, .. } => vec![*initial, *term],
        OptValueKind::Limit { proposed, .. } => vec![*proposed],
        _ => Vec::new(),
    }
}

/// Values that need a derivative array emitted.
///
/// Everything reachable from a seed through operators that propagate. Values
/// outside this set — a compact model's temperature scaling, geometry
/// arithmetic and parameter preprocessing, which is a large share of it — cost
/// a single primal binding.
///
/// Relies on operands preceding their consumers in the value list, which the
/// graph builder guarantees by construction.
pub(super) fn differentiated_values(opt: &OptModel) -> HashSet<ValueId> {
    let mut differentiated = HashSet::new();
    for value in &opt.values {
        let needs = match dependence(&value.kind) {
            Dependence::None => false,
            Dependence::Seed | Dependence::Always => true,
            Dependence::Propagates => differentiating_operands(&value.kind)
                .into_iter()
                .any(|operand| differentiated.contains(&operand)),
        };
        if needs {
            differentiated.insert(value.id);
        }
    }
    differentiated
}

/// How a value's derivative array is built from its operands'.
///
/// `Scaled` covers every unary rule that is a single factor times the operand
/// derivative, which is most of them; the factor is emitted once outside the
/// lane loop rather than recomputed per lane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LaneRule {
    /// The value does not depend on any unknown; no array is emitted.
    Zero,
    /// `d[k] = <factor> * input_d[k]`, with `factor` bound once.
    Scaled { factor: String },
    /// `d[k] = input_d[k] / <divisor>`, with `divisor` bound once.
    ///
    /// Kept as a division rather than multiplication by a reciprocal because
    /// the scalarized rules divide, and `x / y` is not `x * (1/y)` in binary
    /// floating point.
    Divided { divisor: String },
    /// `d[k] = -input_d[k]`.
    Negated,
    /// `d[k] = input_d[k]`.
    Copied,
    /// A rule over two operand arrays, written as a per-lane expression in
    /// terms of `l[k]` and `r[k]`.
    Binary { lane_expr: String },
    /// `d = <unit vector at slot>`; the value is an unknown.
    Seed { slot: usize },
    /// `d = if <condition> { then_d } else { else_d }`.
    ///
    /// The scalarized rules select lane by lane. Selecting the whole array at
    /// once is the same arithmetic — the condition does not vary across lanes —
    /// and leaves the branch outside the unrolled body instead of inside it.
    Selected { condition: String },
}

/// Array-form derivative of a unary operation.
///
/// `input` and `value` are expressions for the operand's primal and this
/// value's own primal; several rules are cheaper in terms of the output (the
/// derivative of `exp` is the result itself) and the scalarized rules already
/// take that shortcut, so matching it also matches their rounding.
pub(super) fn unary_rule(op: OptUnaryOp, input: &str, value: &str) -> LaneRule {
    match op {
        OptUnaryOp::Pos => LaneRule::Copied,
        OptUnaryOp::Neg => LaneRule::Negated,
        // d(exp(x)) = exp(x) dx, and exp(x) is this value.
        OptUnaryOp::Exp => LaneRule::Scaled {
            factor: value.to_string(),
        },
        OptUnaryOp::LimExp => LaneRule::Scaled {
            factor: format!("limexp_derivative({input})"),
        },
        OptUnaryOp::LimitedExp => LaneRule::Scaled {
            factor: format!("limited_exp_derivative({input})"),
        },
        OptUnaryOp::Ln => LaneRule::Divided {
            divisor: input.to_string(),
        },
        // d(sqrt(x)) = dx / (2 sqrt(x)), and sqrt(x) is this value.
        OptUnaryOp::Sqrt => LaneRule::Divided {
            divisor: format!("(2.0 * {value})"),
        },
        OptUnaryOp::Sin => LaneRule::Scaled {
            factor: format!("({input}).cos()"),
        },
        OptUnaryOp::Cos => LaneRule::Scaled {
            factor: format!("(-(({input}).sin()))"),
        },
        OptUnaryOp::Tan => LaneRule::Divided {
            divisor: format!("(({input}).cos() * ({input}).cos())"),
        },
        OptUnaryOp::Sinh => LaneRule::Scaled {
            factor: format!("({input}).cosh()"),
        },
        OptUnaryOp::Cosh => LaneRule::Scaled {
            factor: format!("({input}).sinh()"),
        },
        // d(tanh(x)) = (1 - tanh(x)^2) dx, in terms of this value.
        OptUnaryOp::Tanh => LaneRule::Scaled {
            factor: format!("(1.0 - {value} * {value})"),
        },
        OptUnaryOp::Atan => LaneRule::Divided {
            divisor: format!("(1.0 + {input} * {input})"),
        },
        OptUnaryOp::Asinh => LaneRule::Divided {
            divisor: format!("((1.0 + {input} * {input}).sqrt())"),
        },
        // Piecewise-constant or boolean: no derivative contribution at all.
        // `abs` is deliberately among these — the scalarized rules drop it
        // rather than pick a subgradient at the kink, and diverging here would
        // silently change every model that takes an absolute value.
        OptUnaryOp::Abs
        | OptUnaryOp::Floor
        | OptUnaryOp::Ceil
        | OptUnaryOp::Not
        | OptUnaryOp::LimExpDerivative
        | OptUnaryOp::LimitedExpDerivative => LaneRule::Zero,
    }
}

/// Array-form derivative of a binary operation.
///
/// `left`/`right` are the operand primals and `value` this value's primal.
/// Lane arrays are referred to as `l` and `r`.
pub(super) fn binary_rule(
    op: OptBinaryOp,
    left: &str,
    right: &str,
    value: &str,
) -> LaneRule {
    match op {
        OptBinaryOp::Add => LaneRule::Binary {
            lane_expr: "l[k] + r[k]".to_string(),
        },
        OptBinaryOp::Sub => LaneRule::Binary {
            lane_expr: "l[k] - r[k]".to_string(),
        },
        // Product rule, operands in the same order the scalarized builder uses.
        OptBinaryOp::Mul => LaneRule::Binary {
            lane_expr: format!("l[k] * {right} + {left} * r[k]"),
        },
        // Quotient rule expressed through the already-computed quotient, which
        // is what the scalarized builder does: (dl - q dr) / rhs.
        OptBinaryOp::Div => LaneRule::Binary {
            lane_expr: format!("(l[k] - {value} * r[k]) / {right}"),
        },
        // d(a^b) = a^b (b/a da + ln(a) db). Left as a single expression so the
        // exponent term vanishes cleanly when the exponent is constant.
        OptBinaryOp::Pow => LaneRule::Binary {
            lane_expr: format!(
                "{value} * (({right} / {left}) * l[k] + ({left}).ln() * r[k])"
            ),
        },
        // Comparisons, logic and modulo are piecewise constant.
        OptBinaryOp::Mod
        | OptBinaryOp::Eq
        | OptBinaryOp::Ne
        | OptBinaryOp::Lt
        | OptBinaryOp::Le
        | OptBinaryOp::Gt
        | OptBinaryOp::Ge
        | OptBinaryOp::And
        | OptBinaryOp::Or => LaneRule::Zero,
    }
}

impl LaneRule {
    /// Render the rule as a `[f64; L]` initializer.
    ///
    /// `input` names the single operand array for the unary forms; `left` and
    /// `right` name the two arrays the binary form reads as `l` and `r`.
    pub(super) fn emit(&self, width: usize, input: &str, left: &str, right: &str) -> Option<String> {
        match self {
            Self::Zero => None,
            Self::Copied => Some(input.to_string()),
            Self::Negated => Some(format!(
                "core::array::from_fn::<f64, {width}, _>(|k| -{input}[k])"
            )),
            Self::Scaled { factor } => Some(format!(
                "{{ let s = {factor}; core::array::from_fn::<f64, {width}, _>(|k| s * {input}[k]) }}"
            )),
            Self::Divided { divisor } => Some(format!(
                "{{ let q = {divisor}; core::array::from_fn::<f64, {width}, _>(|k| {input}[k] / q) }}"
            )),
            Self::Binary { lane_expr } => Some(format!(
                "{{ let l = &{left}; let r = &{right}; core::array::from_fn::<f64, {width}, _>(|k| {lane_expr}) }}"
            )),
            Self::Seed { slot } => Some(format!(
                "{{ let mut d = [0.0f64; {width}]; d[{slot}] = 1.0; d }}"
            )),
            Self::Selected { condition } => {
                Some(format!("(if {condition} {{ {left} }} else {{ {right} }})"))
            }
        }
    }
}

/// Array-form derivative of an unknown the model reads directly.
///
/// `slot` is the lane's position from [`super::lanes::LaneSet`]. A seed whose
/// lane the device does not pack — a branch flow on a branch with no unknown —
/// has no derivative, matching the scalarized rules, which return an empty map
/// rather than a zero.
pub(super) fn seed_rule(slot: Option<usize>) -> LaneRule {
    match slot {
        Some(slot) => LaneRule::Seed { slot },
        None => LaneRule::Zero,
    }
}

/// Array-form derivative of `ddt`.
///
/// The scalarized rule multiplies each lane by the integration scale, so this
/// is an ordinary scaling. `scale` is the expression for that factor.
pub(super) fn ddt_rule(scale: &str) -> LaneRule {
    LaneRule::Scaled {
        factor: scale.to_string(),
    }
}

/// Array-form derivative of a conditional.
pub(super) fn select_rule(condition: &str) -> LaneRule {
    LaneRule::Selected {
        condition: condition.to_string(),
    }
}

/// How the emitted body reaches quantities it does not compute itself.
///
/// Parameterized rather than hard-coded so the same emitter serves the stamp,
/// where these are struct fields, and a test harness, where they are locals.
pub(super) struct PackedContext {
    /// Indexing expression for a model parameter, given its index.
    pub(super) parameter: fn(usize) -> String,
    /// Indexing expression for a parameter's `$param_given` flag.
    pub(super) param_given: fn(usize) -> String,
    /// Expression yielding the potential at a node index.
    pub(super) node_potential: fn(usize) -> String,
    pub(super) temperature: String,
    pub(super) thermal_voltage: String,
    pub(super) multiplicity: String,
    pub(super) ddt_scale: String,
    /// Simulator parameter lookup, given its name and a fallback expression.
    pub(super) simparam: fn(&str, &str) -> String,
    /// Whether the netlist supplied a simulator parameter.
    pub(super) simparam_given: fn(&str) -> String,
}

/// A generated body, with the counts that decide whether the rewrite paid off.
#[derive(Debug, Clone)]
pub(super) struct PackedBody {
    pub(super) source: String,
    pub(super) primal_bindings: usize,
    pub(super) derivative_bindings: usize,
}

/// A model split into the passes that re-run at different rates.
///
/// Emitting one body for everything is correct but measures nothing useful:
/// almost all of a compact model is parameter and geometry preprocessing that
/// changes only when the instance does. For bsimbulk that is 5,046 values
/// against 85 per Newton iteration, so a single pass does roughly sixty times
/// the per-iteration work and cannot be compared with a backend that caches.
#[derive(Debug, Clone)]
pub(super) struct PackedSchedule {
    /// Recomputed only when instance parameters change.
    pub(super) instance_static: PackedBody,
    /// Recomputed when temperature changes.
    pub(super) temperature_static: PackedBody,
    /// Recomputed every Newton iteration. This is the hot path.
    pub(super) newton: PackedBody,
}

/// Emit straight-line Rust for a primal graph, packing derivatives into arrays.
///
/// Values are emitted in the order the graph defines them, which is already a
/// valid schedule: the builder guarantees operands precede consumers. One
/// function, no blocks — the split probe in `benchmarks/reference/split-probe`
/// measured that cutting the body costs 2.4x at run time to save about twenty
/// seconds of compile time, because arrays crossing a boundary are forced to
/// memory.
pub(super) fn emit_body(
    opt: &OptModel,
    width: usize,
    lane_slot: &dyn Fn(&OptValueKind) -> Option<usize>,
    differentiated: &HashSet<ValueId>,
    ctx: &PackedContext,
) -> Result<PackedBody, String> {
    let mut source = String::new();
    // Bound once so a rule can name it wherever an undifferentiated operand
    // appears, instead of every such value paying for its own zero array.
    source.push_str(&format!("const {ZERO_ARRAY}: [f64; {width}] = [0.0; {width}];
"));
    let mut primal_bindings = 0usize;
    let mut derivative_bindings = 0usize;

    // Values whose result changes with a counted sum's loop index cannot be
    // bound at the top level, where no index is in scope. They are emitted
    // inside the loop that owns them instead, and skipped here.
    let loop_interior = loop_dependent_values(opt);

    for value in &opt.values {
        if loop_interior.contains(&value.id) {
            continue;
        }
        let id = usize::from(value.id);
        let primal = match &value.kind {
            OptValueKind::CountedSum {
                count,
                initial,
                term,
                ..
            } => counted_sum_expr(
                opt,
                value.id,
                *count,
                *initial,
                *term,
                &loop_interior,
                ctx,
            )?,
            other => primal_expr(value.id, other, ctx)?,
        };
        source.push_str(&format!("let v{id} = {primal};\n"));
        primal_bindings += 1;

        if !differentiated.contains(&value.id) {
            continue;
        }
        let Some(rendered) = derivative_binding(value.id, &value.kind, width, lane_slot, differentiated, ctx)?
        else {
            continue;
        };
        source.push_str(&format!("let d{id}: [f64; {width}] = {rendered};\n"));
        derivative_bindings += 1;
    }

    Ok(PackedBody {
        source,
        primal_bindings,
        derivative_bindings,
    })
}

/// Values whose result depends on some counted sum's loop index.
///
/// These are the loop's interior. They must not be bound at the top level,
/// where no index exists, and are instead re-emitted inside the loop that owns
/// them — which is also why a counted sum cannot simply be an expression over
/// already-computed values.
///
/// Relies on operands preceding consumers, as everything else here does.
fn loop_dependent_values(opt: &OptModel) -> HashSet<ValueId> {
    let mut dependent = HashSet::new();
    for value in &opt.values {
        let carries = match &value.kind {
            OptValueKind::LoopIndex { .. } => true,
            // A counted sum closes over its own index: the sum's *result* is a
            // plain value again, so propagation stops here rather than
            // spreading the whole downstream graph into the loop.
            OptValueKind::CountedSum { count, initial, .. } => {
                dependent.contains(count) || dependent.contains(initial)
            }
            other => operand_values(other)
                .into_iter()
                .any(|operand| dependent.contains(&operand)),
        };
        if carries {
            dependent.insert(value.id);
        }
    }
    dependent
}

/// Every direct value operand of a kind.
fn operand_values(kind: &OptValueKind) -> Vec<ValueId> {
    match kind {
        OptValueKind::Unary { input, .. } => vec![*input],
        OptValueKind::Binary { left, right, .. } => vec![*left, *right],
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => vec![*condition, *then_value, *else_value],
        OptValueKind::Ddx { value, .. } => vec![*value],
        OptValueKind::Ddt { input, .. } => vec![*input],
        OptValueKind::SimParam { fallback, .. } => vec![*fallback],
        OptValueKind::LimitPrevious { proposed, .. } => vec![*proposed],
        OptValueKind::Limit {
            proposed,
            candidate,
            ..
        } => vec![*proposed, *candidate],
        OptValueKind::CountedSum {
            count,
            initial,
            term,
            ..
        } => vec![*count, *initial, *term],
        _ => Vec::new(),
    }
}

/// Emit a counted sum as an accumulating loop.
///
/// The term varies with the loop index, so the part of the graph that depends
/// on the index is re-emitted inside the loop body. Values that do not depend
/// on it are already bound outside and are referenced directly.
fn counted_sum_expr(
    opt: &OptModel,
    owner: ValueId,
    count: ValueId,
    initial: ValueId,
    term: ValueId,
    loop_interior: &HashSet<ValueId>,
    ctx: &PackedContext,
) -> Result<String, String> {
    let id = usize::from(owner);
    let index = format!("li{id}");
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("    let mut acc{id} = v{};\n", usize::from(initial)));
    out.push_str(&format!("    let n{id} = v{};\n", usize::from(count)));
    out.push_str(&format!("    let mut c{id}: i64 = 0;\n"));
    out.push_str(&format!("    while (c{id} as f64) < n{id} {{\n"));
    out.push_str(&format!("        let {index} = c{id} as f64;\n"));

    // Re-emit the interior in definition order. Shadowing the outer binding is
    // intended: inside the loop the indexed value is the live one.
    for value in &opt.values {
        if !loop_interior.contains(&value.id) || value.id == owner {
            continue;
        }
        if usize::from(value.id) > usize::from(term) {
            break;
        }
        let inner = usize::from(value.id);
        let expr = match &value.kind {
            OptValueKind::LoopIndex { .. } => index.clone(),
            other => primal_expr(value.id, other, ctx)?,
        };
        out.push_str(&format!("        let v{inner} = {expr};\n"));
    }

    out.push_str(&format!("        acc{id} += v{};\n", usize::from(term)));
    out.push_str(&format!("        c{id} += 1;\n"));
    out.push_str("    }\n");
    out.push_str(&format!("    acc{id}\n"));
    out.push_str("}");
    Ok(out)
}

/// Name of the shared all-zero derivative array.
///
/// A differentiated value can read an operand that is not itself
/// differentiated — `x * 2.0`, or an arm of a select that depends on no
/// unknown. Uniform width means the rule still wants an array there, so it
/// reads this one rather than forcing a binding for every constant in the
/// model, which would undo what reachability just saved.
pub(super) const ZERO_ARRAY: &str = "DZERO";

/// Emit one body per invalidation class.
///
/// Values keep their names across bodies, so a later pass reads an earlier
/// one's bindings directly. Whether those cross a function boundary as
/// arguments or through a cache is the caller's decision; what matters here is
/// that per-iteration work is separated from work that is not.
pub(super) fn emit_scheduled(
    opt: &OptModel,
    width: usize,
    lane_slot: &dyn Fn(&OptValueKind) -> Option<usize>,
    differentiated: &HashSet<ValueId>,
    ctx: &PackedContext,
) -> Result<PackedSchedule, String> {
    use crate::canonical_ir::InvalidationClass;

    let classes = opt.value_invalidation_classes();
    let loop_interior = loop_dependent_values(opt);

    let mut bodies = [
        // Anything that is not one of the two static classes runs per
        // iteration. Treating an unrecognized class as static would cache a
        // value that changes, which is silent and wrong; treating it as
        // dynamic only costs time.
        (InvalidationClass::InstanceStatic, PackedBody { source: String::new(), primal_bindings: 0, derivative_bindings: 0 }),
        (InvalidationClass::TemperatureStatic, PackedBody { source: String::new(), primal_bindings: 0, derivative_bindings: 0 }),
        (InvalidationClass::NewtonIteration, PackedBody { source: String::new(), primal_bindings: 0, derivative_bindings: 0 }),
    ];

    for value in &opt.values {
        if loop_interior.contains(&value.id) {
            continue;
        }
        let class = classes[usize::from(value.id)];
        let slot = match class {
            InvalidationClass::InstanceStatic => 0,
            InvalidationClass::TemperatureStatic => 1,
            _ => 2,
        };

        let id = usize::from(value.id);
        let primal = match &value.kind {
            OptValueKind::CountedSum { count, initial, term, .. } => counted_sum_expr(
                opt, value.id, *count, *initial, *term, &loop_interior, ctx,
            )?,
            other => primal_expr(value.id, other, ctx)?,
        };
        let body = &mut bodies[slot].1;
        body.source.push_str(&format!("let v{id} = {primal};\n"));
        body.primal_bindings += 1;

        if !differentiated.contains(&value.id) {
            continue;
        }
        if let Some(rendered) =
            derivative_binding(value.id, &value.kind, width, lane_slot, differentiated, ctx)?
        {
            body.source
                .push_str(&format!("let d{id}: [f64; {width}] = {rendered};\n"));
            body.derivative_bindings += 1;
        }
    }

    let [(_, instance_static), (_, temperature_static), (_, newton)] = bodies;
    Ok(PackedSchedule {
        instance_static,
        temperature_static,
        newton,
    })
}

fn derivative_binding(
    id: ValueId,
    kind: &OptValueKind,
    width: usize,
    lane_slot: &dyn Fn(&OptValueKind) -> Option<usize>,
    differentiated: &HashSet<ValueId>,
    ctx: &PackedContext,
) -> Result<Option<String>, String> {
    let own = format!("v{}", usize::from(id));
    let array_of = |operand: &ValueId| {
        if differentiated.contains(operand) {
            format!("d{}", usize::from(*operand))
        } else {
            ZERO_ARRAY.to_string()
        }
    };
    let rule = match kind {
        OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::BranchUnknownFlow { .. } => seed_rule(lane_slot(kind)),
        OptValueKind::Unary { op, input } => {
            unary_rule(*op, &format!("v{}", usize::from(*input)), &own)
        }
        OptValueKind::Binary { op, left, right } => binary_rule(
            *op,
            &format!("v{}", usize::from(*left)),
            &format!("v{}", usize::from(*right)),
            &own,
        ),
        OptValueKind::Select { condition, .. } => {
            select_rule(&truth(&format!("v{}", usize::from(*condition))))
        }
        OptValueKind::Ddt { .. } => ddt_rule(&ctx.ddt_scale),
        other => {
            return Err(format!(
                "packed lowering has no derivative rule for {}",
                kind_label(other)
            ));
        }
    };

    let (input, left, right) = match kind {
        OptValueKind::Unary { input, .. } | OptValueKind::Ddt { input, .. } => {
            (array_of(input), String::new(), String::new())
        }
        OptValueKind::Binary { left, right, .. } => {
            (String::new(), array_of(left), array_of(right))
        }
        OptValueKind::Select {
            then_value,
            else_value,
            ..
        } => (String::new(), array_of(then_value), array_of(else_value)),
        _ => (String::new(), String::new(), String::new()),
    };

    Ok(rule.emit(width, &input, &left, &right))
}

fn primal_expr(
    id: ValueId,
    kind: &OptValueKind,
    ctx: &PackedContext,
) -> Result<String, String> {
    let _ = id;
    Ok(match kind {
        OptValueKind::RealConstant(value) => format_f64(*value),
        OptValueKind::BooleanConstant(value) => {
            if *value {
                "1.0f64".to_string()
            } else {
                "0.0f64".to_string()
            }
        }
        OptValueKind::Parameter { parameter } => (ctx.parameter)(usize::from(*parameter)),
        OptValueKind::ParamGiven { parameter } => (ctx.param_given)(usize::from(*parameter)),
        OptValueKind::Temperature => ctx.temperature.clone(),
        OptValueKind::ThermalVoltage => ctx.thermal_voltage.clone(),
        OptValueKind::Multiplicity => ctx.multiplicity.clone(),
        OptValueKind::DdtScale => ctx.ddt_scale.clone(),
        OptValueKind::NodePotential { node } => (ctx.node_potential)(usize::from(*node)),
        OptValueKind::Unary { op, input } => {
            unary_primal(*op, &format!("v{}", usize::from(*input)))
        }
        OptValueKind::Binary { op, left, right } => binary_primal(
            *op,
            &format!("v{}", usize::from(*left)),
            &format!("v{}", usize::from(*right)),
        ),
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => format!(
            "(if {} {{ v{} }} else {{ v{} }})",
            truth(&format!("v{}", usize::from(*condition))),
            usize::from(*then_value),
            usize::from(*else_value)
        ),
        // `ddt` contributes through the integration scale the caller supplies;
        // its primal is the operand.
        OptValueKind::Ddt { input, .. } => format!("v{}", usize::from(*input)),
        OptValueKind::SimParam { name, fallback } => {
            (ctx.simparam)(name, &format!("v{}", usize::from(*fallback)))
        }
        OptValueKind::SimParamGiven { name } => (ctx.simparam_given)(name),
        other => {
            return Err(format!(
                "packed lowering cannot emit a primal for {}",
                kind_label(other)
            ));
        }
    })
}

/// Booleans travel as `f64` in the emitted body, matching the existing
/// backends, so a condition is a comparison against zero rather than a `bool`.
fn truth(expr: &str) -> String {
    format!("({expr} != 0.0)")
}

fn format_f64(value: f64) -> String {
    if value == value.trunc() && value.abs() < 1.0e15 {
        format!("{value:.1}f64")
    } else {
        format!("{value:e}f64")
    }
}

fn unary_primal(op: OptUnaryOp, input: &str) -> String {
    match op {
        OptUnaryOp::Pos => input.to_string(),
        OptUnaryOp::Neg => format!("(-{input})"),
        OptUnaryOp::Not => format!("(if {input} != 0.0 {{ 0.0f64 }} else {{ 1.0f64 }})"),
        OptUnaryOp::Exp => format!("({input}).exp()"),
        OptUnaryOp::LimExp => format!("limexp({input})"),
        OptUnaryOp::LimExpDerivative => format!("limexp_derivative({input})"),
        OptUnaryOp::LimitedExp => format!("limited_exp({input})"),
        OptUnaryOp::LimitedExpDerivative => format!("limited_exp_derivative({input})"),
        OptUnaryOp::Ln => format!("({input}).ln()"),
        OptUnaryOp::Sqrt => format!("({input}).sqrt()"),
        OptUnaryOp::Abs => format!("({input}).abs()"),
        OptUnaryOp::Sin => format!("({input}).sin()"),
        OptUnaryOp::Cos => format!("({input}).cos()"),
        OptUnaryOp::Tan => format!("({input}).tan()"),
        OptUnaryOp::Sinh => format!("({input}).sinh()"),
        OptUnaryOp::Cosh => format!("({input}).cosh()"),
        OptUnaryOp::Tanh => format!("({input}).tanh()"),
        OptUnaryOp::Atan => format!("({input}).atan()"),
        OptUnaryOp::Asinh => format!("({input}).asinh()"),
        OptUnaryOp::Floor => format!("({input}).floor()"),
        OptUnaryOp::Ceil => format!("({input}).ceil()"),
    }
}

fn binary_primal(op: OptBinaryOp, left: &str, right: &str) -> String {
    let compare = |symbol: &str| {
        format!("(if {left} {symbol} {right} {{ 1.0f64 }} else {{ 0.0f64 }})")
    };
    match op {
        OptBinaryOp::Add => format!("({left} + {right})"),
        OptBinaryOp::Sub => format!("({left} - {right})"),
        OptBinaryOp::Mul => format!("({left} * {right})"),
        OptBinaryOp::Div => format!("({left} / {right})"),
        OptBinaryOp::Mod => format!("({left} % {right})"),
        OptBinaryOp::Pow => format!("({left}).powf({right})"),
        OptBinaryOp::Eq => compare("=="),
        OptBinaryOp::Ne => compare("!="),
        OptBinaryOp::Lt => compare("<"),
        OptBinaryOp::Le => compare("<="),
        OptBinaryOp::Gt => compare(">"),
        OptBinaryOp::Ge => compare(">="),
        OptBinaryOp::And => format!(
            "(if {left} != 0.0 && {right} != 0.0 {{ 1.0f64 }} else {{ 0.0f64 }})"
        ),
        OptBinaryOp::Or => format!(
            "(if {left} != 0.0 || {right} != 0.0 {{ 1.0f64 }} else {{ 0.0f64 }})"
        ),
    }
}

fn kind_label(kind: &OptValueKind) -> &'static str {
    match kind {
        OptValueKind::SimParam { .. } => "SimParam",
        OptValueKind::SimParamGiven { .. } => "SimParamGiven",
        OptValueKind::Time => "Time",
        OptValueKind::Analysis { .. } => "Analysis",
        OptValueKind::Ddx { .. } => "Ddx",
        OptValueKind::LimitPrevious { .. } => "LimitPrevious",
        OptValueKind::Limit { .. } => "Limit",
        OptValueKind::BranchFlow { .. } => "BranchFlow",
        OptValueKind::BranchUnknownFlow { .. } => "BranchUnknownFlow",
        OptValueKind::LoopIndex { .. } => "LoopIndex",
        OptValueKind::CountedSum { .. } => "CountedSum",
        OptValueKind::RuntimeLoopVariable { .. } => "RuntimeLoopVariable",
        OptValueKind::RuntimeLoopVariableDerivative { .. } => "RuntimeLoopVariableDerivative",
        OptValueKind::RuntimeLoopResult { .. } => "RuntimeLoopResult",
        OptValueKind::RuntimeLoopResultDerivative { .. } => "RuntimeLoopResultDerivative",
        OptValueKind::EquationValue { .. } => "EquationValue",
        _ => "value",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_ir::OptModel;
    use crate::{CompilerOptions, VerilogACompiler};

    fn test_context() -> PackedContext {
        PackedContext {
            parameter: |index| format!("p[{index}]"),
            param_given: |index| format!("(if pg[{index}] {{ 1.0f64 }} else {{ 0.0f64 }})"),
            node_potential: |index| format!("nv[{index}]"),
            temperature: "temp".to_string(),
            thermal_voltage: "vt".to_string(),
            multiplicity: "mult".to_string(),
            ddt_scale: "ddt_scale".to_string(),
            simparam: |name, fallback| format!("simparam_or(\"{name}\", {fallback})"),
            simparam_given: |name| format!("(if simparam_given(\"{name}\") {{ 1.0f64 }} else {{ 0.0f64 }})"),
        }
    }

    fn primal_of(source: &str) -> OptModel {
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile to canonical IR");
        OptModel::primal_from_hir_and_mir(&artifact.hir, &artifact.mir).expect("primal OptIR")
    }

    #[test]
    fn emits_a_body_for_a_nonlinear_two_terminal_device() {
        let opt = primal_of(
            r#"
module packed_diode(p, n);
    inout p, n;
    electrical p, n;
    parameter real is = 1e-14;
    parameter real vt = 0.025;
    analog begin
        real vd;
        vd = V(p, n);
        I(p, n) <+ is * (exp(vd / vt) - 1.0);
    end
endmodule
"#,
        );
        let differentiated = differentiated_values(&opt);
        let body = emit_body(
            &opt,
            2,
            &|kind| match kind {
                OptValueKind::NodePotential { node } => Some(usize::from(*node)),
                _ => None,
            },
            &differentiated,
            &test_context(),
        )
        .expect("emit body");

        assert!(body.primal_bindings > 0);
        assert!(
            body.derivative_bindings > 0,
            "a diode's current depends on its terminal voltage"
        );
        // The seed must be a unit vector, and the exponential must scale by the
        // value it just computed rather than recomputing exp.
        assert!(body.source.contains("] = 1.0; d }"), "{}", body.source);
        assert!(body.source.contains(".exp()"), "{}", body.source);
        assert!(
            body.derivative_bindings < body.primal_bindings,
            "constants and parameters must not carry derivative arrays: {} of {}",
            body.derivative_bindings,
            body.primal_bindings
        );
    }

    #[test]
    fn emits_a_body_for_the_phase_one_gate_model() {
        // bsimbulk is the model the rewrite is measured against. Emitting it is
        // what tells us the rule set is complete rather than complete for
        // fixtures; the reported counts are the size half of the gate.
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../models/veriloga/cmc/BSIM-BULK107.2.1_02112025/code/bsimbulk.va");
        let source = std::fs::read_to_string(&path).expect("read bsimbulk");
        let mut options = CompilerOptions::default();
        options
            .include_paths
            .push(path.parent().expect("model directory").to_path_buf());
        let artifact = VerilogACompiler::new(options)
            .compile_canonical_ir_module(&source, Some("bsimbulk"))
            .expect("compile bsimbulk");
        let opt = OptModel::primal_from_hir_and_mir(&artifact.hir, &artifact.mir)
            .expect("primal OptIR");

        let differentiated = differentiated_values(&opt);
        // Which awkward kinds actually need a derivative rule, as opposed to
        // merely a primal one. A counted sum's derivative is another counted
        // sum -- a real loop, not straight-line code -- so it matters a great
        // deal whether any of them are reachable from a seed.
        for (label, count, differentiated_count) in [
            ("SimParam", 0, 0),
            ("LoopIndex", 0, 0),
            ("CountedSum", 0, 0),
        ]
        .iter()
        .map(|(label, _, _): &(&str, usize, usize)| {
            let matches = |kind: &OptValueKind| match (*label, kind) {
                ("SimParam", OptValueKind::SimParam { .. }) => true,
                ("LoopIndex", OptValueKind::LoopIndex { .. }) => true,
                ("CountedSum", OptValueKind::CountedSum { .. }) => true,
                _ => false,
            };
            let total = opt.values.iter().filter(|v| matches(&v.kind)).count();
            let diff = opt
                .values
                .iter()
                .filter(|v| matches(&v.kind) && differentiated.contains(&v.id))
                .count();
            (*label, total, diff)
        }) {
            eprintln!("{label:<12} total={count} differentiated={differentiated_count}");
        }

        let body = emit_body(
            &opt,
            17,
            &|kind| match kind {
                OptValueKind::NodePotential { node } => Some(usize::from(*node)),
                _ => None,
            },
            &differentiated,
            &test_context(),
        );

        match body {
            Ok(body) => {
                eprintln!(
                    "bsimbulk packed: primal={} derivative={} total={} source={}KB",
                    body.primal_bindings,
                    body.derivative_bindings,
                    body.primal_bindings + body.derivative_bindings,
                    body.source.len() / 1024
                );
                assert!(body.derivative_bindings > 0);
                if let Ok(path) = std::env::var("RSPICE_PACKED_DUMP") {
                    std::fs::write(&path, &body.source).expect("dump packed body");
                    // Every equation root, so a harness can keep the whole
                    // model live. Consuming one value lets the optimizer delete
                    // most of it, which reads as a very fast device.
                    use crate::canonical_ir::{InvalidationClass, OptOp};
                    let mut roots = Vec::new();
                    for schedule in &opt.schedules {
                        if schedule.invalidation != InvalidationClass::NewtonIteration {
                            continue;
                        }
                        let mut last = None;
                        for op in &schedule.ops {
                            match op {
                                OptOp::ComputeValue { value } => last = Some(*value),
                                OptOp::EvaluateEquation { .. } => {
                                    if let Some(value) = last.take() {
                                        roots.push(usize::from(value));
                                    }
                                }
                            }
                        }
                    }
                    let listing = roots
                        .iter()
                        .map(|root| root.to_string())
                        .collect::<Vec<_>>()
                        .join("\n");
                    std::fs::write(format!("{path}.roots"), listing).expect("dump roots");
                }
                assert!(
                    body.derivative_bindings < body.primal_bindings,
                    "reachability should spare the parameter and geometry arithmetic"
                );
            }
            Err(message) => {
                // The one gap left, and it is a bounded one: a counted sum's
                // term has to be re-emitted inside a loop because it varies
                // with the loop index, which straight-line emission cannot do.
                // Neither of bsimbulk's two counted sums is differentiated, so
                // only the primal path needs it.
                //
                // Fail on anything else. A new gap must surface here rather
                // than be absorbed into a permissive assertion.
                assert!(
                    message.contains("CountedSum") || message.contains("LoopIndex"),
                    "unexpected gap in the packed lowering: {message}"
                );
                eprintln!("bsimbulk still blocked on the counted-sum machinery: {message}");
            }
        }
    }

    /// Root value of the first equation, in either lowering.
    fn first_equation_root(opt: &OptModel) -> ValueId {
        use crate::canonical_ir::{InvalidationClass, OptOp};
        let newton = opt
            .schedules
            .iter()
            .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
            .expect("NewtonIteration schedule");
        let position = newton
            .ops
            .iter()
            .position(|op| matches!(op, OptOp::EvaluateEquation { .. }))
            .expect("an equation to evaluate");
        newton.ops[..position]
            .iter()
            .rev()
            .find_map(|op| match op {
                OptOp::ComputeValue { value } => Some(*value),
                OptOp::EvaluateEquation { .. } => None,
            })
            .expect("a value feeding the equation")
    }

    #[test]
    fn packed_derivatives_match_the_scalarized_reference() {
        // The claim the whole rewrite rests on: packing the lanes changes how
        // derivatives are *written*, not what they are. Everything else here
        // checks emitted text; this compiles the emitted body, runs it, and
        // compares against the reference evaluator on the same inputs.
        use crate::canonical_ir::{DerivativeLane, NodeId, OptEvalInputs};

        let source = r#"
module parity_check(p, n);
    inout p, n;
    electrical p, n;
    parameter real is = 1e-14;
    parameter real vth = 0.025;
    parameter real rs = 12.0;
    analog begin
        real vd, ratio, shaped;
        vd = V(p, n);
        ratio = vd / vth;
        shaped = exp(ratio) - 1.0;
        if (vd > 0.4)
            shaped = shaped * (1.0 + vd * vd);
        I(p, n) <+ is * shaped + vd / rs;
    end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile parity fixture");
        let primal = OptModel::primal_from_hir_and_mir(&artifact.hir, &artifact.mir)
            .expect("primal OptIR");

        let node_potentials = vec![0.55, 0.0];
        let parameters: Vec<f64> = artifact
            .mir
            .parameters
            .iter()
            .map(|parameter| parameter.default.unwrap_or(0.0))
            .collect();
        let inputs = OptEvalInputs {
            parameters: parameters.clone(),
            node_potentials: node_potentials.clone(),
            branch_flows: Vec::new(),
        };

        let reference = artifact.opt.evaluate(&inputs).expect("evaluate reference");
        let reference_root = first_equation_root(&artifact.opt);
        let expected: Vec<f64> = (0..node_potentials.len())
            .map(|node| {
                reference
                    .derivative(reference_root, DerivativeLane::node(NodeId::from(node)))
                    .unwrap_or(0.0)
            })
            .collect();

        // Emit the packed body and read the same root's array back out.
        let width = node_potentials.len();
        let differentiated = differentiated_values(&primal);
        let ctx = PackedContext {
            parameter: |index| format!("p[{index}]"),
            param_given: |_| "1.0f64".to_string(),
            node_potential: |index| format!("nv[{index}]"),
            temperature: "300.15f64".to_string(),
            thermal_voltage: "0.025852f64".to_string(),
            multiplicity: "1.0f64".to_string(),
            ddt_scale: "0.0f64".to_string(),
            simparam: |_, fallback| fallback.to_string(),
            simparam_given: |_| "0.0f64".to_string(),
        };
        let body = emit_body(
            &primal,
            width,
            &|kind| match kind {
                OptValueKind::NodePotential { node } => Some(usize::from(*node)),
                _ => None,
            },
            &differentiated,
            &ctx,
        )
        .expect("emit packed body");

        let packed_root = first_equation_root(&primal);
        assert!(
            differentiated.contains(&packed_root),
            "the equation root must depend on the terminal voltage"
        );

        let params_literal = parameters
            .iter()
            .map(|value| format!("{value:e}f64"))
            .collect::<Vec<_>>()
            .join(", ");
        let nodes_literal = node_potentials
            .iter()
            .map(|value| format!("{value:e}f64"))
            .collect::<Vec<_>>()
            .join(", ");
        let program = format!(
            "#![allow(unused_parens, unused_variables, dead_code, non_snake_case)]\n\
             fn main() {{\n\
             let p = [{params_literal}];\n\
             let nv = [{nodes_literal}];\n\
             {body_source}\
             let out = d{root};\n\
             println!(\"{{:?}}\", out);\n\
             }}\n",
            body_source = body.source,
            root = usize::from(packed_root),
        );

        let dir = std::env::temp_dir().join(format!("rspice-packed-parity-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let source_path = dir.join("parity.rs");
        let binary_path = dir.join(if cfg!(windows) { "parity.exe" } else { "parity" });
        std::fs::write(&source_path, program).expect("write parity program");

        let compile = std::process::Command::new("rustc")
            .arg("-O")
            .arg("-o")
            .arg(&binary_path)
            .arg(&source_path)
            .output()
            .expect("run rustc");
        assert!(
            compile.status.success(),
            "packed body did not compile:\n{}",
            String::from_utf8_lossy(&compile.stderr)
        );

        let run = std::process::Command::new(&binary_path)
            .output()
            .expect("run parity program");
        assert!(run.status.success(), "parity program failed");
        let printed = String::from_utf8_lossy(&run.stdout);
        let actual: Vec<f64> = printed
            .trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(", ")
            .map(|token| token.parse().expect("parse derivative"))
            .collect();

        let _ = std::fs::remove_dir_all(&dir);

        assert_eq!(actual.len(), expected.len(), "lane count");
        for (lane, (got, want)) in actual.iter().zip(expected.iter()).enumerate() {
            let tolerance = want.abs() * 1e-12 + 1e-15;
            assert!(
                (got - want).abs() <= tolerance,
                "lane {lane}: packed {got:e} vs reference {want:e}"
            );
        }
    }

    #[test]
    fn the_newton_body_is_a_small_fraction_of_the_gate_model() {
        // The point of splitting by invalidation class. A single body does all
        // of bsimbulk's parameter and geometry preprocessing on every call,
        // which is not what the existing backends do and not what their
        // published per-stamp numbers measure.
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../models/veriloga/cmc/BSIM-BULK107.2.1_02112025/code/bsimbulk.va");
        let source = std::fs::read_to_string(&path).expect("read bsimbulk");
        let mut options = CompilerOptions::default();
        options
            .include_paths
            .push(path.parent().expect("model directory").to_path_buf());
        let artifact = VerilogACompiler::new(options)
            .compile_canonical_ir_module(&source, Some("bsimbulk"))
            .expect("compile bsimbulk");
        let opt = OptModel::primal_from_hir_and_mir(&artifact.hir, &artifact.mir)
            .expect("primal OptIR");

        let differentiated = differentiated_values(&opt);
        let ctx = test_context();
        let schedule = emit_scheduled(
            &opt,
            17,
            &|kind| match kind {
                OptValueKind::NodePotential { node } => Some(usize::from(*node)),
                _ => None,
            },
            &differentiated,
            &ctx,
        )
        .expect("emit scheduled bodies");

        let describe = |label: &str, body: &PackedBody| {
            eprintln!(
                "{label:<20} primal={:<6} derivative={:<6} source={}KB",
                body.primal_bindings,
                body.derivative_bindings,
                body.source.len() / 1024
            );
        };
        describe("instance-static", &schedule.instance_static);
        describe("temperature-static", &schedule.temperature_static);
        describe("newton", &schedule.newton);

        if let Ok(path) = std::env::var("RSPICE_PACKED_SPLIT_DUMP") {
            std::fs::write(format!("{path}.static"), &schedule.instance_static.source)
                .expect("dump static body");
            std::fs::write(
                format!("{path}.temperature"),
                &schedule.temperature_static.source,
            )
            .expect("dump temperature body");
            std::fs::write(format!("{path}.newton"), &schedule.newton.source)
                .expect("dump newton body");
        }

        let total = schedule.instance_static.primal_bindings
            + schedule.temperature_static.primal_bindings
            + schedule.newton.primal_bindings;

        // Nothing may be lost in the split; a value emitted into no body is a
        // dropped term, which compiles fine and converges to the wrong answer.
        assert_eq!(
            total,
            opt.values.len() - loop_dependent_values(&opt).len(),
            "the split must place every value that is not loop interior"
        );

        // Hoisting helps less here than the schedule listing suggests. Only 85
        // ops appear in the Newton schedule, but that names one op per
        // equation, not the per-iteration dataflow: everything downstream of a
        // node voltage is genuinely per-iteration, which in a MOSFET is most of
        // the current computation. The static split is worth having, not
        // decisive.
        assert!(
            schedule.instance_static.primal_bindings > total / 5,
            "instance-static work should be a real share: {} of {total}",
            schedule.instance_static.primal_bindings
        );
        assert_eq!(
            schedule.instance_static.derivative_bindings, 0,
            "static values cannot depend on an unknown, so none may carry a derivative array"
        );
    }

    #[test]
    fn unsupported_kinds_fail_loudly_rather_than_emitting_something_wrong() {
        // A model reaching a kind with no rule must stop generation. Silently
        // skipping it would drop a term from the Jacobian, which no test of the
        // emitted source would catch.
        let opt = primal_of(
            r#"
module packed_transient(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ ddt(V(p, n)) + $abstime;
    end
endmodule
"#,
        );
        let differentiated = differentiated_values(&opt);
        let result = emit_body(
            &opt,
            2,
            &|_| None,
            &differentiated,
            &test_context(),
        );
        assert!(
            result.is_err(),
            "$abstime has no packed rule yet and must be reported"
        );
        let message = result.unwrap_err();
        assert!(message.contains("Time"), "{message}");
    }

    #[test]
    fn exp_scales_by_its_own_result_not_a_recomputed_exp() {
        // Recomputing exp(x) here would round differently from the primal and
        // desynchronize the two backends.
        let rule = unary_rule(OptUnaryOp::Exp, "v3", "v7");
        assert_eq!(
            rule,
            LaneRule::Scaled {
                factor: "v7".to_string()
            }
        );
    }

    #[test]
    fn logarithm_divides_rather_than_scaling_by_a_reciprocal() {
        // x / y and x * (1/y) are not the same in binary floating point, and
        // the scalarized rules divide.
        let rule = unary_rule(OptUnaryOp::Ln, "v3", "v7");
        assert_eq!(
            rule,
            LaneRule::Divided {
                divisor: "v3".to_string()
            }
        );
        let emitted = rule.emit(4, "d3", "", "").expect("ln has a derivative");
        assert!(emitted.contains("d3[k] / q"), "{emitted}");
        assert!(!emitted.contains("* q"), "{emitted}");
    }

    #[test]
    fn abs_has_no_derivative() {
        // The scalarized rules refuse to pick a subgradient at the kink.
        // Diverging would silently change every model that takes an absolute
        // value, so this must stay Zero rather than become sign(x).
        assert_eq!(unary_rule(OptUnaryOp::Abs, "v1", "v2"), LaneRule::Zero);
        assert_eq!(unary_rule(OptUnaryOp::Floor, "v1", "v2"), LaneRule::Zero);
        assert_eq!(unary_rule(OptUnaryOp::Ceil, "v1", "v2"), LaneRule::Zero);
    }

    #[test]
    fn comparisons_and_modulo_have_no_derivative() {
        for op in [
            OptBinaryOp::Mod,
            OptBinaryOp::Eq,
            OptBinaryOp::Lt,
            OptBinaryOp::And,
            OptBinaryOp::Or,
        ] {
            assert_eq!(binary_rule(op, "a", "b", "v"), LaneRule::Zero);
        }
    }

    #[test]
    fn product_rule_keeps_the_scalarized_operand_order() {
        let rule = binary_rule(OptBinaryOp::Mul, "va", "vb", "vc");
        assert_eq!(
            rule,
            LaneRule::Binary {
                lane_expr: "l[k] * vb + va * r[k]".to_string()
            }
        );
    }

    #[test]
    fn quotient_rule_reuses_the_quotient_it_already_computed() {
        let rule = binary_rule(OptBinaryOp::Div, "va", "vb", "vq");
        assert_eq!(
            rule,
            LaneRule::Binary {
                lane_expr: "(l[k] - vq * r[k]) / vb".to_string()
            }
        );
    }

    #[test]
    fn emitted_arrays_carry_their_width() {
        // A `from_fn` without an explicit width infers it from context, which
        // silently compiles to the wrong length if a binding's type changes.
        let scaled = unary_rule(OptUnaryOp::Exp, "v1", "v2")
            .emit(19, "d1", "", "")
            .expect("exp has a derivative");
        assert!(scaled.contains("from_fn::<f64, 19, _>"), "{scaled}");

        let binary = binary_rule(OptBinaryOp::Add, "a", "b", "v")
            .emit(7, "", "da", "db")
            .expect("add has a derivative");
        assert!(binary.contains("from_fn::<f64, 7, _>"), "{binary}");
        assert!(binary.contains("let l = &da;"), "{binary}");
        assert!(binary.contains("let r = &db;"), "{binary}");
    }

    #[test]
    fn the_three_unknown_sources_seed_derivatives() {
        use crate::canonical_ir::{BranchId, BranchUnknownId, NodeId};
        assert_eq!(
            dependence(&OptValueKind::NodePotential {
                node: NodeId::from(0)
            }),
            Dependence::Seed
        );
        assert_eq!(
            dependence(&OptValueKind::BranchFlow {
                branch: BranchId::from(0)
            }),
            Dependence::Seed
        );
        assert_eq!(
            dependence(&OptValueKind::BranchUnknownFlow {
                branch_unknown: BranchUnknownId::from(0)
            }),
            Dependence::Seed
        );
    }

    #[test]
    fn derivative_killing_operators_stop_propagation() {
        // A comparison over a node potential is still independent of every
        // unknown. Treating it as dependent would emit a derivative array for
        // every guard in the model.
        assert_eq!(
            dependence(&OptValueKind::Binary {
                op: OptBinaryOp::Lt,
                left: ValueId::from(0),
                right: ValueId::from(1),
            }),
            Dependence::None
        );
        assert_eq!(
            dependence(&OptValueKind::Unary {
                op: OptUnaryOp::Floor,
                input: ValueId::from(0),
            }),
            Dependence::None
        );
    }

    #[test]
    fn ddx_and_previous_iterate_break_the_chain() {
        // `ddx` reports a derivative rather than carrying one, and Newton holds
        // the previous iterate fixed. Both must terminate propagation or the
        // Jacobian picks up terms the reference does not have.
        assert_eq!(
            dependence(&OptValueKind::Ddx {
                value: ValueId::from(0),
                pos_node: None,
                neg_node: None,
            }),
            Dependence::None
        );
        assert_eq!(
            dependence(&OptValueKind::LimitPrevious {
                operator: crate::canonical_ir::ExprId::from(0),
                proposed: ValueId::from(0),
            }),
            Dependence::None
        );
    }

    #[test]
    fn limit_is_differentiated_even_when_its_input_is_not() {
        // $limit contributes an affine correction lane regardless, so it cannot
        // be gated on the proposed value depending on an unknown.
        assert_eq!(
            dependence(&OptValueKind::Limit {
                operator: crate::canonical_ir::ExprId::from(0),
                proposed: ValueId::from(0),
                candidate: ValueId::from(1),
            }),
            Dependence::Always
        );
    }

    #[test]
    fn select_propagates_through_arms_not_its_condition() {
        // The condition is boolean; only the arms can carry a derivative.
        let operands = differentiating_operands(&OptValueKind::Select {
            condition: ValueId::from(7),
            then_value: ValueId::from(8),
            else_value: ValueId::from(9),
        });
        assert_eq!(operands, vec![ValueId::from(8), ValueId::from(9)]);
    }

    #[test]
    fn a_seed_is_a_unit_vector_at_its_own_lane() {
        let emitted = seed_rule(Some(3))
            .emit(5, "", "", "")
            .expect("a seed has a derivative");
        assert!(emitted.contains("[0.0f64; 5]"), "{emitted}");
        assert!(emitted.contains("d[3] = 1.0"), "{emitted}");
    }

    #[test]
    fn a_seed_on_an_unpacked_lane_has_no_derivative() {
        // A branch flow whose branch carries no unknown. The scalarized rules
        // return an empty map here rather than a zero derivative, and emitting
        // a zero array instead would cost a binding per such value.
        assert_eq!(seed_rule(None), LaneRule::Zero);
        assert_eq!(seed_rule(None).emit(8, "d", "l", "r"), None);
    }

    #[test]
    fn select_branches_outside_the_lane_loop() {
        // Selecting per lane would put a branch inside the unrolled body for a
        // condition that cannot vary across lanes.
        let emitted = select_rule("c1 != 0.0")
            .emit(12, "", "dt", "de")
            .expect("select has a derivative");
        assert_eq!(emitted, "(if c1 != 0.0 { dt } else { de })");
        assert!(!emitted.contains("from_fn"), "{emitted}");
    }

    #[test]
    fn ddt_scales_every_lane_by_the_integration_factor() {
        let rule = ddt_rule("ddt_scale");
        assert_eq!(
            rule,
            LaneRule::Scaled {
                factor: "ddt_scale".to_string()
            }
        );
        let emitted = rule.emit(6, "din", "", "").expect("ddt has a derivative");
        assert!(emitted.contains("let s = ddt_scale;"), "{emitted}");
        assert!(emitted.contains("s * din[k]"), "{emitted}");
    }

    #[test]
    fn zero_rules_emit_nothing_at_all() {
        // Not an array of zeros: a value with no dependence on any unknown
        // should cost no binding, which is most of a compact model's
        // temperature and geometry arithmetic.
        assert_eq!(unary_rule(OptUnaryOp::Floor, "a", "b").emit(8, "d", "", ""), None);
        assert_eq!(
            binary_rule(OptBinaryOp::Gt, "a", "b", "v").emit(8, "", "l", "r"),
            None
        );
    }
}
