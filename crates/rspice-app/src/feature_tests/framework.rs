//! The workbench's own containers, observed through a whole application: the
//! chrome, the docks, the palette and the framework's dialogs, drawn for the
//! shell's host or handed its session; and the order the shell's frame runs
//! them in, read from its source.

mod call_order;
mod command_palette;
mod confirmation;
mod console;
mod console_log_anchor;
mod console_problems;
mod console_transcript;
mod docks;
mod document_bar;
mod drawers;
mod inspector;
mod measurement_rows;
mod notification_center;
mod shortcuts;
mod shortcuts_category;
mod shortcuts_resolver;
mod status_bar;
mod surfaces;
mod title_bar;
mod toolbar;
mod toolbar_overflow;
mod window_session;
