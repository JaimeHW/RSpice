//! Shared checked conversion for runtime array indices.
//!
//! Every executable backend must agree on rounding, representability, bounds,
//! and address arithmetic. Keeping this contract independent of a backend's
//! error type prevents malformed programs and non-finite model values from
//! turning into saturating casts, overflow, or divergent behavior.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrayIndexError {
    NonFinite { raw: f64 },
    RoundedOutOfRange { raw: f64 },
    Empty,
    OutOfBounds { index: i64 },
    SlotOverflow,
}

pub fn checked_rounded_i64(value: f64) -> Result<i64, ArrayIndexError> {
    const I64_MAX_EXCLUSIVE_AS_F64: f64 = 9_223_372_036_854_775_808.0;

    if !value.is_finite() {
        return Err(ArrayIndexError::NonFinite { raw: value });
    }

    let rounded = value.round();
    if rounded < i64::MIN as f64 || rounded >= I64_MAX_EXCLUSIVE_AS_F64 {
        return Err(ArrayIndexError::RoundedOutOfRange { raw: value });
    }

    Ok(rounded as i64)
}

pub fn checked_array_slot(
    raw_index: f64,
    base: usize,
    len: usize,
    lower: i64,
) -> Result<usize, ArrayIndexError> {
    if len == 0 {
        return Err(ArrayIndexError::Empty);
    }

    let index = checked_rounded_i64(raw_index)?;
    let offset = i128::from(index) - i128::from(lower);
    if offset < 0
        || u128::try_from(offset)
            .ok()
            .is_none_or(|offset| offset >= len as u128)
    {
        return Err(ArrayIndexError::OutOfBounds { index });
    }

    let offset = usize::try_from(offset).map_err(|_| ArrayIndexError::SlotOverflow)?;
    base.checked_add(offset)
        .ok_or(ArrayIndexError::SlotOverflow)
}

pub fn saturated_array_upper(lower: i64, len: usize) -> i64 {
    if len == 0 {
        return lower.saturating_sub(1);
    }

    let upper = i128::from(lower) + len as i128 - 1;
    upper.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
}

/// One positive i32 holds a chunk's value bits and known-bit mask exactly.
/// X and Z are both unavailable when converted to an analog numeric bit.
pub const PACKED_CHUNK_BITS: u32 = 15;

/// Source bounds behind a flattened word-major bank of encoded packed chunks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PackedArrayLayout {
    pub word_lower: i64,
    pub word_len: u32,
    pub packed_msb: i64,
    pub packed_lsb: i64,
}

impl PackedArrayLayout {
    pub fn width(self) -> Option<u32> {
        self.packed_msb
            .abs_diff(self.packed_lsb)
            .checked_add(1)
            .and_then(|width| u32::try_from(width).ok())
            .filter(|width| *width <= 65_536)
    }

    pub fn chunks_per_word(self) -> Option<u32> {
        self.width().map(|width| width.div_ceil(PACKED_CHUNK_BITS))
    }

    pub fn chunk_len(self) -> Option<usize> {
        if self.word_len == 0
            || self.word_len > 65_536
            || self
                .word_lower
                .checked_add(i64::from(self.word_len) - 1)
                .is_none()
        {
            return None;
        }
        (self.chunks_per_word()? as usize).checked_mul(self.word_len as usize)
    }

    /// Validate each authored coordinate before combining it: an out-of-range
    /// packed bit must never wrap into a neighboring word's chunk.
    pub fn locate(self, word: f64, bit: f64) -> Result<(usize, u32), ArrayIndexError> {
        let width = self.width().ok_or(ArrayIndexError::SlotOverflow)?;
        let len = self.chunk_len().ok_or(ArrayIndexError::SlotOverflow)?;
        let word = checked_array_slot(word, 0, self.word_len as usize, self.word_lower)?;
        let bit = checked_array_slot(bit, 0, width as usize, self.packed_msb.min(self.packed_lsb))?;
        let bit = if self.packed_msb < self.packed_lsb {
            width as usize - 1 - bit
        } else {
            bit
        };
        let chunk = word
            .checked_mul(self.chunks_per_word().unwrap() as usize)
            .and_then(|base| base.checked_add(bit / PACKED_CHUNK_BITS as usize))
            .filter(|chunk| *chunk < len)
            .ok_or(ArrayIndexError::SlotOverflow)?;
        Ok((chunk, bit as u32 % PACKED_CHUNK_BITS))
    }
}

/// Decode only the selected bit. The encoded chunk is always finite; availability
/// belongs to its selected mask bit, not to other bits in the same chunk.
pub fn decode_packed_bit(encoded: f64, bit: u32) -> Result<f64, &'static str> {
    if bit >= PACKED_CHUNK_BITS
        || !encoded.is_finite()
        || encoded.fract() != 0.0
        || !(0.0..f64::from(1u32 << (2 * PACKED_CHUNK_BITS))).contains(&encoded)
    {
        return Err("invalid encoded packed discrete input");
    }
    let encoded = encoded as u32;
    let known = (encoded >> (PACKED_CHUNK_BITS + bit)) & 1;
    let value = (encoded >> bit) & 1;
    crate::checked_discrete_value(f64::from(known), f64::from(value))
}

#[cfg(test)]
mod tests {
    use super::{ArrayIndexError, checked_array_slot, checked_rounded_i64};

    #[test]
    fn checked_array_slot_rounds_once_and_checks_declared_bounds() {
        assert_eq!(checked_array_slot(3.49, 10, 3, 2), Ok(11));
        assert_eq!(
            checked_array_slot(4.5, 10, 3, 2),
            Err(ArrayIndexError::OutOfBounds { index: 5 })
        );
        assert_eq!(
            checked_array_slot(1.49, 10, 3, 2),
            Err(ArrayIndexError::OutOfBounds { index: 1 })
        );
    }

    #[test]
    fn checked_array_slot_rejects_nonfinite_and_unrepresentable_indices() {
        assert!(matches!(
            checked_array_slot(f64::NAN, 0, 1, 0),
            Err(ArrayIndexError::NonFinite { .. })
        ));
        assert!(matches!(
            checked_array_slot(f64::INFINITY, 0, 1, 0),
            Err(ArrayIndexError::NonFinite { .. })
        ));
        assert!(matches!(
            checked_rounded_i64(9_223_372_036_854_775_808.0),
            Err(ArrayIndexError::RoundedOutOfRange { .. })
        ));
    }

    #[test]
    fn checked_array_slot_rejects_empty_and_overflowing_layouts() {
        assert_eq!(
            checked_array_slot(0.0, 0, 0, 0),
            Err(ArrayIndexError::Empty)
        );
        assert_eq!(
            checked_array_slot(1.0, usize::MAX, 2, 0),
            Err(ArrayIndexError::SlotOverflow)
        );
    }
}
