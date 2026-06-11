//! PZ — pole-zero map on the s-plane: stable half shaded, poles as crosses,
//! zeros as circles, and the root table with ζ / ωₙ in the right panel.

use egui::Ui;

use crate::analysis::pole_zero::PoleZeroData;
use crate::common::AppState;
use crate::ui::plot::{self, Axis, PlotSpec, XScale, fmt_si};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;

use super::strip::{self, LegendChip};
use super::well_hint;

fn active_data(state: &AppState) -> Option<&PoleZeroData> {
    let pz = &state.analysis.pole_zero_state;
    pz.datasets
        .get(pz.selected)
        .or_else(|| pz.datasets.first())
        .filter(|data| !data.is_empty())
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the s-plane map.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let Some(data) = active_data(state) else {
        well_hint(ui, "No root data — run a pole-zero analysis");
        return;
    };

    let legend = [
        LegendChip {
            name: "poles x",
            color: c.traces[5],
            on: true,
        },
        LegendChip {
            name: "zeros ○",
            color: c.traces[1],
            on: true,
        },
    ];
    strip::header(
        ui,
        "PZ",
        &format!("{} · {} roots", data.name, data.roots.len()),
        &legend,
        false,
        false,
        false,
    );

    // Symmetric ranges around the roots.
    let mut extent = 1.0f64;
    for root in &data.roots {
        if root.real.is_finite() && root.imag.is_finite() {
            extent = extent.max(root.real.abs()).max(root.imag.abs());
        }
    }
    let extent = extent * 1.25;

    let mut spec = PlotSpec::new(
        Axis::linear(-extent, extent, "σ"),
        XScale::Linear,
        Axis::linear(-extent, extent, "jω"),
    );
    spec.ref_lines.push(plot::RefLine { y: 0.0 });
    // Stable left half-plane.
    spec.bands.push(plot::Band {
        x0: -extent,
        x1: 0.0,
    });

    // Root glyphs as an underlay: ✕ for poles, ○ for zeros.
    let roots = data.roots.clone();
    let pole_color = c.traces[5];
    let zero_color = c.traces[1];
    let grid = c.canvas_grid;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        painter.vline(
            mapper.x(0.0),
            egui::Rangef::new(mapper.rect.top(), mapper.rect.bottom()),
            egui::Stroke::new(1.0, grid),
        );
        for root in &roots {
            let center = egui::pos2(mapper.x(root.real), mapper.y(root.imag));
            if root.is_pole() {
                let r = 4.5;
                let stroke = egui::Stroke::new(1.8, pole_color);
                painter.line_segment(
                    [center + egui::vec2(-r, -r), center + egui::vec2(r, r)],
                    stroke,
                );
                painter.line_segment(
                    [center + egui::vec2(-r, r), center + egui::vec2(r, -r)],
                    stroke,
                );
            } else {
                painter.circle_stroke(center, 4.5, egui::Stroke::new(1.8, zero_color));
            }
        }
    }));

    // Square region for honest geometry.
    let avail = ui.available_rect_before_wrap();
    let side = avail.width().min(avail.height());
    let rect = egui::Rect::from_center_size(avail.center(), egui::vec2(side, side));
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );

    plot::show(&mut plot_ui, &spec, &mut state.shell.results.cache, None, None);
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Root table with damping and natural frequency.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Roots", None);
    let Some(data) = active_data(state) else {
        super::panel_note(ui, "Roots appear once a pole-zero analysis runs.");
        return;
    };

    let unstable = data
        .roots
        .iter()
        .any(|root| root.is_pole() && root.real > 0.0);
    let mut rows: Vec<(String, String, bool)> = vec![
        (
            "Verdict".to_owned(),
            if unstable { "unstable" } else { "stable" }.to_owned(),
            true,
        ),
        ("Gain".to_owned(), format!("{:.4}", data.gain), false),
    ];
    let mut pole_index = 0usize;
    let mut zero_index = 0usize;
    for root in data.roots.iter().take(12) {
        // Conjugates are listed once via the +jω member.
        if root.imag < 0.0 {
            continue;
        }
        let name = if root.is_pole() {
            pole_index += 1;
            format!("p{pole_index}")
        } else {
            zero_index += 1;
            format!("z{zero_index}")
        };
        let value = if root.is_real() {
            fmt_si(root.real, "", 3)
        } else {
            format!(
                "ζ {:.2} · ωₙ {}",
                root.damping_ratio(),
                fmt_si(root.natural_frequency(), "", 2)
            )
        };
        rows.push((name, value, false));
    }
    let row_refs: Vec<(&str, String, bool)> = rows
        .iter()
        .map(|(k, v, h)| (k.as_str(), v.clone(), *h))
        .collect();
    super::stat_table(ui, &row_refs);
    super::panel_note(
        ui,
        "Crosses are poles, circles are zeros; the shaded half-plane is stable.",
    );
}
