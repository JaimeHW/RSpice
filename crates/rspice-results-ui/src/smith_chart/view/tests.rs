use super::*;
use crate::smith_chart::circles::TwoPortFigures;
use crate::smith_chart::state::SmithCircleSet;
use rspice_results::network_matrix::two_port::{TwoPortNoiseParameters, TwoPortScattering};
use rspice_ui_kit::raster::{self, Canvas};
use std::sync::Arc;

/// An impedance reads at the chosen significant digits, not at two
/// decimals whatever the preference says.
#[test]
fn an_impedance_reads_at_the_chosen_significant_digits() {
    assert_eq!(
        impedance_readout(50.123_456, -12.345_678, 4),
        "50.12 Ω − j12.35 Ω"
    );
    assert_eq!(
        impedance_readout(50.123_456, 12.345_678, 6),
        "50.1235 Ω + j12.3457 Ω"
    );
}

#[test]
fn an_admittance_is_the_reciprocal_impedance() {
    let (conductance, susceptance) = admittance_from_gamma(0.0, 0.0, 50.0).unwrap();
    assert!((conductance - 0.02).abs() < 1e-15 && susceptance.abs() < 1e-15);
    let gamma = Complex64::new(0.3, -0.4);
    let (resistance, reactance) = impedance_from_gamma(gamma.re, gamma.im, 50.0).unwrap();
    let (conductance, susceptance) = admittance_from_gamma(gamma.re, gamma.im, 50.0).unwrap();
    let admittance = 1.0 / Complex64::new(resistance, reactance);
    assert!((admittance - Complex64::new(conductance, susceptance)).norm() < 1e-15);
    // Γ = −1 is a short: no finite admittance.
    assert!(admittance_from_gamma(-1.0, 0.0, 50.0).is_none());
}

/// The marker readout states what ADS's does: frequency, Γ, Z and/or Y as
/// the grid reads, and VSWR.
#[test]
fn the_marker_reads_frequency_gamma_impedance_admittance_and_vswr() {
    let trace = owned_trace("S11", &[(1.0e9, 0.2, 0.0)], Some(50.0));
    let policy = QuantityPresentationPolicy::default();
    let keys = |grid| {
        sample_rows(&trace.view(0), 0, grid, &policy, 4)
            .into_iter()
            .map(|(key, _)| key)
            .collect::<Vec<_>>()
    };
    assert_eq!(keys(SmithGrid::Impedance), ["f", "Γ", "Z₀", "Z", "VSWR"]);
    assert_eq!(keys(SmithGrid::Admittance), ["f", "Γ", "Z₀", "Y", "VSWR"]);
    assert_eq!(
        keys(SmithGrid::ImpedanceAdmittance),
        ["f", "Γ", "Z₀", "Z", "Y", "VSWR"]
    );
    let rows = sample_rows(
        &trace.view(0),
        0,
        SmithGrid::ImpedanceAdmittance,
        &policy,
        4,
    );
    assert_eq!(rows[3].1, "75 Ω + j0 Ω");
    assert_eq!(rows[4].1, "13.33 mS + j0 S");
    assert_eq!(rows[5].1, "1.5");
}

/// A trace's columns, owned for a test.
struct OwnedTrace {
    name: String,
    frequencies: SharedWaveformValues,
    real: SharedWaveformValues,
    imaginary: SharedWaveformValues,
    reference: Option<f64>,
}

impl OwnedTrace {
    fn view(&self, index: usize) -> SmithPlotTrace<'_> {
        SmithPlotTrace {
            index,
            name: &self.name,
            frequencies: Arc::clone(&self.frequencies),
            real: Arc::clone(&self.real),
            imaginary: Arc::clone(&self.imaginary),
            reference_impedance_ohm: self.reference,
            visible: true,
            removable: false,
            cache_key: plot::trace_cache_key(0x5EED, index),
        }
    }
}

fn owned_trace(name: &str, points: &[(f64, f64, f64)], reference: Option<f64>) -> OwnedTrace {
    OwnedTrace {
        name: name.to_owned(),
        frequencies: Arc::new(points.iter().map(|point| point.0).collect()),
        real: Arc::new(points.iter().map(|point| point.1).collect()),
        imaginary: Arc::new(points.iter().map(|point| point.2).collect()),
        reference,
    }
}

const SIZE: egui::Vec2 = egui::vec2(560.0, 560.0);

/// Render the chart alone; returns the canvas and the plot rectangle.
fn render_chart(
    traces: &[OwnedTrace],
    visible: bool,
    sheet: &SmithSheetState,
    figures: Option<&TwoPortFigures>,
    pin: Option<(usize, usize)>,
) -> (Canvas, egui::Rect) {
    let mut cache = plot::DecimationCache::default();
    let mut plot_rect = egui::Rect::NOTHING;
    let circles = figures
        .map(|figures| figures.circles(sheet.circles, &sheet.gain_db, &sheet.noise_db))
        .unwrap_or_default();
    let canvas = raster::render(SIZE, |ui, _| {
        let views = traces
            .iter()
            .enumerate()
            .map(|(index, trace)| SmithPlotTrace {
                visible,
                ..trace.view(index)
            })
            .collect::<Vec<_>>();
        let mut view = PlotView::default();
        let mut pin = pin;
        let chart = SmithChart {
            traces: &views,
            sheet,
            figures,
            circles: &circles,
        };
        if let Some(frame) = show(
            ui,
            &chart,
            &mut view,
            &mut pin,
            &mut cache,
            &QuantityPresentationPolicy::default(),
            4,
        ) {
            plot_rect = frame.plot.plot_rect;
        }
    });
    (canvas, plot_rect)
}

/// The device pixel a Γ lands on.
fn pixel_of(rect: egui::Rect, gamma: Complex64) -> (usize, usize) {
    let at = crate::presentation::xy_screen_pos(
        rect,
        (gamma.re, gamma.im),
        (-1.12, 1.12),
        (-1.12, 1.12),
    );
    (at.x.round() as usize, at.y.round() as usize)
}

fn distance(a: egui::Color32, b: egui::Color32) -> i32 {
    (i32::from(a.r()) - i32::from(b.r())).abs()
        + (i32::from(a.g()) - i32::from(b.g())).abs()
        + (i32::from(a.b()) - i32::from(b.b())).abs()
}

/// The pixel in a 5×5 window that stands furthest from the plot well.
fn strongest_ink(canvas: &Canvas, well: egui::Color32, at: (usize, usize)) -> egui::Color32 {
    let mut best = well;
    for dy in 0..5 {
        for dx in 0..5 {
            if let Some(pixel) = canvas.pixel(at.0 + dx - 2, at.1 + dy - 2)
                && distance(pixel, well) > distance(best, well)
            {
                best = pixel;
            }
        }
    }
    best
}

/// Points on one grid family's circles that are far from every circle of
/// the other family, from the chart's rim and from its real axis.
fn probe_points(admittance: bool) -> Vec<Complex64> {
    let (own, other): (Vec<grid::GridCircle>, Vec<grid::GridCircle>) = {
        let impedance = GRID_VALUES
            .iter()
            .map(|value| grid::resistance_circle(*value))
            .chain(
                GRID_VALUES
                    .iter()
                    .flat_map(|value| [*value, -*value])
                    .map(grid::reactance_circle),
            )
            .collect::<Vec<_>>();
        let reflected = impedance
            .iter()
            .map(|circle| grid::GridCircle {
                center: -circle.center,
                radius: circle.radius,
            })
            .collect::<Vec<_>>();
        if admittance {
            (reflected, impedance)
        } else {
            (impedance, reflected)
        }
    };
    let mut points = Vec::new();
    for circle in &own {
        for step in 0..48 {
            let point = circle.center
                + Complex64::from_polar(circle.radius, f64::from(step) * 7.5f64.to_radians());
            let clear_of_other = other
                .iter()
                .all(|grid| ((point - grid.center).norm() - grid.radius).abs() > 0.06);
            // Clear of the Cartesian chrome the plot engine rules at the
            // half-unit ticks as well.
            let clear_of_ticks = [-1.0, -0.5, 0.0, 0.5, 1.0]
                .iter()
                .all(|tick: &f64| (point.re - tick).abs() > 0.05 && (point.im - tick).abs() > 0.05);
            if point.norm() < 0.9 && clear_of_other && clear_of_ticks {
                points.push(point);
            }
        }
    }
    assert!(points.len() >= 4, "too few probe points: {}", points.len());
    points
}

/// The tokens a raster render paints with.
fn raster_tokens() -> Arc<Tokens> {
    let mut tokens = None;
    let _ = raster::render(egui::vec2(8.0, 8.0), |ui, _| {
        tokens = Some(Tokens::get(ui.ctx()));
    });
    tokens.expect("the pass ran")
}

fn hidden_trace() -> OwnedTrace {
    owned_trace("S11", &[(1.0e9, 0.0, 0.0), (2.0e9, 0.1, 0.0)], Some(50.0))
}

/// Each grid mode draws its own family and only its own family, and the
/// Z+Y chart draws the admittance family in its own ink.
#[test]
fn each_grid_mode_draws_exactly_its_circles() {
    let trace = [hidden_trace()];
    let impedance_points = probe_points(false);
    let admittance_points = probe_points(true);
    let mut renders = Vec::new();
    for mode in [
        SmithGrid::Impedance,
        SmithGrid::Admittance,
        SmithGrid::ImpedanceAdmittance,
    ] {
        let sheet = SmithSheetState {
            grid: mode,
            ..SmithSheetState::default()
        };
        renders.push((mode, render_chart(&trace, false, &sheet, None, None)));
    }
    let well = {
        let (canvas, rect) = &renders[0].1;
        // The well colour at the chart centre's upper-left, inside the
        // disc and off every grid line.
        let (x, y) = pixel_of(*rect, Complex64::new(-0.08, 0.92));
        canvas.pixel(x, y).unwrap()
    };
    let inked = |canvas: &Canvas, rect: egui::Rect, gamma: Complex64| {
        distance(strongest_ink(canvas, well, pixel_of(rect, gamma)), well) > 24
    };
    for (mode, (canvas, rect)) in &renders {
        for point in &impedance_points {
            assert_eq!(
                inked(canvas, *rect, *point),
                mode.draws_impedance(),
                "{mode:?}: impedance grid at {point}"
            );
        }
        for point in &admittance_points {
            assert_eq!(
                inked(canvas, *rect, *point),
                mode.draws_admittance(),
                "{mode:?}: admittance grid at {point}"
            );
        }
    }
    // On the Z+Y chart the two families do not share an ink.
    let (canvas, rect) = &renders[2].1;
    let z_ink = strongest_ink(canvas, well, pixel_of(*rect, impedance_points[0]));
    let y_ink = strongest_ink(canvas, well, pixel_of(*rect, admittance_points[0]));
    assert!(distance(z_ink, y_ink) > 30, "{z_ink:?} vs {y_ink:?}");
}

fn amplifier() -> TwoPortFigures {
    let polar = |m: f64, a: f64| Complex64::from_polar(m, a.to_radians());
    TwoPortFigures {
        frequency: 1.4e9,
        index: 0,
        scattering: TwoPortScattering {
            s11: polar(0.533, 176.6),
            s12: polar(0.06, 58.4),
            s21: polar(2.800, 64.5),
            s22: polar(0.604, -58.3),
        },
        reference_impedances_ohm: [50.0, 50.0],
        noise: Some(TwoPortNoiseParameters {
            minimum_noise_factor: 10f64.powf(0.16),
            noise_resistance_ohm: 20.0,
            optimum_source_reflection: polar(0.62, 100.0),
            reference_impedance_ohm: 50.0,
        }),
    }
}

/// A gain circle is drawn where its formula puts it, in the gain ink.
#[test]
fn a_gain_circle_is_drawn_where_its_formula_puts_it() {
    let trace = [hidden_trace()];
    let figures = amplifier();
    let sheet = SmithSheetState {
        circles: SmithCircleSet {
            power_gain: true,
            ..SmithCircleSet::default()
        },
        gain_db: vec![13.0],
        ..SmithSheetState::default()
    };
    let (canvas, rect) = render_chart(&trace, false, &sheet, Some(&figures), None);
    let circle = figures
        .scattering
        .power_gain_circle(10f64.powf(1.3))
        .unwrap();
    let gain_ink = raster_tokens().color.traces[4];
    let mut hits = 0;
    for step in 0..12 {
        let point = circle.center
            + Complex64::from_polar(circle.radius, f64::from(step) * 30f64.to_radians() + 0.1);
        if point.norm() > 0.95 {
            continue;
        }
        let drawn = strongest_ink(&canvas, canvas.background(), pixel_of(rect, point));
        if distance(drawn, gain_ink) < 120 {
            hits += 1;
        }
    }
    assert!(
        hits >= 8,
        "only {hits} of the circle's points carry its ink"
    );
}

/// The polar-only and sheet-free Smith views for review, both themes. Set
/// `RSPICE_REVIEW_DIR` to choose where.
#[test]
#[ignore = "writes PNGs for review"]
fn write_smith_grid_pngs() {
    let directory = std::env::var_os("RSPICE_REVIEW_DIR")
        .map_or_else(std::env::temp_dir, std::path::PathBuf::from);
    let trace = [hidden_trace()];
    for (mode, name) in [
        (SmithGrid::Impedance, "smith-grid-z.png"),
        (SmithGrid::Admittance, "smith-grid-y.png"),
        (SmithGrid::ImpedanceAdmittance, "smith-grid-zy.png"),
    ] {
        let sheet = SmithSheetState {
            grid: mode,
            ..SmithSheetState::default()
        };
        let (canvas, _) = render_chart(&trace, false, &sheet, None, None);
        let path = directory.join(name);
        std::fs::write(&path, canvas.png(canvas.height())).expect("write render");
        println!("{}", path.display());
    }
}

/// S-parameters at 50 Ω of a common-emitter hybrid-π bipolar transistor
/// (gm 0.1 S, rπ 1 kΩ, Cπ 3.2 pF, Cμ 0.3 pF, rb 20 Ω, ro 5 kΩ): the base
/// resistance in series with the intrinsic Y-parameters, converted through Z.
fn hybrid_pi(frequency: f64) -> TwoPortScattering {
    let omega = std::f64::consts::TAU * frequency;
    let j = Complex64::i();
    let (gm, r_pi, c_pi, c_mu, r_b, r_o) = (0.1, 1.0e3, 3.2e-12, 0.3e-12, 20.0, 5.0e3);
    let y11 = 1.0 / r_pi + j * omega * (c_pi + c_mu);
    let y12 = -j * omega * c_mu;
    let y21 = gm - j * omega * c_mu;
    let y22 = 1.0 / r_o + j * omega * c_mu;
    let det = y11 * y22 - y12 * y21;
    let (z11, z12, z21, z22) = (y22 / det + r_b, -y12 / det, -y21 / det, y11 / det);
    let z0 = 50.0;
    let delta = (z11 + z0) * (z22 + z0) - z12 * z21;
    TwoPortScattering {
        s11: ((z11 - z0) * (z22 + z0) - z12 * z21) / delta,
        s12: 2.0 * z0 * z12 / delta,
        s21: 2.0 * z0 * z21 / delta,
        s22: ((z11 + z0) * (z22 - z0) - z12 * z21) / delta,
    }
}

/// An amplifier on the Z+Y grid: the S11 sweep of [`hybrid_pi`] from
/// 100 MHz to 6 GHz with marker m1 pinned near 500 MHz, where the device is
/// potentially unstable, and both stability circles and two available-gain
/// circles at the marker frequency.
struct AmplifierScene {
    traces: [OwnedTrace; 1],
    figures: TwoPortFigures,
    sheet: SmithSheetState,
    circles: Vec<crate::smith_chart::circles::ChartCircle>,
    marker: usize,
}

fn amplifier_scene() -> AmplifierScene {
    use rspice_results::network_matrix::two_port::TwoPortSweep;

    let frequencies = (0..=80)
        .map(|step| 1.0e8 * 60f64.powf(f64::from(step) / 80.0))
        .collect::<Vec<_>>();
    let sweep = TwoPortSweep {
        scattering: frequencies.iter().map(|f| hybrid_pi(*f)).collect(),
        frequencies: frequencies.clone(),
        reference_impedances_ohm: [50.0, 50.0],
        noise: None,
    };
    let marker = sweep.nearest_index(5.0e8).expect("marker sample");
    let figures = TwoPortFigures::at(&sweep, marker).expect("figures");
    let points = frequencies
        .iter()
        .zip(&sweep.scattering)
        .map(|(f, s)| (*f, s.s11.re, s.s11.im))
        .collect::<Vec<_>>();
    let sheet = SmithSheetState {
        grid: SmithGrid::ImpedanceAdmittance,
        responses: vec!["S11".to_owned()],
        circles: SmithCircleSet {
            stability: true,
            available_gain: true,
            ..SmithCircleSet::default()
        },
        gain_db: vec![14.0, 17.0],
        ..SmithSheetState::default()
    };
    let circles = figures.circles(sheet.circles, &sheet.gain_db, &sheet.noise_db);
    AmplifierScene {
        traces: [owned_trace("S11", &points, Some(50.0))],
        figures,
        sheet,
        circles,
        marker,
    }
}

const PAGE: egui::Vec2 = egui::vec2(980.0, 790.0);

/// The sheet bar on its panel band, the chart in the well and the readout
/// panel at the right, as the results page lays them out, settled and
/// returned with the context that drew it.
fn amplifier_page(
    scene: &AmplifierScene,
    theme: rspice_ui_kit::theme::Theme,
    pixels_per_point: f32,
) -> (egui::Context, egui::FullOutput) {
    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    theme.apply(&ctx);
    let mut cache = plot::DecimationCache::default();
    let mut view = PlotView::default();
    let mut pin = Some((0, scene.marker));
    let mut output = None;
    for _ in 0..3 {
        output = Some(
            ctx.run_ui(raster::display_input(PAGE, pixels_per_point), |ui| {
                let c = Tokens::get(ui.ctx()).color;
                let views = scene
                    .traces
                    .iter()
                    .enumerate()
                    .map(|(index, trace)| trace.view(index))
                    .collect::<Vec<_>>();
                let chart = SmithChart {
                    traces: &views,
                    sheet: &scene.sheet,
                    figures: Some(&scene.figures),
                    circles: &scene.circles,
                };
                let bar = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(PAGE.x, 40.0));
                let panel = egui::Rect::from_min_max(
                    egui::pos2(PAGE.x - 300.0, bar.bottom()),
                    egui::pos2(PAGE.x, PAGE.y),
                );
                let well = egui::Rect::from_min_max(
                    egui::pos2(0.0, bar.bottom()),
                    egui::pos2(panel.left(), PAGE.y),
                );
                ui.painter().rect_filled(bar, 0.0, c.bg_panel);
                ui.painter().rect_filled(well, 0.0, c.canvas_bg);
                ui.painter().rect_filled(panel, 0.0, c.bg_panel);
                ui.painter().vline(
                    panel.left(),
                    panel.y_range(),
                    egui::Stroke::new(1.0, c.border),
                );
                let inset = |rect: egui::Rect| egui::UiBuilder::new().max_rect(rect.shrink(8.0));
                ui.scope_builder(inset(bar), |ui| {
                    ui.horizontal_centered(|ui| {
                        let mut bar_sheet = scene.sheet.clone();
                        let _ = domain_bar(
                            ui,
                            &mut bar_sheet,
                            SmithBarOffer {
                                responses: None,
                                two_port: true,
                                noise: false,
                            },
                        );
                    });
                });
                ui.scope_builder(egui::UiBuilder::new().max_rect(well), |ui| {
                    let _ = show(
                        ui,
                        &chart,
                        &mut view,
                        &mut pin,
                        &mut cache,
                        &QuantityPresentationPolicy::default(),
                        4,
                    );
                });
                ui.scope_builder(inset(panel), |ui| {
                    right_panel(ui, &chart, pin, &QuantityPresentationPolicy::default(), 4);
                });
            }),
        );
    }
    (ctx, output.expect("a pass ran"))
}

/// Every run of text on the page stands 4.5:1 off what is painted behind
/// it, in every interface mode over either canvas: the grid labels of both
/// families, the circle tags, the marker label, the bar and the readout.
#[test]
fn every_run_of_text_reads_off_its_ground_in_every_theme() {
    use rspice_ui_kit::theme::{EngineeringCanvasTheme, Theme};
    use rspice_ui_kit::tokens::Mode;

    let scene = amplifier_scene();
    let mut failures = Vec::new();
    for mode in Mode::ALL {
        for canvas_theme in EngineeringCanvasTheme::ALL {
            let theme = Theme {
                mode,
                canvas_theme,
                ..Theme::default()
            };
            let (ctx, output) = amplifier_page(&scene, theme, 1.0);
            let contrasts = raster::text_contrasts(&ctx, PAGE, &output);
            for wanted in ["j0.5", "0.5", "source stab", "GA 17 dB", "m1", "Marker m1"] {
                assert!(
                    contrasts.iter().any(|text| text.text == wanted),
                    "{mode:?}/{canvas_theme:?}: {wanted} is drawn"
                );
            }
            failures.extend(
                contrasts
                    .into_iter()
                    .filter(|text| text.ratio < 4.5)
                    .map(|text| {
                        format!(
                            "{mode:?}/{canvas_theme:?}: {:?} {:.2}:1 ({:?} on {:?})",
                            text.text, text.ratio, text.ink, text.ground
                        )
                    }),
            );
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The judged Smith picture: the page above at 2x in both themes. Set
/// `RSPICE_REVIEW_DIR` to choose where.
#[test]
#[ignore = "writes PNGs for review"]
fn write_smith_amplifier_png() {
    use rspice_ui_kit::theme::{EngineeringCanvasTheme, Theme};
    use rspice_ui_kit::tokens::Mode;

    let directory = std::env::var_os("RSPICE_REVIEW_DIR")
        .map_or_else(std::env::temp_dir, std::path::PathBuf::from);
    let scene = amplifier_scene();
    for (name, mode, canvas_theme) in [
        (
            "smith-amplifier-dark.png",
            Mode::Dark,
            EngineeringCanvasTheme::Dark,
        ),
        (
            "smith-amplifier-light.png",
            Mode::Light,
            EngineeringCanvasTheme::Light,
        ),
    ] {
        let theme = Theme {
            mode,
            canvas_theme,
            ..Theme::default()
        };
        let (ctx, output) = amplifier_page(&scene, theme, 2.0);
        let canvas = raster::rasterize(&ctx, PAGE, &output);
        let path = directory.join(name);
        std::fs::write(&path, canvas.png(canvas.height())).expect("write render");
        println!("{}", path.display());
    }
}
