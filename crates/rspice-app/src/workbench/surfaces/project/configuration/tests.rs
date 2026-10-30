//! The Configuration page as a reader meets it: what it states on Default
//! and on a configured hierarchy, the words it never uses, and how often it
//! resolves the design.

use super::*;

use crate::workbench::examples::hierarchy_reference;
use crate::workbench::project::configuration_state::ConfigurationSelection;

/// Words that name the implementation rather than anything an engineer
/// acts on. None may reach the page.
const BANNED: &[&str] = &[
    "contract",
    "digest",
    "semantic",
    "execution",
    "materialized",
    "projection",
    "precedence",
    "revision",
    "owner",
    "black-box",
    "model profile",
    "catalog",
    "authority",
    "receipt",
];

fn reference_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state = hierarchy_reference::build().state;
    app
}

/// One frame of the page: every accessible name and value, and every string
/// it painted with the colour it was painted in.
struct Frame {
    announced: Vec<String>,
    painted: Vec<(String, Color32)>,
}

fn frame(app: &mut RSpiceApp, width: f32) -> Frame {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width, 900.0),
            )),
            ..Default::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| configuration(ui, app));
        },
    );
    let announced = output
        .platform_output
        .accesskit_update
        .expect("an accessibility update")
        .nodes
        .iter()
        .flat_map(|(_, node)| {
            [node.label(), node.value(), node.description()]
                .into_iter()
                .flatten()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .collect();
    fn collect(shape: &egui::epaint::Shape, out: &mut Vec<(String, Color32)>) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push((
                text.galley.job.text.clone(),
                text.galley
                    .job
                    .sections
                    .first()
                    .map_or(text.fallback_color, |section| section.format.color),
            )),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, out);
                }
            }
            _ => {}
        }
    }
    let mut painted = Vec::new();
    for shape in &output.shapes {
        collect(&shape.shape, &mut painted);
    }
    Frame { announced, painted }
}

impl Frame {
    fn says(&self, text: &str) -> bool {
        self.announced.iter().any(|announced| announced == text)
            || self.painted.iter().any(|(painted, _)| painted == text)
    }

    fn every_word(&self) -> impl Iterator<Item = &str> {
        self.announced
            .iter()
            .map(String::as_str)
            .chain(self.painted.iter().map(|(text, _)| text.as_str()))
    }
}

#[test]
fn the_default_states_the_built_in_lists() {
    let mut app = RSpiceApp::test_instance();
    let project = &app.state.model.workspace.content.project;
    let top = format!("{}/{}/schematic", project.root_library, project.top_cell);
    let shown = frame(&mut app, 1440.0);
    for fact in [
        "Default",
        "Used for runs",
        "New configuration\u{2026}",
        top.as_str(),
        "\u{2014}",
        "the instance's own view, then extracted, then spice",
        "Stop list 1: spice",
        "Stop list 2: verilog",
        "Stop list 3: veriloga",
        "Stop list 4: extracted",
        "Stop the netlist",
    ] {
        assert!(shown.says(fact), "Default must state {fact:?}");
    }
    assert!(
        !shown.says("Apply") && !shown.says("Revert"),
        "Default is read-only"
    );
}

#[test]
fn no_word_on_the_page_names_the_implementation() {
    let mut default = RSpiceApp::test_instance();
    let mut configured = reference_app();
    for (page, shown) in [
        ("Default", frame(&mut default, 1440.0)),
        ("configured", frame(&mut configured, 1440.0)),
        ("configured, narrow", frame(&mut configured, 640.0)),
    ] {
        for word in shown.every_word() {
            let lower = word.to_ascii_lowercase();
            for banned in BANNED {
                assert!(!lower.contains(banned), "{page}: {banned:?} in {word:?}");
            }
            assert!(
                !lower
                    .split(|c: char| !c.is_ascii_alphanumeric())
                    .any(|token| token.len() > 1
                        && token.starts_with('r')
                        && token[1..].bytes().all(|byte| byte.is_ascii_digit())),
                "{page}: a revision number in {word:?}"
            );
        }
    }
}

/// The band counts the circuit the variant populates, not every instance
/// the schematic draws.
#[test]
fn the_band_counts_only_the_populated_variant() {
    let mut app = reference_app();
    hierarchy_reference::omit_missing_instance(&mut app.state);
    let expected = app
        .state
        .model
        .workspace
        .design_projection(
            &app.state.model.library_manager,
            &app.state.model.workspace.content.active_view,
            &app.state.model.schematic,
        )
        .unwrap()
        .hierarchy_resolution()
        .clone();
    let authored = app.state.model.workspace.resolve_hierarchy_with_active(
        &app.state.model.library_manager,
        &app.state.model.workspace.content.active_view,
        &app.state.model.schematic,
    );
    assert_ne!(authored.total_instances, expected.total_instances);
    assert!(expected.is_valid());
    let total = expected.total_instances;
    for width in [640.0, 1440.0] {
        let shown = frame(&mut app, width);
        assert!(
            shown.says(&format!("{total} of {total} instances bound")),
            "the {width}px band must count {total} instances"
        );
    }
}

/// An unbound status carries the kit's mark; its words keep the band's
/// neutral tone.
#[test]
fn unbound_instances_are_marked_rather_than_painted_red() {
    let mut app = reference_app();
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let id = catalog.active_configuration_id().unwrap();
    app.state
        .project
        .configuration
        .edit(&catalog, id, |definition| {
            definition.executable_view_policy = vec!["spice".to_owned()];
            definition.overrides.clear();
        });
    let shown = frame(&mut app, 1440.0);
    let status = shown
        .announced
        .iter()
        .find(|text| text.contains("instances unbound"))
        .expect("the band states the unbound instances");
    assert!(
        status.starts_with("Error: "),
        "the status carries the mark: {status}"
    );
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let tokens = Tokens::get(&ctx);
    for (text, color) in &shown.painted {
        assert!(
            *color != tokens.color.warn && *color != tokens.color.err && *color != tokens.color.ok,
            "{text:?} is painted in a severity tone"
        );
    }
}

/// The candidate hierarchy is resolved when what it depends on changes, and
/// a frame that changes nothing resolves nothing.
#[test]
fn the_design_is_resolved_once_until_an_edit_commits() {
    let mut app = reference_app();
    frame(&mut app, 1440.0);
    assert_eq!(app.state.project.configuration.resolution_builds(), 1);
    rspice_design::projection::reset_materialization_count();
    frame(&mut app, 1440.0);
    frame(&mut app, 1440.0);
    assert_eq!(app.state.project.configuration.resolution_builds(), 1);
    assert_eq!(rspice_design::projection::materialization_count(), 0);

    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let id = catalog.active_configuration_id().unwrap();
    app.state
        .project
        .configuration
        .edit(&catalog, id, |definition| {
            definition.executable_view_policy.push("spice".to_owned());
        });
    frame(&mut app, 1440.0);
    assert_eq!(app.state.project.configuration.resolution_builds(), 2);
    frame(&mut app, 1440.0);
    assert_eq!(app.state.project.configuration.resolution_builds(), 2);

    app.state
        .project
        .configuration
        .select(ConfigurationSelection::Default);
    frame(&mut app, 1440.0);
    assert_eq!(app.state.project.configuration.resolution_builds(), 3);
}

#[test]
fn a_dirty_configuration_offers_apply_and_revert() {
    let mut app = reference_app();
    let shown = frame(&mut app, 1440.0);
    assert!(shown.says("Used for runs"));
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let id = catalog.active_configuration_id().unwrap();
    app.state
        .project
        .configuration
        .edit(&catalog, id, |definition| {
            definition.executable_view_policy.push("spice".to_owned());
        });
    let shown = frame(&mut app, 1440.0);
    assert!(shown.says("View list 2: spice"));
    assert!(shown.says("Apply") && shown.says("Revert"));
}

/// Every instance of the expanded design is a row, under its parent, with
/// what it resolved to and the view it is bound to.
#[test]
fn the_instances_tree_states_what_every_instance_resolved_to() {
    let mut app = reference_app();
    let shown = frame(&mut app, 1440.0);
    for (path, cell, to_use, rule) in [
        ("/X1", "user/amp", "Inherited", "View list"),
        ("/X2", "user/amp", "schematic", "Instance binding"),
    ] {
        let row = format!(
            "Instance {path}: cell {cell}; view found schematic; view to use {to_use}; \
             bound by {rule}; model section"
        );
        assert!(
            shown.announced.iter().any(|label| label.starts_with(&row)),
            "no row starts {row:?} in {:#?}",
            shown.announced
        );
    }
    assert!(shown.says(
        "Instance columns: Instance, Cell, View found, View to use, Bound by, Model section, Runs on, Status"
    ));

    let collapsed = app.state.project.configuration.visible_instances().len();
    app.state.project.configuration.toggle_collapsed("/");
    assert_eq!(app.state.project.configuration.visible_instances().len(), 1);
    assert!(collapsed > 1);
}

/// The Cells tab lists each library/cell once, and keeps a cell binding the
/// design does not use, marked, so it can be removed.
#[test]
fn the_cells_tab_lists_each_cell_once_with_its_binding() {
    let mut app = reference_app();
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let id = catalog.active_configuration_id().unwrap();
    app.state
        .project
        .configuration
        .edit(&catalog, id, |definition| {
            crate::workbench::project::configuration_state::set_cell_view(
                definition,
                "vendor",
                "filter",
                Some("schematic"),
            );
            crate::workbench::project::configuration_state::set_cell_view(
                definition,
                "user",
                "retired_block",
                Some("schematic"),
            );
        });
    app.state.project.configuration.tab =
        crate::workbench::project::configuration_state::ConfigurationTab::Cells;
    let shown = frame(&mut app, 1440.0);
    assert!(shown.says(
        "Cell columns: Library, Cell, Instances, View found, View to use, Inherited view list"
    ));
    for row in [
        "Cell user/amp: 2 instances; view found schematic; view to use Inherited; \
         inherited view list schematic",
        "Cell vendor/filter: 1 instance; view found schematic; view to use schematic; \
         inherited view list schematic",
        "Cell user/retired_block: 0 instances; view found \u{2014}; view to use schematic; \
         inherited view list schematic; Warning: not in this design",
    ] {
        assert!(shown.says(row), "no row {row:?} in {:#?}", shown.announced);
    }
    let amp_rows = shown
        .announced
        .iter()
        .filter(|label| label.starts_with("Cell user/amp:"))
        .count();
    assert_eq!(amp_rows, 1, "a cell instantiated twice is one row");
    // The count is the design's cells; the binding beside them is counted
    // apart, so the header agrees with the rows under it.
    assert!(
        shown.says("4 cells \u{b7} 1 binding not in this design"),
        "{:#?}",
        shown.every_word().collect::<Vec<_>>()
    );
}
