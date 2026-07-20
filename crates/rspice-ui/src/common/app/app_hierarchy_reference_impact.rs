//! Fail-closed impact analysis for unscoped simulation references.
//!
//! Simulation plans currently do not own an explicit cell/view reference
//! root. Moving a device or internal net below a new hierarchy instance makes
//! an automatic rewrite ambiguous: the old spelling remains correct in the
//! child but requires a hierarchy prefix in the parent. Until plans carry that
//! authority, Create hierarchy rejects affected references without modifying
//! plans, retained runs, or regression evidence.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::product::SimulationPlanId;
use crate::simulation::plan::{AnalysisKind, SimulationPlan};
use crate::state::{
    AnalysisResultSourceDomain, HierarchyExtractionPlan, RegressionTargetKind,
    RegressionTargetSelector, SavedOutput, SavedOutputCompatibility, SavedOutputKind,
};

use super::AppState;

pub(super) fn validate_hierarchy_reference_impact(
    state: &AppState,
    extraction: &HierarchyExtractionPlan,
) -> Result<(), String> {
    let affected = extraction
        .source_instance_names
        .iter()
        .map(String::as_str)
        .chain(extraction.internal_source_net_names())
        .filter(|name| !name.trim().is_empty())
        .map(|name| (name.to_ascii_lowercase(), name.to_owned()))
        .collect::<BTreeMap<_, _>>();
    if affected.is_empty() {
        return Ok(());
    }

    let mut findings = Vec::new();
    if let Some(plan) = state.sim_setup.analysis_plan.as_ref() {
        scan_plan(
            state.sim_setup.active_plan_name().as_str(),
            plan,
            &affected,
            &mut findings,
        );
    }
    for stored in state.sim_setup.inactive_plans() {
        scan_plan(
            stored.name().as_str(),
            stored.analysis_plan(),
            &affected,
            &mut findings,
        );
    }
    for record in &state.workspace.simulation_plan_payloads {
        let label = plan_label(state, record.plan_id);
        for (index, output) in record.payload.saved_outputs.iter().enumerate() {
            if let Some(symbol) = affected_reference(&output.source_expression, &affected) {
                findings.push(format!(
                    "{label}, saved output '{}' ({}), source_expression references '{symbol}'",
                    output.name,
                    index + 1
                ));
            }
        }
        let plan = simulation_plan(state, record.plan_id);
        for (index, tolerance) in record.payload.regression_tolerances.iter().enumerate() {
            if tolerance.target.kind != RegressionTargetKind::Waveform
                || record
                    .payload
                    .saved_outputs
                    .iter()
                    .any(|output| saved_output_aliases_target(output, &tolerance.target, plan))
            {
                continue;
            }
            if let Some(symbol) = affected_reference(&tolerance.target.name, &affected) {
                findings.push(format!(
                    "{label}, regression target {} references '{symbol}'",
                    index + 1
                ));
            }
        }
    }

    findings.sort();
    findings.dedup();
    if findings.is_empty() {
        Ok(())
    } else {
        const LIMIT: usize = 6;
        let shown = findings.iter().take(LIMIT).cloned().collect::<Vec<_>>();
        let remainder = findings.len().saturating_sub(shown.len());
        Err(format!(
            "Create hierarchy cannot move referenced devices or internal nets while simulation references are unscoped. {}{}",
            shown.join("; "),
            if remainder == 0 {
                String::new()
            } else {
                format!("; and {remainder} more")
            }
        ))
    }
}

fn scan_plan(
    name: &str,
    plan: &SimulationPlan,
    affected: &BTreeMap<String, String>,
    findings: &mut Vec<String>,
) {
    for (index, instance) in plan.instances().iter().enumerate() {
        let Ok(value) = serde_json::to_value(instance.draft()) else {
            findings.push(format!(
                "plan '{name}', analysis {} could not be inspected",
                index + 1
            ));
            continue;
        };
        scan_value(
            &value,
            &mut vec![format!("analysis {}", index + 1)],
            affected,
            &mut |path, symbol| {
                findings.push(format!(
                    "plan '{name}', {} references '{symbol}'",
                    path.join(".")
                ));
            },
        );
    }
}

fn scan_value(
    value: &Value,
    path: &mut Vec<String>,
    affected: &BTreeMap<String, String>,
    finding: &mut impl FnMut(&[String], &str),
) {
    match value {
        Value::Object(entries) => {
            for (key, value) in entries {
                path.push(key.clone());
                scan_value(value, path, affected, finding);
                path.pop();
            }
        }
        Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                path.push((index + 1).to_string());
                scan_value(value, path, affected, finding);
                path.pop();
            }
        }
        Value::String(text) if reference_field(path) => {
            if let Some(symbol) = affected_reference(text, affected) {
                finding(path, symbol);
            }
        }
        _ => {}
    }
}

fn reference_field(path: &[String]) -> bool {
    let Some(field) = path
        .iter()
        .rev()
        .find(|segment| !segment.bytes().all(|byte| byte.is_ascii_digit()))
        .map(|field| field.to_ascii_lowercase())
    else {
        return false;
    };
    matches!(
        field.as_str(),
        "source"
            | "source2"
            | "input"
            | "output"
            | "reference"
            | "probe"
            | "save_nodes"
            | "input_pos"
            | "input_neg"
            | "output_pos"
            | "output_neg"
            | "node_pos"
            | "node_neg"
    ) || field.ends_with("_node")
        || field.ends_with("_ref")
        || field.ends_with("_source")
        || field.ends_with("_expression")
        || field.ends_with("_expr")
}

fn affected_reference<'a>(value: &str, affected: &'a BTreeMap<String, String>) -> Option<&'a str> {
    let lower = value.to_ascii_lowercase();
    let mut first = None::<(usize, &str, &str)>;
    for (canonical, original) in affected {
        for (start, _) in lower.match_indices(canonical) {
            let end = start + canonical.len();
            let before = (start > 0).then(|| lower.as_bytes()[start - 1]);
            let after = (end < lower.len()).then(|| lower.as_bytes()[end]);
            if before == Some(b'.')
                || before.is_some_and(identifier_byte)
                || after.is_some_and(identifier_byte)
            {
                continue;
            }
            let candidate = (start, canonical.as_str(), original.as_str());
            if first.is_none_or(|current| {
                candidate.0 < current.0 || (candidate.0 == current.0 && candidate.1 < current.1)
            }) {
                first = Some(candidate);
            }
        }
    }
    first.map(|(_, _, original)| original)
}

const fn identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$'
}

fn simulation_plan(state: &AppState, id: SimulationPlanId) -> Option<&SimulationPlan> {
    state
        .sim_setup
        .analysis_plan
        .as_ref()
        .filter(|plan| plan.id() == id)
        .or_else(|| {
            state
                .sim_setup
                .inactive_plans()
                .iter()
                .find(|stored| stored.id() == id)
                .map(|stored| stored.analysis_plan())
        })
}

fn saved_output_aliases_target(
    output: &SavedOutput,
    target: &RegressionTargetSelector,
    plan: Option<&SimulationPlan>,
) -> bool {
    if target.kind != RegressionTargetKind::Waveform
        || target.source_domain != AnalysisResultSourceDomain::SimulationPlan
        || target.occurrence != 0
        || !output.name.eq_ignore_ascii_case(&target.name)
    {
        return false;
    }
    let Some(instance) = plan.and_then(|plan| {
        plan.instances()
            .iter()
            .find(|instance| instance.id() == target.source_instance_id)
    }) else {
        return false;
    };
    if !saved_output_kind_supports_analysis(output.kind, instance.kind()) {
        return false;
    }
    match &output.compatible_analyses {
        SavedOutputCompatibility::OpTranAc => matches!(
            instance.kind(),
            AnalysisKind::OperatingPoint | AnalysisKind::Transient | AnalysisKind::Ac
        ),
        SavedOutputCompatibility::AllCompatibleAnalyses => true,
        SavedOutputCompatibility::SelectedAnalysis { analysis_id } => {
            *analysis_id == target.source_instance_id
        }
    }
}

const fn saved_output_kind_supports_analysis(
    output: SavedOutputKind,
    analysis: AnalysisKind,
) -> bool {
    match output {
        SavedOutputKind::RawVoltageOrCurrent | SavedOutputKind::DerivedExpression => matches!(
            analysis,
            AnalysisKind::OperatingPoint
                | AnalysisKind::Transient
                | AnalysisKind::Ac
                | AnalysisKind::DcSweep
                | AnalysisKind::Noise
                | AnalysisKind::MonteCarlo
                | AnalysisKind::Pss
                | AnalysisKind::Temperature
                | AnalysisKind::HarmonicBalance
                | AnalysisKind::SParameter
                | AnalysisKind::Pac
                | AnalysisKind::Pnoise
                | AnalysisKind::Pxf
                | AnalysisKind::Corner
                | AnalysisKind::Envelope
                | AnalysisKind::Fourier
                | AnalysisKind::Reliability
                | AnalysisKind::Optimization
                | AnalysisKind::Soa
                | AnalysisKind::Qpss
                | AnalysisKind::TransientNoise
        ),
        SavedOutputKind::DeviceOperatingPointQuantity => {
            matches!(analysis, AnalysisKind::OperatingPoint)
        }
        SavedOutputKind::NoiseContributor => matches!(
            analysis,
            AnalysisKind::Noise
                | AnalysisKind::Pnoise
                | AnalysisKind::Qpnoise
                | AnalysisKind::Hbnoise
                | AnalysisKind::TransientNoise
        ),
        SavedOutputKind::RfPortQuantity => matches!(
            analysis,
            AnalysisKind::SParameter | AnalysisKind::Hbsp | AnalysisKind::Psp
        ),
    }
}

fn plan_label(state: &AppState, id: SimulationPlanId) -> String {
    if state
        .sim_setup
        .analysis_plan
        .as_ref()
        .is_some_and(|plan| plan.id() == id)
    {
        return format!("plan '{}'", state.sim_setup.active_plan_name().as_str());
    }
    state
        .sim_setup
        .inactive_plans()
        .iter()
        .find(|plan| plan.id() == id)
        .map_or_else(
            || format!("plan {id}"),
            |plan| format!("plan '{}'", plan.name().as_str()),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ObjectRevision};
    use crate::simulation::plan::{AnalysisDraft, AnalysisInstance};
    use crate::state::{SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming};

    #[test]
    fn semantic_token_matching_does_not_use_substrings() {
        let affected = BTreeMap::from([("r1".to_owned(), "R1".to_owned())]);
        assert_eq!(affected_reference("I(R1)", &affected), Some("R1"));
        assert_eq!(affected_reference("@R1[id]", &affected), Some("R1"));
        assert_eq!(affected_reference("I(R10)", &affected), None);
        assert_eq!(affected_reference("parameter_r1x", &affected), None);
    }

    #[test]
    fn qualified_hierarchical_leaf_does_not_match_same_named_top_level_instance() {
        let affected = BTreeMap::from([("r1".to_owned(), "R1".to_owned())]);
        assert_eq!(affected_reference("I(Xother.R1)", &affected), None);
        assert_eq!(affected_reference("V(Xother.R1)", &affected), None);
        assert_eq!(affected_reference("I(R1)", &affected), Some("R1"));
    }

    #[test]
    fn multiple_affected_references_report_first_source_occurrence_deterministically() {
        let affected = BTreeMap::from([
            ("r1".to_owned(), "R1".to_owned()),
            ("r2".to_owned(), "R2".to_owned()),
        ]);
        assert_eq!(affected_reference("I(R2) + I(R1)", &affected), Some("R2"));
        assert_eq!(affected_reference("I(R1) + I(R2)", &affected), Some("R1"));
    }

    #[test]
    fn selected_analysis_alias_does_not_hide_another_analysis_waveform() {
        let first_id = AnalysisInstanceId::new();
        let second_id = AnalysisInstanceId::new();
        let instances = [first_id, second_id]
            .into_iter()
            .map(|id| {
                AnalysisInstance::supplied(
                    id,
                    AnalysisKind::Transient,
                    AnalysisDraft::for_kind(AnalysisKind::Transient),
                    true,
                    Vec::new(),
                    ObjectRevision::INITIAL,
                    ObjectRevision::INITIAL,
                )
                .expect("valid transient analysis")
            })
            .collect();
        let plan = SimulationPlan::from_ordered_instances(
            SimulationPlanId::new(),
            ObjectRevision::INITIAL,
            instances,
        )
        .expect("valid two-analysis plan");
        let output = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            "I(R1)",
            "V(out)",
            SavedOutputCompatibility::SelectedAnalysis {
                analysis_id: first_id,
            },
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("valid saved output");
        let target = |source_instance_id| RegressionTargetSelector {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            source_instance_id,
            kind: RegressionTargetKind::Waveform,
            name: "I(R1)".to_owned(),
            occurrence: 0,
        };

        assert!(saved_output_aliases_target(
            &output,
            &target(first_id),
            Some(&plan)
        ));
        assert!(!saved_output_aliases_target(
            &output,
            &target(second_id),
            Some(&plan)
        ));
    }
}
