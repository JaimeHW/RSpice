//! Resolve what one module occurrence links outside the design.
//!
//! A SPICE leaf ([`crate::external_cells`]) left its module before analysis,
//! so the flattened hierarchy has no instance of it, and a discrete port on a
//! continuous net of its parent kept a signal of its own in the discrete pass
//! ([`super::super::digital_elaborate::HoistedConnection`]). Each occurrence's
//! scope says what every net they name stands for after flattening: a port of
//! the selected root, ground, or a net inside the design, which the host makes
//! a circuit node of.

use super::{HierarchyElaborator, ScopeMap};
use crate::ast::{DigitalVariableKind, Expression, Identifier, Module, PortDirection};
use crate::canonical_ir::{HirExternalLeaf, HirExternalNet, HirHoistedPort, PortId};
use crate::error::{CompileResult, SemanticError, SemanticErrorKind};
use crate::external_cells::unsupported;
use crate::source::Span;
use smol_str::SmolStr;

impl HierarchyElaborator<'_> {
    /// List what the occurrence at `relative` below the root (empty at the
    /// root), whose scope is `scope`, links outside the design: the discrete
    /// ports of its children on its continuous nets, and the leaves `source`
    /// declares.
    pub(super) fn resolve_external_leaves(
        &mut self,
        source: &Module,
        scope: &ScopeMap,
        relative: &str,
    ) -> CompileResult<()> {
        for connection in self.hoisted.remove(relative).unwrap_or_default() {
            let subject = format!(
                "discrete port `{}` on continuous net `{}`",
                connection.signal, connection.net
            );
            let net = self.continuous_net(
                source,
                scope,
                relative,
                &subject,
                &connection.net,
                connection.span,
            )?;
            if net == HirExternalNet::Ground {
                return Err(unsupported(
                    format!(
                        "{subject}: the net is ground, the voltage reference rather than a net \
                         a discrete value can be read from or driven onto"
                    ),
                    connection.span,
                ));
            }
            self.hoist(HirHoistedPort {
                signal: connection.signal,
                bit: (!connection.real).then_some(0),
                direction: connection.direction,
                net,
            });
        }
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

    fn hoist(&mut self, port: HirHoistedPort) {
        if !self.flattened.hoisted_ports.contains(&port) {
            self.flattened.hoisted_ports.push(port);
        }
    }

    fn external_net(
        &mut self,
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
        if declares_vector(source, &identifier.name) {
            return Err(vector_refusal(subject, identifier));
        }
        if scope.discrete_nets.contains(&identifier.name)
            && !scope.ground_nodes.contains(&identifier.name)
        {
            return self.discrete_net(source, relative, subject, identifier);
        }
        self.continuous_net(
            source,
            scope,
            relative,
            subject,
            &identifier.name,
            identifier.span,
        )
    }

    fn continuous_net(
        &self,
        source: &Module,
        scope: &ScopeMap,
        relative: &str,
        subject: &str,
        name: &SmolStr,
        span: Span,
    ) -> CompileResult<HirExternalNet> {
        if scope.ground_nodes.contains(name) {
            return Ok(HirExternalNet::Ground);
        }
        if let Some(binding) = scope.nodes.get(name) {
            return Ok(self.external_node(&binding.name));
        }
        if declares_vector(source, name) {
            return Err(vector_refusal(
                subject,
                &Identifier {
                    name: name.clone(),
                    span,
                },
            ));
        }
        // A continuous net nothing in the analog half reads or contributes to
        // has no node there; the host makes one.
        if source
            .nets
            .iter()
            .any(|net| net.names.iter().any(|declared| declared == name))
        {
            return Ok(HirExternalNet::Net(qualify(relative, name)));
        }
        Err(SemanticError::new(
            SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
            span,
        )
        .into())
    }

    /// A leaf port on a discrete net reads a variable the design drives; the
    /// variable is linked on a circuit node of its own, as a deck-level output
    /// is, and the converter that node needs is the deck's to choose.
    fn discrete_net(
        &mut self,
        source: &Module,
        relative: &str,
        subject: &str,
        identifier: &Identifier,
    ) -> CompileResult<HirExternalNet> {
        let name = &identifier.name;
        let variable = source.digital_variables.iter().any(|variable| {
            variable.kind == DigitalVariableKind::Reg
                && variable.items.iter().any(|item| &item.name == name)
        });
        if !variable {
            return Err(unsupported(
                format!(
                    "{subject} connects discrete net `{name}`; a SPICE cell port reads a `reg` \
                     the design drives, or joins a continuous net, where each discrete port is \
                     converted on its own, so declare `{name}` as one of those"
                ),
                identifier.span,
            ));
        }
        let signal = if relative.is_empty() {
            name.clone()
        } else {
            self.digital_signal_name(relative, name)?
        };
        if relative.is_empty()
            && let Some(index) = self
                .flattened
                .ports
                .iter()
                .position(|port| port.name == signal)
        {
            // A port of the root is already linked on the deck's node.
            return Ok(HirExternalNet::Port(PortId::from(index)));
        }
        let net = HirExternalNet::Net(signal.clone());
        self.hoist(HirHoistedPort {
            signal,
            bit: Some(0),
            direction: PortDirection::Output,
            net: net.clone(),
        });
        Ok(net)
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

fn vector_refusal(subject: &str, identifier: &Identifier) -> crate::error::CompileError {
    unsupported(
        format!(
            "{subject} connects vector `{}`; a SPICE cell port is one scalar net, and this \
             compiler does not spread a vector over ports, so connect one bit of it",
            identifier.name
        ),
        identifier.span,
    )
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
