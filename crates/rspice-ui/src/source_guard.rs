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

/// Whether this path is a file the crate ships.
///
/// Two things say a file is test-only: its own name, and the parent module
/// that declared it under `#[cfg(test)]`. [`test_only_roots`] carries the
/// second, as path prefixes.
#[cfg(test)]
pub(crate) fn ships(path: &std::path::Path, test_only_roots: &[std::path::PathBuf]) -> bool {
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
#[cfg(test)]
pub(crate) fn test_only_roots(path: &std::path::Path, source: &str) -> Vec<std::path::PathBuf> {
    const ATTRIBUTE: &str = "#[cfg(test)]";
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("");
    let directory = match stem {
        "lib" | "mod" => path.parent().map(std::path::Path::to_path_buf),
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
    ///
    /// The last four are the rest of that same vocabulary rather than a second
    /// list: a supply rail, a differential output leg, the common-mode
    /// reference on its own, and a converter input. None of them was shipped
    /// anywhere when they were added, and that is the point — the ban is what
    /// keeps them from becoming a form preset or a field hint the way the
    /// eleven above nearly did. `vcm` and `vcm_ref` are separate entries
    /// deliberately: the match is on whole identifiers, so banning one says
    /// nothing at all about the other.
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
            ("vdd", "_core"),
            ("out", "_p"),
            ("VC", "M"),
            ("ADC", "_IN"),
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

    /// Nothing this crate ships says an executed deck is a property of the
    /// session that ran it.
    ///
    /// Decks are written to the project file, so a reopened project reopens the
    /// decks its retained runs executed and an absent one is the archive's
    /// retention policy. `7d1f952ae` retracted the session claim and fixed one
    /// of the five surfaces that made it; the other four kept saying it for
    /// another wave, which is what a scan rather than a fix-by-hand prevents.
    ///
    /// The one owner is
    /// `state::simulation::executed_deck::absent_deck_reason`.
    /// Test files are exempt, because a test asserting the retraction has to be
    /// able to name what was retracted.
    #[test]
    fn no_shipped_code_says_an_executed_deck_belongs_to_one_session() {
        const RETRACTED: [&str; 3] = [
            "retained for the session that ran",
            "this session did not run",
            "session does not hold it",
        ];

        let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut paths = Vec::new();
        rust_sources(&source_root, &mut paths);
        paths.sort();
        assert!(
            paths.len() > 100,
            "the crate-wide scan found only {} files under {}; a scan that reaches nothing \
             passes forever",
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

        let mut found = Vec::new();
        for (path, source) in &sources {
            if !ships(path, &roots) {
                continue;
            }
            for (number, line) in without_test_items(source).lines().enumerate() {
                let lowered = line.to_ascii_lowercase();
                for phrase in RETRACTED {
                    if lowered.contains(phrase) {
                        found.push(format!(
                            "{}:{}: shipped copy still ties an executed deck to a session",
                            path.display(),
                            number + 1,
                        ));
                    }
                }
            }
        }

        assert!(
            found.is_empty(),
            "an executed deck's absence is the archive's retention policy, not a fact about \
             which session ran the dataset \u{2014} read \
             `state::simulation::executed_deck::absent_deck_reason`:\n  {}",
            found.join("\n  ")
        );
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

    /// The character a double-encoded UTF-8 sequence starts with.
    ///
    /// A UTF-8 lead byte read back as Latin-1 and written out as UTF-8 again
    /// becomes a printable character: `\u{b7}` becomes `\u{c2}\u{b7}`,
    /// `\u{394}` becomes `\u{ce}\u{201d}`, `\u{2014}` becomes
    /// `\u{e2}\u{80}\u{94}`. These are the four leads that produce every form
    /// this crate has shipped — the Latin-1 supplement, the accented Latin
    /// letters, the Greek block, and the general punctuation the em dash,
    /// ellipsis and curly quotes live in.
    const MOJIBAKE_LEADS: [char; 4] = ['\u{c2}', '\u{c3}', '\u{ce}', '\u{e2}'];

    /// Windows-1252's printable stand-ins for the bytes `0x80`-`0x9F`.
    ///
    /// Latin-1 leaves those bytes unassigned, so the round trip that produces
    /// mojibake usually runs through Windows-1252 instead and a continuation
    /// byte in that range comes back as punctuation rather than as a control
    /// character. Without these the delta the reliability page shipped —
    /// `\u{ce}\u{201d}` — reads as a lead followed by an ordinary right
    /// quotation mark and no scan catches it.
    const WINDOWS_1252_HIGH: [char; 27] = [
        '\u{20ac}', '\u{201a}', '\u{192}', '\u{201e}', '\u{2026}', '\u{2020}', '\u{2021}',
        '\u{2c6}', '\u{2030}', '\u{160}', '\u{2039}', '\u{152}', '\u{17d}', '\u{2018}', '\u{2019}',
        '\u{201c}', '\u{201d}', '\u{2022}', '\u{2013}', '\u{2014}', '\u{2dc}', '\u{2122}',
        '\u{161}', '\u{203a}', '\u{153}', '\u{17e}', '\u{178}',
    ];

    /// Whether `character` is how a UTF-8 continuation byte comes back.
    fn is_mojibake_trail(character: char) -> bool {
        matches!(character, '\u{80}'..='\u{bf}') || WINDOWS_1252_HIGH.contains(&character)
    }

    /// The first double-encoded pair in `line`, if it carries one.
    ///
    /// An ASCII line cannot carry one, and almost every line in the crate is
    /// ASCII, so that is the check the whole-crate scan runs on nearly all of
    /// them.
    fn double_encoded_pair(line: &str) -> Option<(char, char)> {
        if line.is_ascii() {
            return None;
        }
        line.chars()
            .zip(line.chars().skip(1))
            .find(|(lead, trail)| MOJIBAKE_LEADS.contains(lead) && is_mojibake_trail(*trail))
    }

    /// No source file carries a double-encoded UTF-8 sequence.
    ///
    /// A file edited by a tool that read it as Latin-1 and wrote it back as
    /// UTF-8 keeps compiling and keeps its tests green: the literal is still a
    /// valid string, it is simply the wrong one. The studio shipped a run
    /// summary reading `\u{c2}\u{b7}` where its separator belonged and a
    /// reliability column headed `\u{ce}\u{201d}VTH` where the delta belonged,
    /// and every assertion over those strings agreed with them, because the
    /// assertions had been re-encoded by the same pass.
    ///
    /// Only a scan of the bytes catches that, so this is that scan. It covers
    /// test code as well as shipped code: a fixture spelled in mojibake is how
    /// a re-encoded expectation gets frozen into a passing test.
    ///
    /// It covers the engine crate as well as this one. Re-encoding happens to
    /// whatever a tool opens, not to whatever ships a user interface, and the
    /// engine carried two of its own: an approximation sign in a
    /// discretization comment and a delta in a checkpoint assertion. The roots
    /// are reached from `CARGO_MANIFEST_DIR` rather than named absolutely, and
    /// each is asserted to have contributed files — a root that stops
    /// resolving is a scan that has stopped scanning.
    #[test]
    fn no_source_file_carries_double_encoded_utf8() {
        // The pairs the scan is looking for, written as escapes so this file
        // is not itself a match. A detector that stopped detecting would
        // otherwise pass forever.
        for (lead, trail) in [
            ('\u{c2}', '\u{b7}'),
            ('\u{ce}', '\u{201d}'),
            ('\u{e2}', '\u{80}'),
            ('\u{c3}', '\u{a9}'),
        ] {
            let sample = format!("before {lead}{trail} after");
            assert_eq!(
                double_encoded_pair(&sample),
                Some((lead, trail)),
                "the scan recognizes U+{:04X} U+{:04X}",
                lead as u32,
                trail as u32
            );
        }
        assert_eq!(
            double_encoded_pair("plain ASCII, an em dash \u{2014}, and \u{e9}\u{2026}"),
            None,
            "the scan does not flag correctly encoded text"
        );

        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        let crates = manifest
            .parent()
            .expect("this crate sits in the workspace's crates/ directory");
        let mut paths = Vec::new();
        for root in [
            manifest.join("src"),
            manifest.join("tests"),
            crates.join("rspice-core").join("src"),
            crates.join("rspice-core").join("tests"),
        ] {
            let before = paths.len();
            rust_sources(&root, &mut paths);
            assert!(
                paths.len() > before + 2,
                "the scan found {} files under {}; a root that stops resolving is a scan \
                 that has stopped scanning",
                paths.len() - before,
                root.display()
            );
        }
        paths.sort();
        assert!(
            paths.len() > 400,
            "the two-crate scan found only {} files; a scan that reaches nothing passes \
             forever",
            paths.len()
        );

        let mut found = Vec::new();
        for path in &paths {
            let source = std::fs::read_to_string(path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            for (number, line) in source.lines().enumerate() {
                if let Some((lead, trail)) = double_encoded_pair(line) {
                    found.push(format!(
                        "{}:{}: U+{:04X} U+{:04X}",
                        path.display(),
                        number + 1,
                        lead as u32,
                        trail as u32
                    ));
                }
            }
        }

        assert!(
            found.is_empty(),
            "these lines carry UTF-8 that was read as Latin-1 and written out again \
             — re-encode them, and check every assertion over the same text:\n  {}",
            found.join("\n  ")
        );
    }
}
