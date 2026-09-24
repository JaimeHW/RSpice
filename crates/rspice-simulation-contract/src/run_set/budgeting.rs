//! Run-set budget authoring and workload presentation.

/// Storage sizes in the binary units the budgets are authored in.
#[must_use]
pub fn format_bytes(bytes: u64) -> String {
    const TIB: u64 = 1024 * 1024 * 1024 * 1024;
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    const KIB: u64 = 1024;
    if bytes >= TIB {
        format!("{:.2} TiB", bytes as f64 / TIB as f64)
    } else if bytes >= GIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.2} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.2} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{bytes} B")
    }
}

/// Parse a storage budget written in engineering units.
///
/// The field round-trips [`format_bytes`], so its own output must parse back to
/// the same number; decimal suffixes are accepted because vendor quotas are
/// written that way.
pub fn parse_bytes(text: &str) -> Result<u64, String> {
    let trimmed = text.trim();
    let (magnitude, scale) = [
        ("TiB", 1024u64.pow(4)),
        ("GiB", 1024u64.pow(3)),
        ("MiB", 1024u64.pow(2)),
        ("KiB", 1024),
        ("TB", 1_000_000_000_000),
        ("GB", 1_000_000_000),
        ("MB", 1_000_000),
        ("kB", 1_000),
        ("B", 1),
    ]
    .into_iter()
    .find_map(|(suffix, scale)| {
        trimmed
            .strip_suffix(suffix)
            .or_else(|| {
                trimmed
                    .len()
                    .checked_sub(suffix.len())
                    .filter(|split| trimmed.is_char_boundary(*split))
                    .filter(|split| trimmed[*split..].eq_ignore_ascii_case(suffix))
                    .map(|split| &trimmed[..split])
            })
            .map(|magnitude| (magnitude, scale))
    })
    .unwrap_or((trimmed, 1));
    let magnitude: f64 = magnitude
        .trim()
        .replace([',', '_'], "")
        .parse()
        .map_err(|_| format!("{text:?} is not a storage size"))?;
    if !magnitude.is_finite() || magnitude < 0.0 {
        return Err(format!("{text:?} is not a storage size"));
    }
    Ok((magnitude * scale as f64).round() as u64)
}

/// The modelled wall-clock cost of a queue of `task_count` tasks.
///
/// The one owner of the tasks-to-duration arithmetic. Duration is not an
/// independent estimate: it is the task count priced at the run set's own
/// per-task budget, so a surface that states a duration is restating the queue
/// it already promised. Everything that shows one — the Run Set forecast tile,
/// the resolved point table, the preview receipt, the preflight Execution cell
/// and the per-analysis task-rate table — multiplies here rather than locally,
/// which is what makes a duration that disagrees with its own task count
/// inexpressible.
///
/// Saturating rather than checked: a task count that overflows the budget has
/// already been refused by [`super::RunSetBudgets::maximum_tasks`], and a `None`
/// duration on a surface that did state a task count reads as "unknown" when
/// the truth is "longer than anyone will wait".
#[must_use]
pub const fn modelled_cost_ms(task_count: usize, cost_per_task_ms: u64) -> u64 {
    (task_count as u64).saturating_mul(cost_per_task_ms)
}

/// A duration in the form the forecast tile shows.
#[must_use]
pub fn format_duration_ms(milliseconds: u64) -> String {
    let seconds = milliseconds as f64 / 1000.0;
    if seconds < 60.0 {
        format!("{seconds:.2} s")
    } else if seconds < 3600.0 {
        format!("{:.0} m {:02.0} s", seconds / 60.0, seconds % 60.0)
    } else {
        format!(
            "{:.0} h {:02.0} m",
            seconds / 3600.0,
            (seconds % 3600.0) / 60.0
        )
    }
}
