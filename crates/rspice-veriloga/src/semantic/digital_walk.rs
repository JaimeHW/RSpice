//! Walk expression roots in discrete statements without recursive statement
//! traversal. Expression-tree traversal is supplied by the caller, so analysis
//! and rewriting share the same statement, assignment and timing coverage.

use crate::ast::*;

pub(super) fn visit_roots(body: &DigitalStatement, visit: &mut impl FnMut(&Expression)) {
    walk_roots(body, &Default::default(), false, &mut |expr, _| visit(expr));
}

/// Every read root with the lexical locals that shadow module declarations.
pub(super) fn visit_scoped_roots(
    body: &DigitalStatement,
    visit: &mut impl FnMut(&Expression, &std::collections::BTreeSet<smol_str::SmolStr>),
) {
    walk_roots(body, &Default::default(), false, visit);
}

/// Read roots contributing to an implicit event control, with lexical shadows
/// resolved before the caller selects module storage. Declarations initialize
/// once, and explicit nested sensitivity terms do not extend the outer read set.
pub(super) fn visit_sensitivity_roots(
    body: &DigitalStatement,
    locals: &std::collections::BTreeSet<smol_str::SmolStr>,
    visit: &mut impl FnMut(&Expression, &std::collections::BTreeSet<smol_str::SmolStr>),
) {
    walk_roots(body, locals, true, visit);
}

fn walk_roots(
    body: &DigitalStatement,
    initial_locals: &std::collections::BTreeSet<smol_str::SmolStr>,
    sensitivity_only: bool,
    visit: &mut impl FnMut(&Expression, &std::collections::BTreeSet<smol_str::SmolStr>),
) {
    enum Work<'a> {
        Statement(&'a DigitalStatement),
        Assignment(&'a DigitalAssign),
        Target(&'a DigitalLValue),
        Timing(&'a TimingControl, bool),
        Leave(Vec<smol_str::SmolStr>),
    }
    let mut locals = initial_locals.clone();
    let mut pending = vec![Work::Statement(body)];
    while let Some(work) = pending.pop() {
        match work {
            Work::Leave(names) => {
                for name in names {
                    locals.remove(&name);
                }
            }
            Work::Statement(statement) => match statement {
                DigitalStatement::Block(block) => {
                    let names = block
                        .variables
                        .iter()
                        .flat_map(|decl| &decl.items)
                        .map(|item| item.name.clone())
                        .chain(
                            block
                                .digital_variables
                                .iter()
                                .flat_map(|decl| &decl.items)
                                .map(|item| item.name.clone()),
                        )
                        .filter(|name| locals.insert(name.clone()))
                        .collect();
                    pending.push(Work::Leave(names));
                    if !sensitivity_only {
                        for declaration in &block.variables {
                            for item in &declaration.items {
                                if let Some(value) = &item.init {
                                    visit(value, &locals);
                                }
                            }
                        }
                        for declaration in &block.digital_variables {
                            for item in &declaration.items {
                                if let Some(value) = &item.init {
                                    visit(value, &locals);
                                }
                            }
                        }
                    }
                    pending.extend(block.statements.iter().rev().map(Work::Statement));
                }
                DigitalStatement::BlockingAssign(assign)
                | DigitalStatement::NonblockingAssign(assign) => {
                    pending.push(Work::Assignment(assign))
                }
                DigitalStatement::Conditional(conditional) => {
                    visit(&conditional.condition, &locals);
                    if let Some(branch) = &conditional.else_branch {
                        pending.push(Work::Statement(branch));
                    }
                    pending.push(Work::Statement(&conditional.then_branch));
                }
                DigitalStatement::Case(case) => {
                    visit(&case.selector, &locals);
                    if let Some(branch) = &case.default {
                        pending.push(Work::Statement(branch));
                    }
                    for item in &case.items {
                        for label in &item.labels {
                            visit(label, &locals);
                        }
                        pending.push(Work::Statement(&item.statement));
                    }
                }
                DigitalStatement::For(loop_) => {
                    visit(&loop_.condition, &locals);
                    pending.push(Work::Assignment(&loop_.init));
                    pending.push(Work::Assignment(&loop_.update));
                    pending.push(Work::Statement(&loop_.body));
                }
                DigitalStatement::While(loop_) => {
                    visit(&loop_.condition, &locals);
                    pending.push(Work::Statement(&loop_.body));
                }
                DigitalStatement::Repeat(loop_) => {
                    visit(&loop_.count, &locals);
                    pending.push(Work::Statement(&loop_.body));
                }
                DigitalStatement::Forever(loop_) => pending.push(Work::Statement(&loop_.body)),
                DigitalStatement::Timing(timing) => {
                    pending.push(Work::Timing(&timing.control, !sensitivity_only));
                    if let Some(statement) = &timing.statement {
                        pending.push(Work::Statement(statement));
                    }
                }
                DigitalStatement::Null(_) => {}
            },
            Work::Assignment(assign) => {
                visit(&assign.value, &locals);
                pending.push(Work::Target(&assign.target));
                if let Some(timing) = &assign.timing {
                    pending.push(Work::Timing(timing, true));
                }
            }
            Work::Target(target) => match target {
                DigitalLValue::Identifier { .. } => {}
                DigitalLValue::ArraySelect(select) => {
                    for child in select.children() {
                        visit(child, &locals);
                    }
                }
                DigitalLValue::BitSelect { index, .. } => visit(index, &locals),
                DigitalLValue::PartSelect { msb, lsb, .. } => {
                    visit(msb, &locals);
                    visit(lsb, &locals);
                }
                DigitalLValue::Concat { elements, .. } => {
                    pending.extend(elements.iter().map(Work::Target))
                }
            },
            Work::Timing(timing, read_repeat) => match timing {
                TimingControl::Delay(delay) => visit(&delay.value, &locals),
                TimingControl::Event(event) => {
                    if read_repeat && let Some(count) = &event.repeat {
                        visit(count, &locals);
                    }
                    if !sensitivity_only && let Sensitivity::Explicit(terms) = &event.sensitivity {
                        for term in terms {
                            visit(&term.signal, &locals);
                        }
                    }
                }
            },
        }
    }
}

pub(super) fn rewrite_roots(body: &mut DigitalStatement, visit: &mut impl FnMut(&mut Expression)) {
    enum Work<'a> {
        Statement(&'a mut DigitalStatement),
        Assignment(&'a mut DigitalAssign),
        Target(&'a mut DigitalLValue),
        Timing(&'a mut TimingControl),
    }
    let mut pending = vec![Work::Statement(body)];
    while let Some(work) = pending.pop() {
        match work {
            Work::Statement(statement) => match statement {
                DigitalStatement::Block(block) => {
                    for declaration in &mut block.variables {
                        for item in &mut declaration.items {
                            if let Some(value) = &mut item.init {
                                visit(value);
                            }
                        }
                    }
                    for declaration in &mut block.digital_variables {
                        for item in &mut declaration.items {
                            if let Some(value) = &mut item.init {
                                visit(value);
                            }
                        }
                    }
                    pending.extend(block.statements.iter_mut().rev().map(Work::Statement));
                }
                DigitalStatement::BlockingAssign(assign)
                | DigitalStatement::NonblockingAssign(assign) => {
                    pending.push(Work::Assignment(assign))
                }
                DigitalStatement::Conditional(conditional) => {
                    visit(&mut conditional.condition);
                    if let Some(branch) = &mut conditional.else_branch {
                        pending.push(Work::Statement(branch));
                    }
                    pending.push(Work::Statement(&mut conditional.then_branch));
                }
                DigitalStatement::Case(case) => {
                    visit(&mut case.selector);
                    if let Some(branch) = &mut case.default {
                        pending.push(Work::Statement(branch));
                    }
                    for item in &mut case.items {
                        for label in &mut item.labels {
                            visit(label);
                        }
                        pending.push(Work::Statement(&mut item.statement));
                    }
                }
                DigitalStatement::For(loop_) => {
                    visit(&mut loop_.condition);
                    pending.push(Work::Assignment(&mut loop_.init));
                    pending.push(Work::Assignment(&mut loop_.update));
                    pending.push(Work::Statement(&mut loop_.body));
                }
                DigitalStatement::While(loop_) => {
                    visit(&mut loop_.condition);
                    pending.push(Work::Statement(&mut loop_.body));
                }
                DigitalStatement::Repeat(loop_) => {
                    visit(&mut loop_.count);
                    pending.push(Work::Statement(&mut loop_.body));
                }
                DigitalStatement::Forever(loop_) => pending.push(Work::Statement(&mut loop_.body)),
                DigitalStatement::Timing(timing) => {
                    pending.push(Work::Timing(&mut timing.control));
                    if let Some(statement) = &mut timing.statement {
                        pending.push(Work::Statement(statement));
                    }
                }
                DigitalStatement::Null(_) => {}
            },
            Work::Assignment(assign) => {
                visit(&mut assign.value);
                pending.push(Work::Target(&mut assign.target));
                if let Some(timing) = &mut assign.timing {
                    pending.push(Work::Timing(timing));
                }
            }
            Work::Target(target) => match target {
                DigitalLValue::Identifier { .. } => {}
                DigitalLValue::ArraySelect(select) => {
                    for child in select.children_mut() {
                        visit(child);
                    }
                }
                DigitalLValue::BitSelect { index, .. } => visit(index),
                DigitalLValue::PartSelect { msb, lsb, .. } => {
                    visit(msb);
                    visit(lsb);
                }
                DigitalLValue::Concat { elements, .. } => {
                    pending.extend(elements.iter_mut().map(Work::Target))
                }
            },
            Work::Timing(timing) => match timing {
                TimingControl::Delay(delay) => visit(&mut delay.value),
                TimingControl::Event(event) => {
                    if let Some(count) = &mut event.repeat {
                        visit(count);
                    }
                    if let Sensitivity::Explicit(terms) = &mut event.sensitivity {
                        for term in terms {
                            visit(&mut term.signal);
                        }
                    }
                }
            },
        }
    }
}

/// Collect assignments to module storage without mistaking lexical locals for
/// same-named module variables. Inactive branches still establish ownership.
pub(super) fn collect_module_writes(
    body: &DigitalStatement,
    written: &mut std::collections::HashSet<smol_str::SmolStr>,
) {
    enum Work<'a> {
        Statement(&'a DigitalStatement),
        Assignment(&'a DigitalAssign),
        Leave(Vec<&'a smol_str::SmolStr>),
    }
    let mut locals = std::collections::HashMap::<&smol_str::SmolStr, usize>::new();
    let mut pending = vec![Work::Statement(body)];
    while let Some(work) = pending.pop() {
        match work {
            Work::Leave(names) => {
                for name in names {
                    let depth = locals.get_mut(name).expect("entered local scope");
                    *depth -= 1;
                    if *depth == 0 {
                        locals.remove(name);
                    }
                }
            }
            Work::Assignment(assign) => {
                for (name, _) in assign.target.written_names() {
                    if !locals.contains_key(name) {
                        written.insert(name.clone());
                    }
                }
            }
            Work::Statement(statement) => match statement {
                DigitalStatement::Null(_) => {}
                DigitalStatement::Block(block) => {
                    let names: Vec<_> = block
                        .variables
                        .iter()
                        .flat_map(|decl| decl.items.iter().map(|item| &item.name))
                        .chain(
                            block
                                .digital_variables
                                .iter()
                                .flat_map(|decl| decl.items.iter().map(|item| &item.name)),
                        )
                        .collect();
                    for name in &names {
                        *locals.entry(name).or_default() += 1;
                    }
                    pending.push(Work::Leave(names));
                    pending.extend(block.statements.iter().rev().map(Work::Statement));
                }
                DigitalStatement::BlockingAssign(assign)
                | DigitalStatement::NonblockingAssign(assign) => {
                    pending.push(Work::Assignment(assign))
                }
                DigitalStatement::Conditional(conditional) => {
                    if let Some(branch) = &conditional.else_branch {
                        pending.push(Work::Statement(branch));
                    }
                    pending.push(Work::Statement(&conditional.then_branch));
                }
                DigitalStatement::Case(case) => {
                    if let Some(branch) = &case.default {
                        pending.push(Work::Statement(branch));
                    }
                    pending.extend(
                        case.items
                            .iter()
                            .map(|item| Work::Statement(&item.statement)),
                    );
                }
                DigitalStatement::For(loop_) => {
                    pending.push(Work::Assignment(&loop_.init));
                    pending.push(Work::Assignment(&loop_.update));
                    pending.push(Work::Statement(&loop_.body));
                }
                DigitalStatement::While(loop_) => pending.push(Work::Statement(&loop_.body)),
                DigitalStatement::Repeat(loop_) => pending.push(Work::Statement(&loop_.body)),
                DigitalStatement::Forever(loop_) => pending.push(Work::Statement(&loop_.body)),
                DigitalStatement::Timing(timing) => {
                    if let Some(statement) = &timing.statement {
                        pending.push(Work::Statement(statement));
                    }
                }
            },
        }
    }
}
