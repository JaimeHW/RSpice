//! Decide once per module which analog functions require ordered execution.
//! Follow callers transitively so a pure-looking wrapper cannot erase a task
//! in its callee. Names are case-sensitive, as required by the language.

use super::*;

pub(super) fn effectful_functions(functions: &HashMap<SmolStr, FunctionDef>) -> HashSet<SmolStr> {
    let mut callers: HashMap<SmolStr, Vec<SmolStr>> = HashMap::new();
    let mut effectful = HashSet::new();
    let mut pending = Vec::new();
    for (name, function) in functions {
        let mut effects = Effects::default();
        effects.declarations(&function.locals);
        for statement in &function.body.statements {
            effects.statement(statement);
        }
        for (callee, _) in effects.calls {
            callers.entry(callee).or_default().push(name.clone());
        }
        if effects.ordered
            || function
                .params
                .iter()
                .any(|param| param.direction != ParamDirection::Input)
        {
            effectful.insert(name.clone());
            pending.push(name.clone());
        }
    }
    while let Some(callee) = pending.pop() {
        for caller in callers.get(&callee).into_iter().flatten() {
            if effectful.insert(caller.clone()) {
                pending.push(caller.clone());
            }
        }
    }
    effectful
}

#[derive(Default)]
struct Effects {
    initialization_error: Option<(Span, &'static str)>,
    ordered: bool,
    calls: std::collections::BTreeMap<SmolStr, Span>,
}

impl Effects {
    fn declarations(&mut self, declarations: &[VariableDecl]) {
        for declaration in declarations {
            for item in &declaration.items {
                if let Some(initializer) = &item.init {
                    self.expression(initializer);
                }
            }
        }
    }

    fn assignment(&mut self, assignment: &AssignmentStmt) {
        self.expression(&assignment.value);
        if let LValue::ArrayAccess { index, .. } = &assignment.target {
            self.expression(index);
        }
    }

    fn statement(&mut self, statement: &AnalogStatement) {
        match statement {
            AnalogStatement::Call(call) => {
                self.ordered |= call.name.starts_with('$');
                if call.name == "$stop" {
                    self.initialization_error
                        .get_or_insert((call.span, "$stop"));
                }
                self.calls.entry(call.name.clone()).or_insert(call.span);
                for argument in &call.args {
                    self.expression(argument);
                }
            }
            AnalogStatement::Assignment(assignment) => self.assignment(assignment),
            AnalogStatement::Block(block) => {
                self.declarations(&block.variables);
                for statement in &block.statements {
                    self.statement(statement);
                }
            }
            AnalogStatement::Conditional(conditional) => {
                self.expression(&conditional.condition);
                self.statement(&conditional.then_branch);
                if let Some(otherwise) = &conditional.else_branch {
                    self.statement(otherwise);
                }
            }
            AnalogStatement::Case(case) => {
                self.expression(&case.expr);
                for item in &case.items {
                    for value in &item.matches {
                        self.expression(value);
                    }
                    self.statement(&item.statement);
                }
                if let Some(default) = &case.default {
                    self.statement(default);
                }
            }
            AnalogStatement::For(loop_) => {
                self.expression(&loop_.init);
                self.expression(&loop_.condition);
                self.assignment(&loop_.update);
                self.statement(&loop_.body);
            }
            AnalogStatement::While(loop_) => {
                self.expression(&loop_.condition);
                self.statement(&loop_.body);
            }
            AnalogStatement::Repeat(loop_) => {
                self.expression(&loop_.count);
                self.statement(&loop_.body);
            }
            // These statements need the statement analyzer's legality checks;
            // they cannot be replaced by a symbolic function return value.
            AnalogStatement::Contribution(_)
            | AnalogStatement::IndirectContribution(_)
            | AnalogStatement::EventControl(_) => {
                self.ordered = true;
                let span = match statement {
                    AnalogStatement::Contribution(statement) => statement.span,
                    AnalogStatement::IndirectContribution(statement) => statement.span,
                    AnalogStatement::EventControl(statement) => statement.span,
                    _ => unreachable!(),
                };
                self.initialization_error
                    .get_or_insert((span, "contributions and analog events"));
            }
            AnalogStatement::Disable(_) => self.ordered = true,
            AnalogStatement::Null(_) => {}
        }
    }

    fn expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Call(call) => {
                self.calls.entry(call.name.clone()).or_insert(call.span);
                for argument in &call.args {
                    self.expression(argument);
                }
            }
            Expression::SystemFunction(call) => {
                self.calls.entry(call.name.clone()).or_insert(call.span);
                for argument in &call.args {
                    self.expression(argument);
                }
            }
            Expression::Binary(binary) => {
                self.expression(&binary.left);
                self.expression(&binary.right);
            }
            Expression::Unary(unary) => self.expression(&unary.operand),
            Expression::Conditional(conditional) => {
                self.expression(&conditional.condition);
                self.expression(&conditional.then_expr);
                self.expression(&conditional.else_expr);
            }
            Expression::ArrayAccess(access) => self.expression(&access.index),
            Expression::ArrayLiteral(array) => self.elements(&array.elements),
            Expression::Digital(digital) => {
                for child in digital.children() {
                    self.expression(child);
                }
            }
            Expression::AnalogOperator(_) | Expression::NoiseSource(_) => {
                self.ordered = true;
                self.initialization_error
                    .get_or_insert((expression.span(), "analog operators and noise sources"));
            }
            Expression::BranchAccess(_) => {
                self.initialization_error
                    .get_or_insert((expression.span(), "analog access functions"));
            }
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_) => {}
        }
    }

    fn elements(&mut self, elements: &[ArrayLiteralElement]) {
        for element in elements {
            match element {
                ArrayLiteralElement::Value(value) => self.expression(value),
                ArrayLiteralElement::Replication(replication) => {
                    self.expression(&replication.count);
                    self.elements(&replication.elements);
                }
            }
        }
    }
}

/// Validate authored initializer syntax, including unreachable branches and
/// transitive function calls, before constant folding can erase an illegal use.
pub(super) fn validate_initialization(
    statements: &[AnalogStatement],
    functions: &HashMap<SmolStr, FunctionDef>,
) -> CompileResult<()> {
    let mut effects = Effects::default();
    for statement in statements {
        effects.statement(statement);
    }
    validate_initialization_effects(effects, functions)
}

pub(super) fn validate_initializer_expression(
    expression: &Expression,
    functions: &HashMap<SmolStr, FunctionDef>,
) -> CompileResult<()> {
    let mut effects = Effects::default();
    effects.expression(expression);
    validate_initialization_effects(effects, functions)
}

fn validate_initialization_effects(
    mut effects: Effects,
    functions: &HashMap<SmolStr, FunctionDef>,
) -> CompileResult<()> {
    let mut visited = HashSet::new();
    let builtins = FunctionRegistry::new();
    loop {
        if let Some((span, construct)) = effects.initialization_error {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "{construct} are not permitted during pre-simulation initialization"
                )),
                span,
            )));
        }
        let pending = std::mem::take(&mut effects.calls);
        if pending.is_empty() {
            return Ok(());
        }
        for (name, span) in pending {
            if !functions.contains_key(&name)
                && (builtins
                    .get(&name)
                    .is_some_and(|function| function.is_analog_operator)
                    || matches!(
                        name.as_str(),
                        "limexp"
                            | "transition"
                            | "last_crossing"
                            | "cross"
                            | "above"
                            | "timer"
                            | "zi_nd"
                            | "zi_np"
                            | "zi_zd"
                            | "zi_zp"
                            | "noise_table"
                            | "$limit"
                    ))
            {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(format!(
                        "analog operator '{name}' is not permitted during pre-simulation initialization"
                    )),
                    span,
                )));
            }
            if visited.insert(name.clone())
                && let Some(function) = functions.get(&name)
            {
                effects.declarations(&function.locals);
                for statement in &function.body.statements {
                    effects.statement(statement);
                }
            }
        }
    }
}
