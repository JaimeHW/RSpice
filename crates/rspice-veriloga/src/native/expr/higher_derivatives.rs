//! Higher derivatives of canonical expressions, using the shared algebra AD.
//!
//! Only pure expression structure is imported. Opaque leaves retain the MIR
//! expression identity and ordered derivative axes; their loads and state
//! actions are resolved by the same canonical lowerer as ordinary entries.
//! No executable bytecode contributes an equation or an algebraic operand.

use super::*;
use crate::ast::{BinaryOp, UnaryOp};
use crate::ir::IrFunction;
use crate::ir::arena::{ExprArena, NameId, Node, NodeId as AlgebraId, visit};
use crate::ir::autodiff::differentiate_with_variable_derivatives;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Leaf {
    expression: ExprId,
    axes: Vec<CanonicalDerivativeAxis>,
}

#[derive(Default)]
struct Algebra {
    arena: ExprArena,
    leaves: HashMap<NameId, Leaf>,
    leaf_nodes: HashMap<Leaf, AlgebraId>,
    imported: HashMap<ExprId, AlgebraId>,
}

impl Algebra {
    fn leaf(
        &mut self,
        lowerer: &MirEquationLowerer<'_, '_>,
        expression: ExprId,
        axes: &[CanonicalDerivativeAxis],
    ) -> JitResult<AlgebraId> {
        let leaf = Leaf {
            expression,
            axes: axes.to_vec(),
        };
        if let Some(&id) = self.leaf_nodes.get(&leaf) {
            return Ok(id);
        }
        let id = if let Some(value) = lowerer.opaque_derivative_constant(expression, axes)? {
            self.arena.push(Node::Const(value))
        } else {
            // These names never enter a model's storage namespace.
            let name = self
                .arena
                .intern(&format!("mir_leaf_{}", self.leaves.len()));
            self.leaves.insert(name, leaf.clone());
            self.arena.push(Node::Var(name))
        };
        self.leaf_nodes.insert(leaf, id);
        Ok(id)
    }

    fn import(&mut self, lowerer: &MirEquationLowerer<'_, '_>, id: ExprId) -> JitResult<AlgebraId> {
        if let Some(&imported) = self.imported.get(&id) {
            return Ok(imported);
        }
        let node = match &lowerer.expression(id)?.kind {
            HirExprKind::Number { value, .. } => Node::Const(*value),
            HirExprKind::Unary { op, operand } => {
                let inner = self.import(lowerer, *operand)?;
                match op.as_str() {
                    "Pos" => Node::Unary(UnaryOp::Pos, inner),
                    "Neg" => Node::Unary(UnaryOp::Neg, inner),
                    "Not" => Node::Unary(UnaryOp::Not, inner),
                    "BitNot" => Node::Unary(UnaryOp::BitNot, inner),
                    "ToInteger" => Node::Unary(UnaryOp::ToInteger, inner),
                    FROZEN_DERIVATIVE_UNARY => Node::FreezeDerivative(inner),
                    _ => return Err(lowerer.unsupported(format!("unary operator {op}"))),
                }
            }
            HirExprKind::Binary { op, left, right } => {
                let op = binary_operator(op)
                    .ok_or_else(|| lowerer.unsupported(format!("binary operator {op}")))?;
                let left = self.import(lowerer, *left)?;
                let right = self.import(lowerer, *right)?;
                Node::Binary(op, left, right)
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                let condition = self.import(lowerer, *condition)?;
                let then_expr = self.import(lowerer, *then_expr)?;
                let else_expr = self.import(lowerer, *else_expr)?;
                Node::Conditional(condition, then_expr, else_expr)
            }
            HirExprKind::Call { name, args } | HirExprKind::SystemFunction { name, args }
                if math_function(name).is_some() =>
            {
                let function = math_function(name).unwrap();
                let arity = if matches!(
                    function,
                    IrFunction::Atan2
                        | IrFunction::Hypot
                        | IrFunction::Min
                        | IrFunction::Max
                        | IrFunction::Pow
                ) {
                    2
                } else {
                    1
                };
                lowerer.require_intrinsic_arity(name, args, arity)?;
                let args = args
                    .iter()
                    .map(|&arg| self.import(lowerer, arg))
                    .collect::<JitResult<Vec<_>>>()?;
                let node = if matches!(function, IrFunction::Atan2 | IrFunction::Hypot) {
                    // Normalize the primal before repeated AD. Normalizing
                    // only a first derivative leaves its unscaled numerator
                    // exposed to overflow when differentiated again (e.g.
                    // d^3 atan2(1e308*x, 1e308) at x=0 is -2, not -infinity).
                    // Homogeneity makes every derivative independent of this
                    // positive scale, so hold it fixed in every AD pass.
                    let a = self.arena.push_call(IrFunction::Abs, &[args[0]]);
                    let b = self.arena.push_call(IrFunction::Abs, &[args[1]]);
                    let scale = self.arena.push_call(IrFunction::Max, &[a, b]);
                    let scale = self.arena.push(Node::FreezeDerivative(scale));
                    let a = self.arena.push(Node::Binary(BinaryOp::Div, args[0], scale));
                    let b = self.arena.push(Node::Binary(BinaryOp::Div, args[1], scale));
                    let normalized = self.arena.push_call(function, &[a, b]);
                    if function == IrFunction::Hypot {
                        self.arena
                            .push(Node::Binary(BinaryOp::Mul, scale, normalized))
                    } else {
                        normalized
                    }
                } else {
                    self.arena.push_call(function, &args)
                };
                self.imported.insert(id, node);
                return Ok(node);
            }
            HirExprKind::Call { name, args } if normalize_intrinsic_name(name) == "limexp" => {
                lowerer.require_intrinsic_arity(name, args, 1)?;
                Node::Limexp(self.import(lowerer, args[0])?)
            }
            _ => return self.leaf(lowerer, id, &[]),
        };
        let imported = self.arena.push(node);
        self.imported.insert(id, imported);
        Ok(imported)
    }

    fn differentiate(
        &mut self,
        lowerer: &MirEquationLowerer<'_, '_>,
        mut root: AlgebraId,
        axes: &[CanonicalDerivativeAxis],
    ) -> JitResult<AlgebraId> {
        for &axis in axes {
            let mut names = HashSet::new();
            visit(&self.arena, root, &mut |node| {
                if let Node::Var(name) = node {
                    names.insert(*name);
                }
            });
            let mut names = names.into_iter().collect::<Vec<_>>();
            names.sort_by_key(|name| name.index());
            let mut derivatives = HashMap::new();
            for name in names {
                let mut leaf = self.leaves[&name].clone();
                leaf.axes.push(axis);
                let derivative = self.leaf(lowerer, leaf.expression, &leaf.axes)?;
                derivatives.insert(name, derivative);
            }
            root = differentiate_with_variable_derivatives(&mut self.arena, root, &derivatives);
        }
        Ok(root)
    }

    fn emit(&self, lowerer: &mut MirEquationLowerer<'_, '_>, id: AlgebraId) -> JitResult<()> {
        match *self.arena.node(id) {
            Node::Const(value) => lowerer.push(NativeOp::Const(value)),
            Node::Var(name) => {
                let leaf = &self.leaves[&name];
                lowerer.lower_opaque_mixed_derivative(leaf.expression, &leaf.axes)
            }
            Node::FreezeDerivative(inner) => self.emit(lowerer, inner),
            Node::Unary(op, inner) => {
                self.emit(lowerer, inner)?;
                match op {
                    UnaryOp::Pos => Ok(()),
                    UnaryOp::Neg => lowerer.append_unary(NativeOp::Neg),
                    UnaryOp::Not => lowerer.append_unary(NativeOp::Logical(LogicalOp::Not)),
                    UnaryOp::ToInteger => lowerer.append_unary(NativeOp::IntegerCast),
                    UnaryOp::BitNot => {
                        lowerer.push(NativeOp::Const(-1.0))?;
                        lowerer.append_integer_binary("BitXor")
                    }
                }
            }
            Node::Binary(op, left, right) => {
                self.emit(lowerer, left)?;
                self.emit(lowerer, right)?;
                let name = format!("{op:?}");
                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                        lowerer.append_arithmetic(&name)
                    }
                    BinaryOp::Pow | BinaryOp::Mod => lowerer.append_binary_math(&name),
                    BinaryOp::CheckedValue => lowerer.append_checked_value(),
                    BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge => lowerer.append_compare(&name),
                    BinaryOp::And | BinaryOp::Or => lowerer.append_logical(&name),
                    _ => lowerer.append_integer_binary(&name),
                }
            }
            Node::Conditional(condition, then_expr, else_expr) => {
                self.emit(lowerer, condition)?;
                self.emit(lowerer, then_expr)?;
                self.emit(lowerer, else_expr)?;
                lowerer.append_ifelse()
            }
            Node::Call { func, a, b, .. } => {
                let args = a.into_iter().chain(b).collect::<Vec<_>>();
                self.emit_call(lowerer, func, &args)
            }
            Node::CallSpilled { func, args } => {
                self.emit_call(lowerer, func, self.arena.call_args(args))
            }
            Node::Limexp(inner) => {
                self.emit(lowerer, inner)?;
                lowerer.append_unary(NativeOp::UnaryMath(UnaryMathOp::Limexp))
            }
            other => Err(lowerer.unsupported(format!(
                "shared algebra produced non-algebraic node {other:?}"
            ))),
        }
    }

    fn emit_call(
        &self,
        lowerer: &mut MirEquationLowerer<'_, '_>,
        function: IrFunction,
        args: &[AlgebraId],
    ) -> JitResult<()> {
        for &arg in args {
            self.emit(lowerer, arg)?;
        }
        let unary = match function {
            IrFunction::Abs => return lowerer.append_unary(NativeOp::Abs),
            IrFunction::Sqrt => return lowerer.append_unary(NativeOp::Sqrt),
            IrFunction::Min => return lowerer.append_extremum(ExtremumOp::Min),
            IrFunction::Max => return lowerer.append_extremum(ExtremumOp::Max),
            IrFunction::Pow => return lowerer.append_binary_math("Pow"),
            IrFunction::Atan2 | IrFunction::Hypot => {
                lowerer.pop_binary("shared algebra function")?;
                return lowerer.append_binary_math_op(if function == IrFunction::Atan2 {
                    BinaryMathOp::Atan2
                } else {
                    BinaryMathOp::Hypot
                });
            }
            IrFunction::SumProductsDiv => {
                return lowerer.append_sum_products_div((args.len() - 1) / 2);
            }
            IrFunction::Exp => UnaryMathOp::Exp,
            IrFunction::LimitedExp => UnaryMathOp::LimitedExp,
            IrFunction::Log => UnaryMathOp::Log,
            IrFunction::Log10 => UnaryMathOp::Log10,
            IrFunction::Sin => UnaryMathOp::Sin,
            IrFunction::Cos => UnaryMathOp::Cos,
            IrFunction::Tan => UnaryMathOp::Tan,
            IrFunction::Sinh => UnaryMathOp::Sinh,
            IrFunction::Cosh => UnaryMathOp::Cosh,
            IrFunction::Tanh => UnaryMathOp::Tanh,
            IrFunction::Asin => UnaryMathOp::Asin,
            IrFunction::Acos => UnaryMathOp::Acos,
            IrFunction::Atan => UnaryMathOp::Atan,
            IrFunction::Asinh => UnaryMathOp::Asinh,
            IrFunction::Acosh => UnaryMathOp::Acosh,
            IrFunction::Atanh => UnaryMathOp::Atanh,
            IrFunction::Floor => UnaryMathOp::Floor,
            IrFunction::Ceil => UnaryMathOp::Ceil,
        };
        lowerer.append_unary(NativeOp::UnaryMath(unary))
    }
}

impl MirEquationLowerer<'_, '_> {
    pub(super) fn lower_mixed_derivative(
        &mut self,
        expression: ExprId,
        axes: &[CanonicalDerivativeAxis],
    ) -> JitResult<()> {
        match axes {
            [] => self.lower(expression),
            [axis] => self.lower_derivative(expression, *axis),
            [first, second] => self.lower_second_derivative(expression, *first, *second),
            _ => {
                let mut algebra = Algebra::default();
                let root = algebra.import(self, expression)?;
                let derivative = algebra.differentiate(self, root, axes)?;
                algebra.emit(self, derivative)
            }
        }
    }

    fn opaque_derivative_constant(
        &self,
        expression: ExprId,
        axes: &[CanonicalDerivativeAxis],
    ) -> JitResult<Option<f64>> {
        if axes.is_empty() {
            return Ok(None);
        }
        match &self.expression(expression)?.kind {
            HirExprKind::Identifier { name } => {
                let constant = match self.limits.identifier_slot(self.mir, name) {
                    Some(NativeIdentifierSlot::Parameter(_)) => true,
                    Some(NativeIdentifierSlot::Variable(_)) => self
                        .limits
                        .identifier_slot(self.mir, &shadow_name(name, axes))
                        .is_none(),
                    None => {
                        return Err(self.unsupported(format!("derivative of identifier {name}")));
                    }
                };
                Ok(constant.then_some(0.0))
            }
            HirExprKind::ArrayAccess { array, .. } => Ok(self
                .resolve_mixed_array_shadow(array, axes)?
                .is_none()
                .then_some(0.0)),
            HirExprKind::BranchAccess { .. } | HirExprKind::NamedBranchAccess { .. } => {
                if axes.len() > 1 {
                    return Ok(Some(0.0));
                }
                let mut leaf = MirEquationLowerer::new(
                    self.model.clone(),
                    self.entry_kind,
                    self.mir,
                    self.equation_id,
                    self.limits,
                );
                leaf.lower_derivative(expression, axes[0])?;
                match leaf.ops.as_slice() {
                    [NativeOp::Const(value)] => Ok(Some(*value)),
                    _ => Ok(None),
                }
            }
            // Zi must retain its scheduling action even for a zero payload.
            HirExprKind::Call { name, .. }
                if matches!(
                    normalize_intrinsic_name(name).as_str(),
                    "zi_zp" | "zi_zd" | "zi_np" | "zi_nd"
                ) =>
            {
                Ok(None)
            }
            _ => Ok(self
                .expr_derivative_is_zero(expression, axes[0])?
                .then_some(0.0)),
        }
    }

    fn resolve_mixed_array_shadow(
        &self,
        array: &str,
        axes: &[CanonicalDerivativeAxis],
    ) -> JitResult<Option<(usize, usize, i64)>> {
        self.resolve_variable_range_with_affixes(
            array,
            &format!("{array}["),
            &shadow_name("]", axes),
        )
    }

    fn lower_opaque_mixed_derivative(
        &mut self,
        expression: ExprId,
        axes: &[CanonicalDerivativeAxis],
    ) -> JitResult<()> {
        if axes.len() < 3 {
            return self.lower_mixed_derivative(expression, axes);
        }
        match &self.expression(expression)?.kind {
            HirExprKind::Identifier { name } => self.lower_identifier(&shadow_name(name, axes)),
            HirExprKind::ArrayAccess { array, index } => {
                let Some((base, len, lower)) = self.resolve_mixed_array_shadow(array, axes)? else {
                    return self.push(NativeOp::Const(0.0));
                };
                validate_range(
                    self.model.clone(),
                    "canonical array mixed derivative",
                    base,
                    len,
                    self.limits.variable_count,
                )?;
                self.lower(*index)?;
                if !lower_constant_dynamic_variable_read(&mut self.ops, base, len, lower) {
                    self.ops
                        .push(NativeOp::LoadVariableDyn { base, len, lower });
                }
                Ok(())
            }
            HirExprKind::Call { name, args } | HirExprKind::SystemFunction { name, args } => {
                self.lower_opaque_call_mixed_derivative(expression, name, args, axes)
            }
            HirExprKind::AnalogOperator {
                op:
                    HirAnalogOperator::Limit {
                        proposed,
                        type_metadata,
                        ..
                    },
            } => {
                if let Some(metadata) = type_metadata {
                    let mut algebra = Algebra::default();
                    let metadata = algebra.import(self, *metadata)?;
                    let proposed = algebra.import(self, *proposed)?;
                    let root = algebra
                        .arena
                        .push(Node::Binary(BinaryOp::Mul, metadata, proposed));
                    let derivative = algebra.differentiate(self, root, axes)?;
                    algebra.emit(self, derivative)
                } else {
                    self.lower_mixed_derivative(*proposed, axes)
                }
            }
            _ => Err(self.unsupported(format!(
                "mixed derivative of canonical expression {expression}"
            ))),
        }
    }

    pub(super) fn lower_ddx_projection_mixed(
        &mut self,
        expression: ExprId,
        probe: ExprId,
        axes: &[CanonicalDerivativeAxis],
    ) -> JitResult<()> {
        self.lower(expression)?;
        let mut projected = Vec::with_capacity(axes.len() + 1);
        projected.push(CanonicalDerivativeAxis::Node(NodeId::new(0)));
        projected.extend_from_slice(axes);
        match self.ddx_probe_projection(probe)? {
            DdxProjection::Potential(pos, neg) => {
                if let Some(pos) = pos {
                    projected[0] = CanonicalDerivativeAxis::Node(pos);
                    self.lower_mixed_derivative(expression, &projected)?;
                } else {
                    self.push(NativeOp::Const(0.0))?;
                }
                if let Some(neg) = neg {
                    projected[0] = CanonicalDerivativeAxis::Node(neg);
                    self.lower_mixed_derivative(expression, &projected)?;
                    self.append_arithmetic("Sub")?;
                    if pos.is_some() {
                        // Only two live endpoints move symmetrically.
                        // Ground is fixed, including a ground-first probe.
                        self.push(NativeOp::Const(0.5))?;
                        self.append_arithmetic("Mul")?;
                    }
                }
            }
            DdxProjection::Branch {
                runtime_index,
                inverted,
            } => {
                projected[0] = CanonicalDerivativeAxis::Branch(runtime_index);
                self.lower_mixed_derivative(expression, &projected)?;
                if inverted {
                    self.append_unary(NativeOp::Neg)?;
                }
            }
        }
        self.append_checked_value()
    }

    fn lower_opaque_call_mixed_derivative(
        &mut self,
        expression: ExprId,
        name: &str,
        args: &[ExprId],
        axes: &[CanonicalDerivativeAxis],
    ) -> JitResult<()> {
        match normalize_intrinsic_name(name).as_str() {
            "ddt" => self.lower_ddt_derivative(expression, name, args, axes),
            "idt" | "idtmod" => self.lower_idt_derivative(expression, name, args, axes),
            "transition" => self.lower_transition_operator(expression, args, Some(axes)),
            "slew" => {
                self.require_intrinsic_arity_range(name, args, 1, 3)?;
                self.lower_slew_derivative_operator(
                    expression,
                    args[0],
                    args.get(1).copied(),
                    args.get(2).copied(),
                    axes,
                )
            }
            "zi_zp" | "zi_zd" | "zi_np" | "zi_nd" => {
                self.lower_zi_call_derivative(expression, name, args, axes)
            }
            "limit" => {
                self.require_intrinsic_arity_range(name, args, 1, 2)?;
                self.lower_mixed_derivative(args[0], axes)
            }
            "ddx" => {
                self.require_intrinsic_arity(name, args, 2)?;
                self.lower_ddx_projection_mixed(args[0], args[1], axes)
            }
            "vt" | "thermal_vt" => self.lower_thermal_voltage_intrinsic(
                name,
                args,
                NativeOp::Const(0.0),
                |this, argument| this.lower_mixed_derivative(argument, axes),
            ),
            "simparam" => self.lower_simparam_action(name, args, true, |this, fallback| {
                this.lower_mixed_derivative(fallback, axes)
            }),
            "laplace_zp" | "laplace_zd" | "laplace_np" | "laplace_nd" => {
                self.require_intrinsic_arity(name, args, 3)?;
                let slot = self.laplace_slot(expression)?;
                self.lower_mixed_derivative(args[0], axes)?;
                self.append_unary(NativeOp::LaplaceStateDerivative(slot))
            }
            _ => Err(self.unsupported(format!(
                "mixed derivative of intrinsic function '{name}' at {expression}"
            ))),
        }
    }
}

fn shadow_name(name: &str, axes: &[CanonicalDerivativeAxis]) -> String {
    let mut name = name.to_owned();
    for axis in axes {
        name.push('@');
        name.push_str(&axis.shadow_suffix());
    }
    name
}

fn binary_operator(name: &str) -> Option<BinaryOp> {
    Some(match name {
        "Add" => BinaryOp::Add,
        "Sub" => BinaryOp::Sub,
        "Mul" => BinaryOp::Mul,
        "Div" => BinaryOp::Div,
        "Mod" => BinaryOp::Mod,
        "Pow" => BinaryOp::Pow,
        "IntAdd" => BinaryOp::IntAdd,
        "IntSub" => BinaryOp::IntSub,
        "IntMul" => BinaryOp::IntMul,
        "IntDiv" => BinaryOp::IntDiv,
        "IntMod" => BinaryOp::IntMod,
        "IntPow" => BinaryOp::IntPow,
        "Eq" => BinaryOp::Eq,
        "Ne" => BinaryOp::Ne,
        "Lt" => BinaryOp::Lt,
        "Le" => BinaryOp::Le,
        "Gt" => BinaryOp::Gt,
        "Ge" => BinaryOp::Ge,
        "And" => BinaryOp::And,
        "Or" => BinaryOp::Or,
        "BitAnd" => BinaryOp::BitAnd,
        "BitOr" => BinaryOp::BitOr,
        "BitXor" => BinaryOp::BitXor,
        "Shl" => BinaryOp::Shl,
        "Shr" => BinaryOp::Shr,
        "CheckedValue" => BinaryOp::CheckedValue,
        _ => return None,
    })
}

fn math_function(name: &str) -> Option<IrFunction> {
    Some(match normalize_intrinsic_name(name).as_str() {
        "abs" | "fabs" => IrFunction::Abs,
        "sqrt" => IrFunction::Sqrt,
        "exp" => IrFunction::Exp,
        "__rspice_limited_exp" => IrFunction::LimitedExp,
        "ln" | "log" => IrFunction::Log,
        "log10" => IrFunction::Log10,
        "sin" => IrFunction::Sin,
        "cos" => IrFunction::Cos,
        "tan" => IrFunction::Tan,
        "sinh" => IrFunction::Sinh,
        "cosh" => IrFunction::Cosh,
        "tanh" => IrFunction::Tanh,
        "asin" => IrFunction::Asin,
        "acos" => IrFunction::Acos,
        "atan" => IrFunction::Atan,
        "asinh" => IrFunction::Asinh,
        "acosh" => IrFunction::Acosh,
        "atanh" => IrFunction::Atanh,
        "atan2" => IrFunction::Atan2,
        "hypot" => IrFunction::Hypot,
        "floor" => IrFunction::Floor,
        "ceil" => IrFunction::Ceil,
        "min" => IrFunction::Min,
        "max" => IrFunction::Max,
        "pow" => IrFunction::Pow,
        _ => return None,
    })
}
