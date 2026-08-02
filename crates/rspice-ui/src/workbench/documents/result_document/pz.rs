//! PZ — pole-zero map on the s-plane: stable half shaded, poles as crosses,
//! zeros as circles, and the root table with ζ / ωₙ in the right panel.

use egui::Ui;

use crate::analysis::pole_zero::{ComplexRoot, PoleZeroData};
use crate::state::{AnalysisResultPayload, AnalysisType};
use crate::ui::plot::{self, Axis, PlotSpec, XScale, fmt_si};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;
use crate::workbench::AppState;

use super::strip::{self, LegendChip};
use super::well_hint;

fn active_data(state: &AppState) -> Option<PoleZeroData> {
    let analysis = state.simulation.active_analysis()?;
    if !analysis.success || analysis.analysis_type != AnalysisType::PoleZero {
        return None;
    }
    let payload = analysis.result_payload.as_ref()?;
    let AnalysisResultPayload::PoleZero { poles, zeros, gain } = payload else {
        return None;
    };
    if payload.validate_for(analysis.analysis_type).is_err() {
        return None;
    }

    let mut data = PoleZeroData::new(&analysis.label);
    data.gain = *gain;
    data.roots.extend(
        poles
            .iter()
            .map(|root| ComplexRoot::pole(root.real, root.imaginary)),
    );
    data.roots.extend(
        zeros
            .iter()
            .map(|root| ComplexRoot::zero(root.real, root.imaginary)),
    );
    Some(data)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PoleStabilityVerdict {
    Stable,
    Marginal,
    Unstable,
}

impl PoleStabilityVerdict {
    const fn label(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Marginal => "marginal",
            Self::Unstable => "unstable",
        }
    }
}

fn pole_stability(data: &PoleZeroData) -> PoleStabilityVerdict {
    let mut marginal = false;
    for pole in data.roots.iter().filter(|root| root.is_pole()) {
        let scale = pole.real.abs().max(pole.imag.abs()).max(1.0);
        let imaginary_axis_tolerance = 64.0 * f64::EPSILON * scale;
        if pole.real > imaginary_axis_tolerance {
            return PoleStabilityVerdict::Unstable;
        }
        if pole.real.abs() <= imaginary_axis_tolerance {
            marginal = true;
        }
    }
    if marginal {
        PoleStabilityVerdict::Marginal
    } else {
        PoleStabilityVerdict::Stable
    }
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the s-plane map.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();

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
            name: "zeros",
            color: c.traces[1],
            on: true,
        },
    ];
    strip::StripHeader::new(
        "PZ",
        &format!("{} · {} roots", data.name, data.roots.len()),
        &legend,
    )
    .show(ui);

    // Symmetric ranges around the roots.
    let mut extent = 1.0f64;
    for root in &data.roots {
        if root.real.is_finite() && root.imag.is_finite() {
            extent = extent.max(root.real.abs()).max(root.imag.abs());
        }
    }
    let extent = extent * 1.25;
    let view = state.ui.results.plot_view(super::ResultViewer::PoleZero, 0);
    let (x0, x1) = view.x.unwrap_or((-extent, extent));
    let (y0, y1) = view.y.unwrap_or((-extent, extent));

    let pole_count = data.roots.iter().filter(|root| root.is_pole()).count();
    let zero_count = data.roots.len().saturating_sub(pole_count);
    let accessible_detail = format!(
        "{pole_count} poles and {zero_count} zeros; {} pole stability verdict; stable left half-plane shaded",
        pole_stability(&data).label()
    );
    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, "σ"),
        XScale::Linear,
        Axis::linear(y0, y1, "jω"),
    )
    .accessible_name("Pole-zero plot")
    .accessible_detail(&accessible_detail);
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

    // Region sized so the INNER plot area is square — σ and jω on one scale.
    let avail = ui.available_rect_before_wrap();
    let rect = plot::square_outer_rect(avail, &spec);
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );

    let response = plot::show(&mut plot_ui, &spec, &mut state.ui.results.cache, None, None);
    if response.view.any() {
        let change = super::square_xy_view_change((x0, x1), (y0, y1), response.view);
        state
            .ui
            .results
            .plot_view_mut(super::ResultViewer::PoleZero, 0)
            .apply(&change);
    }

    // Nearest root on hover, click to pin: σ, jω, natural frequency, and Q
    // turn the s-plane picture into numbers.
    let ranges = ((x0, x1), (y0, y1));
    let mut hovered: Option<(usize, usize)> = None;
    if let Some(pointer) = response.response.hover_pos()
        && response.plot_rect.contains(pointer)
    {
        let mut best = 16.0f32 * 16.0;
        for (i, root) in data.roots.iter().enumerate() {
            let pos = super::xy_screen_pos(
                response.plot_rect,
                (root.real, root.imag),
                ranges.0,
                ranges.1,
            );
            let d2 = pos.distance_sq(pointer);
            if d2 < best {
                best = d2;
                hovered = Some((0, i));
            }
        }
    }

    if response.response.clicked() {
        let pins = &mut state.ui.results.rf_pin;
        match hovered {
            Some(hit) if pins.get(&super::ResultViewer::PoleZero) == Some(&hit) => {
                pins.remove(&super::ResultViewer::PoleZero);
            }
            Some(hit) => {
                pins.insert(super::ResultViewer::PoleZero, hit);
            }
            None => {
                pins.remove(&super::ResultViewer::PoleZero);
            }
        }
    }

    let pinned = state
        .ui
        .results
        .rf_pin
        .get(&super::ResultViewer::PoleZero)
        .copied()
        .filter(|(_, i)| *i < data.roots.len());
    let target = hovered.or(pinned);

    if let Some((_, i)) = target {
        let root = &data.roots[i];
        let pos = super::xy_screen_pos(
            response.plot_rect,
            (root.real, root.imag),
            ranges.0,
            ranges.1,
        );
        let color = if root.is_pole() {
            c.traces[5]
        } else {
            c.traces[1]
        };
        let painter = plot_ui.painter();
        if pinned == target {
            painter.circle_stroke(pos, 8.0, egui::Stroke::new(1.8, color));
        }
        painter.circle_stroke(pos, 6.0, egui::Stroke::new(1.5, c.accent));

        let magnitude = (root.real * root.real + root.imag * root.imag).sqrt();
        let mut rows = vec![
            ("σ".to_owned(), fmt_si(root.real, "", 3)),
            ("jω".to_owned(), fmt_si(root.imag, "", 3)),
            (
                "fₙ".to_owned(),
                quantity_policy.format_frequency(magnitude / std::f64::consts::TAU, 2),
            ),
        ];
        if root.imag.abs() > 1e-12 && root.real.abs() > 1e-12 {
            rows.push((
                "Q".to_owned(),
                format!("{:.2}", magnitude / (2.0 * root.real.abs())),
            ));
        }
        let title = if root.is_pole() { "pole" } else { "zero" };
        super::point_card(&plot_ui, response.plot_rect, pos, title, color, &rows);
    }
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
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();

    let verdict = pole_stability(&data);
    let mut rows: Vec<(String, String, bool)> = vec![
        ("Verdict".to_owned(), verdict.label().to_owned(), true),
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
                "ζ {:.2} · fₙ {}",
                root.damping_ratio(),
                quantity_policy
                    .format_frequency(root.natural_frequency() / std::f64::consts::TAU, 2)
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
        "Crosses are poles and circles are zeros. The shaded left half-plane is asymptotically stable; poles on the imaginary axis are marginal.",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisResultPayload, AnalysisType, ComplexResultValue, SimulationRun,
    };

    #[test]
    fn retained_payload_is_the_only_pole_zero_viewer_authority() {
        let mut state = AppState::default();
        let mut stale = PoleZeroData::new("stale");
        stale.add_real_pole(-99.0);
        state.analysis.pole_zero_state.load_data(stale);

        let mut run = SimulationRun::new(1);
        run.add_analysis(AnalysisResult::new(7, AnalysisType::PoleZero, "PZ 7"));
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        assert!(active_data(&state).is_none());

        state.simulation.runs[0].analyses[0] =
            AnalysisResult::new(7, AnalysisType::PoleZero, "PZ 7").with_result_payload(
                AnalysisResultPayload::PoleZero {
                    poles: vec![
                        ComplexResultValue {
                            real: -10.0,
                            imaginary: 20.0,
                        },
                        ComplexResultValue {
                            real: -10.0,
                            imaginary: -20.0,
                        },
                    ],
                    zeros: vec![ComplexResultValue {
                        real: -3.0,
                        imaginary: 0.0,
                    }],
                    gain: 4.25,
                },
            );

        let data = active_data(&state).expect("retained PZ payload");
        assert_eq!(data.name, "PZ 7");
        assert_eq!(data.gain, 4.25);
        assert_eq!(data.roots.len(), 3);
        assert!(data.roots[0].is_pole());
        assert_eq!((data.roots[0].real, data.roots[0].imag), (-10.0, 20.0));
        assert!(data.roots[1].is_pole());
        assert_eq!((data.roots[1].real, data.roots[1].imag), (-10.0, -20.0));
        assert!(data.roots[2].is_zero());
        assert_eq!((data.roots[2].real, data.roots[2].imag), (-3.0, 0.0));
    }

    #[test]
    fn imaginary_axis_poles_are_marginal_not_stable() {
        let mut data = PoleZeroData::new("axis pole");
        data.roots.push(ComplexRoot::pole(0.0, 10.0));
        data.roots.push(ComplexRoot::pole(0.0, -10.0));

        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Marginal);
    }

    #[test]
    fn right_half_plane_pole_overrides_marginal_poles() {
        let mut data = PoleZeroData::new("unstable");
        data.roots.push(ComplexRoot::pole(0.0, 10.0));
        data.roots.push(ComplexRoot::pole(0.5, 0.0));

        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Unstable);
    }

    #[test]
    fn strictly_left_half_plane_poles_are_stable() {
        let mut data = PoleZeroData::new("stable");
        data.roots.push(ComplexRoot::pole(-0.5, 0.0));
        data.roots.push(ComplexRoot::pole(-2.0, 10.0));

        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Stable);
    }
}
