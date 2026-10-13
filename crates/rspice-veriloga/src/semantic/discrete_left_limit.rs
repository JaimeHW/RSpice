//! `slew` of a discrete value reads the value's left limit.
//!
//! A discrete value an analog block reads changes at a digital time `T`, and
//! the engine lands an accepted timepoint on `T`. Evaluated with the new
//! value, `slew` limits the change over the whole step that ends at `T`, so
//! its ramp starts at the previous timepoint — wherever the step controller
//! happened to put it — instead of at the edge. Every digital-to-analog path
//! obeys one rule: a transition starts at its instant, and the solution at
//! that instant is the left limit. So `slew`'s operand reads each discrete
//! value as it stood when the coupled trial opened: at `T` the operand is
//! still the old value and the output holds, and from the next timepoint on
//! it ramps from `T` at the limited rate, reaching the new value exactly
//! `|step| / rate` after the edge.
//!
//! The left limit is a second discrete input bound to the same signal and
//! selection, which the mixed host samples from the trial's pre-trial image
//! (`HirModel::discrete_left_limits`). An operating point has no "before" and
//! reads the settled value. An indexed array read has no fixed slot to mirror
//! and is refused inside `slew`.

use super::*;

/// One left limit: its value slot's name, and the discrete read it mirrors.
pub(super) struct LeftLimit {
    twin: SmolStr,
    original_validity: SmolStr,
    original: SmolStr,
    span: Span,
}

impl SemanticAnalyzer {
    /// Rewrite every discrete read in a lowered `slew` operand to its left
    /// limit.
    pub(super) fn read_discrete_left_limits(
        &mut self,
        operand: &mut Expression,
    ) -> CompileResult<()> {
        let mut reads = Vec::new();
        let mut indexed = None;
        let mut pending = vec![&mut *operand];
        while let Some(expression) = pending.pop() {
            let pair = matches!(
                &*expression,
                Expression::Binary(binary)
                    if binary.op == BinaryOp::DiscreteValue
                        && matches!(&*binary.left, Expression::Identifier(_))
                        && matches!(&*binary.right, Expression::Identifier(_))
            );
            if pair {
                let Expression::Binary(BinaryExpr { left, right, .. }) = expression else {
                    unreachable!("matched above");
                };
                let (Expression::Identifier(validity), Expression::Identifier(value)) =
                    (&mut **left, &mut **right)
                else {
                    unreachable!("matched above");
                };
                reads.push((validity, value));
                continue;
            }
            if let Expression::ArrayAccess(access) = &*expression
                && access.discrete_validity.is_some()
            {
                indexed.get_or_insert((access.array.clone(), access.span));
                continue;
            }
            flow_probes::for_child_mut(expression, &mut |child| pending.push(child));
        }
        if let Some((array, span)) = indexed {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "`slew` of the indexed discrete read `{array}[...]`: `slew` reads a discrete \
                     value as it stood before the digital time it changed at, and an indexed \
                     read has no fixed element to hold; read the element into a variable first"
                )),
                span,
            )));
        }
        for (validity, value) in reads {
            if self
                .left_limits
                .iter()
                .any(|limit| limit.twin == value.name)
            {
                continue;
            }
            let existing = self
                .left_limits
                .iter()
                .find(|limit| limit.original == value.name)
                .map(|limit| limit.twin.clone());
            let twin = match existing {
                Some(twin) => twin,
                None => {
                    let mut suffix = self.left_limits.len();
                    let twin: SmolStr = loop {
                        let name = format!("__rspice_left_limit_{suffix}");
                        if self.symbols.lookup(&name).is_none()
                            && self.symbols.lookup(&format!("{name}_valid")).is_none()
                        {
                            break name.into();
                        }
                        suffix += 1;
                    };
                    self.left_limits.push(LeftLimit {
                        twin: twin.clone(),
                        original_validity: validity.name.clone(),
                        original: value.name.clone(),
                        span: value.span,
                    });
                    twin
                }
            };
            let value_type = self
                .symbols
                .lookup(&value.name)
                .map_or(ValueType::Integer, |symbol| symbol.value_type);
            let twin_validity: SmolStr = format!("{twin}_valid").into();
            for (name, value_type) in [
                (twin.clone(), value_type),
                (twin_validity.clone(), ValueType::Integer),
            ] {
                if self.symbols.lookup(&name).is_none() {
                    self.define_symbol(Symbol {
                        name,
                        kind: SymbolKind::Variable,
                        value_type,
                        span: value.span,
                        attrs: Default::default(),
                    })?;
                }
            }
            value.name = twin;
            validity.name = twin_validity;
        }
        Ok(())
    }

    /// Allocate each left limit as a discrete input mirroring the read it
    /// stands for: the same signal, the same selection, the same type.
    pub(super) fn finish_discrete_left_limits(&mut self, module: &mut AnalyzedModule) {
        for limit in std::mem::take(&mut self.left_limits) {
            let slot = |name: &SmolStr| {
                module
                    .variables
                    .iter()
                    .position(|variable| variable.name == *name)
            };
            let (Some(value), Some(validity)) =
                (slot(&limit.original), slot(&limit.original_validity))
            else {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "the left limit of discrete read `{}` has no storage",
                        limit.original
                    )),
                    limit.span,
                );
                continue;
            };
            if !module.discrete_inputs.contains(&(value, validity)) {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "`{}` is not a discrete input",
                        limit.original
                    )),
                    limit.span,
                );
                continue;
            }
            let base = module.variables.len();
            let mut mirror = module.variables[value].clone();
            mirror.name = limit.twin.clone();
            module.variables.push(mirror);
            let mut lane = module.variables[validity].clone();
            lane.name = format!("{}_valid", limit.twin).into();
            module.variables.push(lane);
            module.event_state_variables.extend([base, base + 1]);
            module.discrete_inputs.push((base, base + 1));
            let selection = module
                .discrete_selections
                .iter()
                .find(|selection| selection.value == value)
                .cloned();
            let signal = selection
                .as_ref()
                .map(|selection| selection.signal.clone())
                .or_else(|| module.discrete_bindings.get(&value).cloned())
                .unwrap_or_else(|| limit.original.clone());
            module.discrete_bindings.insert(base, signal);
            if let Some(mut selection) = selection {
                selection.value = base;
                module.discrete_selections.push(selection);
            }
            module.discrete_left_limits.push(base);
        }
        module.event_state_variables.sort_unstable();
        module.event_state_variables.dedup();
    }
}
