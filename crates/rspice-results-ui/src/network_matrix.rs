//! Network-matrix controls, exact cell display and sampled diagnostic presentation.

use crate::presentation::with_table_state;
use egui::Ui;
use rspice_results::engine_evidence::{
    MAX_NETWORK_DIAGNOSTIC_PORTS, NoAbort, SampledPassivity, network_quality_with_abort,
};
use rspice_results::waveform::{HeldColumn, RetainedTrace, WaveformValues};
use rspice_results::{
    analysis_result::AnalysisResult,
    analysis_type::AnalysisType,
    network_matrix::{NetworkLayout, channel_label, channel_reference},
};
use rspice_ui_kit::icons::Icon;
use rspice_ui_kit::plot::fmt_si_significant;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::table::{Align, Cell, Column, DataTable};
use rspice_ui_kit::widgets::{Button, IconButton, SegmentedWidth, segmented, select};
use rspice_vocabulary::quantity::AngleDisplay;

/// Disposable local controls for one source-qualified matrix session.
#[derive(Debug, Clone)]
pub struct NetworkMatrixControls {
    pub block: usize,
    pub sample: usize,
    pub representation: usize,
    pub transpose: bool,
    /// The last passivity check, shown while its block and sample are.
    pub diagnostic: Option<NetworkDiagnostic>,
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

/// A sampled passivity and reciprocity check of one block at one frequency.
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkDiagnostic {
    pub block: usize,
    pub sample: usize,
    /// The verdict in one line, for under the table.
    pub line: String,
    /// Every number at full precision, for the line's hover.
    pub detail: String,
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

/// The coefficient formats, as the format control names them: the parts,
/// the magnitude and angle, and the magnitude in decibels and angle.
const FORMATS: [&str; 3] = ["Re/Im", "Mag/\u{2220}", "dB/\u{2220}"];
/// The gap between the bar's groups of controls.
const GROUP_GAP: f32 = 8.0;
/// The frequency stepper's arrow buttons.
const STEP_WIDTH: f32 = 22.0;
/// The room a select takes beyond its value: the inset and the chevron.
const SELECT_CHROME: f32 = 36.0;
/// The widest the representation select grows; a longer name is shortened
/// from the middle and named whole on hover.
const REPRESENTATION_MOST_WIDTH: f32 = 220.0;
/// The table's footer line: the heat-map key and the passivity verdict.
const FOOTER_HEIGHT: f32 = 22.0;
/// The heat-map key's gradient.
const LEGEND_RAMP: egui::Vec2 = egui::vec2(96.0, 8.0);

/// A stored column, once it has arrived off the frame.
fn held(values: &WaveformValues) -> Option<HeldColumn> {
    values.fetch_whole().ready()
}

/// One coefficient sample, unavailable until its column has arrived.
fn coefficient(values: &WaveformValues, sample: usize) -> f64 {
    held(values).map_or(f64::NAN, |values| {
        values.get(sample).copied().unwrap_or(f64::NAN)
    })
}

/// Why the passivity check cannot run on this matrix, if it cannot.
fn passivity_refusal(
    analysis: &AnalysisResult<RetainedTrace>,
    matrix: &NetworkLayout,
) -> Option<String> {
    if analysis.analysis_type != AnalysisType::SParameter {
        return Some(
            "This is a periodic conversion block. Passivity and reciprocity require the complete lifted network and its wave-frequency convention.".to_owned(),
        );
    }
    (matrix.references().len() > MAX_NETWORK_DIAGNOSTIC_PORTS).then(|| {
        format!("Interactive dense diagnostics support up to {MAX_NETWORK_DIAGNOSTIC_PORTS} ports")
    })
}

/// The sampled passivity and reciprocity of `block` at `sample`: one line
/// for under the table, and every number at full precision for its hover.
fn passivity(
    analysis: &AnalysisResult<RetainedTrace>,
    matrix: &NetworkLayout,
    block_index: usize,
    sample: usize,
    frequency: f64,
    significant_digits: usize,
) -> NetworkDiagnostic {
    let block = &matrix.blocks()[block_index];
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
                        coefficient(&complex.real, sample),
                        coefficient(&complex.imag, sample),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let at = fmt_si_significant(frequency, "Hz", significant_digits);
    let (line, detail) = match network_quality_with_abort(&values, 1e-10, &NoAbort) {
        Ok(quality) => {
            let verdict = match quality.passivity {
                SampledPassivity::Passive => "passive",
                SampledPassivity::Active => "active",
                SampledPassivity::Boundary => "at the passivity boundary",
            };
            (
                format!(
                    "At {at}: {verdict} · largest singular value {} · reciprocity residual {:.1e}",
                    fmt_si_significant(quality.largest_singular_value, "", significant_digits),
                    quality.reciprocity_residual
                ),
                format!(
                    "Sampled passivity: {verdict}\nLargest singular value: {:.17e}\nNormalized reciprocity residual: {:.17e}\nTolerance: {:.3e}\nFrequency: {frequency:.17e} Hz\nApplies only at this frequency.",
                    quality.largest_singular_value, quality.reciprocity_residual, quality.tolerance
                ),
            )
        }
        Err(error) => (
            format!("At {at}: the network check is unavailable"),
            format!("Network diagnostic unavailable: {error}"),
        ),
    };
    NetworkDiagnostic {
        block: block_index,
        sample,
        line,
        detail,
    }
}

/// The width `text` takes in `font` on one line.
fn text_width(ui: &Ui, text: &str, font: &egui::FontId) -> f32 {
    ui.painter()
        .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::PLACEHOLDER)
        .size()
        .x
}

/// The matrix's own controls in its sheet bar: which representation the
/// table holds, how a coefficient reads, which frequency sample it shows,
/// the Display menu and the sampled passivity check.
///
/// The representation leads the bar when there is more than one and the bar
/// has room for it beside the rest; otherwise it is the first thing in the
/// Display menu, so nothing wraps to a second row.
pub fn domain_bar(
    ui: &mut Ui,
    analysis: &AnalysisResult<RetainedTrace>,
    matrix: &NetworkLayout,
    controls: &mut NetworkMatrixControls,
    significant_digits: usize,
) {
    let blocks = matrix.blocks();
    controls.block = controls.block.min(blocks.len() - 1);
    let names = blocks.iter().map(|block| block.label()).collect::<Vec<_>>();
    let chooses = names.len() > 1;
    let sans = theme::sans(tokens::FS_0, FontWeight::Regular);
    let representation_width = (names
        .iter()
        .map(|name| text_width(ui, name, &sans))
        .fold(0.0, f32::max)
        + SELECT_CHROME)
        .min(REPRESENTATION_MOST_WIDTH);
    // What the rest of the bar took last frame. Its parts keep one width
    // whatever they show, so last frame's measure is this frame's.
    let rest_id = ui.id().with("rspice.results.network-matrix.bar-rest");
    let rest = ui.data(|data| data.get_temp::<f32>(rest_id));
    if chooses && rest.is_none() {
        ui.ctx()
            .request_discard("the matrix bar measures its controls before it lays them out");
    }
    let folded =
        chooses && ui.available_width() < representation_width + GROUP_GAP + rest.unwrap_or(0.0);
    if chooses && !folded {
        if let Some(picked) = select(
            ui,
            "rspice.results.network-matrix.representation",
            "Representation",
            &names[controls.block],
            &names,
            representation_width,
        ) {
            controls.block = picked;
        }
        ui.add_space(GROUP_GAP);
    }
    let start = ui.cursor().left();
    let block = &blocks[controls.block];
    let grid = held(&analysis.waveforms[block.cells()[0]].x);
    if let Some(grid) = grid.as_deref() {
        controls.sample = controls.sample.min(grid.len().saturating_sub(1));
    }
    segmented(
        ui,
        "rspice.results.network-matrix.format",
        &FORMATS,
        &mut controls.representation,
        SegmentedWidth::Natural,
    );
    ui.add_space(GROUP_GAP);
    frequency_stepper(ui, controls, grid.as_deref(), significant_digits);
    ui.add_space(GROUP_GAP);
    display_menu(ui, controls, folded.then_some(names.as_slice()));
    let refusal = passivity_refusal(analysis, matrix);
    let check = Button::new("Check passivity")
        .enabled(refusal.is_none() && grid.is_some())
        .show(ui);
    let check = match &refusal {
        Some(reason) => check.on_disabled_hover_text(reason),
        None => check.on_hover_text(
            "Passivity and reciprocity of the whole matrix at this frequency; the verdict is under the table",
        ),
    };
    if check.clicked()
        && let Some(grid) = grid.as_deref()
    {
        controls.diagnostic = Some(passivity(
            analysis,
            matrix,
            controls.block,
            controls.sample,
            grid[controls.sample],
            significant_digits,
        ));
    }
    let measured = check.rect.right() - start;
    ui.data_mut(|data| data.insert_temp(rest_id, measured));
}

/// The frequency sample as a stepper: the arrows move one sample, the
/// readout between them names the frequency and which sample of how many,
/// and dragging it scrubs the samples or a click types a sample number.
fn frequency_stepper(
    ui: &mut Ui,
    controls: &mut NetworkMatrixControls,
    grid: Option<&[f64]>,
    significant_digits: usize,
) {
    let count = grid.map_or(0, <[f64]>::len);
    let last = count.saturating_sub(1);
    let height = rspice_ui_kit::widgets::control_height(ui);
    if IconButton::new(Icon::ChevronLeft)
        .size(STEP_WIDTH, height)
        .enabled(count > 0 && controls.sample > 0)
        .tooltip("Previous frequency sample")
        .show(ui)
        .clicked()
    {
        controls.sample -= 1;
    }
    let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
    let readout = move |sample: usize| match grid {
        Some(grid) => format!(
            "{} · {} of {count}",
            fmt_si_significant(grid[sample.min(last)], "Hz", significant_digits),
            sample.min(last) + 1
        ),
        None => "Frequency pending".to_owned(),
    };
    // The widest frequency the reader's digits can print, so the readout
    // holds one width as it steps and nothing after it moves.
    let widest = format!(
        "{} · {count} of {count}",
        fmt_si_significant(8.888_888_888_888_89e8, "Hz", significant_digits)
    );
    let width = text_width(ui, &widest, &mono).max(text_width(ui, &readout(0), &mono))
        + 2.0 * ui.spacing().button_padding.x;
    ui.scope(|ui| {
        ui.spacing_mut().interact_size.x = width;
        ui.style_mut().override_font_id = Some(mono);
        ui.add_enabled(
            count > 0,
            egui::DragValue::new(&mut controls.sample)
                .range(0..=last)
                .speed(0.1)
                .custom_formatter(move |sample, _| readout(sample as usize))
                .custom_parser(|text| {
                    // Typed: a sample number, alone or as the readout's own
                    // "n of N".
                    let number = text
                        .split(" of ")
                        .next()?
                        .rsplit(|c: char| c.is_whitespace() || c == '·')
                        .find(|part| !part.is_empty())?;
                    number.parse::<f64>().ok().map(|sample| sample - 1.0)
                }),
        )
        .on_hover_text("Frequency sample: drag to scrub, or click and type a sample number");
    });
    if IconButton::new(Icon::ChevronRight)
        .size(STEP_WIDTH, height)
        .enabled(controls.sample < last)
        .tooltip("Next frequency sample")
        .show(ui)
        .clicked()
    {
        controls.sample += 1;
    }
}

/// How the table is laid out and washed: transposed or not, the magnitude
/// heat map and its floor, and the representation when the bar had no room
/// for it.
fn display_menu(
    ui: &mut Ui,
    controls: &mut NetworkMatrixControls,
    representations: Option<&[String]>,
) {
    let display = Button::new("Display")
        .menu()
        .accessible_label("Display options")
        .show(ui);
    egui::Popup::menu(&display)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            if let Some(names) = representations {
                crate::chrome::bars::bar_label(ui, "Representation");
                for (index, name) in names.iter().enumerate() {
                    if ui.radio(controls.block == index, name).clicked() {
                        controls.block = index;
                    }
                }
                ui.separator();
            }
            ui.checkbox(&mut controls.transpose, "Transpose")
                .on_hover_text("Rows read the incident waves and columns the outgoing ones");
            ui.checkbox(&mut controls.heatmap, "Heat map")
                .on_hover_text("Wash each coefficient by its magnitude in dB");
            ui.add_enabled_ui(controls.heatmap, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Heat-map floor");
                    ui.add(
                        egui::DragValue::new(&mut controls.floor_db)
                            .range(-240.0..=-10.0)
                            .speed(1.0)
                            .suffix(" dB"),
                    )
                    .on_hover_text("The magnitude the wash fades out at; 0 dB is full");
                });
            });
        });
}

/// The heat map's key, compact in the table's footer: the floor, a ramp, the
/// top of the scale and the color a gain above it takes.
fn heat_legend(ui: &mut Ui, floor: f64, palette: &rspice_ui_kit::palette::Palette) {
    let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
    let dim = palette.text_dim;
    let response = ui
        .horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            ui.label(
                egui::RichText::new(fmt_si_significant(floor, "dB", 4))
                    .font(mono.clone())
                    .color(dim),
            );
            let (rect, _) = ui.allocate_exact_size(LEGEND_RAMP, egui::Sense::hover());
            for index in 0..48 {
                let fraction = index as f64 / 47.0;
                let swatch = egui::Rect::from_min_max(
                    egui::pos2(rect.left() + rect.width() * index as f32 / 48.0, rect.top()),
                    egui::pos2(
                        rect.left() + rect.width() * (index + 1) as f32 / 48.0,
                        rect.bottom(),
                    ),
                );
                ui.painter().rect_filled(
                    swatch,
                    0.0,
                    heat_color(floor * (1.0 - fraction), floor, palette),
                );
            }
            ui.label(egui::RichText::new("0 dB").font(mono.clone()).color(dim));
            let (key, _) = ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
            ui.painter()
                .rect_filled(key, 0.0, heat_color(1.0, floor, palette));
            ui.label(egui::RichText::new("> 0 dB").font(mono).color(dim));
        })
        .response;
    response.on_hover_text(
        "Each coefficient is washed by its magnitude. The color clips at the floor and at 0 dB; the values remain exact.",
    );
}

/// Draw a validated, aligned matrix: the table, and under it the heat map's
/// key and the passivity verdict at this frequency.
pub fn show(
    ui: &mut Ui,
    analysis: &AnalysisResult<RetainedTrace>,
    matrix: &NetworkLayout,
    controls: &mut NetworkMatrixControls,
    angle_display: AngleDisplay,
    significant_digits: usize,
) -> Option<OpenCoefficient> {
    let mut open_coefficient = None;
    let tokens = Tokens::get(ui.ctx());
    controls.block = controls.block.min(matrix.blocks().len() - 1);
    let block = &matrix.blocks()[controls.block];
    // A stored matrix is read off the frame; it is drawn once it has arrived.
    let grid = held(&analysis.waveforms[block.cells()[0]].x)?;
    controls.sample = controls.sample.min(grid.len() - 1);
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
    // The corner names what the rows and the columns hold, so no caption
    // line restates it above the table.
    let (corner, axes) = if controls.transpose {
        (
            "In \u{2193} / Out \u{2192}",
            "Rows: incident waves · Columns: outgoing waves",
        )
    } else {
        (
            "Out \u{2193} / In \u{2192}",
            "Rows: outgoing waves · Columns: incident waves",
        )
    };
    let mut corner_hint = format!("{axes}\nEach wave channel names its reference impedance.");
    if mixed {
        corner_hint.push_str("\nAdjacent physical ports form (+, −) pairs. Differential and common power waves use (a+ − a−)/√2 and (a+ + a−)/√2.");
    }
    // A matrix has no order to sort: its rows and columns are its ports.
    // The row names keep the width they need; the port columns share the
    // rest equally, so every coefficient cell is the same size.
    let mut columns = vec![
        Column::words(corner, 136.0)
            .unsortable()
            .tooltip(corner_hint),
    ];
    columns.extend((0..ports).map(|column| {
        Column::name(channel(column), 150.0)
            .align(Align::Right)
            .unsortable()
    }));
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
    let verdict = controls
        .diagnostic
        .as_ref()
        .filter(|diagnostic| {
            diagnostic.block == controls.block && diagnostic.sample == controls.sample
        })
        .cloned();
    let footer = controls.heatmap || verdict.is_some();
    let table_height = if footer {
        (ui.available_height() - FOOTER_HEIGHT - ui.spacing().item_spacing.y).max(0.0)
    } else {
        ui.available_height()
    };
    let (response, chosen) = with_table_state(ui, "network-matrix", |ui, state| {
        let response = DataTable::new("rspice.results.network-matrix", &columns, ports)
            .state(state)
            .freeze(1)
            .cell_selection()
            .max_height(table_height)
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
    if footer {
        let width = ui.available_width();
        ui.allocate_ui_with_layout(
            egui::vec2(width, FOOTER_HEIGHT),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.add_space(8.0);
                if controls.heatmap {
                    heat_legend(ui, controls.floor_db, &tokens.color);
                    ui.add_space(16.0);
                }
                if let Some(verdict) = &verdict {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(&verdict.line)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(tokens.color.text),
                        )
                        .truncate()
                        .show_tooltip_when_elided(false),
                    )
                    .on_hover_text(&verdict.detail);
                }
            },
        );
    }
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
