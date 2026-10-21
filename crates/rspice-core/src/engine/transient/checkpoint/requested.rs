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
//! carries.
use super::*;

/// Format 61 adds the pending model-requested breakpoints.
pub(super) const FORMAT_VERSION: u32 = 61;

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

pub(super) fn validate(requests: Option<&[Value]>, time: Value) -> Result<(), String> {
    let requests = requests.unwrap_or_default();
    if requests
        .iter()
        .any(|request| !request.is_finite() || *request <= time)
    {
        return Err("checkpoint model breakpoints must be finite and after the checkpoint".into());
    }
    if requests
        .windows(2)
        .any(|pair| pair[0].total_cmp(&pair[1]) != std::cmp::Ordering::Less)
    {
        return Err("checkpoint model breakpoints must be strictly ascending".into());
    }
    Ok(())
}

/// Refuse an image from before this section when the target holds a
/// connect-rule converter. Its X timeout, delayed starts and ramp ends are
/// model-requested breakpoints, so the image may have dropped one that was
/// pending; a stock XSPICE model's image resumes as it always did.
pub(super) fn validate_target(
    circuit: &CircuitData,
    requests: Option<&[Value]>,
) -> Result<(), String> {
    if requests.is_some() {
        return Ok(());
    }
    match circuit
        .xspice_instances
        .iter()
        .find(|instance| is_connect_converter(instance.model_name()))
    {
        Some(instance) => Err(format!(
            "legacy transient checkpoint does not carry the breakpoints its code models asked \
             for, and {}({}) is a connect-rule converter whose deadline may have been pending; \
             re-run the transient from t=0",
            instance.name,
            instance.model_name()
        )),
        None => Ok(()),
    }
}

#[cfg(feature = "veriloga")]
fn is_connect_converter(model: &str) -> bool {
    crate::xspice::models::is_connect_converter(model)
}

/// Without Verilog-A no connect rule inserts a converter.
#[cfg(not(feature = "veriloga"))]
fn is_connect_converter(_: &str) -> bool {
    false
}

/// Values a parsed image retains, for the checkpoint memory budget.
pub(super) fn retained_values(requests: Option<&[Value]>) -> usize {
    requests.map_or(0, <[_]>::len).saturating_add(1)
}

pub(super) fn write(
    out: &mut String,
    requests: Option<&[Value]>,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    let Some(requests) = requests else {
        out.push_str("pending_model_breakpoints unknown\n");
        return Ok(());
    };
    out.push_str(&format!("pending_model_breakpoints {}", requests.len()));
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
    version: u32,
    budget: &mut CheckpointParseBudget,
) -> Result<Option<Vec<Value>>, String> {
    // A file older than this section did not record what its models asked
    // for.
    if version < FORMAT_VERSION {
        return Ok(None);
    }
    let line = lines
        .next()
        .ok_or("missing pending model breakpoints line")?;
    let mut fields = line.split_whitespace();
    if fields.next() != Some("pending_model_breakpoints") {
        return Err(format!(
            "malformed pending model breakpoints line: '{line}'"
        ));
    }
    let count = match fields.next() {
        Some("unknown") => {
            return if fields.next().is_none() {
                Ok(None)
            } else {
                Err(format!(
                    "malformed pending model breakpoints line: '{line}'"
                ))
            };
        }
        field => field
            .and_then(|field| field.parse::<usize>().ok())
            .ok_or_else(|| format!("malformed pending model breakpoints line: '{line}'"))?,
    };
    let mut requests = allocate_checkpoint_capacity(count, "pending model breakpoints", budget)?;
    for index in 0..count {
        let field = fields.next().ok_or_else(|| {
            format!("pending model breakpoints declared {count} values but contained {index}")
        })?;
        requests.push(
            field
                .parse::<Value>()
                .map_err(|_| format!("malformed pending model breakpoint '{field}'"))?,
        );
    }
    if fields.next().is_some() {
        return Err(format!(
            "pending model breakpoints declared {count} values but contained more"
        ));
    }
    Ok(Some(requests))
}
