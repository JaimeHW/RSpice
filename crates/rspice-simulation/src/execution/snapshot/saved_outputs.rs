//! Saved-output ownership and exact-deck preparation for immutable tasks.
//! Binding follows task expansion so each override supplies its own semantics.

use super::*;

/// Bind each task's saved outputs to the deck it executes.
///
/// A Run Set repeats a few decks many times: every analysis at a point runs
/// its point's deck, and points that differ only in temperature or supply
/// share one. Each distinct deck is parsed once, however many tasks run it;
/// on a PDK whose model cards make a parse cost seconds, parsing it again for
/// every task was most of what preparing the run cost.
///
/// It is still the stage that costs most, so it is the one that reports: a
/// point counts as prepared once the first task at it is bound, which is when
/// its deck has been parsed. `abort` is honoured before each task.
pub(super) fn bind_decks(
    tasks: &mut [PreparedTask],
    parsed_netlist: &rspice_core::Netlist,
    abort: &dyn rspice_core::abort_signal::AbortSignal,
    progress: &dyn Fn(PreparationProgress),
) -> Result<(), PreparationError> {
    let total = {
        let mut points = Vec::new();
        for point in tasks.iter().filter_map(|task| task.pvt_point.as_ref()) {
            if !points.contains(&point) {
                points.push(point);
            }
        }
        points.len()
    };
    progress(PreparationProgress { prepared: 0, total });
    let mut prepared = Vec::with_capacity(total);
    let mut parsed_overrides: HashMap<&str, rspice_core::Netlist> = HashMap::new();
    for task in tasks {
        let PreparedTask {
            saved_output_contracts,
            executable_netlist_override,
            pvt_point,
            ..
        } = task;
        let executable_netlist_override: &Option<String> = executable_netlist_override;
        let pvt_point: &Option<_> = pvt_point;
        if !saved_output_contracts.is_empty() {
            PreparationError::check_abort(abort)?;
            let source = match executable_netlist_override.as_deref() {
                Some(text) => match parsed_overrides.entry(text) {
                    std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(rspice_core::Netlist::parse(text).map_err(|error| {
                            PreparationError::new(PreparationStage::Netlist, error.to_string())
                        })?)
                    }
                },
                None => parsed_netlist,
            };
            PreparedSavedOutput::bind_deck(saved_output_contracts, source)
                .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
        }
        if let Some(point) = pvt_point
            && !prepared.contains(&point)
        {
            prepared.push(point);
            progress(PreparationProgress {
                prepared: prepared.len(),
                total,
            });
        }
    }
    Ok(())
}

pub(super) fn validate_task(task: &PreparedTask) -> Result<(), PreparationError> {
    let mut output_ids = HashSet::with_capacity(task.saved_output_contracts.len());
    let mut output_names = HashSet::with_capacity(task.saved_output_contracts.len());
    let mut output_digests = HashSet::with_capacity(task.saved_output_contracts.len());
    for contract in &task.saved_output_contracts {
        if contract.analysis_id() != task.instance_id {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Prepared saved output {} targets analysis {}, not its owning task {}",
                    contract.output_id(),
                    contract.analysis_id(),
                    task.instance_id
                ),
            ));
        }
        if !output_ids.insert(contract.output_id())
            || !output_names.insert(contract.name())
            || !output_digests.insert(contract.digest())
        {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Prepared task {} contains duplicate saved-output identity, name, or contract digest",
                    task.instance_id
                ),
            ));
        }
    }
    Ok(())
}
