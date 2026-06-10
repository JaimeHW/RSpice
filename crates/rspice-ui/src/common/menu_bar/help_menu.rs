
use crate::common::app::{AppState, ConsoleMessage};

pub(crate) const DOC_REFERENCES: [(&str, &str); 4] = [
    ("User Guide", "docs/user_guide.md"),
    ("SPICE Reference", "docs/spice_reference.md"),
    ("Analysis Guide", "docs/analysis_guide.md"),
    ("Model Library", "docs/models.md"),
];





pub(crate) fn open_documentation_reference(state: &mut AppState, title: &str, relative_path: &str) {
    state.push_user_message(ConsoleMessage::info(format!(
        "{}: See {}",
        title, relative_path
    )));
}








