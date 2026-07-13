//! Shared text and interaction helpers for assistive-technology contracts.

/// Format a count with correct singular/plural grammar.
pub(crate) fn counted(count: usize, singular: &str, plural: &str) -> String {
    format!("{count} {}", if count == 1 { singular } else { plural })
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
}
