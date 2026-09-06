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
/// The exact complement of [`XOR_TABLE`], which the table test proves rather
/// than assumes — so `a ~^ b` and `~(a ^ b)` cannot disagree, whichever way a
/// source spells it and whichever way the lexer munches it.
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

/// Number of plane words a value carries without touching the allocator.
const INLINE_WORDS: usize = 2;

/// The widest value whose planes live inside the value itself.
///
/// Two [`PLANE_WORD_BITS`]-bit words per plane, so sixty-four bits. The cut is
/// not arbitrary: it covers every `reg`, `integer` and bus width a process
/// declares in practice, and it is exactly the width `$realtobits` produces, so
/// the one node that manufactures a value from a machine word manufactures one
/// that still fits. A wider value is legal and falls back to the heap.
pub const INLINE_VALUE_BITS: u32 = PLANE_WORD_BITS * INLINE_WORDS as u32;

/// Where one value's two bit planes live.
///
/// **The invariant**, established by [`FourStateValue::zero`] — the only
/// constructor of a representation — and depended on by every comparison: a
/// value of at most [`INLINE_VALUE_BITS`] bits is always [`Planes::Inline`],
/// and a wider one is always [`Planes::Heap`]. Two values of equal width
/// therefore always have the same variant, which is what lets equality and
/// hashing read the planes as slices without a representation ever standing in
/// for a value.
#[derive(Debug, Clone)]
enum Planes {
    Inline {
        aval: [u32; INLINE_WORDS],
        bval: [u32; INLINE_WORDS],
    },
    Heap {
        aval: Vec<u32>,
        bval: Vec<u32>,
    },
}

/// A four-state value of a fixed width, stored as two bit planes.
///
/// Bit `i` of the value is `aval` bit `i` and `bval` bit `i`, LSB first, both
/// packed into [`PLANE_WORD_BITS`]-bit words. Bits above `width` within the
/// final word are always zero in both planes, which is what makes
/// [`PartialEq`] a value comparison rather than a representation comparison.
///
/// # Why the planes are inline up to 64 bits
///
/// A one-bit signal read is the most frequent operation in a digital run, and
/// it produces a value. Two heap planes made that read two allocations and a
/// clone of it two more; a process activation of five instructions was paying
/// the allocator roughly thirty times. Every width the language actually uses
/// fits in two words per plane, so the planes are carried by value and the
/// heap is kept for the widths that do not — the same value, the same bits, the
/// same answers, with the allocator out of the loop.
#[derive(Clone)]
pub struct FourStateValue {
    width: u32,
    planes: Planes,
}

/// The serialized shape of a value, which is the shape it had when both planes
/// were `Vec`s.
///
/// Written out rather than derived so that the representation split above is
/// invisible on the wire: an artifact written by an earlier build reads back
/// here, and one written here reads back there.
#[derive(Serialize, Deserialize)]
#[serde(rename = "FourStateValue")]
struct FourStateValueRepr {
    width: u32,
    aval: Vec<u32>,
    bval: Vec<u32>,
}

impl Serialize for FourStateValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        FourStateValueRepr {
            width: self.width,
            aval: self.aval().to_vec(),
            bval: self.bval().to_vec(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FourStateValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let repr = FourStateValueRepr::deserialize(deserializer)?;
        Ok(Self::from_planes(repr.width, &repr.aval, &repr.bval))
    }
}

impl std::fmt::Debug for FourStateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FourStateValue")
            .field("width", &self.width)
            .field("aval", &self.aval())
            .field("bval", &self.bval())
            .finish()
    }
}

impl PartialEq for FourStateValue {
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width {
            return false;
        }
        // The invariant above makes equal widths mean equal variants, so the
        // inline pair is the whole of the hot path and the slice comparison is
        // the wide one. Comparing all `INLINE_WORDS` rather than the occupied
        // prefix is exact because the bits above `width` are always zero.
        match (&self.planes, &other.planes) {
            (
                Planes::Inline {
                    aval: left_a,
                    bval: left_b,
                },
                Planes::Inline {
                    aval: right_a,
                    bval: right_b,
                },
            ) => left_a == right_a && left_b == right_b,
            _ => self.aval() == other.aval() && self.bval() == other.bval(),
        }
    }
}

impl Eq for FourStateValue {}

impl std::hash::Hash for FourStateValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Over the occupied words in both cases, so a value hashes the same
        // whichever plane representation carries it.
        self.width.hash(state);
        self.aval().hash(state);
        self.bval().hash(state);
    }
}

impl FourStateValue {
    /// Number of words a value of this width occupies in one plane.
    pub const fn words_for(width: u32) -> usize {
        width.div_ceil(PLANE_WORD_BITS) as usize
    }

    /// A value of `width` bits, every bit `0`.
    ///
    /// The one place the representation is chosen, which is what makes the
    /// [`Planes`] invariant a property of the type rather than a convention.
    pub fn zero(width: u32) -> Self {
        let planes = if width <= INLINE_VALUE_BITS {
            Planes::Inline {
                aval: [0; INLINE_WORDS],
                bval: [0; INLINE_WORDS],
            }
        } else {
            let words = Self::words_for(width);
            Planes::Heap {
                aval: vec![0; words],
                bval: vec![0; words],
            }
        };
        Self { width, planes }
    }

    /// Rebuild a value from raw plane words, masking anything above `width`.
    ///
    /// Only [`Deserialize`] needs this: the planes it is handed came from some
    /// other build's representation, and the masking is what stops a
    /// hand-edited or truncated artifact from producing a value whose spare
    /// bits make it compare unequal to itself.
    fn from_planes(width: u32, aval: &[u32], bval: &[u32]) -> Self {
        let mut value = Self::zero(width);
        let words = Self::words_for(width);
        {
            let (out_a, out_b) = value.plane_words_mut();
            for index in 0..words {
                out_a[index] = aval.get(index).copied().unwrap_or(0);
                out_b[index] = bval.get(index).copied().unwrap_or(0);
            }
        }
        value.mask_top_word();
        value
    }

    /// The plane words this value occupies, LSB word first.
    fn plane_words(&self) -> (&[u32], &[u32]) {
        match &self.planes {
            Planes::Inline { aval, bval } => (aval.as_slice(), bval.as_slice()),
            Planes::Heap { aval, bval } => (aval.as_slice(), bval.as_slice()),
        }
    }

    fn plane_words_mut(&mut self) -> (&mut [u32], &mut [u32]) {
        match &mut self.planes {
            Planes::Inline { aval, bval } => (aval.as_mut_slice(), bval.as_mut_slice()),
            Planes::Heap { aval, bval } => (aval.as_mut_slice(), bval.as_mut_slice()),
        }
    }

    /// Clear both planes above the last value bit.
    ///
    /// The representation invariant every comparison rests on. A word-level
    /// write covers the whole word, and a value whose spare bits were set would
    /// compare unequal to the same value built one bit at a time.
    fn mask_top_word(&mut self) {
        let width = self.width;
        let remainder = width % PLANE_WORD_BITS;
        let words = Self::words_for(width);
        let (aval, bval) = self.plane_words_mut();
        if remainder != 0 {
            let mask = (1u32 << remainder) - 1;
            aval[words - 1] &= mask;
            bval[words - 1] &= mask;
        }
        // The inline representation carries words a narrow value does not
        // occupy at all; they are zero on construction and are kept so here.
        for index in words..aval.len() {
            aval[index] = 0;
            bval[index] = 0;
        }
    }

    /// A value of `width` bits, every bit set to `bit`.
    pub fn splat(width: u32, bit: FourStateBit) -> Self {
        let mut value = Self::zero(width);
        let (a, b) = match bit {
            FourStateBit::Zero => return value,
            FourStateBit::One => (u32::MAX, 0),
            FourStateBit::HighImpedance => (0, u32::MAX),
            FourStateBit::Unknown => (u32::MAX, u32::MAX),
        };
        let words = Self::words_for(width);
        {
            let (aval, bval) = value.plane_words_mut();
            for index in 0..words {
                aval[index] = a;
                bval[index] = b;
            }
        }
        value.mask_top_word();
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
        Self::from_integer(width, i128::from(bits))
    }

    /// Encode a two-state integer of the given width, keeping its low bits.
    ///
    /// Signed and unsigned share this because two's complement gives them the
    /// same bits: -5 and 11 are one four-bit pattern, and which of the two a
    /// reader calls it is [`Self::to_integer`]'s question rather than this
    /// one's.
    pub fn from_integer(width: u32, bits: i128) -> Self {
        let bits = bits as u128;
        let mut value = Self::zero(width);
        for index in 0..width.min(128) {
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
        let words = Self::words_for(self.width);
        &self.plane_words().0[..words]
    }

    /// The `bval` plane words, LSB word first.
    pub fn bval(&self) -> &[u32] {
        let words = Self::words_for(self.width);
        &self.plane_words().1[..words]
    }

    /// Whether any bit is `x` or `z`.
    ///
    /// One word scan, because `bval` is set for exactly those two states.
    pub fn has_unknown(&self) -> bool {
        // Over the whole `bval` plane rather than the occupied prefix: the
        // spare words of an inline value are zero by the masking invariant, so
        // the answer is the same and the bound is a constant.
        self.plane_words().1.iter().any(|word| *word != 0)
    }

    /// Bit `index`, counting from the least significant.
    pub fn bit(&self, index: u32) -> FourStateBit {
        if index >= self.width {
            return FourStateBit::Unknown;
        }
        let (word, offset) = Self::position(index);
        let (aval, bval) = self.plane_words();
        let a = aval[word] >> offset & 1;
        let b = bval[word] >> offset & 1;
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
        let (aval, bval) = self.plane_words_mut();
        aval[word] = aval[word] & !mask | a << offset;
        bval[word] = bval[word] & !mask | b << offset;
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
        // Read by word rather than by bit. With no unknown bit the `bval`
        // plane is empty, so every set `aval` bit is a `1` and the two planes'
        // encoding collapses to the plain integer the words already hold; bits
        // above `width` are zero by the masking invariant.
        let mut bits = 0u64;
        for (index, word) in self.aval().iter().enumerate() {
            bits |= u64::from(*word) << (index as u32 * PLANE_WORD_BITS);
        }
        Some(bits)
    }

    /// The bit in the sign position — the most significant one.
    ///
    /// `0` for a value of no width at all, which has no sign position and no
    /// sign. That is reachable: IEEE 1364-2005 section 4.1.14 permits a
    /// replication count of zero, so `{0{a}}` is a concatenation of nothing.
    /// Such a value is unsigned by rule (f) and never reaches a signed
    /// extension, so this is a guard against an arithmetic underflow rather
    /// than a rule about signedness.
    pub fn sign_bit(&self) -> FourStateBit {
        if self.width == 0 {
            return FourStateBit::Zero;
        }
        self.bit(self.width - 1)
    }

    /// The value as an integer under `signed`, or `None` if any bit is `x`/`z`.
    ///
    /// Unsigned reads the bits as a magnitude. Signed reads them as two's
    /// complement *at this value's own width*, so a four-bit `1111` is -1 while
    /// an eight-bit `00001111` is 15 — the same bits under two declarations are
    /// two numbers, which is the whole of IEEE 1364-2005 section 5.4.2 in one
    /// sentence.
    ///
    /// Reading at the operand's own width is also what lets a comparison
    /// between operands of different widths be made on the numbers rather than
    /// on extended bit patterns: extension preserves the number by
    /// construction, so comparing the numbers compares the extended values
    /// without building them.
    pub fn to_integer(&self, signed: bool) -> Option<i128> {
        let magnitude = i128::from(self.to_u64()?);
        if signed && self.sign_bit() == FourStateBit::One {
            return Some(magnitude - (1i128 << self.width));
        }
        Some(magnitude)
    }

    /// Extend or truncate to `width`, filling by `signed`.
    ///
    /// IEEE 1364-2005 section 5.4.1 extends a context-determined operand before
    /// the operator runs, and section 5.4.2 decides with what: the sign bit when
    /// the *expression* is signed, zero when it is not. Section 4.3.2 makes an
    /// `x` or `z` in the sign position extend with itself, which falls out of
    /// copying the bit rather than testing it.
    ///
    /// Not the literal padding rule of section 3.5.1, where a leading `x`
    /// extends with itself whatever the signedness. That rule belongs to
    /// decoding source text and lives in [`crate::four_state`].
    pub fn extended(&self, width: u32, signed: bool) -> Self {
        // Extending to the width a value already has copies every bit and
        // fills nothing, whatever the fill would have been, so it is the value
        // itself. Named here because it is the common case on the hot path —
        // a right-hand side usually arrives at its target's declared width —
        // and because a copy of an inline value is a register move.
        if width == self.width {
            return self.clone();
        }
        let fill = if signed {
            self.sign_bit()
        } else {
            FourStateBit::Zero
        };
        let mut out = Self::splat(width, fill);
        for index in 0..width.min(self.width) {
            out.set_bit(index, self.bit(index));
        }
        out
    }

    /// Resize to `width`, truncating from the top or zero-extending.
    ///
    /// Assignment-context resizing, IEEE 1364-2005 section 5.2.1: the
    /// unsigned half of [`Self::extended`], named for the clause that asks for
    /// it.
    pub fn resized(&self, width: u32) -> Self {
        self.extended(width, false)
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
/// the reason the standard also defines `===` — which is [`case_match`] with
/// [`DigitalCaseMatch::Exact`], not a variant of this function.
/// `signed` is the sign of the comparison's own context, IEEE 1364-2005
/// section 5.4.2: both operands are signed, or the comparison is unsigned. It
/// decides only how the narrower operand reaches the wider one — the *result*
/// is an unsigned bit either way, which is rule (g).
pub fn equality(
    left: &FourStateValue,
    right: &FourStateValue,
    negate: bool,
    signed: bool,
) -> FourStateValue {
    if left.has_unknown() || right.has_unknown() {
        return one_bit(FourStateBit::Unknown);
    }
    let width = left.width().max(right.width());
    let left = left.extended(width, signed);
    let right = right.extended(width, signed);
    let equal = (0..width).all(|index| left.bit(index) == right.bit(index));
    one_bit(if equal != negate {
        FourStateBit::One
    } else {
        FourStateBit::Zero
    })
}

/// Which `case` form a match test came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DigitalCaseMatch {
    /// `case`: every bit must be identical, `x` and `z` included.
    Exact,
    /// `casez`: a `z` (or `?`) in either operand is a don't-care.
    WildcardZ,
    /// `casex`: an `x`, `z`, or `?` in either operand is a don't-care.
    WildcardXZ,
}

impl DigitalCaseMatch {
    /// Whether a bit of either operand makes its position a don't-care.
    ///
    /// IEEE 1364-2005 section 9.5.1 puts the don't-care on *either* operand,
    /// not only on the case item: a `casez` selector holding `z` matches an
    /// item holding anything at that position. The asymmetric reading — the
    /// item may wildcard, the selector may not — is the common
    /// misimplementation, and this is the whole difference between the forms.
    const fn ignores(self, bit: FourStateBit) -> bool {
        match self {
            Self::Exact => false,
            Self::WildcardZ => matches!(bit, FourStateBit::HighImpedance),
            Self::WildcardXZ => matches!(bit, FourStateBit::HighImpedance | FourStateBit::Unknown),
        }
    }

    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Exact => "case",
            Self::WildcardZ => "casez",
            Self::WildcardXZ => "casex",
        }
    }
}

/// Match a `case` item against the selector, IEEE 1364-2005 sections 9.5 and
/// 9.5.1 — and, with [`DigitalCaseMatch::Exact`], the `===` operator of
/// section 4.1.8.
///
/// One bit, and never an unknown one: a case item either matches or does not,
/// which is what makes `case` usable where `==` is not. The comparison is an
/// identity comparison at the wider of the two widths — section 9.5 extends
/// every case expression to the width of the widest, section 4.1.8 zero-fills
/// the shorter operand of a `===`, and section 5.2.1's zero-fill is what both
/// extensions do.
///
/// `===` and an exact `case` arm being one function is not a shortcut. They are
/// the same comparison in the standard, and giving each its own transcription
/// would give a compiler two chances to disagree about `4'b10xz === 4'b10xz`.
pub fn case_match(
    kind: DigitalCaseMatch,
    selector: &FourStateValue,
    label: &FourStateValue,
    signed: bool,
) -> FourStateValue {
    let width = selector.width().max(label.width());
    let selector = selector.extended(width, signed);
    let label = label.extended(width, signed);
    let matched = (0..width).all(|index| {
        let (left, right) = (selector.bit(index), label.bit(index));
        kind.ignores(left) || kind.ignores(right) || left == right
    });
    one_bit(if matched {
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

/// Apply a relational operator, IEEE 1364-2005 sections 4.1.6 and 5.4.2.
///
/// "If either operand contains an x or z, the result is a 1-bit unknown."
///
/// `signed` is the one place a relational operator's answer depends on how its
/// operands were declared: section 5.4.2 makes the comparison signed when both
/// operands are signed and unsigned when either is not, so `-1 < 0` is true
/// between two signed operands and false the moment one of them is a plain
/// `reg`. One function rather than two, because the only difference is which
/// number the bits stand for.
pub fn relational(
    op: RelationalOp,
    left: &FourStateValue,
    right: &FourStateValue,
    signed: bool,
) -> FourStateValue {
    let (Some(left), Some(right)) = (left.to_integer(signed), right.to_integer(signed)) else {
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

/// Apply an arithmetic operator, IEEE 1364-2005 sections 4.1.5 and 5.4.2.
///
/// "If any operand bit value is the unknown value x, then the entire result
/// value shall be x." The result keeps the operand width, so it is all-`x` of
/// that width rather than a one-bit `x` — the difference matters to whatever
/// the result is assigned to.
///
/// Division or modulus by zero is likewise the whole result unknown, which is
/// the standard's rule and not an error this compiler raises.
///
/// # What `signed` changes, and what it does not
///
/// `+`, `-` and `*` on two's complement produce the same bits either way at a
/// common width, which is why the whole of signed arithmetic lives in the
/// *extension* that section 5.4.1 performs before the operator runs rather than
/// in the operator. The flag is carried through them anyway, and pinned as
/// inert by a test, because a node that says which arithmetic it is performing
/// is worth more than one whose meaning rests on an invariant its reader has to
/// rediscover.
///
/// `/` and `%` are where it bites: section 4.1.5 truncates division toward zero
/// and gives the modulus the sign of its *first* operand, so `-7 / 2` is -3 and
/// `-7 % 2` is -1 — neither of which the unsigned reading of those bits gives.
pub fn arithmetic(
    op: ArithmeticOp,
    left: &FourStateValue,
    right: &FourStateValue,
    signed: bool,
) -> FourStateValue {
    let width = left.width().max(right.width());
    let (Some(left), Some(right)) = (left.to_integer(signed), right.to_integer(signed)) else {
        return FourStateValue::splat(width, FourStateBit::Unknown);
    };
    let value = match op {
        ArithmeticOp::Add => left.wrapping_add(right),
        ArithmeticOp::Sub => left.wrapping_sub(right),
        ArithmeticOp::Mul => left.wrapping_mul(right),
        ArithmeticOp::Div | ArithmeticOp::Mod if right == 0 => {
            return FourStateValue::splat(width, FourStateBit::Unknown);
        }
        // Rust's `/` truncates toward zero and its `%` takes the sign of the
        // left operand, which is section 4.1.5's rule exactly. `wrapping_`
        // covers the one pair that has no representable quotient — the most
        // negative value over -1 — whose truncation to `width` bits is itself.
        ArithmeticOp::Div => left.wrapping_div(right),
        ArithmeticOp::Mod => left.wrapping_rem(right),
    };
    FourStateValue::from_integer(width, value)
}

/// Shift direction, and what fills the positions it vacates.
///
/// Three rather than two, because IEEE 1364-2005 section 4.1.12 gives `>>>` a
/// fill rule of its own. There is no `ArithmeticLeft`: the standard makes
/// `<<<` and `<<` the same operation, so the lowering spells one of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShiftOp {
    Left,
    Right,
    /// `>>>` on a signed expression: vacated positions take the sign bit.
    ///
    /// Emitted only when the shift's own expression is signed. Section 4.1.12
    /// makes `>>>` fill with zero when the result type is unsigned — which is
    /// [`Self::Right`] — so the lowering decides once, and a reader of this node
    /// never has to ask whether it means what it says.
    ArithmeticRight,
}

/// Apply a shift, IEEE 1364-2005 section 4.1.12.
///
/// The result keeps the left operand's width, and the positions the shift
/// vacates take the operator's fill: zero for `<<` and `>>`, and the sign bit
/// for an arithmetic `>>>`. An `x`/`z` bit in the *shift count* makes the whole
/// result unknown; an `x` in the value being shifted simply moves, because a
/// shift does not combine bits — and an `x` in the *sign* position of a `>>>`
/// fills with itself, which is section 4.3.2's rule falling out of copying the
/// bit rather than testing it.
pub fn shift(op: ShiftOp, value: &FourStateValue, count: &FourStateValue) -> FourStateValue {
    let width = value.width();
    let Some(count) = count.to_u64() else {
        return FourStateValue::splat(width, FourStateBit::Unknown);
    };
    let fill = match op {
        ShiftOp::Left | ShiftOp::Right => FourStateBit::Zero,
        ShiftOp::ArithmeticRight => value.sign_bit(),
    };
    let mut out = FourStateValue::splat(width, fill);
    if count >= u64::from(width) {
        return out;
    }
    let count = count as u32;
    for index in 0..width {
        let source = match op {
            ShiftOp::Left => index.checked_sub(count),
            ShiftOp::Right | ShiftOp::ArithmeticRight => {
                index.checked_add(count).filter(|source| *source < width)
            }
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

/// Select the bits at positions `msb` down to `lsb` of a value, IEEE
/// 1364-2005 section 4.2.1.
///
/// **Positions, counting from the least significant end — not declared
/// indices.** A value has no declaration; the bit a declaration names `i`
/// lives at
/// [`VectorBounds::position_of(i)`](crate::semantic::VectorBounds::position_of),
/// and the lowering has already applied that by the time a select reaches
/// here. Passing a declared index straight through is exact only for a range
/// anchored at zero and wrong for every other one.
///
/// Positions outside the value are `x`, which is the standard's rule for an
/// out-of-bounds select rather than an error. That is also what makes a
/// declared index the range does not name come back `x`: it maps to a position
/// off the end, and lands here.
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

// ============================================================================
// Real values
// ============================================================================
//
// Verilog-AMS LRM 2.4 section 3.7 puts a second value domain in the discrete
// half of the language: a `wreal` net carries a real, not bits. These are its
// operators, kept beside the four-state ones rather than in a module of their
// own, because the question they answer is the same question — what one
// discrete-domain operator does to its operands — and a reader comparing the
// two domains should not have to change files to do it.
//
// # What is deliberately absent
//
// Every conversion between the two. Section 3.7 says a `wreal` "cannot be
// connected to any other wires, although connection to explicitly declared
// 64-bit wires can be done via system tasks `$realtobits` and `$bitstoreal`" —
// the standard's own answer to real-versus-bits is an explicit call, not a
// coercion. So there is no `real_from_four_state` here, and none is wanted: a
// four-state value holding `x` has no real to be, and the lowering refuses a
// mixed operand pair by name rather than picking one.

/// An arithmetic operator over real values.
///
/// Four, not five: IEEE 1364-2005 section 5.1 excludes `%` from the operators
/// real operands may be used with, and there is no modulus here to be tempted
/// by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RealArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl RealArithmeticOp {
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
        }
    }
}

/// Apply a real arithmetic operator.
///
/// Plain IEEE 754 double arithmetic, including its answer for division by
/// zero. That is deliberate and is the difference from [`arithmetic`], which
/// makes a four-state division by zero all-`x`: the four-state rule exists
/// because 1364 says so, and there is no real-valued `x` for the same rule to
/// produce. A real model that divides by zero gets an infinity it can see
/// rather than a zero it cannot.
pub fn real_arithmetic(op: RealArithmeticOp, left: f64, right: f64) -> f64 {
    match op {
        RealArithmeticOp::Add => left + right,
        RealArithmeticOp::Sub => left - right,
        RealArithmeticOp::Mul => left * right,
        RealArithmeticOp::Div => left / right,
    }
}

/// A comparison between two real values.
///
/// Equality is here rather than beside [`equality`] because the four-state one
/// is a different operator: section 4.1.7's `==` can answer `x`, and this one
/// cannot — two reals are equal or they are not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RealCompareOp {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

impl RealCompareOp {
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Eq => "==",
            Self::Ne => "!=",
        }
    }
}

/// Compare two reals, yielding IEEE 1364-2005 section 5.4.2 rule (g)'s one
/// unsigned bit.
///
/// Never `x`. The four-state comparisons answer `x` when an operand holds one,
/// and a real cannot hold one, so the two-valued answer here is the complete
/// one rather than a simplification of it.
pub fn real_compare(op: RealCompareOp, left: f64, right: f64) -> FourStateValue {
    let outcome = match op {
        RealCompareOp::Lt => left < right,
        RealCompareOp::Le => left <= right,
        RealCompareOp::Gt => left > right,
        RealCompareOp::Ge => left >= right,
        RealCompareOp::Eq => left == right,
        RealCompareOp::Ne => left != right,
    };
    one_bit(if outcome {
        FourStateBit::One
    } else {
        FourStateBit::Zero
    })
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
            equality(&parse("1010"), &parse("1010"), false, false).spelling(),
            "1"
        );
        assert_eq!(
            equality(&parse("1010"), &parse("1011"), false, false).spelling(),
            "0"
        );
        assert_eq!(
            equality(&parse("1010"), &parse("101x"), false, false).spelling(),
            "x"
        );
        assert_eq!(
            equality(&parse("101z"), &parse("1010"), false, false).spelling(),
            "x"
        );
        // Inequality is poisoned identically, not the complement of `x`.
        assert_eq!(
            equality(&parse("1010"), &parse("1011"), true, false).spelling(),
            "1"
        );
        assert_eq!(
            equality(&parse("1010"), &parse("101x"), true, false).spelling(),
            "x"
        );
    }

    #[test]
    fn relational_operators_are_poisoned_by_unknown_bits() {
        assert_eq!(
            relational(RelationalOp::Lt, &parse("0010"), &parse("0011"), false).spelling(),
            "1"
        );
        assert_eq!(
            relational(RelationalOp::Ge, &parse("0010"), &parse("0011"), false).spelling(),
            "0"
        );
        assert_eq!(
            relational(RelationalOp::Lt, &parse("001x"), &parse("0011"), false).spelling(),
            "x"
        );
    }

    /// IEEE 1364-2005 section 4.1.5: one unknown bit makes the *whole* result
    /// unknown, at the operand width — not a one-bit `x`.
    #[test]
    fn arithmetic_poisons_the_entire_result() {
        assert_eq!(
            arithmetic(ArithmeticOp::Add, &parse("0010"), &parse("0011"), false).spelling(),
            "0101"
        );
        assert_eq!(
            arithmetic(ArithmeticOp::Add, &parse("001x"), &parse("0011"), false).spelling(),
            "xxxx"
        );
        assert_eq!(
            arithmetic(ArithmeticOp::Mul, &parse("0011"), &parse("00z1"), false).spelling(),
            "xxxx"
        );
        // Division by zero is unknown, not a raised error.
        assert_eq!(
            arithmetic(ArithmeticOp::Div, &parse("1000"), &parse("0000"), false).spelling(),
            "xxxx"
        );
        assert_eq!(
            arithmetic(ArithmeticOp::Sub, &parse("0101"), &parse("0011"), false).spelling(),
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

    /// IEEE 1364-2005 section 5.4.2: the same bits are two numbers, and which
    /// one they are is the declaration's answer rather than the value's.
    #[test]
    fn a_value_reads_as_two_numbers_depending_on_its_signedness() {
        assert_eq!(parse("1111").to_integer(false), Some(15));
        assert_eq!(parse("1111").to_integer(true), Some(-1));
        assert_eq!(parse("00001111").to_integer(true), Some(15));
        assert_eq!(parse("1000").to_integer(true), Some(-8));
        assert_eq!(parse("0111").to_integer(true), Some(7));
        // An unknown bit has no number under either reading.
        assert_eq!(parse("1x11").to_integer(true), None);
    }

    /// Section 5.4.1's extension, both fills. The signed one copies the top
    /// bit, which section 4.3.2 makes an `x` or `z` sign extend as itself.
    #[test]
    fn extension_fills_with_zero_or_with_the_sign_bit() {
        assert_eq!(parse("1111").extended(8, false).spelling(), "00001111");
        assert_eq!(parse("1111").extended(8, true).spelling(), "11111111");
        assert_eq!(parse("0111").extended(8, true).spelling(), "00000111");
        assert_eq!(parse("x111").extended(8, true).spelling(), "xxxxx111");
        assert_eq!(parse("z111").extended(8, true).spelling(), "zzzzz111");
        // Truncation does not consult the sign at all.
        assert_eq!(parse("11110000").extended(4, true).spelling(), "0000");
        assert_eq!(parse("11110000").extended(4, false).spelling(), "0000");
    }

    /// Section 4.1.6 with 5.4.2: `-1 < 0` under a signed comparison and not
    /// under an unsigned one, from one pair of bit patterns.
    #[test]
    fn relational_operators_read_their_operands_by_signedness() {
        assert_eq!(
            relational(RelationalOp::Lt, &parse("1111"), &parse("0000"), true).spelling(),
            "1",
            "-1 < 0"
        );
        assert_eq!(
            relational(RelationalOp::Lt, &parse("1111"), &parse("0000"), false).spelling(),
            "0",
            "15 < 0 is false"
        );
        // Operands of different widths are compared as numbers, which is the
        // same answer extending them first would give.
        assert_eq!(
            relational(RelationalOp::Lt, &parse("1111"), &parse("00000001"), true).spelling(),
            "1",
            "-1 < 1"
        );
        assert_eq!(
            relational(RelationalOp::Lt, &parse("1111"), &parse("00000001"), false).spelling(),
            "0",
            "15 < 1 is false"
        );
        // An unknown bit still poisons the result under either reading.
        assert_eq!(
            relational(RelationalOp::Lt, &parse("111x"), &parse("0000"), true).spelling(),
            "x"
        );
    }

    /// Section 4.1.7 with 5.4.2: equality compares extended operands, and the
    /// extension is the comparison's own signedness.
    #[test]
    fn equality_extends_its_operands_by_signedness() {
        // Four-bit `1111` against eight-bit `11111111`: equal when both are
        // signed (-1 and -1) and not when they are unsigned (15 and 255).
        assert_eq!(
            equality(&parse("1111"), &parse("11111111"), false, true).spelling(),
            "1"
        );
        assert_eq!(
            equality(&parse("1111"), &parse("11111111"), false, false).spelling(),
            "0"
        );
    }

    /// Section 4.1.5: `+ - *` are one operation on two's complement, and `/`
    /// and `%` are two.
    #[test]
    fn only_division_and_modulus_read_the_signedness() {
        for (op, left, right) in [
            (ArithmeticOp::Add, "1001", "0010"),
            (ArithmeticOp::Sub, "1001", "0010"),
            (ArithmeticOp::Mul, "1001", "0010"),
        ] {
            assert_eq!(
                arithmetic(op, &parse(left), &parse(right), true),
                arithmetic(op, &parse(left), &parse(right), false),
                "{op:?} must not depend on the signedness at a common width"
            );
        }
        // 9 / 2 = 4, and -7 / 2 truncates toward zero to -3.
        assert_eq!(
            arithmetic(ArithmeticOp::Div, &parse("1001"), &parse("0010"), false).spelling(),
            "0100"
        );
        assert_eq!(
            arithmetic(ArithmeticOp::Div, &parse("1001"), &parse("0010"), true).spelling(),
            "1101"
        );
        // The modulus takes the sign of its first operand: 9 % 2 = 1 and
        // -7 % 2 = -1.
        assert_eq!(
            arithmetic(ArithmeticOp::Mod, &parse("1001"), &parse("0010"), false).spelling(),
            "0001"
        );
        assert_eq!(
            arithmetic(ArithmeticOp::Mod, &parse("1001"), &parse("0010"), true).spelling(),
            "1111"
        );
        // The one signed pair with no representable quotient truncates to
        // itself rather than trapping: -8 / -1 in four bits is -8.
        assert_eq!(
            arithmetic(ArithmeticOp::Div, &parse("1000"), &parse("1111"), true).spelling(),
            "1000"
        );
        // Division by zero is still unknown, under either reading.
        assert_eq!(
            arithmetic(ArithmeticOp::Div, &parse("1001"), &parse("0000"), true).spelling(),
            "xxxx"
        );
    }

    /// Section 4.1.12: `>>>` fills with the sign bit, `>>` with zero, and both
    /// keep the shifted value's width.
    #[test]
    fn arithmetic_right_shift_fills_with_the_sign_bit() {
        assert_eq!(
            shift(ShiftOp::ArithmeticRight, &parse("10000000"), &parse("010")).spelling(),
            "11100000"
        );
        assert_eq!(
            shift(ShiftOp::Right, &parse("10000000"), &parse("010")).spelling(),
            "00100000"
        );
        // A positive value fills with its own zero, so `>>>` and `>>` agree.
        assert_eq!(
            shift(ShiftOp::ArithmeticRight, &parse("01000000"), &parse("010")).spelling(),
            shift(ShiftOp::Right, &parse("01000000"), &parse("010")).spelling(),
        );
        // Section 4.3.2: an unknown sign bit fills with itself.
        assert_eq!(
            shift(ShiftOp::ArithmeticRight, &parse("x0000000"), &parse("010")).spelling(),
            "xxx00000"
        );
        // Shifting past the width leaves the fill and nothing else: all sign
        // bits for `>>>`, all zeros for `>>`.
        assert_eq!(
            shift(ShiftOp::ArithmeticRight, &parse("1000"), &parse("1000")).spelling(),
            "1111"
        );
        assert_eq!(
            shift(ShiftOp::Right, &parse("1000"), &parse("1000")).spelling(),
            "0000"
        );
    }

    #[test]
    fn equal_values_compare_equal_regardless_of_construction() {
        assert_eq!(FourStateValue::from_u64(4, 0b1010), parse("1010"));
        assert_eq!(FourStateValue::zero(4), parse("0000"));
    }

    // ========================================================================
    // The inline / heap representation boundary
    // ========================================================================
    //
    // A value of at most `INLINE_VALUE_BITS` carries its planes by value and a
    // wider one carries them on the heap. Nothing above this line knows that,
    // and these tests are what keeps it that way: every width either side of
    // the cut has to answer exactly what the bit-at-a-time construction does.

    /// Build a value one bit at a time, which is the representation-agnostic
    /// reference the word-level constructors have to agree with.
    fn by_bit(width: u32, mut bit_at: impl FnMut(u32) -> FourStateBit) -> FourStateValue {
        let mut value = FourStateValue::zero(width);
        for index in 0..width {
            value.set_bit(index, bit_at(index));
        }
        value
    }

    /// Every state, at every width around the boundary, read back exactly.
    #[test]
    fn every_width_across_the_inline_boundary_round_trips_every_bit() {
        for width in [0, 1, 31, 32, 33, 63, 64, 65, 96, 128, 129] {
            for bit in TABLE_ORDER {
                let splatted = FourStateValue::splat(width, bit);
                assert_eq!(splatted.width(), width);
                assert_eq!(splatted, by_bit(width, |_| bit), "splat {width} {bit:?}");
                for index in 0..width {
                    assert_eq!(splatted.bit(index), bit, "{width} bit {index} of {bit:?}");
                }
                // The plane words a caller sees are exactly the occupied ones,
                // whichever representation carries them.
                let words = FourStateValue::words_for(width);
                assert_eq!(splatted.aval().len(), words);
                assert_eq!(splatted.bval().len(), words);
            }

            // A pattern that touches every word and every state.
            let mixed = by_bit(width, |index| TABLE_ORDER[(index % 4) as usize]);
            assert_eq!(mixed.width(), width);
            for index in 0..width {
                assert_eq!(mixed.bit(index), TABLE_ORDER[(index % 4) as usize]);
            }
            assert_eq!(mixed.spelling().len(), width as usize);
            assert_eq!(
                FourStateValue::from_bits_msb_first(&mixed.bits_msb_first()),
                mixed
            );
        }
    }

    /// The masking invariant: no bit above `width` is ever set, so a value
    /// built by a word-level constructor equals one built bit by bit, and
    /// `has_unknown` cannot be tripped by a spare bit of an inline plane.
    #[test]
    fn no_representation_carries_a_bit_above_its_width() {
        for width in [0, 1, 7, 31, 32, 33, 63, 64, 65, 100] {
            for bit in TABLE_ORDER {
                let value = FourStateValue::splat(width, bit);
                let words = FourStateValue::words_for(width);
                let remainder = width % PLANE_WORD_BITS;
                if remainder != 0 {
                    let spare = !((1u32 << remainder) - 1);
                    assert_eq!(value.aval()[words - 1] & spare, 0, "{width} {bit:?}");
                    assert_eq!(value.bval()[words - 1] & spare, 0, "{width} {bit:?}");
                }
                let unknown =
                    matches!(bit, FourStateBit::Unknown | FourStateBit::HighImpedance) && width > 0;
                assert_eq!(value.has_unknown(), unknown, "{width} {bit:?}");
            }
        }
    }

    /// `to_u64` refuses above 64 bits and above an unknown bit, and otherwise
    /// reads the same number the bit loop would.
    #[test]
    fn integer_conversion_holds_across_the_inline_boundary() {
        assert_eq!(
            FourStateValue::from_u64(64, u64::MAX).to_u64(),
            Some(u64::MAX)
        );
        assert_eq!(
            FourStateValue::from_u64(63, u64::MAX).to_u64(),
            Some(u64::MAX >> 1)
        );
        assert_eq!(FourStateValue::zero(65).to_u64(), None);
        assert_eq!(FourStateValue::zero(0).to_u64(), Some(0));
        assert_eq!(FourStateValue::from_u64(64, 1 << 63).sign_bit(), ONE_BIT);
        assert_eq!(
            FourStateValue::from_u64(64, 1 << 63).to_integer(true),
            Some(-(1i128 << 63))
        );
        let mut unknown = FourStateValue::from_u64(64, 0);
        unknown.set_bit(63, FourStateBit::Unknown);
        assert_eq!(unknown.to_u64(), None);
    }

    /// Resizing over the boundary in both directions, which is the one
    /// operation that changes a value's representation.
    #[test]
    fn resizing_crosses_the_boundary_in_both_directions() {
        let narrow = parse("1x0z");
        let widened = narrow.resized(96);
        assert_eq!(widened.width(), 96);
        assert_eq!(widened.spelling(), format!("{}1x0z", "0".repeat(92)));
        assert_eq!(widened.resized(4), narrow);

        let wide = FourStateValue::splat(96, FourStateBit::One);
        assert_eq!(wide.resized(4).spelling(), "1111");
        assert_eq!(wide.resized(64).to_u64(), Some(u64::MAX));
        assert_eq!(wide.extended(128, true).spelling(), "1".repeat(128));
        assert_eq!(wide.resized(96), wide);
    }

    /// The serialized shape does not know about the representation split, so a
    /// value written on one side of the boundary reads back on either.
    #[test]
    fn serialization_round_trips_on_both_sides_of_the_boundary() {
        for width in [0u32, 1, 32, 63, 64, 65, 129] {
            let value = by_bit(width, |index| TABLE_ORDER[(index % 4) as usize]);
            let text = serde_json::to_string(&value).expect("serialize");
            assert!(
                text.contains("\"width\"")
                    && text.contains("\"aval\"")
                    && text.contains("\"bval\""),
                "{text}"
            );
            let back: FourStateValue = serde_json::from_str(&text).expect("deserialize");
            assert_eq!(back, value, "width {width}");
        }
    }

    const ONE_BIT: FourStateBit = FourStateBit::One;
}
