//! A guard that reads the crate's shipped source back and bans the status
//! colouring the result viewers no longer use.
//!
//! Text in a result viewer is neutral. Colour is left to data (traces, plot
//! geometry, a heat ramp), to a verdict cell that failed or is near its limit,
//! to the error under the field it rejects, and to the selected item. A status
//! the reader has to notice is a `Problem` mark in front of neutral words, and
//! success is never styled. These scans keep the old shortcuts from coming
//! back one call at a time.

use std::path::{Path, PathBuf};

/// Calls that only ever painted status as coloured text or a dot.
const BANNED_CALLS: [&str; 4] = [
    "colored_label",
    "status_dot(",
    "StatusMark::Success",
    "StatusMark::Neutral",
];

/// Receivers the palette is read through in this crate, and the two roles no
/// shipped viewer paints with: `ok` is success styling and `info` is chrome
/// tinted blue.
const PALETTE_RECEIVERS: [&str; 4] = ["color", "c", "colors", "palette"];
const BANNED_ROLES: [&str; 2] = ["ok", "info"];

/// Data colours that happen to read a banned role, each with its reason.
const ALLOWED: [(&str, &str, &str); 1] = [(
    "network_matrix.rs",
    "palette.info",
    "the coefficient heat ramp is a data scale, not status",
)];

/// Every `.rs` file under `root`, with test files left out. This file is one
/// of them: it is compiled only for tests and spells every banned pattern.
fn shipped_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let entries = std::fs::read_dir(&directory)
            .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()));
        for entry in entries {
            let path = entry.expect("directory entry").path();
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
                .to_owned();
            if path.is_dir() {
                if name != "tests" {
                    pending.push(path);
                }
            } else if name.ends_with(".rs")
                && name != "tests.rs"
                && !name.ends_with("_tests.rs")
                && name != "source_guard.rs"
            {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// `source` with every test-only item blanked, line for line, so a reported
/// line number is the line in the file.
///
/// An item is test-only when its `#[cfg(...)]` names `test` (and is not
/// `not(test)`). An item ending in `;` is one line; a braced item ends at the
/// first `}` indented exactly as far as its attribute, the way `rustfmt`
/// writes it. An item whose end is never found panics rather than leaving the
/// rest of the file unscanned.
fn without_test_items(source: &str) -> Vec<&str> {
    let lines = source.lines().collect::<Vec<_>>();
    let mut kept = Vec::with_capacity(lines.len());
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim_start();
        let test_only = trimmed.starts_with("#[cfg(")
            && trimmed.contains("test")
            && !trimmed.contains("not(test");
        if !test_only {
            kept.push(line);
            index += 1;
            continue;
        }
        let indent = &line[..line.len() - trimmed.len()];
        let mut end = index + 1;
        while end < lines.len() && lines[end].trim_start().starts_with("#[") {
            end += 1;
        }
        let item = lines.get(end).copied().unwrap_or_default();
        if !item.trim_end().ends_with(';') {
            let close = format!("{indent}}}");
            end = (end..lines.len())
                .find(|&candidate| lines[candidate].trim_end() == close)
                .unwrap_or_else(|| panic!("test-only item at line {} never closes", index + 1));
        }
        kept.extend(std::iter::repeat_n("", end + 1 - index));
        index = end + 1;
    }
    kept
}

/// Whether `line` reads `receiver.role` as a whole field access: not part of
/// a longer name on either side, and not a method call such as `.ok()`.
fn reads_role(line: &str, receiver: &str, role: &str) -> bool {
    let needle = format!("{receiver}.{role}");
    line.match_indices(&needle).any(|(at, _)| {
        let before = line[..at].chars().next_back();
        let after = line[at + needle.len()..].chars().next();
        !before.is_some_and(|ch| ch.is_alphanumeric() || ch == '_')
            && !after.is_some_and(|ch| ch.is_alphanumeric() || ch == '_' || ch == '(')
    })
}

#[test]
fn no_shipped_viewer_paints_status_in_colour_or_with_a_dot() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let roots = [
        manifest.join("src"),
        // The standalone viewer's hover readout follows the same grammar.
        manifest.join("../rspice-viewer/src"),
    ];
    let mut scanned = Vec::new();
    let mut offences = Vec::new();
    for root in &roots {
        for path in shipped_files(root) {
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            for (number, line) in without_test_items(&source).into_iter().enumerate() {
                let code = line.split("//").next().unwrap_or_default();
                for call in BANNED_CALLS {
                    if code.contains(call) {
                        offences.push(format!("{}:{}: {call}", path.display(), number + 1));
                    }
                }
                for receiver in PALETTE_RECEIVERS {
                    for role in BANNED_ROLES {
                        let read = format!("{receiver}.{role}");
                        let allowed = ALLOWED
                            .iter()
                            .any(|(file, site, _)| *file == name && *site == read);
                        if !allowed && reads_role(code, receiver, role) {
                            offences.push(format!("{}:{}: {read}", path.display(), number + 1));
                        }
                    }
                }
            }
            scanned.push(path);
        }
    }
    // A guard that found nothing to read passes forever; make sure it read
    // the surfaces it exists for.
    for expected in ["chrome.rs", "specs.rs", "op_inspector.rs", "plot.rs"] {
        assert!(
            scanned.iter().any(|path| path.ends_with(expected)),
            "the scan never reached {expected}"
        );
    }
    assert!(
        offences.is_empty(),
        "status painted in colour or with a dot:\n{}",
        offences.join("\n")
    );
}

#[test]
fn the_role_scan_tells_a_field_from_a_method_or_a_longer_name() {
    assert!(reads_role("    .color(t.color.ok)", "color", "ok"));
    assert!(reads_role("(c.info, 1.0)", "c", "info"));
    assert!(!reads_role("result.ok()", "result", "ok"));
    assert!(!reads_role("spec.ok_count", "spec", "ok"));
    assert!(!reads_role("disc.ok", "c", "ok"));
}

#[test]
fn test_only_items_are_blanked_line_for_line() {
    let source = "fn shipped() {}\n#[cfg(test)]\nmod tests {\n    fn x() {}\n}\nfn after() {}\n";
    let kept = without_test_items(source);
    assert_eq!(kept, ["fn shipped() {}", "", "", "", "", "fn after() {}"]);
    let external = "#[cfg(test)]\nmod tests;\nfn shipped() {}\n";
    assert_eq!(without_test_items(external), ["", "", "fn shipped() {}"]);
}
