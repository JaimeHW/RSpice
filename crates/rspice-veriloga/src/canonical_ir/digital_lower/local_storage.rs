//! Promote only observable or deferred static locals into the shared signal store.
use super::super::digital::DigitalLocalStorage;
use super::*;

impl ProcessLowerer<'_> {
    pub(super) fn prepare_local_storage(
        &mut self,
        entry: BlockId,
        process: DigitalProcessId,
        statement: &DigitalStatement,
    ) {
        let mut required = BTreeSet::new();
        self.storage_dependencies(statement, &mut required);
        // Read every startup SSA definition before changing any local's representation.
        let initial: Vec<_> = required
            .into_iter()
            .map(|local| (local, self.read_local(entry, local)))
            .collect();
        for (local, value) in initial {
            let declaration = &self.locals[usize::from(local)];
            let name = declaration.name.clone().expect("source local");
            let mut storage_name = format!("$local:{process}:{local}:{name}");
            // Escaped source identifiers may contain arbitrary punctuation.
            while self
                .signals
                .iter()
                .any(|signal| signal.name == storage_name)
            {
                storage_name.insert(0, '$');
            }
            let signal = DigitalSignalId::from(self.signals.len());
            self.signals.push(DigitalSignal {
                local: Some(DigitalLocalStorage {
                    process,
                    declaration: local,
                    name,
                }),
                initial_value: None,
                id: signal,
                name: storage_name.into(),
                kind: if declaration.real {
                    DigitalSignalKind::Real(DigitalRealResolution::Single)
                } else {
                    DigitalSignalKind::FourState
                },
                width: declaration.width,
                bounds: declaration
                    .packed
                    .then_some((declaration.bounds.msb, declaration.bounds.lsb)),
                signed: declaration.signed,
                integer: declaration.integer,
                procedurally_assignable: true,
                span: declaration.span.into(),
            });
            self.locals[usize::from(local)].shared = Some(signal);
            self.write_local(entry, local, value);
        }
    }

    fn mark_names(&self, names: &BTreeSet<String>, required: &mut BTreeSet<DigitalLocalId>) {
        required.extend(names.iter().filter_map(|name| self.lookup_local(name)));
    }

    fn mark_target(&self, target: &DigitalLValue, required: &mut BTreeSet<DigitalLocalId>) {
        match target {
            DigitalLValue::Identifier { name, .. }
            | DigitalLValue::BitSelect { name, .. }
            | DigitalLValue::PartSelect { name, .. } => {
                if let Some(local) = self.lookup_local(name) {
                    required.insert(local);
                }
            }
            DigitalLValue::ArraySelect(access) => {
                if let Some(local) = self.lookup_local(&access.name) {
                    required.insert(local);
                }
            }
            DigitalLValue::Concat { elements, .. } => {
                for element in elements {
                    self.mark_target(element, required);
                }
            }
        }
    }

    fn mark_explicit_event(
        &self,
        event: &crate::ast::EventControl,
        required: &mut BTreeSet<DigitalLocalId>,
    ) {
        if let crate::ast::Sensitivity::Explicit(terms) = &event.sensitivity {
            let mut names = BTreeSet::new();
            for term in terms {
                collect_expression_reads(&term.signal, &mut names);
            }
            self.mark_names(&names, required);
        }
    }

    fn assignment_storage_dependencies(
        &self,
        assign: &DigitalAssign,
        nonblocking: bool,
        required: &mut BTreeSet<DigitalLocalId>,
    ) {
        if nonblocking {
            self.mark_target(&assign.target, required);
        }
        if let Some(TimingControl::Event(event)) = &assign.timing {
            self.mark_explicit_event(event, required);
            if matches!(event.sensitivity, crate::ast::Sensitivity::Implicit) {
                let mut names = BTreeSet::new();
                assignment_reads(assign, &mut names);
                self.mark_names(&names, required);
            }
        }
    }

    fn storage_dependencies(
        &mut self,
        statement: &DigitalStatement,
        required: &mut BTreeSet<DigitalLocalId>,
    ) {
        match statement {
            DigitalStatement::Block(block) => {
                let Some(scope) = self.static_scopes.get(&block.span).cloned() else {
                    return;
                };
                self.scopes.push(scope);
                for statement in &block.statements {
                    self.storage_dependencies(statement, required);
                }
                self.scopes.pop();
            }
            DigitalStatement::BlockingAssign(assign) => {
                self.assignment_storage_dependencies(assign, false, required)
            }
            DigitalStatement::NonblockingAssign(assign) => {
                self.assignment_storage_dependencies(assign, true, required)
            }
            DigitalStatement::Timing(timing) => {
                if let TimingControl::Event(event) = &timing.control {
                    self.mark_explicit_event(event, required);
                    if matches!(event.sensitivity, crate::ast::Sensitivity::Implicit) {
                        if let Some(guarded) = &timing.statement {
                            self.scoped_reads(guarded, required, &mut BTreeSet::new());
                        }
                    }
                }
                if let Some(statement) = &timing.statement {
                    self.storage_dependencies(statement, required);
                }
            }
            DigitalStatement::Conditional(conditional) => {
                self.storage_dependencies(&conditional.then_branch, required);
                if let Some(branch) = &conditional.else_branch {
                    self.storage_dependencies(branch, required);
                }
            }
            DigitalStatement::Case(case) => {
                for item in &case.items {
                    self.storage_dependencies(&item.statement, required);
                }
                if let Some(default) = &case.default {
                    self.storage_dependencies(default, required);
                }
            }
            DigitalStatement::For(statement) => {
                self.assignment_storage_dependencies(&statement.init, false, required);
                self.assignment_storage_dependencies(&statement.update, false, required);
                self.storage_dependencies(&statement.body, required);
            }
            DigitalStatement::While(statement) => {
                self.storage_dependencies(&statement.body, required)
            }
            DigitalStatement::Repeat(statement) => {
                self.storage_dependencies(&statement.body, required)
            }
            DigitalStatement::Forever(statement) => {
                self.storage_dependencies(&statement.body, required)
            }
            DigitalStatement::Null(_) => {}
        }
    }

    /// Resolve reads at their lexical occurrence, including nested declarations
    /// that shadow a name visible at the surrounding implicit event control.
    fn scoped_reads(
        &mut self,
        statement: &DigitalStatement,
        locals: &mut BTreeSet<DigitalLocalId>,
        module: &mut BTreeSet<String>,
    ) {
        if let DigitalStatement::Block(block) = statement {
            let Some(scope) = self.static_scopes.get(&block.span).cloned() else {
                return;
            };
            self.scopes.push(scope);
            for statement in &block.statements {
                self.scoped_reads(statement, locals, module);
            }
            self.scopes.pop();
            return;
        }
        let mut names = BTreeSet::new();
        let mut children = Vec::new();
        match statement {
            DigitalStatement::BlockingAssign(assign)
            | DigitalStatement::NonblockingAssign(assign) => {
                assignment_reads(assign, &mut names);
            }
            DigitalStatement::Conditional(conditional) => {
                collect_expression_reads(&conditional.condition, &mut names);
                children.push(conditional.then_branch.as_ref());
                children.extend(conditional.else_branch.as_deref());
            }
            DigitalStatement::Case(case) => {
                collect_expression_reads(&case.selector, &mut names);
                for item in &case.items {
                    for label in &item.labels {
                        collect_expression_reads(label, &mut names);
                    }
                    children.push(&item.statement);
                }
                children.extend(case.default.as_deref());
            }
            DigitalStatement::For(statement) => {
                assignment_reads(&statement.init, &mut names);
                assignment_reads(&statement.update, &mut names);
                collect_expression_reads(&statement.condition, &mut names);
                children.push(statement.body.as_ref());
            }
            DigitalStatement::While(statement) => {
                collect_expression_reads(&statement.condition, &mut names);
                children.push(statement.body.as_ref());
            }
            DigitalStatement::Repeat(statement) => {
                collect_expression_reads(&statement.count, &mut names);
                children.push(statement.body.as_ref());
            }
            DigitalStatement::Forever(statement) => children.push(statement.body.as_ref()),
            DigitalStatement::Timing(timing) => {
                if let TimingControl::Delay(delay) = &timing.control {
                    collect_expression_reads(&delay.value, &mut names);
                }
                children.extend(timing.statement.as_deref());
            }
            DigitalStatement::Null(_) => {}
            DigitalStatement::Block(_) => unreachable!("block handled above"),
        }
        for name in names {
            if let Some(local) = self.lookup_local(&name) {
                locals.insert(local);
            } else {
                module.insert(name);
            }
        }
        for child in children {
            self.scoped_reads(child, locals, module);
        }
    }

    pub(super) fn scoped_read_dependencies(
        &mut self,
        statement: &DigitalStatement,
    ) -> BTreeSet<DigitalSignalId> {
        let mut locals = BTreeSet::new();
        let mut module = BTreeSet::new();
        self.scoped_reads(statement, &mut locals, &mut module);
        locals
            .into_iter()
            .filter_map(|local| self.locals[usize::from(local)].shared)
            // Resolve module bindings directly: an outer shadow must not replace
            // a read already resolved in a nested lexical region.
            .chain(module.into_iter().flat_map(|name| {
                self.index
                    .get(name.as_str())
                    .into_iter()
                    .flat_map(|signal| {
                        self.arrays.get(signal).map_or_else(
                            || vec![*signal],
                            |array| {
                                array
                                    .cell_range()
                                    .into_iter()
                                    .flatten()
                                    .map(DigitalSignalId::new)
                                    .collect()
                            },
                        )
                    })
                    .collect::<Vec<_>>()
            }))
            .collect()
    }
}

fn assignment_reads(assign: &DigitalAssign, reads: &mut BTreeSet<String>) {
    collect_expression_reads(&assign.value, reads);
    match &assign.timing {
        Some(TimingControl::Delay(delay)) => collect_expression_reads(&delay.value, reads),
        Some(TimingControl::Event(event)) => {
            if let Some(count) = &event.repeat {
                collect_expression_reads(count, reads);
            }
        }
        None => {}
    }
    collect_lvalue_index_reads(&assign.target, reads);
}
