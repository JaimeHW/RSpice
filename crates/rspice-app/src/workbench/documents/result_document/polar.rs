//! Polar source selection and shared cursor ownership at the application boundary.

use super::SheetContext;
use egui::Ui;
use rspice_results_ui::polar::{self as view, CanvasResponse, CursorSamples, PolarLocus};
pub(super) use view::quantities;

pub(super) fn domain_bar(ui: &mut Ui, context: &mut SheetContext<'_>) -> bool {
    view::domain_bar(
        ui,
        context
            .simulation
            .active_analysis()
            .map(|analysis| &**analysis),
        &mut context.results.polar,
    )
}

fn active_locus(context: &SheetContext<'_>) -> Option<PolarLocus> {
    view::resolve_locus(
        context.simulation.active_analysis()?,
        &context.results.polar,
    )
}

pub fn show(ui: &mut Ui, context: &mut SheetContext<'_>) {
    let locus = active_locus(context);
    if let Some(locus) = &locus {
        seed_cursors(context, locus);
    }
    let samples = locus
        .as_ref()
        .map(|locus| cursor_samples(context, locus))
        .unwrap_or_default();
    if let Some(response) = view::show(
        ui,
        locus.as_ref(),
        &context.results.polar,
        samples,
        &context.policy,
    ) {
        apply_canvas_input(
            ui,
            context,
            locus.as_ref().expect("a rendered locus"),
            &response,
        );
    }
}

pub fn right_panel(ui: &mut Ui, context: &mut SheetContext<'_>) {
    let locus = active_locus(context);
    let samples = locus
        .as_ref()
        .map(|locus| cursor_samples(context, locus))
        .unwrap_or_default();
    view::right_panel(ui, locus.as_ref(), samples, &context.policy);
}

fn seed_cursors(context: &mut SheetContext<'_>, locus: &PolarLocus) {
    if locus.frequencies().len() < 2 || context.results.cursors.a.is_some() {
        return;
    }
    let first = (0..locus.frequencies().len()).find(|index| locus.is_finite(*index));
    let last = (0..locus.frequencies().len())
        .rev()
        .find(|index| locus.is_finite(*index));
    let (Some(first), Some(last)) = (first, last) else {
        return;
    };
    context.results.cursor_strip = context.simulation.active_analysis_idx;
    context.results.cursors.a = Some(locus.frequencies()[first]);
    context.results.cursors.b = Some(locus.frequencies()[last]);
}

fn cursor_samples(context: &SheetContext<'_>, locus: &PolarLocus) -> CursorSamples {
    CursorSamples {
        a: context
            .results
            .cursors
            .a
            .and_then(|frequency| locus.nearest_to_frequency(frequency)),
        b: context
            .results
            .cursors
            .b
            .and_then(|frequency| locus.nearest_to_frequency(frequency)),
    }
}

fn apply_canvas_input(
    ui: &Ui,
    context: &mut SheetContext<'_>,
    locus: &PolarLocus,
    canvas: &CanvasResponse,
) {
    if canvas.response.clicked()
        && let Some(index) = canvas.hovered
    {
        snap_nearer_cursor(context, locus, index);
    }
    if !canvas.response.has_focus() {
        return;
    }
    let (shift, ctrl) = ui.input(|input| (input.modifiers.shift, input.modifiers.ctrl));
    let mut steps = 0_i64;
    ui.input(|input| {
        for event in &input.events {
            if let egui::Event::Key {
                key,
                pressed: true,
                repeat: _,
                ..
            } = event
            {
                match key {
                    egui::Key::ArrowLeft => steps -= 1,
                    egui::Key::ArrowRight => steps += 1,
                    egui::Key::Escape => {
                        steps = 0;
                    }
                    _ => {}
                }
            }
        }
    });
    let escape = ui.input(|input| input.key_pressed(egui::Key::Escape));
    if escape {
        context.results.clear_cursors();
        return;
    }
    if steps == 0 {
        return;
    }
    // Coarse is one percent of the retained sweep, fine is one sample: a
    // thousand-point sweep is otherwise a thousand key presses wide.
    let stride = if ctrl {
        1
    } else {
        (locus.frequencies().len() / 100).max(1)
    } as i64;
    nudge_cursor(context, locus, shift, steps * stride);
}

fn snap_nearer_cursor(context: &mut SheetContext<'_>, locus: &PolarLocus, index: usize) {
    let frequency = locus.frequencies()[index];
    let cursors = context.results.cursors;
    context.results.cursor_strip = context.simulation.active_analysis_idx;
    match (cursors.a, cursors.b) {
        (None, _) => context.results.cursors.a = Some(frequency),
        (Some(_), None) => context.results.cursors.b = Some(frequency),
        (Some(a), Some(b)) => {
            if (a - frequency).abs() <= (b - frequency).abs() {
                context.results.cursors.a = Some(frequency);
            } else {
                context.results.cursors.b = Some(frequency);
            }
        }
    }
}

fn nudge_cursor(context: &mut SheetContext<'_>, locus: &PolarLocus, cursor_b: bool, steps: i64) {
    let current = if cursor_b {
        context.results.cursors.b
    } else {
        context.results.cursors.a
    };
    let Some(index) = current.and_then(|frequency| locus.nearest_to_frequency(frequency)) else {
        return;
    };
    let last = locus.frequencies().len().saturating_sub(1) as i64;
    let moved = (index as i64 + steps).clamp(0, last) as usize;
    let frequency = locus.frequencies()[moved];
    context.results.cursor_strip = context.simulation.active_analysis_idx;
    if cursor_b {
        context.results.cursors.b = Some(frequency);
    } else {
        context.results.cursors.a = Some(frequency);
    }
}

#[cfg(test)]
mod tests {
    use super::super::ResultsState;
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, SimulationRun, WaveformData,
    };

    fn four_port_analysis() -> AnalysisResult {
        let frequency: Vec<f64> = (0..64)
            .map(|index| 1.0e6 * 10.0_f64.powf(index as f64 / 21.0))
            .collect();
        let mut waveforms = Vec::new();
        for (name, gain) in [
            ("Sdd21", 2.0),
            ("Sdd11", 0.3),
            ("Sdc21", 0.01),
            ("S11", 0.25),
            ("S21", 0.5),
        ] {
            let (real, imaginary): (Vec<f64>, Vec<f64>) = frequency
                .iter()
                .map(|f| {
                    let phase = -(f / 1.0e9) * std::f64::consts::PI;
                    let magnitude = gain / (1.0 + (f / 2.4e9).powi(2)).sqrt();
                    (magnitude * phase.cos(), magnitude * phase.sin())
                })
                .unzip();
            let magnitude = real
                .iter()
                .zip(imaginary.iter())
                .map(|(re, im)| re.hypot(*im))
                .collect::<Vec<_>>();
            waveforms.push(
                WaveformData::new(format!("|{name}|"), frequency.clone(), magnitude, "#00aaff")
                    .with_complex_components(name, real, imaginary),
            );
        }
        AnalysisResult::new(1, AnalysisType::SParameter, "SP")
            .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
                noise_reference_temperature_kelvin: None,
                reference_impedances_ohm: vec![50.0, 50.0, 50.0, 50.0],
            })
            .with_waveforms(waveforms)
    }

    fn fixture(analysis: AnalysisResult) -> (crate::state::SimulationState, ResultsState) {
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut simulation = crate::state::SimulationState::default();
        simulation.runs = vec![run].into();
        assert!(simulation.select_run(0));
        assert!(simulation.select_analysis(0));
        (simulation, ResultsState::default())
    }

    fn context<'a>(
        simulation: &'a crate::state::SimulationState,
        workspace: &'a crate::state::ProjectWorkspace,
        results: &'a mut ResultsState,
    ) -> SheetContext<'a> {
        SheetContext {
            simulation,
            workspace,
            results,
            policy: crate::quantity::QuantityPresentationPolicy::default(),
        }
    }

    #[test]
    fn a_click_moves_the_nearer_cursor_and_leaves_the_other_alone() {
        let (simulation, mut results) = fixture(four_port_analysis());
        let workspace = crate::state::ProjectWorkspace::default();
        let mut ctx = context(&simulation, &workspace, &mut results);
        let locus = active_locus(&ctx).expect("the fixture retains a locus");
        ctx.results.cursors.a = Some(locus.frequencies()[0]);
        ctx.results.cursors.b = Some(locus.frequencies()[locus.frequencies().len() - 1]);

        snap_nearer_cursor(&mut ctx, &locus, 2);
        assert_eq!(ctx.results.cursors.a, Some(locus.frequencies()[2]));
        assert_eq!(
            ctx.results.cursors.b,
            Some(locus.frequencies()[locus.frequencies().len() - 1]),
            "the far cursor moved"
        );

        snap_nearer_cursor(&mut ctx, &locus, locus.frequencies().len() - 3);
        assert_eq!(ctx.results.cursors.a, Some(locus.frequencies()[2]));
        assert_eq!(
            ctx.results.cursors.b,
            Some(locus.frequencies()[locus.frequencies().len() - 3])
        );
    }

    #[test]
    fn seeded_cursors_are_the_shared_pair_bound_to_the_active_analysis() {
        let (simulation, mut results) = fixture(four_port_analysis());
        let workspace = crate::state::ProjectWorkspace::default();
        let mut ctx = context(&simulation, &workspace, &mut results);
        let locus = active_locus(&ctx).expect("the fixture retains a locus");
        assert_eq!(ctx.results.cursors.a, None);

        seed_cursors(&mut ctx, &locus);
        assert_eq!(ctx.results.cursors.a, Some(locus.frequencies()[0]));
        assert_eq!(
            ctx.results.cursors.b,
            Some(locus.frequencies()[locus.frequencies().len() - 1])
        );
        assert_eq!(ctx.results.cursor_strip, Some(0));

        // Seeding is once: a reader who moved A keeps it.
        ctx.results.cursors.a = Some(locus.frequencies()[3]);
        seed_cursors(&mut ctx, &locus);
        assert_eq!(ctx.results.cursors.a, Some(locus.frequencies()[3]));
    }
}
