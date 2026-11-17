//! The Hardcopy Studio drawn in a whole application: its sections, rail and
//! strip at every viewport, and the widget identities they keep.

use crate::feature_tests::hardcopy::hardcopy_parts;
use crate::workbench::app::RSpiceApp;
use egui::Align;
use egui::Context;
use egui::Frame;
use egui::Layout;
use egui::Rect;
use egui::Vec2;
use egui::vec2;
use rspice_app_hardcopy::workbench::app::dialogs::hardcopy::HardcopyRegion;
use rspice_app_hardcopy::workbench::app::dialogs::hardcopy::HardcopySection;
use rspice_app_hardcopy::workbench::app::dialogs::hardcopy::HardcopyWorkflow;
use rspice_app_hardcopy::workbench::app::dialogs::hardcopy::publish;
use rspice_app_hardcopy::workbench::app::dialogs::hardcopy::render::*;
use rspice_app_workbench::ui::tokens;
use rspice_app_workbench::ui::tokens::Tokens;

#[cfg(not(target_arch = "wasm32"))]
fn studio_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let reference = app.state.model.workspace.content.active_view.clone();
    app.state.workbench.documents.activate(
        rspice_app_workbench::workbench::state::workbench_session::WorkspaceDocumentId::CellView(
            reference,
        ),
    );
    publish::open_hardcopy_workflow(&mut hardcopy_parts(&mut app), HardcopyWorkflow::Export);
    app
}

#[cfg(not(target_arch = "wasm32"))]
fn render_studio(ctx: &Context, app: &mut RSpiceApp, viewport: Vec2) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, viewport)),
            ..Default::default()
        },
        |ui| render_hardcopy_dialog(&mut hardcopy_parts(app), ui),
    )
}

/// Fonts build on the first pass, the source resolves on a worker thread,
/// and the exact page raster lands a pass after the plan it belongs to. The
/// measurement is only meaningful once all three have happened and the modal
/// fade has finished, so the probe waits for them using egui's frame clock.
#[cfg(not(target_arch = "wasm32"))]
fn settled_studio(ctx: &Context, app: &mut RSpiceApp, viewport: Vec2) -> egui::FullOutput {
    let _ = render_studio(ctx, app, viewport);
    let fully_visible_at =
        ctx.input(|input| input.time) + f64::from(ctx.global_style().animation_time);
    for _ in 0..400 {
        let _ = render_studio(ctx, app, viewport);
        if app.state.hardcopy.hardcopy.preview.is_some()
            && ctx.input(|input| input.time) >= fully_visible_at
        {
            let _ = render_studio(ctx, app, viewport);
            return render_studio(ctx, app, viewport);
        }
        if app.state.hardcopy.hardcopy.preview.is_none() {
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
    panic!("the hardcopy studio never resolved a source and an exact page preview");
}

/// Every control the dialog laid out this pass, at the rect it was laid out
/// at. A scroll area clips what it paints but still lays its content out at
/// true coordinates, so a control that can only be reached by scrolling
/// reports a rect outside the surface — which is the condition being
/// measured.
#[cfg(not(target_arch = "wasm32"))]
fn dialog_controls(output: &egui::FullOutput) -> Vec<(String, Rect)> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the probe enables AccessKit, so every pass carries a tree")
        .nodes
        .iter()
        // The modal container spans the whole viewport by design: it is the
        // scrim, not the surface.
        .filter(|(_, node)| node.role() != egui::accesskit::Role::Dialog)
        // A scroll bar is the scroll area's own furniture, laid against the
        // surface edge by egui rather than laid out by the studio, and it
        // overhangs that edge by a point. Excluding it does not soften what
        // this probe measures: content a scroll area is hiding still reports
        // its true rect outside the surface, which is the condition being
        // caught. The bar is the symptom's frame, not the symptom.
        .filter(|(_, node)| node.role() != egui::accesskit::Role::ScrollBar)
        .filter_map(|(_, node)| {
            let bounds = node.bounds()?;
            let rect = Rect::from_min_max(
                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
            );
            // A static label carries its text as the node's value; every
            // other control carries it as the node's label.
            let text = node.label().or_else(|| node.value()).unwrap_or_default();
            rect.is_finite().then(|| (text.to_owned(), rect))
        })
        .collect()
}

/// The largest control carrying `label`. A rail entry names itself twice —
/// once as the text inside it and once as the row that selects it — so the
/// row is the taller of the two.
#[cfg(not(target_arch = "wasm32"))]
fn control_rect(controls: &[(String, Rect)], label: &str) -> Option<Rect> {
    controls
        .iter()
        .filter(|(found, _)| found == label)
        .map(|(_, rect)| *rect)
        .reduce(|held, rect| {
            if rect.height() > held.height() {
                rect
            } else {
                held
            }
        })
}

/// Which region the surface is showing and which section it is editing are
/// different questions. Asked by identical chips in one row they read as
/// one list of six peers, and nothing says that choosing the sixth replaces
/// the whole surface.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn the_narrow_strip_asks_one_question_per_control() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = studio_app();
    let viewport = vec2(390.0, 844.0);

    app.state.hardcopy.hardcopy.region = HardcopyRegion::Setup;
    let controls = dialog_controls(&settled_studio(&ctx, &mut app, viewport));
    assert!(control_rect(&controls, "01  Source and scope").is_some());
    assert!(control_rect(&controls, "Setup").is_some());
    assert!(control_rect(&controls, "Preview").is_some());
    assert!(
        !controls
            .iter()
            .any(|(label, _)| label.starts_with("0") && label.contains("Preview")),
        "the preview must not be offered as a sixth section chip"
    );

    // With the page on screen there is no section to choose, so the chips
    // that would choose one are not there to be read as inert.
    app.state.hardcopy.hardcopy.region = HardcopyRegion::Preview;
    let controls = dialog_controls(&settled_studio(&ctx, &mut app, viewport));
    assert!(control_rect(&controls, "01  Source and scope").is_none());
    assert!(control_rect(&controls, "Setup").is_some());
    assert!(control_rect(&controls, "Preview").is_some());
}

/// The studio ships to tablet and phone, so every row and chip it navigates
/// by is a touch target where the shell is in touch mode.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn navigation_targets_meet_the_touch_minimum() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    rspice_app_workbench::ui::Theme::default()
        .apply_responsive_metrics_with_target(&ctx, Some(44.0));
    let mut app = studio_app();

    let rail = dialog_controls(&settled_studio(&ctx, &mut app, vec2(1_440.0, 900.0)));
    for section in HardcopySection::ALL {
        if let Some(rect) = control_rect(&rail, section.label()) {
            assert!(
                rect.height() >= tokens::TOUCH_TARGET,
                "rail row {:?} is {:.0} pt tall",
                section.label(),
                rect.height()
            );
        }
    }

    let strip = dialog_controls(&settled_studio(&ctx, &mut app, vec2(390.0, 844.0)));
    for label in ["01  Source and scope", "Setup", "Preview"] {
        let rect = control_rect(&strip, label).unwrap_or_else(|| panic!("{label} is drawn"));
        assert!(
            rect.height() >= tokens::TOUCH_TARGET,
            "strip control {label:?} is {:.0} pt tall",
            rect.height()
        );
    }
}

/// Every error-filled triangle the pass painted. The rail's problem marker
/// is a path rather than a rectangle, so `painted_rects` cannot see it, and
/// its colour is the only thing that distinguishes it from the section
/// icons drawn beside it.
#[cfg(not(target_arch = "wasm32"))]
fn marker_rects(output: &egui::FullOutput, err: egui::Color32) -> Vec<Rect> {
    fn walk(shape: &egui::Shape, err: egui::Color32, into: &mut Vec<Rect>) {
        match shape {
            egui::Shape::Path(path) if path.fill == err => {
                into.push(path.visual_bounding_rect());
            }
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, err, into);
                }
            }
            _ => {}
        }
    }
    let mut rects = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, err, &mut rects);
    }
    rects
}

/// Every rectangle egui outlined because the widget occupying it answered
/// to a different id than it did on the previous pass. egui reports these
/// as a log warning and a red outline; the outline is the part a test can
/// read, and it carries the rectangle that names the offender.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
fn id_change_marks(output: &egui::FullOutput) -> Vec<Rect> {
    output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Rect(shape)
                if clipped.clip_rect == Rect::EVERYTHING
                    && shape.stroke.color == egui::Color32::RED
                    && shape.stroke.width == 2.0
                    && shape.stroke_kind == egui::StrokeKind::Outside =>
            {
                Some(shape.rect)
            }
            _ => None,
        })
        .collect()
}

/// A widget's id is its continuity: focus, drag state, tooltips and the
/// scroll offsets of everything under it are keyed by it. An id derived
/// from how many siblings happened to be laid out first changes whenever
/// the surface does — a section appearing, a refusal opening the banner —
/// and every one of those is an ordinary move in this dialog.
#[test]
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
fn no_widget_changes_its_id_as_the_studio_changes() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = studio_app();
    let viewport = vec2(1_440.0, 900.0);

    let mut marks = Vec::new();
    let mut record = |output: &egui::FullOutput| {
        marks.extend(id_change_marks(output).into_iter().map(|rect| {
            format!(
                "[{:.0},{:.0}]-[{:.0},{:.0}]",
                rect.left(),
                rect.top(),
                rect.right(),
                rect.bottom()
            )
        }));
    };
    for _ in 0..200 {
        record(&render_studio(&ctx, &mut app, viewport));
        if app.state.hardcopy.hardcopy.preview.is_some() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    // The sheet section only appears once the source has resolved, and that
    // arrival is the move that shifted the rail's ids. Without it the rest
    // of this probe would pass by never having been tested.
    assert!(
        app.state.hardcopy.hardcopy.preview.is_some(),
        "the studio never resolved a source and an exact page preview"
    );
    record(&render_studio(&ctx, &mut app, viewport));
    for section in HardcopySection::ALL {
        app.state.hardcopy.hardcopy.section = section;
        record(&render_studio(&ctx, &mut app, viewport));
        record(&render_studio(&ctx, &mut app, viewport));
    }
    // A refusal opens the banner strip between the body and the command
    // row, and clearing it closes it again. Both are ordinary, and a
    // publication that fails late raises one over a plan that stayed valid.
    for refusal in [true, false] {
        app.state.hardcopy.hardcopy.error =
            refusal.then(|| "the publication was refused".to_owned());
        record(&render_studio(&ctx, &mut app, viewport));
        record(&render_studio(&ctx, &mut app, viewport));
    }
    app.state.hardcopy.hardcopy.margin_left = "100".to_owned();
    app.state.hardcopy.hardcopy.refresh_preview();
    record(&render_studio(&ctx, &mut app, viewport));
    record(&render_studio(&ctx, &mut app, viewport));
    app.state.hardcopy.hardcopy.margin_left = "0.5".to_owned();
    app.state.hardcopy.hardcopy.refresh_preview();
    record(&render_studio(&ctx, &mut app, viewport));
    record(&render_studio(&ctx, &mut app, viewport));

    // Below the rail's width the sections become a strip and the regions a
    // switch, so the same moves rearrange a different composition.
    let narrow = vec2(390.0, 844.0);
    record(&render_studio(&ctx, &mut app, narrow));
    record(&render_studio(&ctx, &mut app, narrow));
    for region in [HardcopyRegion::Preview, HardcopyRegion::Setup] {
        app.state.hardcopy.hardcopy.region = region;
        record(&render_studio(&ctx, &mut app, narrow));
        record(&render_studio(&ctx, &mut app, narrow));
    }

    assert!(
        marks.is_empty(),
        "widgets changed id between passes at {marks:?}"
    );
}

/// An invalid field is invisible from every other section, and a disabled
/// primary with a dialog-level banner says only that something is wrong.
/// The rail has to say where: marked by name in the accessibility tree,
/// marked on the surface by a glyph inside the entry it belongs to, and
/// neither at the cost of the row's height.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn a_blocked_section_is_marked_on_the_rail_from_every_other_section() {
    for ready_preview in [false, true] {
        let ctx = Context::default();
        ctx.enable_accesskit();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        let err = Tokens::get(&ctx).color.err;
        let mut app = studio_app();
        let viewport = vec2(1_440.0, 900.0);

        if ready_preview {
            // A resolved preview can arrive before a newly opened dialog finishes
            // fading in. Warm the real source/preview pipeline in another context
            // so this case does not depend on the machine's worker scheduling.
            let warm_context = Context::default();
            rspice_app_workbench::ui::Theme::default().apply(&warm_context);
            let _ = settled_studio(&warm_context, &mut app, viewport);
        }

        let settled = settled_studio(&ctx, &mut app, viewport);
        let clean = dialog_controls(&settled);
        let clean_row = control_rect(&clean, HardcopySection::Page.label())
            .expect("the rail names every section it offers");
        assert!(
            marker_rects(&settled, err)
                .iter()
                .all(|marker| !clean_row.contains_rect(*marker)),
            "a valid configuration marks nothing"
        );

        // A margin wider than the page, entered while standing in Output.
        app.state.hardcopy.hardcopy.section = HardcopySection::Output;
        app.state.hardcopy.hardcopy.margin_left = "100".to_owned();
        app.state.hardcopy.hardcopy.refresh_preview();
        assert_eq!(
            app.state.hardcopy.hardcopy.error_section,
            Some(HardcopySection::Page)
        );
        // A blocked draft compiles no plan, so the settling probe would wait
        // for a preview that can never arrive. Two passes are what the tree
        // needs: one to lay the rail out and one to report it.
        let _ = render_studio(&ctx, &mut app, viewport);
        let blocked = render_studio(&ctx, &mut app, viewport);
        let controls = dialog_controls(&blocked);

        let marked = control_rect(&controls, "Page and pagination · blocked")
            .expect("the rail entry names the section that holds the refused field");
        // The entry sets its title as text inside itself as well. What has to
        // carry the mark is the row a keyboard or screen reader lands on, which
        // is the one that contains the other.
        let title = control_rect(&controls, HardcopySection::Page.label())
            .expect("the entry still draws its title");
        assert!(
            marked.contains_rect(title) && marked.height() > title.height(),
            "the mark is on a label inside the entry rather than on the entry"
        );
        for section in [
            HardcopySection::Source,
            HardcopySection::Output,
            HardcopySection::Identity,
        ] {
            assert!(
                control_rect(&controls, &format!("{} · blocked", section.label())).is_none(),
                "{section:?} holds nothing invalid and must not be marked"
            );
        }
        assert!(
            (marked.height() - clean_row.height()).abs() < 0.5,
            "the marker grew the rail row from {:.0} pt to {:.0} pt",
            clean_row.height(),
            marked.height()
        );
        let markers: Vec<Rect> = marker_rects(&blocked, err)
            .into_iter()
            .filter(|marker| marked.contains_rect(*marker))
            .collect();
        assert_eq!(
            markers.len(),
            1,
            "the marked entry carries exactly one painted mark"
        );
        assert!(
            marked.right() - markers[0].right() < PROBLEM_MARKER_TRACK,
            "the mark sits at the entry's trailing edge"
        );

        // The strip the primary's refusal is explained in names the same
        // section, so the banner and the rail cannot send the operator to
        // different places.
        assert!(
            controls
                .iter()
                .any(|(label, _)| label == "Blocked in Page and pagination"),
            "the transaction strip names the section holding the primary"
        );
    }
}

/// Below the rail's width the chips are the rail, and a mark that only
/// existed in the wide layout would be missing on exactly the surfaces
/// where a section is hardest to find.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn the_narrow_strip_marks_the_blocked_section_too() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = studio_app();
    let viewport = vec2(390.0, 844.0);
    let _ = settled_studio(&ctx, &mut app, viewport);

    app.state.hardcopy.hardcopy.region = HardcopyRegion::Setup;
    app.state.hardcopy.hardcopy.section = HardcopySection::Identity;
    app.state.hardcopy.hardcopy.margin_left = "100".to_owned();
    app.state.hardcopy.hardcopy.refresh_preview();
    let _ = render_studio(&ctx, &mut app, viewport);
    let controls = dialog_controls(&render_studio(&ctx, &mut app, viewport));

    assert!(control_rect(&controls, "02  Page and pagination · blocked").is_some());
    assert!(control_rect(&controls, "01  Source and scope").is_some());
}

/// The editor column at `measure`, laid out to its own natural height:
/// what the section takes, and how far the widest thing in it reaches.
#[cfg(not(target_arch = "wasm32"))]
fn editor_extent(app: &mut RSpiceApp, section: HardcopySection, measure: f32) -> (f32, f32) {
    app.state.hardcopy.hardcopy.section = section;
    let draft = &mut app.state.hardcopy.hardcopy;
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut natural = 0.0_f32;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                vec2(measure + 40.0, 4_000.0),
            )),
            ..Default::default()
        },
        |ui| {
            egui::CentralPanel::default()
                .frame(Frame::NONE)
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                    let column = ui.allocate_ui_with_layout(
                        vec2(measure, 0.0),
                        Layout::top_down(Align::Min),
                        |ui| {
                            ui.set_width(measure);
                            editor_column(ui, draft, None);
                        },
                    );
                    natural = column.response.rect.height();
                });
        },
    );
    let reach = dialog_controls(&output)
        .iter()
        .filter(|(_, rect)| rect.width() < measure + 1.0)
        .fold(0.0_f32, |acc, (_, rect)| acc.max(rect.right()));
    (natural, reach)
}

/// The division is a claim about the forms, not about the surface: at the
/// measure the studio picks, every section reaches the column's far edge.
/// The fixed fraction this replaced left Identity — a column of switches
/// with nothing two-up in it — standing in a column a third wider than
/// anything it contained, which is the emptiness the measure exists to
/// remove.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn every_section_reaches_the_edge_of_the_measure_it_is_given() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = studio_app();
    // Sections draw what the resolved source gives them, so the measurement
    // is only of the real form once the source has landed.
    let _ = settled_studio(&ctx, &mut app, vec2(1_440.0, 900.0));
    for section in HardcopySection::ALL {
        let (_, reach) = editor_extent(&mut app, section, EDITOR_STACKED_MEASURE);
        assert!(
            reach >= EDITOR_STACKED_MEASURE - 24.0,
            "{section:?} reaches only {reach:.0} pt of its {EDITOR_STACKED_MEASURE:.0} pt \
                 column"
        );
    }
}
