//! The declaration rules IEEE 1364-2005 sets on a module's own text: each
//! port of the list of ports is declared, a port declared twice states one
//! range (section 12.3.3), and a name is declared before it is used (section
//! 4.5).

use super::SymbolKind;
use super::digital::{AnalyzedDigitalSignal, VectorBounds};
use super::{SemanticAnalyzer, SemanticErrorKind};
use crate::ast::*;
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::HashMap;

/// What the second declaration of a port says about its range.
enum Redeclared {
    /// A net or `reg`, with its range or none.
    Range(Option<VectorBounds>),
    /// An `integer` or a `time`, the keyword named, which declares no range
    /// of its own.
    Unranged(&'static str),
}

impl SemanticAnalyzer {
    /// IEEE 1364-2005 section 12.3.3, for the ports of a module with discrete
    /// content.
    ///
    /// "Each port_identifier in a port_expression in the list of ports for the
    /// module declaration shall also be declared in the body of the module";
    /// a name of the list that nothing in the module declares is no port at
    /// all. And a port declaration with no net or variable type of its own
    /// takes one from a second declaration: "If the net or variable is
    /// declared as a vector, the range specification between the two
    /// declarations of a port shall be identical." An `integer` or a `time`
    /// declares no range, so the port declaration it completes declares none
    /// either.
    pub(super) fn check_port_declarations(&mut self, module: &Module) {
        for port in &module.ports {
            if !Self::declares(module, &port.name) {
                self.record_error_at(
                    SemanticErrorKind::PortWithoutDirection {
                        name: port.name.clone(),
                    },
                    port.span,
                );
            }
        }
        for declaration in module
            .port_declarations
            .iter()
            .filter(|port| port.net_type.is_none() && port.discipline.is_none())
        {
            let Some(port_range) = self.constant_bounds(declaration.range.as_ref()) else {
                continue;
            };
            for name in &declaration.names {
                let Some((redeclared, span)) = self.port_redeclaration(module, name) else {
                    continue;
                };
                let declared = match redeclared {
                    Redeclared::Range(range) if range == port_range => continue,
                    Redeclared::Unranged(_) if port_range.is_none() => continue,
                    Redeclared::Range(range) => Self::range_phrase(range),
                    Redeclared::Unranged(keyword) => {
                        let article = if keyword == "integer" { "an" } else { "a" };
                        format!("as {article} `{keyword}`, which has no range")
                    }
                };
                self.record_error_at(
                    SemanticErrorKind::PortRangeMismatch {
                        name: name.clone(),
                        ranges: format!(
                            "{} and redeclared {declared}",
                            Self::range_phrase(port_range)
                        ),
                    },
                    span,
                );
            }
        }
    }

    /// Whether any declaration of the module names `name`.
    fn declares(module: &Module, name: &SmolStr) -> bool {
        module
            .port_declarations
            .iter()
            .any(|port| port.names.contains(name))
            || module.nets.iter().any(|net| net.names.contains(name))
            || module
                .digital_nets
                .iter()
                .any(|net| net.items.iter().any(|item| item.name == *name))
            || module
                .digital_variables
                .iter()
                .any(|variable| variable.items.iter().any(|item| item.name == *name))
            || module
                .variables
                .iter()
                .any(|variable| variable.items.iter().any(|item| item.name == *name))
    }

    /// The net, `reg` or `integer` declaration that types port `name`, with
    /// the span of the name in it. A real net or variable has no range to
    /// compare and is left to the rules of its own type.
    fn port_redeclaration(&self, module: &Module, name: &SmolStr) -> Option<(Redeclared, Span)> {
        for net in module.digital_nets.iter().filter(|net| !net.kind.is_real()) {
            if let Some(item) = net.items.iter().find(|item| item.name == *name) {
                return Some((
                    Redeclared::Range(self.constant_bounds(net.range.as_ref())?),
                    item.span,
                ));
            }
        }
        for variable in &module.digital_variables {
            let Some(item) = variable.items.iter().find(|item| item.name == *name) else {
                continue;
            };
            // The parser reads `time t;` as a `reg [63:0]` whose range it
            // writes at the keyword; an authored range stands after `reg`.
            let time = variable
                .range
                .as_ref()
                .is_some_and(|range| range.span.start == variable.span.start);
            return match variable.kind {
                DigitalVariableKind::Reg if time => Some((Redeclared::Unranged("time"), item.span)),
                DigitalVariableKind::Reg => Some((
                    Redeclared::Range(self.constant_bounds(variable.range.as_ref())?),
                    item.span,
                )),
                DigitalVariableKind::Integer => Some((Redeclared::Unranged("integer"), item.span)),
                DigitalVariableKind::Real | DigitalVariableKind::Event => None,
            };
        }
        module
            .variables
            .iter()
            .filter(|variable| variable.var_type == VarType::Integer)
            .flat_map(|variable| &variable.items)
            .find(|item| item.name == *name)
            .map(|item| (Redeclared::Unranged("integer"), item.span))
    }

    /// A range's constant bounds: `Some(None)` for no range, and `None` for
    /// bounds that are not constant, which the declaration's own check
    /// reports.
    fn constant_bounds(&self, range: Option<&VectorRange>) -> Option<Option<VectorBounds>> {
        let Some(range) = range else {
            return Some(None);
        };
        let bound = |expression| match crate::canonical_ir::digital_lower::selector_constant(
            expression,
            &self.digital_selector_constants,
            self.current_time_scale,
        )? {
            crate::numeric_literal::NumericLiteralValue::Integer(value) => Some(value),
            crate::numeric_literal::NumericLiteralValue::Real(_) => None,
        };
        Some(Some(VectorBounds {
            msb: bound(&range.msb)?,
            lsb: bound(&range.lsb)?,
        }))
    }

    fn range_phrase(range: Option<VectorBounds>) -> String {
        range.map_or_else(
            || "without a range".to_string(),
            |range| format!("with range {}", range.spelling()),
        )
    }

    /// A name a discrete-domain statement or expression uses at `used` is
    /// declared before it.
    ///
    /// IEEE 1364-2005 section 4.5 declares a name implicitly only as a net,
    /// only in a port expression, a terminal list or the target of a
    /// continuous assignment, and only when "that identifier has not been
    /// declared previously". So a variable, a parameter or an explicitly
    /// declared net that the source uses above its declaration is undeclared
    /// where it is used, and a net declared implicitly is declared where that
    /// happens, not above it. Ports, which section 12.3.3 declares in the
    /// list of ports, and hierarchical names are outside the rule; a named
    /// event has its own check.
    pub(super) fn check_declared_before(
        &mut self,
        name: &SmolStr,
        used: Span,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) {
        if is_reference_symbol(name)
            || self
                .digital_scopes
                .iter()
                .any(|scope| scope.iter().any(|local| local.name == *name))
        {
            return;
        }
        let declared = match index.get(name) {
            Some(position) => {
                let signal = &signals[*position];
                if signal.redeclares_port
                    || matches!(
                        signal.class,
                        super::digital::DigitalSignalClass::Variable(DigitalVariableKind::Event)
                    )
                {
                    return;
                }
                signal.span
            }
            None => match self.symbols.lookup(name) {
                Some(symbol) if symbol.kind == SymbolKind::Parameter => symbol.span,
                _ => return,
            },
        };
        if declared.source == used.source && declared.start > used.start {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "`{name}` is used before its declaration; IEEE 1364-2005 section 4.5 \
                     declares a name implicitly only as a net, and only where no declaration of \
                     it precedes"
                )),
                used,
            );
        }
    }
}
