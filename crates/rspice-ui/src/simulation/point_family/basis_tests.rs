//! A family's signal basis must describe every retained point without invention.

use super::*;
use crate::services::simulation_runner::{CornerFrequencySweep, CornerRunConfig, TempRunConfig};
use crate::state::{AnalysisType, SimulationState};

const CONDITIONAL: &str = "Conditional family\n\
    VDD vdd 0 DC 1.8 AC 1\n\
    VBIAS bias 0 0\n\
    R1 vdd out 1k\n\
    R2 out 0 1k\n\
    .if (TEMPER < 50)\n\
    R3 vdd extra 1k\n\
    R4 extra 0 1k\n\
    .endif\n\
    .end\n";

const REORDERED: &str = "Reordered family with an actual zero\n\
    VDD vdd 0 DC 1.8 AC 1\n\
    VBIAS bias 0 0\n\
    .if (TEMPER < 50)\n\
    VA a 0 DC 0 AC 0\n\
    VB b 0 DC 2 AC 2\n\
    .else\n\
    VB b 0 DC 2 AC 2\n\
    VA a 0 DC 0 AC 0\n\
    .endif\n\
    .end\n";

fn base_modes() -> [CornerBaseMode; 6] {
    [
        CornerBaseMode::Op,
        CornerBaseMode::DcSweep {
            source_name: "VDD".to_owned(),
            start: 0.0,
            stop: 1.8,
            step: 0.9,
        },
        CornerBaseMode::DcSweepNested {
            source_name: "VDD".to_owned(),
            start: 0.0,
            stop: 1.8,
            step: 0.9,
            source2: "VBIAS".to_owned(),
            start2: 0.0,
            stop2: 0.1,
            step2: 0.1,
        },
        CornerBaseMode::Transient {
            stop_time: 1.0e-6,
            step_time: 1.0e-8,
        },
        CornerBaseMode::TransientWindow {
            stop_time: 1.0e-6,
            step_time: 1.0e-8,
            start_time: 0.5e-6,
            max_timestep: Some(1.0e-8),
            uic: false,
        },
        CornerBaseMode::Ac {
            start_freq: 1.0e3,
            stop_freq: 1.0e4,
            points_per_unit: 2,
            sweep: CornerFrequencySweep::Decade,
        },
    ]
}

fn run(
    family: AnalysisType,
    deck: &str,
    temperatures: [f64; 2],
    base_mode: CornerBaseMode,
) -> SimulationRun {
    use crate::simulation::runner::pvt_point_evidence;

    let result = match family {
        AnalysisType::Parametric => pvt_point_evidence::run_temperature_declaration(
            deck,
            TempRunConfig {
                temperatures_c: temperatures.to_vec(),
                base_mode,
            },
            27.0,
        ),
        AnalysisType::Corner => pvt_point_evidence::run_corner_declaration(
            deck,
            CornerRunConfig {
                temperatures_c: temperatures.to_vec(),
                voltages: vec![1.8],
                nominal_voltage: Some(1.8),
                supply_source_names: vec!["VDD".to_owned()],
                base_mode,
                ..Default::default()
            },
            27.0,
        ),
        _ => unreachable!(),
    };
    let result = result.expect("the real PVT declaration prepares, authorizes, and executes");
    assert_eq!(result.analyses.len(), 3);
    result
}

fn assert_basis_failure(temperatures: [f64; 2]) {
    for family in [AnalysisType::Parametric, AnalysisType::Corner] {
        for base in base_modes() {
            let run = run(family, CONDITIONAL, temperatures, base.clone());
            for (index, temperature) in temperatures.into_iter().enumerate() {
                let point = &run.analyses[index];
                assert!(
                    point.success,
                    "{family:?} {base:?}: {:?}",
                    point.error_message
                );
                let values =
                    point_node_values(point, &base).expect("the point retains node values");
                assert_eq!(
                    values.iter().any(|(name, _)| name.contains("EXTRA")),
                    temperature < 50.0,
                    "conditional topology really differs at the two executed points"
                );
            }
            let summary = &run.analyses[2];
            assert_eq!(summary.analysis_type, family);
            assert!(
                !summary.success,
                "{family:?} {base:?} must not fabricate zeros or drop later signals: {:?}",
                summary.waveforms
            );
            assert!(summary.waveforms.is_empty());
            assert!(
                summary
                    .error_message
                    .as_deref()
                    .unwrap()
                    .contains("point 2 changed the solved node basis")
            );
            run.validate_provenance()
                .expect("the failed family still answers its authorized task");

            let mut state = SimulationState::default();
            state.next_run_id = run.id;
            state.runs = vec![run].into();
            let persisted = crate::io::project_io::ProjectSimulationResults::from_state(&state);
            persisted
                .validate()
                .expect("valid individual points remain saveable");
            let mut restored = SimulationState::default();
            persisted.apply_to_state(&mut restored).unwrap();
            let restored = &restored.runs[0];
            assert_eq!(restored.analyses.len(), 3);
            assert!(restored.analyses[..2].iter().all(|point| point.success));
            assert!(!restored.analyses[2].success);
        }
    }
}

#[test]
fn pvt_family_rejects_a_node_missing_from_a_later_successful_point() {
    assert_basis_failure([27.0, 85.0]);
}

#[test]
fn pvt_family_rejects_a_node_added_by_a_later_successful_point() {
    assert_basis_failure([85.0, 27.0]);
}

#[test]
fn pvt_family_reorders_matching_nodes_and_preserves_physical_zero() {
    for family in [AnalysisType::Parametric, AnalysisType::Corner] {
        for base in base_modes() {
            let run = run(family, REORDERED, [27.0, 85.0], base.clone());
            assert!(run.analyses.iter().all(|point| point.success));
            let summary = &run.analyses[2];
            for (node, expected) in [("A", 0.0), ("B", 2.0)] {
                let name = if matches!(base, CornerBaseMode::Ac { .. }) {
                    format!("|V({node})|")
                } else {
                    format!("V({node})")
                };
                let trace = summary
                    .waveforms
                    .iter()
                    .find(|trace| trace.name == name)
                    .unwrap_or_else(|| {
                        panic!(
                            "{family:?} {base:?}: missing {name} in {:?}",
                            summary.waveforms
                        )
                    });
                assert_eq!(
                    trace.y.as_slice(),
                    &[expected, expected],
                    "{family:?} {base:?} {node}"
                );
            }
            if matches!(base, CornerBaseMode::Op) {
                let order = |point: &AnalysisResult| {
                    let Some(AnalysisResultPayload::OperatingPoint { mna_node_names, .. }) =
                        &point.result_payload
                    else {
                        unreachable!()
                    };
                    mna_node_names.clone()
                };
                assert_ne!(
                    order(&run.analyses[0]),
                    order(&run.analyses[1]),
                    "the core really changed its node ordering"
                );
            }
        }
    }
}
