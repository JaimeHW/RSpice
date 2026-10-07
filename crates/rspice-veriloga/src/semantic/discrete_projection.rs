//! Numeric analog views of selected four-state storage. Selection happens in
//! the mixed host before conversion, so unselected X/Z bits cannot poison a read.

use super::*;

#[derive(Clone, Copy)]
struct SignalShape {
    range: VectorBounds,
    unpacked: Option<VectorBounds>,
    real: bool,
}

struct Projection {
    signal: SmolStr,
    shape: SignalShape,
    lsb: i64,
    width: u32,
    value: SmolStr,
    validity: SmolStr,
}

#[derive(Default)]
pub(super) struct ProjectionBuilder {
    signals: HashMap<SmolStr, SignalShape>,
    projections: Vec<Projection>,
    indices: HashMap<(SmolStr, i64, u32), usize>,
    cells: usize,
}

impl ProjectionBuilder {
    pub(super) fn register_signals(&mut self, signals: &[AnalyzedDigitalSignal]) {
        self.signals.extend(signals.iter().map(|signal| {
            (
                signal.name.clone(),
                SignalShape {
                    range: signal.range.unwrap_or(VectorBounds::SCALAR),
                    unpacked: signal.unpacked,
                    real: signal.class.is_real(),
                },
            )
        }));
    }

    pub(super) fn is_scalar(&self, name: &str) -> bool {
        self.signals
            .get(name)
            .is_some_and(|shape| shape.unpacked.is_none())
    }
}

impl SemanticAnalyzer {
    pub(super) fn lower_packed_analog_read(
        &mut self,
        name: &SmolStr,
        word: Option<&Expression>,
        select: &PackedSelect,
        span: Span,
    ) -> CompileResult<Expression> {
        let name = self.resolve_substituted_name(name);
        let error = |detail: String| {
            CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(detail),
                span,
            ))
        };
        let Some(shape) = self.discrete_projection.signals.get(&name).copied() else {
            return Err(error(format!(
                "packed analog read of `{name}` requires discrete storage"
            )));
        };
        if shape.real || shape.unpacked.is_some() != word.is_some() {
            return Err(error(format!(
                "packed analog read of `{name}` requires a four-state scalar or a selected unpacked element"
            )));
        }
        let (high, low) = match select {
            PackedSelect::Bit(bit) => {
                let bit = self.lower_expression(bit)?;
                let Some(bit) = self.constant_array_index(&bit, &name)? else {
                    return Err(error(format!(
                        "runtime packed bit selectors in analog reads of `{name}` are not implemented"
                    )));
                };
                (bit, bit)
            }
            PackedSelect::Part { msb, lsb } => {
                let msb = self.lower_expression(msb)?;
                let lsb = self.lower_expression(lsb)?;
                let high = self
                    .eval_const_invariant_value(&msb)
                    .and_then(ConstantValue::as_exact_i64);
                let low = self
                    .eval_const_invariant_value(&lsb)
                    .and_then(ConstantValue::as_exact_i64);
                let (Some(high), Some(low)) = (high, low) else {
                    return Err(error(format!(
                        "packed analog part-select bounds of `{name}` must be constant signed 64-bit integers"
                    )));
                };
                if high != low && (high > low) != (shape.range.msb >= shape.range.lsb) {
                    return Err(error(format!(
                        "packed analog part-select of `{name}` reverses its declared range"
                    )));
                }
                (high, low)
            }
        };
        let width = high.abs_diff(low).checked_add(1)
            .filter(|width| *width <= 31)
            .ok_or_else(|| error(format!(
                "analog read of packed discrete selection `{name}` exceeds the 31-bit grouping limit"
            )))? as u32;
        // Saturation is safe here: the bounded-width group is entirely outside
        // storage when its exact least-significant offset cannot fit in i64.
        let lsb = shape.range.position_of(low);
        let word = word.map(|word| self.lower_expression(word)).transpose()?;
        let key = (name.clone(), lsb, width);
        let existing = self.discrete_projection.indices.get(&key).copied();
        let ordinal = if let Some(ordinal) = existing {
            ordinal
        } else {
            let cells = shape.unpacked.map_or(1, |range| range.width() as usize);
            let total = self
                .discrete_projection
                .cells
                .checked_add(cells)
                .filter(|total| *total <= MAX_PARAMETER_ARRAY_ELEMENTS as usize)
                .ok_or_else(|| {
                    error("packed analog projections exceed the supported storage limit".into())
                })?;
            let ordinal = self.discrete_projection.projections.len();
            // '@' is a compiler-private name separator, also used by function
            // inlining. Source declarations cannot alias a projection lane.
            let value: SmolStr = format!("__packed@{ordinal}_value").into();
            let validity: SmolStr = format!("__packed@{ordinal}_valid").into();
            for name in [&value, &validity] {
                self.define_symbol(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Variable,
                    value_type: ValueType::Integer,
                    span,
                    attrs: Default::default(),
                })?;
                if let Some(bounds) = shape.unpacked {
                    // The array's identity/extent is available during semantic
                    // lowering; final storage is allocated after body locals.
                    self.arrays.insert(
                        name.clone(),
                        AnalyzedArray {
                            base: 0,
                            lower: bounds.msb.min(bounds.lsb),
                            len: cells,
                        },
                    );
                }
            }
            // Preserve the pair if an enclosing function/control-flow lowering
            // revisits the already lowered selection expression.
            self.discrete_validity
                .insert(value.clone(), validity.clone());
            if let Some(bounds) = shape.unpacked {
                let lower = bounds.msb.min(bounds.lsb);
                for offset in 0..cells {
                    let index = lower + offset as i64;
                    self.discrete_validity.insert(
                        format!("{value}[{index}]").into(),
                        format!("{validity}[{index}]").into(),
                    );
                }
            }
            self.discrete_projection.projections.push(Projection {
                signal: name.clone(),
                shape,
                lsb,
                width,
                value,
                validity,
            });
            self.discrete_projection.cells = total;
            self.discrete_projection.indices.insert(key, ordinal);
            ordinal
        };
        // A shared projection may first be encountered in a nested block whose
        // symbol scope has since closed. Its storage remains module-owned.
        let names = {
            let projection = &self.discrete_projection.projections[ordinal];
            [projection.value.clone(), projection.validity.clone()]
        };
        for name in names {
            if self.symbols.lookup(&name).is_none() {
                self.define_symbol(Symbol {
                    name,
                    kind: SymbolKind::Variable,
                    value_type: ValueType::Integer,
                    span,
                    attrs: Default::default(),
                })?;
            }
        }
        let projection = &self.discrete_projection.projections[ordinal];
        let identifier = |name| Expression::Identifier(Identifier { name, span });
        if let Some(word) = word {
            let layout = &self.arrays[&projection.value];
            if let Some(index) = self.constant_array_index(&word, &name)? {
                self.check_array_bounds(&name, layout, index, span)?;
                Ok(Self::binary_expr(
                    BinaryOp::DiscreteValue,
                    identifier(format!("{}[{index}]", projection.validity).into()),
                    identifier(format!("{}[{index}]", projection.value).into()),
                ))
            } else {
                // The paired operation carries the selector once all the way
                // through VM, canonical SSA and native/Wasm lowering.
                Ok(Expression::ArrayAccess(ArrayAccessExpr {
                    array: projection.value.clone(),
                    discrete_validity: Some(projection.validity.clone()),
                    index: Box::new(word),
                    span,
                }))
            }
        } else {
            Ok(Self::binary_expr(
                BinaryOp::DiscreteValue,
                identifier(projection.validity.clone()),
                identifier(projection.value.clone()),
            ))
        }
    }

    pub(super) fn finish_discrete_projections(&mut self, module: &mut AnalyzedModule) {
        for projection in &self.discrete_projection.projections {
            let (lower, len) = projection.shape.unpacked.map_or((0, 1), |bounds| {
                (bounds.msb.min(bounds.lsb), bounds.width() as usize)
            });
            let value_base = module.variables.len();
            let validity_base = value_base + len;
            for (name, base) in [
                (&projection.value, value_base),
                (&projection.validity, validity_base),
            ] {
                if projection.shape.unpacked.is_some() {
                    let layout = AnalyzedArray { base, lower, len };
                    self.arrays.insert(name.clone(), layout.clone());
                    module.arrays.insert(name.clone(), layout);
                }
                for offset in 0..len {
                    let name = if projection.shape.unpacked.is_some() {
                        format!("{name}[{}]", lower + offset as i64).into()
                    } else {
                        name.clone()
                    };
                    module.variables.push(AnalyzedVariable {
                        name,
                        var_type: VarType::Integer,
                        value_type: ValueType::Integer,
                        is_state: true,
                        retains_input: false,
                        is_event_controlled: true,
                    });
                    module.event_state_variables.push(base + offset);
                }
            }
            for offset in 0..len {
                let value = value_base + offset;
                module.discrete_inputs.push((value, validity_base + offset));
                module.discrete_selections.push(AnalyzedDiscreteSelection {
                    value,
                    signal: if projection.shape.unpacked.is_some() {
                        format!("{}[{}]", projection.signal, lower + offset as i64).into()
                    } else {
                        projection.signal.clone()
                    },
                    lsb: projection.lsb,
                    width: projection.width,
                });
            }
        }
        module.event_state_variables.sort_unstable();
        module.event_state_variables.dedup();
    }
}
