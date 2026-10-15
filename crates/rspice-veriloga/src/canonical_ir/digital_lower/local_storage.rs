//! Promote only observable or deferred static locals into the shared signal store.
use super::super::digital::DigitalLocalStorage;
use super::*;

impl ProcessLowerer<'_> {
    pub(super) fn analog_local_signal(&mut self, local: DigitalLocalId) -> Option<DigitalSignalId> {
        let declaration = &self.locals[usize::from(local)];
        let name = self.analog_local_inputs.get(&declaration.span)?;
        if let Some(signal) = self.index.get(name.as_str()) {
            Some(*signal)
        } else {
            self.invariant("analog local input has no shared signal", declaration.span);
            None
        }
    }

    pub(super) fn bind_local_identity(
        &mut self,
        local: DigitalLocalId,
        signal: DigitalSignalId,
        element: Option<i64>,
    ) {
        self.signals[usize::from(signal)].local = Some(DigitalLocalStorage {
            element,
            process: self
                .process
                .expect("local declaration belongs to a process"),
            declaration: local,
            name: self.locals[usize::from(local)]
                .name
                .clone()
                .expect("source local"),
        });
    }

    pub(super) fn prepare_local_storage(
        &mut self,
        entry: BlockId,
        process: DigitalProcessId,
        statement: &DigitalStatement,
    ) {
        let mut required = BTreeSet::new();
        self.storage_dependencies(statement, &mut required);
        // Forked branches share their parent's variables (IEEE 1364-2005
        // section 9.8.2), and a branch starts with no value of its parent's
        // in hand, so every named variable of a forking process is stored.
        if contains_fork(statement) {
            required.extend(
                self.locals
                    .iter()
                    .enumerate()
                    .filter(|(_, local)| local.name.is_some())
                    .map(|(id, _)| DigitalLocalId::from(id)),
            );
        }
        required.extend(self.locals.iter().enumerate().filter_map(|(id, local)| {
            self.analog_local_inputs
                .contains_key(&local.span)
                .then_some(DigitalLocalId::from(id))
        }));
        // Read every startup SSA definition before changing any local's representation.
        let initial: Vec<_> = required
            .into_iter()
            .filter(|local| self.locals[usize::from(*local)].shared.is_none())
            .collect::<Vec<_>>()
            .into_iter()
            .map(|local| (local, self.read_local(entry, local)))
            .collect();
        for (local, value) in initial {
            if let Some(signal) = self.analog_local_signal(local) {
                self.bind_local_identity(local, signal, None);
                self.locals[usize::from(local)].shared = Some(signal);
                self.write_local(entry, local, value);
                continue;
            }
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
                    element: None,
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
            DigitalLValue::IndexedPartSelect(select) => {
                if let Some(local) = self.lookup_local(&select.name) {
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
            // A `$strobe`/`$monitor` reads its arguments in the monitor region,
            // outside any activation of this process, so every local it names
            // must live in signal storage (design P2 section 5.2).
            DigitalStatement::SystemTask(task)
                if crate::canonical_ir::digital::DigitalSystemCallKind::from_name(&task.name)
                    .is_some_and(|kind| kind.is_observer()) =>
            {
                let mut names = BTreeSet::new();
                for argument in &task.args {
                    collect_expression_reads(argument, &mut names);
                }
                self.mark_names(&names, required);
            }
            DigitalStatement::Fork(block) => {
                for statement in &block.statements {
                    self.storage_dependencies(statement, required);
                }
            }
            DigitalStatement::Wait(wait) => {
                // The condition is an event expression the thread waits on.
                let mut names = BTreeSet::new();
                collect_expression_reads(&wait.condition, &mut names);
                self.mark_names(&names, required);
                if let Some(statement) = &wait.statement {
                    self.storage_dependencies(statement, required);
                }
            }
            // A procedural continuous assignment's right-hand side is
            // re-evaluated outside any activation of this process, and its
            // target is overridden in the store: both live in signal storage.
            DigitalStatement::ProceduralContinuous(statement) => {
                let mut names = BTreeSet::new();
                for (name, _) in statement.target.written_names() {
                    names.insert(name.to_string());
                }
                if let Some(value) = &statement.value {
                    collect_expression_reads(value, &mut names);
                }
                self.mark_names(&names, required);
            }
            DigitalStatement::SystemTask(_)
            | DigitalStatement::TaskEnable(_)
            | DigitalStatement::Disable(_)
            | DigitalStatement::EventTrigger(_)
            | DigitalStatement::Null(_) => {}
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
                self.assignment_event_reads(assign, &mut names);
            }
            DigitalStatement::Conditional(conditional) => {
                self.event_expression_reads(&conditional.condition, &mut names);
                children.push(conditional.then_branch.as_ref());
                children.extend(conditional.else_branch.as_deref());
            }
            DigitalStatement::Case(case) => {
                self.event_expression_reads(&case.selector, &mut names);
                for item in &case.items {
                    for label in &item.labels {
                        self.event_expression_reads(label, &mut names);
                    }
                    children.push(&item.statement);
                }
                children.extend(case.default.as_deref());
            }
            DigitalStatement::For(statement) => {
                self.assignment_event_reads(&statement.init, &mut names);
                self.assignment_event_reads(&statement.update, &mut names);
                self.event_expression_reads(&statement.condition, &mut names);
                children.push(statement.body.as_ref());
            }
            DigitalStatement::While(statement) => {
                self.event_expression_reads(&statement.condition, &mut names);
                children.push(statement.body.as_ref());
            }
            DigitalStatement::Repeat(statement) => {
                self.event_expression_reads(&statement.count, &mut names);
                children.push(statement.body.as_ref());
            }
            DigitalStatement::Forever(statement) => children.push(statement.body.as_ref()),
            DigitalStatement::Timing(timing) => {
                if let TimingControl::Delay(delay) = &timing.control {
                    self.event_expression_reads(&delay.value, &mut names);
                }
                children.extend(timing.statement.as_deref());
            }
            DigitalStatement::SystemTask(task) => {
                for argument in &task.args {
                    self.event_expression_reads(argument, &mut names);
                }
            }
            DigitalStatement::TaskEnable(enable) => {
                for argument in &enable.args {
                    self.event_expression_reads(argument, &mut names);
                }
            }
            DigitalStatement::Fork(block) => children.extend(&block.statements),
            DigitalStatement::Wait(wait) => {
                self.event_expression_reads(&wait.condition, &mut names);
                children.extend(wait.statement.as_deref());
            }
            DigitalStatement::ProceduralContinuous(statement) => {
                if let Some(value) = &statement.value {
                    self.event_expression_reads(value, &mut names);
                }
            }
            DigitalStatement::Disable(_)
            | DigitalStatement::EventTrigger(_)
            | DigitalStatement::Null(_) => {}
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
            .flat_map(|local| self.local_read_dependencies(local))
            // Resolve module bindings directly: an outer shadow must not replace
            // a read already resolved in a nested lexical region.
            .chain(
                module
                    .into_iter()
                    .flat_map(|name| self.module_read_dependencies(&name)),
            )
            .collect()
    }
    fn event_expression_reads(&mut self, expression: &Expression, reads: &mut BTreeSet<String>) {
        self.validate_analog_event_reads(expression);
        collect_expression_reads(expression, reads);
    }
    fn assignment_event_reads(&mut self, assign: &DigitalAssign, reads: &mut BTreeSet<String>) {
        self.event_expression_reads(&assign.value, reads);
        match &assign.timing {
            Some(TimingControl::Delay(delay)) => self.event_expression_reads(&delay.value, reads),
            Some(TimingControl::Event(event)) => {
                if let Some(count) = &event.repeat {
                    self.event_expression_reads(count, reads);
                }
            }
            None => {}
        }
        collect_lvalue_index_reads(&assign.target, reads);
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

/// Whether `statement`, or any statement nested in it, satisfies `test`.
pub(super) fn any_statement(
    statement: &DigitalStatement,
    test: &impl Fn(&DigitalStatement) -> bool,
) -> bool {
    if test(statement) {
        return true;
    }
    let nested = |statement: &DigitalStatement| any_statement(statement, test);
    match statement {
        DigitalStatement::Block(block) | DigitalStatement::Fork(block) => {
            block.statements.iter().any(nested)
        }
        DigitalStatement::Conditional(conditional) => {
            nested(&conditional.then_branch)
                || conditional.else_branch.as_deref().is_some_and(nested)
        }
        DigitalStatement::Case(case) => {
            case.items.iter().any(|item| nested(&item.statement))
                || case.default.as_deref().is_some_and(nested)
        }
        DigitalStatement::For(statement) => nested(&statement.body),
        DigitalStatement::While(statement) => nested(&statement.body),
        DigitalStatement::Repeat(statement) => nested(&statement.body),
        DigitalStatement::Forever(statement) => nested(&statement.body),
        DigitalStatement::Timing(timing) => timing.statement.as_deref().is_some_and(nested),
        DigitalStatement::Wait(wait) => wait.statement.as_deref().is_some_and(nested),
        _ => false,
    }
}

/// Whether a body forks (IEEE 1364-2005 section 9.8.2).
pub(super) fn contains_fork(statement: &DigitalStatement) -> bool {
    any_statement(statement, &|statement| {
        matches!(statement, DigitalStatement::Fork(_))
    })
}
