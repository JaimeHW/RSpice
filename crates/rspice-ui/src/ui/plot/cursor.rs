//! A/B measurement cursor state.
//!
//! Placement follows the design: first click places A, second places B,
//! a third restarts at A. Values and deltas are read by the right panel;
//! the plot only draws the verticals and letter flags.

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

    /// Remove both cursors.
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// `true` when at least cursor A is placed.
    pub fn any(&self) -> bool {
        self.a.is_some()
    }

    /// B − A when both cursors are placed.
    pub fn delta(&self) -> Option<f64> {
        Some(self.b? - self.a?)
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
        assert_eq!(c.delta(), Some(1.0));
        c.place(3.0);
        assert_eq!((c.a, c.b), (Some(3.0), None));
        c.clear();
        assert!(!c.any());
    }
}
