//! Scatter source resolution and displayed-axis ownership at the application boundary.

use super::{SheetContext, population};
use egui::Ui;
use rspice_results_ui::scatter as view;

pub(super) fn domain_bar(ui: &mut Ui, context: &mut SheetContext<'_>) -> bool {
    let plan = population::plan(context);
    view::domain_bar(
        ui,
        plan.as_deref().map(|plan| &**plan),
        &mut context.results.session.scatter,
        &mut context.results.session.show_spec_limits,
    )
}

pub fn show(ui: &mut Ui, context: &mut SheetContext<'_>) {
    let plan = population::plan(context);
    if let Some(response) = view::show(
        ui,
        plan.as_deref().map(|plan| &**plan),
        &mut context.results.session.scatter,
        context.results.session.show_spec_limits,
        &mut context.results.session.cache,
    ) {
        super::record_drawn_axes(context.results, super::ResultViewer::Scatter, &response);
    }
}

pub fn right_panel(ui: &mut Ui, context: &mut SheetContext<'_>) {
    let plan = population::plan(context);
    view::right_panel(
        ui,
        plan.as_deref().map(|plan| &**plan),
        &mut context.results.session.scatter,
    );
}

#[cfg(test)]
mod tests {
    use super::super::ResultsState;
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, FamilyMeasurementEvidence,
        FamilyMemberId, FamilyMemberMeasurements, MonteCarloVariableMetadata, SimulationRun,
        SpecEntry, SpecPointScope,
    };
    fn population_analysis(trials: usize) -> AnalysisResult {
        let samples: Vec<f64> = (0..trials)
            .map(|index| (index as f64 - trials as f64 / 2.0) / (trials as f64 / 2.0))
            .collect();
        let members = samples
            .iter()
            .enumerate()
            .map(|(index, sample)| {
                FamilyMemberMeasurements::new(
                    FamilyMemberId::MonteCarloTrial {
                        index,
                        seed: 0x73a4 + index as u64,
                    },
                    vec![FamilyMeasurementEvidence {
                        unit: None,
                        name: "gain_dc".to_owned(),
                        value: Some(40.0 + 2.0 * sample),
                        passed: true,
                        error: None,
                    }],
                )
            })
            .collect::<Vec<_>>();
        let mean = samples.iter().sum::<f64>() / trials as f64;
        let variance =
            samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (trials - 1) as f64;
        AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                seed: 0x73a4,
                runs_requested: trials,
                runs_completed: trials,
                failures: 0,
                all_converged: true,
                variables: vec![MonteCarloVariableMetadata {
                    mean_confidence: None,
                    name: "XBRIDGE.dR".to_owned(),
                    mean,
                    std_dev: variance.sqrt(),
                    min: samples.iter().copied().fold(f64::INFINITY, f64::min),
                    max: samples.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                    samples,
                }],
                member_measurements: members,
            },
        )
    }

    fn fixture(
        trials: usize,
        limit: Option<f64>,
    ) -> (
        crate::state::SimulationState,
        crate::state::ProjectWorkspace,
        ResultsState,
    ) {
        let mut run = SimulationRun::new(1);
        run.add_analysis(population_analysis(trials));
        let mut simulation = crate::state::SimulationState::default();
        simulation.retained.runs = vec![run].into();
        assert!(simulation.select_run(0));
        assert!(simulation.select_analysis(0));
        let mut workspace = crate::state::ProjectWorkspace::default();
        if let Some(min) = limit {
            workspace.content.specs.push(SpecEntry {
                measurement: "gain_dc".to_owned(),
                expression: String::new(),
                min: Some(min),
                max: None,
                unit: "dB".to_owned(),
                scope: SpecPointScope::AllPoints,
            });
        }
        (simulation, workspace, ResultsState::default())
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

    fn rendered_text(
        context: &mut SheetContext<'_>,
        render: fn(&mut Ui, &mut SheetContext<'_>),
    ) -> Vec<String> {
        let egui = egui::Context::default();
        crate::ui::Theme::default().apply(&egui);
        let output = egui.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(720.0, 1200.0),
                )),
                ..Default::default()
            },
            |root| {
                egui::CentralPanel::default().show(root, |ui| render(ui, context));
            },
        );
        fn collect(shape: egui::Shape, text: &mut Vec<String>) {
            match shape {
                egui::Shape::Text(shape) => text.push(shape.galley.text().to_owned()),
                egui::Shape::Vec(shapes) => {
                    for shape in shapes {
                        collect(shape, text);
                    }
                }
                _ => {}
            }
        }
        let mut text = Vec::new();
        for shape in output.shapes {
            collect(shape.shape, &mut text);
        }
        text
    }

    #[test]
    fn population_cache_refreshes_the_scatter_register_after_a_requirement_rename() {
        let (simulation, mut workspace, mut results) = fixture(101, Some(39.5));
        let original =
            population::plan(&mut context(&simulation, &workspace, &mut results)).unwrap();
        assert_eq!(original.failing_count(), 38);
        workspace.content.specs[0].measurement = "gain_ac".to_owned();
        let rows = rendered_text(
            &mut context(&simulation, &workspace, &mut results),
            right_panel,
        );
        assert!(
            rows.windows(2)
                .any(|row| row[0] == "Trials" && row[1] == "101 measured \u{b7} 0 failing"),
            "{rows:?}"
        );
        assert!(
            rows.windows(2).any(|row| row[0] == "Y requirement"
                && row[1] == "No requirement bounds this measurement"),
            "{rows:?}"
        );
    }

    #[test]
    fn the_absent_state_names_the_analysis_that_would_fill_it() {
        let simulation = crate::state::SimulationState::default();
        let workspace = crate::state::ProjectWorkspace::default();
        let mut results = ResultsState::default();
        let mut ctx = context(&simulation, &workspace, &mut results);
        assert!(population::plan(&mut ctx).is_none());
        let text = rendered_text(&mut ctx, show);
        assert!(
            text.iter()
                .any(|text| text
                    == "No Monte Carlo population in this run — run a Monte Carlo analysis"),
            "{text:?}"
        );
    }
}
