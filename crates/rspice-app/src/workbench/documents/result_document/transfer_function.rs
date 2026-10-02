//! XF source selection, validation and retained dataset authority.

use crate::state::{AnalysisResultPayload, SimulationRun};
#[cfg(test)]
use crate::state::{
    TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
    TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
};
use crate::workbench::AppState;
use egui::Ui;
#[cfg(test)]
use rspice_results_ui::transfer_function::gain_unit;
use rspice_results_ui::transfer_function::{self as view, TransferFunctionView};

fn active_transfer_function(state: &AppState) -> Option<TransferFunctionView<'_>> {
    let run = state.simulation.active_run()?;
    let analysis = state.simulation.active_analysis()?;
    let payload = analysis.result_payload.as_ref()?;
    let AnalysisResultPayload::TransferFunction {
        input_source,
        output_expression,
        input_quantity,
        output_quantity,
        input_unit,
        output_unit,
        normalization,
        accuracy,
        gain,
        input_resistance,
        output_resistance,
        nominal_input,
        nominal_output,
    } = payload
    else {
        return None;
    };
    if !analysis.success || payload.validate_for(analysis.analysis_type).is_err() {
        return None;
    }
    Some(TransferFunctionView {
        analysis_label: &analysis.label,
        input_source,
        output_expression,
        input_quantity: *input_quantity,
        output_quantity: *output_quantity,
        input_unit,
        output_unit,
        normalization: *normalization,
        accuracy: *accuracy,
        gain: *gain,
        input_resistance: *input_resistance,
        output_resistance: *output_resistance,
        nominal_input: *nominal_input,
        nominal_output: *nominal_output,
        dataset_id: run.dataset_id,
        dataset_authority: dataset_authority_label(run),
    })
}

fn dataset_authority_label(run: &SimulationRun) -> &'static str {
    match (run.prepared_receipt(), run.validate_provenance()) {
        (Some(_), Ok(())) => "prepared receipt matched",
        (Some(_), Err(_)) => "prepared receipt mismatch",
        (None, _) => "prepared receipt unavailable",
    }
}

pub(super) fn active_payload_is_valid(state: &AppState) -> bool {
    active_transfer_function(state).is_some()
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    view::show(ui, active_transfer_function(state));
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    view::right_panel(ui, active_transfer_function(state));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};

    fn tf_result(
        id: u64,
        label: &str,
        input_quantity: TransferFunctionQuantityEvidence,
        output_quantity: TransferFunctionQuantityEvidence,
        normalization: TransferFunctionNormalizationEvidence,
        gain: f64,
    ) -> AnalysisResult {
        let input_source = match input_quantity {
            TransferFunctionQuantityEvidence::Voltage => "VIN",
            TransferFunctionQuantityEvidence::Current => "IIN",
        };
        let output_expression = match output_quantity {
            TransferFunctionQuantityEvidence::Voltage => "V(out)",
            TransferFunctionQuantityEvidence::Current => "I(VMEAS)",
        };
        let input_unit = match input_quantity {
            TransferFunctionQuantityEvidence::Voltage => "V",
            TransferFunctionQuantityEvidence::Current => "A",
        };
        let output_unit = match output_quantity {
            TransferFunctionQuantityEvidence::Voltage => "V",
            TransferFunctionQuantityEvidence::Current => "A",
        };
        let (nominal_input, nominal_output) =
            if normalization == TransferFunctionNormalizationEvidence::RelativeToNominal {
                (Some(2.0), Some(0.5))
            } else {
                (None, None)
            };

        AnalysisResult::new(id, AnalysisType::Tf, label).with_result_payload(
            AnalysisResultPayload::TransferFunction {
                input_source: input_source.to_owned(),
                output_expression: output_expression.to_owned(),
                input_quantity,
                output_quantity,
                input_unit: input_unit.to_owned(),
                output_unit: output_unit.to_owned(),
                normalization,
                accuracy: TransferFunctionAccuracyEvidence::Balanced,
                gain: Some(TransferFunctionScalarEvidence::Finite(gain)),
                input_resistance: Some(TransferFunctionScalarEvidence::PositiveInfinity),
                output_resistance: Some(TransferFunctionScalarEvidence::Finite(125.0)),
                nominal_input,
                nominal_output,
            },
        )
    }

    fn state_with_analyses(analyses: Vec<AnalysisResult>) -> AppState {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        for analysis in analyses {
            run.add_analysis(analysis);
        }
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        state
    }

    #[test]
    fn viewer_reads_only_the_active_retained_tf_payload() {
        let tf = tf_result(
            2,
            "XF active",
            TransferFunctionQuantityEvidence::Voltage,
            TransferFunctionQuantityEvidence::Current,
            TransferFunctionNormalizationEvidence::None,
            -0.25,
        );
        let mut state = state_with_analyses(vec![
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN selected"),
            tf,
        ]);

        assert!(active_transfer_function(&state).is_none());
        assert!(!active_payload_is_valid(&state));
        assert!(state.simulation.select_analysis(1));

        let view = active_transfer_function(&state).expect("selected TF payload is active");
        assert_eq!(view.analysis_label, "XF active");
        assert_eq!(view.input_source, "VIN");
        assert_eq!(view.output_expression, "I(VMEAS)");
        assert_eq!(
            view.gain,
            Some(TransferFunctionScalarEvidence::Finite(-0.25))
        );
        assert!(active_payload_is_valid(&state));
    }

    #[test]
    fn gain_units_are_isolated_to_the_selected_payload() {
        let mut state = state_with_analyses(vec![
            tf_result(
                1,
                "A per V",
                TransferFunctionQuantityEvidence::Voltage,
                TransferFunctionQuantityEvidence::Current,
                TransferFunctionNormalizationEvidence::None,
                1.0,
            ),
            tf_result(
                2,
                "V per A",
                TransferFunctionQuantityEvidence::Current,
                TransferFunctionQuantityEvidence::Voltage,
                TransferFunctionNormalizationEvidence::PerSourceUnit,
                2.0,
            ),
            tf_result(
                3,
                "Relative",
                TransferFunctionQuantityEvidence::Current,
                TransferFunctionQuantityEvidence::Current,
                TransferFunctionNormalizationEvidence::RelativeToNominal,
                3.0,
            ),
        ]);

        let first = active_transfer_function(&state).expect("first TF");
        assert_eq!(first.analysis_label, "A per V");
        assert_eq!(gain_unit(&first), "A/V");

        assert!(state.simulation.select_analysis(1));
        let second = active_transfer_function(&state).expect("second TF");
        assert_eq!(second.analysis_label, "V per A");
        assert_eq!(gain_unit(&second), "V/A");

        assert!(state.simulation.select_analysis(2));
        let relative = active_transfer_function(&state).expect("relative TF");
        assert_eq!(relative.analysis_label, "Relative");
        assert_eq!(gain_unit(&relative), "1");
    }

    #[test]
    fn viewer_rejects_failed_or_mismatched_payloads_fail_closed() {
        let mut failed = tf_result(
            1,
            "failed XF",
            TransferFunctionQuantityEvidence::Voltage,
            TransferFunctionQuantityEvidence::Voltage,
            TransferFunctionNormalizationEvidence::None,
            1.0,
        );
        failed.success = false;
        assert!(active_transfer_function(&state_with_analyses(vec![failed])).is_none());

        let payload = tf_result(
            2,
            "mismatched",
            TransferFunctionQuantityEvidence::Voltage,
            TransferFunctionQuantityEvidence::Voltage,
            TransferFunctionNormalizationEvidence::None,
            1.0,
        )
        .data
        .result_payload;
        let mut mismatched = AnalysisResult::new(2, AnalysisType::Ac, "AC");
        mismatched.result_payload = payload;
        assert!(active_transfer_function(&state_with_analyses(vec![mismatched])).is_none());
    }

    #[test]
    fn unsealed_dataset_does_not_claim_authenticated_authority() {
        let state = state_with_analyses(vec![tf_result(
            1,
            "XF",
            TransferFunctionQuantityEvidence::Voltage,
            TransferFunctionQuantityEvidence::Voltage,
            TransferFunctionNormalizationEvidence::None,
            1.0,
        )]);

        let view = active_transfer_function(&state).expect("active XF view");

        assert_eq!(view.dataset_authority, "prepared receipt unavailable");
        assert_eq!(
            view.dataset_id,
            state.simulation.active_run().unwrap().dataset_id
        );
    }
}
