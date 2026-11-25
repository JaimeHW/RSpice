//! The eye sheet's bar: the signal, the clock that folds it, its levels and
//! the error rate it is read at, with the mask behind Display, as ViVA's eye
//! diagram assistant and ADS's eye probe set them. A bar too narrow for every
//! control folds the error rate, then the levels, then the clock into the
//! Display menu, so nothing runs under the mask verdict or the export button.

use egui::Ui;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens;
use rspice_ui_kit::widgets::{Button, SegmentedWidth, segmented, select_with_response};
use rspice_vocabulary::quantity::engineering::{
    EngineeringPrecision, format_engineering_value_with, parse_engineering_value,
};

use super::state::{EyeMask, MaskShape};
use super::{
    EyeClockSignal, EyeDiagramState, EyeSettings, EyeTimebase, MAX_EYE_LEVELS, MIN_EYE_LEVELS,
    OPENING_RATES, parse_eye_timebase,
};
use crate::chrome::bars::bar_label;

/// Width of the signal and clock selects.
const SELECT_WIDTH: f32 = 140.0;
/// Width of the clock kind and error-rate selects.
const SHORT_SELECT_WIDTH: f32 = 96.0;
/// Width of a number typed in the bar or the Display menu.
const NUMBER_WIDTH: f32 = 72.0;
/// The gap between two groups of the bar.
const GROUP_GAP: f32 = 8.0;
/// What the Display button is taken to need before it was first drawn.
const DISPLAY_WIDTH_GUESS: f32 = 80.0;
/// The signal select's first entry: fold what the spectrum reads.
const AS_SPECTRUM: &str = "As the spectrum";
/// The clocks an eye is folded by.
const CLOCKS: [&str; 3] = ["Recovered", "Ideal", "Signal"];
/// The level choices, as the segmented control spells them.
const LEVEL_CHOICES: [&str; 3] = ["2", "3", "4"];

/// The sheet's controls, left in its bar. `sources` are the signals the
/// analysis saved, in the order it saved them. Returns the settings the
/// reader set, when they changed one, for the host to keep for the result.
pub fn domain_bar(
    ui: &mut Ui,
    eye: &EyeDiagramState,
    sources: &[String],
    significant_digits: usize,
) -> Option<EyeSettings> {
    let before = eye.settings().clone();
    let mut settings = before.clone();
    let fold = Fold::measure(ui, &settings);

    bar_label(ui, "Signal");
    let mut names = Vec::with_capacity(sources.len() + 1);
    names.push(AS_SPECTRUM.to_owned());
    names.extend(sources.iter().cloned());
    let shown = settings.fold.source.as_deref().unwrap_or(AS_SPECTRUM);
    let source = select_with_response(
        ui,
        "rspice.eye.source",
        "Signal",
        shown,
        &names,
        SELECT_WIDTH,
    );
    if settings.fold.source.is_none() {
        source
            .response
            .on_hover_text("Folds the signal the spectrum reads");
    }
    if let Some(picked) = source.picked {
        settings.fold.source = (picked > 0).then(|| names[picked].clone());
    }

    if !fold.clock {
        ui.add_space(GROUP_GAP);
        bar_label(ui, "Clock");
        clock_controls(ui, &mut settings, sources, significant_digits);
    }

    if !fold.levels {
        ui.add_space(GROUP_GAP);
        bar_label(ui, "Levels");
        levels_control(ui, &mut settings);
    }
    if !fold.rate {
        ui.add_space(GROUP_GAP);
        bar_label(ui, "BER");
        rate_select(ui, &mut settings);
    }

    ui.add_space(GROUP_GAP);
    display_menu(ui, eye, &mut settings, sources, significant_digits, fold);
    settings.mask.clear_test();
    let mut held = before;
    held.mask.clear_test();
    (settings != held && settings.validate().is_ok()).then_some(settings)
}

/// What the eye's mask test found, right in the bar: the samples inside it
/// and how far it clears the eye. Empty while the mask is off.
pub fn outputs(ui: &mut Ui, eye: &EyeDiagramState, significant_digits: usize) {
    let settings = eye.settings();
    if !settings.show_mask || eye.data.traces.is_empty() {
        return;
    }
    let mask = &settings.mask;
    let clearance = mask.clearance.map_or_else(
        || "—".to_owned(),
        |clearance| format!("{} %", significant(100.0 * clearance, significant_digits)),
    );
    let verdict = if mask.violation_count > 0 {
        format!("{} in mask · {clearance}", mask.violation_count)
    } else {
        format!("Mask clear · {clearance}")
    };
    let tone = if mask.violation_count > 0 {
        tokens::Tokens::get(ui.ctx()).color.err
    } else {
        tokens::Tokens::get(ui.ctx()).color.text_dim
    };
    ui.label(
        egui::RichText::new(verdict)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(tone),
    )
    .on_hover_text(format!(
        "{} of {} samples fall inside the mask; it clears the eye by {clearance}",
        mask.violation_count, mask.total_samples
    ));
}

fn significant(value: f64, digits: usize) -> String {
    rspice_ui_kit::plot::fmt_column_reading(value, "", digits)
        .trim()
        .to_owned()
}

/// The clock that folds the eye and what it needs: an ideal clock's rate and
/// offset, or a clock signal and its threshold.
fn clock_controls(
    ui: &mut Ui,
    settings: &mut EyeSettings,
    sources: &[String],
    significant_digits: usize,
) {
    let kind = if settings.fold.clock.is_some() {
        2
    } else if matches!(settings.fold.timebase, EyeTimebase::Explicit { .. }) {
        1
    } else {
        0
    };
    let names: Vec<String> = CLOCKS.iter().map(|&name| name.to_owned()).collect();
    let picked = select_with_response(
        ui,
        "rspice.eye.clock",
        "Clock",
        CLOCKS[kind],
        &names,
        SHORT_SELECT_WIDTH,
    );
    let hint = match kind {
        0 => "Recovers the bit period from the signal's own crossings",
        1 => "Folds at the rate you state, from the offset you state",
        _ => "Starts every acquisition at a rising edge of a clock signal",
    };
    picked.response.on_hover_text(hint);
    if let Some(choice) = picked.picked
        && choice != kind
    {
        settings.fold.clock = None;
        settings.fold.offset = None;
        match choice {
            0 => settings.fold.timebase = EyeTimebase::Auto,
            1 => {
                // Start from the period on screen, which the reader then edits.
                let unit_interval = match settings.fold.timebase {
                    EyeTimebase::Explicit { unit_interval } => unit_interval,
                    EyeTimebase::Auto => 1e-9,
                };
                settings.fold.timebase = EyeTimebase::Explicit { unit_interval };
            }
            _ => {
                settings.fold.timebase = EyeTimebase::Auto;
                settings.fold.clock = sources.first().map(|signal| EyeClockSignal {
                    signal: signal.clone(),
                    threshold: 0.0,
                });
            }
        }
    }

    if let EyeTimebase::Explicit { unit_interval } = settings.fold.timebase
        && settings.fold.clock.is_none()
    {
        let shown = rspice_ui_kit::plot::fmt_column_reading(unit_interval, "s", significant_digits);
        if let Some(text) = text_field(ui, "rspice.eye.rate", &shown, "2.5G · 400p")
            && let Ok(unit_interval) = parse_eye_timebase(&text)
        {
            settings.fold.timebase = EyeTimebase::Explicit { unit_interval };
        }
        bar_label(ui, "from");
        if let Some(offset) = number_field(ui, "rspice.eye.offset", settings.fold.offset, "s") {
            settings.fold.offset = offset;
        }
    }

    if let Some(clock) = settings.fold.clock.as_mut() {
        let picked = select_with_response(
            ui,
            "rspice.eye.clock-signal",
            "Clock signal",
            &clock.signal,
            sources,
            SELECT_WIDTH,
        );
        if let Some(index) = picked.picked {
            clock.signal = sources[index].clone();
        }
        bar_label(ui, "at");
        if let Some(Some(threshold)) =
            number_field(ui, "rspice.eye.clock-threshold", Some(clock.threshold), "V")
        {
            clock.threshold = threshold;
        }
    }
}

fn levels_control(ui: &mut Ui, settings: &mut EyeSettings) {
    let mut index =
        usize::from(settings.levels.clamp(MIN_EYE_LEVELS, MAX_EYE_LEVELS) - MIN_EYE_LEVELS);
    if segmented(
        ui,
        "rspice.eye.levels",
        &LEVEL_CHOICES,
        &mut index,
        SegmentedWidth::Natural,
    ) {
        settings.levels = MIN_EYE_LEVELS + u8::try_from(index).unwrap_or(0);
    }
}

/// An error rate as the bar spells it.
fn rate_label(rate: f64) -> String {
    format!("{rate:e}")
}

fn rate_select(ui: &mut Ui, settings: &mut EyeSettings) {
    let names: Vec<String> = OPENING_RATES.iter().map(|&rate| rate_label(rate)).collect();
    let picked = select_with_response(
        ui,
        "rspice.eye.ber",
        "Error rate",
        &rate_label(settings.opening_ber),
        &names,
        SHORT_SELECT_WIDTH,
    );
    picked
        .response
        .on_hover_text("The error rate the opening and total jitter are read at");
    if let Some(index) = picked.picked {
        settings.opening_ber = OPENING_RATES[index];
    }
}

/// Which of the bar's groups stand in the Display menu for want of room.
#[derive(Debug, Clone, Copy, Default)]
struct Fold {
    clock: bool,
    levels: bool,
    rate: bool,
}

impl Fold {
    /// What the bar's groups need against the width it has. Every control
    /// stands at a fixed width, so the words beside them are all there is
    /// to measure; the Display button is measured as it was last drawn.
    fn measure(ui: &Ui, settings: &EyeSettings) -> Self {
        let gap = ui.spacing().item_spacing.x;
        let font = theme::sans(tokens::FS_0, FontWeight::Regular);
        let words = |text: &str| {
            ui.painter()
                .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::PLACEHOLDER)
                .size()
                .x
                + gap
        };
        let source = words("Signal") + SELECT_WIDTH + gap;
        let mut clock = GROUP_GAP + words("Clock") + SHORT_SELECT_WIDTH + gap;
        if settings.fold.clock.is_some() {
            clock += SELECT_WIDTH + gap + words("at") + NUMBER_WIDTH + gap + words("V");
        } else if matches!(settings.fold.timebase, EyeTimebase::Explicit { .. }) {
            clock += NUMBER_WIDTH + gap + words("from") + NUMBER_WIDTH + gap + words("s");
        }
        let levels = GROUP_GAP + words("Levels") + 3.0 * 32.0 + gap;
        let rate = GROUP_GAP + words("BER") + SHORT_SELECT_WIDTH + gap;
        let display = GROUP_GAP
            + ui.data(|data| data.get_temp::<f32>(display_width_id(ui)))
                .unwrap_or(DISPLAY_WIDTH_GUESS);
        let room = ui.available_width();
        let rate_folds = room < source + clock + levels + rate + display;
        let levels_fold = rate_folds && room < source + clock + levels + display;
        Self {
            clock: levels_fold && room < source + clock + display,
            levels: levels_fold,
            rate: rate_folds,
        }
    }
}

fn display_width_id(ui: &Ui) -> egui::Id {
    ui.id().with("rspice.eye.display-width")
}

fn display_menu(
    ui: &mut Ui,
    eye: &EyeDiagramState,
    settings: &mut EyeSettings,
    sources: &[String],
    significant_digits: usize,
    fold: Fold,
) {
    let display = Button::new("Display")
        .menu()
        .accessible_label("Eye display options")
        .show(ui);
    let width = display.rect.width();
    ui.data_mut(|data| data.insert_temp(display_width_id(ui), width));
    egui::Popup::menu(&display)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            egui::Grid::new("rspice.eye.display")
                .num_columns(2)
                .spacing([12.0, 6.0])
                .show(ui, |ui| {
                    if fold.clock {
                        bar_label(ui, "Clock");
                        ui.horizontal(|ui| {
                            clock_controls(ui, settings, sources, significant_digits);
                        });
                        ui.end_row();
                    }
                    if fold.levels {
                        bar_label(ui, "Levels");
                        levels_control(ui, settings);
                        ui.end_row();
                    }
                    if fold.rate {
                        bar_label(ui, "BER");
                        rate_select(ui, settings);
                        ui.end_row();
                    }
                    mask_rows(ui, eye, settings);
                });
        });
}

/// The mask: whether it is drawn, its shape and size, where it sits and how
/// much larger it is tested, a fit to the eye, and its text file.
fn mask_rows(ui: &mut Ui, eye: &EyeDiagramState, settings: &mut EyeSettings) {
    bar_label(ui, "Mask");
    ui.checkbox(&mut settings.show_mask, "Draw and test");
    ui.end_row();

    let mask = &mut settings.mask;
    bar_label(ui, "Shape");
    let names: Vec<String> = MaskShape::ALL
        .iter()
        .map(|shape| shape.label().to_owned())
        .collect();
    let picked = select_with_response(
        ui,
        "rspice.eye.mask-shape",
        "Mask shape",
        mask.shape.label(),
        &names,
        SHORT_SELECT_WIDTH,
    );
    if let Some(index) = picked.picked {
        let shape = MaskShape::ALL[index];
        if shape == MaskShape::Custom && mask.vertices.len() < 3 {
            // A custom mask starts as the shape it replaces.
            mask.vertices = mask.outline(eye.mask_frame().amplitude);
        }
        mask.shape = shape;
    }
    ui.end_row();

    if mask.shape != MaskShape::Custom {
        bar_label(ui, "Size");
        ui.horizontal(|ui| {
            if let Some(Some(width)) =
                number_field(ui, "rspice.eye.mask-width", Some(mask.width_ui), "UI")
            {
                mask.width_ui = width;
            }
            if let Some(height) = number_field(ui, "rspice.eye.mask-height", mask.height, "V") {
                mask.height = height;
            }
        });
        ui.end_row();
    }
    if mask.shape == MaskShape::Hexagon {
        bar_label(ui, "Flat");
        ui.horizontal(|ui| {
            if let Some(Some(flat)) =
                number_field(ui, "rspice.eye.mask-flat", Some(mask.flat_ui), "UI")
            {
                mask.flat_ui = flat;
            }
        });
        ui.end_row();
    }

    bar_label(ui, "Offset");
    ui.horizontal(|ui| {
        if let Some(offset) =
            number_field(ui, "rspice.eye.mask-offset-ui", Some(mask.offset_ui), "UI")
        {
            mask.offset_ui = offset.unwrap_or(0.0);
        }
        if let Some(offset) = number_field(ui, "rspice.eye.mask-offset-v", Some(mask.offset_v), "V")
        {
            mask.offset_v = offset.unwrap_or(0.0);
        }
    });
    ui.end_row();

    bar_label(ui, "Margin");
    ui.horizontal(|ui| {
        if let Some(margin) =
            number_field(ui, "rspice.eye.mask-margin", Some(100.0 * mask.margin), "%")
        {
            mask.margin = margin.unwrap_or(0.0) / 100.0;
        }
    });
    ui.end_row();

    ui.label("");
    ui.horizontal(|ui| {
        let traces = || {
            eye.data
                .traces
                .iter()
                .map(|trace| (trace.time.as_slice(), trace.amplitude.as_slice()))
        };
        if ui
            .add_enabled(!eye.data.traces.is_empty(), egui::Button::new("Fit to eye"))
            .on_hover_text("The largest mask of this shape the eye clears")
            .clicked()
            && let Some(fitted) = mask.fitted(eye.mask_frame(), traces())
        {
            *mask = fitted;
        }
        if ui
            .button("Copy as text")
            .on_hover_text("The mask's vertices, one per line, for a mask file")
            .clicked()
        {
            ui.ctx().copy_text(mask.to_text(eye.mask_frame().amplitude));
        }
    });
    ui.end_row();

    bar_label(ui, "Vertices");
    vertices_field(ui, mask);
    ui.end_row();
}

/// A custom mask's vertices as the text file spells them, edited in place:
/// paste a mask file here to use it.
fn vertices_field(ui: &mut Ui, mask: &mut EyeMask) {
    let id = ui.make_persistent_id("rspice.eye.mask-vertices");
    let shown = if mask.shape == MaskShape::Custom {
        mask.vertices
            .iter()
            .map(|(x, y)| format!("{x} {y}"))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        String::new()
    };
    let mut text = ui
        .data_mut(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| shown.clone());
    let response = ui.add(
        egui::TextEdit::multiline(&mut text)
            .desired_rows(4)
            .desired_width(2.0 * NUMBER_WIDTH + 40.0)
            .font(theme::field_mono())
            .hint_text("unit intervals volts, one per line"),
    );
    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text));
        return;
    }
    ui.data_mut(|data| data.remove::<String>(id));
    if !response.lost_focus() || text == shown {
        return;
    }
    match EyeMask::from_text(&text) {
        Ok(read) => {
            mask.shape = MaskShape::Custom;
            mask.vertices = read.vertices;
        }
        Err(message) => {
            response.on_hover_text(message);
        }
    }
}

/// A text field that hands back what the reader typed when they commit it
/// with Enter or by leaving the field; `None` while nothing was committed.
fn text_field(ui: &mut Ui, id: &str, shown: &str, hint: &str) -> Option<String> {
    let id = ui.make_persistent_id(id);
    let mut text = ui
        .data_mut(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| shown.to_owned());
    let response = ui.add_sized(
        [NUMBER_WIDTH, ui.spacing().interact_size.y],
        egui::TextEdit::singleline(&mut text)
            .font(theme::field_mono())
            .hint_text(hint),
    );
    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text));
        return None;
    }
    ui.data_mut(|data| data.remove::<String>(id));
    (response.lost_focus() && text != shown).then_some(text)
}

/// A number typed in engineering notation, its unit beside it: `Some(None)`
/// for an emptied field, `Some(Some(value))` for a number read, `None` while
/// nothing was committed.
fn number_field(ui: &mut Ui, id: &str, value: Option<f64>, unit: &str) -> Option<Option<f64>> {
    let shown = value.map_or_else(String::new, |value| {
        format_engineering_value_with(value, EngineeringPrecision::UpTo(6))
    });
    let text = text_field(ui, id, &shown, "");
    bar_label(ui, unit);
    let text = text?;
    if text.trim().is_empty() {
        return Some(None);
    }
    parse_engineering_value(&text)
        .ok()
        .filter(|value| value.is_finite())
        .map(Some)
}
