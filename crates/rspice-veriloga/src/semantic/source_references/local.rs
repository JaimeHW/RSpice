//! Borrow an item a task, function or named block declares.
//!
//! IEEE 1364-2005 section 12.5 lets a hierarchical name pass through a task,
//! a function or a named block, which are scopes of their own, and end at a
//! variable or argument declared there: `ok.error`, `gen.foo_task.x`,
//! `top.blk.count`. A static routine's items and a named block's variables
//! have one copy per module instance (sections 10.2.1 and 9.8.1), so the
//! reference names storage as fixed as a module variable's. The referring
//! module borrows the item's declaration under the reference's symbol, as for
//! a net or variable, and digital lowering gives the item its storage on
//! demand and binds both sides to it.
use super::context::outer_named_blocks;
use super::physical::literal;
use super::*;
use crate::ast::{DigitalBlock, DigitalRoutineValueKind, DigitalStatement};
use crate::semantic::DigitalLocalReference;

/// A task, function or named block a reference has entered.
#[derive(Debug, Clone)]
pub(super) enum LocalScope {
    /// A task or function of the occurrence, by its name there: a generate
    /// block's routine by the name expansion qualified it with.
    Routine(SmolStr),
    /// The named blocks entered, outermost first, of the process at
    /// `process` in the occurrence's process list.
    Block { process: usize, path: Vec<SmolStr> },
}

/// The prefix generate expansion gave the names of the generate scope
/// `scope`: empty for the module itself, `gen[1].` below a loop.
fn scope_prefix(scope: &[HierarchicalScopeKey]) -> String {
    scope
        .iter()
        .map(|key| match key.index {
            Some(index) => format!("{}[{index}].", key.name),
            None => format!("{}.", key.name),
        })
        .collect()
}

/// The bounds IEEE 1364-2005 section 3.9 gives an `integer`: `[31:0]`, which
/// a borrowed declaration has to spell out.
pub(super) fn integer_range(span: Span) -> Option<VectorRange> {
    Some(VectorRange {
        msb: literal(31, span),
        lsb: literal(0, span),
        span,
    })
}

/// The named block `name` that `statement` opens directly.
fn named_block<'a>(statement: &'a DigitalStatement, name: &str) -> Option<&'a DigitalBlock> {
    let mut blocks = Vec::new();
    outer_named_blocks(statement, &mut blocks);
    blocks
        .into_iter()
        .find(|block| block.name.as_deref() == Some(name))
}

impl Resolver {
    /// The task, function or named block `key` names in the generate scope
    /// `scope` of `frame`, when it names one.
    pub(super) fn local_scope(
        &self,
        frame: usize,
        scope: &[HierarchicalScopeKey],
        key: &HierarchicalScopeKey,
    ) -> Option<LocalScope> {
        if key.index.is_some() {
            return None;
        }
        let source = &self.frames[frame].source;
        let prefix = scope_prefix(scope);
        let qualified = format!("{prefix}{}", key.name);
        if source
            .digital_routines
            .iter()
            .any(|routine| routine.name == qualified.as_str())
        {
            return Some(LocalScope::Routine(qualified.into()));
        }
        let generate_scope = prefix.trim_end_matches('.');
        source
            .digital_processes
            .iter()
            .enumerate()
            .filter(|(_, process)| {
                process.generate_scope.as_deref().unwrap_or("") == generate_scope
            })
            .find(|(_, process)| named_block(&process.body, &key.name).is_some())
            .map(|(process, _)| LocalScope::Block {
                process,
                path: vec![key.name.clone()],
            })
    }

    /// Enter the named block `key` below `local`, when it opens one there.
    pub(super) fn enter_local(
        &self,
        frame: usize,
        local: &mut LocalScope,
        key: &HierarchicalScopeKey,
    ) -> bool {
        let LocalScope::Block { process, path } = local else {
            return false;
        };
        if key.index.is_some() {
            return false;
        }
        let entered = self
            .innermost_block(frame, *process, path)
            .is_some_and(|block| {
                block
                    .statements
                    .iter()
                    .any(|statement| named_block(statement, &key.name).is_some())
            });
        if entered {
            path.push(key.name.clone());
        }
        entered
    }

    fn innermost_block(
        &self,
        frame: usize,
        process: usize,
        path: &[SmolStr],
    ) -> Option<&DigitalBlock> {
        let process = self.frames[frame].source.digital_processes.get(process)?;
        let (first, rest) = path.split_first()?;
        let mut block = named_block(&process.body, first)?;
        for name in rest {
            block = block
                .statements
                .iter()
                .find_map(|statement| named_block(statement, name))?;
        }
        Some(block)
    }

    /// Borrow the item `reference` ends at, inside `local` of `target`.
    pub(super) fn import_local(
        &mut self,
        owner: usize,
        target: usize,
        scope: &[HierarchicalScopeKey],
        local: &LocalScope,
        reference: &ScopedHierarchicalReference,
        symbol: &SmolStr,
    ) -> CompileResult<()> {
        let span = reference.source.span;
        let terminal = reference
            .source
            .segments
            .last()
            .expect("parsed terminal")
            .name
            .clone();
        let written = reference
            .source
            .segments
            .iter()
            .map(|segment| segment.name.as_str())
            .collect::<Vec<_>>()
            .join(".");
        let source = &self.frames[target].source;
        let constants = DigitalConstants::from_module(source);
        let time_scale = source.time_scale;
        let close = |expression: &Expression| -> CompileResult<Expression> {
            let value = super::super::node_vectors::integer(expression, &constants, time_scale)?;
            Ok(literal(value, span))
        };
        let close_range = |range: &Option<VectorRange>| -> CompileResult<Option<VectorRange>> {
            range
                .as_ref()
                .map(|range| {
                    Ok(VectorRange {
                        msb: close(&range.msb)?,
                        lsb: close(&range.lsb)?,
                        span,
                    })
                })
                .transpose()
        };
        let item = |dimensions: &[ArrayDimension]| -> CompileResult<Vec<DigitalDeclItem>> {
            if !dimensions.is_empty() {
                return Err(error(
                    format!(
                        "`{written}` names a memory a task, function or named block declares, \
                         and a reference to one is not supported yet"
                    ),
                    span,
                ));
            }
            Ok(vec![DigitalDeclItem {
                name: symbol.clone(),
                dimensions: Vec::new(),
                init: None,
                span,
            }])
        };
        let mut dependencies: Vec<&Expression> = Vec::new();
        let borrowed;
        let target_item;
        match local {
            LocalScope::Routine(name) => {
                let routine = source
                    .digital_routines
                    .iter()
                    .find(|routine| routine.name == *name)
                    .expect("entered routine");
                let kind = if routine.is_function() {
                    "function"
                } else {
                    "task"
                };
                if routine.automatic {
                    // Sections 10.2.3 (tasks) and 10.4.1 (functions).
                    return Err(invalid(
                        format!(
                            "`{written}` names an item of automatic {kind} `{name}`; IEEE \
                             1364-2005 sections 10.2.3 and 10.4.1 forbid a hierarchical \
                             reference to an automatic {kind}'s items"
                        ),
                        span,
                    ));
                }
                let typed = |ty: &crate::ast::DigitalRoutineType| -> DigitalVariableDecl {
                    let (kind, signedness, range) = match ty.kind {
                        DigitalRoutineValueKind::Reg => {
                            (DigitalVariableKind::Reg, ty.signedness, ty.range.clone())
                        }
                        DigitalRoutineValueKind::Integer => (
                            DigitalVariableKind::Integer,
                            Signedness::Signed,
                            integer_range(span),
                        ),
                        DigitalRoutineValueKind::Real => {
                            (DigitalVariableKind::Real, Signedness::Unsigned, None)
                        }
                        // Section 3.9: an unsigned 64-bit variable.
                        DigitalRoutineValueKind::Time => (
                            DigitalVariableKind::Reg,
                            Signedness::Unsigned,
                            Some(VectorRange {
                                msb: literal(63, span),
                                lsb: literal(0, span),
                                span,
                            }),
                        ),
                    };
                    DigitalVariableDecl {
                        kind,
                        signedness,
                        range,
                        items: Vec::new(),
                        span,
                    }
                };
                let short = name.rsplit('.').next().unwrap_or(name);
                let found = if let Some(argument) = routine
                    .arguments
                    .iter()
                    .find(|argument| argument.name == terminal)
                {
                    Some((typed(&argument.ty), argument.name.clone()))
                } else if let Some(declaration) = routine
                    .digital_variables
                    .iter()
                    .find(|declaration| declaration.items.iter().any(|item| item.name == terminal))
                {
                    let declared = declaration
                        .items
                        .iter()
                        .find(|item| item.name == terminal)
                        .unwrap();
                    item(&declared.dimensions)?;
                    Some((
                        DigitalVariableDecl {
                            kind: declaration.kind,
                            signedness: declaration.signedness,
                            range: declaration.range.clone(),
                            items: Vec::new(),
                            span,
                        },
                        terminal.clone(),
                    ))
                } else if let Some((declaration, declared)) =
                    routine.variables.iter().find_map(|declaration| {
                        declaration
                            .items
                            .iter()
                            .find(|item| item.name == terminal)
                            .map(|item| (declaration, item))
                    })
                {
                    item(&declared.dimensions)?;
                    let kind = match declaration.var_type {
                        VarType::Integer => DigitalVariableKind::Integer,
                        VarType::Real => DigitalVariableKind::Real,
                        VarType::String => {
                            return Err(error(
                                format!(
                                    "`{written}` names a string variable, which has no \
                                     discrete-domain storage"
                                ),
                                span,
                            ));
                        }
                    };
                    Some((
                        DigitalVariableDecl {
                            kind,
                            signedness: if kind == DigitalVariableKind::Integer {
                                Signedness::Signed
                            } else {
                                Signedness::Unsigned
                            },
                            range: (kind == DigitalVariableKind::Integer)
                                .then(|| integer_range(span))
                                .flatten(),
                            items: Vec::new(),
                            span,
                        },
                        terminal.clone(),
                    ))
                } else if let crate::ast::DigitalRoutineKind::Function(ty) = &routine.kind
                    && terminal == short
                {
                    // A function's result is a variable named as the function.
                    Some((typed(ty), routine.name.clone()))
                } else {
                    None
                };
                let Some((declaration, local_name)) = found else {
                    return Err(invalid(
                        format!("`{terminal}` is not declared in {kind} `{name}`"),
                        span,
                    ));
                };
                borrowed = declaration;
                target_item = DigitalLocalReference::Routine {
                    routine: name.clone(),
                    item: local_name,
                };
            }
            LocalScope::Block { process, path } => {
                let block = self
                    .innermost_block(target, *process, path)
                    .expect("entered block");
                let generate_scope = source.digital_processes[*process].generate_scope.clone();
                let place = path.join(".");
                if let Some(declaration) = block
                    .digital_variables
                    .iter()
                    .find(|declaration| declaration.items.iter().any(|item| item.name == terminal))
                {
                    let declared = declaration
                        .items
                        .iter()
                        .find(|item| item.name == terminal)
                        .unwrap();
                    item(&declared.dimensions)?;
                    borrowed = DigitalVariableDecl {
                        kind: declaration.kind,
                        signedness: declaration.signedness,
                        range: declaration.range.clone(),
                        items: Vec::new(),
                        span,
                    };
                    target_item = DigitalLocalReference::Block {
                        generate_scope,
                        declaration: declared.span,
                    };
                } else if let Some((declaration, declared)) =
                    block.variables.iter().find_map(|declaration| {
                        declaration
                            .items
                            .iter()
                            .find(|item| item.name == terminal)
                            .map(|item| (declaration, item))
                    })
                {
                    item(&declared.dimensions)?;
                    let kind = match declaration.var_type {
                        VarType::Integer => DigitalVariableKind::Integer,
                        VarType::Real => DigitalVariableKind::Real,
                        VarType::String => {
                            return Err(error(
                                format!(
                                    "`{written}` names a string variable, which has no \
                                     discrete-domain storage"
                                ),
                                span,
                            ));
                        }
                    };
                    borrowed = DigitalVariableDecl {
                        kind,
                        signedness: if kind == DigitalVariableKind::Integer {
                            Signedness::Signed
                        } else {
                            Signedness::Unsigned
                        },
                        range: (kind == DigitalVariableKind::Integer)
                            .then(|| integer_range(span))
                            .flatten(),
                        items: Vec::new(),
                        span,
                    };
                    target_item = DigitalLocalReference::Block {
                        generate_scope,
                        declaration: declared.span,
                    };
                } else {
                    return Err(invalid(
                        format!(
                            "`{terminal}` is not declared in named block `{place}` of {} (IEEE \
                             1364-2005 section 12.5)",
                            super::context::scope_place(source, scope)
                        ),
                        span,
                    ));
                }
            }
        }
        dependencies.extend(borrowed.range.iter().flat_map(|r| [&r.msb, &r.lsb]));
        let dependencies = SourceParameters::new(source).dependencies(dependencies)?;
        let mut borrowed = DigitalVariableDecl {
            range: close_range(&borrowed.range)?,
            items: item(&[])?,
            ..borrowed
        };
        borrowed.span = span;
        let storage: SmolStr = match &target_item {
            DigitalLocalReference::Routine { routine, item } => format!("{routine}.{item}"),
            DigitalLocalReference::Block { .. } => {
                let LocalScope::Block { path, process } = local else {
                    unreachable!("a block item is reached through a block")
                };
                let generate = source.digital_processes[*process]
                    .generate_scope
                    .as_deref()
                    .map_or_else(String::new, |scope| format!("{scope}."));
                format!("{generate}{}.{terminal}", path.join("."))
            }
        }
        .into();
        let path = self.reference_path(owner, target);
        self.retain_dependencies(owner, target, dependencies);
        let owner_source = &mut self.frames[owner].source;
        owner_source.digital_variables.push(borrowed);
        owner_source.foreign_digital.insert(
            symbol.clone(),
            ForeignDigitalReference {
                path,
                name: storage,
                span,
                local: Some(target_item),
            },
        );
        self.frames[owner].physical = None;
        Ok(())
    }
}
