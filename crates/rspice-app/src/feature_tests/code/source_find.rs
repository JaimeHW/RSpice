//! Find in source, opened and typed into through the whole application: its
//! command, the dialogs the frame draws, and the bundle search behind it.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::source_find::{LABEL_COLUMN, LABEL_GAP};
use crate::workbench::documents::code_workspace;
use crate::workbench::documents::code_workspace::CodeWorkspacePage;
use egui::Context;

#[test]
fn initial_focus_accepts_source_search_typing() {
    for read_only in [false, true] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
        app.state.code.code_workspace.page = CodeWorkspacePage::Automation;
        if read_only {
            app.state.model.safe_mode.activate(
                crate::state::write_policy::LocalSafeModeOptions {
                    open_project_read_only: true,
                    ..Default::default()
                },
                "UI focus qualification".to_owned(),
            );
        }
        app.execute_command_key(
            crate::workbench::code::commands::CodeCommand::FindCodeDocument.key(),
        );
        let sources = app.state.model.workspace.content.project_sources.clone();
        let mut render = |events| {
            let _ = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1280.0, 800.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| app.render_frame_dialogs(ui),
            );
        };
        render(vec![egui::Event::Paste("import".to_owned())]);
        if !read_only {
            // A later Tab must keep the chosen replacement field focused.
            render(vec![egui::Event::Key {
                key: egui::Key::Tab,
                physical_key: Some(egui::Key::Tab),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }]);
            render(vec![egui::Event::Paste("replacement".to_owned())]);
        }
        let search = app
            .state
            .code
            .code_workspace
            .source_search
            .as_ref()
            .unwrap();
        assert_eq!(search.query, "import");
        assert_eq!(
            search.replacement,
            if read_only { "" } else { "replacement" }
        );
        assert!(app.state.model.workspace.content.project_sources == sources);
    }
}

/// Find in a Verilog-A or Automation source has the shape of Find on the
/// netlist page: Find, Replace and Scope on the label grid, the options
/// under them, one status in the footer, and Replace all as the footer's
/// primary. It was an inspector sheet: a "Query" section over an "Active
/// document" row, a raw dropdown, a "Replace all eligible" button in the
/// body and a "Matches" section in a dialog held 580 points tall, its
/// fields ending a scroll bar's width short of the footer's buttons.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn find_in_source_is_a_label_grid_whose_fields_end_at_the_footer() {
    use egui::accesskit::{Node, Role};

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.code.code_workspace.page = CodeWorkspacePage::Automation;
    code_workspace::open_active_source_search(&mut app.code_workspace_parts())
        .expect("the Automation page owns a searchable bundle");
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..Default::default()
            },
            |ui| app.render_frame_dialogs(ui),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    let named = |text: &str| {
        nodes
            .values()
            .any(|node| node.label() == Some(text) || node.value() == Some(text))
    };
    for row in ["Find", "Replace", "Scope", "Enter text to search."] {
        assert!(named(row), "the dialog names no {row:?}");
    }
    for gone in [
        "Query",
        "Active document",
        "Matches",
        "Replace all eligible",
    ] {
        assert!(!named(gone), "the dialog still names {gone:?}");
    }
    let replace_all = nodes
        .values()
        .find(|node| node.role() == Role::Button && node.label() == Some("Replace all"))
        .expect("Replace all is the footer's primary");
    assert!(
        replace_all.is_disabled(),
        "Replace all is offered with nothing to replace"
    );
    let footer_right = replace_all.bounds().expect("a laid out button").x1;
    let fields = nodes
        .values()
        .filter(|node| matches!(node.role(), Role::TextInput | Role::ComboBox))
        .filter_map(Node::bounds)
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 3, "Find, Replace and Scope");
    let height = |field: &egui::accesskit::Rect| field.y1 - field.y0;
    for field in &fields {
        assert!(
            (field.x1 - footer_right).abs() <= 1.0,
            "a field ends at {} and the footer at {footer_right}",
            field.x1
        );
        // The text fields are as tall as the select; they were shorter.
        assert!(
            (height(field) - height(&fields[0])).abs() <= 1.0,
            "fields of {} and {} points",
            height(field),
            height(&fields[0])
        );
    }
    // Labelled as Go to line labels its field: the kit form's label
    // column, so every field starts 100 pt past its label's left edge.
    let find_label = nodes
        .values()
        .filter(|node| {
            node.role() == Role::Label
                && (node.label() == Some("Find") || node.value() == Some("Find"))
        })
        .find_map(Node::bounds)
        .expect("the Find label is laid out");
    for field in &fields {
        assert!(
            (field.x0 - find_label.x0 - f64::from(LABEL_COLUMN + LABEL_GAP)).abs() <= 1.0,
            "a field starts {} pt past its label",
            field.x0 - find_label.x0
        );
    }
}

/// A result row is a path, a position, and the line it sits on, spread
/// across three cells of which only the first can take focus. Assistive
/// technology therefore heard a bare filename and nothing that identified
/// which of its matches this was.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_result_row_announces_its_whole_match_and_not_just_the_focusable_cell() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.code.code_workspace.page = CodeWorkspacePage::Automation;
    code_workspace::open_active_source_search(&mut app.code_workspace_parts())
        .expect("the Automation page owns a searchable bundle");
    let Some(search) = app.state.code.code_workspace.source_search.as_mut() else {
        panic!("the search opened");
    };
    search.query = "import".to_owned();
    let path = search.active_path.clone();

    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_100.0, 800.0),
            )),
            ..Default::default()
        },
        |ui| app.render_frame_dialogs(ui),
    );
    let Some(update) = output.platform_output.accesskit_update else {
        panic!("the find dialog publishes an accessibility tree");
    };
    let labels = update
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label())
        .collect::<Vec<_>>();
    let Some(row) = labels
        .iter()
        .find(|label| label.starts_with(&path) && label.contains("line "))
    else {
        panic!("no result row announced its position; labels: {labels:?}");
    };
    assert!(
        row.contains("column ") && row.contains("import"),
        "a row announced its path and line but not its column or its source line: {row:?}"
    );
}
