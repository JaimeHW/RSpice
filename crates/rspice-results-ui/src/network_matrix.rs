//! Network-matrix controls, exact cell display and sampled diagnostic presentation.

use crate::presentation::with_table_state;
use egui::Ui;
use rspice_results::engine_evidence::{
    MAX_NETWORK_DIAGNOSTIC_PORTS, NoAbort, SampledPassivity, network_quality_with_abort,
};
use rspice_results::number_format::SharedReading;
use rspice_results::waveform::{HeldColumn, RetainedTrace, WaveformValues};
use rspice_results::{
    analysis_result::AnalysisResult,
    analysis_type::AnalysisType,
    network_matrix::{
        MatrixBlock, NetworkLayout, channel_label, channel_reference,
        forms::{self, NetworkForm},
    },
};
use rspice_ui_kit::icons::Icon;
use rspice_ui_kit::plot::{fmt_column_reading, fmt_si_significant};
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
    /// The parameter set the table reads in.
    pub form: NetworkForm,
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
            form: NetworkForm::S,
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

/// One coefficient in `unit`, read on its own, as the representation shows
/// it: [`MatrixReading::cell`] of a matrix of one.
pub fn formatted(
    real: f64,
    imaginary: f64,
    unit: &str,
    representation: usize,
    angle_display: AngleDisplay,
    significant_digits: usize,
) -> String {
    MatrixReading::of(
        [(unit, real, imaginary)],
        representation,
        significant_digits,
    )
    .cell(real, imaginary, unit, representation, angle_display)
}

/// How one matrix's coefficients read together at one sample, so the matrix
/// reads with one count of decimals in every cell and both parts.
///
/// The coefficients of one unit take one resolution and one prefix, the
/// largest part's at the sheet's digits; where they span more than four
/// decades, which would leave the smallest no digit at that resolution, each
/// part reads in engineering notation at those digits instead. Phases in
/// radians read at the largest phase's resolution. Decibels and degrees read
/// to the hundredth whatever the matrix holds.
#[derive(Debug, Clone)]
pub struct MatrixReading<'a> {
    coefficients: Vec<(&'a str, SharedReading)>,
    radians: SharedReading,
    significant_digits: usize,
}

impl<'a> MatrixReading<'a> {
    /// The reading of `cells`, each its unit and its real and imaginary
    /// parts, as `representation` shows them.
    pub fn of(
        cells: impl IntoIterator<Item = (&'a str, f64, f64)>,
        representation: usize,
        significant_digits: usize,
    ) -> Self {
        // Each unit's largest part and smallest nonzero magnitude.
        let mut spans: Vec<(&'a str, f64, f64)> = Vec::new();
        let mut largest_phase = 0.0_f64;
        for (unit, real, imaginary) in cells {
            if !(real.is_finite() && imaginary.is_finite()) {
                continue;
            }
            let magnitude = real.hypot(imaginary);
            let part = if representation == 0 {
                real.abs().max(imaginary.abs())
            } else {
                magnitude
            };
            let index = spans
                .iter()
                .position(|(held, ..)| *held == unit)
                .unwrap_or_else(|| {
                    spans.push((unit, 0.0, f64::INFINITY));
                    spans.len() - 1
                });
            let span = &mut spans[index];
            span.1 = span.1.max(part);
            if magnitude > 0.0 {
                span.2 = span.2.min(magnitude);
                largest_phase = largest_phase.max(imaginary.atan2(real).abs());
            }
        }
        Self {
            coefficients: spans
                .into_iter()
                .map(|(unit, largest, smallest)| {
                    (
                        unit,
                        SharedReading::new(largest, smallest, unit, significant_digits),
                    )
                })
                .collect(),
            // A phase is bounded by a half turn, so it reads at one
            // resolution, as a degree reads to the hundredth.
            radians: SharedReading::new(largest_phase, 0.0, "rad", significant_digits),
            significant_digits,
        }
    }

    /// One coefficient in `unit` as the representation shows it, its phase
    /// in the unit Angle display chose. A coefficient with a unit reads its
    /// decibels against one of that unit: dBΩ, dBS.
    pub fn cell(
        &self,
        real: f64,
        imaginary: f64,
        unit: &str,
        representation: usize,
        angle_display: AngleDisplay,
    ) -> String {
        let digits = self.significant_digits;
        let decibels = format!("dB{unit}");
        if real == 0.0 && imaginary == 0.0 && representation != 0 {
            return if representation == 2 {
                format!("−∞ {decibels} · phase undefined")
            } else {
                "0 · phase undefined".to_owned()
            };
        }
        let shared = self
            .coefficients
            .iter()
            .find(|(held, _)| *held == unit)
            .map_or_else(
                || {
                    let alone = real.abs().max(imaginary.abs());
                    SharedReading::new(alone, alone, unit, digits)
                },
                |(_, shared)| *shared,
            );
        let phase = || {
            let radians = imaginary.atan2(real);
            match angle_display {
                AngleDisplay::Degrees => fmt_column_reading(radians.to_degrees(), "°", digits),
                AngleDisplay::Radians => self.radians.reading(radians, "rad"),
            }
        };
        match representation {
            1 => format!(
                "{} ∠ {}",
                shared.reading(real.hypot(imaginary), unit),
                phase()
            ),
            2 => format!(
                "{} ∠ {}",
                fmt_column_reading(magnitude_db(real, imaginary), &decibels, digits),
                phase()
            ),
            _ => shared.complex(real, imaginary, unit),
        }
    }
}

/// The forms a block can be read in: every block in S; a single-ended or
/// mixed-mode block also in Y and Z, and a two-port in H and ABCD. A periodic
/// conversion block is not an ordinary network, so it reads in S only.
pub fn offered_forms(block: &MatrixBlock, channels: usize) -> Vec<NetworkForm> {
    if block.is_conversion() {
        return vec![NetworkForm::S];
    }
    NetworkForm::ALL
        .into_iter()
        .filter(|form| form.applies_to(channels))
        .collect()
}

/// The block's coefficients at `sample` read in `form`, row-major: `None`
/// while a column has not arrived or when the network has no such form.
pub fn form_matrix(
    analysis: &AnalysisResult<RetainedTrace>,
    matrix: &NetworkLayout,
    block: &MatrixBlock,
    form: NetworkForm,
    sample: usize,
) -> Option<Vec<num_complex::Complex64>> {
    let ports = matrix.references().len();
    let scattering = block
        .cells()
        .iter()
        .map(|&index| {
            let complex = analysis.waveforms[index].complex.as_ref()?;
            let value = num_complex::Complex64::new(
                coefficient(&complex.real, sample),
                coefficient(&complex.imag, sample),
            );
            (value.re.is_finite() && value.im.is_finite()).then_some(value)
        })
        .collect::<Option<Vec<_>>>()?;
    let references = (0..ports)
        .map(|index| channel_reference(index, matrix.references(), block.is_mixed()))
        .collect::<Vec<_>>();
    forms::convert(&scattering, &references, form)
}

/// Why `form` has no matrix for this block at this frequency.
fn form_absence(form: NetworkForm) -> &'static str {
    match form {
        NetworkForm::Y => "no admittance matrix at this frequency (I + S is singular)",
        NetworkForm::Z => "no impedance matrix at this frequency (I − S is singular)",
        NetworkForm::H => "no hybrid matrix at this frequency",
        NetworkForm::Abcd => "no chain matrix at this frequency (S21 is zero)",
        NetworkForm::S => "the coefficients have not arrived",
    }
}

/// A frequency as a reader types it: a number, an SI prefix as the readout
/// prints them (`k`, `M` for mega, `G`, `m` for milli, `u` or `µ`), and `Hz`
/// if they like. `meg` reads as mega too, as a deck writes it.
pub fn parse_frequency(text: &str) -> Option<f64> {
    let compact: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    let body = compact
        .strip_suffix("Hz")
        .or_else(|| compact.strip_suffix("hz"))
        .or_else(|| compact.strip_suffix("HZ"))
        .unwrap_or(&compact);
    let split = body
        .char_indices()
        .find(|(_, c)| !(c.is_ascii_digit() || matches!(c, '.' | '-' | '+' | 'e' | 'E')))
        .map_or(body.len(), |(index, _)| index);
    let (number, prefix) = body.split_at(split);
    let scale = match prefix {
        "" => 1.0,
        "T" => 1e12,
        "G" => 1e9,
        "M" | "meg" | "Meg" | "MEG" => 1e6,
        "k" | "K" => 1e3,
        "m" => 1e-3,
        "u" | "µ" | "μ" => 1e-6,
        "n" => 1e-9,
        "p" => 1e-12,
        _ => return None,
    };
    let value = number.parse::<f64>().ok()? * scale;
    value.is_finite().then_some(value)
}

/// The sample of the ascending `grid` nearest `frequency`.
pub fn nearest_sample(grid: &[f64], frequency: f64) -> usize {
    let above = grid.partition_point(|value| *value < frequency);
    match (above.checked_sub(1), grid.get(above)) {
        (Some(below), Some(next)) if frequency - grid[below] <= next - frequency => below,
        (Some(below), None) => below,
        _ => above.min(grid.len().saturating_sub(1)),
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
            // Reciprocity is judged at the same tolerance as passivity.
            let reciprocity = if quality.reciprocity_residual <= quality.tolerance {
                "reciprocal"
            } else {
                "not reciprocal"
            };
            (
                format!(
                    "At {at}: {verdict} (largest singular value {}), {reciprocity} (residual {})",
                    fmt_si_significant(quality.largest_singular_value, "", significant_digits),
                    fmt_si_significant(quality.reciprocity_residual, "", significant_digits)
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
/// The representation leads the bar when there is more than one, and the
/// form follows it as a segmented control. When the bar is short of room the
/// form gives way first, to a select that still shows it; then the
/// representation moves to the head of the Display menu, so nothing wraps to
/// a second row or runs under the Export button.
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
    let forms = offered_forms(&blocks[controls.block], matrix.references().len()).len();
    let spacing = ui.spacing().item_spacing.x;
    let form_select_width = NetworkForm::ALL
        .iter()
        .map(|form| text_width(ui, form.label(), &sans))
        .fold(0.0, f32::max)
        + SELECT_CHROME;
    // What the form took as a segmented control when last drawn so, and what
    // the rest of the bar took last frame. Each keeps one width whatever it
    // shows, so the last measure is this frame's.
    let measures_id = ui
        .id()
        .with(("rspice.results.network-matrix.bar-measures", forms));
    let measures = ui.data(|data| data.get_temp::<(f32, f32)>(measures_id));
    if (chooses || forms > 1) && measures.is_none() {
        ui.ctx()
            .request_discard("the matrix bar measures its controls before it lays them out");
    }
    let (segmented_form, rest) = measures.unwrap_or((0.0, 0.0));
    let available = ui.available_width();
    let representation = if chooses {
        representation_width + spacing + GROUP_GAP
    } else {
        0.0
    };
    let (form_wide, form_narrow) = if forms > 1 {
        (
            segmented_form + GROUP_GAP,
            form_select_width + spacing + GROUP_GAP,
        )
    } else {
        (0.0, 0.0)
    };
    let compact_form = available < representation + form_wide + rest;
    let folded = chooses && available < representation + form_narrow + rest;
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
    let block = &blocks[controls.block];
    let grid = held(&analysis.waveforms[block.cells()[0]].x);
    if let Some(grid) = grid.as_deref() {
        controls.sample = controls.sample.min(grid.len().saturating_sub(1));
    }
    let offered = offered_forms(block, matrix.references().len());
    if !offered.contains(&controls.form) {
        controls.form = NetworkForm::S;
    }
    let mut drawn_form = segmented_form;
    if offered.len() > 1 {
        if compact_form {
            let labels = offered
                .iter()
                .map(|form| form.label().to_owned())
                .collect::<Vec<_>>();
            if let Some(picked) = select(
                ui,
                "rspice.results.network-matrix.form",
                "Parameters",
                controls.form.label(),
                &labels,
                form_select_width,
            ) {
                controls.form = offered[picked];
            }
        } else {
            let labels = offered.iter().map(|form| form.label()).collect::<Vec<_>>();
            let mut chosen = offered
                .iter()
                .position(|form| *form == controls.form)
                .unwrap_or(0);
            let left = ui.cursor().left();
            segmented(
                ui,
                "rspice.results.network-matrix.form",
                &labels,
                &mut chosen,
                SegmentedWidth::Natural,
            );
            drawn_form = ui.cursor().left() - left;
            controls.form = offered[chosen];
        }
        ui.add_space(GROUP_GAP);
    }
    let start = ui.cursor().left();
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
    ui.data_mut(|data| data.insert_temp(measures_id, (drawn_form, measured)));
}

/// The frequency as a stepper: the arrows move one sample, the readout
/// between them names the frequency, dragging it scrubs the samples, and a
/// click types a frequency, which lands on the nearest sample.
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
        Some(grid) => fmt_column_reading(grid[sample.min(last)], "Hz", significant_digits),
        None => "Frequency pending".to_owned(),
    };
    // The widest frequency the reader's digits can print, so the readout
    // holds one width as it steps and nothing after it moves.
    let widest = fmt_column_reading(8.888_888_888_888_89e8, "MHz", significant_digits);
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
                .custom_parser(move |text| {
                    let grid = grid?;
                    parse_frequency(text).map(|frequency| nearest_sample(grid, frequency) as f64)
                }),
        )
        .on_hover_text(format!(
            "Frequency: drag to scrub, or click and type one (2.4G, 900 MHz); \
             it lands on the nearest of the {count} samples"
        ));
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
            let washable = controls.form == NetworkForm::S;
            ui.add_enabled(
                washable,
                egui::Checkbox::new(&mut controls.heatmap, "Heat map"),
            )
            .on_hover_text("Wash each coefficient by its magnitude in dB")
            .on_disabled_hover_text("The heat map washes S-parameters, which are ratios");
            ui.add_enabled_ui(washable && controls.heatmap, |ui| {
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
                egui::RichText::new(fmt_column_reading(floor, "dB", 4))
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
            fmt_column_reading(
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
    let form = controls.form;
    let converted = form_matrix(analysis, matrix, block, form, controls.sample);
    let held_coefficients = form == NetworkForm::S || converted.is_some();
    // Where a cell sits in the matrix, the transpose undone.
    let position = |row: usize, column: usize| {
        if controls.transpose {
            (column, row)
        } else {
            (row, column)
        }
    };
    let entry = |row: usize, column: usize| {
        let (out, input) = position(row, column);
        let waveform = &analysis.waveforms[block.cells()[out * ports + input]];
        let complex = waveform
            .complex
            .as_ref()
            .expect("resolved complex coefficient");
        let (real, imaginary) = match (form, &converted) {
            (NetworkForm::S, _) => (
                coefficient(&complex.real, controls.sample),
                coefficient(&complex.imag, controls.sample),
            ),
            (_, Some(values)) => (
                values[out * ports + input].re,
                values[out * ports + input].im,
            ),
            (_, None) => (f64::NAN, f64::NAN),
        };
        (waveform, complex, real, imaginary)
    };
    let name = |row: usize, column: usize| {
        let (out, input) = position(row, column);
        match form {
            NetworkForm::S => entry(row, column).1.source_name.clone(),
            _ => form.coefficient(out, input, |index| {
                channel_label(index, ports, mixed)
                    .trim_start_matches('P')
                    .to_owned()
            }),
        }
    };
    let frequency = grid[controls.sample];
    let verdict = controls
        .diagnostic
        .as_ref()
        .filter(|diagnostic| {
            diagnostic.block == controls.block && diagnostic.sample == controls.sample
        })
        .cloned();
    let washed = controls.heatmap && form == NetworkForm::S;
    let absence = (!held_coefficients).then(|| form_absence(form));
    let footer = washed || verdict.is_some() || absence.is_some();
    let table_height = if footer {
        (ui.available_height() - FOOTER_HEIGHT - ui.spacing().item_spacing.y).max(0.0)
    } else {
        ui.available_height()
    };
    let reading = MatrixReading::of(
        (0..ports)
            .flat_map(|row| (0..ports).map(move |column| (row, column)))
            .map(|(row, column)| {
                let (_, _, real, imaginary) = entry(row, column);
                let (out, input) = position(row, column);
                (form.unit(out, input), real, imaginary)
            }),
        controls.representation,
        significant_digits,
    );
    let (response, chosen) = with_table_state(ui, "network-matrix", |ui, state| {
        let response = DataTable::new("rspice.results.network-matrix", &columns, ports)
            .state(state)
            .freeze(1)
            .cell_selection()
            .max_height(table_height)
            .show(ui, |row| {
                let mut cells = vec![Cell::text(channel(row))];
                cells.extend((0..ports).map(|column| {
                    let (_, _, real, imaginary) = entry(row, column);
                    let (out, input) = position(row, column);
                    let unit = form.unit(out, input);
                    let text = if real.is_finite() && imaginary.is_finite() {
                        reading.cell(
                            real,
                            imaginary,
                            unit,
                            controls.representation,
                            angle_display,
                        )
                    } else {
                        "\u{2014}".to_owned()
                    };
                    let opens = if form == NetworkForm::S {
                        "\nDouble-click or press Enter to open this coefficient in Polar"
                    } else {
                        ""
                    };
                    let mut cell = Cell::quantity(text).hint(format!(
                        "{}\nReal: {real:.17e} {unit}\nImaginary: {imaginary:.17e} {unit}\nFrequency: {frequency:.17e} Hz{opens}",
                        name(row, column)
                    ));
                    if washed {
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
                if washed {
                    heat_legend(ui, controls.floor_db, &tokens.color);
                    ui.add_space(16.0);
                }
                if let Some(absence) = absence {
                    ui.label(
                        egui::RichText::new(format!("{}: {absence}", form.label()))
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(tokens.color.text_dim),
                    );
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
        .filter(|(_, column)| *column > 0 && form == NetworkForm::S);
    if let Some((row, column)) = opened {
        let (waveform, ..) = entry(row, column - 1);
        open_coefficient = Some(OpenCoefficient {
            waveform_name: waveform.name.clone(),
            frequency,
        });
    }
    open_coefficient
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::analysis_type::AnalysisType;
    use rspice_results::family_metadata::AnalysisResultFamilyMetadata;
    use rspice_results::network_matrix::resolve;

    /// A two-port of one sample with the given S11, S12, S21, S22.
    fn two_port(s: [num_complex::Complex64; 4]) -> AnalysisResult<RetainedTrace> {
        let waveforms = ["S11", "S12", "S21", "S22"]
            .into_iter()
            .zip(s)
            .map(|(name, value)| {
                RetainedTrace::new(format!("|{name}|"), vec![1.0e9], vec![value.norm()], "#fff")
                    .with_complex_components(name, vec![value.re], vec![value.im])
            })
            .collect::<Vec<_>>();
        AnalysisResult::new(1, AnalysisType::SParameter, "SP", 0.0)
            .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm: vec![50.0, 50.0],
                noise_reference_temperature_kelvin: None,
            })
            .with_waveforms(waveforms)
    }

    /// The passivity line says in words whether the network is passive and
    /// whether it is reciprocal, with its numbers at the sheet's digits: a
    /// matched 6 dB attenuator is both, with a largest singular value of
    /// one half; an isolator passes forward only, so it is not reciprocal.
    #[test]
    fn the_passivity_line_reads_in_words_at_the_sheets_digits() {
        let zero = num_complex::Complex64::new(0.0, 0.0);
        let half = num_complex::Complex64::new(0.5, 0.0);
        let attenuator = two_port([zero, half, half, zero]);
        let layout = resolve(&attenuator).expect("a two-port");
        let line = passivity(&attenuator, &layout, 0, 0, 1.0e9, 4).line;
        assert!(
            line.starts_with(
                "At 1 GHz: passive (largest singular value 0.5), reciprocal (residual 0"
            ),
            "{line}"
        );
        let isolator = two_port([zero, zero, num_complex::Complex64::new(0.9, 0.0), zero]);
        let layout = resolve(&isolator).expect("a two-port");
        let line = passivity(&isolator, &layout, 0, 0, 1.0e9, 4).line;
        assert_eq!(
            line,
            "At 1 GHz: passive (largest singular value 0.9), not reciprocal (residual 0.9)"
        );
    }

    #[test]
    fn a_typed_frequency_reads_as_the_readout_prints_it() {
        assert_eq!(parse_frequency("2.4G"), Some(2.4e9));
        assert_eq!(parse_frequency("900 MHz"), Some(900e6));
        assert_eq!(parse_frequency("1.5meg"), Some(1.5e6));
        assert_eq!(parse_frequency("10 kHz"), Some(10e3));
        assert_eq!(parse_frequency("250m"), Some(0.25));
        assert_eq!(parse_frequency("3e9"), Some(3e9));
        assert_eq!(parse_frequency("12 µHz"), Some(12e-6));
        assert_eq!(parse_frequency("fast"), None);
        assert_eq!(parse_frequency("1 X"), None);
    }

    #[test]
    fn a_frequency_lands_on_the_nearest_sample() {
        let grid = [1e6, 2e6, 4e6, 8e6];
        assert_eq!(nearest_sample(&grid, 0.0), 0);
        assert_eq!(nearest_sample(&grid, 1.4e6), 0);
        assert_eq!(nearest_sample(&grid, 1.6e6), 1);
        assert_eq!(nearest_sample(&grid, 4e6), 2);
        assert_eq!(nearest_sample(&grid, 7e6), 3);
        assert_eq!(nearest_sample(&grid, 1e12), 3);
    }

    #[test]
    fn a_coefficient_with_a_unit_reads_in_it() {
        assert_eq!(
            formatted(50.0, -25.0, "Ω", 0, AngleDisplay::Degrees, 4),
            "50.00 \u{2212} 25.00j Ω"
        );
        assert_eq!(
            formatted(0.0, 0.02, "S", 1, AngleDisplay::Degrees, 4),
            "20.00 mS ∠ 90.00\u{b0}"
        );
        assert_eq!(
            formatted(100.0, 0.0, "Ω", 2, AngleDisplay::Degrees, 4),
            "40.00 dBΩ ∠ 0.00\u{b0}"
        );
    }

    /// A matrix reads with one count of decimals in every cell and both
    /// parts, the largest part's at the sheet's digits: a quarter and a
    /// hundredth read `0.2500` and `0.0100`, never `0.01000`. Past four
    /// decades apart every part reads in engineering notation instead.
    #[test]
    fn a_matrix_reads_with_one_count_of_decimals() {
        let cells = |matrix: &[(f64, f64)], representation: usize, angle: AngleDisplay| {
            let reading = MatrixReading::of(
                matrix
                    .iter()
                    .map(|(real, imaginary)| ("", *real, *imaginary)),
                representation,
                4,
            );
            matrix
                .iter()
                .map(|(real, imaginary)| reading.cell(*real, *imaginary, "", representation, angle))
                .collect::<Vec<_>>()
        };
        let mixed = [(0.25, 0.0), (0.01, 0.0), (0.01, -0.002), (0.25, 0.0)];
        assert_eq!(
            cells(&mixed, 0, AngleDisplay::Degrees),
            [
                "0.2500 + 0.0000j",
                "0.0100 + 0.0000j",
                "0.0100 \u{2212} 0.0020j",
                "0.2500 + 0.0000j"
            ]
        );
        assert_eq!(
            cells(&mixed[..2], 1, AngleDisplay::Radians),
            ["0.2500 ∠ 0 rad", "0.0100 ∠ 0 rad"]
        );
        let apart = [(0.5, 0.0), (2.0e-5, -1.0e-5)];
        assert_eq!(
            cells(&apart, 0, AngleDisplay::Degrees),
            [
                "500.0e\u{2212}3 + 0j",
                "20.00e\u{2212}6 \u{2212} 10.00e\u{2212}6j"
            ]
        );
        // Each unit of a mixed-unit matrix reads at its own resolution.
        let chain = MatrixReading::of([("", 0.5, 0.0), ("Ω", 50.0, 0.0)], 0, 4);
        assert_eq!(
            chain.cell(0.5, 0.0, "", 0, AngleDisplay::Degrees),
            "0.5000 + 0.0000j"
        );
        assert_eq!(
            chain.cell(50.0, 0.0, "Ω", 0, AngleDisplay::Degrees),
            "50.00 + 0.00j Ω"
        );
    }
}
