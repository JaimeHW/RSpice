use crate::common::app::{AppState, ConsoleMessage};

pub(crate) const DOC_REFERENCES: [(&str, &str); 4] = [
    ("README", "https://github.com/JaimeHW/RSpice#readme"),
    ("Website Docs", "https://rspice.app/docs.html"),
    ("Validation", "https://rspice.app/parity.html"),
    ("Model Library", "models/README.md"),
];

pub(crate) fn open_documentation_reference(state: &mut AppState, title: &str, relative_path: &str) {
    state.push_user_message(ConsoleMessage::info(format!(
        "{}: See {}",
        title, relative_path
    )));
}
