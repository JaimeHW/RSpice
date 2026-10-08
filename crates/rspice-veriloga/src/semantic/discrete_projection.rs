//! Numeric analog views of selected four-state storage. Selection happens in
//! the mixed host before conversion, so unselected X/Z bits cannot poison a read.

use super::*;
use crate::array_index::{PACKED_CHUNK_BITS, PackedArrayLayout};

#[derive(Clone, Copy)]
struct SignalShape {
    range: VectorBounds,
    /// Distinguish a declared one-bit vector from an un-ranged scalar.
    selectable: bool,
    unpacked: Option<VectorBounds>,
    unpacked_rank: usize,
    real: bool,
}

struct Projection {
    signal: SmolStr,
    shape: SignalShape,
    lsb: i64,
    width: u32,
    value: SmolStr,
    validity: SmolStr,
    packed: Option<PackedArrayLayout>,
}

#[derive(Default)]
pub(super) struct ProjectionBuilder {
    signals: HashMap<SmolStr, SignalShape>,
    projections: Vec<Projection>,
    indices: HashMap<(SmolStr, i64, u32, bool), usize>,
    cells: usize,
}

impl ProjectionBuilder {
    pub(super) fn register_signals(&mut self, signals: &[AnalyzedDigitalSignal]) {
        self.signals.extend(signals.iter().map(|signal| {
            (
                signal.name.clone(),
                SignalShape {
                    range: signal.range.unwrap_or(VectorBounds::SCALAR),
                    selectable: signal.range.is_some(),
                    unpacked: signal.unpacked,
                    unpacked_rank: signal.dimensions.len(),
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
        if shape.unpacked_rank > 1 {
            return Err(error(format!(
                "analog read of multidimensional array '{name}' requires continuous coordinate lowering"
            )));
        }
        if shape.real || shape.unpacked.is_some() != word.is_some() {
            return Err(error(format!(
                "packed analog read of `{name}` requires a four-state scalar or a selected unpacked element"
            )));
        }
        if !shape.selectable {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidExpression(format!(
                    "packed analog read of `{name}` requires a vector or integer; scalar storage has no selectable bits"
                )),
                span,
            )));
        }
        let word = word
            .map(|word| self.lower_packed_selector(word))
            .transpose()?;
        let (high, low) = match select {
            PackedSelect::Bit(bit) => {
                let bit = self.lower_packed_selector(bit)?;
                let Some(bit) = self.constant_array_index(&bit, &name)? else {
                    return self.lower_dynamic_packed_read(&name, shape, word, bit, span);
                };
                (bit, bit)
            }
            PackedSelect::Part { msb, lsb } => {
                let msb = self.lower_packed_selector(msb)?;
                let lsb = self.lower_packed_selector(lsb)?;
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
        let ordinal = self.register_packed_projection(&name, shape, lsb, width, None, span)?;
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
                    packed: None,
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

    /// Keep exact known based literals until the analog integer folder consumes
    /// them. Only literals change representation: parameter defaults must never
    /// become invariant indices, and analog operators keep analog typing.
    fn lower_packed_selector(&mut self, expression: &Expression) -> CompileResult<Expression> {
        let mut expression = expression.clone();
        let mut pending = vec![&mut expression];
        while let Some(expression) = pending.pop() {
            if let Expression::Digital(DigitalExpr::FourState(literal)) = expression {
                let Some(value) = parse_integer_literal(&literal.value.raw).ok().flatten() else {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(
                            "packed analog selector literals must be known signed 64-bit integers"
                                .into(),
                        ),
                        literal.span,
                    )));
                };
                *expression = Expression::Number(NumberLit {
                    value: value as f64,
                    raw: literal.value.raw.clone(),
                    span: literal.span,
                });
            } else {
                flow_probes::for_child_mut(expression, &mut |child| pending.push(child));
            }
        }
        self.lower_expression(&expression)
    }

    fn lower_dynamic_packed_read(
        &mut self,
        name: &SmolStr,
        shape: SignalShape,
        word: Option<Expression>,
        bit: Expression,
        span: Span,
    ) -> CompileResult<Expression> {
        let bounds = shape.unpacked.unwrap_or(VectorBounds::SCALAR);
        let layout = PackedArrayLayout {
            word_lower: bounds.msb.min(bounds.lsb),
            word_len: bounds.width(),
            packed_msb: shape.range.msb,
            packed_lsb: shape.range.lsb,
        };
        let word = word.unwrap_or_else(|| Self::number_expr(0.0, span));
        let ordinal = self.register_packed_projection(
            name,
            shape,
            0,
            shape.range.width(),
            Some(layout),
            span,
        )?;
        let projection = &self.discrete_projection.projections[ordinal];
        Ok(Expression::ArrayAccess(ArrayAccessExpr {
            packed: Some(PackedArrayIndex {
                bit: Box::new(bit),
                layout,
            }),
            array: projection.value.clone(),
            discrete_validity: Some(projection.validity.clone()),
            index: Box::new(word),
            span,
        }))
    }

    fn register_packed_projection(
        &mut self,
        name: &SmolStr,
        shape: SignalShape,
        lsb: i64,
        width: u32,
        packed: Option<PackedArrayLayout>,
        span: Span,
    ) -> CompileResult<usize> {
        let error = |detail: String| {
            CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(detail),
                span,
            ))
        };
        let key = (name.clone(), lsb, width, packed.is_some());
        let existing = self.discrete_projection.indices.get(&key).copied();
        let ordinal = if let Some(ordinal) = existing {
            ordinal
        } else {
            let cells = if let Some(layout) = packed {
                layout
                    .chunk_len()
                    .ok_or_else(|| error("invalid packed projection shape".into()))?
            } else {
                shape.unpacked.map_or(1, |range| range.width() as usize)
            };
            let array_lower = if packed.is_some() {
                Some(0)
            } else {
                shape.unpacked.map(|bounds| bounds.msb.min(bounds.lsb))
            };
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
                if let Some(lower) = array_lower {
                    // The array's identity/extent is available during semantic
                    // lowering; final storage is allocated after body locals.
                    self.arrays.insert(
                        name.clone(),
                        AnalyzedArray {
                            dimensions: Vec::new(),
                            base: 0,
                            lower,
                            len: cells,
                        },
                    );
                }
            }
            // Preserve the pair if an enclosing function/control-flow lowering
            // revisits the already lowered selection expression.
            self.discrete_validity
                .insert(value.clone(), validity.clone());
            if let Some(lower) = array_lower {
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
                packed,
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
        Ok(ordinal)
    }

    pub(super) fn finish_discrete_projections(&mut self, module: &mut AnalyzedModule) {
        for projection in &self.discrete_projection.projections {
            let is_array = projection.packed.is_some() || projection.shape.unpacked.is_some();
            let (lower, len) = if let Some(layout) = projection.packed {
                (0, layout.chunk_len().expect("validated packed layout"))
            } else {
                projection.shape.unpacked.map_or((0, 1), |bounds| {
                    (bounds.msb.min(bounds.lsb), bounds.width() as usize)
                })
            };
            let value_base = module.variables.len();
            let validity_base = value_base + len;
            for (name, base) in [
                (&projection.value, value_base),
                (&projection.validity, validity_base),
            ] {
                if is_array {
                    let layout = AnalyzedArray {
                        dimensions: Vec::new(),
                        base,
                        lower,
                        len,
                    };
                    self.arrays.insert(name.clone(), layout.clone());
                    module.arrays.insert(name.clone(), layout);
                }
                for offset in 0..len {
                    let name = if is_array {
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
                let (source_index, lsb, width) = if let Some(layout) = projection.packed {
                    let chunks = layout.chunks_per_word().unwrap() as usize;
                    let lsb = (offset % chunks) as u32 * PACKED_CHUNK_BITS;
                    (
                        layout.word_lower + (offset / chunks) as i64,
                        i64::from(lsb),
                        (projection.width - lsb).min(PACKED_CHUNK_BITS),
                    )
                } else {
                    (lower + offset as i64, projection.lsb, projection.width)
                };
                module.discrete_selections.push(AnalyzedDiscreteSelection {
                    encoded: projection.packed.is_some(),
                    value,
                    signal: if projection.shape.unpacked.is_some() {
                        format!("{}[{source_index}]", projection.signal).into()
                    } else {
                        projection.signal.clone()
                    },
                    lsb,
                    width,
                });
            }
        }
        module.event_state_variables.sort_unstable();
        module.event_state_variables.dedup();
    }
}
