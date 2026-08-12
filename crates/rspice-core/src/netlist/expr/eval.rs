//! Walks an [`Expr`] and produces a value.
//!
//! Holds the built-in function table and the complex-arithmetic helpers it
//! is built on. Two behaviors here are dialect-sensitive and silently change
//! results rather than erroring: `LOG` is the natural logarithm except under
//! [`ExpressionDialect::Xyce`], where it is base 10, and `PH`/`PHASE`
//! returns degrees. Both are pinned by oracle tests.
use crate::config::ExpressionDialect;

use super::*;
use std::collections::{HashMap, HashSet};

const MAX_EVAL_FUNCTION_CALL_DEPTH: usize = 4096;
const XYCE_ATANH_EPSILON: Value = 1.0e-12;
const XYCE_NONFINITE_REPLACEMENT: Value = 1.0e50;
const XYCE_TANH_SATURATION_THRESHOLD: Value = 20.0;

/// Evaluate an expression with the given context
pub fn evaluate(expr: &Expr, ctx: &ParamContext) -> Result<Value, ExprError> {
    evaluate_complex(expr, ctx).map(ComplexValue::real_projection)
}

/// Evaluate an expression with the given context, preserving complex values.
pub fn evaluate_complex(expr: &Expr, ctx: &ParamContext) -> Result<ComplexValue, ExprError> {
    let value = evaluate_complex_raw(expr, ctx)?;
    Ok(if ctx.expression_dialect() == ExpressionDialect::Xyce {
        normalize_xyce_expression_result(value)
    } else {
        value
    })
}

/// Evaluate without applying the dialect's public root normalization.
///
/// Most expression consumers must use [`evaluate_complex`]. Measurement
/// execution is the exception: Xyce distinguishes a raw `MeasureOp` getter
/// from an authored `ExpressionOp` root, so that layer applies normalization
/// only when the parsed measure operand carries expression provenance.
pub(crate) fn evaluate_complex_raw(
    expr: &Expr,
    ctx: &ParamContext,
) -> Result<ComplexValue, ExprError> {
    ExpressionEvaluator::new(ctx).evaluate(expr)
}

/// Compile an expression into an index-based program that can be evaluated
/// repeatedly without cloning its AST or allocating evaluator stacks. Runtime
/// parameter reads are exposed through a resolver so live measurements can
/// implement Xyce's first-read semantics at the exact lazy evaluation point.
#[derive(Debug)]
pub(crate) struct PreparedExpression {
    programs: Vec<PreparedProgram>,
    root: PreparedNodeRef,
    frames: Vec<PreparedEvalFrame>,
    values: Vec<EvaluatedValue>,
    numeric_args: Vec<ComplexValue>,
    call_scopes: Vec<PreparedCallScope>,
    arg_bindings: Vec<PreparedArgBinding>,
}

#[derive(Debug)]
struct PreparedProgram {
    nodes: Vec<PreparedNode>,
    root: usize,
    formal_args: Vec<String>,
}

#[derive(Debug)]
enum PreparedNode {
    Number(Value),
    ComplexNumber(ComplexValue),
    StringLiteral(String),
    Param {
        name: String,
        formal_index: Option<usize>,
    },
    External(String),
    Unary {
        op: UnaryOpKind,
        operand: usize,
    },
    Binary {
        op: BinOpKind,
        left: usize,
        right: usize,
    },
    Function {
        name: String,
        args: Vec<usize>,
        user_program: Option<usize>,
    },
}

#[derive(Debug, Clone, Copy)]
struct PreparedNodeRef {
    program: usize,
    node: usize,
}

#[derive(Debug, Clone, Copy)]
struct PreparedArgBinding {
    expression: PreparedNodeRef,
    caller_scope: Option<usize>,
}

#[derive(Debug)]
struct PreparedCallScope {
    function_program: usize,
    bindings_start: usize,
    bindings_len: usize,
}

#[derive(Debug)]
enum PreparedEvalFrame {
    Eval {
        expression: PreparedNodeRef,
        scope: Option<usize>,
    },
    ApplyUnary(UnaryOpKind),
    ApplyBinary(BinOpKind),
    ApplyIf {
        then_expression: PreparedNodeRef,
        else_expression: PreparedNodeRef,
        scope: Option<usize>,
    },
    ApplyBuiltin {
        function: PreparedNodeRef,
        argc: usize,
    },
    ReturnUserFunction {
        scope: usize,
        bindings_start: usize,
    },
    MarkRuntime,
}

struct PreparedExpressionBuilder<'a> {
    ctx: &'a ParamContext,
    external_parameters: &'a HashSet<String>,
    programs: Vec<PreparedProgram>,
    function_programs: HashMap<String, usize>,
    maximum_builtin_args: usize,
    maximum_user_args: usize,
}

impl PreparedExpression {
    pub(crate) fn compile(expr: &Expr, ctx: &ParamContext) -> Result<Self, ExprError> {
        Self::compile_with_external_parameters(expr, ctx, &HashSet::new())
    }

    pub(crate) fn compile_with_external_parameters(
        expr: &Expr,
        ctx: &ParamContext,
        external_parameters: &HashSet<String>,
    ) -> Result<Self, ExprError> {
        let mut builder = PreparedExpressionBuilder {
            ctx,
            external_parameters,
            programs: Vec::new(),
            function_programs: HashMap::new(),
            maximum_builtin_args: 0,
            maximum_user_args: 0,
        };
        let root_program = builder.add_program(expr, Vec::new())?;
        let root = PreparedNodeRef {
            program: root_program,
            node: builder.programs[root_program].root,
        };
        let node_count = builder
            .programs
            .iter()
            .map(|program| program.nodes.len())
            .sum::<usize>();
        // The depth limit is a runtime safety guard, not a useful eager
        // allocation size. Most live expressions never call a user function;
        // size scratch storage from the compiled program and let genuinely
        // deep calls grow these reusable vectors on demand.
        let frame_capacity = node_count.max(1);
        let call_scope_capacity = builder.function_programs.len();
        let binding_capacity = builder.maximum_user_args;
        Ok(Self {
            programs: builder.programs,
            root,
            frames: Vec::with_capacity(frame_capacity),
            values: Vec::with_capacity(node_count.max(1)),
            numeric_args: Vec::with_capacity(builder.maximum_builtin_args),
            call_scopes: Vec::with_capacity(call_scope_capacity),
            arg_bindings: Vec::with_capacity(binding_capacity),
        })
    }

    pub(crate) fn evaluate_with(
        &mut self,
        ctx: &ParamContext,
        resolver: &mut impl FnMut(&str) -> Result<Option<ComplexValue>, ExprError>,
    ) -> Result<ComplexValue, ExprError> {
        self.frames.clear();
        self.values.clear();
        self.numeric_args.clear();
        self.call_scopes.clear();
        self.arg_bindings.clear();
        self.frames.push(PreparedEvalFrame::Eval {
            expression: self.root,
            scope: None,
        });

        while let Some(frame) = self.frames.pop() {
            match frame {
                PreparedEvalFrame::Eval { expression, scope } => {
                    match &self.programs[expression.program].nodes[expression.node] {
                        PreparedNode::Number(value) => self
                            .values
                            .push(EvaluatedValue::literal(ComplexValue::real(*value))),
                        PreparedNode::ComplexNumber(value) => {
                            self.values.push(EvaluatedValue::literal(*value))
                        }
                        PreparedNode::StringLiteral(value) => {
                            return Err(ExprError::InvalidArgument(format!(
                                "string literal \"{value}\" is only valid as a file-backed expression argument"
                            )));
                        }
                        PreparedNode::Param { name, formal_index } => {
                            if let Some(formal_index) = formal_index {
                                let scope_index = scope.ok_or_else(|| {
                                    ExprError::InvalidArgument(format!(
                                        "function argument '{name}' evaluated without a call scope"
                                    ))
                                })?;
                                let call_scope = &self.call_scopes[scope_index];
                                if call_scope.function_program != expression.program
                                    || *formal_index >= call_scope.bindings_len
                                {
                                    return Err(ExprError::InvalidArgument(format!(
                                        "function argument '{name}' has an invalid prepared binding"
                                    )));
                                }
                                let binding =
                                    self.arg_bindings[call_scope.bindings_start + *formal_index];
                                self.frames.push(PreparedEvalFrame::MarkRuntime);
                                self.frames.push(PreparedEvalFrame::Eval {
                                    expression: binding.expression,
                                    scope: binding.caller_scope,
                                });
                            } else {
                                let value = if let Some(value) = resolver(name)? {
                                    value
                                } else {
                                    ctx.get_complex(name).ok_or_else(|| {
                                        ExprError::UndefinedParam(name.to_string())
                                    })?
                                };
                                self.values.push(EvaluatedValue::runtime(value));
                            }
                        }
                        PreparedNode::External(name) => {
                            let value = resolver(name)?
                                .ok_or_else(|| ExprError::UndefinedParam(name.to_string()))?;
                            self.values.push(EvaluatedValue::runtime(value));
                        }
                        PreparedNode::Unary { op, operand } => {
                            self.frames.push(PreparedEvalFrame::ApplyUnary(*op));
                            self.frames.push(PreparedEvalFrame::Eval {
                                expression: PreparedNodeRef {
                                    program: expression.program,
                                    node: *operand,
                                },
                                scope,
                            });
                        }
                        PreparedNode::Binary { op, left, right } => {
                            self.frames.push(PreparedEvalFrame::ApplyBinary(*op));
                            self.frames.push(PreparedEvalFrame::Eval {
                                expression: PreparedNodeRef {
                                    program: expression.program,
                                    node: *right,
                                },
                                scope,
                            });
                            self.frames.push(PreparedEvalFrame::Eval {
                                expression: PreparedNodeRef {
                                    program: expression.program,
                                    node: *left,
                                },
                                scope,
                            });
                        }
                        PreparedNode::Function {
                            name,
                            args,
                            user_program,
                        } => {
                            if let Some(function_program) = user_program {
                                if self.call_scopes.len() >= MAX_EVAL_FUNCTION_CALL_DEPTH {
                                    return Err(ExprError::InvalidArgument(format!(
                                        "function nesting exceeds maximum depth of {} while calling {}",
                                        MAX_EVAL_FUNCTION_CALL_DEPTH, name
                                    )));
                                }
                                let formal_count =
                                    self.programs[*function_program].formal_args.len();
                                if args.len() != formal_count {
                                    return Err(ExprError::WrongArgCount(name.clone()));
                                }
                                let bindings_start = self.arg_bindings.len();
                                self.arg_bindings.extend(args.iter().map(|node| {
                                    PreparedArgBinding {
                                        expression: PreparedNodeRef {
                                            program: expression.program,
                                            node: *node,
                                        },
                                        caller_scope: scope,
                                    }
                                }));
                                let scope_index = self.call_scopes.len();
                                self.call_scopes.push(PreparedCallScope {
                                    function_program: *function_program,
                                    bindings_start,
                                    bindings_len: args.len(),
                                });
                                self.frames.push(PreparedEvalFrame::ReturnUserFunction {
                                    scope: scope_index,
                                    bindings_start,
                                });
                                self.frames.push(PreparedEvalFrame::Eval {
                                    expression: PreparedNodeRef {
                                        program: *function_program,
                                        node: self.programs[*function_program].root,
                                    },
                                    scope: Some(scope_index),
                                });
                            } else if name == "IF" {
                                if args.len() != 3 {
                                    return Err(ExprError::WrongArgCount(name.clone()));
                                }
                                self.frames.push(PreparedEvalFrame::ApplyIf {
                                    then_expression: PreparedNodeRef {
                                        program: expression.program,
                                        node: args[1],
                                    },
                                    else_expression: PreparedNodeRef {
                                        program: expression.program,
                                        node: args[2],
                                    },
                                    scope,
                                });
                                self.frames.push(PreparedEvalFrame::Eval {
                                    expression: PreparedNodeRef {
                                        program: expression.program,
                                        node: args[0],
                                    },
                                    scope,
                                });
                            } else {
                                self.frames.push(PreparedEvalFrame::ApplyBuiltin {
                                    function: expression,
                                    argc: args.len(),
                                });
                                for &node in args.iter().rev() {
                                    self.frames.push(PreparedEvalFrame::Eval {
                                        expression: PreparedNodeRef {
                                            program: expression.program,
                                            node,
                                        },
                                        scope,
                                    });
                                }
                            }
                        }
                    }
                }
                PreparedEvalFrame::ApplyUnary(op) => {
                    let value = pop_value(&mut self.values)?;
                    self.values.push(EvaluatedValue {
                        numeric: apply_unary(op, value.numeric),
                        numval: value.numval && matches!(op, UnaryOpKind::Neg | UnaryOpKind::Pos),
                    });
                }
                PreparedEvalFrame::ApplyBinary(op) => {
                    let right = pop_value(&mut self.values)?;
                    let left = pop_value(&mut self.values)?;
                    let numval = ctx.expression_dialect() == ExpressionDialect::Xyce
                        && left.numval
                        && right.numval
                        && xyce_binary_is_constant_foldable(op);
                    let value =
                        apply_binary(op, left.numeric, right.numeric, ctx.expression_dialect())?;
                    self.values.push(EvaluatedValue {
                        numeric: if numval {
                            normalize_xyce_expression_result(value)
                        } else {
                            value
                        },
                        numval,
                    });
                }
                PreparedEvalFrame::ApplyIf {
                    then_expression,
                    else_expression,
                    scope,
                } => {
                    let condition = pop_value(&mut self.values)?;
                    self.frames.push(PreparedEvalFrame::MarkRuntime);
                    self.frames.push(PreparedEvalFrame::Eval {
                        expression: if complex_truth(condition.numeric) {
                            then_expression
                        } else {
                            else_expression
                        },
                        scope,
                    });
                }
                PreparedEvalFrame::ApplyBuiltin { function, argc } => {
                    let Some(start) = self.values.len().checked_sub(argc) else {
                        return Err(ExprError::InvalidArgument(
                            "expression function argument stack underflow".to_string(),
                        ));
                    };
                    let PreparedNode::Function { name, .. } =
                        &self.programs[function.program].nodes[function.node]
                    else {
                        unreachable!("prepared builtin frame references a non-function node")
                    };
                    let args = &self.values[start..];
                    let numval = ctx.expression_dialect() == ExpressionDialect::Xyce
                        && args.iter().all(|arg| arg.numval)
                        && xyce_numeric_function_is_constant_foldable(name);
                    self.numeric_args.clear();
                    self.numeric_args.extend(args.iter().map(|arg| arg.numeric));
                    let value = if numval {
                        xyce_constant_fold_builtin(name, &self.numeric_args)?
                    } else {
                        eval_builtin_function_values(name, &self.numeric_args, ctx)?
                    };
                    self.values.truncate(start);
                    self.values.push(EvaluatedValue {
                        numeric: if numval {
                            normalize_xyce_expression_result(value)
                        } else {
                            value
                        },
                        numval,
                    });
                }
                PreparedEvalFrame::ReturnUserFunction {
                    scope,
                    bindings_start,
                } => {
                    if self.call_scopes.len() != scope + 1 {
                        return Err(ExprError::InvalidArgument(
                            "prepared expression call-scope stack is inconsistent".to_string(),
                        ));
                    }
                    self.call_scopes.pop();
                    self.arg_bindings.truncate(bindings_start);
                    if let Some(value) = self.values.last_mut() {
                        value.numval = false;
                    }
                }
                PreparedEvalFrame::MarkRuntime => {
                    if let Some(value) = self.values.last_mut() {
                        value.numval = false;
                    }
                }
            }
        }

        if self.values.len() == 1 {
            Ok(self.values.pop().expect("length checked").numeric)
        } else {
            Err(ExprError::InvalidArgument(format!(
                "prepared expression evaluation produced {} values",
                self.values.len()
            )))
        }
    }

    pub(crate) fn visit_runtime_parameters(&self, mut visit: impl FnMut(&str)) {
        for program in &self.programs {
            for node in &program.nodes {
                if let PreparedNode::Param {
                    name,
                    formal_index: None,
                } = node
                {
                    visit(name);
                }
            }
        }
    }
}

impl<'a> PreparedExpressionBuilder<'a> {
    fn add_program(
        &mut self,
        expression: &Expr,
        formal_args: Vec<String>,
    ) -> Result<usize, ExprError> {
        let program = self.programs.len();
        self.programs.push(PreparedProgram {
            nodes: Vec::new(),
            root: 0,
            formal_args,
        });
        let root = self.compile_node(program, expression)?;
        self.programs[program].root = root;
        Ok(program)
    }

    fn ensure_function_program(&mut self, name: &str) -> Result<usize, ExprError> {
        if let Some(&program) = self.function_programs.get(name) {
            return Ok(program);
        }
        let function = self
            .ctx
            .get_function(name)
            .cloned()
            .ok_or_else(|| ExprError::UnknownFunction(name.to_string()))?;
        let program = self.programs.len();
        self.function_programs
            .insert(function.name.clone(), program);
        self.maximum_user_args = self.maximum_user_args.max(function.args.len());
        self.programs.push(PreparedProgram {
            nodes: Vec::new(),
            root: 0,
            formal_args: function.args.clone(),
        });
        let body = parse_expression(&function.body)?;
        let root = self.compile_node(program, &body)?;
        self.programs[program].root = root;
        Ok(program)
    }

    fn compile_node(&mut self, program: usize, expression: &Expr) -> Result<usize, ExprError> {
        let node = match expression {
            Expr::Number(value) => PreparedNode::Number(*value),
            Expr::ComplexNumber(value) => PreparedNode::ComplexNumber(*value),
            Expr::StringLiteral(value) => PreparedNode::StringLiteral(value.clone()),
            Expr::Param(name) if self.external_parameters.contains(name) => {
                PreparedNode::External(name.clone())
            }
            Expr::Param(name) => PreparedNode::Param {
                formal_index: self.programs[program]
                    .formal_args
                    .iter()
                    .position(|formal| formal == name),
                name: name.clone(),
            },
            Expr::UnaryOp { op, operand } => PreparedNode::Unary {
                op: *op,
                operand: self.compile_node(program, operand)?,
            },
            Expr::BinOp { op, left, right } => PreparedNode::Binary {
                op: *op,
                left: self.compile_node(program, left)?,
                right: self.compile_node(program, right)?,
            },
            Expr::FnCall { name, args } => {
                let upper = name.to_ascii_uppercase();
                let user_program = if self.ctx.has_function(&upper) {
                    Some(self.ensure_function_program(&upper)?)
                } else {
                    None
                };
                if user_program.is_none() && upper != "IF" {
                    self.maximum_builtin_args = self.maximum_builtin_args.max(args.len());
                }
                let args = args
                    .iter()
                    .map(|argument| self.compile_node(program, argument))
                    .collect::<Result<Vec<_>, _>>()?;
                PreparedNode::Function {
                    name: upper,
                    args,
                    user_program,
                }
            }
        };
        let index = self.programs[program].nodes.len();
        self.programs[program].nodes.push(node);
        Ok(index)
    }
}

/// Match Xyce's public expression-evaluation boundary, which replaces each
/// non-finite result component with a signed finite sentinel before consumers
/// compare, print, or reuse the value.
pub(crate) fn normalize_xyce_expression_result(value: ComplexValue) -> ComplexValue {
    ComplexValue::new(
        normalize_xyce_expression_component(value.re),
        normalize_xyce_expression_component(value.im),
    )
}

pub(crate) fn normalize_xyce_expression_component(component: Value) -> Value {
    if component.is_finite() {
        component
    } else {
        XYCE_NONFINITE_REPLACEMENT.copysign(component)
    }
}

#[derive(Debug, Clone)]
struct EvalScope {
    function_name: Option<String>,
}

impl EvalScope {
    fn global() -> Self {
        Self {
            function_name: None,
        }
    }

    fn function(function_name: String) -> Self {
        Self {
            function_name: Some(function_name),
        }
    }
}

#[derive(Debug, Clone)]
struct FunctionArgExpr {
    id: u64,
    expr: Expr,
    scope: EvalScope,
}

#[derive(Debug, Clone)]
enum FunctionArgBinding {
    Expr(FunctionArgExpr),
    Unset(u64),
}

#[derive(Debug, Default)]
struct FunctionFrame {
    args: HashMap<String, FunctionArgBinding>,
}

struct ExpressionEvaluator<'a> {
    ctx: &'a ParamContext,
    function_frames: HashMap<String, FunctionFrame>,
    body_cache: HashMap<String, Expr>,
    call_depth: usize,
    next_binding_id: u64,
    numeric_args: Vec<ComplexValue>,
}

#[derive(Debug, Clone, Copy)]
struct EvaluatedValue {
    numeric: ComplexValue,
    /// Xyce parser `numvalType` provenance. Only these literal subtrees are
    /// folded with fixNan/fixInf before their parent expression is evaluated.
    numval: bool,
}

impl EvaluatedValue {
    fn literal(numeric: ComplexValue) -> Self {
        Self {
            numeric,
            numval: true,
        }
    }

    fn runtime(numeric: ComplexValue) -> Self {
        Self {
            numeric,
            numval: false,
        }
    }
}

enum EvalFrame {
    Eval(Expr, EvalScope),
    ApplyUnary(UnaryOpKind),
    ApplyBinary(BinOpKind),
    ApplyIf {
        then_expr: Expr,
        else_expr: Expr,
        scope: EvalScope,
    },
    ApplyBuiltin {
        name: String,
        argc: usize,
    },
    ApplyFunctionArg {
        function_name: String,
        arg_name: String,
        binding_id: u64,
        param_name: String,
        scope: EvalScope,
    },
    UnsetUserFunction {
        function_name: String,
        arg_names: Vec<String>,
    },
    MarkRuntime,
}

impl<'a> ExpressionEvaluator<'a> {
    fn new(ctx: &'a ParamContext) -> Self {
        Self {
            ctx,
            function_frames: HashMap::new(),
            body_cache: HashMap::new(),
            call_depth: 0,
            next_binding_id: 1,
            numeric_args: Vec::new(),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<ComplexValue, ExprError> {
        let mut frames = vec![EvalFrame::Eval(expr.clone(), EvalScope::global())];
        let mut values = Vec::<EvaluatedValue>::new();

        while let Some(frame) = frames.pop() {
            match frame {
                EvalFrame::Eval(expr, scope) => match expr {
                    Expr::Number(value) => {
                        values.push(EvaluatedValue::literal(ComplexValue::real(value)))
                    }
                    Expr::ComplexNumber(value) => values.push(EvaluatedValue::literal(value)),
                    Expr::StringLiteral(value) => {
                        return Err(ExprError::InvalidArgument(format!(
                            "string literal \"{value}\" is only valid as a file-backed expression argument"
                        )));
                    }
                    Expr::Param(name) => {
                        self.push_param_eval(&mut frames, &mut values, name, scope)?
                    }
                    Expr::UnaryOp { op, operand } => {
                        frames.push(EvalFrame::ApplyUnary(op));
                        frames.push(EvalFrame::Eval(*operand, scope));
                    }
                    Expr::BinOp { op, left, right } => {
                        frames.push(EvalFrame::ApplyBinary(op));
                        frames.push(EvalFrame::Eval(*right, scope.clone()));
                        frames.push(EvalFrame::Eval(*left, scope));
                    }
                    Expr::FnCall { name, args } => {
                        self.push_function_eval(&mut frames, name, args, scope)?;
                    }
                },
                EvalFrame::ApplyUnary(op) => {
                    let value = pop_value(&mut values)?;
                    values.push(EvaluatedValue {
                        numeric: apply_unary(op, value.numeric),
                        numval: value.numval && matches!(op, UnaryOpKind::Neg | UnaryOpKind::Pos),
                    });
                }
                EvalFrame::ApplyBinary(op) => {
                    let right = pop_value(&mut values)?;
                    let left = pop_value(&mut values)?;
                    let numval = self.ctx.expression_dialect() == ExpressionDialect::Xyce
                        && left.numval
                        && right.numval
                        && xyce_binary_is_constant_foldable(op);
                    let value = apply_binary(
                        op,
                        left.numeric,
                        right.numeric,
                        self.ctx.expression_dialect(),
                    )?;
                    values.push(EvaluatedValue {
                        numeric: if numval {
                            normalize_xyce_expression_result(value)
                        } else {
                            value
                        },
                        numval,
                    });
                }
                EvalFrame::ApplyIf {
                    then_expr,
                    else_expr,
                    scope,
                } => {
                    let cond = pop_value(&mut values)?;
                    frames.push(EvalFrame::MarkRuntime);
                    frames.push(EvalFrame::Eval(
                        if complex_truth(cond.numeric) {
                            then_expr
                        } else {
                            else_expr
                        },
                        scope,
                    ));
                }
                EvalFrame::ApplyBuiltin { name, argc } => {
                    let Some(start) = values.len().checked_sub(argc) else {
                        return Err(ExprError::InvalidArgument(
                            "expression function argument stack underflow".to_string(),
                        ));
                    };
                    let args = &values[start..];
                    let numval = self.ctx.expression_dialect() == ExpressionDialect::Xyce
                        && args.iter().all(|arg| arg.numval)
                        && xyce_numeric_function_is_constant_foldable(&name);
                    self.numeric_args.clear();
                    self.numeric_args.extend(args.iter().map(|arg| arg.numeric));
                    let value = if numval {
                        xyce_constant_fold_builtin(&name, &self.numeric_args)?
                    } else {
                        eval_builtin_function_values(&name, &self.numeric_args, self.ctx)?
                    };
                    values.truncate(start);
                    values.push(EvaluatedValue {
                        numeric: if numval {
                            normalize_xyce_expression_result(value)
                        } else {
                            value
                        },
                        numval,
                    });
                }
                EvalFrame::ApplyFunctionArg {
                    function_name,
                    arg_name,
                    binding_id,
                    param_name,
                    scope,
                } => {
                    let value = pop_value(&mut values)?;
                    if self
                        .current_function_arg_binding_id(&function_name, &arg_name)
                        .is_some_and(|current_id| current_id == binding_id)
                    {
                        values.push(EvaluatedValue::runtime(value.numeric));
                    } else {
                        frames.push(EvalFrame::Eval(Expr::Param(param_name), scope));
                    }
                }
                EvalFrame::UnsetUserFunction {
                    function_name,
                    arg_names,
                } => {
                    self.call_depth = self.call_depth.saturating_sub(1);
                    self.unset_function_args(&function_name, &arg_names);
                    if let Some(value) = values.last_mut() {
                        value.numval = false;
                    }
                }
                EvalFrame::MarkRuntime => {
                    if let Some(value) = values.last_mut() {
                        value.numval = false;
                    }
                }
            }
        }

        if values.len() == 1 {
            Ok(values.pop().expect("length checked").numeric)
        } else {
            Err(ExprError::InvalidArgument(format!(
                "expression evaluation produced {} values",
                values.len()
            )))
        }
    }

    fn push_param_eval(
        &mut self,
        frames: &mut Vec<EvalFrame>,
        values: &mut Vec<EvaluatedValue>,
        name: String,
        scope: EvalScope,
    ) -> Result<(), ExprError> {
        if let Some((function_name, arg_name, binding)) = self.function_arg_binding(&name, &scope) {
            match binding {
                FunctionArgBinding::Expr(arg) => {
                    frames.push(EvalFrame::ApplyFunctionArg {
                        function_name,
                        arg_name,
                        binding_id: arg.id,
                        param_name: name,
                        scope,
                    });
                    frames.push(EvalFrame::Eval(arg.expr, arg.scope));
                }
                FunctionArgBinding::Unset(_) => {
                    values.push(EvaluatedValue::runtime(ComplexValue::zero()))
                }
            }
            return Ok(());
        }

        values.push(EvaluatedValue::runtime(
            self.ctx
                .get_complex(&name)
                .ok_or_else(|| ExprError::UndefinedParam(name.to_string()))?,
        ));
        Ok(())
    }

    fn push_function_eval(
        &mut self,
        frames: &mut Vec<EvalFrame>,
        name: String,
        args: Vec<Expr>,
        scope: EvalScope,
    ) -> Result<(), ExprError> {
        let upper = name.to_ascii_uppercase();
        if self.ctx.has_function(&upper) {
            self.push_user_function_eval(frames, &upper, &args, &scope)?;
        } else if upper == "IF" {
            if args.len() != 3 {
                return Err(ExprError::WrongArgCount(upper));
            }
            let mut args = args.into_iter();
            let cond = args.next().expect("IF arg count checked");
            let then_expr = args.next().expect("IF arg count checked");
            let else_expr = args.next().expect("IF arg count checked");
            frames.push(EvalFrame::ApplyIf {
                then_expr,
                else_expr,
                scope: scope.clone(),
            });
            frames.push(EvalFrame::Eval(cond, scope));
        } else {
            frames.push(EvalFrame::ApplyBuiltin {
                name: upper,
                argc: args.len(),
            });
            for arg in args.into_iter().rev() {
                frames.push(EvalFrame::Eval(arg, scope.clone()));
            }
        }
        Ok(())
    }

    fn push_user_function_eval(
        &mut self,
        frames: &mut Vec<EvalFrame>,
        name: &str,
        args: &[Expr],
        caller_scope: &EvalScope,
    ) -> Result<(), ExprError> {
        let func = self
            .ctx
            .get_function(name)
            .cloned()
            .ok_or_else(|| ExprError::UnknownFunction(name.to_string()))?;
        if args.len() != func.args.len() {
            return Err(ExprError::WrongArgCount(name.to_string()));
        }
        if self.call_depth >= MAX_EVAL_FUNCTION_CALL_DEPTH {
            return Err(ExprError::InvalidArgument(format!(
                "function nesting exceeds maximum depth of {} while calling {}",
                MAX_EVAL_FUNCTION_CALL_DEPTH, func.name
            )));
        }

        let body = if let Some(body) = self.body_cache.get(&func.name) {
            body.clone()
        } else {
            let body = parse_expression(&func.body)?;
            self.body_cache.insert(func.name.clone(), body.clone());
            body
        };

        let bindings = func
            .args
            .iter()
            .zip(args.iter())
            .map(|(arg_name, arg_expr)| {
                let binding = FunctionArgBinding::Expr(FunctionArgExpr {
                    id: self.allocate_binding_id(),
                    expr: arg_expr.clone(),
                    scope: caller_scope.clone(),
                });
                (arg_name.clone(), binding)
            })
            .collect::<Vec<_>>();

        {
            let frame = self.function_frames.entry(func.name.clone()).or_default();
            for (arg_name, binding) in bindings {
                frame.args.insert(arg_name, binding);
            }
        }

        self.call_depth += 1;
        frames.push(EvalFrame::UnsetUserFunction {
            function_name: func.name.clone(),
            arg_names: func.args.clone(),
        });
        frames.push(EvalFrame::Eval(body, EvalScope::function(func.name)));
        Ok(())
    }

    fn function_arg_binding(
        &self,
        name: &str,
        scope: &EvalScope,
    ) -> Option<(String, String, FunctionArgBinding)> {
        let function_name = scope.function_name.as_ref()?;
        let func = self.ctx.get_function(function_name)?;
        let arg_name = name.to_ascii_uppercase();
        if !func.args.iter().any(|arg| arg == &arg_name) {
            return None;
        }
        let binding = self
            .function_frames
            .get(function_name)
            .and_then(|frame| frame.args.get(&arg_name))
            .cloned()
            .unwrap_or(FunctionArgBinding::Unset(0));
        Some((function_name.clone(), arg_name, binding))
    }

    fn unset_function_args(&mut self, function_name: &str, arg_names: &[String]) {
        for arg_name in arg_names {
            let binding_id = self.allocate_binding_id();
            if let Some(frame) = self.function_frames.get_mut(function_name) {
                frame
                    .args
                    .insert(arg_name.clone(), FunctionArgBinding::Unset(binding_id));
            }
        }
    }

    fn current_function_arg_binding_id(&self, function_name: &str, arg_name: &str) -> Option<u64> {
        match self
            .function_frames
            .get(function_name)?
            .args
            .get(arg_name)?
        {
            FunctionArgBinding::Expr(arg) => Some(arg.id),
            FunctionArgBinding::Unset(id) => Some(*id),
        }
    }

    fn allocate_binding_id(&mut self) -> u64 {
        let id = self.next_binding_id;
        self.next_binding_id = self.next_binding_id.saturating_add(1);
        id
    }
}

fn pop_value(values: &mut Vec<EvaluatedValue>) -> Result<EvaluatedValue, ExprError> {
    values.pop().ok_or_else(|| {
        ExprError::InvalidArgument("expression evaluation stack underflow".to_string())
    })
}

fn apply_unary(op: UnaryOpKind, value: ComplexValue) -> ComplexValue {
    match op {
        UnaryOpKind::Neg => ComplexValue::new(-value.re, -value.im),
        UnaryOpKind::Pos => value,
        UnaryOpKind::Not => bool_value(!complex_truth(value)),
    }
}

fn apply_binary(
    op: BinOpKind,
    left: ComplexValue,
    right: ComplexValue,
    dialect: ExpressionDialect,
) -> Result<ComplexValue, ExprError> {
    Ok(match op {
        BinOpKind::Add => complex_add(left, right),
        BinOpKind::Sub => complex_sub(left, right),
        BinOpKind::Mul => complex_mul(left, right),
        BinOpKind::Div if dialect == ExpressionDialect::Xyce => complex_div_xyce(left, right),
        BinOpKind::Div => complex_div(left, right)?,
        BinOpKind::Mod if dialect == ExpressionDialect::Xyce => complex_mod_xyce(left, right),
        BinOpKind::Mod => complex_mod(left, right)?,
        BinOpKind::Pow => complex_pow(left, right),
        BinOpKind::Gt => bool_value(left.re > right.re),
        BinOpKind::Lt => bool_value(left.re < right.re),
        BinOpKind::Ge => bool_value(left.re >= right.re),
        BinOpKind::Le => bool_value(left.re <= right.re),
        BinOpKind::Eq => {
            bool_value((left.re - right.re).abs() < 1e-12 && (left.im - right.im).abs() < 1e-12)
        }
        BinOpKind::Ne => {
            bool_value((left.re - right.re).abs() >= 1e-12 || (left.im - right.im).abs() >= 1e-12)
        }
        BinOpKind::And => bool_value(complex_truth(left) && complex_truth(right)),
        BinOpKind::Or => bool_value(complex_truth(left) || complex_truth(right)),
    })
}

#[inline]
fn bool_value(value: bool) -> ComplexValue {
    ComplexValue::real(if value { 1.0 } else { 0.0 })
}

#[inline]
fn complex_truth(value: ComplexValue) -> bool {
    value.re != 0.0 || value.im != 0.0
}

#[inline]
fn complex_add(left: ComplexValue, right: ComplexValue) -> ComplexValue {
    ComplexValue::new(left.re + right.re, left.im + right.im)
}

#[inline]
fn complex_sub(left: ComplexValue, right: ComplexValue) -> ComplexValue {
    ComplexValue::new(left.re - right.re, left.im - right.im)
}

#[inline]
fn complex_mul(left: ComplexValue, right: ComplexValue) -> ComplexValue {
    ComplexValue::new(
        left.re.mul_add(right.re, -(left.im * right.im)),
        left.re.mul_add(right.im, left.im * right.re),
    )
}

#[inline]
fn complex_div(left: ComplexValue, right: ComplexValue) -> Result<ComplexValue, ExprError> {
    let denom = right.re.mul_add(right.re, right.im * right.im);
    if denom < 1e-300 {
        return Err(ExprError::DivisionByZero);
    }
    Ok(ComplexValue::new(
        (left.re * right.re + left.im * right.im) / denom,
        (left.im * right.re - left.re * right.im) / denom,
    ))
}

fn xyce_numeric_function_is_constant_foldable(name: &str) -> bool {
    matches!(
        name,
        "SQRT"
            | "SIN"
            | "COS"
            | "TAN"
            | "EXP"
            | "LOG"
            | "LN"
            | "LOG10"
            | "ABS"
            | "M"
            | "ASIN"
            | "ACOS"
            | "ATAN"
            | "ARCTAN"
            | "POW"
            | "PWR"
            | "PWRS"
            | "SGN"
            | "SIGN"
            | "SINH"
            | "COSH"
            | "TANH"
            | "ASINH"
            | "ACOSH"
            | "ATANH"
    )
}

fn xyce_binary_is_constant_foldable(op: BinOpKind) -> bool {
    matches!(
        op,
        BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div | BinOpKind::Pow
    )
}

fn xyce_constant_fold_builtin(
    name: &str,
    args: &[ComplexValue],
) -> Result<ComplexValue, ExprError> {
    let unary = |operation: fn(num_complex::Complex64) -> num_complex::Complex64| {
        require_arg_count(name, args, 1)?;
        Ok(from_num_complex(operation(to_num_complex(args[0]))))
    };
    match name {
        "POW" | "PWR" => {
            require_arg_count(name, args, 2)?;
            Ok(from_num_complex(
                to_num_complex(args[0]).powc(to_num_complex(args[1])),
            ))
        }
        "PWRS" => {
            require_arg_count(name, args, 2)?;
            Ok(from_num_complex(xyce_complex_pwrs(
                to_num_complex(args[0]),
                to_num_complex(args[1]),
            )))
        }
        "SGN" => {
            require_arg_count(name, args, 1)?;
            Ok(ComplexValue::real(if args[0].re > 0.0 {
                1.0
            } else if args[0].re < 0.0 {
                -1.0
            } else {
                0.0
            }))
        }
        "SIGN" => {
            require_arg_count(name, args, 2)?;
            let sign = if args[1].re > 0.0 {
                1.0
            } else if args[1].re < 0.0 {
                -1.0
            } else {
                0.0
            };
            Ok(ComplexValue::real(args[0].magnitude() * sign))
        }
        "SQRT" => unary(num_complex::Complex64::sqrt),
        "EXP" => unary(num_complex::Complex64::exp),
        "SIN" => unary(num_complex::Complex64::sin),
        "COS" => unary(num_complex::Complex64::cos),
        "TAN" => unary(num_complex::Complex64::tan),
        "M" | "ABS" => {
            require_arg_count(name, args, 1)?;
            Ok(ComplexValue::real(args[0].magnitude()))
        }
        "ACOS" => unary(xyce_complex_acos),
        "ACOSH" => unary(xyce_complex_acosh),
        "ASIN" => unary(xyce_complex_asin),
        "ASINH" => unary(xyce_complex_asinh),
        "ATAN" | "ARCTAN" => unary(num_complex::Complex64::atan),
        "ATANH" => unary(xyce_complex_atanh),
        "COSH" => unary(num_complex::Complex64::cosh),
        "LN" => unary(num_complex::Complex64::ln),
        "LOG" | "LOG10" => unary(num_complex::Complex64::log10),
        "SINH" => unary(num_complex::Complex64::sinh),
        "TANH" => unary(num_complex::Complex64::tanh),
        _ => Err(ExprError::UnknownFunction(name.to_string())),
    }
}

fn to_num_complex(value: ComplexValue) -> num_complex::Complex64 {
    num_complex::Complex64::new(value.re, value.im)
}

fn from_num_complex(value: num_complex::Complex64) -> ComplexValue {
    ComplexValue::new(value.re, value.im)
}

fn xyce_complex_pwrs(
    base: num_complex::Complex64,
    exponent: num_complex::Complex64,
) -> num_complex::Complex64 {
    if base.re < 0.0 {
        -(-base).powc(exponent)
    } else {
        base.powc(exponent)
    }
}

fn xyce_complex_atanh(value: num_complex::Complex64) -> num_complex::Complex64 {
    if value.im == 0.0 && value.re.abs() > 1.0 {
        // libstdc++'s std::atanh follows the signed-zero side of the real
        // branch cut. num_complex selects the opposite side for +0, so match
        // the C++/Xyce convention explicitly.
        let real = 0.5 * ((1.0 + value.re).abs() / (1.0 - value.re).abs()).ln();
        return num_complex::Complex64::new(real, std::f64::consts::FRAC_PI_2.copysign(value.im));
    }
    value.atanh()
}

fn xyce_complex_asin(value: num_complex::Complex64) -> num_complex::Complex64 {
    if value.im == 0.0 && value.re.abs() > 1.0 {
        return num_complex::Complex64::new(
            std::f64::consts::FRAC_PI_2.copysign(value.re),
            value.re.abs().acosh().copysign(value.im),
        );
    }
    value.asin()
}

fn xyce_complex_acos(value: num_complex::Complex64) -> num_complex::Complex64 {
    if value.im == 0.0 && value.re > 1.0 {
        return num_complex::Complex64::new(0.0, -value.re.acosh().copysign(value.im));
    }
    if value.im == 0.0 && value.re < -1.0 {
        return num_complex::Complex64::new(
            std::f64::consts::PI,
            -(-value.re).acosh().copysign(value.im),
        );
    }
    value.acos()
}

fn xyce_complex_acosh(value: num_complex::Complex64) -> num_complex::Complex64 {
    if value.im == 0.0 && value.re < 1.0 {
        let real = if value.re < -1.0 {
            (-value.re).acosh()
        } else {
            0.0
        };
        let angle = if value.re <= -1.0 {
            std::f64::consts::PI
        } else {
            value.re.acos()
        };
        return num_complex::Complex64::new(real, angle.copysign(value.im));
    }
    value.acosh()
}

fn xyce_complex_asinh(value: num_complex::Complex64) -> num_complex::Complex64 {
    let mut result = value.asinh();
    if value.im == 0.0 && result.im == 0.0 {
        result.im = value.im;
    }
    result
}

#[inline]
fn complex_div_xyce(left: ComplexValue, right: ComplexValue) -> ComplexValue {
    // Xyce's expression scalar is std::complex<double>.  libstdc++ lowers its
    // quotient to GCC's scaled complex-division runtime, including recovery
    // for zero and non-finite operands.  Reproduce that behavior explicitly:
    // num_complex uses the unscaled textbook formula and neither recovers the
    // real infinity in (1+0i)/(0+0i) nor preserves the NaN signs consumed by
    // Xyce's per-component fixNan.
    let (mut a, mut b) = (left.re, left.im);
    let (mut c, mut d) = (right.re, right.im);
    let denominator_scale = c.abs().max(d.abs());
    let mut denominator_exponent = 0;
    if denominator_scale.is_finite() && denominator_scale != 0.0 {
        denominator_exponent = libm::ilogb(denominator_scale);
        c = libm::scalbn(c, -denominator_exponent);
        d = libm::scalbn(d, -denominator_exponent);
    }

    let denominator = c * c + d * d;
    let mut real = libm::scalbn((a * c + b * d) / denominator, -denominator_exponent);
    let mut imaginary = libm::scalbn((b * c - a * d) / denominator, -denominator_exponent);

    if real.is_nan() && imaginary.is_nan() {
        if denominator == 0.0 && (!a.is_nan() || !b.is_nan()) {
            let infinity = Value::INFINITY.copysign(c);
            real = gcc_infinity_product(infinity, a);
            imaginary = gcc_infinity_product(infinity, b);
        } else if (a.is_infinite() || b.is_infinite()) && c.is_finite() && d.is_finite() {
            a = if a.is_infinite() {
                1.0_f64.copysign(a)
            } else {
                0.0_f64.copysign(a)
            };
            b = if b.is_infinite() {
                1.0_f64.copysign(b)
            } else {
                0.0_f64.copysign(b)
            };
            real = gcc_infinity_product(Value::INFINITY, a * c + b * d);
            imaginary = gcc_infinity_product(Value::INFINITY, b * c - a * d);
        } else if denominator_scale.is_infinite() && a.is_finite() && b.is_finite() {
            c = if c.is_infinite() {
                1.0_f64.copysign(c)
            } else {
                0.0_f64.copysign(c)
            };
            d = if d.is_infinite() {
                1.0_f64.copysign(d)
            } else {
                0.0_f64.copysign(d)
            };
            real = gcc_zero_product(a * c + b * d);
            imaginary = gcc_zero_product(b * c - a * d);
        }
    }

    ComplexValue::new(real, imaginary)
}

#[inline]
fn gcc_negative_nan() -> Value {
    Value::NAN.copysign(-1.0)
}

#[inline]
fn gcc_infinity_product(infinity: Value, factor: Value) -> Value {
    if factor == 0.0 {
        // GCC's complex runtime produces a negative quiet NaN for Inf * +/-0
        // on the libstdc++ runtime used by the Xyce oracle.
        gcc_negative_nan()
    } else {
        infinity * factor
    }
}

#[inline]
fn gcc_zero_product(factor: Value) -> Value {
    if factor.is_infinite() {
        gcc_negative_nan()
    } else {
        0.0 * factor
    }
}

#[inline]
fn complex_mod(left: ComplexValue, right: ComplexValue) -> Result<ComplexValue, ExprError> {
    if !left.is_real() || !right.is_real() {
        return Err(ExprError::InvalidArgument(
            "modulo requires real-valued operands".to_string(),
        ));
    }
    if right.re == 0.0 {
        return Err(ExprError::DivisionByZero);
    }
    Ok(ComplexValue::real(left.re % right.re))
}

#[inline]
fn complex_mod_xyce(left: ComplexValue, right: ComplexValue) -> ComplexValue {
    // Xyce's fmodOp explicitly projects std::real from both operands and lets
    // std::fmod produce IEEE NaN for invalid or zero-divisor cases.
    ComplexValue::real(left.re % right.re)
}

#[inline]
fn complex_arg(value: ComplexValue) -> Value {
    if value.im == 0.0 && value.re < 0.0 {
        -std::f64::consts::PI
    } else {
        value.im.atan2(value.re)
    }
}

#[inline]
fn complex_ln(value: ComplexValue) -> ComplexValue {
    ComplexValue::new(value.magnitude().ln(), complex_arg(value))
}

#[inline]
fn complex_exp(value: ComplexValue) -> ComplexValue {
    let scale = value.re.exp();
    ComplexValue::new(scale * value.im.cos(), scale * value.im.sin())
}

#[inline]
fn complex_pow(base: ComplexValue, exponent: ComplexValue) -> ComplexValue {
    if base.is_real() && exponent.is_real() {
        let real = base.re.powf(exponent.re);
        if real.is_finite() || base.re >= 0.0 {
            return ComplexValue::real(real);
        }
    }
    complex_exp(complex_mul(exponent, complex_ln(base)))
}

#[inline]
pub(super) fn complex_sqrt(value: ComplexValue) -> ComplexValue {
    if value.im == 0.0 {
        return if value.re >= 0.0 {
            ComplexValue::new(value.re.sqrt(), value.im)
        } else {
            ComplexValue::new(0.0, (-value.re).sqrt().copysign(value.im))
        };
    }
    complex_pow(value, ComplexValue::real(0.5))
}

#[inline]
fn complex_sin(value: ComplexValue) -> ComplexValue {
    ComplexValue::new(
        value.re.sin() * value.im.cosh(),
        value.re.cos() * value.im.sinh(),
    )
}

#[inline]
fn complex_cos(value: ComplexValue) -> ComplexValue {
    ComplexValue::new(
        value.re.cos() * value.im.cosh(),
        -(value.re.sin() * value.im.sinh()),
    )
}

#[inline]
fn complex_tan(value: ComplexValue) -> Result<ComplexValue, ExprError> {
    complex_div(complex_sin(value), complex_cos(value))
}

fn eval_builtin_function_values(
    name: &str,
    args: &[ComplexValue],
    ctx: &ParamContext,
) -> Result<ComplexValue, ExprError> {
    eval_complex_builtin_function(name, args, ctx)
}

fn eval_complex_builtin_function(
    name: &str,
    args: &[ComplexValue],
    ctx: &ParamContext,
) -> Result<ComplexValue, ExprError> {
    match name {
        "SQRT" => {
            require_arg_count(name, args, 1)?;
            Ok(complex_sqrt(args[0]))
        }
        "SIN" => {
            require_arg_count(name, args, 1)?;
            Ok(complex_sin(args[0]))
        }
        "COS" => {
            require_arg_count(name, args, 1)?;
            Ok(complex_cos(args[0]))
        }
        "TAN" => {
            require_arg_count(name, args, 1)?;
            complex_tan(args[0])
        }
        "EXP" => {
            require_arg_count(name, args, 1)?;
            Ok(complex_exp(args[0]))
        }
        "LOG" => {
            require_arg_count(name, args, 1)?;
            let value = complex_ln(args[0]);
            if ctx.expression_dialect() == ExpressionDialect::Xyce {
                Ok(ComplexValue::new(
                    value.re / std::f64::consts::LN_10,
                    value.im / std::f64::consts::LN_10,
                ))
            } else {
                Ok(value)
            }
        }
        "LN" => {
            require_arg_count(name, args, 1)?;
            Ok(complex_ln(args[0]))
        }
        "LOG10" => {
            require_arg_count(name, args, 1)?;
            let value = complex_ln(args[0]);
            Ok(ComplexValue::new(
                value.re / std::f64::consts::LN_10,
                value.im / std::f64::consts::LN_10,
            ))
        }
        "ABS" | "M" | "MAG" => {
            require_arg_count(name, args, 1)?;
            Ok(ComplexValue::real(args[0].magnitude()))
        }
        "R" | "RE" | "REAL" => {
            require_arg_count(name, args, 1)?;
            Ok(ComplexValue::real(args[0].re))
        }
        "IMG" | "IMAG" => {
            require_arg_count(name, args, 1)?;
            Ok(ComplexValue::real(args[0].im))
        }
        "PH" | "PHASE" => {
            require_arg_count(name, args, 1)?;
            Ok(ComplexValue::real(complex_arg(args[0]).to_degrees()))
        }
        "DB" => {
            require_arg_count(name, args, 1)?;
            Ok(ComplexValue::real(20.0 * args[0].magnitude().log10()))
        }
        "ASIN" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 1)?;
            Ok(from_num_complex(to_num_complex(args[0]).asin()))
        }
        "ASIN" => real_unary(name, args, |x| x.asin()),
        "ACOS" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 1)?;
            Ok(from_num_complex(to_num_complex(args[0]).acos()))
        }
        "ACOS" => real_unary(name, args, |x| x.acos()),
        "ATAN" | "ARCTAN" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 1)?;
            Ok(from_num_complex(to_num_complex(args[0]).atan()))
        }
        "ATAN" | "ARCTAN" => real_unary(name, args, |x| x.atan()),
        "ATAN2" => real_binary(name, args, |left, right| left.atan2(right)),
        "FMOD" | "MOD" => eval_real_fmod_function(name, args, ctx.expression_dialect()),
        "FLOOR" => real_unary(name, args, |x| x.floor()),
        "CEIL" => real_unary(name, args, |x| x.ceil()),
        "ROUND" => real_unary(name, args, |x| x.round()),
        "MIN" => real_binary(name, args, |left, right| left.min(right)),
        "MAX" => real_binary(name, args, |left, right| left.max(right)),
        "POW" | "PWR" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 2)?;
            Ok(from_num_complex(
                to_num_complex(args[0]).powc(to_num_complex(args[1])),
            ))
        }
        "POW" => {
            require_arg_count(name, args, 2)?;
            Ok(complex_pow(args[0], args[1]))
        }
        "PWR" => Ok(ComplexValue::real({
            require_arg_count(name, args, 2)?;
            checked_real_arg(name, args, 0)?
                .abs()
                .powf(checked_real_arg(name, args, 1)?)
        })),
        "PWRS" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 2)?;
            Ok(from_num_complex(xyce_complex_pwrs(
                to_num_complex(args[0]),
                to_num_complex(args[1]),
            )))
        }
        "PWRS" => {
            require_arg_count(name, args, 2)?;
            let base = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(
                crate::expr::ordered_sign(base) * base.abs().powf(checked_real_arg(name, args, 1)?),
            ))
        }
        "LIMIT" => match args.len() {
            2 => {
                let nom = checked_real_arg(name, args, 0)?;
                let avar = checked_real_arg(name, args, 1)?;
                if ctx.statistical_mode() == StatisticalParamMode::Nominal {
                    return Ok(ComplexValue::real(nom));
                }
                let sign = if ctx.random().next_symmetric() > 0.0 {
                    1.0
                } else {
                    -1.0
                };
                Ok(ComplexValue::real(nom + avar * sign))
            }
            3 => {
                let x = checked_real_arg(name, args, 0)?;
                let min = checked_real_arg(name, args, 1)?;
                let max = checked_real_arg(name, args, 2)?;
                Ok(ComplexValue::real(
                    crate::expr::ordered_limit(x, min, max, ctx.expression_dialect()).0,
                ))
            }
            _ => Err(ExprError::WrongArgCount("LIMIT".to_string())),
        },
        "GAUSS" | "AGAUSS" => {
            if !(2..=3).contains(&args.len()) {
                return Err(ExprError::WrongArgCount(name.to_string()));
            }
            let nom = checked_real_arg(name, args, 0)?;
            let var = checked_real_arg(name, args, 1)?;
            let sigma = if args.len() == 3 {
                checked_real_arg(name, args, 2)?
            } else {
                1.0
            };
            if sigma == 0.0 {
                return Err(ExprError::InvalidArgument(format!(
                    "{name}: sigma must be non-zero"
                )));
            }
            let deviation = if name == "GAUSS" { nom * var } else { var };
            if ctx.statistical_mode() == StatisticalParamMode::Nominal {
                return Ok(ComplexValue::real(nom));
            }
            Ok(ComplexValue::real(
                nom + deviation / sigma * ctx.random().next_standard_normal(),
            ))
        }
        "UNIF" | "AUNIF" => {
            require_arg_count(name, args, 2)?;
            let nom = checked_real_arg(name, args, 0)?;
            let var = checked_real_arg(name, args, 1)?;
            let deviation = if name == "UNIF" { nom * var } else { var };
            if ctx.statistical_mode() == StatisticalParamMode::Nominal {
                return Ok(ComplexValue::real(nom));
            }
            Ok(ComplexValue::real(
                nom + deviation * ctx.random().next_symmetric(),
            ))
        }
        "IF" => {
            require_arg_count(name, args, 3)?;
            if complex_truth(args[0]) {
                Ok(args[1])
            } else {
                Ok(args[2])
            }
        }
        "URAMP" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(if x > 0.0 { x } else { 0.0 }))
        }
        "SGN" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 1)?;
            let x = args[0].re;
            Ok(ComplexValue::real(if x > 0.0 {
                1.0
            } else if x < 0.0 {
                -1.0
            } else {
                0.0
            }))
        }
        "SGN" => {
            require_arg_count(name, args, 1)?;
            let x = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(crate::expr::ordered_sign(x)))
        }
        "SIGN" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 2)?;
            let sign = if args[1].re > 0.0 {
                1.0
            } else if args[1].re < 0.0 {
                -1.0
            } else {
                0.0
            };
            Ok(ComplexValue::real(args[0].magnitude() * sign))
        }
        "SIGN" => {
            require_arg_count(name, args, 1)?;
            let x = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(crate::expr::ordered_sign(x)))
        }
        "TABLE" | "PWL" => eval_real_table_function(name, args),
        "SINH" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 1)?;
            Ok(from_num_complex(to_num_complex(args[0]).sinh()))
        }
        "SINH" => real_unary(name, args, |x| x.sinh()),
        "COSH" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 1)?;
            Ok(from_num_complex(to_num_complex(args[0]).cosh()))
        }
        "COSH" => real_unary(name, args, |x| x.cosh()),
        "TANH" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 1)?;
            Ok(from_num_complex(xyce_runtime_tanh(to_num_complex(args[0]))))
        }
        "TANH" => real_unary(name, args, |x| x.tanh()),
        "ASINH" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 1)?;
            Ok(from_num_complex(to_num_complex(args[0]).asinh()))
        }
        "ASINH" => real_unary(name, args, |x| x.asinh()),
        "ACOSH" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 1)?;
            Ok(from_num_complex(to_num_complex(args[0]).acosh()))
        }
        "ACOSH" => real_unary(name, args, |x| x.acosh()),
        "ATANH" if ctx.expression_dialect() == ExpressionDialect::Xyce => {
            require_arg_count(name, args, 1)?;
            Ok(from_num_complex(xyce_runtime_atanh(to_num_complex(
                args[0],
            ))))
        }
        "ATANH" => real_unary(name, args, |x| x.atanh()),
        "INT" | "TRUNC" => real_unary(name, args, |x| x.trunc()),
        "NINT" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(x.round_ties_even()))
        }
        "SQR" => {
            require_arg_count(name, args, 1)?;
            Ok(complex_mul(args[0], args[0]))
        }
        "U" | "USTEP" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(if x > 0.0 {
                1.0
            } else if x == 0.0 {
                0.5
            } else {
                0.0
            }))
        }
        "U2" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(ComplexValue::real(x.clamp(0.0, 1.0)))
        }
        "EQ0" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(bool_value(x.abs() < 1e-12))
        }
        "NE0" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(bool_value(x.abs() >= 1e-12))
        }
        "GT0" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(bool_value(x > 0.0))
        }
        "LT0" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(bool_value(x < 0.0))
        }
        "GE0" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(bool_value(x >= 0.0))
        }
        "LE0" => {
            let x = checked_real_arg(name, args, 0)?;
            Ok(bool_value(x <= 0.0))
        }
        "RAND" | "RANDOM" => Ok(ComplexValue::real(0.5)),
        _ => Err(ExprError::UnknownFunction(name.to_string())),
    }
}

fn require_arg_count(name: &str, args: &[ComplexValue], expected: usize) -> Result<(), ExprError> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(ExprError::WrongArgCount(name.to_string()))
    }
}

fn checked_arg(name: &str, args: &[ComplexValue], index: usize) -> Result<ComplexValue, ExprError> {
    args.get(index)
        .copied()
        .ok_or_else(|| ExprError::WrongArgCount(name.to_string()))
}

fn checked_real_arg(name: &str, args: &[ComplexValue], index: usize) -> Result<Value, ExprError> {
    let value = checked_arg(name, args, index)?;
    if value.is_real() {
        Ok(value.re)
    } else {
        Err(ExprError::InvalidArgument(format!(
            "{name}: complex argument {} is not valid for a real-valued function",
            index + 1
        )))
    }
}

fn real_unary(
    name: &str,
    args: &[ComplexValue],
    op: impl FnOnce(Value) -> Value,
) -> Result<ComplexValue, ExprError> {
    require_arg_count(name, args, 1)?;
    Ok(ComplexValue::real(op(checked_real_arg(name, args, 0)?)))
}

fn real_binary(
    name: &str,
    args: &[ComplexValue],
    op: impl FnOnce(Value, Value) -> Value,
) -> Result<ComplexValue, ExprError> {
    require_arg_count(name, args, 2)?;
    Ok(ComplexValue::real(op(
        checked_real_arg(name, args, 0)?,
        checked_real_arg(name, args, 1)?,
    )))
}

/// Evaluate the real-valued C/POSIX `fmod` operation.
///
/// Rust's floating-point remainder operator has the same sign convention as
/// `fmod`: the quotient is truncated toward zero, so a non-zero remainder has
/// the sign of the dividend.
fn eval_real_fmod_function(
    name: &str,
    args: &[ComplexValue],
    dialect: ExpressionDialect,
) -> Result<ComplexValue, ExprError> {
    require_arg_count(name, args, 2)?;
    if dialect == ExpressionDialect::Xyce {
        // Xyce's fmodOp projects std::real from both operands and invokes
        // std::fmod directly. Invalid/zero-divisor inputs yield IEEE NaN for
        // the expression-root normalization boundary to handle.
        return Ok(ComplexValue::real(args[0].re % args[1].re));
    }
    let dividend = checked_real_arg(name, args, 0)?;
    let divisor = checked_real_arg(name, args, 1)?;

    if !dividend.is_finite() || !divisor.is_finite() {
        return Err(ExprError::InvalidArgument(format!(
            "{name}: operands must be finite"
        )));
    }
    if divisor == 0.0 {
        return Err(ExprError::DivisionByZero);
    }

    let remainder = dividend % divisor;
    if !remainder.is_finite() {
        return Err(ExprError::InvalidArgument(format!(
            "{name}: remainder is not finite"
        )));
    }
    Ok(ComplexValue::real(remainder))
}

fn xyce_runtime_tanh(value: num_complex::Complex64) -> num_complex::Complex64 {
    if value.re > XYCE_TANH_SATURATION_THRESHOLD {
        num_complex::Complex64::new(1.0, 0.0)
    } else if value.re < -XYCE_TANH_SATURATION_THRESHOLD {
        num_complex::Complex64::new(-1.0, 0.0)
    } else {
        value.tanh()
    }
}

fn xyce_runtime_atanh(mut value: num_complex::Complex64) -> num_complex::Complex64 {
    let lower = XYCE_ATANH_EPSILON - 1.0;
    let upper = 1.0 - XYCE_ATANH_EPSILON;
    if value.re < lower {
        value = num_complex::Complex64::new(lower, 0.0);
    } else if value.re > upper {
        value = num_complex::Complex64::new(upper, 0.0);
    }
    if value.im == 0.0 && value.re.abs() <= 1.0 {
        // libstdc++ evaluates std::atanh(complex<double>) on the real interval
        // with the same stable real branch used by std::atanh(double).  The
        // logarithmic identity in num_complex loses a few ulps near Xyce's
        // +/- (1 - epsilon) clamps.
        num_complex::Complex64::new(value.re.atanh(), value.im)
    } else {
        value.atanh()
    }
}

fn eval_real_table_function(name: &str, args: &[ComplexValue]) -> Result<ComplexValue, ExprError> {
    if args.len() < 3 {
        return Err(ExprError::WrongArgCount(name.to_string()));
    }
    let x = checked_real_arg(name, args, 0)?;
    let mut points = Vec::<(Value, Value)>::new();
    let mut i = 1;
    while i + 1 < args.len() {
        points.push((
            checked_real_arg(name, args, i)?,
            checked_real_arg(name, args, i + 1)?,
        ));
        i += 2;
    }
    Ok(ComplexValue::real(table_interpolate(x, &points)))
}

/// Piecewise linear interpolation for TABLE function
///
/// Linearly interpolates between defined points.
/// For x outside the defined range, clamps to the nearest endpoint.
#[inline]
fn table_interpolate(x: Value, points: &[(Value, Value)]) -> Value {
    if points.is_empty() {
        return 0.0;
    }

    if points.len() == 1 {
        return points[0].1;
    }

    // Sort points by x (should already be sorted in valid input)
    // For performance, we assume input is sorted

    // Handle x below first point - clamp
    if x <= points[0].0 {
        return points[0].1;
    }

    // Handle x above last point - clamp
    let last = points.len() - 1;
    if x >= points[last].0 {
        return points[last].1;
    }

    // Find bracketing points and interpolate
    for i in 0..points.len() - 1 {
        let (x0, y0) = points[i];
        let (x1, y1) = points[i + 1];

        if x >= x0 && x <= x1 {
            if (x1 - x0).abs() < 1e-18 {
                return y0;
            }
            let t = (x - x0) / (x1 - x0);
            return y0 + t * (y1 - y0);
        }
    }

    // Fallback (shouldn't reach here)
    points[last].1
}
