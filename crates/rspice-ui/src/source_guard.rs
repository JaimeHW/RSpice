//! Finding the production half of a source file that inspects itself.
//!
//! Several modules assert that their own shipped code contains no panic
//! shortcuts, by reading themselves back through `include_str!` and scanning
//! everything that precedes their test module.
//!
//! One guard here reads the whole crate rather than one file:
//! [`tests::no_shipped_code_names_a_design_reference_fixture`] bans the
//! design-reference mockup's circuit vocabulary from everything `rspice-ui`
//! ships. That scan needs a view of a file with its test-only items removed
//! wherever they sit, not just a boundary to stop at, which is what
//! [`without_test_items`] provides.

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

/// Everything a source file ships, whichever of the two shapes it has.
///
/// Splitting a module leaves submodules that carry no tests of their own, and
/// a guard has to be able to scan those too. A file with no `#[cfg(test)]`
/// anywhere is production in full. A file that has test-only code still must
/// declare the `mod tests` boundary, so the ambiguity [`production_half`]
/// panics on -- a boundary that stopped matching -- stays a red test here as
/// well. What is never allowed is scanning a file that mixes the two without
/// saying where the seam is.
pub(crate) fn production_source(source: &str) -> &str {
    if source.contains("#[cfg(test)]") {
        production_half(source)
    } else {
        source
    }
}

/// `source` with every `#[cfg(test)]` item blanked, line for line.
///
/// [`production_source`] answers "where does the shipped half stop", which
/// only works for a file whose test-only code is one trailing module. A crate
/// carries the other shapes too: a `#[cfg(test)] pub use` re-export halfway
/// down, a `#[cfg(test)] fn` fixture between two shipped functions, several
/// test modules under names of their own. Cutting at the first of those would
/// leave everything after it unscanned, and a guard that scans nothing passes
/// forever.
///
/// So each attributed item is removed where it sits and the rest is kept. The
/// removed lines become empty ones rather than disappearing, because the
/// caller reports the line a match sits on and those numbers have to be the
/// numbers in the file.
///
/// The item's extent is read the way `rustfmt` writes it: an item that ends in
/// `;` is one statement, and a braced item ends at the first `}` indented
/// exactly as far as its attribute. An item whose end is never found is a
/// panic, not a silent skip to end of file.
pub(crate) fn without_test_items(source: &str) -> String {
    const ATTRIBUTE: &str = "#[cfg(test)]";
    // `str::lines` drops the `\r` of a CRLF checkout, so nothing here depends
    // on which line ending the working copy was written with.
    let lines = source.lines().collect::<Vec<_>>();
    let mut kept = Vec::with_capacity(lines.len());
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        let Some(rest) = line.trim_start().strip_prefix(ATTRIBUTE) else {
            kept.push(line);
            index += 1;
            continue;
        };
        let indent = line.len() - line.trim_start().len();
        kept.push("");
        index += 1;
        let mut item = rest.trim();
        let mut opened = false;
        let mut closed = false;
        loop {
            let trimmed = item.trim_end();
            if opened {
                // The closer of a braced item is the only `}` written at the
                // attribute's own indentation; everything the item contains is
                // indented further. Reading the end that way rather than by
                // counting braces keeps a brace inside a fixture string from
                // moving it.
                closed = trimmed.trim_start().starts_with('}')
                    && item.len() - item.trim_start().len() == indent;
            } else {
                let depth =
                    trimmed.matches('{').count() as isize - trimmed.matches('}').count() as isize;
                if depth > 0 {
                    opened = true;
                } else {
                    // Balanced braces on the first line finish the item here —
                    // a one-line `fn`, or a `use` naming a brace list. So does
                    // a statement, an out-of-line `mod`, or an attributed field
                    // or variant of the enclosing item.
                    closed =
                        trimmed.contains('{') || trimmed.ends_with(';') || trimmed.ends_with(',');
                }
            }
            if closed || index >= lines.len() {
                break;
            }
            item = lines[index];
            kept.push("");
            index += 1;
        }
        assert!(
            closed,
            "a `{ATTRIBUTE}` item runs to the end of the file with no closing line; \
             the scan would silently stop here"
        );
    }
    kept.join("\n")
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

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

    /// A split submodule carrying no tests is scanned whole rather than
    /// refused, which is what lets a guard keep covering a module after it
    /// grows children.
    #[test]
    fn a_source_with_no_test_only_code_at_all_is_production_in_full() {
        assert_eq!(production_source("fn shipped() {}\n"), "fn shipped() {}\n");
    }

    /// Test-only code without the `mod tests` boundary is still the ambiguous
    /// case, and still has to fail rather than be scanned as shipped code.
    #[test]
    #[should_panic(expected = "no `#[cfg(test)] mod tests` block")]
    fn test_only_code_without_the_boundary_is_still_refused() {
        let _ = production_source("#[cfg(test)]\nfn fixture() {}\nfn shipped() {}\n");
    }

    /// Every shape a `#[cfg(test)]` item takes in this crate, and the one
    /// property the crate-wide scan depends on: the shipped lines keep their
    /// line numbers.
    #[test]
    fn test_only_items_are_removed_in_place_under_either_line_ending() {
        let unix = concat!(
            "fn first() {}\n",
            "#[cfg(test)]\n",
            "pub use inner::{One, Two};\n",
            "fn second() {}\n",
            "#[cfg(test)]\n",
            "mod fixtures;\n",
            "#[cfg(test)]\n",
            "fn fixture() {\n",
            "    let brace = \"}\";\n",
            "}\n",
            "fn third() {}\n",
        );
        let expected = concat!(
            "fn first() {}\n",
            "\n\n",
            "fn second() {}\n",
            "\n\n",
            "\n\n\n\n",
            "fn third() {}",
        );
        assert_eq!(without_test_items(unix), expected);
        assert_eq!(without_test_items(&unix.replace('\n', "\r\n")), expected);
        assert_eq!(
            without_test_items(unix).lines().count(),
            unix.lines().count(),
            "shipped lines keep the line numbers the failure message reports"
        );
    }

    /// An item whose end is never found must be a red test rather than a scan
    /// that quietly stops at the first attribute it meets.
    #[test]
    #[should_panic(expected = "runs to the end of the file")]
    fn an_unterminated_test_item_is_refused() {
        let _ = without_test_items("#[cfg(test)]\nmod fixtures {\n    fn t() {}\n");
    }

    /// The design-reference mockup's own circuit, spelled in halves.
    ///
    /// Writing these names out here would put them in a file the scan reads,
    /// and a guard that matches itself is permanently red for no reason.
    fn design_fixture_names() -> Vec<String> {
        [
            ("afe", "_out"),
            ("VIN", "_DIFF"),
            ("IIN", "_CAL"),
            ("sensor", "_p"),
            ("sensor", "_n"),
            ("filt", "_out"),
            ("BRIDGE", "_EXC"),
            ("VCM", "_REF"),
            ("RGA", "IN"),
            ("XBRI", "DGE"),
            ("u1", "_sum"),
        ]
        .iter()
        .map(|(head, tail)| format!("{head}{tail}").to_ascii_lowercase())
        .collect()
    }

    /// Whether `haystack` names `needle` as a whole identifier.
    ///
    /// Both are already lowercase. The boundary matters: one of these names is
    /// a substring of an ordinary English word, and a bare `contains` would
    /// fail the build on prose.
    fn names_identifier(haystack: &str, needle: &str) -> bool {
        let identifier = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_';
        haystack.match_indices(needle).any(|(start, _)| {
            let before = start.checked_sub(1).map(|index| haystack.as_bytes()[index]);
            let after = haystack.as_bytes().get(start + needle.len()).copied();
            !before.is_some_and(identifier) && !after.is_some_and(identifier)
        })
    }

    /// Whether this path is a file the crate ships.
    ///
    /// Two things say a file is test-only: its own name, and the parent module
    /// that declared it under `#[cfg(test)]`. `test_only_roots` carries the
    /// second, as path prefixes.
    fn ships(path: &Path, test_only_roots: &[PathBuf]) -> bool {
        let named_tests = path.components().any(|component| {
            component
                .as_os_str()
                .to_str()
                .is_some_and(|part| part == "tests")
        });
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("");
        !named_tests
            && stem != "tests"
            && !stem.ends_with("_tests")
            && !test_only_roots
                .iter()
                .any(|root| path == root.with_extension("rs") || path.starts_with(root))
    }

    /// Where the `#[cfg(test)] mod <name>;` children of `path` would live.
    fn test_only_roots(path: &Path, source: &str) -> Vec<PathBuf> {
        const ATTRIBUTE: &str = "#[cfg(test)]";
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("");
        let directory = match stem {
            "lib" | "mod" => path.parent().map(Path::to_path_buf),
            _ => path.parent().map(|parent| parent.join(stem)),
        };
        let Some(directory) = directory else {
            return Vec::new();
        };
        source
            .match_indices(ATTRIBUTE)
            .filter_map(|(index, _)| {
                let rest = source[index + ATTRIBUTE.len()..].trim_start();
                let declaration = rest.strip_prefix("mod ")?;
                let name = declaration.split(';').next()?.trim();
                (!name.is_empty()
                    && name
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_'))
                .then(|| directory.join(name))
            })
            .collect()
    }

    fn rust_sources(directory: &Path, out: &mut Vec<PathBuf>) {
        let entries = std::fs::read_dir(directory)
            .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()));
        for entry in entries {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                rust_sources(&path, out);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                out.push(path);
            }
        }
    }

    /// Nothing this crate ships names a circuit out of the design-reference
    /// mockup.
    ///
    /// The mockup's fixture circuit is a drawing, not a design anyone here has
    /// opened. Shipping one of its net or instance names as a form preset, a
    /// draft default, or a field hint claims knowledge of the user's circuit
    /// that the studio does not have: the name goes into a request, and the run
    /// fails at engine validation against a design that never had that node.
    ///
    /// Test fixtures are free to name it — a fixture is allowed to be a
    /// fixture. So the scan covers `src/` with the test-only files and the
    /// `#[cfg(test)]` items inside the remaining ones taken out.
    #[test]
    fn no_shipped_code_names_a_design_reference_fixture() {
        let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut paths = Vec::new();
        rust_sources(&source_root, &mut paths);
        paths.sort();
        assert!(
            paths.len() > 100,
            "the crate-wide scan found only {} files under {}; a scan that reaches \
             nothing passes forever",
            paths.len(),
            source_root.display()
        );

        let sources = paths
            .iter()
            .map(|path| {
                let source = std::fs::read_to_string(path)
                    .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
                (path.clone(), source)
            })
            .collect::<Vec<_>>();
        let roots = sources
            .iter()
            .flat_map(|(path, source)| test_only_roots(path, source))
            .collect::<Vec<_>>();

        let names = design_fixture_names();
        let mut found = Vec::new();
        for (path, source) in &sources {
            if !ships(path, &roots) {
                continue;
            }
            for (number, line) in without_test_items(source).lines().enumerate() {
                let lowered = line.to_ascii_lowercase();
                for name in &names {
                    if names_identifier(&lowered, name) {
                        found.push(format!(
                            "{}:{}: shipped code names the design-reference fixture \
                             `{name}`",
                            path.display(),
                            number + 1,
                        ));
                    }
                }
            }
        }

        assert!(
            found.is_empty(),
            "shipped code must not name a circuit out of the design-reference mockup \
             — derive the value from the elaborated design, or leave it unset:\n  {}",
            found.join("\n  ")
        );
    }
}
