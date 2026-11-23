//! The virtual editor's own tests: text fidelity, folding and the bounded
//! viewport.

use super::*;
use crate::workbench::documents::text_editor_commands::queue_caret_char_index;

pub(super) fn test_style<'a>(
    edited: &'a std::collections::HashSet<usize>,
) -> VirtualEditorStyle<'a> {
    VirtualEditorStyle {
        accessible_label: "Test source editor",
        messages: crate::workbench::MessageCatalog::new(
            crate::workbench::UiTextLocale::EnglishUnitedStates,
        ),
        font: FontId::monospace(11.0),
        line_height: 17.0,
        gutter_width: 47.0,
        code_left_padding: 12.0,
        background: Color32::BLACK,
        hover_background: Color32::from_gray(24),
        active_background: Color32::from_gray(28),
        selection_background: Color32::from_rgb(32, 64, 96),
        text: Color32::WHITE,
        text_dim: Color32::LIGHT_GRAY,
        text_faint: Color32::GRAY,
        border: Color32::DARK_GRAY,
        accent: Color32::LIGHT_BLUE,
        error: Color32::RED,
        warning: Color32::YELLOW,
        edited_lines: edited,
        cross_probe_line: None,
        marks: &[],
        marks_outdated: false,
        occurrences: &[],
    }
}

#[test]
fn newline_normalization_respects_document_policy() {
    assert_eq!(normalize_newlines("a\r\nb\rc\n", "\n"), "a\nb\nc\n");
    assert_eq!(normalize_newlines("a\r\nb\rc\n", "\r\n"), "a\r\nb\r\nc\r\n");
}

#[test]
fn accessible_text_is_whitespace_normalized_and_strictly_bounded() {
    assert_eq!(
        accessibility::bounded_accessible_text(" alpha\n\tbeta ", 32),
        "alpha beta"
    );
    let bounded = accessibility::bounded_accessible_text(&"x".repeat(10_000), 64);
    assert_eq!(bounded.chars().count(), 64);
    assert!(bounded.ends_with('…'));
}

#[test]
fn ime_preedit_is_replaced_instead_of_accumulated() {
    let mut session = VirtualEditorSession::default();
    handle_ime(
        &mut session,
        egui::ImeEvent::Preedit {
            text: "k".to_owned(),
            active_range_chars: Some(0..1),
        },
    );
    handle_ime(
        &mut session,
        egui::ImeEvent::Preedit {
            text: "ka".to_owned(),
            active_range_chars: Some(0..2),
        },
    );
    assert_eq!(session.model.to_source(), "ka");
    handle_ime(&mut session, egui::ImeEvent::Commit("か".to_owned()));
    assert_eq!(session.model.to_source(), "か");
    assert!(session.model.undo());
    assert_eq!(session.model.to_source(), "");
}

#[test]
fn release_text_fidelity_matrix_preserves_large_unicode_ime_clipboard_and_paste_content() {
    let mut large_bytes = Vec::with_capacity(1_000_010);
    large_bytes.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
    large_bytes.extend_from_slice(b"* ");
    large_bytes.extend(std::iter::repeat_n(b'R', 1_000_000));
    large_bytes.extend_from_slice(b"\r\n.end\r\n");
    let large = TextDocumentModel::from_bytes(&large_bytes).expect("large source is valid");
    assert_eq!(
        large.encoding(),
        super::super::text_document_model::TextEncoding::Utf8Bom
    );
    assert_eq!(large.preferred_newline(), "\r\n");
    assert_eq!(large.encoded_bytes(), large_bytes);
    let far_right = large.viewport(0, 1, 999_980, 32, 0, 0);
    assert_eq!(far_right.len(), 1);
    assert!(far_right[0].text.chars().count() <= 32);
    assert_eq!(far_right[0].first_column, 999_980);

    let alpha = '\u{03B1}';
    let beta = '\u{03B2}';
    let mut selected = TextDocumentModel::from_source(&format!("{alpha}\r\n{beta}\r\n"));
    selected.set_selections(vec![
        TextSelection {
            anchor: 0,
            cursor: 1,
        },
        TextSelection {
            anchor: 3,
            cursor: 4,
        },
    ]);
    let context = egui::Context::default();
    let output = context.run_ui(Default::default(), |ui| copy_selection(ui, &selected));
    let expected_copy = format!("{alpha}\r\n{beta}");
    assert!(output.platform_output.commands.iter().any(|command| {
        matches!(
            command,
            egui::OutputCommand::CopyText(text) if text.as_str() == expected_copy
        )
    }));

    let mut session = VirtualEditorSession {
        model: TextDocumentModel::from_source("prefix\r\n"),
        ..VirtualEditorSession::default()
    };
    session
        .model
        .set_selections(vec![TextSelection::caret(session.model.len_chars())]);
    let paste = "\u{4E00}\n\u{4E8C}\r\u{4E09}";
    let keys = KeyContext {
        os: egui::os::OperatingSystem::Windows,
        editable: true,
        syntax: EditorSyntax::Toml,
        page_rows: 20,
        tab_moves_focus: false,
        now: 0.0,
    };
    let _ = context.run_ui(Default::default(), |ui| {
        handle_event(
            ui,
            &mut session,
            egui::Event::Paste(paste.to_owned()),
            &keys,
            &mut InputOutcome::default(),
        );
    });
    assert_eq!(
        session.model.to_source(),
        "prefix\r\n\u{4E00}\r\n\u{4E8C}\r\n\u{4E09}"
    );

    let before_ime = session.model.to_source();
    handle_ime(
        &mut session,
        egui::ImeEvent::Preedit {
            text: "\u{65E5}".to_owned(),
            active_range_chars: Some(0..1),
        },
    );
    handle_ime(
        &mut session,
        egui::ImeEvent::Preedit {
            text: "\u{65E5}\u{672C}".to_owned(),
            active_range_chars: Some(0..2),
        },
    );
    handle_ime(
        &mut session,
        egui::ImeEvent::Commit("\u{65E5}\u{672C}\u{8A9E}\u{1F9EA}".to_owned()),
    );
    assert_eq!(
        session.model.to_source(),
        format!("{before_ime}\u{65E5}\u{672C}\u{8A9E}\u{1F9EA}")
    );
    assert!(session.model.undo());
    assert_eq!(session.model.to_source(), before_ime);

    let invalid = [b'o', b'k', 0xF0, 0x28, 0x8C, 0x28];
    assert_eq!(
        TextDocumentModel::from_bytes(&invalid).unwrap_err(),
        super::super::text_document_model::SourceTextError::InvalidUtf8 { valid_up_to: 2 }
    );
}

#[test]
fn word_navigation_is_unicode_safe() {
    let mut model = TextDocumentModel::from_source("αβ gamma");
    model.set_selections(vec![TextSelection::caret(model.len_chars())]);
    move_horizontal(&mut model, false, false, true);
    assert_eq!(model.selections(), &[TextSelection::caret(3)]);
    move_horizontal(&mut model, false, false, true);
    assert_eq!(model.selections(), &[TextSelection::caret(0)]);
}

#[test]
fn disjoint_multicursor_structural_edit_is_one_transaction() {
    let mut model = TextDocumentModel::from_source("a\nb\nc\n");
    model.set_selections(vec![TextSelection::caret(0), TextSelection::caret(4)]);
    assert!(apply_model_structural(
        &mut model,
        EditorSyntax::Spice,
        StructuralCommand::ToggleComment,
    ));
    assert_eq!(model.to_source(), "* a\nb\n* c\n");
    assert_eq!(model.selections().len(), 2);
    assert!(model.undo());
    assert_eq!(model.to_source(), "a\nb\nc\n");
}

#[test]
fn smart_newline_preserves_line_endings_and_indents_language_blocks() {
    let mut model = TextDocumentModel::from_source("    .subckt amp a b\r\n");
    model.set_selections(vec![TextSelection::caret(19)]);
    assert!(insert_smart_newline(&mut model, EditorSyntax::Spice));
    assert_eq!(model.to_source(), "    .subckt amp a b\r\n        \r\n");
}

#[test]
fn fold_all_discovers_language_regions_and_unfold_all_restores_every_line() {
    let mut session = VirtualEditorSession {
        model: TextDocumentModel::from_source(
            "module gain(in, out);\n  analog begin\n    V(out) <+ V(in);\n  end\nendmodule\nmodule other;\nendmodule\n",
        ),
        ..VirtualEditorSession::default()
    };
    handle_view_command(
        &mut session,
        EditorSyntax::VerilogA,
        EditorViewCommand::FoldAll,
    );
    assert_eq!(session.model.folds().len(), 2);
    assert!(session.model.visible_line_count() < session.model.line_count());
    handle_view_command(
        &mut session,
        EditorSyntax::VerilogA,
        EditorViewCommand::UnfoldAll,
    );
    assert!(session.model.folds().is_empty());
    assert_eq!(
        session.model.visible_line_count(),
        session.model.line_count()
    );
}

#[test]
fn requested_line_unfolds_and_reveals_a_collapsed_target() {
    let context = egui::Context::default();
    let editor_id = egui::Id::new("folded-reveal-editor-test");
    let mut session = VirtualEditorSession {
        model: TextDocumentModel::from_source(".subckt amp in out\nR1 in out 1k\n.ends\n.end\n"),
        ..Default::default()
    };
    assert!(session.model.fold_lines(0, 3));
    let mut source = session.model.snapshot();
    session.bound_revision = source.revision();
    session.maximum_cells = session.model.maximum_line_cells();
    context.data_mut(|data| {
        data.insert_temp(editor_id.with("virtual-session"), session);
    });
    let edited = std::collections::HashSet::new();
    let style = test_style(&edited);
    let _ = context.run_ui(Default::default(), |ui| {
        let _ = show_virtual_text_editor(
            ui,
            editor_id,
            &mut source,
            true,
            EditorSyntax::Spice,
            Some(2),
            &style,
            |_, row, font| {
                egui::text::LayoutJob::simple_singleline(row.text.to_owned(), font, Color32::WHITE)
            },
            |_| None,
            |_| None,
        );
    });
    let session = context
        .data_mut(|data| {
            data.remove_temp::<VirtualEditorSession>(editor_id.with("virtual-session"))
        })
        .expect("retained virtual-editor session");
    assert!(session.model.folds().is_empty());
    assert_eq!(
        session
            .model
            .line_column_for_char(session.model.selections()[0].cursor),
        (1, 0)
    );
}

#[test]
fn toml_fold_discovery_is_deterministic() {
    let toml =
        TextDocumentModel::from_source("title = 'x'\n[first]\na = 1\nb = 2\n[second]\nc = 3\n");
    assert_eq!(
        discover_fold_regions(&toml, EditorSyntax::Toml),
        vec![(1, 4), (4, 7)]
    );
}

/// Two branches of an `if` inside the analog block of a module.
const NESTED_BRANCHES: &str = "module amp(inp, out);
  analog begin
    if (g > 1) begin
      V(out) <+ g * V(inp);
    end else begin
      V(out) <+ V(inp);
    end
  end
endmodule
";

#[test]
fn fold_selection_at_a_caret_folds_the_innermost_block_around_it_then_the_next() {
    let mut session = VirtualEditorSession {
        model: TextDocumentModel::from_source(NESTED_BRANCHES),
        ..VirtualEditorSession::default()
    };
    let caret = session.model.char_at_line_column(2, 8);
    session
        .model
        .set_selections(vec![TextSelection::caret(caret)]);
    let folds = |session: &VirtualEditorSession| {
        session
            .model
            .folds()
            .iter()
            .map(|fold| (fold.start_line, fold.end_line))
            .collect::<Vec<_>>()
    };
    handle_view_command(
        &mut session,
        EditorSyntax::VerilogA,
        EditorViewCommand::FoldSelection,
    );
    assert_eq!(folds(&session), [(2, 4)], "the `if` branch alone");
    handle_view_command(
        &mut session,
        EditorSyntax::VerilogA,
        EditorViewCommand::FoldSelection,
    );
    assert_eq!(folds(&session), [(1, 8)], "then the analog block around it");

    // A caret outside every block folds nothing.
    let mut outside = VirtualEditorSession {
        model: TextDocumentModel::from_source(NESTED_BRANCHES),
        ..VirtualEditorSession::default()
    };
    let end = outside.model.len_chars();
    outside
        .model
        .set_selections(vec![TextSelection::caret(end)]);
    handle_view_command(
        &mut outside,
        EditorSyntax::VerilogA,
        EditorViewCommand::FoldSelection,
    );
    assert!(outside.model.folds().is_empty());
}

#[test]
fn verilog_a_indents_after_a_line_that_opens_a_block_and_only_then() {
    let newline_after = |line: &str| {
        let mut model = TextDocumentModel::from_source(line);
        model.set_selections(vec![TextSelection::caret(model.len_chars())]);
        assert!(insert_smart_newline(&mut model, EditorSyntax::VerilogA));
        model.to_source()
    };
    assert_eq!(newline_after("  analog begin"), "  analog begin\n      ");
    assert_eq!(newline_after("  case (x)"), "  case (x)\n      ");
    assert_eq!(newline_after("`ifdef FAST"), "`ifdef FAST\n    ");
    assert_eq!(newline_after("  `else"), "  `else\n      ");
    assert_eq!(
        newline_after("  end else begin"),
        "  end else begin\n      "
    );
    assert_eq!(newline_after("module m(a);"), "module m(a);\n    ");
    assert_eq!(newline_after("  end"), "  end\n  ");
    assert_eq!(newline_after("  x = 1; // begin"), "  x = 1; // begin\n  ");
    assert_eq!(newline_after("  y = {a, b, {"), "  y = {a, b, {\n  ");
}

#[test]
fn five_million_line_fold_discovery_streams_without_a_line_vector() {
    let source = "x\n".repeat(5_000_000);
    let model = TextDocumentModel::from_source(&source);
    assert!(discover_fold_regions(&model, EditorSyntax::Toml).is_empty());
    assert!(discover_fold_regions(&model, EditorSyntax::Spice).is_empty());
}

#[test]
fn widget_lays_out_only_a_bounded_large_document_viewport() {
    let context = egui::Context::default();
    let mut source = TextSnapshot::new(
        &(0..100_000)
            .map(|line| format!("R{line} n{line} 0 1k\n"))
            .collect::<String>(),
    );
    let original = source.clone();
    let edited = std::collections::HashSet::new();
    let style = test_style(&edited);
    let mut laid_out = 0_usize;
    let mut changed = true;
    let raw_input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            Vec2::new(900.0, 600.0),
        )),
        ..Default::default()
    };
    let _ = context.run_ui(raw_input, |ui| {
        let output = show_virtual_text_editor(
            ui,
            egui::Id::new("large-editor-test"),
            &mut source,
            true,
            EditorSyntax::Spice,
            None,
            &style,
            |_, row, font| {
                laid_out += 1;
                egui::text::LayoutJob::simple_singleline(row.text.to_owned(), font, Color32::WHITE)
            },
            |_| None,
            |_| None,
        );
        changed = output.changed;
    });

    assert!(!changed);
    assert_eq!(source, original);
    assert!(laid_out > 0);
    assert!(laid_out < 80, "laid out {laid_out} rows");
}

#[test]
fn maximum_column_reconciliation_is_revision_bound_and_chunked() {
    let mut source = "x\n".repeat(MAXIMUM_COLUMN_SCAN_LINES_PER_FRAME + 2);
    source.push_str(&"z".repeat(1_000));
    let mut session = VirtualEditorSession {
        model: TextDocumentModel::from_source(&source),
        maximum_cells: 0,
        ..VirtualEditorSession::default()
    };
    restart_maximum_column_scan(&mut session);
    advance_maximum_column_scan(&mut session);
    let first = session
        .maximum_column_scan
        .expect("large document scan remains pending");
    assert_eq!(first.next_line, MAXIMUM_COLUMN_SCAN_LINES_PER_FRAME);
    assert_eq!(session.maximum_cells, 1);
    advance_maximum_column_scan(&mut session);
    assert!(session.maximum_column_scan.is_none());
    assert_eq!(session.maximum_cells, 1_000);

    assert!(session.model.synchronize_source("a\n"));
    restart_maximum_column_scan(&mut session);
    assert_eq!(
        session.maximum_cells, 1_000,
        "the old extent remains usable until the bounded scan completes"
    );
    advance_maximum_column_scan(&mut session);
    assert!(session.maximum_column_scan.is_none());
    assert_eq!(session.maximum_cells, 1);

    let inserted = format!("short\n{}\nend", "q".repeat(2_000));
    grow_maximum_cells_for_insertion(&mut session, &inserted);
    assert!(
        session.maximum_cells >= 2_000,
        "a long intermediate pasted line must grow the canvas immediately"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn widget_exposes_active_line_read_only_and_diagnostic_context_to_accesskit() {
    let context = egui::Context::default();
    context.enable_accesskit();
    crate::ui::Theme::default().apply(&context);
    let mut source = TextSnapshot::from("R1 input output 1k\nV1 input 0 1\n");
    let mut edited = std::collections::HashSet::new();
    edited.insert(1);
    let mut style = test_style(&edited);
    style.accessible_label = "Generated SPICE netlist viewer";
    let raw_input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            Vec2::new(900.0, 600.0),
        )),
        ..Default::default()
    };
    let nodes = context
        .run_ui(raw_input, |ui| {
            let _ = show_virtual_text_editor(
                ui,
                egui::Id::new("accessible-editor-test"),
                &mut source,
                false,
                EditorSyntax::Spice,
                Some(2),
                &style,
                |_, row, font| {
                    egui::text::LayoutJob::simple_singleline(
                        row.text.to_owned(),
                        font,
                        Color32::WHITE,
                    )
                },
                |_| None,
                |line| {
                    (line == 1).then(|| {
                        "One warning diagnostic on this line, TEST-001: Check the source."
                            .to_owned()
                    })
                },
            );
        })
        .platform_output
        .accesskit_update
        .expect("AccessKit editor tree")
        .nodes;

    let editor = nodes
        .iter()
        .find_map(|(_, node)| {
            (node.role() == egui::accesskit::Role::MultilineTextInput
                && node.label() == Some("Generated SPICE netlist viewer"))
            .then_some(node)
        })
        .expect("named multiline editor node");
    let description = editor.description().expect("editor description");
    assert!(description.contains("Line 2 of 3, column 1. Read-only."));
    assert!(description.contains("One warning diagnostic on this line"));
    assert!(description.contains("TEST-001"));
    assert!(description.contains("Modified line"));
    assert!(editor.is_read_only());
    assert!(editor.text_selection().is_some());
    let active_text = nodes
        .iter()
        .find_map(|(_, node)| {
            (node.role() == egui::accesskit::Role::TextRun
                && node.value().map(str::trim_end) == Some("V1 input 0 1"))
            .then_some(node)
        })
        .expect("active-line TextRun");
    let bounds = active_text.bounds().expect("active-line bounds");
    assert!(
        bounds.y0 >= f64::from(style.line_height),
        "line-two accessibility geometry must not overlap line one: {bounds:?}"
    );
}

/// A screen reader is given the caret's line and three lines either side,
/// each row of them where it is drawn; the lines beyond are left out.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn accessible_text_is_the_caret_line_and_three_lines_either_side() {
    let context = egui::Context::default();
    context.enable_accesskit();
    crate::ui::Theme::default().apply(&context);
    let mut source = TextSnapshot::from(
        (1..=20)
            .map(|line| format!("line {line}\n"))
            .collect::<String>(),
    );
    let edited = std::collections::HashSet::new();
    let style = test_style(&edited);
    let nodes = context
        .run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(900.0, 600.0),
                )),
                ..Default::default()
            },
            |ui| {
                let _ = show_virtual_text_editor(
                    ui,
                    egui::Id::new("window-accessible-editor-test"),
                    &mut source,
                    true,
                    EditorSyntax::Spice,
                    Some(10),
                    &style,
                    |_, row, font| {
                        egui::text::LayoutJob::simple_singleline(
                            row.text.to_owned(),
                            font,
                            Color32::WHITE,
                        )
                    },
                    |_| None,
                    |_| None,
                );
            },
        )
        .platform_output
        .accesskit_update
        .expect("AccessKit editor tree")
        .nodes;
    let mut rows = nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::TextRun)
        .map(|(_, node)| {
            (
                node.bounds().expect("a row's bounds").y0,
                node.value().unwrap_or_default().trim_end().to_owned(),
            )
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.0.total_cmp(&right.0));
    assert_eq!(
        rows.iter()
            .map(|(_, text)| text.as_str())
            .collect::<Vec<_>>(),
        [
            "line 7", "line 8", "line 9", "line 10", "line 11", "line 12", "line 13"
        ]
    );
    let pitch = rows[1].0 - rows[0].0;
    assert!(
        (pitch - f64::from(style.line_height)).abs() < 0.5,
        "each line's bounds are where it is drawn: {pitch}"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn accesskit_active_line_payload_is_bounded_for_extreme_source_lines() {
    let context = egui::Context::default();
    context.enable_accesskit();
    crate::ui::Theme::default().apply(&context);
    let mut source = TextSnapshot::from("x".repeat(50_000));
    let edited = std::collections::HashSet::new();
    let style = test_style(&edited);
    let nodes = context
        .run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(900.0, 600.0),
                )),
                ..Default::default()
            },
            |ui| {
                let _ = show_virtual_text_editor(
                    ui,
                    egui::Id::new("bounded-accessible-editor-test"),
                    &mut source,
                    true,
                    EditorSyntax::Spice,
                    None,
                    &style,
                    |_, row, font| {
                        egui::text::LayoutJob::simple_singleline(
                            row.text.to_owned(),
                            font,
                            Color32::WHITE,
                        )
                    },
                    |_| None,
                    |_| None,
                );
            },
        )
        .platform_output
        .accesskit_update
        .expect("AccessKit editor tree")
        .nodes;
    let exposed_characters = nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::TextRun)
        .map(|(_, node)| node.value().unwrap_or_default().chars().count())
        .sum::<usize>();
    assert_eq!(exposed_characters, MAX_ACCESSIBLE_LINE_CHARS);
    assert!(nodes.iter().any(|(_, node)| {
        node.description()
            .is_some_and(|description| description.contains("of 50000"))
    }));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn accesskit_excerpt_geometry_tracks_the_real_horizontal_source_column() {
    let context = egui::Context::default();
    context.enable_accesskit();
    crate::ui::Theme::default().apply(&context);
    let editor_id = egui::Id::new("horizontal-accessible-editor-test");
    let mut session = VirtualEditorSession {
        model: TextDocumentModel::from_source(&"x".repeat(2_000)),
        ..Default::default()
    };
    session
        .model
        .set_selections(vec![TextSelection::caret(1_500)]);
    let mut source = session.model.snapshot();
    session.bound_revision = source.revision();
    session.maximum_cells = session.model.maximum_line_cells();
    context.data_mut(|data| {
        data.insert_temp(editor_id.with("virtual-session"), session);
    });
    let edited = std::collections::HashSet::new();
    let style = test_style(&edited);
    let nodes = context
        .run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(900.0, 600.0),
                )),
                ..Default::default()
            },
            |ui| {
                let _ = show_virtual_text_editor(
                    ui,
                    editor_id,
                    &mut source,
                    true,
                    EditorSyntax::Spice,
                    None,
                    &style,
                    |_, row, font| {
                        egui::text::LayoutJob::simple_singleline(
                            row.text.to_owned(),
                            font,
                            Color32::WHITE,
                        )
                    },
                    |_| None,
                    |_| None,
                );
            },
        )
        .platform_output
        .accesskit_update
        .expect("AccessKit editor tree")
        .nodes;
    let text_run = nodes
        .iter()
        .find_map(|(_, node)| (node.role() == egui::accesskit::Role::TextRun).then_some(node))
        .expect("active-line excerpt TextRun");
    let bounds = text_run.bounds().expect("active-line excerpt bounds");
    assert!(
        bounds.x0 > 1_000.0,
        "far-right source text must retain far-right geometry: {bounds:?}"
    );
    assert!(nodes.iter().any(|(_, node)| {
        node.description()
            .is_some_and(|description| description.contains("Source excerpt columns"))
    }));
}

/// The editor driven a frame at a time over a text its owner keeps, the
/// way a page drives it, with the keyboard focused on it.
struct ScriptedEditor {
    context: egui::Context,
    id: egui::Id,
    source: TextSnapshot,
    time: f64,
    /// Seconds from one frame to the next.
    frame_gap: f64,
}

/// What a frame left: whether the text changed and where the caret is.
struct ScriptedFrame {
    changed: bool,
    cursor: usize,
}

impl ScriptedEditor {
    fn new(source: &str) -> Self {
        let context = egui::Context::default();
        crate::ui::Theme::default().apply(&context);
        context.set_os(egui::os::OperatingSystem::Windows);
        let mut editor = Self {
            context,
            id: egui::Id::new("scripted-editor"),
            source: TextSnapshot::new(source),
            time: 0.0,
            frame_gap: 1.0 / 60.0,
        };
        editor
            .context
            .memory_mut(|memory| memory.request_focus(editor.id));
        let _ = editor.frame(Vec::new());
        let _ = editor.frame(Vec::new());
        editor
    }

    fn frame(&mut self, events: Vec<egui::Event>) -> ScriptedFrame {
        self.time += self.frame_gap;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(900.0, 600.0),
            )),
            time: Some(self.time),
            events,
            focused: true,
            ..Default::default()
        };
        let edited = std::collections::HashSet::new();
        let style = test_style(&edited);
        let (id, source) = (self.id, &mut self.source);
        let mut frame = None;
        let _ = self.context.run_ui(input, |ui| {
            let output = show_virtual_text_editor(
                ui,
                id,
                source,
                true,
                EditorSyntax::Spice,
                None,
                &style,
                |_, row, font| {
                    egui::text::LayoutJob::simple_singleline(
                        row.text.to_owned(),
                        font,
                        Color32::WHITE,
                    )
                },
                |_| None,
                |_| None,
            );
            frame = Some(ScriptedFrame {
                changed: output.changed,
                cursor: output.cursor_char_index,
            });
        });
        frame.expect("the editor was drawn")
    }

    /// A key pressed in one frame and released in the next; what the press
    /// did.
    fn key(&mut self, key: Key, modifiers: Modifiers) -> ScriptedFrame {
        let event = |pressed| egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed,
            repeat: false,
            modifiers,
        };
        let pressed = self.frame(vec![event(true)]);
        let _ = self.frame(vec![event(false)]);
        pressed
    }
}

/// A small, seeded source of choices, so a failing script can be run again.
struct Choices(u64);

impl Choices {
    fn below(&mut self, bound: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % bound.max(1) as u64) as usize
    }
}

/// The byte where character `index` of `text` starts.
fn byte_at(text: &str, index: usize) -> usize {
    text.char_indices()
        .nth(index)
        .map_or(text.len(), |(byte, _)| byte)
}

/// A text an edit script passed through: the caret the edit that made it
/// left, and the selection the next edit from it started with.
struct ScriptState {
    text: String,
    reached: usize,
    left_from: (usize, Option<usize>),
}

/// An opening bracket or quote typed before a blank or the line's end
/// brings its closing one, the caret between; the closing one typed there
/// steps over it, and Backspace between an empty pair takes both.
#[test]
fn typing_an_opening_bracket_brings_its_closing_one() {
    let mut editor = ScriptedEditor::new("");
    for typed in ["V", "(", "a"] {
        assert!(
            editor
                .frame(vec![egui::Event::Text(typed.to_owned())])
                .changed
        );
    }
    assert!(editor.source == "V(a)", "{}", String::from(&editor.source));
    let _ = editor.frame(vec![egui::Event::Text(")".to_owned())]);
    assert!(
        editor.source == "V(a)",
        "the closing bracket is stepped over"
    );
    assert_eq!(editor.frame(Vec::new()).cursor, 4);
    let _ = editor.frame(vec![egui::Event::Text(" {".to_owned())]);
    assert!(editor.source == "V(a) {", "pasted text comes as it is");
    for typed in [" ", "'"] {
        let _ = editor.frame(vec![egui::Event::Text(typed.to_owned())]);
    }
    assert!(
        editor.source == "V(a) { ''",
        "{}",
        String::from(&editor.source)
    );
    assert!(editor.key(Key::Backspace, Modifiers::NONE).changed);
    assert!(editor.source == "V(a) { ", "Backspace takes the empty pair");
    for typed in ["x", "'"] {
        let _ = editor.frame(vec![egui::Event::Text(typed.to_owned())]);
    }
    assert!(
        editor.source == "V(a) { x'",
        "a quote after a word is one quote"
    );
}

/// An opening bracket typed over a selection wraps it and keeps it selected.
#[test]
fn an_opening_bracket_wraps_the_selection() {
    let mut editor = ScriptedEditor::new("R1 a b 1k");
    let _ = editor.key(Key::A, Modifiers::COMMAND);
    assert!(
        editor
            .frame(vec![egui::Event::Text("[".to_owned())])
            .changed
    );
    assert!(
        editor.source == "[R1 a b 1k]",
        "{}",
        String::from(&editor.source)
    );
    assert!(editor.key(Key::Z, Modifiers::COMMAND).changed);
    assert!(editor.source == "R1 a b 1k", "the wrap is one step");
}

/// Keys typed one after another are undone together, up to the space
/// before the word.
#[test]
fn typing_without_a_pause_is_undone_a_word_at_a_time() {
    let mut editor = ScriptedEditor::new("");
    for typed in ["R", "1", " ", "a", "b"] {
        assert!(
            editor
                .frame(vec![egui::Event::Text(typed.to_owned())])
                .changed
        );
    }
    assert!(editor.key(Key::Backspace, Modifiers::NONE).changed);
    assert!(editor.source == "R1 a", "{}", String::from(&editor.source));
    assert!(editor.key(Key::Z, Modifiers::COMMAND).changed);
    assert!(
        editor.source == "R1 ab",
        "the deletion is a step of its own"
    );
    assert!(editor.key(Key::Z, Modifiers::COMMAND).changed);
    assert!(editor.source == "R1", "the word goes with its space");
    assert!(editor.key(Key::Z, Modifiers::COMMAND).changed);
    assert!(editor.source == "", "{}", String::from(&editor.source));
}

/// Random edit scripts on a large text leave the editor's text, its caret
/// and its undo history exactly where the same edits leave a plain string:
/// typing, pasting lines, deleting either way, replacing a selection, undo
/// and redo, and the owner putting a new text in.
#[test]
fn random_edits_on_a_large_buffer_keep_text_caret_and_undo_as_a_string_does() {
    let base = (0..20_000)
        .map(|line| format!("R{line} n{line} n{} 1k\n", line + 1))
        .collect::<String>();
    for seed in [0x9E37_79B9_7F4A_7C15_u64, 0xD1B5_4A32_D192_ED03] {
        let mut choices = Choices(seed);
        let mut editor = ScriptedEditor::new(&base);
        // Each keystroke a pause after the last, so each is a step of its
        // own, as the string model below counts them.
        editor.frame_gap = 2.0;
        let mut text = base.clone();
        let mut cursor = 0_usize;
        let mut anchor = None::<usize>;
        let mut states = vec![ScriptState {
            text: text.clone(),
            reached: 0,
            left_from: (0, None),
        }];
        let mut at = 0_usize;
        for step in 0..240 {
            let length = text.chars().count();
            let what = choices.below(10);
            let context = format!("seed {seed:#x}, step {step}, action {what}");
            // The span an edit replaces, and the selection it starts from as
            // the editor records it for undo.
            let (start, end, left_from) = match (what, anchor) {
                (_, Some(anchor)) => (
                    anchor.min(cursor),
                    anchor.max(cursor),
                    (cursor, Some(anchor)),
                ),
                (5, None) if cursor > 0 => (cursor - 1, cursor, (cursor, Some(cursor - 1))),
                (6, None) if cursor < length => (cursor, cursor + 1, (cursor + 1, Some(cursor))),
                _ => (cursor, cursor, (cursor, None)),
            };
            let inserted = match what {
                1..=3 => ["x", "1k", " ", "\u{3A9}", "R9 a b"][choices.below(5)],
                4 => "\nC1 p q 1p\n",
                _ => "",
            };
            let edits = match what {
                1..=4 => {
                    let event = if what == 4 {
                        egui::Event::Paste(inserted.to_owned())
                    } else {
                        egui::Event::Text(inserted.to_owned())
                    };
                    editor.frame(vec![event]).changed
                }
                5 | 6 => {
                    let key = if what == 6 {
                        Key::Delete
                    } else {
                        Key::Backspace
                    };
                    let changed = editor.key(key, Modifiers::NONE).changed;
                    assert_eq!(changed, start < end, "{context}: what a delete deletes");
                    changed
                }
                _ => false,
            };
            match what {
                _ if edits => {
                    let range = byte_at(&text, start)..byte_at(&text, end);
                    text.replace_range(range, inserted);
                    cursor = start + inserted.chars().count();
                    anchor = None;
                    states[at].left_from = left_from;
                    states.truncate(at + 1);
                    states.push(ScriptState {
                        text: text.clone(),
                        reached: cursor,
                        left_from: (cursor, None),
                    });
                    at += 1;
                }
                0 => {
                    cursor = choices.below(length + 1);
                    anchor = None;
                    queue_caret_char_index(&editor.context, editor.id, cursor);
                    assert!(!editor.frame(Vec::new()).changed, "{context}");
                }
                1..=4 => panic!("{context}: typing changed nothing"),
                7 => {
                    let reach = 1 + choices.below(4);
                    let from = anchor.unwrap_or(cursor);
                    for _ in 0..reach {
                        let _ = editor.key(Key::ArrowRight, Modifiers::SHIFT);
                    }
                    cursor = (cursor + reach).min(length);
                    anchor = (from != cursor).then_some(from);
                }
                8 => {
                    if editor.key(Key::Z, Modifiers::COMMAND).changed {
                        at = states[..at]
                            .iter()
                            .rposition(|state| editor.source == state.text)
                            .unwrap_or_else(|| {
                                panic!("{context}: undo reached a text no edit left")
                            });
                        text.clone_from(&states[at].text);
                        (cursor, anchor) = states[at].left_from;
                    } else {
                        assert_eq!(at, 0, "{context}: undo had a step to take");
                    }
                }
                9 => {
                    if editor.key(Key::Y, Modifiers::COMMAND).changed {
                        at += 1 + states[at + 1..]
                            .iter()
                            .position(|state| editor.source == state.text)
                            .unwrap_or_else(|| {
                                panic!("{context}: redo reached a text no edit left")
                            });
                        text.clone_from(&states[at].text);
                        cursor = states[at].reached;
                        anchor = None;
                    } else {
                        assert_eq!(at + 1, states.len(), "{context}: redo had a step to take");
                    }
                }
                _ => {}
            }
            assert!(
                editor.source == text,
                "{context}: the text differs from the string's"
            );
            let shown = editor.frame(Vec::new());
            assert_eq!(shown.cursor, cursor, "{context}: the caret");
            assert!(
                !shown.changed,
                "{context}: a frame with no input edits nothing"
            );
        }

        // Everything undone is the text the script started from, and
        // everything redone is where it ended.
        let end = text.clone();
        while editor.key(Key::Z, Modifiers::COMMAND).changed {}
        assert!(editor.source == base, "seed {seed:#x}: undo all");
        while editor.key(Key::Y, Modifiers::COMMAND).changed {}
        assert!(editor.source == end, "seed {seed:#x}: redo all");

        // A text the owner puts in replaces the editor's, history and all.
        let half = end
            .chars()
            .take(end.chars().count() / 2)
            .collect::<String>();
        let replaced = format!("{half}* appended by the owner\n");
        editor.source = TextSnapshot::new(&replaced);
        assert!(!editor.frame(Vec::new()).changed);
        assert!(editor.source == replaced);
        assert!(!editor.key(Key::Z, Modifiers::COMMAND).changed);
        assert!(
            editor.source == replaced,
            "the owner's text is not undone by the editor"
        );
    }
}

/// An owner keeps the sessions of the documents it has open and no others:
/// closing one drops its text, history and caret, and the count of sessions
/// goes back to what it was before the document opened.
#[test]
fn a_closed_document_releases_its_editor_session() {
    let mut first = ScriptedEditor::new("R1 a b 1k\n");
    let owner = egui::Id::new("scripted-owner");
    let open = [first.id];
    assert!(retain_sessions(&first.context, owner, &open).is_empty());
    let before = session_count(&first.context);

    // A second document opens beside the first and takes an edit.
    let second = egui::Id::new("scripted-second");
    let kept = (first.id, first.source.clone());
    first.id = second;
    first.source = TextSnapshot::new("C1 p q 1p\n");
    first
        .context
        .memory_mut(|memory| memory.request_focus(second));
    let _ = first.frame(Vec::new());
    let _ = first.frame(Vec::new());
    assert!(first.frame(vec![egui::Event::Text("x".to_owned())]).changed);
    assert!(
        retain_sessions(&first.context, owner, &[kept.0, second]).is_empty(),
        "both documents are open"
    );
    assert_eq!(session_count(&first.context), before + 1);

    // Closing it releases its session and nothing else.
    assert_eq!(retain_sessions(&first.context, owner, &open), vec![second]);
    assert_eq!(session_count(&first.context), before);
    assert!(shown_source(&first.context, second).is_none());
    assert_eq!(
        shown_source(&first.context, kept.0).as_deref(),
        Some("R1 a b 1k\n"),
        "the document still open keeps its session"
    );
}

/// A structural command read over the whole text: the reference the editor's
/// reading of only the lines a command touches must match.
fn structural_over_whole_text(
    model: &TextDocumentModel,
    syntax: EditorSyntax,
    command: StructuralCommand,
) -> (String, Vec<TextSelection>) {
    let mut source = model.to_source();
    if command == StructuralCommand::MatchBracket {
        let selection = model
            .selections()
            .last()
            .copied()
            .unwrap_or(TextSelection::caret(model.len_chars()));
        let result = apply_structural_command(
            &mut source,
            ByteSelection {
                anchor: model.char_to_byte(selection.anchor),
                cursor: model.char_to_byte(selection.cursor),
            },
            syntax,
            command,
        );
        let mapped = TextSelection {
            anchor: byte_to_char_index(&source, result.selection.anchor),
            cursor: byte_to_char_index(&source, result.selection.cursor),
        };
        return (source, vec![mapped]);
    }
    let mut groups = structural_line_groups(model);
    if matches!(
        command,
        StructuralCommand::MoveLineUp | StructuralCommand::MoveLineDown
    ) && groups.len() > 1
    {
        let combined = groups[0].start..groups.last().unwrap().end;
        groups = vec![combined];
    }
    let line_start = |line: usize| {
        if line < model.line_count() {
            model.char_at_line_column(line, 0)
        } else {
            model.len_chars()
        }
    };
    let mut mapped_byte_selections = Vec::new();
    for group in groups.into_iter().rev() {
        let selection = ByteSelection {
            anchor: model.char_to_byte(line_start(group.start)),
            cursor: model.char_to_byte(line_start(group.end)),
        };
        let before_len = source.len();
        let result = apply_structural_command(&mut source, selection, syntax, command);
        let delta = source.len() as isize - before_len as isize;
        let changed_from = selection
            .anchor
            .min(selection.cursor)
            .min(result.selection.anchor.min(result.selection.cursor));
        if delta != 0 {
            for mapped in &mut mapped_byte_selections {
                let mapped: &mut ByteSelection = mapped;
                mapped.anchor = shift_byte_after(mapped.anchor, changed_from, delta);
                mapped.cursor = shift_byte_after(mapped.cursor, changed_from, delta);
            }
        }
        mapped_byte_selections.push(result.selection);
    }
    mapped_byte_selections.reverse();
    let mapped = if matches!(
        command,
        StructuralCommand::Indent | StructuralCommand::Outdent
    ) {
        selections_after_reindent(model, 0, 0, &source)
    } else {
        mapped_byte_selections
            .into_iter()
            .map(|selection| TextSelection {
                anchor: byte_to_char_index(&source, selection.anchor),
                cursor: byte_to_char_index(&source, selection.cursor),
            })
            .collect()
    };
    (source, mapped)
}

/// Every structural command, rewriting only the lines it touches, leaves the
/// text and the selections exactly as the same command over the whole text
/// does: at the start and the end of a document, with and without a final
/// line break, with several selections, and in SPICE and Verilog-A.
#[test]
fn structural_commands_on_the_lines_they_touch_match_the_whole_text() {
    const LINES: [&str; 8] = [
        "R1 a b 1k",
        "    C1 a 0 {cap(1)}",
        "* a comment",
        "",
        "  analog begin [x] (y)",
        "\tend",
        "// done",
        "X1 (a b) sub",
    ];
    const COMMANDS: [StructuralCommand; 8] = [
        StructuralCommand::Indent,
        StructuralCommand::Outdent,
        StructuralCommand::ToggleComment,
        StructuralCommand::DuplicateLine,
        StructuralCommand::DeleteLine,
        StructuralCommand::MoveLineUp,
        StructuralCommand::MoveLineDown,
        StructuralCommand::MatchBracket,
    ];
    let mut choices = Choices(0x2545_F491_4F6C_DD1D);
    for case in 0..600 {
        let count = 1 + choices.below(9);
        let newline = if choices.below(3) == 0 { "\r\n" } else { "\n" };
        let mut text = (0..count)
            .map(|_| LINES[choices.below(LINES.len())])
            .collect::<Vec<_>>()
            .join(newline);
        if choices.below(2) == 0 {
            text.push_str(newline);
        }
        let syntax = if choices.below(2) == 0 {
            EditorSyntax::Spice
        } else {
            EditorSyntax::VerilogA
        };
        let command = COMMANDS[choices.below(COMMANDS.len())];
        let mut model = TextDocumentModel::from_source(&text);
        let length = model.len_chars();
        let selections = (0..1 + choices.below(3))
            .map(|_| TextSelection {
                anchor: choices.below(length + 1),
                cursor: choices.below(length + 1),
            })
            .collect();
        model.set_selections(selections);
        let (expected_text, expected_selections) =
            structural_over_whole_text(&model, syntax, command);
        let changed = apply_model_structural(&mut model, syntax, command);
        let context = format!("case {case}: {command:?} in {syntax:?} over {text:?}");
        assert_eq!(model.to_source(), expected_text, "{context}");
        assert_eq!(changed, expected_text != text, "{context}");
        let mut expected = model.clone();
        expected.set_selections(expected_selections);
        assert_eq!(model.selections(), expected.selections(), "{context}");
    }
}

/// Draw the editor `id` over `source` once in `context`, and say what it
/// shows of its document.
fn drawn_view(
    context: &egui::Context,
    id: egui::Id,
    source: &mut TextSnapshot,
    style: &VirtualEditorStyle<'_>,
) -> EditorView {
    let mut view = None;
    let raw_input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            Vec2::new(900.0, 600.0),
        )),
        ..Default::default()
    };
    let _ = context.run_ui(raw_input, |ui| {
        let output = show_virtual_text_editor(
            ui,
            id,
            source,
            true,
            EditorSyntax::Spice,
            None,
            style,
            |_, row, font| {
                egui::text::LayoutJob::simple_singleline(row.text.to_owned(), font, Color32::WHITE)
            },
            |_| None,
            |_| None,
        );
        view = Some(output.view);
    });
    view.expect("the editor draws")
}

/// A view taken from one editor and handed to a new one, as the next launch
/// hands it, shows the document as it was left: the selection, what is
/// folded, how far it is scrolled and what the find bar searches for.
#[test]
fn a_view_handed_back_shows_the_document_as_it_was_left() {
    let text = format!(
        ".subckt amp a b\nR1 a b 1k\n.ends\n{}",
        (0..400)
            .map(|line| format!("R{line} n{line} 0 1k\n"))
            .collect::<String>()
    );
    let edited = std::collections::HashSet::new();
    let style = test_style(&edited);
    let id = egui::Id::new("handed-back-view-test");
    let left = egui::Context::default();
    let mut source = TextSnapshot::new(&text);
    drawn_view(&left, id, &mut source, &style);
    // Far down the document, with "R300" selected on its line.
    let selected = text.find("R300 ").expect("the line is there");
    queue_caret_char_index(&left, id, selected);
    drawn_view(&left, id, &mut source, &style);
    let session_id = id.with("virtual-session");
    left.data_mut(|data| {
        let mut session = data
            .remove_temp::<VirtualEditorSession>(session_id)
            .expect("the editor keeps its session");
        session.model.fold_lines(0, 3);
        session.model.set_selections(vec![TextSelection {
            anchor: selected,
            cursor: selected + 4,
        }]);
        data.insert_temp(session_id, session);
    });
    queue_find_bar(&left, id, false);
    drawn_view(&left, id, &mut source, &style);
    let view = drawn_view(&left, id, &mut source, &style);
    assert_eq!(view.folded(), 1, "{view:?}");
    assert_eq!(view.caret(), Some(selected + 4));
    assert!(view.scrolled_down() > 0.0, "{view:?}");
    assert_eq!(view.searching_for(), Some("R300"));

    let next = egui::Context::default();
    let mut source = TextSnapshot::new(&text);
    queue_view(&next, id, view.clone());
    drawn_view(&next, id, &mut source, &style);
    let shown = drawn_view(&next, id, &mut source, &style);

    assert_eq!(shown, view);
}
