//! A/B measurement cursor state.
//!
//! The first click places A and the second B. After that a click places
//! nothing and deletes nothing: a cursor moves by dragging its line, by the
//! arrow keys, or by a command that names it. Values and deltas are read by
//! the readout; the plot only draws the verticals and letter flags.

/// One cursor of the A/B pair.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CursorId {
    #[default]
    A,
    B,
}

impl CursorId {
    /// The letter the cursor's flag and column carry.
    pub const fn letter(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
        }
    }

    /// The other cursor of the pair.
    pub const fn other(self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}

/// The A/B cursor pair, in data-space X of the strip they were placed on.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CursorPair {
    /// Cursor A position.
    pub a: Option<f64>,
    /// Cursor B position.
    pub b: Option<f64>,
}

impl CursorPair {
    /// Apply one click at `x`, and say which cursor it placed: A first,
    /// then B, then none. A click never takes a cursor away; restarting at A
    /// on the third click deleted B, a measurement the reader had made.
    pub fn place(&mut self, x: f64) -> Option<CursorId> {
        let placed = self.next_placed()?;
        self.set(placed, x);
        Some(placed)
    }

    /// Which cursor the next click places, if any.
    pub fn next_placed(&self) -> Option<CursorId> {
        match (self.a, self.b) {
            (None, _) => Some(CursorId::A),
            (Some(_), None) => Some(CursorId::B),
            (Some(_), Some(_)) => None,
        }
    }

    /// Where cursor `id` is, if it is placed.
    pub fn get(&self, id: CursorId) -> Option<f64> {
        match id {
            CursorId::A => self.a,
            CursorId::B => self.b,
        }
    }

    /// Put cursor `id` at `x`.
    pub fn set(&mut self, id: CursorId, x: f64) {
        match id {
            CursorId::A => self.a = Some(x),
            CursorId::B => self.b = Some(x),
        }
    }

    /// Remove both cursors.
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// `true` when at least cursor A is placed.
    pub fn any(&self) -> bool {
        self.a.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The first click places A, the second B, and a third leaves both
    /// cursors where they were.
    #[test]
    fn a_third_click_leaves_both_cursors_where_they_were() {
        let mut c = CursorPair::default();
        assert_eq!(c.place(1.0), Some(CursorId::A));
        assert_eq!(c.a, Some(1.0));
        assert_eq!(c.place(2.0), Some(CursorId::B));
        assert_eq!(c.b, Some(2.0));
        assert_eq!(c.place(3.0), None);
        assert_eq!((c.a, c.b), (Some(1.0), Some(2.0)));
        c.clear();
        assert!(!c.any());
    }

    #[test]
    fn the_next_click_places_the_cursor_still_missing() {
        let mut c = CursorPair::default();
        assert_eq!(c.next_placed(), Some(CursorId::A));
        c.place(1.0);
        assert_eq!(c.next_placed(), Some(CursorId::B));
        c.place(2.0);
        assert_eq!(c.next_placed(), None);
        c.set(CursorId::B, 5.0);
        assert_eq!(
            (c.get(CursorId::A), c.get(CursorId::B)),
            (Some(1.0), Some(5.0))
        );
        assert_eq!(CursorId::A.other().letter(), "B");
    }
}
