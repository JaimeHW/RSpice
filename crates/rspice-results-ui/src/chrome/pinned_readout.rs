//! The readout a click pins on a plot sheet.
//!
//! Hovering a plot reads it at the pointer. A click pins that readout at the
//! X it chose: a line marks the place, and a card in the plot's corner keeps
//! the rows, read again each frame from what the sheet draws now, so the
//! next run shows its own values at the same X. The card's values are text
//! the reader can select, and its Copy button puts every row on the
//! clipboard as tab-separated text.

use egui::{Align, Layout, RichText, Stroke, Ui, UiBuilder, pos2};
use rspice_results::plot_axis::XScale;
use rspice_ui_kit::icons::Icon;
use rspice_ui_kit::plot::{PlotResponse, ReadoutRow};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::IconButton;

/// The card's inset from the plot's top-right corner.
const CARD_INSET: f32 = 8.0;
/// The side of the card's Copy and Unpin buttons.
const CARD_BUTTON_SIDE: f32 = 18.0;

/// After `plot::show`: a click on the plot pins `readout` at the X it chose,
/// and a pin already placed is marked and its card drawn. Returns whether
/// the reader placed, moved or took away the pin this frame.
pub fn follow(
    ui: &mut Ui,
    plot: &PlotResponse,
    x_scale: XScale,
    pin: &mut Option<f64>,
    readout: &dyn Fn(f64) -> Vec<ReadoutRow>,
) -> bool {
    let mut changed = false;
    if let Some(x) = plot.clicked_x.filter(|x| x.is_finite()) {
        changed = *pin != Some(x);
        *pin = Some(x);
    }
    let Some(x) = *pin else {
        return changed;
    };
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let rect = plot.plot_rect;
    let ((x0, x1), _) = plot.axes;
    let at = x_scale.normalize(x, x0, x1);
    if at.is_finite() && (0.0..=1.0).contains(&at) {
        let screen_x = rect.left() + rect.width() * at as f32;
        ui.painter().with_clip_rect(rect).line_segment(
            [pos2(screen_x, rect.top()), pos2(screen_x, rect.bottom())],
            Stroke::new(1.0, c.canvas_text),
        );
    }
    let rows = readout(x);
    if rows.is_empty() {
        return changed;
    }
    let font = theme::mono(11.0, FontWeight::Regular);
    let mut card = ui.new_child(
        UiBuilder::new()
            .id_salt("rspice.results.pinned-readout")
            .max_rect(rect.shrink(CARD_INSET))
            .layout(Layout::top_down(Align::Max)),
    );
    let mut unpin = false;
    egui::Frame::new()
        .fill(c.bg_elevated)
        .stroke(Stroke::new(1.0, c.border_strong))
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::symmetric(8, 5))
        .show(&mut card, |ui| {
            ui.set_max_width(rect.width() * 0.5);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                ui.label(RichText::new("Pinned").font(font.clone()).color(c.text_dim));
                if IconButton::new(Icon::Copy)
                    .side(CARD_BUTTON_SIDE)
                    .tooltip("Copy the pinned readout")
                    .show(ui)
                    .clicked()
                {
                    ui.ctx().copy_text(tab_separated(&rows));
                }
                unpin = IconButton::new(Icon::Close)
                    .side(CARD_BUTTON_SIDE)
                    .tooltip("Unpin the readout")
                    .show(ui)
                    .clicked();
            });
            egui::Grid::new("rows")
                .num_columns(2)
                .spacing(egui::vec2(10.0, 2.0))
                .show(ui, |ui| {
                    for (name, value) in &rows {
                        ui.add(
                            egui::Label::new(
                                RichText::new(name).font(font.clone()).color(c.text_dim),
                            )
                            .selectable(false),
                        );
                        ui.add(
                            egui::Label::new(RichText::new(value).font(font.clone()).color(c.text))
                                .selectable(true),
                        );
                        ui.end_row();
                    }
                });
        });
    if unpin {
        *pin = None;
        changed = true;
    }
    changed
}

/// The rows as the clipboard takes them: one per line, name and value
/// separated by a tab.
fn tab_separated(rows: &[ReadoutRow]) -> String {
    rows.iter()
        .map(|(name, value)| format!("{name}\t{value}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_ui_kit::plot::{self, Axis, DecimationCache, PlotSpec};

    /// One frame of a plot reading `level(x)`, with the pin followed.
    fn frame(
        ctx: &egui::Context,
        events: Vec<egui::Event>,
        pin: &mut Option<f64>,
        level: f64,
    ) -> egui::FullOutput {
        let mut cache = DecimationCache::default();
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 500.0),
                )),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let spec = PlotSpec::new(
                        Axis::linear(0.0, 10.0, "Hz"),
                        XScale::Linear,
                        Axis::linear(0.0, 1.0, "V"),
                    );
                    let readout = |x: f64| {
                        vec![
                            ("f".to_owned(), format!("{x:.3} Hz")),
                            ("level".to_owned(), format!("{level:.3} V")),
                        ]
                    };
                    let response = plot::show(ui, &spec, &mut cache, None, Some(&readout));
                    follow(ui, &response, spec.x_scale, pin, &readout);
                });
            },
        )
    }

    fn texts(output: &egui::FullOutput) -> Vec<String> {
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("an accessibility tree")
            .nodes
            .iter()
            .filter_map(|(_, node)| node.value().or(node.label()).map(str::to_owned))
            .collect()
    }

    /// A click pins the readout where it fell; the card reads what the plot
    /// draws now, so new data shows its own value at the same X; Copy puts
    /// the rows on the clipboard as tab-separated text.
    #[test]
    fn a_click_pins_the_readout_and_the_card_reads_the_data_drawn_now() {
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut pin = None;
        frame(&ctx, Vec::new(), &mut pin, 0.25);
        let at = egui::pos2(400.0, 250.0);
        for pressed in [true, false] {
            frame(
                &ctx,
                vec![
                    egui::Event::PointerMoved(at),
                    egui::Event::PointerButton {
                        pos: at,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
                &mut pin,
                0.25,
            );
        }
        let pinned = pin.expect("the click pinned the readout");
        assert!((0.0..10.0).contains(&pinned));
        let output = frame(&ctx, vec![egui::Event::PointerGone], &mut pin, 0.75);
        let shown = texts(&output);
        assert!(
            shown.iter().any(|text| text == "0.750 V"),
            "the card reads the data drawn now: {shown:?}"
        );
        assert_eq!(pin, Some(pinned), "new data leaves the pin where it was");
        assert_eq!(
            tab_separated(&[
                ("f".to_owned(), "1.000 Hz".to_owned()),
                ("level".to_owned(), "0.750 V".to_owned())
            ]),
            "f\t1.000 Hz\nlevel\t0.750 V"
        );
    }
}
