//! Exact, frequency-indexed network matrices from retained power-wave data.
//!
//! A periodic sideband block is kept separate from every other block. Missing
//! cells, ambiguous names, and differing grids cannot become zero coefficients.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Arc;

use egui::Ui;

use super::{AnalysisPresentationKey, AppState, ResultSheetTable, SheetContext, well_hint};
use crate::state::{AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType};

#[derive(Debug, Clone)]
pub(super) struct NetworkMatrixState {
    source: Option<(
        AnalysisPresentationKey,
        crate::state::RunHistoryRevision,
        u64,
    )>,
    block: usize,
    sample: usize,
    representation: usize,
    transpose: bool,
    diagnostic: Option<(usize, usize, String)>,
    heatmap: bool,
    floor_db: f64,
    open_trace: bool,
    layout: Option<Arc<NetworkLayout>>,
}

impl Default for NetworkMatrixState {
    fn default() -> Self {
        Self {
            source: None,
            block: 0,
            sample: 0,
            representation: 0,
            transpose: false,
            diagnostic: None,
            heatmap: true,
            floor_db: -80.0,
            open_trace: false,
            layout: None,
        }
    }
}

impl NetworkMatrixState {
    fn bind(&mut self, key: AnalysisPresentationKey, simulation: &crate::state::SimulationState) {
        let source = (key, simulation.runs.revision(), simulation.data_version);
        if self.source.as_ref() != Some(&source) {
            self.source = Some(source);
            self.block = 0;
            self.sample = 0;
            self.diagnostic = None;
            self.open_trace = false;
            self.layout = None;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BlockKey {
    mixed: bool,
    sidebands: Option<(i32, i32)>,
}

impl BlockKey {
    fn label(&self) -> String {
        let basis = if self.mixed {
            "Differential / common"
        } else {
            "Single-ended"
        };
        match self.sidebands {
            Some((output, input)) => format!("{basis} · k={output:+}, m={input:+}"),
            None => basis.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
struct MatrixBlock {
    key: BlockKey,
    /// Row-major indexes into the immutable analysis waveform inventory.
    cells: Vec<usize>,
}

struct NetworkMatrix<'a> {
    analysis: &'a AnalysisResult,
    references: &'a [f64],
    blocks: Cow<'a, [MatrixBlock]>,
}

#[derive(Debug)]
struct NetworkLayout {
    references: Vec<f64>,
    blocks: Vec<MatrixBlock>,
}

fn term(name: &str, ports: usize) -> Option<(BlockKey, usize, usize)> {
    let name = name.trim().trim_matches('|');
    let (base, sidebands) = if let Some((base, tail)) = name.split_once('[') {
        let (output, input) = tail.strip_suffix(']')?.split_once(',')?;
        (
            base,
            Some((
                output.strip_prefix("k=")?.parse().ok()?,
                input.strip_prefix("m=")?.parse().ok()?,
            )),
        )
    } else {
        (name, None)
    };
    let identity = super::smith::trace_identity(base)?;
    let mixed = !identity.physical_ports;
    let (row, column) = if mixed {
        if !ports.is_multiple_of(2) {
            return None;
        }
        let pairs = ports / 2;
        if identity.output_port > pairs || identity.input_port > pairs {
            return None;
        }
        let prefix = base.get(1..3)?.to_ascii_lowercase();
        let modes = prefix.as_bytes();
        (
            identity.output_port - 1 + usize::from(modes[0] == b'c') * pairs,
            identity.input_port - 1 + usize::from(modes[1] == b'c') * pairs,
        )
    } else {
        if identity.output_port > ports || identity.input_port > ports {
            return None;
        }
        (identity.output_port - 1, identity.input_port - 1)
    };
    Some((BlockKey { mixed, sidebands }, row, column))
}

/// Shape resolution is cheap enough for rendering; full grids are checked by
/// the retained structural memo once per source generation.
fn resolve(analysis: &AnalysisResult) -> Option<NetworkMatrix<'_>> {
    if !analysis.success
        || !matches!(
            analysis.analysis_type,
            AnalysisType::SParameter | AnalysisType::Psp | AnalysisType::Hbsp
        )
    {
        return None;
    }
    let Some(AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm: references,
        ..
    }) = &analysis.family_metadata
    else {
        return None;
    };
    let ports = references.len();
    let cells = ports.checked_mul(ports)?;
    if ports == 0 || references.iter().any(|z| !z.is_finite() || *z <= 0.0) {
        return None;
    }
    let mut blocks: BTreeMap<BlockKey, BTreeMap<usize, usize>> = BTreeMap::new();
    for (index, waveform) in analysis.waveforms.iter().enumerate() {
        let Some(complex) = &waveform.complex else {
            continue;
        };
        let name = if complex.source_name.trim().is_empty() {
            &waveform.name
        } else {
            &complex.source_name
        };
        let Some((key, row, column)) = term(name, ports) else {
            continue;
        };
        if waveform.x.is_empty()
            || waveform.x.len() != complex.real.len()
            || waveform.x.len() != complex.imag.len()
        {
            return None;
        }
        if key.mixed
            && references.chunks_exact(2).any(|pair| {
                pair[0] != pair[1] || !(pair[0] * 2.0).is_finite() || pair[0] / 2.0 <= 0.0
            })
        {
            return None;
        }
        if blocks
            .entry(key)
            .or_default()
            .insert(row * ports + column, index)
            .is_some()
        {
            return None;
        }
    }
    // Every advertised block must be complete. A partial matrix cannot prove
    // mode conversion, reciprocity, or the absence of a coupling term.
    if blocks.is_empty() || blocks.values().any(|block| block.len() != cells) {
        return None;
    }
    let blocks = blocks
        .into_iter()
        .map(|(key, cells)| MatrixBlock {
            key,
            cells: cells.into_values().collect(),
        })
        .collect();
    Some(NetworkMatrix {
        analysis,
        references,
        blocks: Cow::Owned(blocks),
    })
}

pub(super) fn structure_is_renderable(analysis: &AnalysisResult) -> bool {
    let Some(matrix) = resolve(analysis) else {
        return false;
    };
    super::frame_work::note(super::frame_work::DatasetWalk::SParameterTraceScan);
    matrix.blocks.iter().all(|block| {
        let grid = &analysis.waveforms[block.cells[0]].x;
        grid.iter().all(|x| x.is_finite() && *x >= 0.0)
            && grid.windows(2).all(|pair| pair[0] < pair[1])
            && block.cells.iter().all(|&index| {
                let waveform = &analysis.waveforms[index];
                waveform.x.as_ref() == grid.as_ref()
                    && waveform
                        .complex
                        .as_ref()
                        .is_some_and(|c| c.real.iter().chain(c.imag.iter()).all(|v| v.is_finite()))
            })
    })
}

/// What the tab strip says about this sheet, in the sheet's own words.
///
/// The structural verdict comes from the workspace memo rather than a fresh
/// walk of every retained coefficient.
pub(super) fn availability(state: &AppState) -> super::ViewerAvailability {
    if state.simulation.active_run().is_some_and(|run| {
        state.simulation.active_analysis().is_some_and(|analysis| {
            super::analysis_answers_structural_gate(
                state,
                run.dataset_id,
                analysis,
                super::StructuralGate::NetworkMatrix,
            ) && super::analysis_evidence_is_valid(state, run.dataset_id, analysis)
        })
    }) {
        super::ViewerAvailability::available(
            "Complete retained network matrices and port references are available",
        )
    } else {
        super::ViewerAvailability::unavailable(
            "Requires complete SP, PSP, or HBSP complex matrices on a shared frequency grid with port references",
        )
    }
}

pub(super) fn right_panel(ui: &mut Ui) {
    super::panel_note(
        ui,
        "Exact retained power-wave coefficients. Select a frequency and sideband block in the matrix. Hover or copy for full numerical precision.",
    );
}

fn channel_label(index: usize, ports: usize, mixed: bool) -> String {
    if mixed {
        let pairs = ports / 2;
        format!(
            "{}{}",
            if index < pairs { "D" } else { "C" },
            index % pairs + 1
        )
    } else {
        format!("P{}", index + 1)
    }
}

fn channel_reference(index: usize, references: &[f64], mixed: bool) -> f64 {
    if mixed {
        let pairs = references.len() / 2;
        references[2 * (index % pairs)] * if index < pairs { 2.0 } else { 0.5 }
    } else {
        references[index]
    }
}

fn formatted(real: f64, imaginary: f64, representation: usize) -> String {
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
fn magnitude_db(real: f64, imaginary: f64) -> f64 {
    let scale = real.abs().max(imaginary.abs());
    if scale == 0.0 {
        f64::NEG_INFINITY
    } else {
        20.0 * (scale.log10() + (real / scale).hypot(imaginary / scale).log10())
    }
}

fn heat_color(db: f64, floor: f64, palette: &crate::ui::palette::Palette) -> egui::Color32 {
    if db > 0.0 {
        return palette.warn;
    }
    let amount = ((db - floor) / -floor).clamp(0.0, 1.0) as f32;
    egui::Color32::from(
        egui::Rgba::from(palette.bg_inset) * (1.0 - amount)
            + egui::Rgba::from(palette.info) * amount,
    )
}

fn heat_legend(ui: &mut Ui, floor: &mut f64, palette: &crate::ui::palette::Palette) {
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

pub(super) fn show(ui: &mut Ui, context: &mut SheetContext<'_>) {
    let Some(run) = context.simulation.active_run() else {
        well_hint(ui, "Run a network analysis to inspect its matrix");
        return;
    };
    let Some(analysis) = context.simulation.active_analysis() else {
        return;
    };
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    let valid = context.results.structural_gates.get_or_insert_with(
        context.simulation,
        (key, super::StructuralGate::NetworkMatrix),
        || structure_is_renderable(analysis),
    ) && context
        .results
        .retained_evidence_validity
        .get_or_insert_with(context.simulation, key, || {
            super::frame_work::note(super::frame_work::DatasetWalk::EvidenceValidation);
            analysis.validate_retained_evidence().is_ok()
        });
    if !valid {
        well_hint(
            ui,
            "The retained network matrix is incomplete or has invalid coefficient or frequency evidence",
        );
        return;
    }
    let controls = &mut context.results.network_matrix;
    controls.bind(key, context.simulation);
    // Matrix topology can be large even when only a few rows are visible.
    // Resolve names and block membership once per retained source generation;
    // subsequent frames borrow its index inventory and only read visible cells.
    if controls.layout.is_none() {
        let Some(matrix) = resolve(analysis) else {
            well_hint(
                ui,
                "Requires a complete complex network matrix and its physical port references",
            );
            return;
        };
        controls.layout = Some(Arc::new(NetworkLayout {
            references: matrix.references.to_vec(),
            blocks: matrix.blocks.into_owned(),
        }));
    }
    let layout = controls.layout.as_ref().expect("resolved layout").clone();
    let matrix = NetworkMatrix {
        analysis,
        references: &layout.references,
        blocks: Cow::Borrowed(&layout.blocks),
    };
    let tokens = crate::ui::tokens::Tokens::get(ui.ctx());
    controls.block = controls.block.min(matrix.blocks.len() - 1);
    ui.horizontal_wrapped(|ui| {
        egui::ComboBox::from_id_salt("network-matrix-block")
            .selected_text(matrix.blocks[controls.block].key.label())
            .show_ui(ui, |ui| {
                for (index, block) in matrix.blocks.iter().enumerate() {
                    ui.selectable_value(&mut controls.block, index, block.key.label());
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
    let block = &matrix.blocks[controls.block];
    let grid = &analysis.waveforms[block.cells[0]].x;
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
                .copy_text(matrix_csv(&matrix, controls.block, controls.sample));
        }
    });
    ui.label(if controls.transpose {
        "Rows: incident waves · Columns: outgoing waves"
    } else {
        "Rows: outgoing waves · Columns: incident waves"
    });
    if block.key.mixed {
        ui.label("Adjacent physical ports form (+, −) pairs. Differential and common power waves use (a+ − a−)/√2 and (a+ + a−)/√2.");
    }
    if analysis.analysis_type == AnalysisType::SParameter {
        if ui
            .add_enabled(
                matrix.references.len()
                    <= rspice_core::analysis::s_param::MAX_NETWORK_DIAGNOSTIC_PORTS,
                egui::Button::new("Check sampled passivity / reciprocity"),
            )
            .on_disabled_hover_text("Interactive dense diagnostics support up to 128 ports")
            .clicked()
        {
            let ports = matrix.references.len();
            let values = (0..ports)
                .map(|row| {
                    (0..ports)
                        .map(|column| {
                            let complex = analysis.waveforms[block.cells[row * ports + column]]
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
    let ports = matrix.references.len();
    let row_height = ui.spacing().interact_size.y.max(32.0);
    egui::ScrollArea::horizontal().id_salt("network-matrix-scroll").auto_shrink([false, false]).show(ui, |ui| {
        egui_extras::TableBuilder::new(ui).id_salt("network-matrix-cells").striped(true)
            .columns(egui_extras::Column::exact(240.0), ports + 1)
            .header(row_height, |mut header| {
                header.col(|ui| { ui.strong("Wave channel / reference"); });
                for column in 0..ports {
                    header.col(|ui| { ui.strong(format!("{} · {:.8e} Ω", channel_label(column, ports, block.key.mixed), channel_reference(column, matrix.references, block.key.mixed))); });
                }
            }).body(|body| body.rows(row_height, ports, |mut table_row| {
                let row = table_row.index();
                table_row.col(|ui| { ui.strong(format!("{} · {:.8e} Ω", channel_label(row, ports, block.key.mixed), channel_reference(row, matrix.references, block.key.mixed))); });
                for column in 0..ports {
                    table_row.col(|ui| {
                    let (out, input) = if controls.transpose { (column, row) } else { (row, column) };
                    let waveform = &analysis.waveforms[block.cells[out * ports + input]];
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
                        context.results.polar.quantity = Some(waveform.name.clone());
                        context.results.cursors.a = Some(grid[controls.sample]);
                        context.results.cursors.b = None;
                        controls.open_trace = true;
                    }
                    });
                }
            }));
    });
    let mut open_trace = controls.open_trace;
    if open_trace {
        let available =
            (ui.ctx().content_rect().size() - egui::vec2(24.0, 24.0)).max(egui::vec2(280.0, 280.0));
        egui::Window::new("Network coefficient · Polar")
            .id(ui.id().with("network-matrix-polar"))
            .open(&mut open_trace)
            .default_size(egui::vec2(600.0, 520.0).min(available))
            .max_size(available)
            .show(ui.ctx(), |ui| {
                ui.horizontal_wrapped(|ui| {
                    super::polar::domain_bar(ui, context);
                });
                ui.collapsing("Cursor measurements", |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| super::polar::right_panel(ui, context));
                });
                super::polar::show(ui, context);
            });
    }
    context.results.network_matrix.open_trace = open_trace;
}

fn matrix_csv(matrix: &NetworkMatrix<'_>, block_index: usize, sample: usize) -> String {
    let table = exact_table(matrix, block_index, sample);
    let mut csv = table.columns.join(",");
    csv.push('\n');
    for row in table.rows {
        csv.push_str(
            &row.iter()
                .map(|v| super::csv_field(v))
                .collect::<Vec<_>>()
                .join(","),
        );
        csv.push('\n');
    }
    csv
}

fn exact_table(matrix: &NetworkMatrix<'_>, block_index: usize, sample: usize) -> ResultSheetTable {
    let block = &matrix.blocks[block_index];
    let ports = matrix.references.len();
    let rows = block
        .cells
        .iter()
        .enumerate()
        .map(|(cell, &index)| {
            let waveform = &matrix.analysis.waveforms[index];
            let complex = waveform
                .complex
                .as_ref()
                .expect("resolved complex coefficient");
            vec![
                complex.source_name.clone(),
                format!("{:.17e}", waveform.x[sample]),
                channel_label(cell / ports, ports, block.key.mixed),
                channel_label(cell % ports, ports, block.key.mixed),
                format!(
                    "{:.17e}",
                    channel_reference(cell / ports, matrix.references, block.key.mixed)
                ),
                format!(
                    "{:.17e}",
                    channel_reference(cell % ports, matrix.references, block.key.mixed)
                ),
                format!("{:.17e}", complex.real[sample]),
                format!("{:.17e}", complex.imag[sample]),
            ]
        })
        .collect();
    ResultSheetTable {
        title: format!("Network matrix · {}", block.key.label()),
        columns: [
            "Coefficient",
            "Frequency (Hz)",
            "Output",
            "Input",
            "Output reference (ohm)",
            "Input reference (ohm)",
            "Real",
            "Imaginary",
        ]
        .map(str::to_owned)
        .to_vec(),
        rows,
    }
}

pub(crate) fn hardcopy_tables(
    analysis: &AnalysisResult,
) -> Result<Vec<ResultSheetTable>, &'static str> {
    if !structure_is_renderable(analysis) {
        return Err("requires a complete finite network matrix");
    }
    let matrix = resolve(analysis).ok_or("requires a complete finite network matrix")?;
    // Check before allocating formatted strings. The hardcopy worker transports
    // at most 64 MiB, and report tables themselves permit at most 100,000 rows.
    let rows = matrix.blocks.iter().try_fold(0usize, |total, block| {
        total.checked_add(
            block
                .cells
                .len()
                .checked_mul(analysis.waveforms[block.cells[0]].x.len())?,
        )
    });
    if rows.is_none_or(|rows| rows > 100_000) {
        return Err(
            "network matrix report exceeds 100,000 coefficient rows; export the result waveforms or copy an individual matrix instead",
        );
    }
    let mut tables = Vec::new();
    for (index, block) in matrix.blocks.iter().enumerate() {
        let mut table = exact_table(&matrix, index, 0);
        for sample in 1..analysis.waveforms[block.cells[0]].x.len() {
            table.rows.extend(exact_table(&matrix, index, sample).rows);
        }
        tables.push(table);
    }
    Ok(tables)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::state::{SimulationRun, WaveformData};

    pub(crate) fn fixture(mixed: bool) -> AnalysisResult {
        let mut traces = Vec::new();
        let ports = if mixed { 4 } else { 2 };
        for row in 0..ports {
            for column in 0..ports {
                let name = if mixed {
                    format!(
                        "S{}{}{}{}",
                        if row < 2 { "d" } else { "c" },
                        if column < 2 { "d" } else { "c" },
                        row % 2 + 1,
                        column % 2 + 1
                    )
                } else {
                    format!("S{}{}", row + 1, column + 1)
                };
                let value = if row == column { 0.25 } else { 0.01 };
                traces.push(
                    WaveformData::new(
                        format!("|{name}|"),
                        vec![1e6, 2e6],
                        vec![value; 2],
                        "#00aaff",
                    )
                    .with_complex_components(
                        name,
                        vec![value; 2],
                        vec![0.0; 2],
                    ),
                );
            }
        }
        AnalysisResult::new(1, AnalysisType::SParameter, "Network")
            .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm: vec![50.0; ports],
                noise_reference_temperature_kelvin: None,
            })
            .with_waveforms(traces)
    }

    #[test]
    fn complete_physical_and_mixed_matrices_have_exact_references_and_values() {
        for mixed in [false, true] {
            let analysis = fixture(mixed);
            assert!(structure_is_renderable(&analysis));
            let matrix = resolve(&analysis).unwrap();
            let csv = matrix_csv(&matrix, 0, 1);
            assert!(csv.contains("2.00000000000000000e6"));
            let tables = hardcopy_tables(&analysis).unwrap();
            assert_eq!(tables.len(), 1);
            assert_eq!(tables[0].rows.len(), 2 * matrix.references.len().pow(2));
            assert_eq!(
                channel_reference(0, matrix.references, mixed),
                if mixed { 100.0 } else { 50.0 }
            );
            if mixed {
                assert_eq!(channel_reference(2, matrix.references, mixed), 25.0);
            }
        }
    }

    #[test]
    fn replacing_evidence_cannot_reuse_a_sampled_diagnostic() {
        let analysis = fixture(false);
        let mut simulation = crate::state::SimulationState::default();
        let mut run = SimulationRun::new(1);
        let key = AnalysisPresentationKey::new(run.dataset_id, &analysis);
        run.add_analysis(analysis);
        simulation.runs.push(run);
        let mut controls = NetworkMatrixState::default();
        controls.bind(key, &simulation);
        controls.diagnostic = Some((0, 0, "old evidence".to_owned()));
        controls.bind(key, &simulation.clone());
        assert!(controls.diagnostic.is_some());
        simulation.data_version = simulation.data_version.wrapping_add(1);
        controls.bind(key, &simulation);
        assert!(controls.diagnostic.is_none());
        controls.diagnostic = Some((0, 0, "old evidence".to_owned()));
        simulation.runs[0].analyses[0] = fixture(false);
        controls.bind(key, &simulation);
        assert!(controls.diagnostic.is_none());
    }

    #[test]
    fn absent_duplicate_or_misaligned_coefficients_are_refused() {
        let mut missing = fixture(false);
        missing.waveforms.pop();
        assert!(!structure_is_renderable(&missing));
        let mut duplicate = fixture(false);
        duplicate.waveforms.push(duplicate.waveforms[0].clone());
        assert!(!structure_is_renderable(&duplicate));
        let mut wrong_grid = fixture(false);
        wrong_grid.waveforms[0].x = vec![1e6, 3e6].into();
        assert!(!structure_is_renderable(&wrong_grid));
    }

    #[test]
    fn translated_blocks_and_large_port_indexes_keep_their_identity() {
        assert_eq!(
            term("S12_10[k=-2,m=+3]", 12),
            Some((
                BlockKey {
                    mixed: false,
                    sidebands: Some((-2, 3))
                },
                11,
                9
            ))
        );
        assert!(term("S12[k=0]", 2).is_none());
        assert!(term("S00", 2).is_none());
        let mut periodic = fixture(false);
        periodic.analysis_type = AnalysisType::Psp;
        let mut second = periodic.waveforms.clone();
        for waveform in &mut second {
            let complex = waveform.complex.as_mut().unwrap();
            complex.source_name.push_str("[k=+1,m=-1]");
            waveform.name = format!("|{}|", complex.source_name);
        }
        periodic.waveforms.extend(second);
        assert!(structure_is_renderable(&periodic));
        assert_eq!(resolve(&periodic).unwrap().blocks.len(), 2);
    }

    #[test]
    fn mixed_mode_needs_complete_equal_reference_pairs() {
        let mut analysis = fixture(true);
        let Some(AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm,
            ..
        }) = &mut analysis.family_metadata
        else {
            unreachable!()
        };
        reference_impedances_ohm[0] = 75.0;
        assert!(!structure_is_renderable(&analysis));
    }

    #[test]
    fn exact_export_round_trips_complex_components() {
        let analysis = fixture(false);
        let matrix = resolve(&analysis).unwrap();
        let table = exact_table(&matrix, 0, 1);
        for (index, row) in table.rows.iter().enumerate() {
            let complex = analysis.waveforms[matrix.blocks[0].cells[index]]
                .complex
                .as_ref()
                .unwrap();
            assert_eq!(
                row[6].parse::<f64>().unwrap().to_bits(),
                complex.real[1].to_bits()
            );
            assert_eq!(
                row[7].parse::<f64>().unwrap().to_bits(),
                complex.imag[1].to_bits()
            );
        }
        assert!(formatted(0.0, 0.0, 2).contains("phase undefined"));
        assert!((magnitude_db(1e308, 1e308) - (6160.0 + 10.0 * 2.0_f64.log10())).abs() < 1e-10);
    }

    fn cell_text_rect(shape: &egui::Shape) -> Option<egui::Rect> {
        match shape {
            egui::Shape::Text(text) if text.galley.text().starts_with("2.50000000e-1") => {
                Some(shape.visual_bounding_rect())
            }
            egui::Shape::Vec(shapes) => shapes.iter().find_map(cell_text_rect),
            _ => None,
        }
    }

    #[test]
    fn clicking_a_matrix_cell_opens_its_retained_coefficient_plot() {
        for width in [1280.0, 768.0] {
            let mut app = crate::workbench::AppState::default();
            let mut run = SimulationRun::new(1);
            run.add_analysis(fixture(true));
            app.simulation.runs.push(run);
            assert!(app.simulation.select_run(0));
            let before = app
                .simulation
                .active_analysis()
                .unwrap()
                .result_data_digest();
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            let mut frame = |events: Vec<egui::Event>| {
                ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 900.0),
                        )),
                        events,
                        ..Default::default()
                    },
                    |ui| {
                        egui::CentralPanel::default()
                            .show(ui, |ui| show(ui, &mut SheetContext::of(&mut app)));
                    },
                )
            };
            let _ = frame(Vec::new());
            let output = frame(Vec::new());
            let position = output
                .shapes
                .iter()
                .find_map(|shape| cell_text_rect(&shape.shape))
                .expect("visible matrix coefficient")
                .center();
            for pressed in [true, false] {
                let _ = frame(vec![
                    egui::Event::PointerMoved(position),
                    egui::Event::PointerButton {
                        pos: position,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ]);
            }
            let _ = frame(Vec::new());
            assert!(app.ui.results.network_matrix.open_trace);
            assert_eq!(app.ui.results.polar.quantity.as_deref(), Some("|Sdd11|"));
            assert_eq!(
                before,
                app.simulation
                    .active_analysis()
                    .unwrap()
                    .result_data_digest()
            );
        }
    }

    #[test]
    fn renderer_runs_at_desktop_and_tablet_widths_without_changing_evidence() {
        for width in [1280.0, 768.0] {
            let mut app = crate::workbench::AppState::default();
            let mut run = SimulationRun::new(1);
            run.add_analysis(fixture(true));
            app.simulation.runs.push(run);
            assert!(app.simulation.select_run(0));
            let before = app
                .simulation
                .active_analysis()
                .unwrap()
                .result_data_digest();
            let egui = egui::Context::default();
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(width, 900.0),
                )),
                ..Default::default()
            };
            crate::ui::Theme::default().apply(&egui);
            let output = egui.run_ui(input, |ui| {
                egui::CentralPanel::default()
                    .show(ui, |ui| show(ui, &mut SheetContext::of(&mut app)));
            });
            assert!(!output.shapes.is_empty());
            assert_eq!(
                before,
                app.simulation
                    .active_analysis()
                    .unwrap()
                    .result_data_digest()
            );
        }
    }

    #[test]
    #[ignore = "writes images for manual layout inspection"]
    fn render_network_matrix_previews() {
        let destination = std::path::PathBuf::from(
            std::env::var("RSPICE_NETWORK_MATRIX_QA_DIR")
                .expect("set the preview output directory"),
        );
        std::fs::create_dir_all(&destination).unwrap();
        for (width, open_plot) in [(1280, false), (768, false), (768, true)] {
            let mut app = crate::workbench::AppState::default();
            let mut run = SimulationRun::new(1);
            run.add_analysis(fixture(true));
            app.simulation.runs.push(run);
            assert!(app.simulation.select_run(0));
            if open_plot {
                let run = app.simulation.active_run().unwrap();
                let analysis = app.simulation.active_analysis().unwrap();
                let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
                app.ui.results.network_matrix.bind(key, &app.simulation);
                app.ui.results.network_matrix.open_trace = true;
                app.ui.results.polar.quantity = Some("|Sdd11|".to_owned());
            }
            let canvas =
                crate::ui::raster::render(egui::vec2(width as f32, 640.0), |ui, background| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE.fill(background).inner_margin(12.0))
                        .show(ui, |ui| show(ui, &mut SheetContext::of(&mut app)));
                });
            std::fs::write(
                destination.join(format!(
                    "network-matrix-{width}{}.png",
                    if open_plot { "-plot" } else { "" }
                )),
                canvas.png(640),
            )
            .unwrap();
        }
    }
}
