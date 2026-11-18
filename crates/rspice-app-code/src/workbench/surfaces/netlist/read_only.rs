//! The word the Netlist header shows beside the dialect chip while the
//! document in front cannot be edited: the generated netlist, a run's input
//! or an opened run's. An editable document says nothing.

use egui::{Align, Layout, Rect, Ui};

use crate::ui::theme;
use crate::ui::tokens::Tokens;
use crate::workbench::MessageId;
use crate::workbench::code::CodeFeature;
use crate::workbench::documents::netlist_document::active_netlist_source_is_editable_in;
use crate::workbench::framework::feature::Cx;

/// The space between the dialect chip and the word.
const GAP: f32 = 8.0;

/// Show "Read-only" in `language_rect` after the dialect chip, `chip_width`
/// wide, when the document in front cannot be edited and the word fits.
/// `None` for the chip says no document is shown, and nothing is.
pub(super) fn show_word(
    ui: &mut Ui,
    language_rect: Rect,
    chip_width: Option<f32>,
    cx: &Cx<'_, CodeFeature>,
) {
    let Some(chip_width) = chip_width else {
        return;
    };
    if active_netlist_source_is_editable_in(cx.model, &cx.state.netlist) {
        return;
    }
    let tokens = Tokens::get(ui.ctx());
    let word = cx.wb.messages().text(MessageId::NetlistReadOnly);
    let width = ui
        .painter()
        .layout_no_wrap(word.clone(), theme::field_sans(), tokens.color.text_dim)
        .size()
        .x;
    let left = language_rect.left() + chip_width + GAP;
    if left + width > language_rect.right() {
        return;
    }
    let mut word_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(Rect::from_x_y_ranges(
                left..=language_rect.right(),
                language_rect.y_range(),
            ))
            .layout(Layout::left_to_right(Align::Center)),
    );
    word_ui.add(
        egui::Label::new(
            egui::RichText::new(word)
                .font(theme::field_sans())
                .color(tokens.color.text_dim),
        )
        .selectable(false),
    );
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::workbench::code::tests::{CodeHarness, harness};
    use crate::workbench::documents::netlist_document::NetlistTab;
    use crate::workbench::framework::command_registry::CommandAvailability;
    use crate::workbench::framework::feature::{Feature, ForeignAvailability};

    /// The texts one frame of the page painted in its header.
    fn header(harness: &mut CodeHarness) -> Vec<String> {
        let context = egui::Context::default();
        crate::ui::Theme::default().apply(&context);
        let mut texts = Vec::new();
        for _ in 0..2 {
            let answers = ForeignAvailability::take(
                <crate::workbench::code::CodeFeature as Feature>::FOREIGN_AVAILABILITY,
                |_| CommandAvailability::Available,
            );
            let output = context.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    super::super::show_prepared(
                        ui,
                        &mut harness.cx().with_foreign_availability(answers),
                    );
                },
            );
            texts = output
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text)
                        if text.visual_bounding_rect().bottom()
                            <= super::super::toolbar::CODE_TOOLBAR_HEIGHT =>
                    {
                        Some(text.galley.text().to_owned())
                    }
                    _ => None,
                })
                .collect();
        }
        texts
    }

    #[test]
    fn the_header_says_read_only_over_the_generated_netlist_and_nothing_over_a_deck() {
        let mut generated = harness();
        super::super::fixtures::retain_generated(
            &mut generated.model,
            &mut generated.state.netlist,
            "generated\nR1 out 0 1k\n.op\n.end\n",
        );
        generated.model.simulation.source.netlist_content =
            generated.model.netlist_artifacts.generated_source.clone();
        generated.state.netlist.front = NetlistTab::Generated;
        generated.state.netlist.active_document_initialized = true;
        let shown = header(&mut generated);
        assert!(shown.iter().any(|text| text == "Read-only"), "{shown:?}");

        let mut deck = harness();
        let open = crate::workbench::documents::deck_files::OpenDeck::in_memory(
            "amp.cir",
            "amp\nR1 a 0 1k\n.op\n.end\n",
        );
        deck.model.workspace.content.decks = vec![crate::state::ProjectDeck {
            id: open.id,
            source: crate::state::DeckSource::File {
                path: PathBuf::from("amp.cir"),
            },
        }];
        crate::workbench::documents::deck_files::install_open_deck(
            &mut deck.model,
            &mut deck.state.netlist,
            &mut deck.state.netlist_open_documents,
            &mut deck.runs,
            open,
        );
        let shown = header(&mut deck);
        assert!(
            !shown.iter().any(|text| text == "Read-only"),
            "an editable deck says nothing: {shown:?}"
        );
    }
}
