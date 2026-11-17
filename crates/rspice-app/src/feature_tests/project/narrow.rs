//! The Project workspace on a tablet and a phone: whole application frames
//! under a coarse pointer, at the widths of a portrait tablet and a phone.
//! Every tab and chip is a touch target, the page in front is in view, and
//! the narrow title names the project.

#![cfg(not(target_arch = "wasm32"))]

use egui::accesskit::Role;
use egui::{Context, Pos2, Rect, Vec2, vec2};

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::ui::tokens::TOUCH_TARGET;
use rspice_app_workbench::workbench::chrome::title_bar::title_context_text;

const TABLET: Vec2 = vec2(820.0, 1180.0);
const PHONE: Vec2 = vec2(390.0, 844.0);

/// The application on `page` of the Project workspace under a finger.
pub(super) fn touch_app(page: ProjectPage) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = page;
    app.state.workbench.coarse_pointer = true;
    app
}

/// Whole application frames on a stage `size` big, a tenth of a second
/// apart, until the layout and any scroll it asked for have settled; the
/// last one.
pub(super) fn settled_frame(ctx: &Context, app: &mut RSpiceApp, size: Vec2) -> egui::FullOutput {
    let mut step = 0_u32;
    let mut frame = || {
        step += 1;
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
                time: Some(f64::from(step) * 0.1),
                ..Default::default()
            },
            |ui| {
                let ctx = ui.ctx().clone();
                app.prepare_frame(&ctx);
                app.render_frame_chrome(ui);
                app.render_frame_dialogs(&ctx);
            },
        )
    };
    for _ in 0..12 {
        frame();
    }
    frame()
}

pub(super) fn touch_context() -> Context {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx
}

/// The page tabs `output` announced: each one's name and bounds.
fn page_tabs(output: &egui::FullOutput) -> Vec<(String, Rect)> {
    announced(output, Role::Tab)
        .into_iter()
        .filter(|(label, _)| ProjectPage::ALL.iter().any(|page| page.label() == label))
        .collect()
}

/// Every announced node `output` published with `role`: its name and
/// bounds.
pub(super) fn announced(output: &egui::FullOutput, role: Role) -> Vec<(String, Rect)> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .filter(|(_, node)| node.role() == role)
        .filter_map(|(_, node)| {
            let bounds = node.bounds()?;
            Some((
                node.label().unwrap_or_default().to_owned(),
                Rect::from_min_max(
                    Pos2::new(bounds.x0 as f32, bounds.y0 as f32),
                    Pos2::new(bounds.x1 as f32, bounds.y1 as f32),
                ),
            ))
        })
        .collect()
}

/// The page tabs on a tablet are as tall as a finger needs, whichever page
/// is in front.
#[test]
fn every_page_tab_is_a_touch_target_on_a_tablet() {
    for page in ProjectPage::ALL {
        let mut app = touch_app(page);
        let output = settled_frame(&touch_context(), &mut app, TABLET);
        let tabs = page_tabs(&output);
        assert_eq!(tabs.len(), ProjectPage::ALL.len(), "{page:?}: {tabs:?}");
        for (label, rect) in tabs {
            assert!(
                rect.height() >= TOUCH_TARGET - 0.5,
                "{page:?}: the {label} tab is {} tall",
                rect.height()
            );
        }
    }
}

/// On a phone the strip of page tabs is wider than the screen: whichever
/// page is in front, its tab is brought into view.
#[test]
fn the_tab_of_the_page_in_front_is_in_view_on_a_phone() {
    for page in ProjectPage::ALL {
        let mut app = touch_app(page);
        let output = settled_frame(&touch_context(), &mut app, PHONE);
        let (_, rect) = page_tabs(&output)
            .into_iter()
            .find(|(label, _)| label == page.label())
            .unwrap_or_else(|| panic!("the {} tab is announced", page.label()));
        assert!(
            rect.left() >= -0.5 && rect.right() <= PHONE.x + 0.5,
            "the {} tab is out of view: {rect:?}",
            page.label()
        );
    }
}

/// The chips over a page on a phone are each on the screen whole and each
/// as tall as a finger needs: they take another row rather than running off
/// its edge.
#[test]
fn the_chips_over_a_page_are_whole_touch_targets_on_a_phone() {
    for page in [ProjectPage::Library, ProjectPage::Dependencies] {
        let mut app = touch_app(page);
        let output = settled_frame(&touch_context(), &mut app, PHONE);
        let chips = announced(&output, Role::Button)
            .into_iter()
            .filter(|(label, _)| {
                label.contains("Configuration used for runs")
                    || label.contains("Latest run")
                    || label.contains("checks")
            })
            .collect::<Vec<_>>();
        assert!(chips.len() >= 2, "{page:?}: {chips:?}");
        for (label, rect) in chips {
            assert!(
                rect.left() >= -0.5 && rect.right() <= PHONE.x + 0.5,
                "{page:?}: `{label}` runs off the screen: {rect:?}"
            );
            assert!(
                rect.height() >= TOUCH_TARGET - 0.5,
                "{page:?}: `{label}` is {} tall",
                rect.height()
            );
        }
    }
}

/// The narrow layout's title names the project: the tabs under it name the
/// page.
#[test]
fn the_narrow_title_names_the_project_on_every_page() {
    for page in ProjectPage::ALL {
        let app = touch_app(page);
        assert_eq!(
            title_context_text(&app, true),
            app.state.model.workspace.content.project.display_name(),
            "{page:?}"
        );
    }
}
