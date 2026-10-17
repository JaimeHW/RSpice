//! What a Console message carries when it leaves the reader's screen.
//!
//! The support bundle and the diagnostic folder's session log both write the
//! Console's messages to a file someone else may read, so both pass them
//! through one function: the reader's home directory becomes `<HOME>`, and a
//! message is bounded in length.

/// The longest message text kept, in bytes.
const MAX_MESSAGE_BYTES: usize = 2_048;

/// `message` as it may be shared: `home` replaced by `<HOME>`, and the text
/// cut at [`MAX_MESSAGE_BYTES`] on a character boundary.
pub(crate) fn redact_log_message(message: &str, home: Option<&str>) -> String {
    let mut redacted = home.filter(|home| !home.is_empty()).map_or_else(
        || message.to_owned(),
        |home| message.replace(home, "<HOME>"),
    );
    if redacted.len() > MAX_MESSAGE_BYTES {
        let mut boundary = MAX_MESSAGE_BYTES;
        while !redacted.is_char_boundary(boundary) {
            boundary -= 1;
        }
        redacted.truncate(boundary);
        redacted.push_str("\u{2026}<truncated>");
    }
    redacted
}

/// The reader's home directory, as the text [`redact_log_message`] replaces.
pub(crate) fn home_directory() -> Option<String> {
    dirs::home_dir().map(|path| path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_message_loses_the_home_directory_and_keeps_utf8_boundaries_when_cut() {
        let text = format!(r"C:\Users\James\project\deck.sp {}", "μ".repeat(2_100));
        let redacted = redact_log_message(&text, Some(r"C:\Users\James"));
        assert!(redacted.starts_with(r"<HOME>\project\deck.sp"));
        assert!(redacted.ends_with("\u{2026}<truncated>"));
        assert!(redacted.is_char_boundary(redacted.len()));
    }
}
