//! The virtual editor's own tests: text fidelity, folding and the bounded
//! viewport.

use super::*;

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
        top_padding: 8.0,
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
        breakpoints: &[],
        breakpoints_enabled: false,
        current_line: None,
        cross_probe_line: None,
        marks: &[],
        marks_outdated: false,
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
        syntax: EditorSyntax::Python,
        page_rows: 20,
        tab_moves_focus: false,
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
    let mut model = TextDocumentModel::from_source("    if ready:\r\n");
    model.set_selections(vec![TextSelection::caret(13)]);
    assert!(insert_smart_newline(&mut model, EditorSyntax::Python));
    assert_eq!(model.to_source(), "    if ready:\r\n        \r\n");
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
    let mut source = ".subckt amp in out\nR1 in out 1k\n.ends\n.end\n".to_owned();
    let mut session = VirtualEditorSession {
        model: TextDocumentModel::from_source(&source),
        ..Default::default()
    };
    assert!(session.model.fold_lines(0, 3));
    session.bound_revision = 1;
    session.maximum_columns = session.model.maximum_line_columns();
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
            1,
            true,
            EditorSyntax::Spice,
            Some(2),
            &style,
            |ui, row, font| {
                ui.fonts_mut(|fonts| {
                    fonts.layout_no_wrap(row.text.to_owned(), font, Color32::WHITE)
                })
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
fn indentation_and_toml_fold_discovery_are_deterministic() {
    let python = TextDocumentModel::from_source(
        "def outer():\n    if ready:\n        return 1\n    return 0\nnext_value = 2\n",
    );
    assert_eq!(
        discover_fold_regions(&python, EditorSyntax::Python),
        vec![(0, 4), (1, 3)]
    );
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
    assert!(discover_fold_regions(&model, EditorSyntax::Python).is_empty());
    assert!(discover_fold_regions(&model, EditorSyntax::Spice).is_empty());
}

#[test]
fn widget_lays_out_only_a_bounded_large_document_viewport() {
    let context = egui::Context::default();
    let mut source = (0..100_000)
        .map(|line| format!("R{line} n{line} 0 1k\n"))
        .collect::<String>();
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
            1,
            true,
            EditorSyntax::Spice,
            None,
            &style,
            |ui, row, font| {
                laid_out += 1;
                ui.fonts_mut(|fonts| {
                    fonts.layout_no_wrap(row.text.to_owned(), font, Color32::WHITE)
                })
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
        maximum_columns: 0,
        ..VirtualEditorSession::default()
    };
    restart_maximum_column_scan(&mut session);
    advance_maximum_column_scan(&mut session);
    let first = session
        .maximum_column_scan
        .expect("large document scan remains pending");
    assert_eq!(first.next_line, MAXIMUM_COLUMN_SCAN_LINES_PER_FRAME);
    assert_eq!(session.maximum_columns, 1);
    advance_maximum_column_scan(&mut session);
    assert!(session.maximum_column_scan.is_none());
    assert_eq!(session.maximum_columns, 1_000);

    assert!(session.model.synchronize_source("a\n"));
    restart_maximum_column_scan(&mut session);
    assert_eq!(
        session.maximum_columns, 1_000,
        "the old extent remains usable until the bounded scan completes"
    );
    advance_maximum_column_scan(&mut session);
    assert!(session.maximum_column_scan.is_none());
    assert_eq!(session.maximum_columns, 1);

    let inserted = format!("short\n{}\nend", "q".repeat(2_000));
    grow_maximum_columns_for_insertion(&mut session, &inserted);
    assert!(
        session.maximum_columns >= 2_000,
        "a long intermediate pasted line must grow the canvas immediately"
    );
}

#[test]
fn focused_debug_editor_exposes_f9_breakpoint_toggle() {
    let context = egui::Context::default();
    let mut source = "print('ready')\n".to_owned();
    let edited = std::collections::HashSet::new();
    let mut style = test_style(&edited);
    style.breakpoints_enabled = true;
    let editor_id = egui::Id::new("breakpoint-shortcut-editor-test");
    let raw_input = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            Vec2::new(900.0, 600.0),
        )),
        ..Default::default()
    };
    let _ = context.run_ui(raw_input(), |ui| {
        let output = show_virtual_text_editor(
            ui,
            editor_id,
            &mut source,
            1,
            true,
            EditorSyntax::Python,
            None,
            &style,
            |ui, row, font| {
                ui.fonts_mut(|fonts| {
                    fonts.layout_no_wrap(row.text.to_owned(), font, Color32::WHITE)
                })
            },
            |_| None,
            |_| None,
        );
        output.response.request_focus();
    });

    let mut input = raw_input();
    input.events.push(egui::Event::Key {
        key: Key::F9,
        physical_key: Some(Key::F9),
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    });
    let mut toggled = None;
    let _ = context.run_ui(input, |ui| {
        toggled = show_virtual_text_editor(
            ui,
            editor_id,
            &mut source,
            1,
            true,
            EditorSyntax::Python,
            None,
            &style,
            |ui, row, font| {
                ui.fonts_mut(|fonts| {
                    fonts.layout_no_wrap(row.text.to_owned(), font, Color32::WHITE)
                })
            },
            |_| None,
            |_| None,
        )
        .breakpoint_toggled;
    });
    assert_eq!(toggled, Some(1));
}

/// The editor takes focus on every press, so a focus ring would frame the
/// source for as long as anyone worked in it.
#[test]
fn a_focused_editor_draws_no_focus_ring() {
    let context = egui::Context::default();
    crate::ui::Theme::default().apply(&context);
    let accent = crate::ui::tokens::Tokens::get(&context).color.accent;
    let mut source = "print('ready')\n".to_owned();
    let edited = std::collections::HashSet::new();
    let style = test_style(&edited);
    let editor_id = egui::Id::new("focus-ring-editor-test");
    let raw_input = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            Vec2::new(900.0, 600.0),
        )),
        ..Default::default()
    };
    let mut focused = false;
    let mut output = None;
    for _ in 0..2 {
        output = Some(context.run_ui(raw_input(), |ui| {
            let shown = show_virtual_text_editor(
                ui,
                editor_id,
                &mut source,
                1,
                true,
                EditorSyntax::Python,
                None,
                &style,
                |ui, row, font| {
                    ui.fonts_mut(|fonts| {
                        fonts.layout_no_wrap(row.text.to_owned(), font, Color32::WHITE)
                    })
                },
                |_| None,
                |_| None,
            );
            focused = shown.response.has_focus();
            shown.response.request_focus();
        }));
    }
    assert!(focused, "the editor held focus while it was drawn");
    let ring = output
        .expect("a frame")
        .shapes
        .iter()
        .any(|clipped| match &clipped.shape {
            egui::Shape::Rect(rect) => rect.stroke.width >= 2.0 && rect.stroke.color == accent,
            _ => false,
        });
    assert!(!ring, "no focus ring around the source");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn widget_exposes_active_line_read_only_and_diagnostic_context_to_accesskit() {
    let context = egui::Context::default();
    context.enable_accesskit();
    crate::ui::Theme::default().apply(&context);
    let mut source = "R1 input output 1k\nV1 input 0 1\n".to_owned();
    let mut edited = std::collections::HashSet::new();
    edited.insert(1);
    let breakpoints = [2];
    let mut style = test_style(&edited);
    style.accessible_label = "Generated SPICE netlist viewer";
    style.breakpoints = &breakpoints;
    style.current_line = Some(2);
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
                1,
                false,
                EditorSyntax::Spice,
                Some(2),
                &style,
                |ui, row, font| {
                    ui.fonts_mut(|fonts| {
                        fonts.layout_no_wrap(row.text.to_owned(), font, Color32::WHITE)
                    })
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
    assert!(description.contains("Breakpoint on this line"));
    assert!(description.contains("Current debugger statement"));
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
        bounds.y0 >= f64::from(style.top_padding + style.line_height),
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
    let mut source = (1..=20)
        .map(|line| format!("line {line}\n"))
        .collect::<String>();
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
                    1,
                    true,
                    EditorSyntax::Spice,
                    Some(10),
                    &style,
                    |ui, row, font| {
                        ui.fonts_mut(|fonts| {
                            fonts.layout_no_wrap(row.text.to_owned(), font, Color32::WHITE)
                        })
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
    let mut source = "x".repeat(50_000);
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
                    1,
                    true,
                    EditorSyntax::Spice,
                    None,
                    &style,
                    |ui, row, font| {
                        ui.fonts_mut(|fonts| {
                            fonts.layout_no_wrap(row.text.to_owned(), font, Color32::WHITE)
                        })
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
    let mut source = "x".repeat(2_000);
    let mut session = VirtualEditorSession {
        model: TextDocumentModel::from_source(&source),
        ..Default::default()
    };
    session
        .model
        .set_selections(vec![TextSelection::caret(1_500)]);
    session.bound_revision = 1;
    session.maximum_columns = session.model.maximum_line_columns();
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
                    1,
                    true,
                    EditorSyntax::Spice,
                    None,
                    &style,
                    |ui, row, font| {
                        ui.fonts_mut(|fonts| {
                            fonts.layout_no_wrap(row.text.to_owned(), font, Color32::WHITE)
                        })
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
