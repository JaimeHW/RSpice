//! Where this surface puts what it paints.
//!
//! `tests` judges what the studio says and `click_tests` what a press on it
//! does. This is the third question and the one neither can ask: two things
//! laid out by two separate measurements have to agree about the space
//! between them, and only their coordinates answer that.

use egui::{Rect, vec2};

use crate::simulation::plan::AnalysisKind;
use crate::workbench::RSpiceApp;

/// One painted line of the Analyses route: its text and where it landed.
#[derive(Clone, Debug)]
struct PaintedLine {
    text: String,
    left: f32,
    right: f32,
    top: f32,
}

/// Every line of text a rendered frame painted, and where each landed.
fn painted_lines(shapes: &[egui::epaint::ClippedShape]) -> Vec<PaintedLine> {
    fn collect(shape: &egui::epaint::Shape, out: &mut Vec<PaintedLine>) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push(PaintedLine {
                text: text.galley.job.text.clone(),
                left: text.pos.x,
                right: text.pos.x + text.galley.size().x,
                top: text.pos.y,
            }),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, out);
                }
            }
            _ => {}
        }
    }

    let mut lines = Vec::new();
    for clipped in shapes {
        collect(&clipped.shape, &mut lines);
    }
    lines
}

/// Render the Analyses route at `width` and report every line it painted.
///
/// Positions, not just wording. The header's identity line and the
/// availability chip beside it are laid out by two separate measurements —
/// the chip is measured from the trailing inset, the line gets whatever is
/// left — and whether they agree is a question about coordinates that no
/// assertion over text alone can ask.
fn analyses_route_painted_lines(width: f32) -> Vec<PaintedLine> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.simulation_page = crate::workbench::state::SimulationPage::Analyses;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(width, 900.0))),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| super::show(ui, &mut app));
        },
    );
    painted_lines(&output.shapes)
}

/// The analysis header states the kind and lifecycle, never the instance's
/// UUID, and a production analysis says nothing about its availability.
///
/// The line used to read `<kind> · <id> · lifecycle <state>` beside an
/// availability chip that said "Production" in green on every default
/// analysis. The identity is internal and the default is not news, so the line
/// is `<kind> · <state>` and the trailing edge holds only the Name… action,
/// which the line stops short of. These are viewport widths; the surface
/// withholds its scrollbar's thirteen-point track from them.
#[test]
fn the_analysis_header_states_kind_and_lifecycle_short_of_its_action() {
    /// The header band: the title, the identity line and the action are the
    /// only things painted in the top 120 points of the form.
    const HEADER_BAND: f32 = 120.0;

    for width in [1016.0, 860.0] {
        let lines = analyses_route_painted_lines(width);
        let header: Vec<_> = lines
            .iter()
            .filter(|line| line.top < HEADER_BAND)
            .cloned()
            .collect();
        assert!(
            !header.iter().any(|line| line.text == "Production"),
            "a production analysis states no availability at {width}: {header:?}"
        );
        let identity = header
            .iter()
            .find(|line| line.text == "Transient · draft")
            .unwrap_or_else(|| {
                panic!("the header states the kind and lifecycle at {width}: {header:?}")
            });
        let action = header
            .iter()
            .find(|line| line.text == "Name\u{2026}")
            .unwrap_or_else(|| panic!("the header offers Name… at {width}: {header:?}"));
        assert!(
            identity.right < action.left,
            "the identity line runs to {} and the action starts at {}",
            identity.right,
            action.left
        );
    }
}

/// An option field states its value and nothing beside its caption.
///
/// Every option field once carried a tag on its caption row, and every tag
/// read `default` — including the fields holding the plan's own value, where
/// it said nothing the value did not. An empty well the engine resolves now
/// reads `default` inside itself, below its caption, and a well holding a value
/// shows only the value.
#[test]
fn an_option_field_states_default_only_inside_an_empty_well() {
    use crate::simulation::plan::NumericOverrideOption as O;

    let lines = analyses_form_lines(AnalysisKind::Transient, 900.0)
        .expect("the default plan holds a transient");
    let caption = |label: &str| {
        lines
            .iter()
            .find(|line| line.text == label)
            .unwrap_or_else(|| panic!("the transient form draws {label:?}"))
            .clone()
    };
    // The plan states a truncation bound, so its well holds a number; the
    // plan states no LTE bound, so that well is empty.
    let valued = caption(O::Trtol.label());
    let empty = caption(O::LteReltol.label());
    let on_row = |row: &PaintedLine| {
        lines
            .iter()
            .filter(|line| (line.top - row.top).abs() < 0.5)
            .map(|line| line.text.clone())
            .collect::<Vec<_>>()
    };
    for row in [&valued, &empty] {
        let beside = on_row(row);
        assert!(
            beside
                .iter()
                .all(|text| !matches!(text.as_str(), "default" | "override" | "study default")),
            "{:?} carries a tag on its caption row: {beside:?}",
            row.text
        );
    }
    let placeholder = lines.iter().find(|line| {
        line.text == "default"
            && line.top > empty.top
            && line.top < empty.top + 40.0
            && line.left >= empty.left
    });
    assert!(
        placeholder.is_some(),
        "the empty LTE well reads `default` inside itself"
    );
    assert!(
        !lines.iter().any(|line| line.text == "default"
            && line.top > valued.top
            && line.top < valued.top + 40.0
            && (line.left - valued.left).abs() < 20.0),
        "a well holding the plan's value shows only the value"
    );
}

/// The contract block states what varies and spells the directive the way a
/// designer writes it.
///
/// It said `Availability: Production` on every production analysis and
/// `Run-set points: 1 · no global axis is enabled` on every plan without an
/// axis — two defaults restated — and spelled the directive with `f64`'s own
/// digits, `.tran 0.000001 0.001`.
#[test]
fn the_contract_block_states_no_defaults_and_spells_the_directive_in_engineering_notation() {
    let lines = analyses_form_lines(AnalysisKind::Transient, 900.0)
        .expect("the default plan holds a transient");
    for gone in [
        "Availability",
        "Production",
        "Run-set points",
        "1 · no global axis is enabled",
    ] {
        assert!(
            !lines.iter().any(|line| line.text == gone),
            "the contract block still states {gone:?}"
        );
    }
    let statement = lines
        .iter()
        .find(|line| line.text.starts_with(".tran "))
        .unwrap_or_else(|| panic!("the contract block states the directive"));
    assert!(
        !statement.text.contains("0.0"),
        "the directive is spelled with raw digits: {:?}",
        statement.text
    );
}

/// The Analyses route with one analysis of `kind` selected, at `width`.
///
/// The kind is inserted where the default plan does not hold one. `None` where
/// the plan refuses the insert, which is a kind that has no form to measure.
fn analyses_form_lines(kind: AnalysisKind, width: f32) -> Option<Vec<PaintedLine>> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    let plan = app.state.sim_setup.stable_analysis_plan_mut().ok()?;
    let instance = match plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == kind)
    {
        Some(instance) => instance.id(),
        None => plan.insert(kind).ok()?.0,
    };
    app.state.workbench.simulation_page = crate::workbench::state::SimulationPage::Analyses;
    app.state.workbench.active_analysis_instance = Some(instance);
    let mut run = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(width, 2_600.0))),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| super::show(ui, &mut app));
            },
        )
    };
    // The surface resolves its content width against the scrollbar track it
    // reserves, which it only knows on a second pass.
    let _ = run();
    Some(painted_lines(&run().shapes))
}

/// Every word a field may state in the helper slot beside its caption.
///
/// A sample of the rules default forms state there: a temperature names its
/// unit, and a field whose label cannot carry its rule states the rule. Option
/// fields state nothing beside their caption; an empty one says `default`
/// inside its well. Each is asserted to be *found* below, so a slot renamed out
/// from under this cannot leave the gate matching nothing.
const FIELD_HELPERS: [&str; 7] = [
    "Celsius",
    "products up to this order",
    "odd, empty is automatic",
    "quoted multiple",
    "empty keeps all",
    "at most the first step",
    "solver trace to the Console",
];

/// No field caption runs under the helper beside it, on any analysis form.
///
/// A caption and its helper are both painted into one strip — the caption from
/// the left, the helper right-aligned — so nothing in the layout stops the two
/// meeting. Every caption was short until the advanced options arrived carrying
/// `Truncation absolute bound · TIMEINT ABSTOL` into a 206-point cell. The
/// caption elides now, and this is the measurement: 460 points is the narrowest
/// surface that still lays the fields out in two columns, so its cells are the
/// narrowest any caption is ever given.
///
/// Judged on the strips a helper is painted in rather than on every baseline
/// the route paints. A well legitimately holds a galley wider than itself and
/// clips it — the noise form's frequency list is one — so "nothing overlaps
/// anywhere" is not true of the surface, while "nothing overlaps a helper" is
/// the claim this is about, and it covers the two cells of one grid row too.
#[test]
fn no_field_caption_overlaps_the_helper_beside_it() {
    /// The narrowest surface whose fields are still two columns wide.
    const NARROWEST_TWO_COLUMN_SURFACE: f32 = 460.0;
    /// Sub-pixel: text resting exactly against its neighbour is not a collision.
    const TOLERANCE: f32 = 0.5;

    let mut collisions = Vec::new();
    let mut found = std::collections::BTreeSet::new();
    let mut forms = 0usize;
    for kind in AnalysisKind::ALL {
        let Some(lines) = analyses_form_lines(kind, NARROWEST_TWO_COLUMN_SURFACE) else {
            continue;
        };
        forms += 1;
        let mut rows: Vec<Vec<PaintedLine>> = Vec::new();
        let mut sorted = lines;
        sorted.sort_by(|a, b| a.top.total_cmp(&b.top).then(a.left.total_cmp(&b.left)));
        for line in sorted {
            match rows.last_mut() {
                Some(row) if (row[0].top - line.top).abs() < TOLERANCE => row.push(line),
                _ => rows.push(vec![line]),
            }
        }
        for row in &rows {
            for (index, helper) in row.iter().enumerate() {
                if !FIELD_HELPERS.contains(&helper.text.as_str()) {
                    continue;
                }
                found.insert(helper.text.clone());
                for (other, line) in row.iter().enumerate() {
                    if other == index
                        || line.right <= helper.left + TOLERANCE
                        || line.left >= helper.right - TOLERANCE
                    {
                        continue;
                    }
                    collisions.push(format!(
                        "{kind:?}: {:?} spans {:.1}..{:.1} over the helper {:?} at {:.1}..{:.1}",
                        line.text, line.left, line.right, helper.text, helper.left, helper.right
                    ));
                }
            }
        }
    }
    collisions.sort();
    collisions.dedup();
    assert!(
        collisions.is_empty(),
        "text painted over a field's helper:\n{}",
        collisions.join("\n")
    );
    // A sweep that opened no form, or found no helper on the ones it opened,
    // would pass forever.
    let kinds = AnalysisKind::ALL.len();
    let missing: Vec<_> = FIELD_HELPERS
        .iter()
        .filter(|helper| !found.contains(**helper))
        .collect();
    assert!(
        forms >= kinds - 4 && missing.is_empty(),
        "the sweep opened {forms} of {kinds} forms and never found the helpers {missing:?}"
    );
}

/// What the analysis catalogue drew, gathered into the rows it drew them in.
///
/// Every analysis code the frame painted, grouped by the top it landed on:
/// two cells of one grid row share a row rect, so they share a top, and two
/// cells of one column never do. That is the whole of what "two columns"
/// means here, and it is a question only the coordinates answer — the same
/// rows, the same wording and the same reading order come out of a
/// one-column layout.
fn analysis_catalogue_code_rows(viewport_width: f32) -> Vec<Vec<PaintedLine>> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.sim_setup.session.palette_open = true;
    let mut run = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(viewport_width, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| super::show_workflow_dialogs(ctx, &mut app),
        )
    };
    // A content-height surface lays out against its previous measurement.
    let _ = run();
    let output = run();
    let codes = AnalysisKind::MANIFEST_ORDER.map(AnalysisKind::code);
    let mut codes_painted = painted_lines(&output.shapes)
        .into_iter()
        .filter(|line| codes.contains(&line.text.as_str()))
        .collect::<Vec<_>>();
    codes_painted.sort_by(|a, b| a.top.total_cmp(&b.top).then(a.left.total_cmp(&b.left)));
    let mut rows: Vec<Vec<PaintedLine>> = Vec::new();
    for line in codes_painted {
        match rows.last_mut() {
            Some(row) if (row[0].top - line.top).abs() < 0.5 => row.push(line),
            _ => rows.push(vec![line]),
        }
    }
    rows
}

/// The catalogue's second column is decided by the body it is drawn in.
///
/// `DialogSize::AnalysisCatalog` is a 1180-point surface inset from the
/// viewport, so the body is 26 points narrower than the window and the rows
/// give another 13 to a solid scrollbar. A rule written against the viewport
/// is wrong by that much, and at 1199 it was: a single column of 57-point
/// rows drawn down a body 1173 wide, with room for a second column standing
/// empty beside them.
///
/// Both directions are pinned, because "wider windows get two columns" is not
/// the claim. The claim is that the body's own width decides. With the
/// current chrome the layout turns over at a 1160-point window; 1199 is above
/// it and carries two columns that each clear
/// [`ANALYSIS_CATALOG_COLUMN_MIN_WIDTH`], and 1100 is below it and carries
/// one.
#[test]
fn the_analysis_catalogue_takes_its_columns_from_its_body_not_the_viewport() {
    let wide = analysis_catalogue_code_rows(1_199.0);
    assert!(
        wide.len() > 8,
        "the catalogue drew its rows at 1199: {} row(s)",
        wide.len()
    );
    assert!(
        wide.iter().all(|row| row.len() <= 2),
        "no row holds more cells than the layout has columns: {wide:?}"
    );
    let pair = wide
        .iter()
        .find(|row| row.len() == 2)
        .unwrap_or_else(|| panic!("an 1199-point viewport pairs its rows: {wide:?}"));
    let column_width = pair[1].left - pair[0].left;
    assert!(
        column_width > super::ANALYSIS_CATALOG_COLUMN_MIN_WIDTH,
        "each of the two columns clears the row's own minimum: {column_width}"
    );

    let narrow = analysis_catalogue_code_rows(1_100.0);
    assert!(
        narrow.len() > 8,
        "the catalogue drew its rows at 1100: {} row(s)",
        narrow.len()
    );
    assert!(
        narrow.iter().all(|row| row.len() == 1),
        "a body that cannot carry two columns of that minimum draws one: {narrow:?}"
    );
}

/// The window the split-route scrolling cases are drawn in.
const ROUTE_WIDTH: f32 = 1_440.0;

/// The stack's first group header, as painted: the stack's topmost line.
const FIRST_GROUP: &str = "Core analyses";

/// The Analyses route held open in a 1440-point window across many passes, so
/// a wheel's eased scroll can run its course between two measurements.
struct AnalysesRoute {
    ctx: egui::Context,
    app: RSpiceApp,
    height: f32,
}

impl AnalysesRoute {
    fn open(mut app: RSpiceApp, height: f32) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        app.state.workbench.simulation_page = crate::workbench::state::SimulationPage::Analyses;
        let mut route = Self { ctx, app, height };
        route.settle(Vec::new());
        route
    }

    fn pass(&mut self, events: Vec<egui::Event>) -> Vec<PaintedLine> {
        let app = &mut self.app;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(ROUTE_WIDTH, self.height),
                )),
                events,
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| super::show(ui, app));
            },
        );
        painted_lines(&output.shapes)
    }

    /// `events`, then passes until anything they set moving has stopped.
    fn settle(&mut self, events: Vec<egui::Event>) -> Vec<PaintedLine> {
        let mut lines = self.pass(events);
        for _ in 0..60 {
            lines = self.pass(Vec::new());
        }
        lines
    }

    /// Turn the wheel over `at`, let the scroll run its course, then take the
    /// pointer away, so a tooltip the resting pointer raised is not measured
    /// as a line the scroll moved.
    fn scroll_at(&mut self, at: egui::Pos2) -> Vec<PaintedLine> {
        self.settle(wheel_over(at));
        self.settle(vec![egui::Event::PointerGone])
    }

    /// Where the stack ends and the editor begins.
    fn divider(&self) -> f32 {
        let gutter = self.ctx.global_style().spacing.scroll.allocated_width();
        let surface = ROUTE_WIDTH - gutter;
        super::analysis_split_widths(surface, surface).0
    }
}

/// A plan holding one instance of every kind it accepts, which is far more
/// than a 900-point window shows, with its transient analysis selected: the
/// longest form, and one that runs past the window too.
fn crowded_plan() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("the test plan resolves");
    for kind in AnalysisKind::MANIFEST_ORDER {
        if plan
            .instances()
            .iter()
            .all(|instance| instance.kind() != kind)
        {
            let _ = plan.insert(kind);
        }
    }
    select_transient(&mut app.state);
    app
}

fn select_transient(state: &mut crate::workbench::AppState) {
    let plan = state
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("the test plan resolves");
    let transient = match plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Transient)
    {
        Some(instance) => instance.id(),
        None => plan.insert(AnalysisKind::Transient).expect("transient").0,
    };
    state.workbench.active_analysis_instance = Some(transient);
}

/// A wheel turned over `at`, asking to move the content up by 400 points.
fn wheel_over(at: egui::Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(at),
        egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: vec2(0.0, -400.0),
            phase: egui::TouchPhase::Move,
            modifiers: egui::Modifiers::NONE,
        },
    ]
}

/// One column's lines below the title row, as (text, top in tenths of a point).
fn column(
    lines: &[PaintedLine],
    below: f32,
    in_column: impl Fn(&PaintedLine) -> bool,
) -> Vec<(String, i64)> {
    lines
        .iter()
        .filter(|line| line.top >= below && in_column(line))
        .map(|line| (line.text.clone(), (line.top * 10.0).round() as i64))
        .collect()
}

fn top_of(lines: &[PaintedLine], text: &str) -> f32 {
    lines
        .iter()
        .find(|line| line.text == text)
        .unwrap_or_else(|| panic!("{text:?} is painted"))
        .top
}

/// Whether a line both measurements hold rose by more than 40 points.
fn rose(before: &[(String, i64)], after: &[(String, i64)]) -> bool {
    before.iter().any(|(text, top)| {
        after
            .iter()
            .find(|(other, _)| other == text)
            .is_some_and(|(_, moved)| *moved < top - 400)
    })
}

/// Fail, naming only the lines that differ, unless `after` repeats `before`.
#[track_caller]
fn assert_unmoved(before: &[(String, i64)], after: &[(String, i64)], what: &str) {
    let gone = before
        .iter()
        .filter(|line| !after.contains(line))
        .collect::<Vec<_>>();
    let new = after
        .iter()
        .filter(|line| !before.contains(line))
        .collect::<Vec<_>>();
    assert!(
        gone.is_empty() && new.is_empty(),
        "{what}: no longer painted {gone:?}; newly painted {new:?}"
    );
}

/// Each column scrolls its own content and nothing else.
///
/// The stack and the editor shared the surface's one scroll, title row
/// included, so turning the wheel over a long form carried the stack and the
/// title off the screen with it. A plan too long for the window and a form too
/// long for it are each turned through on their own here, and the column not
/// under the wheel, and the title over both, have to stay exactly where they
/// were.
#[test]
fn the_stack_and_the_editor_scroll_on_their_own_under_a_title_that_stays() {
    let mut route = AnalysesRoute::open(crowded_plan(), 900.0);
    let divider = route.divider();
    let first = route.settle(Vec::new());
    // The title row's heading is the plan's name.
    let title = route
        .app
        .state
        .sim_setup
        .active_plan_name()
        .as_str()
        .to_owned();
    let title_top = top_of(&first, &title);
    let body_top = top_of(&first, FIRST_GROUP);
    let in_stack = |line: &PaintedLine| line.right <= divider;
    let in_editor = |line: &PaintedLine| line.left > divider;
    let stack = column(&first, body_top, in_stack);
    let editor = column(&first, body_top, in_editor);

    let after_editor = route.scroll_at(egui::pos2(divider + 300.0, 600.0));
    assert_eq!(
        top_of(&after_editor, &title),
        title_top,
        "the title row stays put"
    );
    assert_unmoved(
        &stack,
        &column(&after_editor, body_top, in_stack),
        "turning the wheel over the editor leaves the stack where it was",
    );
    let editor_scrolled = column(&after_editor, body_top, in_editor);
    assert!(rose(&editor, &editor_scrolled), "the editor scrolled");

    let after_stack = route.scroll_at(egui::pos2(divider * 0.5, 600.0));
    assert_eq!(
        top_of(&after_stack, &title),
        title_top,
        "the title row stays put"
    );
    assert_unmoved(
        &editor_scrolled,
        &column(&after_stack, body_top, in_editor),
        "turning the wheel over the stack leaves the editor where it was",
    );
    assert!(
        rose(&stack, &column(&after_stack, body_top, in_stack)),
        "the stack scrolled"
    );
}

/// A column with nothing past the bottom of the window does not scroll.
///
/// A short plan beside a form that runs long: the wheel over the stack used to
/// scroll the whole surface, because the form made it taller than the window,
/// and the stack went with it. And with the window tall enough for the form
/// too, the wheel over either column moves nothing at all.
#[test]
fn a_column_that_fits_does_not_scroll() {
    let mut app = RSpiceApp::test_instance();
    select_transient(&mut app.state);
    let mut route = AnalysesRoute::open(app, 900.0);
    let divider = route.divider();
    let first = route.settle(Vec::new());
    let body_top = top_of(&first, FIRST_GROUP);
    let everything = |_: &PaintedLine| true;
    let before = column(&first, 0.0, everything);
    let after = route.scroll_at(egui::pos2(divider * 0.5, 600.0));
    assert_unmoved(
        &before,
        &column(&after, 0.0, everything),
        "a short stack does not scroll, and takes nothing else with it",
    );
    assert_eq!(top_of(&after, FIRST_GROUP), body_top);

    let mut app = RSpiceApp::test_instance();
    select_transient(&mut app.state);
    let mut tall = AnalysesRoute::open(app, 3_000.0);
    let first = tall.settle(Vec::new());
    let before = column(&first, 0.0, everything);
    for at in [
        egui::pos2(divider * 0.5, 600.0),
        egui::pos2(divider + 300.0, 600.0),
    ] {
        let after = tall.scroll_at(at);
        assert_unmoved(
            &before,
            &column(&after, 0.0, everything),
            "nothing moves where everything fits",
        );
    }
}

/// An analysis made active from outside the stack is scrolled into its view.
///
/// The stack scrolls on its own now, so the analysis the catalogue inserts and
/// selects can land below what the stack shows. The catalogue's own two steps
/// are taken here — insert, then select — with the stack at its top and the
/// new row at its far end.
#[test]
fn an_analysis_made_active_below_the_stacks_view_is_scrolled_into_it() {
    let mut route = AnalysesRoute::open(crowded_plan(), 900.0);
    let first = route.settle(Vec::new());
    let top = top_of(&first, FIRST_GROUP);
    // The lowest group the catalogue can add to: it refuses a kind that
    // cannot run yet.
    let last_kind = {
        let plan = route
            .app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("the test plan resolves");
        let insertable = plan
            .instances()
            .iter()
            .map(|instance| instance.kind())
            .filter(|kind| kind.execution_blocker().is_none())
            .collect::<Vec<_>>();
        super::ANALYSIS_CATEGORY_ORDER
            .into_iter()
            .rev()
            .find_map(|group| {
                insertable
                    .iter()
                    .copied()
                    .find(|kind| super::analysis_catalog_group(*kind) == group)
            })
            .expect("the plan holds a runnable analysis")
    };
    let inserted = super::insert_analysis_instance(&mut route.app, last_kind)
        .expect("a second instance of the kind inserts");
    route.app.state.workbench.active_analysis_instance = Some(inserted);

    let after = route.settle(Vec::new());
    let moved = after
        .iter()
        .find(|line| line.text == FIRST_GROUP)
        .map(|line| line.top);
    assert!(
        moved.is_none_or(|moved| moved < top - 40.0),
        "the stack scrolled down to the new row: its first group stayed at {moved:?}"
    );
}

/// Renders the split Analyses route in a 1440 x 900 window for review: the
/// default plan, whose short stack draws no scroll bar beside a form that
/// runs long, and a crowded plan whose lowest analysis is active, so the
/// stack has scrolled to it.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_split_analyses_route_for_review() {
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");

    let mut short = RSpiceApp::test_instance();
    select_transient(&mut short.state);
    let mut crowded = crowded_plan();
    let lowest = crowded
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            super::ANALYSIS_CATEGORY_ORDER
                .into_iter()
                .rev()
                .find_map(|group| {
                    plan.instances()
                        .iter()
                        .find(|instance| super::analysis_catalog_group(instance.kind()) == group)
                        .map(|instance| instance.id())
                })
        });
    crowded.state.workbench.active_analysis_instance = lowest;

    for (stem, mut app) in [
        ("studio-analyses-short", short),
        ("studio-analyses-crowded", crowded),
    ] {
        app.state.workbench.simulation_page = crate::workbench::state::SimulationPage::Analyses;
        let canvas = crate::ui::raster::render(vec2(ROUTE_WIDTH, 900.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| super::show(ui, &mut app));
        });
        std::fs::write(directory.join(format!("{stem}.png")), canvas.png(900))
            .expect("write the render");
    }
}
