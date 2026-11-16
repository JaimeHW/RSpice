//! A run verified against a plan: the one model every verification view reads.
//!
//! The sheet states, for each specification, what each point of the run gave
//! and what that means against the specification's limit, and rolls the rows
//! up into whether the run can be accepted. It adds no rule of its own: the
//! cells are the sealed judge's own candidates, placed at the point each came
//! from, and a row restates the judge's verdict. A view that read anything else could disagree with the
//! sign-off about the same run; reading this, it cannot.

mod points;
mod sheet;

pub use points::{Point, PointAxis, PointSource};
pub use sheet::{
    Acceptance, Cell, CellStatus, LimitsSinceRun, NEAR_LIMIT_FRACTION, Row, RowStatus, Sheet,
    StatusCounts, Summary, Worst,
};

#[cfg(test)]
mod tests;
