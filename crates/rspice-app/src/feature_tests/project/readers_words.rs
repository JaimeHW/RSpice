//! What the project's open, save, close, checkpoints, History and the
//! launcher say is in the reader's words: no message speaks of the store's
//! own parts (a canonical file, an authority, a journal, a baseline, a
//! working tree, a closure, a root digest, a pinned source, adopting a save).

/// The shipped sources whose messages reach the reader.
const SOURCES: [(&str, &str); 15] = [
    (
        "rspice-project/src/lifecycle.rs",
        include_str!("../../../../rspice-project/src/lifecycle.rs"),
    ),
    (
        "rspice-project/src/lifecycle/save.rs",
        include_str!("../../../../rspice-project/src/lifecycle/save.rs"),
    ),
    (
        "rspice-project/src/lifecycle/dirty.rs",
        include_str!("../../../../rspice-project/src/lifecycle/dirty.rs"),
    ),
    (
        "project/revision_history.rs",
        include_str!("../../../../rspice-app-project/src/workbench/project/revision_history.rs"),
    ),
    (
        "surfaces/project/history.rs",
        include_str!("../../../../rspice-app-project/src/workbench/surfaces/project/history.rs"),
    ),
    (
        "docks/inspector/project/recovery.rs",
        include_str!(
            "../../../../rspice-app-project/src/workbench/docks/inspector/project/recovery.rs"
        ),
    ),
    (
        "workflows/project_workflow.rs",
        include_str!("../../../../rspice-app-project/src/workbench/workflows/project_workflow.rs"),
    ),
    (
        "tools/project_launcher.rs",
        include_str!("../../../../rspice-app-project/src/workbench/tools/project_launcher.rs"),
    ),
    (
        "tools/project_launcher/recovery.rs",
        include_str!(
            "../../../../rspice-app-project/src/workbench/tools/project_launcher/recovery.rs"
        ),
    ),
    (
        "tools/project_launcher/recent_rows.rs",
        include_str!(
            "../../../../rspice-app-project/src/workbench/tools/project_launcher/recent_rows.rs"
        ),
    ),
    (
        "surfaces/project/landing.rs",
        include_str!("../../../../rspice-app-project/src/workbench/surfaces/project/landing.rs"),
    ),
    (
        "session/autosave.rs",
        include_str!("../../../../rspice-app-services/src/workbench/app_state/session/autosave.rs"),
    ),
    (
        "lifecycle/recovery.rs",
        include_str!("../../../../rspice-app-services/src/workbench/lifecycle/recovery.rs"),
    ),
    (
        "lifecycle/project_checkpoint.rs",
        include_str!(
            "../../../../rspice-app-services/src/workbench/lifecycle/project_checkpoint.rs"
        ),
    ),
    (
        "session/project_transactions.rs",
        include_str!("../../workbench/session/project_transactions.rs"),
    ),
];

/// The store's words, lowercase.
const WORDS: [&str; 9] = [
    "canonical",
    "authority",
    "journal",
    "baseline",
    "working tree",
    "closure",
    "root digest",
    "pinned source",
    "adopt",
];

#[test]
fn lifecycle_and_history_messages_use_the_readers_words() {
    let mut found = Vec::new();
    for (name, source) in SOURCES {
        let source = source.replace("\r\n", "\n");
        let shipped = source
            .split("\n#[cfg(test)]\nmod tests")
            .next()
            .unwrap_or_default();
        // A message is a string literal with a space in it; a key, a path or
        // an identifier has none.
        for (line, literal) in string_literals(shipped)
            .into_iter()
            .filter(|(_, text)| text.contains(' '))
        {
            let lower = literal.to_lowercase();
            for word in WORDS.iter().filter(|word| lower.contains(**word)) {
                found.push(format!("{name}:{line}: {word}: {literal}"));
            }
        }
    }
    assert!(found.is_empty(), "{}", found.join("\n"));
}

/// Every string literal in `source` outside its comments, with the line it
/// starts on, escapes left as written; a literal may run over lines.
fn string_literals(source: &str) -> Vec<(usize, String)> {
    let chars = source.chars().collect::<Vec<_>>();
    let mut literals = Vec::new();
    let mut line = 1;
    let mut at = 0;
    while at < chars.len() {
        match chars[at] {
            '\n' => {
                line += 1;
                at += 1;
            }
            '/' if chars.get(at + 1) == Some(&'/') => {
                while at < chars.len() && chars[at] != '\n' {
                    at += 1;
                }
            }
            // A char literal, which may be a quote; a lifetime is passed by.
            '\'' => {
                at += match (chars.get(at + 1), chars.get(at + 2)) {
                    (Some('\\'), _) => {
                        let close = chars[at + 2..].iter().position(|c| *c == '\'');
                        close.map_or(1, |close| close + 3)
                    }
                    (Some(_), Some('\'')) => 3,
                    _ => 1,
                };
            }
            'r' if matches!(chars.get(at + 1), Some('"' | '#'))
                && (at == 0 || !chars[at - 1].is_alphanumeric() && chars[at - 1] != '_') =>
            {
                let hashes = chars[at + 1..].iter().take_while(|c| **c == '#').count();
                let open = at + 1 + hashes;
                if chars.get(open) != Some(&'"') {
                    at += 1;
                    continue;
                }
                let start = open + 1;
                let mut end = start;
                while end < chars.len()
                    && !(chars[end] == '"'
                        && chars[end + 1..]
                            .iter()
                            .take(hashes)
                            .filter(|c| **c == '#')
                            .count()
                            == hashes)
                {
                    end += 1;
                }
                let text = chars[start..end.min(chars.len())]
                    .iter()
                    .collect::<String>();
                literals.push((line, text.clone()));
                line += text.matches('\n').count();
                at = end + 1 + hashes;
            }
            '"' => {
                let start = at + 1;
                let mut end = start;
                while end < chars.len() && chars[end] != '"' {
                    end += if chars[end] == '\\' { 2 } else { 1 };
                }
                let text = chars[start..end.min(chars.len())]
                    .iter()
                    .collect::<String>();
                literals.push((line, text.clone()));
                line += text.matches('\n').count();
                at = end + 1;
            }
            _ => at += 1,
        }
    }
    literals
}
