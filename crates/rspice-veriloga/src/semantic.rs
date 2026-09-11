//! Semantic Analyzer for Verilog-A/AMS
//!
//! Provides comprehensive semantic analysis including:
//! - Hierarchical symbol table with nested scopes
//! - Type inference and checking
//! - Discipline validation
//! - Expression validation
//! - Parameter range checking

use crate::ast::*;
use crate::disciplines::{Discipline, DisciplineDb, Domain, Nature};
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::integer_runtime::{IntegerBinaryOperation, integer_binary, real_to_integer};
use crate::numeric_literal::parse_integer_literal;
use crate::source::Span;
use crate::types::{FunctionRegistry, ParameterRange as TypedParameterRange, ValueType};
use smol_str::SmolStr;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

const RSPICE_LIMITED_EXP_INTRINSIC: &str = "__rspice_limited_exp";
pub(crate) const MAX_PARAMETER_ARRAY_RANK: usize = 16;
pub(crate) const MAX_PARAMETER_ARRAY_ELEMENTS: u64 = 1_048_576;
const MAX_REPLICATION_NESTING: usize = 128;
const MAX_REPLICATION_MATERIALIZATION_WORK: usize = 4_194_304;
const MAX_ANALOG_FILTER_VECTOR_ELEMENTS: usize =
    crate::zfilter::MAX_ZI_RUNTIME_OPERANDS - crate::zfilter::ZI_FIXED_RUNTIME_OPERANDS;

/// The leaf is still authored syntax; operator operands have already been
/// rewritten. Keeping these cases distinct prevents a second operand walk.
enum OperatorRewrite<'a> {
    Leaf(&'a Expression),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
}

/// Share the postorder traversal between function materialization and ordinary
/// lowering. Both preserve association and visit the left operand first.
fn rewrite_operator_tree(
    expr: &Expression,
    mut rewrite: impl FnMut(OperatorRewrite<'_>) -> CompileResult<Expression>,
) -> CompileResult<Expression> {
    let mut pending = vec![(expr, false)];
    let mut values = Vec::new();
    while let Some((expression, children_rewritten)) = pending.pop() {
        let node = match expression {
            Expression::Binary(binary) if !children_rewritten => {
                pending.push((expression, true));
                pending.push((&binary.right, false));
                pending.push((&binary.left, false));
                continue;
            }
            Expression::Unary(unary) if !children_rewritten => {
                pending.push((expression, true));
                pending.push((&unary.operand, false));
                continue;
            }
            Expression::Binary(binary) => {
                let right = values.pop().expect("right operand was rewritten");
                let left = values.pop().expect("left operand was rewritten");
                OperatorRewrite::Binary(BinaryExpr {
                    op: binary.op,
                    left: Box::new(left),
                    right: Box::new(right),
                    span: binary.span,
                })
            }
            Expression::Unary(unary) => OperatorRewrite::Unary(UnaryExpr {
                op: unary.op,
                operand: Box::new(values.pop().expect("unary operand was rewritten")),
                span: unary.span,
            }),
            expression => OperatorRewrite::Leaf(expression),
        };
        values.push(rewrite(node)?);
    }
    debug_assert_eq!(values.len(), 1);
    Ok(values.pop().expect("root expression was rewritten"))
}

/// Resolve each operator while its operand types are still available. The
/// postorder type stack makes long arithmetic chains linear to lower.
fn resolve_integer_operator_tree(
    expr: &Expression,
    mut leaf: impl FnMut(&Expression) -> CompileResult<(Expression, ValueType)>,
) -> CompileResult<Expression> {
    let mut types: Vec<(ValueType, bool)> = Vec::new();
    rewrite_operator_tree(expr, |node| {
        let (expression, value_type, wide) = match node {
            OperatorRewrite::Leaf(expression) => {
                let (expression, value_type) = leaf(expression)?;
                let wide = wide_integer_value(&expression);
                (expression, value_type, wide)
            }
            OperatorRewrite::Binary(mut binary) => {
                let (right, right_wide) = types.pop().expect("right operand typed");
                let (left, left_wide) = types.pop().expect("left operand typed");
                let common = left.common_type(right);
                let wide = left_wide || right_wide;
                let arithmetic = match binary.op {
                    BinaryOp::Add => Some(BinaryOp::IntAdd),
                    BinaryOp::Sub => Some(BinaryOp::IntSub),
                    BinaryOp::Mul => Some(BinaryOp::IntMul),
                    BinaryOp::Div => Some(BinaryOp::IntDiv),
                    BinaryOp::Mod => Some(BinaryOp::IntMod),
                    BinaryOp::Pow => Some(BinaryOp::IntPow),
                    _ => None,
                };
                let integer = matches!(common, ValueType::Integer | ValueType::Boolean);
                if let Some(op) = arithmetic
                    && integer
                    && !wide
                {
                    binary.op = op;
                }
                let value_type = match binary.op {
                    BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge
                    | BinaryOp::And
                    | BinaryOp::Or => ValueType::Boolean,
                    BinaryOp::BitAnd
                    | BinaryOp::BitOr
                    | BinaryOp::BitXor
                    | BinaryOp::Shl
                    | BinaryOp::Shr => {
                        SemanticAnalyzer::validate_integer_operator_operand(
                            left,
                            "left operand of bitwise or shift operator",
                            binary.left.span(),
                        )?;
                        SemanticAnalyzer::validate_integer_operator_operand(
                            right,
                            "right operand of bitwise or shift operator",
                            binary.right.span(),
                        )?;
                        ValueType::Integer
                    }
                    BinaryOp::IntAdd
                    | BinaryOp::IntSub
                    | BinaryOp::IntMul
                    | BinaryOp::IntDiv
                    | BinaryOp::IntMod
                    | BinaryOp::IntPow => ValueType::Integer,
                    _ => common,
                };
                let expression = Expression::Binary(binary);
                if wide && integer {
                    // Retain exact wide constant arithmetic until it has
                    // collapsed, including comparisons above f64's precision.
                    // The runtime's integer storage is explicitly signed-32.
                    let Some(ConstantValue::Integer(value)) =
                        SemanticAnalyzer::eval_const_value_with(&expression, &HashMap::new())
                    else {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::UnsupportedFeature(
                                "runtime analog integer arithmetic wider than 32 bits".into(),
                            ),
                            expression.span(),
                        )));
                    };
                    let expression = folded_integer_expression(value, &expression, value_type)?;
                    let wide = wide_integer_value(&expression);
                    (expression, value_type, wide)
                } else {
                    (expression, value_type, wide && integer)
                }
            }
            OperatorRewrite::Unary(unary) => {
                let (operand_type, wide) = types.pop().expect("unary operand typed");
                let value_type = match unary.op {
                    UnaryOp::ToInteger | UnaryOp::BitNot => ValueType::Integer,
                    UnaryOp::Not => ValueType::Boolean,
                    _ => operand_type,
                };
                if unary.op == UnaryOp::BitNot {
                    SemanticAnalyzer::validate_integer_operator_operand(
                        operand_type,
                        "operand of bitwise complement",
                        unary.operand.span(),
                    )?;
                }
                if unary.op == UnaryOp::Neg
                    && matches!(operand_type, ValueType::Integer | ValueType::Boolean)
                    && !wide
                {
                    let span = unary.span;
                    (
                        Expression::Binary(BinaryExpr {
                            op: BinaryOp::IntSub,
                            left: Box::new(exact_integer_expression(0, span)),
                            right: unary.operand,
                            span,
                        }),
                        ValueType::Integer,
                        false,
                    )
                } else if wide && matches!(unary.op, UnaryOp::Neg | UnaryOp::Pos) {
                    let expression = Expression::Unary(unary);
                    let Some(ConstantValue::Integer(value)) =
                        SemanticAnalyzer::eval_const_value_with(&expression, &HashMap::new())
                    else {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::UnsupportedFeature(
                                "runtime analog integer arithmetic wider than 32 bits".into(),
                            ),
                            expression.span(),
                        )));
                    };
                    let expression = folded_integer_expression(value, &expression, value_type)?;
                    let wide = wide_integer_value(&expression);
                    (expression, value_type, wide)
                } else {
                    (Expression::Unary(unary), value_type, false)
                }
            }
        };
        types.push((value_type, wide));
        // Fold only successful literal arithmetic. Failed operations remain
        // executable so an untaken branch does not acquire a compile error.
        if let Expression::Binary(binary) = &expression
            && let Some(op) = binary.op.integer_arithmetic()
            && let (Expression::Number(left), Expression::Number(right)) =
                (&*binary.left, &*binary.right)
            && let Ok(value) =
                crate::integer_runtime::integer_arithmetic(op, left.value, right.value)
        {
            return Ok(exact_integer_expression(value as i64, binary.span));
        }
        Ok(expression)
    })
}

fn exact_integer_expression(value: i64, span: Span) -> Expression {
    Expression::Number(NumberLit {
        value: value as f64,
        raw: value.to_string().into(),
        span,
    })
}

fn wide_integer_value(expression: &Expression) -> bool {
    if explicit_integer_shape(expression).is_some_and(|(width, _)| width > 32) {
        return true;
    }
    match expression {
        Expression::Number(number) => SemanticAnalyzer::integer_literal_value(number)
            .is_some_and(|v| i32::try_from(v).is_err()),
        Expression::Conditional(conditional) => {
            wide_integer_value(&conditional.then_expr) || wide_integer_value(&conditional.else_expr)
        }
        _ => false,
    }
}

/// Retain explicit wide-literal sizing when a constant subtree collapses.
/// Replacing `64'sd1 + 1` by an unsized `2` would narrow its parent's operation.
fn folded_integer_expression(
    value: i64,
    expression: &Expression,
    value_type: ValueType,
) -> CompileResult<Expression> {
    let shape = (value_type == ValueType::Integer)
        .then(|| explicit_integer_shape(expression))
        .flatten();
    let Some((width, signed)) = shape.filter(|(width, _)| *width > 32) else {
        return Ok(exact_integer_expression(value, expression.span()));
    };
    let bits = if width == 64 {
        value as u64
    } else {
        (value as u64) & ((1_u64 << width) - 1)
    };
    let raw = format!("{width}'{}h{bits:x}", if signed { "s" } else { "" });
    let value = parse_integer_literal(&raw)
        .map_err(|message| {
            CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(message),
                expression.span(),
            ))
        })?
        .expect("integer syntax");
    Ok(Expression::Number(NumberLit {
        value: value as f64,
        raw: raw.into(),
        span: expression.span(),
    }))
}

fn explicit_integer_shape(expression: &Expression) -> Option<(u32, bool)> {
    match expression {
        Expression::Number(number) => {
            let (width, digits) = number.raw.split_once('\'')?;
            let width = width.replace('_', "").parse().ok()?;
            Some((width, digits.starts_with(['s', 'S'])))
        }
        Expression::Unary(unary) if matches!(unary.op, UnaryOp::Neg | UnaryOp::Pos) => {
            explicit_integer_shape(&unary.operand)
        }
        Expression::Binary(binary) => {
            let left = explicit_integer_shape(&binary.left);
            let right = explicit_integer_shape(&binary.right);
            match (left, right) {
                (Some((lw, ls)), Some((rw, rs))) => Some((lw.max(rw), ls && rs)),
                (Some(shape), None) | (None, Some(shape)) => Some(shape),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Numeric value retained by compile-time evaluation.
///
/// Verilog-AMS arithmetic is type-sensitive: notably, `1 / 2` is integer
/// division while `1 / 2.0` is real division. Keeping only an `f64` here
/// silently erased that distinction before defaults, bounds, and loop limits
/// were validated.
#[derive(Debug, Clone, Copy, PartialEq)]
enum ConstantValue {
    Integer(i64),
    Real(f64),
}

impl ConstantValue {
    fn numeric_order(self, other: Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Integer(left), Self::Integer(right)) => Some(left.cmp(&right)),
            _ => self.as_f64().partial_cmp(&other.as_f64()),
        }
    }

    fn as_exact_i64(self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(value),
            Self::Real(value) => SemanticAnalyzer::exact_const_i64(value),
        }
    }

    fn as_f64(self) -> f64 {
        match self {
            Self::Integer(value) => value as f64,
            Self::Real(value) => value,
        }
    }

    fn is_real(self) -> bool {
        matches!(self, Self::Real(_))
    }

    fn is_truthy(self) -> bool {
        self.as_f64() != 0.0
    }
}

/// Diagnostic code for a system task that parses and is then discarded.
const NO_EFFECT_SYSTEM_TASK_CODE: &str = "VA-SEM-NO-EFFECT-SYSTEM-TASK";

/// Diagnostic code for `posedge`/`negedge` lowered to an analog `cross`.
const EDGE_EVENT_AS_CROSS_CODE: &str = "VA-SEM-EDGE-EVENT-AS-CROSS";

/// Widest packed vector this front end will resolve.
///
/// Nothing materializes a bit per entry yet, but a declared width becomes an
/// allocation the moment a later wave lowers one, and the bound belongs with
/// the declaration that produced it.
pub(crate) const MAX_DIGITAL_VECTOR_WIDTH: u32 = 65_536;

mod analyzed;
mod digital;
mod digital_elaborate;
mod digital_walk;
mod elaboration;
mod flow_probes;
mod function_effects;
mod implicit_integrator;
mod switch_branches;
mod symbols;

pub use analyzed::*;
pub use digital::*;
pub(crate) use elaboration::elaborate_executable_module;
pub(crate) use flow_probes::lower as lower_flow_probes;
pub use symbols::*;

// ============================================================================
// Semantic Analyzer
// ============================================================================

/// Semantic analyzer for Verilog-A modules
pub struct SemanticAnalyzer {
    disciplines: DisciplineDb,
    functions: FunctionRegistry,
    symbols: SymbolTable,
    errors: Vec<SemanticError>,
    /// User-defined analog functions of the module under analysis
    user_functions: HashMap<SmolStr, FunctionDef>,
    effectful_functions: HashSet<SmolStr>,
    /// Stack of active guard conditions (innermost last)
    guard_stack: Vec<Expression>,
    /// Structured regions being built, innermost last.
    ///
    /// Runs alongside `guard_stack` and records the shape the guards are about
    /// to erase. The bottom frame is the module's analog block; each nested
    /// construct pushes a frame and pops it into an [`AnalyzedRegion`].
    region_stack: Vec<Vec<AnalyzedRegion>>,
    /// Stack of identifier substitution frames (innermost last). Used for
    /// hoisted block locals, unrolled loop variables, and inlined function
    /// locals.
    subst_stack: Vec<HashMap<SmolStr, Expression>>,
    /// Caller-visible assignments produced by analog function output/inout
    /// arguments while expression lowering is in progress.
    function_side_effects: Vec<AssignmentStmt>,
    /// Counter for generating unique hoisted local names
    local_counter: usize,
    /// Next [`AnalogSiteId`] to mint.
    ///
    /// One id names one analog-block step, and it is stamped on *both* the
    /// structured region and the guard-folded statement the analyzer records
    /// for that step, so canonical lowering can pair the two lowerings of the
    /// module without reconstructing the correspondence.
    next_analog_site: u32,
    in_analog_initial: bool,
    /// Constant parameter default values (compile-time diagnostics only:
    /// instances may override parameters, so these must never influence
    /// generated code)
    param_consts: HashMap<SmolStr, ConstantValue>,
    /// Values that cannot vary per instance (localparams derived purely
    /// from literals). Safe for loop unrolling and code folding.
    invariant_consts: HashMap<SmolStr, ConstantValue>,
    /// Current function inlining depth (recursion guard)
    inline_depth: usize,
    /// Nesting depth of runtime-bounded loops (contributions inside them
    /// are not representable and must error)
    runtime_loop_depth: usize,
    /// Nesting depth of control flow whose selector can change during an
    /// analysis. Stateful analog operators must execute on every Newton
    /// iteration and are therefore illegal beneath such a guard (VAMS-2023
    /// section 4.5.15).
    dynamic_analog_operator_guard_depth: usize,
    current_default_transition: f64,
    current_time_scale: crate::time_scale::ModuleTimeScale,
    /// Array variables of the module under analysis (name -> layout)
    arrays: HashMap<SmolStr, AnalyzedArray>,
    /// Public parameter-array declarations. Until element lowering is wired
    /// through every backend, these names must never acquire scalar semantics.
    parameter_arrays: HashSet<SmolStr>,
    /// Hidden system-task variables ($bound_step, $discontinuity)
    /// registered on first use
    task_vars: HashMap<SmolStr, usize>,
    /// Resets belong to the evaluation prologue even when first used in a loop.
    task_resets: Vec<AnalyzedAssignment>,
    /// Snapshotted guards for enclosing unfiltered `initial_step` events.
    unfiltered_initial_step_guards: Vec<SmolStr>,
    /// Non-fatal findings for the whole file under analysis, deduplicated by
    /// code and span so a construct inside a statically unrolled loop is
    /// reported once per source site rather than once per iteration.
    warnings: Vec<SemanticWarning>,
    /// Declarative regions of the digital process being analyzed, innermost
    /// last (IEEE 1364-2005 section 9.8.1).
    ///
    /// Empty everywhere except inside one process's statement walk. It is a
    /// stack rather than a parameter for the same reason `guard_stack` is: the
    /// walk is recursive over a dozen statement and expression forms, and
    /// threading a scope through every one of them would put the same argument
    /// in every signature to be used by three.
    digital_scopes: Vec<Vec<digital::AnalyzedProcessLocal>>,
    /// Dense identity assigned once, before an analyzed expression is cloned
    /// into the flat compatibility stream and the structured CFG body.
    next_noise_process: u32,
    implicit_integrators: Vec<implicit_integrator::ImplicitIntegrator>,
}

/// How an event expression lowers into the dataflow representation
enum EventLowering {
    /// Body executes when this runtime guard is nonzero
    Guard(Expression),
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzer {
    pub const SIMULATOR_DEFAULT_TRANSITION: f64 = 1.0e-9;

    pub fn new() -> Self {
        Self {
            disciplines: DisciplineDb::with_standard(),
            functions: FunctionRegistry::new(),
            symbols: SymbolTable::new(),
            errors: Vec::new(),
            user_functions: HashMap::new(),
            effectful_functions: HashSet::new(),
            guard_stack: Vec::new(),
            region_stack: vec![Vec::new()],
            subst_stack: Vec::new(),
            function_side_effects: Vec::new(),
            local_counter: 0,
            next_analog_site: 0,
            in_analog_initial: false,
            param_consts: HashMap::new(),
            invariant_consts: HashMap::new(),
            inline_depth: 0,
            runtime_loop_depth: 0,
            dynamic_analog_operator_guard_depth: 0,
            current_default_transition: Self::SIMULATOR_DEFAULT_TRANSITION,
            current_time_scale: crate::time_scale::ModuleTimeScale::default(),
            arrays: HashMap::new(),
            parameter_arrays: HashSet::new(),
            task_vars: HashMap::new(),
            task_resets: Vec::new(),
            unfiltered_initial_step_guards: Vec::new(),
            warnings: Vec::new(),
            digital_scopes: Vec::new(),
            next_noise_process: 0,
            implicit_integrators: Vec::new(),
        }
    }

    pub fn analyze(&mut self, source: &SourceFile) -> CompileResult<AnalyzedFile> {
        let mut modules = HashMap::new();
        let mut module_spans = HashMap::new();
        self.warnings.clear();

        self.register_physical_definitions(source)?;

        // Second pass: analyze modules in declaration order while applying
        // the file-scoped default-transition and default-discipline settings.
        let mut default_transition = Self::SIMULATOR_DEFAULT_TRANSITION;
        let mut default_discipline: Option<SmolStr> = None;
        for item in &source.items {
            if let Item::DefaultDiscipline(directive) = item {
                // Section 10.2 makes this "a default discrete discipline",
                // and Annex F.2.1 step 4b only applies it where the net's
                // domain matches, so a continuous one could never take effect.
                if let Some(name) = &directive.discipline {
                    match self.disciplines.get_discipline(name) {
                        None => {
                            return Err(CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::UndefinedDiscipline(name.to_string()),
                                directive.span,
                            )));
                        }
                        Some(discipline)
                            if discipline.domain != crate::disciplines::Domain::Discrete =>
                        {
                            return Err(CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::UnsupportedFeature(format!(
                                    "`default_discipline {name}` names a continuous discipline; \
                                     Verilog-AMS LRM 2.4 section 10.2 makes the default a \
                                     discrete discipline"
                                )),
                                directive.span,
                            )));
                        }
                        Some(_) => {}
                    }
                }
                default_discipline = directive.discipline.clone();
            } else if let Item::DefaultTransition(directive) = item {
                let Some(value) = Self::eval_const_with(&directive.value, &HashMap::new()) else {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(
                            "`default_transition` requires a numeric constant expression".into(),
                        ),
                        directive.span,
                    )));
                };
                if !value.is_finite() || value < 0.0 {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(format!(
                            "`default_transition` must be finite and non-negative, got {value}"
                        )),
                        directive.span,
                    )));
                }
                default_transition = value;
            } else if let Item::Module(module) = item {
                if let Some(first_defined) = module_spans.insert(module.name.clone(), module.span) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::DuplicateSymbol {
                            name: module.name.clone(),
                            first_defined,
                        },
                        module.span,
                    )));
                }
                self.symbols = SymbolTable::new();
                self.errors.clear();
                self.user_functions.clear();
                self.guard_stack.clear();
                self.region_stack.clear();
                self.region_stack.push(Vec::new());
                self.subst_stack.clear();
                self.function_side_effects.clear();
                self.local_counter = 0;
                self.next_analog_site = 0;
                self.param_consts.clear();
                self.invariant_consts.clear();
                self.inline_depth = 0;
                self.runtime_loop_depth = 0;
                self.dynamic_analog_operator_guard_depth = 0;
                self.current_default_transition = default_transition;
                self.parameter_arrays.clear();
                self.next_noise_process = 0;

                match self.analyze_module(module, default_transition) {
                    Ok(mut analyzed) => {
                        analyzed.default_discipline = default_discipline.clone();
                        modules.insert(module.name.clone(), analyzed);
                    }
                    Err(e) => return Err(e),
                }
            }
        }

        // Third pass: the clause 7 connect specification. It runs after the
        // discipline database is populated, because every check it makes is
        // against a discipline, and after the modules because a `connect`
        // statement's diagnostics are less useful than a broken module's.
        let connect_rules = crate::connect::build_connect_rule_table(source, &self.disciplines)
            .map_err(|error| {
                let span = connect_rules_span(source);
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::ConnectRules(Box::new(error)),
                    span,
                ))
            })?;

        self.warnings.sort_by_key(|warning| warning.span.start);
        Ok(AnalyzedFile {
            source: source.clone(),
            disciplines: self.disciplines.clone(),
            modules,
            warnings: std::mem::take(&mut self.warnings),
            connect_rules,
        })
    }

    /// Record a non-fatal finding once per source site.
    ///
    /// Static loop unrolling and analog-function inlining re-walk the same
    /// statements, so a repeat of an identical code and span is the same
    /// construct seen twice, not a second occurrence in the source.
    fn warn(&mut self, code: &'static str, message: String, span: Span) {
        if self
            .warnings
            .iter()
            .any(|warning| warning.code == code && warning.span == span)
        {
            return;
        }
        self.warnings.push(SemanticWarning {
            code,
            message,
            span,
        });
    }

    /// Inspect the physical declaration closure without analyzing module bodies.
    pub(crate) fn physical_definitions(
        mut self,
        source: &SourceFile,
    ) -> CompileResult<DisciplineDb> {
        self.register_physical_definitions(source)?;
        Ok(self.disciplines)
    }

    fn register_physical_definitions(&mut self, source: &SourceFile) -> CompileResult<()> {
        let mut declared = HashSet::new();
        for item in &source.items {
            if let Item::Nature(nature) = item {
                if !declared.insert(nature.name.as_str()) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "duplicate nature declaration '{}'",
                            nature.name
                        )),
                        nature.span,
                    )));
                }
                self.register_nature(nature)?;
            }
        }
        // Calculus relationships can refer forward, including the reciprocal
        // Current/Charge and Voltage/Flux declarations in disciplines.vams.
        for item in &source.items {
            if let Item::Nature(definition) = item {
                let nature = &self.disciplines.natures[definition.name.as_str()];
                for (attribute, target) in [
                    ("idt_nature", &nature.idt_nature),
                    ("ddt_nature", &nature.ddt_nature),
                ] {
                    if let Some(target) = target
                        && self.disciplines.get_nature(target).is_none()
                    {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::UnsupportedFeature(format!(
                                "nature '{}' {attribute} references undefined nature '{target}'",
                                definition.name
                            )),
                            definition.span,
                        )));
                    }
                }
                if let Some(base) = nature
                    .base
                    .as_deref()
                    .and_then(|name| self.disciplines.get_nature(name))
                {
                    for (attribute, target, inherited) in [
                        ("idt_nature", &nature.idt_nature, &base.idt_nature),
                        ("ddt_nature", &nature.ddt_nature, &base.ddt_nature),
                    ] {
                        let target = target.as_deref().unwrap_or(&nature.name);
                        let inherited = inherited.as_deref().unwrap_or(&base.name);
                        if self.disciplines.nature_base(target)
                            != self.disciplines.nature_base(inherited)
                        {
                            return Err(CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::UnsupportedFeature(format!(
                                    "derived nature '{}' {attribute} must share the base nature of '{inherited}'",
                                    definition.name
                                )),
                                definition.span,
                            )));
                        }
                    }
                }
            }
        }
        for item in &source.items {
            if let Item::Discipline(discipline) = item {
                self.register_discipline(discipline)?;
            }
        }
        Ok(())
    }

    fn register_nature(&mut self, nature: &NatureDef) -> CompileResult<()> {
        let mut ancestor = nature.base.as_deref();
        let mut depth = 0;
        while let Some(name) = ancestor {
            if name == nature.name || depth >= self.disciplines.natures.len() {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "nature '{}' has cyclic inheritance",
                        nature.name
                    )),
                    nature.span,
                )));
            }
            depth += 1;
            ancestor = self
                .disciplines
                .get_nature(name)
                .and_then(|nature| nature.base.as_deref());
        }
        let base = nature
            .base
            .as_deref()
            .map(|base| {
                self.disciplines.get_nature(base).ok_or_else(|| {
                    CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "nature '{}' extends unknown base nature '{}'",
                            nature.name, base
                        )),
                        nature.span,
                    ))
                })
            })
            .transpose()?;

        let access = nature
            .access
            .as_ref()
            .map(|s| s.to_string())
            .or_else(|| base.map(|base| base.access.clone()))
            .ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::MissingAttribute(format!(
                        "access for nature '{}'",
                        nature.name
                    )),
                    nature.span,
                ))
            })?;
        if let Some(base) = base
            && access != base.access
        {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "derived nature '{}' cannot change the inherited access function '{}'",
                    nature.name, base.access
                )),
                nature.span,
            )));
        }
        if base.is_some() && nature.units.is_some() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "derived nature '{}' cannot redeclare inherited units",
                    nature.name
                )),
                nature.span,
            )));
        }
        let units = nature
            .units
            .as_ref()
            .map(|s| s.to_string())
            .or_else(|| base.map(|base| base.units.clone()))
            .ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::MissingAttribute(format!(
                        "units for nature '{}'",
                        nature.name
                    )),
                    nature.span,
                ))
            })?;
        let abstol = if let Some(expression) = &nature.abstol {
            Self::eval_const_with(expression, &HashMap::new())
                .filter(|value| value.is_finite() && *value >= 0.0)
                .ok_or_else(|| {
                    CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "nature '{}' abstol must be a finite, non-negative real constant",
                            nature.name
                        )),
                        expression.span(),
                    ))
                })?
        } else {
            base.map(|base| base.abstol).ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::MissingAttribute(format!(
                        "abstol for nature '{}'",
                        nature.name
                    )),
                    nature.span,
                ))
            })?
        };

        self.disciplines.add_nature(Nature {
            name: nature.name.to_string(),
            base: nature.base.as_ref().map(ToString::to_string),
            units,
            abstol,
            access,
            idt_nature: nature
                .idt_nature
                .as_ref()
                .map(|s| s.to_string())
                .or_else(|| base.and_then(|base| base.idt_nature.clone())),
            ddt_nature: nature
                .ddt_nature
                .as_ref()
                .map(|s| s.to_string())
                .or_else(|| base.and_then(|base| base.ddt_nature.clone())),
            span: Some(nature.span),
        });
        Ok(())
    }

    fn register_discipline(&mut self, discipline: &DisciplineDef) -> CompileResult<()> {
        for nature_name in [discipline.potential.as_ref(), discipline.flow.as_ref()]
            .into_iter()
            .flatten()
        {
            if self.disciplines.get_nature(nature_name).is_none() {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UndefinedDiscipline(format!(
                        "discipline '{}' references unknown nature '{}'",
                        discipline.name, nature_name
                    )),
                    discipline.span,
                )));
            }
        }

        let domain = match discipline.domain.unwrap_or(DomainKind::Continuous) {
            DomainKind::Continuous => Domain::Continuous,
            DomainKind::Discrete => Domain::Discrete,
        };
        self.disciplines.add_discipline(Discipline {
            name: discipline.name.to_string(),
            domain,
            potential: discipline.potential.as_ref().map(|s| s.to_string()),
            flow: discipline.flow.as_ref().map(|s| s.to_string()),
            span: Some(discipline.span),
        });
        Ok(())
    }

    fn require_discipline(&self, name: &str, span: Span) -> CompileResult<()> {
        if self.disciplines.get_discipline(name).is_none() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndefinedDiscipline(name.to_string()),
                span,
            )));
        }
        Ok(())
    }

    fn analyze_module(
        &mut self,
        module: &Module,
        default_transition: f64,
    ) -> CompileResult<AnalyzedModule> {
        self.current_time_scale = module.time_scale;
        let mut analyzed = AnalyzedModule {
            name: module.name.clone(),
            default_transition,
            default_discipline: None,
            noise_process_count: 0,
            ports: Vec::new(),
            parameters: Vec::new(),
            param_aliases: Vec::new(),
            variables: Vec::new(),
            event_state_variables: Vec::new(),
            switch_branch_variables: Vec::new(),
            branches: Vec::new(),
            contributions: Vec::new(),
            statements: Vec::new(),
            prologue_statements: Vec::new(),
            body: Vec::new(),
            analog_site_count: 0,
            internal_nodes: Vec::new(),
            ground_nodes: Vec::new(),
            arrays: HashMap::new(),
            symbol_table: SymbolTable::new(),
            digital: AnalyzedDigital::default(),
        };
        // Evaluation statements accumulate in a local sink so loop bodies
        // can recurse into their own sinks without aliasing the module
        let mut statements: Vec<AnalyzedStatement> = Vec::new();
        self.user_functions = module
            .functions
            .iter()
            .map(|f| (f.name.clone(), f.clone()))
            .collect();
        self.effectful_functions = function_effects::effectful_functions(&self.user_functions);
        self.in_analog_initial = false;
        self.implicit_integrators.clear();
        self.arrays.clear();
        self.task_vars.clear();
        self.task_resets.clear();

        // Phase 1: Collect port names from module header
        let port_names: Vec<SmolStr> = module.ports.iter().map(|p| p.name.clone()).collect();

        // Phase 2: Process port declarations to get direction and discipline
        let mut port_info: HashMap<SmolStr, (PortDirection, Option<SmolStr>)> = HashMap::new();
        let mut declared_ports: HashMap<SmolStr, Span> = HashMap::new();
        for decl in &module.port_declarations {
            if let Some(discipline) = &decl.discipline {
                self.require_discipline(discipline, decl.span)?;
            }
            for name in &decl.names {
                if let Some(first_defined) = declared_ports.get(name) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::DuplicateSymbol {
                            name: name.clone(),
                            first_defined: *first_defined,
                        },
                        decl.span,
                    )));
                }
                declared_ports.insert(name.clone(), decl.span);
                port_info.insert(name.clone(), (decl.direction, decl.discipline.clone()));
            }
        }

        // Phase 3: Resolve all declarations before allocating nodes. Ground
        // qualifies a net; it must not lose to an earlier discipline declaration
        // or overwrite that declaration with an implicit electrical discipline.
        let mut net_disciplines: HashMap<SmolStr, SmolStr> = port_info
            .iter()
            .filter_map(|(name, (_, discipline))| {
                discipline
                    .clone()
                    .map(|discipline| (name.clone(), discipline))
            })
            .collect();
        let mut ground_names = std::collections::HashSet::new();
        for net in &module.nets {
            if let Some(discipline) = &net.discipline {
                self.require_discipline(discipline, net.span)?;
            }
            for name in &net.names {
                if let Some(discipline) = &net.discipline {
                    if let Some(previous) = net_disciplines.insert(name.clone(), discipline.clone())
                        && previous != *discipline
                    {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::IncompatibleDisciplines(
                                previous.to_string(),
                                discipline.to_string(),
                            ),
                            net.span,
                        )));
                    }
                }
                if net.is_ground {
                    ground_names.insert(name.clone());
                }
            }
        }
        for net in module.nets.iter().filter(|net| net.is_ground) {
            for name in &net.names {
                // Preserve the existing shorthand `ground g;` for undeclared
                // electrical nets, while inheriting every explicit discipline.
                let discipline = net_disciplines
                    .get(name)
                    .map_or("electrical", |d| d.as_str());
                if self
                    .disciplines
                    .get_discipline(discipline)
                    .is_some_and(|discipline| {
                        discipline.domain != crate::disciplines::Domain::Continuous
                    })
                {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidNodeReference {
                            name: name.clone(),
                            kind:
                                "discrete net declared as ground (requires a continuous discipline)"
                                    .into(),
                        },
                        net.span,
                    )));
                }
                if port_names.contains(name) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "ground declaration on module port '{name}'; connect this port to ground in the containing circuit"
                        )),
                        net.span,
                    )));
                }
            }
        }

        // Phase 4: Define ports in symbol table FIRST
        for port_name in &port_names {
            let (direction, discipline) = port_info
                .get(port_name)
                .cloned()
                .unwrap_or((PortDirection::Inout, None));

            let disc_name = net_disciplines
                .get(port_name)
                .cloned()
                .or(discipline)
                .unwrap_or_else(|| "electrical".into());

            let disc = self.disciplines.get_discipline(&disc_name).ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UndefinedDiscipline(disc_name.to_string()),
                    module.span,
                ))
            })?;
            let potential = disc.potential.as_ref().map(|s| SmolStr::from(s.as_str()));
            let flow = disc.flow.as_ref().map(|s| SmolStr::from(s.as_str()));

            analyzed.ports.push(AnalyzedPort {
                name: port_name.clone(),
                direction,
                discipline: disc_name.clone(),
                nature_potential: potential,
                nature_flow: flow,
            });

            self.define_symbol(Symbol {
                name: port_name.clone(),
                kind: SymbolKind::Port,
                value_type: ValueType::NatureAccess,
                span: Span::new(crate::source::SourceId::new(0), 0, 0),
                attrs: SymbolAttrs {
                    direction: Some(direction),
                    discipline: Some(disc_name),
                    ..Default::default()
                },
            })?;
        }

        // Phase 5: Define internal and ground nodes (nets that aren't ports)
        let mut internal_node_idx = 0usize;
        for net in &module.nets {
            for name in &net.names {
                // Skip if already defined as a port
                if self.symbols.lookup_local(name).is_some() {
                    continue;
                }

                let discipline = net_disciplines
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| "electrical".into());
                if ground_names.contains(name) {
                    // Ground nets reference the global reference node and
                    // must not consume an internal node slot.
                    self.define_symbol(Symbol {
                        name: name.clone(),
                        kind: SymbolKind::Node,
                        value_type: ValueType::NatureAccess,
                        span: net.span,
                        attrs: SymbolAttrs {
                            discipline: Some(discipline.clone()),
                            is_ground: true,
                            ..Default::default()
                        },
                    })?;
                    analyzed.ground_nodes.push(name.clone());
                    continue;
                }

                // Define as internal node
                self.define_symbol(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Node,
                    value_type: ValueType::NatureAccess,
                    span: net.span,
                    attrs: SymbolAttrs {
                        discipline: Some(discipline.clone()),
                        is_internal: true,
                        internal_node_index: Some(internal_node_idx),
                        ..Default::default()
                    },
                })?;

                // Add to analyzed internal nodes
                analyzed.internal_nodes.push(AnalyzedInternalNode {
                    is_state: false,
                    name: name.clone(),
                    discipline: discipline.clone(),
                    index: internal_node_idx,
                });
                internal_node_idx += 1;
            }
        }

        // Phase 6: Named branch declarations
        for branch in &module.branches {
            self.validate_node(&branch.pos, branch.span)?;
            if !branch.neg.is_empty() {
                self.validate_node(&branch.neg, branch.span)?;
            }
            self.validate_distinct_branch_nodes(&branch.pos, &branch.neg, branch.span)?;
            let discipline_node = if ground_names.contains(&branch.pos) {
                &branch.neg
            } else {
                &branch.pos
            };
            let discipline = self
                .symbols
                .lookup(discipline_node)
                .and_then(|s| s.attrs.discipline.clone())
                .unwrap_or_else(|| "electrical".into());

            analyzed.branches.push(AnalyzedBranch {
                name: branch.name.clone(),
                pos_node: branch.pos.clone(),
                neg_node: branch.neg.clone(),
                discipline: discipline.clone(),
            });

            self.define_symbol(Symbol {
                name: branch.name.clone(),
                kind: SymbolKind::Branch,
                value_type: ValueType::NatureAccess,
                span: branch.span,
                attrs: SymbolAttrs {
                    discipline: Some(discipline),
                    ..Default::default()
                },
            })?;
        }

        // Phase 7: Analyze parameters (defaults may reference earlier ones)
        let param_names: std::collections::HashSet<SmolStr> =
            module.parameters.iter().map(|p| p.name.clone()).collect();
        let parameter_scopes: Vec<_> = module
            .parameters
            .iter()
            .map(|parameter| self.parameter_scope(parameter))
            .collect();
        let parameter_also_model: Vec<_> = module
            .parameters
            .iter()
            .zip(&parameter_scopes)
            .map(|(parameter, scope)| self.parameter_also_model(parameter, *scope))
            .collect();
        let canonical_model_storage = module
            .parameters
            .iter()
            .zip(&parameter_scopes)
            .zip(&parameter_also_model)
            .map(|((parameter, scope), also_model)| {
                (
                    parameter.name.clone(),
                    *scope == ParameterScope::Model || *also_model,
                )
            })
            .collect::<std::collections::HashMap<_, _>>();
        let mut external_model_storage = canonical_model_storage
            .iter()
            .map(|(name, has_model_storage)| (name.to_ascii_lowercase(), *has_model_storage))
            .collect::<std::collections::HashMap<_, _>>();
        for alias in &module.aliasparams {
            if let Some(has_model_storage) = canonical_model_storage.get(&alias.target) {
                external_model_storage.insert(alias.alias.to_ascii_lowercase(), *has_model_storage);
            }
        }
        // Preserve the first case-sensitive declaration so duplicate names do
        // not perturb dependency order before the symbol-table diagnostic.
        // The same table serves scalar defaults and array bounds.
        let parameter_indices = module.parameters.iter().enumerate().fold(
            HashMap::new(),
            |mut indices, (index, parameter)| {
                indices.entry(parameter.name.clone()).or_insert(index);
                indices
            },
        );
        self.parameter_arrays.extend(
            module
                .parameters
                .iter()
                .filter(|parameter| !parameter.dimensions.is_empty())
                .map(|parameter| parameter.name.clone()),
        );
        self.validate_parameter_default_dependencies(
            &module.parameters,
            &module.aliasparams,
            &parameter_indices,
        );
        for (parameter_index, ((param, scope), also_model)) in module
            .parameters
            .iter()
            .zip(parameter_scopes)
            .zip(parameter_also_model)
            .enumerate()
        {
            let is_parameter_array = !param.dimensions.is_empty();
            let mut materialized_array_default = if is_parameter_array {
                param
                    .default
                    .as_ref()
                    .map(|default| {
                        self.materialize_replication_expression(
                            default,
                            MAX_PARAMETER_ARRAY_ELEMENTS as usize,
                            MAX_REPLICATION_MATERIALIZATION_WORK,
                            &format!("default of parameter array '{}'", param.name),
                            false,
                        )
                    })
                    .transpose()?
            } else {
                None
            };
            if is_parameter_array {
                materialized_array_default = materialized_array_default
                    .map(|expression| self.normalize_integer_expression(&expression))
                    .transpose()?;
                self.validate_parameter_array_declaration(
                    param,
                    materialized_array_default.as_ref(),
                    parameter_index,
                    &module.parameters,
                    &parameter_indices,
                );
                if param.param_type == ParamType::Integer {
                    materialized_array_default = materialized_array_default
                        .map(|expression| self.coerce_integer_parameter_array_default(expression))
                        .transpose()?;
                }
            }
            if scope == ParameterScope::Model || also_model {
                let default_reads_instance = param.default.as_ref().is_some_and(|expression| {
                    Self::references_parameter_without_model_storage(
                        expression,
                        &canonical_model_storage,
                        &external_model_storage,
                    )
                });
                let shape_reads_instance = param.dimensions.iter().any(|dimension| {
                    [&dimension.start, &dimension.end]
                        .into_iter()
                        .any(|expression| {
                            Self::references_parameter_without_model_storage(
                                expression,
                                &canonical_model_storage,
                                &external_model_storage,
                            )
                        })
                });
                // Range bounds are validation expressions, not part of the
                // model-card value. CMC models legitimately constrain a model
                // parameter against instance geometry (for example XGL <= L).
                // Those bounds are evaluated after instance overrides and do
                // not make the model parameter itself instance-owned.
                if default_reads_instance {
                    let storage = if scope == ParameterScope::Model {
                        "model parameter"
                    } else {
                        "dual-scope parameter model fallback"
                    };
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "{storage} '{}' cannot depend on an instance parameter that lacks model storage",
                            param.name,
                        )),
                        param.span,
                    );
                }
                if shape_reads_instance {
                    let storage = if scope == ParameterScope::Model {
                        "model parameter array"
                    } else {
                        "dual-scope parameter-array model fallback"
                    };
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "{storage} '{}' cannot have dimensions that depend on an instance parameter lacking model storage",
                            param.name,
                        )),
                        param.span,
                    );
                }
            }
            let value_type = match param.param_type {
                ParamType::Real => ValueType::Real,
                ParamType::Integer => ValueType::Integer,
                ParamType::String => ValueType::String,
            };

            // Resolve the declaration-time value for diagnostics even when
            // the executable default must remain symbolic. Keeping those two
            // concerns separate lets a later array bound be checked through a
            // transitive chain without baking overridable values into code.
            let normalized_default = param
                .default
                .as_ref()
                .filter(|_| !is_parameter_array)
                .map(|expression| self.normalize_integer_expression(expression))
                .transpose()?;
            let declared_default_value = normalized_default
                .as_ref()
                .and_then(|expression| self.eval_const_value(expression));
            let declared_default = declared_default_value
                .and_then(|value| Self::constant_for_declared_type(value, param.param_type))
                .map(ConstantValue::as_f64);

            // A default that references other parameters must stay
            // symbolic: instance overrides of those parameters change it,
            // so it is evaluated per instance at setup time.
            let default_depends_on_parameters = param
                .default
                .as_ref()
                .is_some_and(|expression| Self::references_identifiers(expression, &param_names));
            let default = if is_parameter_array || default_depends_on_parameters {
                None
            } else {
                declared_default
            };

            if param.param_type == ParamType::Integer
                && !default_depends_on_parameters
                && let Some(default) = declared_default_value.map(ConstantValue::as_f64)
                && real_to_integer(default).is_err()
            {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "32-bit integer".into(),
                        found: default.to_string(),
                        context: format!("default of parameter '{}'", param.name),
                    },
                    param.span,
                );
            }

            if !is_parameter_array && let Some(declared_range) = &param.range {
                if declared_range.bounds.len() > 1 {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(
                            "multiple parameter 'from' ranges are not yet supported".into(),
                        ),
                        declared_range.span,
                    );
                }
                let has_unresolved_constraint = declared_range
                    .bounds
                    .iter()
                    .flat_map(|bound| [bound.lower.as_ref(), bound.upper.as_ref()])
                    .flatten()
                    .chain(declared_range.exclude.iter())
                    .any(|expression| {
                        !Self::references_identifiers(expression, &param_names)
                            && self.eval_const(expression).is_none()
                            && Self::direct_parameter_reference(expression, &param_names).is_none()
                    });
                if has_unresolved_constraint {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(
                            "parameter range expressions must be constant or depend only on parameters"
                                .into(),
                        ),
                        declared_range.span,
                    );
                }
                let own_name = std::collections::HashSet::from([param.name.clone()]);
                let has_self_referential_constraint = declared_range
                    .bounds
                    .iter()
                    .flat_map(|bound| [bound.lower.as_ref(), bound.upper.as_ref()])
                    .flatten()
                    .chain(declared_range.exclude.iter())
                    .any(|expression| Self::references_identifiers(expression, &own_name));
                if has_self_referential_constraint {
                    self.record_error_at(
                        SemanticErrorKind::CircularDependency(format!(
                            "range of parameter '{}'",
                            param.name
                        )),
                        declared_range.span,
                    );
                }
            }

            // Bounds may refer to later parameters. Resolve their operator
            // types after all scalar parameter symbols have been installed.
            let range = None;

            // Declared defaults can be placeholders outside the allowed range.
            // Validate the final instance after its overrides are installed.
            if let Some(value) = declared_default_value
                .and_then(|value| Self::constant_for_declared_type(value, param.param_type))
            {
                self.param_consts.insert(param.name.clone(), value);
            }

            analyzed.parameters.push(AnalyzedParameter {
                name: param.name.clone(),
                is_public: true,
                scope,
                also_model,
                param_type: param.param_type,
                value_type,
                dimensions: param
                    .dimensions
                    .iter()
                    .map(|dimension| AnalyzedParameterDimension {
                        left: dimension.start.clone(),
                        right: dimension.end.clone(),
                        span: dimension.span,
                    })
                    .collect(),
                default,
                default_expr: if is_parameter_array {
                    materialized_array_default
                } else {
                    normalized_default
                        .map(|expression| {
                            self.coerce_assignment_expression(expression, value_type)
                                .map(|(expression, _)| expression)
                        })
                        .transpose()?
                },
                range: range.clone(),
            });

            self.define_symbol(Symbol {
                name: param.name.clone(),
                kind: SymbolKind::Parameter,
                value_type,
                span: param.span,
                attrs: SymbolAttrs {
                    range,
                    ..Default::default()
                },
            })?;
        }

        for (parameter, analyzed_parameter) in
            module.parameters.iter().zip(&mut analyzed.parameters)
        {
            if parameter.dimensions.is_empty()
                && let Some(range) = &parameter.range
            {
                let mut range = range.clone();
                for bound in &mut range.bounds {
                    bound.lower = bound
                        .lower
                        .as_ref()
                        .map(|e| self.normalize_integer_expression(e))
                        .transpose()?;
                    bound.upper = bound
                        .upper
                        .as_ref()
                        .map(|e| self.normalize_integer_expression(e))
                        .transpose()?;
                }
                for excluded in &mut range.exclude {
                    *excluded = self.normalize_integer_expression(excluded)?;
                }
                let range = self.parse_range(&range, &param_names);
                analyzed_parameter.range = Some(range.clone());
                if let Some(symbol) = self.symbols.lookup_mut(&parameter.name) {
                    symbol.attrs.range = Some(range);
                }
            }
        }

        // Phase 7b: Parameter aliases (aliasparam). The target must be a
        // declared parameter; the alias name must not collide with any
        // other declaration. The alias enters the symbol table only to
        // reserve its name - it is not a parameter, so the module body
        // cannot reference it and it gets no storage or default.
        for decl in &module.aliasparams {
            let Some(target) = analyzed
                .parameters
                .iter()
                .position(|p| p.name == decl.target)
            else {
                self.record_error_at(
                    SemanticErrorKind::UndeclaredSymbol {
                        name: decl.target.clone(),
                    },
                    decl.span,
                );
                continue;
            };

            self.define_symbol(Symbol {
                name: decl.alias.clone(),
                kind: SymbolKind::ParamAlias,
                value_type: analyzed.parameters[target].value_type,
                span: decl.span,
                attrs: Default::default(),
            })?;

            analyzed.param_aliases.push(AnalyzedParamAlias {
                alias: decl.alias.clone(),
                target,
            });
        }

        // Pre-pass for Phase 8: seed the constant environments with
        // localparam values so array bounds may reference them (their full
        // lowering to computed variables happens in Phase 9)
        let mut localparam_defaults = Vec::with_capacity(module.localparams.len());
        for localparam in &module.localparams {
            if !localparam.dimensions.is_empty() {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "localparam array '{}' is retained with its declared dimensions, but array-valued localparam storage and indexing are not implemented",
                        localparam.name
                    )),
                    localparam.span,
                )));
            }
            let default = localparam
                .default
                .as_ref()
                .map(|e| self.normalize_integer_expression(e))
                .transpose()?;
            if let Some(default) = &default {
                if let Some(value) = self.eval_const_value(default).and_then(|value| {
                    Self::constant_for_declared_type(value, localparam.param_type)
                }) {
                    self.param_consts.insert(localparam.name.clone(), value);
                }
                if let Some(value) = self.eval_const_invariant_value(default).and_then(|value| {
                    Self::constant_for_declared_type(value, localparam.param_type)
                }) {
                    self.invariant_consts.insert(localparam.name.clone(), value);
                }
            }
            localparam_defaults.push(default);
            self.define_symbol(Symbol {
                name: localparam.name.clone(),
                kind: SymbolKind::Parameter,
                value_type: match localparam.param_type {
                    ParamType::Real => ValueType::Real,
                    ParamType::Integer => ValueType::Integer,
                    ParamType::String => ValueType::String,
                },
                span: localparam.span,
                attrs: Default::default(),
            })?;
        }

        // Phase 8: Analyze variables
        for var_decl in &module.variables {
            let value_type = match var_decl.var_type {
                VarType::Real => ValueType::Real,
                VarType::Integer => ValueType::Integer,
                VarType::String => ValueType::String,
            };

            for item in &var_decl.items {
                if !item.dimensions.is_empty() {
                    let name = item.name.clone();
                    if let Some(layout) =
                        self.register_array_variable(item, var_decl.var_type, &name, &mut analyzed)
                    {
                        analyzed.arrays.insert(name.clone(), layout.clone());
                        self.arrays.insert(name.clone(), layout);
                        self.define_symbol(Symbol {
                            name,
                            kind: SymbolKind::Variable,
                            value_type,
                            span: var_decl.span,
                            attrs: Default::default(),
                        })?;
                    }
                    continue;
                }

                analyzed.variables.push(AnalyzedVariable {
                    name: item.name.clone(),
                    var_type: var_decl.var_type,
                    value_type,
                    is_state: false,
                });

                self.define_symbol(Symbol {
                    name: item.name.clone(),
                    kind: SymbolKind::Variable,
                    value_type,
                    span: var_decl.span,
                    attrs: Default::default(),
                })?;
            }
        }

        let module_variable_count = analyzed.variables.len();

        // Phase 9: Lower localparams to computed variables. Their values may
        // depend on parameters, so they are evaluated at runtime before any
        // analog-block assignment, in declaration order.
        for (localparam, default) in module.localparams.iter().zip(&localparam_defaults) {
            let value_type = match localparam.param_type {
                ParamType::Real => ValueType::Real,
                ParamType::Integer => ValueType::Integer,
                ParamType::String => ValueType::String,
            };

            let Some(default) = default else {
                self.record_error_at(
                    SemanticErrorKind::MissingAttribute(format!(
                        "localparam '{}' requires a value",
                        localparam.name
                    )),
                    localparam.span,
                );
                continue;
            };

            let var_index = analyzed.variables.len();
            analyzed.variables.push(AnalyzedVariable {
                name: localparam.name.clone(),
                var_type: match localparam.param_type {
                    ParamType::Real => VarType::Real,
                    ParamType::Integer => VarType::Integer,
                    ParamType::String => VarType::String,
                },
                value_type,
                is_state: false,
            });

            let expression =
                self.lower_expression_with_side_effects(default, &mut analyzed, &mut statements)?;
            let (expression, expr_type) =
                self.coerce_assignment_expression(expression, value_type)?;
            // Prologue statements run before the analog block and have no
            // structured counterpart at all: the body starts at the `analog`
            // keyword. They are still stamped, because a site names an
            // executed step whether or not one of the two lowerings elides it.
            let site = self.next_analog_site();
            analyzed.prologue_statements.push(statements.len());
            statements.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
                target: localparam.name.clone(),
                var_index,
                index: None,
                expression,
                site,
                expression_guard: AnalogSiteGuard::None,
                expr_type,
                span: localparam.span,
                unfiltered_initial_step_guard: None,
            }));
        }

        // Phase 10: Module-level variable initializers run before the
        // analog initialization, in declaration order.
        self.in_analog_initial = true;
        let declaration_site = self.next_analog_site();
        let mut declaration_statements = Vec::new();
        self.open_region();
        for var_decl in &module.variables {
            for item in &var_decl.items {
                let Some(init) = &item.init else { continue };
                function_effects::validate_initializer_expression(init, &self.user_functions)?;

                if let Some(layout) = self.arrays.get(&item.name).cloned() {
                    // Array initializer: '{e0, e1, ...} fills the elements
                    // in declaration order
                    let Expression::ArrayLiteral(lit) = init else {
                        self.record_error_at(
                            SemanticErrorKind::TypeMismatch {
                                expected: "array literal".to_string(),
                                found: "scalar expression".to_string(),
                                context: format!("initializer of array '{}'", item.name),
                            },
                            item.span,
                        );
                        continue;
                    };
                    if lit.elements.len() != layout.len {
                        self.record_error_at(
                            SemanticErrorKind::TypeMismatch {
                                expected: format!("{} elements", layout.len),
                                found: format!("{} elements", lit.elements.len()),
                                context: format!("initializer of array '{}'", item.name),
                            },
                            item.span,
                        );
                        continue;
                    }
                    if let Some(replication) = lit.first_replication() {
                        self.record_error_at(
                            SemanticErrorKind::UnsupportedFeature(
                                "replication in executable array initializers is parsed but not yet supported; write the elements explicitly"
                                    .into(),
                            ),
                            replication.span,
                        );
                        continue;
                    }
                    for (offset, element) in lit.elements.iter().enumerate() {
                        let ArrayLiteralElement::Value(element) = element else {
                            unreachable!("replication was rejected before array lowering");
                        };
                        let var_index = layout.base + offset;
                        let expression = self.lower_expression_with_side_effects(
                            element,
                            &mut analyzed,
                            &mut declaration_statements,
                        )?;
                        let (expression, expr_type) = self.coerce_assignment_expression(
                            expression,
                            analyzed.variables[var_index].value_type,
                        )?;
                        let site = self.next_analog_site();
                        let assignment = AnalyzedAssignment {
                            target: analyzed.variables[var_index].name.clone(),
                            var_index,
                            index: None,
                            expression,
                            site,
                            expression_guard: AnalogSiteGuard::None,
                            expr_type,
                            span: item.span,
                            unfiltered_initial_step_guard: None,
                        };
                        self.record_region(AnalyzedRegion::Assignment(assignment.clone()));
                        declaration_statements.push(AnalyzedStatement::Assignment(assignment));
                    }
                    continue;
                }

                let var_index = analyzed
                    .variables
                    .iter()
                    .position(|v| v.name == item.name)
                    .expect("variable registered above");
                let expression = self.lower_expression_with_side_effects(
                    init,
                    &mut analyzed,
                    &mut declaration_statements,
                )?;
                let (expression, expr_type) = self.coerce_assignment_expression(
                    expression,
                    analyzed.variables[var_index].value_type,
                )?;
                let site = self.next_analog_site();
                let assignment = AnalyzedAssignment {
                    target: item.name.clone(),
                    var_index,
                    index: None,
                    expression,
                    site,
                    expression_guard: AnalogSiteGuard::None,
                    expr_type,
                    span: item.span,
                    unfiltered_initial_step_guard: None,
                };
                self.record_region(AnalyzedRegion::Assignment(assignment.clone()));
                declaration_statements.push(AnalyzedStatement::Assignment(assignment));
            }
        }

        let declaration_body = self.close_region();
        self.in_analog_initial = false;
        if !declaration_statements.is_empty() {
            Self::record_initial_state_variables(
                &declaration_statements,
                &mut analyzed,
                module_variable_count,
            );
            statements.push(AnalyzedStatement::Initialization {
                phase: AnalogEvaluationPhase::Declarations,
                site: declaration_site,
                body: declaration_statements,
                span: module.span,
            });
            self.record_region(AnalyzedRegion::Initialization {
                phase: AnalogEvaluationPhase::Declarations,
                body: declaration_body,
                span: module.span,
            });
        }

        // `analog final` parses into its own block and has no consumer: no
        // phase below reads it. Accepting the module would compile a device
        // whose end-of-analysis behavior the author wrote and the simulator
        // never runs, so it is refused by name.
        //
        // Lowering it as `@(final_step)` appended to the analog block was
        // considered and rejected. The analog block's assignments run *before*
        // the contributions on every device evaluation, so a final-block write
        // to a variable that a contribution reads would change the device stamp
        // at the last time point — the silent miscompile this fail-closed pass
        // exists to remove. `analog final` also runs at the end of every
        // analysis, while `final_step` is a per-analysis filtered transient
        // event, so the two are not the same event.
        if let Some(block) = &module.analog_final {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(
                    "`analog final` is parsed but never executed by this compiler; remove the \
                     block or move its work into the analog block"
                        .to_owned(),
                ),
                block.span,
            );
        }

        // Phase 10b: the discrete (IEEE 1364) half of the module. It runs
        // after every analog declaration is in the symbol table, so a digital
        // name that collides with one is caught and a process can read an
        // analog `integer` or `real`. Unlike `analog final`, digital content
        // is *accepted* here: it is refused at each executable backend
        // boundary instead, where the compiler would have to run it.
        self.analyze_digital(module, &mut analyzed);

        // Phase 11: Capture analog initialization independently of the Newton body.
        self.in_analog_initial = true;
        if let Some(block) = &module.analog_initial {
            function_effects::validate_initialization(&block.statements, &self.user_functions)?;
            let site = self.next_analog_site();
            let mut initialization = Vec::new();
            self.open_region();
            for stmt in &block.statements {
                self.analyze_statement(stmt, &mut analyzed, &mut initialization)?;
            }
            let body = self.close_region();
            Self::record_initial_state_variables(
                &initialization,
                &mut analyzed,
                module_variable_count,
            );
            statements.push(AnalyzedStatement::Initialization {
                phase: AnalogEvaluationPhase::Initialization,
                site,
                body: initialization,
                span: block.span,
            });
            self.record_region(AnalyzedRegion::Initialization {
                phase: AnalogEvaluationPhase::Initialization,
                body,
                span: block.span,
            });
        }
        self.in_analog_initial = false;

        // Phase 12: Analyze analog block
        if let Some(block) = &module.analog_block {
            function_effects::validate_control_tasks(&block.statements, &self.user_functions)?;
            for stmt in &block.statements {
                self.analyze_statement(stmt, &mut analyzed, &mut statements)?;
            }
        }

        self.finish_implicit_integrators(&mut analyzed);
        analyzed.statements = statements;
        analyzed.body = self.take_body();
        Self::prepend_task_resets(&mut analyzed, std::mem::take(&mut self.task_resets));
        analyzed.analog_site_count = self.next_analog_site;

        // Surface every recorded diagnostic instead of silently succeeding
        if !self.errors.is_empty() {
            let errors = std::mem::take(&mut self.errors)
                .into_iter()
                .map(CompileError::Semantic)
                .collect();
            return Err(CompileError::multiple(errors));
        }

        analyzed.symbol_table = self.symbols.clone();
        analyzed.noise_process_count = self.next_noise_process;
        Ok(analyzed)
    }

    fn prepend_task_resets(module: &mut AnalyzedModule, resets: Vec<AnalyzedAssignment>) {
        if resets.is_empty() {
            return;
        }
        let sites = resets
            .iter()
            .map(|assignment| assignment.site)
            .collect::<Vec<_>>();
        let remap = |site: AnalogSiteId| match sites.binary_search(&site) {
            Ok(index) => AnalogSiteId(index as u32),
            Err(before) => AnalogSiteId(site.0 - before as u32 + sites.len() as u32),
        };
        // Reaching definitions rely on site order matching execution order.
        // Move the reset sites to the front in both representations, retaining
        // the relative order and identity correspondence of every other site.
        fn statements(
            body: &mut [AnalyzedStatement],
            remap: &impl Fn(AnalogSiteId) -> AnalogSiteId,
        ) {
            for statement in body {
                match statement {
                    AnalyzedStatement::Assignment(assignment) => {
                        assignment.site = remap(assignment.site)
                    }
                    AnalyzedStatement::Loop(loop_) => {
                        loop_.site = remap(loop_.site);
                        statements(&mut loop_.body, remap);
                    }
                    AnalyzedStatement::Task(task) => task.site = remap(AnalogSiteId(task.site)).0,
                    AnalyzedStatement::Initialization { site, body, .. } => {
                        *site = remap(*site);
                        statements(body, remap);
                    }
                }
            }
        }
        fn regions(body: &mut [AnalyzedRegion], remap: &impl Fn(AnalogSiteId) -> AnalogSiteId) {
            for region in body {
                match region {
                    AnalyzedRegion::Assignment(assignment) => {
                        assignment.site = remap(assignment.site)
                    }
                    AnalyzedRegion::Contribution(contribution) => {
                        contribution.site = remap(contribution.site)
                    }
                    AnalyzedRegion::Conditional {
                        condition_site,
                        then_body,
                        else_body,
                        ..
                    } => {
                        *condition_site = condition_site.map(remap);
                        regions(then_body, remap);
                        regions(else_body, remap);
                    }
                    AnalyzedRegion::Loop { site, body, .. } => {
                        *site = remap(*site);
                        regions(body, remap);
                    }
                    AnalyzedRegion::Task(task) => task.site = remap(AnalogSiteId(task.site)).0,
                    AnalyzedRegion::Initialization { body, .. } => regions(body, remap),
                }
            }
        }
        for index in &mut module.prologue_statements {
            *index += resets.len();
        }
        module
            .body
            .splice(0..0, resets.iter().cloned().map(AnalyzedRegion::Assignment));
        module
            .statements
            .splice(0..0, resets.into_iter().map(AnalyzedStatement::Assignment));
        statements(&mut module.statements, &remap);
        regions(&mut module.body, &remap);
        for contribution in &mut module.contributions {
            contribution.site = remap(contribution.site);
        }
    }

    /// Interpret the CMC parameter storage convention without letting backend
    /// policy override what the Verilog-A source declares.
    fn parameter_scope(&mut self, parameter: &ParameterDecl) -> ParameterScope {
        let mut scope = ParameterScope::Model;
        let mut declared = false;
        for attribute in &parameter.attributes {
            if !attribute.name.eq_ignore_ascii_case("type") {
                continue;
            }
            let Some(Expression::StringLit(value)) = &attribute.value else {
                self.record_error_at(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "parameter '{}' attribute 'type' must be the string \"model\" or \
                         \"instance\"",
                        parameter.name
                    )),
                    attribute.span,
                );
                continue;
            };
            let parsed = if value.value.eq_ignore_ascii_case("model") {
                Some(ParameterScope::Model)
            } else if value.value.eq_ignore_ascii_case("instance") {
                Some(ParameterScope::Instance)
            } else {
                None
            };
            let Some(parsed) = parsed else {
                self.record_error_at(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "parameter '{}' has unsupported type attribute {:?}; expected \"model\" \
                         or \"instance\"",
                        parameter.name, value.value
                    )),
                    attribute.span,
                );
                continue;
            };
            if declared && scope != parsed {
                self.record_error_at(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "parameter '{}' has conflicting model/instance type attributes",
                        parameter.name
                    )),
                    attribute.span,
                );
                continue;
            }
            scope = parsed;
            declared = true;
        }
        scope
    }

    /// Interpret Xyce's CMC dual-scope extension. Xyce's ADMS templates gate
    /// this extension on attribute presence and intentionally ignore its value,
    /// so even a bare attribute or `xyceAlsoModel="no"` enables model storage.
    /// An explicit instance value remains independently given and takes
    /// precedence over the model-card fallback.
    fn parameter_also_model(&mut self, parameter: &ParameterDecl, scope: ParameterScope) -> bool {
        let present = parameter
            .attributes
            .iter()
            .any(|attribute| attribute.name.eq_ignore_ascii_case("xyceAlsoModel"));
        if present && scope != ParameterScope::Instance {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "parameter '{}' may use 'xyceAlsoModel' only with type=\"instance\"",
                    parameter.name
                )),
                parameter.span,
            );
            return false;
        }
        present
    }

    fn define_symbol(&mut self, symbol: Symbol) -> CompileResult<()> {
        if let Err(existing) = self.symbols.define(symbol.clone()) {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::DuplicateSymbol {
                    name: symbol.name,
                    first_defined: existing.span,
                },
                symbol.span,
            )));
        }
        Ok(())
    }

    /// Hard cap on array storage so a typo in a bound cannot silently
    /// allocate gigabytes of per-instance state
    const MAX_ARRAY_ELEMENTS: usize = 65_536;

    /// Register a 1-D array variable: its elements become contiguous
    /// `name[k]` slots in the variable storage (named after `storage_name`,
    /// which differs from the declared name for hoisted block locals).
    /// Bounds must fold to instance-invariant constants
    /// (parameter-dependent shapes would make the storage layout vary per
    /// instance).
    fn register_array_variable(
        &mut self,
        item: &VariableItem,
        var_type: VarType,
        storage_name: &SmolStr,
        analyzed: &mut AnalyzedModule,
    ) -> Option<AnalyzedArray> {
        if item.dimensions.len() != 1 {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "multi-dimensional array '{}' is not supported",
                    item.name
                )),
                item.span,
            );
            return None;
        }
        let dim = &item.dimensions[0];
        let (Some(start), Some(end)) = (
            self.eval_const_invariant(&dim.start),
            self.eval_const_invariant(&dim.end),
        ) else {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "array '{}' bounds must be compile-time constants",
                    item.name
                )),
                dim.span,
            );
            return None;
        };
        let (start, end) = (start.round() as i64, end.round() as i64);
        // The LRM writes ranges [lo:hi]; accept either order
        let (lower, upper) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };
        let len = (upper - lower + 1) as usize;
        if len > Self::MAX_ARRAY_ELEMENTS {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "array '{}' has {len} elements (limit {})",
                    item.name,
                    Self::MAX_ARRAY_ELEMENTS
                )),
                dim.span,
            );
            return None;
        }
        let value_type = match var_type {
            VarType::Real => ValueType::Real,
            VarType::Integer => ValueType::Integer,
            VarType::String => ValueType::String,
        };
        let base = analyzed.variables.len();
        for k in lower..=upper {
            analyzed.variables.push(AnalyzedVariable {
                name: SmolStr::from(format!("{storage_name}[{k}]")),
                var_type,
                value_type,
                is_state: false,
            });
        }
        Some(AnalyzedArray { base, lower, len })
    }

    /// Fold the active guard stack into a single condition expression
    fn current_guard(&self) -> Option<Expression> {
        let mut guards = self.guard_stack.iter();
        let first = guards.next()?.clone();
        Some(guards.fold(first, |acc, g| {
            let span = g.span();
            Expression::Binary(BinaryExpr {
                op: BinaryOp::And,
                left: Box::new(acc),
                right: Box::new(g.clone()),
                span,
            })
        }))
    }

    /// Mint the identity shared by one analog-block step's two recordings.
    ///
    /// Minted once per step and stamped on both copies, which is what makes the
    /// correspondence a fact of construction rather than a reconstruction. The
    /// counter is per module and is reset alongside `local_counter`.
    fn next_analog_site(&mut self) -> AnalogSiteId {
        let site = AnalogSiteId(self.next_analog_site);
        self.next_analog_site = self
            .next_analog_site
            .checked_add(1)
            .expect("analog site ordinal overflow");
        site
    }

    /// The shape [`Self::apply_guard`] will fold around an expression here.
    ///
    /// Asked *before* the fold, because after it an unguarded `a ? b : c` and a
    /// guarded one are indistinguishable.
    fn active_site_guard(&self) -> AnalogSiteGuard {
        match self.current_guard() {
            Some(_) => AnalogSiteGuard::Select,
            None => AnalogSiteGuard::None,
        }
    }

    /// The shape [`Self::fold_guard_into_condition`] will fold around a loop
    /// condition here.
    fn active_condition_guard(&self) -> AnalogSiteGuard {
        match self.current_guard() {
            Some(_) => AnalogSiteGuard::Conjunction,
            None => AnalogSiteGuard::None,
        }
    }

    /// Record a step in the innermost open region.
    ///
    /// Called at each point [`Self::apply_guard`] is about to fold a guard into
    /// an expression, with the expression as written.
    fn record_region(&mut self, region: AnalyzedRegion) {
        if let Some(frame) = self.region_stack.last_mut() {
            frame.push(region);
        }
    }

    /// Begin collecting a nested region.
    fn open_region(&mut self) {
        self.region_stack.push(Vec::new());
    }

    /// Finish the innermost region and return what it collected.
    fn close_region(&mut self) -> Vec<AnalyzedRegion> {
        // The bottom frame is the analog block itself and is closed by
        // `take_body`; popping it here would silently discard the module.
        debug_assert!(
            self.region_stack.len() > 1,
            "close_region beyond the analog block frame"
        );
        self.region_stack.pop().unwrap_or_default()
    }

    /// Take the module's structured body, leaving a fresh frame behind.
    fn take_body(&mut self) -> Vec<AnalyzedRegion> {
        debug_assert_eq!(
            self.region_stack.len(),
            1,
            "analog block finished with {} regions still open",
            self.region_stack.len().saturating_sub(1)
        );
        let body = self.region_stack.pop().unwrap_or_default();
        self.region_stack.push(Vec::new());
        body
    }

    /// Build `guard ? value : fallback` under the active guard (or `value`
    /// when unguarded)
    fn apply_guard(&self, value: Expression, fallback: Expression) -> Expression {
        match self.current_guard() {
            Some(guard) => {
                let span = value.span();
                Expression::Conditional(ConditionalExpr {
                    condition: Box::new(guard),
                    then_expr: Box::new(value),
                    else_expr: Box::new(fallback),
                    span,
                })
            }
            None => value,
        }
    }

    fn not_expr(expr: Expression) -> Expression {
        let span = expr.span();
        Expression::Unary(UnaryExpr {
            op: UnaryOp::Not,
            operand: Box::new(expr),
            span,
        })
    }

    fn binary_expr(op: BinaryOp, left: Expression, right: Expression) -> Expression {
        let span = left.span();
        Expression::Binary(BinaryExpr {
            op,
            left: Box::new(left),
            right: Box::new(right),
            span,
        })
    }

    fn number_expr(value: f64, span: Span) -> Expression {
        Expression::Number(NumberLit {
            value,
            raw: SmolStr::default(),
            span,
        })
    }

    fn value_type_for_var_type(var_type: VarType) -> ValueType {
        match var_type {
            VarType::Real => ValueType::Real,
            VarType::Integer => ValueType::Integer,
            VarType::String => ValueType::String,
        }
    }

    fn register_function_temp(
        &mut self,
        module: &mut AnalyzedModule,
        name: SmolStr,
        var_type: VarType,
        span: Span,
    ) -> CompileResult<()> {
        module.variables.push(AnalyzedVariable {
            name: name.clone(),
            var_type,
            value_type: Self::value_type_for_var_type(var_type),
            is_state: false,
        });
        self.define_symbol(Symbol {
            name,
            kind: SymbolKind::Variable,
            value_type: Self::value_type_for_var_type(var_type),
            span,
            attrs: Default::default(),
        })
    }

    /// Whether a call to this function *must* be lowered at a statement
    /// boundary. An output or inout argument writes a caller variable, which an
    /// expression cannot do.
    fn function_needs_materialization(&self, func: &FunctionDef) -> bool {
        self.effectful_functions.contains(&func.name)
            || func
                .params
                .iter()
                .any(|param| param.direction != ParamDirection::Input)
    }

    /// Whether a call to this function *should* be lowered at a statement
    /// boundary even when it need not be.
    ///
    /// A body with control flow should, because the alternative —
    /// [`Self::inline_function`] — dissolves that control flow into
    /// `guard ? value : previous` and duplicates the whole tree built so far at
    /// every assignment. Materialising instead runs the body through the
    /// ordinary statement path, where one `if` stays one conditional.
    ///
    /// Measured on `EPFL_HEMT_10a`, whose `core` nests three arms over five
    /// chained locals: 186,444 HIR expressions from an analog block holding 191
    /// assignments.
    fn function_should_materialize(&self, func: &FunctionDef) -> bool {
        if self.function_needs_materialization(func) {
            return true;
        }
        // A clamped exponential is recognised and replaced by an intrinsic
        // further down, which is both smaller and differentiable in closed
        // form. Hoisting it to a statement would happen first and take that
        // away, so its conditionals are left alone.
        if Self::is_recognized_limited_exp_function(func) {
            return false;
        }
        func.body.statements.iter().any(Self::statement_branches)
    }

    fn statement_branches(statement: &AnalogStatement) -> bool {
        match statement {
            AnalogStatement::Conditional(_)
            | AnalogStatement::Case(_)
            | AnalogStatement::While(_)
            | AnalogStatement::For(_)
            | AnalogStatement::Repeat(_) => true,
            AnalogStatement::Block(block) => block.statements.iter().any(Self::statement_branches),
            _ => false,
        }
    }

    fn is_recognized_limited_exp_function(func: &FunctionDef) -> bool {
        if !func.name.eq_ignore_ascii_case("lexp")
            || func.return_type != VarType::Real
            || func.params.len() != 1
            || func.params[0].direction != ParamDirection::Input
            || func.params[0].param_type != VarType::Real
        {
            return false;
        }

        let input = &func.params[0].name;
        let mut has_exp_return = false;
        let mut has_upper_linear_return = false;
        let mut has_lower_clamp_return = false;
        Self::collect_limited_exp_return_features(
            &func.body.statements,
            &func.name,
            input,
            &mut has_exp_return,
            &mut has_upper_linear_return,
            &mut has_lower_clamp_return,
        );
        has_exp_return && has_upper_linear_return && has_lower_clamp_return
    }

    fn collect_limited_exp_return_features(
        statements: &[AnalogStatement],
        return_name: &SmolStr,
        input_name: &SmolStr,
        has_exp_return: &mut bool,
        has_upper_linear_return: &mut bool,
        has_lower_clamp_return: &mut bool,
    ) {
        for statement in statements {
            match statement {
                AnalogStatement::Assignment(assignment) if matches!(&assignment.target, LValue::Variable { name, .. } if name == return_name) =>
                {
                    let value = &assignment.value;
                    if Self::expr_contains_call(value, "exp")
                        && Self::expr_contains_identifier(value, input_name)
                    {
                        *has_exp_return = true;
                    }
                    if Self::expr_contains_identifier(value, input_name)
                        && Self::expr_contains_number_close(value, 5.540622384e34)
                    {
                        *has_upper_linear_return = true;
                    }
                    if Self::expr_contains_number_close(value, 1.804851387e-35) {
                        *has_lower_clamp_return = true;
                    }
                }
                AnalogStatement::Conditional(conditional) => {
                    Self::collect_limited_exp_return_features(
                        std::slice::from_ref(&conditional.then_branch),
                        return_name,
                        input_name,
                        has_exp_return,
                        has_upper_linear_return,
                        has_lower_clamp_return,
                    );
                    if let Some(else_branch) = &conditional.else_branch {
                        Self::collect_limited_exp_return_features(
                            std::slice::from_ref(else_branch),
                            return_name,
                            input_name,
                            has_exp_return,
                            has_upper_linear_return,
                            has_lower_clamp_return,
                        );
                    }
                }
                AnalogStatement::Block(block) => Self::collect_limited_exp_return_features(
                    &block.statements,
                    return_name,
                    input_name,
                    has_exp_return,
                    has_upper_linear_return,
                    has_lower_clamp_return,
                ),
                AnalogStatement::Case(case) => {
                    for item in &case.items {
                        Self::collect_limited_exp_return_features(
                            std::slice::from_ref(&item.statement),
                            return_name,
                            input_name,
                            has_exp_return,
                            has_upper_linear_return,
                            has_lower_clamp_return,
                        );
                    }
                    if let Some(default) = &case.default {
                        Self::collect_limited_exp_return_features(
                            std::slice::from_ref(default),
                            return_name,
                            input_name,
                            has_exp_return,
                            has_upper_linear_return,
                            has_lower_clamp_return,
                        );
                    }
                }
                _ => {}
            }
        }
    }

    fn expr_contains_identifier(expr: &Expression, expected: &SmolStr) -> bool {
        match expr {
            Expression::Identifier(identifier) => &identifier.name == expected,
            Expression::Digital(digital) => {
                digital.base_name() == Some(expected)
                    || digital
                        .children()
                        .into_iter()
                        .any(|child| Self::expr_contains_identifier(child, expected))
            }
            Expression::Binary(binary) => {
                Self::expr_contains_identifier(&binary.left, expected)
                    || Self::expr_contains_identifier(&binary.right, expected)
            }
            Expression::Unary(unary) => Self::expr_contains_identifier(&unary.operand, expected),
            Expression::Conditional(conditional) => {
                Self::expr_contains_identifier(&conditional.condition, expected)
                    || Self::expr_contains_identifier(&conditional.then_expr, expected)
                    || Self::expr_contains_identifier(&conditional.else_expr, expected)
            }
            Expression::Call(call) => call
                .args
                .iter()
                .any(|arg| Self::expr_contains_identifier(arg, expected)),
            Expression::SystemFunction(function) => function
                .args
                .iter()
                .any(|arg| Self::expr_contains_identifier(arg, expected)),
            Expression::ArrayAccess(access) => {
                Self::expr_contains_identifier(&access.index, expected)
            }
            Expression::ArrayLiteral(array) => array
                .elements
                .iter()
                .any(|element| Self::array_element_contains_identifier(element, expected)),
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }

    fn array_element_contains_identifier(
        element: &ArrayLiteralElement,
        expected: &SmolStr,
    ) -> bool {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::expr_contains_identifier(expression, expected)
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::expr_contains_identifier(&replication.count, expected)
                    || replication
                        .elements
                        .iter()
                        .any(|element| Self::array_element_contains_identifier(element, expected))
            }
        }
    }

    fn expr_contains_call(expr: &Expression, expected: &str) -> bool {
        match expr {
            Expression::Digital(digital) => digital
                .children()
                .into_iter()
                .any(|child| Self::expr_contains_call(child, expected)),
            Expression::Call(call) => {
                call.name.eq_ignore_ascii_case(expected)
                    || call
                        .args
                        .iter()
                        .any(|arg| Self::expr_contains_call(arg, expected))
            }
            Expression::Binary(binary) => {
                Self::expr_contains_call(&binary.left, expected)
                    || Self::expr_contains_call(&binary.right, expected)
            }
            Expression::Unary(unary) => Self::expr_contains_call(&unary.operand, expected),
            Expression::Conditional(conditional) => {
                Self::expr_contains_call(&conditional.condition, expected)
                    || Self::expr_contains_call(&conditional.then_expr, expected)
                    || Self::expr_contains_call(&conditional.else_expr, expected)
            }
            Expression::SystemFunction(function) => function
                .args
                .iter()
                .any(|arg| Self::expr_contains_call(arg, expected)),
            Expression::ArrayAccess(access) => Self::expr_contains_call(&access.index, expected),
            Expression::ArrayLiteral(array) => array
                .elements
                .iter()
                .any(|element| Self::array_element_contains_call(element, expected)),
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }

    fn array_element_contains_call(element: &ArrayLiteralElement, expected: &str) -> bool {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::expr_contains_call(expression, expected)
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::expr_contains_call(&replication.count, expected)
                    || replication
                        .elements
                        .iter()
                        .any(|element| Self::array_element_contains_call(element, expected))
            }
        }
    }

    fn expr_contains_number_close(expr: &Expression, expected: f64) -> bool {
        match expr {
            Expression::Digital(digital) => digital
                .children()
                .into_iter()
                .any(|child| Self::expr_contains_number_close(child, expected)),
            Expression::Number(number) => {
                let tolerance = expected.abs().max(1.0) * 1.0e-12;
                (number.value - expected).abs() <= tolerance
            }
            Expression::Binary(binary) => {
                Self::expr_contains_number_close(&binary.left, expected)
                    || Self::expr_contains_number_close(&binary.right, expected)
            }
            Expression::Unary(unary) => Self::expr_contains_number_close(&unary.operand, expected),
            Expression::Conditional(conditional) => {
                Self::expr_contains_number_close(&conditional.condition, expected)
                    || Self::expr_contains_number_close(&conditional.then_expr, expected)
                    || Self::expr_contains_number_close(&conditional.else_expr, expected)
            }
            Expression::Call(call) => call
                .args
                .iter()
                .any(|arg| Self::expr_contains_number_close(arg, expected)),
            Expression::SystemFunction(function) => function
                .args
                .iter()
                .any(|arg| Self::expr_contains_number_close(arg, expected)),
            Expression::ArrayAccess(access) => {
                Self::expr_contains_number_close(&access.index, expected)
            }
            Expression::ArrayLiteral(array) => array
                .elements
                .iter()
                .any(|element| Self::array_element_contains_number_close(element, expected)),
            Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }

    fn array_element_contains_number_close(element: &ArrayLiteralElement, expected: f64) -> bool {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::expr_contains_number_close(expression, expected)
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::expr_contains_number_close(&replication.count, expected)
                    || replication
                        .elements
                        .iter()
                        .any(|element| Self::array_element_contains_number_close(element, expected))
            }
        }
    }

    /// Analyze a statement, lowering control flow into guarded dataflow.
    ///
    /// Assignments and contributions inside conditionals become conditional
    /// expressions (`guard ? value : previous`), so the recorded flat lists
    /// preserve branch semantics exactly. Loops whose bounds do not fold to
    /// compile-time constants lower to runtime loop statements.
    fn block_local_canonical_name_is_taken(&self, module: &AnalyzedModule, name: &SmolStr) -> bool {
        module
            .variables
            .iter()
            .any(|variable| variable.name == *name)
            || module
                .parameters
                .iter()
                .any(|parameter| parameter.name == *name)
            || module
                .param_aliases
                .iter()
                .any(|alias| alias.alias == *name)
            // A discrete-only signal still owns its canonical name. A local
            // must not acquire that identity merely because the signal needs
            // no analog input slot in this module.
            || module.digital.signals.iter().any(|signal| signal.name == *name)
            || module.ports.iter().any(|port| port.name == *name)
            || module.internal_nodes.iter().any(|node| node.name == *name)
            || module.branches.iter().any(|branch| branch.name == *name)
            || module.arrays.contains_key(name)
            || self.arrays.contains_key(name)
            || self.symbols.lookup(name).is_some()
    }

    fn analyze_statement(
        &mut self,
        stmt: &AnalogStatement,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        match stmt {
            AnalogStatement::Contribution(contrib) => {
                self.analyze_contribution(contrib, module, sink)?;
            }
            AnalogStatement::Assignment(assign) => {
                self.analyze_assignment(assign, module, sink)?;
            }
            AnalogStatement::Block(block) => {
                self.symbols.enter_scope();
                self.subst_stack.push(HashMap::new());

                // Hoist block-local variables to module scope under unique
                // names; expressions are rewritten through the subst frame.
                for var_decl in &block.variables {
                    let value_type = match var_decl.var_type {
                        VarType::Real => ValueType::Real,
                        VarType::Integer => ValueType::Integer,
                        VarType::String => ValueType::String,
                    };
                    for item in &var_decl.items {
                        self.local_counter += 1;
                        let hoisted = if self
                            .block_local_canonical_name_is_taken(module, &item.name)
                        {
                            loop {
                                let candidate: SmolStr =
                                    format!("{}__blk{}", item.name, self.local_counter).into();
                                if !self.block_local_canonical_name_is_taken(module, &candidate) {
                                    break candidate;
                                }
                                self.local_counter += 1;
                            }
                        } else {
                            item.name.clone()
                        };

                        if !item.dimensions.is_empty() {
                            if let Some(layout) = self.register_array_variable(
                                item,
                                var_decl.var_type,
                                &hoisted,
                                module,
                            ) {
                                module.arrays.insert(hoisted.clone(), layout.clone());
                                self.arrays.insert(hoisted.clone(), layout);
                                self.define_symbol(Symbol {
                                    name: hoisted.clone(),
                                    kind: SymbolKind::Variable,
                                    value_type,
                                    span: item.span,
                                    attrs: Default::default(),
                                })?;
                                self.subst_stack.last_mut().unwrap().insert(
                                    item.name.clone(),
                                    Expression::Identifier(Identifier {
                                        name: hoisted.clone(),
                                        span: item.span,
                                    }),
                                );
                                if item.init.is_some() {
                                    self.record_error_at(
                                        SemanticErrorKind::UnsupportedFeature(format!(
                                            "initializer on block-local array '{}'",
                                            item.name
                                        )),
                                        item.span,
                                    );
                                }
                            }
                            continue;
                        }

                        module.variables.push(AnalyzedVariable {
                            name: hoisted.clone(),
                            var_type: var_decl.var_type,
                            value_type,
                            is_state: false,
                        });
                        self.define_symbol(Symbol {
                            name: hoisted.clone(),
                            kind: SymbolKind::Variable,
                            value_type,
                            span: item.span,
                            attrs: Default::default(),
                        })?;
                        self.subst_stack.last_mut().unwrap().insert(
                            item.name.clone(),
                            Expression::Identifier(Identifier {
                                name: hoisted.clone(),
                                span: item.span,
                            }),
                        );

                        if let Some(init) = &item.init {
                            let written =
                                self.lower_expression_with_side_effects(init, module, sink)?;
                            let (written, _) =
                                self.coerce_assignment_expression(written, value_type)?;
                            let expression_guard = self.active_site_guard();
                            let expression = self
                                .apply_guard(written.clone(), Self::number_expr(0.0, item.span));
                            let expr_type = self.infer_type(&expression)?;
                            let var_index = module
                                .variables
                                .iter()
                                .position(|v| v.name == hoisted)
                                .expect("just registered");
                            // Recorded in both copies under one site: a named
                            // block's local initializer is a step of the block
                            // like any other, so the structured body runs it and
                            // the correspondence pairs it with the flat sink's
                            // copy rather than leaving the region tree short an
                            // assignment the executed list has.
                            let site = self.next_analog_site();
                            // Both copies carry the sink's own post-guard
                            // `expr_type` because one site is one assignment,
                            // and a second inference over the pre-guard
                            // expression would let the two halves of that pair
                            // disagree about its type.
                            self.record_region(AnalyzedRegion::Assignment(AnalyzedAssignment {
                                target: hoisted.clone(),
                                var_index,
                                index: None,
                                expression: written,
                                site,
                                expression_guard,
                                expr_type,
                                span: item.span,
                                unfiltered_initial_step_guard: None,
                            }));
                            sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
                                target: hoisted.clone(),
                                var_index,
                                index: None,
                                expression,
                                site,
                                expression_guard,
                                expr_type,
                                span: item.span,
                                unfiltered_initial_step_guard: None,
                            }));
                        }
                    }
                }

                for s in &block.statements {
                    self.analyze_statement(s, module, sink)?;
                }

                self.subst_stack.pop();
                self.symbols.exit_scope();
            }
            AnalogStatement::Conditional(cond) => {
                let condition =
                    self.lower_expression_with_side_effects(&cond.condition, module, sink)?;
                let dynamic_condition = !self.expression_is_simulation_invariant(&condition);
                let cond_type = self.infer_type(&condition)?;
                if !cond_type.is_condition() {
                    self.record_error_at(
                        SemanticErrorKind::InvalidCondition {
                            found: cond_type.to_string(),
                        },
                        cond.span,
                    );
                }

                // Snapshot the condition into a temporary BEFORE either
                // branch runs: the then-branch may assign variables the
                // condition reads, and a re-evaluated else-guard would then
                // see the mutated state and fire as well.
                //
                // The structured form needs no snapshot and must not use one.
                // Its condition is evaluated once, at the branch, and every read
                // inside an arm resolves against the definitions reaching that
                // arm — so the hazard the snapshot exists for cannot arise, and
                // the snapshot variable's own assignment only ever went into the
                // flat list.
                let unsnapshotted = condition.clone();
                let (condition, condition_site) =
                    self.snapshot_guard_with_site(condition, cond.span, module, sink)?;

                self.guard_stack.push(condition.clone());
                self.open_region();
                if dynamic_condition {
                    self.dynamic_analog_operator_guard_depth += 1;
                }
                let then_result = self.analyze_statement(&cond.then_branch, module, sink);
                if dynamic_condition {
                    self.dynamic_analog_operator_guard_depth -= 1;
                }
                then_result?;
                let then_body = self.close_region();
                self.guard_stack.pop();

                let mut else_body = Vec::new();
                if let Some(else_branch) = &cond.else_branch {
                    self.guard_stack.push(Self::not_expr(condition.clone()));
                    self.open_region();
                    if dynamic_condition {
                        self.dynamic_analog_operator_guard_depth += 1;
                    }
                    let else_result = self.analyze_statement(else_branch, module, sink);
                    if dynamic_condition {
                        self.dynamic_analog_operator_guard_depth -= 1;
                    }
                    else_result?;
                    else_body = self.close_region();
                    self.guard_stack.pop();
                }

                self.record_region(AnalyzedRegion::Conditional {
                    condition: unsnapshotted,
                    condition_site,
                    then_body,
                    else_body,
                    span: cond.span,
                });
            }
            AnalogStatement::Case(case_stmt) => {
                // The selector and ALL match comparisons are evaluated
                // before any arm executes (LRM case semantics); snapshot
                // them so arm bodies cannot perturb later guards.
                let selector =
                    self.lower_expression_with_side_effects(&case_stmt.expr, module, sink)?;
                let mut dynamic_case = !self.expression_is_simulation_invariant(&selector);
                // The structured form keeps the comparisons as written; a case
                // arm's condition is evaluated once at its branch, so the
                // snapshot the flat form needs would only be a variable the
                // region body never assigns.
                let unsnapshotted_selector = selector.clone();
                let selector = self.snapshot_guard(selector, case_stmt.span, module, sink)?;

                let mut item_guards: Vec<Option<Expression>> = Vec::new();
                let mut unsnapshotted_guards: Vec<Option<Expression>> = Vec::new();
                for item in &case_stmt.items {
                    let mut item_match: Option<Expression> = None;
                    let mut unsnapshotted_match: Option<Expression> = None;
                    for m in &item.matches {
                        let m_lowered = self.lower_expression_with_side_effects(m, module, sink)?;
                        dynamic_case |= !self.expression_is_simulation_invariant(&m_lowered);
                        let eq =
                            Self::binary_expr(BinaryOp::Eq, selector.clone(), m_lowered.clone());
                        item_match = Some(match item_match {
                            Some(acc) => Self::binary_expr(BinaryOp::Or, acc, eq),
                            None => eq,
                        });
                        let raw_eq = Self::binary_expr(
                            BinaryOp::Eq,
                            unsnapshotted_selector.clone(),
                            m_lowered,
                        );
                        unsnapshotted_match = Some(match unsnapshotted_match {
                            Some(acc) => Self::binary_expr(BinaryOp::Or, acc, raw_eq),
                            None => raw_eq,
                        });
                    }
                    let snapshotted = match item_match {
                        Some(expr) => {
                            Some(self.snapshot_guard(expr, case_stmt.span, module, sink)?)
                        }
                        None => None,
                    };
                    item_guards.push(snapshotted);
                    unsnapshotted_guards.push(unsnapshotted_match);
                }

                // OR of all guards matched so far (case items are priority
                // ordered: the first matching item wins)
                let mut prior_match: Option<Expression> = None;
                // Structured arms, each holding the item's own match condition
                // rather than the flat form's `match AND NOT prior`. Priority
                // becomes the nesting below, which is what lets the CFG emit
                // one branch per arm instead of an accumulating conjunction.
                let mut arms: Vec<(Expression, Vec<AnalyzedRegion>)> = Vec::new();

                for ((item, item_match), unsnapshotted_match) in case_stmt
                    .items
                    .iter()
                    .zip(item_guards)
                    .zip(unsnapshotted_guards)
                {
                    let (Some(item_match), Some(unsnapshotted_match)) =
                        (item_match, unsnapshotted_match)
                    else {
                        continue;
                    };

                    let guard = match &prior_match {
                        Some(prior) => Self::binary_expr(
                            BinaryOp::And,
                            item_match.clone(),
                            Self::not_expr(prior.clone()),
                        ),
                        None => item_match.clone(),
                    };

                    self.guard_stack.push(guard);
                    self.open_region();
                    if dynamic_case {
                        self.dynamic_analog_operator_guard_depth += 1;
                    }
                    let arm_result = self.analyze_statement(&item.statement, module, sink);
                    if dynamic_case {
                        self.dynamic_analog_operator_guard_depth -= 1;
                    }
                    arm_result?;
                    let body = self.close_region();
                    self.guard_stack.pop();
                    arms.push((unsnapshotted_match, body));

                    prior_match = Some(match prior_match {
                        Some(prior) => Self::binary_expr(BinaryOp::Or, prior, item_match),
                        None => item_match,
                    });
                }

                let mut chain: Vec<AnalyzedRegion> = Vec::new();
                if let Some(default) = &case_stmt.default {
                    match prior_match {
                        Some(prior) => {
                            self.guard_stack.push(Self::not_expr(prior));
                            self.open_region();
                            if dynamic_case {
                                self.dynamic_analog_operator_guard_depth += 1;
                            }
                            let default_result = self.analyze_statement(default, module, sink);
                            if dynamic_case {
                                self.dynamic_analog_operator_guard_depth -= 1;
                            }
                            default_result?;
                            chain = self.close_region();
                            self.guard_stack.pop();
                        }
                        None => {
                            self.open_region();
                            if dynamic_case {
                                self.dynamic_analog_operator_guard_depth += 1;
                            }
                            let default_result = self.analyze_statement(default, module, sink);
                            if dynamic_case {
                                self.dynamic_analog_operator_guard_depth -= 1;
                            }
                            default_result?;
                            chain = self.close_region();
                        }
                    }
                }

                // Fold innermost-first so arm order becomes else-nesting depth.
                // A case arm's structured condition is `raw_selector == match`
                // while its executed counterpart is `__guardN == match`: the two
                // are different expressions, not two copies of one, so no site
                // pairs them. Every state operator reachable from either lives
                // in the selector or a match expression, each of which is
                // snapshotted into its own paired statement — and
                // `HirExecutedCorrespondence` refuses by name if one ever ends
                // up outside a paired subtree.
                for (condition, then_body) in arms.into_iter().rev() {
                    chain = vec![AnalyzedRegion::Conditional {
                        condition,
                        condition_site: None,
                        then_body,
                        else_body: chain,
                        span: case_stmt.span,
                    }];
                }
                for region in chain {
                    self.record_region(region);
                }
            }
            AnalogStatement::For(for_stmt) => {
                self.dynamic_analog_operator_guard_depth += 1;
                let result = self.analyze_for(for_stmt, module, sink);
                self.dynamic_analog_operator_guard_depth -= 1;
                result?;
            }
            AnalogStatement::Repeat(repeat) => {
                self.dynamic_analog_operator_guard_depth += 1;
                let count_expr =
                    self.lower_expression_with_side_effects(&repeat.count, module, sink);
                let result = count_expr.and_then(|count_expr| {
                    match self.eval_const_invariant(&count_expr) {
                        Some(count) if (count as usize) <= Self::MAX_UNROLL_ITERATIONS => {
                            for _ in 0..(count as usize) {
                                self.analyze_statement(&repeat.body, module, sink)?;
                            }
                            Ok(())
                        }
                        Some(count) => Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(format!(
                                "repeat count {count} exceeds the unroll limit"
                            )),
                            repeat.span,
                        ))),
                        // Runtime-dependent count: lower to a runtime loop with
                        // a synthesized counter
                        None => self.lower_runtime_repeat(repeat, count_expr, module, sink),
                    }
                });
                self.dynamic_analog_operator_guard_depth -= 1;
                result?;
            }
            AnalogStatement::While(while_stmt) => {
                self.dynamic_analog_operator_guard_depth += 1;
                let condition =
                    self.lower_expression_with_side_effects(&while_stmt.condition, module, sink);
                let result =
                    condition.and_then(|condition| match self.eval_const_invariant(&condition) {
                        Some(0.0) => Ok(()), // statically dead loop
                        Some(_) => Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(
                                "while loop condition is constant-true (infinite loop)".into(),
                            ),
                            while_stmt.span,
                        ))),
                        None => {
                            // Runtime condition: lower to a runtime loop
                            let cond_type = self.infer_type(&condition)?;
                            if !cond_type.is_condition() {
                                self.record_error_at(
                                    SemanticErrorKind::InvalidCondition {
                                        found: cond_type.to_string(),
                                    },
                                    while_stmt.span,
                                );
                            }
                            // The structured loop sits inside whatever conditional
                            // region encloses it, so folding the guard in again
                            // would only add a read of a snapshot variable that the
                            // region body never assigns.
                            let unguarded = condition.clone();
                            let site = self.next_analog_site();
                            let condition_guard = self.active_condition_guard();
                            let condition = self.fold_guard_into_condition(condition);
                            let (body, regions) =
                                self.analyze_loop_body(&while_stmt.body, None, module)?;
                            self.record_region(AnalyzedRegion::Loop {
                                condition: unguarded,
                                site,
                                body: regions,
                                span: while_stmt.span,
                            });
                            sink.push(AnalyzedStatement::Loop(AnalyzedLoop {
                                condition,
                                site,
                                condition_guard,
                                body,
                                span: while_stmt.span,
                            }));
                            Ok(())
                        }
                    });
                self.dynamic_analog_operator_guard_depth -= 1;
                result?;
            }
            AnalogStatement::EventControl(event_ctrl) => {
                let unfiltered_initial_step = matches!(
                    &event_ctrl.event,
                    EventExpr::InitialStep { analyses, .. } if analyses.is_empty()
                );
                let EventLowering::Guard(guard) =
                    self.event_guard(&event_ctrl.event, module, sink)?;
                // The canonical HIR must retain the event predicate as
                // structured control flow. The snapshot below exists only for
                // the legacy flattened assignment stream; recording event-body
                // assignments directly in the enclosing region would make
                // generated backends execute every event unconditionally.
                let unsnapshotted = guard.clone();
                // Snapshot: the body must not perturb its own guard.
                let (guard, condition_site) =
                    self.snapshot_guard_with_site(guard, event_ctrl.span, module, sink)?;
                // The guard snapshot is an evaluation-local implementation
                // detail, not event-controlled procedural state. Start the
                // write set after it has been emitted so only the event body
                // participates in accepted/candidate transactions.
                let body_start = sink.len();
                let initial_guard_name = match &guard {
                    Expression::Identifier(identifier) if unfiltered_initial_step => {
                        Some(identifier.name.clone())
                    }
                    _ => None,
                };
                self.guard_stack.push(guard);
                self.open_region();
                if let Some(name) = initial_guard_name {
                    self.unfiltered_initial_step_guards.push(name);
                }
                self.dynamic_analog_operator_guard_depth += 1;
                let body_result = self.analyze_statement(&event_ctrl.statement, module, sink);
                self.dynamic_analog_operator_guard_depth -= 1;
                body_result?;
                let then_body = self.close_region();
                Self::record_event_state_variables(&sink[body_start..], module);
                if unfiltered_initial_step {
                    self.unfiltered_initial_step_guards.pop();
                }
                self.guard_stack.pop();
                self.record_region(AnalyzedRegion::Conditional {
                    condition: unsnapshotted,
                    condition_site,
                    then_body,
                    else_body: Vec::new(),
                    span: event_ctrl.span,
                });
            }
            AnalogStatement::IndirectContribution(stmt) => {
                self.analyze_indirect_contribution(stmt, module, sink)?;
            }
            // $bound_step and $discontinuity steer the transient stepper
            // through hidden per-evaluation variables; other system tasks
            // ($strobe, $display, ...) have no effect on the device
            // equations
            AnalogStatement::Call(call) => match call.name.as_str() {
                "$finish" => self.analyze_control_task(call, module, sink)?,
                "$bound_step" => {
                    self.validate_system_task_arity(call, 1, Some(1))?;
                    self.analyze_bound_step(call, module, sink)?;
                }
                "$discontinuity" => {
                    self.validate_system_task_arity(call, 0, Some(1))?;
                    self.analyze_discontinuity(call, module, sink)?;
                }
                name if Self::is_no_effect_system_task(name) => {
                    self.warn_no_effect_system_task(call);
                }
                _ => return Err(Self::unknown_system_task_error(call)),
            },
            AnalogStatement::Disable(_) | AnalogStatement::Null(_) => {}
        }
        Ok(())
    }

    const MAX_STATIC_UNROLL_ITERATIONS: usize = 32;

    fn analyze_control_task(
        &mut self,
        call: &CallStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        use crate::analog_tasks::{AnalogTaskCall, AnalogTaskKind, AnalogTaskOperand};
        self.validate_system_task_arity(call, 0, Some(1))?;
        let diagnostic = match call.args.first() {
            Some(argument) => self.lower_expression_with_side_effects(argument, module, sink)?,
            None => Self::number_expr(1.0, call.span),
        };
        let kind = self.infer_type(&diagnostic)?;
        if !kind.is_numeric() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::TypeMismatch {
                    expected: "numeric diagnostic level".into(),
                    found: kind.to_string(),
                    context: call.name.to_string(),
                },
                call.span,
            )));
        }
        let mut task = AnalogTaskCall {
            kind: AnalogTaskKind::Finish,
            site: self.next_analog_site().0,
            guard: None,
            arguments: vec![AnalogTaskOperand::Integer(diagnostic)],
            span: call.span,
            initialization: self.in_analog_initial,
        };
        self.record_region(AnalyzedRegion::Task(task.clone()));
        task.guard = self.current_guard();
        sink.push(AnalyzedStatement::Task(task));
        Ok(())
    }
    const MAX_UNROLL_ITERATIONS: usize = 65536;

    /// Analyze a for loop: statically unroll when the bounds fold to
    /// compile-time constants, otherwise lower to a runtime loop
    fn analyze_for(
        &mut self,
        for_stmt: &ForStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let loop_var = self.resolve_substituted_name(&for_stmt.var);
        if self.symbols.lookup(&loop_var).is_none() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndeclaredSymbol {
                    name: loop_var.clone(),
                },
                for_stmt.span,
            )));
        }

        if *for_stmt.update.target_name() != for_stmt.var {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(
                    "for-loop update must assign the loop variable".into(),
                ),
                for_stmt.span,
            )));
        }

        // Probe whether init, condition, and update fold to constants;
        // only then is static unrolling sound.
        let init = self.lower_expression_without_side_effects(&for_stmt.init, "for-loop init")?;
        let init_value = self.eval_const_invariant(&init);
        let static_unrollable = if let Some(value) = init_value {
            self.subst_stack.push(HashMap::from([(
                for_stmt.var.clone(),
                Self::number_expr(value, for_stmt.span),
            )]));
            let cond_probe = self
                .lower_expression_without_side_effects(&for_stmt.condition, "for-loop condition")
                .ok()
                .and_then(|c| self.eval_const_invariant(&c));
            let update_probe = self
                .lower_expression_without_side_effects(&for_stmt.update.value, "for-loop update")
                .ok()
                .and_then(|u| self.eval_const_invariant(&u));
            self.subst_stack.pop();
            cond_probe.is_some() && update_probe.is_some()
        } else {
            false
        };

        if static_unrollable {
            let init_value = init_value.expect("checked");
            let iteration_count = self.static_for_iteration_count(for_stmt, init_value)?;
            if iteration_count <= Self::MAX_STATIC_UNROLL_ITERATIONS
                || Self::statement_contains_contribution(&for_stmt.body)
            {
                self.unroll_for(for_stmt, init_value, module, sink)
            } else {
                self.lower_runtime_for(for_stmt, module, sink)
            }
        } else {
            self.lower_runtime_for(for_stmt, module, sink)
        }
    }

    fn static_for_iteration_count(
        &mut self,
        for_stmt: &ForStmt,
        init_value: f64,
    ) -> CompileResult<usize> {
        let mut value = init_value;
        let mut iterations = 0usize;
        loop {
            self.subst_stack.push(HashMap::from([(
                for_stmt.var.clone(),
                Self::number_expr(value, for_stmt.span),
            )]));

            let condition = match self
                .lower_expression_without_side_effects(&for_stmt.condition, "for-loop condition")
                .and_then(|expr| {
                    self.eval_const_invariant(&expr).ok_or_else(|| {
                        CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(
                                "for-loop condition stopped folding during unroll sizing".into(),
                            ),
                            for_stmt.span,
                        ))
                    })
                }) {
                Ok(condition) => condition,
                Err(error) => {
                    self.subst_stack.pop();
                    return Err(error);
                }
            };
            if condition == 0.0 {
                self.subst_stack.pop();
                return Ok(iterations);
            }

            let update = match self
                .lower_expression_without_side_effects(&for_stmt.update.value, "for-loop update")
                .and_then(|expr| {
                    self.eval_const_invariant(&expr).ok_or_else(|| {
                        CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(
                                "for-loop update stopped folding during unroll sizing".into(),
                            ),
                            for_stmt.span,
                        ))
                    })
                }) {
                Ok(update) => update,
                Err(error) => {
                    self.subst_stack.pop();
                    return Err(error);
                }
            };
            self.subst_stack.pop();

            iterations += 1;
            if iterations > Self::MAX_UNROLL_ITERATIONS {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(format!(
                        "for-loop exceeds the unroll limit of {} iterations",
                        Self::MAX_UNROLL_ITERATIONS
                    )),
                    for_stmt.span,
                )));
            }
            value = update;
        }
    }

    fn statement_contains_contribution(stmt: &AnalogStatement) -> bool {
        match stmt {
            AnalogStatement::Contribution(_) | AnalogStatement::IndirectContribution(_) => true,
            AnalogStatement::Block(block) => block
                .statements
                .iter()
                .any(Self::statement_contains_contribution),
            AnalogStatement::Conditional(cond) => {
                Self::statement_contains_contribution(&cond.then_branch)
                    || cond
                        .else_branch
                        .as_deref()
                        .is_some_and(Self::statement_contains_contribution)
            }
            AnalogStatement::Case(case_stmt) => {
                case_stmt
                    .items
                    .iter()
                    .any(|item| Self::statement_contains_contribution(&item.statement))
                    || case_stmt
                        .default
                        .as_deref()
                        .is_some_and(Self::statement_contains_contribution)
            }
            AnalogStatement::For(for_stmt) => Self::statement_contains_contribution(&for_stmt.body),
            AnalogStatement::Repeat(repeat) => Self::statement_contains_contribution(&repeat.body),
            AnalogStatement::While(while_stmt) => {
                Self::statement_contains_contribution(&while_stmt.body)
            }
            AnalogStatement::EventControl(event_ctrl) => {
                Self::statement_contains_contribution(&event_ctrl.statement)
            }
            AnalogStatement::Assignment(_)
            | AnalogStatement::Call(_)
            | AnalogStatement::Disable(_)
            | AnalogStatement::Null(_) => false,
        }
    }

    /// Statically unroll a for loop with compile-time-constant bounds
    fn unroll_for(
        &mut self,
        for_stmt: &ForStmt,
        init_value: f64,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let mut value = init_value;
        let mut iterations = 0usize;
        loop {
            // Bind the loop variable to its current constant value
            self.subst_stack.push(HashMap::from([(
                for_stmt.var.clone(),
                Self::number_expr(value, for_stmt.span),
            )]));

            let condition = self
                .lower_expression_without_side_effects(&for_stmt.condition, "for-loop condition")?;
            let Some(cond_value) = self.eval_const_invariant(&condition) else {
                self.subst_stack.pop();
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(
                        "for-loop condition stopped folding during unrolling".into(),
                    ),
                    for_stmt.span,
                )));
            };
            if cond_value == 0.0 {
                self.subst_stack.pop();
                break;
            }

            self.analyze_statement(&for_stmt.body, module, sink)?;

            let update = self
                .lower_expression_without_side_effects(&for_stmt.update.value, "for-loop update")?;
            let Some(next_value) = self.eval_const_invariant(&update) else {
                self.subst_stack.pop();
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(
                        "for-loop update stopped folding during unrolling".into(),
                    ),
                    for_stmt.span,
                )));
            };
            self.subst_stack.pop();

            value = next_value;
            iterations += 1;
            if iterations > Self::MAX_UNROLL_ITERATIONS {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(format!(
                        "for-loop exceeds the unroll limit of {} iterations",
                        Self::MAX_UNROLL_ITERATIONS
                    )),
                    for_stmt.span,
                )));
            }
        }

        Ok(())
    }

    /// Materialize a guard expression into a synthesized variable assigned
    /// once at this point in the statement stream.
    ///
    /// Guards must capture the state at evaluation time: branch bodies may
    /// assign variables their own guard reads, and re-evaluating the raw
    /// expression inside each guarded assignment would observe the
    /// mutation (e.g. `if (x == UNSET) x = a; else x = x + b;` must never
    /// run both arms). Trivial guards (literals, identifiers) pass through.
    fn snapshot_guard(
        &mut self,
        condition: Expression,
        span: Span,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Expression> {
        Ok(self
            .snapshot_guard_with_site(condition, span, module, sink)?
            .0)
    }

    /// [`Self::snapshot_guard`], also reporting the site the snapshot statement
    /// was stamped with.
    ///
    /// `None` means the condition passed through unsnapshotted, which happens
    /// for exactly an identifier or a numeric literal — neither of which owns a
    /// state record, so an unpaired condition is a leaf rather than a hole.
    fn snapshot_guard_with_site(
        &mut self,
        condition: Expression,
        span: Span,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<(Expression, Option<AnalogSiteId>)> {
        // A variable can be changed by the selected arm. Only literals are
        // intrinsically stable across execution of both flattened guards.
        if matches!(condition, Expression::Number(_)) {
            return Ok((condition, None));
        }

        self.local_counter += 1;
        let name: SmolStr = format!("__guard{}", self.local_counter).into();

        let var_index = module.variables.len();
        module.variables.push(AnalyzedVariable {
            name: name.clone(),
            var_type: VarType::Real,
            value_type: ValueType::Real,
            is_state: false,
        });
        self.define_symbol(Symbol {
            name: name.clone(),
            kind: SymbolKind::Variable,
            value_type: ValueType::Real,
            span,
            attrs: Default::default(),
        })?;

        // The snapshot assignment itself is unconditional: guard
        // expressions are pure, and enclosing guards already gate every
        // consumer of this variable.
        let site = self.next_analog_site();
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: name.clone(),
            var_index,
            index: None,
            expression: condition,
            site,
            expression_guard: AnalogSiteGuard::None,
            expr_type: ValueType::Real,
            span,
            unfiltered_initial_step_guard: None,
        }));

        Ok((
            Expression::Identifier(Identifier { name, span }),
            Some(site),
        ))
    }

    /// Add every variable slot that an event-controlled statement can write.
    ///
    /// Runtime-indexed array writes conservatively cover the array's complete
    /// contiguous storage because the selected element is not known until an
    /// evaluation runs. Loop bodies are recursive assignment streams and are
    /// walked here rather than relying on source-level control-flow shape.
    fn record_event_state_variables(statements: &[AnalyzedStatement], module: &mut AnalyzedModule) {
        fn collect(
            statements: &[AnalyzedStatement],
            module: &AnalyzedModule,
            slots: &mut Vec<usize>,
        ) {
            for statement in statements {
                match statement {
                    AnalyzedStatement::Task(_) => {}
                    AnalyzedStatement::Assignment(assignment) => {
                        if assignment.index.is_some() {
                            if let Some(array) = module.arrays.get(&assignment.target) {
                                slots.extend(array.base..array.base.saturating_add(array.len));
                            } else {
                                // Semantic lowering creates indexed writes
                                // only for registered arrays. If that internal
                                // invariant is ever broken, treating every
                                // slot as transactional is the conservative
                                // behavior and cannot leak speculative state.
                                slots.extend(0..module.variables.len());
                            }
                        } else {
                            slots.push(assignment.var_index);
                        }
                    }
                    AnalyzedStatement::Loop(loop_) => collect(&loop_.body, module, slots),
                    AnalyzedStatement::Initialization { body, .. } => collect(body, module, slots),
                }
            }
        }

        let mut slots = std::mem::take(&mut module.event_state_variables);
        collect(statements, module, &mut slots);
        slots.sort_unstable();
        slots.dedup();
        for &slot in &slots {
            if let Some(variable) = module.variables.get_mut(slot) {
                variable.is_state = true;
            }
        }
        module.event_state_variables = slots;
    }

    fn record_initial_state_variables(
        statements: &[AnalyzedStatement],
        module: &mut AnalyzedModule,
        module_variable_count: usize,
    ) {
        let previous = module.event_state_variables.clone();
        Self::record_event_state_variables(statements, module);
        module
            .event_state_variables
            .retain(|&slot| slot < module_variable_count || previous.binary_search(&slot).is_ok());
        for (slot, variable) in module
            .variables
            .iter_mut()
            .enumerate()
            .skip(module_variable_count)
        {
            variable.is_state = previous.binary_search(&slot).is_ok();
        }
    }

    /// AND the enclosing guard into a runtime loop condition so a guarded
    /// loop runs zero iterations when its guard is inactive
    fn fold_guard_into_condition(&self, condition: Expression) -> Expression {
        match self.current_guard() {
            Some(guard) => Self::binary_expr(BinaryOp::And, guard, condition),
            None => condition,
        }
    }

    /// Analyze loop-body statements into a fresh sink, tracking the
    /// runtime-loop nesting depth (contributions inside are rejected).
    /// An optional trailing statement (the for-loop update) is analyzed
    /// after the body.
    /// Analyze a runtime loop body, returning both forms of it.
    ///
    /// The single funnel for every runtime-bounded loop, which is why the
    /// structured capture lives here rather than at the three record sites.
    fn analyze_loop_body(
        &mut self,
        body: &AnalogStatement,
        trailing: Option<&AnalogStatement>,
        module: &mut AnalyzedModule,
    ) -> CompileResult<(Vec<AnalyzedStatement>, Vec<AnalyzedRegion>)> {
        let mut statements = Vec::new();
        self.runtime_loop_depth += 1;
        self.open_region();
        let result = self
            .analyze_statement(body, module, &mut statements)
            .and_then(|()| match trailing {
                Some(stmt) => self.analyze_statement(stmt, module, &mut statements),
                None => Ok(()),
            });
        let regions = self.close_region();
        self.runtime_loop_depth -= 1;
        result?;
        Ok((statements, regions))
    }

    /// Lower a for loop with runtime-dependent bounds (e.g. iterating to a
    /// parameter like BSIM4's nf finger count) into a runtime loop
    fn lower_runtime_for(
        &mut self,
        for_stmt: &ForStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        // Loop variable initialization through the normal guarded path
        let init_stmt = AssignmentStmt {
            target: LValue::Variable {
                name: for_stmt.var.clone(),
                span: for_stmt.span,
            },
            value: for_stmt.init.clone(),
            span: for_stmt.span,
        };
        self.analyze_assignment(&init_stmt, module, sink)?;

        // Condition re-evaluated each iteration, with the enclosing guard
        // folded in
        let condition =
            self.lower_expression_without_side_effects(&for_stmt.condition, "for-loop condition")?;
        let cond_type = self.infer_type(&condition)?;
        if !cond_type.is_condition() {
            self.record_error_at(
                SemanticErrorKind::InvalidCondition {
                    found: cond_type.to_string(),
                },
                for_stmt.span,
            );
        }
        let unguarded = condition.clone();
        let site = self.next_analog_site();
        let condition_guard = self.active_condition_guard();
        let condition = self.fold_guard_into_condition(condition);

        // Body, then the update assignment, inside the loop sink
        let update_stmt = AnalogStatement::Assignment((*for_stmt.update).clone());
        let (body, regions) = self.analyze_loop_body(&for_stmt.body, Some(&update_stmt), module)?;

        self.record_region(AnalyzedRegion::Loop {
            condition: unguarded,
            site,
            body: regions,
            span: for_stmt.span,
        });
        sink.push(AnalyzedStatement::Loop(AnalyzedLoop {
            condition,
            site,
            condition_guard,
            body,
            span: for_stmt.span,
        }));
        Ok(())
    }

    /// Lower a repeat loop with a runtime-dependent count into a runtime
    /// loop over a synthesized counter
    fn lower_runtime_repeat(
        &mut self,
        repeat: &RepeatStmt,
        count_expr: Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let span = repeat.span;

        // Synthesize counter and count-snapshot variables (the LRM
        // evaluates the repeat count once, before iterating)
        self.local_counter += 1;
        let idx_name: SmolStr = format!("__repeat_i{}", self.local_counter).into();
        let cnt_name: SmolStr = format!("__repeat_n{}", self.local_counter).into();
        let mut register = |this: &mut Self, name: &SmolStr| -> CompileResult<usize> {
            let var_index = module.variables.len();
            module.variables.push(AnalyzedVariable {
                name: name.clone(),
                var_type: VarType::Real,
                value_type: ValueType::Real,
                is_state: false,
            });
            this.define_symbol(Symbol {
                name: name.clone(),
                kind: SymbolKind::Variable,
                value_type: ValueType::Real,
                span,
                attrs: Default::default(),
            })?;
            Ok(var_index)
        };
        let idx_index = register(self, &idx_name)?;
        let cnt_index = register(self, &cnt_name)?;

        let ident = |name: &SmolStr| {
            Expression::Identifier(Identifier {
                name: name.clone(),
                span,
            })
        };

        // cnt = <count>; idx = 0;
        let count_site = self.next_analog_site();
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: cnt_name.clone(),
            var_index: cnt_index,
            index: None,
            expression: count_expr,
            site: count_site,
            expression_guard: AnalogSiteGuard::None,
            expr_type: ValueType::Real,
            span,
            unfiltered_initial_step_guard: None,
        }));
        let index_site = self.next_analog_site();
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: idx_name.clone(),
            var_index: idx_index,
            index: None,
            expression: Self::number_expr(0.0, span),
            site: index_site,
            expression_guard: AnalogSiteGuard::None,
            expr_type: ValueType::Real,
            span,
            unfiltered_initial_step_guard: None,
        }));

        // while (guard && idx < cnt) { body; idx = idx + 1; }
        let unguarded = Self::binary_expr(BinaryOp::Lt, ident(&idx_name), ident(&cnt_name));
        let loop_site = self.next_analog_site();
        let condition_guard = self.active_condition_guard();
        let condition = self.fold_guard_into_condition(unguarded.clone());

        let (mut body, mut regions) = self.analyze_loop_body(&repeat.body, None, module)?;
        // The synthesized counter bump closes the loop in both forms; a
        // structured body without it would describe a loop that never advances.
        // One struct, cloned: the site travels with it, so the two copies of
        // the counter bump are paired without a second stamp.
        let increment = AnalyzedAssignment {
            target: idx_name.clone(),
            var_index: idx_index,
            index: None,
            expression: Self::binary_expr(
                BinaryOp::Add,
                ident(&idx_name),
                Self::number_expr(1.0, span),
            ),
            site: self.next_analog_site(),
            expression_guard: AnalogSiteGuard::None,
            expr_type: ValueType::Real,
            span,
            unfiltered_initial_step_guard: None,
        };
        regions.push(AnalyzedRegion::Assignment(increment.clone()));
        body.push(AnalyzedStatement::Assignment(increment));

        self.record_region(AnalyzedRegion::Loop {
            condition: unguarded,
            site: loop_site,
            body: regions,
            span,
        });
        sink.push(AnalyzedStatement::Loop(AnalyzedLoop {
            condition,
            site: loop_site,
            condition_guard,
            body,
            span,
        }));
        Ok(())
    }

    /// Lower an event expression into a runtime guard
    fn event_guard(
        &mut self,
        event: &EventExpr,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<EventLowering> {
        Ok(match event {
            EventExpr::InitialStep { analyses, span } => {
                let phase = Expression::Call(CallExpr {
                    name: "analysis".into(),
                    args: vec![Expression::StringLit(StringLit {
                        value: "__rspice_initial_step".into(),
                        span: *span,
                    })],
                    span: *span,
                });
                EventLowering::Guard(Self::filter_step_event(phase, analyses, *span))
            }
            EventExpr::FinalStep { analyses, span } => {
                let phase = Expression::Call(CallExpr {
                    name: "analysis".into(),
                    args: vec![Expression::StringLit(StringLit {
                        value: "__rspice_final_step".into(),
                        span: *span,
                    })],
                    span: *span,
                });
                EventLowering::Guard(Self::filter_step_event(phase, analyses, *span))
            }
            EventExpr::Cross {
                signal,
                direction,
                time_tol,
                expr_tol,
                enable,
                span,
            } => {
                let signal = self.lower_expression_with_side_effects(signal, module, sink)?;
                let direction = direction
                    .as_deref()
                    .map(|direction| {
                        self.lower_expression_with_side_effects(direction, module, sink)
                    })
                    .transpose()?
                    .unwrap_or_else(|| Self::number_expr(0.0, *span));
                let mut args = vec![signal, direction];
                if let Some(time_tol) = time_tol {
                    args.push(self.lower_expression_with_side_effects(time_tol, module, sink)?);
                } else if expr_tol.is_some() || enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(expr_tol) = expr_tol {
                    args.push(self.lower_expression_with_side_effects(expr_tol, module, sink)?);
                } else if enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(enable) = enable {
                    args.push(self.lower_expression_with_side_effects(enable, module, sink)?);
                }
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "cross".into(),
                    args,
                    span: *span,
                }))
            }
            EventExpr::Posedge { signal, span } => {
                self.qualify_edge_event_operand("posedge", signal, *span)?;
                let signal = self.lower_expression_with_side_effects(signal, module, sink)?;
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "cross".into(),
                    args: vec![signal, Self::number_expr(1.0, *span)],
                    span: *span,
                }))
            }
            EventExpr::Negedge { signal, span } => {
                self.qualify_edge_event_operand("negedge", signal, *span)?;
                let signal = self.lower_expression_with_side_effects(signal, module, sink)?;
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "cross".into(),
                    args: vec![signal, Self::number_expr(-1.0, *span)],
                    span: *span,
                }))
            }
            EventExpr::Above {
                signal,
                time_tol,
                expr_tol,
                enable,
                span,
            } => {
                let signal = self.lower_expression_with_side_effects(signal, module, sink)?;
                let mut args = vec![signal];
                if let Some(time_tol) = time_tol {
                    args.push(self.lower_expression_with_side_effects(time_tol, module, sink)?);
                } else if expr_tol.is_some() || enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(expr_tol) = expr_tol {
                    args.push(self.lower_expression_with_side_effects(expr_tol, module, sink)?);
                } else if enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(enable) = enable {
                    args.push(self.lower_expression_with_side_effects(enable, module, sink)?);
                }
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "above".into(),
                    args,
                    span: *span,
                }))
            }
            EventExpr::Timer {
                start,
                period,
                time_tol,
                enable,
                span,
            } => {
                let mut args = vec![self.lower_expression_with_side_effects(start, module, sink)?];
                if let Some(period) = period {
                    args.push(self.lower_expression_with_side_effects(period, module, sink)?);
                } else if time_tol.is_some() || enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(time_tol) = time_tol {
                    args.push(self.lower_expression_with_side_effects(time_tol, module, sink)?);
                } else if enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(enable) = enable {
                    args.push(self.lower_expression_with_side_effects(enable, module, sink)?);
                }
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "timer".into(),
                    args,
                    span: *span,
                }))
            }
            EventExpr::Or { left, right, .. } => {
                let left = self.event_guard(left, module, sink)?;
                let right = self.event_guard(right, module, sink)?;
                match (left, right) {
                    (EventLowering::Guard(l), EventLowering::Guard(r)) => {
                        EventLowering::Guard(Self::binary_expr(BinaryOp::Or, l, r))
                    }
                }
            }
        })
    }

    /// Decide what a `posedge`/`negedge` operand means before it is lowered.
    ///
    /// In a *Verilog-A* module there is no discrete domain, so an edge event
    /// can only be the analog `cross` operator — a continuous zero-crossing
    /// detector on the *value* of the operand. That approximation is the
    /// conventional Verilog-A reading of `@(posedge expr)` and is kept, but it
    /// is not silent: an accepted edge event records why the source now means
    /// something else.
    ///
    /// On a discrete-discipline signal it is simply wrong, and the wrongness
    /// is not a matter of degree. Verilog-AMS LRM 2.4 section 7.3.4 makes
    /// `@(posedge clk1 or cross(V(clk2), 1))` inside an `analog` block legal —
    /// its `analog_event_expression` production lists `posedge expression`
    /// beside the analog event functions, and its worked example declares
    /// `clk1` a `wire` — and section 7.3.6.2 fixes when the guarded statement
    /// runs: "at the time corresponding to a real promotion of the digital
    /// time". So the construct means something specific, and `cross` on a
    /// four-state net's absent continuous value is not it: `cross` would stamp
    /// a detector on a quantity the solver never integrates and the event would
    /// never fire at all.
    ///
    /// What this compiler is short of is the seam rather than the semantics.
    /// The engine already breakpoints a digital event's tick and already runs
    /// the discrete half at the accepted timepoint that lands on it, so the
    /// instant section 7.3.6.2 names exists and is reached; what does not exist
    /// is a way for the compiled analog body to be told an edge happened at it.
    /// The refusal names that rather than the construct.
    fn qualify_edge_event_operand(
        &mut self,
        keyword: &str,
        signal: &Expression,
        span: Span,
    ) -> CompileResult<()> {
        if let Some(net) = self.discrete_net_operand(signal) {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "`{keyword} {net}` in an analog event control is a digital edge event on a \
                     discrete-discipline signal; Verilog-AMS LRM 2.4 section 7.3.4 allows it and \
                     section 7.3.6.2 runs the guarded statement at the real promotion of the \
                     digital time, but the compiled analog body has no route to the digital \
                     event yet, and the analog `cross` operator is not a substitute — a discrete \
                     net has no continuous value to cross"
                )),
                span,
            )));
        }
        self.warn(
            EDGE_EVENT_AS_CROSS_CODE,
            format!(
                "`{keyword}` is compiled as `cross(...)` for Verilog-A compatibility: it fires on \
                 a continuous zero crossing of the operand, not on a digital signal edge."
            ),
            span,
        );
        Ok(())
    }

    /// The name of the operand when it is a net or port of a discrete-domain
    /// discipline (the built-in `logic`, or any user discipline declared
    /// `domain discrete`).
    fn discrete_net_operand(&self, signal: &Expression) -> Option<SmolStr> {
        let Expression::Identifier(identifier) = signal else {
            return None;
        };
        let symbol = self.symbols.lookup(identifier.name.as_str())?;
        if !matches!(symbol.kind, SymbolKind::Node | SymbolKind::Port) {
            return None;
        }
        let discipline = self
            .disciplines
            .get_discipline(symbol.attrs.discipline.as_deref()?)?;
        (discipline.domain == Domain::Discrete).then(|| identifier.name.clone())
    }

    fn filter_step_event(phase: Expression, analyses: &[StringLit], span: Span) -> Expression {
        if analyses.is_empty() {
            return phase;
        }

        let analysis_filter = Expression::Call(CallExpr {
            name: "analysis".into(),
            args: analyses
                .iter()
                .cloned()
                .map(|mut analysis| {
                    analysis.value = format!("__rspice_scope_{}", analysis.value).into();
                    analysis
                })
                .map(Expression::StringLit)
                .collect(),
            span,
        });
        Self::binary_expr(BinaryOp::And, phase, analysis_filter)
    }

    fn analyze_contribution(
        &mut self,
        contrib: &ContributionStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        // Contributions accumulate into fixed stamp programs; a contribution
        // executed a runtime-dependent number of times is not representable
        if self.runtime_loop_depth > 0 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(
                    "contributions inside loops require compile-time-constant bounds".into(),
                ),
                contrib.span,
            )));
        }

        let (branch_name, is_current, declared_branch) =
            self.resolve_contribution_target(&contrib.target, module, contrib.span)?;

        self.validate_direct_zi_contribution(&contrib.value, contrib.span)?;

        let expression = self.lower_expression_with_side_effects(&contrib.value, module, sink)?;
        let expr_type = self.infer_type(&expression)?;
        if !expr_type.is_numeric() && expr_type != ValueType::Unknown {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: "numeric".to_string(),
                    found: expr_type.to_string(),
                    context: "contribution expression".to_string(),
                },
                contrib.span,
            );
        }

        let site = self.next_analog_site();
        let expression_guard = self.active_site_guard();
        self.record_region(AnalyzedRegion::Contribution(AnalyzedContribution {
            branch: branch_name.clone(),
            declared_branch: declared_branch.clone(),
            is_current,
            indirect: false,
            equation_abstol: None,
            expression: expression.clone(),
            site,
            expression_guard,
            expr_type,
            span: contrib.span,
        }));
        // A guarded contribution contributes zero when inactive
        let expression = self.apply_guard(expression, Self::number_expr(0.0, contrib.span));

        module.contributions.push(AnalyzedContribution {
            branch: branch_name,
            declared_branch,
            is_current,
            indirect: false,
            equation_abstol: None,
            expression,
            site,
            expression_guard,
            expr_type,
            span: contrib.span,
        });

        Ok(())
    }

    fn validate_direct_zi_contribution(
        &self,
        expression: &Expression,
        span: Span,
    ) -> CompileResult<()> {
        enum Pending<'a> {
            Expression(&'a Expression),
            Element(&'a ArrayLiteralElement),
        }
        let mut pending = Vec::new();
        let mut next = Some(Pending::Expression(expression));
        while let Some(node) = next {
            match node {
                Pending::Expression(expression) => match expression {
                    Expression::Digital(digital) => {
                        pending.extend(
                            digital
                                .children()
                                .into_iter()
                                .rev()
                                .map(Pending::Expression),
                        );
                    }
                    Expression::Call(call) => {
                        if is_zi_operator_name(&call.name) {
                            self.validate_direct_zi_site(
                                call.name.as_str(),
                                call.args.get(4),
                                span,
                            )?;
                        }
                        pending.extend(call.args.iter().rev().map(Pending::Expression));
                    }
                    Expression::AnalogOperator(AnalogOperator::Limit {
                        proposed,
                        candidate,
                        type_metadata,
                        ..
                    }) => {
                        if let Some(value) = type_metadata {
                            pending.push(Pending::Expression(value));
                        }
                        pending.push(Pending::Expression(candidate));
                        pending.push(Pending::Expression(proposed));
                    }
                    Expression::Binary(binary) => {
                        pending.push(Pending::Expression(&binary.right));
                        pending.push(Pending::Expression(&binary.left));
                    }
                    Expression::Unary(unary) => {
                        pending.push(Pending::Expression(&unary.operand));
                    }
                    Expression::Conditional(conditional) => {
                        pending.push(Pending::Expression(&conditional.else_expr));
                        pending.push(Pending::Expression(&conditional.then_expr));
                        pending.push(Pending::Expression(&conditional.condition));
                    }
                    Expression::SystemFunction(function) => {
                        pending.extend(function.args.iter().rev().map(Pending::Expression));
                    }
                    Expression::ArrayAccess(access) => {
                        pending.push(Pending::Expression(&access.index));
                    }
                    Expression::ArrayLiteral(array) => {
                        pending.extend(array.elements.iter().rev().map(Pending::Element));
                    }
                    Expression::Number(_)
                    | Expression::StringLit(_)
                    | Expression::NullArgument(_)
                    | Expression::Identifier(_)
                    | Expression::BranchAccess(_)
                    | Expression::AnalogOperator(AnalogOperator::LimiterArgument { .. }) => {}
                    Expression::NoiseSource(noise) => match noise {
                        NoiseSource::White { power, .. } => {
                            pending.push(Pending::Expression(power));
                        }
                        NoiseSource::Flicker {
                            power, exponent, ..
                        } => {
                            pending.push(Pending::Expression(exponent));
                            pending.push(Pending::Expression(power));
                        }
                        NoiseSource::Table { data, .. } => {
                            pending.extend(data.iter().rev().map(Pending::Expression));
                        }
                    },
                },
                Pending::Element(ArrayLiteralElement::Value(expression)) => {
                    pending.push(Pending::Expression(expression));
                }
                Pending::Element(ArrayLiteralElement::Replication(replication)) => {
                    pending.extend(replication.elements.iter().rev().map(Pending::Element));
                    pending.push(Pending::Expression(&replication.count));
                }
            }
            next = pending.pop();
        }
        Ok(())
    }

    fn validate_direct_zi_site(
        &self,
        name: &str,
        transition: Option<&Expression>,
        contribution_span: Span,
    ) -> CompileResult<()> {
        let statically_invalid = match transition {
            None => (self.current_default_transition == 0.0)
                .then_some("the effective `default_transition` is zero".to_string()),
            Some(Expression::NullArgument(_)) => {
                Some("the transition argument is null".to_string())
            }
            Some(value) => self.eval_const_invariant(value).and_then(|value| {
                (!value.is_finite() || value <= 0.0)
                    .then_some(format!("the transition expression evaluates to {value}"))
            }),
        };
        if let Some(reason) = statically_invalid {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "{name} cannot be contributed directly to an analog branch because {reason}; its transition time must evaluate strictly positive, or the Zi result must first be assigned to an intermediate variable (VAMS-2023 section 4.5.12)"
                )),
                contribution_span,
            )));
        }
        Ok(())
    }

    /// Analyze an indirect contribution `V(x): lhs == rhs`: the target
    /// branch carries an unknown source whose value the solver picks so
    /// the constraint holds. The recorded expression is the residual
    /// `lhs - rhs`; an inactive guard opens the branch by letting structural
    /// stamping pin its unused current unknown to zero.
    fn analyze_indirect_contribution(
        &mut self,
        stmt: &IndirectContributionStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        if self.dynamic_analog_operator_guard_depth != 0 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(
                    "indirect contribution controls must remain constant during an analysis".into(),
                ),
                stmt.span,
            )));
        }
        if self.runtime_loop_depth > 0 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(
                    "contributions inside loops require compile-time-constant bounds".into(),
                ),
                stmt.span,
            )));
        }

        let (branch_name, is_current, declared_branch) =
            self.resolve_contribution_target(&stmt.branch, module, stmt.span)?;

        let mut lhs_source = stmt.lhs.clone();
        if let Expression::Call(call) = &mut lhs_source
            && call.name == "ddt"
            && let Some(Expression::Identifier(identifier)) = call.args.get_mut(1)
            && self.symbols.lookup(&identifier.name).is_none()
            && let Some(nature) = self.disciplines.get_nature(&identifier.name)
        {
            call.args[1] = Self::number_expr(nature.abstol, identifier.span);
        }
        // The optional derivative tolerance belongs to the equation, while
        // ordinary numerical ddt lowering accepts its signal operand only.
        let explicit_ddt_abstol = if let Expression::Call(call) = &mut lhs_source
            && call.name == "ddt"
            && call.args.len() == 2
        {
            call.args.pop()
        } else {
            None
        };
        // Resolve the physical access before implicit integrators introduce
        // electrical state nodes, which no longer describe the authored units.
        let lhs = if let Expression::Call(call) = &lhs_source
            && matches!(call.name.as_str(), "ddt" | "idt" | "idtmod")
        {
            self.validate_builtin_call_arity(call)?;
            Expression::Call(CallExpr {
                name: call.name.clone(),
                args: vec![self.lower_expression(&call.args[0])?],
                span: call.span,
            })
        } else {
            self.lower_expression(&lhs_source)?
        };
        let valid_lhs = matches!(&lhs, Expression::BranchAccess(_))
            || matches!(&lhs, Expression::Call(call)
                if matches!(call.name.as_str(), "ddt" | "idt" | "idtmod")
                    && matches!(call.args.first(), Some(Expression::BranchAccess(_))));
        if !valid_lhs {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(
                    "the left side of an indirect equation must be an access function, or ddt, idt, or idtmod applied to an access function".into(),
                ),
                stmt.lhs.span(),
            )));
        }
        let equation_abstol = match explicit_ddt_abstol {
            Some(tolerance) => self.lower_expression(&tolerance)?,
            None => self.indirect_equation_abstol(&lhs, module)?,
        };
        let lhs = self.lower_expression_with_side_effects(&lhs_source, module, sink)?;
        let rhs = self.lower_expression_with_side_effects(&stmt.rhs, module, sink)?;
        for (side, expr) in [("left", &lhs), ("right", &rhs)] {
            let ty = self.infer_type(expr)?;
            if !ty.is_numeric() && ty != ValueType::Unknown && ty != ValueType::NatureAccess {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "numeric".to_string(),
                        found: ty.to_string(),
                        context: format!("{side} side of indirect contribution"),
                    },
                    stmt.span,
                );
            }
        }

        let residual = Self::binary_expr(BinaryOp::Sub, lhs, rhs);

        let site = self.next_analog_site();
        let expression_guard = self.active_site_guard();
        self.record_region(AnalyzedRegion::Contribution(AnalyzedContribution {
            branch: branch_name.clone(),
            declared_branch: declared_branch.clone(),
            is_current,
            indirect: true,
            equation_abstol: Some(equation_abstol.clone()),
            expression: residual.clone(),
            site,
            expression_guard,
            expr_type: ValueType::Real,
            span: stmt.span,
        }));
        // The ordinary static-guard pass must see a zero fallback so it can
        // deactivate this source. Structural stamping then pins the unused
        // current with one identity row; a synthetic I(branch) fallback would
        // hide the activation guard and disagree with the structured CFG.
        let expression = self.apply_guard(residual, Self::number_expr(0.0, stmt.span));

        module.contributions.push(AnalyzedContribution {
            branch: branch_name,
            declared_branch,
            is_current,
            indirect: true,
            equation_abstol: Some(equation_abstol),
            expression,
            site,
            expression_guard,
            expr_type: ValueType::Real,
            span: stmt.span,
        });

        Ok(())
    }

    fn indirect_equation_abstol(
        &self,
        lhs: &Expression,
        module: &AnalyzedModule,
    ) -> CompileResult<Expression> {
        let (access, operator) = match lhs {
            Expression::BranchAccess(access) => (access, None),
            Expression::Call(call) => {
                let Some(Expression::BranchAccess(access)) = call.args.first() else {
                    unreachable!("indirect LHS was validated before tolerance resolution");
                };
                (access, Some(call.name.as_str()))
            }
            _ => unreachable!("indirect LHS was validated before tolerance resolution"),
        };
        let kind = self.resolve_branch_access_kind(access, lhs.span())?;
        let names = match access {
            BranchAccess::Nodes { pos, neg, .. } => [Some(pos.as_str()), neg.as_deref()],
            BranchAccess::Branch { name, .. } => [Some(name.as_str()), None],
        };
        let names = if let [Some(name), None] = names
            && let Some(branch) = module.branches.iter().find(|branch| branch.name == name)
        {
            [
                Some(branch.pos_node.as_str()),
                Some(branch.neg_node.as_str()),
            ]
        } else {
            names
        };
        let mut tolerance: Option<f64> = None;
        for name in names.into_iter().flatten() {
            let Some(symbol) = self.symbols.lookup(name) else {
                continue;
            };
            if symbol.attrs.is_ground {
                continue;
            }
            let discipline = symbol.attrs.discipline.as_deref().unwrap_or("electrical");
            if let Some(value) = self.indirect_discipline_abstol(discipline, kind, operator) {
                tolerance = Some(tolerance.map_or(value, |current| current.min(value)));
            }
        }
        let tolerance = tolerance
            .or_else(|| {
                names
                    .into_iter()
                    .flatten()
                    .filter_map(|name| {
                        self.symbols
                            .lookup(name)
                            .and_then(|symbol| symbol.attrs.discipline.as_deref())
                            .and_then(|discipline| {
                                self.indirect_discipline_abstol(discipline, kind, operator)
                            })
                    })
                    .reduce(f64::min)
            })
            .or_else(|| self.indirect_discipline_abstol("electrical", kind, operator))
            .ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidContribution(
                        "indirect equation has no resolved nature tolerance".into(),
                    ),
                    lhs.span(),
                ))
            })?;
        Ok(Self::number_expr(tolerance, lhs.span()))
    }

    fn indirect_discipline_abstol(
        &self,
        discipline: &str,
        kind: AccessKind,
        operator: Option<&str>,
    ) -> Option<f64> {
        let discipline = self.disciplines.get_discipline(discipline)?;
        let name = match kind {
            AccessKind::Potential => &discipline.potential,
            AccessKind::Flow => &discipline.flow,
        }
        .as_deref()?;
        let nature = self.disciplines.get_nature(name)?;
        let transformed = match operator {
            Some("ddt") => nature.ddt_nature.as_deref(),
            Some("idt" | "idtmod") => nature.idt_nature.as_deref(),
            _ => None,
        };
        Some(match transformed {
            Some(name) => self.disciplines.get_nature(name)?.abstol,
            None => nature.abstol,
        })
    }

    /// `$bound_step(max_dt)`: cap the next transient step while the call
    /// is active. Lowers to `$bound_step = min($bound_step, max_dt)` on a
    /// hidden variable reset to +inf at the top of every evaluation, so
    /// multiple calls and guards compose naturally.
    fn analyze_bound_step(
        &mut self,
        call: &CallStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let Some(arg) = call.args.first() else {
            self.record_error_at(
                SemanticErrorKind::MissingAttribute(
                    "$bound_step requires a maximum-step argument".into(),
                ),
                call.span,
            );
            return Ok(());
        };
        let var_index = self.ensure_task_variable("$bound_step", f64::INFINITY, module, call.span);
        let bound = self.lower_expression_with_side_effects(arg, module, sink)?;
        let current = Expression::Identifier(Identifier {
            name: "$bound_step".into(),
            span: call.span,
        });
        let min = Expression::Call(CallExpr {
            name: "min".into(),
            args: vec![current.clone(), bound],
            span: call.span,
        });
        let expression_guard = self.active_site_guard();
        let mut assignment = AnalyzedAssignment {
            target: "$bound_step".into(),
            var_index,
            index: None,
            expression: min,
            site: self.next_analog_site(),
            expression_guard,
            expr_type: ValueType::Real,
            span: call.span,
            unfiltered_initial_step_guard: None,
        };
        self.record_region(AnalyzedRegion::Assignment(assignment.clone()));
        assignment.expression = self.apply_guard(assignment.expression, current);
        sink.push(AnalyzedStatement::Assignment(assignment));
        Ok(())
    }

    /// A discontinuity publishes independent transient and Newton hints.
    /// Bit 2 is reserved for invalid instance-dependent degree values.
    fn analyze_discontinuity(
        &mut self,
        call: &CallStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let degree = self.discontinuity_degree(call)?;
        let binary = |op, left, right| {
            Expression::Binary(BinaryExpr {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span: call.span,
            })
        };
        let number = |value| Self::number_expr(value, call.span);
        let mask = if let Some(value) = self.eval_const_invariant(&degree) {
            number(if value == -1.0 { 2.0 } else { 1.0 })
        } else {
            let integral = binary(
                BinaryOp::Eq,
                degree.clone(),
                Expression::Call(CallExpr {
                    name: "floor".into(),
                    args: vec![degree.clone()],
                    span: call.span,
                }),
            );
            let finite = binary(BinaryOp::Lt, degree.clone(), number(f64::INFINITY));
            let valid = binary(
                BinaryOp::And,
                binary(BinaryOp::Ge, degree.clone(), number(-1.0)),
                binary(BinaryOp::And, finite, integral),
            );
            Expression::Conditional(ConditionalExpr {
                condition: Box::new(valid),
                then_expr: Box::new(Expression::Conditional(ConditionalExpr {
                    condition: Box::new(binary(BinaryOp::Eq, degree, number(-1.0))),
                    then_expr: Box::new(number(2.0)),
                    else_expr: Box::new(number(1.0)),
                    span: call.span,
                })),
                else_expr: Box::new(number(4.0)),
                span: call.span,
            })
        };
        let var_index = self.ensure_task_variable("$discontinuity", 0.0, module, call.span);
        let current = Expression::Identifier(Identifier {
            name: "$discontinuity".into(),
            span: call.span,
        });
        let expression_guard = self.active_site_guard();
        let mut assignment = AnalyzedAssignment {
            target: "$discontinuity".into(),
            var_index,
            index: None,
            expression: binary(BinaryOp::BitOr, current.clone(), mask),
            site: self.next_analog_site(),
            expression_guard,
            expr_type: ValueType::Real,
            span: call.span,
            unfiltered_initial_step_guard: None,
        };
        self.record_region(AnalyzedRegion::Assignment(assignment.clone()));
        assignment.expression = self.apply_guard(assignment.expression, current);
        sink.push(AnalyzedStatement::Assignment(assignment));
        Ok(())
    }

    fn discontinuity_degree(&mut self, call: &CallStmt) -> CompileResult<Expression> {
        let degree = match call.args.first() {
            Some(value) => {
                self.lower_expression_without_side_effects(value, "$discontinuity degree")?
            }
            None => Self::number_expr(0.0, call.span),
        };
        let invalid = || {
            CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::InvalidAnalogOperator(
                "$discontinuity degree must be a numeric constant expression with a finite integer value >= -1".into(),
            ), call.span,
        ))
        };
        let mut pending = vec![&degree];
        while let Some(value) = pending.pop() {
            match value {
                Expression::Number(_) => {}
                Expression::Identifier(id)
                    if id.name == "inf"
                        || self.invariant_consts.contains_key(&id.name)
                        || self
                            .symbols
                            .lookup(&id.name)
                            .is_some_and(|symbol| symbol.kind == SymbolKind::Parameter) => {}
                Expression::Unary(value) => pending.push(&value.operand),
                Expression::Binary(value) => {
                    pending.push(&value.left);
                    pending.push(&value.right);
                }
                Expression::Conditional(value) => {
                    pending.extend([&*value.condition, &*value.then_expr, &*value.else_expr]);
                }
                Expression::Call(value)
                    if self
                        .functions
                        .get(&value.name)
                        .is_some_and(|function| !function.is_analog_operator) =>
                {
                    pending.extend(&value.args)
                }
                _ => return Err(invalid()),
            }
        }
        if !self.infer_type(&degree)?.is_numeric()
            || self
                .eval_const_invariant(&degree)
                .is_some_and(|value| !value.is_finite() || value.fract() != 0.0 || value < -1.0)
        {
            return Err(invalid());
        }
        Ok(degree)
    }

    /// Register a hidden system-task variable on first use and emit its
    /// unguarded per-evaluation reset (the `$`-prefixed name cannot
    /// collide with user identifiers)
    fn ensure_task_variable(
        &mut self,
        name: &str,
        reset: f64,
        module: &mut AnalyzedModule,
        span: Span,
    ) -> usize {
        if let Some(&idx) = self.task_vars.get(name) {
            return idx;
        }
        let var_index = module.variables.len();
        module.variables.push(AnalyzedVariable {
            name: name.into(),
            var_type: VarType::Real,
            value_type: ValueType::Real,
            is_state: false,
        });
        self.task_vars.insert(name.into(), var_index);
        // The reset runs unconditionally: every evaluation starts neutral
        // and only active calls move the value
        let site = self.next_analog_site();
        self.task_resets.push(AnalyzedAssignment {
            target: name.into(),
            var_index,
            index: None,
            expression: Self::number_expr(reset, span),
            site,
            expression_guard: AnalogSiteGuard::None,
            expr_type: ValueType::Real,
            span,
            unfiltered_initial_step_guard: None,
        });
        var_index
    }

    /// Resolve a contribution target (node pair or named branch) to the
    /// IR branch string and its flow/potential kind
    fn resolve_contribution_target(
        &mut self,
        target: &BranchAccess,
        module: &AnalyzedModule,
        span: Span,
    ) -> CompileResult<(SmolStr, bool, Option<SmolStr>)> {
        let is_current = self.resolve_branch_access_kind(target, span)? == AccessKind::Flow;
        match target {
            BranchAccess::Nodes { pos, neg, .. } => {
                // V(name)/I(name) where `name` is a declared branch resolves
                // through the branch table
                if neg.is_none()
                    && let Some(branch) = module.branches.iter().find(|b| b.name == *pos)
                {
                    let branch_str = if branch.neg_node.is_empty() {
                        branch.pos_node.to_string()
                    } else {
                        format!("{},{}", branch.pos_node, branch.neg_node)
                    };
                    Ok((
                        SmolStr::from(branch_str),
                        is_current,
                        Some(branch.name.clone()),
                    ))
                } else {
                    self.validate_node(pos, span)?;
                    if let Some(n) = neg {
                        self.validate_node(n, span)?;
                    }
                    self.validate_distinct_branch_nodes(pos, neg.as_deref().unwrap_or("0"), span)?;
                    // Format as "pos,neg" for IR parser compatibility
                    let branch = if neg.is_some() {
                        format!("{},{}", pos, neg.as_deref().unwrap())
                    } else {
                        pos.to_string()
                    };
                    Ok((branch.into(), is_current, None))
                }
            }
            BranchAccess::Branch { name, .. } => {
                match module.branches.iter().find(|b| b.name == *name) {
                    Some(branch) => {
                        let branch_str = if branch.neg_node.is_empty() {
                            branch.pos_node.to_string()
                        } else {
                            format!("{},{}", branch.pos_node, branch.neg_node)
                        };
                        Ok((
                            SmolStr::from(branch_str),
                            is_current,
                            Some(branch.name.clone()),
                        ))
                    }
                    None => {
                        self.validate_node(name, span)?;
                        Ok((name.to_string().into(), is_current, None))
                    }
                }
            }
        }
    }

    fn validate_distinct_branch_nodes(
        &self,
        pos: &str,
        neg: &str,
        span: Span,
    ) -> CompileResult<()> {
        let ground = |name: &str| {
            name.is_empty()
                || is_global_ground_name(name)
                || self
                    .symbols
                    .lookup(name)
                    .is_some_and(|symbol| symbol.attrs.is_ground)
        };
        if pos == neg || (ground(pos) && ground(neg)) {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidBranch("branch endpoints must name distinct nets".into()),
                span,
            )));
        }
        Ok(())
    }

    fn resolve_branch_access_kind(
        &self,
        access_expr: &BranchAccess,
        span: Span,
    ) -> CompileResult<AccessKind> {
        let (access, pos, neg) = match access_expr {
            BranchAccess::Nodes {
                access, pos, neg, ..
            } => (access.as_str(), pos.as_str(), neg.as_deref()),
            BranchAccess::Branch { access, name, .. } => (access.as_str(), name.as_str(), None),
        };
        let pos_kind = self.resolve_access_with_symbol(access, pos, span)?;
        let neg_kind = neg
            .map(|neg| self.resolve_access_with_symbol(access, neg, span))
            .transpose()?
            .flatten();
        if let (Some(pos), Some(neg)) = (pos_kind, neg_kind)
            && pos != neg
        {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(format!(
                    "access function '{access}' has different physical roles at the branch endpoints"
                )),
                span,
            )));
        }
        let kind = pos_kind
            .or(neg_kind)
            .or_else(|| {
                // Ground-only potential reads still retain the declared
                // physical role (for example, Temp(thermal_ground)).
                [Some(pos), neg].into_iter().flatten().find_map(|name| {
                    self.symbols
                        .lookup(name)
                        .and_then(|symbol| symbol.attrs.discipline.as_deref())
                        .and_then(|discipline| self.disciplines.access_kind(discipline, access))
                })
            })
            .or_else(|| self.disciplines.access_kind("electrical", access))
            .ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidContribution(format!(
                        "access function '{access}' has no resolved branch discipline"
                    )),
                    span,
                ))
            })?;
        // Existing compact models use V(n,n) as a zero-valued read. Retain
        // that compatibility without accepting undefined self-flow probes.
        if kind == AccessKind::Flow
            && matches!(access_expr, BranchAccess::Nodes { .. })
            && (neg.is_some()
                || !self
                    .symbols
                    .lookup(pos)
                    .is_some_and(|symbol| symbol.kind == SymbolKind::Branch))
        {
            self.validate_distinct_branch_nodes(pos, neg.unwrap_or("0"), span)?;
        }
        Ok(kind)
    }

    fn resolve_access_with_symbol(
        &self,
        access: &str,
        name: &str,
        span: Span,
    ) -> CompileResult<Option<AccessKind>> {
        if self.disciplines.resolve_access(access).is_none() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(format!(
                    "unknown access function '{access}'"
                )),
                span,
            )));
        }
        let Some(symbol) = self.symbols.lookup(name) else {
            if is_global_ground_name(name) {
                return Ok(None);
            }
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndeclaredSymbol { name: name.into() },
                span,
            )));
        };
        if !matches!(
            symbol.kind,
            SymbolKind::Port | SymbolKind::Node | SymbolKind::Branch
        ) {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidNodeReference {
                    name: name.into(),
                    kind: format!("{:?}", symbol.kind),
                },
                span,
            )));
        }
        // A reference net is compatible with every continuous discipline;
        // resolve the physical role using the other branch endpoint.
        if symbol.attrs.is_ground {
            return Ok(None);
        }
        let discipline = symbol.attrs.discipline.as_deref().unwrap_or("electrical");
        if self.disciplines.get_discipline(discipline).is_none() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndefinedDiscipline(discipline.to_string()),
                span,
            )));
        }
        if let Some(kind) = self.disciplines.access_kind(discipline, access) {
            return Ok(Some(kind));
        }

        Err(CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::InvalidContribution(format!(
                "access function '{access}' is incompatible with discipline '{discipline}'"
            )),
            span,
        )))
    }

    /// Put implicit real-to-integer conversion into the shared expression
    /// before guards, correspondence and AD are built. The explicit internal
    /// operator remains distinguishable from user-authored bitwise syntax.
    fn coerce_assignment_expression(
        &self,
        expression: Expression,
        target_type: ValueType,
    ) -> CompileResult<(Expression, ValueType)> {
        let expression = self.normalize_integer_expression(&expression)?;
        let source_type = self.infer_type(&expression)?;
        if target_type == ValueType::Integer
            && matches!(source_type, ValueType::Real | ValueType::NatureAccess)
        {
            let span = expression.span();
            Ok((
                Expression::Unary(UnaryExpr {
                    op: UnaryOp::ToInteger,
                    operand: Box::new(expression),
                    span,
                }),
                ValueType::Integer,
            ))
        } else {
            Ok((expression, source_type))
        }
    }

    fn analyze_assignment(
        &mut self,
        assign: &AssignmentStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        // `symbol_name` is the declared symbol checked for kind/type;
        // `target_name` is the storage slot (for const-index array elements
        // they differ: symbol `arr`, storage `arr[k]`)
        let (symbol_name, target_name, span, dyn_index) = match &assign.target {
            LValue::Variable { name, span } => {
                let resolved = self.resolve_substituted_name(name);
                if self.symbols.lookup(&resolved).is_none() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                        *span,
                    )));
                }
                self.symbols.mark_used(&resolved);
                (resolved.clone(), resolved, *span, None)
            }
            LValue::ArrayAccess { name, index, span } => {
                let array_name = self.resolve_substituted_name(name);
                let Some(layout) = self.arrays.get(&array_name).cloned() else {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "'{}' is indexed but is not a declared array variable",
                            name
                        )),
                        *span,
                    );
                    return Ok(());
                };
                self.symbols.mark_used(&array_name);
                let index = self.lower_expression_with_side_effects(index, module, sink)?;
                if let Some(k) = self.eval_const_invariant(&index) {
                    // Compile-time index: target the element slot directly
                    let k = k.round() as i64;
                    self.check_array_bounds(&array_name, &layout, k, *span)?;
                    let elem = SmolStr::from(format!("{array_name}[{k}]"));
                    (array_name, elem, *span, None)
                } else {
                    (array_name.clone(), array_name, *span, Some(index))
                }
            }
        };

        // Assignments may only target variables
        if let Some(sym) = self.symbols.lookup(&symbol_name)
            && !matches!(sym.kind, SymbolKind::Variable | SymbolKind::LoopVar)
        {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: "variable".to_string(),
                    found: format!("{:?}", sym.kind).to_lowercase(),
                    context: format!("assignment to '{}'", target_name),
                },
                span,
            );
            return Ok(());
        }

        let expression = self.lower_expression_with_side_effects(&assign.value, module, sink)?;
        let target_type = self
            .symbols
            .lookup(&symbol_name)
            .map_or(ValueType::Unknown, |symbol| symbol.value_type);
        let (expression, value_type) =
            self.coerce_assignment_expression(expression, target_type)?;

        if let Some(sym) = self.symbols.lookup(&symbol_name)
            && !value_type.can_coerce_to(&sym.value_type)
        {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: sym.value_type.to_string(),
                    found: value_type.to_string(),
                    context: format!("assignment to '{}'", target_name),
                },
                span,
            );
        }

        if let Some(index) = dyn_index {
            return self.push_indexed_assignment(
                target_name,
                index,
                expression,
                value_type,
                span,
                sink,
            );
        }

        // Find variable index; assignments to unknown storage are an error
        let Some(var_index) = module.variables.iter().position(|v| v.name == target_name) else {
            self.record_error_at(
                SemanticErrorKind::UndeclaredSymbol {
                    name: target_name.clone(),
                },
                span,
            );
            return Ok(());
        };

        // Under a guard, the variable keeps its previous value when the
        // guard is inactive
        let fallback = Expression::Identifier(Identifier {
            name: target_name.clone(),
            span,
        });
        // Structured form first: it needs the expression as written, and the
        // next line is where that stops being available.
        let site = self.next_analog_site();
        let expression_guard = self.active_site_guard();
        self.record_region(AnalyzedRegion::Assignment(AnalyzedAssignment {
            target: target_name.clone(),
            var_index,
            index: None,
            expression: expression.clone(),
            site,
            expression_guard,
            expr_type: value_type,
            span,
            unfiltered_initial_step_guard: self.unfiltered_initial_step_guards.last().cloned(),
        }));
        let expression = self.apply_guard(expression, fallback);

        // Record the assignment for code generation
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: target_name,
            var_index,
            index: None,
            expression,
            site,
            expression_guard,
            expr_type: value_type,
            span,
            unfiltered_initial_step_guard: self.unfiltered_initial_step_guards.last().cloned(),
        }));

        Ok(())
    }

    /// Record an array element assignment whose index is only known at
    /// runtime. Guards fall back to re-reading the same element, so an
    /// inactive guard leaves the array untouched.
    fn push_indexed_assignment(
        &mut self,
        array_name: SmolStr,
        index: Expression,
        expression: Expression,
        value_type: ValueType,
        span: Span,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let layout = self.arrays.get(&array_name).cloned().expect("checked");
        let fallback = Expression::ArrayAccess(ArrayAccessExpr {
            array: array_name.clone(),
            index: Box::new(index.clone()),
            span,
        });
        let site = self.next_analog_site();
        let expression_guard = self.active_site_guard();
        self.record_region(AnalyzedRegion::Assignment(AnalyzedAssignment {
            target: array_name.clone(),
            var_index: layout.base,
            index: Some(index.clone()),
            expression: expression.clone(),
            site,
            expression_guard,
            expr_type: value_type,
            span,
            unfiltered_initial_step_guard: None,
        }));
        let expression = self.apply_guard(expression, fallback);
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: array_name,
            var_index: layout.base,
            index: Some(index),
            expression,
            site,
            expression_guard,
            expr_type: value_type,
            span,
            unfiltered_initial_step_guard: None,
        }));
        Ok(())
    }

    /// Validate a compile-time array index against the declared bounds
    fn check_array_bounds(
        &self,
        name: &SmolStr,
        layout: &AnalyzedArray,
        k: i64,
        span: Span,
    ) -> CompileResult<()> {
        if k < layout.lower || k >= layout.lower + layout.len as i64 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::IndexOutOfBounds(format!(
                    "index {k} outside '{name}[{}:{}]'",
                    layout.lower,
                    layout.lower + layout.len as i64 - 1
                )),
                span,
            )));
        }
        Ok(())
    }

    /// Resolve a name through the substitution stack (innermost first).
    /// Only identity renames (identifier-to-identifier) are returned; other
    /// substitutions keep the original name.
    fn resolve_substituted_name(&self, name: &SmolStr) -> SmolStr {
        for frame in self.subst_stack.iter().rev() {
            if let Some(expr) = frame.get(name) {
                if let Expression::Identifier(id) = expr {
                    return id.name.clone();
                }
                return name.clone();
            }
        }
        name.clone()
    }

    /// Look up a substitution for an identifier
    fn lookup_substitution(&self, name: &SmolStr) -> Option<Expression> {
        for frame in self.subst_stack.iter().rev() {
            if let Some(expr) = frame.get(name) {
                return Some(expr.clone());
            }
        }
        None
    }

    fn materialize_output_function_call(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Option<Expression>> {
        let Expression::Call(call) = expr else {
            return Ok(None);
        };
        let Some(func) = self.user_functions.get(&call.name).cloned() else {
            return Ok(None);
        };
        if !self.function_should_materialize(&func) {
            return Ok(None);
        }
        if self.inline_depth >= Self::MAX_INLINE_DEPTH {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::CircularDependency(format!(
                    "analog function '{}' (recursive call chain?)",
                    call.name
                )),
                call.span,
            )));
        }
        if call.args.len() != func.params.len() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: call.name.to_string(),
                    expected: func.params.len().to_string(),
                    got: call.args.len(),
                },
                call.span,
            )));
        }

        let mut output_bindings = Vec::new();
        for (param, arg) in func.params.iter().zip(call.args.iter()) {
            if param.direction != ParamDirection::Input {
                let target = self.function_output_lvalue(&func.name, param, arg)?;
                output_bindings.push((param.name.clone(), target, param.span));
            }
        }

        self.local_counter += 1;
        let call_id = self.local_counter;
        let prefix = format!("__fn{call_id}_{}", func.name);
        let make_name = |name: &SmolStr| -> SmolStr { SmolStr::from(format!("{prefix}__{name}")) };

        let return_name: SmolStr = format!("{prefix}__return").into();
        self.register_function_temp(module, return_name.clone(), func.return_type, func.span)?;

        let mut frame = HashMap::new();
        frame.insert(
            func.name.clone(),
            Expression::Identifier(Identifier {
                name: return_name.clone(),
                span: func.span,
            }),
        );

        let mut formal_temps = HashMap::new();
        for param in &func.params {
            let temp_name = make_name(&param.name);
            self.register_function_temp(module, temp_name.clone(), param.param_type, param.span)?;
            formal_temps.insert(param.name.clone(), temp_name.clone());
            frame.insert(
                param.name.clone(),
                Expression::Identifier(Identifier {
                    name: temp_name,
                    span: param.span,
                }),
            );
        }

        let zero_return = AssignmentStmt {
            target: LValue::Variable {
                name: return_name.clone(),
                span: func.span,
            },
            value: Self::number_expr(0.0, func.span),
            span: func.span,
        };
        self.analyze_assignment(&zero_return, module, sink)?;

        for (param, arg) in func.params.iter().zip(call.args.iter()) {
            let formal_temp = formal_temps
                .get(&param.name)
                .expect("formal temp registered")
                .clone();
            let value = match param.direction {
                ParamDirection::Input | ParamDirection::Inout => arg.clone(),
                ParamDirection::Output => Self::number_expr(0.0, param.span),
            };
            let assignment = AssignmentStmt {
                target: LValue::Variable {
                    name: formal_temp,
                    span: param.span,
                },
                value,
                span: param.span,
            };
            self.analyze_assignment(&assignment, module, sink)?;
        }

        for var_decl in &func.locals {
            for item in &var_decl.items {
                if func.params.iter().any(|param| param.name == item.name) {
                    continue;
                }
                if !item.dimensions.is_empty() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "array local '{}' in analog function '{}'",
                            item.name, func.name
                        )),
                        item.span,
                    )));
                }
                let temp_name = make_name(&item.name);
                self.register_function_temp(
                    module,
                    temp_name.clone(),
                    var_decl.var_type,
                    item.span,
                )?;
                frame.insert(
                    item.name.clone(),
                    Expression::Identifier(Identifier {
                        name: temp_name.clone(),
                        span: item.span,
                    }),
                );
            }
        }

        self.subst_stack.push(frame);
        self.inline_depth += 1;
        let body_result = (|| -> CompileResult<()> {
            for var_decl in &func.locals {
                for item in &var_decl.items {
                    if func.params.iter().any(|param| param.name == item.name) {
                        continue;
                    }
                    let target_name = self.resolve_substituted_name(&item.name);
                    let value = item
                        .init
                        .clone()
                        .unwrap_or_else(|| Self::number_expr(0.0, item.span));
                    let assignment = AssignmentStmt {
                        target: LValue::Variable {
                            name: target_name,
                            span: item.span,
                        },
                        value,
                        span: item.span,
                    };
                    self.analyze_assignment(&assignment, module, sink)?;
                }
            }
            for statement in &func.body.statements {
                self.analyze_statement(statement, module, sink)?;
            }
            Ok(())
        })();
        self.inline_depth -= 1;
        self.subst_stack.pop().expect("function frame");
        body_result?;

        for (formal, target, span) in output_bindings {
            let formal_temp = formal_temps.get(&formal).expect("formal temp registered");
            let assignment = AssignmentStmt {
                target,
                value: Expression::Identifier(Identifier {
                    name: formal_temp.clone(),
                    span,
                }),
                span,
            };
            self.analyze_assignment(&assignment, module, sink)?;
        }

        Ok(Some(Expression::Identifier(Identifier {
            name: return_name,
            span: call.span,
        })))
    }

    fn lower_expression_with_side_effects(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Expression> {
        // Borrow trees that need neither function materialization nor a solver
        // equation for an integrator without an explicit initial condition.
        let materialized = if self.user_functions.is_empty()
            && !self.expression_contains_output_function_call(expr)
        {
            None
        } else {
            Some(self.materialize_output_function_calls(expr, module, sink)?)
        };
        let expr = materialized.as_ref().unwrap_or(expr);
        let side_effect_start = self.function_side_effects.len();
        let lowered = self.lower_expression(expr)?;
        let side_effects = self.function_side_effects.split_off(side_effect_start);
        for assignment in side_effects {
            self.analyze_assignment(&assignment, module, sink)?;
        }
        Ok(lowered)
    }

    fn materialize_output_function_calls(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Expression> {
        if matches!(expr, Expression::Binary(_) | Expression::Unary(_)) {
            rewrite_operator_tree(expr, |node| match node {
                OperatorRewrite::Leaf(expression) => {
                    self.materialize_non_operator_function_calls(expression, module, sink)
                }
                OperatorRewrite::Binary(binary) => Ok(Expression::Binary(binary)),
                OperatorRewrite::Unary(unary) => Ok(Expression::Unary(unary)),
            })
        } else {
            self.materialize_non_operator_function_calls(expr, module, sink)
        }
    }

    fn materialize_non_operator_function_calls(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Expression> {
        Ok(match expr {
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_)
            // A discrete-domain expression cannot contain an analog function
            // call, so there is nothing to materialize out of one.
            | Expression::Digital(_)
            | Expression::BranchAccess(_) => expr.clone(),
            Expression::SystemFunction(function) => {
                if let Some(resolved) = self.lower_module_time_function(function)? {
                    return Ok(resolved);
                }
                let mut function = SystemFunction {
                    name: function.name.clone(),
                    args: function.args.iter()
                        .map(|arg| self.materialize_output_function_calls(arg, module, sink))
                        .collect::<CompileResult<Vec<_>>>()?,
                    span: function.span,
                };
                if self.uses_default_limit_recommendation(&function) {
                    // Preserve evaluation and name checking of recommendation
                    // operands even though the chosen default does not use
                    // their values. Literal metadata needs no runtime storage.
                    for argument in &function.args[2..] {
                        if matches!(argument, Expression::Number(_) | Expression::StringLit(_)) {
                            continue;
                        }
                        self.local_counter += 1;
                        let name: SmolStr = format!("__limit_argument{}", self.local_counter).into();
                        let var_type = if self.infer_type(argument)? == ValueType::Integer {
                            VarType::Integer
                        } else { VarType::Real };
                        self.register_function_temp(module, name.clone(), var_type, argument.span())?;
                        self.analyze_assignment(&AssignmentStmt {
                            target: LValue::Variable { name, span: argument.span() },
                            value: argument.clone(), span: argument.span(),
                        }, module, sink)?;
                    }
                    function.args.truncate(1);
                }
                Expression::SystemFunction(function)
            }
            Expression::Binary(_) | Expression::Unary(_) => {
                unreachable!("operator expressions use the iterative materialization path")
            }
            Expression::Conditional(conditional) => {
                if self.expression_contains_output_function_call(expr) {
                    let value_type = self.infer_type(expr)?;
                    self.local_counter += 1;
                    let name: SmolStr = format!("__conditional{}", self.local_counter).into();
                    let var_type = if value_type == ValueType::Integer { VarType::Integer } else { VarType::Real };
                    self.register_function_temp(module, name.clone(), var_type, conditional.span)?;
                    let assignment = |value: &Expression| Box::new(AnalogStatement::Assignment(AssignmentStmt {
                        target: LValue::Variable { name: name.clone(), span: conditional.span },
                        value: value.clone(), span: conditional.span,
                    }));
                    self.analyze_statement(&AnalogStatement::Conditional(ConditionalStmt {
                        condition: (*conditional.condition).clone(),
                        then_branch: assignment(&conditional.then_expr),
                        else_branch: Some(assignment(&conditional.else_expr)),
                        span: conditional.span,
                    }), module, sink)?;
                    return Ok(Expression::Identifier(Identifier { name, span: conditional.span }));
                }
                Expression::Conditional(ConditionalExpr {
                    condition: Box::new(self.materialize_output_function_calls(
                        &conditional.condition,
                        module,
                        sink,
                    )?),
                    then_expr: Box::new(self.materialize_output_function_calls(
                        &conditional.then_expr,
                        module,
                        sink,
                    )?),
                    else_expr: Box::new(self.materialize_output_function_calls(
                        &conditional.else_expr,
                        module,
                        sink,
                    )?),
                    span: conditional.span,
                })
            }
            Expression::Call(call) => {
                if call.name == "idt" && call.args.len() == 1
                    && !matches!(call.args[0], Expression::NullArgument(_))
                {
                    return self.materialize_implicit_integrator(call, module, sink);
                }
                if let Some(func) = self.user_functions.get(&call.name).cloned()
                    && self.function_should_materialize(&func)
                {
                    let args = if call.args.len() == func.params.len() {
                        call.args
                            .iter()
                            .zip(func.params.iter())
                            .map(|(arg, param)| match param.direction {
                                ParamDirection::Input => {
                                    self.materialize_output_function_calls(arg, module, sink)
                                }
                                ParamDirection::Output | ParamDirection::Inout => Ok(arg.clone()),
                            })
                            .collect::<CompileResult<Vec<_>>>()?
                    } else {
                        call.args.clone()
                    };
                    let call = Expression::Call(CallExpr {
                        name: call.name.clone(),
                        args,
                        span: call.span,
                    });
                    return self
                        .materialize_output_function_call(&call, module, sink)?
                        .ok_or_else(|| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidAnalogOperator(
                                    "analog function output/inout call could not be materialized"
                                        .into(),
                                ),
                                expr.span(),
                            ))
                        });
                }
                Expression::Call(CallExpr {
                    name: call.name.clone(),
                    args: call
                        .args
                        .iter()
                        .map(|arg| self.materialize_output_function_calls(arg, module, sink))
                        .collect::<CompileResult<Vec<_>>>()?,
                    span: call.span,
                })
            }
            Expression::ArrayAccess(access) => Expression::ArrayAccess(ArrayAccessExpr {
                array: access.array.clone(),
                index: Box::new(self.materialize_output_function_calls(
                    &access.index,
                    module,
                    sink,
                )?),
                span: access.span,
            }),
            Expression::ArrayLiteral(array) => Expression::ArrayLiteral(ArrayLiteralExpr {
                elements: array
                    .elements
                    .iter()
                    .map(|element| {
                        self.materialize_output_function_calls_in_array_element(
                            element, module, sink,
                        )
                    })
                    .collect::<CompileResult<Vec<_>>>()?,
                assignment_pattern: array.assignment_pattern,
                span: array.span,
            }),
            Expression::AnalogOperator(op) => Expression::AnalogOperator(
                self.materialize_output_function_calls_in_analog_operator(op, module, sink)?,
            ),
            Expression::NoiseSource(noise) => Expression::NoiseSource(
                self.materialize_output_function_calls_in_noise_source(noise, module, sink)?,
            ),
        })
    }

    fn materialize_output_function_calls_in_array_element(
        &mut self,
        element: &ArrayLiteralElement,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<ArrayLiteralElement> {
        Ok(match element {
            ArrayLiteralElement::Value(expression) => ArrayLiteralElement::Value(
                self.materialize_output_function_calls(expression, module, sink)?,
            ),
            ArrayLiteralElement::Replication(replication) => {
                ArrayLiteralElement::Replication(ReplicationExpr {
                    count: Box::new(self.materialize_output_function_calls(
                        &replication.count,
                        module,
                        sink,
                    )?),
                    elements: replication
                        .elements
                        .iter()
                        .map(|element| {
                            self.materialize_output_function_calls_in_array_element(
                                element, module, sink,
                            )
                        })
                        .collect::<CompileResult<Vec<_>>>()?,
                    span: replication.span,
                })
            }
        })
    }

    fn materialize_output_function_calls_in_analog_operator(
        &mut self,
        op: &AnalogOperator,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<AnalogOperator> {
        Ok(match op {
            AnalogOperator::Limit {
                proposed,
                candidate,
                type_metadata,
                selector,
                span,
            } => AnalogOperator::Limit {
                proposed: self.materialize_output_function_calls_box(proposed, module, sink)?,
                candidate: self.materialize_output_function_calls_box(candidate, module, sink)?,
                type_metadata: self.materialize_output_function_calls_opt_box(
                    type_metadata,
                    module,
                    sink,
                )?,
                selector: selector.clone(),
                span: *span,
            },
            AnalogOperator::LimiterArgument { argument, span } => AnalogOperator::LimiterArgument {
                argument: *argument,
                span: *span,
            },
        })
    }

    fn materialize_output_function_calls_box(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Box<Expression>> {
        Ok(Box::new(
            self.materialize_output_function_calls(expr, module, sink)?,
        ))
    }

    fn materialize_output_function_calls_opt_box(
        &mut self,
        expr: &Option<Box<Expression>>,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Option<Box<Expression>>> {
        expr.as_deref()
            .map(|expr| self.materialize_output_function_calls_box(expr, module, sink))
            .transpose()
    }

    fn materialize_output_function_calls_in_noise_source(
        &mut self,
        noise: &NoiseSource,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<NoiseSource> {
        Ok(match noise {
            NoiseSource::White {
                process_id,
                power,
                name,
                span,
            } => NoiseSource::White {
                process_id: *process_id,
                power: Box::new(self.materialize_output_function_calls(power, module, sink)?),
                name: name.clone(),
                span: *span,
            },
            NoiseSource::Flicker {
                process_id,
                power,
                exponent,
                name,
                span,
            } => NoiseSource::Flicker {
                process_id: *process_id,
                power: Box::new(self.materialize_output_function_calls(power, module, sink)?),
                exponent: Box::new(self.materialize_output_function_calls(exponent, module, sink)?),
                name: name.clone(),
                span: *span,
            },
            NoiseSource::Table {
                process_id,
                data,
                log_interp,
                name,
                span,
            } => NoiseSource::Table {
                process_id: *process_id,
                data: self.materialize_output_function_calls_in_expr_list(data, module, sink)?,
                log_interp: *log_interp,
                name: name.clone(),
                span: *span,
            },
        })
    }

    fn materialize_output_function_calls_in_expr_list(
        &mut self,
        exprs: &[Expression],
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Vec<Expression>> {
        exprs
            .iter()
            .map(|expr| self.materialize_output_function_calls(expr, module, sink))
            .collect()
    }

    fn expression_contains_output_function_call(&self, expr: &Expression) -> bool {
        if !matches!(expr, Expression::Binary(_) | Expression::Unary(_)) {
            return self.non_operator_contains_output_function_call(expr);
        }
        // Operator chains can be much deeper than the host call stack.
        let mut pending = vec![expr];
        while let Some(expression) = pending.pop() {
            match expression {
                Expression::Binary(binary) => {
                    pending.push(&binary.right);
                    pending.push(&binary.left);
                }
                Expression::Unary(unary) => pending.push(&unary.operand),
                _ if self.non_operator_contains_output_function_call(expression) => return true,
                _ => {}
            }
        }
        false
    }

    fn non_operator_contains_output_function_call(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Digital(digital) => digital
                .children()
                .into_iter()
                .any(|child| self.expression_contains_output_function_call(child)),
            Expression::Call(call) => {
                (call.name == "idt" && call.args.len() == 1)
                    || self
                        .user_functions
                        .get(&call.name)
                        .is_some_and(|func| self.function_needs_materialization(func))
                    || call
                        .args
                        .iter()
                        .any(|arg| self.expression_contains_output_function_call(arg))
            }
            Expression::SystemFunction(function) => {
                self.uses_default_limit_recommendation(function)
                    || function
                        .args
                        .iter()
                        .any(|arg| self.expression_contains_output_function_call(arg))
            }
            Expression::Binary(binary) => {
                self.expression_contains_output_function_call(&binary.left)
                    || self.expression_contains_output_function_call(&binary.right)
            }
            Expression::Unary(unary) => {
                self.expression_contains_output_function_call(&unary.operand)
            }
            Expression::Conditional(conditional) => {
                self.expression_contains_output_function_call(&conditional.condition)
                    || self.expression_contains_output_function_call(&conditional.then_expr)
                    || self.expression_contains_output_function_call(&conditional.else_expr)
            }
            Expression::ArrayAccess(access) => {
                self.expression_contains_output_function_call(&access.index)
            }
            Expression::ArrayLiteral(array) => array
                .elements
                .iter()
                .any(|element| self.array_element_contains_output_function_call(element)),
            Expression::AnalogOperator(op) => {
                self.analog_operator_contains_output_function_call(op)
            }
            Expression::NoiseSource(noise) => {
                self.noise_source_contains_output_function_call(noise)
            }
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_) => false,
        }
    }

    fn array_element_contains_output_function_call(&self, element: &ArrayLiteralElement) -> bool {
        match element {
            ArrayLiteralElement::Value(expression) => {
                self.expression_contains_output_function_call(expression)
            }
            ArrayLiteralElement::Replication(replication) => {
                self.expression_contains_output_function_call(&replication.count)
                    || replication
                        .elements
                        .iter()
                        .any(|element| self.array_element_contains_output_function_call(element))
            }
        }
    }

    fn analog_operator_contains_output_function_call(&self, op: &AnalogOperator) -> bool {
        let contains_opt = |expr: &Option<Box<Expression>>| {
            expr.as_ref()
                .is_some_and(|expr| self.expression_contains_output_function_call(expr))
        };
        match op {
            AnalogOperator::Limit {
                proposed,
                candidate,
                type_metadata,
                ..
            } => {
                self.expression_contains_output_function_call(proposed)
                    || self.expression_contains_output_function_call(candidate)
                    || contains_opt(type_metadata)
            }
            AnalogOperator::LimiterArgument { .. } => false,
        }
    }

    fn noise_source_contains_output_function_call(&self, noise: &NoiseSource) -> bool {
        match noise {
            NoiseSource::White { power, .. } => {
                self.expression_contains_output_function_call(power)
            }
            NoiseSource::Flicker {
                power, exponent, ..
            } => {
                self.expression_contains_output_function_call(power)
                    || self.expression_contains_output_function_call(exponent)
            }
            NoiseSource::Table { data, .. } => data
                .iter()
                .any(|expr| self.expression_contains_output_function_call(expr)),
        }
    }

    fn lower_expression_without_side_effects(
        &mut self,
        expr: &Expression,
        context: &str,
    ) -> CompileResult<Expression> {
        let side_effect_start = self.function_side_effects.len();
        let lowered = self.lower_expression(expr)?;
        if self.function_side_effects.len() != side_effect_start {
            self.function_side_effects.truncate(side_effect_start);
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "analog function output/inout arguments are not supported in {context}"
                )),
                expr.span(),
            )));
        }
        Ok(lowered)
    }

    /// Materialize transition timing defaults before IR construction so the
    /// runtime receives the module-scoped `default_transition` value without
    /// extending the compact four-operand VM instruction. Explicit zero rise
    /// or fall values select the same default; an omitted fall reuses the
    /// effective rise time as required by VAMS-2023 4.5.8.
    fn materialize_transition_defaults(&self, args: &mut Vec<Expression>, span: Span) {
        debug_assert!((1..=4).contains(&args.len()));
        if args.len() == 1 {
            args.push(Self::number_expr(0.0, span));
        }
        let raw_rise = args
            .get(2)
            .cloned()
            .unwrap_or_else(|| Self::number_expr(self.current_default_transition, span));
        let effective_rise = self.materialize_transition_time_default(raw_rise, span);
        let raw_fall = args
            .get(3)
            .cloned()
            .unwrap_or_else(|| effective_rise.clone());
        let effective_fall = self.materialize_transition_time_default(raw_fall, span);
        if args.len() == 2 {
            args.push(effective_rise);
        } else {
            args[2] = effective_rise;
        }
        if args.len() == 3 {
            args.push(effective_fall);
        } else {
            args[3] = effective_fall;
        }
    }

    fn materialize_transition_time_default(&self, value: Expression, span: Span) -> Expression {
        let condition = Expression::Binary(BinaryExpr {
            op: BinaryOp::Eq,
            left: Box::new(value.clone()),
            right: Box::new(Self::number_expr(0.0, span)),
            span,
        });
        Expression::Conditional(ConditionalExpr {
            condition: Box::new(condition),
            then_expr: Box::new(Self::number_expr(self.current_default_transition, span)),
            else_expr: Box::new(value),
            span,
        })
    }

    /// Rewrite an expression: apply substitutions (block locals, loop
    /// variables) and inline calls to user-defined analog functions.
    fn lower_expression(&mut self, expr: &Expression) -> CompileResult<Expression> {
        if matches!(expr, Expression::Binary(_) | Expression::Unary(_)) {
            self.lower_operator_expression(expr)
        } else {
            self.lower_non_operator_expression(expr)
        }
    }

    /// Operator chains retain their authored association and left-to-right
    /// lowering order without reserving a semantic-analysis frame per operand.
    fn lower_operator_expression(&mut self, expr: &Expression) -> CompileResult<Expression> {
        resolve_integer_operator_tree(expr, |expression| {
            let expression = self.lower_non_operator_expression(expression)?;
            let value_type = self.infer_type(&expression)?;
            Ok((expression, value_type))
        })
    }

    /// Parameter/default coercion does not perform executable lowering, but
    /// still must retain arithmetic types in every nested scalar expression.
    fn normalize_integer_expression(&self, expr: &Expression) -> CompileResult<Expression> {
        if let Expression::ArrayLiteral(array) = expr {
            let mut array = array.clone();
            for element in &mut array.elements {
                if let ArrayLiteralElement::Value(value) = element {
                    *value = self.normalize_integer_expression(value)?;
                }
            }
            return Ok(Expression::ArrayLiteral(array));
        }
        resolve_integer_operator_tree(expr, |expression| {
            if let Expression::SystemFunction(function) = expression
                && let Some(resolved) = self.lower_module_time_function(function)?
            {
                return Ok((resolved, ValueType::Real));
            }
            let mut expression = expression.clone();
            match &mut expression {
                Expression::Conditional(c) => {
                    *c.condition = self.normalize_integer_expression(&c.condition)?;
                    *c.then_expr = self.normalize_integer_expression(&c.then_expr)?;
                    *c.else_expr = self.normalize_integer_expression(&c.else_expr)?;
                }
                Expression::Call(c) => {
                    self.validate_null_arguments(c)?;
                    for arg in &mut c.args {
                        // A permitted omission carries no scalar value to
                        // coerce. Preserve it for the owning operator.
                        if !matches!(arg, Expression::NullArgument(_)) {
                            *arg = self.normalize_integer_expression(arg)?;
                        }
                    }
                }
                Expression::SystemFunction(c) => {
                    for arg in &mut c.args {
                        *arg = self.normalize_integer_expression(arg)?;
                    }
                }
                Expression::ArrayAccess(a) => {
                    *a.index = self.normalize_integer_expression(&a.index)?
                }
                _ => {}
            }
            let value_type = self.infer_type(&expression)?;
            Ok((expression, value_type))
        })
    }

    /// Resolve declaration-owned queries before flattening and before any
    /// function call in an unused fallback can acquire executable side effects.
    fn lower_module_time_function(
        &self,
        function: &SystemFunction,
    ) -> CompileResult<Option<Expression>> {
        let invalid = |detail: String| {
            CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidExpression(detail),
                function.span,
            ))
        };
        let real = |value| {
            Expression::Number(NumberLit {
                value,
                raw: format!("{value:e}").into(),
                span: function.span,
            })
        };
        if function.name.eq_ignore_ascii_case("$realtime") {
            if !function.args.is_empty() {
                return Err(invalid("$realtime expects no arguments".into()));
            }
            let unit = self
                .current_time_scale
                .unit_seconds()
                .map_err(|detail| invalid(detail.into()))?;
            return Ok(Some(Expression::Binary(BinaryExpr {
                op: BinaryOp::Div,
                left: Box::new(Expression::SystemFunction(SystemFunction {
                    name: "$abstime".into(),
                    args: Vec::new(),
                    span: function.span,
                })),
                right: Box::new(real(unit)),
                span: function.span,
            })));
        }
        if !function.name.eq_ignore_ascii_case("$simparam") {
            return Ok(None);
        }
        let Some(Expression::StringLit(name)) = function.args.first() else {
            return Ok(None);
        };
        let Some(value) = self
            .current_time_scale
            .parameter_value(&name.value)
            .map_err(|detail| invalid(detail.into()))?
        else {
            return Ok(None);
        };
        if !(1..=2).contains(&function.args.len()) {
            return Err(invalid(
                "$simparam expects a query name and at most one numeric fallback".into(),
            ));
        }
        if let Some(fallback) = function.args.get(1) {
            // Retain static name/type checks, but do not execute a fallback for
            // a module declaration that is always available.
            self.validate_discarded_query_names(fallback)?;
            let fallback = self.normalize_integer_expression(fallback)?;
            if self.infer_type(&fallback)? == ValueType::String {
                return Err(invalid("$simparam fallback must be numeric".into()));
            }
        }
        Ok(Some(real(value)))
    }

    fn validate_discarded_query_names(&self, expression: &Expression) -> CompileResult<()> {
        let mut result = Ok(());
        flow_probes::visit_expression(expression, &mut |expression| {
            if result.is_err() {
                return;
            }
            let name = match expression {
                Expression::Identifier(identifier) => Some(&identifier.name),
                Expression::ArrayAccess(access) => Some(&access.array),
                _ => None,
            };
            if let Some(name) = name
                && name != "inf"
                && self.symbols.lookup(name).is_none()
                && self.lookup_substitution(name).is_none()
                && !name
                    .split_once('[')
                    .is_some_and(|(base, _)| self.arrays.contains_key(base))
            {
                result = Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                    expression.span(),
                )));
            }
            if let Expression::Call(call) = expression {
                result = self.validate_builtin_call_arity(call);
                if let Some(function) = self.user_functions.get(&call.name)
                    && call.args.len() != function.params.len()
                {
                    result = Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::ArgumentCountMismatch {
                            name: call.name.to_string(),
                            expected: function.params.len().to_string(),
                            got: call.args.len(),
                        },
                        call.span,
                    )));
                }
            }
        });
        result
    }

    fn lower_non_operator_expression(&mut self, expr: &Expression) -> CompileResult<Expression> {
        Ok(match expr {
            // The refusal that keeps four-state literals and part-selects out
            // of the continuous domain. Every analog expression is lowered
            // here, so a discrete form written in an analog block, an analog
            // function, a parameter default, or a contribution stops with the
            // same diagnostic naming the construct.
            Expression::Digital(digital) => {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "a {} has no value in the continuous (analog) domain; it \
                         is legal only inside an `always`/`initial` process or a \
                         continuous `assign`",
                        digital.construct()
                    )),
                    digital.span(),
                )));
            }
            Expression::Identifier(id) => {
                if self.parameter_arrays.contains(&id.name) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "parameter array '{}' cannot be used as a scalar expression",
                            id.name
                        )),
                        id.span,
                    )));
                }
                match self.lookup_substitution(&id.name) {
                    Some(subst) => subst,
                    None => expr.clone(),
                }
            }
            Expression::Number(_) | Expression::StringLit(_) | Expression::NullArgument(_) => {
                expr.clone()
            }
            Expression::BranchAccess(access) => {
                let kind = self.resolve_branch_access_kind(access, access.span())?;
                Expression::BranchAccess(access.with_kind(kind))
            }
            Expression::Binary(_) | Expression::Unary(_) => {
                unreachable!("operator expressions use the iterative lowering path")
            }
            Expression::Conditional(c) => {
                let condition = self.lower_expression(&c.condition)?;
                let dynamic_condition = !self.expression_is_simulation_invariant(&condition);
                if dynamic_condition {
                    self.dynamic_analog_operator_guard_depth += 1;
                }
                let then_result = self.lower_expression(&c.then_expr);
                let else_result = self.lower_expression(&c.else_expr);
                if dynamic_condition {
                    self.dynamic_analog_operator_guard_depth -= 1;
                }
                Expression::Conditional(ConditionalExpr {
                    condition: Box::new(condition),
                    then_expr: Box::new(then_result?),
                    else_expr: Box::new(else_result?),
                    span: c.span,
                })
            }
            Expression::SystemFunction(f) => {
                if let Some(resolved) = self.lower_module_time_function(f)? {
                    return Ok(resolved);
                }
                self.validate_limit_call(f)?;
                if let Some(limit) = self.lower_custom_limit_call(f)? {
                    return Ok(limit);
                }
                if f.name == "$limit"
                    && let Some(Expression::StringLit(selector)) = f.args.get(1)
                    && Self::builtin_limit_arity(&selector.value).is_none()
                {
                    // Validate the expressions in an unsupported recommendation
                    // before lowering the documented default algorithm.
                    let lowered = f
                        .args
                        .iter()
                        .map(|arg| self.lower_expression(arg))
                        .collect::<CompileResult<Vec<_>>>()?;
                    return Ok(Expression::SystemFunction(SystemFunction {
                        name: f.name.clone(),
                        args: lowered.into_iter().take(1).collect(),
                        span: f.span,
                    }));
                }
                let args = f
                    .args
                    .iter()
                    .map(|a| self.lower_expression(a))
                    .collect::<CompileResult<Vec<_>>>()?;
                if let Some(noise) = self.lower_noise_process_call(&f.name, &args, f.span)? {
                    Expression::NoiseSource(noise)
                } else {
                    Expression::SystemFunction(SystemFunction {
                        name: f.name.clone(),
                        args,
                        span: f.span,
                    })
                }
            }
            Expression::Call(call) => {
                self.validate_builtin_call_arity(call)?;
                self.validate_null_arguments(call)?;
                self.validate_filter_vector_operands(call)?;
                if call.name.eq_ignore_ascii_case("absdelay")
                    && let Some(max_delay) = call.args.get(2)
                    && !self.expression_is_simulation_invariant(max_delay)
                {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(
                            "absdelay maxdelay must be constant for the duration of an analysis"
                                .into(),
                        ),
                        max_delay.span(),
                    )));
                }
                if call.name.eq_ignore_ascii_case("transition") && call.args.len() == 5 {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(
                            "transition time_tol is parsed but exact tolerance-controlled corner placement is not implemented; omit the fifth operand instead of silently discarding it"
                                .into(),
                        ),
                        call.args[4].span(),
                    )));
                }
                let call = self.materialize_filter_call_replication(call)?;
                if is_zi_operator_name(&call.name) {
                    self.validate_zi_operand_budget(&call)?;
                    self.validate_zi_definition_purity(&call)?;
                }

                if let Some(operator) = stateful_analog_operator_call_name(&call.name) {
                    self.validate_stateful_analog_operator_placement(operator, call.span)?;
                }

                // Nature access functions other than V/I (Pwr, Temp, ...)
                // parse as calls; rewrite them into branch accesses.
                if !self.user_functions.contains_key(&call.name)
                    && self.disciplines.resolve_access(&call.name).is_some()
                    && matches!(call.args.len(), 1 | 2)
                    && call.args.iter().all(|a| {
                        matches!(a, Expression::Identifier(id)
                        if self.symbols.lookup(&id.name).is_some_and(|s| matches!(
                            s.kind,
                            SymbolKind::Port | SymbolKind::Node | SymbolKind::Branch
                        )))
                    })
                {
                    let mut nodes = call.args.iter().map(|a| match a {
                        Expression::Identifier(id) => id.name.clone(),
                        _ => unreachable!(),
                    });
                    let access_expr = BranchAccess::Nodes {
                        access: call.name.clone(),
                        kind: None,
                        pos: nodes.next().unwrap(),
                        neg: nodes.next(),
                        span: call.span,
                    };
                    let kind = self.resolve_branch_access_kind(&access_expr, call.span)?;
                    return Ok(Expression::BranchAccess(access_expr.with_kind(kind)));
                }

                if let Some(func) = self.user_functions.get(&call.name) {
                    if Self::is_recognized_limited_exp_function(func) {
                        let args = call
                            .args
                            .iter()
                            .map(|a| self.lower_expression(a))
                            .collect::<CompileResult<Vec<_>>>()?;
                        return Ok(Expression::Call(CallExpr {
                            name: RSPICE_LIMITED_EXP_INTRINSIC.into(),
                            args,
                            span: call.span,
                        }));
                    }
                    if self.function_needs_materialization(func) {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(format!(
                                "analog function '{}': output/inout calls must be lowered at a statement boundary",
                                call.name
                            )),
                            call.span,
                        )));
                    }
                    self.inline_function(&call.name, &call.args, call.span)?
                } else {
                    let mut args = call
                        .args
                        .iter()
                        .map(|a| self.lower_expression(a))
                        .collect::<CompileResult<Vec<_>>>()?;
                    if call.name.eq_ignore_ascii_case("transition") {
                        self.materialize_transition_defaults(&mut args, call.span);
                    }
                    // Materialize the declaration-order-scoped default now.
                    // Hierarchy flattening may later combine modules authored
                    // under different directives into one analyzed module.
                    if is_zi_operator_name(&call.name) && args.len() == 4 {
                        args.push(Self::number_expr(
                            self.current_default_transition,
                            call.span,
                        ));
                    }
                    if let Some(noise) =
                        self.lower_noise_process_call(&call.name, &args, call.span)?
                    {
                        Expression::NoiseSource(noise)
                    } else {
                        Expression::Call(CallExpr {
                            name: call.name.clone(),
                            args,
                            span: call.span,
                        })
                    }
                }
            }
            Expression::ArrayAccess(a) => {
                let index = self.lower_expression(&a.index)?;
                let array_name = self.resolve_substituted_name(&a.array);
                if self.parameter_arrays.contains(&array_name) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "indexed access to parameter array '{}' is represented by its declaration metadata, but parameter-array element lowering is not implemented",
                            a.array
                        )),
                        a.span,
                    )));
                }
                let Some(layout) = self.arrays.get(&array_name).cloned() else {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "'{}' is indexed but is not a declared array variable",
                            a.array
                        )),
                        a.span,
                    )));
                };
                // Indexes that fold to instance-invariant constants (literals,
                // unrolled loop variables) resolve straight to the element
                // variable; everything else stays a runtime indexed access
                if let Some(k) = self.eval_const_invariant(&index) {
                    let k = k.round() as i64;
                    self.check_array_bounds(&array_name, &layout, k, a.span)?;
                    Expression::Identifier(Identifier {
                        name: SmolStr::from(format!("{array_name}[{k}]")),
                        span: a.span,
                    })
                } else {
                    Expression::ArrayAccess(ArrayAccessExpr {
                        array: array_name,
                        index: Box::new(index),
                        span: a.span,
                    })
                }
            }
            Expression::ArrayLiteral(a) => {
                if let Some(replication) = a.first_replication() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(
                            "replication in executable expressions is parsed but not yet supported; write the elements explicitly"
                                .into(),
                        ),
                        replication.span,
                    )));
                }
                let elements = a
                    .elements
                    .iter()
                    .map(|element| {
                        let ArrayLiteralElement::Value(expression) = element else {
                            unreachable!("replication was rejected before expression lowering");
                        };
                        self.lower_expression(expression)
                            .map(ArrayLiteralElement::Value)
                    })
                    .collect::<CompileResult<Vec<_>>>()?;
                Expression::ArrayLiteral(ArrayLiteralExpr {
                    elements,
                    assignment_pattern: a.assignment_pattern,
                    span: a.span,
                })
            }
            Expression::AnalogOperator(_) => expr.clone(),
            Expression::NoiseSource(noise) => {
                Expression::NoiseSource(self.lower_typed_noise_source(noise)?)
            }
        })
    }

    fn allocate_noise_process_id(&mut self, span: Span) -> CompileResult<u32> {
        if self.runtime_loop_depth != 0 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(
                    "noise sources in runtime-bounded loops do not have a defined finite process set; use a compile-time-bounded loop"
                        .into(),
                ),
                span,
            )));
        }
        let process_id = self.next_noise_process;
        self.next_noise_process = self.next_noise_process.checked_add(1).ok_or_else(|| {
            CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(
                    "module contains more than u32::MAX noise processes".into(),
                ),
                span,
            ))
        })?;
        Ok(process_id)
    }

    fn lower_noise_process_call(
        &mut self,
        name: &str,
        args: &[Expression],
        span: Span,
    ) -> CompileResult<Option<NoiseSource>> {
        let normalized = name.trim_start_matches('$').to_ascii_lowercase();
        let allowed = match normalized.as_str() {
            "white_noise" | "noise_table" | "noise_table_log" => 1..=2,
            "flicker_noise" => 2..=3,
            _ => return Ok(None),
        };
        if !allowed.contains(&args.len()) {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "'{name}' requires {} to {} arguments, found {}",
                    allowed.start(),
                    allowed.end(),
                    args.len()
                )),
                span,
            )));
        }
        let process_id = match normalized.as_str() {
            "white_noise" | "flicker_noise" | "noise_table" | "noise_table_log" => {
                self.allocate_noise_process_id(span)?
            }
            _ => unreachable!("recognized above"),
        };
        let label_at = |index: usize| match args.get(index) {
            Some(Expression::StringLit(label)) => Some(label.value.clone()),
            _ => None,
        };
        let source = match normalized.as_str() {
            "white_noise" => NoiseSource::White {
                process_id: Some(process_id),
                power: Box::new(args[0].clone()),
                name: label_at(1),
                span,
            },
            "flicker_noise" => NoiseSource::Flicker {
                process_id: Some(process_id),
                power: Box::new(args[0].clone()),
                exponent: Box::new(args[1].clone()),
                name: label_at(2),
                span,
            },
            "noise_table" | "noise_table_log" => {
                let data = match &args[0] {
                    Expression::ArrayLiteral(array) => array
                        .elements
                        .iter()
                        .map(|element| match element {
                            ArrayLiteralElement::Value(value) => Ok(value.clone()),
                            ArrayLiteralElement::Replication(replication) => Err(
                                CompileError::Semantic(SemanticError::new(
                                    SemanticErrorKind::UnsupportedFeature(
                                        "replication in a noise table is not supported; write each frequency/power pair explicitly"
                                            .into(),
                                    ),
                                    replication.span,
                                )),
                            ),
                        })
                        .collect::<CompileResult<Vec<_>>>()?,
                    value => vec![value.clone()],
                };
                NoiseSource::Table {
                    process_id: Some(process_id),
                    data,
                    log_interp: normalized == "noise_table_log",
                    name: label_at(1),
                    span,
                }
            }
            _ => unreachable!("recognized above"),
        };
        Ok(Some(source))
    }

    fn lower_typed_noise_source(&mut self, source: &NoiseSource) -> CompileResult<NoiseSource> {
        let process_id = match source {
            NoiseSource::White { process_id, .. }
            | NoiseSource::Flicker { process_id, .. }
            | NoiseSource::Table { process_id, .. } => match process_id {
                Some(process_id) => *process_id,
                None => self.allocate_noise_process_id(source.span())?,
            },
        };
        Ok(match source {
            NoiseSource::White {
                power, name, span, ..
            } => NoiseSource::White {
                process_id: Some(process_id),
                power: Box::new(self.lower_expression(power)?),
                name: name.clone(),
                span: *span,
            },
            NoiseSource::Flicker {
                power,
                exponent,
                name,
                span,
                ..
            } => NoiseSource::Flicker {
                process_id: Some(process_id),
                power: Box::new(self.lower_expression(power)?),
                exponent: Box::new(self.lower_expression(exponent)?),
                name: name.clone(),
                span: *span,
            },
            NoiseSource::Table {
                data,
                log_interp,
                name,
                span,
                ..
            } => NoiseSource::Table {
                process_id: Some(process_id),
                data: data
                    .iter()
                    .map(|value| self.lower_expression(value))
                    .collect::<CompileResult<Vec<_>>>()?,
                log_interp: *log_interp,
                name: name.clone(),
                span: *span,
            },
        })
    }

    fn validate_stateful_analog_operator_placement(
        &self,
        operator: &str,
        span: Span,
    ) -> CompileResult<()> {
        let context = match (
            self.inline_depth != 0,
            self.dynamic_analog_operator_guard_depth != 0,
        ) {
            (false, false) => return Ok(()),
            (true, true) => "inside a user-defined analog function under runtime control flow",
            (true, false) => "inside a user-defined analog function",
            (false, true) => "under runtime control flow",
        };
        Err(CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::InvalidAnalogOperator(format!(
                "'{operator}' analog operator must be evaluated on every Newton iteration and cannot appear {context} (VAMS-2023 section 4.5.15)"
            )),
            span,
        )))
    }

    fn validate_zi_definition_purity(&self, call: &CallExpr) -> CompileResult<()> {
        for (index, argument) in call.args.iter().enumerate() {
            if matches!(index, 1 | 2 | 3 | 5) {
                Self::validate_zi_freeze_expression(argument, &call.name, index)?;
            }
        }
        Ok(())
    }

    fn validate_zi_operand_budget(&self, call: &CallExpr) -> CompileResult<()> {
        let scalar_count = |expression: &Expression| match expression {
            Expression::NullArgument(_) => 0,
            Expression::ArrayLiteral(array) => array.elements.len(),
            _ => 1,
        };
        self.validate_zi_scalar_budget(
            &call.name,
            scalar_count(&call.args[1]),
            scalar_count(&call.args[2]),
            call.span,
        )
    }

    fn validate_zi_scalar_budget(
        &self,
        operator: &str,
        numerator: usize,
        denominator: usize,
        span: Span,
    ) -> CompileResult<()> {
        crate::zfilter::validate_zi_runtime_operand_budget(operator, numerator, denominator)
            .map(|_| ())
            .map_err(|error| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(error.to_string()),
                    span,
                ))
            })
    }

    fn validate_zi_freeze_expression(
        expression: &Expression,
        operator: &str,
        argument_index: usize,
    ) -> CompileResult<()> {
        let reject = |detail: String, span: Span| {
            Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "{operator} constant argument {} must be side-effect-free and deterministic for analysis-start freezing: {detail}",
                    argument_index + 1
                )),
                span,
            )))
        };
        match expression {
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_) => Ok(()),
            Expression::Digital(digital) => reject(
                format!("a {} is not a continuous-domain value", digital.construct()),
                digital.span(),
            ),
            Expression::Binary(binary) => {
                Self::validate_zi_freeze_expression(&binary.left, operator, argument_index)?;
                Self::validate_zi_freeze_expression(&binary.right, operator, argument_index)
            }
            Expression::Unary(unary) => {
                Self::validate_zi_freeze_expression(&unary.operand, operator, argument_index)
            }
            Expression::Conditional(conditional) => {
                Self::validate_zi_freeze_expression(
                    &conditional.condition,
                    operator,
                    argument_index,
                )?;
                Self::validate_zi_freeze_expression(
                    &conditional.then_expr,
                    operator,
                    argument_index,
                )?;
                Self::validate_zi_freeze_expression(
                    &conditional.else_expr,
                    operator,
                    argument_index,
                )
            }
            Expression::ArrayAccess(access) => {
                Self::validate_zi_freeze_expression(&access.index, operator, argument_index)
            }
            Expression::ArrayLiteral(array) => {
                for element in &array.elements {
                    Self::validate_zi_freeze_array_element(element, operator, argument_index)?;
                }
                Ok(())
            }
            Expression::SystemFunction(function) => {
                let normalized = function.name.to_ascii_lowercase();
                if normalized.contains("random")
                    || normalized.starts_with("$dist_")
                    || normalized.starts_with("$rdist_")
                    || normalized == "$limit"
                {
                    return reject(
                        format!(
                            "system function '{}' is stateful or nondeterministic",
                            function.name
                        ),
                        function.span,
                    );
                }
                for argument in &function.args {
                    Self::validate_zi_freeze_expression(argument, operator, argument_index)?;
                }
                Ok(())
            }
            Expression::Call(nested) => {
                let normalized = nested.name.to_ascii_lowercase();
                if matches!(
                    normalized.as_str(),
                    "ddt"
                        | "idt"
                        | "idtmod"
                        | "transition"
                        | "slew"
                        | "absdelay"
                        | "cross"
                        | "last_crossing"
                        | "above"
                        | "timer"
                        | "laplace_zp"
                        | "laplace_zd"
                        | "laplace_np"
                        | "laplace_nd"
                        | "zi_zp"
                        | "zi_zd"
                        | "zi_np"
                        | "zi_nd"
                        | "white_noise"
                        | "flicker_noise"
                        | "noise_table"
                ) {
                    return reject(
                        format!(
                            "nested operator '{}' is stateful or nondeterministic",
                            nested.name
                        ),
                        nested.span,
                    );
                }
                for argument in &nested.args {
                    Self::validate_zi_freeze_expression(argument, operator, argument_index)?;
                }
                Ok(())
            }
            Expression::AnalogOperator(analog) => reject(
                format!("nested analog operator at {:?} is stateful", analog.span()),
                analog.span(),
            ),
            Expression::NoiseSource(noise) => reject(
                "a noise source is nondeterministic in this context".into(),
                noise.span(),
            ),
        }
    }

    fn validate_zi_freeze_array_element(
        element: &ArrayLiteralElement,
        operator: &str,
        argument_index: usize,
    ) -> CompileResult<()> {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::validate_zi_freeze_expression(expression, operator, argument_index)
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::validate_zi_freeze_expression(&replication.count, operator, argument_index)?;
                for element in &replication.elements {
                    Self::validate_zi_freeze_array_element(element, operator, argument_index)?;
                }
                Ok(())
            }
        }
    }

    /// Validate the selector and callable contract of named `$limit` forms.
    ///
    /// Numeric `$limit(value)` and `$limit(value, step)` calls intentionally
    /// remain outside this check. A named limiter receives the proposed and
    /// previous values implicitly, followed by every argument after its
    /// selector in the `$limit` call.
    fn validate_limit_call(&self, function: &SystemFunction) -> CompileResult<()> {
        if function.name != "$limit" {
            return Ok(());
        }

        let named_function = matches!(function.args.get(1), Some(Expression::Identifier(id)) if self.user_functions.contains_key(&id.name));
        let selector = match function.args.get(1) {
            Some(Expression::StringLit(selector)) => selector.value.as_str(),
            Some(Expression::Identifier(selector)) if named_function => selector.name.as_str(),
            _ if function.args.len() <= 2 => return Ok(()),
            _ => {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(
                        "named $limit requires a function identifier or literal string selector as its second argument"
                            .into(),
                    ),
                    function.span,
                )));
            }
        };

        // Xyce's named limiter ABI has untyped and typed selector families.
        // Typed built-ins carry one additional type/polarity argument; dummy
        // selectors retain the same shape for initialization bookkeeping even
        // though they intentionally leave the proposed value unchanged.
        if let Some(expected) = Self::builtin_limit_arity(selector).filter(|_| !named_function) {
            if function.args.len() != expected {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::ArgumentCountMismatch {
                        name: format!("$limit(\"{selector}\")"),
                        expected: expected.to_string(),
                        got: function.args.len(),
                    },
                    function.span,
                )));
            }
            return Ok(());
        }

        let Some(limiter) = self.user_functions.get(selector) else {
            // Unknown string recommendations use the simulator's default
            // algorithm (VAMS-2023 9.17.3). An identifier must name a function.
            if matches!(function.args.get(1), Some(Expression::StringLit(_))) {
                return Ok(());
            }
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnknownFunction(selector.to_string()),
                function.span,
            )));
        };

        if !matches!(limiter.return_type, VarType::Real | VarType::Integer) {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "named $limit function '{selector}' must return a numeric value"
                )),
                function.span,
            )));
        }

        // The source function's first two inputs receive the proposed and
        // previous values. In Xyce's typed custom-limiter convention, the
        // literal `"typed"` and the following type/polarity expression are
        // metadata and are not forwarded to the analog function.
        let typed_custom = !named_function
            && function.args.get(2).is_some_and(|argument| {
                matches!(
                    argument,
                    Expression::StringLit(marker) if marker.value == "typed"
                )
            });
        if typed_custom && function.args.len() < 4 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "typed named $limit function '{selector}' requires a type/polarity metadata argument after \"typed\""
                )),
                function.span,
            )));
        }
        let expected_formals = if typed_custom {
            function.args.len() - 2
        } else {
            function.args.len()
        };
        if limiter.params.len() != expected_formals {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: selector.to_string(),
                    expected: expected_formals.to_string(),
                    got: limiter.params.len(),
                },
                function.span,
            )));
        }
        if let Some(param) = limiter
            .params
            .iter()
            .find(|param| param.direction != ParamDirection::Input)
        {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "named $limit function '{selector}' requires input formal '{}', found {}",
                    param.name,
                    match param.direction {
                        ParamDirection::Input => unreachable!(),
                        ParamDirection::Output => "output",
                        ParamDirection::Inout => "inout",
                    }
                )),
                function.span,
            )));
        }
        if let Some(param) = limiter
            .params
            .iter()
            .find(|param| !matches!(param.param_type, VarType::Real | VarType::Integer))
        {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "named $limit function '{selector}' requires numeric formal '{}', found {:?}",
                    param.name, param.param_type
                )),
                function.span,
            )));
        }

        Ok(())
    }

    fn uses_default_limit_recommendation(&self, function: &SystemFunction) -> bool {
        function.name == "$limit"
            && matches!(function.args.get(1), Some(Expression::StringLit(selector))
                if Self::builtin_limit_arity(&selector.value).is_none()
                    && !self.user_functions.contains_key(&selector.value))
    }

    /// Built-in string recommendations retained for Xyce model compatibility.
    fn builtin_limit_arity(selector: &str) -> Option<usize> {
        match selector {
            "pnjlim" | "pnjlim_new" | "dummy" => Some(4),
            "typedpnjlim" | "typedpnjlim_new" | "typeddummy" => Some(5),
            _ => None,
        }
    }

    /// Lower a callback into a stateful operator with implicit proposed and
    /// previous values. Quoted custom selectors additionally accept Xyce's
    /// `"typed"` marker and polarity expression as metadata, not formals.
    fn lower_custom_limit_call(
        &mut self,
        function: &SystemFunction,
    ) -> CompileResult<Option<Expression>> {
        if function.name != "$limit" {
            return Ok(None);
        }
        let (selector, named_function) = match function.args.get(1) {
            Some(Expression::StringLit(literal)) => (literal.value.clone(), false),
            Some(Expression::Identifier(id)) if self.user_functions.contains_key(&id.name) => {
                (id.name.clone(), true)
            }
            _ => return Ok(None),
        };
        if !named_function && Self::builtin_limit_arity(&selector).is_some() {
            return Ok(None);
        }
        if !self.user_functions.contains_key(&selector) {
            // An unknown string recommendation selects the default algorithm.
            return Ok(None);
        }

        let typed = !named_function && function.args.get(2).is_some_and(
            |argument| matches!(argument, Expression::StringLit(marker) if marker.value == "typed"),
        );
        let proposed = self.lower_expression(
            function
                .args
                .first()
                .expect("validated custom $limit has a proposed value"),
        )?;
        let type_metadata = typed
            .then(|| self.lower_expression(&function.args[3]))
            .transpose()?
            .map(Box::new);
        let implicit_proposed = Expression::AnalogOperator(AnalogOperator::LimiterArgument {
            argument: LimiterArgument::Proposed,
            span: function.span,
        });
        let implicit_previous = Expression::AnalogOperator(AnalogOperator::LimiterArgument {
            argument: LimiterArgument::Previous,
            span: function.span,
        });
        let forwarded_start = if typed { 4 } else { 2 };
        let mut limiter_args = Vec::with_capacity(2 + function.args.len() - forwarded_start);
        limiter_args.extend([implicit_proposed, implicit_previous]);
        limiter_args.extend(function.args[forwarded_start..].iter().cloned());
        let candidate = self.inline_function(&selector, &limiter_args, function.span)?;

        Ok(Some(Expression::AnalogOperator(AnalogOperator::Limit {
            proposed: Box::new(proposed),
            candidate: Box::new(candidate),
            type_metadata,
            selector,
            span: function.span,
        })))
    }

    fn validate_builtin_call_arity(&self, call: &CallExpr) -> CompileResult<()> {
        let Some(signature) = self.functions.get(&call.name) else {
            return Ok(());
        };

        let min_args = signature.min_args();
        let max_args = signature.max_args();
        let got = call.args.len();
        if got < min_args || got > max_args {
            let expected = if min_args == max_args {
                min_args.to_string()
            } else {
                format!("{min_args}..{max_args}")
            };
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: call.name.to_string(),
                    expected,
                    got,
                },
                call.span,
            )));
        }

        Ok(())
    }

    fn validate_null_arguments(&self, call: &CallExpr) -> CompileResult<()> {
        let normalized = call.name.to_ascii_lowercase();
        for (index, argument) in call.args.iter().enumerate() {
            if !matches!(argument, Expression::NullArgument(_)) {
                continue;
            }
            let authorized = match normalized.as_str() {
                "cross" => (1..=4).contains(&index),
                "above" => (1..=3).contains(&index),
                "zi_zp" | "zi_zd" | "laplace_zp" | "laplace_zd" => index == 1,
                _ => false,
            };
            if !authorized {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(format!(
                        "{} argument {} may not be null at this position",
                        call.name,
                        index + 1
                    )),
                    argument.span(),
                )));
            }
        }
        let defined = |index: usize| {
            call.args
                .get(index)
                .is_some_and(|argument| !matches!(argument, Expression::NullArgument(_)))
        };
        let dependency_error = |message: &str| {
            CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(message.to_string()),
                call.span,
            ))
        };
        match normalized.as_str() {
            "cross" => {
                if (defined(2) || defined(3)) && !defined(1) {
                    return Err(dependency_error(
                        "cross tolerances require a defined direction",
                    ));
                }
                if defined(3) && !defined(2) {
                    return Err(dependency_error("cross expr_tol requires time_tol"));
                }
            }
            "above" if defined(2) && !defined(1) => {
                return Err(dependency_error("above expr_tol requires time_tol"));
            }
            _ => {}
        }
        Ok(())
    }

    /// Enforce the Verilog-AMS array-value grammar used by the Laplace and Zi
    /// operators. An ordinary brace concatenation is a packed value, not an
    /// unpacked coefficient/root vector, and a bare scalar cannot be silently
    /// promoted to a one-element vector.
    fn validate_filter_vector_operands(&self, call: &CallExpr) -> CompileResult<()> {
        let normalized = call.name.to_ascii_lowercase();
        let roles = match normalized.as_str() {
            "laplace_zp" | "zi_zp" => ("zeros", "poles", true),
            "laplace_zd" | "zi_zd" => ("zeros", "denominator", true),
            "laplace_np" | "zi_np" => ("numerator", "poles", false),
            "laplace_nd" | "zi_nd" => ("numerator", "denominator", false),
            _ => return Ok(()),
        };

        for (index, role, allow_null) in [(1, roles.0, roles.2), (2, roles.1, false)] {
            let Some(argument) = call.args.get(index) else {
                // The arity validator owns the missing-argument diagnostic.
                continue;
            };
            match argument {
                Expression::NullArgument(_) if allow_null => {}
                Expression::ArrayLiteral(array) if array.assignment_pattern => {}
                Expression::ArrayLiteral(array) => {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(format!(
                            "{} {role} vector must be an assignment pattern opened with `'{{` or an array identifier; ordinary concatenation `{{...}}` is not a Verilog-AMS array value",
                            call.name
                        )),
                        array.span,
                    )));
                }
                Expression::Identifier(identifier)
                    if self.parameter_arrays.contains(&identifier.name)
                        || self.arrays.contains_key(&identifier.name) =>
                {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "{} {role} array identifier '{}' is valid Verilog-AMS syntax, but executable filter array operands are not implemented yet",
                            call.name, identifier.name
                        )),
                        identifier.span,
                    )));
                }
                other => {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(format!(
                            "{} {role} operand must be an assignment pattern opened with `'{{` or an array identifier; a scalar expression is not a filter vector",
                            call.name
                        )),
                        other.span(),
                    )));
                }
            }
        }
        Ok(())
    }

    fn materialize_filter_call_replication(&self, call: &CallExpr) -> CompileResult<CallExpr> {
        let normalized = call.name.to_ascii_lowercase();
        let roles = match normalized.as_str() {
            "laplace_zp" | "zi_zp" => ("zeros", "poles"),
            "laplace_zd" | "zi_zd" => ("zeros", "denominator"),
            "laplace_np" | "zi_np" => ("numerator", "poles"),
            "laplace_nd" | "zi_nd" => ("numerator", "denominator"),
            _ => return Ok(call.clone()),
        };

        let mut materialized = call.clone();
        for (index, role) in [(1, roles.0), (2, roles.1)] {
            let Some(argument) = materialized.args.get_mut(index) else {
                continue;
            };
            if !matches!(argument, Expression::ArrayLiteral(_)) {
                continue;
            }
            *argument = self.materialize_replication_expression(
                argument,
                MAX_ANALOG_FILTER_VECTOR_ELEMENTS,
                MAX_ANALOG_FILTER_VECTOR_ELEMENTS.saturating_mul(4),
                &format!("{} {role} vector", call.name),
                true,
            )?;
        }
        Ok(materialized)
    }

    const MAX_INLINE_DEPTH: usize = 16;

    /// Inline a call to a user-defined analog function by symbolically
    /// executing its body. The return value is the final expression bound
    /// to the function-name variable.
    ///
    /// # This is guard flattening, and it is the backend rebuild's last one
    ///
    /// A function body's control flow is dissolved here exactly as the analog
    /// block's used to be: each assignment under a guard becomes
    /// `guard ? value : previous`, where `previous` is the *entire* expression
    /// tree built so far. A chain of `n` assignments under nested conditions
    /// therefore duplicates its predecessor at every step, and the tree grows
    /// multiplicatively rather than additively.
    ///
    /// Measured on `EPFL_HEMT_10a`: its analog block holds **6 conditionals and
    /// 191 assignments**, and the front end turns them into **186,444 HIR
    /// expressions** — roughly a thousand nodes per assignment — because its
    /// `core` function nests three arms over five chained locals and is called
    /// from several places. Lowering those `?:` trees produces 8,248 CFG blocks
    /// and 6.3 MB of Rust against 78 KB from the tier being replaced. It is the
    /// single largest obstacle to the plan's size gate.
    ///
    /// The fix is the one Phase 1 applied a level up: materialise a call whose
    /// body has control flow at a statement boundary and inline the body as
    /// statements, so one `if` in a function becomes one
    /// [`AnalyzedRegion::Conditional`] rather than a duplicated expression. The
    /// machinery for statement-boundary materialisation already exists here for
    /// output/inout arguments. See `design/VERILOGA_BACKEND_PLAN.md`.
    fn inline_function(
        &mut self,
        name: &SmolStr,
        args: &[Expression],
        span: Span,
    ) -> CompileResult<Expression> {
        if self.inline_depth >= Self::MAX_INLINE_DEPTH {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::CircularDependency(format!(
                    "analog function '{}' (recursive call chain?)",
                    name
                )),
                span,
            )));
        }

        let func = self
            .user_functions
            .get(name)
            .cloned()
            .expect("checked by caller");

        if args.len() != func.params.len() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: name.to_string(),
                    expected: func.params.len().to_string(),
                    got: args.len(),
                },
                span,
            )));
        }

        // Bind parameters and locals in a fresh substitution frame
        let mut frame = HashMap::new();
        let mut variable_types = HashMap::new();
        let mut output_bindings = Vec::new();
        for (param, arg) in func.params.iter().zip(args.iter()) {
            let target_type = Self::value_type_for_var_type(param.param_type);
            variable_types.insert(param.name.clone(), target_type);
            match param.direction {
                ParamDirection::Input => {
                    let value = self.lower_expression(arg)?;
                    frame.insert(
                        param.name.clone(),
                        self.coerce_assignment_expression(value, target_type)?.0,
                    );
                }
                ParamDirection::Output => {
                    let target = self.function_output_lvalue(name, param, arg)?;
                    frame.insert(param.name.clone(), Self::number_expr(0.0, param.span));
                    output_bindings.push((param.name.clone(), target, param.span));
                }
                ParamDirection::Inout => {
                    let target = self.function_output_lvalue(name, param, arg)?;
                    let value = self.lower_expression(arg)?;
                    frame.insert(
                        param.name.clone(),
                        self.coerce_assignment_expression(value, target_type)?.0,
                    );
                    output_bindings.push((param.name.clone(), target, param.span));
                }
            }
        }
        for var_decl in &func.locals {
            for item in &var_decl.items {
                if func.params.iter().any(|p| p.name == item.name) {
                    continue;
                }
                if !item.dimensions.is_empty() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "array local '{}' in analog function '{}'",
                            item.name, name
                        )),
                        item.span,
                    )));
                }
                variable_types.insert(
                    item.name.clone(),
                    Self::value_type_for_var_type(var_decl.var_type),
                );
                frame.insert(item.name.clone(), Self::number_expr(0.0, item.span));
            }
        }
        // The return value accumulates in a variable named after the function
        frame.insert(func.name.clone(), Self::number_expr(0.0, span));
        variable_types.insert(
            func.name.clone(),
            Self::value_type_for_var_type(func.return_type),
        );

        self.subst_stack.push(frame);
        self.inline_depth += 1;
        let result = (|| {
            // Bind initializers once, in declaration order, after the formal
            // arguments are visible. Keeping raw initializer syntax in the
            // frame would make it read later writes when a local is used.
            for declaration in &func.locals {
                for item in &declaration.items {
                    if func.params.iter().any(|param| param.name == item.name) {
                        continue;
                    }
                    if let Some(initializer) = &item.init {
                        let value = self.lower_expression_without_side_effects(
                            initializer,
                            "analog function initializer",
                        )?;
                        let value = self
                            .coerce_assignment_expression(value, variable_types[&item.name])?
                            .0;
                        self.subst_stack
                            .last_mut()
                            .expect("function frame")
                            .insert(item.name.clone(), value);
                    }
                }
            }
            self.exec_function_body(&func.body.statements, None, &variable_types)
        })();
        self.inline_depth -= 1;
        let frame = self.subst_stack.pop().expect("pushed above");
        result?;

        for (formal, target, span) in output_bindings {
            let value = frame
                .get(&formal)
                .cloned()
                .unwrap_or_else(|| Self::number_expr(0.0, span));
            self.function_side_effects.push(AssignmentStmt {
                target,
                value,
                span,
            });
        }

        Ok(frame
            .get(&func.name)
            .cloned()
            .unwrap_or_else(|| Self::number_expr(0.0, span)))
    }

    fn function_output_lvalue(
        &mut self,
        function_name: &SmolStr,
        param: &FunctionParam,
        arg: &Expression,
    ) -> CompileResult<LValue> {
        match arg {
            Expression::Identifier(id) => {
                let resolved = self.resolve_substituted_name(&id.name);
                Ok(LValue::Variable {
                    name: resolved,
                    span: id.span,
                })
            }
            Expression::ArrayAccess(access) => {
                let array = self.resolve_substituted_name(&access.array);
                let index = self.lower_expression_without_side_effects(
                    &access.index,
                    "analog function output argument index",
                )?;
                Ok(LValue::ArrayAccess {
                    name: array,
                    index: Box::new(index),
                    span: access.span,
                })
            }
            _ => Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "analog function '{}': {} argument '{}' must be an assignable variable",
                    function_name,
                    match param.direction {
                        ParamDirection::Output => "output",
                        ParamDirection::Inout => "inout",
                        ParamDirection::Input => "input",
                    },
                    param.name
                )),
                arg.span(),
            ))),
        }
    }

    /// Symbolically execute function-body statements, updating the topmost
    /// substitution frame.
    fn exec_function_body(
        &mut self,
        statements: &[AnalogStatement],
        guard: Option<&Expression>,
        variable_types: &HashMap<SmolStr, ValueType>,
    ) -> CompileResult<()> {
        for stmt in statements {
            self.exec_function_statement(stmt, guard, variable_types)?;
        }
        Ok(())
    }

    fn exec_function_statement(
        &mut self,
        stmt: &AnalogStatement,
        guard: Option<&Expression>,
        variable_types: &HashMap<SmolStr, ValueType>,
    ) -> CompileResult<()> {
        match stmt {
            AnalogStatement::Assignment(assign) => {
                let LValue::Variable { name, span } = &assign.target else {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(
                            "array assignment inside analog functions is not supported".into(),
                        ),
                        assign.span,
                    )));
                };
                let value = self
                    .lower_expression_without_side_effects(&assign.value, "analog function body")?;
                let value = self
                    .coerce_assignment_expression(
                        value,
                        variable_types
                            .get(name)
                            .copied()
                            .unwrap_or(ValueType::Unknown),
                    )?
                    .0;
                let prev = self
                    .lookup_substitution(name)
                    .unwrap_or_else(|| Self::number_expr(0.0, *span));
                let new_value = match guard {
                    Some(g) => Expression::Conditional(ConditionalExpr {
                        condition: Box::new(g.clone()),
                        then_expr: Box::new(value),
                        else_expr: Box::new(prev),
                        span: *span,
                    }),
                    None => value,
                };
                self.subst_stack
                    .last_mut()
                    .expect("function frame")
                    .insert(name.clone(), new_value);
            }
            AnalogStatement::Conditional(cond) => {
                let condition = self.lower_expression_without_side_effects(
                    &cond.condition,
                    "analog function body",
                )?;
                let dynamic_condition = !self.expression_is_simulation_invariant(&condition);
                let then_guard = match guard {
                    Some(g) => Self::binary_expr(BinaryOp::And, g.clone(), condition.clone()),
                    None => condition.clone(),
                };
                if dynamic_condition {
                    self.dynamic_analog_operator_guard_depth += 1;
                }
                let then_result = self.exec_function_statement(
                    &cond.then_branch,
                    Some(&then_guard),
                    variable_types,
                );
                if dynamic_condition {
                    self.dynamic_analog_operator_guard_depth -= 1;
                }
                then_result?;
                if let Some(else_branch) = &cond.else_branch {
                    let not_cond = Self::not_expr(condition);
                    let else_guard = match guard {
                        Some(g) => Self::binary_expr(BinaryOp::And, g.clone(), not_cond),
                        None => not_cond,
                    };
                    if dynamic_condition {
                        self.dynamic_analog_operator_guard_depth += 1;
                    }
                    let else_result = self.exec_function_statement(
                        else_branch,
                        Some(&else_guard),
                        variable_types,
                    );
                    if dynamic_condition {
                        self.dynamic_analog_operator_guard_depth -= 1;
                    }
                    else_result?;
                }
            }
            AnalogStatement::Case(case_stmt) => {
                let selector = self.lower_expression_without_side_effects(
                    &case_stmt.expr,
                    "analog function body",
                )?;
                let mut prior_match: Option<Expression> = None;
                for item in &case_stmt.items {
                    let mut item_match: Option<Expression> = None;
                    for m in &item.matches {
                        let m_lowered =
                            self.lower_expression_without_side_effects(m, "analog function body")?;
                        let eq = Self::binary_expr(BinaryOp::Eq, selector.clone(), m_lowered);
                        item_match = Some(match item_match {
                            Some(acc) => Self::binary_expr(BinaryOp::Or, acc, eq),
                            None => eq,
                        });
                    }
                    let Some(item_match) = item_match else {
                        continue;
                    };
                    let mut item_guard = match &prior_match {
                        Some(prior) => Self::binary_expr(
                            BinaryOp::And,
                            item_match.clone(),
                            Self::not_expr(prior.clone()),
                        ),
                        None => item_match.clone(),
                    };
                    if let Some(g) = guard {
                        item_guard = Self::binary_expr(BinaryOp::And, g.clone(), item_guard);
                    }
                    self.exec_function_statement(
                        &item.statement,
                        Some(&item_guard),
                        variable_types,
                    )?;
                    prior_match = Some(match prior_match {
                        Some(prior) => Self::binary_expr(BinaryOp::Or, prior, item_match),
                        None => item_match,
                    });
                }
                if let Some(default) = &case_stmt.default {
                    let default_guard = match (guard, prior_match) {
                        (Some(g), Some(prior)) => Some(Self::binary_expr(
                            BinaryOp::And,
                            g.clone(),
                            Self::not_expr(prior),
                        )),
                        (None, Some(prior)) => Some(Self::not_expr(prior)),
                        (Some(g), None) => Some(g.clone()),
                        (None, None) => None,
                    };
                    self.exec_function_statement(default, default_guard.as_ref(), variable_types)?;
                }
            }
            AnalogStatement::Block(block) => {
                let mut scope_types = variable_types.clone();
                let mut shadowed = Vec::new();
                let result = (|| {
                    for var_decl in &block.variables {
                        for item in &var_decl.items {
                            if !item.dimensions.is_empty() {
                                return Err(CompileError::Semantic(SemanticError::new(
                                    SemanticErrorKind::UnsupportedFeature(format!(
                                        "array local '{}' in analog function",
                                        item.name
                                    )),
                                    item.span,
                                )));
                            }
                            let target_type = Self::value_type_for_var_type(var_decl.var_type);
                            scope_types.insert(item.name.clone(), target_type);
                            let previous = self
                                .subst_stack
                                .last_mut()
                                .expect("function frame")
                                .insert(item.name.clone(), Self::number_expr(0.0, item.span));
                            shadowed.push((item.name.clone(), previous));
                            let init = match &item.init {
                                Some(init) => self.lower_expression_without_side_effects(
                                    init,
                                    "analog function body",
                                )?,
                                None => Self::number_expr(0.0, item.span),
                            };
                            let init = self.coerce_assignment_expression(init, target_type)?.0;
                            self.subst_stack
                                .last_mut()
                                .expect("function frame")
                                .insert(item.name.clone(), init);
                        }
                    }
                    self.exec_function_body(&block.statements, guard, &scope_types)
                })();
                let frame = self.subst_stack.last_mut().expect("function frame");
                for (name, previous) in shadowed.into_iter().rev() {
                    match previous {
                        Some(value) => {
                            frame.insert(name, value);
                        }
                        None => {
                            frame.remove(&name);
                        }
                    }
                }
                result?;
            }
            AnalogStatement::Null(_) => {}
            AnalogStatement::Call(call) => self.validate_no_effect_system_task(call)?,
            other => {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(format!(
                        "statement not supported inside analog functions: {:?}",
                        std::mem::discriminant(other)
                    )),
                    Span::dummy(),
                )));
            }
        }
        Ok(())
    }

    fn is_no_effect_system_task(name: &str) -> bool {
        matches!(
            name,
            "$display" | "$error" | "$info" | "$monitor" | "$strobe" | "$warning" | "$write"
        )
    }

    /// Report one no-effect system task the analyzer is about to drop.
    ///
    /// The task is legal Verilog-A and parses, but this compiler emits device
    /// equations rather than a simulation program, so the call contributes
    /// nothing. Silence would leave the author believing the model prints.
    fn warn_no_effect_system_task(&mut self, call: &CallStmt) {
        self.warn(
            NO_EFFECT_SYSTEM_TASK_CODE,
            format!(
                "System task '{}' is parsed and discarded: it has no effect on the device equations and writes nothing at run time.",
                call.name
            ),
            call.span,
        );
    }

    fn validate_no_effect_system_task(&mut self, call: &CallStmt) -> CompileResult<()> {
        if Self::is_no_effect_system_task(call.name.as_str()) {
            self.warn_no_effect_system_task(call);
            return Ok(());
        }

        if matches!(call.name.as_str(), "$bound_step" | "$discontinuity") {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "system task '{}' is not supported inside analog functions",
                    call.name
                )),
                call.span,
            )));
        }

        Err(Self::unknown_system_task_error(call))
    }

    fn validate_system_task_arity(
        &self,
        call: &CallStmt,
        min_args: usize,
        max_args: Option<usize>,
    ) -> CompileResult<()> {
        let got = call.args.len();
        let too_many = max_args.is_some_and(|max| got > max);
        if got >= min_args && !too_many {
            return Ok(());
        }

        let expected = match max_args {
            Some(max) if min_args == max => min_args.to_string(),
            Some(max) => format!("{min_args}..{max}"),
            None => format!("{min_args}+"),
        };
        Err(CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::ArgumentCountMismatch {
                name: call.name.to_string(),
                expected,
                got,
            },
            call.span,
        )))
    }

    fn unknown_system_task_error(call: &CallStmt) -> CompileError {
        CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::UnknownFunction(call.name.to_string()),
            call.span,
        ))
    }

    fn validate_node(&self, name: &str, span: Span) -> CompileResult<()> {
        if is_global_ground_name(name) {
            return Ok(());
        }
        if let Some(sym) = self.symbols.lookup(name) {
            match sym.kind {
                SymbolKind::Port | SymbolKind::Node | SymbolKind::Branch => Ok(()),
                _ => Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidNodeReference {
                        name: name.into(),
                        kind: format!("{:?}", sym.kind),
                    },
                    span,
                ))),
            }
        } else {
            Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndeclaredSymbol { name: name.into() },
                span,
            )))
        }
    }

    fn infer_type(&self, expr: &Expression) -> CompileResult<ValueType> {
        if !matches!(
            expr,
            Expression::Binary(_) | Expression::Unary(_) | Expression::Conditional(_)
        ) {
            return self.infer_non_operator_type(expr);
        }
        let mut pending = vec![(expr, false)];
        let mut types = Vec::new();
        while let Some((expression, children_typed)) = pending.pop() {
            let value_type = match expression {
                Expression::Binary(binary) if !children_typed => {
                    pending.push((expression, true));
                    pending.push((&binary.right, false));
                    pending.push((&binary.left, false));
                    continue;
                }
                Expression::Unary(unary) if !children_typed => {
                    pending.push((expression, true));
                    pending.push((&unary.operand, false));
                    continue;
                }
                Expression::Conditional(conditional) if !children_typed => {
                    pending.push((expression, true));
                    pending.push((&conditional.else_expr, false));
                    pending.push((&conditional.then_expr, false));
                    continue;
                }
                Expression::Binary(binary) => {
                    let right = types.pop().expect("right operand type was inferred");
                    let left: ValueType = types.pop().expect("left operand type was inferred");
                    match binary.op {
                        BinaryOp::CheckedValue => ValueType::Real,
                        BinaryOp::IntAdd
                        | BinaryOp::IntSub
                        | BinaryOp::IntMul
                        | BinaryOp::IntDiv
                        | BinaryOp::IntMod
                        | BinaryOp::IntPow => ValueType::Integer,
                        BinaryOp::Eq
                        | BinaryOp::Ne
                        | BinaryOp::Lt
                        | BinaryOp::Le
                        | BinaryOp::Gt
                        | BinaryOp::Ge
                        | BinaryOp::And
                        | BinaryOp::Or => ValueType::Boolean,
                        BinaryOp::Add
                        | BinaryOp::Sub
                        | BinaryOp::Mul
                        | BinaryOp::Div
                        | BinaryOp::Pow
                        | BinaryOp::Mod => left.common_type(right),
                        BinaryOp::BitAnd
                        | BinaryOp::BitOr
                        | BinaryOp::BitXor
                        | BinaryOp::Shl
                        | BinaryOp::Shr => {
                            Self::validate_integer_operator_operand(
                                left,
                                "left operand of bitwise or shift operator",
                                binary.left.span(),
                            )?;
                            Self::validate_integer_operator_operand(
                                right,
                                "right operand of bitwise or shift operator",
                                binary.right.span(),
                            )?;
                            ValueType::Integer
                        }
                    }
                }
                Expression::Unary(unary) => {
                    let operand_type = types.pop().expect("unary operand type was inferred");
                    match unary.op {
                        UnaryOp::ToInteger => ValueType::Integer,
                        UnaryOp::Pos | UnaryOp::Neg => operand_type,
                        UnaryOp::Not => ValueType::Boolean,
                        UnaryOp::BitNot => {
                            Self::validate_integer_operator_operand(
                                operand_type,
                                "operand of bitwise complement",
                                unary.operand.span(),
                            )?;
                            ValueType::Integer
                        }
                    }
                }
                Expression::Conditional(_) => {
                    let else_type = types.pop().expect("else branch type was inferred");
                    let then_type: ValueType = types.pop().expect("then branch type was inferred");
                    then_type.common_type(else_type)
                }
                expression => self.infer_non_operator_type(expression)?,
            };
            types.push(value_type);
        }
        debug_assert_eq!(types.len(), 1);
        Ok(types.pop().expect("root expression type was inferred"))
    }

    fn infer_non_operator_type(&self, expr: &Expression) -> CompileResult<ValueType> {
        match expr {
            Expression::Digital(digital) => Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "a {} has no continuous-domain type",
                    digital.construct()
                )),
                digital.span(),
            ))),
            Expression::Number(number) => Ok(if Self::integer_literal_value(number).is_some() {
                ValueType::Integer
            } else {
                ValueType::Real
            }),
            Expression::StringLit(_) => Ok(ValueType::String),
            Expression::NullArgument(span) => Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidExpression(
                    "a null argument is legal only in the zero-vector position of zi_zp, zi_zd, laplace_zp, or laplace_zd"
                        .into(),
                ),
                *span,
            ))),
            Expression::Identifier(ident) => {
                if self.parameter_arrays.contains(&ident.name) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "parameter array '{}' cannot be used as a scalar expression",
                            ident.name
                        )),
                        ident.span,
                    )));
                }
                if let Some(sym) = self.symbols.lookup(&ident.name) {
                    Ok(sym.value_type)
                } else if ident.name == "inf" {
                    // Keep arithmetic on an infinite range endpoint real.
                    // Unknown would adopt the integer type of `2` in `2*inf`.
                    Ok(ValueType::Real)
                } else if let Some((base, _)) = ident.name.split_once('[')
                    && self.arrays.contains_key(base)
                    && let Some(sym) = self.symbols.lookup(base)
                {
                    // Array element slot (`arr[k]`): typed like the array
                    Ok(sym.value_type)
                } else {
                    Ok(ValueType::Unknown)
                }
            }
            Expression::BranchAccess(_) => Ok(ValueType::NatureAccess),
            Expression::SystemFunction(_) => Ok(ValueType::Real),
            // `analysis` has variadic string arguments and is validated outside
            // the fixed-arity function registry. Its result is still numeric
            // when used directly as a task argument or an integer operand.
            Expression::Call(call) if call.name == "analysis" => Ok(ValueType::Integer),
            Expression::Call(call) => {
                if let Some(sig) = self.functions.get(&call.name) {
                    Ok(sig.return_type)
                } else {
                    Ok(ValueType::Unknown)
                }
            }
            Expression::Unary(_) | Expression::Binary(_) | Expression::Conditional(_) => {
                unreachable!("operator types use the iterative inference path")
            }
            Expression::ArrayAccess(a) => {
                if let Some(sym) = self.symbols.lookup(&a.array) {
                    Ok(sym.value_type)
                } else {
                    Ok(ValueType::Unknown)
                }
            }
            Expression::ArrayLiteral(_) => Ok(ValueType::Unknown),
            Expression::AnalogOperator(_) => Ok(ValueType::Real),
            Expression::NoiseSource(_) => Ok(ValueType::Real),
        }
    }

    fn validate_integer_operator_operand(
        operand_type: ValueType,
        context: &str,
        span: Span,
    ) -> CompileResult<()> {
        if matches!(
            operand_type,
            ValueType::Integer | ValueType::Boolean | ValueType::Unknown | ValueType::Error
        ) {
            return Ok(());
        }
        Err(CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: "integer".into(),
                found: operand_type.to_string(),
                context: context.into(),
            },
            span,
        )))
    }

    fn validate_integer_operator_expression(
        &mut self,
        expression: &Expression,
        context: &str,
    ) -> bool {
        match self.infer_type(expression) {
            Ok(operand_type) => {
                match Self::validate_integer_operator_operand(
                    operand_type,
                    context,
                    expression.span(),
                ) {
                    Ok(()) => true,
                    Err(CompileError::Semantic(error)) => {
                        self.errors.push(error);
                        false
                    }
                    Err(error) => {
                        self.record_error_at(
                            SemanticErrorKind::InvalidExpression(error.to_string()),
                            expression.span(),
                        );
                        false
                    }
                }
            }
            Err(CompileError::Semantic(error)) => {
                self.errors.push(error);
                false
            }
            Err(error) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(error.to_string()),
                    expression.span(),
                );
                false
            }
        }
    }

    /// Constant evaluation against parameter defaults. Suitable only for
    /// compile-time diagnostics (range checks on declared defaults):
    /// instances may override parameters.
    fn eval_const(&self, expr: &Expression) -> Option<f64> {
        self.eval_const_value(expr).map(ConstantValue::as_f64)
    }

    fn eval_const_value(&self, expr: &Expression) -> Option<ConstantValue> {
        Self::eval_const_value_with(expr, &self.param_consts)
    }

    /// Whether an expression is fixed for the duration of an analysis.
    /// Model parameters may differ between instances, but each resolved
    /// parameter value is constant while that instance is evaluated, so it is
    /// legal as an analog-operator control expression even though it is not
    /// safe for compile-time code-shape folding.
    fn expression_is_simulation_invariant(&self, expr: &Expression) -> bool {
        if self.eval_const(expr).is_some() {
            return true;
        }
        match expr {
            // Every resolved parameter is fixed on the instance even when its
            // default comes from an elaboration-time service such as
            // `$simparam` and therefore cannot be numerically folded here.
            Expression::Identifier(identifier) => self
                .symbols
                .lookup(&identifier.name)
                .is_some_and(|symbol| symbol.kind == SymbolKind::Parameter),
            Expression::Unary(unary) => self.expression_is_simulation_invariant(&unary.operand),
            Expression::Binary(binary) => {
                self.expression_is_simulation_invariant(&binary.left)
                    && self.expression_is_simulation_invariant(&binary.right)
            }
            Expression::Conditional(conditional) => {
                self.expression_is_simulation_invariant(&conditional.condition)
                    && self.expression_is_simulation_invariant(&conditional.then_expr)
                    && self.expression_is_simulation_invariant(&conditional.else_expr)
            }
            // Physical analysis selection is fixed across its solver phases.
            // IC, static, nodeset, and step-event queries may change within it.
            // Unknown literal queries are consistently false.
            Expression::Call(call) if call.name == "analysis" && !call.args.is_empty() => {
                call.args.iter().all(|argument| {
                    matches!(argument, Expression::StringLit(value)
                    if !matches!(
                        rspice_veriloga_runtime::analysis_query_id(&value.value),
                        Some(4 | 5 | 7..=9)
                    ))
                })
            }
            // Connectivity is fixed when an instance is elaborated. Compact
            // models use it together with model selectors to choose which
            // terminal owns a thermal `ddt`; that choice cannot change during
            // Newton or between accepted transient steps.
            Expression::SystemFunction(function)
                if function.name.eq_ignore_ascii_case("$port_connected")
                    && function.args.len() == 1 =>
            {
                matches!(
                    &function.args[0],
                    Expression::Identifier(identifier)
                        if self.symbols.lookup(&identifier.name).is_some_and(|symbol| {
                            matches!(symbol.kind, SymbolKind::Port | SymbolKind::Node)
                        })
                )
            }
            _ => false,
        }
    }

    /// Constant evaluation that only resolves instance-invariant values.
    /// Anything that shapes generated code (loop unrolling, repeat counts)
    /// must use this: folding a parameter's *default* would bake it in and
    /// break per-instance overrides.
    fn eval_const_invariant(&self, expr: &Expression) -> Option<f64> {
        self.eval_const_invariant_value(expr)
            .map(ConstantValue::as_f64)
    }

    fn eval_const_invariant_value(&self, expr: &Expression) -> Option<ConstantValue> {
        Self::eval_const_value_with(expr, &self.invariant_consts)
    }

    pub(crate) fn exact_const_i64(value: f64) -> Option<i64> {
        (value.is_finite()
            && value.fract() == 0.0
            && value >= i64::MIN as f64
            && value < 9_223_372_036_854_775_808.0)
            .then_some(value as i64)
    }

    fn integer_literal_value(number: &NumberLit) -> Option<i64> {
        parse_integer_literal(number.raw.as_str()).ok().flatten()
    }

    fn constant_for_declared_type(
        value: ConstantValue,
        declared_type: ParamType,
    ) -> Option<ConstantValue> {
        match declared_type {
            ParamType::Real => Some(ConstantValue::Real(value.as_f64())),
            ParamType::Integer => real_to_integer(value.as_f64())
                .ok()
                .map(|value| ConstantValue::Integer(i64::from(value))),
            ParamType::String => None,
        }
    }

    fn constant_integer(value: ConstantValue) -> Option<i64> {
        match value {
            ConstantValue::Integer(value) => Some(value),
            ConstantValue::Real(_) => None,
        }
    }

    fn constant_add(left: ConstantValue, right: ConstantValue) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
                left.checked_add(right).map(ConstantValue::Integer)
            }
            _ => Some(ConstantValue::Real(left.as_f64() + right.as_f64())),
        }
    }

    fn constant_sub(left: ConstantValue, right: ConstantValue) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
                left.checked_sub(right).map(ConstantValue::Integer)
            }
            _ => Some(ConstantValue::Real(left.as_f64() - right.as_f64())),
        }
    }

    fn constant_mul(left: ConstantValue, right: ConstantValue) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
                left.checked_mul(right).map(ConstantValue::Integer)
            }
            _ => Some(ConstantValue::Real(left.as_f64() * right.as_f64())),
        }
    }

    fn constant_div(left: ConstantValue, right: ConstantValue) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
                left.checked_div(right).map(ConstantValue::Integer)
            }
            _ => Some(ConstantValue::Real(left.as_f64() / right.as_f64())),
        }
    }

    fn constant_mod(left: ConstantValue, right: ConstantValue) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
                left.checked_rem(right).map(ConstantValue::Integer)
            }
            _ => Some(ConstantValue::Real(left.as_f64() % right.as_f64())),
        }
    }

    fn constant_pow(left: ConstantValue, right: ConstantValue) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
                let exponent = u32::try_from(right).ok()?;
                left.checked_pow(exponent).map(ConstantValue::Integer)
            }
            _ => Some(ConstantValue::Real(left.as_f64().powf(right.as_f64()))),
        }
    }

    fn constant_min(left: ConstantValue, right: ConstantValue) -> ConstantValue {
        if left.is_real() || right.is_real() {
            ConstantValue::Real(left.as_f64().min(right.as_f64()))
        } else {
            ConstantValue::Integer(
                Self::constant_integer(left)
                    .expect("integer checked")
                    .min(Self::constant_integer(right).expect("integer checked")),
            )
        }
    }

    fn constant_max(left: ConstantValue, right: ConstantValue) -> ConstantValue {
        if left.is_real() || right.is_real() {
            ConstantValue::Real(left.as_f64().max(right.as_f64()))
        } else {
            ConstantValue::Integer(
                Self::constant_integer(left)
                    .expect("integer checked")
                    .max(Self::constant_integer(right).expect("integer checked")),
            )
        }
    }

    fn eval_const_value_with(
        expr: &Expression,
        env: &HashMap<SmolStr, ConstantValue>,
    ) -> Option<ConstantValue> {
        let eval = |e: &Expression| Self::eval_const_value_with(e, env);
        match expr {
            Expression::Number(number) => Some(
                Self::integer_literal_value(number)
                    .map(ConstantValue::Integer)
                    .unwrap_or(ConstantValue::Real(number.value)),
            ),
            Expression::Unary(u) => {
                let v = eval(&u.operand)?;
                Some(match u.op {
                    UnaryOp::ToInteger => {
                        ConstantValue::Integer(i64::from(real_to_integer(v.as_f64()).ok()?))
                    }
                    UnaryOp::Neg => match v {
                        ConstantValue::Integer(value) => {
                            ConstantValue::Integer(value.checked_neg()?)
                        }
                        ConstantValue::Real(value) => ConstantValue::Real(-value),
                    },
                    UnaryOp::Pos => v,
                    UnaryOp::Not => ConstantValue::Integer(i64::from(!v.is_truthy())),
                    UnaryOp::BitNot => ConstantValue::Integer(i64::from(
                        !i32::try_from(Self::constant_integer(v)?).ok()?,
                    )),
                })
            }
            Expression::Binary(b) => {
                let l = eval(&b.left)?;
                let r = eval(&b.right)?;
                Some(match b.op {
                    BinaryOp::CheckedValue => return None,
                    BinaryOp::IntAdd
                    | BinaryOp::IntSub
                    | BinaryOp::IntMul
                    | BinaryOp::IntDiv
                    | BinaryOp::IntMod
                    | BinaryOp::IntPow => ConstantValue::Integer(
                        crate::integer_runtime::integer_arithmetic(
                            b.op.integer_arithmetic()?,
                            l.as_f64(),
                            r.as_f64(),
                        )
                        .ok()? as i64,
                    ),
                    BinaryOp::Add => Self::constant_add(l, r)?,
                    BinaryOp::Sub => Self::constant_sub(l, r)?,
                    BinaryOp::Mul => Self::constant_mul(l, r)?,
                    BinaryOp::Div => Self::constant_div(l, r)?,
                    BinaryOp::Mod => Self::constant_mod(l, r)?,
                    BinaryOp::Pow => Self::constant_pow(l, r)?,
                    BinaryOp::Eq => ConstantValue::Integer(i64::from(
                        l.numeric_order(r) == Some(Ordering::Equal),
                    )),
                    BinaryOp::Ne => ConstantValue::Integer(i64::from(
                        l.numeric_order(r) != Some(Ordering::Equal),
                    )),
                    BinaryOp::Lt => ConstantValue::Integer(i64::from(
                        l.numeric_order(r) == Some(Ordering::Less),
                    )),
                    BinaryOp::Le => ConstantValue::Integer(i64::from(matches!(
                        l.numeric_order(r),
                        Some(Ordering::Less | Ordering::Equal)
                    ))),
                    BinaryOp::Gt => ConstantValue::Integer(i64::from(
                        l.numeric_order(r) == Some(Ordering::Greater),
                    )),
                    BinaryOp::Ge => ConstantValue::Integer(i64::from(matches!(
                        l.numeric_order(r),
                        Some(Ordering::Greater | Ordering::Equal)
                    ))),
                    BinaryOp::And => {
                        ConstantValue::Integer(i64::from(l.is_truthy() && r.is_truthy()))
                    }
                    BinaryOp::Or => {
                        ConstantValue::Integer(i64::from(l.is_truthy() || r.is_truthy()))
                    }
                    BinaryOp::Shl => {
                        let value = integer_binary(
                            IntegerBinaryOperation::Shl,
                            Self::constant_integer(l)? as f64,
                            Self::constant_integer(r)? as f64,
                        )
                        .ok()?;
                        ConstantValue::Integer(i64::from(real_to_integer(value).ok()?))
                    }
                    BinaryOp::Shr => {
                        let value = integer_binary(
                            IntegerBinaryOperation::Shr,
                            Self::constant_integer(l)? as f64,
                            Self::constant_integer(r)? as f64,
                        )
                        .ok()?;
                        ConstantValue::Integer(i64::from(real_to_integer(value).ok()?))
                    }
                    BinaryOp::BitAnd | BinaryOp::BitOr | BinaryOp::BitXor => {
                        let operation = match b.op {
                            BinaryOp::BitAnd => IntegerBinaryOperation::BitAnd,
                            BinaryOp::BitOr => IntegerBinaryOperation::BitOr,
                            BinaryOp::BitXor => IntegerBinaryOperation::BitXor,
                            _ => unreachable!("bitwise branch only receives bitwise operators"),
                        };
                        let value = integer_binary(
                            operation,
                            Self::constant_integer(l)? as f64,
                            Self::constant_integer(r)? as f64,
                        )
                        .ok()?;
                        ConstantValue::Integer(i64::from(real_to_integer(value).ok()?))
                    }
                })
            }
            Expression::Conditional(c) => {
                let cond = eval(&c.condition)?;
                if cond.is_truthy() {
                    eval(&c.then_expr)
                } else {
                    eval(&c.else_expr)
                }
            }
            Expression::Call(call) => {
                let args: Option<Vec<ConstantValue>> = call.args.iter().map(eval).collect();
                let args = args?;
                match (call.name.as_str(), args.as_slice()) {
                    ("abs", [ConstantValue::Integer(value)]) => {
                        Some(ConstantValue::Integer(value.checked_abs()?))
                    }
                    ("abs", [value]) => Some(ConstantValue::Real(value.as_f64().abs())),
                    ("sqrt", [value]) => Some(ConstantValue::Real(value.as_f64().sqrt())),
                    ("exp", [value]) => Some(ConstantValue::Real(value.as_f64().exp())),
                    ("ln" | "log", [value]) => Some(ConstantValue::Real(value.as_f64().ln())),
                    ("log10", [value]) => Some(ConstantValue::Real(value.as_f64().log10())),
                    ("floor", [value]) => Some(ConstantValue::Real(value.as_f64().floor())),
                    ("ceil", [value]) => Some(ConstantValue::Real(value.as_f64().ceil())),
                    ("min", [a, b]) => Some(Self::constant_min(*a, *b)),
                    ("max", [a, b]) => Some(Self::constant_max(*a, *b)),
                    ("pow", [a, b]) => Some(ConstantValue::Real(a.as_f64().powf(b.as_f64()))),
                    _ => None,
                }
            }
            Expression::Identifier(ident) => match ident.name.as_str() {
                "inf" => Some(ConstantValue::Real(f64::INFINITY)),
                name => env.get(name).copied(),
            },
            _ => None,
        }
    }

    /// Compatibility entry point for hierarchy elaboration, whose resolved
    /// scalar vector is still represented as `f64`. Literal arithmetic keeps
    /// its source type; resolved identifiers are real until the hierarchy
    /// parameter vector gains explicit type metadata.
    fn eval_const_with(expr: &Expression, env: &HashMap<SmolStr, f64>) -> Option<f64> {
        let typed_env = env
            .iter()
            .map(|(name, value)| (name.clone(), ConstantValue::Real(*value)))
            .collect();
        Self::eval_const_value_with(expr, &typed_env).map(ConstantValue::as_f64)
    }

    fn parse_range(
        &self,
        range: &ParameterRange,
        param_names: &std::collections::HashSet<SmolStr>,
    ) -> TypedParameterRange {
        // Bounds that directly reference another parameter remain dynamic;
        // they must be checked against the final instance parameter vector.
        let fold = |e: &Expression| -> Option<f64> {
            if Self::references_identifiers(e, param_names) {
                None
            } else {
                self.eval_const(e)
            }
        };
        let computed = |expression: &Expression| {
            (Self::references_identifiers(expression, param_names)
                && Self::direct_parameter_reference(expression, param_names).is_none())
            .then(|| expression.clone())
        };

        // Extract bounds from first range bound if present
        if let Some(bound) = range.bounds.first() {
            let min = bound.lower.as_ref().and_then(fold);
            let max = bound.upper.as_ref().and_then(fold);
            let min_parameter = bound
                .lower
                .as_ref()
                .and_then(|expression| Self::direct_parameter_reference(expression, param_names));
            let max_parameter = bound
                .upper
                .as_ref()
                .and_then(|expression| Self::direct_parameter_reference(expression, param_names));
            let min_expression = bound.lower.as_ref().and_then(computed);
            let max_expression = bound.upper.as_ref().and_then(computed);
            let exclude: Vec<f64> = range.exclude.iter().filter_map(fold).collect();
            let exclude_parameters = range
                .exclude
                .iter()
                .filter_map(|expression| Self::direct_parameter_reference(expression, param_names))
                .collect();
            let exclude_expressions = range.exclude.iter().filter_map(computed).collect();

            TypedParameterRange {
                min,
                max,
                min_parameter,
                max_parameter,
                min_expression,
                max_expression,
                min_exclusive: !bound.lower_inclusive,
                max_exclusive: !bound.upper_inclusive,
                exclude,
                exclude_parameters,
                exclude_expressions,
            }
        } else {
            TypedParameterRange::unrestricted()
        }
    }

    fn record_error_at(&mut self, kind: SemanticErrorKind, span: Span) {
        self.errors.push(SemanticError::new(kind, span));
    }

    /// Validate the declaration-only portion of a public parameter array.
    /// Bounds remain symbolic in analyzed output: instance overrides of an
    /// earlier scalar parameter may change the eventual shape. This pass only
    /// proves that resolving those bounds later is deterministic and numeric.
    /// Expand retained replication only for language constructs whose result
    /// is an unpacked assignment-pattern value. The source AST is never
    /// mutated; analyzed/canonical metadata receives a bounded materialized
    /// clone. A complete sizing pass runs before any expanded vector is
    /// allocated, including checked multiplication for nested replication.
    fn materialize_replication_expression(
        &self,
        expression: &Expression,
        max_elements_per_pattern: usize,
        max_work: usize,
        owner: &str,
        require_instance_invariant_count: bool,
    ) -> CompileResult<Expression> {
        let Expression::ArrayLiteral(array) = expression else {
            return Ok(expression.clone());
        };
        self.measure_replication_array(
            array,
            max_elements_per_pattern,
            max_work,
            owner,
            require_instance_invariant_count,
            0,
        )?;
        Ok(Expression::ArrayLiteral(self.build_materialized_array(
            array,
            owner,
            require_instance_invariant_count,
        )?))
    }

    fn replication_count(
        &self,
        replication: &ReplicationExpr,
        owner: &str,
        require_instance_invariant_count: bool,
    ) -> CompileResult<u64> {
        let value = if require_instance_invariant_count {
            self.eval_const_invariant_value(&replication.count)
        } else {
            self.eval_const_value(&replication.count)
        };
        let count = match value {
            Some(ConstantValue::Integer(value)) => value,
            Some(ConstantValue::Real(value)) => {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} replication count must be an integer constant expression; found real value {value}"
                    )),
                    replication.count.span(),
                )));
            }
            None => {
                let requirement = if require_instance_invariant_count {
                    "an instance-invariant integer constant expression (overridable parameters cannot determine executable operand shape)"
                } else {
                    "an integer constant expression"
                };
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} replication count must be {requirement}"
                    )),
                    replication.count.span(),
                )));
            }
        };
        if count < 0 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidExpression(format!(
                    "{owner} replication count must be non-negative; found {count}"
                )),
                replication.count.span(),
            )));
        }
        u64::try_from(count).map_err(|_| {
            CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidExpression(format!(
                    "{owner} replication count {count} is not representable by the materialization contract"
                )),
                replication.count.span(),
            ))
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn measure_replication_array(
        &self,
        array: &ArrayLiteralExpr,
        max_elements_per_pattern: usize,
        max_work: usize,
        owner: &str,
        require_instance_invariant_count: bool,
        depth: usize,
    ) -> CompileResult<(usize, usize)> {
        if depth >= MAX_REPLICATION_NESTING {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "{owner} replication nesting exceeds the safety limit of {MAX_REPLICATION_NESTING}"
                )),
                array.span,
            )));
        }
        let (elements, work) = self.measure_replication_elements(
            &array.elements,
            max_elements_per_pattern,
            max_work,
            owner,
            require_instance_invariant_count,
            depth,
        )?;
        if elements > max_elements_per_pattern {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "{owner} materializes {elements} elements in one assignment-pattern dimension; the supported safety limit is {max_elements_per_pattern}"
                )),
                array.span,
            )));
        }
        if work > max_work {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "{owner} replication materialization requires {work} syntax-tree items; the work budget is {max_work}"
                )),
                array.span,
            )));
        }
        Ok((elements, work))
    }

    #[allow(clippy::too_many_arguments)]
    fn measure_replication_elements(
        &self,
        elements: &[ArrayLiteralElement],
        max_elements_per_pattern: usize,
        max_work: usize,
        owner: &str,
        require_instance_invariant_count: bool,
        depth: usize,
    ) -> CompileResult<(usize, usize)> {
        if depth >= MAX_REPLICATION_NESTING {
            let span = elements
                .first()
                .map(ArrayLiteralElement::span)
                .unwrap_or_else(|| Span::new(crate::source::SourceId::new(0), 0, 0));
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "{owner} replication nesting exceeds the safety limit of {MAX_REPLICATION_NESTING}"
                )),
                span,
            )));
        }
        let mut output_len = 0_usize;
        let mut work = 0_usize;
        for element in elements {
            let (element_len, element_work, span) = match element {
                ArrayLiteralElement::Value(Expression::ArrayLiteral(nested)) => {
                    let (_, nested_work) = self.measure_replication_array(
                        nested,
                        max_elements_per_pattern,
                        max_work,
                        owner,
                        require_instance_invariant_count,
                        depth + 1,
                    )?;
                    (1, nested_work.checked_add(1), nested.span)
                }
                ArrayLiteralElement::Value(expression) => (1, Some(1), expression.span()),
                ArrayLiteralElement::Replication(replication) => {
                    if replication.elements.is_empty() {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} replication body must contain at least one element"
                            )),
                            replication.span,
                        )));
                    }
                    let count = self.replication_count(
                        replication,
                        owner,
                        require_instance_invariant_count,
                    )?;
                    let (body_len, body_work) = self.measure_replication_elements(
                        &replication.elements,
                        max_elements_per_pattern,
                        max_work,
                        owner,
                        require_instance_invariant_count,
                        depth + 1,
                    )?;
                    let expanded_len = count
                        .checked_mul(u64::try_from(body_len).map_err(|_| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidExpression(format!(
                                    "{owner} replication body length is not representable as u64"
                                )),
                                replication.span,
                            ))
                        })?)
                        .ok_or_else(|| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidExpression(format!(
                                    "{owner} replication element count overflows u64"
                                )),
                                replication.span,
                            ))
                        })?;
                    let expanded_work = count
                        .checked_mul(u64::try_from(body_work).map_err(|_| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidExpression(format!(
                                    "{owner} replication body work is not representable as u64"
                                )),
                                replication.span,
                            ))
                        })?)
                        .ok_or_else(|| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidExpression(format!(
                                    "{owner} replication work count overflows u64"
                                )),
                                replication.span,
                            ))
                        })?;
                    if expanded_len > max_elements_per_pattern as u64 {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::UnsupportedFeature(format!(
                                "{owner} materializes {expanded_len} elements in one assignment-pattern dimension; the supported safety limit is {max_elements_per_pattern}"
                            )),
                            replication.span,
                        )));
                    }
                    if expanded_work > max_work as u64 {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::UnsupportedFeature(format!(
                                "{owner} replication materialization requires {expanded_work} syntax-tree items; the work budget is {max_work}"
                            )),
                            replication.span,
                        )));
                    }
                    (
                        usize::try_from(expanded_len).map_err(|_| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidExpression(format!(
                                    "{owner} replication element count {expanded_len} is not representable on this platform"
                                )),
                                replication.span,
                            ))
                        })?,
                        usize::try_from(expanded_work).ok(),
                        replication.span,
                    )
                }
            };
            let element_work = element_work.ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} replication work count overflows usize"
                    )),
                    span,
                ))
            })?;
            output_len = output_len.checked_add(element_len).ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} replication element count overflows usize"
                    )),
                    span,
                ))
            })?;
            work = work.checked_add(element_work).ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} replication work count overflows usize"
                    )),
                    span,
                ))
            })?;
            if output_len > max_elements_per_pattern {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "{owner} materializes {output_len} elements in one assignment-pattern dimension; the supported safety limit is {max_elements_per_pattern}"
                    )),
                    span,
                )));
            }
            if work > max_work {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "{owner} replication materialization requires more than {max_work} syntax-tree items"
                    )),
                    span,
                )));
            }
        }
        Ok((output_len, work))
    }

    fn build_materialized_array(
        &self,
        array: &ArrayLiteralExpr,
        owner: &str,
        require_instance_invariant_count: bool,
    ) -> CompileResult<ArrayLiteralExpr> {
        let elements = self.build_materialized_elements(
            &array.elements,
            owner,
            require_instance_invariant_count,
        )?;
        Ok(ArrayLiteralExpr {
            elements,
            assignment_pattern: array.assignment_pattern,
            span: array.span,
        })
    }

    fn build_materialized_elements(
        &self,
        elements: &[ArrayLiteralElement],
        owner: &str,
        require_instance_invariant_count: bool,
    ) -> CompileResult<Vec<ArrayLiteralElement>> {
        let mut materialized = Vec::new();
        for element in elements {
            match element {
                ArrayLiteralElement::Value(Expression::ArrayLiteral(nested)) => {
                    materialized.push(ArrayLiteralElement::Value(Expression::ArrayLiteral(
                        self.build_materialized_array(
                            nested,
                            owner,
                            require_instance_invariant_count,
                        )?,
                    )));
                }
                ArrayLiteralElement::Value(expression) => {
                    materialized.push(ArrayLiteralElement::Value(expression.clone()));
                }
                ArrayLiteralElement::Replication(replication) => {
                    let count = self.replication_count(
                        replication,
                        owner,
                        require_instance_invariant_count,
                    )?;
                    let count = usize::try_from(count).map_err(|_| {
                        CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} replication count is not representable on this platform during materialization"
                            )),
                            replication.span,
                        ))
                    })?;
                    let body = self.build_materialized_elements(
                        &replication.elements,
                        owner,
                        require_instance_invariant_count,
                    )?;
                    let additional = count.checked_mul(body.len()).ok_or_else(|| {
                        CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} replication element count overflows usize during materialization"
                            )),
                            replication.span,
                        ))
                    })?;
                    materialized.try_reserve(additional).map_err(|_| {
                        CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::UnsupportedFeature(format!(
                                "{owner} replication materialization could not reserve storage for {additional} elements"
                            )),
                            replication.span,
                        ))
                    })?;
                    for _ in 0..count {
                        materialized.extend(body.iter().cloned());
                    }
                }
            }
        }
        Ok(materialized)
    }

    fn validate_parameter_array_declaration(
        &mut self,
        parameter: &ParameterDecl,
        materialized_default: Option<&Expression>,
        parameter_index: usize,
        parameters: &[ParameterDecl],
        parameter_indices: &HashMap<SmolStr, usize>,
    ) {
        if !parameter.type_is_explicit {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "parameter array '{}' requires an explicit integer or real element type",
                    parameter.name
                )),
                parameter.span,
            );
        }
        if parameter.default.is_none() {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "parameter array '{}' requires a default value",
                    parameter.name
                )),
                parameter.span,
            );
        }
        if parameter.param_type == ParamType::String {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "string parameter array '{}' is not supported; parameter arrays must have integer or real elements",
                    parameter.name
                )),
                parameter.span,
            );
        }
        if let Some(range) = &parameter.range {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "parameter array '{}' may not use from/exclude constraints until array-valued constraint semantics are implemented",
                    parameter.name
                )),
                range.span,
            );
        }
        if parameter.dimensions.len() > MAX_PARAMETER_ARRAY_RANK {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "parameter array '{}' has rank {}; the supported safety limit is {}",
                    parameter.name,
                    parameter.dimensions.len(),
                    MAX_PARAMETER_ARRAY_RANK
                )),
                parameter.span,
            );
        }

        let initializer = match materialized_default.or(parameter.default.as_ref()) {
            Some(Expression::ArrayLiteral(initializer)) if initializer.assignment_pattern => {
                Some(initializer)
            }
            Some(Expression::ArrayLiteral(initializer)) => {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "constant assignment pattern opened with `'{'".into(),
                        found: "concatenation opened with '{'".into(),
                        context: format!("default of parameter array '{}'", parameter.name),
                    },
                    initializer.span,
                );
                None
            }
            Some(other) => {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "constant assignment pattern".into(),
                        found: "scalar expression".into(),
                        context: format!("default of parameter array '{}'", parameter.name),
                    },
                    other.span(),
                );
                None
            }
            None => None,
        };

        for (dimension_index, dimension) in parameter.dimensions.iter().enumerate() {
            for (side, bound) in [("left", &dimension.start), ("right", &dimension.end)] {
                let owner = format!(
                    "{side} bound of dimension {} of parameter array '{}'",
                    dimension_index + 1,
                    parameter.name
                );
                if !self.validate_parameter_array_bound_expression(
                    bound,
                    &owner,
                    parameter_index,
                    parameters,
                    parameter_indices,
                ) {
                    continue;
                }

                match self.eval_const(bound) {
                    Some(value) if !value.is_finite() => {
                        self.record_error_at(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} resolves to non-finite value {value}"
                            )),
                            bound.span(),
                        );
                    }
                    Some(value) if value.fract() != 0.0 => {
                        self.record_error_at(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} resolves to non-integral value {value}"
                            )),
                            bound.span(),
                        );
                    }
                    Some(value)
                        if !(-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0)
                            .contains(&value) =>
                    {
                        self.record_error_at(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} resolves outside the signed 64-bit index range: {value}"
                            )),
                            bound.span(),
                        );
                    }
                    Some(_) => {}
                    None => self.record_error_at(
                        SemanticErrorKind::InvalidExpression(format!(
                            "{owner} does not resolve to a valid integer using declared parameter defaults"
                        )),
                        bound.span(),
                    ),
                }
            }
        }

        let constant_shape = self.resolve_parameter_array_default_shape(parameter);
        if let Some(initializer) = initializer {
            if parameter.param_type != ParamType::String {
                let owner = format!("default of parameter array '{}'", parameter.name);
                for element in &initializer.elements {
                    match element {
                        ArrayLiteralElement::Value(expression) => {
                            self.validate_parameter_array_initializer_elements(
                                expression,
                                parameter,
                                &owner,
                                parameter_index,
                                parameters,
                                parameter_indices,
                            );
                        }
                        ArrayLiteralElement::Replication(replication) => {
                            self.record_error_at(
                                SemanticErrorKind::InvalidExpression(format!(
                                    "{owner} retained replication reached parameter-array validation without bounded materialization"
                                )),
                                replication.span,
                            );
                        }
                    }
                }
            }

            if let Some(shape) = constant_shape
                && let Err(detail) =
                    Self::validate_parameter_array_initializer_shape(initializer, &shape, 0)
            {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: format!("rectangular assignment pattern with shape {shape:?}"),
                        found: detail,
                        context: format!("default of parameter array '{}'", parameter.name),
                    },
                    initializer.span,
                );
            }
        }
    }

    fn resolve_parameter_array_default_shape(
        &mut self,
        parameter: &ParameterDecl,
    ) -> Option<Vec<u64>> {
        if parameter.dimensions.len() > MAX_PARAMETER_ARRAY_RANK {
            return None;
        }

        let mut shape = Vec::with_capacity(parameter.dimensions.len());
        let mut total_elements = 1_u64;
        for (dimension_index, dimension) in parameter.dimensions.iter().enumerate() {
            let left = Self::exact_const_i64(self.eval_const(&dimension.start)?)?;
            let right = Self::exact_const_i64(self.eval_const(&dimension.end)?)?;
            let Some(extent) = left.abs_diff(right).checked_add(1) else {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "dimension {} of parameter array '{}' has an unrepresentable extent",
                        dimension_index + 1,
                        parameter.name
                    )),
                    dimension.span,
                );
                return None;
            };
            let Some(next_total) = total_elements.checked_mul(extent) else {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "parameter array '{}' element count overflows the canonical shape representation",
                        parameter.name
                    )),
                    parameter.span,
                );
                return None;
            };
            if next_total > MAX_PARAMETER_ARRAY_ELEMENTS {
                self.record_error_at(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "parameter array '{}' declares {next_total} elements; the supported safety limit is {MAX_PARAMETER_ARRAY_ELEMENTS}",
                        parameter.name
                    )),
                    parameter.span,
                );
                return None;
            }
            total_elements = next_total;
            shape.push(extent);
        }
        Some(shape)
    }

    fn validate_parameter_array_initializer_elements(
        &mut self,
        expression: &Expression,
        parameter: &ParameterDecl,
        owner: &str,
        parameter_index: usize,
        parameters: &[ParameterDecl],
        parameter_indices: &HashMap<SmolStr, usize>,
    ) -> bool {
        if let Expression::ArrayLiteral(array) = expression {
            return array.elements.iter().fold(true, |valid, element| {
                let element_valid = match element {
                    ArrayLiteralElement::Value(expression) => {
                        self.validate_parameter_array_initializer_elements(
                            expression,
                            parameter,
                            owner,
                            parameter_index,
                            parameters,
                            parameter_indices,
                        )
                    }
                    ArrayLiteralElement::Replication(replication) => {
                        self.record_error_at(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} retained replication reached parameter-array validation without bounded materialization"
                            )),
                            replication.span,
                        );
                        false
                    }
                };
                element_valid && valid
            });
        }

        let valid = self.validate_parameter_array_bound_expression(
            expression,
            owner,
            parameter_index,
            parameters,
            parameter_indices,
        );
        if !valid {
            return false;
        }

        let Some(value) = self.eval_const(expression) else {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "{owner} element does not resolve using declared parameter defaults"
                )),
                expression.span(),
            );
            return false;
        };
        if !value.is_finite() {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "{owner} element resolves to non-finite value {value}"
                )),
                expression.span(),
            );
            return false;
        }
        if parameter.param_type == ParamType::Integer && real_to_integer(value).is_err() {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: "32-bit integer array element".into(),
                    found: value.to_string(),
                    context: owner.into(),
                },
                expression.span(),
            );
            return false;
        }
        true
    }

    fn coerce_integer_parameter_array_default(
        &self,
        expression: Expression,
    ) -> CompileResult<Expression> {
        match expression {
            Expression::ArrayLiteral(mut array) => {
                for element in &mut array.elements {
                    if let ArrayLiteralElement::Value(value) = element {
                        *value = self.coerce_integer_parameter_array_default(value.clone())?;
                    }
                }
                Ok(Expression::ArrayLiteral(array))
            }
            value => self
                .coerce_assignment_expression(value, ValueType::Integer)
                .map(|(value, _)| value),
        }
    }

    fn validate_parameter_array_initializer_shape(
        initializer: &crate::ast::ArrayLiteralExpr,
        shape: &[u64],
        dimension: usize,
    ) -> Result<(), String> {
        let Some(&expected_len) = shape.get(dimension) else {
            return Err(format!(
                "nested assignment pattern extends beyond declared rank {}",
                shape.len()
            ));
        };
        if u64::try_from(initializer.elements.len()).ok() != Some(expected_len) {
            return Err(format!(
                "dimension {} has {} elements",
                dimension + 1,
                initializer.elements.len()
            ));
        }

        let is_leaf = dimension + 1 == shape.len();
        for element in &initializer.elements {
            match (is_leaf, element) {
                (_, ArrayLiteralElement::Replication(_)) => {
                    return Err(format!(
                        "dimension {} contains retained replication after the materialization boundary",
                        dimension + 1
                    ));
                }
                (true, ArrayLiteralElement::Value(Expression::ArrayLiteral(_))) => {
                    return Err(format!(
                        "dimension {} contains an unexpected nested pattern",
                        dimension + 1
                    ));
                }
                (false, ArrayLiteralElement::Value(Expression::ArrayLiteral(nested))) => {
                    if !nested.assignment_pattern {
                        return Err(format!(
                            "dimension {} contains a concatenation instead of an assignment pattern",
                            dimension + 1
                        ));
                    }
                    Self::validate_parameter_array_initializer_shape(nested, shape, dimension + 1)?;
                }
                (false, ArrayLiteralElement::Value(_)) => {
                    return Err(format!(
                        "dimension {} contains a scalar before the final dimension",
                        dimension + 1
                    ));
                }
                (true, ArrayLiteralElement::Value(_)) => {}
            }
        }
        Ok(())
    }

    /// Return true when `expression` belongs to the numeric constant-expression
    /// subset used by parameter-array bounds. Diagnostics identify the first
    /// invalid leaf so malformed shapes cannot degrade into an unresolved
    /// runtime expression.
    fn validate_parameter_array_bound_expression(
        &mut self,
        expression: &Expression,
        owner: &str,
        parameter_index: usize,
        parameters: &[ParameterDecl],
        parameter_indices: &HashMap<SmolStr, usize>,
    ) -> bool {
        let validate_child = |this: &mut Self, child: &Expression| {
            this.validate_parameter_array_bound_expression(
                child,
                owner,
                parameter_index,
                parameters,
                parameter_indices,
            )
        };

        match expression {
            Expression::Number(_) => true,
            Expression::Digital(digital) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} uses a {}, which has no continuous-domain value",
                        digital.construct()
                    )),
                    digital.span(),
                );
                false
            }
            Expression::Identifier(identifier) if identifier.name == "inf" => true,
            Expression::Identifier(identifier) => {
                let Some(&referenced_index) = parameter_indices.get(&identifier.name) else {
                    self.record_error_at(
                        SemanticErrorKind::InvalidExpression(format!(
                            "{owner} references unknown identifier '{}'",
                            identifier.name
                        )),
                        identifier.span,
                    );
                    return false;
                };
                let referenced = &parameters[referenced_index];
                if referenced_index == parameter_index {
                    self.record_error_at(
                        SemanticErrorKind::CircularDependency(format!(
                            "{owner} references parameter '{}' itself",
                            parameters[parameter_index].name
                        )),
                        identifier.span,
                    );
                    false
                } else if referenced_index > parameter_index {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "{owner} references later parameter '{}'; parameter-array bounds may reference only previously declared scalar parameters",
                            referenced.name
                        )),
                        identifier.span,
                    );
                    false
                } else if !referenced.dimensions.is_empty() {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "{owner} references parameter array '{}'; parameter-array bounds may reference only previously declared scalar parameters",
                            referenced.name
                        )),
                        identifier.span,
                    );
                    false
                } else if referenced.param_type == ParamType::String {
                    self.record_error_at(
                        SemanticErrorKind::TypeMismatch {
                            expected: "previously declared numeric scalar parameter".into(),
                            found: format!("string parameter '{}'", referenced.name),
                            context: owner.into(),
                        },
                        identifier.span,
                    );
                    false
                } else {
                    true
                }
            }
            Expression::Unary(unary) => {
                let child = validate_child(self, &unary.operand);
                let operand = unary.op != UnaryOp::BitNot
                    || self.validate_integer_operator_expression(
                        &unary.operand,
                        "operand of bitwise complement",
                    );
                child && operand
            }
            Expression::Binary(binary) => {
                let left = validate_child(self, &binary.left);
                let right = validate_child(self, &binary.right);
                let operands = if matches!(
                    binary.op,
                    BinaryOp::BitAnd
                        | BinaryOp::BitOr
                        | BinaryOp::BitXor
                        | BinaryOp::Shl
                        | BinaryOp::Shr
                ) {
                    let left_type = self.validate_integer_operator_expression(
                        &binary.left,
                        "left operand of bitwise or shift operator",
                    );
                    let right_type = self.validate_integer_operator_expression(
                        &binary.right,
                        "right operand of bitwise or shift operator",
                    );
                    left_type && right_type
                } else {
                    true
                };
                left && right && operands
            }
            Expression::Conditional(conditional) => {
                let condition = validate_child(self, &conditional.condition);
                let then_expr = validate_child(self, &conditional.then_expr);
                let else_expr = validate_child(self, &conditional.else_expr);
                condition && then_expr && else_expr
            }
            Expression::Call(call) => {
                let expected_arity = match call.name.as_str() {
                    "abs" | "sqrt" | "exp" | "ln" | "log" | "log10" | "floor" | "ceil" => Some(1),
                    "min" | "max" | "pow" => Some(2),
                    _ => None,
                };
                let Some(expected_arity) = expected_arity else {
                    self.record_error_at(
                        SemanticErrorKind::InvalidExpression(format!(
                            "{owner} calls non-constant function '{}'",
                            call.name
                        )),
                        call.span,
                    );
                    return false;
                };
                if call.args.len() != expected_arity {
                    self.record_error_at(
                        SemanticErrorKind::ArgumentCountMismatch {
                            name: call.name.to_string(),
                            expected: expected_arity.to_string(),
                            got: call.args.len(),
                        },
                        call.span,
                    );
                    return false;
                }
                call.args.iter().fold(true, |valid, argument| {
                    validate_child(self, argument) && valid
                })
            }
            Expression::StringLit(string) => {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "numeric constant expression".into(),
                        found: "string literal".into(),
                        context: owner.into(),
                    },
                    string.span,
                );
                false
            }
            Expression::ArrayAccess(access) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} indexes array '{}'; parameter-array bounds may reference only previously declared scalar parameters",
                        access.array
                    )),
                    access.span,
                );
                false
            }
            Expression::ArrayLiteral(array) => {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "numeric constant expression".into(),
                        found: "array literal".into(),
                        context: owner.into(),
                    },
                    array.span,
                );
                false
            }
            Expression::SystemFunction(function) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} calls non-constant system function '{}'",
                        function.name
                    )),
                    function.span,
                );
                false
            }
            Expression::NullArgument(span) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} contains a null argument"
                    )),
                    *span,
                );
                false
            }
            Expression::BranchAccess(access) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} reads non-constant branch access"
                    )),
                    access.span(),
                );
                false
            }
            Expression::AnalogOperator(operator) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} contains non-constant analog operator"
                    )),
                    operator.span(),
                );
                false
            }
            Expression::NoiseSource(noise) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} contains non-constant noise source"
                    )),
                    noise.span(),
                );
                false
            }
        }
    }

    /// Verilog-AMS 2.3.1 section 3.4 permits a parameter initializer to read
    /// only parameters declared before it. Enforce that language rule here,
    /// before symbolic defaults reach any backend, so composite forward and
    /// cyclic references cannot acquire order-dependent values.
    fn validate_parameter_default_dependencies(
        &mut self,
        parameters: &[ParameterDecl],
        aliases: &[AliasParamDecl],
        indices: &HashMap<SmolStr, usize>,
    ) {
        // External SPICE parameter names and the generated `$param_given`
        // resolver are intentionally case-insensitive. Reject collisions up
        // front: otherwise the backend's sorted lookup table would pick one
        // declaration nondeterministically. Aliases resolve to their exact,
        // case-sensitive Verilog-A target.
        let mut param_given_indices: std::collections::HashMap<String, (usize, SmolStr)> =
            std::collections::HashMap::new();
        for (index, parameter) in parameters.iter().enumerate() {
            Self::insert_external_parameter_name(
                &mut param_given_indices,
                parameter.name.as_str(),
                parameter.name.as_str(),
                index,
                parameter.span,
                &mut self.errors,
            );
        }
        for alias in aliases {
            let Some(&target) = indices.get(&alias.target) else {
                continue;
            };
            Self::insert_external_parameter_name(
                &mut param_given_indices,
                alias.alias.as_str(),
                parameters[target].name.as_str(),
                target,
                alias.span,
                &mut self.errors,
            );
        }

        for (index, parameter) in parameters.iter().enumerate() {
            let Some(default) = &parameter.default else {
                continue;
            };
            let mut references = Vec::new();
            Self::collect_parameter_identifier_references(
                default,
                indices,
                &param_given_indices,
                &mut references,
            );
            let mut reported = std::collections::HashSet::new();
            for (referenced_index, referenced, span) in references {
                if !reported.insert(referenced_index) {
                    continue;
                }
                if referenced_index == index {
                    self.record_error_at(
                        SemanticErrorKind::CircularDependency(format!(
                            "default of parameter '{}' references itself",
                            parameter.name
                        )),
                        span,
                    );
                } else if referenced_index > index {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "default of parameter '{}' references later parameter '{}'; Verilog-AMS parameter defaults may reference only previously declared parameters",
                            parameter.name, referenced
                        )),
                        span,
                    );
                }
            }
        }
    }

    fn insert_external_parameter_name(
        names: &mut std::collections::HashMap<String, (usize, SmolStr)>,
        name: &str,
        canonical_name: &str,
        index: usize,
        span: Span,
        errors: &mut Vec<SemanticError>,
    ) {
        let folded = name.to_ascii_lowercase();
        if let Some((_, first_name)) = names.get(&folded) {
            errors.push(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "parameter lookup name '{name}' conflicts case-insensitively with '{first_name}'; generated SPICE parameter names and aliases must be unique ignoring ASCII case"
                )),
                span,
            ));
            return;
        }
        names.insert(folded, (index, canonical_name.into()));
    }

    fn collect_parameter_identifier_references<'a>(
        expression: &'a Expression,
        parameter_indices: &std::collections::HashMap<SmolStr, usize>,
        param_given_indices: &std::collections::HashMap<String, (usize, SmolStr)>,
        references: &mut Vec<(usize, SmolStr, Span)>,
    ) {
        match expression {
            Expression::Identifier(identifier) => {
                if let Some(&index) = parameter_indices.get(&identifier.name) {
                    references.push((index, identifier.name.clone(), identifier.span));
                }
            }
            Expression::Digital(digital) => {
                for child in digital.children() {
                    Self::collect_parameter_identifier_references(
                        child,
                        parameter_indices,
                        param_given_indices,
                        references,
                    );
                }
            }
            Expression::Binary(binary) => {
                Self::collect_parameter_identifier_references(
                    &binary.left,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
                Self::collect_parameter_identifier_references(
                    &binary.right,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
            }
            Expression::Unary(unary) => Self::collect_parameter_identifier_references(
                &unary.operand,
                parameter_indices,
                param_given_indices,
                references,
            ),
            Expression::Conditional(conditional) => {
                Self::collect_parameter_identifier_references(
                    &conditional.condition,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
                Self::collect_parameter_identifier_references(
                    &conditional.then_expr,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
                Self::collect_parameter_identifier_references(
                    &conditional.else_expr,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
            }
            Expression::Call(call) => {
                for argument in &call.args {
                    Self::collect_parameter_identifier_references(
                        argument,
                        parameter_indices,
                        param_given_indices,
                        references,
                    );
                }
            }
            Expression::SystemFunction(function) => {
                if function.name.eq_ignore_ascii_case("$param_given")
                    || function.name.eq_ignore_ascii_case("param_given")
                {
                    if let [Expression::Identifier(identifier)] = function.args.as_slice()
                        && let Some((index, canonical)) =
                            param_given_indices.get(&identifier.name.to_ascii_lowercase())
                    {
                        references.push((*index, canonical.clone(), identifier.span));
                    }
                    return;
                }
                for argument in &function.args {
                    Self::collect_parameter_identifier_references(
                        argument,
                        parameter_indices,
                        param_given_indices,
                        references,
                    );
                }
            }
            Expression::ArrayAccess(access) => {
                if let Some(&index) = parameter_indices.get(&access.array) {
                    references.push((index, access.array.clone(), access.span));
                }
                Self::collect_parameter_identifier_references(
                    &access.index,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
            }
            Expression::ArrayLiteral(array) => {
                for element in &array.elements {
                    Self::collect_array_element_parameter_identifier_references(
                        element,
                        parameter_indices,
                        param_given_indices,
                        references,
                    );
                }
            }
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => {}
        }
    }

    fn collect_array_element_parameter_identifier_references<'a>(
        element: &'a ArrayLiteralElement,
        parameter_indices: &HashMap<SmolStr, usize>,
        param_given_indices: &HashMap<String, (usize, SmolStr)>,
        references: &mut Vec<(usize, SmolStr, Span)>,
    ) {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::collect_parameter_identifier_references(
                    expression,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::collect_parameter_identifier_references(
                    &replication.count,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
                for element in &replication.elements {
                    Self::collect_array_element_parameter_identifier_references(
                        element,
                        parameter_indices,
                        param_given_indices,
                        references,
                    );
                }
            }
        }
    }

    /// Whether an expression references any identifier from the given set
    fn references_identifiers(
        expr: &Expression,
        names: &std::collections::HashSet<SmolStr>,
    ) -> bool {
        match expr {
            Expression::Identifier(id) => names.contains(&id.name),
            Expression::Digital(digital) => {
                digital.base_name().is_some_and(|name| names.contains(name))
                    || digital
                        .children()
                        .into_iter()
                        .any(|child| Self::references_identifiers(child, names))
            }
            Expression::Number(_) | Expression::StringLit(_) | Expression::NullArgument(_) => false,
            Expression::Binary(b) => {
                Self::references_identifiers(&b.left, names)
                    || Self::references_identifiers(&b.right, names)
            }
            Expression::Unary(u) => Self::references_identifiers(&u.operand, names),
            Expression::Conditional(c) => {
                Self::references_identifiers(&c.condition, names)
                    || Self::references_identifiers(&c.then_expr, names)
                    || Self::references_identifiers(&c.else_expr, names)
            }
            Expression::Call(call) => call
                .args
                .iter()
                .any(|a| Self::references_identifiers(a, names)),
            Expression::SystemFunction(f) => f
                .args
                .iter()
                .any(|a| Self::references_identifiers(a, names)),
            Expression::ArrayAccess(a) => {
                names.contains(&a.array) || Self::references_identifiers(&a.index, names)
            }
            Expression::ArrayLiteral(a) => a
                .elements
                .iter()
                .any(|element| Self::array_element_references_identifiers(element, names)),
            Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }

    fn array_element_references_identifiers(
        element: &ArrayLiteralElement,
        names: &std::collections::HashSet<SmolStr>,
    ) -> bool {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::references_identifiers(expression, names)
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::references_identifiers(&replication.count, names)
                    || replication
                        .elements
                        .iter()
                        .any(|element| Self::array_element_references_identifiers(element, names))
            }
        }
    }

    /// Whether a model-parameter default reads a parameter that has no
    /// model-card storage. Ordinary Verilog-A identifiers resolve with exact
    /// case; `$param_given` follows the generated external lookup and accepts
    /// canonical names and aliases case-insensitively.
    fn references_parameter_without_model_storage(
        expr: &Expression,
        canonical_storage: &std::collections::HashMap<SmolStr, bool>,
        external_storage: &std::collections::HashMap<String, bool>,
    ) -> bool {
        match expr {
            Expression::Identifier(identifier) => canonical_storage
                .get(&identifier.name)
                .is_some_and(|has_model_storage| !has_model_storage),
            Expression::Digital(digital) => digital.children().into_iter().any(|child| {
                Self::references_parameter_without_model_storage(
                    child,
                    canonical_storage,
                    external_storage,
                )
            }),
            Expression::SystemFunction(function)
                if function.name.eq_ignore_ascii_case("$param_given")
                    || function.name.eq_ignore_ascii_case("param_given") =>
            {
                let [Expression::Identifier(identifier)] = function.args.as_slice() else {
                    return false;
                };
                external_storage
                    .get(&identifier.name.to_ascii_lowercase())
                    .is_some_and(|has_model_storage| !has_model_storage)
            }
            Expression::Binary(binary) => {
                Self::references_parameter_without_model_storage(
                    &binary.left,
                    canonical_storage,
                    external_storage,
                ) || Self::references_parameter_without_model_storage(
                    &binary.right,
                    canonical_storage,
                    external_storage,
                )
            }
            Expression::Unary(unary) => Self::references_parameter_without_model_storage(
                &unary.operand,
                canonical_storage,
                external_storage,
            ),
            Expression::Conditional(conditional) => {
                Self::references_parameter_without_model_storage(
                    &conditional.condition,
                    canonical_storage,
                    external_storage,
                ) || Self::references_parameter_without_model_storage(
                    &conditional.then_expr,
                    canonical_storage,
                    external_storage,
                ) || Self::references_parameter_without_model_storage(
                    &conditional.else_expr,
                    canonical_storage,
                    external_storage,
                )
            }
            Expression::Call(call) => call.args.iter().any(|argument| {
                Self::references_parameter_without_model_storage(
                    argument,
                    canonical_storage,
                    external_storage,
                )
            }),
            Expression::SystemFunction(function) => function.args.iter().any(|argument| {
                Self::references_parameter_without_model_storage(
                    argument,
                    canonical_storage,
                    external_storage,
                )
            }),
            Expression::ArrayAccess(access) => {
                canonical_storage
                    .get(&access.array)
                    .is_some_and(|has_model_storage| !has_model_storage)
                    || Self::references_parameter_without_model_storage(
                        &access.index,
                        canonical_storage,
                        external_storage,
                    )
            }
            Expression::ArrayLiteral(array) => array.elements.iter().any(|element| {
                Self::array_element_references_parameter_without_model_storage(
                    element,
                    canonical_storage,
                    external_storage,
                )
            }),
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }

    fn array_element_references_parameter_without_model_storage(
        element: &ArrayLiteralElement,
        canonical_storage: &std::collections::HashMap<SmolStr, bool>,
        external_storage: &std::collections::HashMap<String, bool>,
    ) -> bool {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::references_parameter_without_model_storage(
                    expression,
                    canonical_storage,
                    external_storage,
                )
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::references_parameter_without_model_storage(
                    &replication.count,
                    canonical_storage,
                    external_storage,
                ) || replication.elements.iter().any(|element| {
                    Self::array_element_references_parameter_without_model_storage(
                        element,
                        canonical_storage,
                        external_storage,
                    )
                })
            }
        }
    }

    fn direct_parameter_reference(
        expression: &Expression,
        names: &std::collections::HashSet<SmolStr>,
    ) -> Option<SmolStr> {
        let Expression::Identifier(identifier) = expression else {
            return None;
        };
        names
            .contains(&identifier.name)
            .then(|| identifier.name.clone())
    }
}

/// Where to point a connect specification refusal.
///
/// [`crate::connect::ConnectError`] carries a span on the variants that have
/// one construct to blame, but several — an ambiguous rule, an excluded pair —
/// are about a *relation* between two. The first connect construct in the file
/// is the honest fallback: it is the block the reader has to look at either
/// way, and it beats pointing at offset zero.
fn connect_rules_span(source: &SourceFile) -> Span {
    source
        .items
        .iter()
        .find_map(|item| match item {
            Item::ConnectRules(block) => Some(block.span),
            Item::ConnectModule(module) => Some(module.span),
            _ => None,
        })
        .unwrap_or(source.span)
}

fn is_global_ground_name(name: &str) -> bool {
    name == "0"
}

fn is_zi_operator_name(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "zi_zp" | "zi_zd" | "zi_np" | "zi_nd"
    )
}

/// Canonical name of a source-level analog operator whose state must be
/// visited on every Newton iteration. Keep this list deliberately narrower
/// than the built-in-function registry: `ddx`, math, access, nature, and noise
/// calls are not stateful evaluation sites.
fn stateful_analog_operator_call_name(name: &str) -> Option<&'static str> {
    Some(match name.to_ascii_lowercase().as_str() {
        "ddt" => "ddt",
        "idt" => "idt",
        "idtmod" => "idtmod",
        "absdelay" => "absdelay",
        "transition" => "transition",
        "slew" => "slew",
        "cross" => "cross",
        "above" => "above",
        "last_crossing" => "last_crossing",
        "timer" => "timer",
        "laplace_zp" => "laplace_zp",
        "laplace_zd" => "laplace_zd",
        "laplace_np" => "laplace_np",
        "laplace_nd" => "laplace_nd",
        "zi_zp" => "zi_zp",
        "zi_zd" => "zi_zd",
        "zi_np" => "zi_np",
        "zi_nd" => "zi_nd",
        _ => return None,
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;
