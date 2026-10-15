//! Procedural continuous assignments (IEEE 1364-2005 section 9.3) as
//! override-table entries and override nodes.
//!
//! `force`, `release`, `assign` and `deassign` lower to one
//! [`CfgValueKind::DigitalOverride`] node naming a plan entry. The entry holds
//! the targets and, for `force` and `assign`, the right-hand side as its own
//! function: the right-hand side is continuous, so the host evaluates it when
//! the statement runs and again whenever an operand changes, outside any
//! activation of this process. Every process-local it reads or targets was
//! promoted to signal storage before lowering began.

use super::super::digital::{
    DigitalOverride, DigitalOverrideId, DigitalOverrideTarget, DigitalOverrideValue,
    DigitalProcessKind, DigitalWriteSelect,
};
use super::*;
use crate::ast::{DigitalProceduralContinuous, ProceduralContinuousKind};

impl ProcessLowerer<'_> {
    pub(super) fn procedural_continuous(
        &mut self,
        block: BlockId,
        statement: &DigitalProceduralContinuous,
    ) {
        let mut targets = Vec::new();
        if !self.override_targets(statement.kind, &statement.target, &mut targets) {
            return;
        }
        let real = targets
            .iter()
            .any(|target| self.real_signal(target.target.signal));
        if real && targets.len() != 1 {
            self.error(
                format!(
                    "a `{}` cannot concatenate a real target; IEEE 1364-2005 section 4.8 gives \
                     a real no bits to concatenate",
                    statement.kind.keyword()
                ),
                statement.target.span(),
            );
            return;
        }
        let value = match &statement.value {
            None => None,
            Some(rhs) => {
                if let Some(name) = impure_system_function(rhs) {
                    self.error(
                        format!(
                            "the right-hand side of a `{}` calls `{name}`; it is re-evaluated \
                             whenever an operand changes (IEEE 1364-2005 section 9.3), so it \
                             may call only pure system functions",
                            statement.kind.keyword()
                        ),
                        rhs.span(),
                    );
                    return;
                }
                let width: u32 = targets.iter().map(|target| target.width).sum();
                let Some(value) = self.override_value(rhs, width, real, statement.span) else {
                    return;
                };
                Some(value)
            }
        };
        let site = DigitalOverrideId(self.calls.override_base + self.calls.overrides.len() as u32);
        self.calls.overrides.push(DigitalOverride {
            id: site,
            kind: statement.kind,
            targets,
            value,
            span: SourceSpanRef::from(statement.span),
        });
        self.builder.push(
            block,
            CfgValueType::Effect,
            CfgValueKind::DigitalOverride { site },
        );
    }

    /// Lower a right-hand side into its own function.
    fn override_value(
        &mut self,
        rhs: &Expression,
        width: u32,
        real: bool,
        span: Span,
    ) -> Option<DigitalOverrideValue> {
        let process_builder = std::mem::replace(&mut self.builder, ProcessBuilder::new());
        let entry = self.builder.create_block();
        self.builder.seal_block(entry);
        let result = if real {
            self.real_expression(entry, rhs)
        } else {
            let value = self.assigned_value(entry, rhs, width);
            self.resize(entry, value, width, false)
        };
        self.builder.set_terminator(entry, CfgTerminator::Return);
        self.builder.seal_all_blocks();
        let evaluator_builder = std::mem::replace(&mut self.builder, process_builder);
        let (function, outputs) = match evaluator_builder.finish_with_outputs(entry, &[result]) {
            Ok(finished) => finished,
            Err(error) => {
                self.invariant(
                    format!("procedural continuous assignment function is invalid: {error}"),
                    span,
                );
                return None;
            }
        };
        let mut dependencies: Vec<DigitalSignalId> = Vec::new();
        for value in &function.values {
            match &value.kind {
                CfgValueKind::DigitalSignalRead { signal }
                | CfgValueKind::DigitalRealSignalRead { signal } => dependencies.push(*signal),
                CfgValueKind::DigitalArrayRead { array, .. } => {
                    if let Some(range) = array.cell_range() {
                        dependencies.extend(range.map(DigitalSignalId::new));
                    }
                }
                _ => {}
            }
        }
        dependencies.sort_unstable();
        dependencies.dedup();
        Some(DigitalOverrideValue {
            evaluator: CfgDigitalProcess {
                time_scale: self.time_scale,
                id: self.process.unwrap_or(DigitalProcessId::new(0)),
                kind: DigitalProcessKind::ContinuousAssign,
                function,
                static_sensitivity: None,
                span: SourceSpanRef::from(span),
            },
            result: outputs[0],
            dependencies,
        })
    }

    /// Resolve the targets of one statement, most significant first. A
    /// refused target has been reported when this returns `false`.
    fn override_targets(
        &mut self,
        kind: ProceduralContinuousKind,
        target: &DigitalLValue,
        out: &mut Vec<DigitalOverrideTarget>,
    ) -> bool {
        let variables_only = matches!(
            kind,
            ProceduralContinuousKind::Assign | ProceduralContinuousKind::Deassign
        );
        let keyword = kind.keyword();
        let (name, span) = match target {
            DigitalLValue::Concat { elements, .. } => {
                return elements
                    .iter()
                    .all(|element| self.override_targets(kind, element, out));
            }
            DigitalLValue::Identifier { name, span } => (name, *span),
            DigitalLValue::BitSelect { name, span, .. }
            | DigitalLValue::PartSelect { name, span, .. } => (name, *span),
            DigitalLValue::IndexedPartSelect(select) => (&select.name, select.span),
            DigitalLValue::ArraySelect(select) => (&select.name, select.span),
        };
        if self.digital_array(name).is_some() || matches!(target, DigitalLValue::ArraySelect(_)) {
            self.error(
                format!(
                    "a `{keyword}` names `{name}`, a memory or one of its words; IEEE 1364-2005 \
                     section 9.3 does not allow either as its target"
                ),
                span,
            );
            return false;
        }
        let Some(signal) = self.resolved_signal(name) else {
            if self.ssa_local(name).is_some() {
                self.invariant(
                    format!("the `{keyword}` target `{name}` was not given signal storage"),
                    span,
                );
            } else {
                self.error(
                    format!("a `{keyword}` names `{name}`, which is not a digital signal"),
                    span,
                );
            }
            return false;
        };
        let declared = &self.signals[usize::from(signal)];
        let variable = declared.procedurally_assignable;
        let real = declared.kind.is_real();
        let range = declared.declared_range();
        if variables_only && !variable {
            self.error(
                format!(
                    "a procedural `{keyword}` names `{name}`, a net; IEEE 1364-2005 section \
                     9.3.1 allows only a variable or a concatenation of variables, and a net is \
                     overridden with `force`"
                ),
                span,
            );
            return false;
        }
        let select = if let DigitalLValue::Identifier { .. } = target {
            DigitalWriteSelect::Whole
        } else {
            if variable || variables_only {
                self.error(
                    format!(
                        "a `{keyword}` names a bit- or part-select of the variable `{name}`; \
                         IEEE 1364-2005 section 9.3 allows a select of a net only"
                    ),
                    span,
                );
                return false;
            }
            if real {
                self.error(
                    format!("a `{keyword}` selects bits of the real `{name}`, which has none"),
                    span,
                );
                return false;
            }
            match self.write_target(target) {
                Some(resolved) => resolved.select,
                None => return false,
            }
        };
        let width = if real {
            0
        } else {
            match select.checked_width(range) {
                Some(width) => width,
                None => {
                    self.error(
                        format!("the `{keyword}` target select of `{name}` has no valid width"),
                        span,
                    );
                    return false;
                }
            }
        };
        out.push(DigitalOverrideTarget {
            target: DigitalWriteTarget { signal, select },
            width,
        });
        true
    }
}

/// The first system function a right-hand side calls that is not a pure
/// function of its arguments.
fn impure_system_function(expression: &Expression) -> Option<SmolStr> {
    let mut found = None;
    crate::ast::visit_expression(expression, &mut |expression| {
        if found.is_none()
            && let Expression::SystemFunction(function) = expression
            && super::super::digital::DigitalSystemCallKind::from_name(&function.name).is_some()
        {
            found = Some(function.name.clone());
        }
    });
    found
}
