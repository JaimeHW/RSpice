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
//!
//! The same format refuses an older image of a circuit whose code models share
//! event nets with an HDL module. Such a run keys a code model's output on the
//! tick image it is a rounding from (`xspice::event_scheduler::Instant`'s "one
//! instant, one key"), and an older image queued those outputs at the sums
//! their models computed. A resumed run would land them where the
//! uninterrupted run never does.
use super::*;
use crate::engine::transient::{FloorLandingGrid, FloorMarch};

/// Format 63 adds the floor march, and keys coupled code-model outputs on the
/// HDL tick lattice.
pub(super) const FORMAT_VERSION: u32 = 63;

pub(super) fn validate(grid: Option<&FloorLandingGrid>, time: Value) -> Result<(), String> {
    let Some(march) = grid.and_then(|grid| grid.march) else {
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

/// Refuse an image from before this format when the target couples code
/// models to an HDL module, naming the format: its queued code-model times
/// predate the tick-lattice keys the resumed run would schedule beside them.
pub(super) fn validate_target(
    circuit: &CircuitData,
    grid: Option<&FloorLandingGrid>,
) -> Result<(), String> {
    if grid.is_some() || !circuit.has_coupled_event_nets() {
        return Ok(());
    }
    Err(format!(
        "legacy transient checkpoint predates format {FORMAT_VERSION}, which keys every \
         code-model event coupled to an HDL module on the HDL tick it is a rounding from; \
         this circuit couples code models to an HDL module, so the event times its image \
         queued would not resume where the uninterrupted run lands them; re-run the \
         transient from t=0"
    ))
}

/// Values a parsed image retains, for the checkpoint memory budget.
pub(super) fn retained_values(grid: Option<&FloorLandingGrid>) -> usize {
    if grid.and_then(|grid| grid.march).is_some() {
        6
    } else {
        1
    }
}

pub(super) fn write(out: &mut String, grid: Option<&FloorLandingGrid>) {
    match grid.map(|grid| grid.march) {
        None => out.push_str("floor_march unknown\n"),
        Some(None) => out.push_str("floor_march none\n"),
        Some(Some(FloorMarch {
            origin,
            hard_min_dt,
            steps,
            accepted,
            landed,
        })) => out.push_str(&format!(
            "floor_march {origin} {hard_min_dt} {steps} {accepted} {landed}\n"
        )),
    }
}

pub(super) fn read(
    lines: &mut CheckpointLines<'_>,
    version: u32,
) -> Result<Option<FloorLandingGrid>, String> {
    // A file older than this format started every resumed run on a fresh
    // grid, which is what reading it as unknown keeps.
    if version < FORMAT_VERSION {
        return Ok(None);
    }
    let line = lines.next().ok_or("missing floor march line")?;
    let malformed = || format!("malformed floor march line: '{line}'");
    let mut fields = line.split_whitespace();
    if fields.next() != Some("floor_march") {
        return Err(malformed());
    }
    let values = fields.collect::<Vec<_>>();
    match values.as_slice() {
        ["unknown"] => Ok(None),
        ["none"] => Ok(Some(FloorLandingGrid::default())),
        [origin, hard_min_dt, steps, accepted, landed] => {
            let parse = |field: &str| field.parse::<Value>().map_err(|_| malformed());
            Ok(Some(FloorLandingGrid {
                march: Some(FloorMarch {
                    origin: parse(origin)?,
                    hard_min_dt: parse(hard_min_dt)?,
                    steps: parse(steps)?,
                    accepted: parse(accepted)?,
                    landed: parse(landed)?,
                }),
            }))
        }
        _ => Err(malformed()),
    }
}
