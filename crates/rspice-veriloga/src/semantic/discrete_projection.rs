//! Numeric analog views of selected four-state storage. Selection happens in
//! the mixed host before conversion, so unselected X/Z bits cannot poison a read.

use super::*;
use crate::array_index::{PACKED_CHUNK_BITS, PackedArrayLayout};
use crate::canonical_ir::digital_value::{DigitalCaseMatch, FourStateValue, case_match};
use crate::four_state::FourStateBit;

#[derive(Clone, Copy)]
struct SignalShape {
    range: VectorBounds,
    /// Distinguish a declared one-bit vector from an un-ranged scalar.
    selectable: bool,
    unpacked: Option<VectorBounds>,
    unpacked_rank: usize,
    real: bool,
    signed: bool,
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

impl Projection {
    /// A numeric group of 32 bits or more reads as a real holding its integer
    /// value; a narrower one, and every encoded chunk, as an integer.
    fn value_type(width: u32, packed: Option<PackedArrayLayout>) -> ValueType {
        if packed.is_none() && width > 31 {
            ValueType::Real
        } else {
            ValueType::Integer
        }
    }
}

#[derive(Default)]
pub(super) struct ProjectionBuilder {
    signals: HashMap<SmolStr, SignalShape>,
    dimensions: HashMap<SmolStr, Vec<(i64, i64)>>,
    projections: Vec<Projection>,
    indices: HashMap<(SmolStr, i64, u32, bool), usize>,
    cells: usize,
}

impl ProjectionBuilder {
    pub(super) fn register_signals(&mut self, signals: &[AnalyzedDigitalSignal]) {
        self.dimensions.extend(
            signals
                .iter()
                .filter(|signal| signal.dimensions.len() > 1)
                .map(|signal| {
                    (
                        signal.name.clone(),
                        signal
                            .dimensions
                            .iter()
                            .map(|axis| (axis.msb, axis.lsb))
                            .collect(),
                    )
                }),
        );
        self.signals.extend(signals.iter().map(|signal| {
            (
                signal.name.clone(),
                SignalShape {
                    range: signal.range.unwrap_or(VectorBounds::SCALAR),
                    selectable: signal.range.is_some(),
                    unpacked: signal.unpacked,
                    unpacked_rank: signal.dimensions.len(),
                    real: signal.class.is_real(),
                    signed: signal.signedness.is_signed(),
                },
            )
        }));
    }

    pub(super) fn dimensions(&self, name: &str) -> Option<&[(i64, i64)]> {
        self.dimensions.get(name).map(Vec::as_slice)
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
        self.lower_packed_analog_read_inner(name, word, select, span, false)
    }

    pub(super) fn lower_normalized_packed_analog_read(
        &mut self,
        name: &SmolStr,
        word: Option<&Expression>,
        select: &PackedSelect,
        span: Span,
    ) -> CompileResult<Expression> {
        self.lower_packed_analog_read_inner(name, word, select, span, true)
    }

    fn lower_packed_analog_read_inner(
        &mut self,
        name: &SmolStr,
        word: Option<&Expression>,
        select: &PackedSelect,
        span: Span,
        normalized: bool,
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
        if shape.unpacked_rank > 1 && !normalized {
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
        let width = high
            .abs_diff(low)
            .checked_add(1)
            .and_then(|width| u32::try_from(width).ok())
            .ok_or_else(|| {
                error(format!(
                    "analog read of packed discrete selection `{name}` is wider than a bit \
                     grouping can be"
                ))
            })?;
        // A selection of 32 bits or more reads as a real holding its unsigned
        // integer value, as a whole grouping that wide does (see
        // `bind_discrete_analog_reads`).
        if width > 31 {
            self.warn(
                super::digital::WIDE_DISCRETE_READ_CODE,
                format!(
                    "analog read of the {width}-bit part-select of packed discrete signal \
                     `{name}` goes beyond Verilog-AMS 2023 Table 7-1, which limits a bit \
                     grouping read from the analog context to 31 bits; RSpice reads it as a \
                     real holding the selection's unsigned integer value, exact below 2^53"
                ),
                span,
            );
        }
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
                    normalized: false,
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
    pub(super) fn lower_packed_selector(
        &mut self,
        expression: &Expression,
    ) -> CompileResult<Expression> {
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
            normalized: false,
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
        let value_type = Projection::value_type(width, packed);
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
            for (name, value_type) in [(&value, value_type), (&validity, ValueType::Integer)] {
                self.define_symbol(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Variable,
                    value_type,
                    span,
                    attrs: Default::default(),
                })?;
                if let Some(lower) = array_lower {
                    // The array's identity/extent is available during semantic
                    // lowering; final storage is allocated after body locals.
                    self.arrays.insert(
                        name.clone(),
                        AnalyzedArray {
                            declared_dimensions: Vec::new(),
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
            [
                (projection.value.clone(), value_type),
                (projection.validity.clone(), ValueType::Integer),
            ]
        };
        for (name, value_type) in names {
            if self.symbols.lookup(&name).is_none() {
                self.define_symbol(Symbol {
                    name,
                    kind: SymbolKind::Variable,
                    value_type,
                    span,
                    attrs: Default::default(),
                })?;
            }
        }
        Ok(ordinal)
    }

    pub(super) fn finish_discrete_projections(&mut self, module: &mut AnalyzedModule) {
        for projection in &self.discrete_projection.projections {
            let source_shape =
                self.discrete_projection
                    .dimensions(&projection.signal)
                    .map(|axes| {
                        crate::array_index::UnpackedArrayLayout::new(axes, Self::MAX_ARRAY_ELEMENTS)
                            .expect("projection source shape")
                    });
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
            let value_type = Projection::value_type(projection.width, projection.packed);
            for (name, base, value_type) in [
                (&projection.value, value_base, value_type),
                (&projection.validity, validity_base, ValueType::Integer),
            ] {
                if is_array {
                    let layout = AnalyzedArray {
                        declared_dimensions: Vec::new(),
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
                        var_type: if value_type == ValueType::Real {
                            VarType::Real
                        } else {
                            VarType::Integer
                        },
                        value_type,
                        is_state: true,
                        retains_input: false,
                        is_event_controlled: true,
                        output: None,
                    });
                    module.event_state_variables.push(base + offset);
                }
            }
            for offset in 0..len {
                let value = value_base + offset;
                module.discrete_inputs.push((value, validity_base + offset));
                let (source_index, lsb, width) = if let Some(layout) = projection.packed {
                    // Chunks of the selection, from its least significant
                    // bit: the whole word for a runtime-indexed read, a case
                    // comparison's own selection for one.
                    let chunks = layout.chunks_per_word().unwrap() as usize;
                    let lsb = (offset % chunks) as u32 * PACKED_CHUNK_BITS;
                    (
                        layout.word_lower + (offset / chunks) as i64,
                        projection.lsb + i64::from(lsb),
                        (projection.width - lsb).min(PACKED_CHUNK_BITS),
                    )
                } else {
                    (lower + offset as i64, projection.lsb, projection.width)
                };
                module.discrete_selections.push(AnalyzedDiscreteSelection {
                    encoded: projection.packed.is_some(),
                    value,
                    signal: if let Some(shape) = &source_shape {
                        let mut name = projection.signal.to_string();
                        for index in shape
                            .indices(source_index as usize)
                            .expect("projection source element")
                        {
                            name.push_str(&format!("[{index}]"));
                        }
                        name.into()
                    } else if projection.shape.unpacked.is_some() {
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

// `===`, `!==` and an exact `case` in analog code: Verilog-AMS LRM 2.4
// section 7.3.2's way to read an `x` or `z` bit of a discrete net there.
//
// The continuous domain has no `x` or `z` value, and the section makes it an
// error for a discrete operand to return one when an analog expression reads
// it. The case equality and case inequality operators and the `case`
// statement are the exception it names: they compare the four-state bits
// themselves, and their result is a known 0 or 1 whatever the bits hold.
//
// An operand is a constant, which the compiler holds exactly, or a selection
// of discrete storage, which the mixed host samples into the analog block as
// encoded chunks of up to `PACKED_CHUNK_BITS` bits: each bit's IEEE 1364
// `aval` (1 for `1` and `x`) in the low half and its known flag (set for `0`
// and `1`) above it, so each of the four states is a distinct word and no
// chunk is ever unavailable. A comparison is a conjunction of exact integer
// equalities, chunk by chunk, which every backend executes.

/// One operand of a case comparison, as the bits it compares.
enum CaseOperand {
    /// A constant, with whether it is signed (IEEE 1364-2005 section 5.4.2).
    Constant { value: FourStateValue, signed: bool },
    /// A selection of discrete storage, read through projection `ordinal`.
    Discrete {
        ordinal: usize,
        width: u32,
        signed: bool,
    },
    /// An analog integer expression: 32 known bits, signed. A discrete value
    /// it reads numerically is held to section 7.3.2's rule where it reads it.
    Integer(Expression),
}

impl CaseOperand {
    const fn signed(&self) -> bool {
        match self {
            Self::Constant { signed, .. } | Self::Discrete { signed, .. } => *signed,
            Self::Integer(_) => true,
        }
    }
}

/// Why a real operand is refused.
const REAL_OPERAND: &str = "this operand is real, and Verilog-AMS LRM 2.4 section 4.2.1 makes `===` and `!==` illegal \
     on a real";

/// The width of an analog integer (Verilog-AMS LRM 2.4 section 3.2).
const INTEGER_BITS: u32 = 32;

/// Chunk `chunk` of `value`'s low `width` bits, encoded as the mixed host
/// encodes a sampled chunk.
fn encoded_chunk(value: &FourStateValue, width: u32, chunk: u32) -> i64 {
    let first = chunk * PACKED_CHUNK_BITS;
    let (mut aval, mut known) = (0i64, 0i64);
    for bit in 0..chunk_bits(width, chunk) {
        match value.bit(first + bit) {
            FourStateBit::Zero => known |= 1 << bit,
            FourStateBit::One => {
                aval |= 1 << bit;
                known |= 1 << bit;
            }
            FourStateBit::Unknown => aval |= 1 << bit,
            FourStateBit::HighImpedance => {}
        }
    }
    aval | known << PACKED_CHUNK_BITS
}

/// How many of a `width`-bit selection's bits chunk `chunk` holds.
fn chunk_bits(width: u32, chunk: u32) -> u32 {
    (width - chunk * PACKED_CHUNK_BITS).min(PACKED_CHUNK_BITS)
}

/// The encoding of known zeros at chunk-relative positions `from..to`.
fn known_zeros(from: u32, to: u32) -> i64 {
    (((1i64 << to) - 1) & !((1i64 << from) - 1)) << PACKED_CHUNK_BITS
}

/// The encoding of a chunk of `bits` known bits, all zero.
fn all_known(bits: u32) -> i64 {
    known_zeros(0, bits)
}

fn integer(value: i64, span: Span) -> Expression {
    Expression::Number(NumberLit {
        value: value as f64,
        raw: value.to_string().into(),
        span,
    })
}

impl SemanticAnalyzer {
    /// `left === right`, or `left !== right` when `negate`, in analog code.
    pub(super) fn lower_analog_case_equality(
        &mut self,
        equality: &CaseEqualityExpr,
    ) -> CompileResult<Expression> {
        let operator = if equality.negate { "!==" } else { "===" };
        let matched =
            self.lower_case_match(&equality.left, &equality.right, operator, equality.span)?;
        Ok(if equality.negate {
            Self::not_expr(matched)
        } else {
            matched
        })
    }

    /// Whether an analog `case` compares its labels as case equality: its
    /// selector reads four-state discrete storage, whose `x` and `z` bits a
    /// label may name, and every label is a constant or such a read; or a
    /// label is a constant holding `x` or `z`. Any other `case` compares by
    /// value, as it always has.
    pub(super) fn analog_case_compares_bits(&self, case: &CaseStmt) -> bool {
        let mut labels = case.items.iter().flat_map(|item| &item.matches);
        if self.four_state_case_operand(&case.expr) {
            labels.all(|label| self.case_label_compares_bits(label))
        } else {
            labels.any(|label| {
                matches!(label, Expression::Digital(DigitalExpr::FourState(literal))
                    if literal.value.has_unknown_bits())
            })
        }
    }

    /// Whether `expression` reads four-state discrete storage that is not an
    /// array.
    fn four_state_case_operand(&self, expression: &Expression) -> bool {
        self.discrete_case_operand_name(expression)
            .is_some_and(|name| {
                let shape = self.discrete_projection.signals[&name];
                !shape.real && shape.unpacked.is_none()
            })
    }

    /// Whether a label of a bit-comparing `case` is an operand
    /// [`Self::case_operand`] takes: a four-state read, an integral literal,
    /// or an integer parameter.
    fn case_label_compares_bits(&self, label: &Expression) -> bool {
        match label {
            Expression::Digital(DigitalExpr::FourState(_)) => true,
            Expression::Number(number) => {
                number.raw.contains('\'') || Self::integer_literal_value(number).is_some()
            }
            Expression::Unary(unary) if matches!(unary.op, UnaryOp::Neg | UnaryOp::Pos) => {
                self.case_label_compares_bits(&unary.operand)
            }
            Expression::Identifier(identifier) => {
                self.four_state_case_operand(label)
                    || self.symbols.lookup(&identifier.name).is_some_and(|symbol| {
                        symbol.kind == SymbolKind::Parameter
                            && symbol.value_type == ValueType::Integer
                    })
            }
            _ => self.four_state_case_operand(label),
        }
    }

    /// One `===` of an analog `case` arm or operator, 1 when the operands'
    /// bits are identical at the wider width and 0 otherwise.
    pub(super) fn lower_case_match(
        &mut self,
        left: &Expression,
        right: &Expression,
        construct: &str,
        span: Span,
    ) -> CompileResult<Expression> {
        let left = self.case_operand(left, construct)?;
        let right = self.case_operand(right, construct)?;
        let signed = left.signed() && right.signed();
        let conjunction = |terms: Vec<Expression>| {
            terms
                .into_iter()
                .reduce(|all, term| Self::binary_expr(BinaryOp::And, all, term))
                .unwrap_or_else(|| integer(1, span))
        };
        let mut matched = match (left, right) {
            (
                CaseOperand::Constant { value: left, .. },
                CaseOperand::Constant { value: right, .. },
            ) => {
                let matched = case_match(DigitalCaseMatch::Exact, &left, &right, signed);
                integer(i64::from(matched.bit(0) == FourStateBit::One), span)
            }
            (CaseOperand::Discrete { ordinal, width, .. }, CaseOperand::Constant { value, .. })
            | (CaseOperand::Constant { value, .. }, CaseOperand::Discrete { ordinal, width, .. }) =>
            {
                // Above the selection's own bits its extension is known: zeros,
                // or its sign bit, which the constant's bit `width - 1` must
                // equal. A constant whose high bits differ never matches.
                let fill = if signed {
                    value.bit(width - 1)
                } else {
                    FourStateBit::Zero
                };
                if (width..value.width()).any(|bit| value.bit(bit) != fill) {
                    integer(0, span)
                } else {
                    let value = value.extended(width, signed);
                    conjunction(
                        (0..width.div_ceil(PACKED_CHUNK_BITS))
                            .map(|chunk| {
                                Self::binary_expr(
                                    BinaryOp::Eq,
                                    self.encoded_chunk_read(ordinal, chunk, span),
                                    integer(encoded_chunk(&value, width, chunk), span),
                                )
                            })
                            .collect(),
                    )
                }
            }
            (
                CaseOperand::Discrete {
                    ordinal: left,
                    width: left_width,
                    ..
                },
                CaseOperand::Discrete {
                    ordinal: right,
                    width: right_width,
                    ..
                },
            ) => {
                if signed && left_width != right_width {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "`{construct}` in analog code between two signed discrete operands \
                             of different widths ({left_width} and {right_width} bits) would \
                             sign-extend the narrower one by its own `x` or `z` bit; compare \
                             operands of one width"
                        )),
                        span,
                    )));
                }
                let ((narrow, narrow_width), (wide, wide_width)) = if left_width <= right_width {
                    ((left, left_width), (right, right_width))
                } else {
                    ((right, right_width), (left, left_width))
                };
                // The narrower selection zero-extends: its last chunk gains
                // known zeros up to the wider one's width, and every chunk it
                // does not reach is all known zeros.
                conjunction(
                    (0..wide_width.div_ceil(PACKED_CHUNK_BITS))
                        .map(|chunk| {
                            let wide_read = self.encoded_chunk_read(wide, chunk, span);
                            let held = chunk_bits(wide_width, chunk);
                            let narrow_read = if chunk * PACKED_CHUNK_BITS < narrow_width {
                                let own = chunk_bits(narrow_width, chunk);
                                let read = self.encoded_chunk_read(narrow, chunk, span);
                                if own < held {
                                    Self::binary_expr(
                                        BinaryOp::Add,
                                        read,
                                        integer(known_zeros(own, held), span),
                                    )
                                } else {
                                    read
                                }
                            } else {
                                integer(known_zeros(0, held), span)
                            };
                            Self::binary_expr(BinaryOp::Eq, narrow_read, wide_read)
                        })
                        .collect(),
                )
            }
            (CaseOperand::Integer(left), CaseOperand::Integer(right)) => {
                Self::binary_expr(BinaryOp::Eq, left, right)
            }
            (CaseOperand::Integer(known), CaseOperand::Constant { value, .. })
            | (CaseOperand::Constant { value, .. }, CaseOperand::Integer(known)) => {
                // The integer's 32 bits, extended as the comparison extends
                // them, against the constant's: an `x` or `z` bit never
                // matches, and otherwise the bits name one integer or none.
                let width = value.width().max(INTEGER_BITS);
                let value = value.extended(width, signed);
                let target = value.to_integer(signed).and_then(|target| {
                    if signed {
                        i32::try_from(target).ok()
                    } else {
                        u32::try_from(target).ok().map(|bits| bits as i32)
                    }
                });
                match target {
                    Some(target) => {
                        Self::binary_expr(BinaryOp::Eq, known, integer(i64::from(target), span))
                    }
                    None => integer(0, span),
                }
            }
            (CaseOperand::Integer(known), CaseOperand::Discrete { ordinal, width, .. })
            | (CaseOperand::Discrete { ordinal, width, .. }, CaseOperand::Integer(known)) => {
                if signed || width >= INTEGER_BITS {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "`{construct}` in analog code compares an integer with a discrete \
                             operand of at most 31 unsigned bits; this one is {width} bits{}",
                            if signed { " and signed" } else { "" }
                        )),
                        span,
                    )));
                }
                // The selection zero-extends to the integer's 32 bits, so the
                // two match when every selected bit is known and the selection
                // reads as the integer.
                let chunks = width.div_ceil(PACKED_CHUNK_BITS);
                let known_bits = (0..chunks).map(|chunk| {
                    Self::binary_expr(
                        BinaryOp::Ge,
                        self.encoded_chunk_read(ordinal, chunk, span),
                        integer(all_known(chunk_bits(width, chunk)), span),
                    )
                });
                let value = (0..chunks)
                    .map(|chunk| {
                        let aval = Self::binary_expr(
                            BinaryOp::Sub,
                            self.encoded_chunk_read(ordinal, chunk, span),
                            integer(all_known(chunk_bits(width, chunk)), span),
                        );
                        Self::binary_expr(
                            BinaryOp::Mul,
                            aval,
                            integer(1 << (chunk * PACKED_CHUNK_BITS), span),
                        )
                    })
                    .reduce(|sum, term| Self::binary_expr(BinaryOp::Add, sum, term))
                    .expect("a selection has a chunk");
                let matched = Self::binary_expr(BinaryOp::Eq, known, value);
                conjunction(known_bits.chain(std::iter::once(matched)).collect())
            }
        };
        self.read_settled_discrete_values(&mut matched)?;
        Ok(matched)
    }

    /// Chunk `chunk` of projection `ordinal`, read as discrete input: an
    /// encoded chunk is never unavailable, so the read never refuses.
    fn encoded_chunk_read(&self, ordinal: usize, chunk: u32, span: Span) -> Expression {
        let projection = &self.discrete_projection.projections[ordinal];
        let identifier = |name: &SmolStr| {
            Expression::Identifier(Identifier {
                name: format!("{name}[{chunk}]").into(),
                span,
            })
        };
        Self::binary_expr(
            BinaryOp::DiscreteValue,
            identifier(&projection.validity),
            identifier(&projection.value),
        )
    }

    /// The discrete signal `expression` selects from, when it names one.
    fn discrete_case_operand_name(&self, expression: &Expression) -> Option<SmolStr> {
        let name = match expression {
            Expression::Identifier(identifier) => &identifier.name,
            Expression::ArrayAccess(access) if access.packed.is_none() => &access.array,
            Expression::Digital(DigitalExpr::PartSelect(select)) => &select.name,
            _ => return None,
        };
        let name = self.resolve_substituted_name(name);
        self.discrete_projection
            .signals
            .contains_key(&name)
            .then_some(name)
    }

    fn case_operand(
        &mut self,
        expression: &Expression,
        construct: &str,
    ) -> CompileResult<CaseOperand> {
        let span = expression.span();
        let refuse = |detail: String| {
            CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "`{construct}` in analog code compares the bits of a discrete net or \
                     variable, a constant bit- or part-select of one, or a constant \
                     (Verilog-AMS LRM 2.4 section 7.3.2); {detail}"
                )),
                span,
            ))
        };
        if let Some(name) = self.discrete_case_operand_name(expression) {
            let shape = self.discrete_projection.signals[&name];
            if shape.real {
                return Err(refuse(format!(
                    "`{name}` is real, and Verilog-AMS LRM 2.4 section 4.2.1 makes `===` and \
                     `!==` illegal on a real"
                )));
            }
            if shape.unpacked.is_some() {
                return Err(refuse(format!(
                    "`{name}` is an array, which this compiler does not compare by case \
                     equality in analog code"
                )));
            }
            let declared = shape.range.width();
            if !shape.selectable && !matches!(expression, Expression::Identifier(_)) {
                return Err(refuse(format!(
                    "`{name}` is a scalar, which has no bits to select"
                )));
            }
            let (lsb, width, signed) = match expression {
                Expression::Identifier(_) => (0, declared, shape.signed),
                Expression::ArrayAccess(access) => {
                    let bit = self.lower_packed_selector(&access.index)?;
                    let Some(bit) = self.constant_array_index(&bit, &name)? else {
                        return Err(refuse(format!(
                            "the bit of `{name}` it selects is not constant"
                        )));
                    };
                    (shape.range.position_of(bit), 1, false)
                }
                Expression::Digital(DigitalExpr::PartSelect(select)) => {
                    let bound = |analyzer: &mut Self, bound: &Expression| {
                        let bound = analyzer.lower_packed_selector(bound)?;
                        Ok::<_, CompileError>(
                            analyzer
                                .eval_const_invariant_value(&bound)
                                .and_then(ConstantValue::as_exact_i64),
                        )
                    };
                    let (Some(msb), Some(lsb)) =
                        (bound(self, &select.msb)?, bound(self, &select.lsb)?)
                    else {
                        return Err(refuse(format!(
                            "the part of `{name}` it selects is not constant"
                        )));
                    };
                    if msb != lsb && (msb > lsb) != (shape.range.msb >= shape.range.lsb) {
                        return Err(refuse(format!(
                            "the part-select of `{name}` reverses its declared range"
                        )));
                    }
                    let width = u32::try_from(msb.abs_diff(lsb) + 1).unwrap_or(u32::MAX);
                    (shape.range.position_of(lsb), width, false)
                }
                _ => unreachable!("named by discrete_case_operand_name"),
            };
            if lsb < 0 || lsb.saturating_add(i64::from(width)) > i64::from(declared) {
                return Err(refuse(format!(
                    "it selects outside `{name}`'s declared range"
                )));
            }
            // The whole signal shares its projection with a runtime-indexed
            // read of it; a narrower selection is its own word.
            let layout = if lsb == 0 && width == declared {
                PackedArrayLayout {
                    word_lower: 0,
                    word_len: 1,
                    packed_msb: shape.range.msb,
                    packed_lsb: shape.range.lsb,
                }
            } else {
                PackedArrayLayout {
                    word_lower: 0,
                    word_len: 1,
                    packed_msb: i64::from(width) - 1,
                    packed_lsb: 0,
                }
            };
            let ordinal =
                self.register_packed_projection(&name, shape, lsb, width, Some(layout), span)?;
            return Ok(CaseOperand::Discrete {
                ordinal,
                width,
                signed,
            });
        }
        match expression {
            Expression::Digital(DigitalExpr::FourState(literal)) => {
                return Ok(CaseOperand::Constant {
                    value: FourStateValue::from_literal(&literal.value),
                    signed: literal.value.signed,
                });
            }
            Expression::Number(number) if number.raw.contains('\'') => {
                if let Ok(literal) = crate::four_state::decode(&number.raw) {
                    return Ok(CaseOperand::Constant {
                        value: FourStateValue::from_literal(&literal),
                        signed: literal.signed,
                    });
                }
                // A based decimal, `4'd9`: as wide as its size says, signed
                // only when marked (IEEE 1364-2005 section 3.5.1).
                if let Some(value) = Self::integer_literal_value(number) {
                    let size: String = number
                        .raw
                        .split_once('\'')
                        .map_or("", |(size, _)| size)
                        .chars()
                        .filter(|character| character.is_ascii_digit())
                        .collect();
                    return Ok(CaseOperand::Constant {
                        value: FourStateValue::from_integer(
                            size.parse()
                                .unwrap_or(crate::four_state::UNSIZED_FOUR_STATE_WIDTH),
                            i128::from(value),
                        ),
                        signed: crate::four_state::has_signed_marker(&number.raw),
                    });
                }
            }
            _ => {}
        }
        let lowered = self.lower_expression(expression)?;
        match self.eval_const_invariant_value(&lowered) {
            Some(ConstantValue::Integer(value)) => Ok(CaseOperand::Constant {
                value: FourStateValue::from_integer(
                    crate::numeric_literal::unsized_integer_width(value),
                    i128::from(value),
                ),
                signed: true,
            }),
            Some(ConstantValue::Real(_)) => Err(refuse(REAL_OPERAND.into())),
            None if self.infer_type(&lowered)? == ValueType::Integer => {
                Ok(CaseOperand::Integer(lowered))
            }
            None => Err(refuse(REAL_OPERAND.into())),
        }
    }
}
