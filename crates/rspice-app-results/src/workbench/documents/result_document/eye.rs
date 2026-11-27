//! Application coordination for the eye: its settings, kept per result, and
//! the sheet's bar, figures and picture.

use super::{
    ResultViewer,
    frame_work::{self, DatasetWalk},
};
use crate::io::file_exchange::{self, FileKind};
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use egui::Ui;
use rspice_results_ui::eye_diagram::bar::{self, MaskFile};
use rspice_results_ui::eye_diagram::state::{EyeMask, MaskShape};
use rspice_results_ui::eye_diagram::{EyeSettings, view};

/// A mask file, as the picker offers it: the vertices `EyeMask::to_text`
/// writes and `EyeMask::from_text` reads.
const MASK_FILE: FileKind = FileKind {
    label: "Eye mask",
    extensions: &["txt"],
    subject: "The mask file",
    fallback_name: "eye-mask.txt",
};

/// A mask file is read whatever its size, to what the browser page can hold.
fn mask_file_bytes() -> usize {
    usize::try_from(file_exchange::BROWSER_PAGE_BYTES).unwrap_or(usize::MAX)
}

/// The exchange a Save mask… press writes through.
fn mask_save_id() -> egui::Id {
    egui::Id::new("rspice.results.eye.mask-save")
}

/// The exchange an Open mask… press reads through.
fn mask_open_id() -> egui::Id {
    egui::Id::new("rspice.results.eye.mask-open")
}

/// Take in the settings kept for the result shown, so its levels, error rate
/// and mask are the ones the reader left.
fn adopt_settings(state: &mut ResultsParts<'_>) {
    let Some(owner) = state.model.active_specialized_viewer_cache_provenance() else {
        return;
    };
    let settings = state.results.eye_settings_for(owner);
    state.analysis.eye_diagram_state.adopt_settings(&settings);
}

/// The sheet's controls, left in its bar: the signal, the clock, the levels,
/// the error rate and the mask. What the reader sets is kept for the result.
pub(super) fn domain_bar(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    adopt_settings(state);
    let sources: Vec<String> = state
        .model
        .simulation
        .active_analysis()
        .map(|analysis| {
            analysis
                .waveforms
                .iter()
                .map(|waveform| waveform.name.clone())
                .collect()
        })
        .unwrap_or_default();
    let significant_digits = significant_digits(state.results_reads());
    let change = bar::domain_bar(
        ui,
        &state.analysis.eye_diagram_state,
        &sources,
        significant_digits,
    );
    if let Some(settings) = change.settings {
        keep_settings(state, settings);
    }
    if let Some(asked) = change.mask_file {
        ask_for_mask_file(ui.ctx(), state, asked);
    }
    take_mask_files(ui.ctx(), state);
    true
}

/// Show `settings` and keep them for the result shown.
fn keep_settings(state: &mut ResultsParts<'_>, settings: EyeSettings) {
    state.analysis.eye_diagram_state.adopt_settings(&settings);
    if let Some(owner) = state.model.active_specialized_viewer_cache_provenance() {
        state.results.set_eye_settings(owner, settings);
    }
}

/// Start the mask file exchange the reader asked for: the mask's vertices
/// written to a file they choose, or a file they choose read as a mask.
fn ask_for_mask_file(ctx: &egui::Context, state: &mut ResultsParts<'_>, asked: MaskFile) {
    let started = match asked {
        MaskFile::Save => {
            let eye = &state.analysis.eye_diagram_state;
            let text = eye.settings().mask.to_text(eye.mask_frame().amplitude);
            file_exchange::save_file(
                ctx,
                mask_save_id(),
                MASK_FILE,
                MASK_FILE.fallback_name.to_owned(),
                text.into_bytes(),
            )
        }
        MaskFile::Open => {
            file_exchange::open_file(ctx, mask_open_id(), MASK_FILE, mask_file_bytes())
        }
    };
    if let Err(refusal) = started {
        state.prompt(refusal);
    }
}

/// Take what a mask file exchange came to. A mask read draws and tests as
/// the custom mask its vertices outline, where the reader's margin and
/// offset place it; a file that is not a mask says why and changes nothing.
fn take_mask_files(ctx: &egui::Context, state: &mut ResultsParts<'_>) {
    match file_exchange::take_saved(ctx, mask_save_id()) {
        Some(Ok(Some(saved))) => state.prompt(format!("Saved the mask to {}", saved.name)),
        Some(Err(error)) => state.prompt(error),
        Some(Ok(None)) | None => {}
    }
    match file_exchange::take_opened(ctx, mask_open_id()) {
        Some(Ok(Some(opened))) => match EyeMask::from_text(&opened.text) {
            Ok(read) => {
                let mut settings = state.analysis.eye_diagram_state.settings().clone();
                settings.mask.shape = MaskShape::Custom;
                settings.mask.vertices = read.vertices;
                settings.show_mask = true;
                keep_settings(state, settings);
                state.prompt(format!("Opened the mask in {}", opened.name));
            }
            Err(error) => state.prompt(format!("{} is not a mask: {error}", opened.name)),
        },
        Some(Err(error)) => state.prompt(error),
        Some(Ok(None)) | None => {}
    }
}

/// What the sheet reads out right in its bar: the mask's verdict.
pub fn inline_actions(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    adopt_settings(state);
    let significant_digits = significant_digits(state.results_reads());
    bar::outputs(ui, &state.analysis.eye_diagram_state, significant_digits);
}

/// The reader's significant digits, which the eye's figures read at.
fn significant_digits(state: ResultsReads<'_>) -> usize {
    usize::from(
        state
            .presentation_policy()
            .displayed_significant_digits()
            .get(),
    )
}

/// The signal the eye folds, as the sheet names it.
fn selected_source(state: ResultsReads<'_>) -> String {
    let named = state
        .model
        .active_specialized_viewer_cache_provenance()
        .and_then(|owner| state.results.eye_fold_for(owner).source);
    named
        .or_else(|| state.analysis.fft_state.selected_source.clone())
        .unwrap_or_else(|| "the active trace".to_owned())
}

pub fn unavailable_hint(state: ResultsReads<'_>) -> Option<String> {
    view::unavailable_hint(&state.analysis.eye_diagram_state, &selected_source(state))
}

pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    adopt_settings(state);
    let source = selected_source(state.results_reads());
    let plot_view = state.results.session.plot_view(ResultViewer::Eye, 0);
    let significant_digits = significant_digits(state.results_reads());
    let kind_tag = super::active_kind_tag(state.results_reads());
    let out = view::show(
        ui,
        &state.analysis.eye_diagram_state,
        &source,
        plot_view,
        &mut state.results.session.eye_texture,
        &mut state.results.session.cache,
        significant_digits,
        &kind_tag,
    );
    if out.density_baked {
        frame_work::note(DatasetWalk::EyeRaster);
    }
    if let Some(response) = out.plot {
        super::record_drawn_axes(state.results, ResultViewer::Eye, &response);
        if response.view.any() {
            state
                .results
                .session
                .plot_view_mut(ResultViewer::Eye, 0)
                .apply(&response.view);
        }
    }
}

pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    adopt_settings(state);
    view::right_panel(
        ui,
        &state.analysis.eye_diagram_state,
        &selected_source(state.results_reads()),
        significant_digits(state.results_reads()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::documents::result_document::test_host::ResultsTestHost;
    use rspice_results_ui::eye_diagram::{EyeData, EyeTrace};

    /// Panning and zooming must not rebake. The bake walks every acquisition,
    /// and a drag is exactly when the reader can least afford it.
    #[test]
    fn moving_the_view_does_not_rebake_the_density() {
        let mut state = ResultsTestHost::default();
        let mut data = EyeData::new(1.0e-9, 2);
        for trace in 0..8 {
            data.add_trace(EyeTrace::new(
                vec![0.0, 0.5, 1.0, 1.5],
                vec![0.0, 1.0, 0.0, f64::from(trace) * 0.1],
            ));
        }
        state.results.analysis.eye_diagram_state.load_data(data);

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let frame = |state: &mut ResultsTestHost| {
            let _ = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1_200.0, 800.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default()
                        .show(ctx, |ui| show(ui, &mut state.results_parts()));
                },
            );
        };

        frame(&mut state);
        let baseline = super::super::frame_work::WorkCounts::reset();

        // The reader zooms into the middle of the eye, then pans.
        for window in [(0.4, 1.2, 0.1, 0.8), (0.6, 1.4, 0.2, 0.9)] {
            let view = state
                .results
                .results
                .session
                .plot_view_mut(super::super::ResultViewer::Eye, 0);
            view.x = Some((window.0, window.1));
            view.y = Some((window.2, window.3));
            frame(&mut state);
        }

        assert_eq!(
            baseline
                .since()
                .get(super::super::frame_work::DatasetWalk::EyeRaster),
            0,
            "the eye rebaked its density texture while the reader moved the view"
        );
    }

    /// The mask the reader saves is the mask they open again: the file holds
    /// its vertices, and opening it draws and tests that outline as a custom
    /// mask, in place of the mask drawn since.
    #[test]
    fn a_mask_saved_to_a_file_opens_as_the_same_mask() {
        use crate::io::file_exchange::{ScriptedChoice, script_next_choice};

        let mut state = ResultsTestHost::default();
        let mut data = EyeData::new(1.0e-9, 2);
        for trace in 0..8 {
            data.add_trace(EyeTrace::new(
                vec![0.0, 0.5, 1.0, 1.5],
                vec![0.0, 1.0, 0.0, f64::from(trace) * 0.1],
            ));
        }
        let eye = &mut state.results.analysis.eye_diagram_state;
        eye.load_data(data);
        let mut drawn = eye.settings().clone();
        drawn.mask = EyeMask {
            shape: MaskShape::Diamond,
            width_ui: 0.4,
            height: Some(0.3),
            ..EyeMask::default()
        };
        eye.adopt_settings(&drawn);
        let amplitude = eye.mask_frame().amplitude;
        let outline = drawn.mask.outline(amplitude);

        let folder = std::env::temp_dir().join(format!(
            "rspice-eye-mask-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&folder).expect("a folder for the mask file");
        let path = folder.join("diamond.txt");
        let ctx = egui::Context::default();
        script_next_choice(ScriptedChoice::Chose(path.clone()));
        ask_for_mask_file(&ctx, &mut state.results_parts(), MaskFile::Save);
        take_mask_files(&ctx, &mut state.results_parts());
        let written = std::fs::read_to_string(&path).expect("the mask file is written");
        assert_eq!(
            EyeMask::from_text(&written).expect("a mask file").vertices,
            outline
        );

        // The reader draws another mask and turns it off, then opens the file.
        let mut other = drawn.clone();
        other.mask = EyeMask::default();
        other.show_mask = false;
        state
            .results
            .analysis
            .eye_diagram_state
            .adopt_settings(&other);
        script_next_choice(ScriptedChoice::Chose(path));
        ask_for_mask_file(&ctx, &mut state.results_parts(), MaskFile::Open);
        take_mask_files(&ctx, &mut state.results_parts());
        let opened = state.results.analysis.eye_diagram_state.settings();
        assert_eq!(opened.mask.shape, MaskShape::Custom);
        assert_eq!(opened.mask.outline(amplitude), outline);
        assert!(opened.show_mask, "an opened mask is drawn and tested");
        let _ = std::fs::remove_dir_all(folder);
    }
}
