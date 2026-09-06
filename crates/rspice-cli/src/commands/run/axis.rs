//! Execution of a `.STEP`/`.TEMP`/`.DATA` axis: one coordinate at a time, one
//! transaction for the set.
//!
//! Every coordinate is materialized from the canonical plan, preflighted for
//! its topology and result schema, and executed into its own artifact. The
//! whole set is staged in one transaction and committed only once every
//! coordinate has completed, so a failure or a cancellation leaves the
//! destination exactly as it was rather than a directory that looks like a
//! shorter sweep. The set's own manifests -- the coordinate schema union and
//! the run-set record -- are the last members of that transaction.

// This module was split out of `run.rs` and still works against the run
// command's own context, errors, and helpers, so it takes the parent's
// imports rather than restating them.
use super::*;

fn map_step_core_error(
    error: rspice_core::SimulationError,
    timeout_seconds: Option<f64>,
    analysis: impl Into<String>,
) -> CliError {
    if matches!(error, rspice_core::SimulationError::Aborted) {
        cancellation_cli_error(timeout_seconds)
    } else {
        CliError::CoreSimulationError {
            source: error,
            analysis: Some(analysis.into()),
        }
    }
}

pub(super) fn map_deck_plan_error(error: DeckPlanError, args: &RunArgs) -> CliError {
    match error {
        DeckPlanError::Aborted => cancellation_cli_error(args.timeout),
        DeckPlanError::ResourceLimit(source) => map_step_core_error(
            rspice_core::SimulationError::ResourceLimit(source),
            args.timeout,
            "Run-axis planning",
        ),
        // The plan names run axes, analysis instances, and the carrier each
        // periodic card depends on, so this covers more than a malformed
        // `.STEP`: the message must not tell the author to fix an axis their
        // deck does not have.
        error => CliError::InvalidArgument {
            message: format!("canonical deck planning failed: {error}"),
            suggestion: Some(
                "fix the run axis or the analysis card it names before any coordinate is simulated"
                    .to_string(),
            ),
        },
    }
}

pub(super) fn map_materialized_run_error(
    error: MaterializedRunError,
    args: &RunArgs,
    analysis: impl Into<String>,
) -> CliError {
    match error {
        MaterializedRunError::Aborted => cancellation_cli_error(args.timeout),
        MaterializedRunError::DeckPlan(error) => map_deck_plan_error(error, args),
        MaterializedRunError::Simulation(error) => {
            map_step_core_error(error, args.timeout, analysis)
        }
        error => CliError::InternalError {
            message: format!("canonical deck materialization failed: {error}"),
        },
    }
}

pub(super) fn canonical_coordinate_description(coordinate: &RunCoordinate) -> String {
    coordinate
        .assignments()
        .iter()
        .map(|assignment| match assignment.value() {
            RunAxisValue::Numeric(value) => {
                format!("{} = {value}", canonical_assignment_target(assignment))
            }
            RunAxisValue::DataRow(bindings) => format!(
                "DATA {} row {} ({})",
                assignment.name(),
                assignment.value_index() + 1,
                bindings
                    .iter()
                    .map(|binding| format!("{}={}", binding.name(), binding.value()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            RunAxisValue::AlterVariant { label, .. } => format!("ALTER {label}"),
            unsupported => format!(
                "{} = {unsupported:?}",
                canonical_assignment_target(assignment)
            ),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn canonical_assignment_target(assignment: &AxisAssignment) -> String {
    match assignment.step_target() {
        Some(StepAxisTarget::Parameter { name }) => format!("PARAM {name}"),
        Some(StepAxisTarget::Device {
            name,
            parameter: Some(parameter),
        }) => format!("DEVICE {name}.{parameter}"),
        Some(StepAxisTarget::Device {
            name,
            parameter: None,
        }) => format!("DEVICE {name}"),
        Some(StepAxisTarget::Model { name, parameter }) => {
            format!("MODEL {name}.{parameter}")
        }
        Some(StepAxisTarget::Temperature) | None if assignment.kind() == AxisKind::Temperature => {
            "TEMP".to_string()
        }
        None | Some(_) => assignment.name().to_string(),
    }
}

/// The contract one axis coordinate declared before any solver work began.
///
/// The preflight materializes every coordinate once, so this is where its
/// child-analysis signature and its structural fingerprint are captured. The
/// execution pass re-materializes to run, and checks what it gets against this
/// rather than re-deriving it, so a coordinate whose topology moved between the
/// two passes fails instead of publishing under a contract it no longer meets.
pub(super) struct StepCoordinateContract {
    signature: Vec<&'static str>,
    pub(super) topology: rspice_core::execution::TopologyFingerprint,
}

pub(super) fn preflight_step_coordinates(
    engine: &Engine,
    materializer: &DeckPlanMaterializer<'_>,
    base_signature: &[&'static str],
    aggregate_report_values: Option<usize>,
    args: &RunArgs,
) -> Result<Vec<StepCoordinateContract>, CliError> {
    let mut contracts = Vec::with_capacity(materializer.len());
    let mut retained_report_values = aggregate_report_values.unwrap_or(0);
    let retained_limit = engine.config().resource_limits.max_result_values;
    if retained_report_values > retained_limit {
        return Err(map_step_core_error(
            rspice_core::SimulationError::ResourceLimit(rspice_core::ResourceLimitError {
                resource: rspice_core::ResourceKind::ResultValues,
                requested: retained_report_values,
                limit: retained_limit,
            }),
            args.timeout,
            "Step reporting preflight",
        ));
    }
    for (run_index, canonical_coordinate) in materializer.coordinates().iter().enumerate() {
        let coordinate = canonical_coordinate_description(canonical_coordinate);
        let materialized = materializer
            .materialize_run_with_abort(run_index, &crate::abort::ProcessAbort)
            .map_err(|error| {
                map_materialized_run_error(
                    error,
                    args,
                    format!(
                        ".STEP {} ({coordinate}) preflight",
                        canonical_coordinate.stable_tag()
                    ),
                )
            })?;
        let signature = step_analysis_signature(materialized.netlist());
        if signature != base_signature {
            return Err(CliError::InvalidArgument {
                message: format!(
                    ".STEP coordinate {} ({coordinate}) conditionally changes the child-analysis signature from {:?} to {:?}",
                    run_index + 1,
                    base_signature,
                    signature
                ),
                suggestion: Some(
                    "keep the authored physical-analysis and post-processing card set unconditional across every coordinate"
                        .to_string(),
                ),
            });
        }
        contracts.push(StepCoordinateContract {
            signature,
            topology: materialized.topology_fingerprint(),
        });

        // Each report retains one duration plus up to value/goal/tolerance
        // for every measurement. Bound the numeric reporting payload before
        // any solver or output file starts.
        if aggregate_report_values.is_none() {
            retained_report_values = retained_report_values
                .saturating_add(1)
                .saturating_add(materialized.netlist().measurements.len().saturating_mul(3));
            if retained_report_values > retained_limit {
                return Err(map_step_core_error(
                    rspice_core::SimulationError::ResourceLimit(rspice_core::ResourceLimitError {
                        resource: rspice_core::ResourceKind::ResultValues,
                        requested: retained_report_values,
                        limit: retained_limit,
                    }),
                    args.timeout,
                    "Step reporting preflight",
                ));
            }
        }
    }
    Ok(contracts)
}

fn implicit_axis_assignment(coordinate: &RunCoordinate) -> Result<&AxisAssignment, CliError> {
    let [assignment] = coordinate.assignments() else {
        return Err(CliError::InternalError {
            message: format!(
                "implicit operating-point aggregation requires exactly one run axis, but coordinate {} has {}",
                coordinate.stable_tag(),
                coordinate.assignments().len()
            ),
        });
    };
    Ok(assignment)
}

fn implicit_axis_value(assignment: &AxisAssignment) -> Result<f64, CliError> {
    match assignment.value() {
        RunAxisValue::Numeric(value) => Ok(*value),
        RunAxisValue::DataRow(_) => Ok(assignment.value_index() as f64),
        RunAxisValue::AlterVariant { .. } => Err(CliError::InvalidArgument {
            message: "textual .ALTER cannot drive an implicit operating-point table".to_string(),
            suggestion: Some("expand each .ALTER variant before execution".to_string()),
        }),
        value => Err(CliError::InternalError {
            message: format!("unsupported implicit run-axis value {value:?}"),
        }),
    }
}

fn implicit_axis_scale_name(assignment: &AxisAssignment) -> String {
    match assignment.value() {
        RunAxisValue::DataRow(_) => format!("DATA({})", assignment.name()),
        _ => match assignment.step_target() {
            Some(StepAxisTarget::Parameter { name }) => name.clone(),
            Some(StepAxisTarget::Device {
                name,
                parameter: Some(parameter),
            })
            | Some(StepAxisTarget::Model { name, parameter }) => {
                format!("{name}:{parameter}")
            }
            Some(StepAxisTarget::Device {
                name,
                parameter: None,
            }) => name.clone(),
            Some(StepAxisTarget::Temperature) | None
                if assignment.kind() == AxisKind::Temperature =>
            {
                "TEMP".to_string()
            }
            None | Some(_) => assignment.name().to_string(),
        },
    }
}

fn run_implicit_step_op_table(
    netlist: &Netlist,
    args: &RunArgs,
    config: &Config,
    verbose: bool,
    quiet: bool,
    engine: &Engine,
    plan: &DeckPlan,
    materializer: &DeckPlanMaterializer<'_>,
    run_label: Option<&str>,
) -> Result<DeckOutcome, CliError> {
    let first_coordinate =
        materializer
            .coordinates()
            .first()
            .ok_or_else(|| CliError::InternalError {
                message: "implicit deck materializer has no coordinate".to_string(),
            })?;
    let one_dimensional = first_coordinate.assignments().len() == 1;
    let (target, scale_name) = if one_dimensional {
        let first_assignment = implicit_axis_assignment(first_coordinate)?;
        (
            canonical_assignment_target(first_assignment),
            Some(implicit_axis_scale_name(first_assignment)),
        )
    } else {
        ("Cartesian run axes".to_string(), None)
    };
    // This path publishes under the implicit-OP identity each coordinate
    // carries, resolved per coordinate below. The shared context owns no
    // authored analysis namespace of its own.
    let ctx = RunContext::new(
        engine,
        netlist,
        args,
        config,
        verbose,
        quiet,
        run_label,
        RunIdentity {
            coordinate: None,
            topology: None,
            analyses: PlannedAnalysisIdentities::default(),
        },
    )?;
    let start_time = Instant::now();
    let mut retained_values = 0usize;
    let mut preflight = Vec::with_capacity(materializer.len());

    for run_index in 0..materializer.len() {
        if crate::abort::reason().is_some() {
            break;
        }
        let materialized =
            match materializer.materialize_run_with_abort(run_index, &crate::abort::ProcessAbort) {
                Ok(materialized) => materialized,
                Err(MaterializedRunError::Aborted) if crate::abort::reason().is_some() => {
                    break;
                }
                Err(error) => {
                    return Err(map_materialized_run_error(error, args, "Step"));
                }
            };
        let canonical_coordinate = materialized.coordinate();
        let value = one_dimensional
            .then(|| implicit_axis_assignment(canonical_coordinate))
            .transpose()?
            .map(implicit_axis_value)
            .transpose()?;
        let [implicit_analysis] = materialized.analyses() else {
            return Err(CliError::InternalError {
                message: format!(
                    "implicit coordinate {} materialized {} analysis identities",
                    canonical_coordinate.stable_id(),
                    materialized.analyses().len()
                ),
            });
        };
        if implicit_analysis.command().is_some() {
            return Err(CliError::InternalError {
                message: format!(
                    "implicit coordinate {} unexpectedly owns an authored analysis command",
                    canonical_coordinate.stable_id()
                ),
            });
        }
        let coordinate_engine =
            Engine::try_new(build_sim_config(args, config, materialized.netlist()))?;
        let topology = materialized.topology_fingerprint();
        // The device operating-point report is taken at every coordinate, not
        // only for a scalar deck: it is what carries the complete typed
        // device-observable inventory into the coordinate's typed document, and
        // a sweep that dropped it would publish fewer observables than the same
        // deck run without an axis.
        let (result, device_report) = match coordinate_engine
            .run_dc_op_with_report_and_abort(materialized.netlist(), &crate::abort::ProcessAbort)
        {
            Ok(solved) => solved,
            Err(rspice_core::SimulationError::Aborted) if crate::abort::reason().is_some() => {
                break;
            }
            Err(error) => {
                let coordinate_context = value.map_or_else(
                    || canonical_coordinate_description(canonical_coordinate),
                    |value| format!("{target} = {value}"),
                );
                return Err(CliError::simulation_error_in(
                    format!(
                        ".STEP {} ({coordinate_context}): {error}",
                        canonical_coordinate.stable_tag()
                    ),
                    "Step",
                ));
            }
        };
        let signals = crate::commands::run_signals::dc_operating_point_export_signals(
            materialized.netlist(),
            &result,
            &crate::abort::ProcessAbort,
        )
        .map_err(|source| CliError::CoreSimulationError {
            source,
            analysis: Some(format!(
                ".STEP {} output-schema preflight",
                canonical_coordinate.stable_tag()
            )),
        })?;
        let schema =
            crate::commands::run_signals::scalar_signal_schema(&signals).map_err(|error| {
                CliError::CoreSimulationError {
                    source: rspice_core::SimulationError::Circuit(format!(
                        ".STEP {} has an invalid coordinate-local signal schema: {error}",
                        canonical_coordinate.stable_tag()
                    )),
                    analysis: Some("Step output-schema preflight".to_string()),
                }
            })?;
        shared::ensure_finite_series(
            args.allow_nonfinite,
            "Step",
            signals
                .iter()
                .map(|signal| (signal.display_name.as_str(), signal.values.as_slice())),
        )?;
        retained_values = retained_values
            .saturating_add(result.retained_value_count())
            .saturating_add(1);
        let retained_limit = engine.config().resource_limits.max_result_values;
        if retained_values > retained_limit {
            return Err(map_step_core_error(
                rspice_core::SimulationError::ResourceLimit(rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::ResultValues,
                    requested: retained_values,
                    limit: retained_limit,
                }),
                args.timeout,
                "Step result aggregation",
            ));
        }
        preflight.push(ImplicitStepCoordinate {
            value,
            coordinate_id: canonical_coordinate.stable_id(),
            coordinate_tag: canonical_coordinate.stable_tag(),
            canonical: canonical_coordinate.clone(),
            analysis: implicit_analysis.id(),
            analysis_id: implicit_analysis.output_namespace().analysis_component(),
            topology,
            result,
            device_report,
            signals,
            schema,
            validity: Vec::new(),
        });
    }
    if preflight.len() != materializer.len() {
        return Err(cancellation_cli_error(args.timeout));
    }

    let schema_union =
        rspice_core::execution::SignalSchema::union(preflight.iter().map(|run| {
            rspice_core::execution::CoordinateSchema::new(run.coordinate_id, &run.schema)
        }))
        .map_err(|error| CliError::CoreSimulationError {
            source: rspice_core::SimulationError::Circuit(format!(
                ".STEP coordinate schemas cannot form a typed union: {error}"
            )),
            analysis: Some("Step output-schema preflight".to_string()),
        })?;
    for run in &mut preflight {
        let values = run
            .signals
            .iter()
            .map(|signal| {
                signal
                    .values
                    .first()
                    .copied()
                    .ok_or_else(|| CliError::CoreSimulationError {
                        source: rspice_core::SimulationError::Circuit(format!(
                            ".STEP {} signal '{}' has no operating-point value",
                            run.coordinate_id, signal.display_name
                        )),
                        analysis: Some("Step output-schema preflight".to_string()),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let aligned = schema_union
            .align_values(run.coordinate_id, &values)
            .map_err(|error| CliError::CoreSimulationError {
                source: rspice_core::SimulationError::Circuit(format!(
                    ".STEP {} cannot align its coordinate-local values to the union schema: {error}",
                    run.coordinate_id
                )),
                analysis: Some("Step output-schema preflight".to_string()),
            })?;
        run.validity = aligned.iter().map(Option::is_some).collect();
    }

    let stable_topology_and_schema = preflight.first().is_none_or(|first| {
        preflight
            .iter()
            .all(|run| run.topology == first.topology && run.schema == first.schema)
    });
    let mut outputs = Vec::new();
    if one_dimensional && stable_topology_and_schema {
        let scale_name = scale_name
            .as_deref()
            .ok_or_else(|| CliError::InternalError {
                message: "one-dimensional implicit plan has no scale name".to_string(),
            })?;
        let results = preflight
            .iter()
            .map(|run| {
                run.value
                    .map(|value| (value, run.result.clone()))
                    .ok_or_else(|| CliError::InternalError {
                        message: format!(
                            "one-dimensional implicit coordinate {} has no scalar axis value",
                            run.coordinate_id
                        ),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        advanced::export_step_sweep(&ctx, scale_name, &results)?;
        outputs.extend(ctx.outputs.borrow().iter().cloned());
    } else if let Some(base_output) = ctx.output.clone() {
        // Flat artifacts stay coordinate-local when topology changes.  Every
        // coordinate was solved and schema-checked above, before this first
        // write, so coordinate order can neither select columns nor leave a
        // partial batch due to a late schema mismatch. The set is published
        // as one transaction, so the schema manifest can never name a
        // coordinate artifact that a cancellation left unwritten.
        let transaction = publish::begin()?;
        let mut coordinate_publications = Vec::with_capacity(preflight.len());
        let mut set_coordinates = Vec::with_capacity(preflight.len());
        for run in &preflight {
            let coordinate_path =
                tag_output_path(&base_output, &sanitize_run_tag(&run.coordinate_tag));
            let path = tag_output_path(&coordinate_path, &run.analysis_id);
            if ctx.format == OutputFormat::Json {
                // A coordinate-local implicit operating point is a result like
                // any other: it publishes the shared typed document, naming the
                // coordinate and topology that produced it.
                let builder = rspice_core::execution::AnalysisResultDocument::from_operating_point(
                    run.analysis,
                    &run.result,
                    Some(&run.device_report),
                )
                .map_err(|error| document::document_error(&ctx, run.analysis, error))?;
                let built = document::finish_at_coordinate(
                    &ctx,
                    run.analysis,
                    &run.canonical,
                    run.topology,
                    builder,
                )?;
                document::write_document(&ctx, &path, &built)?;
            } else {
                basic::write_dc_op_output(
                    &path,
                    &run.signals,
                    ctx.format,
                    Some(&crate::hdf5::Hdf5ResultIdentity {
                        analysis_id: run.analysis_id.clone(),
                        coordinate_id: Some(run.coordinate_id.to_string()),
                        coordinate_tag: Some(run.coordinate_tag.clone()),
                        coordinate_assignment: Some(canonical_coordinate_description(
                            &run.canonical,
                        )),
                        topology_fingerprint: Some(run.topology.to_string()),
                    }),
                )?;
            }
            coordinate_publications.push(CoordinatePublication {
                coordinate: run.canonical.clone(),
                topology: run.topology,
                results: vec![PublishedResult {
                    analysis_id: run.analysis_id.clone(),
                    schema: run.schema.clone(),
                    artifact: path.clone(),
                }],
            });
            set_coordinates.push(AxisSetCoordinate {
                identity: ArtifactCoordinate::from_run_coordinate(&run.canonical),
                artifacts: vec![path.clone()],
            });
            outputs.push(path);
        }
        let manifest_path = conditional_step_schema_path(&base_output);
        write_step_schema_manifest(&manifest_path, &coordinate_publications)?;
        // This is a coordinate set like any other, so it commits the same
        // manifest declaring the set complete. Without it a reader could not
        // tell a finished implicit-operating-point set from one a cancellation
        // stopped part-way, and would have to re-derive the artifact names it
        // expected instead of reading the ones the run published.
        let set_manifest_path = axis_set_manifest_path(args, config, run_label)?;
        if let Some(set_manifest_path) = &set_manifest_path {
            write_axis_set_manifest(set_manifest_path, args, plan, &set_coordinates)?;
        }
        if crate::abort::reason().is_some() {
            drop(transaction);
            return Err(cancellation_cli_error(args.timeout));
        }
        transaction.commit()?;
        outputs.push(manifest_path);
        outputs.extend(set_manifest_path);
    }
    ctx.record_unevaluated_measurements();
    let measurements = ctx.measurements.borrow().clone();
    let base_name = args
        .input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| *stem != "-")
        .unwrap_or("stdin")
        .to_string();
    let report_name = match run_label {
        Some(label) => format!("{base_name} [{label}]"),
        None => base_name,
    };
    Ok(DeckOutcome {
        reports: vec![SimulationReport {
            name: report_name,
            netlist: args.input.display().to_string(),
            passed: measurements.iter().all(|measurement| measurement.passed),
            duration_secs: start_time.elapsed().as_secs_f64(),
            error: None,
            error_details: None,
            measurements,
        }],
        outputs,
    })
}

struct ImplicitStepCoordinate {
    pub(super) value: Option<f64>,
    pub(super) coordinate_id: rspice_core::execution::RunCoordinateId,
    pub(super) coordinate_tag: String,
    /// The canonical coordinate itself, so a typed coordinate document can
    /// name every axis assignment rather than only its identity string.
    canonical: RunCoordinate,
    pub(super) analysis: rspice_core::execution::AnalysisInstanceId,
    pub(super) analysis_id: String,
    pub(super) topology: rspice_core::execution::TopologyFingerprint,
    pub(super) result: rspice_core::solver::SimulationResult,
    /// The coordinate's complete device operating-point inventory, published in
    /// its typed document beside the node and branch series.
    device_report: rspice_core::circuit::DeviceOpReport,
    pub(super) signals: Vec<crate::commands::run_signals::ScalarSignal>,
    pub(super) schema: rspice_core::execution::SignalSchema,
    pub(super) validity: Vec<bool>,
}

/// Version of the coordinate schema manifest.
///
/// Version 2 groups the union by analysis instance. Version 1 described one
/// implicit operating point, which could not name the several analyses a
/// stepped physical deck publishes at each coordinate.
const STEP_SCHEMA_MANIFEST_VERSION: u32 = 2;

/// The union schema and per-coordinate validity of one analysis instance
/// across an axis deck's coordinates.
struct AnalysisSchemaUnion {
    pub(super) analysis_id: String,
    union: rspice_core::execution::SchemaUnion,
    coordinates: Vec<CoordinateValidity>,
}

/// What one coordinate published for one analysis, and which of the union's
/// columns it carried.
struct CoordinateValidity {
    pub(super) coordinate_id: rspice_core::execution::RunCoordinateId,
    pub(super) assignment: String,
    pub(super) topology: rspice_core::execution::TopologyFingerprint,
    pub(super) artifact: PathBuf,
    pub(super) validity: Vec<bool>,
}

/// Union each analysis instance's coordinate-local schemas and record, per
/// coordinate, which union columns that coordinate actually carried.
///
/// An analysis that only some coordinates published — a conditional that adds
/// or drops a card — is still named, with its own coordinate list. Nothing is
/// inferred from a coordinate that did not publish it.
fn analysis_schema_unions(
    published: &[CoordinatePublication],
) -> Result<Vec<AnalysisSchemaUnion>, CliError> {
    let mut order: Vec<String> = Vec::new();
    let mut grouped: std::collections::HashMap<
        String,
        Vec<(&CoordinatePublication, &PublishedResult)>,
    > = std::collections::HashMap::new();
    for coordinate in published {
        for result in &coordinate.results {
            let entry = grouped.entry(result.analysis_id.clone()).or_default();
            if entry.is_empty() {
                order.push(result.analysis_id.clone());
            }
            entry.push((coordinate, result));
        }
    }

    let mut unions = Vec::with_capacity(order.len());
    for analysis_id in order {
        let entries = grouped.remove(&analysis_id).unwrap_or_default();
        let union = rspice_core::execution::SignalSchema::union(entries.iter().map(
            |(coordinate, result)| {
                rspice_core::execution::CoordinateSchema::new(
                    coordinate.coordinate.stable_id(),
                    &result.schema,
                )
            },
        ))
        .map_err(|error| CliError::CoreSimulationError {
            source: rspice_core::SimulationError::Circuit(format!(
                "coordinate schemas of {analysis_id} cannot form a typed union: {error}"
            )),
            analysis: Some("Step output-schema union".to_string()),
        })?;
        let mut coordinates = Vec::with_capacity(entries.len());
        for (coordinate, result) in entries {
            let indices = union
                .source_indices()
                .get(&coordinate.coordinate.stable_id())
                .ok_or_else(|| CliError::InternalError {
                    message: format!(
                        "coordinate {} vanished from the {analysis_id} schema union",
                        coordinate.coordinate.stable_id()
                    ),
                })?;
            coordinates.push(CoordinateValidity {
                coordinate_id: coordinate.coordinate.stable_id(),
                assignment: canonical_coordinate_description(&coordinate.coordinate),
                topology: coordinate.topology,
                artifact: result.artifact.clone(),
                validity: indices.iter().map(Option::is_some).collect(),
            });
        }
        unions.push(AnalysisSchemaUnion {
            analysis_id,
            union,
            coordinates,
        });
    }
    Ok(unions)
}

/// Publish the manifest that says what each coordinate of an axis deck
/// carried.
///
/// Flat formats have no representation for an absent column, so this is where
/// a consumer learns which coordinate published which signal: the union names
/// every column any coordinate had, and each coordinate's validity bitmap says
/// which of them its own artifact contains.
fn write_step_schema_manifest(
    path: &std::path::Path,
    published: &[CoordinatePublication],
) -> Result<(), CliError> {
    let unions = analysis_schema_unions(published)?;
    let analyses = unions
        .iter()
        .map(|entry| {
            let coordinates = entry
                .coordinates
                .iter()
                .map(|coordinate| {
                    let artifact = coordinate
                        .artifact
                        .file_name()
                        .and_then(std::ffi::OsStr::to_str)
                        .ok_or_else(|| CliError::InternalError {
                            message: format!(
                                "coordinate artifact '{}' has no portable UTF-8 filename",
                                coordinate.artifact.display()
                            ),
                        })?;
                    Ok(serde_json::json!({
                        "coordinate_id": coordinate.coordinate_id.to_string(),
                        "assignment": coordinate.assignment,
                        "topology_fingerprint": coordinate.topology.to_string(),
                        "validity": coordinate.validity,
                        "artifact": artifact,
                    }))
                })
                .collect::<Result<Vec<_>, CliError>>()?;
            Ok(serde_json::json!({
                "analysis_id": entry.analysis_id,
                "union_schema": entry
                    .union
                    .schema()
                    .descriptors()
                    .iter()
                    .map(signal_descriptor_json)
                    .collect::<Vec<_>>(),
                "coordinates": coordinates,
            }))
        })
        .collect::<Result<Vec<_>, CliError>>()?;
    let document = serde_json::json!({
        "schema_version": STEP_SCHEMA_MANIFEST_VERSION,
        "aggregation": "coordinate_local",
        "missingness": "union_validity_bitmap",
        "analyses": analyses,
    });
    let text = serde_json::to_string_pretty(&document)
        .map_err(|error| CliError::output_json_error(path, error))?
        + "\n";
    publish::artifact(path, |writer: &mut dyn std::io::Write| {
        writer
            .write_all(text.as_bytes())
            .map_err(|error| CliError::output_error(path, error))
    })
    .map_err(|error| map_atomic_output_error(path, error))
}

fn signal_descriptor_json(
    descriptor: &rspice_core::execution::SignalDescriptor,
) -> serde_json::Value {
    serde_json::json!({
        "canonical_name": descriptor.canonical_name(),
        "display_name": descriptor.display_name(),
        "kind": execution_signal_kind_name(descriptor.kind()),
        "unit": execution_signal_unit_name(descriptor.unit()),
        "value_type": execution_signal_value_type_name(descriptor.value_type()),
        "shape": execution_signal_shape_name(descriptor.shape()),
        "owner": execution_signal_owner_json(descriptor.owner()),
    })
}

fn execution_signal_kind_name(kind: rspice_core::execution::SignalKind) -> &'static str {
    use rspice_core::execution::SignalKind;
    match kind {
        SignalKind::Voltage => "voltage",
        SignalKind::Current => "current",
        SignalKind::DeviceObservable => "device_observable",
        SignalKind::Scalar => "scalar",
        SignalKind::Digital => "digital",
        _ => "unknown",
    }
}

fn execution_signal_unit_name(unit: &rspice_core::execution::SignalUnit) -> String {
    use rspice_core::execution::SignalUnit;
    match unit {
        SignalUnit::Volt => "volt".to_string(),
        SignalUnit::Ampere => "ampere".to_string(),
        SignalUnit::Ohm => "ohm".to_string(),
        SignalUnit::Siemens => "siemens".to_string(),
        SignalUnit::Watt => "watt".to_string(),
        SignalUnit::Hertz => "hertz".to_string(),
        SignalUnit::Second => "second".to_string(),
        SignalUnit::Degree => "degree".to_string(),
        SignalUnit::Radian => "radian".to_string(),
        SignalUnit::Dimensionless => "dimensionless".to_string(),
        SignalUnit::Logic => "logic".to_string(),
        SignalUnit::Custom(name) => format!("custom:{name}"),
        _ => "unknown".to_string(),
    }
}

fn execution_signal_value_type_name(
    value_type: rspice_core::execution::SignalValueType,
) -> &'static str {
    use rspice_core::execution::SignalValueType;
    match value_type {
        SignalValueType::Real => "real",
        SignalValueType::Complex => "complex",
        SignalValueType::Logic => "logic",
        _ => "unknown",
    }
}

fn execution_signal_shape_name(shape: rspice_core::execution::SignalShape) -> &'static str {
    use rspice_core::execution::SignalShape;
    match shape {
        SignalShape::Scalar => "scalar",
        SignalShape::Vector => "vector",
        SignalShape::Matrix => "matrix",
        _ => "unknown",
    }
}

fn execution_signal_owner_json(owner: &rspice_core::execution::SignalOwner) -> serde_json::Value {
    use rspice_core::execution::SignalOwner;
    match owner {
        SignalOwner::Node(name) => serde_json::json!({"kind": "node", "name": name}),
        SignalOwner::Branch(name) => serde_json::json!({"kind": "branch", "name": name}),
        SignalOwner::Device(name) => serde_json::json!({"kind": "device", "name": name}),
        SignalOwner::Analysis => serde_json::json!({"kind": "analysis"}),
        _ => serde_json::json!({"kind": "unknown"}),
    }
}

pub(super) fn run_deck(
    netlist: &Netlist,
    args: &RunArgs,
    config: &Config,
    verbose: bool,
    quiet: bool,
    run_label: Option<&str>,
) -> Result<DeckOutcome, CliError> {
    validate_pss_flag_conflict(netlist, args)?;
    validate_step_frontend_compatibility(netlist, args)?;

    let resource_limits = config.resources.limits();
    let canonical_plan =
        DeckPlan::from_netlist_with_abort(netlist, &resource_limits, &crate::abort::ProcessAbort)
            .map_err(|error| map_deck_plan_error(error, args))?;
    if canonical_plan.axes().is_empty() {
        // An axis-free deck still takes its artifact namespaces from the
        // canonical plan. Reading the authored identities straight off the
        // plan costs no materialization, so a scalar run does not pay for a
        // second elaboration to learn what it already planned.
        let outcome = run_concrete_deck(
            netlist,
            args,
            config,
            verbose,
            quiet,
            run_label,
            RunIdentity {
                coordinate: None,
                topology: None,
                analyses: PlannedAnalysisIdentities::from_plan(&canonical_plan, netlist),
            },
        )?;
        return Ok(DeckOutcome {
            reports: vec![outcome.report],
            outputs: outcome.outputs,
        });
    }

    let base_signature = step_analysis_signature(netlist);
    let sim_config = build_sim_config(args, config, netlist);
    let engine = Engine::try_new(sim_config)?;
    let materializer = engine
        .prepare_deck_plan_materializer_with_abort(
            netlist,
            &canonical_plan,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_materialized_run_error(error, args, "Step planning"))?;
    let aggregate_report_values = base_signature
        .is_empty()
        .then(|| 1usize.saturating_add(netlist.measurements.len().saturating_mul(3)));
    let coordinate_contracts = preflight_step_coordinates(
        &engine,
        &materializer,
        &base_signature,
        aggregate_report_values,
        args,
    )?;

    if base_signature.is_empty() {
        return run_implicit_step_op_table(
            netlist,
            args,
            config,
            verbose,
            quiet,
            &engine,
            &canonical_plan,
            &materializer,
            run_label,
        );
    }

    if !quiet {
        println!(
            "Canonical Cartesian run plan: {} dimension(s), {} run(s); first canonical dimension varies fastest",
            canonical_plan.axes().len(),
            materializer.len()
        );
    }
    // Every coordinate of one axis deck is one result. The transaction holds
    // each coordinate's complete artifact in a staging file beside its
    // destination, so a cancellation or a failure at coordinate k publishes
    // nothing at all instead of a directory that looks like a shorter sweep.
    let transaction = publish::begin()?;
    let mut reports = Vec::with_capacity(materializer.len());
    let mut outputs = Vec::new();
    let mut coordinates = Vec::new();
    let mut published = Vec::new();
    for (run_index, expected) in coordinate_contracts.iter().enumerate() {
        if crate::abort::reason().is_some() {
            break;
        }
        let materialized =
            match materializer.materialize_run_with_abort(run_index, &crate::abort::ProcessAbort) {
                Ok(materialized) => materialized,
                Err(MaterializedRunError::Aborted) if crate::abort::reason().is_some() => {
                    break;
                }
                Err(error) => {
                    return Err(map_materialized_run_error(
                        error,
                        args,
                        format!("Step coordinate {}", run_index + 1),
                    ));
                }
            };
        let canonical_coordinate = materialized.coordinate();
        let materialized_signature = step_analysis_signature(materialized.netlist());
        if materialized_signature != expected.signature {
            return Err(CliError::InternalError {
                message: format!(
                    ".STEP coordinate {} changed its preflight physical-analysis signature from {:?} to {:?}",
                    run_index + 1,
                    expected.signature,
                    materialized_signature
                ),
            });
        }
        // The preflight captured this coordinate's structural fingerprint
        // before any solver work. Executing it a second time must reproduce the
        // same circuit, or the artifact would be published under a topology
        // contract it no longer meets.
        let topology = materialized.topology_fingerprint();
        if topology != expected.topology {
            return Err(CliError::InternalError {
                message: format!(
                    ".STEP coordinate {} changed its preflight topology fingerprint from {} to {topology}",
                    run_index + 1,
                    expected.topology
                ),
            });
        }
        let coordinate_label = canonical_coordinate.stable_tag();
        let label = compose_run_label(run_label, Some(&coordinate_label)).ok_or_else(|| {
            CliError::InternalError {
                message: "STEP coordinate unexpectedly has no output namespace".to_string(),
            }
        })?;
        if verbose && !quiet {
            println!(
                "\n=== {label} ({}): {} ===",
                canonical_coordinate.stable_tag(),
                canonical_coordinate_description(canonical_coordinate)
            );
        }
        let outcome = match run_concrete_deck(
            materialized.netlist(),
            args,
            config,
            verbose,
            quiet,
            Some(&label),
            RunIdentity {
                coordinate: Some(canonical_coordinate),
                topology: Some(topology),
                analyses: PlannedAnalysisIdentities::from_materialized(
                    &canonical_plan,
                    canonical_coordinate,
                    materialized.analyses(),
                )?,
            },
        ) {
            Ok(outcome) => outcome,
            Err(_) if crate::abort::reason().is_some() => break,
            Err(error) => return Err(error),
        };
        reports.push(outcome.report);
        published.push(CoordinatePublication {
            coordinate: canonical_coordinate.clone(),
            topology,
            results: outcome.published,
        });
        coordinates.push(AxisSetCoordinate {
            identity: ArtifactCoordinate::from_run_coordinate(canonical_coordinate),
            artifacts: outcome.outputs.clone(),
        });
        outputs.extend(outcome.outputs);
        if crate::abort::reason().is_some() {
            break;
        }
    }
    if crate::abort::reason().is_some() {
        // Dropping the transaction removes every staged coordinate, so the
        // destination directory keeps exactly the artifacts it had before.
        drop(transaction);
        if !quiet {
            println!(
                "Cancelled after {} of {} coordinates: the incomplete set was discarded and no coordinate artifact was published",
                reports.len(),
                materializer.len()
            );
        }
        return Ok(DeckOutcome {
            reports,
            outputs: Vec::new(),
        });
    }
    if reports.len() != materializer.len() {
        return Err(CliError::InternalError {
            message: format!(
                ".STEP completed {} of {} planned coordinates without a cancellation or error",
                reports.len(),
                materializer.len()
            ),
        });
    }
    // A coordinate that failed published nothing, so the set is not the
    // complete one the manifests would declare. Committing them anyway would
    // leave a reader a `run_set` naming a set it can never load: dropping the
    // transaction leaves the destination exactly as it was.
    if reports.iter().any(|report| report.error.is_some()) {
        drop(transaction);
        if !quiet {
            println!(
                "{} of {} coordinates failed: the incomplete set was discarded and no coordinate artifact was published",
                reports
                    .iter()
                    .filter(|report| report.error.is_some())
                    .count(),
                materializer.len()
            );
        }
        return Ok(DeckOutcome {
            reports,
            outputs: Vec::new(),
        });
    }

    // Every coordinate published under its own schema, so the set declares the
    // union of those schemas and, per coordinate, which of its columns the
    // coordinate actually carried. A signal a conditional removed is absent
    // from that coordinate's artifact and invalid in the bitmap; it is never
    // inferred from another coordinate or written as zero.
    if let Some(base_output) = resolve_output_path(args.output.clone(), config)? {
        let base_output = match run_label {
            Some(label) => tag_output_path(&base_output, &sanitize_run_tag(label)),
            None => base_output,
        };
        let manifest_path = conditional_step_schema_path(&base_output);
        write_step_schema_manifest(&manifest_path, &published)?;
        outputs.push(manifest_path);
    }

    if let Some(manifest_path) = axis_set_manifest_path(args, config, run_label)? {
        write_axis_set_manifest(&manifest_path, args, &canonical_plan, &coordinates)?;
        outputs.push(manifest_path);
    }
    transaction.commit()?;
    Ok(DeckOutcome { reports, outputs })
}

/// Everything one axis coordinate published, with the identity it published
/// under.
pub(super) struct CoordinatePublication {
    pub(super) coordinate: RunCoordinate,
    pub(super) topology: rspice_core::execution::TopologyFingerprint,
    pub(super) results: Vec<PublishedResult>,
}

/// One coordinate's identity and the artifacts it staged, for the set
/// manifest published with the coordinate set.
pub(super) struct AxisSetCoordinate {
    pub(super) identity: ArtifactCoordinate,
    pub(super) artifacts: Vec<PathBuf>,
}

/// Describe the complete coordinate set as the last member of its own
/// transaction.
///
/// The manifest is committed after every coordinate artifact, so a reader
/// that finds it knows every artifact it names is present and complete.
fn write_axis_set_manifest(
    path: &std::path::Path,
    args: &RunArgs,
    plan: &DeckPlan,
    coordinates: &[AxisSetCoordinate],
) -> Result<(), CliError> {
    let coordinate_documents = coordinates
        .iter()
        .map(|coordinate| {
            let artifacts = coordinate
                .artifacts
                .iter()
                .map(|artifact| {
                    artifact
                        .file_name()
                        .and_then(std::ffi::OsStr::to_str)
                        .map(str::to_owned)
                        .ok_or_else(|| CliError::InternalError {
                            message: format!(
                                "coordinate artifact '{}' has no portable UTF-8 filename",
                                artifact.display()
                            ),
                        })
                })
                .collect::<Result<Vec<_>, CliError>>()?;
            Ok(serde_json::json!({
                "coordinate_id": coordinate.identity.id,
                "ordinal": coordinate.identity.ordinal,
                "tag": coordinate.identity.tag,
                "assignment": coordinate.identity.assignment,
                "artifacts": artifacts,
            }))
        })
        .collect::<Result<Vec<_>, CliError>>()?;
    let document = serde_json::json!({
        "schema_version": 1,
        "kind": "axis_coordinate_set",
        "deck": args.input.display().to_string(),
        "axes": plan
            .axes()
            .iter()
            .map(|axis| match axis.kind() {
                AxisKind::Temperature => "temperature".to_string(),
                AxisKind::Step => "step".to_string(),
                other => format!("{other:?}").to_ascii_lowercase(),
            })
            .collect::<Vec<_>>(),
        "coordinate_count": coordinates.len(),
        "coordinates": coordinate_documents,
    });
    let text = serde_json::to_string_pretty(&document)
        .map_err(|error| CliError::output_json_error(path, error))?
        + "\n";
    publish::set_manifest(path, |writer| {
        writer
            .write_all(text.as_bytes())
            .map_err(|error| CliError::output_error(path, error))
    })
    .map_err(|error| map_atomic_output_error(path, error))
}
