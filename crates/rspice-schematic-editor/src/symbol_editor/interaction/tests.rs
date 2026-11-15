//! Local symbol draft edits and their undo intent, without application state.

use super::*;

#[test]
fn finishing_pending_polyline_creates_one_selected_shape() {
    let mut session = SymbolEditorSession::default();
    let mut edit = CanvasEdit {
        session: &mut session,
        capabilities: SymbolEditCapabilities { edit: true },
        outcome: SymbolEditOutcome::default(),
    };
    let mut document = SymbolDocument::default();
    edit.session.pending_polyline = vec![Point::new(0, 0), Point::new(10, 0)];

    assert!(finish_pending_polyline(&mut edit, &mut document));

    assert_eq!(document.body.len(), 1);
    assert_eq!(edit.session.selected_shape, Some(0));
    assert!(edit.session.selection.shapes.contains(&0));
    assert!(edit.session.pending_polyline.is_empty());
}

/// With no pin waiting to be placed, the Pin tool makes one: the first
/// `PIN<n>` free, an analog terminal, as one edit, its name open for typing.
#[test]
fn the_pin_tool_makes_a_pin_when_none_is_waiting() {
    let mut session = SymbolEditorSession::default();
    let mut edit = CanvasEdit {
        session: &mut session,
        capabilities: SymbolEditCapabilities { edit: true },
        outcome: SymbolEditOutcome::default(),
    };
    let mut document = SymbolDocument::default();

    assert!(place_next_pin(&mut edit, &mut document, Point::new(10, 0)));

    assert!(edit.outcome.undo_before.is_some());
    let pin = document.pin("PIN1").expect("the tool made PIN1");
    assert_eq!(pin.position, Some(Point::new(10, 0)));
    assert_eq!(
        pin.electrical_type(),
        rspice_design_model::symbol_pin::SymbolElectricalType::Analog
    );
    assert_eq!(
        edit.session
            .pin_name_draft
            .as_ref()
            .map(|draft| draft.text.as_str()),
        Some("PIN1")
    );
}

/// A drag reports as started only past egui's threshold, by which time a
/// quick hand is well away from the pin it pressed on.
#[test]
fn a_drag_starts_where_the_press_was() {
    let ctx = egui::Context::default();
    let source = SymbolRequestSource {
        project: rspice_vocabulary::product::ProjectId::new(),
        document: rspice_design_model::cell_view::CellViewRef::new("work", "amp", "symbol"),
        occurrence: None,
        design_epoch: 3,
        document_epoch: 4,
        library_revision: 5,
        window: 1,
    };
    let session = SymbolEditorSession::default();
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
    let viewport = SymbolViewport::new(screen, 4.0, egui::Vec2::ZERO);
    let press = egui::pos2(100.0, 100.0);
    let frames = [
        vec![egui::Event::PointerMoved(press)],
        vec![egui::Event::PointerButton {
            pos: press,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }],
        vec![egui::Event::PointerMoved(egui::pos2(140.0, 100.0))],
        vec![egui::Event::PointerMoved(egui::pos2(150.0, 100.0))],
    ];
    let mut started = None;
    for events in frames {
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(screen),
                events,
                ..Default::default()
            },
            |ui| {
                let response = ui.interact(
                    screen,
                    ui.id().with("canvas"),
                    egui::Sense::click_and_drag(),
                );
                if let Some(request) = canvas_request(&source, &session, viewport, &response)
                    && request.input.drag_started
                {
                    started = Some(request.input.pointer);
                }
            },
        );
    }
    assert_eq!(started, Some(press));
}

fn screen() -> egui::Rect {
    egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 400.0))
}

/// One unit to one point, the symbol's origin at the middle of the screen.
fn unit_viewport() -> SymbolViewport {
    SymbolViewport::new(screen(), 1.0, egui::Vec2::ZERO)
}

/// Feed `input` to the canvas at symbol point `at`.
fn feed(
    session: &mut SymbolEditorSession,
    document: &mut SymbolDocument,
    at: Point,
    input: SymbolCanvasInput,
) -> SymbolEditOutcome {
    let mut metadata = SymbolEditorMetadata::for_document(document);
    edit_canvas(
        session,
        document,
        &mut metadata,
        unit_viewport(),
        SymbolCanvasInput {
            pointer: unit_viewport().world_to_screen(at),
            ..input
        },
        SymbolEditCapabilities { edit: true },
    )
}

fn click(session: &mut SymbolEditorSession, document: &mut SymbolDocument, at: Point) -> bool {
    feed(
        session,
        document,
        at,
        SymbolCanvasInput {
            clicked: true,
            ..Default::default()
        },
    )
    .changed
}

fn drag(session: &mut SymbolEditorSession, document: &mut SymbolDocument, from: Point, to: Point) {
    feed(
        session,
        document,
        from,
        SymbolCanvasInput {
            drag_started: true,
            dragged: true,
            ..Default::default()
        },
    );
    feed(
        session,
        document,
        to,
        SymbolCanvasInput {
            dragged: true,
            ..Default::default()
        },
    );
    feed(
        session,
        document,
        to,
        SymbolCanvasInput {
            drag_stopped: true,
            ..Default::default()
        },
    );
}

fn key(
    session: &mut SymbolEditorSession,
    document: &mut SymbolDocument,
    input: SymbolCanvasInput,
) -> bool {
    feed(session, document, Point::origin(), input).changed
}

fn armed(tool: SymbolTool) -> SymbolEditorSession {
    SymbolEditorSession {
        tool,
        ..SymbolEditorSession::default()
    }
}

/// A line is clicked out point by point and finished with Enter, and the
/// tool stays armed for the next one.
#[test]
fn a_line_is_clicked_out_and_finished_with_enter_and_the_tool_stays_armed() {
    let mut session = armed(SymbolTool::Line);
    let mut document = SymbolDocument::default();
    for point in [Point::new(0, 0), Point::new(20, 0), Point::new(20, 10)] {
        assert!(!click(&mut session, &mut document, point));
    }
    assert!(
        document.body.is_empty(),
        "nothing is drawn until it is finished"
    );

    let outcome = feed(
        &mut session,
        &mut document,
        Point::origin(),
        SymbolCanvasInput {
            finish: true,
            ..Default::default()
        },
    );

    assert!(outcome.changed);
    assert!(outcome.undo_before.is_some());
    assert_eq!(
        document.body,
        vec![SymbolShape::Polyline {
            points: vec![Point::new(0, 0), Point::new(20, 0), Point::new(20, 10)],
            closed: false,
        }]
    );
    assert_eq!(session.tool, SymbolTool::Line);
    assert!(session.pending_polyline.is_empty());
    assert!(!click(&mut session, &mut document, Point::new(40, 0)));
    assert_eq!(session.pending_polyline, vec![Point::new(40, 0)]);
}

/// The second click of a double-click ends the line on the point the first
/// placed, without placing it twice.
#[test]
fn a_double_click_ends_the_line_where_it_was_clicked() {
    let mut session = armed(SymbolTool::Line);
    let mut document = SymbolDocument::default();
    click(&mut session, &mut document, Point::new(0, 0));
    click(&mut session, &mut document, Point::new(30, 0));
    let outcome = feed(
        &mut session,
        &mut document,
        Point::new(30, 0),
        SymbolCanvasInput {
            clicked: true,
            double_clicked: true,
            ..Default::default()
        },
    );
    assert!(outcome.changed);
    assert_eq!(
        document.body,
        vec![SymbolShape::Polyline {
            points: vec![Point::new(0, 0), Point::new(30, 0)],
            closed: false,
        }]
    );
}

/// Backspace takes back the last point placed, whatever the tool.
#[test]
fn backspace_takes_back_the_last_point() {
    let back = SymbolCanvasInput {
        remove_last_point: true,
        ..Default::default()
    };
    let mut session = armed(SymbolTool::Polygon);
    let mut document = SymbolDocument::default();
    click(&mut session, &mut document, Point::new(0, 0));
    click(&mut session, &mut document, Point::new(20, 0));
    assert!(!key(&mut session, &mut document, back));
    assert_eq!(session.pending_polyline, vec![Point::new(0, 0)]);

    let mut session = armed(SymbolTool::Arc);
    click(&mut session, &mut document, Point::new(0, 0));
    click(&mut session, &mut document, Point::new(20, 0));
    key(&mut session, &mut document, back);
    assert!(session.arc_sweep.is_none());
    assert_eq!(session.shape_start, Some(Point::new(0, 0)));
    key(&mut session, &mut document, back);
    assert!(!session.has_draft());
    assert!(document.body.is_empty());
}

/// A polygon is always closed: Enter waits for a third corner, and clicking
/// the first corner again closes it.
#[test]
fn a_polygon_always_closes_and_needs_three_corners() {
    let finish = SymbolCanvasInput {
        finish: true,
        ..Default::default()
    };
    let mut session = armed(SymbolTool::Polygon);
    let mut document = SymbolDocument::default();
    click(&mut session, &mut document, Point::new(0, 0));
    click(&mut session, &mut document, Point::new(20, 0));
    let outcome = feed(&mut session, &mut document, Point::origin(), finish);
    assert!(!outcome.changed);
    assert!(outcome.refusal.is_some());
    assert_eq!(session.pending_polyline.len(), 2);

    click(&mut session, &mut document, Point::new(20, 20));
    assert!(key(&mut session, &mut document, finish));
    assert_eq!(
        document.body.last(),
        Some(&SymbolShape::Polyline {
            points: vec![Point::new(0, 0), Point::new(20, 0), Point::new(20, 20)],
            closed: true,
        })
    );

    for point in [Point::new(40, 0), Point::new(60, 0), Point::new(60, 20)] {
        click(&mut session, &mut document, point);
    }
    assert!(click(&mut session, &mut document, Point::new(40, 0)));
    assert_eq!(
        document.body.last(),
        Some(&SymbolShape::Polyline {
            points: vec![Point::new(40, 0), Point::new(60, 0), Point::new(60, 20)],
            closed: true,
        })
    );
    assert_eq!(session.tool, SymbolTool::Polygon);
}

/// Press-drag draws as well as clicking: a line segment, a rectangle from
/// corner to corner, a circle by its radius.
#[test]
fn press_drag_draws_a_segment_a_rectangle_and_a_circle() {
    let mut document = SymbolDocument::default();
    let mut line = armed(SymbolTool::Line);
    drag(
        &mut line,
        &mut document,
        Point::new(0, 0),
        Point::new(30, 10),
    );
    let mut rectangle = armed(SymbolTool::Rectangle);
    drag(
        &mut rectangle,
        &mut document,
        Point::new(-10, -10),
        Point::new(10, 20),
    );
    let mut circle = armed(SymbolTool::Circle);
    drag(
        &mut circle,
        &mut document,
        Point::new(50, 50),
        Point::new(50, 60),
    );

    assert_eq!(
        document.body,
        vec![
            SymbolShape::Polyline {
                points: vec![Point::new(0, 0), Point::new(30, 10)],
                closed: false,
            },
            rectangle_shape(Point::new(-10, -10), Point::new(10, 20)),
            SymbolShape::Circle {
                center: Point::new(50, 50),
                radius: 10,
            },
        ]
    );
    for session in [&line, &rectangle, &circle] {
        assert!(!session.has_draft());
        assert!(session.tool.draws(), "the tool stays armed");
    }
}

/// The rectangle tool draws the rectangle kind, named by its least and
/// greatest corners whichever way the pointer went.
#[test]
fn the_rectangle_tool_draws_a_rectangle_whichever_way_it_is_dragged() {
    let mut document = SymbolDocument::default();
    let mut session = armed(SymbolTool::Rectangle);
    drag(
        &mut session,
        &mut document,
        Point::new(30, 20),
        Point::new(-10, -20),
    );

    assert_eq!(
        document.body,
        vec![SymbolShape::Rectangle {
            min: Point::new(-10, -20),
            max: Point::new(30, 20),
        }]
    );
}

/// Each canvas edit names the step Undo and Redo take: drawing a rectangle
/// is "Add rectangle", dragging it is "Move rectangle".
#[test]
fn canvas_edits_name_their_undo_step() {
    let mut document = SymbolDocument::default();
    let mut session = armed(SymbolTool::Rectangle);
    let clicked = SymbolCanvasInput {
        clicked: true,
        ..Default::default()
    };
    feed(&mut session, &mut document, Point::new(40, 40), clicked);
    let drawn = feed(&mut session, &mut document, Point::new(80, 80), clicked);
    assert!(drawn.undo_before.is_some());
    assert_eq!(drawn.step, "Add rectangle");

    let mut session = armed(SymbolTool::Select);
    feed(
        &mut session,
        &mut document,
        Point::new(40, 60),
        SymbolCanvasInput {
            drag_started: true,
            dragged: true,
            ..Default::default()
        },
    );
    let moved = feed(
        &mut session,
        &mut document,
        Point::new(50, 60),
        SymbolCanvasInput {
            dragged: true,
            ..Default::default()
        },
    );
    assert!(moved.undo_before.is_some());
    assert_eq!(moved.step, "Move rectangle");
}

/// An armed drawing tool draws over what is there: pressing on an existing
/// shape neither selects it nor moves it.
#[test]
fn an_armed_drawing_tool_never_picks_up_what_is_drawn() {
    let circle = SymbolShape::Circle {
        center: Point::origin(),
        radius: 20,
    };
    let mut document = SymbolDocument {
        body: vec![circle.clone()],
        ..SymbolDocument::default()
    };
    let mut session = armed(SymbolTool::Rectangle);
    drag(
        &mut session,
        &mut document,
        Point::new(20, 0),
        Point::new(40, 30),
    );

    assert_eq!(document.body[0], circle);
    assert_eq!(
        document.body[1],
        rectangle_shape(Point::new(20, 0), Point::new(40, 30))
    );
    assert!(session.dragging_shape.is_none());
    assert!(session.marquee_start.is_none());

    let mut session = armed(SymbolTool::Line);
    assert!(!click(&mut session, &mut document, Point::new(20, 0)));
    assert_eq!(session.pending_polyline, vec![Point::new(20, 0)]);
    assert!(!session.effective_selection().shapes.contains(&0));
}

/// The arc is its center, its start and its end, and runs from the start the
/// way the pointer turned: the long way round when the pointer went that way.
#[test]
fn an_arc_runs_from_its_start_the_way_the_pointer_turned() {
    let mut document = SymbolDocument::default();
    let mut session = armed(SymbolTool::Arc);
    click(&mut session, &mut document, Point::new(0, 0));
    click(&mut session, &mut document, Point::new(20, 0));
    // Down (clockwise on screen), round through the left, up to the top.
    for point in [Point::new(10, 10), Point::new(0, 20), Point::new(-20, 0)] {
        track_pointer(
            &mut session,
            unit_viewport(),
            Some(unit_viewport().world_to_screen(point)),
        );
    }
    assert!(click(&mut session, &mut document, Point::new(0, -20)));

    assert_eq!(
        document.body,
        vec![SymbolShape::Arc {
            center: Point::new(0, 0),
            radius: 20,
            start_degrees: 0,
            sweep_degrees: 270,
        }]
    );
    assert_eq!(session.tool, SymbolTool::Arc);

    // Straight to the top from the start turns the other way: a quarter.
    click(&mut session, &mut document, Point::new(0, 0));
    click(&mut session, &mut document, Point::new(20, 0));
    click(&mut session, &mut document, Point::new(0, -20));
    assert_eq!(
        document.body.last(),
        Some(&SymbolShape::Arc {
            center: Point::new(0, 0),
            radius: 20,
            start_degrees: 0,
            sweep_degrees: -90,
        })
    );
}

/// A circle may be as small as one unit, and one with no radius is not
/// drawn: the tool waits for a point off the center.
#[test]
fn a_circle_may_be_one_unit_and_never_zero() {
    let mut document = SymbolDocument::default();
    let mut session = armed(SymbolTool::Circle);
    session.snap_to_grid = false;
    click(&mut session, &mut document, Point::new(5, 5));
    assert!(!click(&mut session, &mut document, Point::new(5, 5)));
    assert_eq!(session.shape_start, Some(Point::new(5, 5)));
    assert!(click(&mut session, &mut document, Point::new(6, 5)));
    assert_eq!(
        document.body,
        vec![SymbolShape::Circle {
            center: Point::new(5, 5),
            radius: 1,
        }]
    );
}

/// The text tool opens a draft where it is clicked; nothing is placed until
/// the host commits the entry.
#[test]
fn the_text_tool_opens_a_draft_and_places_nothing() {
    let mut document = SymbolDocument::default();
    let mut session = armed(SymbolTool::Text);
    assert!(!click(&mut session, &mut document, Point::new(10, -20)));
    assert!(document.body.is_empty());
    assert_eq!(
        session.text_draft,
        Some(SymbolTextDraft {
            anchor: Point::new(10, -20),
            text: String::new(),
            caret_placed: false,
        })
    );
}

/// A read-only symbol takes no point at all, so nothing part drawn can be
/// finished on it.
#[test]
fn a_read_only_symbol_takes_no_point() {
    let mut document = SymbolDocument::default();
    let mut session = armed(SymbolTool::Line);
    let mut metadata = SymbolEditorMetadata::for_document(&document);
    let outcome = edit_canvas(
        &mut session,
        &mut document,
        &mut metadata,
        unit_viewport(),
        SymbolCanvasInput {
            clicked: true,
            ..Default::default()
        },
        SymbolEditCapabilities { edit: false },
    );
    assert!(outcome.edit_denied);
    assert!(session.pending_polyline.is_empty());
}

fn run_canvas(
    ctx: &egui::Context,
    session: &SymbolEditorSession,
    events: Vec<egui::Event>,
) -> Option<SymbolCanvasRequest> {
    let source = SymbolRequestSource {
        project: rspice_vocabulary::product::ProjectId::new(),
        document: rspice_design_model::cell_view::CellViewRef::new("work", "amp", "symbol"),
        occurrence: None,
        design_epoch: 1,
        document_epoch: 1,
        library_revision: 1,
        window: 1,
    };
    let mut request = None;
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(screen()),
            events,
            ..Default::default()
        },
        |ui| {
            let response = ui.interact(
                screen(),
                ui.id().with("canvas"),
                egui::Sense::click_and_drag(),
            );
            response.request_focus();
            request = canvas_request(&source, session, unit_viewport(), &response);
        },
    );
    request
}

fn key_event(key: egui::Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }
}

/// Enter and Backspace reach the shape being drawn while the canvas holds
/// the keyboard, and are left alone when nothing is part drawn.
#[test]
fn enter_and_backspace_belong_to_the_shape_being_drawn() {
    let ctx = egui::Context::default();
    let mut session = armed(SymbolTool::Line);
    let _ = run_canvas(&ctx, &session, vec![]);

    assert!(run_canvas(&ctx, &session, vec![key_event(egui::Key::Enter)]).is_none());

    session.pending_polyline = vec![Point::origin(), Point::new(10, 0)];
    let finish = run_canvas(&ctx, &session, vec![key_event(egui::Key::Enter)])
        .expect("Enter finishes the line");
    assert!(finish.input.finish);
    let back = run_canvas(&ctx, &session, vec![key_event(egui::Key::Backspace)])
        .expect("Backspace takes back a point");
    assert!(back.input.remove_last_point);
}

/// Right-click while drawing does to the shape what it does to a route on the
/// schematic's wire tool: nothing. It neither finishes nor cancels it.
#[test]
fn a_right_click_leaves_the_shape_being_drawn_alone() {
    let ctx = egui::Context::default();
    let mut session = armed(SymbolTool::Line);
    session.pending_polyline = vec![Point::origin(), Point::new(10, 0)];
    let at = egui::pos2(250.0, 210.0);
    let right = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Secondary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    let mut requests = Vec::new();
    for events in [
        vec![egui::Event::PointerMoved(at)],
        vec![right(true)],
        vec![right(false)],
    ] {
        requests.extend(run_canvas(&ctx, &session, events));
    }
    assert!(requests.is_empty(), "{requests:?}");
}

/// A body box with a stray line far below and to its left, drawn before any
/// pin was placed: the artwork that once turned a pin placed beside the
/// body onto the wrong side.
fn body_with_stray_artwork() -> SymbolDocument {
    SymbolDocument {
        pins: vec![SymbolPin::new(
            "A",
            rspice_design_model::port::PortDirection::In,
            None,
        )],
        body: vec![
            rectangle_shape(Point::new(-20, -20), Point::new(20, 20)),
            SymbolShape::Polyline {
                points: vec![Point::new(-200, 150), Point::new(-190, 150)],
                closed: false,
            },
        ],
        ..SymbolDocument::default()
    }
}

/// A pin placed beside the body takes the side of the body it stands
/// beside, whatever stray artwork is elsewhere, and keeps that side and its
/// lead: dragging moves its tip and nothing else, and artwork drawn later
/// moves nothing.
#[test]
fn a_pin_placed_beside_stray_artwork_keeps_its_side_and_its_lead() {
    use rspice_design_model::symbol_pin::SymbolPinSide;
    let mut document = body_with_stray_artwork();
    let mut session = armed(SymbolTool::PlacePin);
    session.select_pin("A");
    assert!(click(&mut session, &mut document, Point::new(-40, 0)));
    let pin = document.pin("A").expect("A");
    assert_eq!(pin.side, SymbolPinSide::Left);
    assert_eq!(pin.lead_inner(), Some(Point::new(-20, 0)));

    document.body.push(SymbolShape::Polyline {
        points: vec![Point::new(-60, -90), Point::new(-60, 90)],
        closed: false,
    });
    session.tool = SymbolTool::Select;
    drag(
        &mut session,
        &mut document,
        Point::new(-40, 0),
        Point::new(-40, -50),
    );
    let pin = document.pin("A").expect("A");
    assert_eq!(pin.position, Some(Point::new(-40, -50)));
    assert_eq!(pin.side, SymbolPinSide::Left);
    assert_eq!(pin.lead_inner(), Some(Point::new(-20, -50)));
}

/// The Pin tool puts down what is waiting first: a pin the symbol has that
/// is not placed, then a port the interface declares that has no pin, then
/// the first free `PIN<n>`. It stays armed, and each pin it places opens its
/// name for typing.
#[test]
fn the_pin_tool_offers_waiting_pins_then_declared_ports_then_numbered_pins() {
    use rspice_design_model::port::{PortDirection, PortSpec};
    let mut document = body_with_stray_artwork();
    document.pins.push(SymbolPin::new(
        "PIN1",
        PortDirection::InOut,
        Some(Point::new(40, 10)),
    ));
    let mut session = armed(SymbolTool::PlacePin);
    session.declared_ports = ["A", "B"]
        .into_iter()
        .map(|name| PortSpec {
            name: name.to_owned(),
            direction: PortDirection::Out,
        })
        .collect();

    for (at, name) in [
        (Point::new(-40, 0), "A"),
        (Point::new(40, -10), "B"),
        (Point::new(0, -40), "PIN2"),
    ] {
        assert_eq!(next_pin_offer(&document, &session).name(), name);
        assert!(click(&mut session, &mut document, at), "{name}");
        assert_eq!(document.pin(name).and_then(|pin| pin.position), Some(at));
        assert_eq!(
            session
                .pin_name_draft
                .as_ref()
                .map(|draft| draft.pin.as_str()),
            Some(name)
        );
        assert_eq!(session.tool, SymbolTool::PlacePin);
    }
    assert_eq!(
        document.pin("B").map(|pin| pin.direction),
        Some(PortDirection::Out),
        "a declared port's pin takes the port's direction"
    );
    assert_eq!(document.pins.len(), 4);
}

/// The Pin tool's pin takes the side of the artwork nearest it until R
/// turns it; then it keeps the side it was turned to, wherever it goes.
#[test]
fn r_turns_the_pin_tool_pin_and_it_keeps_that_side() {
    use rspice_design_model::symbol_pin::SymbolPinSide;
    let mut document = body_with_stray_artwork();
    document.pins.clear();
    let mut session = armed(SymbolTool::PlacePin);
    assert_eq!(
        pin_tool_side(&document, &session, Point::new(-40, 0)),
        SymbolPinSide::Left
    );
    turn_pin_tool_side(&mut session, &document, Some(Point::new(-40, 0)));
    assert_eq!(session.pin_side, Some(SymbolPinSide::Top));

    assert!(click(&mut session, &mut document, Point::new(-40, 0)));
    assert!(click(&mut session, &mut document, Point::new(40, 0)));
    let sides = document
        .pins
        .iter()
        .map(|pin| (pin.name.as_str(), pin.side))
        .collect::<Vec<_>>();
    assert_eq!(
        sides,
        [("PIN1", SymbolPinSide::Top), ("PIN2", SymbolPinSide::Top)]
    );
}

/// The Pin tool never stacks a pin on another: the click is refused and
/// says why, and nothing changes.
#[test]
fn the_pin_tool_refuses_a_point_another_pin_stands_on() {
    let mut document = body_with_stray_artwork();
    document.pins[0].position = Some(Point::new(-40, 0));
    let mut session = armed(SymbolTool::PlacePin);
    let before = document.clone();
    let outcome = feed(
        &mut session,
        &mut document,
        Point::new(-40, 0),
        SymbolCanvasInput {
            clicked: true,
            ..Default::default()
        },
    );
    assert!(!outcome.changed);
    assert!(outcome.undo_before.is_none());
    assert!(
        outcome
            .refusal
            .is_some_and(|why| why.contains("pin A already stands there"))
    );
    assert_eq!(document, before);
}

/// A document holding `shape` alone, selected, with Select armed.
fn selected(shape: SymbolShape) -> (SymbolEditorSession, SymbolDocument) {
    let mut session = armed(SymbolTool::Select);
    session.select_shape(0);
    let document = SymbolDocument {
        body: vec![shape],
        ..SymbolDocument::default()
    };
    (session, document)
}

/// Press at `from`, carry the pointer to `to` and let go: each frame's
/// outcome, so the undo steps the drag records can be read.
fn drag_steps(
    session: &mut SymbolEditorSession,
    document: &mut SymbolDocument,
    from: Point,
    to: Point,
) -> Vec<SymbolEditOutcome> {
    let pressed = SymbolCanvasInput {
        drag_started: true,
        dragged: true,
        ..Default::default()
    };
    let carried = SymbolCanvasInput {
        dragged: true,
        ..Default::default()
    };
    let released = SymbolCanvasInput {
        drag_stopped: true,
        ..Default::default()
    };
    vec![
        feed(session, document, from, pressed),
        feed(session, document, to, carried),
        feed(session, document, to, released),
    ]
}

/// The undo steps a drag recorded, by name.
fn recorded(outcomes: &[SymbolEditOutcome]) -> Vec<&str> {
    outcomes
        .iter()
        .filter(|outcome| outcome.undo_before.is_some())
        .map(|outcome| outcome.step.as_str())
        .collect()
}

/// A point of the selected line follows the pointer, snapped as drawing
/// snaps, as one undo step; the rest of the line stays.
#[test]
fn dragging_a_point_of_the_selected_line_moves_that_point_alone() {
    let line = SymbolShape::Polyline {
        points: vec![Point::new(0, 0), Point::new(40, 0), Point::new(40, 40)],
        closed: false,
    };
    let (mut session, mut document) = selected(line.clone());
    let steps = drag_steps(
        &mut session,
        &mut document,
        Point::new(40, 0),
        Point::new(57, -12),
    );

    assert_eq!(
        document.body[0],
        SymbolShape::Polyline {
            points: vec![Point::new(0, 0), Point::new(60, -10), Point::new(40, 40)],
            closed: false,
        }
    );
    assert_eq!(recorded(&steps), vec!["Move vertex"]);
    let before = steps
        .iter()
        .find_map(|outcome| outcome.undo_before.as_ref());
    assert_eq!(before.map(|document| &document.body[0]), Some(&line));
    assert_eq!(session.selected_vertex, Some((0, 1)));
}

/// Grabbing an edge's middle puts a point there, which the drag carries.
#[test]
fn grabbing_an_edge_middle_adds_a_point_there() {
    let (mut session, mut document) = selected(SymbolShape::Polyline {
        points: vec![Point::new(0, 0), Point::new(40, 0)],
        closed: false,
    });
    let steps = drag_steps(
        &mut session,
        &mut document,
        Point::new(20, 0),
        Point::new(20, 20),
    );

    assert_eq!(
        document.body[0],
        SymbolShape::Polyline {
            points: vec![Point::new(0, 0), Point::new(20, 20), Point::new(40, 0)],
            closed: false,
        }
    );
    assert!(steps[0].changed);
    assert_eq!(recorded(&steps), vec!["Add vertex"]);
}

/// A rectangle's corner carries that corner and holds the opposite one; a
/// side carries that side alone; it stays the rectangle kind throughout.
#[test]
fn a_rectangle_is_resized_by_its_corners_and_sides_and_stays_one() {
    let (mut session, mut document) = selected(SymbolShape::Rectangle {
        min: Point::new(0, 0),
        max: Point::new(40, 20),
    });
    let steps = drag_steps(
        &mut session,
        &mut document,
        Point::new(40, 20),
        Point::new(60, 30),
    );
    assert_eq!(
        document.body[0],
        SymbolShape::Rectangle {
            min: Point::new(0, 0),
            max: Point::new(60, 30),
        }
    );
    assert_eq!(recorded(&steps), vec!["Resize rectangle"]);

    // The left side's middle, pulled up and out: only x follows.
    drag_steps(
        &mut session,
        &mut document,
        Point::new(0, 15),
        Point::new(-20, 0),
    );
    assert_eq!(
        document.body[0],
        SymbolShape::Rectangle {
            min: Point::new(-20, 0),
            max: Point::new(60, 30),
        }
    );
}

/// A circle's radius handle sets its radius; an arc's end turns the arc
/// about its centre and keeps its start; neither moves its centre.
#[test]
fn a_circle_and_an_arc_are_reshaped_by_their_handles() {
    let (mut session, mut document) = selected(SymbolShape::Circle {
        center: Point::origin(),
        radius: 20,
    });
    let steps = drag_steps(
        &mut session,
        &mut document,
        Point::new(20, 0),
        Point::new(30, 0),
    );
    assert_eq!(
        document.body[0],
        SymbolShape::Circle {
            center: Point::origin(),
            radius: 30,
        }
    );
    assert_eq!(recorded(&steps), vec!["Resize circle"]);

    // Clockwise on the screen from east to south; the end is carried west.
    let (mut session, mut document) = selected(SymbolShape::Arc {
        center: Point::origin(),
        radius: 20,
        start_degrees: 0,
        sweep_degrees: 90,
    });
    let steps = drag_steps(
        &mut session,
        &mut document,
        Point::new(0, 20),
        Point::new(-20, 0),
    );
    assert_eq!(
        document.body[0],
        SymbolShape::Arc {
            center: Point::origin(),
            radius: 20,
            start_degrees: 0,
            sweep_degrees: 180,
        }
    );
    assert_eq!(recorded(&steps), vec!["Move arc end"]);
}

/// A click on a point of the selected line picks it, and Delete then takes
/// that point out and leaves the line; a line keeps two points.
#[test]
fn delete_takes_out_the_picked_point_and_a_line_keeps_two() {
    use super::super::commands::{SymbolEditAction, apply_edit};
    let (mut session, mut document) = selected(SymbolShape::Polyline {
        points: vec![Point::new(0, 0), Point::new(40, 0), Point::new(40, 40)],
        closed: false,
    });
    assert!(!click(&mut session, &mut document, Point::new(40, 0)));
    assert_eq!(session.selected_vertex, Some((0, 1)));
    assert_eq!(
        session.effective_selection(),
        SymbolSelection::single_shape(0)
    );

    let delete = |session: &mut SymbolEditorSession, document: &mut SymbolDocument| {
        let mut metadata = SymbolEditorMetadata::for_document(document);
        apply_edit(
            session,
            document,
            Some(&mut metadata),
            SymbolEditAction::Delete { cut: false },
            SymbolEditCapabilities { edit: true },
        )
    };
    let outcome = delete(&mut session, &mut document);
    assert!(outcome.changed);
    assert_eq!(outcome.step, "Delete vertex");
    assert_eq!(
        document.body,
        vec![SymbolShape::Polyline {
            points: vec![Point::new(0, 0), Point::new(40, 40)],
            closed: false,
        }]
    );

    assert!(!click(&mut session, &mut document, Point::new(40, 40)));
    let before = document.clone();
    let outcome = delete(&mut session, &mut document);
    assert!(!outcome.changed);
    assert!(outcome.undo_before.is_none());
    assert_eq!(
        outcome.refusal.as_deref(),
        Some("A line keeps at least two points; delete the line instead.")
    );
    assert_eq!(document, before);
}

/// Stretch takes hold of the handle under the pointer; the handle follows
/// the pointer, changing nothing, until a click puts it down as one edit.
/// Escape lets it go where it was.
#[test]
fn stretch_carries_the_handle_under_the_pointer_until_a_click() {
    use super::super::commands::finish_cancel;
    let rectangle = SymbolShape::Rectangle {
        min: Point::new(0, 0),
        max: Point::new(40, 20),
    };
    let (mut session, mut document) = selected(rectangle.clone());
    let corner = unit_viewport().world_to_screen(Point::new(40, 20));
    track_handle(&mut session, &document, unit_viewport(), Some(corner));
    assert!(arm_stretch(&mut session));

    let hover = feed(
        &mut session,
        &mut document,
        Point::new(60, 40),
        SymbolCanvasInput::default(),
    );
    assert!(!hover.changed);
    assert_eq!(document.body[0], rectangle);

    let put_down = feed(
        &mut session,
        &mut document,
        Point::new(60, 40),
        SymbolCanvasInput {
            clicked: true,
            ..Default::default()
        },
    );
    assert!(put_down.changed);
    assert_eq!(put_down.step, "Resize rectangle");
    assert_eq!(
        document.body[0],
        SymbolShape::Rectangle {
            min: Point::new(0, 0),
            max: Point::new(60, 40),
        }
    );
    assert!(session.stretch.is_none());

    let corner = unit_viewport().world_to_screen(Point::new(60, 40));
    track_handle(&mut session, &document, unit_viewport(), Some(corner));
    assert!(arm_stretch(&mut session));
    finish_cancel(&mut session);
    assert!(session.stretch.is_none());
    assert_eq!(session.tool, SymbolTool::Select);

    // Nothing under the pointer: nothing to take hold of.
    track_handle(&mut session, &document, unit_viewport(), None);
    assert!(!arm_stretch(&mut session));
}

/// With more selected, grabbing a dot moves them all rather than the dot
/// alone by the point it stands at.
#[test]
fn a_shape_merely_standing_at_a_point_moves_with_the_rest_of_the_selection() {
    let mut document = SymbolDocument {
        body: vec![
            SymbolShape::Dot {
                center: Point::new(0, 0),
                radius: 2,
            },
            SymbolShape::Circle {
                center: Point::new(40, 0),
                radius: 10,
            },
        ],
        ..SymbolDocument::default()
    };
    let mut session = armed(SymbolTool::Select);
    session.set_selection(SymbolSelection {
        shapes: [0, 1].into_iter().collect(),
        ..SymbolSelection::default()
    });
    drag_steps(
        &mut session,
        &mut document,
        Point::new(0, 0),
        Point::new(10, 10),
    );
    assert_eq!(
        document.body,
        vec![
            SymbolShape::Dot {
                center: Point::new(10, 10),
                radius: 2,
            },
            SymbolShape::Circle {
                center: Point::new(50, 10),
                radius: 10,
            },
        ]
    );
}
