//! Link borrowed physical declarations to the concrete hierarchy. No aliases
//! survive into HIR, and a node reference never acquires the target's branch.
use super::*;
use crate::ast::ForeignPhysicalKind;

type Key = (SmolStr, SmolStr);
type Pending = BTreeMap<SmolStr, (Key, Span)>;

#[derive(Default)]
pub(super) struct PhysicalReferences {
    nodes: BTreeMap<Key, NodeBinding>,
    branches: BTreeMap<Key, SmolStr>,
    ports: BTreeMap<Key, SmolStr>,
    node_aliases: Pending,
    branch_aliases: Pending,
    port_aliases: Pending,
    requested_ports: BTreeMap<Key, Span>,
    requested_branches: HashSet<Key>,
    node_targets: HashSet<Key>,
    branch_targets: HashSet<Key>,
}

fn qualify(owner: &str, target: &str) -> SmolStr {
    match (owner.is_empty(), target.is_empty()) {
        (true, _) => target.into(),
        (_, true) => owner.into(),
        _ => format!("{owner}.{target}").into(),
    }
}

pub(super) fn canonicalize_node_references(
    source: &Module,
    module: &AnalyzedModule,
    inventory: &mut super::super::flow_probes::HierarchyBranches,
) -> HashMap<SmolStr, SmolStr> {
    let mut identities = BTreeMap::new();
    let mut aliases = HashMap::new();
    for (name, reference) in &source.foreign_physical {
        if !matches!(reference.kind, ForeignPhysicalKind::Node { .. }) {
            continue;
        }
        for (alias, lane) in module
            .physical_nodes
            .reference_lanes(name, false)
            .into_iter()
            .zip(&reference.lanes)
        {
            let canonical = identities
                .entry((reference.path.clone(), lane.clone()))
                .or_insert_with(|| alias.clone());
            aliases.insert(alias, canonical.clone());
        }
    }
    let pair = |(pos, neg): (SmolStr, SmolStr)| {
        let pos = aliases.get(&pos).unwrap_or(&pos);
        let neg = aliases.get(&neg).unwrap_or(&neg);
        let (pos, neg, _) =
            super::super::flow_probes::canonical_node_pair(pos, neg, &module.ground_nodes);
        (pos, neg)
    };
    inventory.unnamed = std::mem::take(&mut inventory.unnamed)
        .into_iter()
        .map(|(key, span)| (pair(key), span))
        .collect();
    inventory.conducting_unnamed = std::mem::take(&mut inventory.conducting_unnamed)
        .into_iter()
        .map(pair)
        .collect();
    aliases
}

pub(super) fn is_borrowed_branch(source: &Module, module: &AnalyzedModule, lane: &SmolStr) -> bool {
    source.foreign_physical.iter().any(|(name, reference)| {
        matches!(reference.kind, ForeignPhysicalKind::Branch)
            && module
                .physical_nodes
                .reference_lanes(name, true)
                .contains(lane)
    })
}

impl PhysicalReferences {
    pub(super) fn has_node_aliases(&self) -> bool {
        !self.node_aliases.is_empty()
    }

    pub(super) fn request(
        &mut self,
        source: &Module,
        module: &AnalyzedModule,
        owner: &str,
        inventory: &super::super::flow_probes::HierarchyBranches,
    ) {
        for (name, reference) in &source.foreign_physical {
            let path = qualify(owner, &reference.path);
            let branch = matches!(reference.kind, ForeignPhysicalKind::Branch);
            let aliases = module.physical_nodes.reference_lanes(name, branch);
            for (alias, lane) in aliases.iter().zip(&reference.lanes) {
                let key = (path.clone(), lane.clone());
                if branch {
                    self.branch_targets.insert(key.clone());
                } else {
                    self.node_targets.insert(key.clone());
                }
                match reference.kind {
                    ForeignPhysicalKind::Node { is_port: true } => {
                        self.requested_ports.insert(key, reference.span);
                    }
                    ForeignPhysicalKind::Branch if inventory.conducting_named.contains(alias) => {
                        self.requested_branches.insert(key);
                    }
                    _ => {}
                }
            }
        }
    }

    pub(super) fn extend_inventory(
        &self,
        path: &str,
        inventory: &mut super::super::flow_probes::HierarchyBranches,
    ) {
        for ((owner, name), span) in &self.requested_ports {
            if owner == path {
                inventory.port_flows.insert(name.clone(), *span);
            }
        }
        for (owner, name) in &self.requested_branches {
            if owner == path {
                inventory.conducting_named.insert(name.clone());
            }
        }
    }

    pub(super) fn register(
        &mut self,
        source: &Module,
        module: &AnalyzedModule,
        owner: &str,
        scope: &ScopeMap,
    ) -> CompileResult<()> {
        let owner: SmolStr = owner.into();
        for (name, node) in &scope.nodes {
            let key = (owner.clone(), name.clone());
            if self.node_targets.contains(&key) {
                self.nodes.insert(key, node.clone());
            }
        }
        for (name, branch) in &scope.branches {
            let key = (owner.clone(), name.clone());
            if self.branch_targets.contains(&key) {
                self.branches.insert(key, branch.clone());
            }
        }
        for (name, token) in &scope.port_flows {
            let key = (owner.clone(), name.clone());
            if self.requested_ports.contains_key(&key) {
                self.ports.insert(key, token.clone());
            }
        }
        for (name, reference) in &source.foreign_physical {
            let path = qualify(&owner, &reference.path);
            let branch = matches!(reference.kind, ForeignPhysicalKind::Branch);
            let aliases = module.physical_nodes.reference_lanes(name, branch);
            if aliases.len() != reference.lanes.len() {
                return Err(internal_error(
                    "foreign physical reference changed shape during occurrence binding".into(),
                ));
            }
            for (alias, lane) in aliases.iter().zip(&reference.lanes) {
                let key = (path.clone(), lane.clone());
                match reference.kind {
                    ForeignPhysicalKind::Branch => {
                        let name = scope
                            .branches
                            .get(alias)
                            .ok_or_else(|| internal_error("missing borrowed branch".into()))?;
                        self.branch_aliases
                            .insert(name.clone(), (key, reference.span));
                    }
                    ForeignPhysicalKind::Node { is_port } => {
                        let node = scope
                            .nodes
                            .get(alias)
                            .ok_or_else(|| internal_error("missing borrowed node".into()))?;
                        // Child ground aliases already resolve to global ground.
                        if node.name != "0" {
                            self.node_aliases
                                .insert(node.name.clone(), (key.clone(), reference.span));
                        }
                        if is_port {
                            let token = if owner.is_empty() {
                                Some(alias)
                            } else {
                                scope.port_flows.get(alias)
                            };
                            if let Some(token) = token {
                                self.port_aliases
                                    .insert(token.clone(), (key, reference.span));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(super) fn finish(
        &self,
        module: &mut AnalyzedModule,
        ports: &mut BTreeMap<SmolStr, PortFlow>,
        terms: &[(SmolStr, NodeBinding, SmolStr, i8)],
    ) -> CompileResult<()> {
        let mut nodes = BTreeMap::new();
        if self.node_aliases.is_empty()
            && self.branch_aliases.is_empty()
            && self.port_aliases.is_empty()
        {
            record_terms(ports, terms, &nodes);
            return Ok(());
        }
        for (alias, (key, span)) in &self.node_aliases {
            let mut node = self
                .nodes
                .get(key)
                .cloned()
                .ok_or_else(|| missing(key, *span))?;
            let mut seen = HashSet::new();
            while let Some((next, span)) = self.node_aliases.get(&node.name) {
                if !seen.insert(node.name.clone()) {
                    return Err(internal_error("cyclic physical reference binding".into()));
                }
                let next = self.nodes.get(next).ok_or_else(|| missing(next, *span))?;
                let mut boundaries = node.port_boundaries;
                boundaries.extend(next.port_boundaries.iter().cloned());
                node = next.clone();
                node.port_boundaries = boundaries;
            }
            nodes.insert(alias.clone(), node);
        }
        let mut names = BTreeMap::new();
        for (aliases, targets) in [
            (&self.branch_aliases, &self.branches),
            (&self.port_aliases, &self.ports),
        ] {
            for (alias, (key, span)) in aliases {
                names.insert(
                    alias.clone(),
                    targets
                        .get(key)
                        .cloned()
                        .ok_or_else(|| missing(key, *span))?,
                );
            }
        }
        let rename = |name: &mut SmolStr| {
            if let Some(bound) = nodes.get(name) {
                *name = bound.name.clone();
            }
        };
        let rewrite = |expression: &mut Expression| {
            let mut pending = vec![expression];
            while let Some(expression) = pending.pop() {
                if let Expression::BranchAccess(access) = expression {
                    match access {
                        BranchAccess::Nodes { pos, neg, .. } => {
                            if neg.is_none()
                                && self.branch_aliases.contains_key(pos)
                                && let Some(branch) = names.get(pos)
                            {
                                *pos = branch.clone();
                            } else {
                                rename(pos);
                                if let Some(neg) = neg {
                                    rename(neg);
                                }
                            }
                        }
                        BranchAccess::Branch { name, .. } => {
                            if let Some(bound) = names.get(name) {
                                *name = bound.clone();
                            }
                        }
                    }
                }
                super::super::flow_probes::for_child_mut(expression, &mut |child| {
                    pending.push(child)
                });
            }
        };
        super::super::flow_probes::rewrite_module_expressions(module, &rewrite);
        let contribution = |c: &mut AnalyzedContribution| {
            let (pos, neg) = c
                .branch
                .split_once(',')
                .map_or((c.branch.as_str(), None), |(p, n)| (p, Some(n)));
            let mut pos: SmolStr = pos.into();
            rename(&mut pos);
            c.branch = if let Some(neg) = neg {
                let mut neg: SmolStr = neg.into();
                rename(&mut neg);
                format!("{pos},{neg}").into()
            } else {
                pos
            };
            if let Some(name) = &mut c.declared_branch
                && let Some(bound) = names.get(name)
            {
                *name = bound.clone();
            }
            if let Some(abstol) = &mut c.equation_abstol {
                rewrite(abstol);
            }
        };
        for c in &mut module.contributions {
            contribution(c);
        }
        let mut pending = vec![module.body.as_mut_slice()];
        while let Some(body) = pending.pop() {
            for region in body {
                match region {
                    AnalyzedRegion::Contribution(c) => contribution(c),
                    AnalyzedRegion::Conditional {
                        then_body,
                        else_body,
                        ..
                    } => {
                        pending.push(then_body);
                        pending.push(else_body);
                    }
                    AnalyzedRegion::Loop { body, .. }
                    | AnalyzedRegion::Initialization { body, .. } => pending.push(body),
                    _ => {}
                }
            }
        }
        module
            .internal_nodes
            .retain(|node| !self.node_aliases.contains_key(&node.name));
        for (index, node) in module.internal_nodes.iter_mut().enumerate() {
            node.index = index;
        }
        module
            .ground_nodes
            .retain(|name| !self.node_aliases.contains_key(name));
        module
            .branches
            .retain(|branch| !self.branch_aliases.contains_key(&branch.name));
        for branch in &mut module.branches {
            rename(&mut branch.pos_node);
            rename(&mut branch.neg_node);
        }
        module.physical_nodes.external_ports.clear();
        record_terms(ports, terms, &nodes);
        Ok(())
    }
}

fn record_terms(
    ports: &mut BTreeMap<SmolStr, PortFlow>,
    terms: &[(SmolStr, NodeBinding, SmolStr, i8)],
    nodes: &BTreeMap<SmolStr, NodeBinding>,
) {
    for (owner, binding, branch, sign) in terms {
        let resolved = nodes.get(&binding.name);
        let mut boundaries = binding.port_boundaries.iter().collect::<HashSet<_>>();
        if let Some(node) = resolved {
            boundaries.extend(&node.port_boundaries);
        }
        for boundary in boundaries {
            let Some(port) = ports.get_mut(boundary) else {
                continue;
            };
            // An OOMR node source belongs to its caller. It cannot count as
            // a source inside the descendant whose net it happens to use.
            if port.owner != *owner
                && !owner
                    .strip_prefix(port.owner.as_str())
                    .is_some_and(|rest| rest.starts_with('.'))
            {
                continue;
            }
            let coefficient = port.terms.entry(branch.clone()).or_default();
            *coefficient += sign;
            if *coefficient == 0 {
                port.terms.remove(branch);
            }
        }
    }
}

fn missing(key: &Key, span: Span) -> CompileError {
    semantic_error(
        SemanticErrorKind::InvalidExpression(format!(
            "hierarchical physical reference '{}.{}' has no matching elaborated storage",
            key.0, key.1,
        )),
        span,
    )
}
