//! Run one `.DCMATCH` card against a design.
//!
//! The card is not spelled here. The Studio writes one line — the same line
//! the Analyses page displays — and this service reads it back through
//! `rspice-core`'s own parser to obtain the `DcMatchCard` the engine runs. So
//! there is one probe grammar in the build, and the run executes exactly the
//! request the page stated.
//!
//! Why the line rather than the deck: a generated deck carries every queued
//! analysis's directive, and a hand-written deck may carry several `.DCMATCH`
//! cards. Nothing in an executable deck identifies which of them is *this*
//! task's, so picking one out of it would be a guess. The line the task was
//! dispatched with is not a guess.
//!
//! The spreads come from the design's own Spectre `statistics` block, and the
//! engine refuses by name when none is bound. That refusal arrives verbatim;
//! what this service adds — only when the design really does declare no
//! variation — is the sentence that says where a Studio project gets one.

use std::path::Path;

use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::dcmatch::DcMatchResult;
use rspice_core::engine::Engine;
use rspice_core::netlist::{AnalysisCommand, DcMatchCard};

use super::error::ensure_not_aborted;
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, parse_runner_netlist_with_abort,
};

/// What one `.DCMATCH` task produced, with the card it actually ran.
///
/// The card travels back because the report has to explain itself: a list of
/// ten contributors out of four hundred evaluated is a different answer from
/// a list of ten out of ten, and the limits that trimmed it are the card's.
pub struct DcMismatchData {
    pub result: DcMatchResult,
    pub card: DcMatchCard,
}

/// What the Studio can do about a design that declares no statistics.
///
/// Appended after the engine's own sentence, never instead of it, and only
/// when the design really does declare no variation — the engine refuses
/// for other reasons too, and telling an operator to attach a library when
/// one is already attached would send them the wrong way.
const STATISTICS_REMEDY: &str = " Attach a Spectre model library whose bound section declares one, \
                                 or include a Spectre file that does.";

/// Run the card `card_line` spells against the design in `netlist_text`.
pub fn run_dc_mismatch_analysis_with_source_path_and_abort(
    netlist_text: &str,
    card_line: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<DcMismatchData> {
    ensure_not_aborted(abort)?;
    let card = studio_card(card_line, abort)?;
    ensure_not_aborted(abort)?;

    let parsed = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let engine = Engine::try_new(build_engine_config(&parsed, None)).map_err(|error| {
        ServiceRunError::Failure(format!("Invalid DC mismatch numerical policy: {error}"))
    })?;
    let result = engine
        .run_dc_match_with_abort(&parsed, &card, abort)
        .map_err(|error| {
            let failure = ServiceRunError::from_core("DC mismatch analysis error", error);
            // Cancellation and resource limits stay typed; only a plain
            // failure can carry the remedy, and only when the design is the
            // thing that is missing.
            match failure {
                ServiceRunError::Failure(message)
                    if parsed.spectre_statistics.variations.is_empty() =>
                {
                    ServiceRunError::Failure(format!("{message}{STATISTICS_REMEDY}"))
                }
                other => other,
            }
        })?;
    ensure_not_aborted(abort)?;

    Ok(DcMismatchData { result, card })
}

/// Read the one `.DCMATCH` card the Studio wrote.
///
/// The line is parsed in a carrier deck of its own so the engine's parser is
/// the only reader of it. A carrier that produced anything other than exactly
/// one `.DCMATCH` request is refused by name rather than run: it would mean
/// the writer and the parser had stopped agreeing, which is the defect a
/// string comparison between them cannot see.
fn studio_card(card_line: &str, abort: &dyn AbortSignal) -> ServiceRunResult<DcMatchCard> {
    let carrier = format!("RSpice DC mismatch card\n{}\n.end\n", card_line.trim());
    let parsed =
        parse_runner_netlist_with_abort(&carrier, None, abort).map_err(|error| match error {
            ServiceRunError::Failure(message) => ServiceRunError::Failure(format!(
                "the DC mismatch card the Studio wrote did not read back as one .DCMATCH card: \
                 {message}"
            )),
            typed => typed,
        })?;
    match parsed.analyses.as_slice() {
        [AnalysisCommand::DcMatch(card)] => Ok((**card).clone()),
        other => Err(ServiceRunError::Failure(format!(
            "the DC mismatch card the Studio wrote did not read back as one .DCMATCH card: \
             `{card_line}` parsed as {other:?}"
        ))),
    }
}
