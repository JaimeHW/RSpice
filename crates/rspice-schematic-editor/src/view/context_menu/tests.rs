//! Geometry and visual contracts for schematic context surfaces.
use super::*;

fn catalog(commands: usize, separators: usize) -> Vec<ContextEntry<()>> {
    std::iter::repeat_n(ContextEntry::Command(()), commands)
        .chain(std::iter::repeat_n(ContextEntry::Separator, separators))
        .collect()
}

/// The menu each target gets, as rows and rules: the app's catalogs, pinned
/// label for label in `rspice-app`'s `schematic/view/context_menu/tests.rs`.
///
/// An instance, a Verilog-A instance (whose Descend row reads "Open Verilog-A
/// view") and a wire have nineteen rows under four rules; a placed source
/// trades the three master rows for its stimulus group under a fifth; the
/// empty canvas has one verb, "New Verilog-A device…", where the three
/// master rows stand.
const INSTANCE: (usize, usize) = (19, 4);
const SOURCE: (usize, usize) = (19, 5);
const CANVAS: (usize, usize) = (17, 4);

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
    let entries = catalog(INSTANCE.0, INSTANCE.1);
    let desktop =
        SurfaceGeometry::for_viewport(vec2(1440.0, 900.0), ContextInvocation::Pointer, &entries);
    assert_eq!(desktop.width, 286.0);
    assert_eq!(desktop.max_height, 608.0);
    assert_eq!(desktop.row_height, 27.0);
    assert_eq!(desktop.radius, 3);
    // Every target's menu, measured on the mockup's 27 px row: 47 header +
    // rows + 9 px per separator + 2 border. `outer_height` clamps to
    // `max_height`, so measuring strictly under the ceiling is the same
    // statement as "no row is below the fold": a future entry that would turn
    // a menu into a scroller fails here rather than silently hiding the
    // entries it pushed past the edge.
    for (target, (commands, separators), height) in [
        // An instance, a Verilog-A instance or a wire: 513 rows + 36 rules.
        ("instance", INSTANCE, 598.0),
        // The tallest there is: a placed source whose standing calls for
        // three stimulus verbs, at a fifth separator.
        ("placed source", SOURCE, 607.0),
        // The empty canvas: one verb where an instance has three.
        ("canvas", CANVAS, 544.0),
    ] {
        let menu = SurfaceGeometry::for_viewport(
            vec2(1440.0, 900.0),
            ContextInvocation::Pointer,
            &catalog(commands, separators),
        );
        assert_eq!(menu.outer_height(), height, "{target}");
        assert!(
            menu.outer_height() < menu.max_height,
            "the {target} context menu must fit without scrolling"
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
    let screen = Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 700.0));
    let desktop = SurfaceGeometry::for_viewport(
        screen.size(),
        ContextInvocation::Pointer,
        &catalog(INSTANCE.0, INSTANCE.1),
    );

    assert_eq!(
        clamp_desktop_surface_origin(screen, pos2(-20.0, -10.0), desktop),
        pos2(6.0, 6.0)
    );
    // A click near the bottom edge lifts the whole 598 px surface so it
    // hangs off neither the right edge nor the bottom: 700 - 598 - 6.
    assert_eq!(
        clamp_desktop_surface_origin(screen, pos2(790.0, 690.0), desktop),
        pos2(508.0, 96.0)
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
