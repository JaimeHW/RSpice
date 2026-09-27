//! Complete optimizer history CSV, revalidated for each export.
use crate::table::escape_csv_field as csv_field;
use rspice_results::{
    analysis_result::AnalysisResult, optimization::history::OptimizationIndices,
    waveform::RetainedWaveform,
};

/// Encoded history and the iteration count used by its publisher.
#[derive(Debug)]
pub struct EncodedOptimizationCsv {
    pub contents: String,
    pub iteration_count: usize,
}

/// Validate and encode the exact retained optimum, objectives and candidate history.
pub fn encode_optimization_csv<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
) -> Option<EncodedOptimizationCsv> {
    let view =
        OptimizationIndices::locate(analysis, analysis.validate_retained_evidence().is_ok())?
            .view(analysis)?;
    let mut contents = String::from("field,value\n");
    contents.push_str(&format!("converged,{}\n", view.converged));
    contents.push_str(&format!("best_cost,{:.17e}\n", view.best_cost));
    contents.push_str(&format!("best_index,{}\n", view.best_index));
    if !view.best_constraints.is_empty() {
        contents.push_str(&format!(
            "feasible,{}\n",
            view.best_constraints.iter().all(|row| row.violation == 0.0)
        ));
        contents.push_str("\nconstraint,measurement,unit,lower,upper,tolerance,scale,value,normalized_violation,satisfied\n");
        for (index, observation) in view.best_constraints.iter().enumerate() {
            let term = &observation.constraint;
            contents.push_str(&format!(
                "{},{},{},{},{},{:.17e},{:.17e},{:.17e},{:.17e},{}\n",
                index + 1,
                csv_field(&term.measurement),
                csv_field(&term.unit),
                term.lower.map(|v| format!("{v:.17e}")).unwrap_or_default(),
                term.upper.map(|v| format!("{v:.17e}")).unwrap_or_default(),
                term.tolerance,
                term.scale,
                observation.value,
                observation.violation,
                observation.violation == 0.0
            ));
        }
        contents.push_str("\nfield,value\n");
    }
    if !view.best_objectives.is_empty() {
        contents.push_str(
            "\nobjective,measurement,unit,goal,target,scale,weight,value,cost_contribution\n",
        );
        for (index, observation) in view.best_objectives.iter().enumerate() {
            let term = &observation.objective;
            contents.push_str(&format!(
                "{},{},{},{:?},{},{:.17e},{:.17e},{:.17e},{:.17e}\n",
                index + 1,
                csv_field(&term.measurement),
                csv_field(&term.unit),
                term.goal,
                term.target
                    .map(|value| format!("{value:.17e}"))
                    .unwrap_or_default(),
                term.scale,
                term.weight,
                observation.value,
                observation.contribution
            ));
        }
        contents.push_str("\nfield,value\n");
    }
    contents.push_str(&format!(
        "best_iteration,{:.17e}\n\niteration,cost",
        view.iterations[view.best_index]
    ));
    for (name, _) in &view.variables {
        contents.push(',');
        contents.push_str(&csv_field(name));
    }
    contents.push('\n');
    for index in 0..view.iterations.len() {
        contents.push_str(&format!(
            "{:.17e},{:.17e}",
            view.iterations[index], view.cost.y[index]
        ));
        for (_, waveform) in &view.variables {
            contents.push_str(&format!(",{:.17e}", waveform.y[index]));
        }
        contents.push('\n');
    }
    Some(EncodedOptimizationCsv {
        contents,
        iteration_count: view.iterations.len(),
    })
}
