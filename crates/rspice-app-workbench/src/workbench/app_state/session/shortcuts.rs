//! Chord resolution.
//!
//! Turns a frame of raw keyboard input into at most one command: it tracks
//! partial chord prefixes across frames, scopes candidate bindings to the
//! active workspace and focus, and refuses to fire a command the current
//! context does not offer. Runtime-only — a partial sequence is never
//! persisted across sessions.

use egui::{InputState, Key, Modifiers};

use crate::state::command_catalog::{CommandCatalog, CommandKey, CommandPlatform};
use crate::state::shortcuts::ShortcutPreferences;
use crate::workbench::framework::command_registry::Registry;

mod canvas_focus;
pub mod category;
mod context_match;
mod input_snapshot;
mod platform;
pub mod resolver;

pub use canvas_focus::engineering_canvas_has_focus;
pub use canvas_focus::report_engineering_canvas_focus;
pub use canvas_focus::{
    escape_owner_was_let_go, report_escape_owner, request_engineering_canvas_focus,
    request_engineering_canvas_focus_in_this_pass,
};
pub use category::ShortcutCategory;
pub use context_match::ShortcutContextMatch;
pub use input_snapshot::ShortcutInputSnapshot;
pub use platform::runtime_command_platform;
pub use resolver::ShortcutEnvironment;
pub use resolver::ShortcutResolverState;

/// Accessibility projection of the same effective bindings consumed by the
/// runtime resolver. Empty/unsupported bindings are omitted rather than
/// announcing a key that cannot execute on the current host. `catalog` and
/// `registry` are the shell's: the commands the profile binds, and what each
/// is called.
pub fn accessibility_shortcut_summary(
    shortcuts: &ShortcutPreferences,
    catalog: &impl CommandCatalog,
    registry: &Registry,
    platform: CommandPlatform,
    operating_system: egui::os::OperatingSystem,
    commands: &[CommandKey],
) -> String {
    let entries = commands
        .iter()
        .filter_map(|command| {
            let labels = shortcuts.resolved_labels(catalog, *command, platform, operating_system);
            (!labels.is_empty()).then(|| {
                format!(
                    "{}: {}",
                    labels.join(" or "),
                    registry.label(*command).trim_end_matches('\u{2026}')
                )
            })
        })
        .collect::<Vec<_>>();
    if entries.is_empty() {
        String::new()
    } else {
        format!(" Shortcuts: {}.", entries.join("; "))
    }
}
