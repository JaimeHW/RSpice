//! The Automation page's source editor menu, opened and closed in whole
//! frames of the application.
//!
//! The page's context holds the menu's answers from the frame that opens the
//! menu until it closes, and then only the three the code feature reads on
//! every frame.

use egui::accesskit::Role;
use egui::{Key, Modifiers};

use crate::feature_tests::replay::{Replay, controls};
use crate::state::{ProjectSourceLanguage, ProjectSourceOwner};
use crate::workbench::code::CodeFeature;
use crate::workbench::documents::code_workspace::{self, CodeWorkspacePage};
use crate::workbench::documents::virtual_text_editor::context_menu_may_show;
use crate::workbench::features;
use crate::workbench::framework::feature::{Cx, Feature};
use crate::workbench::state::Workspace;

/// The id of the editor the Automation page shows the workspace's entry
/// script in.
fn entry_editor(replay: &Replay) -> egui::Id {
    let bundle = replay
        .app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
        .expect("the project has an Automation workspace");
    code_workspace::source_editor_id(
        ProjectSourceLanguage::RSpiceAutomation,
        bundle.id(),
        bundle.root().logical_path(),
    )
}

/// A session with a project open, on the Automation page, with the keyboard
/// on the entry script's editor as a screen reader puts it there.
fn automation_with_focused_editor() -> Replay {
    let mut replay = Replay::new();
    replay.app.state.workbench.activate(Workspace::Netlist);
    replay.app.state.code.code_workspace.page = CodeWorkspacePage::Automation;
    replay.frames(Vec::new());
    let editor = entry_editor(&replay);
    replay
        .ctx()
        .memory_mut(|memory| memory.request_focus(editor));
    replay.frame(Vec::new());
    assert!(
        replay.ctx().memory(|memory| memory.has_focus(editor)),
        "the editor has the keyboard"
    );
    replay
}

/// Whether the frame that put out `output` drew the source editor's menu.
fn draws_the_menu(output: &egui::FullOutput) -> bool {
    let tree = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("a frame with AccessKit on publishes its tree");
    controls(tree)
        .iter()
        .any(|control| control.role == Role::MenuItem && control.label == "Toggle line comment")
}

/// Whether the editor's menu may show, as the shell reads it before it
/// lends the page its context.
fn menu_may_show(replay: &Replay) -> bool {
    context_menu_may_show(replay.ctx(), entry_editor(replay))
}

/// The page's context as the shell lends it: with the menu's answers while
/// the menu may show.
fn page_context(replay: &mut Replay) -> Cx<'_, CodeFeature> {
    let ctx = replay.ctx().clone();
    let editor = entry_editor(replay);
    features::cx_with_menu_answers(&mut replay.app, |_| context_menu_may_show(&ctx, editor))
}

/// `key` pressed with `modifiers`, then released, in one frame each; what
/// the frame of the press put out.
fn press(replay: &mut Replay, key: Key, modifiers: Modifiers) -> egui::FullOutput {
    let event = |pressed| egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed,
        repeat: false,
        modifiers,
    };
    let output = replay.frame(vec![event(true)]);
    replay.frame(vec![event(false)]);
    output
}

/// Shift+F10 in the Automation page's editor draws the menu in the frame of
/// the key, and the page's context then holds an answer for every row of the
/// menu, each the one the dispatcher gives. Once Escape has closed the menu,
/// the context holds only the three answers the page reads every frame.
#[test]
fn the_automation_page_holds_its_menus_answers_only_while_the_menu_is_open() {
    let mut replay = automation_with_focused_editor();
    assert!(!menu_may_show(&replay));
    assert!(
        draws_the_menu(&press(&mut replay, Key::F10, Modifiers::SHIFT)),
        "the frame of Shift+F10 draws the menu"
    );
    assert!(menu_may_show(&replay), "the shell reads the menu as open");
    let expected: Vec<_> = CodeFeature::MENU_AVAILABILITY
        .iter()
        .map(|key| (*key, replay.app.command_availability(*key)))
        .collect();
    {
        let cx = page_context(&mut replay);
        assert_eq!(
            cx.foreign_answers_taken(),
            CodeFeature::FOREIGN_AVAILABILITY.len() + expected.len(),
            "the page's context holds the menu's answers"
        );
        for (key, answer) in expected {
            assert_eq!(cx.foreign_availability(key), answer, "{key:?}");
        }
    }
    press(&mut replay, Key::Escape, Modifiers::NONE);
    assert!(
        !draws_the_menu(&replay.frame(Vec::new())),
        "Escape closes the menu"
    );
    assert!(
        !menu_may_show(&replay),
        "the shell reads the menu as closed"
    );
    assert_eq!(
        page_context(&mut replay).foreign_answers_taken(),
        CodeFeature::FOREIGN_AVAILABILITY.len(),
        "the page's context holds the three answers it reads every frame"
    );
}
