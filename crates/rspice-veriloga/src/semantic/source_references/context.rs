//! Select and bind concrete source occurrences before either domain checks bodies.
use super::*;

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
        Err(error(
            format!(
                "scope `{}` is not declared in this module or its generated ancestors or enclosing instances",
                first.name
            ),
            span,
        ))
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
