//! Names and descriptive text for a MATLAB v5 file that RSpice can reopen.

use std::collections::HashSet;

use super::{HEADER_SIGNATURE, HEADER_TEXT_BYTES, MAX_NAME_CHARS};

/// What a header text says when it is cut to fit its field.
pub const ELLIPSIS: &str = "...";

/// A MATLAB identifier built from a source name.
///
/// A letter first, then letters, digits and underscores, at most
/// `namelengthmax` characters. Everything else becomes an underscore.
pub fn matlab_identifier(source: &str) -> String {
    let mut name = String::with_capacity(source.len());
    for character in source.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            name.push(character);
        } else {
            name.push('_');
        }
    }
    if !name.starts_with(|character: char| character.is_ascii_alphabetic()) {
        name.insert(0, 'x');
    }
    // Every character is ASCII by now, so this is a character count.
    name.truncate(MAX_NAME_CHARS);
    name
}

/// Assigns distinct MATLAB variable names, reserving the coordinate first.
///
/// Comparison is case-insensitive because RSpice's importer refuses two
/// signals whose names differ only in case, although MATLAB accepts them.
pub struct MatNameAllocator {
    taken: HashSet<String>,
}

impl MatNameAllocator {
    pub fn new(coordinate_name: &str, signal_count: usize) -> Self {
        let mut taken = HashSet::with_capacity(signal_count.saturating_add(1));
        taken.insert(coordinate_name.to_ascii_lowercase());
        Self { taken }
    }

    /// Return a valid name, with the first available numeric suffix if needed.
    pub fn allocate(&mut self, source: &str) -> String {
        let candidate = matlab_identifier(source);
        if self.taken.insert(candidate.to_ascii_lowercase()) {
            return candidate;
        }
        let mut suffix = 2_u32;
        loop {
            let tail = format!("_{suffix}");
            let stem = &candidate[..candidate.len().min(MAX_NAME_CHARS - tail.len())];
            let name = format!("{stem}{tail}");
            if self.taken.insert(name.to_ascii_lowercase()) {
                return name;
            }
            suffix += 1;
        }
    }
}

/// The dataset's creation time as UTC, independent of the export wall clock.
pub fn created_on(timestamp: f64) -> String {
    let seconds = timestamp.trunc();
    if !seconds.is_finite() || seconds < i64::MIN as f64 || seconds > i64::MAX as f64 {
        return "unstated".to_owned();
    }
    let Ok(stamp) = time::OffsetDateTime::from_unix_timestamp(seconds as i64) else {
        return "unstated".to_owned();
    };
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        stamp.year(),
        u8::from(stamp.month()),
        stamp.day(),
        stamp.hour(),
        stamp.minute(),
        stamp.second()
    )
}

/// One provenance entry: the published variable, its source name, and unit.
pub fn note_entry(variable: &str, source: &str, unit: Option<&str>) -> String {
    let mut entry = variable.to_owned();
    if variable != source {
        entry.push_str(" = ");
        entry.push_str(source);
    }
    if let Some(unit) = unit {
        entry.push_str(" in ");
        entry.push_str(unit);
    }
    entry
}

/// The header's descriptive text and whether its provenance note was cut.
///
/// The signature comes first because RSpice's importer identifies the file by it.
pub fn header_text(created_on: &str, note: &str) -> (String, bool) {
    let mut text =
        format!("{HEADER_SIGNATURE}, Platform: RSpice, Created on: {created_on}; {note}");
    if text.len() <= HEADER_TEXT_BYTES {
        return (text, false);
    }
    let mut kept = HEADER_TEXT_BYTES - ELLIPSIS.len();
    while !text.is_char_boundary(kept) {
        kept -= 1;
    }
    text.truncate(kept);
    text.push_str(ELLIPSIS);
    (text, true)
}
