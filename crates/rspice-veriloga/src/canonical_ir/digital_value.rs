//! Two-plane four-state values and the IEEE 1364-2005 section 4.1 truth
//! tables that operate on them.
//!
//! # Why two planes
//!
//! The analog half of this compiler has one value representation, `f64`, and
//! that ABI is frozen. A four-state bit is not a number: `x` and `z` are not
//! points on the real line, and any encoding that hides them inside an `f64`
//! makes every analog consumer one arithmetic operation away from producing a
//! nonsense voltage from an unknown bit.
//!
//! So a four-state value is stored the way IEEE 1364-2005 section 27.14 stores
//! one at the VPI boundary: two parallel bit planes, `aval` and `bval`, one bit
//! of each per value bit.
//!
//! | `aval` | `bval` | bit |
//! | :--- | :--- | :--- |
//! | 0 | 0 | `0` |
//! | 1 | 0 | `1` |
//! | 0 | 1 | `z` |
//! | 1 | 1 | `x` |
//!
//! The encoding is worth stating plainly because it has one property the
//! whole digital half leans on: `bval == 0` for every bit exactly when the
//! value is two-state. A great deal of 1364 semantics is "if any operand bit
//! is `x` or `z`, the result is all `x`", and in this encoding that test is
//! `bval != 0` over the words — not a scan of per-bit enums.
//!
//! # Why the tables are data
//!
//! Section 4.1 defines the bitwise operators by truth table. They are
//! transcribed here as `const` arrays in the standard's own row and column
//! order — `0`, `1`, `x`, `z` — so the tables can be read against the document
//! line by line, and so every consumer shares one copy.
//!
//! This matters more than it looks. The process interpreter, and any later
//! emitter that compiles a process to Rust or to machine code, must agree
//! bit-for-bit on what `1 & x` is. Written as `match` arms in each backend
//! they would be three transcriptions of the same table and two chances to get
//! one entry wrong. Written once as data, a disagreement is a compile error or
//! a table test, not a wrong waveform.

use crate::four_state::{FourStateBit, FourStateLiteral};
use serde::{Deserialize, Serialize};

/// Number of value bits carried by one plane word.
pub const PLANE_WORD_BITS: u32 = 32;

/// Index of a bit in the section 4.1 truth tables.
///
/// The tables are transcribed in the standard's own order, `0 1 x z`, which is
/// also [`FourStateBit`]'s declaration order. Keeping the two aligned is what
/// lets a table literal be compared against the document without a mapping
/// step in between; [`tests::table_index_matches_the_standard_column_order`]
/// pins the alignment so a future reordering of the enum cannot silently
/// permute every table.
const fn table_index(bit: FourStateBit) -> usize {
    match bit {
        FourStateBit::Zero => 0,
        FourStateBit::One => 1,
        FourStateBit::Unknown => 2,
        FourStateBit::HighImpedance => 3,
    }
}

/// The inverse of [`table_index`], used to check the alignment holds both ways.
#[cfg(test)]
const fn from_table_index(index: usize) -> FourStateBit {
    match index {
        0 => FourStateBit::Zero,
        1 => FourStateBit::One,
        2 => FourStateBit::Unknown,
        _ => FourStateBit::HighImpedance,
    }
}

/// Every four-state bit, in truth-table order.
pub const TABLE_ORDER: [FourStateBit; 4] = [
    FourStateBit::Zero,
    FourStateBit::One,
    FourStateBit::Unknown,
    FourStateBit::HighImpedance,
];

// ============================================================================
// IEEE 1364-2005 section 4.1 truth tables
// ============================================================================
//
// Rows are the left operand, columns the right, both in `0 1 x z` order.
// Transcribed from the tables in section 4.1.9 (bitwise operators). Note the
// property that distinguishes these from a three-valued logic: the result of a
// bitwise operator is never `z`. A `z` operand behaves as `x`, so `z` appears
// in the inputs and never in the outputs.

const Z: FourStateBit = FourStateBit::Zero;
const O: FourStateBit = FourStateBit::One;
const X: FourStateBit = FourStateBit::Unknown;

/// Bitwise AND (`&`), IEEE 1364-2005 section 4.1.9.
pub const AND_TABLE: [[FourStateBit; 4]; 4] = [
    //        0  1  x  z
    /* 0 */ [Z, Z, Z, Z],
    /* 1 */ [Z, O, X, X],
    /* x */ [Z, X, X, X],
    /* z */ [Z, X, X, X],
];

/// Bitwise OR (`|`), IEEE 1364-2005 section 4.1.9.
pub const OR_TABLE: [[FourStateBit; 4]; 4] = [
    //        0  1  x  z
    /* 0 */ [Z, O, X, X],
    /* 1 */ [O, O, O, O],
    /* x */ [X, O, X, X],
    /* z */ [X, O, X, X],
];

/// Bitwise XOR (`^`), IEEE 1364-2005 section 4.1.9.
pub const XOR_TABLE: [[FourStateBit; 4]; 4] = [
    //        0  1  x  z
    /* 0 */ [Z, O, X, X],
    /* 1 */ [O, Z, X, X],
    /* x */ [X, X, X, X],
    /* z */ [X, X, X, X],
];

/// Bitwise XNOR (`~^` / `^~`), IEEE 1364-2005 section 4.1.9.
///
/// Retained even though this wave's grammar has no `~^` token, because it is
/// the exact complement of [`XOR_TABLE`] and the table test proves it. A later
/// wave that adds the token gets the semantics already pinned.
pub const XNOR_TABLE: [[FourStateBit; 4]; 4] = [
    //        0  1  x  z
    /* 0 */ [O, Z, X, X],
    /* 1 */ [Z, O, X, X],
    /* x */ [X, X, X, X],
    /* z */ [X, X, X, X],
];

/// Bitwise negation (`~`), IEEE 1364-2005 section 4.1.9.
pub const NOT_TABLE: [FourStateBit; 4] = [
    //  0  1  x  z
    O, Z, X, X,
];

/// The conditional operator's ambiguous merge, IEEE 1364-2005 section 4.1.13
/// table 4-6.
///
/// Not a bitwise operator, and here anyway, because it is a section 4.1 truth
/// table and the alternative is a private transcription in the interpreter that
/// no test compares against the document.
///
/// Reached only when the condition itself is `x` or `z`: the standard then
/// evaluates *both* arms and merges them bit by bit, keeping a bit the two arms
/// agree on and yielding `x` where they do not. Rows are the `then` operand,
/// columns the `else` one. Note this is not [`XNOR_TABLE`] — agreement on `0`
/// gives `0` here, where XNOR gives `1`.
pub const CONDITIONAL_TABLE: [[FourStateBit; 4]; 4] = [
    //        0  1  x  z
    /* 0 */ [Z, X, X, X],
    /* 1 */ [X, O, X, X],
    /* x */ [X, X, X, X],
    /* z */ [X, X, X, X],
];

/// Apply a binary bitwise truth table to one bit pair.
pub const fn apply_binary(
    table: &[[FourStateBit; 4]; 4],
    left: FourStateBit,
    right: FourStateBit,
) -> FourStateBit {
    table[table_index(left)][table_index(right)]
}

/// Apply a unary bitwise truth table to one bit.
pub const fn apply_unary(table: &[FourStateBit; 4], input: FourStateBit) -> FourStateBit {
    table[table_index(input)]
}

// ============================================================================
// The two-plane value
// ============================================================================

/// A four-state value of a fixed width, stored as two bit planes.
///
/// Bit `i` of the value is `aval` bit `i` and `bval` bit `i`, LSB first, both
/// packed into [`PLANE_WORD_BITS`]-bit words. Bits above `width` within the
/// final word are always zero in both planes, which is what makes
/// [`PartialEq`] a value comparison rather than a representation comparison.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FourStateValue {
    width: u32,
    aval: Vec<u32>,
    bval: Vec<u32>,
}

impl FourStateValue {
    /// Number of words a value of this width occupies in one plane.
    pub const fn words_for(width: u32) -> usize {
        width.div_ceil(PLANE_WORD_BITS) as usize
    }

    /// A value of `width` bits, every bit `0`.
    pub fn zero(width: u32) -> Self {
        let words = Self::words_for(width);
        Self {
            width,
            aval: vec![0; words],
            bval: vec![0; words],
        }
    }

    /// A value of `width` bits, every bit set to `bit`.
    pub fn splat(width: u32, bit: FourStateBit) -> Self {
        let mut value = Self::zero(width);
        for index in 0..width {
            value.set_bit(index, bit);
        }
        value
    }

    /// Build from bits given most-significant first, the order a literal is
    /// written in and the order [`FourStateLiteral::bits`] stores.
    pub fn from_bits_msb_first(bits: &[FourStateBit]) -> Self {
        let width = bits.len() as u32;
        let mut value = Self::zero(width);
        for (offset, bit) in bits.iter().rev().enumerate() {
            value.set_bit(offset as u32, *bit);
        }
        value
    }

    /// Encode a decoded source literal.
    pub fn from_literal(literal: &FourStateLiteral) -> Self {
        Self::from_bits_msb_first(&literal.bits)
    }

    /// Encode a two-state unsigned integer of the given width, truncating.
    pub fn from_u64(width: u32, bits: u64) -> Self {
        let mut value = Self::zero(width);
        for index in 0..width.min(64) {
            if bits >> index & 1 == 1 {
                value.set_bit(index, FourStateBit::One);
            }
        }
        value
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The `aval` plane words, LSB word first.
    pub fn aval(&self) -> &[u32] {
        &self.aval
    }

    /// The `bval` plane words, LSB word first.
    pub fn bval(&self) -> &[u32] {
        &self.bval
    }

    /// Whether any bit is `x` or `z`.
    ///
    /// One word scan, because `bval` is set for exactly those two states.
    pub fn has_unknown(&self) -> bool {
        self.bval.iter().any(|word| *word != 0)
    }

    /// Bit `index`, counting from the least significant.
    pub fn bit(&self, index: u32) -> FourStateBit {
        if index >= self.width {
            return FourStateBit::Unknown;
        }
        let (word, offset) = Self::position(index);
        let a = self.aval[word] >> offset & 1;
        let b = self.bval[word] >> offset & 1;
        match (a, b) {
            (0, 0) => FourStateBit::Zero,
            (1, 0) => FourStateBit::One,
            (0, _) => FourStateBit::HighImpedance,
            (_, _) => FourStateBit::Unknown,
        }
    }

    /// Overwrite bit `index`. Out-of-range indices are ignored, which is what
    /// a part-select clamped to a declared range needs.
    pub fn set_bit(&mut self, index: u32, bit: FourStateBit) {
        if index >= self.width {
            return;
        }
        let (word, offset) = Self::position(index);
        let (a, b) = match bit {
            FourStateBit::Zero => (0, 0),
            FourStateBit::One => (1, 0),
            FourStateBit::HighImpedance => (0, 1),
            FourStateBit::Unknown => (1, 1),
        };
        let mask = 1u32 << offset;
        self.aval[word] = self.aval[word] & !mask | a << offset;
        self.bval[word] = self.bval[word] & !mask | b << offset;
    }

    const fn position(index: u32) -> (usize, u32) {
        ((index / PLANE_WORD_BITS) as usize, index % PLANE_WORD_BITS)
    }

    /// Bits most-significant first, the order the source spells them.
    pub fn bits_msb_first(&self) -> Vec<FourStateBit> {
        (0..self.width).rev().map(|index| self.bit(index)).collect()
    }

    /// Canonical `0`/`1`/`x`/`z` spelling, most significant first.
    pub fn spelling(&self) -> String {
        self.bits_msb_first()
            .into_iter()
            .map(FourStateBit::as_char)
            .collect()
    }

    /// The value as an unsigned integer, or `None` if any bit is `x` or `z`.
    ///
    /// The refusal is the point: a caller that wants a number from a value
    /// that has unknown bits is asking a question 1364 answers with `x`, and
    /// returning some number here would answer it with a lie.
    pub fn to_u64(&self) -> Option<u64> {
        if self.has_unknown() || self.width > 64 {
            return None;
        }
        let mut bits = 0u64;
        for index in 0..self.width {
            if self.bit(index) == FourStateBit::One {
                bits |= 1u64 << index;
            }
        }
        Some(bits)
    }

    /// Resize to `width`, truncating from the top or zero-extending.
    ///
    /// This is assignment-context resizing (IEEE 1364-2005 section 5.2.1),
    /// which zero-fills — *not* the literal padding rule in section 3.5.1,
    /// where a leading `x` extends with itself. The two rules differ and are
    /// deliberately in different places: the literal rule belongs to decoding
    /// source text and lives in [`crate::four_state`].
    pub fn resized(&self, width: u32) -> Self {
        let mut out = Self::zero(width);
        for index in 0..width.min(self.width) {
            out.set_bit(index, self.bit(index));
        }
        out
    }
}

// ============================================================================
// Operations
// ============================================================================

/// A bitwise operator over four-state values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BitwiseOp {
    And,
    Or,
    Xor,
    Xnor,
}

impl BitwiseOp {
    pub const fn table(self) -> &'static [[FourStateBit; 4]; 4] {
        match self {
            Self::And => &AND_TABLE,
            Self::Or => &OR_TABLE,
            Self::Xor => &XOR_TABLE,
            Self::Xnor => &XNOR_TABLE,
        }
    }
}

/// Apply a bitwise operator elementwise over two values.
///
/// Operands of unequal width are first extended to the wider of the two, per
/// IEEE 1364-2005 section 5.4.1.
pub fn bitwise(op: BitwiseOp, left: &FourStateValue, right: &FourStateValue) -> FourStateValue {
    let width = left.width().max(right.width());
    let left = left.resized(width);
    let right = right.resized(width);
    let table = op.table();
    let mut out = FourStateValue::zero(width);
    for index in 0..width {
        out.set_bit(
            index,
            apply_binary(table, left.bit(index), right.bit(index)),
        );
    }
    out
}

/// Bitwise negation (`~`).
pub fn bitwise_not(input: &FourStateValue) -> FourStateValue {
    let mut out = FourStateValue::zero(input.width());
    for index in 0..input.width() {
        out.set_bit(index, apply_unary(&NOT_TABLE, input.bit(index)));
    }
    out
}

/// The truth value of a whole value, IEEE 1364-2005 section 4.1.8.
///
/// A value is true when any bit is `1`, false when every bit is `0`, and
/// ambiguous when it is neither — that is, when it has an `x`/`z` bit but no
/// `1` to settle the question.
pub fn truth(value: &FourStateValue) -> FourStateBit {
    let mut saw_unknown = false;
    for index in 0..value.width() {
        match value.bit(index) {
            FourStateBit::One => return FourStateBit::One,
            FourStateBit::Zero => {}
            FourStateBit::Unknown | FourStateBit::HighImpedance => saw_unknown = true,
        }
    }
    if saw_unknown {
        FourStateBit::Unknown
    } else {
        FourStateBit::Zero
    }
}

/// A logical operator, which yields a one-bit result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogicalOp {
    And,
    Or,
}

/// Apply a logical operator (`&&`, `||`), IEEE 1364-2005 section 4.1.8.
///
/// Defined on the operands' truth values, so it reuses the same tables: the
/// standard's logical tables are the bitwise tables restricted to one bit.
pub fn logical(op: LogicalOp, left: &FourStateValue, right: &FourStateValue) -> FourStateValue {
    let table = match op {
        LogicalOp::And => &AND_TABLE,
        LogicalOp::Or => &OR_TABLE,
    };
    let bit = apply_binary(table, truth(left), truth(right));
    one_bit(bit)
}

/// Logical negation (`!`), IEEE 1364-2005 section 4.1.8.
pub fn logical_not(input: &FourStateValue) -> FourStateValue {
    one_bit(apply_unary(&NOT_TABLE, truth(input)))
}

/// A one-bit value carrying `bit`.
pub fn one_bit(bit: FourStateBit) -> FourStateValue {
    let mut value = FourStateValue::zero(1);
    value.set_bit(0, bit);
    value
}

/// Logical equality (`==`) and inequality (`!=`), IEEE 1364-2005 section 4.1.7.
///
/// The result is `x` if either operand has any `x` or `z` bit. This is the
/// rule that makes `==` unusable for testing whether a signal is unknown, and
/// the reason the standard also defines `===`; that operator is not in this
/// wave's grammar, so it is not implemented here.
pub fn equality(left: &FourStateValue, right: &FourStateValue, negate: bool) -> FourStateValue {
    if left.has_unknown() || right.has_unknown() {
        return one_bit(FourStateBit::Unknown);
    }
    let width = left.width().max(right.width());
    let left = left.resized(width);
    let right = right.resized(width);
    let equal = (0..width).all(|index| left.bit(index) == right.bit(index));
    one_bit(if equal != negate {
        FourStateBit::One
    } else {
        FourStateBit::Zero
    })
}

/// A relational operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationalOp {
    Lt,
    Le,
    Gt,
    Ge,
}

/// Apply a relational operator, IEEE 1364-2005 section 4.1.6.
///
/// "If either operand contains an x or z, the result is a 1-bit unknown."
pub fn relational(
    op: RelationalOp,
    left: &FourStateValue,
    right: &FourStateValue,
) -> FourStateValue {
    let (Some(left), Some(right)) = (left.to_u64(), right.to_u64()) else {
        return one_bit(FourStateBit::Unknown);
    };
    let outcome = match op {
        RelationalOp::Lt => left < right,
        RelationalOp::Le => left <= right,
        RelationalOp::Gt => left > right,
        RelationalOp::Ge => left >= right,
    };
    one_bit(if outcome {
        FourStateBit::One
    } else {
        FourStateBit::Zero
    })
}

/// An arithmetic operator over four-state values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

/// Apply an arithmetic operator, IEEE 1364-2005 section 4.1.5.
///
/// "If any operand bit value is the unknown value x, then the entire result
/// value shall be x." The result keeps the operand width, so it is all-`x` of
/// that width rather than a one-bit `x` — the difference matters to whatever
/// the result is assigned to.
///
/// Division or modulus by zero is likewise the whole result unknown, which is
/// the standard's rule and not an error this compiler raises.
pub fn arithmetic(
    op: ArithmeticOp,
    left: &FourStateValue,
    right: &FourStateValue,
) -> FourStateValue {
    let width = left.width().max(right.width());
    let (Some(left), Some(right)) = (left.to_u64(), right.to_u64()) else {
        return FourStateValue::splat(width, FourStateBit::Unknown);
    };
    let value = match op {
        ArithmeticOp::Add => left.wrapping_add(right),
        ArithmeticOp::Sub => left.wrapping_sub(right),
        ArithmeticOp::Mul => left.wrapping_mul(right),
        ArithmeticOp::Div | ArithmeticOp::Mod if right == 0 => {
            return FourStateValue::splat(width, FourStateBit::Unknown);
        }
        ArithmeticOp::Div => left / right,
        ArithmeticOp::Mod => left % right,
    };
    FourStateValue::from_u64(width, value)
}

/// Shift direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShiftOp {
    Left,
    Right,
}

/// Apply a logical shift, IEEE 1364-2005 section 4.1.12.
///
/// Vacated bits are filled with zero, and the result keeps the left operand's
/// width. An `x`/`z` bit in the *shift count* makes the whole result unknown;
/// an `x` in the value being shifted simply moves, because a shift does not
/// combine bits.
pub fn shift(op: ShiftOp, value: &FourStateValue, count: &FourStateValue) -> FourStateValue {
    let width = value.width();
    let Some(count) = count.to_u64() else {
        return FourStateValue::splat(width, FourStateBit::Unknown);
    };
    let mut out = FourStateValue::zero(width);
    if count >= u64::from(width) {
        return out;
    }
    let count = count as u32;
    for index in 0..width {
        let source = match op {
            ShiftOp::Left => index.checked_sub(count),
            ShiftOp::Right => index.checked_add(count).filter(|source| *source < width),
        };
        if let Some(source) = source {
            out.set_bit(index, value.bit(source));
        }
    }
    out
}

/// The conditional operator (`?:`), IEEE 1364-2005 section 4.1.13.
///
/// A known condition selects an arm. An ambiguous one selects neither: both
/// arms are evaluated and merged through [`CONDITIONAL_TABLE`], so a bit the
/// two arms agree on survives and one they disagree on becomes `x`. This is why
/// `x ? 1'b1 : 1'b1` is `1` rather than `x` — the answer does not depend on the
/// condition, so not knowing the condition costs nothing.
///
/// The result width is the wider arm's, per section 5.4.1.
pub fn conditional(
    condition: &FourStateValue,
    then_value: &FourStateValue,
    else_value: &FourStateValue,
) -> FourStateValue {
    let width = then_value.width().max(else_value.width());
    let then_value = then_value.resized(width);
    let else_value = else_value.resized(width);
    match truth(condition) {
        FourStateBit::One => then_value,
        FourStateBit::Zero => else_value,
        FourStateBit::Unknown | FourStateBit::HighImpedance => {
            let mut out = FourStateValue::zero(width);
            for index in 0..width {
                out.set_bit(
                    index,
                    apply_binary(
                        &CONDITIONAL_TABLE,
                        then_value.bit(index),
                        else_value.bit(index),
                    ),
                );
            }
            out
        }
    }
}

/// Select bits `[msb:lsb]` of a value, IEEE 1364-2005 section 4.2.1.
///
/// Bits outside the value are `x`, which is the standard's rule for an
/// out-of-bounds select rather than an error.
pub fn part_select(value: &FourStateValue, msb: i64, lsb: i64) -> FourStateValue {
    let (low, high) = if msb <= lsb { (msb, lsb) } else { (lsb, msb) };
    let width = (high - low + 1).max(0) as u32;
    let mut out = FourStateValue::zero(width);
    for offset in 0..width {
        let source = low + i64::from(offset);
        let bit = if source < 0 || source >= i64::from(value.width()) {
            FourStateBit::Unknown
        } else {
            value.bit(source as u32)
        };
        out.set_bit(offset, bit);
    }
    out
}

/// Concatenate values, IEEE 1364-2005 section 4.1.14.
///
/// The first operand supplies the most significant bits, matching the source
/// order in `{a, b}`.
pub fn concat(parts: &[FourStateValue]) -> FourStateValue {
    let width: u32 = parts.iter().map(FourStateValue::width).sum();
    let mut out = FourStateValue::zero(width);
    let mut offset = width;
    for part in parts {
        offset -= part.width();
        for index in 0..part.width() {
            out.set_bit(offset + index, part.bit(index));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(spelling: &str) -> FourStateValue {
        let bits: Vec<FourStateBit> = spelling
            .chars()
            .map(|character| match character {
                '0' => FourStateBit::Zero,
                '1' => FourStateBit::One,
                'x' => FourStateBit::Unknown,
                'z' => FourStateBit::HighImpedance,
                other => panic!("not a four-state digit: {other}"),
            })
            .collect();
        FourStateValue::from_bits_msb_first(&bits)
    }

    /// The tables are transcribed in the standard's column order. If
    /// [`FourStateBit`] is ever reordered, every table silently permutes, so
    /// the alignment is pinned rather than assumed.
    #[test]
    fn table_index_matches_the_standard_column_order() {
        assert_eq!(table_index(FourStateBit::Zero), 0);
        assert_eq!(table_index(FourStateBit::One), 1);
        assert_eq!(table_index(FourStateBit::Unknown), 2);
        assert_eq!(table_index(FourStateBit::HighImpedance), 3);
        for (index, bit) in TABLE_ORDER.into_iter().enumerate() {
            assert_eq!(table_index(bit), index);
            assert_eq!(from_table_index(index), bit);
        }
    }

    /// IEEE 1364-2005 section 4.1.9: a bitwise operator never yields `z`.
    #[test]
    fn no_bitwise_table_produces_high_impedance() {
        for table in [&AND_TABLE, &OR_TABLE, &XOR_TABLE, &XNOR_TABLE] {
            for row in table {
                for entry in row {
                    assert_ne!(*entry, FourStateBit::HighImpedance, "{table:?}");
                }
            }
        }
        for entry in NOT_TABLE {
            assert_ne!(entry, FourStateBit::HighImpedance);
        }
    }

    /// `z` behaves exactly as `x` in every bitwise table: the two rows and the
    /// two columns are identical. This is one statement of section 4.1.9's
    /// "z is treated as x" rule, checked against the transcription rather than
    /// assumed by it.
    #[test]
    fn high_impedance_behaves_as_unknown_in_every_table() {
        let x = table_index(FourStateBit::Unknown);
        let z = table_index(FourStateBit::HighImpedance);
        for table in [&AND_TABLE, &OR_TABLE, &XOR_TABLE, &XNOR_TABLE] {
            assert_eq!(table[x], table[z], "rows differ: {table:?}");
            for row in table {
                assert_eq!(row[x], row[z], "columns differ: {row:?}");
            }
        }
        assert_eq!(NOT_TABLE[x], NOT_TABLE[z]);
    }

    /// Both operands two-state must reproduce ordinary Boolean algebra, and
    /// XNOR must be the exact complement of XOR everywhere.
    #[test]
    fn tables_agree_with_two_state_logic_and_each_other() {
        for left in [false, true] {
            for right in [false, true] {
                let l = if left { O } else { Z };
                let r = if right { O } else { Z };
                let expect = |flag: bool| if flag { O } else { Z };
                assert_eq!(apply_binary(&AND_TABLE, l, r), expect(left && right));
                assert_eq!(apply_binary(&OR_TABLE, l, r), expect(left || right));
                assert_eq!(apply_binary(&XOR_TABLE, l, r), expect(left != right));
                assert_eq!(apply_binary(&XNOR_TABLE, l, r), expect(left == right));
            }
        }
        for left in TABLE_ORDER {
            for right in TABLE_ORDER {
                let xor = apply_binary(&XOR_TABLE, left, right);
                let xnor = apply_binary(&XNOR_TABLE, left, right);
                assert_eq!(apply_unary(&NOT_TABLE, xor), xnor, "{left:?} ^ {right:?}");
            }
        }
    }

    /// AND and OR are commutative; the standard's tables are symmetric.
    #[test]
    fn commutative_tables_are_symmetric() {
        for table in [&AND_TABLE, &OR_TABLE, &XOR_TABLE, &XNOR_TABLE] {
            for left in TABLE_ORDER {
                for right in TABLE_ORDER {
                    assert_eq!(
                        apply_binary(table, left, right),
                        apply_binary(table, right, left),
                    );
                }
            }
        }
    }

    /// The controlling values settle an operator regardless of the other
    /// operand: `0 & anything` is `0`, `1 | anything` is `1`.
    #[test]
    fn controlling_values_dominate() {
        for other in TABLE_ORDER {
            assert_eq!(apply_binary(&AND_TABLE, Z, other), Z);
            assert_eq!(apply_binary(&OR_TABLE, O, other), O);
        }
    }

    #[test]
    fn planes_encode_each_state_exactly() {
        let value = parse("10xz");
        assert_eq!(value.width(), 4);
        // MSB-first "10xz" is, LSB-first, z x 0 1.
        assert_eq!(value.bit(0), FourStateBit::HighImpedance);
        assert_eq!(value.bit(1), FourStateBit::Unknown);
        assert_eq!(value.bit(2), FourStateBit::Zero);
        assert_eq!(value.bit(3), FourStateBit::One);
        // aval: 1 for `1` and `x`; bval: 1 for `x` and `z`.
        assert_eq!(value.aval(), [0b1010]);
        assert_eq!(value.bval(), [0b0011]);
        assert_eq!(value.spelling(), "10xz");
    }

    #[test]
    fn a_two_state_value_has_an_empty_b_plane() {
        let value = parse("1011");
        assert!(!value.has_unknown());
        assert_eq!(value.bval(), [0]);
        assert_eq!(value.to_u64(), Some(0b1011));
        assert!(parse("10x1").has_unknown());
        assert_eq!(parse("10x1").to_u64(), None);
        assert!(parse("10z1").has_unknown());
    }

    #[test]
    fn values_wider_than_one_word_round_trip() {
        let mut spelling = String::new();
        for index in 0..70 {
            spelling.push(['0', '1', 'x', 'z'][index % 4]);
        }
        let value = parse(&spelling);
        assert_eq!(value.width(), 70);
        assert_eq!(value.aval().len(), 3);
        assert_eq!(value.spelling(), spelling);
    }

    #[test]
    fn a_literal_encodes_to_its_exact_planes() {
        let literal = crate::four_state::decode("4'b10xz").expect("decodes");
        let value = FourStateValue::from_literal(&literal);
        assert_eq!(value.spelling(), "10xz");
        assert_eq!(value.aval(), [0b1010]);
        assert_eq!(value.bval(), [0b0011]);
    }

    #[test]
    fn bitwise_operations_run_the_tables_elementwise() {
        assert_eq!(
            bitwise(BitwiseOp::And, &parse("1100"), &parse("1x0z")).spelling(),
            "1x00"
        );
        assert_eq!(
            bitwise(BitwiseOp::Or, &parse("1100"), &parse("1x0z")).spelling(),
            // `1|x` is 1 because 1 controls OR; `0|z` is x because 0 does not.
            "110x"
        );
        assert_eq!(
            bitwise(BitwiseOp::Xor, &parse("1100"), &parse("1x0z")).spelling(),
            "0x0x"
        );
        assert_eq!(bitwise_not(&parse("10xz")).spelling(), "01xx");
    }

    /// IEEE 1364-2005 section 4.1.8: truth is "any 1" / "all 0" / else `x`.
    #[test]
    fn truth_values_follow_the_standard() {
        assert_eq!(truth(&parse("0000")), FourStateBit::Zero);
        assert_eq!(truth(&parse("0010")), FourStateBit::One);
        assert_eq!(truth(&parse("00x0")), FourStateBit::Unknown);
        assert_eq!(truth(&parse("00z0")), FourStateBit::Unknown);
        // A `1` settles it even when other bits are unknown.
        assert_eq!(truth(&parse("1x")), FourStateBit::One);
        assert_eq!(logical_not(&parse("00x0")).spelling(), "x");
        assert_eq!(logical_not(&parse("0000")).spelling(), "1");
        assert_eq!(
            logical(LogicalOp::And, &parse("1x"), &parse("00")).spelling(),
            "0"
        );
        assert_eq!(
            logical(LogicalOp::Or, &parse("0x"), &parse("00")).spelling(),
            "x"
        );
    }

    /// IEEE 1364-2005 section 4.1.7: `==` is unknown if either side has an
    /// unknown bit, even when the two-state bits already disagree.
    #[test]
    fn equality_is_poisoned_by_unknown_bits() {
        assert_eq!(
            equality(&parse("1010"), &parse("1010"), false).spelling(),
            "1"
        );
        assert_eq!(
            equality(&parse("1010"), &parse("1011"), false).spelling(),
            "0"
        );
        assert_eq!(
            equality(&parse("1010"), &parse("101x"), false).spelling(),
            "x"
        );
        assert_eq!(
            equality(&parse("101z"), &parse("1010"), false).spelling(),
            "x"
        );
        // Inequality is poisoned identically, not the complement of `x`.
        assert_eq!(
            equality(&parse("1010"), &parse("1011"), true).spelling(),
            "1"
        );
        assert_eq!(
            equality(&parse("1010"), &parse("101x"), true).spelling(),
            "x"
        );
    }

    #[test]
    fn relational_operators_are_poisoned_by_unknown_bits() {
        assert_eq!(
            relational(RelationalOp::Lt, &parse("0010"), &parse("0011")).spelling(),
            "1"
        );
        assert_eq!(
            relational(RelationalOp::Ge, &parse("0010"), &parse("0011")).spelling(),
            "0"
        );
        assert_eq!(
            relational(RelationalOp::Lt, &parse("001x"), &parse("0011")).spelling(),
            "x"
        );
    }

    /// IEEE 1364-2005 section 4.1.5: one unknown bit makes the *whole* result
    /// unknown, at the operand width — not a one-bit `x`.
    #[test]
    fn arithmetic_poisons_the_entire_result() {
        assert_eq!(
            arithmetic(ArithmeticOp::Add, &parse("0010"), &parse("0011")).spelling(),
            "0101"
        );
        assert_eq!(
            arithmetic(ArithmeticOp::Add, &parse("001x"), &parse("0011")).spelling(),
            "xxxx"
        );
        assert_eq!(
            arithmetic(ArithmeticOp::Mul, &parse("0011"), &parse("00z1")).spelling(),
            "xxxx"
        );
        // Division by zero is unknown, not a raised error.
        assert_eq!(
            arithmetic(ArithmeticOp::Div, &parse("1000"), &parse("0000")).spelling(),
            "xxxx"
        );
        assert_eq!(
            arithmetic(ArithmeticOp::Sub, &parse("0101"), &parse("0011")).spelling(),
            "0010"
        );
    }

    /// IEEE 1364-2005 section 4.1.12: zero-fill, keep the left width, and an
    /// unknown *count* poisons everything.
    #[test]
    fn shifts_zero_fill_and_keep_their_width() {
        assert_eq!(
            shift(ShiftOp::Left, &parse("0011"), &parse("01")).spelling(),
            "0110"
        );
        assert_eq!(
            shift(ShiftOp::Right, &parse("0011"), &parse("01")).spelling(),
            "0001"
        );
        // An `x` in the shifted value simply moves.
        assert_eq!(
            shift(ShiftOp::Left, &parse("001x"), &parse("01")).spelling(),
            "01x0"
        );
        // An `x` in the count does not.
        assert_eq!(
            shift(ShiftOp::Left, &parse("0011"), &parse("x1")).spelling(),
            "xxxx"
        );
        // Shifting past the width empties the value.
        assert_eq!(
            shift(ShiftOp::Left, &parse("0011"), &parse("1000")).spelling(),
            "0000"
        );
    }

    #[test]
    fn part_select_reads_bits_and_pads_out_of_range_with_unknown() {
        let value = parse("10xz");
        assert_eq!(part_select(&value, 3, 2).spelling(), "10");
        assert_eq!(part_select(&value, 1, 0).spelling(), "xz");
        assert_eq!(part_select(&value, 0, 0).spelling(), "z");
        // Reaching past the declared width reads `x`, not a wrapped bit.
        assert_eq!(part_select(&value, 5, 4).spelling(), "xx");
        assert_eq!(part_select(&value, 4, 3).spelling(), "x1");
    }

    /// IEEE 1364-2005 section 4.1.13: a known condition selects an arm; an
    /// ambiguous one merges both through table 4-6.
    #[test]
    fn the_conditional_operator_merges_its_arms_when_the_condition_is_unknown() {
        assert_eq!(
            conditional(&parse("1"), &parse("1010"), &parse("0101")).spelling(),
            "1010"
        );
        assert_eq!(
            conditional(&parse("0"), &parse("1010"), &parse("0101")).spelling(),
            "0101"
        );
        // Bits the arms agree on survive an unknown condition; the rest are `x`.
        assert_eq!(
            conditional(&parse("x"), &parse("1100"), &parse("1010")).spelling(),
            "1xx0"
        );
        assert_eq!(
            conditional(&parse("z"), &parse("1100"), &parse("1010")).spelling(),
            "1xx0"
        );
        // Agreement is enough on its own: the condition never has to be known.
        assert_eq!(
            conditional(&parse("x"), &parse("1010"), &parse("1010")).spelling(),
            "1010"
        );
        // Agreeing on an ambiguous bit is not agreement.
        assert_eq!(
            conditional(&parse("x"), &parse("1x"), &parse("1x")).spelling(),
            "1x"
        );
        // A wide condition is reduced by its truth value, not its low bit.
        assert_eq!(
            conditional(&parse("0010"), &parse("11"), &parse("00")).spelling(),
            "11"
        );
    }

    /// Table 4-6 is its own table: agreement on `0` yields `0`, where XNOR — the
    /// other table whose diagonal is "the operands match" — yields `1`.
    #[test]
    fn the_conditional_table_is_not_the_xnor_table() {
        assert_eq!(apply_binary(&CONDITIONAL_TABLE, Z, Z), Z);
        assert_eq!(apply_binary(&XNOR_TABLE, Z, Z), O);
        for left in TABLE_ORDER {
            for right in TABLE_ORDER {
                let merged = apply_binary(&CONDITIONAL_TABLE, left, right);
                let expected = if left == right && !matches!(left, X | FourStateBit::HighImpedance)
                {
                    left
                } else {
                    X
                };
                assert_eq!(merged, expected, "{left:?} ? : {right:?}");
            }
        }
    }

    #[test]
    fn concatenation_puts_the_first_operand_highest() {
        assert_eq!(concat(&[parse("10"), parse("xz")]).spelling(), "10xz");
        assert_eq!(concat(&[parse("1")]).spelling(), "1");
    }

    /// Assignment-context resizing zero-fills, unlike the literal padding rule
    /// in section 3.5.1 where a leading `x` extends with itself.
    #[test]
    fn resizing_zero_fills_rather_than_extending_unknown() {
        assert_eq!(parse("x1").resized(4).spelling(), "00x1");
        assert_eq!(parse("1010").resized(2).spelling(), "10");
        assert_eq!(
            FourStateValue::splat(3, FourStateBit::Unknown).spelling(),
            "xxx"
        );
    }

    #[test]
    fn equal_values_compare_equal_regardless_of_construction() {
        assert_eq!(FourStateValue::from_u64(4, 0b1010), parse("1010"));
        assert_eq!(FourStateValue::zero(4), parse("0000"));
    }
}
