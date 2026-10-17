//! The printed legend's grid: the one rule a plan sizes the legend band by
//! and every writer lays the legend out by.
//!
//! Entries fill the band column by column. A plan reserves as many rows as
//! its entries need across the columns its printable width holds at the
//! narrowest column an entry is given. A writer may widen a column for a long
//! label but never narrows one below that width, so the band a plan reserves
//! always holds the legend a writer prints.

use super::*;

/// Rows and columns of a page's legend band.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegendGrid {
    columns: u64,
    rows: u64,
}

impl LegendGrid {
    /// The narrowest column a legend entry is given.
    pub const MIN_COLUMN_WIDTH: Length = Length(64_000);
    /// The pitch from one legend row to the next.
    pub const ROW_PITCH: Length = Length(5_000);
    /// The band's height beyond its rows.
    pub const VERTICAL_PADDING: Length = Length(4_000);

    /// The grid a plan reserves for `entries` legend entries.
    ///
    /// # Errors
    ///
    /// Refuses a printable width narrower than one column.
    pub fn reserve(printable_width: Length, entries: usize) -> Result<Self, HardcopyError> {
        let columns = printable_width.0 / Self::MIN_COLUMN_WIDTH.0;
        if columns == 0 {
            return Err(HardcopyError::NoPrintableContentArea);
        }
        let entries = u64::try_from(entries).map_err(|_| HardcopyError::ScaledGeometryOverflow)?;
        Ok(Self {
            columns,
            rows: entries.div_ceil(columns),
        })
    }

    /// The grid a reserved legend band holds.
    #[must_use]
    pub const fn of_band(printable_width: Length, legend_band: Length) -> Self {
        Self {
            columns: printable_width.0 / Self::MIN_COLUMN_WIDTH.0,
            rows: legend_band.0.saturating_sub(Self::VERTICAL_PADDING.0) / Self::ROW_PITCH.0,
        }
    }

    /// The band height this grid needs.
    ///
    /// # Errors
    ///
    /// Refuses a height that overflows.
    pub fn band_height(self) -> Result<Length, HardcopyError> {
        self.rows
            .checked_mul(Self::ROW_PITCH.0)
            .and_then(|height| height.checked_add(Self::VERTICAL_PADDING.0))
            .map(Length)
            .ok_or(HardcopyError::ScaledGeometryOverflow)
    }

    #[must_use]
    pub const fn columns(self) -> u64 {
        self.columns
    }

    #[must_use]
    pub const fn rows(self) -> u64 {
        self.rows
    }

    /// How many entries the grid holds.
    #[must_use]
    pub const fn capacity(self) -> u64 {
        self.columns.saturating_mul(self.rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_reserved_band_holds_the_entries_it_was_reserved_for() {
        for width in [64_000, 127_999, 190_000, 277_000, 400_000] {
            for entries in 1..=40 {
                let reserved = LegendGrid::reserve(Length(width), entries).unwrap();
                let band = reserved.band_height().unwrap();
                let held = LegendGrid::of_band(Length(width), band);
                assert_eq!(held, reserved);
                assert!(held.capacity() >= entries as u64);
                assert!(held.capacity() < entries as u64 + held.columns());
            }
        }
        assert_eq!(
            LegendGrid::reserve(Length(63_999), 1),
            Err(HardcopyError::NoPrintableContentArea)
        );
    }
}
