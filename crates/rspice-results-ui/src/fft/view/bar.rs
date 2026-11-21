//! The spectrum sheet's bar: the signal, the window and its β and the
//! amplitude, with the record, its samples and what the figures are measured
//! over behind Display, as ViVA's spectrum assistant and Eldo's FFT set them.

use egui::Ui;
use rspice_results::fft::settings::{
    MAX_SPECTRUM_AVERAGES, SPECTRUM_HARMONICS, SPECTRUM_PADDING, SpectrumAmplitude,
    SpectrumResampling, SpectrumSampling, SpectrumScale, SpectrumSettings, SpectrumWindow,
};
use rspice_results::fft::window::KaiserBeta;
use rspice_ui_kit::widgets::{Button, select_with_response};
use rspice_vocabulary::quantity::engineering::{
    EngineeringPrecision, format_engineering_value_with, parse_engineering_value,
};

use super::super::FftState;
use super::super::pipeline::{MAX_REFERENCE_RESAMPLE_POINTS, MIN_FFT_SAMPLES};
use crate::chrome::ResultBarMetrics;
use crate::chrome::bars::bar_label;

/// Width of the source, window and amplitude selects: the longest choice of
/// the window's, so the three stand at one width.
const SELECT_WIDTH: f32 = 140.0;
/// Width of a number typed in the bar or the Display menu.
const NUMBER_WIDTH: f32 = 72.0;
/// The source select's first entry: follow the first voltage plotted.
const FIRST_PLOTTED: &str = "First plotted voltage";

/// What the reader may take a record from: the A and B cursors on the
/// transient, when both are placed.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct RecordCursors {
    pub a: Option<f64>,
    pub b: Option<f64>,
}

impl RecordCursors {
    fn record(self) -> Option<(f64, f64)> {
        let (a, b) = (self.a?, self.b?);
        (a.is_finite() && b.is_finite() && a != b).then(|| (a.min(b), a.max(b)))
    }
}

/// The sheet's controls, left in its bar. `sources` are the signals the
/// analysis saved, in the order it saved them. Returns the settings the
/// reader set, when they changed one, for the host to keep in the sheet's
/// view.
pub fn domain_bar(
    ui: &mut Ui,
    fft: &FftState,
    sources: &[String],
    cursors: RecordCursors,
) -> Option<SpectrumSettings> {
    let before = fft.settings();
    let mut settings = before.clone();
    let control = ResultBarMetrics::of(ui).instrument_control;

    bar_label(ui, "Source");
    let mut names = Vec::with_capacity(sources.len() + 1);
    names.push(FIRST_PLOTTED.to_owned());
    names.extend(sources.iter().cloned());
    let shown = fft.source().unwrap_or(FIRST_PLOTTED);
    let source = select_with_response(
        ui,
        "rspice.fft.source",
        "Source",
        shown,
        &names,
        SELECT_WIDTH,
    );
    if settings.source.is_none() {
        source
            .response
            .on_hover_text("Follows the first voltage plotted");
    }
    if let Some(picked) = source.picked {
        settings.source = (picked > 0).then(|| names[picked].clone());
    }

    ui.add_space(8.0);
    bar_label(ui, "Window");
    let windows = SpectrumWindow::ALL
        .iter()
        .map(|window| window.label().to_owned())
        .collect::<Vec<_>>();
    let window = select_with_response(
        ui,
        "rspice.fft.window",
        "Window",
        settings.window.label(),
        &windows,
        SELECT_WIDTH,
    );
    if let Some(picked) = window.picked {
        settings.window = SpectrumWindow::ALL[picked];
    }
    if settings.window == SpectrumWindow::Kaiser {
        ui.add_space(4.0);
        bar_label(ui, "β");
        ui.add_sized(
            [NUMBER_WIDTH * 0.66, control],
            egui::DragValue::new(&mut settings.kaiser_beta)
                .range(0.0..=KaiserBeta::MAX)
                .speed(0.25)
                .max_decimals(2),
        )
        .on_hover_text(format!(
            "The Kaiser window's β. At {} its sidelobes sit below double-precision rounding",
            KaiserBeta::DEFAULT.get()
        ));
    }

    ui.add_space(8.0);
    bar_label(ui, "Normalization");
    let amplitudes = [
        crate::fft::data::SpectrumNormalization::Peak,
        crate::fft::data::SpectrumNormalization::Rms,
    ];
    let amplitude_names = amplitudes
        .iter()
        .map(|amplitude| amplitude.display_name().to_owned())
        .collect::<Vec<_>>();
    let amplitude = select_with_response(
        ui,
        "rspice.fft.normalization",
        "Normalization",
        match settings.amplitude {
            SpectrumAmplitude::Peak => amplitudes[0].display_name(),
            SpectrumAmplitude::Rms => amplitudes[1].display_name(),
        },
        &amplitude_names,
        SELECT_WIDTH,
    );
    if let Some(picked) = amplitude.picked {
        settings.amplitude = SpectrumAmplitude::ALL[picked];
    }

    ui.add_space(8.0);
    let held = fft.source_cache.as_ref().map(|source| source.record);
    display_menu(ui, &mut settings, cursors, held);
    (settings != before && settings.validate().is_ok()).then_some(settings)
}

/// The record, its samples and what the figures are measured over: the
/// settings a reader sets once per measurement, behind one menu.
fn display_menu(
    ui: &mut Ui,
    settings: &mut SpectrumSettings,
    cursors: RecordCursors,
    held: Option<(f64, f64)>,
) {
    let display = Button::new("Display")
        .menu()
        .accessible_label("Spectrum display options")
        .show(ui);
    egui::Popup::menu(&display)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            egui::Grid::new("rspice.fft.display")
                .num_columns(2)
                .spacing([12.0, 6.0])
                .show(ui, |ui| {
                    record_rows(ui, settings, cursors, held);
                    sampling_rows(ui, settings);
                    measurement_rows(ui, settings);
                    drawing_rows(ui, settings);
                });
        });
}

fn record_rows(
    ui: &mut Ui,
    settings: &mut SpectrumSettings,
    cursors: RecordCursors,
    held: Option<(f64, f64)>,
) {
    bar_label(ui, "Record");
    ui.horizontal(|ui| {
        let mut whole = settings.record.is_none();
        if ui
            .checkbox(&mut whole, "Whole run")
            .on_hover_text("One period from the run's first sample to its last")
            .changed()
        {
            // Typing a record starts from the one the spectrum read.
            settings.record = if whole { None } else { held };
        }
        let from = cursors.record();
        let button = ui
            .add_enabled(from.is_some(), egui::Button::new("From A–B"))
            .on_hover_text("The record between the transient's A and B cursors")
            .on_disabled_hover_text("Place the A and B cursors on the transient first");
        if button.clicked() {
            settings.record = from;
        }
    });
    ui.end_row();
    if let Some((start, stop)) = settings.record.as_mut() {
        bar_label(ui, "Start, stop");
        ui.horizontal(|ui| {
            if let Some(value) = si_field(ui, "rspice.fft.record.start", Some(*start), "s") {
                *start = value.unwrap_or(0.0);
            }
            if let Some(value) = si_field(ui, "rspice.fft.record.stop", Some(*stop), "s") {
                *stop = value.unwrap_or(*stop);
            }
        });
        ui.end_row();
    }
}

fn sampling_rows(ui: &mut Ui, settings: &mut SpectrumSettings) {
    bar_label(ui, "Samples");
    ui.horizontal(|ui| {
        let mut mode = match settings.sampling {
            SpectrumSampling::Auto => 0,
            SpectrumSampling::Count(_) => 1,
            SpectrumSampling::Rate(_) => 2,
        };
        let before = mode;
        ui.radio_value(&mut mode, 0, "Auto")
            .on_hover_text("As many as the record holds, resampled onto a uniform grid");
        ui.radio_value(&mut mode, 1, "Count");
        ui.radio_value(&mut mode, 2, "Rate");
        if mode != before {
            settings.sampling = match mode {
                1 => SpectrumSampling::Count(4096),
                2 => SpectrumSampling::Rate(1.0e6),
                _ => SpectrumSampling::Auto,
            };
        }
        match &mut settings.sampling {
            SpectrumSampling::Count(count) => {
                ui.add_sized(
                    [NUMBER_WIDTH, ui.spacing().interact_size.y],
                    egui::DragValue::new(count)
                        .range(MIN_FFT_SAMPLES..=MAX_REFERENCE_RESAMPLE_POINTS)
                        .speed(16.0),
                );
            }
            SpectrumSampling::Rate(rate) => {
                if let Some(Some(value)) = si_field(ui, "rspice.fft.rate", Some(*rate), "Hz") {
                    *rate = value;
                }
            }
            SpectrumSampling::Auto => {}
        }
    });
    ui.end_row();

    bar_label(ui, "Resampling");
    ui.horizontal(|ui| {
        ui.radio_value(
            &mut settings.resampling,
            SpectrumResampling::Interpolate,
            "Interpolate",
        );
        ui.radio_value(
            &mut settings.resampling,
            SpectrumResampling::SampleAndHold,
            "Sample and hold",
        );
    });
    ui.end_row();

    bar_label(ui, "Padding");
    ui.horizontal(|ui| {
        for factor in SPECTRUM_PADDING {
            ui.radio_value(&mut settings.padding, factor, format!("×{factor}"));
        }
    })
    .response
    .on_hover_text(
        "Zeros appended to draw the spectrum on a finer grid; the figures read the record",
    );
    ui.end_row();

    bar_label(ui, "Averages");
    ui.add_sized(
        [NUMBER_WIDTH, ui.spacing().interact_size.y],
        egui::DragValue::new(&mut settings.averages)
            .range(1..=MAX_SPECTRUM_AVERAGES)
            .speed(0.1),
    )
    .on_hover_text("Equal records the record is cut into, whose powers are averaged");
    ui.end_row();
}

fn measurement_rows(ui: &mut Ui, settings: &mut SpectrumSettings) {
    bar_label(ui, "Fundamental");
    if let Some(value) = si_field(ui, "rspice.fft.fundamental", settings.fundamental, "Hz") {
        settings.fundamental = value;
    }
    ui.end_row();

    bar_label(ui, "Harmonics");
    ui.add_sized(
        [NUMBER_WIDTH, ui.spacing().interact_size.y],
        egui::DragValue::new(&mut settings.harmonics)
            .range(SPECTRUM_HARMONICS)
            .speed(0.2)
            .prefix("HD2–HD"),
    );
    ui.end_row();

    bar_label(ui, "Signal bins");
    ui.horizontal(|ui| {
        let mut auto = settings.signal_bins.is_none();
        if ui
            .checkbox(&mut auto, "Main lobe")
            .on_hover_text("The bins either side of a tone the window's main lobe spans")
            .changed()
        {
            settings.signal_bins = if auto { None } else { Some(3) };
        }
        if let Some(bins) = settings.signal_bins.as_mut() {
            ui.add_sized(
                [NUMBER_WIDTH * 0.66, ui.spacing().interact_size.y],
                egui::DragValue::new(bins).range(0..=64).speed(0.1),
            );
        }
    });
    ui.end_row();

    bar_label(ui, "Band");
    ui.horizontal(|ui| {
        let (low, high) = settings.band.unwrap_or((0.0, f64::NAN));
        let low_field = si_field(ui, "rspice.fft.band.low", settings.band.map(|_| low), "Hz");
        let high_field = si_field(
            ui,
            "rspice.fft.band.high",
            settings.band.map(|_| high),
            "Hz",
        );
        match (low_field, high_field) {
            (_, Some(None)) => settings.band = None,
            (_, Some(Some(high))) => settings.band = Some((low.max(0.0), high)),
            (Some(low), None) if settings.band.is_some() => {
                settings.band = Some((low.unwrap_or(0.0), high));
            }
            _ => {}
        }
    })
    .response
    .on_hover_text(
        "The band the noise, distortion and spurs are counted over; empty is DC to Nyquist",
    );
    ui.end_row();
}

fn drawing_rows(ui: &mut Ui, settings: &mut SpectrumSettings) {
    bar_label(ui, "Scale");
    ui.horizontal(|ui| {
        for scale in SpectrumScale::ALL {
            ui.radio_value(&mut settings.scale, scale, scale.label());
        }
    });
    ui.end_row();

    bar_label(ui, "Axes");
    ui.horizontal(|ui| {
        ui.checkbox(&mut settings.log_frequency, "Log frequency");
        ui.checkbox(&mut settings.phase, "Phase");
    });
    ui.end_row();
}

/// One quantity typed in engineering notation, its unit beside it, committed
/// when the field loses focus or the reader presses Enter: `Some(None)` for
/// an emptied field, `Some(Some(value))` for a number read, `None` while
/// nothing was committed. The field shows the value as the deck's notation
/// writes it, `1Meg` and `10u`, so what it shows reads back as itself.
fn si_field(ui: &mut Ui, id: &str, value: Option<f64>, unit: &str) -> Option<Option<f64>> {
    let id = ui.make_persistent_id(id);
    let shown = value.map_or_else(String::new, |value| {
        format_engineering_value_with(value, EngineeringPrecision::UpTo(6))
    });
    let mut text = ui
        .data_mut(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| shown.clone());
    let response = ui.add_sized(
        [NUMBER_WIDTH, ui.spacing().interact_size.y],
        egui::TextEdit::singleline(&mut text),
    );
    bar_label(ui, unit);
    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text));
        return None;
    }
    ui.data_mut(|data| data.remove::<String>(id));
    if !response.lost_focus() || text == shown {
        return None;
    }
    if text.trim().is_empty() {
        return Some(None);
    }
    parse_engineering_value(&text).ok().map(Some)
}
