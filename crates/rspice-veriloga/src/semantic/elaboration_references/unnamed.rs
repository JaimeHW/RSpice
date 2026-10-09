//! Match explicit branch references before electrical connections erase local
//! net identities, and retain the direction of both declarations and reads.
use super::*;
use crate::ast::ForeignPhysicalNode;

type Key = (SmolStr, ForeignPhysicalNode, ForeignPhysicalNode);

#[derive(Default)]
pub(super) struct UnnamedReferences {
    owners: HashSet<SmolStr>,
    requested: BTreeMap<Key, (Span, bool)>,
    aliases: BTreeMap<SmolStr, (Key, i8, Span)>,
    targets: BTreeMap<Key, (SmolStr, i8)>,
}

fn qualify_node(source: &Module, owner: &str, node: &ForeignPhysicalNode) -> ForeignPhysicalNode {
    if node.name == "0" && node.path.is_empty() {
        return node.clone();
    }
    ForeignPhysicalNode {
        path: reference_path(source, owner, &node.path),
        name: node.name.clone(),
    }
}

fn key(owner: &str, pos: ForeignPhysicalNode, neg: ForeignPhysicalNode) -> (Key, i8) {
    if pos <= neg {
        ((owner.into(), pos, neg), 1)
    } else {
        ((owner.into(), neg, pos), -1)
    }
}

fn local_aliases(
    source: &Module,
    module: &AnalyzedModule,
    owner: &str,
) -> HashMap<SmolStr, ForeignPhysicalNode> {
    let mut aliases = HashMap::new();
    for (alias, reference) in &source.foreign_physical {
        if !matches!(reference.kind, ForeignPhysicalKind::Node { .. }) {
            continue;
        }
        for (lane, target) in module
            .physical_nodes
            .reference_lanes(alias, false)
            .into_iter()
            .zip(&reference.lanes)
        {
            aliases.insert(
                lane,
                ForeignPhysicalNode {
                    path: reference_path(source, owner, &reference.path),
                    name: target.clone(),
                },
            );
        }
    }
    aliases
}

fn local_node(
    module: &AnalyzedModule,
    owner: &str,
    name: &SmolStr,
    aliases: &HashMap<SmolStr, ForeignPhysicalNode>,
) -> ForeignPhysicalNode {
    if name.is_empty() || name == "0" || module.ground_nodes.contains(name) {
        return ForeignPhysicalNode {
            path: "".into(),
            name: "0".into(),
        };
    }
    aliases
        .get(name)
        .cloned()
        .unwrap_or_else(|| ForeignPhysicalNode {
            path: owner.into(),
            name: name.clone(),
        })
}

impl UnnamedReferences {
    pub(super) fn request(
        &mut self,
        source: &Module,
        module: &AnalyzedModule,
        owner: &str,
        inventory: &super::super::super::flow_probes::HierarchyBranches,
    ) {
        for (name, reference) in &source.foreign_physical {
            let ForeignPhysicalKind::Unnamed { pairs } = &reference.kind else {
                continue;
            };
            let path = reference_path(source, owner, &reference.path);
            self.owners.insert(path.clone());
            for (alias, (pos, neg)) in module
                .physical_nodes
                .reference_lanes(name, true)
                .iter()
                .zip(pairs)
            {
                let (key, _) = key(
                    &path,
                    qualify_node(source, &path, pos),
                    qualify_node(source, &path, neg),
                );
                let entry = self.requested.entry(key).or_insert((reference.span, false));
                entry.1 |= inventory.conducting_named.contains(alias);
            }
        }
    }

    pub(super) fn extend_inventory(
        &self,
        source: &Module,
        module: &AnalyzedModule,
        owner: &str,
        inventory: &mut super::super::super::flow_probes::HierarchyBranches,
    ) {
        if !self.owners.contains(owner) {
            return;
        }
        let aliases = local_aliases(source, module, owner);
        for (pos, neg) in inventory.unnamed.keys() {
            let (key, _) = key(
                owner,
                local_node(module, owner, pos, &aliases),
                local_node(module, owner, neg, &aliases),
            );
            if self
                .requested
                .get(&key)
                .is_some_and(|(_, conducting)| *conducting)
            {
                inventory
                    .conducting_unnamed
                    .insert((pos.clone(), neg.clone()));
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
        for (name, reference) in &source.foreign_physical {
            let ForeignPhysicalKind::Unnamed { pairs } = &reference.kind else {
                continue;
            };
            let aliases = module.physical_nodes.reference_lanes(name, true);
            if aliases.len() != pairs.len() {
                return Err(internal_error(
                    "foreign unnamed branch changed shape".into(),
                ));
            }
            let path = reference_path(source, owner, &reference.path);
            for (alias, (pos, neg)) in aliases.iter().zip(pairs) {
                let (key, sign) = key(
                    &path,
                    qualify_node(source, &path, pos),
                    qualify_node(source, &path, neg),
                );
                let name = scope
                    .branches
                    .get(alias)
                    .ok_or_else(|| internal_error("missing borrowed unnamed branch".into()))?;
                self.aliases
                    .insert(name.clone(), (key, sign, reference.span));
            }
        }
        if !self.owners.contains(owner) {
            return Ok(());
        }
        let aliases = local_aliases(source, module, owner);
        for ((pos, neg), branch) in &scope.unnamed_branches {
            let (key, sign) = key(
                owner,
                local_node(module, owner, pos, &aliases),
                local_node(module, owner, neg, &aliases),
            );
            if self.requested.contains_key(&key) {
                self.targets.insert(key, (branch.name.clone(), sign));
            }
        }
        Ok(())
    }

    pub(super) fn resolve(&self) -> CompileResult<BTreeMap<SmolStr, (SmolStr, i8)>> {
        self.aliases.iter().map(|(alias, (key, sign, span))| {
            let (name, target_sign) = self.targets.get(key).ok_or_else(|| semantic_error(
                SemanticErrorKind::InvalidExpression(format!(
                    "hierarchical branch() reference has no existing unnamed branch in instance '{}' between '{}.{}' and '{}.{}'",
                    key.0, key.1.path, key.1.name, key.2.path, key.2.name)), *span))?;
            Ok((alias.clone(), (name.clone(), sign * target_sign)))
        }).collect()
    }
}
