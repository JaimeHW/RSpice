//! Transfer-function cards and inspector over host-qualified scalar evidence.

use crate::presentation::{panel_note, well_hint};
use crate::strip::StripHeader;
use egui::Ui;
use rspice_app_types::product::DatasetId;
use rspice_results::transfer_function::{
    TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
    TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
};
use rspice_ui_kit::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{measurement_table, section_header},
};

/// Borrowed retained values and provenance labels selected and validated by the host.
pub struct TransferFunctionView<'a> {
    pub analysis_label: &'a str,
    pub input_source: &'a str,
    pub output_expression: &'a str,
    pub input_quantity: TransferFunctionQuantityEvidence,
    pub output_quantity: TransferFunctionQuantityEvidence,
    pub input_unit: &'a str,
    pub output_unit: &'a str,
    pub normalization: TransferFunctionNormalizationEvidence,
    pub accuracy: TransferFunctionAccuracyEvidence,
    pub gain: Option<TransferFunctionScalarEvidence>,
    pub input_resistance: Option<TransferFunctionScalarEvidence>,
    pub output_resistance: Option<TransferFunctionScalarEvidence>,
    pub nominal_input: Option<f64>,
    pub nominal_output: Option<f64>,
    pub dataset_id: DatasetId,
    pub dataset_authority: &'static str,
}

pub fn show(ui: &mut Ui, view: Option<TransferFunctionView<'_>>) {
    let Some(view) = view else {
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

pub fn right_panel(ui: &mut Ui, view: Option<TransferFunctionView<'_>>) {
    section_header(ui, "Transfer function", None);
    let Some(view) = view else {
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

pub fn gain_unit(view: &TransferFunctionView<'_>) -> &'static str {
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
}
