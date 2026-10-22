//! Resolve the SPICE leaves of one module occurrence to the nets they connect.
//!
//! A leaf ([`crate::external_cells`]) left its module before analysis, so the
//! flattened hierarchy has no instance of it. Each occurrence's scope says
//! what every name it connects stands for after flattening: a port of the
//! selected root, ground, or a net inside the design, which the host makes a
//! circuit node of.

use super::{HierarchyElaborator, ScopeMap};
use crate::ast::{Expression, Module};
use crate::canonical_ir::{HirExternalLeaf, HirExternalNet, PortId};
use crate::error::{CompileResult, SemanticError, SemanticErrorKind};
use crate::external_cells::unsupported;
use smol_str::SmolStr;

impl HierarchyElaborator<'_> {
    /// List the leaves `source` declares, for the occurrence at `relative`
    /// below the root (empty at the root) whose scope is `scope`.
    pub(super) fn resolve_external_leaves(
        &mut self,
        source: &Module,
        scope: &ScopeMap,
        relative: &str,
    ) -> CompileResult<()> {
        for leaf in &source.external_leaves {
            let instance = qualify(relative, &leaf.name);
            let mut connections = Vec::with_capacity(leaf.actuals.len());
            for (formal, actual) in &leaf.actuals {
                let subject = format!(
                    "port `{formal}` of instance `{instance}` of SPICE cell `{}`",
                    leaf.cell
                );
                connections.push(self.external_net(source, scope, relative, &subject, actual)?);
            }
            self.flattened.external_leaves.push(HirExternalLeaf {
                cell: leaf.cell.clone(),
                instance,
                connections,
                parameters: leaf.parameters.clone(),
            });
        }
        Ok(())
    }

    fn external_net(
        &self,
        source: &Module,
        scope: &ScopeMap,
        relative: &str,
        subject: &str,
        actual: &Expression,
    ) -> CompileResult<HirExternalNet> {
        let Expression::Identifier(identifier) = actual else {
            return Err(unsupported(
                format!("{subject} must connect to a scalar net or to ground"),
                actual.span(),
            ));
        };
        let name = &identifier.name;
        if declares_vector(source, name) {
            return Err(unsupported(
                format!(
                    "{subject} connects vector `{name}`; a SPICE cell port is one scalar net, and \
                     this compiler does not spread a vector over ports, so connect one bit of it"
                ),
                identifier.span,
            ));
        }
        if scope.ground_nodes.contains(name) {
            return Ok(HirExternalNet::Ground);
        }
        if scope.discrete_nets.contains(name) {
            return Err(unsupported(
                format!(
                    "{subject} connects discrete net `{name}`; a SPICE cell port is a circuit \
                     node, so declare `{name}` with a continuous discipline"
                ),
                identifier.span,
            ));
        }
        if let Some(binding) = scope.nodes.get(name) {
            return Ok(self.external_node(&binding.name));
        }
        // A continuous net nothing in the analog half reads or contributes to
        // has no node there; the host makes one for the leaf.
        if source
            .nets
            .iter()
            .any(|net| net.names.iter().any(|declared| declared == name))
        {
            return Ok(HirExternalNet::Net(qualify(relative, name)));
        }
        Err(SemanticError::new(
            SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
            identifier.span,
        )
        .into())
    }

    /// What the flattened node `name` is from outside the design.
    fn external_node(&self, name: &SmolStr) -> HirExternalNet {
        if name == "0" || self.flattened.ground_nodes.contains(name) {
            HirExternalNet::Ground
        } else if let Some(index) = self
            .flattened
            .ports
            .iter()
            .position(|port| &port.name == name)
        {
            HirExternalNet::Port(PortId::from(index))
        } else {
            HirExternalNet::Net(name.clone())
        }
    }
}

/// Whether `module` declares `name` as a vector or an array.
fn declares_vector(module: &Module, name: &str) -> bool {
    module.nets.iter().any(|net| {
        net.names.iter().any(|declared| declared == name)
            && (net.range.is_some() || net.dimensions.iter().any(|(array, _)| array == name))
    }) || module
        .port_declarations
        .iter()
        .any(|port| port.range.is_some() && port.names.iter().any(|declared| declared == name))
        || module.digital_nets.iter().any(|net| {
            net.items.iter().any(|item| {
                item.name == name && (net.range.is_some() || !item.dimensions.is_empty())
            })
        })
        || module.digital_variables.iter().any(|variable| {
            variable.items.iter().any(|item| {
                item.name == name && (variable.range.is_some() || !item.dimensions.is_empty())
            })
        })
}

fn qualify(prefix: &str, leaf: &str) -> SmolStr {
    if prefix.is_empty() {
        SmolStr::from(leaf)
    } else {
        SmolStr::from(format!("{prefix}.{leaf}"))
    }
}
