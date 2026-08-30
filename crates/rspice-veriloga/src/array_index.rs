//! Shared checked conversion for runtime array indices.
//!
//! Every executable backend must agree on rounding, representability, bounds,
//! and address arithmetic. Keeping this contract independent of a backend's
//! error type prevents malformed programs and non-finite model values from
//! turning into saturating casts, overflow, or divergent behavior.

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ArrayIndexError {
    NonFinite { raw: f64 },
    RoundedOutOfRange { raw: f64 },
    Empty,
    OutOfBounds { index: i64 },
    SlotOverflow,
}

pub(crate) fn checked_rounded_i64(value: f64) -> Result<i64, ArrayIndexError> {
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

pub(crate) fn checked_array_slot(
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

pub(crate) fn saturated_array_upper(lower: i64, len: usize) -> i64 {
    if len == 0 {
        return lower.saturating_sub(1);
    }

    let upper = i128::from(lower) + len as i128 - 1;
    upper.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
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
