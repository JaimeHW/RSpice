//! Saved-output ownership and exact-deck preparation for immutable tasks.
//! Binding follows task expansion so each override supplies its own semantics.

use super::*;

pub(super) fn bind_decks(
    tasks: &mut [PreparedTask],
    parsed_netlist: &rspice_core::Netlist,
) -> Result<(), PreparationError> {
    for task in tasks {
        if task.saved_output_contracts.is_empty() {
            continue;
        }
        let override_netlist;
        let source = if let Some(text) = &task.executable_netlist_override {
            override_netlist = rspice_core::Netlist::parse(text).map_err(|error| {
                PreparationError::new(PreparationStage::Netlist, error.to_string())
            })?;
            &override_netlist
        } else {
            parsed_netlist
        };
        PreparedSavedOutput::bind_deck(&mut task.saved_output_contracts, source)
            .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
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
