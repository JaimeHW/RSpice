//! Pending completions of zero-duration edges.
//!
//! An edge that takes no time is realized over the shortest transition its
//! model admits, and the model asks for the breakpoint that completes it at
//! the edge's own accepted point
//! ([`crate::numerics::integration::BreakpointManager::replace_edge_completions`]).
//! From then on the request lives only in the stepper's schedule, and a
//! resumed run evaluates no model at its first point to ask again. So a
//! checkpoint taken on an edge carries the completions still ahead and the
//! edge being crossed; without them the resumed run steps over the completion
//! the uninterrupted run lands on, and the two runs part at the first edge.
use super::*;
use crate::numerics::integration::EdgeCompletionSchedule;

/// Format 60 adds the pending edge completions.
pub(super) const FORMAT_VERSION: u32 = 60;

/// Keep only what is still ahead of `time`, ascending.
pub(super) fn capture(schedule: &EdgeCompletionSchedule, time: Value) -> EdgeCompletionSchedule {
    let mut completions = schedule
        .completions
        .iter()
        .copied()
        .filter(|completion| completion.is_finite() && *completion > time)
        .collect::<Vec<_>>();
    completions.sort_by(Value::total_cmp);
    completions.dedup_by(|left, right| left.to_bits() == right.to_bits());
    EdgeCompletionSchedule {
        completions,
        crossing: schedule
            .crossing
            .filter(|(completion, _)| completion.is_finite() && *completion > time),
    }
}

pub(super) fn validate(schedule: &EdgeCompletionSchedule, time: Value) -> Result<(), String> {
    if schedule
        .completions
        .iter()
        .any(|completion| !completion.is_finite() || *completion <= time)
    {
        return Err("checkpoint edge completions must be finite and after the checkpoint".into());
    }
    if schedule
        .completions
        .windows(2)
        .any(|pair| pair[0].total_cmp(&pair[1]) != std::cmp::Ordering::Less)
    {
        return Err("checkpoint edge completions must be strictly ascending".into());
    }
    if let Some((completion, resume)) = schedule.crossing {
        if !completion.is_finite() || completion <= time {
            return Err("checkpoint edge crossing must complete after the checkpoint".into());
        }
        if resume.is_some_and(|resume| !resume.is_finite() || resume <= 0.0) {
            return Err("checkpoint edge crossing resume step must be finite and positive".into());
        }
    }
    Ok(())
}

/// Values a parsed image retains, for the checkpoint memory budget.
pub(super) fn retained_values(schedule: &EdgeCompletionSchedule) -> usize {
    schedule.completions.len().saturating_add(2)
}

pub(super) fn write(
    out: &mut String,
    schedule: &EdgeCompletionSchedule,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    out.push_str(&format!(
        "pending_edge_completions {}",
        schedule.completions.len()
    ));
    for (index, completion) in schedule.completions.iter().enumerate() {
        poll_checkpoint_abort(abort, index)?;
        out.push(' ');
        out.push_str(&completion.to_string());
    }
    out.push('\n');
    match schedule.crossing {
        None => out.push_str("edge_crossing none\n"),
        Some((completion, resume)) => out.push_str(&format!(
            "edge_crossing {completion} {}\n",
            resume.map_or_else(|| "none".to_string(), |resume| resume.to_string())
        )),
    }
    Ok(())
}

pub(super) fn read(
    lines: &mut CheckpointLines<'_>,
    version: u32,
    budget: &mut CheckpointParseBudget,
) -> Result<EdgeCompletionSchedule, String> {
    // A file older than this section came from a run that never asked for an
    // edge completion, so its schedule was empty.
    if version < FORMAT_VERSION {
        return Ok(EdgeCompletionSchedule::default());
    }
    let line = lines
        .next()
        .ok_or("missing pending edge completions line")?;
    let mut fields = line.split_whitespace();
    if fields.next() != Some("pending_edge_completions") {
        return Err(format!("malformed pending edge completions line: '{line}'"));
    }
    let count = fields
        .next()
        .and_then(|field| field.parse::<usize>().ok())
        .ok_or_else(|| format!("malformed pending edge completions line: '{line}'"))?;
    let mut completions = allocate_checkpoint_capacity(count, "pending edge completions", budget)?;
    for index in 0..count {
        let field = fields.next().ok_or_else(|| {
            format!("pending edge completions declared {count} values but contained {index}")
        })?;
        completions.push(
            field
                .parse::<Value>()
                .map_err(|_| format!("malformed pending edge completion '{field}'"))?,
        );
    }
    if fields.next().is_some() {
        return Err(format!(
            "pending edge completions declared {count} values but contained more"
        ));
    }
    let line = lines.next().ok_or("missing edge crossing line")?;
    let fields = line.split_whitespace().collect::<Vec<_>>();
    let parse = |field: &str| {
        field
            .parse::<Value>()
            .map_err(|_| format!("malformed edge crossing line: '{line}'"))
    };
    let crossing = match fields.as_slice() {
        ["edge_crossing", "none"] => None,
        ["edge_crossing", completion, "none"] => Some((parse(completion)?, None)),
        ["edge_crossing", completion, resume] => Some((parse(completion)?, Some(parse(resume)?))),
        _ => return Err(format!("malformed edge crossing line: '{line}'")),
    };
    Ok(EdgeCompletionSchedule {
        completions,
        crossing,
    })
}
