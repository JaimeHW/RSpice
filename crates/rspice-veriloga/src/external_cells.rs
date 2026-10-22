//! SPICE cells a Verilog-AMS design instantiates as leaves.
//!
//! A host that builds a SPICE deck hands the compiler the cells the deck
//! declares, its `.subckt` definitions and its model cards, as
//! [`ExternalCell`]s ([`crate::CompilerOptions::external_cells`]). An instance
//! of a module no HDL source declares, whose name matches one of them with
//! case ignored, is an external leaf. So are Verilog-AMS LRM 2.4 Annex E's
//! `resistor`, `capacitor` and `inductor`; the other Annex E primitives are
//! refused by name.
//!
//! A leaf is taken out of its module before analysis, so no HDL pass sees an
//! instance of a module it cannot find. Elaboration then resolves its
//! connections to the nets of every occurrence of that module
//! ([`crate::canonical_ir::HirExternalLeaf`]), and the host builds it from its
//! own netlist.

use crate::ast::{BinaryOp, Connection, Expression, Item, ModuleInstance, SourceFile, UnaryOp};
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::HashSet;

/// One SPICE cell a design may instantiate as a leaf: a deck's `.subckt`, or
/// a device its model card makes.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ExternalCell {
    /// The cell's name as the deck spells it. A design names it with case
    /// ignored, as SPICE does.
    pub name: String,
    /// The cell's ports, in order: what ordered connections bind and named
    /// connections name.
    pub ports: Vec<String>,
    /// The parameters an instance may override, by name, in the order
    /// ordered overrides bind.
    pub parameters: Vec<String>,
}

/// One leaf as the module that instantiates it declares it, with its
/// connections in the cell's port order.
#[derive(Debug, Clone)]
pub(crate) struct ExternalLeafInstance {
    /// The cell's name as the table spells it, or the primitive's.
    pub(crate) cell: SmolStr,
    pub(crate) name: SmolStr,
    /// One actual per cell port: the formal's name and what it connects.
    pub(crate) actuals: Vec<(SmolStr, Expression)>,
    /// Numeric overrides by the cell's parameter names.
    pub(crate) parameters: Vec<(SmolStr, f64)>,
}

/// Annex E's passive primitives: ports and the one parameter each takes.
const PRIMITIVES: [(&str, &str); 3] = [("resistor", "r"), ("capacitor", "c"), ("inductor", "l")];

/// The primitive's two terminals.
const PRIMITIVE_PORTS: [&str; 2] = ["p", "n"];

/// The other Verilog-AMS LRM 2.4 Annex E SPICE primitives. A design reaches a
/// diode, transistor or source through a deck model card or `.subckt`
/// instead.
const REFUSED_PRIMITIVES: &[&str] = &[
    "bjt",
    "cccs",
    "ccvs",
    "diode",
    "iexp",
    "ipulse",
    "ipwl",
    "isine",
    "jfet",
    "mesfet",
    "mosfet",
    "mutual_inductor",
    "tline",
    "vccs",
    "vcvs",
    "vexp",
    "vpulse",
    "vpwl",
    "vsine",
];

/// Take every external leaf out of the modules of `source`.
///
/// `cells` is `None` for a host that builds no leaves: nothing changes, and an
/// instance of an undeclared module is refused as before.
pub(crate) fn extract(
    source: &mut SourceFile,
    cells: Option<&[ExternalCell]>,
) -> CompileResult<()> {
    let Some(cells) = cells else {
        return Ok(());
    };
    let declared: HashSet<SmolStr> = source
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Module(module) | Item::ConnectModule(module) => Some(module.name.clone()),
            Item::ParamSet(paramset) => Some(paramset.name.clone()),
            _ => None,
        })
        .collect();
    for item in &mut source.items {
        let Item::Module(module) = item else {
            continue;
        };
        if module
            .instances
            .iter()
            .all(|instance| declared.contains(&instance.module))
        {
            continue;
        }
        for instance in std::mem::take(&mut module.instances) {
            if declared.contains(&instance.module) {
                module.instances.push(instance);
                continue;
            }
            match leaf_cell(cells, &instance)? {
                Some(cell) => module.external_leaves.push(leaf(&cell, &instance)?),
                // An undefined module, refused where it always was.
                None => module.instances.push(instance),
            }
        }
    }
    Ok(())
}

/// The cell an instance of an undeclared module names: a deck cell first,
/// then an Annex E primitive.
fn leaf_cell(
    cells: &[ExternalCell],
    instance: &ModuleInstance,
) -> CompileResult<Option<ExternalCell>> {
    let name = instance.module.as_str();
    if let Some(cell) = cells
        .iter()
        .find(|cell| cell.name.eq_ignore_ascii_case(name))
    {
        return Ok(Some(cell.clone()));
    }
    if let Some((primitive, parameter)) = PRIMITIVES
        .iter()
        .find(|(primitive, _)| primitive.eq_ignore_ascii_case(name))
    {
        return Ok(Some(ExternalCell {
            name: (*primitive).to_owned(),
            ports: PRIMITIVE_PORTS
                .iter()
                .map(|port| (*port).to_owned())
                .collect(),
            parameters: vec![(*parameter).to_owned()],
        }));
    }
    if let Some(primitive) = REFUSED_PRIMITIVES
        .iter()
        .find(|primitive| primitive.eq_ignore_ascii_case(name))
    {
        return Err(unsupported(
            format!(
                "instance `{}` names the Verilog-AMS SPICE primitive `{primitive}`; of those this \
                 compiler builds only `resistor`, `capacitor` and `inductor`, so instantiate a \
                 deck model card or `.subckt` by name instead",
                instance.name
            ),
            instance.span,
        ));
    }
    Ok(None)
}

/// The leaf `instance` makes of `cell`, its connections and overrides checked
/// against the cell's ports and parameters.
fn leaf(cell: &ExternalCell, instance: &ModuleInstance) -> CompileResult<ExternalLeafInstance> {
    let subject = format!("instance `{}` of SPICE cell `{}`", instance.name, cell.name);
    if instance.array.is_some() {
        return Err(unsupported(
            format!("{subject} is an array of instances, which a SPICE cell cannot be"),
            instance.span,
        ));
    }
    let mut actuals: Vec<Option<Expression>> = vec![None; cell.ports.len()];
    let mut ordered = 0;
    for connection in &instance.connections {
        let (index, signal, span) = match connection {
            Connection::Ordered { signal, span } => {
                let index = ordered;
                ordered += 1;
                if index >= cell.ports.len() {
                    return Err(invalid(
                        format!(
                            "{subject} has {} ordered connections, and the cell has {} ports",
                            instance.connections.len(),
                            cell.ports.len()
                        ),
                        *span,
                    ));
                }
                (index, signal, *span)
            }
            Connection::Named { port, signal, span } => {
                let index = cell
                    .ports
                    .iter()
                    .position(|formal| formal.eq_ignore_ascii_case(port))
                    .ok_or_else(|| {
                        invalid(
                            format!(
                                "{subject} connects `{port}`, which is not one of its ports ({})",
                                cell.ports.join(", ")
                            ),
                            *span,
                        )
                    })?;
                if actuals[index].is_some() {
                    return Err(invalid(
                        format!("{subject} connects port `{}` twice", cell.ports[index]),
                        *span,
                    ));
                }
                (index, signal, *span)
            }
        };
        let signal = signal.clone().ok_or_else(|| {
            invalid(
                format!(
                    "{subject} leaves port `{}` unconnected; every port of a SPICE cell must be \
                     connected",
                    cell.ports[index]
                ),
                span,
            )
        })?;
        actuals[index] = Some(signal);
    }
    let actuals = cell
        .ports
        .iter()
        .zip(actuals)
        .map(|(formal, actual)| {
            actual.map(|actual| (SmolStr::from(formal.as_str()), actual)).ok_or_else(|| {
                invalid(
                    format!(
                        "{subject} leaves port `{formal}` unconnected; it has {} ports and every \
                         one must be connected",
                        cell.ports.len()
                    ),
                    instance.span,
                )
            })
        })
        .collect::<CompileResult<Vec<_>>>()?;
    let mut parameters: Vec<(SmolStr, f64)> = Vec::with_capacity(instance.parameters.len());
    for (position, parameter) in instance.parameters.iter().enumerate() {
        let name = match &parameter.name {
            Some(name) => cell
                .parameters
                .iter()
                .find(|declared| declared.eq_ignore_ascii_case(name))
                .ok_or_else(|| {
                    invalid(
                        format!(
                            "{subject} overrides `{name}`, which is not one of its parameters{}",
                            listed(&cell.parameters)
                        ),
                        parameter.span,
                    )
                })?,
            None => cell.parameters.get(position).ok_or_else(|| {
                invalid(
                    format!(
                        "{subject} has {} ordered parameter overrides, and the cell has {} \
                         parameters",
                        instance.parameters.len(),
                        cell.parameters.len()
                    ),
                    parameter.span,
                )
            })?,
        };
        if parameters.iter().any(|(seen, _)| seen == name.as_str()) {
            return Err(invalid(
                format!("{subject} overrides parameter `{name}` twice"),
                parameter.span,
            ));
        }
        let value = numeric(&parameter.value).ok_or_else(|| {
            unsupported(
                format!(
                    "{subject} overrides `{name}` with an expression that is not a numeric \
                     constant; a SPICE cell takes only numbers from the design"
                ),
                parameter.span,
            )
        })?;
        parameters.push((name.as_str().into(), value));
    }
    Ok(ExternalLeafInstance {
        cell: cell.name.as_str().into(),
        name: instance.name.clone(),
        actuals,
        parameters,
    })
}

fn listed(names: &[String]) -> String {
    if names.is_empty() {
        "; it takes none".to_owned()
    } else {
        format!(" ({})", names.join(", "))
    }
}

/// The value of a constant arithmetic expression over numeric literals.
fn numeric(expression: &Expression) -> Option<f64> {
    let value = match expression {
        Expression::Number(number) => number.value,
        Expression::Unary(unary) => match unary.op {
            UnaryOp::Neg => -numeric(&unary.operand)?,
            UnaryOp::Pos => numeric(&unary.operand)?,
            _ => return None,
        },
        Expression::Binary(binary) => {
            let (left, right) = (numeric(&binary.left)?, numeric(&binary.right)?);
            match binary.op {
                BinaryOp::Add => left + right,
                BinaryOp::Sub => left - right,
                BinaryOp::Mul => left * right,
                BinaryOp::Div => left / right,
                _ => return None,
            }
        }
        _ => return None,
    };
    value.is_finite().then_some(value)
}

fn invalid(message: String, span: Span) -> CompileError {
    SemanticError::new(SemanticErrorKind::InvalidExpression(message), span).into()
}

pub(crate) fn unsupported(message: String, span: Span) -> CompileError {
    SemanticError::new(SemanticErrorKind::UnsupportedFeature(message), span).into()
}
