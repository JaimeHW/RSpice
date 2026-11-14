//! The netlist-first landing drawn through a whole application: what it says
//! about the deck that owns the project and what its two actions run.

use crate::workbench::RSpiceApp;
use crate::workbench::code::CodeFeature;
use crate::workbench::code::netlist_first::*;
use crate::workbench::documents::netlist_document::root_deck_source_index;
use crate::workbench::features::{self, Registered};
use crate::workbench::state::Workspace;
use egui::{Rect, Ui, Vec2, pos2, vec2};

const DECK: &str = "Precision front-end bench\n\
    * differential input stage\n\
    V1 in 0 AC 1\n\
    R1 in inn 4.7k\n\
    C1 inn 0 220p\n\
    M1 out inn 0 0 NFET W=10u L=1u\n\
    .model NFET NMOS (LEVEL=1 VTO=0.7)\n\
    .subckt buffer a y\n\
    E1 y 0 a 0 1\n\
    .ends\n\
    .tran 1n 1u\n\
    .op\n\
    .end\n";

const SUPPORTED_VIEWPORTS: [(&str, Vec2); 3] = [
    ("1440x900", Vec2::new(1440.0, 900.0)),
    ("820x1180", Vec2::new(820.0, 1180.0)),
    ("390x844", Vec2::new(390.0, 844.0)),
];
#[derive(Clone, Copy)]
struct NetlistFirstBaseline {
    name: &'static str,
    size: Vec2,
    truncated: bool,
    fingerprint: &'static str,
}

const NETLIST_FIRST_BASELINES: [NetlistFirstBaseline; 4] = [
    NetlistFirstBaseline {
        name: "design-netlist-first-1440x900",
        size: Vec2::new(1440.0, 900.0),
        truncated: false,
        fingerprint: "0fe385bbc9e9e9184f5dcd7a1a423c8982a6d685ac699052a0705279088c0bae",
    },
    NetlistFirstBaseline {
        name: "design-netlist-first-820x1180",
        size: Vec2::new(820.0, 1180.0),
        truncated: false,
        fingerprint: "3896b344cd981b7bbaece1211152139c8cba4e47b26bc799e99f49926e97191e",
    },
    NetlistFirstBaseline {
        name: "design-netlist-first-390x844",
        size: Vec2::new(390.0, 844.0),
        truncated: false,
        fingerprint: "4b4438a47ded9018f41f5ea44b1567f16cc295544bd762628f7d3430bada79ab",
    },
    NetlistFirstBaseline {
        name: "design-netlist-first-1440x900-truncated",
        size: Vec2::new(1440.0, 900.0),
        truncated: true,
        fingerprint: "5e51b693fe0cbee83fb9c359f009e909f5980aa486653135d308e5ecba7314af",
    },
];

pub(super) fn netlist_first_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    assert!(
        app.state
            .apply_imported_netlist(DECK.to_owned(), None, "front_end.sp")
    );
    assert!(app.state.is_netlist_first_without_schematic());
    app
}

#[test]
fn netlist_first_visual_baselines_cover_supported_viewports_and_truncation_and_are_unique() {
    assert_eq!(NETLIST_FIRST_BASELINES.len(), SUPPORTED_VIEWPORTS.len() + 1);
    assert_eq!(
        NETLIST_FIRST_BASELINES[..SUPPORTED_VIEWPORTS.len()]
            .iter()
            .map(|baseline| baseline.size)
            .collect::<Vec<_>>(),
        SUPPORTED_VIEWPORTS.map(|(_, size)| size),
        "the baseline table must cover every supported netlist-first viewport"
    );
    assert!(
        NETLIST_FIRST_BASELINES[..SUPPORTED_VIEWPORTS.len()]
            .iter()
            .all(|baseline| !baseline.truncated)
    );
    let truncated = NETLIST_FIRST_BASELINES
        .last()
        .expect("truncated netlist-first baseline");
    assert!(truncated.truncated);
    assert_eq!(truncated.size, SUPPORTED_VIEWPORTS[0].1);
    let names = NETLIST_FIRST_BASELINES
        .iter()
        .map(|baseline| baseline.name)
        .collect::<std::collections::HashSet<_>>();
    let fingerprints = NETLIST_FIRST_BASELINES
        .iter()
        .map(|baseline| baseline.fingerprint)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(names.len(), NETLIST_FIRST_BASELINES.len());
    assert_eq!(
        fingerprints.len(),
        NETLIST_FIRST_BASELINES.len(),
        "every supported viewport/state must have an independently reviewed fingerprint"
    );
}

/// One pass of the landing the code feature registers, drawn over `app` in
/// the context the shell lends it.
fn landing(app: &mut RSpiceApp, ui: &mut Ui) {
    let surface = CodeFeature::registry()
        .surface_section()
        .expect("the code feature registers the netlist-first landing");
    (surface.show)(&mut features::surface_cx::<CodeFeature>(app, |_| false), ui);
}

fn truncated_netlist_first_app() -> RSpiceApp {
    let mut deck = String::from("Long resistor ladder bench\n");
    for element in 0..400 {
        deck.push_str(&format!("R{element} n{element} 0 1k\n"));
    }
    deck.push_str(".op\n.end\n");
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.apply_imported_netlist(deck, None, "ladder.sp"));
    assert!(app.state.is_netlist_first_without_schematic());
    app
}

fn netlist_first_canvas(size: Vec2, truncated: bool) -> crate::ui::raster::Canvas {
    let mut app = if truncated {
        truncated_netlist_first_app()
    } else {
        netlist_first_app()
    };
    crate::ui::raster::render(size, |ui, _| landing(&mut app, ui))
}

fn regression_height(canvas: &crate::ui::raster::Canvas) -> usize {
    canvas.content_height().max(1)
}

#[test]
fn supported_netlist_first_landings_match_their_reviewed_visual_baselines() {
    for baseline in NETLIST_FIRST_BASELINES {
        let canvas = netlist_first_canvas(baseline.size, baseline.truncated);
        canvas.assert_regression(
            baseline.name,
            regression_height(&canvas),
            baseline.fingerprint,
        );
    }
}

#[test]
fn netlist_first_empty_state_actions_use_the_canonical_commands() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();

    raise_netlist_first_action(
        &mut app.state.workbench,
        NetlistFirstAction::OpenNetlistWorkspace,
    );
    app.after_contribution();
    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);

    raise_netlist_first_action(
        &mut app.state.workbench,
        NetlistFirstAction::CreateSchematic,
    );
    app.after_contribution();
    assert!(app.state.project.new_cell_dialog);
    assert!(app.state.project.new_cell_create_schematic);
}

/// Every string the page paints in one pass, so wording is asserted on the
/// galleys rather than read off pixels.
fn painted_text(size: Vec2, mut pass: impl FnMut(&mut Ui)) -> Vec<String> {
    fn walk(shape: &egui::epaint::Shape, out: &mut Vec<String>) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push(text.galley.text().to_owned()),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, out);
                }
            }
            _ => {}
        }
    }

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut shapes = Vec::new();
    for _ in 0..2 {
        shapes = ctx
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| pass(ui));
                },
            )
            .shapes;
    }
    let mut out = Vec::new();
    for clipped in &shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

#[test]
fn the_page_names_the_deck_its_actions_and_its_facts() {
    let mut app = netlist_first_app();
    let text = painted_text(vec2(1440.0, 900.0), |ui| landing(&mut app, ui)).join("\n");
    for expected in [
        EYEBROW,
        "Open Netlist & Script Editor",
        "Create schematic\u{2026}",
        "Promote to a schematic-driven project",
        "Deck source",
        "front_end",
        "Deck summary",
        "Devices",
        "Analyses",
        "Precision front-end bench",
    ] {
        assert!(text.contains(expected), "missing {expected:?} in:\n{text}");
    }
}

#[test]
fn a_restored_session_shows_its_deck_before_the_editor_has_been_opened() {
    let saved = netlist_first_app();
    let session = serde_json::to_string(&saved.state).expect("session serializes");
    let mut app = RSpiceApp::test_instance();
    app.state = serde_json::from_str(&session).expect("session restores");
    assert!(app.state.is_netlist_first_without_schematic());

    let text = painted_text(vec2(1440.0, 900.0), |ui| landing(&mut app, ui)).join("\n");
    assert!(!text.contains("The deck has no source text."), "{text}");
    assert!(
        text.contains("R1 in inn 4.7k"),
        "deck lines missing in:\n{text}"
    );
    let index = root_deck_source_index(&app.state.model, &mut app.state.code.netlist);
    assert_eq!(
        deck_facts(&app.state.model, &app.state.code.netlist, &index).lines,
        13
    );
}

#[test]
fn the_landing_shows_the_deck_while_the_editor_shows_other_text() {
    let mut app = netlist_first_app();
    // An include file open in the editor, or nothing projected into it yet.
    for editor in ["* models.inc\n.model NFET NMOS (LEVEL=1)\n", ""] {
        editor.clone_into(&mut app.state.model.simulation.source.netlist_content);
        let text = painted_text(vec2(1440.0, 900.0), |ui| landing(&mut app, ui)).join("\n");
        assert!(text.contains("Precision front-end bench"), "{text}");
        assert!(!text.contains("* models.inc"), "{text}");
    }
}

#[test]
fn a_long_deck_truncates_behind_a_footer_that_counts_what_was_left_out() {
    let mut app = truncated_netlist_first_app();

    let text = painted_text(vec2(1440.0, 900.0), |ui| landing(&mut app, ui)).join("\n");
    assert!(
        text.contains("more lines"),
        "no truncation footer in:\n{text}"
    );
    assert!(
        !text.contains("R399"),
        "the deck's tail painted past the viewport"
    );
}

#[test]
fn the_page_paints_the_full_landing_at_all_supported_viewports() {
    for ((_, size), minimum_content_height) in SUPPORTED_VIEWPORTS.into_iter().zip([470, 760, 740])
    {
        let canvas = netlist_first_canvas(size, false);
        let content_height = canvas.content_height();
        assert!(
            content_height >= minimum_content_height,
            "{}x{}: content stops at row {content_height}",
            size.x,
            size.y
        );
    }
}

/// On the two-column layout the deck panel and the rail both carry ink in
/// their own columns — the failure mode this catches is the old page,
/// whose body was one empty band.
#[test]
fn the_two_column_landing_fills_both_columns() {
    let mut app = netlist_first_app();
    let size = vec2(1440.0, 900.0);
    let canvas = crate::ui::raster::render(size, |ui, _| landing(&mut app, ui));

    // Content column: 1240 centered in 1440 leaves 100 on each side. The
    // preview occupies the left column and the rail the last 322 points.
    let preview_probe = Rect::from_min_size(pos2(160.0, 300.0), vec2(300.0, 200.0));
    let rail_probe = Rect::from_min_size(pos2(1040.0, 160.0), vec2(240.0, 60.0));
    for (label, probe) in [("deck preview", preview_probe), ("action rail", rail_probe)] {
        assert!(
            canvas
                .pixels_in(probe)
                .any(|pixel| pixel != canvas.background()),
            "{label} painted nothing"
        );
    }
}
