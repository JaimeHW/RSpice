//! An analog read of a discrete value takes its left limit; an analog event
//! body reads the settled value.
//!
//! A discrete value an analog block reads changes at a digital time `T`, and
//! the engine lands an accepted timepoint on `T`. Every digital-to-analog path
//! obeys one rule there: the solution at `T` is the left limit, and the new
//! value acts from the first instant after it. So the mixed host samples every
//! discrete input at its left limit while a transient trial is open — the value
//! as it stood before the trial's digital activity — and reports the accepted
//! point as a discontinuity, so the step leaving `T` restarts integration from
//! it. `idt`, `laplace_*`, `ddt`, `slew`, `zi_*` sampling at `T`, and a
//! contribution feeding a capacitor of the netlist then each see the old value
//! up to `T` and the new one after it, which is the step the digital half took,
//! through whatever variables it flows through on the way.
//!
//! An analog event body fired at `T` is the exception: it runs because of the
//! digital activity at `T`, and it reads what that activity settled — as do a
//! `$display`, a `$strobe` or a discrete assignment inside it. Each discrete
//! read in one is rewritten to a twin: a second discrete input bound to the
//! same signal and selection, which the host samples from the settled state
//! (`HirModel::discrete_event_reads`). An indexed read's twin is a whole array
//! mirroring the one it reads. An operating point has no "before", and both
//! read the settled value.
//!
//! The analog statements a digital process's `@(cross/above/timer/absdelta)`
//! synthesizes — the guard of its occurrence counter and its `$operandK`
//! observations — read twins too. They stand for the process's own wait: an
//! `above` re-armed by a threshold its body moved must compare against the
//! moved threshold at `T`, or the next crossing would start from a level the
//! process already left.

use super::*;

/// One settled-value twin: its value slot's name, and the discrete read it
/// mirrors.
pub(super) struct EventRead {
    twin: SmolStr,
    original_validity: SmolStr,
    original: SmolStr,
    /// The twin mirrors a whole array, read through a runtime index.
    array: bool,
    span: Span,
}

impl SemanticAnalyzer {
    /// Inside an analog event body, or in an operand of a digital process's
    /// event control, rewrite every discrete read of a lowered expression to
    /// its settled-value twin. Elsewhere a read keeps its own input, which the
    /// host samples at the left limit.
    pub(super) fn read_settled_discrete_values(
        &mut self,
        expression: &mut Expression,
    ) -> CompileResult<()> {
        if !self.assignment_events.reads_settled() {
            return Ok(());
        }
        let mut pending = vec![expression];
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
                if let Some(twin) =
                    self.event_read_twin(&validity.name, &value.name, false, value.span)?
                {
                    validity.name = format!("{twin}_valid").into();
                    value.name = twin;
                }
                continue;
            }
            if let Expression::ArrayAccess(access) = &mut *expression
                && let Some(validity) = access.discrete_validity.clone()
                && let Some(twin) =
                    self.event_read_twin(&validity, &access.array, true, access.span)?
            {
                access.discrete_validity = Some(format!("{twin}_valid").into());
                access.array = twin;
            }
            flow_probes::for_child_mut(expression, &mut |child| pending.push(child));
        }
        Ok(())
    }

    /// The twin of one discrete read, allocated on first use; `None` when the
    /// read already is a twin.
    fn event_read_twin(
        &mut self,
        validity: &SmolStr,
        original: &SmolStr,
        array: bool,
        span: Span,
    ) -> CompileResult<Option<SmolStr>> {
        if self.event_reads.iter().any(|read| read.twin == *original) {
            return Ok(None);
        }
        if let Some(read) = self
            .event_reads
            .iter()
            .find(|read| read.original == *original)
        {
            return Ok(Some(read.twin.clone()));
        }
        let mut suffix = self.event_reads.len();
        let twin: SmolStr = loop {
            let name = format!("__rspice_event_read_{suffix}");
            let valid = format!("{name}_valid");
            if [&name, &valid].into_iter().all(|name| {
                self.symbols.lookup(name).is_none() && !self.arrays.contains_key(name.as_str())
            }) {
                break name.into();
            }
            suffix += 1;
        };
        let twin_validity: SmolStr = format!("{twin}_valid").into();
        let value_type = self
            .symbols
            .lookup(original)
            .map_or(ValueType::Integer, |symbol| symbol.value_type);
        for (name, value_type) in [
            (twin.clone(), value_type),
            (twin_validity.clone(), ValueType::Integer),
        ] {
            self.define_symbol(Symbol {
                name,
                kind: SymbolKind::Variable,
                value_type,
                span,
                attrs: Default::default(),
            })?;
        }
        if array {
            // The extent is known now; storage is allocated after the body's
            // locals, as a packed projection's is.
            for (name, mirrored) in [(&twin, original), (&twin_validity, validity)] {
                let Some(layout) = self.arrays.get(mirrored).cloned() else {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidExpression(format!(
                            "the discrete array `{mirrored}` read in an analog event body has \
                             no layout"
                        )),
                        span,
                    )));
                };
                self.arrays.insert(name.clone(), layout);
            }
            // A revisited element read keeps its pair, as the original's does.
            let layout = &self.arrays[&twin];
            let elements: Vec<SmolStr> = (0..layout.len)
                .map(|offset| format!("[{}]", layout.lower + offset as i64).into())
                .collect();
            for element in elements {
                self.discrete_validity.insert(
                    format!("{twin}{element}").into(),
                    format!("{twin_validity}{element}").into(),
                );
            }
        }
        self.discrete_validity
            .insert(twin.clone(), twin_validity.clone());
        self.event_reads.push(EventRead {
            twin: twin.clone(),
            original_validity: validity.clone(),
            original: original.clone(),
            array,
            span,
        });
        Ok(Some(twin))
    }

    /// Allocate each twin as discrete inputs mirroring the read it stands for:
    /// the same signal, the same selection, the same type, element by element
    /// for an array.
    pub(super) fn finish_discrete_event_reads(&mut self, module: &mut AnalyzedModule) {
        for read in std::mem::take(&mut self.event_reads) {
            let storage = if read.array {
                module
                    .arrays
                    .get(&read.original)
                    .zip(module.arrays.get(&read.original_validity))
                    .filter(|(values, validity)| values.len == validity.len)
                    .map(|(values, validity)| (values.clone(), validity.base))
            } else {
                let slot = |name: &SmolStr| {
                    module
                        .variables
                        .iter()
                        .position(|variable| variable.name == *name)
                };
                slot(&read.original)
                    .zip(slot(&read.original_validity))
                    .map(|(value, validity)| {
                        (
                            AnalyzedArray {
                                declared_dimensions: Vec::new(),
                                dimensions: Vec::new(),
                                base: value,
                                lower: 0,
                                len: 1,
                            },
                            validity,
                        )
                    })
            };
            let Some((layout, validity_base)) = storage else {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "the discrete read `{}` in an analog event body has no storage",
                        read.original
                    )),
                    read.span,
                );
                continue;
            };
            let pairs: Vec<_> = (0..layout.len)
                .map(|offset| (layout.base + offset, validity_base + offset))
                .collect();
            if let Some((value, _)) = pairs
                .iter()
                .find(|pair| !module.discrete_inputs.contains(pair))
            {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "`{}` is not a discrete input",
                        module.variables[*value].name
                    )),
                    read.span,
                );
                continue;
            }
            let twin_validity: SmolStr = format!("{}_valid", read.twin).into();
            let value_base = module.variables.len();
            let mirror_base = value_base + layout.len;
            for (twin, original, from) in [
                (&read.twin, &read.original, layout.base),
                (&twin_validity, &read.original_validity, validity_base),
            ] {
                for offset in 0..layout.len {
                    let mut mirror = module.variables[from + offset].clone();
                    mirror.name = if read.array {
                        // `m[2]` mirrors as `twin[2]`, whatever its axes.
                        let element = &mirror.name;
                        let suffix = element.strip_prefix(original.as_str()).unwrap_or("");
                        format!("{twin}{suffix}").into()
                    } else {
                        twin.clone()
                    };
                    module.variables.push(mirror);
                }
            }
            if read.array {
                for (name, base) in [(&read.twin, value_base), (&twin_validity, mirror_base)] {
                    module.arrays.insert(
                        name.clone(),
                        AnalyzedArray {
                            base,
                            ..layout.clone()
                        },
                    );
                    self.arrays.insert(
                        name.clone(),
                        AnalyzedArray {
                            base,
                            ..layout.clone()
                        },
                    );
                }
            }
            for (offset, (value, _)) in pairs.into_iter().enumerate() {
                let twin = value_base + offset;
                let twin_valid = mirror_base + offset;
                module.event_state_variables.extend([twin, twin_valid]);
                module.discrete_inputs.push((twin, twin_valid));
                let selection = module
                    .discrete_selections
                    .iter()
                    .find(|selection| selection.value == value)
                    .cloned();
                let signal = selection
                    .as_ref()
                    .map(|selection| selection.signal.clone())
                    .or_else(|| module.discrete_bindings.get(&value).cloned())
                    .unwrap_or_else(|| module.variables[value].name.clone());
                module.discrete_bindings.insert(twin, signal);
                if let Some(mut selection) = selection {
                    selection.value = twin;
                    module.discrete_selections.push(selection);
                }
                module.discrete_event_reads.push(twin);
            }
        }
        module.event_state_variables.sort_unstable();
        module.event_state_variables.dedup();
    }
}
