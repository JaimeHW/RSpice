//! The CLI README lists what `rspice capabilities` reports this CLI does not offer, written
//! from the same declarations the command prints: each analysis the CLI does not execute in
//! full, the study files, and the workflow gaps. The test fails when the README's copy
//! differs; `RSPICE_WRITE_SUPPORT_TABLE=1` rewrites the README's copy instead.

use super::{AnalysisKind, STUDY_SUPPORT, WORKFLOW_GAPS, execution_support};

const README: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/README.md");
const BEGIN: &str = "<!-- capability gaps: written from `rspice capabilities` by \
                     crates/rspice-cli/src/commands/capabilities/readme_gaps.rs -->";
const END: &str = "<!-- end of capability gaps -->";

/// One bullet per gap: the analyses that share an execution status and reason, then the study
/// files and each workflow gap.
fn gaps() -> String {
    let mut analyses: Vec<(&str, &str, Vec<String>)> = Vec::new();
    for kind in AnalysisKind::ALL {
        let support = execution_support(kind);
        let Some(reason) = support.reason else {
            continue;
        };
        let name = format!("`{}`", kind.tag());
        match analyses
            .iter_mut()
            .find(|(status, known, _)| *status == support.status && *known == reason)
        {
            Some((_, _, names)) => names.push(name),
            None => analyses.push((support.status, reason, vec![name])),
        }
    }
    let mut lines = analyses
        .into_iter()
        .map(|(status, reason, names)| format!("- {} ({status}): {reason}", names.join(", ")))
        .collect::<Vec<_>>();
    lines.push(format!("- `study_files` (partial): {STUDY_SUPPORT}"));
    lines.extend(
        WORKFLOW_GAPS
            .iter()
            .map(|(id, reason)| format!("- `{id}` (unsupported): {reason}")),
    );
    lines.join("\n")
}

#[test]
fn the_readme_lists_what_the_capabilities_command_says_the_cli_does_not_offer() {
    let readme = std::fs::read_to_string(README).expect("the CLI README is readable");
    let begin = readme
        .find(BEGIN)
        .expect("the README marks where the capability gaps begin")
        + BEGIN.len();
    let end = begin
        + readme[begin..]
            .find(END)
            .expect("the README marks where the capability gaps end");
    let expected = format!("\n\n{}\n\n", gaps());
    if std::env::var_os("RSPICE_WRITE_SUPPORT_TABLE").is_some() {
        let mut written = readme.clone();
        written.replace_range(begin..end, &expected);
        std::fs::write(README, written).expect("the CLI README is writable");
        eprintln!(
            "RSPICE_WRITE_SUPPORT_TABLE is set: rewrote the CLI README's capability gaps instead of checking them"
        );
        return;
    }
    assert_eq!(
        readme[begin..end],
        expected,
        "the README's capability gaps differ from `rspice capabilities`; rewrite them with \
         RSPICE_WRITE_SUPPORT_TABLE=1"
    );
}
