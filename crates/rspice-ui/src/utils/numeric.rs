/// MSRV-safe replacement for `usize::is_multiple_of`.
#[inline]
pub(crate) fn is_multiple_of(value: usize, divisor: usize) -> bool {
    matches!(value.checked_rem(divisor), Some(0))
}

#[cfg(test)]
mod tests {
    use super::is_multiple_of;

    #[test]
    fn test_is_multiple_of_detects_multiples() {
        assert!(is_multiple_of(12, 3));
        assert!(is_multiple_of(0, 5));
    }

    #[test]
    fn test_is_multiple_of_rejects_non_multiples_and_zero_divisor() {
        assert!(!is_multiple_of(12, 5));
        assert!(!is_multiple_of(12, 0));
    }
}
