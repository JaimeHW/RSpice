//! The menus' access keys on the Windows and Linux desktop: Alt and a menu's
//! letter opens it, a tap of Alt takes the bar, a row's letter chooses it,
//! and every row a menu shows has a letter of its own.

use super::*;
use crate::state::GridStyle;
use crate::state::command_catalog::CommandPlatform;
use crate::workbench::chrome::title_bar::application_menus::{access_key_index, access_keys_apply};
use egui::os::OperatingSystem;

/// Rows that go without an access key, each because its menu has no letter
/// left to give it (see the menu's own note): a command's row by its id, a
/// submenu's by its label.
const KEYLESS_ROWS: [&str; 4] = ["save-all", "live-session", "deselect-all", "Select"];

fn key_press(key: Key, modifiers: Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers,
    }
}

/// A window on the Windows desktop drawing the full menu bar, which first
/// runs the key dispatch the way a frame does.
struct Bar {
    ctx: Context,
    app: RSpiceApp,
}

impl Bar {
    fn new(os: OperatingSystem) -> Self {
        let ctx = Context::default();
        ctx.set_os(os);
        crate::ui::Theme::default().apply(&ctx);
        Self {
            ctx,
            app: title_test_app(),
        }
    }

    fn frame(&mut self, events: Vec<egui::Event>, modifiers: Modifiers) -> egui::FullOutput {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_440.0, 900.0),
            )),
            events,
            modifiers,
            ..Default::default()
        };
        let app = &mut self.app;
        self.ctx.run_ui(input, |ui| {
            app.handle_shortcuts(ui.ctx());
            egui::CentralPanel::default().show(ui, |ui| {
                menus(ui, app, MenuProjection::All, false, 35.0);
            });
        })
    }

    /// Alt held for a pass, then let go.
    fn tap_alt(&mut self) {
        self.frame(Vec::new(), Modifiers::ALT);
        self.frame(Vec::new(), Modifiers::NONE);
    }

    fn press(&mut self, key: Key, modifiers: Modifiers) {
        self.frame(vec![key_press(key, modifiers)], modifiers);
        self.frame(Vec::new(), Modifiers::NONE);
    }

    fn settle(&mut self) -> egui::FullOutput {
        self.frame(Vec::new(), Modifiers::NONE);
        self.frame(Vec::new(), Modifiers::NONE)
    }

    fn is_open(&self, menu: ApplicationMenu) -> bool {
        Popup::is_id_open(&self.ctx, trigger_id(menu).with("popup"))
    }

    fn focused_trigger(&self) -> Option<ApplicationMenu> {
        let focused = self.ctx.memory(|memory| memory.focused())?;
        ALL_MENUS
            .into_iter()
            .find(|&menu| trigger_id(menu) == focused)
    }
}

fn trigger_id(menu: ApplicationMenu) -> Id {
    Id::new((
        "workbench.title_menu.trigger",
        MenuTriggerKey::Application(menu),
    ))
}

/// The rules a reader can see drawn across the bar's trigger for `menu`.
fn rules_under(bar: &Bar, output: &egui::FullOutput, menu: ApplicationMenu) -> Vec<Rect> {
    fn collect(shape: &egui::Shape, out: &mut Vec<Rect>) {
        match shape {
            egui::Shape::LineSegment { points, .. } if (points[0].y - points[1].y).abs() < 0.01 => {
                out.push(Rect::from_two_pos(points[0], points[1]));
            }
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| collect(shape, out)),
            _ => {}
        }
    }
    let trigger = bar
        .ctx
        .read_response(trigger_id(menu))
        .expect("the trigger is drawn")
        .rect;
    let mut rules = Vec::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut rules);
    }
    rules.retain(|rule| trigger.contains_rect(*rule));
    rules
}

#[test]
fn access_keys_belong_to_the_windows_and_linux_desktop() {
    use OperatingSystem::{Mac, Nix, Windows};
    assert!(access_keys_apply(CommandPlatform::Desktop, Windows));
    assert!(access_keys_apply(CommandPlatform::Desktop, Nix));
    // A macOS menu has none, and the browser keeps Alt and a letter for its
    // own menus.
    assert!(!access_keys_apply(CommandPlatform::Desktop, Mac));
    for os in [Windows, Nix, Mac] {
        assert!(!access_keys_apply(CommandPlatform::Browser, os));
        assert!(!access_keys_apply(CommandPlatform::Tablet, os));
        assert!(!access_keys_apply(CommandPlatform::Phone, os));
    }
}

#[test]
fn an_access_key_is_underlined_at_a_word_start_where_it_can_be() {
    assert_eq!(access_key_index("Save as…", 'A'), Some(5));
    assert_eq!(access_key_index("Exit", 'x'), Some(1));
    assert_eq!(access_key_index("Pole-zero", 'z'), Some(5));
    assert_eq!(access_key_index("Verify", 'Y'), Some(5));
    assert_eq!(access_key_index("Close", 'Q'), None);
}

/// Every row each menu shows, in every context that changes what it shows,
/// has a letter of its own among the rows beside it, and the letter is one
/// of its label's, so it can be underlined.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_menu_row_has_its_own_access_key_in_every_context() {
    fn check(rows: &[menu_entries::MenuRow], place: &str, problems: &mut Vec<String>) {
        let mut taken: Vec<(char, String)> = Vec::new();
        for row in rows {
            let (label, access, id) = match row {
                menu_entries::MenuRow::Command(row) => {
                    if row.action.is_some() {
                        // A recent file's row: its name keeps no letter.
                        continue;
                    }
                    (row.label.to_string(), row.access, row.command.as_str())
                }
                menu_entries::MenuRow::Submenu {
                    label,
                    rows,
                    access,
                } => {
                    check(rows, &format!("{place} > {label}"), problems);
                    ((*label).to_owned(), *access, *label)
                }
                menu_entries::MenuRow::Separator => continue,
            };
            let Some(access) = access else {
                if !KEYLESS_ROWS.contains(&id) {
                    problems.push(format!("{place}: {label} has no access key"));
                }
                continue;
            };
            if access_key_index(&label, access).is_none() {
                problems.push(format!("{place}: {label} has no letter {access}"));
            }
            let access = access.to_ascii_uppercase();
            if let Some((_, other)) = taken.iter().find(|(key, _)| *key == access) {
                problems.push(format!("{place}: {label} and {other} share {access}"));
            }
            taken.push((access, label));
        }
    }
    let apps = [
        ("a drawing", title_test_app()),
        ("a source editor", source_editor_app()),
        ("a symbol", symbol_view_app()),
        ("a Verilog-A view", veriloga_view_app()),
        ("no project", pins::no_project_app()),
    ];
    let mut problems = Vec::new();
    for (context, app) in &apps {
        for menu in ALL_MENUS {
            let rows = menu_entries::resolve_menu_rows(
                application_menu_entries(menu),
                app,
                CommandPlatform::Desktop,
                OperatingSystem::Windows,
            );
            check(
                &rows,
                &format!("{} with {context}", menu.label()),
                &mut problems,
            );
        }
        for (name, entries) in [
            ("the compact Menu", app.compact_menu()),
            ("the source editor's menu", app.source_editor_menu()),
        ] {
            let rows = menu_entries::resolve_menu_rows(
                entries,
                app,
                CommandPlatform::Desktop,
                OperatingSystem::Windows,
            );
            check(&rows, &format!("{name} with {context}"), &mut problems);
        }
    }
    let mut bar: Vec<char> = Vec::new();
    for menu in ALL_MENUS {
        let key = menu.access_key();
        if access_key_index(menu.label(), key).is_none() || bar.contains(&key) {
            problems.push(format!("the bar's {} cannot take {key}", menu.label()));
        }
        bar.push(key);
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// Alt and a menu's letter opens that menu with its first row focused, from
/// anywhere; on macOS it opens nothing.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn alt_and_a_menus_letter_opens_it() {
    for menu in ALL_MENUS {
        let mut bar = Bar::new(OperatingSystem::Windows);
        bar.settle();
        let key = Key::from_name(&menu.access_key().to_string()).expect("a letter key");
        bar.press(key, Modifiers::ALT);
        bar.settle();
        assert!(
            bar.is_open(menu),
            "Alt+{} opens {}",
            menu.access_key(),
            menu.label()
        );
        assert!(
            ALL_MENUS
                .into_iter()
                .filter(|&other| other != menu)
                .all(|other| !bar.is_open(other)),
            "Alt+{} opens {} alone",
            menu.access_key(),
            menu.label()
        );
    }
    let mut mac = Bar::new(OperatingSystem::Mac);
    mac.settle();
    mac.press(Key::F, Modifiers::ALT);
    mac.settle();
    assert!(!mac.is_open(ApplicationMenu::File));
}

/// A tap of Alt takes the focus to the bar and underlines each menu's
/// letter; another gives the focus back and takes the underlines away.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_tap_of_alt_takes_the_bar_and_underlines_its_letters() {
    let mut bar = Bar::new(OperatingSystem::Windows);
    let before = bar.settle();
    assert!(rules_under(&bar, &before, ApplicationMenu::File).is_empty());

    bar.tap_alt();
    assert_eq!(bar.focused_trigger(), Some(ApplicationMenu::File));
    let tapped = bar.settle();
    for menu in ALL_MENUS {
        let rules = rules_under(&bar, &tapped, menu);
        assert_eq!(rules.len(), 1, "{} underlines its letter", menu.label());
        assert!(
            (3.0..12.0).contains(&rules[0].width()),
            "{}'s underline spans one letter, not {} px",
            menu.label(),
            rules[0].width()
        );
    }
    // The letter of a focused bar opens its menu, as Alt and the letter
    // does.
    bar.press(Key::V, Modifiers::NONE);
    bar.settle();
    assert!(bar.is_open(ApplicationMenu::View));

    bar.tap_alt();
    let after = bar.settle();
    assert!(!bar.is_open(ApplicationMenu::View));
    assert_eq!(bar.focused_trigger(), None);
    assert!(rules_under(&bar, &after, ApplicationMenu::File).is_empty());
}

/// Alt held through a digit (the workspaces' keys), a click or a drag (the
/// canvas pans with Alt) is no tap: the bar stays where it was, and Alt+2
/// still opens Design.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn alt_with_a_digit_or_a_drag_is_no_tap() {
    let mut bar = Bar::new(OperatingSystem::Windows);
    bar.settle();
    bar.frame(vec![key_press(Key::Num2, Modifiers::ALT)], Modifiers::ALT);
    bar.frame(Vec::new(), Modifiers::ALT);
    bar.frame(Vec::new(), Modifiers::NONE);
    bar.settle();
    assert_eq!(bar.app.state.workbench.workspace, Workspace::Design);
    assert_eq!(bar.focused_trigger(), None);
    assert!(ALL_MENUS.into_iter().all(|menu| !bar.is_open(menu)));

    let at = egui::pos2(700.0, 500.0);
    bar.frame(vec![egui::Event::PointerMoved(at)], Modifiers::ALT);
    bar.frame(
        vec![egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: Modifiers::ALT,
        }],
        Modifiers::ALT,
    );
    bar.frame(
        vec![egui::Event::PointerMoved(at + egui::vec2(40.0, 0.0))],
        Modifiers::ALT,
    );
    bar.frame(
        vec![egui::Event::PointerButton {
            pos: at + egui::vec2(40.0, 0.0),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: Modifiers::ALT,
        }],
        Modifiers::ALT,
    );
    bar.frame(Vec::new(), Modifiers::NONE);
    bar.settle();
    assert_eq!(bar.focused_trigger(), None);
}

/// With a menu open, a row's letter chooses it, and a submenu's letter
/// opens the submenu, whose own letters then choose among its rows.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_rows_letter_chooses_it_in_an_open_menu() {
    let mut bar = Bar::new(OperatingSystem::Windows);
    bar.settle();
    let navigator = Command::ToggleNavigator.key();
    let shown = bar.app.checked(navigator);
    bar.press(Key::V, Modifiers::ALT);
    bar.settle();
    bar.press(Key::N, Modifiers::NONE);
    bar.settle();
    assert_ne!(bar.app.checked(navigator), shown, "View > Navigator ran");
    assert!(!bar.is_open(ApplicationMenu::View), "choosing closed View");

    assert_eq!(bar.app.state.design.grid, GridStyle::Dots);
    bar.press(Key::V, Modifiers::ALT);
    bar.settle();
    bar.press(Key::G, Modifiers::NONE);
    bar.settle();
    assert!(
        bar.is_open(ApplicationMenu::View),
        "Grid opened inside View"
    );
    bar.press(Key::L, Modifiers::NONE);
    bar.settle();
    assert_eq!(
        bar.app.state.design.grid,
        GridStyle::Lines,
        "View > Grid > Lines ran"
    );
    assert!(!bar.is_open(ApplicationMenu::View));
}
