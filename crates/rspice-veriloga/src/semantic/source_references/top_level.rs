//! A hierarchical name that starts at another top-level module of the source
//! (IEEE 1364-2005 section 12.5): `othertop.flag` from inside `top`.
//!
//! That module is a root of its own and is compiled on its own. The referring
//! module declares a copy of the named signal as storage of its own, and
//! linking the roots into one design joins the copy to the signal. A scope
//! argument records the instance's path, a parameter is read by value, and a
//! task or function name borrows the routine's declaration, which linking
//! replaces with that module's own routine.
use super::*;

impl Resolver {
    /// The frame of top-level module `name`: one the source declares and no
    /// module of it instantiates.
    pub(super) fn top_level(&mut self, name: &SmolStr) -> Option<usize> {
        if let Some(&frame) = self.top_levels.get(name) {
            return Some(frame);
        }
        if *name == self.frames[0].source.name
            || self
                .sources
                .modules
                .values()
                .any(|module| module.instantiates(name))
        {
            return None;
        }
        let mut source = self.sources.modules.get(name)?.clone();
        source.reference_sources = None;
        source.reference_context = Some(name.clone());
        let mut frame = Frame::new(source, name.clone(), 0);
        frame.top_level = Some(name.clone());
        let index = self.frames.len();
        self.frames.push(frame);
        self.top_levels.insert(name.clone(), index);
        Some(index)
    }

    /// Bind `symbol` of `owner` to what `reference` names under another
    /// top-level module: `target` and the generate scope in it.
    pub(super) fn import_top_level(
        &mut self,
        owner: usize,
        target: usize,
        scope: &[HierarchicalScopeKey],
        local: bool,
        reference: &ScopedHierarchicalReference,
        symbol: &SmolStr,
    ) -> CompileResult<()> {
        let span = reference.source.span;
        if local {
            let written: Vec<_> = reference
                .source
                .segments
                .iter()
                .map(|segment| segment.name.as_str())
                .collect();
            return Err(error(
                format!(
                    "`{}` reaches into a task, function or named block of another top-level \
                     module, which is not supported yet",
                    written.join(".")
                ),
                span,
            ));
        }
        let root = self.frames[target]
            .top_level
            .clone()
            .expect("a frame under another top-level module");
        let below = |path: &str| -> SmolStr {
            path.strip_prefix(root.as_str())
                .map_or(path, |rest| rest.trim_start_matches('.'))
                .into()
        };
        let name = self.terminal(target, scope, reference)?;
        let terminal = reference.source.segments.last().expect("parsed terminal");
        if let Some(&ordinal) = self.frames[target].instances.get(&name)
            && terminal.index.is_none()
        {
            let child = self.child(target, ordinal, span)?;
            let frame = &self.frames[child];
            let named = DigitalTopLevelReference {
                root: root.clone(),
                path: below(&frame.path),
                scope: Some((frame.source.name.clone(), frame.source.time_scale)),
                routine: None,
                span,
            };
            self.frames[owner]
                .source
                .foreign_top_levels
                .insert(symbol.clone(), named);
            return Ok(());
        }
        if let Some(routine) = self.frames[target]
            .source
            .digital_routines
            .iter()
            .find(|routine| routine.name == name)
        {
            // A call runs that module's own routine, in its scope (sections
            // 10.2.2 and 10.4.3); linking binds the call to it.
            let routine = self.borrowed_routine(target, routine, span)?;
            let within = below(&self.frames[target].path);
            self.frames[owner].source.foreign_top_levels.insert(
                symbol.clone(),
                DigitalTopLevelReference {
                    root,
                    path: if within.is_empty() {
                        name
                    } else {
                        format!("{within}.{name}").into()
                    },
                    scope: None,
                    routine: Some(Box::new(routine)),
                    span,
                },
            );
            return Ok(());
        }
        if !self.declares_digital(target, &name) {
            self.import_symbol(owner, target, &name, Some(symbol.clone()), span)?;
            return Ok(());
        }
        self.import_digital(owner, target, &name, symbol, span)?;
        let within = below(&self.frames[target].path);
        let owner_source = &mut self.frames[owner].source;
        // The borrowed declaration is this module's own storage here.
        owner_source.foreign_digital.remove(symbol);
        owner_source.foreign_top_levels.insert(
            symbol.clone(),
            DigitalTopLevelReference {
                root,
                path: if within.is_empty() {
                    name
                } else {
                    format!("{within}.{name}").into()
                },
                scope: None,
                routine: None,
                span,
            },
        );
        Ok(())
    }
    /// `routine` of `target` as a call from another top-level module sees
    /// it: its arguments and result, every range closed in `target`'s scope,
    /// and no body.
    fn borrowed_routine(
        &self,
        target: usize,
        routine: &DigitalRoutine,
        span: Span,
    ) -> CompileResult<DigitalRoutine> {
        let source = &self.frames[target].source;
        let constants = DigitalConstants::from_module(source);
        let time_scale = source.time_scale;
        let close = |ty: &DigitalRoutineType| -> CompileResult<DigitalRoutineType> {
            let bound = |expression: &Expression| -> CompileResult<Expression> {
                let value =
                    super::super::node_vectors::integer(expression, &constants, time_scale)?;
                Ok(super::physical::literal(value, span))
            };
            let range = match &ty.range {
                Some(range) => Some(VectorRange {
                    msb: bound(&range.msb)?,
                    lsb: bound(&range.lsb)?,
                    span,
                }),
                None => None,
            };
            Ok(DigitalRoutineType {
                kind: ty.kind,
                signedness: ty.signedness,
                range,
            })
        };
        Ok(DigitalRoutine {
            kind: match &routine.kind {
                crate::ast::DigitalRoutineKind::Task => crate::ast::DigitalRoutineKind::Task,
                crate::ast::DigitalRoutineKind::Function(ty) => {
                    crate::ast::DigitalRoutineKind::Function(Box::new(close(ty)?))
                }
            },
            automatic: routine.automatic,
            name: routine.name.clone(),
            arguments: routine
                .arguments
                .iter()
                .map(|argument| {
                    Ok(DigitalRoutineArgument {
                        direction: argument.direction,
                        ty: close(&argument.ty)?,
                        name: argument.name.clone(),
                        span: argument.span,
                    })
                })
                .collect::<CompileResult<_>>()?,
            variables: Vec::new(),
            digital_variables: Vec::new(),
            body: DigitalStatement::Null(span),
            span: routine.span,
        })
    }
}
