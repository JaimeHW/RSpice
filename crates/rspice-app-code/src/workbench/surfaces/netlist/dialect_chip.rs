//! The dialect chip, which says how the engine reads the deck and offers the
//! other dialects a deck can declare, and the band over a deck whose text
//! names the program that wrote it.

#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the chip and the band are drawn once the deck's check carries its reading"
    )
)]

use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::documents::netlist_document::dialect::{DeckReading, Dialect, DialectHint};
use crate::workbench::{MessageCatalog, MessageId};

/// Room a select keeps beside its value: the inset before it and the
/// chevron after it.
const SELECT_ROOM: f32 = 8.0 + 24.0;

/// The chip for a deck read as `reading`. It is a select of the dialects a
/// deck can declare when the deck is `editable` and its dialect can be
/// chosen, and the reading's name otherwise. Returns the dialect chosen this
/// frame, when it is another than the deck's.
pub(crate) fn dialect_chip(
    ui: &mut Ui,
    reading: DeckReading,
    editable: bool,
    messages: MessageCatalog,
) -> Option<Dialect> {
    let name = messages.text(reading.name());
    let detail = messages.text(reading.detail());
    if !(editable && reading.choosable()) {
        let t = Tokens::get(ui.ctx());
        ui.add(egui::Label::new(
            egui::RichText::new(name)
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(t.color.text_dim),
        ))
        .on_hover_text(detail);
        return None;
    }
    let options = Dialect::CHOICES.map(|dialect| messages.text(dialect.name()));
    let t = Tokens::get(ui.ctx());
    let width = options
        .iter()
        .chain([&name])
        .map(|label| {
            ui.painter()
                .layout_no_wrap(label.clone(), theme::field_mono(), t.color.text)
                .size()
                .x
        })
        .fold(0.0, f32::max)
        + SELECT_ROOM;
    let output = crate::ui::widgets::select_mono_with_response(
        ui,
        "netlist-dialect",
        &messages.text(MessageId::NetlistDialect),
        &name,
        &options,
        width.ceil(),
    );
    output.response.on_hover_text(detail);
    output
        .picked
        .and_then(|index| Dialect::CHOICES.get(index).copied())
        .filter(|dialect| reading != DeckReading::Declared(*dialect))
}

/// The band over a deck whose text names the program that wrote it, until it
/// is dismissed: dismissing it lasts the session, for the deck `deck_id`
/// names. Returns the dialect to read the deck in when its button is
/// pressed.
pub(crate) fn dialect_hint_band(
    ui: &mut Ui,
    hint: DialectHint,
    deck_id: egui::Id,
    messages: MessageCatalog,
) -> Option<Dialect> {
    let dismissed_id = deck_id.with("dialect-hint-dismissed");
    if ui
        .ctx()
        .data(|data| data.get_temp::<bool>(dismissed_id))
        .unwrap_or_default()
    {
        return None;
    }
    let program = messages.text(hint.dialect.name());
    let line = hint.line.to_string();
    let mut read = false;
    let mut dismiss = false;
    super::advisory_band(ui, None, |ui| {
        ui.label(messages.format(
            MessageId::NetlistDialectHint,
            &[
                ("program", program.as_str()),
                ("marker", hint.marker),
                ("line", line.as_str()),
            ],
        ));
        read = ui
            .button(messages.format(MessageId::NetlistDialectHintRead, &[("program", &program)]))
            .clicked();
        dismiss = ui
            .button(messages.text(MessageId::NetlistDialectHintDismiss))
            .clicked();
    });
    if dismiss {
        ui.ctx()
            .data_mut(|data| data.insert_temp(dismissed_id, true));
    }
    read.then_some(hint.dialect)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::UiTextLocale;

    fn messages() -> MessageCatalog {
        MessageCatalog::new(UiTextLocale::EnglishUnitedStates)
    }

    fn context() -> egui::Context {
        let context = egui::Context::default();
        crate::ui::Theme::default().apply(&context);
        context
    }

    fn input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 600.0),
            )),
            events,
            ..Default::default()
        }
    }

    /// Every text a frame painted, and where.
    fn painted(output: &egui::FullOutput) -> Vec<(String, egui::Rect)> {
        output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) => {
                    Some((text.galley.text().to_owned(), text.visual_bounding_rect()))
                }
                _ => None,
            })
            .collect()
    }

    /// The chip for `reading` drawn with its menu open: what it painted.
    fn chip_with_its_menu_open(reading: DeckReading, editable: bool) -> Vec<String> {
        let context = context();
        let mut shown = Vec::new();
        for _ in 0..3 {
            let output = context.run_ui(input(Vec::new()), |ui| {
                let menu = ui.make_persistent_id(("rspice.select", "netlist-dialect"));
                egui::Popup::open_id(ui.ctx(), menu);
                assert_eq!(dialect_chip(ui, reading, editable, messages()), None);
            });
            shown = painted(&output).into_iter().map(|(text, _)| text).collect();
        }
        shown
    }

    /// The menu offers the four dialects a deck can declare, and the well
    /// names the reading whole: a deck read with Xyce's devices is named so
    /// and that reading is not among the choices.
    #[test]
    fn the_menu_offers_the_four_dialects_a_deck_can_declare() {
        for reading in [
            DeckReading::Declared(Dialect::Ngspice),
            DeckReading::XyceDevices,
        ] {
            let shown = chip_with_its_menu_open(reading, true);
            for dialect in Dialect::CHOICES {
                let choice = messages().text(dialect.name());
                assert!(
                    shown.contains(&choice),
                    "{reading:?}: {choice} in {shown:?}"
                );
            }
            let name = messages().text(reading.name());
            assert!(shown.contains(&name), "{reading:?}: {name} in {shown:?}");
        }
        let shown = chip_with_its_menu_open(DeckReading::XyceDevices, true);
        assert_eq!(
            shown.iter().filter(|text| *text == "Xyce devices").count(),
            1,
            "{shown:?}"
        );
    }

    /// A read-only tab, and a Spectre deck whose file decides how it is
    /// read, name the reading and offer no menu.
    #[test]
    fn a_reading_that_cannot_be_changed_is_named_and_offers_nothing() {
        for (reading, editable) in [
            (DeckReading::Declared(Dialect::Ltspice), false),
            (DeckReading::Spectre, true),
        ] {
            assert_eq!(
                chip_with_its_menu_open(reading, editable),
                vec![messages().text(reading.name())],
                "{reading:?}"
            );
        }
    }

    /// One frame of the band for `deck` with `events`: what it returned and
    /// what it painted.
    fn band(
        context: &egui::Context,
        deck: egui::Id,
        events: Vec<egui::Event>,
    ) -> (Option<Dialect>, Vec<(String, egui::Rect)>) {
        let hint = DialectHint {
            dialect: Dialect::Ltspice,
            marker: ".backanno",
            line: 52,
        };
        let mut chosen = None;
        let output = context.run_ui(input(events), |ui| {
            chosen = dialect_hint_band(ui, hint, deck, messages());
        });
        (chosen, painted(&output))
    }

    /// Press the button `label` on the band for `deck`: what the band
    /// returned on the frame the press ended.
    fn press(context: &egui::Context, deck: egui::Id, label: &str) -> Option<Dialect> {
        let (_, shown) = band(context, deck, Vec::new());
        let at = shown
            .iter()
            .find(|(text, _)| text == label)
            .unwrap_or_else(|| panic!("{label} is on the band: {shown:?}"))
            .1
            .center();
        let button = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        let _ = band(context, deck, vec![egui::Event::PointerMoved(at)]);
        let _ = band(context, deck, vec![button(true)]);
        band(context, deck, vec![button(false)]).0
    }

    /// The band names the program and the card that names it; its button
    /// reads the deck as that program, and dismissing it hides it for that
    /// deck only.
    #[test]
    fn the_hint_reads_the_deck_as_its_program_or_is_dismissed_for_that_deck() {
        let context = context();
        let deck = egui::Id::new("amp.cir");
        let (_, shown) = band(&context, deck, Vec::new());
        assert!(
            shown
                .iter()
                .any(|(text, _)| text == "Written by LTspice? .backanno on line 52."),
            "{shown:?}"
        );
        assert_eq!(
            press(&context, deck, "Read as LTspice"),
            Some(Dialect::Ltspice)
        );
        assert_eq!(press(&context, deck, "Dismiss"), None);
        assert!(band(&context, deck, Vec::new()).1.is_empty(), "dismissed");
        assert!(
            !band(&context, egui::Id::new("filter.cir"), Vec::new())
                .1
                .is_empty(),
            "another deck's hint stays"
        );
    }
}
