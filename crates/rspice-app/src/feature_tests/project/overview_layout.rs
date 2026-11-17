//! The Overview's cards as a window of each width lays them out: nothing a
//! card shows is cut, its lists take the room the window has, and its links
//! read in the theme's ink.

use egui::{Color32, Rect, Vec2, vec2};

use super::surface::{populated_project_app, show};
use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_state::state::{AnalysisResult, AnalysisType, Cell, Library, View, ViewType};
use rspice_app_state::state::{SimulationRunLifecycle, SimulationRunProvenance};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::ui::tokens::{Mode, Tokens};

/// The Overview of `app` on a stage `size` big in `mode`, drawn until its
/// layout settles: the context it was drawn on and the last pass.
fn frame(app: &mut RSpiceApp, size: Vec2, mode: Mode) -> (egui::Context, egui::FullOutput) {
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Overview;
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme {
        mode,
        ..rspice_app_workbench::ui::Theme::default()
    }
    .apply(&ctx);
    let input = || egui::RawInput {
        screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
        ..Default::default()
    };
    let mut output = ctx.run_ui(input(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| show(ui, app));
    });
    for _ in 0..2 {
        output = ctx.run_ui(input(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, app));
        });
    }
    (ctx, output)
}

/// Every text run `output` painted: its words, where they were painted and
/// the rect they were clipped to.
fn runs(output: &egui::FullOutput) -> Vec<(String, Rect, Rect)> {
    fn walk(shape: &egui::epaint::Shape, clip: Rect, into: &mut Vec<(String, Rect, Rect)>) {
        match shape {
            egui::epaint::Shape::Text(run) => into.push((
                run.galley.job.text.clone(),
                Rect::from_min_size(run.pos, run.galley.size()),
                clip,
            )),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, clip, into);
                }
            }
            _ => {}
        }
    }
    let mut into = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, clipped.clip_rect, &mut into);
    }
    into
}

/// The Overview's painted runs, as [`frame`] draws it.
fn painted(app: &mut RSpiceApp, size: Vec2, mode: Mode) -> Vec<(String, Rect, Rect)> {
    runs(&frame(app, size, mode).1)
}

/// A project whose bench library holds `count` cells, `cell00` on.
fn project_of(count: usize) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let mut bench = Library::new("bench");
    for index in 0..count {
        let mut cell = Cell::new(format!("cell{index:02}"));
        cell.add_view(View::new("schematic", ViewType::Schematic));
        bench.add_cell(cell);
    }
    app.state.model.library_manager.add_library(bench);
    app
}

/// The run `text` was painted as, which the test says is there whole.
fn whole(painted: &[(String, Rect, Rect)], text: &str) -> (Rect, Rect) {
    painted
        .iter()
        .find(|(run, _, _)| run == text)
        .map(|(_, rect, clip)| (*rect, *clip))
        .unwrap_or_else(|| {
            panic!(
                "`{text}` is not painted whole: {:?}",
                painted.iter().map(|(run, _, _)| run).collect::<Vec<_>>()
            )
        })
}

/// "All cells" and "All runs" are painted in the theme's dim ink, light
/// or dark: not in a colour of their own that one theme's card drowns.
#[test]
fn the_card_links_read_in_the_dim_ink_of_either_theme() {
    for mode in [Mode::Light, Mode::Dark] {
        let mut app = populated_project_app();
        let (ctx, output) = frame(&mut app, vec2(1440.0, 900.0), mode);
        let dim = Tokens::get(&ctx).color.text_dim;
        for link in ["All cells", "All runs"] {
            let ink = ink_of(&output, link).unwrap_or_else(|| panic!("`{link}` is painted"));
            assert_eq!(ink, dim, "`{link}` in {mode:?}");
        }
    }
}

/// The colour the run reading `text` was painted in.
fn ink_of(output: &egui::FullOutput, text: &str) -> Option<Color32> {
    fn walk(shape: &egui::epaint::Shape, text: &str) -> Option<Color32> {
        match shape {
            egui::epaint::Shape::Text(run) if run.galley.job.text == text => {
                Some(run.override_text_color.unwrap_or_else(|| {
                    let own = run.galley.job.sections[0].format.color;
                    if own == Color32::PLACEHOLDER {
                        run.fallback_color
                    } else {
                        own
                    }
                }))
            }
            egui::epaint::Shape::Vec(shapes) => shapes.iter().find_map(|shape| walk(shape, text)),
            _ => None,
        }
    }
    output
        .shapes
        .iter()
        .find_map(|clipped| walk(&clipped.shape, text))
}

/// A window too narrow for two columns stacks the cards, and each list
/// takes the room the window has, not a sliver: a forty-cell project shows
/// its last cell on a tall window.
#[test]
fn a_stacked_overview_gives_its_lists_the_height_of_the_window() {
    let mut app = project_of(40);
    let painted = painted(&mut app, vec2(800.0, 1600.0), Mode::Dark);
    whole(&painted, "cell39");
}

/// Every time and every duration in Activity is painted whole and inside
/// its table, a date a day old and an hour-long run among them.
#[test]
fn the_activity_table_shows_every_time_and_duration_whole() {
    let mut app = populated_project_app();
    let now = rspice_app_state::time_compat::unix_epoch().as_secs_f64();
    let started = now - 3.0 * 86_400.0;
    {
        let run = app.state.model.simulation.start_run();
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.success = true;
        run.timestamp = started;
        run.elapsed_time = 4_000.0;
        run.analyses
            .push(AnalysisResult::new(9, AnalysisType::Transient, "tran"));
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("seal run");
    }
    let stamp = rspice_app_state::time_compat::utc_stamp((started * 1_000.0).round() as u64);
    let painted = painted(&mut app, vec2(1440.0, 900.0), Mode::Dark);
    for text in [stamp.as_str(), "1 h 6 min 40 s", "1 min 36 s", "8.30 s"] {
        let (run, clip) = whole(&painted, text);
        assert!(
            clip.expand(1.0).contains_rect(run),
            "`{text}` runs past its table: {run:?} in {clip:?}"
        );
    }
}

/// The two columns a laptop window gives the Overview hold the Cells table
/// whole: its last head shows inside the card, with no sideways scroll.
#[test]
fn the_cells_table_fits_its_card_without_scrolling_sideways() {
    let mut app = project_of(3);
    // The stage the workspace has in a 1600-point window, its rails open.
    let painted = painted(&mut app, vec2(980.0, 900.0), Mode::Dark);
    let (run, clip) = whole(&painted, "Instances");
    assert!(
        clip.expand(1.0).contains_rect(run),
        "the Instances head runs past the card: {run:?} in {clip:?}"
    );
}
