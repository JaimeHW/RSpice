//! Frequency coefficients of an already differentiated analog CFG.
//!
//! Primal values are held at the operating point. The derivative graph is a
//! polynomial in the two integration multipliers, D and I. Keeping its real
//! coefficients in the original blocks preserves branches and loop-carried
//! tangents, and lets the ordinary optimizer share their arithmetic with the
//! transient Jacobian. A frequency sweep only substitutes D = jω, I = 1/jω;
//! it neither repeats the primal body nor touches operator histories.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use super::cfg::{CfgBinaryOp, CfgFunction, CfgInstruction, CfgValue, CfgValueKind, CfgValueType};
use super::{ExprId, ValueId};

/// Freeze noise metadata and routing gains at their DC primal values, retaining
/// input validation. Existing value IDs stay valid; no transient history is read.
///
/// `carried_ddt` and `carried_idt` give each operator site a periodic solver
/// carries as an unknown of its own its index among the model's carried
/// sites. Such a site's primal reads the output the evaluation's periodic
/// frame supplies, so noise sampled over a periodic orbit sees the orbit's
/// value there, and falls back to the DC value with no frame.
pub(crate) fn freeze_noise_primal(
    function: &mut CfgFunction,
    carried_ddt: &HashMap<ExprId, usize>,
    carried_idt: &HashMap<ExprId, usize>,
) {
    if !function.values.iter().any(|value| {
        matches!(
            value.kind,
            CfgValueKind::Ddt { .. }
                | CfgValueKind::DdtDerivative { .. }
                | CfgValueKind::Idt { .. }
                | CfgValueKind::IdtMod { .. }
                | CfgValueKind::IntegralDerivative { .. }
                | CfgValueKind::IdtScale
        )
    }) {
        return;
    }
    let circular_initials = function
        .values
        .iter()
        .filter_map(|value| {
            if let CfgValueKind::IdtMod { ic, .. } = value.kind {
                Some((value.id, ic))
            } else {
                None
            }
        })
        .collect::<BTreeMap<_, _>>();
    let zero = ValueId::from(function.values.len());
    function.values.push(CfgValue {
        id: zero,
        value_type: CfgValueType::Real,
        kind: CfgValueKind::RealConstant(0.0),
    });
    function.blocks[usize::from(function.entry)]
        .instructions
        .insert(0, CfgInstruction { result: zero });
    let derivative_shapes = function
        .values
        .iter()
        .filter_map(|value| {
            matches!(value.kind, CfgValueKind::DdtDerivative { .. })
                .then(|| value.value_type.shape())
                .flatten()
        })
        .collect::<BTreeSet<_>>();
    let mut zeros = BTreeMap::new();
    for shape in derivative_shapes {
        let id = ValueId::from(function.values.len());
        function.values.push(CfgValue {
            id,
            value_type: CfgValueType::Lanes(shape),
            kind: CfgValueKind::LaneSplat(0.0),
        });
        function.blocks[usize::from(function.entry)]
            .instructions
            .insert(0, CfgInstruction { result: id });
        zeros.insert(shape, id);
    }
    let mut validation = BTreeMap::new();
    for index in 0..function.values.len() {
        let value = &function.values[index];
        if let CfgValueKind::DdtDerivative { primal, .. } = value.kind
            && let Some(shape) = value.value_type.shape()
        {
            let mut instructions = Vec::new();
            let factor = validation_factor(function, primal, &mut instructions);
            function.values[index].kind = CfgValueKind::LaneScalar {
                op: CfgBinaryOp::Mul,
                input: zeros[&shape],
                scalar: factor,
            };
            validation.insert(ValueId::from(index), instructions);
        }
    }
    for block in &mut function.blocks {
        block.instructions = block
            .instructions
            .iter()
            .flat_map(|instruction| {
                let mut before = validation.remove(&instruction.result).unwrap_or_default();
                before.push(instruction.clone());
                before
            })
            .collect();
    }
    for value in &mut function.values {
        value.kind = match value.kind {
            CfgValueKind::Ddt { operator, input } => match carried_ddt.get(&operator) {
                Some(&site) => CfgValueKind::PeriodicSiteOutput {
                    site: carried_site(site),
                    input,
                    fallback: zero,
                },
                None => CfgValueKind::Binary {
                    op: CfgBinaryOp::CheckedValue,
                    left: input,
                    right: zero,
                },
            },
            CfgValueKind::Idt {
                operator,
                input,
                ic,
            } => match carried_idt.get(&operator) {
                Some(&site) => CfgValueKind::PeriodicSiteOutput {
                    site: carried_site(site),
                    input,
                    fallback: ic,
                },
                None => CfgValueKind::Binary {
                    op: CfgBinaryOp::CheckedValue,
                    left: input,
                    right: ic,
                },
            },
            CfgValueKind::IdtMod {
                input,
                ic,
                modulus,
                offset,
                ..
            } => CfgValueKind::IdtModInitial {
                input,
                ic,
                modulus,
                offset,
            },
            CfgValueKind::IdtScale => CfgValueKind::RealConstant(0.0),
            CfgValueKind::DdtDerivative { primal, .. } => CfgValueKind::Binary {
                op: CfgBinaryOp::CheckedValue,
                left: primal,
                right: value.value_type.shape().map_or(zero, |shape| zeros[&shape]),
            },
            CfgValueKind::IntegralDerivative {
                primal,
                ic_derivative,
                wrap: None,
                ..
            } => CfgValueKind::Binary {
                op: CfgBinaryOp::CheckedValue,
                left: primal,
                right: ic_derivative,
            },
            CfgValueKind::IntegralDerivative {
                primal,
                ic_derivative,
                wrap: Some((modulus, offset, modulus_derivative)),
                ..
            } => {
                let Some(&ic) = circular_initials.get(&primal) else {
                    continue;
                };
                CfgValueKind::IdtModBranchDerivative {
                    primal,
                    ic,
                    modulus,
                    offset,
                    integral_derivative: ic_derivative,
                    modulus_derivative,
                }
            }
            _ => continue,
        };
    }
}

/// A carried site's index as the frame addresses it.
fn carried_site(site: usize) -> u32 {
    u32::try_from(site).expect("a model carries fewer than 2^32 operator sites")
}

/// Return one after evaluating a scalar dependency, suitable for multiplying
/// a packed coefficient without changing its lane shape or hiding a fault.
fn validation_factor(
    function: &mut CfgFunction,
    primal: ValueId,
    instructions: &mut Vec<CfgInstruction>,
) -> ValueId {
    let one = ValueId::from(function.values.len());
    function.values.push(CfgValue {
        id: one,
        value_type: CfgValueType::Real,
        kind: CfgValueKind::RealConstant(1.0),
    });
    instructions.push(CfgInstruction { result: one });
    let checked = ValueId::from(function.values.len());
    function.values.push(CfgValue {
        id: checked,
        value_type: CfgValueType::Real,
        kind: CfgValueKind::Binary {
            op: CfgBinaryOp::CheckedValue,
            left: primal,
            right: one,
        },
    });
    instructions.push(CfgInstruction { result: checked });
    checked
}

// Runtime evaluators only need primal freezing. Keep source-emission analysis
// out of their compilation while retaining its backend-neutral IR ownership.
#[cfg(feature = "rust-codegen")]
mod expansion;
#[cfg(feature = "rust-codegen")]
pub(crate) use expansion::{DynamicPower, FrequencyError, expand};
