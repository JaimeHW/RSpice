//! The coefficient spectrum sheet's bar: the level its lines are drawn at
//! and the amplitude it reads, the resistance a dBm is read into, and the
//! phase and waveform strips behind Display.

use egui::Ui;
use rspice_results::harmonic_spectrum::settings::{
    CoefficientAmplitude, CoefficientScale, CoefficientSettings,
};
use rspice_ui_kit::widgets::{Button, select_with_response};

use crate::chrome::ResultBarMetrics;
use crate::chrome::bars::bar_label;

/// Width of the scale and amplitude selects: their longest choice.
const SELECT_WIDTH: f32 = 104.0;
/// Width of the reference resistance field.
const OHMS_WIDTH: f32 = 64.0;

/// The sheet's controls, left in its bar. Returns the settings the reader
/// set, when they changed one, for the host to keep in the sheet's view.
pub fn domain_bar(ui: &mut Ui, held: &CoefficientSettings) -> Option<CoefficientSettings> {
    let mut settings = held.clone();
    let control = ResultBarMetrics::of(ui).instrument_control;

    bar_label(ui, "Level");
    let scales = CoefficientScale::ALL
        .iter()
        .map(|scale| scale.label().to_owned())
        .collect::<Vec<_>>();
    let scale = select_with_response(
        ui,
        "rspice.harmonic-balance.scale",
        "Level",
        settings.scale.label(),
        &scales,
        SELECT_WIDTH,
    );
    if let Some(picked) = scale.picked {
        settings.scale = CoefficientScale::ALL[picked];
    }

    // A power and a ratio of peaks read the same either way, so the
    // amplitude only matters to a magnitude or a dB level.
    let amplitude_matters = matches!(
        settings.scale,
        CoefficientScale::Magnitude | CoefficientScale::Db
    );
    ui.add_space(8.0);
    ui.add_enabled_ui(amplitude_matters, |ui| {
        bar_label(ui, "Amplitude");
        let amplitudes = CoefficientAmplitude::ALL
            .iter()
            .map(|amplitude| amplitude.label().to_owned())
            .collect::<Vec<_>>();
        let amplitude = select_with_response(
            ui,
            "rspice.harmonic-balance.amplitude",
            "Amplitude",
            settings.amplitude.label(),
            &amplitudes,
            SELECT_WIDTH,
        );
        amplitude
            .response
            .on_disabled_hover_text("A power or a ratio reads the same at the peak or as rms");
        if let Some(picked) = amplitude.picked {
            settings.amplitude = CoefficientAmplitude::ALL[picked];
        }
    });

    if settings.scale == CoefficientScale::Dbm {
        ui.add_space(8.0);
        bar_label(ui, "Into");
        ui.add_sized(
            [OHMS_WIDTH, control],
            egui::DragValue::new(&mut settings.reference_ohms)
                .range(1.0e-3..=1.0e9)
                .speed(1.0)
                .suffix(" Ω"),
        )
        .on_hover_text("The resistance each voltage and current line delivers its power into");
    }

    ui.add_space(8.0);
    let display = Button::new("Display")
        .menu()
        .accessible_label("Spectrum display options")
        .show(ui);
    egui::Popup::menu(&display)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            ui.checkbox(&mut settings.phase, "Phase")
                .on_hover_text("Each line's phase, on a strip of its own");
            ui.checkbox(&mut settings.waveform, "Waveform")
                .on_hover_text("The time waveform the lines add up to, over two periods");
        });

    (settings != *held && settings.validate().is_ok()).then_some(settings)
}
