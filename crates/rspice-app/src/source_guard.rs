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
        let item = source[index + ATTRIBUTE.len()..].trim_start();
        // A test module whose fixtures other modules' tests build from is
        // visible to the crate; it is the same boundary.
        let item = item
            .strip_prefix("pub(crate)")
            .map_or(item, str::trim_start);
        if item.starts_with("mod tests") {
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

/// An item's text with any leading `pub`, `pub(crate)` or `pub(in path)` taken
/// off.
///
/// A fixture module several suites share is declared `pub(crate) mod`, and a
/// scan that only knew the bare spelling read that file as shipped code.
#[cfg(test)]
fn without_visibility(item: &str) -> &str {
    let Some(rest) = item.strip_prefix("pub") else {
        return item;
    };
    if let Some(scoped) = rest.strip_prefix('(') {
        return scoped
            .split_once(')')
            .map_or(item, |(_, after)| after.trim_start());
    }
    if rest.starts_with(char::is_whitespace) {
        rest.trim_start()
    } else {
        item
    }
}

/// Where the `#[cfg(test)] mod <name>;` children of `path` would live, whatever
/// visibility they are declared with. A feature crate's fixture module that its
/// `test-support` feature opens to the application's tests is declared
/// `#[cfg(any(test, feature = "test-support"))]`, and is test-only the same way:
/// only the tests' dev-dependencies turn that feature on.
#[cfg(test)]
pub(crate) fn test_only_roots(path: &std::path::Path, source: &str) -> Vec<std::path::PathBuf> {
    const ATTRIBUTES: [&str; 2] = [
        "#[cfg(test)]",
        "#[cfg(any(test, feature = \"test-support\"))]",
    ];
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
    ATTRIBUTES
        .iter()
        .flat_map(|attribute| {
            source
                .match_indices(attribute)
                .map(move |(index, _)| index + attribute.len())
        })
        .filter_map(|end| {
            let rest = without_visibility(source[end..].trim_start());
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

    /// [`test_only_roots`] reads a module under the `test-support` gate as
    /// test-only because only the tests' dev-dependencies turn that feature on.
    /// This keeps it true: no application crate's manifest enables another
    /// crate's `test-support` outside its `[dev-dependencies]`, or a
    /// `[features]` list, which forwards the feature only while the crate's own
    /// `test-support` is on.
    #[test]
    fn only_dev_dependencies_turn_test_support_on() {
        // Every application crate's manifest; the discovery asserts it found them.
        let mut strays = Vec::new();
        for root in app_source_roots() {
            let manifest = root
                .parent()
                .expect("a source root sits in its crate's directory")
                .join("Cargo.toml");
            let text = std::fs::read_to_string(&manifest)
                .unwrap_or_else(|error| panic!("read {}: {error}", manifest.display()));
            let mut table = String::new();
            for (number, line) in text.lines().enumerate() {
                let code = line.split('#').next().unwrap_or_default().trim();
                if code.starts_with('[') {
                    table = code.trim_matches(|c| c == '[' || c == ']').to_owned();
                    continue;
                }
                let test_only = table == "features"
                    || table.ends_with("dev-dependencies")
                    || table.contains("dev-dependencies.");
                if !test_only && code.contains("\"test-support\"") {
                    strays.push(format!("{}:{}: {code}", manifest.display(), number + 1));
                }
            }
        }
        assert!(
            strays.is_empty(),
            "test-support is turned on outside [dev-dependencies], so a module under its gate \
             would ship:\n  {}",
            strays.join("\n  ")
        );
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

    /// Every authored `.rs` file under `directory`, recursively.
    ///
    /// Build output is skipped: a crate that has been built on its own — the
    /// repository's ignore rules say `crates/rspice-core` and the benchmark
    /// probes both grow one — holds a `target/` directory full of generated
    /// sources that no guard here has anything to say about, and walking it
    /// would turn a scan of the workspace into a scan of a build cache.
    fn rust_sources(directory: &Path, out: &mut Vec<PathBuf>) {
        let entries = std::fs::read_dir(directory)
            .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()));
        for entry in entries {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name == "target") {
                    continue;
                }
                rust_sources(&path, out);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                out.push(path);
            }
        }
    }

    /// Every application crate's `src`: this crate's and each `rspice-app-*`
    /// crate's, read at run time rather than listed here.
    ///
    /// The rules the crate-wide scans hold are the application's, not one
    /// crate's, so each of them reads every crate the application compiles as.
    /// A module that moves to another application crate keeps its path under
    /// that crate's `src` and is still read there, and a new application crate
    /// is read without an edit here.
    fn app_source_roots() -> Vec<PathBuf> {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        let crates = manifest
            .parent()
            .expect("this crate sits in the workspace's crates/ directory");
        let mut roots: Vec<PathBuf> = std::fs::read_dir(crates)
            .unwrap_or_else(|error| panic!("read {}: {error}", crates.display()))
            .map(|entry| entry.expect("directory entry").path())
            .filter(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name == "rspice-app" || name.starts_with("rspice-app-"))
                    && path.join("Cargo.toml").is_file()
            })
            .map(|path| path.join("src"))
            .collect();
        roots.sort();
        // A floor, not a tally: the application is this crate and the fourteen
        // split out of it, and a discovery that finds fewer has lost its roots.
        assert!(
            roots.len() >= 15 && roots.contains(&manifest.join("src")),
            "the scan found only {} application crates under {}; a root list that stops \
             resolving is a scan that has stopped scanning",
            roots.len(),
            crates.display()
        );
        roots
    }

    /// Every authored `.rs` file of every application crate, sorted.
    ///
    /// Each crate is asserted to have contributed files: a root that stops
    /// resolving is a scan that has stopped scanning, and a scan that reaches
    /// nothing passes forever.
    fn app_rust_sources() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        for root in app_source_roots() {
            let before = paths.len();
            rust_sources(&root, &mut paths);
            assert!(
                paths.len() > before,
                "the scan found no Rust source under {}; a root that stops resolving is a \
                 scan that has stopped scanning",
                root.display()
            );
        }
        paths.sort();
        paths
    }

    /// Every authored `.rs` file in the workspace, sorted, with the floors the
    /// whole-workspace scans rest on.
    ///
    /// The roots are the crate directories `crates/` actually holds, read at
    /// run time rather than listed here, so a crate added to the workspace is
    /// scanned without anyone remembering to add it. Each is asserted to have
    /// contributed files — a root that stops resolving is a scan that has
    /// stopped scanning, and a scan that reaches nothing passes forever.
    ///
    /// These are repository-hygiene scans rather than anything about a user
    /// interface, so `rspice-ui` is not their natural owner; they live here
    /// because the workspace has no crate whose subject is the repository, and
    /// adding one would add a workspace member, which edits `Cargo.toml` and
    /// `Cargo.lock` — both source-digest inputs of the Verilog-A built-ins
    /// generator — and restamp the whole generated model tree. If such a crate
    /// ever exists, this walk, [`rust_sources`] and both scans over them belong
    /// in it.
    fn workspace_rust_sources() -> Vec<PathBuf> {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        let crates = manifest
            .parent()
            .expect("this crate sits in the workspace's crates/ directory");
        // A crate is a directory here with a manifest in it. Nested members —
        // the generated Verilog-A model crates — are reached by recursion from
        // the directory that holds them, so only the top level is enumerated.
        let mut roots: Vec<PathBuf> = std::fs::read_dir(crates)
            .unwrap_or_else(|error| panic!("read {}: {error}", crates.display()))
            .map(|entry| entry.expect("directory entry").path())
            .filter(|path| path.join("Cargo.toml").is_file())
            .collect();
        roots.sort();
        // A floor, not a tally: crates come and go, but a `crates/` that
        // resolves to a handful of them is a scan that has lost its roots.
        assert!(
            roots.len() >= 20,
            "the workspace scan found only {} crates under {}; a root list that stops \
             resolving is a scan that has stopped scanning",
            roots.len(),
            crates.display()
        );

        let mut paths = Vec::new();
        for root in &roots {
            let before = paths.len();
            rust_sources(root, &mut paths);
            assert!(
                paths.len() > before,
                "the scan found no Rust source under {}; a root that stops resolving is a \
                 scan that has stopped scanning",
                root.display()
            );
        }
        paths.sort();
        assert!(
            paths.len() > 2_000,
            "the workspace scan found only {} files; a scan that reaches nothing passes \
             forever",
            paths.len()
        );
        paths
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
        let paths = app_rust_sources();
        assert!(
            paths.len() > 100,
            "the crate-wide scan found only {} files; a scan that reaches nothing passes \
             forever",
            paths.len()
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
    /// It covers every crate in the workspace, not only this one. Re-encoding
    /// happens to whatever a tool opens, not to whatever ships a user
    /// interface, and the two-crate version of this scan proved that by
    /// missing what the command-line interface shipped: 49 lines across eight
    /// files of `rspice-cli`, including the check marks a multi-run prints
    /// beside every deck, the box frame of the corner-sweep summary, and the
    /// whole Fourier results table, plus three em dashes in `rspice-veriloga`.
    /// Every one of them was outside the root list and every one of them
    /// reached a user's terminal as mojibake.
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

        let paths = workspace_rust_sources();

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

    /// No source file starts with a byte-order mark.
    ///
    /// The mark is the other half of what a tool that re-encodes a file leaves
    /// behind, and it is quieter: `rustc` skips a leading U+FEFF, so the file
    /// compiles, its tests pass, and nothing in the build says a word. Everyone
    /// downstream of the compiler sees it — the first line of a diff hunk, a
    /// module doc comment that no longer starts at column one, `include_str!`
    /// of the file, and any scan of a source's first bytes — and it spreads,
    /// because a tool that writes one file's mark writes the next one's too.
    /// `rspice-cli` shipped three: `commands/run.rs` and two of its children.
    ///
    /// This is the same walk the double-encoding scan runs, for the same
    /// reason: catching it needs the bytes, not the decoded text, and the
    /// crates that carry marks are not the crates that ship an interface.
    #[test]
    fn no_source_file_starts_with_a_byte_order_mark() {
        const BOM: [u8; 3] = [0xef, 0xbb, 0xbf];

        let found: Vec<String> = workspace_rust_sources()
            .iter()
            .filter(|path| {
                std::fs::read(path)
                    .unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
                    .starts_with(&BOM)
            })
            .map(|path| path.display().to_string())
            .collect();

        assert!(
            found.is_empty(),
            "these files start with a UTF-8 byte-order mark — strip the first three \
             bytes; Rust sources are UTF-8 by definition and the mark only travels:\n  {}",
            found.join("\n  ")
        );
    }

    /// No Console line or status-bar prompt is formatted with `{:?}`.
    ///
    /// Debug formatting prints a type's internal spelling — `Some("amp")`,
    /// `ProjectRevision(7)` — where the reader needs a sentence, and every
    /// one of these sites is product text. The scan covers every shipped
    /// call of the verbs that write the session's log, a run's own log or the
    /// status bar.
    #[test]
    fn no_console_line_or_prompt_is_formatted_with_debug() {
        const VERBS: [&str; 5] = [
            ".record(",
            ".record_anchored(",
            ".prompt(",
            ".run_line(",
            "write_run_line(",
        ];
        let paths = app_rust_sources();
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

        let mut calls = 0_usize;
        let mut found = Vec::new();
        for (path, source) in &sources {
            if !ships(path, &roots) {
                continue;
            }
            let shipped = without_test_items(source);
            for verb in VERBS {
                for (start, _) in shipped.match_indices(verb) {
                    calls += 1;
                    let arguments = call_arguments(&shipped[start + verb.len()..]);
                    if arguments.contains(":?}") || arguments.contains(":#?}") {
                        let line = shipped[..start].lines().count();
                        found.push(format!("{}:{line}", path.display()));
                    }
                }
            }
        }

        assert!(
            calls > 300,
            "the scan found only {calls} calls; a scan that reaches nothing passes forever"
        );
        assert!(
            found.is_empty(),
            "a Console line or prompt is formatted with `{{:?}}`; say it in words:\n  {}",
            found.join("\n  ")
        );
    }

    /// The text of a call's arguments: everything up to the parenthesis that
    /// closes the one the call opened, with string and character literals
    /// stepped over so a parenthesis inside a message does not end it.
    fn call_arguments(after_open: &str) -> &str {
        let bytes = after_open.as_bytes();
        let mut depth = 1_usize;
        let mut index = 0;
        while index < bytes.len() {
            match bytes[index] {
                b'"' => {
                    // A raw string's hashes make no difference here: its
                    // closing quote is the next quote it holds.
                    index += 1;
                    while index < bytes.len() && bytes[index] != b'"' {
                        index += if bytes[index] == b'\\' { 2 } else { 1 };
                    }
                }
                b'\'' if bytes.get(index + 2) == Some(&b'\'') => index += 2,
                b'\'' if bytes.get(index + 1) == Some(&b'\\') => {
                    while index + 1 < bytes.len() && bytes[index + 1] != b'\'' {
                        index += 1;
                    }
                    index += 1;
                }
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        return &after_open[..index];
                    }
                }
                _ => {}
            }
            index += 1;
        }
        after_open
    }

    /// The places a whole `AppState` is held under the names the crate gives
    /// it, with the dereference an assignment through `&mut AppState` needs.
    const WHOLE_STATE: [&str; 6] = [
        "*state",
        "*app_state",
        "state",
        "app_state",
        "self.state",
        "app.state",
    ];

    /// Whether `line` installs a whole application state built elsewhere:
    /// an assignment, swap, replace or `clone_from` whose target is the whole
    /// state and whose value is a local binding.
    fn writes_back_whole_state(line: &str) -> bool {
        let line = line.trim();
        let is_binding = |text: &str| {
            let text = text
                .trim()
                .trim_start_matches("&mut ")
                .trim_start_matches('&');
            text.starts_with(|first: char| first.is_ascii_lowercase() || first == '_')
                && text
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == '_')
        };
        let is_whole_state = |text: &str| {
            let text = text.trim().trim_start_matches("&mut ");
            WHOLE_STATE.contains(&text)
        };
        for target in WHOLE_STATE {
            let Some(rest) = line.strip_prefix(target) else {
                continue;
            };
            if let Some(value) = rest.trim_start().strip_prefix('=')
                && !value.starts_with('=')
                && let Some(value) = value.trim().strip_suffix(';')
                && target != "state"
                && target != "app_state"
                && is_binding(value)
            {
                return true;
            }
            if rest.starts_with(".clone_from(") {
                return true;
            }
        }
        ["mem::swap(", "mem::replace("].into_iter().any(|call| {
            line.find(call).is_some_and(|start| {
                let arguments = call_arguments(&line[start + call.len()..]);
                let (first, second) = arguments.split_once(',').unwrap_or((arguments, ""));
                is_whole_state(first) || is_whole_state(second)
            })
        })
    }

    /// No shipped code installs a project transaction's candidate by hand.
    ///
    /// A transaction builds its result in a clone of `AppState`, and a clone
    /// leaves out the runtime sessions the live state owns: the hardcopy
    /// session with its in-flight print, export or publish, and the reference
    /// pickers specification drafts wait on. Assigning the candidate over the
    /// live state drops them, and the next frame cancels the print or export
    /// the operator started. `AppState::commit_candidate` carries them across;
    /// it is the one place a whole state is installed.
    #[test]
    fn project_transactions_install_their_candidate_only_through_their_commit() {
        let owner = Path::new("workbench")
            .join("app_state")
            .join("transaction.rs");
        let paths = app_rust_sources();
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

        let mut owners = 0_usize;
        let mut commits = 0_usize;
        let mut found = Vec::new();
        for (path, source) in &sources {
            if !ships(path, &roots) {
                continue;
            }
            if path.ends_with(&owner) {
                owners += source.matches("fn commit_candidate(").count();
                continue;
            }
            for (number, line) in without_test_items(source).lines().enumerate() {
                commits += line.matches(".commit_candidate(").count();
                if writes_back_whole_state(line) {
                    found.push(format!("{}:{}", path.display(), number + 1));
                }
            }
        }

        assert_eq!(
            owners,
            1,
            "`AppState::commit_candidate` is defined once, in {}",
            owner.display()
        );
        assert!(
            commits > 0,
            "the scan found no transaction commit outside {}; a scan that reaches nothing \
             passes forever",
            owner.display()
        );
        assert!(
            found.is_empty(),
            "these install a whole application state by hand, which drops the runtime \
             sessions a clone leaves out — commit it with `AppState::commit_candidate`:\n  {}",
            found.join("\n  ")
        );
    }

    /// Whether a line assigns the model's project-open fact outright.
    fn assigns_project_open(code: &str) -> bool {
        let assigned = code.match_indices(".project_open").any(|(at, name)| {
            let rest = code[at + name.len()..].trim_start();
            rest.starts_with('=') && !rest.starts_with("==")
        });
        // `mem` is spelled in two halves so the scan does not find this line.
        assigned || (code.contains("project_open") && code.contains(concat!("mem", "::")))
    }

    /// Model code reads whether a project is open from `AppState::project_open`,
    /// and that copy is only right while one function writes it: the project
    /// lifecycle's `publish_open`, which each transition that opens, resets or
    /// closes the project calls in the same call. A second writer, or shipped
    /// code that moves the lifecycle's authority past those transitions, leaves
    /// the model reading an answer the lifecycle no longer gives.
    #[test]
    fn only_the_project_lifecycle_writes_whether_a_project_is_open() {
        // Spelled in two halves so this scan does not find itself.
        const WRITER: &str = concat!("set_from_", "lifecycle(");
        let owner = Path::new("workbench")
            .join("lifecycle")
            .join("project_lifecycle.rs");
        let fact = Path::new("state").join("project_open.rs");
        let paths = app_rust_sources();
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

        let mut owner_writes = 0_usize;
        let mut owner_reads = 0_usize;
        let mut writers = Vec::new();
        let mut moves = Vec::new();
        for (path, source) in &sources {
            if path.ends_with(&fact) {
                continue;
            }
            let owned = path.ends_with(&owner);
            for (number, line) in source.lines().enumerate() {
                let code = line.split("//").next().unwrap_or_default();
                let writes = code.matches(WRITER).count();
                if owned {
                    owner_writes += writes;
                    owner_reads += code.matches(".publish_open(").count();
                } else if writes > 0 || assigns_project_open(code) {
                    writers.push(format!("{}:{}", path.display(), number + 1));
                }
            }
            if owned || !ships(path, &roots) {
                continue;
            }
            for (number, line) in without_test_items(source).lines().enumerate() {
                let code = line.split("//").next().unwrap_or_default();
                if [
                    ".open_session()",
                    ".close_project()",
                    ".reset_for_new_project()",
                ]
                .iter()
                .any(|transition| code.contains(transition))
                {
                    moves.push(format!("{}:{}", path.display(), number + 1));
                }
            }
        }

        assert_eq!(
            owner_writes,
            1,
            "the project lifecycle writes the model's project-open fact from one function, in {}",
            owner.display()
        );
        assert!(
            owner_reads >= 3,
            "only {owner_reads} lifecycle transitions publish whether a project is open; open, \
             new and close each must"
        );
        assert!(
            writers.is_empty(),
            "these write the model's project-open fact outside the project lifecycle — open, \
             reset or close the project through it instead:\n  {}",
            writers.join("\n  ")
        );
        assert!(
            moves.is_empty(),
            "these open or close the project lifecycle's authority directly, which leaves the \
             model's project-open fact behind — go through the lifecycle's transitions:\n  {}",
            moves.join("\n  ")
        );
    }
}
