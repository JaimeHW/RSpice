//! Find references and Rename over a deck and the file it includes, as the
//! editor's commands answer them.

use super::super::fixtures::{DECK_INCLUDE, seed_deck};
use super::*;
use crate::workbench::code::tests::{SourceHarness, harness};

/// A deck reading `root`, which includes the fixture's include reading
/// `include`, open beside it. The editor shows the deck.
fn deck(root: &str, include: &str) -> SourceHarness {
    let mut harness = harness();
    seed_deck(
        &mut harness.model,
        &mut harness.state.netlist,
        root,
        include,
    );
    harness
}

/// The text of the open deck's file `include`, the deck itself for `None`.
fn file_text(harness: &SourceHarness, include: Option<&str>) -> String {
    harness
        .state
        .netlist
        .open_deck
        .as_ref()
        .and_then(|deck| deck.file(include))
        .map(|file| file.text.clone())
        .expect("the file is open")
}

/// Put the caret on the start of `word` in the shown text, after `after`.
fn caret_on(harness: &mut SourceHarness, after: &str, word: &str) {
    let text = &harness.model.simulation.source.netlist_content;
    let from = text.find(after).expect("the text holds the anchor");
    let byte = from
        + text[from..]
            .find(word)
            .expect("the word follows the anchor");
    harness.state.netlist.cursor_char_index = text[..byte].chars().count();
}

/// The file, line and text of each row the References page lists.
fn listed(harness: &SourceHarness) -> Vec<(String, Option<String>, usize, String)> {
    harness
        .wb
        .source_references
        .as_ref()
        .expect("the References page holds a list")
        .rows
        .iter()
        .map(|row| {
            (
                row.logical_path.clone(),
                row.include.clone(),
                row.line,
                row.text.clone(),
            )
        })
        .collect()
}

const INCLUDE_INCLUDE: &str = ".include \"models/resistor.inc\"\n";

#[test]
fn a_parameter_is_found_on_a_continuation_line_and_in_the_file_the_deck_includes() {
    let root = format!(
        "amp_tb\n{INCLUDE_INCLUDE}.param w=2u\nM1 d g 0 0 nch L=1u\n+ W={{w}}\nX1 a b cell\n.end\n"
    );
    let include = ".subckt cell a b\nR1 a b {w*1k}\n.ends cell\n";
    let mut harness = deck(&root, include);
    caret_on(&mut harness, ".param", "w=");

    let said = find_references(&harness.model, &mut harness.state.netlist, &mut harness.wb)
        .expect("the parameter has places");

    let rows = listed(&harness);
    let root_name = rows[0].0.clone();
    assert_eq!(
        rows,
        [
            (root_name.clone(), None, 3, ".param w=2u".to_owned()),
            (root_name, None, 5, "+ W={w}".to_owned()),
            (
                "resistor.inc".to_owned(),
                Some(DECK_INCLUDE.to_owned()),
                2,
                "R1 a b {w*1k}".to_owned()
            ),
        ]
    );
    assert_eq!(said, "Parameter w is written in 3 places in 2 files.");

    // Following the include's row opens the include on that line.
    let ctx = egui::Context::default();
    let row = harness.wb.source_references.as_ref().unwrap().rows[2].clone();
    open_deck_place(
        &ctx,
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        &mut harness.wb,
        (row.include.as_deref(), row.line, row.column),
    )
    .expect("the include opens at the row");
    assert_eq!(harness.state.netlist.front.include(), Some(DECK_INCLUDE));
    assert_eq!(harness.state.netlist.cursor_line, 1);
}

#[test]
fn renaming_a_subcircuit_lists_each_line_in_both_files_and_undoes_a_file_at_a_time() {
    let root = format!("amp_tb\n{INCLUDE_INCLUDE}X1 a b cell\nX2 b 0 cell\n.end\n");
    let include = ".subckt cell a b\nR1 a b 1k\n.ends cell\n";
    let mut harness = deck(&root, include);
    caret_on(&mut harness, "X1", "cell");

    let mut rename = DeckRename::at_caret(
        &harness.model,
        &mut harness.state.netlist,
        harness.wb.messages(),
    )
    .expect("the subcircuit can be renamed");
    rename.replacement = "core".to_owned();
    let plan = rename.plan().clone().expect("nothing refuses");
    let rows = plan
        .rows
        .iter()
        .map(|row| (row.line, row.before.as_str(), row.after.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(
        rows,
        [
            (3, "X1 a b cell", "X1 a b core"),
            (4, "X2 b 0 cell", "X2 b 0 core"),
            (1, ".subckt cell a b", ".subckt core a b"),
            (3, ".ends cell", ".ends core"),
        ]
    );
    assert_eq!(plan.rows[2].file, "resistor.inc");
    assert_eq!((plan.places, plan.file_count()), (4, 2));

    commit_rename(&mut harness.model, &mut harness.state.netlist, &mut rename)
        .expect("the rename is made");
    let renamed_root = root.replace("cell", "core");
    assert_eq!(file_text(&harness, None), renamed_root);
    assert_eq!(
        file_text(&harness, Some(DECK_INCLUDE)),
        ".subckt core a b\nR1 a b 1k\n.ends core\n"
    );

    // One Undo takes back one file; the next, the other.
    let undo = |harness: &mut SourceHarness| {
        super::super::undo_netlist_edit(
            &mut harness.model,
            &mut harness.state.netlist,
            crate::workbench::MessageCatalog::new(
                crate::workbench::UiTextLocale::EnglishUnitedStates,
            ),
        )
        .expect("the step is taken back")
        .expect("there is a step")
    };
    assert_eq!(undo(&mut harness), "rename cell to core");
    assert_eq!(
        (
            file_text(&harness, None),
            file_text(&harness, Some(DECK_INCLUDE))
        ),
        (renamed_root, include.to_owned()),
        "the first Undo takes back the include alone"
    );
    undo(&mut harness);
    assert_eq!(file_text(&harness, None), root);
}

#[test]
fn a_rename_refuses_a_name_written_inside_a_string_and_names_its_line() {
    let root =
        format!("amp_tb\n{INCLUDE_INCLUDE}.param w=2u\nR1 a 0 {{w}}\n.echo \"sweeping w\"\n.end\n");
    let mut harness = deck(&root, "* models\n");
    caret_on(&mut harness, ".param", "w=");
    let mut rename = DeckRename::at_caret(
        &harness.model,
        &mut harness.state.netlist,
        harness.wb.messages(),
    )
    .expect("the parameter can be named");
    rename.replacement = "width".to_owned();
    let refusal = rename.plan().clone().expect_err("it refuses");
    assert!(
        refusal.contains("line 5") && refusal.contains("inside a string"),
        "{refusal}"
    );
}

#[test]
fn a_rename_to_a_name_a_reader_already_sees_is_refused() {
    let root = format!(
        "amp_tb\n{INCLUDE_INCLUDE}.param w=1\n.param l=2\nR1 a 0 {{w*l}}\n\
         .subckt cell a b\n.param k=3\nR2 a b {{w*k}}\n.ends cell\n.end\n"
    );
    let mut harness = deck(&root, "* models\n");
    caret_on(&mut harness, ".param w", "w=");
    let mut rename = DeckRename::at_caret(
        &harness.model,
        &mut harness.state.netlist,
        harness.wb.messages(),
    )
    .expect("the parameter can be named");

    rename.replacement = "l".to_owned();
    let refusal = rename.plan().clone().expect_err("l is defined");
    assert!(refusal.contains("line 4"), "{refusal}");

    // `k` is not defined where `w` is, but a read of `w` inside the
    // subcircuit would bind the subcircuit's own `k`.
    rename.replacement = "k".to_owned();
    let refusal = rename.plan().clone().expect_err("k shadows");
    assert!(refusal.contains("line 7"), "{refusal}");

    rename.replacement = "width".to_owned();
    assert!(rename.plan().is_ok());
}
