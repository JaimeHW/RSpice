//! Clip event timelines on an exact integer grid, retaining held state.
use crate::cli::CliError;
use rspice_core::io::{VcdDocument, VcdTimescale};

fn empty_range() -> CliError {
    CliError::ConversionError {
        message: "no VCD data remains within --start/--stop on the input timescale".into(),
    }
}

/// Convert a seconds boundary once, without rounding each event's integer tick
/// to binary64. Recover grid points within arithmetic roundoff, capped well
/// below a tick so a real fractional boundary cannot be snapped at large times.
fn grid_position(time: f64, period: f64) -> f64 {
    let ratio = time / period;
    let nearest = ratio.round();
    let roundoff = (ratio.abs() * 8.0 * f64::EPSILON).min(1e-7);
    if nearest * period == time || (ratio - nearest).abs() <= roundoff {
        nearest
    } else {
        ratio
    }
}

fn clipped_grid(original: VcdTimescale, start: f64) -> Result<(VcdTimescale, u64, u64), CliError> {
    for scale in VcdTimescale::ALL {
        if scale.femtoseconds() > original.femtoseconds() {
            continue;
        }
        let position = grid_position(start, scale.seconds());
        // u64::MAX rounds up to 2^64 in binary64, which is outside the grid.
        if position < u64::MAX as f64 && position.fract() == 0.0 {
            return Ok((
                scale,
                original.femtoseconds() / scale.femtoseconds(),
                position as u64,
            ));
        }
    }
    Err(CliError::ConversionError {
        message: "--start cannot be represented on a VCD tick grid without rounding or overflow"
            .into(),
    })
}

pub(super) fn clip(
    document: &mut VcdDocument,
    start: Option<f64>,
    stop: Option<f64>,
) -> Result<(), CliError> {
    if start.is_none() && stop.is_none() {
        return Ok(());
    }
    let (timescale, factor, lower) =
        clipped_grid(document.timescale, start.unwrap_or(0.0).max(0.0))?;
    // The upper bound needs no new event; select original ticks before scaling
    // so discarded late events cannot cause a false rescaling overflow.
    let upper = match stop {
        Some(stop) if stop < 0.0 => return Err(empty_range()),
        Some(stop) => grid_position(stop, document.timescale.seconds()).floor() as u64,
        None => u64::MAX,
    };
    for signal in &mut document.signals {
        let end = signal
            .changes
            .partition_point(|change| change.tick <= upper);
        signal.changes.truncate(end);
        if signal
            .changes
            .last()
            .is_some_and(|change| change.tick.checked_mul(factor).is_none())
        {
            return Err(CliError::ConversionError {
                message: "representing --start would overflow retained VCD event ticks; narrow --stop or choose a start on the original grid".into(),
            });
        }
        for change in &mut signal.changes {
            change.tick *= factor;
        }
        let mut first = signal.changes.partition_point(|change| change.tick < lower);
        if first > 0
            && signal
                .changes
                .get(first)
                .is_none_or(|change| change.tick > lower)
        {
            // Reuse the last preceding record instead of allocating a new
            // event. Unknown/high-impedance bits and real values stay exact.
            first -= 1;
            signal.changes[first].tick = lower;
        }
        signal.changes.drain(..first);
    }
    document.timescale = timescale;
    if document
        .signals
        .iter()
        .all(|signal| signal.changes.is_empty())
    {
        return Err(empty_range());
    }
    Ok(())
}
