//! A run verified against a plan: the one model every verification view reads.
//!
//! The sheet states, for each specification, what each point of the run gave
//! and what that means against the specification's limit, and rolls the rows
//! up into whether the run can be accepted. It adds no rule of its own: the
//! cells are the sealed judge's own candidates, placed at the point each came
//! from, a row restates the judge's verdict, and a yield is the acceptance
//! gate's own count. A view that read anything else could disagree with the
//! sign-off about the same run; reading this, it cannot.

mod points;
mod sheet;
mod statistics;

pub use points::{Point, PointAxis, PointSource};
pub use sheet::{
    Acceptance, Cell, CellStatus, LimitsSinceRun, NEAR_LIMIT_FRACTION, Row, RowStatus, Sheet,
    StatusCounts, Summary, Worst,
};
pub use statistics::{JointYield, PointStatistics, Statistics, Trial, YieldCount};

#[cfg(test)]
mod tests;
