//! `$analog_node_alias` and `$analog_port_alias` (Verilog-AMS LRM 2.4
//! section 9): binding a local net to a node of the deck.
//!
//! `$analog_node_alias(net, "xdiv.m")` makes the declared, non-port
//! continuous net `net` the deck node `xdiv.m`; `$analog_port_alias` does the
//! same for a port of an instance, which is a node once the deck is
//! flattened. Each may be called only in `analog initial`, with constant
//! arguments: the net an identifier, the path a string literal.
//!
//! The net becomes a hidden terminal of the module: a trailing port that no
//! formal port owns, which the host connects to the deck node or leaves
//! unconnected when the path names nothing. Reads and contributions go
//! through an ordinary terminal, so the Jacobian is exact in every analysis,
//! and the call's value is the terminal's `$port_connected`: 1 when bound and
//! 0 when not, as the LRM defines it.

use smol_str::SmolStr;

use super::{AnalyzedModule, AnalyzedPort, SemanticAnalyzer, SymbolKind};
use crate::ast::{Expression, Identifier, PortDirection, SystemFunction};
use crate::canonical_ir::HirNodeAlias;
use crate::disciplines::Domain;
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;

/// Whether `name` is one of the two alias calls.
pub(super) fn is_alias_call(name: &str) -> bool {
    matches!(name, "$analog_node_alias" | "$analog_port_alias")
}

fn refuse(name: &str, detail: impl std::fmt::Display, span: Span) -> CompileError {
    CompileError::Semantic(SemanticError::new(
        SemanticErrorKind::UnsupportedFeature(format!("`{name}`: {detail}")),
        span,
    ))
}

impl SemanticAnalyzer {
    /// Record one alias call, refusing it outside `analog initial` or with
    /// arguments that are not constant, and return the net it binds.
    pub(super) fn record_node_alias(
        &mut self,
        name: &str,
        args: &[Expression],
        span: Span,
    ) -> CompileResult<SmolStr> {
        if !self.in_analog_initial {
            return Err(refuse(
                name,
                "may be called only in `analog initial` (Verilog-AMS LRM 2.4 section 9)",
                span,
            ));
        }
        let [net, path] = args else {
            return Err(refuse(
                name,
                format!(
                    "takes a local net and a path, and was given {} argument(s)",
                    args.len()
                ),
                span,
            ));
        };
        let Expression::StringLit(path) = path else {
            return Err(refuse(
                name,
                "the path must be a constant string literal",
                span,
            ));
        };
        if path.value.trim().is_empty() {
            return Err(refuse(name, "the path names nothing", span));
        }
        let Expression::Identifier(net) = net else {
            return Err(refuse(
                name,
                "the first argument must name a net the module declares",
                span,
            ));
        };
        let symbol = self.symbols.lookup(&net.name);
        let continuous = symbol
            .and_then(|symbol| symbol.attrs.discipline.as_ref())
            .and_then(|discipline| self.disciplines.get_discipline(discipline))
            .is_some_and(|discipline| discipline.domain == Domain::Continuous);
        match symbol.map(|symbol| (&symbol.kind, symbol.attrs.is_internal)) {
            Some((SymbolKind::Node, true)) if continuous => {}
            Some((SymbolKind::Port, _)) => {
                return Err(refuse(
                    name,
                    format!(
                        "`{}` is a port; only a net the module declares inside itself can be \
                         bound to a deck node",
                        net.name
                    ),
                    span,
                ));
            }
            _ => {
                return Err(refuse(
                    name,
                    format!(
                        "`{}` is not a scalar continuous net the module declares",
                        net.name
                    ),
                    span,
                ));
            }
        }
        if self
            .node_aliases
            .iter()
            .any(|(alias, _)| alias.net == net.name)
        {
            return Err(refuse(
                name,
                format!("`{}` is already bound to a deck node", net.name),
                span,
            ));
        }
        self.node_aliases.push((
            HirNodeAlias {
                net: net.name.clone(),
                path: path.value.clone(),
                port: name == "$analog_port_alias",
            },
            span,
        ));
        Ok(net.name.clone())
    }

    /// The expression form: the call's value is whether the host bound the
    /// net, which is its hidden terminal's `$port_connected`.
    pub(super) fn lower_node_alias_call(
        &mut self,
        call: &SystemFunction,
    ) -> CompileResult<Expression> {
        let net = self.record_node_alias(&call.name, &call.args, call.span)?;
        Ok(Expression::SystemFunction(SystemFunction {
            name: "$port_connected".into(),
            args: vec![Expression::Identifier(Identifier {
                name: net,
                span: call.span,
            })],
            span: call.span,
        }))
    }

    /// Move every aliased net out of the internal nodes and onto the end of
    /// the ports, in call order, where no formal port owns it.
    pub(super) fn promote_node_aliases(
        &mut self,
        analyzed: &mut AnalyzedModule,
    ) -> CompileResult<()> {
        for (alias, span) in std::mem::take(&mut self.node_aliases) {
            let Some(position) = analyzed
                .internal_nodes
                .iter()
                .position(|node| node.name == alias.net)
            else {
                return Err(refuse(
                    if alias.port {
                        "$analog_port_alias"
                    } else {
                        "$analog_node_alias"
                    },
                    format!("`{}` is not a node of the module's own", alias.net),
                    span,
                ));
            };
            let node = analyzed.internal_nodes.remove(position);
            let discipline = self.disciplines.get_discipline(&node.discipline);
            analyzed.ports.push(AnalyzedPort {
                name: node.name.clone(),
                direction: PortDirection::Inout,
                discipline: node.discipline.clone(),
                nature_potential: discipline
                    .and_then(|discipline| discipline.potential.as_deref())
                    .map(SmolStr::from),
                nature_flow: discipline
                    .and_then(|discipline| discipline.flow.as_deref())
                    .map(SmolStr::from),
                supply_sensitivity: None,
                ground_sensitivity: None,
                coerced_from_input: false,
            });
            if let Some(symbol) = analyzed.symbol_table.lookup_mut(&alias.net) {
                symbol.kind = SymbolKind::Port;
                symbol.attrs.is_internal = false;
                symbol.attrs.direction = Some(PortDirection::Inout);
            }
            analyzed.node_aliases.push(alias);
        }
        for (index, node) in analyzed.internal_nodes.iter_mut().enumerate() {
            node.index = index;
        }
        Ok(())
    }
}
