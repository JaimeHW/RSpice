//! What each line of the deck was written for, recorded as it is written.

use super::*;

/// What one line of the generated deck was written for.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WrittenLine {
    /// The cell view whose netlist the line belongs to: the design's own, or
    /// the master's whose `.subckt` body holds it. `None` for a line no cell
    /// view writes, such as the deck's analysis cards.
    pub cell_view: Option<CellViewRef>,
    /// The schematic component in that cell view the line is the card of.
    pub component: Option<u64>,
    /// The element the card writes, as the deck names it (`R1`, `XAMP`).
    pub element: Option<String>,
    /// The component's place in the design: below the design root for a card
    /// of the design's own netlist, below the occurrence a master's body is
    /// written at for a card in a `.subckt` body.
    pub instance: Option<InstancePath>,
    /// The nets the card connects, as the deck names them, in pin order.
    pub nets: Vec<String>,
}

/// The lines a generator writes, each with what it was written for.
///
/// Every line pushed takes the origin set at the time, so whoever writes a
/// line says what it is for by setting the origin before writing it. Read
/// access is the text alone; the only ways to change the lines keep the two
/// in step.
#[derive(Debug, Clone, Default)]
pub(super) struct WrittenLines {
    text: Vec<String>,
    origins: Vec<WrittenLine>,
    origin: WrittenLine,
}

impl std::ops::Deref for WrittenLines {
    type Target = [String];

    fn deref(&self) -> &[String] {
        &self.text
    }
}

impl From<Vec<String>> for WrittenLines {
    /// Lines written for nothing in particular.
    fn from(text: Vec<String>) -> Self {
        let origins = vec![WrittenLine::default(); text.len()];
        Self {
            text,
            origins,
            origin: WrittenLine::default(),
        }
    }
}

impl WrittenLines {
    /// Write `line` for the current origin.
    pub(super) fn push(&mut self, line: impl Into<String>) {
        self.text.push(line.into());
        self.origins.push(self.origin.clone());
    }

    /// Write `line` for `origin`, leaving the current origin as it is.
    pub(super) fn push_for(&mut self, line: impl Into<String>, origin: WrittenLine) {
        self.text.push(line.into());
        self.origins.push(origin);
    }

    pub(super) fn extend(&mut self, lines: impl IntoIterator<Item = String>) {
        for line in lines {
            self.push(line);
        }
    }

    /// Append lines another writer wrote, with what each was written for.
    pub(super) fn append(&mut self, mut other: Self) {
        self.text.append(&mut other.text);
        self.origins.append(&mut other.origins);
    }

    /// Keep the lines `keep` accepts.
    pub(super) fn retain(&mut self, keep: impl Fn(&str) -> bool) {
        let (text, origins) = std::mem::take(&mut self.text)
            .into_iter()
            .zip(std::mem::take(&mut self.origins))
            .filter(|(line, _)| keep(line))
            .unzip();
        self.text = text;
        self.origins = origins;
    }

    pub(super) fn clear(&mut self) {
        self.text.clear();
        self.origins.clear();
    }

    /// The origin every line written from now on takes.
    pub(super) fn set_origin(&mut self, origin: WrittenLine) {
        self.origin = origin;
    }

    /// The lines with what each was written for, emptying this.
    pub(super) fn take_pairs(&mut self) -> Vec<(String, WrittenLine)> {
        std::mem::take(&mut self.text)
            .into_iter()
            .zip(std::mem::take(&mut self.origins))
            .collect()
    }

    /// Replace the lines with `pairs`.
    pub(super) fn set_pairs(&mut self, pairs: Vec<(String, WrittenLine)>) {
        (self.text, self.origins) = pairs.into_iter().unzip();
    }

    /// The deck text, and what each of its physical lines was written for.
    /// A written line holding line breaks is that many deck lines, each
    /// written for it.
    pub(super) fn deck(&self) -> (String, Vec<WrittenLine>) {
        let mut map = Vec::with_capacity(self.text.len());
        for (line, origin) in self.text.iter().zip(&self.origins) {
            map.extend(std::iter::repeat_n(origin, line.split('\n').count()).cloned());
        }
        (self.text.join("\n"), map)
    }
}
