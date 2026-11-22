//! The phase-noise sheet: every retained L(f) trace of the run, the spot
//! offsets the reader lists read on them, the band the reader sets
//! integrated to phase error and jitter, and slope guides.

use crate::fft::view::columns::Fitting;
use crate::presentation::{PlotView, frequency_readout, well_hint};
use crate::strip::{self, LegendChip};
use crate::waves::ReadoutPolicy;
use egui::Ui;
use rspice_results::phase_noise::{
    CycleJitter, IntegratedPhaseNoise, cycle_jitter, integrate_phase_noise, spot_level_at,
    timing_density_from_phase_noise,
};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::fmt_si_significant;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::Button;
use rspice_ui_kit::widgets::table::{Align, Cell, Column, DataTable, SortKey};
use rspice_vocabulary::quantity::{AngleDisplay, QuantityPresentationPolicy};

/// One retained L(f) trace qualified by the host's source gate.
pub struct PhaseNoiseTrace<'a> {
    /// What the legend and the tables call it.
    pub label: &'a str,
    /// The carrier its offsets are measured from.
    pub carrier_frequency_hz: f64,
    pub offset_hz: &'a [f64],
    pub level_dbc_per_hz: &'a [f64],
    pub cache_key: u64,
}

impl PhaseNoiseTrace<'_> {
    fn sweep(&self) -> Option<(f64, f64)> {
        Some((*self.offset_hz.first()?, *self.offset_hz.last()?))
    }
}

/// What the reader set the sheet to read: the spot offsets, the band it
/// integrates and whether it draws slope guides. The host keeps it in the
/// view.
#[derive(Debug, Clone, PartialEq)]
pub struct PhaseNoiseSettings {
    pub spots: Vec<f64>,
    /// The band integrated, or the whole sweep when `None`.
    pub band: Option<(f64, f64)>,
    pub slopes: bool,
}

impl PhaseNoiseSettings {
    /// What a sweep opens with: a spot at every decade strictly inside it,
    /// the whole sweep integrated, no slope guides.
    #[must_use]
    pub fn for_sweep((first, last): (f64, f64)) -> Self {
        let mut spots = Vec::new();
        if first > 0.0 && last > first {
            let mut exponent = first.log10().floor() as i32;
            loop {
                let decade = 10.0_f64.powi(exponent);
                if decade >= last {
                    break;
                }
                if decade > first {
                    spots.push(decade);
                }
                exponent += 1;
            }
        }
        Self {
            spots,
            band: None,
            slopes: false,
        }
    }

    /// The band integrated on a sweep of `first..last`: the reader's, kept
    /// inside the sweep, or the whole sweep. `None` when nothing of the
    /// reader's band lies inside it.
    #[must_use]
    pub fn band_within(&self, (first, last): (f64, f64)) -> Option<(f64, f64)> {
        let (low, high) = self.band.unwrap_or((first, last));
        let (low, high) = (low.max(first), high.min(last));
        (low < high).then_some((low, high))
    }
}

/// Requests applied by the host to the same viewport that produced this frame.
pub struct PhaseNoiseResponse {
    pub fit: bool,
    pub plot: Option<plot::PlotResponse>,
}

/// The slopes a guide is drawn at, in dB per decade: the 1/f² region of
/// white frequency noise and the 1/f³ region of flicker frequency noise,
/// whose meeting is the corner an oscillator's L(f) is read for.
const SLOPE_GUIDES: [f64; 2] = [-20.0, -30.0];

/// Explain the host's reason for having no phase-noise source.
pub fn show_absent(ui: &mut Ui, selected_periodic_noise_without_phase_trace: bool) {
    well_hint(
        ui,
        if selected_periodic_noise_without_phase_trace {
            "The selected PNOISE result retains no trace explicitly identified as phase noise"
        } else {
            "No retained phase-noise spectrum in the active dataset"
        },
    );
}

/// The offsets written as a reader types them: "10k, 100k, 1M".
fn offsets_text(offsets: &[f64], digits: usize) -> String {
    offsets
        .iter()
        .map(|offset| {
            fmt_si_significant(*offset, "Hz", digits)
                .replace(' ', "")
                .trim_end_matches("Hz")
                .to_owned()
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// A list of offsets as a reader types them, separated by commas or
/// semicolons; `None` when any entry is not a positive frequency.
fn parse_offsets(text: &str) -> Option<Vec<f64>> {
    let mut offsets = Vec::new();
    for entry in text.split([',', ';']).map(str::trim) {
        if entry.is_empty() {
            continue;
        }
        let offset = crate::network_matrix::parse_frequency(entry)?;
        if offset <= 0.0 {
            return None;
        }
        offsets.push(offset);
    }
    offsets.sort_by(f64::total_cmp);
    offsets.dedup();
    Some(offsets)
}

/// A frequency field in the bar, committed on Enter or focus loss; text
/// `commit` refuses is let go and the field shows the setting again.
fn frequency_field(
    ui: &mut Ui,
    id: &str,
    shown: String,
    width: f32,
    hint: &str,
    tooltip: &str,
    commit: &mut dyn FnMut(&str),
) {
    let id = egui::Id::new(id);
    let draft = ui.data(|data| data.get_temp::<String>(id));
    let drafting = draft.is_some();
    let mut text = draft.unwrap_or(shown);
    let field = ui
        .add(
            egui::TextEdit::singleline(&mut text)
                .id(id.with("field"))
                .desired_width(width)
                .font(rspice_ui_kit::theme::field_mono())
                .hint_text(hint),
        )
        .on_hover_text(tooltip);
    if field.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text));
    } else if field.lost_focus() || drafting {
        commit(&text);
        ui.data_mut(|data| data.remove::<String>(id));
    }
}

/// The sheet's bar: the spot offsets, the band it integrates, and the
/// slope guides under Display. Returns the settings the reader changed.
pub fn domain_bar(
    ui: &mut Ui,
    settings: &PhaseNoiseSettings,
    sweep: Option<(f64, f64)>,
    digits: usize,
) -> Option<PhaseNoiseSettings> {
    let mut changed: Option<PhaseNoiseSettings> = None;
    ui.add_enabled_ui(sweep.is_some(), |ui| {
        let sweep = sweep.unwrap_or((1.0, 10.0));
        crate::chrome::bars::bar_label(ui, "Spots");
        frequency_field(
            ui,
            "rspice.results.phase-noise.spots",
            offsets_text(&settings.spots, digits),
            132.0,
            "10k, 1M",
            "Offsets L(f) is read at, separated by commas (10k, 100k, 1M)",
            &mut |text| {
                if let Some(spots) = parse_offsets(text)
                    && spots != settings.spots
                {
                    changed = Some(PhaseNoiseSettings {
                        spots,
                        ..settings.clone()
                    });
                }
            },
        );
        ui.add_space(8.0);
        crate::chrome::bars::bar_label(ui, "Band");
        let band = settings.band_within(sweep).unwrap_or(sweep);
        for index in 0..2 {
            if index == 1 {
                crate::chrome::bars::bar_label(ui, "\u{2013}");
            }
            let edge = if index == 0 { band.0 } else { band.1 };
            frequency_field(
                ui,
                if index == 0 {
                    "rspice.results.phase-noise.band-low"
                } else {
                    "rspice.results.phase-noise.band-high"
                },
                offsets_text(&[edge], digits),
                56.0,
                if index == 0 { "from" } else { "to" },
                if index == 0 {
                    "Lowest offset integrated into phase error and jitter"
                } else {
                    "Highest offset integrated into phase error and jitter"
                },
                &mut |text| {
                    let Some(value) = crate::network_matrix::parse_frequency(text.trim()) else {
                        return;
                    };
                    let (low, high) = if index == 0 {
                        (value, band.1)
                    } else {
                        (band.0, value)
                    };
                    let (low, high) = (low.max(sweep.0), high.min(sweep.1));
                    if low >= high {
                        return;
                    }
                    // The whole sweep is what an unset band integrates.
                    let band = (low > sweep.0 || high < sweep.1).then_some((low, high));
                    if band != settings.band {
                        changed = Some(PhaseNoiseSettings {
                            band,
                            ..settings.clone()
                        });
                    }
                },
            );
        }
        ui.add_space(8.0);
        let display = Button::new("Display")
            .menu()
            .accessible_label("Display options")
            .show(ui);
        egui::Popup::menu(&display)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                let mut slopes = settings.slopes;
                if ui
                    .checkbox(&mut slopes, "Slope guides")
                    .on_hover_text(
                        "Lines at \u{2212}20 and \u{2212}30 dB/decade through the first spot",
                    )
                    .changed()
                {
                    changed = Some(PhaseNoiseSettings {
                        slopes,
                        ..settings.clone()
                    });
                }
            });
    });
    changed
}

/// Draw every trace with the spots read on the first and the integrated
/// band shaded; its cursor reads every trace at the reader's significant
/// digits and by the reader's cursor interpolation.
#[allow(clippy::too_many_arguments)]
pub fn show(
    ui: &mut Ui,
    traces: &[PhaseNoiseTrace<'_>],
    settings: &PhaseNoiseSettings,
    view: PlotView,
    level_range: Option<(f64, f64)>,
    quantities: &QuantityPresentationPolicy,
    presentation: ReadoutPolicy,
    cache: &mut plot::DecimationCache,
    pin: &mut Option<f64>,
) -> PhaseNoiseResponse {
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let colors = Tokens::get(ui.ctx()).color;
    let color = |index: usize| colors.traces[index % colors.traces.len()];
    let sweep = traces
        .iter()
        .filter_map(PhaseNoiseTrace::sweep)
        .reduce(|(a0, a1), (b0, b1)| (a0.min(b0), a1.max(b1)));
    let primary = traces.first();
    // A guide passes through the first trace at the first spot it reads.
    let anchor = primary.and_then(|trace| {
        settings.spots.iter().find_map(|&spot| {
            spot_level_at(trace.offset_hz, trace.level_dbc_per_hz, spot).map(|level| (spot, level))
        })
    });
    let guides_on = settings.slopes && anchor.is_some();

    let slope_names: Vec<String> = SLOPE_GUIDES
        .iter()
        .map(|slope| fmt_si_significant(*slope, "dB/dec", digits))
        .collect();
    let mut legend: Vec<LegendChip<'_>> = traces
        .iter()
        .enumerate()
        .map(|(index, trace)| LegendChip {
            name: trace.label,
            color: color(index),
            on: true,
        })
        .collect();
    if guides_on {
        legend.extend(slope_names.iter().map(|name| LegendChip {
            name,
            color: colors.text_dim,
            on: true,
        }));
    }
    let header = strip::StripHeader::new("PNOISE", "", &legend)
        .zoomed(view.is_zoomed())
        .show(ui);
    let requested = PhaseNoiseResponse {
        fit: header.fit_clicked,
        plot: None,
    };

    let Some((x0, x1)) = sweep.filter(|(x0, x1)| x1 > x0) else {
        well_hint(ui, "Degenerate retained offset-frequency axis");
        return requested;
    };
    let Some((level_min, level_max)) = level_range else {
        well_hint(
            ui,
            "The retained phase-noise trace contains no finite levels",
        );
        return requested;
    };
    let (x0, x1) = view.x.unwrap_or((x0, x1));
    let (y0, y1) = view
        .y
        .unwrap_or_else(|| plot::padded_fit((level_min, level_max)));
    let (frequency_scale, frequency_offset, frequency_unit) = quantities.frequency_axis_transform();
    let x_axis = Axis::log_decades(x0, x1, "Hz").with_display_transform(
        frequency_scale,
        frequency_offset,
        frequency_unit,
    );
    let y_axis = Axis::linear_with(y0, y1, "dBc/Hz", 6).with_label("L(f)");
    let guide_x = [x0, x1];
    let guide_y: Vec<[f64; 2]> = SLOPE_GUIDES
        .iter()
        .map(|slope| {
            let (spot, level) = anchor.unwrap_or((1.0, 0.0));
            guide_x.map(|x| level + slope * (x / spot).log10())
        })
        .collect();
    let mut spec = PlotSpec::new(x_axis, XScale::Log10, y_axis)
        .accessible_name("Phase-noise plot")
        .accessible_detail("Retained phase-noise traces shown as L(f) in dBc/Hz.");
    if let Some(band) = sweep.and_then(|sweep| settings.band_within(sweep)) {
        spec.bands.push(plot::Band {
            x0: band.0,
            x1: band.1,
        });
    }
    for (index, trace) in traces.iter().enumerate() {
        spec.traces.push(
            Trace::new(trace.offset_hz, trace.level_dbc_per_hz, color(index))
                .cache_key(trace.cache_key),
        );
    }
    if guides_on {
        for y in &guide_y {
            spec.traces
                .push(Trace::new(&guide_x, y, colors.text_dim).dashed().thin());
        }
    }
    if let Some(trace) = primary {
        for &spot in &settings.spots {
            if let Some(level) = spot_level_at(trace.offset_hz, trace.level_dbc_per_hz, spot) {
                spec.markers.push(plot::Marker {
                    drop_line: true,
                    ..plot::Marker::point(
                        spot,
                        level,
                        colors.accent,
                        format!(
                            "{} {} dBc/Hz",
                            frequency_readout(quantities, spot, digits),
                            decibels(level)
                        ),
                    )
                });
            }
        }
    }

    let several = traces.len() > 1;
    let readout = |offset| {
        let mut rows = vec![(
            "offset".to_owned(),
            frequency_readout(quantities, offset, digits),
        )];
        for trace in traces {
            rows.push((
                if several {
                    trace.label.to_owned()
                } else {
                    "L(f)".to_owned()
                },
                format!(
                    "{} dBc/Hz",
                    decibels(presentation.cursor_sample(
                        trace.offset_hz,
                        trace.level_dbc_per_hz,
                        offset
                    ))
                ),
            ));
        }
        rows
    };
    let response = plot::show(ui, &spec, cache, None, Some(&readout));
    crate::chrome::pinned_readout::follow(ui, &response, spec.x_scale, pin, &readout);
    PhaseNoiseResponse {
        plot: Some(response),
        ..requested
    }
}

/// The inspector's evidence: the sheet's traces and settings, and the
/// device shares the run retained.
pub struct PhaseNoiseInspector<'a> {
    pub traces: &'a [PhaseNoiseTrace<'a>],
    pub settings: &'a PhaseNoiseSettings,
    pub device_shares: Vec<(&'a str, f64)>,
}

/// A level in dBc or dBc/Hz, to the hundredth of a decibel.
fn decibels(level: f64) -> String {
    crate::presentation::fixed_places(level, crate::presentation::DECIBEL_PLACES)
}

/// A number for a table cell whose column head carries the unit: the
/// prefix the unit takes, and no unit.
pub(crate) fn plain(value: f64, unit: &str, digits: usize) -> String {
    let text = fmt_si_significant(value, unit, digits);
    text.strip_suffix(unit)
        .map_or_else(|| text.clone(), |number| number.trim_end().to_owned())
}

/// One trace's figures over the band: integrated L(f), rms phase error and
/// jitter, and Jc and Jcc where the sweep reaches half the carrier.
#[derive(Debug, Clone, Copy)]
pub struct BandFigures {
    pub integrated: IntegratedPhaseNoise,
    pub jitter_s: f64,
    pub cycle: Option<CycleJitter>,
}

/// What `trace` integrates to over the settings' band.
#[must_use]
pub fn band_figures(
    trace: &PhaseNoiseTrace<'_>,
    settings: &PhaseNoiseSettings,
) -> Option<BandFigures> {
    let band = settings.band_within(trace.sweep()?)?;
    let integrated = integrate_phase_noise(trace.offset_hz, trace.level_dbc_per_hz, band)?;
    let timing =
        timing_density_from_phase_noise(trace.level_dbc_per_hz, trace.carrier_frequency_hz);
    Some(BandFigures {
        integrated,
        jitter_s: integrated.jitter_rms_s(trace.carrier_frequency_hz),
        cycle: cycle_jitter(trace.offset_hz, &timing, trace.carrier_frequency_hz),
    })
}

/// The carrier, the spots read on every trace and the band's figures, at
/// the reader's `significant_digits`.
pub fn right_panel(
    ui: &mut Ui,
    input: Option<PhaseNoiseInspector<'_>>,
    quantities: &QuantityPresentationPolicy,
    significant_digits: usize,
) {
    let digits = significant_digits;
    section_header(ui, "Phase noise", None);
    let Some(PhaseNoiseInspector {
        traces,
        settings,
        device_shares,
    }) = input
    else {
        crate::presentation::panel_note(ui, "No phase-noise trace in this run.");
        return;
    };
    let Some(first) = traces.first() else {
        crate::presentation::panel_note(ui, "No phase-noise trace in this run.");
        return;
    };
    // The carrier names which oscillation L(f) is referred to; each trace
    // states its own when they differ.
    let one_carrier = traces
        .iter()
        .all(|trace| trace.carrier_frequency_hz == first.carrier_frequency_hz);
    let mut rows: Vec<(String, String)> = Vec::new();
    if one_carrier {
        rows.push((
            "Carrier".to_owned(),
            frequency_readout(quantities, first.carrier_frequency_hz, digits),
        ));
    } else {
        rows.extend(traces.iter().map(|trace| {
            (
                format!("Carrier · {}", trace.label),
                frequency_readout(quantities, trace.carrier_frequency_hz, digits),
            )
        }));
    }
    if let Some(band) = first.sweep().and_then(|sweep| settings.band_within(sweep)) {
        rows.push((
            "Band".to_owned(),
            format!(
                "{} \u{2013} {}",
                frequency_readout(quantities, band.0, digits),
                frequency_readout(quantities, band.1, digits)
            ),
        ));
    }
    let rows: Vec<(&str, String, bool)> = rows
        .iter()
        .map(|(name, value)| (name.as_str(), value.clone(), false))
        .collect();
    crate::presentation::stat_table(ui, &rows);

    let figures: Vec<Option<BandFigures>> = traces
        .iter()
        .map(|trace| band_figures(trace, settings))
        .collect();
    let spots = spot_rows(traces, &settings.spots, quantities, digits);
    let band = if figures.iter().any(Option::is_some) {
        band_rows(&figures, quantities, digits)
    } else {
        Vec::new()
    };
    // The spot table and the band table stand one over the other, so they
    // share one width per column and read as one grid.
    let spot_heads: Vec<String> = traces
        .iter()
        .map(|trace| {
            if traces.len() > 1 {
                trace.label.to_owned()
            } else {
                "L(f) (dBc/Hz)".to_owned()
            }
        })
        .collect();
    let band_heads: Vec<String> = traces.iter().map(|trace| trace.label.to_owned()).collect();
    let widths = stacked_widths(
        ui,
        [
            (("Offset", true), &spot_heads, &spots),
            (("Figure", false), &band_heads, &band),
        ],
    );

    if !spots.is_empty() {
        // Several traces each head a column by name, so the unit they share
        // is stated once over the table.
        let unit = (traces.len() > 1).then_some("L(f) in dBc/Hz");
        section_header(ui, "Spots", unit);
        let mut columns = vec![Column::number("Offset", widths[0])];
        columns.extend(
            spot_heads
                .iter()
                .zip(&widths[1..])
                .map(|(head, width)| Column::name(head, *width).align(Align::Right)),
        );
        DataTable::new("rspice.results.phase-noise.spots", &columns, spots.len())
            .freeze(1)
            .show(ui, |index| spots[index].clone());
    }

    if !band.is_empty() {
        section_header(ui, "Integrated", None);
        let mut columns = vec![Column::words("Figure", widths[0]).unsortable()];
        columns.extend(
            band_heads
                .iter()
                .zip(&widths[1..])
                .map(|(head, width)| Column::name(head, *width).align(Align::Right).unsortable()),
        );
        DataTable::new("rspice.results.phase-noise.band", &columns, band.len())
            .freeze(1)
            .show(ui, |index| band[index].clone());
    }

    if device_shares.is_empty() {
        return;
    }
    section_header(ui, "Device noise shares", None);
    let shares: Vec<Vec<Cell>> = device_shares
        .iter()
        .map(|&(device, share)| {
            vec![
                Cell::text(device).sort_by(SortKey::Text(device.to_owned())),
                Cell::number(crate::presentation::fixed_places(
                    share,
                    crate::presentation::SHARE_PLACES,
                ))
                .sort_by(SortKey::Number(share)),
            ]
        })
        .collect();
    let fitting = [
        ("Device", Align::Left, false),
        ("Share (%)", Align::Right, true),
    ]
    .into_iter()
    .enumerate()
    .map(|(column, (head, align, number))| Fitting {
        head: head.to_owned(),
        tooltip: None,
        align,
        cells: shares
            .iter()
            .map(|row| (row[column].text.clone(), number))
            .collect(),
    })
    .collect::<Vec<_>>();
    let widths = crate::presentation::details_column_widths(
        ui,
        &[&fitting],
        &[true, false],
        crate::presentation::details_table_room(ui),
    );
    let columns = [
        Column::name("Device", widths[0]),
        Column::number("Share (%)", widths[1]),
    ];
    DataTable::new("rspice.results.phase-noise.shares", &columns, shares.len())
        .show(ui, |index| shares[index].clone());
}

/// A table stacked with another: its first column's head and whether that
/// column holds numbers, the traces' heads, and its rows.
type StackedTable<'a> = ((&'a str, bool), &'a [String], &'a [Vec<Cell>]);

/// The widths of the spot and band tables' columns, one per column for
/// both: the first as wide as the widest offset or figure, fixed; each
/// trace's as wide as its widest value, and as its name when the pane
/// allows, sharing what the pane leaves.
fn stacked_widths(ui: &Ui, tables: [StackedTable<'_>; 2]) -> Vec<f32> {
    let fitting = |((first, first_numbers), heads, rows): StackedTable<'_>| {
        std::iter::once(first.to_owned())
            .chain(heads.iter().cloned())
            .enumerate()
            .map(|(column, head)| {
                let numbers = column > 0 || first_numbers;
                Fitting {
                    head,
                    tooltip: None,
                    align: if numbers { Align::Right } else { Align::Left },
                    cells: rows
                        .iter()
                        .map(|row| (row[column].text.clone(), numbers))
                        .collect(),
                }
            })
            .collect::<Vec<_>>()
    };
    let [spots, band] = tables.map(fitting);
    let mut flexible = vec![true; spots.len()];
    flexible[0] = false;
    crate::presentation::details_column_widths(
        ui,
        &[&spots, &band],
        &flexible,
        crate::presentation::details_table_room(ui),
    )
}

/// One row per spot: the offset, then L(f) of each trace there.
fn spot_rows(
    traces: &[PhaseNoiseTrace<'_>],
    spots: &[f64],
    quantities: &QuantityPresentationPolicy,
    digits: usize,
) -> Vec<Vec<Cell>> {
    spots
        .iter()
        .map(|&spot| {
            let mut cells = vec![
                Cell::number(frequency_readout(quantities, spot, digits))
                    .sort_by(SortKey::Number(spot)),
            ];
            cells.extend(traces.iter().map(|trace| {
                spot_level_at(trace.offset_hz, trace.level_dbc_per_hz, spot)
                    .map_or_else(Cell::absent, |level| {
                        Cell::number(decibels(level)).sort_by(SortKey::Number(level))
                    })
            }));
            cells
        })
        .collect()
}

/// One row per figure over the band, one column per trace: integrated L(f),
/// rms phase error and jitter, and Jc and Jcc where the sweep reaches half
/// the carrier. A figure's unit heads its row, so every figure shows
/// without a sideways scroll.
fn band_rows(
    figures: &[Option<BandFigures>],
    quantities: &QuantityPresentationPolicy,
    digits: usize,
) -> Vec<Vec<Cell>> {
    let cycles = figures
        .iter()
        .flatten()
        .any(|figure| figure.cycle.is_some());
    let (angle_scale, angle_unit) = match quantities.angle_display {
        AngleDisplay::Degrees => (180.0 / std::f64::consts::PI, "°"),
        AngleDisplay::Radians => (1.0, "rad"),
    };
    type Read = fn(&BandFigures, f64) -> Option<f64>;
    // A level in dBc prints to its hundredth; the rest at the reader's digits.
    let mut rows: Vec<(String, &str, Read)> = vec![
        ("L(f) (dBc)".to_owned(), "dBc", |figure, _| {
            Some(figure.integrated.integrated_dbc())
        }),
        (
            format!("Phase ({angle_unit} rms)"),
            angle_unit,
            |figure, scale| Some(figure.integrated.phase_error_rms_rad() * scale),
        ),
        ("Jitter (s rms)".to_owned(), "s", |figure, _| {
            Some(figure.jitter_s)
        }),
    ];
    if cycles {
        rows.push(("Jc (s rms)".to_owned(), "s", |figure, _| {
            figure.cycle.map(|cycle| cycle.cycle_s)
        }));
        rows.push(("Jcc (s rms)".to_owned(), "s", |figure, _| {
            figure.cycle.map(|cycle| cycle.cycle_to_cycle_s)
        }));
    }
    rows.iter()
        .map(|(head, unit, read)| {
            let mut cells = vec![Cell::text(head.clone())];
            cells.extend(figures.iter().map(|figure| {
                figure
                    .as_ref()
                    .and_then(|figure| read(figure, angle_scale))
                    .map_or_else(Cell::absent, |value| {
                        Cell::number(if *unit == "dBc" {
                            decibels(value)
                        } else {
                            plain(value, unit, digits)
                        })
                    })
            }));
            cells
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A sweep opens with a spot at each decade strictly inside it.
    #[test]
    fn a_sweep_opens_with_a_spot_at_each_decade_inside_it() {
        assert_eq!(
            PhaseNoiseSettings::for_sweep((1.0e3, 1.0e7)).spots,
            [1.0e4, 1.0e5, 1.0e6]
        );
        assert_eq!(
            PhaseNoiseSettings::for_sweep((300.0, 2.0e5)).spots,
            [1.0e3, 1.0e4, 1.0e5]
        );
    }

    /// The reader's band is kept inside the sweep; a band wholly outside
    /// it integrates nothing.
    #[test]
    fn the_band_is_kept_inside_the_sweep() {
        let mut settings = PhaseNoiseSettings::for_sweep((1.0e3, 1.0e7));
        assert_eq!(settings.band_within((1.0e3, 1.0e7)), Some((1.0e3, 1.0e7)));
        settings.band = Some((1.0e2, 1.0e5));
        assert_eq!(settings.band_within((1.0e3, 1.0e7)), Some((1.0e3, 1.0e5)));
        settings.band = Some((2.0e7, 3.0e7));
        assert_eq!(settings.band_within((1.0e3, 1.0e7)), None);
    }

    /// Offsets read as typed, in SI as the readout prints them, sorted and
    /// without repeats; one entry that is not a frequency refuses the list.
    #[test]
    fn spot_offsets_read_as_typed() {
        assert_eq!(
            parse_offsets("1M, 10k; 100 kHz, 10k"),
            Some(vec![1.0e4, 1.0e5, 1.0e6])
        );
        assert_eq!(parse_offsets(""), Some(vec![]));
        assert_eq!(parse_offsets("10k, fast"), None);
        assert_eq!(parse_offsets("-10k"), None);
        assert_eq!(offsets_text(&[1.0e4, 1.0e5, 2.5e6], 4), "10k, 100k, 2.5M");
    }
}
