use super::*;

// =============================================================================
// Info Panel
// =============================================================================

pub(super) fn render_info_panel(ui: &mut Ui, layout: &FftLayout, state: &mut FftState) {
    ui.painter()
        .rect_filled(layout.info, Rounding::ZERO, panel_bg_color());
    if let Some(outline_rect) = info_outline_rect(layout) {
        ui.painter().rect_stroke(
            outline_rect,
            Rounding::ZERO,
            Stroke::new(1.0, panel_border_color()),
        );
    }

    let panel_rect = info_content_rect(layout);
    ui.allocate_new_ui(UiBuilder::new().max_rect(panel_rect), |ui| {
        egui::ScrollArea::vertical()
            .id_salt("fft_info_panel_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Keep the right-aligned value column stable while scrolling.
                ui.set_width(ui.available_width());
                render_info_panel_content(ui, state);
            });
    });
}

fn render_info_panel_content(ui: &mut Ui, state: &mut FftState) {
    ui.vertical(|ui| {
        ui.label(
            egui::RichText::new("Analysis")
                .size(10.0)
                .color(text_color()),
        );
        ui.add_space(4.0);

        if let Some(ref analysis) = state.analysis {
            if let Some(fund) = analysis.fundamental_frequency {
                info_row(ui, "Fund.", &format_freq(fund));
            }

            if let Some(fund_db) = analysis.fundamental_db {
                info_row(ui, "Level", &format!("{:.1} dB", fund_db));
            }

            if let Some(thd) = analysis.thd_percent {
                let color = if thd < 1.0 {
                    Color32::from_rgb(100, 200, 100)
                } else if thd < 5.0 {
                    Color32::from_rgb(200, 200, 100)
                } else {
                    Color32::from_rgb(255, 100, 100)
                };
                info_row_colored(ui, "THD", &format!("{:.3}%", thd), color);
            }

            if let Some(sfdr) = analysis.sfdr_db {
                info_row(ui, "SFDR", &format!("{:.1} dB", sfdr));
            }

            if let Some(snr) = analysis.snr_db {
                info_row(ui, "SNR", &format!("{:.1} dB", snr));
            }

            if let Some(sinad) = analysis.sinad_db {
                info_row(ui, "SINAD", &format!("{:.1} dB", sinad));
            }

            if let Some(noise) = analysis.noise_floor_db {
                info_row(ui, "Noise", &format!("{:.1} dB", noise));
            }

            info_row(ui, "Harmonics", &format!("{}", analysis.harmonics.len()));
        } else {
            ui.label(
                egui::RichText::new("No data")
                    .size(10.0)
                    .color(Color32::from_rgb(100, 105, 115)),
            );
        }

        ui.add_space(6.0);

        // Window info
        ui.label(egui::RichText::new("Window").size(10.0).color(text_color()));
        info_row(ui, "Type", state.window.display_name());
        info_row(ui, "Norm", state.normalization.display_name());
        info_row(ui, "Fidelity", state.input_fidelity.display_name());
        if !state.time_window_auto {
            info_row(
                ui,
                "Tstart",
                &crate::waveform::axis::format_time(state.time_window_start),
            );
            info_row(
                ui,
                "Tstop",
                &crate::waveform::axis::format_time(state.time_window_end),
            );
        }
        if !state.sample_count_auto {
            info_row(ui, "N set", &format!("{}", state.sample_count));
        }
        info_row(
            ui,
            "Sidelobe",
            &format!("{:.0} dB", state.window.sidelobe_level()),
        );
        info_row(
            ui,
            "ENBW",
            &format!("{:.2} bins", state.window.noise_bandwidth()),
        );

        if let Some(ref source) = state.source_cache {
            ui.add_space(6.0);
            ui.label(egui::RichText::new("Source").size(10.0).color(text_color()));
            info_row(ui, "Trace", &source.name);
            info_row(ui, "Input N", &format!("{}", source.original_count));
            info_row(ui, "Samples", &format!("{}", source.samples.len()));
            if source.decimation_factor > 1 {
                info_row(ui, "Decim", &format!("x{}", source.decimation_factor));
            }
            info_row(ui, "Fs", &format_freq(source.sample_rate));
        }

        ui.add_space(8.0);
        render_fft_markers_panel(ui, state);
    });
}

fn render_fft_markers_panel(ui: &mut Ui, state: &mut FftState) {
    ui.scope(|ui| {
        // Match waveform legend marker manager spacing exactly.
        ui.spacing_mut().item_spacing = egui::vec2(4.0, 4.0);
        ui.spacing_mut().interact_size.y = 18.0;

        ui.separator();
        ui.label(
            egui::RichText::new("Markers")
                .size(11.0)
                .strong()
                .color(Color32::from_rgb(160, 165, 175)),
        );

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("{}", state.marker_count()))
                    .size(9.0)
                    .color(Color32::from_rgb(120, 125, 135)),
            );
            ui.label(
                egui::RichText::new("entries")
                    .size(9.0)
                    .color(Color32::from_rgb(120, 125, 135)),
            );
            if ui.small_button("Clear").clicked() {
                state.clear_markers();
            }
        });
        ui.label(
            egui::RichText::new("Alt+LMB add, Alt+RMB remove")
                .size(9.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );

        if state.marker_frequencies.is_empty() {
            ui.label(
                egui::RichText::new("No markers")
                    .size(10.0)
                    .color(Color32::from_rgb(100, 105, 115)),
            );
            return;
        }

        let mut jump_to_freq: Option<f64> = None;
        let mut remove_idx: Option<usize> = None;
        let markers: Vec<f64> = state.marker_frequencies.clone();
        for (idx, marker_freq) in markers.iter().copied().enumerate() {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("M{}", idx + 1))
                        .size(10.0)
                        .color(marker_color_for_slot(idx)),
                );
                let mut jump_btn = ui.small_button(format_freq(marker_freq));
                if let Some(marker_mag) = state
                    .data
                    .as_ref()
                    .and_then(|data| data.interpolate(marker_freq))
                    .map(|point| format_marker_magnitude(state, &point))
                {
                    jump_btn = jump_btn.on_hover_text(format!(
                        "Center frequency view on marker\nMagnitude: {}",
                        marker_mag
                    ));
                } else {
                    jump_btn = jump_btn.on_hover_text("Center frequency view on marker");
                }
                if jump_btn.clicked() {
                    jump_to_freq = Some(marker_freq);
                }
                if ui
                    .small_button("x")
                    .on_hover_text("Delete marker")
                    .clicked()
                {
                    remove_idx = Some(idx);
                }
            });
        }

        if let Some(idx) = remove_idx {
            state.remove_marker_at(idx);
        }
        if let Some(freq) = jump_to_freq {
            center_fft_frequency_view_on_marker(state, freq);
        }
    });
}

fn center_fft_frequency_view_on_marker(state: &mut FftState, marker_freq: f64) {
    if !marker_freq.is_finite() {
        return;
    }
    state.freq_auto = false;
    match state.freq_scale {
        FrequencyScale::Linear => {
            let span = (state.freq_max - state.freq_min).max(1e-12);
            if !span.is_finite() {
                return;
            }
            let mut min = marker_freq - span * 0.5;
            let mut max = marker_freq + span * 0.5;
            if min < 0.0 {
                max -= min;
                min = 0.0;
            }
            if max <= min {
                max = min + span;
            }
            state.freq_min = min;
            state.freq_max = max;
        }
        FrequencyScale::Log => {
            if marker_freq <= 0.0 {
                return;
            }
            let min = state.freq_min.max(1e-12);
            let max = state.freq_max.max(min * 1.000_001);
            let span_log = (max.log10() - min.log10()).max(1e-9);
            let center_log = marker_freq.max(1e-12).log10();
            let half_span_log = span_log * 0.5;
            let min_log = center_log - half_span_log;
            let max_log = center_log + half_span_log;
            state.freq_min = 10.0_f64.powf(min_log).max(1e-12);
            state.freq_max = 10.0_f64.powf(max_log).max(state.freq_min * 1.000_001);
        }
    }
}

fn format_marker_magnitude(state: &FftState, point: &FftPoint) -> String {
    match state.mag_scale {
        MagnitudeScale::Linear => format!("{:.5}", state.display_magnitude(point)),
        MagnitudeScale::DB => format!("{:.2} dB", state.display_magnitude(point)),
        MagnitudeScale::DBc => format!("{:.2} dBc", state.display_magnitude(point)),
        MagnitudeScale::DBm => format!("{:.2} dBm", state.display_magnitude(point)),
    }
}

fn info_row(ui: &mut Ui, label: &str, value: &str) {
    info_row_colored(ui, label, value, Color32::from_rgb(200, 205, 215));
}

fn info_row_colored(ui: &mut Ui, label: &str, value: &str, color: Color32) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{}:", label))
                .size(10.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(value).size(11.0).color(color));
        });
    });
}
