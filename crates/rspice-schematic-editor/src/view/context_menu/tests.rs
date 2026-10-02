//! Geometry and visual contracts for schematic context surfaces.
use super::*;

fn catalog(separators: usize) -> Vec<ContextEntry<()>> {
    std::iter::repeat_n(ContextEntry::Command(()), 16)
        .chain(std::iter::repeat_n(ContextEntry::Separator, separators))
        .collect()
}

#[test]
fn an_open_popup_cannot_retarget_its_action_when_the_source_or_selection_changes() {
    for changed in ["source", "selection", "none"] {
        let ctx = Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let mut view = ContextMenuView {
            binding: ContextMenuBinding {
                source: EditorRequestSource {
                    project: rspice_app_types::product::ProjectId::new(),
                    document: rspice_design_model::cell_view::CellViewRef::new(
                        "work",
                        "amp",
                        "schematic",
                    ),
                    occurrence: None,
                    design_epoch: 1,
                    document_epoch: 2,
                    content_version: 3,
                    topology_version: 4,
                    symbol_revision: 5,
                    sheet: None,
                },
                selection: Selection::default(),
                tool: Tool::Select,
                target: ContextTarget::Component(7),
                click_pos: Point::new(20, 30),
            },
            summary: "R1".to_owned(),
            entries: vec![ContextEntry::Command(ContextCommand {
                action: ContextAction::Delete,
                icon: ContextIcon::Trash,
                label: "Delete",
                shortcut: "Delete".to_owned(),
                enabled: true,
                disabled_reason: "",
            })],
        };
        view.binding.selection.select_only_component(7);
        let original = view.binding.clone();
        let mut requested = None;
        let mut open = false;
        for frame in 0..2 {
            if frame == 1 {
                match changed {
                    "source" => view.binding.source.document_epoch += 1,
                    "selection" => view.binding.selection.select_only_component(8),
                    _ => {}
                }
            }
            let _ = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(800.0, 700.0))),
                    events: if frame == 1 {
                        vec![egui::Event::Key {
                            key: Key::Enter,
                            physical_key: Some(Key::Enter),
                            pressed: true,
                            repeat: false,
                            modifiers: Modifiers::NONE,
                        }]
                    } else {
                        Vec::new()
                    },
                    ..Default::default()
                },
                |root| {
                    egui::CentralPanel::default().show(root, |ui| {
                        let (_, response) =
                            ui.allocate_exact_size(vec2(600.0, 500.0), Sense::click());
                        let opening = opening(&response, frame == 0);
                        let output = show(&response, opening, None, &view);
                        if output.request.is_some() {
                            requested = output.request;
                        }
                        open = output.open;
                    });
                },
            );
            if frame == 0 {
                assert!(open, "menu should open");
            }
        }
        assert!(!open);
        assert_eq!(requested.is_some(), changed == "none", "{changed}");
        if let Some(request) = requested {
            assert_eq!(request.binding, original);
            assert_eq!(request.action, ContextAction::Delete);
        }
    }
}

#[test]
fn surface_geometry_matches_desktop_and_touch_contracts() {
    let entries = catalog(4);
    let desktop =
        SurfaceGeometry::for_viewport(vec2(1440.0, 900.0), ContextInvocation::Pointer, &entries);
    assert_eq!(desktop.width, 286.0);
    assert_eq!(desktop.max_height, 528.0);
    assert_eq!(desktop.row_height, 27.0);
    assert_eq!(desktop.radius, 3);
    // Sixteen rows and four separators on the mockup's 27 px row measure
    // 517 px: 47 header + 432 rows + 36 separators + 2 border.
    assert_eq!(desktop.outer_height(), 517.0);
    // `outer_height` clamps to `max_height`, so measuring strictly under
    // the ceiling is the same statement as "no row is below the fold".
    // A future entry that would turn the menu into a scroller fails here
    // rather than silently hiding the entries it pushed past the edge.
    assert!(
        desktop.outer_height() < desktop.max_height,
        "the desktop context menu must fit without scrolling"
    );
    // The tallest menu there is: a placed source whose standing calls for
    // three stimulus verbs, at sixteen rows and a fifth separator.
    {
        let source = SurfaceGeometry::for_viewport(
            vec2(1440.0, 900.0),
            ContextInvocation::Pointer,
            &catalog(5),
        );
        assert_eq!(source.outer_height(), 526.0);
        assert!(
            source.outer_height() < source.max_height,
            "a placed source's context menu must fit without scrolling"
        );
    }

    let touch =
        SurfaceGeometry::for_viewport(vec2(390.0, 844.0), ContextInvocation::TouchSheet, &entries);
    assert_eq!(touch.width, 374.0);
    assert_eq!(touch.max_height, 560.0);
    assert_eq!(touch.row_height, 44.0);
    assert_eq!(touch.radius, 7);
    assert_eq!(touch.outer_height(), 560.0);

    let short_touch =
        SurfaceGeometry::for_viewport(vec2(1024.0, 500.0), ContextInvocation::TouchSheet, &entries);
    assert_eq!(short_touch.width, 420.0);
    assert_eq!(short_touch.max_height, 350.0);
    assert_eq!(short_touch.outer_height(), 350.0);
}

#[test]
fn desktop_origin_and_keyboard_anchor_match_the_mockup_contract() {
    let screen = Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0));
    let desktop =
        SurfaceGeometry::for_viewport(screen.size(), ContextInvocation::Pointer, &catalog(4));

    assert_eq!(
        clamp_desktop_surface_origin(screen, pos2(-20.0, -10.0), desktop),
        pos2(6.0, 6.0)
    );
    // A click near the bottom edge lifts the whole 517 px surface so it
    // hangs off neither the right edge nor the bottom: 600 - 517 - 6.
    assert_eq!(
        clamp_desktop_surface_origin(screen, pos2(790.0, 590.0), desktop),
        pos2(508.0, 77.0)
    );
    assert_eq!(
        keyboard_surface_anchor(Rect::from_min_size(pos2(100.0, 200.0), vec2(1000.0, 500.0),)),
        pos2(124.0, 224.0)
    );
    assert_eq!(
        keyboard_surface_anchor(Rect::from_min_size(pos2(100.0, 200.0), vec2(20.0, 10.0),)),
        pos2(110.0, 205.0)
    );
}

#[test]
fn context_shadow_matches_dark_and_light_mockup_tokens() {
    let dark = Tokens::new(
        tokens::Direction::Instrument,
        tokens::Mode::Dark,
        tokens::Density::default(),
    );
    let light = Tokens::new(
        tokens::Direction::Instrument,
        tokens::Mode::Light,
        tokens::Density::default(),
    );

    assert_eq!(
        context_shadow(&dark),
        Shadow {
            offset: [0, 16],
            blur: 40,
            spread: 0,
            color: Color32::from_rgba_premultiplied(0, 0, 0, 97),
        }
    );
    assert_eq!(
        context_shadow(&light),
        Shadow {
            offset: [0, 16],
            blur: 40,
            spread: 0,
            color: Color32::from_rgba_premultiplied(9, 10, 11, 56),
        }
    );
}
