//! Source document dialog tests: include search roots, role labels, the lifecycle
//! transactions, and the creation form a project with no deck opens.

use super::*;
use crate::state::{ProjectDescriptor, ProjectSourceOwner};
use crate::workbench::documents::code_workspace::CodeWorkspacePage;
use rspice_simulation::netlist_preparation::IncludeSearchChain;
use std::path::{Path, PathBuf};

#[test]
fn a_project_places_its_relative_entries_against_its_own_folder() {
    let mut project = ProjectDescriptor::default();
    project.path = Some(PathBuf::from("/projects/mixer/mixer.rspiceproj"));
    project
        .set_include_search_paths(vec![PathBuf::from("models"), PathBuf::from("/opt/pdk/lib")])
        .expect("chain is accepted");

    let directories = rspice_simulation::netlist_preparation::IncludeSearchChain::resolve(
        project.include_search_paths(),
        project.data_root(),
    )
    .directories();
    assert_eq!(directories.len(), 2);
    assert_eq!(directories[0], Path::new("/projects/mixer").join("models"));
    assert_eq!(directories[1], PathBuf::from("/opt/pdk/lib"));
}

#[test]
fn relative_entries_resolve_against_the_project_data_root() {
    let root = crate::fixture_root::canonical_temp_dir().join(format!(
        "rspice-include-chain-relative-{}",
        std::process::id()
    ));
    let models = root.join("models");
    std::fs::create_dir_all(&models).expect("create chain fixture");

    let chain = IncludeSearchChain::resolve(
        &[PathBuf::from("models"), PathBuf::from("absent")],
        Some(root.as_path()),
    );

    assert_eq!(chain.entries().len(), 2);
    assert_eq!(chain.entries()[0].resolved(), models.as_path());
    assert!(chain.entries()[0].exists());
    assert_eq!(chain.entries()[1].resolved(), root.join("absent"));
    assert!(!chain.entries()[1].exists());
    std::fs::remove_dir_all(&root).ok();
}

/// The choice of engine entry point lives here and nowhere else, so it is
/// asserted by its effect: the same deck refuses to resolve without a
/// chain and resolves through it with one.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_chain_decides_which_engine_entry_point_a_host_parse_takes() {
    let root = crate::fixture_root::canonical_temp_dir().join(format!(
        "rspice-include-chain-entry-point-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos()
    ));
    let deck_dir = root.join("deck");
    let library_dir = root.join("pdk");
    std::fs::create_dir_all(&deck_dir).expect("create deck directory");
    std::fs::create_dir_all(&library_dir).expect("create library directory");
    std::fs::write(
        library_dir.join("device.lib"),
        ".model DCHAIN D(Is=1e-14)\n",
    )
    .expect("write library");
    let deck_path = deck_dir.join("top.cir");
    let source = "chain entry point\n.include device.lib\nD1 out 0 DCHAIN\n.end\n";
    let options = rspice_core::netlist::NetlistParseOptions::default();

    let without = IncludeSearchChain::default().parse_with_abort(
        source,
        Some(&deck_path),
        options,
        &rspice_core::abort_signal::NoAbort,
    );
    assert!(
        without.is_err(),
        "without a chain the include is nowhere the resolver looks"
    );

    let chain = IncludeSearchChain::resolve(&[library_dir.clone()], Some(root.as_path()));
    let with = chain
        .parse_with_abort(
            source,
            Some(&deck_path),
            options,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("the chain resolves the include");
    assert_eq!(with.source_path.as_deref(), Some(deck_path.as_path()));

    std::fs::remove_dir_all(&root).ok();
}

/// Line endings are reported from the retained bytes. A whole-file rewrite
/// normalizes a mixed document, so the dialog has to be able to say so.
fn english() -> MessageCatalog {
    MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates)
}

#[test]
fn line_endings_are_measured_rather_than_assumed() {
    let english = english();
    let summary = |content| english.text(line_ending_summary(content));
    assert_eq!(summary("a\nb\n"), "LF");
    assert_eq!(summary("a\r\nb\r\n"), "CRLF");
    assert_eq!(summary("a\r\nb\n"), "mixed LF and CRLF");
    assert_eq!(summary("single"), "none \u{00b7} single line");
}

/// The role picker resolves a selection by matching the label it drew, so
/// every assignable role has to round-trip through its own text in every
/// locale — including the one whose expansion rewrites every vowel.
#[test]
fn every_assignable_role_round_trips_through_its_label_in_every_locale() {
    for locale in crate::workbench::UiTextLocale::ALL {
        let messages = MessageCatalog::new(locale);
        let mut drawn = std::collections::BTreeSet::new();
        for role in ASSIGNABLE_AUTOMATION_ROLES {
            let label = role_label(messages, role);
            assert!(
                drawn.insert(label.clone()),
                "two roles share the label {label:?} in {locale:?}"
            );
            assert_eq!(
                ASSIGNABLE_AUTOMATION_ROLES
                    .iter()
                    .find(|candidate| role_label(messages, **candidate) == label)
                    .copied(),
                Some(role)
            );
        }
    }
}

/// The defect this dialog exists for. Every file operation was reachable
/// only from a test, so the whole lifecycle shipped unreachable. Assert a
/// real transaction can be opened, edited, and committed through the same
/// entry points the body uses.
#[test]
fn a_rename_transaction_runs_end_to_end_through_the_dialog_entry_points() {
    let mut app = RSpiceApp::test_instance();
    app.state.code.code_workspace.page = CodeWorkspacePage::Automation;
    let language = ProjectSourceLanguage::RSpiceAutomation;
    let owner = ProjectSourceOwner::code_workspace(language);
    let original = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .map(|bundle| bundle.root().logical_path().to_owned())
        .unwrap_or_default();
    assert!(!original.is_empty(), "the fixture project owns a bundle");

    open_source_document_dialog(&mut app.state).expect("the Automation page owns a source bundle");
    let facts = document_facts(&app, english()).expect("the page states a document");
    assert_eq!(facts.logical_path, original);
    assert!(facts.is_root, "the bundle root is the default document");

    app.apply_lifecycle_action(LifecycleAction::Begin(CodeSourceFileAction::Rename), &facts);
    let draft = app
        .state
        .code
        .code_workspace
        .source_file_dialog
        .as_mut()
        .expect("Rename opens a transaction");
    draft.proposed_path = "workflows/renamed_entry.py".to_owned();

    app.apply_lifecycle_action(LifecycleAction::Commit, &facts);
    assert!(
        app.state.code.code_workspace.source_file_dialog.is_none(),
        "a committed transaction closes"
    );
    let renamed = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .map(|bundle| bundle.root().logical_path().to_owned())
        .unwrap_or_default();
    assert_eq!(renamed, "workflows/renamed_entry.py");
}

/// The environment lock refuses graph mutation and its refusal tells the
/// user to reassign the role first. That instruction was impossible to
/// follow, because no control could set a role. Assert the refusal, the
/// reassignment, and that the refusal then lifts.
#[test]
fn reassigning_a_role_lifts_the_environment_lock_refusal() {
    let mut app = RSpiceApp::test_instance();
    app.state.code.code_workspace.page = CodeWorkspacePage::Automation;
    let language = ProjectSourceLanguage::RSpiceAutomation;
    let owner = ProjectSourceOwner::code_workspace(language);
    let locked = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .and_then(|bundle| {
            bundle
                .paths_for_role(crate::state::ProjectSourceRole::AutomationEnvironmentLock)
                .next()
                .map(str::to_owned)
        })
        .unwrap_or_default();
    assert!(!locked.is_empty(), "the fixture bundle pins an environment");
    app.state.code.code_workspace.automation.selected_file = Some(locked.clone());

    let before = document_facts(&app, english()).expect("the lock is a document");
    assert!(before.is_environment_lock);
    assert!(
        refusal_reason(&before, CodeSourceFileAction::Rename).is_some(),
        "the lock refuses graph mutation"
    );

    app.apply_lifecycle_action(LifecycleAction::AssignRole(None), &before);

    let after = document_facts(&app, english()).expect("the document survives the role change");
    assert!(!after.is_environment_lock, "the role was reassigned");
    assert!(
        refusal_reason(&after, CodeSourceFileAction::Rename).is_none(),
        "the refusal the service states is now followable"
    );
}

/// An empty project exposes the lifecycle manager so the first top deck
/// can be created without a generated or imported predecessor.
#[test]
fn a_generated_netlist_page_refuses_the_lifecycle_until_a_deck_is_owned() {
    let mut app = RSpiceApp::test_instance();
    app.state.code.code_workspace.page = CodeWorkspacePage::Netlist;
    // The generated deck is read-only and the project owns no other:
    // there is nothing to manage, and the refusal says so.
    assert!(!source_document_dialog_is_available(&app.state));
    let refusal = open_source_document_dialog(&mut app.state).unwrap_err();
    assert!(refusal.contains("read-only"), "{refusal}");
    assert!(!app.state.code.code_workspace.source_document_dialog);

    crate::workbench::documents::netlist_document::begin_netlist_lifecycle_action(
        &mut app.state,
        CodeSourceFileAction::New,
    )
    .unwrap();
    crate::workbench::documents::netlist_document::commit_netlist_lifecycle_action(&mut app.state)
        .unwrap();
    assert!(source_document_dialog_is_available(&app.state));
    open_source_document_dialog(&mut app.state).unwrap();
    assert!(app.state.code.code_workspace.source_document_dialog);
    let facts = netlist_lifecycle_facts(&app).unwrap();
    assert!(facts.deck_id.is_some());
    assert_eq!(facts.decks.len(), 1);
}

/// The page's New source document and New top deck make the deck that
/// refusal asks for, so they open the dialog on its review where Manage
/// source document refuses. They opened nothing but the refusal while
/// they went through Manage's route.
#[test]
fn new_source_document_opens_on_a_project_that_owns_no_deck() {
    let mut app = RSpiceApp::test_instance();
    app.state.code.code_workspace.page = CodeWorkspacePage::Netlist;
    assert!(open_source_document_dialog(&mut app.state).is_err());

    begin_new_top_deck(&mut app.state).unwrap();
    assert!(app.state.code.code_workspace.source_document_dialog);
    let transaction = app
        .state
        .code
        .netlist
        .lifecycle_dialog
        .transaction
        .as_ref()
        .expect("the dialog reviews the new deck");
    assert_eq!(transaction.action, CodeSourceFileAction::New);
    assert!(!transaction.proposed_path.is_empty());
}

/// The deck's properties are its name, where it is on disk and its line
/// endings. The dialog used to add a twelve-digit deck and document
/// identity, a line and byte count, "Not published to a reopenable
/// native path" and a count of the project's decks.
#[test]
fn an_owned_deck_states_its_properties_without_identity_jargon() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = owned_deck_app();
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| app.render_source_document_dialog(ui.ctx()),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    let says = |text: &str| {
        nodes.values().any(|node| {
            node.label().is_some_and(|label| label.contains(text))
                || node.value().is_some_and(|value| value.contains(text))
        })
    };
    assert!(says("Top deck"), "the deck row is missing");
    assert!(says("Line endings"), "the line endings row is missing");
    for jargon in [
        "Identity",
        "Published origin",
        "reopenable",
        "Project top decks",
        " bytes",
    ] {
        assert!(!says(jargon), "the dialog states {jargon:?}");
    }
}

/// An Automation or Verilog-A document states its path, role, line
/// endings and, when stale, its validation. Its twelve-digit identity,
/// line and byte count and bundle digest are gone, and its includes are
/// said in plain words: not "Recorded includes" over "no recorded
/// include edge in either direction".
#[test]
fn a_source_document_states_its_properties_without_identity_jargon() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.code.code_workspace.page = CodeWorkspacePage::Automation;
    open_source_document_dialog(&mut app.state).unwrap();
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| app.render_source_document_dialog(ui.ctx()),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    let says = |text: &str| {
        nodes.values().any(|node| {
            node.label().is_some_and(|label| label.contains(text))
                || node.value().is_some_and(|value| value.contains(text))
        })
    };
    assert!(says("Path"), "the path row is missing");
    assert!(says("Line endings"), "the line endings row is missing");
    assert!(says("Includes"), "the includes section is missing");
    for jargon in [
        "Identity",
        " bytes",
        "Bundle",
        "Recorded",
        "include edge",
        "either direction",
    ] {
        assert!(!says(jargon), "the dialog states {jargon:?}");
    }
}

/// A project that owns one authored top deck, on the netlist page.
fn owned_deck_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.model.preferences.set_toggle(
        crate::workbench::preferences::TogglePreference::ReducedMotion,
        true,
    );
    app.state.code.code_workspace.page = CodeWorkspacePage::Netlist;
    crate::workbench::documents::netlist_document::begin_netlist_lifecycle_action(
        &mut app.state,
        CodeSourceFileAction::New,
    )
    .unwrap();
    crate::workbench::documents::netlist_document::commit_netlist_lifecycle_action(&mut app.state)
        .unwrap();
    open_source_document_dialog(&mut app.state).unwrap();
    app
}

/// Every AccessKit node the dialog shows over three 1440x900 frames.
fn dialog_nodes(app: &mut RSpiceApp) -> Vec<egui::accesskit::Node> {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| app.render_source_document_dialog(ui.ctx()),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    nodes.into_values().collect()
}

fn node_left(nodes: &[egui::accesskit::Node], label: &str) -> f64 {
    nodes
        .iter()
        .filter(|node| node.role() != egui::accesskit::Role::Dialog)
        .find(|node| node.label() == Some(label))
        .and_then(egui::accesskit::Node::bounds)
        .unwrap_or_else(|| panic!("no {label:?} node"))
        .x0
}

/// On a project that owns no deck, New source document is a form that
/// makes one: its path, what it is made against, Create and Cancel. It
/// was Manage source document with "No top deck", every file operation
/// disabled and a Create button inside the body above a footer Close.
#[test]
fn new_source_document_on_a_deckless_project_is_a_creation_form() {
    let mut app = RSpiceApp::test_instance();
    app.state.code.code_workspace.page = CodeWorkspacePage::Netlist;
    begin_new_top_deck(&mut app.state).unwrap();
    let nodes = dialog_nodes(&mut app);
    let says = |text: &str| {
        nodes.iter().any(|node| {
            node.label().is_some_and(|label| label.contains(text))
                || node.value().is_some_and(|value| value.contains(text))
        })
    };
    assert!(
        nodes
            .iter()
            .any(|node| node.role() == egui::accesskit::Role::Dialog
                && node.label() == Some("New source document")),
        "the dialog is not titled New source document"
    );
    assert!(says("Logical path") && says("Against"));
    for gone in [
        "Manage source document",
        "Current document",
        "File operations",
        "No top deck",
        "Include search paths",
    ] {
        assert!(!says(gone), "the creation form states {gone:?}");
    }
    assert!(says("Cancel"), "the form has no Cancel");

    // Create says why it is refused while there is no path.
    app.state
        .code
        .netlist
        .lifecycle_dialog
        .transaction
        .as_mut()
        .unwrap()
        .proposed_path
        .clear();
    let nodes = dialog_nodes(&mut app);
    let create = nodes
        .iter()
        .find(|node| node.role() == egui::accesskit::Role::Button && node.label() == Some("Create"))
        .expect("the footer's Create");
    assert!(create.is_disabled());
    assert!(nodes.iter().any(|node| {
        node.label()
            .or_else(|| node.value())
            .is_some_and(|text| text.contains("Enter a logical path"))
    }));
}

/// Manage source document's rows start where its title does. Its
/// sections were panel sections inside the padded dialog body, so every
/// row stood 10 points right of the title and the footer.
#[test]
fn the_lifecycle_rows_share_the_dialog_titles_left_edge() {
    let mut app = owned_deck_app();
    let nodes = dialog_nodes(&mut app);
    // The title and the footer stand as far in from the surface's left
    // edge as the footer's last button does from its right.
    let surface = nodes
        .iter()
        .find(|node| node.role() == egui::accesskit::Role::Dialog)
        .and_then(egui::accesskit::Node::bounds)
        .expect("the dialog has bounds");
    let footer = nodes
        .iter()
        .filter(|node| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("Close")
        })
        .filter_map(egui::accesskit::Node::bounds)
        .max_by(|a, b| a.y1.total_cmp(&b.y1))
        .expect("the footer's Close");
    let title = surface.x0 + (surface.x1 - footer.x1);
    for row in ["Top deck", "Line endings"] {
        let left = node_left(&nodes, row);
        assert!(
            (left - title).abs() < 1.0,
            "{row} starts at x={left}, the title at x={title}"
        );
    }
}
