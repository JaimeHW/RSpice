//! Shared text helpers for UI copy and assistive-technology contracts.

/// Format a count with correct singular/plural grammar.
pub(crate) fn counted(count: usize, singular: &str, plural: &str) -> String {
    format!("{count} {}", if count == 1 { singular } else { plural })
}

/// The bare pluralizing suffix for a count, for copy that inflects a regular
/// noun inside a larger sentence. An irregular noun takes [`counted`].
pub(crate) const fn plural_suffix(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counted_uses_professional_singular_and_plural_grammar() {
        assert_eq!(counted(0, "trace", "traces"), "0 traces");
        assert_eq!(counted(1, "trace", "traces"), "1 trace");
        assert_eq!(counted(2, "trace", "traces"), "2 traces");
    }

    #[test]
    fn plural_suffix_inflects_everything_but_exactly_one() {
        assert_eq!(plural_suffix(0), "s");
        assert_eq!(plural_suffix(1), "");
        assert_eq!(plural_suffix(2), "s");
    }
}
