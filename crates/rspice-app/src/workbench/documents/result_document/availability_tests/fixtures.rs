//! The evidence each Results sheet is shown, and the derivations that
//! produce a cache a sheet reads.
//!
//! Split out of `availability_tests` when that file crossed the line budget:
//! the seam is the one the tests already used — this module answers "what
//! does a sheet get", and its parent answers "what does the sheet do with
//! it". Nothing here asserts anything.

use super::*;

pub(super) fn soa_analysis() -> AnalysisResult {
    use crate::state::{
        AnalysisResultFamilyMetadata, SoaEvaluationEvidence, SoaParameterEvidence,
        SoaRuleVerdictEvidence, SoaViolationEvidence, SoaViolationSeverityEvidence,
    };
    let time = vec![0.0, 1.0e-9];
    AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time: time.clone() })
        .with_waveforms(vec![
            RetainedTrace::new(
                "SOA_VIOLATION_COUNT",
                time.clone(),
                vec![0.0, 0.0],
                "#ffbd2e",
            ),
            // The stress history the producer now retains, named exactly as
            // the sheet addresses it.
            RetainedTrace::new(
                crate::results::safety::soa_stress_waveform_name(
                    "M1",
                    crate::results::safety::SoAParameter::Vds,
                ),
                time,
                vec![2.0, 3.0],
                "#00aaff",
            ),
        ])
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations: vec![SoaEvaluationEvidence {
                duration: None,
                thresholds: Default::default(),
                envelope: None,
                derating: None,
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                worst_actual_value: 3.0,
                worst_time_s: 1.0e-9,
                sample_count: 2,
                unit: "V".to_owned(),
                description: "Maximum drain-source voltage".to_owned(),
                // 3.0 V against a 3.3 V limit is inside the warning band,
                // which the payload validator derives rather than trusts.
                verdict: SoaRuleVerdictEvidence::Warning,
            }],
            // A non-passing rule must carry the exact event at its worst
            // point; the validator refuses a verdict with nothing behind it.
            violations: vec![SoaViolationEvidence {
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                actual_value: 3.0,
                time_s: 1.0e-9,
                severity: SoaViolationSeverityEvidence::Warning,
            }],
        })
}

pub(super) fn optimization_analysis() -> AnalysisResult {
    use crate::state::AnalysisResultFamilyMetadata;
    AnalysisResult::new(1, AnalysisType::Optimization, "Optimization")
        .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
            best_objectives: Vec::new(),
            best_constraints: Vec::new(),
            iterations: vec![0.0, 1.0],
            best_cost: 0.25,
            best_variables: [("w".to_owned(), 1.5e-6)].into_iter().collect(),
            converged: true,
        })
        // The names the optimization runner emits: one OPT_COST trace over
        // the iteration axis, plus OPT_<variable> per design variable.
        .with_waveforms(vec![
            RetainedTrace::new("OPT_COST", vec![0.0, 1.0], vec![1.0, 0.25], "#ffbd2e"),
            RetainedTrace::new("OPT_w", vec![0.0, 1.0], vec![1.0e-6, 1.5e-6], "#00aaff"),
        ])
}

/// A committed event history with all three kinds of row the Events sheet
/// draws: a scalar conductor, a real-valued event node, and two declared
/// buses over their own member traces.
///
/// The buses are here rather than in a second fixture because every gate that
/// reads this one is a gate about the sheet — the fit gate below measures the
/// radix bar and the per-row disclosures, which a history declaring no bus
/// does not paint at all.
pub(super) fn events_analysis() -> AnalysisResult {
    use crate::state::{
        DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
        DigitalEventTraceEvidence, RealEventPointEvidence, RealEventTraceEvidence,
    };
    let trace = |name: &str, points: &[(f64, u8)]| DigitalEventTraceEvidence {
        node_name: name.to_owned(),
        points: points
            .iter()
            .map(|(time_s, value_code)| DigitalEventPointEvidence {
                time_s: *time_s,
                value_code: *value_code,
            })
            .collect(),
    };
    let bus = |name: &str| DigitalBusEvidence {
        name: name.to_owned(),
        msb: 1,
        lsb: 0,
        members: vec![format!("{name}[1]"), format!("{name}[0]")],
        source: DigitalBusSourceEvidence::Import,
    };
    AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
        AnalysisResultPayload::TransientEvents {
            voltage_impulses: None,
            current_impulses: None,
            digital_traces: vec![
                trace("clk", &[(0.0, 0), (5.0e-10, 1), (1.0e-9, 12)]),
                trace("addr[1]", &[(0.0, 0), (1.0e-9, 1)]),
                trace("addr[0]", &[(0.0, 0), (5.0e-10, 1), (1.0e-9, 0)]),
                // One bit the run never states before its first change, so a
                // word the sheet cannot spell as a number is on screen too.
                trace("data[1]", &[(5.0e-10, 2)]),
                trace("data[0]", &[(0.0, 1), (1.0e-9, 12)]),
            ],
            real_traces: vec![RealEventTraceEvidence {
                node_name: "level".to_owned(),
                points: vec![RealEventPointEvidence {
                    time_s: 2.5e-10,
                    value: 0.75,
                }],
            }],
            digital_buses: vec![bus("addr"), bus("data")],
        },
    )
}
