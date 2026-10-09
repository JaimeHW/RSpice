//! Bind authored paths against concrete generated scopes, after all siblings exist.
use super::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};

pub(super) type ScopeKey = HierarchicalScopeKey;
type Scope = HierarchicalScope;
struct Pending {
    symbol: SmolStr,
    origin: Vec<ScopeKey>,
    scopes: Vec<ScopeKey>,
    terminal: SmolStr,
    source: HierarchicalName,
    index_dependencies: Vec<Expression>,
}
#[derive(Default)]
struct State {
    reserved: HashSet<SmolStr>,
    pending: Vec<Pending>,
    errors: Vec<ParseError>,
    serial: usize,
}
impl State {
    fn fresh(&mut self) -> SmolStr {
        loop {
            let name = SmolStr::from(format!("$rspice_bound_reference_{}", self.serial));
            self.serial += 1;
            if self.reserved.insert(name.clone()) {
                return name;
            }
        }
    }
}

pub(super) struct Bindings {
    pub authored: BTreeMap<SmolStr, HierarchicalName>,
    pub(super) scopes: HashMap<Vec<ScopeKey>, Scope>,
    state: RefCell<State>,
}
impl Bindings {
    pub fn new(module: &Module) -> Self {
        let members: HashMap<_, _> = module
            .declared_names()
            .into_iter()
            .map(|name| (name.clone(), name))
            .collect();
        let mut state = State::default();
        state
            .reserved
            .extend(module.reserved_identifiers.iter().cloned());
        state.reserved.extend(members.keys().cloned());
        state
            .reserved
            .extend(module.hierarchical_names.keys().cloned());
        // Reserve authored names even in inactive scopes, including escaped names.
        fn reserve(module: &Module, names: &mut HashSet<SmolStr>) {
            names.extend(module.declared_names());
            for construct in &module.generates {
                for block in construct_blocks(construct) {
                    reserve(&block.items, names);
                    for nested in &block.nested {
                        reserve_construct(nested, names);
                    }
                }
            }
        }
        fn reserve_construct(construct: &GenerateConstruct, names: &mut HashSet<SmolStr>) {
            for block in construct_blocks(construct) {
                reserve(&block.items, names);
                for nested in &block.nested {
                    reserve_construct(nested, names);
                }
            }
        }
        reserve(module, &mut state.reserved);
        Self {
            authored: module.hierarchical_names.clone(),
            scopes: HashMap::from([(
                Vec::new(),
                Scope {
                    members,
                    explicit: true,
                    children: HashSet::new(),
                },
            )]),
            state: RefCell::new(state),
        }
    }

    pub fn register_scope(
        &mut self,
        path: &[ScopeKey],
        prefix: &str,
        module: &Module,
        explicit: bool,
    ) -> HashMap<SmolStr, SmolStr> {
        // Directly nested conditionals are transparent; merge into their actual scope.
        let mut members = HashMap::new();
        let mut names: Vec<_> = module.declared_names().into_iter().collect();
        names.sort();
        let mut state = self.state.borrow_mut();
        for name in names {
            let candidate: SmolStr = format!("{prefix}{name}").into();
            let qualified = if state.reserved.insert(candidate.clone()) {
                candidate
            } else {
                state.fresh()
            };
            members.insert(name, qualified);
        }
        if let Some((last, parent)) = path.split_last() {
            if let Some(scope) = self.scopes.get_mut(parent) {
                scope.children.insert(last.name.clone());
            }
        }
        let scope = self.scopes.entry(path.to_vec()).or_insert_with(|| Scope {
            members: HashMap::new(),
            explicit,
            children: HashSet::new(),
        });
        scope.members.extend(members.clone());
        members
    }

    pub fn resolve(
        &self,
    ) -> Result<
        (
            HashMap<SmolStr, SmolStr>,
            BTreeMap<SmolStr, ScopedHierarchicalReference>,
        ),
        ParseError,
    > {
        let mut state = self.state.borrow_mut();
        if !state.errors.is_empty() {
            return Err(state.errors.remove(0));
        }
        let mut result = HashMap::new();
        let mut deferred = BTreeMap::new();
        for pending in &state.pending {
            let mut target = None;
            if !pending.source.absolute && pending.source.branch.is_none() {
                for level in (0..=pending.origin.len()).rev() {
                    let parent = &pending.origin[..level];
                    let first = &pending.scopes[0];
                    let declares_first = self.scopes.get(parent).is_some_and(|scope| {
                        scope.members.contains_key(&first.name)
                            || scope.children.contains(&first.name)
                    });
                    if !declares_first {
                        continue;
                    }
                    let mut path = parent.to_vec();
                    let mut visible = true;
                    for key in &pending.scopes {
                        path.push(key.clone());
                        if !self.scopes.get(&path).is_some_and(|scope| {
                            scope.explicit || pending.origin.starts_with(&path)
                        }) {
                            visible = false;
                            break;
                        }
                    }
                    if visible {
                        target = self
                            .scopes
                            .get(&path)
                            .and_then(|scope| scope.members.get(&pending.terminal))
                            .cloned();
                    }
                    // A nearer declaration shadows outer scopes even if the rest
                    // of this path is invalid; never accidentally bind an ancestor.
                    break;
                }
            }
            let Some(target) = target else {
                deferred.insert(
                    pending.symbol.clone(),
                    ScopedHierarchicalReference {
                        source: pending.source.clone(),
                        origin: pending.origin.clone(),
                        scopes: pending.scopes.clone(),
                        index_dependencies: pending.index_dependencies.clone(),
                    },
                );
                continue;
            };
            result.insert(pending.symbol.clone(), target);
        }
        Ok((result, deferred))
    }
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

impl Unroller<'_> {
    pub(super) fn bind_hierarchical_reference(&self, name: &mut SmolStr) -> bool {
        let Some(source) = self.hierarchy.authored.get(name) else {
            return false;
        };
        let mut scopes = Vec::new();
        let mut index_dependencies = Vec::new();
        for segment in &source.segments[..source.segments.len() - 1] {
            let index = match &segment.index {
                Some(expression) => match self.value(expression, "hierarchical scope index") {
                    Ok(value) => {
                        let mut dependency = expression.clone();
                        self.substitute(&mut dependency);
                        index_dependencies.push(dependency);
                        Some(value)
                    }
                    Err(error) => {
                        self.hierarchy.state.borrow_mut().errors.push(error);
                        return true;
                    }
                },
                None => None,
            };
            scopes.push(ScopeKey {
                name: segment.name.clone(),
                index,
            });
        }
        let mut source = source.clone();
        if let Some(branch) = &mut source.branch {
            for terminal in std::iter::once(&mut branch.pos).chain(branch.neg.iter_mut()) {
                let mut expressions = terminal.prefix.iter_mut().collect::<Vec<_>>();
                expressions.extend(
                    terminal
                        .name
                        .segments
                        .iter_mut()
                        .filter_map(|s| s.index.as_mut()),
                );
                if let Some(select) = &mut terminal.select {
                    match select {
                        PackedSelect::Bit(index) => expressions.push(index.as_mut()),
                        PackedSelect::Part { msb, lsb } => {
                            expressions.extend([msb.as_mut(), lsb.as_mut()])
                        }
                    }
                }
                for expression in expressions {
                    self.substitute(expression);
                    index_dependencies.push(expression.clone());
                }
            }
        }
        let mut state = self.hierarchy.state.borrow_mut();
        let symbol = state.fresh();
        state.pending.push(Pending {
            symbol: symbol.clone(),
            origin: self.scope_path.clone(),
            scopes,
            terminal: source.segments.last().unwrap().name.clone(),
            source,
            index_dependencies,
        });
        *name = symbol;
        true
    }

    /// Run the same exhaustive substitutions for root items and the final binding
    /// pass. Generated process identities and already-bound loop values survive.
    pub(super) fn rewrite_items(&mut self, module: &mut Module) {
        let constants_len = self.constants.definitions.len();
        let genvars_len = self.genvars.len();
        let mut rewritten = Module::new(module.name.clone(), module.span);
        self.expand_items(module, &mut rewritten, false);
        macro_rules! replace { ($($field:ident),*) => { $(module.$field = rewritten.$field;)* }; }
        replace!(
            localparams,
            variables,
            digital_variables,
            genvars,
            branches,
            functions,
            analog_block,
            analog_initial,
            analog_final,
            nets,
            digital_nets,
            instances,
            continuous_assigns,
            digital_processes
        );
        for parameter in &mut module.parameters {
            self.substitute_range(&mut parameter.packed_range);
            for dimension in &mut parameter.dimensions {
                self.substitute(&mut dimension.start);
                self.substitute(&mut dimension.end);
            }
            if let Some(value) = &mut parameter.default {
                self.substitute(value);
            }
        }
        for port in &mut module.port_declarations {
            self.substitute_range(&mut port.range);
            for item in &mut port.initializers {
                self.substitute(&mut item.1);
            }
        }
        self.constants.definitions.truncate(constants_len);
        self.genvars.truncate(genvars_len);
    }
}

pub(super) fn iteration_parameter(name: &SmolStr, value: i64, span: Span) -> ParameterDecl {
    ParameterDecl {
        is_given: false,
        param_type: ParamType::Integer,
        type_is_explicit: true,
        signedness: None,
        packed_range: None,
        name: name.clone(),
        dimensions: Vec::new(),
        default: Some(Expression::Number(NumberLit {
            value: value as f64,
            raw: format!("32'sb{:032b}", value as i32 as u32).into(),
            span,
        })),
        range: None,
        units: None,
        description: None,
        attributes: Vec::new(),
        span,
    }
}
