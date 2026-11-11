//! The safe-operating-area verdict as a row states it.
//!
//! An instance's Checks row and the Verify flow's device-safety step state
//! the same verdict, read the same way, so neither feature owns it.

use rspice_ui_kit::panels::Problem;

use super::shared::count_label;
use crate::state::{AnalysisResultPayload, SoaRuleVerdictEvidence};

/// A safe-operating-area verdict as a row states it: the worst margin, or the
/// violation count, with a problem's mark when the stress is near or past a
/// limit. A pass is neutral.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentSoaDisplay {
    pub label: String,
    pub problem: Option<Problem>,
}

/// The worst verdict one SOA analysis holds across every device it checked,
/// or `None` when the analysis carries no evaluations to judge.
pub fn analysis_soa_display(
    analysis: &crate::state::AnalysisResult,
) -> Option<ComponentSoaDisplay> {
    let Some(AnalysisResultPayload::Soa {
        evaluations,
        violations,
        ..
    }) = analysis.result_payload.as_ref()
    else {
        return None;
    };
    component_soa_display(&evaluations.iter().collect::<Vec<_>>(), violations.len())
}

/// The verdict a set of evaluations and violations amounts to: the violation
/// count when any limit was crossed, else the worst margin. `None` when there
/// is nothing to judge.
pub fn component_soa_display(
    evaluations: &[&crate::state::SoaEvaluationEvidence],
    violations: usize,
) -> Option<ComponentSoaDisplay> {
    if violations > 0 {
        return Some(ComponentSoaDisplay {
            label: count_label(violations, "violation"),
            problem: Some(Problem::Failure),
        });
    }
    if evaluations.is_empty() {
        return None;
    }

    let verdict = evaluations
        .iter()
        .map(|evaluation| evaluation.verdict)
        .max()
        .unwrap_or(SoaRuleVerdictEvidence::Pass);
    let margin_percent = evaluations
        .iter()
        .filter_map(|evaluation| {
            let limit = evaluation.limit_value.abs();
            (limit > f64::EPSILON)
                .then_some((limit - evaluation.worst_actual_value.abs()) / limit * 100.0)
        })
        .min_by(f64::total_cmp);
    let (problem, without_margin) = match verdict {
        SoaRuleVerdictEvidence::Pass => (None, "within limits"),
        SoaRuleVerdictEvidence::Warning => (Some(Problem::Warning), "near a limit"),
        SoaRuleVerdictEvidence::Violation | SoaRuleVerdictEvidence::Critical => {
            (Some(Problem::Failure), "past a limit")
        }
    };
    Some(ComponentSoaDisplay {
        label: margin_percent.map_or_else(
            || without_margin.to_owned(),
            |margin| format!("{margin:.1}% margin"),
        ),
        problem,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soa_display_reports_real_worst_margin_and_failures() {
        let pass = crate::state::SoaEvaluationEvidence {
            duration: None,
            thresholds: Default::default(),
            envelope: None,
            derating: None,
            device_id: "M1".to_owned(),
            parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 5.0,
            worst_actual_value: 3.0,
            worst_time_s: 2.0e-6,
            sample_count: 101,
            unit: "V".to_owned(),
            description: "Drain-source voltage".to_owned(),
            verdict: SoaRuleVerdictEvidence::Pass,
        };
        let warning = crate::state::SoaEvaluationEvidence {
            duration: None,
            thresholds: Default::default(),
            envelope: None,
            derating: None,
            device_id: "M1".to_owned(),
            parameter: crate::state::SoaParameterEvidence::PowerDissipation,
            limit_value: 1.0,
            worst_actual_value: 0.75,
            worst_time_s: 3.0e-6,
            sample_count: 101,
            unit: "W".to_owned(),
            description: "Power dissipation".to_owned(),
            verdict: SoaRuleVerdictEvidence::Warning,
        };

        let display = component_soa_display(&[&pass, &warning], 0).expect("a verdict");
        assert_eq!(display.label, "25.0% margin");
        assert_eq!(display.problem, Some(Problem::Warning));

        let passed = component_soa_display(&[&pass], 0).expect("a verdict");
        assert_eq!(passed.label, "40.0% margin");
        assert_eq!(passed.problem, None, "a pass is neutral");

        let failed = component_soa_display(&[&pass], 2).expect("a verdict");
        assert_eq!(failed.label, "2 violations");
        assert_eq!(failed.problem, Some(Problem::Failure));
    }

    #[test]
    fn soa_display_does_not_invent_evidence() {
        assert_eq!(component_soa_display(&[], 0), None);
    }
}
