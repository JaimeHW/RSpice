//! Select and bind concrete source occurrences before either domain checks bodies.
use super::*;
use crate::ast::{DigitalBlock, DigitalStatement, GenerateBlock, GenerateConstruct};

impl Resolver {
    fn new(sources: Sources, mut root: Module, selected: bool) -> CompileResult<Self> {
        validate_parameter_references(&root)?;
        root.reference_sources = None;
        root.reference_context = selected.then(|| "".into());
        extend_insertions(&sources, &mut root, "");
        let specialization = root
            .parameters
            .iter()
            .enumerate()
            .filter(|(_, parameter)| parameter.is_given)
            .filter_map(|(index, parameter)| parameter.default.as_ref().map(|value| (index, value)))
            .map(|(index, value)| {
                super::super::instance_parameters::override_identity(value)
                    .map(|identity| (index, identity))
                    .map_err(|detail| error(detail, value.span()))
            })
            .collect::<CompileResult<Vec<_>>>()?;
        let mut frame = Frame::new(root, "".into(), 0);
        frame.specialization = specialization;
        Ok(Self {
            sources,
            selected,
            building: HashSet::new(),
            frames: vec![frame],
            children: HashMap::new(),
            active: HashSet::new(),
            resolved: HashSet::new(),
            imports: HashMap::new(),
            active_imports: HashSet::new(),
            next_import: 0,
            builtins: crate::types::FunctionRegistry::new(),
        })
    }

    pub(super) fn reference_path(&self, owner: usize, target: usize) -> SmolStr {
        let path = &self.frames[target].path;
        if self.selected {
            return path.clone();
        }
        path.strip_prefix(self.frames[owner].path.as_str())
            .expect("unselected resolution only follows descendants")
            .trim_start_matches('.')
            .into()
    }

    /// Local scopes shadow outer scopes, even when the remaining path is invalid.
    pub(super) fn start_scope(
        &self,
        owner: usize,
        reference: &ScopedHierarchicalReference,
    ) -> CompileResult<(usize, Vec<HierarchicalScopeKey>, usize)> {
        let span = reference.source.span;
        let first = reference
            .scopes
            .first()
            .ok_or_else(|| error("missing instance path", span))?;
        if reference.source.absolute {
            if !self.selected {
                return Err(error(
                    "absolute $root paths require a selected design",
                    span,
                ));
            }
            if first.name != self.frames[0].source.name || first.index.is_some() {
                return Err(error(
                    format!(
                        "$root must start with selected top-level module `{}`",
                        self.frames[0].source.name
                    ),
                    span,
                ));
            }
            return Ok((0, Vec::new(), 1));
        }
        let mut frame = owner;
        let mut origin = reference.origin.as_slice();
        loop {
            let current = &self.frames[frame];
            if let Some(scope) = (0..=origin.len())
                .rev()
                .map(|depth| origin[..depth].to_vec())
                .find(|scope| {
                    current.member(scope, &first.name).is_some()
                        || current.has_scope(scope, &first.name)
                })
            {
                return Ok((frame, scope, 0));
            }
            if self.selected
                && first.index.is_none()
                && (first.name == current.source.name
                    || first.name == current.instance_name
                    || self.sources.design.as_ref().is_some_and(|design| {
                        design.module_names.get(&current.source.name) == Some(&first.name)
                    }))
            {
                return Ok((frame, Vec::new(), 1));
            }
            let Some(parent) = current.parent.filter(|_| self.selected) else {
                break;
            };
            origin = &current.parent_scope;
            frame = parent;
        }
        Err(self.unresolved_scope(owner, reference))
    }

    /// Why the first scope of `reference` names nothing the resolver can
    /// enter: a source error where the language rules the reference out, a
    /// refusal naming the construct where RSpice does not follow it yet.
    fn unresolved_scope(
        &self,
        owner: usize,
        reference: &ScopedHierarchicalReference,
    ) -> CompileError {
        let span = reference.source.span;
        let first = &reference.scopes[0];
        let terminal = reference
            .source
            .segments
            .get(reference.scopes.len())
            .map_or("", |segment| segment.name.as_str());
        // A task or function of this module or an enclosing one, searched the
        // way IEEE 1364-2005 section 12.6 searches upward.
        let mut frame = Some(owner);
        while let Some(index) = frame {
            let current = &self.frames[index];
            if let Some(routine) = current
                .source
                .digital_routines
                .iter()
                .find(|routine| routine.name == first.name)
            {
                let kind = if routine.is_function() {
                    "function"
                } else {
                    "task"
                };
                if routine.automatic {
                    // Section 10.2.3 (tasks) and 10.4.1 (functions).
                    return invalid(
                        format!(
                            "`{}.{terminal}` names an item of automatic {kind} `{}`; IEEE \
                             1364-2005 sections 10.2.3 and 10.4.1 forbid a hierarchical \
                             reference to an automatic {kind}'s items",
                            first.name, first.name
                        ),
                        span,
                    );
                }
                let declared = routine
                    .arguments
                    .iter()
                    .any(|argument| argument.name == terminal)
                    || routine
                        .variables
                        .iter()
                        .flat_map(|declaration| &declaration.items)
                        .any(|item| item.name == terminal)
                    || routine
                        .digital_variables
                        .iter()
                        .flat_map(|declaration| &declaration.items)
                        .any(|item| item.name == terminal)
                    || (routine.is_function() && terminal == routine.name.as_str());
                if reference.scopes.len() == 1 && !declared {
                    return invalid(
                        format!("`{terminal}` is not declared in {kind} `{}`", first.name),
                        span,
                    );
                }
                return error(
                    format!(
                        "`{}.{terminal}` reads or writes an item of {kind} `{}`, and a reference \
                         into a {kind}'s own scope is not supported yet",
                        first.name, first.name
                    ),
                    span,
                );
            }
            frame = current.parent.filter(|_| self.selected);
        }
        if self.sources.modules.contains_key(&first.name) {
            return error(
                format!(
                    "`{}` names module `{}`, which is not instantiated here; a reference that \
                     starts at another top-level module is not supported yet",
                    reference
                        .source
                        .segments
                        .iter()
                        .map(|segment| segment.name.as_str())
                        .collect::<Vec<_>>()
                        .join("."),
                    first.name
                ),
                span,
            );
        }
        invalid(
            format!(
                "scope `{}` is not declared in this module or its generated ancestors or \
                 enclosing instances",
                first.name
            ),
            span,
        )
    }
}

impl Resolver {
    /// Why `key`, below `scope` in `frame`, is not a scope the resolver can
    /// enter: a refusal naming the construct `key` is, or a source error when
    /// `key` is a named block and the reference ends at a name the block does
    /// not declare (IEEE 1364-2005 section 12.5).
    pub(super) fn unentered_scope(
        &self,
        frame: usize,
        scope: &[HierarchicalScopeKey],
        key: &HierarchicalScopeKey,
        last: bool,
        reference: &ScopedHierarchicalReference,
    ) -> CompileError {
        let span = reference.source.span;
        let source = &self.frames[frame].source;
        let path = reference
            .source
            .segments
            .iter()
            .map(|segment| segment.name.as_str())
            .collect::<Vec<_>>()
            .join(".");
        let place = scope_place(source, scope);
        let full = [scope, std::slice::from_ref(key)].concat();
        if source.hierarchical_scopes.contains_key(&full) {
            return error(
                format!(
                    "`{path}` passes through `{}`, the name {place} gives an unnamed generate \
                     block, and a reference from outside that block through it is not \
                     supported yet",
                    key.name
                ),
                span,
            );
        }
        if !generate_items(source, &full).is_empty() {
            return error(
                format!(
                    "`{path}` names `{}`, a generate block of {place} this elaboration does not \
                     create",
                    scope_place(source, &full)
                        .trim_start_matches("generate block ")
                        .trim_matches('`')
                ),
                span,
            );
        }
        let items = generate_items(source, scope);
        if let Some(routine) = items
            .iter()
            .flat_map(|items| &items.digital_routines)
            .find(|routine| routine.name == key.name)
        {
            let kind = if routine.is_function() {
                "function"
            } else {
                "task"
            };
            return error(
                format!(
                    "`{path}` reaches into {kind} `{}` of {place}, and a reference into a \
                     {kind}'s own scope is not supported yet",
                    key.name
                ),
                span,
            );
        }
        let mut blocks = Vec::new();
        if scope.is_empty() {
            for process in source
                .digital_processes
                .iter()
                .filter(|process| process.generate_scope.is_none())
            {
                outer_named_blocks(&process.body, &mut blocks);
            }
        } else {
            for process in items.iter().flat_map(|items| &items.digital_processes) {
                outer_named_blocks(&process.body, &mut blocks);
            }
        }
        let named: Vec<_> = blocks
            .into_iter()
            .filter(|block| block.name.as_ref() == Some(&key.name))
            .collect();
        if named.is_empty() {
            return error(
                format!(
                    "`{}` is not a module instance, generate block, task, function or named \
                     block of {place}",
                    key.name
                ),
                span,
            );
        }
        let terminal = reference
            .source
            .segments
            .get(reference.scopes.len())
            .map_or("", |segment| segment.name.as_str());
        let declares = |block: &DigitalBlock| {
            block
                .variables
                .iter()
                .flat_map(|declaration| &declaration.items)
                .any(|item| item.name == terminal)
                || block
                    .digital_variables
                    .iter()
                    .flat_map(|declaration| &declaration.items)
                    .any(|item| item.name == terminal)
        };
        if last && !named.iter().any(|block| declares(block)) {
            return invalid(
                format!(
                    "`{terminal}` is not declared in named block `{}` of {place} (IEEE \
                     1364-2005 section 12.5)",
                    key.name
                ),
                span,
            );
        }
        error(
            format!(
                "`{path}` reaches into named block `{}` of {place}, and a reference into a \
                 named block's scope is not supported yet",
                key.name
            ),
            span,
        )
    }

    /// Why `terminal` names nothing in `scope`: a task or function a generate
    /// block declares is named as one, anything else as undeclared.
    pub(super) fn undeclared_terminal(
        &self,
        frame: usize,
        scope: &[HierarchicalScopeKey],
        terminal: &str,
        span: Span,
    ) -> CompileError {
        let source = &self.frames[frame].source;
        let place = scope_place(source, scope);
        if !scope.is_empty() {
            if let Some(routine) = generate_items(source, scope)
                .iter()
                .flat_map(|items| &items.digital_routines)
                .find(|routine| routine.name == terminal)
            {
                let kind = if routine.is_function() {
                    "function"
                } else {
                    "task"
                };
                return error(
                    format!(
                        "`{terminal}` is a {kind} of {place}, and calling a task or function a \
                         generate block declares is not supported yet"
                    ),
                    span,
                );
            }
        }
        error(format!("`{terminal}` is not declared in {place}"), span)
    }
}

/// Where `scope` is, for a message: the module, or a generate block below it.
fn scope_place(source: &Module, scope: &[HierarchicalScopeKey]) -> String {
    if scope.is_empty() {
        return format!("module `{}`", source.name);
    }
    let path = scope
        .iter()
        .map(|key| match key.index {
            Some(index) => format!("{}[{index}]", key.name),
            None => key.name.to_string(),
        })
        .collect::<Vec<_>>()
        .join(".");
    format!("generate block `{path}`")
}

/// The items of every generate block, as written, that `scope` names: one per
/// branch that declares a block of that name. The module itself for its own
/// scope.
fn generate_items<'a>(source: &'a Module, scope: &[HierarchicalScopeKey]) -> Vec<&'a Module> {
    if scope.is_empty() {
        return vec![source];
    }
    let mut constructs: Vec<&GenerateConstruct> = source
        .generate_template
        .as_ref()
        .map_or(&source.generates, |template| &template.module.generates)
        .iter()
        .collect();
    let mut found = Vec::new();
    for key in scope {
        let mut candidates = Vec::new();
        let mut pending: Vec<&GenerateBlock> = constructs
            .iter()
            .copied()
            .flat_map(construct_blocks)
            .collect();
        while let Some(block) = pending.pop() {
            if block.name.as_ref() == Some(&key.name) {
                candidates.push(block);
            } else if block.directly_nested || block.name.is_none() {
                // A directly nested conditional, or a block with no name of its
                // own, adds no scope: its blocks sit in the enclosing one.
                pending.extend(block.nested.iter().flat_map(construct_blocks));
            }
        }
        constructs = candidates
            .iter()
            .flat_map(|block| block.nested.iter())
            .collect();
        found = candidates;
    }
    found
        .into_iter()
        .map(|block| block.items.as_ref())
        .collect()
}

fn construct_blocks(construct: &GenerateConstruct) -> Vec<&GenerateBlock> {
    match construct {
        GenerateConstruct::Loop(value) => vec![&value.body],
        GenerateConstruct::Block(value) => vec![value],
        GenerateConstruct::Conditional(value) => std::iter::once(&value.then_block)
            .chain(value.else_block.iter())
            .collect(),
        GenerateConstruct::Case(value) => value
            .items
            .iter()
            .map(|item| &item.block)
            .chain(value.default.iter())
            .collect(),
    }
}

/// The named blocks a body opens directly, through unnamed blocks and control
/// statements but not into another named block, which is a scope of its own.
fn outer_named_blocks<'a>(statement: &'a DigitalStatement, blocks: &mut Vec<&'a DigitalBlock>) {
    match statement {
        DigitalStatement::Block(block) | DigitalStatement::Fork(block) => {
            if block.name.is_some() {
                blocks.push(block);
            } else {
                for statement in &block.statements {
                    outer_named_blocks(statement, blocks);
                }
            }
        }
        DigitalStatement::Conditional(conditional) => {
            outer_named_blocks(&conditional.then_branch, blocks);
            if let Some(branch) = &conditional.else_branch {
                outer_named_blocks(branch, blocks);
            }
        }
        DigitalStatement::Case(case) => {
            for item in &case.items {
                outer_named_blocks(&item.statement, blocks);
            }
            if let Some(default) = &case.default {
                outer_named_blocks(default, blocks);
            }
        }
        DigitalStatement::For(statement) => outer_named_blocks(&statement.body, blocks),
        DigitalStatement::While(statement) => outer_named_blocks(&statement.body, blocks),
        DigitalStatement::Repeat(statement) => outer_named_blocks(&statement.body, blocks),
        DigitalStatement::Forever(statement) => outer_named_blocks(&statement.body, blocks),
        DigitalStatement::Timing(timing) => {
            if let Some(statement) = &timing.statement {
                outer_named_blocks(statement, blocks);
            }
        }
        DigitalStatement::Wait(wait) => {
            if let Some(statement) = &wait.statement {
                outer_named_blocks(statement, blocks);
            }
        }
        _ => {}
    }
}

pub(super) fn prepare(
    source: &SourceFile,
    sources: Sources,
    root: Module,
) -> CompileResult<SourceFile> {
    let selected = root.name.clone();
    let mut resolver = Resolver::new(sources.clone(), root.clone(), true)?;
    let mut cursor = 0;
    // Overrides may discover later siblings on demand. Every discovered occurrence
    // joins this same graph and is visited exactly once here.
    while cursor < resolver.frames.len() {
        let count = resolver.frames[cursor].source.instances.len();
        for ordinal in 0..count {
            let span = resolver.frames[cursor].source.instances[ordinal].span;
            resolver.child(cursor, ordinal, span)?;
        }
        let references: Vec<_> = resolver.frames[cursor]
            .source
            .pending_hierarchical_references
            .keys()
            .cloned()
            .collect();
        for symbol in references {
            resolver.reference(cursor, &symbol)?;
        }
        cursor += 1;
    }
    let mut representatives = HashMap::new();
    let mut occurrences = HashMap::new();
    for frame in resolver.frames {
        representatives
            .entry(frame.source.name.clone())
            .or_insert_with(|| frame.path.clone());
        occurrences.insert(frame.path, frame.source);
    }
    let catalog = Arc::new(ReferenceSourceCatalog {
        modules: sources.modules.clone(),
        disciplines: sources.disciplines.clone(),
        design: Some(ReferenceDesign {
            root: selected,
            root_source: root,
            occurrences,
            inserted: HashMap::new(),
            module_names: HashMap::new(),
        }),
    });
    let design = catalog.design.as_ref().unwrap();
    let mut prepared = source.clone();
    for item in &mut prepared.items {
        let (Item::Module(module) | Item::ConnectModule(module)) = item else {
            continue;
        };
        *module = if let Some(path) = representatives.get(&module.name) {
            design.occurrences[path].clone()
        } else {
            catalog.modules[&module.name].clone()
        };
        module.reference_sources = Some(catalog.clone());
    }
    Ok(prepared)
}

pub(super) fn resolver(sources: Sources, module: &Module) -> CompileResult<(Resolver, usize)> {
    let Some(design) = sources
        .design
        .as_ref()
        .filter(|_| module.reference_context.is_some())
    else {
        return Ok((Resolver::new(sources, module.clone(), false)?, 0));
    };
    let root = design.root_source.clone();
    let path = module.reference_context.as_ref().unwrap();
    let mut resolver = Resolver::new(sources, root, true)?;
    // Instance names are flattened generate names and may themselves contain a
    // period. Follow actual instances, never split the path into guessed names.
    let mut owner = 0;
    while resolver.frames[owner].path != *path {
        let frame = &resolver.frames[owner];
        let ordinal = frame
            .source
            .instances
            .iter()
            .position(|instance| {
                let candidate = if frame.path.is_empty() {
                    instance.name.to_string()
                } else {
                    format!("{}.{}", frame.path, instance.name)
                };
                path == &candidate
                    || path
                        .strip_prefix(&candidate)
                        .is_some_and(|rest| rest.starts_with('.'))
            })
            .ok_or_else(|| {
                error(
                    format!("occurrence `{path}` is no longer in the selected design"),
                    module.span,
                )
            })?;
        owner = resolver.child(owner, ordinal, module.span)?;
    }
    resolver.frames[owner].source = module.clone();
    resolver.frames[owner].source.reference_sources = None;
    let sources = resolver.sources.clone();
    extend_insertions(&sources, &mut resolver.frames[owner].source, path);
    resolver.frames[owner].physical = None;
    Ok((resolver, owner))
}

/// A complete occurrence source replaces template defaults only at its own path.
pub(crate) fn occurrence(source: &Module, path: &str) -> Option<Module> {
    let catalog = source.reference_sources.as_ref()?;
    let design = catalog.design.as_ref()?;
    let mut module = design.occurrences.get(path)?.clone();
    if module.name != source.name {
        return None;
    }
    module.reference_sources = Some(catalog.clone());
    Some(module)
}

pub(super) fn extend_insertions(sources: &Sources, module: &mut Module, path: &str) {
    if let Some(instances) = sources
        .design
        .as_ref()
        .and_then(|design| design.inserted.get(path))
    {
        for instance in instances {
            if !module
                .instances
                .iter()
                .any(|existing| existing.name == instance.name)
            {
                module.instances.push(instance.clone());
            }
        }
    }
}

/// Resolve an automatically inserted body and its descendants in the upper
/// connection's context (VAMS-2023 7.8.4, 7.8.6). The retained declarations and
/// insertion descriptors have no catalog back-references.
pub(crate) fn insert(
    sources: Sources,
    parent: &Module,
    body: &Module,
    authored_name: &SmolStr,
    instance: ModuleInstance,
) -> CompileResult<(Sources, Vec<Module>)> {
    let parent_path = parent.reference_context.as_deref().ok_or_else(|| {
        error(
            "connect insertion has no selected parent occurrence",
            instance.span,
        )
    })?;
    let path: SmolStr = if parent_path.is_empty() {
        instance.name.clone()
    } else {
        format!("{parent_path}.{}", instance.name).into()
    };
    let mut catalog = (*sources).clone();
    let mut template = body.clone();
    template.reference_sources = None;
    template.reference_context = None;
    catalog.modules.insert(body.name.clone(), template);
    let design = catalog
        .design
        .as_mut()
        .ok_or_else(|| error("connect insertion has no selected design", instance.span))?;
    design
        .module_names
        .insert(body.name.clone(), authored_name.clone());
    design
        .inserted
        .entry(parent_path.into())
        .or_default()
        .push(instance.clone());
    let sources = Arc::new(catalog);
    let (mut resolver, owner) = resolver(sources.clone(), parent)?;
    // resolver replaces the parent with its resolved physical view. Add this
    // insertion there too, then refresh declaration discovery for the new member.
    extend_insertions(&sources, &mut resolver.frames[owner].source, parent_path);
    let source = resolver.frames[owner].source.clone();
    for symbol in source.pending_hierarchical_references.keys() {
        resolver.resolved.insert((owner, symbol.clone()));
    }
    resolver.frames[owner].root_members = source.declared_names();
    resolver.frames[owner].instances = source
        .instances
        .iter()
        .enumerate()
        .map(|(index, instance)| (instance.name.clone(), index))
        .collect();
    let ordinal = resolver.frames[owner].instances[&instance.name];
    let first = resolver.child(owner, ordinal, instance.span)?;
    let within = |candidate: &str| {
        candidate == path.as_str()
            || candidate
                .strip_prefix(path.as_str())
                .is_some_and(|rest| rest.starts_with('.'))
    };
    let mut cursor = first;
    while cursor < resolver.frames.len() {
        if within(&resolver.frames[cursor].path) {
            for ordinal in 0..resolver.frames[cursor].source.instances.len() {
                let span = resolver.frames[cursor].source.instances[ordinal].span;
                resolver.child(cursor, ordinal, span)?;
            }
            let references: Vec<_> = resolver.frames[cursor]
                .source
                .pending_hierarchical_references
                .keys()
                .cloned()
                .collect();
            for reference in references {
                resolver.reference(cursor, &reference)?;
            }
        }
        cursor += 1;
    }
    let mut catalog = (*sources).clone();
    let design = catalog.design.as_mut().unwrap();
    let mut paths = Vec::new();
    for frame in resolver.frames {
        if within(&frame.path) {
            paths.push(frame.path.clone());
            design.occurrences.insert(frame.path, frame.source);
        }
    }
    let catalog = Arc::new(catalog);
    let modules = paths
        .iter()
        .map(|path| {
            let mut source = catalog.design.as_ref().unwrap().occurrences[path].clone();
            source.reference_sources = Some(catalog.clone());
            source
        })
        .collect();
    Ok((catalog, modules))
}
