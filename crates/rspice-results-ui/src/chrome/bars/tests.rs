use super::*;

#[derive(Default)]
struct FilterHost {
    filter: String,
}

impl SheetBarHost for FilterHost {
    fn viewer(&self) -> ResultViewer {
        ResultViewer::Op
    }
    fn export(&mut self, _: ExportRequests) {}
    fn fft_summary(&self) -> Option<String> {
        None
    }
    fn operating_point_filter(&mut self) -> &mut String {
        &mut self.filter
    }
    fn specification_editing(&self) -> bool {
        false
    }
    fn request(&mut self, _: SpecificationAction) {}
    fn instrument(&mut self, _: &mut Ui) {}
    fn eye_actions(&mut self, _: &mut Ui) {}
    fn table_actions(&mut self, _: &mut Ui) {}
    fn domain_controls(&mut self, _: &mut Ui) -> bool {
        false
    }
    fn purpose(&self) -> String {
        String::new()
    }
}

fn strip() -> egui::Context {
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    ctx
}

fn frame(ctx: &egui::Context, host: &mut FilterHost, events: Vec<egui::Event>) {
    let mut input = rspice_ui_kit::raster::display_input(egui::vec2(900.0, 40.0), 1.0);
    input.events = events;
    let _ = ctx.run_ui(input, |ui| show_structured_result_strip(ui, host));
}

fn settle(ctx: &egui::Context, host: &mut FilterHost) {
    for _ in 0..3 {
        frame(ctx, host, Vec::new());
    }
}

fn field_rect(ctx: &egui::Context) -> egui::Rect {
    ctx.read_response(operating_point_filter_id())
        .expect("the strip drew the filter field")
        .rect
}

#[test]
fn the_filter_field_stays_where_it_is_when_the_reader_starts_typing() {
    let ctx = strip();
    let mut host = FilterHost::default();
    settle(&ctx, &mut host);
    let empty = field_rect(&ctx);

    host.filter.push('m');
    frame(&ctx, &mut host, Vec::new());
    assert_eq!(field_rect(&ctx), empty, "on the keystroke's own frame");
    settle(&ctx, &mut host);
    assert_eq!(field_rect(&ctx), empty);
}

#[test]
fn escape_empties_the_filter_the_reader_is_in() {
    let ctx = strip();
    let mut host = FilterHost {
        filter: "linear".to_owned(),
    };
    settle(&ctx, &mut host);
    ctx.memory_mut(|memory| memory.request_focus(operating_point_filter_id()));
    settle(&ctx, &mut host);

    frame(
        &ctx,
        &mut host,
        vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    assert_eq!(host.filter, "");
}

#[test]
fn the_clear_mark_empties_the_filter_and_leaves_the_reader_in_it() {
    let ctx = strip();
    let mut host = FilterHost {
        filter: "linear".to_owned(),
    };
    settle(&ctx, &mut host);
    let field = field_rect(&ctx);
    let at = egui::pos2(field.right() - 15.0, field.center().y);
    let pointer = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };

    frame(&ctx, &mut host, vec![egui::Event::PointerMoved(at)]);
    frame(&ctx, &mut host, vec![pointer(true)]);
    frame(&ctx, &mut host, vec![pointer(false)]);
    settle(&ctx, &mut host);

    assert_eq!(host.filter, "");
    assert!(ctx.memory(|memory| memory.has_focus(operating_point_filter_id())));
    assert_eq!(field_rect(&ctx), field);
}
