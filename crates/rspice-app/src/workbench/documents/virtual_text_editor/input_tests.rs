//! What a frame of keyboard, pointer and text input does to the editor:
//! where focus stays, where the caret goes, how much of the source the view
//! shows around it, and what text input the platform is asked for.

use super::*;
use crate::workbench::documents::text_editor_commands::{
    queue_caret_position, queue_go_to_line, queue_reveal_line, show_go_to_line, take_reveal_line,
};

const SCREEN: Vec2 = Vec2::new(900.0, 600.0);
const EDITOR: Vec2 = Vec2::new(600.0, 340.0);
const LINE_HEIGHT: f32 = 17.0;

/// What one frame showed of the editor.
struct Shown {
    cursor_line: usize,
    cursor_char_index: usize,
    selected: Option<(usize, usize)>,
    ime: bool,
    /// The primary caret as painted, and the clip it was painted in.
    caret: Option<(egui::Rect, egui::Rect)>,
    output: egui::FullOutput,
}

/// An editor between other controls that take focus, driven a frame at a
/// time the way the application drives it.
struct Bench {
    context: egui::Context,
    id: egui::Id,
    source: String,
    editable: bool,
    edited: std::collections::HashSet<usize>,
    time: f64,
}

impl Bench {
    fn new(source: &str, editable: bool) -> Self {
        let context = egui::Context::default();
        // The application's fonts: without them every glyph measures nothing.
        crate::ui::Theme::default().apply(&context);
        context.enable_accesskit();
        // Keys are read by Windows conventions unless a test says otherwise.
        context.set_os(egui::os::OperatingSystem::Windows);
        Self {
            context,
            id: egui::Id::new("input-bench-editor"),
            source: source.to_owned(),
            editable,
            edited: std::collections::HashSet::new(),
            time: 0.0,
        }
    }

    fn frame_with(&mut self, events: Vec<egui::Event>, take_focus: bool) -> Shown {
        self.time += 1.0 / 60.0;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN)),
            time: Some(self.time),
            events,
            ..Default::default()
        };
        let style = super::tests::test_style(&self.edited);
        let (id, editable) = (self.id, self.editable);
        let source = &mut self.source;
        let mut shown = None;
        let output = self.context.run_ui(input, |ui| {
            let _ = ui.button("Above");
            ui.horizontal(|ui| {
                let _ = ui.button("Left");
                ui.allocate_ui(EDITOR, |ui| {
                    let output = show_virtual_text_editor(
                        ui,
                        id,
                        source,
                        1,
                        editable,
                        EditorSyntax::VerilogA,
                        None,
                        &style,
                        |ui, text, _line, font| {
                            ui.fonts_mut(|fonts| {
                                fonts.layout_no_wrap(text.to_owned(), font, Color32::WHITE)
                            })
                        },
                        |_| None,
                        |_| None,
                    );
                    if take_focus {
                        output.response.request_focus();
                    }
                    shown = Some((
                        output.cursor_line,
                        output.cursor_char_index,
                        output.selected_char_range,
                    ));
                });
                let _ = ui.button("Right");
            });
            let _ = ui.button("Below");
        });
        let (cursor_line, cursor_char_index, selected) = shown.expect("the editor was drawn");
        let caret = output
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(rect)
                    if rect.fill == Color32::WHITE && (rect.rect.width() - 1.25).abs() < 0.01 =>
                {
                    Some((rect.rect, clipped.clip_rect))
                }
                _ => None,
            });
        Shown {
            cursor_line,
            cursor_char_index,
            selected,
            ime: output.platform_output.ime.is_some(),
            caret,
            output,
        }
    }

    fn frame(&mut self, events: Vec<egui::Event>) -> Shown {
        self.frame_with(events, false)
    }

    /// Give the editor focus and draw one more frame, as a click does.
    fn focus(&mut self) {
        let _ = self.frame_with(Vec::new(), true);
        let _ = self.frame(Vec::new());
    }

    fn focused(&self) -> bool {
        self.context.memory(|memory| memory.focused()) == Some(self.id)
    }

    /// A key pressed and released over two frames; what the press showed.
    fn key(&mut self, key: Key, modifiers: Modifiers) -> Shown {
        let event = |pressed| egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed,
            repeat: false,
            modifiers,
        };
        let shown = self.frame(vec![event(true)]);
        let _ = self.frame(vec![event(false)]);
        shown
    }

    fn session(&self) -> VirtualEditorSession {
        self.context
            .data(|data| data.get_temp::<VirtualEditorSession>(self.id.with("virtual-session")))
            .expect("the editor keeps a session")
    }

    fn scroll_offset(&self) -> Vec2 {
        let session = self.session();
        egui::scroll_area::State::load(&self.context, session.scroll_id.expect("drawn"))
            .expect("the scroll area keeps its state")
            .offset
    }

    fn move_caret_to(&mut self, line: usize, column: usize) {
        queue_caret_position(&self.context, self.id, line, column);
        let _ = self.frame(Vec::new());
    }
}

fn numbered_lines(count: usize) -> String {
    (1..=count)
        .map(|line| format!("    x{line} = {line};\n"))
        .collect()
}

/// The caret is drawn whole inside the view, clear of the line-number gutter
/// that covers the view's left edge.
fn assert_caret_in_view(shown: &Shown, what: &str) {
    let (caret, clip) = shown
        .caret
        .unwrap_or_else(|| panic!("{what}: no caret drawn"));
    assert!(
        clip.contains_rect(caret) && caret.left() >= clip.left() + 47.0,
        "{what}: caret {caret:?} is outside the view {clip:?}"
    );
}

#[test]
fn arrows_tab_and_escape_stay_in_the_editor_and_act_on_the_text() {
    let mut bench = Bench::new(
        "module m;\n  analog begin\n    x = 1;\n  end\nendmodule\n",
        true,
    );
    bench.focus();
    bench.move_caret_to(3, 5);
    let start = bench.frame(Vec::new());
    assert_eq!((start.cursor_line, start.cursor_char_index), (2, 29));

    let left = bench.key(Key::ArrowLeft, Modifiers::NONE);
    assert!(bench.focused(), "ArrowLeft keeps focus in the editor");
    assert_eq!(left.cursor_char_index, 28, "ArrowLeft moves the caret");

    let down = bench.key(Key::ArrowDown, Modifiers::NONE);
    assert!(bench.focused(), "ArrowDown keeps focus in the editor");
    assert_eq!(down.cursor_line, 3);

    let up = bench.key(Key::ArrowUp, Modifiers::NONE);
    assert!(bench.focused(), "ArrowUp keeps focus in the editor");
    assert_eq!(up.cursor_line, 2);

    let tabbed = bench.key(Key::Tab, Modifiers::NONE);
    assert!(bench.focused(), "Tab keeps focus in the editor");
    assert_eq!(
        bench.source, "module m;\n  analog begin\n     x = 1;\n  end\nendmodule\n",
        "Tab at column 3 pads to the next indent stop"
    );
    assert_eq!(
        (tabbed.cursor_char_index, tabbed.selected),
        (29, None),
        "the caret stays a caret, after the padding"
    );

    let selected = bench.key(Key::ArrowRight, Modifiers::SHIFT);
    assert!(selected.selected.is_some());
    let escaped = bench.key(Key::Escape, Modifiers::NONE);
    assert!(bench.focused(), "Escape with a selection keeps focus");
    assert_eq!(escaped.selected, None, "Escape collapses the selection");

    let _ = bench.key(Key::Escape, Modifiers::NONE);
    assert!(
        !bench.focused(),
        "Escape with nothing to collapse hands the keyboard back"
    );
}

#[test]
fn tab_indents_at_the_caret_and_whole_lines_under_a_selection() {
    let mut bench = Bench::new("a\n  bb\n    ccc\n", true);
    bench.focus();
    bench.move_caret_to(2, 4);
    let _ = bench.key(Key::Tab, Modifiers::NONE);
    assert_eq!(bench.source, "a\n  b b\n    ccc\n", "one space to the stop");
    let _ = bench.key(Key::Tab, Modifiers::NONE);
    assert_eq!(bench.source, "a\n  b     b\n    ccc\n", "a whole stop");

    // A selection across lines indents those lines and stays selected.
    bench.move_caret_to(1, 1);
    let _ = bench.key(Key::ArrowDown, Modifiers::SHIFT);
    let _ = bench.key(Key::ArrowRight, Modifiers::SHIFT);
    let indented = bench.key(Key::Tab, Modifiers::NONE);
    assert_eq!(bench.source, "    a\n      b     b\n    ccc\n");
    assert_eq!(
        indented.selected,
        Some((0, 11)),
        "the selection keeps the lines' text and their new indentation"
    );
    let outdented = bench.key(Key::Tab, Modifiers::SHIFT);
    assert_eq!(bench.source, "a\n  b     b\n    ccc\n");
    assert_eq!(outdented.selected, Some((0, 3)));

    // Shift+Tab on a caret takes one stop off its line and keeps the caret
    // on the text it was before.
    bench.move_caret_to(3, 6);
    let outdented = bench.key(Key::Tab, Modifiers::SHIFT);
    assert_eq!(bench.source, "a\n  b     b\nccc\n");
    assert_eq!(
        (outdented.cursor_char_index, outdented.selected),
        (13, None)
    );
}

#[test]
fn a_key_in_the_first_focused_frame_stays_in_the_editor() {
    for key in [Key::ArrowLeft, Key::ArrowDown, Key::Tab] {
        let mut bench = Bench::new("alpha\nbeta\ngamma\n", true);
        let _ = bench.frame_with(Vec::new(), true);
        let _ = bench.key(key, Modifiers::NONE);
        assert!(bench.focused(), "{key:?} on focus's first frame kept focus");
    }
}

#[test]
fn ctrl_m_lets_tab_move_focus_and_says_so() {
    let mut bench = Bench::new("alpha\nbeta\n", true);
    bench.focus();
    let toggled = bench.key(Key::M, Modifiers::COMMAND | Modifiers::CTRL);
    assert!(bench.focused());
    let announced = toggled
        .output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .any(|(_, node)| {
            node.role() == egui::accesskit::Role::Status
                && node.live() == Some(egui::accesskit::Live::Polite)
                && node.label() == Some("Tab moves focus; Ctrl+M makes it indent again.")
        });
    assert!(announced, "the new Tab mode is read out");

    let _ = bench.key(Key::Tab, Modifiers::NONE);
    assert!(!bench.focused(), "Tab moves focus on");
    assert_eq!(bench.source, "alpha\nbeta\n", "Tab inserted nothing");

    // The mode is the window's: switched back, Tab indents again.
    bench.focus();
    let _ = bench.key(Key::M, Modifiers::COMMAND | Modifiers::CTRL);
    let _ = bench.key(Key::Tab, Modifiers::NONE);
    assert!(bench.focused());
    assert_eq!(bench.source, "    alpha\nbeta\n");
}

#[test]
fn text_input_is_asked_for_on_every_focused_frame_and_never_by_a_read_only_view() {
    let mut bench = Bench::new("alpha\nbeta\n", true);
    bench.focus();
    let mut caret_hidden = 0;
    for frame in 0..60 {
        let shown = bench.frame(Vec::new());
        assert!(shown.ime, "frame {frame} asked for no text input");
        caret_hidden += usize::from(shown.caret.is_none());
    }
    assert!(
        caret_hidden > 0,
        "the sixty frames include frames the blink hides the caret on"
    );

    let mut viewer = Bench::new("alpha\nbeta\n", false);
    viewer.focus();
    for _ in 0..10 {
        assert!(!viewer.frame(Vec::new()).ime);
    }
}

#[test]
fn the_caret_stands_still_under_reduced_motion() {
    let mut bench = Bench::new("alpha\nbeta\n", true);
    bench
        .context
        .global_style_mut(|style| style.animation_time = 0.0);
    bench.focus();
    for frame in 0..60 {
        assert!(
            bench.frame(Vec::new()).caret.is_some(),
            "frame {frame} hid the caret"
        );
    }
}

#[test]
fn the_caret_stays_in_view_as_keys_move_it() {
    let mut source = numbered_lines(400);
    source.insert_str(
        source.find("    x200").expect("line 200"),
        &format!("    long = {};\n", "1 + ".repeat(120)),
    );
    let mut bench = Bench::new(&source, true);
    bench.focus();
    for press in 1..=100 {
        let shown = bench.key(Key::ArrowDown, Modifiers::NONE);
        assert_eq!(shown.cursor_line, press);
        assert_caret_in_view(&shown, &format!("ArrowDown {press}"));
    }
    let end = bench.key(Key::End, Modifiers::COMMAND | Modifiers::CTRL);
    assert_eq!(end.cursor_line, 401);
    assert_caret_in_view(&end, "Ctrl+End");
    let home = bench.key(Key::Home, Modifiers::COMMAND | Modifiers::CTRL);
    assert_eq!(home.cursor_char_index, 0);
    assert_caret_in_view(&home, "Ctrl+Home");

    bench.move_caret_to(200, 1);
    let line_end = bench.key(Key::End, Modifiers::NONE);
    assert_caret_in_view(&line_end, "End of a line wider than the view");
    assert!(
        bench.scroll_offset().x > 0.0,
        "the view followed the caret right"
    );
    let line_start = bench.key(Key::Home, Modifiers::NONE);
    assert_caret_in_view(&line_start, "Home back from the far right");
}

#[test]
fn page_down_moves_the_caret_and_the_view_by_one_viewport() {
    let mut bench = Bench::new(&numbered_lines(400), true);
    bench.focus();
    let viewport = bench.session().viewport_size.expect("the view was drawn");
    let rows = ((viewport.y - 8.0) / LINE_HEIGHT).floor() as usize;
    assert!(rows > 5, "the view shows {rows} rows");

    let first = bench.key(Key::PageDown, Modifiers::NONE);
    assert_eq!(first.cursor_line, rows);
    assert_eq!(bench.scroll_offset().y, rows as f32 * LINE_HEIGHT);
    assert_caret_in_view(&first, "PageDown");

    let second = bench.key(Key::PageDown, Modifiers::SHIFT);
    assert_eq!(second.cursor_line, 2 * rows);
    let line_start = |line: usize| -> usize {
        bench
            .source
            .lines()
            .take(line)
            .map(|text| text.len() + 1)
            .sum()
    };
    assert_eq!(
        second.selected,
        Some((line_start(rows), line_start(2 * rows))),
        "Shift+PageDown extends the selection by a page"
    );
    assert_caret_in_view(&second, "Shift+PageDown");

    let back = bench.key(Key::PageUp, Modifiers::NONE);
    assert_eq!(back.cursor_line, rows);
    assert_eq!(back.selected, None);
    assert_caret_in_view(&back, "PageUp");
}

#[test]
fn macos_keys_follow_macos_conventions() {
    let mut bench = Bench::new("alpha beta gamma\nsecond\n", true);
    bench.context.set_os(egui::os::OperatingSystem::Mac);
    bench.focus();
    let command = Modifiers {
        mac_cmd: true,
        command: true,
        ..Modifiers::NONE
    };
    assert_eq!(bench.key(Key::ArrowRight, command).cursor_char_index, 16);
    assert_eq!(
        bench.key(Key::ArrowLeft, Modifiers::ALT).cursor_char_index,
        11
    );
    let _ = bench.key(Key::Backspace, Modifiers::ALT);
    assert_eq!(bench.source, "alpha gamma\nsecond\n");
    assert_eq!(bench.key(Key::ArrowDown, command).cursor_line, 2);
    assert_eq!(bench.key(Key::ArrowUp, command).cursor_char_index, 0);
    let _ = bench.key(Key::ArrowRight, command);
    let _ = bench.key(Key::Backspace, command);
    assert_eq!(bench.source, "\nsecond\n");
}

#[test]
fn control_backspace_and_delete_remove_a_word_on_windows() {
    let mut bench = Bench::new("alpha beta gamma\n", true);
    bench.context.set_os(egui::os::OperatingSystem::Windows);
    bench.focus();
    let control = Modifiers::COMMAND | Modifiers::CTRL;
    bench.move_caret_to(1, 12);
    let _ = bench.key(Key::Backspace, control);
    assert_eq!(bench.source, "alpha gamma\n");
    let _ = bench.key(Key::Delete, control);
    assert_eq!(bench.source, "alpha \n");
    // On the line's edge the key takes the line break.
    let _ = bench.key(Key::Delete, control);
    assert_eq!(bench.source, "alpha ");
}

#[test]
fn go_to_line_proposes_the_caret_line() {
    let mut bench = Bench::new(&numbered_lines(40), true);
    bench.focus();
    bench.move_caret_to(27, 3);
    queue_go_to_line(&bench.context, bench.id);
    let (id, source) = (bench.id, bench.source.clone());
    let input = |events: Vec<egui::Event>| egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN)),
        events,
        ..Default::default()
    };
    let mut accepted = None;
    for _ in 0..3 {
        let _ = bench.context.run_ui(input(Vec::new()), |ui| {
            accepted = show_go_to_line(ui, id, &source);
        });
    }
    let enter = egui::Event::Key {
        key: Key::Enter,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    };
    let _ = bench.context.run_ui(input(vec![enter]), |ui| {
        accepted = show_go_to_line(ui, id, &source);
    });
    assert_eq!(accepted, Some(27), "Enter goes to the line it proposed");
}

#[test]
fn a_jump_to_a_line_and_column_lands_the_caret_in_view() {
    let mut bench = Bench::new(&numbered_lines(400), true);
    bench.focus();
    let line_start = |source: &str, line: usize| -> usize {
        source.lines().take(line).map(|text| text.len() + 1).sum()
    };

    // A position far below the view: the caret goes there and the view
    // centres on it.
    bench.move_caret_to(300, 6);
    let far = bench.frame(Vec::new());
    assert_eq!(far.cursor_line, 299);
    assert_eq!(far.cursor_char_index, line_start(&bench.source, 299) + 5);
    assert_caret_in_view(&far, "a jump far below");
    let viewport = bench.session().viewport_size.expect("the view was drawn");
    let caret_middle = 8.0 + 299.5 * LINE_HEIGHT;
    let view_middle = bench.scroll_offset().y + viewport.y * 0.5;
    assert!(
        (caret_middle - view_middle).abs() <= LINE_HEIGHT,
        "the view centres on the caret: {caret_middle} against {view_middle}"
    );

    // A position already in view leaves the view where it is.
    let before = bench.scroll_offset();
    bench.move_caret_to(303, 1);
    assert_eq!(bench.frame(Vec::new()).cursor_line, 302);
    assert_eq!(bench.scroll_offset(), before);

    // A revealed line lands the same way, at its start.
    queue_reveal_line(&bench.context, bench.id, 40);
    let (id, source) = (bench.id, bench.source.clone());
    let _ = bench.context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN)),
            time: Some(bench.time),
            ..Default::default()
        },
        |ui| {
            assert_eq!(take_reveal_line(ui, id, &source), Some(40));
        },
    );
    let revealed = bench.frame(Vec::new());
    assert_eq!(revealed.cursor_char_index, line_start(&bench.source, 39));
    let row_top = 8.0 + 39.0 * LINE_HEIGHT;
    let view = bench.scroll_offset().y..=bench.scroll_offset().y + viewport.y;
    assert!(
        view.contains(&row_top) && view.contains(&(row_top + LINE_HEIGHT)),
        "the revealed line {row_top} is in the view {view:?}"
    );
}

/// Pointer events at `at`, with a finger down for the press when `touch`.
fn pointer(at: egui::Pos2, phase: Option<bool>, touch: bool) -> Vec<egui::Event> {
    let mut events = Vec::new();
    if touch {
        events.push(egui::Event::Touch {
            device_id: egui::TouchDeviceId(1),
            id: egui::TouchId(1),
            phase: match phase {
                Some(true) => egui::TouchPhase::Start,
                Some(false) => egui::TouchPhase::End,
                None => egui::TouchPhase::Move,
            },
            pos: at,
            force: None,
        });
    }
    events.push(egui::Event::PointerMoved(at));
    if let Some(pressed) = phase {
        events.push(egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: Modifiers::NONE,
        });
    }
    events
}

/// A point in column `column` of the `row`th row the editor shows, counted
/// from the top of its view; the source must not be scrolled sideways.
fn cell(bench: &Bench, row: usize, column: usize) -> egui::Pos2 {
    // The part of the editor on screen: its content scrolls under it.
    let editor = bench
        .context
        .read_response(bench.id)
        .expect("the editor is on screen")
        .interact_rect;
    let char_width = bench.context.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap("M".to_owned(), FontId::monospace(11.0), Color32::WHITE)
            .size()
            .x
    });
    egui::pos2(
        editor.left() + 47.0 + 12.0 + (column as f32 + 0.3) * char_width,
        editor.top() + 8.0 + (row as f32 + 0.5) * LINE_HEIGHT,
    )
}

#[test]
fn a_click_places_the_caret_and_a_drag_selects() {
    let mut bench = Bench::new("alpha beta\ngamma delta\n", true);
    let _ = bench.frame(Vec::new());
    let at = cell(&bench, 1, 3);
    let _ = bench.frame(pointer(at, Some(true), false));
    let clicked = bench.frame(pointer(at, Some(false), false));
    assert!(bench.focused(), "a click focuses the editor");
    assert_eq!(
        clicked.cursor_char_index,
        11 + 3,
        "the caret lands where it was clicked"
    );

    let from = cell(&bench, 0, 0);
    let to = cell(&bench, 0, 5);
    let _ = bench.frame(pointer(from, Some(true), false));
    let _ = bench.frame(pointer(from + Vec2::new(10.0, 0.0), None, false));
    let _ = bench.frame(pointer(to, None, false));
    let dragged = bench.frame(pointer(to, Some(false), false));
    assert_eq!(dragged.selected, Some((0, 5)));
}

#[test]
fn a_finger_scrolls_taps_the_caret_and_selects_a_word_with_handles() {
    let mut bench = Bench::new(&numbered_lines(200), true);
    let _ = bench.frame(Vec::new());
    let start = cell(&bench, 10, 6);

    // One finger dragging up scrolls the source down and selects nothing.
    let _ = bench.frame(pointer(start, Some(true), true));
    for step in 1..=6 {
        let _ = bench.frame(pointer(
            start - Vec2::new(0.0, 20.0 * step as f32),
            None,
            true,
        ));
    }
    let lifted = bench.frame(pointer(start - Vec2::new(0.0, 120.0), Some(false), true));
    let _ = bench.frame(Vec::new());
    assert!(
        bench.scroll_offset().y > 60.0,
        "the drag scrolled: {:?}",
        bench.scroll_offset()
    );
    assert_eq!(lifted.selected, None, "a finger drag selects nothing");
    assert!(!bench.focused(), "scrolling raises no keyboard");

    // A tap places the caret and focuses the editor.
    let tap = cell(&bench, 2, 4);
    let _ = bench.frame(pointer(tap, Some(true), true));
    let tapped = bench.frame(pointer(tap, Some(false), true));
    assert!(bench.focused(), "a tap focuses the editor");
    let offset_rows = (bench.scroll_offset().y / LINE_HEIGHT).floor() as usize;
    assert!(tapped.cursor_line >= offset_rows, "the tap landed in view");

    // A double tap selects the word under the finger, and the selection
    // carries handles.
    let _ = bench.frame(pointer(tap, Some(true), true));
    let double = bench.frame(pointer(tap, Some(false), true));
    let (start_char, end_char) = double.selected.expect("a double tap selects a word");
    assert_eq!(
        end_char - start_char,
        format!("x{}", tapped.cursor_line + 1).len()
    );
    let _ = bench.frame(Vec::new());
    let knobs = bench
        .frame(Vec::new())
        .output
        .shapes
        .iter()
        .filter(|clipped| {
            matches!(&clipped.shape, egui::Shape::Circle(circle)
                if circle.fill == Color32::LIGHT_BLUE)
        })
        .count();
    assert_eq!(knobs, 2, "the selection shows a handle at each end");
}
