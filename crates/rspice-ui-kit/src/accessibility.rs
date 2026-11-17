//! Shared text helpers for UI copy and assistive-technology contracts.

/// Format a count with correct singular/plural grammar.
pub fn counted(count: usize, singular: &str, plural: &str) -> String {
    format!("{count} {}", if count == 1 { singular } else { plural })
}

/// A count of findings as every place that states one says it: errors
/// first, then advisories, both always named. An error stops the design
/// working; every other current finding is an advisory.
pub fn finding_counts(errors: usize, advisories: usize) -> String {
    format!(
        "{} \u{b7} {}",
        counted(errors, "error", "errors"),
        counted(advisories, "advisory", "advisories")
    )
}

/// The bare pluralizing suffix for a count, for copy that inflects a regular
/// noun inside a larger sentence. An irregular noun takes [`counted`].
pub const fn plural_suffix(count: usize) -> &'static str {
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
    fn finding_counts_name_both_counts_in_their_number() {
        assert_eq!(finding_counts(1, 1), "1 error \u{b7} 1 advisory");
        assert_eq!(finding_counts(0, 2), "0 errors \u{b7} 2 advisories");
        assert_eq!(finding_counts(3, 0), "3 errors \u{b7} 0 advisories");
    }

    #[test]
    fn plural_suffix_inflects_everything_but_exactly_one() {
        assert_eq!(plural_suffix(0), "s");
        assert_eq!(plural_suffix(1), "");
        assert_eq!(plural_suffix(2), "s");
    }
}
