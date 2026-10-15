//! Review renders of the Preferences dialog, one PNG per category.
//!
//! The PNGs go to `RSPICE_RASTER_DIR` (default: the system temp directory).
//! Read them for layout, not for wording; the rasterizer's own header says
//! why. Run the renders with `RSPICE_RASTER_PASSES=30` so the modal's
//! fade-in has finished by the pass that is rasterized.

use egui::vec2;

use crate::workbench::RSpiceApp;

use super::preference_pages;
use super::preferences_shell::{self, PreferenceCategory};

const DIALOG_WIDTH: f32 = 1_060.0;
const DIALOG_HEIGHT: f32 = 760.0;

#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_every_preferences_category_for_review() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    // Dark is the default theme, and a dialog reviewed only there can be
    // unreadable in light without anyone seeing it: the two palettes are not
    // inversions of each other. The densest page is rendered in both.
    let mut themes = vec![(crate::ui::Theme::default(), "")];
    themes.push((
        crate::ui::Theme {
            mode: crate::ui::Mode::Light,
            ..crate::ui::Theme::default()
        },
        "-light",
    ));

    for category in PreferenceCategory::ALL {
        for (theme, suffix) in &themes {
            if !suffix.is_empty() && category != PreferenceCategory::Schematic {
                continue;
            }
            let mut app = RSpiceApp::test_instance();
            let mut active = category;
            let canvas = crate::ui::raster::render_themed(
                *theme,
                vec2(DIALOG_WIDTH, DIALOG_HEIGHT),
                |ui, background| {
                    let state = &mut app.state;
                    let mut actions = super::PreferencePageActions::default();
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE.fill(background))
                        .show(ui, |ui| {
                            let _ = preferences_shell::show(
                                ui.ctx(),
                                &mut active,
                                true,
                                |ui, category| {
                                    preference_pages::render(ui, category, state, &mut actions);
                                },
                            );
                        });
                },
            );
            let path = directory.join(format!("preferences-{}{suffix}.png", category.stable_id()));
            std::fs::write(&path, canvas.png(DIALOG_HEIGHT as usize)).expect("write png");
            writeln!(report, "wrote {}", path.display()).ok();
        }
    }
}

/// Where the dialog actually put its edges, as numbers.
///
/// A render shows the design only if someone measures it: "nothing clipped,
/// nothing overlapping" passes a page whose title, labels and buttons each
/// start on a different vertical line. This prints the left edge of every
/// label, every control and both footer buttons, so the one column they are
/// all supposed to share can be read off rather than judged by eye.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "prints a geometry report for a human to read; run with --ignored"]
fn report_preferences_geometry() {
    use std::io::Write as _;

    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    for category in PreferenceCategory::ALL {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        let mut active = category;
        let mut update = None;
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        vec2(DIALOG_WIDTH, DIALOG_HEIGHT),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    let state = &mut app.state;
                    let mut actions = super::PreferencePageActions::default();
                    let _ = preferences_shell::show(ctx, &mut active, true, |ui, category| {
                        preference_pages::render(ui, category, state, &mut actions);
                    });
                },
            );
            update = output.platform_output.accesskit_update;
        }
        let nodes = update.expect("AccessKit tree update").nodes;
        writeln!(report, "\n=== {} ===", category.label()).ok();
        let dialog = nodes
            .iter()
            .find(|(_, node)| node.role() == egui::accesskit::Role::Dialog)
            .and_then(|(_, node)| node.bounds())
            .expect("the dialog publishes its bounds");
        writeln!(
            report,
            "dialog  x {:.0}..{:.0}  y {:.0}..{:.0}",
            dialog.x0, dialog.x1, dialog.y0, dialog.y1
        )
        .ok();
        let mut rows: Vec<(f64, f64, f64, String, String)> = nodes
            .iter()
            .filter_map(|(_, node)| {
                let bounds = node.bounds()?;
                let label = node.label()?;
                let kind = match node.role() {
                    egui::accesskit::Role::Label => "label",
                    egui::accesskit::Role::Heading => "heading",
                    egui::accesskit::Role::ComboBox => "select",
                    egui::accesskit::Role::CheckBox => "switch",
                    egui::accesskit::Role::Slider => "slider",
                    egui::accesskit::Role::Button => "button",
                    egui::accesskit::Role::SpinButton => "number",
                    _ => return None,
                };
                Some((
                    bounds.y0,
                    bounds.y1,
                    bounds.x0,
                    kind.to_owned(),
                    label.to_string(),
                ))
            })
            .collect();
        rows.sort_by(|left, right| left.partial_cmp(right).expect("finite bounds"));
        for (top, bottom, left, kind, label) in rows {
            let inset = left - dialog.x0;
            let middle = f64::midpoint(top, bottom);
            writeln!(
                report,
                "  y {top:7.1}..{bottom:7.1} mid {middle:7.1}  x {left:7.1} (+{inset:6.1})  \
                 {kind:<8} {label}"
            )
            .ok();
        }
    }
}
