//! Geometry and visual contracts for schematic context surfaces.
use super::*;
/// A one-row menu on instance 7.
fn one_row_view(summary: Option<String>) -> ContextMenuView {
    ContextMenuView {
        binding: ContextMenuBinding {
            source: EditorRequestSource {
                project: rspice_vocabulary::product::ProjectId::new(),
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
        summary,
        entries: vec![ContextEntry::Command(ContextCommand {
            action: ContextAction::Delete,
            icon: ContextIcon::Trash,
            label: "Delete",
            shortcut: "Delete".to_owned(),
            enabled: true,
            disabled_reason: "",
        })],
        choices: Vec::new(),
        arrange: Vec::new(),
    }
}

#[test]
fn a_menu_with_nothing_to_name_has_no_header() {
    let painted = |summary: Option<String>| {
        let ctx = Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let view = one_row_view(summary);
        ctx.run_ui(egui::RawInput::default(), |ui| {
            let _ = render_context_contents(ui, &view, DESKTOP_ROW_HEIGHT, false);
        })
        .shapes
        .into_iter()
        .any(|clipped| {
            matches!(
                clipped.shape,
                egui::Shape::Text(text) if text.galley.text() == "Schematic selection"
            )
        })
    };

    assert!(painted(Some("R1".to_owned())), "an object's menu keeps it");
    assert!(!painted(None), "a bare canvas has nothing for it to name");
}

#[test]
fn an_open_popup_cannot_retarget_its_action_when_the_source_or_selection_changes() {
    for changed in ["source", "selection", "none"] {
        let ctx = Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let mut view = one_row_view(Some("R1".to_owned()));
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
