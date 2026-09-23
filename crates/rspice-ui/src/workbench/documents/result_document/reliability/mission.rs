//! Mission phase, lifetime, parameter and electrical views for calibrated fits.
use super::*;

pub(super) fn is_active(state: &AppState) -> bool {
    active_evidence_is_valid(state)
        && state.simulation.active_analysis().is_some_and(|a| {
            a.success
                && a.analysis_type == AnalysisType::Reliability
                && matches!(
                    a.result_payload,
                    Some(AnalysisResultPayload::ReliabilityMission { .. })
                )
        })
}

pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    let analysis = state.simulation.active_analysis().unwrap();
    let Some(AnalysisResultPayload::ReliabilityMission { response }) = &analysis.result_payload
    else {
        return;
    };
    let key = ui.make_persistent_id((
        "reliability-mission",
        analysis.id,
        state.simulation.active_run().unwrap().dataset_id,
    ));
    let (mut phase, mut age, mut selected_trace) = ui
        .data_mut(|d| d.get_temp::<(usize, usize, String)>(key))
        .unwrap_or_default();
    phase = phase.min(response.stress.phases.len() - 1);
    age = age.min(response.stress.checkpoints.len() - 1);
    section_header(ui, "Reliability mission", Some("engineering preview"));
    ui.label(format!(
        "{} · {} · {}",
        response.stress.request.study.model_pack.id,
        response.stress.request.study.model_pack.process,
        analysis.label
    ));
    ui.horizontal_wrapped(|ui| {
        egui::ComboBox::from_id_salt("mission-phase")
            .selected_text(&response.stress.request.study.mission[phase].name)
            .show_ui(ui, |ui| {
                for (index, p) in response.stress.request.study.mission.iter().enumerate() {
                    ui.selectable_value(&mut phase, index, format!("{}: {}", index + 1, p.name));
                }
            });
        egui::ComboBox::from_id_salt("mission-age")
            .selected_text(format!(
                "{} years",
                response.stress.request.target_years[age]
            ))
            .show_ui(ui, |ui| {
                for (index, years) in response.stress.request.target_years.iter().enumerate() {
                    ui.selectable_value(&mut age, index, format!("{years} years"));
                }
            });
    });
    let point = &response.aged[phase * response.stress.checkpoints.len() + age];
    if !analysis.waveforms.iter().any(|w| w.name == selected_trace) {
        selected_trace = analysis
            .waveforms
            .first()
            .map(|w| w.name.clone())
            .unwrap_or_default();
    }
    egui::ComboBox::from_id_salt("mission-trace")
        .width(ui.available_width().min(650.0))
        .selected_text(&selected_trace)
        .show_ui(ui, |ui| {
            for trace in &analysis.waveforms {
                ui.selectable_value(&mut selected_trace, trace.name.clone(), &trace.name);
            }
        });
    if let Some(trace) = analysis.waveforms.iter().find(|w| w.name == selected_trace)
        && let Some((x0, x1)) = lifetime_range(&trace.x)
        && let Some((y0, y1)) = padded_range(&trace.y, false)
    {
        let mut spec = PlotSpec::new(
            Axis::linear_with(x0, x1, "years", 6),
            XScale::Linear,
            Axis::linear_with(y0, y1, trace.unit.as_deref().unwrap_or(""), 6),
        );
        spec.traces.push(
            Trace::new(&trace.x, &trace.y, Tokens::get(ui.ctx()).color.traces[0]).marker_style(0),
        );
        ui.allocate_ui(egui::vec2(ui.available_width(), 190.0), |ui| {
            plot::show(ui, &spec, &mut state.ui.results.cache, None, None);
        });
    }
    ui.label(format!(
        "Phase {}: {} °C · {:.6e} s per mission · {} applied model parameters",
        phase + 1,
        response.stress.request.study.mission[phase].temperature_c,
        response.stress.request.study.mission[phase].duration_s,
        point.parameters.len()
    ));
    TableBuilder::new(ui)
        .striped(true)
        .column(Column::remainder())
        .column(Column::auto())
        .column(Column::auto())
        .column(Column::auto())
        .column(Column::auto())
        .column(Column::auto())
        .max_scroll_height(240.0)
        .header(24.0, |mut h| {
            for name in [
                "Device / model",
                "Parameter",
                "Mode",
                "Shift",
                "Fresh",
                "Aged",
            ] {
                h.col(|ui| {
                    ui.strong(name);
                });
            }
        })
        .body(|body| {
            body.rows(23.0, point.parameters.len(), |mut row| {
                let p = &point.parameters[row.index()];
                for text in [
                    format!("{} / {}", p.device, p.compact_model),
                    p.parameter.clone(),
                    format!("{:?}", p.update),
                    format!("{:+.8e}", p.shift),
                    format!("{:.8e}", p.fresh_value),
                    format!("{:.8e}", p.aged_value),
                ] {
                    row.col(|ui| {
                        ui.label(text);
                    });
                }
            });
        });
    ui.collapsing(
        "Aging clocks, trap occupancy and electromigration lifetime",
        |ui| {
            for device in &response.stress.checkpoints[age].devices {
                for c in &device.contributions {
                    ui.label(format!(
                        "{} / {}: {:.8e} {} seconds{}",
                        device.device,
                        c.model_id,
                        c.equivalent_seconds,
                        if c.trap_occupancies.is_empty() {
                            "reference"
                        } else {
                            "elapsed"
                        },
                        c.electromigration_lifetime_fraction
                            .map_or_else(String::new, |v| format!(", {v:.8e} consumed lifetime"))
                    ));
                    for trap in &c.trap_occupancies {
                        ui.label(format!(
                            "{}: {:.8e} occupied fraction",
                            trap.trap_id, trap.occupancy
                        ));
                    }
                }
            }
        },
    );
    ui.data_mut(|d| d.insert_temp(key, (phase, age, selected_trace)));
}

pub(super) fn details(ui: &mut Ui, state: &AppState) {
    let Some(AnalysisResultPayload::ReliabilityMission { response }) = state
        .simulation
        .active_analysis()
        .and_then(|a| a.result_payload.as_ref())
    else {
        return;
    };
    let pack = &response.stress.request.study.model_pack;
    section_header(ui, "Calibration and mission", None);
    for text in [
        format!("Model pack: {}", pack.id),
        format!("Process: {}", pack.process),
        format!("Source: {}", pack.source),
        format!("License: {}", pack.license),
        format!("Qualification: {:?}", pack.qualification),
        pack.characterization.clone(),
    ] {
        ui.label(text);
    }
    panel_note(
        ui,
        "Stress comes from the fresh circuit. Power-law fits accumulate irreversible age; two-state tables retain capture and recovery in mission order. EM consumed lifetime is not a failure probability. CSV export retains stress samples, trap occupancies, model changes and electrical observations.",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reliability_mission_viewer_uses_retained_evidence_and_renders_a_frame() {
        let analysis =
            crate::simulation::SimulationResult::reliability_mission_retained_test_fixture();
        let mut state = AppState::default();
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(analysis);
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(is_active(&state));
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show(ui, &mut state);
                details(ui, &state);
            });
        });
    }
}
