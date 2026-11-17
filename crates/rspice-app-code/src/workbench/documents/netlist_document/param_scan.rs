//! Character runs the highlighter reads a card by.
//!
//! What a deck defines and reads comes from the engine's index of it; these
//! only find where a run of characters or a `{...}` expression ends.

/// End of the run of characters from byte `from` that `keep` accepts: the
/// byte offset of the first character it refuses, or the end of the text.
pub(super) fn run_end(text: &str, from: usize, keep: impl Fn(char) -> bool) -> usize {
    text[from..]
        .find(|ch: char| !keep(ch))
        .map_or(text.len(), |offset| from + offset)
}

/// End of the `{expression}` that opens at byte `start`: just past its
/// matching brace, or the end of the text when it never closes.
pub(super) fn expression_end(text: &str, start: usize) -> usize {
    let mut depth = 0i32;
    for (offset, ch) in text[start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return start + offset + 1;
                }
            }
            _ => {}
        }
    }
    text.len()
}
