/// MSRV-safe replacement for `usize::is_multiple_of`.
#[inline]
pub(crate) fn is_multiple_of(value: usize, divisor: usize) -> bool {
    matches!(value.checked_rem(divisor), Some(0))
}

