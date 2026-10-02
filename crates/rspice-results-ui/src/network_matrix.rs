//! Network-matrix controls, exact cell display and sampled diagnostic presentation.

use crate::waveform::WaveformData;
use egui::Ui;
use rspice_results::{
    analysis_result::AnalysisResult,
    analysis_type::AnalysisType,
    network_matrix::{NetworkLayout, channel_label, channel_reference},
};

/// Disposable local controls for one source-qualified matrix session.
#[derive(Debug, Clone)]
pub struct NetworkMatrixControls {
    pub block: usize,
    pub sample: usize,
    pub representation: usize,
    pub transpose: bool,
    pub diagnostic: Option<(usize, usize, String)>,
    pub heatmap: bool,
    pub floor_db: f64,
}

impl Default for NetworkMatrixControls {
    fn default() -> Self {
        Self {
            block: 0,
            sample: 0,
            representation: 0,
            transpose: false,
            diagnostic: None,
            heatmap: true,
            floor_db: -80.0,
        }
    }
}

impl NetworkMatrixControls {
    /// Clear source-bound choices while preserving display preferences.
    pub fn reset_source(&mut self) {
        self.block = 0;
        self.sample = 0;
        self.diagnostic = None;
    }
}

/// A retained coefficient the host should open from the matrix's bound analysis.
pub struct OpenCoefficient {
    pub waveform_name: String,
    pub frequency: f64,
}

pub fn right_panel(ui: &mut Ui) {
    crate::presentation::panel_note(
        ui,
        "Exact retained power-wave coefficients. Select a frequency and sideband block in the matrix. Hover or copy for full numerical precision.",
    );
}

pub fn formatted(real: f64, imaginary: f64, representation: usize) -> String {
    if real == 0.0 && imaginary == 0.0 && representation != 0 {
        return if representation == 2 {
            "−∞ dB · phase undefined"
        } else {
            "0 · phase undefined"
        }
        .to_owned();
    }
    match representation {
        1 => format!(
            "{:.8e} ∠ {:.6}°",
            real.hypot(imaginary),
            imaginary.atan2(real).to_degrees()
        ),
        2 => {
            let magnitude = real.hypot(imaginary);
            if magnitude == 0.0 {
                "−∞ dB · phase undefined".to_owned()
            } else {
                format!(
                    "{:.6} dB ∠ {:.6}°",
                    magnitude_db(real, imaginary),
                    imaginary.atan2(real).to_degrees()
                )
            }
        }
        _ => format!("{real:.8e} {imaginary:+.8e}j"),
    }
}

/// Avoid overflowing the magnitude before taking its logarithm.
pub fn magnitude_db(real: f64, imaginary: f64) -> f64 {
    let scale = real.abs().max(imaginary.abs());
    if scale == 0.0 {
        f64::NEG_INFINITY
    } else {
        20.0 * (scale.log10() + (real / scale).hypot(imaginary / scale).log10())
    }
}

fn heat_color(db: f64, floor: f64, palette: &rspice_ui_kit::palette::Palette) -> egui::Color32 {
    if db > 0.0 {
        return palette.warn;
    }
    let amount = ((db - floor) / -floor).clamp(0.0, 1.0) as f32;
    egui::Color32::from(
        egui::Rgba::from(palette.bg_inset) * (1.0 - amount)
            + egui::Rgba::from(palette.info) * amount,
    )
}

fn heat_legend(ui: &mut Ui, floor: &mut f64, palette: &rspice_ui_kit::palette::Palette) {
    ui.horizontal_wrapped(|ui| {
        ui.label("Magnitude color scale");
        ui.add(
            egui::DragValue::new(floor)
                .range(-240.0..=-10.0)
                .speed(1.0)
                .suffix(" dB"),
        );
        let (rect, _) = ui.allocate_exact_size(egui::vec2(140.0, 12.0), egui::Sense::hover());
        for index in 0..64 {
            let fraction = index as f64 / 63.0;
            let swatch = egui::Rect::from_min_max(
                egui::pos2(rect.left() + rect.width() * index as f32 / 64.0, rect.top()),
                egui::pos2(
                    rect.left() + rect.width() * (index + 1) as f32 / 64.0,
                    rect.bottom(),
                ),
            );
            ui.painter().rect_filled(
                swatch,
                0.0,
                heat_color(*floor * (1.0 - fraction), *floor, palette),
            );
        }
        ui.label("0 dB");
        ui.colored_label(palette.warn, "> 0 dB");
        ui.label("Color clips at limits; values remain exact.");
    });
}

/// Draw a validated, aligned matrix; the host supplies its existing exact exporter.
pub fn show(
    ui: &mut Ui,
    analysis: &AnalysisResult<WaveformData>,
    matrix: &NetworkLayout,
    controls: &mut NetworkMatrixControls,
    mut copy_exact: impl FnMut(usize, usize) -> String,
) -> Option<OpenCoefficient> {
    let mut open_coefficient = None;
    let tokens = rspice_ui_kit::tokens::Tokens::get(ui.ctx());
    controls.block = controls.block.min(matrix.blocks().len() - 1);
    ui.horizontal_wrapped(|ui| {
        egui::ComboBox::from_id_salt("network-matrix-block")
            .selected_text(matrix.blocks()[controls.block].label())
            .show_ui(ui, |ui| {
                for (index, block) in matrix.blocks().iter().enumerate() {
                    ui.selectable_value(&mut controls.block, index, block.label());
                }
            });
        for (index, label) in ["Real / imaginary", "Magnitude / phase", "dB / phase"]
            .iter()
            .enumerate()
        {
            ui.selectable_value(&mut controls.representation, index, *label);
        }
        ui.checkbox(&mut controls.transpose, "Transpose display");
        ui.checkbox(&mut controls.heatmap, "Magnitude heat map");
    });
    if controls.heatmap {
        heat_legend(ui, &mut controls.floor_db, &tokens.color);
    }
    let block = &matrix.blocks()[controls.block];
    let grid = &analysis.waveforms[block.cells()[0]].x;
    controls.sample = controls.sample.min(grid.len() - 1);
    ui.horizontal_wrapped(|ui| {
        ui.label("Frequency sample");
        ui.add(egui::Slider::new(&mut controls.sample, 0..=grid.len() - 1).show_value(false));
        ui.add(
            egui::DragValue::new(&mut controls.sample)
                .range(0..=grid.len() - 1)
                .speed(1),
        );
        ui.label(format!(
            "/ {} · {:.17e} Hz",
            grid.len() - 1,
            grid[controls.sample]
        ));
        if ui.button("Copy exact matrix").clicked() {
            ui.ctx()
                .copy_text(copy_exact(controls.block, controls.sample));
        }
    });
    ui.label(if controls.transpose {
        "Rows: incident waves · Columns: outgoing waves"
    } else {
        "Rows: outgoing waves · Columns: incident waves"
    });
    if block.is_mixed() {
        ui.label("Adjacent physical ports form (+, −) pairs. Differential and common power waves use (a+ − a−)/√2 and (a+ + a−)/√2.");
    }
    if analysis.analysis_type == AnalysisType::SParameter {
        if ui
            .add_enabled(
                matrix.references().len()
                    <= rspice_core::analysis::s_param::MAX_NETWORK_DIAGNOSTIC_PORTS,
                egui::Button::new("Check sampled passivity / reciprocity"),
            )
            .on_disabled_hover_text("Interactive dense diagnostics support up to 128 ports")
            .clicked()
        {
            let ports = matrix.references().len();
            let values = (0..ports)
                .map(|row| {
                    (0..ports)
                        .map(|column| {
                            let complex = analysis.waveforms[block.cells()[row * ports + column]]
                                .complex
                                .as_ref()
                                .expect("resolved coefficient");
                            num_complex::Complex64::new(
                                complex.real[controls.sample],
                                complex.imag[controls.sample],
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let result = rspice_core::analysis::s_param::network_quality_with_abort(
                &values,
                1e-10,
                &rspice_core::abort_signal::NoAbort,
            );
            let text = match result {
                Ok(quality) => format!(
                    "Sampled passivity: {:?} · largest singular value {:.17e} · normalized reciprocity residual {:.17e} · tolerance {:.3e}. Applies only at this frequency.",
                    quality.passivity,
                    quality.largest_singular_value,
                    quality.reciprocity_residual,
                    quality.tolerance
                ),
                Err(error) => format!("Network diagnostic unavailable: {error}"),
            };
            controls.diagnostic = Some((controls.block, controls.sample, text));
        }
        if let Some((block, sample, text)) = &controls.diagnostic
            && *block == controls.block
            && *sample == controls.sample
        {
            ui.label(text);
        }
    } else {
        ui.label("This is a periodic conversion block. Passivity and reciprocity require the complete lifted network and its wave-frequency convention.");
    }
    let ports = matrix.references().len();
    let row_height = ui.spacing().interact_size.y.max(32.0);
    egui::ScrollArea::horizontal().id_salt("network-matrix-scroll").auto_shrink([false, false]).show(ui, |ui| {
        egui_extras::TableBuilder::new(ui).id_salt("network-matrix-cells").striped(true)
            .columns(egui_extras::Column::exact(240.0), ports + 1)
            .header(row_height, |mut header| {
                header.col(|ui| { ui.strong("Wave channel / reference"); });
                for column in 0..ports {
                    header.col(|ui| { ui.strong(format!("{} · {:.8e} Ω", channel_label(column, ports, block.is_mixed()), channel_reference(column, matrix.references(), block.is_mixed()))); });
                }
            }).body(|body| body.rows(row_height, ports, |mut table_row| {
                let row = table_row.index();
                table_row.col(|ui| { ui.strong(format!("{} · {:.8e} Ω", channel_label(row, ports, block.is_mixed()), channel_reference(row, matrix.references(), block.is_mixed()))); });
                for column in 0..ports {
                    table_row.col(|ui| {
                    let (out, input) = if controls.transpose { (column, row) } else { (row, column) };
                    let waveform = &analysis.waveforms[block.cells()[out * ports + input]];
                    let complex = waveform.complex.as_ref().expect("resolved complex coefficient");
                    let real = complex.real[controls.sample];
                    let imaginary = complex.imag[controls.sample];
                    let label = formatted(real, imaginary, controls.representation);
                    if controls.heatmap {
                        let color = heat_color(magnitude_db(real, imaginary), controls.floor_db, &tokens.color);
                        let rect = ui.max_rect().shrink(2.0);
                        ui.painter().rect_filled(rect, tokens.radius, color.gamma_multiply(0.22));
                        ui.painter().rect_filled(egui::Rect::from_min_size(rect.min, egui::vec2(3.0, rect.height())), 0.0, color);
                    }
                    if ui.selectable_label(false, egui::RichText::new(label).monospace().color(tokens.color.text)).on_hover_text(format!("{}\nReal: {real:.17e}\nImaginary: {imaginary:.17e}\nFrequency: {:.17e} Hz\nOpen this coefficient in Polar", complex.source_name, grid[controls.sample])).clicked() {
                        open_coefficient = Some(OpenCoefficient {
                            waveform_name: waveform.name.clone(),
                            frequency: grid[controls.sample],
                        });
                    }
                    });
                }
            }));
    });
    open_coefficient
}
