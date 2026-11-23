//! The source editor's menu through the whole application: the commands the
//! source feature reads, and the rows a page's context reads once it has drawn.

use crate::workbench::{RSpiceApp, features};
use egui::os::OperatingSystem;
use rspice_app_code::workbench::code::SourceFeature;
use rspice_app_code::workbench::documents::netlist_document::open_run_deck_snapshot_in;
use rspice_app_code::workbench::surfaces::netlist::fixtures::seal_manual_run;
use rspice_app_code::workbench::surfaces::netlist::source_menu::SourceMenuLookup;
use rspice_app_code::workbench::surfaces::netlist::{prepare_workspace, show_prepared};
use rspice_app_state::state::command_catalog::CommandPlatform;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::chrome::title_bar::menu_entries::{
    menu_keys, resolve_menu_rows,
};
use rspice_app_workbench::workbench::framework::command_registry::CommandAvailability;
use rspice_app_workbench::workbench::framework::feature::Feature;
use rspice_app_workbench::workbench::framework::host::Host;

/// The commands the source feature declares its menus read are the source
/// editor menu's, in its order.
#[test]
fn the_source_feature_reads_the_source_editor_menus_commands() {
    let app = RSpiceApp::test_instance();
    let mut keys = Vec::new();
    menu_keys(app.source_editor_menu(), &mut keys);
    assert_eq!(keys, SourceFeature::MENU_AVAILABILITY);
}

/// What the deck page shows, and so the editor it puts in front.
#[derive(Clone, Copy, Debug)]
enum DeckPage {
    /// No deck: the page offers to start or import one.
    Empty,
    /// A deck the project owns, which the editor writes.
    Owned,
    /// The deck a run used, which the editor only reads.
    RunSnapshot,
}

impl DeckPage {
    /// Whether the editor the page puts in front writes and navigates;
    /// `None` where it draws no editor.
    const fn editor(self) -> Option<(bool, bool)> {
        match self {
            Self::Empty => None,
            Self::Owned => Some((true, true)),
            Self::RunSnapshot => Some((false, true)),
        }
    }
}

/// The application on the deck page showing `page`.
fn deck_page(page: DeckPage, project_open: bool) -> RSpiceApp {
    const DECK: &str = "menu fixture\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let mut app = RSpiceApp::test_instance();
    if project_open {
        app.state.open_project_session();
    }
    app.state.workbench.workspace = Workspace::Netlist;
    if !matches!(page, DeckPage::Empty) {
        app.state.seed_test_deck(DECK.to_owned());
    }
    if matches!(page, DeckPage::RunSnapshot) {
        app.state.runs.manual_run.last_run_buffer = Some(DECK.into());
        app.state.runs.manual_run.last_run_id = Some(7);
        seal_manual_run(&mut app.state.model, 7, 0xAB, 4);
        assert!(open_run_deck_snapshot_in(
            &mut app.state.model,
            &mut app.state.code.netlist,
            &app.state.runs,
        ));
    }
    app
}

/// One frame of the deck page, in the context the shell lends it while
/// its menu is closed.
fn draw(app: &mut RSpiceApp) {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let _ = ctx.run_ui(Default::default(), |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            show_prepared(ui, &mut features::cx_with_menu_answers(app, |_| false));
        });
    });
}

/// The dispatcher's answers for the commands the source feature reads on
/// every frame: the toolbar's.
fn toolbar_answers(app: &RSpiceApp) -> Vec<CommandAvailability> {
    SourceFeature::FOREIGN_AVAILABILITY
        .iter()
        .map(|key| app.command_availability(*key))
        .collect()
}

/// The deck page's context, lent with its menu's answers once the page
/// has prepared, reads the rows the host's menu reads once the page has
/// drawn, label, key, availability, check mark, icon and access letter
/// alike: with no deck, with a deck the project owns and with the
/// read-only deck a run used, with or without a project open, on each
/// platform. Preparing puts in front the editor the page then draws, and
/// leaves the toolbar's answers as they were without it.
#[test]
fn the_pages_context_reads_the_rows_the_host_reads_once_the_page_has_drawn() {
    let entries = rspice_app_code::workbench::code::SOURCE_EDITOR_MENU;
    let places = [
        (CommandPlatform::Desktop, OperatingSystem::Windows),
        (CommandPlatform::Desktop, OperatingSystem::Mac),
        (CommandPlatform::Browser, OperatingSystem::Nix),
        (CommandPlatform::Phone, OperatingSystem::Android),
    ];
    for project_open in [false, true] {
        for page in [DeckPage::Empty, DeckPage::Owned, DeckPage::RunSnapshot] {
            let case = format!("{page:?}, project open {project_open}");
            let mut app = deck_page(page, project_open);
            // The surfaces leave no editor in front before the page.
            app.state.workbench.active_source_editor = None;
            let toolbar = toolbar_answers(&app);
            prepare_workspace(&mut features::cx(&mut app));
            let prepared = app.state.workbench.active_source_editor;
            assert_eq!(
                prepared.map(|editor| (editor.editable, editor.navigable)),
                page.editor(),
                "{case}"
            );
            assert_eq!(toolbar_answers(&app), toolbar, "the toolbar's, {case}");
            let lent: Vec<String> = places
                .iter()
                .map(|&(platform, os)| {
                    let cx = features::cx_with_menu_answers::<SourceFeature>(&mut app, |_| true);
                    let rows = resolve_menu_rows(entries, &SourceMenuLookup(&cx), platform, os);
                    format!("{rows:?}")
                })
                .collect();
            draw(&mut app);
            assert_eq!(
                app.state.workbench.active_source_editor, prepared,
                "the page publishes the editor preparing put in front, {case}"
            );
            for (&(platform, os), code) in places.iter().zip(lent) {
                let host = format!("{:?}", resolve_menu_rows(entries, &app, platform, os));
                assert_eq!(code, host, "{case}, {platform:?} {os:?}");
            }
        }
    }
}

/// One frame of the application's deck page on `ctx`, which keeps the
/// editor's session from frame to frame, carrying `events`.
fn page_frame(ctx: &egui::Context, app: &mut RSpiceApp, events: Vec<egui::Event>) {
    let input = egui::RawInput {
        events,
        ..Default::default()
    };
    let _ = ctx.run_ui(input, |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            show_prepared(ui, &mut features::cx_with_menu_answers(app, |_| false));
        });
    });
}

/// What Edit > Undo, Cut, Copy, Paste, Delete and Select All answer.
fn edit_answers(app: &RSpiceApp) -> [CommandAvailability; 6] {
    use rspice_app_workbench::workbench::framework::command_registry::ids;
    [
        ids::UNDO,
        ids::CUT,
        ids::COPY,
        ids::PASTE,
        ids::DELETE,
        ids::SELECT_ALL,
    ]
    .map(|key| app.command_availability(key))
}

/// Edit > Undo offers a step only once the deck's editor has one to take.
#[test]
fn undo_waits_for_a_step_of_the_editors_history() {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = deck_page(DeckPage::Owned, true);
    prepare_workspace(&mut features::cx(&mut app));
    page_frame(&ctx, &mut app, Vec::new());
    page_frame(&ctx, &mut app, Vec::new());
    assert_eq!(
        edit_answers(&app)[0],
        CommandAvailability::Disabled("nothing to undo"),
        "a deck just opened has nothing to undo"
    );

    let editor = rspice_app_code::workbench::documents::netlist_document::editor_id_in(
        &app.state.model,
        &app.state.code.netlist,
    );
    ctx.memory_mut(|memory| memory.request_focus(editor));
    page_frame(&ctx, &mut app, Vec::new());
    page_frame(&ctx, &mut app, vec![egui::Event::Text("x".to_owned())]);
    page_frame(&ctx, &mut app, Vec::new());
    assert_eq!(
        edit_answers(&app)[0],
        CommandAvailability::Available,
        "a typed character can be undone"
    );
}

/// Edit > Cut and Copy act on selected text, so they wait for the reader
/// to select some.
#[test]
fn cut_and_copy_wait_for_selected_text() {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = deck_page(DeckPage::Owned, true);
    prepare_workspace(&mut features::cx(&mut app));
    page_frame(&ctx, &mut app, Vec::new());
    page_frame(&ctx, &mut app, Vec::new());
    let nothing = CommandAvailability::Disabled("nothing is selected");
    assert_eq!(edit_answers(&app)[1..3], [nothing, nothing]);

    let editor = rspice_app_code::workbench::documents::netlist_document::editor_id_in(
        &app.state.model,
        &app.state.code.netlist,
    );
    ctx.memory_mut(|memory| memory.request_focus(editor));
    page_frame(&ctx, &mut app, Vec::new());
    let select_all = egui::Event::Key {
        key: egui::Key::A,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::COMMAND,
    };
    page_frame(&ctx, &mut app, vec![select_all]);
    page_frame(&ctx, &mut app, Vec::new());
    assert_eq!(
        edit_answers(&app)[1..3],
        [
            CommandAvailability::Available,
            CommandAvailability::Available
        ],
        "the whole deck is selected"
    );
}

/// On the read-only deck a run used, every edit is unavailable, Undo
/// included, because the document is read-only, not because there is
/// nothing to undo; reading it stays possible.
#[test]
fn a_read_only_deck_shows_its_edits_unavailable_as_read_only() {
    let mut app = deck_page(DeckPage::RunSnapshot, true);
    prepare_workspace(&mut features::cx(&mut app));
    draw(&mut app);
    let read_only = CommandAvailability::Disabled("the document is read-only");
    assert_eq!(
        edit_answers(&app),
        [
            read_only,
            read_only,
            CommandAvailability::Disabled("nothing is selected"),
            read_only,
            read_only,
            CommandAvailability::Available,
        ]
    );
}
