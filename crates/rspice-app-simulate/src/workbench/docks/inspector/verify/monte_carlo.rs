//! The Monte Carlo view's inspector subject: the failing trial the reader
//! selected, or else the campaign behind the sheet's statistics (its samples,
//! seed, sampling and duration), which the view itself never states.

use egui::Ui;
use rspice_results::yield_analysis::monte_carlo_provenance;
use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::section_header;

use crate::workbench::app_state::Model;
use crate::workbench::state::VerificationSessionState;
use crate::workbench::surfaces::verify::{FactSection, trial_facts};

/// Paint the selected trial, or else the campaign the statistics came from.
pub(super) fn show(ui: &mut Ui, model: &Model, session: &VerificationSessionState) {
    let trial = trial_facts(model, session);
    if !trial.is_empty() {
        super::point::facts(ui, &trial);
        return;
    }
    let campaigns = campaign_facts(model);
    if campaigns.is_empty() {
        section_header(ui, "Monte Carlo", None);
        muted_inspector_copy(ui, "No Monte Carlo run");
    } else {
        super::point::facts(ui, &campaigns);
    }
}

/// One section per Monte Carlo analysis whose trials the sheet's statistics
/// count, in the order they first appear; every Monte Carlo analysis of the
/// run when no specification judged its trials yet.
fn campaign_facts(model: &Model) -> Vec<FactSection> {
    let Some(run) = model.simulation.active_run() else {
        return Vec::new();
    };
    let mut analyses = Vec::new();
    if let Some(sheet) = model.simulation.verification_sheet() {
        let judged = sheet
            .rows
            .iter()
            .filter_map(|row| row.statistics.as_ref())
            .flat_map(|statistics| statistics.trials.iter().map(|trial| trial.analysis));
        for analysis in judged {
            if !analyses.contains(&analysis) {
                analyses.push(analysis);
            }
        }
    }
    if analyses.is_empty() {
        analyses = (0..run.analyses.len()).collect();
    }
    let campaigns = analyses
        .into_iter()
        .filter_map(|index| {
            let analysis = run.analyses.get(index)?;
            monte_carlo_provenance(run.run_id, run.dataset_id, analysis)
                .map(|provenance| (analysis.label.clone(), provenance))
        })
        .collect::<Vec<_>>();
    let several = campaigns.len() > 1;
    campaigns
        .into_iter()
        .map(|(label, provenance)| FactSection {
            title: if several {
                label
            } else {
                "Monte Carlo".to_owned()
            },
            rows: vec![
                (
                    "Samples".to_owned(),
                    format!(
                        "{} / {}",
                        provenance.runs_completed, provenance.runs_requested
                    ),
                ),
                ("Seed".to_owned(), format!("0x{:X}", provenance.seed)),
                (
                    "Sampling".to_owned(),
                    provenance.sampling_mode.display_name().to_owned(),
                ),
                (
                    "Duration".to_owned(),
                    super::super::compact_readout(run.elapsed_time, "s"),
                ),
            ],
        })
        .collect()
}
