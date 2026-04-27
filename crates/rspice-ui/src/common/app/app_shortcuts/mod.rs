use egui::{InputState, Key, Modifiers};

mod category;
mod command;
mod input_snapshot;
mod resolver;

pub(super) use category::ShortcutCategory;
pub(super) use command::ShortcutCommand;
pub(super) use input_snapshot::ShortcutInputSnapshot;
pub(super) use resolver::collect_shortcut_commands;
