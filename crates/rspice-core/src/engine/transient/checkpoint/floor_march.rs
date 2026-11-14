//! The floor grid a run of sub-minimum landings walks.
//!
//! A schedule finer than the solver's floor is landed on one grid,
//! `origin + k * hard_min`, rather than on a chain of nudged steps
//! (`engine::transient`'s `landed_veriloga_event_time`). The grid belongs to
//! the run of landings, not to any one point, so a resumed run that started a
//! fresh grid at its first point put every later landing of a march it resumed
//! into one rounding off the grid the uninterrupted run kept walking: the two
//! runs parted at the first landing after the seam. A checkpoint therefore
//! carries the march in progress.
use super::*;
use crate::engine::transient::{FloorLandingGrid, FloorMarch};

pub(super) fn validate(grid: &FloorLandingGrid, time: Value) -> Result<(), String> {
    let Some(march) = grid.march else {
        return Ok(());
    };
    let FloorMarch {
        origin,
        hard_min_dt,
        steps,
        accepted,
        landed,
    } = march;
    if ![origin, hard_min_dt, steps, accepted, landed]
        .iter()
        .all(|value| value.is_finite())
        || hard_min_dt <= 0.0
        || steps < 1.0
        || steps.fract() != 0.0
        || origin > accepted
        || accepted >= landed
        || accepted > time
    {
        return Err(format!(
            "checkpoint floor march {origin} + {steps} * {hard_min_dt} from {accepted} to \
             {landed} does not describe a run of landings at or before {time}"
        ));
    }
    Ok(())
}

/// Values a parsed image retains, for the checkpoint memory budget.
pub(super) fn retained_values(grid: &FloorLandingGrid) -> usize {
    if grid.march.is_some() { 6 } else { 1 }
}

pub(super) fn write(out: &mut String, grid: &FloorLandingGrid) {
    match grid.march {
        None => out.push_str("floor_march none\n"),
        Some(FloorMarch {
            origin,
            hard_min_dt,
            steps,
            accepted,
            landed,
        }) => out.push_str(&format!(
            "floor_march {origin} {hard_min_dt} {steps} {accepted} {landed}\n"
        )),
    }
}

pub(super) fn read(lines: &mut CheckpointLines<'_>) -> Result<FloorLandingGrid, String> {
    let line = lines.next().ok_or("missing floor march line")?;
    let malformed = || format!("malformed floor march line: '{line}'");
    let mut fields = line.split_whitespace();
    if fields.next() != Some("floor_march") {
        return Err(malformed());
    }
    let values = fields.collect::<Vec<_>>();
    match values.as_slice() {
        ["none"] => Ok(FloorLandingGrid::default()),
        [origin, hard_min_dt, steps, accepted, landed] => {
            let parse = |field: &str| field.parse::<Value>().map_err(|_| malformed());
            Ok(FloorLandingGrid {
                march: Some(FloorMarch {
                    origin: parse(origin)?,
                    hard_min_dt: parse(hard_min_dt)?,
                    steps: parse(steps)?,
                    accepted: parse(accepted)?,
                    landed: parse(landed)?,
                }),
            })
        }
        _ => Err(malformed()),
    }
}
