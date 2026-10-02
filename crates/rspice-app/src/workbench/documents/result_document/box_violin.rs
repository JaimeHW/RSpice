//! Distribution source resolution and displayed-axis ownership at the application boundary.

use super::{SheetContext, population};
use egui::Ui;
use rspice_results_ui::box_violin as view;

pub(super) fn domain_bar(ui: &mut Ui, context: &mut SheetContext<'_>) -> bool {
    let plan = population::plan(context);
    view::domain_bar(
        ui,
        plan.as_deref().map(|plan| &**plan),
        &mut context.results.box_violin,
        &mut context.results.show_spec_limits,
    )
}

pub fn show(ui: &mut Ui, context: &mut SheetContext<'_>) {
    let plan = population::plan(context);
    if let Some(response) = view::show(
        ui,
        plan.as_deref().map(|plan| &**plan),
        &context.results.box_violin,
        context.results.show_spec_limits,
        &mut context.results.cache,
    ) {
        super::record_drawn_axes(context.results, super::ResultViewer::BoxViolin, &response);
    }
}

pub fn right_panel(ui: &mut Ui, context: &mut SheetContext<'_>) {
    let plan = population::plan(context);
    view::right_panel(
        ui,
        plan.as_deref().map(|plan| &**plan),
        &mut context.results.box_violin,
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
        let members = (0..trials)
            .map(|index| {
                let position = (index as f64 - trials as f64 / 2.0) / (trials as f64 / 2.0);
                FamilyMemberMeasurements::new(
                    FamilyMemberId::MonteCarloTrial {
                        index,
                        seed: 0x73a4 + index as u64,
                    },
                    vec![
                        FamilyMeasurementEvidence {
                            unit: None,
                            name: "gain_dc".to_owned(),
                            value: Some(40.0 + 2.0 * position),
                            passed: true,
                            error: None,
                        },
                        FamilyMeasurementEvidence {
                            unit: None,
                            name: "vos".to_owned(),
                            value: Some(60.0 * position),
                            passed: true,
                            error: None,
                        },
                    ],
                )
            })
            .collect::<Vec<_>>();
        let samples: Vec<f64> = (0..trials).map(|index| index as f64).collect();
        AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
            AnalysisResultFamilyMetadata::MonteCarlo {
                seed: 0x73a4,
                runs_requested: trials,
                runs_completed: trials,
                failures: 0,
                all_converged: true,
                variables: vec![MonteCarloVariableMetadata {
                    mean_confidence: None,
                    name: "RGAIN.r".to_owned(),
                    mean: (trials as f64 - 1.0) / 2.0,
                    std_dev: rspice_results::population::std_dev(&samples).expect("a spread"),
                    min: 0.0,
                    max: trials as f64 - 1.0,
                    samples,
                }],
                member_measurements: members,
            },
        )
    }
    fn fixture(
        trials: usize,
    ) -> (
        crate::state::SimulationState,
        crate::state::ProjectWorkspace,
        ResultsState,
    ) {
        let mut run = SimulationRun::new(1);
        run.add_analysis(population_analysis(trials));
        let mut simulation = crate::state::SimulationState::default();
        simulation.runs = vec![run].into();
        assert!(simulation.select_run(0));
        assert!(simulation.select_analysis(0));
        let mut workspace = crate::state::ProjectWorkspace::default();
        workspace.content.specs.push(SpecEntry {
            measurement: "gain_dc".to_owned(),
            expression: String::new(),
            min: Some(39.5),
            max: None,
            unit: "dB".to_owned(),
            scope: SpecPointScope::AllPoints,
        });
        workspace.content.specs.push(SpecEntry {
            measurement: "vos".to_owned(),
            expression: String::new(),
            min: Some(-50.0),
            max: Some(50.0),
            unit: "V".to_owned(),
            scope: SpecPointScope::AllPoints,
        });
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
    fn rendered_columns(context: &mut SheetContext<'_>) -> Vec<String> {
        let egui = egui::Context::default();
        crate::ui::Theme::default().apply(&egui);
        egui.enable_accesskit();
        let output = egui.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(720.0, 1200.0),
                )),
                ..Default::default()
            },
            |root| {
                egui::CentralPanel::default().show(root, |ui| right_panel(ui, context));
            },
        );
        let mut columns = output
            .platform_output
            .accesskit_update
            .expect("the distribution inspector publishes its columns")
            .nodes
            .into_iter()
            .filter(|(_, node)| node.role() != egui::accesskit::Role::TextRun)
            .filter_map(|(_, node)| {
                let name = node
                    .label()
                    .or_else(|| node.value())
                    .and_then(|text| text.split_once("  "))?
                    .0;
                Some((node.bounds()?.y0, name.to_owned()))
            })
            .collect::<Vec<_>>();
        // Accessibility updates are keyed by node identity, not visual row order.
        columns.sort_by(|left, right| left.0.total_cmp(&right.0));
        columns.into_iter().map(|(_, name)| name).collect()
    }

    #[test]
    fn population_cache_refreshes_the_normalized_columns_after_a_requirement_rename() {
        let (simulation, mut workspace, mut results) = fixture(101);
        let columns = rendered_columns(&mut context(&simulation, &workspace, &mut results));
        assert_eq!(columns, ["gain_dc", "vos"]);
        workspace.content.specs[0].measurement = "gain_ac".to_owned();
        let columns = rendered_columns(&mut context(&simulation, &workspace, &mut results));
        assert_eq!(columns, ["vos"]);
    }
}
