use super::*;

// =============================================================================
// Header Rendering
// =============================================================================

fn sync_manual_fft_time_window(state: &mut FftState, source_time_bounds: Option<(f64, f64)>) {
    let Some((min_t, max_t)) = source_time_bounds else {
        return;
    };

    if state.time_window_auto {
        state.time_window_start = min_t;
        state.time_window_end = max_t;
        return;
    }

    state.time_window_start = state.time_window_start.clamp(min_t, max_t);
    state.time_window_end = state.time_window_end.clamp(min_t, max_t);
    if state.time_window_end <= state.time_window_start {
        state.time_window_start = min_t;
        state.time_window_end = max_t;
    }
}

pub(super) fn render_time_controls_header(
    ui: &mut Ui,
    layout: &FftLayout,
    state: &mut FftState,
    source_time_bounds: Option<(f64, f64)>,
) -> HeaderActions {
    let mut actions = HeaderActions::default();
    ui.painter()
        .rect_filled(layout.header_top, Rounding::ZERO, header_bg_color());

    sync_manual_fft_time_window(state, source_time_bounds);

    let header_rect = layout.header_top.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.spacing_mut().item_spacing = egui::vec2(6.0, 4.0);

        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().interact_size.y = HEADER_CONTROL_HEIGHT;
                ui.spacing_mut().button_padding.y = 2.0;
                ui.add_space(4.0);

                ui.label("Auto Time");
                let mut time_changed = ui.checkbox(&mut state.time_window_auto, "").changed();
                let time_speed = source_time_bounds
                    .map(|(min_t, max_t)| ((max_t - min_t).abs() / 1000.0).max(1e-15))
                    .unwrap_or(1e-9);
                ui.add_enabled_ui(!state.time_window_auto, |ui| {
                    ui.label("Start");
                    if ui
                        .add(
                            egui::DragValue::new(&mut state.time_window_start)
                                .speed(time_speed)
                                .max_decimals(12),
                        )
                        .changed()
                    {
                        time_changed = true;
                    }
                    ui.label("End");
                    if ui
                        .add(
                            egui::DragValue::new(&mut state.time_window_end)
                                .speed(time_speed)
                                .max_decimals(12),
                        )
                        .changed()
                    {
                        time_changed = true;
                    }
                });
                sync_manual_fft_time_window(state, source_time_bounds);
                if time_changed {
                    queue_fft_refresh(&mut actions, state);
                }

                ui.separator();

                ui.label("Auto N");
                let mut sample_changed = ui.checkbox(&mut state.sample_count_auto, "").changed();
                ui.add_enabled_ui(!state.sample_count_auto, |ui| {
                    ui.label("N");
                    let mut sample_count = state.sample_count as u64;
                    if ui
                        .add(
                            egui::DragValue::new(&mut sample_count)
                                .range(
                                    crate::analysis::fft::MIN_FFT_SAMPLES as u64
                                        ..=crate::analysis::fft::MAX_REFERENCE_RESAMPLE_POINTS
                                            as u64,
                                )
                                .speed(1.0),
                        )
                        .changed()
                    {
                        state.sample_count = sample_count as usize;
                        sample_changed = true;
                    }
                });
                state.sync_sample_count_control_value();
                if sample_changed {
                    queue_fft_refresh(&mut actions, state);
                }
            });
        });
    });

    actions
}

pub(super) fn render_header(
    ui: &mut Ui,
    layout: &FftLayout,
    state: &mut FftState,
    source_names: &[String],
) -> HeaderActions {
    let mut actions = HeaderActions::default();
    ui.painter()
        .rect_filled(layout.header_main, Rounding::ZERO, header_bg_color());

    let header_rect = layout.header_main.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.spacing_mut().item_spacing = egui::vec2(6.0, 4.0);

        ui.horizontal(|ui| {
            ui.spacing_mut().interact_size.y = HEADER_CONTROL_HEIGHT;
            ui.spacing_mut().button_padding.y = 2.0;
            ui.add_space(4.0);

            // ui.label(
            //     egui::RichText::new("FFT Spectrum")
            //         .size(13.0)
            //         .strong()
            //         .color(Color32::from_rgb(200, 200, 210)),
            // );

            // ui.add_space(8.0);

            let mut selected_source = state
                .selected_source
                .clone()
                .or_else(|| state.source_cache.as_ref().map(|src| src.name.clone()));
            let source_selected_text = selected_source.as_deref().unwrap_or("Source waveform");
            let source_width = combo_width_from_texts(
                ui,
                source_selected_text,
                source_names
                    .iter()
                    .map(String::as_str)
                    .chain(std::iter::once("Source waveform")),
                HEADER_DROPDOWN_MIN_WIDTH,
                HEADER_DROPDOWN_MAX_WIDTH,
            );
            egui::ComboBox::from_id_salt("fft_source")
                .width(source_width)
                .selected_text(source_selected_text)
                .show_ui(ui, |ui| {
                    for name in source_names {
                        ui.selectable_value(&mut selected_source, Some(name.clone()), name);
                    }
                });
            if selected_source != state.selected_source {
                state.set_selected_source(selected_source.clone());
                actions.refresh_source = selected_source;
            }

            let mut window = state.window;
            let window_width = combo_width_from_texts(
                ui,
                window.display_name(),
                WindowFunction::all().iter().map(|w| w.display_name()),
                HEADER_DROPDOWN_MIN_WIDTH,
                HEADER_DROPDOWN_MAX_WIDTH,
            );
            egui::ComboBox::from_id_salt("fft_window")
                .width(window_width)
                .selected_text(window.display_name())
                .show_ui(ui, |ui| {
                    for w in WindowFunction::all() {
                        ui.selectable_value(&mut window, *w, w.display_name());
                    }
                });
            if window != state.window {
                state.set_window(window);
            }

            let mut mag_scale = state.mag_scale;
            let mag_scale_width = combo_width_from_texts(
                ui,
                mag_scale.display_name(),
                MagnitudeScale::all().iter().map(|s| s.display_name()),
                HEADER_DROPDOWN_MIN_WIDTH,
                HEADER_DROPDOWN_MAX_WIDTH,
            );
            egui::ComboBox::from_id_salt("fft_mag_scale")
                .width(mag_scale_width)
                .selected_text(mag_scale.display_name())
                .show_ui(ui, |ui| {
                    for s in MagnitudeScale::all() {
                        ui.selectable_value(&mut mag_scale, *s, s.display_name());
                    }
                });
            if mag_scale != state.mag_scale {
                state.set_mag_scale(mag_scale);
            }

            let mut freq_scale = state.freq_scale;
            let freq_scale_width = combo_width_from_texts(
                ui,
                freq_scale.display_name(),
                FrequencyScale::all().iter().map(|s| s.display_name()),
                HEADER_DROPDOWN_MIN_WIDTH,
                HEADER_DROPDOWN_MAX_WIDTH,
            );
            egui::ComboBox::from_id_salt("fft_freq_scale")
                .width(freq_scale_width)
                .selected_text(freq_scale.display_name())
                .show_ui(ui, |ui| {
                    for s in FrequencyScale::all() {
                        ui.selectable_value(&mut freq_scale, *s, s.display_name());
                    }
                });
            if freq_scale != state.freq_scale {
                state.set_freq_scale(freq_scale);
            }

            let mut normalization = state.normalization;
            let normalization_width = combo_width_from_texts(
                ui,
                normalization.display_name(),
                SpectrumNormalization::all()
                    .iter()
                    .map(|mode| mode.display_name()),
                HEADER_DROPDOWN_MIN_WIDTH,
                HEADER_DROPDOWN_MAX_WIDTH,
            );
            egui::ComboBox::from_id_salt("fft_norm")
                .width(normalization_width)
                .selected_text(normalization.display_name())
                .show_ui(ui, |ui| {
                    for mode in SpectrumNormalization::all() {
                        ui.selectable_value(&mut normalization, *mode, mode.display_name());
                    }
                });
            if normalization != state.normalization {
                state.set_normalization(normalization);
            }

            let mut fidelity = state.input_fidelity;
            let fidelity_width = combo_width_from_texts(
                ui,
                fidelity.display_name(),
                InputFidelity::all().iter().map(|mode| mode.display_name()),
                HEADER_DROPDOWN_MIN_WIDTH,
                HEADER_DROPDOWN_MAX_WIDTH,
            );
            egui::ComboBox::from_id_salt("fft_fidelity")
                .width(fidelity_width)
                .selected_text(fidelity.display_name())
                .show_ui(ui, |ui| {
                    for mode in InputFidelity::all() {
                        ui.selectable_value(&mut fidelity, *mode, mode.display_name());
                    }
                });
            if fidelity != state.input_fidelity {
                state.set_input_fidelity(fidelity);
                queue_fft_refresh(&mut actions, state);
            }

            ui.separator();

            let peaks_label = if state.show_peaks {
                "Peaks [on]"
            } else {
                "Peaks"
            };
            if ui.button(peaks_label).clicked() {
                state.toggle_peaks();
            }

            let harmonics_label = if state.show_harmonics {
                "Harm [on]"
            } else {
                "Harm"
            };
            if ui.button(harmonics_label).clicked() {
                state.toggle_harmonics();
            }

            let grid_label = if state.show_grid { "Grid [on]" } else { "Grid" };
            if ui.button(grid_label).clicked() {
                state.toggle_grid();
            }

            ui.separator();
            if ui.button("Clear Markers").clicked() {
                state.clear_markers();
            }
            ui.label(egui::RichText::new("Alt+LMB add, Alt+RMB remove").small());

            ui.separator();

            ui.label("Auto Freq");
            if ui.checkbox(&mut state.freq_auto, "").changed() && state.freq_auto {
                state.update_auto_scale();
            }

            ui.add_enabled_ui(!state.freq_auto, |ui| {
                ui.label("Min");
                ui.add(egui::DragValue::new(&mut state.freq_min).speed(10.0));
                ui.label("Max");
                ui.add(egui::DragValue::new(&mut state.freq_max).speed(10.0));
            });

            if state.freq_scale == FrequencyScale::Log {
                state.freq_min = state.freq_min.max(1e-12);
            }
            if state.freq_max <= state.freq_min {
                state.freq_max = state.freq_min * 1.01;
            }

            ui.separator();

            ui.label("Auto Mag");
            if ui.checkbox(&mut state.mag_auto, "").changed() && state.mag_auto {
                state.update_auto_scale();
            }

            ui.add_enabled_ui(!state.mag_auto, |ui| {
                ui.label("Min");
                ui.add(egui::DragValue::new(&mut state.mag_min).speed(0.5));
                ui.label("Max");
                ui.add(egui::DragValue::new(&mut state.mag_max).speed(0.5));
            });
            if state.mag_max <= state.mag_min {
                state.mag_max = state.mag_min + 1.0;
            }

            ui.separator();

            ui.label("Peak Th (dB)");
            ui.add(
                egui::DragValue::new(&mut state.peak_threshold_db)
                    .speed(0.5)
                    .fixed_decimals(1),
            );
            state.peak_threshold_db = state.peak_threshold_db.clamp(-180.0, 20.0);

            ui.label("Harmonics");
            let mut harmonics = state.num_harmonics as u32;
            if ui
                .add(egui::DragValue::new(&mut harmonics).speed(1.0))
                .changed()
            {
                state.set_num_harmonics(harmonics.clamp(1, 64) as usize);
            }
        });
    });

    actions
}
