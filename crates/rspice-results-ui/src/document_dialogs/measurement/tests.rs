use super::*;

fn preview(value: Result<Option<String>, String>) -> MeasurementPreview {
    MeasurementPreview {
        pane: "Run 1 (8:31:04 PM)".to_owned(),
        signals: vec!["V(in)".to_owned(), "V(out)".to_owned()],
        signal_units: vec!["V".to_owned(), "V".to_owned()],
        value,
        refusal: None,
    }
}

/// Before anything is typed, the line under Expression says what Add
/// waits for, so no blank slot stands between the field and the footer.
#[test]
fn an_empty_expression_says_what_add_waits_for() {
    let ctx = Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut draft = MeasurementDraft::default();
    let preview = preview(Ok(None));
    let mut said = false;
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..Default::default()
            },
            |ui| {
                show(ui.ctx(), &mut draft, &preview);
            },
        );
        said |= output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes
            .iter()
            .any(|(_, node)| {
                node.label() == Some(EXPRESSION_MISSING) || node.value() == Some(EXPRESSION_MISSING)
            });
    }
    assert!(said, "the line under Expression is blank");
}

/// What the picked measurement does takes one line under it however long it
/// is, and the whole of it is still read out.
#[test]
fn the_description_takes_one_line_and_is_read_out_whole() {
    let ctx = Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut draft = MeasurementDraft {
        expression: "delay(V(in), V(out))".to_owned(),
        ..MeasurementDraft::default()
    };
    reread(&mut draft);
    let description = draft.function.expect("delay is catalogued").description;
    let preview = preview(Ok(None));
    let mut bounds = None;
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..Default::default()
            },
            |ui| {
                show(ui.ctx(), &mut draft, &preview);
            },
        );
        bounds = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes
            .iter()
            .find(|(_, node)| node.value() == Some(description))
            .and_then(|(_, node)| node.bounds());
    }
    let bounds = bounds.expect("the whole description is read out");
    assert!(
        bounds.y1 - bounds.y0 < 20.0,
        "the description takes {} points",
        bounds.y1 - bounds.y0
    );
}

/// Picking a measurement fills its signals from the pane's traces, in
/// order, and writes the call; picking another keeps the signals chosen.
#[test]
fn picking_a_measurement_fills_its_signals_and_writes_the_call() {
    let signals = vec!["V(in)".to_owned(), "V(out)".to_owned()];
    let mut draft = MeasurementDraft::default();
    pick(&mut draft, catalogue::find("delay").unwrap(), &signals);
    assert_eq!(draft.expression, "delay(V(in), V(out))");
    draft.arguments[0] = "V(out)".to_owned();
    draft.arguments[1] = "V(in)".to_owned();
    pick(&mut draft, catalogue::find("slewRate").unwrap(), &signals);
    assert_eq!(draft.expression, "slewRate(V(out))");
    // A measurement with a required number shows no call until the number
    // is given.
    pick(&mut draft, catalogue::find("cross").unwrap(), &signals);
    assert_eq!(draft.expression, "");
    assert_eq!(
        catalogue::find("cross")
            .unwrap()
            .call(&draft.arguments)
            .unwrap_err(),
        "Level is required."
    );
}

/// A level is typed in its own signal's unit: the pane's word for a trace
/// it shows, else the accessor's, else none.
#[test]
fn a_level_takes_the_unit_of_its_signal() {
    let preview = MeasurementPreview {
        signals: vec!["V(in)".to_owned(), "I(R1)".to_owned(), "gain".to_owned()],
        signal_units: vec!["V".to_owned(), "A".to_owned(), "dB".to_owned()],
        ..preview(Ok(None))
    };
    assert_eq!(unit_of("I(R1)", &preview), "A");
    assert_eq!(unit_of(" gain ", &preview), "dB");
    assert_eq!(unit_of("v(typed)", &preview), "V");
    assert_eq!(unit_of("V(in) * 2", &preview), "V");
    assert_eq!(unit_of("frequency(x)", &preview), "");
    assert_eq!(unit_of("", &preview), "");
}

/// A catalogued call typed into the expression fills the fields; anything
/// else leaves the fields, except while the same call is still being typed.
#[test]
fn a_typed_call_fills_the_fields_and_free_text_releases_them() {
    let mut draft = MeasurementDraft {
        expression: "cross(V(out), 2.5, 2, \"Rising\")".to_owned(),
        ..MeasurementDraft::default()
    };
    reread(&mut draft);
    assert_eq!(draft.function.map(|spec| spec.name), Some("cross"));
    assert_eq!(draft.arguments, ["V(out)", "2.5", "2", "rising"]);
    draft.expression = "cross(V(out), 2.5, 2, \"ris".to_owned();
    reread(&mut draft);
    assert_eq!(draft.function.map(|spec| spec.name), Some("cross"));
    draft.expression = "rms(V(out)) / 2".to_owned();
    reread(&mut draft);
    assert_eq!(draft.function, None);
    assert!(draft.arguments.is_empty());
}

/// Drive `show` through eight passes of a 2x display, with a click at the
/// centre of the dropdown labelled `click` on the fourth and fifth, and
/// rasterize the last.
fn render_form(
    theme: rspice_ui_kit::Theme,
    size: egui::Vec2,
    draft: &mut MeasurementDraft,
    preview: &MeasurementPreview,
    click: Option<&str>,
) -> rspice_ui_kit::raster::Canvas {
    let ctx = Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    theme.apply(&ctx);
    ctx.enable_accesskit();
    let mut target = None;
    let mut output = None;
    for frame in 0..8u8 {
        let mut input = rspice_ui_kit::raster::display_input(size, 2.0);
        // A quarter second a pass, so the list's fade-in has finished by the
        // pass that is rasterized.
        input.time = Some(f64::from(frame) * 0.25);
        if let (3 | 4, Some(at)) = (frame, target) {
            input.events = vec![
                egui::Event::PointerMoved(at),
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: frame == 3,
                    modifiers: egui::Modifiers::NONE,
                },
            ];
        }
        let frame_output = ctx.run_ui(input, |ui| {
            show(ui.ctx(), draft, preview);
        });
        // The form has settled where it opens by the third pass.
        if let (2, Some(label), Some(update)) =
            (frame, click, &frame_output.platform_output.accesskit_update)
        {
            target = update
                .nodes
                .iter()
                .find(|(_, node)| {
                    node.label() == Some(label) && node.role() == egui::accesskit::Role::ComboBox
                })
                .and_then(|(_, node)| node.bounds())
                // AccessKit bounds are in points.
                .map(|bounds| {
                    egui::pos2(
                        ((bounds.x0 + bounds.x1) / 2.0) as f32,
                        ((bounds.y0 + bounds.y1) / 2.0) as f32,
                    )
                });
        }
        output = Some(frame_output);
    }
    rspice_ui_kit::raster::rasterize(&ctx, size, &output.expect("a pass ran"))
}

/// Writes the form for review: a delay being set up, and the measurement
/// list opened by a click. Set `RSPICE_REVIEW_DIR` to choose where.
#[test]
#[ignore = "writes PNGs for review"]
fn write_measurement_form_pngs() {
    let directory = std::env::var_os("RSPICE_REVIEW_DIR")
        .map_or_else(std::env::temp_dir, std::path::PathBuf::from);
    let delay = || {
        let mut draft = MeasurementDraft {
            expression: "delay(V(in), V(out), 0.5, \"falling\", 1, 0.5, \"rising\")".to_owned(),
            ..MeasurementDraft::default()
        };
        reread(&mut draft);
        draft
    };
    let filled = preview(Ok(Some("29.2067 ps".to_owned())));
    for (mode, suffix) in [
        (rspice_ui_kit::tokens::Mode::Dark, "dark"),
        (rspice_ui_kit::tokens::Mode::Light, "light"),
    ] {
        let theme = rspice_ui_kit::Theme {
            mode,
            ..rspice_ui_kit::Theme::default()
        };
        for (click, name, height) in [(None, "delay", 700.0), (Some("Measurement"), "list", 900.0)]
        {
            let canvas = render_form(
                theme,
                egui::vec2(900.0, height),
                &mut delay(),
                &filled,
                click,
            );
            let path = directory.join(format!("add-measurement-{name}-{suffix}.png"));
            std::fs::write(&path, canvas.png(canvas.height())).expect("write form render");
            println!("render: {}", path.display());
        }
    }
}
