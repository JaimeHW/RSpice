//! Reading a hand-written `.fft` card as a recorded FFT analysis.
//!
//! Layer: controller, manual-deck intake. The deck already carries the cards,
//! so nothing is spliced here and nothing is written: each card the parser
//! read becomes one queue item whose `analysis_line` is that same card, in
//! the one canonical spelling both ends of the spectrum pairing use.
//!
//! Before this existed the engine still evaluated the cards — it does so
//! inside the transient — and the Studio discarded the spectra: computed,
//! then dropped, with no task and no result.

use rspice_core::Netlist;

use crate::simulation::config::FftRequest;
use crate::simulation::controller::QueuedAnalysis;
use crate::simulation::multi_run::AnalysisSpec;

/// One queue item per `.fft` card the deck holds, in card order.
pub(super) fn manual_fft_tasks(parsed: &Netlist) -> Result<Vec<QueuedAnalysis>, Vec<String>> {
    if parsed.fft_analyses.is_empty() {
        return Ok(Vec::new());
    }
    let mut tasks = Vec::with_capacity(parsed.fft_analyses.len());
    let mut errors = Vec::new();
    for (index, analysis) in parsed.fft_analyses.iter().enumerate() {
        let request = FftRequest::from_core(analysis);
        if let Err(error) = request.validate() {
            errors.push(format!(".fft card {}: {error}", index + 1));
            continue;
        }
        let analysis_line = request.to_card();
        tasks.push(QueuedAnalysis {
            spec: AnalysisSpec::Fft { request },
            config: None,
            spec_options: crate::simulation::runner::SpecExecutionOptions::default(),
            analysis_line,
            numeric_override: None,
        });
    }
    if errors.is_empty() {
        Ok(tasks)
    } else {
        Err(errors)
    }
}
