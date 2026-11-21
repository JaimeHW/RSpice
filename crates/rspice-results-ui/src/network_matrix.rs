//! Network-matrix controls, exact cell display and sampled diagnostic presentation.

use crate::presentation::with_table_state;
use egui::Ui;
use rspice_results::engine_evidence::{
    MAX_NETWORK_DIAGNOSTIC_PORTS, NoAbort, network_quality_with_abort,
};
use rspice_results::waveform::RetainedTrace;
use rspice_results::{
    analysis_result::AnalysisResult,
    analysis_type::AnalysisType,
    network_matrix::{NetworkLayout, channel_label, channel_reference},
};
use rspice_ui_kit::plot::fmt_si_significant;
use rspice_ui_kit::widgets::table::{Cell, Column, DataTable};
use rspice_vocabulary::quantity::AngleDisplay;

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

/// One coefficient as the representation shows it, its phase in the unit
/// Angle display chose.
pub fn formatted(
    real: f64,
    imaginary: f64,
    representation: usize,
    angle_display: AngleDisplay,
    significant_digits: usize,
) -> String {
    let digits = significant_digits;
    if real == 0.0 && imaginary == 0.0 && representation != 0 {
        return if representation == 2 {
            "−∞ dB · phase undefined"
        } else {
            "0 · phase undefined"
        }
        .to_owned();
    }
    let phase = || {
        let radians = imaginary.atan2(real);
        match angle_display {
            AngleDisplay::Degrees => fmt_si_significant(radians.to_degrees(), "°", digits),
            AngleDisplay::Radians => fmt_si_significant(radians, "rad", digits),
        }
    };
    match representation {
        1 => format!(
            "{} ∠ {}",
            fmt_si_significant(real.hypot(imaginary), "", digits),
            phase()
        ),
        2 => {
            let magnitude = real.hypot(imaginary);
            if magnitude == 0.0 {
                "−∞ dB · phase undefined".to_owned()
            } else {
                format!(
                    "{} ∠ {}",
                    fmt_si_significant(magnitude_db(real, imaginary), "dB", digits),
                    phase()
                )
            }
        }
        _ => crate::presentation::complex_readout(real, imaginary, "", digits),
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
        // The key is a cell of the colour a gain above 0 dB paints, beside
        // neutral words.
        let (key, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
        ui.painter()
            .rect_filled(key, 0.0, heat_color(1.0, *floor, palette));
        ui.label("> 0 dB");
        ui.label("Color clips at limits; values remain exact.");
    });
}

/// Draw a validated, aligned matrix; the host supplies its existing exact exporter.
pub fn show(
    ui: &mut Ui,
    analysis: &AnalysisResult<RetainedTrace>,
    matrix: &NetworkLayout,
    controls: &mut NetworkMatrixControls,
    angle_display: AngleDisplay,
    significant_digits: usize,
    mut copy_exact: impl FnMut(usize, usize) -> String,
) -> Option<OpenCoefficient> {
    let mut open_coefficient = None;
    let tokens = rspice_ui_kit::tokens::Tokens::get(ui.ctx());
    controls.block = controls.block.min(matrix.blocks().len() - 1);
    ui.horizontal_wrapped(|ui| {
        rspice_ui_kit::widgets::Combo::from_id_salt("network-matrix-block")
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
    // A stored matrix is read off the frame; it is drawn once it has arrived.
    let held = |values: &rspice_results::waveform::WaveformValues| values.fetch_whole().ready();
    let grid = held(&analysis.waveforms[block.cells()[0]].x)?;
    // One coefficient sample, unavailable until its column has arrived.
    let coefficient = |values: &rspice_results::waveform::WaveformValues, sample: usize| {
        held(values).map_or(f64::NAN, |values| {
            values.get(sample).copied().unwrap_or(f64::NAN)
        })
    };
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
            "/ {} · {}",
            grid.len() - 1,
            fmt_si_significant(grid[controls.sample], "Hz", significant_digits)
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
                matrix.references().len() <= MAX_NETWORK_DIAGNOSTIC_PORTS,
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
                                coefficient(&complex.real, controls.sample),
                                coefficient(&complex.imag, controls.sample),
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let result = network_quality_with_abort(&values, 1e-10, &NoAbort);
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
    let mixed = block.is_mixed();
    let channel = |index: usize| {
        format!(
            "{} · {}",
            channel_label(index, ports, mixed),
            fmt_si_significant(
                channel_reference(index, matrix.references(), mixed),
                "Ω",
                significant_digits
            )
        )
    };
    // A matrix has no order to sort: its rows and columns are its ports.
    let mut columns = vec![Column::name("Wave channel / reference", 200.0).unsortable()];
    columns.extend((0..ports).map(|column| Column::number(channel(column), 200.0).unsortable()));
    let entry = |row: usize, column: usize| {
        let (out, input) = if controls.transpose {
            (column, row)
        } else {
            (row, column)
        };
        let waveform = &analysis.waveforms[block.cells()[out * ports + input]];
        let complex = waveform
            .complex
            .as_ref()
            .expect("resolved complex coefficient");
        (
            waveform,
            complex,
            coefficient(&complex.real, controls.sample),
            coefficient(&complex.imag, controls.sample),
        )
    };
    let frequency = grid[controls.sample];
    let (response, chosen) = with_table_state(ui, "network-matrix", |ui, state| {
        let response = DataTable::new("rspice.results.network-matrix", &columns, ports)
            .state(state)
            .freeze(1)
            .cell_selection()
            .max_height(ui.available_height())
            .show(ui, |row| {
                let mut cells = vec![Cell::text(channel(row))];
                cells.extend((0..ports).map(|column| {
                    let (_, complex, real, imaginary) = entry(row, column);
                    let mut cell = Cell::number(formatted(
                        real,
                        imaginary,
                        controls.representation,
                        angle_display,
                        significant_digits,
                    ))
                    .hint(format!(
                        "{}\nReal: {real:.17e}\nImaginary: {imaginary:.17e}\nFrequency: {frequency:.17e} Hz\nDouble-click or press Enter to open this coefficient in Polar",
                        complex.source_name
                    ));
                    if controls.heatmap {
                        let color = heat_color(
                            magnitude_db(real, imaginary),
                            controls.floor_db,
                            &tokens.color,
                        );
                        cell = cell.wash(color.gamma_multiply(0.22));
                    }
                    cell
                }));
                cells
            });
        (response, state.selected_column())
    });
    // A click selects a coefficient to copy; a double-click, or Enter on
    // the selected one, opens it.
    let opened = response
        .double_clicked
        .and_then(|cell| Some((cell.row, cell.column?)))
        .or_else(|| Some((response.activate?, chosen?)))
        .filter(|(_, column)| *column > 0);
    if let Some((row, column)) = opened {
        let (waveform, ..) = entry(row, column - 1);
        open_coefficient = Some(OpenCoefficient {
            waveform_name: waveform.name.clone(),
            frequency,
        });
    }
    open_coefficient
}
