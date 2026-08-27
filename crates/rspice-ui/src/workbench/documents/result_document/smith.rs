//! SMITH — S-parameter loci on the reflection-coefficient plane with the
//! canonical resistance/reactance grid, in design-system colors.
//!
//! The chart grid is painted as an underlay in `canvas_grid`; traces come
//! from the token cycle like every other viewer.

use egui::Ui;

use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;
use crate::workbench::AppState;
use crate::workbench::app_state::{
    ActiveViewer, SpecializedViewerAnalysisIdentity, SpecializedViewerCacheProvenance,
};

use super::frame_work::{self, DatasetWalk};
use super::strip::{self, LegendChip};
use super::well_hint;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SParameterTraceIdentity {
    output_port: usize,
    input_port: usize,
    physical_ports: bool,
}

fn trace_identity(name: &str) -> Option<SParameterTraceIdentity> {
    let core = name
        .trim()
        .trim_matches('|')
        .split_once('[')
        .map_or_else(|| name.trim().trim_matches('|'), |(core, _)| core);
    let suffix = core.strip_prefix('S').or_else(|| core.strip_prefix('s'))?;
    let (physical_ports, indices) = match suffix.get(..2) {
        Some(prefix)
            if matches!(
                prefix.to_ascii_lowercase().as_str(),
                "dd" | "dc" | "cd" | "cc"
            ) =>
        {
            (false, &suffix[2..])
        }
        _ => (true, suffix),
    };
    let (output_port, input_port) = if let Some((output, input)) = indices.split_once('_') {
        (output.parse().ok()?, input.parse().ok()?)
    } else if indices.len() == 2 && indices.bytes().all(|byte| byte.is_ascii_digit()) {
        let bytes = indices.as_bytes();
        ((bytes[0] - b'0') as usize, (bytes[1] - b'0') as usize)
    } else {
        return None;
    };
    (output_port > 0 && input_port > 0).then_some(SParameterTraceIdentity {
        output_port,
        input_port,
        physical_ports,
    })
}

fn trace_is_well_formed(waveform: &crate::state::WaveformData) -> bool {
    let Some(complex) = waveform.complex.as_ref() else {
        return false;
    };
    !waveform.x.is_empty()
        && waveform.x.len() == complex.real.len()
        && waveform.x.len() == complex.imag.len()
        && waveform
            .x
            .iter()
            .all(|frequency| frequency.is_finite() && *frequency > 0.0)
        && waveform.x.windows(2).all(|pair| pair[0] < pair[1])
        && complex
            .real
            .iter()
            .chain(complex.imag.iter())
            .all(|value| value.is_finite())
}

pub(super) fn analysis_is_renderable(analysis: &crate::state::AnalysisResult) -> bool {
    frame_work::note(DatasetWalk::EvidenceValidation);
    analysis.validate_retained_evidence().is_ok() && structure_is_renderable(analysis)
}

/// The same question with the retained-evidence verdict left to the caller.
///
/// Split out because the per-frame callers — the tab strip's availability
/// gate and the cache synchronizer — take that verdict from the workspace
/// memo instead of walking every retained complex sample again.
pub(super) fn structure_is_renderable(analysis: &crate::state::AnalysisResult) -> bool {
    if !analysis.success
        || !matches!(
            analysis.analysis_type,
            crate::state::AnalysisType::SParameter
                | crate::state::AnalysisType::Psp
                | crate::state::AnalysisType::Hbsp
        )
    {
        return false;
    }
    let Some(crate::state::AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm,
    }) = analysis.family_metadata.as_ref()
    else {
        return false;
    };
    analysis.waveforms.iter().any(|waveform| {
        let Some(identity) = trace_identity(&waveform.name) else {
            return false;
        };
        let port_count = if identity.physical_ports {
            reference_impedances_ohm.len()
        } else if reference_impedances_ohm.len().is_multiple_of(2) {
            reference_impedances_ohm.len() / 2
        } else {
            return false;
        };
        identity.output_port <= port_count
            && identity.input_port <= port_count
            && trace_is_well_formed(waveform)
    })
}

/// Rebuild the mutable drawing cache solely from the selected immutable
/// result. Cache provenance makes repeated frames a no-op and prevents a
/// same-shape result from inheriting another run's RF traces.
pub(super) fn synchronize_active_analysis(state: &mut AppState) -> bool {
    if state.viewer_capability(ActiveViewer::SmithChart).available {
        return true;
    }
    let Some(run) = state.simulation.active_run() else {
        state.analysis.smith_chart_state.clear_traces();
        state.clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
        return false;
    };
    let Some(analysis) = state.simulation.active_analysis() else {
        state.analysis.smith_chart_state.clear_traces();
        state.clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
        return false;
    };
    if !structure_is_renderable(analysis)
        || !super::analysis_evidence_is_valid(state, run.dataset_id, analysis)
    {
        state.analysis.smith_chart_state.clear_traces();
        state.clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
        return false;
    }
    let provenance = SpecializedViewerCacheProvenance::for_analysis(run.dataset_id, analysis);
    let crate::state::AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm,
    } = analysis
        .family_metadata
        .as_ref()
        .expect("renderable S-parameter metadata")
    else {
        unreachable!("renderability checked the S-parameter metadata variant");
    };
    let mut waveforms = analysis.waveforms.clone();
    let reference_impedances_ohm = reference_impedances_ohm.clone();
    waveforms.sort_by(|left, right| left.name.cmp(&right.name));

    state.analysis.smith_chart_state.clear_traces();
    for waveform in waveforms {
        let Some(identity) = trace_identity(&waveform.name) else {
            continue;
        };
        let port_count = if identity.physical_ports {
            reference_impedances_ohm.len()
        } else {
            reference_impedances_ohm.len() / 2
        };
        if identity.output_port > port_count || identity.input_port > port_count {
            continue;
        }
        let reference_impedance_ohm = (identity.physical_ports
            && identity.output_port == identity.input_port)
            .then(|| reference_impedances_ohm[identity.output_port - 1]);
        let Some(complex) = waveform.complex.as_ref() else {
            continue;
        };
        if state
            .analysis
            .smith_chart_state
            .load_sparam_data(
                &waveform.name,
                &waveform.x,
                &complex.real,
                &complex.imag,
                reference_impedance_ohm,
            )
            .is_err()
        {
            state.analysis.smith_chart_state.clear_traces();
            state.clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
            return false;
        }
    }
    if state.analysis.smith_chart_state.traces.is_empty() {
        state.clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
        return false;
    }
    state.bind_specialized_viewer_cache(ActiveViewer::SmithChart, provenance);
    true
}

fn impedance_from_gamma(gamma_re: f64, gamma_im: f64, z0: f64) -> Option<(f64, f64)> {
    let denominator = (1.0 - gamma_re).powi(2) + gamma_im.powi(2);
    (denominator > 1.0e-12).then(|| {
        (
            z0 * (1.0 - gamma_re * gamma_re - gamma_im * gamma_im) / denominator,
            z0 * (2.0 * gamma_im) / denominator,
        )
    })
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the Smith chart.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();

    let smith = &state.analysis.smith_chart_state;
    let visible: Vec<usize> = smith
        .traces
        .iter()
        .enumerate()
        .filter(|(_, tr)| tr.visible && !tr.points.is_empty())
        .map(|(i, _)| i)
        .collect();
    if visible.is_empty() {
        well_hint(
            ui,
            "No S-parameter traces — run an SP, PSP, or HBSP analysis",
        );
        return;
    }

    // Γ-plane component arrays per trace, cached per data version.
    //
    // The cache and the traces live in disjoint halves of the session, so the
    // arrays are built by reference. Copying the loci out first — to release
    // the borrow — meant every frame paid for a full copy of every visible
    // trace's points before finding out the cache already held them.
    //
    // The key carries the cache's owning result as well as the trace ordinal:
    // selecting another analysis of the same run reloads the loci without
    // moving the data version, and a bare ordinal would hand the new
    // selection the previous one's coefficients.
    let owner = state
        .active_specialized_viewer_cache_provenance()
        .map_or(0, |provenance| {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            provenance.dataset_id.hash(&mut hasher);
            match provenance.analysis_identity {
                SpecializedViewerAnalysisIdentity::Prepared(id) => {
                    id.hash(&mut hasher);
                }
                SpecializedViewerAnalysisIdentity::LegacyResultId(id) => id.hash(&mut hasher),
            }
            hasher.finish()
        });
    let mut arrays = Vec::new();
    {
        let smith = &state.analysis.smith_chart_state;
        let derived = &mut state.ui.results.derived;
        for &index in &visible {
            let points = &smith.traces[index].points;
            let key = owner ^ ((index as u64) << 8);
            let re = derived.get_or(key ^ 0x501_0000, || {
                std::sync::Arc::new(points.iter().map(|p| p.s.re).collect::<Vec<_>>())
            });
            let im = derived.get_or(key ^ 0x501_0001, || {
                std::sync::Arc::new(points.iter().map(|p| p.s.im).collect::<Vec<_>>())
            });
            arrays.push((index, re, im));
        }
    }

    let smith = &state.analysis.smith_chart_state;
    let legend: Vec<LegendChip> = visible
        .iter()
        .enumerate()
        .map(|(slot, &index)| LegendChip {
            name: &smith.traces[index].name,
            color: c.traces[slot % c.traces.len()],
            on: true,
        })
        .collect();
    let mut retained_references = visible
        .iter()
        .filter_map(|index| smith.traces[*index].reference_impedance_ohm)
        .collect::<Vec<_>>();
    retained_references.sort_by(f64::total_cmp);
    retained_references.dedup_by(|left, right| left.to_bits() == right.to_bits());
    let reference_label = match retained_references.as_slice() {
        [reference] => format!("Z₀ = {reference} Ω"),
        [] => "Coefficient loci".to_owned(),
        _ => "Per-port retained Z₀".to_owned(),
    };
    strip::StripHeader::new("SMITH", &reference_label, &legend).show(ui);

    let view = state.ui.results.plot_view(super::ResultViewer::Smith, 0);
    let (x0, x1) = view.x.unwrap_or((-1.12, 1.12));
    let (y0, y1) = view.y.unwrap_or((-1.12, 1.12));
    let accessible_detail = format!(
        "{} visible loci with {} retained samples; {} reflection references retained",
        visible.len(),
        arrays.iter().map(|(_, re, _)| re.len()).sum::<usize>(),
        visible
            .iter()
            .filter(|index| smith.traces[**index].reference_impedance_ohm.is_some())
            .count()
    );
    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, "Re Γ"),
        XScale::Linear,
        Axis::linear(y0, y1, "Im Γ"),
    )
    .accessible_name("Smith chart")
    .accessible_detail(&accessible_detail);
    for (slot, (index, re, im)) in arrays.iter().enumerate() {
        spec.traces.push(
            // Re Γ is a coordinate, not an ordering: the locus crosses the
            // same abscissa on the way out and on the way back.
            Trace::new(re, im, c.traces[slot % c.traces.len()])
                .parametric()
                .cache_key(plot::trace_cache_key(0x501_00F0, *index)),
        );
    }

    // The canonical chart grid: constant-resistance circles and
    // constant-reactance arcs in token grid color, |Γ| = 1 boundary in
    // the strong border color.
    let grid = c.canvas_grid;
    let boundary = c.border_strong;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        let center = egui::pos2(mapper.x(0.0), mapper.y(0.0));
        let unit = (mapper.x(1.0) - mapper.x(0.0)).abs();
        let stroke = egui::Stroke::new(1.0, grid);

        // Constant resistance r: center ((r/(r+1)), 0), radius 1/(r+1).
        for r in [0.2, 0.5, 1.0, 2.0, 5.0] {
            let cx = r / (r + 1.0);
            let radius = 1.0 / (r + 1.0);
            painter.circle_stroke(
                egui::pos2(mapper.x(cx), mapper.y(0.0)),
                (radius * unit as f64) as f32,
                stroke,
            );
        }
        // Constant reactance x: circles centered (1, ±1/x) with radius
        // 1/x, clipped to the unit disc. Walk the full circle and emit a
        // polyline per contiguous in-disc run.
        for x in [0.2f64, 0.5, 1.0, 2.0, 5.0] {
            for sign in [1.0f64, -1.0] {
                let cy = sign / x;
                let radius = 1.0 / x;
                let mut run: Vec<egui::Pos2> = Vec::new();
                for step in 0..=128 {
                    let theta = std::f64::consts::TAU * step as f64 / 128.0;
                    let gx = 1.0 + radius * theta.cos();
                    let gy = cy + radius * theta.sin();
                    if gx * gx + gy * gy <= 1.0001 {
                        run.push(egui::pos2(mapper.x(gx), mapper.y(gy)));
                    } else if run.len() >= 2 {
                        painter.add(egui::Shape::line(std::mem::take(&mut run), stroke));
                    } else {
                        run.clear();
                    }
                }
                if run.len() >= 2 {
                    painter.add(egui::Shape::line(run, stroke));
                }
            }
        }
        // Real axis + boundary.
        painter.hline(
            egui::Rangef::new(mapper.x(-1.0), mapper.x(1.0)),
            center.y,
            stroke,
        );
        painter.circle_stroke(center, unit, egui::Stroke::new(1.2, boundary));
    }));

    // Region sized so the INNER plot area is square — the chart's circle
    // grid and the Γ loci must share one scale.
    let avail = ui.available_rect_before_wrap();
    let rect = plot::square_outer_rect(avail, &spec);
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );

    let response = plot::show(&mut plot_ui, &spec, &mut state.ui.results.cache, None, None);
    super::record_drawn_axes(&mut state.ui.results, super::ResultViewer::Smith, &response);
    if response.view.any() {
        let change = super::square_xy_view_change((x0, x1), (y0, y1), response.view);
        state
            .ui
            .results
            .plot_view_mut(super::ResultViewer::Smith, 0)
            .apply(&change);
    }

    // Interactive readout: nearest locus point on hover, click to pin.
    // The chart space is the complex-coefficient plane. Only a physical
    // diagonal Sii trace has the retained Z₀ needed for impedance and VSWR.
    let ranges = ((x0, x1), (y0, y1));
    let mut hovered: Option<(usize, usize)> = None;
    if let Some(pointer) = response.response.hover_pos()
        && response.plot_rect.contains(pointer)
    {
        let mut best = 14.0f32 * 14.0;
        for (slot, (_, re, im)) in arrays.iter().enumerate() {
            for i in 0..re.len() {
                let pos =
                    super::xy_screen_pos(response.plot_rect, (re[i], im[i]), ranges.0, ranges.1);
                let d2 = pos.distance_sq(pointer);
                if d2 < best {
                    best = d2;
                    hovered = Some((slot, i));
                }
            }
        }
    }

    if response.response.clicked() {
        let pins = &mut state.ui.results.rf_pin;
        match hovered {
            Some(hit) if pins.get(&super::ResultViewer::Smith) == Some(&hit) => {
                pins.remove(&super::ResultViewer::Smith);
            }
            Some(hit) => {
                pins.insert(super::ResultViewer::Smith, hit);
            }
            None => {
                pins.remove(&super::ResultViewer::Smith);
            }
        }
    }

    let pinned = state
        .ui
        .results
        .rf_pin
        .get(&super::ResultViewer::Smith)
        .copied()
        .filter(|(slot, i)| arrays.get(*slot).is_some_and(|(_, re, _)| *i < re.len()));
    let target = hovered.or(pinned);

    if let Some((slot, i)) = target {
        let (trace_index, re, im) = &arrays[slot];
        let gamma_re = re[i];
        let gamma_im = im[i];
        let pos =
            super::xy_screen_pos(response.plot_rect, (gamma_re, gamma_im), ranges.0, ranges.1);
        let color = c.traces[slot % c.traces.len()];
        let painter = plot_ui.painter();
        if pinned == Some((slot, i)) {
            painter.circle_stroke(pos, 6.0, egui::Stroke::new(1.8, color));
        }
        painter.circle_stroke(pos, 4.0, egui::Stroke::new(1.5, c.accent));

        let smith = &state.analysis.smith_chart_state;
        let trace = &smith.traces[*trace_index];
        let frequency = trace.points.get(i).map(|p| p.frequency).unwrap_or(0.0);
        let mag = (gamma_re * gamma_re + gamma_im * gamma_im).sqrt();
        let phase = gamma_im.atan2(gamma_re);
        let mut rows = vec![
            (
                "f".to_owned(),
                quantity_policy.format_frequency(frequency, 2),
            ),
            (
                "Γ".to_owned(),
                format!("{mag:.3} ∠ {}", quantity_policy.format_angle(phase, 1)),
            ),
        ];
        if let Some(reference_impedance_ohm) = trace.reference_impedance_ohm {
            rows.push(("Z₀".to_owned(), fmt_si(reference_impedance_ohm, "Ω", 2)));
            rows.push((
                "Z".to_owned(),
                if let Some((resistance, reactance)) =
                    impedance_from_gamma(gamma_re, gamma_im, reference_impedance_ohm)
                {
                    format!(
                        "{} {} j{}",
                        fmt_si(resistance, "Ω", 2),
                        if reactance >= 0.0 { "+" } else { "−" },
                        fmt_si(reactance.abs(), "Ω", 2)
                    )
                } else {
                    "open".to_owned()
                },
            ));
            rows.push((
                "VSWR".to_owned(),
                if mag < 1.0 {
                    format!("{:.2}", (1.0 + mag) / (1.0 - mag))
                } else {
                    "∞".to_owned()
                },
            ));
        } else {
            rows.push(("Readout".to_owned(), "complex coefficient only".to_owned()));
        }
        super::point_card(&plot_ui, response.plot_rect, pos, &trace.name, color, &rows);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Trace summary.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "S-parameters", None);
    let smith = &state.analysis.smith_chart_state;
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let visible_traces = smith
        .traces
        .iter()
        .filter(|tr| tr.visible && !tr.points.is_empty())
        .collect::<Vec<_>>();
    let Some(trace) = visible_traces.first().copied() else {
        super::panel_note(ui, "Trace metrics appear once S-parameter data is loaded.");
        return;
    };

    let first = trace.points.first();
    let last = trace.points.last();
    let maximum_gamma = trace
        .points
        .iter()
        .map(|point| point.s.norm())
        .filter(|value| value.is_finite())
        .max_by(f64::total_cmp);
    let maximum_vswr = trace
        .reference_impedance_ohm
        .and(maximum_gamma)
        .map(|gamma| {
            if gamma < 1.0 {
                format!("{:.2} : 1", (1.0 + gamma) / (1.0 - gamma))
            } else {
                "∞".to_owned()
            }
        });
    let marker = state
        .ui
        .results
        .rf_pin
        .get(&super::ResultViewer::Smith)
        .and_then(|(slot, point)| {
            let trace = visible_traces.get(*slot)?;
            let sample = trace.points.get(*point)?;
            let gamma = sample.s;
            let readout = if let Some(reference_impedance_ohm) = trace.reference_impedance_ohm {
                if let Some((resistance, reactance)) =
                    impedance_from_gamma(gamma.re, gamma.im, reference_impedance_ohm)
                {
                    format!(
                        "{} · {} {} j{}",
                        quantity_policy.format_frequency(sample.frequency, 2),
                        fmt_si(resistance, "Ω", 2),
                        if reactance >= 0.0 { "+" } else { "−" },
                        fmt_si(reactance.abs(), "Ω", 2)
                    )
                } else {
                    format!(
                        "{} · open",
                        quantity_policy.format_frequency(sample.frequency, 2)
                    )
                }
            } else {
                format!(
                    "{} · |S| {:.3}",
                    quantity_policy.format_frequency(sample.frequency, 2),
                    gamma.norm()
                )
            };
            Some(format!("{} · {readout}", trace.name))
        })
        .unwrap_or_else(|| "No marker pinned".to_owned());
    let rows = [
        ("Network", trace.name.clone(), true),
        ("Points", trace.points.len().to_string(), false),
        (
            "Sweep",
            match (first, last) {
                (Some(a), Some(b)) => format!(
                    "{} – {}",
                    quantity_policy.format_frequency(a.frequency, 1),
                    quantity_policy.format_frequency(b.frequency, 1)
                ),
                _ => "—".to_owned(),
            },
            false,
        ),
        (
            "Z₀",
            trace.reference_impedance_ohm.map_or_else(
                || "Not applicable to this locus".to_owned(),
                |reference| format!("{reference} Ω"),
            ),
            false,
        ),
        ("Marker", marker, false),
        (
            "|Γ| max",
            maximum_gamma.map_or_else(|| "Not retained".to_owned(), |value| format!("{value:.3}")),
            true,
        ),
        (
            "Max VSWR",
            maximum_vswr.unwrap_or_else(|| "Not retained".to_owned()),
            false,
        ),
        ("Reference plane", "Not retained".to_owned(), false),
    ];
    super::stat_table(ui, &rows);
    super::panel_note(
        ui,
        "Loci are plotted on the reflection-coefficient plane. De-embedding is not asserted without retained reference-plane provenance.",
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_identity_distinguishes_reflection_transmission_and_mixed_mode() {
        assert_eq!(
            trace_identity("S22"),
            Some(SParameterTraceIdentity {
                output_port: 2,
                input_port: 2,
                physical_ports: true,
            })
        );
        assert_eq!(
            trace_identity("S2_10[k=+0,m=+0]"),
            Some(SParameterTraceIdentity {
                output_port: 2,
                input_port: 10,
                physical_ports: true,
            })
        );
        assert_eq!(
            trace_identity("Sdd11"),
            Some(SParameterTraceIdentity {
                output_port: 1,
                input_port: 1,
                physical_ports: false,
            })
        );
    }

    #[test]
    fn nondefault_reference_impedance_controls_impedance_conversion() {
        let (resistance, reactance) =
            impedance_from_gamma(0.2, 0.0, 75.0).expect("finite impedance");
        assert!((resistance - 112.5).abs() < 1.0e-12);
        assert_eq!(reactance, 0.0);
    }

    #[test]
    fn wheel_zoom_keeps_smith_axes_at_equal_scale() {
        let change = super::super::square_xy_view_change(
            (-1.12, 1.12),
            (-1.12, 1.12),
            crate::ui::plot::ViewChange {
                x: Some((-0.5, 0.5)),
                ..Default::default()
            },
        );
        let x = change.x.expect("coupled x range");
        let y = change.y.expect("coupled y range");

        assert!(((x.1 - x.0) - (y.1 - y.0)).abs() < 1.0e-12);
        assert!((x.1 - x.0 - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn box_zoom_expands_shorter_axis_instead_of_distorting_chart() {
        let change = super::super::square_xy_view_change(
            (-1.12, 1.12),
            (-1.12, 1.12),
            crate::ui::plot::ViewChange {
                x: Some((-0.75, 0.75)),
                y: Some((-0.25, 0.25)),
                ..Default::default()
            },
        );
        let x = change.x.expect("square x range");
        let y = change.y.expect("square y range");

        assert!(((x.1 - x.0) - (y.1 - y.0)).abs() < 1.0e-12);
        assert!((y.1 - y.0 - 1.5).abs() < 1.0e-12);
    }

    #[test]
    fn fit_request_remains_a_reset() {
        let change = super::super::square_xy_view_change(
            (-0.5, 0.5),
            (-0.5, 0.5),
            crate::ui::plot::ViewChange {
                reset: true,
                ..Default::default()
            },
        );

        assert!(change.reset);
        assert!(change.x.is_none());
        assert!(change.y.is_none());
    }

    /// A retained S-parameter analysis whose S11 locus is `values`.
    fn sparam_analysis(
        id: u64,
        label: &str,
        values: &[(f64, f64)],
    ) -> crate::state::AnalysisResult {
        let x: Vec<f64> = (0..values.len())
            .map(|index| 1.0e9 * (index as f64 + 1.0))
            .collect();
        let waveform = crate::state::WaveformData::new(
            "S11",
            x.clone(),
            values.iter().map(|(re, _)| *re).collect::<Vec<_>>(),
            "#0af",
        )
        .with_complex_components(
            "S11",
            values.iter().map(|(re, _)| *re).collect::<Vec<_>>(),
            values.iter().map(|(_, im)| *im).collect::<Vec<_>>(),
        );
        crate::state::AnalysisResult::new(id, crate::state::AnalysisType::SParameter, label)
            .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm: vec![50.0, 50.0],
            })
            .with_waveforms(vec![waveform])
    }

    fn draw(state: &mut AppState) {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 700.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, state));
            },
        );
    }

    /// The Γ-component arrays are cached, and the cache belongs to the result
    /// that filled it. Selecting another analysis of the same run reloads the
    /// loci without moving the data version, so a cache keyed only by trace
    /// ordinal would draw the previous analysis' coefficients under the new
    /// one's name.
    #[test]
    fn the_gamma_arrays_belong_to_the_analysis_that_filled_them() {
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(sparam_analysis(1, "SP low", &[(0.1, 0.0), (0.2, 0.1)]));
        run.add_analysis(sparam_analysis(2, "SP high", &[(-0.6, 0.3), (-0.5, 0.4)]));
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(0));

        assert!(synchronize_active_analysis(&mut state));
        draw(&mut state);
        let first = state.analysis.smith_chart_state.traces[0].points[0].s.re;
        assert!((first - 0.1).abs() < 1.0e-12, "{first}");

        assert!(state.simulation.select_analysis(1));
        assert!(synchronize_active_analysis(&mut state));
        draw(&mut state);
        let second = state.analysis.smith_chart_state.traces[0].points[0].s.re;
        assert!((second + 0.6).abs() < 1.0e-12, "{second}");

        // The cached component array has to have moved with the selection.
        let owner = state
            .active_specialized_viewer_cache_provenance()
            .expect("the reloaded cache names its owner");
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        {
            use std::hash::Hash as _;
            owner.dataset_id.hash(&mut hasher);
            match owner.analysis_identity {
                SpecializedViewerAnalysisIdentity::Prepared(id) => {
                    id.hash(&mut hasher);
                }
                SpecializedViewerAnalysisIdentity::LegacyResultId(id) => id.hash(&mut hasher),
            }
        }
        let key = {
            use std::hash::Hasher as _;
            hasher.finish()
        };
        let cached = state
            .ui
            .results
            .derived
            .get_or(key ^ 0x501_0000, || std::sync::Arc::new(Vec::new()));
        assert_eq!(
            cached.first().copied(),
            Some(-0.6),
            "the sheet drew the previous analysis' coefficients"
        );
    }
}
