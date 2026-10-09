//! Concrete declarations and lexical substitutions in generated scopes.
use super::*;

type Shadow = (SmolStr, Option<SmolStr>, Option<i64>);

impl Unroller<'_> {
    pub(super) fn expand_scoped_declarations(&mut self, items: &Module, out: &mut Module) {
        for declaration in &items.localparams {
            let mut copy = declaration.clone();
            self.rename(&mut copy.name);
            self.substitute_range(&mut copy.packed_range);
            self.substitute_dimensions(&mut copy.dimensions);
            if let Some(value) = &mut copy.default {
                self.substitute(value);
            }
            if let Some(range) = &mut copy.range {
                for bound in &mut range.bounds {
                    if let Some(value) = &mut bound.lower {
                        self.substitute(value);
                    }
                    if let Some(value) = &mut bound.upper {
                        self.substitute(value);
                    }
                }
                for value in &mut range.exclude {
                    self.substitute(value);
                }
            }
            for attribute in &mut copy.attributes {
                if let Some(value) = &mut attribute.value {
                    self.substitute(value);
                }
            }
            self.constants.definitions.push(copy.clone());
            out.localparams.push(copy);
        }
        for declaration in &items.genvars {
            let mut copy = declaration.clone();
            for name in &mut copy.names {
                self.rename(name);
            }
            self.genvars.extend(copy.names.iter().cloned());
            out.genvars.push(copy);
        }
        for declaration in &items.digital_variables {
            let mut copy = declaration.clone();
            self.substitute_range(&mut copy.range);
            self.substitute_digital_items(&mut copy.items, true);
            out.digital_variables.push(copy);
        }
        for declaration in &items.variables {
            let mut copy = declaration.clone();
            self.substitute_variables(&mut copy, true);
            out.variables.push(copy);
        }
        for declaration in &items.branches {
            let mut copy = declaration.clone();
            self.rename(&mut copy.name);
            self.rename(&mut copy.pos);
            self.rename(&mut copy.neg);
            self.substitute_range(&mut copy.range);
            for value in copy.pos_prefix.iter_mut().chain(&mut copy.neg_prefix) {
                self.substitute(value);
            }
            for selector in [&mut copy.pos_select, &mut copy.neg_select]
                .into_iter()
                .flatten()
            {
                match selector {
                    PackedSelect::Bit(value) => self.substitute(value),
                    PackedSelect::Part { msb, lsb } => {
                        self.substitute(msb);
                        self.substitute(lsb);
                    }
                }
            }
            out.branches.push(copy);
        }
        for function in &items.functions {
            let mut copy = function.clone();
            self.rename(&mut copy.name);
            let locals =
                copy.params
                    .iter()
                    .map(|argument| argument.name.clone())
                    .chain(copy.locals.iter().flat_map(|declaration| {
                        declaration.items.iter().map(|item| item.name.clone())
                    }))
                    .collect::<Vec<_>>();
            let shadows = self.hide(locals);
            for declaration in &mut copy.locals {
                self.substitute_variables(declaration, false);
            }
            for statement in &mut copy.body.statements {
                self.substitute_analog(statement);
            }
            self.restore(shadows);
            out.functions.push(copy);
        }
        for (source, slot) in [
            (&items.analog_block, &mut out.analog_block),
            (&items.analog_initial, &mut out.analog_initial),
            (&items.analog_final, &mut out.analog_final),
        ] {
            if let Some(source) = source {
                let mut copy = source.clone();
                for statement in &mut copy.statements {
                    self.substitute_analog(statement);
                }
                Parser::merge_analog_block(slot, copy);
            }
        }
    }

    pub(super) fn substitute_range(&self, range: &mut Option<VectorRange>) {
        if let Some(range) = range {
            self.substitute(&mut range.msb);
            self.substitute(&mut range.lsb);
        }
    }

    fn substitute_dimensions(&self, dimensions: &mut [ArrayDimension]) {
        for dimension in dimensions {
            self.substitute(&mut dimension.start);
            self.substitute(&mut dimension.end);
        }
    }

    pub(super) fn substitute_variables(&self, declaration: &mut VariableDecl, qualify: bool) {
        for item in &mut declaration.items {
            if qualify {
                self.rename(&mut item.name);
            }
            self.substitute_dimensions(&mut item.dimensions);
            if let Some(value) = &mut item.init {
                self.substitute(value);
            }
        }
    }

    pub(super) fn substitute_digital_items(&self, items: &mut [DigitalDeclItem], qualify: bool) {
        for item in items {
            if qualify {
                self.rename(&mut item.name);
            }
            self.substitute_dimensions(&mut item.dimensions);
            if let Some(value) = &mut item.init {
                self.substitute(value);
            }
        }
    }

    fn hide(&mut self, names: Vec<SmolStr>) -> Vec<Shadow> {
        names
            .into_iter()
            .map(|name| {
                let scoped = self.scoped_names.remove(&name);
                let index = self.bindings.remove(&name);
                (name, scoped, index)
            })
            .collect()
    }

    fn restore(&mut self, shadows: Vec<Shadow>) {
        for (name, scoped, index) in shadows.into_iter().rev() {
            if let Some(scoped) = scoped {
                self.scoped_names.insert(name.clone(), scoped);
            }
            if let Some(index) = index {
                self.bindings.insert(name, index);
            }
        }
    }

    fn substitute_branch(&self, branch: &mut BranchAccess) {
        // Reuse the expression traversal for terminal, array and branch selectors.
        let mut expression = Expression::BranchAccess(branch.clone());
        self.substitute(&mut expression);
        let Expression::BranchAccess(resolved) = expression else {
            unreachable!()
        };
        *branch = resolved;
    }

    fn substitute_assignment(&self, assignment: &mut AssignmentStmt) {
        match &mut assignment.target {
            LValue::Variable { name, .. } => self.rename(name),
            LValue::ArrayAccess {
                name,
                index,
                additional_indices,
                ..
            } => {
                self.rename(name);
                self.substitute(index);
                for value in additional_indices {
                    self.substitute(value);
                }
            }
        }
        self.substitute(&mut assignment.value);
    }

    fn substitute_analog(&mut self, statement: &mut AnalogStatement) {
        match statement {
            AnalogStatement::Contribution(value) => {
                self.substitute_branch(&mut value.target);
                self.substitute(&mut value.value);
            }
            AnalogStatement::IndirectContribution(value) => {
                self.substitute_branch(&mut value.branch);
                self.substitute(&mut value.lhs);
                self.substitute(&mut value.rhs);
            }
            AnalogStatement::Assignment(value) => self.substitute_assignment(value),
            AnalogStatement::Conditional(value) => {
                self.substitute(&mut value.condition);
                self.substitute_analog(&mut value.then_branch);
                if let Some(branch) = &mut value.else_branch {
                    self.substitute_analog(branch);
                }
            }
            AnalogStatement::Case(value) => {
                self.substitute(&mut value.expr);
                for item in &mut value.items {
                    for value in &mut item.matches {
                        self.substitute(value);
                    }
                    self.substitute_analog(&mut item.statement);
                }
                if let Some(branch) = &mut value.default {
                    self.substitute_analog(branch);
                }
            }
            AnalogStatement::For(value) => {
                self.rename(&mut value.var);
                self.substitute(&mut value.init);
                self.substitute(&mut value.condition);
                self.substitute_assignment(&mut value.update);
                self.substitute_analog(&mut value.body);
            }
            AnalogStatement::While(value) => {
                self.substitute(&mut value.condition);
                self.substitute_analog(&mut value.body);
            }
            AnalogStatement::Repeat(value) => {
                self.substitute(&mut value.count);
                self.substitute_analog(&mut value.body);
            }
            AnalogStatement::Block(block) => {
                let locals = block
                    .variables
                    .iter()
                    .flat_map(|declaration| declaration.items.iter().map(|item| item.name.clone()))
                    .collect();
                let shadows = self.hide(locals);
                let block_binding = block.name.as_mut().map(|name| {
                    let original = name.clone();
                    *name = format!("{}{name}", self.scope_prefix).into();
                    let previous = self.scoped_names.insert(original.clone(), name.clone());
                    (original, previous)
                });
                for declaration in &mut block.variables {
                    self.substitute_variables(declaration, false);
                }
                for statement in &mut block.statements {
                    self.substitute_analog(statement);
                }
                if let Some((name, previous)) = block_binding {
                    if let Some(previous) = previous {
                        self.scoped_names.insert(name, previous);
                    } else {
                        self.scoped_names.remove(&name);
                    }
                }
                self.restore(shadows);
            }
            AnalogStatement::EventControl(value) => {
                self.substitute_event(&mut value.event);
                self.substitute_analog(&mut value.statement);
            }
            AnalogStatement::Call(value) => {
                self.rename(&mut value.name);
                for argument in &mut value.args {
                    self.substitute(argument);
                }
            }
            AnalogStatement::Disable(value) => self.rename(&mut value.name),
            AnalogStatement::Null(_) => {}
        }
    }

    fn substitute_event(&self, event: &mut EventExpr) {
        match event {
            EventExpr::Posedge { signal, .. } | EventExpr::Negedge { signal, .. } => {
                self.substitute(signal)
            }
            EventExpr::Cross {
                signal,
                direction,
                time_tol,
                expr_tol,
                enable,
                ..
            } => {
                self.substitute(signal);
                for value in [direction, time_tol, expr_tol, enable]
                    .into_iter()
                    .flatten()
                {
                    self.substitute(value);
                }
            }
            EventExpr::Above {
                signal,
                time_tol,
                expr_tol,
                enable,
                ..
            } => {
                self.substitute(signal);
                for value in [time_tol, expr_tol, enable].into_iter().flatten() {
                    self.substitute(value);
                }
            }
            EventExpr::Timer {
                start,
                period,
                time_tol,
                enable,
                ..
            } => {
                self.substitute(start);
                for value in [period, time_tol, enable].into_iter().flatten() {
                    self.substitute(value);
                }
            }
            EventExpr::Or { left, right, .. } => {
                self.substitute_event(left);
                self.substitute_event(right);
            }
            EventExpr::InitialStep { .. } | EventExpr::FinalStep { .. } => {}
        }
    }

    pub(super) fn qualify_noise_label(&self, call: &mut CallExpr) {
        let index = match call.name.as_str() {
            "white_noise" | "noise_table" | "noise_table_log" => 1,
            "flicker_noise" => 2,
            _ => return,
        };
        if let Some(Expression::StringLit(label)) = call.args.get_mut(index) {
            label.value = format!("{}{}", self.scope_prefix, label.value).into();
        }
    }
}

pub(super) fn analog_span(statement: &AnalogStatement) -> Span {
    match statement {
        AnalogStatement::Contribution(value) => value.span,
        AnalogStatement::IndirectContribution(value) => value.span,
        AnalogStatement::Conditional(value) => value.span,
        AnalogStatement::Case(value) => value.span,
        AnalogStatement::For(value) => value.span,
        AnalogStatement::While(value) => value.span,
        AnalogStatement::Repeat(value) => value.span,
        AnalogStatement::Block(value) => value.span,
        AnalogStatement::Assignment(value) => value.span,
        AnalogStatement::EventControl(value) => value.span,
        AnalogStatement::Call(value) => value.span,
        AnalogStatement::Disable(value) => value.span,
        AnalogStatement::Null(span) => *span,
    }
}

/// Order authored items within one lexical occurrence, keeping each nested
/// generate construct together as an already ordered group (VAMS-2023 6.9.1).
pub(super) fn sort_analog_items(module: &mut Module) {
    for block in [
        &mut module.analog_block,
        &mut module.analog_initial,
        &mut module.analog_final,
    ]
    .into_iter()
    .flatten()
    {
        block
            .statements
            .sort_by_key(|statement| analog_span(statement).start);
    }
}

pub(super) fn group_analog_items(module: &mut Module, span: Span) {
    for block in [
        &mut module.analog_block,
        &mut module.analog_initial,
        &mut module.analog_final,
    ]
    .into_iter()
    .flatten()
    {
        let statements = std::mem::take(&mut block.statements);
        block.statements.push(AnalogStatement::Block(BlockStmt {
            name: None,
            variables: Vec::new(),
            statements,
            span,
        }));
        block.span = span;
    }
}
