//! Clip event timelines on an exact integer grid, retaining held state.
use crate::cli::{CliError, NumericBound};
use rspice_core::io::{VcdDocument, VcdTimescale};

fn empty_range() -> CliError {
    CliError::ConversionError {
        message: "no VCD data remains within --start/--stop on the input timescale".into(),
    }
}

/// Every VCD period is an exact power of ten seconds.
fn period_power(scale: VcdTimescale) -> i64 {
    i64::from(scale.femtoseconds().ilog10()) - 15
}

fn clipped_grid(
    original: VcdTimescale,
    start: &NumericBound,
) -> Result<(VcdTimescale, u64, u64), CliError> {
    if start.is_negative() {
        return Ok((original, 1, 0));
    }
    for scale in VcdTimescale::ALL {
        if scale.femtoseconds() > original.femtoseconds() {
            continue;
        }
        if let Some((position, true)) = start.grid_position(period_power(scale)) {
            return Ok((
                scale,
                original.femtoseconds() / scale.femtoseconds(),
                position,
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
    start: Option<&NumericBound>,
    stop: Option<&NumericBound>,
) -> Result<(), CliError> {
    if start.is_none() && stop.is_none() {
        return Ok(());
    }
    let (timescale, factor, lower) = match start {
        Some(start) => clipped_grid(document.timescale, start)?,
        None => (document.timescale, 1, 0),
    };
    // The upper bound needs no new event; select original ticks before scaling
    // so discarded late events cannot cause a false rescaling overflow.
    let upper = match stop {
        Some(stop) if stop.is_negative() => return Err(empty_range()),
        Some(stop) => stop
            .grid_position(period_power(document.timescale))
            .map_or(u64::MAX, |(position, _)| position),
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
