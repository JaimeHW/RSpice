//! Close analog functions over their declaring occurrence, before inlining.
use super::*;

type Locals = HashMap<SmolStr, SmolStr>;

impl Resolver {
    pub(super) fn import_function(
        &mut self,
        owner: usize,
        target: usize,
        mut function: FunctionDef,
        symbol: &SmolStr,
    ) -> CompileResult<()> {
        // Function locals must remain disjoint from the importing module's
        // symbols, especially genvars consulted during analog loop lowering.
        let mut locals = Locals::new();
        for parameter in &mut function.params {
            let name = self.fresh_import(owner);
            locals.insert(parameter.name.clone(), name.clone());
            parameter.name = name;
        }
        locals.insert(function.name.clone(), symbol.clone());
        function.name = symbol.clone();
        self.declare_locals(owner, &function.locals, &mut locals, false);
        self.bind_function_declarations(owner, target, &mut function.locals, &locals)?;
        for statement in &mut function.body.statements {
            self.bind_function_statement(owner, target, statement, &locals)?;
        }
        self.frames[owner].source.functions.push(function);
        Ok(())
    }

    fn declare_locals(
        &mut self,
        owner: usize,
        declarations: &[VariableDecl],
        locals: &mut Locals,
        shadow: bool,
    ) {
        for declaration in declarations {
            for item in &declaration.items {
                if shadow || !locals.contains_key(&item.name) {
                    locals.insert(item.name.clone(), self.fresh_import(owner));
                }
            }
        }
    }

    fn bind_function_declarations(
        &mut self,
        owner: usize,
        target: usize,
        declarations: &mut [VariableDecl],
        locals: &Locals,
    ) -> CompileResult<()> {
        for declaration in declarations {
            for item in &mut declaration.items {
                item.name = locals[&item.name].clone();
                for dimension in &mut item.dimensions {
                    self.bind_function_expression(owner, target, &mut dimension.start, locals)?;
                    self.bind_function_expression(owner, target, &mut dimension.end, locals)?;
                }
                if let Some(initializer) = &mut item.init {
                    self.bind_function_expression(owner, target, initializer, locals)?;
                }
            }
        }
        Ok(())
    }

    fn function_reference_target(
        &mut self,
        owner: usize,
        target: usize,
        name: &SmolStr,
    ) -> CompileResult<(usize, SmolStr)> {
        let Some(reference) = self.frames[target]
            .source
            .pending_hierarchical_references
            .get(name)
            .cloned()
        else {
            return Ok((target, name.clone()));
        };
        let dependencies = SourceParameters::new(&self.frames[target].source)
            .dependencies(reference.index_dependencies.iter())?;
        self.retain_dependencies(owner, target, dependencies);
        self.target(target, &reference)
    }

    fn bind_function_value(
        &mut self,
        owner: usize,
        target: usize,
        name: &mut SmolStr,
        span: Span,
        locals: &Locals,
    ) -> CompileResult<()> {
        if let Some(local) = locals.get(name) {
            *name = local.clone();
            return Ok(());
        }
        let (target, declaration) = self.function_reference_target(owner, target, name)?;
        let source = &self.frames[target].source;
        if !source
            .parameters
            .iter()
            .chain(&source.localparams)
            .any(|parameter| parameter.name == declaration)
        {
            return Err(error(
                format!(
                    "analog function cannot capture `{declaration}`: only arguments, local variables and parameters are permitted (VAMS-2023 4.7)"
                ),
                span,
            ));
        }
        *name = self.import_symbol(owner, target, &declaration, None, span)?;
        Ok(())
    }

    fn bind_function_call(
        &mut self,
        owner: usize,
        target: usize,
        call: &mut CallExpr,
    ) -> CompileResult<()> {
        if call.resolved_builtin {
            return Ok(());
        }
        let (target, name) = self.function_reference_target(owner, target, &call.name)?;
        if self.frames[target]
            .source
            .functions
            .iter()
            .any(|function| function.name == name)
        {
            call.name = self.import_symbol(owner, target, &name, None, call.span)?;
        } else if let Some(signature) = self.builtins.get(&name) {
            if signature.is_analog_operator || stateful_analog_operator_call_name(&name).is_some() {
                return Err(error(
                    "analog operators are not permitted inside analog functions",
                    call.span,
                ));
            }
            call.name = name;
            call.resolved_builtin = true;
        } else {
            return Err(error(
                format!("function `{name}` is not declared in the target occurrence"),
                call.span,
            ));
        }
        Ok(())
    }

    fn bind_function_expression(
        &mut self,
        owner: usize,
        target: usize,
        expression: &mut Expression,
        locals: &Locals,
    ) -> CompileResult<()> {
        let given = parameter_given::GivenParameters::new(&self.frames[target].source);
        let (folded, dependencies) = given
            .fold(expression)
            .map_err(|message| error(message, expression.span()))?;
        if let Cow::Owned(folded) = folded {
            *expression = folded;
        }
        self.retain_dependencies(
            owner,
            target,
            ParameterDependencies {
                values: HashSet::new(),
                given: dependencies,
            },
        );
        let mut pending = vec![expression];
        while let Some(expression) = pending.pop() {
            let span = expression.span();
            match expression {
                Expression::Identifier(identifier) => {
                    self.bind_function_value(owner, target, &mut identifier.name, span, locals)?;
                }
                Expression::ArrayAccess(access) => {
                    self.bind_function_value(owner, target, &mut access.array, span, locals)?;
                }
                Expression::Digital(DigitalExpr::PartSelect(select)) => {
                    self.bind_function_value(owner, target, &mut select.name, span, locals)?;
                }
                Expression::Digital(DigitalExpr::ArraySelect(select)) => {
                    self.bind_function_value(owner, target, &mut select.name, span, locals)?;
                }
                Expression::Call(call) => self.bind_function_call(owner, target, call)?,
                Expression::SystemFunction(function)
                    if function.name == "$limit"
                        || stateful_analog_operator_call_name(
                            function.name.trim_start_matches('$'),
                        )
                        .is_some() =>
                {
                    return Err(error(
                        "analog operators are not permitted inside analog functions",
                        span,
                    ));
                }
                Expression::BranchAccess(_)
                | Expression::AnalogOperator(_)
                | Expression::NoiseSource(_) => {
                    return Err(error(
                        "analog functions cannot contain branch probes, analog operators or noise sources",
                        span,
                    ));
                }
                _ => {}
            }
            super::super::flow_probes::for_child_mut(expression, &mut |child| pending.push(child));
        }
        Ok(())
    }

    fn bind_function_assignment(
        &mut self,
        owner: usize,
        target: usize,
        assignment: &mut AssignmentStmt,
        locals: &Locals,
    ) -> CompileResult<()> {
        let (name, span) = match &mut assignment.target {
            LValue::Variable { name, span } => (name, *span),
            LValue::ArrayAccess {
                name,
                index,
                additional_indices,
                span,
                ..
            } => {
                self.bind_function_expression(owner, target, index, locals)?;
                for index in additional_indices {
                    self.bind_function_expression(owner, target, index, locals)?;
                }
                (name, *span)
            }
        };
        *name = locals.get(name).cloned().ok_or_else(|| {
            error(
                "analog functions may assign only their own variables and arguments",
                span,
            )
        })?;
        self.bind_function_expression(owner, target, &mut assignment.value, locals)
    }

    fn bind_function_statement(
        &mut self,
        owner: usize,
        target: usize,
        statement: &mut AnalogStatement,
        locals: &Locals,
    ) -> CompileResult<()> {
        match statement {
            AnalogStatement::Assignment(assignment) => {
                self.bind_function_assignment(owner, target, assignment, locals)?;
            }
            AnalogStatement::Block(block) => {
                if block.name.is_some() {
                    return Err(error(
                        "named blocks are not permitted inside analog functions",
                        block.span,
                    ));
                }
                let mut nested = locals.clone();
                self.declare_locals(owner, &block.variables, &mut nested, true);
                self.bind_function_declarations(owner, target, &mut block.variables, &nested)?;
                for statement in &mut block.statements {
                    self.bind_function_statement(owner, target, statement, &nested)?;
                }
            }
            AnalogStatement::Conditional(conditional) => {
                self.bind_function_expression(owner, target, &mut conditional.condition, locals)?;
                self.bind_function_statement(owner, target, &mut conditional.then_branch, locals)?;
                if let Some(branch) = &mut conditional.else_branch {
                    self.bind_function_statement(owner, target, branch, locals)?;
                }
            }
            AnalogStatement::Case(case) => {
                self.bind_function_expression(owner, target, &mut case.expr, locals)?;
                for item in &mut case.items {
                    for value in &mut item.matches {
                        self.bind_function_expression(owner, target, value, locals)?;
                    }
                    self.bind_function_statement(owner, target, &mut item.statement, locals)?;
                }
                if let Some(default) = &mut case.default {
                    self.bind_function_statement(owner, target, default, locals)?;
                }
            }
            AnalogStatement::For(loop_) => {
                loop_.var = locals.get(&loop_.var).cloned().ok_or_else(|| {
                    error("analog function loop variable must be local", loop_.span)
                })?;
                self.bind_function_expression(owner, target, &mut loop_.init, locals)?;
                self.bind_function_expression(owner, target, &mut loop_.condition, locals)?;
                self.bind_function_assignment(owner, target, &mut loop_.update, locals)?;
                self.bind_function_statement(owner, target, &mut loop_.body, locals)?;
            }
            AnalogStatement::While(loop_) => {
                self.bind_function_expression(owner, target, &mut loop_.condition, locals)?;
                self.bind_function_statement(owner, target, &mut loop_.body, locals)?;
            }
            AnalogStatement::Repeat(loop_) => {
                self.bind_function_expression(owner, target, &mut loop_.count, locals)?;
                self.bind_function_statement(owner, target, &mut loop_.body, locals)?;
            }
            AnalogStatement::Call(call) => {
                if !call.name.starts_with('$') {
                    let mut expression = CallExpr {
                        resolved_builtin: false,
                        name: call.name.clone(),
                        args: Vec::new(),
                        span: call.span,
                    };
                    self.bind_function_call(owner, target, &mut expression)?;
                    call.name = expression.name;
                }
                for argument in &mut call.args {
                    self.bind_function_expression(owner, target, argument, locals)?;
                }
            }
            AnalogStatement::Null(_) => {}
            AnalogStatement::Contribution(value) => {
                return Err(error(
                    "analog functions cannot contribute to branches",
                    value.span,
                ));
            }
            AnalogStatement::IndirectContribution(value) => {
                return Err(error(
                    "analog functions cannot contribute to branches",
                    value.span,
                ));
            }
            AnalogStatement::EventControl(value) => {
                return Err(error(
                    "analog functions cannot contain event controls",
                    value.span,
                ));
            }
            AnalogStatement::Disable(value) => {
                return Err(error(
                    "analog functions cannot contain disable statements",
                    value.span,
                ));
            }
        }
        Ok(())
    }
}
