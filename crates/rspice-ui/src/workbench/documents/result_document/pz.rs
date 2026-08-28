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
    let AnalysisResultPayload::PoleZero {
        poles,
        zeros,
        pole_evidence,
        zero_evidence,
        gain,
    } = payload
    else {
        return None;
    };
    if payload.validate_for(analysis.analysis_type).is_err() {
        return None;
    }

    let mut data = PoleZeroData::new(&analysis.label);
    data.gain = *gain;
    data.pole_evidence = pole_evidence.clone();
    data.zero_evidence = zero_evidence.clone();
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
    Unstable,
    Indeterminate,
}

impl PoleStabilityVerdict {
    const fn label(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Unstable => "unstable",
            Self::Indeterminate => "indeterminate",
        }
    }
}

fn pole_stability(data: &PoleZeroData) -> PoleStabilityVerdict {
    let poles = data
        .roots
        .iter()
        .filter(|root| root.is_pole())
        .collect::<Vec<_>>();
    if !data.pole_evidence.is_qualified()
        || !data.pole_evidence.is_consistent_with_count(poles.len())
    {
        return PoleStabilityVerdict::Indeterminate;
    }
    if poles.iter().all(|pole| pole.real < 0.0) {
        PoleStabilityVerdict::Stable
    } else {
        PoleStabilityVerdict::Unstable
    }
}

fn format_root_evidence(evidence: &crate::state::PoleZeroRootSetEvidence) -> String {
    evidence.certificate().map_or_else(
        || evidence.label().to_owned(),
        |certificate| {
            format!(
                "{} · order {} · infinite {} · residual {:.3e} / {:.3e}",
                evidence.label(),
                certificate.problem_order,
                certificate.infinite_count,
                certificate.max_backward_error,
                certificate.qualification_tolerance
            )
        },
    )
}

#[derive(Debug)]
struct PoleZeroSummary<'a> {
    pole_count: usize,
    zero_count: usize,
    dominant_pole: Option<&'a ComplexRoot>,
    worst_q: Option<f64>,
    right_half_plane_poles: usize,
    imaginary_axis_poles: usize,
}

fn imaginary_axis_tolerance(root: &ComplexRoot) -> f64 {
    64.0 * f64::EPSILON * root.real.abs().max(root.imag.abs()).max(1.0)
}

fn summarize_roots(data: &PoleZeroData) -> PoleZeroSummary<'_> {
    let poles = data
        .roots
        .iter()
        .filter(|root| root.is_pole())
        .collect::<Vec<_>>();
    let dominant_pole = poles.iter().copied().max_by(|left, right| {
        left.real
            .total_cmp(&right.real)
            .then_with(|| left.imag.total_cmp(&right.imag))
    });
    let mut worst_q: Option<f64> = None;
    let mut right_half_plane_poles = 0;
    let mut imaginary_axis_poles = 0;
    for pole in &poles {
        let tolerance = imaginary_axis_tolerance(pole);
        if pole.real > tolerance {
            right_half_plane_poles += 1;
        } else if pole.real.abs() <= tolerance {
            imaginary_axis_poles += 1;
        }
        let q = if pole.imag.abs() <= tolerance {
            None
        } else if pole.real.abs() <= tolerance {
            Some(f64::INFINITY)
        } else {
            let q = pole.natural_frequency() / (2.0 * pole.real.abs());
            (q.is_finite() && q >= 0.0).then_some(q)
        };
        if let Some(q) = q
            && worst_q.is_none_or(|current| q > current)
        {
            worst_q = Some(q);
        }
    }

    PoleZeroSummary {
        pole_count: poles.len(),
        zero_count: data.roots.len().saturating_sub(poles.len()),
        dominant_pole,
        worst_q,
        right_half_plane_poles,
        imaginary_axis_poles,
    }
}

fn accessible_root_inventory(data: &PoleZeroData) -> String {
    data.roots
        .iter()
        .enumerate()
        .map(|(index, root)| {
            format!(
                "{} {} sigma {:.9e}, omega {:.9e}",
                if root.is_pole() { "pole" } else { "zero" },
                index + 1,
                root.real,
                root.imag
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn retained_root_representatives(data: &PoleZeroData, limit: usize) -> (Vec<&ComplexRoot>, usize) {
    let representatives = data
        .roots
        .iter()
        .filter(|root| root.imag >= 0.0)
        .collect::<Vec<_>>();
    let omitted = representatives.len().saturating_sub(limit);
    (representatives.into_iter().take(limit).collect(), omitted)
}

fn format_dominant_pole(
    root: &ComplexRoot,
    quantity_policy: &crate::quantity::QuantityPresentationPolicy,
) -> String {
    if root.is_real() {
        let sign = if root.real < 0.0 { "−" } else { "+" };
        format!(
            "{sign}{}",
            quantity_policy.format_frequency(root.real.abs() / std::f64::consts::TAU, 3)
        )
    } else {
        format!("σ {:.6e} · jω {:+.6e} rad/s", root.real, root.imag)
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
    let view = state.ui.results.plot_view(super::ResultViewer::PoleZero, 0);
    let header = strip::StripHeader::new(
        "PZ",
        &format!("{} · {} roots", data.name, data.roots.len()),
        &legend,
    )
    .zoomed(view.is_zoomed())
    .show(ui);
    if header.fit_clicked {
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::PoleZero, 0);
    }

    // Symmetric ranges around the roots.
    let mut extent = 1.0f64;
    for root in &data.roots {
        if root.real.is_finite() && root.imag.is_finite() {
            extent = extent.max(root.real.abs()).max(root.imag.abs());
        }
    }
    let extent = extent * 1.25;
    let (x0, x1) = view.x.unwrap_or((-extent, extent));
    let (y0, y1) = view.y.unwrap_or((-extent, extent));

    let summary = summarize_roots(&data);
    let inventory = accessible_root_inventory(&data);
    let accessible_detail = format!(
        "{} poles and {} zeros; {} pole stability verdict; stable left half-plane shaded; exact retained roots: {inventory}",
        summary.pole_count,
        summary.zero_count,
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
        x0,
        x1: 0.0_f64.clamp(x0, x1),
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
    super::record_drawn_axes(
        &mut state.ui.results,
        super::ResultViewer::PoleZero,
        &response,
    );
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
    let summary = summarize_roots(&data);
    let dominant = summary.dominant_pole.map_or_else(
        || "Unavailable — no retained poles".to_owned(),
        |root| format_dominant_pole(root, &quantity_policy),
    );
    let worst_q = summary.worst_q.map_or_else(
        || "Unavailable — no retained pole Q".to_owned(),
        |q| {
            if q.is_infinite() {
                "∞ · imaginary-axis pole".to_owned()
            } else {
                format!("{q:.4}")
            }
        },
    );
    let rhp = if summary.right_half_plane_poles == 0 {
        "none".to_owned()
    } else {
        format!("{} poles", summary.right_half_plane_poles)
    };
    let axis = if summary.imaginary_axis_poles == 0 {
        "none".to_owned()
    } else {
        format!("{} poles", summary.imaginary_axis_poles)
    };
    let summary_rows = [
        ("Verdict".to_owned(), verdict.label().to_owned(), true),
        (
            "Poles / zeros".to_owned(),
            format!("{} / {} · retained", summary.pole_count, summary.zero_count),
            false,
        ),
        (
            "Pole evidence".to_owned(),
            format_root_evidence(&data.pole_evidence),
            false,
        ),
        (
            "Zero evidence".to_owned(),
            format_root_evidence(&data.zero_evidence),
            false,
        ),
        ("Dominant pole".to_owned(), dominant, false),
        ("Worst Q".to_owned(), worst_q, false),
        ("RHP content".to_owned(), rhp, false),
        ("Imaginary axis".to_owned(), axis, false),
        (
            "Gain".to_owned(),
            data.gain
                .map(|gain| format!("{gain:.4}"))
                .unwrap_or_else(|| "Unavailable — no finite DC gain".to_owned()),
            false,
        ),
        (
            "Reduction".to_owned(),
            "Unavailable — matrix reduction not retained".to_owned(),
            false,
        ),
    ];
    let summary_refs: Vec<(&str, String, bool)> = summary_rows
        .iter()
        .map(|(key, value, highlight)| (key.as_str(), value.clone(), *highlight))
        .collect();
    super::stat_table(ui, &summary_refs);

    ui.add_space(8.0);
    section_header(ui, "Retained roots", None);
    let mut rows: Vec<(String, String, bool)> = Vec::new();
    let mut pole_index = 0usize;
    let mut zero_index = 0usize;
    let (representatives, omitted) = retained_root_representatives(&data, 12);
    for root in representatives {
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
    if omitted > 0 {
        super::panel_note(
            ui,
            &format!(
                "{omitted} additional retained root representatives are omitted from this compact table."
            ),
        );
    }
    super::panel_note(
        ui,
        "Conjugate pairs are listed once using the +jω member. Crosses are poles and circles are zeros. The shaded left half-plane is asymptotically stable; poles on the imaginary axis are marginal.",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisResultPayload, AnalysisType, ComplexResultValue, SimulationRun,
    };

    fn qualified_evidence(root_count: u64) -> crate::state::PoleZeroRootSetEvidence {
        let certificate = crate::state::PoleZeroSpectrumCertificate {
            problem_order: root_count,
            infinite_count: 0,
            max_backward_error: 1.0e-14,
            qualification_tolerance:
                crate::state::PoleZeroSpectrumCertificate::canonical_qualification_tolerance(
                    root_count,
                )
                .unwrap(),
        };
        if root_count == 0 {
            crate::state::PoleZeroRootSetEvidence::QualifiedEmpty { certificate }
        } else {
            crate::state::PoleZeroRootSetEvidence::Qualified { certificate }
        }
    }

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
                    pole_evidence: qualified_evidence(2),
                    zero_evidence: qualified_evidence(1),
                    gain: Some(4.25),
                },
            );

        let data = active_data(&state).expect("retained PZ payload");
        assert_eq!(data.name, "PZ 7");
        assert_eq!(data.gain, Some(4.25));
        assert_eq!(data.roots.len(), 3);
        assert!(data.roots[0].is_pole());
        assert_eq!((data.roots[0].real, data.roots[0].imag), (-10.0, 20.0));
        assert!(data.roots[1].is_pole());
        assert_eq!((data.roots[1].real, data.roots[1].imag), (-10.0, -20.0));
        assert!(data.roots[2].is_zero());
        assert_eq!((data.roots[2].real, data.roots[2].imag), (-3.0, 0.0));
    }

    #[test]
    fn qualified_imaginary_axis_poles_are_unstable() {
        let mut data = PoleZeroData::new("axis pole");
        data.roots.push(ComplexRoot::pole(0.0, 10.0));
        data.roots.push(ComplexRoot::pole(0.0, -10.0));
        data.pole_evidence = qualified_evidence(2);

        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Unstable);
    }

    #[test]
    fn right_half_plane_pole_overrides_marginal_poles() {
        let mut data = PoleZeroData::new("unstable");
        data.roots.push(ComplexRoot::pole(0.0, 10.0));
        data.roots.push(ComplexRoot::pole(0.5, 0.0));
        data.pole_evidence = qualified_evidence(2);

        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Unstable);
    }

    #[test]
    fn strictly_left_half_plane_poles_are_stable() {
        let mut data = PoleZeroData::new("stable");
        data.roots.push(ComplexRoot::pole(-0.5, 0.0));
        data.roots.push(ComplexRoot::pole(-2.0, 10.0));
        data.pole_evidence = qualified_evidence(2);

        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Stable);
    }

    #[test]
    fn unqualified_pole_evidence_is_always_indeterminate() {
        let mut data = PoleZeroData::new("unqualified");
        data.roots.push(ComplexRoot::pole(-1.0, 0.0));
        for evidence in [
            crate::state::PoleZeroRootSetEvidence::NotRequested,
            crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
            crate::state::PoleZeroRootSetEvidence::Approximate {
                certificate: crate::state::PoleZeroSpectrumCertificate {
                    problem_order: 1,
                    infinite_count: 0,
                    max_backward_error: 1.0e-9,
                    qualification_tolerance: crate::state::PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1).unwrap(),
                },
            },
        ] {
            data.pole_evidence = evidence;
            assert_eq!(pole_stability(&data), PoleStabilityVerdict::Indeterminate);
        }

        data.roots.clear();
        data.pole_evidence = crate::state::PoleZeroRootSetEvidence::NotRequested;
        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Indeterminate);
        data.pole_evidence = qualified_evidence(0);
        assert_eq!(pole_stability(&data), PoleStabilityVerdict::Stable);
    }

    #[test]
    fn root_summary_derives_mockup_metrics_only_from_retained_roots() {
        let mut data = PoleZeroData::new("summary");
        data.roots.push(ComplexRoot::pole(-100.0, 0.0));
        data.roots.push(ComplexRoot::pole(-10.0, 40.0));
        data.roots.push(ComplexRoot::pole(-10.0, -40.0));
        data.roots.push(ComplexRoot::zero(-3.0, 0.0));

        let summary = summarize_roots(&data);
        assert_eq!(summary.pole_count, 3);
        assert_eq!(summary.zero_count, 1);
        assert_eq!(summary.dominant_pole.map(|pole| pole.real), Some(-10.0));
        assert_eq!(summary.right_half_plane_poles, 0);
        assert_eq!(summary.imaginary_axis_poles, 0);
        assert!((summary.worst_q.unwrap() - 1700.0_f64.sqrt() / 20.0).abs() < 1.0e-12);
    }

    #[test]
    fn root_summary_reports_rhp_and_axis_content_without_inventing_reduction() {
        let mut data = PoleZeroData::new("mixed");
        data.roots.push(ComplexRoot::pole(1.0, 0.0));
        data.roots.push(ComplexRoot::pole(0.0, 2.0));

        let summary = summarize_roots(&data);
        assert_eq!(summary.right_half_plane_poles, 1);
        assert_eq!(summary.imaginary_axis_poles, 1);
        assert_eq!(summary.worst_q, Some(f64::INFINITY));
        assert!(accessible_root_inventory(&data).contains("pole 2"));
    }

    #[test]
    fn real_poles_do_not_invent_quality_factor() {
        let mut data = PoleZeroData::new("real poles");
        data.roots.push(ComplexRoot::pole(-1.0, 0.0));
        data.roots.push(ComplexRoot::pole(-10.0, 0.0));

        assert_eq!(summarize_roots(&data).worst_q, None);
    }

    #[test]
    fn root_table_filters_conjugates_before_applying_its_limit() {
        let mut data = PoleZeroData::new("ordered roots");
        for index in 1..=12 {
            data.roots
                .push(ComplexRoot::pole(-(index as f64), -(index as f64)));
        }
        for index in 1..=14 {
            data.roots
                .push(ComplexRoot::pole(-(index as f64), index as f64));
        }

        let (rows, omitted) = retained_root_representatives(&data, 12);

        assert_eq!(rows.len(), 12);
        assert!(rows.iter().all(|root| root.imag > 0.0));
        assert_eq!(omitted, 2);
    }
}
