//! The Hardcopy Studio's content options drawn in a whole application: the
//! raster resolution a page publishes is the one its field shows.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::hardcopy::render::content_options::*;
use crate::workbench::app::dialogs::hardcopy::render::output_summary;
use crate::workbench::app::dialogs::hardcopy::{
    DEFAULT_RASTER_DPI, HardcopyRegion, HardcopySection, HardcopyWorkflow, PaperDraft,
    hardcopy_parts, publish, render_hardcopy_dialog,
};
use egui::accesskit::Role;
use egui::{Context, Rect, Vec2, vec2};
use rspice_hardcopy::render::max_raster_dpi;
use rspice_hardcopy_contract::{OutputFormat, StandardPaper};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// The save picker the export reaches once the plan is sealed. It refuses a
/// destination so nothing is written, and counts the calls so a test can
/// tell "the export ran and carried this plan" from "the export never got
/// past the plan".
struct RefusedDestination(Arc<AtomicUsize>);

impl crate::workbench::workflows::export_workflow::ExportWorkflowIo for RefusedDestination {
    fn show_save_dialog(
        &self,
        _config: crate::workbench::workflows::export_workflow::SaveDialogConfig<'_>,
    ) -> Result<Option<PathBuf>, String> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(None)
    }

    fn write_text_file(&self, _path: &Path, _contents: &str) -> Result<(), String> {
        Ok(())
    }

    fn write_waveform_csv(
        &self,
        _dataset: &crate::io::WaveformDataset,
        _path: &Path,
    ) -> Result<(), String> {
        Ok(())
    }
}

struct Studio {
    ctx: Context,
    app: RSpiceApp,
    exports: Arc<AtomicUsize>,
    viewport: Vec2,
}

impl Studio {
    /// The Export workflow open on the active cell view, settled onto a
    /// 150 dpi raster target: a resolution the operator can be seen to
    /// change, an order of magnitude clear of this page's ceiling.
    fn raster(viewport: Vec2) -> Self {
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let exports = Arc::new(AtomicUsize::new(0));
        let mut app = RSpiceApp::test_instance();
        app.export_workflow_io = Box::new(RefusedDestination(exports.clone()));
        let reference = app.state.model.workspace.content.active_view.clone();
        app.state.workbench.documents.activate(
            crate::workbench::state::WorkspaceDocumentId::CellView(reference),
        );
        publish::open_hardcopy_workflow(&mut hardcopy_parts(&mut app), HardcopyWorkflow::Export);
        let mut studio = Self {
            ctx,
            app,
            exports,
            viewport,
        };
        studio.settle();
        studio.app.state.hardcopy.hardcopy.section = HardcopySection::Output;
        studio.app.state.hardcopy.hardcopy.region = HardcopyRegion::Setup;
        studio.app.state.hardcopy.hardcopy.format = OutputFormat::Png { dpi: 150 };
        studio.app.state.hardcopy.hardcopy.raster_dpi_draft = "150".to_owned();
        studio.app.state.hardcopy.hardcopy.embed_fonts = false;
        studio.app.state.hardcopy.hardcopy.searchable_text = false;
        studio.app.state.hardcopy.hardcopy.refresh_preview();
        studio.settle();
        studio
    }

    /// The desktop composition: rail, editor and preview all on screen.
    fn wide() -> Self {
        Self::raster(vec2(1_440.0, 900.0))
    }

    /// A phone-width surface, which is below `STUDIO_SPLIT_WIDTH` and so
    /// shows the setup or the page but never both.
    fn narrow() -> Self {
        Self::raster(vec2(390.0, 844.0))
    }

    fn pass(&mut self, events: Vec<egui::Event>) -> egui::FullOutput {
        let app = &mut self.app;
        let viewport = self.viewport;
        self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, viewport)),
                events,
                ..Default::default()
            },
            |ui| render_hardcopy_dialog(&mut hardcopy_parts(app), ui),
        )
    }

    /// The source resolves on a worker and the exact page raster lands a
    /// pass after the plan it belongs to, so a measurement is only
    /// meaningful once both have happened.
    ///
    /// The budget is wall-clock, not a count of passes. A count is not a
    /// deadline: a loaded box starves the worker far harder than it slows
    /// this thread, so a fixed number of passes buys less and less real
    /// time exactly when the worker needs more of it. That is how this
    /// test came to fail on a machine running three builds and pass in
    /// 0.37 s on its own.
    fn settle(&mut self) -> egui::FullOutput {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
        loop {
            let _ = self.pass(Vec::new());
            if self.app.state.hardcopy.hardcopy.preview.is_some() {
                let _ = self.pass(Vec::new());
                return self.pass(Vec::new());
            }
            assert!(
                std::time::Instant::now() < deadline,
                "the hardcopy studio never resolved a source and an exact page preview"
            );
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }

    fn control(&mut self, role: Role, label: &str) -> Rect {
        let output = self.pass(Vec::new());
        controls(&output)
            .into_iter()
            .find(|(found, name, _)| *found == role && name == label)
            .unwrap_or_else(|| panic!("the studio drew no {role:?} labelled {label:?}"))
            .2
    }

    fn field(&mut self) -> Rect {
        let output = self.pass(Vec::new());
        controls(&output)
            .into_iter()
            .find(|(role, _, _)| *role == Role::TextInput)
            .expect("the output section draws the resolution field")
            .2
    }

    /// Put the caret in the resolution field and replace what it holds.
    fn type_resolution(&mut self, text: &str) {
        let field = self.field();
        let caret = egui::pos2(field.right() - 4.0, field.center().y);
        let _ = self.pass(press(caret));
        let _ = self.pass(release(caret));
        let _ = self.pass(vec![
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            },
            egui::Event::Text(text.to_owned()),
        ]);
        assert_eq!(
            self.app.state.hardcopy.hardcopy.raster_dpi_draft, text,
            "the field did not take the typed resolution"
        );
    }

    /// The resolution of the plan the export sealed. `current_plan` hands
    /// the publication the dialog's own preview plan, so this is the plan
    /// that ran — and the export is only credited if the workflow reached
    /// its destination picker rather than refusing before it.
    fn published_resolution(&self) -> Option<u16> {
        assert_eq!(
            self.exports.load(Ordering::SeqCst),
            1,
            "the export never reached its destination: {:?}",
            self.app.state.hardcopy.hardcopy.error
        );
        self.app
            .state
            .hardcopy
            .hardcopy
            .preview_plan
            .as_ref()
            .and_then(|plan| plan.setup().render().format().raster_dpi())
    }
}

fn controls(output: &egui::FullOutput) -> Vec<(Role, String, Rect)> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the probe enables AccessKit, so every pass carries a tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| {
            let bounds = node.bounds()?;
            let rect = Rect::from_min_max(
                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
            );
            rect.is_finite().then(|| {
                (
                    node.role(),
                    node.label()
                        .or_else(|| node.value())
                        .unwrap_or_default()
                        .to_owned(),
                    rect,
                )
            })
        })
        .collect()
}

fn press(pos: egui::Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(pos),
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
    ]
}

fn release(pos: egui::Pos2) -> Vec<egui::Event> {
    vec![egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::NONE,
    }]
}

fn click(pos: egui::Pos2) -> Vec<egui::Event> {
    let mut events = press(pos);
    events.extend(release(pos));
    events
}

/// A click delivered inside one pass — a press and a release between two
/// repaints, which is what an ordinary click on a settled surface is. The
/// dialog draws its body before its command row, so the field has to notice
/// it is no longer being edited within the same pass that publishes.
#[test]
fn a_resolution_typed_and_published_in_one_gesture_publishes_what_it_shows() {
    let mut studio = Studio::wide();
    studio.type_resolution("300");
    let button = studio.control(Role::Button, "Export hardcopy");
    let _ = studio.pass(click(button.center()));

    assert_eq!(studio.published_resolution(), Some(300));
    assert_eq!(studio.app.state.hardcopy.hardcopy.raster_dpi_draft, "300");
}

/// The same gesture spread over two passes, which is what a click looks
/// like when the surface repaints between the press and the release.
#[test]
fn a_resolution_published_by_a_click_spanning_two_passes_publishes_what_it_shows() {
    let mut studio = Studio::wide();
    studio.type_resolution("300");
    let button = studio.control(Role::Button, "Export hardcopy");
    let _ = studio.pass(press(button.center()));
    let _ = studio.pass(release(button.center()));

    assert_eq!(studio.published_resolution(), Some(300));
    assert_eq!(studio.app.state.hardcopy.hardcopy.raster_dpi_draft, "300");
}

/// The rail replaces the section in the pass it is clicked, so the field is
/// never drawn again to see that its editing session ended. Publishing from
/// the section the operator landed on used to seal the previous resolution
/// while the field still held the typed one.
#[test]
fn a_resolution_left_behind_by_the_rail_is_still_the_one_published() {
    let mut studio = Studio::wide();
    studio.type_resolution("300");
    let rail = studio.control(Role::Button, "Identity");
    let _ = studio.pass(click(rail.center()));
    assert_eq!(
        studio.app.state.hardcopy.hardcopy.section,
        HardcopySection::Identity,
        "the rail did not leave the output section"
    );

    let button = studio.control(Role::Button, "Export hardcopy");
    let _ = studio.pass(click(button.center()));

    assert_eq!(studio.published_resolution(), Some(300));
    assert_eq!(studio.app.state.hardcopy.hardcopy.raster_dpi_draft, "300");
}

/// Below `STUDIO_SPLIT_WIDTH` the surface shows the setup or the page and
/// never both, so choosing the page replaces the whole editor — the section
/// panels included — in the pass the switch is clicked. Publishing from
/// there used to seal the previous resolution while the field held the
/// typed one, and no section panel was left running to notice.
#[test]
fn a_resolution_left_behind_by_the_region_switch_is_still_the_one_published() {
    let mut studio = Studio::narrow();
    studio.type_resolution("300");
    let preview = studio.control(Role::Button, "Preview");
    let _ = studio.pass(click(preview.center()));
    assert_eq!(
        studio.app.state.hardcopy.hardcopy.region,
        HardcopyRegion::Preview,
        "the switch did not leave the setup region"
    );

    let button = studio.control(Role::Button, "Export hardcopy");
    let _ = studio.pass(click(button.center()));

    assert_eq!(studio.published_resolution(), Some(300));
    assert_eq!(studio.app.state.hardcopy.hardcopy.raster_dpi_draft, "300");
}

/// Settling belongs on the path every pass takes, not on the surfaces a
/// composition may choose not to draw. Whatever the rail, the tab strip and
/// the region switch select — in either composition — a pass has to leave
/// the draft holding the resolution the field is showing.
#[test]
fn no_section_or_region_can_strand_a_typed_resolution() {
    // `Studio` owns a complete `AppState`; keep only one fixture on the
    // Windows test-thread stack at a time. Building both values in an
    // array can overflow the default stack as production state grows.
    for narrow in [false, true] {
        let mut studio = if narrow {
            Studio::narrow()
        } else {
            Studio::wide()
        };
        let composition = studio.viewport;
        for section in HardcopySection::ALL {
            for region in [HardcopyRegion::Setup, HardcopyRegion::Preview] {
                studio.app.state.hardcopy.hardcopy.format = OutputFormat::Png { dpi: 150 };
                studio.app.state.hardcopy.hardcopy.raster_dpi_draft = "300".to_owned();
                studio.app.state.hardcopy.hardcopy.section = section;
                studio.app.state.hardcopy.hardcopy.region = region;
                let _ = studio.pass(Vec::new());
                assert_eq!(
                    studio.app.state.hardcopy.hardcopy.format.raster_dpi(),
                    Some(300),
                    "{composition:?} {section:?} {region:?} left a typed resolution uncommitted"
                );
            }
        }
    }
}

/// The largest page a standard media offers, which is what pulls the
/// ceiling well under both the contract's range and the default a raster
/// target would otherwise commit.
fn ceiling_bound_media(studio: &mut Studio) -> u16 {
    studio.app.state.hardcopy.hardcopy.paper = PaperDraft::Standard(StandardPaper::A0);
    studio.app.state.hardcopy.hardcopy.refresh_preview();
    let _ = studio.pass(Vec::new());
    let plan = studio
        .app
        .state
        .hardcopy
        .hardcopy
        .preview_plan
        .clone()
        .expect("A0 is a legal page");
    let ceiling = max_raster_dpi(&plan, OutputFormat::Png { dpi: 300 })
        .expect("an A0 page rasters at some resolution");
    assert!(
        ceiling < DEFAULT_RASTER_DPI,
        "this page rasters to {ceiling} dpi, which is not under the default a target commits"
    );
    ceiling
}

/// A raster target is selected here and refused nowhere else until the
/// render round trip, so the entries the list offers carry the resolution
/// they would commit — not the default, which on a browser's raster budget
/// is above the ceiling for an ordinary Letter page. The two formats charge
/// different bytes per pixel, so each entry is bounded by its own ceiling
/// rather than by one number for both.
#[test]
fn the_target_list_offers_no_resolution_the_page_it_is_on_refuses() {
    let mut studio = Studio::wide();
    let ceiling = ceiling_bound_media(&mut studio);
    let draft = &studio.app.state.hardcopy.hardcopy;

    let png = bounded_raster(
        draft,
        OutputFormat::Png {
            dpi: DEFAULT_RASTER_DPI,
        },
    );
    let tiff = bounded_raster(
        draft,
        OutputFormat::Tiff {
            dpi: DEFAULT_RASTER_DPI,
        },
    );
    assert_eq!(png, OutputFormat::Png { dpi: ceiling });
    assert!(
        tiff.raster_dpi().is_some_and(|dpi| dpi < ceiling),
        "TIFF holds a third more bytes per pixel and cannot share PNG's ceiling: {tiff:?}"
    );
    // A resolution already under the ceiling is offered untouched, and a
    // vector target has no resolution to bound.
    assert_eq!(
        bounded_raster(draft, OutputFormat::Png { dpi: 72 }),
        OutputFormat::Png { dpi: 72 }
    );
    assert_eq!(
        bounded_raster(draft, OutputFormat::SvgVector),
        OutputFormat::SvgVector
    );
}

/// The ceiling belongs to the page, so every control that changes the page
/// moves it — under a resolution that was legal when it was committed. The
/// draft has to be brought back inside it before anything is published from
/// it, and what is published has to be what the surface was reporting.
#[test]
fn a_page_that_lowers_the_ceiling_lowers_the_resolution_committed_with_it() {
    let mut studio = Studio::wide();
    studio.app.state.hardcopy.hardcopy.format = OutputFormat::Png {
        dpi: DEFAULT_RASTER_DPI,
    };
    studio.app.state.hardcopy.hardcopy.raster_dpi_draft = DEFAULT_RASTER_DPI.to_string();
    studio.app.state.hardcopy.hardcopy.refresh_preview();
    let _ = studio.pass(Vec::new());
    assert_eq!(
        studio.app.state.hardcopy.hardcopy.format,
        OutputFormat::Png {
            dpi: DEFAULT_RASTER_DPI
        },
        "the default resolution is legal on the page the studio opened on"
    );

    let ceiling = ceiling_bound_media(&mut studio);
    assert_eq!(
        studio.app.state.hardcopy.hardcopy.format,
        OutputFormat::Png { dpi: ceiling },
        "the wider media left a resolution behind that this host cannot raster"
    );
    assert_eq!(
        output_summary(studio.app.state.hardcopy.hardcopy.format),
        format!("PNG · {ceiling} dpi")
    );

    let button = studio.control(Role::Button, "Export hardcopy");
    let _ = studio.pass(click(button.center()));
    assert_eq!(studio.published_resolution(), Some(ceiling));
}

/// An entry that names no resolution cannot be published, so an unfocused
/// field states the one in force rather than a number that will not be used.
#[test]
fn a_field_nobody_is_editing_states_the_resolution_in_force() {
    let mut studio = Studio::wide();
    studio.app.state.hardcopy.hardcopy.raster_dpi_draft = "3oo".to_owned();
    studio.app.state.hardcopy.hardcopy.section = HardcopySection::Page;
    let _ = studio.pass(Vec::new());

    assert_eq!(studio.app.state.hardcopy.hardcopy.raster_dpi_draft, "150");
    assert_eq!(
        studio.app.state.hardcopy.hardcopy.format,
        OutputFormat::Png { dpi: 150 }
    );
}

/// A resolution being typed is not a resolution yet: `60` is a legal prefix
/// of `600`, so nothing commits while the caret is still in the field.
#[test]
fn a_resolution_still_being_typed_commits_nothing() {
    let mut studio = Studio::wide();
    studio.type_resolution("1200");
    assert_eq!(
        studio.app.state.hardcopy.hardcopy.format,
        OutputFormat::Png { dpi: 150 },
        "the field committed while it was still being typed into"
    );
    let _ = studio.pass(Vec::new());
    assert_eq!(
        studio.app.state.hardcopy.hardcopy.format,
        OutputFormat::Png { dpi: 150 },
        "the field committed while it still held the caret"
    );
}
