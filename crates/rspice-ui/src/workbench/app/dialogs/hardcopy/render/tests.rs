//! The hardcopy studio decides once, then renders what it decided.
//!
//! Two rules carry most of these: a summary reports a decision only where
//! there is one to make, and the preview worker is bounded — two rotating
//! texture slots, no launch over an active worker, and a cancel that
//! completes before the dialog can reopen. The fit cases hold the studio to
//! every viewport without scrolling, which is the constraint the section
//! layout exists to satisfy.

use super::*;

/// A source with no governed drawing sheet has no sheet furniture to
/// configure, so the rail must not offer the section — and a selection
/// left pointing at it must land somewhere real rather than on a blank
/// editor.
#[test]
fn a_source_without_an_authored_sheet_offers_no_sheet_section() {
    let draft = HardcopyDialogState::default();
    let sections = available_sections(&draft);

    assert!(!sections.contains(&HardcopySection::DrawingSheet));
    assert_eq!(sections.first(), Some(&HardcopySection::Source));
    assert!(sections.contains(&HardcopySection::Output));
}

/// The rail summarises the sections nobody is looking at, so a summary that
/// names a decision the document does not pose is worse than none.
#[test]
fn the_sheet_summary_only_reports_a_decision_there_is_one_to_make() {
    let mut draft = HardcopyDialogState::default();
    draft.outside_sheet_content = OutsideSheetContentPolicy::Ask;
    // No resolved document: nothing crosses a sheet that does not exist.
    assert_eq!(
        section_detail(&draft, HardcopySection::DrawingSheet),
        "authored · undecided"
    );
    draft.schematic_extent = SchematicHardcopyExtent::CompleteSchematicContent;
    assert_eq!(
        section_detail(&draft, HardcopySection::DrawingSheet),
        "complete content"
    );
}

/// A raster target is meaningless without its resolution, and the rail is
/// where it is read while another section is open.
#[test]
fn the_output_summary_carries_the_resolution_of_a_raster_target() {
    assert_eq!(
        output_summary(OutputFormat::Png { dpi: 300 }),
        "PNG · 300 dpi"
    );
    assert_eq!(
        output_summary(OutputFormat::Tiff { dpi: 1_200 }),
        "TIFF · 1200 dpi"
    );
    assert_eq!(output_summary(OutputFormat::PdfA), "PDF/A · vector");
}

/// The field must refuse what the publisher would refuse, and for the
/// same reason. A ceiling of `None` means the plan has not compiled yet,
/// so only the contract's own range is known.
#[test]
fn a_resolution_outside_the_offered_range_is_refused() {
    assert_eq!(parse_raster_dpi("300", Some(600)), Ok(300));
    assert_eq!(parse_raster_dpi("  600 ", Some(600)), Ok(600));
    assert!(parse_raster_dpi("601", Some(600)).is_err());
    assert!(parse_raster_dpi("71", Some(600)).is_err());
    assert!(parse_raster_dpi("", Some(600)).is_err());
    assert!(parse_raster_dpi("600dpi", Some(600)).is_err());
    assert!(parse_raster_dpi("-300", Some(600)).is_err());
    // Without a compiled plan the contract is the only authority.
    assert_eq!(parse_raster_dpi("9600", None), Ok(MAX_RASTER_DPI));
    assert!(parse_raster_dpi("9601", None).is_err());
}

#[test]
fn preview_texture_cache_is_bounded_to_two_rotating_slots() {
    assert_eq!(preview_texture_slot_id(0), preview_texture_slot_id(0));
    assert_eq!(preview_texture_slot_id(1), preview_texture_slot_id(8));
    assert_ne!(preview_texture_slot_id(0), preview_texture_slot_id(1));
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn rapid_preview_invalidation_never_launches_over_an_active_worker() {
    assert_eq!(
        preview_schedule_decision(None, 10),
        PreviewScheduleDecision::Launch
    );
    assert_eq!(
        preview_schedule_decision(Some(10), 10),
        PreviewScheduleDecision::Wait
    );
    for generation in 11..1_000 {
        assert_eq!(
            preview_schedule_decision(Some(10), generation),
            PreviewScheduleDecision::CancelAndWait
        );
    }
    assert_eq!(
        preview_schedule_decision(None, 999),
        PreviewScheduleDecision::Launch
    );
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn closing_preview_cancels_and_clears_worker_before_reopen() {
    let (_sender, receiver) = std::sync::mpsc::sync_channel(1);
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    PREVIEW_WORKER.with(|runtime| {
        runtime.borrow_mut().active = Some(ActivePreviewWorker {
            generation: 41,
            receiver,
            cancelled: cancelled.clone(),
        });
    });

    cancel_preview_worker();

    assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
    PREVIEW_WORKER.with(|runtime| {
        assert!(runtime.borrow().active.is_none());
    });
    assert_eq!(
        preview_schedule_decision(None, 42),
        PreviewScheduleDecision::Launch
    );
}

/// Viewports the studio has to fit: the desktop sizes the workbench ships
/// against, the widths where its three columns stop fitting side by side,
/// and the edge-to-edge tablet and phone surfaces.
#[cfg(not(target_arch = "wasm32"))]
const FIT_VIEWPORTS: [(f32, f32); 9] = [
    (1_920.0, 1_200.0),
    (1_440.0, 900.0),
    (1_366.0, 768.0),
    (1_280.0, 800.0),
    (1_024.0, 768.0),
    (900.0, 700.0),
    (820.0, 1_180.0),
    (768.0, 1_024.0),
    (390.0, 844.0),
];

/// The last two controls in the preview column — the toolbar's Fit page
/// button and the final row of the estimate strip. They are named because
/// they are what a surface that does not fit loses first, and because
/// bounds alone cannot tell a control that fits from one that was never
/// drawn.
#[cfg(not(target_arch = "wasm32"))]
const PREVIEW_TAIL_CONTROLS: [&str; 2] = ["Fit page", "COLOR"];

/// Where `DialogSize::SchematicPageSetup` puts the surface: 1280 × 760 pt
/// inside a 12 pt viewport gutter, edge to edge at or below the 820 pt
/// manager breakpoint. Anything the studio lays out beyond this rectangle
/// is off the dialog.
#[cfg(not(target_arch = "wasm32"))]
fn studio_surface(viewport: Vec2) -> Rect {
    let screen = Rect::from_min_size(egui::Pos2::ZERO, viewport);
    if viewport.x <= 820.0 {
        return screen;
    }
    Rect::from_center_size(
        screen.center(),
        vec2(
            1_280.0_f32.min(viewport.x - 12.0),
            760.0_f32.min(viewport.y - 12.0),
        ),
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn studio_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let reference = app.state.workspace.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(crate::workbench::state::WorkspaceDocumentId::CellView(
            reference,
        ));
    publish::open_hardcopy_workflow(&mut app, HardcopyWorkflow::Export);
    app
}

#[cfg(not(target_arch = "wasm32"))]
fn render_studio(ctx: &Context, app: &mut RSpiceApp, viewport: Vec2) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, viewport)),
            ..Default::default()
        },
        |ui| app.render_hardcopy_dialog(ui),
    )
}

/// Fonts build on the first pass, the source resolves on a worker thread,
/// and the exact page raster lands a pass after the plan it belongs to. The
/// measurement is only meaningful once all three have happened, so the
/// probe waits for them.
#[cfg(not(target_arch = "wasm32"))]
fn settled_studio(ctx: &Context, app: &mut RSpiceApp, viewport: Vec2) -> egui::FullOutput {
    for _ in 0..400 {
        let _ = render_studio(ctx, app, viewport);
        if app.state.dialogs.hardcopy.preview.is_some() {
            let _ = render_studio(ctx, app, viewport);
            return render_studio(ctx, app, viewport);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
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

/// How far outside the surface a control reaches, in points.
#[cfg(not(target_arch = "wasm32"))]
fn overflow_beyond(surface: Rect, control: Rect) -> f32 {
    (control.bottom() - surface.bottom())
        .max(surface.top() - control.top())
        .max(control.right() - surface.right())
        .max(surface.left() - control.left())
}

/// The studio's whole claim is that it fits by construction. Rendering it
/// and looking at the picture is how a size gets missed, so the claim is
/// measured: across every viewport the workbench supports and every section
/// the rail offers, no control may land outside the dialog surface. A
/// control outside it is one a scroll area is hiding, which is the failure
/// this layout exists to remove.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn the_studio_fits_every_viewport_and_section_without_scrolling() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = studio_app();
    // The workflow resolves its source on a worker thread and rewrites the
    // draft when it lands, so the matrix starts from a settled one.
    let _ = settled_studio(&ctx, &mut app, vec2(FIT_VIEWPORTS[0].0, FIT_VIEWPORTS[0].1));
    let mut failures = Vec::new();
    for (width, height) in FIT_VIEWPORTS {
        let viewport = vec2(width, height);
        let surface = studio_surface(viewport);
        // Below the split width the studio shows one region at a time, so
        // each one is measured holding the whole track.
        let regions: &[bool] = if surface.width() < STUDIO_SPLIT_WIDTH {
            &[false, true]
        } else {
            &[true]
        };
        for section in HardcopySection::ALL {
            for previewing in regions.iter().copied() {
                app.state.dialogs.hardcopy.section = section;
                app.state.dialogs.hardcopy.region = if previewing {
                    HardcopyRegion::Preview
                } else {
                    HardcopyRegion::Setup
                };
                let output = settled_studio(&ctx, &mut app, viewport);
                let case = format!(
                    "{width}x{height} {section:?}{}",
                    if previewing && regions.len() > 1 {
                        " previewing"
                    } else {
                        ""
                    }
                );
                let controls = dialog_controls(&output);
                assert!(
                    !controls.is_empty(),
                    "{case}: the dialog drew no controls at all"
                );
                // The header sits on the surface's top edge and the command
                // row on its bottom one, so both edges must be reached.
                // Without this the probe would be measuring against a
                // rectangle larger than the dialog, and would pass on
                // anything.
                let reach = controls
                    .iter()
                    .fold((f32::MAX, f32::MIN), |acc, (_, rect)| {
                        (acc.0.min(rect.top()), acc.1.max(rect.bottom()))
                    });
                assert!(
                    reach.0 - surface.top() <= 16.0 && surface.bottom() - reach.1 <= 16.0,
                    "{case}: the surface this probe measures against is not the one the \
                         dialog drew — its controls span {:.0}..{:.0} inside {:.0}..{:.0}",
                    reach.0,
                    reach.1,
                    surface.top(),
                    surface.bottom()
                );
                if previewing {
                    for missing in PREVIEW_TAIL_CONTROLS
                        .into_iter()
                        .filter(|wanted| !controls.iter().any(|(label, _)| label == wanted))
                    {
                        failures.push(format!("{case}: {missing:?} was never drawn"));
                    }
                }
                if let Some((label, overflow)) = controls
                    .iter()
                    .map(|(label, rect)| (label, overflow_beyond(surface, *rect)))
                    .max_by(|left, right| left.1.total_cmp(&right.1))
                    .filter(|(_, overflow)| *overflow > 0.5)
                {
                    failures.push(format!(
                        "{case}: {label:?} is {overflow:.0} pt outside the {:.0}x{:.0} pt \
                             surface",
                        surface.width(),
                        surface.height()
                    ));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "the hardcopy studio does not fit:\n{}",
        failures.join("\n")
    );
}

/// Every filled rectangle the dialog painted this pass. Geometry the
/// AccessKit tree cannot see: a panel's fill is not a control, and the
/// defect it carries — a fill that stops short of the track it was given —
/// is invisible to a probe that only reads controls.
#[cfg(not(target_arch = "wasm32"))]
fn painted_rects(output: &egui::FullOutput) -> Vec<(egui::Color32, Rect)> {
    fn walk(shape: &egui::Shape, into: &mut Vec<(egui::Color32, Rect)>) {
        match shape {
            egui::Shape::Rect(rect) => into.push((rect.fill, rect.rect)),
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, into);
                }
            }
            _ => {}
        }
    }
    let mut rects = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut rects);
    }
    rects
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

/// The strip is a band across the surface, so its fill is the width of the
/// surface's content box — the resolved rect measures the outer edge, and the
/// surface's own border is not track for the band to cover. Sized to its chips
/// instead, it stopped after the last one and the rest of the band read as a
/// second, lighter panel butted against it.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn the_tab_strip_fills_the_track_it_was_given() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let panel = Tokens::get(&ctx).color.bg_panel;
    let mut app = studio_app();
    let viewport = vec2(900.0, 700.0);
    let surface = studio_surface(viewport);
    let output = settled_studio(&ctx, &mut app, viewport);
    let chip = control_rect(&dialog_controls(&output), "01  Source and scope")
        .expect("the strip carries a chip for every section");
    let strip = painted_rects(&output)
        .into_iter()
        .filter(|(fill, rect)| *fill == panel && rect.contains_rect(chip))
        .fold(None::<Rect>, |narrowest, (_, rect)| {
            Some(narrowest.map_or(rect, |held| {
                if rect.height() < held.height() {
                    rect
                } else {
                    held
                }
            }))
        })
        .expect("the strip paints a panel behind its chips");
    let track = surface.shrink(1.0).width();
    assert!(
        strip.width() >= track - 1.0,
        "the strip's fill is {:.0} pt wide inside a {:.0} pt track",
        strip.width(),
        track
    );
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
    crate::ui::Theme::default().apply(&ctx);
    let mut app = studio_app();
    let viewport = vec2(390.0, 844.0);

    app.state.dialogs.hardcopy.region = HardcopyRegion::Setup;
    let controls = dialog_controls(&settled_studio(&ctx, &mut app, viewport));
    assert!(control_rect(&controls, "01  Source and scope").is_some());
    assert!(control_rect(&controls, "Setup").is_some());
    assert!(control_rect(&controls, "Exact preview").is_some());
    assert!(
        !controls
            .iter()
            .any(|(label, _)| label.starts_with("0") && label.contains("Exact preview")),
        "the preview must not be offered as a sixth section chip"
    );

    // With the page on screen there is no section to choose, so the chips
    // that would choose one are not there to be read as inert.
    app.state.dialogs.hardcopy.region = HardcopyRegion::Preview;
    let controls = dialog_controls(&settled_studio(&ctx, &mut app, viewport));
    assert!(control_rect(&controls, "01  Source and scope").is_none());
    assert!(control_rect(&controls, "Setup").is_some());
    assert!(control_rect(&controls, "Exact preview").is_some());
}

/// The studio ships to tablet and phone, so every row and chip it navigates
/// by is a touch target where the shell is in touch mode.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn navigation_targets_meet_the_touch_minimum() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    crate::ui::Theme::default().apply_responsive_metrics_with_target(&ctx, Some(44.0));
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
    for label in ["01  Source and scope", "Setup", "Exact preview"] {
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
    crate::ui::Theme::default().apply(&ctx);
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
        if app.state.dialogs.hardcopy.preview.is_some() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    // The sheet section only appears once the source has resolved, and that
    // arrival is the move that shifted the rail's ids. Without it the rest
    // of this probe would pass by never having been tested.
    assert!(
        app.state.dialogs.hardcopy.preview.is_some(),
        "the studio never resolved a source and an exact page preview"
    );
    record(&render_studio(&ctx, &mut app, viewport));
    for section in HardcopySection::ALL {
        app.state.dialogs.hardcopy.section = section;
        record(&render_studio(&ctx, &mut app, viewport));
        record(&render_studio(&ctx, &mut app, viewport));
    }
    // A refusal opens the banner strip between the body and the command
    // row, and clearing it closes it again. Both are ordinary, and a
    // publication that fails late raises one over a plan that stayed valid.
    for refusal in [true, false] {
        app.state.dialogs.hardcopy.error =
            refusal.then(|| "the publication was refused".to_owned());
        record(&render_studio(&ctx, &mut app, viewport));
        record(&render_studio(&ctx, &mut app, viewport));
    }
    app.state.dialogs.hardcopy.margin_left = "100".to_owned();
    app.state.dialogs.hardcopy.refresh_preview();
    record(&render_studio(&ctx, &mut app, viewport));
    record(&render_studio(&ctx, &mut app, viewport));
    app.state.dialogs.hardcopy.margin_left = "0.5".to_owned();
    app.state.dialogs.hardcopy.refresh_preview();
    record(&render_studio(&ctx, &mut app, viewport));
    record(&render_studio(&ctx, &mut app, viewport));

    // Below the rail's width the sections become a strip and the regions a
    // switch, so the same moves rearrange a different composition.
    let narrow = vec2(390.0, 844.0);
    record(&render_studio(&ctx, &mut app, narrow));
    record(&render_studio(&ctx, &mut app, narrow));
    for region in [HardcopyRegion::Preview, HardcopyRegion::Setup] {
        app.state.dialogs.hardcopy.region = region;
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
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let err = Tokens::get(&ctx).color.err;
    let mut app = studio_app();
    let viewport = vec2(1_440.0, 900.0);

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
    app.state.dialogs.hardcopy.section = HardcopySection::Output;
    app.state.dialogs.hardcopy.margin_left = "100".to_owned();
    app.state.dialogs.hardcopy.refresh_preview();
    assert_eq!(
        app.state.dialogs.hardcopy.error_section,
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

/// Below the rail's width the chips are the rail, and a mark that only
/// existed in the wide layout would be missing on exactly the surfaces
/// where a section is hardest to find.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn the_narrow_strip_marks_the_blocked_section_too() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = studio_app();
    let viewport = vec2(390.0, 844.0);
    let _ = settled_studio(&ctx, &mut app, viewport);

    app.state.dialogs.hardcopy.region = HardcopyRegion::Setup;
    app.state.dialogs.hardcopy.section = HardcopySection::Identity;
    app.state.dialogs.hardcopy.margin_left = "100".to_owned();
    app.state.dialogs.hardcopy.refresh_preview();
    let _ = render_studio(&ctx, &mut app, viewport);
    let controls = dialog_controls(&render_studio(&ctx, &mut app, viewport));

    assert!(control_rect(&controls, "02  Page and pagination · blocked").is_some());
    assert!(control_rect(&controls, "01  Source and scope").is_some());
}

/// The editor column at `measure`, laid out to its own natural height:
/// what the section takes, and how far the widest thing in it reaches.
#[cfg(not(target_arch = "wasm32"))]
fn editor_extent(app: &mut RSpiceApp, section: HardcopySection, measure: f32) -> (f32, f32) {
    app.state.dialogs.hardcopy.section = section;
    let draft = &mut app.state.dialogs.hardcopy;
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
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
    crate::ui::Theme::default().apply(&ctx);
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

/// Whatever the track, the editor takes its measure and the page desk takes
/// the rest — and a column too short for a stacked form buys its height
/// back with the width the desk would have had.
#[test]
fn the_page_desk_takes_every_point_the_form_does_not_need() {
    let tall = STACKED_FORM_HEIGHT;
    for track in [1_050.0_f32, 888.0, 782.0, 768.0] {
        let (editor, preview) = column_widths(track, tall);
        assert_eq!(editor, EDITOR_STACKED_MEASURE);
        assert!((editor + preview - track).abs() < 0.5);
    }
    let (editor, preview) = column_widths(1_050.0, STACKED_FORM_HEIGHT - 1.0);
    assert_eq!(editor, EDITOR_PAIRED_MEASURE);
    assert!((editor + preview - 1_050.0).abs() < 0.5);
    // A track that cannot pay for both gives way from the editor first: a
    // page too small to read is not worth a wider form.
    let (editor, preview) = column_widths(EDITOR_MIN + PREVIEW_MIN, tall);
    assert_eq!((editor, preview), (EDITOR_MIN, PREVIEW_MIN));
}

/// The toolbar's band is measured, not assumed. The desk must give up the
/// exact band the toolbar draws at both wide and constrained widths.
#[test]
fn the_preview_toolbar_reserves_exactly_the_band_it_draws() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply_responsive_metrics_with_target(&ctx, Some(44.0));
    let mut bands = Vec::new();
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            for width in [620.0_f32, 340.0] {
                let view = PreviewView { page: 0, zoom: 100 };
                let reserved = preview_toolbar_height(ui, &view, 4, width);
                let drawn = ui
                    .allocate_ui_with_layout(
                        vec2(width, 400.0),
                        Layout::top_down(Align::Min),
                        |ui| preview_toolbar(ui, &mut { view }, 4, width),
                    )
                    .inner;
                bands.push((width, reserved, drawn));
            }
        });
    });
    for (width, reserved, drawn) in &bands {
        assert!(
            (reserved - drawn).abs() < 0.5,
            "at {width:.0} pt the desk gave up {reserved:.0} pt for a toolbar that drew \
                 {drawn:.0}"
        );
    }
    let row = 44.0 + 12.0;
    assert!(
        bands[0].1 < row + 6.0,
        "a 620 pt column holds the toolbar in one row, but {:.0} pt was reserved",
        bands[0].1
    );
}

/// The tablet width the studio ships to is edge to edge and touch-sized,
/// and the page desk keeps only the column the form does not need. The
/// toolbar has to hold that column in one row: wrapping there costs the
/// page a whole row and buys a strip with one control stranded under six.
#[test]
fn the_preview_toolbar_holds_one_row_at_the_tablet_width() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply_responsive_metrics_with_target(&ctx, Some(44.0));
    let (_, desk) = column_widths(820.0, STACKED_FORM_HEIGHT);
    let mut band = 0.0;
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let view = PreviewView { page: 0, zoom: 100 };
            band = preview_toolbar_height(ui, &view, 4, desk);
        });
    });
    assert!(
        band < 44.0 + 12.0 + 6.0,
        "the toolbar took {band:.0} pt of a {desk:.0} pt column, which is more than one row"
    );
}

#[test]
fn document_type_choice_prefers_active_extent_over_selection() {
    use crate::hardcopy::{HardcopyDocumentKind, HardcopyScope};
    use crate::workbench::hardcopy_adapters::sources::{
        RetainedHardcopySourceAvailability, RetainedHardcopySourceDescriptor,
    };

    let candidate = RetainedHardcopySourceDescriptor {
        source_key: "project:test:schematic".to_owned(),
        display_name: "top / schematic".to_owned(),
        document_kind: HardcopyDocumentKind::SchematicOrSymbol,
        allowed_scopes: vec![
            HardcopyScope::Selection,
            HardcopyScope::CurrentSheet,
            HardcopyScope::AllSheetsOrPanes,
        ],
        availability: RetainedHardcopySourceAvailability::Available,
    };
    let candidates = vec![candidate];
    let choice = source_choice_for_active_extent(&candidates, candidates.first()).unwrap();
    assert_eq!(choice.0, "project:test:schematic");
    assert_eq!(choice.1, HardcopyScope::CurrentSheet);
}
