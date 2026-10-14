//! Library binding: IEEE 1364-2005 clause 13 applied to a parsed source.
//!
//! A library map puts each source file in a library, and a `config` binds the
//! instances below a design's top cell to cells of those libraries. This pass
//! runs once, right after parsing, and leaves an ordinary set of modules
//! behind, so nothing after it knows about libraries or configurations:
//!
//! - **The default view.** An instance no configuration governs binds to the
//!   first library, in the map's declaration order, that declares its cell
//!   (section 13.5.1); `work` comes last unless the map declares it. That
//!   module keeps its own name. A module another library shadows is dropped:
//!   nothing can bind it by default.
//! - **Configurations.** Each `config` binds its design's hierarchy by its
//!   rules: the instance clause first, then the cell clause, then the liblist
//!   an ancestor's clause passed down, then the `default` liblist, then the
//!   map's order (sections 13.3.1.2 to 13.3.1.6). A `use` clause without a
//!   library takes the parent cell's. Every module the binding needs that the
//!   default view does not already provide is copied under the name
//!   `library.cell`, with a variant name where a path-specific rule binds one
//!   cell two ways. The top cell is copied under the configuration's own name,
//!   so a deck selects a configuration as it selects a module.
//!
//! A module bound from a library other than `work` carries its `library.cell`
//! for `%l` (section 13.6). A source with no library marker and no `config`
//! is left exactly as parsed.

use crate::ast::{
    ConfigDecl, ConfigExpansion, ConfigSelection, GenerateBlock, GenerateConstruct, Item,
    LibraryCell, Module, ModuleBinding, ModuleInstance, SourceFile,
};
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::parser::WORK_LIBRARY;
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::{BTreeMap, HashMap, HashSet};

/// Bounds the binding graph: each node is one module binding.
const MAX_BINDING_NODES: usize = 65_536;

fn binding_error(detail: impl Into<String>, span: Span) -> CompileError {
    SemanticError::new(SemanticErrorKind::LibraryBinding(detail.into()), span).into()
}

fn unsupported(detail: impl Into<String>, span: Span) -> CompileError {
    SemanticError::new(SemanticErrorKind::UnsupportedFeature(detail.into()), span).into()
}

/// Apply the library map's placement and every configuration to `file`.
pub(crate) fn bind(file: &mut SourceFile) -> CompileResult<()> {
    let placed = file
        .items
        .iter()
        .any(|item| matches!(item, Item::Module(module) if module.library.is_some()));
    if file.configs.is_empty() && file.library_order.is_none() && !placed {
        return Ok(());
    }
    let mut binder = Binder::new(file)?;
    for index in 0..binder.configs.len() {
        binder.validate(index)?;
    }
    binder.build_default_view()?;
    let roots = (0..binder.configs.len())
        .map(|index| binder.config_root(index))
        .collect::<CompileResult<Vec<_>>>()?;
    let emitted = binder.emit(&roots)?;
    file.items = emitted;
    Ok(())
}

/// What a node of the binding graph is keyed by.
#[derive(Clone, PartialEq, Eq, Hash)]
enum NodeKey {
    /// A module the default view binds.
    Default { cell: (SmolStr, SmolStr) },
    /// A module one configuration binds. `liblist` is the one inherited from
    /// an ancestor's clause; `path` is kept only where an instance clause
    /// below it could tell two paths apart.
    Config {
        config: usize,
        cell: (SmolStr, SmolStr),
        liblist: Option<Vec<SmolStr>>,
        path: Option<String>,
    },
}

/// One module binding: a library cell and what each of its instances binds.
struct Node {
    cell: (SmolStr, SmolStr),
    item: usize,
    default: bool,
    /// One entry per instance of the module, in order; `None` for an instance
    /// of something no library declares (a primitive, a UDP, an analog
    /// built-in, or an undefined module the analysis reports).
    children: Vec<Option<usize>>,
    /// The bindings of the cells a generate region instantiates that the
    /// region's default expansion did not: a parameter override may expand
    /// them later.
    regional: Vec<(SmolStr, usize)>,
}

struct Binder {
    items: Vec<Item>,
    configs: Vec<ConfigDecl>,
    /// The library search order of the map, `work` last unless declared.
    order: Vec<SmolStr>,
    /// `(library, cell)` to the item that declares it.
    cells: HashMap<(SmolStr, SmolStr), usize>,
    nodes: Vec<Node>,
    memo: HashMap<NodeKey, usize>,
}

impl Binder {
    fn new(file: &mut SourceFile) -> CompileResult<Self> {
        let mut order: Vec<SmolStr> = file.library_order.clone().unwrap_or_default();
        let mut cells = HashMap::new();
        for (index, item) in file.items.iter().enumerate() {
            let Item::Module(module) = item else {
                continue;
            };
            let library = module
                .library
                .clone()
                .unwrap_or_else(|| SmolStr::new_static(WORK_LIBRARY));
            if !order.contains(&library) && library != WORK_LIBRARY {
                order.push(library.clone());
            }
            if cells
                .insert((library.clone(), module.name.clone()), index)
                .is_some()
            {
                return Err(binding_error(
                    format!(
                        "module '{}' is declared twice in library '{library}', which declares a \
                         cell name once",
                        module.name
                    ),
                    module.span,
                ));
            }
        }
        if !order.iter().any(|library| library == WORK_LIBRARY) {
            order.push(SmolStr::new_static(WORK_LIBRARY));
        }
        Ok(Self {
            items: std::mem::take(&mut file.items),
            configs: std::mem::take(&mut file.configs),
            order,
            cells,
            nodes: Vec::new(),
            memo: HashMap::new(),
        })
    }

    fn module(&self, item: usize) -> &Module {
        match &self.items[item] {
            Item::Module(module) => module,
            _ => unreachable!("a binding node names a module item"),
        }
    }

    fn declares(&self, cell: &str) -> bool {
        self.order
            .iter()
            .any(|library| self.cells.contains_key(&(library.clone(), cell.into())))
    }

    /// The first library of `libraries` that declares `cell`.
    fn search(&self, libraries: &[SmolStr], cell: &SmolStr) -> Option<SmolStr> {
        libraries
            .iter()
            .find(|library| self.cells.contains_key(&((*library).clone(), cell.clone())))
            .cloned()
    }

    fn require_library(
        &self,
        library: &SmolStr,
        config: &ConfigDecl,
        span: Span,
    ) -> CompileResult<()> {
        if self.order.contains(library) {
            return Ok(());
        }
        Err(binding_error(
            format!(
                "configuration '{}' names library '{library}', which is not declared; the \
                 libraries are {}",
                config.name,
                self.order.join(", ")
            ),
            span,
        ))
    }

    /// The checks a configuration answers to before anything is bound.
    fn validate(&self, index: usize) -> CompileResult<()> {
        let config = &self.configs[index];
        let design = match config.design.as_slice() {
            [design] => design,
            [] => {
                return Err(binding_error(
                    format!("configuration '{}' names no design cell", config.name),
                    config.span,
                ));
            }
            many => {
                return Err(unsupported(
                    format!(
                        "configuration '{}' names {} design cells; RSpice binds one top cell per \
                         configuration",
                        config.name,
                        many.len()
                    ),
                    config.span,
                ));
            }
        };
        if let Some(library) = &design.library {
            self.require_library(library, config, design.span)?;
        }
        if self.order.iter().any(|library| {
            self.cells
                .contains_key(&(library.clone(), config.name.clone()))
        }) || self.configs[..index]
            .iter()
            .any(|other| other.name == config.name)
        {
            // Section 13.3.1: a configuration's name is in the module
            // namespace.
            return Err(binding_error(
                format!(
                    "configuration '{}' has the name of a module or of another configuration",
                    config.name
                ),
                config.span,
            ));
        }
        let mut selections = HashSet::new();
        for rule in &config.rules {
            let label = match &rule.selection {
                ConfigSelection::Default => "default".to_string(),
                ConfigSelection::Instance(path) => {
                    if path.first() != Some(&design.cell) {
                        return Err(binding_error(
                            format!(
                                "instance clause '{}' of configuration '{}' does not start at its \
                                 design cell '{}'",
                                path.join("."),
                                config.name,
                                design.cell
                            ),
                            rule.span,
                        ));
                    }
                    format!("instance {}", path.join("."))
                }
                ConfigSelection::Cell(cell) => {
                    if let Some(library) = &cell.library {
                        self.require_library(library, config, cell.span)?;
                    }
                    format!("cell {}", display_cell(cell))
                }
            };
            if !selections.insert(label.clone()) {
                return Err(binding_error(
                    format!("configuration '{}' has two `{label}` clauses", config.name),
                    rule.span,
                ));
            }
            match &rule.expansion {
                ConfigExpansion::Liblist(libraries) => {
                    if libraries.is_empty() {
                        return Err(unsupported(
                            format!(
                                "an empty liblist in configuration '{}'; name the libraries to \
                                 search",
                                config.name
                            ),
                            rule.span,
                        ));
                    }
                    for library in libraries {
                        self.require_library(library, config, rule.span)?;
                    }
                }
                ConfigExpansion::Use {
                    cell,
                    configuration,
                } => {
                    if *configuration {
                        return Err(unsupported(
                            format!(
                                "`use {}:config` in configuration '{}': a hierarchical \
                                 configuration",
                                display_cell(cell),
                                config.name
                            ),
                            rule.span,
                        ));
                    }
                    if let Some(library) = &cell.library {
                        self.require_library(library, config, cell.span)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn default_library(&self, cell: &SmolStr) -> Option<SmolStr> {
        self.search(&self.order, cell)
    }

    fn add_node(&mut self, key: NodeKey, cell: (SmolStr, SmolStr)) -> CompileResult<usize> {
        if self.nodes.len() >= MAX_BINDING_NODES {
            let item = self.cells[&cell];
            return Err(binding_error(
                format!("the library binding needs more than {MAX_BINDING_NODES} module bindings"),
                self.module(item).span,
            ));
        }
        let id = self.nodes.len();
        self.memo.insert(key.clone(), id);
        self.nodes.push(Node {
            item: self.cells[&cell],
            default: matches!(key, NodeKey::Default { .. }),
            cell,
            children: Vec::new(),
            regional: Vec::new(),
        });
        Ok(id)
    }

    /// The instances of an item, as `(instance name, cell)`.
    fn instances(&self, item: usize) -> Vec<(SmolStr, SmolStr)> {
        self.module(item)
            .instances
            .iter()
            .map(|instance| (instance.name.clone(), instance.module.clone()))
            .collect()
    }

    /// The library cells an item's generate regions instantiate that its
    /// default expansion does not, in first-written order.
    fn regional_cells(&self, item: usize) -> Vec<SmolStr> {
        let module = self.module(item);
        let expanded: HashSet<&SmolStr> = module
            .instances
            .iter()
            .map(|instance| &instance.module)
            .collect();
        let mut cells = Vec::new();
        let mut collect = |instance: &mut ModuleInstance| -> CompileResult<()> {
            if !expanded.contains(&instance.module)
                && self.declares(&instance.module)
                && !cells.contains(&instance.module)
            {
                cells.push(instance.module.clone());
            }
            Ok(())
        };
        // The walk takes the regions by value: the closure only reads.
        let mut generates = module.generates.clone();
        if let Some(template) = &module.generate_template {
            generates.extend(template.module.generates.iter().cloned());
        }
        for construct in &mut generates {
            let _ = rename_in_construct(construct, &mut collect);
        }
        cells
    }

    /// A node for every module the default view binds.
    fn build_default_view(&mut self) -> CompileResult<()> {
        let mut defaults = Vec::new();
        for (index, item) in self.items.iter().enumerate() {
            let Item::Module(module) = item else {
                continue;
            };
            let library = module
                .library
                .clone()
                .unwrap_or_else(|| SmolStr::new_static(WORK_LIBRARY));
            if self.default_library(&module.name).as_ref() == Some(&library) {
                defaults.push((index, (library, module.name.clone())));
            }
        }
        for (_, cell) in &defaults {
            let key = NodeKey::Default { cell: cell.clone() };
            self.add_node(key, cell.clone())?;
        }
        for (index, cell) in defaults {
            let id = self.memo[&NodeKey::Default { cell }];
            let default_node = |target: SmolStr| {
                self.default_library(&target).map(|library| {
                    self.memo[&NodeKey::Default {
                        cell: (library, target),
                    }]
                })
            };
            let children = self
                .instances(index)
                .into_iter()
                .map(|(_, target)| default_node(target))
                .collect();
            let regional = self
                .regional_cells(index)
                .into_iter()
                .filter_map(|target| Some((target.clone(), default_node(target)?)))
                .collect();
            self.nodes[id].children = children;
            self.nodes[id].regional = regional;
        }
        Ok(())
    }

    /// Whether an instance clause of `config` names a path below `path`.
    fn path_dependent(&self, config: usize, path: &str) -> bool {
        self.configs[config].rules.iter().any(|rule| {
            matches!(&rule.selection, ConfigSelection::Instance(segments)
                if segments.join(".").strip_prefix(path).is_some_and(|rest| rest.starts_with('.')))
        })
    }

    /// The node of configuration `index`'s top cell.
    fn config_root(&mut self, index: usize) -> CompileResult<usize> {
        let config = &self.configs[index];
        let design = &config.design[0];
        let library = design
            .library
            .clone()
            .unwrap_or_else(|| config.library.clone());
        let cell = (library, design.cell.clone());
        if !self.cells.contains_key(&cell) {
            return Err(binding_error(
                format!(
                    "the design cell '{}.{}' of configuration '{}' is not declared",
                    cell.0, cell.1, config.name
                ),
                design.span,
            ));
        }
        let path = design.cell.to_string();
        self.config_node(index, cell, None, path)
    }

    fn config_node(
        &mut self,
        config: usize,
        cell: (SmolStr, SmolStr),
        liblist: Option<Vec<SmolStr>>,
        path: String,
    ) -> CompileResult<usize> {
        let key = NodeKey::Config {
            config,
            cell: cell.clone(),
            liblist: liblist.clone(),
            path: self.path_dependent(config, &path).then(|| path.clone()),
        };
        if let Some(&id) = self.memo.get(&key) {
            return Ok(id);
        }
        let id = self.add_node(key, cell.clone())?;
        let mut children = Vec::new();
        for (name, target) in self.instances(self.nodes[id].item) {
            if !self.declares(&target) {
                children.push(None);
                continue;
            }
            let child_path = format!("{path}.{name}");
            let (bound, inherited) = self.resolve(
                config,
                &cell.0,
                &target,
                liblist.as_deref(),
                &child_path,
                id,
            )?;
            children.push(Some(
                self.config_node(config, bound, inherited, child_path)?,
            ));
        }
        // An instance a generate region has not expanded has no path an
        // instance clause could name: it binds by the cell rules.
        let mut regional = Vec::new();
        for target in self.regional_cells(self.nodes[id].item) {
            let child_path = format!("{path}.<generate>");
            let (bound, inherited) = self.resolve(
                config,
                &cell.0,
                &target,
                liblist.as_deref(),
                &child_path,
                id,
            )?;
            regional.push((
                target,
                self.config_node(config, bound, inherited, child_path)?,
            ));
        }
        self.nodes[id].children = children;
        self.nodes[id].regional = regional;
        Ok(id)
    }

    /// Bind one instance of `target` at `path`, whose parent cell is in
    /// `parent_library`. Returns the cell and the liblist its own instances
    /// inherit.
    fn resolve(
        &self,
        config: usize,
        parent_library: &SmolStr,
        target: &SmolStr,
        inherited: Option<&[SmolStr]>,
        path: &str,
        parent: usize,
    ) -> CompileResult<((SmolStr, SmolStr), Option<Vec<SmolStr>>)> {
        let declaration = &self.configs[config];
        let span = self.module(self.nodes[parent].item).span;
        if let Some(rule) = declaration.rules.iter().find(|rule| {
            matches!(&rule.selection, ConfigSelection::Instance(segments) if segments.join(".") == path)
        }) {
            return self.apply(config, &rule.expansion, parent_library, target, inherited, path, rule.span);
        }
        let default_liblist =
            declaration
                .rules
                .iter()
                .find_map(|rule| match (&rule.selection, &rule.expansion) {
                    (ConfigSelection::Default, ConfigExpansion::Liblist(libraries)) => {
                        Some(libraries.as_slice())
                    }
                    _ => None,
                });
        let searched = inherited.or(default_liblist).unwrap_or(&self.order);
        let candidate = self.search(searched, target);
        // A cell clause that names a library selects the instances that
        // would otherwise bind to that library's cell; one without selects
        // every instance of the cell.
        let cell_rule = |library: Option<&SmolStr>| {
            declaration.rules.iter().find(|rule| {
                matches!(&rule.selection, ConfigSelection::Cell(cell)
                    if cell.cell == *target && cell.library.as_ref() == library)
            })
        };
        if let Some(rule) = candidate
            .as_ref()
            .and_then(|library| cell_rule(Some(library)))
            .or_else(|| cell_rule(None))
        {
            return self.apply(
                config,
                &rule.expansion,
                parent_library,
                target,
                inherited,
                path,
                rule.span,
            );
        }
        match candidate {
            Some(library) => Ok(((library, target.clone()), inherited.map(<[_]>::to_vec))),
            None => Err(binding_error(
                format!(
                    "configuration '{}' binds instance '{path}' of cell '{target}', which none \
                     of the libraries {} declares",
                    declaration.name,
                    searched.join(", ")
                ),
                span,
            )),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn apply(
        &self,
        config: usize,
        expansion: &ConfigExpansion,
        parent_library: &SmolStr,
        target: &SmolStr,
        inherited: Option<&[SmolStr]>,
        path: &str,
        span: Span,
    ) -> CompileResult<((SmolStr, SmolStr), Option<Vec<SmolStr>>)> {
        let name = &self.configs[config].name;
        match expansion {
            ConfigExpansion::Liblist(libraries) => match self.search(libraries, target) {
                Some(library) => Ok(((library, target.clone()), Some(libraries.clone()))),
                None => Err(binding_error(
                    format!(
                        "configuration '{name}' binds instance '{path}' of cell '{target}' from \
                         the liblist {}, none of whose libraries declares it",
                        libraries.join(", ")
                    ),
                    span,
                )),
            },
            ConfigExpansion::Use { cell, .. } => {
                // Without a library, the parent cell's (section 13.3.1.6).
                let library = cell
                    .library
                    .clone()
                    .unwrap_or_else(|| parent_library.clone());
                let bound = (library, cell.cell.clone());
                if !self.cells.contains_key(&bound) {
                    return Err(binding_error(
                        format!(
                            "configuration '{name}' binds instance '{path}' to '{}.{}', which is \
                             not declared",
                            bound.0, bound.1
                        ),
                        span,
                    ));
                }
                Ok((bound, inherited.map(<[_]>::to_vec)))
            }
        }
    }

    /// Group nodes that bind the same cell to the same subtree, by partition
    /// refinement: a class is one module.
    fn classes(&self) -> Vec<usize> {
        let mut class: Vec<usize> = {
            let mut ids = HashMap::new();
            self.nodes
                .iter()
                .map(|node| {
                    let next = ids.len();
                    *ids.entry(node.cell.clone()).or_insert(next)
                })
                .collect()
        };
        loop {
            let mut ids = HashMap::new();
            let refined: Vec<usize> = self
                .nodes
                .iter()
                .enumerate()
                .map(|(id, node)| {
                    let signature = (
                        class[id],
                        node.children
                            .iter()
                            .map(|child| child.map(|child| class[child]))
                            .collect::<Vec<_>>(),
                        node.regional
                            .iter()
                            .map(|(cell, child)| (cell.clone(), class[*child]))
                            .collect::<Vec<_>>(),
                    );
                    let next = ids.len();
                    *ids.entry(signature).or_insert(next)
                })
                .collect();
            let before = class.iter().collect::<HashSet<_>>().len();
            let after = ids.len();
            class = refined;
            if after == before {
                return class;
            }
        }
    }

    /// The bound module set: the default view in source order, then each
    /// module a configuration needs, then each configuration's top cell.
    fn emit(mut self, roots: &[usize]) -> CompileResult<Vec<Item>> {
        let class = self.classes();
        let mut taken: HashSet<SmolStr> = self
            .items
            .iter()
            .filter_map(|item| match item {
                Item::Module(module) | Item::ConnectModule(module) => Some(module.name.clone()),
                _ => None,
            })
            .chain(self.configs.iter().map(|config| config.name.clone()))
            .collect();
        // The name each class's module goes by.
        let mut names: BTreeMap<usize, SmolStr> = BTreeMap::new();
        for (id, node) in self.nodes.iter().enumerate() {
            if node.default {
                names.insert(class[id], node.cell.1.clone());
            }
        }
        let referenced: HashSet<usize> = self
            .nodes
            .iter()
            .flat_map(|node| {
                node.children
                    .iter()
                    .flatten()
                    .copied()
                    .chain(node.regional.iter().map(|(_, child)| *child))
            })
            .map(|child| class[child])
            .collect();
        let mut copies = Vec::new();
        for (id, node) in self.nodes.iter().enumerate() {
            let class_id = class[id];
            if names.contains_key(&class_id) || !referenced.contains(&class_id) {
                continue;
            }
            let base = format!("{}.{}", node.cell.0, node.cell.1);
            let name = fresh_name(&base, &mut taken);
            names.insert(class_id, name.clone());
            copies.push((id, name));
        }
        let mut emitted = Vec::new();
        let items = std::mem::take(&mut self.items);
        let defaults: HashMap<usize, usize> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node.default)
            .map(|(id, node)| (node.item, id))
            .collect();
        let mut sources: HashMap<usize, Module> = HashMap::new();
        for (index, item) in items.into_iter().enumerate() {
            match item {
                Item::Module(mut module) => {
                    let Some(&id) = defaults.get(&index) else {
                        // Shadowed by an earlier library; a configuration may
                        // still copy it.
                        sources.insert(index, module);
                        continue;
                    };
                    let library = &self.nodes[id].cell.0;
                    if library != WORK_LIBRARY {
                        module.binding = Some(ModuleBinding {
                            display: format!("{library}.{}", module.name).into(),
                            copy: false,
                        });
                    }
                    sources.insert(index, module.clone());
                    emitted.push(Item::Module(module));
                }
                other => emitted.push(other),
            }
        }
        let bound_copy = |id: usize, name: SmolStr| -> CompileResult<Module> {
            let node = &self.nodes[id];
            let mut module = sources[&node.item].clone();
            let renames: Vec<Option<SmolStr>> = node
                .children
                .iter()
                .map(|child| child.map(|child| names[&class[child]].clone()))
                .collect();
            let regional: Vec<(SmolStr, SmolStr)> = node
                .regional
                .iter()
                .map(|(cell, child)| (cell.clone(), names[&class[*child]].clone()))
                .collect();
            rename_instances(&mut module, &renames, &regional)?;
            module.name = name;
            module.binding = Some(ModuleBinding {
                display: format!("{}.{}", node.cell.0, node.cell.1).into(),
                copy: true,
            });
            Ok(module)
        };
        for (id, name) in copies {
            emitted.push(Item::Module(bound_copy(id, name)?));
        }
        for (config, &root) in roots.iter().enumerate() {
            let name = self.configs[config].name.clone();
            emitted.push(Item::Module(bound_copy(root, name)?));
        }
        Ok(emitted)
    }
}

fn display_cell(cell: &LibraryCell) -> String {
    match &cell.library {
        Some(library) => format!("{library}.{}", cell.cell),
        None => cell.cell.to_string(),
    }
}

/// `base`, or `base` with the first variant number that no module, copy or
/// configuration has taken.
fn fresh_name(base: &str, taken: &mut HashSet<SmolStr>) -> SmolStr {
    let mut name = SmolStr::from(base);
    let mut variant = 2usize;
    while taken.contains(&name) {
        name = format!("{base}~{variant}").into();
        variant += 1;
    }
    taken.insert(name.clone());
    name
}

/// Rename a copy's instances to the modules they bind. `renames` follows the
/// module's instances, in order. A generate region keeps the names it was
/// written with for a later re-expansion, so an instance there is renamed by
/// cell, which needs every instance of that cell bound alike.
fn rename_instances(
    module: &mut Module,
    renames: &[Option<SmolStr>],
    regional: &[(SmolStr, SmolStr)],
) -> CompileResult<()> {
    let mut by_instance: HashMap<SmolStr, SmolStr> = HashMap::new();
    let mut by_cell: HashMap<SmolStr, Option<SmolStr>> = regional
        .iter()
        .map(|(cell, rename)| (cell.clone(), Some(rename.clone())))
        .collect();
    for (instance, rename) in module.instances.iter().zip(renames) {
        let Some(rename) = rename else {
            continue;
        };
        by_instance.insert(instance.name.clone(), rename.clone());
        by_cell
            .entry(instance.module.clone())
            .and_modify(|uniform| {
                if uniform.as_ref() != Some(rename) {
                    *uniform = None;
                }
            })
            .or_insert_with(|| Some(rename.clone()));
    }
    for instance in &mut module.instances {
        if let Some(rename) = by_instance.get(&instance.name) {
            instance.module = rename.clone();
        }
    }
    let mut regional = |instance: &mut ModuleInstance| -> CompileResult<()> {
        match by_cell.get(&instance.module) {
            Some(Some(rename)) => {
                instance.module = rename.clone();
                Ok(())
            }
            Some(None) => Err(unsupported(
                format!(
                    "an instance clause binds instances of cell '{}' two ways in a module whose \
                     generate region instantiates it",
                    instance.module
                ),
                instance.span,
            )),
            None => Ok(()),
        }
    };
    for construct in &mut module.generates {
        rename_in_construct(construct, &mut regional)?;
    }
    if let Some(template) = module.generate_template.as_mut() {
        let template = &mut template.module;
        for instance in &mut template.instances {
            if let Some(rename) = by_instance.get(&instance.name) {
                instance.module = rename.clone();
            }
        }
        for construct in &mut template.generates {
            rename_in_construct(construct, &mut regional)?;
        }
    }
    Ok(())
}

fn rename_in_construct(
    construct: &mut GenerateConstruct,
    rename: &mut dyn FnMut(&mut ModuleInstance) -> CompileResult<()>,
) -> CompileResult<()> {
    match construct {
        GenerateConstruct::Loop(generate) => rename_in_block(&mut generate.body, rename),
        GenerateConstruct::Conditional(conditional) => {
            rename_in_block(&mut conditional.then_block, rename)?;
            if let Some(block) = conditional.else_block.as_mut() {
                rename_in_block(block, rename)?;
            }
            Ok(())
        }
        GenerateConstruct::Case(case) => {
            for item in &mut case.items {
                rename_in_block(&mut item.block, rename)?;
            }
            if let Some(block) = case.default.as_mut() {
                rename_in_block(block, rename)?;
            }
            Ok(())
        }
        GenerateConstruct::Block(block) => rename_in_block(block, rename),
    }
}

fn rename_in_block(
    block: &mut GenerateBlock,
    rename: &mut dyn FnMut(&mut ModuleInstance) -> CompileResult<()>,
) -> CompileResult<()> {
    for instance in &mut block.items.instances {
        rename(instance)?;
    }
    for construct in &mut block.items.generates {
        rename_in_construct(construct, rename)?;
    }
    for construct in &mut block.nested {
        rename_in_construct(construct, rename)?;
    }
    Ok(())
}
