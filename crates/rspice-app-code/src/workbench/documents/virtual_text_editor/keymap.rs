//! Which edit or caret move a key press asks the source editor for.
//!
//! The map follows the conventions of the platform the editor runs on: on
//! macOS Command reaches a line or document edge and Option reaches a word,
//! elsewhere Control reaches a word and Home/End reach the edges. Keys the
//! map leaves alone (Alt with an arrow, for instance) belong to application
//! commands with rebindable shortcuts.

use egui::os::OperatingSystem;
use egui::{Key, Modifiers};

/// How far a horizontal move or a deletion reaches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Reach {
    Character,
    Word,
    /// The start of the line backward, its end forward.
    LineEdge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Motion {
    Backward(Reach),
    Forward(Reach),
    Up,
    Down,
    PageUp,
    PageDown,
    DocumentStart,
    DocumentEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum KeyAction {
    SelectAll,
    Undo,
    Redo,
    Copy,
    Cut,
    Move { motion: Motion, extend: bool },
    Delete { forward: bool, reach: Reach },
    Newline,
    Indent,
    Outdent,
    Escape,
    ToggleTabMovesFocus,
}

/// The chord that switches Tab between indenting and moving focus: Ctrl+M,
/// and Ctrl+Shift+M on macOS, where Ctrl+M is not free.
pub(super) fn tab_moves_focus_chord(os: OperatingSystem) -> &'static str {
    if os == OperatingSystem::Mac {
        "Ctrl+Shift+M"
    } else {
        "Ctrl+M"
    }
}

pub(super) fn key_action(os: OperatingSystem, key: Key, modifiers: Modifiers) -> Option<KeyAction> {
    let mac = os == OperatingSystem::Mac;
    let extend = modifiers.shift;
    // Command on macOS, Control elsewhere.
    let command = modifiers.command;
    let movement = |motion| Some(KeyAction::Move { motion, extend });
    // The reach of a horizontal move or a deletion from its modifiers.
    let reach = || {
        if mac {
            if command {
                Some(Reach::LineEdge)
            } else if modifiers.alt {
                Some(Reach::Word)
            } else if modifiers.ctrl {
                // Control with an arrow switches spaces on macOS.
                None
            } else {
                Some(Reach::Character)
            }
        } else if modifiers.alt {
            None
        } else if command {
            Some(Reach::Word)
        } else {
            Some(Reach::Character)
        }
    };
    match key {
        Key::A if command && !modifiers.alt => Some(KeyAction::SelectAll),
        Key::Z if command && modifiers.shift => Some(KeyAction::Redo),
        Key::Z if command => Some(KeyAction::Undo),
        Key::Y if command => Some(KeyAction::Redo),
        Key::C if command => Some(KeyAction::Copy),
        Key::X if command => Some(KeyAction::Cut),
        Key::M
            if mac && modifiers.ctrl && modifiers.shift && !modifiers.mac_cmd && !modifiers.alt =>
        {
            Some(KeyAction::ToggleTabMovesFocus)
        }
        Key::M if !mac && command && !modifiers.shift && !modifiers.alt => {
            Some(KeyAction::ToggleTabMovesFocus)
        }
        Key::ArrowLeft => movement(Motion::Backward(reach()?)),
        Key::ArrowRight => movement(Motion::Forward(reach()?)),
        // Alt with Up or Down moves lines, an application command.
        Key::ArrowUp | Key::ArrowDown if modifiers.alt => None,
        Key::ArrowUp if mac && command => movement(Motion::DocumentStart),
        Key::ArrowDown if mac && command => movement(Motion::DocumentEnd),
        Key::ArrowUp => movement(Motion::Up),
        Key::ArrowDown => movement(Motion::Down),
        Key::Home if command => movement(Motion::DocumentStart),
        Key::End if command => movement(Motion::DocumentEnd),
        Key::Home => movement(Motion::Backward(Reach::LineEdge)),
        Key::End => movement(Motion::Forward(Reach::LineEdge)),
        Key::PageUp | Key::PageDown if command || modifiers.alt => None,
        Key::PageUp => movement(Motion::PageUp),
        Key::PageDown => movement(Motion::PageDown),
        Key::Backspace => Some(KeyAction::Delete {
            forward: false,
            reach: reach()?,
        }),
        Key::Delete => Some(KeyAction::Delete {
            forward: true,
            reach: reach()?,
        }),
        Key::Enter => Some(KeyAction::Newline),
        Key::Tab if command || modifiers.alt => None,
        Key::Tab if modifiers.shift => Some(KeyAction::Outdent),
        Key::Tab => Some(KeyAction::Indent),
        Key::Escape => Some(KeyAction::Escape),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Command as macOS reports it: the physical key and the portable flag.
    const MAC_COMMAND: Modifiers = Modifiers {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: true,
        command: true,
    };
    const OPTION: Modifiers = Modifiers::ALT;

    fn moved(motion: Motion) -> Option<KeyAction> {
        Some(KeyAction::Move {
            motion,
            extend: false,
        })
    }

    #[test]
    fn macos_reaches_line_edges_with_command_and_words_with_option() {
        let mac = OperatingSystem::Mac;
        assert_eq!(
            key_action(mac, Key::ArrowLeft, MAC_COMMAND),
            moved(Motion::Backward(Reach::LineEdge))
        );
        assert_eq!(
            key_action(mac, Key::ArrowRight, MAC_COMMAND),
            moved(Motion::Forward(Reach::LineEdge))
        );
        assert_eq!(
            key_action(mac, Key::ArrowLeft, OPTION),
            moved(Motion::Backward(Reach::Word))
        );
        assert_eq!(
            key_action(mac, Key::ArrowRight, OPTION),
            moved(Motion::Forward(Reach::Word))
        );
        assert_eq!(
            key_action(mac, Key::ArrowUp, MAC_COMMAND),
            moved(Motion::DocumentStart)
        );
        assert_eq!(
            key_action(mac, Key::ArrowDown, MAC_COMMAND),
            moved(Motion::DocumentEnd)
        );
        assert_eq!(
            key_action(mac, Key::ArrowRight, MAC_COMMAND | Modifiers::SHIFT),
            Some(KeyAction::Move {
                motion: Motion::Forward(Reach::LineEdge),
                extend: true,
            })
        );
        assert_eq!(
            key_action(mac, Key::Backspace, OPTION),
            Some(KeyAction::Delete {
                forward: false,
                reach: Reach::Word,
            })
        );
        assert_eq!(
            key_action(mac, Key::Backspace, MAC_COMMAND),
            Some(KeyAction::Delete {
                forward: false,
                reach: Reach::LineEdge,
            })
        );
        assert_eq!(
            key_action(mac, Key::Delete, OPTION),
            Some(KeyAction::Delete {
                forward: true,
                reach: Reach::Word,
            })
        );
        // Control with an arrow belongs to the system on macOS.
        assert_eq!(key_action(mac, Key::ArrowLeft, Modifiers::CTRL), None);
        assert_eq!(
            key_action(mac, Key::M, Modifiers::CTRL | Modifiers::SHIFT),
            Some(KeyAction::ToggleTabMovesFocus)
        );
        assert_eq!(key_action(mac, Key::M, MAC_COMMAND), None);
    }

    #[test]
    fn windows_and_linux_reach_words_with_control() {
        for os in [OperatingSystem::Windows, OperatingSystem::Nix] {
            let control = Modifiers::COMMAND | Modifiers::CTRL;
            assert_eq!(
                key_action(os, Key::ArrowLeft, control),
                moved(Motion::Backward(Reach::Word))
            );
            assert_eq!(
                key_action(os, Key::ArrowRight, control),
                moved(Motion::Forward(Reach::Word))
            );
            assert_eq!(
                key_action(os, Key::Backspace, control),
                Some(KeyAction::Delete {
                    forward: false,
                    reach: Reach::Word,
                })
            );
            assert_eq!(
                key_action(os, Key::Delete, control),
                Some(KeyAction::Delete {
                    forward: true,
                    reach: Reach::Word,
                })
            );
            assert_eq!(
                key_action(os, Key::Home, control),
                moved(Motion::DocumentStart)
            );
            assert_eq!(
                key_action(os, Key::End, control),
                moved(Motion::DocumentEnd)
            );
            assert_eq!(
                key_action(os, Key::ArrowUp, control),
                moved(Motion::Up),
                "Control does not reach the document edge with an arrow"
            );
            assert_eq!(
                key_action(os, Key::Backspace, Modifiers::NONE),
                Some(KeyAction::Delete {
                    forward: false,
                    reach: Reach::Character,
                })
            );
            assert_eq!(
                key_action(os, Key::M, control),
                Some(KeyAction::ToggleTabMovesFocus)
            );
            assert_eq!(key_action(os, Key::ArrowUp, Modifiers::ALT), None);
        }
    }

    #[test]
    fn pages_and_tab_answer_without_modifiers_only() {
        let os = OperatingSystem::Windows;
        assert_eq!(
            key_action(os, Key::PageDown, Modifiers::SHIFT),
            Some(KeyAction::Move {
                motion: Motion::PageDown,
                extend: true,
            })
        );
        assert_eq!(key_action(os, Key::PageDown, Modifiers::COMMAND), None);
        assert_eq!(key_action(os, Key::Tab, Modifiers::COMMAND), None);
        assert_eq!(
            key_action(os, Key::Tab, Modifiers::SHIFT),
            Some(KeyAction::Outdent)
        );
    }
}
