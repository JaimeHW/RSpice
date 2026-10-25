//! Finding the production half of a source file that inspects itself.
//!
//! The run controller asserts that its own shipped code fails a run through
//! one sealing helper only, by reading itself back through `include_str!` and
//! scanning everything that precedes its test module. This is the run tier's
//! own copy of `rspice-app`'s rule: a crate cannot reach another's test-only
//! code, and a shared dev crate would cost a lock change for one function.

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
    let boundary = TEST_ONLY
        .iter()
        .flat_map(|attribute| {
            source
                .match_indices(attribute)
                .map(|(index, _)| (index, index + attribute.len()))
        })
        .filter(|&(_, item)| declares_the_test_module(&source[item..]))
        .map(|(index, _)| index)
        .min();
    match boundary {
        Some(index) => &source[..index],
        None => panic!("the scanned source declares no `#[cfg(test)] mod tests` block to stop at"),
    }
}

/// Whether the item after a test-only attribute is the `tests` module. A test
/// module whose fixtures other modules' tests build from is visible beyond its
/// parent; it is the same boundary.
fn declares_the_test_module(item: &str) -> bool {
    let item = item.trim_start();
    let item = ["pub(crate)", "pub"]
        .iter()
        .find_map(|visibility| item.strip_prefix(visibility))
        .map_or(item, str::trim_start);
    item.starts_with("mod tests")
}

/// The attributes a test module is compiled under: in the crate's own tests
/// only, or also for the application's tests through the `test-support`
/// feature.
const TEST_ONLY: [&str; 2] = [
    "#[cfg(test)]",
    "#[cfg(any(test, feature = \"test-support\"))]",
];

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

    /// A test module other modules' tests import fixtures from is a boundary,
    /// whether the crate's own tests or the application's import them.
    #[test]
    fn a_test_module_with_fixtures_is_a_boundary_under_either_attribute() {
        assert_eq!(
            production_half("fn shipped() {}\n#[cfg(test)]\npub(crate) mod tests;\n"),
            "fn shipped() {}\n"
        );
        assert_eq!(
            production_half(
                "fn shipped() {}\n#[cfg(any(test, feature = \"test-support\"))]\npub mod tests;\n"
            ),
            "fn shipped() {}\n"
        );
    }

    #[test]
    #[should_panic(expected = "no `#[cfg(test)] mod tests` block")]
    fn a_source_with_no_test_module_is_a_failure_and_not_an_empty_scan() {
        let _ = production_half("fn shipped() {}\n");
    }
}
