//! The menu bar on the macOS desktop: the brand opens the RSpice menu, which
//! holds About, Check for updates, Settings and Quit in place of Help, Edit
//! and File; Window
//! begins with Minimize and Zoom; and every key reads the way an Apple menu
//! writes it. Read for the macOS platform value on any host.

use super::*;
use crate::state::command_catalog::CommandPlatform;
use crate::workbench::framework::requests::WindowFrame;
use egui::os::OperatingSystem;

/// `rows` without their key: `label | command id | state`.
fn without_keys(rows: &[String]) -> Vec<String> {
    rows.iter()
        .map(|row| {
            let mut parts = row.split(" | ").collect::<Vec<_>>();
            if parts.len() > 2 {
                parts.remove(2);
            }
            parts.join(" | ")
        })
        .collect()
}

/// The key a pinned row shows.
fn key_of(row: &str) -> &str {
    row.split(" | ").nth(2).unwrap_or("")
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_rspice_menu_holds_about_updates_settings_and_quit_on_the_macos_desktop() {
    let app = title_test_app();
    assert_eq!(
        menu_pin_on(app.application_menu(), &app, OperatingSystem::Mac),
        [
            "About RSpice | about |  | on",
            "Check for updates\u{2026} | check-for-updates |  | on",
            "---",
            "Settings\u{2026} | preferences | \u{2318}, | on",
            "---",
            "Quit RSpice | exit-rspice | \u{2318}Q | on",
        ]
    );
}

/// On the macOS desktop About and Check for updates leave Help, Preferences
/// leaves Edit and Exit leaves File, and Window begins with the window
/// itself; every other menu holds what it holds on Windows.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_macos_menus_give_the_rspice_menu_its_rows_and_window_its_own() {
    let app = title_test_app();
    let updates = "Check for updates\u{2026} | check-for-updates | on";
    for menu in ALL_MENUS {
        let entries = application_menu_entries(menu);
        let mut windows = without_keys(&menu_pin_on(entries, &app, OperatingSystem::Windows));
        let mac = without_keys(&menu_pin_on(entries, &app, OperatingSystem::Mac));
        if menu == ApplicationMenu::Help {
            assert!(
                windows.iter().any(|row| row == updates),
                "Help checks for updates on Windows: {windows:#?}"
            );
            windows.retain(|row| row != updates);
        } else {
            assert!(
                !windows.iter().any(|row| row == updates),
                "{}",
                menu.label()
            );
        }
        let (head, tail): (&[&str], &[&str]) = match menu {
            ApplicationMenu::File => (&[], &["---", "Exit | exit-rspice | on"]),
            ApplicationMenu::Edit => (&[], &["---", "Preferences\u{2026} | preferences | on"]),
            ApplicationMenu::Help => (&[], &["About RSpice | about | on"]),
            ApplicationMenu::Window => (
                &[
                    "Minimize | minimize-window | on",
                    "Zoom | zoom-window | on",
                    "---",
                ],
                &[],
            ),
            _ => (&[], &[]),
        };
        assert!(
            windows.ends_with(&tail.iter().map(|row| (*row).to_owned()).collect::<Vec<_>>()),
            "{}: {windows:#?}",
            menu.label()
        );
        let expected = head
            .iter()
            .map(|row| (*row).to_owned())
            .chain(windows[..windows.len() - tail.len()].iter().cloned())
            .collect::<Vec<_>>();
        assert_eq!(mac, expected, "{}", menu.label());
    }
}

/// The browser has its own application menu even on a Mac, so the page keeps
/// About in Help and Preferences in Edit, and has no window to minimize. It is
/// always served the current version, so no menu checks for updates.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_browser_on_a_mac_keeps_about_and_preferences_and_has_no_window_rows() {
    let app = title_test_app();
    for os in [OperatingSystem::Mac, OperatingSystem::Windows] {
        for menu in ALL_MENUS {
            let rows = menu_pin_for(
                application_menu_entries(menu),
                &app,
                CommandPlatform::Browser,
                os,
            );
            assert!(
                rows.iter().all(|row| !row.contains("check-for-updates")),
                "{os:?} {}: {rows:#?}",
                menu.label()
            );
        }
    }
    let rows = |menu| {
        menu_pin_for(
            application_menu_entries(menu),
            &app,
            CommandPlatform::Browser,
            OperatingSystem::Mac,
        )
    };
    assert!(
        rows(ApplicationMenu::Help)
            .iter()
            .any(|row| row.starts_with("About RSpice | about |")),
        "{:#?}",
        rows(ApplicationMenu::Help)
    );
    assert!(
        rows(ApplicationMenu::Edit)
            .iter()
            .any(|row| row.starts_with("Preferences\u{2026} | preferences | \u{2318}, |")),
        "{:#?}",
        rows(ApplicationMenu::Edit)
    );
    let window = rows(ApplicationMenu::Window);
    assert!(
        window
            .iter()
            .all(|row| !row.contains("minimize-window") && !row.contains("zoom-window")),
        "{window:#?}"
    );
}

/// Every key a macOS menu shows is written in glyphs, the modifiers in the
/// order Apple sets (Control, Option, Shift, Command) and the key straight
/// after them.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn macos_menus_write_keys_as_glyphs_in_apples_order() {
    const ORDER: [char; 4] = ['\u{2303}', '\u{2325}', '\u{21E7}', '\u{2318}'];
    let app = title_test_app();
    let mut rows = menu_pin_on(app.application_menu(), &app, OperatingSystem::Mac);
    for menu in ALL_MENUS {
        rows.extend(menu_pin_on(
            application_menu_entries(menu),
            &app,
            OperatingSystem::Mac,
        ));
    }
    for row in &rows {
        let key = key_of(row);
        for word in ["Ctrl", "Cmd", "Alt", "Shift", "Option"] {
            assert!(!key.contains(word), "{row}");
        }
        let modifiers = key
            .chars()
            .take_while(|glyph| ORDER.contains(glyph))
            .map(|glyph| ORDER.iter().position(|&order| order == glyph))
            .collect::<Vec<_>>();
        assert!(modifiers.windows(2).all(|pair| pair[0] < pair[1]), "{row}");
        assert!(
            !key.chars()
                .skip(modifiers.len())
                .any(|glyph| ORDER.contains(&glyph)),
            "{row}"
        );
    }
    for expected in [
        "New project\u{2026} | new-project | \u{21E7}\u{2318}N | on",
        "Save as\u{2026} | save-project-as | \u{2325}\u{21E7}\u{2318}S | on",
        "Undo | undo | \u{2318}Z | off: nothing to undo",
        "  Project | project | \u{2325}1 | on",
        "Previous document | previous-document | \u{21E7}\u{2318}\u{21E5} | off: only one document is open",
    ] {
        assert!(
            rows.iter().any(|row| row == expected),
            "{expected}\n{rows:#?}"
        );
    }
}

/// A window drawing the whole title bar on `os`'s desktop.
struct TitleBar {
    ctx: Context,
    app: RSpiceApp,
}

impl TitleBar {
    fn new(os: OperatingSystem) -> Self {
        let ctx = Context::default();
        ctx.set_os(os);
        crate::ui::Theme::default().apply(&ctx);
        Self {
            ctx,
            app: title_test_app(),
        }
    }

    fn frame(&mut self, events: Vec<egui::Event>) {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_440.0, 900.0),
            )),
            events,
            ..Default::default()
        };
        let app = &mut self.app;
        let _ = self.ctx.run_ui(input, |ui| {
            let layout = LayoutSpec::resolve(1_440.0, 900.0, &app.state.workbench);
            show(ui, app, layout);
        });
    }

    fn press(&mut self, key: Key) {
        self.frame(vec![egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }]);
        self.frame(Vec::new());
    }
}

fn trigger_id(key: MenuTriggerKey) -> Id {
    Id::new(("workbench.title_menu.trigger", key))
}

/// On the macOS desktop the brand is the RSpice menu's trigger and the first
/// the bar's keys reach: F10 takes the focus to it and the down arrow opens
/// it. On Windows the brand is no menu and F10 lands on File.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn f10_reaches_the_rspice_menu_first_on_macos_and_file_elsewhere() {
    let mut bar = TitleBar::new(OperatingSystem::Mac);
    bar.frame(Vec::new());
    bar.press(Key::F10);
    assert_eq!(
        bar.ctx.memory(|memory| memory.focused()),
        Some(trigger_id(MenuTriggerKey::RSpice))
    );
    bar.press(Key::ArrowDown);
    assert!(Popup::is_id_open(
        &bar.ctx,
        trigger_id(MenuTriggerKey::RSpice).with("popup")
    ));

    let mut bar = TitleBar::new(OperatingSystem::Windows);
    bar.frame(Vec::new());
    bar.press(Key::F10);
    assert_eq!(
        bar.ctx.memory(|memory| memory.focused()),
        Some(trigger_id(MenuTriggerKey::Application(
            ApplicationMenu::File
        )))
    );
    assert!(
        bar.ctx
            .read_response(trigger_id(MenuTriggerKey::RSpice))
            .is_none()
    );
}

/// Minimize and Zoom ask the platform's window to go to the Dock and to fill
/// the screen.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn minimize_and_zoom_ask_the_window_to_change() {
    let mut app = title_test_app();
    Command::MinimizeWindow.execute(&mut app);
    assert_eq!(
        app.state.workbench.outbox.take::<WindowFrame>(),
        Some(WindowFrame::Minimize)
    );
    Command::ZoomWindow.execute(&mut app);
    assert_eq!(
        app.state.workbench.outbox.take::<WindowFrame>(),
        Some(WindowFrame::Zoom)
    );
}
