//! A/B measurement cursor state.
//!
//! Placement follows the design: first click places A, second places B,
//! a third restarts at A. Values and deltas are read by the right panel;
//! the plot only draws the verticals and letter flags.

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
    /// Apply one click at `x`.
    pub fn place(&mut self, x: f64) {
        match (self.a, self.b) {
            (None, _) => self.a = Some(x),
            (Some(_), None) => self.b = Some(x),
            (Some(_), Some(_)) => {
                self.a = Some(x);
                self.b = None;
            }
        }
    }

    /// Which cursor the next click places.
    pub fn next_placed(&self) -> CursorId {
        if self.a.is_some() && self.b.is_none() {
            CursorId::B
        } else {
            CursorId::A
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

    #[test]
    fn place_cycle() {
        let mut c = CursorPair::default();
        c.place(1.0);
        assert_eq!(c.a, Some(1.0));
        c.place(2.0);
        assert_eq!(c.b, Some(2.0));
        c.place(3.0);
        assert_eq!((c.a, c.b), (Some(3.0), None));
        c.clear();
        assert!(!c.any());
    }

    #[test]
    fn the_next_click_places_the_cursor_the_cycle_is_on() {
        let mut c = CursorPair::default();
        assert_eq!(c.next_placed(), CursorId::A);
        c.place(1.0);
        assert_eq!(c.next_placed(), CursorId::B);
        c.place(2.0);
        assert_eq!(c.next_placed(), CursorId::A);
        c.set(CursorId::B, 5.0);
        assert_eq!(
            (c.get(CursorId::A), c.get(CursorId::B)),
            (Some(1.0), Some(5.0))
        );
        assert_eq!(CursorId::A.other().letter(), "B");
    }
}
