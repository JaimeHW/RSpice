//! A scan of the symbol editor and the component editor's own sources.
//!
//! Text in these surfaces is neutral. A warning or a failure the reader has
//! to notice is a [`rspice_ui_kit::widgets::Problem`] mark in front of that
//! text, success is never styled at all, and a dot is kept for live state,
//! which none of these panels has. The shortcuts that used to say it in
//! colour — `colored_label`, `status_dot(`, `StatusMark::Success`,
//! `StatusMark::Neutral`, and a read of the `ok` or `info` palette role — are
//! what this reads the files back for.
//!
//! It covers the two files in `rspice-app` that build and host these
//! surfaces as well, because the host is where a colour would come back
//! from: the surface it feeds has no palette of its own.
//!
//! The module is test-only, so it ships nothing.

/// Everything in `source` that precedes its test module.
///
/// The boundary is located structurally rather than by a literal, so it holds
/// under either line ending and under both spellings this crate uses:
/// `#[cfg(test)]` and `#[cfg(all(test, not(target_arch = "wasm32")))]`.
///
/// Only an inline `mod tests {` is a boundary. An out-of-line `mod tests;`
/// declares another file, and in this slice one of them sits at the top of
/// its host — cutting there left four lines of a four-hundred-line surface
/// scanned, which is a guard that has quietly stopped guarding. A file that
/// declares test-only code and names no test module at all is a panic for
/// the same reason.
fn production_source(source: &str) -> &str {
    let boundary = source.match_indices("mod tests").find(|(at, _)| {
        source[at + "mod tests".len()..]
            .trim_start()
            .starts_with('{')
    });
    let Some((boundary, _)) = boundary else {
        assert!(
            !source.contains("#[cfg(test") || source.contains("mod tests"),
            "the scanned source declares test-only code and no `mod tests` block to stop at"
        );
        return source;
    };
    let attribute = source[..boundary]
        .rfind("#[cfg(")
        .expect("a `mod tests` block is introduced by a cfg attribute");
    &source[..attribute]
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::production_source;

    /// This crate's files, then the host's, each relative to its own crate.
    const EDITOR_FILES: [&str; 11] = [
        "symbol_editor/inspector.rs",
        "symbol_editor/surface.rs",
        "symbol_editor/session.rs",
        "symbol_editor/interaction.rs",
        "component_properties.rs",
        "component_properties/context.rs",
        "component_properties/dialog.rs",
        "component_properties/evidence.rs",
        "component_properties/form.rs",
        "pwl_editor/render.rs",
        "export.rs",
    ];
    const HOST_FILES: [&str; 3] = [
        "workbench/docks/navigator/symbol.rs",
        "workbench/app/dialogs/property_dialog.rs",
        "schematic/symbol_editor.rs",
    ];

    const BANNED_CALLS: [&str; 4] = [
        "colored_label",
        "status_dot(",
        "StatusMark::Success",
        "StatusMark::Neutral",
    ];
    /// The names these files read the palette through.
    const PALETTE_RECEIVERS: [&str; 4] = ["color", "colors", "c", "palette"];
    const BANNED_ROLES: [&str; 2] = ["ok", "info"];

    /// Whether `line` reads `receiver.role` as a whole field access: not part
    /// of a longer name on either side, and not a method call such as `.ok()`.
    fn reads_palette_role(line: &str, receiver: &str, role: &str) -> bool {
        let needle = format!("{receiver}.{role}");
        line.match_indices(&needle).any(|(at, _)| {
            let before = line[..at].chars().next_back();
            let after = line[at + needle.len()..].chars().next();
            !before.is_some_and(|ch| ch.is_alphanumeric() || ch == '_')
                && !after.is_some_and(|ch| ch.is_alphanumeric() || ch == '_' || ch == '(')
        })
    }

    /// Every file this guard reads, with the name to report it under.
    fn scanned() -> Vec<(String, PathBuf)> {
        let crate_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let host_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app/src");
        EDITOR_FILES
            .iter()
            .map(|file| ((*file).to_owned(), crate_root.join(file)))
            .chain(
                HOST_FILES
                    .iter()
                    .map(|file| (format!("rspice-app/{file}"), host_root.join(file))),
            )
            .collect()
    }

    /// The symbol editor and the component editor paint no status in colour.
    #[test]
    fn the_symbol_and_component_editors_paint_no_status_in_colour() {
        let mut found = Vec::new();
        for (name, path) in scanned() {
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            for (index, line) in production_source(&source).lines().enumerate() {
                let code = line.split("//").next().unwrap_or_default();
                let banned_call = BANNED_CALLS.iter().any(|call| code.contains(call));
                let banned_role = PALETTE_RECEIVERS.iter().any(|receiver| {
                    BANNED_ROLES
                        .iter()
                        .any(|role| reads_palette_role(code, receiver, role))
                });
                if banned_call || banned_role {
                    found.push(format!("{name}:{}: {}", index + 1, line.trim()));
                }
            }
        }
        assert!(
            found.is_empty(),
            "status painted in colour; use neutral text, a Problem mark, or nothing:\n  {}",
            found.join("\n  ")
        );
    }

    /// The scan reaches a shipped half of every file it names.
    ///
    /// A path that stops resolving, or a boundary that stops matching, is a
    /// guard that has quietly stopped guarding.
    #[test]
    fn the_scan_reaches_every_file_it_names() {
        for (name, path) in scanned() {
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            let shipped = production_source(&source);
            assert!(
                shipped.lines().count() > 20,
                "{name} shipped only {} lines to scan",
                shipped.lines().count()
            );
            assert!(
                !shipped.contains("mod tests {"),
                "{name} was scanned with its test module included"
            );
        }
    }

    /// The boundary is found under either line ending and under both cfg
    /// spellings, and a file with no test module at all is scanned whole.
    #[test]
    fn the_boundary_holds_under_both_cfg_spellings() {
        let plain = "fn shipped() {}\n\n#[cfg(test)]\nmod tests {\n    fn t() {}\n}\n";
        assert_eq!(production_source(plain), "fn shipped() {}\n\n");
        assert_eq!(
            production_source(&plain.replace('\n', "\r\n")),
            "fn shipped() {}\r\n\r\n"
        );
        let gated =
            "fn shipped() {}\n#[cfg(all(test, not(target_arch = \"wasm32\")))]\nmod tests {}\n";
        assert_eq!(production_source(gated), "fn shipped() {}\n");
        assert_eq!(production_source("fn shipped() {}\n"), "fn shipped() {}\n");
        // An out-of-line declaration is not a boundary, wherever it sits:
        // cutting at one at the top of a file leaves the file unscanned.
        let out_of_line = "#[cfg(test)]\nmod tests;\nfn shipped() {}\n";
        assert_eq!(production_source(out_of_line), out_of_line);
    }

    /// A file that carries test-only code without the boundary is refused.
    #[test]
    #[should_panic(expected = "no `mod tests` block")]
    fn test_only_code_without_the_boundary_is_refused() {
        let _ = production_source("#[cfg(test)]\nfn fixture() {}\nfn shipped() {}\n");
    }
}
