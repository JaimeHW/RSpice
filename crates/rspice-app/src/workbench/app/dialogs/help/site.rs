//! The pages of the RSpice site the Help menu opens in the reader's browser:
//! this version's release notes, and the documentation for what is in front.
//!
//! The site publishes one release-notes entry per version, under the
//! element id `v<version>`, so the link lands on the entry for the build
//! the reader is running. Its documentation is one page per topic under
//! `/docs/<slug>`, and an index at `/docs`. This module is the one table of
//! which topic the reader is looking at: the dialog on top, when it has an
//! obvious topic, else the workspace on screen.

use crate::state::ViewType;
use crate::workbench::app_state::AppState;
use crate::workbench::framework::dialogs::ModalGate;
use crate::workbench::framework::requests::{OpenWebPage, Outbox};
use crate::workbench::state::Workspace;

/// This version's entry in the release notes.
pub(in crate::workbench) const RELEASE_NOTES_URL: &str =
    concat!("https://rspice.app/changelog#v", env!("CARGO_PKG_VERSION"));

/// The documentation's index, and the parent of every topic's page.
pub(in crate::workbench) const DOCS_URL: &str = "https://rspice.app/docs";

/// Ask the frame to open this version's release notes in a new tab.
pub(in crate::workbench) fn open_release_notes(outbox: &mut Outbox) {
    outbox.push(OpenWebPage(RELEASE_NOTES_URL.to_owned()));
}

/// Ask the frame to open, in a new tab, the documentation for what is in
/// front of the reader in `state`.
pub(in crate::workbench) fn open_docs(state: &mut AppState) {
    let url = docs_url(topic_in_front(state));
    state.workbench.outbox.push(OpenWebPage(url));
}

/// A topic of the documentation: one page of the site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::workbench) enum DocsTopic {
    GettingStarted,
    SchematicEntry,
    SimulationSetup,
    Results,
    Verification,
    ModelsAndPdks,
    VerilogA,
    NetlistsAndScripts,
    Troubleshooting,
    KeyboardShortcuts,
}

impl DocsTopic {
    #[cfg(test)]
    const ALL: [Self; 10] = [
        Self::GettingStarted,
        Self::SchematicEntry,
        Self::SimulationSetup,
        Self::Results,
        Self::Verification,
        Self::ModelsAndPdks,
        Self::VerilogA,
        Self::NetlistsAndScripts,
        Self::Troubleshooting,
        Self::KeyboardShortcuts,
    ];

    /// The topic's page under [`DOCS_URL`].
    pub(in crate::workbench) const fn slug(self) -> &'static str {
        match self {
            Self::GettingStarted => "getting-started",
            Self::SchematicEntry => "schematic-entry",
            Self::SimulationSetup => "simulation-setup",
            Self::Results => "results",
            Self::Verification => "verification",
            Self::ModelsAndPdks => "models-and-pdks",
            Self::VerilogA => "verilog-a",
            Self::NetlistsAndScripts => "netlists-and-scripts",
            Self::Troubleshooting => "troubleshooting",
            Self::KeyboardShortcuts => "keyboard-shortcuts",
        }
    }
}

/// The page for `topic`; the index when there is none.
pub(in crate::workbench) fn docs_url(topic: Option<DocsTopic>) -> String {
    match topic {
        Some(topic) => format!("{DOCS_URL}/{}", topic.slug()),
        None => DOCS_URL.to_owned(),
    }
}

/// What a workspace's pages are for. Design is about schematic entry, or
/// about Verilog-A while it shows a Verilog-A view.
const fn workspace_topic(workspace: Workspace, view: ViewType) -> DocsTopic {
    match workspace {
        Workspace::Project => DocsTopic::GettingStarted,
        Workspace::Design if matches!(view, ViewType::VerilogA) => DocsTopic::VerilogA,
        Workspace::Design => DocsTopic::SchematicEntry,
        Workspace::Simulate | Workspace::Stimulus => DocsTopic::SimulationSetup,
        Workspace::Results => DocsTopic::Results,
        Workspace::Verify => DocsTopic::Verification,
        Workspace::Models => DocsTopic::ModelsAndPdks,
        Workspace::Netlist => DocsTopic::NetlistsAndScripts,
    }
}

/// The dialogs with an obvious topic, by the key the dialog order lists
/// them under. A dialog not listed here has none, and opens the index.
const DIALOG_TOPICS: &[(&str, DocsTopic)] = &[
    ("project.new_project", DocsTopic::GettingStarted),
    ("project.launcher", DocsTopic::GettingStarted),
    ("project.launcher_route", DocsTopic::GettingStarted),
    ("project.examples", DocsTopic::GettingStarted),
    ("design.property", DocsTopic::SchematicEntry),
    ("design.bus_tap", DocsTopic::SchematicEntry),
    ("design.builtin_xspice_placement", DocsTopic::SchematicEntry),
    ("design.net_label", DocsTopic::SchematicEntry),
    ("design.grid_snap_routing", DocsTopic::SchematicEntry),
    ("design.schematic_visibility", DocsTopic::SchematicEntry),
    ("design.pin_port", DocsTopic::SchematicEntry),
    ("design.design_note", DocsTopic::SchematicEntry),
    ("design.documentation_shape", DocsTopic::SchematicEntry),
    ("design.array_selection", DocsTopic::SchematicEntry),
    ("design.replace_instance", DocsTopic::SchematicEntry),
    ("design.create_hierarchy", DocsTopic::SchematicEntry),
    ("design.connectivity_manager", DocsTopic::SchematicEntry),
    ("design.selection_bulk_edit", DocsTopic::SchematicEntry),
    ("design.object_properties", DocsTopic::SchematicEntry),
    ("design.rename_selection", DocsTopic::SchematicEntry),
    ("design.find_replace", DocsTopic::SchematicEntry),
    ("design.shared_properties", DocsTopic::SchematicEntry),
    ("design.stimulus_link", DocsTopic::SimulationSetup),
    ("simulate.workflow_dialogs", DocsTopic::SimulationSetup),
    ("simulate.jobs_manager", DocsTopic::SimulationSetup),
    ("results.result_import", DocsTopic::Results),
    ("results.dataset_export", DocsTopic::Results),
    ("results.run_details", DocsTopic::Results),
    ("results.create_result_document", DocsTopic::Results),
    ("results.result_document", DocsTopic::Results),
    (
        "simulate.regression_baseline_picker",
        DocsTopic::Verification,
    ),
    ("simulate.tuning_review", DocsTopic::Verification),
    ("models.pdk_settings", DocsTopic::ModelsAndPdks),
    ("models.model_browser", DocsTopic::ModelsAndPdks),
    ("models.model_correlation", DocsTopic::ModelsAndPdks),
    ("models.models_view", DocsTopic::ModelsAndPdks),
    ("project.technology_attachment", DocsTopic::ModelsAndPdks),
    ("code.new_veriloga_device", DocsTopic::VerilogA),
    ("code.veriloga_build_settings", DocsTopic::VerilogA),
    ("code.netlist", DocsTopic::NetlistsAndScripts),
    ("services.autosave_restore", DocsTopic::Troubleshooting),
    ("shell.shortcuts_help", DocsTopic::KeyboardShortcuts),
];

/// The topic of what is in front of the reader: the dialog on top when one
/// holds the window, by its own topic or none; else the start screen's,
/// with no project open; else the workspace's.
pub(in crate::workbench) fn topic_in_front(state: &AppState) -> Option<DocsTopic> {
    if let Some(key) = dialog_in_front(state) {
        return DIALOG_TOPICS
            .iter()
            .find(|(listed, _)| *listed == key)
            .map(|(_, topic)| *topic);
    }
    if !state.platform.project_lifecycle.is_open() {
        return Some(DocsTopic::GettingStarted);
    }
    Some(workspace_topic(
        state.workbench.workspace,
        state.model.workspace.content.active_view_type(),
    ))
}

/// The key of the modal dialog on top, if one is open: the last open step of
/// the dialog order, which draws over every step before it, else the last
/// open modal the steps do not draw.
fn dialog_in_front(state: &AppState) -> Option<&'static str> {
    <AppState as ModalGate>::ORDER
        .iter()
        .rev()
        .find(|spec| spec.is_open.is_some_and(|is_open| is_open(state)))
        .map(|spec| spec.key.0)
        .or_else(|| {
            <AppState as ModalGate>::MODAL_ONLY
                .iter()
                .rev()
                .find(|spec| (spec.is_open)(state))
                .map(|spec| spec.key.0)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::RSpiceApp;
    use crate::workbench::commands::vocabulary::Command;
    use egui::{Context, OpenUrl, OutputCommand, Rect, pos2, vec2};

    /// One whole application frame with `events`; the pages it asked the
    /// platform to open. Generic over the app, so the helper is not a
    /// handler that takes the whole application.
    fn opened_pages_after(app: &mut impl eframe::App, events: Vec<egui::Event>) -> Vec<OpenUrl> {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut frame = eframe::Frame::_new_kittest();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(1440.0, 900.0))),
                time: Some(0.1),
                focused: true,
                events,
                ..Default::default()
            },
            |ui| eframe::App::ui(app, ui, &mut frame),
        );
        output
            .platform_output
            .commands
            .into_iter()
            .filter_map(|command| match command {
                OutputCommand::OpenUrl(open) => Some(open),
                _ => None,
            })
            .collect()
    }

    fn opened_pages(app: &mut impl eframe::App) -> Vec<OpenUrl> {
        opened_pages_after(app, Vec::new())
    }

    fn f1() -> egui::Event {
        egui::Event::Key {
            key: egui::Key::F1,
            physical_key: Some(egui::Key::F1),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn open_project_on(workspace: Workspace) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        app.state.workbench.activate(workspace);
        app
    }

    #[test]
    fn release_notes_open_this_versions_entry_on_the_site_in_a_new_tab() {
        assert_eq!(
            RELEASE_NOTES_URL,
            format!(
                "https://rspice.app/changelog#v{}",
                env!("CARGO_PKG_VERSION")
            )
        );
        let mut app = RSpiceApp::test_instance();
        assert!(opened_pages(&mut app).is_empty(), "nothing opens unasked");
        Command::ReleaseNotes.execute(&mut app);
        assert_eq!(
            opened_pages(&mut app),
            [OpenUrl::new_tab(RELEASE_NOTES_URL)]
        );
        assert!(
            !app.state.shell.help_center.open,
            "release notes are the site's, not a page of the Help center"
        );
        assert!(opened_pages(&mut app).is_empty(), "the page opens once");
    }

    /// The topics are exactly the pages the site publishes, in its order.
    #[test]
    fn the_topics_are_the_sites_documentation_pages() {
        assert_eq!(
            DocsTopic::ALL.map(DocsTopic::slug),
            [
                "getting-started",
                "schematic-entry",
                "simulation-setup",
                "results",
                "verification",
                "models-and-pdks",
                "verilog-a",
                "netlists-and-scripts",
                "troubleshooting",
                "keyboard-shortcuts",
            ]
        );
        assert_eq!(docs_url(None), "https://rspice.app/docs");
        assert_eq!(
            docs_url(Some(DocsTopic::ModelsAndPdks)),
            "https://rspice.app/docs/models-and-pdks"
        );
    }

    /// Every workspace has a topic of its own, by an exhaustive match: a new
    /// workspace does not compile until it is given one.
    #[test]
    fn every_workspace_maps_to_a_topic() {
        let expected = [
            (Workspace::Project, DocsTopic::GettingStarted),
            (Workspace::Design, DocsTopic::SchematicEntry),
            (Workspace::Simulate, DocsTopic::SimulationSetup),
            (Workspace::Stimulus, DocsTopic::SimulationSetup),
            (Workspace::Results, DocsTopic::Results),
            (Workspace::Verify, DocsTopic::Verification),
            (Workspace::Models, DocsTopic::ModelsAndPdks),
            (Workspace::Netlist, DocsTopic::NetlistsAndScripts),
        ];
        assert_eq!(
            expected.map(|(workspace, _)| workspace),
            Workspace::ALL,
            "the table lists every workspace"
        );
        for (workspace, topic) in expected {
            assert_eq!(workspace_topic(workspace, ViewType::Schematic), topic);
            assert!(DocsTopic::ALL.contains(&workspace_topic(workspace, ViewType::Schematic)));
        }
        assert_eq!(
            workspace_topic(Workspace::Design, ViewType::VerilogA),
            DocsTopic::VerilogA
        );
    }

    /// Every dialog the table names is one the dialog order draws or gates,
    /// listed once.
    #[test]
    fn the_dialog_table_names_only_dialogs_the_order_lists() {
        let listed: Vec<&str> = <AppState as ModalGate>::ORDER
            .iter()
            .filter(|spec| spec.is_open.is_some())
            .map(|spec| spec.key.0)
            .chain(
                <AppState as ModalGate>::MODAL_ONLY
                    .iter()
                    .map(|spec| spec.key.0),
            )
            .collect();
        for (index, (key, _)) in DIALOG_TOPICS.iter().enumerate() {
            assert!(
                listed.contains(key),
                "`{key}` is no modal of the dialog order"
            );
            assert!(
                !DIALOG_TOPICS[..index]
                    .iter()
                    .any(|(earlier, _)| earlier == key),
                "`{key}` is listed twice"
            );
        }
    }

    /// RSpice Help opens the page for the workspace on screen, the start
    /// screen's with no project open, and Verilog-A's over a Verilog-A view.
    #[test]
    fn help_opens_the_page_for_the_workspace_in_front() {
        let mut app = RSpiceApp::test_instance();
        app.state.close_project_session();
        Command::HelpCenter.execute(&mut app);
        assert_eq!(
            opened_pages(&mut app),
            [OpenUrl::new_tab("https://rspice.app/docs/getting-started")]
        );
        assert!(!app.state.shell.help_center.open, "Help is the site's");

        for (workspace, slug) in [
            (Workspace::Design, "schematic-entry"),
            (Workspace::Results, "results"),
            (Workspace::Models, "models-and-pdks"),
        ] {
            let mut app = open_project_on(workspace);
            Command::HelpCenter.execute(&mut app);
            assert_eq!(
                opened_pages(&mut app),
                [OpenUrl::new_tab(format!("https://rspice.app/docs/{slug}"))],
                "{workspace:?}"
            );
        }
    }

    /// F1 still answers while a dialog holds the keyboard, and opens that
    /// dialog's topic, or the index for a dialog without one. No other key
    /// gets through.
    #[test]
    fn f1_over_a_dialog_opens_its_topic() {
        let mut app = open_project_on(Workspace::Design);
        app.state.shell.shortcuts_help = true;
        assert!(app.state.application_modal_open());
        assert_eq!(
            opened_pages_after(&mut app, vec![f1()]),
            [OpenUrl::new_tab(
                "https://rspice.app/docs/keyboard-shortcuts"
            )]
        );
        assert!(app.state.shell.shortcuts_help, "the dialog stays open");

        let mut app = open_project_on(Workspace::Design);
        app.state.shell.about = true;
        assert_eq!(
            opened_pages_after(&mut app, vec![f1()]),
            [OpenUrl::new_tab("https://rspice.app/docs")]
        );

        let mut app = open_project_on(Workspace::Results);
        assert_eq!(
            opened_pages_after(&mut app, vec![f1()]),
            [OpenUrl::new_tab("https://rspice.app/docs/results")]
        );
    }
}
