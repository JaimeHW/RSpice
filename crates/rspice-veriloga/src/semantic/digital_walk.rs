//! Walk expression roots in discrete statements without recursive statement
//! traversal. Expression-tree traversal is supplied by the caller, so analysis
//! and rewriting share the same statement, assignment and timing coverage.

use crate::ast::*;

pub(super) fn visit_roots(body: &DigitalStatement, visit: &mut impl FnMut(&Expression)) {
    enum Work<'a> {
        Statement(&'a DigitalStatement),
        Assignment(&'a DigitalAssign),
        Target(&'a DigitalLValue),
        Timing(&'a TimingControl),
    }
    let mut pending = vec![Work::Statement(body)];
    while let Some(work) = pending.pop() {
        match work {
            Work::Statement(statement) => match statement {
                DigitalStatement::Block(block) => {
                    for declaration in &block.variables {
                        for item in &declaration.items {
                            if let Some(value) = &item.init {
                                visit(value);
                            }
                        }
                    }
                    for declaration in &block.digital_variables {
                        for item in &declaration.items {
                            if let Some(value) = &item.init {
                                visit(value);
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
                    visit(&conditional.condition);
                    if let Some(branch) = &conditional.else_branch {
                        pending.push(Work::Statement(branch));
                    }
                    pending.push(Work::Statement(&conditional.then_branch));
                }
                DigitalStatement::Case(case) => {
                    visit(&case.selector);
                    if let Some(branch) = &case.default {
                        pending.push(Work::Statement(branch));
                    }
                    for item in &case.items {
                        for label in &item.labels {
                            visit(label);
                        }
                        pending.push(Work::Statement(&item.statement));
                    }
                }
                DigitalStatement::For(loop_) => {
                    visit(&loop_.condition);
                    pending.push(Work::Assignment(&loop_.init));
                    pending.push(Work::Assignment(&loop_.update));
                    pending.push(Work::Statement(&loop_.body));
                }
                DigitalStatement::While(loop_) => {
                    visit(&loop_.condition);
                    pending.push(Work::Statement(&loop_.body));
                }
                DigitalStatement::Repeat(loop_) => {
                    visit(&loop_.count);
                    pending.push(Work::Statement(&loop_.body));
                }
                DigitalStatement::Forever(loop_) => pending.push(Work::Statement(&loop_.body)),
                DigitalStatement::Timing(timing) => {
                    pending.push(Work::Timing(&timing.control));
                    if let Some(statement) = &timing.statement {
                        pending.push(Work::Statement(statement));
                    }
                }
                DigitalStatement::Null(_) => {}
            },
            Work::Assignment(assign) => {
                visit(&assign.value);
                pending.push(Work::Target(&assign.target));
                if let Some(timing) = &assign.timing {
                    pending.push(Work::Timing(timing));
                }
            }
            Work::Target(target) => match target {
                DigitalLValue::Identifier { .. } => {}
                DigitalLValue::BitSelect { index, .. } => visit(index),
                DigitalLValue::PartSelect { msb, lsb, .. } => {
                    visit(msb);
                    visit(lsb);
                }
                DigitalLValue::Concat { elements, .. } => {
                    pending.extend(elements.iter().map(Work::Target))
                }
            },
            Work::Timing(timing) => match timing {
                TimingControl::Delay(delay) => visit(&delay.value),
                TimingControl::Event(event) => {
                    if let Some(count) = &event.repeat {
                        visit(count);
                    }
                    if let Sensitivity::Explicit(terms) = &event.sensitivity {
                        for term in terms {
                            visit(&term.signal);
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
