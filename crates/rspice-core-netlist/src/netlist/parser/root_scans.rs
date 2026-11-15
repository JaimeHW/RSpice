//! What scanning a root's lines finds ahead of its first card.
//!
//! The scans read the lines alone, never a parameter value: the Spectre
//! statistics plan, the `.OPTIONS` and `.TEMP` cards, the root `.PARAM` names
//! a card may name before their definition, how many element cards to size
//! for, and the model cards nothing names. Every parse of one reading of a
//! deck therefore finds the same. A reading keeps them beside its text (see
//! `ReplaySource`), and each coordinate of a study (`.STEP`, `.TEMP`, `.DATA`,
//! Monte Carlo trials, sensitivity perturbations) replays the deck from that
//! one immutable reading instead of scanning its lines again, from any thread.

use super::*;
use std::sync::{Arc, OnceLock};

/// The scans of one root's lines.
#[derive(Debug)]
pub(super) struct RootScans {
    /// What the lines were read as: where the body starts and whether
    /// comments follow Xyce's syntax.
    body_start: usize,
    xyce_syntax: bool,
    pub(super) spectre_statistics: SpectreStatisticsPlan,
    pub(super) option_and_temperature_cards: Vec<ScannedCard>,
    pub(super) forward_parameter_names: Arc<forward_params::ForwardParameterNames>,
    /// Lines that start with a letter, each of which may be an element card.
    pub(super) element_cards: usize,
    /// Scanned by the first parse that skips the model cards nothing names.
    unreferenced_models: OnceLock<Option<Arc<model_references::UnreferencedModels>>>,
}

impl RootScans {
    fn scan(
        lines: &[&str],
        body_start: usize,
        xyce_syntax: bool,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        let allow_non_semicolon_comments = !xyce_syntax;
        let spectre_statistics = prescan_spectre_statistics_with_abort(
            lines,
            body_start,
            allow_non_semicolon_comments,
            xyce_syntax,
            abort,
        )?;
        let option_and_temperature_cards = scan_option_and_temperature_cards(
            lines,
            body_start,
            allow_non_semicolon_comments,
            xyce_syntax,
            abort,
        )?;
        let forward_parameter_names = Arc::new(forward_params::ForwardParameterNames::scan(
            lines,
            body_start,
            allow_non_semicolon_comments,
            xyce_syntax,
            abort,
        )?);
        let element_cards = lines
            .iter()
            .skip(body_start)
            .filter(|line| {
                line.trim_start()
                    .as_bytes()
                    .first()
                    .is_some_and(u8::is_ascii_alphabetic)
            })
            .count();
        Ok(Self {
            body_start,
            xyce_syntax,
            spectre_statistics,
            option_and_temperature_cards,
            forward_parameter_names,
            element_cards,
            unreferenced_models: OnceLock::new(),
        })
    }

    /// The model cards no other card of `lines`, the lines these scans read,
    /// reaches.
    pub(super) fn unreferenced_models(
        &self,
        lines: &[&str],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Arc<model_references::UnreferencedModels>>, ParseWithAbortError> {
        if let Some(models) = self.unreferenced_models.get() {
            return Ok(models.clone());
        }
        let models = model_references::UnreferencedModels::scan(lines, self.body_start, abort)?
            .map(Arc::new);
        Ok(self.unreferenced_models.get_or_init(|| models).clone())
    }
}

/// A reading's [`RootScans`], filled by the first parse of it.
#[derive(Debug, Default)]
pub(crate) struct RootScansCell(OnceLock<Arc<RootScans>>);

/// The scans of `lines`, taken from `cell` when it holds those of lines read
/// the same way, and otherwise scanned, and kept in `cell` for the parses
/// after this one.
pub(super) fn root_scans(
    cell: Option<&RootScansCell>,
    lines: &[&str],
    body_start: usize,
    xyce_syntax: bool,
    abort: &dyn AbortSignal,
) -> Result<Arc<RootScans>, ParseWithAbortError> {
    if let Some(scans) = cell
        .and_then(|cell| cell.0.get())
        .filter(|scans| scans.body_start == body_start && scans.xyce_syntax == xyce_syntax)
    {
        return Ok(Arc::clone(scans));
    }
    let scans = Arc::new(RootScans::scan(lines, body_start, xyce_syntax, abort)?);
    if let Some(cell) = cell {
        // A parse on another thread may have filled the cell first, with the
        // same scans of the same lines.
        let _ = cell.0.set(Arc::clone(&scans));
    }
    Ok(scans)
}
