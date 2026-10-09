//! Link borrowed physical declarations to the concrete hierarchy. No aliases
//! survive into HIR, and a node reference never acquires the target's branch.
use super::*;
use crate::ast::ForeignPhysicalKind;

#[path = "elaboration_references/unnamed.rs"]
mod unnamed;

type Key = (SmolStr, SmolStr);
type Pending = BTreeMap<SmolStr, (Key, Span)>;

#[derive(Default)]
pub(super) struct PhysicalReferences {
    unnamed: unnamed::UnnamedReferences,
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

fn reference_path(source: &Module, owner: &str, target: &str) -> SmolStr {
    if source.reference_context.is_some() {
        target.into()
    } else {
        qualify(owner, target)
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
        reference.kind.is_branch()
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
        self.unnamed.request(source, module, owner, inventory);
        for (name, reference) in &source.foreign_physical {
            if matches!(reference.kind, ForeignPhysicalKind::Unnamed { .. }) {
                continue;
            }
            let path = reference_path(source, owner, &reference.path);
            let branch = reference.kind.is_branch();
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
        source: &Module,
        module: &AnalyzedModule,
        path: &str,
        inventory: &mut super::super::flow_probes::HierarchyBranches,
    ) {
        self.unnamed
            .extend_inventory(source, module, path, inventory);
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
        self.unnamed.register(source, module, owner, scope)?;
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
            if matches!(reference.kind, ForeignPhysicalKind::Unnamed { .. }) {
                continue;
            }
            let path = reference_path(source, &owner, &reference.path);
            let branch = reference.kind.is_branch();
            let aliases = module.physical_nodes.reference_lanes(name, branch);
            if aliases.len() != reference.lanes.len() {
                return Err(internal_error(
                    "foreign physical reference changed shape during occurrence binding".into(),
                ));
            }
            for (alias, lane) in aliases.iter().zip(&reference.lanes) {
                let key = (path.clone(), lane.clone());
                match reference.kind {
                    ForeignPhysicalKind::Unnamed { .. } => unreachable!("handled above"),
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
        let unnamed = self.unnamed.resolve()?;
        let mut nodes = BTreeMap::new();
        if unnamed.is_empty()
            && self.node_aliases.is_empty()
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
        for (alias, (name, _)) in &unnamed {
            names.insert(alias.clone(), name.clone());
        }
        validate_contributions(module, &names)?;
        let rename = |name: &mut SmolStr| {
            if let Some(bound) = nodes.get(name) {
                *name = bound.name.clone();
            }
        };
        let rewrite = |expression: &mut Expression| {
            let mut pending = vec![&mut *expression];
            while let Some(expression) = pending.pop() {
                let mut sign = 1;
                if let Expression::BranchAccess(access) = expression {
                    match access {
                        BranchAccess::Nodes { pos, neg, .. } => {
                            if neg.is_none()
                                && (self.branch_aliases.contains_key(pos)
                                    || unnamed.contains_key(pos))
                                && let Some(branch) = names.get(pos)
                            {
                                sign = unnamed.get(pos).map_or(1, |(_, sign)| *sign);
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
                                sign = unnamed.get(name).map_or(1, |(_, sign)| *sign);
                                *name = bound.clone();
                            }
                        }
                    }
                }
                if sign < 0 {
                    *expression = super::super::flow_probes::signed(expression.clone(), -1.0);
                }
                super::super::flow_probes::for_child_mut(expression, &mut |child| {
                    pending.push(child)
                });
            }
            repair_derivative_axes(expression);
        };
        super::super::flow_probes::rewrite_module_expressions(module, &rewrite);
        let branch_nodes: HashMap<SmolStr, SmolStr> = module
            .branches
            .iter()
            .filter(|b| !names.contains_key(&b.name))
            .map(|b| {
                let mut pos = b.pos_node.clone();
                let mut neg = b.neg_node.clone();
                rename(&mut pos);
                rename(&mut neg);
                (
                    b.name.clone(),
                    format!("{pos},{}", if neg.is_empty() { "0" } else { neg.as_str() }).into(),
                )
            })
            .collect();
        let contribution = |c: &mut AnalyzedContribution, flat: bool| {
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
            let mut sign = 1;
            if let Some(name) = &mut c.declared_branch
                && let Some(bound) = names.get(name)
            {
                sign = unnamed.get(name).map_or(1, |(_, sign)| *sign);
                *name = bound.clone();
                if let Some(nodes) = branch_nodes.get(bound) {
                    c.branch = nodes.clone();
                }
            }
            if sign < 0 {
                super::super::flow_probes::negate_contribution(c, flat);
            }
            if let Some(abstol) = &mut c.equation_abstol {
                rewrite(abstol);
            }
        };
        for c in &mut module.contributions {
            contribution(c, true);
        }
        let mut pending = vec![module.body.as_mut_slice()];
        while let Some(body) = pending.pop() {
            for region in body {
                match region {
                    AnalyzedRegion::Contribution(c) => contribution(c, false),
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
        module.branches.retain(|branch| {
            !self.branch_aliases.contains_key(&branch.name) && !unnamed.contains_key(&branch.name)
        });
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

/// An external contribution may augment a source, but cannot introduce a
/// potential/flow switch or contribute to an indirectly constrained branch.
fn validate_contributions(
    module: &AnalyzedModule,
    names: &BTreeMap<SmolStr, SmolStr>,
) -> CompileResult<()> {
    let targets: HashSet<_> = names.values().collect();
    let mut kinds: BTreeMap<SmolStr, (u8, u8, bool, Option<Span>)> = BTreeMap::new();
    for contribution in &module.contributions {
        let Some(branch) = &contribution.declared_branch else {
            continue;
        };
        let external = names.get(branch);
        let target = external.unwrap_or(branch);
        if !targets.contains(target) {
            continue;
        }
        let entry = kinds.entry(target.clone()).or_default();
        let kind = if contribution.is_current { 1 } else { 2 };
        if external.is_some() {
            if contribution.indirect {
                return Err(semantic_error(
                    SemanticErrorKind::InvalidContribution(
                        "hierarchical indirect contributions are not permitted".into(),
                    ),
                    contribution.span,
                ));
            }
            entry.1 |= kind;
            entry.3 = Some(contribution.span);
        } else {
            entry.0 |= kind;
        }
        entry.2 |= contribution.indirect;
    }
    for (_, (local, external, indirect, span)) in kinds {
        let Some(span) = span else {
            continue;
        };
        if indirect {
            return Err(semantic_error(
                SemanticErrorKind::InvalidContribution(
                    "a hierarchical contribution cannot target an indirectly constrained branch"
                        .into(),
                ),
                span,
            ));
        }
        if local != 3 && (local | external) == 3 {
            return Err(semantic_error(
                SemanticErrorKind::InvalidContribution(
                    "a hierarchical contribution cannot change the target into a switch branch"
                        .into(),
                ),
                span,
            ));
        }
    }
    Ok(())
}

/// A reversed branch probe is a negative expression. ddx requires a bare probe
/// for its axis, so transfer that sign to the derivative result after binding.
pub(super) fn repair_derivative_axes(expression: &mut Expression) {
    let mut pending = vec![expression];
    while let Some(expression) = pending.pop() {
        let args = match expression {
            Expression::Call(call) if call.name == "ddx" => Some(&mut call.args),
            Expression::SystemFunction(call) if call.name == "ddx" => Some(&mut call.args),
            _ => None,
        };
        if let Some(args) = args
            && args.len() == 2
            && matches!(&args[1], Expression::Unary(unary) if unary.op == UnaryOp::Neg)
            && let Expression::Unary(unary) = args.remove(1)
        {
            args.push(*unary.operand);
            *expression = super::super::flow_probes::signed(expression.clone(), -1.0);
        }
        super::super::flow_probes::for_child_mut(expression, &mut |child| pending.push(child));
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
