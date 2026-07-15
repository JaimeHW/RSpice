use egui::{InputState, Key, Modifiers};

mod category;
mod input_snapshot;
mod resolver;

pub(super) use category::ShortcutCategory;
pub(super) use input_snapshot::ShortcutInputSnapshot;
pub(super) use resolver::resolve_shortcuts;
