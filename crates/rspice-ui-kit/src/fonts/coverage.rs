//! Whether a character has a face to letter it: one this build embeds, one
//! installed on the machine, or none.
//!
//! An embedded face answers at once. Whether an installed face has a
//! character is the system-font worker's answer ([`rspice_fonts::system`]),
//! asked for once through the plugin [`super::system`] attaches and kept for
//! the life of the process; until it comes the character is unsettled, and
//! nothing is to be said of it yet. A browser reads no installed faces, so
//! what no embedded face has is missing there.

/// What letters a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coverage {
    /// An embedded or an installed face has it, or it draws nothing.
    Lettered,
    /// No face this build letters in has it: the screen and print draw the
    /// replacement square in its place.
    Missing,
    /// Not known yet: an installed face may have it.
    Unsettled,
}

/// Whether `character` has a face to letter it.
#[must_use]
pub fn coverage(character: char) -> Coverage {
    if !rspice_fonts::needs_a_glyph(character) || rspice_fonts::embedded_covers(character) {
        return Coverage::Lettered;
    }
    installed(character)
}

#[cfg(not(target_arch = "wasm32"))]
fn installed(character: char) -> Coverage {
    match rspice_fonts::system::installed_has(character) {
        Some(true) => Coverage::Lettered,
        Some(false) => Coverage::Missing,
        None => Coverage::Unsettled,
    }
}

#[cfg(target_arch = "wasm32")]
const fn installed(_character: char) -> Coverage {
    Coverage::Missing
}

/// Asks whether an installed face has each of `characters` that is
/// unsettled. The answer comes on a later frame, once the plugin hands the
/// question to its worker, and [`changes`] moves when it does.
pub fn ask(characters: impl IntoIterator<Item = char>) {
    #[cfg(not(target_arch = "wasm32"))]
    rspice_fonts::system::ask(
        characters
            .into_iter()
            .filter(|&character| coverage(character) == Coverage::Unsettled),
    );
    #[cfg(target_arch = "wasm32")]
    let _ = characters;
}

/// A count that moves whenever a character's coverage is settled, so a
/// reader of [`coverage`] knows when to read again.
#[must_use]
pub fn changes() -> u64 {
    #[cfg(not(target_arch = "wasm32"))]
    {
        rspice_fonts::system::settled_changes()
    }
    #[cfg(target_arch = "wasm32")]
    {
        0
    }
}
