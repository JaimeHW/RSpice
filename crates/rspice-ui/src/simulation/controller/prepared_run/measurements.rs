//! Bind explicitly authored plan measurements to the sealed generated source.

use std::collections::{HashMap, HashSet};

use crate::product::AnalysisInstanceId;
use crate::simulation::execution::{PreparationError, PreparationStage};
use crate::simulation::plan::AnalysisDraft;
use crate::state::SpecificationDefinition;

fn failure(message: impl Into<String>) -> PreparationError {
    PreparationError::new(PreparationStage::AnalysisPlan, message)
}

/// Families whose Studio executors evaluate native `.MEAS` statements.
/// Study owners inherit their exact configured base's family.
fn family(
    mut id: AnalysisInstanceId,
    drafts: &HashMap<AnalysisInstanceId, &AnalysisDraft>,
) -> Option<&'static str> {
    let mut visited = HashSet::new();
    while visited.insert(id) {
        match *drafts.get(&id)? {
            AnalysisDraft::Transient(_) => return Some("TRAN"),
            AnalysisDraft::Ac(_) => return Some("AC"),
            AnalysisDraft::DcSweep(_) => return Some("DC"),
            AnalysisDraft::Noise(_) => return Some("NOISE"),
            AnalysisDraft::MonteCarlo(config) => id = config.base_analysis?,
            AnalysisDraft::Optimization(config) => id = config.base_analysis?,
            _ => return None,
        }
    }
    None
}

pub(super) fn materialize(
    source: &str,
    definitions: &[SpecificationDefinition],
    drafts: &HashMap<AnalysisInstanceId, &AnalysisDraft>,
) -> Result<String, PreparationError> {
    if !definitions
        .iter()
        .any(|definition| definition.define_measurement)
    {
        return Ok(source.to_owned());
    }
    let parse_options = rspice_core::netlist::NetlistParseOptions {
        statistical_mode: rspice_core::netlist::StatisticalParamMode::Nominal,
        ..Default::default()
    };
    let parsed = rspice_core::Netlist::parse_with_options(source, parse_options)
        .map_err(|error| failure(format!("Cannot prepare plan measurements: {error}")))?;
    let names = parsed
        .measurements
        .iter()
        .map(|measurement| measurement.name.to_ascii_lowercase())
        .collect();
    let source = append_statements(source, definitions, drafts, names)?;
    // Use the design's actual parameter and expression context.
    rspice_core::Netlist::parse_with_options(&source, parse_options)
        .map_err(|error| failure(format!("Invalid authored plan measurement: {error}")))?;
    Ok(source)
}

pub(super) fn append_to_generated_source(
    source: &str,
    definitions: &[SpecificationDefinition],
    drafts: &HashMap<AnalysisInstanceId, &AnalysisDraft>,
) -> Result<String, PreparationError> {
    let names = super::executable_logical_lines(source)
        .into_iter()
        .filter_map(|(_, line)| {
            let mut tokens = line.split_whitespace();
            let head = tokens.next()?;
            if !(head.eq_ignore_ascii_case(".MEAS") || head.eq_ignore_ascii_case(".MEASURE")) {
                return None;
            }
            tokens.next()?;
            tokens.next().map(str::to_ascii_lowercase)
        })
        .collect();
    append_statements(source, definitions, drafts, names)
}

fn append_statements(
    source: &str,
    definitions: &[SpecificationDefinition],
    drafts: &HashMap<AnalysisInstanceId, &AnalysisDraft>,
    mut names: HashSet<String>,
) -> Result<String, PreparationError> {
    let mut statements = Vec::new();
    for definition in definitions {
        definition.validate().map_err(failure)?;
        let Some(statement) = definition.measurement_statement().map_err(failure)? else {
            continue;
        };
        let declared_family = statement.split_whitespace().nth(1).unwrap_or_default();
        let compatible = if let Some(producer) = definition.producing_analysis {
            family(producer, drafts)
                .is_some_and(|family| family.eq_ignore_ascii_case(declared_family))
        } else {
            drafts.keys().any(|id| {
                family(*id, drafts)
                    .is_some_and(|family| family.eq_ignore_ascii_case(declared_family))
            })
        };
        if !compatible {
            return Err(failure(format!(
                "Measurement {:?} requires an enabled {declared_family} producer that evaluates .MEAS statements; select the matching analysis or reference an existing scalar result",
                definition.measurement
            )));
        }
        if !names.insert(definition.measurement.to_ascii_lowercase()) {
            return Err(failure(format!(
                "Measurement {:?} is already defined in the design or plan; reference the existing result or choose another measurement name",
                definition.measurement
            )));
        }
        statements.push(statement);
    }
    if statements.is_empty() {
        return Ok(source.to_owned());
    }
    Ok(
        crate::services::simulation_runner::splice_before_terminal_end_card(
            source,
            &statements.join("\n"),
        ),
    )
}

#[cfg(test)]
mod tests;
