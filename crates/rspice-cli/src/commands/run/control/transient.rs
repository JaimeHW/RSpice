//! Reuse the ordinary transient execution and publication policy for scripts.
use super::*;
use rspice_core::engine::TransientResult;
use std::collections::BTreeMap;

pub(super) struct Completed {
    pub result: TransientResult,
    pub outputs: Vec<PathBuf>,
    pub published: Vec<PublishedResult>,
    pub measurements: Vec<MeasurementReport>,
    pub evaluated: std::collections::HashSet<String>,
}

pub(super) fn run(
    engine: &Engine,
    snapshot: &Netlist,
    command: &AnalysisCommand,
    analysis: AnalysisInstanceId,
    post_ordinals: &mut BTreeMap<rspice_core::execution::AnalysisKind, u32>,
    args: &RunArgs,
    config: &Config,
    verbose: bool,
    quiet: bool,
    run_label: Option<&str>,
    identity: &RunIdentity<'_>,
) -> Result<Completed, CliError> {
    let plan = DeckPlan::from_netlist_with_abort(
        snapshot,
        &engine.config().resource_limits,
        &crate::abort::ProcessAbort,
    )
    .map_err(|error| map_deck_plan_error(error, args))?;
    let posts = plan
        .post_process_analyses()
        .iter()
        .map(|post| {
            let next = post_ordinals.entry(post.id().kind()).or_default();
            let ordinal = *next;
            *next = next.checked_add(1).ok_or_else(|| CliError::InternalError {
                message: "control post-process ordinal overflow".into(),
            })?;
            post.for_control_execution(analysis, ordinal)
                .map_err(|error| map_deck_plan_error(error, args))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut ctx = RunContext::new(
        engine,
        snapshot,
        args,
        config,
        verbose,
        quiet,
        run_label,
        RunIdentity {
            coordinate: identity.coordinate,
            topology: identity.topology,
            analyses: PlannedAnalysisIdentities::from_pairs([(command, analysis)], &posts),
        },
    )?;
    ctx.qualify_control_outputs();
    let AnalysisCommand::Tran {
        step,
        stop,
        start,
        max_step,
        uic,
    } = command
    else {
        return Err(CliError::InternalError {
            message: "control transient callback received another analysis".into(),
        });
    };
    let outcome = basic::run_transient(&ctx, *stop, *step, start.unwrap_or(0.0), *max_step, *uic)?;
    let result = ctx.finish_control_transient(outcome)?;
    Ok(Completed {
        result,
        outputs: ctx.outputs.into_inner(),
        published: ctx.published.into_inner(),
        measurements: ctx.measurements.into_inner(),
        evaluated: ctx.evaluated_meas.into_inner(),
    })
}
