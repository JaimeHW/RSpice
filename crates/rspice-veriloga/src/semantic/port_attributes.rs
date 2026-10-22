//! The attributes a port carries to the engine.
//!
//! `supplySensitivity` and `groundSensitivity` are Cadence's names for the
//! nets a discrete port's converter takes its supply and its reference from:
//! `(* supplySensitivity = "vddl" *) input a;`. They are written on the port's
//! declarations, either one, and the engine binds the converter on that port
//! to the named net of the instance's scope.

use smol_str::SmolStr;

use crate::ast::{Attribute, Module};
use crate::error::{CompileError, SemanticError, SemanticErrorKind};

/// A port's supply and ground nets, as its attributes name them.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(super) struct PortSensitivity {
    pub(super) supply: Option<SmolStr>,
    pub(super) ground: Option<SmolStr>,
}

/// The sensitivity attributes written on `port`'s port and net declarations.
///
/// The value names a net, as a string or an identifier; Cadence's global
/// spelling (`cds_globals.\vdd! `) reads as the net it escapes. Two
/// declarations naming different nets for one port are refused, because the
/// engine would otherwise pick one.
pub(super) fn port_sensitivity(
    module: &Module,
    port: &str,
) -> Result<PortSensitivity, CompileError> {
    let attributes = module
        .port_declarations
        .iter()
        .filter(|declaration| declaration.names.iter().any(|name| name == port))
        .flat_map(|declaration| declaration.attributes.iter())
        .chain(
            module
                .nets
                .iter()
                .filter(|net| net.names.iter().any(|name| name == port))
                .flat_map(|net| net.attributes.iter()),
        );
    let mut sensitivity = PortSensitivity::default();
    for attribute in attributes {
        let slot = match attribute.name.as_str() {
            "supplySensitivity" => &mut sensitivity.supply,
            "groundSensitivity" => &mut sensitivity.ground,
            _ => continue,
        };
        let net = named_net(attribute, port)?;
        match slot {
            Some(first) if *first != net => {
                return Err(refusal(
                    format!(
                        "port '{port}' has two '{}' attributes, naming nets '{first}' and \
                         '{net}'; give the port one supply net",
                        attribute.name
                    ),
                    attribute,
                ));
            }
            _ => *slot = Some(net),
        }
    }
    Ok(sensitivity)
}

fn named_net(attribute: &Attribute, port: &str) -> Result<SmolStr, CompileError> {
    let net = attribute
        .text_value()
        .map(crate::connect::inherited_net_name)
        .filter(|net| !net.is_empty());
    net.ok_or_else(|| {
        refusal(
            format!(
                "attribute '{}' on port '{port}' must name a net, as a string or an identifier",
                attribute.name
            ),
            attribute,
        )
    })
}

fn refusal(detail: String, attribute: &Attribute) -> CompileError {
    CompileError::Semantic(SemanticError::new(
        SemanticErrorKind::InvalidExpression(detail),
        attribute.span,
    ))
}
