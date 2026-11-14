//! Pending breakpoints the code models asked for.
//!
//! At every accepted point a code model asks for the instants the stepper
//! must land on next: a converter's X-timeout deadline, the start of a change
//! it delays, the end of a ramp
//! ([`crate::numerics::integration::BreakpointManager::replace_model_requests`]).
//! A request holds for the next step only, and the model asks again at the
//! point that step accepts. A resumed run evaluates no model at its first
//! point, so a checkpoint taken less than the resumed run's first step before
//! one of those instants lost it: the resumed run stepped over the instant the
//! uninterrupted run lands on, and published the X, started the change or
//! ended the ramp at its first point instead. A checkpoint therefore carries
//! the requests still ahead of it, other than the edge completions `edge.rs`
//! carries, and apart from them the temporary ones, which it lands on without
//! restarting.
use super::*;

/// Which of the two request lists a line carries.
#[derive(Clone, Copy)]
pub(super) struct RequestList {
    keyword: &'static str,
    noun: &'static str,
}

/// The breakpoints the stepper restarts at.
pub(super) const BREAKPOINTS: RequestList = RequestList {
    keyword: "pending_model_breakpoints",
    noun: "pending model breakpoints",
};

/// The temporary breakpoints, landed on without a restart.
pub(super) const LANDINGS: RequestList = RequestList {
    keyword: "pending_model_landings",
    noun: "pending model temporary breakpoints",
};

/// Keep only what is still ahead of `time`, ascending.
pub(super) fn capture(requests: &[Value], time: Value) -> Vec<Value> {
    let mut requests = requests
        .iter()
        .copied()
        .filter(|request| request.is_finite() && *request > time)
        .collect::<Vec<_>>();
    requests.sort_by(Value::total_cmp);
    requests.dedup_by(|left, right| left.to_bits() == right.to_bits());
    requests
}

pub(super) fn validate(list: RequestList, requests: &[Value], time: Value) -> Result<(), String> {
    let noun = list.noun;
    if requests
        .iter()
        .any(|request| !request.is_finite() || *request <= time)
    {
        return Err(format!(
            "checkpoint {noun} must be finite and after the checkpoint"
        ));
    }
    if requests
        .windows(2)
        .any(|pair| pair[0].total_cmp(&pair[1]) != std::cmp::Ordering::Less)
    {
        return Err(format!("checkpoint {noun} must be strictly ascending"));
    }
    Ok(())
}

/// Values a parsed image retains, for the checkpoint memory budget.
pub(super) fn retained_values(requests: &[Value]) -> usize {
    requests.len().saturating_add(1)
}

pub(super) fn write(
    out: &mut String,
    list: RequestList,
    requests: &[Value],
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    out.push_str(&format!("{} {}", list.keyword, requests.len()));
    for (index, request) in requests.iter().enumerate() {
        poll_checkpoint_abort(abort, index)?;
        out.push(' ');
        out.push_str(&request.to_string());
    }
    out.push('\n');
    Ok(())
}

pub(super) fn read(
    lines: &mut CheckpointLines<'_>,
    list: RequestList,
    budget: &mut CheckpointParseBudget,
) -> Result<Vec<Value>, String> {
    let noun = list.noun;
    let line = lines.next().ok_or_else(|| format!("missing {noun} line"))?;
    let mut fields = line.split_whitespace();
    if fields.next() != Some(list.keyword) {
        return Err(format!("malformed {noun} line: '{line}'"));
    }
    let count = fields
        .next()
        .and_then(|field| field.parse::<usize>().ok())
        .ok_or_else(|| format!("malformed {noun} line: '{line}'"))?;
    let mut requests = allocate_checkpoint_capacity(count, noun, budget)?;
    for index in 0..count {
        let field = fields
            .next()
            .ok_or_else(|| format!("{noun} declared {count} values but contained {index}"))?;
        requests.push(
            field
                .parse::<Value>()
                .map_err(|_| format!("malformed value '{field}' in {noun}"))?,
        );
    }
    if fields.next().is_some() {
        return Err(format!("{noun} declared {count} values but contained more"));
    }
    Ok(requests)
}
