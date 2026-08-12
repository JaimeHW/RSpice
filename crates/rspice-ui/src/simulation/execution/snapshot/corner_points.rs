//! Expanding a corner declaration into one prepared task per declared point.
//!
//! A corner run is a base analysis solved once at each PVT point of its
//! contract. The declaration itself is never dispatched, so everything the
//! plotting family needs that a single point cannot state is frozen onto each
//! point task here.

use super::*;

use crate::simulation::corner_family::CornerRunPoint;

/// One prepared task per point of a corner run's declared space.
///
/// The points come from the corner task's own contract rather than from the
/// run-level PVT set, because these tasks must solve exactly the points the
/// corner task itself solves; a run-level set merges the spaces of every
/// corner declaration in the run and would put a foreign point on this one.
///
/// Each point states itself three times over, and the three are not
/// interchangeable: the contract narrowed to a single point *declares* which
/// point this is and is what makes the task's configuration digest its own;
/// the deck *materializes* everything a deck can carry — the process models
/// and the temperature card; and the supply scale is applied to the elaborated
/// circuit at execution, because a supply corner multiplies existing source
/// values and no card can express that.
pub(super) fn expand_corner_run_point_tasks(
    corner_task: &PreparedTask,
    executable_netlist: &str,
    reference_process: ProcessCorner,
    reference_temperature_celsius: f64,
) -> Result<Vec<PreparedTask>, PreparationError> {
    let default_contract;
    let contract = match corner_task.task.spec_options.corner.as_ref() {
        Some(contract) => contract,
        None => {
            default_contract = crate::services::simulation_runner::CornerRunConfig::default();
            &default_contract
        }
    };
    contract.validate().map_err(|error| {
        PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!("Corner PVT contract is invalid: {error}"),
        )
    })?;
    let points = crate::services::simulation_runner::expand_corner_pvt_points(contract).map_err(
        |error| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!("Corner PVT expansion failed: {error}"),
            )
        },
    )?;
    if points.is_empty() {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            "Corner run declared no PVT execution points",
        ));
    }

    let contract_digest = corner_contract_digest(contract);
    let count = points.len();
    let mut point_tasks = Vec::with_capacity(count);
    for (index, (process, voltage, temperature_celsius)) in points.into_iter().enumerate() {
        let point = PreparedPvtPoint {
            process: process_from_corner_runner(process),
            voltage: Some(voltage),
            temperature_celsius,
            corner_contract: Some(contract.clone()),
        };
        let (source_override, nominal_supply_voltage) =
            prepare_pvt_point_source(executable_netlist, &point)?;

        let mut point_task = corner_task.clone();
        point_task.instance_id = AnalysisInstanceId::from_namespace(
            corner_task.instance_id.as_uuid(),
            format!(
                "rspice-corner-run-point/v1/{index}/{count}/{}/{:016x}/{:016x}/{contract_digest}",
                process_tag(point.process),
                voltage.to_bits(),
                temperature_celsius.to_bits(),
            )
            .as_bytes(),
        );
        point_task.pvt_point = Some(
            crate::state::AnalysisResultPvtPoint::new(
                point.process.short_name(),
                point.voltage,
                point.temperature_celsius,
                Some(contract_digest),
                point_is_nominal(
                    &point,
                    nominal_supply_voltage,
                    reference_process,
                    reference_temperature_celsius,
                ),
            )
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Corner run point {}/{count} cannot be attributed: {error}",
                        index + 1
                    ),
                )
            })?,
        );
        point_task.label = format!(
            "{} \u{00b7} {} \u{00b7} {voltage} V \u{00b7} {temperature_celsius} \u{00b0}C",
            corner_task.label,
            point.process.short_name(),
        );
        point_task.corner_point = Some(CornerRunPoint::new(
            contract.base_mode.clone(),
            count,
            index,
            crate::services::simulation_runner::CornerPoint {
                process,
                voltage,
                temperature_c: temperature_celsius,
            },
        ));

        // The temperature is written into the deck rather than handed to the
        // engine beside it, so the point's own bytes state the condition it
        // was solved at. It also reaches deck expressions in `TEMPER`, which a
        // configuration field never did.
        let deck = source_override.unwrap_or_else(|| executable_netlist.to_owned());
        point_task.executable_netlist_override = Some(splice_before_terminal_end_card(
            &deck,
            &format!(".OPTIONS TEMP={temperature_celsius}"),
        ));

        let (spec, config, analysis_line) = corner_point_request(
            &contract.base_mode,
            &point,
            index,
            count,
            nominal_supply_voltage,
        );
        spec.validate().map_err(|error| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Corner run point {}/{count} base analysis is invalid: {error}",
                    index + 1
                ),
            )
        })?;
        point_task.task.spec = spec;
        point_task.task.config = config;
        point_task.task.analysis_line = analysis_line;
        point_task.task.spec_options.corner =
            Some(crate::services::simulation_runner::CornerRunConfig {
                nominal_voltage: nominal_supply_voltage,
                points: vec![crate::services::simulation_runner::CornerPoint {
                    process,
                    voltage,
                    temperature_c: temperature_celsius,
                }],
                ..contract.clone()
            });

        point_task.saved_output_contracts = corner_task
            .saved_output_contracts
            .iter()
            .map(|saved| {
                saved
                    .rebind_analysis(point_task.instance_id, &point_task.task.spec)
                    .map_err(|error| {
                        PreparationError::new(
                            PreparationStage::AnalysisPlan,
                            format!(
                                "Failed to bind saved output to corner run point {}/{count}: {error}",
                                index + 1
                            ),
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        point_task.config_digest = point_task.payload_digest();
        point_tasks.push(point_task);
    }

    Ok(point_tasks)
}

/// The base analysis a corner point actually runs.
///
/// The operating point takes its temperature and supply from its own typed
/// run-point contract, because that contract is the only channel its engine
/// build reads. The other three have no such field: their temperature is
/// already in the point's deck, and their supply is applied from the narrowed
/// corner contract that travels with the request.
fn corner_point_request(
    base_mode: &crate::services::simulation_runner::CornerBaseMode,
    point: &PreparedPvtPoint,
    index: usize,
    count: usize,
    nominal_supply_voltage: Option<f64>,
) -> (
    AnalysisSpec,
    Option<crate::simulation::AnalysisConfig>,
    String,
) {
    use crate::services::simulation_runner::{CornerBaseMode, CornerFrequencySweep};
    use crate::simulation::dialog::{OpConfig, OpRunPointContext, OpTemperatureMode};
    use crate::simulation::multi_run::FrequencySweep;

    match base_mode {
        CornerBaseMode::Op => {
            let config = OpConfig {
                temperature_mode: OpTemperatureMode::Explicit,
                temperature_celsius: point.temperature_celsius,
                run_point: OpRunPointContext {
                    index,
                    count,
                    process: point.process,
                    supply_voltage: point.voltage,
                    nominal_supply_voltage,
                },
                ..OpConfig::default()
            };
            let spec = operating_point_spec(&config);
            (
                spec,
                Some(crate::simulation::AnalysisConfig::DcOp(config)),
                ".op".to_owned(),
            )
        }
        CornerBaseMode::DcSweep {
            source_name,
            start,
            stop,
            step,
        } => (
            AnalysisSpec::DcSweep {
                source_name: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                source2: None,
                start2: None,
                stop2: None,
                step2: None,
            },
            None,
            format!(".dc {source_name} {start} {stop} {step}"),
        ),
        CornerBaseMode::Transient {
            stop_time,
            step_time,
        } => (
            AnalysisSpec::Transient {
                stop_time: *stop_time,
                step_time: *step_time,
                start_time: 0.0,
                max_timestep: None,
                uic: false,
            },
            None,
            format!(".tran {step_time} {stop_time}"),
        ),
        CornerBaseMode::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => {
            let (sweep, keyword) = match sweep {
                CornerFrequencySweep::Decade => (FrequencySweep::Decade, "dec"),
                CornerFrequencySweep::Octave => (FrequencySweep::Octave, "oct"),
                CornerFrequencySweep::Linear => (FrequencySweep::Linear, "lin"),
            };
            (
                AnalysisSpec::Ac {
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    points_per_unit: *points_per_unit,
                    sweep,
                },
                None,
                format!(".ac {keyword} {points_per_unit} {start_freq} {stop_freq}"),
            )
        }
    }
}
