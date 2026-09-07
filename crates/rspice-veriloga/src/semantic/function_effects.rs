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
        for callee in effects.calls {
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
    ordered: bool,
    calls: HashSet<SmolStr>,
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
                self.calls.insert(call.name.clone());
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
            | AnalogStatement::EventControl(_)
            | AnalogStatement::Disable(_) => self.ordered = true,
            AnalogStatement::Null(_) => {}
        }
    }

    fn expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Call(call) => {
                self.calls.insert(call.name.clone());
                for argument in &call.args {
                    self.expression(argument);
                }
            }
            Expression::SystemFunction(call) => {
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
            Expression::AnalogOperator(_) | Expression::NoiseSource(_) => self.ordered = true,
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_) => {}
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
