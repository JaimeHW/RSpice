//! What adopting and saving do to a placed source, and what the dialog that
//! offers them states.
//!
//! The claims that matter are the ones a reader cannot check by eye: a
//! cross-family adoption really is a re-type that keeps both terminals on
//! their nets, one undo puts the old type *and* the old card back, a
//! cross-kind adoption is refused rather than converted, and the list a reader
//! walks never arrives on a row the primary would refuse.

use super::*;
use crate::state::{ComponentType, Point};
#[cfg(not(target_arch = "wasm32"))]
use crate::ui::tokens::Mode;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;

/// Draw the dialog as the frame's step does, over the whole state.
fn render_stimulus_link_dialog(ctx: &egui::Context, state: &mut AppState) {
    super::render_stimulus_link_dialog(
        &mut state.design.stimulus_link,
        &mut state.design.selected_stimulus_definition,
        &mut state.model,
        &mut state.workbench,
        ctx,
    );
}

pub(crate) fn definition(
    name: &str,
    kind: ComponentType,
    value: &str,
    params: &str,
) -> StimulusDefinition {
    let mut definition =
        StimulusDefinition::new(name, kind, crate::state::stimulus_library::now_unix_ms)
            .expect("definition");
    definition.value = value.to_owned();
    definition.params = params.to_owned();
    definition
}

/// One sheet with a wired `PULSE` source and a library holding a `SIN`
/// definition of the same quantity and a current definition of the other.
pub(crate) fn fixture() -> (RSpiceApp, u64) {
    let mut app = RSpiceApp::test_instance();
    let id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::VoltageSourcePulse, Point::new(40, 40));
    for held in [
        definition(
            "sensor_drive",
            ComponentType::VoltageSourceSin,
            "0",
            "va=3m freq=1k",
        ),
        definition("bias_rail", ComponentType::VoltageSource, "1.8", ""),
        definition("leak", ComponentType::CurrentSource, "1u", ""),
    ] {
        app.state
            .model
            .workspace
            .content
            .stimulus_library
            .insert(held)
            .expect("insert");
    }
    (app, id)
}

/// The fixture the list is judged on: five definitions covering all three
/// fits, so every group is non-empty and every group is ordered.
pub(crate) fn wide_fixture() -> (RSpiceApp, u64) {
    let (mut app, id) = fixture();
    for held in [
        definition(
            "clock",
            ComponentType::VoltageSourcePulse,
            "0",
            "v2=1.8 pw=1u per=2u",
        ),
        definition(
            "probe_tone",
            ComponentType::CurrentSourceSin,
            "0",
            "ia=1u freq=10k",
        ),
    ] {
        app.state
            .model
            .workspace
            .content
            .stimulus_library
            .insert(held)
            .expect("insert");
    }
    (app, id)
}

pub(crate) fn instance_of(app: &RSpiceApp, id: u64) -> Component {
    app.state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .cloned()
        .expect("the fixture instance is on the sheet")
}

/// Both modes state their whole subject, in both themes, at the two desktop
/// viewports the shell gates — and nothing they paint is cut off.
///
/// A height assertion proves nothing here: the rasterizer clips at the
/// viewport, so a surface that overflowed would report exactly the viewport's
/// height. What a reader loses when a dialog does not fit is a run of text, so
/// the runs are what this checks — that the ones that carry the transaction are
/// there, and that every run painted lies inside the region it was clipped to.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn both_modes_fit_and_state_their_subject_at_every_gated_viewport() {
    for screen in [egui::vec2(1024.0, 640.0), egui::vec2(1440.0, 900.0)] {
        for mode in [Mode::Dark, Mode::Light] {
            let unpicked = painted(Scene::Adopt(None), screen, mode);
            for expected in [
                "Adopt a definition onto V1",
                "Same family \u{00b7} PULSE \u{00b7} copies parameters",
                "Other families \u{00b7} re-places V1",
                "Current sources \u{00b7} cannot drive V1",
                "sensor_drive",
                "leak",
                "Pick a definition to see the card V1 would hold",
                "Adopt",
                "Cancel",
            ] {
                assert!(
                    unpicked.text.contains(expected),
                    "{screen:?} {mode:?}: adopt lost {expected:?}: {}",
                    unpicked.text
                );
            }
            unpicked.assert_nothing_runs_past_its_pane(screen, mode);

            let picked = painted(Scene::Adopt(Some("sensor_drive")), screen, mode);
            for expected in [
                "After adoption \u{00b7} sensor_drive r1 \u{00b7} SIN",
                "SIN(",
                "Re-place and adopt",
                "Terminals",
                "Provenance",
            ] {
                assert!(
                    picked.text.contains(expected),
                    "{screen:?} {mode:?}: adopt lost {expected:?}: {}",
                    picked.text
                );
            }
            picked.assert_nothing_runs_past_its_pane(screen, mode);

            let save = painted(Scene::Save(None), screen, mode);
            for expected in [
                "Save V1 as a definition",
                "Name",
                "Purpose",
                "Published card",
                // An accepted name says nothing: the validation line renders
                // only the refusal that stops the save.
                "Stored in",
                "Save definition",
            ] {
                assert!(
                    save.text.contains(expected),
                    "{screen:?} {mode:?}: save lost {expected:?}: {}",
                    save.text
                );
            }
            save.assert_nothing_runs_past_its_pane(screen, mode);
        }
    }
}

/// The two waveforms are one instrument: the same plot width, the same left
/// and right edges and the same height, so a time on one row is directly above
/// the same time on the other.
#[test]
fn the_two_waveform_rows_are_one_instrument() {
    let frame = egui::Rect::from_min_size(
        egui::pos2(120.0, 40.0),
        egui::vec2(
            400.0,
            crate::properties::source_preview::instrument_height(2, 64.0) - 16.0,
        ),
    );
    let rows = crate::properties::source_preview::strip_rows(frame, 64.0, 2);
    assert_eq!(rows.len(), 2);
    assert!(
        (rows[0].plot.left() - rows[1].plot.left()).abs() <= 0.5
            && (rows[0].plot.right() - rows[1].plot.right()).abs() <= 0.5,
        "the plots do not share their x range: {:?} {:?}",
        rows[0].plot,
        rows[1].plot
    );
    assert!(
        (rows[0].plot.height() - rows[1].plot.height()).abs() <= 0.5,
        "the plots are different heights: {} {}",
        rows[0].plot.height(),
        rows[1].plot.height()
    );
    for row in &rows {
        let gap = row.plot.top() - row.caption.bottom();
        assert!(
            (0.0..=4.0).contains(&gap),
            "a caption sits {gap} points above the plot it names"
        );
        assert_eq!(
            row.caption.left(),
            row.plot.left(),
            "both captions start at the plot's own left edge"
        );
        for other in &rows {
            // Touching edges are not an overlap: a caption's floor is above
            // the plot's ceiling, which is the whole point of giving it a row.
            assert!(
                row.caption.bottom() <= other.plot.top()
                    || row.caption.top() >= other.plot.bottom(),
                "a caption is painted over a plot: {:?} over {:?}",
                row.caption,
                other.plot
            );
        }
    }
    let air = rows[1].plot.top() - rows[0].plot.bottom();
    assert!(
        air >= 30.0,
        "only {air} points separate the two plots: the divider, the caption and both traces \
         are stacked with no air between them"
    );
}

/// One left edge serves the whole preview pane: the caption, the widest value
/// label and the card block all start on it, so there is no dead band between
/// the pane's inset and the numbers.
///
/// The footer's consequence note starts at the dialog's own left inset too,
/// directly after its mark, rather than floating in the middle of the footer.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_preview_pane_and_the_footer_note_share_one_left_edge() {
    let painted = painted(
        Scene::Adopt(Some("sensor_drive")),
        egui::vec2(1024.0, 640.0),
        Mode::Dark,
    );
    let facts = painted
        .run("Component")
        .expect("the pane's first fact label");
    let tick = painted
        .runs
        .iter()
        .filter(|(text, rect, _)| {
            rect.left() > facts.left() - 4.0 && (text.ends_with(" V") || text.ends_with(" mV"))
        })
        .map(|(_, rect, _)| rect.left())
        .fold(f32::INFINITY, f32::min);
    assert!(
        tick.is_finite() && (tick - facts.left()).abs() <= 2.0,
        "the widest value label starts at {tick} and the facts list at {}: there is a dead \
         band between the pane's inset and its numbers",
        facts.left()
    );
    // The fixture's pulse repeats five hundred times over the project's
    // transient, so the row is a measured band and its caption says so.
    let caption = painted
        .run("Now \u{00b7} PULSE \u{00b7} 500 cycles \u{00b7} envelope")
        .expect("the first row's caption");
    assert!(
        caption.left() > tick + 10.0,
        "the first row's caption is inside the frame, right of the shared value gutter"
    );

    // Every run that lies in the value gutter is a tick label. No two of them
    // crowd each other, which is the same claim as the two plots having air
    // between them, read off what was actually painted.
    let mut ticks: Vec<egui::Rect> = painted
        .runs
        .iter()
        .filter(|(_, rect, _)| rect.left() >= facts.left() - 1.0 && rect.right() <= caption.left())
        .map(|(_, rect, _)| *rect)
        .collect();
    ticks.sort_by(|left, right| left.top().total_cmp(&right.top()));
    assert!(
        ticks.len() >= 5,
        "the gutter labels were not found: {ticks:?}"
    );
    for pair in ticks.windows(2) {
        let gap = pair[1].top() - pair[0].bottom();
        assert!(
            gap >= 8.0,
            "two value labels are {gap} points apart: {:?} and {:?}",
            pair[0],
            pair[1]
        );
    }

    let note = painted
        .run("Re-places V1 as a SIN source. One undo restores the PULSE instance.")
        .expect("the consequence note");
    assert!(
        note.left() - painted.surface.left() <= 60.0,
        "the footer note starts {} points into a {} point dialog, which is centred rather \
         than left-aligned behind its mark",
        note.left() - painted.surface.left(),
        painted.surface.width()
    );
}

/// No axis label, readout or fact this dialog publishes is in exponent
/// notation. `3.00e-3` on a surface whose whole job is to say what a source
/// does was the defect that closed this class.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn nothing_the_dialog_paints_is_exponent_notation() {
    for scene in [
        Scene::Adopt(None),
        Scene::Adopt(Some("sensor_drive")),
        Scene::Adopt(Some("bias_rail")),
        Scene::Save(None),
    ] {
        let painted = painted(scene, egui::vec2(1024.0, 640.0), Mode::Dark);
        for run in painted.text.lines() {
            assert!(
                !is_exponent_notation(run),
                "{scene:?} painted {run:?} in exponent notation"
            );
        }
        assert!(
            painted.text.contains("mV") || painted.text.contains(" V"),
            "{scene:?} states no engineering unit at all: {}",
            painted.text
        );
    }
}

/// Whether a painted run holds a number in exponent notation. Spelled out
/// rather than matched on `e-`, because `re-places` is a word this dialog says.
#[cfg(not(target_arch = "wasm32"))]
fn is_exponent_notation(run: &str) -> bool {
    let bytes: Vec<char> = run.chars().collect();
    bytes.windows(3).any(|window| {
        window[0].is_ascii_digit()
            && (window[1] == 'e' || window[1] == 'E')
            && (window[2].is_ascii_digit() || window[2] == '-' || window[2] == '+')
    })
}

/// Which dialog, over which fixture, with what picked.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy)]
enum Scene {
    /// Adopt over the five-definition library, with an optional pick.
    Adopt(Option<&'static str>),
    /// Save, optionally with a name the library already holds.
    Save(Option<&'static str>),
}

#[cfg(not(target_arch = "wasm32"))]
impl Scene {
    /// One app with this scene's dialog already open on it.
    fn open(self) -> RSpiceApp {
        let (mut app, id) = wide_fixture();
        let mode = match self {
            Self::Save(_) => StimulusLinkMode::Extract,
            _ => StimulusLinkMode::Adopt,
        };
        open_stimulus_link(
            &mut app.state.design.stimulus_link,
            &app.state.model,
            id,
            mode,
        )
        .expect("opens");
        match self {
            Self::Adopt(pick) => app.state.design.stimulus_link.pick = pick.map(str::to_owned),
            Self::Save(Some(name)) => {
                app.state.design.stimulus_link.name = name.to_owned();
            }
            _ => {}
        }
        app
    }
}

/// Everything one scene paints, and where.
#[cfg(not(target_arch = "wasm32"))]
struct Painted {
    text: String,
    /// Every painted run with the rectangle it occupies and the rectangle it
    /// was clipped to.
    runs: Vec<(String, egui::Rect, egui::Rect)>,
    /// The widest filled rectangle narrower than the viewport, which is the
    /// dialog's own surface. Read back rather than recomputed, so this stays a
    /// test of what was painted and not a second copy of the kit's geometry.
    surface: egui::Rect,
}

#[cfg(not(target_arch = "wasm32"))]
impl Painted {
    /// Where one exact run was painted.
    fn run(&self, text: &str) -> Option<egui::Rect> {
        self.runs
            .iter()
            .find(|(painted, _, _)| painted == text)
            .map(|(_, rect, _)| *rect)
    }

    /// Nothing this surface painted lands outside the dialog, and nothing the
    /// list painted runs out of the pane it was clipped to. Both are facts a
    /// reader silently does not get.
    ///
    /// Only the horizontal extent is checked against the clip: a long library
    /// is meant to scroll, so a row below the fold is a row the reader can
    /// reach, while a name past the pane's right edge is one they cannot.
    fn assert_nothing_runs_past_its_pane(&self, screen: egui::Vec2, theme_mode: Mode) {
        for (text, rect, clip) in &self.runs {
            let past_surface = (self.surface.left() - rect.left())
                .max(rect.right() - self.surface.right())
                .max(self.surface.top() - rect.top())
                .max(rect.bottom() - self.surface.bottom());
            assert!(
                past_surface <= 1.0,
                "{screen:?} {theme_mode:?}: {text:?} lands {past_surface} points outside the \
                 dialog (run {rect:?}, surface {:?})",
                self.surface
            );
            let past_clip = (clip.left() - rect.left()).max(rect.right() - clip.right());
            assert!(
                past_clip <= 1.0,
                "{screen:?} {theme_mode:?}: {text:?} runs {past_clip} points past its pane \
                 (run {rect:?}, pane {clip:?})"
            );
        }
    }
}

/// A theme in one mode, everything else as the product ships it.
#[cfg(not(target_arch = "wasm32"))]
fn themed(mode: Mode) -> crate::ui::Theme {
    crate::ui::Theme {
        mode,
        ..crate::ui::Theme::default()
    }
}

/// Lay one scene out and read back every run it painted.
#[cfg(not(target_arch = "wasm32"))]
fn painted(scene: Scene, screen: egui::Vec2, mode: Mode) -> Painted {
    fn walk(shape: &egui::epaint::Shape, clip: egui::Rect, into: &mut Painted, screen: egui::Vec2) {
        match shape {
            egui::epaint::Shape::Text(painted) => {
                let text = painted.galley.job.text.clone();
                into.text.push_str(&text);
                into.text.push('\n');
                into.runs.push((
                    text,
                    egui::Rect::from_min_size(painted.pos, painted.galley.size()),
                    clip,
                ));
            }
            egui::epaint::Shape::Rect(painted) => {
                let rect = painted.rect;
                if rect.width() < screen.x - 1.0 && rect.width() > into.surface.width() {
                    into.surface = rect;
                }
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, clip, into, screen);
                }
            }
            _ => {}
        }
    }

    let mut app = scene.open();
    let ctx = egui::Context::default();
    themed(mode).apply(&ctx);
    let input = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, screen)),
        ..egui::RawInput::default()
    };
    // Two passes: the first builds the font set and measures the surface, the
    // second lays out against both, which is when the runs are worth reading.
    let _ = ctx.run_ui(input(), |ctx| {
        render_stimulus_link_dialog(ctx, &mut app.state);
    });
    let output = ctx.run_ui(input(), |ctx| {
        render_stimulus_link_dialog(ctx, &mut app.state);
    });
    let mut painted = Painted {
        text: String::new(),
        runs: Vec::new(),
        surface: egui::Rect::NOTHING,
    };
    for clipped in &output.shapes {
        walk(&clipped.shape, clipped.clip_rect, &mut painted, screen);
    }
    assert!(
        painted.surface.width() > 400.0,
        "the dialog surface was not painted: {:?}",
        painted.surface
    );
    painted
}
