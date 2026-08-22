//! Expanding a PVT declaration into one prepared task per declared point.
//!
//! A corner run and a temperature step are both a base analysis solved once at
//! each point of a declared space. Neither declaration is ever dispatched, so
//! everything the plotting family needs that a single point cannot state is
//! frozen onto each point task here.
//!
//! The two axes enumerate different spaces and narrow different contracts, and
//! that is all they do differently. Minting the task a point solves — its
//! derived identity, its attribution, its deck's temperature card, its base
//! analysis and its rebound saved outputs — is one procedure, because a point
//! of either space is the same kind of thing once it has been named.

use super::*;

use crate::services::simulation_runner::{CornerBaseMode, CornerRunConfig, TempRunConfig};
use crate::simulation::point_family::{DeclaredAxisPoint, DeclaredRunPoint};

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
            default_contract = CornerRunConfig::default();
            &default_contract
        }
    };
    contract.validate().map_err(|error| {
        PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!("Corner PVT contract is invalid: {error}"),
        )
    })?;
    let expanded = crate::services::simulation_runner::expand_corner_pvt_points(contract).map_err(
        |error| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!("Corner PVT expansion failed: {error}"),
            )
        },
    )?;
    if expanded.is_empty() {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            "Corner run declared no PVT execution points",
        ));
    }

    let contract_digest = corner_contract_digest(contract);
    let mut points = Vec::with_capacity(expanded.len());
    for (process, voltage, temperature_celsius) in expanded {
        let pvt = PreparedPvtPoint {
            process: process_from_corner_runner(process),
            voltage: Some(voltage),
            supply_source_names: contract.supply_source_names.clone(),
            temperature_celsius,
            parameter_overrides: Vec::new(),
            source_overrides: Vec::new(),
            run_set_point_key: None,
            corner_contract: Some(contract.clone()),
        };
        let (deck, nominal_supply_voltage) = prepare_pvt_point_source(executable_netlist, &pvt)?;
        points.push(DeclaredPoint {
            identity: format!(
                "{}/{:016x}/{:016x}/{contract_digest}",
                process_tag(pvt.process),
                voltage.to_bits(),
                temperature_celsius.to_bits(),
            ),
            label: format!(
                "{} \u{00b7} {voltage} V \u{00b7} {temperature_celsius} \u{00b0}C",
                pvt.process.short_name(),
            ),
            is_nominal: point_is_nominal(
                &pvt,
                nominal_supply_voltage,
                reference_process,
                reference_temperature_celsius,
            ),
            nominal_supply_voltage,
            deck,
            contract: NarrowedContract::Corner(CornerRunConfig {
                nominal_voltage: nominal_supply_voltage,
                points: vec![crate::services::simulation_runner::CornerPoint {
                    process,
                    voltage,
                    temperature_c: temperature_celsius,
                }],
                ..contract.clone()
            }),
            origin: DeclaredAxisPoint::Corner(crate::services::simulation_runner::CornerPoint {
                process,
                voltage,
                temperature_c: temperature_celsius,
            }),
            pvt,
        });
    }

    mint_declared_point_tasks(
        corner_task,
        executable_netlist,
        &contract.base_mode,
        "corner run",
        "rspice-corner-run-point/v1",
        points,
    )
}

/// One prepared task per temperature of a temperature step's declared space.
///
/// A temperature step declares one axis and materializes nothing: there are no
/// process models to bind and no supply to scale, so the point's whole
/// condition fits in the temperature card its deck carries. That is why it is
/// the run's reference process that gets attributed — the point really did
/// solve the reference models — and why its supply is named as absent rather
/// than as the deck's own value, which the step never touched.
pub(super) fn expand_temperature_run_point_tasks(
    temperature_task: &PreparedTask,
    contract: &TempRunConfig,
    executable_netlist: &str,
    reference_process: ProcessCorner,
    reference_temperature_celsius: f64,
) -> Result<Vec<PreparedTask>, PreparationError> {
    contract.validate().map_err(|error| {
        PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!("Temperature step contract is invalid: {error}"),
        )
    })?;
    ensure_pvt_point_capacity(
        0,
        contract.temperatures_c.len(),
        rspice_core::ResourceLimits::default().max_batch_runs,
    )?;

    let mut points = Vec::with_capacity(contract.temperatures_c.len());
    for temperature_celsius in contract.temperatures_c.iter().copied() {
        let pvt = PreparedPvtPoint {
            process: reference_process,
            voltage: None,
            supply_source_names: Vec::new(),
            temperature_celsius,
            parameter_overrides: Vec::new(),
            source_overrides: Vec::new(),
            run_set_point_key: None,
            corner_contract: None,
        };
        let (deck, nominal_supply_voltage) = prepare_pvt_point_source(executable_netlist, &pvt)?;
        points.push(DeclaredPoint {
            identity: format!("{:016x}", temperature_celsius.to_bits()),
            label: format!("{temperature_celsius} \u{00b0}C"),
            is_nominal: point_is_nominal(
                &pvt,
                nominal_supply_voltage,
                reference_process,
                reference_temperature_celsius,
            ),
            nominal_supply_voltage,
            deck,
            contract: NarrowedContract::Temperature(TempRunConfig {
                temperatures_c: vec![temperature_celsius],
                base_mode: contract.base_mode.clone(),
            }),
            origin: DeclaredAxisPoint::Temperature(temperature_celsius),
            pvt,
        });
    }

    mint_declared_point_tasks(
        temperature_task,
        executable_netlist,
        &contract.base_mode,
        "temperature step",
        "rspice-temperature-run-point/v1",
        points,
    )
}

/// The declaration's contract, narrowed to the one point a task solves.
///
/// It is what makes the point's configuration digest its own, so it travels on
/// the task even where the deck already states the same condition.
enum NarrowedContract {
    Corner(CornerRunConfig),
    Temperature(TempRunConfig),
}

/// One point of a declared space, named and ready to become a task.
struct DeclaredPoint {
    /// The condition the point was declared at. It is what the attribution
    /// records and what the deck's temperature card is written from.
    pvt: PreparedPvtPoint,
    /// Whether the point sits on the run's own reference condition.
    is_nominal: bool,
    /// The supply the point's contract calls nominal. Only a declaration with
    /// a supply axis resolves one, and only the operating point carries it to
    /// the engine.
    nominal_supply_voltage: Option<f64>,
    /// The deck the point solves before its temperature card is spliced in.
    /// `None` takes the run-level executable source unchanged.
    deck: Option<String>,
    /// What distinguishes this point from the others of its declaration.
    identity: String,
    /// What distinguishes it to a reader, appended to the declaration's label.
    label: String,
    contract: NarrowedContract,
    /// What the family assembler needs and the point's own task cannot state.
    origin: DeclaredAxisPoint,
}

/// The task that solves one declared point.
///
/// The identity is derived from the declaration's, so a point can never be
/// mistaken for the declaration or for a point of another one, and preparing
/// the same declaration twice lands on the same identity. The digest is
/// recomputed last, because everything above it is part of the payload it
/// authenticates.
fn mint_declared_point_tasks(
    declaration: &PreparedTask,
    executable_netlist: &str,
    base_mode: &CornerBaseMode,
    what: &str,
    identity_namespace: &str,
    points: Vec<DeclaredPoint>,
) -> Result<Vec<PreparedTask>, PreparationError> {
    let count = points.len();
    let mut minted = Vec::with_capacity(count);
    for (index, point) in points.into_iter().enumerate() {
        let mut task = declaration.clone();
        task.adopt_derived_identity(declaration.derive(format!(
            "{identity_namespace}/{index}/{count}/{}",
            point.identity.as_str()
        )));
        task.pvt_point = Some(
            crate::state::AnalysisResultPvtPoint::new(
                point.pvt.process.short_name(),
                point.pvt.voltage,
                point.pvt.temperature_celsius,
                point
                    .pvt
                    .corner_contract
                    .as_ref()
                    .map(corner_contract_digest),
                point.is_nominal,
            )
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Point {}/{count} of the {what} cannot be attributed: {error}",
                        index + 1
                    ),
                )
            })?,
        );
        task.label = format!("{} \u{00b7} {}", declaration.label, point.label);
        task.declared_point = Some(DeclaredRunPoint::new(
            base_mode.clone(),
            count,
            index,
            point.origin,
        ));

        // The temperature is written into the deck rather than handed to the
        // engine beside it, so the point's own bytes state the condition it
        // was solved at. It also reaches deck expressions in `TEMPER`, which a
        // configuration field never did.
        let deck = point.deck.unwrap_or_else(|| executable_netlist.to_owned());
        task.executable_netlist_override = Some(splice_before_terminal_end_card(
            &deck,
            &format!(".OPTIONS TEMP={}", point.pvt.temperature_celsius),
        ));

        let (spec, config, analysis_line) = point_base_analysis_request(
            base_mode,
            &point.pvt,
            index,
            count,
            point.nominal_supply_voltage,
        );
        spec.validate().map_err(|error| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Point {}/{count} of the {what} has an invalid base analysis: {error}",
                    index + 1
                ),
            )
        })?;
        task.task.spec = spec;
        task.task.config = config;
        task.task.analysis_line = analysis_line;
        match point.contract {
            NarrowedContract::Corner(contract) => task.task.spec_options.corner = Some(contract),
            NarrowedContract::Temperature(contract) => task.task.spec_options.temp = Some(contract),
        }

        task.saved_output_contracts = declaration
            .saved_output_contracts
            .iter()
            .map(|saved| {
                saved
                    .rebind_analysis(task.instance_id, &task.task.spec)
                    .map_err(|error| {
                        PreparationError::new(
                            PreparationStage::AnalysisPlan,
                            format!(
                                "Failed to bind saved output to point {}/{count} of the {what}: {error}",
                                index + 1
                            ),
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        task.config_digest = task.payload_digest();
        minted.push(task);
    }

    Ok(minted)
}

/// The base analysis a declared point actually runs.
///
/// The operating point takes its temperature and supply from its own typed
/// run-point contract, because that contract is the only channel its engine
/// build reads. The other three have no such field: their temperature is
/// already in the point's deck, and their supply is applied from the narrowed
/// corner contract that travels with the request.
fn point_base_analysis_request(
    base_mode: &CornerBaseMode,
    point: &PreparedPvtPoint,
    index: usize,
    count: usize,
    nominal_supply_voltage: Option<f64>,
) -> (
    AnalysisSpec,
    Option<crate::simulation::AnalysisConfig>,
    String,
) {
    use crate::services::simulation_runner::CornerFrequencySweep;
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
                    supply_source_names: point.supply_source_names.clone(),
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
                // A point-family base sweep travels once through its range.
                //
                // The base is taken from the plan's DC sweep draft, which *can*
                // declare a retrace, but `CornerBaseMode::DcSweep` carries no
                // such field — it is the run's wire contract to the worker as
                // well as its in-process form — so the flag is dropped rather
                // than carried. The Corner and Temperature forms say so beside
                // the Base control instead of leaving "repeats the base
                // analysis" to imply otherwise.
                hysteresis: false,
            },
            None,
            format!(".dc {source_name} {start} {stop} {step}"),
        ),
        CornerBaseMode::DcSweepNested {
            source_name,
            start,
            stop,
            step,
            source2,
            start2,
            stop2,
            step2,
        } => (
            AnalysisSpec::DcSweep {
                source_name: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                source2: Some(source2.clone()),
                start2: Some(*start2),
                stop2: Some(*stop2),
                step2: Some(*step2),
                // As above: a corner base sweep travels once, and a nested one
                // could not retrace anyway.
                hysteresis: false,
            },
            None,
            format!(".dc {source_name} {start} {stop} {step} {source2} {start2} {stop2} {step2}"),
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
        CornerBaseMode::TransientWindow {
            stop_time,
            step_time,
            start_time,
            max_timestep,
            uic,
        } => (
            AnalysisSpec::Transient {
                stop_time: *stop_time,
                step_time: *step_time,
                start_time: *start_time,
                max_timestep: *max_timestep,
                uic: *uic,
            },
            None,
            format!(".tran {step_time} {stop_time} {start_time}"),
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
