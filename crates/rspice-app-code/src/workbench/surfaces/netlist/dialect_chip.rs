//! The dialect chip, which says how the engine reads the deck and offers the
//! other dialects a deck can declare, and the band over a deck whose text
//! names the program that wrote it.

use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::Button;
use crate::workbench::app_state::Model;
use crate::workbench::documents::netlist_document::dialect::{
    DeckReading, Dialect, DialectHint, declare, dialect_name, hint_once_per_check, reading_detail,
    reading_name, shown_source_in,
};
use crate::workbench::documents::netlist_document::{
    NetlistDocumentState, active_netlist_source_is_editable_in, editor_id_in,
};
use crate::workbench::{MessageCatalog, MessageId};

/// Room a select keeps beside its value: the inset before it and the
/// chevron after it.
const SELECT_ROOM: f32 = 8.0 + 24.0;

/// The chip in the header of the document the page shows: the reading its
/// last check found, with the menu of dialects on an editable deck whose
/// dialect can be chosen.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DeckChip {
    reading: DeckReading,
    menu: bool,
}

impl DeckChip {
    /// The chip for the document the page shows; `None` before its first
    /// check, and for a comparison, which is no deck.
    pub(crate) fn shown(model: &Model, netlist: &NetlistDocumentState) -> Option<Self> {
        let source = shown_source_in(model, netlist)?;
        let reading = netlist.diagnostics.reading()?;
        Some(Self {
            reading,
            menu: source.root
                && reading.choosable()
                && active_netlist_source_is_editable_in(model, netlist),
        })
    }

    /// The width it takes.
    pub(crate) fn width(self, ui: &Ui, messages: MessageCatalog) -> f32 {
        let t = Tokens::get(ui.ctx());
        let measure = |label: String| {
            ui.painter()
                .layout_no_wrap(label, theme::field_sans(), t.color.text)
                .size()
                .x
        };
        let name = measure(messages.text(reading_name(self.reading)));
        if !self.menu {
            return name;
        }
        (Dialect::CHOICES
            .iter()
            .map(|dialect| measure(messages.text(dialect_name(*dialect))))
            .fold(name, f32::max)
            + SELECT_ROOM)
            .ceil()
    }

    /// Draw it. A dialect chosen from its menu is declared in the deck as
    /// one edit of the editor's own, saved like any edit.
    pub(crate) fn show(
        self,
        ui: &mut Ui,
        model: &Model,
        netlist: &NetlistDocumentState,
        messages: MessageCatalog,
    ) {
        if let Some(dialect) = dialect_chip(ui, self.reading, self.menu, messages) {
            read_deck_as(ui.ctx(), model, netlist, dialect);
        }
    }
}

/// Declare `dialect` in the deck the page shows, as one edit of the editor's
/// own.
fn read_deck_as(
    context: &egui::Context,
    model: &Model,
    netlist: &NetlistDocumentState,
    dialect: Dialect,
) {
    let deck = &model.simulation.source.netlist_content;
    if let Some(edit) = declare(deck, dialect) {
        edit.queue(context, editor_id_in(model, netlist), deck);
    }
}

/// The chip for a deck read as `reading`: a select of the dialects a deck can
/// declare when it offers the `menu` and its dialect can be chosen, and
/// otherwise the reading's name in the select's face, without the well. Returns the dialect chosen this frame,
/// when it is another than the deck's.
fn dialect_chip(
    ui: &mut Ui,
    reading: DeckReading,
    menu: bool,
    messages: MessageCatalog,
) -> Option<Dialect> {
    let name = messages.text(reading_name(reading));
    let detail = messages.text(reading_detail(reading));
    if !(menu && reading.choosable()) {
        let t = Tokens::get(ui.ctx());
        ui.add(egui::Label::new(
            egui::RichText::new(name)
                .font(theme::field_sans())
                .color(t.color.text_dim),
        ))
        .on_hover_text(detail);
        return None;
    }
    let options = Dialect::CHOICES.map(|dialect| messages.text(dialect_name(dialect)));
    let width = DeckChip { reading, menu }.width(ui, messages);
    let output = crate::ui::widgets::compact_select_with_response(
        ui,
        "netlist-dialect",
        &messages.text(MessageId::NetlistDialect),
        &name,
        &options,
        width,
    );
    output.response.on_hover_text(detail);
    output
        .picked
        .and_then(|index| Dialect::CHOICES.get(index).copied())
        .filter(|dialect| reading != DeckReading::Declared(*dialect))
}

/// The band over the editable deck the page shows when its text names the
/// program that wrote it and declares no dialect; its button declares that
/// program's dialect in the deck.
pub(crate) fn deck_hint_band(
    ui: &mut Ui,
    model: &Model,
    netlist: &NetlistDocumentState,
    messages: MessageCatalog,
) {
    let editable_deck = shown_source_in(model, netlist).is_some_and(|source| source.root)
        && active_netlist_source_is_editable_in(model, netlist);
    let Some(checked) = netlist.validated_revision().filter(|_| editable_deck) else {
        return;
    };
    let editor_id = editor_id_in(model, netlist);
    let deck = &model.simulation.source.netlist_content;
    let Some(hint) = hint_once_per_check(ui.ctx(), editor_id, checked, deck) else {
        return;
    };
    if let Some(dialect) = dialect_hint_band(ui, hint, editor_id, messages) {
        read_deck_as(ui.ctx(), model, netlist, dialect);
    }
}

/// The band over a deck whose text names the program that wrote it, until it
/// is dismissed: dismissing it lasts the session, for the deck `deck_id`
/// names. Returns the dialect to read the deck in when its button is
/// pressed.
fn dialect_hint_band(
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
    let program = messages.text(dialect_name(hint.dialect));
    let line = hint.line.to_string();
    let mut read = false;
    let mut dismiss = false;
    super::advisory_band(ui, None, |ui| {
        let sentence = messages.format(
            MessageId::NetlistDialectHint,
            &[
                ("program", program.as_str()),
                ("marker", CARD_SLOT),
                ("line", line.as_str()),
            ],
        );
        // The buttons stand apart from each other, and further from the
        // sentence they answer.
        ui.spacing_mut().item_spacing.x = HINT_BUTTON_GAP;
        ui.label(with_card(ui, &sentence, hint.marker));
        ui.add_space(HINT_BUTTON_GAP);
        let read_label =
            messages.format(MessageId::NetlistDialectHintRead, &[("program", &program)]);
        let dismiss_label = messages.text(MessageId::NetlistDialectHintDismiss);
        let read_button = Button::new(&read_label).compact();
        let dismiss_button = Button::new(&dismiss_label).compact();
        // The pair wraps as one: on a narrow page both go under the
        // sentence, never one beside it and the other alone below.
        let pair =
            read_button.measured_width(ui) + HINT_BUTTON_GAP + dismiss_button.measured_width(ui);
        if ui.available_width() < pair {
            ui.end_row();
        }
        read = read_button.show(ui).clicked();
        dismiss = dismiss_button.show(ui).clicked();
    });
    if dismiss {
        ui.ctx()
            .data_mut(|data| data.insert_temp(dismissed_id, true));
    }
    read.then_some(hint.dialect)
}

/// The air between the hint band's buttons, and again before the first.
const HINT_BUTTON_GAP: f32 = 8.0;

/// Stands in for a card's name while the sentence around it is formatted.
const CARD_SLOT: &str = "\u{e000}";

/// `sentence` with [`CARD_SLOT`] replaced by `card`: the card in the code
/// face, the words around it in the body face, on the body's row so the
/// sentence sits level with the buttons beside it.
fn with_card(ui: &Ui, sentence: &str, card: &str) -> egui::text::LayoutJob {
    let body = egui::TextStyle::Body.resolve(ui.style());
    let code = theme::mono(body.size, FontWeight::Regular);
    let row = ui.text_style_height(&egui::TextStyle::Body);
    let color = ui.visuals().text_color();
    let format = |font_id| egui::TextFormat {
        font_id,
        color,
        line_height: Some(row),
        valign: egui::Align::Center,
        ..Default::default()
    };
    let mut job = egui::text::LayoutJob::default();
    match sentence.split_once(CARD_SLOT) {
        Some((before, after)) => {
            job.append(before, 0.0, format(body.clone()));
            job.append(card, 0.0, format(code));
            job.append(after, 0.0, format(body));
        }
        None => job.append(sentence, 0.0, format(body)),
    }
    job
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
                let choice = messages().text(dialect_name(dialect));
                assert!(
                    shown.contains(&choice),
                    "{reading:?}: {choice} in {shown:?}"
                );
            }
            let name = messages().text(reading_name(reading));
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
                vec![messages().text(reading_name(reading))],
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

    /// The Netlist page over a deck, drawn as the application draws it.
    mod page {
        use std::path::PathBuf;

        use crate::workbench::code::tests::{CodeHarness, harness};
        use crate::workbench::documents::netlist_document::{
            NetlistRoot, NetlistTab, editor_id_in,
        };
        use crate::workbench::framework::command_registry::CommandAvailability;
        use crate::workbench::framework::feature::{Feature, ForeignAvailability};
        use crate::workbench::framework::requests::{SourceEditorRequest, StandardCommand};

        /// An application window wide enough for the full toolbar.
        fn input(events: Vec<egui::Event>) -> egui::RawInput {
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                events,
                ..Default::default()
            }
        }

        /// The deck `source` open on the page as its file `path`.
        fn deck(path: &str, source: &str) -> CodeHarness {
            let mut harness = harness();
            let deck = crate::workbench::documents::deck_files::OpenDeck::in_memory(path, source);
            harness.model.workspace.content.decks = vec![crate::state::ProjectDeck {
                id: deck.id,
                source: crate::state::DeckSource::File {
                    path: PathBuf::from(path),
                },
            }];
            crate::workbench::documents::deck_files::install_open_deck(
                &mut harness.model,
                &mut harness.state.netlist,
                &mut harness.state.netlist_open_documents,
                &mut harness.runs,
                deck,
            );
            assert_eq!(harness.state.netlist.front.root_kind(), NetlistRoot::Deck);
            harness
        }

        /// Every text one frame of the page painted, and where.
        fn frame(
            context: &egui::Context,
            harness: &mut CodeHarness,
            events: Vec<egui::Event>,
        ) -> Vec<(String, egui::Rect)> {
            fn walk(shape: &egui::Shape, out: &mut Vec<(String, egui::Rect)>) {
                match shape {
                    egui::Shape::Text(text) => {
                        out.push((text.galley.text().to_owned(), text.visual_bounding_rect()));
                    }
                    egui::Shape::Vec(shapes) => {
                        for shape in shapes {
                            walk(shape, out);
                        }
                    }
                    _ => {}
                }
            }
            // The shell lends the page with the dispatcher's answer for each
            // command it reads, Run among them.
            let answers = ForeignAvailability::take(
                <crate::workbench::code::CodeFeature as Feature>::FOREIGN_AVAILABILITY,
                |_| CommandAvailability::Available,
            );
            let output = context.run_ui(input(events), |ui| {
                super::super::super::show_prepared(
                    ui,
                    &mut harness.cx().with_foreign_availability(answers),
                );
            });
            let mut painted = Vec::new();
            for clipped in &output.shapes {
                walk(&clipped.shape, &mut painted);
            }
            painted
        }

        fn context() -> egui::Context {
            let context = egui::Context::default();
            crate::ui::Theme::default().apply(&context);
            context
        }

        /// Press what reads `label` on the page; what the page painted on the
        /// frame after the press.
        fn press(
            context: &egui::Context,
            harness: &mut CodeHarness,
            label: &str,
        ) -> Vec<(String, egui::Rect)> {
            let shown = frame(context, harness, Vec::new());
            let at = shown
                .iter()
                .rev()
                .find(|(text, _)| text == label)
                .unwrap_or_else(|| panic!("{label} is on the page: {shown:?}"))
                .1
                .center();
            let button = |pressed| egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            };
            let _ = frame(context, harness, vec![egui::Event::PointerMoved(at)]);
            let _ = frame(context, harness, vec![button(true)]);
            let _ = frame(context, harness, vec![button(false)]);
            frame(context, harness, Vec::new())
        }

        /// The texts the header painted: what stands above the editor's
        /// first line.
        fn header(painted: &[(String, egui::Rect)]) -> Vec<String> {
            painted
                .iter()
                .filter(|(_, rect)| {
                    rect.bottom() <= super::super::super::toolbar::CODE_TOOLBAR_HEIGHT
                })
                .map(|(text, _)| text.clone())
                .collect()
        }

        /// The deck header names the dialect the deck declares, in place of
        /// the words that said who owns the deck, on a deck and on the
        /// generated netlist alike.
        #[test]
        fn the_header_names_the_dialect_the_deck_declares() {
            let context = context();
            let mut ngspice = deck(
                "amp.cir",
                "amp\n.options rspice_dialect=ngspice\nR1 a 0 1k\n.op\n.end\n",
            );
            let _ = frame(&context, &mut ngspice, Vec::new());
            let shown = header(&frame(&context, &mut ngspice, Vec::new()));
            assert!(shown.iter().any(|text| text == "ngspice"), "{shown:?}");

            let mut generated = harness();
            crate::workbench::surfaces::netlist::fixtures::retain_generated(
                &mut generated.model,
                &mut generated.state.netlist,
                "generated\nR1 out 0 1k\n.op\n.end\n",
            );
            generated.model.simulation.source.netlist_content =
                generated.model.netlist_artifacts.generated_source.clone();
            generated.state.netlist.front = NetlistTab::Generated;
            generated.state.netlist.active_document_initialized = true;
            let _ = frame(&context, &mut generated, Vec::new());
            let generated_shown = header(&frame(&context, &mut generated, Vec::new()));
            assert!(
                generated_shown.iter().any(|text| text == "RSpice"),
                "{generated_shown:?}"
            );
            for text in shown.iter().chain(&generated_shown) {
                for word in ["OWNED", "EDITABLE", "READ-ONLY", "GENERATED", "SPICE ·"] {
                    assert!(!text.contains(word), "{text:?} in the header");
                }
            }
        }

        /// LTspice chosen from the header's chip is one edit to the deck's
        /// text, which one Undo takes back.
        #[test]
        fn choosing_a_dialect_from_the_header_is_one_edit_one_undo_takes_back() {
            let source = "amp\nR1 a 0 1k\n.op\n.end\n";
            let context = context();
            let mut harness = deck("amp.cir", source);
            let _ = frame(&context, &mut harness, Vec::new());
            let menu = press(&context, &mut harness, "RSpice");
            assert!(
                menu.iter().any(|(text, _)| text == "QSPICE"),
                "the menu is open: {menu:?}"
            );
            let _ = press(&context, &mut harness, "LTspice");
            assert_eq!(
                harness.model.simulation.source.netlist_content,
                "amp\n.options rspice_source=ltspice\nR1 a 0 1k\n.op\n.end\n"
            );
            let editor_id = editor_id_in(&harness.model, &harness.state.netlist);
            crate::workbench::documents::text_editor_commands::queue_source_editor_request(
                &context,
                editor_id,
                SourceEditorRequest::Standard(StandardCommand::Undo),
            );
            let _ = frame(&context, &mut harness, Vec::new());
            let _ = frame(&context, &mut harness, Vec::new());
            assert_eq!(harness.model.simulation.source.netlist_content, source);
        }

        /// A `.scs` deck is checked by the Spectre front end: the header
        /// names Spectre and offers no menu, and the check reports none of
        /// the SPICE parser's reading of Spectre syntax.
        #[test]
        fn a_spectre_deck_is_named_spectre_and_checked_as_spectre() {
            let context = context();
            let mut harness = deck(
                "amp.scs",
                "simulator lang=spectre\nv1 (a 0) vsource dc=1\nr1 (a 0) resistor r=1k\n",
            );
            let _ = frame(&context, &mut harness, Vec::new());
            let shown = header(&frame(&context, &mut harness, Vec::new()));
            assert!(shown.iter().any(|text| text == "Spectre"), "{shown:?}");
            for choice in ["RSpice", "ngspice", "LTspice", "QSPICE"] {
                assert!(!shown.iter().any(|text| text == choice), "{shown:?}");
            }
            let findings = harness
                .state
                .netlist
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.message.clone())
                .collect::<Vec<_>>();
            assert!(
                !findings
                    .iter()
                    .any(|message| message.contains("Expected node name")),
                "{findings:?}"
            );
        }

        /// A deck whose text names LTspice and declares nothing shows the
        /// band, and its button declares LTspice in the deck.
        #[test]
        fn the_hint_band_reads_the_deck_as_the_program_its_text_names() {
            let context = context();
            let mut harness = deck("amp.cir", "* amp.asc\nR1 a 0 1k\n.backanno\n.end\n");
            let _ = frame(&context, &mut harness, Vec::new());
            let shown = frame(&context, &mut harness, Vec::new());
            assert!(
                shown
                    .iter()
                    .any(|(text, _)| text == "Written by LTspice? .backanno on line 3."),
                "{shown:?}"
            );
            let _ = press(&context, &mut harness, "Read as LTspice");
            assert_eq!(
                harness.model.simulation.source.netlist_content,
                "* amp.asc\n.options rspice_source=ltspice\nR1 a 0 1k\n.backanno\n.end\n"
            );
        }
    }
}
