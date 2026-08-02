//! XF - retained scalar DC transfer-function evidence.

use egui::Ui;

use crate::product::DatasetId;
use crate::state::{
    AnalysisResultPayload, SimulationRun, TransferFunctionAccuracyEvidence,
    TransferFunctionNormalizationEvidence, TransferFunctionQuantityEvidence,
    TransferFunctionScalarEvidence,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};
use crate::workbench::AppState;

use super::strip::StripHeader;
use super::{panel_note, well_hint};

struct TransferFunctionView<'a> {
    analysis_label: &'a str,
    input_source: &'a str,
    output_expression: &'a str,
    input_quantity: TransferFunctionQuantityEvidence,
    output_quantity: TransferFunctionQuantityEvidence,
    input_unit: &'a str,
    output_unit: &'a str,
    normalization: TransferFunctionNormalizationEvidence,
    accuracy: TransferFunctionAccuracyEvidence,
    gain: Option<TransferFunctionScalarEvidence>,
    input_resistance: Option<TransferFunctionScalarEvidence>,
    output_resistance: Option<TransferFunctionScalarEvidence>,
    nominal_input: Option<f64>,
    nominal_output: Option<f64>,
    dataset_id: DatasetId,
    dataset_authority: &'static str,
}

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
    let Some(view) = active_transfer_function(state) else {
        well_hint(
            ui,
            "Select an analysis with a retained transfer-function result",
        );
        return;
    };

    StripHeader::new(
        "XF",
        &format!(
            "{} -> {} - DC operating point",
            view.input_source, view.output_expression
        ),
        &[],
    )
    .show(ui);

    let width = ui.available_width();
    egui::ScrollArea::vertical()
        .id_salt("rspice.results.transfer-function")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_min_width(width);
            ui.add_space(14.0);
            metric_cards(ui, &view);

            ui.add_space(14.0);
            contract_cards(ui, &view);
        });
}

fn metric_cards(ui: &mut Ui, view: &TransferFunctionView<'_>) {
    let available = ui.available_width().max(1.0);
    if available >= 590.0 {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            let card_width = (available - 20.0) / 3.0;
            metric_card(
                ui,
                card_width,
                "TRANSFER GAIN",
                view.gain,
                gain_unit(view),
                "exact retained scalar",
            );
            metric_card(
                ui,
                card_width,
                "INPUT RESISTANCE",
                view.input_resistance,
                "ohm",
                "small-signal input resistance",
            );
            metric_card(
                ui,
                card_width,
                "OUTPUT RESISTANCE",
                view.output_resistance,
                "ohm",
                "small-signal output resistance",
            );
        });
    } else {
        metric_card(
            ui,
            available,
            "TRANSFER GAIN",
            view.gain,
            gain_unit(view),
            "exact retained scalar",
        );
        ui.add_space(8.0);
        metric_card(
            ui,
            available,
            "INPUT RESISTANCE",
            view.input_resistance,
            "ohm",
            "small-signal input resistance",
        );
        ui.add_space(8.0);
        metric_card(
            ui,
            available,
            "OUTPUT RESISTANCE",
            view.output_resistance,
            "ohm",
            "small-signal output resistance",
        );
    }
}

fn metric_card(
    ui: &mut Ui,
    width: f32,
    title: &str,
    value: Option<TransferFunctionScalarEvidence>,
    unit: &str,
    detail: &str,
) {
    let t = Tokens::get(ui.ctx());
    ui.allocate_ui_with_layout(
        egui::vec2(width.max(1.0), 104.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            egui::Frame::new()
                .fill(t.color.bg_panel)
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .inner_margin(egui::Margin::symmetric(12, 11))
                .show(ui, |ui| {
                    ui.set_min_size(egui::vec2((width - 26.0).max(1.0), 80.0));
                    ui.label(
                        egui::RichText::new(title)
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                    ui.add_space(7.0);
                    ui.label(
                        egui::RichText::new(display_scalar(value, unit))
                            .font(theme::mono(tokens::FS_3, FontWeight::Medium))
                            .color(if value.is_some() {
                                t.color.text
                            } else {
                                t.color.text_faint
                            }),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(if value.is_some() {
                            detail
                        } else {
                            "scalar not retained"
                        })
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                    );
                });
        },
    );
}

fn contract_cards(ui: &mut Ui, view: &TransferFunctionView<'_>) {
    let available = ui.available_width().max(1.0);
    if available >= 610.0 {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 10.0;
            let width = (available - 10.0) / 2.0;
            transfer_definition_card(ui, width, view);
            nominal_evidence_card(ui, width, view);
        });
    } else {
        transfer_definition_card(ui, available, view);
        ui.add_space(10.0);
        nominal_evidence_card(ui, available, view);
    }
}

fn transfer_definition_card(ui: &mut Ui, width: f32, view: &TransferFunctionView<'_>) {
    contract_card(ui, width, "Transfer definition", "retained", |ui| {
        measurement_table(
            ui,
            &[
                ("Input source", view.input_source),
                ("Output expression", view.output_expression),
                ("Solve point", "DC operating point"),
                ("Normalization", normalization_label(view.normalization)),
                ("Accuracy", accuracy_label(view.accuracy)),
            ],
        );
    });
}

fn nominal_evidence_card(ui: &mut Ui, width: f32, view: &TransferFunctionView<'_>) {
    let nominal_input = nominal_value_label(view.nominal_input, view.input_unit);
    let nominal_output = nominal_value_label(view.nominal_output, view.output_unit);
    let input_quantity = quantity_label(view.input_quantity);
    let output_quantity = quantity_label(view.output_quantity);
    let dataset = view.dataset_id.to_string();
    let status = if view.nominal_input.is_some() && view.nominal_output.is_some() {
        "retained"
    } else {
        "unavailable"
    };
    contract_card(ui, width, "Nominal evidence", status, |ui| {
        measurement_table(
            ui,
            &[
                ("Nominal input", nominal_input.as_str()),
                ("Nominal output", nominal_output.as_str()),
                ("Input quantity", input_quantity),
                ("Output quantity", output_quantity),
                ("Dataset", dataset.as_str()),
                ("Authority", view.dataset_authority),
            ],
        );
    });
}

fn contract_card(
    ui: &mut Ui,
    width: f32,
    title: &str,
    status: &str,
    add_contents: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    ui.allocate_ui_with_layout(
        egui::vec2(width.max(1.0), 232.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            egui::Frame::new()
                .fill(t.color.bg_panel)
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .inner_margin(egui::Margin::symmetric(10, 9))
                .show(ui, |ui| {
                    ui.set_min_size(egui::vec2((width - 22.0).max(1.0), 212.0));
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(title)
                                .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                                .color(t.color.text),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(status)
                                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                    .color(if status == "retained" {
                                        t.color.ok
                                    } else {
                                        t.color.text_faint
                                    }),
                            );
                        });
                    });
                    ui.separator();
                    add_contents(ui);
                });
        },
    );
}

fn nominal_value_label(value: Option<f64>, unit: &str) -> String {
    value.map_or_else(
        || "Not retained".to_owned(),
        |value| exact_value(value, unit),
    )
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Transfer function", None);
    let Some(view) = active_transfer_function(state) else {
        panel_note(
            ui,
            "Select an analysis with retained transfer-function evidence.",
        );
        return;
    };

    let input_quantity = quantity_label(view.input_quantity);
    let output_quantity = quantity_label(view.output_quantity);
    measurement_table(
        ui,
        &[
            ("Analysis", view.analysis_label),
            ("Input", view.input_source),
            ("Input quantity", input_quantity),
            ("Output", view.output_expression),
            ("Output quantity", output_quantity),
            ("Operating state", "DC operating point"),
            ("Normalization", normalization_label(view.normalization)),
            ("Accuracy", accuracy_label(view.accuracy)),
        ],
    );

    section_header(ui, "Exact scalar evidence", None);
    let gain = display_scalar(view.gain, gain_unit(&view));
    let rin = display_scalar(view.input_resistance, "ohm");
    let rout = display_scalar(view.output_resistance, "ohm");
    measurement_table(
        ui,
        &[
            ("Transfer gain", gain.as_str()),
            ("Input resistance", rin.as_str()),
            ("Output resistance", rout.as_str()),
        ],
    );
}

fn display_scalar(value: Option<TransferFunctionScalarEvidence>, unit: &str) -> String {
    match value {
        Some(TransferFunctionScalarEvidence::Finite(value)) => exact_value(value, unit),
        Some(TransferFunctionScalarEvidence::PositiveInfinity) => format!("+infinity {unit}"),
        Some(TransferFunctionScalarEvidence::NegativeInfinity) => format!("-infinity {unit}"),
        None => "Not retained".to_owned(),
    }
}

fn exact_value(value: f64, unit: &str) -> String {
    if unit == "1" {
        format!("{value:.9e}")
    } else {
        format!("{value:.9e} {unit}")
    }
}

fn quantity_label(quantity: TransferFunctionQuantityEvidence) -> &'static str {
    match quantity {
        TransferFunctionQuantityEvidence::Voltage => "Voltage",
        TransferFunctionQuantityEvidence::Current => "Current",
    }
}

fn normalization_label(value: TransferFunctionNormalizationEvidence) -> &'static str {
    match value {
        TransferFunctionNormalizationEvidence::None => "Disabled",
        TransferFunctionNormalizationEvidence::RelativeToNominal => "Relative to nominal",
        TransferFunctionNormalizationEvidence::PerSourceUnit => "Per source unit",
    }
}

fn accuracy_label(value: TransferFunctionAccuracyEvidence) -> &'static str {
    match value {
        TransferFunctionAccuracyEvidence::Fast => "Fast",
        TransferFunctionAccuracyEvidence::Balanced => "Balanced",
        TransferFunctionAccuracyEvidence::Accurate => "Accurate",
        TransferFunctionAccuracyEvidence::Robust => "Robust",
    }
}

fn gain_unit(view: &TransferFunctionView<'_>) -> &'static str {
    if view.normalization == TransferFunctionNormalizationEvidence::RelativeToNominal {
        return "1";
    }
    match (view.input_quantity, view.output_quantity) {
        (TransferFunctionQuantityEvidence::Voltage, TransferFunctionQuantityEvidence::Voltage) => {
            "V/V"
        }
        (TransferFunctionQuantityEvidence::Voltage, TransferFunctionQuantityEvidence::Current) => {
            "A/V"
        }
        (TransferFunctionQuantityEvidence::Current, TransferFunctionQuantityEvidence::Voltage) => {
            "V/A"
        }
        (TransferFunctionQuantityEvidence::Current, TransferFunctionQuantityEvidence::Current) => {
            "A/A"
        }
    }
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
        state.simulation.runs = vec![run];
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
        .result_payload;
        let mut mismatched = AnalysisResult::new(2, AnalysisType::Ac, "AC");
        mismatched.result_payload = payload;
        assert!(active_transfer_function(&state_with_analyses(vec![mismatched])).is_none());
    }

    #[test]
    fn scalar_display_keeps_infinity_and_units_explicit() {
        assert_eq!(
            display_scalar(
                Some(TransferFunctionScalarEvidence::PositiveInfinity),
                "ohm"
            ),
            "+infinity ohm"
        );
        assert_eq!(
            display_scalar(
                Some(TransferFunctionScalarEvidence::NegativeInfinity),
                "V/A"
            ),
            "-infinity V/A"
        );
        assert_eq!(display_scalar(None, "A/V"), "Not retained");
        assert_eq!(exact_value(0.5, "1"), "5.000000000e-1");
    }

    #[test]
    fn missing_nominal_evidence_remains_explicit() {
        assert_eq!(nominal_value_label(None, "V"), "Not retained");
        assert_eq!(nominal_value_label(Some(1.25), "V"), "1.250000000e0 V");
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
