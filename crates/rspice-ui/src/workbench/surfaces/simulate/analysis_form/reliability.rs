//! The reliability form: the ages the degradation is projected to, and which
//! wear-out mechanisms are projected.

use egui::Ui;

use crate::simulation::dialog::ReliabilityDialogState;
use crate::simulation::dialog::reliability::{ReliabilityBindingDraft, ReliabilityMissionDraft};

use super::{
    action_line, action_line_enabled, clear_pending_cell, field_advisory, field_note, input_row,
    sub_header, switch_row,
};

/// Render the reliability fields.
pub(super) fn fields(ui: &mut Ui, setup: &mut ReliabilityDialogState) {
    input_row(ui, "Years", &mut setup.years_csv);
    field_note(
        ui,
        "Years selects the lifetime checkpoints. Every bound-device stress sample is retained for aging calculations and evidence export.",
    );
    input_row(ui, "Min stress V", &mut setup.min_stress_voltage);
    field_note(
        ui,
        "Below the minimum gate stress, power-law aging and trap capture stop. Trap recovery continues at the observed bias and temperature.",
    );
    switch_row(ui, "Hot carrier (HCI)", &mut setup.enable_hci);
    switch_row(ui, "Bias instability (NBTI)", &mut setup.enable_nbti);
    switch_row(ui, "Electromigration", &mut setup.enable_em);
    sub_header(ui, "Aging model pack");
    let study = &mut setup.study;
    #[cfg(not(target_arch = "wasm32"))]
    if action_line(ui, "Load model pack…")
        && let Some(path) = rfd::FileDialog::new()
            .add_filter("Aging model pack", &["json"])
            .pick_file()
    {
        study.import_error = study.load_model_pack(&path).err();
    }
    if let Some(error) = &study.import_error {
        field_advisory(ui, error);
    }
    field_note(
        ui,
        "Load calibration with its process, source, units and validity limits. Two-state tables include capture/emission rates, initial occupancies and parameter couplings. The project retains a copy of the loaded data.",
    );
    ui.collapsing("Model pack JSON", |ui| {
        ui.add(
            egui::TextEdit::multiline(&mut study.model_pack_json)
                .code_editor()
                .desired_rows(8)
                .desired_width(f32::INFINITY)
                .char_limit(rspice_core::analysis::reliability::MAX_AGING_PACK_BYTES),
        );
    });

    sub_header(ui, "Device assignments");
    field_note(
        ui,
        "Use exact device and compact-model names. Aging model IDs come from the loaded pack; separate multiple IDs with commas. EM requires conductor area in m².",
    );
    let mut remove = None;
    for (index, binding) in study.bindings.iter_mut().enumerate() {
        ui.push_id(("aging-binding", index), |ui| {
            input_row(ui, "Device", &mut binding.device);
            input_row(ui, "Compact model", &mut binding.compact_model);
            input_row(ui, "Aging model IDs", &mut binding.aging_models);
            input_row(ui, "Conductor area · m²", &mut binding.conductor_area_m2);
            if action_line(ui, "Remove assignment") {
                remove = Some(index);
            }
        });
    }
    if let Some(index) = remove {
        study.bindings.remove(index);
    }
    if action_line_enabled(ui, "Add device assignment", study.bindings.len() < 4096) {
        study.bindings.push(ReliabilityBindingDraft::default());
    }

    sub_header(ui, "Mission profile");
    switch_row(ui, "Repeat mission", &mut study.repeat_mission);
    field_note(
        ui,
        "Phases run in order. Duration is in seconds; temperature is in °C. Repetition covers lifetime checkpoints beyond one mission.",
    );
    let mut remove = None;
    let mut movement = None;
    let count = study.mission.len();
    for (index, phase) in study.mission.iter_mut().enumerate() {
        ui.push_id(("aging-mission", index), |ui| {
            sub_header(ui, &format!("Phase {}", index + 1));
            input_row(ui, "Name", &mut phase.name);
            input_row(ui, "Duration · s", &mut phase.duration_s);
            input_row(ui, "Temperature · °C", &mut phase.temperature_c);
            input_row(ui, "Parameters", &mut phase.parameters)
                .on_hover_text("Existing .param overrides, e.g. VDD=1.2, LOAD=10k");
            if action_line_enabled(ui, "Move phase earlier", index > 0) {
                movement = Some((index, index - 1));
            }
            if action_line_enabled(ui, "Move phase later", index + 1 < count) {
                movement = Some((index, index + 1));
            }
            if action_line(ui, "Remove phase") {
                remove = Some(index);
            }
        });
    }
    if let Some(index) = remove {
        study.mission.remove(index);
    } else if let Some((a, b)) = movement {
        study.mission.swap(a, b);
    }
    if action_line_enabled(ui, "Add mission phase", study.mission.len() < 4096) {
        study.mission.push(ReliabilityMissionDraft {
            name: format!("phase{}", study.mission.len() + 1),
            ..Default::default()
        });
    }
    sub_header(ui, "Stress sampling");
    switch_row(ui, "Transient stress", &mut study.transient_stress);
    if study.transient_stress {
        input_row(ui, "Step · s", &mut study.step_s);
        input_row(ui, "Stop · s", &mut study.stop_s);
        input_row(ui, "Start · s", &mut study.start_s);
        input_row(ui, "Maximum step · s", &mut study.max_step_s);
        switch_row(
            ui,
            "Use initial conditions",
            &mut study.use_initial_conditions,
        );
    } else {
        field_note(
            ui,
            "Each phase uses its circuit operating point as constant stress.",
        );
    }
    clear_pending_cell(ui);
}
