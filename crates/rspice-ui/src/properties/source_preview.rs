//! The one painter of an independent source's engine-evaluated waveform.
//!
//! Two surfaces draw this curve — the component editor's evidence pane and the
//! stimulus link dialog, which shows what a card would look like *after* a
//! definition was adopted onto the instance — and they have to agree down to
//! the axis labels. Three implementations of the source families used to
//! exist, which is the defect `simulation::stimulus_realize` closed for the
//! values; this closes the same class for the drawing.
//!
//! It owns no semantics either. The component becomes a card, the card becomes
//! a `SourceSpec` through the engine's own parser, and the curve is the
//! transient evaluator stepped over the stop time; everything here is geometry
//! and type.

use egui::{Sense, Stroke, Ui, pos2, vec2};

use crate::simulation::stimulus_realize::{self, PreviewTiming, WaveformReadouts};
use crate::state::Component;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// How many points the preview asks the engine for.
///
/// Finer than the stroke at the card's widest, and the density the card has
/// always drawn at.
const PREVIEW_SAMPLES: usize = 96;

/// The card's height, shared so the two surfaces reserve the same block.
pub(crate) const PREVIEW_HEIGHT: f32 = 132.0;

/// This source's waveform over the given transient, or why the engine cannot
/// draw one.
///
/// The generator's and the parser's refusals already name the component and
/// the field, so they are carried through unrewritten: a preview that
/// reworded them would be a second voice saying the same thing differently.
pub(crate) fn source_curve(
    component: &Component,
    timing: PreviewTiming,
) -> Result<Vec<(f64, f64)>, String> {
    let spec = stimulus_realize::source_spec(component)?;
    match stimulus_realize::preview_defect(&spec) {
        Some(defect) => Err(defect),
        None => Ok(stimulus_realize::evaluate_waveform(
            &spec,
            timing.window(PREVIEW_SAMPLES),
            timing.tstep,
            timing.tstop,
            stimulus_realize::PREVIEW_DIALECT,
        )),
    }
}

/// Paint one evaluated curve, its axes and its readouts.
///
/// The block is allocated at [`PREVIEW_HEIGHT`] whatever the outcome: a card
/// that vanished when the engine could not draw it would look like a source
/// the editor does not understand, so a refusal is stated inside the same box
/// the curve would have filled.
pub(crate) fn paint_source_preview(
    ui: &mut Ui,
    curve: &Result<Vec<(f64, f64)>, String>,
    timing: PreviewTiming,
) {
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), PREVIEW_HEIGHT), Sense::hover());
    let caption = timing.caption();
    let readouts = curve
        .as_ref()
        .ok()
        .and_then(|samples| WaveformReadouts::of(samples));
    let label = match (curve, readouts) {
        (Ok(_), Some(_)) => format!("Engine-evaluated waveform of this source over {caption}"),
        (Ok(_), None) => format!("This source has no curve over {caption}"),
        (Err(reason), _) => reason.clone(),
    };
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Image, true, &label));

    let t = Tokens::get(ui.ctx());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    let plot = egui::Rect::from_min_max(rect.min + vec2(46.0, 8.0), rect.max - vec2(12.0, 20.0));
    ui.painter().text(
        pos2(plot.right(), rect.top() + 2.0),
        egui::Align2::RIGHT_TOP,
        &caption,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    ui.painter().line_segment(
        [plot.left_bottom(), plot.left_top()],
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().line_segment(
        [plot.left_bottom(), plot.right_bottom()],
        Stroke::new(1.0, t.color.border),
    );
    for fraction in [0.0_f32, 0.5, 1.0] {
        let y = egui::lerp(plot.bottom()..=plot.top(), fraction);
        ui.painter()
            .hline(plot.x_range(), y, Stroke::new(0.5, t.color.border));
    }

    let (samples, readouts) = match (curve, readouts) {
        (Err(reason), _) => {
            statement(ui, plot, reason, t.color.text_dim);
            rule(ui, rect, t.color.border);
            return;
        }
        (Ok(_), None) => {
            statement(
                ui,
                plot,
                "Preview unavailable until values are valid",
                t.color.text_dim,
            );
            rule(ui, rect, t.color.border);
            return;
        }
        (Ok(samples), Some(readouts)) => (samples, readouts),
    };

    let span = readouts.span().max(1e-12);
    for (fraction, label) in [
        (1.0_f32, format!("{:.2e}", readouts.maximum)),
        (0.5, format!("{:.2e}", readouts.midpoint)),
        (0.0, format!("{:.2e}", readouts.minimum)),
    ] {
        let y = egui::lerp(plot.bottom()..=plot.top(), fraction);
        ui.painter().text(
            pos2(plot.left() - 5.0, y),
            egui::Align2::RIGHT_CENTER,
            label,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    for (fraction, label) in [
        (0.0_f32, "0".to_owned()),
        (0.5, crate::state::format_engineering(timing.tstop * 0.5)),
        (1.0, crate::state::format_engineering(timing.tstop)),
    ] {
        let x = egui::lerp(plot.left()..=plot.right(), fraction);
        ui.painter().text(
            pos2(x, plot.bottom() + 5.0),
            egui::Align2::CENTER_TOP,
            format!("{label}s"),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    let points = samples
        .iter()
        .enumerate()
        .map(|(index, (_, value))| {
            let x = egui::lerp(
                plot.left()..=plot.right(),
                index as f32 / (samples.len() - 1) as f32,
            );
            let normalized = if readouts.span() <= 1e-12 {
                0.5
            } else {
                ((value - readouts.minimum) / span) as f32
            };
            let y = egui::lerp(plot.bottom()..=plot.top(), normalized);
            pos2(x, y)
        })
        .collect::<Vec<_>>();
    ui.painter()
        .add(egui::Shape::line(points, Stroke::new(1.5, t.color.accent)));
    rule(ui, rect, t.color.border);
}

fn statement(ui: &Ui, plot: egui::Rect, message: &str, color: egui::Color32) {
    ui.painter().text(
        plot.center(),
        egui::Align2::CENTER_CENTER,
        message,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        color,
    );
}

fn rule(ui: &Ui, rect: egui::Rect, color: egui::Color32) {
    ui.painter()
        .hline(rect.x_range(), rect.bottom(), Stroke::new(1.0, color));
}
