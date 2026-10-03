//! Comparison policy controls over immutable source choices and an application commit.
use super::super::widgets::dock_intro;
use egui::Ui;
use rspice_app_types::product::{DatasetId, short_identity as short_dataset};
use rspice_ui_kit::{panels::property_row, widgets::Button};

use rspice_results::studio_presentation::ComparisonAlignmentDraft;

pub struct ComparisonDraft<'a> {
    pub dataset: &'a mut Option<DatasetId>,
    pub alignment: &'a mut ComparisonAlignmentDraft,
    pub signal: &'a mut String,
    pub threshold: &'a mut f64,
    pub maximum_lag_samples: &'a mut u32,
    pub absolute_tolerance: &'a mut f64,
    pub relative_tolerance: &'a mut f64,
    pub difference_trace: &'a mut bool,
}
pub trait ComparisonHost {
    fn prepare(&mut self) -> bool;
    fn candidate(&self) -> Option<(DatasetId, &str)>;
    fn selected_label(&self) -> Option<&str>;
    fn baselines(
        &mut self,
    ) -> (
        &mut Option<DatasetId>,
        impl Iterator<Item = (DatasetId, &str)>,
    );
    fn draft(&mut self) -> ComparisonDraft<'_>;
    fn signals(&self) -> Vec<String>;
    fn create(&mut self) -> bool;
}

pub fn show(ui: &mut Ui, host: &mut impl ComparisonHost) -> bool {
    dock_intro(
        ui,
        "RESULTS · IMMUTABLE COMPARISON",
        "Create an executable receipt only after every numerical policy is explicit.",
    );
    let has_candidate = host.prepare();
    if let Some((dataset, label)) = host.candidate() {
        property_row(
            ui,
            "Candidate",
            &format!("{} · {}", label, short_dataset(dataset)),
        );
    }
    ui.label("Baseline dataset");
    let selected_label = host.selected_label().unwrap_or("Select immutable dataset");
    egui::ComboBox::from_id_salt("visualization.comparison.dataset")
        .selected_text(selected_label)
        .show_ui(ui, |ui| {
            let (selected, rows) = host.baselines();
            for (dataset, label) in rows {
                ui.selectable_value(
                    selected,
                    Some(dataset),
                    format!("{} · {}", label, short_dataset(dataset)),
                );
            }
        });
    ui.label("Alignment");
    egui::ComboBox::from_id_salt("visualization.comparison.alignment")
        .selected_text(host.draft().alignment.label())
        .show_ui(ui, |ui| {
            for alignment in ComparisonAlignmentDraft::ALL {
                ui.selectable_value(host.draft().alignment, alignment, alignment.label());
            }
        });
    let signal_names = host.signals();
    let requires_signal = *host.draft().alignment != ComparisonAlignmentDraft::AbsoluteXAxis;
    if requires_signal {
        if !signal_names.contains(host.draft().signal) {
            *host.draft().signal = signal_names.first().cloned().unwrap_or_default();
        }
        ui.label("Alignment signal");
        egui::ComboBox::from_id_salt("visualization.comparison.alignment-signal")
            .selected_text(host.draft().signal.clone())
            .show_ui(ui, |ui| {
                for signal in &signal_names {
                    ui.selectable_value(host.draft().signal, signal.clone(), signal);
                }
            });
    }
    match *host.draft().alignment {
        ComparisonAlignmentDraft::FirstThresholdCrossing => {
            ui.horizontal(|ui| {
                ui.label("Threshold");
                ui.add(egui::DragValue::new(host.draft().threshold).speed(1.0e-6));
            });
        }
        ComparisonAlignmentDraft::CrossCorrelation => {
            ui.horizontal(|ui| {
                ui.label("Maximum lag (samples)");
                ui.add(egui::DragValue::new(host.draft().maximum_lag_samples).range(1..=4_096));
            });
        }
        ComparisonAlignmentDraft::AbsoluteXAxis => {}
    }
    let (interpolation, resampling) = match *host.draft().alignment {
        ComparisonAlignmentDraft::AbsoluteXAxis => {
            ("None · exact coordinates", "Exact coordinate intersection")
        }
        ComparisonAlignmentDraft::FirstThresholdCrossing => {
            ("Monotone linear", "Baseline onto candidate grid")
        }
        ComparisonAlignmentDraft::CrossCorrelation => ("Monotone linear", "Uniform overlap grid"),
    };
    for (label, value) in [
        ("Units", "Require identical units"),
        ("Interpolation", interpolation),
        ("Resampling", resampling),
        ("Extrapolation", "Forbidden"),
        ("Precision", "Source f64 · no rounding"),
    ] {
        property_row(ui, label, value);
    }
    ui.horizontal(|ui| {
        ui.label("Absolute tolerance");
        ui.add(
            egui::DragValue::new(host.draft().absolute_tolerance)
                .speed(1.0e-6)
                .range(0.0..=f64::MAX),
        );
    });
    ui.horizontal(|ui| {
        ui.label("Relative tolerance");
        ui.add(
            egui::DragValue::new(host.draft().relative_tolerance)
                .speed(1.0e-6)
                .range(0.0..=f64::MAX),
        );
    });
    ui.checkbox(
        host.draft().difference_trace,
        "Difference trace · absolute, relative, and normalized",
    );
    let draft = host.draft();
    let valid = has_candidate
        && draft.dataset.is_some()
        && draft.absolute_tolerance.is_finite()
        && *draft.absolute_tolerance >= 0.0
        && draft.relative_tolerance.is_finite()
        && *draft.relative_tolerance >= 0.0
        && draft.threshold.is_finite()
        && (!requires_signal || !draft.signal.is_empty())
        && (*draft.alignment != ComparisonAlignmentDraft::CrossCorrelation
            || *draft.maximum_lag_samples > 0);
    let create = Button::new("Create comparison receipt")
        .accent()
        .enabled(valid)
        .show(ui)
        .clicked();
    if !create {
        return false;
    }
    host.create()
}
