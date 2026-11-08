use super::*;
use crate::state::code_diagnostics::CodeDiagnosticCollection;
use rspice_vocabulary::diagnostics::SourceDiagnostic;

const SOURCE: &str = "module amp(out);\n  analog V(out) <+ gbww * 2;\nendmodule\n";

fn finding(
    severity: DiagnosticSeverity,
    message: &str,
    range: Option<Range<usize>>,
    line: usize,
    column: usize,
) -> SourceDiagnostic {
    SourceDiagnostic::current(
        "rspice.test",
        "VA-TEST",
        severity,
        message,
        "",
        Some("amp.va".to_owned()),
        None,
        range,
        Some(line),
        Some(column),
    )
}

/// The findings of `SOURCE`: an error on `gbww` with its range, and a
/// warning that names only the place of `out` on the first line.
fn held() -> HeldFindings {
    let start = SOURCE.find("gbww").expect("the error is in the source");
    let collection = CodeDiagnosticCollection::try_new(vec![
        finding(
            DiagnosticSeverity::Error,
            "unknown identifier",
            Some(start..start + 4),
            2,
            20,
        ),
        finding(DiagnosticSeverity::Warning, "unused port", None, 1, 12),
    ])
    .expect("a collection");
    let mut held = HeldFindings::default();
    held.take_current(SOURCE, 1, 7, &collection.view(Some("amp.va")));
    held
}

fn covered<'a>(held: &HeldFindings, text: &'a str) -> Vec<&'a str> {
    held.iter()
        .map(|finding| &text[finding.range.clone()])
        .collect()
}

#[test]
fn a_finding_that_names_only_a_place_covers_the_word_there() {
    let held = held();
    assert_eq!(covered(&held, SOURCE), ["out", "gbww"]);
    assert!(!held.is_outdated());
    assert_eq!(
        held.marks(),
        [
            SourceMark {
                line: 0,
                columns: 11..14,
                severity: DiagnosticSeverity::Warning,
            },
            SourceMark {
                line: 1,
                columns: 19..23,
                severity: DiagnosticSeverity::Error,
            },
        ]
    );
    assert_eq!(
        held.worst_on_line(1)
            .map(|finding| finding.message.as_ref()),
        Some("unknown identifier")
    );
}

#[test]
fn a_range_that_disagrees_with_its_line_and_column_gives_way_to_them() {
    // A range into some other text: the line and column are this one's.
    let collection = CodeDiagnosticCollection::try_new(vec![finding(
        DiagnosticSeverity::Error,
        "unknown identifier",
        Some(0..6),
        2,
        20,
    )])
    .expect("a collection");
    let mut held = HeldFindings::default();
    held.take_current(SOURCE, 1, 7, &collection.view(Some("amp.va")));
    assert_eq!(covered(&held, SOURCE), ["gbww"]);
}

#[test]
fn findings_move_through_edits_and_go_when_their_text_does() {
    let mut held = held();
    // Typing above both moves them down a line and keeps them.
    let edited = format!("// gain stage\n{SOURCE}");
    held.keep_through_edits(&edited, 2);
    assert!(held.is_outdated());
    assert_eq!(covered(&held, &edited), ["out", "gbww"]);
    assert_eq!(
        held.marks()
            .iter()
            .map(|mark| mark.line)
            .collect::<Vec<_>>(),
        [1, 2]
    );
    // Typing after both leaves them where they are.
    let edited = format!("{edited}// end\n");
    held.keep_through_edits(&edited, 3);
    assert_eq!(covered(&held, &edited), ["out", "gbww"]);
    // Deleting the offending word takes its finding with it.
    let edited = edited.replace("gbww * ", "");
    held.keep_through_edits(&edited, 4);
    assert_eq!(covered(&held, &edited), ["out"]);
    // The same text again moves nothing.
    held.keep_through_edits(&edited, 5);
    assert_eq!(covered(&held, &edited), ["out"]);
}

#[test]
fn a_deleted_line_takes_its_finding_even_when_its_neighbour_starts_alike() {
    let source = "  vin = 1;\n  vslw = 2;\n  vtarget = 3;\n";
    let collection = CodeDiagnosticCollection::try_new(vec![finding(
        DiagnosticSeverity::Error,
        "undeclared",
        None,
        2,
        3,
    )])
    .expect("a collection");
    let mut held = HeldFindings::default();
    held.take_current(source, 1, 7, &collection.view(Some("amp.va")));
    assert_eq!(covered(&held, source), ["vslw"]);
    // The next line starts with the same indentation and letter, so the
    // deletion could be read as starting inside the finding.
    let deleted = source.replace("  vslw = 2;\n", "");
    held.keep_through_edits(&deleted, 2);
    assert!(held.is_empty());
    // A line typed back in the same place is new text, not the finding.
    held.keep_through_edits(source, 3);
    assert!(held.is_empty());
}

#[test]
fn an_edit_inside_a_finding_keeps_what_is_left_of_it() {
    let mut held = held();
    // A character typed inside the word widens the finding over it.
    let widened = SOURCE.replace("gbww", "gbXww");
    held.keep_through_edits(&widened, 2);
    assert_eq!(covered(&held, &widened), ["out", "gbXww"]);
    // Replacing its end cuts it back to the part that was not replaced.
    let cut = widened.replace("Xww *", "w +");
    held.keep_through_edits(&cut, 3);
    assert_eq!(covered(&held, &cut), ["out", "gb"]);
}

#[test]
fn a_fresh_check_replaces_the_held_findings() {
    let mut held = held();
    let edited = format!("{SOURCE}\n");
    held.keep_through_edits(&edited, 2);
    assert!(held.is_outdated());
    held.take_current(
        &edited,
        2,
        8,
        &CodeDiagnosticCollection::default().view(None),
    );
    assert!(!held.is_outdated());
    assert!(held.is_empty());
    assert!(held.marks().is_empty());
}

#[test]
fn the_caret_asks_about_the_finding_under_it_or_else_its_line() {
    let held = held();
    let char_of = |text: &str| {
        SOURCE[..SOURCE.find(text).expect("on the page")]
            .chars()
            .count()
    };
    let message =
        |finding: Option<&HeldFinding>| finding.map(|finding| finding.message.to_string());
    assert_eq!(
        message(held.at_caret(char_of("bww"))),
        Some("unknown identifier".to_owned())
    );
    // Elsewhere on the line, the line's finding.
    assert_eq!(
        message(held.at_caret(char_of("analog"))),
        Some("unknown identifier".to_owned())
    );
    assert!(held.covering(char_of("analog")).is_empty());
    assert_eq!(message(held.at_caret(char_of("endmodule"))), None);
}

#[test]
fn an_edit_is_found_on_character_boundaries() {
    let edit = TextEdit::between("a µ b", "a µµ b");
    assert!("a µ b".is_char_boundary(edit.start));
    assert!("a µ b".is_char_boundary(edit.old_end));
    assert_eq!(edit.new_end - edit.old_end, 'µ'.len_utf8());
    assert_eq!(
        TextEdit::between("abc", "abc"),
        TextEdit {
            start: 3,
            old_end: 3,
            new_end: 3,
        }
    );
}
