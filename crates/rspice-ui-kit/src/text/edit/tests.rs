use egui::{Context, Event, FontId, Id, Key, Modifiers, PointerButton, RawInput, TextEdit, pos2};

use super::*;

const SIZE: f32 = 30.0;

/// An editor over one value in a bare context with the application's fonts,
/// driven one pass at a time.
struct Field {
    ctx: Context,
    value: String,
    id: Id,
    /// The clock, in seconds: a pass takes a sixtieth.
    time: f64,
    /// What the last pass showed: where the galley stood and what it held.
    shown: Option<(Pos2, Arc<Galley>)>,
    shapes: Vec<Shape>,
}

impl Field {
    fn new(value: &str) -> Self {
        let ctx = Context::default();
        ctx.set_os(egui::os::OperatingSystem::Nix);
        crate::Theme::default().apply(&ctx);
        let mut field = Self {
            ctx,
            value: value.to_owned(),
            id: Id::new("reading-order field"),
            time: 0.0,
            shown: None,
            shapes: Vec::new(),
        };
        // Fonts are built on the first pass and laid out from the second.
        field.pass(Vec::new());
        field.pass(Vec::new());
        field
    }

    fn pass(&mut self, events: Vec<Event>) {
        let Self {
            ctx,
            value,
            id,
            time,
            shown,
            ..
        } = self;
        *time += 1.0 / 60.0;
        let output = ctx.run_ui(
            RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(800.0, 200.0))),
                time: Some(*time),
                events,
                ..Default::default()
            },
            |ui| {
                let font = FontId::proportional(SIZE);
                let reading = ReadingOrderEdit::new(ui, value, font.clone(), None, false);
                let mut layouter = reading.as_ref().map(ReadingOrderEdit::layouter);
                let mut edit = TextEdit::singleline(value)
                    .id(*id)
                    .font(font)
                    .desired_width(600.0);
                if let Some(layouter) = layouter.as_mut() {
                    edit = edit.layouter(layouter);
                }
                let output = match &reading {
                    Some(reading) => reading.show(ui, |ui| edit.show(ui)),
                    None => edit.show(ui),
                };
                *shown = Some((
                    output.galley_pos - vec2(output.galley.rect.left(), 0.0),
                    output.galley,
                ));
            },
        );
        self.shapes = output
            .shapes
            .into_iter()
            .map(|clipped| clipped.shape)
            .collect();
    }

    fn focus(&mut self) {
        self.ctx.memory_mut(|memory| memory.request_focus(self.id));
        self.pass(Vec::new());
    }

    fn key(&mut self, key: Key, modifiers: Modifiers) {
        self.pass(vec![Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        }]);
    }

    fn type_text(&mut self, text: &str) {
        self.pass(vec![Event::Text(text.to_owned())]);
    }

    /// A press and release at `at`, a second after the last pass, so it is
    /// never taken for the second click of a double click.
    fn click(&mut self, at: Pos2) {
        self.time += 1.0;
        let button = |pressed| Event::PointerButton {
            pos: at,
            button: PointerButton::Primary,
            pressed,
            modifiers: Modifiers::NONE,
        };
        self.pass(vec![Event::PointerMoved(at), button(true)]);
        self.pass(vec![button(false)]);
    }

    fn cursor(&self) -> CCursorRange {
        egui::TextEdit::load_state(&self.ctx, self.id)
            .and_then(|state| state.cursor.char_range())
            .expect("the field holds a cursor")
    }

    fn select(&mut self, range: Range<usize>) {
        let mut state = egui::TextEdit::load_state(&self.ctx, self.id).unwrap_or_default();
        state.cursor.set_char_range(Some(CCursorRange::two(
            CCursor::new(range.start),
            CCursor::new(range.end),
        )));
        state.store(&self.ctx, self.id);
        self.pass(Vec::new());
    }

    /// The left and right edges, on the screen, of the glyph lettering
    /// `character` in what the last pass showed.
    fn glyph(&self, character: char) -> (f32, f32) {
        let (origin, galley) = self.shown.as_ref().expect("a pass ran");
        galley
            .rows
            .iter()
            .flat_map(|row| row.row.glyphs.iter().map(move |glyph| (row.pos.x, glyph)))
            .find(|(_, glyph)| glyph.chr == character)
            .map(|(x, glyph)| {
                let left = origin.x + x + glyph.pos.x;
                (left, left + glyph.advance_width)
            })
            .unwrap_or_else(|| panic!("{character:?} is lettered"))
    }

    /// Where the caret was drawn in the last pass.
    fn caret_x(&self) -> f32 {
        let stroke = self.ctx.global_style().visuals.text_cursor.stroke;
        self.shapes
            .iter()
            .find_map(|shape| match shape {
                Shape::LineSegment {
                    points,
                    stroke: drawn,
                } if *drawn == stroke => Some(points[0].x),
                _ => None,
            })
            .expect("the caret is drawn")
    }

    /// The galley the last pass painted for the field.
    fn painted(&self) -> Arc<Galley> {
        self.shapes
            .iter()
            .find_map(|shape| match shape {
                Shape::Text(text) if text.galley.text() == self.value => {
                    Some(Arc::clone(&text.galley))
                }
                _ => None,
            })
            .expect("the field's text is painted")
    }

    fn row_y(&self) -> f32 {
        let (origin, galley) = self.shown.as_ref().expect("a pass ran");
        origin.y + galley.rows[0].rect().center().y
    }
}

#[test]
fn the_arrow_keys_walk_arabic_in_the_order_it_is_typed() {
    let mut field = Field::new("عدد 123 مرحبا");
    field.focus();
    field.key(Key::End, Modifiers::NONE);
    for _ in 0..3 {
        field.key(Key::ArrowLeft, Modifiers::NONE);
    }
    field.type_text("x");
    assert_eq!(field.value, "عدد 123 مرxحبا");
}

#[test]
fn a_word_jump_back_over_hebrew_after_latin_lands_before_the_word() {
    let mut field = Field::new("Aurora בב");
    field.focus();
    field.key(Key::End, Modifiers::NONE);
    field.key(Key::ArrowLeft, Modifiers::CTRL);
    field.type_text("x");
    assert_eq!(field.value, "Aurora xבב");
}

#[test]
fn the_caret_stands_at_the_leading_edge_of_the_character_after_it() {
    // Displayed "42 םולש": the paragraph reads from the right, and the
    // number inside it from its own left.
    let mut field = Field::new("שלום 42");
    field.focus();
    field.select(0..0);
    let (_, shin_right) = field.glyph('ש');
    assert!((field.caret_x() - shin_right).abs() < 0.5, "before ש");
    field.select(5..5);
    let (four_left, _) = field.glyph('4');
    assert!((field.caret_x() - four_left).abs() < 0.5, "before 4");
    field.select(7..7);
    let (_, two_right) = field.glyph('2');
    assert!((field.caret_x() - two_right).abs() < 0.5, "after 2");
    assert!(four_left < shin_right);
}

#[test]
fn a_click_puts_the_caret_at_the_character_edge_nearest_it() {
    let mut field = Field::new("שלום 42");
    let y = field.row_y();
    let (shin_left, shin_right) = field.glyph('ש');
    field.click(pos2(shin_right - 0.2 * (shin_right - shin_left), y));
    assert_eq!(field.cursor(), CCursorRange::one(CCursor::new(0)));
    field.click(pos2(shin_left + 0.2 * (shin_right - shin_left), y));
    assert_eq!(field.cursor(), CCursorRange::one(CCursor::new(1)));
    let (four_left, four_right) = field.glyph('4');
    field.click(pos2(four_left + 0.2 * (four_right - four_left), y));
    assert_eq!(field.cursor(), CCursorRange::one(CCursor::new(5)));
}

#[test]
fn a_selection_bands_each_stretch_of_its_glyphs() {
    // "c", the space and "ד" are selected: the first two at the end of the
    // Latin, the last at the far right of the Hebrew, displayed "והד".
    let mut field = Field::new("abc דהו");
    field.focus();
    field.select(2..5);
    let galley = field.painted();
    let (origin, _) = field.shown.clone().expect("a pass ran");
    let fill = field.ctx.global_style().visuals.selection.bg_fill;
    let row = &galley.rows[0].row;
    let mut bands: Vec<(f32, f32)> = row
        .visuals
        .mesh
        .vertices
        .chunks(4)
        .filter(|quad| quad.iter().all(|vertex| vertex.color == fill))
        .map(|quad| {
            let xs = quad.iter().map(|vertex| vertex.pos.x);
            (
                origin.x + xs.clone().fold(f32::INFINITY, f32::min),
                origin.x + xs.fold(f32::NEG_INFINITY, f32::max),
            )
        })
        .collect();
    bands.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (c_left, _) = field.glyph('c');
    let (_, space_right) = field.glyph(' ');
    let (dalet_left, dalet_right) = field.glyph('ד');
    assert_eq!(bands.len(), 2, "{bands:?}");
    assert!((bands[0].0 - c_left).abs() < 0.5 && (bands[0].1 - space_right).abs() < 0.5);
    assert!((bands[1].0 - dalet_left).abs() < 0.5 && (bands[1].1 - dalet_right).abs() < 0.5);
    // Only the selected glyphs take the selection's ink.
    let ink = field.ctx.global_style().visuals.selection.stroke.color;
    let inked: Vec<char> = row
        .glyphs
        .iter()
        .filter(|glyph| {
            let first = glyph.first_vertex as usize;
            row.visuals
                .mesh
                .vertices
                .get(first)
                .is_some_and(|vertex| vertex.color == ink)
                && glyph.uv_rect.size.x > 0.0
        })
        .map(|glyph| glyph.chr)
        .collect();
    assert_eq!(inked, ['c', 'ד']);
}

#[test]
fn wrapped_rows_hold_the_characters_and_line_breaks_of_the_logical_rows() {
    let text = "שלום עולם 42 מגבר ראשי\nאב 7";
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
    let mut laid = None;
    for _ in 0..2 {
        let _ = ctx.run_ui(RawInput::default(), |ui| {
            laid = ui.fonts_mut(|fonts| {
                edit_layout(
                    fonts,
                    text,
                    TextFormat::simple(FontId::proportional(SIZE), Color32::WHITE),
                    TextWrapping::wrap_at_width(160.0),
                )
                .1
            });
        });
    }
    let laid = laid.expect("laid out in display order");
    let galley = &laid.galley;
    assert!(galley.rows.len() > 2, "the first paragraph wraps");
    assert_eq!(galley.text(), text);
    assert_eq!(galley.end().index.0, text.chars().count());
    let breaks: Vec<bool> = galley
        .rows
        .iter()
        .map(|row| row.ends_with_newline)
        .collect();
    assert_eq!(breaks.iter().filter(|ends| **ends).count(), 1, "{breaks:?}");
    // Every character but the line break is lettered once, on its own row.
    let mut lettered: Vec<usize> = laid
        .rows
        .iter()
        .flat_map(|row| row.glyph_chars.iter().copied())
        .collect();
    lettered.sort_unstable();
    let line_break = text.chars().position(|character| character == '\n');
    let expected: Vec<usize> = (0..text.chars().count())
        .filter(|index| Some(*index) != line_break)
        .collect();
    assert_eq!(lettered, expected);
}

#[test]
fn text_without_right_to_left_characters_keeps_egui_s_editor() {
    let ctx = Context::default();
    crate::Theme::default().apply(&ctx);
    let _ = ctx.run_ui(RawInput::default(), |ui| {
        assert!(
            ReadingOrderEdit::new(ui, "R1 4.7 kΩ", FontSelection::Default, None, false).is_none()
        );
        assert!(
            ReadingOrderEdit::new(ui, "电阻 ภาษา", FontSelection::Default, None, true).is_none()
        );
        assert!(
            ReadingOrderEdit::new(ui, "R1 שלום", FontSelection::Default, None, false).is_some()
        );
    });
}
