//! Typed family selection, encoding, and filter controls.
use super::super::widgets::{dock_intro, empty_note};
use egui::Ui;
use rspice_results::family_projection::{FamilyManifest, FamilyValueKind};
use rspice_ui_kit::widgets::Button;

pub struct FamilyDraft<'a> {
    pub x: &'a mut String,
    pub family: &'a mut String,
    pub color: &'a mut String,
    pub dash: &'a mut String,
    pub marker: &'a mut String,
    pub query: &'a mut String,
    pub exclude_missing: &'a mut bool,
}
pub trait FamilyHost {
    fn draft(&mut self) -> FamilyDraft<'_>;
    fn manifest(&self) -> Result<FamilyManifest, String>;
    fn matching_indices(&self, manifest: &FamilyManifest) -> Result<Vec<usize>, String>;
    fn trace_count(&self) -> usize;
    fn validate(&self, manifest: &FamilyManifest, indices: Vec<usize>) -> Result<(), String>;
    fn apply(&mut self);
}

pub fn slice(ui: &mut Ui, host: &mut impl FamilyHost) -> bool {
    dock_intro(
        ui,
        "RESULTS · N-DIMENSIONAL DATA",
        "Choose typed dimensions from the active immutable result family.",
    );
    let manifest = host.manifest();
    let valid = match manifest.as_ref() {
        Ok(manifest) => {
            family_dimension_combo(
                ui,
                "family.slice.x",
                "X dimension",
                host.draft().x,
                manifest,
                true,
                false,
            );
            family_dimension_combo(
                ui,
                "family.slice.family",
                "Family dimension",
                host.draft().family,
                manifest,
                false,
                false,
            );
            ui.label("Filter");
            ui.text_edit_singleline(host.draft().query);
            family_policy_preview_is_valid(ui, host, manifest)
        }
        Err(error) => {
            empty_note(ui, error);
            false
        }
    };
    let apply = Button::new("Apply slice and pivot")
        .accent()
        .enabled(valid)
        .show(ui)
        .clicked();
    if apply {
        host.apply();
    }
    apply
}

pub fn encoding(ui: &mut Ui, host: &mut impl FamilyHost) -> bool {
    dock_intro(
        ui,
        "RESULTS · ACCESSIBLE TRACE FAMILIES",
        "Configure redundant visual encoding supported by the retained renderer.",
    );
    let manifest = host.manifest();
    let valid = match manifest.as_ref() {
        Ok(manifest) => {
            family_dimension_combo(
                ui,
                "family.encoding.color",
                "Color",
                host.draft().color,
                manifest,
                false,
                false,
            );
            family_dimension_combo(
                ui,
                "family.encoding.dash",
                "Dash",
                host.draft().dash,
                manifest,
                false,
                true,
            );
            family_dimension_combo(
                ui,
                "family.encoding.marker",
                "Marker",
                host.draft().marker,
                manifest,
                false,
                true,
            );
            family_policy_preview_is_valid(ui, host, manifest)
        }
        Err(error) => {
            empty_note(ui, error);
            false
        }
    };
    let apply = Button::new("Apply encoding")
        .accent()
        .enabled(valid)
        .show(ui)
        .clicked();
    if apply {
        host.apply();
    }
    apply
}

pub fn filter(ui: &mut Ui, host: &mut impl FamilyHost) -> bool {
    dock_intro(
        ui,
        "RESULTS · DATA QUERY",
        "Filter exact typed family points without changing immutable solver output.",
    );
    ui.label("Expression");
    ui.text_edit_singleline(host.draft().query);
    ui.label("Missing points");
    ui.radio_value(host.draft().exclude_missing, false, "Preserve as not-run");
    ui.radio_value(
        host.draft().exclude_missing,
        true,
        "Exclude with omission record",
    );
    let manifest = host.manifest();
    let valid = match manifest.as_ref() {
        Ok(manifest) => family_policy_preview_is_valid(ui, host, manifest),
        Err(error) => {
            empty_note(ui, error);
            false
        }
    };
    let apply = Button::new("Apply filter")
        .accent()
        .enabled(valid)
        .show(ui)
        .clicked();
    if apply {
        host.apply();
    }
    apply
}

fn family_dimension_combo(
    ui: &mut Ui,
    id: &'static str,
    label: &str,
    selected: &mut String,
    manifest: &FamilyManifest,
    numeric_only: bool,
    allow_none: bool,
) {
    ui.horizontal(|ui| {
        ui.label(label);
        let selected_text = if selected.is_empty() {
            "none".to_owned()
        } else {
            selected.clone()
        };
        egui::ComboBox::from_id_salt(id)
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                if allow_none {
                    ui.selectable_value(selected, String::new(), "none");
                }
                for dimension in &manifest.dimensions {
                    if dimension.id == "status"
                        || (numeric_only
                            && !matches!(
                                dimension.kind,
                                FamilyValueKind::Number | FamilyValueKind::Integer
                            ))
                    {
                        continue;
                    }
                    let display = dimension.unit.as_ref().map_or_else(
                        || dimension.label.clone(),
                        |unit| format!("{} ({unit})", dimension.label),
                    );
                    ui.selectable_value(selected, dimension.id.clone(), display);
                }
            });
    });
}

fn family_preview(
    ui: &mut Ui,
    host: &impl FamilyHost,
    manifest: &FamilyManifest,
) -> Result<Vec<usize>, String> {
    let indices = match host.matching_indices(manifest) {
        Ok(indices) => indices,
        Err(error) => {
            empty_note(ui, &error);
            return Err(error);
        }
    };
    let trace_count = host.trace_count();
    let selected_samples = trace_count.saturating_mul(indices.len());
    let omission = if manifest.omitted_points == 0 {
        String::new()
    } else {
        format!(
            " · {} unavailable point(s) recorded",
            manifest.omitted_points
        )
    };
    ui.label(format!(
        "{} of {} retained points · {trace_count} traces · {selected_samples} selected samples{omission}",
        indices.len(),
        manifest.points.len()
    ));
    Ok(indices)
}

fn family_policy_preview_is_valid(
    ui: &mut Ui,
    host: &impl FamilyHost,
    manifest: &FamilyManifest,
) -> bool {
    let result = (|| {
        let indices = family_preview(ui, host, manifest)?;
        host.validate(manifest, indices)
    })();
    if let Err(error) = result {
        empty_note(
            ui,
            &format!(
                "This draft cannot be applied to the waveform renderer: {error} Choose an X dimension that is finite, losslessly numeric, and strictly increasing within every selected family group."
            ),
        );
        false
    } else {
        true
    }
}
