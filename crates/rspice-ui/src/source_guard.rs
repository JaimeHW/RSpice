//! Finding the production half of a source file that inspects itself.
//!
//! Several modules assert that their own shipped code contains no panic
//! shortcuts, by reading themselves back through `include_str!` and scanning
//! everything that precedes their test module.

/// Everything in `source` that precedes its `#[cfg(test)] mod tests` block.
///
/// Spelling that boundary as a literal `"\n#[cfg(test)]\nmod tests {"` ties the
/// scan to the file's line endings. The same source checked out with CRLF never
/// matches, the split hands back the whole file, and the guard fails on the
/// assertions doing the scanning rather than on anything shipped.
///
/// The quieter failure is the one worth designing against. A boundary that
/// stops matching for any other reason -- a reordered attribute, a renamed
/// module, a blank line where the pattern wanted none -- leaves a guard
/// scanning a truncated region, and a guard that scans nothing passes forever.
/// So this locates the boundary structurally and panics when there is none: a
/// guard with nothing left to guard should be a red test, not a green one.
pub(crate) fn production_half(source: &str) -> &str {
    const ATTRIBUTE: &str = "#[cfg(test)]";
    for (index, _) in source.match_indices(ATTRIBUTE) {
        if source[index + ATTRIBUTE.len()..]
            .trim_start()
            .starts_with("mod tests")
        {
            return &source[..index];
        }
    }
    panic!("the scanned source declares no `#[cfg(test)] mod tests` block to stop at")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The bug this module exists for: the boundary has to be found whichever
    /// way the working copy checked out.
    #[test]
    fn the_boundary_is_found_under_either_line_ending() {
        let unix = "fn shipped() {}\n\n#[cfg(test)]\nmod tests {\n    fn t() {}\n}\n";
        let windows = unix.replace('\n', "\r\n");
        assert_eq!(production_half(unix), "fn shipped() {}\n\n");
        assert_eq!(production_half(&windows), "fn shipped() {}\r\n\r\n");
    }

    /// An earlier `#[cfg(test)]` on some other item is not the boundary, and
    /// stopping at one would hide every shortcut between it and the real one.
    #[test]
    fn a_test_only_item_before_the_module_is_not_mistaken_for_the_boundary() {
        let source = "#[cfg(test)]\nfn fixture() {}\nfn shipped() {}\n#[cfg(test)]\nmod tests {}\n";
        assert_eq!(
            production_half(source),
            "#[cfg(test)]\nfn fixture() {}\nfn shipped() {}\n"
        );
    }

    /// A test module in its own file still declares the boundary.
    #[test]
    fn an_out_of_line_test_module_is_a_boundary() {
        assert_eq!(
            production_half("fn shipped() {}\n#[cfg(test)]\nmod tests;\n"),
            "fn shipped() {}\n"
        );
    }

    #[test]
    #[should_panic(expected = "no `#[cfg(test)] mod tests` block")]
    fn a_source_with_no_test_module_is_a_failure_and_not_an_empty_scan() {
        let _ = production_half("fn shipped() {}\n");
    }
}
