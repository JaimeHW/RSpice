//! A run's refusal for going past one of the engine's limits, in the words
//! the desktop app says it in: what is too large, how large it is and how
//! large it can be, "This deck is 95.4 MB; a deck can be at most 64 MB.",
//! then the engine's own words for it, which scripts match on.

use rspice_core::{ResourceKind, ResourceLimitError};

/// How a resource's amounts are said.
#[derive(Clone, Copy)]
enum Measure {
    /// Bytes, in KB, MB or GB.
    Bytes,
    /// A count, with its thousands grouped.
    Count,
}

/// The sentence a refusal for `resource` is said in, its amounts put in for
/// `{requested}` and `{limit}`, and how they are said: the app's catalogue
/// sentences, in English.
const fn said_as(resource: ResourceKind) -> (&'static str, Measure) {
    use Measure::{Bytes, Count};
    match resource {
        ResourceKind::NetlistBytes => (
            "This deck is {requested}; a deck can be at most {limit}.",
            Bytes,
        ),
        ResourceKind::NetlistLines => (
            "This deck has {requested} lines; a deck can have at most {limit}.",
            Count,
        ),
        ResourceKind::ExpandedSourceBytes => (
            "This deck comes to {requested} with the files it includes read in; it can come to \
             at most {limit}.",
            Bytes,
        ),
        ResourceKind::DependencySourceBytes => (
            "The files this deck includes come to {requested}; they can come to at most {limit}.",
            Bytes,
        ),
        ResourceKind::ExternalDataBytes => (
            "A data file a source reads is {requested}; a data file can be at most {limit}.",
            Bytes,
        ),
        ResourceKind::ExternalDataValues => (
            "A data file a source reads holds {requested} values; a data file can hold at most \
             {limit}.",
            Count,
        ),
        ResourceKind::SharedCacheBytes => (
            "The compiled devices kept between runs come to {requested}; they can come to at most \
             {limit}.",
            Bytes,
        ),
        ResourceKind::IncludeDepth => (
            "This deck's includes nest {requested} deep; they can nest at most {limit} deep.",
            Count,
        ),
        ResourceKind::HierarchyDepth => (
            "This circuit's subcircuits nest {requested} deep; they can nest at most {limit} deep.",
            Count,
        ),
        ResourceKind::FlattenedElements => (
            "This circuit has {requested} devices with its subcircuits expanded; a circuit can \
             have at most {limit}.",
            Count,
        ),
        ResourceKind::CircuitNodes => (
            "This circuit has {requested} nodes; a circuit can have at most {limit}.",
            Count,
        ),
        ResourceKind::MatrixUnknowns => (
            "This circuit has {requested} unknowns to solve for; a circuit can have at most \
             {limit}.",
            Count,
        ),
        ResourceKind::ResultValues => (
            "This result would hold {requested} values; a result can hold at most {limit}.",
            Count,
        ),
        ResourceKind::TransportHistoryBytes => (
            "The delay lines' histories need {requested}; they can take at most {limit}.",
            Bytes,
        ),
        ResourceKind::TransportHistoryRecords => (
            "A delay line's history holds {requested} points; it can hold at most {limit}.",
            Count,
        ),
        ResourceKind::MixedIntervalEvents => (
            "One step of the mixed-signal run has {requested} events; a step can have at most \
             {limit}.",
            Count,
        ),
        ResourceKind::ParallelWorkers => (
            "This run asks for {requested} threads; a run can use at most {limit}.",
            Count,
        ),
        ResourceKind::BatchRuns => (
            "This analysis asks for {requested} runs; an analysis can make at most {limit}.",
            Count,
        ),
    }
}

/// What `refusal` refused, as the desktop app says it, then the engine's
/// words for it: "This deck is 95.4 MB; a deck can be at most 64 MB.
/// (netlist_bytes limit exceeded: requested 100000091, limit 67108864)".
pub(crate) fn said(refusal: &ResourceLimitError) -> String {
    let (sentence, measure) = said_as(refusal.resource);
    let limit = amount(refusal.limit, measure);
    let requested = amount(refusal.requested, measure);
    // The engine stops counting at the first one over, and an amount can
    // round to the limit: either is said as more than the limit, never as an
    // amount that reads as within it.
    let requested = if requested == limit || refusal.requested == refusal.limit.saturating_add(1) {
        format!("more than {limit}")
    } else {
        requested
    };
    let sentence = sentence
        .replace("{requested}", &requested)
        .replace("{limit}", &limit);
    format!("{sentence} ({refusal})")
}

/// What `error` says, a limit the run went past said as [`said`] says it.
pub(crate) fn simulation_words(error: &rspice_core::SimulationError) -> String {
    match error {
        rspice_core::SimulationError::ResourceLimit(refusal) => said(refusal),
        rspice_core::SimulationError::DeviceResourceLimit { instance, source } => {
            format!("Device '{instance}': {}", said(source))
        }
        other => other.to_string(),
    }
}

/// `amount` as `measure` says it: bytes in the largest of KB, MB and GB it
/// reaches, to a tenth, or a count with its thousands grouped.
fn amount(amount: usize, measure: Measure) -> String {
    const KILO: f64 = 1024.0;
    let Measure::Bytes = measure else {
        return grouped(amount);
    };
    #[allow(clippy::cast_precision_loss)]
    let bytes = amount as f64;
    let (unit, scaled) = if bytes >= KILO * KILO * KILO {
        ("GB", bytes / (KILO * KILO * KILO))
    } else if bytes >= KILO * KILO {
        ("MB", bytes / (KILO * KILO))
    } else if bytes >= KILO {
        ("KB", bytes / KILO)
    } else {
        return format!("{} bytes", grouped(amount));
    };
    let tenths = format!("{scaled:.1}");
    format!("{} {unit}", tenths.strip_suffix(".0").unwrap_or(&tenths))
}

/// `count` with its thousands grouped: "2,000,000".
fn grouped(count: usize) -> String {
    let digits = count.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A deck past the shipped size is said as the app says it, and the
    /// engine's words for it follow, for a script that matches on them.
    #[test]
    fn a_refusal_reads_as_the_app_says_it_then_as_the_engine_does() {
        let refusal = ResourceLimitError {
            resource: ResourceKind::NetlistBytes,
            requested: 100_000_091,
            limit: 64 * 1024 * 1024,
        };
        assert_eq!(
            said(&refusal),
            "This deck is 95.4 MB; a deck can be at most 64 MB. (netlist_bytes limit exceeded: \
             requested 100000091, limit 67108864)"
        );
        let refusal = ResourceLimitError {
            resource: ResourceKind::FlattenedElements,
            requested: 250_001,
            limit: 250_000,
        };
        assert!(said(&refusal).starts_with(
            "This circuit has more than 250,000 devices with its subcircuits expanded; a circuit \
             can have at most 250,000."
        ));
    }
}
