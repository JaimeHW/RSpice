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

    checked_integer_array_slot(checked_rounded_i64(raw_index)?, base, len, lower)
}

/// Integer selectors must not pass through floating point before addressing
/// storage: neighboring indices beyond 2^53 can have the same f64 value.
pub fn checked_integer_array_slot(
    index: i64,
    base: usize,
    len: usize,
    lower: i64,
) -> Result<usize, ArrayIndexError> {
    if len == 0 {
        return Err(ArrayIndexError::Empty);
    }
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

/// Storage for an unpacked array. The rightmost dimension varies fastest;
/// physical storage uses increasing indices in each dimension. Authored
/// directions are retained separately for initialization and source readback.
/// Construction validates the complete shape before allocating element storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnpackedArrayLayout {
    axes: Vec<UnpackedArrayAxis>,
    len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnpackedArrayAxis {
    pub left: i64,
    pub right: i64,
    len: usize,
    stride: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayShapeError {
    Empty,
    ElementLimit { limit: usize },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrayCoordinateError {
    Rank { expected: usize, actual: usize },
    Axis { axis: usize, error: ArrayIndexError },
    SlotOverflow,
}

impl UnpackedArrayLayout {
    pub fn new(bounds: &[(i64, i64)], element_limit: usize) -> Result<Self, ArrayShapeError> {
        if bounds.is_empty() {
            return Err(ArrayShapeError::Empty);
        }
        let mut len = 1usize;
        // Validate without allocating a partial shape. u128 covers even the
        // full signed i64 index domain (2^64 elements) on every target.
        for &(left, right) in bounds {
            let extent = u128::from(left.abs_diff(right)) + 1;
            if extent > element_limit as u128 || len > element_limit / extent as usize {
                return Err(ArrayShapeError::ElementLimit {
                    limit: element_limit,
                });
            }
            len *= extent as usize;
        }
        let mut stride = len;
        let axes = bounds
            .iter()
            .map(|&(left, right)| {
                let len = (u128::from(left.abs_diff(right)) + 1) as usize;
                stride /= len;
                UnpackedArrayAxis {
                    left,
                    right,
                    len,
                    stride,
                }
            })
            .collect();
        Ok(Self { axes, len })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn axes(&self) -> &[UnpackedArrayAxis] {
        &self.axes
    }

    pub fn slot(&self, indices: &[i64], base: usize) -> Result<usize, ArrayCoordinateError> {
        self.locate(indices.len(), base, |axis| Ok(indices[axis]))
    }

    pub fn real_slot(&self, indices: &[f64], base: usize) -> Result<usize, ArrayCoordinateError> {
        self.locate(indices.len(), base, |axis| {
            checked_rounded_i64(indices[axis])
        })
    }

    fn locate(
        &self,
        rank: usize,
        base: usize,
        mut index: impl FnMut(usize) -> Result<i64, ArrayIndexError>,
    ) -> Result<usize, ArrayCoordinateError> {
        if rank != self.axes.len() {
            return Err(ArrayCoordinateError::Rank {
                expected: self.axes.len(),
                actual: rank,
            });
        }
        let mut offset = 0usize;
        for (axis, shape) in self.axes.iter().enumerate() {
            let error = |error| ArrayCoordinateError::Axis { axis, error };
            let coordinate = checked_integer_array_slot(
                index(axis).map_err(error)?,
                0,
                shape.len,
                shape.left.min(shape.right),
            )
            .map_err(error)?;
            // Every coordinate is independently in bounds. A bad coordinate
            // cannot alias the next row even if its flattened sum would fit.
            offset += coordinate * shape.stride;
        }
        base.checked_add(offset)
            .ok_or(ArrayCoordinateError::SlotOverflow)
    }

    /// Source indices corresponding to a physical slot, for element identity.
    pub fn indices(&self, offset: usize) -> Option<Vec<i64>> {
        (offset < self.len).then(|| {
            self.axes
                .iter()
                .map(|axis| {
                    (i128::from(axis.left.min(axis.right))
                        + ((offset / axis.stride) % axis.len) as i128) as i64
                })
                .collect()
        })
    }

    /// Translate an initializer's declaration-order ordinal into physical
    /// storage. No values move until the caller has validated the full pattern.
    pub fn declaration_slot(&self, ordinal: usize) -> Option<usize> {
        (ordinal < self.len).then(|| {
            self.axes
                .iter()
                .map(|axis| {
                    let position = (ordinal / axis.stride) % axis.len;
                    let increasing = if axis.left <= axis.right {
                        position
                    } else {
                        axis.len - 1 - position
                    };
                    increasing * axis.stride
                })
                .sum()
        })
    }
}

impl UnpackedArrayAxis {
    pub fn len(self) -> usize {
        self.len
    }
    pub fn is_empty(self) -> bool {
        false
    }
    pub fn stride(self) -> usize {
        self.stride
    }
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

/// Validate a numeric read at the discrete-to-continuous boundary.
/// A separate finite validity lane keeps unavailable values out of numeric
/// state/checkpoints. Evaluate this operation only at an executed source read.
#[inline]
pub fn checked_discrete_value(validity: f64, value: f64) -> Result<f64, &'static str> {
    if validity == 0.0 {
        Err("analog read of discrete input has an X, Z, or non-finite value")
    } else if validity != 1.0 || !value.is_finite() {
        Err("analog discrete input has an invalid value/validity encoding")
    } else {
        Ok(value)
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
    checked_discrete_value(f64::from(known), f64::from(value))
}

#[cfg(test)]
mod tests {
    use super::{
        ArrayCoordinateError, ArrayIndexError, ArrayShapeError, UnpackedArrayLayout,
        checked_array_slot, checked_integer_array_slot, checked_rounded_i64,
    };

    #[test]
    fn multidimensional_layout_keeps_storage_and_declaration_order_distinct() {
        let shape = UnpackedArrayLayout::new(&[(2, 1), (-2, 0), (8, 7)], 65_536).unwrap();
        assert_eq!(shape.len(), 12);
        assert_eq!(
            shape
                .axes()
                .iter()
                .map(|axis| axis.stride())
                .collect::<Vec<_>>(),
            [6, 2, 1]
        );
        let authored = [
            [2, -2, 8],
            [2, -2, 7],
            [2, -1, 8],
            [2, -1, 7],
            [2, 0, 8],
            [2, 0, 7],
            [1, -2, 8],
            [1, -2, 7],
            [1, -1, 8],
            [1, -1, 7],
            [1, 0, 8],
            [1, 0, 7],
        ];
        let physical = [7, 6, 9, 8, 11, 10, 1, 0, 3, 2, 5, 4];
        for (ordinal, (indices, slot)) in authored.iter().zip(physical).enumerate() {
            assert_eq!(shape.declaration_slot(ordinal), Some(slot));
            assert_eq!(shape.indices(slot).as_deref(), Some(indices.as_slice()));
            assert_eq!(shape.slot(indices, 17), Ok(17 + slot));
        }
        assert!(shape.indices(12).is_none());
        assert!(shape.declaration_slot(12).is_none());
    }

    #[test]
    fn multidimensional_coordinates_cannot_alias_another_row() {
        let shape = UnpackedArrayLayout::new(&[(0, 1), (0, 2)], 6).unwrap();
        for (indices, axis, index) in [([0, 3], 1, 3), ([1, -1], 1, -1), ([2, 0], 0, 2)] {
            assert_eq!(
                shape.slot(&indices, 0),
                Err(ArrayCoordinateError::Axis {
                    axis,
                    error: ArrayIndexError::OutOfBounds { index }
                })
            );
        }
        assert_eq!(
            shape.slot(&[0], 0),
            Err(ArrayCoordinateError::Rank {
                expected: 2,
                actual: 1
            })
        );
        assert_eq!(
            shape.slot(&[0, 0, 0], 0),
            Err(ArrayCoordinateError::Rank {
                expected: 2,
                actual: 3
            })
        );
        assert_eq!(shape.real_slot(&[0.49, 1.5], 0), Ok(2));
        assert!(matches!(
            shape.real_slot(&[0.0, f64::NAN], 0),
            Err(ArrayCoordinateError::Axis {
                axis: 1,
                error: ArrayIndexError::NonFinite { .. }
            })
        ));
        assert_eq!(
            shape.slot(&[1, 0], usize::MAX),
            Err(ArrayCoordinateError::SlotOverflow)
        );
    }

    #[test]
    fn multidimensional_layout_preserves_extreme_integer_indices() {
        let shape =
            UnpackedArrayLayout::new(&[(i64::MIN, i64::MIN + 1), (i64::MAX, i64::MAX - 1)], 4)
                .unwrap();
        for (indices, slot) in [
            ([i64::MIN, i64::MAX - 1], 0),
            ([i64::MIN, i64::MAX], 1),
            ([i64::MIN + 1, i64::MAX - 1], 2),
            ([i64::MIN + 1, i64::MAX], 3),
        ] {
            assert_eq!(shape.slot(&indices, 0), Ok(slot));
            assert_eq!(shape.indices(slot).as_deref(), Some(indices.as_slice()));
        }
        let origin = 9_007_199_254_740_992;
        for offset in 0..3 {
            assert_eq!(
                checked_integer_array_slot(origin + offset, 3, 3, origin),
                Ok(3 + offset as usize)
            );
        }
    }

    #[test]
    fn multidimensional_shape_limits_precede_storage_allocation() {
        assert_eq!(
            UnpackedArrayLayout::new(&[], 65_536),
            Err(ArrayShapeError::Empty)
        );
        for bounds in [
            vec![(i64::MIN, i64::MAX)],
            vec![(0, 256), (0, 256)],
            vec![(0, 65535), (0, 1)],
        ] {
            assert_eq!(
                UnpackedArrayLayout::new(&bounds, 65_536),
                Err(ArrayShapeError::ElementLimit { limit: 65_536 })
            );
        }
        assert_eq!(
            UnpackedArrayLayout::new(&[(0, 0)], 0),
            Err(ArrayShapeError::ElementLimit { limit: 0 })
        );
        assert_eq!(
            UnpackedArrayLayout::new(&[(0, 255), (0, 255)], 65_536)
                .unwrap()
                .len(),
            65_536
        );
    }

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
